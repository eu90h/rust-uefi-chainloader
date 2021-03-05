#![allow(dead_code)]
use core::{ffi::c_void, ptr::NonNull, ptr::null_mut, intrinsics::size_of};
use alloc::string::String;
use alloc::vec::Vec;
use r_efi::{system::InterfaceType, efi::{Boolean, Guid, Handle, Status}, protocols::{simple_text_input::InputKey}};
use crate::{BS, ST, efi_alloc::alloc_pool, hooks::X64_HOOK_TEMPLATE};
use crate::buffer::Buffer;
use crate::boot_services::uninstall_boot_services_hooks;
pub fn exit(handle: Handle, status: Status) -> ! {
    if is_at_runtime() {
        panic!("exit called at runtime.")
    }
    
    unsafe {
        if crate::boot_services::HOOKS_INSTALLED {
            uninstall_boot_services_hooks()
        }
    }
    
    efi_bs_call!(exit, handle, status, 0, null_mut());
    halt()
}

#[inline]
pub fn str_to_u16(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[inline]
pub fn string_to_u16(s: &String) -> Vec<u16> {
    s.encode_utf16().collect()
}

#[inline]
pub fn halt() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}

#[inline]
pub fn booted_by_uefi_firmware() -> bool {
   unsafe { (*crate::ST).hdr.signature == crate::SYSTEM_TABLE_SIGNATURE }
}

#[inline]
pub fn is_at_runtime() -> bool {
    unsafe { crate::AT_RUNTIME }
}

#[inline]
pub fn this_img() -> r_efi::efi::Handle {
    unsafe { crate::THIS_IMG }
}

pub fn get_handles_supporting(protocol_guid: &Guid) -> Option<Buffer<Handle>> {
    let mut handle_count = 0;
    let mut handles_raw: *mut Handle = null_mut();
    let mut protocol_guid = *protocol_guid;
    let status = efi_bs_call!(locate_handle_buffer, r_efi::system::LocateSearchType::ByProtocol, &mut protocol_guid, null_mut(), &mut handle_count, &mut handles_raw);
    if status != Status::SUCCESS || handle_count == 0 {
        return None;
    } else if handle_count == 0 {
        None
    } else {
        assert!(!handles_raw.is_null());
        assert!(!unsafe{*handles_raw}.is_null());
        let handles:Buffer<Handle> = Buffer::from_raw_parts(handles_raw, handle_count);
        Some(handles)
    }
}

pub fn get_protocol_instance<Protocol>(handle: Handle, protocol_guid: &Guid) -> NonNull<Protocol> {
    let mut proto: *mut Protocol = alloc_pool::<Protocol>(1);
    let proto_ptr: *mut *mut Protocol = &mut proto;
    let mut protocol_guid = *protocol_guid;
    let status = efi_bs_call!(open_protocol, handle, &mut protocol_guid, proto_ptr as *mut *mut core::ffi::c_void, this_img(), null_mut(), r_efi::system::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
    match status {
        Status::SUCCESS => {
            assert_eq!(proto_ptr.is_null(), false);
            assert_eq!(proto.is_null(), false);
            match NonNull::new(proto as *mut Protocol) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("get_protocol_instance failed to make non-null pointer")
                }
            }
        }
        Status::INVALID_PARAMETER => {
            panic!("get_protocol_instance failed: EFI_INVALID_PARAMETER")
        }
        Status::UNSUPPORTED => {
            panic!("get_protocol_instance failed: EFI_UNSUPPORTED")
        }
        Status::ACCESS_DENIED => {
            panic!("get_protocol_instance failed: EFI_ACCESS_DENIED")
        }
        Status::ALREADY_STARTED => {
            panic!("get_protocol_instance failed: EFI_ALREADY_STARTED")
        }
        _ => {
            panic!("get_protocol_instance failed: returned unexpected status code")
        }
    }
}
pub unsafe fn copy(dst: *mut u8, src: *mut u8, bytes: usize) {
    for i in 0..bytes{
        *dst.offset(i as isize) = *src.offset(i as isize);
    }
}

pub fn find_pattern(pattern: &[u8], wildcard: u8, base: *const c_void, size: u32) -> Option<*const u8> {
    if base.is_null() {
        return None;
    }
    let base_addr: u64 = base as u64;
    let buf_sz: u64 = base_addr + (size as u64) - (pattern.len() as u64);
    let mut p = base_addr as *mut u8;
    let  pat_len = pattern.len() as isize ;
    for _ in 0..buf_sz {
        let mut i = 0;
        let mut t = p;
        for byte in pattern {
            if *byte != wildcard && unsafe { *t } != *byte {
                break
            } else {
                i += 1;
                unsafe { t = t.offset(1) };
            }
        }
        if i == pat_len {
            return Some(p);
        }
        p = unsafe { p.offset(1) };
    }
    None
}


#[allow(unused_unsafe)]
#[allow(const_item_mutation)]
pub const fn calc_relative_call_offset(call_address: u64, target_address: u64) -> u32 {
    (target_address - (call_address + 1 + unsafe { size_of::<u32>() as u64 })) as u32
}

pub fn call_address(call_address: *mut c_void) -> *mut c_void {
    let p_u8 = call_address as *mut u8;
    let p2 = unsafe { p_u8.offset(1)  };
    let relative_call_offset = unsafe { *(p2 as *mut u32) };
    unsafe { p2.offset(relative_call_offset as isize + 1 + size_of::<u32>() as isize) as *mut c_void}
}

pub fn write_call(at: *mut c_void, call_target: u64) {
    let tpl = efi_bs_call!(raise_tpl, 31);

    let dst: *mut u8 = at as *mut u8;
    #[allow(const_item_mutation)]
    let src: *const [u8; 12] = &mut X64_HOOK_TEMPLATE;
    let size = X64_HOOK_TEMPLATE.len();
    copy_mem(dst as *mut c_void, src as *mut c_void, size);

    let address_location: *mut u64 = unsafe { dst.offset(2).cast::<u64>() };
    unsafe { *address_location = call_target }

    efi_bs_call!(restore_tpl, tpl);
}
pub fn get_bytes(address: *mut u8, n: usize) -> Vec<u8> {
    let buf = alloc_pool::<u8>(n);
    copy_mem(buf as *mut c_void, address as *mut c_void, n);
    unsafe { Vec::from_raw_parts(buf, n, n) }
}

#[inline]
pub fn copy_mem(dst: *mut c_void, src: *mut c_void, size: usize) {
    efi_bs_call!(copy_mem, dst as *mut c_void, src as *mut c_void, size);
}

#[inline]
pub fn set_mem(buffer: *mut c_void, size: usize, value: u8) {
    efi_bs_call!(set_mem, buffer, size, value)
}

pub fn wait_for_key() {
    let mut index: usize = 0;
    let mut key: InputKey = unsafe { core::mem::zeroed::<InputKey>() };
    let mut wait_for_key = unsafe { (*(*ST).con_in).wait_for_key };
    efi_bs_call!(wait_for_event, 1, &mut wait_for_key, &mut index);
    efi_con_in_call!(read_key_stroke, efi_st_get!(con_in), &mut key);
}

#[allow(non_snake_case, non_upper_case_globals)]
pub static mut _PCD_GET_MODE_BOOL_PcdComponentNameDisable: Boolean = Boolean::FALSE;
#[allow(non_snake_case, non_upper_case_globals)]
pub static mut _PCD_GET_MODE_BOOL_PcdComponentName2Disable: Boolean = Boolean::FALSE;

#[allow(unused_unsafe)]
#[allow(const_item_mutation)]
pub fn efi_lib_install_driver_binding_component_name2(image_handle: Handle, driver_binding: *mut crate::protocols::driver_binding::Protocol, driver_binding_handle: Handle, component_name:*mut crate::protocols::component_name::Protocol, component_name2: *mut crate::protocols::component_name::Protocol) -> Status {
    assert!(driver_binding.is_null() == false);
    unsafe {
        (*driver_binding).image_handle = image_handle;
        (*driver_binding).driver_binding_handle = driver_binding_handle;
        #[allow(unused_unsafe)]
        #[allow(const_item_mutation)]
        let mut guid_ptr = &mut crate::protocols::driver_binding::PROTOCOL_GUID as *mut Guid;
        let status = ((*BS).install_protocol_interface)(&mut (*driver_binding).driver_binding_handle, guid_ptr, InterfaceType::NativeInterface, driver_binding as *mut c_void);
        if component_name.is_null() || _PCD_GET_MODE_BOOL_PcdComponentNameDisable == Boolean::TRUE {
            if component_name2.is_null() || _PCD_GET_MODE_BOOL_PcdComponentName2Disable == Boolean::TRUE {
                status
            } else {
                assert!(component_name2.is_null() == false);
                guid_ptr = &mut crate::protocols::component_name::PROTOCOL_GUID as *mut Guid;
                ((*BS).install_protocol_interface)(&mut (*driver_binding).driver_binding_handle, guid_ptr, InterfaceType::NativeInterface, driver_binding as *mut c_void)
            }
        } else {
            assert!(component_name.is_null() == false);
            if component_name2.is_null() || _PCD_GET_MODE_BOOL_PcdComponentName2Disable == Boolean::TRUE {
                guid_ptr = &mut crate::protocols::component_name::PROTOCOL_GUID as *mut Guid;
                ((*BS).install_protocol_interface)(&mut (*driver_binding).driver_binding_handle, guid_ptr, InterfaceType::NativeInterface, driver_binding as *mut c_void)
            } else {
                assert!(component_name.is_null() == false);
                assert!(component_name2.is_null() == false);
                guid_ptr = &mut crate::protocols::component_name::PROTOCOL_GUID as *mut Guid;
                ((*BS).install_protocol_interface)(&mut (*driver_binding).driver_binding_handle, guid_ptr, InterfaceType::NativeInterface, driver_binding as *mut c_void);

                guid_ptr = &mut crate::protocols::component_name::PROTOCOL_GUID as *mut Guid;
                ((*BS).install_protocol_interface)(&mut (*driver_binding).driver_binding_handle, guid_ptr, InterfaceType::NativeInterface, driver_binding as *mut c_void)
            }
        }
    }
}

// from https://github.com/x1tan/rust-uefi-runtime-driver/blob/master/src/utils.rs
static mut GDB_ATTACHED: bool = false;
pub fn wait_for_debugger() {
    unsafe {
        while !GDB_ATTACHED {
            asm!("pause");
        }
    }
}

pub fn disable_write_protection() {
    unsafe {
        asm!("
        mov rax, cr0
        btc rax, 16
        mov cr0, rax
        ")
    }
}

pub fn enable_write_protection() {
    unsafe {
        asm!("
        mov rax, cr0
        bts rax, 16
        mov cr0, rax
        ")
    }
}