use std::sync::Arc;

use pandora::{drawable::Model, material::Material, mesh::Mesh};

use crate::vertex::Vertex;

pub struct Shape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Shape {
    pub fn pentagon() -> Self {
        Self {
            vertices: vec![
                Vertex {
                    position: [-0.0868241, 0.49240386, 0.0],
                    tex_coords: [0.4131759, 0.00759614],
                },
                Vertex {
                    position: [-0.49513406, 0.06958647, 0.0],
                    tex_coords: [0.0048659444, 0.43041354],
                },
                Vertex {
                    position: [-0.21918549, -0.44939706, 0.0],
                    tex_coords: [0.28081453, 0.949397],
                },
                Vertex {
                    position: [0.35966998, -0.3473291, 0.0],
                    tex_coords: [0.85967, 0.84732914],
                },
                Vertex {
                    position: [0.44147372, 0.2347359, 0.0],
                    tex_coords: [0.9414737, 0.2652641],
                },
            ],
            indices: vec![0, 1, 2, 0, 2, 3, 0, 3, 4],
        }
    }

    pub fn build(
        self,
        device: &wgpu::Device,
        pipeline: Arc<wgpu::RenderPipeline>,
        texture_bind_group: Arc<wgpu::BindGroup>,
    ) -> Model {
        let mesh = Arc::new(Mesh::new(
            device,
            &self.vertices,
            Some(&self.indices),
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        ));

        let material = Arc::new(Material {
            pipeline,
            diffuse_bind_group: texture_bind_group,
        });

        Model { mesh, material }
    }
}
