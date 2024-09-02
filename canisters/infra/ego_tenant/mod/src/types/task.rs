use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_stable_structures::storable::Blob;
use serde::Serialize;

use ego_utils::util::time;

use crate::memory::TASKS;

pub const MAX_TRY_COUNT: u8 = 5; // 4M

// Task
#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Task {
  pub canister_id: Principal,
  pub next_check_time: u64,
  // second
  pub last_cycle: Option<u128>,
  pub last_update: u64,
  // second
  pub try_count: u8,
}

impl Task {
  pub fn new(canister_id: &Principal, next_check_time: u64, last_cycle: Option<u128>) -> Self {
    Task {
      canister_id: canister_id.clone(),
      next_check_time,
      last_cycle,
      last_update: 0,
      try_count: 0,
    }
  }

  pub fn len() -> u64 {
    TASKS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn by_last_update(start: usize, end: usize, last_update: u64) -> Vec<Task> {
    Self::iter(start, end, |(_, task)| {
      match task.last_update >= last_update {
        true => { Some(task) }
        false => { None }
      }
    })
  }

  pub fn by_sentinel(sentinel: u64) -> Vec<Task> {
    Self::iter(0, Self::len() as usize, |(_, task)|
    match task.next_check_time <= sentinel && task.try_count < MAX_TRY_COUNT {
      true => { Some(task) }
      false => { None }
    })
  }

  pub fn list(start: usize, end: usize) -> Vec<Task> {
    Self::iter(start, end, |(_, task)| Some(task))
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
      self.last_update = time();
      inst.insert(key, self.clone())
    });
  }

  pub fn remove(canister_id: &Principal) {
    TASKS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(canister_id.as_slice()).unwrap();
      inst.remove(&key);
    });
  }

  fn iter<F>(start: usize, end: usize, filter: F) -> Vec<Task>
  where
    F: Fn((Blob<29>, Task)) -> Option<Task>,
  {
    let mut idx = 0;

    TASKS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().skip(start).take(end - start).filter_map(|entry| {
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
      }).collect()
    })
  }
}

impl Storable for Task {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for Task {
  const MAX_SIZE: u32 = 512;
  const IS_FIXED_SIZE: bool = false;
}
