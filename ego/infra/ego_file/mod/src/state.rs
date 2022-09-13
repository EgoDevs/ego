use std::cell::RefCell;

use crate::storage::Storage;

thread_local! {
   pub static STORAGE: RefCell<Storage> = RefCell::new(Storage::new());
}
