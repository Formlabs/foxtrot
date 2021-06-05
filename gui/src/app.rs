use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{ElementState, ModifiersState, MouseButton, WindowEvent, VirtualKeyCode, MouseScrollDelta},
};

use triangulate::mesh::Mesh;
use crate::model::Model;
use crate::backdrop::Backdrop;

pub struct App {
    surface: wgpu::Surface,
    device: wgpu::Device,
    swapchain_format: wgpu::TextureFormat,
    swapchain: wgpu::SwapChain,
    loader: Option<std::thread::JoinHandle<Mesh>>,
    model: Option<Model>,
    backdrop: Backdrop,

    depth: (wgpu::Texture, wgpu::TextureView),
    size: PhysicalSize<u32>,

    modifiers: ModifiersState,
    last_cursor: Option<PhysicalPosition<f64>>,
    left_mouse_down: bool,
    right_mouse_down: bool,

    first_frame: bool,
}

pub enum Reply {
    Continue,
    Redraw,
    Quit,
}

impl App {
    pub fn new(size: PhysicalSize<u32>, adapter: wgpu::Adapter,
               surface: wgpu::Surface, device: wgpu::Device,
               loader: std::thread::JoinHandle<Mesh>) -> Self
    {
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        let swapchain = Self::rebuild_swapchain_(
            size, swapchain_format, &surface, &device);
        let depth = Self::rebuild_depth_(size, &device);
        let backdrop = Backdrop::new(&device, swapchain_format);

        Self {
            swapchain,
            depth,
            backdrop,
            swapchain_format,
            loader: Some(loader),
            model: None,
            surface,
            device,
            size,

            modifiers: ModifiersState::empty(),
            last_cursor: None,
            left_mouse_down: false,
            right_mouse_down: false,

            first_frame: true,
        }
    }

    pub fn window_event(&mut self, e: WindowEvent) -> Reply {
        match e {
            WindowEvent::Resized(size) => {
                self.resize(size);
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
                if button == MouseButton::Left {
                    self.left_mouse_down = state == ElementState::Pressed;
                }
                if button == MouseButton::Right {
                    self.right_mouse_down = state == ElementState::Pressed;
                }
                Reply::Continue
            }
            WindowEvent::CursorMoved { position, .. } => {
                if let Some(prev) = self.last_cursor {
                    if self.left_mouse_down {
                        self.drag(position.x - prev.x, position.y - prev.y);
                    }
                    if self.right_mouse_down {
                        self.pan(position.x - prev.x, position.y - prev.y);
                    }
                }
                self.last_cursor = Some(position);
                Reply::Redraw
            },
            WindowEvent::MouseWheel { delta, ..} => {
                if let MouseScrollDelta::LineDelta(_,verti)=delta{
                    self.scale(1.0 + verti / 10.0);
                }
                Reply::Redraw
            }
            _ => Reply::Continue,
        }
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
        self.swapchain = Self::rebuild_swapchain_(
            size, self.swapchain_format,
            &self.surface, &self.device);
        self.depth = Self::rebuild_depth_(size, &self.device);
        if let Some(model) = &mut self.model {
            model.set_aspect(size.width as f32 / size.height as f32);
        }
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
            model.draw(&queue, &frame, &self.depth.1, &mut encoder);
        }
        let drew_model = self.model.is_some();
        queue.submit(Some(encoder.finish()));

        // This is very awkward, but WebGPU doesn't actually do the GPU work
        // until after a queue is submitted, so we don't wait to wait for
        // the model until the _second_ frame.
        if !self.first_frame && self.model.is_none() {
            println!("Waiting for mesh");
            let mesh = self.loader.take()
                .unwrap()
                .join()
                .expect("Failed to load mesh");
            let mut model = Model::new(&self.device, self.swapchain_format,
                                       &mesh.verts, &mesh.triangles);
            model.set_aspect(self.size.width as f32 / self.size.height as f32);
            self.model = Some(model);
        }
        self.first_frame = false;

        !drew_model
    }

    fn drag(&mut self, dx: f64, dy: f64) {
        if let Some(model) = &mut self.model {
            model.spin(dx as f32 / 100.0, dy as f32 / 100.0);
        }
    }

    fn pan(&mut self, dx:f64, dy:f64){
        if let Some(model) = &mut self.model {
            model.translate_camera(dx as f32 / -10000.0, dy as f32 / 10000.0 );
        }
    }

    fn scale(&mut self, value: f32) {
        if let Some(model) = &mut self.model {
            model.scale(value);
        }
    }
}
