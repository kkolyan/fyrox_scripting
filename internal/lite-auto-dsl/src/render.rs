use crate::ImportResult;
use fmt::Write;
use lite_model::DataType;
use std::path::Path;
use std::{fmt, fs};

pub fn render(file: impl AsRef<Path>, symbols: ImportResult) {
    let file = file.as_ref();
    let _ = fs::create_dir_all(file.parent().unwrap());

    let mut s = String::new();
    for (original_name, struct_) in symbols.struct_classes {
        let struct_name = struct_.class_name;
        let mut fields = String::new();
        for field in struct_.fields {
            let field_name = field.name;
            let field_ty = ty_to_rs(&field.ty);
            write!(
                fields,
                "
                pub {field_name}: {field_ty},
            "
            )
            .unwrap();
        }
        write!(
            &mut s,
            r###"
            pub struct {struct_name}
            {{{fields}
            }}
            "###,
        )
        .unwrap();
    }
    fs::write(file, s).unwrap();
    todo!()
}

fn ty_to_rs(ty: &DataType) -> String {
    match ty {
        DataType::UnresolvedClass(it) => todo!("{:?}", it),
        DataType::Unit => todo!(),
        DataType::Bool => todo!(),
        DataType::Byte => todo!(),
        DataType::I32 => format!("i32"),
        DataType::I64 => format!("i64"),
        DataType::F32 => format!("f32"),
        DataType::F64 => format!("f64"),
        DataType::String => todo!(),
        DataType::ClassName => todo!(),
        DataType::Vec(it) => format!("Vec<{}>", ty_to_rs(&it)),
        DataType::UserScript => todo!(),
        DataType::UserScriptMessage => todo!(),
        DataType::UserScriptGenericStub => todo!(),
        DataType::Object(_) => todo!(),
        DataType::Option(_) => todo!(),
        DataType::Result { .. } => todo!(),
    }
}
