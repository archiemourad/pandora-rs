use cgmath::{Deg, Rad, Vector3};
use std::{
    collections::{HashMap, HashSet},
    f32::consts::FRAC_PI_2,
    sync::Arc,
};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{CursorGrabMode, WindowBuilder, WindowId},
};

use pandora::{
    app::App,
    camera::{Camera, GPUCamera, Projection},
    context::WGPUContextBuilder,
    drawable::Model,
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

struct WindowState {
    camera: Camera,
    gpu_camera: GPUCamera,
    pressed_keys: HashSet<Key>,
    mouse_captured: bool,
}

fn main() {
    env_logger::init();

    let mut app = App::new(WGPUContextBuilder::new()).expect("Failed to create app");

    let (width, height) = (800, 600);

    let projection = Projection::new(width, height, Deg(45.0), 0.1, 100.0);

    let camera = Camera::new((0.0, 0.0, 2.0), Deg(-90.0), Deg(0.0));
    let gpu_camera = GPUCamera::new(app.context.device(), &camera, &projection);

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
            &gpu_camera.bind_group_layout(),
        ])
        .build(),
    );

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

    let pentagon = Arc::new(Model::new(pentagon_mesh, pentagon_material));

    let mut window_states: HashMap<WindowId, WindowState> = HashMap::new();

    for i in 0..2 {
        let window_id = app
            .add_window_with_builder(
                WindowBuilder::new()
                    .with_title(format!("FPS Camera Window {}", i + 1))
                    .with_inner_size(PhysicalSize::new(width, height)),
            )
            .expect(format!("Failed to add window {}", i + 1).as_str());

        app.window_mut(&window_id)
            .expect(format!("Failed to get window {}", i + 1).as_str())
            .drawables_mut()
            .push(pentagon.clone());

        let camera_clone = camera.clone();

        window_states.insert(
            window_id,
            WindowState {
                camera: camera_clone,
                gpu_camera: GPUCamera::new(app.context.device(), &camera_clone, &projection),
                pressed_keys: HashSet::new(),
                mouse_captured: false,
            },
        );
    }

    let mut running = true;

    while running {
        app.poll_events(|event, elwt, windows| match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                ..
            } => {
                for state in window_states.values_mut() {
                    if state.mouse_captured {
                        let sensitivity = 0.001;

                        state.camera.yaw += Rad(*dx as f32 * sensitivity);
                        state.camera.pitch -= Rad(*dy as f32 * sensitivity);

                        let pitch_limit = Rad(FRAC_PI_2 - 0.01);

                        if state.camera.pitch > pitch_limit {
                            state.camera.pitch = pitch_limit;
                        } else if state.camera.pitch < -pitch_limit {
                            state.camera.pitch = -pitch_limit;
                        }
                    }
                }
            }
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    if App::close_window(windows, window_id) {
                        elwt.exit();

                        running = false;
                    }
                }
                WindowEvent::Resized(size) => {
                    App::resize_window(windows, window_id, size);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let Some(state) = window_states.get_mut(window_id) {
                        match event.state {
                            ElementState::Pressed => {
                                state.pressed_keys.insert(event.key_without_modifiers());
                            }
                            ElementState::Released => {
                                state.pressed_keys.remove(&event.key_without_modifiers());
                            }
                        }
                    }
                }
                WindowEvent::MouseInput {
                    state: button_state,
                    button,
                    ..
                } => {
                    if let Some(window) = windows.get_mut(window_id).map(|w| w.window()) {
                        if let Some(state) = window_states.get_mut(window_id) {
                            match button {
                                MouseButton::Right => match button_state {
                                    ElementState::Pressed => {
                                        window
                                            .set_cursor_grab(CursorGrabMode::Locked)
                                            .or_else(|_| {
                                                window.set_cursor_grab(CursorGrabMode::Confined)
                                            })
                                            .ok();
                                        window.set_cursor_visible(false);

                                        state.mouse_captured = true;
                                    }
                                    ElementState::Released => {
                                        window.set_cursor_grab(CursorGrabMode::None).ok();
                                        window.set_cursor_visible(true);

                                        state.mouse_captured = false;
                                    }
                                },
                                _ => {}
                            }
                        }
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = windows.get_mut(window_id) {
                        if let Some(state) = window_states.get_mut(window_id) {
                            window.window().request_redraw();

                            let mut frame = match window.frame() {
                                Ok(frame) => frame,
                                Err(e) => {
                                    if App::handle_redraw_error(windows, window_id, e) {
                                        elwt.exit();

                                        running = false;
                                    }

                                    return;
                                }
                            };

                            let movements: Vec<(Key, Vector3<f32>)> = vec![
                                (Key::Character("w".into()), state.camera.forward()),
                                (Key::Character("s".into()), -state.camera.forward()),
                                (Key::Character("a".into()), -state.camera.right()),
                                (Key::Character("d".into()), state.camera.right()),
                                (Key::Named(NamedKey::Space), state.camera.up()),
                                (Key::Named(NamedKey::Shift), -state.camera.up()),
                            ];

                            for (key, direction) in movements {
                                if state.pressed_keys.contains(&key) {
                                    state.camera.position += direction * 0.01;
                                }
                            }

                            state.gpu_camera.update(
                                frame.context.queue(),
                                &state.camera,
                                &projection,
                            );

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
                                                        load: wgpu::LoadOp::Clear(
                                                            wgpu::Color::BLACK,
                                                        ),
                                                        store: wgpu::StoreOp::Store,
                                                    },
                                                },
                                            )],
                                            depth_stencil_attachment: None,
                                            occlusion_query_set: None,
                                            timestamp_writes: None,
                                        });

                                render_pass.set_bind_group(1, &state.gpu_camera.bind_group(), &[]);

                                for drawable in window.drawables() {
                                    drawable.draw(&mut render_pass);
                                }
                            }

                            frame.present();
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        });
    }
}
