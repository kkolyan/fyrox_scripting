//! Editor with your game connected to it as a plugin.

use std::ffi::{c_char, CStr, CString};
use std::path::PathBuf;
use std::str::FromStr;
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;
use native_dialog::DialogBuilder;

#[no_mangle]
pub extern "C" fn ask_user_for_project_directory() -> *const u8 {
    let result = DialogBuilder::file()
        .set_title("Fyrox Editor with C# scripting: choose project directory")
        .open_single_dir()
        .show();

    let path = result.unwrap();
    let Some(path) = path else {
        return std::ptr::null();
    };
    CString::from_str(path.to_str().unwrap()).unwrap()
        .as_bytes_with_nul()
        .to_vec()
        .leak()
        .as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn fyrox_lite_editor_run(working_dir: *const c_char) {
    Log::set_verbosity(MessageKind::Information);
    let working_dir = CStr::from_ptr(working_dir).to_str().expect("failed to parse working directory argument");
    let event_loop = EventLoop::new().unwrap();
    println!("Using working dir: {}", working_dir);
    let mut editor = Editor::new(Some(StartupData {
        working_directory: PathBuf::from(working_dir),
        scenes: vec!["data/scene.rgs".into()],
    }));

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "fyrox-c_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libfyrox-c_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libfyrox-c_dylib.dylib";
        editor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        let plugin = fyrox_c_loader::fyrox_c_plugin(true);
        if let Err(err) = editor.add_dynamic_plugin_custom(plugin) {
            Log::err(err);
        }

        // editor.add_editor_plugin(LuaPluginRefreshOnFocus);
    }

    editor.run(event_loop)
}

// #[cfg(not(feature = "dylib"))]
// struct LuaPluginRefreshOnFocus;

// #[cfg(not(feature = "dylib"))]
// impl EditorPlugin for LuaPluginRefreshOnFocus {
//
//     fn on_resumed(&mut self, #[allow(unused_variables)] editor: &mut Editor) {
//         for it in editor.engine.plugins_mut() {
//             if let Some(it) = it.cast_mut::<fyrox_lua::LuaPlugin>() {
//                 it.check_for_script_changes();
//             }
//         }
//     }
// }