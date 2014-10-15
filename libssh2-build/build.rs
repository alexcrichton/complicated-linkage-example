extern crate "pkg-config" as pkg_config;
extern crate "make-build" as make;

use std::os;

fn main() {
    let target = os::getenv("TARGET").unwrap();
    let target = target.as_slice();

    // Register a dependency with make to ensure that CFLAGS/LDFLAGS are updated
    // accordingly with the output of the package `openssl`.
    make::register_dependency("openssl");

    // like libgit2, just don't bother looking at the system and always build a
    // static local copy.
    //
    // Note that this will also pass --prefix and adjust CFLAGS appropriately
    let mut args = vec![
        "--enable-shared=no",
        "--disable-examples-build"
    ];
    if target.contains("mingw") || target.contains("windows") {
        args.push("--with-wincng");
        args.push("--without-openssl");
    }
    make::configure(args.as_slice()).unwrap();
    make::make().unwrap();
    make::make_install().unwrap();
}
