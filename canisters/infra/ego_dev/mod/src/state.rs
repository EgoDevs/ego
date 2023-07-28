use std::cell::RefCell;

use ego_macros::{inject_cycle_info, inject_ego_data, inject_seq_info};

use crate::memory::CONFIG;
use crate::service::EgoDevService;
use crate::types::stable_state::StableState;

inject_ego_data!();
inject_cycle_info!();
inject_seq_info!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
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

    match &state.users {
      None => {}
      Some(users) => {
        users_post_upgrade(users.clone());
      }
    }

    match &state.registry {
      None => {}
      Some(registry) => {
        registry_post_upgrade(registry.clone());
      }
    }

    match &state.cycle_info {
      None => {}
      Some(cycle_info) => {
        cycle_info_post_upgrade(cycle_info.clone());
      }
    }

    match &state.seq {
      None => {}
      Some(seq) => {
        seq_post_upgrade(seq.clone());
      }
    }
  });
}