use std::slice::Iter;

use super::*;

proc_macro::bundle_cache_for! {
    system: System,
    viewports: Viewport
}

#[derive(Debug)]
pub struct BundleCache {
    inner: BundleCacheInner,
}

impl BundleCache {
    pub fn new(resources: resource::Resources) -> Self {
        Self {
            inner: BundleCacheInner::new(resources),
        }
    }

    pub fn iter<'a, B>(&'a self) -> impl Iterator<Item = &B>
    where
        B: Bundle + 'a,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.iter()
    }

    pub fn insert<B>(&mut self, bundle: B) -> Result<(), BundleError>
    where
        B: Bundle,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.insert(bundle)
    }

    pub fn remaining<B>(&self) -> u32
    where
        B: Bundle,
        BundleCacheInner: BundleStore<B>,
    {
        self.inner.remaining()
    }

    /// Builds and inserts bundles directly from bundles defined in [`gpu_common::Runner`].
    pub fn build_from_runner(
        &mut self,
        ctx: &crate::Context,
        runner: &gpu_common::Runner,
    ) -> Result<(), BundleError> {
        // System bundle will be created by default
        // Will panic if two system bundles are inserted into the cache
        self.insert(super::System::new(ctx)).unwrap();

        for (ident, arg) in runner.bundles.iter() {
            match arg {
                BundleArgs::Viewport(arg) => self.insert::<Viewport>(
                    Viewport::new(ctx, ident.to_owned(), arg).map_err(BundleError::Viewport)?,
                )?,
            }
        }
        Ok(())
    }

    pub fn on_run_start(&mut self, sys: &crate::system::System) -> Result<(), BundleError> {
        self.inner.on_run_start(sys)
    }

    pub fn on_run_end(&mut self, sys: &crate::system::System) -> Result<(), BundleError> {
        self.inner.on_run_end(sys)
    }
    pub fn on_frame_start(&mut self, sys: &crate::system::System) -> Result<(), BundleError> {
        self.inner.on_frame_start(sys)
    }

    pub fn on_frame_end(&mut self, sys: &crate::system::System) -> Result<(), BundleError> {
        self.inner.on_frame_end(sys)
    }
}

pub trait BundleStore<B: Bundle> {
    fn iter(&'_ self) -> Iter<'_, B>;
    fn insert(&mut self, bundle: B) -> Result<(), BundleError>;
    fn remaining(&self) -> u32;
}

#[macro_use]
mod proc_macro {

    macro_rules! bundle_cache_for {
        ($($field:ident: $bundle:ty), *) => {
            $crate::bundle::cache::proc_macro::bundle_cache_for!(struct $($field $bundle)*);
            $crate::bundle::cache::proc_macro::bundle_cache_for!(impl $($field $bundle)*);
        };

        (struct $($field:ident $bundle:ty)*) => {
            #[derive(Debug)]
            pub struct BundleCacheInner {
                resources: resource::Resources,
                $(
                    $field: ArrayVec<$bundle, {<$bundle as Bundle>::MAX_INSTANCES as usize}>,
                )*
            }
        };

        (impl $($field:ident $bundle:ty)*) => {
            impl BundleCacheInner {
                pub fn new(resources: resource::Resources) -> Self {
                    Self {
                        resources,
                        $(
                            $field: ArrayVec::new(),
                        )*
                    }
                }
            }

            impl BundleCacheInner {
                $crate::bundle::cache::proc_macro::bundle_cache_for!(on_run_start $($field $bundle)*);
                $crate::bundle::cache::proc_macro::bundle_cache_for!(on_run_end $($field $bundle)*);
                $crate::bundle::cache::proc_macro::bundle_cache_for!(on_frame_start $($field $bundle)*);
                $crate::bundle::cache::proc_macro::bundle_cache_for!(on_frame_end $($field $bundle)*);
            }

            $(
                impl BundleStore<$bundle> for BundleCacheInner {
                    fn iter(&'_ self) -> Iter<'_, $bundle> {
                        self.$field.iter()
                    }
                    fn insert(&mut self, bundle: $bundle) -> Result<(), BundleError> {
                        self
                            .$field
                            .try_push(bundle)
                            .map_err(|_| {
                                BundleError::BundleLimitExceeded(
                                    stringify!($bundle).to_owned(),
                                    <$bundle as Bundle>::MAX_INSTANCES
                                )
                            })
                    }
                    fn remaining(&self) -> u32 {
                        <$bundle as Bundle>::MAX_INSTANCES - self.$field.len() as u32
                    }
                }
            )*
        };

        ($lifecycle:ident $($field:ident $bundle:ty)*) => {

            fn $lifecycle(
                &mut self,
                sys: &crate::system::System,
            ) -> Result<(), BundleError> {
                $(
                    self
                        .$field
                        .iter_mut()
                        .map(|bundle| bundle.$lifecycle(sys, &self.resources))
                        .collect::<Result<(), <$bundle as Bundle>::Error>>()?;
                )*
                Ok(())
            }
        };
    }
    pub(super) use bundle_cache_for;
}
