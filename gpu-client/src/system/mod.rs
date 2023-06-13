use crate::Error;

mod state;
pub use state::SystemState;
use wgpu::Limits;

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
        let instance = wgpu::Instance::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                // Request an adapter which can render to our surface
                compatible_surface: None,
            })
            .await
            .ok_or(Error::NoAdapter)?;

        gpu_log::debug!("Adapter limits: {limits:#?}");

        #[cfg(target_arch = "wasm32")]
        let features = wgpu::Features::empty();
        #[cfg(not(target_arch = "wasm32"))]
        let features = wgpu::Features::all_native_mask();

        let desc = wgpu::DeviceDescriptor {
            label: None,
            features,
            limits: Limits::downlevel_webgl2_defaults(),
        };
        gpu_log::info!("Requesting device");
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
