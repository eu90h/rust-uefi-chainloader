#![allow(dead_code)]

use crate::efi_alloc::alloc_pool;
use core::convert::TryInto;

#[derive(Debug)]
pub struct Buffer<T> {
    pub data: *mut T,
    /// Holds the number of elements in the buffer.
    pub count: usize,
    capacity: usize,
    iter_state_elt_num: usize,
}

impl<T> Buffer<T> {
    pub fn with_capacity(capacity: usize) -> Buffer<T> {
        let data = alloc_pool::<T>(capacity);
        let count = 0;
        let iter_state_elt_num = 0;
        Buffer { data, count, iter_state_elt_num, capacity }
    }
    pub const fn from_raw_parts(data: *mut T, count: usize) -> Buffer<T> {
        let iter_state_elt_num = 0;
        let capacity = count;
        Buffer { data, count, iter_state_elt_num, capacity }
    }
    pub const fn as_mut_ptr(&self) -> *mut T {
        self.data
    }
    pub const fn as_ptr(&self) -> *const T {
        self.data
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.data, self.count) }
    }
    pub fn as_mut_slice(&self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.data, self.count) }
    }
    /// Returns a mutable reference to the number of elements in the buffer.
    pub fn count_mut_ref(&mut self) -> &mut usize {
        &mut self.count 
    }
    /// Returns the size of the buffer in bytes.
    pub const fn size(&self) -> usize {
        self.count * core::mem::size_of::<T>()
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub unsafe fn set_count(&mut self, count: usize) {
        self.count = count;
    }
}

impl<T> Iterator for Buffer<T> {
    type Item = *mut T;

    fn next(&mut self) -> Option<Self::Item> {
        assert!(self.iter_state_elt_num <= self.count);
        if self.iter_state_elt_num == self.count {
            self.iter_state_elt_num = 0;
            None
        } else {
            let t = unsafe { self.data.offset((self.iter_state_elt_num).try_into().unwrap()) };
            self.iter_state_elt_num += 1;
            Some(t)
        }
    }
}