use winit::{
    dpi::PhysicalSize,
};

use crate::model::Model;
use crate::backdrop::Backdrop;

pub struct App {
    surface: wgpu::Surface,
    device: wgpu::Device,
    swapchain_format: wgpu::TextureFormat,
    swapchain: wgpu::SwapChain,
    model: Model,
    backdrop: Backdrop,
}

impl App {
    pub fn new(size: PhysicalSize<u32>, adapter: wgpu::Adapter,
           surface: wgpu::Surface, device: wgpu::Device) -> Self
    {
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        let step = step::ap214::parse(&[]);
        let (verts, tris) = step::triangulate::triangulate(&step);

        Self {
            swapchain_format,
            swapchain: Self::rebuild_swapchain_(
                size, swapchain_format, &surface, &device),
            model: Model::new(&device, swapchain_format, &verts, &tris),
            backdrop: Backdrop::new(&device, swapchain_format),
            surface,
            device,
        }
    }

    pub fn rebuild_swapchain(&mut self,size: PhysicalSize<u32>) {
        self.swapchain = Self::rebuild_swapchain_(
            size, self.swapchain_format,
            &self.surface, &self.device);
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

    pub fn redraw(&self, queue: &wgpu::Queue) {
        let frame = self.swapchain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None });

        self.backdrop.draw(&frame, &mut encoder);
        self.model.draw(&frame, &mut encoder);

        queue.submit(Some(encoder.finish()));
    }
}
