use std::cell::RefCell;

use ic_stable_structures::{DefaultMemoryImpl, memory_manager::{MemoryId, MemoryManager, VirtualMemory}, RestrictedMemory, StableBTreeMap, StableCell, storable::Blob};

use crate::types::app_key::AppKey;
use crate::types::app_version::AppVersion;
use crate::types::developer::Developer;
use crate::types::ego_dev_app::EgoDevApp;
use crate::types::ego_file::EgoFile;
use crate::types::stable_state::StableState;

pub const MB: u32 = 1024 * 1024;

const EGO_DEV_APP_MEM_ID: MemoryId = MemoryId::new(0);
const FILE_MEM_ID: MemoryId = MemoryId::new(1);
const DEVELOPER_MEM_ID: MemoryId = MemoryId::new(2);
const APP_VERSION_MEM_ID: MemoryId = MemoryId::new(3);

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

    pub static EGO_DEV_APPS: RefCell<StableBTreeMap<AppKey, EgoDevApp, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(EGO_DEV_APP_MEM_ID)))
    });

    pub static FILES: RefCell<StableBTreeMap<Blob<29>, EgoFile, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(FILE_MEM_ID)))
    });

    pub static DEVELOPERS: RefCell<StableBTreeMap<Blob<29>, Developer, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(DEVELOPER_MEM_ID)))
    });

    pub static APP_VERSIONS: RefCell<StableBTreeMap<u64, AppVersion, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(APP_VERSION_MEM_ID)))
    });
}
