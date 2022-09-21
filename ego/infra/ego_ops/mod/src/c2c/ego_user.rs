use ic_cdk::api;
use ic_types::Principal;

use async_trait::async_trait;
use ego_types::app::FileId;
use ego_types::ego_error::EgoError;
use crate::c2c::c2c_types::{FileMainWriteRequest};

#[async_trait]
pub trait TEgoUser {
  async fn role_user_add(&self, canister_id: Principal, principal: Principal) -> Result<bool, EgoError>;
}

pub struct EgoUser {
}

impl EgoUser{
  pub fn new() -> Self {
    EgoUser{}
  }
}

#[async_trait]
impl TEgoUser for EgoUser {
  async fn role_user_add(&self, canister_id: Principal, principal: Principal) -> Result<bool, EgoError>{
    let notify_result = api::call::notify(
      canister_id,
      "role_user_add",
      (principal,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "role_user_add failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}