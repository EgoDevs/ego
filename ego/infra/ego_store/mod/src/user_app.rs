use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_types::app::{AppId, Canister, Category};
use ego_types::version::Version;

use crate::app::EgoStoreApp;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct WalletApp {
  pub app_id: AppId,
  pub current_version: Version,
  pub frontend: Option<Canister>,
  pub backend: Option<Canister>,
}

impl WalletApp {
  pub fn new(
    app_id: &AppId,
    current_version: &Version,
    frontend: Option<Canister>,
    backend: Option<Canister>,
  ) -> Self {
    WalletApp {
      app_id: app_id.clone(),
      current_version: current_version.clone(),
      frontend,
      backend,
    }
  }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct UserApp {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub current_version: Version,
  pub frontend: Option<Canister>,
  pub backend: Option<Canister>,
  pub latest_version: Version,
}

impl UserApp {
  pub fn new(user_app: &WalletApp, app: &EgoStoreApp) -> Self {
    UserApp {
      app_id: user_app.app_id.clone(),
      name: app.name.clone(),
      category: app.category.clone(),
      logo: app.logo.clone(),
      description: app.description.clone(),
      current_version: user_app.current_version.clone(),
      frontend: user_app.frontend.clone(),
      backend: user_app.backend.clone(),
      latest_version: app.current_version.clone()
    }
  }

  pub fn wallet_app(&self) -> WalletApp {
    WalletApp {
      app_id: self.app_id.clone(),
      current_version: self.current_version.clone(),
      frontend: self.frontend.clone(),
      backend: self.backend.clone(),
    }
  }
}
