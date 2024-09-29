use thiserror::Error;

#[derive(Error, Debug)]
pub enum WGPUContextError {
    #[error("Failed to find a suitable GPU adapter")]
    AdapterNotFound,
    #[error("Failed to request device: {0:?}")]
    RequestDeviceError(#[from] wgpu::RequestDeviceError),
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    EventLoopError(#[from] winit::error::EventLoopError),
    #[error(transparent)]
    WGPUContextError(#[from] WGPUContextError),
}
