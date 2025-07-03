use crate::OriginalName;
use lite_model::EngineClass;

pub struct AutoEngineClass {
    pub original_name: OriginalName,
    pub class: EngineClass,
    pub skipped_methods: Vec<SkippedMethod>,
}

pub struct SkippedMethod {
    pub source: syn::ImplItemFn,
    pub reason: String,
}
