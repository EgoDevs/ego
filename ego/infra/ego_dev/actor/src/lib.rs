use candid::candid_method;
use ic_cdk::storage;
use ic_cdk_macros::*;
use ego_dev_mod::c2c::ego_file::EgoFile;

use ego_dev_mod::ego_dev::EgoDev;
use ego_dev_mod::service::*;
use ego_dev_mod::state::{EGO_DEV};
use ego_dev_mod::types::{AdminFileAddRequest, AdminFileAddResponse, AppMainGetRequest, AppMainGetResponse, AppMainNewRequest, AppMainNewResponse, AppVersionApproveRequest, AppVersionApproveResponse, AppVersionNewRequest, AppVersionNewResponse, AppVersionRejectRequest, AppVersionRejectResponse, AppVersionReleaseRequest, AppVersionReleaseResponse, AppVersionRevokeRequest, AppVersionRevokeResponse, AppVersionSetFrontendAddressRequest, AppVersionSetFrontendAddressResponse, AppVersionSubmitRequest, AppVersionSubmitResponse, AppVersionUploadWasmRequest, AppVersionUploadWasmResponse, AppVersionWaitForAuditResponse, DeveloperAppListResponse, DeveloperMainGetResponse, DeveloperMainRegisterRequest, DeveloperMainRegisterResponse, UserMainListRequest, UserMainListResponse, UserRoleSetRequest, UserRoleSetResponse};
use ego_types::ego_error::EgoError;

#[init]
#[candid_method(init)]
fn init() {
  let caller = ic_cdk::caller();
  ic_cdk::println!("ego-dev: init, caller is {}", caller);

  EGO_DEV.with(|ego_dev| {
    ego_dev.borrow_mut().developer_main_register(caller, "admin".to_string());
  });
}

#[pre_upgrade]
fn pre_upgrade() {
  ic_cdk::println!("ego-dev: pre_upgrade");
  EGO_DEV.with(|ego_dev| storage::stable_save((ego_dev, )).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
  ic_cdk::println!("ego-dev: post_upgrade");
  let (old_ego_dev, ): (EgoDev, ) = storage::stable_restore().unwrap();
  EGO_DEV.with(|ego_dev|
    *ego_dev.borrow_mut() = old_ego_dev
  );
}


/********************  for app user  ********************/

// 注册开发者账号
#[update(name = "developer_main_register")]
pub async fn developer_main_register(request: DeveloperMainRegisterRequest) -> Result<DeveloperMainRegisterResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_main_register");
  let developer = EgoDevService::developer_main_register(ic_cdk::caller(), request.name)?;
  Ok(DeveloperMainRegisterResponse { developer })
}

/********************  for app developer  ********************/

// 获取个人信息
#[query(name = "developer_main_get")]
pub fn developer_main_get() -> Result<DeveloperMainGetResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_main_get");
  let developer = EgoDevService::developer_main_get(ic_cdk::caller())?;
  Ok(DeveloperMainGetResponse { developer })
}

// 创建的应用列表
#[query(name = "developer_app_list", guard = "developer_guard")]
pub fn developer_app_list() -> Result<DeveloperAppListResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_app_list");
  let apps = EgoDevService::developer_app_list(ic_cdk::caller())?;
  Ok(DeveloperAppListResponse { apps })
}

// 获取应用详情
#[query(name = "developer_app_get")]
pub fn developer_app_get(request: AppMainGetRequest) -> Result<AppMainGetResponse, EgoError> {
  ic_cdk::println!("ego-dev: developer_app_get");
  let app = EgoDevService::developer_app_get(ic_cdk::caller(), request.app_id)?;
  Ok(AppMainGetResponse { app })
}


// 新建App
#[update(name = "developer_app_new", guard = "developer_guard")]
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
pub fn app_version_new(
  request: AppVersionNewRequest,
) -> Result<AppVersionNewResponse, EgoError> {
  ic_cdk::println!("ego-dev: new_app_version");

  let app_version = EgoDevService::app_version_new(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionNewResponse { app_version })
}

#[update(name = "app_version_upload_wasm")]
async fn app_version_upload_wasm(req: AppVersionUploadWasmRequest) -> Result<AppVersionUploadWasmResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_upload_wasm");

  match EgoDevService::app_version_upload_wasm(EgoFile::new(), ic_cdk::caller(), req.app_id, req.version, req.data, req.hash).await {
    Ok(ret) => Ok(AppVersionUploadWasmResponse{ret}),
    Err(e) => Err(e.into())
  }
}


#[update(name = "app_version_set_frontend_address", guard = "developer_guard")]
pub fn app_version_set_frontend_address(
  request: AppVersionSetFrontendAddressRequest,
) -> Result<AppVersionSetFrontendAddressResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_set_frontend_address: {}", request.canister_id);
  let ret = EgoDevService::app_version_set_frontend_address(ic_cdk::caller(), request.app_id, request.version, request.canister_id)?;
  Ok(AppVersionSetFrontendAddressResponse { ret })
}


// 提交审核
#[update(name = "app_version_submit", guard = "developer_guard")]
pub fn app_version_submit(
  request: AppVersionSubmitRequest,
) -> Result<AppVersionSubmitResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_submit");
  let app_version = EgoDevService::app_version_submit(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionSubmitResponse { app_version })
}


// 撤回审核
#[update(name = "app_version_revoke", guard = "developer_guard")]
pub fn app_version_revoke(
  request: AppVersionRevokeRequest,
) -> Result<AppVersionRevokeResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_revoke");
  let app_version = EgoDevService::app_version_revoke(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionRevokeResponse { app_version })
}


// 发布版本
#[update(name = "app_version_release", guard = "developer_guard")]
pub async fn app_version_release(
  request: AppVersionReleaseRequest,
) -> Result<AppVersionReleaseResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_release");
  let app_version = EgoDevService::app_version_release(ic_cdk::caller(), request.app_id, request.version)?;
  Ok(AppVersionReleaseResponse { app_version })
}


/********************  for app auditor  ********************/
// 待审核应用列表
#[query(name = "app_version_wait_for_audit", guard = "auditer_guard")]
pub fn app_version_wait_for_audit() -> Result<AppVersionWaitForAuditResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_wait_for_audit");

  let wait_for_audit_apps = EgoDevService::app_version_wait_for_audit();
  Ok(AppVersionWaitForAuditResponse { apps: wait_for_audit_apps })
}

// 通过当前版本审核
#[update(name = "app_version_approve", guard = "auditer_guard")]
pub fn app_version_approve(
  request: AppVersionApproveRequest,
) -> Result<AppVersionApproveResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_approve");
  let app_version = EgoDevService::app_version_approve(request.app_id, request.version)?;
  Ok(AppVersionApproveResponse { app_version })
}

// 驳回当前版本审核
#[update(name = "app_version_reject", guard = "auditer_guard")]
pub fn app_version_reject(
  request: AppVersionRejectRequest,
) -> Result<AppVersionRejectResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_reject");
  let app_version = EgoDevService::app_version_reject(request.app_id, request.version)?;
  Ok(AppVersionRejectResponse { app_version })
}

/********************  for app manager  ********************/
#[update(name = "user_role_set")]
pub fn user_role_set(request: UserRoleSetRequest) -> Result<UserRoleSetResponse, EgoError> {
  ic_cdk::println!("ego-dev: user_role_set");
  let ret = EgoDevService::user_role_set(request.user_id, request.is_app_auditor, request.is_manager)?;
  Ok(UserRoleSetResponse { ret })
}

#[query(name = "user_main_list")]
pub fn user_main_list(request: UserMainListRequest) -> Result<UserMainListResponse, EgoError> {
  ic_cdk::println!("ego-dev: app_version_wait_for_audit");
  let users = EgoDevService::user_main_list(request.name);
  Ok(UserMainListResponse { users })
}

/********************  owner methods  ********************/
#[update]
pub fn admin_file_add(req: AdminFileAddRequest) -> Result<AdminFileAddResponse, EgoError> {
  ic_cdk::println!("ego-dev: admin_file_add");

  let ret = EgoDevService::admin_file_add(req.file_id)?;
  Ok(AdminFileAddResponse { ret })
}

