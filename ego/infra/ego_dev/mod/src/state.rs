use std::cell::RefCell;

use crate::ego_dev::EgoDev;

use ego_macros::{inject_canister_log, inject_canister_registry, inject_canister_users};
use crate::service::EgoDevService;

inject_canister_log!();
inject_canister_registry!();
inject_canister_users!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  let _ = match name {
    "ego_file" => {
      EgoDevService::admin_ego_file_add(canister_id);
    }
    "ego_store" => user_add(canister_id),
    _ => {}
  };
}

thread_local! {
  pub static EGO_DEV: RefCell<EgoDev> = RefCell::new(EgoDev::new());
}
