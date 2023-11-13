//! Rust FFI for patchelf
//! ## Example for print soname.
//! ```ignore
//! PatchElf::new()
//!     .input("libpng.so")
//!     .print_soname()
//!     .run();
//! ```
//! ## Example for set soname.
//! ```ignore
//! PatchElf::new()
//!     .input("libpng.so")
//!     .output("libpng2.so")
//!     .set_soname("libpng2.so")
//!     .run();
//! ```

use std::ffi::CString;
pub struct PatchElf {
}

extern "C" {
    fn patchelf_run() -> bool;
    fn patchelf_set_input(name: *const i8);
    fn patchelf_set_soname(name: *const i8);
    fn patchelf_print_soname();
    fn patchelf_set_output(name: *const i8);

}
impl PatchElf {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn input(&self, name: &str) -> &Self {
        let c_name = CString::new(name).unwrap();
        unsafe {
            patchelf_set_input(c_name.as_ptr());
        }
        self
    }

    pub fn output(&self, name: &str) -> &Self {
        let c_name = CString::new(name).unwrap();
        unsafe {
            patchelf_set_output(c_name.as_ptr());
        }
        self
    }

    pub fn set_soname(&self, name: &str) -> &Self {
        let c_name = CString::new(name).unwrap();
        unsafe {
            patchelf_set_soname(c_name.as_ptr());
        }
        self
    }

    pub fn print_soname(&self) -> &Self {
        unsafe {
            patchelf_print_soname();
        }
        self
    }

    pub fn run(&self) {
        unsafe {
            if !patchelf_run() {
                eprintln!("fail to run");
            }
        }
    }

}

#[test]
fn test_print() {
    PatchElf::new()
     .input("/Users/chenpeng/Workspace/test/libtest.so")
     .print_soname()
     .run();
}