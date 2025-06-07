use std::collections::HashMap;
use lite_model::{ClassName, DataType};

#[allow(clippy::useless_format)]
pub fn type_rust_to_md(ty: &DataType, class_md_links: &HashMap<ClassName, String>) -> String {
    match ty {
        DataType::UnresolvedClass(it) => panic!("unresolved class: {}", it),
        DataType::Unit => format!("void"), //format!("[void⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/void)"),
        DataType::Bool => format!("bool"), //format!("[bool⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/bool)"),
        DataType::Byte => format!("byte"), //format!("[byte⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/integral-numeric-types)"),
        DataType::I32 => format!("int"), //format!("[int⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/integral-numeric-types)"),
        DataType::I64 => format!("long"), //format!("[long⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/integral-numeric-types)"),
        DataType::F32 => format!("float"), //format!("[float⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/floating-point-numeric-types)"),
        DataType::F64 => format!("double"), //format!("[double⤴](https://learn.microsoft.com/en-us/dotnet/csharp/language-reference/builtin-types/floating-point-numeric-types)"),
        DataType::String => format!("string"), //format!("[string⤴](https://learn.microsoft.com/en-us/dotnet/csharp/programming-guide/strings)"),
        DataType::ClassName => format!("`T`"),
        DataType::Vec(item_ty) => format!("List< {} >", type_rust_to_md(item_ty, class_md_links)),
        DataType::UserScript => format!("T"),
        DataType::UserScriptMessage => format!("object"), //format!("[object⤴](https://learn.microsoft.com/en-us/dotnet/csharp/fundamentals/object-oriented/objects)"),
        DataType::UserScriptGenericStub => {
            panic!("UserScriptGenericStub is not allowed in this context")
        }
        DataType::Object(class_name) => class_md_links.get(class_name).map(|it| format!("[{}]({})", class_name.to_string(), it)).unwrap_or(class_name.to_string()),
        DataType::Option(item_ty) => format!("{}?", type_rust_to_md(item_ty, class_md_links)),
        DataType::Result { ok } => type_rust_to_md(ok, class_md_links),
    }
}
