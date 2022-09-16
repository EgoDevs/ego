use std::fmt;

use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

use ego_utils::types::{EgoError, Version};

use crate::app::CanisterType::{ASSET, BACKEND};
use crate::types::{AppId, EgoDevErr};

#[derive(Clone, Debug, CandidType, Deserialize, Serialize, PartialEq)]
pub enum Category {
  System,
  Vault,
}


/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct App {
  pub app_id: AppId,
  pub developer_id: Principal,
  pub category: Category,
  pub name: String,
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

impl App {
  pub fn new(user_id: Principal, app_id: AppId, name: String, category: Category, price: f32) -> Self {
    App {
      app_id,
      developer_id: user_id,
      name,
      category,
      release_version: None,
      audit_version: None,
      versions: vec![],
      price,
    }
  }

  pub fn version_get(&self, version: Version) -> Option<&AppVersion> {
    self.versions.iter().find(|app_version| app_version.version == version)
  }

  pub fn version_get_mut(&mut self, version: Version) -> Option<&mut AppVersion> {
    self.versions.iter_mut().find(|app_version| app_version.version == version)
  }

  pub fn version_new(&mut self, file_id: Principal, version: Version) -> Result<AppVersion, EgoError> {
    match self.version_get(version) {
      Some(_) => {
        Err(EgoDevErr::VersionExists.into())
      }
      None => {
        let app_version = AppVersion::new(self.app_id.clone(), file_id, version);
        self.versions.push(app_version.clone());
        Ok(app_version)
      }
    }
  }

  pub fn version_submit(&mut self, version: Version) -> Result<AppVersion, EgoError> {
    self.audit_version = Some(version);

    match self.version_get_mut(version) {
      Some(app_ver) => {
        app_ver.status = AppVersionStatus::SUBMITTED;
        Ok(app_ver.clone())
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_revoke(&mut self, version: Version) -> Result<AppVersion, EgoError> {
    self.audit_version = None;

    match self.version_get_mut(version) {
      Some(app_ver) => {
        app_ver.status = AppVersionStatus::REVOKED;
        Ok(app_ver.clone())
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_release(&mut self, version: Version) -> Result<AppVersion, EgoError> {
    self.release_version = Some(version);

    match self.version_get_mut(version) {
      Some(app_ver) => {
        app_ver.status = AppVersionStatus::RELEASED;
        Ok(app_ver.clone())
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_approve(&mut self, version: Version) -> Result<AppVersion, EgoError> {
    self.audit_version = None;

    match self.version_get_mut(version) {
      Some(app_ver) => {
        app_ver.status = AppVersionStatus::APPROVED;
        Ok(app_ver.clone())
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_reject(&mut self, version: Version) -> Result<AppVersion, EgoError> {
    self.audit_version = None;

    match self.version_get_mut(version) {
      Some(app_ver) => {
        app_ver.status = AppVersionStatus::REJECTED;
        Ok(app_ver.clone())
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn wasm_release_find(&self) -> Result<&Vec<Wasm>, EgoError> {
    match self.release_version {
      None => {Err(EgoDevErr::VersionNotExists.into())}
      Some(version) => {
        match self.version_get(version) {
          Some(app_ver) => {
            Ok(&app_ver.wasms)
          }
          None => Err(EgoDevErr::VersionNotExists.into()),
        }
      }
    }
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
      wasms: vec![Wasm::new(app_id.clone(), version, ASSET, None), Wasm::new(app_id.clone(), version, BACKEND, Some(file_id))],
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

  pub fn wasm_get(&self, canister_type: CanisterType) -> &Wasm{
    self.wasms.iter().find(|wasm| wasm.canister_type == canister_type).unwrap()
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
  /// share frontend canister id
  pub canister_id: Option<Principal>,
  /// unique id of file
  pub fid: String,
  pub file_id: Option<Principal>,
}

impl Wasm {
  pub fn new(app_id: AppId, version: Version, canister_type: CanisterType, file_id: Option<Principal>) -> Self {
    let id = format!("{}|{}", app_id.clone(), canister_type);
    let fid = get_md5(&format!("{}|{}|{}", app_id.clone(), version.to_string(), canister_type).into_bytes());
    Wasm { id, app_id, version, canister_type, canister_id: None, fid, file_id }
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}


