#[derive(Debug)]
pub struct Runner {
    pub render_pipeline: wgpu::RenderPipeline,
}

impl Runner {
    pub fn render_frame(&self, context: &crate::context::Context) {
        let output = context.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&Default::default());
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                ..Default::default()
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 1.0,
                            g: 1.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..6, 0..1);
        }
        context.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }
}
