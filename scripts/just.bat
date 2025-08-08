@echo off
REM Lightweight replacement for the `just` command on Windows.
REM Usage:  just <task-name>
REM Example: just release-medium

setlocal enabledelayedexpansion
if "%~1"=="" goto :help
set TASK=%~1
shift

REM Helper: ensure we are at repo root (this script resides in scripts/)
pushd %~dp0\..

REM Common env hardening
set CARGO_TERM_COLOR=always

REM Dispatch table
if /I "%TASK%"=="release"            goto :release_default
if /I "%TASK%"=="release-light"      goto :release_light
if /I "%TASK%"=="release-medium"     goto :release_medium
if /I "%TASK%"=="release-heavy"      goto :release_heavy
if /I "%TASK%"=="release-maximum"    goto :release_maximum
if /I "%TASK%"=="builder-light"      goto :builder_light
if /I "%TASK%"=="builder-medium"     goto :builder_medium
if /I "%TASK%"=="builder-maximum"    goto :builder_maximum
if /I "%TASK%"=="post-obfuscate"     goto :post_obfuscate
if /I "%TASK%"=="advanced-pe-obfuscate" goto :advanced_pe
if /I "%TASK%"=="pack-runtime"       goto :pack_runtime
if /I "%TASK%"=="obfuscate-complete" goto :obf_complete
if /I "%TASK%"=="obfuscate-ultimate" goto :obf_ultimate
if /I "%TASK%"=="obfuscation-info"   goto :obf_info
if /I "%TASK%"=="obfuscation-report" goto :obf_report
if /I "%TASK%"=="clean"              goto :clean
if /I "%TASK%"=="debug"              goto :debug
if /I "%TASK%"=="run"                goto :run
if /I "%TASK%"=="help"               goto :help

echo Unknown task: %TASK%
echo.
goto :help

:release_default
REM Mirrors original release recipe
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -C link-arg=/OPT:REF -C link-arg=/OPT:ICF -C link-arg=/INCREMENTAL:NO -C link-arg=/DEBUG:NONE -C link-arg=/RELEASE
cargo build --release
goto :eof

:release_light
set OBFUSCATION_LEVEL=light
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -C link-arg=/OPT:REF -C link-arg=/OPT:ICF -C link-arg=/INCREMENTAL:NO -C link-arg=/DEBUG:NONE -C link-arg=/RELEASE
cargo build --release --features builder_build
goto :eof

:release_medium
set OBFUSCATION_LEVEL=medium
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -C link-arg=/OPT:REF -C link-arg=/OPT:ICF -C link-arg=/GUARD:CF -C link-arg=/DYNAMICBASE -C link-arg=/INCREMENTAL:NO -C link-arg=/DEBUG:NONE -C link-arg=/RELEASE
cargo build --release --features builder_build
goto :eof

:release_heavy
set OBFUSCATION_LEVEL=heavy
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -Z randomize-layout -C debuginfo=0 -C link-arg=/OPT:REF -C link-arg=/OPT:ICF -C link-arg=/GUARD:CF -C link-arg=/DYNAMICBASE -C link-arg=/INTEGRITYCHECK -C link-arg=/NXCOMPAT -C link-arg=/INCREMENTAL:NO -C link-arg=/DEBUG:NONE -C link-arg=/RELEASE
cargo build --release --features builder_build
goto :eof

:release_maximum
set OBFUSCATION_LEVEL=maximum
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -Z randomize-layout -C debuginfo=0 -C link-arg=/OPT:REF -C link-arg=/OPT:ICF -C link-arg=/GUARD:CF -C link-arg=/DYNAMICBASE -C link-arg=/INTEGRITYCHECK -C link-arg=/NXCOMPAT -C link-arg=/MERGE:.rdata=.text -C link-arg=/DELAYLOAD:kernel32.dll -C link-arg=/DELAYLOAD:user32.dll -C link-arg=/INCREMENTAL:NO -C link-arg=/DEBUG:NONE -C link-arg=/RELEASE
cargo build --release --features builder_build
goto :eof

:builder_light
set OBFUSCATION_LEVEL=light
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -C debuginfo=0
cargo build --release --bin builder --features builder_build
goto :eof

:builder_medium
set OBFUSCATION_LEVEL=medium
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -Z randomize-layout
cargo build --release --bin builder --features builder_build
goto :eof

:builder_maximum
set OBFUSCATION_LEVEL=maximum
set RUSTFLAGS=-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -Z randomize-layout
cargo build --release --bin builder --features builder_build
goto :eof

:post_obfuscate
call :release_maximum
if exist tools\post_obfuscate.bat call tools\post_obfuscate.bat
goto :eof

:advanced_pe
call :release_maximum
if exist tools\advanced_pe_obfuscator.py python tools\advanced_pe_obfuscator.py target\release\ShadowSniff.exe --all
goto :eof

:pack_runtime
call :release_maximum
if exist tools\runtime_packer.py python tools\runtime_packer.py target\release\ShadowSniff.exe target\release\ShadowSniff_packed.exe --polymorphic --stats
goto :eof

:obf_complete
call :clean
call :release_maximum
call :post_obfuscate
goto :eof

:obf_ultimate
call :clean
call :release_maximum
call :advanced_pe
call :pack_runtime
goto :eof

:obf_info
call :obf_report
goto :eof

:obf_report
echo === ShadowSniff Obfuscation (Batch Wrapper) ===
echo Tasks:
echo   release-light       Basic obfuscation
echo   release-medium      Recommended level
echo   release-heavy       Advanced CFG + layout
echo   release-maximum     All compile-time features
echo   obfuscate-complete  Max + post-processing
echo   obfuscate-ultimate  All (PE + packer)
echo   advanced-pe-obfuscate  PE structure obfuscation
echo   pack-runtime        Runtime packer/crypter
echo   builder-light|medium|maximum  Builder variants
echo   clean, debug, run
echo.
echo Examples:
echo   scripts\just.bat release-medium
echo   scripts\just.bat obfuscate-ultimate
goto :eof

:clean
cargo clean
goto :eof

:debug
cargo build
goto :eof

:run
cargo run
goto :eof

:help
echo Usage: just ^<task^>
echo See available tasks with: just obfuscation-info
echo Or run   scripts\just.bat obfuscation-report
goto :eof

:end
popd
endlocal
