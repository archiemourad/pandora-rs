use pandora::{app::App, context::WGPUContextConfiguration, pipeline::PipelineBuilder};
use std::sync::Arc;
use winit::event_loop::ControlFlow;

fn main() {
    let mut app = App::new(WGPUContextConfiguration::default()).expect("Failed to create app");

    let pipeline = Arc::new(
        PipelineBuilder::new(
            app.context.device(),
            wgpu::include_wgsl!("shader.wgsl"),
            "vs_main",
            "fs_main",
            wgpu::TextureFormat::Bgra8UnormSrgb,
        )
        .build(),
    );

    let window1_id = app
        .create_window("Triangle Window 1", 800, 600)
        .expect("Failed to create window 1");
    let window2_id = app
        .create_window("Triangle Window 2", 800, 600)
        .expect("Failed to create window 2");

    app.window_mut(window1_id)
        .expect("Failed to get window 1")
        .set_pipeline(pipeline.clone());

    app.window_mut(window2_id)
        .expect("Failed to get window 2")
        .set_pipeline(pipeline.clone());

    app.run(ControlFlow::Poll, None).expect("Failed to run app");
}
