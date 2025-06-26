use winit::event_loop::ControlFlow;

use pandora::{app::App, context::WGPUContextBuilder};

fn main() {
    env_logger::init();

    let mut app = App::new(WGPUContextBuilder::new()).expect("Failed to create app");

    app.create_window("Simple Window 1", 800, 600)
        .expect("Failed to create window 1");
    app.create_window("Simple Window 2", 800, 600)
        .expect("Failed to create window 2");

    app.run(ControlFlow::Poll, None).expect("Failed to run app");
}
