use crate::{context::WGPUContext, error::WindowError};
use std::{iter::once, sync::Arc};
use winit::dpi::PhysicalSize;

pub struct Window<'window> {
    context: Arc<WGPUContext>,
    pipeline: Option<Arc<wgpu::RenderPipeline>>,
    surface: wgpu::Surface<'window>,
    window: Arc<winit::window::Window>,
    config: wgpu::SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl<'window> Window<'window> {
    pub fn set_pipeline(&mut self, pipeline: Arc<wgpu::RenderPipeline>) {
        self.pipeline = Some(pipeline);
    }

    pub fn surface(&self) -> &wgpu::Surface<'window> {
        &self.surface
    }

    pub fn get_surface_capabilities(&self) -> wgpu::SurfaceCapabilities {
        self.surface.get_capabilities(&self.context.adapter())
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn config(&self) -> &wgpu::SurfaceConfiguration {
        &self.config
    }

    pub fn size(&self) -> PhysicalSize<u32> {
        self.size
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;

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
            pipeline: None,
            surface,
            window,
            config,
            size,
        })
    }

    pub fn create_pipeline(
        &mut self,
        shader: wgpu::ShaderModuleDescriptor,
        vs_entry: &str,
        fs_entry: &str,
    ) -> wgpu::RenderPipeline {
        let shader = self.context.device().create_shader_module(shader);

        let layout =
            self.context
                .device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

        let pipeline =
            self.context
                .device()
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: None,
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: vs_entry,
                        buffers: &[],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: fs_entry,
                        targets: &[Some(wgpu::ColorTargetState {
                            format: self.config.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: Some(wgpu::Face::Back),
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

        pipeline
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

            if let Some(pipeline) = &self.pipeline {
                _pass.set_pipeline(pipeline);
                _pass.draw(0..3, 0..1);
            }
        }

        self.context.queue().submit(once(encoder.finish()));
        output.present();

        Ok(())
    }
}
