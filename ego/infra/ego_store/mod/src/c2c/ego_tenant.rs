use ic_cdk::api;
use ic_types::Principal;
use async_trait::async_trait;
use ego_tenant_mod::types::{WalletMainAddRequest, WalletMainAddResponse};
use ego_types::ego_error::EgoError;

#[async_trait]
pub trait TEgoTenant {
  async fn wallet_main_add(canister_id: Principal, wallet_id: Principal) -> Result<bool, EgoError>;
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
  async fn wallet_main_add(canister_id: Principal, wallet_id: Principal) -> Result<bool, EgoError>{
    let req = WalletMainAddRequest{wallet_id};

    let call_result = api::call::call(
      canister_id,
      "wallet_main_add",
      (req,),
    )
      .await as Result<(Result<WalletMainAddResponse, EgoError>,), _>;

    match call_result.unwrap().0 {
      Ok(resp) => {
        Ok(resp.ret)
      },
      Err(e) => {
        Err(e)
      }
    }
  }
}