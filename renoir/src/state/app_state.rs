use crate::state::camera::CameraController;
use crate::state::flow::Flow;
use crate::state::input::RenoirInput;
use crate::state::time::DeltaTime;
use crate::state::window_options::WindowOptions;

#[cfg(feature = "ecs")]
use hecs::World;

/// RenoirAppState contains everything that the game dev can modify and read while the app is running.
#[derive(Default)]
pub struct RenoirAppState {
    pub camera: CameraController,
    pub input: RenoirInput,
    pub flow: Flow,
    pub time: DeltaTime,
    pub window_options: WindowOptions,
    #[cfg(feature = "ecs")]
    pub world: World,
}

impl RenoirAppState {
    pub fn new() -> Self {
        Self {
            camera: CameraController::new(),
            input: RenoirInput::new(),
            flow: Flow::new(),
            time: DeltaTime::new(),
            window_options: WindowOptions::new(),
            #[cfg(feature = "ecs")]
            world: World::new(),
        }
    }

    pub fn close(&mut self) {
        self.flow.should_close = true;
    }
}
