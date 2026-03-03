use spin::{Mutex, MutexGuard};

pub fn lock_mutex<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    loop {
        if !mutex.is_locked() {
            return mutex.lock();
        }
        for _ in 0..1000 {
            core::hint::spin_loop();
        }
    }
}
