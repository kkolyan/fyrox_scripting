use lite_model::{Class, Domain};

use gen_common::{by_package::classes_by_package, code_model::{Module}, context::GenerationContext };
use crate::bindings::{generate_engine_class_bindings::generate_engine_class_bindings, generate_enum_class_bindings::generate_enum_class_bindings, generate_registry::generate_registry, generate_struct_class_bindings::generate_struct_class_bindings};



pub fn generate_lua_bindings(domain: &Domain) -> Module {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };

    let mut bindings = Module::root();

    bindings.add_child(generate_registry(&ctx));

    let by_package = classes_by_package(ctx.domain);

    for (package, class_names) in by_package {
        let mut mods = Module::no_code(package.to_string());
        for class_name in class_names.iter() {
            let class = ctx.domain.get_class(class_name).unwrap();
            match class {
                Class::Engine(it) => {
                    mods.add_child(generate_engine_class_bindings(it, &ctx));
                }
                Class::Struct(it) => {
                    mods.add_child(generate_struct_class_bindings(it, &ctx));
                }
                Class::Enum(it) => {
                    mods.add_child(generate_enum_class_bindings(it, &ctx));
                }
            }
        }
        bindings.add_child(mods);
    }
    bindings
}