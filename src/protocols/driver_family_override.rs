#![allow(dead_code)]
//! Driver Family Override
//!
//! The driver family override protocol provides a method for an EFI driver to opt in to a higher priorty rule for connecting drivers to controllers in the EFI Boot Service.

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0xb1ee129e,
    0xda36,
    0x4181,
    0x91,
    0xf8,
    &[0x04,0xa4,0x92,0x37,0x66,0xa7]
);

type FnGetVersion = extern "win64" fn (this: *mut Protocol) -> u32;

#[repr(C)]
pub struct Protocol {
    pub get_version: FnGetVersion
}