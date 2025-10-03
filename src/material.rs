use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Material {
    pub label: String,
    pub pipeline: Arc<wgpu::RenderPipeline>,
    pub diffuse_bind_group: Arc<wgpu::BindGroup>,
}

impl Material {
    pub fn new<L: Into<String>>(
        label: L,
        pipeline: Arc<wgpu::RenderPipeline>,
        diffuse_bind_group: Arc<wgpu::BindGroup>,
    ) -> Self {
        Self {
            label: label.into(),
            pipeline,
            diffuse_bind_group,
        }
    }
}
