use std::borrow::Cow;

use candid::{Decode, Encode};
use candid::{CandidType, Deserialize};
use ego_backup::backup_info::BackupInfo;
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use ego_types::cycle_info::CycleInfo;
use ego_types::registry::Registry;
use ego_types::seq::Seq;
use ego_types::user::User;

const STATE_SIZE: u32 = 4 * 1024 * 1024; // 4M

#[derive(CandidType, Deserialize, Serialize)]
pub struct StableState {
  pub users: Option<User>,
  pub registry: Option<Registry>,
  pub cycle_info: Option<CycleInfo>,
  pub backup_info: Option<BackupInfo>,
  pub seq: Option<Seq>,
}

impl Default for StableState {
  fn default() -> Self {
    StableState {
      users: None,
      registry: None,
      cycle_info: None,
      backup_info: None,
      seq: None,
    }
  }
}

impl Storable for StableState {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for StableState {
  const MAX_SIZE: u32 = STATE_SIZE;
  const IS_FIXED_SIZE: bool = false;
}