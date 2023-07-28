use std::cell::RefCell;

use ic_stable_structures::{DefaultMemoryImpl, memory_manager::{MemoryId, MemoryManager, VirtualMemory}, RestrictedMemory, StableBTreeMap, StableCell, storable::Blob};

use crate::types::stable_state::StableState;
use crate::types::task::Task;

const TASK_MEM_ID: MemoryId = MemoryId::new(0);
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

    pub static TASKS: RefCell<StableBTreeMap<Blob<29>, Task, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(TASK_MEM_ID)))
    });
}
