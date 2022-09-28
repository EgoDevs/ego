mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ego_tenant_mod::types::*;
    use ego_types::ego_error::EgoError;
    use ic_cdk::export::Principal;

    candid::export_service!();
    std::print!("{}", __export_service());
}
