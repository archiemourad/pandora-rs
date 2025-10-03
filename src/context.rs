use crate::error::WGPUContextError;

pub struct WGPUContext {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl WGPUContext {
    pub fn instance(&self) -> &wgpu::Instance {
        &self.instance
    }

    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }
}

#[derive(Debug)]
pub struct WGPUContextBuilder<'a, 'b> {
    instance_desc: wgpu::InstanceDescriptor,
    adapter_opts: wgpu::RequestAdapterOptions<'a, 'b>,
    device_desc: wgpu::DeviceDescriptor<'a>,
}

impl<'a, 'b> Default for WGPUContextBuilder<'a, 'b> {
    fn default() -> Self {
        Self {
            instance_desc: wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            },
            adapter_opts: wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            },
            device_desc: wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: Some("Device"),
                memory_hints: Default::default(),
            },
        }
    }
}

impl<'a, 'b> WGPUContextBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instance_desc(mut self, desc: wgpu::InstanceDescriptor) -> Self {
        self.instance_desc = desc;
        self
    }

    pub fn with_adapter_opts(mut self, opts: wgpu::RequestAdapterOptions<'a, 'b>) -> Self {
        self.adapter_opts = opts;
        self
    }

    pub fn with_device_desc(mut self, desc: wgpu::DeviceDescriptor<'a>) -> Self {
        self.device_desc = desc;
        self
    }

    pub fn build(self) -> Result<WGPUContext, WGPUContextError> {
        let instance = wgpu::Instance::new(self.instance_desc);

        let adapter = pollster::block_on(instance.request_adapter(&self.adapter_opts))
            .ok_or(WGPUContextError::AdapterNotFound)?;

        let (device, queue) = pollster::block_on(adapter.request_device(&self.device_desc, None))?;

        Ok(WGPUContext {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
