/********************  app version  ********************/
use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use ego_types::app::{AppId, Version, Wasm};
use ego_types::app::CanisterType::{ASSET, BACKEND};
use ego_utils::util::time;

use crate::memory::APP_VERSIONS;
use crate::state::SEQ;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppVersion {
  pub id: u64,
  pub app_id: AppId,
  pub version: Version,
  pub status: AppVersionStatus,
  pub file_id: Principal,
  pub wasm: Option<Wasm>,
  pub last_update: u64,    // second
}

#[derive(
CandidType, Serialize, Deserialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq,
)]
pub enum AppVersionStatus {
  NEW,
  SUBMITTED,
  REJECTED,
  APPROVED,
  RELEASED,
  REVOKED,
}

impl PartialEq for AppVersion {
  fn eq(&self, other: &Self) -> bool {
    self.version == other.version
  }
}

impl AppVersion {
  pub fn new(app_id: &AppId, ego_file_canister_id: &Principal, version: &Version) -> Self {
    let next_id = SEQ.with(|cell| cell.borrow_mut().next_number("app_version", 0));
    AppVersion {
      id: next_id,
      app_id: app_id.clone(),
      version: version.clone(),
      status: AppVersionStatus::NEW,
      file_id: ego_file_canister_id.clone(),
      wasm: None,
      last_update: 0,
    }
  }

  pub fn frontend_update(&mut self, frontend_id: &Principal) {
    if self.wasm.is_none() {
      self.wasm = Some(Wasm::new(
        self.app_id.clone(),
        self.version,
        ASSET,
        frontend_id.clone(),
      ));
      self.save();
    }
  }

  pub fn backend_update(&mut self) {
    if self.wasm.is_none() {
      self.wasm = Some(Wasm::new(
        self.app_id.clone(),
        self.version,
        BACKEND,
        self.file_id,
      ));
      self.save();
    }
  }

  pub fn len() -> u64 {
    APP_VERSIONS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn list() -> Vec<AppVersion> {
    Self::iter(|(_, app_version)| Some(app_version))
  }

  pub fn by_app_id(app_id: &AppId) -> Vec<AppVersion> {
    Self::iter(|(_, app_version)| match app_version.app_id == *app_id {
      true => {
        Some(app_version)
      }
      false => { None }
    })
  }

  pub fn by_last_update(last_update: u64) -> Vec<AppVersion> {
    Self::iter(|(_, app_version)| match app_version.last_update >= last_update {
      true => {
        Some(app_version)
      }
      false => { None }
    })
  }

  pub fn get(id: &u64) -> Option<AppVersion> {
    APP_VERSIONS.with(|cell| {
      let inst = cell.borrow_mut();
      inst.get(id)
    })
  }

  pub fn get_by_app_id_and_version(app_id: &AppId, version: &Version) -> Option<AppVersion> {
    Self::by_app_id(app_id).iter().find(|app_version| {
      app_version.version == *version
    }).cloned()
  }

  pub fn save(&mut self) {
    APP_VERSIONS.with(|cell| {
      let mut inst = cell.borrow_mut();
      self.last_update = time();
      inst.insert(self.id, self.clone());
    });
  }

  fn iter<F>(filter: F) -> Vec<Self>
    where F: FnMut((u64, Self)) -> Option<Self> {
    APP_VERSIONS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter_map(filter).collect()
    })
  }
}

impl Storable for AppVersion {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for AppVersion {
  const MAX_SIZE: u32 = 2048;
  const IS_FIXED_SIZE: bool = false;
}