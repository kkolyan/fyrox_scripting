//! Editor with your game connected to it as a plugin.

use std::env;
use std::path::{Path, PathBuf};
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::plugin::EditorPlugin;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let project_dir: PathBuf = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        Default::default()
    };
    let project_dir = dunce::canonicalize(project_dir).unwrap();
    println!("project directory: {}", project_dir.display());
    Log::set_verbosity(MessageKind::Warning);
    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new(Some(StartupData {
        working_directory: project_dir.clone(),
        scenes: vec!["data/scene.rgs".into()],
    }));

    // Dynamic linking with hot reloading.
    #[cfg(feature = "dylib")]
    {
        #[cfg(target_os = "windows")]
        let file_name = "fyrox-lua_dylib.dll";
        #[cfg(target_os = "linux")]
        let file_name = "libfyrox-lua_dylib.so";
        #[cfg(target_os = "macos")]
        let file_name = "libfyrox-lua_dylib.dylib";
        editor.add_dynamic_plugin(file_name, true, true).unwrap();
    }

    // Static linking.
    #[cfg(not(feature = "dylib"))]
    {
        let scripts_dir = Path::join(&project_dir, "scripts");
        if let Err(err) = editor.add_dynamic_plugin_custom(fyrox_lite_lua_lib::LuaPlugin::new(
            scripts_dir,
            true
        )) {
            Log::err(err);
        }

        editor.add_editor_plugin(LuaPluginRefreshOnFocus);
    }

    editor.run(event_loop)
}

#[cfg(not(feature = "dylib"))]
struct LuaPluginRefreshOnFocus;

#[cfg(not(feature = "dylib"))]
impl EditorPlugin for LuaPluginRefreshOnFocus {

    fn on_resumed(&mut self, #[allow(unused_variables)] editor: &mut Editor) {
        for it in editor.engine.plugins_mut() {
            if let Some(it) = it.cast_mut::<fyrox_lite_lua_lib::LuaPlugin>() {
                it.check_for_script_changes();
            }
        }
    }
}