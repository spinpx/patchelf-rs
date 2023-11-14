
// C style API for patchelf.
#include <iostream>

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

void patchelf_clear() {
  debugMode = false;
  fileNames.clear();
  setSoname = false;
  printSoname = false;
  newSoname.clear();
  outputFileName.clear();
#ifdef DEFAULT_PAGESIZE
  forcedPageSize = DEFAULT_PAGESIZE;
#else
  forcedPageSize = -1;
#endif
  setOsAbi = false;
  newOsAbi.clear();
  newInterpreter.clear();
  shrinkRPath = false;
  setRPath = false;
  newRPath.clear();
  removeRPath = false;
  addRPath = false;
  newRPath.clear();
  forceRPath = false;
  allowedRpathPrefixes.clear();
  neededLibsToAdd.clear();
  neededLibsToReplace.clear();
  neededLibsToRemove.clear();
}

void patchelf_debug() { debugMode = true; }

void patchelf_set_input(char* name) { fileNames.push_back(std::string(name)); }

void patchelf_set_soname(char* name) {
  setSoname = true;
  newSoname = std::string(name);
}

void patchelf_print_soname() { printSoname = true; }

void patchelf_set_output(char* name) { outputFileName = std::string(name); }

void patchelf_set_page_size(int size) {
  forcedPageSize = size;
}

void patchelf_set_osabi(char* name) {
  setOsAbi = true;
  newOsAbi = std::string(name);
}

void patchelf_set_interpreter(char* name) {
  newInterpreter = std::string(name);
}

void patchelf_shrink_rpath() {
  shrinkRPath = true;
}

void patchelf_set_rpath(char* name) {
  setRPath = true;
  newRPath = std::string(name);
}

void patchelf_remove_rpath() {
  removeRPath = true;
}

void patchelf_add_rpath(char* name) {
  addRPath = true;
  newRPath = std::string(name);
}

void patchelf_force_rpath() {
  forceRPath = true;
}

void patchelf_allowed_rpath_prefixes(char* name) {
  allowedRpathPrefixes.push_back(std::string(name));
}

void patchelf_add_needed(char* name) {
  neededLibsToAdd.insert(std::string(name));
}

void patchelf_replace_needed(char* from, char* to) {
  neededLibsToReplace[ std::string(from)] = std::string(to);
}

void patchelf_remove_needed(char* name) {
   neededLibsToRemove.insert(std::string(name));
}

}