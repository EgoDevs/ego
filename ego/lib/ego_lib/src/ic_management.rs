use ic_cdk::api;
use ic_cdk::api::management_canister::main::{CanisterIdRecord, CanisterSettings, CanisterStatusResponse, UpdateSettingsArgument};
use ic_cdk::export::Principal;
use tracing::error;

use ego_types::app::EgoError;

async fn controllers_update(canister_id: Principal, controllers: Vec<Principal>) -> Result<(), EgoError> {
  let in_arg = UpdateSettingsArgument {
    canister_id,
    settings: CanisterSettings {
      controllers: Some(controllers),
      compute_allocation: None,
      memory_allocation: None,
      freezing_threshold: None,
    },
  };

  let (_, ): ((), ) = match api::call::call(
    Principal::management_canister(),
    "update_settings",
    (in_arg, ),
  )
    .await
  {
    Ok(x) => x,
    Err((code, msg)) => {
      let code = code as u16;
      error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling controllers_update"
        );
      return Err(EgoError { code, msg });
    }
  };

  Ok(())
}

async fn canister_status_get(canister_id: Principal) -> Result<CanisterStatusResponse, EgoError> {
  let req = CanisterIdRecord {
    canister_id,
  };

  let (status_result, ): (CanisterStatusResponse, ) = match api::call::call(
    Principal::management_canister(),
    "canister_status",
    (req, ),
  ).await {
    Ok(x) => x,
    Err((code, msg)) => {
      let code = code as u16;
      error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling canister_status_get"
        );
      return Err(EgoError { code, msg });
    }
  };

  Ok(status_result)
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

pub async fn controller_remove(canister_id: Principal, principal: Principal) -> Result<(), EgoError> {
  let status_resp = canister_status_get(canister_id).await?;

  let mut current_controllers = status_resp.settings.controllers;

  if current_controllers.contains(&principal) {
    current_controllers.retain(|p| *p != principal);
    controllers_update(canister_id, current_controllers).await
  } else {
    Ok(())
  }
}

pub async fn controller_set(canister_id: Principal, principals: Vec<Principal>) -> Result<(), EgoError> {
  controllers_update(canister_id, principals).await
}