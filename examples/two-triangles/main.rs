use std::sync::Arc;
use winit::{dpi::PhysicalSize, event_loop::ControlFlow, window::WindowBuilder};

use pandora::{
    app::App, context::WGPUContextBuilder, drawable::Primitive, mesh::Mesh,
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

    let mut app = App::new(WGPUContextBuilder::new()).expect("Failed to create app");

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
        .add_window(
            WindowBuilder::new()
                .with_title("Triangle Window 1")
                .with_inner_size(PhysicalSize::new(800, 600)),
        )
        .expect("Failed to add window 1");
    let window2_id = app
        .add_window(
            WindowBuilder::new()
                .with_title("Triangle Window 2")
                .with_inner_size(PhysicalSize::new(800, 600)),
        )
        .expect("Failed to add window 2");

    let triangle_mesh = Arc::new(Mesh::new(
        app.context.device(),
        VERTICES,
        None,
        wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
    ));

    let triangle = Arc::new(Primitive::new(triangle_mesh, pipeline));

    app.window_mut(window1_id)
        .expect("Failed to get window 1")
        .drawables_mut()
        .push(triangle.clone());

    app.window_mut(window2_id)
        .expect("Failed to get window 2")
        .drawables_mut()
        .push(triangle);

    app.run_with(ControlFlow::Poll, |windows, window_id| {
        if let Some(window) = windows.get_mut(&window_id) {
            let mut frame = window.frame()?;

            {
                let mut render_pass =
                    frame
                        .encoder
                        .begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            occlusion_query_set: None,
                            timestamp_writes: None,
                        });

                for drawable in window.drawables() {
                    drawable.draw(&mut render_pass);
                }
            }

            frame.present();
        }

        Ok(())
    })
    .expect("Failed to run app");
}
