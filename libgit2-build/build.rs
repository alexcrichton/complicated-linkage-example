extern crate "cmake-build" as cmake;

fn main() {
    // Like curl-rust, register out dependencies if they have metadata in our
    // env vars.
    //
    // This will adjust CMAKE_PREFIX_PATH and other env vars appropriately.
    cmake::register_dependency("openssl");
    cmake::register_dependency("ssh2");
    cmake::register_dependency("z");

    // We don't even bother trying to find a local installation of libgit2,
    // it's likely too old so we just always build from source.
    //
    // Note that cmake will automatically pass CMAKE_BUILD_TYPE based on the
    // PROFILE/OPT_LEVEL/DEBUG env vars and will adjust CMAKE_C_FLAGS
    // appropriately for the requested target.
    //
    // This will also build the appropriate build system for the platform being
    // hosted on.
    cmake::build([
        "-DTHREADSAFE=ON",
        "-DBUILD_SHARED_LIBS=OFF",
        "-DBUILD_CLAR=OFF",
        "-DBUILD_EXAMPLES=OFF",
    ]).unwrap();
}
