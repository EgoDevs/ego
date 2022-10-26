use candid::candid_method;
use candid::CandidType;
use ego_assets_mod::rc_bytes::RcBytes;
use ego_assets_mod::state::STATE;
use ego_assets_mod::state_machine::{AssetDetails, EncodedAsset, StableState, State};
use ego_assets_mod::types::{
    CommitBatchArguments, CreateAssetArguments, CreateBatchResponse, CreateChunkArg,
    CreateChunkResponse, DeleteAssetArguments, GetArg, GetChunkArg, GetChunkResponse, HttpRequest,
    HttpResponse, Key, SetAssetContentArguments, StoreArg, StreamingCallbackHttpResponse,
    StreamingCallbackToken, UnsetAssetContentArguments,
};
use ic_cdk::api::{data_certificate, set_certified_data, time};
use ic_cdk::export::candid::Deserialize;
use ic_cdk::export::Principal;
use ic_cdk::{caller, trap};
use ic_cdk_macros::*;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct InitArg {
    init_caller: Option<Principal>,
}

#[init]
#[candid_method(init)]
pub fn init(arg: InitArg) {
    let caller = arg.init_caller.unwrap_or(caller());
    STATE.with(|s| {
        let mut s = s.borrow_mut();
        s.clear();
        s.authorize_unconditionally(caller);
    });
}

#[pre_upgrade]
pub fn pre_upgrade() {
    ic_cdk::println!("enter ego_assets pre_upgrade");
    let stable_state: StableState = STATE.with(|s| s.take().into());
    ic_cdk::storage::stable_save((stable_state,)).expect("failed to save stable state");
}

#[post_upgrade]
pub fn post_upgrade() {
    ic_cdk::println!("enter ego_assets post_upgrade");
    let (stable_state,): (StableState,) =
        ic_cdk::storage::stable_restore().expect("failed to restore stable state");
    STATE.with(|s| {
        *s.borrow_mut() = State::from(stable_state);
        set_certified_data(&s.borrow().root_hash());
    });
}

#[update]
#[candid_method(update)]
fn authorize(other: Principal) {
    ic_cdk::println!("enter authorize with principal {}", other);
    let caller = caller();
    STATE.with(|s| {
        if let Err(msg) = s.borrow_mut().authorize(&caller, other) {
            trap(&msg);
        }
    })
}

#[update]
#[candid_method(update)]
fn drain_authorize() {
    ic_cdk::println!("enter drain_authorize");
    let caller = caller();
    STATE.with(|s| {
        if let Err(msg) = s.borrow_mut().drain(&caller) {
            trap(&msg);
        }
    })
}

#[query(name = "list_authorize", guard = "is_authorized")]
#[candid_method(query)]
fn list_authorize() -> Result<Vec<Principal>, String> {
    let caller = caller();
    STATE.with(|s| s.borrow().list_authorized(&caller))
}

#[query]
#[candid_method(query)]
fn retrieve(key: Key) -> RcBytes {
    STATE.with(|s| match s.borrow().retrieve(&key) {
        Ok(bytes) => bytes,
        Err(msg) => trap(&msg),
    })
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn store(arg: StoreArg) {
    STATE.with(move |s| {
        if let Err(msg) = s.borrow_mut().store(arg, time()) {
            trap(&msg);
        }
        set_certified_data(&s.borrow().root_hash());
    });
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn create_batch() -> CreateBatchResponse {
    STATE.with(|s| CreateBatchResponse {
        batch_id: s.borrow_mut().create_batch(time()),
    })
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn create_chunk(arg: CreateChunkArg) -> CreateChunkResponse {
    STATE.with(|s| match s.borrow_mut().create_chunk(arg, time()) {
        Ok(chunk_id) => CreateChunkResponse { chunk_id },
        Err(msg) => trap(&msg),
    })
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn create_asset(arg: CreateAssetArguments) {
    STATE.with(|s| {
        if let Err(msg) = s.borrow_mut().create_asset(arg) {
            trap(&msg);
        }
        set_certified_data(&s.borrow().root_hash());
    })
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn set_asset_content(arg: SetAssetContentArguments) {
    STATE.with(|s| {
        if let Err(msg) = s.borrow_mut().set_asset_content(arg, time()) {
            trap(&msg);
        }
        set_certified_data(&s.borrow().root_hash());
    })
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn unset_asset_content(arg: UnsetAssetContentArguments) {
    STATE.with(|s| {
        if let Err(msg) = s.borrow_mut().unset_asset_content(arg) {
            trap(&msg);
        }
        set_certified_data(&s.borrow().root_hash());
    })
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn delete_asset(arg: DeleteAssetArguments) {
    STATE.with(|s| {
        s.borrow_mut().delete_asset(arg);
        set_certified_data(&s.borrow().root_hash());
    });
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn clear() {
    STATE.with(|s| {
        s.borrow_mut().clear();
        set_certified_data(&s.borrow().root_hash());
    });
}

#[update(guard = "is_authorized")]
#[candid_method(update)]
fn commit_batch(arg: CommitBatchArguments) {
    STATE.with(|s| {
        if let Err(msg) = s.borrow_mut().commit_batch(arg, time()) {
            trap(&msg);
        }
        set_certified_data(&s.borrow().root_hash());
    });
}

#[query]
#[candid_method(query)]
fn get(arg: GetArg) -> EncodedAsset {
    STATE.with(|s| match s.borrow().get(arg) {
        Ok(asset) => asset,
        Err(msg) => trap(&msg),
    })
}

#[query]
#[candid_method(query)]
fn get_chunk(arg: GetChunkArg) -> GetChunkResponse {
    STATE.with(|s| match s.borrow().get_chunk(arg) {
        Ok(content) => GetChunkResponse { content },
        Err(msg) => trap(&msg),
    })
}

#[query]
#[candid_method(query)]
fn list() -> Vec<AssetDetails> {
    STATE.with(|s| s.borrow().list_assets())
}

#[query]
#[candid_method(query)]
fn http_request(req: HttpRequest) -> HttpResponse {
    let certificate = data_certificate().unwrap_or_else(|| trap("no data certificate available"));

    STATE.with(|s| {
        s.borrow().http_request(
            req,
            &certificate,
            candid::Func {
                method: "http_request_streaming_callback".to_string(),
                principal: ic_cdk::id(),
            },
        )
    })
}

#[query]
#[candid_method(query)]
fn http_request_streaming_callback(token: StreamingCallbackToken) -> StreamingCallbackHttpResponse {
    STATE.with(|s| {
        s.borrow()
            .http_request_streaming_callback(token)
            .unwrap_or_else(|msg| trap(&msg))
    })
}

fn is_authorized() -> Result<(), String> {
    STATE.with(|s| {
        s.borrow()
            .is_authorized(&caller())
            .then(|| ())
            .ok_or_else(|| "Caller is not authorized".to_string())
    })
}
