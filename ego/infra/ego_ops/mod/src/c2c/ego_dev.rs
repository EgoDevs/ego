use ic_cdk::api;
use ic_cdk::export::Principal;

use ego_dev_mod::types::{AdminAppCreateRequest};
use ego_types::app::{AppId, Category, DeployMode};
use ego_types::version::Version;

pub trait TEgoDev {
    fn admin_app_create(
        &self,
        canister_id: Principal,
        app_id: AppId,
        name: String,
        version: Version,
        category: Category,
        logo: String,
        description: String,
        backend_data: Vec<u8>,
        backend_data_hash: String,
        frontend: Option<Principal>,
        deploy_mode: DeployMode,
    ) ;
}

pub struct EgoDev {}

impl EgoDev {
    pub fn new() -> Self {
        EgoDev {}
    }
}

impl TEgoDev for EgoDev {
    fn admin_app_create(
        &self,
        canister_id: Principal,
        app_id: AppId,
        name: String,
        version: Version,
        category: Category,
        logo: String,
        description: String,
        backend_data: Vec<u8>,
        backend_data_hash: String,
        frontend: Option<Principal>,
        deploy_mode: DeployMode,
    ) {
        let req = AdminAppCreateRequest {
            app_id,
            name,
            version,
            category,
            logo,
            description,
            backend_data,
            backend_data_hash,
            frontend,
            deploy_mode,
        };

        let _result = api::call::notify(canister_id, "admin_app_create", (req,));
    }
}
