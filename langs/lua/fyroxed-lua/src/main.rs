//! Editor with your game connected to it as a plugin.

use std::env;
use std::path::{Path, PathBuf};
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox_build_tools::{BuildProfile, CommandDescriptor};
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::plugin::EditorPlugin;
use fyroxed_base::Editor;
use fyroxed_base::settings::Settings;
use fyroxed_base::StartupData;

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let project_dir: PathBuf = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        let s = ask_user::ask_user_for_directory("Fyrox Lua SDK: choose project directory");
        match s {
            None => {
                return;
            }
            Some(s) => s.into()
        }
    };
    let project_dir = dunce::canonicalize(project_dir).unwrap();
    env::set_current_dir(&project_dir).unwrap();

    println!("project directory: {}", project_dir.display());

    let settings = ensure_lua_profiles(&project_dir);

    Log::set_verbosity(MessageKind::Warning);
    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new_with_settings(
        Some(StartupData {
            working_directory: project_dir.clone(),
            scenes: vec!["data/scene.rgs".into()],
        }),
        settings
    );

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

fn ensure_lua_profiles(_working_dir: &PathBuf) -> Settings {
    let (mut settings, loaded) = match Settings::load() {
        Ok(it) => (it, true),
        Err(_) => (Settings::default(), false),
    };
    settings.build.profiles.clear();
    let sdk_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();
    settings.build.profiles.push(BuildProfile {
        name: "Lua Project".to_string(),
        build_commands: vec![
            CommandDescriptor {
                command: format!("cmd"),
                args: vec!["/C".to_string(), "echo".to_string()],
                environment_variables: vec![],
            }
        ],
        run_command: CommandDescriptor {
            command: format!("{}/fyrox_lite_lua.exe", sdk_dir.display()),
            args: vec![],
            environment_variables: vec![],
        },
    });
    if loaded {
        settings.force_save();
    }
    settings
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