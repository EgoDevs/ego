use ego_macros::inject_app_info;
use std::cell::RefCell;
use ic_cdk::export::Principal;

inject_app_info!();

static WALLET_ID: &str = "amybd-zyaaa-aaaah-qc4hq-cai";
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