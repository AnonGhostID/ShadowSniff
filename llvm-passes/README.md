# LLVM Pass Integration for ShadowSniff

This directory contains custom LLVM passes for advanced obfuscation.

## Available Passes

### 1. Instruction Substitution Pass
Replaces simple instructions with functionally equivalent but more complex sequences.

### 2. Control Flow Flattening Pass  
Flattens control flow by converting if-else chains into switch-case state machines.

### 3. Opaque Predicate Insertion Pass
Inserts conditional branches that always evaluate to true/false but are hard to analyze statically.

### 4. Function Call Indirection Pass
Replaces direct function calls with indirect calls through function pointers.

## Building LLVM Passes

These passes would need to be compiled as LLVM plugins and integrated into the Rust compilation pipeline.

## Alternative: Compiler Plugin Approach

Since custom LLVM passes are complex to integrate with Rust's compilation model, we use compiler flags and build-time code generation instead.

## Integration Points

The build system automatically applies these techniques through:
1. Build-time code generation
2. Macro-based obfuscation  
3. Linker-level transformations
4. Post-build processing
