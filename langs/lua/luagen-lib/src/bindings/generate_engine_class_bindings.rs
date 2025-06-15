use convert_case::{Case, Casing};
use lite_model::{DataType, EngineClass};
use std::borrow::Cow;
use to_vec::ToVec;

use super::{
    eq::generate_eq,
    expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua},
    generate_methods::generate_methods,
    supress_lint::SUPRESSIONS,
};
use crate::bindings::generate_methods::USER_SCRIPT_IMPL;
use gen_common::properties::Setter;
use gen_common::{
    code_model::Module, context::GenerationContext, properties::is_getter, templating::render,
};

pub fn generate_engine_class_bindings(class: &EngineClass, ctx: &GenerationContext) -> Module {
    let mut s: String = Default::default();
    s.push_str(SUPRESSIONS);

    render(
        &mut s,
        r#"

        use fyrox_lite::*;
        use fyrox_lite_math::*;
        use mlua;

        use crate::{
            user_data_plus::{FyroxUserData, Traitor, UserDataClass},
            script_object::ScriptObject,
            typed_userdata::TypedUserData,
            user_script_impl::{UserScriptProxy, LuaUserScriptMessageEnvelope},
        };

        impl FyroxUserData for ${rust_struct_path} {
            const CLASS_NAME: &'static str = "${class_name}";
        "#,
        [
            ("class_name", &class.class_name.0),
            ("rust_struct_path", &class.rust_struct_path.0),
        ],
    );

    generate_instance_methods(&mut s, class, ctx);
    generate_class_methods(&mut s, class, ctx);
    generate_instance_fields(&mut s, class, ctx);
    generate_class_fields(&mut s, class, ctx);

    s += "
        }
    ";

    Module::code(class.class_name.0.to_case(Case::Snake), s)
}

fn generate_class_methods(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(methods: &mut M) {
    "#,
        [],
    );
    generate_methods(s, class, ctx, false);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_instance_methods(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            
            fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
                methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
                    Ok(format!("{:?}", this.inner()))
                });
        "#,
        [],
    );
    generate_eq(s, &class.features);
    generate_methods(s, class, ctx, true);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_instance_fields(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
    "#,
        [],
    );

    generate_instance_getters(s, class, ctx);

    generate_setters(s, class, ctx, true);

    render(
        s,
        r#"
            }
    "#,
        [],
    );
}

fn generate_setters(s: &mut String, class: &EngineClass, ctx: &GenerationContext, instance: bool) {
    let writable_props = class
        .methods
        .iter()
        .filter(|it| it.instance == instance)
        .flat_map(|method| Setter::try_from(method).map(|it| (it, method)))
        .to_vec();

    for (prop, setter) in writable_props {
        let target = match instance {
            true => Cow::Borrowed("this."),
            false => Cow::Owned(format!("{}::", class.rust_struct_path.0)),
        };
        render(
            s,
            r#"
                fields.add_field_method_set("${field_name}", |lua, this, value: ${input_type}| {
                    let value = ${target}set_${field_name}${generics}(${expression}, ${stub});
                    Ok(${output_expression})
                });
        "#,
            [
                (
                    "generics",
                    &if setter.is_generic() {
                        format!("::<{}>", USER_SCRIPT_IMPL)
                    } else {
                        "".to_owned()
                    },
                ),
                (
                    "stub",
                    &if setter
                        .signature
                        .params
                        .iter()
                        .any(|it| matches!(it.ty, DataType::UserScriptGenericStub))
                    {
                        "()"
                    } else {
                        ""
                    },
                ),
                ("field_name", &prop.prop_name),
                (
                    "input_type",
                    &type_to_mlua(&setter.signature.params[0].ty, ctx),
                ),
                (
                    "expression",
                    &mlua_to_rust_expr("value", &setter.signature.params[0].ty, ctx),
                ),
                (
                    "target",
                    match instance {
                        true => &"this.",
                        false => &target,
                    },
                ),
                (
                    "output_expression",
                    &rust_expr_to_mlua(ctx, "value", setter.signature.return_ty.as_ref().unwrap_or(&DataType::Unit)),
                ),
            ],
        );
    }
}

fn generate_instance_getters(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    let readable_props = class
        .methods
        .iter()
        .filter(|it| is_getter(it))
        .filter(|it| it.instance)
        .map(|method| (method.method_name.strip_prefix("get_").unwrap(), method))
        .to_vec();

    for (prop, getter) in readable_props {
        render(
            s,
            r#"
                fields.add_field_method_get("${field_name}", |lua, this| {
                    let value = this.get_${field_name}${generics}(${stub});
                    Ok(${expression})
                });
        "#,
            [
                (
                    "generics",
                    &if getter.is_generic() {
                        format!("::<{}>", USER_SCRIPT_IMPL)
                    } else {
                        "".to_owned()
                    },
                ),
                (
                    "stub",
                    &if getter
                        .signature
                        .params
                        .iter()
                        .any(|it| matches!(it.ty, DataType::UserScriptGenericStub))
                    {
                        "()"
                    } else {
                        ""
                    },
                ),
                ("field_name", &prop),
                (
                    "expression",
                    &rust_expr_to_mlua(ctx, "value", getter.signature.return_ty.as_ref().unwrap()),
                ),
            ],
        );
    }
}

fn generate_class_fields(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    render(
        s,
        r#"
            fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
    "#,
        [],
    );

    generate_class_getters(s, class, ctx);
    generate_setters(s, class, ctx, false);

    *s += "
            }
    ";
}

fn generate_class_getters(s: &mut String, class: &EngineClass, ctx: &GenerationContext) {
    for constant in class.constants.iter() {
        render(
            s,
            r#"                
                fields.add_field_method_get("${field_name}", |lua, this| {
                    let value = ${rust_struct_path}::${field_name};
                    Ok(${expression})
                });
        "#,
            [
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &constant.const_name),
                ("expression", &rust_expr_to_mlua(ctx, "value", &constant.ty)),
            ],
        );
    }

    let readable_static_props = class
        .methods
        .iter()
        .filter(|it| is_getter(it))
        .filter(|it| !it.instance)
        .map(|method| (method.method_name.strip_prefix("get_").unwrap(), method))
        .to_vec();

    for (prop, getter) in readable_static_props {
        render(
            s,
            r#"                
                fields.add_field_method_get("${field_name}", |lua, this| {
                    let value = ${rust_struct_path}::get_${field_name}${generics}(${stub});
                    Ok(${expression})
                });
        "#,
            [
                (
                    "generics",
                    &if getter.is_generic() {
                        format!("::<{}>", USER_SCRIPT_IMPL)
                    } else {
                        "".to_owned()
                    },
                ),
                (
                    "stub",
                    &if getter
                        .signature
                        .params
                        .iter()
                        .any(|it| matches!(it.ty, DataType::UserScriptGenericStub))
                    {
                        "()"
                    } else {
                        ""
                    },
                ),
                ("rust_struct_path", &class.rust_struct_path.0),
                ("field_name", &prop),
                (
                    "expression",
                    &rust_expr_to_mlua(ctx, "value", getter.signature.return_ty.as_ref().unwrap()),
                ),
            ],
        );
    }
}
