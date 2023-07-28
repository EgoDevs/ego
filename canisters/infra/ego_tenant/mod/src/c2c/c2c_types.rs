use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

// type for ego_store
#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCycleChargeRequest {
  pub canister_id: Principal,
  pub cycle: u128,
  pub comment: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCycleChargeResponse {
  pub ret: bool,
}
