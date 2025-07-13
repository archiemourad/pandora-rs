use cgmath::{InnerSpace, SquareMatrix};
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub struct Camera {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

impl Camera {
    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(self.eye, self.target, self.up);

        let projection =
            cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);

        OPENGL_TO_WGPU_MATRIX * projection * view
    }

    pub fn forward(&mut self, distance: f32) {
        let forward = self.target - self.eye;

        if forward.magnitude() > distance {
            self.eye += forward.normalize() * distance;
        }
    }

    pub fn backward(&mut self, distance: f32) {
        self.eye -= (self.target - self.eye).normalize() * distance;
    }

    pub fn right(&mut self, distance: f32) {
        let forward = self.target - self.eye;

        self.eye = self.target
            - (forward + forward.normalize().cross(self.up) * distance).normalize()
                * forward.magnitude();
    }

    pub fn left(&mut self, distance: f32) {
        let forward = self.target - self.eye;

        self.eye = self.target
            - (forward - forward.normalize().cross(self.up) * distance).normalize()
                * forward.magnitude();
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix().into();
    }

    pub fn new() -> Self {
        Self {
            view_projection: cgmath::Matrix4::identity().into(),
        }
    }
}

pub struct CameraBinding {
    pub uniform: CameraUniform,
    buffer: wgpu::Buffer,
    bind_group_layout: Arc<wgpu::BindGroupLayout>,
    bind_group: Arc<wgpu::BindGroup>,
}

impl CameraBinding {
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn bind_group_layout(&self) -> &Arc<wgpu::BindGroupLayout> {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &Arc<wgpu::BindGroup> {
        &self.bind_group
    }

    pub fn update(&mut self, queue: &wgpu::Queue, camera: &Camera) {
        self.uniform.update_view_projection(camera);

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    pub fn new(device: &wgpu::Device, camera: &Camera) -> Self {
        let mut uniform = CameraUniform::new();

        uniform.update_view_projection(&camera);

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&[uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let bind_group_layout = Arc::new(device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: None,
            },
        ));

        let bind_group = Arc::new(device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
        }));

        Self {
            uniform,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
