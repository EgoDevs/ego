use std::collections::BTreeMap;

use candid::{CandidType, Deserialize, Principal};

use ego_types::app::{AppId, Category, Version};

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
    pub backend_data_hash: String,
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
