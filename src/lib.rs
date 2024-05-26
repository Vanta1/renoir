use wgpu::RenderBundle;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use std::sync::Arc;

mod render;

use render::renderer::Renderer;

pub mod prelude {
    pub use crate::RenoiredApp;
}

pub struct RenoiredApp {
    renderer: Option<Renderer<'static>>,
    pub window: Option<Arc<Window>>,
}

impl RenoiredApp {
    pub fn new() -> Self {
        RenoiredApp { 
            renderer: None,
            window: None
        }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        match event_loop.run_app(self) {
            Ok(_) => {
                println!("See you next time...")
            }
            Err(e) => println!("{e}"),
        }
    }

    fn create_renderer(&self) -> Renderer {
        todo!()
    }
}

impl ApplicationHandler for RenoiredApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // I owe my life to this file:
        let window = Arc::new(event_loop.create_window(Window::default_attributes()).unwrap());
        let renderer = Renderer::new(Arc::clone(&window));

        self.window = Some(window);
        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {

            },
            _ => (),
        }
    }
}
