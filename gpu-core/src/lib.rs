// TODO: remove global cfg
// Rust-analyzer doesn't bother analyzing any code within
// a target_arch block
#![cfg(target_arch = "wasm32")]

use gpu_types as types;
use thiserror::Error;
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::{ImageBitmapRenderingContext, OffscreenCanvas};
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

#[wasm_bindgen]
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

#[cfg(target_arch = "wasm32")]
struct OffscreenCanvasSetup {
    offscreen_canvas: OffscreenCanvas,
    bitmap_renderer: ImageBitmapRenderingContext,
}

impl Context {
    pub async fn new() -> Result<Context, Error> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .map_err(Error::WindowCreation)?;

        let backend = wgpu::Backends::BROWSER_WEBGPU;
        let instance = wgpu::Instance::new(backend);

        {
            use winit::platform::web::WindowExtWebSys;
            let query_string = web_sys::window().unwrap().location().search().unwrap();
            log::info!("query_String: {query_string}");
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
        }

        #[cfg(target_arch = "wasm32")]
        let mut offscreen_canvas_setup: Option<OffscreenCanvasSetup> = None;

        #[cfg(target_arch = "wasm32")]
        {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowExtWebSys;

            let query_string = web_sys::window().unwrap().location().search().unwrap();
            log::debug!("Query string found: {}", query_string);
            if let Some(offscreen_canvas_param) =
                parse_url_query_string(&query_string, "offscreen_canvas")
            {
                if std::str::FromStr::from_str(offscreen_canvas_param) == Ok(true) {
                    log::info!("Creating OffscreenCanvasSetup");

                    let offscreen_canvas =
                        OffscreenCanvas::new(1024, 768).expect("couldn't create OffscreenCanvas");

                    let bitmap_renderer = window
                        .canvas()
                        .get_context("bitmaprenderer")
                        .expect("couldn't create ImageBitmapRenderingContext (Result)")
                        .expect("couldn't create ImageBitmapRenderingContext (Option)")
                        .dyn_into::<ImageBitmapRenderingContext>()
                        .expect("couldn't convert into ImageBitmapRenderingContext");

                    offscreen_canvas_setup = Some(OffscreenCanvasSetup {
                        offscreen_canvas,
                        bitmap_renderer,
                    })
                }
            }
        };

        #[cfg(not(target_arch = "wasm32"))]
        let surface = unsafe { instance.create_surface(&window) };
        #[cfg(target_arch = "wasm32")]
        let surface = if let Some(offscreen) = offscreen_canvas_setup {
            instance.create_surface_from_offscreen_canvas(&offscreen.offscreen_canvas)
        } else {
            unsafe { instance.create_surface(&window) }
        };
        log::info!("Initializing the surface...");

        let size = window.inner_size();

        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
                .await
                .ok_or(Error::NoAdapter)?;

        let features = adapter.features();
        log::info!("Adapter features: {features:?}");

        let downlevel_capabilities = adapter.get_downlevel_capabilities();
        log::info!("Adapter downlevel capabilities: {downlevel_capabilities:?}");

        let limits = adapter.limits();
        log::info!("Adapter limits: {limits:?}");

        let desc = wgpu::DeviceDescriptor {
            label: None,
            features: features,
            limits: limits,
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
}

/// Parse the query string as returned by `web_sys::window()?.location().search()?` and get a
/// specific key out of it.
pub fn parse_url_query_string<'a>(query: &'a str, search_key: &str) -> Option<&'a str> {
    let query_string = query.strip_prefix('?')?;

    for pair in query_string.split('&') {
        let mut pair = pair.split('=');
        let key = pair.next()?;
        let value = pair.next()?;

        if key == search_key {
            return Some(value);
        }
    }

    None
}
