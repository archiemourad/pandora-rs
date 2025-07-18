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
    instance_descriptor: wgpu::InstanceDescriptor,
    adapter_options: wgpu::RequestAdapterOptions<'a, 'b>,
    device_descriptor: wgpu::DeviceDescriptor<'a>,
}

impl<'a, 'b> Default for WGPUContextBuilder<'a, 'b> {
    fn default() -> Self {
        Self {
            instance_descriptor: wgpu::InstanceDescriptor {
                backends: wgpu::Backends::PRIMARY,
                ..Default::default()
            },
            adapter_options: wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            },
            device_descriptor: wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: Default::default(),
            },
        }
    }
}

impl<'a, 'b> WGPUContextBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_instance_descriptor(mut self, descriptor: wgpu::InstanceDescriptor) -> Self {
        self.instance_descriptor = descriptor;
        self
    }

    pub fn with_adapter_options(mut self, options: wgpu::RequestAdapterOptions<'a, 'b>) -> Self {
        self.adapter_options = options;
        self
    }

    pub fn with_device_descriptor(mut self, descriptor: wgpu::DeviceDescriptor<'a>) -> Self {
        self.device_descriptor = descriptor;
        self
    }

    pub fn build(self) -> Result<WGPUContext, WGPUContextError> {
        let instance = wgpu::Instance::new(self.instance_descriptor);

        let adapter = pollster::block_on(instance.request_adapter(&self.adapter_options))
            .ok_or(WGPUContextError::AdapterNotFound)?;

        let (device, queue) =
            pollster::block_on(adapter.request_device(&self.device_descriptor, None))?;

        Ok(WGPUContext {
            instance,
            adapter,
            device,
            queue,
        })
    }
}
