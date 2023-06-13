use crate::Context;

use super::{Resource, SubResource, TextureHandle};

pub type TextureArgs = gpu_common::TextureArgs;

#[derive(Debug)]
enum TextureInner {
    /// Standard wgpu texture.
    ///
    /// This variant can only be generated from user arguments or bundles.
    Texture(wgpu::Texture),

    /// Contains result of `get_current_texture` for [`wgpu::Surface`]
    ///
    /// By storing the surface texture this way, the surface can be decoupled from the surface texture
    /// and surface textures can be queried exactly like any other texture.
    Surface(wgpu::SurfaceTexture),
}

#[derive(Debug)]
pub struct TextureResource {
    texture: TextureInner,
    args: TextureArgs,
}

pub struct TextureView {}
impl SubResource for TextureView {}

impl TextureResource {
    pub fn from_raw(texture: wgpu::Texture, args: gpu_common::TextureArgs) -> Self {
        Self {
            texture: TextureInner::Texture(texture),
            args,
        }
    }
    pub(crate) fn from_surface(
        surface: &wgpu::Surface,
        args: gpu_common::TextureArgs,
    ) -> Result<Self, wgpu::SurfaceError> {
        let surface_texture = surface.get_current_texture()?;
        Ok(Self {
            texture: TextureInner::Surface(surface_texture),
            args,
        })
    }
    pub fn create_view(&self) -> wgpu::TextureView {
        match &self.texture {
            TextureInner::Texture(texture) => texture.create_view(&self.args().into()),
            TextureInner::Surface(surface_texture) => {
                surface_texture.texture.create_view(&self.args().into())
            }
        }
    }

    pub fn format(&self) -> gpu_common::TextureFormat {
        self.args.format
    }

    /// Replace inner texture with current texture from surface.
    ///
    /// Should only be used by [`crate::bundle::Viewport`].
    pub(crate) fn replace_from_surface(
        &mut self,
        surface: &wgpu::Surface,
    ) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = surface.get_current_texture()?;
        self.texture = TextureInner::Surface(surface_texture);
        Ok(())
    }
}

impl Resource for TextureResource {
    type Args = TextureArgs;
    type SubResource = TextureView;
    type Handle = TextureHandle;

    const LABEL: &'static str = "texture";
    const SHADER_DECL: &'static str = "";

    fn new(ctx: &Context, args: &Self::Args) -> Self {
        let texture = TextureInner::Texture(ctx.system.device.create_texture(&args.into()));
        Self {
            texture,
            args: args.clone(),
        }
    }

    fn destroy(self) {}

    fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        // shut clippy up
        let _ = binding;
        todo!()
    }

    fn bind_group_layout_entry(
        &self,
        binding: u32,
        visibility: wgpu::ShaderStages,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding,
            visibility,
            count: None,
            ty: wgpu::BindingType::Texture {
                sample_type: self.args.format.into(),
                view_dimension: self.args().into(),
                multisampled: self.args.sample_count > 1,
            },
        }
    }

    fn args(&self) -> &Self::Args {
        &self.args
    }
}
