use ic_cdk::export::Principal;
use ic_ledger_types::Memo;

use ego_lib::ego_canister::TEgoCanister;
use ego_store_mod::memory::WALLETS;
use ego_store_mod::types::wallet::Wallet;


static WALLET_ID: &str = "23vqh-waaaa-aaaai-qhcya-cai";
static TENANT_ID: &str = "2avdy-paaaa-aaaaf-abcga-cai";
static USER_ID: &str = "o2ivq-5dsz3-nba5d-pwbk2-hdd3i-vybeq-qfz35-rqg27-lyesf-xghzc-3ae";

#[test]
fn new() {
  WALLETS.with(|cell| {
    assert_eq!(0, cell.borrow().len());
  });

  let wallet_id = Principal::from_text(WALLET_ID).unwrap();
  let tenant_id = Principal::from_text(TENANT_ID).unwrap();
  let user_id = Principal::from_text(USER_ID).unwrap();
  Wallet::new(&wallet_id, &tenant_id, &user_id);

  WALLETS.with(|cell| {
    assert_eq!(1, cell.borrow().len());
  });
}

#[test]
fn app_install() {

}

#[test]
fn app_upgrade() {

}

#[test]
fn app_remove() {

}