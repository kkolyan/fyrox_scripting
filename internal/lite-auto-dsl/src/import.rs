use crate::adt_class::AdtClass;
use crate::auto_engine_class::{AutoEngineClass, SkippedMethod};
use crate::skipped_class::SkippedClass;
use crate::{ImportResult, OriginalName};
use lite_model::{
    ClassName, DataType, EngineClass, EnumClass, EnumValue, EnumVariant, Field, Method, Param,
    RustQualifiedName, Signature, StructClass,
};
use lite_parser::extract_doc;
use lite_parser::extract_ty::{extract_ty, extract_ty_path};
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use syn::__private::ToTokens;
use syn::{Fields, ImplItem, ImplItemFn, Item, Type, Variant, Visibility};

pub fn import(path: impl AsRef<Path>, package: &str) -> ImportResult {
    let file = syn::parse_file(fs::read_to_string(path).unwrap().as_str()).unwrap();

    let mut structs_by_name = HashMap::new();
    let mut impls_by_name = HashMap::new();
    let mut enums_by_name = HashMap::new();

    for item in file.items {
        match item {
            Item::Enum(it) => {
                enums_by_name.insert(it.ident.to_string(), it);
            }
            Item::Impl(item) => {
                if item.trait_.is_some() {
                    continue;
                }
                if let Type::Path(it) = item.self_ty.as_ref() {
                    impls_by_name.insert(it.path.get_ident().unwrap().to_string(), item.clone());
                }
            }
            Item::Struct(it) => {
                structs_by_name.insert(it.ident.to_string(), it);
            }
            _ => {}
        }
    }

    let mut struct_classes: Vec<(OriginalName, StructClass)> = vec![];
    let mut engine_classes: Vec<AutoEngineClass> = vec![];
    let mut enum_classes: Vec<(OriginalName, EnumClass)> = vec![];
    let mut adt_classes: HashMap<ClassName, AdtClass> = HashMap::new();
    let mut skipped_classes: Vec<SkippedClass> = vec![];

    for (original_name, enum_) in enums_by_name.iter() {
        println!("INFO:  scan enum {}", original_name);
        let all_units = enum_
            .variants
            .iter()
            .all(|it| matches!(it.fields, Fields::Unit));

        if all_units {
            enum_classes.push((
                OriginalName::new(original_name),
                EnumClass {
                    class_name: ClassName(format!("Lite{}", original_name)),
                    rust_struct_path: RustQualifiedName(format!(
                        "fyrox_lite::{}::Lite{}",
                        package, original_name
                    )),
                    variants: enum_.variants.iter().map(extract_variant).collect(),
                    features: vec![],
                    description: extract_doc(&enum_.attrs),
                },
            ));
        } else {
            let class_name = ClassName(format!("Lite{}", &original_name));
            adt_classes.insert(
                class_name.clone(),
                AdtClass {
                    original_name: OriginalName::new(original_name),
                    class_name,
                    rust_struct_path: RustQualifiedName(format!(
                        "fyrox_lite::{}::Lite{}",
                        package, original_name
                    )),
                    children: vec![],
                },
            );
        }
    }

    'structs_by_name: for (original_name, struct_) in structs_by_name.iter() {
        println!("INFO:  scan struct def {}", original_name);
        let all_fields_are_named_and_public = struct_
            .fields
            .iter()
            .all(|it| it.ident.is_some() && matches!(it.vis, Visibility::Public(_)));
        let mut fields = vec![];
        for field in struct_.fields.iter() {
            let Some(field_name) = field.ident.as_ref() else {
                skipped_classes.push(SkippedClass {
                    original_name: OriginalName::new(original_name),
                    reason: "unnamed fields are not supported".to_string(),
                });
                continue 'structs_by_name;
            };
            fields.push(Field {
                name: field_name.to_string(),
                ty: match ty_to_lite(&field.ty) {
                    Err(err) => {
                        skipped_classes.push(SkippedClass {
                            original_name: OriginalName::new(original_name),
                            reason: format!(
                                "unsupported field type {}::{}: {:?}",
                                original_name,
                                field.ident.as_ref().unwrap(),
                                err
                            ),
                        });
                        continue 'structs_by_name;
                    }
                    Ok(it) => it,
                },
                description: extract_doc(&field.attrs),
            })
        }
        if all_fields_are_named_and_public {
            let class_name = ClassName(format!("Lite{}", original_name));
            struct_classes.push((
                OriginalName::new(original_name),
                StructClass {
                    parent: adt_classes.get(&class_name).map(|it| it.class_name.clone()),
                    class_name,
                    rust_struct_path: RustQualifiedName(format!(
                        "fyrox_lite::{}::Lite{}",
                        package, original_name
                    )),
                    fields,
                    description: extract_doc(&struct_.attrs),
                },
            ))
        }
    }

    for (original_name, impl_) in impls_by_name.iter() {
        println!("INFO:  scan struct impl {}", original_name);
        let class_name = ClassName(format!("Lite{}", original_name));
        let mut methods = vec![];
        let mut skipped_methods = vec![];
        for it in impl_.items.iter() {
            if let ImplItem::Fn(impl_item) = it {
                let method = extract_fn(impl_item, &original_name);
                match method {
                    Ok(method) => methods.push(method),
                    Err(err) => {
                        skipped_methods.push(SkippedMethod {
                            source: impl_item.clone(),
                            reason: err.to_string(),
                        });
                    }
                }
            }
        }
        engine_classes.push(AutoEngineClass {
            original_name: OriginalName::new(original_name),
            class: EngineClass {
                parent: adt_classes.get(&class_name).map(|it| it.class_name.clone()),
                class_name: class_name.clone(),
                rust_struct_path: RustQualifiedName(format!(
                    "fyrox_lite::{}::Lite{}",
                    package, original_name
                )),
                methods,
                constants: vec![],
                features: vec![],
                description: extract_doc(&impl_.attrs),
            },
            skipped_methods,
        });
    }
    ImportResult {
        struct_classes,
        engine_classes,
        enum_classes,
        adt_classes,
        skipped_classes,
    }
}

fn extract_variant(variant: &Variant) -> EnumVariant {
    assert!(matches!(variant.fields, Fields::Unit));
    EnumVariant {
        tag: variant.ident.to_string(),
        value: EnumValue::Unit,
        description: extract_doc(&variant.attrs),
    }
}

fn ty_to_lite(ty: &Type) -> Result<DataType, String> {
    match ty {
        Type::Path(it) => match extract_ty_path(it.qself.as_ref(), &it.path, it, None) {
            Ok(it) => Ok(it),
            Err(err) => Err(format!("syn error: {}", err)),
        },
        _ => Err(format!(
            "unsupported element: {}: {:?}",
            ty.to_token_stream(),
            ty,
        )),
    }
}

pub fn extract_fn(func: &ImplItemFn, original_owner_name: &dyn Display) -> Result<Method, String> {
    println!(
        "INFO:  scanning function {}::{}",
        original_owner_name, func.sig.ident
    );
    let mut types: Vec<syn::Type> = Default::default();
    let method_name = func.sig.ident.to_token_stream().to_string();
    if !func.sig.generics.params.is_empty() {
        return Err("generic arguments not supported".to_string());
    }
    let mut instance = false;
    let mut params = Vec::new();
    'params: for (i, fn_arg) in func.sig.inputs.iter().enumerate() {
        let arg = match fn_arg {
            syn::FnArg::Receiver(_receiver) => {
                instance = true;
                continue 'params;
            }
            syn::FnArg::Typed(pat_type) => pat_type,
        };
        let ty = arg.ty.as_ref();
        types.push(ty.clone());
        let param_name = match arg.pat.as_ref() {
            syn::Pat::Ident(pat_ident) => pat_ident.ident.to_string(),
            _ => {
                return Err(format!("param name is not a name"));
            }
        };
        params.push(Param {
            name: param_name.clone(),
            ty: match ty_to_lite(ty) {
                Err(err) => {
                    return Err(format!(
                        "invalid param type {}::{}::{}: {:?}",
                        original_owner_name, method_name, param_name, err
                    ));
                }
                Ok(it) => it,
            },
            variadic: false,
        });
    }
    Ok(Method {
        method_name,
        instance,
        signature: Signature {
            params,
            return_ty: match &func.sig.output {
                syn::ReturnType::Default => None,
                syn::ReturnType::Type(_rarrow, ty) => {
                    types.push(ty.as_ref().clone());
                    Some(match extract_ty(ty, None) {
                        Ok(it) => it,
                        Err(err) => {
                            return Err(format!("extract ty error: {}", err));
                        }
                    })
                }
            },
        },
        description: extract_doc(&func.attrs),
    })
}
