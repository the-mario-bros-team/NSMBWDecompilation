//! Opt-in switch table: run a Rust body from the `nsmbw` reconstruction in
//! place of the guest function at one address.
//!
//! [`install`] registers the bodies enabled by `NSMBW_RUST_BODIES=1`. Each
//! adapter reads its arguments from the incoming GPRs, runs the body under
//! `nsmbw::platform::with_guest_runtime` (so its loads, stores and guest calls
//! use the live `System`), and returns the value for r3. `hle_os::System`
//! returns to `lr` and writes r3.
//!
//! Only the interpreted dispatcher consults this table. Recompiled play uses
//! the hooks baked into `game-recomp`, so the gate runs with `--interpreter`.

use hle_os::System;

/// `fBase_c::__ct` (`0x80161AD0`): constructs the node every base derives from.
unsafe fn f_base_ctor(system: &mut System, gprs: &[u32; 32], _fprs: &[[f64; 2]; 32]) -> u32 {
    let this = gprs[3] as usize;
    nsmbw::platform::with_guest_runtime(system, || nsmbw::resource::registry::fn_80161ad0(this))
        as u32
}

/// Registers the switched bodies when `NSMBW_RUST_BODIES=1`; otherwise does
/// nothing, so the default port is unchanged.
pub fn install(system: &mut System) {
    if std::env::var("NSMBW_RUST_BODIES").as_deref() != Ok("1") {
        return;
    }
    system.register_native(0x8016_1AD0, f_base_ctor);
    eprintln!(
        "rust bodies: {} switched (0x80161AD0 fBase_c::__ct)",
        system.native_count()
    );
}
