use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;
use tracing::error;

use ego_types::app::{App, AppId, Version};
use ego_types::app_info::AppInfo;
use ego_types::cycle_info::{CycleInfo, CycleRecord};

// facility to call canister method which is created by the inject_ego_macros
#[async_trait]
pub trait TEgoCanister {
  fn ego_owner_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
  fn ego_owner_add(&self, target_canister_id: Principal, principal: Principal);
  fn ego_owner_remove(&self, target_canister_id: Principal, principal: Principal);

  fn ego_user_set(&self, target_canister_id: Principal, user_ids: Vec<Principal>);
  fn ego_user_add(&self, target_canister_id: Principal, principal: Principal);
  fn ego_user_remove(&self, target_canister_id: Principal, principal: Principal);

  fn ego_op_add(&self, target_canister_id: Principal, user_id: Principal);

  fn ego_canister_add(&self, target_canister_id: Principal, name: String, principal: Principal);

  fn ego_controller_set(&self, target_canister_id: Principal, principals: Vec<Principal>);
  async fn ego_controller_add(&self, target_canister_id: Principal, principal: Principal) -> Result<(), String>;
  fn ego_controller_remove(&self, target_canister_id: Principal, principal: Principal);

  async fn balance_get(&self, target_canister_id: Principal) -> Result<u128, String>;

  // app info
  fn ego_app_info_update(&self, target_canister_id: Principal, wallet_id: Option<Principal>, app_id: AppId, version: Version);
  async fn ego_app_info_get(&self, target_canister_id: Principal) -> Result<AppInfo, String>;
  async fn ego_app_version_check(&self, target_canister_id: Principal) -> Result<App, String>;

  // canister relative
  fn ego_canister_upgrade(&self, target_canister_id: Principal);
  fn ego_canister_remove(&self, target_canister_id: Principal);

  // canister cycle info
  fn ego_cycle_check(&self, target_canister_id: Principal);
  async fn ego_cycle_history(&self, target_canister_id: Principal) -> Result<Vec<CycleRecord>, String>;
  async fn ego_cycle_info(&self, target_canister_id: Principal) -> Result<CycleInfo, String>;
  fn ego_cycle_estimate_set(&self, target_canister_id: Principal, estimate: u64);
}

pub struct EgoCanister {}

impl EgoCanister {
  pub fn new() -> Self {
    EgoCanister {}
  }
}

#[async_trait]
impl TEgoCanister for EgoCanister {
  fn ego_owner_set(&self, target_canister_id: Principal, principals: Vec<Principal>) {
    let _result = api::call::notify(target_canister_id, "ego_owner_set", (principals, ));
  }

  fn ego_owner_add(&self, target_canister_id: Principal, principal: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_owner_add", (principal, ));
  }

  fn ego_owner_remove(&self, target_canister_id: Principal, principal: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_owner_remove", (principal, ));
  }

  fn ego_user_set(&self, target_canister_id: Principal, user_ids: Vec<Principal>) {
    let _result = api::call::notify(target_canister_id, "role_user_set", (user_ids, ));
  }

  fn ego_user_add(&self, target_canister_id: Principal, principal: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_user_add", (principal, ));
  }

  fn ego_user_remove(&self, target_canister_id: Principal, principal: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_user_remove", (principal, ));
  }

  fn ego_op_add(&self, target_canister_id: Principal, user_id: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_op_add", (user_id, ));
  }

  fn ego_canister_add(&self, target_canister_id: Principal, name: String, principal: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_canister_add", (name, principal, ));
  }

  fn ego_controller_set(&self, target_canister_id: Principal, principals: Vec<Principal>) {
    let _result = api::call::notify(target_canister_id, "ego_controller_set", (principals, ));
  }


  async fn ego_controller_add(&self, target_canister_id: Principal, principal: Principal) -> Result<(), String> {
    let call_result = api::call::call(target_canister_id, "ego_controller_add", (principal, )).await
      as Result<(Result<(), String>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(resp) => Ok(resp),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling ego_controller_add"
        );
        Err(msg)
      }
    }
  }

  fn ego_controller_remove(&self, target_canister_id: Principal, principal: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_controller_remove", (principal, ));
  }

  async fn balance_get(&self, target_canister_id: Principal) -> Result<u128, String> {
    let call_result = api::call::call(target_canister_id, "balance_get", ()).await
      as Result<(Result<u128, String>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(balance) => Ok(balance),
        Err(msg) => Err(msg),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling balance_get"
        );
        Err(msg)
      }
    }
  }

  fn ego_app_info_update(&self, target_canister_id: Principal, wallet_id: Option<Principal>, app_id: AppId, version: Version) {
    let _result = api::call::notify(target_canister_id, "ego_app_info_update", (wallet_id, app_id, version, ));

    // let call_result = api::call::call(target_canister_id, "ego_app_info_update", (wallet_id, app_id, version, )).await
    //   as Result<(Result<(), String>, ), (RejectionCode, String)>;
    //
    // match call_result {
    //   Ok(resp) => {
    //     match resp.0 {
    //       Ok(_) => Ok(()),
    //       Err(msg) => Err(msg)
    //     }
    //   }
    //   Err((_code, msg)) => {
    //     Err(msg)
    //   }
    // }
  }

  async fn ego_app_info_get(&self, target_canister_id: Principal) -> Result<AppInfo, String> {
    let call_result = api::call::call(target_canister_id, "ego_app_info_get", ()).await
      as Result<(Result<AppInfo, String>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => {
        match resp.0 {
          Ok(app_info) => Ok(app_info),
          Err(msg) => Err(msg)
        }
      }
      Err((_code, msg)) => {
        Err(msg)
      }
    }
  }

  async fn ego_app_version_check(&self, target_canister_id: Principal) -> Result<App, String> {
    let call_result = api::call::call(target_canister_id, "ego_app_version_check", ()).await
      as Result<(Result<App, String>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => {
        match resp.0 {
          Ok(app) => Ok(app),
          Err(msg) => Err(msg)
        }
      }
      Err((_code, msg)) => {
        Err(msg)
      }
    }
  }

  fn ego_canister_upgrade(&self, target_canister_id: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_canister_upgrade", ());
  }

  fn ego_canister_remove(&self, target_canister_id: Principal) {
    let _result = api::call::notify(target_canister_id, "ego_canister_remove", ());
  }

  // canister cycle info
  fn ego_cycle_check(&self, target_canister_id: Principal){
    let _result = api::call::notify(target_canister_id, "ego_cycle_check", ());
  }

  async fn ego_cycle_history(&self, target_canister_id: Principal) -> Result<Vec<CycleRecord>, String>{
    let call_result = api::call::call(target_canister_id, "ego_cycle_history", ()).await
      as Result<(Result<Vec<CycleRecord>, String>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => {
        match resp.0 {
          Ok(records) => Ok(records),
          Err(msg) => Err(msg)
        }
      }
      Err((_code, msg)) => {
        Err(msg)
      }
    }
  }

  async fn ego_cycle_info(&self, target_canister_id: Principal) -> Result<CycleInfo, String>{
    let call_result = api::call::call(target_canister_id, "ego_cycle_info", ()).await
      as Result<(Result<CycleInfo, String>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => {
        match resp.0 {
          Ok(cycle_info) => Ok(cycle_info),
          Err(msg) => Err(msg)
        }
      }
      Err((_code, msg)) => {
        Err(msg)
      }
    }
  }

  fn ego_cycle_estimate_set(&self, target_canister_id: Principal, estimate: u64){
    let _result = api::call::notify(target_canister_id, "ego_cycle_estimate_set", (estimate, ));
  }
}