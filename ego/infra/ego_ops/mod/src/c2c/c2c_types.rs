use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ego_types::app::{AppId};
use ego_types::version::Version;

// type for ego_dev
#[derive(CandidType, Deserialize)]
pub struct AdminEgoFileAddRequest {
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct AdminEgoStoreSetRequest {
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct AdminAppCreateRequest {
  pub app_id: AppId,
  pub name: String,
  pub logo: String,
  pub description: String,
  pub version: Version,
  pub backend_data: Vec<u8>,
  pub backend_data_hash: String,
  pub frontend: Option<Principal>
}

// type for ego_store
#[derive(CandidType, Deserialize)]
pub struct AdminEgoTenantAddRequest {
  pub tenant_id: Principal
}

#[derive(CandidType, Deserialize)]
pub struct WalletAppInstallRequest {
  pub app_id: AppId
}

// type for ego_tenant
#[derive(CandidType, Deserialize)]
pub enum CronInterval {
  PerSecond,
  PerMinute,
  PerHour,
  PerDay,
}

#[derive(CandidType, Deserialize)]
pub struct TaskMainAddRequest {
  pub canister_id: Principal,
  pub method: String,
  pub interval: CronInterval,
}