use lite_model::{Class, ClassName, Domain};
use std::collections::HashMap;

use crate::md::cs_to_domain::CSharpDomain;
use crate::md::sections::{Section, Sections};
use crate::{
    md::{engine_class::generate_engine, enum_class::generate_enum, struct_class::generate_struct},
    Naming,
};
use gen_common::by_package::extract_package;
use gen_common::{by_package::classes_by_package, code_model::Module};

pub fn generate_md(domain: &Domain, csharp_domain: &CSharpDomain, naming: Naming) -> Module {
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

    for package in csharp_domain.packages.iter() {
        for ty in package.collect_type_names() {
            class_page_links.insert(
                ClassName(ty.clone()),
                format!("../{}/{}.md", package.name.clone(), &ty),
            );
        }
    }

    let mut sections = generate_rust_md(domain, naming, &mut class_page_links);
    sections.merge(csharp_domain.generate_md(&class_page_links));
    sections.to_module(&mut class_page_links)
}

fn generate_rust_md(
    domain: &Domain,
    naming: Naming,
    class_page_links: &mut HashMap<ClassName, String>,
) -> Sections {
    let mut root = Sections::default();
    let by_package = classes_by_package(domain);
    for (package, classes) in by_package.iter() {
        let description = &domain
            .packages
            .iter()
            .find(|it| {
                ["fyrox_lite::", "fyrox_lite_math::", "fyrox_lite_color::"]
                    .iter()
                    .filter_map(|prefix| it.name.strip_prefix(prefix))
                    .next()
                    .unwrap()
                    == package
            })
            .unwrap_or_else(|| {
                panic!(
                    "expected to find package {} along packages {:?}",
                    package, domain.packages
                )
            })
            .description;

        let mut package_mod = Section::new(naming.package_name(package), Some(description.clone()));

        for class in classes.iter() {
            let class = domain.get_class(class).unwrap();
            let mut s = String::new();
            match class {
                Class::Engine(class) => {
                    generate_engine(&mut s, class, naming, class_page_links);
                    package_mod.classes.insert(
                        class.class_name.0.clone(),
                        Module::code(class.class_name.clone(), s),
                    );
                }
                Class::Struct(class) => {
                    generate_struct(&mut s, class, naming, class_page_links);
                    package_mod.classes.insert(
                        class.class_name.0.clone(),
                        Module::code(class.class_name.clone(), s),
                    );
                }
                Class::Enum(class) => {
                    generate_enum(&mut s, class, naming);
                    package_mod.classes.insert(
                        class.class_name.0.clone(),
                        Module::code(class.class_name.clone(), s),
                    );
                }
            };
        }

        root.add_child(package_mod);
    }
    root
}
