#[cfg(feature = "schema")]
use schemars::JsonSchema;
#[cfg(feature = "deserialize")]
use serde::Deserialize;
#[cfg(feature = "serialize")]
use serde::Serialize;

use crate::BundleArgs;
use crate::FastHashMap;
use crate::PipelineArgs;

/// # Runner
///
/// A runner is used to orchestrate shader execution and resource management.
///
/// A project can have multiple runners, but will default to /run.json.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "schema", derive(JsonSchema))]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "deserialize", derive(Deserialize))]
pub struct Runner {
    /// # Runner bundles
    ///
    /// Bundles are a collection of Resources surrounding some form of input/ouput that
    /// are maintained around a runner's lifecycle (run start, run end, frame start, frame end).
    ///
    /// For example, the Viewport bundle has a 'surface' texture resource which can be written to
    /// in fragment shader, a 'mouse' uniform buffer corresponding to mouse position over said viewport,
    /// and a 'resolution' uniform buffer that contains the current resolution of the viewport.
    ///
    /// In the pipelines, these resources can be used just like a resource defined by the user.
    /// Only instead of 'res::{some_resource_name}', the identifier will be
    /// '{some_bundle_name}::{some_resource_within_bundle}'.
    ///
    /// For instance, if you wanted to use the surface of a Viewport bundle named 'view'
    /// within a pipeline, you would identiy it like 'view::surface'.
    pub bundles: FastHashMap<String, BundleArgs>,
    /// # Runner pipelines
    ///
    /// A pipeline represents an execution of shader.
    pub pipelines: Vec<PipelineArgs>,
}
