use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use serde::Serialize;

use ego_types::app::{AppId, Category, DeployMode, FileId};
use ego_types::ego_error::EgoError;
use ego_types::version::Version;

use crate::app::*;
use crate::developer::Developer;

#[derive(CandidType, Deserialize, Serialize)]
pub enum EgoDevErr {
  AppExists,
  AppNotExists,
  VersionExists,
  VersionNotExists,
  NoFile,
  UnAuthorized,
  WasmExists,
  UserExists,
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
      EgoDevErr::NoFile => EgoError::new(1006, "ego-dev: no ego_file canister configured"),
      EgoDevErr::UnAuthorized => EgoError::new(1007, "ego-dev: unauthorized"),
      EgoDevErr::WasmExists => EgoError::new(1008, "ego-dev: wasm exists"),
      EgoDevErr::UserExists => EgoError::new(1010, "ego-dev: user exists"),
      EgoDevErr::NotADeveloper => EgoError::new(1011, "ego-dev: user is not a developer"),
      EgoDevErr::UserNotExists => EgoError::new(1012, "ego-dev: user not exists"),
      EgoDevErr::OperationNotPermitted => {
        EgoError::new(1013, "ego-dev: operation not permitted")
      }
      EgoDevErr::EgoFileAlreadyAdded => {
        EgoError::new(1014, "ego-dev: ego file canister already added")
      }
      EgoDevErr::SystemError(msg) => msg.into(),
    }
  }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainGetRequest {
  pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainGetResponse {
  pub app: EgoDevApp,
}

impl AppMainGetResponse {
  pub fn to_string(&self) -> String {
    self.app.to_string()
  }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainNewRequest {
  pub app_id: AppId,
  pub name: String,
  pub logo: String,
  pub description: String,
  pub category: Category,
  pub price: f32,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainNewResponse {
  pub app: EgoDevApp,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionNewRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionNewResponse {
  pub app_version: AppVersion,
}

#[derive(CandidType, Deserialize)]
pub struct AppVersionUploadWasmRequest {
  pub app_id: String,
  pub version: Version,
  pub data: Vec<u8>,
  pub hash: String,
}

#[derive(CandidType, Deserialize)]
pub struct AppVersionUploadWasmResponse {
  pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionSetFrontendAddressRequest {
  pub app_id: AppId,
  pub version: Version,
  pub canister_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionSetFrontendAddressResponse {
  pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionSubmitRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionSubmitResponse {
  pub app_version: AppVersion,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionRevokeRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionRevokeResponse {
  pub app_version: AppVersion,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionReleaseRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionReleaseResponse {
  pub app_version: AppVersion,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionApproveRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionApproveResponse {
  pub app_version: AppVersion,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionRejectRequest {
  pub app_id: AppId,
  pub version: Version,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionRejectResponse {
  pub app_version: AppVersion,
}

/*------------- For Development Used Only ------------------*/
#[derive(CandidType, Deserialize, Serialize)]
pub struct FileUploadSuccessRequest {
  pub app_id: AppId,
  pub version: Version,
  pub fid: FileId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct RegisterDeveloperRequest {
  pub name: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct RegisterDeveloperResponse {
  pub user: Developer,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DeveloperMainRegisterRequest {
  pub name: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DeveloperMainRegisterResponse {
  pub developer: Developer,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DeveloperAppListResponse {
  pub apps: Vec<EgoDevApp>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppVersionWaitForAuditResponse {
  pub apps: Vec<EgoDevApp>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct DeveloperMainGetResponse {
  pub developer: Developer,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserRoleSetRequest {
  pub user_id: Principal,
  pub is_app_auditor: bool,
  pub is_manager: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserRoleSetResponse {
  pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserMainListRequest {
  pub name: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct UserMainListResponse {
  pub users: Vec<Developer>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminAppCreateRequest {
  pub app_id: AppId,
  pub name: String,
  pub category: Category,
  pub logo: String,
  pub description: String,
  pub version: Version,
  pub backend_data: Vec<u8>,
  pub backend_data_hash: String,
  pub frontend: Option<Principal>,
  pub deploy_mode: DeployMode,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminAppCreateResponse {
  pub app_version: AppVersion,
}
