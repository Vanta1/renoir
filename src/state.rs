use std::sync::Arc;
use winit::window::Window;

use crate::input::RenoiredInput;
use crate::time::DeltaTime;

pub struct RenoiredAppState {
    pub window: Option<Arc<Window>>,
    pub input: RenoiredInput,
    pub time: DeltaTime,
    pub(crate) should_close: bool,
}

impl RenoiredAppState {
    pub fn close(&mut self) {
        self.should_close = true;
    }
}
