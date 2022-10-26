use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use crate::app::EgoStoreApp;
use ego_types::app::{AppId, Canister, Category};
use ego_types::version::Version;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserApp {
    pub app_id: AppId,
    pub current_version: Version,
    pub frontend: Option<Canister>,
    pub backend: Option<Canister>,
}

impl UserApp {
    pub fn new(
        app_id: &AppId,
        current_version: &Version,
        frontend: Option<Canister>,
        backend: Option<Canister>,
    ) -> Self {
        UserApp {
            app_id: app_id.clone(),
            current_version: current_version.clone(),
            frontend,
            backend,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppInstalled {
    pub app_id: AppId,
    pub name: String,
    pub category: Category,
    pub logo: String,
    pub description: String,
    pub current_version: Version,
    pub frontend: Option<Canister>,
    pub backend: Option<Canister>,
}

impl AppInstalled {
    pub fn new(user_app: &UserApp, app: &EgoStoreApp) -> Self {
        AppInstalled {
            app_id: user_app.app_id.clone(),
            name: app.name.clone(),
            category: app.category.clone(),
            logo: app.logo.clone(),
            description: app.description.clone(),
            current_version: user_app.current_version.clone(),
            frontend: user_app.frontend.clone(),
            backend: user_app.backend.clone(),
        }
    }
}
