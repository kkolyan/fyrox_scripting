use crate::lua_lang::LuaLang;
use fyrox_lite::global_script_object_residence::GlobalScriptResidence;

#[derive(Debug, Clone)]
pub struct ExternalGlobalScriptProxy {
    pub name: String,
    pub data: GlobalScriptResidence<LuaLang>,
}
