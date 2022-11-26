use thiserror::Error;

#[cfg(test)]
mod mock;

#[cfg(not(test))]
mod context;

#[cfg(test)]
pub use mock::MockContext as Context;

#[cfg(not(test))]
pub use context::Context;

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
}
