use ic_cdk::api;
use ic_cdk::export::Principal;

use async_trait::async_trait;
use ego_types::app::AppId;
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use crate::c2c::c2c_types::{AdminAppCreateRequest, AdminEgoFileAddRequest, AdminEgoStoreSetRequest};

#[async_trait]
pub trait TEgoDev {
  async fn admin_ego_file_add(&self, canister_id: Principal, ego_file_id: Principal) -> Result<bool, EgoError>;
  async fn admin_ego_store_set(&self, canister_id: Principal, ego_store_id: Principal) -> Result<bool, EgoError>;
  async fn admin_app_create(&self, canister_id: Principal, app_id: AppId, name: String, version: Version, backend_data: Vec<u8>, backend_data_hash: String, frontend: Option<Principal>) -> Result<bool, EgoError>;
}

pub struct EgoDev {
}

impl EgoDev{
  pub fn new() -> Self {
    EgoDev{}
  }
}

#[async_trait]
impl TEgoDev for EgoDev {
  async fn admin_ego_file_add(&self, canister_id: Principal, ego_file_id: Principal) -> Result<bool, EgoError>{
    let req = AdminEgoFileAddRequest {
      canister_id: ego_file_id
    };

    let notify_result = api::call::notify(
      canister_id,
      "admin_ego_file_add",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "admin_ego_file_add failed".to_string() })
      },
      _ => Ok(true)
    }
  }

  async fn admin_ego_store_set(&self, canister_id: Principal, ego_store_id: Principal) -> Result<bool, EgoError>{
    let req = AdminEgoStoreSetRequest {
      canister_id: ego_store_id
    };

    let notify_result = api::call::notify(
      canister_id,
      "admin_ego_store_set",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "admin_ego_store_set failed".to_string() })
      },
      _ => Ok(true)
    }
  }

  async fn admin_app_create(&self, canister_id: Principal, app_id: AppId, name: String, version: Version, backend_data: Vec<u8>, backend_data_hash: String, frontend: Option<Principal>) -> Result<bool, EgoError>{
    let req = AdminAppCreateRequest {
      app_id, name, version, logo: "".to_string(), description: "".to_string(), backend_data, backend_data_hash, frontend
    };

    let notify_result = api::call::notify(
      canister_id,
      "admin_app_create",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "admin_app_create failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}