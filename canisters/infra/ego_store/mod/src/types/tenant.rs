use std::borrow::Cow;
use std::cmp::Ordering;
use candid::{Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use candid::{CandidType, Deserialize, Principal};
use ic_stable_structures::storable::Blob;
use serde::Serialize;
use crate::memory::{TENANTS};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tenant {
  pub wallet_count: u16,
  pub canister_id: Principal,
}

impl Eq for Tenant {}

impl PartialEq<Self> for Tenant {
  fn eq(&self, other: &Self) -> bool {
    self.canister_id == other.canister_id
  }
}

impl PartialOrd<Self> for Tenant {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.wallet_count.cmp(&other.wallet_count))
  }
}

impl Ord for Tenant {
  fn cmp(&self, other: &Self) -> Ordering {
    self.wallet_count.cmp(&other.wallet_count)
  }
}

impl Tenant {
  pub fn new(tenant_id: &Principal) -> Self {
    Tenant {
      canister_id: tenant_id.clone(),
      wallet_count: 0,
    }
  }

  pub fn list() -> Vec<Tenant> {
    TENANTS.with(|cell| {
      let inst = cell.borrow();
      inst.iter()
        .map(|(_, tenant)| {
          tenant
        }).collect()
    })
  }

  pub fn get(canister_id: &Principal) -> Option<Tenant> {
    TENANTS.with(|cell| {
      let inst = cell.borrow_mut();
      let key = Blob::try_from(canister_id.as_slice()).unwrap();
      inst.get(&key)
    })
  }

  pub fn save(&self) {
    TENANTS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = Blob::try_from(self.canister_id.as_slice()).unwrap();
      inst.insert(key, self.clone());
    });
  }
}

impl Storable for Tenant {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self  {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for Tenant {
  const MAX_SIZE: u32 = 64;
  const IS_FIXED_SIZE: bool = false;
}