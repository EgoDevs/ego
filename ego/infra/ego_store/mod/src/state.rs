use std::cell::RefCell;

use crate::ego_store::EgoStore;

use ego_macros::{inject_canister_log, inject_canister_registry, inject_canister_users};
use crate::service::EgoStoreService;
inject_canister_log!();
inject_canister_registry!();
inject_canister_users!();


/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  log_add(&format!("on_canister_added name: {}, canister_id: {}", name, canister_id));
  let _ = match name {
    "ego_dev" => user_add(canister_id),
    "ego_ledger" => user_add(canister_id),
    "ego_tenant" => {
      user_add(canister_id);
      EgoStoreService::admin_ego_tenant_add(canister_id);
    }
    _ => {}
  };
}

thread_local! {
  pub static EGO_STORE: RefCell<EgoStore> = RefCell::new(EgoStore::new());
}
