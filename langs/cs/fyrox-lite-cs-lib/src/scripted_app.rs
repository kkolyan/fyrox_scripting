use std::{cell::RefCell, collections::HashMap};

use convert_case::Casing;
use fyrox::core::Uuid;
use lite_runtime::script_metadata::{
    ScriptField, ScriptFieldValueType, ScriptKind, ScriptMetadata,
};
use to_vec::ToVec;

use crate::bindings_manual::{
    NativeBool, NativeClassId, NativeScriptAppFunctions, NativeScriptKind, NativeScriptMetadata,
    NativeValueType,
};

// TODO replace with SendWrapper
thread_local! {
    pub static APP: RefCell<Option<ScriptedApp>> = Default::default();
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct HasCallback: u8 {
        const ON_INIT = 0x01;
        const ON_START = 0x02;
        const ON_DEINIT = 0x03;
        const ON_UPDATE = 0x04;
        const ON_MESSAGE = 0x05;
    }
}

impl HasCallback {
    pub fn from(md: &NativeScriptMetadata) -> HasCallback {
        let mut has_callback = HasCallback::empty();
        if md.has_node_on_init.into() {
            has_callback.insert(HasCallback::ON_INIT);
        }
        if md.has_node_on_start.into() {
            has_callback.insert(HasCallback::ON_START);
        }
        if md.has_node_on_deinit.into() {
            has_callback.insert(HasCallback::ON_DEINIT);
        }
        if md.has_node_on_update.into() {
            has_callback.insert(HasCallback::ON_UPDATE);
        }
        if md.has_node_on_message.into() {
            has_callback.insert(HasCallback::ON_MESSAGE);
        }
        has_callback
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct GlobalHasCallback: u8 {
        const ON_INIT = 0x01;
        const ON_UPDATE = 0x02;
    }
}

impl GlobalHasCallback {
    pub fn from(md: &NativeScriptMetadata) -> GlobalHasCallback {
        let mut has_callback = GlobalHasCallback::empty();
        if md.has_global_on_init.into() {
            has_callback.insert(GlobalHasCallback::ON_INIT);
        }
        if md.has_global_on_update.into() {
            has_callback.insert(GlobalHasCallback::ON_UPDATE);
        }
        has_callback
    }
}

pub struct CScriptMetadata {
    pub id: NativeClassId,
    pub md: ScriptMetadata,
    pub has_callback: HasCallback,
}

pub struct CGlobalScriptMetadata {
    pub id: NativeClassId,
    pub md: ScriptMetadata,
    pub has_callback: GlobalHasCallback,
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
                    let metadata = CScriptMetadata {
                        id: native_class.id,
                        md,
                        has_callback: HasCallback::from(&native_class),
                    };
                    node_scripts.insert(uuid, metadata);
                }
                NativeScriptKind::Global => {
                    global_scripts.insert(
                        uuid,
                        CGlobalScriptMetadata {
                            id: native_class.id,
                            md,
                            has_callback: GlobalHasCallback::from(&native_class),
                        },
                    );
                }
            }
        }
        let uuid_by_class = node_scripts
            .iter()
            .map(|(uuid, md)| (md.id, *uuid))
            .collect();

        self.scripts_metadata = Some(ScriptsMetadata {
            node_scripts,
            uuid_by_class,
            global_scripts,
        });
    }
}

pub fn extract_for_def(md: &NativeScriptMetadata) -> ScriptMetadata {
    let properties: Vec<_> = md.properties.into();
    let class: String = md.name.into();
    println!("C# class: {}", &class);
    let uuid: String = md.uuid.into();
    println!(
        "Register class. name: {}, id: {}, uuid: {}",
        class, md.id.value, uuid
    );
    let fields = properties
        .into_iter()
        .map(|property| {
            let name: String = property.name.into();
            println!("    C# field: {}", &name);
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
