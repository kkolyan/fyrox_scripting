use lite_macro::lite_api;
use lite_runtime::script_context::with_script_context;
use lite_runtime::spi::UserScript;

#[derive(Debug, Clone)]
pub struct LiteGlobalScript;

#[lite_api(class=GlobalScript)]
impl LiteGlobalScript {
    /// find a global script by type
    pub fn get<T: UserScript>(
        class_id: T::ClassId,
        _stub: T::UserScriptGenericStub,
    ) -> Result<T, T::LangSpecificError> {
        T::find_global_script(&class_id)
    }
}

#[derive(Debug, Clone)]
pub struct LiteTime;

#[lite_api(class=Time)]
impl LiteTime {
    pub fn get_fps() -> f32 {
        with_script_context(|ctx| {
            ctx.graphics_context
                .as_initialized_ref()
                .renderer
                .get_statistics()
                .frames_per_second as f32
        })
    }
}
