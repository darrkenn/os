use spin::{Mutex, MutexGuard};

pub fn lock_mutex<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    loop {
        if let Some(guard) = mutex.try_lock() {
            return guard;
        };
        for _ in 0..1000 {
            core::hint::spin_loop();
        }
    }
}
