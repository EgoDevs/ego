use std::cell::RefCell;

use ego_backup::inject_backup_data;

use ego_macros::{inject_cycle_info, inject_ego_data, inject_seq_info};

use crate::memory::CONFIG;
use crate::service::EgoDevService;
use crate::types::stable_state::StableState;

inject_ego_data!();
inject_cycle_info!();
inject_seq_info!();
inject_backup_data!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  info_log_add(&format!(
    "ego_dev: on_canister_added name: {}, canister_id: {}",
    name, canister_id
  ));

  let _ = match name {
    "ego_file" => {
      EgoDevService::admin_ego_file_add(&canister_id);
    }
    "ego_store" => user_add(canister_id),
    _ => {}
  };
}

pub fn pre_upgrade() {
  // composite StableState
  let stable_state = StableState {
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
    cycle_info: Some(cycle_info_pre_upgrade()),
    backup_info: Some(backup_info_pre_upgrade()),
    seq: Some(seq_pre_upgrade()),
  };

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