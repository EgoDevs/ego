use candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Record {
    pub id: u64,
    pub scope: String,
    pub event: String,
    pub message: String,
    pub create_at: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct EgoRecord {
    pub records: Vec<Record>,
    pub record_id: u64,
}

impl EgoRecord {
    pub fn new() -> Self {
        EgoRecord {
            records: vec![],
            record_id: 0,
        }
    }
}

impl EgoRecord {
    pub fn record_add(&mut self, scope: &str, event: &str, message: &str, create_at: u64) {
        self.record_id += 1;
        self.records.push(Record {
            id: self.record_id,
            scope: scope.to_string(),
            event: event.to_string(),
            message: message.to_string(),
            create_at,
        });
    }
}
