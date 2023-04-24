mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {

    use ic_cdk::export::Principal;
    use ego_record_mod::record::*;
    use ego_types::cycle_info::*;
    use std::collections::BTreeMap;
    use crate::actor::InitArg;

    candid::export_service!();
    std::print!("{}", __export_service());
}

