use std::collections::BTreeMap;
use ic_cdk::export::candid::Deserialize;

use ic_cdk::api::{
    stable::{stable64_grow, stable64_read, stable64_size, stable64_write},
    trap,
};
use ego_users::users::User;
use ego_utils::types::EgoError;
use crate::types::EgoFileError;

const KB: u64 = 1024;
const MB: u64 = 1024 * 1024;

pub(crate) const DEFAULT_MAX_FILES: u64 = 1_000_000_000;

pub const DEFAULT_FILE_SIZE: u64 = 2 * MB;
const HEADER_SIZE: u64 = DEFAULT_FILE_SIZE;

const WASM_PAGE_SIZE: u64 = 64 * KB;

#[derive(Deserialize)]
pub struct Storage {
    pub header: Header
}

#[derive(Deserialize)]
pub struct Header {
    length: u64,
    capacity: u64,
    files: BTreeMap<String, u64>,
    pub user: User
}

impl Storage {
    pub fn from(st: Storage) -> Self {
        Self {
            header: st.header,
        }
    }

    pub fn new() -> Self {
        Self {
            header: Header {
                length: 0,
                capacity: 0,
                files: BTreeMap::new(),
                user: User::default()
            }
        }
    }

    pub fn file_count(&self) -> u64 {
        self.header.capacity
    }

    fn next_file_num(&mut self) -> Result<u64, EgoError> {
        if self.header.capacity >= self.header.length {
            if self.header.length + 10 > DEFAULT_MAX_FILES {
                return Err(EgoFileError::StorageFull.into());
            } else {
                // increase 10 wasm file
                let mut pages_to_grow = 10 * DEFAULT_FILE_SIZE / WASM_PAGE_SIZE;

                // add one more page for header at the beginning
                if self.header.length == 0 {
                    pages_to_grow += HEADER_SIZE / WASM_PAGE_SIZE;
                }

                let result = stable64_grow(pages_to_grow);
                if result.is_err() {
                    trap(&format!("failed to grow stable memory by {} pages", pages_to_grow))
                }

                self.header.length += 10;
            }
        }

        if self.header.capacity < self.header.length {
            let num = self.header.capacity;
            self.header.capacity += 1;
            Ok(num)
        } else {
            Err(EgoFileError::StorageFull.into())
        }
    }

    /// Writes the file to stable memory.
    pub fn file_write(&mut self, fid: &str, hash: &str, data: Vec<u8>) -> Result<bool, EgoError> {
        if data.len() > DEFAULT_FILE_SIZE as usize {
            return Err(EgoFileError::FileTooLarge.into());
        }

        if get_md5(&data) != hash {
            return Err(EgoFileError::InvalidFileHash.into());
        }

        let file_num = match self.header.files.get(fid) {
            Some(file_num) => *file_num,
            None => {
                let file_num = self.next_file_num()?;
                self.header.files.insert(fid.to_string(), file_num);
                file_num
            }
        };

        ic_cdk::println!("write file to file_num: {}", file_num);

        //write file
        let file_offset = HEADER_SIZE / WASM_PAGE_SIZE + file_num * DEFAULT_FILE_SIZE;
        stable64_write(file_offset, &(data.len() as u64).to_le_bytes()); // file length
        stable64_write(file_offset + std::mem::size_of::<u64>() as u64, &data);
        Ok(true)
    }

    /// Reads file from stable memory.
    pub fn file_read(&self, fid: &str) -> Result<Vec<u8>, EgoError> {
        match self.header.files.get(fid) {
            Some(file_num) => {
                ic_cdk::println!("read file from file_num: {}", file_num);

                let file_offset = HEADER_SIZE / WASM_PAGE_SIZE + file_num * DEFAULT_FILE_SIZE;

                // read file
                let mut buf = vec![0; DEFAULT_FILE_SIZE as usize];
                stable64_read(file_offset, &mut buf); // file length
                let len = u64::from_le_bytes(buf[0..8].try_into().unwrap()) as usize;
                let data = buf[8..8 + len].to_vec();
                Ok(data)
            },
            None => Err(EgoFileError::FidNotFound.into())
        }
    }

    pub fn persist(&self) {
        if stable64_size() < 1 {
            let result = stable64_grow(1);
            if result.is_err() {
                trap("failed to grow stable memory by 1 page");
            }
        }
        unsafe {
            let slice = std::slice::from_raw_parts(
                &self.header as *const _ as *const u8,
                std::mem::size_of::<Header>(),
            );
            stable64_write(0, &slice);
        }
    }

    pub fn restore(&self) -> Option<Self> {
        if stable64_size() < 1 {
            return None;
        }

        let mut header: Header = unsafe { std::mem::zeroed() };

        unsafe {
            let slice = std::slice::from_raw_parts_mut(
                &mut header as *mut _ as *mut u8,
                std::mem::size_of::<Header>(),
            );
            stable64_read(0, slice);
        }

        Some(Self {
            header
        })
    }
}


fn get_md5(data: &Vec<u8>) -> String {
    let digest = md5::compute(data);
    return format!("{:?}", digest);
}

