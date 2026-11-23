//! Which build of the game this processor runs. A Windows release holds two:
//! `nsmbw-port.exe` for x86-64-v3 (AVX2, FMA, BMI2: Intel Haswell, AMD
//! Excavator and newer) and `compat\nsmbw-port.exe` for x86-64-v2 (SSE4.2,
//! POPCNT: Intel Nehalem, AMD Bulldozer and newer). The launcher itself is
//! built for any x86-64, so it runs on both kinds to choose.

/// Set to anything, the compat build is used whatever the processor (for
/// testing it). The game's own check reads it too.
pub const FORCE_ENV: &str = "NSMBW_FORCE_COMPAT";

/// The compat build's folder, next to the game program.
pub const COMPAT_DIR: &str = port_update::COMPAT_DIR;

/// Start the compat build rather than the normal one.
pub fn wants_compat() -> bool {
    std::env::var_os(FORCE_ENV).is_some() || !has_v3()
}

/// Has the processor every feature of x86-64-v3, and does Windows save the
/// AVX registers (`avx` is only reported when it does).
pub fn has_v3() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::is_x86_feature_detected as has;
        has!("avx") && has!("avx2") && has!("fma") && has!("bmi1") && has!("bmi2") && has!("lzcnt") && has!("movbe") && has!("f16c") && has!("xsave") && has_v2()
    }
    #[cfg(not(target_arch = "x86_64"))]
    true
}

/// Has the processor every feature of x86-64-v2, which even the compat
/// build needs.
pub fn has_v2() -> bool {
    #[cfg(target_arch = "x86_64")]
    {
        use std::arch::is_x86_feature_detected as has;
        has!("sse3") && has!("ssse3") && has!("sse4.1") && has!("sse4.2") && has!("popcnt") && has!("cmpxchg16b")
    }
    #[cfg(not(target_arch = "x86_64"))]
    true
}
