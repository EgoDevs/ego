use std::borrow::Cow;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::time;
use ic_stable_structures::{BoundedStorable, Storable};
use ego_types::app::{App, Canister, Version};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_stable_structures::storable::Blob;
use serde::Serialize;
use crate::memory::USER_APPS;
use crate::types::ego_store_app::EgoStoreApp;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserApp {
  pub app: App,
  pub canister: Canister,
  pub latest_version: Version,
  pub wallet_id: Option<Principal>,
  pub last_update: u64 // second
}

impl UserApp {
  pub fn new(app: &App, canister: Canister, wallet_id: Option<Principal>) -> Self {
    UserApp {
      app: app.clone(),
      latest_version: app.current_version.clone(),
      canister,
      wallet_id,
      last_update: 0,
    }
  }

  pub fn by_last_update(last_update: u64) -> Vec<UserApp> {
    USER_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, user_app)| user_app.last_update > last_update)
        .map(|(_, mut user_app)| {
          let ego_store_app = EgoStoreApp::get(&user_app.app.app_id).expect("ego store app not exists");
          user_app.latest_version = ego_store_app.app.current_version;

          user_app
        }).collect()
    })
  }

  pub fn by_wallet_id(wallet_id: &Principal) -> Vec<UserApp> {
    USER_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, user_app)| user_app.wallet_id.is_some() && user_app.wallet_id.unwrap() == *wallet_id)
        .map(|(_, mut user_app)| {
          let ego_store_app = EgoStoreApp::get(&user_app.app.app_id).expect("ego store app not exists");
          user_app.latest_version = ego_store_app.app.current_version;

          user_app
        }).collect()
    })
  }

  pub fn get(canister_id: &Principal) -> Option<UserApp> {
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
      self.last_update = time() / 1000000000;
      inst.insert(key, self.clone());
    });
  }

  pub fn remove(&self) {
    USER_APPS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.canister.canister_id.as_slice()).unwrap();
      inst.remove(&key);
    });
  }

  pub fn into_ego_user_app(&self) -> ego_types::app::UserApp {
    ego_types::app::UserApp{
      app: self.app.clone(),
      canister: self.canister.clone(),
      latest_version: self.latest_version,
      wallet_id: self.wallet_id,
    }
  }
}

impl Storable for UserApp {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self  {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for UserApp {
  const MAX_SIZE: u32 = 512;
  const IS_FIXED_SIZE: bool = false;
}
