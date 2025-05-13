fn main() {
    let lib_name = "agent";

    #[cfg(target_os = "windows")]
    let file_name = format!("{}.dll", lib_name);
    #[cfg(target_os = "linux")]
    let file_name = format!("lib{}.so", lib_name);
    #[cfg(target_os = "macos")]
    let file_name = format!("lib{}.dylib", lib_name);

    let file_name = file_name.as_str();
    unsafe {
        let lib = libloading::Library::new(file_name)
            .map_err(|e| e.to_string())
            .unwrap();
        let entry = lib
            .get::<fn()>("hello_dylib_issue".as_bytes())
            .map_err(|e| e.to_string())
            .unwrap();
        entry();
    }
}
