// use ic_cdk::export::Principal;
// use ic_ledger_types::Memo;
//
// use ego_store_mod::order::{Order, OrderStatus};
// use ego_store_mod::service::EgoStoreService;
// use ego_store_mod::state::EGO_STORE;
// use ego_store_mod::wallet::Wallet;
//
// static STORE_ID: &str = "22cl3-kqaaa-aaaaf-add7q-cai";
// static EXISTS_WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
// static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";
// static EXISTS_TENANT_ID: &str = "22ayq-aiaaa-aaaai-qgmma-cai";
//
//
// pub fn set_up() {
//   let tenant_principal = Principal::from_text(EXISTS_TENANT_ID.to_string()).unwrap();
//   let wallet_principal = Principal::from_text(EXISTS_WALLET_ID.to_string()).unwrap();
//   let user_principal = Principal::from_text(EXISTS_USER_ID.to_string()).unwrap();
//   let store_principal = Principal::from_text(STORE_ID.to_string()).unwrap();
//
//   EGO_STORE.with(|ego_store| {
//     // add wallet
//     let mut wallet = Wallet::new(tenant_principal, wallet_principal, user_principal);
//
//     // add order
//     let order = Order::new(wallet_principal, &store_principal, 1.2f32, 10);
//     ego_store.borrow_mut().orders.insert(Memo(10), order);
//
//     wallet.orders.push(Memo(10));
//
//     ego_store.borrow_mut().wallets.insert(wallet_principal, wallet);
//   });
// }
//
// #[test]
// fn wallet_order_new() {
//   set_up();
//
//   let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
//   let store_id = Principal::from_text(STORE_ID).unwrap();
//
//   // get order list before make order
//   let result = EgoStoreService::wallet_order_list(exist_wallet_id);
//   assert!(result.is_ok());
//   assert_eq!(1, result.unwrap().len());
//
//   // create order
//   let result = EgoStoreService::wallet_order_new(exist_wallet_id, store_id, 1.2f32);
//   assert!(result.is_ok());
//   assert_eq!(1, result.unwrap().memo.0);
//
//   // get order list after make order
//   let result = EgoStoreService::wallet_order_list(exist_wallet_id);
//   assert!(result.is_ok());
//   assert_eq!(2, result.unwrap().len());
// }
//
// #[test]
// fn wallet_order_notify() {
//   set_up();
//
//   let exist_wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
//   // get order list before make order
//   let orders = EgoStoreService::wallet_order_list(exist_wallet_id).unwrap();
//   assert_eq!(1, orders.len());
//
//   EGO_STORE.with(|ego_store| {
//     assert_eq!(0, ego_store.borrow().wallets.get(&exist_wallet_id).unwrap().cycles);
//   });
//
//   let order = orders.get(0).unwrap();
//   assert_eq!(OrderStatus::NEW, order.status);
//
//   // notify order
//   let result = EgoStoreService::wallet_order_notify(Memo(10));
//   assert!(result.is_ok());
//
//   // get order list after make order
//   let orders = EgoStoreService::wallet_order_list(exist_wallet_id).unwrap();
//   assert_eq!(1, orders.len());
//
//   let order = orders.get(0).unwrap();
//   assert_eq!(OrderStatus::SUCCESS, order.status);
//
//   EGO_STORE.with(|ego_store| {
//     assert_eq!((1.2f32 * 1000000f32) as u128, ego_store.borrow().wallets.get(&exist_wallet_id).unwrap().cycles);
//   });
// }