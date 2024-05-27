use std::sync::Arc;
use winit::window::Window;

use crate::input::RenoiredInput;
use crate::time::DeltaTime;

pub struct Flow {
    should_close: bool,
}

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
    pub window: Option<Arc<Window>>,
    pub input: RenoiredInput,
    pub time: DeltaTime,
    pub flow: Flow,
}

impl RenoiredAppState {
    pub fn close(&mut self) {
        self.flow.should_close = true;
    }
}
