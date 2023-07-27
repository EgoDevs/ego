use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use ego_types::app::AppId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Developer {
    pub developer_id: Principal,
    pub name: String,
    pub is_app_auditor: bool,
    pub is_manager: bool,
    pub created_apps: Vec<AppId>,
}

impl Developer {
    pub fn new(developer_id: Principal, name: String) -> Self {
        Developer {
            developer_id,
            name,
            is_app_auditor: false,
            is_manager: false,
            created_apps: vec![],
        }
    }
}
