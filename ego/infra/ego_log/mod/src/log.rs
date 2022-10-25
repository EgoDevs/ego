use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_cdk::export::Principal;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Log {
  pub canister_id: Principal,
  pub log: String,
  pub created_at: u64,
}