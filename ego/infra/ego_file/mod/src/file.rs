
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct FileInfo {
    pub file_num: u64,
    pub file_id: String,
    pub file_hash: String,
    pub file_size: u64
}

impl FileInfo {
    pub fn new(file_num: u64, file_id: String, file_hash: String, file_size: u64) -> Self {
        FileInfo {
            file_num,
            file_id,
            file_hash,
            file_size
        }
    }
}

