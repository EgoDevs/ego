use ic_stable_structures::{memory_manager::{MemoryId, MemoryManager, VirtualMemory}, DefaultMemoryImpl, StableBTreeMap, RestrictedMemory, StableCell, storable::Blob};
use std::cell::{RefCell};
use crate::types::*;

const TASK_MEM_ID: MemoryId = MemoryId::new(0);
const METADATA_PAGES: u64 = 64; // 4M

type RM = RestrictedMemory<DefaultMemoryImpl>;
type VM = VirtualMemory<RM>;


thread_local! {
    pub static CONFIG: RefCell<StableCell<StableState, RM>> = RefCell::new(StableCell::init(RM::new(DefaultMemoryImpl::default(), 0..METADATA_PAGES), StableState::default()).expect("failed to initialize the config cell"));

    static MEMORY_MANAGER: RefCell<MemoryManager<RM>> = RefCell::new(
        MemoryManager::init(RM::new(DefaultMemoryImpl::default(), METADATA_PAGES..1024))
    );

    pub static TASKS: RefCell<StableBTreeMap<Blob<29>, Task, VM>> = MEMORY_MANAGER.with(|mm| {
        RefCell::new(StableBTreeMap::init(mm.borrow().get(TASK_MEM_ID)))
    });
}
