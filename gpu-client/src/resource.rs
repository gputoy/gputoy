use serde::{Deserialize, Serialize};

use crate::context::Context;

pub trait Resource {
    type Args: Sized + Serialize + Deserialize<'static>;
    const LABEL: &'static str;
    const SHADER_DECL: &'static str;

    fn new(ctx: Context, args: &Self::Args) -> Self;

    fn destroy(self);

    fn bind_group_entry(&self, binding: usize) -> wgpu::BindGroupEntry;

    fn bind_group_layout_entry(
        &self,
        binding: usize,
        visibility: usize,
    ) -> wgpu::BindGroupLayoutEntry;

    fn write(buf: &[u8], queue: wgpu::Queue);
}
