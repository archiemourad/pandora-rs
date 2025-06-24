use wgpu::util::DeviceExt;

use crate::vertex::VertexLayout;

pub struct Mesh {
    vertex_buffer: wgpu::Buffer,
    index_buffer: Option<wgpu::Buffer>,
    vertex_count: u32,
    index_count: u32,
}

impl Mesh {
    pub fn vertex_buffer(&self) -> &wgpu::Buffer {
        &self.vertex_buffer
    }

    pub fn set_vertices<T: VertexLayout>(
        &mut self,
        device: &wgpu::Device,
        vertices: &[T],
        usage: wgpu::BufferUsages,
    ) {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage,
        });

        self.vertex_buffer = vertex_buffer;
        self.vertex_count = vertices.len() as u32;
    }

    pub fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }

    pub fn set_indices(
        &mut self,
        device: &wgpu::Device,
        indices: Option<&[u32]>,
        usage: wgpu::BufferUsages,
    ) {
        if let Some(indices) = indices {
            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage,
            });

            self.index_buffer = Some(index_buffer);
            self.index_count = indices.len() as u32;
        } else {
            self.index_buffer = None;
            self.index_count = 0;
        }
    }

    pub fn vertex_count(&self) -> u32 {
        self.vertex_count
    }

    pub fn index_count(&self) -> u32 {
        self.index_count
    }

    pub fn new<T: VertexLayout>(
        device: &wgpu::Device,
        vertices: &[T],
        indices: Option<&[u32]>,
        usage: wgpu::BufferUsages,
    ) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(vertices),
            usage,
        });

        let (index_buffer, index_count) = if let Some(indices) = indices {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(indices),
                usage,
            });

            (Some(buffer), indices.len() as u32)
        } else {
            (None, 0)
        };

        Self {
            vertex_buffer,
            index_buffer,
            vertex_count: vertices.len() as u32,
            index_count,
        }
    }
}
