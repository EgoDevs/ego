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
    Self {
      wallet_provider: wallet_provider.clone(),
      app_id: app_id.clone(),
    }
  }

  pub fn len() -> u64 {
    WALLET_PROVIDERS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn list(start: usize, end: usize) -> Vec<Self> {
    Self::iter(start, end, |(_, wallet_provider)| Some(wallet_provider))
  }

  pub fn get(wallet_provider: &Principal) -> Option<Self> {
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

  pub fn remove(wallet_provider: &Principal) {
    WALLET_PROVIDERS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(wallet_provider.as_slice()).unwrap();
      inst.remove(&key)
    });
  }

  pub fn iter<F>(start: usize, end: usize, filter: F) -> Vec<Self>
  where
    F: Fn((Blob<29>, Self)) -> Option<Self>,
  {
    WALLET_PROVIDERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().skip(start).take(end - start).filter_map(|entry| {
        filter(entry)
      }).collect()
    })
  }
}

impl Storable for WalletProvider {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for WalletProvider {
  const MAX_SIZE: u32 = 64;
  const IS_FIXED_SIZE: bool = false;
}