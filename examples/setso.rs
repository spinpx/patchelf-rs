use patchelf::*;

fn main() {
    let lib_name = std::env::args().nth(1).expect("no libname given");
    let new_name = std::env::args().nth(2).expect("no libname given");
    PatchElf::config().debug().input(&lib_name).set_soname(&new_name).patch();
}