use candid::CandidType;
use ic_cdk::export::candid::Deserialize;
use serde::Serialize;

use crate::app::{AppId, Version};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppInfo {
  pub app_id: AppId,
  pub current_version: Version,
}

impl Default for AppInfo {
  fn default() -> Self {
    AppInfo {
      app_id: "".to_string(),
      current_version: Default::default(),
    }
  }
}