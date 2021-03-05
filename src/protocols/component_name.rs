#![allow(dead_code)]
//! Component Name Protocol
//! 
//! The component name protocol allows a driver to provide user readable names of UEFI drivers and the controllers managed by that driver.
//! This protocol is used by platform management utilities that wish to display names of components.

pub type FnGetDriverName = unsafe extern "win64" fn(this: *mut Protocol, language: *mut r_efi::base::Char8, driver_name: *mut *mut r_efi::base::Char16,) -> r_efi::base::Status;
pub type FnGetControllerName = unsafe extern "win64" fn(
    this: *mut Protocol,
    controller_handle: r_efi::base::Handle,
    child_handle: r_efi::base::Handle,
    language: *mut r_efi::base::Char8,
    controller_name: *mut *mut r_efi::base::Char16,
) -> r_efi::base::Status;

#[repr(C)]
pub struct Protocol {
    pub get_driver_name: FnGetDriverName,
    pub get_controller_name: FnGetControllerName,
    #[doc = ""]
    #[doc = " A Null-terminated ASCII string that contains one or more"]
    #[doc = " ISO 639-2 language codes. This is the list of language codes"]
    #[doc = " that this protocol supports."]
    #[doc = ""]
    pub supported_languages: *mut r_efi::base::Char8,
}

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(0x6A7A5CFF, 0xE8D9, 0x4F70, 0xBA, 0xDA, &[0x75, 0xAB, 0x30, 0x25, 0xCE, 0x14]);