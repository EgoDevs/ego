use ic_cdk::export::Principal;

use async_trait::async_trait;
use ic_cdk::api;
use ego_tenant_mod::types::EgoTenantSetupRequest;
use ego_types::ego_error::EgoError;

#[async_trait]
pub trait TEgoTenant {
  async fn ego_tenant_setup(&self, ego_tenant_id: Principal, ego_store_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>;
}

pub struct EgoTenant {
}

impl EgoTenant{
  pub fn new() -> Self {
    EgoTenant{}
  }
}

#[async_trait]
impl TEgoTenant for EgoTenant {
  async fn ego_tenant_setup(&self, ego_tenant_id: Principal, ego_store_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>{
    let req = EgoTenantSetupRequest {
      ego_store_id,
      ego_cron_id
    };

    let notify_result = api::call::notify(
      ego_tenant_id,
      "ego_tenant_setup",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "ego_tenant_setup failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}