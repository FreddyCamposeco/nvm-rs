@echo off
echo Testing nvm internationalization with YAML
echo.

REM Change to project root directory (parent of scripts)
cd /d "%~dp0.."

echo Setting language to English:
set NVM_LANG=en
nvm --help
echo.

echo Setting language to Spanish:
set NVM_LANG=es
nvm --help
echo.

echo Testing lang command:
nvm lang en
nvm lang es