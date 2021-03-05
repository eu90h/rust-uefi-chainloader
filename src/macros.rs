//TODO: merge println/printlns macros together, if possible.
///print a str type to con_out, followed by a newline which in UEFI land is \r\n.
#[macro_export]
macro_rules! println {
    ($string: expr) => {
        unsafe {
            if crate::is_at_runtime() {
                panic!("println called at runtime.")
            }
            let mut __s = crate::utils::str_to_u16(concat!($string, "\r\n\0"));
            ((*(*crate::ST).con_out).output_string)((*crate::ST).con_out,__s.as_mut_ptr());
        }
    };
}

///print a String type to con_out, followed by a newline which in UEFI land is \r\n.
#[macro_export]
macro_rules! printlns {
    ($string: expr) => {
        unsafe {
            if crate::is_at_runtime() {
                panic!("printlns called at runtime.")
            }
            let mut __s = $string.clone();
            __s.push_str("\r\n\0");
            let mut __s = crate::utils::string_to_u16(&__s);
            ((*(*crate::ST).con_out).output_string)((*crate::ST).con_out,__s.as_mut_ptr());
        }
    };
}

///call a boot services function
#[macro_export]
macro_rules! efi_bs_call {
    ($fn_name:ident, $arg: expr $(,$args:expr)*) => {
        unsafe { ((*crate::BS).$fn_name)($arg$(,$args)*) }
    }
}

#[macro_export]
macro_rules! efi_st_call {
    ($fn_name:ident, $arg: expr $(,$args:expr)*) => {
        unsafe { ((*crate::ST).$fn_name)($arg$(,$args)*) }
    }
}

#[macro_export]
macro_rules! efi_con_out_call {
    ($fn_name:ident()) => {
        unsafe { ((*(*crate::ST).con_out).$fn_name)() }
    };
    
    ($fn_name:ident, $arg: expr $(,$args:expr)*) => {
        unsafe { ((*(*crate::ST).con_out).$fn_name)($arg$(,$args)*) }
    }
}
#[macro_export]
macro_rules! efi_con_in_call {
    ($fn_name:ident()) => {
        unsafe { ((*(*crate::ST).con_in).$fn_name)() }
    };
    
    ($fn_name:ident, $arg: expr $(,$args:expr)*) => {
        unsafe { ((*(*crate::ST).con_in).$fn_name)($arg$(,$args)*) }
    }
}
#[macro_export]
macro_rules! efi_st_get {
    ($property:ident) => {
        #[allow(unused_unsafe)]
        unsafe { (*crate::ST).$property }
    }
}
#[macro_export]
macro_rules! efi_proto_call { 
    ($protocol_handle: expr, $fn_name:ident, $arg: expr $(,$args:expr)*) => {
        #[allow(unused_unsafe)]
        unsafe { ((*$protocol_handle).$fn_name)($arg$(,$args)*) }
    }
}
#[macro_export]
macro_rules! efi_rs_call {
    ($fn_name:ident, $arg: expr $(,$args:expr)*) => {
        unsafe { ((*crate::RS).$fn_name)($arg$(,$args)*) }
    }
}