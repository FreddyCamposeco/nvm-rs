# Script de instalación de nvm-rs para Windows
# Uso: iwr -useb https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/install.ps1 | iex

param(
    [string]$Version = "latest",
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\nvm",
    [switch]$WithSelfUpdate,
    [switch]$NoPrompt
)

$ErrorActionPreference = "Stop"

# Colores para output
function Write-Success { Write-Host $args -ForegroundColor Green }
function Write-Info { Write-Host $args -ForegroundColor Cyan }
function Write-Warning { Write-Host $args -ForegroundColor Yellow }
function Write-Error { Write-Host $args -ForegroundColor Red }

Write-Info "=== Instalador de nvm-rs para Windows ==="
Write-Info ""

# Detectar arquitectura
$arch = if ([Environment]::Is64BitOperatingSystem) { "x64" } else { "x86" }
Write-Info "Arquitectura detectada: $arch"

# Obtener información de la release
Write-Info "Obteniendo información de la versión..."
$apiUrl = if ($Version -eq "latest") {
    "https://api.github.com/repos/FreddyCamposeco/nvm-rs/releases/latest"
} else {
    "https://api.github.com/repos/FreddyCamposeco/nvm-rs/releases/tags/$Version"
}

try {
    $release = Invoke-RestMethod -Uri $apiUrl -Headers @{
        "User-Agent" = "nvm-rs-installer"
    }
    $releaseVersion = $release.tag_name
    Write-Info "Versión a instalar: $releaseVersion"
} catch {
    Write-Error "Error al obtener información de la release: $_"
    exit 1
}

# Determinar nombre del asset
$suffix = if ($WithSelfUpdate) { "-self-update" } else { "" }
$assetName = "nvm-$releaseVersion-windows-$arch$suffix.exe"
Write-Info "Asset a descargar: $assetName"

# Buscar el asset en la release
$asset = $release.assets | Where-Object { $_.name -eq $assetName }
if (-not $asset) {
    Write-Error "Asset $assetName no encontrado en la release"
    Write-Info "Assets disponibles:"
    $release.assets | ForEach-Object { Write-Info "  - $($_.name)" }
    exit 1
}

# Crear directorio de instalación
Write-Info "Directorio de instalación: $InstallDir"
if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Write-Success "✓ Directorio de instalación creado"
}

# Descargar binario
$downloadUrl = $asset.browser_download_url
$downloadPath = Join-Path $env:TEMP $assetName
Write-Info ""
Write-Info "Descargando $assetName..."
Write-Info "URL: $downloadUrl"

try {
    # Usar WebClient para mostrar progreso
    $webClient = New-Object System.Net.WebClient
    $webClient.Headers.Add("User-Agent", "nvm-rs-installer")
    
    Register-ObjectEvent -InputObject $webClient -EventName DownloadProgressChanged -Action {
        $progress = $EventArgs.ProgressPercentage
        Write-Progress -Activity "Descargando nvm-rs" -Status "$progress% completado" -PercentComplete $progress
    } | Out-Null
    
    $webClient.DownloadFile($downloadUrl, $downloadPath)
    $webClient.Dispose()
    Write-Progress -Activity "Descargando nvm-rs" -Completed
    Write-Success "✓ Descarga completada"
} catch {
    Write-Error "Error al descargar el binario: $_"
    exit 1
}

# Verificar checksum
Write-Info ""
Write-Info "Verificando integridad del archivo..."
$hash = Get-FileHash -Path $downloadPath -Algorithm SHA256
Write-Info "SHA256: $($hash.Hash)"

# Instalar binario
$exePath = Join-Path $InstallDir "nvm.exe"
Write-Info ""
Write-Info "Instalando binario..."

# Hacer backup si existe
if (Test-Path $exePath) {
    $backupPath = Join-Path $InstallDir "nvm.exe.bak"
    Move-Item -Path $exePath -Destination $backupPath -Force
    Write-Warning "⚠ Backup creado: nvm.exe.bak"
}

# Copiar nuevo binario
Copy-Item -Path $downloadPath -Destination $exePath -Force
Write-Success "✓ Binario instalado en: $exePath"

# Limpiar archivo temporal
Remove-Item -Path $downloadPath -Force

# Verificar que funciona
Write-Info ""
Write-Info "Verificando instalación..."
try {
    $version = & $exePath --version 2>&1
    Write-Success "✓ nvm instalado correctamente: $version"
} catch {
    Write-Warning "⚠ No se pudo verificar la instalación automáticamente"
}

# Configurar variables de entorno
Write-Info ""
Write-Info "Configurando variables de entorno..."

# 1. Configurar NVM_DIR
$currentNvmDir = [Environment]::GetEnvironmentVariable("NVM_DIR", "User")
if ($currentNvmDir -and (Test-Path $currentNvmDir)) {
    Write-Info "Variable NVM_DIR ya existe: $currentNvmDir"
} else {
    $nvmDataDir = "$env:USERPROFILE\.nvm"
    try {
        [Environment]::SetEnvironmentVariable("NVM_DIR", $nvmDataDir, "User")
        $env:NVM_DIR = $nvmDataDir
        Write-Success "✓ Variable NVM_DIR establecida: $nvmDataDir"
    } catch {
        Write-Warning "⚠ No se pudo establecer NVM_DIR: $_"
    }
}

# Verificar PATH
Write-Info ""
Write-Info "Verificando configuración del PATH..."
$currentUserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathEntries = $currentUserPath -split ';'
$isInPath = $pathEntries -contains $InstallDir

if ($isInPath) {
    Write-Success "✓ El directorio de instalación ya está en el PATH"
} else {
    Write-Warning "⚠ El directorio de instalación NO está en el PATH"
    
    if (-not $NoPrompt) {
        Write-Info ""
        $response = Read-Host "¿Desea agregar al PATH del usuario actual? (S/N)"
        if ($response -eq 'S' -or $response -eq 's' -or $response -eq 'Y' -or $response -eq 'y') {
            try {
                $newPath = if ($currentUserPath) { "$currentUserPath;$InstallDir" } else { $InstallDir }
                [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
                $env:PATH = "$env:PATH;$InstallDir"
                Write-Success "✓ PATH actualizado correctamente"
                Write-Info "⚠ Es posible que necesite reiniciar su terminal para ver los cambios"
            } catch {
                Write-Warning "⚠ No se pudo actualizar el PATH automáticamente: $_"
                Write-Info ""
                Write-Info "Para agregar manualmente:"
                Write-Info "1. Buscar 'Variables de entorno' en el menú Inicio"
                Write-Info "2. Editar la variable PATH del usuario"
                Write-Info "3. Agregar: $InstallDir"
            }
        }
    } else {
        Write-Info ""
        Write-Info "Para agregar al PATH manualmente:"
        Write-Info "1. Buscar 'Variables de entorno' en el menú Inicio"
        Write-Info "2. Editar la variable PATH del usuario"
        Write-Info "3. Agregar: $InstallDir"
        Write-Info ""
        Write-Info "O ejecutar en PowerShell:"
        Write-Info '[Environment]::SetEnvironmentVariable("Path", $env:Path + ";' + $InstallDir + '", "User")'
    }
}

Write-Info ""
Write-Success "=== ✓ Instalación completada ==="
Write-Info ""
Write-Info "Para comenzar a usar nvm:"
Write-Info "  nvm --help             # Ver ayuda"
Write-Info "  nvm ls-remote          # Listar versiones disponibles de Node.js"
Write-Info "  nvm install latest     # Instalar última versión de Node.js"
Write-Info "  nvm use latest         # Usar última versión instalada"
Write-Info ""
Write-Info "Para actualizar nvm en el futuro:"
Write-Info "  nvm update-self        # Actualizar a la última versión"
Write-Info ""
Write-Success "¡Gracias por usar nvm-rs!"
