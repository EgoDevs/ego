/********************  ego_dev  ********************/
use std::collections::BTreeMap;
use std::vec;

use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use ego_types::app::EgoError;
use ego_types::app::{AppId, Category};
use crate::types::app_version::AppVersion;

use crate::types::developer::Developer;
use crate::types::ego_dev_app::EgoDevApp;
use crate::types::ego_file::EgoFile;
use crate::types::EgoDevErr;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoDev {
    /// created apps
    pub apps: BTreeMap<AppId, EgoDevApp>,

    pub app_versions: BTreeMap<AppId, AppVersion>,

    /// registered ego developers
    pub developers: BTreeMap<Principal, Developer>,

    /// ego_file canisters
    pub ego_files: Vec<EgoFile>,
}

impl EgoDev {
    pub fn new() -> Self {
        EgoDev {
            apps: BTreeMap::new(),
            app_versions: Default::default(),
            developers: BTreeMap::new(),
            ego_files: Vec::new(),
        }
    }

    pub fn developer_app_new(
        &mut self,
        user_id: &Principal,
        app_id: &AppId,
        name: String,
        logo: String,
        description: String,
        category: Category,
        price: f32,
    ) -> Result<EgoDevApp, EgoError> {
        if self.apps.contains_key(app_id) {
            let app = self.apps.get(app_id).unwrap();

            if app.developer_id == *user_id {
                Ok(app.clone())
            } else {
                Err(EgoDevErr::AppExists.into())
            }
        } else {
            let mut developer = Developer::get(user_id).expect("developer not exists");

            let app = EgoDevApp::new(
                user_id,
                app_id,
                name,
                logo,
                description,
                category,
                price,
            );
            self.apps.insert(app_id.clone(), app.clone());

            developer.created_apps.push(app_id.clone());
            developer.save();

            Ok(app)
        }
    }

    pub fn developer_app_get(
        &self,
        user_id: &Principal,
        app_id: &AppId,
    ) -> Result<&EgoDevApp, EgoError> {
        match self.apps.get(app_id) {
            None => Err(EgoDevErr::AppNotExists.into()),
            Some(app) => {
                if app.developer_id != *user_id {
                    Err(EgoDevErr::UnAuthorized.into())
                } else {
                    Ok(app)
                }
            }
        }
    }

    pub fn developer_app_get_mut(
        &mut self,
        user_id: &Principal,
        app_id: &AppId,
    ) -> Result<&mut EgoDevApp, EgoError> {
        match self.apps.get_mut(app_id) {
            None => Err(EgoDevErr::AppNotExists.into()),
            Some(app) => {
                if app.developer_id != *user_id {
                    Err(EgoDevErr::UnAuthorized.into())
                } else {
                    Ok(app)
                }
            }
        }
    }

    pub fn developer_app_transfer(
        &mut self,
        developer_id: &Principal,
        app_id: &AppId,
    ) -> Result<(), EgoError> {
        match self.apps.get_mut(app_id) {
            None => Err(EgoDevErr::AppNotExists.into()),
            Some(app) => {
                app.developer_id = developer_id.clone();
                Ok(())
            }
        }
    }

    pub fn version_wait_for_audit(&self) -> Vec<EgoDevApp> {
        self.apps
            .iter()
            .filter(|(_app_id, app)| app.audit_version.is_some())
            .map(|(_app_id, app)| app.clone())
            .collect()
    }

    pub fn file_get(&mut self) -> Result<Principal, EgoError> {
        if self.ego_files.is_empty() {
            Err(EgoDevErr::NoFile.into())
        } else {
            let file = self.ego_files.iter_mut().min().unwrap();
            file.wasm_count += 1;
            Ok(file.canister_id)
        }
    }

    pub fn admin_file_add(&mut self, file_id: Principal) {
        let file = EgoFile::new(&file_id);
        if !self.ego_files.contains(&file) {
            self.ego_files.push(file);
        }
    }
}
