use winit::window::Window;

pub struct WindowOptions {
    pub fullscreen: bool,
    pub grab_cursor: bool,
}

impl WindowOptions {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub(crate) fn apply_to(&self, window: &Window) {
        window.set_fullscreen(match self.fullscreen {
            true => Some(winit::window::Fullscreen::Borderless(None)),
            false => None,
        });

        let _ = window.set_cursor_grab(match self.grab_cursor {
            true => winit::window::CursorGrabMode::Locked,
            false => winit::window::CursorGrabMode::None,
        });

        window.set_cursor_visible(self.grab_cursor)
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            fullscreen: true,
            grab_cursor: true,
        }
    }
}
