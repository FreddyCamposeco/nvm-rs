# Guía de Pruebas de Instalación/Desinstalación

Esta guía te ayudará a probar las funcionalidades de instalación y desinstalación de nvm-rs localmente.

## ✅ Implementación Completada

### Nuevas Funcionalidades

1. **Gestión de Variables de Entorno**
   - ✅ Creación automática de `NVM_DIR`
   - ✅ Adición automática al `PATH`
   - ✅ Reutilización de variables existentes
   - ✅ Eliminación de variables al desinstalar

2. **Scripts Mejorados**
   - ✅ `install.ps1` - Instalación para Windows con variables de entorno
   - ✅ `install.sh` - Instalación para Linux/macOS con configuración shell
   - ✅ `uninstall.ps1` - Desinstalación para Windows con limpieza de variables
   - ✅ `uninstall.sh` - Desinstalación para Linux/macOS con limpieza de configs

3. **Comandos Integrados**
   - ✅ `nvm install-self` - Instala y configura variables automáticamente
   - ✅ `nvm uninstall-self` - Desinstala y limpia variables automáticamente
   - ✅ `nvm update-self` - Actualiza sin afectar las variables

## 🧪 Pruebas Locales

### Compilar el Proyecto

```powershell
# Compilar en modo release
cargo build --release

# El binario estará en: target\release\nvm.exe (Windows) o target/release/nvm (Unix)
```

### Verificar Binario

```powershell
# Ver versión
.\target\release\nvm.exe --version

# Ver ayuda
.\target\release\nvm.exe --help

# Probar comando doctor
.\target\release\nvm.exe doctor
```

### Prueba 1: Verificar Estado Inicial

```powershell
# Windows PowerShell
Write-Host "=== Estado Inicial ==="
Write-Host "NVM_DIR: $env:NVM_DIR"
Write-Host "PATH: $env:PATH"

# Verificar variables permanentes del usuario
[Environment]::GetEnvironmentVariable("NVM_DIR", "User")
[Environment]::GetEnvironmentVariable("Path", "User")
```

```bash
# Linux/macOS
echo "=== Estado Inicial ==="
echo "NVM_DIR: $NVM_DIR"
echo "PATH: $PATH"
cat ~/.bashrc | grep nvm
cat ~/.zshrc | grep nvm
```

### Prueba 2: Instalación con Script

#### Windows

```powershell
# Prueba en directorio personalizado
$TestDir = "$env:TEMP\nvm-test"
.\install.ps1 -InstallDir $TestDir -NoPrompt

# Verificar instalación
Get-ChildItem $TestDir
[Environment]::GetEnvironmentVariable("NVM_DIR", "User")
[Environment]::GetEnvironmentVariable("Path", "User")
```

#### Linux/macOS

```bash
# Prueba en directorio personalizado
export NVM_INSTALL_DIR="$HOME/test-nvm"
./install.sh

# Verificar instalación
ls -la $HOME/test-nvm
cat ~/.bashrc | grep -A2 "nvm-rs"
```

### Prueba 3: Comando install-self

```powershell
# Windows - Instalación en directorio de prueba
.\target\release\nvm.exe install-self --dir "$env:TEMP\nvm-install-test"

# Verificar:
# 1. Binario instalado
Test-Path "$env:TEMP\nvm-install-test\nvm.exe"

# 2. Variables de entorno configuradas
[Environment]::GetEnvironmentVariable("NVM_DIR", "User")
$path = [Environment]::GetEnvironmentVariable("Path", "User")
$path -like "*nvm-install-test*"
```

```bash
# Linux/macOS
./target/release/nvm install-self --dir "$HOME/test-nvm-install"

# Verificar:
ls -la $HOME/test-nvm-install/nvm
cat ~/.bashrc | grep -A2 "nvm-rs"
```

### Prueba 4: Desinstalación con Script

#### Windows

```powershell
# Desinstalar con script
.\uninstall.ps1 -InstallDir "$env:TEMP\nvm-test" -NoPrompt

# Verificar limpieza:
# 1. Binario eliminado
Test-Path "$env:TEMP\nvm-test\nvm.exe"  # Debe retornar False

# 2. Variables eliminadas
[Environment]::GetEnvironmentVariable("NVM_DIR", "User")  # Debe estar vacío
$path = [Environment]::GetEnvironmentVariable("Path", "User")
$path -notlike "*nvm-test*"  # Debe retornar True
```

#### Linux/macOS

```bash
# Desinstalar con script
export NVM_INSTALL_DIR="$HOME/test-nvm"
./uninstall.sh

# Verificar limpieza
ls $HOME/test-nvm/nvm  # No debe existir
cat ~/.bashrc | grep nvm  # No debe aparecer
```

### Prueba 5: Comando uninstall-self

```powershell
# Windows
.\target\release\nvm.exe uninstall-self --dir "$env:TEMP\nvm-install-test" --yes

# Verificar limpieza completa
Test-Path "$env:TEMP\nvm-install-test\nvm.exe"
[Environment]::GetEnvironmentVariable("NVM_DIR", "User")
[Environment]::GetEnvironmentVariable("Path", "User") | Select-String "nvm-install-test"
```

```bash
# Linux/macOS
./target/release/nvm uninstall-self --dir "$HOME/test-nvm-install" --yes

# Verificar limpieza
ls $HOME/test-nvm-install/nvm
cat ~/.bashrc | grep nvm
```

## 🔍 Verificaciones Importantes

### Variables de Entorno (Windows)

1. **NVM_DIR debe apuntar a `%USERPROFILE%\.nvm`**
   ```powershell
   [Environment]::GetEnvironmentVariable("NVM_DIR", "User")
   # Esperado: C:\Users\<usuario>\.nvm
   ```

2. **PATH debe contener el directorio de instalación**
   ```powershell
   $path = [Environment]::GetEnvironmentVariable("Path", "User")
   $path -split ';' | Where-Object { $_ -like '*nvm*' }
   ```

3. **Variables deben persistir entre sesiones**
   ```powershell
   # Abrir nueva ventana de PowerShell
   Write-Host $env:NVM_DIR
   ```

### Configuración Shell (Linux/macOS)

1. **Archivo shell debe contener exportaciones**
   ```bash
   cat ~/.bashrc | grep -A3 "nvm-rs"
   # Esperado:
   # # nvm-rs configuration
   # export NVM_DIR="$HOME/.nvm"
   # export PATH="$HOME/.local/bin:$PATH"
   ```

2. **Variables deben estar disponibles en nuevas sesiones**
   ```bash
   # Abrir nueva terminal
   echo $NVM_DIR
   echo $PATH | grep nvm
   ```

## 📊 Checklist de Funcionalidades

### Instalación
- [ ] Script `install.ps1` funciona correctamente
- [ ] Script `install.sh` funciona correctamente
- [ ] Comando `install-self` instala el binario
- [ ] Variable `NVM_DIR` se crea automáticamente
- [ ] Directorio de instalación se agrega al `PATH`
- [ ] Variables existentes se reutilizan sin duplicar
- [ ] Confirmación interactiva funciona
- [ ] Opción `--NoPrompt` / sin TTY funciona
- [ ] Backup de binarios anteriores funciona

### Desinstalación
- [ ] Script `uninstall.ps1` funciona correctamente
- [ ] Script `uninstall.sh` funciona correctamente
- [ ] Comando `uninstall-self` elimina el binario
- [ ] Variable `NVM_DIR` se elimina correctamente
- [ ] Directorio se elimina del `PATH`
- [ ] Configuraciones shell se limpian (Unix)
- [ ] Backups de configs se crean (Unix)
- [ ] Confirmación interactiva funciona
- [ ] Opción `--yes` / `--NoPrompt` funciona
- [ ] Versiones de Node.js no se eliminan por defecto

### Actualización
- [ ] Comando `update-self` funciona
- [ ] Variables de entorno se mantienen
- [ ] Backup del binario anterior se crea
- [ ] Versión se actualiza correctamente

## 🚨 Casos de Borde a Probar

1. **Variables ya existen**
   - Instalar cuando `NVM_DIR` ya está configurado
   - Instalar cuando el directorio ya está en `PATH`
   - Verificar que no se duplican

2. **Permisos**
   - Instalar sin permisos de administrador (Windows)
   - Instalar en directorio sin permisos de escritura
   - Verificar mensajes de error apropiados

3. **Desinstalación parcial**
   - Desinstalar cuando solo existe el binario
   - Desinstalar cuando solo existen las variables
   - Desinstalar cuando hay versiones de Node.js instaladas

4. **Múltiples instalaciones**
   - Instalar en diferentes directorios
   - Verificar que las variables apuntan a la última instalación
   - Desinstalar selectivamente

## 📝 Notas

- Las variables de entorno en Windows se persisten en el registro
- En Unix, las configuraciones se guardan en archivos shell
- Los cambios de `PATH` requieren reiniciar la terminal
- Los backups se crean automáticamente antes de cambios importantes
- Las versiones de Node.js instaladas no se eliminan por defecto

## 🎯 Próximos Pasos

Para probar con GitHub releases reales:
1. Compilar el binario en release mode
2. Crear un release en GitHub con el binario
3. Usar el tag correcto (ej: v0.1.0)
4. Probar la instalación desde el release público
5. Verificar checksums SHA256

## 💡 Comandos Útiles

```powershell
# Windows - Ver todas las variables de entorno del usuario
[Environment]::GetEnvironmentVariables("User")

# Windows - Limpiar variables de prueba
[Environment]::SetEnvironmentVariable("NVM_DIR", $null, "User")

# Unix - Ver exports en archivos shell
grep -n "export.*nvm\|export.*NVM" ~/.bashrc ~/.zshrc ~/.profile

# Unix - Recargar configuración
source ~/.bashrc
```
