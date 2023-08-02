use candid::Principal;

use ego_types::app::{AppId, Category};
use ego_types::app::EgoError;
use ego_types::app::Version;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::types::app_version::{AppVersion, AppVersionStatus};
use crate::types::developer::Developer;
use crate::types::ego_dev_app::EgoDevApp;
use crate::types::EgoDevErr;
use crate::types::file::File;

pub struct EgoDevService {}

impl EgoDevService {
  pub fn developer_main_register(caller: &Principal, name: &str) -> Result<Developer, EgoError> {
    match Developer::get(caller) {
      None => {
        match Developer::list_by_name(name).is_empty() {
          true => {
            let mut developer = Developer::new(caller, name);
            developer.save();
            Ok(developer)
          }
          false => {
            Err(EgoDevErr::UserExists.into())
          }
        }
      }
      Some(developer) => Ok(developer),
    }
  }

  pub fn developer_app_transfer(developer_id: &Principal, app_id: &AppId) -> Result<(), EgoError> {
    match EgoDevApp::get(app_id) {
      None => Err(EgoDevErr::AppNotExists.into()),
      Some(mut ego_dev_app) => {
        ego_dev_app.developer_id = developer_id.clone();
        ego_dev_app.save();

        Ok(())
      }
    }
  }

  pub fn developer_app_new(
    caller: &Principal,
    app_id: &AppId,
    name: &str,
    logo: &str,
    description: &str,
    category: &Category,
    price: f32,
  ) -> Result<EgoDevApp, EgoError> {
    match EgoDevApp::by_developer_id_and_id(caller, app_id) {
      None => {
        match EgoDevApp::get(app_id) {
          None => {
            let mut developer = Developer::get(caller).expect("developer not exists");

            let mut ego_dev_app = EgoDevApp::new(
              caller,
              app_id,
              name,
              logo,
              description,
              category,
              price,
            );
            ego_dev_app.save();

            developer.created_apps.push(app_id.clone());
            developer.save();

            Ok(ego_dev_app)
          }
          Some(_) => {
            Err(EgoDevErr::AppExists.into())
          }
        }
      }
      Some(ego_dev_app) => {
        Ok(ego_dev_app)
      }
    }
  }

  pub fn app_version_new(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
  ) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::by_developer_id_and_id(caller, app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;

    let ego_file_canister_id = EgoDevService::ego_file_get()?;
    let app_version = ego_dev_app.version_new(&ego_file_canister_id, &version)?;

    Ok(app_version)
  }

  pub async fn app_version_upload_wasm<F: TEgoFile>(
    ego_file: F,
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
    data: Vec<u8>,
    hash: String,
  ) -> Result<bool, EgoError> {
    let ego_dev_app = EgoDevApp::by_developer_id_and_id(caller, app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;

    match ego_dev_app.version_get(version) {
      Some(mut app_version) => {
        if app_version.status == AppVersionStatus::RELEASED {
          Err(EgoDevErr::OperationNotPermitted.into())
        } else {
          app_version.backend_update();
          ego_file
            .file_main_write(app_version.wasm.clone().unwrap().canister_id, app_version.wasm.clone().unwrap().fid(), hash, data)
            .await
        }
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn app_version_set_frontend_address(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
    canister_id: &Principal,
  ) -> Result<bool, EgoError> {
    let ego_dev_app = EgoDevApp::by_developer_id_and_id(caller, app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;

    match ego_dev_app.version_get(version) {
      Some(mut app_version) => {
        app_version.frontend_update(canister_id);
        Ok(true)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn app_version_submit(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
  ) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::by_developer_id_and_id(caller, app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;

    let app_version = ego_dev_app.version_submit(version)?;
    Ok(app_version)
  }

  pub fn app_version_revoke(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
  ) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::by_developer_id_and_id(caller, app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;

    let app_version = ego_dev_app.version_revoke(version)?;
    Ok(app_version)
  }

  pub fn app_version_release<S: TEgoStore>(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
    ego_store: S,
  ) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::by_developer_id_and_id(caller, app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;

    let app_version = ego_dev_app.version_release(version)?;

    ego_dev_app.app.app_hash_update();
    ego_dev_app.save();

    ego_store.app_main_release(ego_dev_app.app, app_version.clone().wasm.unwrap());

    Ok(app_version)
  }

  pub fn app_version_approve(app_id: &AppId) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::get(app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;
    let app_version = ego_dev_app.version_approve()?;

    Ok(app_version)
  }

  pub fn app_version_reject(app_id: &AppId) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::get(app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;
    let app_version = ego_dev_app.version_reject()?;

    Ok(app_version)
  }

  pub fn user_role_set(
    user_id: &Principal,
    is_app_auditer: bool,
    is_manager: bool,
  ) -> Result<bool, EgoError> {
    let mut developer = Developer::get(user_id).ok_or(EgoError::from(EgoDevErr::NotADeveloper))?;
    developer.is_app_auditor = is_app_auditer;
    developer.is_manager = is_manager;
    developer.save();
    Ok(true)
  }

  pub fn admin_ego_file_add(file_id: &Principal) {
    let ego_file = File::new(file_id);
    ego_file.save();
  }

  pub fn ego_file_get() -> Result<Principal, EgoError> {
    let ego_files = File::list();

    if ego_files.is_empty() {
      Err(EgoDevErr::NoFile.into())
    } else {
      let mut file = ego_files.iter().min().unwrap().clone();
      file.wasm_count += 1;
      file.save();
      Ok(file.canister_id)
    }
  }
}
