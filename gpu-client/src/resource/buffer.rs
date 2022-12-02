use super::{BufferHandle, SubResource};

#[derive(Debug)]
pub struct BufferResource {
    buffer: wgpu::Buffer,
    args: gpu_common::BufferArgs,
}
pub struct BufferSubresource;
impl SubResource for BufferSubresource {}

impl super::Resource for BufferResource {
    type Args = gpu_common::BufferArgs;
    type SubResource = BufferSubresource;
    type Handle = BufferHandle;

    const LABEL: &'static str = "buffer";
    const SHADER_DECL: &'static str = "";

    fn new(ctx: &crate::Context, args: &Self::Args) -> Self {
        let buffer = ctx.device.create_buffer(&args.into());
        Self {
            buffer,
            args: args.clone(),
        }
    }
    fn destroy(self) {
        self.buffer.destroy();
    }
    fn bind_group_entry(&self, binding: u32) -> wgpu::BindGroupEntry {
        wgpu::BindGroupEntry {
            binding,
            resource: self.buffer.as_entire_binding(),
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
            ty: wgpu::BindingType::Buffer {
                ty: self.args.binding_type.into(),
                has_dynamic_offset: false,
                min_binding_size: None,
            },
        }
    }

    fn args(&self) -> &Self::Args {
        &self.args
    }
}
