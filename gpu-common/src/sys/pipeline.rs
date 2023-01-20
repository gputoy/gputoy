#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{FastHashMap, FilePath};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PipelineArgs {
    /// # Vertex/Fragment pipeline
    ///
    /// Plain rasterizer pipeline with configurable vertex inputs and texture output targets.
    ///
    /// Note: While this can be used for fullscreen quad, the easier method would to be to use
    /// the built in FullscreenQuad pipeline, which handles the vertex shader automatically.
    VertexFragement(VertexFragment),
    /// # Fullscreen quad pipeline
    ///
    /// Fragment shader over rasterized fullscreen quad.
    ///
    /// Similar in function to a 'shadertoy' shader.
    FullscreenQuad(FullscreenQuad),
    /// # Compute pipeline
    ///
    /// Compute pipeline to be ran over the range of some resource.
    Compute(Compute),
}

type Targets = Vec<String>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VertexFragment {
    /// # Vertex shader
    ///
    /// A path to "wgsl" or "glsl" shader file that contains a vertex entry point.
    ///
    /// Note: this may be the same as fragment shader as long as that file has both vertex and fragment entry points.
    pub vertex: FilePath,
    /// # Fragment shader
    ///
    /// A path to "wgsl" or "glsl" shader file that contains a fragment entry point.
    ///
    /// Note: this may be the same as vertex shader as long as that file has both vertex and fragment entry points.
    pub fragment: FilePath,
    pub binds: Option<Binds>,
    /// # Fragment shader targets
    ///
    /// Ordered list of texture resources the fragment shader will draw to.
    /// Ordering will correspond to location attributes in shader output.
    pub targets: Targets,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FullscreenQuad {
    /// # Fragment shader
    ///
    /// A path to "wgsl" or "glsl" shader file that contains a fragment entry point.
    pub fragment: FilePath,
    pub binds: Option<Binds>,
    /// # Fragment shader targets
    ///
    /// Ordered list of texture resources the fragment shader will draw to.
    /// Ordering will correspond to location attributes in shader output.
    pub targets: Targets,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Compute {
    /// # Compute shader
    ///
    /// A path to "wgsl" or "glsl" shader file that contains a compute entry point.
    pub shader: FilePath,
    pub binds: Option<Binds>,
}

/// # Pipeline binds
///
/// Describes a map between syned shader variables and resource path.
///
/// During build, the resource at named sync variable will be bound via bind groups.
///
/// Resources can be from either project resources or from a bundle. For example, "resource::particles"
/// will look for defined resource named "particles", and "view::surface" will look for the "surface" resource from
/// defined bundle called "view".
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Binds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: FastHashMap<String, String>,
}
