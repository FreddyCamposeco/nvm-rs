# Estructura de Directorios y PATH - nvm-rs

## Comparación entre Windows y Linux/macOS

### 📁 Estructura de Directorios

#### Windows

```
C:\Users\{usuario}\
├── .nvm\                         # NVM_HOME
│   ├── bin\
│   │   └── nvm.exe               # Binario nvm ($NVM_BIN)
│   ├── current\
│   │   └── bin\                  # Junction → v{version}\ ($NVM_NODE)
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
│   └── temp\                     # Descargas temporales
```

#### Linux/macOS

```
/home/{usuario}/  o  /Users/{usuario}/
├── .nvm/                         # NVM_HOME
│   ├── bin/
│   │   └── nvm                   # Binario nvm ($NVM_BIN)
│   ├── current/
│   │   └── bin/                  # Symlink → v{version}/bin ($NVM_NODE)
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
│   └── temp/                     # Descargas temporales
```

---

## 🔗 Variables de Entorno

### Windows

```powershell
# Variables de entorno persistentes
NVM_HOME = %USERPROFILE%\.nvm
NVM_BIN = %NVM_HOME%\bin
NVM_NODE = %NVM_HOME%\current\bin

# PATH del usuario incluye (en orden):
PATH = %NVM_HOME%\bin;%NVM_HOME%\current\bin;...
```

### Linux/macOS

```bash
# En .bashrc o .zshrc
export NVM_HOME="$HOME/.nvm"
export NVM_BIN="$NVM_HOME/bin"
export NVM_NODE="$NVM_HOME/current/bin"
export PATH="$NVM_HOME/bin:$NVM_HOME/current/bin:$PATH"
```

---

## 📊 Tabla Comparativa

| Concepto | Windows | Linux/macOS | Homologado | Variable |
|----------|---------|-------------|------------|----------|
| **nvm home** | `%USERPROFILE%\.nvm` | `~/.nvm` | `$NVM_HOME` | `NVM_HOME` |
| **binario nvm** | `%NVM_HOME%\bin\nvm.exe` | `$NVM_HOME/bin/nvm` | `$NVM_BIN` | `NVM_BIN` |
| **node activo** | `%NVM_HOME%\current\bin` | `$NVM_HOME/current/bin` | `$NVM_NODE` | `NVM_NODE` |
| **Node instalado** | `%NVM_HOME%\v{version}\*` | `$NVM_HOME/v{version}/bin/*` | `$NVM_HOME/v{version}` | N/A |
| **Separador PATH** | `;` (punto y coma) | `:` (dos puntos) | Según SO | N/A |

---

## 🎯 Rutas Homologadas (Cross-Platform)

Para escribir código que funcione en ambos sistemas:

```rust
// ✅ Rutas homologadas - Usa PathBuf
let nvm_home = home::home_dir()?.join(".nvm");
let nvm_bin = nvm_home.join("bin");         // $NVM_BIN
let nvm_node = nvm_home.join("current").join("bin");  // Construye ruta equivalente a $NVM_NODE
let version_dir = nvm_home.join("v18.17.0"); // $NVM_HOME/v{version}

// ✅ Para acceder a binarios de versión
#[cfg(windows)]
let node_exe = version_dir.join("node.exe");

#[cfg(not(windows))]
let node_exe = version_dir.join("bin").join("node");

// ✅ Para el symlink/junction activo
#[cfg(windows)]
let symlink_target = version_dir;  // Junction: current\bin → v{version}\

#[cfg(not(windows))]
let symlink_target = version_dir.join("bin");  // Symlink: current/bin → v{version}/bin
```

---

## 🔄 Cómo funciona el Symlink

### Windows (Junction)

```
%NVM_HOME%\current\bin  →  %NVM_HOME%\v18.17.0\
├── node.exe
├── npm.cmd
└── npx.cmd
```

- **Tipo**: Directory Junction (no requiere permisos admin)
- **Variable**: `$NVM_NODE = %NVM_HOME%\current\bin`
- **Destino**: `%NVM_HOME%\v{version}\` (raíz de versión)
- **Resolución**: `%NVM_HOME%\v18.17.0\node.exe`

### Linux/macOS (Symlink)

```
$NVM_HOME/current/bin  →  $NVM_HOME/v18.17.0/bin/
├── node
├── npm
└── npx
```

- **Tipo**: Symbolic Link estándar
- **Variable**: `$NVM_NODE = $NVM_HOME/current/bin`
- **Destino**: `$NVM_HOME/v{version}/bin/` (carpeta bin de versión)
- **Resolución**: `$NVM_HOME/v18.17.0/bin/node`

---

## 📝 Configuración en Scripts de Instalación

### install.ps1 (Windows)

```powershell
# Definir variables homologadas
$NvmHome = "$env:USERPROFILE\.nvm"
$NvmBin = "$NvmHome\bin"
$NvmNode = "$NvmHome\current\bin"

# Agregar variables de entorno persistentes
[Environment]::SetEnvironmentVariable('NVM_HOME', $NvmHome, 'User')
[Environment]::SetEnvironmentVariable('NVM_BIN', $NvmBin, 'User')
[Environment]::SetEnvironmentVariable('NVM_NODE', $NvmNode, 'User')

# Actualizar PATH (agregar NVM_BIN y NVM_NODE)
$currentPath = [Environment]::GetEnvironmentVariable('PATH', 'User')
$newPath = "$NvmBin;$NvmNode;$currentPath"
[Environment]::SetEnvironmentVariable('PATH', $newPath, 'User')
```

### install.sh (Linux/macOS)

```bash
# Definir variables homologadas
NVM_HOME="$HOME/.nvm"
NVM_BIN="$NVM_HOME/bin"
NVM_NODE="$NVM_HOME/current/bin"

# Agregar a .bashrc o .zshrc
echo 'export NVM_HOME="$HOME/.nvm"' >> ~/.bashrc
echo 'export NVM_BIN="$NVM_HOME/bin"' >> ~/.bashrc
echo 'export NVM_NODE="$NVM_HOME/current/bin"' >> ~/.bashrc
echo 'export PATH="$NVM_HOME/bin:$NVM_HOME/current/bin:$PATH"' >> ~/.bashrc
```

---

## ✅ Ventajas de esta Estructura Homologada

1. **Consistencia**: `current/bin` existe en ambas plataformas
2. **Un solo PATH**: `$NVM_HOME/current/bin` funciona igual en ambos sistemas
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
