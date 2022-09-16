use ego_utils::types::EgoError;
use crate::state::STORAGE;

pub struct EgoFileService {}

impl EgoFileService {
    pub fn file_main_write(fid: &str, hash: &str, data: Vec<u8>) -> Result<bool, EgoError> {
        STORAGE.with(|s| {
            s.borrow_mut().file_write(fid, hash, data)
        })
    }

    pub fn file_main_read(fid: &str) -> Result<Vec<u8>, EgoError> {
        STORAGE.with(|s| {
            s.borrow().file_read(fid)
        })
    }

    pub fn file_count() -> Result<u64, EgoError> {
        STORAGE.with(|s| {
            Ok(s.borrow().file_count())
        })
    }
}