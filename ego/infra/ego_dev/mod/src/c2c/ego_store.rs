use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::export::Principal;

use crate::app::{AppVersion, EgoDevApp};
use crate::c2c::c2c_types::EgoStoreApp;

#[async_trait]
pub trait TEgoStore {
  fn app_main_release(
    &self,
    app: EgoDevApp,
    app_version: AppVersion
  );
}

pub struct EgoStore {
  pub canister_id: Principal,
}

impl EgoStore {
  pub fn new(canister_id: Principal) -> Self {
    EgoStore { canister_id }
  }
}

#[async_trait]
impl TEgoStore for EgoStore {
  fn app_main_release(
    &self,
    ego_dev_app: EgoDevApp,
    released_version: AppVersion,
  )  {
    let ego_store_app = EgoStoreApp {
      app: ego_dev_app.app.clone(),
      wasm: released_version.wasm.unwrap(),
    };

    let _result = api::call::notify(self.canister_id, "app_main_release", (ego_store_app, ));

    // let call_result = api::call::call(self.canister_id, "app_main_release", (ego_store_app, )).await as Result<(Result<bool, EgoError>, ), (RejectionCode, String)>;
    //
    // match call_result {
    //   Ok(resp) => match resp.0 {
    //     Ok(ret) => Ok(ret),
    //     Err(e) => Err(e),
    //   },
    //   Err((code, msg)) => {
    //     let code = code as u16;
    //     error!(
    //       error_code = code,
    //       error_message = msg.as_str(),
    //       "Error calling wallet_main_new"
    //     );
    //     Err(EgoError { code, msg })
    //   }
    // }
  }
}
