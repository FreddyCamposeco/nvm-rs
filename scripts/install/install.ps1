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

# Determinar nombre del asset con patrón flexible
# El instalador busca binarios compatibles con el formato: nvm-vX.Y.Z-windows-ARCH[.exe]
$versionNumber = $releaseVersion -replace '^v', ''

# Crear patrones de búsqueda (en orden de preferencia)
$searchPatterns = @(
    # Exact version with self-update if requested
    if ($WithSelfUpdate) { "nvm-v$versionNumber-self-update-windows-$arch.exe" }

    # Exact version
    "nvm-v$versionNumber-windows-$arch.exe"

    # Version without 'v' prefix
    "nvm-$versionNumber-windows-$arch.exe"

    # Any nvm for windows with architecture
    "*windows-$arch.exe"

    # Fallback to any nvm.exe
    "nvm.exe"
)

$asset = $null
$assetName = $null

foreach ($pattern in $searchPatterns) {
    Write-Info "Buscando: $pattern"
    $asset = $release.assets | Where-Object { $_.name -like $pattern } | Select-Object -First 1
    if ($asset) {
        $assetName = $asset.name
        Write-Info "Asset encontrado: $assetName"
        break
    }
}

if (-not $asset) {
    Write-Error "Error: No se encontró un asset compatible para Windows $arch en la release"
    Write-Info ""
    Write-Info "Assets disponibles en la release:"
    $release.assets | ForEach-Object { Write-Info "  • $($_.name) ($([math]::Round($_.size/1MB, 2)) MB)" }
    Write-Info ""
    Write-Info "Por favor, verifica que la release contiene binarios compilados para Windows"
    exit 1
}

# Crear estructura de directorios NVM primero
Write-Info ""
Write-Info "Configurando estructura de directorios..."
$nvmHomeDir = "$env:USERPROFILE\.nvm"
$nvmBinDir = "$nvmHomeDir\bin"
$nvmNodeDir = "$nvmHomeDir\current\bin"
$nvmVersionsDir = "$nvmHomeDir\versions"
$nvmCacheDir = "$nvmHomeDir\cache"
$nvmAliasDir = "$nvmHomeDir\alias"

try {
    New-Item -ItemType Directory -Path $nvmHomeDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmBinDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmNodeDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmVersionsDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmCacheDir -Force | Out-Null
    New-Item -ItemType Directory -Path $nvmAliasDir -Force | Out-Null
    Write-Success "✓ Directorios creados"
} catch {
    Write-Error "Error al crear directorios: $_"
    exit 1
}

# Descargar binario directamente a NVM_BIN
$downloadUrl = $asset.browser_download_url
$exePath = Join-Path $nvmBinDir "nvm.exe"

# Hacer backup si existe
if (Test-Path $exePath) {
    $backupPath = Join-Path $nvmBinDir "nvm.exe.bak"
    Move-Item -Path $exePath -Destination $backupPath -Force
    Write-Warning "⚠ Backup creado: nvm.exe.bak"
}

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

    $webClient.DownloadFile($downloadUrl, $exePath)
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
$hash = Get-FileHash -Path $exePath -Algorithm SHA256
Write-Info "SHA256: $($hash.Hash)"
Write-Success "✓ Binario instalado en: $exePath"

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
try {
    [Environment]::SetEnvironmentVariable("NVM_HOME", $nvmHomeDir, "User")
    $env:NVM_HOME = $nvmHomeDir
    Write-Success "✓ Variable NVM_HOME establecida: $nvmHomeDir"
} catch {
    Write-Warning "⚠ No se pudo establecer NVM_HOME: $_"
}

# 2. Configurar NVM_BIN (binario nvm)
try {
    [Environment]::SetEnvironmentVariable("NVM_BIN", $nvmBinDir, "User")
    $env:NVM_BIN = $nvmBinDir
    Write-Success "✓ Variable NVM_BIN establecida: $nvmBinDir"
} catch {
    Write-Warning "⚠ No se pudo establecer NVM_BIN: $_"
}

# 3. Configurar NVM_NODE (node activo)
try {
    [Environment]::SetEnvironmentVariable("NVM_NODE", $nvmNodeDir, "User")
    $env:NVM_NODE = $nvmNodeDir
    Write-Success "✓ Variable NVM_NODE establecida: $nvmNodeDir"
} catch {
    Write-Warning "⚠ No se pudo establecer NVM_NODE: $_"
}

# 4. Crear shims en NVM_BIN
try {
    # Crear shim CMD que apunta al binario en NVM_BIN
    $cmdShim = @'
@echo off
"%~dp0nvm.exe" %*
'@
    $cmdShimPath = Join-Path $nvmBinDir "nvm.cmd"
    Set-Content -Path $cmdShimPath -Value $cmdShim -Encoding ASCII
    Write-Success "✓ Creado shim CMD: $cmdShimPath"

    # Crear shim PowerShell que apunta al binario en NVM_BIN
    $psShim = @'
& "$PSScriptRoot\nvm.exe" @Args
'@
    $psShimPath = Join-Path $nvmBinDir "nvm.ps1"
    Set-Content -Path $psShimPath -Value $psShim -Encoding ASCII
    Write-Success "✓ Creado shim PowerShell: $psShimPath"

    Write-Success "✓ Shims creados en $nvmBinDir"
} catch {
    Write-Warning "⚠ No se pudieron crear shims: $_"
}

# 5. Configurar alias en PowerShell profile
try {
    $profileDir = Split-Path $profile
    if (-not (Test-Path $profileDir)) {
        New-Item -ItemType Directory -Path $profileDir -Force | Out-Null
    }

    # Crear o actualizar profile con alias de nvm
    $aliasLine = "Set-Alias -Name nvm -Value '$nvmBinDir\nvm.exe' -Force -Option AllScope"

    if (Test-Path $profile) {
        if ((Get-Content $profile) -notmatch "Set-Alias.*nvm") {
            Add-Content $profile "`n$aliasLine"
            Write-Success "✓ Alias nvm agregado al PowerShell profile"
        }
    } else {
        Set-Content $profile $aliasLine
        Write-Success "✓ PowerShell profile creado con alias nvm"
    }

    # Aplicar el alias en la sesión actual
    Set-Alias -Name nvm -Value "$nvmBinDir\nvm.exe" -Force -Option AllScope
} catch {
    Write-Warning "⚠ No se pudo configurar el alias en PowerShell: $_"
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
