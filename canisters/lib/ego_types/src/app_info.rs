use candid::{CandidType, Deserialize, Principal};
use serde::Serialize;

use crate::app::{AppId, Version};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct AppInfo {
  pub wallet_id: Option<Principal>,
  pub app_id: AppId,
  pub current_version: Version,
  pub latest_version: Version,
}

impl Default for AppInfo {
  fn default() -> Self {
    AppInfo {
      wallet_id: None,
      app_id: "".to_string(),
      current_version: Default::default(),
      latest_version: Default::default(),
    }
  }
}
