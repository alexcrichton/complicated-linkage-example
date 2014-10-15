extern crate libc;

pub struct mz_stream;

// Note the lack of #[link], it's not this package's job to indicate where these
// symbols come from
extern {
    pub fn mz_deflate(stream: *mut mz_stream, flush: libc::c_int) -> libc::c_int;
    pub fn mz_deflateEnd(stream: *mut mz_stream) -> libc::c_int;
    // ...
}
