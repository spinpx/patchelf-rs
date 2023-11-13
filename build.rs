use std::{path::PathBuf, env};

fn main() {
    // remove main function
    let path = "patchelf/src/patchelf.cc";
    let mut code = std::fs::read_to_string(path).unwrap();
    code = code.replace("int main(", "int __patchelf_entry(");
    let ffi = std::fs::read_to_string("ffi.cc").unwrap();
    code.push_str(&ffi);
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let new_path = out_path.join("patchelf.cc");
    std::fs::write(&new_path, code).unwrap();
    cc::Build::new()
        .cpp(true)
        .file(&new_path)
        .include("patchelf/src/")
        .flag_if_supported("-Wall")
        .flag_if_supported("-Wextra")
        .flag_if_supported("-Wcast-qual")
        .define("FILE_OFFSET_BITS", "64")
        .std("c++17")
        //.shared_flag(true)
        .static_flag(true)
        .compile("patchelf");
}

