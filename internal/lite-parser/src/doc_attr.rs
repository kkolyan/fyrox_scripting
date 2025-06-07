use itertools::Itertools;
use syn::{Attribute, Meta, Lit, Expr};

pub fn extract_doc(attrs: &[Attribute]) -> String {
    attrs.iter()
        .filter_map(|attr| {
            if attr.path().is_ident("doc") {
                if let Meta::NameValue(meta) = &attr.meta {
                    if let Expr::Lit(it) = &meta.value {
                        if let Lit::Str(lit_str) = &it.lit {
                            return Some(lit_str.value().trim().to_string());
                        }
                    }
                }
            }
            None
        })
        .join("\n")
}