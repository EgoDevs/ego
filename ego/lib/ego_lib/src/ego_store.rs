use async_trait::async_trait;
use ic_cdk::api;
use ic_cdk::api::call::RejectionCode;
use ic_cdk::export::Principal;
use tracing::error;

use ego_types::app::{App, AppId, EgoError, UserApp};

#[async_trait]
pub trait TEgoStore {
  async fn wallet_main_new(&self, user_id: Principal) -> Result<UserApp, EgoError>;

  async fn app_main_list(&self) -> Result<Vec<App>, EgoError>;
  async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError>;

  async fn wallet_app_install(&self, app_id: AppId) -> Result<UserApp, EgoError>;
  fn wallet_app_upgrade(&self, wallet_id: Principal);
  fn wallet_app_remove(&self, wallet_id: Principal);
  async fn wallet_app_list(&self) -> Result<Vec<UserApp>, EgoError>;
}

pub struct EgoStore {
  pub canister_id: Principal,
}

impl EgoStore {
  pub fn new(canister_id: Principal) -> Self {
    EgoStore {
      canister_id
    }
  }
}

#[async_trait]
impl TEgoStore for EgoStore {
  async fn wallet_main_new(&self, user_id: Principal) -> Result<UserApp, EgoError> {
    let call_result = api::call::call(self.canister_id, "wallet_main_new", (user_id, )).await as Result<(Result<UserApp, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(wallet_app) => Ok(wallet_app),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling wallet_main_new"
        );
        Err(EgoError { code, msg })
      }
    }
  }

  async fn app_main_list(&self) -> Result<Vec<App>, EgoError> {
    let call_result = api::call::call(self.canister_id, "app_main_list", ()).await as Result<(Result<Vec<App>, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(apps) => Ok(apps),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling app_main_list"
        );
        Err(EgoError { code, msg })
      }
    }
  }

  async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError> {
    let call_result = api::call::call(self.canister_id, "app_main_get", (app_id, )).await as Result<(Result<App, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(app) => Ok(app),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling app_main_get"
        );
        Err(EgoError { code, msg })
      }
    }
  }

  async fn wallet_app_list(&self) -> Result<Vec<UserApp>, EgoError> {
    let call_result = api::call::call(self.canister_id, "wallet_app_list", ()).await as Result<(Result<Vec<UserApp>, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(user_apps) => Ok(user_apps),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling wallet_app_list"
        );
        Err(EgoError { code, msg })
      }
    }
  }

  async fn wallet_app_install(&self, app_id: AppId) -> Result<UserApp, EgoError> {
    let call_result = api::call::call(self.canister_id, "wallet_app_install", (app_id, )).await as Result<(Result<UserApp, EgoError>, ), (RejectionCode, String)>;

    match call_result {
      Ok(resp) => match resp.0 {
        Ok(user_app) => Ok(user_app),
        Err(e) => Err(e),
      },
      Err((code, msg)) => {
        let code = code as u16;
        error!(
          error_code = code,
          error_message = msg.as_str(),
          "Error calling wallet_app_install"
        );
        Err(EgoError { code, msg })
      }
    }
  }

  fn wallet_app_upgrade(&self, wallet_id: Principal) {
    let _result = api::call::notify(self.canister_id, "wallet_app_upgrade", (wallet_id, ));

    // let call_result = api::call::call(self.canister_id, "wallet_app_upgrade", (wallet_id, )).await as Result<(Result<(), EgoError>, ), (RejectionCode, String)>;
    //
    // match call_result {
    //   Ok(resp) => match resp.0 {
    //     Ok(user_app) => Ok(user_app),
    //     Err(e) => Err(e),
    //   },
    //   Err((code, msg)) => {
    //     let code = code as u16;
    //     error!(
    //       error_code = code,
    //       error_message = msg.as_str(),
    //       "Error calling wallet_app_upgrade"
    //     );
    //     Err(EgoError { code, msg })
    //   }
    // }
  }

  fn wallet_app_remove(&self, wallet_id: Principal) {
    let _result = api::call::notify(self.canister_id, "wallet_app_remove", (wallet_id, ));

    // let call_result = api::call::call(self.canister_id, "wallet_app_remove", (wallet_id, )).await as Result<(Result<(), EgoError>, ), (RejectionCode, String)>;
    //
    // match call_result {
    //   Ok(resp) => match resp.0 {
    //     Ok(_) => Ok(()),
    //     Err(e) => Err(e),
    //   },
    //   Err((code, msg)) => {
    //     let code = code as u16;
    //     error!(
    //       error_code = code,
    //       error_message = msg.as_str(),
    //       "Error calling wallet_app_remove"
    //     );
    //     Err(EgoError { code, msg })
    //   }
    // }
  }
}