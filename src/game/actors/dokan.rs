//! The `obj_dokan` pipe actor: spawn-parameter decode, BRRES model creation,
//! per-frame rotation, and its delete/create lifecycle hooks
//! (`0x80041C00..0x80042A70`).
//!
//! `obj_dokan` is a pipe/valve obstacle made of a central model plus 32
//! repeated pipe segments; its execute hook spins it. Five functions cover the
//! whole class: spawn and initialise, load `g3d/obj_dokan.brres`, rotate each
//! frame, unlink on delete, and forward the create callback. The recovered
//! strings `obj_dokan_A/B/CU/CD/D` and `g3d/obj_dokan.brres` are what tie these
//! addresses together (`// inferred:` from those strings and the shared field
//! layout; the image carries no symbol table).
//!
//! Promoted from the range module `src/subsystems/r80040_80080.rs` by the
//! promotion pass; that file still stages the player-side functions of the same
//! address range.
//!
//! # Dependencies
//!
//! [`crate::platform`] for volatile access and indirect calls, plus the small
//! private ABI shims below (`call4..call6`, `read_i16`, `write_i16`,
//! `write_f32`). `platform` exposes the byte/word accessors and `call0..call3`;
//! this actor also needs 16-bit signed access, `f32` stores and six-argument
//! calls, and every range module keeps the same private set so that promoting a
//! function never changes the ABI helpers it uses. Callees outside this module
//! are kept as raw addresses with a comment.
//!
//! # Start here
//!
//! [`fn_80041c00`] - it decodes the spawn parameter word and is the clearest
//! entry into the class. Then read [`fn_80041d40`], which loads the BRRES and
//! creates the models, and [`fn_80042350`] for the per-frame rotation.
//!
//! # Object layout (the fields this module touches)
//!
//! | Offset | Type | Meaning |
//! |---|---|---|
//! | `+0x04` | `u32` | spawn parameters (`OFF_PARAMS`) |
//! | `+0x60` | `*vtable` | virtual table, slots `+0xD4/+0xDC/+0xE8` (`OFF_VTABLE`) |
//! | `+0xAC` | `float[3]` | world position (`OFF_POS`) |
//! | `+0x104` | `i16` | heading angle (`OFF_ANGLE`) |
//! | `+0x38D` | `u8` | owning-player index (`OFF_PLAYER`) |
//! | `+0x394` | object | model setup context |
//! | `+0x3B0` | handle | loaded BRRES |
//! | `+0x3B4` | model | main pipe model |
//! | `+0x3C0` | 32 models | `B` segments, stride `0xC` |
//! | `+0x3FC` | list node | unlinked by the delete hook |
//! | `+0x540` | model | `CD` cap model |
//! | `+0x54C`, `+0x54E` | `i16` | mirrored/offset heading angles |
//! | `+0x550` | `i16` | rotation delta per frame (`// inferred:`) |
//! | `+0x558` | `u32` | variant selected by the spawn parameters (`OFF_MODE`) |
//!
//! # Unknowns
//!
//! * `// unknown:` sound id `0x70E` in [`fn_80041c00`]; the sample is not
//!   resolved.
//! * `// unknown:` the transform fields at `+0x318..+0x32C`, `+0x554` and
//!   `+0xDC..+0xE4`; the values are exact, the roles are inferred.
//! * `// unknown:` mode `3` in [`fn_80041d40`] would index one entry past both
//!   model-name tables; no valid caller is known to pass it.
//!
//! # Related, still unclassified
//!
//! `0x80045710` (732 bytes) loads the same BRRES with `A`/`B` models and four
//! animation bindings; `0x800460D0` (828 bytes) is a large constructor that
//! likely owns this object. Both stay staged until a lane reads them.

use crate::platform;

// ---------------------------------------------------------------------------
// Small-data bases and the few globals this actor reads.
// ---------------------------------------------------------------------------

/// `r13` small-data base, observed at `0x80004244/0x80004248` (`lis`/`ori`):
/// `0x8042F680`. The companion `r2` base is `0x80433080`, observed at
/// `0x8000423C/0x80004240`; this module uses no `r2`-relative value. The same
/// pair is recorded in `src/engine/startup.rs`.
const SDA_BASE: usize = 0x8042_F680;

/// `r13-0x5648` → `0x8042A038`. Holds a pointer to the running archive/resource
/// manager; the dokan loader passes `*ptr + 4` as the archive handle.
/// `// inferred:` role from the `0x800DF180(handle, name, path)` call.
const G_ARCHIVE_MANAGER: usize = SDA_BASE - 0x5648;

/// `r13-0x5920` → `0x80429D60`. First argument of `0x80198C30` in the dokan
/// spawn path.
/// `// inferred:` sound-system handle (the callee allocates a voice slot).
const G_SOUND_SYSTEM: usize = SDA_BASE - 0x5920;

/// `0x80377C48`, read `DAT_80377c48` in Ghidra. Passed to `0x800690C0` while
/// preparing the dokan model.
/// `// unknown:` a BSS handle (allocator or model-manager context); the value
/// is written at runtime, so it is read here rather than hard-coded.
const G_MODEL_CONTEXT: usize = 0x8037_7C48;

// ---------------------------------------------------------------------------
// Field offsets used by more than one function.
// ---------------------------------------------------------------------------

/// Spawn parameter word: bits 4..5 select the dokan variant, bits 12..13 the
/// owning player, bits 16..23 the actor kind.
const OFF_PARAMS: usize = 0x004;

/// Pointer to the class vtable; slots `+0xD4` (spawn), `+0xDC` (execute) and
/// `+0xE8` (destructor) are used in this file.
const OFF_VTABLE: usize = 0x060;

/// `float[3]` world position, handed to the screen-projection helper.
const OFF_POS: usize = 0x0AC;

/// 16-bit heading angle; one full turn is `0x10000` (16-bit angle units).
const OFF_ANGLE: usize = 0x104;

/// `u8` owning-player index (0..3); written from the spawn parameters.
const OFF_PLAYER: usize = 0x38D;

/// `u32` dokan variant selected by the spawn parameters (observed 0..2).
const OFF_MODE: usize = 0x558;

// ---------------------------------------------------------------------------
// Local ABI shims. `platform` exposes the byte/word accessors and
// `call0..call3`; this actor also needs 16-bit signed access, `f32` stores and
// six-argument calls.
// ---------------------------------------------------------------------------

/// Read a signed 16-bit field (`lha` in the original).
#[inline]
unsafe fn read_i16(addr: usize) -> i16 {
    platform::read_u16(addr) as i16
}

/// Write a signed 16-bit field (`sth` in the original).
#[inline]
unsafe fn write_i16(addr: usize, value: i16) {
    platform::write_u16(addr, value as u16);
}

/// Write an IEEE-754 single as its raw big-endian bits.
#[inline]
unsafe fn write_f32(addr: usize, value: f32) {
    platform::write_u32(addr, value.to_bits());
}

/// Call a PowerPC function with four integer arguments (`r3..r6`).
///
/// # Safety
/// `addr` must be a function with this exact ABI.
unsafe fn call4(addr: usize, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32) -> u32);
    f(a, b, c, d)
}

/// Call a PowerPC function with five integer arguments (`r3..r7`).
///
/// # Safety
/// `addr` must be a function with this exact ABI.
unsafe fn call5(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32) -> u32);
    f(a, b, c, d, e)
}

/// Call a PowerPC function with six integer arguments (`r3..r8`).
///
/// # Safety
/// `addr` must be a function with this exact ABI.
unsafe fn call6(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f6: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32) -> u32);
    f(a, b, c, d, e, f6)
}

// ---------------------------------------------------------------------------
// 0x80041C00 - dokan spawn/init
// ---------------------------------------------------------------------------

/// Decodes a dokan pipe actor's spawn parameters, installs its initial
/// transforms and bounds, notifies the class vtable, and plays its spawn
/// sound at the projected screen position.
///
/// # Original
/// - Ghidra: `FUN_80041C00` at `0x80041C00` (308 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80041c00_FUN_80041c00.c`
///
/// # Arguments
/// - `actor`: object being initialised. Field `+0x04` carries the spawn
///   parameters; the actor must already have a vtable at `+0x60`.
///
/// # Returns
/// `1` (success).
///
/// # Side effects
/// Writes the variant and player fields, the initial transform/scale block at
/// `+0x318..+0x32C`, and three angles; calls virtual slot `+0xD4`; projects
/// `+0xAC` to screen space through `0x8006A4B0`; starts sound `0x70E` through
/// `0x80198C30` with the sound-system handle from `0x80429D60`.
///
/// # Safety
/// `actor` must be a live dokan object with a valid vtable and readable
/// `+0xAC` position. The call performs indirect calls and audio work.
///
/// # Confidence
/// medium - every constant and store is observed, but the meaning of the
/// transform fields is inferred from their use; the sound id `0x70E` is
/// observed, its sample is not identified.
///
/// # Angle encoding
/// Ghidra writes `angle - 0x8000` for mode 2; the instructions are `lha`,
/// `addis r3,r3,1`, `addi r0,r3,-0x8000`, `sth`, i.e. `angle + 0x8000` in the
/// 32-bit intermediate. Because the result is stored as 16 bits, `+0x8000`
/// and `-0x8000` are the same angle; the code below uses `+0x8000` to match
/// the register values. `0x54C` keeps the unmodified heading.
pub unsafe fn fn_80041c00(actor: usize) -> u32 {
    let params = platform::read_u32(actor + OFF_PARAMS);

    // observed 0x80041C1C/0x80041C24: mode = (params >> 4) & 3.
    let mode = (params >> 4) & 3;
    platform::write_u32(actor + OFF_MODE, mode);

    // observed 0x80041C20/0x80041C28: player = (params >> 12) & 3, stored as
    // a single byte at +0x38D.
    let player = (params >> 12) & 3;
    platform::write_u8(actor + OFF_PLAYER, player as u8);

    // observed 0x80041C2C: load the BRRES models (and their 32 B parts).
    fn_80041d40(actor);

    let kind = (params >> 16) & 0xFF;
    if kind < 2 && mode == 1 {
        // observed 0x80041C4C..0x80041C64:
        //   +0x554 = [0x8042B754] = 8.0f
        //   +0x0DC = [0x8042B758] = 2.0f
        //   +0x0E0 = [0x8042B75C] = 1.0f
        //   +0x0E4 = [0x8042B75C] = 1.0f
        write_f32(actor + 0x554, 8.0); // observed: 0x8042B754
        write_f32(actor + 0x0DC, 2.0); // observed: 0x8042B758
        write_f32(actor + 0x0E0, 1.0); // observed: 0x8042B75C
        write_f32(actor + 0x0E4, 1.0); // observed: 0x8042B75C
    } else {
        // observed 0x80041C6C..0x80041C80:
        //   +0x554 = [0x8042B760] = 16.0f; +0x0DC/+0x0E0/+0x0E4 = 1.0f.
        write_f32(actor + 0x554, 16.0); // observed: 0x8042B760
        write_f32(actor + 0x0DC, 1.0);  // observed: 0x8042B75C
        write_f32(actor + 0x0E0, 1.0);  // observed: 0x8042B75C
        write_f32(actor + 0x0E4, 1.0);  // observed: 0x8042B75C
    }

    // observed 0x80041C84..0x80041CB0: bounds/scale block.
    //   +0x318 = 128.0f [0x8042B76C]   +0x31C = 128.0f
    //   +0x320 =   0.0f [0x8042B764]   +0x324 =  48.0f [0x8042B768]
    //   +0x328 =  80.0f [0x8042BB08]   +0x32C = 256.0f [0x8042BB0C]
    write_f32(actor + 0x318, 128.0); // observed: 0x8042B76C
    write_f32(actor + 0x31C, 128.0); // observed: 0x8042B76C
    write_f32(actor + 0x320, 0.0);   // observed: 0x8042B764
    write_f32(actor + 0x324, 48.0);  // observed: 0x8042B768
    write_f32(actor + 0x328, 80.0);  // observed: 0x8042BB08
    write_f32(actor + 0x32C, 256.0); // observed: 0x8042BB0C

    // observed 0x80041CB4..0x80041CC0: virtual slot +0xD4 with `actor` in r3.
    let vtable = platform::read_u32(actor + OFF_VTABLE) as usize;
    let on_spawn = platform::read_u32(vtable + 0xD4) as usize;
    platform::call1(on_spawn, actor as u32);

    // observed 0x80041CC4/0x80041CCC: copy the heading to the first angle.
    let angle = read_i16(actor + OFF_ANGLE);
    write_i16(actor + 0x54C, angle);

    // observed 0x80041CD0..0x80041CEC: project the world position to screen
    // space and start sound 0x70E at that position.
    let mut screen = [0.0f32; 4]; // 16-byte stack slot in the original
    platform::call2(
        0x8006_A4B0,
        screen.as_mut_ptr() as u32,
        (actor + OFF_POS) as u32,
    );
    let sound_system = platform::read_u32(G_SOUND_SYSTEM);
    call4(
        0x8019_8C30,
        sound_system,
        0x70E, // observed: sound id; the sample name is not resolved
        screen.as_ptr() as u32,
        0,
    );

    // observed 0x80041CF0..0x80041D14: mode 2 mirrors the heading by half a
    // turn; all other modes copy it unchanged.
    if mode == 2 {
        // observed: `addis +1` then `addi -0x8000` = +0x8000 (see doc above).
        write_i16(actor + 0x54E, (angle as u16).wrapping_add(0x8000) as i16);
    } else {
        write_i16(actor + 0x54E, angle);
    }

    1
}

// ---------------------------------------------------------------------------
// 0x80041D40 - load the dokan BRRES and create its models
// ---------------------------------------------------------------------------

/// Loads `g3d/obj_dokan.brres`, creates the main pipe model, 32 `B` parts and
/// the `CD` cap model, then detaches the model-setup context.
///
/// # Original
/// - Ghidra: `FUN_80041D40` at `0x80041D40` (380 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80041d40_FUN_80041d40.c`
///
/// # Arguments
/// - `actor`: dokan object. Uses the spawn parameters at `+0x04`, the variant
///   at `+0x558`, and writes the model handles into `+0x3B0` onward.
///
/// # Returns
/// Nothing (`void`).
///
/// # Side effects
/// - `0x800690C0` prepares the model context (`0x80377C48`, flags `-1`/`0x20`).
/// - `0x800DF180` opens the BRRES (`obj_dokan` in `g3d/obj_dokan.brres`).
/// - `0x80239E30` resolves a named resource in that BRRES.
/// - `0x8016A3A0` creates each model, followed by `0x80064BF0` finalisation.
/// - `0x8016A8E0` closes/detaches the context.
///
/// # Safety
/// `actor` must be a valid dokan object; the archive-manager pointer at
/// `0x8042A038` must be live. The function performs many indirect calls.
///
/// # Confidence
/// medium - the strings, model counts and offsets are observed; the exact
/// meaning of the archive handle and `0x20` model flag is inferred.
///
/// # Table evidence
/// The name tables are absolute pointers in the image, so they are read from
/// their original addresses instead of being rebuilt as Rust arrays:
/// - `0x80306CFC`: `[obj_dokan_A, obj_dokan_CU, obj_dokan_A]`;
/// - `0x80306D18`: `[obj_dokan_CD, obj_dokan_CD, obj_dokan_A]`.
/// Both have three entries while `mode` is a two-bit field (0..3).
/// `// unknown:` mode 3 would read the first four bytes of the next string as
/// a pointer, exactly as the original does; no valid caller is known to pass 3.
pub unsafe fn fn_80041d40(actor: usize) {
    // observed 0x80041D48..0x80041D80: model context, heap flag `-1`,
    // context word from 0x80377C48, 32-byte alignment/flags.
    call5(
        0x8006_90C0,
        (actor + 0x394) as u32,
        0xFFFF_FFFF,
        platform::read_u32(G_MODEL_CONTEXT),
        0,
        0x20,
    );

    // observed 0x80041D84..0x80041D94:
    //   arg0 = *(*0x8042A038) + 4   (archive handle)
    //   arg1 = "obj_dokan"
    //   arg2 = "g3d/obj_dokan.brres"
    let archive = platform::read_u32(G_ARCHIVE_MANAGER).wrapping_add(4);
    let brres = platform::call3(
        0x800D_F180,
        archive,
        b"obj_dokan\0".as_ptr() as u32,
        b"g3d/obj_dokan.brres\0".as_ptr() as u32,
    );
    platform::write_u32(actor + 0x3B0, brres);

    let params = platform::read_u32(actor + OFF_PARAMS);
    let kind = (params >> 16) & 0xFF;
    let mode = platform::read_u32(actor + OFF_MODE) as usize;

    // observed 0x80041D98..0x80041DDC: choose "obj_dokan_D" when kind < 2 and
    // the variant is not 1, otherwise table[mode].
    let main_model = if kind < 2 && mode != 1 {
        b"obj_dokan_D\0".as_ptr()
    } else {
        platform::read_u32(0x8030_6CFC + mode * 4) as usize as *const u8
    };

    // observed 0x80041DB8/0x80041DC0: resolve the model and store it in the
    // temporary that 0x8016A3A0 consumes by pointer.
    let mut resolved = platform::call2(0x8023_9E30, (actor + 0x3B0) as u32, main_model as u32);

    // observed 0x80041DE0..0x80041DFC: create the main model at +0x3B4.
    call6(
        0x8016_A3A0,
        (actor + 0x3B4) as u32,
        &mut resolved as *mut u32 as u32,
        (actor + 0x394) as u32,
        0x20,
        1,
        0,
    );
    // observed 0x80041E00: finalise the model.
    platform::call1(0x8006_4BF0, (actor + 0x3B4) as u32);

    // observed 0x80041E08..0x80041E50: 32 parts, 0xC bytes apart, all using
    // "obj_dokan_B".
    let mut part = actor + 0x3C0;
    let mut index = 0u32;
    while index < 0x20 {
        let mut part_name = platform::call2(
            0x8023_9E30,
            (actor + 0x3B0) as u32,
            b"obj_dokan_B\0".as_ptr() as u32,
        );
        call6(
            0x8016_A3A0,
            part as u32,
            &mut part_name as *mut u32 as u32,
            (actor + 0x394) as u32,
            0x20,
            1,
            0,
        );
        platform::call1(0x8006_4BF0, part as u32);

        part += 0x0C;
        index += 1;
    }

    // observed 0x80041E54..0x80041E90: CD cap model from table[mode].
    let cd_model = platform::read_u32(0x8030_6D18 + mode * 4) as usize as *const u8;
    let mut cd_resolved =
        platform::call2(0x8023_9E30, (actor + 0x3B0) as u32, cd_model as u32);
    call6(
        0x8016_A3A0,
        (actor + 0x540) as u32,
        &mut cd_resolved as *mut u32 as u32,
        (actor + 0x394) as u32,
        0x20,
        1,
        0,
    );
    platform::call1(0x8006_4BF0, (actor + 0x540) as u32);

    // observed 0x80041E94/0x80041E98: close the model setup context.
    platform::call1(0x8016_A8E0, (actor + 0x394) as u32);
}

// ---------------------------------------------------------------------------
// 0x80042350 - per-frame dokan rotation
// ---------------------------------------------------------------------------

/// Rotates the dokan pipe by the signed 16-bit delta stored at `+0x550` and
/// lets the base class do its per-frame work.
///
/// # Original
/// - Ghidra: `FUN_80042350` at `0x80042350` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80042350_FUN_80042350.c`
///
/// # Arguments
/// - `actor`: live dokan object.
///
/// # Returns
/// Nothing (`void`).
///
/// # Side effects
/// - Virtual slot `+0xDC` runs the base-class execute step.
/// - `0x8006CD40(actor)` updates the actor's transform/child list.
/// - Adds the signed 16-bit delta at `+0x550` (`// inferred:` angular
///   velocity) to the three angles at `+0x104`, `+0x54C` and `+0x54E`,
///   wrapping modulo 65536.
///
/// # Safety
/// `actor` must be a valid object with the listed fields and vtable.
///
/// # Confidence
/// high for the three adds and the `lha`/`sth` sequence; medium for the
/// "rotation" role.
pub unsafe fn fn_80042350(actor: usize) {
    // observed 0x80042364..0x80042370: vtable[+0xDC](actor).
    let vtable = platform::read_u32(actor + OFF_VTABLE) as usize;
    let execute = platform::read_u32(vtable + 0xDC) as usize;
    platform::call1(execute, actor as u32);

    // observed 0x80042374/0x80042378: 0x8006CD40(actor).
    platform::call1(0x8006_CD40, actor as u32);

    // observed 0x8004237C..0x800423A0: four `lha`/`sth` pairs; all three
    // angles advance by the same signed 16-bit delta.
    let delta = read_i16(actor + 0x550);
    write_i16(actor + 0x104, read_i16(actor + 0x104).wrapping_add(delta));
    write_i16(actor + 0x54C, read_i16(actor + 0x54C).wrapping_add(delta));
    write_i16(actor + 0x54E, read_i16(actor + 0x54E).wrapping_add(delta));
}

// ---------------------------------------------------------------------------
// 0x80042970 - dokan delete hook
// ---------------------------------------------------------------------------

/// Runs the class destructor chain and unlinks the actor from the model/actor
/// list, then reports success.
///
/// # Original
/// - Ghidra: `FUN_80042970` at `0x80042970` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80042970_FUN_80042970.c`
///
/// # Arguments
/// - `actor`: object being destroyed.
///
/// # Returns
/// `1` (the caller treats this as "destroyed").
///
/// # Side effects
/// - Virtual slot `+0xE8` (`// inferred:` base/derived destructor step).
/// - `0x8007F950(actor + 0x3FC)` unlinks a list node (its own body reads
///   `prev`/`next` at `+4`/`+8`).
///
/// # Safety
/// `actor` must be live and still linked when the function starts; the list
/// unlink makes the node invalid.
///
/// # Confidence
/// medium - control flow is exact; the "delete hook" role comes from the
/// destructor slot and the list unlink.
pub unsafe fn fn_80042970(actor: usize) -> u32 {
    // observed 0x80042984..0x80042990: vtable[+0xE8](actor).
    let vtable = platform::read_u32(actor + OFF_VTABLE) as usize;
    let destroy = platform::read_u32(vtable + 0xE8) as usize;
    platform::call1(destroy, actor as u32);

    // observed 0x80042994/0x80042998: unlink actor + 0x3FC.
    platform::call1(0x8007_F950, (actor + 0x3FC) as u32);

    1
}

// ---------------------------------------------------------------------------
// 0x80042A70 - create hook
// ---------------------------------------------------------------------------

/// Forwards the create argument to `0x80162510` and reports `0`.
///
/// # Original
/// - Ghidra: `FUN_80042A70` at `0x80042A70` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80042a70_FUN_80042a70.c`
///
/// # Arguments
/// - `arg`: value already in `r3` at entry. Ghidra types the function
///   `void(void)` because the body never names this argument, but the
///   instruction at `0x80042A7C` calls `0x80162510` without touching `r3`, so
///   the value is forwarded unchanged.
///
/// # Returns
/// `0`, the usual "nothing spawned" result of a create callback.
///
/// # Side effects
/// `0x80162510(arg)` may mark an object started (`+0xB = 1`), call its virtual
/// slot `+0x38`, and recurse over the child list at `+0x14`.
///
/// # Safety
/// `arg` must be acceptable to `0x80162510` (it is treated as an object
/// pointer with a vtable at `+0x60`).
///
/// # Confidence
/// medium - the call and return value are exact; the role and the argument
/// type are inferred from the call shape.
pub unsafe fn fn_80042a70(arg: usize) -> u32 {
    // observed 0x80042A7C: `bl 0x80162510` with r3 untouched.
    platform::call1(0x8016_2510, arg as u32);
    0 // observed 0x80042A84: `li r3,0`
}

// ===========================================================================
// Wave 292 lane A (`_A292`): the five uncovered methods of the `obj_dokan`
// family base vtable `0x80306D60`
// ===========================================================================
//
// ## Shape and identity
//
// `0x80306D60`/`0x80306D64` are zero and `0x80306D68..0x80306E30` is a
// fifty-one-word method array. Because the installed pointer is the header, the
// class dispatches through `[[this+0x60] + 8 + 4*slot]`, so slot 50 lives at
// header `+0xD0`. The words at `0x80306D68` begin the same `obj_dokan` band
// documented at the top of this module; the data words immediately before the
// header spell `obj_dokan_A`, `obj_dokan_CU`, `obj_dokan_CD`,
// `g3d/obj_dokan.brres`, `obj_dokan`, `obj_dokan_D` and `obj_dokan_B`
// (`0x80306CE0..0x80306D5C`). The imported model-name tables used by
// [`fn_80041d40`] sit at `0x80306CFC` and `0x80306D18`, inside that block.
//
// The header is the *shared intermediate* table of the `obj_dokan`-family
// subclasses: four constructors in `d_basesNP` store it at `object + 0x60` and
// then overwrite the same field with a derived table:
//
// | constructor | object size | intermediate store | final table |
// |---|---|---|---|
// | `0x80687850` | `0x560` | `0x80687878..0x80687884` | `0x807EE1E8` |
// | `0x80687A50` | `0x560` | `0x80687A78..0x80687A84` | `0x807EE2D8` |
// | `0x80687D20` | `0x560` | `0x80687D48..0x80687D54` | `0x807EE3C8` |
// | `0x80687F30` | `0x560` | `0x80687F58..0x80687F64` | `0x807EE4B8` |
//
// Each store is `lis r4,0x8030` / `addi r4,r4,28000` (`0x6D60`) /
// `stw r4,96(r31)`. The four derived tables are contiguous in the `d_basesNP`
// data segment and extend the base array with four more slots (`0xCC..0xD8`);
// the nearby state-name blocks in the same data segment include
// `daFloorDokanEight_c`, `daFloorHoleDokan_c` and `daRotDokan_c`. The low band
// `0x80041C00..0x80042A70` is already translated in this module, including
// [`fn_80041c00`], [`fn_80041d40`] and [`fn_80042350`].
//
// ## Containment
//
// ```text
// 0x80306CE0  "obj_dokan_A"       ends 0x80306CEC
// 0x80306CEC  "obj_dokan_CU"      ends 0x80306CF8
// 0x80306CFC  [obj_dokan_A, obj_dokan_CU, obj_dokan_A]
// 0x80306D08  "obj_dokan_CD"      ends 0x80306D14
// 0x80306D18  [obj_dokan_CD, obj_dokan_CD, obj_dokan_A]
// 0x80306D24  "g3d/obj_dokan.brres" ends 0x80306D38
// 0x80306D38  "obj_dokan"         ends 0x80306D44
// 0x80306D44  "obj_dokan_D"       ends 0x80306D50
// 0x80306D50  "obj_dokan_B"       ends 0x80306D5C
// 0x80306D5C  0x00000000
// 0x80306D60  0x00000000 0x00000000   this header
// 0x80306D68  fifty-one method words ends 0x80306E34
// 0x80306E34  0x00000000
// 0x80306E38  [0x800423D0, 0x800423C0]  a separate two-word table
// 0x80306E40  "lift_ice_ashiba_3x3" ... the next class block
// ```
//
// All fifty-one targets are distinct main-DOL text addresses; none of the five
// bodies below lies inside a `functions.tsv` row or a translated extent
// (`inside=[]`). The nearest previous `functions.tsv` rows end:
//
// | target | previous row end | gap |
// |---|---|---|
// | `0x80041EC0` | `0x80041EBC` (`FUN_80041d40`) | `0x4` |
// | `0x80041F00` | `0x80041EBC` (`FUN_80041d40`) | `0x44` |
// | `0x800422A0` | `0x80041EBC` (`FUN_80041d40`) | `0x3E4` |
// | `0x800422B0` | `0x80041EBC` (`FUN_80041d40`) | `0x3F4` |
// | `0x800423E0` | `0x800423B8` (`FUN_80042350`) | `0x28` |
//
// ## Census
//
// | | |
// |---|---|
// | slots / distinct | 51 / 51 |
// | covered before this lane | 46 (9 `functions.tsv` rows, all ledgered: 8 `game::player`, 1 `game::actors::dokan`; the same 46 have `src` bodies, 37 of them body-only) |
// | uncovered | **5** (4 prologue-shaped + 1 leaf) |
// | prologue-shaped words in the array | 21 (`stwu` + `mflr`) |
// | prologue-shaped uncovered | 4 (`0x80041EC0`, `0x80041F00`, `0x800422B0`, `0x800423E0`) |
// | targets outside main-DOL text | 0 |
// | interior-label slots | 0 |
// | bodies below / bytes | 5 / 1300 |
//
// Forty-five of the fifty-one targets are shared with the ice-lift table
// `0x80306F78` handled by lane A's other half; the five uncovered targets here
// are **not** shared with that table. The derived tables `0x807EE1E8`,
// `0x807EE2D8`, `0x807EE3C8` and `0x807EE4B8` each extend the array to 55 slots;
// the base array stops at slot 50, which is why this census reports 51.
//
// ## Extent rule
//
// Extents were measured with `work/wave292/a_vt.mjs` and the raw bodies were
// read with `work/wave292/a_dis.mjs`. `bcctrl` and conditional `bc`/`bclr`
// fall through; an unconditional `bclr 20,0`, unconditional `bcctr`, `bctr` or
// a forward `b` out of the body ends a path. Every extent below closes on the
// returning `bclr`/`blr`.
//
// ## Scan-first
//
// None of the five addresses had an `fn_<addr>` body, a semantic body, a
// `functions.tsv` row or a ledger row before this lane, and no string in `src/`
// mentions them.

/// `0x8042B770` (`r2 - 0x7910`) - `0.00390625f` (`1/256`), the 16-bit angle
/// scale used by [`fn_80041f00`].
const A292_F32_ANGLE_SCALE_8042B770: usize = 0x8042_B770;

/// `0x8017A340` - element destructor handed to the array destructor
/// [`crate::runtime::cpp::fn_802dca98`] when [`fn_800423e0`] releases the
/// thirty-two `obj_dokan_B` part records at `+0x3C0`.
const A292_PART_DTOR_8017A340: usize = 0x8017_A340;

/// Read an IEEE-754 single (`lfs` in the original).
#[inline]
unsafe fn a292_read_f32(addr: usize) -> f32 {
    f32::from_bits(platform::read_u32(addr))
}

/// Calls an `f32 -> f32` routine with the value in `f1`.
///
/// # Safety
/// `addr` must be a function with this ABI.
#[inline]
unsafe fn a292_call_f32(addr: usize, value: f32) -> f32 {
    let f = crate::platform::guest_fn!(addr, fn(f32) -> f32);
    f(value)
}

/// Builds the position/angle packet for one model and runs the model update.
///
/// Reproduces the block shared by the three placements in [`fn_80041f00`]:
/// the three floats are written to a stack triple, the three halfword angles
/// (with `angle` as the third) to a stack triple, `0x80064AF0` converts the
/// packet, `0x801C0BD0` writes the translation matrix at `owner + 0x7C`, and
/// the six registry/model calls finish with virtual slot `+0x14`.
///
/// # Safety
/// `owner` must be a live `obj_dokan` actor with a `+0x7C` matrix and
/// `+0xDC` source vector; `model` must hold a vtable at `+0`.
#[inline]
unsafe fn a292_place_part(
    owner: usize,
    model: usize,
    cosine: f32,
    sine: f32,
    angle: u16,
) -> u32 {
    let x = a292_read_f32(owner + 0xAC);
    let y = a292_read_f32(owner + 0xB0);
    let z = a292_read_f32(owner + 0xB4);

    let mut position = [0.0f32; 3];
    position[0] = x + x * cosine;
    position[1] = y + x * sine;
    position[2] = z;

    let mut angles = [0u16; 3];
    angles[0] = platform::read_u16(owner + 0x100);
    angles[1] = platform::read_u16(owner + 0x102);
    angles[2] = angle;

    platform::call3(
        0x8006_4AF0,
        position.as_mut_ptr() as u32,
        angles.as_mut_ptr() as u32,
        1,
    );
    crate::runtime::dvd::fn_801c0bd0(position[0], position[1], position[2], owner + 0x7C);
    crate::resource::registry::fn_8016eef0(
        (owner + 0x7C) as u32,
        angles.as_mut_ptr().add(2) as usize,
    );
    crate::resource::registry::fn_8016a170(model, (owner + 0x7C) as u32);
    crate::resource::registry::fn_8016a150(model, owner + 0xDC);
    crate::resource::registry::fn_8016a1a0(model, 0);

    let vtable = platform::read_u32(model) as usize;
    let finish = platform::read_u32(vtable + 0x14) as usize;
    platform::call1(finish, model as u32)
}

// ---------------------------------------------------------------------------
// slot 6 - 0x80041EC0
// ---------------------------------------------------------------------------

/// Advances the class rotation, then runs the actor's collision update, and
/// reports success.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80041EC0`.
/// - Extent: `0x80041EC0..0x80041EFC` (60 bytes, 15 words); the next word is
///   zero, then [`fn_80041f00`] at `0x80041F00`.
/// - Table: slot 6 of the `0x80306D68` array (header `0x80306D60`; word
///   `0x80306D80`, header offset `+0x20`).
/// - Observed: `stwu r1,-16(r1)`; `mflr r0`; `stw r0,20(r1)`;
///   `stw r31,12(r1)`; `mr r31,r3`; `bl 0x80042350`; `mr r3,r31`;
///   `li r4,0`; `bl 0x80064F50`; `li r3,1`; epilogue; `bclr`.
///
/// # Arguments
/// - `actor`: live `obj_dokan` actor; [`fn_80042350`] reads `+0x550` and the
///   three angles, and `0x80064F50` reads `+0x36C` and the hit boxes.
///
/// # Returns
/// `1`, the successful-step convention of this table.
///
/// # Side effects
/// Runs one rotation step, then one collision/hit-box update.
///
/// # Safety
/// `actor` must be a live dokan object with a valid vtable at `+0x60`.
///
/// # Confidence
/// high - both callees and the `li r3,1` return are literal.
pub unsafe fn fn_80041ec0(actor: usize) -> u32 {
    fn_80042350(actor);
    crate::game::player::fn_80064f50(actor, 0);
    1
}

// ---------------------------------------------------------------------------
// slot 9 - 0x80041F00
// ---------------------------------------------------------------------------

/// Places the main pipe, the selected part chain and the `CD` cap every frame.
///
/// The body computes a sine/cosine pair from the 16-bit heading at `+0x104`
/// (`phase + 0x4000`, scaled by `1/256`), places the main model at `+0x3B4`,
/// and then repeats the placement for `(kind - 2)` part records starting at
/// `+0x3C0` (stride `0xC`) and for the cap model at `+0x540`. `kind` is
/// `(spawn_params >> 16) & 0xFF`. When `kind <= 1` and the variant word at
/// `+0x558` is not 1, only the main model is placed.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80041F00`.
/// - Extent: `0x80041F00..0x8004229C` (924 bytes, 231 words); one zero word
///   follows, then [`fn_800422a0`] at `0x800422A0`.
/// - Table: slot 9 of the `0x80306D68` array (word `0x80306D8C`, header offset
///   `+0x2C`).
/// - Observed: `stwu r1,-176(r1)`; `mflr r0`; six `stfd`/`psq_st` FPR saves
///   (`f26..f31`); `lha r4,260(r3)` + `addi 0x4000` + `sth` to the stack;
///   `psq_l`; `fmuls` by `[0x8042B770] = 1/256`; `bl 0x80237C40` (cosine) and
///   `bl 0x80237BD0` (sine); `lwz r3,4(r31)` + `rlwinm r3,16,24,31` (kind);
///   three `bl 0x80064AF0`/`0x801C0BD0` packet builds; `cmpwi r30,1` and
///   `cmpwi +0x558,1` guards; a `+0x3C0`/`0xC`/32-record loop; the `+0x540`
///   cap block; `li r3,1`; epilogue; `bclr`.
///
/// # Arguments
/// - `actor`: live `obj_dokan` family object. Reads `+0x04` (spawn params),
///   `+0x7C` (matrix), `+0xAC..+0xB4` (position), `+0xDC` (source vector),
///   `+0x100/+0x102/+0x104` (angles), `+0x318..+0x324` (bounds),
///   `+0x3B4` (main model), `+0x3C0` (32 part records), `+0x540` (cap model),
///   `+0x54C/+0x54E` (mirrored angles), `+0x554` (radius/offset) and `+0x558`
///   (variant).
///
/// # Returns
/// `1`, the successful-step convention of this table.
///
/// # Side effects
/// Writes the `+0x7C` matrix three or more times and runs each model's virtual
/// slot `+0x14`.
///
/// # Safety
/// Every model handle and child record above must be live; the registry
/// helpers dereference `+0x7C` and the model vtables.
///
/// # Confidence
/// medium - the control flow, offsets and callees are literal. The original
/// also computes `f29 = 16*x + [+0x554]` and `f28 = 0.5*x` and decrements
/// `f29` by 16 per part, but both values are overwritten before any use; they
/// are omitted here and recorded as dead arithmetic.
pub unsafe fn fn_80041f00(actor: usize) -> u32 {
    let phase = platform::read_u16(actor + 0x104).wrapping_add(0x4000);
    let argument = phase as f32 * a292_read_f32(A292_F32_ANGLE_SCALE_8042B770);
    let cosine = a292_call_f32(0x8023_7C40, argument);
    let sine = a292_call_f32(0x8023_7BD0, argument);

    let kind = (platform::read_u32(actor + OFF_PARAMS) >> 16) & 0xFF;

    a292_place_part(
        actor,
        actor + 0x3B4,
        cosine,
        sine,
        platform::read_u16(actor + 0x54C),
    );

    let run_chain = kind > 1 || platform::read_u32(actor + 0x558) == 1;
    if run_chain {
        let count = kind as i32 - 2;
        let mut index = 0i32;
        let mut part = actor + 0x3C0;
        while index < count {
            a292_place_part(actor, part, cosine, sine, platform::read_u16(actor + 0x104));
            part += 0x0C;
            index += 1;
        }
    }

    a292_place_part(
        actor,
        actor + 0x540,
        cosine,
        sine,
        platform::read_u16(actor + 0x54E),
    );

    1
}

// ---------------------------------------------------------------------------
// slot 12 - 0x800422A0
// ---------------------------------------------------------------------------

/// Empty virtual hook.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800422A0`.
/// - Extent: `0x800422A0..0x800422A4` (4 bytes, 1 word); three zero words
///   follow, then [`fn_800422b0`] at `0x800422B0`.
/// - Table: slot 12 of the `0x80306D68` array (word `0x80306D98`, header offset
///   `+0x38`).
/// - Observed: `4E800020` (`blr`).
///
/// # Returns
/// None; the incoming `r3` is returned untouched by the original.
///
/// # Safety
/// Always safe; no target memory is touched.
///
/// # Confidence
/// high - the word is exactly a `blr`.
pub unsafe fn fn_800422a0() {}

// ---------------------------------------------------------------------------
// slot 3 - 0x800422B0
// ---------------------------------------------------------------------------

/// Releases the thirty-two part models and the two cap models when the stored
/// stream word is stale.
///
/// The body reads the current stream singleton with
/// [`crate::resource::registry::fn_8016a5d0`] and compares it with `+0x3A8`.
/// Only when the two differ does it run virtual `+0x10` on every part record at
/// `+0x3C0` (32 records, stride `0xC`) and then on the main model at `+0x3B4`
/// and the cap model at `+0x540`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800422B0`.
/// - Extent: `0x800422B0..0x80042350` (160 bytes, 40 words); this extent ends
///   exactly at the `functions.tsv` row [`fn_80042350`].
/// - Table: slot 3 of the `0x80306D68` array (word `0x80306D74`, header offset
///   `+0x14`).
/// - Observed: `stwu r1,-32(r1)`; `mflr r0`; `stw r0,36(r1)`;
///   `stw r31/30/29`; `mr r29,r3`; `bl 0x8016A5D0`; `lwz r0,0x3A8(r29)`;
///   `cmplw cr0,r0,r3`; `bc 12,2` (skip the release when equal);
///   `addi r31,r29,0x3C0`; `li r30,0`; a `lwz r12,0(r31)` + `lwz r12,0x10(r12)`
///   + `bctrl` + `addi r31,0xC` + `cmpwi r30,0x20` loop; then the same virtual
///   call on `+0x3B4` and `+0x540`; `li r3,1`; epilogue; `bclr`.
///
/// # Arguments
/// - `actor`: live `obj_dokan` family object; `+0x3A8` stream word, `+0x3C0`
///   part records, `+0x3B4` main model, `+0x540` cap model.
///
/// # Returns
/// `1`, the successful-step convention of this table.
///
/// # Side effects
/// May release 34 model objects through their virtual slot `+0x10`.
///
/// # Safety
/// `actor` must be live with valid vtables on every part record and both
/// models; the registry singleton must be readable.
///
/// # Confidence
/// high - every offset, the equality test, the 32-iteration loop and both
/// trailing virtual calls are literal.
pub unsafe fn fn_800422b0(actor: usize) -> u32 {
    let stream = crate::resource::registry::fn_8016a5d0();
    if platform::read_u32(actor + 0x3A8) != stream {
        let mut part = actor + 0x3C0;
        let mut index = 0;
        while index < 32 {
            let vtable = platform::read_u32(part) as usize;
            let release = platform::read_u32(vtable + 0x10) as usize;
            platform::call1(release, part as u32);
            part += 0x0C;
            index += 1;
        }

        for model in [actor + 0x3B4, actor + 0x540] {
            let vtable = platform::read_u32(model) as usize;
            let release = platform::read_u32(vtable + 0x10) as usize;
            platform::call1(release, model as u32);
        }
    }
    1
}

// ---------------------------------------------------------------------------
// slot 16 - 0x800423E0
// ---------------------------------------------------------------------------

/// Deleting destructor: releases the cap model, the part array, the main model
/// and the model context, then runs the base release and optionally frees.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800423E0`.
/// - Extent: `0x800423E0..0x80042478` (152 bytes, 38 words); two zero words
///   follow, then [`crate::game::player::fn_80042480`] at `0x80042480`.
/// - Table: slot 16 of the `0x80306D68` array (word `0x80306DA8`, header offset
///   `+0x48`).
/// - Observed: `stwu r1,-16(r1)`; `mflr r0`; `cmpwi cr0,r3,0` (null returns
///   the null receiver); `stw r0,20(r1)`; `mr r31,r4`; `mr r30,r3`;
///   `addi r3,r3,0x540` + `li r4,-1` + `bl 0x8016A340`;
///   `addi r3,r30,0x3C0` + `lis r4,0x8017` + `addi r4,r4,-23744`
///   (`0x8017A340`) + `li r5,0xC` + `li r6,0x20` + `bl 0x802DCA98`;
///   `addi r3,r30,0x3B4` + `li r4,-1` + `bl 0x8016A340`;
///   `addi r3,r30,0x394` + `li r4,-1` + `bl 0x80069060`;
///   `mr r3,r30` + `li r4,0` + `bl 0x800642B0`;
///   `cmpwi cr0,r31,0` + `bc 4,1` (skip the free when `destroy <= 0`);
///   `mr r3,r30` + `bl 0x80162920`; epilogue; `bclr`.
///
/// # Arguments
/// - `actor`: object being destroyed; `+0x540` cap model, `+0x3C0` part array,
///   `+0x3B4` main model, `+0x394` model context.
/// - `destroy`: `> 0` requests the heap free after the release chain.
///
/// # Returns
/// The receiver, exactly as the original returns `r30`.
///
/// # Side effects
/// Releases four members and may free the object on the game heap.
///
/// # Safety
/// `actor` must be null or a live dokan object with the listed members.
///
/// # Confidence
/// high - every offset, count (`0x20`), stride (`0xC`) and callee is literal.
pub unsafe fn fn_800423e0(actor: usize, destroy: i32) -> usize {
    if actor != 0 {
        crate::resource::registry::fn_8016a340(actor + 0x540, -1);
        crate::runtime::cpp::fn_802dca98(actor + 0x3C0, A292_PART_DTOR_8017A340, 12, 32);
        crate::resource::registry::fn_8016a340(actor + 0x3B4, -1);
        crate::game::player::fn_80069060(actor + 0x394, -1);
        crate::game::player::fn_800642b0(actor, 0);
        if destroy > 0 {
            crate::resource::registry::fn_80162920(actor as u32);
        }
    }
    actor
}
