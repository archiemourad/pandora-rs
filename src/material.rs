use std::sync::Arc;

#[derive(Clone)]
pub struct Material {
    pub pipeline: Arc<wgpu::RenderPipeline>,
    pub diffuse_bind_group: Arc<wgpu::BindGroup>,
}
