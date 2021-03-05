#![no_std]
#![no_main]

#![feature(core_intrinsics)]
#![feature(panic_info_message)]
#![feature(lang_items)]
#![feature(default_alloc_error_handler)]
#![feature(asm)]
#![feature(stmt_expr_attributes)]

#![allow(unreachable_code)]

extern crate alloc;

use core::ptr::null_mut;
use boot_services::{init_exit_boot_services_event_notification, install_boot_services_hooks};
use loader::{get_loaded_image_protocol, load_image_from_path, start_image};
use r_efi::base::*;
use r_efi::system::*;
use utils::{is_at_runtime, wait_for_key};

pub mod protocols;
#[macro_use]
mod macros;
#[macro_use]
mod logger;
mod utils;
mod eh;
mod efi_alloc;
mod buffer;
mod hooks;
mod loader;
mod boot_services;

#[global_allocator]
static GLOBAL_ALLOC: efi_alloc::Allocator = efi_alloc::Allocator {};

static mut ST: *mut SystemTable = null_mut();
static mut BS: *mut BootServices = null_mut();
static mut RS: *mut RuntimeServices = null_mut();
static mut THIS_IMG: Handle = null_mut();
static mut AT_RUNTIME: bool = false;
static mut BOOTLOADER_IMAGE_BASE: Handle = null_mut();
static mut BOOTLOADER_IMAGE_SIZE: usize = 0;

const PROGRAM_TO_RUN_PATH: &'static str = "bzImage.efi\0";

#[no_mangle]
fn efi_main(this_img: Handle, system_table: *mut SystemTable) -> Status {
    if this_img.is_null() || system_table.is_null() || unsafe { (*system_table).boot_services.is_null() } {
        return Status::INVALID_PARAMETER;
    }

    unsafe {
        ST = system_table;
        BS = (*system_table).boot_services;
        RS = (*system_table).runtime_services;
        THIS_IMG = this_img;
    }

    init_exit_boot_services_event_notification();
    unsafe { 
        install_boot_services_hooks()
    }
    
    let mut path_buf = alloc::string::String::from(PROGRAM_TO_RUN_PATH).encode_utf16().collect::<alloc::vec::Vec<u16>>();
    let path_size = path_buf.len() * 2;
    let path = path_buf.as_mut_ptr();
    
  
    let new_image: Handle = load_image_from_path(path, path_size);
    let loaded_new_image_proto: *mut r_efi::protocols::loaded_image::Protocol = get_loaded_image_protocol(new_image).as_ptr();
    let image_base = unsafe { (*loaded_new_image_proto).image_base };
    let _image_size = unsafe { (*loaded_new_image_proto).image_size } as u32;

    unsafe {
        BOOTLOADER_IMAGE_BASE = image_base;
        BOOTLOADER_IMAGE_SIZE = (*loaded_new_image_proto).image_size as usize;
    }

    println!("Press any key to continue.");
    wait_for_key();
    
    start_image(new_image);

    Status::SUCCESS
}