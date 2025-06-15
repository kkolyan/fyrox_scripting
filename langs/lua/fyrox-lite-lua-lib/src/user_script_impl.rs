use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::mem;

use crate::lua_lang::LuaLang;
use crate::{
    external_script_proxy::ExternalScriptProxy, fyrox_lua_plugin::LuaPlugin, lua_error,
    lua_lang::UnpackedScriptObjectVisit, lua_lifecycle::lua_vm, script_class::ScriptClass,
    typed_userdata::TypedUserData, user_data_plus::Traitor,
};
use fyrox::core::pool::Handle;
use fyrox::scene::node::Node;
use fyrox::script::{DynamicTypeId, ScriptMessagePayload};
use fyrox_lite::global_script_object::ScriptObject;
use fyrox_lite::script_object::NodeScriptObject;
use fyrox_lite::spi::ClassId;
use fyrox_lite::{script_context::with_script_context, spi::UserScript, LiteDataType};
use mlua::{UserDataRef, Value};
use mlua::prelude::LuaResult;
use send_wrapper::SendWrapper;

#[derive(Clone, Debug)]
pub enum UserScriptProxy {
    Global(ScriptObject<LuaLang>),
    Node(NodeScriptObject<LuaLang>),
}

impl UserScriptProxy {
    pub fn as_script_object(&self) -> &ScriptObject<LuaLang> {
        match self {
            UserScriptProxy::Global(it) => it,
            UserScriptProxy::Node(it) => &it.obj,
        }
    }
    pub fn as_script_object_mut(&mut self) -> &mut ScriptObject<LuaLang> {
        match self {
            UserScriptProxy::Global(it) => it,
            UserScriptProxy::Node(it) => &mut it.obj,
        }
    }

    pub fn as_global(&self) -> Option<&ScriptObject<LuaLang>> {
        match self {
            UserScriptProxy::Global(it) => Some(it),
            UserScriptProxy::Node(_) => None,
        }
    }
    pub fn to_global(self) -> Option<ScriptObject<LuaLang>> {
        match self {
            UserScriptProxy::Global(it) => Some(it),
            UserScriptProxy::Node(_) => None,
        }
    }
    pub fn as_node(&self) -> Option<&NodeScriptObject<LuaLang>> {
        match self {
            UserScriptProxy::Global(it) => None,
            UserScriptProxy::Node(it) => Some(it),
        }
    }
    pub fn to_node(self) -> Option<NodeScriptObject<LuaLang>> {
        match self {
            UserScriptProxy::Global(it) => None,
            UserScriptProxy::Node(it) => Some(it),
        }
    }
}

type ClassIdImpl = String;
type UserScriptMessageImpl = LuaUserScriptMessageEnvelope;

impl<'a> UserScript for TypedUserData<'a, UserScriptProxy> {
    type Plugin = LuaPlugin;

    type ProxyScript = ExternalScriptProxy;

    type ClassId = ClassIdImpl;

    type LangSpecificError = mlua::Error;

    type UserScriptMessage = UserScriptMessageImpl;

    type UserScriptGenericStub = ();

    fn extract_from(
        node: Handle<Node>,
        proxy: &mut Self::ProxyScript,
        class: &Self::ClassId,
        plugin: &mut Self::Plugin,
    ) -> Option<Self> {
        if &proxy.name == class {
            proxy.data.ensure_unpacked(&mut plugin.failed, node);
            let script_data = &mut proxy.data.inner_unpacked();
            return Some(TypedUserData::clone(
                &script_data.expect("expected to be unpacked here").0,
            ));
        }
        None
    }

    fn into_proxy_script(self, _class: &Self::ClassId) -> mlua::Result<Self::ProxyScript> {
        let name = self.borrow()?.as_script_object().def.metadata.class.to_string();
        // it's sound, because Lua outlives a process
        let ud: TypedUserData<'static, UserScriptProxy> = unsafe { mem::transmute(self) };
        let data = crate::script_object_residence::ScriptResidence::Unpacked(
            UnpackedScriptObjectVisit(SendWrapper::new(ud)),
        );
        Ok(ExternalScriptProxy { name, data })
    }

    fn find_global_script(class: &Self::ClassId) -> Result<Self, Self::LangSpecificError> {
        with_script_context(|ctx| {
            let Some(plugins) = ctx.plugins.as_mut() else {
                return Err(lua_error!("plugins not available here"));
            };
            let plugin = plugins
                .of_type_mut::<Self::Plugin>()
                .expect("WTF: Lua Plugin not found!");
            for script in plugin.scripts.borrow_mut().inner_mut().iter_mut() {
                if &script.name == class {
                    return Ok(TypedUserData::clone(
                        &script.data.inner_unpacked().unwrap().0,
                    ));
                }
            }
            Err(lua_error!("plugin script not found: {}", class))
        })
    }

    fn create_error(msg: &str) -> Self::LangSpecificError {
        mlua::Error::runtime(msg)
    }

    fn new_instance(
        node: Handle<Node>,
        class_id: &Self::ClassId,
    ) -> Result<Self, Self::LangSpecificError> {
        let class = lua_vm()
            .globals()
            .get::<_, Option<UserDataRef<ScriptClass>>>(class_id.clone())?;
        let Some(class) = class else {
            return Err(lua_error!(
                "class not found: {}",
                class_id.lookup_class_name()
            ));
        };
        let Some(def) = &class.def else {
            return Err(lua_error!(
                "invalid class: {}",
                class_id.lookup_class_name()
            ));
        };
        let mut script_object = NodeScriptObject::new(def);
        script_object.node = node;
        let obj = lua_vm().create_userdata(UserScriptProxy::Node(script_object))?;
        Ok(TypedUserData::<UserScriptProxy>::new(obj))
    }

    fn pack_class_id(class_id: &Self::ClassId) -> DynamicTypeId {
        CLASS_MAPPINGS.with_borrow_mut(|x| {
            let v = x.packed.get(class_id);
            let Some(v) = v else {
                let new_type_id = (x.packed.len() + 1) as DynamicTypeId;
                x.packed.insert(class_id.clone(), new_type_id);
                x.unpacked.insert(new_type_id, class_id.clone());
                return new_type_id
            };
            *v
        })
    }

    fn unpack_class_id(class_id: DynamicTypeId) -> Self::ClassId {
        CLASS_MAPPINGS.with_borrow(|x| {
            x.unpacked.get(&class_id).unwrap().clone()
        })
    }
}

thread_local! {
    static CLASS_MAPPINGS: RefCell<ClassMappings> = Default::default();
}

#[derive(Default)]
struct ClassMappings {
    unpacked: HashMap<DynamicTypeId, ClassIdImpl>,
    packed: HashMap<ClassIdImpl, DynamicTypeId>,
}

impl LiteDataType for UserScriptMessageImpl {}

impl<'a> LiteDataType for TypedUserData<'a, UserScriptProxy> {}

#[derive(Debug, Clone)]
pub struct LuaUserScriptMessageEnvelope {
    pub class: String,
    pub message: Traitor<SendWrapper<Value<'static>>>,
}

impl LuaUserScriptMessageEnvelope {
    pub fn new(ty: Value, value: Value) -> LuaResult<Self> {
        // Traitor::new(send_wrapper::SendWrapper::new(unsafe {{ std::mem::transmute::<mlua::Value<'_>, mlua::Value<'static>>({}) }} ))
        Ok(Self {
            class: ty.to_string().map_err(|err| lua_error!("Failed to get class name from argument. error: {}", err))?,
            // we use Lua interpreter as long as we use the process, so its lifetime is effectively static.
            message: Traitor::new(SendWrapper::new(unsafe { mem::transmute::<Value<'_>, Value<'static>>(value) } )),
        })
    }
}

impl ScriptMessagePayload for LuaUserScriptMessageEnvelope {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn get_dynamic_type_id(&self) -> Option<DynamicTypeId> {
        Some(TypedUserData::pack_class_id(&self.class))
    }
}
