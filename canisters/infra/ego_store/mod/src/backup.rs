/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ic_cdk::trap;
use serde::Serialize;

use ego_utils::util::get_md5;

use crate::state::{backup_info_pre_upgrade, cycle_info_pre_upgrade, info_log_add, registry_pre_upgrade, seq_pre_upgrade, users_pre_upgrade};
use crate::types::cash_flow::CashFlow;
use crate::types::ego_store_app::EgoStoreApp;
use crate::types::order::Order;
use crate::types::stable_state::StableState;
use crate::types::tenant::Tenant;
use crate::types::user_app::UserApp;
use crate::types::wallet::Wallet;
use crate::types::wallet_provider::WalletProvider;

pub fn job_list() -> Vec<BackupJob> {
  let mut jobs = vec![];

  jobs.push(BackupJob {
    name: "config".to_string(),
    amount: 1 as usize,
  });

  jobs.push(BackupJob {
    name: "ego_store_apps".to_string(),
    amount: EgoStoreApp::len() as usize,
  });

  jobs.push(BackupJob {
    name: "tenants".to_string(),
    amount: Tenant::len() as usize,
  });

  jobs.push(BackupJob {
    name: "wallet_providers".to_string(),
    amount: WalletProvider::len() as usize,
  });

  jobs.push(BackupJob {
    name: "wallets".to_string(),
    amount: Wallet::len() as usize,
  });

  jobs.push(BackupJob {
    name: "user_apps".to_string(),
    amount: UserApp::len() as usize,
  });

  jobs.push(BackupJob {
    name: "orders".to_string(),
    amount: Order::len() as usize,
  });

  jobs.push(BackupJob {
    name: "cash_flows".to_string(),
    amount: CashFlow::len() as usize,
  });

  jobs
}

/********************  methods for export  ********************/
pub fn record_export(name: String, start: usize, end: usize, last_update: Option<u64>) -> Option<ByteReadResponse> {
  info_log_add(format!("record_export name: {}, start: {}, end: {}, last_update: {:?}", name, start, end, last_update).as_str());
  let (data, total) = match name.as_str() {
    "config" => {
      let data = StableState {
        users: Some(users_pre_upgrade()),
        registry: Some(registry_pre_upgrade()),
        cycle_info: Some(cycle_info_pre_upgrade()),
        backup_info: Some(backup_info_pre_upgrade()),
        seq: Some(seq_pre_upgrade()),
      };
      let data = serde_json::to_vec(&data).unwrap();
      (data, 1)
    }
    "ego_store_apps" => {
      let records = match last_update {
        None => {
          EgoStoreApp::list()
        }
        Some(ts) => {
          EgoStoreApp::by_last_update(ts)
        }
      };
      get_result(&records, start, end)
    }
    "tenants" => {
      let records = Tenant::list();
      get_result(&records, start, end)
    }
    "wallet_providers" => {
      let records = WalletProvider::list();
      get_result(&records, start, end)
    }
    "wallets" => {
      let records = match last_update {
        None => {
          Wallet::list()
        }
        Some(ts) => {
          Wallet::by_last_update(ts)
        }
      };

      get_result(&records, start, end)
    }
    "user_apps" => {
      let records = match last_update {
        None => {
          UserApp::list()
        }
        Some(ts) => {
          UserApp::by_last_update(ts)
        }
      };

      get_result(&records, start, end)
    }
    "orders" => {
      let records = match last_update {
        None => {
          Order::list()
        }
        Some(ts) => {
          Order::by_last_update(ts)
        }
      };

      get_result(&records, start, end)
    }
    "cash_flows" => {
      let records = match last_update {
        None => {
          CashFlow::list()
        }
        Some(ts) => {
          CashFlow::by_last_update(ts)
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