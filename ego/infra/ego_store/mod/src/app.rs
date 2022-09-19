use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_utils::types::{AppId, Category, Version, Wasm};

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct App {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub current_version: Version,
  pub frontend: Wasm,
  pub backend: Wasm,
  pub price: f32
}


impl App {
  pub fn to_string(&self) -> String {
    format!("app_id: {:?},category:{:?},current_version:{:?},",
            self.app_id, self.category,self.current_version)
  }
}

impl App {
  pub fn new(app_id: AppId, name: String, category: Category, logo: String, description: String, current_version: Version, frontend: Wasm, backend: Wasm, price: f32) -> Self {
    App {
      app_id,
      name,
      category,
      logo,
      description,
      current_version,
      frontend,
      backend,
      price
    }
  }
}