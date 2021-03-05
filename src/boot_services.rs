use core::{ffi::c_void, ptr::null_mut};

use r_efi::{efi::{Event, Handle, Status}, system::{EVT_SIGNAL_EXIT_BOOT_SERVICES, TPL_NOTIFY}};

use crate::{AT_RUNTIME, BS, efi_alloc::alloc_pool, utils::is_at_runtime};
pub type FnExitBootServices = extern "win64" fn(*mut c_void, usize) -> r_efi::base::Status;
pub static mut HOOKS_INSTALLED: bool = false;
static mut REAL_EXIT_BOOT_SERVICES: FnExitBootServices = exit_boot_services;
static mut EXIT_BOOT_SERVICES_EVENT: *mut Event = null_mut();

extern "win64"  fn notify_exit_boot_services(_event: Event, _context: *mut c_void) {
    unsafe {
        AT_RUNTIME = true;
        BS = null_mut();
    }
}

extern "win64"  fn exit_boot_services(img_handle: Handle, size: usize) -> r_efi::base::Status {
    if is_at_runtime() {
        panic!("my_exit_boot_services called at runtime.")
    }
    unsafe {
        //Uninstall our hooks now in order to prevent this function from getting called twice.
        uninstall_boot_services_hooks();

        REAL_EXIT_BOOT_SERVICES(img_handle, size)
    }
}

pub fn init_exit_boot_services_event_notification() {
    unsafe { EXIT_BOOT_SERVICES_EVENT = alloc_pool::<Event>(1) };
    let status = efi_bs_call!(create_event, EVT_SIGNAL_EXIT_BOOT_SERVICES, TPL_NOTIFY, notify_exit_boot_services, null_mut(), EXIT_BOOT_SERVICES_EVENT);
    match status {
        Status::SUCCESS => {}
        Status::INVALID_PARAMETER => {
            panic!("Invalid parameter for create_event.")
        }
        Status::OUT_OF_RESOURCES => {
            panic!("Out of resources.")
        }
        _ => {
            panic!("Unhandled status code returned from create_event.")
        }
    }
}

pub unsafe fn install_boot_services_hooks() {
    if is_at_runtime() {
        panic!("install_hooks called at runtime.")
    }
    
    if HOOKS_INSTALLED { return; }

    REAL_EXIT_BOOT_SERVICES = (*BS).exit_boot_services;
    (*BS).exit_boot_services = exit_boot_services;

    HOOKS_INSTALLED = true;
}

pub unsafe fn uninstall_boot_services_hooks() {
    if is_at_runtime() {
        panic!("uninstall_hooks called at runtime.")
    }

    if HOOKS_INSTALLED { return; }

    (*BS).exit_boot_services = REAL_EXIT_BOOT_SERVICES;
    HOOKS_INSTALLED = false;
}