use std::sync::Arc;
use time::DeltaTime;
use winit::{
	application::ApplicationHandler,
	event::WindowEvent,
	event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
	window::{Window, WindowId},
};

mod input;
mod render;
mod state;
mod time;

use input::RenoiredInput;
use render::renderer::Renderer;
use state::RenoiredAppState;

pub mod prelude {
	pub use crate::input::Key;
	pub use crate::RenoiredApp;
}

pub type GameLoop = Box<dyn FnMut(&mut RenoiredAppState)>;

pub struct RenoiredApp {
	renderer: Option<Renderer<'static>>,
	state: RenoiredAppState,
	run_fn: Option<GameLoop>,
}

impl RenoiredApp {
	pub fn new() -> Self {
		RenoiredApp {
			renderer: None,
			run_fn: None,
			state: RenoiredAppState {
				window: None,
				input: RenoiredInput::new(),
				time: DeltaTime::new(),
				should_close: false,
			},
		}
	}

	pub fn run(&mut self, run_fn: impl FnMut(&mut RenoiredAppState) + 'static) {
		let event_loop = EventLoop::new().unwrap();
		event_loop.set_control_flow(ControlFlow::Poll);
		self.run_fn = Some(Box::new(run_fn));
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

		self.state.window = Some(window);
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
				self.state.input.set_mods(modifiers.state());
			}
			WindowEvent::KeyboardInput { event, .. } => {
				self.state.input.set_key(event);
			}
			WindowEvent::CloseRequested => {
				event_loop.exit();
			}
			WindowEvent::RedrawRequested => {
				// update delta time
				self.state.time.update();

				// Run the user's main function
				self.run_fn.as_mut().unwrap()(&mut self.state);
				// if after running the main function the user has decided the application should close, close it.
				if self.state.should_close {
					event_loop.exit()
				}
				
				// update the input struct, done after running the user's main function so that we don't unset keys before we need to.
				self.state.input.update();

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
				self.state.window.as_ref().unwrap().request_redraw();
			}
			_ => (),
		}
	}
}
