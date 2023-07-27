use std::borrow::Cow;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use crate::memory::TASKS;

const MAX_TRY_COUNT:u8 = 5; // 4M

// Task
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
  pub wallet_id: Principal,
  pub canister_id: Principal,
  pub next_check_time: u64, // second
  pub last_cycle: Option<u128>,
  pub last_update: u64, // second
  pub try_count: u8
}

impl Task {
  pub fn new(wallet_id: &Principal, canister_id: &Principal, next_check_time: u64, last_cycle: Option<u128>) -> Self {
    Task {
      wallet_id: wallet_id.clone(),
      canister_id: canister_id.clone(),
      next_check_time,
      last_cycle,
      last_update: 0,
      try_count: 0
    }
  }

  pub fn by_last_update(last_update: u64)  -> Vec<Task> {
    TASKS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .filter(|(_, task)| task.last_update > last_update)
        .map(|(_, task)| {
          task
        }).collect()
    })
  }

  pub fn by_sentinel(sentinel: u64) -> Vec<Task> {
    TASKS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter(|(_, task)| task.next_check_time <= sentinel && task.try_count < MAX_TRY_COUNT)
        .map(|(_, task)| task.clone()).collect()
    })
  }

  pub fn list() -> Vec<Task> {
    TASKS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().map(|(_, task)| task).collect()
    })
  }

  pub fn get(canister_id: &Principal) -> Option<Task> {
    TASKS.with(|cell| {
      let inst = cell.borrow();
      let key = Blob::try_from(canister_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn save(&mut self) {
    TASKS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.canister_id.as_slice()).unwrap();
      self.last_update = time() / 1000000000;
      inst.insert(key, self.clone())
    });
  }

  pub fn remove(&self) {
    TASKS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.canister_id.as_slice()).unwrap();
      inst.remove(&key);
    });
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
