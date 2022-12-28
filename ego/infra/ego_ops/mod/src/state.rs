use std::cell::RefCell;

use crate::ego_ops::EgoOps;

use ego_macros::{inject_canister_log, inject_canister_registry, inject_canister_users};

inject_canister_log!();
inject_canister_users!();
inject_canister_registry!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  log_add(&format!("on_canister_added name: {}, canister_id: {}", name, canister_id));
}

thread_local! {
  pub static EGO_OPS: RefCell<EgoOps> = RefCell::new(EgoOps::new());
}
