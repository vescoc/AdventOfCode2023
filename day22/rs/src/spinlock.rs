use std::cell::UnsafeCell;
use std::hint::spin_loop;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>,
}

unsafe impl<T: Send> Sync for SpinLock<T> {}

pub struct SpinLockGuard<'a, T>(&'a SpinLock<T>);

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> SpinLockGuard<T> {
        while self.lock.swap(true, Ordering::Acquire) {
            spin_loop();
        }
        
        SpinLockGuard(self)
    }
}

impl<T> Deref for SpinLockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety: ok
        unsafe { &*self.0.data.get() as &T }
    }
}

impl<T> DerefMut for SpinLockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety: ok
        unsafe { &mut *self.0.data.get() as &mut T }
    }
}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.0
            .lock
            .store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_compile() {
        let a = SpinLock::new(Vec::new());

        a.lock().push("ciccio");
        a.lock().push("pippo");

        assert_eq!(a.lock().deref(), &vec!["ciccio", "pippo"]);
    }

    #[test]
    fn test_compile_static() {
        static A: SpinLock<Vec<&str>> = SpinLock::new(Vec::new());

        A.lock().push("ciccio");
        A.lock().push("pippo");

        assert_eq!(A.lock().deref(), &vec!["ciccio", "pippo"]);
    }
}
