use std::borrow::Cow;

use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use serde::Serialize;

use ego_types::app::{App, AppId, Category};
use ego_types::app::EgoError;
use ego_types::app::Version;
use ego_utils::util::time;

use crate::memory::EGO_DEV_APPS;
use crate::types::app_key::AppKey;
use crate::types::app_version::{AppVersion, AppVersionStatus};
use crate::types::EgoDevErr;

/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoDevApp {
  pub app: App,
  pub developer_id: Principal,
  pub audit_version: Option<Version>,
  pub last_update: u64,    // second
}

impl EgoDevApp {
  pub fn new(
    developer_id: &Principal,
    app_id: &AppId,
    name: &str,
    logo: &str,
    description: &str,
    category: &Category,
    price: f32,
  ) -> Self {
    EgoDevApp {
      app: App::new(
        app_id.clone(),
        name.to_string(),
        category.clone(),
        logo.to_string(),
        description.to_string(),
        Version::default(),
        price,
      ),
      developer_id: developer_id.clone(),
      audit_version: None,
      last_update: 0,
    }
  }

  pub fn version_get(&self, version: &Version) -> Option<AppVersion> {
    AppVersion::get_by_app_id_and_version(&self.app.app_id, version)
  }

  pub fn version_new(
    &mut self,
    ego_file_canister_id: &Principal,
    version: &Version,
  ) -> Result<AppVersion, EgoError> {
    match self.version_get(version) {
      Some(_) => Err(EgoDevErr::VersionExists.into()),
      None => {
        let mut app_version =
          AppVersion::new(&self.app.app_id, ego_file_canister_id, version);
        app_version.save();
        Ok(app_version)
      }
    }
  }

  pub fn version_submit(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
    if self.audit_version.is_some() {
      return Err(EgoDevErr::OperationNotPermitted.into());
    }

    match self.version_get(version) {
      Some(mut app_version) => {
        match app_version.status {
          AppVersionStatus::NEW | AppVersionStatus::REJECTED | AppVersionStatus::REVOKED => {
            self.audit_version = Some(version.clone());
            self.save();

            app_version.status = AppVersionStatus::SUBMITTED;
            app_version.save();

            Ok(app_version)
          }
          _ => {
            Err(EgoDevErr::OperationNotPermitted.into())
          }
        }
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_revoke(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
    match self.version_get(version) {
      Some(mut app_version) => {
        match app_version.status {
          AppVersionStatus::SUBMITTED => {
            self.audit_version = None;
            self.save();

            app_version.status = AppVersionStatus::REVOKED;
            app_version.save();
            Ok(app_version)
          }
          AppVersionStatus::RELEASED => {
            app_version.status = AppVersionStatus::REVOKED;
            app_version.save();
            Ok(app_version)
          }
          _ => {
            Err(EgoDevErr::OperationNotPermitted.into())
          }
        }
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_release(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
    match self.version_get(version) {
      Some(mut app_version) => {
        match app_version.status {
          AppVersionStatus::APPROVED => {
            self.app.current_version = version.clone();
            self.save();

            app_version.status = AppVersionStatus::RELEASED;
            app_version.save();
            Ok(app_version)
          }
          _ => {
            Err(EgoDevErr::OperationNotPermitted.into())
          }
        }
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_approve(&mut self) -> Result<AppVersion, EgoError> {
    if self.audit_version.is_none() {
      return Err(EgoDevErr::OperationNotPermitted.into());
    }

    match self.version_get(&self.audit_version.unwrap()) {
      Some(mut app_version) => {
        self.audit_version = None;
        self.save();

        app_version.status = AppVersionStatus::APPROVED;
        app_version.save();

        Ok(app_version)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn version_reject(&mut self) -> Result<AppVersion, EgoError> {
    if self.audit_version.is_none() {
      return Err(EgoDevErr::OperationNotPermitted.into());
    }

    match self.version_get(&self.audit_version.unwrap()) {
      Some(mut app_version) => {
        self.audit_version = None;
        self.save();

        app_version.status = AppVersionStatus::REJECTED;
        app_version.save();
        Ok(app_version)
      }
      None => Err(EgoDevErr::VersionNotExists.into()),
    }
  }

  pub fn released_version(&self) -> Option<AppVersion> {
    self.version_get(&self.app.current_version)
  }

  pub fn list(start: usize, end: usize) -> Vec<EgoDevApp> {
    Self::iter(start, end, |(_, ego_dev_app)| Some(ego_dev_app))
  }

  pub fn by_last_update(start: usize, end: usize, last_update: u64) -> Vec<EgoDevApp> {
    Self::iter(start, end, |(_, ego_dev_app)| {
      match ego_dev_app.last_update >= last_update {
        true => { Some(ego_dev_app) }
        false => { None }
      }
    })
  }

  pub fn by_developer_id(developer_id: &Principal) -> Vec<EgoDevApp> {
    Self::iter(0, Self::len() as usize, |(_, ego_dev_app)| {
      match ego_dev_app.developer_id == *developer_id {
        true => { Some(ego_dev_app) }
        false => { None }
      }
    })
  }

  pub fn by_developer_id_and_id(developer_id: &Principal, app_id: &AppId) -> Option<EgoDevApp> {
    match EgoDevApp::get(app_id) {
      None => { None }
      Some(ego_dev_app) => {
        match ego_dev_app.developer_id == *developer_id {
          true => { Some(ego_dev_app) }
          false => { None }
        }
      }
    }
  }

  pub fn version_wait_for_audit() -> Vec<EgoDevApp> {
    Self::iter(0, Self::len() as usize, |(_, ego_dev_app)| {
      match ego_dev_app.audit_version.is_some() {
        true => { Some(ego_dev_app) }
        false => { None }
      }
    })
  }

  pub fn len() -> u64 {
    EGO_DEV_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.len()
    })
  }

  pub fn get(app_id: &AppId) -> Option<EgoDevApp> {
    EGO_DEV_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.get(&AppKey::new(&app_id))
    })
  }

  pub fn save(&mut self) {
    EGO_DEV_APPS.with(|cell| {
      let mut inst = cell.borrow_mut();
      let key = AppKey::new(&self.app.app_id);
      self.last_update = time();
      inst.insert(key, self.clone());
    });
  }

  fn iter<F>(start: usize, end: usize, filter: F) -> Vec<Self>
    where F: Fn((AppKey, Self)) -> Option<Self> {
    let mut idx = 0;

    EGO_DEV_APPS.with(|cell| {
      let inst = cell.borrow();
      inst.iter().filter_map(|entry| {
        if idx >= end {
          // 如果过了上界，直接忽略
          None
        } else {
          match filter(entry) {
            None => {
              None
            }
            Some(record) => {
              let ret = if idx >= start && idx < end {
                Some(record)
              } else {
                None
              };
              idx += 1;
              ret
            }
          }
        }
      }).collect()
    })
  }
}

impl Storable for EgoDevApp {
  fn to_bytes(&self) -> Cow<[u8]> {
    Cow::Owned(Encode!(self).unwrap())
  }

  fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
  }
}

impl BoundedStorable for EgoDevApp {
  const MAX_SIZE: u32 = 2048;
  const IS_FIXED_SIZE: bool = false;
}
