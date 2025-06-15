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
#![allow(clippy::unit_arg)]

use fyrox_lite::*;
use fyrox_lite_math::*;
use mlua;

use crate::{
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
    user_script_impl::{LuaUserScriptMessageEnvelope, UserScriptProxy},
};

impl FyroxUserData for fyrox_lite_math::lite_math::LiteVector2 {
    const CLASS_NAME: &'static str = "Vector2";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
        methods.add_method_mut("mul", |lua, this, (o): (f32)| {
            let o = o;
            let ret = this.mul(o);
            let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(ret));
            Ok(ret)
        });
        methods.add_method_mut(
            "add",
            |lua, this, (o): (TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector2>>)| {
                let o = o.borrow()?.inner().clone().into();
                let ret = this.add(o);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(ret));
                Ok(ret)
            },
        );
        methods.add_method_mut("normalize", |lua, this, (): ()| {
            let ret = this.normalize();
            let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(ret));
            Ok(ret)
        });
        methods.add_method_mut(
            "sub",
            |lua, this, (o): (TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector2>>)| {
                let o = o.borrow()?.inner().clone().into();
                let ret = this.sub(o);
                let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(ret));
                Ok(ret)
            },
        );
        methods.add_method_mut("magnitude", |lua, this, (): ()| {
            let ret = this.magnitude();
            let ret = ret;
            Ok(ret)
        });
        methods.add_method_mut("normalize_inplace", |lua, this, (): ()| {
            let ret = this.normalize_inplace();
            let ret = ret;
            Ok(ret)
        });
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
        methods.add_method_mut("new", |lua, this, (x, y): (f32, f32)| {
            let x = x;
            let y = y;
            let ret = fyrox_lite_math::lite_math::LiteVector2::new(x, y);
            let ret = Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(ret));
            Ok(ret)
        });
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("x", |lua, this| {
            let value = this.get_x();
            Ok(value)
        });
        fields.add_field_method_get("y", |lua, this| {
            let value = this.get_y();
            Ok(value)
        });
        fields.add_field_method_set("x", |lua, this, value: f32| {
            let value = this.set_x(value);
            Ok(value)
        });
        fields.add_field_method_set("y", |lua, this, value: f32| {
            let value = this.set_y(value);
            Ok(value)
        });
    }
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("ZERO", |lua, this| {
            let value = fyrox_lite_math::lite_math::LiteVector2::get_ZERO();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector2::from(
                value,
            )))
        });
    }
}
