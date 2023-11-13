
// C style API for patchelf.
#include<iostream>

extern "C" {
bool patchelf_run() {
  try {
    patchElf();
    return true;
  } catch (const std::exception& e) {
    std::cout << "Caught " << e.what() << std::endl;
    return false;
  }
}

void patchelf_set_input(char* name) { fileNames.push_back(std::string(name)); }

void patchelf_set_soname(char* name) {
  setSoname = true;
  newSoname = std::string(name);
}

void patchelf_print_soname() { printSoname = true; }

void patchelf_set_output(char* name) { outputFileName = std::string(name); }
}