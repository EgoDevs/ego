use std::cell::RefCell;

use crate::ego_tenant::EgoTenant;
use ego_macros::{inject_canister_log, inject_canister_registry, inject_canister_users};
inject_canister_log!();
inject_canister_registry!();
inject_canister_users!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  let _ = match name {
    "ego_store" => user_add(canister_id),
    _ => {}
  };
}

thread_local! {
  pub static EGO_TENANT: RefCell<EgoTenant> = RefCell::new(EgoTenant::new());
}
