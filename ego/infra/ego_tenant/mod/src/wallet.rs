use std::collections::HashMap;
use std::vec;
use std::vec::Vec;

use ic_cdk::{call};
use ic_cdk::export::candid::CandidType;
use ic_types::Principal;
use itertools::Itertools;
use serde::Deserialize;

use ego_bucket_mod::types::{LoadFileRequest, LoadFileResponse};
use ego_crond_mod::types::{AddCronTaskRequest, AddCronTaskResponse, CronInterval};
use ego_store_mod::api::{EgoStoreErr, GetAppRequest, GetAppResponse};
use ego_store_mod::app::{AppId, CanisterType};
use ego_types::canister::{InstallMode};
use ego_types::error::EgoError;
use ego_types::version::Version;
use ego_utils::canister::*;

// use crate::service::Wallet;
use crate::state::WALLET;
use crate::types::{AppInstallRequest, CheckAppStatus, CheckAppStatusResponse, EgoWalletError, InitWalletCanister};
use crate::types::{ Canister, UserAppResponse };
use crate::types::UserApp;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Wallet {
  pub apps: HashMap<String, UserApp>,
  pub store_canister: Principal,
  pub cron_canister: Principal,
  pub wallet_version: Version,
}


impl From<&Wallet> for Wallet {
  fn from(stable_storage: &Wallet) -> Self {
    Wallet {
      apps: stable_storage.apps.clone(),
      store_canister: stable_storage.store_canister.clone(),
      cron_canister: stable_storage.cron_canister.clone(),
      wallet_version: stable_storage.wallet_version.clone(),
    }
  }
}


async fn get_app_from_store(
  canister_id: Principal,
  request: GetAppRequest,
) -> Result<GetAppResponse, EgoError> {
  ic_cdk::println!("enter get_app_from_store");

  let call_result = call(canister_id, "get_app", (request, )).await
    as Result<(Result<GetAppResponse, EgoError>, ), _>;

  match call_result.unwrap().0 {
    Ok(r) => {
      ic_cdk::println!("get_app_from_store Ok");
      Ok(r)
    }

    Err(err) => {
      ic_cdk::println!("get_app_from_store failed {:?}", err);
      Err(err)
    }
  }
}

async fn get_app_wasm_from_bucket(
  canister_id: Principal,
  request: LoadFileRequest,
) -> Result<LoadFileResponse, EgoError> {
  ic_cdk::println!("enter get_app_wasm_from_bucket canister_id:{}, request:{:?}", canister_id, request);

  let call_result = call(
    canister_id,
    "load_file",
    (request, ),
  )
    .await
    as Result<(Result<LoadFileResponse, EgoError>, ), _>;

  match call_result.unwrap().0 {
    Ok(r) => {
      ic_cdk::println!("get_app_wasm_from_bucket Ok");
      Ok(r)
    }

    Err(err) => {
      ic_cdk::println!("get_app_wasm_from_bucket failed {:?}", err);
      Err(err)
    }
  }
}

async fn registry_cron_task_from_wallet(initial_req: InitWalletCanister) -> Result<AddCronTaskResponse, EgoError> {
  ic_cdk::println!("enter registry_cron_task_from_wallet");
  let init_method_name = String::from("wallet_balance");
  let req = AddCronTaskRequest { method: init_method_name, interval: CronInterval::PerHour };
  let call_result = call(
    initial_req.cron_canister_id,
    "add_cron_task",
    (req, ),
  )
    .await
    as Result<(Result<AddCronTaskResponse, EgoError>, ), _>;

  match call_result.unwrap().0 {
    Ok(r) => {
      ic_cdk::println!("registry_cron_task_from_wallet Ok");
      Ok(r)
    }

    Err(err) => {
      ic_cdk::println!("registry_cron_task_from_wallet failed {:?}", err);
      Err(err.into())
    }
  }
}

fn add_apps(app: UserApp)  {
  //不需要判断已存在 app_id 唯一性有问题
  WALLET.with(|wallet| {
    wallet
      .borrow_mut()
      .apps
      .entry(app.app_id.clone()).or_insert(app);
  })
}


fn remove_app(app: UserApp) -> Result<bool, EgoError> {
  WALLET.with(|wallet| {
    wallet
      .borrow_mut()
      .apps
      .remove(&app.app_id)
  });
  Ok(true)
}


pub async fn upgrade_app(
  request: AppInstallRequest,
) -> Result<bool, EgoWalletError> {
  let caller = ic_cdk::caller();
  ic_cdk::println!("----step 0 in upgrade app: caller is {:?}", caller.to_text());
  let apps = WALLET.with(|b| b.borrow_mut().apps.clone());
  let store_canister = WALLET.with(|b| b.borrow_mut().store_canister.clone());
  let get_file_arg = GetAppRequest {
    app_id: request.app_id.clone(),
  };
  //1. get app wasms from store
  let store_app_response: GetAppResponse = get_app_from_store(
    store_canister,
    get_file_arg,
  ).await.unwrap();

  let app = store_app_response.app.clone();
  ic_cdk::println!("----step 1: app_id is {:?}", app.app_id);
  let wasms = app.find_release_wasms().unwrap();

  let user_app_clone: UserApp = apps.get(&request.app_id).unwrap().clone();
  ic_cdk::println!("----step 2: user_app_clone is {:?}", user_app_clone);

  for wasm in wasms.iter() {
    ic_cdk::println!("--wasm.app_id is {:?}--", wasm.app_id);

    //ignore asset wasm install
    if wasm.canister_type == CanisterType::ASSET {
      continue;
    }

    let load_file_arg = LoadFileRequest {
      appid: app.app_id.clone(),
      version: wasm.version.to_string(),
      fid: wasm.file_id.clone(),
    };

    for c in user_app_clone.canisters.clone() {
      if wasm.canister_type == c.canister_type {
        ic_cdk::println!("----step 3: get wasm from bucket {:?},fileId {:?}", wasm.bucket_id.to_text(), wasm.file_id);
        let wasm_module_response: LoadFileResponse =
          get_app_wasm_from_bucket(wasm.bucket_id, load_file_arg.clone()).await.unwrap();
        install_code(&c.canister_id, wasm_module_response.data, InstallMode::Upgrade).await?;
        ic_cdk::println!("----step 4: install code successful!");
      }
    }
  }

  let version = store_app_response.app.clone().release_version.clone().unwrap();
  let user_app = UserApp {
    app_id: request.app_id.clone(),
    version,
    canisters: user_app_clone.canisters.clone(),
  };

  add_apps(user_app);
  Ok(true)
}

pub async fn install_app(
  app_id: AppId,
) -> Result<bool, EgoError> {
  let store_canister = WALLET.with(|b| b.borrow_mut().store_canister.clone());
  let caller = ic_cdk::caller();
  ic_cdk::println!("----step 0 in install app: caller is {:?}", caller.to_text());

  //1. get App wasms from store
  let get_app_request = GetAppRequest {
    app_id: app_id.clone(),
  };

  let get_app_response: GetAppResponse = match get_app_from_store(
    store_canister,
    get_app_request,
  ).await {
    Ok(res) => {
      res
    }
    Err(e) => {
      return Err(e);
    }
  };

  let app = get_app_response.app.clone();
  let version = app.release_version.unwrap();

  ic_cdk::println!("----step 1.1: app_id is {:?}", app.app_id);

  let wasms = match app.find_wasms(version) {
    Ok(res) => {
      res
    }
    Err(e) => {
      return Err(EgoError::from(e));
    }
  };

  let mut canister_set = Vec::new();

  for wasm in wasms.iter() {
    ic_cdk::println!("--wasm.file_id is {:?}--", wasm.file_id);
    ic_cdk::println!("--wasm.app_id is {:?}--", wasm.app_id);

    //ignore asset wasm install
    if wasm.canister_type == CanisterType::ASSET {
      canister_set.push(
        Canister {
          canister_id: wasm.canister_id.unwrap(),
          canister_type: CanisterType::ASSET,
        }
      );
      continue;
    }

    let load_file_arg = LoadFileRequest {
      appid: app.app_id.clone(),
      version: version.to_string(),
      fid: wasm.file_id.clone(),
    };
    ic_cdk::println!("----step 1: load wasm");
    let wasm_module_response: LoadFileResponse = get_app_wasm_from_bucket(wasm.bucket_id, load_file_arg).await.unwrap();

    ic_cdk::println!("----step 2: create and install code");
    let canister_id = create_and_install(None, wasm_module_response.data, b" ".to_vec(), 910000000000).await?;

    canister_set.push(Canister {
      canister_id: canister_id.clone(),
      canister_type: wasm.canister_type.clone(),
    });
  }

  let version = app.release_version.unwrap();
  let user_app = UserApp {
    app_id: app_id.clone(),
    version,
    canisters: canister_set.clone(),
  };

  ic_cdk::println!("----step 3: add_apps");
  add_apps(user_app);
  Ok(true)
}

pub async fn uninstall_app(
  request: AppInstallRequest,
) -> Result<bool, EgoWalletError> {
  ic_cdk::println!("---- enter uninstall_app");
  let apps = WALLET.with(|b| b.borrow_mut().apps.clone());
  let a = apps.get(&request.app_id).unwrap();
  let mut canister_ids: Vec<Canister> = Vec::new();

  for e in &a.canisters {
    canister_ids.push(e.clone())
  }

  for c in &canister_ids {
    ic_cdk::println!("---- begin uninstall code with canister id {:?}", c);
    uninstall_code(&c.canister_id).await?;
    ic_cdk::println!("---- end uninstall code with canister id {:?}", c);
  }

  match remove_app(a.clone()) {
    Ok(_) => {
      ic_cdk::println!("---- remove app while uninstall code with app id {:?}", a.app_id);
      Ok(true)
    }
    Err(_) => {
      Err(EgoWalletError::WalletError(String::from("uninstall wallet error")))
    }
  }
}

impl Wallet {
  pub fn new() -> Self {
    Wallet {
      apps: Default::default(),
      store_canister: Principal::anonymous(),
      cron_canister: Principal::anonymous(),
      wallet_version: Default::default(),
    }
  }

  pub async fn init_wallet_canister(&mut self, req: InitWalletCanister) -> Result<(), EgoError> {
    self.store_canister = req.store_canister_id;
    self.cron_canister = req.cron_canister_id;
    self.wallet_version = req.wallet_version;
    //TODO: add cron task in batch
    let _res = registry_cron_task_from_wallet(req).await;
    Ok(())
  }


  // pub fn is_manager(&self, caller: Principal) -> bool {
  //     let name_opt = self.managers.get(&caller);
  //     match name_opt {
  //         Some(_) => true,
  //         None => false,
  //     }
  // }

  pub fn get_apps(&self) -> UserAppResponse {
    UserAppResponse {
      canisters: self.apps.iter().map(|(_, val)| val.clone()).collect_vec(),
    }
  }

  pub fn wallet_version(&self) -> Option<Version> {
    Some(self.wallet_version.clone())
  }

  pub async fn get_app(&self, req: GetAppRequest) -> Result<GetAppResponse, EgoError> {
    let result = ic_cdk::call(
      self.store_canister.clone(),
      "get_app",
      (GetAppRequest { app_id: req.app_id }, ),
    )
      .await
      as Result<(Result<GetAppResponse, EgoStoreErr>, ), _>;


    let res: Result<GetAppResponse, EgoError> = match result.unwrap().0 {
      Ok(r) => Ok(r),
      Err(err) => {
        Err(err.into())
      }
    };
    res
  }

  pub async fn check_apps_status(&mut self) -> Result<CheckAppStatusResponse, EgoError> {
    ic_cdk::println!("---- enter check_apps_status");
    let apps = self.apps.clone();
    let mut resp: Vec<CheckAppStatus> = Vec::new();
    for (app_id, user_app) in apps {
      let canisters = &user_app.canisters;
      for _canister in canisters {
        match canister_status(&_canister.canister_id).await {
          Ok(x) => {
            let status = CheckAppStatus {
              native_status: x,
              app_id: app_id.clone(),
              canister_id: _canister.canister_id,
              canister_type: _canister.canister_type.clone(),
            };
            resp.push(status);
          }
          Err(e) => {
            return Err(e);
          }
        }
      }
    }
    Ok(CheckAppStatusResponse { app_status_result: resp })
  }
}
