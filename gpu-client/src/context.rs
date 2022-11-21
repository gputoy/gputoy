use gpu_common::PrebuildResult;
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
    #[error("Project build failed")]
    ProjectBuildFailed,
    #[error("No runner")]
    NoRunner,
    #[error(transparent)]
    CompileError(gpu_compiler::Error),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Context {
    pub window: winit::window::Window,
    pub event_loop: EventLoop<()>,
    pub instance: wgpu::Instance,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub runner: Option<crate::runner::Runner>,
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
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &surface_config);

        Ok(Context {
            event_loop,
            window,
            adapter,
            device,
            queue,
            instance,
            size,
            surface,
            runner: None,
        })
    }

    pub async fn build(
        &mut self,
        project: &gpu_common::Project,
        prebuild_result: PrebuildResult,
    ) -> Result<(), Error> {
        let fs = &prebuild_result
            .file_builds
            .get("/shaders/main.wgsl")
            .ok_or(Error::ProjectBuildFailed)?
            .processed_shader;
        let vs = &prebuild_result
            .file_builds
            .get("/shaders/types.wgsl")
            .ok_or(Error::ProjectBuildFailed)?
            .processed_shader;
        let vertex = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(vs.into()),
            });
        let shader_desc = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::ShaderSource::Wgsl(fs.into()),
            });
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Pipeline layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });
        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &vertex,
                    entry_point: "main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader_desc,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: self.surface.get_supported_formats(&self.adapter)[0],
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
                depth_stencil: None,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Cw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    conservative: false,
                },
            });

        self.runner = Some(crate::runner::Runner { render_pipeline });
        Ok(())
    }

    pub async fn render(&self) -> Result<(), Error> {
        let Some(runner) = &self.runner else {
            return Err(Error::NoRunner);
        };
        runner.render_frame(self);
        Ok(())
    }

    pub fn has_runner(&self) -> bool {
        self.runner.is_some()
    }
}
