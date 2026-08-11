@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Configure HLAE for TF2 SFM capture
set "PIPELINE_ROOT=%~dp0"

echo.
echo TF2 STV to SFM - HLAE setup
echo.
echo Required downloads:
echo   HLAE 2.189.0 or newer:
echo   https://github.com/advancedfx/advancedfx/releases/latest
echo.
echo   AdvancedFX SFM import scripts:
echo   https://github.com/advancedfx/afx-sfm-scripts
echo.
echo HLAE is technically a hook. This pipeline always launches TF2 with -insecure,
echo uses playdemo only, and never issues connect or matchmaking commands.
echo Never join a server from an HLAE-launched TF2 window.
echo.

set "HLAE_EXE=%~1"
if not defined HLAE_EXE if exist "%PIPELINE_ROOT%HLAE_PATH.txt" set /p HLAE_EXE=<"%PIPELINE_ROOT%HLAE_PATH.txt"
if not defined HLAE_EXE if exist "C:\HLAE\HLAE.exe" set "HLAE_EXE=C:\HLAE\HLAE.exe"
if not defined HLAE_EXE if exist "%ProgramFiles(x86)%\HLAE\HLAE.exe" set "HLAE_EXE=%ProgramFiles(x86)%\HLAE\HLAE.exe"

if not defined HLAE_EXE (
  start "" "https://github.com/advancedfx/advancedfx/releases/latest"
  echo Install HLAE, then paste its executable path below.
  set /p "HLAE_EXE=Full path to HLAE.exe: "
)
if not exist "%HLAE_EXE%" goto :missing

for %%I in ("%HLAE_EXE%") do (
  set "HLAE_EXE=%%~fI"
  set "HLAE_DIR=%%~dpI"
)
set "HOOK_FOUND="
for /f "usebackq delims=" %%I in (`powershell.exe -NoProfile -NonInteractive -Command "$hook = Get-ChildItem -LiteralPath $env:HLAE_DIR -Filter AfxHookSource.dll -File -Recurse -ErrorAction SilentlyContinue ^| Where-Object FullName -Match '\\x64\\' ^| Select-Object -First 1 -ExpandProperty FullName; if ($hook) { $hook }"`) do if not defined HOOK_FOUND set "HOOK_FOUND=%%I"
if not defined HOOK_FOUND goto :old_hlae

>"%PIPELINE_ROOT%HLAE_PATH.txt" echo %HLAE_EXE%
echo.
echo HLAE CONFIGURATION PASSED
echo HLAE: %HLAE_EXE%
echo Hook: %HOOK_FOUND%
echo.
echo Next:
echo   1. Run Build_All.bat once to build the parser.
echo   2. Drag a demo onto Process_STV_To_SFM.bat.
echo   3. Enter a short start/end tick range; 10 seconds is about 667 ticks.
pause
exit /b 0

:missing
echo.
echo ERROR: HLAE.exe was not found at:
echo   %HLAE_EXE%
goto :fail
:old_hlae
echo.
echo ERROR: No x64\AfxHookSource.dll was found under:
echo   %HLAE_DIR%
echo Install HLAE 2.189.0 or newer from the official release page.
:fail
echo.
pause
exit /b 1
