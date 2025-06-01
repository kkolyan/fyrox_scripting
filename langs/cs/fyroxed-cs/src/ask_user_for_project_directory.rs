use std::ffi::CString;
use std::str::FromStr;
use native_dialog::DialogBuilder;

#[no_mangle]
pub extern "C" fn ask_user_for_project_directory() -> *const u8 {
    loop {
        let result = DialogBuilder::file()
            .set_title("Fyrox C# SDK: choose project directory")
            .open_single_dir()
            .show();

        let path = result.unwrap();
        let Some(dialog_path) = path else {
            return std::ptr::null();
        };
        let mut path = dialog_path.clone();

        #[cfg(target_os = "windows")]
        if !path.exists() {
            path = path.parent().unwrap().to_path_buf();
            if !path.is_dir() {
                continue;
            }
            println!("DialogBuilder returned non-existing path {:?}. On Windows there are scenarios when that means that user want to select a project.", dialog_path.to_string_lossy());
        }

        println!("Notice: string allocated here is known to be leaked, because it's not critical in expected amounts.");

        return CString::from_str(path.to_str().unwrap()).unwrap()
            .as_bytes_with_nul()
            .to_vec()
            .leak()
            .as_ptr();
    }
}