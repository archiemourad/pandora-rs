use pandora::{
    app::App, context::WGPUContextConfiguration, mesh::Mesh, pipeline::PipelineBuilder,
    renderable::Renderable, vertex::Vertex as VertexLayout,
};
use std::{mem, sync::Arc};
use winit::event_loop::ControlFlow;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
}

impl VertexLayout for Vertex {
    fn layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

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

    let triangle_renderable = Arc::new(Renderable::new(
        Arc::new(Mesh::new(
            app.context.device(),
            VERTICES,
            None,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        )),
        pipeline,
    ));

    app.window_mut(window1_id)
        .expect("Failed to get window 1")
        .add_renderable(triangle_renderable.clone());

    app.window_mut(window2_id)
        .expect("Failed to get window 2")
        .add_renderable(triangle_renderable);

    app.run(ControlFlow::Poll, None).expect("Failed to run app");
}
