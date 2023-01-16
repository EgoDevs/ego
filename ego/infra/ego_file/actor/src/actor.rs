use std::collections::BTreeMap;

use candid::{candid_method, Decode, Encode};
use ic_cdk::{caller, trap};
use ic_cdk::api::stable::{stable64_grow, stable64_read, stable64_write};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;

use ego_file_mod::service::EgoFileService;
use ego_file_mod::state::*;
use ego_file_mod::state::STORAGE;
use ego_file_mod::storage::{DEFAULT_FILE_SIZE, HEADER_SIZE, Storage, WASM_PAGE_SIZE};
use ego_file_mod::types::EgoFileError;
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_types::app::{EgoError, FileId};
use ego_types::registry::Registry;
use ego_types::user::User;

inject_ego_api!();
inject_cycle_info_api!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());

  log_add("==> create stable page for state");
  let pages_to_grow = HEADER_SIZE / WASM_PAGE_SIZE;
  let result = stable64_grow(pages_to_grow);
  if result.is_err() {
    trap(&format!(
      "failed to grow stable memory by {} pages",
      pages_to_grow
    ))
  }

  log_add("==> add caller as the owner");
  owner_add(caller.clone());
}

#[pre_upgrade]
fn pre_upgrade() {
  log_add("ego-file: pre_upgrade");
  match state_persist() {
    Ok(_) => {}
    Err(_) => {}
  }
}

#[post_upgrade]
fn post_upgrade() {
  log_add("ego-file: post_upgrade");
  match state_restore() {
    Ok(_) => {}
    Err(_) => {}
  }
}


/********************  file method ********************/
#[update(name = "file_main_write", guard = "user_guard")]
#[candid_method(update, rename = "file_main_write")]
fn file_main_write(fid: FileId, hash: String, data: Vec<u8>) -> Result<bool, EgoError> {
  log_add("ego-file: file_main_write");

  let ret = EgoFileService::file_main_write(&fid, &hash, data)?;
  Ok(ret)
}

#[query(name = "file_main_read", guard = "user_guard")]
#[candid_method(query, rename = "file_main_read")]
fn file_main_read(fid: FileId) -> Result<Vec<u8>, EgoError> {
  log_add("ego-file: file_main_read");

  let data = EgoFileService::file_main_read(&fid)?;
  Ok(data)
}

#[derive(CandidType, Deserialize)]
struct PersistState {
  pub storage: Storage,
  users: Option<User>,
  registry: Option<Registry>,
  cycle_info: Option<CycleInfo>
}

/********************  persist method ********************/
#[update(name = "state_persist")]
#[candid_method(update, rename = "state_persist")]
fn state_persist() -> Result<bool, EgoError> {
  log_add("ego-file: state_persist");

  let storage = STORAGE.with(|s| s.borrow().clone());

  let state = PersistState {
    storage,
    users: Some(users_pre_upgrade()),
    registry: Some(registry_pre_upgrade()),
    cycle_info: Some(cycle_info_pre_upgrade())
  };

  let data = Encode!(&state).unwrap();

  log_add(&format!("==> data length is: {}", data.len()));

  if data.len() > DEFAULT_FILE_SIZE as usize {
    Err(EgoFileError::UnknownError("state too large".to_string()).into())
  } else {
    stable64_write(0, &(data.len() as u64).to_le_bytes()); // file length
    stable64_write(8, &data);

    Ok(true)
  }
}

#[update(name = "state_restore")]
#[candid_method(update, rename = "state_restore")]
fn state_restore() -> Result<bool, EgoError> {
  log_add("ego-file: state_restore");

  // read file
  let mut buf = vec![0; DEFAULT_FILE_SIZE as usize];
  stable64_read(0, &mut buf); // file length
  let len = u64::from_le_bytes(buf[0..8].try_into().unwrap()) as usize;

  log_add(&format!("==> data length is: {}", len));

  let data = &buf[8..8 + len];
  let state = Decode!(data, PersistState).unwrap();

  STORAGE.with(|s| *s.borrow_mut() = state.storage);
  match state.users {
    None => {}
    Some(users) => {
      users_post_upgrade(users);
    }
  }

  match state.registry {
    None => {}
    Some(registry) => {
      registry_post_upgrade(registry);
    }
  }

  match state.cycle_info {
    None => {}
    Some(cycle_info) => {
      cycle_info_post_upgrade(cycle_info);
    }
  }

  Ok(true)
}
