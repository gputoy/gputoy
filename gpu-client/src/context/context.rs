use std::{cell::RefCell, rc::Rc};

use gpu_common::BundleArgs;

use super::Error;

use crate::{
    bundle::{self, Bundle},
    pipeline::{AnyPipeline, Pipeline},
    resource,
};

#[derive(Debug)]
pub struct Context {
    pub(crate) instance: wgpu::Instance,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) files: gpu_common::PrebuildResult,
    pub(crate) resources: resource::Resources,
    pub(crate) bundles: bundle::BundleCache,
    pub(crate) pipelines: Option<Vec<AnyPipeline>>,
}

impl Context {
    pub async fn new() -> Result<Context, Error> {
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
        let resources = Rc::new(RefCell::new(resource::ResourceCache::new()));
        let bundles = bundle::BundleCache::new(resources.clone());

        let context = Context {
            adapter,
            device,
            queue,
            instance,
            resources,
            bundles,
            files: Default::default(),
            pipelines: None,
        };

        Ok(context)
    }

    pub async fn build(
        &mut self,
        runner: gpu_common::Runner,
        prebuild_result: gpu_common::PrebuildResult,
    ) -> Result<(), Error> {
        // Processed shaders for other parts to access.
        self.files = prebuild_result;

        // TODO: Initilize resources

        // Initialize bundles and store in bundle cache.
        for (ident, arg) in runner.bundles.iter() {
            match arg {
                BundleArgs::Viewport(arg) => self.bundles.insert::<bundle::Viewport>(
                    bundle::Viewport::new(self, ident.to_owned(), &arg)
                        .map_err(bundle::BundleError::Viewport)
                        .map_err(Error::Bundle)?,
                ),
            }
        }

        // Build pipelines
        let mut pipelines = Vec::new();
        for pipeline_arg in runner.pipelines.into_iter() {
            let pipeline = AnyPipeline::from_args(
                &self.files,
                &self.device,
                self.resources.clone(),
                &pipeline_arg,
            )
            .map_err(Error::Pipeline)?;
            pipelines.push(pipeline);
        }

        self.pipelines = Some(pipelines);
        Ok(())
    }

    pub async fn render(&mut self) -> Result<(), Error> {
        let Some(pipelines) = &mut self.pipelines else {
            return Err(Error::NoRunner);
        };
        // Allow bundles to write/modify resources.
        self.bundles.on_frame_start();
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Dispatch all pipelines
        pipelines
            .iter_mut()
            .for_each(|pipeline| pipeline.dispatch(&mut encoder));
        self.queue.submit(std::iter::once(encoder.finish()));
        // Allow bundles to read from resources.
        self.bundles.on_frame_end();
        Ok(())
    }
}
