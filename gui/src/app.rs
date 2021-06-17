use nalgebra_glm as glm;
use glm::Vec2;
use winit::{
    dpi::{PhysicalSize},
    event::{ElementState, ModifiersState, WindowEvent, DeviceEvent, VirtualKeyCode, MouseScrollDelta},
};

use triangulate::mesh::Mesh;
use crate::{backdrop::Backdrop, camera::Camera, model::Model};

pub struct App {
    start_time: std::time::SystemTime,

    surface: wgpu::Surface,
    device: wgpu::Device,
    swapchain_format: wgpu::TextureFormat,
    swapchain: wgpu::SwapChain,

    loader: Option<std::thread::JoinHandle<Mesh>>,
    model: Option<Model>,
    backdrop: Backdrop,
    camera: Camera,

    depth: (wgpu::Texture, wgpu::TextureView),
    size: PhysicalSize<u32>,

    modifiers: ModifiersState,

    first_frame: bool,
}

pub enum Reply {
    Continue,
    Redraw,
    Quit,
}

impl App {
    pub fn new(start_time: std::time::SystemTime, size: PhysicalSize<u32>,
               adapter: wgpu::Adapter, surface: wgpu::Surface,
               device: wgpu::Device, loader: std::thread::JoinHandle<Mesh>)
        -> Self
    {
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface)
            .expect("Could not get swapchain format");

        let swapchain = Self::rebuild_swapchain_(
            size, swapchain_format, &surface, &device);
        let depth = Self::rebuild_depth_(size, &device);
        let backdrop = Backdrop::new(&device, swapchain_format);

        Self {
            start_time,

            swapchain,
            depth,
            backdrop,
            swapchain_format,
            loader: Some(loader),
            model: None,
            camera: Camera::new(size.width as f32, size.height as f32),
            surface,
            device,
            size,

            modifiers: ModifiersState::empty(),

            first_frame: true,
        }
    }

    pub fn device_event(&mut self, e: DeviceEvent) {
        if let DeviceEvent::MouseWheel { delta } = e {
            if let MouseScrollDelta::PixelDelta(p) = delta {
                self.camera.mouse_scroll(p.y as f32);
            }
        }
    }

    pub fn window_event(&mut self, e: WindowEvent) -> Reply {
        match e {
            WindowEvent::Resized(size) => {
                self.resize(size);
                Reply::Redraw
            },
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                self.resize(*new_inner_size);
                Reply::Redraw
            },
            WindowEvent::CloseRequested => Reply::Quit,
            WindowEvent::ModifiersChanged(m) => {
                self.modifiers = m;
                Reply::Continue
            },
            WindowEvent::KeyboardInput { input, .. } => {
                if self.modifiers.logo() && input.virtual_keycode == Some(VirtualKeyCode::Q) {
                    Reply::Quit
                } else {
                    Reply::Continue
                }
            },
            WindowEvent::MouseInput { button, state, .. } => {
                use ElementState::*;
                match state {
                    Pressed => self.camera.mouse_pressed(button),
                    Released => self.camera.mouse_released(button),
                }
                Reply::Continue
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.camera.mouse_move(Vec2::new(position.x as f32, position.y as f32));
                Reply::Redraw
            },
            WindowEvent::MouseWheel { delta, ..} => {
                if let MouseScrollDelta::LineDelta(_, verti) = delta {
                    self.camera.mouse_scroll(verti * 10.0);
                }
                Reply::Redraw
            },
            _ => Reply::Continue,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.swapchain = Self::rebuild_swapchain_(
            size, self.swapchain_format,
            &self.surface, &self.device);
        self.depth = Self::rebuild_depth_(size, &self.device);
        self.camera.set_size(size.width as f32, size.height as f32);
    }

    fn rebuild_depth_(size: PhysicalSize<u32>, device: &wgpu::Device)
        -> (wgpu::Texture, wgpu::TextureView)
    {
        let size = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some("depth tex"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT |
                   wgpu::TextureUsage::SAMPLED,
        };
        let tex = device.create_texture(&desc);
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
        (tex, view)
    }

    fn rebuild_swapchain_(size: PhysicalSize<u32>, format: wgpu::TextureFormat,
                          surface: &wgpu::Surface, device: &wgpu::Device)
        -> wgpu::SwapChain
    {
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        device.create_swap_chain(surface, &sc_desc)
    }

    // Redraw the GUI, returning true if the model was not drawn (which means
    // that the parent loop should keep calling redraw to force model load)
    pub fn redraw(&mut self, queue: &wgpu::Queue) -> bool {
        let frame = self.swapchain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None });

        self.backdrop.draw(&frame, &self.depth.1, &mut encoder);
        if let Some(model) = &self.model {
            model.draw(&self.camera, &queue, &frame, &self.depth.1, &mut encoder);
        }
        let drew_model = self.model.is_some();
        queue.submit(Some(encoder.finish()));

        if drew_model && self.first_frame {
            let end = std::time::SystemTime::now();
            let dt = end.duration_since(self.start_time).expect("dt < 0??");
            println!("First redraw at {:?}", dt);
            self.first_frame = false;
        }

        // This is very awkward, but WebGPU doesn't actually do the GPU work
        // until after a queue is submitted, so we don't wait to wait for
        // the model until the _second_ frame.
        if !self.first_frame && self.model.is_none() {
            println!("Waiting for mesh");
            let mesh = self.loader.take()
                .unwrap()
                .join()
                .expect("Failed to load mesh");
            let model = Model::new(&self.device, self.swapchain_format,
                                   &mesh.verts, &mesh.triangles);
            self.model = Some(model);
            self.camera.fit_verts(&mesh.verts);
            self.first_frame = true;
        } else {
            self.first_frame = false;
        }

        !drew_model
    }
}
