use std::cell::RefCell;

use ego_macros::{inject_cycle_info, inject_ego_data};

use crate::service::EgoStoreService;
use crate::store::EgoStore;

inject_ego_data!();
inject_cycle_info!();



/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  info_log_add(&format!("on_canister_added name: {}, canister_id: {}", name, canister_id));
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
