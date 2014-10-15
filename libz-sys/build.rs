extern crate "pkg-config" as pkg_config;

fn main() {
    // Attempt to use pkg-config to find a the zlib library first in case it's
    // in a special location.
    match pkg_config::find_libary("z") {
        Ok(()) => return,
        Err(..) => {}
    }

    // Ok, let's just do the semi-lazy thing for now and hope the system has a
    // zlib library installed. Note that this is not robust for applications
    // such as cross compilation.
    println!("cargo:rustc-flags=-l dylib:z");
}
