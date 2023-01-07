use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::{App, AppId, Category, Wasm};
use ego_types::app::CanisterType::{ASSET, BACKEND};
use ego_types::app::EgoError;
use ego_types::app::Version;

use crate::types::EgoDevErr;

/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoDevApp {
  pub app: App,
  pub developer_id: Principal,
  pub versions: Vec<AppVersion>,
  pub audit_version: Option<Version>,
}

impl EgoDevApp {
  pub fn to_string(&self) -> String {
    format!(
      "app_id: {:?},user_id:{:?},category:{:?},release_version:{:?},versions:{:?},",
      self.app.app_id,
      self.developer_id.to_text(),
      self.app.category,
      self.app.current_version,
      self.versions
    )
  }
}

impl EgoDevApp {
  pub fn new(
    user_id: Principal,
    app_id: AppId,
    name: String,
    logo: String,
    description: String,
    category: Category,
    price: f32,
  ) -> Self {
    EgoDevApp {
      app: App::new(app_id, name, category, logo, description, Version::default(), price),
      developer_id: user_id,
      audit_version: None,
      versions: vec![],
    }
  }

  pub fn version_get(&self, version: Version) -> Option<&AppVersion> {
    self.versions
      .iter()
      .find(|app_version| app_version.version == version)
  }

  pub fn version_get_mut(&mut self, version: Version) -> Option<&mut AppVersion> {
    self.versions
      .iter_mut()
      .find(|app_version| app_version.version == version)
  }

  pub fn version_new(
    &mut self,
    ego_file_canister_id: Principal,
    version: Version,
  ) -> Result<AppVersion, EgoError> {
    match self.version_get(version) {
      Some(_) => Err(EgoDevErr::VersionExists.into()),
      None => {
        let app_version =
          AppVersion::new(self.app.app_id.clone(), ego_file_canister_id, version);
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
    self.app.current_version = version;

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
    match self.version_get(self.app.current_version) {
      Some(app_version) => Ok(app_version.clone()),
      None => Err(EgoDevErr::VersionNotExists.into()),
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
  pub wasm: Option<Wasm>,
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
      wasm: None,
    }
  }

  pub fn frontend_update(&mut self, frontend_id: Principal) -> Wasm {
    if self.wasm.is_none() {
      self.wasm = Some(Wasm::new(
        self.app_id.clone(),
        self.version,
        ASSET,
        frontend_id,
      ));
    }

    self.wasm.clone().unwrap()
  }

  pub fn backend_update(&mut self) -> Wasm {
    if self.wasm.is_none() {
      self.wasm = Some(Wasm::new(
        self.app_id.clone(),
        self.version,
        BACKEND,
        self.file_id,
      ));
    }

    self.wasm.clone().unwrap()
  }
}
