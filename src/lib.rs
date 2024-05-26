use winit::{
    application::ApplicationHandler,
    event::{self, WindowEvent},
    event_loop::{self, ActiveEventLoop, ControlFlow, EventLoop},
    keyboard::PhysicalKey,
    window::{ResizeDirection, Window, WindowAttributes, WindowId},
};

pub mod prelude {
    pub use crate::Piece;
}

pub struct Piece {
    window: Option<Window>,
}

impl Piece {
    pub fn new() -> Piece {
        Piece { window: None }
    }

    pub fn run(&mut self) {
        let event_loop = EventLoop::new().unwrap();
        match event_loop.run_app(self) {
            Ok(_) => {
                println!("See you next time...")
            }
            Err(e) => println!("{e}"),
        }
    }
}

impl ApplicationHandler for Piece {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.window = Some(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
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
            WindowEvent::RedrawRequested => self.window.as_ref().unwrap().request_redraw(),
            _ => (),
        }
    }
}
