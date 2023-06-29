use core::cell::UnsafeCell;
use core::sync::atomic::AtomicBool;
use core::sync::atomic::Ordering;

// A mutual exclusion primitive
pub struct Mutex<T> {
    // Space for the data to be protected behind the Mutex
    data: UnsafeCell<T>,
    // Tells if the Mutex has been locked by any code
    locked: AtomicBool
}

unsafe impl<T: Sync> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for Mutex<T> {}

impl<T> Mutex<T> {
    // Creates a new Mutex
    pub const fn new(val: T) -> Self {
        Self {
            data: UnsafeCell::new(val),
            locked: AtomicBool::new(false)
        }
    }

    // Obtains mutually exclusive access to the data in the Mutex
    pub fn lock(&self) -> MutexGuard<T> {
        while self.locked.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Signal the processor to go into an efficient loop
            core::hint::spin_loop();
        }
        MutexGuard {
            data: unsafe { &mut *self.data.get() },
            locked: &self.locked
        }
    }
}

// A structure to give temporary access to data in a Mutex
pub struct MutexGuard<'a, T> {
    data: &'a mut T,
    locked: &'a AtomicBool
}

impl<'a, T> Drop for MutexGuard<'a, T> {
    // Releases the lock
    fn drop(&mut self) {
        self.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> core::ops::Deref for MutexGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.data
    }
}

impl<'a, T> core::ops::DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.data
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mutex() {
        static NO: Mutex<i32> = Mutex::new(0);
        let mut handles = vec![];
        for _ in 0..10 {
            let handle = std::thread::spawn(|| {
                *NO.lock() += 1;
            });
            handles.push(handle);
        }
        handles.into_iter().for_each(|handle| { handle.join(); });
        assert_eq!(*NO.lock(), 10);
    }
}