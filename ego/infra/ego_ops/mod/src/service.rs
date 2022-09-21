use ic_types::Principal;
use ego_types::app::{AppId, CanisterType, FileId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;
use crate::c2c::ego_dev::{EgoDev, TEgoDev};
use crate::c2c::ego_file::{EgoFile, TEgoFile};
use crate::c2c::ego_store::{EgoStore, TEgoStore};
use crate::c2c::ego_user::{EgoUser, TEgoUser};
use crate::c2c::ic_management::{IcManagement, TIcManagement};
use crate::state::EGO_OPS;

pub struct EgoOpsService {}

impl EgoOpsService {
  pub async fn canister_main_create(app_id: AppId, version: Version, data: Vec<u8>, hash: String) -> Result<bool, EgoError> {
    if get_md5(&data) == hash {
      let ic_management = IcManagement::new();

      ic_cdk::println!("==> 1. create canister");
      let canister_id = ic_management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;

      ic_cdk::println!("==> 2. install code");
      ic_management.canister_code_install(canister_id, data.clone()).await?;

      ic_cdk::println!("==> 3. add the canister to ego_ops");
      EGO_OPS.with(|ego_ops| {
        ego_ops.borrow_mut().app_canister_register(app_id.clone(), canister_id)
      });
    }


    // ic_cdk::println!("==> 4. upload wasm to ego_file if exists");
    // if let Some(canister_id) = EGO_OPS.with(|ego_ops| {
    //   match ego_ops.borrow().canisters.get("ego_file") {
    //     None => None,
    //     Some(can_ids) => Some(can_ids.get(0).unwrap().clone())
    //   }
    // }) {
    //   let fid = fid(app_id.clone(), version, CanisterType::BACKEND);
    //   ego_file.file_main_write(canister_id, fid, hash, data).await?;
    // }
    //
    // ic_cdk::println!("==> 5. call post install script of the newly created canister");
    // EgoOpsService::canister_installed(app_id.clone(), canister_id).await?;

    Ok(true)
  }

  pub async fn canister_main_register(ego_dev_id: Principal, ego_store_id: Principal, ego_file_id: Principal, ego_tenant_id: Principal) -> Result<bool, EgoError>{
    let ego_user = EgoUser::new();
    let ego_dev = EgoDev::new();
    let ego_store = EgoStore::new();

    // ego_dev
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register("ego_dev".to_string(), ego_dev_id)
    });

    ego_dev.admin_ego_file_add(ego_dev_id, ego_file_id).await?;
    ego_dev.admin_ego_store_set(ego_dev_id, ego_store_id).await?;

    // ego_file
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register("ego_file".to_string(), ego_file_id)
    });

    ego_user.role_user_add(ego_file_id, ego_dev_id).await?;
    ego_user.role_user_add(ego_file_id, ego_tenant_id).await?;

    // ego_store
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register("ego_store".to_string(), ego_store_id)
    });
    ego_store.admin_egp_tenant_add(ego_store_id, ego_tenant_id).await?;

    // ego_tenant
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register("ego_tenant".to_string(), ego_tenant_id)
    });
    ego_user.role_user_add(ego_tenant_id, ego_store_id).await?;

    Ok(true)
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}


