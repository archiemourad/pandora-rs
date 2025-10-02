use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use std::sync::Arc;
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use pandora::{
    context::WGPUContextBuilder,
    instance_group::InstanceGroup,
    pipeline::PipelineBuilder,
    texture::Texture,
    transform::{GPUTransform, Transform},
    vertex::VertexLayout,
    window_manager::WindowManager,
};

mod shape;
mod vertex;

use crate::{shape::Shape, vertex::Vertex};

fn main() {
    env_logger::init();

    let context = Arc::new(
        WGPUContextBuilder::new()
            .build()
            .expect("Failed to create WGPU context"),
    );

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut windows = WindowManager::new();

    let texture = Texture::from_bytes(
        context.device(),
        context.queue(),
        include_bytes!("happy-tree.png"),
    )
    .expect("Failed to load texture");

    let pipeline = Arc::new(
        PipelineBuilder::new(
            context.device(),
            wgpu::include_wgsl!("shader.wgsl"),
            "vs_main",
            "fs_main",
            wgpu::TextureFormat::Bgra8UnormSrgb,
        )
        .with_vertex_buffers(vec![Vertex::layout(), GPUTransform::layout()])
        .with_bind_group_layouts(&[&texture.bind_group_layout()])
        .build(),
    );

    let shape = Shape::pentagon().build(
        context.device(),
        pipeline.clone(),
        texture.bind_group().clone(),
    );

    let transforms = vec![Transform {
        position: Vector3::new(0.0, 0.0, 0.0),
        rotation: Quaternion::from_angle_y(Deg(0.0)),
    }];

    let instance_group = InstanceGroup::new(context.device(), &transforms);

    let (width, height) = (800, 600);

    for i in 0..2 {
        windows
            .insert_window_with_builder(
                context.clone(),
                &event_loop,
                WindowBuilder::new()
                    .with_title(format!("Window {}", i + 1))
                    .with_inner_size(PhysicalSize::new(width, height)),
            )
            .expect(format!("Failed to create window {}", i + 1).as_str());
    }

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    if windows.close_window(&window_id) {
                        elwt.exit();
                    }
                }
                WindowEvent::Resized(size) => {
                    windows.resize_window(&window_id, size);
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = windows.window_mut(&window_id) {
                        window.window().request_redraw();

                        let mut frame = match window.frame() {
                            Ok(frame) => frame,
                            Err(e) => {
                                if windows.handle_redraw_error(&window_id, e) {
                                    elwt.exit();
                                }

                                return;
                            }
                        };

                        {
                            let mut render_pass =
                                frame
                                    .encoder
                                    .begin_render_pass(&wgpu::RenderPassDescriptor {
                                        label: None,
                                        color_attachments: &[Some(
                                            wgpu::RenderPassColorAttachment {
                                                view: &frame.view,
                                                resolve_target: None,
                                                ops: wgpu::Operations {
                                                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                                    store: wgpu::StoreOp::Store,
                                                },
                                            },
                                        )],
                                        depth_stencil_attachment: None,
                                        occlusion_query_set: None,
                                        timestamp_writes: None,
                                    });

                            instance_group.draw(&mut render_pass, &shape);
                        }

                        frame.present();
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .expect("Failed to run event loop");
}
