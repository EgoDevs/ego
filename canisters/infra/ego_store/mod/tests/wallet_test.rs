use candid::Principal;

use ego_store_mod::service::EgoStoreService;
use ego_store_mod::types::tenant::Tenant;
use ego_store_mod::types::wallet::Wallet;

static WALLET_ID: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static TENANT_ID: &str = "2avdy-paaaa-aaaaf-abcga-cai";
static USER_ID: &str = "o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae";

pub fn set_up() {
  let tenant_principal = Principal::from_text(TENANT_ID.to_string()).unwrap();

  // add tenant
  let tenant = Tenant::new(&tenant_principal);
  tenant.save();
}

#[test]
fn new() {
  set_up();
  assert_eq!(0, Wallet::list().len());

  let wallet_id = Principal::from_text(WALLET_ID).unwrap();
  let user_id = Principal::from_text(USER_ID).unwrap();

  EgoStoreService::wallet_main_register(&wallet_id, &user_id).unwrap();

  assert_eq!(1, Wallet::list().len());
}

#[test]
fn app_install() {}

#[test]
fn app_upgrade() {}

#[test]
fn app_remove() {}