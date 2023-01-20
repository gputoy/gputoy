use gpu_common::FastHashMap;
use slotmap::{HopSlotMap, SecondaryMap};

use super::Resource;
use crate::Context;

/// Stores all resource instances during runtime.
///
/// During project build/rebuilds,
#[derive(Debug, Default)]
pub struct ResourceCache {
    inner: ResourceCacheInner,
}

impl ResourceCache {
    /// Insert resource into cache, returning a handle that can then be used
    /// to query the resource in the future with [`Self::get()`].
    ///
    ///
    /// ```
    /// use gpu_client::{Context, resource::{Buffer, BufferArgs, ResourceCache}};
    ///
    /// let context = Context::new();
    /// let mut cache = ResourceCache::default();
    ///
    /// let args = BufferArgs::default();
    /// let resource = Buffer::new(&context, &args);
    /// let handle = cache.insert(resource);
    /// ```
    pub fn insert<R>(&mut self, resource: R) -> R::Handle
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        self.inner.store_mut().insert(resource)
    }

    /// Create resource from argument and insert into cache, returning a handle
    /// that can then be use to query the resource in the future with [`Self::get()`].
    ///
    ///
    /// ```
    /// use gpu_client::{Context, resource};
    ///
    /// let context = Context::new();
    /// let mut cache = resource::ResourceCache::default();
    ///
    /// let args = resource::BufferArgs::default();
    /// let handle = cache.insert(&context, &args);
    /// ```
    ///
    pub fn insert_from_args<R>(&mut self, ctx: &Context, args: &R::Args) -> R::Handle
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        let resource = R::new(ctx, args);
        self.insert_with_ident(resource, gpu_common::ResourceArguments::ident(args))
    }

    ///
    pub fn insert_with_ident<R>(&mut self, resource: R, ident: &str) -> R::Handle
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        let handle = self.inner.store_mut().insert(resource);
        self.inner.names_mut().insert(ident.to_owned(), handle);
        handle
    }

    /// Get reference to resource of type R by handle.
    pub fn get<R>(&self, handle: R::Handle) -> Option<&R>
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        self.inner.store().get(handle)
    }

    /// Get mutable reference to resource of type R by handle.
    pub fn get_mut<R>(&mut self, handle: R::Handle) -> Option<&mut R>
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        self.inner.store_mut().get_mut(handle)
    }

    /// Find resource of type R by string identifier, returning the proper handle alongside resource for
    /// faster future access.
    pub fn get_by_ident<R>(&self, ident: &str) -> Option<(R::Handle, &R)>
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        self.inner
            .names()
            .get(ident)
            // Unwrap ok, were sure to have a resource if a key exists in the name map
            .map(|handle| (*handle, self.inner.store().get(*handle).unwrap()))
    }

    /// Get immutable iterator over a particular resource type R.
    pub fn iter<'a, R>(&'a self) -> impl Iterator<Item = &R>
    where
        R: Resource + 'a,
        ResourceCacheInner: ResourceStore<R>,
    {
        self.inner.store().values()
    }

    /// Clears store all of particluar resource R.
    ///
    /// TODO: Do some testing to determine if it is better to destroy all
    /// cleared resources is preferrable to letting wgpu internal handle it lazily.
    pub fn clear<R>(&mut self)
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        self.inner.names_mut().clear();
        self.inner.store_mut().clear();
    }

    pub fn clear_all(&mut self) {
        self.inner.clear()
    }

    /// Removes resource from cache, returning the owned resource if found at handle.
    ///
    /// Note: It is up to the caller to destroy or drop the resource, with destroying
    /// immediately invalidating any use of the resource.
    pub fn remove<R>(&mut self, handle: R::Handle) -> Option<R>
    where
        R: Resource,
        ResourceCacheInner: ResourceStore<R>,
    {
        let removed = self.inner.store_mut().remove(handle)?;
        let ident = self
            .inner
            .names()
            .iter()
            .find(|(_, &other_handle)| other_handle == handle)
            .map(|(ident, _)| ident.to_owned())?;
        let _ = self.inner.names_mut().remove(&ident);
        Some(removed)
    }
}

/// Trait for directly accessing stores for a certain Resource type.
///
/// This allows the public api of ResourceCache to be generic across Resource
/// while keeping direct access to these stores private.
pub trait ResourceStore<R: Resource> {
    /// For read operations on Resource instance(s).
    fn store(&self) -> &HopSlotMap<R::Handle, R>;
    /// For mutating operations on Resource instance(s).
    fn store_mut(&mut self) -> &mut HopSlotMap<R::Handle, R>;
    /// For read operations on Resource identifier(s).
    fn names(&self) -> &FastHashMap<String, R::Handle>;
    /// For mutating operations on Resource identifier(s).
    fn names_mut(&mut self) -> &mut FastHashMap<String, R::Handle>;
}

use super::{Buffer, BufferHandle, Texture, TextureHandle};

/// Private inner cache containing direct resource stores.
///
/// These can be fetched generically using the ResourceStore<R> trait.
#[derive(Debug)]
pub struct ResourceCacheInner {
    textures: HopSlotMap<TextureHandle, Texture>,
    texture_views: SecondaryMap<TextureHandle, Vec<wgpu::TextureView>>,
    texture_idents: FastHashMap<String, TextureHandle>,
    buffers: HopSlotMap<BufferHandle, Buffer>,
    buffer_idents: FastHashMap<String, BufferHandle>,
}

impl Default for ResourceCacheInner {
    fn default() -> Self {
        Self {
            textures: HopSlotMap::with_key(),
            texture_views: SecondaryMap::new(),
            texture_idents: FastHashMap::default(),
            buffers: HopSlotMap::with_key(),
            buffer_idents: FastHashMap::default(),
        }
    }
}

// Inner methods not generic over Resource.
impl ResourceCacheInner {
    /// Clears entire cache for every resource type.
    fn clear(&mut self) {
        self.textures.clear();
        self.texture_views.clear();
        self.texture_idents.clear();
        self.buffers.clear();
        self.buffer_idents.clear();
    }
}

// Resource store implementations for ResourceCacheInner
// While this is very macro'able, there will only be three anyway.

// Resource store for buffer.
// Accesses buffers, buffer_idents from ResourceCacheInner
impl ResourceStore<Buffer> for ResourceCacheInner {
    #[inline(always)]
    fn store(&self) -> &HopSlotMap<<Buffer as Resource>::Handle, Buffer> {
        &self.buffers
    }

    #[inline(always)]
    fn store_mut(&mut self) -> &mut HopSlotMap<<Buffer as Resource>::Handle, Buffer> {
        &mut self.buffers
    }

    #[inline(always)]
    fn names(&self) -> &FastHashMap<String, <Buffer as Resource>::Handle> {
        &self.buffer_idents
    }

    #[inline(always)]
    fn names_mut(&mut self) -> &mut FastHashMap<String, <Buffer as Resource>::Handle> {
        &mut self.buffer_idents
    }
}

// Resource store for buffer.
// Accesses textures, texture_idents, texture_views from ResourceCacheInner
impl ResourceStore<Texture> for ResourceCacheInner {
    #[inline(always)]
    fn store(&self) -> &HopSlotMap<<Texture as Resource>::Handle, Texture> {
        &self.textures
    }

    #[inline(always)]
    fn store_mut(&mut self) -> &mut HopSlotMap<<Texture as Resource>::Handle, Texture> {
        &mut self.textures
    }

    #[inline(always)]
    fn names(&self) -> &FastHashMap<String, <Texture as Resource>::Handle> {
        &self.texture_idents
    }

    #[inline(always)]
    fn names_mut(&mut self) -> &mut FastHashMap<String, <Texture as Resource>::Handle> {
        &mut self.texture_idents
    }
}
