use std::num::NonZeroU32;

pub struct PipelineBuilder<'a> {
    device: &'a wgpu::Device,
    shader_descriptor: wgpu::ShaderModuleDescriptor<'a>,
    vs_entry: &'a str,
    fs_entry: &'a str,
    layout_descriptor: wgpu::PipelineLayoutDescriptor<'a>,
    vertex_buffers: Vec<wgpu::VertexBufferLayout<'a>>,
    color_targets: Vec<Option<wgpu::ColorTargetState>>,
    primitive: wgpu::PrimitiveState,
    depth_stencil: Option<wgpu::DepthStencilState>,
    multisample: wgpu::MultisampleState,
    multiview: Option<NonZeroU32>,
}

impl<'a> PipelineBuilder<'a> {
    pub fn new(
        device: &'a wgpu::Device,
        shader_descriptor: wgpu::ShaderModuleDescriptor<'a>,
        vs_entry: &'a str,
        fs_entry: &'a str,
        format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            device,
            shader_descriptor,
            vs_entry,
            fs_entry,
            layout_descriptor: wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            },
            vertex_buffers: Vec::new(),
            color_targets: vec![Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
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
        }
    }

    pub fn with_layout_descriptor(
        mut self,
        layout_descriptor: wgpu::PipelineLayoutDescriptor<'a>,
    ) -> Self {
        self.layout_descriptor = layout_descriptor;
        self
    }

    pub fn with_bind_group_layouts(
        mut self,
        bind_group_layouts: &'a [&'a wgpu::BindGroupLayout],
    ) -> Self {
        self.layout_descriptor.bind_group_layouts = bind_group_layouts;
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
        self.color_targets = color_targets;
        self
    }

    pub fn with_primitive(mut self, primitive: wgpu::PrimitiveState) -> Self {
        self.primitive = primitive;
        self
    }

    pub fn with_depth_stencil(mut self, depth_stencil: Option<wgpu::DepthStencilState>) -> Self {
        self.depth_stencil = depth_stencil;
        self
    }

    pub fn with_multisample(mut self, multisample: wgpu::MultisampleState) -> Self {
        self.multisample = multisample;
        self
    }

    pub fn with_multiview(mut self, multiview: Option<NonZeroU32>) -> Self {
        self.multiview = multiview;
        self
    }

    pub fn build(self) -> wgpu::RenderPipeline {
        let shader = self.device.create_shader_module(self.shader_descriptor);

        let layout = self.device.create_pipeline_layout(&self.layout_descriptor);

        self.device
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
                    targets: &self.color_targets,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: self.primitive,
                depth_stencil: self.depth_stencil,
                multisample: self.multisample,
                multiview: self.multiview,
                cache: None,
            })
    }
}
