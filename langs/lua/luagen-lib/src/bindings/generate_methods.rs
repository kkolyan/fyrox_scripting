use std::borrow::Cow;

use lite_model::{DataType, EngineClass};
use to_vec::ToVec;

use super::expressions::{mlua_to_rust_expr, rust_expr_to_mlua, type_to_mlua};
use gen_common::properties::is_regular;
use gen_common::{context::GenerationContext, templating::render};

pub const USER_SCRIPT_IMPL: &str = "TypedUserData<UserScriptProxy>";

pub fn generate_methods(
    s: &mut String,
    class: &EngineClass,
    ctx: &GenerationContext,
    instance: bool,
) {
    for method in class
        .methods
        .iter()
        .filter(|it| is_regular(it))
        .filter(|it| it.instance == instance)
    {
        let params = method
            .signature
            .params
            .iter()
            .filter(|it| !matches!(it.ty, DataType::UserScriptGenericStub))
            .to_vec();

        let generics = match method.is_generic() {
            true => format!("::<{}>", USER_SCRIPT_IMPL),
            false => "".to_owned(),
        };

        let input_names = params
            .iter()
            .flat_map(|it| match it.ty {
                DataType::UserScriptMessage => {
                    vec![format!("{}_type", it.name.as_str()), it.name.to_string()]
                }
                _ => vec![it.name.to_string()],
            })
            .to_vec()
            .join(", ");

        let output_names = method
            .signature
            .params
            .iter()
            .map(|it| it.name.as_str())
            .to_vec()
            .join(", ");

        let param_types = params
            .iter()
            .flat_map(|it| match it.ty {
                DataType::UserScriptMessage => {
                    vec!["mlua::Value".to_string(), "mlua::Value".to_string()]
                }
                _ => vec![type_to_mlua(&it.ty, ctx)],
            })
            .to_vec()
            .join(", ");

        render(
            s,
            r#"
                methods.add_method_mut(
                    "${method_name}",
                    |lua, this, (${input_names}): (${param_types})| {
            "#,
            [
                ("method_name", &method.method_name),
                ("input_names", &input_names),
                ("param_types", &param_types),
            ],
        );

        for param in method.signature.params.iter() {
            let expression = mlua_to_rust_expr(&param.name, &param.ty, ctx);
            render(
                s,
                r#"
                    let ${param_name} = ${expression};
            "#,
                [("param_name", &param.name), ("expression", &expression)],
            );
        }

        let target = match instance {
            true => Cow::Borrowed("this."),
            false => Cow::Owned(format!("{}::", class.rust_struct_path.0)),
        };

        let ret_expression = rust_expr_to_mlua(
            ctx,
            "ret",
            method
                .signature
                .return_ty
                .as_ref()
                .unwrap_or(&DataType::Unit),
        );

        render(
            s,
            r#"
                        let ret = ${target}${method_name}${generics}(${output_names});
                        let ret = ${ret_expression};
                        Ok(ret)
                    },
                );
            "#,
            [
                ("target", &target),
                ("method_name", &method.method_name),
                ("output_names", &output_names),
                ("generics", &generics),
                ("ret_expression", &ret_expression),
            ],
        );
    }
}
