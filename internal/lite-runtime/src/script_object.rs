use std::{fmt::Debug, sync::Arc};

use super::script_metadata::ScriptDefinition;
use crate::global_script_object::ScriptObject;
use fyrox::core::algebra::Vector2;
use fyrox::{
    asset::Resource,
    core::{
        algebra::{UnitQuaternion, Vector3},
        pool::Handle,
        reflect::Reflect,
        visitor::Visit,
        Uuid,
    },
    gui::UiNode,
    resource::model::Model,
    scene::node::Node,
};

/// Useful for persisting script data, but for some languages could be used as a runtime type
#[derive(Clone)]
pub struct NodeScriptObject<T: Lang> {
    pub node: Handle<Node>,
    pub obj: ScriptObject<T>,
}

pub trait Lang: Debug + Clone + 'static {
    type String<'a>;
    type RuntimePin: Clone + Debug + Visit + Default;
    type UnpackedScriptObject: Visit + Debug;
    type UnpackedGlobalScriptObject: Visit + Debug;

    fn drop_runtime_pin(runtime_pin: &mut Self::RuntimePin);
    fn clone_runtime_pin(runtime_pin: &Self::RuntimePin) -> Self::RuntimePin;
    fn drop_script_object_to_prevent_delayed_destructor(_script: &mut Self::UnpackedScriptObject) {}
    fn drop_script_object_to_prevent_delayed_destructor_global(
        _script: &mut Self::UnpackedGlobalScriptObject,
    ) {
    }
    fn id_of(script: &Self::UnpackedScriptObject) -> Uuid;
    fn id_of_global(script: &Self::UnpackedGlobalScriptObject) -> Uuid;
    fn unpack_node_script(
        script: &NodeScriptObject<Self>,
    ) -> Result<Self::UnpackedScriptObject, String>;
    fn unpack_global_script(
        script: &ScriptObject<Self>,
    ) -> Result<Self::UnpackedGlobalScriptObject, String>;
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum ScriptFieldValue<T: Lang> {
    bool(bool),
    f32(f32),
    f64(f64),
    i16(i16),
    i32(i32),
    i64(i64),
    String(String),
    Node(Handle<Node>),
    UiNode(Handle<UiNode>),
    Prefab(Option<Resource<Model>>),
    Vector3(Vector3<f32>),
    Vector2(Vector2<f32>),
    Vector2I(Vector2<i32>),
    Quaternion(UnitQuaternion<f32>),
    // global key of the value. is useful for hot-reload only, because in persistent data it's always None
    RuntimePin(T::RuntimePin),
}

impl<T: Lang> Debug for NodeScriptObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.obj.fmt(f)
    }
}

impl<T: Lang> NodeScriptObject<T> {
    pub fn new(def: &Arc<ScriptDefinition>) -> Self {
        NodeScriptObject {
            node: Default::default(),
            obj: ScriptObject::new(def),
        }
    }
}

impl<T: Lang> Clone for ScriptFieldValue<T> {
    fn clone(&self) -> Self {
        match self {
            ScriptFieldValue::String(it) => Self::String(it.clone()),
            ScriptFieldValue::Node(it) => Self::Node(*it),
            ScriptFieldValue::UiNode(it) => Self::UiNode(*it),
            ScriptFieldValue::Prefab(it) => Self::Prefab(it.clone()),
            ScriptFieldValue::Vector3(it) => Self::Vector3(*it),
            ScriptFieldValue::Vector2(it) => Self::Vector2(*it),
            ScriptFieldValue::Vector2I(it) => Self::Vector2I(*it),
            ScriptFieldValue::Quaternion(it) => Self::Quaternion(*it),
            ScriptFieldValue::RuntimePin(it) => {
                let new = T::clone_runtime_pin(it);
                ScriptFieldValue::RuntimePin(new)
            }
            ScriptFieldValue::bool(it) => ScriptFieldValue::bool(*it),
            ScriptFieldValue::f32(it) => ScriptFieldValue::f32(*it),
            ScriptFieldValue::f64(it) => ScriptFieldValue::f64(*it),
            ScriptFieldValue::i16(it) => ScriptFieldValue::i16(*it),
            ScriptFieldValue::i32(it) => ScriptFieldValue::i32(*it),
            ScriptFieldValue::i64(it) => ScriptFieldValue::i64(*it),
        }
    }
}

impl<T: Lang> ScriptFieldValue<T> {
    pub fn as_reflect_mut(&mut self) -> &mut dyn Reflect {
        match self {
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::UiNode(it) => it,
            ScriptFieldValue::Prefab(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Vector2(it) => it,
            ScriptFieldValue::Vector2I(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
            ScriptFieldValue::RuntimePin(_) => panic!("WTF, it shouldn't be reachable"),
            ScriptFieldValue::bool(it) => it,
            ScriptFieldValue::f32(it) => it,
            ScriptFieldValue::f64(it) => it,
            ScriptFieldValue::i16(it) => it,
            ScriptFieldValue::i32(it) => it,
            ScriptFieldValue::i64(it) => it,
        }
    }
    pub fn as_reflect(&self) -> &dyn Reflect {
        match self {
            ScriptFieldValue::String(it) => it,
            ScriptFieldValue::Node(it) => it,
            ScriptFieldValue::UiNode(it) => it,
            ScriptFieldValue::Prefab(it) => it,
            ScriptFieldValue::Vector3(it) => it,
            ScriptFieldValue::Vector2(it) => it,
            ScriptFieldValue::Vector2I(it) => it,
            ScriptFieldValue::Quaternion(it) => it,
            ScriptFieldValue::RuntimePin(_) => panic!("WTF, it shouldn't be reachable"),
            ScriptFieldValue::bool(it) => it,
            ScriptFieldValue::f32(it) => it,
            ScriptFieldValue::f64(it) => it,
            ScriptFieldValue::i16(it) => it,
            ScriptFieldValue::i32(it) => it,
            ScriptFieldValue::i64(it) => it,
        }
    }
}

// impl<T: Lang> Debug for ScriptFieldValue<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         match self {
//             ScriptFieldValue::String(it) => it.fmt(f),
//             ScriptFieldValue::Node(it) => it.fmt(f),
//             ScriptFieldValue::UiNode(it) => it.fmt(f),
//             ScriptFieldValue::Prefab(it) => it.fmt(f),
//             ScriptFieldValue::Vector3(it) => it.fmt(f),
//             ScriptFieldValue::Quaternion(it) => it.fmt(f),
//             ScriptFieldValue::RuntimePin(it) => it.fmt(f),
//             ScriptFieldValue::bool(it) => it.fmt(f),
//             ScriptFieldValue::f32(it) => it.fmt(f),
//             ScriptFieldValue::f64(it) => it.fmt(f),
//             ScriptFieldValue::i16(it) => it.fmt(f),
//             ScriptFieldValue::i32(it) => it.fmt(f),
//             ScriptFieldValue::i64(it) => it.fmt(f),
//         }
//     }
// }
