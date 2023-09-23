/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ego_backup::util::{get_bin_result, get_json_result};
use ic_cdk::trap;

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
pub fn record_export(name: String, start: usize, end: usize, last_update: u64) -> Option<ByteReadResponse> {
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
      let records = EgoDevApp::by_last_update(start, end, last_update);
      get_json_result(&records)
    }
    "files" => {
      let records = File::list(start, end);
      get_json_result(&records)
    }
    "developers" => {
      let records = Developer::by_last_update(start, end, last_update);
      get_json_result(&records)
    }
    "app_versions" => {
      let records = AppVersion::by_last_update(start, end, last_update);
      get_json_result(&records)
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

/********************  methods for backup/restore  ********************/
pub fn data_backup(name: String, start: usize, end: usize) -> Option<ByteReadResponse> {
  let (data, total) = match name.as_str() {
    "config" => {
      let records = StableState {
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        cycle_info: Some(cycle_info_pre_upgrade()),
        backup_info: Some(backup_info_pre_upgrade()),
        seq: Some(seq_pre_upgrade()),
      };
      let data = candid::encode_one(&records).unwrap();
      (data, 1)
    }
    "ego_dev_apps" => {
      let records = EgoDevApp::list(start, end);
      get_bin_result(&records)
    }
    "files" => {
      let records = File::list(start, end);
      get_bin_result(&records)
    }
    "developers" => {
      let records = Developer::list(start, end);
      get_bin_result(&records)
    }
    "app_versions" => {
      let records = AppVersion::list(start, end);
      get_bin_result(&records)
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

pub fn data_restore(name: String, data: Vec<u8>) -> Result<(), String> {
  match name.as_str() {
    "config" => {
      let config: StableState = candid::decode_one(data.as_slice()).unwrap();
      StableState::restore(config);
    }
    "ego_dev_apps" => {
      let mut records: Vec<EgoDevApp> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "files" => {
      let mut records: Vec<File> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "developers" => {
      let mut records: Vec<Developer> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "app_versions" => {
      let mut records: Vec<AppVersion> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    _ => trap("no job matched")
  };

  Ok(())
}