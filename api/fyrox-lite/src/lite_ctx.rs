use crate::lite_node::LiteNode;
use fyrox::{
    core::pool::Handle,
    event::Event,
    plugin::Plugin,
    scene::node::Node,
    script::{ScriptContext, ScriptMessageContext, ScriptMessagePayload},
};
use lite_runtime::script_context::{with_script_context, without_script_context};

pub trait LiteScript {
    fn on_init(&mut self, #[allow(unused_variables)] ctx: &mut LiteContext) {}
    fn on_start(&mut self, #[allow(unused_variables)] ctx: &mut LiteContext) {}
    fn on_update(&mut self, #[allow(unused_variables)] ctx: &mut LiteContext) {}

    fn on_message(
        &mut self,
        #[allow(unused_variables)] message: &mut dyn fyrox::script::ScriptMessagePayload,
        #[allow(unused_variables)] ctx: &mut LiteContext,
    ) {
    }

    fn on_os_event(
        &mut self,
        #[allow(unused_variables)] event: &Event<()>,
        #[allow(unused_variables)] ctx: &mut LiteContext,
    ) {
    }

    fn redispatch_init(&mut self, ctx: &mut ScriptContext) {
        let mut lite_ctx = LiteContext::new(ctx.handle, ctx.dt);

        without_script_context(ctx, || {
            self.on_init(&mut lite_ctx);
        });
    }

    fn redispatch_start(&mut self, ctx: &mut ScriptContext) {
        let mut lite_ctx = LiteContext::new(ctx.handle, ctx.dt);

        without_script_context(ctx, || {
            self.on_start(&mut lite_ctx);
        });
    }

    fn redispatch_update(&mut self, ctx: &mut ScriptContext) {
        let mut lite_ctx = LiteContext::new(ctx.handle, ctx.dt);

        without_script_context(ctx, || {
            self.on_update(&mut lite_ctx);
        });
    }

    fn redispatch_message(
        &mut self,
        message: &mut dyn ScriptMessagePayload,
        ctx: &mut ScriptMessageContext,
    ) {
        let mut lite_ctx = LiteContext::new(ctx.handle, ctx.dt);

        without_script_context(ctx, || {
            self.on_message(message, &mut lite_ctx);
        });
    }

    fn redispatch_os_event(&mut self, event: &Event<()>, ctx: &mut ScriptContext) {
        let mut lite_ctx = LiteContext::new(ctx.handle, ctx.dt);

        without_script_context(ctx, || {
            self.on_os_event(event, &mut lite_ctx);
        });
    }
}

#[derive(Debug)]
pub struct LiteContext {
    pub node: LiteNode,
    pub dt: f32,
}

impl LiteContext {
    pub fn new(handle: Handle<Node>, dt: f32) -> Self {
        Self {
            node: LiteNode::new(handle),
            dt,
        }
    }
    // TODO contribute "take" method to "ctx.plugins"
    pub fn with_plugin<T: Plugin, R>(&mut self, f: impl FnOnce(&mut T) -> R) -> R {
        with_script_context(|ctx| {
            f(ctx
                .plugins
                .as_mut()
                .expect("plugins not available")
                .get_mut::<T>())
        })
    }
}
