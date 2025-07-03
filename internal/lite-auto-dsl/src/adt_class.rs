use crate::OriginalName;
use lite_model::{ClassName, RustQualifiedName};

pub struct AdtClass {
    pub original_name: OriginalName,
    pub class_name: ClassName,

    pub rust_struct_path: RustQualifiedName,

    pub children: Vec<ClassName>,
}
