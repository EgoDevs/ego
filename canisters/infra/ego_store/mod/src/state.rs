use std::cell::RefCell;

use ego_backup::inject_backup_data;

use ego_macros::{inject_cycle_info, inject_ego_data, inject_seq_info};

use crate::memory::CONFIG;
use crate::types::stable_state::StableState;
use crate::types::tenant::Tenant;

inject_ego_data!();
inject_cycle_info!();
inject_backup_data!();
inject_seq_info!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  info_log_add(&format!(
    "ego_store: on_canister_added name: {}, canister_id: {}",
    name, canister_id
  ));
  let _ = match name {
    "ego_dev" => user_add(canister_id),
    "ego_ledger" => user_add(canister_id),
    "ego_tenant" => {
      user_add(canister_id);
      Tenant::new(&canister_id).save();
    }
    _ => {}
  };
}

pub fn pre_upgrade() {
  // composite StableState
  let stable_state = StableState::load();

  CONFIG.with(|config| {
    config.borrow_mut().set(stable_state).expect("persist stable state failed");
  });
}

pub fn post_upgrade() {
  CONFIG.with(|config| {
    let config_borrow = config.borrow();
    let state = config_borrow.get();

    StableState::restore(state.to_owned());
  });
}