use std::{
    fmt::Debug,
    sync::Arc,
};

use fyrox::{
    asset::Resource,
    core::{
        algebra::{UnitQuaternion, Vector3}, pool::Handle, reflect::Reflect, visitor::Visit, Uuid
    },
    gui::UiNode,
    resource::model::Model,
    scene::node::Node,
};
use fyrox::core::algebra::Vector2;
use crate::lite_node::LiteNode;
use crate::script_object::{Lang, ScriptFieldValue};
use super::script_metadata::{ScriptDefinition, ScriptFieldValueType};

/// Useful for persisting script data, but for some languages could be used as a runtime type
#[derive(Clone)]
pub struct ScriptObject<T: Lang> {
    pub def: Arc<ScriptDefinition>,
    pub values: Vec<ScriptFieldValue<T>>,
}

impl<T: Lang> Debug for ScriptObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let class_name = self.def.metadata.class.clone();
        let mut fields = Vec::new();
        for (i, field) in self.def.metadata.fields.iter().enumerate() {
            fields.push(format!(
                "{} ({:?}): {:?}",
                field.name, field.ty, self.values[i]
            ));
        }
        write!(f, "{}{{ {} }}", class_name, fields.join(", "))
    }
}

impl<T: Lang> Drop for ScriptObject<T> {
    fn drop(&mut self) {
        for it in self.values.iter_mut() {
            if let ScriptFieldValue::RuntimePin(it) = it {
                T::drop_runtime_pin(it);
            }
        }
    }
}

impl<T: Lang> ScriptObject<T> {
    pub fn new(def: &Arc<ScriptDefinition>) -> Self {
        ScriptObject {
            def: def.clone(),
            values: def
                .metadata
                .fields
                .iter()
                .map(|it| match &it.ty {
                    ScriptFieldValueType::String => ScriptFieldValue::String(Default::default()),
                    ScriptFieldValueType::Node => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Prefab => ScriptFieldValue::Prefab(Default::default()),
                    ScriptFieldValueType::UiNode => ScriptFieldValue::Node(Default::default()),
                    ScriptFieldValueType::Vector3 => ScriptFieldValue::Vector3(Default::default()),
                    ScriptFieldValueType::Vector2 => ScriptFieldValue::Vector2(Default::default()),
                    ScriptFieldValueType::Vector2I => ScriptFieldValue::Vector2I(Default::default()),
                    ScriptFieldValueType::Quaternion => {
                        ScriptFieldValue::Quaternion(Default::default())
                    }
                    ScriptFieldValueType::RuntimePin => ScriptFieldValue::RuntimePin(T::RuntimePin::default()),
                    ScriptFieldValueType::bool => ScriptFieldValue::bool(Default::default()),
                    ScriptFieldValueType::f32 => ScriptFieldValue::f32(Default::default()),
                    ScriptFieldValueType::f64 => ScriptFieldValue::f64(Default::default()),
                    ScriptFieldValueType::i16 => ScriptFieldValue::i16(Default::default()),
                    ScriptFieldValueType::i32 => ScriptFieldValue::i32(Default::default()),
                    ScriptFieldValueType::i64 => ScriptFieldValue::i64(Default::default()),
                })
                .collect(),
        }
    }
}
