use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};
use itertools::Itertools;
use nalgebra_glm as glm;
use glm::{Vec3, Mat4};
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
    uniform_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    index_count: u32,
    render_pipeline: wgpu::RenderPipeline,

    // Model matrix parameters
    aspect: f32, pitch: f32, yaw: f32, scale: f32, center: Vec3,
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

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            size: std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                },
            ],
        });

        let xb = verts.iter().map(|v| v.pos.x).minmax().into_option().unwrap();
        let yb = verts.iter().map(|v| v.pos.y).minmax().into_option().unwrap();
        let zb = verts.iter().map(|v| v.pos.z).minmax().into_option().unwrap();
        let dx = xb.1 - xb.0;
        let dy = xb.1 - xb.0;
        let dz = xb.1 - xb.0;
        let scale = (1.0 / dx.max(dy).max(dz)) as f32;
        let center = Vec3::new((xb.0 + xb.1) as f32 / 2.0,
                               (yb.0 + yb.1) as f32 / 2.0,
                               (zb.0 + zb.1) as f32 / 2.0);

        // Create pipeline layout
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let vertex_buf_layout = wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<GPUVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
            ],
        };

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buf.as_entire_binding(),
                },
            ],
            label: None,
        });

        // Load the shaders from disk
        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("model.wgsl"))),
            flags: wgpu::ShaderFlags::all(),
        });

        let render_pipeline = device.create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: &[vertex_buf_layout],
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
            uniform_buf,
            bind_group,
            index_count: tris.len() as u32 * 3,
            pitch: 0.0, yaw: 0.0, aspect: 1.0,
            scale, center,
        }
    }

    pub fn set_aspect(&mut self, a: f32) {
        self.aspect = a;
    }

    fn generate_matrix(&self) -> Mat4 {
        let i = Mat4::identity();
        glm::rotate_x(&i, self.yaw) *
        glm::rotate_x(&i, self.pitch) *
        glm::scale(&i, &Vec3::new(1.0, self.aspect, 1.0)) *
        glm::scale(&i, &Vec3::new(self.scale, self.scale, self.scale)) *
        glm::translate(&i, &-self.center)
    }

    pub fn draw(&self, queue: &wgpu::Queue, frame: &wgpu::SwapChainTexture,
                encoder: &mut wgpu::CommandEncoder)
    {
        // Update the uniform buffer with our new matrix
        let mat = self.generate_matrix();
        queue.write_buffer(&self.uniform_buf, 0, bytemuck::cast_slice(mat.as_slice()));

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
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
