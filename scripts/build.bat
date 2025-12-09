@echo off
REM Script wrapper para build-releases.ps1
REM Permite ejecutar builds desde cmd.exe

setlocal enabledelayedexpansion

REM Parámetros por defecto
set "TARGET="
set "BUILD_TYPE=release"
set "OUTPUT_DIR=release-builds"
set "WITH_SELF_UPDATE="
set "SKIP_CLEAN="

REM Parsear argumentos
:parse_args
if "%~1"=="" goto run_build

if "%~1"=="--help" goto show_help
if "%~1"=="-h" goto show_help
if "%~1"=="--target" (
    set "TARGET=%~2"
    shift
    shift
    goto parse_args
)
if "%~1"=="--build-type" (
    set "BUILD_TYPE=%~2"
    shift
    shift
    goto parse_args
)
if "%~1"=="--output" (
    set "OUTPUT_DIR=%~2"
    shift
    shift
    goto parse_args
)
if "%~1"=="--with-self-update" (
    set "WITH_SELF_UPDATE=-WithSelfUpdate"
    shift
    goto parse_args
)
if "%~1"=="--skip-clean" (
    set "SKIP_CLEAN=-SkipClean"
    shift
    goto parse_args
)

shift
goto parse_args

:show_help
echo Build Script para nvm-rs
echo.
echo Uso: build.bat [opciones]
echo.
echo Opciones:
echo   --target ^<target^>        Target específico (windows-x64, linux-x64, etc.)
echo   --build-type ^<type^>      Tipo de build: release (default), debug
echo   --output ^<dir^>           Directorio de salida (default: release-builds)
echo   --with-self-update      Incluir capacidad de self-update
echo   --skip-clean            Saltar limpieza de build anterior
echo   -h, --help              Mostrar esta ayuda
echo.
echo Ejemplos:
echo   build.bat
echo   build.bat --target windows-x64
echo   build.bat --with-self-update
echo   build.bat --target windows-x64 --with-self-update
exit /b 0

:run_build
REM Construir comando PowerShell
set "PS_CMD=.\scripts\build-releases.ps1"

if not "!TARGET!"=="" (
    set "PS_CMD=!PS_CMD! -Target !TARGET!"
)

set "PS_CMD=!PS_CMD! -BuildType !BUILD_TYPE!"
set "PS_CMD=!PS_CMD! -OutputDir !OUTPUT_DIR!"

if not "!WITH_SELF_UPDATE!"=="" (
    set "PS_CMD=!PS_CMD! !WITH_SELF_UPDATE!"
)

if not "!SKIP_CLEAN!"=="" (
    set "PS_CMD=!PS_CMD! !SKIP_CLEAN!"
)

REM Ejecutar PowerShell
powershell -NoProfile -ExecutionPolicy Bypass -Command "& !PS_CMD!"
exit /b %ERRORLEVEL%
