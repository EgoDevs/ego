use candid::{candid_method};

use ic_cdk::caller;
use ic_cdk_macros::*;

use ego_file_mod::service::{EgoFileService};
use ego_file_mod::state::STORAGE;
use ego_file_mod::types::{FileMainReadRequest, FileMainReadResponse, FileMainWriteRequest, FileMainWriteResponse};
use ego_utils::types::EgoError;


use ego_users::inject_ego_users;


#[init]
#[candid_method(init)]
fn init() {
    ic_cdk::println!("ego-file: init, caller is {}", caller());
}

/********************  file method ********************/
#[update(name = "file_main_write")]
#[candid_method(update, rename = "file_main_write")]
fn file_main_write(req: FileMainWriteRequest) -> Result<FileMainWriteResponse, EgoError> {
    ic_cdk::println!("ego-file: file_main_write");

    let ret = EgoFileService::file_main_set(&req.fid, &req.hash, req.data)?;
    Ok(FileMainWriteResponse {ret})
}


#[query(name = "file_main_read")]
#[candid_method(query, rename = "file_main_read")]
fn file_main_read(req: FileMainReadRequest) -> Result<FileMainReadResponse, EgoError> {
    ic_cdk::println!("ego-file: file_main_read");

    let data = EgoFileService::file_main_read(&req.fid)?;
    Ok(FileMainReadResponse {data})
}

#[pre_upgrade]
fn pre_upgrade() {
    ic_cdk::println!("ego-file: pre_upgrade");

    STORAGE.with(|s| {
        USER.with(|u| {
            s.borrow_mut().header.user = u.borrow().clone();
        });
        s.borrow().persist()
    })
}

#[post_upgrade]
fn post_upgrade() {
    ic_cdk::println!("ego-file: post_upgrade");

    STORAGE.with(|s| {
        match s.borrow().restore() {
            Some(storage) => {
                *s.borrow_mut() = storage;
                USER.with(|u| {
                   *u.borrow_mut() = s.borrow().header.user.clone();
                });
            },
            _ => {}
        }
    })
}

inject_ego_users!();