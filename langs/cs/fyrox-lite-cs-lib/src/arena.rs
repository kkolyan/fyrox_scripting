use std::{
    cell::RefCell, os::raw::c_void
};

// TODO replace with SendWrapper
thread_local! {
    static ARENA: RefCell<Arena> = Default::default();
}

#[derive(Default)]
pub(crate) struct Arena {
    ptrs: Vec<Ptr>,
}

#[derive()]
enum Ptr {
    Vec {
        ptr: *mut c_void,
        len: usize,
        cap: usize,
    },
}

impl Arena {
    pub(crate) fn allocate_vec<T>(mut v: Vec<T>) -> *mut T {
        ARENA.with_borrow_mut(|arena| {
            let ptr = v.as_mut_ptr();
            arena.ptrs.push(Ptr::Vec {
                ptr: ptr as *mut c_void,
                len: v.len(),
                cap: v.capacity(),
            });
            std::mem::forget(v);
            ptr
        })
    }

    pub(crate) fn free() {
        ARENA.with_borrow_mut(|arena| {
            while let Some(ptr) = arena.ptrs.pop() {
                match ptr {
                    Ptr::Vec { ptr, len, cap } => {
                        let _ = unsafe { Vec::from_raw_parts(ptr, len, cap) };
                    }
                }
            }
        })
    }
}
