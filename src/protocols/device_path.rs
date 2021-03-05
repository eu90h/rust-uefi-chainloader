use core::{ptr::NonNull, convert::TryInto, ffi::c_void, ptr::null_mut};
use r_efi::{efi::{Handle, Status}, protocols::device_path};
use crate::{buffer::Buffer, efi_alloc::{alloc_pool, free_pool}, utils::{copy_mem, get_handles_supporting, get_protocol_instance}};
use crate::efi_bs_call;
use crate::efi_proto_call;
pub const MEDIA_FILEPATH_DP: u8 = 0x04; //https://dox.ipxe.org/DevicePath_8h.html
pub const END_INSTANCE_DEVICE_PATH_SUBTYPE:u8 = 0x01;
pub const END_ENTIRE_DEVICE_PATH_SUBTYPE:u8 = 0xFF;

#[repr(C)]
pub struct FileDevicePath {
    pub header: r_efi::protocols::device_path::Protocol,
    pub path_name: [r_efi::base::Char16; 1],
}

pub const END_DEVICE_PATH_LENGTH: usize = core::mem::size_of::<r_efi::protocols::device_path::Protocol>();

#[allow(dead_code)]
pub const END_INSTANCE_DEVICE_PATH: r_efi::protocols::device_path::Protocol = r_efi::protocols::device_path::Protocol {
    r#type: r_efi::protocols::device_path::TYPE_END,
    sub_type: END_INSTANCE_DEVICE_PATH_SUBTYPE,
    length: [END_DEVICE_PATH_LENGTH as u8, 0]
};

pub const END_DEVICE_PATH: r_efi::protocols::device_path::Protocol = r_efi::protocols::device_path::Protocol {
    r#type: r_efi::protocols::device_path::TYPE_END,
    sub_type: END_ENTIRE_DEVICE_PATH_SUBTYPE,
    length: [END_DEVICE_PATH_LENGTH as u8, 0]
};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct MemoryMappedDevicePath {
    pub header: r_efi::protocols::device_path::Protocol,
    pub memory_type: r_efi::system::MemoryType,
    pub start: r_efi::base::PhysicalAddress,
    pub end: r_efi::base::PhysicalAddress,
}

pub fn device_path_from_handle(handle: Handle) -> NonNull<device_path::Protocol> {
    let mut device_path: Handle = null_mut();
    #[allow(const_item_mutation)]
    let status = efi_bs_call!(handle_protocol, handle, &mut device_path::PROTOCOL_GUID, &mut device_path);
    match status {
        Status::SUCCESS => {
            assert_eq!(device_path.is_null(), false);
            match NonNull::new(device_path as *mut device_path::Protocol) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("device_path_from_handle failed to make non-null pointer")
                }
            }
        }
        Status::UNSUPPORTED => {
            panic!("device_path_from_handle failed: EFI_UNSUPPORTED")
        }
        Status::INVALID_PARAMETER => {
            panic!("device_path_from_handle failed: EFI_INVALID_PARAMETER")
        }
        _ => {
            panic!("device_path_from_handle failed: returned unexpected status code")
        }
    }
}

pub fn append_device_path(first_device_path: *mut device_path::Protocol, second_device_path: *mut device_path::Protocol) -> NonNull<device_path::Protocol> {
    assert_eq!(first_device_path.is_null(), false);
    assert_eq!(second_device_path.is_null(), false);
    let maybe_handles = get_handles_supporting(&r_efi::protocols::device_path_utilities::PROTOCOL_GUID);
    if maybe_handles.is_none() {
        panic!("append_device_path failed: no handles supported given protocol")
    }
    let handles: Buffer<Handle> = maybe_handles.unwrap();
    for handle_ptr in handles {
        if handle_ptr.is_null() {
            panic!("append_device_path failed: handle_ptr was null")
        }
        let handle = unsafe { *handle_ptr };
        let device_path_utils: *mut r_efi::protocols::device_path_utilities::Protocol = get_protocol_instance(handle, &r_efi::protocols::device_path_utilities::PROTOCOL_GUID).as_ptr();
        if device_path_utils.is_null() == false {
            unsafe {
                let ret = efi_proto_call!(device_path_utils,append_device_path,first_device_path, second_device_path);
                if !ret.is_null() {
                    match NonNull::new(ret) {
                        Some(non_null) => {
                            return non_null;
                        }
                        None => {
                            panic!("append_device_path failed to make non-null pointer")
                        }
                    }
                } else {
                    panic!("append_device_path failed: NULL was returned, possibly indicating an allocation failure")
                }
            }
        }
    }
    panic!("append_device_path failed: no handles found that support the device path utilities protocol")
}

pub fn file_device_path(device: Handle, file_name: *mut r_efi::base::Char16, file_name_size_in_bytes: usize) -> *mut r_efi::protocols::device_path::Protocol {
    let mut fdp_size_original: usize = core::mem::size_of::<r_efi::protocols::device_path::Protocol>() + file_name_size_in_bytes;
    let buf = alloc_pool::<u8>(fdp_size_original + END_DEVICE_PATH_LENGTH);
    let fdp = buf as *mut FileDevicePath;
    let fdp_size = fdp_size_original.try_into();
    if fdp_size.is_err() {
        panic!("file_device_path failed to convert fdp_size from usize to isize")
    }
    let fdp_size = fdp_size.unwrap();
    let end: *mut r_efi::protocols::device_path::Protocol = unsafe { buf.offset(fdp_size) } as *mut r_efi::protocols::device_path::Protocol;
    unsafe {
        (*fdp).header.r#type = r_efi::protocols::device_path::TYPE_MEDIA;
        (*fdp).header.sub_type = MEDIA_FILEPATH_DP;

        let dst: *mut [u8;2] = &mut (*fdp).header.length;
        let fdp_size_ptr: *mut usize = &mut fdp_size_original;
        copy_mem(dst as *mut core::ffi::c_void, fdp_size_ptr as *mut c_void, 2);

        let dst: *mut [u16; 1] = &mut (*fdp).path_name;
        let src = file_name as *mut u8;
        copy_mem(dst as *mut core::ffi::c_void, src as *mut c_void, file_name_size_in_bytes);

        let p: *const device_path::Protocol = &END_DEVICE_PATH;
        copy_mem(end as *mut c_void, p as *mut c_void, END_DEVICE_PATH_LENGTH);
    }
   
    let mut device_path = device_path_from_handle(device).as_ptr();
    device_path = append_device_path(device_path, fdp as *mut device_path::Protocol).as_ptr();
    assert!(!device_path.is_null());
    free_pool(buf);
    device_path
}