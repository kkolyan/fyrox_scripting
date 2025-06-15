#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::redundant_locals)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]
#![allow(clippy::let_and_return)]
#![allow(clippy::just_underscores_and_digits)]
#![allow(clippy::manual_map)]
#![allow(clippy::needless_match)]
#![allow(clippy::let_unit_value)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};
impl<'lua> mlua::IntoLua<'lua> for Traitor<fyrox_lite::lite_ui::TextBuilder> {
    fn into_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Table({
            let t = lua.create_table()?;
            t.set("foreground", {
                let foreground = self.foreground.clone();
                if let Some(foreground) = foreground {
                    Some(Traitor::new(fyrox_lite::lite_ui::Brush::from(foreground)))
                } else {
                    None
                }
            })?;
            t.set("font_size", {
                let font_size = self.font_size.clone();
                if let Some(font_size) = font_size {
                    Some(font_size)
                } else {
                    None
                }
            })?;
            t
        }))
    }
}
impl<'lua> mlua::FromLua<'lua> for Traitor<fyrox_lite::lite_ui::TextBuilder> {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::Table(value) = value else {
            return Err(lua_error!(
                "cannot extract TextBuilder from {:?}. expected table.",
                value
            ));
        };
        let foreground =
            value.get::<_, Option<Traitor<fyrox_lite::lite_ui::Brush>>>("foreground")?;
        let foreground = if let Some(foreground) = foreground {
            Some(foreground.inner().clone().into())
        } else {
            None
        };
        let font_size = value.get::<_, Option<f32>>("font_size")?;
        let font_size = if let Some(font_size) = font_size {
            Some(font_size)
        } else {
            None
        };
        Ok(Traitor::new(fyrox_lite::lite_ui::TextBuilder {
            foreground,
            font_size,
        }))
    }
}
