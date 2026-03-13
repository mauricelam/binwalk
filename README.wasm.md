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

When compiled for WebAssembly, certain non-portable features are conditionally compiled out or stubbed. Below is a detailed list of what is currently disabled and how it might be re-enabled in a browser environment.

### 1. Command-Line Interface (CLI)
- **Status:** Disabled. The CLI entry point in `src/main.rs` contains a `compile_error!` for WASM targets.
- **Why:** The CLI depends on many OS-specific features like standard input/output, signal handling, and process management.
- **Re-enabling:** In a browser, the CLI can be replaced by a web-based UI. If a command-line experience is desired, libraries like `xterm.js` can be used to create a terminal emulator that communicates with the WASM module.

### 2. Filesystem I/O
- **Status:** Stubbed. `std::fs` operations are mostly no-ops or return errors.
- **Details:** The `Chroot` struct in `src/extractors/common.rs` provides a unified interface for filesystem operations but its methods (like `create_file`, `create_directory`, `create_symlink`) do nothing on WASM.
- **Re-enabling:** Use the [File System Access API](https://developer.mozilla.org/en-US/docs/Web/API/File_System_Access_API) to allow the browser to interact with the local filesystem, or implement an in-memory virtual filesystem (e.g., using a crate like `memfs`) and map `Chroot` operations to it.

### 3. External Extractors (Subprocesses)
- **Status:** Disabled. External extraction utilities that rely on `std::process::Command` are not supported.
- **Details:** Many signatures (e.g., 7-zip, tar, squashfs, zstd) normally use external tools for extraction. These are currently set to `None` in `src/magic.rs` for WASM.
- **Re-enabling:** External utilities can be replaced by:
    - **Internal WASM Implementations:** Compiling the C/C++ source of those utilities to WASM and linking them.
    - **JS/WASM Libraries:** Calling out to existing JavaScript or WASM versions of these tools (e.g., `sqlglot` for SQL, `pako` for zlib).

### 4. Multithreading
- **Status:** Disabled for scanning. The CLI's multithreaded architecture is not available.
- **Why:** Rust's standard `std::thread` does not map directly to browser Web Workers without additional glue code like `wasm-bindgen-rayon` or `web-sys`.
- **Re-enabling:** Implement a worker-based pool using [Web Workers](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API). The library's `scan` and `extract` methods are currently single-threaded and can be called from within a Web Worker.

### 5. Entropy Plotting
- **Status:** Stubbed. The `plot` function in `src/entropy.rs` returns an error on WASM.
- **Why:** It depends on `plotly` with `kaleido` for image generation, which is not portable to the browser.
- **Re-enabling:** Use a client-side plotting library like `Plotly.js` or `Chart.js`. The WASM module can calculate the entropy data and return it as a JSON object (the `FileEntropy` struct is already serializable) for the frontend to render.

### 6. Specific Decryption Logic
- **Status:** Stubbed. Extractors that rely on the `delink` crate (like `encfw` and `mh01`) are disabled because `delink` is not currently WASM-compatible.
- **Re-enabling:** Port the required decryption logic to WASM or use the [Web Crypto API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API).

## What works?

- **Core Scanning:** The Aho-Corasick based signature scanning works fully in memory.
- **Signature Validation:** All internal signature parsers and validation logic are functional.
- **In-Memory Extraction:** Internal extractors (like `gzip`, `lzma`, `uimage`, `bmp`, `png`, `jpeg`, etc.) are functional, though they currently "extract" to nowhere unless a virtual filesystem is provided to the `Chroot` implementation.
- **32-bit Compatibility:** Structural parsers include fixes to prevent arithmetic overflows on 32-bit architectures, which is common for many WASM runtimes.

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
