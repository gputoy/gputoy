use arrayvec::ArrayVec;
use std::{cell::RefCell, rc::Rc, str::FromStr};
use thiserror::Error;

use gpu_common::BundleArgs;

mod cache;
mod sys;
mod viewport;

pub use cache::BundleCache;
pub use sys::SystemBundle as System;
pub use viewport::ViewportBundle as Viewport;

use crate::resource;

pub type Bundles = Rc<RefCell<cache::BundleCache>>;

/// Bundle trait
///
/// Describes a bundle which is a collection of resources and external I/O that
/// provides some functionality to shaders. Bundles are supposed to act as the bridge
/// between shaders and the external environment, whether that be the operating system or
/// browser.
pub trait Bundle: Sized {
    /// All arguments needed to initialize bundle.  
    type Args: Sized + for<'a> serde::Deserialize<'a>;

    /// Statically enumerated set of resources that bundle will have ownership over.
    type ResourceKeys: FromStr + AsRef<str>;

    /// Error type for this bundle.
    type Error: std::error::Error;

    /// Maximum instances of this bundle in any given run.
    const MAX_INSTANCES: u32;

    /// Create new bundle.
    ///
    /// Creates resources, attaches event listeners, intakes data, etc. necessary to provide
    /// functionality of bundle.
    fn new(ctx: &crate::Context, ident: String, args: &Self::Args) -> Result<Self, Self::Error>;

    /// Destroy bundle
    ///
    /// Frees all bundle resources, dettaches event listeners, drops files handles, etc.
    fn destroy(&mut self);

    /// Prefix key with identifier
    ///
    /// Get string resource identifier for a particular resource key. This just prefixes the ResourceKey with
    /// the identifier of this bundle. For examople if this buffer had an identifier 'view' and we input something
    /// like Self::ResourceKey::Surface (example from Viewport), then the output would be 'view::surface'.
    fn prefix_key_with_ident(ident: &str, key: Self::ResourceKeys) -> String {
        format!("{ident}::{}", key.as_ref())
    }

    /// On run start
    ///
    /// Called once at the start of execution. This is a good place to calculate things that are
    /// needed throughout the run of the project.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_run_start(
        &mut self,
        sys: &crate::system::System,
        resources: &resource::Resources,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// On run end
    ///
    /// Called once at the end of execution (either by reaching the end of project timeline or manual stop).
    /// Would typically be used to write the result of some resource to file or console.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_run_end(
        &mut self,
        sys: &crate::system::System,
        resources: &resource::Resources,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// On frame start
    ///
    /// Called at the start of every frame.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_frame_start(
        &mut self,
        sys: &crate::system::System,
        resources: &resource::Resources,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// On frame end
    ///
    /// Called at the start of every frame.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_frame_end(
        &mut self,
        sys: &crate::system::System,
        resources: &resource::Resources,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum BundleError {
    #[error("Bundle limit exceeded for {0}. Max allowed: {1}")]
    BundleLimitExceeded(String, u32),
    #[error(transparent)]
    Viewport(#[from] viewport::ViewportError),
    #[error(transparent)]
    System(#[from] sys::SystemBundleError),
}
