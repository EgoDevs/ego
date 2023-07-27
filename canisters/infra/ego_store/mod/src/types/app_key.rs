use std::borrow::Cow;
use std::cmp::Ordering;
use candid::{Decode, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_types::app::AppId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppKey {
  pub app_id: AppId
}

impl Eq for AppKey {}

impl PartialEq<Self> for AppKey {
  fn eq(&self, other: &Self) -> bool {
    self.app_id == other.app_id
  }
}

impl PartialOrd<Self> for AppKey {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.app_id.cmp(&other.app_id))
  }
}

impl Ord for AppKey {
  fn cmp(&self, other: &Self) -> Ordering {
    self.app_id.cmp(&other.app_id)
  }
}

impl AppKey {
  pub fn new(app_id: &AppId) -> Self {
    AppKey {
      app_id: app_id.clone(),
    }
  }
}

impl Storable for AppKey {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self  {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for AppKey {
  const MAX_SIZE: u32 = 64;
  const IS_FIXED_SIZE: bool = false;
}