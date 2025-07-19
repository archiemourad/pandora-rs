use winit::{dpi::PhysicalSize, event_loop::ControlFlow, window::WindowBuilder};

use pandora::{app::App, context::WGPUContextBuilder};

fn main() {
    env_logger::init();

    let mut app = App::new(WGPUContextBuilder::new()).expect("Failed to create app");

    let (width, height) = (800, 600);

    app.add_window_with_builder(
        WindowBuilder::new()
            .with_title("Simple Window 1")
            .with_inner_size(PhysicalSize::new(width, height)),
    )
    .expect("Failed to add window 1");
    app.add_window_with_builder(
        WindowBuilder::new()
            .with_title("Simple Window 2")
            .with_inner_size(PhysicalSize::new(width, height)),
    )
    .expect("Failed to add window 2");

    app.run(ControlFlow::Poll, wgpu::Color::BLACK)
        .expect("Failed to run app");
}
