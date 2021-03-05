//! PCI I/O Protocol
//! 
//! This protocol is used by 
//! code, typically drivers, running in the EFI boot services environment to access memory and I/O on a PCI 
//! controller. In particular, functions for managing devices on PCI buses are defined here.
use core::ffi::c_void;

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0x4CF5B200, 0x68B8, 0x4CA5, 0x9E, 0xEC, &[0xB2, 0x3E, 0x3F, 0x50, 0x2, 0x9A]
);
type FnPollMem = *mut c_void;
type FnPollIo = *mut c_void;
type FnMem = *mut c_void;
type FnIo = *mut c_void;
type FnPci = *mut c_void;
type FnCopyMem = *mut c_void;
type FnMap = *mut c_void;
type FnUnmap = *mut c_void;
type FnAllocateBuffer = *mut c_void;
type FnFreeBuffer = *mut c_void;
type FnFlush = *mut c_void;
type FnGetLocation = *mut c_void;
type FnAttributes = *mut c_void;
type FnGetBarAttributes = *mut c_void;
type FnSetBarAttributes = *mut c_void;
#[repr(C)]
pub struct Protocol {
    pub poll_mem: FnPollMem,
    pub poll_io: FnPollIo,
    pub mem: FnMem,
    pub io: FnIo,
    pub pci: FnPci,
    pub copy_mem: FnCopyMem,
    pub map: FnMap,
    pub unmap: FnUnmap,
    pub allocate_buffer: FnAllocateBuffer,
    pub free_buffer: FnFreeBuffer,
    pub flush: FnFlush,
    pub get_location: FnGetLocation,
    pub attributes: FnAttributes,
    pub get_bar_attributes: FnGetBarAttributes,
    pub set_bar_attributes: FnSetBarAttributes,
    pub rom_size: u64,
    pub rom_image: *mut c_void
}