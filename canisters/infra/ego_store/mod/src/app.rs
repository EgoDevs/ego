use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_types::app::{App, Wasm};

/********************  app  ********************/
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoStoreApp {
    pub app: App,
    pub wasm: Wasm,
}

impl EgoStoreApp {
    pub fn to_string(&self) -> String {
        format!(
            "app_id: {:?},category:{:?},current_version:{:?},",
            self.app.app_id, self.app.category, self.app.current_version
        )
    }
}

impl EgoStoreApp {
    pub fn new(app: App, wasm: Wasm) -> Self {
        EgoStoreApp { app, wasm }
    }
}
