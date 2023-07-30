use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use ego_types::app::{AppId, Category};
use ego_types::app::Version;

// type for ego_dev
#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminAppCreateRequest {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub version: Version,
  pub backend_data: Vec<u8>,
  pub backend_data_hash: String,
}

// type for ego_store
#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletCycleRechargeRequest {
  pub wallet_id: Principal,
  pub cycle: u128,
  pub comment: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletProviderAddRequest {
  pub wallet_provider: Principal,
  pub wallet_app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletMainRegisterRequest {
  pub user_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderNewRequest {
  pub amount: f32,
}

// type for ego_tenant
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
