use std::{cell::RefCell, rc::Rc};

use super::Error;

use crate::{
    bundle,
    pipeline::{AnyPipeline, Pipeline},
    resource,
};

#[derive(Debug)]
pub struct Context {
    /// Internal wgpu system resources
    pub(crate) system: crate::system::System,
    /// Winit event loop for creating new windows
    pub(crate) event_loop: winit::event_loop::EventLoop<()>,
    /// Project file cache
    pub(crate) files: gpu_common::PrebuildResult,
    pub(crate) resources: resource::Resources,
    pub(crate) bundles: bundle::Bundles,
    pub(crate) pipelines: Option<Vec<AnyPipeline>>,
}

impl Context {
    pub async fn new() -> Result<Context, Error> {
        gpu_log::error!("test");
        let system = crate::system::System::new().await?;
        let resources = Rc::new(RefCell::new(resource::ResourceCache::default()));
        let bundles = Rc::new(RefCell::new(bundle::BundleCache::new(resources.clone())));

        let context = Context {
            system,
            event_loop: winit::event_loop::EventLoop::new(),
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

        let start_time = chrono::Utc::now();
        gpu_log::info!("Initializing resources");
        // TODO: Initilize resources

        // Initialize bundles and store in bundle cache.
        gpu_log::info!("Initializing bundles");
        self.bundles.borrow_mut().build_from_runner(self, &runner)?;

        gpu_log::info!("Initializing pipelines");
        // Build pipelines
        let mut pipelines = Vec::new();
        for pipeline_arg in runner.pipelines.into_iter() {
            let pipeline = AnyPipeline::from_args(
                &self.files,
                &self.system.device,
                self.resources.clone(),
                &pipeline_arg,
            )?;
            pipelines.push(pipeline);
        }

        self.pipelines = Some(pipelines);

        let _elapsed_time = chrono::Utc::now().timestamp_millis() - start_time.timestamp_millis();
        gpu_log::info!("Build finished in {_elapsed_time}ms");
        Ok(())
    }

    pub async fn render(&mut self) -> Result<(), Error> {
        self.system.state.tick();
        let Some(pipelines) = &mut self.pipelines else {
            return Err(Error::NoRunner);
        };
        // Allow bundles to write/modify resources.
        self.bundles
            .borrow_mut()
            .on_frame_start(&self.system)
            .map_err(Error::Bundle)?;
        let mut encoder = self
            .system
            .device
            .create_command_encoder(&Default::default());
        // Dispatch all pipelines
        pipelines
            .iter_mut()
            .for_each(|pipeline| pipeline.dispatch(&mut encoder));
        self.system.queue.submit(std::iter::once(encoder.finish()));
        // Allow bundles to read from resources.
        self.bundles
            .borrow_mut()
            .on_frame_end(&self.system)
            .map_err(Error::Bundle)?;
        Ok(())
    }
}
