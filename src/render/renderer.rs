use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};
use wgpu::{Backends, Device, DeviceDescriptor, Features, Instance, InstanceDescriptor, Limits, PowerPreference, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration, TextureUsages};
use std::sync::Arc;

pub struct Renderer<'a> {
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: PhysicalSize<u32>,
}

impl<'a> Renderer<'a> {
    pub fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(InstanceDescriptor { 
            backends: Backends::all(), 
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }
        )).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(),
                required_limits: Limits::default(),
            }, 
            None
        )).unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            desired_maximum_frame_latency: 2,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            format: surface_format,
            width: size.width,
            height: size.height,
            usage: TextureUsages::RENDER_ATTACHMENT,
            view_formats: vec![],
        };

        Renderer {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}