@echo off
setlocal
title Install AdvancedFX SFM game-record rig
set "SFM_ROOT=%~1"
if not defined SFM_ROOT set "SFM_ROOT=%ProgramFiles(x86)%\Steam\steamapps\common\SourceFilmmaker"
powershell.exe -NoLogo -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\Install_AdvancedFX_SFM_Rig.ps1" -SfmRoot "%SFM_ROOT%" %2 %3
set "CODE=%ERRORLEVEL%"
echo.
if not "%CODE%"=="0" echo AdvancedFX rig installation failed.
pause
exit /b %CODE%
