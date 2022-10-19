use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;
use ego_types::app::{AppId, Category, DeployMode, Wasm};
use ego_types::app::CanisterType::{ASSET, BACKEND};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;

use crate::types::{EgoDevErr};

/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoDevApp {
  pub app_id: AppId,
  pub developer_id: Principal,
  pub category: Category,
  pub name: String,
  pub logo: String,
  pub description: String,
  pub release_version: Option<Version>,
  pub audit_version: Option<Version>,
  pub versions: Vec<AppVersion>,
  pub price: f32,
  pub deploy_mode: DeployMode
}

impl EgoDevApp {
  pub fn to_string(&self) -> String {
    format!("app_id: {:?},user_id:{:?},category:{:?},release_version:{:?},versions:{:?},",
            self.app_id, self.developer_id.to_text(), self.category, self.release_version, self.versions)
  }
}

impl EgoDevApp {
  pub fn new(user_id: Principal, app_id: AppId, name: String, logo: String, description: String, category: Category, price: f32, deploy_mode: DeployMode) -> Self {
    EgoDevApp {
      app_id,
      developer_id: user_id,
      name,
      logo,
      description,
      category,
      release_version: None,
      audit_version: None,
      versions: vec![],
      price,
      deploy_mode
    }
  }

  pub fn version_get(&self, version: Version) -> Option<&AppVersion> {
    self.versions.iter().find(|app_version| app_version.version == version)
  }

  pub fn version_get_mut(&mut self, version: Version) -> Option<&mut AppVersion> {
    self.versions.iter_mut().find(|app_version| app_version.version == version)
  }

  pub fn version_new(&mut self, ego_file_canister_id: Principal, version: Version) -> Result<AppVersion, EgoError> {
    match self.version_get(version) {
      Some(_) => {
        Err(EgoDevErr::VersionExists.into())
      }
      None => {
        let app_version = AppVersion::new(self.app_id.clone(), ego_file_canister_id, version);
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

  pub fn released_version(&self) -> Result<AppVersion, EgoError> {
    match self.version_get(self.release_version.unwrap()) {
      Some(app_version) => Ok(app_version.clone()),
      None => Err(EgoDevErr::VersionNotExists.into())
    }
  }
}

/********************  app version  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppVersion {
  pub app_id: AppId,
  pub version: Version,
  pub status: AppVersionStatus,
  pub file_id: Principal,
  pub frontend: Option<Wasm>,
  pub backend: Option<Wasm>
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
  pub fn new(app_id: AppId, ego_file_canister_id: Principal, version: Version) -> Self {
    AppVersion {
      app_id: app_id.clone(),
      version,
      status: AppVersionStatus::NEW,
      file_id: ego_file_canister_id,
      frontend: None,
      backend: None
    }
  }

  pub fn frontend_update(&mut self, frontend_id: Principal) -> Wasm {
    if self.frontend.is_none() {
      self.frontend = Some(Wasm::new(self.app_id.clone(), self.version, ASSET, frontend_id));
    }

    self.frontend.clone().unwrap()
  }

  pub fn backend_update(&mut self) -> Wasm {
    if self.backend.is_none() {
      self.backend = Some(Wasm::new(self.app_id.clone(), self.version, BACKEND, self.file_id));
    }

    self.backend.clone().unwrap()
  }
}


