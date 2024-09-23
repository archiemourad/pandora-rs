use winit::{
    error::{EventLoopError, OsError},
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder, WindowId},
};

pub struct App {
    pub event_loop: EventLoop<()>,
    pub windows: std::collections::HashMap<WindowId, Window>,
    pub context: crate::context::WGPUContext,
}

impl Default for App {
    fn default() -> Self {
        Self::new(ControlFlow::Poll).expect("Failed to create App.")
    }
}

impl App {
    pub fn new(control_flow: ControlFlow) -> Result<Self, EventLoopError> {
        let event_loop = EventLoop::new()?;

        event_loop.set_control_flow(control_flow);

        Ok(Self {
            event_loop,
            windows: std::collections::HashMap::new(),
            context: crate::context::WGPUContext::default(),
        })
    }

    pub fn create_window(
        &mut self,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<WindowId, OsError> {
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::PhysicalSize::new(width, height))
            .build(&self.event_loop)?;

        let window_id = window.id();

        self.windows.insert(window_id, window);

        Ok(window_id)
    }

    pub fn run(mut self) -> Result<(), EventLoopError> {
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
