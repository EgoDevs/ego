use ego_macros::{inject_app_info, inject_cycle_info, inject_ego_data};
use std::cell::RefCell;
// use ic_cdk::export::Principal;

inject_app_info!();
inject_cycle_info!();
inject_ego_data!();

static WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static APP_ID: &str = "app_exists";

fn set_up(){
  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };

  let wallet_id = Principal::from_text(WALLET_ID.to_string()).unwrap();

  APP_INFO.with(|info| {
    info.borrow_mut().wallet_id = Some(wallet_id);
    info.borrow_mut().app_id = APP_ID.to_string();
    info.borrow_mut().current_version = version;
    info.borrow_mut().latest_version = version;
  });
}

#[test]
fn app_info_update_test(){
  set_up();

  let mut app_info = app_info_get();

  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };

  assert_eq!(APP_ID, app_info.app_id);
  assert_eq!(version, app_info.current_version);

  let new_version = Version{
    major: 1,
    minor: 0,
    patch: 1
  };

  app_info_update(None, APP_ID.to_string(), new_version);

  app_info = app_info_get();
  assert_eq!(APP_ID, app_info.app_id);
  assert_eq!(new_version, app_info.current_version);
}

#[test]
fn app_info_get_test() {
  set_up();

  let app_info = app_info_get();

  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };

  assert_eq!(APP_ID, app_info.app_id);
  assert_eq!(version, app_info.current_version);
}

#[test]
fn app_info_pre_upgrade_test() {
  set_up();
  let version = Version{
    major: 1,
    minor: 0,
    patch: 0
  };

  let app_info = app_info_pre_upgrade();
  assert_eq!(version, app_info.current_version);
  assert_eq!(APP_ID, app_info.app_id);
}

#[test]
fn app_info_post_upgrade_test() {
  set_up();
  let app1 = app_info_get();

  let app_info = app_info_post_upgrade(app1);
  assert_eq!((), app_info);
}

#[test]
fn cycle_recore_test()
{
  let recharge_num = 864000;
  let ts = 64;

  // before cycle add
  let pre_cycle_info = cycle_info_get();
  assert_eq!(0, pre_cycle_info.estimate_remaining);
  // println!("pre_cycle_info {:?}", pre_cycle_info);
  
  // before cycle add check cycle record list
  let pre_cycle_record_list = cycle_record_list();
  assert_eq!(0, pre_cycle_record_list.len());

  // cycle recharge
  let cycle_record= cycle_record_add(recharge_num, ts);
  assert_eq!((), cycle_record);

  // check cycle info after cycle recharge
  let cycle_info_1 = cycle_info_get();
  assert_eq!(recharge_num, cycle_info_1.records[0].balance);
  assert_eq!(ts, cycle_info_1.records[0].ts);
  assert_eq!(0, cycle_info_1.estimate_remaining);
  // println!("cycle_info {:?}", cycle_info_1);

  let cycle_record_list = cycle_record_list();
  // assert_eq!(cycle_record_list);
  assert_eq!(1, cycle_record_list.len());

  // estimate remaining set
  let es_timate = 128;
  let estimate = estimate_remaining_set(es_timate);
  assert_eq!((), estimate);

  // check cycle info pre upgrade
  let cycle_pre_upgrade = cycle_info_pre_upgrade();
  assert_eq!(recharge_num, cycle_pre_upgrade.records[0].balance);
  assert_eq!(ts, cycle_pre_upgrade.records[0].ts);
  assert_eq!(es_timate, cycle_pre_upgrade.estimate_remaining);
  // println!("cycle_pre_upgrade {:?}", cycle_pre_upgrade);

  let cycle_post_upgrade = cycle_info_post_upgrade(cycle_pre_upgrade);
  assert_eq!((), cycle_post_upgrade);
  // println!("cycle_post_upgrade {:?}", cycle_post_upgrade);

  // check cycle info after upgrade
  let cycle_info_2 = cycle_info_get();
  assert_eq!(recharge_num, cycle_info_2.records[0].balance);
  assert_eq!(ts, cycle_info_2.records[0].ts);
  assert_eq!(es_timate, cycle_info_2.estimate_remaining);
}

#[test]
fn log_list_test(){
  // set_up();
  let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
  let canister_add = canister_add("user1".to_string(), canister_id);
  println!("canister_add {:?}", canister_add);
}
