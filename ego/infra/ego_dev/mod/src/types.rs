use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

use ego_utils::types::{EgoError, Version};

use crate::app::*;
use crate::developer::Developer;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoDevErr {
  AppExists,
  AppNotExists,
  VersionExists,
  VersionNotExists,
  BucketExists,
  NoBucket,
  UnAuthorized,
  WasmExists,
  OrderNotExists,
  EgoWalletNotExists,
  SystemError(String),
  NotADeveloper,
  UserNotExists,
}

impl From<EgoDevErr> for EgoError {
  fn from(e: EgoDevErr) -> Self {
    match e {
      EgoDevErr::AppExists => EgoError::new(1001, "ego-dev: app exists"),
      EgoDevErr::AppNotExists => EgoError::new(1002, "ego-dev: app not exists"),
      EgoDevErr::VersionExists => EgoError::new(1003, "ego-dev: version exists"),
      EgoDevErr::VersionNotExists => EgoError::new(1004, "ego-dev: version not exists"),
      EgoDevErr::BucketExists => EgoError::new(1005, "ego-dev: bucket exists"),
      EgoDevErr::NoBucket => EgoError::new(1006, "ego-dev: bucket not exists"),
      EgoDevErr::UnAuthorized => EgoError::new(1007, "ego-dev: unauthorized"),
      EgoDevErr::WasmExists => EgoError::new(1008, "ego-dev: wasm exists"),
      EgoDevErr::OrderNotExists => EgoError::new(1009, "ego-dev: order not exists"),
      EgoDevErr::EgoWalletNotExists => EgoError::new(1010, "ego-dev: ego wallet not exists"),
      EgoDevErr::NotADeveloper => EgoError::new(1011, "ego-dev: user is not a developer"),
      EgoDevErr::UserNotExists => EgoError::new(1012, "ego-dev: user not exists"),
      EgoDevErr::SystemError(msg) => msg.into(),
    }
  }
}

impl From<std::string::String> for EgoDevErr {
  fn from(msg: String) -> Self {
    EgoDevErr::SystemError(msg)
  }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainGetRequest {
  pub app_id: AppId,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainGetResponse {
  pub app: App,
}

impl AppMainGetResponse {
  pub fn to_string(&self) -> String {
    self.app.to_string()
  }
}


#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainNewRequest {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub price: f32,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainNewResponse {
  pub app: App,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionNewRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionNewResponse {
  pub app_version: AppVersion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionSetFrontendAddressRequest {
  pub app_id: AppId,
  pub version: Version,
  pub canister_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionSetFrontendAddressResponse {
  pub ret: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionSubmitRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionSubmitResponse {
  pub app_version: AppVersion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionRevokeRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionRevokeResponse {
  pub app_version: AppVersion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionReleaseRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionReleaseResponse {
  pub app_version: AppVersion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionApproveRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionApproveResponse {
  pub app_version: AppVersion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionRejectRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionRejectResponse {
  pub app_version: AppVersion,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AddBucketRequest {
  pub bucket_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AddBucketResponse {
  pub ret: bool,
}
/*------------- For Development Used Only ------------------*/
#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct FileUploadSuccessRequest {
  pub app_id: AppId,
  pub version: Version,
  pub fid: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RegisterDeveloperRequest {
  pub name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct RegisterDeveloperResponse {
  pub user: Developer,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DeveloperMainRegisterRequest {
  pub name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DeveloperMainRegisterResponse {
  pub developer: Developer,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DeveloperAppListResponse {
  pub apps: Vec<App>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppVersionWaitForAuditResponse {
  pub apps: Vec<App>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct DeveloperMainGetResponse {
  pub developer: Developer,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct UserRoleSetRequest {
  pub user_id: Principal,
  pub is_app_developer: bool,
  pub is_app_auditer: bool,
  pub is_manager: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct UserRoleSetResponse {
  pub ret: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct UserMainListRequest {
  pub name: String,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct UserMainListResponse {
  pub users: Vec<Developer>,
}
