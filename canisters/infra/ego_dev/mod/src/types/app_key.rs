use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use ego_types::app::AppId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppKey {
  pub app_id: AppId,
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

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for AppKey {
  const MAX_SIZE: u32 = 64;
  const IS_FIXED_SIZE: bool = false;
}