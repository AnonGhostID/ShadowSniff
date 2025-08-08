#!/usr/bin/env python3
"""
Advanced PE manipulation and obfuscation tool
Implements sophisticated binary-level protection techniques
"""

import sys
import os
import struct
import random
import hashlib
import zlib
from pathlib import Path
from typing import Optional, List, Tuple
import time

class PEParser:
    """Advanced PE file parser and manipulator"""
    
    def __init__(self, pe_path: Path):
        self.pe_path = pe_path
        self.pe_data = bytearray()
        self.dos_header = {}
        self.nt_headers = {}
        self.sections = []
        self.imports = []
        self.exports = []
        
    def load(self) -> bool:
        """Load and parse PE file"""
        try:
            with open(self.pe_path, 'rb') as f:
                self.pe_data = bytearray(f.read())
            
            self._parse_dos_header()
            self._parse_nt_headers()
            self._parse_sections()
            self._parse_imports()
            
            return True
        except Exception as e:
            print(f"Error loading PE: {e}")
            return False
    
    def save(self, output_path: Optional[Path] = None) -> bool:
        """Save modified PE file"""
        output_path = output_path or self.pe_path
        try:
            with open(output_path, 'wb') as f:
                f.write(self.pe_data)
            return True
        except Exception as e:
            print(f"Error saving PE: {e}")
            return False
    
    def _parse_dos_header(self):
        """Parse DOS header"""
        if len(self.pe_data) < 64:
            raise ValueError("Invalid DOS header")
        
        dos_sig = struct.unpack('<H', self.pe_data[0:2])[0]
        if dos_sig != 0x5A4D:  # 'MZ'
            raise ValueError("Invalid DOS signature")
        
        self.dos_header = {
            'e_magic': dos_sig,
            'e_lfanew': struct.unpack('<L', self.pe_data[60:64])[0]
        }
    
    def _parse_nt_headers(self):
        """Parse NT headers"""
        nt_offset = self.dos_header['e_lfanew']
        if nt_offset + 24 > len(self.pe_data):
            raise ValueError("Invalid NT headers")
        
        nt_sig = struct.unpack('<L', self.pe_data[nt_offset:nt_offset+4])[0]
        if nt_sig != 0x00004550:  # 'PE\0\0'
            raise ValueError("Invalid PE signature")
        
        # Parse file header
        machine = struct.unpack('<H', self.pe_data[nt_offset+4:nt_offset+6])[0]
        num_sections = struct.unpack('<H', self.pe_data[nt_offset+6:nt_offset+8])[0]
        
        self.nt_headers = {
            'signature': nt_sig,
            'machine': machine,
            'number_of_sections': num_sections,
            'offset': nt_offset
        }
    
    def _parse_sections(self):
        """Parse section headers"""
        nt_offset = self.nt_headers['offset']
        opt_header_size = struct.unpack('<H', self.pe_data[nt_offset+20:nt_offset+22])[0]
        sections_offset = nt_offset + 24 + opt_header_size
        
        self.sections = []
        for i in range(self.nt_headers['number_of_sections']):
            section_offset = sections_offset + (i * 40)
            if section_offset + 40 > len(self.pe_data):
                break
            
            name = self.pe_data[section_offset:section_offset+8].rstrip(b'\x00')
            virtual_size = struct.unpack('<L', self.pe_data[section_offset+8:section_offset+12])[0]
            virtual_address = struct.unpack('<L', self.pe_data[section_offset+12:section_offset+16])[0]
            raw_size = struct.unpack('<L', self.pe_data[section_offset+16:section_offset+20])[0]
            raw_address = struct.unpack('<L', self.pe_data[section_offset+20:section_offset+24])[0]
            
            self.sections.append({
                'name': name.decode('ascii', errors='ignore'),
                'virtual_size': virtual_size,
                'virtual_address': virtual_address,
                'raw_size': raw_size,
                'raw_address': raw_address,
                'offset': section_offset
            })
    
    def _parse_imports(self):
        """Parse import table (simplified)"""
        # This is a simplified implementation
        # Real implementation would need full PE parsing
        self.imports = []

class AdvancedObfuscator:
    """Advanced obfuscation techniques"""
    
    def __init__(self, pe_parser: PEParser):
        self.pe = pe_parser
        self.encryption_key = None
        
    def generate_encryption_key(self) -> bytes:
        """Generate random encryption key"""
        self.encryption_key = os.urandom(32)
        return self.encryption_key
    
    def encrypt_section(self, section_name: str) -> bool:
        """Encrypt a specific section"""
        print(f"Encrypting section: {section_name}")
        
        section = None
        for s in self.pe.sections:
            if s['name'].startswith(section_name):
                section = s
                break
        
        if not section:
            print(f"Section {section_name} not found")
            return False
        
        # Simple XOR encryption
        if not self.encryption_key:
            self.generate_encryption_key()
        
        start = section['raw_address']
        size = min(section['raw_size'], section['virtual_size'])
        
        for i in range(size):
            if start + i < len(self.pe.pe_data):
                self.pe.pe_data[start + i] ^= self.encryption_key[i % len(self.encryption_key)]
        
        return True
    
    def add_decryption_stub(self) -> bool:
        """Add decryption code to binary (simplified)"""
        print("Adding decryption stub...")
        
        # In a real implementation, this would inject assembly code
        # that decrypts sections at runtime
        decryption_code = b'\x90' * 64  # NOP sled placeholder
        
        # Append to end of file (simplified)
        self.pe.pe_data.extend(decryption_code)
        return True
    
    def obfuscate_imports(self) -> bool:
        """Obfuscate import table"""
        print("Obfuscating import table...")
        
        # Find and modify import names
        for section in self.pe.sections:
            if section['name'] == '.idata':
                start = section['raw_address']
                size = section['raw_size']
                
                # Simple string obfuscation in import section
                for i in range(start, min(start + size, len(self.pe.pe_data) - 8)):
                    # Look for potential API names
                    if self.pe.pe_data[i:i+4] == b'Get':
                        # XOR obfuscate
                        for j in range(8):
                            if i + j < len(self.pe.pe_data):
                                self.pe.pe_data[i + j] ^= 0xAA
        
        return True
    
    def add_anti_debug_code(self) -> bool:
        """Inject anti-debugging code"""
        print("Injecting anti-debugging code...")
        
        # Anti-debug assembly (x86_64)
        anti_debug_shellcode = bytes([
            0x65, 0x48, 0x8B, 0x04, 0x25, 0x60, 0x00, 0x00, 0x00,  # mov rax, gs:[60h] ; PEB
            0x48, 0x8B, 0x40, 0x02,                                  # mov rax, [rax+2] ; BeingDebugged
            0x84, 0xC0,                                              # test al, al
            0x75, 0x05,                                              # jnz exit_process
            0xC3,                                                    # ret (continue normally)
            0x48, 0x31, 0xC9,                                        # xor rcx, rcx
            0xFF, 0x15, 0x00, 0x00, 0x00, 0x00                      # call [ExitProcess]
        ])
        
        # Find a good place to inject (simplified - append to end)
        self.pe.pe_data.extend(anti_debug_shellcode)
        return True
    
    def polymorphic_transform(self) -> bool:
        """Apply polymorphic transformation"""
        print("Applying polymorphic transformation...")
        
        # Replace common instruction patterns with equivalent ones
        transformations = [
            (b'\x48\x31\xC0', b'\x48\x33\xC0'),  # xor rax,rax -> xor rax,rax
            (b'\x48\x89\xC1', b'\x48\x8B\xC8'),  # mov rcx,rax -> mov rcx,rax  
            (b'\x90\x90\x90', b'\x66\x90\x90'),  # nop nop nop -> 66 nop nop
        ]
        
        for old_pattern, new_pattern in transformations:
            data = bytes(self.pe.pe_data)
            offset = 0
            while True:
                offset = data.find(old_pattern, offset)
                if offset == -1:
                    break
                
                # Replace pattern
                for i, byte in enumerate(new_pattern):
                    if offset + i < len(self.pe.pe_data):
                        self.pe.pe_data[offset + i] = byte
                
                offset += len(old_pattern)
        
        return True
    
    def add_fake_functions(self) -> bool:
        """Add fake function calls to confuse analysis"""
        print("Adding fake function calls...")
        
        fake_functions = [
            b'CreateFileW\x00',
            b'RegOpenKeyW\x00', 
            b'GetWindowsDirectoryW\x00',
            b'LoadLibraryW\x00',
            b'GetProcAddress\x00'
        ]
        
        # Add fake function names to confuse static analysis
        for fake_func in fake_functions:
            # Find a good place to insert (simplified)
            insert_pos = len(self.pe.pe_data) // 2
            self.pe.pe_data[insert_pos:insert_pos] = fake_func
        
        return True
    
    def calculate_complexity_metrics(self) -> dict:
        """Calculate obfuscation complexity metrics"""
        data = bytes(self.pe.pe_data)
        
        # Calculate entropy
        entropy = self._calculate_entropy(data)
        
        # Count unique byte patterns
        unique_bytes = len(set(data))
        
        # Count potential strings (simplified)
        string_count = 0
        for i in range(len(data) - 4):
            if all(32 <= b <= 126 for b in data[i:i+4]):
                string_count += 1
        
        return {
            'entropy': entropy,
            'unique_bytes': unique_bytes,
            'string_patterns': string_count,
            'file_size': len(data),
            'sections': len(self.pe.sections)
        }
    
    def _calculate_entropy(self, data: bytes) -> float:
        """Calculate Shannon entropy"""
        if not data:
            return 0.0
        
        frequencies = [0] * 256
        for byte in data:
            frequencies[byte] += 1
        
        entropy = 0.0
        data_len = len(data)
        for freq in frequencies:
            if freq > 0:
                p = freq / data_len
                entropy -= p * (p.bit_length() - 1 if p > 0 else 0)
        
        return entropy

def main():
    if len(sys.argv) < 2:
        print("Usage: python advanced_pe_obfuscator.py <pe_file> [options]")
        print("Options:")
        print("  --encrypt-text    Encrypt .text section")
        print("  --encrypt-rdata   Encrypt .rdata section")  
        print("  --anti-debug      Add anti-debugging code")
        print("  --polymorphic     Apply polymorphic transformation")
        print("  --fake-functions  Add fake function references")
        print("  --all             Apply all techniques")
        sys.exit(1)
    
    pe_file = Path(sys.argv[1])
    if not pe_file.exists():
        print(f"File not found: {pe_file}")
        sys.exit(1)
    
    # Parse options
    options = sys.argv[2:] if len(sys.argv) > 2 else ['--all']
    
    print(f"Advanced PE obfuscation: {pe_file}")
    print(f"Options: {' '.join(options)}")
    
    # Load and parse PE
    parser = PEParser(pe_file)
    if not parser.load():
        print("Failed to load PE file")
        sys.exit(1)
    
    obfuscator = AdvancedObfuscator(parser)
    
    # Calculate original metrics
    original_metrics = obfuscator.calculate_complexity_metrics()
    print(f"Original entropy: {original_metrics['entropy']:.3f}")
    
    # Create backup
    backup_path = pe_file.with_suffix(pe_file.suffix + '.advanced_backup')
    if not backup_path.exists():
        import shutil
        shutil.copy2(pe_file, backup_path)
    
    # Apply obfuscation techniques
    if '--encrypt-text' in options or '--all' in options:
        obfuscator.encrypt_section('.text')
        obfuscator.add_decryption_stub()
    
    if '--encrypt-rdata' in options or '--all' in options:
        obfuscator.encrypt_section('.rdata')
    
    if '--anti-debug' in options or '--all' in options:
        obfuscator.add_anti_debug_code()
    
    if '--polymorphic' in options or '--all' in options:
        obfuscator.polymorphic_transform()
    
    if '--fake-functions' in options or '--all' in options:
        obfuscator.add_fake_functions()
        obfuscator.obfuscate_imports()
    
    # Save obfuscated file
    if not parser.save():
        print("Failed to save obfuscated file")
        sys.exit(1)
    
    # Calculate final metrics
    final_metrics = obfuscator.calculate_complexity_metrics()
    
    print("\n=== Obfuscation Results ===")
    print(f"Original entropy: {original_metrics['entropy']:.3f}")
    print(f"Final entropy: {final_metrics['entropy']:.3f}")
    print(f"Entropy increase: {final_metrics['entropy'] - original_metrics['entropy']:.3f}")
    print(f"Size change: {final_metrics['file_size'] - original_metrics['file_size']} bytes")
    print(f"Backup saved as: {backup_path}")
    print("Advanced obfuscation complete!")

if __name__ == "__main__":
    main()
