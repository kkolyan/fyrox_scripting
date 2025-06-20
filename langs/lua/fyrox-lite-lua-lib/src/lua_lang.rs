use std::fmt::{Debug, Formatter};
use std::ops::DerefMut;

use crate::script_object::ScriptObject;
use crate::user_script_impl::UserScriptProxy;
use crate::{
    lua_lifecycle::lua_vm, script_object::NodeScriptObject, typed_userdata::TypedUserData,
};
use fyrox::core::{visitor::Visit, Uuid};
use lite_runtime::{script_object::Lang, script_object_residence::uuid_of_script};
use mlua::{Table, Value};
use send_wrapper::SendWrapper;

#[derive(Debug, Clone)]
pub struct LuaLang;

impl Lang for LuaLang {
    type String<'a> = mlua::String<'a>;
    type RuntimePin = Option<String>;
    type UnpackedScriptObject = UnpackedScriptObjectVisit;
    type UnpackedGlobalScriptObject = UnpackedScriptObjectVisit;

    fn drop_runtime_pin(runtime_pin: &mut Self::RuntimePin) {
        let Some(runtime_pin) = runtime_pin else {
            return;
        };
        lua_vm()
            .globals()
            .get::<_, Table>("PINS")
            .unwrap()
            .set(runtime_pin.as_str(), Value::Nil)
            .unwrap();
    }

    fn clone_runtime_pin(runtime_pin: &Self::RuntimePin) -> Self::RuntimePin {
        let Some(runtime_pin) = runtime_pin else {
            return None;
        };
        let new = Uuid::new_v4().to_string();
        let ex_value = lua_vm()
            .globals()
            .get::<_, Table>("PINS")
            .unwrap()
            .get::<_, mlua::Value>(runtime_pin.as_str())
            .unwrap();
        lua_vm()
            .globals()
            .get::<_, Table>("PINS")
            .unwrap()
            .set(new.as_str(), ex_value)
            .unwrap();
        Some(new)
    }

    fn drop_script_object_to_prevent_delayed_destructor(script: &mut Self::UnpackedScriptObject) {
        // take ScriptObject out of Lua VM and destroy it right now to prevent nested destructors
        // to be invoked at random moment in future by Lua GC, anth thus ruin Hit Reload
        if let Ok(it) = TypedUserData::take(script.0.deref_mut()) {
            let _s: NodeScriptObject = it.into_node().unwrap();
        }
    }
    fn drop_script_object_to_prevent_delayed_destructor_global(
        script: &mut Self::UnpackedGlobalScriptObject,
    ) {
        // take ScriptObject out of Lua VM and destroy it right now to prevent nested destructors
        // to be invoked at random moment in future by Lua GC, anth thus ruin Hit Reload
        if let Ok(it) = TypedUserData::take(script.0.deref_mut()) {
            let _s: ScriptObject = it.into_global().unwrap();
        }
    }

    fn id_of(script: &Self::UnpackedScriptObject) -> Uuid {
        uuid_of_script(&script.0.borrow().unwrap().as_node().unwrap().obj)
    }

    fn id_of_global(script: &Self::UnpackedGlobalScriptObject) -> Uuid {
        uuid_of_script(script.0.borrow().unwrap().as_global().unwrap())
    }

    fn unpack_node_script(
        script: &lite_runtime::script_object::NodeScriptObject<Self>,
    ) -> Result<Self::UnpackedScriptObject, String> {
        let so = lua_vm()
            .create_userdata(UserScriptProxy::Node(script.clone()))
            .map(TypedUserData::<UserScriptProxy>::new)
            .map(SendWrapper::new)
            .map(UnpackedScriptObjectVisit);
        so.map_err(|it| it.to_string())
    }

    fn unpack_global_script(
        script: &lite_runtime::global_script_object::ScriptObject<Self>,
    ) -> Result<Self::UnpackedGlobalScriptObject, String> {
        let so = lua_vm()
            .create_userdata(UserScriptProxy::Global(script.clone()))
            .map(TypedUserData::<UserScriptProxy>::new)
            .map(SendWrapper::new)
            .map(UnpackedScriptObjectVisit);
        so.map_err(|it| it.to_string())
    }
}

pub struct UnpackedScriptObjectVisit(pub SendWrapper<TypedUserData<'static, UserScriptProxy>>);

impl Debug for UnpackedScriptObjectVisit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Visit for UnpackedScriptObjectVisit {
    fn visit(
        &mut self,
        name: &str,
        visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        let mut ref_mut = self.0.borrow_mut().unwrap();
        match ref_mut.deref_mut() {
            UserScriptProxy::Global(it) => it.visit(name, visitor),
            UserScriptProxy::Node(it) => it.visit(name, visitor),
        }
    }
}
