use candid::Principal;

use ego_store_mod::types::wallet_provider::WalletProvider;

static APP_NAME: &str = "app1";
static WALLET_PROVIDER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";

static APP_NAME2: &str = "test";
static WALLET_PROVIDER_ID2: &str = "225da-yaaaa-aaaah-qahrq-cai";


pub fn set_up() {
  // add wallet provider
  let wallet_provider_id = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();
  let wallet_provider = WalletProvider::new(&wallet_provider_id, &APP_NAME.to_string());
  wallet_provider.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(1, WalletProvider::len());

  let wallet_provider_id = Principal::from_text(WALLET_PROVIDER_ID2.to_string()).unwrap();
  let wallet_provider = WalletProvider::new(&wallet_provider_id, &APP_NAME2.to_string());
  wallet_provider.save();

  assert_eq!(2, WalletProvider::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(1, WalletProvider::len());
}

#[test]
pub fn list() {
  set_up();

  let wallet_providers = WalletProvider::list(0, 100);

  assert_eq!(1, wallet_providers.len());
  assert_eq!(WALLET_PROVIDER_ID, wallet_providers.get(0).unwrap().wallet_provider.to_string());
}

#[test]
pub fn get() {
  set_up();

  let wallet_provider_id = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();
  let result = WalletProvider::get(&wallet_provider_id);
  assert!(result.is_some());

  let wallet_provider_id = Principal::from_text(WALLET_PROVIDER_ID2.to_string()).unwrap();
  let result = WalletProvider::get(&wallet_provider_id);
  assert!(result.is_none());
}

#[test]
pub fn remove() {
  set_up();

  assert_eq!(1, WalletProvider::len());

  let wallet_provider_id = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();
  WalletProvider::remove(&wallet_provider_id);

  assert_eq!(0, WalletProvider::len());
}