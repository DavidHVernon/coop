use libc::c_char;
use std::ffi::CStr;

pub fn c_str_to_string(str_ptr: *mut c_char) -> String {
    unsafe { String::from_utf8_lossy(CStr::from_ptr(str_ptr).to_bytes()).to_string() }
}
