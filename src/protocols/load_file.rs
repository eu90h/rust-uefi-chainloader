#![allow(dead_code)]
//! Load File Protocol
//!
//! The load file protocol is used to obtain files, usually used as boot options, from arbitrary devices.

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0x56EC3091,
    0x954C,
    0x11d2,
    0x8e,
    0x3f,
    &[0x00,0xa0, 0xc9,0x69,0x72,0x3b]
);

pub type FnLoadFile = extern "win64" fn(*mut Protocol, *mut r_efi::protocols::device_path::Protocol, r_efi::base::Boolean, *mut u64, *mut core::ffi::c_void) -> r_efi::base::Status;

#[repr(C)]
pub struct Protocol {
    pub load_file: FnLoadFile,
}