/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ic_cdk::trap;

use ego_utils::util::get_md5;

use crate::state::{backup_info_pre_upgrade, cycle_info_pre_upgrade, info_log_add, registry_pre_upgrade, users_pre_upgrade};
use crate::types::stable_state::StableState;
use crate::types::task::Task;

pub fn job_list() -> Vec<BackupJob> {
  let mut jobs = vec![];

  jobs.push(BackupJob {
    name: "config".to_string(),
    amount: 1 as usize,
  });

  jobs.push(BackupJob {
    name: "tasks".to_string(),
    amount: Task::len() as usize,
  });

  jobs
}

/********************  methods for export  ********************/
pub fn record_export(name: String, last_update: Option<u64>) -> Option<ByteReadResponse> {
  info_log_add(format!("record_export name: {}, last_update: {:?}", name, last_update).as_str());
  let data = match name.as_str() {
    "config" => {
      let data = StableState {
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        cycle_info: Some(cycle_info_pre_upgrade()),
        backup_info: Some(backup_info_pre_upgrade()),
      };
      serde_json::to_vec(&data).unwrap()
    }
    "tasks" => {
      let data = match last_update {
        None => {
          Task::list()
        }
        Some(ts) => {
          Task::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    _ => trap("no job matched")
  };

  let hash = get_md5(&data);

  let resp = ByteReadResponse {
    name: name.clone(),
    data,
    hash,
  };

  Some(resp)
}
