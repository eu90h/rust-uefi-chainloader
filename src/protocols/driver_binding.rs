use r_efi::{efi::Handle, protocols::device_path::Protocol as DevicePathProtocol, efi::Status};

pub const PROTOCOL_GUID: r_efi::base::Guid = r_efi::base::Guid::from_fields(
    0x18A031AB,0xB443,0x4D1A,0xA5,0xC0,&[0x0C,0x09,0x26,0x1E,0x9F,0x71]
);

#[doc = ""]
#[doc = " This protocol provides the services required to determine if a driver supports a given controller."]
#[doc = " If a controller is supported, then it also provides routines to start and stop the controller."]
#[doc = ""]
#[repr(C)]
pub struct Protocol {
    pub supported: FnDriverBindingSupported,
    pub start: FnDriverBindingStart,
    pub stop: FnDriverBindingStop,
    #[doc = ""]
    #[doc = " The version number of the UEFI driver that produced the"]
    #[doc = " EFI_DRIVER_BINDING_PROTOCOL. This field is used by"]
    #[doc = " the EFI boot service ConnectController() to determine"]
    #[doc = " the order that driver's Supported() service will be used when"]
    #[doc = " a controller needs to be started. EFI Driver Binding Protocol"]
    #[doc = " instances with higher Version values will be used before ones"]
    #[doc = " with lower Version values. The Version values of 0x0-"]
    #[doc = " 0x0f and 0xfffffff0-0xffffffff are reserved for"]
    #[doc = " platform/OEM specific drivers. The Version values of 0x10-"]
    #[doc = " 0xffffffef are reserved for IHV-developed drivers."]
    #[doc = ""]
    pub version: u32,
    #[doc = ""]
    #[doc = " The image handle of the UEFI driver that produced this instance"]
    #[doc = " of the EFI_DRIVER_BINDING_PROTOCOL."]
    #[doc = ""]
    pub image_handle: Handle,
    #[doc = ""]
    #[doc = " The handle on which this instance of the"]
    #[doc = " EFI_DRIVER_BINDING_PROTOCOL is installed. In most"]
    #[doc = " cases, this is the same handle as ImageHandle. However, for"]
    #[doc = " UEFI drivers that produce more than one instance of the"]
    #[doc = " EFI_DRIVER_BINDING_PROTOCOL, this value may not be"]
    #[doc = " the same as ImageHandle."]
    #[doc = ""]
    pub driver_binding_handle: Handle,
}

pub type FnDriverBindingSupported =
    unsafe extern "win64" fn(
        This: *mut Protocol,
        ControllerHandle: Handle,
        RemainingDevicePath: *mut DevicePathProtocol,
    ) -> Status;

pub type FnDriverBindingStart = 
    unsafe extern "win64" fn(
        This: *mut Protocol,
        ControllerHandle: Handle,
        RemainingDevicePath: *mut DevicePathProtocol,
    ) -> Status;

pub type FnDriverBindingStop = 
    unsafe extern "win64" fn(
        This: *mut Protocol,
        ControllerHandle: Handle,
        NumberOfChildren: usize,
        ChildHandleBuffer: *mut Handle,
    ) -> Status;