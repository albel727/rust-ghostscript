use std::sync::{Mutex, MutexGuard};

lazy_static! {
    static ref GHOSTSCRIPT_LOCK: Mutex<()> = {
         Mutex::new(())
    };
}

pub type LockType = MutexGuard<'static, ()>;

pub fn get_lock() -> LockType {
    GHOSTSCRIPT_LOCK.lock().unwrap()
}
