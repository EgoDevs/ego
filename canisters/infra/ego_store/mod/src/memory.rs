use std::cell::RefCell;

use ic_stable_structures::{DefaultMemoryImpl, memory_manager::{MemoryId, MemoryManager, VirtualMemory}, RestrictedMemory, StableBTreeMap, StableCell, storable::Blob};

use crate::types::app_key::AppKey;
use crate::types::cash_flow::CashFlow;
use crate::types::ego_store_app::EgoStoreApp;
use crate::types::order::Order;
use crate::types::stable_state::StableState;
use crate::types::tenant::Tenant;
use crate::types::user_app::UserApp;
use crate::types::wallet::Wallet;
use crate::types::wallet_provider::WalletProvider;

pub const MB: u32 = 1024 * 1024;

const EGO_STORE_APP_MEM_ID: MemoryId = MemoryId::new(0);
const TENANT_MEM_ID: MemoryId = MemoryId::new(1);
const WALLET_PROVIDER_MEM_ID: MemoryId = MemoryId::new(2);
const WALLET_MEM_ID: MemoryId = MemoryId::new(3);
const USER_APP_MEM_ID: MemoryId = MemoryId::new(4);
const ORDER_MEM_ID: MemoryId = MemoryId::new(5);
const CASH_FLOW_MEM_ID: MemoryId = MemoryId::new(6);

const METADATA_PAGES: u64 = 64;
// 4M
const WASM_PAGE_SIZE: u64 = 65536;

/// The maximum number of stable memory pages a canister can address.
pub const MAX_PAGES: u64 = u64::MAX / WASM_PAGE_SIZE;


type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;

thread_local! {
    pub static CONFIG: RefCell<StableCell<StableState, RM>> = RefCell::new(StableCell::init(RM::new(DefaultMemoryImpl::default(), 0..METADATA_PAGES), StableState::default()).expect("failed to initialize the config cell"));

    static MEMORY_MANAGER: RefCell<MemoryManager<RM>> = RefCell::new(
        MemoryManager::init(RM::new(DefaultMemoryImpl::default(), METADATA_PAGES..MAX_PAGES))
    );

    pub static EGO_STORE_APPS: RefCell<StableBTreeMap<AppKey, EgoStoreApp, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(EGO_STORE_APP_MEM_ID)))
    });

    pub static TENANTS: RefCell<StableBTreeMap<Blob<29>, Tenant, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(TENANT_MEM_ID)))
    });

    pub static WALLET_PROVIDERS: RefCell<StableBTreeMap<Blob<29>, WalletProvider, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(WALLET_PROVIDER_MEM_ID)))
    });

    pub static WALLETS: RefCell<StableBTreeMap<Blob<29>, Wallet, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(WALLET_MEM_ID)))
    });

    pub static USER_APPS: RefCell<StableBTreeMap<Blob<29>, UserApp, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(USER_APP_MEM_ID)))
    });

    pub static ORDERS: RefCell<StableBTreeMap<u64, Order, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(ORDER_MEM_ID)))
    });

    pub static CASH_FLOWS: RefCell<StableBTreeMap<u64, CashFlow, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(CASH_FLOW_MEM_ID)))
    });
}
