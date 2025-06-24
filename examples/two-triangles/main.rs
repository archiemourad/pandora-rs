use std::sync::Arc;
use winit::event_loop::ControlFlow;

use pandora::{
    app::App, context::WGPUContextConfiguration, drawable::Drawable, mesh::Mesh,
    pipeline::PipelineBuilder, vertex::VertexLayout,
};

mod vertex;
use vertex::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5, 0.0],
        color: [0.0, 0.0, 1.0],
    },
];

fn main() {
    env_logger::init();

    let mut app = App::new(WGPUContextConfiguration::default()).expect("Failed to create app");

    let pipeline = Arc::new(
        PipelineBuilder::new(
            app.context.device(),
            wgpu::include_wgsl!("shader.wgsl"),
            "vs_main",
            "fs_main",
            wgpu::TextureFormat::Bgra8UnormSrgb,
        )
        .with_vertex_buffers(vec![Vertex::layout()])
        .build(),
    );

    let window1_id = app
        .create_window("Triangle Window 1", 800, 600)
        .expect("Failed to create window 1");
    let window2_id = app
        .create_window("Triangle Window 2", 800, 600)
        .expect("Failed to create window 2");

    let triangle_mesh = Arc::new(Mesh::new(
        app.context.device(),
        VERTICES,
        None,
        wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    ));

    let triangle = Arc::new(Drawable::new(triangle_mesh, pipeline));

    app.window_mut(window1_id)
        .expect("Failed to get window 1")
        .add_drawable(triangle.clone());

    app.window_mut(window2_id)
        .expect("Failed to get window 2")
        .add_drawable(triangle);

    app.run(ControlFlow::Poll, None).expect("Failed to run app");
}
