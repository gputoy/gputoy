use gpu_common::ViewportBundleArgs;
use thiserror::Error;

use winit::{event_loop::EventLoop, window::WindowBuilder};

use super::Bundle;

#[derive(Debug)]
pub struct ViewportBundle {
    window: winit::window::Window,
    surface: wgpu::Surface,
    // mouse_buffer_handle: BufferHandle,
    // resolution_buffer_handle: BufferHandle,
    args: ViewportBundleArgs,
}

#[derive(Debug, Error)]
pub enum ViewportError {
    #[error("Could not create window: {0}")]
    WindowCreation(winit::error::OsError),
}

impl ViewportBundle {
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }
}

impl Bundle for ViewportBundle {
    type Args = gpu_common::ViewportBundleArgs;

    type Error = ViewportError;

    fn new(ctx: &crate::context::Context, args: &Self::Args) -> Result<Self, Self::Error> {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .build(&event_loop)
            .map_err(Self::Error::WindowCreation)?;
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::WindowExtWebSys;
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.get_element_by_id(&args.target))
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
        let surface = unsafe { ctx.instance.create_surface(&window) };
        let size = window.inner_size();
        let preferred_format = surface.get_supported_formats(&ctx.adapter)[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: preferred_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&ctx.device, &surface_config);
        Ok(Self {
            window,
            surface,
            args: args.clone(),
        })
    }

    fn destroy(&mut self) {}

    fn on_run_start(&mut self) {
        todo!()
    }

    fn on_run_end(&mut self) {
        todo!()
    }

    fn on_frame_start(&mut self) {
        todo!()
    }

    fn on_frame_end(&mut self) {
        todo!()
    }
}
