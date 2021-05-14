use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub(crate) mod model;
use crate::model::Model;

struct App {
    surface: wgpu::Surface,
    device: wgpu::Device,
    swapchain_format: wgpu::TextureFormat,
    swapchain: wgpu::SwapChain,
    model: Model,
}

impl App {
    fn new(size: PhysicalSize<u32>, adapter: wgpu::Adapter,
           surface: wgpu::Surface, device: wgpu::Device) -> Self
    {
        let swapchain_format = adapter.get_swap_chain_preferred_format(&surface).unwrap();

        Self {
            swapchain_format,
            swapchain: Self::rebuild_swapchain_(
                size, swapchain_format, &surface, &device),
            model: Model::new(&device, swapchain_format),
            surface,
            device,
        }
    }

    fn rebuild_swapchain(&mut self,size: PhysicalSize<u32>) {
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

    fn redraw(&self, queue: &wgpu::Queue) {
        let frame = self.swapchain
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None });

        self.model.draw(&frame, &mut encoder);

        queue.submit(Some(encoder.finish()));
    }
}

async fn run(event_loop: EventLoop<()>, window: Window) {
    let size = window.inner_size();
    let (surface, adapter) = {
        let instance = wgpu::Instance::new(wgpu::BackendBit::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                // Request an adapter which can render to our surface
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");
        (surface, adapter)
    };

    // Create the logical device and command queue
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let mut app = App::new(size, adapter, surface, device);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => app.rebuild_swapchain(size),
            Event::RedrawRequested(_) => app.redraw(&queue),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}

fn main() {
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    env_logger::init();
    pollster::block_on(run(event_loop, window));
}
