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

/// `dActor_c::preExecute` (`0x800643F0`): the per-frame teardown gate.
unsafe fn d_actor_pre_execute(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let this = gprs[3] as usize;
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::game::player::fn_800643f0(this))
}

/// `dActor_c::preDraw` (`0x80064540`): the "may step" predicate.
unsafe fn d_actor_pre_draw(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let this = gprs[3] as usize;
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::game::player::fn_80064540(this))
}

/// `dBaseActor_c`'s movement functions (`calcSpeed` and family): `this` only.
macro_rules! this_only {
    ($($name:ident => $body:path),* $(,)?) => {$(
        unsafe fn $name(
            runtime: &mut (dyn GuestRuntime + 'static),
            gprs: &[u32; 32],
            _fprs: &[[f64; 2]; 32],
        ) -> u32 {
            let this = gprs[3] as usize;
            nsmbw::platform::with_guest_runtime(runtime, || $body(this));
            0
        }
    )*};
}

this_only! {
    d_base_actor_calc_speed => nsmbw::game::player::fn_8006cc00,
    d_base_actor_pos_move => nsmbw::game::player::fn_8006cd40,
    d_base_actor_calc_speed_xy => nsmbw::game::player::fn_8006cd50,
    d_base_actor_calc_speed_x => nsmbw::game::player::fn_8006cd90,
    d_base_actor_calc_speed_y => nsmbw::game::player::fn_8006cde0,
    d_base_actor_calc_speed_f => nsmbw::game::player::fn_8006ce10,
    d_base_actor_calc_fall_speed => nsmbw::game::player::fn_8006ce70,
}

/// `dBaseActor_c::posMove(mVec3_c&)` (`0x8006CD00`): `this` and the step.
unsafe fn d_base_actor_pos_move_by(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let (this, delta) = (gprs[3] as usize, gprs[4] as usize);
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::game::player::fn_8006cd00(this, delta));
    0
}

/// `dBaseActor_c::getCenterPos() const` (`0x8006CED0`): returns a `mVec3_c`
/// by value, so `r3` is the result's address and `r4` is `this`.
unsafe fn d_base_actor_get_center_pos(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let (out, this) = (gprs[3] as usize, gprs[4] as usize);
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::game::player::fn_8006ced0(out, this));
    0
}

/// Registers the switched bodies when `NSMBW_RUST_BODIES` is `1` (lockstep:
/// the original runs, the body is validated against it), `accounted` (the body
/// runs, cycles come from the shadow original) or `direct` (the body runs with
/// no accounting). `NSMBW_RUST_BODIES_VALIDATE` sets how many calls are
/// validated per body in lockstep, or accounted in all (default 16). Otherwise
/// does nothing, so the default
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
    system.register_native(0x8006_43F0, d_actor_pre_execute);
    system.register_native(0x8006_4540, d_actor_pre_draw);
    // Functions that return nothing: their checks compare RAM, not r3.
    system.register_native_void(0x8006_CC00, d_base_actor_calc_speed);
    system.register_native_void(0x8006_CD00, d_base_actor_pos_move_by);
    system.register_native_void(0x8006_CD40, d_base_actor_pos_move);
    system.register_native_void(0x8006_CD50, d_base_actor_calc_speed_xy);
    system.register_native_void(0x8006_CD90, d_base_actor_calc_speed_x);
    system.register_native_void(0x8006_CDE0, d_base_actor_calc_speed_y);
    system.register_native_void(0x8006_CE10, d_base_actor_calc_speed_f);
    system.register_native_void(0x8006_CE70, d_base_actor_calc_fall_speed);
    system.register_native_void(0x8006_CED0, d_base_actor_get_center_pos);
    let set = std::env::var("NSMBW_RUST_BODIES_SET").unwrap_or_default();
    if !set.is_empty() {
        let wanted = |addr: u32| {
            set.split(',').any(|s| {
                u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok() == Some(addr)
            })
        };
        system.retain_native(|addr| wanted(addr));
    }
    eprintln!(
        "rust bodies: {} switched in {mode} mode (budget {budget} per body)",
        system.native_count(),
    );
}
