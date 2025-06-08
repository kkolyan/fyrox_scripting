use core::panic;
use std::collections::HashMap;
use lite_model::{ClassName, EnumClass, EnumValue};
use to_vec::ToVec;
use gen_common::by_package::extract_package;
use gen_common::writelnu;
use crate::{md::type_to_md::type_rust_to_md, Naming};

pub fn generate_enum(s: &mut String, class: &EnumClass, naming: Naming, x1: &HashMap<ClassName, String>) {
	writelnu!(s, "# {}", class.class_name);
	let package = naming.package_name(extract_package(&class.rust_struct_path));
	writelnu!(s, "enum in [FyroxLite](../../scripting_api.md).[{package}](../{package}.md)");
	if !class.description.is_empty() {
		writelnu!(s, "## Description");
		writelnu!(s, "{}", class.description);
	}
	
	writelnu!(s, "## Properties");
	if class.variants.iter().any(|it| !matches!(it.value, EnumValue::Unit)) {
		todo!("ADT not implemented")
	}
	writelnu!(
        s,
        "| Property | Description |"
    );
	writelnu!(s, "|---|---|");
	for variant in class.variants.iter() {
		writelnu!(s, "| `{}` | {} |", variant.tag, variant.description);
	}
}
