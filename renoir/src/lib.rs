mod app;
mod math;
mod render;
mod state;

pub mod prelude {
    pub use crate::app::RenoirApp;
    pub use crate::math::prelude::*;
    pub use crate::state::app_state::RenoirAppState;
    pub use crate::state::camera::TransformSpace;
    pub use crate::state::input::{Key, MouseBtn};
    pub use crate::state::window_options::WindowOptions;
}
