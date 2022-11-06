use ic_cdk::export::candid::{CandidType, Deserialize};
use ic_cdk::export::Principal;
use ic_ledger_types::Memo;
use serde::Serialize;

use crate::app::EgoStoreApp;
use ego_types::app::{App, AppId, Category};
use ego_types::ego_error::EgoError;
use crate::cash_flow::CashFlow;

use crate::order::Order;
use crate::user_app::{AppInstalled, UserApp};

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
    WalletExists,
    WalletProviderExists,
    WalletProviderNotExists,
}

impl From<EgoStoreErr> for EgoError {
    fn from(e: EgoStoreErr) -> Self {
        match e {
            EgoStoreErr::AppExists => EgoError::new(3001, "ego-store: app exists"),
            EgoStoreErr::AppNotExists => EgoError::new(3002, "ego-store: app not exists"),
            EgoStoreErr::NoTenant => EgoError::new(3003, "ego-store: no ego tenant installed"),
            EgoStoreErr::UnAuthorized => EgoError::new(3004, "ego-store: unauthorized"),
            EgoStoreErr::OrderNotExists => EgoError::new(3005, "ego-store: order not exists"),
            EgoStoreErr::WalletNotExists => EgoError::new(3006, "ego-store: wallet not exists"),
            EgoStoreErr::WalletExists => EgoError::new(3007, "ego-store: wallet exists"),
            EgoStoreErr::TenantExists => EgoError::new(3008, "ego-store: tenant exists"),
            EgoStoreErr::AppAlreadyInstall => {
                EgoError::new(3009, "ego-store: app already installed")
            }
            EgoStoreErr::AppNotInstall => EgoError::new(3010, "ego-store: app not install"),
            EgoStoreErr::WalletProviderExists => {
                EgoError::new(3011, "ego-store: wallet provider exists")
            }
            EgoStoreErr::WalletProviderNotExists => {
                EgoError::new(3012, "ego-store: wallet provider not exists")
            }
            EgoStoreErr::SystemError(msg) => msg.into(),
        }
    }
}

impl From<std::string::String> for EgoStoreErr {
    fn from(msg: String) -> Self {
        EgoStoreErr::SystemError(msg)
    }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppListResponse {
    pub apps: Vec<AppInstalled>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppInstallRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppInstallResponse {
    pub user_app: AppInstalled,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppUpgradeRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppUpgradeResponse {
    pub user_app: AppInstalled,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppRemoveRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletAppRemoveResponse {}

#[derive(CandidType, Deserialize, Serialize)]
pub enum QueryParam {
    ByCategory { category: Category },
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainListRequest {
    pub query_param: QueryParam,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainListResponse {
    pub apps: Vec<App>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainGetRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainGetResponse {
    pub app: App,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderNotifyRequest {
    pub memo: Memo,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderNotifyResponse {
    pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterTrackRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCanisterUnTrackRequest {
    pub app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderNewRequest {
    pub amount: f32,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderNewResponse {
    pub memo: Memo,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCycleListResponse {
    pub cash_flows: Vec<CashFlow>,
}


#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletOrderListResponse {
    pub orders: Vec<Order>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCycleChargeRequest {
    pub wallet_id: Principal,
    pub cycle: u128,
    pub comment: String,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletCycleChargeResponse {
    pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletTenantGetResponse {
    pub tenant_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletMainRegisterRequest {
    pub user_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletMainRegisterResponse {
    pub tenant_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletMainNewRequest {
    pub user_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct WalletMainNewResponse {
    pub user_app: UserApp,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminEgoTenantAddRequest {
    pub tenant_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminEgoTenantAddResponse {
    pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainReleaseRequest {
    pub app: EgoStoreApp,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AppMainReleaseResponse {
    pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletProviderAddRequest {
    pub wallet_provider: Principal,
    pub wallet_app_id: AppId,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletProviderAddResponse {
    pub ret: bool,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct AdminWalletCycleRechargeRequest {
    pub wallet_id: Principal,
    pub cycle: u128,
    pub comment: String,
}
