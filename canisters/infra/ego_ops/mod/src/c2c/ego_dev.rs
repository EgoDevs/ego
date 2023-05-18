use ic_cdk::api;
use ic_cdk::export::Principal;

use ego_types::app::Version;
use ego_types::app::{AppId, Category};

use crate::c2c::c2c_types::AdminAppCreateRequest;

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
    );
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
        };

        let _result = api::call::notify(canister_id, "admin_app_create", (req,));
    }
}
