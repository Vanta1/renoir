use strum::EnumCount;
use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

/// Represents the state of a button (keyboard and mouse). JustPressed & JustReleased mean that the key's state has changed since the last processed engine tick.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum KeyState {
    JustPressed,
    Pressed,
    JustReleased,
    #[default]
    Released,
}

impl KeyState {
    pub fn pressed(&self) -> bool {
        *self == Self::JustPressed || *self == Self::Pressed
    }

    pub fn just_pressed(&self) -> bool {
        *self == Self::JustPressed
    }

    pub fn released(&self) -> bool {
        *self == Self::JustReleased || *self == Self::Released
    }

    pub fn just_released(&self) -> bool {
        *self == Self::JustReleased
    }

    pub(crate) fn update(&self) -> Self {
        match self {
            KeyState::JustPressed => KeyState::Pressed,
            KeyState::Pressed => KeyState::Pressed,
            KeyState::JustReleased => KeyState::Released,
            KeyState::Released => KeyState::Released,
        }
    }
}

impl From<ElementState> for KeyState {
    fn from(value: ElementState) -> Self {
        match value {
            ElementState::Pressed => KeyState::JustPressed,
            ElementState::Released => KeyState::JustReleased,
        }
    }
}

// taken from winit::event::MouseButton
#[derive(Debug, strum::EnumCount, strum::FromRepr)]
#[allow(dead_code)]
pub enum MouseBtn {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Default)]
pub struct MouseState {
    // winit supports more mouse buttons than this but.... who cares about those buttons (for now (i should find a better way to represent this))
    buttons: [KeyState; MouseBtn::COUNT],
    cursor_delta: (f32, f32),
    scroll_delta: (f32, f32),
}

impl MouseState {
    fn update(&mut self) {
        self.buttons.iter_mut().for_each(|btn| {
            btn.update();
        });

        self.cursor_delta = (0.0, 0.0);
        self.scroll_delta = (0.0, 0.0);
    }
}

pub struct RenoirInput {
    keys: [KeyState; Key::COUNT],
    pub(crate) key_stream: Vec<(Key, KeyState)>,
    prev_modifiers_state: winit::keyboard::ModifiersState,
    pub(crate) mouse: MouseState,
}

impl RenoirInput {
    pub fn new() -> Self {
        RenoirInput {
            keys: [KeyState::Released; Key::COUNT],
            key_stream: Vec::new(),
            prev_modifiers_state: winit::keyboard::ModifiersState::empty(),
            mouse: MouseState::default(),
        }
    }

    // this is called after the run_fn, so that held keys are handled correctly.
    // all the other setters are called in the main loop in lib.rs
    pub(crate) fn update(&mut self) {
        self.mouse.update();

        self.keys.iter_mut().for_each(|key| {
            key.update();
        });

        self.key_stream.clear();
    }

    pub(crate) fn set_key(&mut self, input: winit::event::KeyEvent) {
        match input.physical_key {
            PhysicalKey::Code(keycode) => {
                self.keys[keycode as usize] = input.state.into();
                self.key_stream.push((
                    Key::from_repr(keycode as usize).unwrap(),
                    input.state.into(),
                ))
            }
            PhysicalKey::Unidentified(_) => {
                // unhandled by RenoirInput as of now, but I could add these to some sort of extra input field
            }
        }
    }

    pub(crate) fn set_mods(&mut self, mods: winit::keyboard::ModifiersState) {
        if mods.shift_key() && !self.prev_modifiers_state.shift_key() {
            self.keys[Key::Shift as usize] = KeyState::JustPressed;
        } else if !mods.shift_key() && self.prev_modifiers_state.shift_key() {
            self.keys[Key::Shift as usize] = KeyState::JustReleased
        }

        if mods.control_key() && !self.prev_modifiers_state.control_key() {
            self.keys[Key::Ctrl as usize] = KeyState::JustPressed
        } else if !mods.control_key() && self.prev_modifiers_state.control_key() {
            self.keys[Key::Ctrl as usize] = KeyState::JustReleased
        }

        if mods.alt_key() && !self.prev_modifiers_state.alt_key() {
            self.keys[Key::Alt as usize] = KeyState::JustPressed
        } else if !mods.alt_key() && self.prev_modifiers_state.alt_key() {
            self.keys[Key::Alt as usize] = KeyState::JustReleased
        }

        if mods.super_key() && !self.prev_modifiers_state.super_key() {
            self.keys[Key::Logo as usize] = KeyState::JustPressed
        } else if !mods.super_key() && self.prev_modifiers_state.super_key() {
            self.keys[Key::Logo as usize] = KeyState::JustReleased
        }

        self.prev_modifiers_state = mods;
    }

    pub(crate) fn set_mouse_button(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse.buttons[0] = state.into(),
            MouseButton::Right => self.mouse.buttons[1] = state.into(),
            MouseButton::Middle => self.mouse.buttons[2] = state.into(),
            MouseButton::Back => self.mouse.buttons[3] = state.into(),
            MouseButton::Forward => self.mouse.buttons[4] = state.into(),
            MouseButton::Other(_id) => { /* currently unhandled */ }
        }
    }

    pub(crate) fn set_cursor_delta(&mut self, delta: (f32, f32)) {
        self.mouse.cursor_delta = delta;
    }

    pub(crate) fn set_scroll_delta(&mut self, delta: MouseScrollDelta) {
        match delta {
            MouseScrollDelta::LineDelta(x, y) => {
                self.mouse.scroll_delta = (x, y);
            }
            MouseScrollDelta::PixelDelta(pos) => {
                self.mouse.scroll_delta = (pos.x as f32, pos.y as f32);
            }
        }
    }

    pub fn get_key(&self, key: Key) -> KeyState {
        self.keys[key as usize]
    }

    pub fn pressed(&self, key: Key) -> bool {
        self.keys[key as usize].pressed()
    }

    pub fn just_pressed(&self, key: Key) -> bool {
        self.keys[key as usize].just_pressed()
    }

    pub fn released(&self, key: Key) -> bool {
        self.keys[key as usize].released()
    }

    pub fn just_released(&self, key: Key) -> bool {
        self.keys[key as usize].just_released()
    }

    pub fn get_mouse_btn(&self, btn: MouseBtn) -> KeyState {
        self.mouse.buttons[btn as usize]
    }

    pub fn mouse_pressed(&self, btn: MouseBtn) -> bool {
        self.mouse.buttons[btn as usize].pressed()
    }

    pub fn mouse_just_pressed(&self, btn: MouseBtn) -> bool {
        self.mouse.buttons[btn as usize].just_pressed()
    }

    pub fn mouse_released(&self, btn: MouseBtn) -> bool {
        self.mouse.buttons[btn as usize].released()
    }

    pub fn mouse_just_released(&self, btn: MouseBtn) -> bool {
        self.mouse.buttons[btn as usize].just_released()
    }

    pub fn get_mouse_delta(&self) -> (f32, f32) {
        self.mouse.cursor_delta
    }
}

impl Default for RenoirInput {
    fn default() -> Self {
        Self::new()
    }
}

/// Taken from winit::keyboard::KeyCode, with additional modifiers and "KeyA" changed to "A" for example
#[derive(Debug, strum::EnumCount, strum::FromRepr)]
#[allow(dead_code)]
pub enum Key {
    Backquote,
    Backslash,
    BracketLeft,
    BracketRight,
    Comma,
    Digit0,
    Digit1,
    Digit2,
    Digit3,
    Digit4,
    Digit5,
    Digit6,
    Digit7,
    Digit8,
    Digit9,
    Equal,
    IntlBackslash,
    IntlRo,
    IntlYen,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Minus,
    Period,
    Quote,
    Semicolon,
    Slash,
    AltLeft,
    AltRight,
    Backspace,
    CapsLock,
    ContextMenu,
    ControlLeft,
    ControlRight,
    Enter,
    SuperLeft,
    SuperRight,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    Convert,
    KanaMode,
    Lang1,
    Lang2,
    Lang3,
    Lang4,
    Lang5,
    NonConvert,
    Delete,
    End,
    Help,
    Home,
    Insert,
    PageDown,
    PageUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadAdd,
    NumpadBackspace,
    NumpadClear,
    NumpadClearEntry,
    NumpadComma,
    NumpadDecimal,
    NumpadDivide,
    NumpadEnter,
    NumpadEqual,
    NumpadHash,
    NumpadMemoryAdd,
    NumpadMemoryClear,
    NumpadMemoryRecall,
    NumpadMemoryStore,
    NumpadMemorySubtract,
    NumpadMultiply,
    NumpadParenLeft,
    NumpadParenRight,
    NumpadStar,
    NumpadSubtract,
    Escape,
    Fn,
    FnLock,
    PrintScreen,
    ScrollLock,
    Pause,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    Eject,
    LaunchApp1,
    LaunchApp2,
    LaunchMail,
    MediaPlayPause,
    MediaSelect,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    Power,
    Sleep,
    AudioVolumeDown,
    AudioVolumeMute,
    AudioVolumeUp,
    WakeUp,
    Meta,
    Hyper,
    Turbo,
    Abort,
    Resume,
    Suspend,
    Again,
    Copy,
    Cut,
    Find,
    Open,
    Paste,
    Props,
    Select,
    Undo,
    Hiragana,
    Katakana,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
    // new renoir stuff here, these are useful for not specifying ShiftL or ShiftR, for example. 'Logo' is the windows key or mac's Command
    Shift,
    Ctrl,
    Alt,
    Logo,
}
