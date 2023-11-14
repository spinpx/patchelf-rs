//! Rust FFI for patchelf
//! ## Example for print soname.
//! ```ignore
//! PatchElf::config()
//!     .input("libpng.so")
//!     .print_soname()
//!     .patch();
//! ```
//! ## Example for set soname.
//! ```ignore
//! PatchElf::config()
//!     .input("libpng.so")
//!     .output("libpng2.so")
//!     .set_soname("libpng2.so")
//!     .patch();
//! ```

use std::ffi::CString;
pub struct PatchElf {
    debug: bool,
    inputs: Vec<String>,
    output: Option<String>,
    action: PatchAction,
    page_size: Option<isize>,
    allowed_rpath_prefixes: Vec<String>,
}

pub enum PatchAction {
    Nop,
    SetSoname { so_name: String },
    PrintSoName,
    SetOsAbi { abi: String },
    SetInterpreter { interpreter: String },
    ShrinkRpath,
    SetRpath { rpath: String },
    RemoveRpath,
    AddRpath { rpath: String },
    ForceRpath,
    AddNeeded { needed: String },
    RemoveNeeded { needed: String },
    ReplaceNeeded { from: String, to: String },
}

extern "C" {
    fn patchelf_run() -> bool;
    fn patchelf_clear();
    fn patchelf_debug();
    fn patchelf_set_input(name: *const i8);
    fn patchelf_set_soname(name: *const i8);
    fn patchelf_print_soname();
    fn patchelf_set_output(name: *const i8);
    fn patchelf_set_page_size(size: isize);
    fn patchelf_set_osabi(name: *const i8);
    fn patchelf_set_interpreter(name: *const i8);
    fn patchelf_shrink_rpath();
    fn patchelf_set_rpath(name: *const i8);
    fn patchelf_remove_rpath();
    fn patchelf_add_rpath(name: *const i8);
    fn patchelf_force_rpath();
    fn patchelf_allowed_rpath_prefixes(name: *const i8);
    fn patchelf_add_needed(name: *const i8);
    fn patchelf_remove_needed(name: *const i8);
    fn patchelf_replace_needed(from: *const i8, to: *const i8);
}
impl PatchElf {
    pub fn config() -> Self {
        Self {
            debug: false,
            inputs: vec![],
            output: None,
            action: PatchAction::Nop,
            page_size: None,
            allowed_rpath_prefixes: vec![],
        }
    }

    pub fn debug(mut self) -> Self {
        self.debug = true;
        self
    }

    pub fn page_size(mut self, page_size: isize) -> Self {
        self.page_size = Some(page_size);
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

    pub fn set_osabi(mut self, name: &str) -> Self {
        self.action = PatchAction::SetOsAbi { 
            abi: name.to_string(),
        };
        self
    }

    pub fn set_interpreter(mut self, name: &str) -> Self {
        self.action = PatchAction::SetInterpreter {
            interpreter: name.to_string(),
        };
        self
    }

    pub fn shrink_rpath(mut self) -> Self {
        self.action = PatchAction::ShrinkRpath;
        self
    }

    pub fn remove_rpath(mut self) -> Self {
        self.action = PatchAction::RemoveRpath;
        self
    }

    pub fn force_rpath(mut self) -> Self {
        self.action = PatchAction::ForceRpath;
        self
    }

    pub fn set_set_rpath(mut self, name: &str) -> Self {
        self.action = PatchAction::SetRpath {
            rpath: name.to_string(),
        };
        self
    }

    pub fn set_add_rpath(mut self, name: &str) -> Self {
        self.action = PatchAction::AddRpath {
            rpath: name.to_string(),
        };
        self
    }

    pub fn allowed_rpath_prefixes(mut self, name: &str) -> Self {
        self.allowed_rpath_prefixes.push(name.to_string());
        self
    }

    pub fn set_add_needed(mut self, name: &str) -> Self {
        self.action = PatchAction::AddNeeded {
            needed: name.to_string(),
        };
        self
    }

    pub fn set_remove_needed(mut self, name: &str) -> Self {
        self.action = PatchAction::RemoveNeeded {
            needed: name.to_string(),
        };
        self
    }

    pub fn set_replace_needed(mut self, from: &str, to: &str) -> Self {
        self.action = PatchAction::ReplaceNeeded {
            from: from.to_string(),
            to: to.to_string()
        };
        self
    }

    pub fn patch(&self) -> bool {
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
            if let Some(page_size) = &self.page_size {
                patchelf_set_page_size(*page_size);
            }
            for prefix in &self.allowed_rpath_prefixes {
                let c_name = CString::new(prefix.as_str()).unwrap();
                patchelf_allowed_rpath_prefixes(c_name.as_ptr());
            }
            match &self.action {
                PatchAction::PrintSoName => {
                    patchelf_print_soname();
                }
                PatchAction::SetSoname { so_name } => {
                    let c_name = CString::new(so_name.as_str()).unwrap();
                    patchelf_set_soname(c_name.as_ptr());
                }
                PatchAction::SetOsAbi { abi } => {
                    let c_name = CString::new(abi.as_str()).unwrap();
                    patchelf_set_osabi(c_name.as_ptr()); 
                }
                PatchAction::SetInterpreter { interpreter } => {
                    let c_name = CString::new(interpreter.as_str()).unwrap();
                    patchelf_set_interpreter(c_name.as_ptr()); 
                }
                PatchAction::ShrinkRpath => {
                    patchelf_shrink_rpath();
                }
                PatchAction::SetRpath { rpath } => {
                    let c_name = CString::new(rpath.as_str()).unwrap();
                    patchelf_set_rpath(c_name.as_ptr());  
                }
                PatchAction::RemoveRpath => {
                    patchelf_remove_rpath();
                }
                PatchAction::ForceRpath => {
                    patchelf_force_rpath();
                }
                PatchAction::AddRpath { rpath } => {
                    let c_name = CString::new(rpath.as_str()).unwrap();
                    patchelf_add_rpath(c_name.as_ptr());  
                }
                PatchAction::AddNeeded { needed } => {
                    let c_name = CString::new(needed.as_str()).unwrap();
                    patchelf_add_needed(c_name.as_ptr());  
                }
                PatchAction::RemoveNeeded { needed } => {
                    let c_name = CString::new(needed.as_str()).unwrap();
                    patchelf_remove_needed(c_name.as_ptr());  
                }
                PatchAction::ReplaceNeeded { from, to } => {
                    let from = CString::new(from.as_str()).unwrap();
                    let to = CString::new(to.as_str()).unwrap();
                    patchelf_replace_needed(from.as_ptr(), to.as_ptr());  
                }
                _ => {}
            }
            return patchelf_run();
        }
    }
}
