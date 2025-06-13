use std::collections::HashMap;
use gen_common::by_package::extract_package;
use gen_common::doc::strExt;
use gen_common::writelnu;
use lite_model::{ClassName, StructClass};

use crate::{md::type_to_md::type_rust_to_md, Naming};

pub fn generate_struct(s: &mut String, class: &StructClass, naming: Naming, class_page_links: &HashMap<ClassName, String>) {
    writelnu!(s, "# {}", class.class_name);
    let package = naming.package_name(extract_package(&class.rust_struct_path));
    writelnu!(s, "struct in [FyroxLite](../../scripting_api.md).[{package}](../{package}.md)");
    if !class.description.is_empty() {
        writelnu!(s, "\n## Description");
        writelnu!(s, "{}", class.description.to_book());
    }
    writelnu!(s, "\n## Fields");

    writelnu!(s, "| Name | Type | Description |");
    writelnu!(s, "|---|---|---|");
    for field in class.fields.iter() {
        writelnu!(
            s,
            "| `{}` | {} | {} |",
            naming.member_name(&field.name),
            type_rust_to_md(&field.ty, class_page_links),
            field.description.to_book().replace("\n", " ")
        );
    }
}
