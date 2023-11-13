use patchelf::*;

fn main() {
    let lib_name = std::env::args().nth(1).expect("no libname given");
    PatchElf::new().debug().input(&lib_name).print_soname().run();
}