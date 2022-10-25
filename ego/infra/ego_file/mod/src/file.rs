use ego_types::app::FileId;
use ic_cdk::export::candid::{CandidType, Deserialize};
use serde::Serialize;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct File {
    pub file_id: FileId,
    pub file_num: u64,
    pub file_hash: String,
    pub file_size: usize,
}

impl File {
    pub fn new(file_id: FileId, file_num: u64, file_hash: String, file_size: usize) -> Self {
        File {
            file_id,
            file_num,
            file_hash,
            file_size,
        }
    }
}
