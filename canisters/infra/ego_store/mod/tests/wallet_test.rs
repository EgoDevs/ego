use candid::Principal;

use ego_store_mod::types::tenant::Tenant;
use ego_store_mod::types::wallet::Wallet;
use ego_utils::util::time;

static TENANT_ID: &str = "2avdy-paaaa-aaaaf-abcga-cai";

static WALLET_ID1: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static USER_ID1: &str = "o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae";

static WALLET_ID2: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static USER_ID2: &str = "223xb-saaaa-aaaaf-arlqa-cai";

pub fn set_up() {
  let tenant_id = Principal::from_text(TENANT_ID.to_string()).unwrap();

  // add tenant
  let tenant = Tenant::new(&tenant_id);
  tenant.save();

  // add wallet
  let wallet_id = Principal::from_text(WALLET_ID1).unwrap();
  let user_id = Principal::from_text(USER_ID1).unwrap();

  let mut wallet = Wallet::new(&tenant_id, &wallet_id, &user_id);
  wallet.save();
}

#[test]
fn new() {
  set_up();
  assert_eq!(1, Wallet::len());

  let tenant_id = Principal::from_text(TENANT_ID.to_string()).unwrap();
  let wallet_id = Principal::from_text(WALLET_ID2).unwrap();
  let user_id = Principal::from_text(USER_ID2).unwrap();

  let mut wallet = Wallet::new(&tenant_id, &wallet_id, &user_id);
  wallet.save();

  assert_eq!(2, Wallet::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, Wallet::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(1, Wallet::by_last_update(0, 100, now).len());
}

#[test]
pub fn list() {
  set_up();

  let wallet1 = Principal::from_text(WALLET_ID1.to_string()).unwrap();
  let wallets = Wallet::list(0, 100);

  assert_eq!(1, wallets.len());
  assert_eq!(wallet1, wallets.get(0).unwrap().wallet_id);
}

#[test]
pub fn get() {
  set_up();

  let wallet1 = Principal::from_text(WALLET_ID1.to_string()).unwrap();
  let wallet = Wallet::get(&wallet1);
  assert!(wallet.is_some());

  let wallet2 = Principal::from_text(WALLET_ID2.to_string()).unwrap();
  let wallet = Wallet::get(&wallet2);
  assert!(wallet.is_none());
}
