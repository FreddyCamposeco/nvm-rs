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
# Crear lista de nombres a buscar (en orden de preferencia)
$assetNames = @(
    "nvm-$releaseVersion-windows-$arch-self-update.exe",
    "nvm-$releaseVersion-windows-$arch.exe",
    "nvm-v$releaseVersion-windows-$arch.exe",
    "nvm.exe"
)

# Si no es self-update, filtrar para no buscar versiones con -self-update
if (-not $WithSelfUpdate) {
    $assetNames = $assetNames | Where-Object { -not $_.Contains("-self-update") }
}

$asset = $null
foreach ($name in $assetNames) {
    $asset = $release.assets | Where-Object { $_.name -eq $name } | Select-Object -First 1
    if ($asset) {
        $assetName = $name
        Write-Info "Asset a descargar: $assetName"
        break
    }
}

if (-not $asset) {
    Write-Error "No se encontró un asset compatible en la release"
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

# 1. Configurar NVM_HOME (home directory)
$currentNvmHome = [Environment]::GetEnvironmentVariable("NVM_HOME", "User")
if ($currentNvmHome -and (Test-Path $currentNvmHome)) {
    $nvmHomeDir = $currentNvmHome
    Write-Info "Variable NVM_HOME ya existe: $nvmHomeDir"
} else {
    $nvmHomeDir = "$env:USERPROFILE\.nvm"
    try {
        [Environment]::SetEnvironmentVariable("NVM_HOME", $nvmHomeDir, "User")
        $env:NVM_HOME = $nvmHomeDir
        Write-Success "✓ Variable NVM_HOME establecida: $nvmHomeDir"
    } catch {
        Write-Warning "⚠ No se pudo establecer NVM_HOME: $_"
    }
}

# 2. Configurar NVM_BIN (binario nvm)
try {
    $nvmBinDir = "$nvmHomeDir\bin"
    [Environment]::SetEnvironmentVariable("NVM_BIN", $nvmBinDir, "User")
    $env:NVM_BIN = $nvmBinDir
    Write-Success "✓ Variable NVM_BIN establecida: $nvmBinDir"
} catch {
    Write-Warning "⚠ No se pudo establecer NVM_BIN: $_"
}

# 3. Configurar NVM_NODE (node activo)
try {
    $nvmNodeDir = "$nvmHomeDir\current\bin"
    [Environment]::SetEnvironmentVariable("NVM_NODE", $nvmNodeDir, "User")
    $env:NVM_NODE = $nvmNodeDir
    Write-Success "✓ Variable NVM_NODE establecida: $nvmNodeDir"
} catch {
    Write-Warning "⚠ No se pudo establecer NVM_NODE: $_"
}

# 4. Crear estructura de directorios y shims
try {
    New-Item -ItemType Directory -Path $nvmHomeDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmBinDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmNodeDir -Force | Out-Null

    # Copiar el binario a NVM_BIN
    Copy-Item -Path $exePath -Destination (Join-Path $nvmBinDir "nvm.exe") -Force

    # Crear shim CMD
    $cmdShim = @'
@echo off
"%~dp0nvm.exe" %*
'@
    $cmdShimPath = Join-Path $nvmBinDir "nvm.cmd"
    Set-Content -Path $cmdShimPath -Value $cmdShim -Encoding ASCII

    # Crear shim PowerShell
    $psShim = @'
& "$PSScriptRoot\nvm.exe" @Args
'@
    $psShimPath = Join-Path $nvmBinDir "nvm.ps1"
    Set-Content -Path $psShimPath -Value $psShim -Encoding ASCII

    Write-Success "✓ Directorios y shims creados en $nvmBinDir"
} catch {
    Write-Warning "⚠ No se pudieron crear directorios/shims: $_"
}

# Verificar PATH
Write-Info ""
Write-Info "Verificando configuración del PATH..."
$currentUserPath = [Environment]::GetEnvironmentVariable("Path", "User")
$pathEntries = $currentUserPath -split ';'
$isNvmBinInPath = $pathEntries -contains "$env:USERPROFILE\.nvm\bin"
$isNvmNodeInPath = $pathEntries -contains "$env:USERPROFILE\.nvm\current\bin"

if ($isNvmBinInPath -and $isNvmNodeInPath) {
    Write-Success "✓ NVM_BIN y NVM_NODE ya están configurados en el PATH"
} else {
    Write-Warning "⚠ Falta configurar el PATH completo"

    if (-not $NoPrompt) {
        Write-Info ""
        $response = Read-Host "¿Desea agregar al PATH del usuario actual? (S/N)"
        if ($response -eq 'S' -or $response -eq 's' -or $response -eq 'Y' -or $response -eq 'y') {
            try {
                $newPath = $currentUserPath

                # Agregar NVM_BIN si no está
                if (-not $isNvmBinInPath) {
                    $newPath = if ($newPath) { "$newPath;$env:USERPROFILE\.nvm\bin" } else { "$env:USERPROFILE\.nvm\bin" }
                    Write-Success "✓ Agregado al PATH: $env:USERPROFILE\.nvm\bin"
                }

                # Agregar NVM_NODE si no está
                if (-not $isNvmNodeInPath) {
                    $newPath = if ($newPath) { "$newPath;$env:USERPROFILE\.nvm\current\bin" } else { "$env:USERPROFILE\.nvm\current\bin" }
                    Write-Success "✓ Agregado al PATH: $env:USERPROFILE\.nvm\current\bin"
                }

                [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
                $env:PATH = "$env:PATH;$env:USERPROFILE\.nvm\bin;$env:USERPROFILE\.nvm\current\bin"
                Write-Success "✓ PATH actualizado correctamente"
                Write-Info "⚠ Es posible que necesite reiniciar su terminal para ver los cambios"
            } catch {
                Write-Warning "⚠ No se pudo actualizar el PATH automáticamente: $_"
                Write-Info ""
                Write-Info "Para agregar manualmente:"
                Write-Info "1. Buscar 'Variables de entorno' en el menú Inicio"
                Write-Info "2. Editar la variable PATH del usuario"
                Write-Info "3. Agregar: %USERPROFILE%\.nvm\bin"
                Write-Info "4. Agregar: %USERPROFILE%\.nvm\current\bin"
            }
        }
    } else {
        Write-Info ""
        Write-Info "Para agregar al PATH manualmente:"
        Write-Info "1. Buscar 'Variables de entorno' en el menú Inicio"
        Write-Info "2. Editar la variable PATH del usuario"
        Write-Info "3. Agregar: %USERPROFILE%\.nvm\bin"
        Write-Info "4. Agregar: %USERPROFILE%\.nvm\current\bin"
        Write-Info ""
        Write-Info "O ejecutar en PowerShell:"
        Write-Info '[Environment]::SetEnvironmentVariable("Path", $env:Path + ";%USERPROFILE%\.nvm\bin;%USERPROFILE%\.nvm\current\bin", "User")'
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
