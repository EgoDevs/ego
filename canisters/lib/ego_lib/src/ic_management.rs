use candid::Principal;
use ic_cdk::api::management_canister::main::{
  canister_status, CanisterIdRecord, CanisterSettings, CanisterStatusResponse, update_settings,
  UpdateSettingsArgument,
};

use ego_types::app::EgoError;

pub async fn controllers_update(
  canister_id: Principal,
  controllers: Vec<Principal>,
) -> Result<(), EgoError> {
  let in_arg = UpdateSettingsArgument {
    canister_id,
    settings: CanisterSettings {
      controllers: Some(controllers),
      compute_allocation: None,
      memory_allocation: None,
      freezing_threshold: None,
    },
  };

  match update_settings(in_arg).await {
    Ok(_) => Ok(()),
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }
}

pub async fn canister_status_get(
  canister_id: Principal,
) -> Result<CanisterStatusResponse, EgoError> {
  match canister_status(CanisterIdRecord { canister_id }).await {
    Ok(result) => Ok(result.0),
    Err((code, msg)) => {
      let code = code as u16;
      Err(EgoError { code, msg })
    }
  }
}

pub async fn controller_add(canister_id: Principal, principal: Principal) -> Result<(), EgoError> {
  let status_resp = canister_status_get(canister_id).await?;

  let mut current_controllers = status_resp.settings.controllers;

  if !current_controllers.contains(&principal) {
    current_controllers.push(principal);
    controllers_update(canister_id, current_controllers).await
  } else {
    Ok(())
  }
}

pub async fn controller_remove(
  canister_id: Principal,
  principal: Principal,
) -> Result<(), EgoError> {
  let status_resp = canister_status_get(canister_id).await?;

  let mut current_controllers = status_resp.settings.controllers;

  if current_controllers.contains(&principal) {
    current_controllers.retain(|p| *p != principal);
    controllers_update(canister_id, current_controllers).await
  } else {
    Ok(())
  }
}

pub async fn controller_set(
  canister_id: Principal,
  principals: Vec<Principal>,
) -> Result<(), EgoError> {
  controllers_update(canister_id, principals).await
}
