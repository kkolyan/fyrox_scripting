use std::any::TypeId;
use fyrox::asset::Resource;
use fyrox::core::algebra::{UnitQuaternion, Vector2, Vector3};
use fyrox::core::pool::Handle;
use fyrox::core::reflect::{FieldInfo, Reflect};
use fyrox::gui::UiNode;
use fyrox::resource::model::Model;
use fyrox::scene::node::Node;
use fyrox_lite::global_script_object::ScriptObject;
use fyrox_lite::reflect_base;
use fyrox_lite::script_metadata::ScriptFieldValueType;
use fyrox_lite::script_object::{Lang, NodeScriptObject, ScriptFieldValue};
use crate::user_script_impl::UserScriptProxy;

impl Reflect for UserScriptProxy {
    reflect_base!();

    fn fields_info(&self, func: &mut dyn FnMut(&[FieldInfo])) {
        match self {
            UserScriptProxy::Global(it) => it.fields_info(func),
            UserScriptProxy::Node(it) => it.fields_info(func),
        }
    }

    fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
        match self {
            UserScriptProxy::Global(it) => it.fields(func),
            UserScriptProxy::Node(it) => it.fields(func),
        }
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
        match self {
            UserScriptProxy::Global(it) => it.fields_mut(func),
            UserScriptProxy::Node(it) => it.fields_mut(func),
        }
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        match self {
            UserScriptProxy::Global(it) => it.field(name, func),
            UserScriptProxy::Node(it) => it.field(name, func),
        }
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        match self {
            UserScriptProxy::Global(it) => it.field_mut(name, func),
            UserScriptProxy::Node(it) => it.field_mut(name, func),
        }
    }
}