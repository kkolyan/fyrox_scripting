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

impl FyroxUserData for fyrox_lite::lite_node::LiteNode {
    const CLASS_NAME: &'static str = "Node";

    fn add_instance_methods<'lua, M: mlua::UserDataMethods<'lua, Traitor<Self>>>(methods: &mut M) {
        methods.add_meta_method(mlua::MetaMethod::ToString.name(), |lua, this, args: ()| {
            Ok(format!("{:?}", this.inner()))
        });
        methods.add_meta_method(
            mlua::MetaMethod::Eq.name(),
            |lua, this, args: TypedUserData<Traitor<Self>>| {
                Ok(<Self as PartialEq>::eq(
                    this.inner(),
                    args.borrow()?.inner(),
                ))
            },
        );

        methods.add_method_mut("as_rigid_body", |lua, this, (): ()| {
            let _stub = Default::default();
            let ret = this.as_rigid_body::<TypedUserData<UserScriptProxy>>(_stub);
            let ret = match ret {
                Ok(ret) => {
                    if let Some(ret) = ret {
                        Some(Traitor::new(fyrox_lite::lite_physics::LiteRigidBody::from(
                            ret,
                        )))
                    } else {
                        None
                    }
                }
                Err(err) => return Err(err),
            };
            Ok(ret)
        });
        methods.add_method_mut("destroy", |lua, this, (): ()| {
            let ret = this.destroy();
            let ret = ret;
            Ok(ret)
        });
        methods.add_method_mut(
            "send_hierarchical",
            |lua,
             this,
             (routing, payload_type, payload): (
                TypedUserData<Traitor<fyrox_lite::lite_node::LiteRoutingStrategy>>,
                mlua::Value,
                mlua::Value,
            )| {
                let routing = routing.borrow()?.inner().clone().into();
                let payload = LuaUserScriptMessageEnvelope::new(payload_type, payload)?;
                let ret =
                    this.send_hierarchical::<TypedUserData<UserScriptProxy>>(routing, payload);
                let ret = ret;
                Ok(ret)
            },
        );
        methods.add_method_mut("subscribe_to", |lua, this, (class_id): (mlua::String)| {
            let _stub = Default::default();
            let class_id = class_id.to_str()?.to_string();
            let ret = this.subscribe_to::<TypedUserData<UserScriptProxy>>(_stub, class_id);
            let ret = ret;
            Ok(ret)
        });
        methods.add_method_mut("find_collider_in_children", |lua, this, (): ()| {
            let ret = this.find_collider_in_children();
            let ret = if let Some(ret) = ret {
                Some(Traitor::new(fyrox_lite::lite_node::LiteNode::from(ret)))
            } else {
                None
            };
            Ok(ret)
        });
        methods.add_method_mut("add_script", |lua, this, (class_id): (mlua::String)| {
            let class_id = class_id.to_str()?.to_string();
            let _stub = Default::default();
            let ret = this.add_script::<TypedUserData<UserScriptProxy>>(class_id, _stub);
            let ret = match ret {
                Ok(ret) => ret,
                Err(err) => return Err(err),
            };
            Ok(ret)
        });
        methods.add_method_mut("find_script", |lua, this, (class_id): (mlua::String)| {
            let class_id = class_id.to_str()?.to_string();
            let _stub = Default::default();
            let ret = this.find_script::<TypedUserData<UserScriptProxy>>(class_id, _stub);
            let ret = match ret {
                Ok(ret) => {
                    if let Some(ret) = ret {
                        Some(ret)
                    } else {
                        None
                    }
                }
                Err(err) => return Err(err),
            };
            Ok(ret)
        });
        methods.add_method_mut("tag_is", |lua, this, (tag): (mlua::String)| {
            let tag = tag.to_str()?.to_string();
            let ret = this.tag_is(tag);
            let ret = ret;
            Ok(ret)
        });
    }
    fn add_class_methods<'lua, M: mlua::UserDataMethods<'lua, UserDataClass<Self>>>(
        methods: &mut M,
    ) {
    }
    fn add_instance_fields<'lua, F: mlua::UserDataFields<'lua, Traitor<Self>>>(fields: &mut F) {
        fields.add_field_method_get("name", |lua, this| {
            let value = this.get_name::<TypedUserData<UserScriptProxy>>(());
            Ok(match value {
                Ok(value) => value,
                Err(err) => return Err(err),
            })
        });
        fields.add_field_method_get("alive", |lua, this| {
            let value = this.get_alive();
            Ok(value)
        });
        fields.add_field_method_get("global_position", |lua, this| {
            let value = this.get_global_position();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(
                value,
            )))
        });
        fields.add_field_method_get("local_position", |lua, this| {
            let value = this.get_local_position();
            Ok(Traitor::new(fyrox_lite_math::lite_math::LiteVector3::from(
                value,
            )))
        });
        fields.add_field_method_get("local_rotation", |lua, this| {
            let value = this.get_local_rotation();
            Ok(Traitor::new(
                fyrox_lite_math::lite_math::LiteQuaternion::from(value),
            ))
        });
        fields.add_field_method_get("valid", |lua, this| {
            let value = this.get_valid();
            Ok(value)
        });
        fields.add_field_method_get("parent", |lua, this| {
            let value = this.get_parent();
            Ok(Traitor::new(fyrox_lite::lite_node::LiteNode::from(value)))
        });
        fields.add_field_method_get("global_rotation", |lua, this| {
            let value = this.get_global_rotation();
            Ok(Traitor::new(
                fyrox_lite_math::lite_math::LiteQuaternion::from(value),
            ))
        });
        fields.add_field_method_get("tag", |lua, this| {
            let value = this.get_tag();
            Ok(value)
        });
        fields.add_field_method_set(
            "local_position",
            |lua, this, value: TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteVector3>>| {
                let value = this.set_local_position::<TypedUserData<UserScriptProxy>>(
                    value.borrow()?.inner().clone().into(),
                    (),
                );
                Ok(match value {
                    Ok(value) => value,
                    Err(err) => return Err(err),
                })
            },
        );
        fields.add_field_method_set("local_rotation", |lua, this, value: TypedUserData<Traitor<fyrox_lite_math::lite_math::LiteQuaternion>>| {
                    let value = this.set_local_rotation::<TypedUserData<UserScriptProxy>>(value.borrow()?.inner().clone().into(), ());
                    Ok(match value { Ok(value) => value, Err(err) => return Err(err) })
                });
        fields.add_field_method_set("tag", |lua, this, value: mlua::String| {
            let value = this.set_tag(value.to_str()?.to_string());
            Ok(value)
        });
    }
    fn add_class_fields<'lua, F: mlua::UserDataFields<'lua, UserDataClass<Self>>>(fields: &mut F) {}
}
