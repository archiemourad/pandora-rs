use std::sync::Arc;

pub struct Material {
    pub pipeline: Arc<wgpu::RenderPipeline>,
    pub diffuse_bind_group: Arc<wgpu::BindGroup>,
}
