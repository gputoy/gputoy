use super::Error;

use crate::{bundle, bundle::Bundle, resource};

#[derive(Debug)]
pub struct Context<'a> {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) runner: Option<crate::runner::Runner>,
    pub(crate) resources: resource::ResourceCache<'a>,
    pub(crate) bundles: bundle::BundleCache,
}

impl<'a> Context<'a> {
    pub async fn new() -> Result<Context<'a>, Error> {
        #[cfg(target_arch = "wasm32")]
        let backend = wgpu::Backends::BROWSER_WEBGPU;
        #[cfg(not(target_arch = "wasm32"))]
        let backend = wgpu::Backends::PRIMARY;
        let instance = wgpu::Instance::new(backend);

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

        let resources = resource::ResourceCache::new();
        let bundles = bundle::BundleCache::new();

        Ok(Context {
            adapter,
            device,
            queue,
            instance,
            runner: None,
            resources,
            bundles,
        })
    }

    pub async fn build(
        &mut self,
        prebuild_result: gpu_common::PrebuildResult,
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

        let viewport = bundle::Viewport::new(
            &self,
            &gpu_common::ViewportBundleArgs {
                target: "canvas-root".to_owned(),
            },
        )
        .expect("Viewport bundle to work");

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
                        format: viewport.surface().get_supported_formats(&self.adapter)[0],
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

        self.bundles.insert(viewport);
        self.runner = Some(crate::runner::Runner { render_pipeline });
        Ok(())
    }

    pub async fn render(&mut self) -> Result<(), Error> {
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
