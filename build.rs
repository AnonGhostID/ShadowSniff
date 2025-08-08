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
    
    // Apply obfuscation techniques during build
    apply_build_obfuscation();
}

fn apply_build_obfuscation() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SHADOWSNIFF_OBFUSCATION_LEVEL");
    
    let obfuscation_level = env::var("SHADOWSNIFF_OBFUSCATION_LEVEL")
        .unwrap_or_else(|_| "high".to_string());
    
    match obfuscation_level.as_str() {
        "maximum" => apply_maximum_obfuscation(),
        "high" => apply_high_obfuscation(), 
        "medium" => apply_medium_obfuscation(),
        "low" => apply_low_obfuscation(),
        _ => apply_high_obfuscation(), // Default
    }
    
    // Set additional LLVM flags for obfuscation
    set_llvm_obfuscation_flags();
}

fn apply_maximum_obfuscation() {
    println!("cargo:rustc-env=SHADOWSNIFF_OBF_LEVEL=maximum");
    
    // Maximum obfuscation settings
    set_rustc_flags(&[
        "-C", "llvm-args=-obfuscate-literals",
        "-C", "llvm-args=-enable-bcf", // Bogus Control Flow
        "-C", "llvm-args=-enable-split", // Basic Block Splitting
        "-C", "llvm-args=-enable-sub", // Instruction Substitution
        "-C", "llvm-args=-enable-fla", // Control Flow Flattening
        "-C", "llvm-args=-mllvm -fla-prob=0.8", // High flattening probability
        "-C", "llvm-args=-mllvm -bcf-prob=0.8", // High bogus control flow
        "-C", "llvm-args=-mllvm -sub-prob=0.8", // High substitution probability
    ]);
}

fn apply_high_obfuscation() {
    println!("cargo:rustc-env=SHADOWSNIFF_OBF_LEVEL=high");
    
    set_rustc_flags(&[
        "-C", "llvm-args=-obfuscate-literals",
        "-C", "llvm-args=-enable-bcf",
        "-C", "llvm-args=-enable-split",
        "-C", "llvm-args=-mllvm -fla-prob=0.6",
        "-C", "llvm-args=-mllvm -bcf-prob=0.6",
    ]);
}

fn apply_medium_obfuscation() {
    println!("cargo:rustc-env=SHADOWSNIFF_OBF_LEVEL=medium");
    
    set_rustc_flags(&[
        "-C", "llvm-args=-obfuscate-literals",
        "-C", "llvm-args=-enable-bcf",
        "-C", "llvm-args=-mllvm -bcf-prob=0.4",
    ]);
}

fn apply_low_obfuscation() {
    println!("cargo:rustc-env=SHADOWSNIFF_OBF_LEVEL=low");
    
    set_rustc_flags(&[
        "-C", "llvm-args=-obfuscate-literals",
    ]);
}

fn set_llvm_obfuscation_flags() {
    // Additional LLVM passes for obfuscation
    let llvm_flags = [
        // Control flow obfuscation
        "-mllvm", "-enable-cff", // Control Flow Flattening
        "-mllvm", "-enable-bcf", // Bogus Control Flow
        "-mllvm", "-enable-split", // Basic Block Splitting
        
        // Instruction obfuscation
        "-mllvm", "-enable-substitution", // Instruction Substitution
        "-mllvm", "-enable-constant-obf", // Constant Obfuscation
        
        // Anti-analysis
        "-mllvm", "-enable-funcwra", // Function Wrapper
        "-mllvm", "-enable-strcry", // String Encryption
        
        // Additional passes
        "-mllvm", "-seed=", // Random seed (would be generated)
    ];
    
    for flag in llvm_flags.chunks(2) {
        if flag.len() == 2 {
            println!("cargo:rustc-link-arg-bin=ShadowSniff={}", flag[0]);
            if !flag[1].is_empty() {
                println!("cargo:rustc-link-arg-bin=ShadowSniff={}", flag[1]);
            }
        }
    }
}

fn set_rustc_flags(flags: &[&str]) {
    let rustflags = env::var("RUSTFLAGS").unwrap_or_default();
    let mut new_flags = vec![rustflags];
    
    for flag in flags {
        new_flags.push(flag.to_string());
    }
    
    println!("cargo:rustc-env=RUSTFLAGS={}", new_flags.join(" "));
}

// Helper function to check if obfuscation tools are available
fn check_obfuscation_tools() -> bool {
    // Check for LLVM obfuscation pass availability
    if let Ok(output) = Command::new("llvm-config")
        .arg("--version")
        .output()
    {
        if output.status.success() {
            println!("cargo:warning=LLVM obfuscation tools detected");
            return true;
        }
    }
    
    println!("cargo:warning=LLVM obfuscation tools not found, using basic obfuscation");
    false
}
