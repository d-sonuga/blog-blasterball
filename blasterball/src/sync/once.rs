use core::cell::UnsafeCell;
use core::mem::MaybeUninit;
use core::sync::atomic::AtomicU8;
use core::sync::atomic::Ordering;

// A structure that allows for a one-time initialization of
// values at runtime
pub struct Once<T> {
    // The data that will be initialized at runtime
    data: UnsafeCell<MaybeUninit<T>>,
    // Provides atomic access to the status of the data
    status: AtomicStatus
}

unsafe impl<T: Sync> Sync for Once<T> {}

// Tells the state of a Once
#[derive(Clone, Copy, Debug, PartialEq)]
enum OnceStatus {
    // The data in the Once us uninitialized
    Uninit = 0,
    // The data in the Once has been initialized
    Init = 1,
    // The initializer function is running
    Running = 2,
    // The initializer panicked
    Panicked = 3
}

#[repr(transparent)]
struct AtomicStatus(AtomicU8);

impl OnceStatus {
    // Interpret a `u8` as a `OnceStatus`
    fn new(status: u8) -> Self {
        unsafe { core::mem::transmute(status) }
    }
}

impl AtomicStatus {
    // Creates a new AtomicStatus from a OnceStatus
    const fn new(status: OnceStatus) -> Self {
        Self(AtomicU8::new(status as u8))
    }

    // Atomically read the status
    fn load(&self, ordering: Ordering) -> OnceStatus {
        OnceStatus::new(self.0.load(ordering))
    }

    // Atomically writes the status
    fn store(&self, val: OnceStatus, ordering: Ordering) {
        self.0.store(val as u8, ordering);
    }

    fn compare_exchange(
        &self,
        old: OnceStatus,
        new: OnceStatus,
        success: Ordering,
        failure: Ordering
    ) -> Result<OnceStatus, OnceStatus> {
        match self.0.compare_exchange(old as u8, new as u8, success, failure) {
            Ok(status) => Ok(OnceStatus::new(status)),
            Err(status) => Err(OnceStatus::new(status))
        }
    }
}

impl<T> Once<T> {
    // Creates a new Once
    pub const fn new() -> Self {
        Self {
            status: AtomicStatus::new(OnceStatus::Uninit),
            data: UnsafeCell::new(MaybeUninit::uninit())
        }
    }

    // Retrieves the data if it has been initialized
    fn get(&self) -> Option<&T> {
        match self.status.load(Ordering::Acquire) {
            OnceStatus::Init => {
                unsafe {
                    let data_ptr = self.data.get();
                    let maybeinit_ptr = (*data_ptr).as_ptr();
                    Some(&*maybeinit_ptr)
                }
            }
            _ => None
        }
    }

    // Performs an initialization routine once and only once
    pub fn call_once<F: FnOnce() -> T>(&self, f: F) {
        let status = self.status.load(Ordering::Acquire);
        if status == OnceStatus::Uninit {
            match self.status.compare_exchange(
                OnceStatus::Uninit,
                OnceStatus::Running,
                Ordering::Acquire,
                Ordering::Acquire
            ) {
                Ok(_) => {
                    // Creating a Finish value with a reference to the Once's 
                    // status
                    let finish = Finish { status: &self.status };
                    let val = f();
                    unsafe {
                        let data_ptr = self.data.get();
                        let mut_data_ptr = (*data_ptr).as_mut_ptr();
                        mut_data_ptr.write(val);
                    }
                    // If a panic occurs before this `mem::forget` executes, that means
                    // the initializer function panicked.
                    // finish will get dropped and the Once status will be set to OnceStatus::Panicked
                    core::mem::forget(finish);
                    self.status.store(OnceStatus::Init, Ordering::Release);
                }
                Err(_) => () // Don't do anything if the Once is in any other state
            }
        }
    }
}

// A structure that changes the Once status to OnceStatus::Panicked
// on drop
struct Finish<'a> {
    // A reference to the status of the Once
    status: &'a AtomicStatus
}

impl<'a> Drop for Finish<'a> {
    fn drop(&mut self) {
        self.status.store(OnceStatus::Panicked, Ordering::SeqCst);
    }
}

impl<T> core::ops::Deref for Once<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        loop {
            match self.status.load(Ordering::Acquire) {
                OnceStatus::Init => return self.get().unwrap(),
                OnceStatus::Running => core::hint::spin_loop(),
                OnceStatus::Uninit => panic!("Once has not been initialized"),
                OnceStatus::Panicked => panic!("Once initializer panicked")
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_once() {
        static VAL: Once<i32> = Once::new();
        VAL.call_once(|| 5);
        assert_eq!(*VAL, 5);
        VAL.call_once(|| 10000000);
        assert_eq!(*VAL, 5);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        static VAL: Once<i32> = Once::new();
        assert_eq!(*VAL, 5);
    }
}