use std::cmp::min;
use ic_cdk::export::candid::CandidType;
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Serialize)]
pub struct Log {
  pub logs: Vec<String>,
}

impl Log {
  pub fn new() -> Self {
    Log { logs: vec![] }
  }

  pub fn info_log_add(&mut self, log: String) {
    self.logs.insert(0, log);
    if self.logs.len() > 1000 {
      self.log_clear(500)
    }
  }

  pub fn info_info_log_add(&mut self, log: String) {
    self.info_log_add(format!("info - {}", log).to_string());
  }

  pub fn error_info_log_add(&mut self, log: String) {
    self.info_log_add(format!("error - {}", log).to_string());
  }

  pub fn log_list(&self, amount: usize) -> Vec<String> {
    let size = min(amount, self.logs.len());
    self.logs[0..size].to_vec()
  }

  pub fn log_clear(&mut self, remain: usize) {
    self.logs.truncate(remain);
  }
}
