use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    window::WindowBuilder,
};

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

    let mut running = true;

    while running {
        app.poll_events(|event, elwt, windows| match event {
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
                WindowEvent::RedrawRequested => {
                    if let Some(window) = windows.get_mut(window_id) {
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

                        {
                            let mut _render_pass =
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
                        }

                        frame.present();
                    }
                }
                _ => {}
            },
            _ => {}
        });
    }
}
