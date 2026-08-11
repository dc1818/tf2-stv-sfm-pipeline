@echo off
setlocal EnableExtensions DisableDelayedExpansion
title TF2 STV to SFM - SDK diagnostic only

set "PIPELINE_ROOT=%~dp0"
echo WARNING: This is the old SDK ghost experiment. It produced T-pose/reference-pose players in visual testing.
echo Use Process_STV_To_SFM.bat or Record_Demo_With_HLAE.bat for SFM animation.
echo.
set "SDK_ROOT=%TF2_SDK_ROOT%"
if not defined SDK_ROOT if exist "%PIPELINE_ROOT%SDK_PATH.txt" set /p SDK_ROOT=<"%PIPELINE_ROOT%SDK_PATH.txt"
if not defined SDK_ROOT set "SDK_ROOT=%PIPELINE_ROOT%..\source-sdk-2013"
for %%I in ("%SDK_ROOT%") do set "SDK_ROOT=%%~fI"
set "DEMO=%~1"
if not defined DEMO goto :usage
for %%I in ("%DEMO%") do (
  set "DEMO=%%~fI"
  set "OUTPUT_BASE=%%~dpnI_sfm_project"
)
for /f "usebackq delims=" %%I in (`powershell -NoProfile -Command "Get-Date -Format yyyyMMdd_HHmmss"`) do set "RUN_STAMP=%%I"
set "OUTPUT=%OUTPUT_BASE%_%RUN_STAMP%"

set "EXPORTER=%PIPELINE_ROOT%parser\target\release\export_all.exe"
set "AGR_CONVERTER=%PIPELINE_ROOT%parser\target\release\bones_to_agr.exe"
set "WORKER_RUNNER=%PIPELINE_ROOT%tools\Run_Worker.ps1"
set "BONES_VALIDATOR=%PIPELINE_ROOT%tools\Validate_Bones.ps1"
set "CLIENT_DLL=%SDK_ROOT%\game\mod_tf\bin\x64\client.dll"
set "SERVER_DLL=%SDK_ROOT%\game\mod_tf\bin\x64\server.dll"
set "GAME_DIR=%SDK_ROOT%\game\mod_tf"
set "CONSOLE_LOG=%GAME_DIR%\console.log"
set "WORKER_STAMP=%SDK_ROOT%\game\mod_tf\bin\x64\tf_bone_worker_build.txt"
set "REQUIRED_WORKER_VERSION=0.1.4"

if not exist "%DEMO%" goto :missing_demo
if /i not "%~x1"==".dem" goto :usage
if not exist "%EXPORTER%" goto :missing_exporter
if not exist "%AGR_CONVERTER%" goto :missing_converter
if not exist "%WORKER_RUNNER%" goto :missing_runner
if not exist "%BONES_VALIDATOR%" goto :missing_runner
if not exist "%CLIENT_DLL%" goto :missing_client
if not exist "%SERVER_DLL%" goto :missing_server
if not exist "%WORKER_STAMP%" goto :missing_stamp
set "STAMP_FILE=%WORKER_STAMP%"
powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -Command "$actual = (Get-Content -LiteralPath $env:STAMP_FILE -Raw).Trim(); if ($actual -ne $env:REQUIRED_WORKER_VERSION) { Write-Host ('Worker stamp is ' + $actual + '; expected ' + $env:REQUIRED_WORKER_VERSION); exit 1 }"
if errorlevel 1 goto :old_worker

call :find_hl2
if not defined HL2_EXE goto :missing_base

if not exist "%OUTPUT%" mkdir "%OUTPUT%"
if errorlevel 1 goto :fail

echo.
echo [1/4] Parsing all STV data and resolving animation inputs...
"%EXPORTER%" "%DEMO%" "%OUTPUT%"
if errorlevel 1 goto :fail
if not exist "%OUTPUT%\worker_frames.tsv" goto :fail

if exist "%CONSOLE_LOG%" move /y "%CONSOLE_LOG%" "%OUTPUT%\previous_console.log" >nul

echo.
echo [2/4] Running the noninteractive TF2 client animation worker...
echo No demo playback or game menu is used. This can take several minutes.
powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%WORKER_RUNNER%" -Executable "%HL2_EXE%" -GameDirectory "%GAME_DIR%" -InputPath "%OUTPUT%\worker_frames.tsv" -OutputPath "%OUTPUT%\bones.ndjson" -ConsoleLog "%CONSOLE_LOG%"
if errorlevel 1 goto :worker_failed

powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%BONES_VALIDATOR%" -Path "%OUTPUT%\bones.ndjson" -TimeoutSeconds 60
if errorlevel 1 goto :worker_failed

echo.
echo [3/4] Converting parent-local matrices to a 30 fps SFM GameRecord...
"%AGR_CONVERTER%" "%OUTPUT%\bones.ndjson" "%OUTPUT%\sfm_import.agr" --fps 30
if errorlevel 1 goto :fail

echo.
echo [4/4] Writing the combined project manifest...
powershell -NoProfile -ExecutionPolicy Bypass -File "%PIPELINE_ROOT%tools\Finalize_Project.ps1" -ProjectDirectory "%OUTPUT%" -SourceDemo "%DEMO%"
if errorlevel 1 goto :fail

echo.
echo SUCCESS
echo Project: %OUTPUT%
echo Bones:  %OUTPUT%\bones.ndjson
echo SFM:    %OUTPUT%\sfm_import.agr
echo Read SFM_IMPORT.md for the SFM steps.
pause
exit /b 0

:find_hl2
if defined SOURCE_SDK_BASE_2013_EXE if exist "%SOURCE_SDK_BASE_2013_EXE%" (
  set "HL2_EXE=%SOURCE_SDK_BASE_2013_EXE%"
  exit /b 0
)
set "HL2_EXE=%ProgramFiles(x86)%\Steam\steamapps\common\Source SDK Base 2013 Multiplayer\hl2.exe"
if exist "%HL2_EXE%" exit /b 0
set "HL2_EXE=%ProgramFiles%\Steam\steamapps\common\Source SDK Base 2013 Multiplayer\hl2.exe"
if exist "%HL2_EXE%" exit /b 0
set "HL2_EXE="
for /f "tokens=2,*" %%A in ('reg query "HKCU\Software\Valve\Steam" /v SteamPath 2^>nul') do set "STEAM_ROOT=%%B"
if defined STEAM_ROOT if exist "%STEAM_ROOT%\steamapps\common\Source SDK Base 2013 Multiplayer\hl2.exe" set "HL2_EXE=%STEAM_ROOT%\steamapps\common\Source SDK Base 2013 Multiplayer\hl2.exe"
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
goto :build_required
:missing_converter
echo ERROR: Missing SFM converter:
echo   %AGR_CONVERTER%
goto :build_required
:missing_runner
echo ERROR: Missing worker launcher:
echo   %WORKER_RUNNER%
echo   %BONES_VALIDATOR%
echo Extract the complete pipeline ZIP over this folder.
goto :fail
:missing_client
echo ERROR: Missing client DLL:
echo   %CLIENT_DLL%
goto :build_required
:missing_server
echo ERROR: Missing server DLL:
echo   %SERVER_DLL%
goto :build_required
:missing_stamp
echo ERROR: Missing worker build stamp:
echo   %WORKER_STAMP%
goto :build_required
:old_worker
echo ERROR: The installed SDK worker does not match required version %REQUIRED_WORKER_VERSION%.
goto :build_required
:build_required
echo Run Install_Or_Update.bat, then Build_All.bat and wait for BUILD SUCCEEDED.
goto :fail
:missing_base
echo ERROR: Source SDK Base 2013 Multiplayer hl2.exe was not found.
echo Install it in Steam, or set SOURCE_SDK_BASE_2013_EXE to its full hl2.exe path.
goto :fail
:worker_failed
echo.
echo ERROR: The worker did not produce a successful bones.ndjson.
echo Console log: %CONSOLE_LOG%
if exist "%CONSOLE_LOG%" powershell -NoProfile -Command "Get-Content -LiteralPath $env:CONSOLE_LOG -Tail 120"
goto :fail
:fail
echo.
pause
exit /b 1
