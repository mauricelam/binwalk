# Binwalk for WebAssembly (Browser)

This project is now compilable for the `wasm32-unknown-unknown` target, allowing it to be used in web browsers.

## Building for WASM

To build the library for WebAssembly:

```bash
RUSTFLAGS="--cfg=getrandom_backend=\"wasm_js\"" cargo build --target wasm32-unknown-unknown --release
```

### Why `getrandom_backend="wasm_js"`?

The `getrandom` crate (v0.3), used by `uuid`, requires explicit backend selection when targeting `wasm32-unknown-unknown`.

Since WebAssembly (without WASI) does not provide a standard host entropy source that the Rust standard library can automatically discover, you must explicitly tell `getrandom` to use the browser's JavaScript-based entropy source (`crypto.getRandomValues()`). This is done by passing the `--cfg=getrandom_backend="wasm_js"` flag to the compiler.

## Functionality on WASM

When compiled for WebAssembly, certain non-portable features are conditionally compiled out or stubbed:

1.  **CLI:** The command-line interface in `src/main.rs` is disabled. Attempting to build the binary for WASM will result in a compile-time error.
2.  **Filesystem I/O:** `std::fs` operations are mostly no-ops or return errors on WASM.
3.  **Subprocesses:** External extraction utilities (calling `Command::new`) are disabled.
4.  **Multithreading:** The CLI's multithreaded scanning is not available.
5.  **Plotting:** Entropy plotting (which depends on `plotly` and `kaleido`) is disabled.

### What works?

-   **Core Scanning:** The Aho-Corasick based signature scanning works fully in memory.
-   **Signature Validation:** All internal signature parsers and validation logic are functional.
-   **In-Memory Extraction:** Internal extractors that perform decompression or data manipulation in memory are functional.

## Library Usage

In a browser environment, you should use `binwalk` as a library:

```rust
use binwalk::Binwalk;

// Initialize binwalker
let binwalker = Binwalk::new();

// Scan a buffer (e.g., from a File object or Fetch API)
let file_data: Vec<u8> = ...;
let results = binwalker.scan(&file_data);

for result in results {
    println!("Found {} at offset {:#X}", result.description, result.offset);
}
```
