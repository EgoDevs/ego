use candid::CandidType;
use ic_cdk::export::candid::Deserialize;
use ic_cdk::export::Principal;
use ic_ledger_types::{AccountIdentifier, Memo, Tokens};
use serde::Serialize;

use ego_types::app::Wasm;

// type for ego_ledger
#[derive(CandidType, Deserialize, Serialize)]
pub struct LedgerPaymentAddRequest {
  pub from: AccountIdentifier,
  pub to: AccountIdentifier,
  pub amount: Tokens,
  pub memo: Memo,
}

// type for ego_tenant
#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainInstallRequest {
  pub wallet_id: Principal,
  pub user_id: Principal,
  pub wasm: Wasm,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainInstallResponse {
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainUpgradeRequest {
  pub canister_id: Principal,
  pub wasm: Wasm,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainUpgradeResponse {
  pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterMainTrackRequest {
  pub wallet_id: Principal,
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct CanisterMainUnTrackRequest {
  pub wallet_id: Principal,
  pub canister_id: Principal,
}
