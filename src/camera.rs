use cgmath::{InnerSpace, Matrix4, Point3, Rad, SquareMatrix, Vector3, Vector4};
use std::sync::Arc;
use wgpu::util::DeviceExt;

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::from_cols(
    Vector4::new(1.0, 0.0, 0.0, 0.0),
    Vector4::new(0.0, 1.0, 0.0, 0.0),
    Vector4::new(0.0, 0.0, 0.5, 0.0),
    Vector4::new(0.0, 0.0, 0.5, 1.0),
);

pub struct Camera {
    pub position: Point3<f32>,
    pub yaw: Rad<f32>,
    pub pitch: Rad<f32>,
}

impl Camera {
    pub fn build_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        Matrix4::look_to_rh(
            self.position,
            Vector3::new(cos_pitch * cos_yaw, sin_pitch, cos_pitch * sin_yaw).normalize(),
            Vector3::unit_y(),
        )
    }

    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        }
    }
}

pub struct Projection {
    pub aspect: f32,
    pub fovy: Rad<f32>,
    pub znear: f32,
    pub zfar: f32,
}

impl Projection {
    pub fn build_matrix(&self) -> Matrix4<f32> {
        OPENGL_TO_WGPU_MATRIX * cgmath::perspective(self.fovy, self.aspect, self.znear, self.zfar)
    }

    pub fn new<F: Into<Rad<f32>>>(width: u32, height: u32, fovy: F, znear: f32, zfar: f32) -> Self {
        Self {
            aspect: width as f32 / height as f32,
            fovy: fovy.into(),
            znear,
            zfar,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    pub view_position: [f32; 4],
    pub view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn update_view_projection(&mut self, camera: &Camera, projection: &Projection) {
        self.view_position = camera.position.to_homogeneous().into();
        self.view_projection = (projection.build_matrix() * camera.build_matrix()).into();
    }

    pub fn new() -> Self {
        Self {
            view_position: [0.0; 4],
            view_projection: cgmath::Matrix4::identity().into(),
        }
    }
}

pub struct GPUCamera {
    pub uniform: CameraUniform,
    buffer: wgpu::Buffer,
    bind_group_layout: Arc<wgpu::BindGroupLayout>,
    bind_group: Arc<wgpu::BindGroup>,
}

impl GPUCamera {
    pub fn buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn bind_group_layout(&self) -> &Arc<wgpu::BindGroupLayout> {
        &self.bind_group_layout
    }

    pub fn bind_group(&self) -> &Arc<wgpu::BindGroup> {
        &self.bind_group
    }

    pub fn update(&mut self, queue: &wgpu::Queue, camera: &Camera, projection: &Projection) {
        self.uniform.update_view_projection(&camera, &projection);

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }

    pub fn new(device: &wgpu::Device, camera: &Camera, projection: &Projection) -> Self {
        let mut uniform = CameraUniform::new();

        uniform.update_view_projection(&camera, &projection);

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
