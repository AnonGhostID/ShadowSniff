# ShadowSniff Obfuscation Guide

## Overview

This guide covers the multi-level obfuscation system implemented for ShadowSniff, providing protection against reverse engineering and analysis.

## Obfuscation Levels

### 1. Light Obfuscation (`just release-light`)
**Basic protection with minimal performance impact**

- ✅ Symbol stripping
- ✅ Debug info removal  
- ✅ Basic linker optimizations
- ✅ Size optimization

**Use case:** General distribution, minimal AV detection risk

### 2. Medium Obfuscation (`just release-medium`) ⭐ **Recommended**
**Balanced protection and performance**

- ✅ Everything in Light
- ✅ String obfuscation via `obfstr` 
- ✅ Control Flow Guard (CFG)
- ✅ Address Space Layout Randomization (ASLR)
- ✅ Dynamic base addressing

**Use case:** Production deployment with good protection

### 3. Heavy Obfuscation (`just release-heavy`)
**Advanced protection with some performance cost**

- ✅ Everything in Medium
- ✅ Control flow obfuscation 
- ✅ Layout randomization (`-Z randomize-layout`)
- ✅ Integrity checking
- ✅ NX compatibility
- ✅ Enhanced linker security

**Use case:** High-security environments, advanced threat protection

### 4. Maximum Obfuscation (`just release-maximum`)
**Extreme protection with significant overhead**

- ✅ Everything in Heavy
- ✅ Anti-debugging techniques
- ✅ Section merging (.rdata → .text)
- ✅ Import hiding via delay loading
- ✅ Advanced binary protection
- ✅ Post-build processing

**Use case:** Maximum stealth, research purposes

## Technical Implementation

### Compile-Time Obfuscation

1. **String Obfuscation**
   - XOR encryption with build-time keys
   - Runtime decryption
   - Unicode character substitution

2. **Control Flow Protection**
   - Opaque predicates
   - Fake conditional branches
   - State machine flattening

3. **Layout Randomization**
   - Struct/enum layout randomization
   - Function ordering randomization
   - Register allocation randomization

### Link-Time Obfuscation

1. **Section Management**
   - Section merging to reduce analysis surface
   - Fake section injection
   - Resource embedding

2. **Import Obfuscation**
   - Delay-loaded DLLs
   - Import address table hiding
   - Dynamic resolution

### Post-Build Obfuscation

1. **Binary Modification** 
   - Junk code injection
   - String scrambling
   - Entropy increase

2. **Packing (Optional)**
   - UPX compression
   - Custom packers
   - Runtime decompression

## Usage Examples

### Basic Usage
```powershell
# Medium obfuscation (recommended)
just release-medium

# Maximum obfuscation with post-processing  
just obfuscate-complete
```

### Custom Environment Variables
```powershell
# Set obfuscation level
$env:OBFUSCATION_LEVEL="heavy"
just release

# Enable specific features
cargo build --release --features "builder_build"
```

### Builder-Specific Builds
```powershell
# Obfuscated builder executable
just builder-maximum
```

## File Structure

```
ShadowSniff/
├── obfuscation/              # Obfuscation utilities crate
│   ├── src/
│   │   ├── string_obfuscation.rs
│   │   ├── control_flow.rs
│   │   ├── anti_debug.rs
│   │   └── binary_protection.rs
│   └── Cargo.toml
├── tools/                    # Post-build tools
│   ├── post_obfuscate.py     # Python obfuscation script
│   └── post_obfuscate.bat    # Windows batch wrapper
├── obfuscation.toml          # Configuration file
├── build.rs                  # Enhanced build script
└── justfile                  # Build recipes
```

## Security Considerations

### Anti-Analysis Techniques

1. **Anti-Debugging**
   - Hardware breakpoint detection
   - Timing-based detection
   - PEB manipulation detection

2. **Anti-Disassembly**
   - Opaque predicates
   - Fake function calls
   - Control flow confusion

3. **String Protection**
   - Runtime decryption
   - Key derivation from build entropy
   - Unicode obfuscation

### Evasion Techniques

1. **Static Analysis Evasion**
   - High entropy content
   - Fake API references
   - Misleading metadata

2. **Dynamic Analysis Evasion**
   - Environment detection
   - Sandbox detection
   - VM detection

## Performance Impact

| Level | Binary Size | Runtime Overhead | Protection Level |
|-------|-------------|------------------|------------------|
| Light | +5-10% | <1% | Basic |
| Medium | +15-25% | 2-5% | Good |
| Heavy | +30-50% | 5-15% | High |
| Maximum | +50-100% | 15-30% | Extreme |

## Troubleshooting

### Build Issues

1. **RUSTFLAGS not recognized**
   ```powershell
   # Use PowerShell syntax
   $env:RUSTFLAGS="flags here"
   ```

2. **Linker errors on Windows**
   ```powershell
   # Ensure MSVC toolchain is installed
   rustup default stable-x86_64-pc-windows-msvc
   ```

3. **Post-processing fails**
   ```powershell
   # Install required tools
   pip install pefile  # For Python script
   # Download UPX from https://upx.github.io/
   ```

### Runtime Issues

1. **Antivirus detection**
   - Use lighter obfuscation levels
   - Disable UPX packing
   - Add exclusions for build directory

2. **Performance issues**
   - Profile with lighter obfuscation first
   - Disable anti-debugging for testing
   - Use medium level for production

## Advanced Configuration

### Custom Build Scripts

Create custom build configurations in `build.rs`:

```rust
// Custom obfuscation level
if env::var("CUSTOM_OBFUSCATION").is_ok() {
    println!("cargo:rustc-cfg=feature=\"custom_protection\"");
}
```

### External Tools Integration

Integrate with professional obfuscation tools:

```powershell
# Example with commercial obfuscator
just release-heavy
.\external-obfuscator.exe target\release\ShadowSniff.exe
```

## Best Practices

1. **Development Workflow**
   - Use `debug` builds during development
   - Test with `medium` obfuscation first
   - Use `maximum` only for final releases

2. **Security Workflow**
   - Always backup original binaries
   - Test obfuscated binaries thoroughly
   - Monitor for false positive AV detection

3. **Distribution**
   - Use different obfuscation levels per target
   - Consider split distribution (staged loading)
   - Implement integrity checking

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review build logs for specific errors  
3. Test with lower obfuscation levels first
4. Verify all required tools are installed

---

**⚠️ Disclaimer:** These obfuscation techniques are for legitimate security research and software protection. Always comply with applicable laws and regulations.
