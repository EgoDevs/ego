use candid::Principal;
use serde::{Deserialize, Serialize};
use candid::{CandidType};

#[derive(Serialize, Deserialize)]
pub struct RechargeCycleRecord {
    // pub user_name: String,
    pub cycles: u128,
    pub canister_id: Principal, //充值的canister
    pub wallet_id: Principal, //支付的principalId
    pub response: GeneralEnumResponse,
}

#[derive(Serialize, Deserialize)]
pub struct SnapshotCycleBalanceRecord {
    pub cycles: u128,
    pub ts: u64,  // timestamp in seconds
    pub canister_id: Principal,
    pub wallet_id: Principal, //用户的principalId
    pub response: GeneralEnumResponse,
}

#[derive(Serialize, Deserialize)]
pub struct ConsumeCycleRecord {
    pub canister_id: Principal,
    pub cycles: u128, //余额
    pub response: GeneralEnumResponse,
}

#[derive(Serialize, Deserialize)]
pub struct AppOperationRecord {
    pub app_name: String,
    pub canister_id: Principal,
    pub wallet_id: Principal,
    pub action: AppOperationAction,
    pub response: GeneralEnumResponse,
}

#[derive(Serialize, Deserialize)]
pub struct MethodCallRecord {
    pub canister_id: Principal,
    pub response: GeneralEnumResponse,
}


#[derive(Clone, Debug, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub enum GeneralEnumResponse {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failed")]
    Failed,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize, Eq, PartialEq)]
pub enum AppOperationAction {
    #[serde(rename = "install")]
    Install,
    #[serde(rename = "upgrade")]
    Upgrade,
    #[serde(rename = "uninstall")]
    Uninstall,

}

