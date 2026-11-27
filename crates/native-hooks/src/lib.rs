//! Opt-in switch table: run a Rust body from the `nsmbw` reconstruction in
//! place of the guest function at one address.
//!
//! [`install`] registers the bodies for `NSMBW_RUST_BODIES`. In `1` mode the
//! original still runs and each body is validated on a shadow machine
//! (lockstep); `direct` runs the body instead (not yet cycle-accounted).
//! Each adapter reads its arguments from the incoming GPRs, runs the body under
//! `nsmbw::platform::with_guest_runtime` (so its loads, stores and guest calls
//! use the live `System`), and returns the value for r3.
//!
//! Only the interpreted dispatcher consults this table. Recompiled play uses
//! the hooks baked into `game-recomp`, so the gate runs with `--interpreter`.

use guest_cpu::GuestRuntime;
use hle_os::System;

/// `fBase_c::__ct` (`0x80161AD0`): constructs the node every base derives from.
unsafe fn f_base_ctor(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let this = gprs[3] as usize;
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::resource::registry::fn_80161ad0(this))
        as u32
}

/// Registers the switched bodies when `NSMBW_RUST_BODIES` is `1` (lockstep:
/// the original runs, the body is validated against it), `accounted` (the body
/// runs, cycles come from the shadow original) or `direct` (the body runs with
/// no accounting). `NSMBW_RUST_BODIES_VALIDATE` sets how many calls are
/// validated/accounted (default 16). Otherwise does nothing, so the default
/// port is unchanged.
pub fn install(system: &mut System) {
    let mode = std::env::var("NSMBW_RUST_BODIES").unwrap_or_default();
    let budget = std::env::var("NSMBW_RUST_BODIES_VALIDATE")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(16);
    match mode.as_str() {
        "1" => system.set_native_lockstep(true, budget),
        "accounted" => system.set_native_accounted(true, budget),
        "direct" => system.set_native_lockstep(false, 0),
        _ => return,
    }
    system.register_native(0x8016_1AD0, f_base_ctor);
    eprintln!(
        "rust bodies: {} switched in {mode} mode (0x80161AD0 fBase_c::__ct, budget {budget})",
        system.native_count(),
    );
}
