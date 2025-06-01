use std::ffi::CString;
use std::str::FromStr;

#[no_mangle]
pub extern "C" fn ask_user_for_project_directory() -> *const u8 {
    let s = ask_user::ask_user_for_directory("Fyrox C# SDK: choose project directory");
    match s {
        None => std::ptr::null(),
        Some(s) => CString::from_str(s.as_str()).unwrap()
            .as_bytes_with_nul()
            .to_vec()
            .leak()
            .as_ptr(),
    }
}