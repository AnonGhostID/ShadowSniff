@echo off
REM Advanced Obfuscation Setup Script for ShadowSniff
REM Installs required dependencies and tools for Phase 2 obfuscation

echo ========================================
echo    ShadowSniff Advanced Obfuscation
echo         Setup Script v2.0
echo ========================================
echo.

echo [1/5] Checking Python installation...
python --version >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo ERROR: Python not found. Please install Python 3.7+ from python.org
    echo Required for advanced obfuscation tools
    pause
    exit /b 1
)
echo ✓ Python detected

echo.
echo [2/5] Installing Python dependencies...
echo Installing pycryptodome for encryption...
pip install pycryptodome >nul 2>&1
if %ERRORLEVEL% neq 0 (
    echo WARNING: Could not install pycryptodome. Using fallback encryption.
) else (
    echo ✓ pycryptodome installed
)

echo Installing additional dependencies...
pip install hashlib pathlib >nul 2>&1
echo ✓ Basic dependencies ready

echo.
echo [3/5] Checking optional tools...

REM Check for UPX packer
echo Checking for UPX packer...
upx --version >nul 2>&1
if %ERRORLEVEL% == 0 (
    echo ✓ UPX packer detected
    echo   - Runtime compression available
) else (
    echo ⚠ UPX packer not found
    echo   - Download from: https://upx.github.io/
    echo   - Optional: Provides executable compression
)

REM Check for NASM assembler
echo Checking for NASM assembler...
nasm -v >nul 2>&1
if %ERRORLEVEL% == 0 (
    echo ✓ NASM assembler detected
    echo   - Advanced shellcode generation available
) else (
    echo ⚠ NASM assembler not found
    echo   - Download from: https://www.nasm.us/
    echo   - Optional: For custom assembly code generation
)

echo.
echo [4/5] Testing obfuscation tools...

REM Test Python obfuscation generator
if exist "tools\rust_obfuscation_generator.py" (
    echo Testing Rust obfuscation generator...
    python tools\rust_obfuscation_generator.py --output temp_test.rs --seed 12345 >nul 2>&1
    if exist "temp_test.rs" (
        echo ✓ Rust obfuscation generator working
        del temp_test.rs >nul 2>&1
    ) else (
        echo ⚠ Rust obfuscation generator test failed
    )
) else (
    echo ⚠ Rust obfuscation generator not found
)

REM Test PE obfuscator
if exist "tools\advanced_pe_obfuscator.py" (
    echo ✓ Advanced PE obfuscator available
) else (
    echo ⚠ Advanced PE obfuscator not found
)

REM Test runtime packer
if exist "tools\runtime_packer.py" (
    echo ✓ Runtime packer available
) else (
    echo ⚠ Runtime packer not found
)

echo.
echo [5/5] Configuration validation...

REM Check Rust toolchain
echo Checking Rust toolchain...
rustc --version >nul 2>&1
if %ERRORLEVEL__ neq 0 (
    echo ERROR: Rust compiler not found
    echo Please install Rust from rustup.rs
    pause
    exit /b 1
)
echo ✓ Rust compiler detected

REM Check for nightly toolchain (needed for advanced features)
rustup show | findstr "nightly" >nul 2>&1
if %ERRORLEVEL__ == 0 (
    echo ✓ Nightly toolchain available
    echo   - Advanced RUSTFLAGS supported
) else (
    echo ⚠ Nightly toolchain not detected
    echo   - Install with: rustup install nightly
    echo   - Required for maximum obfuscation level
)

REM Check workspace structure
if exist "obfuscation\Cargo.toml" (
    echo ✓ Obfuscation crate found
) else (
    echo ERROR: Obfuscation crate missing
    echo Please ensure all files are properly installed
    pause
    exit /b 1
)

echo.
echo ========================================
echo         Setup Complete!
echo ========================================
echo.
echo Available obfuscation commands:
echo   just release-light      # Basic obfuscation
echo   just release-medium     # Recommended level
echo   just release-heavy      # Advanced obfuscation  
echo   just release-maximum    # Maximum protection
echo   just obfuscate-ultimate # All techniques
echo.
echo Advanced commands:
echo   just advanced-pe-obfuscate  # PE-level obfuscation
echo   just pack-runtime           # Encrypt + compress
echo   just obfuscation-report     # View all options
echo.
echo For help: just obfuscation-info
echo.

REM Create quick test
echo Testing basic build...
just debug >nul 2>&1
if %ERRORLEVEL__ == 0 (
    echo ✓ Basic build test passed
    echo.
    echo Ready for advanced obfuscation!
) else (
    echo ⚠ Basic build test failed
    echo Check your Rust installation and try again
)

echo.
echo Press any key to exit...
pause >nul
