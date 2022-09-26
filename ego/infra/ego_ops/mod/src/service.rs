use ic_types::Principal;
use ego_types::app::{AppId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use crate::c2c::c2c_types::CronInterval;
use crate::c2c::ego_dev::{EgoDev, TEgoDev};
use crate::c2c::ego_store::{EgoStore, TEgoStore};
use crate::c2c::ego_cron::{EgoCron, TEgoCron};
use crate::c2c::ego_user::{EgoUser, TEgoUser};
use crate::state::EGO_OPS;

pub struct EgoOpsService {}

impl EgoOpsService {
  pub fn canister_main_register(app_id: String, canister_id: Principal) {
    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register(app_id, canister_id)
    });
  }

  pub async fn canister_relation_update()  -> Result<bool, EgoError>{
    let ego_user = EgoUser::new();
    let ego_dev = EgoDev::new();
    let ego_store = EgoStore::new();
    let ego_cron = EgoCron::new();

    let canisters = EGO_OPS.with(|ego_ops| {
      ego_ops.borrow().canisters.clone()
    });

    let ego_dev_id = canisters.get("ego_dev").unwrap().get(0).unwrap();
    let ego_store_id = canisters.get("ego_store").unwrap().get(0).unwrap();
    let ego_file_ids = canisters.get("ego_file").unwrap();
    let ego_tenant_ids = canisters.get("ego_tenant").unwrap();
    let ego_cron_id = canisters.get("ego_cron").unwrap().get(0).unwrap();
    let ego_ledger_id = canisters.get("ego_ledger").unwrap().get(0).unwrap();


    // ego_dev
    for ego_file_id in ego_file_ids {
      ego_dev.admin_ego_file_add(ego_dev_id.clone(), ego_file_id.clone()).await?;
    }
    ego_dev.admin_ego_store_set(ego_dev_id.clone(), ego_store_id.clone()).await?;

    // ego_file
    for ego_file_id in ego_file_ids {
      ego_user.role_user_add(ego_file_id.clone(), ego_dev_id.clone()).await?;
      for ego_tenant_id in ego_tenant_ids {
        ego_user.role_user_add(ego_file_id.clone(), ego_tenant_id.clone()).await?;
      }
    }

    // ego_store
    for ego_tenant_id in ego_tenant_ids {
      ego_store.admin_egp_tenant_add(ego_store_id.clone(), ego_tenant_id.clone()).await?;
    }
    ego_store.wallet_main_new(ego_store_id.clone()).await?;

    // ego_tenant
    for ego_tenant_id in ego_tenant_ids {
      ego_store.admin_egp_tenant_add(ego_tenant_id.clone(), ego_store_id.clone()).await?;

      ego_user.role_user_add(ego_tenant_id.clone(), ego_cron_id.clone()).await?;
      ego_cron.task_main_add(ego_cron_id.clone(), ego_tenant_id.clone(), "message_main_notify".to_string(), CronInterval::PerSecond).await?;
    }

    // ego_ledger
    ego_user.role_user_add(ego_ledger_id.clone(), ego_cron_id.clone()).await?;
    ego_cron.task_main_add(ego_cron_id.clone(), ego_ledger_id.clone(), "message_main_notify".to_string(), CronInterval::PerSecond).await?;


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


