use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::Wasm;
use ego_types::app::EgoError;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoTenantErr {
  WalletExists,
  WalletNotExists,
  AppNotInstalled,
  CanisterNotFounded,
  SystemError(String),
}

impl From<EgoTenantErr> for EgoError {
  fn from(e: EgoTenantErr) -> Self {
    match e {
      EgoTenantErr::WalletExists => EgoError::new(4001, "ego-tenant: wallet exists"),
      EgoTenantErr::WalletNotExists => EgoError::new(4002, "ego-tenant: wallet not exists"),
      EgoTenantErr::AppNotInstalled => {
        EgoError::new(4003, "ego-tenant: you have not install this app")
      }
      EgoTenantErr::CanisterNotFounded => {
        EgoError::new(4004, "ego-tenant: can not find canister to installed")
      }
      EgoTenantErr::SystemError(msg) => msg.into(),
    }
  }
}

impl From<std::string::String> for EgoTenantErr {
  fn from(msg: String) -> Self {
    EgoTenantErr::SystemError(msg)
  }
}

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
