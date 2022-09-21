use std::collections::BTreeMap;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use ego_types::app::AppId;
use ego_types::version::Version;

#[derive(CandidType, Deserialize)]
pub struct CanisterMainCreateRequest {
  pub app_id: AppId,
  pub version: Version,
  pub data: Vec<u8>,
  pub hash: String,
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMainCreateResponse {
  pub ret: bool
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMainListResponse {
  pub canisters: BTreeMap<AppId, Vec<Principal>>
}

#[derive(CandidType, Deserialize)]
pub struct AppMainCreateRequest {
  pub app_id: AppId,
  pub name: String,
  pub version: Version,
  pub data: Vec<u8>,
  pub hash: String,
}

#[derive(CandidType, Deserialize)]
pub struct AppMainCreateResponse {
  pub ret: bool
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMainRegisterRequest {
  pub ego_dev_id: Principal,
  pub ego_store_id: Principal,
  pub ego_file_id: Principal,
  pub ego_tenant_id: Principal
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMainRegisterResponse {
  pub ret: bool
}
