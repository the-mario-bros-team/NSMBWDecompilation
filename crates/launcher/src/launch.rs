//! Starting the game, and the few things asked of Windows directly: is
//! Shift (or a controller's Back) held, and opening a web page.

use std::path::Path;
use std::process::{Child, Command, Stdio};

use crate::store::Store;
use crate::versions::Version;

/// The game's console output goes here, in the version's folder.
pub const LOG: &str = "nsmbw-port.log";

/// Keeps a console program from opening a console window of its own.
pub fn no_console(command: &mut Command) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    #[cfg(not(windows))]
    let _ = command;
}

/// Why `version` cannot be started right now, in words for the player.
pub fn problem(store: &Store, version: &Version) -> Option<String> {
    if !version.playable {
        return Some(if version.status.is_empty() { format!("The port cannot run {} yet.", version.title) } else { version.status.clone() });
    }
    let Some(game) = store.game_dir(version) else {
        return Some(format!("No game folder is set for {}.", version.title));
    };
    if !game.join("sys").join("boot.bin").is_file() {
        return Some(format!("The game folder is not there any more: {}", game.display()));
    }
    let exe = store.game_exe();
    if !exe.is_file() {
        return Some(format!("The game program is missing: {}", exe.display()));
    }
    None
}

/// Starts the game for `version`: `nsmbw-port --game <folder>` with the
/// version's folder as its working folder, so its settings and saves are its
/// own. The game's output goes to [`LOG`] there.
pub fn start(store: &Store, version: &Version) -> Result<Child, String> {
    if let Some(p) = problem(store, version) {
        return Err(p);
    }
    let (exe, dir) = (store.game_exe(), store.version_dir(version));
    let game = store.game_dir(version).expect("problem() saw it");
    std::fs::create_dir_all(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    let mut command = Command::new(&exe);
    command.arg("--game").arg(&game).args(store.options(version)).current_dir(&dir).stdin(Stdio::null());
    match std::fs::File::create(dir.join(LOG)).and_then(|f| Ok((f.try_clone()?, f))) {
        Ok((out, err)) => {
            command.stdout(out).stderr(err);
        }
        Err(_) => {
            command.stdout(Stdio::null()).stderr(Stdio::null());
        }
    }
    no_console(&mut command);
    command.spawn().map_err(|e| format!("{} could not be started: {e}", exe.display()))
}

/// The last lines the game wrote before it stopped, for an error message.
pub fn log_tail(dir: &Path) -> String {
    let text = std::fs::read_to_string(dir.join(LOG)).unwrap_or_default();
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    let tail = &lines[lines.len().saturating_sub(4)..];
    let mut tail = tail.join("\n");
    if tail.chars().count() > 600 {
        tail = tail.chars().take(600).collect::<String>() + "...";
    }
    tail
}

/// Is a Shift key down right now.
pub fn shift_held() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_SHIFT};
        // SAFETY: a plain query of the keyboard's state.
        unsafe { GetAsyncKeyState(i32::from(VK_SHIFT)) as u16 & 0x8000 != 0 }
    }
    #[cfg(not(windows))]
    false
}

/// Is Back (View) held on any Xbox-style controller right now.
pub fn pad_back_held() -> bool {
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::Input::XboxController::{XInputGetState, XINPUT_GAMEPAD_BACK, XINPUT_STATE};
        (0..4).any(|i| {
            let mut state: XINPUT_STATE = unsafe { std::mem::zeroed() };
            // SAFETY: `state` is a valid XINPUT_STATE to fill in.
            unsafe { XInputGetState(i, &mut state) == 0 && state.Gamepad.wButtons & XINPUT_GAMEPAD_BACK != 0 }
        })
    }
    #[cfg(not(windows))]
    false
}

/// Opens a web page in the player's browser. Only web addresses
/// (`port_update::is_web_url`) are handed to Windows.
pub fn open_url(url: &str) -> bool {
    if !port_update::is_web_url(url) {
        return false;
    }
    #[cfg(windows)]
    {
        use windows_sys::Win32::UI::Shell::ShellExecuteW;
        use windows_sys::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;
        let wide = |s: &str| -> Vec<u16> { s.encode_utf16().chain(Some(0)).collect() };
        // SAFETY: both strings are NUL-terminated and outlive the call.
        let r = unsafe { ShellExecuteW(std::ptr::null_mut(), wide("open").as_ptr(), wide(url).as_ptr(), std::ptr::null(), std::ptr::null(), SW_SHOWNORMAL) };
        r as usize > 32
    }
    #[cfg(not(windows))]
    false
}

/// Opens a folder in Explorer.
pub fn open_folder(dir: &Path) {
    let _ = Command::new("explorer").arg(dir).spawn();
}
