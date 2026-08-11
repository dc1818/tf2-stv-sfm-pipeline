@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Test TF2 bone export

echo NOTE: This validates the old SDK file structure only. It does not prove animation fidelity.
echo The visually tested SDK export produced T-pose/reference-pose players in SFM.
echo.

set "PIPELINE_ROOT=%~dp0"
set "TARGET=%~1"
if not defined TARGET goto :usage

powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%PIPELINE_ROOT%tools\Validate_Bones.ps1" -Path "%TARGET%" -TimeoutSeconds 2
if errorlevel 1 goto :failed

echo.
echo The SDK animation export is complete and structurally valid.
pause
exit /b 0

:usage
echo Drag either bones.ndjson or its timestamped _sfm_project folder onto this BAT.
goto :failed

:failed
echo.
echo BONE EXPORT TEST FAILED.
pause
exit /b 1
