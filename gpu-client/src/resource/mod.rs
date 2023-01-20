use slotmap::{new_key_type, Key};
use std::{cell::RefCell, rc::Rc};

use crate::context::Context;

mod buffer;
mod cache;
mod sampler;
mod texture;

pub use buffer::{BufferArgs, BufferResource as Buffer};
pub use cache::ResourceCache;
pub use texture::{TextureArgs, TextureResource as Texture};

new_key_type! { pub struct TextureHandle; }
new_key_type! { pub struct BufferHandle; }

/// Resource store. Safe to share in multiple parts of the context.
pub type Resources = Rc<RefCell<cache::ResourceCache>>;

pub enum AnyResource {
    Buffer(Buffer),
    Texture(Texture),
}

pub trait SubResource {}

/// A gpu-bound resource
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
