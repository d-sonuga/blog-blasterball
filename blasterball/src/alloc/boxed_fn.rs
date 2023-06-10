use crate::event_hook::EventInfo;
use crate::alloc::boxed::Box;
use crate::allocator::Allocator;

// The polymorphic representation of a base function which can stand in
// for any event handler function
#[repr(C)]
struct BaseFn {
    call_boxed: fn(*const BoxedFn, EventInfo) -> (),
    drop: fn(*const BoxedFn) -> (),
    clone: fn(*const BoxedFn) -> BoxedFn
}

// A stand in for `Box<dyn FnMut>`
pub struct BoxedFn<'a>(*mut BaseFn, &'a dyn Allocator);


#[repr(C)]
struct Repr<F: FnMut(EventInfo)> {
    base: BaseFn,
    func: F
}

// Calls the concrete function wrapped by the BoxedFn
fn call_boxed<F>(boxed_fn_ptr: *const BoxedFn, event: EventInfo) where F: FnMut(EventInfo) {
    unsafe {
        let concrete_repr_ptr = (*boxed_fn_ptr).0 as *mut Repr<F>;
        ((*concrete_repr_ptr).func)(event) 
    }
}

// Drops the boxed function
fn drop<F>(boxed_fn_ptr: *const BoxedFn) where F: FnMut(EventInfo) {
    unsafe {
        let base_fn_ptr = (*boxed_fn_ptr).0;
        let concrete_ptr: *mut Repr<F> = base_fn_ptr as *mut Repr<F>;
        let allocator = (*boxed_fn_ptr).1;
        Box::<Repr<F>>::from_raw(concrete_ptr, allocator);
        // Box is dropped at the end of the scope
    }
}

// Clones the boxed function
//
// # Safety
//
// Cloning a BoxedFn is highly unsafe. The function may contains mutable
// references to the outer scope. Cloning the BoxedFn will result in cloning
// mutable references, defeating Rust's safety guarantees.
fn clone<F>(boxed_fn_ptr: *const BoxedFn) -> BoxedFn where F: FnMut(EventInfo) {
    unsafe {
        let base_fn_ptr = (*boxed_fn_ptr).0;
        let concrete_ptr: *mut Repr<F> = base_fn_ptr.cast::<Repr<F>>();
        let allocator = (*boxed_fn_ptr).1;
        let func_ptr = &(*concrete_ptr).func as *const F;
        BoxedFn::new(func_ptr.read(), allocator)
    }
}

impl<'a> BoxedFn<'a> {
    // Creates a new BoxedFn from the given function-thing
    pub fn new<F>(func: F, allocator: &'a dyn Allocator) -> Self where F: FnMut(EventInfo) {
        let base_fn = BaseFn { call_boxed: call_boxed::<F>, drop: drop::<F>, clone: clone::<F> };
        let concrete_repr = Repr {
            base: base_fn,
            func
        };
        let concrete_repr_ptr: *mut Repr<F> = Box::<Repr<F>>::into_raw(Box::new(concrete_repr, allocator));
        let polymorphic_ptr: *mut BaseFn = concrete_repr_ptr as *mut BaseFn;
        BoxedFn(polymorphic_ptr, allocator)
    }
}

impl<'a> Drop for BoxedFn<'a> {
    fn drop(&mut self) {
        let base_fn_ptr = self.0;
        unsafe { ((*base_fn_ptr).drop)(self as *const BoxedFn) };
    }
}

impl<'a> Clone for BoxedFn<'a> {
    fn clone(&self) -> Self {
        let base_fn_ptr = self.0;
        unsafe { ((*base_fn_ptr).clone)(self as *const BoxedFn) }
    }
}

impl<'a> Fn<(EventInfo,)> for BoxedFn<'a> {
    extern "rust-call" fn call(&self, args: (EventInfo,)) -> Self::Output {
        let base_fn_ptr = self.0;
        unsafe { ((*base_fn_ptr).call_boxed)(self as *const BoxedFn, args.0) }
    }
}

impl<'a> FnMut<(EventInfo,)> for BoxedFn<'a> {
    extern "rust-call" fn call_mut(&mut self, args: (EventInfo,)) -> Self::Output {
        self.call(args)
    }
}

impl<'a> FnOnce<(EventInfo,)> for BoxedFn<'a> {
    type Output = ();
    extern "rust-call" fn call_once(self, args: (EventInfo,)) -> Self::Output {
        self.call(args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_hook::EventInfo;
    use crate::alloc::vec;
    use crate::alloc::test_utils::*;

    #[test]
    fn test_fn_call() {
        let mut was_called = false;
        let f: _ = BoxedFn::new(|_| {
            was_called = true;
        }, successful_allocator());
        assert!(!was_called);
        f(EventInfo::Timer);
        assert!(was_called);
    }

    #[test]
    fn test_vec_of_boxed_fn() {
        let mut no_of_fns_called = 0;
        let allocator = successful_allocator();
        let mut v: vec::Vec<BoxedFn> = vec::Vec::with_capacity(3, allocator);
        v.push(BoxedFn::new(|_| no_of_fns_called += 1, allocator));
        v.push(BoxedFn::new(|_| no_of_fns_called += 1, allocator));
        v.push(BoxedFn::new(|_| no_of_fns_called += 1, allocator));
        v.iter().for_each(|f| f(EventInfo::Timer));
        assert_eq!(no_of_fns_called, 3);
    }

    #[test]
    fn test_clone() {
        let allocator = successful_allocator();
        let mut x = 0;
        let f = BoxedFn::new(|_| x += 1, allocator);
        let g = f.clone();
        core::mem::drop(f);
        assert_eq!(x, 0);
        g(EventInfo::Timer);
        assert_eq!(x, 1);
    }
}