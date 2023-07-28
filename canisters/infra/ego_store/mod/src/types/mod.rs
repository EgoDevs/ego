use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use ego_types::app::{AppId, CashFlow, EgoError, UserApp};

use crate::types::ego_store_app::EgoStoreApp;
use crate::types::order::Order;
use crate::types::stable_state::StableState;
use crate::types::tenant::Tenant;
use crate::types::wallet::Wallet;
use crate::types::wallet_provider::WalletProvider;

pub mod app_key;
pub mod cash_flow;
pub mod ego_store_app;
pub mod order;
pub mod stable_state;
pub mod tenant;
pub mod user_app;
pub mod wallet;
pub mod wallet_provider;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoStoreErr {
  AppExists,
  AppNotExists,
  NoTenant,
  UnAuthorized,
  OrderNotExists,
  SystemError(String),
  WalletNotExists,
  TenantExists,
  AppAlreadyInstall,
  AppNotInstall,
  WalletExists,
  WalletProviderExists,
  WalletProviderNotExists,
  CyclesNotEnouth,
}

impl From<EgoStoreErr> for EgoError {
  fn from(e: EgoStoreErr) -> Self {
    match e {
      EgoStoreErr::AppExists => EgoError::new(3001, "ego-store: app exists"),
      EgoStoreErr::AppNotExists => EgoError::new(3002, "ego-store: app not exists"),
      EgoStoreErr::NoTenant => EgoError::new(3003, "ego-store: no ego tenant installed"),
      EgoStoreErr::UnAuthorized => EgoError::new(3004, "ego-store: unauthorized"),
      EgoStoreErr::OrderNotExists => EgoError::new(3005, "ego-store: order not exists"),
      EgoStoreErr::WalletNotExists => EgoError::new(3006, "ego-store: wallet not exists"),
      EgoStoreErr::WalletExists => EgoError::new(3007, "ego-store: wallet exists"),
      EgoStoreErr::TenantExists => EgoError::new(3008, "ego-store: tenant exists"),
      EgoStoreErr::AppAlreadyInstall => {
        EgoError::new(3009, "ego-store: app already installed")
      }
      EgoStoreErr::AppNotInstall => EgoError::new(3010, "ego-store: app not install"),
      EgoStoreErr::WalletProviderExists => {
        EgoError::new(3011, "ego-store: wallet provider exists")
      }
      EgoStoreErr::WalletProviderNotExists => {
        EgoError::new(3012, "ego-store: wallet provider not exists")
      }
      EgoStoreErr::CyclesNotEnouth => EgoError::new(3003, "ego-store: cycles not enough"),
      EgoStoreErr::SystemError(msg) => msg.into(),
    }
  }
}

impl From<std::string::String> for EgoStoreErr {
  fn from(msg: String) -> Self {
    EgoStoreErr::SystemError(msg)
  }
}

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

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletMainNewRequest {
  pub user_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletProviderAddRequest {
  pub wallet_provider: Principal,
  pub wallet_app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletCycleRechargeRequest {
  pub wallet_id: Principal,
  pub cycle: u128,
  pub comment: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletImport {
  pub wallet_id: Principal,
  pub tenant_id: Principal,
  pub cycles: u128,
  pub user_id: Principal,
  pub user_apps: Vec<UserApp>,
  pub cash_flows: Vec<CashFlow>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletAppListRequest {
  pub wallet_id: Option<Principal>,
  pub ts: u64,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletCashFlowListRequest {
  pub wallet_id: Option<Principal>,
  pub ts: u64,
}

// for export
#[derive(CandidType, Deserialize, Serialize)]
pub struct DataExport {
  pub state: StableState,
  pub ego_store_apps: Vec<EgoStoreApp>,
  pub tenants: Vec<Tenant>,
  pub wallet_providers: Vec<WalletProvider>,
  pub wallets: Vec<Wallet>,
  pub user_apps: Vec<user_app::UserApp>,
  pub orders: Vec<Order>,
  pub cash_flows: Vec<cash_flow::CashFlow>,
}
