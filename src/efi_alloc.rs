use core::alloc::{GlobalAlloc, Layout};
use r_efi::base::*;
use r_efi::system::*;
use core::ffi::c_void;
use crate::utils::is_at_runtime;

pub struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        alloc_pool::<u8>(layout.size()) as *mut u8
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free_pool(ptr as *mut c_void)
    }
}

pub fn alloc_pool<T>(num_objs: usize) -> *mut T {
    let mut ptr: *mut c_void = core::ptr::null_mut();

    if is_at_runtime() {
        panic!("Can't currently allocate memory at runtime.")
    } else {
        let sz = num_objs * core::mem::size_of::<T>();
        if efi_bs_call!(allocate_pool, MemoryType::LoaderData, sz , &mut ptr) == Status::SUCCESS {
            ptr as *mut T
        } else {
            panic!("allocate_pool failed")
        }
    }
}

pub fn free_pool<T>(ptr: *mut T) {
    if is_at_runtime() {
        panic!("Can't currently free memory at runtime.")
    } else {
        if efi_bs_call!(free_pool, ptr as *mut c_void) != Status::SUCCESS {
            panic!("free_pool failed")
        }
    }
}
