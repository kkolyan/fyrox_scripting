use crate::global_script_object::ScriptObject;
use crate::{
    reflect_base,
    script_metadata::ScriptFieldValueType,
    script_object::{Lang, NodeScriptObject, ScriptFieldValue},
};
use fyrox::core::reflect::*;
use std::process::exit;

impl<T: Lang> Reflect for NodeScriptObject<T> {
    crate::wrapper_reflect!(obj);
}

impl<T: Lang> Reflect for ScriptObject<T> {
    reflect_base!();

    crate::reflect_base_lite!();

    fn fields_ref(&self, func: &mut dyn FnMut(&[FieldRef])) {
        let def = self.def.clone();
        let metadata: Vec<FieldMetadata> = def
            .metadata
            .fields
            .iter()
            .map(|it| FieldMetadata {
                name: it.name.as_str(),
                display_name: it.title.as_str(),
                description: it.name.as_str(),
                tag: "",
                doc: it.description.unwrap_or(""),
                read_only: false,
                immutable_collection: false,
                min_value: None,
                max_value: None,
                step: None,
                precision: None,
            })
            .collect();

        let fields = def
            .metadata
            .fields
            .iter()
            .enumerate()
            .filter(|(_i, it)| it.ty != ScriptFieldValueType::RuntimePin)
            .filter(|(_i, it)| !it.private)
            .map(|(i, _field)| {
                let value_metadata = metadata.get(i);
                let value = self.values.get(i);
                if value_metadata.is_none() || value.is_none() {
                    exit(-789);
                }
                FieldRef {
                    metadata: value_metadata.as_ref().unwrap(),
                    value: match value.unwrap() {
                        ScriptFieldValue::String(it) => it,
                        ScriptFieldValue::Node(it) => it,
                        ScriptFieldValue::UiNode(it) => it,
                        ScriptFieldValue::Prefab(it) => it,
                        ScriptFieldValue::Vector3(it) => it,
                        ScriptFieldValue::Vector2(it) => it,
                        ScriptFieldValue::Vector2I(it) => it,
                        ScriptFieldValue::Quaternion(it) => it,
                        ScriptFieldValue::RuntimePin(_it) => panic!("WTF, it's excluded above"),
                        ScriptFieldValue::bool(it) => it,
                        ScriptFieldValue::f32(it) => it,
                        ScriptFieldValue::f64(it) => it,
                        ScriptFieldValue::i16(it) => it,
                        ScriptFieldValue::i32(it) => it,
                        ScriptFieldValue::i64(it) => it,
                    },
                }
            })
            .collect::<Vec<_>>();
        func(&fields)
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [FieldMut])) {
        let def = self.def.clone();
        let metadata: Vec<FieldMetadata> = def
            .metadata
            .fields
            .iter()
            .map(|it| {
                FieldMetadata {
                    name: it.name.as_str(),
                    display_name: it.title.as_str(),
                    description: it.name.as_str(),
                    tag: "",
                    doc: it.description.unwrap_or(""),
                    read_only: false,
                    immutable_collection: false,
                    min_value: None,
                    max_value: None,
                    step: None,
                    precision: None,
                }
            })
            .collect();

        let mut fields = vec![];
        for (i, _field) in def
            .metadata
            .fields
            .iter()
            .enumerate()
            .filter(|(_i, it)| it.ty != ScriptFieldValueType::RuntimePin)
            .filter(|(_i, it)| !it.private)
        {
            // it's sound, because we never apply it twice for the same index
            let value_raw = unsafe { &mut *self.values.as_mut_ptr().add(i) };
            fields.push(FieldMut {
                metadata: metadata.get(i).as_ref().unwrap(),
                value: match value_raw {
                    ScriptFieldValue::String(it) => it,
                    ScriptFieldValue::Node(it) => it,
                    ScriptFieldValue::UiNode(it) => it,
                    ScriptFieldValue::Prefab(it) => it,
                    ScriptFieldValue::Vector3(it) => it,
                    ScriptFieldValue::Vector2(it) => it,
                    ScriptFieldValue::Vector2I(it) => it,
                    ScriptFieldValue::Quaternion(it) => it,
                    ScriptFieldValue::RuntimePin(_it) => panic!("WTF, it's excluded above"),
                    ScriptFieldValue::bool(it) => it,
                    ScriptFieldValue::f32(it) => it,
                    ScriptFieldValue::f64(it) => it,
                    ScriptFieldValue::i16(it) => it,
                    ScriptFieldValue::i32(it) => it,
                    ScriptFieldValue::i64(it) => it,
                },
            })
        }
        func(&mut fields)
    }

    // fn fields(&self, func: &mut dyn FnMut(&[&dyn Reflect])) {
    //     let fields = self
    //         .values
    //         .iter()
    //         .enumerate()
    //         .filter(|(i, _it)| !self.def.metadata.fields[*i].private)
    //         .map(|(_i, it)| {
    //             let it: &dyn Reflect = it.as_reflect();
    //             it
    //         })
    //         .collect::<Vec<_>>();
    //     func(&fields)
    // }

    // fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [&mut dyn Reflect])) {
    //     let mut fields = self
    //         .values
    //         .iter_mut()
    //         .enumerate()
    //         .filter(|(i, _it)| !self.def.metadata.fields[*i].private)
    //         .map(|(_i, it)| {
    //             let it: &mut dyn Reflect = it.as_reflect_mut();
    //             it
    //         })
    //         .collect::<Vec<_>>();
    //     func(&mut fields)
    // }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        let def = self.def.clone();
        let value = self.values.get(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &dyn Reflect = it.as_reflect();
            x
        }))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        let def = self.def.clone();
        let value = self.values.get_mut(def.metadata.field_name_to_index[name]);
        func(value.map(|it| {
            let x: &mut dyn Reflect = it.as_reflect_mut();
            x
        }))
    }
}
