use thiserror::Error;

mod context;
#[cfg(test)]
mod mock;

pub use context::Context;
#[cfg(test)]
pub use mock::MockContext;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Placeholder error!")]
    Placeholder,
    #[error("Could not create window: {0}")]
    WindowCreation(winit::error::OsError),
    #[error("Failed to obtain adapter")]
    NoAdapter,
    #[error(transparent)]
    DeviceCreationFailed(#[from] wgpu::RequestDeviceError),
    #[error("Project build failed")]
    ProjectBuildFailed,
    #[error("No runner")]
    NoRunner,
    #[error(transparent)]
    Bundle(crate::bundle::BundleError),
    #[error(transparent)]
    Pipeline(crate::pipeline::PipelineError),
}
