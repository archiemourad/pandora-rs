pub struct WGPUContextConfiguration {
    pub instance: wgpu::InstanceDescriptor,
    pub adapter: wgpu::RequestAdapterOptions<'static, 'static>,
    pub device: wgpu::DeviceDescriptor<'static>,
}

impl Default for WGPUContextConfiguration {
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

impl WGPUContextConfiguration {
    pub fn new(
        instance: wgpu::InstanceDescriptor,
        adapter: wgpu::RequestAdapterOptions<'static, 'static>,
        device: wgpu::DeviceDescriptor<'static>,
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

impl Default for WGPUContext {
    fn default() -> Self {
        Self::new(WGPUContextConfiguration::default()).expect("Failed to create WGPU context.")
    }
}

impl WGPUContext {
    pub fn new(config: WGPUContextConfiguration) -> Result<Self, wgpu::RequestDeviceError> {
        let instance = wgpu::Instance::new(config.instance);

        let adapter = pollster::block_on(instance.request_adapter(&config.adapter)).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(&config.device, None))?;

        Ok(Self {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
