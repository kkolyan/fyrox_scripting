pub(crate) mod arena;
mod auto_dispose;
pub(crate) mod bindings_lite_2;
pub(crate) mod bindings_manual;
pub(crate) mod buffer;
pub(crate) mod c_lang;
pub(crate) mod c_script_metadata;
mod errors;
pub(crate) mod external_script_proxy;
pub mod fyrox_c_plugin;
pub(crate) mod hello;
pub(crate) mod native_utils;
pub(crate) mod scripted_app;
mod string;
pub(crate) mod user_script_impl;
mod utils;

pub(crate) mod external_global_script_proxy;
mod internal_auto;

use crate::bindings_manual::UserScriptMessage;
use crate::c_lang::UnpackedObject;
pub(crate) use arena::Arena;

pub(crate) use bindings_lite_2::*;
pub(crate) use bindings_manual::*;
pub(crate) use internal_auto::*;

pub use fyrox;

pub(crate) type LangSpecificError = String;
pub(crate) type UserScriptImpl = UnpackedObject;
