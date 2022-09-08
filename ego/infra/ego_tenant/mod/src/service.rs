use ic_types::Principal;
use ego_utils::consts::CREATE_CANISTER_CYCLES_FEE;
use ego_utils::types::Management;
use ego_utils::types::EgoError;

pub struct TenantService {

}

impl TenantService {
    pub async fn wallet_app_install<T: Management>(service: T, _app_id: &str) -> Result<Principal, EgoError> {
        service.canister_main_create(CREATE_CANISTER_CYCLES_FEE).await
    }
}
