//! stdout and stderr into logcat (tag `nsmbw`): an app's own are not
//! connected to anything, and the game thread, the IOS layer and panics
//! report through them. `adb logcat -s nsmbw` shows the lines. Once the
//! app's files are known ([`keep_in`]), each line also goes to this run's
//! log file there, for the app's Download Logs.

use std::ffi::{c_char, c_int, CString};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::os::fd::FromRawFd;
use std::path::Path;
use std::sync::{Mutex, Once};
use std::time::{SystemTime, UNIX_EPOCH};

#[link(name = "log")]
extern "C" {
    fn __android_log_write(prio: c_int, tag: *const c_char, text: *const c_char) -> c_int;
}

const INFO: c_int = 4;
const WARN: c_int = 5;

/// This run's log file.
static FILE: Mutex<Option<File>> = Mutex::new(None);

/// Runs whose logs are kept (the newest).
const KEEP: usize = 5;

/// From now on, lines also go to a new file in `dir`, named by the time
/// (`run-<seconds since 1970>.log`); the oldest beyond [`KEEP`] go.
pub fn keep_in(dir: &Path) {
    let _ = std::fs::create_dir_all(dir);
    let mut old: Vec<_> = std::fs::read_dir(dir).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.extension().is_some_and(|e| e == "log")).collect();
    old.sort();
    for p in old.iter().rev().skip(KEEP - 1) {
        let _ = std::fs::remove_file(p);
    }
    let secs = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs());
    if let Ok(f) = File::create(dir.join(format!("run-{secs:012}.log"))) {
        *FILE.lock().unwrap() = Some(f);
    }
}

/// One line to logcat (and the log file).
pub fn line(warn: bool, text: &str) {
    if let Some(f) = FILE.lock().unwrap().as_mut() {
        let _ = writeln!(f, "{}{text}", if warn { "! " } else { "" });
    }
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
