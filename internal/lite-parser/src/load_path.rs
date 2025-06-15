use std::{collections::HashMap, fs, path::Path, str::FromStr};

use crate::doc_attr::extract_doc;
use crate::{
    extract_engine_class::extract_engine_class_and_inject_assertions,
    extract_pod_enum::extract_pod_enum, extract_pod_struct::extract_pod_struct,
    lite_api_attr::LiteApiAttr, RustSymbol,
};
use lite_model::{Class, Domain, Package};
use proc_macro2::{Span, TokenStream};
use syn::{parse2, spanned::Spanned};

pub fn load_path(
    crate_name: &str,
    path: &Path,
    domain: &mut Domain,
    aliases: &mut HashMap<String, RustSymbol>,
) {
    let text = fs::read_to_string(path).unwrap();
    let file = parse2::<syn::File>(TokenStream::from_str(&text).unwrap()).unwrap();

    let mod_name = path
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace(".rs", "");
    let root_mod_name = crate_name.replace("-", "_");
    let mod_name = if mod_name == "lib" {
        root_mod_name
    } else {
        format!("{}::{}", root_mod_name, mod_name)
    };
    println!("mod name: {}", mod_name);

    let mut definitions_found = 0;

    for item in file.items {
        match item {
            syn::Item::Impl(mut it) => {
                if let Some(attr) = extract_attr(&it.attrs, "lite_api") {
                    if let Some((rust_name, class)) = extract_engine_class_and_inject_assertions(
                        &mod_name,
                        attr,
                        &mut it,
                        &mut vec![],
                    ) {
                        aliases.insert(
                            class.class_name.0.clone(),
                            RustSymbol(rust_name.to_string().clone()),
                        );
                        domain.classes.push(Class::Engine(class));
                        definitions_found += 1;
                    }
                }
            }
            syn::Item::Struct(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "lite_api") {
                    if let Some((rust_name, class)) =
                        extract_pod_struct(&mod_name, attr, &it, &mut vec![])
                    {
                        aliases.insert(
                            class.class_name.0.clone(),
                            RustSymbol(rust_name.to_string().clone()),
                        );
                        domain.classes.push(Class::Struct(class));
                        definitions_found += 1;
                    }
                }
            }
            syn::Item::Enum(it) => {
                if let Some(attr) = extract_attr(&it.attrs, "lite_api") {
                    if let Some((rust_name, class)) =
                        extract_pod_enum(&mod_name, attr, &it, &mut vec![], &mut vec![])
                    {
                        aliases.insert(
                            class.class_name.0.clone(),
                            RustSymbol(rust_name.to_string().clone()),
                        );
                        domain.classes.push(Class::Enum(class));
                        definitions_found += 1;
                    }
                }
            }
            _ => {}
        }
    }
    if definitions_found > 0 {
        domain.packages.push(Package {
            name: mod_name,
            description: extract_doc(&file.attrs),
        })
    }
}

fn extract_attr(attrs: &[syn::Attribute], attr_name: &str) -> Option<(LiteApiAttr, Span)> {
    let attr = attrs.iter().find(|it| {
        it.path()
            .get_ident()
            .map(|it| it == attr_name)
            .unwrap_or_default()
    });
    let attr = attr?;
    Some((
        match &attr.meta {
            syn::Meta::Path(_it) => Default::default(),
            syn::Meta::List(it) => LiteApiAttr::from_attr_args(it.tokens.clone()).unwrap(),
            syn::Meta::NameValue(_it) => {
                panic!(
                    "usage: #[lite_api] or #[lite_api({})]",
                    LiteApiAttr::OPTIONS_HINT
                );
            }
        },
        attr.span(),
    ))
}
