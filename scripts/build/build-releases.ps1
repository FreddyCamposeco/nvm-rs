#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build script for nvm-rs cross-platform releases

.DESCRIPTION
    Compiles nvm-rs for multiple platforms and architectures, creating properly named
    release artifacts compatible with the installer logic.

    Supported targets:
    - Windows x86_64 (x64)
    - Windows ARM64 (aarch64)
    - Linux x86_64 (gnu, musl)
    - Linux ARM64 (gnu, musl)
    - macOS x86_64 (intel)
    - macOS ARM64 (apple-silicon)

.PARAMETER Target
    Specific target to build (e.g., 'windows-x64', 'linux-gnu-x64')
    If not specified, builds all configured targets

.PARAMETER BuildType
    Type of build: 'release', 'debug'
    Default: release

.PARAMETER OutputDir
    Directory where release artifacts will be stored
    Default: ./release-builds

.PARAMETER SkipClean
    Skip cleaning before build

.PARAMETER WithSelfUpdate
    Include self-update capability in the build

.PARAMETER SignWin
    Sign Windows binaries (requires code signing certificate)

.EXAMPLE
    ./scripts/build-releases.ps1
    # Build all targets in release mode

.EXAMPLE
    ./scripts/build-releases.ps1 -Target windows-x64 -BuildType release
    # Build only Windows x64 release

.EXAMPLE
    ./scripts/build-releases.ps1 -WithSelfUpdate
    # Build all targets with self-update feature enabled
#>

param(
    [string]$Target,
    [ValidateSet('release', 'debug')]
    [string]$BuildType = 'release',
    [string]$OutputDir = "./release-builds",
    [switch]$SkipClean,
    [switch]$WithSelfUpdate,
    [switch]$SignWin,
    [switch]$Help
)

# Colors for output
$Colors = @{
    Success = 'Green'
    Error   = 'Red'
    Warning = 'Yellow'
    Info    = 'Cyan'
    Header  = 'Magenta'
}

function Write-Section {
    param([string]$Message)
    Write-Host ""
    Write-Host "=" * 60 -ForegroundColor $Colors.Header
    Write-Host $Message -ForegroundColor $Colors.Header
    Write-Host "=" * 60 -ForegroundColor $Colors.Header
    Write-Host ""
}

function Write-Success {
    param([string]$Message)
    Write-Host "✓ $Message" -ForegroundColor $Colors.Success
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "✗ $Message" -ForegroundColor $Colors.Error
}

function Write-Warning-Custom {
    param([string]$Message)
    Write-Host "⚠ $Message" -ForegroundColor $Colors.Warning
}

function Write-Info {
    param([string]$Message)
    Write-Host "ℹ $Message" -ForegroundColor $Colors.Info
}

# Display help
if ($Help) {
    Get-Help $PSCommandPath -Detailed
    exit 0
}

# Get version from Cargo.toml
function Get-Version {
    $cargoContent = Get-Content "Cargo.toml" -Raw
    if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
        return $matches[1]
    }
    return "0.5.0"
}

$Version = Get-Version

# Define build targets with their Rust target triple and output extensions
$Targets = @{
    "windows-x64" = @{
        triple = "x86_64-pc-windows-msvc"
        os = "windows"
        arch = "x64"
        ext = "exe"
        install_target = $true
    }
    "windows-arm64" = @{
        triple = "aarch64-pc-windows-msvc"
        os = "windows"
        arch = "arm64"
        ext = "exe"
        install_target = $false
    }
    "linux-gnu-x64" = @{
        triple = "x86_64-unknown-linux-gnu"
        os = "linux"
        arch = "x64"
        ext = ""
        install_target = $false
    }
    "linux-musl-x64" = @{
        triple = "x86_64-unknown-linux-musl"
        os = "linux"
        arch = "x64"
        ext = ""
        install_target = $false
    }
    "linux-gnu-arm64" = @{
        triple = "aarch64-unknown-linux-gnu"
        os = "linux"
        arch = "arm64"
        ext = ""
        install_target = $false
    }
    "linux-musl-arm64" = @{
        triple = "aarch64-unknown-linux-musl"
        os = "linux"
        arch = "arm64"
        ext = ""
        install_target = $false
    }
    "macos-x64" = @{
        triple = "x86_64-apple-darwin"
        os = "macos"
        arch = "x64"
        ext = ""
        install_target = $false
    }
    "macos-arm64" = @{
        triple = "aarch64-apple-darwin"
        os = "macos"
        arch = "arm64"
        ext = ""
        install_target = $false
    }
}

# Filter targets if specified
$TargetsToRun = if ($Target) {
    if ($Targets.ContainsKey($Target)) {
        @{ $Target = $Targets[$Target] }
    } else {
        Write-Error-Custom "Target '$Target' not found. Available targets: $(($Targets.Keys -join ', '))"
        exit 1
    }
} else {
    # On Windows, only build compatible targets
    $isWindows = $PSVersionTable.Platform -eq 'Win32NT' -or -not $PSVersionTable.Platform

    if ($isWindows) {
        # Windows can build Windows targets
        @{
            "windows-x64" = $Targets["windows-x64"]
            "windows-arm64" = $Targets["windows-arm64"]
        }
    } else {
        # Unix-like systems
        $Targets
    }
}

Write-Section "NVM-RS Multi-Platform Build Script v$Version"

Write-Info "Build Type: $BuildType"
Write-Info "Targets to build: $(($TargetsToRun.Keys -join ', '))"
Write-Info "Output Directory: $OutputDir"
if ($WithSelfUpdate) {
    Write-Info "Self-Update: Enabled"
}
Write-Host ""

# Check prerequisites
Write-Section "Checking Prerequisites"

# Check Rust
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error-Custom "Rust/Cargo not found. Install from https://rustup.rs"
    exit 1
}
Write-Success "Cargo found: $(cargo --version)"

# Check required targets are installed
$installedTargets = @(rustup target list --installed)
foreach ($targetKey in $TargetsToRun.Keys) {
    $triple = $TargetsToRun[$targetKey].triple
    if ($triple -notin $installedTargets) {
        Write-Warning-Custom "Target $triple not installed. Installing..."
        rustup target add $triple
    }
}

# Create output directory
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
    Write-Success "Created output directory: $OutputDir"
} else {
    Write-Info "Output directory exists: $OutputDir"
}

# Clean if requested
if (-not $SkipClean) {
    Write-Section "Cleaning Previous Builds"
    cargo clean
    Write-Success "Cleaned"
}

# Build configuration
$CargoFlags = @()
if ($BuildType -eq 'release') {
    $CargoFlags += '--release'
}

# Add self-update feature if requested
if ($WithSelfUpdate) {
    $CargoFlags += '--features', 'self-update'
}

# Build each target
Write-Section "Building Targets"

$buildResults = @()
$failedBuilds = @()

foreach ($targetKey in $TargetsToRun.Keys) {
    $targetInfo = $TargetsToRun[$targetKey]
    $triple = $targetInfo.triple

    Write-Info "Building $targetKey ($triple)..."

    # Build the cargo command as a script block
    $cargoArgs = @('build', '--target', $triple) + $CargoFlags

    try {
        # Execute cargo with arguments
        $buildOutput = & cargo @cargoArgs 2>&1

        # Show output
        $buildOutput | ForEach-Object {
            if ($_ -match 'error') {
                Write-Host $_ -ForegroundColor Red
            } elseif ($_ -match 'warning') {
                Write-Host $_ -ForegroundColor Yellow
            } else {
                Write-Host $_
            }
        }

        if ($LASTEXITCODE -ne 0) {
            Write-Error-Custom "Build failed for $targetKey"
            $failedBuilds += $targetKey
            continue
        }

        # Locate binary
        $binaryPath = if ($BuildType -eq 'release') {
            "target/$triple/release/nvm$(if ($targetInfo.ext) { '.' + $targetInfo.ext })"
        } else {
            "target/$triple/debug/nvm$(if ($targetInfo.ext) { '.' + $targetInfo.ext })"
        }

        if (-not (Test-Path $binaryPath)) {
            Write-Error-Custom "Binary not found at $binaryPath"
            $failedBuilds += $targetKey
            continue
        }

        # Generate output filename
        $outputName = "nvm-v$Version-$($targetInfo.os)-$($targetInfo.arch)"
        if ($targetInfo.ext) {
            $outputName += ".$($targetInfo.ext)"
        }

        # Add self-update variant name if building with self-update
        if ($WithSelfUpdate) {
            $outputName = $outputName -replace "nvm-v", "nvm-v$Version-self-update-"
        }

        $outputPath = Join-Path $OutputDir $outputName

        # Copy binary to output directory
        Copy-Item -Path $binaryPath -Destination $outputPath -Force

        # Get file size and hash
        $fileInfo = Get-Item $outputPath
        $hash = (Get-FileHash -Path $outputPath -Algorithm SHA256).Hash

        Write-Success "Built: $outputName ($('{0:N0}' -f $fileInfo.Length) bytes)"
        Write-Info "SHA256: $hash"

        $buildResults += @{
            Target = $targetKey
            OutputName = $outputName
            OutputPath = $outputPath
            Size = $fileInfo.Length
            Hash = $hash
            Triple = $triple
        }

    } catch {
        Write-Error-Custom "Exception building $targetKey : $_"
        $failedBuilds += $targetKey
    }
}

# Generate checksums file
if ($buildResults.Count -gt 0) {
    Write-Section "Generating Checksums"

    $checksumsPath = Join-Path $OutputDir "CHECKSUMS.sha256"
    $checksumContent = @()

    foreach ($result in $buildResults) {
        $checksumContent += "$($result.Hash)  $($result.OutputName)"
    }

    $checksumContent | Out-File -FilePath $checksumsPath -Encoding UTF8
    Write-Success "Checksums saved to: CHECKSUMS.sha256"
}

# Generate release manifest
if ($buildResults.Count -gt 0) {
    Write-Section "Generating Release Manifest"

    $manifestPath = Join-Path $OutputDir "manifest.json"

    $manifest = @{
        version = $Version
        release_date = (Get-Date -Format 'o')
        build_type = $BuildType
        with_self_update = $WithSelfUpdate
        binaries = @($buildResults | ForEach-Object {
            @{
                name = $_.OutputName
                platform = $_.Target
                size = $_.Size
                sha256 = $_.Hash
                target_triple = $_.Triple
            }
        })
    }

    $manifest | ConvertTo-Json -Depth 10 | Out-File -FilePath $manifestPath -Encoding UTF8
    Write-Success "Manifest saved to: manifest.json"
}

# Generate release notes
Write-Section "Release Summary"
Write-Info "Version: $Version"
Write-Info "Build Type: $BuildType"
Write-Info "Total Targets: $($TargetsToRun.Count)"
Write-Info "Successful Builds: $($buildResults.Count)"

if ($failedBuilds.Count -gt 0) {
    Write-Warning-Custom "Failed Builds: $($failedBuilds -join ', ')"
}

Write-Host ""
Write-Host "Built binaries:" -ForegroundColor $Colors.Info
foreach ($result in $buildResults) {
    Write-Host "  • $($result.OutputName) - $('{0:N0}' -f $result.Size) bytes" -ForegroundColor $Colors.Success
}

# Instructions for next steps
Write-Section "Next Steps"

Write-Host "1. Review the release artifacts:" -ForegroundColor $Colors.Info
Write-Host "   ls $OutputDir`n" -ForegroundColor Gray

Write-Host "2. Create GitHub Release:" -ForegroundColor $Colors.Info
Write-Host "   gh release create v$Version --title 'Release v$Version' --draft`n" -ForegroundColor Gray

Write-Host "3. Upload artifacts:" -ForegroundColor $Colors.Info
Write-Host "   gh release upload v$Version $OutputDir/*`n" -ForegroundColor Gray

Write-Host "4. Publish release:" -ForegroundColor $Colors.Info
Write-Host "   gh release edit v$Version --draft=false`n" -ForegroundColor Gray

Write-Host "5. Test installation:" -ForegroundColor $Colors.Info
Write-Host "   irm https://github.com/FreddyCamposeco/nvm-rs/releases/download/v$Version/install.ps1 | iex`n" -ForegroundColor Gray

# Exit with appropriate code
if ($failedBuilds.Count -gt 0) {
    exit 1
} else {
    Write-Success "Build completed successfully!"
    exit 0
}
