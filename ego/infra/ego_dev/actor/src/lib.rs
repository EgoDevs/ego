use candid::candid_method;
use ic_cdk::storage;
use ic_cdk_macros::*;
use ic_types::Principal;
use ego_dev_mod::c2c::ego_file::EgoFile;
use ego_dev_mod::c2c::ego_store::EgoStore;

use ego_dev_mod::ego_dev::EgoDev;
use ego_dev_mod::service::*;
use ego_dev_mod::state::{EGO_DEV, EGO_STORE_CANISTER_ID};
use ego_dev_mod::types::{AdminEgoStoreSetRequest, AdminEgoStoreSetResponse, AdminFileAddRequest, AdminFileAddResponse, AppMainGetRequest, AppMainGetResponse, AppMainNewRequest, AppMainNewResponse, AppVersionApproveRequest, AppVersionApproveResponse, AppVersionNewRequest, AppVersionNewResponse, AppVersionRejectRequest, AppVersionRejectResponse, AppVersionReleaseRequest, AppVersionReleaseResponse, AppVersionRevokeRequest, AppVersionRevokeResponse, AppVersionSetFrontendAddressRequest, AppVersionSetFrontendAddressResponse, AppVersionSubmitRequest, AppVersionSubmitResponse, AppVersionUploadWasmRequest, AppVersionUploadWasmResponse, AppVersionWaitForAuditResponse, DeveloperAppListResponse, DeveloperMainGetResponse, DeveloperMainRegisterRequest, DeveloperMainRegisterResponse, UserMainListRequest, UserMainListResponse, UserRoleSetRequest, UserRoleSetResponse};
use ego_types::ego_error::EgoError;

use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

#[init]
#[candid_method(init)]
fn init() {
  let caller = ic_cdk::caller();
  ic_cdk::println!("ego-dev: init, caller is {}", caller);

  EGO_DEV.with(|ego_dev| {
    ego_dev.borrow_mut().developer_main_register(caller, "admin".to_string());
  });
}

#[derive(CandidType, Deserialize, Serialize)]
struct PersistState{
  pub ego_dev: EgoDev,
  pub ego_store: Option<Principal>
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-dev: pre_upgrade");
  let ego_dev = EGO_DEV.with(|ego_dev| ego_dev.borrow().clone());
  let ego_store = EGO_STORE_CANISTER_ID.with(|r_c| r_c.borrow().clone());

  let state = PersistState{ego_dev, ego_store};
  storage::stable_save((state, )).unwrap();
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-dev: post_upgrade");
  let (state, ): (PersistState, ) = storage::stable_restore().unwrap();
  EGO_DEV.with(|ego_dev|
    *ego_dev.borrow_mut() = state.ego_dev
  );
  EGO_STORE_CANISTER_ID.with(|r_c| *r_c.borrow_mut() = state.ego_store);
}


/********************  for app user  ********************/

// 注册开发者账号
#[update(name = "developer_main_register")]
#[candid_method(update, rename = "developer_main_register")]
pub async fn developer_main_register(request: DeveloperMainRegisterRequest) -> Result<DeveloperMainRegisterResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_main_register");
  let developer = EgoDevService::developer_main_register(ic_cdk::caller(), request.name)?;
  Ok(DeveloperMainRegisterResponse { developer })
}

/********************  for app developer  ********************/

// 获取个人信息
#[query(name = "developer_main_get")]
#[candid_method(query, rename = "developer_main_get")]
pub fn developer_main_get() -> Result<DeveloperMainGetResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_main_get");
  let developer = EgoDevService::developer_main_get(ic_cdk::caller())?;
  Ok(DeveloperMainGetResponse { developer })
}

// 创建的应用列表
#[query(name = "developer_app_list", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_list")]
pub fn developer_app_list() -> Result<DeveloperAppListResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_app_list");
  let apps = EgoDevService::developer_app_list(ic_cdk::caller())?;
  Ok(DeveloperAppListResponse { apps })
}

// 获取应用详情
#[query(name = "developer_app_get")]
#[candid_method(query, rename = "developer_app_get")]
pub fn developer_app_get(request: AppMainGetRequest) -> Result<AppMainGetResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_app_get");
  let app = EgoDevService::developer_app_get(ic_cdk::caller(), request.app_id)?;
  Ok(AppMainGetResponse { app })
}

// TODO: developer_cycle_list


// 新建App
#[update(name = "developer_app_new", guard = "developer_guard")]
#[candid_method(update, rename = "developer_app_new")]
pub fn developer_app_new(request: AppMainNewRequest) -> Result<AppMainNewResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_app_new");

  let app = EgoDevService::developer_app_new(
    ic_cdk::caller(),
    request.app_id,
    request.name,
    request.category,
    request.price,
  )?;
  Ok(AppMainNewResponse { app })
}

// 新建版本
#[update(name = "app_version_new", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_new")]
pub fn app_version_new(
  request: AppVersionNewRequest,
) -> Result<AppVersionNewResponse, EgoError> {
  ic_cdk::println!("ego-dev: new_app_version");

  let app_version = EgoDevService::app_version_new(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionNewResponse { app_version })
}

#[update(name = "app_version_upload_wasm")]
#[candid_method(update, rename = "app_version_upload_wasm")]
async fn app_version_upload_wasm(req: AppVersionUploadWasmRequest) -> Result<AppVersionUploadWasmResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_upload_wasm");

  match EgoDevService::app_version_upload_wasm(EgoFile::new(), ic_cdk::caller(), req.app_id, req.version, req.data, req.hash).await {
    Ok(ret) => Ok(AppVersionUploadWasmResponse{ret}),
    Err(e) => Err(e.into())
  }
}


#[update(name = "app_version_set_frontend_address", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_set_frontend_address")]
pub fn app_version_set_frontend_address(
  request: AppVersionSetFrontendAddressRequest,
) -> Result<AppVersionSetFrontendAddressResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_set_frontend_address: {}", request.canister_id);
  let ret = EgoDevService::app_version_set_frontend_address(ic_cdk::caller(), request.app_id, request.version, request.canister_id)?;
  Ok(AppVersionSetFrontendAddressResponse { ret })
}


// 提交审核
#[update(name = "app_version_submit", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_submit")]
pub fn app_version_submit(
  request: AppVersionSubmitRequest,
) -> Result<AppVersionSubmitResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_submit");
  let app_version = EgoDevService::app_version_submit(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionSubmitResponse { app_version })
}


// 撤回审核
#[update(name = "app_version_revoke", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_revoke")]
pub fn app_version_revoke(
  request: AppVersionRevokeRequest,
) -> Result<AppVersionRevokeResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_revoke");
  let app_version = EgoDevService::app_version_revoke(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionRevokeResponse { app_version })
}


// 发布版本
#[update(name = "app_version_release", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_release")]
pub async fn app_version_release(
  request: AppVersionReleaseRequest,
) -> Result<AppVersionReleaseResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_release");
  let ego_store = EgoStore::new();
  let app_version = EgoDevService::app_version_release(ic_cdk::caller(), request.app_id, request.version, ego_store).await?;
  Ok(AppVersionReleaseResponse { app_version })
}


/********************  for app auditor  ********************/
// 待审核应用列表
#[query(name = "app_version_wait_for_audit", guard = "auditer_guard")]
#[candid_method(query, rename = "app_version_wait_for_audit")]
pub fn app_version_wait_for_audit() -> Result<AppVersionWaitForAuditResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_wait_for_audit");

  let wait_for_audit_apps = EgoDevService::app_version_wait_for_audit();
  Ok(AppVersionWaitForAuditResponse { apps: wait_for_audit_apps })
}

// 通过当前版本审核
#[update(name = "app_version_approve", guard = "auditer_guard")]
#[candid_method(update, rename = "app_version_approve")]
pub fn app_version_approve(
  request: AppVersionApproveRequest,
) -> Result<AppVersionApproveResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_approve");
  let app_version = EgoDevService::app_version_approve(request.app_id, request.version)?;
  Ok(AppVersionApproveResponse { app_version })
}

// 驳回当前版本审核
#[update(name = "app_version_reject", guard = "auditer_guard")]
#[candid_method(update, rename = "app_version_reject")]
pub fn app_version_reject(
  request: AppVersionRejectRequest,
) -> Result<AppVersionRejectResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_reject");
  let app_version = EgoDevService::app_version_reject(request.app_id, request.version)?;
  Ok(AppVersionRejectResponse { app_version })
}

/********************  for dev manager  ********************/
#[query(name = "user_main_list")]
#[candid_method(query, rename = "user_main_list")]
pub fn user_main_list(request: UserMainListRequest) -> Result<UserMainListResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_wait_for_audit");
  let users = EgoDevService::user_main_list(request.name);
  Ok(UserMainListResponse { users })
}

#[update(name = "user_role_set")]
#[candid_method(update, rename = "user_role_set")]
pub fn user_role_set(request: UserRoleSetRequest) -> Result<UserRoleSetResponse, EgoError> {
  ic_cdk::println!("ego-dev: user_role_set");
  let ret = EgoDevService::user_role_set(request.user_id, request.is_app_auditor, request.is_manager)?;
  Ok(UserRoleSetResponse { ret })
}


/********************  for ego_store  ********************/
// TODO: developer_cycle_recharge

/********************  owner methods  ********************/
#[update]
#[candid_method(update, rename = "admin_file_add")]
pub fn admin_file_add(req: AdminFileAddRequest) -> Result<AdminFileAddResponse, EgoError> {
  ic_cdk::println!("ego-dev: admin_file_add");

  let ret = EgoDevService::admin_file_add(req.canister_id)?;
  Ok(AdminFileAddResponse { ret })
}

#[update]
#[candid_method(update, rename = "admin_ego_store_set")]
pub fn admin_ego_store_set(req: AdminEgoStoreSetRequest) -> Result<AdminEgoStoreSetResponse, EgoError> {
  ic_cdk::println!("ego-dev: admin_ego_store_set");

  EGO_STORE_CANISTER_ID.with(|rc| *rc.borrow_mut() = Some(req.canister_id));

  Ok(AdminEgoStoreSetResponse{ret: true})
}


