@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Make a small SFM animation clip

echo WARNING: This tool only clips the old SDK bones.ndjson experiment.
echo It cannot repair T-pose/reference-pose transforms already stored in that file.
echo Use Record_Demo_With_HLAE.bat and choose the tick range during retail TF2 capture instead.
echo.

if "%~1"=="" goto :usage
set "PIPELINE_ROOT=%~dp0"
set "INPUT=%~f1"
set "BONES=%INPUT%"

if exist "%INPUT%\NUL" set "BONES=%INPUT%\bones.ndjson"
if not exist "%BONES%" goto :missing_bones

set "CONVERTER=%PIPELINE_ROOT%parser\target\release\bones_to_agr.exe"
if not exist "%CONVERTER%" goto :missing_converter

echo Choose timeline coordinate:
echo   1 = Seconds from the first captured timeline frame ^(recommended^)
echo   2 = Raw demo ticks
set /p "MODE=Choose 1 or 2 [1]: "
if not defined MODE set "MODE=1"

if "%MODE%"=="2" goto :ticks
if not "%MODE%"=="1" goto :invalid_mode

set /p "START=Clip start in relative seconds (for example 0): "
if not defined START set "START=0"
set /p "END=Clip end in relative seconds (for example 5): "
if not defined END set "END=5"
set "RANGE_ARGS=--start %START% --end %END%"
set "RANGE_NAME=%START%s_to_%END%s"
goto :fps

:ticks
echo TF2 is normally about 66.67 ticks per second, so 10 seconds is about 667 ticks.
set /p "START_TICK=Start demo tick: "
if not defined START_TICK goto :missing_tick
set /p "END_TICK=End demo tick (start + 667 is about 10 seconds): "
if not defined END_TICK goto :missing_tick
set "RANGE_ARGS=--start-demo-tick %START_TICK% --end-demo-tick %END_TICK%"
set "RANGE_NAME=tick_%START_TICK%_to_%END_TICK%"

:fps
set /p "FPS=Import frames per second (recommended 30): "
if not defined FPS set "FPS=30"

for %%I in ("%BONES%") do set "PROJECT_DIR=%%~dpI"
set "CLIP_DIR=%PROJECT_DIR%sfm_clips"
if not exist "%CLIP_DIR%" mkdir "%CLIP_DIR%"
set "OUTPUT=%CLIP_DIR%\sfm_clip_%RANGE_NAME%_%FPS%fps.agr"

echo.
echo Creating a standalone SFM clip.
echo Input:  %BONES%
echo Range:  %RANGE_NAME%
echo Output: %OUTPUT%
echo.
echo This streams only as far as the end of the requested range; it does not load the master file into memory.
"%CONVERTER%" "%BONES%" "%OUTPUT%" --fps %FPS% %RANGE_ARGS% --trusted-complete
if errorlevel 1 goto :fail

echo.
echo CLIP EXPORT PASSED
echo Import this AGR through Rig ^> advancedfx_import_gameRecord:
echo   %OUTPUT%
pause
exit /b 0

:usage
echo Drag a timestamped SFM project folder or its bones.ndjson onto this BAT.
echo It creates a short standalone AGR in that project's sfm_clips folder.
pause
exit /b 1

:missing_bones
echo ERROR: Could not find bones.ndjson at:
echo   %BONES%
goto :fail

:missing_converter
echo ERROR: bones_to_agr.exe is missing.
echo Run Build_SFM_Clip_Tool.bat once, then try again.
goto :fail

:invalid_mode
echo ERROR: Enter 1 for seconds or 2 for demo ticks.
goto :fail

:missing_tick
echo ERROR: Both a start and end demo tick are required for tick mode.
goto :fail

:fail
echo.
echo CLIP EXPORT FAILED. The first error above is the useful one.
pause
exit /b 1
