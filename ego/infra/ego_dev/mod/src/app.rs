use std::collections::BTreeMap;
use std::fmt;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

use ego_utils::types::{EgoError, Version};

use crate::app::CanisterType::{ASSET, BACKEND};
use crate::developer::*;
use crate::types::*;

pub type AppId = String;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Category {
  System,
  Vault,
}


/********************  app store  ********************/
#[derive(CandidType, Deserialize, Serialize)]
pub struct AppStore {
  pub apps: BTreeMap<AppId, App>,
  pub developers: BTreeMap<Principal, Developer>,
  pub files: BTreeMap<Principal, File>,
}

impl AppStore {
  pub fn new() -> Self {
    AppStore {
      apps: BTreeMap::new(),
      developers: BTreeMap::new(),
      files: BTreeMap::new(),
    }
  }

  pub fn app_get(&self, app_id: &AppId) -> Option<&App> {
    self.apps.get(app_id)
  }

  pub fn app_get_mut(&mut self, app_id: &AppId) -> Option<&mut App> {
    self.apps.get_mut(app_id)
  }

  pub fn developer_get(&self, user_id: Principal) -> Result<&Developer, EgoError> {
    match self.developers.get(&user_id) {
      Some(developer) => Ok(developer),
      None => Err(EgoDevErr::NotADeveloper.into())
    }
  }

  pub fn developer_get_mut(&mut self, user_id: Principal) -> Result<&mut Developer, EgoError> {
    match self.developers.get_mut(&user_id) {
      Some(user) => Ok(user),
      None => Err(EgoDevErr::NotADeveloper.into())
    }
  }

  pub fn developer_main_register(&mut self, user_id: Principal, name: String) -> Developer {
    let developer = self.developers.entry(user_id).or_insert(Developer::new(user_id, name));
    developer.clone()
  }

  pub fn app_new(&mut self, user_id: Principal, app_id: AppId, name: String, category: Category, price: f32) -> Result<App, EgoError> {
    if self.apps.contains_key(&app_id) {
      Ok(self.apps.get(&app_id).unwrap().clone())
    } else {
      let _ = self.developer_get(user_id)?;

      let app = App::new(user_id, app_id.clone(), name, category, self.get_file_id(), price);
      self.apps.insert(app_id.clone(), app.clone());

      self.developer_get_mut(user_id)?.created_apps.push(app_id.clone());

      Ok(app)
    }
  }

  pub fn created_apps(&self, user_id: Principal) -> Result<Vec<App>, EgoError> {
    let developer = self.developer_get(user_id)?;

    let created_apps = developer.created_apps.iter().map(|app_id| self.apps.get(app_id).unwrap().clone()).collect();
    Ok(created_apps)
  }

  pub fn is_manager(&self, caller: Principal) -> bool {
    match self.developers.get(&caller) {
      Some(user) => user.is_manager,
      None => false
    }
  }

  pub fn is_app_auditer(&self, caller: Principal) -> bool {
    match self.developers.get(&caller) {
      Some(user) => user.is_app_auditer,
      None => false
    }
  }

  pub fn is_app_developer(&self, caller: Principal) -> bool {
    self.developers.contains_key(&caller)
  }

  pub fn wait_for_audit_apps(&self) -> Vec<App> {
    self.apps.iter().filter(|(_app_id, app)| app.audit_version.is_some()).map(|(_app_id, app)| app.clone()).collect()
  }

  fn get_file_id(&self) -> Principal {
    // TODO: replace with actual code
    Principal::from_text("227b5-saaaa-aaaan-qaqeq-cai").unwrap()
  }
}

/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct App {
  pub app_id: AppId,
  pub developer_id: Principal,
  pub category: Category,
  pub name: String,
  pub status: AppStatus,
  pub file_id: Principal,
  pub release_version: Option<Version>,
  pub audit_version: Option<Version>,
  pub versions: Vec<AppVersion>,
  pub price: f32,
}


impl App {
  pub fn to_string(&self) -> String {
    format!("app_id: {:?},user_id:{:?},category:{:?},release_version:{:?},versions:{:?},",
            self.app_id, self.developer_id.to_text(), self.category, self.release_version, self.versions)
  }
}

#[derive(
CandidType, Serialize, Deserialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq,
)]
pub enum AppStatus {
  NEW,
  RELEASED,
  CLOSED,
}

impl App {
  pub fn new(user_id: Principal, app_id: AppId, name: String, category: Category, file_id: Principal, price: f32) -> Self {
    App {
      app_id,
      developer_id: user_id,
      name,
      category,
      file_id,
      status: AppStatus::NEW,
      release_version: None,
      audit_version: None,
      versions: vec![],
      price,
    }
  }

  pub fn get_version(&self, version: Version) -> Option<&AppVersion> {
    self.versions.iter().find(|app_version| app_version.version == version)
  }

  pub fn get_version_mut(&mut self, version: Version) -> Option<&mut AppVersion> {
    self.versions.iter_mut().find(|app_version| app_version.version == version)
  }

  pub fn new_version(&mut self, version: Version) -> Result<AppVersion, EgoError> {
    match self.get_version(version) {
      Some(_) => {
        Err(EgoDevErr::VersionExists.into())
      }
      None => {
        let app_version = AppVersion::new(self.app_id.clone(), self.file_id, version);
        self.versions.push(app_version.clone());
        Ok(app_version)
      }
    }
  }

  pub fn submit_version(&mut self, version: Version) -> Result<bool, EgoError> {
    match self.get_version_mut(version) {
      Some(app_ver) => {
        app_ver.submit();
        self.audit_version = Some(version);
        Ok(true)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn revoke_version(&mut self, version: Version) -> Result<bool, EgoError> {
    match self.get_version_mut(version) {
      Some(app_ver) => {
        app_ver.revoke();
        self.audit_version = None;
        Ok(true)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn release_version(&mut self, version: Version) -> Result<bool, EgoError> {
    match self.get_version_mut(version) {
      Some(app_ver) => {
        app_ver.release();
        self.release_version = Some(version);
        Ok(true)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn approve_version(&mut self, version: Version) -> Result<bool, EgoError> {
    match self.get_version_mut(version) {
      Some(app_ver) => {
        app_ver.approve();
        self.audit_version = None;
        Ok(true)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn reject_version(&mut self, version: Version) -> Result<bool, EgoError> {
    match self.get_version_mut(version) {
      Some(app_ver) => {
        app_ver.reject();
        self.audit_version = None;
        Ok(true)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn find_wasms(&self, version: Version) -> Result<&Vec<Wasm>, EgoError> {
    match self.get_version(version) {
      Some(app_ver) => {
        Ok(&app_ver.wasms)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn find_release_wasms(&self) -> Result<&Vec<Wasm>, EgoError> {
    self.find_wasms(self.release_version.unwrap())
  }
}

/********************  app version  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppVersion {
  pub app_id: AppId,
  pub version: Version,
  pub status: AppVersionStatus,
  pub wasms: Vec<Wasm>,
  pub file_id: Principal,
}

#[derive(
CandidType, Serialize, Deserialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq,
)]
pub enum AppVersionStatus {
  NEW,
  SUBMITTED,
  REJECTED,
  APPROVED,
  RELEASED,
  REVOKED,
}

impl PartialEq for AppVersion {
  fn eq(&self, other: &Self) -> bool {
    self.version == other.version
  }
}

impl AppVersion {
  pub fn new(app_id: AppId, file_id: Principal, version: Version) -> Self {
    AppVersion {
      app_id: app_id.clone(),
      version,
      status: AppVersionStatus::NEW,
      file_id,
      wasms: vec![Wasm::new(app_id.clone(), version, ASSET, file_id), Wasm::new(app_id.clone(), version, BACKEND, file_id)],
    }
  }

  pub fn set_frontend_address(&mut self, canister_id: Principal) {
    self.wasms.iter_mut().for_each(|mut wasm| {
      if wasm.canister_type == ASSET {
        wasm.canister_id = Some(canister_id);
      }
    });
  }

  pub fn get_frontend_address(&self) -> Option<Principal> {
    match self.wasms.iter().find(|wasm| wasm.canister_type == ASSET) {
      Some(wasm) => wasm.canister_id,
      None => None
    }
  }

  pub fn submit(&mut self) {
    self.status = AppVersionStatus::SUBMITTED;
  }

  pub fn revoke(&mut self) {
    self.status = AppVersionStatus::REVOKED;
  }

  pub fn release(&mut self) {
    self.status = AppVersionStatus::RELEASED;
  }

  pub fn approve(&mut self) {
    self.status = AppVersionStatus::APPROVED;
  }

  pub fn reject(&mut self) {
    self.status = AppVersionStatus::REJECTED;
  }
}

/********************  wasm  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub enum CanisterType {
  BACKEND,
  ASSET,
}

impl fmt::Display for CanisterType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Wasm {
  pub id: String,
  pub app_id: AppId,
  pub version: Version,
  pub canister_type: CanisterType,
  pub canister_id: Option<Principal>,
  pub file_id: Principal,
}

impl Wasm {
  pub fn new(app_id: AppId, version: Version, canister_type: CanisterType, file_id: Principal) -> Self {
    let id = format!("{}|{}|{}", app_id, version.to_string(), canister_type);
    Wasm { id, app_id, version, canister_type, canister_id: None, file_id }
  }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct File {
  canister_id: Principal,
}