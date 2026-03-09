# Binwalk WASM

This document summarizes the changes made to make `binwalk` compilable for the `wasm32-unknown-unknown` target, suitable for browser environments.

## Compilation

To compile the library for WASM, use the following command:

```bash
RUSTFLAGS="--cfg=getrandom_backend=\"wasm_js\"" cargo build --lib --target wasm32-unknown-unknown
```

Note: The `wasm_js` flag for `getrandom` is required for WASM targets in the browser to provide a source of entropy.

## Supported Functionality

The following functionalities are preserved in the WASM build:

- **Signature Scanning**: The core Aho-Corasick scanning engine is fully functional.
- **Signature Validation**: Most signature parsers still perform validation using internal, portable extractors (e.g., `gzip`, `zlib`, `lzma`, `bzip2`, `bmp`, `gif`, `jpeg`, `png`). This ensures high-confidence results even in the browser.
- **Structural Parsing**: All structural parsers are available, with fixes applied to support 32-bit architectures.

## Limitations on WASM

To support the browser environment, certain features have been conditionally compiled out or stubbed:

- **CLI Binary**: `src/main.rs` is excluded from WASM builds as it relies heavily on terminal I/O and the local filesystem.
- **Extraction to Disk**: Actual extraction of files to the local filesystem is disabled. The `Binwalk::extract` and `Binwalk::analyze` (file-based) methods are gated out.
- **External Extractors**: Support for external extraction utilities (like `7z`, `tar`, etc.) is removed.
- **Filesystem Operations**: The `Chroot` and file carving logic in `src/extractors/common.rs` act as no-ops or return success without performing actual I/O.
- **Entropy Plotting**: The `entropy::plot` function is a no-op on WASM as it depends on `plotly` features that are not portable.
- **Non-portable Dependencies**: Dependencies such as `termsize`, `walkdir`, `threadpool`, and `delink` are gated out for WASM targets.

## Usage in the Browser

The primary interface for WASM is `Binwalk::scan` or `Binwalk::analyze_buf`. You can provide a `&[u8]` buffer of the data to be analyzed and receive a list of identified signatures.

```rust
use binwalk::Binwalk;

let binwalker = Binwalk::new();
let data: &[u8] = ...; // Your data buffer
let results = binwalker.scan(data);

for result in results {
    println!("Found {} at offset {:#X}", result.description, result.offset);
}
```
