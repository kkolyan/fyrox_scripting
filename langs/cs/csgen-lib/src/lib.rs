use gen_common::code_model::Module;
use proc_macro2::TokenStream;
use std::collections::HashSet;
use std::{fs, str::FromStr};
use syn::{parse2, File};

pub mod lite_cgen;
pub mod lite_csgen;
pub mod rust_decl_to_cs;

pub fn generate_manual_bindings_cs() -> Module {
    let s = fs::read_to_string("langs/cs/fyrox-lite-cs-lib/src/bindings_manual.rs").unwrap();
    let file = parse2::<File>(TokenStream::from_str(&s).unwrap()).unwrap();

    rust_decl_to_cs::rust_decl_to_c(&file, &HashSet::new())
}
