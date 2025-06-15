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
    lua_error,
    script_object::ScriptObject,
    typed_userdata::TypedUserData,
    user_data_plus::{FyroxUserData, Traitor, UserDataClass},
};

impl FyroxUserData for fyrox_lite::lite_physics::LiteFeatureKind {
    const CLASS_NAME: &'static str = "FeatureKind";
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Vertex", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_physics::LiteFeatureKind::Vertex,
            ))
        });
        fields.add_field_method_get("Edge", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_physics::LiteFeatureKind::Edge,
            ))
        });
        fields.add_field_method_get("Face", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_physics::LiteFeatureKind::Face,
            ))
        });
        fields.add_field_method_get("Unknown", |lua, this| {
            Ok(Traitor::new(
                fyrox_lite::lite_physics::LiteFeatureKind::Unknown,
            ))
        });
    }

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
    }

    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("Vertex", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureKind::Vertex = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Edge", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureKind::Edge = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Face", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureKind::Face = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
        fields.add_field_method_get("Unknown", |lua, this| {
            let fyrox_lite::lite_physics::LiteFeatureKind::Unknown = this.inner() else {
                return Ok(mlua::Value::Nil);
            };
            Ok(mlua::Value::Boolean(true))
        });
    }
}
