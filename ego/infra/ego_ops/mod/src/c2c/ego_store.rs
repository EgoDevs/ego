use ic_cdk::export::Principal;

use async_trait::async_trait;
use ic_cdk::api;
use ego_store_mod::types::AdminEgoTenantAddRequest;
use ego_types::ego_error::EgoError;

#[async_trait]
pub trait TEgoStore {
  async fn admin_ego_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>;
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
  async fn admin_ego_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>{
    let req = AdminEgoTenantAddRequest {
      tenant_id: ego_tenant_id
    };

    let notify_result = api::call::notify(
      canister_id,
      "admin_ego_tenant_add",
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