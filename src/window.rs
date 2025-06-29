use std::{iter::once, sync::Arc};
use winit::dpi::PhysicalSize;

use crate::{context::WGPUContext, drawable::Drawable, error::WindowError};

pub struct Window<'window> {
    context: Arc<WGPUContext>,
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'window>,
    surface_config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
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

    pub fn surface_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.surface_config
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn drawables(&self) -> &[Arc<dyn Drawable>] {
        &self.drawables
    }

    pub fn drawables_mut(&mut self) -> &mut Vec<Arc<dyn Drawable>> {
        &mut self.drawables
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;

            self.surface
                .configure(&self.context.device(), &self.surface_config);
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

        let surface_config = wgpu::SurfaceConfiguration {
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
            surface_config,
            size,
            drawables: Vec::new(),
        })
    }

    pub fn render(&mut self, clear_color: Option<wgpu::Color>) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .context
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(clear_color.unwrap_or(wgpu::Color::TRANSPARENT)),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            for drawable in &self.drawables {
                drawable.draw(&mut _pass);
            }
        }

        self.context.queue().submit(once(encoder.finish()));
        output.present();

        Ok(())
    }
}
