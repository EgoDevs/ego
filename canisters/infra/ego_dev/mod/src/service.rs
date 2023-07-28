use candid::Principal;

use ego_types::app::{AppId, Category};
use ego_types::app::EgoError;
use ego_types::app::Version;

use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::types::app_version::{AppVersion, AppVersionStatus};
use crate::types::developer::Developer;
use crate::types::ego_dev_app::EgoDevApp;
use crate::types::ego_file::EgoFile;
use crate::types::EgoDevErr;

pub struct EgoDevService {}

impl EgoDevService {
  pub fn developer_main_register(caller: &Principal, name: &str) -> Result<Developer, EgoError> {
    match Developer::get(caller) {
      None => {
        match Developer::list_by_name(name).is_empty() {
          true => {
            let developer = Developer::new(caller, name);
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
    match EgoDevApp::get_developer_app(caller, app_id)? {
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
    match EgoDevApp::get_developer_app(caller, app_id)? {
      None => {
        Err(EgoDevErr::AppNotExists.into())
      }
      Some(mut ego_dev_app) => {
        let ego_file_canister_id = EgoDevService::ego_file_get()?;
        let app_version = ego_dev_app.version_new(&ego_file_canister_id, &version)?;
        app_version.save();
        ego_dev_app.save();

        Ok(app_version)
      }
    }
  }

  pub async fn app_version_upload_wasm<F: TEgoFile>(
    ego_file: F,
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
    data: Vec<u8>,
    hash: String,
  ) -> Result<bool, EgoError> {
    match EgoDevApp::get_developer_app(caller, app_id)? {
      None => {
        Err(EgoDevErr::AppNotExists.into())
      }
      Some(ego_dev_app) => {
        match ego_dev_app.version_get(version) {
          Some(mut app_version) => {
            if app_version.status == AppVersionStatus::RELEASED {
              Err(EgoDevErr::OperationNotPermitted.into())
            } else {
              app_version.backend_update();
              app_version.save();
              ego_file
                .file_main_write(app_version.wasm.clone().unwrap().canister_id, app_version.wasm.clone().unwrap().fid(), hash, data)
                .await
            }
          }
          None => Err(EgoDevErr::VersionNotExists.into()),
        }
      }
    }
  }

  pub fn app_version_set_frontend_address(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
    canister_id: &Principal,
  ) -> Result<bool, EgoError> {
    match EgoDevApp::get_developer_app(caller, app_id)? {
      None => {
        Err(EgoDevErr::AppNotExists.into())
      }
      Some(ego_dev_app) => {
        match ego_dev_app.version_get(version) {
          Some(mut app_version) => {
            app_version.frontend_update(canister_id);
            app_version.save();
            Ok(true)
          }
          None => Err(EgoDevErr::VersionNotExists.into()),
        }
      }
    }
  }

  pub fn app_version_submit(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
  ) -> Result<AppVersion, EgoError> {
    match EgoDevApp::get_developer_app(caller, app_id)? {
      None => {
        Err(EgoDevErr::AppNotExists.into())
      }
      Some(mut ego_dev_app) => {
        let app_version = ego_dev_app.version_submit(version)?;
        app_version.save();
        ego_dev_app.save();
        Ok(app_version)
      }
    }
  }

  pub fn app_version_revoke(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
  ) -> Result<AppVersion, EgoError> {
    match EgoDevApp::get_developer_app(caller, app_id)? {
      None => {
        Err(EgoDevErr::AppNotExists.into())
      }
      Some(mut ego_dev_app) => {
        let app_version = ego_dev_app.version_revoke(version)?;
        app_version.save();
        ego_dev_app.save();
        Ok(app_version)
      }
    }
  }

  pub fn app_version_release<S: TEgoStore>(
    caller: &Principal,
    app_id: &AppId,
    version: &Version,
    ego_store: S,
  ) -> Result<AppVersion, EgoError> {
    match EgoDevApp::get_developer_app(caller, app_id)? {
      None => {
        Err(EgoDevErr::AppNotExists.into())
      }
      Some(mut ego_dev_app) => {
        let app_version = ego_dev_app.version_release(version)?;
        app_version.save();

        ego_dev_app.app.app_hash_update();
        ego_dev_app.save();

        ego_store.app_main_release(ego_dev_app.app, app_version.clone().wasm.unwrap());

        Ok(app_version)
      }
    }
  }

  pub fn app_version_approve(app_id: &AppId, version: &Version) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::get(app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;
    let app_version = ego_dev_app.version_approve(version)?;
    app_version.save();

    ego_dev_app.save();
    Ok(app_version)
  }

  pub fn app_version_reject(app_id: &AppId, version: &Version) -> Result<AppVersion, EgoError> {
    let mut ego_dev_app = EgoDevApp::get(app_id).ok_or(EgoError::from(EgoDevErr::AppNotExists))?;
    let app_version = ego_dev_app.version_reject(version)?;
    app_version.save();

    ego_dev_app.save();
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
    let ego_file = EgoFile::new(file_id);
    ego_file.save();
  }

  pub fn ego_file_get() -> Result<Principal, EgoError> {
    let ego_files = EgoFile::list();

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
