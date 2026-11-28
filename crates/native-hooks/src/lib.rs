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

mod generated;

/// Generated bodies (`generated.rs`) the gate runs: each passed it, or 1-1
/// never calls it. `NSMBW_RUST_BODIES_ALL=1` registers every generated body
/// instead, to find out which ones a run calls and which ones differ.
/// `0x8006CAE0` (`draw2DActorOnLyt1`) is left out: it differs in 1-1.
const GATED: &[u32] = &[
    0x8006_4110,
    0x8006_42B0,
    0x8006_4380,
    0x8006_4490,
    0x8006_4600,
    0x8006_4610,
    0x8006_4680,
    0x8006_46E0,
    0x8006_4720,
    0x8006_4790,
    0x8006_47A0,
    0x8006_4A10,
    0x8006_4A30,
    0x8006_4A50,
    0x8006_4AF0,
    0x8006_4B00,
    0x8006_4BD0,
    0x8006_4BE0,
    0x8006_4BF0,
    0x8006_4C00,
    0x8006_4C10,
    0x8006_4D10,
    0x8006_4F50,
    0x8006_5110,
    0x8006_51C0,
    0x8006_5480,
    0x8006_5520,
    0x8006_5A40,
    0x8006_5B80,
    0x8006_5CB0,
    0x8006_5FC0,
    0x8006_6FC0,
    0x8006_71B0,
    0x8006_7270,
    0x8006_C6D0,
    0x8006_C7F0,
    0x8006_C870,
    0x8006_C8A0,
    0x8006_C8B0,
    0x8006_C8F0,
    0x8006_CA70,
    0x8006_CA80,
    0x8006_CBA0,
    0x8009_4E80,
    0x8009_5130,
    0x8009_51D0,
    0x8009_5240,
    0x8009_5480,
    0x8009_54A0,
    0x8009_54D0,
    0x8009_55F0,
    0x8009_57B0,
    0x8009_5C20,
    0x8009_5C80,
    0x8009_6240,
    0x8009_6300,
    0x8009_6330,
    0x8009_64A0,
    0x8009_6700,
    0x8009_6780,
    0x8009_6790,
    0x8009_67A0,
    0x8009_67F0,
    0x8009_6CA0,
    0x8009_6D00,
    0x8009_6E60,
    0x8009_6F60,
    0x8009_7050,
    0x8009_71F0,
    0x8009_7250,
    0x8009_76B0,
    0x8009_7700,
    0x8009_78C0,
    0x8009_79B0,
    0x8009_7A20,
    0x8009_7C40,
    0x8009_7DF0,
    0x8009_7E30,
    0x8009_7E50,
    0x8009_7EA0,
    0x8009_7F40,
    0x8009_7F70,
    0x800A_02D0,
    0x800A_0550,
    0x800A_0700,
    0x800A_0A30,
    0x800A_0C70,
    0x800A_0DC0,
    0x800A_0EA0,
    0x800A_0FB0,
    0x800A_1190,
    0x800A_12A0,
    0x800A_1370,
    0x800A_16A0,
    0x800A_1830,
    0x800A_1980,
    0x800A_1C80,
    0x800A_8170,
    0x800B_56E0,
];

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
    // `makeMtx`: hands `YrotM` the address of its angle copy (a GuestLocal).
    d_base_actor_make_mtx => nsmbw::game::player::fn_8006ccb0,
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

/// `dBaseActor_c::postExecute(MAIN_STATE_e)` (`0x8006C980`): `this` and the state.
unsafe fn d_base_actor_post_execute(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let (this, state) = (gprs[3] as usize, gprs[4]);
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::game::player::fn_8006c980(this, state));
    0
}

/// `dBaseActor_c::preDraw` (`0x8006C9F0`): whether to draw this frame.
unsafe fn d_base_actor_pre_draw(
    runtime: &mut (dyn GuestRuntime + 'static),
    gprs: &[u32; 32],
    _fprs: &[[f64; 2]; 32],
) -> u32 {
    let this = gprs[3] as usize;
    nsmbw::platform::with_guest_runtime(runtime, || nsmbw::game::player::fn_8006c9f0(this))
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

/// Bodies that hand the game the address of one of their locals (a
/// `GuestLocal`): two words in, one word out. Their gate lines prove the
/// locals mechanism on live calls.
macro_rules! two_words {
    ($($name:ident, |$a:ident, $b:ident| $call:expr;)*) => {$(
        unsafe fn $name(
            runtime: &mut (dyn GuestRuntime + 'static),
            gprs: &[u32; 32],
            _fprs: &[[f64; 2]; 32],
        ) -> u32 {
            let ($a, $b) = (gprs[3], gprs[4]);
            nsmbw::platform::with_guest_runtime(runtime, || $call)
        }
    )*};
}

two_words! {
    // `0x800669B0`: is the actor's position outside the scene rectangle?
    d_actor_outside_scene,
        |a, b| nsmbw::game::player::fn_800669b0(a, b as usize);
    // `0x80065080`: the sprite position and rectangle cull test.
    d_actor_sprite_cull,
        |a, _b| nsmbw::game::player::fn_80065080(a as usize);
    // `0x80064740`: probes the actor's centre for the find policy.
    d_actor_probe_centre,
        |a, b| nsmbw::game::player::fn_80064740(a as usize, b as usize as *mut f32);
    // `0x80065AC0`: fills a stack matrix and stores its translation.
    d_actor_matrix_translation,
        |a, b| { nsmbw::game::player::fn_80065ac0(a as usize, b); 0 };
    // `0x8006A5E0`: passes a float pair and its swap.
    d_actor_float_pair,
        |a, b| { nsmbw::game::player::fn_8006a5e0(a, b); 0 };
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
    system.register_native(0x8006_C9F0, d_base_actor_pre_draw);
    system.register_native_void(0x8006_C980, d_base_actor_post_execute);
    // Functions that return nothing: their checks compare RAM, not r3.
    system.register_native_void(0x8006_CC00, d_base_actor_calc_speed);
    system.register_native_void(0x8006_CCB0, d_base_actor_make_mtx);
    // Candidates that hand the game a local's address (see `two_words!`).
    system.register_native(0x8006_69B0, d_actor_outside_scene);
    system.register_native(0x8006_5080, d_actor_sprite_cull);
    system.register_native(0x8006_4740, d_actor_probe_centre);
    system.register_native_void(0x8006_5AC0, d_actor_matrix_translation);
    system.register_native_void(0x8006_A5E0, d_actor_float_pair);
    system.register_native_void(0x8006_CD00, d_base_actor_pos_move_by);
    system.register_native_void(0x8006_CD40, d_base_actor_pos_move);
    system.register_native_void(0x8006_CD50, d_base_actor_calc_speed_xy);
    system.register_native_void(0x8006_CD90, d_base_actor_calc_speed_x);
    system.register_native_void(0x8006_CDE0, d_base_actor_calc_speed_y);
    system.register_native_void(0x8006_CE10, d_base_actor_calc_speed_f);
    system.register_native_void(0x8006_CE70, d_base_actor_calc_fall_speed);
    system.register_native_void(0x8006_CED0, d_base_actor_get_center_pos);
    let all = std::env::var_os("NSMBW_RUST_BODIES_ALL").is_some();
    for &(addr, body, void) in generated::BODIES {
        if all || GATED.contains(&addr) {
            if void {
                system.register_native_void(addr, body);
            } else {
                system.register_native(addr, body);
            }
        }
    }
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
