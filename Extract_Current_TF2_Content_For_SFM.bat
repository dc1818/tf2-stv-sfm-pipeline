@echo off
setlocal
powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0tools\Extract_Current_TF2_Content_For_SFM.ps1" %*
if errorlevel 1 pause
