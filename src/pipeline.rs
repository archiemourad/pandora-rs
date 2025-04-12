use std::num::NonZeroU32;

pub struct PipelineBuilder<'a> {
    device: &'a wgpu::Device,
    shader: wgpu::ShaderModuleDescriptor<'a>,
    vs_entry: &'a str,
    fs_entry: &'a str,
    format: wgpu::TextureFormat,
    layout: Option<wgpu::PipelineLayoutDescriptor<'a>>,
    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,
    color_targets: Option<Vec<Option<wgpu::ColorTargetState>>>,
    primitive: Option<wgpu::PrimitiveState>,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: Option<wgpu::MultisampleState>,
    multiview: Option<NonZeroU32>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn with_layout(mut self, layout: wgpu::PipelineLayoutDescriptor<'a>) -> Self {
        self.layout = Some(layout);
        self
    }

    pub fn with_vertex_buffers(
        mut self,
        vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,
    ) -> Self {
        self.vertex_buffers = vertex_buffers;
        self
    }

    pub fn with_color_targets(
        mut self,
        color_targets: Vec<Option<wgpu::ColorTargetState>>,
    ) -> Self {
        self.color_targets = Some(color_targets);
        self
    }

    pub fn with_primitive(mut self, primitive: wgpu::PrimitiveState) -> Self {
        self.primitive = Some(primitive);
        self
    }

    pub fn with_depth_stencil(mut self, depth_stencil: wgpu::DepthStencilState) -> Self {
        self.depth_stencil = Some(depth_stencil);
        self
    }

    pub fn with_multisample(mut self, multisample: wgpu::MultisampleState) -> Self {
        self.multisample = Some(multisample);
        self
    }

    pub fn with_multiview(mut self, multiview: NonZeroU32) -> Self {
        self.multiview = Some(multiview);
        self
    }

    pub fn new(
        device: &'a wgpu::Device,
        shader: wgpu::ShaderModuleDescriptor<'a>,
        vs_entry: &'a str,
        fs_entry: &'a str,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            device,
            shader,
            vs_entry,
            fs_entry,
            format,
            layout: None,
            vertex_buffers: Vec::new(),
            color_targets: None,
            primitive: None,
            depth_stencil: None,
            multisample: None,
            multiview: None,
        }
    }

    pub fn build(self) -> wgpu::RenderPipeline {
        let shader = self.device.create_shader_module(self.shader);

        let layout = self
            .device
            .create_pipeline_layout(&self.layout.unwrap_or_else(|| {
                wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                }
            }));

        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: self.vs_entry,
                    buffers: &self.vertex_buffers,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: self.fs_entry,
                    targets: &self.color_targets.unwrap_or_else(|| {
                        vec![Some(wgpu::ColorTargetState {
                            format: self.format,
                            blend: Some(wgpu::BlendState::REPLACE),
                            write_mask: wgpu::ColorWrites::ALL,
                        })]
                    }),
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: self.primitive.unwrap_or_else(|| wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: Some(wgpu::Face::Back),
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                }),
                depth_stencil: self.depth_stencil,
                multisample: self.multisample.unwrap_or_else(|| wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                }),
                multiview: self.multiview,
                cache: None,
            });

        pipeline
    }
}
