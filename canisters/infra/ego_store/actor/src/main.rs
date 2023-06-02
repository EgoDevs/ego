mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ego_store_mod::app::EgoStoreApp;
    use ego_store_mod::order::Order;
    use ego_store_mod::types::*;
    use ego_types::app::EgoError;
    use ego_types::app::UserApp;
    use ego_types::app::{App, AppId, CashFlow};
    use ego_types::cycle_info::*;
    use ic_cdk::export::Principal;
    use serde_bytes::ByteBuf;
    use std::collections::BTreeMap;

    use ic_ledger_types::Memo;

    candid::export_service!();
    std::print!("{}", __export_service());
}
