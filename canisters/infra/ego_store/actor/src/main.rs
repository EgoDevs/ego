mod actor;

#[allow(dead_code)]
#[cfg(any(target_arch = "wasm32", test))]
fn main() {}

#[allow(dead_code)]
#[cfg(not(any(target_arch = "wasm32", test)))]
fn main() {
    use ego_store_mod::types::ego_store_app::EgoStoreApp;
    use ego_store_mod::types::order::Order;
    use ego_store_mod::types::wallet_provider::WalletProvider;
    use ego_store_mod::types::*;
    use ego_types::app::EgoError;
    use ego_types::app::UserApp;
    use ego_types::types::*;
    use ego_types::app::{App, AppId, CashFlow};
    use ego_store_mod::types::wallet::Wallet;
    use ego_types::cycle_info::*;
    use ic_cdk::export::Principal;
    use std::collections::BTreeMap;

    use ic_ledger_types::Memo;

    candid::export_service!();
    std::print!("{}", __export_service());
}
