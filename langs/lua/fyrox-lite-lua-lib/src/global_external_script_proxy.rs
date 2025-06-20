use crate::lua_lang::LuaLang;
use lite_runtime::global_script_object_residence::GlobalScriptResidence;

#[derive(Debug, Clone)]
pub struct ExternalGlobalScriptProxy {
    pub name: String,
    pub data: GlobalScriptResidence<LuaLang>,
}
