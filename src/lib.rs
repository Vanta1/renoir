use std::sync::Arc;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

mod camera;
mod flow;
mod input;
mod math;
mod render;
mod state;
mod time;
mod window_options;

use render::renderer::Renderer;
use state::RenoiredAppState;

pub mod prelude {
    pub use crate::camera::TransformSpace;
    pub use crate::input::{Key, MouseBtn};
    pub use crate::math::prelude::*;
    pub use crate::RenoiredApp;
}

#[derive(Default)]
pub struct RenoiredApp {
    renderer: Option<Renderer<'static>>,
    window: Option<Arc<Window>>,
    state: RenoiredAppState,
    // this type is only used here, and only one can exist in the entire program, so idc if it's complex
    #[allow(clippy::type_complexity)]
    run_fn: Option<Box<dyn FnMut(&mut RenoiredAppState)>>,
}

impl RenoiredApp {
    pub fn new() -> Self {
        RenoiredApp {
            renderer: None,
            window: None,
            run_fn: None,
            state: RenoiredAppState::new(),
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

    pub fn grab_cursor(&mut self, grab: bool) {
        self.state.grab_cursor(grab);
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

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => self
                .state
                .input
                .set_cursor_delta((delta.0 as f32, delta.1 as f32)),
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
            // TODO: add some way to pass this to the game before the program just quits
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
                // unwrapping is safe here as a RedrawRequested event cannot happen before the developer specifies a run_fn when calling RenoiredApp::run()
                self.run_fn.as_mut().unwrap()(&mut self.state);

                // apply WindowOptions to Window
                self.state
                    .window_options
                    .apply_to(self.window.as_ref().unwrap());

                // if after running the main function the user has decided the application should close, close it.
                if self.state.flow.should_close() {
                    event_loop.exit()
                }

                // TODO: this is producing multiple inputs for a held key, which may be the desired behavior but I need to think it through
                if !self.state.input.key_stream.is_empty() {
                    //dbg!(&self.state.input.key_stream);
                }
                // update the input struct, done after running the user's main function so that we don't unset keys before we need to.
                self.state.input.update();

                // this should always unwrap, as RedrawRequested only happens after the renderer has been initialized.
                // additionally, destructuring in order to get a single mutable reference makes this whole thing simpler
                if let Some(renderer) = self.renderer.as_mut() {
                    match renderer.render(&mut self.state.camera) {
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
            _ => {}
        }
    }
}
