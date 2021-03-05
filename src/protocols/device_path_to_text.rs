#![allow(dead_code)]
//! Device Path to Text Protocol
//!
//! The device path to text protocol converts device nodes and paths to text.

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0x8b843e20,
    0x8132,
    0x4852,
    0x90,
    0xcc,
    &[0x55,0x1a,0x4e,0x4a,0x7f,0x1c]
);

pub type FnDevicePathToTextPath = extern "win64" fn(*const r_efi::protocols::device_path::Protocol, display_only: r_efi::base::Boolean, allow_shortcuts: r_efi::base::Boolean) -> *mut r_efi::base::Char16;
pub type FnDevicePathToTextNode = extern "win64" fn(device_node: *const r_efi::protocols::device_path::Protocol, display_only: r_efi::base::Boolean, allow_shortcuts: r_efi::base::Boolean) -> *mut r_efi::base::Char16;

#[repr(C)]
pub struct Protocol {
    /// Converts a device node to its text representation.
    /// 
    /// Parameters
    /// ----------
    /// The device_node parameter points to the device node to be converted.
    /// 
    /// If display_only is TRUE, then the shorter text representation of the display node is used, where applicable. If display_only is FALSE, then the longer text
    /// representation of the display node is used.
    /// 
    /// If allow_shortcuts is TRUE, then the shortcut forms of text representation for a device node can be used, where applicable.
    pub convert_device_node_to_text: FnDevicePathToTextNode,
    pub convert_device_path_to_text: FnDevicePathToTextPath
}