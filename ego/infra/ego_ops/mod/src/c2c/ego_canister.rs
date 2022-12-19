use ic_cdk::api;
use ic_cdk::export::Principal;

pub trait TEgoCanister {
  fn ego_canister_add(&self, canister_id: &Principal, name: String, principal: &Principal);
}

pub struct EgoCanister {}

impl EgoCanister {
  pub fn new() -> Self {
    EgoCanister {}
  }
}

impl TEgoCanister for EgoCanister {
  fn ego_canister_add(&self, canister_id: &Principal, name: String, principal: &Principal) {
    ic_cdk::println!("ego_ops add canister {}", name);
    let _result = api::call::notify(canister_id.clone(), "ego_canister_add", (name, principal.clone()));
  }
}
