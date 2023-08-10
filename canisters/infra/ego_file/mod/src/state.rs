use std::cell::RefCell;

use ego_macros::{inject_cycle_info, inject_ego_data};

use crate::storage::Storage;

inject_ego_data!();
inject_cycle_info!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  info_log_add(&format!(
    "ego_file: on_canister_added name: {}, canister_id: {}",
    name, canister_id
  ));
  let _ = match name {
    "ego_dev" => user_add(canister_id),
    "ego_tenant" => user_add(canister_id),
    _ => {}
  };
}

thread_local! {
   pub static STORAGE: RefCell<Storage> = RefCell::new(Storage::new());
}
