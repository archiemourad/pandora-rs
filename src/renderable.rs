use crate::mesh::Mesh;
use std::sync::Arc;

pub struct Renderable {
    pub mesh: Arc<Mesh>,
    pub pipeline: Arc<wgpu::RenderPipeline>,
}

impl Renderable {
    pub fn new(mesh: Arc<Mesh>, pipeline: Arc<wgpu::RenderPipeline>) -> Self {
        Self { mesh, pipeline }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer.slice(..));

        if let Some(index_buffer) = &self.mesh.index_buffer {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);
            render_pass.draw_indexed(0..self.mesh.index_count, 0, 0..1);
        } else {
            render_pass.draw(0..self.mesh.vertex_count, 0..1);
        }
    }
}
