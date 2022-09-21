use ic_types::Principal;
use ego_types::app::{AppId, Category};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;

use crate::app::*;
use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ego_store::TEgoStore;
use crate::developer::Developer;
use crate::state::{EGO_DEV, EGO_STORE_CANISTER_ID};
use crate::types::*;

pub struct EgoDevService {}

impl EgoDevService {
  pub fn developer_main_register(
    caller: Principal,
    name: String,
  ) -> Result<Developer, EgoError> {
    EGO_DEV.with(|ego_dev| {
      let developer =
        ego_dev.borrow_mut()
          .developer_main_register(caller, name);
      Ok(developer)
    })
  }

  pub fn developer_main_get(caller: Principal) -> Result<Developer, EgoError> {
    EGO_DEV.with(|ego_dev|
      match ego_dev.borrow().developer_main_get(caller) {
        Ok(developer) => { Ok(developer.clone()) }
        Err(e) => { Err(e) }
      }
    )
  }

  pub fn developer_app_list(caller: Principal) -> Result<Vec<App>, EgoError> {
    EGO_DEV.with(|ego_dev| ego_dev.borrow().developer_app_list(caller))
  }

  pub fn developer_app_get(caller: Principal, app_id: AppId) -> Result<App, EgoError> {
    EGO_DEV.with(
      |ego_dev| {
        match ego_dev.borrow().developer_app_get(&caller, &app_id) {
          Ok(app) => Ok(app.clone()),
          Err(e) => Err(e)
        }
      })
  }

  pub fn developer_app_new(
    caller: Principal,
    app_id: AppId,
    name: String,
    logo: String,
    description: String,
    category: Category,
    price: f32,
  ) -> Result<App, EgoError> {
    EGO_DEV.with(|ego_dev| ego_dev.borrow_mut().developer_app_new(caller, app_id.clone(), name, logo, description, category, price))
  }

  pub fn app_version_new(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| {
      let mut ego_borrow = ego_dev.borrow_mut();
      let file_id = ego_borrow.file_get()?;
      match ego_borrow.developer_app_get_mut(&caller, &app_id) {
        Ok(app) => {app.version_new(file_id, version)}
        Err(e) => {Err(e)}
      }

    })
  }

  pub async fn app_version_upload_wasm<F: TEgoFile>(ego_dev: F, caller: Principal, app_id: AppId, version: Version, data: Vec<u8>, hash: String) -> Result<bool, EgoError> {
    match EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow().developer_app_get(&caller, &app_id){
        Ok(app) => {
          match app.version_get(version){
            Some(app_version) => {
              if app_version.status == AppVersionStatus::RELEASED {
                Err(EgoDevErr::OperationNotPermitted.into())
              } else {
                Ok(app_version.backend.clone())
              }
            },
            None => Err(EgoDevErr::VersionNotExists.into())
          }
        }
        Err(e) => Err(e)
      }
    }) {
      Ok(wasm) => {
        ego_dev.file_main_write(wasm.canister_id.unwrap(), wasm.fid(), hash, data).await
      },
      Err(e) => Err(e)
    }
  }

  pub fn app_version_set_frontend_address(
    caller: Principal,
    app_id: AppId,
    version: Version,
    canister_id: Principal,
  ) -> Result<bool, EgoError> {
    EGO_DEV.with(
      |ego_dev| {
        match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id){
          Ok(app) => {
            match app.version_get_mut(version) {
              Some(app_version) => {
                app_version.frontend.canister_id = Some(canister_id);
                Ok(true)
              }
              None => Err(EgoError::from(EgoDevErr::VersionNotExists)),
            }
          }
          Err(e) => Err(e)
        }
      },
    )
  }


  pub fn app_version_submit(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    // let frontend_canister_id =
    //   EGO_DEV.with(|ego_dev| match ego_dev.borrow_mut().app_get(&app_id) {
    //     Some(app) => {
    //       if caller != app.developer_id {
    //         Err(EgoError::from(EgoDevErr::UnAuthorized))
    //       } else {
    //         match app.get_version(version) {
    //           Some(app_version) => Ok(app_version.get_frontend_address()),
    //           None => Err(EgoError::from(EgoDevErr::VersionNotExists)),
    //         }
    //       }
    //     }
    //     None => Err(EgoError::from(EgoDevErr::AppNotExists)),
    //   })?;
    //
    // ic_cdk::println!(
    //   "1. check is it has frontend address {}",
    //   frontend_canister_id.is_some()
    // );
    // if frontend_canister_id.is_some() {
    //   // upgrade_to_ego_assets(&frontend_canister_id.unwrap()).await?;
    //   // drain_authorize(frontend_canister_id.unwrap()).await?;
    // }

    EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id){
        Ok(app) => {
          app.version_submit(version)
        }
        Err(e) => {Err(e)}
      }
    })
  }

  pub fn app_version_revoke(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id){
        Ok(app) => {
          app.version_revoke(version)
        }
        Err(e) => {Err(e)}
      }
    })
  }

  pub async fn app_version_release<S: TEgoStore>(
    caller: Principal,
    app_id: AppId,
    version: Version,
    ego_store: S
  ) -> Result<AppVersion, EgoError> {
    let ego_store_canister_id = EGO_STORE_CANISTER_ID.with(|rc| rc.borrow().unwrap());

    let result = EGO_DEV.with(
      |ego_dev| {
        match ego_dev.borrow_mut().developer_app_get_mut(&caller, &app_id){
          Ok(app) => {
            app.version_release(version)
          }
          Err(e) => {Err(e)}
        }
      },
    );

    let app = EGO_DEV.with(|ego_dev| { ego_dev.borrow_mut().developer_app_get(&caller, &app_id).unwrap().clone()});

    ego_store.app_main_release(ego_store_canister_id, app).await?;

    result
  }

  pub fn app_version_wait_for_audit() -> Vec<App> {
    EGO_DEV.with(|ego_dev| ego_dev.borrow().version_wait_for_audit())
  }

  pub fn app_version_approve(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| {

      match ego_dev.borrow_mut().apps.get_mut(&app_id) {
        Some(app) => app.version_approve(version),
        None => Err(EgoDevErr::VersionNotExists.into())
      }
    })
  }

  pub fn app_version_reject(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    EGO_DEV.with(|ego_dev| {
      match ego_dev.borrow_mut().apps.get_mut(&app_id) {
        Some(app) => app.version_reject(version),
        None => Err(EgoDevErr::VersionNotExists.into())
      }
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
      ego_dev.borrow()
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

  pub fn admin_ego_file_add(file_id: Principal) -> Result<bool, EgoError> {
    EGO_DEV.with(|ego_dev| {
      ego_dev.borrow_mut()
        .admin_file_add(file_id)
    })
  }
}
