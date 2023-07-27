use std::borrow::Cow;
use candid::{Decode, Encode};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use ego_types::app::AppId;
use crate::memory::WALLET_PROVIDERS;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WalletProvider {
    pub wallet_provider: Principal,
    pub app_id: AppId,
}

impl WalletProvider {
    pub fn new(wallet_provider: &Principal, app_id: &AppId) -> Self {
        WalletProvider {
            wallet_provider: wallet_provider.clone(),
            app_id: app_id.clone(),
        }
    }

    pub fn list() -> Vec<WalletProvider> {
        WALLET_PROVIDERS.with(|cell| {
            let inst = cell.borrow_mut();
            inst.iter().map(|(_, wallet_provider)| wallet_provider).collect()
        })
    }

    pub fn get(wallet_provider: &Principal) -> Option<WalletProvider> {
        WALLET_PROVIDERS.with(|cell| {
            let inst = cell.borrow_mut();
            let key = Blob::try_from(wallet_provider.as_slice()).unwrap();
            inst.get(&key)
        })
    }

    pub fn save(&self) {
        WALLET_PROVIDERS.with(|cell| {
            let mut inst = cell.borrow_mut();
            let key = Blob::try_from(self.wallet_provider.as_slice()).unwrap();
            inst.insert(key, self.clone());
        });
    }

    pub fn remove(&self) {
        WALLET_PROVIDERS.with(|cell| {
            let mut inst = cell.borrow_mut();
            let key = Blob::try_from(self.wallet_provider.as_slice()).unwrap();
            inst.remove(&key)
        });
    }
}

impl Storable for WalletProvider {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self  {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for WalletProvider {
    const MAX_SIZE: u32 = 64;
    const IS_FIXED_SIZE: bool = false;
}