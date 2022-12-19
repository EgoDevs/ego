use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EgoError {
  pub code: u16,
  pub msg: String,
}

impl EgoError {
  pub fn new(code: u16, msg: &str) -> Self {
    EgoError {
      code,
      msg: msg.to_string(),
    }
  }
}

impl From<std::string::String> for EgoError {
  fn from(msg: String) -> Self {
    EgoError { code: 255, msg }
  }
}

impl From<(RejectionCode, std::string::String)> for EgoError {
  fn from((code, msg): (RejectionCode, String)) -> Self {
    EgoError {
      code: code as u16,
      msg,
    }
  }
}
