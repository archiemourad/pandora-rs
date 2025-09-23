use log::{error, info, warn};
use std::{collections::HashMap, sync::Arc};
use winit::{
    dpi::PhysicalSize,
    event_loop::EventLoop,
    window::{WindowBuilder, WindowId},
};

use crate::{context::WGPUContext, error::CreateWindowError, window::Window};

pub struct WindowManager<'w> {
    windows: HashMap<WindowId, Window<'w>>,
}

impl<'w> WindowManager<'w> {
    fn format_window_id(id: &WindowId) -> String {
        format!("{:?}", id)
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect()
    }

    pub fn new() -> Self {
        Self {
            windows: HashMap::new(),
        }
    }

    pub fn from(windows: HashMap<WindowId, Window<'w>>) -> Self {
        Self { windows }
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

    pub fn insert_window(
        &mut self,
        context: Arc<WGPUContext>,
        window: winit::window::Window,
    ) -> Result<WindowId, CreateWindowError> {
        let window_id = window.id();

        let title = window.title();
        let size = window.inner_size();

        self.windows
            .insert(window_id, Window::new(context, Arc::new(window))?);

        info!(
            "Created window: '{}' (id: {}, size: {}x{})",
            title,
            Self::format_window_id(&window_id),
            size.width,
            size.height
        );

        Ok(window_id)
    }

    pub fn insert_window_with_builder(
        &mut self,
        context: Arc<WGPUContext>,
        event_loop: &EventLoop<()>,
        builder: WindowBuilder,
    ) -> Result<WindowId, CreateWindowError> {
        let window = builder.build(event_loop)?;

        self.insert_window(context, window)
    }

    pub fn close_window(&mut self, id: &WindowId) -> bool {
        let title = self
            .window(id)
            .map_or("<unknown>".to_string(), |w| format!("'{}'", w.title()));

        info!(
            "Request to close window: {} (id: {})",
            title,
            Self::format_window_id(&id)
        );

        if self.windows.remove(id).is_some() {
            info!(
                "Closed window: {} (id: {})",
                title,
                Self::format_window_id(&id)
            );
        } else {
            warn!("No window found with id: {}", Self::format_window_id(&id));
        }

        self.windows.is_empty()
    }

    pub fn resize_window(&mut self, id: &WindowId, size: PhysicalSize<u32>) {
        if let Some(window) = self.window_mut(id) {
            window.resize(size);
        } else {
            warn!("No window found with id: {}", Self::format_window_id(&id));
        }
    }

    pub fn handle_redraw_error(&mut self, id: &WindowId, error: wgpu::SurfaceError) -> bool {
        if let Some(window) = self.window_mut(id) {
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
            warn!("No window found with id: {}", Self::format_window_id(&id));

            false
        }
    }
}
