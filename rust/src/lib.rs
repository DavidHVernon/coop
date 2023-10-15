use libc::c_char;
use util::c_str_to_string;

mod util;

extern crate libc;

#[no_mangle]
extern "C" fn node(name: *mut c_char) {
    let name = c_str_to_string(name);
    println!("Hello, World.");
    match name.as_str() {
        "Under Construction" => {
            panic!("Under Construction");
        }
        _ => {}
    }
}

#[no_mangle]
extern "C" fn node_1(name: *mut c_char, arg_1: *mut c_char) {
    let name = c_str_to_string(name);
    let _arg_1 = c_str_to_string(arg_1);

    match name {
        _ => {}
    }
}

#[no_mangle]
extern "C" fn node_2(name: *mut c_char, arg_1: *mut c_char, arg_2: *mut c_char) {
    let name = c_str_to_string(name);
    let _arg_1 = c_str_to_string(arg_1);
    let _arg_2 = c_str_to_string(arg_2);

    match name {
        _ => {}
    }
}

#[no_mangle]
extern "C" fn node_3(
    name: *mut c_char,
    arg_1: *mut c_char,
    arg_2: *mut c_char,
    arg_3: *mut c_char,
) {
    let name = c_str_to_string(name);
    let _arg_1 = c_str_to_string(arg_1);
    let _arg_2 = c_str_to_string(arg_2);
    let _arg_3 = c_str_to_string(arg_3);

    match name {
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
