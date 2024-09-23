use winit::window::Window;

pub struct WindowOptions {
    pub fullscreen: bool,
    pub grab_cursor: bool,
    pub show_cursor: bool,
}

impl WindowOptions {
    pub(crate) fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    // applies own window settings to a winit window, this is called from the main loop only
    pub(crate) fn apply_to(&self, window: &Window) {
        window.set_fullscreen(match self.fullscreen {
            true => Some(winit::window::Fullscreen::Borderless(None)),
            false => None,
        });

        let _ = window.set_cursor_grab(match self.grab_cursor {
            true => winit::window::CursorGrabMode::Locked,
            false => winit::window::CursorGrabMode::None,
        });

        window.set_cursor_visible(self.show_cursor);
    }

    pub fn set_options(&mut self, options: Self) {
        *self = options;
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        Self {
            fullscreen: true,
            grab_cursor: true,
            show_cursor: false,
        }
    }
}
