use wgpu::util::DeviceExt;

use crate::{
    model::Model,
    transform::{GPUTransform, Transform},
};

pub struct InstanceGroup {
    instance_buffer: wgpu::Buffer,
    instance_count: u32,
}

impl InstanceGroup {
    pub fn new(device: &wgpu::Device, transforms: &[Transform]) -> Self {
        let gpu_transforms: Vec<GPUTransform> = transforms.iter().map(Transform::to_gpu).collect();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&gpu_transforms),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            instance_buffer,
            instance_count: gpu_transforms.len() as u32,
        }
    }

    pub fn update(&self, queue: &wgpu::Queue, transforms: &[Transform]) {
        let gpu_transforms: Vec<GPUTransform> = transforms.iter().map(Transform::to_gpu).collect();

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&gpu_transforms),
        );
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, model: &'a Model) {
        render_pass.set_pipeline(&model.material.pipeline);

        render_pass.set_bind_group(0, &model.material.diffuse_bind_group, &[]);
        render_pass.set_vertex_buffer(0, model.mesh.vertex_buffer().slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));

        if let Some(index_buffer) = model.mesh.index_buffer() {
            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

            render_pass.draw_indexed(0..model.mesh.index_count(), 0, 0..self.instance_count);
        } else {
            render_pass.draw(0..model.mesh.vertex_count(), 0..self.instance_count);
        }
    }
}
