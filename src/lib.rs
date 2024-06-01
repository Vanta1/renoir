use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod input;
mod macros;
mod render;
mod state;
mod time;

use render::renderer::Renderer;
use state::RenoiredAppState;

pub mod prelude {
    pub use crate::alias;
    pub use crate::input::Key;
    pub use crate::RenoiredApp;
}

pub type GameLoop = Box<dyn FnMut(&mut RenoiredAppState)>;

pub struct RenoiredApp {
    renderer: Option<Renderer<'static>>,
    pub state: RenoiredAppState,
    run_fn: Option<GameLoop>,
}

impl RenoiredApp {
    pub fn new() -> Self {
        RenoiredApp {
            renderer: None,
            run_fn: None,
            state: RenoiredAppState::new()
        }
    }

    pub fn run(&mut self, run_fn: impl FnMut(&mut RenoiredAppState) + 'static) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        self.run_fn = Some(Box::new(run_fn));
        env_logger::init();
        match event_loop.run_app(self) {
            Ok(_) => {
                println!("See you next time...") // i miss osu
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

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => self.state.input.set_cursor_delta(delta),
            DeviceEvent::MouseWheel { delta } => self.state.input.set_scroll_delta(delta),
            _ => {}
        }
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

            // Renoired uses a custom input system so that users don't have to deal with handling WindowEvents, it reads all inputs before running the main game loop and processes them with the RenoirInput structure
            WindowEvent::ModifiersChanged(modifiers) => {
                self.state.input.set_mods(modifiers.state());
            }
            WindowEvent::KeyboardInput { event, .. } => {
                self.state.input.set_key(event);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                self.state.input.set_mouse_button(state, button);
            }

            WindowEvent::RedrawRequested => {
                // update delta time
                self.state.time.update();

                // Run the user's main function
                self.run_fn.as_mut().unwrap()(&mut self.state);

                // if after running the main function the user has decided the application should close, close it.
                if self.state.flow.should_close() {
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
            _ => {}
        }
    }
}
