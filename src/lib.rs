pub mod comments;
pub mod endpoints;
pub mod search;

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

const URL: &'static str = "https://api.twist.com";
const API_VERSION: &'static str = "/api/v3";

#[no_mangle]
pub extern "C" fn search_ffi(token: *mut c_char, query: *mut c_char) -> *mut c_char {
    let c_str_token = unsafe { CStr::from_ptr(token) };
    let str_token = c_str_token.to_str().unwrap();

    let c_str_query = unsafe { CStr::from_ptr(query) };
    let str_query = c_str_query.to_str().unwrap();

    let res = endpoints::search::search(str_token.to_string(), str_query.to_string());

    let s = CString::new(res.unwrap().text().unwrap()).unwrap();
    return s.into_raw();
}
