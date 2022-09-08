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
    user_id: Principal,
    name: String,
  ) -> Result<Developer, EgoError> {
    APP_STORE.with(|app_store| {
      let developer =
        app_store
          .borrow_mut()
          .developer_main_register(user_id, name);
      Ok(developer)
    })
  }

  pub fn developer_main_get(user_id: Principal) -> Result<Developer, EgoError> {
    APP_STORE.with(|app_store|
      match app_store.borrow().developer_get(user_id) {
        Ok(developer) => { Ok(developer.clone()) }
        Err(e) => { Err(e) }
      }
    )
  }

  pub fn developer_app_list(user_id: Principal) -> Result<Vec<App>, EgoError> {
    APP_STORE.with(|app_store| app_store.borrow().created_apps(user_id))
  }

  pub fn developer_app_get(caller: Principal, app_id: AppId) -> Result<App, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            Err(EgoDevErr::UnAuthorized.into())
          } else {
            Ok(app.clone())
          }
        }
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
  }

  pub fn developer_app_new(
    user_id: Principal,
    app_id: AppId,
    name: String,
    category: Category,
    price: f32,
  ) -> Result<App, EgoError> {
    let result = APP_STORE.with(|store| match store.borrow().app_get(&app_id) {
      Some(app) => {
        if app.developer_id == user_id {
          Ok(Some(app.clone()))
        } else {
          Err(EgoError::from(EgoDevErr::AppExists))
        }
      }
      None => Ok(None),
    });

    match result {
      Ok(Some(app)) => Ok(app),
      Ok(None) => {
        let app = APP_STORE.with(|app_store| {
          app_store.borrow_mut().app_new(
            user_id,
            app_id.clone(),
            name,
            category,
            price,
          )
        })?;

        Ok(app)
      }
      Err(e) => Err(e)
    }
  }

  pub async fn app_version_new(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            Err(EgoDevErr::UnAuthorized.into())
          } else {
            match app.new_version(version) {
              Ok(app_version) => Ok(app_version),
              Err(err) => Err(EgoError::from(err)),
            }
          }
        }
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
  }

  pub fn app_version_set_frontend_address(
    caller: Principal,
    app_id: AppId,
    version: Version,
    canister_id: Principal,
  ) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            Err(EgoError::from(EgoDevErr::UnAuthorized))
          } else {
            match app.get_version_mut(version) {
              Some(app_version) => {
                app_version.set_frontend_address(canister_id);
                Ok(true)
              }
              None => Err(EgoError::from(EgoDevErr::VersionNotExists)),
            }
          }
        }
        None => Err(EgoError::from(EgoDevErr::AppNotExists)),
      },
    )
  }


  pub async fn app_version_submit(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<bool, EgoError> {
    let frontend_canister_id =
      APP_STORE.with(|app_store| match app_store.borrow_mut().app_get(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            Err(EgoError::from(EgoDevErr::UnAuthorized))
          } else {
            match app.get_version(version) {
              Some(app_version) => Ok(app_version.get_frontend_address()),
              None => Err(EgoError::from(EgoDevErr::VersionNotExists)),
            }
          }
        }
        None => Err(EgoError::from(EgoDevErr::AppNotExists)),
      })?;

    ic_cdk::println!(
      "1. check is it has frontend address {}",
      frontend_canister_id.is_some()
    );
    if frontend_canister_id.is_some() {
      // upgrade_to_ego_assets(&frontend_canister_id.unwrap()).await?;
      // drain_authorize(frontend_canister_id.unwrap()).await?;
    }

    ic_cdk::println!("2. submit version");
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            Err(EgoDevErr::UnAuthorized.into())
          } else {
            match app.submit_version(version) {
              Ok(ret) => Ok(ret),
              Err(e) => Err(e),
            }
          }
        }
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
  }

  pub fn app_version_revoke(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            Err(EgoDevErr::UnAuthorized.into())
          } else {
            app.revoke_version(version)
          }
        }
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
  }

  pub async fn app_version_release(
    caller: Principal,
    app_id: AppId,
    version: Version,
  ) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => {
          if caller != app.developer_id {
            return Err(EgoDevErr::UnAuthorized.into());
          } else {
            match app.release_version(version) {
              Ok(_) => Ok(true),
              Err(err) => Err(err),
            }
          }
        }
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
  }


  pub fn app_version_wait_for_audit() -> Vec<App> {
    APP_STORE.with(|app_store| app_store.borrow().wait_for_audit_apps())
  }

  pub fn app_version_approve(app_id: AppId, version: Version) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => app.approve_version(version),
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
  }

  pub fn app_version_reject(app_id: AppId, version: Version) -> Result<bool, EgoError> {
    APP_STORE.with(
      |app_store| match app_store.borrow_mut().app_get_mut(&app_id) {
        Some(app) => app.reject_version(version),
        None => Err(EgoDevErr::AppNotExists.into()),
      },
    )
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
