mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ic_cdk::export::Principal;
    use ego_log_mod::log::Log;
    use crate::lib::InitArg;
    candid::export_service!();
    std::print!("{}", __export_service());
}
