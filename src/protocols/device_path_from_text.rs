#![allow(dead_code)]
//! Device Path from Text Protocol
//!
//! The device path from text protocol converts text to device paths.

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0x5c99a21,
    0xc70f,
    0x4ad2,
    0x8a,
    0x5f,
    &[0x35,0xdf,0x33,0x43,0xf5, 0x1e]
);

pub type FnDevicePathFromTextPath = extern "win64" fn(*mut r_efi::base::Char16) -> *mut r_efi::protocols::device_path::Protocol;
pub type FnDevicePathFromTextNode = extern "win64" fn(*mut r_efi::base::Char16) -> *mut r_efi::protocols::device_path::Protocol;

#[repr(C)]
pub struct Protocol {
    pub convert_device_node_from_text: FnDevicePathFromTextNode,
    pub convert_device_path_from_text: FnDevicePathFromTextPath
}