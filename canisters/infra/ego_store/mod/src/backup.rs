/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ic_cdk::trap;

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
    amount: EgoStoreApp::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "tenants".to_string(),
    amount: Tenant::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "wallet_providers".to_string(),
    amount: WalletProvider::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "wallets".to_string(),
    amount: Wallet::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "user_apps".to_string(),
    amount: UserApp::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "orders".to_string(),
    amount: Order::list().len() as usize,
  });

  jobs.push(BackupJob {
    name: "cash_flows".to_string(),
    amount: CashFlow::list().len() as usize,
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
    "ego_store_apps" => {
      let data = match last_update {
        None => {
          EgoStoreApp::list()
        }
        Some(ts) => {
          EgoStoreApp::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    "tenants" => {
      let data = Tenant::list();
      serde_json::to_vec(&data).unwrap()
    }
    "wallet_providers" => {
      let data = WalletProvider::list();
      serde_json::to_vec(&data).unwrap()
    }
    "wallets" => {
      let data = match last_update {
        None => {
          Wallet::list()
        }
        Some(ts) => {
          Wallet::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    "user_apps" => {
      let data = match last_update {
        None => {
          UserApp::list()
        }
        Some(ts) => {
          UserApp::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    "orders" => {
      let data = match last_update {
        None => {
          Order::list()
        }
        Some(ts) => {
          Order::by_last_update(ts)
        }
      };
      serde_json::to_vec(&data).unwrap()
    }
    "cash_flows" => {
      let data = match last_update {
        None => {
          CashFlow::list()
        }
        Some(ts) => {
          CashFlow::by_last_update(ts)
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
