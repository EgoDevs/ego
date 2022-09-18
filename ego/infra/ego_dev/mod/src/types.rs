use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_types::Principal;
use serde::Serialize;

use ego_utils::types::{AppId, Category, EgoError, Version, WasmId};

use crate::app::*;
use crate::developer::Developer;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoDevErr {
  AppExists,
  AppNotExists,
  VersionExists,
  VersionNotExists,
  BucketExists,
  NoFile,
  UnAuthorized,
  WasmExists,
  OrderNotExists,
  EgoWalletNotExists,
  NotADeveloper,
  UserNotExists,
  OperationNotPermitted,
  EgoFileAlreadyAdded,
  SystemError(String),
}

impl From<EgoDevErr> for EgoError {
  fn from(e: EgoDevErr) -> Self {
    match e {
      EgoDevErr::AppExists => EgoError::new(1001, "ego-dev: app exists"),
      EgoDevErr::AppNotExists => EgoError::new(1002, "ego-dev: app not exists"),
      EgoDevErr::VersionExists => EgoError::new(1003, "ego-dev: version exists"),
      EgoDevErr::VersionNotExists => EgoError::new(1004, "ego-dev: version not exists"),
      EgoDevErr::BucketExists => EgoError::new(1005, "ego-dev: bucket exists"),
      EgoDevErr::NoFile => EgoError::new(1006, "ego-dev: no ego_file canister configured"),
      EgoDevErr::UnAuthorized => EgoError::new(1007, "ego-dev: unauthorized"),
      EgoDevErr::WasmExists => EgoError::new(1008, "ego-dev: wasm exists"),
      EgoDevErr::OrderNotExists => EgoError::new(1009, "ego-dev: order not exists"),
      EgoDevErr::EgoWalletNotExists => EgoError::new(1010, "ego-dev: ego wallet not exists"),
      EgoDevErr::NotADeveloper => EgoError::new(1011, "ego-dev: user is not a developer"),
      EgoDevErr::UserNotExists => EgoError::new(1012, "ego-dev: user not exists"),
      EgoDevErr::OperationNotPermitted => EgoError::new(1013, "ego-dev: operation not permitted"),
      EgoDevErr::EgoFileAlreadyAdded => EgoError::new(1014, "ego-dev: ego file canister already added"),
      EgoDevErr::SystemError(msg) => msg.into(),
    }
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

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AppVersionUploadWasmRequest {
  pub app_id: String,
  pub version: Version,
  pub data: Vec<u8>,
  pub hash: String,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AppVersionUploadWasmResponse {
  pub ret: bool
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
  pub fid: WasmId,
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
  pub is_app_auditor: bool,
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


#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AdminFileAddRequest {
  pub file_id: Principal
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AdminFileAddResponse {
  pub ret: bool,
}