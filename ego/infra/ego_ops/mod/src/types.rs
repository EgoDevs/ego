use std::collections::BTreeMap;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ego_types::app::AppId;
use ego_types::version::Version;

#[derive(CandidType, Deserialize)]
pub struct CanisterMainListResponse {
  pub canisters: BTreeMap<AppId, Vec<Principal>>
}

#[derive(CandidType, Deserialize)]
pub struct CanisterMainRegisterRequest {
  pub app_id: String,
  pub canister_id: Principal
}

#[derive(CandidType, Deserialize)]
pub struct AdminAppCreateRequest {
  pub app_id: AppId,
  pub name: String,
  pub version: Version,
  pub backend_data: Vec<u8>,
  pub backend_hash: String,
  pub frontend: Option<Principal>,
}

#[derive(CandidType, Deserialize)]
pub struct AdminAppCreateResponse {
  pub ret: bool
}



#[derive(CandidType, Deserialize)]
pub struct AdminAppDeployRequest {
  pub app_id: AppId
}

#[derive(CandidType, Deserialize)]
pub struct AdminAppDeployResponse {
  pub ret: bool
}