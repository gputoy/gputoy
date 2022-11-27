use std::error::Error;

mod viewport;
use thiserror::Error;
pub use viewport::ViewportBundle as Viewport;

#[derive(Debug)]
pub struct BundleCache {
    inner: BundleCacheInner,
}

#[derive(Debug)]
pub struct BundleCacheInner {
    viewports: Vec<Viewport>,
}

impl<'a> BundleCache {
    pub fn new() -> Self {
        Self {
            inner: BundleCacheInner {
                viewports: Vec::new(),
            },
        }
    }
    pub fn insert<B>(&mut self, bundle: B)
    where
        B: Bundle,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.get_mut().push(bundle)
    }

    pub fn iter<B>(&'a self) -> impl Iterator<Item = &'a B>
    where
        B: Bundle + 'a,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.get().iter()
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

pub trait Bundle: Sized {
    type Args: Sized;
    type Error: Error;

    fn new(ctx: &crate::context::Context, args: &Self::Args) -> Result<Self, Self::Error>;
    fn destroy(&mut self);
    fn on_run_start(&mut self);
    fn on_run_end(&mut self);
    fn on_frame_start(&mut self);
    fn on_frame_end(&mut self);
}

#[derive(Debug, Error)]
pub enum BundleError {
    #[error(transparent)]
    ViewportError(viewport::ViewportError),
}
