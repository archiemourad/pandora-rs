use winit::{dpi::PhysicalSize, event_loop::ControlFlow, window::WindowBuilder};

use pandora::{app::App, context::WGPUContextBuilder};

fn main() {
    env_logger::init();

    let mut app = App::new(WGPUContextBuilder::new()).expect("Failed to create app");

    app.add_window(
        WindowBuilder::new()
            .with_title("Simple Window 1")
            .with_inner_size(PhysicalSize::new(800, 600)),
    )
    .expect("Failed to add window 1");
    app.add_window(
        WindowBuilder::new()
            .with_title("Simple Window 2")
            .with_inner_size(PhysicalSize::new(800, 600)),
    )
    .expect("Failed to add window 2");

    app.run(ControlFlow::Poll, None).expect("Failed to run app");
}
