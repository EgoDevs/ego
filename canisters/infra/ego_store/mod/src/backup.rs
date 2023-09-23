/********************  methods for backup_lib  ********************/
use ego_backup::backup_info::{BackupJob, ByteReadResponse};
use ego_backup::util::{get_bin_result, get_json_result};
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
pub fn record_export(name: String, start: usize, end: usize, last_update: u64) -> Option<ByteReadResponse> {
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
      let records = EgoStoreApp::by_last_update(start, end, last_update);
      get_json_result(&records)
    }
    "tenants" => {
      let records = Tenant::list(start, end);
      get_json_result(&records)
    }
    "wallet_providers" => {
      let records = WalletProvider::list(start, end);
      get_json_result(&records)
    }
    "wallets" => {
      let records = Wallet::by_last_update(start, end, last_update);
      get_json_result(&records)
    }
    "user_apps" => {
      let records = UserApp::by_last_update(start, end, last_update);
      get_json_result(&records)
    }
    "orders" => {
      let records = Order::by_last_update(start, end, last_update);
      get_json_result(&records)
    }
    "cash_flows" => {
      let records = CashFlow::by_last_update(start, end, last_update);

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
    "ego_store_apps" => {
      let records = EgoStoreApp::list(start, end);
      get_bin_result(&records)
    }
    "tenants" => {
      let records = Tenant::list(start, end);
      get_bin_result(&records)
    }
    "wallet_providers" => {
      let records = WalletProvider::list(start, end);
      get_bin_result(&records)
    }
    "wallets" => {
      let records = Wallet::list(start, end);
      get_bin_result(&records)
    }
    "user_apps" => {
      let records = UserApp::list(start, end);
      get_bin_result(&records)
    }
    "orders" => {
      let records = Order::list(start, end);
      get_bin_result(&records)
    }
    "cash_flows" => {
      let records = CashFlow::list(start, end);

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
    "ego_store_apps" => {
      let mut records: Vec<EgoStoreApp> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "tenants" => {
      let mut records: Vec<Tenant> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "wallet_providers" => {
      let mut records: Vec<WalletProvider> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "wallets" => {
      let mut records: Vec<Wallet> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "user_apps" => {
      let mut records: Vec<UserApp> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "orders" => {
      let mut records: Vec<Order> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    "cash_flows" => {
      let mut records: Vec<CashFlow> = candid::decode_one(data.as_slice()).unwrap();

      records.iter_mut().for_each(|record| {
        record.save();
      })
    }
    _ => trap("no job matched")
  };

  Ok(())
}