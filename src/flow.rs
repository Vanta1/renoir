/// Similar to winit's 'ControlFlow', this tells the RenoiredApp when it should close.
#[derive(Default)]
pub struct Flow {
    pub should_close: bool,
}

impl Flow {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn close(&mut self) {
        self.should_close = true;
    }
}
