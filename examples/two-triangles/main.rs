use pandora::{app::App, context::WGPUContextConfiguration};
use std::sync::Arc;
use winit::event_loop::ControlFlow;

fn main() {
    let mut app = App::new(WGPUContextConfiguration::default()).expect("Failed to create app");

    let window1_id = app
        .create_window("Triangle Window 1", 800, 600)
        .expect("Failed to create window 1");
    let window2_id = app
        .create_window("Triangle Window 2", 800, 600)
        .expect("Failed to create window 2");

    let window1 = app.window_mut(window1_id).expect("Failed to get window 1");

    let pipeline =
        Arc::new(window1.create_pipeline(wgpu::include_wgsl!("shader.wgsl"), "vs_main", "fs_main"));

    window1.set_pipeline(pipeline.clone());

    app.window_mut(window2_id)
        .expect("Failed to get window 2")
        .set_pipeline(pipeline.clone());

    app.run(ControlFlow::Poll, None).expect("Failed to run app");
}
