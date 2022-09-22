use ic_types::Principal;
use ego_types::app::{AppId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use crate::c2c::ego_dev::{EgoDev, TEgoDev};
use crate::c2c::ego_store::{EgoStore, TEgoStore};
use crate::c2c::ego_user::{EgoUser, TEgoUser};
use crate::state::EGO_OPS;

pub struct EgoOpsService {}

impl EgoOpsService {
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
    ego_store.wallet_main_new(ego_store_id).await?;

    // ego_tenant
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register("ego_tenant".to_string(), ego_tenant_id)
    });
    ego_user.role_user_add(ego_tenant_id, ego_store_id).await?;

    Ok(true)
  }

  pub async fn admin_app_create(app_id: AppId, name: String, version: Version, backend_data: Vec<u8>, backend_hash: String, frontend: Option<Principal>) -> Result<bool, EgoError> {
    if get_md5(&backend_data) == backend_hash {
      let ego_dev_id = EGO_OPS.with(|ego_ops| ego_ops.borrow().canisters.get("ego_dev").unwrap().get(0).unwrap().clone());

      let ego_dev = EgoDev::new();
      ego_dev.admin_app_create(ego_dev_id, app_id.clone(), name, version, backend_data, backend_hash, frontend).await?;

      Ok(true)
    }else{
      Ok(false)
    }
  }

  pub async fn admin_app_deploy(app_id: AppId) -> Result<bool, EgoError> {
    let ego_store = EgoStore::new();

    let ego_store_id = EGO_OPS.with(|ego_ops| ego_ops.borrow().canisters.get("ego_store").unwrap().get(0).unwrap().clone());

    ego_store.wallet_app_install(ego_store_id, app_id.clone()).await?;

    if "ego_cron" == app_id {

    }

    Ok(true)
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}


