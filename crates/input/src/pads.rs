//! Controllers through SDL 3, as WitherBlitz's NSMB-Wii-Recompilation reads
//! them: buttons, sticks and triggers, rumble, and the motion sensors of
//! the controllers that have them (DualShock 4, DualSense, Switch Pro,
//! Joy-Cons and the others SDL knows).
//!
//! One library reads the whole controller because SDL, to read a
//! PlayStation controller's sensors over Bluetooth, or any Switch
//! controller at all, switches it to a report format that Windows' own
//! game-controller view no longer understands.
//!
//! Controllers keep a player slot each, in the order they connect; a slot
//! whose controller went away is the next new controller's.
//! SDL's events are off: [`Pads::update`] asks SDL for the controllers'
//! state and the connected list itself.

use std::ffi::{c_char, CStr};

use sdl3_sys::gamepad::*;
use sdl3_sys::hints::{SDL_SetHint, SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS};
use sdl3_sys::init::{SDL_Init, SDL_INIT_GAMEPAD};
use sdl3_sys::joystick::{SDL_JoystickID, SDL_SetJoystickEventsEnabled};
use sdl3_sys::sensor::SDL_SENSOR_ACCEL;
use sdl3_sys::stdinc::SDL_free;

use crate::padmap::{Axis, Button, PadInput};
use crate::PLAYERS;

struct Slot {
    pad: *mut SDL_Gamepad,
    id: SDL_JoystickID,
    /// Its accelerometer: not asked for yet, on, or not there.
    accel: Accel,
}

#[derive(Clone, Copy, PartialEq)]
enum Accel {
    Unasked,
    On,
    None,
}

pub struct Pads {
    slots: Vec<Option<Slot>>,
}

fn sdl_error() -> String {
    // SAFETY: SDL_GetError returns a valid, NUL-terminated string.
    unsafe { CStr::from_ptr(sdl3_sys::error::SDL_GetError()) }.to_string_lossy().into_owned()
}

fn button(b: Button) -> SDL_GamepadButton {
    match b {
        Button::South => SDL_GAMEPAD_BUTTON_SOUTH,
        Button::East => SDL_GAMEPAD_BUTTON_EAST,
        Button::West => SDL_GAMEPAD_BUTTON_WEST,
        Button::North => SDL_GAMEPAD_BUTTON_NORTH,
        Button::LeftTrigger => SDL_GAMEPAD_BUTTON_LEFT_SHOULDER,
        Button::RightTrigger => SDL_GAMEPAD_BUTTON_RIGHT_SHOULDER,
        Button::Select => SDL_GAMEPAD_BUTTON_BACK,
        Button::Start => SDL_GAMEPAD_BUTTON_START,
        Button::Mode => SDL_GAMEPAD_BUTTON_GUIDE,
        Button::LeftThumb => SDL_GAMEPAD_BUTTON_LEFT_STICK,
        Button::RightThumb => SDL_GAMEPAD_BUTTON_RIGHT_STICK,
        Button::DPadUp => SDL_GAMEPAD_BUTTON_DPAD_UP,
        Button::DPadDown => SDL_GAMEPAD_BUTTON_DPAD_DOWN,
        Button::DPadLeft => SDL_GAMEPAD_BUTTON_DPAD_LEFT,
        Button::DPadRight => SDL_GAMEPAD_BUTTON_DPAD_RIGHT,
        // The triggers are axes (`value`).
        Button::LeftTrigger2 | Button::RightTrigger2 => SDL_GAMEPAD_BUTTON_INVALID,
    }
}

/// SDL's axis, and whether SDL's positive end is ours (SDL's y grows down).
fn axis(a: Axis) -> (SDL_GamepadAxis, bool) {
    match a {
        Axis::LeftStickX => (SDL_GAMEPAD_AXIS_LEFTX, true),
        Axis::LeftStickY => (SDL_GAMEPAD_AXIS_LEFTY, false),
        Axis::RightStickX => (SDL_GAMEPAD_AXIS_RIGHTX, true),
        Axis::RightStickY => (SDL_GAMEPAD_AXIS_RIGHTY, false),
    }
}

impl Pads {
    /// SDL's controllers, and the ones already connected; `None` (said on
    /// the console) if SDL can't start.
    pub fn new() -> Option<Self> {
        // SAFETY: plain SDL calls with NUL-terminated strings, before and
        // after SDL_Init on this thread.
        unsafe {
            // Read whether or not the game's window has the focus, as before.
            SDL_SetHint(SDL_HINT_JOYSTICK_ALLOW_BACKGROUND_EVENTS, c"1".as_ptr());
            if !SDL_Init(SDL_INIT_GAMEPAD) {
                eprintln!("controllers unavailable: {}", sdl_error());
                return None;
            }
            SDL_SetJoystickEventsEnabled(false);
            SDL_SetGamepadEventsEnabled(false);
        }
        Some(Self { slots: Vec::new() })
    }

    /// Reads the controllers' state; closes the ones that went away and
    /// opens new ones. Returns the new ones' slots and names.
    pub fn update(&mut self) -> Vec<(usize, String)> {
        let mut new = Vec::new();
        // SAFETY: SDL was started on this thread; every pointer used is one
        // SDL gave and still owns (closed ones are dropped from the slots).
        unsafe {
            SDL_UpdateGamepads();
            for slot in &mut self.slots {
                if slot.as_ref().is_some_and(|s| !SDL_GamepadConnected(s.pad)) {
                    SDL_CloseGamepad(slot.take().unwrap().pad);
                }
            }
            let mut count = 0;
            let ids = SDL_GetGamepads(&mut count);
            if ids.is_null() {
                return new;
            }
            for &id in std::slice::from_raw_parts(ids, count.max(0) as usize) {
                if self.slots.iter().flatten().any(|s| s.id == id) {
                    continue;
                }
                let free = self.slots.iter().position(Option::is_none);
                if free.is_none() && self.slots.len() >= PLAYERS {
                    break;
                }
                let pad = SDL_OpenGamepad(id);
                if pad.is_null() {
                    eprintln!("controller {}: {}", id.0, sdl_error());
                    continue;
                }
                let slot = Slot { pad, id, accel: Accel::Unasked };
                let at = match free {
                    Some(i) => {
                        self.slots[i] = Some(slot);
                        i
                    }
                    None => {
                        self.slots.push(Some(slot));
                        self.slots.len() - 1
                    }
                };
                let name = SDL_GetGamepadName(pad);
                let mut name = if name.is_null() { "controller".to_string() } else { CStr::from_ptr(name as *const c_char).to_string_lossy().into_owned() };
                if SDL_GamepadHasSensor(pad, SDL_SENSOR_ACCEL) {
                    name.push_str(", motion sensors");
                }
                new.push((at, name));
            }
            SDL_free(ids.cast());
        }
        new
    }

    fn pad(&self, slot: usize) -> Option<*mut SDL_Gamepad> {
        self.slots.get(slot)?.as_ref().map(|s| s.pad)
    }

    pub fn connected(&self, slot: usize) -> bool {
        self.pad(slot).is_some()
    }

    pub fn any_connected(&self) -> bool {
        self.slots.iter().any(Option::is_some)
    }

    /// How far `p` is pushed on the controller in `slot`, 0 to 1.
    pub fn value(&self, slot: usize, p: PadInput) -> f32 {
        let Some(pad) = self.pad(slot) else { return 0.0 };
        let read = |a| f32::from(unsafe { SDL_GetGamepadAxis(pad, a) }) / 32767.0;
        // SAFETY: `pad` is an open gamepad of this thread's SDL.
        match p {
            PadInput::Button(Button::LeftTrigger2) => read(SDL_GAMEPAD_AXIS_LEFT_TRIGGER).clamp(0.0, 1.0),
            PadInput::Button(Button::RightTrigger2) => read(SDL_GAMEPAD_AXIS_RIGHT_TRIGGER).clamp(0.0, 1.0),
            PadInput::Button(b) => f32::from(u8::from(unsafe { SDL_GetGamepadButton(pad, button(b)) })),
            PadInput::Stick(a, positive) => {
                let (a, same) = axis(a);
                let v = read(a).clamp(-1.0, 1.0);
                if positive == same {
                    v.max(0.0)
                } else {
                    (-v).max(0.0)
                }
            }
        }
    }

    /// Runs the controller's motors (0 for both stops them) for at most
    /// `ms`; false if it has none.
    pub fn rumble(&self, slot: usize, low: u16, high: u16, ms: u32) -> bool {
        // SAFETY: an open gamepad of this thread's SDL.
        self.pad(slot).is_some_and(|pad| unsafe { SDL_RumbleGamepad(pad, low, high, ms) })
    }

    /// The controller's accelerometer (m/s², gravity included: x to its
    /// right, y up out of its face, z towards the player), switched on the
    /// first time it is asked for; `None` without one.
    pub fn accel(&mut self, slot: usize) -> Option<[f32; 3]> {
        let s = self.slots.get_mut(slot)?.as_mut()?;
        // SAFETY: an open gamepad of this thread's SDL; `a` holds 3 floats.
        unsafe {
            if s.accel == Accel::Unasked {
                // One that fails to switch on is asked again next time.
                if !SDL_GamepadHasSensor(s.pad, SDL_SENSOR_ACCEL) {
                    s.accel = Accel::None;
                } else if SDL_SetGamepadSensorEnabled(s.pad, SDL_SENSOR_ACCEL, true) {
                    s.accel = Accel::On;
                }
            }
            if s.accel != Accel::On {
                return None;
            }
            let mut a = [0.0f32; 3];
            SDL_GetGamepadSensorData(s.pad, SDL_SENSOR_ACCEL, a.as_mut_ptr(), 3).then_some(a)
        }
    }
}

impl Drop for Pads {
    fn drop(&mut self) {
        for s in self.slots.drain(..).flatten() {
            // SAFETY: an open gamepad of this thread's SDL, closed once.
            unsafe { SDL_CloseGamepad(s.pad) };
        }
    }
}
