use ic_cdk::export::Principal;

use ego_types::app::{AppId, Category};
use ego_types::app::EgoError;
use ego_types::app::Version;

use crate::app::*;
use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::developer::Developer;
use crate::state::EGO_DEV;
use crate::types::*;

pub struct EgoDevService {}

impl EgoDevService {
  pub fn developer_main_register(caller: Principal, name: String) -> Result<Developer, EgoError> {
    EGO_DEV.with(|ego_dev| ego_dev.borrow_mut().developer_main_register(caller, name))
  }

  pub fn developer_main_get(caller: Principal) -> Result<Developer, EgoError> {
    EGO_DEV.with(
      |ego_dev| match ego_dev.borrow().developer_main_get(caller) {
        Ok(developer) => Ok(developer.clone()),
        Err(e) => Err(e),
      },
    )
  }

  pub fn developer_app_list(caller: Principal) -> Result<Vec<EgoDevApp>, EgoError> {
    EGO_DEV.with(|ego_dev| ego_dev.borrow().developer_app_list(caller))
  }

  pub fn developer_app_get(caller: Principal, app_id: AppId) -> Result<EgoDevApp, EgoError> {
    EGO_DEV.with(
      |ego_dev| match ego_dev.borrow().developer_app_get(&caller, &app_id) {
        Ok(app) => Ok(app.clone()),
        Err(e) => Err(e),
      },
    )
  }

  pub fn developer_app_new(
    caller: Principal,
    app_id: AppId,
    name: String,
    logo: String,
    description: String,
    category: Category,
    price: f32,
  ) -> Result<EgoDevApp, EgoError> {
    EGO_DEV.with(|ego_dev| {
      ego_dev.borrow_mut().developer_app_new(
        caller,
        app_id.clone(),
        name,
        logo,
        description,
        category,
        price,
      )
    })
  }

  pub fn app_version_new(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| {
      let mut ego_borrow = ego_dev.borrow_mut();
      let ego_file_canister_id = ego_borrow.file_get()?;
      match ego_borrow.developer_app_get_mut(&caller, &app_id) {
        Ok(app) => app.version_new(ego_file_canister_id, version),
        Err(e) => Err(e),
      }
    })
  }

  pub async fn app_version_upload_wasm<F: TEgoFile>(
    ego_dev: F,
    caller: Principal,
    app_id: AppId,
    version: Version,
    data: Vec<u8>,
    hash: String,
  ) -> Result<bool, EgoError> {
    match EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id) {
        Ok(app) => match app.version_get_mut(version) {
          Some(app_version) => {
            if app_version.status == AppVersionStatus::RELEASED {
              Err(EgoDevErr::OperationNotPermitted.into())
            } else {
              Ok(app_version.backend_update())
            }
          }
          None => Err(EgoDevErr::VersionNotExists.into()),
        },
        Err(e) => Err(e),
      }
    }) {
      Ok(wasm) => {
        ego_dev
          .file_main_write(wasm.canister_id, wasm.fid(), hash, data)
          .await
      }
      Err(e) => Err(e),
    }
  }

  pub fn app_version_set_frontend_address(
    caller: Principal,
    app_id: AppId,
    version: Version,
    canister_id: Principal,
  ) -> Result<bool, EgoError> {
    EGO_DEV.with(
      |ego_dev| match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id) {
        Ok(app) => match app.version_get_mut(version) {
          Some(app_version) => {
            app_version.frontend_update(canister_id);
            Ok(true)
          }
          None => Err(EgoError::from(EgoDevErr::VersionNotExists)),
        },
        Err(e) => Err(e),
      },
    )
  }

  pub fn app_version_submit(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(
      |ego_dev| match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id) {
        Ok(app) => app.version_submit(version),
        Err(e) => Err(e),
      },
    )
  }

  pub fn app_version_revoke(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(
      |ego_dev| match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id) {
        Ok(app) => app.version_revoke(version),
        Err(e) => Err(e),
      },
    )
  }

  pub fn app_version_release<S: TEgoStore>(
    caller: Principal,
    app_id: AppId,
    version: Version,
    ego_store: S,
  ) -> Result<AppVersion, EgoError> {
    let app_version = EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id) {
        Ok(app) => app.version_release(version),
        Err(e) => Err(e),
      }
    })?;

    let ego_dev_app = EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow_mut().developer_app_get(&caller, &app_id) {
        Ok(ego_dev_app) => Ok(ego_dev_app.clone()),
        Err(e) => Err(e),
      }
    })?;

    let mut app = ego_dev_app.app;
    app.app_hash_update();

    ego_store
      .app_main_release(app, app_version.clone().wasm.unwrap());

    Ok(app_version)
  }

  pub fn app_version_wait_for_audit() -> Vec<EgoDevApp> {
    EGO_DEV.with(|ego_dev| ego_dev.borrow().version_wait_for_audit())
  }

  pub fn app_version_approve(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| match ego_dev.borrow_mut().apps.get_mut(&app_id) {
      Some(app) => app.version_approve(version),
      None => Err(EgoDevErr::AppNotExists.into()),
    })
  }

  pub fn app_version_reject(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| match ego_dev.borrow_mut().apps.get_mut(&app_id) {
      Some(app) => app.version_reject(version),
      None => Err(EgoDevErr::AppNotExists.into()),
    })
  }

  pub fn user_role_set(
    user_id: Principal,
    is_app_auditer: bool,
    is_manager: bool,
  ) -> Result<bool, EgoError> {
    EGO_DEV.with(
      |ego_dev| match ego_dev.borrow_mut().developer_main_get_mut(user_id) {
        Ok(user) => {
          user.is_app_auditor = is_app_auditer;
          user.is_manager = is_manager;
          Ok(true)
        }
        Err(e) => Err(e),
      },
    )
  }

  pub fn user_main_list(name: String) -> Vec<Developer> {
    EGO_DEV.with(|ego_dev| {
      ego_dev
        .borrow()
        .developers
        .values()
        .filter_map(|user| {
          if user.name == name {
            Some(user.clone())
          } else {
            None
          }
        })
        .collect()
    })
  }

  pub fn admin_ego_file_add(file_id: Principal) {
    EGO_DEV.with(|ego_dev| ego_dev.borrow_mut().admin_file_add(file_id))
  }
}
