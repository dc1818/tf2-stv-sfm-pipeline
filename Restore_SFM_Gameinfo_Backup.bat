@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Restore SFM gameinfo backup
set "SFM_ROOT=%~1"
if not defined SFM_ROOT set "SFM_ROOT=%ProgramFiles(x86)%\Steam\steamapps\common\SourceFilmmaker"
for %%I in ("%SFM_ROOT%") do set "SFM_ROOT=%%~fI"
set "GAMEINFO=%SFM_ROOT%\game\usermod\gameinfo.txt"
if not exist "%GAMEINFO%" goto :missing
for /f "delims=" %%I in ('dir /b /a-d /o-d "%GAMEINFO%.tf2sfm-backup-*" 2^>nul') do if not defined BACKUP set "BACKUP=%SFM_ROOT%\game\usermod\%%I"
if not defined BACKUP goto :none
echo Restoring the newest backup before the TF2 mount:
echo   %BACKUP%
copy /y "%BACKUP%" "%GAMEINFO%" >nul
if errorlevel 1 goto :fail
echo Restored gameinfo.txt:
echo   %GAMEINFO%
echo SFM should now launch with its original search paths.
pause
exit /b 0
:missing
echo ERROR: SFM gameinfo.txt was not found:
echo   %GAMEINFO%
goto :fail
:none
echo No TF2SFM gameinfo backup was found beside:
echo   %GAMEINFO%
:fail
pause
exit /b 1
