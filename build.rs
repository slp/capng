use std::env;

const LIBCAPNG_LIB_NAME: &str = "cap-ng";
const LIBCAPNG_LIB_PATH: &str = "LIBCAPNG_LIB_PATH";
const LIBCAPNG_LINK_TYPE: &str = "LIBCAPNG_LINK_TYPE";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed={}", LIBCAPNG_LIB_PATH);
    println!("cargo:rerun-if-env-changed={}", LIBCAPNG_LINK_TYPE);

    if let Ok(path) = env::var(LIBCAPNG_LIB_PATH) {
        println!("cargo:rustc-link-search=native={}", path);
    }

    let link_type = match env::var(LIBCAPNG_LINK_TYPE) {
            Ok(val) if matches!(val.as_str(), "dylib" | "static") => val,
            _ => String::from("dylib"),
    };

    println!("cargo:rustc-link-lib={}={}", link_type, LIBCAPNG_LIB_NAME);
}
