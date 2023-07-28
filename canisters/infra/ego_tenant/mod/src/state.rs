use std::cell::RefCell;

use ego_macros::{inject_cycle_info, inject_ego_data};

use crate::memory::CONFIG;
use crate::types::stable_state::StableState;

inject_ego_data!();
inject_cycle_info!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  info_log_add(&format!(
    "on_canister_added name: {}, canister_id: {}",
    name, canister_id
  ));
  let _ = match name {
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
  });
}
