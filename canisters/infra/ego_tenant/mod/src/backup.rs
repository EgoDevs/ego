/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ic_cdk::trap;
use serde::Serialize;

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
pub fn record_export(name: String, start: usize, end: usize, last_update: Option<u64>) -> Option<ByteReadResponse> {
  info_log_add(format!("record_export name: {}, start: {}, end: {}, last_update: {:?}", name, start, end, last_update).as_str());
  let (data, total) = match name.as_str() {
    "config" => {
      let records = StableState {
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        cycle_info: Some(cycle_info_pre_upgrade()),
        backup_info: Some(backup_info_pre_upgrade()),
      };
      let data = serde_json::to_vec(&records).unwrap();

      (data, 1)
    }
    "tasks" => {
      let records = match last_update {
        None => {
          Task::list()
        }
        Some(ts) => {
          Task::by_last_update(ts)
        }
      };
      get_result(&records, start, end)
    }
    _ => trap("no job matched")
  };

  let hash = get_md5(&data);
  let resp = ByteReadResponse {
    name: name.clone(),
    data,
    hash,
    total,
  };

  Some(resp)
}

fn get_result<T: Serialize>(records: &Vec<T>, mut start: usize, mut end: usize) -> (Vec<u8>, usize) {
  let total = records.len();
  if start > total {
    start = 0
  }
  if end > total {
    end = total
  }
  let data = serde_json::to_vec(&records[start..end]).unwrap();
  (data, total)
}
