@echo off
REM Post-build obfuscation script for Windows

echo Starting post-build obfuscation...

set TARGET_DIR=target\release
set MAIN_EXE=%TARGET_DIR%\ShadowSniff.exe
set BUILDER_EXE=%TARGET_DIR%\builder.exe

REM Check if Python is available for advanced obfuscation
python --version >nul 2>&1
if %ERRORLEVEL% == 0 (
    echo Python detected - applying advanced obfuscation
    
    if exist "%MAIN_EXE%" (
        echo Obfuscating main executable...
        python tools\post_obfuscate.py "%MAIN_EXE%"
    )
    
    if exist "%BUILDER_EXE%" (
        echo Obfuscating builder executable...
        python tools\post_obfuscate.py "%BUILDER_EXE%"
    )
) else (
    echo Python not found - skipping advanced obfuscation
)

REM Check if UPX is available for packing
upx --version >nul 2>&1
if %ERRORLEVEL% == 0 (
    echo UPX detected - packing executables
    
    if exist "%MAIN_EXE%.original" (
        echo Packing main executable...
        upx --best --lzma "%MAIN_EXE%"
    )
    
    if exist "%BUILDER_EXE%.original" (
        echo Packing builder executable...
        upx --best --lzma "%BUILDER_EXE%"
    )
) else (
    echo UPX not found - skipping executable packing
    echo Download from: https://upx.github.io/
)

REM Generate checksums for verification
echo Generating checksums...
if exist "%MAIN_EXE%" (
    certutil -hashfile "%MAIN_EXE%" SHA256 > "%MAIN_EXE%.sha256"
)
if exist "%BUILDER_EXE%" (
    certutil -hashfile "%BUILDER_EXE%" SHA256 > "%BUILDER_EXE%.sha256"
)

echo Post-build obfuscation complete!
echo.
echo Files processed:
if exist "%MAIN_EXE%" echo   - %MAIN_EXE%
if exist "%BUILDER_EXE%" echo   - %BUILDER_EXE%
echo.
echo Backup files (if created):
if exist "%MAIN_EXE%.original" echo   - %MAIN_EXE%.original
if exist "%BUILDER_EXE%.original" echo   - %BUILDER_EXE%.original

pause
