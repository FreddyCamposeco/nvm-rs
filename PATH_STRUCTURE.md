# Estructura de Directorios y PATH - nvm-rs

## Comparación entre Windows y Linux/macOS

### 📁 Estructura de Directorios

#### Windows
```
C:\Users\{usuario}\
├── .nvm\                          # NVM_DIR
│   ├── bin\
│   │   └── nvm.exe               # Binario de nvm
│   ├── current\
│   │   └── bin\                  # Symlink → v{version}
│   │       ├── node.exe
│   │       ├── npm.cmd
│   │       └── npx.cmd
│   ├── v18.17.0\                 # Versión instalada
│   │   ├── node.exe
│   │   ├── npm.cmd
│   │   ├── npx.cmd
│   │   └── node_modules\
│   ├── v20.10.0\                 # Otra versión
│   │   └── ...
│   └── downloads\                # Archivos temporales
```

#### Linux/macOS
```
/home/{usuario}/  o  /Users/{usuario}/
├── .nvm/                          # NVM_DIR
│   ├── current/
│   │   └── bin/                  # Symlink → v{version}/bin
│   │       ├── node
│   │       ├── npm
│   │       └── npx
│   ├── v18.17.0/                 # Versión instalada
│   │   ├── bin/
│   │   │   ├── node
│   │   │   ├── npm
│   │   │   └── npx
│   │   └── lib/
│   ├── v20.10.0/                 # Otra versión
│   │   └── ...
│   └── downloads/                # Archivos temporales
├── .local/
│   └── bin/
│       └── nvm                   # Binario de nvm
```

---

## 🔗 Variables de Entorno

### Windows
```powershell
# Variable de entorno persistente
NVM_DIR = C:\Users\{usuario}\.nvm

# PATH del usuario incluye:
PATH = C:\Users\{usuario}\.nvm\bin;C:\Users\{usuario}\.nvm\current\bin;...
```

### Linux/macOS
```bash
# En .bashrc o .zshrc
export NVM_DIR="$HOME/.nvm"
export PATH="$HOME/.local/bin:$NVM_DIR/current/bin:$PATH"
```

---

## 📊 Tabla Comparativa

| Concepto | Windows | Linux/macOS | Homologado |
|----------|---------|-------------|------------|
| **NVM_DIR** | `%USERPROFILE%\.nvm` | `~/.nvm` | `$NVM_DIR` |
| **Binario nvm** | `%USERPROFILE%\.nvm\bin\nvm.exe` | `~/.local/bin/nvm` | N/A |
| **Versión activa** | `%NVM_DIR%\current\bin` | `$NVM_DIR/current/bin` | ✅ `$NVM_DIR/current/bin` |
| **Node instalado** | `%NVM_DIR%\v18.17.0\node.exe` | `$NVM_DIR/v18.17.0/bin/node` | `$NVM_DIR/v{version}` |
| **Separador PATH** | `;` (punto y coma) | `:` (dos puntos) | N/A |

---

## 🎯 Rutas Homologadas (Cross-Platform)

Para escribir código que funcione en ambos sistemas:

```rust
// ✅ Correcto - Usa PathBuf
let nvm_dir = dirs::home_dir()?.join(".nvm");
let current_bin = nvm_dir.join("current").join("bin");
let version_dir = nvm_dir.join("v18.17.0");

// ✅ Para acceder a binarios
#[cfg(windows)]
let node_exe = version_dir.join("node.exe");

#[cfg(not(windows))]
let node_exe = version_dir.join("bin").join("node");

// ✅ Para el symlink/junction
#[cfg(windows)]
let symlink_target = version_dir;  // Apunta a la raíz

#[cfg(not(windows))]
let symlink_target = version_dir.join("bin");  // Apunta a bin/
```

---

## 🔄 Cómo funciona el Symlink

### Windows (Junction)
```
current\bin  →  v18.17.0\
├── node.exe
├── npm.cmd
└── npx.cmd
```
- **Tipo**: Directory Junction (no requiere permisos admin)
- **PATH apunta a**: `%NVM_DIR%\current\bin`
- **Resuelve a**: `%NVM_DIR%\v18.17.0\node.exe`

### Linux/macOS (Symlink)
```
current/bin  →  v18.17.0/bin/
├── node
├── npm
└── npx
```
- **Tipo**: Symbolic Link estándar
- **PATH apunta a**: `$NVM_DIR/current/bin`
- **Resuelve a**: `$NVM_DIR/v18.17.0/bin/node`

---

## 📝 Configuración en Scripts de Instalación

### install.ps1 (Windows)
```powershell
$NvmDir = "$env:USERPROFILE\.nvm"
$NvmBin = "$NvmDir\bin"
$CurrentBin = "$NvmDir\current\bin"

# Agregar al PATH
[Environment]::SetEnvironmentVariable('NVM_DIR', $NvmDir, 'User')
# Agregar $NvmBin y $CurrentBin al PATH del usuario
```

### install.sh (Linux/macOS)
```bash
NVM_DIR="$HOME/.nvm"
NVM_BIN="$HOME/.local/bin"
CURRENT_BIN="$NVM_DIR/current/bin"

# Agregar a .bashrc o .zshrc
echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.bashrc
echo 'export PATH="$HOME/.local/bin:$NVM_DIR/current/bin:$PATH"' >> ~/.bashrc
```

---

## ✅ Ventajas de esta Estructura Homologada

1. **Consistencia**: `current/bin` existe en ambas plataformas
2. **Un solo PATH**: `$NVM_DIR/current/bin` funciona igual en ambos sistemas
3. **Aislamiento**: Las versiones de Node están separadas por carpetas
4. **Fácil cambio**: Solo se actualiza el symlink `current/bin`
5. **No conflictos**: Cada versión está autocontenida

---

## 🚀 Comandos de Usuario (Idénticos en ambos sistemas)

```bash
# Instalar nvm
nvm install-self

# Instalar Node.js
nvm install 18.17.0

# Cambiar versión activa
nvm use 18.17.0

# Ver versión actual
node --version

# Actualizar nvm
nvm update-self

# Desinstalar nvm
nvm uninstall-self
```

Todos estos comandos funcionan **exactamente igual** en Windows, Linux y macOS gracias a la estructura homologada.
