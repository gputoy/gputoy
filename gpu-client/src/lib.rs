pub mod bundle;
pub mod context;
pub mod resource;
pub mod runner;

#[cfg(test)]
mod tests {
    use super::context::Context;
    use super::resource::{self, Resource};

    fn make_buffer_args() -> gpu_common::BufferArgs {
        gpu_common::BufferArgs {
            binding_type: gpu_common::BufferBindingType::Uniform,
            label: String::from("TestBuffer"),
            mapped_at_creation: false,
            size: 1,
            usage: gpu_common::sys::BufferUsages::UNFIORM | gpu_common::sys::BufferUsages::COPY_DST,
        }
    }

    fn make_context<'a>() -> Context<'a> {
        tokio_test::block_on(async {
            crate::context::Context::new()
                .await
                .expect("context new to suceed")
        })
    }

    #[test]
    fn test_resource_cache() {
        let mut cache = resource::ResourceCache::new();
        let ctx = make_context();
        let args = make_buffer_args();
        let buffer = resource::Buffer::new(&ctx, &args);
        let buffer_handle = cache.insert(buffer);
        assert_eq!(cache.iter::<resource::Buffer>().count(), 1);
        let _ = cache.remove::<resource::Buffer>(buffer_handle);
        assert_eq!(cache.iter::<resource::Buffer>().count(), 0);
    }

    #[test]
    fn debug_test_buffer_resource() {
        let ctx = make_context();
        let args = make_buffer_args();
        let buffer = resource::Buffer::new(&ctx, &args);
        println!("built a buffer! {buffer:?}");
        println!(
            "buffer layout entry: {:?}",
            buffer.bind_group_layout_entry(3, wgpu::ShaderStages::COMPUTE)
        );
        println!("buffer bind group entry: {:?}", buffer.bind_group_entry(3));
        assert_eq!((), buffer.destroy());
    }
}
