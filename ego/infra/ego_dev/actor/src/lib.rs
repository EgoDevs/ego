use std::collections::BTreeMap;

use candid::candid_method;
use ic_cdk::{api, caller, storage, trap};
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use serde::Serialize;

use ego_dev_mod::c2c::ego_file::EgoFile;
use ego_dev_mod::c2c::ego_store::EgoStore;
use ego_dev_mod::ego_dev::EgoDev;
use ego_dev_mod::ego_macros::inject_ego_macros;
use ego_dev_mod::service::*;
use ego_dev_mod::state::EGO_DEV;
use ego_dev_mod::types::{
  AdminAppCreateRequest, AdminAppCreateResponse, AppMainGetRequest, AppMainGetResponse,
  AppMainNewRequest, AppMainNewResponse, AppVersionApproveRequest, AppVersionApproveResponse,
  AppVersionNewRequest, AppVersionNewResponse, AppVersionRejectRequest, AppVersionRejectResponse,
  AppVersionReleaseRequest, AppVersionReleaseResponse, AppVersionRevokeRequest,
  AppVersionRevokeResponse, AppVersionSetFrontendAddressRequest,
  AppVersionSetFrontendAddressResponse, AppVersionSubmitRequest, AppVersionSubmitResponse,
  AppVersionUploadWasmRequest, AppVersionUploadWasmResponse, AppVersionWaitForAuditResponse,
  DeveloperAppListResponse, DeveloperMainGetResponse, DeveloperMainRegisterRequest,
  DeveloperMainRegisterResponse, UserMainListRequest, UserMainListResponse, UserRoleSetRequest,
  UserRoleSetResponse,
};
use ego_types::app::DeployMode;
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
  ego_log(format!("ego-dev: init, caller is {}", caller.clone()).as_str());

  ego_log("==> add caller as the owner");
  owner_add(caller.clone());

  ego_log("==> caller register as an developer");
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
  pub user: User,
  pub registry: Registry,
}

#[pre_upgrade]
fn pre_upgrade() {
  ego_log("ego-dev: pre_upgrade");
  let ego_dev = EGO_DEV.with(|ego_dev| ego_dev.borrow().clone());
  let user = users_pre_upgrade();
  let registry = registry_pre_upgrade();

  let state = PersistState {
    ego_dev,
    user,
    registry,
  };
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ego_log("ego-dev: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_DEV.with(|ego_dev| *ego_dev.borrow_mut() = state.ego_dev);

  users_post_upgrade(state.user);
  registry_post_upgrade(state.registry);
}


/********************  anonymous  ********************/

// 注册开发者账号
#[update(name = "developer_main_register")]
#[candid_method(update, rename = "developer_main_register")]
pub async fn developer_main_register(
  request: DeveloperMainRegisterRequest,
) -> Result<DeveloperMainRegisterResponse, EgoError> {
  ego_log("ego-dev: developer_main_register");
  let developer = EgoDevService::developer_main_register(ic_cdk::caller(), request.name)?;
  Ok(DeveloperMainRegisterResponse { developer })
}

/********************  developer  ********************/

// 获取个人信息
#[query(name = "developer_main_get")]
#[candid_method(query, rename = "developer_main_get")]
pub fn developer_main_get() -> Result<DeveloperMainGetResponse, EgoError> {
  ego_log("ego-dev: developer_main_get");
  let developer = EgoDevService::developer_main_get(ic_cdk::caller())?;
  Ok(DeveloperMainGetResponse { developer })
}

// 创建的应用列表
#[query(name = "developer_app_list", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_list")]
pub fn developer_app_list() -> Result<DeveloperAppListResponse, EgoError> {
  ego_log("ego-dev: developer_app_list");
  let apps = EgoDevService::developer_app_list(ic_cdk::caller())?;
  Ok(DeveloperAppListResponse { apps })
}

// 获取应用详情
#[query(name = "developer_app_get", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_get")]
pub fn developer_app_get(request: AppMainGetRequest) -> Result<AppMainGetResponse, EgoError> {
  ego_log("ego-dev: developer_app_get");
  let app = EgoDevService::developer_app_get(ic_cdk::caller(), request.app_id)?;
  Ok(AppMainGetResponse { app })
}

// TODO: developer_cycle_list

// 新建App
#[update(name = "developer_app_new", guard = "developer_guard")]
#[candid_method(update, rename = "developer_app_new")]
pub fn developer_app_new(request: AppMainNewRequest) -> Result<AppMainNewResponse, EgoError> {
  ego_log("ego-dev: developer_app_new");

  let app = EgoDevService::developer_app_new(
    ic_cdk::caller(),
    request.app_id,
    request.name,
    request.logo,
    request.description,
    request.category,
    request.price,
    DeployMode::DEDICATED,
  )?;
  Ok(AppMainNewResponse { app })
}

// 新建版本
#[update(name = "app_version_new", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_new")]
pub fn app_version_new(request: AppVersionNewRequest) -> Result<AppVersionNewResponse, EgoError> {
  ego_log("ego-dev: new_app_version");

  let app_version =
    EgoDevService::app_version_new(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionNewResponse { app_version })
}

#[update(name = "app_version_upload_wasm", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_upload_wasm")]
async fn app_version_upload_wasm(
  request: AppVersionUploadWasmRequest,
) -> Result<AppVersionUploadWasmResponse, EgoError> {
  ego_log("ego-dev: app_version_upload_wasm");

  let ret = EgoDevService::app_version_upload_wasm(
    EgoFile::new(),
    ic_cdk::caller(),
    request.app_id,
    request.version,
    request.data,
    request.hash,
  )
    .await?;
  Ok(AppVersionUploadWasmResponse { ret })
}

#[update(name = "app_version_set_frontend_address", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_set_frontend_address")]
pub fn app_version_set_frontend_address(
  request: AppVersionSetFrontendAddressRequest,
) -> Result<AppVersionSetFrontendAddressResponse, EgoError> {
  ego_log(&format!(
    "ego-dev: app_version_set_frontend_address: {}",
    request.canister_id)
  );
  let ret = EgoDevService::app_version_set_frontend_address(
    ic_cdk::caller(),
    request.app_id,
    request.version,
    request.canister_id,
  )?;
  Ok(AppVersionSetFrontendAddressResponse { ret })
}

// 提交审核
#[update(name = "app_version_submit", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_submit")]
pub fn app_version_submit(
  request: AppVersionSubmitRequest,
) -> Result<AppVersionSubmitResponse, EgoError> {
  ego_log("ego-dev: app_version_submit");
  let app_version =
    EgoDevService::app_version_submit(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionSubmitResponse { app_version })
}

// 撤回审核
#[update(name = "app_version_revoke", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_revoke")]
pub fn app_version_revoke(
  request: AppVersionRevokeRequest,
) -> Result<AppVersionRevokeResponse, EgoError> {
  ego_log("ego-dev: app_version_revoke");
  let app_version =
    EgoDevService::app_version_revoke(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionRevokeResponse { app_version })
}

// 发布版本
#[update(name = "app_version_release", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_release")]
pub async fn app_version_release(
  request: AppVersionReleaseRequest,
) -> Result<AppVersionReleaseResponse, EgoError> {
  ego_log("ego-dev: app_version_release");
  let caller = caller();

  let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
  let ego_store = EgoStore::new(ego_store_id);
  let app_version = EgoDevService::app_version_release(caller, request.app_id.clone(), request.version, ego_store)?;
  Ok(AppVersionReleaseResponse { app_version })
}

/********************  auditor  ********************/
// 待审核应用列表
#[query(name = "app_version_wait_for_audit", guard = "auditor_guard")]
#[candid_method(query, rename = "app_version_wait_for_audit")]
pub fn app_version_wait_for_audit() -> Result<AppVersionWaitForAuditResponse, EgoError> {
  ego_log("ego-dev: app_version_wait_for_audit");

  let wait_for_audit_apps = EgoDevService::app_version_wait_for_audit();
  Ok(AppVersionWaitForAuditResponse {
    apps: wait_for_audit_apps,
  })
}

// 通过当前版本审核
#[update(name = "app_version_approve", guard = "auditor_guard")]
#[candid_method(update, rename = "app_version_approve")]
pub fn app_version_approve(
  request: AppVersionApproveRequest,
) -> Result<AppVersionApproveResponse, EgoError> {
  ego_log("ego-dev: app_version_approve");
  let app_version = EgoDevService::app_version_approve(request.app_id, request.version)?;
  Ok(AppVersionApproveResponse { app_version })
}

// 驳回当前版本审核
#[update(name = "app_version_reject", guard = "auditor_guard")]
#[candid_method(update, rename = "app_version_reject")]
pub fn app_version_reject(
  request: AppVersionRejectRequest,
) -> Result<AppVersionRejectResponse, EgoError> {
  ego_log("ego-dev: app_version_reject");
  let app_version = EgoDevService::app_version_reject(request.app_id, request.version)?;
  Ok(AppVersionRejectResponse { app_version })
}

/********************  manager  ********************/
#[query(name = "user_main_list", guard = "manager_guard")]
#[candid_method(query, rename = "user_main_list")]
pub fn user_main_list(request: UserMainListRequest) -> Result<UserMainListResponse, EgoError> {
  ego_log("ego-dev: user_main_list");
  let users = EgoDevService::user_main_list(request.name);
  Ok(UserMainListResponse { users })
}

#[update(name = "user_role_set", guard = "manager_guard")]
#[candid_method(update, rename = "user_role_set")]
pub fn user_role_set(request: UserRoleSetRequest) -> Result<UserRoleSetResponse, EgoError> {
  ego_log("ego-dev: user_role_set");
  let ret =
    EgoDevService::user_role_set(request.user_id, request.is_app_auditor, request.is_manager)?;
  Ok(UserRoleSetResponse { ret })
}

/********************  ego_store  ********************/
// TODO: developer_cycle_recharge

/********************  owner  ********************/
#[update(name = "admin_app_create", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_create")]
pub async fn admin_app_create(
  request: AdminAppCreateRequest,
) -> Result<AdminAppCreateResponse, EgoError> {
  ego_log("ego-dev: admin_app_create");

  let caller = ic_cdk::caller();

  ego_log("1. developer_main_register");
  EgoDevService::developer_main_register(caller, "astrox".to_string())?;

  ego_log("2. developer_app_new");
  EgoDevService::developer_app_new(
    caller,
    request.app_id.clone(),
    request.name,
    request.logo,
    request.description,
    request.category,
    0f32,
    request.deploy_mode,
  )?;

  ego_log("3. app_version_new");
  EgoDevService::app_version_new(caller, request.app_id.clone(), request.version)?;

  ego_log("4. app_version_upload_wasm");
  EgoDevService::app_version_upload_wasm(
    EgoFile::new(),
    ic_cdk::caller(),
    request.app_id.clone(),
    request.version,
    request.backend_data,
    request.backend_data_hash,
  )
    .await?;

  if request.frontend.is_some() {
    ego_log("5. app_version_set_frontend_address");
    EgoDevService::app_version_set_frontend_address(
      caller,
      request.app_id.clone(),
      request.version,
      request.frontend.unwrap(),
    )?;
  }


  ego_log("6. app_version_release");
  let ego_store_id = REGISTRY.with(|r| r.borrow().canister_get_one("ego_store")).unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  let app_version = EgoDevService::app_version_release(caller, request.app_id.clone(), request.version, ego_store)?;
  Ok(AdminAppCreateResponse { app_version })
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
