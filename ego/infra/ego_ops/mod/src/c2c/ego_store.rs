use ic_cdk::export::Principal;

use async_trait::async_trait;
use ic_cdk::api;
use ego_types::app::AppId;
use ego_types::ego_error::EgoError;
use crate::c2c::c2c_types::{AdminEgoTenantAddRequest, WalletAppInstallRequest};

#[async_trait]
pub trait TEgoStore {
  async fn admin_egp_tenant_add(&self, canister_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>;
  async fn wallet_main_new(&self, canister_id: Principal) -> Result<bool, EgoError>;
  async fn wallet_app_install(&self, canister_id: Principal, app_id: AppId) -> Result<bool, EgoError>;
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

  async fn wallet_main_new(&self, canister_id: Principal) -> Result<bool, EgoError>{
    let notify_result = api::call::notify(
      canister_id,
      "wallet_main_new",
      (),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "wallet_main_new failed".to_string() })
      },
      _ => Ok(true)
    }
  }

  async fn wallet_app_install(&self, canister_id: Principal, app_id: AppId) -> Result<bool, EgoError>{
    let req = WalletAppInstallRequest {
      app_id
    };

    let notify_result = api::call::notify(
      canister_id,
      "wallet_app_install",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "wallet_app_install failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}