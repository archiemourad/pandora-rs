use cgmath::{Deg, Quaternion, Rad, Rotation3, Vector3};
use std::{
    collections::{HashMap, HashSet},
    f32::consts::FRAC_PI_2,
    sync::Arc,
};
use winit::{
    dpi::PhysicalSize,
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, NamedKey},
    platform::modifier_supplement::KeyEventExtModifierSupplement,
    window::{CursorGrabMode, WindowBuilder},
};

use pandora::{
    camera::{Camera, GPUCamera, Projection},
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

struct WindowState {
    camera: Camera,
    gpu_camera: GPUCamera,
    keys_pressed: HashSet<Key>,
    mouse_captured: bool,
}

fn main() {
    env_logger::init();

    let context = Arc::new(
        WGPUContextBuilder::new()
            .build()
            .expect("Failed to create WGPU context"),
    );

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut windows = WindowManager::new();

    let (width, height) = (800, 600);

    let projection = Projection::new(width, height, Deg(45.0), 0.1, 100.0);
    let camera = Camera::new((0.0, 0.0, 2.0), Deg(-90.0), Deg(0.0));
    let gpu_camera = GPUCamera::new(context.device(), &camera, &projection);

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
        .with_bind_group_layouts(&[
            &texture.bind_group_layout(),
            &gpu_camera.bind_group_layout(),
        ])
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

    let mut states = HashMap::new();

    for i in 0..2 {
        let window_id = windows
            .insert_window_with_builder(
                context.clone(),
                &event_loop,
                WindowBuilder::new()
                    .with_title(format!("Window {}", i + 1))
                    .with_inner_size(PhysicalSize::new(width, height)),
            )
            .expect(format!("Failed to create window {}", i + 1).as_str());

        let camera = camera.clone();

        states.insert(
            window_id,
            WindowState {
                camera,
                gpu_camera: GPUCamera::new(context.device(), &camera, &projection),
                keys_pressed: HashSet::new(),
                mouse_captured: false,
            },
        );
    }

    event_loop.set_control_flow(ControlFlow::Poll);

    event_loop
        .run(move |event, elwt| match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta: (dx, dy) },
                ..
            } => {
                for state in states.values_mut() {
                    if state.mouse_captured {
                        let sensitivity = 0.001;

                        state.camera.yaw += Rad(dx as f32 * sensitivity);
                        state.camera.pitch -= Rad(dy as f32 * sensitivity);

                        let pitch_limit = Rad(FRAC_PI_2 - 0.01);

                        if state.camera.pitch > pitch_limit {
                            state.camera.pitch = pitch_limit;
                        } else if state.camera.pitch < -pitch_limit {
                            state.camera.pitch = -pitch_limit;
                        }
                    }
                }
            }
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::CloseRequested => {
                    if windows.close_window(&window_id) {
                        elwt.exit();
                    }
                }
                WindowEvent::Resized(size) => {
                    windows.resize_window(&window_id, size);
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if let Some(state) = states.get_mut(&window_id) {
                        match event.state {
                            ElementState::Pressed => {
                                state.keys_pressed.insert(event.key_without_modifiers());
                            }
                            ElementState::Released => {
                                state.keys_pressed.remove(&event.key_without_modifiers());
                            }
                        }
                    }
                }
                WindowEvent::MouseInput {
                    state: button_state,
                    button,
                    ..
                } => {
                    if let Some(window) = windows.window_mut(&window_id).map(|w| w.window()) {
                        if let Some(state) = states.get_mut(&window_id) {
                            match button {
                                MouseButton::Right => match button_state {
                                    ElementState::Pressed => {
                                        state.mouse_captured = window
                                            .set_cursor_grab(CursorGrabMode::Confined)
                                            .or_else(|_| {
                                                window.set_cursor_grab(CursorGrabMode::Locked)
                                            })
                                            .map(|_| window.set_cursor_visible(false))
                                            .is_ok();
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
                    if let Some(window) = windows.window_mut(&window_id) {
                        if let Some(state) = states.get_mut(&window_id) {
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

                            for (key, direction) in [
                                (Key::Character("w".into()), state.camera.forward()),
                                (Key::Character("s".into()), -state.camera.forward()),
                                (Key::Character("a".into()), -state.camera.right()),
                                (Key::Character("d".into()), state.camera.right()),
                                (Key::Named(NamedKey::Space), Vector3::unit_y()),
                                (Key::Named(NamedKey::Shift), -Vector3::unit_y()),
                            ] {
                                if state.keys_pressed.contains(&key) {
                                    state.camera.position += direction * 0.01;
                                }
                            }

                            state
                                .gpu_camera
                                .update(context.queue(), &state.camera, &projection);

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

                                instance_group.draw(&mut render_pass, &shape);
                            }

                            frame.present();
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        })
        .expect("Failed to run event loop");
}
