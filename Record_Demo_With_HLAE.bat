@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Record TF2 demo animation with HLAE

set "PIPELINE_ROOT=%~dp0"
set "DEMO=%~1"
if not defined DEMO goto :usage
for %%I in ("%DEMO%") do set "DEMO=%%~fI"
if not exist "%DEMO%" goto :missing
if /i not "%~x1"==".dem" goto :usage

echo.
echo This records a SHORT SFM-ready AGR from the real TF2 demo playback.
echo You will be asked for start and end demo ticks.
echo 10 seconds is about 667 ticks. Start with 300-667 ticks.
echo.
powershell.exe -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%PIPELINE_ROOT%tools\Run_HLAE_AGR_Capture.ps1" -DemoPath "%DEMO%"
if errorlevel 1 goto :fail

echo.
echo Done. Read SFM_IMPORT.md, then import sfm_import.agr.
pause
exit /b 0

:usage
echo Drag one TF2 .dem onto this BAT, or run:
echo   Record_Demo_With_HLAE.bat "C:\Demos\match.dem"
goto :fail
:missing
echo ERROR: Demo not found:
echo   %DEMO%
:fail
echo.
echo CAPTURE FAILED. The first error above is the useful one.
pause
exit /b 1

