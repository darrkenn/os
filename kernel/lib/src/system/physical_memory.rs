use core::cell::OnceCell;

use spin::Mutex;

use crate::utils::deadlock::lock_mutex;

pub static PHYSICAL_MEMORY_OFFSET: spin::Mutex<OnceCell<u64>> = Mutex::new(OnceCell::new());

pub fn convert_physical_to_virtual_addr(value: u64) -> u64 {
    // This is unsafe, but it should never get to this point and have the OnceCell unset.
    value + lock_mutex(&PHYSICAL_MEMORY_OFFSET).get().unwrap()
}
