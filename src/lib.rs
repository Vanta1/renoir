use std::sync::Arc;
use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
	window::{Window, WindowId},
};

mod input;
mod render;

use input::RenoiredInput;
use render::renderer::Renderer;

pub mod prelude {
	pub use crate::RenoiredApp;
}

pub struct RenoiredApp {
	renderer: Option<Renderer<'static>>,
	input: RenoiredInput,
	pub window: Option<Arc<Window>>,
}

impl RenoiredApp {
	pub fn new() -> Self {
		RenoiredApp {
			renderer: None,
			input: RenoiredInput::new(),
			window: None,
		}
	}

	pub fn run(&mut self) {
		let event_loop = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Poll);
		env_logger::init();
		match event_loop.run_app(self) {
			Ok(_) => {
				println!("See you next time...")
			}
			Err(e) => println!("{e}"),
		}
	}
}

impl ApplicationHandler for RenoiredApp {
	fn resumed(&mut self, event_loop: &ActiveEventLoop) {
		// I owe my life to this file: https://github.com/nical/lyon/blob/main/examples/wgpu/src/main.rs
		// TODO: double check that the Arc around Window is necessary
		let window = Arc::new(
			event_loop
				.create_window(Window::default_attributes())
				.unwrap(),
		);
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
			// Renoired uses a custom input system so that users don't have to deal with handling WindowEvents, it reads all inputs before running the main game loop and processes them with the RenoirInput structure
			WindowEvent::ModifiersChanged(modifiers) => {
				self.input.set_mods(modifiers.state());
			}
			WindowEvent::KeyboardInput { event, .. } => {
				self.input.set_key(event);
			}
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				self.input.update();

				if let Some(renderer) = self.renderer.as_mut() {
					match renderer.render() {
						Ok(_) => {}
						// Reconfigure the surface if lost
						Err(wgpu::SurfaceError::Lost) => renderer.resize(renderer.size),
						// The system is out of memory, we should probably quit
						Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
						// All other errors (Outdated, Timeout) should be resolved by the next frame
						Err(e) => eprintln!("{:?}", e),
					}
				} else {
					println!("Renderer wasn't initialized prior to trying to render.. ??");
				}
				self.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}
