use std::borrow::Cow;

use bytemuck::{Pod, Zeroable};
use itertools::Itertools;
use nalgebra_glm as glm;
use glm::{Vec3, Vec4, Mat4};
use wgpu::util::DeviceExt;

use triangulate::mesh::{Vertex, Triangle};

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

    // Model matrix parameters
    aspect: f32, pitch: f32, yaw: f32, scale: f32, center: Vec3,
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
            pitch: 0.0, yaw: 0.0, aspect: 1.0,
            scale, center,
        }
    }

    pub fn set_aspect(&mut self, a: f32) {
        self.aspect = a;
    }

    fn generate_matrix(&self) -> Mat4 {
        let i = Mat4::identity();
        // The transforms below are applied bottom-to-top when thinking about
        // the model, i.e. it's translated, then scaled, then rotated, etc.

        // The Z clipping range is 0-1, so push forward
        glm::translate(&i, &Vec3::new(0.0, 0.0, 0.5)) *

        // Scale to compensate for aspect ratio and reduce Z scale to improve
        // clipping
        glm::scale(&i, &Vec3::new(1.0, self.aspect, 0.1)) *

        // Rotation!
        glm::rotate_x(&i, self.yaw) *
        glm::rotate_y(&i, self.pitch) *

        // Scale to compensate for model size
        glm::scale(&i, &Vec3::new(self.scale, self.scale, self.scale)) *

        // Recenter model
        glm::translate(&i, &-self.center)
    }

    pub fn draw(&self, queue: &wgpu::Queue,
                frame: &wgpu::SwapChainTexture,
                depth_view: &wgpu::TextureView,
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

    pub fn spin(&mut self, dx: f32, dy: f32) {
        self.pitch += dx;
        self.yaw += dy;
    }

    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32){
        self.center.x += dx;
        self.center.y += dy;
        self.center.z += dz;
    }

    pub fn translate_camera(&mut self, dx: f32, dy: f32){
        let i = Mat4::identity();
        let vec=glm::rotate_y(&i, -self.pitch) *glm::rotate_x(&i, -self.yaw) *Vec4::new(dx, dy, 0.0, 1.0);
        self.translate(vec.x, vec.y, vec.z);
    }

    pub fn scale(&mut self, value: f32){
        self.scale*=value;
    }
}
