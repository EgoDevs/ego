use ic_cdk::api;
use ic_cdk::export::Principal;
use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use crate::app::App;
use ego_types::app::App as EgoStoreApp;
use crate::c2c::c2c_types::{AppMainReleaseRequest};

#[async_trait]
pub trait TEgoStore {
  async fn app_main_release(&self, canister_id: Principal, app: App) -> Result<bool, EgoError>;
}

pub struct EgoStore {

}

impl EgoStore{
  pub fn new() -> Self {
    EgoStore{}
  }
}

#[async_trait]
impl TEgoStore for EgoStore {
  async fn app_main_release(&self, canister_id: Principal, app: App) -> Result<bool, EgoError> {
    let released_version = app.released_version()?;

    let req = AppMainReleaseRequest{app: EgoStoreApp::new(app.app_id, app.name, app.category, app.logo, app.description, app.release_version.unwrap(), released_version.frontend, released_version.backend, app.price)};

    let notify_result = api::call::notify(
      canister_id,
      "app_main_release",
      (req,),
    );

    match notify_result {
      Err(code) => {
        let code = code as u16;
        Err(EgoError { code, msg: "app_main_release notify failed".to_string() })
      },
      _ => Ok(true)
    }
  }
}