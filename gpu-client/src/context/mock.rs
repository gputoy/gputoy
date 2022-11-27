use crate::resource::ResourceCache;

use super::Error;
use winit::{
    event_loop::{EventLoop, EventLoopBuilder},
    platform::unix::EventLoopBuilderExtUnix,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct MockContext<'a> {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub runner: Option<crate::runner::Runner>,
    pub resources: ResourceCache<'a>,
}

impl<'a> MockContext<'a> {
    pub async fn new() -> Result<MockContext<'a>, Error> {
        let backend = wgpu::Backends::PRIMARY;
        let instance = wgpu::Instance::new(backend);
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: Default::default(),
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .expect("request_adapter to suceed");

        let features = adapter.features();
        let limits = adapter.limits();

        let desc = wgpu::DeviceDescriptor {
            label: None,
            features,
            limits,
        };
        let (device, queue) = adapter.request_device(&desc, None).await?;
        let resources = ResourceCache::new();

        Ok(MockContext {
            adapter,
            device,
            queue,
            runner: None,
            instance,
            resources,
        })
    }

    pub fn has_runner(&self) -> bool {
        self.runner.is_some()
    }
}
