use std::collections::HashMap;
use lite_model::{ClassName, EnumClass, EnumValue};
use gen_common::by_package::extract_package;
use gen_common::doc::strExt;
use gen_common::writelnu;
use crate::Naming;

pub fn generate_enum(s: &mut String, class: &EnumClass, naming: Naming, x1: &HashMap<ClassName, String>) {
	writelnu!(s, "# {}", class.class_name);
	let package = naming.package_name(extract_package(&class.rust_struct_path));
	writelnu!(s, "enum in [{package}](../{package}.md)");
	if !class.description.is_empty() {
		writelnu!(s, "\n## Description");
		writelnu!(s, "{}", class.description.to_book());
	}
	
	writelnu!(s, "\n## Properties");
	if class.variants.iter().any(|it| !matches!(it.value, EnumValue::Unit)) {
		todo!("ADT not implemented")
	}
	writelnu!(
        s,
        "| Property | Description |"
    );
	writelnu!(s, "|---|---|");
	for variant in class.variants.iter() {
		writelnu!(s, "| `{}` | {} |", variant.tag, variant.description.to_book().replace("\n", " "));
	}
}
