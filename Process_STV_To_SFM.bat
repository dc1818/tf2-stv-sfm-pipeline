@echo off
setlocal EnableExtensions DisableDelayedExpansion
title TF2 STV to SFM - retail HLAE capture

set "PIPELINE_ROOT=%~dp0"
set "DEMO=%~1"
if not defined DEMO goto :usage
for %%I in ("%DEMO%") do (
  set "DEMO=%%~fI"
  set "OUTPUT_BASE=%%~dpnI_sfm_project"
)
if not exist "%DEMO%" goto :missing_demo
if /i not "%~x1"==".dem" goto :usage

for /f "usebackq delims=" %%I in (`powershell -NoProfile -Command "Get-Date -Format yyyyMMdd_HHmmss"`) do set "RUN_STAMP=%%I"
set "OUTPUT=%OUTPUT_BASE%_%RUN_STAMP%"
set "EXPORTER=%PIPELINE_ROOT%parser\target\release\export_all.exe"
set "HLAE_RUNNER=%PIPELINE_ROOT%tools\Run_HLAE_AGR_Capture.ps1"
set "FINALIZER=%PIPELINE_ROOT%tools\Finalize_HLAE_Project.ps1"

if not exist "%EXPORTER%" goto :missing_exporter
if not exist "%HLAE_RUNNER%" goto :missing_runner
if not exist "%FINALIZER%" goto :missing_runner
if not exist "%OUTPUT%" mkdir "%OUTPUT%"
if errorlevel 1 goto :fail

echo.
echo [1/3] Parsing all STV packets and resolved event/state data...
"%EXPORTER%" "%DEMO%" "%OUTPUT%"
if errorlevel 1 goto :fail

echo.
echo [2/3] Recording the selected tick range with retail TF2 plus HLAE...
echo This is the animation-faithful stage. It plays only the copied local demo.
echo 10 seconds is about 667 TF2 ticks. Start with a short range for SFM.
powershell.exe -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%HLAE_RUNNER%" -DemoPath "%DEMO%" -ProjectDirectory "%OUTPUT%"
if errorlevel 1 goto :capture_failed

echo.
echo [3/3] Combining parser metadata and the HLAE capture manifest...
powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%FINALIZER%" -ProjectDirectory "%OUTPUT%" -SourceDemo "%DEMO%"
if errorlevel 1 goto :fail

echo.
echo SUCCESS
echo Project: %OUTPUT%
echo SFM AGR: %OUTPUT%\sfm_import.agr
echo.
echo This AGR came from the actual retail TF2 client animation state.
echo Read SFM_IMPORT.md for the import steps.
pause
exit /b 0

:usage
echo Drag one TF2 STV .dem onto this BAT, or run:
echo   Process_STV_To_SFM.bat "C:\Demos\match.dem"
goto :fail
:missing_demo
echo ERROR: Demo not found:
echo   %DEMO%
goto :fail
:missing_exporter
echo ERROR: Missing parser exporter:
echo   %EXPORTER%
echo Run Build_Parser_Only.bat.
goto :fail
:missing_runner
echo ERROR: The HLAE capture scripts are missing. Extract the complete updated bundle.
goto :fail
:capture_failed
echo.
echo ERROR: The retail HLAE animation capture failed.
echo Project diagnostics: %OUTPUT%
echo Search tf2_console.log for TF2SFM_, mirv_agr, or Unknown command.
:fail
echo.
echo PIPELINE FAILED. The first error above is the useful one.
pause
exit /b 1

