use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use ego_types::app::AppId;
use ego_utils::util::time;

use crate::memory::DEVELOPERS;
use crate::types::ego_dev_app::EgoDevApp;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Developer {
  pub developer_id: Principal,
  pub name: String,
  pub is_app_auditor: bool,
  pub is_manager: bool,
  pub created_apps: Vec<AppId>,
  pub last_update: u64,    // mini second
}

impl Developer {
  pub fn new(developer_id: &Principal, name: &str) -> Self {
    Developer {
      developer_id: developer_id.clone(),
      name: name.to_owned(),
      is_app_auditor: false,
      is_manager: false,
      created_apps: vec![],
      last_update: 0,
    }
  }

  pub fn list() -> Vec<Developer> {
    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .map(|(_, developer)| {
          developer
        }).collect()
    })
  }

  pub fn by_last_update(last_update: u64) -> Vec<Developer> {
    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, developer)| developer.last_update > last_update)
        .map(|(_, developer)| {
          developer
        }).collect()
    })
  }

  pub fn developer_app_list(&self) -> Vec<EgoDevApp> {
    let created_apps = self
      .created_apps
      .iter()
      .filter_map(|app_id| EgoDevApp::get(app_id))
      .collect();
    created_apps
  }

  pub fn get(developer_id: &Principal) -> Option<Developer> {
    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      let key = Blob::try_from(developer_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn list_by_name(name: &str) -> Vec<Developer> {
    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter(|(_, developer)| developer.name == *name).map(|(_, developer)| developer).collect()
    })
  }

  pub fn save(&mut self) {
    DEVELOPERS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.developer_id.as_slice()).unwrap();
      self.last_update = time();
      inst.insert(key, self.clone());
    });
  }
}

impl Storable for Developer {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for Developer {
  const MAX_SIZE: u32 = 1024;
  const IS_FIXED_SIZE: bool = false;
}

