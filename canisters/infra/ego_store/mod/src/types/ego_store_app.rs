use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use ego_types::app::{App, AppId, Wasm};
use ego_utils::util::time;

use crate::memory::EGO_STORE_APPS;
use crate::types::app_key::AppKey;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoStoreApp {
  pub app: App,
  pub wasm: Wasm,
  pub last_update: u64, // mini second
}

impl EgoStoreApp {
  pub fn to_string(&self) -> String {
    format!(
      "app_id: {:?},category:{:?},current_version:{:?},",
      self.app.app_id, self.app.category, self.app.current_version
    )
  }
}

impl EgoStoreApp {
  pub fn new(app: &App, wasm: &Wasm) -> Self {
    EgoStoreApp { app: app.clone(), wasm: wasm.clone(), last_update: 0 }
  }

  pub fn list() -> Vec<EgoStoreApp> {
    EGO_STORE_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .map(|(_, ego_store_app)| {
          ego_store_app
        }).collect()
    })
  }

  pub fn by_last_update(last_update: u64) -> Vec<EgoStoreApp> {
    EGO_STORE_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, ego_store_app)| ego_store_app.last_update > last_update)
        .map(|(_, ego_store_app)| {
          ego_store_app
        }).collect()
    })
  }

  pub fn get(app_id: &AppId) -> Option<EgoStoreApp> {
    EGO_STORE_APPS.with(|cell| {
      let inst = cell.borrow_mut();
      inst.get(&AppKey::new(&app_id))
    })
  }

  pub fn save(&mut self) {
    EGO_STORE_APPS.with(|cell| {
      let mut inst = cell.borrow_mut();
      self.last_update = time();
      inst.insert(AppKey::new(&self.app.app_id), self.clone());
    });
  }
}

impl Storable for EgoStoreApp {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for EgoStoreApp {
  const MAX_SIZE: u32 = 512;
  const IS_FIXED_SIZE: bool = false;
}
