use native_dialog::{DialogBuilder, MessageLevel};
use std::ffi::{c_char, CStr};
use std::path::{Path, PathBuf};
use std::{env, fs};
use std::process::{Command, Stdio};
use uuid::Uuid;

#[no_mangle]
pub extern "C" fn prepare_project_directory(working_dir: *const c_char, is_cli: i32) -> i32 {
    let working_dir = unsafe { CStr::from_ptr(working_dir) };
    let working_dir = PathBuf::from(working_dir.to_str().unwrap());

    if !fs::exists(&working_dir).unwrap() {
        fs::create_dir_all(&working_dir).unwrap();
    }

    println!("setting env::current_dir to: {}", working_dir.display());
    env::set_current_dir(&working_dir).unwrap();

    let is_cli = is_cli != 0;

    if !ensure_project_files(&working_dir, is_cli) {
        println!("ensure_project_files failed");
        return 0;
    }

    if !ensure_assembly(&working_dir, is_cli) {
        return 0;
    }

    1
}

fn ensure_assembly(working_dir: &Path, is_cli: bool) -> bool {
    let assembly_name = working_dir.file_name().unwrap().to_str().unwrap();
    let assembly_path = format!("{}/bin/Debug/net8.0/{assembly_name}.dll", working_dir.display());
    println!("expected assembly location: {}", assembly_name);
    while !fs::exists(&assembly_path).unwrap() {
        println!("compiling C# assembly at {}", env::current_dir().unwrap().display());
        let Ok(spawn) = Command::new("dotnet")
            .stdout(if is_cli { Stdio::inherit() } else { Stdio::piped() })
            .stderr(if is_cli { Stdio::inherit() } else { Stdio::piped() })
            .args(["build"])
            .spawn() else
        {
            if is_cli {
                println!("Failed to run `dotnet` command. Please install .NET 8.0 or later.");
                return false;
            }
            if !ask_user_for_confirmation(format!("Failed to run `dotnet` command. Please install .NET 8.0 or later. Try again?")) {
                return false;
            }
            continue;
        };
        match spawn.wait_with_output() {
            Ok(result) => {
                let assembly_exists = fs::exists(&assembly_path).unwrap();
                let success = result.status.success();
                if success && assembly_exists {
                    return true;
                }
                let log_file = "error.log";
                let mut combined = vec![];
                combined.extend_from_slice(&result.stdout);
                combined.extend_from_slice("\n".as_bytes());
                combined.extend_from_slice(&result.stderr);
                combined.extend_from_slice("\n".as_bytes());
                if success && !assembly_exists {
                    combined.extend_from_slice("WTF: dotnet command succeed, but assembly still doesn't exist".as_bytes());
                    combined.extend_from_slice("\n".as_bytes());
                }
                // let _ = io::stdout().write_all(&combined);
                if let Err(err) = fs::write(log_file, &combined) {
                    println!("failed to write log: {}\n{}", err, String::from_utf8_lossy(&combined));
                }
                if is_cli {
                    println!("failed to compile C# project.");
                    return false;
                }
                if !ask_user_for_confirmation(format!("failed to compile C# project: See `{log_file}` for details. Try again?")) {
                    return false;
                }
                continue;
            }
            Err(err) => {
                if is_cli {
                    println!("failed to compile C# project due to {}", err);
                    return false;
                }
                if !ask_user_for_confirmation(format!("failed to compile C# project: {err}. Try again?")) {
                    return false;
                }
                continue;
            }
        }
    }
    true
}

fn ask_user_for_confirmation(msg: String) -> bool {
    DialogBuilder::message()
        .set_level(MessageLevel::Error)
        .set_title("Fyrox C# SDK")
        .set_text(msg)
        .confirm()
        .show()
        .unwrap()
}

fn ensure_project_files(working_dir: &Path, is_cli: bool) -> bool {
    let dir_name = working_dir.file_name().unwrap().to_str().unwrap();
    let sln_path = format!("{}/{}.sln", working_dir.to_str().unwrap(), dir_name);

    if !fs::exists(&sln_path).unwrap() {
        if !is_cli {
            let confirm = ask_user_for_confirmation(format!(
                "There is no C# project in {:?}. Create new C# project here?",
                working_dir.as_os_str()
            ));
            if !confirm {
                return false;
            }
        }

        let solution_uuid = Uuid::new_v4();
        let project_uuid = Uuid::new_v4();
        let sln = include_str!("template.sln.txt")
            .replace(
                "${solution_uuid}",
                solution_uuid.to_string().to_uppercase().as_str(),
            )
            .replace(
                "${project_uuid}",
                project_uuid.to_string().to_uppercase().as_str(),
            )
            .replace("${project_name}", dir_name);

        fs::write(&sln_path, sln).unwrap();
        fs::write(
            format!("{}/Program.cs", working_dir.to_str().unwrap()),
            include_str!("Program.cs.txt"),
        )
        .unwrap();

        let run = include_str!("Debug.run.xml.txt").replace("${project_name}", dir_name);
        let _ = fs::create_dir_all(format!("{}/.run", working_dir.to_str().unwrap()));
        fs::write(
            format!("{}/.run/Debug.run.xml", working_dir.to_str().unwrap()),
            run,
        )
        .unwrap();
    }

    let csproj = include_str!("template.csproj.txt").replace(
        "${editor_installation_path}",
        env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .to_str()
            .unwrap(),
    );
    fs::write(
        format!("{}/{}.csproj", working_dir.to_str().unwrap(), dir_name),
        csproj,
    )
    .unwrap();

    // open::that(&sln_path).unwrap();
    true
}
