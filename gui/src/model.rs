use std::borrow::Cow;
use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;
use step::triangulate::{Vertex, Triangle};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GPUVertex {
    pos: [f32; 4],
}

pub struct Model {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: u32,
    render_pipeline: wgpu::RenderPipeline,
}

impl Model {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat,
               verts: &[Vertex], tris: &[Triangle]) -> Self {

        let vertex_data: Vec<GPUVertex> = verts.into_iter()
            .map(|v| GPUVertex {
                pos: [v.pos.x as f32, v.pos.y as f32, v.pos.z as f32, 1.0]
            })
            .collect();
        let index_data: Vec<u32> = tris.into_iter()
            .flat_map(|t| t.verts.iter())
            .copied()
            .collect();

        let vertex_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buf = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsage::INDEX,
        });

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

        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GPUVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        }];

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &vertex_buffers,
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
            index_buf,
            vertex_buf,
            index_count: tris.len() as u32,
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
        rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint32);
        rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        rpass.draw(0..self.index_count, 0..1);
    }
}
