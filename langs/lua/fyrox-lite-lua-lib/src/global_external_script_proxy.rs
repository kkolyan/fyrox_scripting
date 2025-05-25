use fyrox::core::reflect::{FieldInfo, Reflect};
use fyrox_lite::global_script_object_residence::GlobalScriptResidence;
use fyrox_lite::reflect_base;
use crate::lua_lang::LuaLang;
use crate::user_script_impl::UserScriptProxy;

#[derive(Debug, Clone)]
pub struct ExternalGlobalScriptProxy {
    pub name: String,
    pub data: GlobalScriptResidence<LuaLang>,

}
