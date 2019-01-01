use lazy_static::lazy_static;

use std::sync::Mutex;

lazy_static! {
    static ref SHARED_TEST_MUTEX: Mutex<()> = Mutex::new(());
}

#[cfg(test)]
pub fn no_parallel() {
    let _ = SHARED_TEST_MUTEX.lock().unwrap();
}
