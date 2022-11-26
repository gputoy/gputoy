use crate::context::Context;

use super::{Resource, SubResource, TextureHandle};

#[derive(Debug)]
pub struct TextureResource {
    texture: wgpu::Texture,
    args: gpu_common::TextureArgs,
}

pub struct TextureView {}
impl SubResource for TextureView {}

impl Resource for TextureResource {
    type Args = gpu_common::TextureArgs;
    type SubResource = TextureView;
    type Handle = TextureHandle;

    const LABEL: &'static str = "texture";
    const SHADER_DECL: &'static str = "";

    fn new(ctx: &Context, args: &Self::Args) -> Self {
        let texture = ctx.device.create_texture(&args.into());
        Self {
            texture,
            args: args.clone(),
        }
    }
    fn destroy(self) {
        self.texture.destroy();
    }
    fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: wgpu::BindingResource::TextureView(todo!()),
        }
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

impl TextureResource {
    pub fn create_view(&self) -> wgpu::TextureView {
        self.texture.create_view(&self.args().into())
    }
}
