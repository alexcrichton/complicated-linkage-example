// Make sure we link in openssl support
#[cfg(not(windows))]
extern crate "openssl-sys" as openssl;

// like miniz-sys, note the lack of #[link] directives
extern {
    // ffi definitions of libssh2
}
