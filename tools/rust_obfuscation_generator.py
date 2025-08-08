"""
Rust-integrated obfuscation pass generator
Generates obfuscated code at build time to be included in Rust compilation
"""

import random
import hashlib
import os

class RustObfuscationGenerator:
    """Generates obfuscated Rust code patterns"""
    
    def __init__(self, seed: int = None):
        self.seed = seed or int.from_bytes(os.urandom(4), 'big')
        random.seed(self.seed)
    
    def generate_opaque_predicates(self, count: int = 10) -> str:
        """Generate opaque predicate functions"""
        predicates = []
        
        for i in range(count):
            # Always true predicate
            x = random.randint(10, 100)
            y = random.randint(10, 100)
            
            predicate_code = f"""
#[inline(never)]
fn opaque_predicate_{i}_true() -> bool {{
    let x = {x}u64;
    let y = {y}u64;
    (x * x + y * y) >= (x * y * 2)  // Always true: (a-b)² >= 0
}}

#[inline(never)]  
fn opaque_predicate_{i}_false() -> bool {{
    let x = {x}u64;
    let y = {y}u64;
    (x * x + y * y) < (x * y * 2)   // Always false: (a-b)² >= 0
}}
"""
            predicates.append(predicate_code)
        
        return '\n'.join(predicates)
    
    def generate_dummy_functions(self, count: int = 20) -> str:
        """Generate dummy computational functions"""
        functions = []
        
        operations = ['+', '-', '*', '^', '&', '|']
        
        for i in range(count):
            params = random.randint(2, 5)
            param_list = ', '.join([f'p{j}: u64' for j in range(params)])
            
            # Generate complex computation
            computation = f"let mut result = p0;"
            for j in range(1, params):
                op = random.choice(operations)
                computation += f"\n    result = result {op} p{j};"
            
            # Add some loops and conditions
            computation += f"""
    for i in 1..{random.randint(5, 20)} {{
        result = result.wrapping_mul(i).wrapping_add({random.randint(1, 255)});
    }}
    
    if result % 2 == 0 {{
        result = result.wrapping_shl({random.randint(1, 3)});
    }} else {{
        result = result.wrapping_shr({random.randint(1, 3)});
    }}"""
            
            function_code = f"""
#[inline(never)]
fn dummy_computation_{i}({param_list}) -> u64 {{
    {computation}
    result
}}
"""
            functions.append(function_code)
        
        return '\n'.join(functions)
    
    def generate_control_flow_flattening(self, function_name: str, states: list) -> str:
        """Generate state machine for control flow flattening"""
        
        state_cases = []
        for i, state_code in enumerate(states):
            next_state = (i + 1) % len(states) if i < len(states) - 1 else 999  # 999 = exit
            
            case_code = f"""
        {i} => {{
            {state_code}
            obf_state = {next_state};
        }}"""
            state_cases.append(case_code)
        
        flattened_function = f"""
#[inline(never)]
fn {function_name}_flattened() {{
    let mut obf_state = 0u32;
    
    loop {{
        match obf_state {{
{chr(10).join(state_cases)}
            999 => break,
            _ => unreachable!(),
        }}
    }}
}}
"""
        return flattened_function
    
    def generate_string_encryption_table(self, strings: list) -> str:
        """Generate encrypted string table"""
        key = os.urandom(16)
        encrypted_strings = []
        
        for i, s in enumerate(strings):
            encrypted = self._xor_encrypt(s.encode('utf-8'), key)
            encrypted_hex = ', '.join([f'0x{b:02x}' for b in encrypted])
            
            encrypted_strings.append(f"""
const ENCRYPTED_STRING_{i}: &[u8] = &[{encrypted_hex}];""")
        
        key_hex = ', '.join([f'0x{b:02x}' for b in key])
        
        decryption_code = f"""
const DECRYPTION_KEY: &[u8] = &[{key_hex}];

#[inline(never)]
fn decrypt_string(encrypted: &[u8]) -> String {{
    let decrypted: Vec<u8> = encrypted
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ DECRYPTION_KEY[i % DECRYPTION_KEY.len()])
        .collect();
    
    String::from_utf8_lossy(&decrypted).into_owned()
}}

// Usage macro
macro_rules! obfuscated_string {{
    ($index:expr) => {{
        match $index {{
{chr(10).join([f'            {i} => decrypt_string(ENCRYPTED_STRING_{i}),' for i in range(len(strings))])}
            _ => String::new(),
        }}
    }};
}}
"""
        
        return '\n'.join(encrypted_strings) + '\n' + decryption_code
    
    def generate_function_indirection_table(self, function_names: list) -> str:
        """Generate function pointer indirection table"""
        
        function_pointers = []
        for i, func_name in enumerate(function_names):
            function_pointers.append(f"    {func_name} as *const ()")
        
        indirection_code = f"""
static FUNCTION_TABLE: [*const (); {len(function_names)}] = [
{chr(10).join(function_pointers)}
];

#[inline(never)]
fn call_function_indirect(index: usize) {{
    if index < FUNCTION_TABLE.len() {{
        unsafe {{
            let func: fn() = std::mem::transmute(FUNCTION_TABLE[index]);
            func();
        }}
    }}
}}

// Obfuscated function call macro
macro_rules! obf_call {{
    ($index:expr) => {{
        call_function_indirect($index);
    }};
}}
"""
        return indirection_code
    
    def generate_complete_obfuscation_module(self) -> str:
        """Generate complete obfuscation module"""
        
        sample_strings = [
            "System initialization",
            "Processing data",
            "Network connection", 
            "File operation",
            "Registry access",
            "Memory allocation"
        ]
        
        sample_states = [
            "// State 0: Initialize\nlet mut counter = 0u64;",
            "// State 1: Process\ncounter = counter.wrapping_add(1);",
            "// State 2: Validate\nif counter > 100 { counter = 0; }",
            "// State 3: Cleanup\n// Cleanup operations"
        ]
        
        module_code = f"""
//! Auto-generated obfuscation module
//! Generated with seed: {self.seed}

#![allow(dead_code)]
#![allow(unused_variables)]

use std::hint::black_box;

// Opaque predicates
{self.generate_opaque_predicates(5)}

// Dummy computational functions  
{self.generate_dummy_functions(10)}

// String encryption
{self.generate_string_encryption_table(sample_strings)}

// Control flow flattening example
{self.generate_control_flow_flattening("sample_function", sample_states)}

// Anti-optimization helpers
#[inline(never)]
pub fn confuse_optimizer() {{
    for i in 0..10 {{
        if opaque_predicate_0_true() {{
            black_box(dummy_computation_0(i, i * 2, i * 3));
        }}
    }}
}}

// Runtime integrity check
#[inline(never)]
pub fn runtime_integrity_check() -> bool {{
    let expected_seed = {self.seed};
    let current_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    // Simple integrity check based on build-time seed
    (current_time % 1000) != (expected_seed as u64 % 1000)
}}

// Main obfuscation entry point
pub fn apply_runtime_obfuscation() {{
    confuse_optimizer();
    
    if runtime_integrity_check() {{
        // Additional obfuscation if integrity check passes
        for i in 0..3 {{
            black_box(obfuscated_string!(i));
        }}
    }}
}}
"""
        return module_code
    
    def _xor_encrypt(self, data: bytes, key: bytes) -> bytes:
        """Simple XOR encryption"""
        return bytes(d ^ key[i % len(key)] for i, d in enumerate(data))

def main():
    import argparse
    
    parser = argparse.ArgumentParser(description='Generate obfuscated Rust code')
    parser.add_argument('--output', '-o', required=True, help='Output file path')
    parser.add_argument('--seed', '-s', type=int, help='Random seed')
    args = parser.parse_args()
    
    generator = RustObfuscationGenerator(args.seed)
    obfuscated_code = generator.generate_complete_obfuscation_module()
    
    with open(args.output, 'w') as f:
        f.write(obfuscated_code)
    
    print(f"Generated obfuscated code in: {args.output}")
    print(f"Seed used: {generator.seed}")

if __name__ == "__main__":
    main()
