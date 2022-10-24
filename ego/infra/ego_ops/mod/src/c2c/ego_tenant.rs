use ic_cdk::export::Principal;

use async_trait::async_trait;
use ic_cdk::api;
use ego_tenant_mod::types::{CanisterMainTrackRequest, CanisterMainUnTrackRequest, EgoTenantSetupRequest};
use ego_types::ego_error::EgoError;

#[async_trait]
pub trait TEgoTenant {
  async fn ego_tenant_setup(&self, ego_tenant_id: Principal, ego_store_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>;
  async fn canister_main_track(&self, ego_tenant_id: Principal, wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError>;
  async fn canister_main_untrack(&self, ego_tenant_id: Principal, wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError>;
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

  async fn canister_main_track(&self, ego_tenant_id: Principal, wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
    let req = CanisterMainTrackRequest { wallet_id, canister_id };

    let notify_result = api::call::notify(
      ego_tenant_id,
      "canister_main_track",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "canister_main_track failed".to_string() })
      },
      _ => Ok(())
    }
  }

  async fn canister_main_untrack(&self, ego_tenant_id: Principal, wallet_id: Principal, canister_id: Principal) -> Result<(), EgoError> {
    let req = CanisterMainUnTrackRequest { wallet_id, canister_id };

    let notify_result = api::call::notify(
      ego_tenant_id,
      "canister_main_untrack",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "canister_main_untrack failed".to_string() })
      },
      _ => Ok(())
    }
  }
}