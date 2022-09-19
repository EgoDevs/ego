use std::collections::BTreeMap;
use ic_cdk::export::candid::{CandidType, Deserialize};


use ic_cdk::api::{
    stable::{stable64_grow, stable64_read, stable64_write},
    trap,
};
use ego_types::app::FileId;
use ego_types::ego_error::EgoError;
use crate::file::File;
use crate::types::EgoFileError;

const KB: u64 = 1024;
const MB: u64 = 1024 * 1024;

pub(crate) const DEFAULT_MAX_FILES: u64 = 1_000_000_000;

pub const DEFAULT_FILE_SIZE: u64 = 2 * MB;
pub const HEADER_SIZE: u64 = DEFAULT_FILE_SIZE;
pub const WASM_PAGE_SIZE: u64 = 64 * KB;

#[derive(CandidType, Deserialize, Clone)]
pub struct Storage {
    pub length: u64,
    pub capacity: u64,
    pub files: BTreeMap<FileId, File>,
}

impl Storage {
    pub fn from(st: Storage) -> Self {
        Self {
            length: st.length,
            capacity: st.capacity,
            files: st.files,
        }
    }

    pub fn new() -> Self {
        Self {
            length: 0,
            capacity: 0,
            files: BTreeMap::new()
        }
    }

    pub fn file_count(&self) -> u64 {
        self.capacity
    }

    fn next_file_num(&mut self) -> Result<u64, EgoError> {
        if self.capacity >= self.length {
            if self.length + 10 > DEFAULT_MAX_FILES {
                return Err(EgoFileError::StorageFull.into());
            } else {
                // increase 10 wasm file
                let pages_to_grow = 10 * DEFAULT_FILE_SIZE / WASM_PAGE_SIZE;

                let result = stable64_grow(pages_to_grow);
                if result.is_err() {
                    trap(&format!("failed to grow stable memory by {} pages", pages_to_grow))
                }

                self.length += 10;
            }
        }

        if self.capacity < self.length {
            let num = self.capacity;
            self.capacity += 1;
            Ok(num)
        } else {
            Err(EgoFileError::StorageFull.into())
        }
    }

    /// Writes the file to stable memory.
    pub fn file_write(&mut self, fid: &FileId, hash: &str, data: Vec<u8>) -> Result<bool, EgoError> {
        if data.len() > DEFAULT_FILE_SIZE as usize {
            return Err(EgoFileError::FileTooLarge.into());
        }

        if get_md5(&data) != hash {
            return Err(EgoFileError::InvalidFileHash.into());
        }

        let file = match self.files.get(fid) {
            Some(file) => file,
            None => {
                let file_num = self.next_file_num()?;
                let file = File::new(fid.to_string(), file_num, hash.to_string(), data.len());
                self.files.insert(fid.to_string(), file.clone());
                self.files.get(fid).unwrap()
            }
        };

        //write file
        let file_offset = HEADER_SIZE + file.file_num * DEFAULT_FILE_SIZE;

        ic_cdk::println!("==> write file to file_num: {}, offset: {}, with len: {}", file.file_num, file_offset, data.len());
        stable64_write(file_offset, &data);
        Ok(true)
    }

    /// Reads file from stable memory.
    pub fn file_read(&self, fid: &FileId) -> Result<Vec<u8>, EgoError> {
        match self.files.get(fid) {
            Some(file) => {
                let file_offset = HEADER_SIZE + file.file_num * DEFAULT_FILE_SIZE;

                // read file
                let mut buf = vec![0; DEFAULT_FILE_SIZE as usize];
                let len = file.file_size;
                stable64_read(file_offset, &mut buf); // file length
                ic_cdk::println!("==> read file from file_num: {}, offset: {}, with len: {}", file.file_num, file_offset, len);
                let data = buf[0..len].to_vec();
                Ok(data)
            },
            None => Err(EgoFileError::FidNotFound.into())
        }
    }
}


fn get_md5(data: &Vec<u8>) -> String {
    let digest = md5::compute(data);
    return format!("{:?}", digest);
}

