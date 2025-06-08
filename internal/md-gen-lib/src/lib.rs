use convert_case::{Case, Casing};

pub mod generate_md;
pub mod md;

#[derive(Debug, Copy, Clone)]
pub enum Naming {
    Cs,
    Lua,
}

impl Naming {

    pub fn package_name(&self, name: &str) -> String {
        let n = name.to_case(Case::Pascal).strip_prefix("Lite").unwrap().to_string();
        if n == "Ui" {
            return "UI".to_string();
        }
        n
    }

    pub fn member_name(&self, name: &str) -> String {
        match self {
            Naming::Cs => name.to_case(Case::Pascal),
            Naming::Lua => name.to_string(),
        }
    }
    pub fn param_name(&self, name: &str) -> String {
        match self {
            Naming::Cs => name.to_case(Case::Camel),
            Naming::Lua => name.to_string(),
        }
    }
}