use std::{iter::once, sync::Arc};
use winit::dpi::PhysicalSize;

use crate::{context::WGPUContext, drawable::Drawable, error::WindowError};

pub struct Frame<'a> {
    pub context: &'a WGPUContext,
    pub surface_texture: wgpu::SurfaceTexture,
    pub view: wgpu::TextureView,
    pub encoder: wgpu::CommandEncoder,
}

impl<'a> Frame<'a> {
    pub fn present(self) {
        self.context.queue().submit(once(self.encoder.finish()));
        self.surface_texture.present();
    }
}

pub struct Window<'window> {
    context: Arc<WGPUContext>,
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'window>,
    config: wgpu::SurfaceConfiguration,
    drawables: Vec<Arc<dyn Drawable>>,
}

impl<'window> Window<'window> {
    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn title(&self) -> String {
        self.window.title()
    }

    pub fn surface(&self) -> &wgpu::Surface<'window> {
        &self.surface
    }

    pub fn get_surface_capabilities(&self) -> wgpu::SurfaceCapabilities {
        self.surface.get_capabilities(&self.context.adapter())
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn drawables(&self) -> &[Arc<dyn Drawable>] {
        &self.drawables
    }

    pub fn drawables_mut(&mut self) -> &mut Vec<Arc<dyn Drawable>> {
        &mut self.drawables
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.context.device(), &self.config);
        }
    }

    pub fn new(
        context: Arc<WGPUContext>,
        window: Arc<winit::window::Window>,
    ) -> Result<Self, WindowError> {
        let size = window.inner_size();

        let surface = context.instance().create_surface(window.clone())?;

        if !context.adapter().is_surface_supported(&surface) {
            return Err(WindowError::SurfaceNotSupported);
        }

        let capabilities = surface.get_capabilities(&context.adapter());

        let format = capabilities
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(capabilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: size.width,
            height: size.height,
            present_mode: capabilities.present_modes[0],
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Ok(Self {
            context,
            window,
            surface,
            config,
            drawables: Vec::new(),
        })
    }

    pub fn frame(&self) -> Result<Frame, wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;

        let view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self
            .context
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        Ok(Frame {
            context: &self.context,
            surface_texture,
            view,
            encoder,
        })
    }
}
