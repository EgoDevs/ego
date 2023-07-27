use async_trait::async_trait;
use ic_cdk::api::call::RejectionCode;
use candid::{Principal};
use ic_cdk::{api, trap};

use ego_types::app::{App, AppId, CashFlow, EgoError, UserApp};
use ego_types::types::{AppInstallRequest, AppReInstallRequest, AppUpgradeRequest, WalletUpgradeAppRequest};

#[async_trait]
pub trait TEgoStore {
    async fn wallet_main_register(&self, user_id: Principal) -> Result<Principal, EgoError>;

    async fn wallet_main_new(&self, user_id: Principal) -> Result<UserApp, EgoError>;

    async fn app_main_list(&self) -> Result<Vec<App>, EgoError>;
    async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError>;

    async fn wallet_app_install(&self, app_id: AppId) -> Result<UserApp, EgoError>;
    async fn wallet_app_upgrade(&self, wallet_id: Principal);
    async fn wallet_app_upgrade_by_wallet(&self, canister_id: Principal);

    fn wallet_app_remove(&self, wallet_id: Principal);
    async fn wallet_app_list(&self) -> Result<Vec<UserApp>, EgoError>;

    async fn wallet_cycle_balance(&self) -> Result<u128, EgoError>;
    async fn wallet_cycle_list(&self) -> Result<Vec<CashFlow>, EgoError>;

    // canister track
    fn wallet_canister_track(&self, canister_id: Principal);
    fn wallet_canister_track_self(&self, wallet_id: Principal);
    fn wallet_canister_untrack(&self, canister_id: Principal);
    fn wallet_canister_untrack_self(&self, wallet_id: Principal);

    // v2 interface
    async fn wallet_app_install_v2(&self, req: AppInstallRequest) -> Result<UserApp, EgoError>;
    async fn wallet_app_upgrade_v2(&self, req: AppUpgradeRequest);
    async fn wallet_app_reinstall_by_wallet_v2(&self, req: AppReInstallRequest);
    async fn wallet_app_upgrade_by_wallet_v2(&self, req: WalletUpgradeAppRequest);
}

#[derive(Copy, Clone)]
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
    async fn wallet_main_register(&self, user_id: Principal) -> Result<Principal, EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_main_register", (user_id,))
            .await
            as Result<(Result<Principal, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(tenant_id) => Ok(tenant_id),
                Err(e) => trap(
                    format!(
                        "Error calling wallet_main_register code: {}, msg: {}",
                        e.code, e.msg
                    )
                    .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "Error calling wallet_main_register code: {}, msg: {}",
                        code, msg
                    )
                    .as_str(),
                )
            }
        }
    }

    async fn wallet_main_new(&self, user_id: Principal) -> Result<UserApp, EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_main_new", (user_id,)).await
            as Result<(Result<UserApp, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(wallet_app) => Ok(wallet_app),
                Err(e) => trap(
                    format!(
                        "Error calling wallet_main_new code: {}, msg: {}",
                        e.code, e.msg
                    )
                    .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(format!("Error calling wallet_main_new code: {}, msg: {}", code, msg).as_str())
            }
        }
    }

    async fn app_main_list(&self) -> Result<Vec<App>, EgoError> {
        let call_result = api::call::call(self.canister_id, "app_main_list", ()).await
            as Result<(Result<Vec<App>, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(apps) => Ok(apps),
                Err(e) => trap(
                    format!(
                        "Error calling app_main_list code: {}, msg: {}",
                        e.code, e.msg
                    )
                    .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(format!("Error calling app_main_list code: {}, msg: {}", code, msg).as_str())
            }
        }
    }

    async fn app_main_get(&self, app_id: AppId) -> Result<App, EgoError> {
        let call_result = api::call::call(self.canister_id, "app_main_get", (app_id,)).await
            as Result<(Result<App, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(app) => Ok(app),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn wallet_app_list(&self) -> Result<Vec<UserApp>, EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_app_list", ()).await
            as Result<(Result<Vec<UserApp>, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(user_apps) => Ok(user_apps),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn wallet_app_install(&self, app_id: AppId) -> Result<UserApp, EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_app_install", (app_id,)).await
            as Result<(Result<UserApp, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(user_app) => Ok(user_app),
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_install code:{}, msg:{}",
                        e.code, e.msg
                    )
                    .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_install code:{}, msg:{}",
                        code as u16, msg
                    )
                    .as_str(),
                );
            }
        }
    }

    async fn wallet_app_upgrade(&self, wallet_id: Principal) {
        let call_result = api::call::call(self.canister_id, "wallet_app_upgrade", (wallet_id,))
            .await
            as Result<(Result<(), EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(_) => {
                    ic_cdk::println!("wallet_app_upgrade success");
                }
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_upgrade code:{}, msg:{}",
                        e.code, e.msg
                    )
                    .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_upgrade code:{}, msg:{}",
                        code as u16, msg
                    )
                    .as_str(),
                );
            }
        }
    }

    async fn wallet_app_upgrade_by_wallet(&self, canister_id: Principal) {
        let call_result = api::call::call(
            self.canister_id,
            "wallet_app_upgrade_by_wallet",
            (canister_id,),
        )
        .await
            as Result<(Result<(), EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(_) => {
                    ic_cdk::println!("wallet_app_upgrade_by_wallet success");
                }
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_upgrade_by_wallet code:{}, msg:{}",
                        e.code, e.msg
                    )
                    .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_upgrade_by_wallet code:{}, msg:{}",
                        code as u16, msg
                    )
                    .as_str(),
                );
            }
        }
    }

    fn wallet_app_remove(&self, wallet_id: Principal) {
        let _result = api::call::notify(self.canister_id, "wallet_app_remove", (wallet_id,));
    }

    async fn wallet_cycle_balance(&self) -> Result<u128, EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_cycle_balance", ()).await
            as Result<(Result<u128, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(balance) => Ok(balance),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    async fn wallet_cycle_list(&self) -> Result<Vec<CashFlow>, EgoError> {
        let call_result = api::call::call(self.canister_id, "wallet_cycle_list", ()).await
            as Result<(Result<Vec<CashFlow>, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(cash_flows) => Ok(cash_flows),
                Err(e) => Err(e),
            },
            Err((code, msg)) => {
                let code = code as u16;
                Err(EgoError { code, msg })
            }
        }
    }

    fn wallet_canister_track(&self, canister_id: Principal) {
        let _result = api::call::notify(self.canister_id, "wallet_canister_track", (canister_id,));
    }

    fn wallet_canister_track_self(&self, wallet_id: Principal) {
        let _result = api::call::notify(self.canister_id, "wallet_canister_track_self", (wallet_id,));
    }

    fn wallet_canister_untrack(&self, canister_id: Principal) {
        let _result =
            api::call::notify(self.canister_id, "wallet_canister_untrack", (canister_id,));
    }

    fn wallet_canister_untrack_self(&self, wallet_id: Principal) {
        let _result =
          api::call::notify(self.canister_id, "wallet_canister_untrack_self", (wallet_id,));
    }

    // v2 interface
    async fn wallet_app_install_v2(&self, req: AppInstallRequest) -> Result<UserApp, EgoError>{
        let call_result = api::call::call(self.canister_id, "wallet_app_install_v2", (req,)).await
          as Result<(Result<UserApp, EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(user_app) => Ok(user_app),
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_install_v2 code:{}, msg:{}",
                        e.code, e.msg
                    )
                      .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_install_v2 code:{}, msg:{}",
                        code as u16, msg
                    )
                      .as_str(),
                );
            }
        }
    }

    async fn wallet_app_reinstall_by_wallet_v2(&self, req: AppReInstallRequest) {
        let call_result = api::call::call(self.canister_id, "wallet_app_reinstall_by_wallet_v2", (req,)).await
          as Result<(Result<(), EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(_) => {
                    ic_cdk::println!("wallet_app_reinstall_by_wallet_v2 success");
                }
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_reinstall_by_wallet_v2 code:{}, msg:{}",
                        e.code, e.msg
                    )
                      .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_reinstall_by_wallet_v2 code:{}, msg:{}",
                        code as u16, msg
                    )
                      .as_str(),
                );
            }
        }
    }

    async fn wallet_app_upgrade_v2(&self, req: AppUpgradeRequest){
        let call_result = api::call::call(self.canister_id, "wallet_app_upgrade_v2", (req,))
          .await
          as Result<(Result<(), EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(_) => {
                    ic_cdk::println!("wallet_app_upgrade_v2 success");
                }
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_upgrade_v2 code:{}, msg:{}",
                        e.code, e.msg
                    )
                      .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_upgrade_v2 code:{}, msg:{}",
                        code as u16, msg
                    )
                      .as_str(),
                );
            }
        }
    }

    async fn wallet_app_upgrade_by_wallet_v2(&self, req: WalletUpgradeAppRequest){
        let call_result = api::call::call(
            self.canister_id,
            "wallet_app_upgrade_by_wallet_v2",
            (req,),
        )
          .await
          as Result<(Result<(), EgoError>,), (RejectionCode, String)>;

        match call_result {
            Ok(resp) => match resp.0 {
                Ok(_) => {
                    ic_cdk::println!("wallet_app_upgrade_by_wallet_v2 success");
                }
                Err(e) => trap(
                    format!(
                        "error calling wallet_app_upgrade_by_wallet_v2 code:{}, msg:{}",
                        e.code, e.msg
                    )
                      .as_str(),
                ),
            },
            Err((code, msg)) => {
                let code = code as u16;
                trap(
                    format!(
                        "error calling wallet_app_upgrade_by_wallet_v2 code:{}, msg:{}",
                        code as u16, msg
                    )
                      .as_str(),
                );
            }
        }
    }
}
