use async_trait::async_trait;
use ego_types::app::EgoError;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;

use crate::c2c::c2c_types::{WalletCycleChargeRequest, WalletCycleChargeResponse};

#[async_trait]
pub trait TEgoStore {
  async fn wallet_cycle_charge(
    &self,
    canister_id: Principal,
    wallet_id: Principal,
    cycle: u128,
    comment: String,
  ) -> Result<bool, EgoError>;
}

pub struct EgoStore {}

impl EgoStore {
  pub fn new() -> Self {
    EgoStore {}
  }
}

#[async_trait]
impl TEgoStore for EgoStore {
  async fn wallet_cycle_charge(
    &self,
    canister_id: Principal,
    wallet_id: Principal,
    cycle: u128,
    comment: String,
  ) -> Result<bool, EgoError> {
    let req = WalletCycleChargeRequest {
      wallet_id,
      cycle,
      comment,
    };

    let call_result = api::call::call(canister_id, "wallet_cycle_charge", (req, )).await
      as Result<(Result<WalletCycleChargeResponse, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(resp) => Ok(resp.ret),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        Err(EgoError { code, msg })
      }
    }
  }
}
