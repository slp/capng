fn main() {
    pkg_config::probe_library("libcap-ng").unwrap();
}
