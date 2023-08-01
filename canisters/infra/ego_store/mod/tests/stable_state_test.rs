use candid::Principal;

use ego_store_mod::state::{canister_add, owner_add};
use ego_store_mod::types::stable_state::StableState;

static EXISTS_CANISTER_ID: &str = "22fyd-yaaaa-aaaaf-aml4q-cai";
static EXISTS_USER_ID: &str = "225da-yaaaa-aaaah-qahrq-cai";

#[test]
pub fn load() {
  let state = StableState::load();

  assert!(state.users.is_some());
  assert!(state.registry.is_some());

  assert_eq!(0, state.users.unwrap().owners().unwrap().len());
  assert_eq!(0, state.registry.unwrap().canister_list_all().len());

  owner_add(Principal::from_text(EXISTS_USER_ID.to_string()).unwrap());
  canister_add("test".to_string(), Principal::from_text(EXISTS_CANISTER_ID.to_string()).unwrap());

  let state = StableState::load();

  assert!(state.users.is_some());
  assert!(state.registry.is_some());

  assert_eq!(1, state.users.unwrap().owners().unwrap().len());
  assert_eq!(1, state.registry.unwrap().canister_list_all().len());
}