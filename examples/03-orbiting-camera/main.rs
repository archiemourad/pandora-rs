use std::sync::Arc;
use winit::{dpi::PhysicalSize, event_loop::ControlFlow, window::WindowBuilder};

use pandora::{
    app::App,
    camera::{Camera, CameraBinding},
    context::WGPUContextBuilder,
    drawable::RenderObject,
    material::Material,
    mesh::Mesh,
    pipeline::PipelineBuilder,
    texture::Texture,
    vertex::VertexLayout,
};

mod vertex;
use vertex::Vertex;

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.0868241, 0.49240386, 0.0],
        tex_coords: [0.4131759, 0.00759614],
    },
    Vertex {
        position: [-0.49513406, 0.06958647, 0.0],
        tex_coords: [0.0048659444, 0.43041354],
    },
    Vertex {
        position: [-0.21918549, -0.44939706, 0.0],
        tex_coords: [0.28081453, 0.949397],
    },
    Vertex {
        position: [0.35966998, -0.3473291, 0.0],
        tex_coords: [0.85967, 0.84732914],
    },
    Vertex {
        position: [0.44147372, 0.2347359, 0.0],
        tex_coords: [0.9414737, 0.2652641],
    },
];

const INDICES: &[u32] = &[0, 1, 2, 0, 2, 3, 0, 3, 4];

fn main() {
    env_logger::init();

    let mut app = App::new(WGPUContextBuilder::new()).expect("Failed to create app");

    let mut camera = Camera {
        eye: (0.0, 1.0, 2.0).into(),
        target: (0.0, 0.0, 0.0).into(),
        up: cgmath::Vector3::unit_y(),
        aspect: 800.0 / 600.0,
        fovy: 45.0,
        znear: 0.1,
        zfar: 100.0,
    };

    let mut camera_binding = CameraBinding::new(app.context.device(), &camera);

    let pentagon_texture = Texture::from_bytes(
        app.context.device(),
        app.context.queue(),
        include_bytes!("happy-tree.png"),
    )
    .expect("Failed to load texture");

    let pipeline = Arc::new(
        PipelineBuilder::new(
            app.context.device(),
            wgpu::include_wgsl!("shader.wgsl"),
            "vs_main",
            "fs_main",
            wgpu::TextureFormat::Bgra8UnormSrgb,
        )
        .with_vertex_buffers(vec![Vertex::layout()])
        .with_bind_group_layouts(&[
            &pentagon_texture.bind_group_layout(),
            &camera_binding.bind_group_layout(),
        ])
        .build(),
    );

    let window1_id = app
        .add_window(
            WindowBuilder::new()
                .with_title("Orbit Window 1")
                .with_inner_size(PhysicalSize::new(800, 600)),
        )
        .expect("Failed to add window 1");
    let window2_id = app
        .add_window(
            WindowBuilder::new()
                .with_title("Orbit Window 2")
                .with_inner_size(PhysicalSize::new(800, 600)),
        )
        .expect("Failed to add window 2");

    let pentagon_mesh = Arc::new(Mesh::new(
        app.context.device(),
        VERTICES,
        Some(INDICES),
        wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    ));

    let pentagon_material = Arc::new(Material {
        pipeline: pipeline.clone(),
        diffuse_bind_group: pentagon_texture.bind_group().clone(),
    });

    let pentagon = Arc::new(RenderObject::new(pentagon_mesh, pentagon_material));

    app.window_mut(window1_id)
        .expect("Failed to get window 1")
        .drawables_mut()
        .push(pentagon.clone());

    app.window_mut(window2_id)
        .expect("Failed to get window 2")
        .drawables_mut()
        .push(pentagon);

    app.run_with(ControlFlow::Poll, |windows, window_id| {
        if let Some(window) = windows.get_mut(&window_id) {
            let mut frame = window.frame()?;

            camera.right(0.01);
            camera_binding.update(frame.context.queue(), &camera);

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

                render_pass.set_bind_group(1, &camera_binding.bind_group(), &[]);

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
