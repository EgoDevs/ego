use std::collections::HashMap;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_types::Principal;
use ego_store_mod::app::App;
use ego_utils::types::EgoError;

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
        match e{
            EgoTenantErr::WalletExists => EgoError::new(4001, "ego-tenant: wallet exists"),
            EgoTenantErr::WalletNotExists=> EgoError::new(4002, "ego-tenant: wallet not exists"),
            EgoTenantErr::AppNotInstalled=> EgoError::new(4003, "ego-tenant: you have not install this app"),
            EgoTenantErr::CanisterNotFounded=> EgoError::new(4004, "ego-tenant: can not find canister to installed"),
            EgoTenantErr::SystemError(msg) => msg.into(),
        }
    }
}

impl From<std::string::String> for EgoTenantErr {
    fn from(msg: String) -> Self {
        EgoTenantErr::SystemError(msg)
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletMainAddRequest {
    pub wallet_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletMainAddResponse {
    pub ret: bool
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppInstallRequest {
    pub app: App,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppInstallResponse {
    pub canisters: HashMap<String, Principal>
}