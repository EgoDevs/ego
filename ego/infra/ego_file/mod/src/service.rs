use astrox_macros::{inject_canister_log, inject_canister_registry, inject_canister_users};

use ego_types::app::FileId;
use ego_types::ego_error::EgoError;

use crate::state::STORAGE;

inject_canister_log!();
inject_canister_registry!();
inject_canister_users!();


/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  let _ = match name {
    "ego_dev" => user_add(canister_id),
    "ego_tenant" => user_add(canister_id),
    _ => {}
  };
}


pub struct EgoFileService {}

impl EgoFileService {
  pub fn file_main_write(fid: &FileId, hash: &str, data: Vec<u8>) -> Result<bool, EgoError> {
    STORAGE.with(|s| s.borrow_mut().file_write(fid, hash, data))
  }

  pub fn file_main_read(fid: &FileId) -> Result<Vec<u8>, EgoError> {
    STORAGE.with(|s| s.borrow().file_read(fid))
  }

  pub fn file_count() -> Result<u64, EgoError> {
    STORAGE.with(|s| Ok(s.borrow().file_count()))
  }
}
