use crate::vertex::Vertex;
use wgpu::util::DeviceExt;

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: Option<wgpu::Buffer>,
    pub index_count: u32,
    pub vertex_count: u32,
}

impl Mesh {
    pub fn new<T: Vertex>(
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
            index_count,
            vertex_count: vertices.len() as u32,
        }
    }
}
