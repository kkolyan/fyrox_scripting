use std::{collections::HashMap, sync::Arc};

use lite_runtime::script_metadata::ScriptDefinition;

/// In Fyrox Lite Lua classes and instances have different metatables to emulate conventional OOP more explicitly
#[derive(Debug)]
pub struct ScriptClass {
    pub name: String,
    // static fields (including user-defined methods)
    pub table: HashMap<String, mlua::Value<'static>>,
    pub def: Option<Arc<ScriptDefinition>>,
}
