mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ego_utils::types::EgoError;

    use ego_file_mod::types::*;

    candid::export_service!();
    std::print!("{}", __export_service());
}
