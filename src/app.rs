use log::{error, info, warn};
use std::{collections::HashMap, sync::Arc};
use winit::{
    dpi::PhysicalSize,
    event::Event,
    event_loop::{EventLoop, EventLoopWindowTarget},
    platform::pump_events::EventLoopExtPumpEvents,
    window::{WindowBuilder, WindowId},
};

use crate::{
    context::{WGPUContext, WGPUContextBuilder},
    error::{AppError, CreateWindowError},
    window::Window,
};

pub struct App<'w> {
    pub context: Arc<WGPUContext>,
    event_loop: EventLoop<()>,
    windows: HashMap<WindowId, Window<'w>>,
}

impl<'w> App<'w> {
    fn format_window_id(id: &WindowId) -> String {
        format!("{:?}", id)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect()
    }

    pub fn new(builder: WGPUContextBuilder) -> Result<Self, AppError> {
        info!("Initializing app...");

        let context = Arc::new(builder.build()?);
        let event_loop = EventLoop::new()?;

        Ok(Self {
            context,
            event_loop,
            windows: HashMap::new(),
        })
    }

    pub fn event_loop(&self) -> &EventLoop<()> {
        &self.event_loop
    }

    pub fn windows(&self) -> &HashMap<WindowId, Window<'w>> {
        &self.windows
    }

    pub fn windows_mut(&mut self) -> &mut HashMap<WindowId, Window<'w>> {
        &mut self.windows
    }

    pub fn window(&self, id: &WindowId) -> Option<&Window<'w>> {
        self.windows.get(&id)
    }

    pub fn window_mut(&mut self, id: &WindowId) -> Option<&mut Window<'w>> {
        self.windows.get_mut(&id)
    }

    pub fn add_window(
        &mut self,
        window: winit::window::Window,
    ) -> Result<WindowId, CreateWindowError> {
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

    pub fn add_window_with_builder(
        &mut self,
        builder: WindowBuilder,
    ) -> Result<WindowId, CreateWindowError> {
        self.add_window(builder.build(&self.event_loop)?)
    }

    pub fn poll_events<F>(&mut self, mut event_handler: F)
    where
        F: FnMut(&Event<()>, &EventLoopWindowTarget<()>, &mut HashMap<WindowId, Window<'w>>),
    {
        self.event_loop.pump_events(None, |event, elwt| {
            event_handler(&event, elwt, &mut self.windows);
        });
    }

    pub fn close_window(windows: &mut HashMap<WindowId, Window<'w>>, id: &WindowId) -> bool {
        info!(
            "Request to close window: {} (id: {})",
            windows
                .get(id)
                .map_or("Unknown".to_string(), |w| format!("'{}'", w.title())),
            Self::format_window_id(id)
        );

        windows.remove(id);

        windows.is_empty()
    }

    pub fn resize_window(
        windows: &mut HashMap<WindowId, Window<'w>>,
        id: &WindowId,
        size: &PhysicalSize<u32>,
    ) {
        if let Some(window) = windows.get_mut(id) {
            window.resize(*size);
        }
    }

    pub fn handle_redraw_error(
        windows: &mut HashMap<WindowId, Window<'w>>,
        id: &WindowId,
        error: wgpu::SurfaceError,
    ) -> bool {
        if let Some(window) = windows.get_mut(id) {
            match error {
                wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated => {
                    warn!(
                        "Surface lost or outdated for window: '{}' (id: {}), resizing...",
                        window.title(),
                        Self::format_window_id(id)
                    );

                    window.resize(window.size());

                    false
                }
                wgpu::SurfaceError::OutOfMemory => {
                    error!(
                        "Out of memory error during render for window: '{}' (id: {})",
                        window.title(),
                        Self::format_window_id(id)
                    );

                    true
                }
                wgpu::SurfaceError::Timeout => {
                    warn!(
                        "Timeout error during render for window: '{}' (id: {})",
                        window.title(),
                        Self::format_window_id(id)
                    );

                    false
                }
            }
        } else {
            warn!(
                "Received redraw error for unknown window (id: {}), ignoring...",
                Self::format_window_id(id)
            );

            false
        }
    }
}
