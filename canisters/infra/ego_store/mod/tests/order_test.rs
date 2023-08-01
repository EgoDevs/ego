use candid::Principal;
use ic_ledger_types::Memo;

use ego_store_mod::types::order::Order;
use ego_utils::util::time;

static WALLET1: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static WALLET2: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static WALLET3: &str = "wtb37-uyaaa-aaaai-qa3zq-cai";

static STORE: &str = "225da-yaaaa-aaaah-qahrq-cai";

pub fn set_up() {
  let store = Principal::from_text(STORE.to_string()).unwrap();

  let wallet1 = Principal::from_text(WALLET1.to_string()).unwrap();
  let mut order1 = Order::new(&wallet1, &store, 10 as f32);
  order1.save();

  let mut order2 = Order::new(&wallet1, &store, 20 as f32);
  order2.save();

  let wallet2 = Principal::from_text(WALLET2.to_string()).unwrap();
  let mut order3 = Order::new(&wallet2, &store, 30 as f32);
  order3.save();
}

#[test]
pub fn new() {
  set_up();

  assert_eq!(3, Order::len());

  let store = Principal::from_text(STORE.to_string()).unwrap();
  let wallet3 = Principal::from_text(WALLET3.to_string()).unwrap();
  let mut order4 = Order::new(&wallet3, &store, 40 as f32);

  order4.save();

  assert_eq!(4, Order::len());
}

#[test]
pub fn len() {
  set_up();
  assert_eq!(3, Order::len());
}

#[test]
pub fn by_last_update() {
  set_up();

  let now = time();

  assert_eq!(3, Order::by_last_update(now).len());
}

#[test]
pub fn by_wallet_id() {
  set_up();

  let wallet1 = Principal::from_text(WALLET1.to_string()).unwrap();

  assert_eq!(2, Order::by_wallet_id(&wallet1).len());
}

#[test]
pub fn list() {
  set_up();

  let wallet1 = Principal::from_text(WALLET1.to_string()).unwrap();

  let orders = Order::list();

  assert_eq!(3, orders.len());
  assert_eq!(wallet1, orders.get(0).unwrap().wallet_id);
}

#[test]
pub fn get() {
  set_up();

  let wallet1 = Principal::from_text(WALLET1.to_string()).unwrap();
  let order = Order::get(Memo(1));
  assert!(order.is_some());
  assert_eq!(wallet1, order.unwrap().wallet_id);

  let order = Order::get(Memo(100));
  assert!(order.is_none());
}