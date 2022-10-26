use crate::log::Log;
use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct EgoLog {
    pub logs: Vec<Log>,
}

impl EgoLog {
    pub fn new() -> Self {
        EgoLog { logs: vec![] }
    }

    pub fn canister_log_add(&mut self, canister_id: Principal, ts: u64, log: String) {
        self.logs.insert(
            0,
            Log {
                canister_id,
                created_at: ts,
                log,
            },
        );
    }

    pub fn canister_log_get(&self, from_ts: u64, to_ts: u64) -> Vec<Log> {
        self.logs
            .iter()
            .filter_map(|log| {
                if log.created_at >= from_ts && log.created_at <= to_ts {
                    Some(log.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}
