use std::time::SystemTime;
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub(crate) mod app;
pub(crate) mod model;
pub(crate) mod backdrop;

use crate::app::App;

async fn run(start: SystemTime, event_loop: EventLoop<()>, window: Window) {
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
    let mut first = true;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        use app::Reply;
        match event {
            Event::WindowEvent { event, .. } => match app.window_event(event) {
                Reply::Continue => (),
                Reply::Quit => *control_flow = ControlFlow::Exit,
                Reply::Redraw => app.redraw(&queue),
            },
            Event::RedrawRequested(_) => {
                app.redraw(&queue);
                if first {
                    let end = SystemTime::now();
                    let dt = end.duration_since(start).expect("dt < 0??");
                    println!("First redraw at {:?}", dt);
                    first = false;
                }
            },
            _ => {}
        }
    });
}

fn main() {
    let start = SystemTime::now();
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    env_logger::init();
    pollster::block_on(run(start, event_loop, window));
}
