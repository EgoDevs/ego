use ic_types::Principal;
use ego_types::app::{AppId, CanisterType, FileId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;
use crate::c2c::ego_dev::{EgoDev, TEgoDev};
use crate::c2c::ego_file::{EgoFile, TEgoFile};
use crate::c2c::ic_management::{IcManagement, TIcManagement};
use crate::state::EGO_OPS;

pub struct EgoOpsService {}

impl EgoOpsService {
  pub async fn canister_main_create(app_id: AppId, version: Version, data: Vec<u8>, hash: String) -> Result<bool, EgoError> {
    let ic_management = IcManagement::new();
    let ego_file = EgoFile::new();

    ic_cdk::println!("==> 1. create canister");
    let canister_id = ic_management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;

    ic_cdk::println!("==> 2. install code");
    ic_management.canister_code_install(canister_id, data.clone()).await?;

    ic_cdk::println!("==> 3. add the canister to ego_ops");
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register(app_id.clone(), canister_id)
    });

    ic_cdk::println!("==> 4. upload wasm to ego_file if exists");
    if let Some(canister_id) = EGO_OPS.with(|ego_ops| {
      match ego_ops.borrow().canisters.get("ego_file") {
        None => None,
        Some(can_ids) => Some(can_ids.get(0).unwrap().clone())
      }
    }) {
      let fid = fid(app_id.clone(), version, CanisterType::BACKEND);
      ego_file.file_main_write(canister_id, fid, hash, data).await?;
    }

    ic_cdk::println!("==> 5. call post install script of the newly created canister");
    EgoOpsService::canister_installed(app_id.clone(), canister_id).await?;

    Ok(true)
  }

  /// should be call when an ego_file canister is installed
  pub async fn canister_installed(app_id: AppId, canister_id: Principal) -> Result<bool, EgoError>{
    if app_id == "ego_file" {

    } else if app_id == "ego_tenant" {

    } else if app_id == "ego_dev" {
      let ego_dev = EgoDev::new();
      let ego_file_ids = EGO_OPS.with(|ego_ops| ego_ops.borrow().canisters.get("ego_file").unwrap().clone());
      for ego_file_id in ego_file_ids {
        ego_dev.admin_file_add(canister_id, ego_file_id.clone()).await?;
      }
    }

    Ok(true)
  }
}

fn fid(app_id: AppId, version: Version, canister_type: CanisterType) -> FileId {
  let data = &format!("{}|{}|{}", app_id, canister_type, version.to_string()).into_bytes();
  let digest = md5::compute(data);
  format!("{:?}", digest)
}
