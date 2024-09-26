pub struct App {
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub windows: std::collections::HashMap<winit::window::WindowId, winit::window::Window>,
    pub context: crate::context::WGPUContext,
}

impl App {
    pub fn new(
        control_flow: winit::event_loop::ControlFlow,
    ) -> Result<Self, crate::error::AppError> {
        use crate::context::{WGPUContext, WGPUContextConfiguration};
        use winit::event_loop::EventLoop;

        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(control_flow);

        Ok(Self {
            event_loop,
            windows: std::collections::HashMap::new(),
            context: WGPUContext::new(WGPUContextConfiguration::default())?,
        })
    }

    pub fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<winit::window::WindowId, winit::error::OsError> {
        use winit::window::WindowBuilder;

        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(&self.event_loop)?;

        let window_id = window.id();

        self.windows.insert(window_id, window);

        Ok(window_id)
    }

    pub fn run(mut self) -> Result<(), winit::error::EventLoopError> {
        use winit::event::{Event, WindowEvent};

        self.event_loop.run(move |event, elwt| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } => {
                self.windows.remove(&window_id);

                if self.windows.is_empty() {
                    elwt.exit();
                }
            }
            _ => (),
        })
    }
}
