# Comando `nvm doctor` - Diagnóstico y Auto-Fix

## Descripción General

El comando `nvm doctor` verifica la instalación de nvm y ayuda a diagnosticar problemas. En Unix, incluye una opción `--fix` para auto-configurar el PATH y las variables de entorno.

## Uso Básico

```bash
# Verificación rápida
nvm doctor

# Auto-configurar PATH y variables (Unix)
nvm doctor --fix

# Mostrar solo Node.js del sistema (no NVM-managed)
nvm doctor --system

# Mostrar todas las instalaciones encontradas
nvm doctor --all
```

## Salida de `nvm doctor`

```
nvm - Doctor Information
==================================================
✓ NVM Directory: : /Users/freddy/.nvm
✓ Installed versions:: 2
Connectivity to nodejs.org: OK
Symlink support: Supported
NVM env & PATH OK
```

### Explicación de los Checks

1. **NVM Directory** - Verifica que el directorio NVM_HOME existe
2. **Installed versions** - Cuenta cuántas versiones de Node.js están instaladas
3. **Connectivity to nodejs.org** - Comprueba acceso a internet (para descargas)
4. **Symlink support** - Verifica soporte de symlinks
   - En Unix: Siempre OK
   - En Windows: Requiere permisos especiales o modo desarrollador
5. **NVM env & PATH** - Verifica que las variables de entorno están configuradas

## Opción `--fix` (Unix)

Disponible en Linux y macOS. Auto-configura:

- `NVM_HOME` - Directorio base de nvm
- `NVM_BIN` - Ubicación del binario nvm
- `NVM_NODE` - Directorio del Node.js activo
- `PATH` - Agrega `$NVM_BIN` y `$NVM_NODE`

### Detección Automática de Shell

`doctor --fix` detecta tu shell y configura el archivo correcto:

- **bash** → Modifica `~/.bashrc` o `~/.bash_profile`
- **zsh** → Modifica `~/.zshrc`
- **fish** → Modifica `~/.config/fish/config.fish`

### Cómo Funciona

```bash
nvm doctor --fix
```

1. Detecta tu shell del PATH (`$SHELL`)
2. Lee tu archivo de configuración actual (si existe)
3. Agrega bloque nvm-rs:

```bash
# >>> nvm-rs >>>
export NVM_HOME="/Users/user/.nvm"
export NVM_BIN="$NVM_HOME/bin"
export NVM_NODE="$NVM_HOME/current/bin"
export PATH="$NVM_BIN:$NVM_NODE:$PATH"
# <<< nvm-rs <<<
```

4. Escribe los cambios en el archivo
5. Muestra confirmación

### Después de `--fix`

**Reinicia tu terminal** para aplicar los cambios:

```bash
# Opción 1: Salir y volver a entrar
exit

# Opción 2: Recargar configuración manualmente
# Para bash/zsh:
source ~/.bashrc    # o ~/.zshrc

# Para fish:
source ~/.config/fish/config.fish
```

## Opción `--system`

Detecta y muestra Node.js instalado en el sistema (fuera de NVM):

```bash
nvm doctor --system

# Ejemplo de output:
# 🔍 System Node.js Found:
#    Version: v18.19.0
#    Path: /usr/local/bin/node
#    npm: 9.8.1
```

Busca en ubicaciones comunes:

- **Windows**: `C:\Program Files\nodejs`, `C:\Program Files (x86)\nodejs`
- **macOS**: `/usr/local/bin`, `/opt/homebrew/bin`
- **Linux**: `/usr/local/bin`, `/usr/bin`

## Opción `--all`

Encuentra todas las instalaciones de Node.js en el sistema:

```bash
nvm doctor --all

# Ejemplo de output:
# 📍 All Node.js Installations (3):
#    1. v20.18.0 @ /Users/user/.nvm/versions/v20.18.0
#    2. v22.12.0 @ /Users/user/.nvm/versions/v22.12.0
#    3. v18.19.0 @ /usr/local/bin/node
```

## Casos de Uso

### Instalación Nueva

Después de instalar nvm por primera vez:

```bash
# 1. Instalar nvm
nvm install-self

# 2. Verificar instalación
nvm doctor

# 3. Auto-configurar (Unix)
nvm doctor --fix

# 4. Reiniciar terminal
exit
```

### Troubleshooting

Si `nvm` no funciona después de instalar:

```bash
# 1. Ejecutar doctor para ver qué está mal
nvm doctor

# 2. Ver output detallado
# Si dice "NVM env & PATH Missing or incomplete":
nvm doctor --fix

# 3. Reiniciar terminal
```

### Verificar Conflictos

Si tienes múltiples instalaciones de Node.js:

```bash
# Ver todas las instalaciones
nvm doctor --all

# Mostrar solo sistema (no NVM)
nvm doctor --system

# Cambiar versión activa
nvm use 20.18.0
```

### Después de Actualizar nvm

```bash
# Verificar que todo sigue funcionando
nvm doctor

# Reconfigurar si es necesario
nvm doctor --fix
```

## Variables Configuradas por `--fix`

| Variable | Significado | Ejemplo |
|----------|-------------|---------|
| `NVM_HOME` | Directorio base de nvm | `/Users/user/.nvm` |
| `NVM_BIN` | Ubicación binario nvm | `$NVM_HOME/bin` |
| `NVM_NODE` | Node.js activo en PATH | `$NVM_HOME/current/bin` |
| `PATH` | Incluye NVM_BIN y NVM_NODE | `$NVM_BIN:$NVM_NODE:...` |

## Estructura de Configuración en Shell

El bloque agregado por `nvm doctor --fix`:

```bash
# >>> nvm-rs >>>
# ... variables de nvm-rs ...
# <<< nvm-rs <<<
```

- Está marcado con comentarios de inicio/fin para fácil identificación
- Se puede remover manualmente buscando estos comentarios
- Si ya existe, `--fix` no lo duplica

## Compatibilidad

| Sistema | `doctor` | `--fix` | `--system` | `--all` |
|---------|----------|---------|-----------|---------|
| Windows | ✅ | ❌ | ✅ | ✅ |
| Linux | ✅ | ✅ | ✅ | ✅ |
| macOS | ✅ | ✅ | ✅ | ✅ |

## Problemas Comunes

### "NVM env & PATH Missing or incomplete"

```bash
# Ejecutar doctor --fix
nvm doctor --fix

# Reiniciar terminal
exit
```

### Cambios no se aplican después de `--fix`

```bash
# Opción 1: Salir y volver a entrar
exit

# Opción 2: Recargar manualmente
# Bash:
source ~/.bashrc

# Zsh:
source ~/.zshrc

# Fish:
source ~/.config/fish/config.fish
```

### Shell no detectado correctamente

```bash
# Ver qué shell estás usando
echo $SHELL

# Cambiar de shell permanentemente
chsh -s /bin/zsh        # para zsh
chsh -s /bin/bash       # para bash
chsh -s /opt/homebrew/bin/fish  # para fish
```

## Ejemplos Completos

### Instalación desde Cero en macOS

```bash
# 1. Descargar e instalar
curl -fsSL https://raw.githubusercontent.com/FreddyCamposeco/nvm-rs/main/scripts/install/install.sh | bash

# 2. Verificar que está bien
nvm doctor

# 3. Auto-configurar
nvm doctor --fix

# 4. Reiniciar terminal
exit

# 5. Verificar nuevamente (todo debe estar OK)
nvm doctor
```

### Verificar Múltiples Versiones

```bash
# Ver todas las versiones instaladas
nvm ls

# Ver todas las instalaciones en el sistema
nvm doctor --all

# Ver solo sistema (no NVM)
nvm doctor --system

# Activar una versión NVM
nvm use 20.18.0

# Verificar
nvm current
```

## Referencia Rápida

```bash
nvm doctor              # Diagnóstico básico
nvm doctor --fix        # Auto-configurar (Unix)
nvm doctor --system     # Mostrar Node.js del sistema
nvm doctor --all        # Mostrar todas las instalaciones
```

---

**Última actualización**: Febrero 2, 2026  
**Versión**: v0.5.1
