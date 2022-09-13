use ic_types::Principal;
use std::borrow::{Borrow, BorrowMut};
use std::collections::{BTreeMap, HashMap};
use itertools::Itertools;

use crate::file::{FileInfo};
use serde::Deserialize;
use ego_utils::types::EgoError;

use crate::storage::{Storage, DEFAULT_MAX_FILES};
use crate::types::{EgoFileError, FileCountResponse, GetFileInfoResponse, FileMainGetResponse, ListFileResponse, FileMainSetResponse, SetFileStableResponse};

#[derive(Clone, Debug, Deserialize)]
pub struct FileBucket {
    pub files: BTreeMap<String, FileInfo>,
    pub storage: Storage<FileInfo>,
}

impl Default for FileBucket {
    fn default() -> Self {
        FileBucket {
            files: BTreeMap::default(),
            storage: Storage::new(DEFAULT_MAX_FILES),
        }
    }
}

impl FileBucket {
    fn write_file(&mut self, file_num: u64, file_info: FileInfo, data: Vec<u8>) {
        self.storage
            .borrow_mut()
            .write_file(file_num, file_info, data)
            .map_err(|f| ic_cdk::trap(&format!("{}", f)))
            .ok();
    }

    fn update_info(&mut self, file_num: u64, file_info: FileInfo) {
        self.storage
            .borrow_mut()
            .write_info(file_num, file_info)
            .map_err(|f| ic_cdk::trap(&format!("{}", f)))
            .ok();
    }

    // fn read_info(&self, file_num: u64) -> FileInfo {
    //     self.storage.borrow().read_info(file_num)
    //         .map_err(|f| ic_cdk::trap(&format!("{}", f))).unwrap()
    // }

    fn read_file(&self, file_num: u64) -> Vec<u8> {
        self.storage.borrow().read_file(file_num)
            .map_err(|f| ic_cdk::trap(&format!("{}", f))).unwrap()
    }
}

impl FileBucket {
    pub fn file_set(&mut self, hash: String, data: Vec<u8>)
                    -> Result<FileMainSetResponse, EgoError>  {
        if get_md5(data.clone()) != hash {
            return Err(EgoFileError::InvalidFileHash.into());
        }

        match self.files.get(&fid) {
            Some(f) => {
                let file_num = f.file_num;
                let file_size = data.len() as u64;
                let file_info = FileInfo::new( file_num, fid.clone(), hash, file_size);
                self.files.insert(fid.clone(), file_info.clone());
                self.write_file(file_num, file_info, data);
                Ok(FileMainSetResponse {
                    fid,
                    file_num,
                    file_size,
                })
            }
            None => match self.storage.borrow_mut().alloc_new_file() {
                Some(file_num) => {
                    let file_size = data.len() as u64;
                    let file_info = FileInfo::new(file_num, fid.clone(), hash, file_size);
                    self.files.insert(fid.clone(), file_info.clone());
                    self.write_file(file_num, file_info, data);
                    Ok(FileMainSetResponse {
                        fid,
                        file_num,
                        file_size,
                    })
                }
                None => Err(EgoFileError::StorageFull)
            }
        }
    }


    pub fn file_set(&self, fid: String) -> Result<FileMainGetResponse, EgoError> {
        match self.files.get(&fid) {
            Some(f) => {
                let data = self.read_file(f.file_num);
                Ok(FileMainGetResponse {
                    data
                })
            }
            None => Err(EgoFileError::FidNotFound)
        }
    }

    pub fn list_file(&self) -> Result<ListFileResponse, EgoFileError> {
        let list = self.files.values().cloned().collect_vec();
        Ok(ListFileResponse{
            list
        })
    }

    pub fn file_count(&self) -> Result<FileCountResponse, EgoFileError> {
        let count = self.storage.file_count() as u64;
        Ok(FileCountResponse{
            count
        })
    }

    pub fn post_upgrade(&mut self) {
        match Storage::from_stable_memory() {
            Some(storage) => {
                self.storage = Storage::from(storage.clone());
            }
            None => {
                self.storage.borrow().flush();
            }
        }
    }
}

pub fn get_md5(data: Vec<u8>) -> String {
    let digest = md5::compute(data);
    return format!("{:?}", digest);
}

