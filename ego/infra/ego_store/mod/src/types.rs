use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_ledger_types::Memo;
use ic_types::Principal;
use serde::Serialize;

use ego_utils::types::{AppId, Category, EgoError};
use crate::app::App;
use crate::order::Order;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum EgoStoreErr {
    AppExists,
    AppNotExists,
    NoTenant,
    UnAuthorized,
    OrderNotExists,
    SystemError(String),
    WalletNotExists,
    TenantExists,
    AppAlreadyInstall,
    AppNotInstall,
    WalletExists
}

impl From<EgoStoreErr> for EgoError {
    fn from(e: EgoStoreErr) -> Self {
        match e{
            EgoStoreErr::AppExists => EgoError::new(3001, "ego-store: app exists"),
            EgoStoreErr::AppNotExists=> EgoError::new(3002, "ego-store: app not exists"),
            EgoStoreErr::NoTenant => EgoError::new(3003, "ego-store: no ego tenant installed"),
            EgoStoreErr::UnAuthorized => EgoError::new(3004, "ego-store: unauthorized"),
            EgoStoreErr::OrderNotExists => EgoError::new(3005, "ego-store: order not exists"),
            EgoStoreErr::WalletNotExists => EgoError::new(3006, "ego-store: wallet not exists"),
            EgoStoreErr::WalletExists => EgoError::new(3007, "ego-store: wallet exists"),
            EgoStoreErr::TenantExists => EgoError::new(3008, "ego-store: tenant exists"),
            EgoStoreErr::AppAlreadyInstall => EgoError::new(3009, "ego-store: app already installed"),
            EgoStoreErr::AppNotInstall => EgoError::new(3010, "ego-store: app not install"),
            EgoStoreErr::SystemError(msg) => msg.into(),
        }
    }
}

impl From<std::string::String> for EgoStoreErr {
    fn from(msg: String) -> Self {
        EgoStoreErr::SystemError(msg)
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppListResponse {
    pub apps: Vec<App>
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppInstallRequest {
    pub app_id: AppId
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppInstallResponse {
    pub canister_ids: Vec<Principal>
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppUpgradeRequest {
    pub app_id: AppId
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppUpgradeResponse {
    pub canister_ids: Vec<Principal>
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppRemoveRequest {
    pub app_id: AppId
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletAppRemoveResponse {
    pub canister_ids: Vec<Principal>
}


#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub enum QueryParam {
    ByCategory { category: Category },
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainListRequest {
    pub query_param: QueryParam,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AppMainListResponse {
    pub apps: Vec<App>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct GetAppRequest {
    pub app_id: AppId,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct GetAppResponse {
    pub app: App,
}

impl GetAppResponse {
  pub fn to_string(&self) -> String {
        self.app.to_string()
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AddBucketRequest {
    pub bucket_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AddBucketResponse {
    pub ret: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletOrderNotifyRequest {
    pub memo: Memo
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletOrderNotifyResponse {
    pub ret: bool,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CreateOrderRequest {
    pub app_id: AppId
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct CreateOrderResponse {
    pub order: Order,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletOrderNewRequest {
    pub amount: f32
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletOrderNewResponse {
    pub order: Order,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletOrderListResponse {
    pub orders: Vec<Order>,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletTenantGetResponse {
    pub tenant_id: Principal
}


#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletMainNewRequest {
    pub name: String
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct WalletMainNewResponse {
    pub tenant_id: Principal,
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AdminTenantAddRequest {
    pub tenant_id: Principal
}

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct AdminTenantAddResponse {
    pub ret: bool,
}