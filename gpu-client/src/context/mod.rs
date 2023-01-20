use thiserror::Error;

#[allow(clippy::module_inception)]
mod context;
#[cfg(test)]
mod mock;

pub use context::Context;
#[cfg(test)]
pub use mock::MockContext;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to obtain adapter")]
    NoAdapter,
    #[error(transparent)]
    DeviceCreationFailed(#[from] wgpu::RequestDeviceError),
    #[error("No runner")]
    NoRunner,
    #[error(transparent)]
    Bundle(#[from] crate::bundle::BundleError),
    #[error(transparent)]
    Pipeline(#[from] crate::pipeline::PipelineError),
}
