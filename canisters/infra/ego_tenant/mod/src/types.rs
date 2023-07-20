use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;
use ic_stable_structures::{BoundedStorable, Storable};

use ego_types::app::EgoError;
use ego_types::app::Wasm;
use ego_types::cycle_info::CycleInfo;
use ego_types::registry::Registry;
use ego_types::user::User;


const STATE_SIZE:u32 = 4 * 1024 * 1024; // 4M

// stable state
#[derive(CandidType, Deserialize, Serialize)]
pub struct StableState {
    pub users: Option<User>,
    pub registry: Option<Registry>,
    pub cycle_info: Option<CycleInfo>,
}

impl Default for StableState {
    fn default() -> Self {
        StableState {
            users: None,
            registry: None,
            cycle_info: None,
        }
    }
}

impl Storable for StableState {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self  {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for StableState {
    const MAX_SIZE: u32 = STATE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

// Task
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
    pub wallet_id: Principal,
    pub canister_id: Principal,
    pub next_check_time: u64, // second
    pub last_cycle: Option<u128>
}

impl Task {
    pub fn new(wallet_id: Principal, canister_id: Principal, next_check_time: u64, last_cycle: Option<u128>) -> Self {
        Task {
            wallet_id,
            canister_id,
            next_check_time,
            last_cycle
        }
    }
}

impl Storable for Task {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self  {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Task {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}


#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoTenantErr {
    WalletExists,
    WalletNotExists,
    AppNotInstalled,
    CanisterNotFounded,
    CycleNotEnough,
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
            EgoTenantErr::CycleNotEnough => EgoError::new(4004, "ego-tenant: cycle not enough"),
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
pub struct AppMainUpgradeRequest {
    pub canister_id: Principal,
    pub wasm: Wasm,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainReInstallRequest {
    pub canister_id: Principal,
    pub wasm: Wasm,
}
