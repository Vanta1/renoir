use std::sync::Arc;
use winit::window::{CursorGrabMode, Window};

use crate::camera::CameraController;
use crate::input::RenoiredInput;
use crate::time::DeltaTime;

pub struct Flow {
    should_close: bool,
}

/// Similar to winit's 'ControlFlow', this tells the RenoiredApp when it should close.
impl Flow {
    pub fn new() -> Self {
        Flow {
            should_close: false,
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn close(&mut self) {
        self.should_close = true;
    }
}

pub struct RenoiredAppState {
    pub window: Option<Arc<Window>>, // window needs to be an option as one can only be created with an ActiveEventLoop
    pub input: RenoiredInput,
    pub time: DeltaTime,
    pub flow: Flow,
    pub camera: CameraController,
}

impl RenoiredAppState {
    pub fn new() -> Self {
        Self {
            window: None,
            input: RenoiredInput::new(),
            time: DeltaTime::new(),
            flow: Flow::new(),
            camera: CameraController::new(),
        }
    }

    pub fn close(&mut self) {
        self.flow.should_close = true;
    }

    pub fn grab_cursor(&self, grab: bool) {
        // the result of set_cursor_grab is ignored as it's only necessary for Wayland, and so it doesn't matter on other platforms

        if let Some(window) = self.window.as_ref() {
            if grab {
                window.set_cursor_visible(false);
                let _ = window.set_cursor_grab(CursorGrabMode::Locked);
            } else {
                self.window.as_ref().unwrap().set_cursor_visible(true);
                let _ = window.set_cursor_grab(CursorGrabMode::None);
            }
        } else {
            // TODO: implement a window configuration that can be initialized and edited prior to window creation, and then applied after
        }
    }
}
