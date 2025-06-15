use std::collections::BTreeMap;
use std::{
    fmt::Display,
    fs,
};
use to_vec::ToVec;

#[derive(Default)]
pub struct HierarchicalCodeBase {
    pub mods: BTreeMap<String, Module>,
}
impl HierarchicalCodeBase {
    pub fn write_rust(&self, target_dir: &str) {
        write_rust_mods(target_dir, self.mods.values().to_vec().as_slice());
    }
    pub fn write_lua(&self, target_dir: &str) {
        write_lua_mods(target_dir, self.mods.values().to_vec().as_slice());
    }
    pub fn write_md(&self, target_dir: &str) {
        write_md_mods(target_dir, self.mods.values().to_vec().as_slice());
    }
}

pub struct Module {
    pub name: String,
    pub code: Option<String>,
    pub children: BTreeMap<String, Module>,
}

impl Module {

    pub fn merge(&mut self, m: Module) {
        if m.code.is_some() {
            self.code = m.code;
        }
        for (key, child) in m.children {
            let prev_child = self.children.remove(&key);
            let new_child = match prev_child {
                None => child,
                Some(mut prev) => {
                    prev.merge(child);
                    prev
                }
            };
            self.children.insert(key, new_child);
        }
    }

    pub fn root() -> Self {
        Module {
            name: "".to_string(),
            code: None,
            children: Default::default(),
        }
    }

    pub fn code(name: impl Display, code: impl Display) -> Module {
        Module {
            name: name.to_string(),
            code: Some(code.to_string()),
            children: Default::default(),
        }
    }

    pub fn no_code(name: impl Display) -> Module {
        Module {
            name: name.to_string(),
            code: None,
            children: Default::default(),
        }
    }
    
    pub fn set_code(&mut self, code: impl Display) {
        self.code = Some(code.to_string());
    }

    pub fn add_child(&mut self, m: Module) {
        self.children.insert(m.name.clone(), m);
    }

    pub fn write_rust(&self, parent_dir: &str) {
        let dir = format!("{}/{}", parent_dir, self.name);

        write_rust_mods(&dir, self.children.values().to_vec().as_slice());

        if let Some(code) = &self.code {
            let file = format!("{}/{}.rs", parent_dir, self.name);
            fs::write(&file, code).unwrap();
            crate::fmt::fmt_file(file);
        }
    }
    pub fn write_lua(&self, parent_dir: &str) {
        let dir = format!("{}/{}", parent_dir, self.name);

        write_lua_mods(&dir, self.children.values().to_vec().as_slice());

        if let Some(code) = &self.code {
            let file = format!("{}/{}.lua", parent_dir, self.name);
            fs::write(&file, code).unwrap();
            crate::fmt::fmt_file(file);
        }
    }
    pub fn write_md(&self, parent_dir: &str) {
        let dir = format!("{}/{}", parent_dir, self.name);

        write_md_mods(&dir, self.children.values().to_vec().as_slice());

        if let Some(code) = &self.code {
            let file = format!("{}/{}.md", parent_dir, self.name);
            fs::write(&file, code).unwrap();
            crate::fmt::fmt_file(file);
        }
    }
}

fn write_rust_mods(dir: &str, children: &[&Module]) {
    if children.is_empty() {
        return;
    }
    let _ = fs::create_dir_all(dir);

    let lib_rs = children
        .iter()
        .map(|it| format!("pub mod {};\n", it.name))
        .collect::<Vec<_>>()
        .join("");

    fs::write(format!("{}/mod.rs", &dir), lib_rs).unwrap();

    for m in children.iter() {
        m.write_rust(dir);
    }
}

fn write_lua_mods(dir: &str, children: &[&Module]) {
    if children.is_empty() {
        return;
    }
    let _ = fs::create_dir_all(dir);

    for m in children.iter() {
        m.write_lua(dir);
    }
}

fn write_md_mods(dir: &str, children: &[&Module]) {
    if children.is_empty() {
        return;
    }
    let _ = fs::create_dir_all(dir);

    for m in children.iter() {
        m.write_md(dir);
    }
}
