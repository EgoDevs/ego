use ego_types::app::{AppId, CanisterType, FileId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;
use crate::c2c::ego_file::TEgoFile;
use crate::c2c::ic_management::TIcManagement;
use crate::state::EGO_OPS;

pub struct EgoOpsService {}

impl EgoOpsService {
  pub async fn canister_main_create<E: TEgoFile, M: TIcManagement>(ego_file: E, ic_management: M, app_id: AppId, version: Version, data: Vec<u8>, hash: String) -> Result<bool, EgoError> {
    ic_cdk::println!("1 create canister");
    let canister_id = ic_management.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await?;

    ic_cdk::println!("2 install code");
    ic_management.canister_code_install(canister_id, data.clone()).await?;

    EGO_OPS.with(|ego_ops| {
      ego_ops.borrow_mut().app_canister_register(app_id.clone(), canister_id)
    });

    if let Some(canister_id) = EGO_OPS.with(|ego_ops| {
      match ego_ops.borrow().canisters.get("ego_file") {
        None => None,
        Some(can_ids) => Some(can_ids.get(0).unwrap().clone())
      }
    }) {
      let fid = fid(app_id.clone(), version, CanisterType::BACKEND);
      ego_file.file_main_write(canister_id, fid, hash, data).await
    } else {
      Ok(true)
    }
  }
}

fn fid(app_id: AppId, version: Version, canister_type: CanisterType) -> FileId {
  let data = &format!("{}|{}|{}", app_id, canister_type, version.to_string()).into_bytes();
  let digest = md5::compute(data);
  format!("{:?}", digest)
}
