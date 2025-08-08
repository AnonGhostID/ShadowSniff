/*
 * Enhanced String Obfuscation Module
 */

use obfstr::obfstr;
use rand::{RngCore, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Advanced string obfuscation beyond basic obfstr
pub struct ObfuscatedString {
    data: &'static [u8],
    key: u64,
    length: usize,
}

impl ObfuscatedString {
    /// Create a new obfuscated string at compile time
    pub const fn new(data: &'static [u8], key: u64) -> Self {
        Self {
            data,
            key,
            length: data.len(),
        }
    }
    
    /// Decrypt string at runtime with additional obfuscation
    pub fn decrypt(&self) -> alloc::string::String {
        use alloc::string::String;
        use alloc::vec::Vec;
        
        let mut result = Vec::with_capacity(self.length);
        let mut working_key = self.key;
        
        // Multi-layer decryption with key evolution
        for (i, &byte) in self.data.iter().enumerate() {
            // Evolve the key for each byte
            working_key = working_key.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
            let key_byte = ((working_key >> (8 * (i % 8))) & 0xFF) as u8;
            
            // XOR with evolved key
            let decrypted = byte ^ key_byte ^ ((i as u8).wrapping_mul(0xAA));
            result.push(decrypted);
        }
        
        String::from_utf8_lossy(&result).into_owned()
    }
}

/// Macro for creating obfuscated strings with compile-time encryption
#[macro_export]
macro_rules! obf_string {
    ($s:expr) => {{
        const fn encrypt_string(s: &str) -> ([u8; 256], u64, usize) {
            let bytes = s.as_bytes();
            let mut encrypted = [0u8; 256];
            let key = 0x1337DEADBEEF1337u64; // Could be randomized per build
            let mut working_key = key;
            
            let mut i = 0;
            while i < bytes.len() && i < 256 {
                // Evolve key
                working_key = working_key.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
                let key_byte = ((working_key >> (8 * (i % 8))) & 0xFF) as u8;
                
                // Encrypt byte
                encrypted[i] = bytes[i] ^ key_byte ^ ((i as u8).wrapping_mul(0xAA));
                i += 1;
            }
            
            (encrypted, key, bytes.len())
        }
        
        const ENCRYPTED_DATA: ([u8; 256], u64, usize) = encrypt_string($s);
        static DATA: [u8; 256] = ENCRYPTED_DATA.0;
        
        $crate::string_obfuscation::ObfuscatedString::new(&DATA[..ENCRYPTED_DATA.2], ENCRYPTED_DATA.1)
    }};
}

/// Runtime string mutation to prevent static analysis
pub struct MutableString {
    data: alloc::vec::Vec<u8>,
    mutations: u32,
}

impl MutableString {
    pub fn new(s: &str) -> Self {
        use alloc::vec::Vec;
        
        let mut data = Vec::from(s.as_bytes());
        Self::mutate_buffer(&mut data);
        
        Self {
            data,
            mutations: 0,
        }
    }
    
    pub fn get(&mut self) -> alloc::string::String {
        use alloc::string::String;
        
        // Restore original data
        Self::restore_buffer(&mut self.data);
        let result = String::from_utf8_lossy(&self.data).into_owned();
        
        // Re-mutate for next access
        Self::mutate_buffer(&mut self.data);
        self.mutations = self.mutations.wrapping_add(1);
        
        result
    }
    
    fn mutate_buffer(data: &mut [u8]) {
        // Simple XOR mutation with rotating key
        let key = 0xAAu8;
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key.wrapping_add(i as u8);
        }
    }
    
    fn restore_buffer(data: &mut [u8]) {
        // Reverse the mutation
        let key = 0xAAu8;
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key.wrapping_add(i as u8);
        }
    }
}

/// Polymorphic string generation
pub struct PolymorphicString {
    templates: &'static [&'static str],
    rng_state: u64,
}

impl PolymorphicString {
    pub fn new(templates: &'static [&'static str]) -> Self {
        Self {
            templates,
            rng_state: 0x1337DEADBEEF,
        }
    }
    
    pub fn generate(&mut self) -> &'static str {
        // Update RNG state
        self.rng_state = self.rng_state.wrapping_mul(0x5DEECE66D).wrapping_add(0xB);
        let index = (self.rng_state as usize) % self.templates.len();
        self.templates[index]
    }
}

/// Stack-based string obfuscation for temporary strings
pub fn obf_stack_string<F, R>(s: &str, f: F) -> R
where
    F: FnOnce(&str) -> R,
{
    use alloc::vec::Vec;
    
    // Create obfuscated copy on stack
    let mut obfuscated = Vec::from(s.as_bytes());
    
    // Apply obfuscation
    let key = 0x55u8;
    for (i, byte) in obfuscated.iter_mut().enumerate() {
        *byte ^= key ^ (i as u8);
    }
    
    // Deobfuscate and use
    for (i, byte) in obfuscated.iter_mut().enumerate() {
        *byte ^= key ^ (i as u8);
    }
    
    let deobfuscated = core::str::from_utf8(&obfuscated).unwrap_or("");
    let result = f(deobfuscated);
    
    // Clear stack data
    obfuscated.fill(0);
    
    result
}

/// String interning with obfuscation
static mut STRING_POOL: Option<alloc::collections::BTreeMap<u64, ObfuscatedString>> = None;

pub fn init_string_pool() {
    use alloc::collections::BTreeMap;
    
    unsafe {
        STRING_POOL = Some(BTreeMap::new());
    }
}

pub fn intern_string(s: &str) -> u64 {
    use alloc::collections::BTreeMap;
    
    // Hash the string
    let mut hash = 0x811C9DC5u64; // FNV-1a offset
    for &byte in s.as_bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x100000001B3); // FNV-1a prime
    }
    
    unsafe {
        if let Some(ref mut pool) = STRING_POOL {
            if !pool.contains_key(&hash) {
                // Create obfuscated version
                let data = s.as_bytes();
                let key = hash;
                
                // Store encrypted version
                let obf_str = ObfuscatedString::new(data, key);
                pool.insert(hash, obf_str);
            }
        }
    }
    
    hash
}

pub fn get_interned_string(id: u64) -> Option<alloc::string::String> {
    unsafe {
        STRING_POOL.as_ref()?.get(&id).map(|obf| obf.decrypt())
    }
}

/// Anti-analysis string techniques
pub mod anti_analysis {
    use super::*;
    
    /// Split string across multiple variables to avoid detection
    pub fn split_string(s: &str) -> (alloc::string::String, alloc::string::String) {
        use alloc::string::String;
        
        let mid = s.len() / 2;
        let (first, second) = s.split_at(mid);
        (String::from(first), String::from(second))
    }
    
    /// Build string character by character to avoid static detection
    pub fn build_string_dynamically(chars: &[char]) -> alloc::string::String {
        use alloc::string::String;
        
        let mut result = String::new();
        for &c in chars {
            result.push(c);
            
            // Add noise operations
            let _ = result.len();
        }
        result
    }
    
    /// Create fake strings to confuse analysts
    pub fn create_decoy_strings() {
        use obfstr::obfstr as s;
        
        let _decoys = [
            s!("This is a decoy string"),
            s!("Fake API call GetProcAddress"),
            s!("Dummy malware signature"),
            s!("Red herring configuration"),
        ];
        
        // Don't actually use these, just have them in binary
    }
}