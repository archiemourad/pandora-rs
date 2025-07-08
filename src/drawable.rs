use std::sync::Arc;

use crate::mesh::Mesh;

pub trait Drawable {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
}

pub struct Material {
    pub pipeline: Arc<wgpu::RenderPipeline>,
    pub bind_group: Arc<wgpu::BindGroup>,
}

pub struct Model {
    mesh: Arc<Mesh>,
    material: Arc<Material>,
}

impl Model {
    pub fn mesh(&self) -> &Arc<Mesh> {
        &self.mesh
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn new(mesh: Arc<Mesh>, material: Arc<Material>) -> Self {
        Self { mesh, material }
    }
}

impl Drawable for Model {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_pipeline(&self.material.pipeline);
        render_pass.set_bind_group(0, &self.material.bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.mesh.vertex_buffer().slice(..));

        if let Some(index_buffer) = &self.mesh.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..self.mesh.index_count(), 0, 0..1);
        } else {
            render_pass.draw(0..self.mesh.vertex_count(), 0..1);
        }
    }
}
