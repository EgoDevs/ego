use astrox_macros::{inject_canister_log, inject_canister_registry};
use astrox_macros::inject_canister_users;

use ego_macros::inject_log;
use ego_types::app::{AppId, Category, DeployMode};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;

use crate::c2c::ego_dev::TEgoDev;

inject_log!();
inject_canister_users!();
inject_canister_registry!();
inject_canister_log!();

/********************  methods for ego_registry   ********************/
fn on_canister_added(name: &str, canister_id: Principal) {
  ego_log(&format!("on_canister_added name: {}, canister_id: {}", name, canister_id));
}

pub struct EgoOpsService {}

impl EgoOpsService {
  pub fn admin_app_create<T: TEgoDev>(
    ego_dev: T,
    ego_dev_id: Principal,
    app_id: AppId,
    name: String,
    version: Version,
    category: Category,
    logo: String,
    description: String,
    backend_data: Vec<u8>,
    backend_hash: String,
    frontend: Option<Principal>,
    deploy_mode: DeployMode,
  ) -> Result<bool, EgoError> {
    if get_md5(&backend_data) == backend_hash {
      ego_dev
        .admin_app_create(
          ego_dev_id,
          app_id.clone(),
          name,
          version,
          category,
          logo,
          description,
          backend_data,
          backend_hash,
          frontend,
          deploy_mode,
        );

      Ok(true)
    } else {
      Ok(false)
    }
  }
}

fn get_md5(data: &Vec<u8>) -> String {
  let digest = md5::compute(data);
  return format!("{:?}", digest);
}
