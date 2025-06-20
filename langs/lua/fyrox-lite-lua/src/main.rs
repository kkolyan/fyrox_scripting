//! Executor with your game connected to it as a plugin.
use fyrox::core::log::Log;
use fyrox::core::log::MessageKind;
use fyrox::dpi::LogicalSize;
use fyrox::engine::executor::Executor;
use fyrox::engine::GraphicsContextParams;
use fyrox::event_loop::EventLoop;
use fyrox::window::WindowAttributes;
use lite_runtime::script_failure::ScriptFailureHandler;

fn main() {
    #[cfg(feature = "profiling")]
    {
        let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
        let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
        eprintln!("Run this to view profiling data:  puffin_viewer {server_addr}");
        puffin::set_scopes_on(true);
        std::thread::spawn(|| {
            std::process::Command::new("puffin_viewer")
                .args(["--url", "127.0.0.1:8585"])
                .output()
                .unwrap();
        });
    }

    Log::set_verbosity(MessageKind::Error);
    let mut window_attributes = WindowAttributes::default();
    window_attributes.inner_size = Some(LogicalSize::new(1280.0, 720.0).into());
    let mut executor = Executor::from_params(
        Some(EventLoop::new().unwrap()),
        GraphicsContextParams {
            window_attributes,
            vsync: false,
            msaa_sample_count: None,
            graphics_server_constructor: Default::default(),
        },
    );

    executor.add_dynamic_plugin_custom(fyrox_lite_lua_lib::LuaPlugin::new(
        "scripts".into(),
        true,
        ScriptFailureHandler::new_for_game(),
    ));

    executor.run()
}
