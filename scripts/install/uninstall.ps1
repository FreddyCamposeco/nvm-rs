# Script de desinstalación de nvm-rs para Windows
param(
    [string]$InstallDir = "$env:LOCALAPPDATA\Programs\nvm",
    [switch]$RemoveNodeVersions,
    [switch]$NoPrompt
)

$ErrorActionPreference = "Stop"

# Colores para output
function Write-Success { Write-Host $args -ForegroundColor Green }
function Write-Info { Write-Host $args -ForegroundColor Cyan }
function Write-Warning { Write-Host $args -ForegroundColor Yellow }
function Write-Error { Write-Host $args -ForegroundColor Red }

Write-Info "=== Desinstalador de nvm-rs para Windows ==="
Write-Info ""

$exePath = Join-Path $InstallDir "nvm.exe"

# Verificar si nvm está instalado
if (-not (Test-Path $exePath)) {
    Write-Error "nvm no encontrado en: $exePath"
    Write-Info ""
    Write-Info "Si nvm está instalado en otra ubicación, ejecute:"
    Write-Info "  .\uninstall.ps1 -InstallDir 'C:\ruta\personalizada'"
    exit 1
}

Write-Info "Ubicación de nvm: $exePath"

# Confirmación
if (-not $NoPrompt) {
    Write-Warning ""
    Write-Warning "Esta acción eliminará:"
    Write-Warning "  - El binario de nvm: $exePath"
    Write-Warning "  - Las variables de entorno NVM_DIR y PATH"
    if ($RemoveNodeVersions) {
        Write-Warning "  - TODAS las versiones de Node.js instaladas"
    } else {
        Write-Info ""
        Write-Info "Las versiones de Node.js instaladas NO serán eliminadas."
        Write-Info "Use -RemoveNodeVersions para eliminarlas también."
    }
    Write-Warning ""
    $response = Read-Host "¿Desea continuar? (S/N)"
    if ($response -ne 'S' -and $response -ne 's' -and $response -ne 'Y' -and $response -ne 'y') {
        Write-Info "Desinstalación cancelada"
        exit 0
    }
}

Write-Info ""
Write-Info "Desinstalando nvm-rs..."

# 1. Eliminar del PATH
Write-Info ""
Write-Info "Eliminando del PATH..."
try {
    $currentPath = [Environment]::GetEnvironmentVariable("Path", "User")
    if ($currentPath) {
        $pathEntries = $currentPath -split ';' | Where-Object { $_.Trim() -ne $InstallDir }
        $newPath = $pathEntries -join ';'
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        Write-Success "✓ Eliminado del PATH"
    }
} catch {
    Write-Warning "⚠ No se pudo actualizar el PATH: $_"
}

# 2. Eliminar variable NVM_DIR
Write-Info ""
Write-Info "Eliminando variable de entorno NVM_DIR..."
try {
    [Environment]::SetEnvironmentVariable("NVM_DIR", $null, "User")
    Write-Success "✓ Variable NVM_DIR eliminada"
} catch {
    Write-Warning "⚠ No se pudo eliminar NVM_DIR: $_"
}

# 3. Eliminar binario
Write-Info ""
Write-Info "Eliminando binario..."
try {
    Remove-Item -Path $exePath -Force
    Write-Success "✓ Binario eliminado"
    
    # Eliminar backup si existe
    $backupPath = Join-Path $InstallDir "nvm.exe.bak"
    if (Test-Path $backupPath) {
        Remove-Item -Path $backupPath -Force
        Write-Success "✓ Backup eliminado"
    }
    
    # Eliminar directorio si está vacío
    $dirContents = Get-ChildItem -Path $InstallDir -ErrorAction SilentlyContinue
    if (-not $dirContents) {
        Remove-Item -Path $InstallDir -Force
        Write-Success "✓ Directorio de instalación eliminado"
    }
} catch {
    Write-Error "✗ Error al eliminar binario: $_"
    exit 1
}

# 4. Eliminar versiones de Node.js (opcional)
if ($RemoveNodeVersions) {
    Write-Info ""
    Write-Info "Eliminando versiones de Node.js..."
    
    $nvmDir = "$env:USERPROFILE\.nvm"
    if (Test-Path $nvmDir) {
        try {
            Remove-Item -Path $nvmDir -Recurse -Force
            Write-Success "✓ Versiones de Node.js eliminadas"
        } catch {
            Write-Warning "⚠ No se pudieron eliminar las versiones: $_"
        }
    } else {
        Write-Info "No se encontraron versiones de Node.js instaladas"
    }
}

Write-Info ""
Write-Success "=== ✓ Desinstalación completada ==="
Write-Info ""
Write-Info "nvm-rs ha sido desinstalado del sistema."
Write-Warning "Es posible que necesite reiniciar su terminal para que los cambios surtan efecto."

if (-not $RemoveNodeVersions) {
    Write-Info ""
    Write-Info "Nota: Las versiones de Node.js instaladas permanecen en:"
    Write-Info "  $env:USERPROFILE\.nvm"
    Write-Info ""
    Write-Info "Para eliminarlas manualmente, ejecute:"
    Write-Info "  Remove-Item -Recurse -Force $env:USERPROFILE\.nvm"
}

Write-Info ""
Write-Info "¡Gracias por usar nvm-rs!"
