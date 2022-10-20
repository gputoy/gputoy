use thiserror::Error;

use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Placeholder error!")]
    Placeholder,
    #[error("Could not create window: {0}")]
    WindowCreation(winit::error::OsError),
    #[error("Failed to obtain adapter")]
    NoAdapter,
    #[error(transparent)]
    DeviceCreationFailed(#[from] wgpu::RequestDeviceError),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Context {
    window: winit::window::Window,
    event_loop: EventLoop<()>,
    instance: wgpu::Instance,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Context {
    pub async fn new() -> Result<Context, Error> {
        log::info!("Calling new context from rust");

        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .map_err(Error::WindowCreation)?;

        let backend = wgpu::Backends::BROWSER_WEBGPU;
        let instance = wgpu::Instance::new(backend);

        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            let query_string = web_sys::window().unwrap().location().search().unwrap();
            log::info!("query_String: {query_string}");
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.get_element_by_id("canvas-root"))
                .and_then(|root| {
                    let element = web_sys::Element::from(window.canvas());
                    let _ = element.set_id("canvas");
                    let _ = element.set_attribute("width", "");
                    let _ = element.set_attribute("height", "");
                    root.append_child(&element).ok()
                })
                .expect("couldn't append canvas to document body");
        }

        log::info!("Initializing the surface...");
        #[cfg(not(target_arch = "wasm32"))]
        let surface = unsafe { instance.create_surface(&window) };
        #[cfg(target_arch = "wasm32")]
        let surface = unsafe { instance.create_surface(&window) };

        let size = window.inner_size();

        let adapter = wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, None)
            .await
            .ok_or(Error::NoAdapter)?;

        let features = adapter.features();
        log::debug!("Adapter features: {features:?}");

        let downlevel_capabilities = adapter.get_downlevel_capabilities();
        log::debug!("Adapter downlevel capabilities: {downlevel_capabilities:?}");

        let limits = adapter.limits();
        log::debug!("Adapter limits: {limits:?}");

        let desc = wgpu::DeviceDescriptor {
            label: None,
            features,
            limits,
        };
        let (device, queue) = adapter.request_device(&desc, None).await?;

        Ok(Context {
            event_loop,
            window,
            adapter,
            device,
            queue,
            instance,
            size,
            surface,
        })
    }

    pub async fn build(&mut self, _project: &gpu_common::Project) -> Result<(), Error> {
        Ok(())
    }

    pub async fn render(&self) -> Result<(), Error> {
        Ok(())
    }
}
