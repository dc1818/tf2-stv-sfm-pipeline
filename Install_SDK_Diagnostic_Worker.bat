@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Install deprecated TF2 SDK diagnostic worker

echo WARNING: This installs the SDK ghost experiment that produced T-pose/reference-pose output.
echo Use Install_Or_Update.bat for the supported retail TF2/HLAE capture path.
echo.

set "PIPELINE_ROOT=%~dp0"
set "SDK_ROOT=%~1"
if not defined SDK_ROOT set "SDK_ROOT=%PIPELINE_ROOT%..\source-sdk-2013"
for %%I in ("%SDK_ROOT%") do set "SDK_ROOT=%%~fI"
set "BASE_PATCH=%PIPELINE_ROOT%patches\TF2_Noninteractive_Bone_Worker.patch"
set "AUTORUN_PATCH=%PIPELINE_ROOT%patches\TF2_Noninteractive_Bone_Worker_Autorun_Update.patch"
set "REQUIRED_COMMIT=22288b919617be6c8ca3cefd7cca979cbb39a88c"

if not exist "%SDK_ROOT%\.git\HEAD" goto :missing_sdk
if not exist "%BASE_PATCH%" goto :missing_patch
if not exist "%AUTORUN_PATCH%" goto :missing_patch

for /f "usebackq delims=" %%I in (`git -C "%SDK_ROOT%" rev-parse HEAD 2^>nul`) do set "SDK_COMMIT=%%I"
if not "%SDK_COMMIT%"=="%REQUIRED_COMMIT%" goto :wrong_commit

rem Check the newest patch first. Once installed it intentionally changes a
rem file created by the base patch, so the base patch is no longer exactly
rem reverse-applicable even though it is present.
git -C "%SDK_ROOT%" apply --reverse --check "%AUTORUN_PATCH%" >nul 2>&1
if not errorlevel 1 (
  echo Base animation worker and post-signon autorun update are already installed.
  goto :done
)

git -C "%SDK_ROOT%" apply --check "%AUTORUN_PATCH%" >nul 2>&1
if not errorlevel 1 goto :apply_autorun

git -C "%SDK_ROOT%" apply --check "%BASE_PATCH%" >nul 2>&1
if not errorlevel 1 (
  git -C "%SDK_ROOT%" apply "%BASE_PATCH%"
  if errorlevel 1 goto :fail
  echo Installed base animation worker.
) else (
  git -C "%SDK_ROOT%" apply --reverse --check "%BASE_PATCH%" >nul 2>&1
  if errorlevel 1 goto :patch_overlap
  echo Base animation worker is already installed.
)

:apply_autorun
git -C "%SDK_ROOT%" apply --check "%AUTORUN_PATCH%" >nul 2>&1
if errorlevel 1 goto :patch_overlap
git -C "%SDK_ROOT%" apply "%AUTORUN_PATCH%"
if errorlevel 1 goto :fail
echo Installed post-signon autorun update.

:done
set "SDK_PATH_FILE=%PIPELINE_ROOT%SDK_PATH.txt"
powershell -NoProfile -ExecutionPolicy Bypass -Command "[IO.File]::WriteAllText($env:SDK_PATH_FILE, $env:SDK_ROOT)"
if errorlevel 1 goto :fail
echo.
echo Install/update completed. Next run Build_All.bat.
exit /b 0

:patch_overlap
echo.
echo The SDK files overlap an older or different patch, so this update cannot be applied safely.
echo No additional files were changed. Use a clean checkout at the required commit if you edited the worker sources manually.
goto :fail

:missing_sdk
echo ERROR: Source SDK checkout was not found at:
echo   %SDK_ROOT%
echo Pass its path as the first argument to this BAT.
goto :fail

:missing_patch
echo ERROR: One or more worker patch files are missing from:
echo   %PIPELINE_ROOT%patches
goto :fail

:wrong_commit
echo ERROR: SDK is at commit %SDK_COMMIT%
echo Required commit: %REQUIRED_COMMIT%
echo Run this in the SDK folder, only if you do not need uncommitted work there:
echo   git checkout %REQUIRED_COMMIT%
goto :fail

:fail
echo.
pause
exit /b 1
