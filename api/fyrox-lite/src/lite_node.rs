use crate::externalizable::Externalizable;
use crate::lite_math::{PodQuaternion, PodVector3};
use crate::spi::{ClassId, UserScript};
use std::fmt::Debug;

extern crate lite_macro;

use fyrox::{
    core::{algebra::UnitQuaternion, pool::Handle, reflect::*, visitor::Visit},
    scene::node::Node,
    script::RoutingStrategy,
};
use lite_macro::lite_api;

use crate::{lite_physics::LiteRigidBody, script_context::with_script_context};
use fyrox::graph::BaseSceneGraph;

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

macro_rules! extract_node {
    ($ctx:expr,$self:expr) => {{
        let Some(scene) = &mut $ctx.scene else {
            return Err(T::create_error("scene unavailable"));
        };
        let Some(node) = scene.graph.try_get($self.handle) else {
            return Err(T::create_error(
                format!("attempt to access detached node {}", $self.handle).as_str(),
            ));
        };
        node
    }};
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
            for x in node.scripts() {}
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
    crate::wrapper_reflect!(handle);
}

#[macro_export]
macro_rules! reflect_base_lite {
    () => {
        fn query_derived_types(&self) -> &'static [core::any::TypeId] {
            Self::derived_types()
        }

        fn derived_types() -> &'static [core::any::TypeId] {
            &[]
        }

        fn try_clone_box(&self) -> Option<Box<dyn Reflect>> {
            Some(Box::new(self.clone()))
        }
    };
}
#[macro_export]
macro_rules! wrapper_reflect {
    ($ident:tt) => {
        fn source_path() -> &'static str
        where
            Self: Sized,
        {
            file!()
        }

        fn assembly_name(&self) -> &'static str {
            env!("CARGO_PKG_NAME")
        }

        fn type_assembly_name() -> &'static str
        where
            Self: Sized,
        {
            env!("CARGO_PKG_NAME")
        }

        fn type_name(&self) -> &'static str {
            Reflect::type_name(&self.$ident)
        }

        fn doc(&self) -> &'static str {
            self.$ident.doc()
        }

        fn fields_ref(&self, func: &mut dyn FnMut(&[FieldRef])) {
            self.$ident.fields_ref(func)
        }

        fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [FieldMut])) {
            self.$ident.fields_mut(func)
        }

        fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }

        fn as_any(&self, func: &mut dyn FnMut(&dyn std::any::Any)) {
            fyrox::core::reflect::Reflect::as_any(&self.$ident, func)
        }

        fn as_any_mut(&mut self, func: &mut dyn FnMut(&mut dyn std::any::Any)) {
            fyrox::core::reflect::Reflect::as_any_mut(&mut self.$ident, func)
        }

        fn as_reflect(&self, func: &mut dyn FnMut(&dyn Reflect)) {
            self.$ident.as_reflect(func)
        }

        fn as_reflect_mut(&mut self, func: &mut dyn FnMut(&mut dyn Reflect)) {
            self.$ident.as_reflect_mut(func)
        }

        fn set(&mut self, value: Box<dyn Reflect>) -> Result<Box<dyn Reflect>, Box<dyn Reflect>> {
            self.$ident.set(value)
        }

        fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
            self.$ident.field(name, func)
        }

        fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
            self.$ident.field_mut(name, func)
        }

        fn as_array(&self, func: &mut dyn FnMut(Option<&dyn ReflectArray>)) {
            self.$ident.as_array(func)
        }

        fn as_array_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectArray>)) {
            self.$ident.as_array_mut(func)
        }

        fn as_list(&self, func: &mut dyn FnMut(Option<&dyn ReflectList>)) {
            self.$ident.as_list(func)
        }

        fn as_list_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectList>)) {
            self.$ident.as_list_mut(func)
        }

        fn as_inheritable_variable(
            &self,
            func: &mut dyn FnMut(Option<&dyn ReflectInheritableVariable>),
        ) {
            self.$ident.as_inheritable_variable(func)
        }

        fn as_inheritable_variable_mut(
            &mut self,
            func: &mut dyn FnMut(Option<&mut dyn ReflectInheritableVariable>),
        ) {
            self.$ident.as_inheritable_variable_mut(func)
        }

        fn as_hash_map(&self, func: &mut dyn FnMut(Option<&dyn ReflectHashMap>)) {
            self.$ident.as_hash_map(func)
        }

        fn as_hash_map_mut(&mut self, func: &mut dyn FnMut(Option<&mut dyn ReflectHashMap>)) {
            self.$ident.as_hash_map_mut(func)
        }

        fn query_derived_types(&self) -> &'static [core::any::TypeId] {
            self.$ident.query_derived_types()
        }

        fn derived_types() -> &'static [core::any::TypeId] {
            &[]
        }

        fn try_clone_box(&self) -> Option<Box<dyn Reflect>> {
            Some(Box::new(self.clone()))
        }
    };
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
