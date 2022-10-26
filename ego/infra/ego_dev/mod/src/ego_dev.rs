/********************  ego_dev  ********************/
use ic_cdk::export::Principal;
use std::collections::BTreeMap;
use std::vec;

use crate::app::EgoDevApp;
use crate::developer::Developer;
use crate::file::File;
use crate::types::EgoDevErr;
use ego_types::app::{AppId, Category, DeployMode};
use ego_types::ego_error::EgoError;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct EgoDev {
    /// created apps
    pub apps: BTreeMap<AppId, EgoDevApp>,

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
            None => Err(EgoDevErr::NotADeveloper.into()),
        }
    }

    pub fn developer_main_get_mut(
        &mut self,
        user_id: Principal,
    ) -> Result<&mut Developer, EgoError> {
        match self.developers.get_mut(&user_id) {
            Some(user) => Ok(user),
            None => Err(EgoDevErr::NotADeveloper.into()),
        }
    }

    pub fn developer_main_register(
        &mut self,
        user_id: Principal,
        name: String,
    ) -> Result<Developer, EgoError> {
        match self.developers.get(&user_id) {
            None => {
                if self
                    .developers
                    .values()
                    .any(|developer| developer.name == name)
                {
                    Err(EgoDevErr::UserExists.into())
                } else {
                    let developer = Developer::new(user_id, name);
                    self.developers.insert(user_id, developer.clone());
                    Ok(developer)
                }
            }
            Some(developer) => Ok(developer.clone()),
        }
    }

    pub fn developer_app_new(
        &mut self,
        user_id: Principal,
        app_id: AppId,
        name: String,
        logo: String,
        description: String,
        category: Category,
        price: f32,
        deploy_mode: DeployMode,
    ) -> Result<EgoDevApp, EgoError> {
        if self.apps.contains_key(&app_id) {
            let app = self.apps.get(&app_id).unwrap();

            if app.developer_id == user_id {
                Ok(app.clone())
            } else {
                Err(EgoDevErr::AppExists.into())
            }
        } else {
            let _ = self.developer_main_get(user_id)?;

            let app = EgoDevApp::new(
                user_id,
                app_id.clone(),
                name,
                logo,
                description,
                category,
                price,
                deploy_mode,
            );
            self.apps.insert(app_id.clone(), app.clone());

            self.developer_main_get_mut(user_id)?
                .created_apps
                .push(app_id.clone());

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

    pub fn developer_app_list(&self, user_id: Principal) -> Result<Vec<EgoDevApp>, EgoError> {
        let developer = self.developer_main_get(user_id)?;

        let created_apps = developer
            .created_apps
            .iter()
            .map(|app_id| self.apps.get(app_id).unwrap().clone())
            .collect();
        Ok(created_apps)
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

    pub fn admin_file_add(&mut self, file_id: Principal){
        let file = File::new(file_id);
        if !self.ego_files.contains(&file) {
            self.ego_files.push(file);
        }
    }

    pub fn is_manager(&self, caller: Principal) -> bool {
        match self.developers.get(&caller) {
            Some(user) => user.is_manager,
            None => false,
        }
    }

    pub fn is_app_auditor(&self, caller: Principal) -> bool {
        match self.developers.get(&caller) {
            Some(user) => user.is_app_auditor,
            None => false,
        }
    }

    pub fn is_app_developer(&self, caller: Principal) -> bool {
        self.developers.contains_key(&caller)
    }
}
