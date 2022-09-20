use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;
use ego_types::app::App;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainReleaseRequest {
  pub app: App
}