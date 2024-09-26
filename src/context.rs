pub struct WGPUContextConfiguration<'a, 'b> {
    pub instance: wgpu::InstanceDescriptor,
    pub adapter: wgpu::RequestAdapterOptions<'a, 'b>,
    pub device: wgpu::DeviceDescriptor<'a>,
}

impl<'a, 'b> Default for WGPUContextConfiguration<'a, 'b> {
    fn default() -> Self {
        Self::new(
            wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            },
            wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            },
            wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
        )
    }
}

impl<'a, 'b> WGPUContextConfiguration<'a, 'b> {
    pub fn new(
        instance: wgpu::InstanceDescriptor,
        adapter: wgpu::RequestAdapterOptions<'a, 'b>,
        device: wgpu::DeviceDescriptor<'a>,
    ) -> Self {
        Self {
            instance,
            adapter,
            device,
        }
    }
}

pub struct WGPUContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
}

impl WGPUContext {
    pub fn new(config: WGPUContextConfiguration) -> Result<Self, crate::error::WGPUContextError> {
        use crate::error::WGPUContextError;

        let instance = wgpu::Instance::new(config.instance);

        let adapter = pollster::block_on(instance.request_adapter(&config.adapter))
            .ok_or(WGPUContextError::AdapterNotFound)?;

        let (device, queue) = pollster::block_on(adapter.request_device(&config.device, None))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
