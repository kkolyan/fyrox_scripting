//! Editor with your game connected to it as a plugin.

use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox::core::Uuid;
use fyrox_build_tools::{BuildProfile, CommandDescriptor};
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::plugin::EditorPlugin;
use fyroxed_base::settings::Settings;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;
use lite_runtime::script_failure::ScriptFailureHandler;
use std::path::{Path, PathBuf};
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<_>>();

    let is_cli;
    let project_dir: PathBuf = if args.len() > 1 {
        is_cli = true;
        PathBuf::from(&args[1])
    } else {
        is_cli = false;
        // There are usability issues on Macos, and on Linux probably too.
        // Let's support UI selector for Windows only

        #[cfg(target_os = "windows")]
        {
            let s = ask_user::ask_user_for_directory("Fyrox Lua SDK: choose project directory");
            match s {
                None => {
                    return;
                }
                Some(s) => s.into(),
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            println!("usage: ./fyroxed_lua <path to the project>");
            return;
        }
    };
    let _ = fs::create_dir_all(&project_dir);
    let project_dir = dunce::canonicalize(project_dir).unwrap();
    env::set_current_dir(&project_dir).unwrap();

    println!("project directory: {}", project_dir.display());

    extract_annotations();
    create_gitignore();

    let settings = ensure_lua_profiles(&project_dir);

    Log::set_verbosity(MessageKind::Warning);
    let event_loop = EventLoop::new().unwrap();
    let mut editor = Editor::new_with_settings(
        Some(StartupData {
            working_directory: project_dir.clone(),
            scenes: vec!["data/scene.rgs".into()],
        }),
        settings,
    );

    editor.user_project_icon =
        Some(include_bytes!("../../../../target/fyrox_lua_003.ico").to_vec());
    editor.user_project_name = "/Lua".to_string();
    editor.user_project_version = "/0.1".to_string();

    let scripts_dir = Path::join(&project_dir, "scripts");
    if !fs::exists(&scripts_dir).unwrap() {
        fs::create_dir(&scripts_dir).unwrap();
        fs::write(
            format!("{}/Game.lua", scripts_dir.display()),
            include_str!("Game.lua.txt")
                .replace("${SCRIPT_UUID}", Uuid::new_v4().to_string().as_str()),
        )
        .unwrap();
        fs::write(
            format!("{}/.gitignore", scripts_dir.display()),
            include_str!("template.gitignore.txt"),
        )
        .unwrap();
    }
    if let Err(err) = editor.add_dynamic_plugin_custom(fyrox_lite_lua_lib::LuaPlugin::new(
        scripts_dir,
        true,
        ScriptFailureHandler::new_for_editor(is_cli),
    )) {
        Log::err(err);
    }

    editor.add_editor_plugin(LuaPluginRefreshOnFocus);

    editor.run(event_loop)
}

fn create_gitignore() {
    if fs::exists(".gitignore").unwrap() {
        let _ = fs::write(
            ".gitignore",
            "/.annotations\n/settings.ron\n*.log\n/.idea\n",
        );
    }
}

fn extract_annotations() {
    const ANNOTATIONS_SUB_DIR: &str = ".annotations";

    if !fs::exists(ANNOTATIONS_SUB_DIR).unwrap() {
        fs::create_dir_all(ANNOTATIONS_SUB_DIR).unwrap();
        let src = env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join("fyrox_lite_lua_annotations.tar.gz");

        if src.exists() {
            println!("extracting annotations");
            comprexor::Extractor::new(src.to_str().unwrap(), ANNOTATIONS_SUB_DIR)
                .extract()
                .unwrap();
        } else {
            println!("WARNING: annotations not found at {}", src.display());
        }
    }
}

fn ensure_lua_profiles(_working_dir: &Path) -> Settings {
    let (mut settings, loaded) = match Settings::load() {
        Ok(it) => (it, true),
        Err(_) => (Settings::default(), false),
    };
    settings.build.profiles.clear();
    let sdk_dir = env::current_exe().unwrap().parent().unwrap().to_path_buf();

    #[cfg(target_os = "windows")]
    let build = CommandDescriptor {
        command: "cmd".to_string(),
        args: vec!["/C".to_string(), "echo".to_string()],
        environment_variables: vec![],
        skip_passthrough_marker: true,
    };

    #[cfg(not(target_os = "windows"))]
    let build = CommandDescriptor {
        command: format!("sh"),
        args: vec!["-c".to_string(), "echo".to_string()],
        environment_variables: vec![],
        skip_passthrough_marker: true,
    };

    settings.build.profiles.push(BuildProfile {
        name: "Lua Project".to_string(),
        build_commands: vec![build],
        run_command: CommandDescriptor {
            #[cfg(target_os = "windows")]
            command: format!("{}/fyrox_lite_lua.exe", sdk_dir.display()),

            #[cfg(not(target_os = "windows"))]
            command: format!("{}/fyrox_lite_lua", sdk_dir.display()),

            args: vec![],
            environment_variables: vec![],
            skip_passthrough_marker: true,
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
