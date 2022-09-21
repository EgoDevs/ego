use ic_cdk::api;
use ic_types::Principal;

use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use crate::c2c::c2c_types::{AdminEgoFileAddRequest, AdminEgoStoreSetRequest, AdminEgoTenantAddRequest};

#[async_trait]
pub trait TEgoStore {
  async fn admin_egp_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>;
}

pub struct EgoStore {
}

impl EgoStore{
  pub fn new() -> Self {
    EgoStore{}
  }
}

#[async_trait]
impl TEgoStore for EgoStore {
  async fn admin_egp_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>{
    let req = AdminEgoTenantAddRequest {
      tenant_id: ego_tenant_id
    };

    let notify_result = api::call::notify(
      canister_id,
      "admin_egp_tenant_add",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "admin_egp_tenant_add failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}