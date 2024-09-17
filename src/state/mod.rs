pub mod camera;
pub mod flow;
pub mod input;
pub mod time;
pub mod window_options;

use crate::state::camera::CameraController;
use crate::state::flow::Flow;
use crate::state::input::RenoiredInput;
use crate::state::time::DeltaTime;
use crate::state::window_options::WindowOptions;

/// RenoiredAppState contains everything that the game dev can modify and read while the app is running.
#[derive(Default)]
pub struct RenoiredAppState {
    pub window_options: WindowOptions,
    pub input: RenoiredInput,
    pub time: DeltaTime,
    pub flow: Flow,
    pub camera: CameraController,
}

impl RenoiredAppState {
    pub fn new() -> Self {
        Self {
            window_options: WindowOptions::new(),
            input: RenoiredInput::new(),
            time: DeltaTime::new(),
            flow: Flow::new(),
            camera: CameraController::new(),
        }
    }

    pub fn close(&mut self) {
        self.flow.should_close = true;
    }

    pub fn grab_cursor(&mut self, grab: bool) {
        self.window_options.grab_cursor = grab;
    }
}
