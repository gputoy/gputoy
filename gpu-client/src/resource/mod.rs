use crate::context::Context;
use gpu_common::FastHashMap;
use slotmap::{new_key_type, HopSlotMap, Key, SecondaryMap};

mod buffer;
mod sampler;
mod texture;

pub use buffer::BufferResource as Buffer;
pub use texture::TextureResource as Texture;

new_key_type! { pub struct TextureHandle; }
new_key_type! { pub struct BufferHandle; }

pub trait SubResource {}

pub trait Resource {
    type Args: gpu_common::ResourceArguments + Sized;
    type SubResource: SubResource;
    type Handle: Key;

    const LABEL: &'static str;
    const SHADER_DECL: &'static str;

    fn new(ctx: &Context, args: &Self::Args) -> Self;

    fn destroy(self);

    fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry;

    fn bind_group_layout_entry(
        &self,
        binding: u32,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry;
    fn args(&self) -> &Self::Args;
}

/// Stores all resource instances during runtime.
///
/// During project build/rebuilds,
#[derive(Debug)]
pub struct ResourceCache<'a> {
    inner: ResourceCacheInner<'a>,
}

impl<'a> ResourceCache<'a> {
    pub fn new() -> Self {
        Self {
            inner: ResourceCacheInner::new(),
        }
    }

    pub fn insert<R>(&mut self, resource: R) -> R::Handle
    where
        R: Resource,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        self.inner.store_mut().insert(resource)
    }

    pub fn insert_from_args<R>(&'a mut self, ctx: &Context, args: &'a R::Args) -> R::Handle
    where
        R: Resource + 'a,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        let resource = R::new(ctx, args);
        self.insert_with_ident(resource, gpu_common::ResourceArguments::ident(args))
    }

    ///
    pub fn insert_with_ident<R>(&'a mut self, resource: R, ident: &'a str) -> R::Handle
    where
        R: Resource + 'a,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        let handle = self.inner.store_mut().insert(resource);
        self.inner.names_mut().insert(ident, handle);
        handle
    }

    /// Get resource of type R by handle.
    pub fn get<R>(&'a self, handle: R::Handle) -> Option<&'a R>
    where
        R: Resource,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        self.inner.store().get(handle)
    }

    /// Find resource of type R by string identifier, returning the proper handle alongside resource for
    /// faster future access.
    pub fn get_by_ident<R>(&'a self, ident: &str) -> Option<(R::Handle, &'a R)>
    where
        R: Resource,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        self.inner
            .names()
            .get(ident)
            // Unwrap ok, were sure to have a resource if a key exists in the name map
            .map(|handle| (*handle, self.inner.store().get(*handle).unwrap()))
    }

    /// Get immutable iterator over a particular resource type R.
    pub fn iter<R>(&'a self) -> impl Iterator<Item = &'a R>
    where
        R: Resource + 'a,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        self.inner.store().values()
    }

    /// Clears store all of particluar resource R.
    ///
    /// TODO: Do some testing to determine if it is better to destroy all
    /// cleared resources is preferrable to letting wgpu internal handle it lazily.
    pub fn clear<R>(&'a mut self)
    where
        R: Resource + 'a,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        self.inner.names_mut().clear();
        self.inner.store_mut().clear();
    }

    pub fn clear_all(&'a mut self) {
        self.inner.clear()
    }

    /// Removes resource from cache, returning the owned resource if found at handle.
    ///
    /// Note: It is up to the caller to destroy or drop the resource, with destroying
    /// immediately invalidating any use of the resource.
    pub fn remove<R>(&mut self, handle: R::Handle) -> Option<R>
    where
        R: Resource,
        ResourceCacheInner<'a>: ResourceStore<'a, R>,
    {
        let removed = self.inner.store_mut().remove(handle)?;
        let ident = self
            .inner
            .names()
            .iter()
            .find(|(_, &other_handle)| other_handle == handle)
            .map(|(&ident, _)| ident.to_owned())?;
        let _ = self.inner.names_mut().remove(ident.as_str());
        Some(removed)
    }
}

// Private inner cache containing direct resource stores.
#[derive(Debug)]
pub struct ResourceCacheInner<'a> {
    textures: HopSlotMap<TextureHandle, Texture>,
    texture_views: SecondaryMap<TextureHandle, Vec<wgpu::TextureView>>,
    texture_idents: FastHashMap<&'a str, TextureHandle>,
    buffers: HopSlotMap<BufferHandle, Buffer>,
    buffer_idents: FastHashMap<&'a str, BufferHandle>,
}

// Inner methods not generic over Resource.
impl<'a> ResourceCacheInner<'a> {
    fn new() -> Self {
        Self {
            textures: HopSlotMap::with_key(),
            texture_views: SecondaryMap::new(),
            texture_idents: FastHashMap::default(),
            buffers: HopSlotMap::with_key(),
            buffer_idents: FastHashMap::default(),
        }
    }

    /// Clears entire cache for every resource type.
    fn clear(&mut self) {
        self.textures.clear();
        self.texture_views.clear();
        self.texture_idents.clear();
        self.buffers.clear();
        self.buffer_idents.clear();
    }
}

/// Trait for directly accessing stores for a certain Resource type.
///
/// This allows the public api of ResourceCache to be generic across Resource
/// while keeping direct access to these stores private.
pub trait ResourceStore<'a, R: Resource> {
    /// For read operations on Resource instance(s).
    fn store(&self) -> &HopSlotMap<R::Handle, R>;
    /// For mutating operations on Resource instance(s).
    fn store_mut(&mut self) -> &mut HopSlotMap<R::Handle, R>;
    /// For read operations on Resource identifier(s).
    fn names(&self) -> &FastHashMap<&'a str, R::Handle>;
    /// For mutating operations on Resource identifier(s).
    fn names_mut(&mut self) -> &mut FastHashMap<&'a str, R::Handle>;
}

// Resource store implementations for ResourceCacheInner
// While this is very macro'able, there will only be three anyway.

// Resource store for buffer.
// Accesses buffers, buffer_idents from ResourceCacheInner
impl<'a> ResourceStore<'a, Buffer> for ResourceCacheInner<'a> {
    #[inline(always)]
    fn store(&self) -> &HopSlotMap<<Buffer as Resource>::Handle, Buffer> {
        &self.buffers
    }

    #[inline(always)]
    fn store_mut(&mut self) -> &mut HopSlotMap<<Buffer as Resource>::Handle, Buffer> {
        &mut self.buffers
    }

    #[inline(always)]
    fn names(&self) -> &FastHashMap<&'a str, <Buffer as Resource>::Handle> {
        &self.buffer_idents
    }

    #[inline(always)]
    fn names_mut(&mut self) -> &mut FastHashMap<&'a str, <Buffer as Resource>::Handle> {
        &mut self.buffer_idents
    }
}

// Resource store for buffer.
// Accesses textures, texture_idents, texture_views from ResourceCacheInner
impl<'a> ResourceStore<'a, Texture> for ResourceCacheInner<'a> {
    #[inline(always)]
    fn store(&self) -> &HopSlotMap<<Texture as Resource>::Handle, Texture> {
        &self.textures
    }

    #[inline(always)]
    fn store_mut(&mut self) -> &mut HopSlotMap<<Texture as Resource>::Handle, Texture> {
        &mut self.textures
    }

    #[inline(always)]
    fn names(&self) -> &FastHashMap<&'a str, <Texture as Resource>::Handle> {
        &self.texture_idents
    }

    #[inline(always)]
    fn names_mut(&mut self) -> &mut FastHashMap<&'a str, <Texture as Resource>::Handle> {
        &mut self.texture_idents
    }
}
