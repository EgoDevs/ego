use ego_macros::{inject_app_info, inject_cycle_info, inject_ego_data};
use std::cell::RefCell;
// use ic_cdk::export::Principal;

inject_app_info!();
inject_cycle_info!();
inject_ego_data!();

static WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
static EXISTS_CANISTER_ID: &str = "223xb-saaaa-aaaaf-arlqa-cai";
static EXISTS_USER_1: &str ="3dxkw-giaaa-aaaah-qbwea-cai";
static EXISTS_USER_2: &str ="ekaet-lqaaa-aaaaf-bdkta-cai";
static APP_ID: &str = "app_exists";
// static EXISTS_USER: &str ="user1";

fn on_canister_added(_name: &str, _canister_id: Principal) {}

fn set_up() {
    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
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
fn app_info_update_test() {
    set_up();

    let mut app_info = app_info_get();

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };

    assert_eq!(APP_ID, app_info.app_id);
    assert_eq!(version, app_info.current_version);

    let new_version = Version {
        major: 1,
        minor: 0,
        patch: 1,
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

    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
    };

    assert_eq!(APP_ID, app_info.app_id);
    assert_eq!(version, app_info.current_version);
}

#[test]
fn app_info_pre_upgrade_test() {
    set_up();
    let version = Version {
        major: 1,
        minor: 0,
        patch: 0,
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
fn cycle_record_test() {
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
    let cycle_record = cycle_record_add(recharge_num, ts);
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
fn cnaister_add_test() {
    // get canister list befor canister add
    let pre_canister_list = canister_list();
    assert!(pre_canister_list.is_empty());

    // canister add
    let name = "user1";
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let _canister_add = canister_add(name.to_string(), canister_id);
    assert_eq!((), _canister_add);

    // get canister list after canister add
    let after_canister_list = canister_list();
    assert_eq!(1, after_canister_list.len());
    assert!(after_canister_list.contains_key(name));
}

#[test]
fn canister_remove_test() {
    // canister add
    let name = "user1";
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let _canister_add = canister_add(name.to_string(), canister_id);
    assert_eq!((), _canister_add);

    // remove canister
    let canister_remove = canister_remove(name.to_string(), canister_id);
    assert_eq!((), canister_remove);

    // get canister list after canister remove
    let remove_canister_list = canister_list();
    assert!(remove_canister_list.contains_key(name));
}

#[test]
fn canister_remove_all_test() {
    // canister add
    let name = "user1";
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let _canister_add = canister_add(name.to_string(), canister_id);
    assert_eq!((), _canister_add);

    // remove canister
    let canister_remove_all = canister_remove_all(name.to_string());
    assert_eq!((), canister_remove_all);

    // get canister list after canister remove
    let remove_canister_list = canister_list();
    assert!(remove_canister_list.is_empty());
}

#[test]
fn canister_get_one_test() {
    // canister get one befor canister add
    let get_one = canister_get_one("user1");
    assert!(get_one.is_none());

    // canister add
    let name = "user1";
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let _canister_add = canister_add(name.to_string(), canister_id);
    assert_eq!((), _canister_add);

    // canister get one after canister added
    let get_one_1 = canister_get_one("user1");
    assert!(get_one_1.is_some());
}

#[test]
fn canister_get_all_test() {
    // canister get all before canister add
    // let get_all = canister_get_all("user1");
    // println!("get_all {:?}", get_all);

    // canister add
    let name = "user1";
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let _canister_add = canister_add(name.to_string(), canister_id);
    assert_eq!((), _canister_add);

    // canister get one after canister added
    let get_all_1 = canister_get_all("user1");
    assert!(!get_all_1.is_empty());
}

#[test]
fn registry_upgrade_test() {
    // registry pre upgrade befor canister add
    let _befor_registry_upgrade = registry_pre_upgrade();

    // get canister list befor canister add
    let pre_canister_list = canister_list();
    assert!(pre_canister_list.is_empty());

    // canister add
    let name = "user1";
    let canister_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let _canister_add = canister_add(name.to_string(), canister_id);
    assert_eq!((), _canister_add);

    // git canister list befor registry upgrade
    let after_canister_list_1 = canister_list();
    assert_eq!(1, after_canister_list_1.len());
    assert!(after_canister_list_1.contains_key(name));

    // registry pre upgrade after canister added
    let _registry_upgrade = registry_pre_upgrade();

    // registry post upgrade
    let _registry_uupgrade_post = registry_post_upgrade(_registry_upgrade);

    // git canister list after registry upgrade
    let after_canister_list_2 = canister_list();
    assert_eq!(1, after_canister_list_2.len());
    assert!(after_canister_list_2.contains_key(name));
}

#[test]
fn owner_interface_test() {
    let user_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
    let exists_user_1 = Principal::from_text(EXISTS_USER_1.to_string()).unwrap();
    let exists_user_2 = Principal::from_text(EXISTS_USER_2.to_string()).unwrap();
    // check is owner befor owner add
    let _owner = is_owner(user_id);
    assert_eq!(false, _owner);

    // get owners befor owner add
    let _owners = owners();
    let _owners = _owners.unwrap();
    assert!(_owners.is_empty());

    // set two owners
    let mut users_ = BTreeMap::new();
    users_.insert(exists_user_1, "exists_user1".to_string());
    users_.insert(exists_user_2, "exists_user2".to_string());
    let owner_set_1 = owners_set(users_);
    assert_eq!((), owner_set_1);

    // get&check owners after set two owners
    let owners_ = owners();
    let owners_ = owners_.unwrap();
    assert!(owners_.contains_key(&exists_user_1));
    assert!(owners_.contains_key(&exists_user_2));
    assert_eq!(2, owners_.len());

    // third owner add user_id
    let _third_owner_add = owner_add(user_id);

    // check third owner staus
    let owner_1 = is_owner(user_id);
    assert!(owner_1);

    // get owners after third owner add
    let after_owners_ = owners();
    let after_owners_ = after_owners_.unwrap();
    assert!(after_owners_.contains_key(&user_id));
    assert_eq!(3, after_owners_.len());

    // owner remove user_id
    let _owner_remove_ = owner_remove(user_id);
    // println!("owner_remove_ {:?}", owner_remove_);

    // get owners after user_id owner remove
    let owners_after_remove = owners();
    let owners_after_remove = owners_after_remove.unwrap();
    assert!(!owners_after_remove.contains_key(&user_id));

    // owner add with name: owner add user_id
    let _owner_add_with_name = owner_add_with_name("user_01".to_string(), user_id);
    // println!("owner_add_user_name {:?}", owner_add_with_name);

    // get&check owners after owner add with name
    let get_owners = owners();
    let get_owners = get_owners.unwrap();
    assert!(get_owners.contains_key(&user_id));

    // get users 
    let users_3 = users();
    let users_3 = users_3.unwrap();
    assert!(users_3.is_empty());

    let is_user_ = is_user(user_id);
    println!("is_user_ {:?}", is_user_);

}

#[test]
fn user_interface_test () {
  let user_id = Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap();
  let exists_user_1 = Principal::from_text(EXISTS_USER_1.to_string()).unwrap();
  let exists_user_2 = Principal::from_text(EXISTS_USER_2.to_string()).unwrap();

  // get users befor user set
  let users_1 = users();
  let users_1 = users_1.unwrap();
  assert!(users_1.is_empty());

  // check is user befor user add
  let _is_user = is_user(user_id);
  assert!(!_is_user);

  // users set 
  let mut users_ = BTreeMap::new();
  users_.insert(exists_user_1, "exists_user1".to_string());
  users_.insert(exists_user_2, "exists_user2".to_string());
  let _user_set_1 = users_set(users_);

  // get users after users set
  let users_2 = users();
  let users_2 = users_2.unwrap();
  assert!(users_2.contains_key(&exists_user_1));
  assert!(users_2.contains_key(&exists_user_2));

  // user add 
  let _user_add = user_add(user_id);

  // check is user after user add  
  let _is_user_1 = is_user(user_id);
  assert!(_is_user_1);

  // get users after users add
  let users_3 = users();
  let users_3 = users_3.unwrap();
  assert!(users_3.contains_key(&user_id));

  // remove user
  let _user_remove = user_remove(user_id);
  // println!("user remove {:?}", _user_remove);

  // user_id not in users after remove user_id
  let users_4 = users();
  let users_4 = users_4.unwrap();
  assert!(!users_4.contains_key(&user_id));

  // user add with name
  let _user_add_with_name = user_add_with_name("user_01".to_string(), user_id);

  // check user_id has been added with by name
  let users_5 = users();
  let users_5 = users_5.unwrap();
  assert!(users_5.contains_key(&user_id));

  let is_owner_ = is_owner(user_id);
  assert!(!is_owner_);
}