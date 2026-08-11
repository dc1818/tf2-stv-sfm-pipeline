@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Emergency restore SFM gameinfo
set "GAMEINFO=%ProgramFiles(x86)%\Steam\steamapps\common\SourceFilmmaker\game\usermod\gameinfo.txt"
if not exist "%GAMEINFO%" goto :missing
for /f "delims=" %%I in ('dir /b /a-d /o-d "%GAMEINFO%.tf2sfm-backup-*" 2^>nul') do if not defined BACKUP_NAME set "BACKUP_NAME=%%I"
if not defined BACKUP_NAME goto :none
for %%I in ("%GAMEINFO%") do set "GAMEINFO_DIR=%%~dpI"
set "BACKUP=%GAMEINFO_DIR%%BACKUP_NAME%"
echo Restoring:
echo   %BACKUP%
copy /y "%BACKUP%" "%GAMEINFO%" >nul
if errorlevel 1 goto :fail
echo Restored. Try launching SFM now.
pause
exit /b 0
:missing
echo SFM gameinfo.txt was not found at:
echo   %GAMEINFO%
goto :fail
:none
echo No backup matching this pattern was found:
echo   %GAMEINFO%.tf2sfm-backup-*
:fail
pause
exit /b 1

