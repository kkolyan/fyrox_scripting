use fyrox::plugin::DynamicPlugin;
use fyrox_lite_cs_lib::fyrox_c_plugin::CPlugin;
use std::ffi::{c_char, CStr};
use std::path::PathBuf;

pub mod executor_cs;

#[no_mangle]
pub unsafe fn fyrox_c_plugin(reloadable_assembly_path: *const c_char) -> Box<dyn DynamicPlugin> {
    let path: Option<PathBuf>;
    if reloadable_assembly_path.is_null() {
        path = None
    } else {
        let c_path = CStr::from_ptr(reloadable_assembly_path);
        path = Some(c_path.to_str().unwrap().into());
    }
    Box::new(CPlugin::new(path))
}
