use crate::Error;

mod state;
pub use state::SystemState;

#[derive(Debug)]
pub struct System {
    /// Current state of system run
    pub(crate) state: state::SystemState,
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl System {
    pub async fn new() -> Result<Self, Error> {
        #[cfg(target_arch = "wasm32")]
        let backend = wgpu::Backends::BROWSER_WEBGPU;
        #[cfg(not(target_arch = "wasm32"))]
        let backend = wgpu::Backends::PRIMARY;
        let instance = wgpu::Instance::new(backend);

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, None)
            .await
            .ok_or(Error::NoAdapter)?;

        let features = adapter.features();
        gpu_log::debug!("Adapter features: {features:#?}");

        let downlevel_capabilities = adapter.get_downlevel_capabilities();
        gpu_log::debug!("Adapter downlevel capabilities: {downlevel_capabilities:#?}");

        let limits = adapter.limits();
        gpu_log::debug!("Adapter limits: {limits:#?}");

        let desc = wgpu::DeviceDescriptor {
            label: None,
            features,
            limits,
        };
        let (device, queue) = adapter.request_device(&desc, None).await?;

        Ok(Self {
            state: SystemState::default(),
            instance,
            adapter,
            device,
            queue,
        })
    }
}
