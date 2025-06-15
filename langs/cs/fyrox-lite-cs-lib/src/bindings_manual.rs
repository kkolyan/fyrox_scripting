use crate::auto_dispose::DisposableHandle;
use crate::scripted_app::{ScriptedApp, APP};
use crate::*;
use fyrox_lite::spi::ClassId;
use fyrox_lite::spi::UserScript;
use std::fmt::Display;
use std::fmt::{Debug, Formatter};

#[no_mangle]
///@owner_class FyroxCApi
pub extern "C" fn init_fyrox_lite(app: NativeScriptAppFunctions, is_editor: NativeBool) {
    APP.set(Some(ScriptedApp::new(app, is_editor)));
}

/// identifier of some entity allocated on scripting side and managed by scripting engine.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeInstanceId {
    pub value: i64,
}

#[repr(C)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct NativeClassId {
    pub value: i64,
}

impl Debug for NativeClassId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.lookup_class_name(), self.value)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptProperty {
    pub id: i32,
    pub name: NativeString,
    pub ty: NativeValueType,
    pub hide_in_inspector: NativeBool,
    pub transient: NativeBool,
}

#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NativeValueType {
    bool,
    f32,
    f64,
    i16,
    i32,
    i64,
    String,
    Handle,
    Prefab,
    Vector3,
    Vector2,
    Vector2I,
    Quaternion,
}

// native_utils!(u8, u8_array, u8_option, u8_result);
// native_utils!(bool, bool_array, bool_option, bool_result);
// native_utils!(f32, f32_array, f32_option, f32_result);
// native_utils!(f64, f64_array, f64_option, f64_result);
// native_utils!(i16, i16_array, i16_option, i16_result);
// native_utils!(i32, i32_array, i32_option, i32_result);
// native_utils!(i64, i64_array, i64_option, i64_result);
// native_utils!(
//     u8_array,
//     NativeString_array,
//     NativeString_option,
//     NativeString_result
// );
// native_utils!(
//     NativeHandle,
//     NativeHandle_array,
//     NativeHandle_option,
//     NativeHandle_result
// );
// native_utils!(
//     NativeInstanceId,
//     NativeInstanceId_array,
//     NativeInstanceId_option,
//     NativeInstanceId_result
// );
// native_utils!(
//     NativeInstanceId_option,
//     NativeInstanceId_option_array,
//     NativeInstanceId_option_option,
//     NativeInstanceId_option_result
// );
// native_utils!(
//     NativeHandle_option,
//     NativeHandle_option_array,
//     NativeHandle_option_option,
//     NativeHandle_option_result
// );

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeString {
    pub data: u8_slice,
}

impl Debug for NativeString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s: String = self.data.into();
        write!(f, "{:?}", s)
    }
}

impl From<NativeString> for String {
    fn from(value: NativeString) -> Self {
        value.data.into()
    }
}

impl From<String> for NativeString {
    fn from(value: String) -> Self {
        Self { data: value.into() }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub union NativeValue {
    pub bool: NativeBool,
    pub f32: f32,
    pub f64: f64,
    pub i16: i16,
    pub i32: i32,
    pub i64: i64,
    pub String: NativeString,
    pub Handle: NativeHandle,
    pub Vector3: NativeVector3,
    pub Vector2: NativeVector2,
    pub Vector2I: NativeVector2I,
    pub Quaternion: NativeQuaternion,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativePropertyValue {
    pub name: NativeString,
    pub ty: NativeValueType,
    pub value: NativeValue,
}

impl Debug for NativePropertyValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ {:?}: {:?} }}", self.name, self.ty)
    }
}

pub trait NativeType: Sized {
    type Slice;
    type Option;
    type Result;

    fn to_native_array(v: Vec<Self>) -> Self::Slice;
    fn from_native_array(v: Self::Slice) -> Vec<Self>;

    fn to_native_option(v: Option<Self>) -> Self::Option;
    fn from_native_option(v: Self::Option) -> Option<Self>;

    fn to_native_result<E: Display>(v: Result<Self, E>) -> Self::Result;
    fn from_native_result<U: UserScript>(v: Self::Result) -> Result<Self, U::LangSpecificError>;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct NativeHandle {
    pub high: u64,
    pub low: u64,
}

impl NativeHandle {
    pub fn from_u128(value: u128) -> Self {
        Self {
            high: (value >> 64) as u64,
            low: value as u64,
        }
    }

    pub fn as_u128(&self) -> u128 {
        (self.high as u128) << 64 | (self.low as u128)
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptMetadata {
    pub id: NativeClassId,
    pub uuid: NativeString,
    pub kind: NativeScriptKind,
    pub name: NativeString,
    pub has_global_on_init: NativeBool,
    pub has_global_on_update: NativeBool,
    pub has_node_on_init: NativeBool,
    pub has_node_on_start: NativeBool,
    pub has_node_on_deinit: NativeBool,
    pub has_node_on_update: NativeBool,
    pub has_node_on_message: NativeBool,
    pub properties: NativeScriptProperty_slice,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum NativeScriptKind {
    Node,
    Global,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct NativeScriptedApp {
    pub functions: NativeScriptAppFunctions,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeScriptAppFunctions {
    pub get_scripts_metadata: GetScriptsMetadata,
    pub on_init: NodeOnInit,
    pub on_start: NodeOnStart,
    pub on_deinit: NodeOnDeinit,
    pub on_update: NodeOnUpdate,
    pub on_message: NodeOnMessage,

    pub on_game_init: GameOnInit,
    pub on_game_update: GameOnUpdate,

    pub create_script_instance: CreateScriptInstance,
    pub dispose_message: DisposeMessage,
    pub dispose_script: DisposeScript,
}

impl DisposableHandle for NativeInstanceId {
    fn dispose_handle(&self, f: &NativeScriptAppFunctions) {
        (f.dispose_script)(*self);
    }
}

impl DisposableHandle for UserScriptMessage {
    fn dispose_handle(&self, f: &NativeScriptAppFunctions) {
        (f.dispose_message)(*self);
    }
}

pub type GetScriptsMetadata = extern "C" fn() -> NativeScriptMetadata_slice;
pub type NodeOnUpdate = extern "C" fn(thiz: NativeInstanceId, dt: f32) -> Unit_result;
pub type NodeOnInit = extern "C" fn(thiz: NativeInstanceId) -> Unit_result;
pub type NodeOnDeinit = extern "C" fn(thiz: NativeInstanceId) -> Unit_result;
pub type NodeOnStart = extern "C" fn(thiz: NativeInstanceId) -> Unit_result;
pub type NodeOnMessage =
    extern "C" fn(thiz: NativeInstanceId, message: UserScriptMessage) -> Unit_result;

pub type GameOnInit = extern "C" fn(
    thiz: NativeInstanceId,
    initial_scene_override: NativeString_optional,
) -> Unit_result;
pub type GameOnUpdate = extern "C" fn(thiz: NativeInstanceId) -> Unit_result;

pub type CreateScriptInstance = extern "C" fn(
    thiz: NativeClassId,
    state: NativePropertyValue_slice,
    node: NativeHandle_optional,
) -> NativeInstanceId_result;
pub type DisposeMessage = extern "C" fn(message: UserScriptMessage);
pub type DisposeScript = extern "C" fn(script: NativeInstanceId);

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct UserScriptMessage {
    pub id: i64,
    pub class_id: NativeClassId,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NativeBool {
    pub value: i32,
}

impl From<bool> for NativeBool {
    fn from(value: bool) -> Self {
        NativeBool {
            value: if value { 1 } else { 0 },
        }
    }
}

impl From<NativeBool> for bool {
    fn from(value: NativeBool) -> Self {
        value.value != 0
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Unit_result {
    pub ok: i32,
    pub err: NativeString,
}

impl Unit_result {
    pub fn into_result(self) -> Result<(), crate::LangSpecificError> {
        if self.ok != 0 {
            Ok(())
        } else {
            Err(self.err.into())
        }
    }
}

impl From<Result<(), crate::LangSpecificError>> for Unit_result {
    fn from(value: Result<(), crate::LangSpecificError>) -> Self {
        match value {
            Ok(_) => Self {
                ok: 1,
                err: unsafe { std::mem::zeroed() },
            },
            Err(err) => Self {
                ok: 0,
                err: err.into(),
            },
        }
    }
}

impl From<Unit_result> for Result<(), crate::LangSpecificError> {
    fn from(value: Unit_result) -> Self {
        value.into_result()
    }
}
