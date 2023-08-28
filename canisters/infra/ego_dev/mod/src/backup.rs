/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ic_cdk::trap;
use serde::Serialize;

use ego_utils::util::get_md5;

use crate::state::{backup_info_pre_upgrade, cycle_info_pre_upgrade, info_log_add, registry_pre_upgrade, seq_pre_upgrade, users_pre_upgrade};
use crate::types::app_version::AppVersion;
use crate::types::developer::Developer;
use crate::types::ego_dev_app::EgoDevApp;
use crate::types::file::File;
use crate::types::stable_state::StableState;

pub fn job_list() -> Vec<BackupJob> {
  let mut jobs = vec![];

  jobs.push(BackupJob {
    name: "config".to_string(),
    amount: 1 as usize,
  });

  jobs.push(BackupJob {
    name: "ego_dev_apps".to_string(),
    amount: EgoDevApp::len() as usize,
  });

  jobs.push(BackupJob {
    name: "files".to_string(),
    amount: File::len() as usize,
  });

  jobs.push(BackupJob {
    name: "developers".to_string(),
    amount: Developer::len() as usize,
  });

  jobs.push(BackupJob {
    name: "app_versions".to_string(),
    amount: AppVersion::len() as usize,
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
        seq: Some(seq_pre_upgrade()),
      };
      let data = serde_json::to_vec(&records).unwrap();
      (data, 1)
    }
    "ego_dev_apps" => {
      let records = match last_update {
        None => {
          EgoDevApp::list()
        }
        Some(ts) => {
          EgoDevApp::by_last_update(ts)
        }
      };
      get_result(&records, start, end)
    }
    "files" => {
      let records = File::list();
      get_result(&records, start, end)
    }
    "developers" => {
      let records = match last_update {
        None => {
          Developer::list()
        }
        Some(ts) => {
          Developer::by_last_update(ts)
        }
      };
      get_result(&records, start, end)
    }
    "app_versions" => {
      let records = match last_update {
        None => {
          AppVersion::list()
        }
        Some(ts) => {
          AppVersion::by_last_update(ts)
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