#![allow(dead_code)]
use core::ffi::c_void;
use core::convert::TryInto;
use crate::utils::{enable_write_protection, disable_write_protection};

pub const X64_HOOK_TEMPLATE: [u8; 12] = [0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0xc3];
pub static mut BACKUP_BYTES_BUFFER: [u8; 12] = [0; 12];
pub static mut BACKUP_BYTES_PTR: *mut [u8;12] = unsafe { &mut BACKUP_BYTES_BUFFER };

pub struct X64Hook {
    pub address: u64,
    pub backup: [u8;12],
    our_fn: *const c_void,
}

impl X64Hook {
    pub const fn empty() -> X64Hook {
        X64Hook { address: 0, backup: [0,0,0,0,0,0,0,0,0,0,0,0], our_fn: core::ptr::null_mut() }
    }
    pub fn new(target_address: u64, our_fn: *const c_void) -> X64Hook {
        let mut h = X64Hook::empty();
        h.address = target_address;
        h.our_fn = our_fn;
        h.set_backup(target_address as *mut u8);
        h.set_hook();
        h
    }
    pub fn set_backup(&mut self, src: *const u8) {
        let backup_ptr: *mut [u8;X64_HOOK_TEMPLATE.len()] = &mut self.backup;
        let backup_ptr: *mut u8 = backup_ptr as *mut u8;

        disable_write_protection();

        for i in 0..self.backup.len() {
            unsafe { *backup_ptr.offset(i.try_into().unwrap()) = *src.offset(i as isize) };
        }

        enable_write_protection();

        crate::info!("finished setting backup for {:02X}", self.address);
    }
    pub fn revert(&self) {
        disable_write_protection();

        let dst: *mut u8 = unsafe { core::mem::transmute(self.address) };
        for i in 0..self.backup.len()  {
            unsafe {
                *dst.offset(i as isize) = self.backup[i];
            }
        }

        enable_write_protection();
    }
    pub fn set_hook(&self) {
        let call_target: u64 = unsafe { core::mem::transmute(self.our_fn as *const c_void) };
        let dst: *mut u8 = self.address as *mut u8;
        let src: *const u8 = &X64_HOOK_TEMPLATE as *const u8;

        unsafe {
            for i in 0..X64_HOOK_TEMPLATE.len() {
                *dst.offset(i as isize) = *src.offset(i as isize);
            }
        }

        let address_location: *mut u64 = unsafe { dst.offset(2).cast::<u64>() };
        unsafe { *address_location = call_target }
        crate::info!("set hook for {:02X}", self.address);
    }
}