use ego_types::app::{AppId, Category, DeployMode};
use ego_types::version::Version;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use std::collections::BTreeMap;

#[derive(CandidType, Deserialize)]
pub struct CanisterMainListResponse {
    pub canisters: BTreeMap<AppId, Vec<Principal>>,
}

#[derive(CandidType, Deserialize)]
pub struct AdminAppCreateRequest {
    pub app_id: AppId,
    pub name: String,
    pub category: Category,
    pub logo: String,
    pub description: String,
    pub version: Version,
    pub backend_data: Vec<u8>,
    pub backend_hash: String,
    pub frontend: Option<Principal>,
    pub deploy_mode: DeployMode,
}


#[derive(CandidType, Deserialize)]
pub struct AdminAppDeployRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize)]
pub struct AdminAppDeployResponse {
    pub ret: bool,
}

#[derive(CandidType, Deserialize)]
pub struct AdminWalletProviderAddRequest {
    pub wallet_provider: Principal,
    pub wallet_app_id: AppId,
}

#[derive(CandidType, Deserialize)]
pub struct AdminWalletCycleRechargeRequest {
    pub wallet_id: Principal,
    pub cycle: u128,
    pub comment: String,
}
