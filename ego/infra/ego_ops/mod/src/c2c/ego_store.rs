use ic_cdk::export::Principal;

use async_trait::async_trait;
use ic_cdk::api;
use ego_store_mod::types::{AdminEgoTenantAddRequest, AdminWalletProviderAddRequest, EgoStoreSetupRequest};
use ego_types::app::AppId;
use ego_types::ego_error::EgoError;

#[async_trait]
pub trait TEgoStore {
  async fn admin_ego_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>;
  async fn ego_store_setup(&self, ego_store_id: Principal, ego_dev_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>;
  async fn admin_wallet_provider_add(&self, ego_store_id: Principal, wallet_provider: Principal, wallet_app_id: AppId) -> Result<(), EgoError>;
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

  async fn ego_store_setup(&self, ego_store_id: Principal, ego_dev_id: Principal, ego_cron_id: Principal) -> Result<bool, EgoError>{
    let req = EgoStoreSetupRequest {
      ego_dev_id,
      ego_cron_id
    };

    let notify_result = api::call::notify(
      ego_store_id,
      "ego_store_setup",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "ego_store_setup failed".to_string() })
      },
      _ => Ok(true)
    }
  }

  async fn admin_wallet_provider_add(&self, ego_store_id: Principal, wallet_provider: Principal, wallet_app_id: AppId) -> Result<(), EgoError> {
    let req = AdminWalletProviderAddRequest {
      wallet_provider,
      wallet_app_id
    };

    let notify_result = api::call::notify(
      ego_store_id,
      "admin_wallet_provider_add",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "admin_wallet_provider_add failed".to_string() })
      },
      _ => Ok(())
    }
  }
}