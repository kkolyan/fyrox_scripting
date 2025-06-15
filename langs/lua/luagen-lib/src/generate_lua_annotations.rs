use lite_model::{Class, Domain};

use crate::annotations::{
    engine_class::generate_engine, enum_class::generate_enum, struct_class::generate_struct,
};
use gen_common::{
    by_package::classes_by_package, code_model::Module, templating::strExt, writelnu,
};

const HEADER: &str = "
---@meta
-- Code below is not intended to be executed. It contains annotations for VSCode and other compatible IDEs.
-- More about Lua annotations format:
-- https://luals.github.io/wiki/annotations
-- https://github.com/LuaLS/lua-language-server/wiki/Annotations
--
-- This file is auto-generated, do not edit it manually.

---@diagnostic disable: missing-return, lowercase-global, missing-fields
";

pub fn generate_lua_annotations(domain: &Domain) -> Module {
    let manual = generate_lua_annotations_manual();
    let mut from_domain = generate_lua_annotations_from_domain(domain);
    from_domain.merge(manual);
    from_domain
}

fn generate_lua_annotations_manual() -> Module {
    let mut root = Module::root();
    let mut script = Module::no_code("Script");
    script.add_child(Module::code(
        "NodeScript",
        format!(
            "{}

			---@class NodeScript
			---@field node Node
			NodeScript = {{}}

			function script_class() end
		",
            HEADER.trim()
        )
        .as_str()
        .deindent(),
    ));
    root.add_child(script);
    root
}

fn generate_lua_annotations_from_domain(domain: &Domain) -> Module {
    let mut mods = Module::root();

    let by_package = classes_by_package(domain);
    for (package, classes) in by_package {
        let mut package_mods = Module::no_code(package.strip_prefix("lite_").unwrap());

        for class in classes {
            let class = domain.get_class(&class).unwrap();
            let mut s = String::new();
            writelnu!(s, "{}", HEADER.trim());
            writelnu!(s, "");
            writelnu!(
                s,
                "-----------------------------------------------------------"
            );
            writelnu!(s, "------ {}", class.rust_name());
            writelnu!(
                s,
                "-----------------------------------------------------------"
            );
            match class {
                Class::Engine(class) => generate_engine(&mut s, class),
                Class::Struct(class) => generate_struct(&mut s, class),
                Class::Enum(class) => generate_enum(&mut s, class),
            }
            writelnu!(s, "");

            package_mods.add_child(Module::code(class.class_name(), s));
        }

        mods.add_child(package_mods);
    }
    mods
}
