use std::{collections::HashSet, ops::Deref};

use gen_common::templating::render_string;
use lite_model::{ClassName, DataType};

pub(crate) fn generate_ffi_type(
    ty: &DataType,
    client_replicated_types: &HashSet<ClassName>,
) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => "void".to_string(),
        DataType::Bool => "bool".to_string(),
        DataType::Byte => "u8".to_string(),
        DataType::I32 => "i32".to_string(),
        DataType::I64 => "i64".to_string(),
        DataType::F32 => "f32".to_string(),
        DataType::F64 => "f64".to_string(),
        DataType::String => "NativeString".to_string(),
        DataType::ClassName => "NativeString".to_string(),
        DataType::Vec(it) => format!("{}_array", generate_ffi_type(it, client_replicated_types)),
        DataType::UserScript => "NativeHandle".to_string(),
        DataType::UserScriptMessage => "NativeInstanceId".to_string(),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub should not be exposed in bindings")
        }
        DataType::Object(it) => match client_replicated_types.contains(it) {
            false => "NativeHandle".to_string(),
            true => format!("Native{}", it),
        },
        DataType::Option(it) => {
            format!("{}_option", generate_ffi_type(it, client_replicated_types))
        }
        DataType::Result { ok } => {
            format!("{}_result", generate_ffi_type(ok, client_replicated_types))
        }
    }
}

pub(crate) fn generate_to_native(
    ty: &DataType,
    var: &str,
    client_replicated_types: &HashSet<ClassName>,
) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => panic!("`void` should be handled before"),
        DataType::Bool => var.to_string(),
        DataType::Byte => var.to_string(),
        DataType::I32 => var.to_string(),
        DataType::I64 => var.to_string(),
        DataType::F32 => var.to_string(),
        DataType::F64 => var.to_string(),
        DataType::String => format!("<u8 as NativeType>::to_native_array({}.into_bytes())", var),
        DataType::ClassName => format!("<u8 as NativeType>::to_native_array({}.into_bytes())", var),
        DataType::Vec(it) => render_string(
            "<${element} as NativeType>::to_native_array(${var}.into_iter().map(|${var}| ${expr}).collect::<Vec<_>>())",
            [
                ("element", &generate_ffi_type(it.deref(), client_replicated_types)),
                ("var", &var),
                ("expr", &generate_to_native(it.deref(), var, client_replicated_types))
            ],
        ),
        DataType::UserScript => format!("{}.into()", var),
        DataType::UserScriptMessage => var.to_string(),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub should not be exposed in bindings")
        }
        DataType::Object(it) => match client_replicated_types.contains(it) {
            true => format!("{}.into()", var),
            false => format!("NativeHandle::from_u128(Externalizable::to_external(&{}))", var),
        },
        DataType::Option(it) => {
            render_string(
                "<${class} as NativeType>::to_native_option(if let Some(${var}) = ${var} { Some(${expr} ) } else { None })", 
                [
                    ("class", &generate_ffi_type(it.deref(), client_replicated_types)), 
                    ("var", &var),
                    ("expr", &generate_to_native(it.deref(), var, client_replicated_types)),
                ]
            )
        }
        DataType::Result { ok } => {
            render_string(
                "<${class} as NativeType>::to_native_result( match ${var} { Ok(${var}) => Ok(${expr}), Err(err) => Err(err) } )", 
                [
                    ("class", &generate_ffi_type(ok.deref(), client_replicated_types)),
                    ("var", &var),
                    ("expr", &generate_to_native(ok.deref(), var, client_replicated_types)),
                ]
            )
        },
    }
}

pub(crate) fn generate_from_native(
    ty: &DataType,
    var: &str,
    client_replicated_types: &HashSet<ClassName>,
) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => "()".to_string(),
        DataType::Bool => var.to_string(),
        DataType::Byte => var.to_string(),
        DataType::I32 => var.to_string(),
        DataType::I64 => var.to_string(),
        DataType::F32 => var.to_string(),
        DataType::F64 => var.to_string(),
        DataType::String => format!(
            "String::from_utf8(<u8 as NativeType>::from_native_array({})).unwrap()",
            var
        ),
        DataType::ClassName => format!(
            "String::from_utf8(<u8 as NativeType>::from_native_array({})).unwrap()",
            var
        ),
        DataType::Vec(it) => render_string(
            "<${element} as NativeType>::from_native_array(${var}).into_iter().map(|${var}| ${expr}).collect::<Vec<_>>()",
            [
                ("element", &generate_ffi_type(it.deref(), client_replicated_types)),
                ("var", &var),
                ("expr", &generate_from_native(it.deref(), var, client_replicated_types))
            ],
        ),
        DataType::UserScript => var.to_string(),
        DataType::UserScriptMessage => var.to_string(),
        DataType::UserScriptGenericStub => "()".to_string(),
        DataType::Object(it) => match client_replicated_types.contains(it) {
            true => format!("{}.into()", var),
            false => format!("Externalizable::from_external({}.as_u128())", var),
        },
        DataType::Option(it) => render_string(
            "if ${var}.present { Some( { let ${var} = ${var}.value; ${expr} } ) } else { None }",
            [
                ("var", &var),
                (
                    "expr",
                    &generate_from_native(it.deref(), var, client_replicated_types),
                ),
            ],
        ),
        DataType::Result { ok } => render_string(
            "if ${var}.ok { Ok((${expr}).value) } else { Err(${var}.err) }",
            [
                ("var", &var),
                (
                    "expr",
                    &generate_from_native(ok.deref(), var, client_replicated_types),
                ),
            ],
        ),
    }
}
