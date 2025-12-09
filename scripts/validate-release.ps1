#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Validate nvm-rs release artifacts

.DESCRIPTION
    Validates release artifacts for integrity, correctness, and platform compatibility.
    Checks:
    - File integrity (checksums)
    - Binary compatibility
    - Asset naming conventions
    - Required assets for full release

.PARAMETER ReleaseDir
    Directory containing release artifacts
    Default: ./release-builds

.PARAMETER Strict
    Strict validation mode - fails on any warnings

.EXAMPLE
    ./scripts/validate-release.ps1
    # Validate artifacts in default directory

.EXAMPLE
    ./scripts/validate-release.ps1 -ReleaseDir ./release-v0.5.0 -Strict
    # Strict validation of specific directory
#>

param(
    [string]$ReleaseDir = "./release-builds",
    [switch]$Strict,
    [switch]$Help
)

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

if ($Help) {
    Get-Help $PSCommandPath -Detailed
    exit 0
}

Write-Section "NVM-RS Release Validator"

# Check release directory exists
if (-not (Test-Path $ReleaseDir)) {
    Write-Error-Custom "Release directory not found: $ReleaseDir"
    exit 1
}

Write-Info "Checking directory: $ReleaseDir"

# Collect artifacts
$artifacts = @(Get-ChildItem -Path $ReleaseDir -File)
$binaries = @($artifacts | Where-Object { $_.Name -like "nvm-*" -and $_.Name -notlike "*.sha256" -and $_.Name -notlike "*.json" })
$checksumFile = $artifacts | Where-Object { $_.Name -like "CHECKSUMS*" }
$manifestFile = $artifacts | Where-Object { $_.Name -like "manifest.json" }

Write-Info "Found $($artifacts.Count) files"
Write-Info "  Binaries: $($binaries.Count)"
Write-Info "  Checksums: $(if ($checksumFile) { 1 } else { 0 })"
Write-Info "  Manifest: $(if ($manifestFile) { 1 } else { 0 })"

$validationErrors = 0
$validationWarnings = 0

# Define required platforms for a complete release
$RequiredPlatforms = @(
    "windows-x64"
    "linux-gnu-x64"
    "macos-x64"
)

Write-Section "Validating Binaries"

# Expected naming format: nvm-vX.Y.Z-OS-ARCH[.ext]
$namePattern = '^nvm-v(\d+\.\d+\.\d+)-(windows|linux|macos)-(x64|arm64)(\.exe)?$'

$platforms = @{}

foreach ($binary in $binaries) {
    Write-Info "Checking: $($binary.Name)"

    # Validate naming
    if ($binary.Name -match $namePattern) {
        $version = $matches[1]
        $os = $matches[2]
        $arch = $matches[3]

        Write-Host "  Version: $version" -ForegroundColor Gray
        Write-Host "  Platform: $os-$arch" -ForegroundColor Gray

        $key = "$os-$arch"
        $platforms[$key] = $binary

        # Check file size
        $sizeKB = [math]::Round($binary.Length / 1KB, 2)
        Write-Host "  Size: $sizeKB KB" -ForegroundColor Gray

        # Validate file is not empty
        if ($binary.Length -lt 100KB) {
            Write-Warning-Custom "  Binary suspiciously small ($sizeKB KB)"
            $validationWarnings++
        }

        # Validate binary is executable
        if ($os -eq "windows") {
            if (-not $binary.Name.EndsWith(".exe")) {
                Write-Error-Custom "  Windows binary must have .exe extension"
                $validationErrors++
            }
        } else {
            if ($binary.Name.EndsWith(".exe")) {
                Write-Error-Custom "  Non-Windows binary should not have .exe extension"
                $validationErrors++
            }
        }

        Write-Success "  Valid"
    } else {
        Write-Error-Custom "  Invalid filename format"
        Write-Info "  Expected format: nvm-vX.Y.Z-OS-ARCH[.exe]"
        Write-Info "  Valid OS: windows, linux, macos"
        Write-Info "  Valid ARCH: x64, arm64"
        $validationErrors++
    }
}

Write-Host ""

# Check for complete platform coverage
Write-Section "Validating Platform Coverage"

$missingPlatforms = @()
foreach ($platform in $RequiredPlatforms) {
    if ($platforms.ContainsKey($platform)) {
        Write-Success "$platform included"
    } else {
        Write-Warning-Custom "$platform missing (optional for beta releases)"
        $missingPlatforms += $platform
        $validationWarnings++
    }
}

# Validate checksums if present
if ($checksumFile) {
    Write-Section "Validating Checksums"

    try {
        $checksumContent = Get-Content $checksumFile.FullName -Raw
        $checksumLines = $checksumContent.Trim().Split("`n")

        Write-Info "Found $($checksumLines.Count) checksums"

        foreach ($line in $checksumLines) {
            if ([string]::IsNullOrWhiteSpace($line)) { continue }

            $parts = $line.Split([char[]]@(' ', "`t"), [System.StringSplitOptions]::RemoveEmptyEntries)
            if ($parts.Count -lt 2) { continue }

            $expectedHash = $parts[0]
            $filename = $parts[-1]

            # Find corresponding binary
            $binary = $binaries | Where-Object { $_.Name -eq $filename }

            if ($binary) {
                $actualHash = (Get-FileHash -Path $binary.FullName -Algorithm SHA256).Hash

                if ($actualHash -eq $expectedHash) {
                    Write-Success "  $filename"
                } else {
                    Write-Error-Custom "  $filename - CHECKSUM MISMATCH"
                    Write-Info "    Expected: $expectedHash"
                    Write-Info "    Actual:   $actualHash"
                    $validationErrors++
                }
            } else {
                Write-Warning-Custom "  $filename - file not found"
                $validationWarnings++
            }
        }
    } catch {
        Write-Error-Custom "Failed to validate checksums: $_"
        $validationErrors++
    }
} else {
    Write-Warning-Custom "No checksum file found"
    $validationWarnings++
}

# Validate manifest if present
if ($manifestFile) {
    Write-Section "Validating Manifest"

    try {
        $manifest = Get-Content $manifestFile.FullName | ConvertFrom-Json

        Write-Info "Version: $($manifest.version)"
        Write-Info "Build Type: $($manifest.build_type)"
        Write-Info "Release Date: $($manifest.release_date)"

        if ($manifest.binaries) {
            Write-Info "Binaries in manifest: $($manifest.binaries.Count)"

            # Verify all binaries are listed
            foreach ($binary in $binaries) {
                if ($manifest.binaries.name -contains $binary.Name) {
                    Write-Success "  $($binary.Name) listed"
                } else {
                    Write-Warning-Custom "  $($binary.Name) not in manifest"
                    $validationWarnings++
                }
            }
        }
    } catch {
        Write-Error-Custom "Failed to parse manifest: $_"
        $validationErrors++
    }
} else {
    Write-Warning-Custom "No manifest file found"
    $validationWarnings++
}

# Summary
Write-Section "Validation Summary"

Write-Host "Total Binaries: $($binaries.Count)" -ForegroundColor $Colors.Info
Write-Host "Platforms Detected: $($platforms.Count)" -ForegroundColor $Colors.Info
Write-Host ""

if ($validationErrors -gt 0) {
    Write-Error-Custom "Validation Errors: $validationErrors"
}

if ($validationWarnings -gt 0) {
    Write-Warning-Custom "Validation Warnings: $validationWarnings"
}

if ($validationErrors -eq 0 -and $validationWarnings -eq 0) {
    Write-Success "All validations passed!"
    Write-Host ""
    Write-Host "Release is ready for publication:" -ForegroundColor $Colors.Success
    foreach ($platform in $platforms.Keys | Sort-Object) {
        Write-Host "  • $platform" -ForegroundColor $Colors.Success
    }
    exit 0
} elseif ($validationErrors -eq 0) {
    if ($Strict) {
        Write-Error-Custom "Validation failed in strict mode"
        exit 1
    } else {
        Write-Warning-Custom "Release has warnings but is usable"
        exit 0
    }
} else {
    Write-Error-Custom "Validation failed"
    exit 1
}
