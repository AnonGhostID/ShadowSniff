/*
 * This file is part of ShadowSniff (https://github.com/sqlerrorthing/ShadowSniff)
 *
 * MIT License
 *
 * Copyright (c) 2025 sqlerrorthing
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
use std::env;
use std::process::Command;
use std::path::Path;

fn main() {
    let before = env::var("CARGO_FEATURE_MESSAGE_BOX_BEFORE_EXECUTION").is_ok();
    let after = env::var("CARGO_FEATURE_MESSAGE_BOX_AFTER_EXECUTION").is_ok();

    if before && after {
        panic!("Only one of `message_box_before_execution` or `message_box_after_execution` can be enabled at a time.");
    }
    
    // Re-run if build script changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=obfuscation.toml");
    
    // Get obfuscation level from environment
    let obfuscation_level = env::var("OBFUSCATION_LEVEL")
        .unwrap_or_else(|_| "medium".to_string());
    
    println!("cargo:rustc-cfg=obfuscation_level=\"{}\"", obfuscation_level);
    
    // Apply different RUSTFLAGS based on obfuscation level
    match obfuscation_level.as_str() {
        "light" => apply_light_obfuscation(),
        "medium" => apply_medium_obfuscation(),
        "heavy" => apply_heavy_obfuscation(),
        "maximum" => apply_maximum_obfuscation(),
        _ => apply_medium_obfuscation(),
    }
    
    // Platform-specific optimizations
    if cfg!(target_os = "windows") {
        apply_windows_obfuscation();
    }
    
    // Generate build-time entropy for dynamic obfuscation keys
    let build_entropy = generate_build_entropy();
    println!("cargo:rustc-env=BUILD_ENTROPY={}", build_entropy);
    
    // Generate obfuscated code if in advanced mode
    if obfuscation_level == "maximum" {
        generate_advanced_obfuscation(&build_entropy);
    }
    
    // Add dependencies for obfuscation crate
    println!("cargo:rustc-link-lib=user32");
    println!("cargo:rustc-link-lib=kernel32");
    println!("cargo:rustc-link-lib=ntdll");
}

fn apply_light_obfuscation() {
    println!("cargo:rustc-link-arg=/OPT:REF");
    println!("cargo:rustc-link-arg=/OPT:ICF");
    println!("cargo:rustc-cfg=obfuscation_light");
}

fn apply_medium_obfuscation() {
    apply_light_obfuscation();
    
    // Control flow guard
    println!("cargo:rustc-link-arg=/GUARD:CF");
    
    // Randomize base address
    println!("cargo:rustc-link-arg=/DYNAMICBASE");
    
    // Additional optimization flags
    println!("cargo:rustc-cfg=feature=\"string_obfuscation\"");
    println!("cargo:rustc-cfg=obfuscation_medium");
}

fn apply_heavy_obfuscation() {
    apply_medium_obfuscation();
    
    // Enable control flow obfuscation
    println!("cargo:rustc-cfg=feature=\"control_flow_obfuscation\"");
    println!("cargo:rustc-cfg=obfuscation_heavy");
    
    // Force function inlining randomization
    println!("cargo:rustc-env=RUSTFLAGS_EXTRA=-Z randomize-layout");
    
    // Additional linker hardening
    println!("cargo:rustc-link-arg=/INTEGRITYCHECK");
    println!("cargo:rustc-link-arg=/NXCOMPAT");
    
    // Advanced code generation
    println!("cargo:rustc-cfg=advanced_obfuscation");
}

fn apply_maximum_obfuscation() {
    apply_heavy_obfuscation();
    
    // Enable all obfuscation features
    println!("cargo:rustc-cfg=feature=\"anti_debugging\"");
    println!("cargo:rustc-cfg=feature=\"binary_protection\"");
    println!("cargo:rustc-cfg=feature=\"advanced_anti_analysis\"");
    println!("cargo:rustc-cfg=obfuscation_maximum");
    
    // Maximum optimization and stripping
    println!("cargo:rustc-link-arg=/MERGE:.rdata=.text");
    println!("cargo:rustc-link-arg=/MERGE:.pdata=.text");
    
    // Advanced linker options
    println!("cargo:rustc-link-arg=/EMITPOGOPHASEINFO");
    println!("cargo:rustc-link-arg=/POGOPHASEINFO:WRITE");
}

fn apply_windows_obfuscation() {
    // Windows-specific obfuscation
    println!("cargo:rustc-link-arg=/SUBSYSTEM:WINDOWS");
    
    // Hide imports with delay loading
    println!("cargo:rustc-link-arg=/DELAYLOAD:kernel32.dll");
    println!("cargo:rustc-link-arg=/DELAYLOAD:user32.dll");
    println!("cargo:rustc-link-arg=/DELAYLOAD:advapi32.dll");
    // Required helper library for delay load thunks
    println!("cargo:rustc-link-lib=delayimp");
    
    // Additional security features
    println!("cargo:rustc-link-arg=/CETCOMPAT");
    println!("cargo:rustc-link-arg=/DEPENDENTLOADFLAG:0x800");
}

fn generate_build_entropy() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);
    env::var("CARGO_PKG_VERSION").unwrap_or_default().hash(&mut hasher);
    env::var("CARGO_PKG_NAME").unwrap_or_default().hash(&mut hasher);
    
    format!("{:x}", hasher.finish())
}

fn generate_advanced_obfuscation(entropy: &str) {
    println!("cargo:warning=Generating advanced obfuscation code...");
    
    // Generate obfuscated code using Python script if available
    let output_dir = env::var("OUT_DIR").unwrap();
    let obf_file = Path::new(&output_dir).join("generated_obfuscation.rs");
    
    let python_script = "tools/rust_obfuscation_generator.py";
    
    if Path::new(python_script).exists() {
        let result = Command::new("python")
            .args(&[
                python_script,
                "--output", obf_file.to_str().unwrap(),
                "--seed", &entropy.chars().take(8).collect::<String>()
            ])
            .output();
            
        match result {
            Ok(output) => {
                if output.status.success() {
                    println!("cargo:warning=Advanced obfuscation code generated successfully");
                    println!("cargo:rustc-cfg=generated_obfuscation");
                } else {
                    println!("cargo:warning=Failed to generate obfuscation code: {}",
                        String::from_utf8_lossy(&output.stderr));
                }
            }
            Err(e) => {
                println!("cargo:warning=Python obfuscation generator not available: {}", e);
                generate_fallback_obfuscation(&obf_file, entropy);
            }
        }
    } else {
        println!("cargo:warning=Python script not found, using fallback obfuscation");
        generate_fallback_obfuscation(&obf_file, entropy);
    }
}

fn generate_fallback_obfuscation(output_file: &Path, entropy: &str) {
    use std::fs::File;
    use std::io::Write;
    
    let obf_code = format!(r#"
//! Fallback obfuscation code generated at build time
//! Entropy: {entropy}

#![allow(dead_code)]

// Build-time constants
pub const BUILD_ENTROPY: &str = "{entropy}";
pub const OBFUSCATION_SEED: u64 = 0x{entropy_hex};

// Simple obfuscation helpers
#[inline(never)]
pub fn runtime_check() -> bool {{
    let mut checksum = 0u64;
    for byte in BUILD_ENTROPY.bytes() {{
        checksum = checksum.wrapping_mul(31).wrapping_add(byte as u64);
    }}
    checksum % 17 != 0  // Simple check
}}

#[inline(never)]
pub fn confuse_analysis() {{
    let mut dummy = OBFUSCATION_SEED;
    for i in 0..100 {{
        dummy = dummy.wrapping_mul(i + 1);
        dummy ^= 0xDEADBEEF;
        if dummy == 0 {{ break; }}
    }}
    
    // Use dummy to prevent optimization
    std::hint::black_box(dummy);
}}
"#, 
        entropy = entropy,
        entropy_hex = &entropy[..std::cmp::min(16, entropy.len())]
    );
    
    if let Ok(mut file) = File::create(output_file) {
        let _ = file.write_all(obf_code.as_bytes());
        println!("cargo:rustc-cfg=fallback_obfuscation");
    }
}
