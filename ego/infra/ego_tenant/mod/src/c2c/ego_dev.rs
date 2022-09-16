use ic_cdk::api;
use ic_types::Principal;
use ego_dev_mod::app::{App};
use ego_dev_mod::types::{AppId, AppMainGetRequest, AppMainGetResponse};
use ego_utils::types::EgoError;
use ego_file_mod::types::{FileMainReadRequest, FileMainReadResponse};
use async_trait::async_trait;

#[async_trait]
pub trait TEgoDev {
  async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError>;
  async fn file_main_read(&self, app_id: AppId, fid: String) -> Result<Vec<u8>, EgoError>;
}

pub struct EgoDev {
  pub canister_id: Principal
}

impl EgoDev{
  pub fn new(canister_id: Principal) -> Self {
    EgoDev{canister_id}
  }
}

#[async_trait]
impl TEgoDev for EgoDev {
  async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError>{
    let req = AppMainGetRequest {
      app_id,
    };

    let call_result = api::call::call(
      self.canister_id,
      "app_main_get",
      (req,),
    )
      .await as Result<(Result<AppMainGetResponse, EgoError>,), _>;

    match call_result.unwrap().0 {
      Ok(resp) => {
        Ok(resp.app)
      },
      Err(e) => {
        Err(e)
      }
    }
  }

  async fn file_main_read(&self, app_id: AppId, fid: String) -> Result<Vec<u8>, EgoError>{
    let req = FileMainReadRequest {
      fid
    };

    let call_result = api::call::call(
      self.canister_id,
      "file_main_read",
      (req,),
    )
      .await as Result<(Result<FileMainReadResponse, EgoError>,), _>;

    match call_result.unwrap().0 {
      Ok(resp) => {
        Ok(resp.data)
      },
      Err(e) => {
        Err(e)
      }
    }
  }
}