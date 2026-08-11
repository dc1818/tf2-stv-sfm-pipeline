@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\Enable_TF_Fix_In_SFM.ps1"
pause
