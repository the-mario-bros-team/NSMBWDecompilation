//! The frontend's `win` functions (`emu.rs` calls them) for Android: the
//! game and GX threads above the app's other threads, and thread CPU time.

use std::time::Duration;

#[allow(dead_code)]
pub fn tune_process() {}

/// Nice -8 (Android's "urgent display" level), as games' main threads use.
/// An app may raise its own threads this far; a refusal leaves it as it is.
pub fn raise_this_thread() {
    // SAFETY: plain syscalls on the calling thread.
    unsafe {
        let tid = libc::gettid();
        libc::setpriority(libc::PRIO_PROCESS, tid as libc::id_t, -8);
    }
}

#[allow(dead_code)]
pub fn sleep(d: Duration) {
    std::thread::sleep(d);
}

/// CPU time the calling thread has used so far.
pub fn thread_cpu_time() -> Duration {
    let mut t = libc::timespec { tv_sec: 0, tv_nsec: 0 };
    // SAFETY: a valid timespec for the calling thread's clock.
    if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &mut t) } == 0 {
        Duration::new(t.tv_sec as u64, t.tv_nsec as u32)
    } else {
        Duration::ZERO
    }
}
