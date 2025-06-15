//! Editor with your game connected to it as a plugin.

use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox_build_tools::{BuildProfile, CommandDescriptor};
use fyrox_lite_cs_lib::fyrox_c_plugin::CPlugin;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::plugin::EditorPlugin;
use fyroxed_base::settings::Settings;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;
use std::env;
use std::ffi::{c_char, CStr};
use std::path::PathBuf;

#[no_mangle]
pub unsafe extern "C" fn fyrox_lite_editor_run(
    working_dir: *const c_char,
    assembly_path: *const c_char,
) {
    Log::set_verbosity(MessageKind::Warning);
    let working_dir = CStr::from_ptr(working_dir)
        .to_str()
        .expect("failed to parse working directory argument");
    let working_dir = dunce::canonicalize(working_dir).unwrap();
    let event_loop = EventLoop::new().unwrap();
    println!("Using working dir: {}", working_dir.display());
    // Fyrox has some places that rely on `env::current_dir == working_dir`. To avoid potential
    // issues just make them the same
    env::set_current_dir(&working_dir).unwrap();
    let settings = ensure_cs_profiles(&working_dir);

    let mut editor = Editor::new_with_settings(
        Some(StartupData {
            working_directory: working_dir.clone(),
            scenes: vec!["data/scene.rgs".into()],
        }),
        settings,
    );

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
        let assembly_path = CStr::from_ptr(assembly_path).to_str().unwrap();
        let plugin = CPlugin::new(Some(assembly_path.into()));
        if let Err(err) = editor.add_dynamic_plugin_custom(plugin) {
            Log::err(err);
        }

        editor.add_editor_plugin(CSharpPluginRefreshOnFocus);
    }

    editor.run(event_loop)
}

fn ensure_cs_profiles(_working_dir: &PathBuf) -> Settings {
    let (mut settings, loaded) = match Settings::load() {
        Ok(it) => (it, true),
        Err(_) => (Settings::default(), false),
    };
    settings.build.profiles.clear();
    settings.build.profiles.push(BuildProfile {
        name: "C# Project".to_string(),
        build_commands: vec![CommandDescriptor {
            command: "dotnet".to_string(),
            args: vec!["build".to_string()],
            environment_variables: vec![],
        }],
        run_command: CommandDescriptor {
            command: "dotnet".to_string(),
            args: vec!["run".to_string()],
            environment_variables: vec![],
        },
    });
    if loaded {
        settings.force_save();
    }
    settings
}

#[cfg(not(feature = "dylib"))]
struct CSharpPluginRefreshOnFocus;

#[cfg(not(feature = "dylib"))]
impl EditorPlugin for CSharpPluginRefreshOnFocus {
    fn on_update(&mut self, editor: &mut Editor) {
        for it in editor.engine.plugins_mut() {
            if let Some(it) = it.cast_mut::<CPlugin>() {
                it.check_for_script_changes();
            }
        }
    }
}
