use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::AppId;

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
