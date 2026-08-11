@echo off
setlocal EnableExtensions
title Build TF2 STV parser
set "PARSER_ROOT=%~dp0parser"
if not exist "%PARSER_ROOT%\Cargo.toml" goto :missing
if not exist "%PARSER_ROOT%\src\bin\export_all.rs" goto :parser_source_missing
where cargo >nul 2>&1
if errorlevel 1 goto :rust
pushd "%PARSER_ROOT%"
cargo build --release --locked --bin export_all
if errorlevel 1 (popd & goto :fail)
popd
echo.
echo PARSER BUILD SUCCEEDED.
echo You can now drag a demo onto Process_STV_To_SFM.bat.
pause
exit /b 0
:missing
echo ERROR: Bundled parser source is missing.
goto :fail
:parser_source_missing
echo ERROR: The bundled parser is incomplete: src\bin\export_all.rs is missing.
echo Extract a complete TF2 STV to SFM bundle into a new folder, then run this again.
goto :fail
:rust
echo ERROR: cargo was not found. Install Rust from https://rustup.rs/ and reopen this terminal.
:fail
echo.
echo BUILD FAILED. The first error above is the useful one.
pause
exit /b 1
