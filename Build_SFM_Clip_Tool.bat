@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Build SFM clip tool

set "PIPELINE_ROOT=%~dp0"
set "PARSER_ROOT=%PIPELINE_ROOT%parser"
if not exist "%PARSER_ROOT%\Cargo.toml" goto :missing_parser
where cargo >nul 2>&1
if errorlevel 1 goto :missing_rust

echo Building only the SFM AGR clip converter. No SDK rebuild is performed.
pushd "%PARSER_ROOT%"
cargo build --release --locked --bin bones_to_agr
if errorlevel 1 (popd & goto :fail)
popd
echo.
echo CLIP TOOL BUILD SUCCEEDED.
echo You can now drag a project folder onto Make_SFM_Test_Clip.bat.
pause
exit /b 0

:missing_parser
echo ERROR: Bundled parser source is missing.
goto :fail

:missing_rust
echo ERROR: Rust/Cargo was not found. Install it from https://rustup.rs/ then reopen this terminal.
goto :fail

:fail
echo.
echo CLIP TOOL BUILD FAILED. The first error above is the useful one.
pause
exit /b 1
