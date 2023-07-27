use candid::{CandidType, Deserialize};
use serde::{Serialize};
use std::cmp::min;

#[derive(Clone, Debug, CandidType, Serialize)]
pub struct Log {
    logs: Vec<LogEntry>,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct LogEntry {
    pub kind: String,
    pub msg: String,
    pub ts: u64,
}

impl Log {
    pub fn new() -> Self {
        Log { logs: vec![] }
    }

    pub fn info_info_log_add(&mut self, log: String) {
        self.log_add("info".to_string(), log);
    }

    pub fn error_info_log_add(&mut self, log: String) {
        self.log_add("error".to_string(), log);
    }

    fn log_add(&mut self, kind: String, msg: String) {
        self.logs.insert(
            0,
            LogEntry {
                kind,
                msg,
                ts: time(),
            },
        );
        if self.logs.len() > 1000 {
            self.log_clear(500)
        }
    }

    pub fn log_list(&self, amount: usize) -> Vec<LogEntry> {
        let size = min(amount, self.logs.len());
        self.logs[0..size].to_vec()
    }

    pub fn log_clear(&mut self, remain: usize) {
        self.logs.truncate(remain);
    }
}

fn time() -> u64 {
    #[cfg(feature = "test_mode")]
    {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to get timestamp")
            .as_secs()
    }

    #[cfg(not(feature = "test_mode"))]
    {
        ic_cdk::api::time() / 1000000
    }
}
