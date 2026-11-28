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
/// Left out: they differ from the original in 1-1 (docs/RUST_PLAN.md):
/// 0x8007CA90, 0x800773E0, 0x8007A6C0, 0x8006D170, 0x80075CA0, and with more
/// than 16 calls checked 0x80075820, 0x8007CD70, 0x800DAA10.
const GATED: &[u32] = &[
    0x8007_7B00,
    0x8016_EF50,
    0x8007_0650,
    0x8006_47A0,
    0x800D_9860,
    0x800D_9880,
    0x800D_98C0,
    0x800D_98F0,
    0x800D_9900,
    0x800D_9920,
    0x800D_9940,
    0x800D_9A30,
    0x800D_9AC0,
    0x800D_9AD0,
    0x800D_9B60,
    0x800D_9D00,
    0x800D_9D40,
    0x800D_9D70,
    0x800D_A650,
    0x8010_1950,
    0x8010_1960,
    0x8010_1BC0,
    0x8010_1FD0,
    0x8010_2060,
    0x8010_20F0,
    0x8010_2140,
    0x8010_21B0,
    0x8010_21D0,
    0x8010_2220,
    0x8010_2260,
    0x8005_EC90,
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
    0x8006_5A00,
    0x8006_5A40,
    0x8006_5B80,
    0x8006_5CB0,
    0x8006_5FC0,
    0x8006_6420,
    0x8006_6490,
    0x8006_64F0,
    0x8006_6540,
    0x8006_6570,
    0x8006_65D0,
    0x8006_6630,
    0x8006_6740,
    0x8006_67B0,
    0x8006_6820,
    0x8006_68F0,
    0x8006_6A40,
    0x8006_6C60,
    0x8006_6EA0,
    0x8006_6EC0,
    0x8006_6F70,
    0x8006_6FC0,
    0x8006_71B0,
    0x8006_7270,
    0x8006_90F0,
    0x8006_9110,
    0x8006_9240,
    0x8006_9270,
    0x8006_93E0,
    0x8006_C6D0,
    0x8006_C7F0,
    0x8006_C870,
    0x8006_C8A0,
    0x8006_C8B0,
    0x8006_C8F0,
    0x8006_CA70,
    0x8006_CA80,
    0x8006_CAE0,
    0x8006_CBA0,
    0x8006_CF40,
    0x8006_CFA0,
    0x8006_CFE0,
    0x8006_D090,
    0x8006_D120,
    0x8006_D160,
    0x8006_D3B0,
    0x8006_D440,
    0x8006_D460,
    0x8006_D6D0,
    0x8006_D770,
    0x8006_D8A0,
    0x8006_D9B0,
    0x8006_DB30,
    0x8006_DD50,
    0x8006_DE10,
    0x8006_DFC0,
    0x8006_E490,
    0x8006_E720,
    0x8006_EDB0,
    0x8006_EF40,
    0x8006_F0E0,
    0x8006_F1D0,
    0x8006_F3E0,
    0x8006_F600,
    0x8006_F750,
    0x8006_F7E0,
    0x8006_FA40,
    0x8006_FAA0,
    0x8006_FBF0,
    0x8006_FCF0,
    0x8006_FEA0,
    0x8007_0130,
    0x8007_04D0,
    0x8007_0530,
    0x8007_05B0,
    0x8007_0600,
    0x8007_0630,
    0x8007_06A0,
    0x8007_06E0,
    0x8007_0720,
    0x8007_0730,
    0x8007_0740,
    0x8007_0760,
    0x8007_07E0,
    0x8007_0820,
    0x8007_0870,
    0x8007_08C0,
    0x8007_0900,
    0x8007_0920,
    0x8007_0960,
    0x8007_0980,
    0x8007_09C0,
    0x8007_0A40,
    0x8007_0B10,
    0x8007_0BA0,
    0x8007_0BF0,
    0x8007_0C40,
    0x8007_0C60,
    0x8007_0C80,
    0x8007_0CA0,
    0x8007_0CB0,
    0x8007_0CD0,
    0x8007_0E70,
    0x8007_0EF0,
    0x8007_1060,
    0x8007_1210,
    0x8007_13B0,
    0x8007_14D0,
    0x8007_17A0,
    0x8007_1A60,
    0x8007_1DC0,
    0x8007_2180,
    0x8007_2440,
    0x8007_28C0,
    0x8007_2BC0,
    0x8007_2CB0,
    0x8007_2DB0,
    0x8007_2F10,
    0x8007_3070,
    0x8007_31E0,
    0x8007_3670,
    0x8007_3B40,
    0x8007_3FC0,
    0x8007_43D0,
    0x8007_4AF0,
    0x8007_4FF0,
    0x8007_5070,
    0x8007_5090,
    0x8007_51C0,
    0x8007_5230,
    0x8007_5270,
    0x8007_5560,
    0x8007_5750,
    0x8007_5780,
    0x8007_57B0,
    0x8007_57E0,
    0x8007_5800,
    0x8007_5FD0,
    0x8007_6530,
    0x8007_6630,
    0x8007_67F0,
    0x8007_68D0,
    0x8007_6FD0,
    0x8007_7020,
    0x8007_7060,
    0x8007_7490,
    0x8007_74A0,
    0x8007_7520,
    0x8007_75A0,
    0x8007_7610,
    0x8007_7650,
    0x8007_76F0,
    0x8007_7700,
    0x8007_7730,
    0x8007_77B0,
    0x8007_7810,
    0x8007_7860,
    0x8007_7980,
    0x8007_7A00,
    0x8007_7B40,
    0x8007_7D80,
    0x8007_7EB0,
    0x8007_8250,
    0x8007_8300,
    0x8007_8410,
    0x8007_84D0,
    0x8007_8510,
    0x8007_8520,
    0x8007_8540,
    0x8007_85E0,
    0x8007_8670,
    0x8007_8780,
    0x8007_8800,
    0x8007_8830,
    0x8007_89A0,
    0x8007_8A70,
    0x8007_8B40,
    0x8007_8C10,
    0x8007_8CE0,
    0x8007_8CF0,
    0x8007_8D00,
    0x8007_8EB0,
    0x8007_8EF0,
    0x8007_9020,
    0x8007_9060,
    0x8007_91D0,
    0x8007_9250,
    0x8007_9380,
    0x8007_94B0,
    0x8007_95E0,
    0x8007_9710,
    0x8007_9780,
    0x8007_99E0,
    0x8007_9AA0,
    0x8007_9B60,
    0x8007_9BF0,
    0x8007_9E80,
    0x8007_A6B0,
    0x8007_AB70,
    0x8007_AC40,
    0x8007_BEF0,
    0x8007_E0E0,
    0x8008_1D20,
    0x8008_1F40,
    0x8008_2050,
    0x8008_20D0,
    0x8008_2160,
    0x8008_2180,
    0x8008_21E0,
    0x8008_2240,
    0x8008_C200,
    0x8008_C260,
    0x8008_C2C0,
    0x8008_C330,
    0x8008_C380,
    0x8008_C3E0,
    0x8008_C440,
    0x8008_C4D0,
    0x8008_C4F0,
    0x8008_C510,
    0x8008_C530,
    0x8008_C550,
    0x8008_C570,
    0x8008_C6E0,
    0x8008_CA50,
    0x8008_D3D0,
    0x8008_D460,
    0x8008_D820,
    0x8008_D8B0,
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
    0x800A_74C0,
    0x800A_7540,
    0x800A_7550,
    0x800A_7560,
    0x800A_7690,
    0x800A_77A0,
    0x800A_7860,
    0x800A_7890,
    0x800A_78D0,
    0x800A_7BF0,
    0x800A_7C00,
    0x800A_7C60,
    0x800A_8170,
    0x800B_56E0,
    0x8010_1AA0,
    0x8010_25C0,
    0x8014_4D50,
    0x8008_2120,
    0x8008_2140,
    0x8016_EBF0,
    0x8016_ED10,
    0x8016_EDD0,
    0x8016_EE30,
    0x8016_EEF0,
    0x8016_F130,
    0x8016_F150,
    0x8016_ECB0,
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
    // Addresses to leave out (hex, comma-separated), e.g. one that hangs.
    let skip = std::env::var("NSMBW_RUST_BODIES_SKIP").unwrap_or_default();
    let skipped = |addr: u32| {
        skip.split(',').any(|s| u32::from_str_radix(s.trim().trim_start_matches("0x"), 16).ok() == Some(addr))
    };
    for &(addr, body, kind) in generated::BODIES {
        if (all || GATED.contains(&addr)) && !skipped(addr) {
            match kind {
                generated::Kind::Word => system.register_native(addr, body),
                generated::Kind::Void => system.register_native_void(addr, body),
                generated::Kind::Float => system.register_native_float(addr, body),
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
