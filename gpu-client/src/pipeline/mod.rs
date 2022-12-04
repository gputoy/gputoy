mod vertex_fragment;

use crate::resource;
use gpu_common::{PipelineArgs, PrebuildResult};
use thiserror::Error;
pub use vertex_fragment::*;

pub trait Pipeline: std::fmt::Debug + Sized {
    type Args: Sized;

    /// Build pipeline from its associated type Self::Args.
    fn from_args(
        files: &PrebuildResult,
        device: &wgpu::Device,
        resources: resource::Resources,
        args: &Self::Args,
    ) -> Result<Self, PipelineError>;
    /// Run pipeline.
    fn dispatch(&mut self, encoder: &mut wgpu::CommandEncoder);
}

#[derive(Debug, Error)]
pub enum PipelineError {
    #[error("File not found: {0}")]
    FileNotFound(gpu_common::FilePath),
    #[error("No target textures for fragment shader to draw to. Consider adding targets within the pipeline.")]
    NoFragmentTargets,
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
}

/// DIY dynamic dispatch for pipline trait.
/// Wraps all pipeline types in enum variant.
#[derive(Debug)]
pub enum AnyPipeline {
    VertexFragment(VertexFragment),
}

/// For simplicity of the rendering code, AnyPipeline can be used as a pipline as well.
/// It will defer to its inner pipeline to actually handle the work.
impl Pipeline for AnyPipeline {
    type Args = PipelineArgs;

    fn from_args(
        files: &PrebuildResult,
        device: &wgpu::Device,
        resources: resource::Resources,
        args: &Self::Args,
    ) -> Result<Self, PipelineError> {
        match args {
            PipelineArgs::VertexFragement(args) => Ok(Self::VertexFragment(
                VertexFragment::from_args(files, device, resources, args)?,
            )),
            PipelineArgs::FullscreenQuad(_) => todo!(),
            PipelineArgs::Compute(_) => todo!(),
        }
    }

    fn dispatch(&mut self, encoder: &mut wgpu::CommandEncoder) {
        match self {
            Self::VertexFragment(pipeline) => pipeline.dispatch(encoder),
        }
    }
}
