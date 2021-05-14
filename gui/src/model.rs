use std::borrow::Cow;

pub struct Model {
    render_pipeline: wgpu::RenderPipeline,
}

impl Model {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat) -> Self {
        // Load the shaders from disk
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("model.wgsl"))),
            flags: wgpu::ShaderFlags::all(),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main",
                    targets: &[swapchain_format.into()],
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
        });

        Model {
            render_pipeline,
        }
    }

    pub fn draw(&self, frame: &wgpu::SwapChainTexture, encoder: &mut wgpu::CommandEncoder) {
        let mut rpass = encoder.begin_render_pass(
            &wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.draw(0..3, 0..1);
    }
}
