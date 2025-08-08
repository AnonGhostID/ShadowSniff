use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use sha2::{Sha256, Digest};

/// Advanced string obfuscation using XOR with key derivation
pub struct StringObfuscator {
    key: Vec<u8>,
}

impl StringObfuscator {
    pub fn new(seed: &str) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(seed.as_bytes());
        let key = hasher.finalize().to_vec();
        
        Self { key }
    }

    pub fn obfuscate(&self, data: &str) -> Vec<u8> {
        data.bytes()
            .enumerate()
            .map(|(i, byte)| byte ^ self.key[i % self.key.len()])
            .collect()
    }

    pub fn deobfuscate(&self, data: &[u8]) -> String {
        let bytes: Vec<u8> = data
            .iter()
            .enumerate()
            .map(|(i, &byte)| byte ^ self.key[i % self.key.len()])
            .collect();
        
        String::from_utf8_lossy(&bytes).into_owned()
    }
}

/// Generate dummy string data for padding
pub fn generate_dummy_strings(count: usize, seed: u64) -> Vec<String> {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut dummy_strings = Vec::with_capacity(count);
    
    let words = [
        "system", "process", "memory", "thread", "handle", "registry", "service",
        "driver", "kernel", "user", "admin", "security", "policy", "config",
        "temp", "cache", "log", "data", "file", "directory", "path", "url",
        "network", "socket", "connection", "protocol", "packet", "header",
        "response", "request", "client", "server", "browser", "cookie",
    ];
    
    for _ in 0..count {
        let word_count = rng.gen_range(2..=6);
        let mut dummy = String::new();
        
        for i in 0..word_count {
            if i > 0 { dummy.push('_'); }
            dummy.push_str(words[rng.gen_range(0..words.len())]);
        }
        
        dummy_strings.push(dummy);
    }
    
    dummy_strings
}

/// Unicode obfuscation - convert ASCII to similar Unicode chars
pub fn unicode_obfuscate(input: &str) -> String {
    input.chars().map(|c| {
        match c {
            'a' => 'α',  // Greek small letter alpha
            'e' => 'е',  // Cyrillic small letter ie
            'o' => 'о',  // Cyrillic small letter o
            'p' => 'р',  // Cyrillic small letter er
            'c' => 'с',  // Cyrillic small letter es
            'x' => 'х',  // Cyrillic small letter ha
            'y' => 'у',  // Cyrillic small letter u
            _ => c,
        }
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_obfuscation() {
        let obfuscator = StringObfuscator::new("test_key");
        let original = "Hello, World!";
        let obfuscated = obfuscator.obfuscate(original);
        let deobfuscated = obfuscator.deobfuscate(&obfuscated);
        
        assert_eq!(original, deobfuscated);
        assert_ne!(original.as_bytes(), obfuscated.as_slice());
    }
}
