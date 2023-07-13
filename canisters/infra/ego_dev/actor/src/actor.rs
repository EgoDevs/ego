use std::collections::BTreeMap;

use candid::candid_method;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk::{api, caller, storage, trap};
use ic_cdk_macros::*;
use serde::Serialize;

use ego_dev_mod::app::{AppVersion, EgoDevApp};
use ego_dev_mod::c2c::ego_file::EgoFile;
use ego_dev_mod::c2c::ego_store::EgoStore;
use ego_dev_mod::developer::Developer;
use ego_dev_mod::ego_dev::EgoDev;
use ego_dev_mod::service::*;
use ego_dev_mod::state::EGO_DEV;
use ego_dev_mod::state::*;
use ego_dev_mod::types::{AdminAppCreateBackendRequest, AppMainNewRequest, AppVersionSetFrontendAddressRequest, AppVersionUploadWasmRequest, UserRoleSetRequest};
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_types::app::EgoError;
use ego_types::app::{AppId, Version};
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
    info_log_add(format!("ego-dev: init, caller is {}", caller.clone()).as_str());

    info_log_add("==> add caller as the owner");
    owner_add(caller.clone());

    info_log_add("==> caller register as an developer");
    match EgoDevService::developer_main_register(caller.clone(), "admin".to_string()) {
        _ => {}
    }
    match EgoDevService::user_role_set(caller.clone(), true, true) {
        _ => {}
    }
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState {
    pub ego_dev: EgoDev,
    users: Option<User>,
    registry: Option<Registry>,
    cycle_info: Option<CycleInfo>,
}

#[pre_upgrade]
fn pre_upgrade() {
    info_log_add("ego-dev: pre_upgrade");
    let ego_dev = EGO_DEV.with(|ego_dev| ego_dev.borrow().clone());

    let state = PersistState {
        ego_dev,
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        cycle_info: Some(cycle_info_pre_upgrade()),
    };
    storage::stable_save((state,)).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
    info_log_add("ego-dev: post_upgrade");
    let (state,): (PersistState,) = storage::stable_restore().unwrap();
    EGO_DEV.with(|ego_dev| *ego_dev.borrow_mut() = state.ego_dev);

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
}

/********************  anonymous  ********************/

// 注册开发者账号
#[update(name = "developer_main_register")]
#[candid_method(update, rename = "developer_main_register")]
pub async fn developer_main_register(name: String) -> Result<Developer, EgoError> {
    info_log_add("ego-dev: developer_main_register");
    let developer = EgoDevService::developer_main_register(ic_cdk::caller(), name)?;
    Ok(developer)
}

/********************  developer  ********************/

// 获取个人信息
#[query(name = "developer_main_get")]
#[candid_method(query, rename = "developer_main_get")]
pub fn developer_main_get() -> Result<Developer, EgoError> {
    info_log_add("ego-dev: developer_main_get");
    let developer = EgoDevService::developer_main_get(ic_cdk::caller())?;
    Ok(developer)
}

// 创建的应用列表
#[query(name = "developer_app_list", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_list")]
pub fn developer_app_list() -> Result<Vec<EgoDevApp>, EgoError> {
    info_log_add("ego-dev: developer_app_list");
    let apps = EgoDevService::developer_app_list(ic_cdk::caller())?;
    Ok(apps)
}

// 获取应用详情
#[query(name = "developer_app_get", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_get")]
pub fn developer_app_get(app_id: AppId) -> Result<EgoDevApp, EgoError> {
    info_log_add("ego-dev: developer_app_get");
    let app = EgoDevService::developer_app_get(ic_cdk::caller(), app_id)?;
    Ok(app)
}

// TODO: developer_cycle_list

// 新建App
#[update(name = "developer_app_new", guard = "developer_guard")]
#[candid_method(update, rename = "developer_app_new")]
pub fn developer_app_new(request: AppMainNewRequest) -> Result<EgoDevApp, EgoError> {
    info_log_add("ego-dev: developer_app_new");

    let app = EgoDevService::developer_app_new(
        ic_cdk::caller(),
        request.app_id,
        request.name,
        request.logo,
        request.description,
        request.category,
        request.price,
    )?;
    Ok(app)
}

// 新建版本
#[update(name = "app_version_new", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_new")]
pub fn app_version_new(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: new_app_version");

    let app_version = EgoDevService::app_version_new(ic_cdk::caller(), app_id, version)?;
    Ok(app_version)
}

#[update(name = "app_version_upload_wasm", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_upload_wasm")]
async fn app_version_upload_wasm(request: AppVersionUploadWasmRequest) -> Result<bool, EgoError> {
    info_log_add("ego-dev: app_version_upload_wasm");

    let ret = EgoDevService::app_version_upload_wasm(
        EgoFile::new(),
        ic_cdk::caller(),
        request.app_id,
        request.version,
        request.data,
        request.hash,
    )
    .await?;
    Ok(ret)
}

#[update(name = "app_version_set_frontend_address", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_set_frontend_address")]
pub fn app_version_set_frontend_address(
    request: AppVersionSetFrontendAddressRequest,
) -> Result<bool, EgoError> {
    info_log_add(&format!(
        "ego-dev: app_version_set_frontend_address: {}",
        request.canister_id
    ));
    let ret = EgoDevService::app_version_set_frontend_address(
        ic_cdk::caller(),
        request.app_id,
        request.version,
        request.canister_id,
    )?;
    Ok(ret)
}

// 提交审核
#[update(name = "app_version_submit", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_submit")]
pub fn app_version_submit(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: app_version_submit");
    let app_version = EgoDevService::app_version_submit(ic_cdk::caller(), app_id, version)?;
    Ok(app_version)
}

// 撤回审核
#[update(name = "app_version_revoke", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_revoke")]
pub fn app_version_revoke(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: app_version_revoke");
    let app_version = EgoDevService::app_version_revoke(ic_cdk::caller(), app_id, version)?;
    Ok(app_version)
}

// 发布版本
#[update(name = "app_version_release", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_release")]
pub async fn app_version_release(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: app_version_release");
    let caller = caller();

    let ego_store_id = canister_get_one("ego_store").unwrap();
    let ego_store = EgoStore::new(ego_store_id);
    EgoDevService::app_version_release(caller, app_id, version, ego_store)
}

/********************  auditor  ********************/
// 待审核应用列表
#[query(name = "app_version_wait_for_audit", guard = "auditor_guard")]
#[candid_method(query, rename = "app_version_wait_for_audit")]
pub fn app_version_wait_for_audit() -> Result<Vec<EgoDevApp>, EgoError> {
    info_log_add("ego-dev: app_version_wait_for_audit");

    let wait_for_audit_apps = EgoDevService::app_version_wait_for_audit();
    Ok(wait_for_audit_apps)
}

// 通过当前版本审核
#[update(name = "app_version_approve", guard = "auditor_guard")]
#[candid_method(update, rename = "app_version_approve")]
pub fn app_version_approve(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: app_version_approve");
    let app_version = EgoDevService::app_version_approve(app_id, version)?;
    Ok(app_version)
}

// 驳回当前版本审核
#[update(name = "app_version_reject", guard = "auditor_guard")]
#[candid_method(update, rename = "app_version_reject")]
pub fn app_version_reject(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: app_version_reject");
    let app_version = EgoDevService::app_version_reject(app_id, version)?;
    Ok(app_version)
}

/********************  manager  ********************/
#[query(name = "user_main_list", guard = "manager_guard")]
#[candid_method(query, rename = "user_main_list")]
pub fn user_main_list(name: String) -> Result<Vec<Developer>, EgoError> {
    info_log_add("ego-dev: user_main_list");
    let users = EgoDevService::user_main_list(name);
    Ok(users)
}

#[update(name = "user_role_set", guard = "manager_guard")]
#[candid_method(update, rename = "user_role_set")]
pub fn user_role_set(request: UserRoleSetRequest) -> Result<bool, EgoError> {
    info_log_add("ego-dev: user_role_set");
    let ret =
        EgoDevService::user_role_set(request.user_id, request.is_app_auditor, request.is_manager)?;
    Ok(ret)
}

/********************  ego_store  ********************/
// TODO: developer_cycle_recharge

/********************  owner  ********************/
#[update(name = "admin_app_create", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_create")]
pub async fn admin_app_create(
    request: AdminAppCreateBackendRequest,
) -> Result<AppVersion, EgoError> {
    info_log_add("ego-dev: admin_app_create");

    let caller = ic_cdk::caller();

    if EGO_DEV.with(|ego_dev| ego_dev.borrow().developers.contains_key(&caller)) {
        info_log_add("1. developer exists. skip developer registration");
    } else {
        info_log_add("1. developer_main_register");
        EgoDevService::developer_main_register(caller, "ego_deployer".to_string())?;
    }

    info_log_add("2. developer_app_new");
    EgoDevService::developer_app_new(
        caller,
        request.app_id.clone(),
        request.name,
        request.logo,
        request.description,
        request.category,
        0f32,
    )?;

    info_log_add("3. app_version_new");
    EgoDevService::app_version_new(caller, request.app_id.clone(), request.version)?;

    info_log_add("4. app_version_upload_wasm");
    EgoDevService::app_version_upload_wasm(
        EgoFile::new(),
        ic_cdk::caller(),
        request.app_id.clone(),
        request.version,
        request.backend_data,
        request.backend_data_hash,
    )
    .await?;

    info_log_add("5. app_version_release");
    let ego_store_id = canister_get_one("ego_store").unwrap();
    let ego_store = EgoStore::new(ego_store_id);

    EgoDevService::app_version_release(caller, request.app_id.clone(), request.version, ego_store)
}

#[update(name = "admin_app_transfer", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_transfer")]
pub async fn admin_app_transfer(app_id: AppId) -> Result<(), EgoError> {
    info_log_add("ego-dev: admin_app_transfer");

    let caller = ic_cdk::caller();

    if EGO_DEV.with(|ego_dev| ego_dev.borrow().developers.contains_key(&caller)) {
        info_log_add("1. developer exists. skip developer registration");
    } else {
        info_log_add("1. developer_main_register");
        EgoDevService::developer_main_register(caller, "ego_deployer".to_string())?;
    }

    info_log_add("2. transfer app");

    EgoDevService::developer_app_transfer(
        caller,
        app_id.clone()
    )
}

/********************  guard  ********************/
#[inline(always)]
pub fn manager_guard() -> Result<(), String> {
    if EGO_DEV.with(|ego_dev| ego_dev.borrow().is_manager(api::caller())) {
        Ok(())
    } else {
        trap(&format!("{} unauthorized", api::caller()));
    }
}

#[inline(always)]
pub fn auditor_guard() -> Result<(), String> {
    if EGO_DEV.with(|ego_dev| ego_dev.borrow().is_app_auditor(api::caller())) {
        Ok(())
    } else {
        trap(&format!("{} unauthorized", api::caller()));
    }
}

#[inline(always)]
pub fn developer_guard() -> Result<(), String> {
    if EGO_DEV.with(|ego_dev| ego_dev.borrow().is_app_developer(api::caller())) {
        Ok(())
    } else {
        trap(&format!("{} unauthorized", api::caller()));
    }
}

/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
    1_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
    1_000_000_000_000
}
