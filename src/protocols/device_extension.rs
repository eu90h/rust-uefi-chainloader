#[inline]
pub const fn signature_16(a: u8, b: u8) -> u16 {
    a as u16 | ((b as u16) << 8) 
}

#[inline]
pub const fn signature_32(a: u8, b: u8, c: u8, d: u8) -> u32 {
    signature_16(a, b) as u32 | ((signature_16(c,d) as u32) << 16) 
}

#[inline]
pub const fn signature_64(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8, g: u8, h: u8) -> u64 {
    signature_32(a,b,c,d) as u64 | ((signature_32(e,f,g,h) as u64) << 32) 
}

pub const DEVICE_EXTENSION_SIGNATURE: u64 = signature_32('R' as u8,'T' as u8,'D' as u8,'R' as u8) as u64;

#[repr(C)]
pub struct DeviceExtension {
    pub signature: u64,
    pub device_protocol: crate::protocols::runtime_driver::Protocol,
    pub device_handle: r_efi::efi::Handle,
    pub pci_io: *mut crate::protocols::pci_io::Protocol
}