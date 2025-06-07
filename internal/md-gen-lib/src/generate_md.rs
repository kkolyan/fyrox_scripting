use lite_model::{Class, ClassName, DataType, Domain};
use std::collections::HashMap;
use std::mem;

use crate::{
    md::{engine_class::generate_engine, enum_class::generate_enum, struct_class::generate_struct},
    writelnu, Naming,
};
use gen_common::by_package::extract_package;
use gen_common::{
    by_package::classes_by_package,
    code_model::{HierarchicalCodeBase, Module},
    templating::strExt,
};

pub fn generate_md(domain: &Domain, naming: Naming) -> HierarchicalCodeBase {
    let mut mods = vec![];

    let mut class_page_links = HashMap::new();
    for x in domain.classes.iter() {
        class_page_links.insert(
            x.class_name().clone(),
            format!(
                "../{}/{}.md",
                naming.package_name(extract_package(x.rust_name())),
                x.class_name()
            ),
        );
    }

    let by_package = classes_by_package(domain);
    for (package, classes) in by_package.iter() {
        let mut package_mods = vec![];

        for class in classes.iter() {
            let class = domain.get_class(&class).unwrap();
            let mut s = String::new();
            match class {
                Class::Engine(class) => generate_engine(&mut s, class, naming, &class_page_links),
                Class::Struct(class) => generate_struct(&mut s, class, naming, &class_page_links),
                Class::Enum(class) => generate_enum(&mut s, class, naming, &class_page_links),
                _ => {}
            }
            writelnu!(s, "");

            package_mods.push(Module::code(class.class_name(), s));
        }
        package_mods.push(Module::code(
            "README",
            generate_package(&package, &classes, domain, naming, &class_page_links),
        ));

        mods.push(Module::children(
            naming.package_name(&package),
            package_mods,
        ));
    }
    mods.push(Module::code("README", generate_index(&by_package, naming)));
    HierarchicalCodeBase { mods }
}

fn generate_index(packages: &Vec<(String, Vec<ClassName>)>, naming: Naming) -> String {
    let mut s = "".to_string();

    writelnu!(s, "## Packages");
    for (package, _) in packages.iter() {
        let package = naming.package_name(package);
        writelnu!(s, "* [{}]({}/README.md)", package, package);
    }
    s
}

fn generate_package(
    package: &str,
    classes_of_package: &Vec<ClassName>,
    domain: &Domain,
    naming: Naming,
    class_page_links: &HashMap<ClassName, String>,
) -> String {
    let mut s = "".to_string();
    writelnu!(s, "# {}", naming.package_name(package));
    writelnu!(s, "package in [FyroxLite](../README.md)");

    let description = &domain
        .packages
        .iter()
        .find(|it| it.name.strip_prefix("fyrox_lite::").unwrap() == package)
        .unwrap_or_else(|| {
            panic!(
                "expected to find package {} along packages {:?}",
                package, domain.packages
            )
        })
        .description;
    if !description.is_empty() {
        writelnu!(s, "## Description");
        writelnu!(s, "{}", description);
    }

    let mut classes = vec![];
    let mut structs = vec![];
    let mut enums = vec![];

    for class in classes_of_package {
        let class = domain.get_class(&class).unwrap();
        match class {
            Class::Engine(it) => classes.push(it),
            Class::Struct(it) => structs.push(it),
            Class::Enum(it) => enums.push(it),
        }
    }

    if !classes.is_empty() {
        writelnu!(s, "## Classes");
        for x in classes.iter() {
            writelnu!(
                s,
                "* [{}]({})",
                x.class_name,
                class_page_links.get(&x.class_name).unwrap()
            );
        }
    }

    if !structs.is_empty() {
        writelnu!(s, "## Structs");
        for x in structs.iter() {
            writelnu!(
                s,
                "* [{}]({})",
                x.class_name,
                class_page_links.get(&x.class_name).unwrap()
            );
        }
    }

    if !enums.is_empty() {
        writelnu!(s, "## Enums");
        for x in enums.iter() {
            writelnu!(
                s,
                "* [{}]({})",
                x.class_name,
                class_page_links.get(&x.class_name).unwrap()
            );
        }
    }
    s
}
