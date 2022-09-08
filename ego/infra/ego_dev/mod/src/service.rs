use ic_cdk::{api, trap};
use ic_types::Principal;

use ego_utils::types::{EgoError, Version};

use crate::app::*;
use crate::developer::Developer;
use crate::state::APP_STORE;
use crate::types::*;

pub struct EgoDevService {}

impl EgoDevService {
  pub fn developer_main_register(
    caller: Principal,
    name: String,
  ) -> Result<Developer, EgoError> {
    APP_STORE.with(|app_store| {
      let developer =
        app_store
          .borrow_mut()
          .developer_register(caller, name);
      Ok(developer)
    })
  }

  pub fn developer_main_get(caller: Principal) -> Result<Developer, EgoError> {
    APP_STORE.with(|app_store|
      match app_store.borrow().developer_get(caller) {
        Ok(developer) => { Ok(developer.clone()) }
        Err(e) => { Err(e) }
      }
    )
  }

  pub fn developer_app_list(caller: Principal) -> Result<Vec<App>, EgoError> {
    APP_STORE.with(|app_store| app_store.borrow().app_list(caller))
  }

  pub fn developer_app_get(caller: Principal, app_id: AppId) -> Result<App, EgoError> {
    APP_STORE.with(
      |app_store| {
        match app_store.borrow().developer_app_get(&caller, &app_id) {
          Ok(app) => Ok(app.clone()),
          Err(e) => Err(e)
        }
      })
  }

  pub fn developer_app_new(
    caller: Principal,
    app_id: AppId,
    name: String,
    category: Category,
    price: f32,
  ) -> Result<App, EgoError> {
    APP_STORE.with(|store| store.borrow_mut().app_new(caller, app_id.clone(), name, category, price))
  }

  pub fn app_version_new(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    APP_STORE.with(|app_store| {
      let _ = app_store.borrow().developer_app_get(&caller, &app_id)?;
      app_store.borrow_mut().app_get_mut(&app_id).unwrap().version_new(version)
    })
  }

  pub fn app_version_set_frontend_address(
    caller: Principal,
    app_id: AppId,
    version: Version,
    canister_id: Principal,
  ) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| {
        let _ = app_store.borrow_mut().developer_app_get(&caller, &app_id)?;
        match app_store.borrow_mut().app_get_mut(&app_id) {
          Ok(app) => {
            match app.version_get_mut(version) {
              Some(app_version) => {
                app_version.set_frontend_address(canister_id);
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
    //   APP_STORE.with(|app_store| match app_store.borrow_mut().app_get(&app_id) {
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

    APP_STORE.with(|app_store| {
      let _ = app_store.borrow().developer_app_get(&caller, &app_id)?;
      app_store.borrow_mut().app_get_mut(&app_id).unwrap().version_submit(version)
    })
  }

  pub fn app_version_revoke(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    APP_STORE.with(|app_store| {
      let _ = app_store.borrow().developer_app_get(&caller, &app_id)?;
      app_store.borrow_mut().app_get_mut(&app_id).unwrap().version_revoke(version)
    })
  }

  pub fn app_version_release(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    APP_STORE.with(
      |app_store| {
        let _ = app_store.borrow().developer_app_get(&caller, &app_id)?;
        app_store.borrow_mut().app_get_mut(&app_id).unwrap().release_version(version)
      },
    )
  }

  pub fn app_version_wait_for_audit() -> Vec<App> {
    APP_STORE.with(|app_store| app_store.borrow().version_wait_for_audit())
  }

  pub fn app_version_approve(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    APP_STORE.with(|app_store| {
      match app_store.borrow_mut().app_get_mut(&app_id) {
        Ok(app) => app.version_approve(version),
        Err(e) => Err(e)
      }
    })
  }

  pub fn app_version_reject(app_id: AppId, version: Version) -> Result<AppVersion, EgoError> {
    APP_STORE.with(|app_store| {
      match app_store.borrow_mut().app_get_mut(&app_id) {
        Ok(app) => app.version_reject(version),
        Err(e) => Err(e)
      }
    })
  }

  pub fn user_role_set(
    user_id: Principal,
    is_app_auditer: bool,
    is_manager: bool,
  ) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().developer_get_mut(user_id) {
        Ok(user) => {
          user.is_app_auditer = is_app_auditer;
          user.is_manager = is_manager;
          Ok(true)
        }
        Err(e) => Err(e),
      },
    )
  }

  pub fn user_main_list(name: String) -> Vec<Developer> {
    APP_STORE.with(|app_store| {
      app_store
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
}


/********************  guard methods  ********************/
#[inline(always)]
pub fn manager_guard() -> Result<(), String> {
  if APP_STORE.with(|app_store| app_store.borrow().is_manager(api::caller())) {
    Ok(())
  } else {
    trap(&format!("{} unauthorized", api::caller()));
  }
}

#[inline(always)]
pub fn auditer_guard() -> Result<(), String> {
  if APP_STORE.with(|app_store| app_store.borrow().is_app_auditer(api::caller())) {
    Ok(())
  } else {
    trap(&format!("{} unauthorized", api::caller()));
  }
}

#[inline(always)]
pub fn developer_guard() -> Result<(), String> {
  if APP_STORE.with(|app_store| app_store.borrow().is_app_developer(api::caller())) {
    Ok(())
  } else {
    trap(&format!("{} unauthorized", api::caller()));
  }
}
