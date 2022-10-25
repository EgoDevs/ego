use ic_cdk::api;
use ic_cdk::export::{Principal};

pub fn log(canister_id: Principal, message: String) {
  let _result = api::call::notify(
    canister_id,
    "canister_log_add",
    (message,),
  );
}