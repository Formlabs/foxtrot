use std::time::SystemTime;
use winit::{
    event::{Event},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

pub(crate) mod app;
pub(crate) mod backdrop;
pub(crate) mod camera;
pub(crate) mod model;

use crate::app::App;
use triangulate::mesh::Mesh;

async fn run(start: SystemTime, event_loop: EventLoop<()>, window: Window,
             loader: std::thread::JoinHandle<Mesh>)
{
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

    let mut app = App::new(start, size, adapter, surface, device, loader);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        use app::Reply;
        match event {
            Event::WindowEvent { event, .. } => match app.window_event(event) {
                Reply::Continue => (),
                Reply::Quit => *control_flow = ControlFlow::Exit,
                Reply::Redraw => if app.redraw(&queue) {
                    window.request_redraw();
                },
            },
            Event::RedrawRequested(_) => if app.redraw(&queue) {
                window.request_redraw();
            },
            Event::DeviceEvent { event, .. } => app.device_event(event),
            _ => (),
        }
    });
}

fn main() {
    let start = SystemTime::now();
    env_logger::init();

    let matches = clap::App::new("gui")
        .author("Matt Keeter <matt@formlabs.com>")
        .about("Renders a STEP file")
        .arg(clap::Arg::with_name("input")
            .takes_value(true)
            .required(true))
        .get_matches();
    let input = matches.value_of("input")
        .expect("Could not get input file")
        .to_owned();

    // Kick off the loader thread immediately, so that the STEP file is parsed
    // and triangulated in the background while we wait for a GPU context
    let loader = std::thread::spawn(|| {
        println!("Loading mesh!");
        use step::step_file::StepFile;
        use triangulate::triangulate::triangulate;

        let data = std::fs::read(input).expect("Could not open file");
        let flat = StepFile::strip_flatten(&data);
        let step = StepFile::parse(&flat);
        let (mesh, _stats) = triangulate(&step);
        mesh
    });

    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("Foxtrot");
    pollster::block_on(run(start, event_loop, window, loader));
}
