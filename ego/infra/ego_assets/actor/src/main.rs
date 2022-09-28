mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ego_assets_mod::rc_bytes::*;
    use ego_assets_mod::state_machine::*;
    use ego_assets_mod::types::*;
    use ic_types::Principal;
    candid::export_service!();
    std::print!("{}", __export_service());
}
