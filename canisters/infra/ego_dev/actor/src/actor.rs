use std::collections::BTreeMap;

use candid::candid_method;
use candid::Principal;
use ego_backup::inject_backup_api;
use ic_cdk::{api, caller, trap};
use ic_cdk_macros::*;

use ego_dev_mod::backup::*;
use ego_dev_mod::c2c::ego_file::EgoFile;
use ego_dev_mod::c2c::ego_store::EgoStore;
use ego_dev_mod::service::*;
use ego_dev_mod::state::*;
use ego_dev_mod::types::{AdminAppCreateBackendRequest, AppMainNewRequest, AppVersionSetFrontendAddressRequest, AppVersionUploadWasmRequest, EgoDevErr, UserRoleSetRequest};
use ego_dev_mod::types::app_version::AppVersion;
use ego_dev_mod::types::developer::Developer;
use ego_dev_mod::types::ego_dev_app::EgoDevApp;
use ego_macros::{inject_cycle_info_api, inject_ego_api};
use ego_types::app::{AppId, Version};
use ego_types::app::EgoError;

inject_ego_api!();
inject_cycle_info_api!();
inject_backup_api!();

#[init]
#[candid_method(init)]
pub fn init() {
  let caller = caller();
  info_log_add(format!("ego_dev: init, caller is {}", caller.clone()).as_str());

  info_log_add("==> add caller as the owner");
  owner_add(caller.clone());

  info_log_add("==> caller register as an developer");
  EgoDevService::developer_main_register(&caller, "admin").expect("register developer admin failed");
  EgoDevService::user_role_set(&caller, true, true).expect("admin role set failed");
}

#[pre_upgrade]
fn pre_upgrade() {
  info_log_add("pre_upgrade");
  ego_dev_mod::state::pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
  info_log_add("post_upgrade");
  ego_dev_mod::state::post_upgrade();
}

/********************  anonymous  ********************/
// 注册开发者账号
#[update(name = "developer_main_register")]
#[candid_method(update, rename = "developer_main_register")]
pub async fn developer_main_register(name: String) -> Result<Developer, EgoError> {
  let caller = caller();
  info_log_add(format!("developer_main_register {}", caller).as_str());
  let developer = EgoDevService::developer_main_register(&caller, &name)?;
  Ok(developer)
}

/********************  developer  ********************/
// 获取个人信息
#[query(name = "developer_main_get")]
#[candid_method(query, rename = "developer_main_get")]
pub fn developer_main_get() -> Result<Developer, EgoError> {
  info_log_add("developer_main_get");

  match Developer::get(&caller()) {
    None => {
      Err(EgoDevErr::NotADeveloper.into())
    }
    Some(developer) => {
      Ok(developer)
    }
  }
}

// 创建的应用列表
#[query(name = "developer_app_list", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_list")]
pub fn developer_app_list() -> Result<Vec<EgoDevApp>, EgoError> {
  info_log_add("developer_app_list");

  let apps = EgoDevApp::by_developer_id(&caller());

  Ok(apps)
}

// 获取应用详情
#[query(name = "developer_app_get", guard = "developer_guard")]
#[candid_method(query, rename = "developer_app_get")]
pub fn developer_app_get(app_id: AppId) -> Result<EgoDevApp, EgoError> {
  info_log_add("developer_app_get");

  match EgoDevApp::by_developer_id_and_id(&caller(), &app_id) {
    None => {
      Err(EgoDevErr::AppNotExists.into())
    }
    Some(ego_dev_app) => {
      Ok(ego_dev_app)
    }
  }
}

// 新建App
#[update(name = "developer_app_new", guard = "developer_guard")]
#[candid_method(update, rename = "developer_app_new")]
pub fn developer_app_new(request: AppMainNewRequest) -> Result<EgoDevApp, EgoError> {
  info_log_add("developer_app_new");

  let app = EgoDevService::developer_app_new(
    &caller(),
    &request.app_id,
    &request.name,
    &request.logo,
    &request.description,
    &request.category,
    request.price,
  )?;
  Ok(app)
}

// 新建版本
#[update(name = "app_version_new", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_new")]
pub fn app_version_new(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
  info_log_add("new_app_version");

  let app_version = EgoDevService::app_version_new(&caller(), &app_id, &version)?;
  Ok(app_version)
}

#[update(name = "app_version_upload_wasm", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_upload_wasm")]
async fn app_version_upload_wasm(request: AppVersionUploadWasmRequest) -> Result<bool, EgoError> {
  info_log_add("app_version_upload_wasm");

  let ret = EgoDevService::app_version_upload_wasm(
    EgoFile::new(),
    &caller(),
    &request.app_id,
    &request.version,
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
    "app_version_set_frontend_address: {}",
    request.canister_id
  ));
  let ret = EgoDevService::app_version_set_frontend_address(
    &caller(),
    &request.app_id,
    &request.version,
    &request.canister_id,
  )?;
  Ok(ret)
}

// 提交审核
#[update(name = "app_version_submit", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_submit")]
pub fn app_version_submit(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
  info_log_add("app_version_submit");
  let app_version = EgoDevService::app_version_submit(&caller(), &app_id, &version)?;
  Ok(app_version)
}

// 撤回审核
#[update(name = "app_version_revoke", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_revoke")]
pub fn app_version_revoke(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
  info_log_add("app_version_revoke");
  let app_version = EgoDevService::app_version_revoke(&caller(), &app_id, &version)?;
  Ok(app_version)
}

// 发布版本
#[update(name = "app_version_release", guard = "developer_guard")]
#[candid_method(update, rename = "app_version_release")]
pub async fn app_version_release(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
  info_log_add("app_version_release");
  let caller = caller();

  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);
  EgoDevService::app_version_release(&caller, &app_id, &version, ego_store)
}

// TODO: developer_cycle_list

/********************  auditor  ********************/
// 待审核应用列表
#[query(name = "app_version_wait_for_audit", guard = "auditor_guard")]
#[candid_method(query, rename = "app_version_wait_for_audit")]
pub fn app_version_wait_for_audit() -> Result<Vec<EgoDevApp>, EgoError> {
  info_log_add("app_version_wait_for_audit");

  let wait_for_audit_apps = EgoDevApp::version_wait_for_audit();
  Ok(wait_for_audit_apps)
}

// 通过当前版本审核
#[update(name = "app_version_approve", guard = "auditor_guard")]
#[candid_method(update, rename = "app_version_approve")]
pub fn app_version_approve(app_id: AppId) -> Result<AppVersion, EgoError> {
  info_log_add("app_version_approve");
  let app_version = EgoDevService::app_version_approve(&app_id)?;
  Ok(app_version)
}

// 驳回当前版本审核
#[update(name = "app_version_reject", guard = "auditor_guard")]
#[candid_method(update, rename = "app_version_reject")]
pub fn app_version_reject(app_id: AppId) -> Result<AppVersion, EgoError> {
  info_log_add("app_version_reject");
  let app_version = EgoDevService::app_version_reject(&app_id)?;
  Ok(app_version)
}

/********************  manager  ********************/
#[query(name = "user_main_list", guard = "manager_guard")]
#[candid_method(query, rename = "user_main_list")]
pub fn user_main_list(name: String) -> Result<Vec<Developer>, EgoError> {
  info_log_add("user_main_list");
  let users = Developer::list_by_name(&name);
  Ok(users)
}

#[update(name = "user_role_set", guard = "manager_guard")]
#[candid_method(update, rename = "user_role_set")]
pub fn user_role_set(request: UserRoleSetRequest) -> Result<bool, EgoError> {
  info_log_add("user_role_set");
  let ret =
    EgoDevService::user_role_set(&request.user_id, request.is_app_auditor, request.is_manager)?;
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
  info_log_add("admin_app_create");

  let caller = ic_cdk::caller();

  match Developer::get(&caller) {
    None => {
      EgoDevService::developer_main_register(&caller, "ego_deployer")?;
    }
    Some(_) => {
      info_log_add("1. developer exists. skip developer registration");
    }
  }

  info_log_add("2. developer_app_new");
  EgoDevService::developer_app_new(
    &caller,
    &request.app_id,
    &request.name,
    &request.logo,
    &request.description,
    &request.category,
    0f32,
  )?;

  info_log_add("3. app_version_new");
  EgoDevService::app_version_new(&caller, &request.app_id, &request.version)?;

  info_log_add("4. app_version_upload_wasm");
  EgoDevService::app_version_upload_wasm(
    EgoFile::new(),
    &caller,
    &request.app_id,
    &request.version,
    request.backend_data,
    request.backend_data_hash,
  )
    .await?;

  info_log_add("5. app_version_submit");
  EgoDevService::app_version_submit(&caller, &request.app_id, &request.version).expect("app_version_submit should success");

  info_log_add("6. app_version_approve");
  EgoDevService::app_version_approve(&request.app_id).expect("app_version_approve should success");

  info_log_add("7. app_version_release");
  let ego_store_id = canister_get_one("ego_store").unwrap();
  let ego_store = EgoStore::new(ego_store_id);

  EgoDevService::app_version_release(&caller, &request.app_id, &request.version, ego_store)
}

#[update(name = "admin_app_transfer", guard = "owner_guard")]
#[candid_method(update, rename = "admin_app_transfer")]
pub async fn admin_app_transfer(app_id: AppId) -> Result<(), EgoError> {
  info_log_add("admin_app_transfer");

  let caller = ic_cdk::caller();

  match Developer::get(&caller) {
    None => {
      EgoDevService::developer_main_register(&caller, "ego_deployer")?;
    }
    Some(_) => {
      info_log_add("1. developer exists. skip developer registration");
    }
  }

  info_log_add("2. transfer app");

  match EgoDevApp::get(&app_id) {
    None => Err(EgoDevErr::AppNotExists.into()),
    Some(mut ego_dev_app) => {
      info_log_add("3. remove app from previous developer");
      let mut previous_developer = Developer::get(&ego_dev_app.developer_id).expect("developer not exists");
      previous_developer.created_apps.retain(|exists_app_id| app_id != *exists_app_id);
      previous_developer.save();

      info_log_add("4. add app to current developer");
      let mut curr_developer = Developer::get(&caller).expect("developer not exists");
      curr_developer.created_apps.push(app_id);
      curr_developer.save();

      info_log_add("5. update ego_dev_app's developer id to current developer");
      ego_dev_app.developer_id = caller;
      ego_dev_app.save();
      Ok(())
    }
  }
}

/********************  guard  ********************/
#[inline(always)]
pub fn manager_guard() -> Result<(), String> {
  match Developer::get(&caller()) {
    None => {
      trap(&format!("{} unauthorized", api::caller()));
    }
    Some(user) => {
      match user.is_manager {
        true => {
          Ok(())
        }
        false => {
          trap(&format!("{} unauthorized", api::caller()));
        }
      }
    }
  }
}

#[inline(always)]
pub fn auditor_guard() -> Result<(), String> {
  match Developer::get(&caller()) {
    None => {
      trap(&format!("{} unauthorized", api::caller()));
    }
    Some(user) => {
      match user.is_app_auditor {
        true => {
          Ok(())
        }
        false => {
          trap(&format!("{} unauthorized", api::caller()));
        }
      }
    }
  }
}

#[inline(always)]
pub fn developer_guard() -> Result<(), String> {
  match Developer::get(&caller()) {
    None => {
      trap(&format!("{} unauthorized", api::caller()));
    }
    Some(_) => {
      Ok(())
    }
  }
}

/********************  methods for ego_cycle_threshold_get   ********************/
pub fn cycle_threshold_get() -> u128 {
  1_000_000_000_000
}

pub fn runtime_cycle_threshold_get() -> u128 {
  1_000_000_000_000
}
