# nvm-rs en macOS con Apple Silicon (M1/M2/M3+)

## 🍎 Soporte Completo para Apple Silicon

nvm-rs v0.5.1 incluye soporte nativo para macOS con Apple Silicon (ARM64). El binario `nvm-v0.5.1-macos-arm64` está optimizado para máxima velocidad en procesadores M-series.

## ⚡ Características en macOS

Todas las características están completamente funcionales en macOS Apple Silicon:

- ✅ **Instalación rápida** de Node.js (arquitectura darwin-arm64)
- ✅ **Cambio automático entre versiones** con symlinks nativos
- ✅ **Soporte .nvmrc** (detección automática del árbol de directorios)
- ✅ **Auto-configuración de PATH** usando `nvm doctor --fix`
- ✅ **Filtrado inteligente** de versiones (darwin platform detection)
- ✅ **Caché eficiente** (24 horas)
- ✅ **Multiidioma** (ES/EN)

## 📦 Instalación en macOS

### Opción 1: Script automático (Recomendado)

```bash
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.sh | bash
```

El script detecta automáticamente Apple Silicon y descarga el binario correcto.

### Opción 2: Instalación manual

```bash
# Descargar binario para Apple Silicon
wget https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.1/nvm-v0.5.1-macos-arm64

# O usando curl
curl -L -o nvm-v0.5.1-macos-arm64 \
  https://github.com/FreddyCamposeco/nvm-rs/releases/download/v0.5.1/nvm-v0.5.1-macos-arm64

# Hacer ejecutable e instalar
chmod +x nvm-v0.5.1-macos-arm64
mkdir -p ~/.nvm/bin
mv nvm-v0.5.1-macos-arm64 ~/.nvm/bin/nvm
```

### Opción 3: Compilar desde código

```bash
# Clonar repositorio
git clone https://github.com/FreddyCamposeco/nvm-rs.git
cd nvm-rs

# Compilar para macOS ARM64
cargo build --target aarch64-apple-darwin --release

# Instalar binario
mkdir -p ~/.nvm/bin
cp target/aarch64-apple-darwin/release/nvm ~/.nvm/bin/
chmod +x ~/.nvm/bin/nvm
```

## 🔧 Configuración Automática

Después de instalar, ejecuta `nvm doctor --fix` para configurar automáticamente el PATH y las variables de entorno:

```bash
nvm doctor --fix
```

Esto:
1. Detecta tu shell (bash, zsh, o fish)
2. Agrega variables: `NVM_HOME`, `NVM_BIN`, `NVM_NODE`
3. Configura `PATH` para incluir nvm y Node.js activo
4. Crea un backup de tu configuración anterior

**Después**, reinicia tu terminal o ejecuta:

```bash
# Para bash/zsh
source ~/.bashrc    # o ~/.zshrc

# Para fish
source ~/.config/fish/config.fish
```

## 🎯 Uso Rápido

```bash
# Ver versión instalada
nvm --version

# Listar versiones LTS disponibles
nvm ls-remote --lts

# Instalar una versión específica
nvm install 20.10.0
nvm install lts  # Última LTS

# Cambiar a una versión
nvm use 20.10.0

# Ver versión actual
nvm current

# Listar versiones instaladas
nvm ls

# Diagnóstico completo
nvm doctor
```

## 📊 Verificación de Instalación

Para verificar que todo está funcionando correctamente:

```bash
# Verificar nvm
nvm --version       # Debería mostrar v0.5.1
nvm doctor          # Todos los checks deberían estar OK

# Verificar Node.js
node --version      # Debería mostrar la versión instalada
npm --version       # npm debe estar disponible
```

## 🚀 Características Específicas de macOS

### Detección Inteligente de Versiones Darwin

nvm-rs detecta automáticamente que estás en macOS y descarga la versión correcta:

```bash
# Esto funciona automáticamente sin necesidad de especificar "darwin"
nvm install 22.0.0         # Se descarga darwin-arm64

# Funciona con alias también
nvm install lts            # Última LTS darwin-arm64
nvm install latest/iron    # Iron LTS darwin-arm64
```

### Soporte de Shells

La instalación y configuración automática soporta:

- **bash** (~/.bashrc, ~/.bash_profile)
- **zsh** (~/.zshrc)
- **fish** (~/.config/fish/config.fish)

nvm-rs detecta tu shell predeterminado y configura el archivo correcto.

### Symlinks Nativos en macOS

nvm-rs usa symlinks Unix estándar (no requiere permisos especiales):

```bash
# Ver el symlink activo
ls -l ~/.nvm/current
# lrwxr-xr-x  1 user  staff  40 Feb  1 23:14 bin -> /Users/user/.nvm/versions/v20.18.0/bin
```

## 🔍 Troubleshooting

### nvm: command not found

Si después de instalar aún ves este error:

```bash
# Verificar que el binario está en el lugar correcto
ls -la ~/.nvm/bin/nvm

# Verificar que PATH está configurado
echo $PATH | grep ".nvm"

# Si no está, ejecutar doctor --fix nuevamente
nvm doctor --fix

# Luego reiniciar terminal
```

### "Version not found" al instalar

Verifica que estés descargando la versión correcta de darwin:

```bash
# Ver versiones remotas disponibles
nvm ls-remote | grep darwin  # Debe mostrar versiones darwin-arm64
```

### PATH no se actualiza después de `nvm doctor --fix`

```bash
# Reinicia tu shell
exit  # Salir de la sesión actual

# O recargar la configuración manualmente
# Para bash/zsh:
source ~/.bashrc

# Para fish:
source ~/.config/fish/config.fish
```

## 📚 Recursos Adicionales

- 📖 [README Principal](../README.md) - Documentación completa
- 🔧 [INSTALLATION.md](../INSTALLATION.md) - Guía de instalación detallada
- 🏗️ [BUILD_GUIDE.md](../scripts/BUILD_GUIDE.md) - Compilación desde código

## 🐛 Reportar Issues

Si encuentras problemas específicos con macOS Apple Silicon:

1. Ejecuta `nvm doctor`
2. Incluye el output en tu reporte
3. Menciona tu modelo de Mac (M1/M2/M3, etc)
4. Abre un issue en [GitHub](https://github.com/FreddyCamposeco/nvm-rs/issues)

## ✅ Certificación

| Característica | macOS x64 | macOS ARM64 |
|---|---|---|
| Instalación | ✅ | ✅ |
| Node.js install/uninstall | ✅ | ✅ |
| Cambio de versiones | ✅ | ✅ |
| .nvmrc support | ✅ | ✅ |
| Aliases | ✅ | ✅ |
| Stats & Doctor | ✅ | ✅ |
| Auto-PATH config | ✅ | ✅ |
| Multiidioma | ✅ | ✅ |

**Última verificación**: Febrero 1, 2026
**Versión**: v0.5.1
**Estado**: ✅ Completamente Funcional
