pub struct App<'window> {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub windows: std::collections::HashMap<winit::window::WindowId, crate::window::Window<'window>>,
    pub context: std::sync::Arc<crate::context::WGPUContext>,
}

impl<'window> App<'window> {
    pub fn new(
        control_flow: winit::event_loop::ControlFlow,
    ) -> Result<Self, crate::error::AppError> {
        use crate::context::{WGPUContext, WGPUContextConfiguration};
        use std::sync::Arc;
        use winit::event_loop::EventLoop;

        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(control_flow);

        Ok(Self {
            event_loop,
            windows: std::collections::HashMap::new(),
            context: Arc::new(WGPUContext::new(WGPUContextConfiguration::default())?),
        })
    }

    pub fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<winit::window::WindowId, crate::error::CreateWindowError> {
        use crate::window::Window;
        use std::sync::Arc;
        use winit::window::WindowBuilder;

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(&self.event_loop)?;

        let window_id = window.id();

        self.windows.insert(
            window_id,
            Window::new(self.context.clone(), Arc::new(window))?,
        );

        Ok(window_id)
    }

    pub fn run(mut self) -> Result<(), winit::error::EventLoopError> {
        use winit::event::{Event, WindowEvent};

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
