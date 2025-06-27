extern crate lite_macro;
use crate::lite_math::{PodQuaternion, PodVector3};
use std::fmt::Debug;

use fyrox::{
    core::{algebra::UnitQuaternion, pool::Handle, reflect::*, visitor::Visit},
    scene::node::Node,
    script::RoutingStrategy,
};
use lite_macro::lite_api;

use crate::lite_physics::LiteRigidBody;
use fyrox::graph::BaseSceneGraph;
use lite_runtime::externalizable::Externalizable;
use lite_runtime::script_context::with_script_context;
use lite_runtime::spi::{ClassId, UserScript};
use lite_runtime::wrapper_reflect;

#[derive(Clone, Copy, Eq, PartialEq, Default)]
pub struct LiteNode {
    handle: Handle<Node>,
}

impl Debug for LiteNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.handle, f)
    }
}

impl LiteNode {
    pub fn new(handle: Handle<Node>) -> Self {
        Self { handle }
    }

    pub fn inner(&self) -> Handle<Node> {
        self.handle
    }

    fn with_node_mut<T: UserScript, R>(
        &self,
        f: impl FnOnce(&mut Node) -> Result<R, T::LangSpecificError>,
    ) -> Result<R, T::LangSpecificError> {
        with_script_context(|ctx| {
            let Some(scene) = &mut ctx.scene else {
                return Err(T::create_error("scene unavailable"));
            };
            if self.handle.is_none() {
                return Err(T::create_error("this node is null reference"));
            }
            let Some(node) = scene.graph.try_get_mut(self.handle) else {
                return Err(T::create_error(
                    format!("attempt to access detached node {}", self.handle).as_str(),
                ));
            };
            f(node)
        })
    }
}

#[lite_api(class=Node, eq)]
impl LiteNode {
    pub fn as_rigid_body<T: UserScript>(
        &mut self,
        _stub: T::UserScriptGenericStub,
    ) -> Result<Option<LiteRigidBody>, T::LangSpecificError> {
        self.with_node_mut::<T, _>(|node| {
            Ok(if node.is_rigid_body() {
                Some(LiteRigidBody {
                    handle: self.handle,
                })
            } else {
                None
            })
        })
    }

    pub fn get_name<T: UserScript>(
        &self,
        _stub: T::UserScriptGenericStub,
    ) -> Result<String, T::LangSpecificError> {
        with_script_context(|ctx| {
            ctx.scene
                .as_ref()
                .expect("scene unavailable")
                .graph
                .try_get(self.handle)
                .ok_or_else(|| T::create_error("node doesn't exist"))
                .map(|it| it.name_owned())
        })
    }

    pub fn get_alive(&self) -> bool {
        with_script_context(|ctx| {
            ctx.scene
                .as_ref()
                .expect("scene unavailable")
                .graph
                .try_get(self.handle)
                .is_some()
        })
    }

    pub fn destroy(&mut self) {
        with_script_context(|ctx| {
            ctx.scene
                .as_mut()
                .expect("scene unavailable")
                .graph
                .remove_node(self.handle)
        });
    }

    pub fn get_global_position(&self) -> PodVector3 {
        with_script_context(|ctx| {
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .global_position()
                .into()
        })
    }

    pub fn get_local_position(&self) -> PodVector3 {
        with_script_context(|ctx| {
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .local_transform()
                .position()
                .get_value_ref()
                .to_owned()
                .into()
        })
    }

    pub fn get_local_rotation(&self) -> PodQuaternion {
        with_script_context(|ctx| {
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .local_transform()
                .rotation()
                .get_value_ref()
                .to_owned()
                .into()
        })
    }

    pub fn get_local_scale<T: UserScript>(&self, _stub: T::UserScriptGenericStub) -> PodVector3 {
        with_script_context(|ctx| {
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .local_transform()
                .scale()
                .get_value_ref()
                .to_owned()
                .into()
        })
    }

    /// Sends a hierarchical script message with the given payload.
    pub fn send_hierarchical<T: UserScript>(
        &self,
        routing: LiteRoutingStrategy,
        payload: T::UserScriptMessage,
    ) {
        with_script_context(|ctx| {
            let routing = match routing {
                LiteRoutingStrategy::Up => RoutingStrategy::Up,
                LiteRoutingStrategy::Down => RoutingStrategy::Down,
            };
            ctx.message_sender
                .as_mut()
                .expect("message sender unavailable")
                .send_hierarchical(self.handle, routing, payload);
        });
    }

    pub fn set_local_position<T: UserScript>(
        &self,
        new_pos: PodVector3,
        _stub: T::UserScriptGenericStub,
    ) -> Result<(), T::LangSpecificError> {
        self.with_node_mut::<T, _>(move |node| {
            node.local_transform_mut().set_position(new_pos.into());
            Ok(())
        })
    }

    pub fn set_local_rotation<T: UserScript>(
        &self,
        value: PodQuaternion,
        _stub: T::UserScriptGenericStub,
    ) -> Result<(), T::LangSpecificError> {
        self.with_node_mut::<T, _>(|node| {
            node.local_transform_mut().set_rotation(value.into());
            Ok(())
        })
    }

    pub fn set_local_scale<T: UserScript>(
        &self,
        value: PodVector3,
        _stub: T::UserScriptGenericStub,
    ) -> Result<(), T::LangSpecificError> {
        self.with_node_mut::<T, _>(|node| {
            node.local_transform_mut().set_scale(value.into());
            Ok(())
        })
    }

    pub fn subscribe_to<T: UserScript>(
        &self,
        _stub: T::UserScriptGenericStub,
        class_id: T::ClassId,
    ) {
        with_script_context(|ctx| {
            let packed_class_id = T::pack_class_id(&class_id);
            ctx.message_dispatcher.as_mut()
                .expect("cannot subscribe from on_message callback. do it in on_init, on_start or on_update")
                .subscribe_dynamic_to(self.handle, packed_class_id);
        });
    }

    pub fn find_collider_in_children(&self) -> Option<LiteNode> {
        with_script_context(|ctx| {
            ctx.scene.as_ref().expect("scene unavailable").graph[self.handle]
                .children()
                .iter()
                .copied()
                .find(|it| ctx.scene.as_ref().expect("scene unavailable").graph[*it].is_collider())
                .map(LiteNode::new)
        })
    }

    pub fn get_valid(&self) -> bool {
        self.handle.is_some()
    }

    pub fn get_parent(&self) -> LiteNode {
        with_script_context(|ctx| {
            LiteNode::new(
                ctx.scene.as_mut().expect("scene unavailable").graph[self.handle].parent(),
            )
        })
    }

    pub fn add_script<T: UserScript>(
        &self,
        class_id: T::ClassId,
        _stub: T::UserScriptGenericStub,
    ) -> Result<T, T::LangSpecificError> {
        if self.find_script::<T>(class_id.clone(), _stub)?.is_some() {
            return Err(T::create_error(
                format!(
                    "node {:?} already contains script of class {}",
                    self,
                    class_id.lookup_class_name()
                )
                .as_str(),
            ));
        }
        let script = T::new_instance(self.handle, &class_id)?;
        let proxy = T::into_proxy_script(script, &class_id)?;

        with_script_context(|ctx| {
            let node = &mut ctx.scene.as_mut().expect("scene unavailable").graph[self.handle];
            node.add_script(proxy);
        });
        let script = self.find_script(class_id.clone(), _stub)?;
        Ok(script.expect("WTF: it was just added"))
    }

    pub fn find_script<T: UserScript>(
        &self,
        class_id: T::ClassId,
        _stub: T::UserScriptGenericStub,
    ) -> Result<Option<T>, T::LangSpecificError> {
        with_script_context(|ctx| {
            let node = &mut ctx.scene.as_mut().expect("scene unavailable").graph[self.handle];
            for script in node.try_get_scripts_mut::<T::ProxyScript>() {
                let Some(plugin) = &mut ctx.plugins else {
                    return Err(T::create_error(
                        "plugins access not allowed from GlobalScript",
                    ));
                };
                let plugin = plugin
                    .of_type_mut::<T::Plugin>()
                    .expect("WTF: Lua plugin unavailable");
                if let Some(r) = T::extract_from(self.handle, script, &class_id, plugin) {
                    return Ok(Some(r));
                }
            }
            Ok(None)
        })
    }

    pub fn get_global_rotation(&self) -> PodQuaternion {
        with_script_context(|ctx| {
            let camera_global_transform = ctx.scene.as_mut().expect("scene unavailable").graph
                [self.handle]
                .global_transform();

            let rot = camera_global_transform.fixed_view::<3, 3>(0, 0);
            let r = UnitQuaternion::from_matrix(&rot.into());
            PodQuaternion {
                i: r.i,
                j: r.j,
                k: r.k,
                w: r.w,
            }
        })
    }

    pub fn tag_is(&self, tag: String) -> bool {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle].tag() == tag
        })
    }

    pub fn set_tag(&self, tag: String) {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .set_tag(tag.to_string());
        });
    }

    pub fn get_tag(&self) -> String {
        with_script_context(|ctx| {
            ctx.scene.as_mut().expect("scene unavailable").graph[self.handle]
                .tag()
                .to_string()
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[lite_api(class=RoutingStrategy)]
pub enum LiteRoutingStrategy {
    /// An message will be passed to the specified root node and then to every node up in the hierarchy.
    Up,
    /// An message will be passed to every node down the tree in the hierarchy.
    Down,
}

impl Visit for LiteNode {
    fn visit(
        &mut self,
        name: &str,
        visitor: &mut fyrox::core::visitor::Visitor,
    ) -> fyrox::core::visitor::VisitResult {
        self.handle.visit(name, visitor)
    }
}

impl Reflect for LiteNode {
    wrapper_reflect!(handle);
}
impl Externalizable for LiteNode {
    fn to_external(&self) -> u128 {
        self.handle.encode_to_u128()
    }

    fn from_external(v: u128) -> Self {
        Self {
            handle: Handle::decode_from_u128(v),
        }
    }
}
