use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};
use nalgebra_glm as glm;
use glm::{Vec4, Mat4};
use wgpu::util::DeviceExt;

use triangulate::mesh::{Vertex, Triangle};

use crate::camera::Camera;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct GPUVertex {
    pos: [f32; 4],
    norm: [f32; 4],
    color: [f32; 4],
}

impl GPUVertex {
    fn from_vertex(v: &Vertex) -> Self {
        Self {
            pos: [v.pos.x as f32, v.pos.y as f32, v.pos.z as f32, 1.0],
            norm: [v.norm.x as f32, v.norm.y as f32, v.norm.z as f32, 1.0],
            color: [v.color.x as f32, v.color.y as f32, v.color.z as f32, 1.0],
        }
    }
}

pub struct Model {
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    uniform_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    index_count: u32,
    render_pipeline: wgpu::RenderPipeline,
}

impl Model {
    pub fn new(device: &wgpu::Device, swapchain_format: wgpu::TextureFormat,
               verts: &[Vertex], tris: &[Triangle]) -> Self {

        let vertex_data: Vec<GPUVertex> = verts.into_iter()
            .map(GPUVertex::from_vertex)
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
            size: std::mem::size_of::<Mat4>() as wgpu::BufferAddress * 2,
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
                        min_binding_size: wgpu::BufferSize::new(
                            std::mem::size_of::<Mat4>() as u64 * 2),
                    },
                    count: None,
                },
            ],
        });

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
                // Positions
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 0,
                    shader_location: 0,
                },
                // Normals
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                    shader_location: 1,
                },
                // Colors
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x4,
                    offset: 2*std::mem::size_of::<Vec4>() as wgpu::BufferAddress,
                    shader_location: 2,
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

        // Load the shaders from disk, either at runtime or compile-time
        #[cfg(feature = "bundle-shaders")]
        let model_src = Cow::Borrowed(include_str!("model.wgsl"));

        #[cfg(not(feature = "bundle-shaders"))]
        let model_src = Cow::Owned(
            String::from_utf8(
                std::fs::read("gui/src/model.wgsl")
                    .expect("Could not read shader"))
                    .expect("Shader is invalid UTF-8"));

        let shader = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(model_src),
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
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Greater,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState::default(),
                }),
                multisample: wgpu::MultisampleState::default(),
        });

        Model {
            render_pipeline,
            index_buf,
            vertex_buf,
            uniform_buf,
            bind_group,
            index_count: tris.len() as u32 * 3,
        }
    }

    pub fn draw(&self, camera: &Camera,
                queue: &wgpu::Queue,
                frame: &wgpu::SwapChainTexture,
                depth_view: &wgpu::TextureView,
                encoder: &mut wgpu::CommandEncoder)
    {
        // Update the uniform buffer with our new matrix
        let view_mat = camera.view_matrix();
        let model_mat = camera.model_matrix();
        queue.write_buffer(&self.uniform_buf, 0,
            bytemuck::cast_slice(view_mat.as_slice()));
        queue.write_buffer(&self.uniform_buf,
            std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
            bytemuck::cast_slice(model_mat.as_slice()));

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
                depth_stencil_attachment: Some(
                    wgpu::RenderPassDepthStencilAttachment {
                        view: &depth_view,
                        depth_ops: Some(wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: true,
                        }),
                        stencil_ops: None,
                    }),
            });
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_index_buffer(self.index_buf.slice(..), wgpu::IndexFormat::Uint32);
        rpass.set_vertex_buffer(0, self.vertex_buf.slice(..));
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw_indexed(0..self.index_count, 0, 0..1);
    }
}
