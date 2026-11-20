//! Windows process tuning, as in WitherBlitz's port: above-normal priority,
//! and no power-saving slowdown of the game's threads.

#[cfg(windows)]
pub fn tune_process() {
    use windows_sys::Win32::System::Threading::{
        GetCurrentProcess, ProcessPowerThrottling, SetPriorityClass, SetProcessInformation, ABOVE_NORMAL_PRIORITY_CLASS, PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        PROCESS_POWER_THROTTLING_EXECUTION_SPEED, PROCESS_POWER_THROTTLING_STATE,
    };
    // SAFETY: plain Win32 calls on the current process with a valid struct.
    unsafe {
        let me = GetCurrentProcess();
        SetPriorityClass(me, ABOVE_NORMAL_PRIORITY_CLASS);
        let state = PROCESS_POWER_THROTTLING_STATE { Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION, ControlMask: PROCESS_POWER_THROTTLING_EXECUTION_SPEED, StateMask: 0 };
        SetProcessInformation(me, ProcessPowerThrottling, (&state as *const PROCESS_POWER_THROTTLING_STATE).cast(), std::mem::size_of::<PROCESS_POWER_THROTTLING_STATE>() as u32);
    }
}

#[cfg(not(windows))]
pub fn tune_process() {}

/// CPU time the calling thread has used so far (kernel plus user). Unlike
/// wall time, it does not grow while other processes take the CPU, so speed
/// measurements hold up on a busy machine.
#[cfg(windows)]
pub fn thread_cpu_time() -> std::time::Duration {
    use windows_sys::Win32::Foundation::FILETIME;
    use windows_sys::Win32::System::Threading::{GetCurrentThread, GetThreadTimes};
    let mut t = [FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 }; 4];
    // SAFETY: the current thread's pseudo-handle and four valid FILETIMEs.
    let ok = unsafe { GetThreadTimes(GetCurrentThread(), &mut t[0], &mut t[1], &mut t[2], &mut t[3]) } != 0;
    let ticks = |f: &FILETIME| (u64::from(f.dwHighDateTime) << 32) | u64::from(f.dwLowDateTime);
    // 100 ns units.
    if ok {
        std::time::Duration::from_nanos((ticks(&t[2]) + ticks(&t[3])) * 100)
    } else {
        Default::default()
    }
}

#[cfg(not(windows))]
pub fn thread_cpu_time() -> std::time::Duration {
    Default::default()
}
