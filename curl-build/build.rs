extern crate "pkg-config" as pkg_config;
extern crate "make-build" as make;

use std::os;

fn main() {
    let target = os::getenv("TARGET").unwrap();
    let target = target.as_slice();

    // Register in pkg-config that we have a dependency named `openssl`. This
    // will probe the relevant environment variables to see if it can add
    // something to PKG_CONFIG_PATH. This is used for both the configure and
    // pkg-config calls below.
    pkg_config::register_dependency("openssl");

    // OSX almost always has libcurl installed in a system location, so there's
    // nothing much we need to do that's extra
    if target.contains("-apple-") {
        return println!("cargo:rustc-flags=-lcurl");
    }

    // When building libcurl, it's commonly installed on the system, so let's
    // try to use pkg-config to probe the system to see if it's installed.
    //
    // This assumes that the `find_library` function will print necessary output
    // onto stdout as necessary.
    //
    // NOTE: it's easy for this to go wrong in terms of cross compilation as
    //       pkg-config has to have an env var to tell it to not look at the
    //       host libs.
    match pkg_config::find_library("libcurl") {
        Ok(()) => return,
        Err(..) => {}
    }

    // If pkg-config failed, attempt to use curl-config to see if its installed.
    //
    // NOTE: again, this may have surprising results with cross compilation.
    match curl_config() {
        Ok(()) => return,
        Err(..) => {}
    }

    // Ok, system detection failed, we need to build from source.
    //
    // Our ficticious library `make` knows to cd into OUT_DIR first before
    // configuring, and will pass these options along with all necessary CFLAGS
    // for the current architecture.
    //
    // The default behavior also knows to pass `--enable-optimize` as necessary
    // as well as `--prefix` into `$OUT_DIR`.
    //
    // The `make_install` step also understands to print relevant metadata on
    // stdout like the include dir for openssl.
    let mut args = vec![
        "--enable-static=yes", "--enable-shared=no",
        // lots of --disable flags...
    ];
    if target.contains("mingw") || target.contains("windows") {
        args.push("--with-winssl");
    }
    make::configure(args.as_slice()).unwrap();
    make::make().unwrap();
    make::make_install().unwrap();
}

// Try executing the local `curl-config` binary if one exists and parse its
// output (similar to pkg-config).
fn curl_config() -> Result<(), ()> {
    fail!()
}
