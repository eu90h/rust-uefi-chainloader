#![allow(dead_code)]
//! Runtime Driver Protocol
//!
//! The runtime driver protocol...

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0xd487ddb4,
    0x008b,
    0x11d9,
    0xaf,
    0xdc,
    &[0x00, 0x10, 0x83, 0xff, 0xca, 0x4d],
);

#[repr(C)]
pub struct Protocol {
    pub value: r_efi::base::Handle,
}