@echo off
setlocal EnableExtensions DisableDelayedExpansion
title Build TF2 STV to SFM SDK diagnostic worker

echo WARNING: This builds the deprecated SDK ghost experiment, not the retail HLAE capture path.
echo.

set "PIPELINE_ROOT=%~dp0"
set "SDK_ROOT=%~1"
if not defined SDK_ROOT if exist "%PIPELINE_ROOT%SDK_PATH.txt" set /p SDK_ROOT=<"%PIPELINE_ROOT%SDK_PATH.txt"
if not defined SDK_ROOT set "SDK_ROOT=%PIPELINE_ROOT%..\source-sdk-2013"
for %%I in ("%SDK_ROOT%") do set "SDK_ROOT=%%~fI"
set "PARSER_ROOT=%PIPELINE_ROOT%parser"
set "VSWHERE=%ProgramFiles(x86)%\Microsoft Visual Studio\Installer\vswhere.exe"
set "WORKER_BUILD_VERSION=0.1.4"

if not exist "%SDK_ROOT%\src\devtools\bin\vpc.exe" goto :missing_sdk
findstr /c:"CTFBoneWorkerAutoRunner" "%SDK_ROOT%\src\game\client\tf\tf_bone_worker.cpp" >nul 2>&1
if errorlevel 1 goto :missing_worker_update
if not exist "%PARSER_ROOT%\Cargo.toml" goto :missing_parser
where cargo >nul 2>&1
if errorlevel 1 goto :missing_rust
if not exist "%VSWHERE%" goto :missing_vs

for /f "usebackq delims=" %%I in (`"%VSWHERE%" -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath`) do set "VSROOT=%%I"
if not defined VSROOT goto :missing_cpp
call "%VSROOT%\Common7\Tools\VsDevCmd.bat" -arch=x64 -host_arch=x64
if errorlevel 1 goto :fail

if not exist "%SDK_ROOT%\src\thirdparty\protobuf-2.6.1\bin\win64\2015\staticcrt\release\protoc.exe" goto :missing_protoc
if not exist "%PIPELINE_ROOT%tools\TextToArray.ps1" goto :missing_text_converter

echo.
echo [1/3] Building the STV parser and AGR converter...
pushd "%PARSER_ROOT%"
cargo build --release --locked --bin export_all --bin bones_to_agr
if errorlevel 1 (popd & goto :fail)
popd

echo.
echo [2/3] Generating the TF client/server solution...
pushd "%SDK_ROOT%\src"
devtools\bin\vpc.exe /tf /win64 /define:SOURCESDK +gamedlls +mathlib +tier1 +matsys_controls +vgui_controls /mksln tf_bone_worker.sln
if errorlevel 1 (popd & goto :fail)

findstr /c:"Release|win64" tf_bone_worker.sln >nul
if errorlevel 1 (popd & goto :missing_win64_configuration)

rem These files are generated build outputs. A failed command with shell
rem redirection can leave empty, newer files that MSBuild mistakes as valid.
del /q "game\server\spawn_helper_nut.h" >nul 2>&1
del /q "game\server\vscript_server_nut.h" >nul 2>&1

powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%PIPELINE_ROOT%tools\TextToArray.ps1" -InputPath "%SDK_ROOT%\src\game\server\spawn_helper.nut" -ObjectName "g_Script_spawn_helper" -OutputPath "%SDK_ROOT%\src\game\server\spawn_helper_nut.h"
if errorlevel 1 (popd & goto :script_conversion_failed)
powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File "%PIPELINE_ROOT%tools\TextToArray.ps1" -InputPath "%SDK_ROOT%\src\game\server\vscript_server.nut" -ObjectName "g_Script_vscript_server" -OutputPath "%SDK_ROOT%\src\game\server\vscript_server_nut.h"
if errorlevel 1 (popd & goto :script_conversion_failed)

echo.
echo [3/3] Building required SDK libraries, client.dll, and server.dll...
msbuild tf_bone_worker.sln /m /p:Configuration=Release /p:Platform=win64
if errorlevel 1 (popd & goto :fail)
popd

if not exist "%SDK_ROOT%\game\mod_tf\bin\x64\client.dll" goto :missing_output
if not exist "%SDK_ROOT%\game\mod_tf\bin\x64\server.dll" goto :missing_output
set "WORKER_STAMP=%SDK_ROOT%\game\mod_tf\bin\x64\tf_bone_worker_build.txt"
powershell.exe -NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -Command "[IO.File]::WriteAllText($env:WORKER_STAMP, $env:WORKER_BUILD_VERSION + [Environment]::NewLine)"
if errorlevel 1 goto :missing_output
if not exist "%SDK_ROOT%\game\mod_tf\maps" mkdir "%SDK_ROOT%\game\mod_tf\maps"
copy /y "%SDK_ROOT%\game\mod_hl2mp\maps\dm_lockdown.bsp" "%SDK_ROOT%\game\mod_tf\maps\tf_bone_worker.bsp" >nul
if errorlevel 1 goto :missing_output

echo.
echo BUILD SUCCEEDED.
echo You can now drag a .dem onto Process_STV_To_SFM.bat.
exit /b 0

:missing_sdk
echo ERROR: Source SDK was not found or is incomplete at:
echo   %SDK_ROOT%
goto :fail
:missing_worker_update
echo ERROR: The SDK does not contain the post-signon worker update.
echo Run Install_Or_Update.bat from bundle 0.1.4, then run Build_All.bat again.
goto :fail
:missing_parser
echo ERROR: Bundled parser source is missing at:
echo   %PARSER_ROOT%
goto :fail
:missing_rust
echo ERROR: cargo was not found. Install Rust from https://rustup.rs/ and reopen this terminal.
goto :fail
:missing_vs
echo ERROR: Visual Studio Installer or vswhere.exe was not found.
goto :fail
:missing_cpp
echo ERROR: Visual Studio's Desktop development with C++ workload is not installed.
goto :fail
:missing_protoc
echo ERROR: Valve's bundled Protocol Buffer compiler is missing:
echo   %SDK_ROOT%\src\thirdparty\protobuf-2.6.1\bin\win64\2015\staticcrt\release\protoc.exe
echo The SDK checkout is incomplete. Restore that tracked file or use a clean clone of the required commit.
goto :fail
:missing_text_converter
echo ERROR: The bundled Source text converter is missing from the pipeline tools folder.
echo Extract the complete pipeline ZIP again.
goto :fail
:script_conversion_failed
echo ERROR: Could not generate Source's embedded VScript headers.
echo The PowerShell error above identifies the failing input or output path.
goto :fail
:missing_win64_configuration
echo ERROR: VPC did not generate the expected Release^|win64 solution configuration.
echo Delete tf_bone_worker.sln from the SDK src folder and run this build again.
goto :fail
:missing_output
echo ERROR: MSBuild returned success but the x64 TF game DLLs were not published to game\mod_tf\bin\x64.
goto :fail
:fail
echo.
echo BUILD FAILED. The first error above is the useful one.
pause
exit /b 1
