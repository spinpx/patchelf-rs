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
    debug: bool,
    inputs: Vec<String>,
    output: Option<String>,
    action: PatchAction,
}

pub enum PatchAction {
    Nop,
    SetSoname { so_name: String },
    PrintSoName,
}

extern "C" {
    fn patchelf_run() -> bool;
    fn patchelf_clear();
    fn patchelf_debug();
    fn patchelf_set_input(name: *const i8);
    fn patchelf_set_soname(name: *const i8);
    fn patchelf_print_soname();
    fn patchelf_set_output(name: *const i8);

}
impl PatchElf {
    pub fn new() -> Self {
        Self {
            debug: false,
            inputs: vec![],
            output: None,
            action: PatchAction::Nop,
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn input(mut self, name: &str) -> Self {
        self.inputs.push(name.to_string());
        self
    }

    pub fn output(mut self, name: &str) -> Self {
        self.output = Some(name.to_string());
        self
    }

    pub fn set_soname(mut self, name: &str) -> Self {
        self.action = PatchAction::SetSoname {
            so_name: name.to_string(),
        };
        self
    }

    pub fn print_soname(mut self) -> Self {
        self.action = PatchAction::PrintSoName;
        self
    }

    pub fn run(&self) {
        unsafe {
            patchelf_clear();
            if self.debug {
                patchelf_debug();
            }
            for input in &self.inputs {
                let c_name = CString::new(input.as_str()).unwrap();
                patchelf_set_input(c_name.as_ptr());
            }
            if let Some(output) = &self.output {
                let c_name = CString::new(output.as_str()).unwrap();
                patchelf_set_output(c_name.as_ptr());
            }
            match &self.action {
                PatchAction::PrintSoName => {
                    patchelf_print_soname();
                }
                PatchAction::SetSoname { so_name } => {
                    let c_name = CString::new(so_name.as_str()).unwrap();
                    patchelf_set_soname(c_name.as_ptr());
                }
                _ => {}
            }
            if !patchelf_run() {
                eprintln!("fail to run");
            }
        }
    }
}

