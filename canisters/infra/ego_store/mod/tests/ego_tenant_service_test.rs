use candid::Principal;

use ego_store_mod::service::EgoStoreService;
use ego_store_mod::types::user_app::UserApp;
use ego_store_mod::types::wallet::Wallet;
use ego_types::app::{App, Canister, CanisterType, Category, Version};

static EXISTS_APP_ID: &str = "app_exists";
static APP_NAME: &str = "app1";
static APP_LOGO: &str = "logo";
static APP_DESCRIPTION: &str = "test is app description";

static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
static EXISTS_USER_APP_BACKEND: &str = "224jh-lqaaa-aaaad-qaxda-cai";
static FAKE_USER_APP_BACKEND: &str = "223vg-sqaaa-aaaak-abtmq-cai";
static EXISTS_TENANT_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";

pub fn set_up() {
  let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
  let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();

  let version = Version::new(1, 0, 1);

  let app = App {
    app_id: EXISTS_APP_ID.to_string(),
    name: APP_NAME.to_string(),
    category: Category::Vault,
    logo: APP_LOGO.to_string(),
    description: APP_DESCRIPTION.to_string(),
    current_version: version,
    price: 0.0,
    app_hash: "".to_string(),
  };

  // add wallet
  let mut wallet = Wallet::new(&tenant_principal, &wallet_principal, &user_principal);
  wallet.cycles = 1000;
  wallet.save();

  // add user app
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let mut user_app = UserApp::new(&app,
                                  &Canister::new(backend_principal, CanisterType::BACKEND), Some(wallet.wallet_id));
  user_app.save();
}

#[test]
fn canister_charge_success() {
  set_up();

  let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
  let backend_principal = Principal::from_text(EXISTS_USER_APP_BACKEND.to_string()).unwrap();
  let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();

  let wallet = Wallet::get(&wallet_principal).unwrap();
  assert_eq!(1000, wallet.cycles);

  EgoStoreService::canister_cycle_charge(&backend_principal, 100, &tenant_principal, "cycle charge".to_string()).unwrap();

  let wallet = Wallet::get(&wallet_principal).unwrap();
  assert_eq!(900, wallet.cycles);
}

#[test]
#[should_panic]
fn canister_charge_failed() {
  set_up();

  let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
  let fake_backend_principal = Principal::from_text(FAKE_USER_APP_BACKEND.to_string()).unwrap();

  EgoStoreService::canister_cycle_charge(&fake_backend_principal, 100, &tenant_principal, "cycle charge".to_string()).unwrap();
}
