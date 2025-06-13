use std::collections::{HashMap, HashSet};

use lite_model::{DataType, EngineClass, Method};
use to_vec::ToVec;
use gen_common::{methods::analyze_method_result, properties::{is_getter, is_setter}, writelnu};
use gen_common::doc::strExt;
use crate::{
    annotations::type_to_lua::type_rust_to_lua,
};

pub fn generate_engine(s: &mut String, class: &EngineClass) {
    writelnu!(s, "");
    s.push_str(class.description.to_luadoc("").as_str());
    writelnu!(s, "---@class {}_static", class.class_name);
    properties(s, class, false);
    writelnu!(s, "{} = {{}}", class.class_name);

    writelnu!(s, "");
    writelnu!(s, "---@class {}", class.class_name);
    properties(s, class, true);
    writelnu!(s, "{}_instance = {{}}", class.class_name);

    methods(s, class, false);
    methods(s, class, true);
}

fn properties(s: &mut String, class: &EngineClass, instance: bool) {
    if !instance {
        for c in class.constants.iter() {
            writelnu!(s, "---@field {} {} ---{}", &c.const_name, type_rust_to_lua(&c.ty), c.description.to_luadoc_inline());
        }
    }
    let mut prop_names: Vec<&str> = Default::default();
    let mut getters: HashMap<&str, &Method> = Default::default();
    let mut setters: HashMap<&str, &Method> = Default::default();
    for method in class.methods.iter() {
        if method.instance != instance {
            continue;
        }
        if is_setter(method) {
            let name = method.method_name.strip_prefix("set_").unwrap();
            if !prop_names.contains(&name) {
                prop_names.push(name);
            }
            setters.insert(name, method);
        }
        if is_getter(method) {
            let name = method.method_name.strip_prefix("get_").unwrap();
            if !prop_names.contains(&name) {
                prop_names.push(name);
            }
            getters.insert(name, method);
        }
    }
    for prop in prop_names {
        let getter = getters
            .get(prop);
        let get_ty = getter
            .map(|it| it.signature.return_ty.as_ref().unwrap());
        let setter = setters.get(prop);
        let set_ty = setter.map(|it| &it.signature.params[0].ty);
        let mut types = HashSet::new();
        get_ty.map(|it| types.insert(it));
        set_ty.map(|it| types.insert(it));
        if types.len() > 1 {
            panic!("conflicting accessors for {}::{}", class.class_name, prop);
        }
        let ty = types.into_iter().next().unwrap();
        let desc = getter.map(|it| &it.description).unwrap_or_else(|| &setter.as_ref().unwrap().description);
        writelnu!(s, "---@field {} {} {}", prop, type_rust_to_lua(ty), desc.to_luadoc_inline());
    }
}

pub fn methods(s: &mut String, class: &EngineClass, instance: bool) {
    for method in class.methods.iter() {
        if method.instance != instance {
            continue;
        }
        if is_getter(method) || is_setter(method) {
            continue;
        }
        *s += "\n";
        s.push_str(method.description.to_luadoc("").as_str());
        let params = method
            .signature
            .params
            .iter()
            .filter(|it| !matches!(it.ty, DataType::UserScriptGenericStub))
            .to_vec();
        let arg_names = params
            .iter()
            .flat_map(|it| match &it.ty {
                DataType::UserScriptMessage => vec![format!("{}_type", &it.name), it.name.to_string()],
                _ => vec![it.name.to_string()],
            })
            .to_vec()
            .join(", ");

        if params.iter().any(|it| matches!(it.ty, DataType::ClassName)) {
            writelnu!(s, "---@generic T");
        }

        if params.iter().any(|it| matches!(it.ty, DataType::UserScriptMessage)) {
            writelnu!(s, "---@generic M");
        }

        for param in params.iter() {
            match &param.ty {
                DataType::UserScriptMessage => {
                    writelnu!(
                        s,
                        "---@param {}_type `M`",
                        &param.name
                    );
                    writelnu!(
                        s,
                        "---@param {} M",
                        &param.name
                    );
                }
                _ => {
                    writelnu!(
                        s,
                        "---@param {} {}",
                        &param.name,
                        &type_rust_to_lua(&param.ty)
                    );
                },
            }
        }

        let method_result = analyze_method_result(method);

        if !matches!(method_result.success_type, DataType::Unit) {
            writelnu!(s, "---@return {}", type_rust_to_lua(&method_result.success_type),);
        }

        writelnu!(
            s,
            "function {}{}:{}({}) end",
            &class.class_name,
            if instance { "_instance" } else { "" },
            &method.method_name,
            &arg_names,
        );
    }
}
