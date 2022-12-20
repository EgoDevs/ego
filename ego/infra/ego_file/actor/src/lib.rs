use std::collections::BTreeMap;

use candid::{candid_method, Decode, Encode};
use ic_cdk::{caller, trap};
use ic_cdk::api::stable::{stable64_grow, stable64_read, stable64_write};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;

use ego_file_mod::ego_macros::inject_ego_macros;
use ego_file_mod::service::{canister_add, canister_list, ego_log, EgoFileService, is_owner, log_list_after, LogEntry, owner_add, Registry, registry_post_upgrade, registry_pre_upgrade, User, USER, user_add, users_post_upgrade, users_pre_upgrade};
use ego_file_mod::state::STORAGE;
use ego_file_mod::storage::{DEFAULT_FILE_SIZE, HEADER_SIZE, Storage, WASM_PAGE_SIZE};
use ego_file_mod::types::{
  EgoFileError, FileMainReadRequest, FileMainReadResponse, FileMainWriteRequest,
  FileMainWriteResponse,
};
use ego_types::ego_error::EgoError;

inject_ego_macros!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
  init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
  let caller = arg.init_caller.unwrap_or(caller());

  ego_log("==> create stable page for state");
  let pages_to_grow = HEADER_SIZE / WASM_PAGE_SIZE;
  let result = stable64_grow(pages_to_grow);
  if result.is_err() {
    trap(&format!(
      "failed to grow stable memory by {} pages",
      pages_to_grow
    ))
  }

  ego_log("==> add caller as the owner");
  owner_add(caller.clone());
}

#[pre_upgrade]
fn pre_upgrade() {
  ego_log("ego-file: pre_upgrade");
  match state_persist() {
    Ok(_) => {}
    Err(_) => {}
  }
}

#[post_upgrade]
fn post_upgrade() {
  ego_log("ego-file: post_upgrade");
  match state_restore() {
    Ok(_) => {}
    Err(_) => {}
  }
}


/********************  file method ********************/
#[update(name = "file_main_write", guard = "user_guard")]
#[candid_method(update, rename = "file_main_write")]
fn file_main_write(req: FileMainWriteRequest) -> Result<FileMainWriteResponse, EgoError> {
  ego_log("ego-file: file_main_write");

  let ret = EgoFileService::file_main_write(&req.fid, &req.hash, req.data)?;
  Ok(FileMainWriteResponse { ret })
}

#[query(name = "file_main_read", guard = "user_guard")]
#[candid_method(query, rename = "file_main_read")]
fn file_main_read(req: FileMainReadRequest) -> Result<FileMainReadResponse, EgoError> {
  ego_log("ego-file: file_main_read");

  let data = EgoFileService::file_main_read(&req.fid)?;
  Ok(FileMainReadResponse { data })
}

#[derive(CandidType, Deserialize)]
struct PersistState {
  pub storage: Storage,
  pub user: User,
  pub registry: Registry,
}

/********************  persist method ********************/
#[update(name = "state_persist")]
#[candid_method(update, rename = "state_persist")]
fn state_persist() -> Result<bool, EgoError> {
  ego_log("ego-file: state_persist");

  let storage = STORAGE.with(|s| s.borrow().clone());
  let user = users_pre_upgrade();
  let registry = registry_pre_upgrade();

  let state = PersistState { storage, user, registry };

  let data = Encode!(&state).unwrap();

  ego_log(&format!("==> data length is: {}", data.len()));

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
  ego_log("ego-file: state_restore");

  // read file
  let mut buf = vec![0; DEFAULT_FILE_SIZE as usize];
  stable64_read(0, &mut buf); // file length
  let len = u64::from_le_bytes(buf[0..8].try_into().unwrap()) as usize;

  ego_log(&format!("==> data length is: {}", len));

  let data = &buf[8..8 + len];
  let state = Decode!(data, PersistState).unwrap();

  STORAGE.with(|s| *s.borrow_mut() = state.storage);
  users_post_upgrade(state.user);
  registry_post_upgrade(state.registry);

  Ok(true)
}
