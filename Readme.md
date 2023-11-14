# patchelf-rs

[![Crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]

Rust FFI for patchelf that we can call patchelf directly in a dirty way.

[crates-badge]: https://img.shields.io/crates/v/patchelf.svg
[crates-url]: https://crates.io/crates/patchelf
[docs-badge]: https://docs.rs/patchelf/badge.svg
[docs-url]: https://docs.rs/patchelf/

```toml
[dependencies]
patchelf = "0.2.0"
```

## Usage
- set soname
```rust
PatchElf::config()
   .input("libpng.so")
   .output("libpng2.so")
   .set_soname("libpng2.so")
   .patch();
```

- print soname
```rust
PatchElf::config()
   .input("libz.so")
   .print_soname()
   .patch();
```

Seed [doc][docs-url] for more usages.