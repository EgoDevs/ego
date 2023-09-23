use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use ego_types::app::AppId;
use ego_utils::util::time;

use crate::memory::DEVELOPERS;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Developer {
  pub developer_id: Principal,
  pub name: String,
  pub is_app_auditor: bool,
  pub is_manager: bool,
  pub created_apps: Vec<AppId>,
  pub last_update: u64,    // second
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

  pub fn len() -> u64 {
    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn list(start: usize, end: usize) -> Vec<Developer> {
    Self::iter(start, end, |(_, developer)| Some(developer))
  }

  pub fn by_last_update(start: usize, end: usize, last_update: u64) -> Vec<Developer> {
    Self::iter(start, end, |(_, developer)| match developer.last_update >= last_update {
      true => { Some(developer) }
      false => { None }
    })
  }

  pub fn get(developer_id: &Principal) -> Option<Developer> {
    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      let key = Blob::try_from(developer_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn list_by_name(name: &str) -> Vec<Developer> {
    Self::iter(0, Self::len() as usize, |(_, developer)| match developer.name == *name {
      true => { Some(developer) }
      false => { None }
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

  fn iter<F>(start: usize, end: usize, filter: F) -> Vec<Self>
    where F: Fn((Blob<29>, Self)) -> Option<Self> {
    let mut idx = 0;

    DEVELOPERS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter_map(|entry| {
        if idx >= end {
          // 如果过了上界，直接忽略
          None
        } else {
          match filter(entry) {
            None => {
              None
            }
            Some(record) => {
              let ret = if idx >= start && idx < end {
                Some(record)
              } else {
                None
              };
              idx += 1;
              ret
            }
          }
        }
      }).collect()
    })
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

