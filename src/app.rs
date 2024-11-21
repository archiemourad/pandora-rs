use crate::{
    context::{WGPUContext, WGPUContextConfiguration},
    error::{AppError, CreateWindowError},
    window::Window,
};
use std::{collections::HashMap, sync::Arc};
use winit::{
    dpi::PhysicalSize,
    error::EventLoopError,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, WindowId},
};

pub struct App<'window> {
    pub event_loop: EventLoop<()>,
    pub windows: HashMap<WindowId, Window<'window>>,
    pub context: Arc<WGPUContext>,
}

impl<'window> App<'window> {
    pub fn new(control_flow: ControlFlow) -> Result<Self, AppError> {
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(control_flow);

        Ok(Self {
            event_loop,
            windows: HashMap::new(),
            context: Arc::new(WGPUContext::new(WGPUContextConfiguration::default())?),
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

        Ok(window_id)
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
        self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent { event, window_id } => match event {
                WindowEvent::CloseRequested => {
                    self.windows.remove(&window_id);

                    if self.windows.is_empty() {
                        elwt.exit();
                    }
                }
                WindowEvent::Resized(new_size) => {
                    if let Some(window) = self.windows.get_mut(&window_id) {
                        window.resize(new_size);
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Some(window) = self.windows.get_mut(&window_id) {
                        window.window().request_redraw();

                        match window.render() {
                            Ok(_) => {}

                            Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                                window.resize(window.size)
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => elwt.exit(),
                            Err(wgpu::SurfaceError::Timeout) => todo!(),
                        }
                    }
                }
                _ => (),
            },
            _ => (),
        })
    }
}
