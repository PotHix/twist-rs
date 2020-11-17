use super::super::endpoints;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn search(token: *mut c_char, query: *mut c_char) -> *mut c_char {
    // Gets a Cstr from a c_char pointer
    let c_str_token = unsafe { CStr::from_ptr(token) };

    // Transforms the CStr type into a &str
    let str_token = c_str_token.to_str().unwrap();

    let c_str_query = unsafe { CStr::from_ptr(query) };
    let str_query = c_str_query.to_str().unwrap();

    // Since we use String as arguments for `search` we have to convert the &str to a String (heap
    // allocated string of variable size) to use as function arguments
    let res = endpoints::search::search(str_token.to_string(), str_query.to_string())
        .expect("No valid result returned from endpoints::search::search");

    let response_text = res
        .text()
        .expect("Couldn't convert the response object into text");

    let s = CString::new(response_text).expect("Couldn't convert the response text into a CString");
    return s.into_raw();
}

#[no_mangle]
pub extern "C" fn string_free(s: *mut c_char) {
    unsafe {
        if s.is_null() {
            return;
        }
        CString::from_raw(s)
    };
}
