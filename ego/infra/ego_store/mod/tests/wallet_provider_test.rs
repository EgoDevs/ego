use ic_cdk::export::Principal;

use ego_store_mod::service::EgoStoreService;
use ego_store_mod::state::EGO_STORE;

static WALLET_PROVIDER_ID: &str = "2265i-mqaaa-aaaad-qbsga-cai";
static WALLET_APP_ID: &str = "app_exists";
static EXISTS_WALLET_ID: &str = "23vqh-waaaa-aaaai-qhcya-cai";
// static TENANT_ID: &str ="2avdy-paaaa-aaaaf-abcga-cai";

#[test]
fn admin_wallet_provider_add() {
    let wallet_provider = Principal::from_text(WALLET_PROVIDER_ID.to_string()).unwrap();

    // before add
    EGO_STORE.with(|ego_store| {
        assert_eq!(0, ego_store.borrow().wallet_providers.len());
    });

    let result =
        EgoStoreService::admin_wallet_provider_add(&wallet_provider, &WALLET_APP_ID.to_string());
    assert!(result.is_ok());

    // after add
    EGO_STORE.with(|ego_store| {
        assert_eq!(1, ego_store.borrow().wallet_providers.len());
    });
}

#[test]
fn admin_wallet_cycle_recharge_wallet_not_exists(){
    let wallet_id = Principal::from_text(EXISTS_WALLET_ID).unwrap();
    let operator = Principal::from_text(WALLET_PROVIDER_ID).unwrap();
    let result = EgoStoreService::admin_wallet_cycle_recharge(wallet_id, 128, operator, 64, "comment".to_string());
    assert!(result.is_err());
    assert_eq!(3006, result.as_ref().unwrap_err().code);
    assert_eq!("ego-store: wallet not exists", result.as_ref().unwrap_err().msg);
    println!("{:?}", result);
}