use log::{debug, error, info, warn};
use std::{collections::HashMap, sync::Arc};
use winit::{
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowId},
};

use crate::{
    context::{WGPUContext, WGPUContextBuilder},
    error::{AppError, CreateWindowError},
    window::Window,
};

pub struct App<'window> {
    pub context: Arc<WGPUContext>,
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Window<'window>>,
}

impl<'window> App<'window> {
    fn format_window_id(id: &WindowId) -> String {
        format!("{:?}", id)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect()
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub fn windows(&self) -> &HashMap<WindowId, Window<'window>> {
        &self.windows
    }

    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut Window<'window>> {
        self.windows.get_mut(&id)
    }

    pub fn add_window(&mut self, builder: WindowBuilder) -> Result<WindowId, CreateWindowError> {
        let window = builder.build(&self.event_loop)?;

        let window_id = window.id();

        let title = window.title();
        let size = window.inner_size();

        self.windows.insert(
            window_id,
            Window::new(self.context.clone(), Arc::new(window))?,
        );

        info!(
            "Created window: '{}' (id: {}, size: {}x{})",
            title,
            Self::format_window_id(&window_id),
            size.width,
            size.height
        );

        Ok(window_id)
    }

    pub fn new(builder: WGPUContextBuilder) -> Result<Self, AppError> {
        info!("Initializing app...");

        Ok(Self {
            context: Arc::new(builder.build()?),
            event_loop: EventLoop::new()?,
            windows: HashMap::new(),
        })
    }

    pub fn run_with<F>(
        mut self,
        control_flow: ControlFlow,
        mut draw: F,
    ) -> Result<(), EventLoopError>
    where
        F: FnMut(
            &mut HashMap<WindowId, Window<'window>>,
            WindowId,
        ) -> Result<(), wgpu::SurfaceError>,
    {
        self.event_loop.set_control_flow(control_flow);

        info!("Starting event loop...");

        self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    info!(
                        "Request to close window: {} (id: {})",
                        self.windows
                            .get(&window_id)
                            .map_or("Unknown".to_string(), |w| format!("'{}'", w.title())),
                            Self::format_window_id(&window_id)
                    );

                    self.windows.remove(&window_id);

                    if self.windows.is_empty() {
                        info!("No more windows open, exiting event loop...");

                        elwt.exit();
                    }
                }
                WindowEvent::Resized(new_size) => {
                    if let Some(window) = self.windows.get_mut(&window_id) {
                        window.resize(new_size);

                        debug!(
                            "Window resized: '{}' (id: {}) to: {}x{}",
                            window.title(),
                            Self::format_window_id(&window_id),
                            new_size.width,
                            new_size.height
                        );
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = self.windows.get_mut(&window_id) {
                        window.window().request_redraw();
                    }

                    if let Err(e) = draw(&mut self.windows, window_id) {
                        if let Some(window) = self.windows.get_mut(&window_id) {
                            match e {
                                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                                    warn!(
                                        "Surface lost or outdated for window: '{}' (id: {}), resizing...",
                                        window.title(),
                                        Self::format_window_id(&window_id)
                                    );
    
                                    window.resize(window.size());
                                }
                                wgpu::SurfaceError::OutOfMemory => {
                                    error!(
                                        "Out of memory error during render for window: '{}' (id: {}), exiting...",
                                        window.title(),
                                        Self::format_window_id(&window_id)
                                    );
    
                                    elwt.exit();
                                }
                                wgpu::SurfaceError::Timeout => warn!(
                                    "Render timeout for window: '{}' (id: {})",
                                    window.title(),
                                    Self::format_window_id(&window_id)
                                    )
                            }
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        })
    }

    pub fn run(self, control_flow: ControlFlow, clear_color: wgpu::Color) -> Result<(), EventLoopError> {
        self.run_with(control_flow, |windows, window_id| {
            if let Some(window) = windows.get_mut(&window_id) {
                let mut frame = window.frame()?;

                {
                    let mut _render_pass =
                    frame
                        .encoder
                        .begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: None,
                            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                view: &frame.view,
                                resolve_target: None,
                                ops: wgpu::Operations {
                                    load: wgpu::LoadOp::Clear(clear_color),
                                    store: wgpu::StoreOp::Store,
                                },
                            })],
                            depth_stencil_attachment: None,
                            occlusion_query_set: None,
                            timestamp_writes: None,
                        });
                }

                frame.present();
            }

            Ok(())
        })
    }
}
