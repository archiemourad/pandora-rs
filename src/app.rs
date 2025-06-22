use crate::{
    context::{WGPUContext, WGPUContextConfiguration},
    error::{AppError, CreateWindowError},
    window::Window,
};
use log::{debug, error, info, warn};
use std::{collections::HashMap, sync::Arc};
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowId},
};

pub struct App<'window> {
    pub context: Arc<WGPUContext>,
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Window<'window>>,
}

impl<'window> App<'window> {
    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub fn windows(&self) -> &HashMap<WindowId, Window<'window>> {
        &self.windows
    }

    pub fn window_mut(&mut self, id: WindowId) -> Option<&mut Window<'window>> {
        self.windows.get_mut(&id)
    }

    pub fn new(config: WGPUContextConfiguration) -> Result<Self, AppError> {
        info!("Initializing app...");

        Ok(Self {
            context: Arc::new(WGPUContext::new(config)?),
            event_loop: EventLoop::new()?,
            windows: HashMap::new(),
        })
    }

    pub fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<WindowId, CreateWindowError> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(PhysicalSize::new(width, height))
            .build(&self.event_loop)?;

        let window_id = window.id();

        self.windows.insert(
            window_id,
            Window::new(self.context.clone(), Arc::new(window))?,
        );

        info!(
            "Created window: '{}' (id: {}, size: {}x{})",
            title,
            format!("{:?}", window_id)
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>(),
            width,
            height
        );

        Ok(window_id)
    }

    pub fn run(
        mut self,
        control_flow: ControlFlow,
        clear_color: Option<wgpu::Color>,
    ) -> Result<(), EventLoopError> {
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
                        format!("{:?}", window_id)
                            .chars()
                            .filter(|c| c.is_ascii_digit())
                            .collect::<String>()
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
                            format!("{:?}", window_id)
                                .chars()
                                .filter(|c| c.is_ascii_digit())
                                .collect::<String>(),
                            new_size.width,
                            new_size.height
                        );
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = self.windows.get_mut(&window_id) {
                        window.window().request_redraw();

                        match window.render(clear_color) {
                            Ok(_) => {}

                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                warn!(
                                    "Surface lost or outdated for window: '{}' (id: {}), resizing...",
                                    window.title(),
                                    format!("{:?}", window_id)
                                .chars()
                                .filter(|c| c.is_ascii_digit())
                                .collect::<String>()
                                );

                                window.resize(window.size());
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                error!(
                                    "Out of memory error during render for window: '{}' (id: {}), exiting...",
                                    window.title(),
                                    format!("{:?}", window_id)
                                .chars()
                                .filter(|c| c.is_ascii_digit())
                                .collect::<String>()
                                );

                                elwt.exit();
                            }
                            Err(wgpu::SurfaceError::Timeout) => warn!(
                                "Render timeout for window: '{}' (id: {})",
                                window.title(),
                                format!("{:?}", window_id)
                                .chars()
                                .filter(|c| c.is_ascii_digit())
                                .collect::<String>()
                                )
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        })
    }
}
