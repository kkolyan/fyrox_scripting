use native_dialog::DialogBuilder;

pub fn ask_user_for_directory(title: &str) -> Option<String> {
    let s: Option<String>;
    loop {
        let result = DialogBuilder::file()
            .set_title(title)
            .open_single_dir()
            .show();

        let path = result.unwrap();
        let Some(dialog_path) = path else {
            s = None;
            break;
        };
        let mut path = dialog_path.clone();

        #[cfg(target_os = "windows")]
        if !path.exists() {
            path = path.parent().unwrap().to_path_buf();
            if !path.is_dir() {
                continue;
            }
            println!(
                "DialogBuilder returned non-existing path {:?}. \
                On Windows there are scenarios when that means that user want to select a project.",
                dialog_path.to_string_lossy()
            );
        }

        println!(
            "Notice: string allocated here is known to be leaked, \
            because it's not critical in expected amounts."
        );

        s = Some(path.to_str().unwrap().to_owned());
        break;
    }
    s
}
