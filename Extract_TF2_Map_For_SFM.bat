@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Prepare TF2 map for SFM

if "%~1"=="" goto :usage

set "PIPELINE_ROOT=%~dp0"
powershell.exe -NoProfile -ExecutionPolicy Bypass -File "%PIPELINE_ROOT%tools\Extract_TF2_Map_For_SFM.ps1" -MapPath "%~f1"
set "RESULT=%ERRORLEVEL%"
echo.
if not "%RESULT%"=="0" echo Extraction did not finish successfully. Read the error above.
pause
exit /b %RESULT%

:usage
echo Drag a TF2 map onto this BAT.
echo Accepted: .bsp or a server-download .bsp.bz2 file
echo.
echo It creates a sibling folder containing an SFM-compatible decompressed
echo copy of the BSP. Your original map is never modified.
pause
exit /b 1
