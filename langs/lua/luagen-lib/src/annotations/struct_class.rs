use gen_common::doc::strExt;
use gen_common::writelnu;
use lite_model::StructClass;

use crate::{annotations::type_to_lua::type_rust_to_lua};

pub fn generate_struct(s: &mut String, class: &StructClass) {
    writelnu!(s, "");
    s.push_str(class.description.to_luadoc("").as_str());
    writelnu!(s, "---@class {}", class.class_name);
    fields(s, class);
    writelnu!(s, "{}_instance = {{}}", class.class_name);
}

pub fn fields(s: &mut String, class: &StructClass) {
    for field in class.fields.iter() {
        writelnu!(
            s,
            "---@field {} {} ---{}",
            &field.name,
            &type_rust_to_lua(&field.ty),
            field.description.to_luadoc_inline(),
        );
    }
}
