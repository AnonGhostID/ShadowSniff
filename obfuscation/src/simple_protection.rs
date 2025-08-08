/*
 * Simplified Binary Protection Module (for testing)
 */

use core::arch::asm;

/// Basic binary protection functions
pub struct SimpleBinaryProtection;

impl SimpleBinaryProtection {
    /// Basic XOR encryption
    pub fn encrypt_data(data: &mut [u8], key: u8) {
        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= key.wrapping_add(i as u8);
        }
    }
    
    /// Basic XOR decryption (same as encryption)
    pub fn decrypt_data(data: &mut [u8], key: u8) {
        Self::encrypt_data(data, key);
    }
    
    /// Simple checksum calculation
    pub fn calculate_checksum(data: &[u8]) -> u32 {
        let mut checksum = 0u32;
        for &byte in data {
            checksum = checksum.wrapping_add(byte as u32);
        }
        checksum
    }
}