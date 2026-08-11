@echo off
setlocal EnableExtensions
title Build TF2 STV to SFM retail HLAE pipeline
echo.
echo The primary pipeline now uses retail TF2 plus HLAE for animation fidelity.
echo Only the Rust parser needs to be built; the SDK ghost worker is diagnostic-only.
echo.
call "%~dp0Build_GUI_And_Parser.bat"
exit /b %errorlevel%
