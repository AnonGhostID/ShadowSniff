# Standard release build (original)
release:
    RUSTFLAGS="-Z location-detail=none \
               -Z fmt-debug=none \
               -C debuginfo=0 \
               -C link-arg=/OPT:REF \
               -C link-arg=/OPT:ICF \
               -C link-arg=/INCREMENTAL:NO \
               -C link-arg=/DEBUG:NONE \
               -C link-arg=/RELEASE \
               " \
    cargo build --release

# Light obfuscation - Basic optimizations
release-light:
    $env:OBFUSCATION_LEVEL="light"; \
    $env:RUSTFLAGS="-Z location-detail=none \
                    -Z fmt-debug=none \
                    -C debuginfo=0 \
                    -C link-arg=/OPT:REF \
                    -C link-arg=/OPT:ICF \
                    -C link-arg=/INCREMENTAL:NO \
                    -C link-arg=/DEBUG:NONE \
                    -C link-arg=/RELEASE \
                    "; \
    cargo build --release --features builder_build

# Medium obfuscation - String obfuscation + CFG
release-medium:
    $env:OBFUSCATION_LEVEL="medium"; \
    $env:RUSTFLAGS="-Z location-detail=none \
                    -Z fmt-debug=none \
                    -C debuginfo=0 \
                    -C link-arg=/OPT:REF \
                    -C link-arg=/OPT:ICF \
                    -C link-arg=/GUARD:CF \
                    -C link-arg=/DYNAMICBASE \
                    -C link-arg=/INCREMENTAL:NO \
                    -C link-arg=/DEBUG:NONE \
                    -C link-arg=/RELEASE \
                    "; \
    cargo build --release --features builder_build

# Heavy obfuscation - Control flow + layout randomization
release-heavy:
    $env:OBFUSCATION_LEVEL="heavy"; \
    $env:RUSTFLAGS="-Z location-detail=none \
                    -Z fmt-debug=none \
                    -Z randomize-layout \
                    -C debuginfo=0 \
                    -C link-arg=/OPT:REF \
                    -C link-arg=/OPT:ICF \
                    -C link-arg=/GUARD:CF \
                    -C link-arg=/DYNAMICBASE \
                    -C link-arg=/INTEGRITYCHECK \
                    -C link-arg=/NXCOMPAT \
                    -C link-arg=/INCREMENTAL:NO \
                    -C link-arg=/DEBUG:NONE \
                    -C link-arg=/RELEASE \
                    "; \
    cargo build --release --features builder_build

# Maximum obfuscation - All techniques enabled
release-maximum:
    $env:OBFUSCATION_LEVEL="maximum"; \
    $env:RUSTFLAGS="-Z location-detail=none \
                    -Z fmt-debug=none \
                    -Z randomize-layout \
                    -C debuginfo=0 \
                    -C link-arg=/OPT:REF \
                    -C link-arg=/OPT:ICF \
                    -C link-arg=/GUARD:CF \
                    -C link-arg=/DYNAMICBASE \
                    -C link-arg=/INTEGRITYCHECK \
                    -C link-arg=/NXCOMPAT \
                    -C link-arg=/MERGE:.rdata=.text \
                    -C link-arg=/DELAYLOAD:kernel32.dll \
                    -C link-arg=/DELAYLOAD:user32.dll \
                    -C link-arg=/INCREMENTAL:NO \
                    -C link-arg=/DEBUG:NONE \
                    -C link-arg=/RELEASE \
                    "; \
    cargo build --release --features builder_build

# Builder-specific obfuscated releases
builder-light:
    $env:OBFUSCATION_LEVEL="light"; \
    $env:RUSTFLAGS="-Z location-detail=none -Z fmt-debug=none -C debuginfo=0"; \
    cargo build --release --bin builder --features builder_build

builder-medium:
    $env:OBFUSCATION_LEVEL="medium"; \
    $env:RUSTFLAGS="-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -Z randomize-layout"; \
    cargo build --release --bin builder --features builder_build

builder-maximum:
    $env:OBFUSCATION_LEVEL="maximum"; \
    $env:RUSTFLAGS="-Z location-detail=none -Z fmt-debug=none -C debuginfo=0 -Z randomize-layout"; \
    cargo build --release --bin builder --features builder_build

# Post-build obfuscation (requires external tools)
post-obfuscate: release-maximum
    @echo "Running post-build obfuscation..."
    @if (Test-Path "tools\post_obfuscate.bat") { .\tools\post_obfuscate.bat } else { echo "Post-obfuscation tools not found" }
    @echo "Post-obfuscation complete"

# Advanced PE-level obfuscation
advanced-pe-obfuscate: release-maximum
    @echo "Applying advanced PE obfuscation..."
    @if (Test-Path "tools\advanced_pe_obfuscator.py") { python tools\advanced_pe_obfuscator.py target\release\ShadowSniff.exe --all } else { echo "Advanced PE obfuscator not found" }
    @echo "Advanced PE obfuscation complete"

# Runtime packing with encryption
pack-runtime: release-maximum
    @echo "Applying runtime packing..."
    @if (Test-Path "tools\runtime_packer.py") { python tools\runtime_packer.py target\release\ShadowSniff.exe target\release\ShadowSniff_packed.exe --polymorphic --stats } else { echo "Runtime packer not found" }
    @echo "Runtime packing complete"

# Complete obfuscation pipeline
obfuscate-complete: clean release-maximum post-obfuscate
    @echo "Complete obfuscation pipeline finished!"
    @echo "Check target\release\ for obfuscated binaries"

# Ultimate obfuscation - all techniques
obfuscate-ultimate: clean release-maximum advanced-pe-obfuscate pack-runtime
    @echo "Ultimate obfuscation pipeline complete!"
    @echo "Files generated:"
    @if (Test-Path "target\release\ShadowSniff.exe") { echo "  - target\release\ShadowSniff.exe (advanced obfuscated)" }
    @if (Test-Path "target\release\ShadowSniff_packed.exe") { echo "  - target\release\ShadowSniff_packed.exe (packed + encrypted)" }

# Generate obfuscation report
obfuscation-report:
    @echo "=== ShadowSniff Obfuscation Report ==="
    @echo "Available obfuscation levels:"
    @echo "  light       - Basic optimizations (5-10% size increase)"
    @echo "  medium      - String obfuscation + CFG (15-25% size increase)" 
    @echo "  heavy       - Control flow + layout randomization (30-50% size increase)"
    @echo "  maximum     - All techniques (50-100% size increase)"
    @echo ""
    @echo "Advanced techniques:"
    @echo "  PE-level    - Binary structure obfuscation"
    @echo "  Packing     - Runtime encryption + compression"
    @echo "  Polymorphic - Code mutation for each build"
    @echo ""
    @echo "Usage examples:"
    @echo "  just release-medium          # Recommended for production"
    @echo "  just obfuscate-complete      # Full pipeline"
    @echo "  just obfuscate-ultimate      # Maximum protection"

debug:
    cargo build

run:
    cargo run

# Clean all build artifacts
clean:
    cargo clean

# Show obfuscation information
obfuscation-info:
    @echo "Available obfuscation levels:"
    @echo "  light    - Basic optimizations and stripping"
    @echo "  medium   - + String obfuscation, CFG, ASLR"
    @echo "  heavy    - + Control flow obfuscation, layout randomization"
    @echo "  maximum  - + Anti-debugging, section merging, import hiding"
    @echo ""
    @echo "Usage: just release-[level]"
    @echo "Example: just release-maximum"