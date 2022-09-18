use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_types::Principal;
use ego_utils::types::AppId;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Developer {
  pub user_id: Principal,
  pub name: String,
  pub is_app_auditor: bool,
  pub is_manager: bool,
  pub created_apps: Vec<AppId>,
}

impl Developer {
  pub fn new(user_id: Principal, name: String) -> Self {
    Developer {
      user_id,
      name,
      is_app_auditor: false,
      is_manager: false,
      created_apps: vec![],
    }
  }
}