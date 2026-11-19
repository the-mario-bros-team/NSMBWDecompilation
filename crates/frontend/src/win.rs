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
