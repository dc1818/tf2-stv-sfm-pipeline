@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Mount current TF2 content into SFM
echo.
echo This adds the live TF2 tf folder to SFM's search paths.
echo It does not copy or overwrite TF2 files, and it creates a gameinfo backup first.
echo Close SFM before continuing.
echo.
powershell.exe -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\Mount_Live_TF2_Content_For_SFM.ps1" %*
if errorlevel 1 goto :fail
echo.
echo Restart SFM completely before importing the AGR.
pause
exit /b 0
:fail
echo.
echo MOUNT FAILED. The first error above is the useful one.
pause
exit /b 1

