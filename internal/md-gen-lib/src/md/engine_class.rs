use crate::{md::type_to_md::type_rust_to_md, Naming};
use gen_common::by_package::extract_package;
use gen_common::properties::is_regular;
use gen_common::{methods::analyze_method_result, properties::{is_getter, is_setter}, writelnu};
use itertools::Itertools;
use lite_model::{
    ClassName, Constant, ConstantValue, DataType, EngineClass, Literal, Method, Signature,
};
use std::borrow::Cow;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use to_vec::ToVec;

struct Property<'a> {
    name: &'a str,
    description: String,
    ty: &'a DataType,
    read: bool,
    write: bool,
}

pub fn generate_engine(
    s: &mut String,
    class: &EngineClass,
    naming: Naming,
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "# {}", class.class_name);
    writelnu!(
        s,
        "class in [FyroxLite](../README.md).[{}](README.md)",
        naming.package_name(extract_package(&class.rust_struct_path))
    );
    if !class.description.is_empty() {
        writelnu!(s, "## Description");
        writelnu!(s, "{}", class.description);
    }

    let constants = class.constants.as_slice();
    if !constants.is_empty() {
        writelnu!(s, "## Constants");
        render_constants(s, constants, naming, class_page_links);
    }

    let properties = extract_properties(class, true);
    if !properties.is_empty() {
        writelnu!(s, "## Properties");
        render_properties(s, properties.as_slice(), naming, class_page_links);
    }

    let methods = extract_regular_methods(class, true);
    if !methods.is_empty() {
        writelnu!(s, "## Methods");
        render_methods(s, methods.as_slice(), naming, class_page_links);
    }

    let mut static_props = extract_properties(class, false);
    if !static_props.is_empty() {
        writelnu!(s, "## Static Properties");
        render_properties(s, static_props.as_slice(), naming, class_page_links);
    }

    let static_methods = extract_regular_methods(class, false);
    if !static_methods.is_empty() {
        writelnu!(s, "## Static Methods");
        render_methods(s, static_methods.as_slice(), naming, class_page_links);
    }
}

fn render_methods(
    s: &mut String,
    methods: &[&Method],
    naming: Naming,
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Return Type | Signature | Description |");
    writelnu!(s, "|---|---|---|");
    for method in methods {
        let return_ty = analyze_method_result(method);
        let generics = match method
            .signature
            .params
            .iter()
            .any(|it| matches!(it.ty, DataType::ClassName))
        {
            true => " <`T`>",
            false => "",
        };
        let params = method
            .signature
            .params
            .iter()
            .filter(|it| !matches!(it.ty, DataType::UserScriptGenericStub))
            .filter(|it| !matches!(it.ty, DataType::ClassName))
            .to_vec();
        writelnu!(
            s,
            "| {} | `{}`{} ( {} ) | {} |",
            type_rust_to_md(&return_ty.success_type, class_page_links),
            naming.member_name(method.method_name.as_str()),
            generics,
            params
                .iter()
                .map(|it| format!(
                    "{} <ins>{}</ins>",
                    type_rust_to_md(&it.ty, class_page_links),
                    naming.param_name(&it.name)
                ))
                .join(", "),
            &method.description
        );
    }
}

fn render_properties(
    s: &mut String,
    props: &[Property],
    naming: Naming,
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Name | Type | Access | Description |");
    writelnu!(s, "|---|---|---|---|");
    for prop in props {
        let access = match (prop.read, prop.write) {
            (true, false) => "get",
            (false, true) => "set",
            (true, true) => "get / set",
            _ => unreachable!(),
        };
        writelnu!(
            s,
            "| `{}` | {} | {} | {} |",
            naming.member_name(prop.name),
            type_rust_to_md(prop.ty, class_page_links),
            access,
            prop.description
        );
    }
}

fn render_constants(
    s: &mut String,
    constants: &[Constant],
    naming: Naming,
    class_page_links: &HashMap<ClassName, String>,
) {
    writelnu!(s, "| Name | Type | Value | Description |");
    writelnu!(s, "|---|---|---|---|");
    for constant in constants {
        writelnu!(
            s,
            "| `{}` | {} | {} | {} |",
            naming.member_name(&constant.const_name),
            type_rust_to_md(&constant.ty, class_page_links),
            constant_to_display(&constant.value),
            &constant.description
        );
    }
}

fn constant_to_display(value: &ConstantValue) -> &dyn Display {
    match value {
        ConstantValue::Literal(it) => match it {
            Literal::Bool(it) => it,
            Literal::Byte(it) => it,
            Literal::Number(it) => it,
            Literal::String(it) => it,
        },
        ConstantValue::ComplexExpression(it) => it,
    }
}

fn extract_regular_methods(class: &EngineClass, instance: bool) -> Vec<&Method> {
    class
        .methods
        .iter()
        .filter(|it| it.instance == instance)
        .filter(|it| is_regular(it))
        .collect()
}

fn extract_properties(class: &EngineClass, instance: bool) -> Vec<Property> {
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
    prop_names
        .iter()
        .map(|prop| {
            let get = getters.get(prop);
            let get_ty = get.map(|it| it.signature.return_ty.as_ref().unwrap());
            let set = setters.get(prop);
            let set_ty = set.map(|it| &it.signature.params[0].ty);
            let mut types = HashSet::new();
            get_ty.map(|it| types.insert(it));
            set_ty.map(|it| types.insert(it));
            if types.len() > 1 {
                panic!("conflicting accessors for {}::{}", class.class_name, prop);
            }
            let ty = types.into_iter().next().unwrap();
            Property {
                name: prop,
                description: format!(
                    "{}{}",
                    get.map(|it| it.description.as_str()).unwrap_or(""),
                    get.map(|it| it.description.as_str()).unwrap_or("")
                ),
                ty,
                read: get_ty.is_some(),
                write: set_ty.is_some(),
            }
        })
        .collect()
}
