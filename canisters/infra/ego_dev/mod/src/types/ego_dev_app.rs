use std::borrow::Cow;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use serde::Serialize;

use ego_types::app::EgoError;
use ego_types::app::Version;
use ego_types::app::{App, AppId, Category};
use crate::types::EgoDevErr;
use ic_stable_structures::{BoundedStorable, Storable};
use crate::memory::EGO_DEV_APPS;
use crate::types::app_key::AppKey;
use crate::types::app_version::{AppVersion, AppVersionStatus};


/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoDevApp {
    pub app: App,
    pub developer_id: Principal,
    pub versions: Vec<u64>,
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
        user_id: &Principal,
        app_id: &AppId,
        name: String,
        logo: String,
        description: String,
        category: Category,
        price: f32,
    ) -> Self {
        EgoDevApp {
            app: App::new(
                app_id.clone(),
                name,
                category,
                logo,
                description,
                Version::default(),
                price,
            ),
            developer_id: user_id.clone(),
            audit_version: None,
            versions: vec![],
        }
    }

    pub fn version_get(&self, version: &Version) -> Option<AppVersion> {
        let app_versions: Vec<AppVersion> = self.versions
            .iter()
            .filter_map(|id| {
                match AppVersion::get(id) {
                    None => {
                        None
                    }
                    Some(app_version) => {
                        match app_version.version == *version {
                            true => {
                                Some(app_version)
                            }
                            false => {
                                None
                            }
                        }
                    }
                }
            }).collect();
        if app_versions.is_empty() {
            None
        } else {
            Some(app_versions.get(0).unwrap().clone())
        }
    }

    pub fn version_new(
        &mut self,
        ego_file_canister_id: &Principal,
        version: &Version,
    ) -> Result<AppVersion, EgoError> {
        match self.version_get(version) {
            Some(_) => Err(EgoDevErr::VersionExists.into()),
            None => {
                let app_version =
                    AppVersion::new(&self.app.app_id, ego_file_canister_id, version);
                self.versions.push(app_version.id);
                Ok(app_version)
            }
        }
    }

    pub fn version_submit(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
        self.audit_version = Some(version.clone());

        match self.version_get(version) {
            Some(mut app_version) => {
                app_version.status = AppVersionStatus::SUBMITTED;

                Ok(app_version)
            }
            None => Err(EgoDevErr::VersionNotExists.into()),
        }
    }

    pub fn version_revoke(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
        self.audit_version = None;

        match self.version_get(version) {
            Some(mut app_version) => {
                app_version.status = AppVersionStatus::REVOKED;
                Ok(app_version)
            }
            None => Err(EgoDevErr::VersionNotExists.into()),
        }
    }

    pub fn version_release(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
        self.app.current_version = version.clone();

        match self.version_get(version) {
            Some(mut app_version) => {
                app_version.status = AppVersionStatus::RELEASED;
                Ok(app_version)
            }
            None => Err(EgoDevErr::VersionNotExists.into()),
        }
    }

    pub fn version_approve(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
        self.audit_version = None;

        match self.version_get(version) {
            Some(mut app_version) => {
                app_version.status = AppVersionStatus::APPROVED;
                Ok(app_version)
            }
            None => Err(EgoDevErr::VersionNotExists.into()),
        }
    }

    pub fn version_reject(&mut self, version: &Version) -> Result<AppVersion, EgoError> {
        self.audit_version = None;

        match self.version_get(version) {
            Some(mut app_version) => {
                app_version.status = AppVersionStatus::REJECTED;
                Ok(app_version)
            }
            None => Err(EgoDevErr::VersionNotExists.into()),
        }
    }

    pub fn released_version(&self) -> Result<AppVersion, EgoError> {
        match self.version_get(&self.app.current_version) {
            Some(app_version) => Ok(app_version.clone()),
            None => Err(EgoDevErr::VersionNotExists.into()),
        }
    }

    pub fn list() -> Vec<EgoDevApp> {
        EGO_DEV_APPS.with(|cell| {
            let inst = cell.borrow();
            inst.iter()
              .map(|(_, app)| {
                  app
              }).collect()
        })
    }

    pub fn version_wait_for_audit() -> Vec<EgoDevApp> {
        EGO_DEV_APPS.with(|cell| {
            let inst = cell.borrow();
            inst.iter()
              .filter(|(_, app)| {
                  app.audit_version.is_some()
              })
              .map(|(_, app)| {
                  app
              }).collect()
        })
    }

    pub fn get(app_id: &AppId) -> Option<EgoDevApp> {
        EGO_DEV_APPS.with(|cell| {
            let inst = cell.borrow();
            inst.get(&AppKey::new(&app_id))
        })
    }

    pub fn get_developer_app(developer_id: &Principal, app_id: &AppId) -> Result<Option<EgoDevApp>, EgoError> {
        match EgoDevApp::get(app_id) {
            None => {
                Ok(None)
            }
            Some(ego_dev_app) => {
                match ego_dev_app.developer_id == *developer_id {
                    true => {
                        Ok(Some(ego_dev_app))
                    }
                    false => {
                        Err(EgoDevErr::AppExists.into())
                    }
                }
            }
        }
    }

    pub fn save(&self) {
        EGO_DEV_APPS.with(|cell| {
            let mut inst = cell.borrow_mut();
            let key = AppKey::new(&self.app.app_id);
            inst.insert(key, self.clone());
        });
    }
}

impl Storable for EgoDevApp {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self  {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EgoDevApp {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}
