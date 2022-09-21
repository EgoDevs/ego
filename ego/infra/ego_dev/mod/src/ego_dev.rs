/********************  ego_dev  ********************/
use std::collections::{BTreeMap};
use std::vec;
use ic_types::Principal;

use crate::app::{App};
use crate::developer::Developer;
use crate::file::File;
use crate::types::{EgoDevErr};
use serde::Serialize;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ego_types::app::{AppId, Category};
use ego_types::ego_error::EgoError;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoDev {
  /// created apps
  pub apps: BTreeMap<AppId, App>,

  /// registered ego developers
  pub developers: BTreeMap<Principal, Developer>,

  /// ego_file canisters
  pub ego_files: Vec<File>,
}

impl EgoDev {
  pub fn new() -> Self {
    EgoDev {
      apps: BTreeMap::new(),
      developers: BTreeMap::new(),
      ego_files: Vec::new(),
    }
  }

  pub fn developer_main_get(&self, user_id: Principal) -> Result<&Developer, EgoError> {
    match self.developers.get(&user_id) {
      Some(developer) => Ok(developer),
      None => Err(EgoDevErr::NotADeveloper.into())
    }
  }

  pub fn developer_main_get_mut(&mut self, user_id: Principal) -> Result<&mut Developer, EgoError> {
    match self.developers.get_mut(&user_id) {
      Some(user) => Ok(user),
      None => Err(EgoDevErr::NotADeveloper.into())
    }
  }

  pub fn developer_main_register(&mut self, user_id: Principal, name: String) -> Developer {
    let developer = self.developers.entry(user_id).or_insert(Developer::new(user_id, name));
    developer.clone()
  }

  pub fn developer_app_new(&mut self, user_id: Principal, app_id: AppId, name: String, logo: String, description: String, category: Category, price: f32) -> Result<App, EgoError> {
    if self.apps.contains_key(&app_id) {
      let app = self.apps.get(&app_id).unwrap();

      if app.developer_id == user_id {
        Ok(app.clone())
      } else {
        Err(EgoDevErr::AppExists.into())
      }
    } else {
      let _ = self.developer_main_get(user_id)?;

      let app = App::new(user_id, app_id.clone(), name, logo, description, category, price);
      self.apps.insert(app_id.clone(), app.clone());

      self.developer_main_get_mut(user_id)?.created_apps.push(app_id.clone());

      Ok(app)
    }
  }

  pub fn developer_app_get(&self, user_id: &Principal, app_id: &AppId) -> Result<&App, EgoError> {
    match self.apps.get(app_id) {
      None => {Err(EgoDevErr::AppNotExists.into())}
      Some(app) => {
        if app.developer_id != *user_id {
          Err(EgoDevErr::UnAuthorized.into())
        } else {
          Ok(app)
        }
      }
    }
  }

  pub fn developer_app_get_mut(&mut self, user_id: &Principal, app_id: &AppId) -> Result<&mut App, EgoError> {
    match self.apps.get_mut(app_id) {
      None => {Err(EgoDevErr::AppNotExists.into())}
      Some(app) => {
        if app.developer_id != *user_id {
          Err(EgoDevErr::UnAuthorized.into())
        } else {
          Ok(app)
        }
      }
    }
  }

  pub fn developer_app_list(&self, user_id: Principal) -> Result<Vec<App>, EgoError> {
    let developer = self.developer_main_get(user_id)?;

    let created_apps = developer.created_apps.iter().map(|app_id| self.apps.get(app_id).unwrap().clone()).collect();
    Ok(created_apps)
  }

  pub fn version_wait_for_audit(&self) -> Vec<App> {
    self.apps.iter().filter(|(_app_id, app)| app.audit_version.is_some()).map(|(_app_id, app)| app.clone()).collect()
  }

  pub fn file_get(&mut self) -> Result<Principal, EgoError>{
    if self.ego_files.is_empty() {
      Err(EgoDevErr::NoFile.into())
    } else {
      let file = self.ego_files.iter_mut().min().unwrap();
      file.wasm_count += 1;
      Ok(file.canister_id)
    }
  }

  pub fn admin_file_add(&mut self, file_id: Principal) -> Result<bool, EgoError> {
    let file = File::new(file_id);
    if !self.ego_files.contains(&file) {
      self.ego_files.push(file);
    }

    Ok(true)
  }

  pub fn is_manager(&self, caller: Principal) -> bool {
    match self.developers.get(&caller) {
      Some(user) => user.is_manager,
      None => false
    }
  }

  pub fn is_app_auditor(&self, caller: Principal) -> bool {
    match self.developers.get(&caller) {
      Some(user) => user.is_app_auditor,
      None => false
    }
  }

  pub fn is_app_developer(&self, caller: Principal) -> bool {
    self.developers.contains_key(&caller)
  }
}