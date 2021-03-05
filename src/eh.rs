use r_efi::efi::Status;
use crate::utils::{exit, halt, is_at_runtime, this_img};
use alloc::string::String;

#[panic_handler]
#[cold]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    if is_at_runtime() {
        halt()
    } else {
        if let Some(args) = info.message() {
            if let Some(msg) = args.as_str() {
                printlns!(String::from(msg))
            }
        }
        exit(this_img(), Status::ABORTED)
    }
}

#[lang = "eh_personality"]
#[cold]
extern fn eh_personality() {}

#[no_mangle]
#[cold]
fn rust_oom() -> ! {
    if is_at_runtime() {
        halt()
    } else {
        exit(this_img(), Status::ABORTED)
    }
}