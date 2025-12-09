#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Prepare and publish nvm-rs releases to GitHub

.DESCRIPTION
    Prepares release artifacts, generates documentation, and uploads to GitHub Releases.
    Requires GitHub CLI (gh) to be installed and authenticated.

.PARAMETER Version
    Version to release (e.g., 'v0.5.0')
    If not specified, extracts from Cargo.toml

.PARAMETER BuildDir
    Directory containing built artifacts
    Default: ./release-builds

.PARAMETER TagOnly
    Create Git tag without uploading to GitHub

.PARAMETER Draft
    Create release as draft

.PARAMETER PreRelease
    Mark as pre-release

.PARAMETER UpdateInstaller
    Update installer scripts to reference new version

.PARAMETER Publish
    Publish draft release immediately

.EXAMPLE
    ./scripts/publish-release.ps1 -Version v0.5.0
    # Publish version 0.5.0 to GitHub

.EXAMPLE
    ./scripts/publish-release.ps1 -Draft
    # Create draft release for current version

.EXAMPLE
    ./scripts/publish-release.ps1 -TagOnly
    # Create local Git tag only (no GitHub upload)
#>

param(
    [string]$Version,
    [string]$BuildDir = "./release-builds",
    [switch]$TagOnly,
    [switch]$Draft,
    [switch]$PreRelease,
    [switch]$UpdateInstaller,
    [switch]$Publish,
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

# Get version from Cargo.toml if not provided
function Get-Version {
    param([string]$ProvidedVersion)

    if ([string]::IsNullOrEmpty($ProvidedVersion)) {
        $cargoContent = Get-Content "Cargo.toml" -Raw
        if ($cargoContent -match 'version\s*=\s*"([^"]+)"') {
            return "v$($matches[1])"
        }
        return "v0.5.0"
    }

    # Ensure version starts with 'v'
    if ($ProvidedVersion -notmatch '^v') {
        return "v$ProvidedVersion"
    }
    return $ProvidedVersion
}

$Version = Get-Version -ProvidedVersion $Version

Write-Section "NVM-RS Release Publisher"

Write-Info "Version: $Version"
Write-Info "Build Directory: $BuildDir"
Write-Info "Draft: $Draft"
Write-Info "Pre-Release: $PreRelease"

# Check prerequisites
Write-Section "Checking Prerequisites"

# Check Git
if (-not (Get-Command git -ErrorAction SilentlyContinue)) {
    Write-Error-Custom "Git not found. Install from https://git-scm.com"
    exit 1
}
Write-Success "Git found: $(git --version)"

# Check gh CLI only if not tag-only
if (-not $TagOnly) {
    if (-not (Get-Command gh -ErrorAction SilentlyContinue)) {
        Write-Error-Custom "GitHub CLI not found. Install from https://cli.github.com"
        exit 1
    }
    Write-Success "GitHub CLI found: $(gh --version)"

    # Check authentication
    try {
        $null = gh auth status 2>&1
        Write-Success "GitHub CLI authenticated"
    } catch {
        Write-Error-Custom "Not authenticated with GitHub. Run: gh auth login"
        exit 1
    }
}

# Check build artifacts exist
Write-Section "Verifying Build Artifacts"

if (-not (Test-Path $BuildDir)) {
    Write-Error-Custom "Build directory not found: $BuildDir"
    Write-Info "Run: ./scripts/build-releases.ps1"
    exit 1
}

$artifacts = @(Get-ChildItem -Path $BuildDir -File | Where-Object { $_.Name -like "nvm-*" -and $_.Name -notlike "*.sha256" -and $_.Name -notlike "*.json" })

if ($artifacts.Count -eq 0) {
    Write-Error-Custom "No build artifacts found in $BuildDir"
    exit 1
}

Write-Success "Found $($artifacts.Count) artifacts:"
foreach ($artifact in $artifacts) {
    Write-Info "  • $($artifact.Name) ($('{0:N0}' -f $artifact.Length) bytes)"
}

# Check if tag already exists
Write-Section "Checking Git Repository"

$tagExists = & { git rev-parse $Version 2>$null; $? }

if ($tagExists) {
    Write-Warning-Custom "Git tag $Version already exists"
    $confirm = Read-Host "Overwrite existing tag? (y/n)"
    if ($confirm -ne 'y') {
        Write-Info "Cancelled"
        exit 0
    }
    # Delete existing tag
    git tag -d $Version | Out-Null
    Write-Info "Deleted existing local tag"
}

# Create or update Git tag
Write-Section "Creating Git Tag"

$tagMessage = "Release $Version"

# Check if release notes file exists
$releaseNotesFile = "RELEASE_NOTES_$($Version.TrimStart('v')).md"
if (Test-Path $releaseNotesFile) {
    $releaseNotes = Get-Content $releaseNotesFile -Raw
    $tagMessage = $releaseNotes.Split("`n")[0]  # Use first line as message
}

git tag -a $Version -m $tagMessage
Write-Success "Created Git tag: $Version"

if (-not $TagOnly) {
    # Push tag to remote
    Write-Info "Pushing tag to remote..."
    git push origin $Version --force 2>&1 | ForEach-Object {
        if ($_ -match 'error|fatal') {
            Write-Warning-Custom $_
        }
    }
    Write-Success "Tag pushed"
}

# If tag-only mode, exit here
if ($TagOnly) {
    Write-Success "Git tag created successfully"
    Write-Info "Push with: git push origin $Version"
    exit 0
}

# Create GitHub Release
Write-Section "Creating GitHub Release"

# Load release notes if available
$releaseBodyPath = $releaseNotesFile
$releaseBody = if (Test-Path $releaseBodyPath) {
    Get-Content $releaseBodyPath -Raw
} else {
    # Generate basic release notes
    @"
## Release $Version

### What's New
- Multi-platform support (Windows, Linux, macOS)
- Multiple architectures (x64, ARM64)
- Cross-compiled binaries for all platforms

### Platforms
- **Windows**: x64, ARM64
- **Linux**: x64 (glibc/musl), ARM64 (glibc/musl)
- **macOS**: x64 (Intel), ARM64 (Apple Silicon)

### Installation
Download the appropriate binary for your platform and run the installer.

For more information, see [Installation Guide](https://github.com/FreddyCamposeco/nvm-rs#installation)
"@
}

# Build gh release command
$ghArgs = @(
    'release', 'create', $Version
    '--title', "NVM-RS Release $Version"
    '--notes', $releaseBody
)

if ($Draft) {
    $ghArgs += '--draft'
}

if ($PreRelease) {
    $ghArgs += '--prerelease'
}

# Add artifacts
$ghArgs += ($artifacts | ForEach-Object { $_.FullName })

# Also add checksums and manifest if they exist
$checksumFile = Join-Path $BuildDir "CHECKSUMS.sha256"
$manifestFile = Join-Path $BuildDir "manifest.json"

if (Test-Path $checksumFile) {
    $ghArgs += $checksumFile
}

if (Test-Path $manifestFile) {
    $ghArgs += $manifestFile
}

Write-Info "Creating release with gh CLI..."
try {
    & gh @ghArgs
    Write-Success "Release created successfully"
} catch {
    Write-Error-Custom "Failed to create release: $_"
    exit 1
}

# Show release info
Write-Section "Release Information"

Write-Info "Release: $Version"
Write-Info "Draft: $Draft"
Write-Info "Pre-Release: $PreRelease"
Write-Info "Artifacts: $($artifacts.Count)"

Write-Host ""
Write-Host "View release:" -ForegroundColor $Colors.Info
Write-Host "  gh release view $Version`n" -ForegroundColor Gray

if ($Draft -and -not $Publish) {
    Write-Host "Publish release:" -ForegroundColor $Colors.Info
    Write-Host "  gh release edit $Version --draft=false`n" -ForegroundColor Gray
}

# Update installer scripts
if ($UpdateInstaller) {
    Write-Section "Updating Installer Scripts"

    $versionNumber = $Version.TrimStart('v')

    # Update PowerShell installer
    $psInstallerPath = "scripts/install.ps1"
    if (Test-Path $psInstallerPath) {
        $content = Get-Content $psInstallerPath -Raw

        # Update version check or reference
        $content = $content -replace '\$latestRelease\s*=\s*"[^"]*"', "`$latestRelease = `"$Version`""
        $content = $content -replace 'v\d+\.\d+\.\d+', $Version

        $content | Out-File -FilePath $psInstallerPath -Encoding UTF8 -NoNewline
        Write-Success "Updated: $psInstallerPath"
    }

    # Update Shell installer
    $shInstallerPath = "scripts/install.sh"
    if (Test-Path $shInstallerPath) {
        $content = Get-Content $shInstallerPath -Raw

        $content = $content -replace 'LATEST_RELEASE="[^"]*"', "LATEST_RELEASE=`"$Version`""
        $content = $content -replace 'v\d+\.\d+\.\d+', $Version

        $content | Out-File -FilePath $shInstallerPath -Encoding UTF8 -NoNewline
        Write-Success "Updated: $shInstallerPath"
    }
}

# Publish draft if requested
if ($Draft -and $Publish) {
    Write-Section "Publishing Release"

    Write-Info "Publishing draft release..."
    & gh release edit $Version --draft=false
    Write-Success "Release published"
}

Write-Section "Release Completed"

Write-Host "Version: $Version" -ForegroundColor Green
Write-Host "Artifacts uploaded: $($artifacts.Count)" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor $Colors.Info
Write-Host "1. View release: gh release view $Version" -ForegroundColor Gray
Write-Host "2. Test installer: irm https://github.com/FreddyCamposeco/nvm-rs/releases/download/$Version/install.ps1 | iex" -ForegroundColor Gray
Write-Host "3. Announce release on social media/forums" -ForegroundColor Gray
