use crate::{context::WGPUContext, error::WindowError};
use std::{iter::once, sync::Arc};
use winit::dpi::PhysicalSize;

pub struct Window<'window> {
    pub surface: wgpu::Surface<'window>,
    pub config: wgpu::SurfaceConfiguration,
    pub size: PhysicalSize<u32>,

    context: Arc<WGPUContext>,
    window: Arc<winit::window::Window>,
}

impl<'window> Window<'window> {
    pub fn new(
        context: Arc<WGPUContext>,
        window: Arc<winit::window::Window>,
    ) -> Result<Self, WindowError> {
        let size = window.inner_size();

        let surface = context.instance.create_surface(window.clone())?;

        if !context.adapter.is_surface_supported(&surface) {
            return Err(WindowError::SurfaceNotSupported);
        }

        let capabilities = surface.get_capabilities(&context.adapter);

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
            surface,
            config,
            size,
            context,
            window,
        })
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

            self.config.width = new_size.width;
            self.config.height = new_size.height;

            self.surface.configure(&self.context.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let _pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });
        }

        self.context.queue.submit(once(encoder.finish()));
        output.present();

        Ok(())
    }
}
