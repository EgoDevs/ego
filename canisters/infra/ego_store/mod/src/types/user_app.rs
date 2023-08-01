use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use ego_types::app::{App, Canister};
use ego_utils::util::time;

use crate::memory::USER_APPS;
use crate::types::ego_store_app::EgoStoreApp;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserApp {
  pub app: App,
  pub canister: Canister,
  pub wallet_id: Option<Principal>,
  pub last_update: u64, // second
}

impl UserApp {
  pub fn new(app: &App, canister: &Canister, wallet_id: Option<Principal>) -> Self {
    Self {
      app: app.clone(),
      canister: canister.clone(),
      wallet_id,
      last_update: 0,
    }
  }

  pub fn len() -> u64 {
    USER_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn list() -> Vec<Self> {
    Self::iter(|(_, user_app)| {
      Some(user_app)
    })
  }

  pub fn by_last_update(last_update: u64) -> Vec<Self> {
    Self::iter(|(_, user_app)| match user_app.last_update >= last_update {
      true => {
        Some(user_app)
      }
      false => {
        None
      }
    })
  }

  pub fn by_wallet_id(wallet_id: &Principal) -> Vec<Self> {
    Self::iter(|(_, user_app)| match user_app.wallet_id.is_some() && user_app.wallet_id.unwrap() == *wallet_id {
      true => {
        Some(user_app)
      }
      false => {
        None
      }
    })
  }

  pub fn by_wallet_id_and_id(wallet_id: &Principal, canister_id: &Principal) -> Option<Self> {
    match Self::get(canister_id) {
      None => { None }
      Some(user_app) => {
        match user_app.wallet_id.is_some() && user_app.wallet_id.unwrap() == *wallet_id {
          true => {
            Some(user_app)
          }
          false => {
            None
          }
        }
      }
    }
  }

  pub fn get(canister_id: &Principal) -> Option<Self> {
    USER_APPS.with(|cell| {
      let inst = cell.borrow_mut();
      let key = Blob::try_from(canister_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn save(&mut self) {
    USER_APPS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.canister.canister_id.as_slice()).unwrap();
      self.last_update = time();
      inst.insert(key, self.clone());
    });
  }

  pub fn remove(canister_id: &Principal) {
    USER_APPS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(canister_id.as_slice()).unwrap();
      inst.remove(&key);
    });
  }

  fn iter<F>(filter: F) -> Vec<Self>
    where F: FnMut((Blob<29>, Self)) -> Option<Self> {
    USER_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter_map(filter).collect()
    })
  }
}

impl Into<ego_types::app::UserApp> for UserApp {
  fn into(self) -> ego_types::app::UserApp {
    let ego_store_app = EgoStoreApp::get(&self.app.app_id).expect("ego store app not exists");

    ego_types::app::UserApp {
      app: self.app.clone(),
      canister: self.canister.clone(),
      latest_version: ego_store_app.app.current_version,
      wallet_id: self.wallet_id,
    }
  }
}

impl Storable for UserApp {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for UserApp {
  const MAX_SIZE: u32 = 512;
  const IS_FIXED_SIZE: bool = false;
}
