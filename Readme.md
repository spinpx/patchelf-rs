# patchelf-rs

Rust FFI for patchelf that we can call patchelf directly in a dirty way.

```
[dependencies]
patchelf = "0.1.0"
```

## Usage
- set soname
```rust
PatchElf::new()
   .input("libpng.so")
   .output("libpng2.so")
   .set_soname("libpng2.so")
   .run();
```

- print soname
```rust
PatchElf::new()
   .input("libz.so")
   .print_soname()
   .run();
```