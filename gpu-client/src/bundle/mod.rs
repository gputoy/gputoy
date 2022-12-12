use std::error::Error;
use std::str::FromStr;

mod viewport;
use gpu_common::BundleArgs;
use thiserror::Error;
pub use viewport::ViewportBundle as Viewport;

use crate::resource;

use self::viewport::ViewportError;

pub type Bundles = BundleCache;

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
    type Error: Error;

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
    fn on_run_start(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }

    /// On run end
    ///
    /// Called once at the end of execution (either by reaching the end of project timeline or manual stop).
    /// Would typically be used to write the result of some resource to file or console.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_run_end(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }

    /// On frame start
    ///
    /// Called at the start of every frame.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_frame_start(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }

    /// On frame end
    ///
    /// Called at the start of every frame.
    ///
    /// Noop by default.
    #[allow(unused_variables)]
    fn on_frame_end(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct BundleCache {
    inner: BundleCacheInner,
    resources: resource::Resources,
}

impl BundleCache {
    pub fn new(resources: resource::Resources) -> Self {
        Self {
            inner: BundleCacheInner::new(),
            resources,
        }
    }
    pub fn insert<B>(&mut self, bundle: B)
    where
        B: Bundle,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.get_mut().push(bundle)
    }

    pub fn iter<'a, B>(&'a self) -> impl Iterator<Item = &B>
    where
        B: Bundle + 'a,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.get().iter()
    }

    /// Builds and inserts bundles directly from bundles defined in [`gpu_common::Runner`].
    pub fn build_from_runner(
        &mut self,
        ctx: &crate::Context,
        runner: &gpu_common::Runner,
    ) -> Result<(), BundleError> {
        for (ident, arg) in runner.bundles.iter() {
            match arg {
                BundleArgs::Viewport(arg) => self.insert::<Viewport>(
                    Viewport::new(ctx, ident.to_owned(), &arg).map_err(BundleError::Viewport)?,
                ),
            }
        }
        Ok(())
    }

    pub fn on_run_start(&mut self) -> Result<(), BundleError> {
        self.inner
            .viewports
            .iter_mut()
            .map(|viewport| viewport.on_run_start(&self.resources))
            .collect::<Result<(), ViewportError>>()
            .map_err(BundleError::Viewport)
    }

    pub fn on_run_end(&mut self) -> Result<(), BundleError> {
        self.inner
            .viewports
            .iter_mut()
            .map(|viewport| viewport.on_run_end(&self.resources))
            .collect::<Result<(), ViewportError>>()
            .map_err(BundleError::Viewport)
    }
    pub fn on_frame_start(&mut self) -> Result<(), BundleError> {
        self.inner
            .viewports
            .iter_mut()
            .map(|viewport| viewport.on_frame_start(&self.resources))
            .collect::<Result<(), ViewportError>>()
            .map_err(BundleError::Viewport)
    }

    pub fn on_frame_end(&mut self) -> Result<(), BundleError> {
        self.inner
            .viewports
            .iter_mut()
            .map(|viewport| viewport.on_frame_end(&self.resources))
            .collect::<Result<(), ViewportError>>()
            .map_err(BundleError::Viewport)
    }
}

#[derive(Debug)]
pub struct BundleCacheInner {
    viewports: Vec<Viewport>,
}

impl BundleCacheInner {
    fn new() -> Self {
        BundleCacheInner {
            viewports: Vec::new(),
        }
    }
}

pub trait BundleStore<B: Bundle> {
    fn get(&self) -> &Vec<B>;
    fn get_mut(&mut self) -> &mut Vec<B>;
}

impl BundleStore<Viewport> for BundleCacheInner {
    fn get(&self) -> &Vec<Viewport> {
        &self.viewports
    }
    fn get_mut(&mut self) -> &mut Vec<Viewport> {
        &mut self.viewports
    }
}

#[derive(Debug, Error)]
pub enum BundleError {
    #[error(transparent)]
    Viewport(viewport::ViewportError),
}
