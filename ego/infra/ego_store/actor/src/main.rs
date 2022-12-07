mod lib;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use crate::lib::InitArg;
    use ego_store_mod::types::*;
    use ego_types::ego_error::EgoError;
    use ic_cdk::export::Principal;
    use std::collections::BTreeMap;
    use ego_store_mod::order::Order;
    use ego_store_mod::user_app::UserApp;
    use ego_store_mod::user_app::AppInstalled;


    candid::export_service!();
    std::print!("{}", __export_service());
}
