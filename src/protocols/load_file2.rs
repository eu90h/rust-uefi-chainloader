#![allow(dead_code)]
//! Load File2 Protocol
//!
//! The load file 2 protocol is used to obtain files from arbitrary devices that are not used as boot options.

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0x4006c0c1,
    0xfcb3,
    0x403e,
    0x99,
    0x6d,
    &[0x4a, 0x6c, 0x87, 0x24, 0xe0, 0x6d]
);

///The LoadFile() function interprets the device-specific FilePath parameter, returns the entire file 
///into Buffer, and sets BufferSize to the amount of data returned. If Buffer is NULL, then the size of 
///the file is returned in BufferSize. If Buffer is not NULL, and BufferSize is not large enough to 
///hold the entire file, then EFI_BUFFER_TOO_SMALL is returned, and BufferSize is updated to indicate 
///the size of the buffer needed to obtain the file. In this case, no data is returned in Buffer. 
///FilePath contains the file path value in the boot selection option. Normally the firmware would 
///implement the policy on how to handle an inexact boot file path; however, since in this case the firmware 
///cannot interpret the file path, the LoadFile() function is responsible for implementing the policy.
pub type FnLoadFile = extern "win64" fn(*mut Protocol, *mut r_efi::protocols::device_path::Protocol, r_efi::base::Boolean, *mut u64, *mut core::ffi::c_void) -> r_efi::base::Status;

#[repr(C)]
pub struct Protocol {
///The LoadFile() function interprets the device-specific FilePath parameter, returns the entire file 
///into Buffer, and sets BufferSize to the amount of data returned. If Buffer is NULL, then the size of 
///the file is returned in BufferSize. If Buffer is not NULL, and BufferSize is not large enough to 
///hold the entire file, then EFI_BUFFER_TOO_SMALL is returned, and BufferSize is updated to indicate 
///the size of the buffer needed to obtain the file. In this case, no data is returned in Buffer. 
///FilePath contains the file path value in the boot selection option. Normally the firmware would 
///implement the policy on how to handle an inexact boot file path; however, since in this case the firmware 
///cannot interpret the file path, the LoadFile() function is responsible for implementing the policy.
    pub load_file: FnLoadFile,
}