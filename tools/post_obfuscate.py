#!/usr/bin/env python3
"""
Post-build binary obfuscation tool for ShadowSniff
Applies additional obfuscation techniques after Rust compilation
"""

import sys
import os
import struct
import random
import hashlib
from pathlib import Path

class PEObfuscator:
    def __init__(self, pe_path):
        self.pe_path = Path(pe_path)
        self.pe_data = None
        
    def load_pe(self):
        """Load PE file into memory"""
        try:
            with open(self.pe_path, 'rb') as f:
                self.pe_data = bytearray(f.read())
            return True
        except Exception as e:
            print(f"Error loading PE: {e}")
            return False
    
    def save_pe(self, output_path=None):
        """Save modified PE file"""
        output_path = output_path or self.pe_path
        try:
            with open(output_path, 'wb') as f:
                f.write(self.pe_data)
            return True
        except Exception as e:
            print(f"Error saving PE: {e}")
            return False
    
    def add_fake_sections(self):
        """Add fake sections to confuse analysis tools"""
        print("Adding fake sections...")
        
        # This is a simplified implementation
        # Real implementation would need proper PE parsing
        fake_data = self.generate_fake_data(1024)
        
        # Append fake data at the end of file
        self.pe_data.extend(fake_data)
        
        return True
    
    def generate_fake_data(self, size):
        """Generate realistic-looking fake data"""
        fake_data = bytearray()
        
        # Add some fake strings
        fake_strings = [
            b"Microsoft Windows",
            b"System32\\kernel32.dll",
            b"CreateProcessW",
            b"GetModuleHandleA",
            b"LoadLibraryA",
            b"VirtualAlloc",
            b"SOFTWARE\\Microsoft\\Windows\\CurrentVersion",
            b"explorer.exe",
            b"notepad.exe",
            b"cmd.exe"
        ]
        
        for _ in range(size // 100):
            fake_data.extend(random.choice(fake_strings))
            fake_data.extend(b'\x00' * random.randint(1, 10))
        
        # Fill remaining space with random data
        while len(fake_data) < size:
            fake_data.extend(bytes([random.randint(0, 255)]))
        
        return fake_data[:size]
    
    def scramble_strings(self):
        """Obfuscate string constants in the binary"""
        print("Scrambling string constants...")
        
        # Simple XOR obfuscation of potential strings
        for i in range(len(self.pe_data) - 4):
            # Look for potential ASCII strings (4+ consecutive printable chars)
            if all(32 <= b <= 126 for b in self.pe_data[i:i+4]):
                # XOR with a simple key
                key = 0x42
                for j in range(4):
                    if i + j < len(self.pe_data):
                        self.pe_data[i + j] ^= key
        
        return True
    
    def add_junk_code(self):
        """Add junk code patterns to increase entropy"""
        print("Adding junk code patterns...")
        
        # Add random bytes at various positions
        junk_patterns = [
            b'\x90\x90\x90\x90',  # NOP sled
            b'\x83\xC0\x00',      # add eax, 0
            b'\x83\xE8\x00',      # sub eax, 0  
            b'\x85\xC0',          # test eax, eax
        ]
        
        for _ in range(50):  # Add 50 junk patterns
            pattern = random.choice(junk_patterns)
            pos = random.randint(len(self.pe_data) // 2, len(self.pe_data) - len(pattern))
            # Insert at random position (simplified)
            self.pe_data[pos:pos] = pattern
        
        return True
    
    def calculate_entropy(self):
        """Calculate file entropy"""
        if not self.pe_data:
            return 0.0
        
        # Count byte frequencies
        frequencies = [0] * 256
        for byte in self.pe_data:
            frequencies[byte] += 1
        
        # Calculate entropy
        entropy = 0.0
        data_len = len(self.pe_data)
        for freq in frequencies:
            if freq > 0:
                p = freq / data_len
                entropy -= p * (p.bit_length() - 1)
        
        return entropy

def main():
    if len(sys.argv) != 2:
        print("Usage: python post_obfuscate.py <pe_file>")
        sys.exit(1)
    
    pe_file = sys.argv[1]
    if not os.path.exists(pe_file):
        print(f"File not found: {pe_file}")
        sys.exit(1)
    
    print(f"Post-build obfuscating: {pe_file}")
    
    obfuscator = PEObfuscator(pe_file)
    
    if not obfuscator.load_pe():
        print("Failed to load PE file")
        sys.exit(1)
    
    # Calculate original entropy
    original_entropy = obfuscator.calculate_entropy()
    print(f"Original entropy: {original_entropy:.2f}")
    
    # Apply obfuscation techniques
    obfuscator.add_fake_sections()
    obfuscator.scramble_strings()
    obfuscator.add_junk_code()
    
    # Create backup
    backup_path = pe_file + ".original"
    if not os.path.exists(backup_path):
        os.rename(pe_file, backup_path)
    
    # Save obfuscated version
    if obfuscator.save_pe(pe_file):
        final_entropy = obfuscator.calculate_entropy()
        print(f"Final entropy: {final_entropy:.2f}")
        print(f"Entropy increase: {final_entropy - original_entropy:.2f}")
        print(f"Obfuscation complete! Backup saved as: {backup_path}")
    else:
        print("Failed to save obfuscated PE file")
        sys.exit(1)

if __name__ == "__main__":
    main()
