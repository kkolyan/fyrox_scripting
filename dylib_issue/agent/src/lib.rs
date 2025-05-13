use fyrox::core::log::{Log, MessageKind};
// use fyrox::dpi::LogicalSize;
// use fyrox::engine::executor::Executor;
// use fyrox::engine::GraphicsContextParams;
// use fyrox::event_loop::EventLoop;
// use fyrox::window::WindowAttributes;

#[unsafe(no_mangle)]
pub extern "C" fn hello_dylib_issue() {
    Log::set_verbosity(MessageKind::Error);
    // let mut window_attributes = WindowAttributes::default();
    // window_attributes.inner_size = Some(LogicalSize::new(1280.0, 720.0).into());
    // let mut executor = Executor::from_params(
    //     EventLoop::new().unwrap(),
    //     GraphicsContextParams {
    //         window_attributes,
    //         vsync: false,
    //         msaa_sample_count: None,
    //         graphics_server_constructor: Default::default(),
    //     },
    // );
    println!("hello dylib issue");
}