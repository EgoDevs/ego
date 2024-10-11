use std::borrow::Cow;
use std::cmp::Ordering;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use crate::memory::FILES;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct File {
  pub wasm_count: u16,
  pub canister_id: Principal,
}

impl Eq for File {}

impl PartialEq<Self> for File {
  fn eq(&self, other: &Self) -> bool {
    self.canister_id == other.canister_id
  }
}

impl PartialOrd<Self> for File {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.wasm_count.cmp(&other.wasm_count))
  }
}

impl Ord for File {
  fn cmp(&self, other: &Self) -> Ordering {
    self.wasm_count.cmp(&other.wasm_count)
  }
}

impl File {
  pub fn new(canister_id: &Principal) -> Self {
    File {
      canister_id: canister_id.clone(),
      wasm_count: 0,
    }
  }

  pub fn len() -> u64 {
    FILES.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn list(start: usize, end: usize) -> Vec<File> {
    Self::iter(start, end, |(_, file)| Some(file))
  }

  pub fn get(canister_id: &Principal) -> Option<File> {
    FILES.with(|cell| {
      let inst = cell.borrow_mut();
      let key = Blob::try_from(canister_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn save(&self) {
    FILES.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.canister_id.as_slice()).unwrap();
      inst.insert(key, self.clone());
    });
  }

  fn iter<F>(start: usize, end: usize, filter: F) -> Vec<Self>
  where
    F: Fn((Blob<29>, Self)) -> Option<Self>,
  {
    FILES.with(|cell| {
      let inst = cell.borrow();
      inst.iter().skip(start).take(end - start).filter_map(|entry| {
        filter(entry)
      }).collect()
    })
  }
}

impl Storable for File {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for File {
  const MAX_SIZE: u32 = 64;
  const IS_FIXED_SIZE: bool = false;
}