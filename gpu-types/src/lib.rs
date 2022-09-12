mod project;

pub use project::*;

/// Handle returned by after successful build
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RunHandle(u32);
