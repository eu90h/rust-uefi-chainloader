#![allow(dead_code)]
use core::{ffi::c_void, ptr::{NonNull, null_mut}};
use crate::{buffer::Buffer, protocols::device_path::file_device_path, efi_alloc::{alloc_pool, free_pool}, utils::{get_handles_supporting, get_protocol_instance, this_img}};
use r_efi::{efi::{Boolean, Char16, Guid, Handle, Status}};
use r_efi::protocols::loaded_image::Protocol as LoadedImageProtocol;
use r_efi::protocols::loaded_image::PROTOCOL_GUID as LoadedImageProtocolGUID;
use r_efi::protocols::simple_file_system::Protocol as SimpleFileSystemProtocol;
use r_efi::protocols::simple_file_system::PROTOCOL_GUID as SimpleFileSystemProtocolGUID;
use r_efi::protocols::file::Protocol as File;
use r_efi::protocols::device_path::Protocol as DevicePathProtocol;

pub type UefiResult<T> = core::result::Result<T, r_efi::base::Status>;

pub fn get_loaded_image_protocol(handle: Handle) -> NonNull<LoadedImageProtocol> {
    assert_eq!(handle.is_null(), false);
    let mut parent_loaded_image: *mut LoadedImageProtocol = alloc_pool::<LoadedImageProtocol>(1);
    let parent_loaded_image_ptr: *mut *mut LoadedImageProtocol = &mut parent_loaded_image;
    let mut guid = LoadedImageProtocolGUID;
    let status = efi_bs_call!(open_protocol, handle, &mut guid, parent_loaded_image_ptr as *mut *mut core::ffi::c_void, handle, null_mut(), r_efi::system::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
    match status {
        Status::SUCCESS => {
            assert_eq!(parent_loaded_image_ptr.is_null(), false);
            assert_eq!(parent_loaded_image.is_null(), false);
            match NonNull::new(parent_loaded_image) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("get_parents_loaded_image_protocol failed to make non-null pointer")
                }
            }
        }
        Status::INVALID_PARAMETER => {
            panic!("get_parents_loaded_image_protocol failed: EFI_INVALID_PARAMETER")
        }
        Status::UNSUPPORTED => {
            panic!("get_parents_loaded_image_protocol failed: EFI_UNSUPPORTED")
        }
        Status::ACCESS_DENIED => {
            panic!("get_parents_loaded_image_protocol failed: EFI_ACCESS_DENIED")
        }
        Status::ALREADY_STARTED => {
            panic!("get_parents_loaded_image_protocol failed: EFI_ALREADY_STARTED")
        }
        _ => {
            panic!("get_parents_loaded_image_protocol failed: returned unexpected status code")
        }
    }
}

fn get_parents_loaded_image_protocol(parent: Handle) -> NonNull<LoadedImageProtocol> {
    assert_eq!(parent.is_null(), false);
    get_loaded_image_protocol(parent)
}

fn open_parent_device_fs(parent_device: Handle, parent: Handle) -> NonNull<SimpleFileSystemProtocol> {
    assert_eq!(parent_device.is_null(), false);
    assert_eq!(parent.is_null(), false);
    let mut sfs: *mut SimpleFileSystemProtocol = alloc_pool::<SimpleFileSystemProtocol>(1);
    let sfs_ptr: *mut *mut SimpleFileSystemProtocol = &mut sfs;
    let mut guid = SimpleFileSystemProtocolGUID;
    let status = efi_bs_call!(open_protocol, parent_device, &mut guid, sfs_ptr as *mut *mut core::ffi::c_void, parent, null_mut(), r_efi::system::OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
    match status {
        Status::SUCCESS => {
            assert_eq!(sfs_ptr.is_null(), false);
            assert_eq!(sfs.is_null(), false);
            match NonNull::new(sfs) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("open_parent_device_fs failed to make non-null pointer")
                }
            }
        }
        Status::INVALID_PARAMETER => {
            panic!("open_parent_device_fs failed: EFI_INVALID_PARAMETER")
        }
        Status::UNSUPPORTED => {
            panic!("open_parent_device_fs failed: EFI_UNSUPPORTED")
        }
        Status::ACCESS_DENIED => {
            panic!("open_parent_device_fs failed: EFI_ACCESS_DENIED")
        }
        Status::ALREADY_STARTED => {
            panic!("open_parent_device_fs failed: EFI_ALREADY_STARTED")
        }
        _ => {
            panic!("open_parent_device_fs failed: returned unexpected status code")
        }
    }
}

fn get_parent_device(parent: Handle) -> Handle {
    assert_eq!(parent.is_null(), false);
    let parent_loaded_image_protocol: NonNull<LoadedImageProtocol> = get_parents_loaded_image_protocol(parent);
    let parent_device: Handle = unsafe { (*parent_loaded_image_protocol.as_ptr()).device_handle };
    assert_eq!(parent_device.is_null(), false);
    parent_device
}

fn get_parent_file_path(parent: Handle) -> NonNull<DevicePathProtocol> {
    assert_eq!(parent.is_null(), false);
    let parent_loaded_image_protocol: NonNull<LoadedImageProtocol> = get_parents_loaded_image_protocol(parent);
    let file_path: *mut DevicePathProtocol = unsafe { (*parent_loaded_image_protocol.as_ptr()).file_path };
    assert_eq!(file_path.is_null(), false);
    match NonNull::new(file_path) {
        Some(non_null) => {
            return non_null;
        }
        None => {
            panic!("get_parent_file_path failed to make non-null pointer")
        }
    }
}

pub fn file_path_to_device_path(path: *mut Char16, file_name_size_in_bytes: usize) -> NonNull<DevicePathProtocol> {
    let sfs_guid = &r_efi::protocols::simple_file_system::PROTOCOL_GUID;
    let maybe_handles = get_handles_supporting(sfs_guid);
    if maybe_handles.is_none() {
        panic!("file_path_to_device_path failed: no handles support simple file system protocol")
    }
    let handles: Buffer<Handle> = maybe_handles.unwrap();
    for handle_ptr in handles {
        let handle = unsafe { *handle_ptr };
        let fs: *mut SimpleFileSystemProtocol = get_protocol_instance(handle, sfs_guid).as_ptr();
        let fs_root: *mut File = open_volume(fs).as_ptr();
        let mut file: *mut File = alloc_pool::<File>(1);
        let file_ptr: *mut *mut File = &mut file;
        let status = efi_proto_call!(fs_root, open, fs_root, file_ptr, path, r_efi::protocols::file::MODE_READ, r_efi::protocols::file::READ_ONLY);
        if status == Status::SUCCESS {
            let status = efi_proto_call!(fs_root, close, file);
            if status != Status::SUCCESS {
                panic!("file_path_to_device_path failed: failed to close file")
            }
            let device_path: *mut DevicePathProtocol = file_device_path(handle, path, file_name_size_in_bytes);
            match NonNull::new(device_path) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("file_path_to_device_path failed to make non-null pointer")
                }
            }
        }
    }
    panic!("file_path_to_device_path failed to find a suitable device handle")
}

pub fn close_protocol(protocol_handle: *mut c_void, protocol_guid: &Guid, agent_handle: Handle, controller_handle: Handle) {
    assert_eq!(protocol_handle.is_null(), false);
    let mut g = *protocol_guid;
    let status = efi_bs_call!(close_protocol, protocol_handle, &mut g, agent_handle, controller_handle);
    match status {
        Status::SUCCESS => {}
        Status::INVALID_PARAMETER => {
            panic!("close_protocol failed: EFI_INVALID_PARAMETER")
        }
        Status::NOT_FOUND => {
            panic!("close_protocol failed: EFI_NOT_FOUND")
        }
        _ => {
            panic!("close_protocol failed: returned unexpected status code")
        }
    }
}

pub fn read_file(path: *mut Char16) -> Buffer<u8> {
    assert_eq!(path.is_null(), false);
    let file: *mut File = load_file(this_img(), path).as_ptr();
    let mut size = 10_000_000;
    let mut buf: Buffer<u8> = Buffer::with_capacity(size);
    let status = efi_proto_call!(file, read, file, &mut size, buf.as_mut_ptr() as *mut c_void);
    match status {
        Status::SUCCESS => {
            unsafe { buf.set_count(size) };
            assert_eq!(buf.as_mut_ptr().is_null(), false);
            assert!(buf.count > 0);
            assert!(buf.count < 10_000_000);
            return buf;
        }
        Status::NO_MEDIA => {
            panic!("read_file failed: EFI_NO_MEDIA")
        }
        Status::DEVICE_ERROR => {
            panic!("read_file failed: EFI_DEVICE_ERROR")
        }
        Status:: VOLUME_CORRUPTED => {
            panic!("read_file failed: EFI_VOLUME_CORRUPTED")
        }
        Status::BUFFER_TOO_SMALL => {
            panic!("read_file failed: EFI_BUFFER_TOO_SMALL")
        }
        _ => {
            panic!("read_file failed: returned unexpected status code");
        }
    }
}

pub fn load_image(image_data: Buffer<u8>, parent_image: Handle, device_path: *mut DevicePathProtocol) -> NonNull<Handle> {
    assert_eq!(parent_image.is_null(), false);
    assert_eq!(device_path.is_null(), false);
    let image_data_ptr: *mut c_void = image_data.as_mut_ptr() as *mut c_void;
    let image_data_size = image_data.count;
    let image: *mut Handle = alloc_pool::<Handle>(1);
    let status = efi_bs_call!(load_image, Boolean::TRUE, parent_image, device_path, image_data_ptr, image_data_size, image);
    match status {
        Status::SUCCESS => {
            assert_eq!(image.is_null(), false);
            assert_eq!(unsafe {*image}.is_null(), false);
            match NonNull::new(image) {
                Some(non_null) => {
                    free_pool(image_data.data);
                    return non_null;
                }
                None => {
                    panic!("load_image failed to make non-null pointer")
                }
            }
        }
        Status::NOT_FOUND => {
            panic!("load_image failed: EFI_NOT_FOUND")
        }
        Status::INVALID_PARAMETER => {
            panic!("load_image failed: EFI_INVALID_PARAMETER")
        }
        Status::UNSUPPORTED => {
            panic!("load_image failed: EFI_UNSUPPORTED")
        }
        Status::OUT_OF_RESOURCES => {
            panic!("load_image failed: EFI_OUT_OF_RESOURCES")
        }
        Status::LOAD_ERROR => {
            panic!("load_image failed: EFI_LOAD_ERROR")
        }
        Status::DEVICE_ERROR => {
            panic!("load_image failed: EFI_DEVICE_ERROR")
        }
        Status::ACCESS_DENIED => {
            panic!("load_image failed: EFI_ACCESS_DENIED")
        }
        Status::SECURITY_VIOLATION => {
            panic!("load_image failed: EFI_SECURITY_VIOLATION")
        }
        _ => {
            panic!("load_image failed: returned unexpected status code")
        }
    }
}

pub fn load_image_from_path(path: *mut Char16, path_size: usize) -> Handle {
    assert_eq!(path.is_null(), false);
    if path_size == 0 {
        panic!("load_image_from_path failed: path_size is zero")
    }
    let file_contents: Buffer<u8> = read_file(path);
    let parent_image: Handle = this_img();
    let device_path: *mut DevicePathProtocol = file_path_to_device_path(path, path_size).as_ptr();
    let new_image: Handle = unsafe {*load_image(file_contents, parent_image, device_path).as_ptr()};
    new_image
}

pub fn start_image(image: Handle) -> ! {
    let status = efi_bs_call!(start_image, image, null_mut(), null_mut());
    if status != Status::SUCCESS {
        panic!("Start failed.")
    }
    panic!("Shouldn't have gotten here.");
}

pub fn load_and_start(path: *mut Char16, path_size: usize) -> ! {
    assert_eq!(path.is_null(), false);
    if path_size == 0 {
        panic!("load_and_start failed: path_size is zero")
    }
    let new_image: Handle = load_image_from_path(path, path_size);
    start_image(new_image)
}

fn open_volume(fs: *mut SimpleFileSystemProtocol) -> NonNull<File> {
    assert_eq!(fs.is_null(), false);
    let mut drive_root: *mut File = alloc_pool::<File>(1);
    let drive_root_ptr: *mut *mut File = &mut drive_root;
    let status = efi_proto_call!(fs, open_volume, fs, drive_root_ptr);
    match status {
        Status::SUCCESS => {
            assert_eq!(drive_root_ptr.is_null(), false);
            assert_eq!(drive_root.is_null(), false);
            match NonNull::new(drive_root) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("open_volume failed to make non-null pointer")
                }
            }
        }
        Status::NO_MEDIA => {
            panic!("open_volume failed: EFI_NO_MEDIA")
        }
        Status::UNSUPPORTED => {
            panic!("open_volume failed: EFI_UNSUPPORTED")
        }
        Status::ACCESS_DENIED => {
            panic!("open_volume failed: EFI_ACCESS_DENIED")
        }
        Status::DEVICE_ERROR => {
            panic!("open_volume failed: EFI_DEVICE_ERROR")
        }
        Status::VOLUME_CORRUPTED => {
            panic!("open_volume failed: EFI_VOLUME_CORRUPTED")
        }
        Status::OUT_OF_RESOURCES => {
            panic!("open_volume failed: EFI_OUT_OF_RESOURCES")
        }
        Status::MEDIA_CHANGED => {
            panic!("open_volume failed: EFI_MEDIA_CHANGED")
        }
        _ => {
            panic!("open_volume failed: returned unexpected status code")
        }
    }
}

pub fn open_file(fs_root: *mut File, path: *mut Char16) -> NonNull<File> {
    assert_eq!(fs_root.is_null(), false);
    assert_eq!(path.is_null(), false);
    let mut file: *mut File = alloc_pool::<File>(1);
    let file_ptr: *mut *mut File = &mut file;
    let status = efi_proto_call!(fs_root, open, fs_root, file_ptr, path, r_efi::protocols::file::MODE_READ, r_efi::protocols::file::READ_ONLY);
    match status {
        Status::SUCCESS => {
            assert_eq!(file_ptr.is_null(), false);
            assert_eq!(file.is_null(), false);
            match NonNull::new(file) {
                Some(non_null) => {
                    return non_null;
                }
                None => {
                    panic!("load_file failed to make non-null pointer")
                }
            }
        }
        Status::NOT_FOUND => {
            panic!("load_file failed: EFI_NOT_FOUND")
        }
        Status::NO_MEDIA => {
            panic!("load_file failed: EFI_NO_MEDIA")
        }
        Status::MEDIA_CHANGED => {
            panic!("load_file failed: EFI_MEDIA_CHANGED")
        }
        Status::DEVICE_ERROR => {
            panic!("load_file failed: EFI_DEVICE_ERROR")
        }
        Status::VOLUME_CORRUPTED => {
            panic!("load_file failed: EFI_VOLUME_CORRUPTED")
        }
        Status::WRITE_PROTECTED => {
            panic!("load_file failed: EFI_WRITE_PROTECTED")
        }
        Status::ACCESS_DENIED => {
            panic!("load_file failed: EFI_ACCESS_DENIED")
        } 
        Status::OUT_OF_RESOURCES => {
            panic!("load_file failed: EFI_OUT_OF_RESOURCES")
        }
        Status::VOLUME_FULL => {
            panic!("load_file failed: EFI_VOLUME_FULL")
        }
        _ => {
            panic!("load_file failed: returned unexpected status code")
        }
    }
}

pub fn load_file(parent: Handle, path: *mut Char16) -> NonNull<File> {
    assert_eq!(parent.is_null(), false);
    assert_eq!(path.is_null(), false);
    let parent_device: Handle = get_parent_device(parent);
    let parent_fs: *mut SimpleFileSystemProtocol = open_parent_device_fs(parent_device, parent).as_ptr();
    let fs_root: *mut File = open_volume(parent_fs).as_ptr();
    open_file(fs_root, path)
}