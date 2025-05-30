//! Editor with your game connected to it as a plugin.

use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyroxed_base::fyrox::event_loop::EventLoop;
use fyroxed_base::Editor;
use fyroxed_base::StartupData;
use native_dialog::DialogBuilder;
use native_dialog::MessageLevel::Warning;
use std::ffi::{c_char, CStr, CString};
use std::{env, fs};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use uuid::Uuid;
use fyrox_lite_cs_lib::fyrox_c_plugin::CPlugin;

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

        return CString::from_str(path.to_str().unwrap()).unwrap()
            .as_bytes_with_nul()
            .to_vec()
            .leak()
            .as_ptr();
    }
}

#[no_mangle]
pub unsafe extern "C" fn fyrox_lite_editor_run(working_dir: *const c_char, assembly_path: *const c_char) {
    Log::set_verbosity(MessageKind::Warning);
    let working_dir = CStr::from_ptr(working_dir).to_str().expect("failed to parse working directory argument");
    let working_dir = dunce::canonicalize(working_dir).unwrap();
    let event_loop = EventLoop::new().unwrap();
    println!("Using working dir: {}", working_dir.display());
    let Some(solution_file) = ensure_project_files(&working_dir) else {
        println!("ensure_project_files returned false");
        return;
    };
    // Fyrox has some places that rely on `env::current_dir == working_dir`. To avoid potential
    // issues just make them the same
    env::set_current_dir(&working_dir).unwrap();
    // let _ = open::that(solution_file);
    let mut editor = Editor::new(Some(StartupData {
        working_directory: working_dir.clone(),
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
        let assembly_path = CStr::from_ptr(assembly_path).to_str().unwrap();
        let plugin = CPlugin::new(Some(assembly_path.into()));
        if let Err(err) = editor.add_dynamic_plugin_custom(plugin) {
            Log::err(err);
        }

        // editor.add_editor_plugin(CSharpPluginRefreshOnFocus);
    }

    editor.run(event_loop)
}

fn ensure_project_files(working_dir: &Path) -> Option<PathBuf> {
    let dir_name = working_dir.file_name().unwrap().to_str().unwrap();
    let sln_path = format!("{}/{}.sln", working_dir.to_str().unwrap(), dir_name);
    
    if !fs::exists(&sln_path).unwrap() {
        let confirm = DialogBuilder::message()
            .set_level(Warning)
            .set_title("Fyrox C# SDK")
            .set_text(format!("There is no C# project in {:?}. Create new C# project here?", working_dir.as_os_str()))
            .confirm()
            .show()
            .unwrap();
        if !confirm {
            return None;
        }
        
        let solution_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();
        let sln = include_str!("template.sln.txt")
            .replace("${solution_uuid}", solution_uuid.to_string().to_uppercase().as_str())
            .replace("${project_uuid}", project_uuid.to_string().to_uppercase().as_str())
            .replace("${project_name}", dir_name)
            ;

        fs::write(&sln_path, sln).unwrap();
        fs::write(format!("{}/Program.cs", working_dir.to_str().unwrap()), include_str!("Program.cs.txt")).unwrap();
    }

    let csproj = include_str!("template.csproj.txt")
        .replace("${editor_installation_path}", env::current_exe().unwrap().parent().unwrap().to_str().unwrap());
    fs::write(format!("{}/{}.csproj", working_dir.to_str().unwrap(), dir_name), csproj).unwrap();

    open::that(&sln_path).unwrap();
    Some(sln_path.into())
}
// #[cfg(not(feature = "dylib"))]
// struct CSharpPluginRefreshOnFocus;
// 
// #[cfg(not(feature = "dylib"))]
// impl EditorPlugin for CSharpPluginRefreshOnFocus {
// 
//     fn on_resumed(&mut self, #[allow(unused_variables)] editor: &mut Editor) {
//         for it in editor.engine.plugins_mut() {
//             if let Some(it) = it.cast_mut::<CPlugin>() {
//                 it.check_for_script_changes();
//             }
//         }
//     }
// }