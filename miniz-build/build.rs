// hypothetical crate which abstracts over gcc/clang/msvc/etc
extern crate gcc;

// Note that we do not inspect `os::args` as we're just going to unconditionally
// build miniz. If miniz weren't being built (such as because of a local
// override) then we wouldn't be getting built.
fn main() {

    // Invoke a library function of the `gcc` library which will assemble
    // `libminiz.a` from the set of compiled input files passed as the second
    // argument. The input files are all relative to $OUT_DIR, and
    // `build_library` will print the appropriate `rustc-flags` output onto
    // stdout, but won't print any other metadata.
    //
    // Note that this knows to add flags like -m32/-m64 based on $TARGET as well
    // as passing -fPIC/-ffunction-sections/-fdata-sections as necessary. This
    // also knows to optimize code based on $OPT_LEVEL and generate debuginfo
    // based on $DEBUG.
    gcc::build_library("miniz", ["miniz.c"]);
}
