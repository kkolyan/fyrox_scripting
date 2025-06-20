pub(crate) mod debug;
pub(crate) mod external_script_proxy;
pub(crate) mod fmt_pretty;
pub(crate) mod fyrox_lua_plugin;
pub(crate) mod generated;
mod global_external_script_proxy;
pub(crate) mod lua_lang;
pub(crate) mod lua_lifecycle;
pub(crate) mod lua_script_metadata;
pub(crate) mod lua_utils;
pub(crate) mod manual_lua_bindings;
pub(crate) mod script_class;
pub(crate) mod script_object_residence;
pub(crate) mod typed_userdata;
pub(crate) mod user_data_plus;
pub(crate) mod user_script_impl;
mod user_script_impl_fyrox_traits;

pub use fyrox_lua_plugin::LuaPlugin;

pub(crate) mod script_object {
    use crate::lua_lang::LuaLang;

    pub type NodeScriptObject = lite_runtime::script_object::NodeScriptObject<LuaLang>;
    pub type ScriptObject = lite_runtime::global_script_object::ScriptObject<LuaLang>;
    pub type ScriptFieldValue = lite_runtime::script_object::ScriptFieldValue<LuaLang>;
}
