use std::collections::HashMap;
use itertools::Itertools;
use gen_common::code_model::Module;
use gen_common::writelnu;
use lite_model::ClassName;

#[derive(Default)]
pub struct Sections {
    pub sections: HashMap<String, Section>,
}

impl Sections {

    pub fn merge(&mut self, m: Sections) {
        for (key, child) in m.sections {
            let prev_child = self.sections.remove(&key);
            let new_child = match prev_child {
                None => child,
                Some(mut prev) => {
                    prev.merge(child);
                    prev
                }
            };
            self.sections.insert(key, new_child);
        }
    }
}

impl Sections {
    pub fn add_child(&mut self, section: Section) {
        self.sections.insert(section.name.clone(), section);
    }

    pub fn to_module(self, class_page_links: &mut HashMap<ClassName, String>) -> Module {
        let mut root = Module::root();
        for section in self.sections.into_values() {
            root.add_child(section.to_module(class_page_links));
        }
        root
    }
}

pub struct Section {
    pub name: String,
    pub description: Option<String>,
    pub classes: HashMap<String, Module>,
    pub enums: HashMap<String, Module>,
    pub structs: HashMap<String, Module>,
}

impl Section {
    pub fn to_module(self, class_page_links: &mut HashMap<ClassName, String>) -> Module {
        let package = self.name.as_str();
        let mut s = "".to_string();
        writelnu!(s, "# {}", package);
        writelnu!(s, "package");

        let mut section = Module::no_code(&self.name);

        if let Some(description) = self.description {
            writelnu!(s, "## Description");
            s.push_str(&description);
            writelnu!(s, "");
        }

        if !self.classes.is_empty() {
            writelnu!(s, "## Classes");
            for x in self.classes.into_values().sorted_by_key(|it| it.name.to_string()) {
                writelnu!(
                    s,
                    "* [{}]({}/{})",
                    x.name,
                    package,
                    class_page_links.get(&ClassName(x.name.clone())).unwrap()
                );
                section.add_child(x);
            }
        }

        if !self.structs.is_empty() {
            writelnu!(s, "## Structs");
            for x in self.structs.into_values().sorted_by_key(|it| it.name.to_string()) {
                writelnu!(
                    s,
                    "* [{}]({}/{})",
                    x.name,
                    package,
                    class_page_links.get(&ClassName(x.name.clone())).unwrap()
                );
                section.add_child(x);
            }
        }

        if !self.enums.is_empty() {
            writelnu!(s, "## Enums");
            for x in self.enums.into_values().sorted_by_key(|it| it.name.to_string()) {
                writelnu!(
                    s,
                    "* [{}]({}/{})",
                    x.name,
                    package,
                    class_page_links.get(&ClassName(x.name.clone())).unwrap()
                );
                section.add_child(x);
            }
        }
        section.set_code(s);
        section
    }
}

impl Section {
    pub fn new(name: String, description: Option<String>) -> Section {
        Self {
            name,
            description,
            classes: Default::default(),
            enums: Default::default(),
            structs: Default::default(),
        }
    }

    pub fn merge(&mut self, m: Section) {
        if self.description.is_none() {
            self.description = m.description;
        }
        Self::merge1(&mut self.classes, m.classes);
        Self::merge1(&mut self.enums, m.enums);
        Self::merge1(&mut self.structs, m.structs);
    }

    fn merge1(this: &mut HashMap<String, Module>, other: HashMap<String, Module>) {
        for (key, child) in other {
            let prev_child = this.remove(&key);
            let new_child = match prev_child {
                None => child,
                Some(mut prev) => {
                    prev.merge(child);
                    prev
                }
            };
            this.insert(key.clone(), new_child);
        }
    }
}
