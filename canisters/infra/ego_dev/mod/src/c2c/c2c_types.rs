use candid::{CandidType, Deserialize};
use serde::Serialize;

use ego_types::app::{App, Wasm};

// type for ego_store

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoStoreApp {
  pub app: App,
  pub wasm: Wasm,
}
