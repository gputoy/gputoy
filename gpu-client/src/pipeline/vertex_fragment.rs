use gpu_common::PrebuildResult;
use wgpu::CommandEncoder;

use crate::resource;

use super::{Pipeline, PipelineError};

#[derive(Debug)]
pub struct VertexFragment {
    _vs: wgpu::ShaderModule,
    _fs: wgpu::ShaderModule,
    pipeline: wgpu::RenderPipeline,
    target_handles: Vec<resource::TextureHandle>,
    resources: resource::Resources,
}

impl Pipeline for VertexFragment {
    type Args = gpu_common::VertexFragment;

    fn from_args(
        files: &PrebuildResult,
        device: &wgpu::Device,
        resources: resource::Resources,
        args: &Self::Args,
    ) -> Result<Self, PipelineError> {
        // Fetch shader files from file cache.
        let vs = &files
            .file_builds
            .get(&args.vertex)
            .ok_or(PipelineError::FileNotFound(args.vertex.clone()))?
            .processed_shader;
        let fs = &files
            .file_builds
            .get(&args.fragment)
            .ok_or(PipelineError::FileNotFound(args.fragment.clone()))?
            .processed_shader;

        // Create wgpu shader modules from processed source.
        let vs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(vs.into()),
        });
        let fs = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(fs.into()),
        });

        // Fetch target textures from resource cache, while
        // remembering the handles for faster future access.
        let mut target_textures = Vec::new();
        let mut target_handles = Vec::new();

        let resources_borrow = resources.borrow();
        for target in args.targets.iter() {
            let (handle, resource) = resources_borrow
                .get_by_ident::<resource::Texture>(&target)
                .ok_or(PipelineError::ResourceNotFound(target.to_owned()))?;
            target_handles.push(handle);
            target_textures.push(resource);
        }

        // Create ColorTargetState array for the pipeline from texture format.
        let targets = target_textures
            .iter()
            .map(|texture| {
                Some(wgpu::ColorTargetState {
                    format: texture.format().into(),
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })
            })
            .collect::<Vec<_>>();

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs,
                entry_point: "main",
                targets: targets.as_slice(),
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

        Ok(VertexFragment {
            _vs: vs,
            _fs: fs,
            pipeline,
            target_handles,
            resources: resources.clone(),
        })
    }

    fn dispatch(&mut self, encoder: &mut CommandEncoder) {
        // Create color pass attachments from textures
        // TODO: Determine if this needs to be fetched on dispatch.
        // I suspect this would only apply to surface textures.

        let views = {
            let resources = self.resources.borrow();
            self.target_handles
                .iter()
                .map(|handle| {
                    resources
                        .get::<resource::Texture>(*handle)
                        .map(resource::Texture::create_view)
                })
                .collect::<Vec<_>>()
        };

        let color_pass_attachments = views
            .iter()
            .map(|view| {
                view.as_ref().map(|view| wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: true,
                    },
                })
            })
            .collect::<Vec<_>>();

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: color_pass_attachments.as_slice(),
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&self.pipeline);
        render_pass.draw(0..6, 0..1);
    }
}
