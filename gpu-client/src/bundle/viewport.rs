use thiserror::Error;

use winit::window::WindowBuilder;

use crate::resource;

use super::Bundle;

#[derive(Debug)]
pub struct ViewportBundle {
    _window: winit::window::Window,
    surface: wgpu::Surface,
    surface_texture_handle: crate::resource::TextureHandle,
    // mouse_buffer_handle: BufferHandle,
    // resolution_buffer_handle: BufferHandle,
    _args: gpu_common::bundle::viewport::Args,
}

#[derive(Debug, Error)]
pub enum ViewportError {
    #[error("Could not create window: {0}")]
    WindowCreation(winit::error::OsError),
    #[error(transparent)]
    Surface(wgpu::SurfaceError),
    #[error("Missing surface texture in resource cache")]
    MissingSurfaceTexture,
}

impl ViewportBundle {
    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }
}

impl Bundle for ViewportBundle {
    type Proto = gpu_common::bundle::viewport::Bundle;
    type Args = gpu_common::bundle::viewport::Args;
    type ResourceKeys = gpu_common::bundle::viewport::Resources;
    type Error = ViewportError;

    const MAX_INSTANCES: u32 = 4;

    fn new(ctx: &crate::Context, ident: String, args: &Self::Args) -> Result<Self, Self::Error> {
        let window = WindowBuilder::new()
            .build(&ctx.event_loop)
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

        gpu_log::info!("Initializing surface {}", args.target);
        let surface = unsafe { ctx.system.instance.create_surface(&window).unwrap() };

        let capabilities = surface.get_capabilities(&ctx.system.adapter);
        let preferred_format = capabilities.formats[0];

        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: preferred_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        // Initialize surface for presentation.
        surface.configure(&ctx.system.device, &surface_config);

        // Create texture resource from surface.
        let surface_texture = crate::resource::Texture::from_surface(
            &surface,
            gpu_common::TextureArgs {
                label: format!("viewport_surface_{}", &ident),
                dimension: gpu_common::ImageDimension::D2,
                format: preferred_format.into(),
                size: [size.width, size.height, 1],
                mip_level_count: 1,
                sample_count: 1,
                usage: gpu_common::TextureUsages::RENDER_ATTACHMENT,
            },
        )
        .map_err(ViewportError::Surface)?;

        // Insert texture surface with full identifier i.e. '{viewport_ident}::surface'.
        let full_ident = Self::prefix_key_with_ident(&ident, Self::ResourceKeys::Surface);
        let mut resources = ctx.resources.borrow_mut();
        let surface_texture_handle = resources.insert_with_ident(surface_texture, &full_ident);

        Ok(Self {
            _window: window,
            surface,
            _args: args.clone(),
            surface_texture_handle,
        })
    }

    fn destroy(&mut self) {}

    fn on_frame_start(
        &mut self,
        _sys: &crate::system::System,
        resources: &resource::Resources,
    ) -> Result<(), Self::Error> {
        // Swap surface texture in resource cache.
        // Pipelines with handles to this resource will point to the updated swapchain texture.
        resources
            .borrow_mut()
            .get_mut::<crate::resource::Texture>(self.surface_texture_handle)
            .ok_or(ViewportError::MissingSurfaceTexture)?
            .replace_from_surface(&self.surface)
            .map_err(ViewportError::Surface)
    }
}
