use ic_cdk::api;
use ic_cdk::export::Principal;
use async_trait::async_trait;
use ego_types::ego_error::EgoError;
use crate::app::EgoDevApp;
use crate::c2c::c2c_types::{AppMainReleaseRequest, EgoStoreApp};

#[async_trait]
pub trait TEgoStore {
  async fn app_main_release(&self, canister_id: Principal, app: EgoDevApp) -> Result<bool, EgoError>;
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
  async fn app_main_release(&self, canister_id: Principal, app: EgoDevApp) -> Result<bool, EgoError> {
    let released_version = app.released_version()?;

    let req = AppMainReleaseRequest{
      app: EgoStoreApp{
        app_id: app.app_id,
        name: app.name,
        category: app.category,
        logo: app.logo,
        description: app.description,
        current_version: app.release_version.unwrap(),
        frontend: released_version.frontend,
        backend: released_version.backend,
        price: app.price,
        deploy_mode: app.deploy_mode
      }
    };

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