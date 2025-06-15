use crate::lite_csgen::api_types::CsType;
use crate::lite_csgen::gen_rs::RustEmitter;
use crate::lite_csgen::{api_types, wrappers};
use convert_case::{Case, Casing};
use gen_common::code_model::Module;
use gen_common::context::GenerationContext;
use gen_common::doc::strExt;
use gen_common::properties::{Getter, Setter};
use gen_common::templating::{render, render_string};
use lite_model::{Class, ConstantValue, DataType, EngineClass, Literal, Method};
use std::collections::{HashMap, HashSet};
use to_vec::ToVec;

pub(crate) fn generate_bindings(
    class: &EngineClass,
    ctx: &GenerationContext,
    rust: &mut RustEmitter,
) -> Module {
    let static_class = !class.methods.iter().any(|it| it.instance)
        && !ctx.domain.classes.iter().any(|it| match it {
            Class::Engine(it) => {
                let methods_can_create =
                    it.methods.iter().any(|it| match &it.signature.return_ty {
                        Some(it) => it.contains_type(&class.class_name),
                        _ => false,
                    });
                let constants_has = it
                    .constants
                    .iter()
                    .any(|it| it.ty.contains_type(&class.class_name));
                methods_can_create || constants_has
            }
            _ => false,
        });
    let mut s = String::new();

    let doc = class.description.to_xmldoc("            ");

    if class.class_name.0 == "GlobalScript" {
        // do not want to expose "abstract" flag to domain model for the single possible case
        render(
            &mut s,
            r#"
            // ${rust_path}
            ${doc}
            public abstract partial class ${class}
            {
            "#,
            [
                ("class", &class.class_name),
                ("doc", &doc),
                ("rust_path", &class.rust_struct_path),
            ],
        );
    } else if static_class {
        render(
            &mut s,
            r#"
            // ${rust_path}
            ${doc}
            public static partial class ${class}
            {
            "#,
            [
                ("class", &class.class_name),
                ("doc", &doc),
                ("rust_path", &class.rust_struct_path),
            ],
        );
    } else {
        render(
            &mut s,
            r#"
            // ${rust_path}
            ${doc}
            public partial struct ${class} : IEquatable<${class}>
            {
                #region internal fields and constructor
                private readonly NativeHandle handle;

                internal ${class}(NativeHandle handle)
                {
                    this.handle = handle;
                }
                #endregion

            "#,
            [
                ("class", &class.class_name),
                ("doc", &doc),
                ("rust_path", &class.rust_struct_path),
            ],
        );
    }

    rust.emit_statement(render_string(
        "
            #[repr(C)]
            #[derive(Clone, Copy)]
            pub struct ${class} {
                pub handle: NativeHandle
            }
        ",
        [(
            "class",
            &api_types::type_rs(&DataType::Object(class.class_name.clone()), ctx).to_native(),
        )],
    ));

    rust.emit_statement(generate_rust_conversions(class, static_class, ctx));

    for constant in class.constants.iter() {
        let doc = constant.description.to_xmldoc_commented("                ");

        let value = match &constant.value {
            ConstantValue::Literal(it) => match it {
                Literal::Bool(it) => format!("{}", it),
                Literal::Byte(it) => format!("{}", it),
                Literal::Number(it) => it.to_string(),
                Literal::String(it) => format!("{:?}", it),
            },
            ConstantValue::ComplexExpression(expr) => {
                render(
                    &mut s,
                    r#"
                ${doc}
                //public const ${type} ${name} = ${value};
                "#,
                    [
                        ("type", &api_types::type_cs(&constant.ty).to_facade()),
                        ("doc", &doc),
                        ("name", &constant.const_name),
                        ("value", &expr),
                    ],
                );
                continue;
            }
        };
        let doc = constant.description.to_xmldoc("                ");

        render(
            &mut s,
            r#"
                ${doc}
                public const ${type} ${name} = ${value};
                "#,
            [
                ("type", &api_types::type_cs(&constant.ty).to_facade()),
                ("doc", &doc),
                ("name", &constant.const_name),
                ("value", &value),
            ],
        );
    }

    let mut props: HashMap<String, (Option<Getter>, Option<Setter>)> = HashMap::new();

    for method in class.methods.iter() {
        if let Some(getter) = Getter::try_from(method) {
            let key = getter.prop_name.to_string();
            props.entry(key).or_default().0 = Some(getter);
        }
        if let Some(setter) = Setter::try_from(method) {
            let key = setter.prop_name.to_string();
            props.entry(key).or_default().1 = Some(setter);
        }
    }

    for method in class.methods.iter() {
        if let Some(getter) = Getter::try_from(method) {
            let Some((getter, setter)) = props.remove(&getter.prop_name) else {
                continue;
            };
            generate_property(&mut s, class, getter, setter, ctx);
            continue;
        }
        if let Some(setter) = Setter::try_from(method) {
            let Some((getter, setter)) = props.remove(&setter.prop_name) else {
                continue;
            };
            generate_property(&mut s, class, getter, setter, ctx);
            continue;
        }
        generate_method(&mut s, class, method, ctx);
    }
    s.push_str(
        "\n
    #region native internal methods",
    );
    for method in class.methods.iter() {
        let native_input_params = method
            .signature
            .params
            .iter()
            .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
            .map(|param| {
                format!(
                    "{}{} {}",
                    api_types::type_cs(&param.ty).to_blittable(),
                    if ctx.is_struct(&param.ty) { "*" } else { "" },
                    param.name
                )
            })
            .to_vec();

        render(
            &mut s,
            r#"

                [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
                private static unsafe partial ${return_ty} ${rust_path_escaped}_${name}(${this}${native_input_params});
        "#,
            [
                (
                    "return_ty",
                    &method
                        .signature
                        .return_ty
                        .as_ref()
                        .map(|it| api_types::type_cs(it).to_blittable())
                        .unwrap_or("void".to_string()),
                ),
                ("name", &method.method_name),
                ("native_input_params", &native_input_params.join(", ")),
                (
                    "rust_path_escaped",
                    &class.rust_struct_path.to_string().replace("::", "_"),
                ),
                (
                    "this",
                    &if method.instance {
                        format!(
                            "{} {}",
                            class.class_name,
                            if native_input_params.is_empty() {
                                "self"
                            } else {
                                "self, "
                            }
                        )
                    } else {
                        "".to_string()
                    },
                ),
            ],
        );
        generate_rust_entry_point(rust, class, method, ctx);
    }
    s.push_str(
        "
    #endregion\n",
    );

    if !static_class {
        render(
            &mut s,
            r#"

                public bool Equals(${class} other)
                {
                    return handle.Equals(other.handle);
                }

                public override bool Equals(object? obj)
                {
                    return obj is ${class} other && Equals(other);
                }

                public override int GetHashCode()
                {
                    return handle.GetHashCode();
                }

                public static bool operator ==(${class} left, ${class} right)
                {
                    return left.Equals(right);
                }

                public static bool operator !=(${class} left, ${class} right)
                {
                    return !left.Equals(right);
                }
        "#,
            [("class", &class.class_name)],
        );
    }

    render(
        &mut s,
        r#"
            }
        "#,
        [],
    );

    s.push_str("\n#region internal type wrappers\n");

    if !static_class {
        wrappers::generate_optional(
            &mut s,
            rust,
            &DataType::Object(class.class_name.clone()),
            ctx,
        );
        wrappers::generate_result(
            &mut s,
            rust,
            &DataType::Object(class.class_name.clone()),
            ctx,
        );
        wrappers::generate_result(
            &mut s,
            rust,
            &DataType::Option(Box::new(DataType::Object(class.class_name.clone()))),
            ctx,
        );
    }
    s.push_str("\n#endregion\n");

    Module::code(&class.class_name, s)
}

fn generate_property(
    s: &mut String,
    class: &EngineClass,
    getter: Option<Getter>,
    setter: Option<Setter>,
    ctx: &GenerationContext,
) {
    let instance: HashSet<_> = [
        getter.as_ref().map(|it| it.instance),
        setter.as_ref().map(|it| it.instance),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert_eq!(instance.len(), 1);
    let instance = instance.into_iter().next().unwrap();

    let prop_type: HashSet<_> = [
        getter.as_ref().map(|it| &it.prop_type),
        setter.as_ref().map(|it| &it.prop_type),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert_eq!(prop_type.len(), 1);
    let prop_type = prop_type.into_iter().next().unwrap();

    let prop_name: HashSet<_> = [
        getter.as_ref().map(|it| &it.prop_name),
        setter.as_ref().map(|it| &it.prop_name),
    ]
    .into_iter()
    .flatten()
    .collect();
    assert_eq!(prop_name.len(), 1);
    let prop_name = prop_name.into_iter().next().unwrap();

    let ty_marshalling = api_types::type_cs(prop_type);

    let desc = getter
        .as_ref()
        .map(|it| it.description.clone())
        .unwrap_or_else(|| setter.as_ref().unwrap().description.clone());

    let doc = desc.to_xmldoc("                ");

    render(
        s,
        r#"
                ${doc}
                public ${static}${property_type} ${property_name}
                {
    "#,
        [
            ("static", &if !instance { "static " } else { "" }),
            ("property_name", &prop_name.to_case(Case::Pascal)),
            ("doc", &doc),
            ("property_type", &ty_marshalling.to_facade()),
        ],
    );

    if let Some(Getter { .. }) = getter.as_ref() {
        render(
            s,
            r#"
                    get
                    {
                        #region native call
                        unsafe {
                            var __ret = ${rust_path_escaped}_get_${name}(${this});
                            return ${ret_expr};
                        }
                        #endregion
                    }
                "#,
            [
                (
                    "ret_expr",
                    &if ty_marshalling.is_mapped() {
                        format!("{}.ToFacade(__ret)", ty_marshalling.to_blittable())
                    } else {
                        "__ret".to_string()
                    },
                ),
                ("name", &prop_name),
                (
                    "rust_path_escaped",
                    &class.rust_struct_path.to_string().replace("::", "_"),
                ),
                ("this", &if instance { "this" } else { "" }),
            ],
        );
    }
    if let Some(Setter { has_result, .. }) = setter.as_ref() {
        let conversion = if let CsType::Mapped { blittable, .. } = api_types::type_cs(prop_type) {
            render_string(
                r#"var _value = ${blittable}.FromFacade(value)"#,
                [("blittable", &blittable)],
            )
        } else {
            render_string(r#"var _value = value"#, [])
        };
        render(
            s,
            r#"
                    set
                    {
                        #region native call
                        unsafe {
                            ${conversion};
                            ${ret_var}${rust_path_escaped}_set_${name}(${this}${deref}_value);
        "#,
            [
                ("name", &prop_name),
                ("deref", &if ctx.is_struct(prop_type) { "&" } else { "" }),
                (
                    "rust_path_escaped",
                    &class.rust_struct_path.to_string().replace("::", "_"),
                ),
                ("this", &if instance { "this, " } else { "" }),
                ("conversion", &conversion),
                ("ret_var", &if *has_result { "var __ret = " } else { "" }),
            ],
        );

        if *has_result {
            render(
                s,
                r#"
                            void_result.ToFacade(__ret);
            "#,
                [],
            );
        }

        render(
            s,
            r#"
                        }
                        #endregion
                    }
            "#,
            [],
        );
    }

    render(
        s,
        r#"
                }
    "#,
        [],
    );
}

fn generate_method(s: &mut String, class: &EngineClass, method: &Method, ctx: &GenerationContext) {
    let has_class_name_arg = method
        .signature
        .params
        .iter()
        .any(|it| matches!(&it.ty, DataType::ClassName));
    let input_params = method
        .signature
        .params
        .iter()
        .filter(|it| !matches!(&it.ty, DataType::ClassName))
        .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
        .map(|param| {
            format!(
                "{} {}",
                api_types::type_cs(&param.ty).to_facade(),
                param.name
            )
        })
        .to_vec();
    let output_params = method
        .signature
        .params
        .iter()
        .filter(|it| !matches!(&it.ty, DataType::UserScriptGenericStub))
        .map(|param| match &param.ty {
            DataType::ClassName => "NativeClassId.By<T>.Resolve()".to_string(),
            ty => {
                if ctx.is_struct(ty) {
                    format!("&_{}", param.name)
                } else {
                    format!("_{}", param.name)
                }
            }
        })
        .to_vec();

    let mut converted_params = String::new();
    for param in method.signature.params.iter() {
        if matches!(
            &param.ty,
            DataType::ClassName | DataType::UserScriptGenericStub
        ) {
            continue;
        }
        if let CsType::Mapped { blittable, .. } = api_types::type_cs(&param.ty) {
            render(
                &mut converted_params,
                r#"
                        var _${name} = ${blittable}.FromFacade(${name});
                "#,
                [("name", &param.name), ("blittable", &blittable)],
            );
        } else {
            render(
                &mut converted_params,
                r#"
                        var _${name} = ${name};
                "#,
                [("name", &param.name)],
            );
        }
    }
    let doc = method.description.to_xmldoc("                ");

    if let Some(return_ty) = &method.signature.return_ty {
        let return_ty = api_types::type_cs(return_ty);
        let rendered_return_ty = if has_class_name_arg {
            return_ty.to_facade_generic()
        } else {
            return_ty.to_facade()
        };
        render(
            s,
            r#"

                ${doc}
                public ${static}${return_ty} ${name}${generics}(${input_params})${generic_where}
                {
                    #region native call
                    unsafe {
                        ${converted_params}
                        var __ret = ${rust_path_escaped}_${rust_name}(${this}${output_params});
                        ${return}${ret_expr}${generic_cast};
                    }
                    #endregion
                }
            "#,
            [
                ("static", &if !method.instance { "static " } else { "" }),
                ("doc", &doc),
                ("return_ty", &rendered_return_ty),
                (
                    "ret_expr",
                    &if return_ty.is_mapped() {
                        format!("{}.ToFacade(__ret)", return_ty.to_blittable())
                    } else {
                        "__ret".to_string()
                    },
                ),
                (
                    "return",
                    &if rendered_return_ty == "void" {
                        ""
                    } else {
                        "return "
                    },
                ),
                (
                    "generic_cast",
                    &if has_class_name_arg { " as T" } else { "" },
                ),
                (
                    "generic_where",
                    &if has_class_name_arg {
                        " where T : class"
                    } else {
                        ""
                    },
                ),
                ("name", &method.method_name.to_case(Case::Pascal)),
                ("rust_name", &method.method_name),
                ("input_params", &input_params.join(", ")),
                ("output_params", &output_params.join(", ")),
                (
                    "rust_path_escaped",
                    &class.rust_struct_path.to_string().replace("::", "_"),
                ),
                ("generics", &if has_class_name_arg { "<T>" } else { "" }),
                (
                    "this",
                    &if method.instance {
                        if output_params.is_empty() {
                            "this"
                        } else {
                            "this, "
                        }
                    } else {
                        ""
                    },
                ),
                ("converted_params", &converted_params.trim_start()),
            ],
        );
    } else {
        render(
            s,
            r#"

                ${doc}
                public ${static}void ${name}${generics}(${input_params})
                {
                    #region native call
                    unsafe {
                        ${converted_params}
                        ${rust_path_escaped}_${rust_name}(${this}${output_params});
                    }
                    #endregion
                }
        "#,
            [
                ("static", &if !method.instance { "static " } else { "" }),
                ("doc", &doc),
                ("name", &method.method_name.to_case(Case::Pascal)),
                ("rust_name", &method.method_name),
                ("input_params", &input_params.join(", ")),
                ("output_params", &output_params.join(", ")),
                (
                    "rust_path_escaped",
                    &class.rust_struct_path.to_string().replace("::", "_"),
                ),
                ("generics", &if has_class_name_arg { "<T>" } else { "" }),
                (
                    "this",
                    &if method.instance {
                        if output_params.is_empty() {
                            "this"
                        } else {
                            "this, "
                        }
                    } else {
                        ""
                    },
                ),
                ("converted_params", &converted_params.trim_start()),
            ],
        );
    }
}
fn generate_rust_conversions(
    class: &EngineClass,
    static_class: bool,
    ctx: &GenerationContext,
) -> String {
    let mut rs = String::new();
    let ty = DataType::Object(class.class_name.clone());

    if !static_class {
        render(
            &mut rs,
            r#"

            impl From<${class_lite}> for ${class_native} {
                fn from(value: ${class_lite}) -> Self {
                    Self { handle: NativeHandle::from_u128(value.to_external()) }
                }
            }

            impl From<${class_native}> for ${class_lite} {
                fn from(value: ${class_native}) -> Self {
                    Self::from_external(value.handle.as_u128())
                }
            }
    "#,
            [
                ("class_native", &api_types::type_rs(&ty, ctx).to_native()),
                ("class_lite", &api_types::type_rs(&ty, ctx).to_lite()),
            ],
        );
    }
    rs
}

fn generate_rust_entry_point(
    rust: &mut RustEmitter,
    class: &EngineClass,
    method: &Method,
    ctx: &GenerationContext,
) {
    let rs_type = api_types::type_rs(&DataType::Object(class.class_name.clone()), ctx);

    let mut output_params = String::new();
    let mut input_params = String::new();
    let mut conversions = String::new();

    let this = format!(
        "this: {},
                ",
        rs_type.to_native(),
    );

    for param in method.signature.params.iter() {
        if matches!(param.ty, DataType::UserScriptGenericStub) {
            output_params += "(),";
            continue;
        }
        if ctx.is_struct(&param.ty) {
            render(
                &mut conversions,
                r#"
                let ${name} = unsafe { *${name} }.into();
            "#,
                [("name", &param.name)],
            );
        } else {
            render(
                &mut conversions,
                r#"
                let ${name} = ${name}.into();
            "#,
                [("name", &param.name)],
            );
        }
        output_params += param.name.as_str();
        output_params += ",";
        input_params += format!(
            "{}: {}{},",
            param.name,
            if ctx.is_struct(&param.ty) {
                "*mut "
            } else {
                ""
            },
            api_types::type_rs(&param.ty, ctx).to_native()
        )
        .as_str();
    }

    let mut rs = String::new();
    render(
        &mut rs,
        r#"
            #[no_mangle]
            pub extern "C" fn ${rust_path_escaped}_${name}(
                ${this}${input_params}
            ) -> ${return_ty} {
                ${conversions}
                let ret = ${receiver}${name}${generics}(${output_params});
                ret.into()
            }
            "#,
        [
            (
                "generics",
                &if method.is_generic() {
                    "::<crate::UserScriptImpl>"
                } else {
                    ""
                },
            ),
            (
                "return_ty",
                &method
                    .signature
                    .return_ty
                    .as_ref()
                    .map(|it| api_types::type_rs(it, ctx).to_native())
                    .unwrap_or("()".to_string()),
            ),
            (
                "rust_path_escaped",
                &class.rust_struct_path.to_string().replace("::", "_"),
            ),
            ("name", &method.method_name),
            ("output_params", &output_params),
            ("input_params", &input_params),
            ("conversions", &conversions),
            (
                "this",
                &if method.instance {
                    this
                } else {
                    "".to_string()
                },
            ),
            (
                "receiver",
                &if method.instance {
                    format!("{}::from(this).", rs_type.to_lite())
                } else {
                    format!("{}::", rs_type.to_lite())
                },
            ),
        ],
    );

    rust.emit_statement(rs);
}
