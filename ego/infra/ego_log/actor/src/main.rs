mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::lib::InitArg;
    use ego_log_mod::log::Log;
    use ic_cdk::export::Principal;
    candid::export_service!();
    std::print!("{}", __export_service());
}
