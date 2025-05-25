use std::{cell::RefCell, collections::HashMap};

use convert_case::Casing;
use fyrox::core::Uuid;
use fyrox_lite::script_metadata::{ScriptField, ScriptFieldValueType, ScriptKind, ScriptMetadata};
use to_vec::ToVec;

use crate::bindings_manual::{NativeBool, NativeClassId, NativeScriptAppFunctions, NativeScriptKind, NativeScriptMetadata, NativeValueType};

// TODO replace with SendWrapper
thread_local! {
    pub static APP: RefCell<Option<ScriptedApp>> = Default::default();
}

pub struct CScriptMetadata {
    pub id: NativeClassId,
    pub md: ScriptMetadata,
    pub has_on_init: bool,
    pub has_on_start: bool,
    pub has_on_deinit: bool,
    pub has_on_update: bool,
    pub has_on_message: bool,
}

pub struct CGlobalScriptMetadata {
    pub id: NativeClassId,
    pub md: ScriptMetadata,
    pub has_on_init: bool,
    pub has_on_update: bool,
}

pub struct ScriptedApp {
    pub is_editor: bool,
    pub scripts_metadata: Option<ScriptsMetadata>,
    pub functions: NativeScriptAppFunctions,
}

pub struct ScriptsMetadata {
    pub node_scripts: HashMap<Uuid, CScriptMetadata>,
    pub global_scripts: HashMap<Uuid, CGlobalScriptMetadata>,
    pub uuid_by_class: HashMap<NativeClassId, Uuid>,
}


impl ScriptedApp {
    pub fn new(init_params: NativeScriptAppFunctions, is_editor: NativeBool) -> Self {
        ScriptedApp {
            is_editor: is_editor.into(),
            scripts_metadata: None,
            functions: init_params,
        }
    }

    pub fn load_scripts_metadata(&mut self) {
        let scripts = (self.functions.get_scripts_metadata)();

        let scripts: Vec<_> = scripts.into();
        let mut global_scripts: HashMap<Uuid, CGlobalScriptMetadata> = Default::default();
        let mut node_scripts: HashMap<Uuid, CScriptMetadata> = Default::default();
        for native_class in scripts {
            let uuid = Uuid::parse_str(String::from(native_class.uuid).as_str()).unwrap();
            let md = extract_for_def(&native_class);
            match native_class.kind {
                NativeScriptKind::Node => {
                    node_scripts.insert(
                        uuid,
                        CScriptMetadata {
                            id: native_class.id,
                            md,
                            has_on_init: true,
                            has_on_start: true,
                            has_on_deinit: true,
                            has_on_update: true,
                            has_on_message: true,
                        },
                    );
                }
                NativeScriptKind::Global => {
                    global_scripts.insert(
                        uuid,
                        CGlobalScriptMetadata {
                            id: native_class.id,
                            md,
                            has_on_init: true,
                            has_on_update: true,
                        },
                    );
                }
            }
        }
        let uuid_by_class = node_scripts.iter()
            .map(|(uuid, md)| (md.id, *uuid))
            .collect();

        self.scripts_metadata = Some(ScriptsMetadata { node_scripts, uuid_by_class, global_scripts });
    }
}

pub fn extract_for_def(md: &NativeScriptMetadata) -> ScriptMetadata {
    let properties: Vec<_> = md.properties.into();
    let class: String = md.name.into();
    let uuid: String = md.uuid.into();
    println!("Register class. name: {}, id: {}, uuid: {}", class, md.id.value, uuid);
    let fields = properties
        .into_iter()
        .map(|property| {
            let name: String = property.name.into();
            let title = name.to_case(convert_case::Case::Title);
            ScriptField {
                name,
                title,
                ty: match property.ty {
                    NativeValueType::bool => ScriptFieldValueType::bool,
                    NativeValueType::f32 => ScriptFieldValueType::f32,
                    NativeValueType::f64 => ScriptFieldValueType::f64,
                    NativeValueType::i16 => ScriptFieldValueType::i16,
                    NativeValueType::i32 => ScriptFieldValueType::i32,
                    NativeValueType::i64 => ScriptFieldValueType::i64,
                    NativeValueType::String => ScriptFieldValueType::String,
                    NativeValueType::Handle => ScriptFieldValueType::Node,
                    NativeValueType::Prefab => ScriptFieldValueType::Prefab,
                    NativeValueType::Vector3 => ScriptFieldValueType::Vector3,
                    NativeValueType::Vector2 => ScriptFieldValueType::Vector2,
                    NativeValueType::Vector2I => ScriptFieldValueType::Vector2I,
                    NativeValueType::Quaternion => ScriptFieldValueType::Quaternion,
                },
                description: None,
                private: false,
            }
        })
        .to_vec();
    let field_name_to_index = fields
        .iter()
        .enumerate()
        .map(|(i, v)| (v.name.clone(), i))
        .collect();
    ScriptMetadata {
        class,
        uuid: Uuid::parse_str(&uuid).unwrap(),
        kind: match md.kind {
            NativeScriptKind::Node => ScriptKind::Node,
            NativeScriptKind::Global => ScriptKind::Global,
        },
        fields,
        field_name_to_index,
    }
}
