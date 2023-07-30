use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::app::AppId;

// for ego_store v2 api
#[derive(CandidType, Deserialize, Serialize)]
pub struct AppInstallRequest {
  pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppReInstallRequest {
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppUpgradeRequest {
  pub wallet_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletUpgradeAppRequest {
  pub canister_id: Principal,
}