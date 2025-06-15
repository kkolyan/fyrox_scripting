use std::collections::HashMap;

use fyrox::core::Uuid;

#[derive(Debug)]
pub struct ScriptDefinition {
    pub metadata: ScriptMetadata,
    pub assembly_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct ScriptMetadata {
    pub class: String,
    pub uuid: Uuid,
    pub kind: ScriptKind,
    pub fields: Vec<ScriptField>,
    pub field_name_to_index: HashMap<String, usize>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    Node,
    Global,
}

#[derive(Debug, Clone)]
pub struct ScriptField {
    pub name: String,
    pub title: String,
    pub ty: ScriptFieldValueType,
    pub description: Option<&'static str>,
    pub private: bool,
}

#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ScriptFieldValueType {
    bool,
    f32,
    f64,
    i16,
    i32,
    i64,
    String,
    Node,
    UiNode,
    Prefab,
    Vector3,
    Vector2,
    Vector2I,
    Quaternion,
    // not available in editor, but supported to allow script type annotations
    RuntimePin,
}
