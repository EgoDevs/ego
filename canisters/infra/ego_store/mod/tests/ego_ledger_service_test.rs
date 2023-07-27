use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use mockall::mock;

use ego_store_mod::c2c::ego_ledger::TEgoLedger;
use ego_store_mod::order::{Order, OrderStatus};
use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;
use ego_store_mod::types::wallet::Wallet;

static LEDGER_ID: &str = "22k5f-nqaaa-aaaad-qaigq-cai";
static STORE_ID: &str = "22cl3-kqaaa-aaaaf-add7q-cai";
static TEST_OPERATOR: &str = "c5jhr-faaaa-aaaaf-acebq-cai";
static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
static EXISTS_TENANT_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";
static TEST_WALLET_ID: &str = "5vreg-2yaaa-aaaaf-ajkdq-cai";

mock! {
  Ledger{}

  impl TEgoLedger for Ledger{
    fn ledger_payment_add(&self, order: &Order);
  }
}

pub fn set_up() {
    let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
    let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
    let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
    let store_principal = Principal::from_text(STORE_ID.to_string()).unwrap();

    EGO_STORE.with(|ego_store| {
        // add wallet
        let mut wallet = Wallet::new(tenant_principal, wallet_principal, user_principal);

        // add order
        let order = Order::new(wallet_principal, &store_principal, 1.2f32, 10);
        ego_store.borrow_mut().orders.insert(Memo(10), order);

        wallet.orders.push(Memo(10));

        ego_store
            .borrow_mut()
            .wallets
            .insert(wallet_principal, wallet);
    });
}

#[test]
fn wallet_order_new() {
    set_up();

    let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
    let store_id = Principal::from_text(STORE_ID).unwrap();

    // get order list before make order
    let result = EgoStoreService::wallet_order_list(exist_wallet_id);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().len());

    // create order
    let mut ego_ledger = MockLedger::new();
    ego_ledger.expect_ledger_payment_add().returning(|_| ());

    let result = EgoStoreService::wallet_order_new(ego_ledger, exist_wallet_id, store_id, 1.2);
    assert!(result.is_ok());
    assert_eq!(1, result.unwrap().memo.0);

    // get order list after make order
    let result = EgoStoreService::wallet_order_list(exist_wallet_id);
    assert!(result.is_ok());
    assert_eq!(2, result.unwrap().len());
}

#[test]
fn wallet_order_notify() {
    set_up();

    let ledger_principal = Principal::from_text(LEDGER_ID.to_string()).unwrap();
    let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
    // get order list before make order
    let orders = EgoStoreService::wallet_order_list(exist_wallet_id).unwrap();
    assert_eq!(1, orders.len());

    EGO_STORE.with(|ego_store| {
        assert_eq!(
            0,
            ego_store
                .borrow()
                .wallets
                .get(&exist_wallet_id)
                .unwrap()
                .cycles
        );
    });

    let order = orders.get(0).unwrap();
    assert_eq!(OrderStatus::NEW, order.status);

    // notify order
    let result = EgoStoreService::wallet_order_notify(order.memo, ledger_principal, 1234567890);
    assert!(result.is_ok());

    // get order list after make order
    let orders = EgoStoreService::wallet_order_list(exist_wallet_id).unwrap();
    assert_eq!(1, orders.len());

    let order = orders.get(0).unwrap();
    assert_eq!(OrderStatus::SUCCESS, order.status);

    EGO_STORE.with(|ego_store| {
        assert_eq!(
            (1.2f32 * 1000000f32) as u128,
            ego_store
                .borrow()
                .wallets
                .get(&exist_wallet_id)
                .unwrap()
                .cycles
        );
    });
}

#[test]
fn wallet_order_list_wallet_not_exists() {
    // set_up();
    let test_wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();

    //ego store wallet not exists
    let result = EgoStoreService::wallet_order_list(test_wallet_id);
    assert!(result.is_err());
    assert_eq!(3006, result.as_ref().unwrap_err().code);
    assert_eq!(
        "ego-store: wallet not exists",
        result.as_ref().unwrap_err().msg
    );
}

#[test]
fn wallet_order_new_wallet_not_exists() {
    // set_up();
    let mut ego_ledger = MockLedger::new();
    ego_ledger.expect_ledger_payment_add().returning(|_| ());
    let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
    let store_id = Principal::from_text(STORE_ID).unwrap();
    // wallet not exists
    let result = EgoStoreService::wallet_order_new(ego_ledger, wallet_id, store_id, 1.5f32);
    assert!(result.is_err());
    assert_eq!(3006, result.as_ref().unwrap_err().code);
}

#[test]
fn wallet_cycle_charge_wallet_not_exists() {
    set_up();
    let wallet_id = Principal::from_text(TEST_WALLET_ID).unwrap();
    let ledger_id = Principal::from_text(TEST_OPERATOR).unwrap();

    // wallet not exists
    let result = EgoStoreService::wallet_cycle_charge(
        wallet_id,
        128,
        ledger_id,
        64,
        "charge cycle".to_string(),
    );
    assert!(result.is_err());
    assert_eq!(3006, result.as_ref().unwrap_err().code);
    assert_eq!(
        "ego-store: wallet not exists",
        result.as_ref().unwrap_err().msg
    );
}

#[test]
fn wallet_cycle_charge() {
    set_up();
    let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
    let ledger_id = Principal::from_text(LEDGER_ID).unwrap();
    // wallet charge cycle
    let result = EgoStoreService::wallet_cycle_charge(
        wallet_id,
        128,
        ledger_id,
        64,
        "charge cycle".to_string(),
    );
    assert!(result.is_ok());
}
