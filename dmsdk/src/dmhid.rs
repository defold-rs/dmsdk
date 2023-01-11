//! Structs for interacting with input devices.

use crate::ffi::dmHID;

/// HID context.
pub struct Context {
    ptr: *mut dmHID::Context,
}

impl Context {
    /// Creates a new [`Context`] from the given pointer.
    ///
    /// You probably want [`dmengine::get_hid_context()`](crate::dmengine::get_hid_context()) instead.
    pub fn new(ptr: *mut dmHID::Context) -> Self {
        Self { ptr }
    }

    /// Returns the keyboard at the given index if it exists.
    pub fn get_keyboard(&self, index: u8) -> Option<Keyboard> {
        let keyboard = unsafe { dmHID::GetKeyboard(self.ptr, index) };

        Keyboard::new(keyboard)
    }

    /// Returns the mouse at the given index if it exists.
    pub fn get_mouse(&self, index: u8) -> Option<Mouse> {
        let mouse = unsafe { dmHID::GetMouse(self.ptr, index) };

        Mouse::new(mouse)
    }

    /// Returns the touch device at the given index if it exists.
    pub fn get_touch_device(&self, index: u8) -> Option<TouchDevice> {
        let touch_device = unsafe { dmHID::GetTouchDevice(self.ptr, index) };

        TouchDevice::new(touch_device)
    }

    /// Returns the gamepad at the given index if it exists.
    pub fn get_gamepad(&self, index: u8) -> Option<Gamepad> {
        let gamepad = unsafe { dmHID::GetGamepad(self.ptr, index) };

        Gamepad::new(gamepad)
    }

    /// Adds the given character as text input.
    pub fn add_keyboard_char(&self, char: i32) {
        unsafe { dmHID::AddKeyboardChar(self.ptr, char) }
    }
}

/// Wrapper around a [`dmHID::Keyboard`] pointer.
pub struct Keyboard {
    ptr: *mut dmHID::Keyboard,
}

impl Keyboard {
    /// Creates a new [`Keyboard`] from the given pointer.
    ///
    /// You probably want [`Context::get_keyboard()`] instead.
    pub fn new(ptr: *mut dmHID::Keyboard) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Sets whether or not the given key on the keyboard is pressed.
    pub fn set_key(&self, key: Key, value: bool) {
        unsafe { dmHID::SetKey(self.ptr, key.into(), value) }
    }
}

/// Wrapper around a [`dmHID::Mouse`] pointer.
pub struct Mouse {
    ptr: *mut dmHID::Mouse,
}

impl Mouse {
    /// Creates a new [`Mouse`] from the given pointer.
    ///
    /// You probably want [`Context::get_mouse()`] instead.
    pub fn new(ptr: *mut dmHID::Mouse) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Sets whether or not the given mouse button is pressed.
    pub fn set_button(&self, button: MouseButton, value: bool) {
        unsafe { dmHID::SetMouseButton(self.ptr, button.into(), value) }
    }

    /// Sets the position of the cursor.
    pub fn set_position(&self, x: i32, y: i32) {
        unsafe { dmHID::SetMousePosition(self.ptr, x, y) }
    }

    /// Sets the value of the scroll wheel.
    pub fn set_wheel(&self, value: i32) {
        unsafe { dmHID::SetMouseWheel(self.ptr, value) }
    }

    /// Returns a [`MousePacket`] containing the state of the mouse, or [`None`] if it's not connected.
    pub fn get_packet(&self) -> Option<MousePacket> {
        let mut raw_packet = dmHID::MousePacket {
            m_PositionX: 0,
            m_PositionY: 0,
            m_Wheel: 0,
            m_Buttons: [0; 1],
        };

        let ok = unsafe { dmHID::GetMousePacket(self.ptr, &mut raw_packet) };

        if ok {
            Some(raw_packet.into())
        } else {
            None
        }
    }

    /// Returns whether or not the given mouse button is pressed.
    ///
    /// This function fetches a new [`MousePacket`] every time it's called,
    /// so consider using [`Mouse::get_packet()`] and [`MousePacket::get_button()`]
    /// if you need to check the state of multiple buttons at once.
    pub fn get_button(&self, button: MouseButton) -> Option<bool> {
        let packet = self.get_packet()?;

        Some(packet.get_button(button))
    }
}

#[derive(Clone, Copy)]
/// Contains the state of a mouse at a given point in time.
pub struct MousePacket {
    /// X position.
    pub x: i32,
    /// Y position.
    pub y: i32,
    /// Scroll wheel value.
    pub wheel: i32,
    /// Pressed buttons.
    pub buttons: u32,
    raw_packet: dmHID::MousePacket,
}

impl MousePacket {
    /// Returns whether or not the given button is pressed in the packet.
    pub fn get_button(&self, button: MouseButton) -> bool {
        let mut packet_clone = *self;
        unsafe { dmHID::GetMouseButton(&mut packet_clone.raw_packet, button.into()) }
    }
}

impl From<dmHID::MousePacket> for MousePacket {
    fn from(raw_packet: dmHID::MousePacket) -> Self {
        Self {
            x: raw_packet.m_PositionX,
            y: raw_packet.m_PositionY,
            wheel: raw_packet.m_Wheel,
            buttons: raw_packet.m_Buttons[0],
            raw_packet,
        }
    }
}

/// Wrapper around a [`dmHID::TouchDevice`] pointer.
pub struct TouchDevice {
    ptr: *mut dmHID::TouchDevice,
}

impl TouchDevice {
    /// Creates a new [`TouchDevice`] from the given pointer.
    ///
    /// You probably want [`Context::get_touch_device()`] instead.
    pub fn new(ptr: *mut dmHID::TouchDevice) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Adds a touch event.
    pub fn add_touch(&self, x: i32, y: i32, id: u32, phase: Phase) {
        unsafe { dmHID::AddTouch(self.ptr, x, y, id, phase.into()) }
    }
}

/// Wrapper around a [`dmHID::Gamepad`] pointer.
pub struct Gamepad {
    ptr: *mut dmHID::Gamepad,
}

impl Gamepad {
    /// Creates a new [`Gamepad`] from the given pointer.
    ///
    /// You probably want [`Context::get_touch_device()`] instead.
    pub fn new(ptr: *mut dmHID::Gamepad) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Sets whether or not the given button is pressed.
    pub fn set_button(&self, button: u32, pressed: bool) {
        unsafe { dmHID::SetGamepadButton(self.ptr, button, pressed) }
    }

    /// Sets the value of the given gamepad axis.
    pub fn set_axis(&self, axis: u32, value: f32) {
        unsafe { dmHID::SetGamepadAxis(self.ptr, axis, value) }
    }
}

#[allow(missing_docs)]
pub enum Phase {
    Began,
    Moved,
    Stationary,
    Ended,
    Cancelled,
}

impl From<Phase> for u32 {
    fn from(phase: Phase) -> Self {
        phase as u32
    }
}

#[allow(missing_docs)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    M1,
    M2,
    M3,
    M4,
    M5,
    M6,
    M7,
    M8,
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> Self {
        match button {
            MouseButton::Left => 0,
            MouseButton::Middle => 2,
            MouseButton::Right => 1,
            MouseButton::M1 => 0,
            MouseButton::M2 => 1,
            MouseButton::M3 => 2,
            MouseButton::M4 => 3,
            MouseButton::M5 => 4,
            MouseButton::M6 => 5,
            MouseButton::M7 => 6,
            MouseButton::M8 => 7,
        }
    }
}

#[allow(missing_docs)]
#[repr(u32)]
pub enum Key {
    // ASCII numbering
    Space = 32,
    Exclaim,
    DoubleQuote,
    Hash,
    Dollar,
    Percent, // Not in C++ dmHID for some reason
    Ampersand,
    Apostrophe,
    ParenLeft,
    ParenRight,
    Asterisk,
    Plus,
    Comma,
    Minus,
    Period,
    Slash,

    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,

    Colon,
    Semicolon,
    Less,
    Equals,
    Greater,
    Question,
    At,

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

    BracketLeft,
    Backslash,
    BracketRight,
    Caret,
    Underscore,
    Backquote,
    // ...lowercase letters...
    BraceLeft = 123,
    Pipe,
    BraceRight,
    Tilde,

    // GLFW numbering (GLFW_KEY_SPECIAL + X)
    Escape = 256 + 1,
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
    // ...F13-F25...
    Up = 256 + 27,
    Down,
    Left,
    Right,

    ShiftLeft,
    ShiftRight,
    ControlLeft,
    ControlRight,
    AltLeft,
    AltRight,

    Tab,
    Enter,
    Backspace,
    Insert,
    Delete,
    PageUp,
    PageDown,
    Home,
    End,

    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpDecimal,
    KpEqual,
    KpEnter,
    KpNumLock,

    CapsLock,
    ScollLock,
    Pause,
    SuperLeft,
    SuperRight,
    Menu,
    Back,
}

impl From<Key> for u32 {
    fn from(key: Key) -> Self {
        key as u32
    }
}
