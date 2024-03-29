use std::collections::BTreeMap;

use candid::{CandidType, Deserialize};
use ic_cdk::api::{
  stable::{stable64_grow, stable64_read, stable64_write},
  trap,
};

use ego_types::app::EgoError;
use ego_types::app::FileId;

use crate::file::File;
use crate::state::{error_log_add, info_log_add};
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
      files: BTreeMap::new(),
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
          trap(&format!(
            "failed to grow stable memory by {} pages",
            pages_to_grow
          ))
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
  pub fn file_write(
    &mut self,
    fid: &FileId,
    hash: &str,
    data: Vec<u8>,
  ) -> Result<bool, EgoError> {
    info_log_add("1. check file size");
    if data.len() > DEFAULT_FILE_SIZE as usize {
      error_log_add("file too large");
      return Err(EgoFileError::FileTooLarge.into());
    }

    info_log_add("2. check md5");
    if get_md5(&data) != hash {
      error_log_add("hash mismatch");
      return Err(EgoFileError::InvalidFileHash.into());
    }

    info_log_add(format!("3. get file by fid:{}", fid).as_str());
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

    info_log_add(format!("==> write file to file_num: {}, offset: {}, with len: {}",
                         file.file_num,
                         file_offset,
                         data.len()).as_str());
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
        info_log_add(format!("==> read file from file_num: {}, offset: {}, with len: {}",
                             file.file_num,
                             file_offset,
                             len).as_str());
        let data = buf[0..len].to_vec();
        Ok(data)
      }
      None => {
        error_log_add(format!("error reading file fid:{}", fid).as_str());
        Err(EgoFileError::FidNotFound.into())
      }
    }
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}
