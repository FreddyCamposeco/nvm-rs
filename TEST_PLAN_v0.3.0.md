# Plan de Pruebas: nvm-rs v0.3.0

## 🎯 Objetivo

Validar que:

1. La instalación de nvm funciona correctamente
2. Las variables de entorno se configuran
3. La desinstalación limpia todos los rastros
4. El sistema queda limpio como si nvm nunca se hubiera instalado

## 📋 Flujo de Pruebas

### Fase 1: Verificación Previa

```powershell
# 1. Verificar que nvm NO está instalado previamente
Get-ChildItem Env:NVM_* -ErrorAction SilentlyContinue
# Esperado: Vacío (no debe haber variables NVM_*)

# 2. Verificar que nvm.exe no está en el PATH
Get-Command nvm -ErrorAction SilentlyContinue
# Esperado: Error - nvm no encontrado
```

### Fase 2: Instalación

```powershell
# 1. Instalar nvm
cd d:\Elementum\repo\nvm-rs
.\target\release\nvm.exe install-self -y

# 2. Verificar que el binario se instaló
Get-Command nvm
# Esperado: Ruta al nvm.exe instalado
```

### Fase 3: Configuración de Variables

```powershell
# 1. Verificar que las variables se establecieron
Get-ChildItem Env:NVM_HOME
Get-ChildItem Env:NVM_BIN
Get-ChildItem Env:NVM_NODE
Get-ChildItem Env:NODE_MIRROR

# Esperado: Todas las variables deben existir

# 2. Verificar que PATH incluye NVM_BIN
Write-Host $ENV:PATH

# Esperado: Debe incluir la ruta a NVM_BIN
```

### Fase 4: Operaciones Básicas

```powershell
# 1. Instalar una versión LTS
nvm install lts
# Esperado: Descarga e instala Node.js LTS

# 2. Cambiar a esa versión
nvm use lts
# Esperado: Activa la versión LTS

# 3. Verificar que node funciona
node --version
npm --version

# Esperado: Versiones correctas mostradas
```

### Fase 5: Desinstalación Completa

```powershell
# 1. Ejecutar desinstalación
nvm uninstall-self -y
# Esperado: Proceso de limpieza completado con mensajes de progreso

# 2. Verificar que nvm.exe fue eliminado
Get-Command nvm -ErrorAction SilentlyContinue
# Esperado: Error - nvm no encontrado
```

### Fase 6: Verificación de Limpieza

```powershell
# 1. Verificar variables de entorno
Get-ChildItem Env:NVM_HOME -ErrorAction SilentlyContinue
Get-ChildItem Env:NVM_BIN -ErrorAction SilentlyContinue
Get-ChildItem Env:NVM_NODE -ErrorAction SilentlyContinue
Get-ChildItem Env:NODE_MIRROR -ErrorAction SilentlyContinue

# Esperado: TODAS deben no existir (sin errores silenciados = OK)

# 2. Verificar PATH
Write-Host $ENV:PATH | Select-String "nvm"
# Esperado: No debe encontrar referencias a nvm

# 3. Verificar directorios de datos
ls $HOME\.nvm -ErrorAction SilentlyContinue
# Esperado: Dirección no existe o está vacía

# 4. Verificar directorios de instalación
ls "C:\Program Files (x86)\nvm" -ErrorAction SilentlyContinue
# Esperado: Directorio no existe
```

### Fase 7: Prueba de Limpidez Completa

```powershell
# Crear un hash de estado antes de instalación
$pre_install = @{
    'env_vars' = (Get-ChildItem Env: | Select-Object -Property Name, Value)
    'path' = $ENV:PATH
}

# Después de desinstalación
$post_uninstall = @{
    'env_vars' = (Get-ChildItem Env: | Select-Object -Property Name, Value)
    'path' = $ENV:PATH
}

# Comparar (deben ser iguales)
if ($pre_install.path -eq $post_uninstall.path) {
    Write-Host "✅ PATH está limpio" -ForegroundColor Green
} else {
    Write-Host "❌ PATH contiene rastros de nvm" -ForegroundColor Red
}
```

## 📊 Matriz de Verificación

| Paso | Verificación | Estado | Notas |
|------|-------------|--------|-------|
| 1. Pre-instalación | Variables vacías | ⏳ | |
| 2. Instalación | nvm.exe instalado | ⏳ | |
| 3. Variables ENV | Todas presentes | ⏳ | |
| 4. PATH | Actualizado | ⏳ | |
| 5. Node.js | Funciona | ⏳ | |
| 6. Desinstalación | Sin errores | ⏳ | |
| 7. Limpieza vars | Variables ausentes | ⏳ | |
| 8. Limpieza PATH | Sin referencias nvm | ⏳ | |
| 9. Limpieza directorios | Eliminados | ⏳ | |

## ✅ Criterios de Éxito

1. ✅ Instalación se completa sin errores
2. ✅ Todas las variables se establecen
3. ✅ Node.js funciona correctamente después de la instalación
4. ✅ Desinstalación se completa sin errores
5. ✅ Todas las variables se eliminan
6. ✅ PATH está limpio (sin referencias a nvm)
7. ✅ Directorios de datos se eliminan
8. ✅ Sistema está "como nuevo" - nvm completamente removido

## 🐛 Posibles Problemas y Soluciones

### Problema: Variables no se eliminan

- **Causa**: PowerShell requiere permisos de usuario
- **Solución**: Ejecutar terminal como administrador

### Problema: PATH no se limpia

- **Causa**: Cambios en PATH requieren reinicio
- **Solución**: Abrir nueva terminal después de desinstalación

### Problema: Directorios no se eliminan

- **Causa**: Archivos abiertos o permisos
- **Solución**: Cerrar terminal y archivos abiertos

## 📝 Notas

- Las pruebas deben ejecutarse en PowerShell Core (v7+) en Windows
- Se requieren permisos de usuario para modificar variables de entorno
- Reiniciar terminal después de cambios en variables de entorno
- Documentar cualquier discrepancia encontrada
