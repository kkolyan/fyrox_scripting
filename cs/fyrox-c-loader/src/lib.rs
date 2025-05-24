use std::ffi::{c_char, CString};
use std::path::PathBuf;
use std::ptr::null;
use std::str::FromStr;
use fyrox::plugin::{DynamicPlugin, Plugin};

type CPluginEntryPoint = fn(reloadable_assembly_path: *const c_char) -> Box<dyn DynamicPlugin>;

pub struct DynamicPluginProxy(Box<dyn DynamicPlugin>);

pub fn fyrox_c_plugin(reloadable_assembly_path: Option<PathBuf>) -> DynamicPluginProxy {

    #[cfg(target_os = "windows")]
    let file_name = "fyrox_c.dll";
    #[cfg(target_os = "linux")]
    let file_name = "libfyrox_c.so";
    #[cfg(target_os = "macos")]
    let file_name = "libfyrox_c.dylib";
    unsafe {
        let lib = libloading::Library::new(file_name)
            .map_err(|e| e.to_string())
            .unwrap();

        let entry = lib
            .get::<CPluginEntryPoint>("fyrox_c_plugin".as_bytes())
            .map_err(|e| e.to_string()).unwrap();
        match reloadable_assembly_path {
            None => DynamicPluginProxy(entry(null())),
            Some(reloadable_assembly_path) => {
                let path = reloadable_assembly_path.to_str().unwrap();
                let path = CString::from_str(path).unwrap();
                DynamicPluginProxy(entry(path.as_ptr()))
            }
        }
    }
}

impl DynamicPlugin for DynamicPluginProxy {
    fn display_name(&self) -> String {
        self.0.display_name()
    }

    fn is_reload_needed_now(&self) -> bool {
        self.0.is_reload_needed_now()
    }

    fn as_loaded_ref(&self) -> &dyn Plugin {
        self.0.as_loaded_ref()
    }

    fn as_loaded_mut(&mut self) -> &mut dyn Plugin {
        self.0.as_loaded_mut()
    }

    fn is_loaded(&self) -> bool {
        self.0.is_loaded()
    }
    
    fn prepare_to_reload(&mut self) {
        self.0.prepare_to_reload()
    }

    fn reload(&mut self, fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>) -> Result<(), String> {
        self.0.reload(fill_and_register)
    }
}