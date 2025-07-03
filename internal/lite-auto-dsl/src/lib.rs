mod adt_class;
mod auto_engine_class;
mod import;
mod render;
mod skipped_class;

use crate::adt_class::AdtClass;
use crate::auto_engine_class::AutoEngineClass;
use crate::skipped_class::SkippedClass;
use lite_model::{ClassName, EnumClass, StructClass};
use std::collections::HashMap;
use std::fmt::Display;
use syn::__private::ToTokens;

pub struct OriginalName {
    name: String,
}

impl OriginalName {
    pub fn new(name: &str) -> OriginalName {
        OriginalName {
            name: name.to_string(),
        }
    }
}

pub struct ImportResult {
    struct_classes: Vec<(OriginalName, StructClass)>,
    engine_classes: Vec<AutoEngineClass>,
    enum_classes: Vec<(OriginalName, EnumClass)>,
    adt_classes: HashMap<ClassName, AdtClass>,
    skipped_classes: Vec<SkippedClass>,
}

pub use import::*;
pub use render::*;
