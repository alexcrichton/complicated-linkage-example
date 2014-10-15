extern crate "pkg-confg" as pkg_config;

fn main() {
    pkg_config::find_library("openssl").unwrap();
}
