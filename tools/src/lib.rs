pub mod csgen;
pub mod luagen;
pub mod mdgen;

use std::fs;

use gen_common::code_model::Module;
use lite_model::Domain;
use lite_parser::parse_domain_metadata::parse_domain_metadata;

pub fn write_annotations(annotations: Module) {
    let target_path = "langs/lua/annotations";
    println!("clearing {}", target_path);
    let _ = fs::remove_dir_all(target_path);
    println!("writing bindings to {}", target_path);
    annotations.write_lua(target_path);
}

pub fn write_bindings(bindings: Module) {
    let target_path = "langs/lua/fyrox-lite-lua-lib/src/generated";
    println!("clearing {}", target_path);
    let _ = fs::remove_dir_all(target_path);
    println!("writing bindings to {}", target_path);
    bindings.write_rust(target_path);
}

pub fn get_fyrox_lite_domain() -> Domain {
    parse_domain_metadata("fyrox-lite")
}

pub fn get_combined_domain() -> Domain {
    let mut fyrox: Domain = get_fyrox_lite_domain();
    let math: Domain = parse_domain_metadata("fyrox-lite-math");
    let color: Domain = parse_domain_metadata("fyrox-lite-color");

    // math "overrides" classes in fyrox by name
    fyrox.classes.retain_mut(|fyrox_class| {
        let override_class = None
            .or_else(|| math.get_class(fyrox_class.class_name()))
            .or_else(|| color.get_class(fyrox_class.class_name()));
        if let Some(override_class) = override_class {
            println!(
                "overriding {} by {}",
                fyrox_class.rust_name(),
                override_class.rust_name()
            );
        }
        override_class.is_none()
    });

    Domain::merge_all([fyrox, math, color])
}
