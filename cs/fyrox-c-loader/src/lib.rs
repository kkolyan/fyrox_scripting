use fyrox::plugin::{DynamicPlugin, Plugin};

type CPluginEntryPoint = fn(hot_reload_enabled: bool) -> Box<dyn DynamicPlugin>;

pub struct DynamicPluginProxy(Box<dyn DynamicPlugin>);

pub fn fyrox_c_plugin(hot_reload_enabled: bool) -> DynamicPluginProxy {

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
        DynamicPluginProxy(entry(hot_reload_enabled))
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

    fn reload(&mut self, fill_and_register: &mut dyn FnMut(&mut dyn Plugin) -> Result<(), String>) -> Result<(), String> {
        self.0.reload(fill_and_register)
    }
}