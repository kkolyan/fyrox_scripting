use std::collections::HashMap;
use std::fs;
use syn::{Item, Type};

pub fn import() {
    let file = syn::parse_file(
        fs::read_to_string("engine/fyrox-impl/src/scene/collider.rs")
            .unwrap()
            .as_str(),
    )
    .unwrap();

    let mut structs_by_name = HashMap::new();
    let mut impls_by_name = HashMap::new();
    let mut enums_by_name = HashMap::new();

    for item in file.items {
        match item {
            Item::Enum(it) => {
                enums_by_name.insert(it.ident.to_string(), it);
            }
            Item::Impl(it) => {
                if it.trait_.is_some() {
                    continue;
                }
                if let Type::Path(it) = it.self_ty.as_ref() {
                    impls_by_name.insert(it.path.get_ident().unwrap().to_string(), it.clone());
                }
            }
            Item::Struct(it) => {
                structs_by_name.insert(it.ident.to_string(), it);
            }
            _ => {}
        }
    }

    // true enums are true enums
    // ADT enums are abstract classes (sealed?)
    // structs
}
