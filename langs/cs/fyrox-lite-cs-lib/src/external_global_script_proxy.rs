use crate::auto_dispose::AutoDispose;
use crate::bindings_manual::{NativeClassId, UserScriptMessage};
use crate::c_lang::CCompatibleLang;
use crate::errors::ResultTcrateLangSpecificErrorExt;
use crate::fyrox_c_plugin::CPlugin;
use crate::scripted_app::ScriptedApp;
use crate::scripted_app::APP;
use fyrox::core::reflect::prelude::*;
use fyrox::core::type_traits::prelude::*;
use fyrox::core::visitor::prelude::*;
use fyrox::script::BaseScript;
use fyrox::script::ScriptContext;
use fyrox::script::ScriptTrait;
use fyrox_lite::global_script_object_residence::GlobalScriptResidence;
use fyrox_lite::reflect_base;
use fyrox_lite::script_context::without_script_context;
use fyrox_lite::script_context::UnsafeAsUnifiedContext;
use std::any::Any;
use std::fmt::Debug;

#[derive(Debug, Clone, ComponentProvider)]
pub struct ExternalGlobalScriptProxy {
    pub name: String,
    pub class: NativeClassId,
    pub data: GlobalScriptResidence<CCompatibleLang>,
}

impl ScriptTrait for ExternalGlobalScriptProxy {
    fn on_init(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed);
        invoke_callback(ctx, |app| {
            (app.functions.on_init)(self.data.inner_unpacked().unwrap().instance.inner())
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_start(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed);
        invoke_callback(ctx, |app| {
            (app.functions.on_start)(self.data.inner_unpacked().unwrap().instance.inner())
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_deinit(&mut self, ctx: &mut fyrox::script::ScriptDeinitContext) {
        invoke_callback(ctx, |app| {
            (app.functions.on_deinit)(self.data.inner_unpacked().unwrap().instance.inner())
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_os_event(&mut self, event: &fyrox::event::Event<()>, ctx: &mut ScriptContext) {}

    fn on_update(&mut self, ctx: &mut ScriptContext) {
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed);
        let dt = ctx.dt;
        invoke_callback(ctx, |app| {
            (app.functions.on_update)(self.data.inner_unpacked().unwrap().instance.inner(), dt)
                .into_result()
                .handle_scripting_error();
        });
    }

    fn on_message(
        &mut self,
        message: &mut dyn fyrox::script::ScriptMessagePayload,
        ctx: &mut fyrox::script::ScriptMessageContext,
    ) {
        let Some(message) = message.downcast_ref::<AutoDispose<UserScriptMessage>>() else {
            return;
        };
        self.data
            .ensure_unpacked(&mut ctx.plugins.get_mut::<CPlugin>().failed);
        invoke_callback(ctx, |app| {
            (app.functions.on_message)(
                self.data.inner_unpacked().unwrap().instance.inner(),
                message.inner(),
            )
            .into_result()
            .handle_scripting_error();
        });
    }
}

pub(crate) fn invoke_callback(
    sc: &mut dyn UnsafeAsUnifiedContext<'_, '_, '_>,
    callback: impl FnOnce(&ScriptedApp),
) {
    APP.with_borrow(|app| {
        without_script_context(sc, || {
            callback(app.as_ref().unwrap());
        });
    });
}

impl BaseScript for ExternalGlobalScriptProxy {
    fn clone_box(&self) -> Box<dyn ScriptTrait> {
        Box::new(self.clone())
    }
    fn as_any_ref(&self) -> &dyn Any {
        self
    }
    fn as_any_ref_mut(&mut self) -> &mut dyn Any {
        self
    }
    fn id(&self) -> Uuid {
        self.data.id()
    }
}

impl Visit for ExternalGlobalScriptProxy {
    fn visit(&mut self, name: &str, visitor: &mut Visitor) -> VisitResult {
        self.data.visit(name, visitor)
    }
}

impl Reflect for ExternalGlobalScriptProxy {
    reflect_base!();

    fyrox_lite::reflect_base_lite!();

    fn fields_ref(&self, func: &mut dyn FnMut(&[FieldRef])) {
        self.data.with_script_object(|it| it.fields_ref(func))
    }

    fn fields_mut(&mut self, func: &mut dyn FnMut(&mut [FieldMut])) {
        self.data.with_script_object_mut(|it| it.fields_mut(func))
    }

    fn field(&self, name: &str, func: &mut dyn FnMut(Option<&dyn Reflect>)) {
        self.data.with_script_object(|it| it.field(name, func))
    }

    fn field_mut(&mut self, name: &str, func: &mut dyn FnMut(Option<&mut dyn Reflect>)) {
        self.data
            .with_script_object_mut(|it| it.field_mut(name, func))
    }
}
