use lite_model::Class;

use gen_common::{code_model::Module, context::GenerationContext, templating::render};

pub fn generate_registry(ctx: &GenerationContext) -> Module {
    let mut s = String::new();
    render(
        &mut s,
        r#"
        pub fn register_classes(lua: &mlua::Lua) {
            use crate::user_data_plus::FyroxUserData;
    "#,
        [],
    );
    s.push('\n');
    for class in ctx.domain.classes.iter() {
        let provides_class = match class {
            Class::Engine(_) => true,
            Class::Struct(_) => false,
            Class::Enum(_) => true,
        };
        if provides_class {
            s += format!(
                "
            {}::register_class(lua);
            ",
                class.rust_name()
            )
            .as_str();
        }
    }
    s += r#"
        }
    "#;

    Module::code("registry", s)
}
