#![allow(dead_code)]
//! Driver Supported Efi Version
//!
//! The driver supported EFI version protocol provides information about the version of the EFI specification that a driver is following. This protocol is required for EFI
//! drivers that are on PCI and other plug-in cards.

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields( 0x5c198761, 0x16a8, 0x4e69, 0x97, 0x2c,     
    &[0x89, 0xd6, 0x79, 0x54, 0xf8, 0x1d]
);

#[repr(C)]
pub struct Protocol {
    pub length: u32,
    pub firmware_version: u32
}