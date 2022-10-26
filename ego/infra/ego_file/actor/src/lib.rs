use candid::{candid_method, Decode, Encode};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk_macros::*;

use ego_file_mod::service::EgoFileService;
use ego_file_mod::state::STORAGE;
use ego_file_mod::types::{
    EgoFileError, FileMainReadRequest, FileMainReadResponse, FileMainWriteRequest,
    FileMainWriteResponse,
};

use ego_file_mod::storage::{Storage, DEFAULT_FILE_SIZE, HEADER_SIZE, WASM_PAGE_SIZE};
use ego_macros::inject_balance_get;
use ego_types::ego_error::EgoError;
use ego_users::inject_ego_users;
use ego_registry::inject_ego_registry;
use ic_cdk::api::stable::{stable64_grow, stable64_read, stable64_write};

inject_balance_get!();
inject_ego_users!();
inject_ego_registry!();

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());

    ic_cdk::println!("==> create stable page for state");
    let pages_to_grow = HEADER_SIZE / WASM_PAGE_SIZE;
    let result = stable64_grow(pages_to_grow);
    if result.is_err() {
        trap(&format!(
            "failed to grow stable memory by {} pages",
            pages_to_grow
        ))
    }

    ic_cdk::println!("==> add caller as the owner");
    users_init(caller.clone());
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-file: pre_upgrade");
    match state_persist() {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-file: post_upgrade");
    match state_restore() {
        Ok(_) => {}
        Err(_) => {}
    }
}

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
    let _ = match name {
        "ego_dev" => role_user_add(canister_id).unwrap(),
        "ego_tenant" => role_user_add(canister_id).unwrap(),
        _ => {}
    };
}

/********************  file method ********************/
#[update(name = "file_main_write", guard = "user_guard")]
#[candid_method(update, rename = "file_main_write")]
fn file_main_write(req: FileMainWriteRequest) -> Result<FileMainWriteResponse, EgoError> {
    ic_cdk::println!("ego-file: file_main_write");

    let ret = EgoFileService::file_main_write(&req.fid, &req.hash, req.data)?;
    Ok(FileMainWriteResponse { ret })
}

#[query(name = "file_main_read", guard = "user_guard")]
#[candid_method(query, rename = "file_main_read")]
fn file_main_read(req: FileMainReadRequest) -> Result<FileMainReadResponse, EgoError> {
    ic_cdk::println!("ego-file: file_main_read");

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
    ic_cdk::println!("ego-file: state_persist");

    let storage = STORAGE.with(|s| s.borrow().clone());
    let user = users_pre_upgrade();
    let registry = registry_pre_upgrade();

    let state = PersistState { storage, user, registry };

    let data = Encode!(&state).unwrap();

    ic_cdk::println!("==> data length is: {}", data.len());

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
    ic_cdk::println!("ego-file: state_restore");

    // read file
    let mut buf = vec![0; DEFAULT_FILE_SIZE as usize];
    stable64_read(0, &mut buf); // file length
    let len = u64::from_le_bytes(buf[0..8].try_into().unwrap()) as usize;

    ic_cdk::println!("==> data length is: {}", len);

    let data = &buf[8..8 + len];
    let state = Decode!(data, PersistState).unwrap();

    STORAGE.with(|s| *s.borrow_mut() = state.storage);
    users_post_upgrade(state.user);
    registry_post_upgrade(state.registry);

    Ok(true)
}
