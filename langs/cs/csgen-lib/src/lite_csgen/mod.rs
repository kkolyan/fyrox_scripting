use crate::lite_csgen::gen_rs::RustEmitter;
use gen_common::by_package::classes_by_package;
use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use lite_model::{Class, ClassName, DataType, Domain};

pub mod api_types;
pub mod engine_class;
pub mod enum_class;
pub mod gen_rs;
pub mod struct_class;
pub mod wrappers;
pub mod write_cs;

pub fn generate_cs_facade(domain: &Domain) -> (Module, RustEmitter) {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain,
    };

    let mut rust = RustEmitter::default();

    let mut bindings = Module::root();

    let by_package = classes_by_package(ctx.domain);

    for (package, class_names) in by_package {
        let mut package_content = Module::no_code(package.as_str());
        for class_name in class_names.iter() {
            let class = ctx.domain.get_class(class_name).unwrap();
            match class {
                Class::Engine(it) => {
                    package_content.add_child(engine_class::generate_bindings(it, &ctx, &mut rust));
                }
                Class::Struct(it) => {
                    package_content.add_child(struct_class::generate_bindings(it, &ctx, &mut rust));
                }
                Class::Enum(it) => {
                    package_content.add_child(enum_class::generate_bindings(it, &ctx, &mut rust));
                }
            }
        }
        bindings.add_child(package_content);
    }
    (bindings, rust)
}

pub fn generate_base() -> (Module, RustEmitter) {
    let ctx = GenerationContext {
        internal_to_external: Default::default(),
        domain: &Default::default(),
    };
    let ctx = &ctx;

    let mut rust_owned = RustEmitter::default();
    let rust = &mut rust_owned;

    let mut s = String::new();
    let basic_types = [
        DataType::Bool,
        DataType::Byte,
        DataType::I32,
        DataType::I64,
        DataType::F32,
        DataType::F64,
        DataType::String,
        DataType::UserScript,
    ];
    for basic_type in basic_types {
        wrappers::generate_result(&mut s, rust, &basic_type, ctx);
        wrappers::generate_optional(&mut s, rust, &basic_type, ctx);
        wrappers::generate_slice(&mut s, rust, &basic_type, ctx);
    }
    wrappers::generate_slice(
        &mut s,
        rust,
        &DataType::Object(ClassName("NativeScriptMetadata".to_string())),
        ctx,
    );
    wrappers::generate_slice(
        &mut s,
        rust,
        &DataType::Object(ClassName("NativeScriptProperty".to_string())),
        ctx,
    );
    wrappers::generate_slice(
        &mut s,
        rust,
        &DataType::Object(ClassName("NativeValue".to_string())),
        ctx,
    );
    wrappers::generate_optional(
        &mut s,
        rust,
        &DataType::Object(ClassName("NativeHandle".to_string())),
        ctx,
    );
    wrappers::generate_slice(
        &mut s,
        rust,
        &DataType::Object(ClassName("NativePropertyValue".to_string())),
        ctx,
    );
    wrappers::generate_result(
        &mut s,
        rust,
        &DataType::Option(Box::new(DataType::UserScript)),
        ctx,
    );
    let mut root = Module::root();
    root.add_child(Module::code("LiteBase", s));
    (root, rust_owned)
}
