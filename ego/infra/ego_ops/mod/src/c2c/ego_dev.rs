use ic_cdk::api;
use ic_types::Principal;

use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use crate::c2c::c2c_types::{AdminFileAddRequest};

#[async_trait]
pub trait TEgoDev {
  async fn admin_file_add(&self, canister_id: Principal, ego_file_id: Principal) -> Result<bool, EgoError>;
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
  async fn admin_file_add(&self, canister_id: Principal, ego_file_id: Principal) -> Result<bool, EgoError>{
    let req = AdminFileAddRequest {
      canister_id: ego_file_id
    };

    let notify_result = api::call::notify(
      canister_id,
      "admin_file_add",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "admin_file_add failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}