use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
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
