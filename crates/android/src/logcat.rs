//! stdout and stderr into logcat (tag `nsmbw`): an app's own are not
//! connected to anything, and the game thread, the IOS layer and panics
//! report through them. `adb logcat -s nsmbw` shows the lines.

use std::ffi::{c_char, c_int, CString};
use std::io::{BufRead, BufReader};
use std::os::fd::FromRawFd;
use std::sync::Once;

#[link(name = "log")]
extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

const INFO: c_int = 4;
const WARN: c_int = 5;

/// One line to logcat.
pub fn line(warn: bool, text: &str) {
    let text = CString::new(text.replace('\0', " ")).unwrap_or_default();
    // SAFETY: two valid NUL-terminated strings.
    unsafe {
        __android_log_write(if warn { WARN } else { INFO }, c"nsmbw".as_ptr(), text.as_ptr());
    }
}

/// Once per process: stdout and stderr become pipes, each read by a thread
/// that writes its lines to logcat.
pub fn redirect_std() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        for (fd, warn) in [(1, false), (2, true)] {
            let mut p = [0 as c_int; 2];
            // SAFETY: a fresh pipe; its write end replaces the standard descriptor.
            let read = unsafe {
                if libc::pipe(p.as_mut_ptr()) != 0 {
                    continue;
                }
                libc::dup2(p[1], fd);
                libc::close(p[1]);
                std::fs::File::from_raw_fd(p[0])
            };
            let _ = std::thread::Builder::new().name(if warn { "stderr" } else { "stdout" }.into()).spawn(move || {
                for l in BufReader::new(read).lines() {
                    match l {
                        Ok(l) => line(warn, &l),
                        Err(_) => break,
                    }
                }
            });
        }
    });
}
