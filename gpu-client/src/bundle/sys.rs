use bytemuck::{Pod, Zeroable};
use thiserror::Error;

use crate::resource::{Buffer, BufferHandle};

use super::Bundle;

const SYSTEM_BUNDLE_IDENTIFIER: &str = "system";

#[derive(Debug)]
pub struct SystemBundle {
    sys_buffer_handle: BufferHandle,
}

#[derive(Debug, Error)]
pub enum SystemBundleError {
    #[error("System buffer missing")]
    MissingBuffer,
}

#[derive(Debug, Copy, Clone, Pod, Zeroable)]
#[repr(C)]
struct SysBuffer {
    time: f32,
    dt: f32,
    frame: u64,
}

impl Bundle for SystemBundle {
    type Args = ();
    type ResourceKeys = gpu_common::SystemBundleResources;
    type Error = SystemBundleError;

    const MAX_INSTANCES: u32 = 1;

    fn new(ctx: &crate::Context, ident: String, _args: &Self::Args) -> Result<Self, Self::Error> {
        let mut resources = ctx.resources.borrow_mut();
        let handle = resources.insert_from_args::<Buffer>(
            ctx,
            &gpu_common::BufferArgs {
                label: ident,
                binding_type: gpu_common::BufferBindingType::Uniform,
                mapped_at_creation: false,
                size: std::mem::size_of::<SysBuffer>() as u32,
                usage: gpu_common::BufferUsages::UNFIORM | gpu_common::BufferUsages::COPY_DST,
            },
        );

        Ok(Self {
            sys_buffer_handle: handle,
        })
    }

    fn on_frame_start(
        &mut self,
        sys: &crate::system::System,
        resources: &crate::resource::Resources,
    ) -> Result<(), Self::Error> {
        let resources = resources.borrow();
        let buffer = resources
            .get::<Buffer>(self.sys_buffer_handle)
            .ok_or(Self::Error::MissingBuffer)?;

        let buffer_data = SysBuffer {
            time: sys.state.time_elapsed as f32,
            dt: sys.state.delta as f32,
            frame: sys.state.frame_num,
        };
        let buffer_data = bytemuck::bytes_of(&buffer_data);

        buffer.write(sys, buffer_data);
        Ok(())
    }

    fn destroy(&mut self) {}
}

impl SystemBundle {
    /// Conveinence constructor. This bundle will only ever be constructed by the context, so it
    /// can deviate from the bundle trait.
    pub fn new(ctx: &crate::Context) -> SystemBundle {
        // unwrap ok because the bundle constructor above is infallible
        <Self as Bundle>::new(ctx, String::from(SYSTEM_BUNDLE_IDENTIFIER), &()).unwrap()
    }
}
