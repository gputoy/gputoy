use std::error::Error;
use std::str::FromStr;

mod viewport;
use gpu_common::BundleArgs;
use thiserror::Error;
pub use viewport::ViewportBundle as Viewport;

use crate::resource;

use self::viewport::ViewportError;

pub trait Bundle: Sized {
    type Args: Sized;
    type ResourceKeys: FromStr + AsRef<str>;
    type Error: Error;

    fn new(ctx: &crate::Context, ident: String, args: &Self::Args) -> Result<Self, Self::Error>;
    fn destroy(&mut self);
    // fn query_resource<A: AsRef<str>>(&self, ident: A) -> Option<&crate::resource::AnyResource> {
    //     let key = Self::ResourceKeys::from_str(ident.as_ref()).ok()?;
    //     self.query_resource_from_key(key)
    // }
    // fn query_resource_from_key(
    //     &self,
    //     key: Self::ResourceKeys,
    // ) -> Option<&crate::resource::AnyResource>;
    fn prefix_key_with_ident(ident: &str, key: Self::ResourceKeys) -> String {
        format!("{ident}::{}", key.as_ref())
    }
    fn on_run_start(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }
    fn on_run_end(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }
    fn on_frame_start(&mut self, resources: &resource::Resources) -> Result<(), Self::Error> {
        Ok(())
    }
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
