/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ic_cdk::trap;

use ego_utils::util::get_md5;

use crate::state::{backup_info_pre_upgrade, cycle_info_pre_upgrade, info_log_add, registry_pre_upgrade, seq_pre_upgrade, users_pre_upgrade};
use crate::types::app_version::AppVersion;
use crate::types::developer::Developer;
use crate::types::ego_dev_app::EgoDevApp;
use crate::types::ego_file::EgoFile;
use crate::types::stable_state::StableState;

pub fn job_list() -> Vec<BackupJob> {
  let mut jobs = vec![];

  jobs.push(BackupJob {
    name: "config".to_string(),
    amount: 1 as usize,
  });

  jobs.push(BackupJob {
    name: "ego_dev_apps".to_string(),
    amount: EgoDevApp::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "files".to_string(),
    amount: EgoFile::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "developers".to_string(),
    amount: Developer::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "app_versions".to_string(),
    amount: AppVersion::list().len() as usize,
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
        seq: Some(seq_pre_upgrade()),
      };
      serde_json::to_vec(&data).unwrap()
    }
    "ego_dev_apps" => {
      let data = match last_update {
        None => {
          EgoDevApp::list()
        }
        Some(ts) => {
          EgoDevApp::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    "files" => {
      let data = EgoFile::list();
      serde_json::to_vec(&data).unwrap()
    }
    "developers" => {
      let data = match last_update {
        None => {
          Developer::list()
        }
        Some(ts) => {
          Developer::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    "app_versions" => {
      let data = match last_update {
        None => {
          AppVersion::list()
        }
        Some(ts) => {
          AppVersion::by_last_update(ts)
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
