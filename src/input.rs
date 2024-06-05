#![allow(dead_code)] // TODO: get rid of thiss

// TODO: add some sort of event/keypress streaming option (list of keyboard) so that text elements can be created
// that players can type in without having to check all keys to determine which is pressed
// .. this could also be accomplished with a way to get a stream of all pressed keys

// #[allow(unused_imports)]
// use renoir_proc_macros::VariantVector;

use winit::{
    event::{ElementState, MouseButton, MouseScrollDelta},
    keyboard::PhysicalKey,
};

// Because I want to use VirtualKeycode as an index into a list of keys, we get the number of
// possible VirtualKeycodes and then subtract one as arrays are zero-indexed.
// const NUM_KEYCODES: usize = std::mem::variant_count::<winit::keyboard::KeyCode>();
const NUM_KEYCODES: usize = 194 + 4; // 194 winit, 4 provided by renoired

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

#[derive(Default)]
pub struct MouseState {
    // winit supports more mouse buttons than this but.... who cares about those buttons (for now (i should find a better way to represent this))
    left_click: KeyState,
    middle_click: KeyState,
    right_click: KeyState,
    cursor_delta: (f32, f32),
    scroll_delta: (f32, f32),
}

impl MouseState {
    fn update(&mut self) {
        self.left_click.update();
        self.middle_click.update();
        self.right_click.update();
        self.cursor_delta = (0.0, 0.0);
        self.scroll_delta = (0.0, 0.0);
    }
}

pub struct RenoiredInput {
    keys: [KeyState; NUM_KEYCODES],
    prev_modifiers_state: winit::keyboard::ModifiersState,
    pub(crate) mouse: MouseState,
}

impl RenoiredInput {
    pub fn new() -> Self {
        RenoiredInput {
            keys: [KeyState::Released; NUM_KEYCODES],
            prev_modifiers_state: winit::keyboard::ModifiersState::empty(),
            mouse: MouseState::default(),
        }
    }

    pub(crate) fn update(&mut self) {
        self.mouse.update();

        self.keys.iter_mut().for_each(|key| {
            key.update();
        });
    }

    pub(crate) fn set_key(&mut self, input: winit::event::KeyEvent) {
        match input.physical_key {
            PhysicalKey::Code(keycode) => self.keys[keycode as usize] = input.state.into(),
            PhysicalKey::Unidentified(_) => { /* TODO: figure out what to do with these */ }
        }
    }

    pub(crate) fn set_mods(&mut self, mods: winit::keyboard::ModifiersState) {
        if mods.shift_key() && !self.prev_modifiers_state.shift_key() {
            self.keys[NUM_KEYCODES - 4] = KeyState::JustPressed
        } else if !mods.shift_key() && self.prev_modifiers_state.shift_key() {
            self.keys[NUM_KEYCODES - 4] = KeyState::JustReleased
        }

        if mods.control_key() && !self.prev_modifiers_state.control_key() {
            self.keys[NUM_KEYCODES - 3] = KeyState::JustPressed
        } else if !mods.control_key() && self.prev_modifiers_state.control_key() {
            self.keys[NUM_KEYCODES - 3] = KeyState::JustReleased
        }

        if mods.alt_key() && !self.prev_modifiers_state.alt_key() {
            self.keys[NUM_KEYCODES - 2] = KeyState::JustPressed
        } else if !mods.alt_key() && self.prev_modifiers_state.alt_key() {
            self.keys[NUM_KEYCODES - 2] = KeyState::JustReleased
        }

        if mods.super_key() && !self.prev_modifiers_state.super_key() {
            self.keys[NUM_KEYCODES - 1] = KeyState::JustPressed
        } else if !mods.super_key() && self.prev_modifiers_state.super_key() {
            self.keys[NUM_KEYCODES - 1] = KeyState::JustReleased
        }

        self.prev_modifiers_state = mods;
    }

    pub(crate) fn set_mouse_button(&mut self, state: ElementState, button: MouseButton) {
        match button {
            MouseButton::Left => self.mouse.left_click = state.into(),
            MouseButton::Middle => self.mouse.middle_click = state.into(),
            MouseButton::Right => self.mouse.middle_click = state.into(),
            _ => {}
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

    pub fn get_mouse_delta(&self) -> (f32, f32) {
        self.mouse.cursor_delta
    }

    // TODO: add functions for getting mouse buttons. im too tired to pick names.

    /* TODO: I dont think this can be implemented without a proc macro to get a Vec of keys, that we can use 'index' as an index into to
             get a list of pressed keys w/ associated renoir Key names
    pub fn get_pressed(&self) {
        self.keys.iter().enumerate().map(|(index, key)| match key {
            KeyState::JustPressed | KeyState::Pressed => {

            }
            _ => {}
        });

        todo!()
    }
    */
}

// taken from winit::keyboard::KeyCode, with additional modifiers and "KeyA" changed to "A" for example
#[derive(Debug)]
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
