//! # The engine frame body - `0x800E4E40`
//!
//! ## What this module covers
//!
//! The 360-byte frame body [`fn_800e4e40`], plus the three one-word getters it
//! calls: [`fn_80107850`] (the main object's byte at `+4`), [`fn_80109340`]
//! (display-manager pointer) and [`fn_80107f10`] (layout-object pointer). The
//! getters are translated here for readers and for later modules; the frame
//! itself still calls their original addresses so its call sequence stays a
//! literal copy of `0x800E4E40`.
//!
//! Wave 20 adds three frame-support bodies from the same engine band: the
//! `0x800D08A0` renderer thunk [`fn_800d08a0`], the static heap registration
//! [`fn_800c9a10`], and the vertex-buffer builder [`fn_800e2ad0`]. They are the
//! pieces the frame and the scene draw code reach that were still untranslated
//! in `0x800C0000..0x80100000`.
//!
//! It is the busiest sequencing point in the engine: it reads
//! the main game object, drives the `dSys` step callbacks, updates audio, sets
//! up the 3-D view, draws and updates the scene, runs the input/controller
//! layer, and finally increments the global frame counter.
//!
//! ## Where it sits in the boot chain
//!
//! ```text
//! DOL entry 0x80004050
//!   -> 0x800C9F90 start_main_thread          (creates the game thread)
//!      -> 0x800C9F60 game_main               (thread entry)
//!         -> 0x800C9F10 game_startup         (0x800E48D0 dSys init once)
//!         -> 0x800C9F40 frame_loop           (do { ... } while (true))
//!            -> 0x800E4E40 one_game_frame    (this module)
//! ```
//!
//! `engine::startup` owns the hand-off; [`crate::engine::dsys`] owns the
//! one-time start-up and the `dSys` step helpers this frame calls.
//!
//! ## Observed frame order
//!
//! The order below is the exact order of the calls in
//! `work/ghidra/main-gb/export/800e4e40_FUN_800e4e40.c`. Group names are
//! `// inferred:` from the callee families; the addresses are observed.
//!
//! | # | Call | Group | What it touches |
//! |---:|---|---|---|
//! | 1 | `0x80107850(main)` | state | byte at `main + 4` (start-of-frame flag) |
//! | 2 | `0x80109340()` | state | display-manager pointer; mode field `+0x1D0` |
//! | 3 | `0x800E46E0` | dSys | begin-of-frame step: child `+8`, vtable `+0x24` |
//! | 4 | `0x80164D50` | audio | `0x802590F0(G_AUDIO_52B4)` |
//! | 5 | `0x800D0610` | render | 4-view 3-D setup; `0x80370D10` records |
//! | 6 | `0x80164EC0` | audio | `0x8025EFA0` plus an optional flush |
//! | 7 | `0x800E4660` | dSys | mid-frame step: child `+0xC` |
//! | 8 | `0x800B64E0` / `0x800B0B20` | conditional | only when mode != 2 and flag == 0 |
//! | 9 | `0x80109730(display)` | display | mode-2 scene reset path |
//! | 10 | `0x80108130(layout)` | layout | virtual slot `+0x1C` on `layout+4` |
//! | 11 | `0x80107460(main)` | render | draws the 3-D scene |
//! | 12 | `0x801097F0(display)` | display | post-draw effect |
//! | 13 | `0x800E4680` | dSys | sets three one-shot bytes, child `+0x10` |
//! | 14 | `0x80163170` / `0x80164E70` | conditional | only when mode != 2: clear `0x80377AB0`, drain the `0x52B4` queue |
//! | 15 | `0x801076B0(main)` | game | main-object update |
//! | 16 | `0x80107850(main)` | state | second flag read (edge detection) |
//! | 17 | `0x801BE2F0(1)` | state | only on flag 0 -> non-0 edge |
//! | 18 | `0x80109660(display)` | display | display state machine |
//! | 19 | `0x801080F0(layout)` | layout | `0x801084E0` + virtual slot `+0x18` |
//! | 20 | - (re-read `+0x1D0`) | state | second mode comparison |
//! | 21 | `0x800DC480` | game | four-slot (player) update |
//! | 22 | `0x80102140` | engine | per-slot callback (`engine::globals`) |
//! | 23 | `0x800B5AB0(G_5730)` | engine | four-entry loop over `0x800B5C60` |
//! | 24 | `0x8006A130` | input | controller/pad state snapshot |
//! | 25 | `0x8015F620(task_list, 0)` | conditional | task list; `2` = finished, then `0x800E1DE0` / `0x80162E10` / `0x800B0B00` |
//! | 26 | `0x800E4820` | dSys | thunk to `0x8006A320` (controller tick) |
//! | 27 | `0x8016F410` | render | virtual slot `+0xC` on the renderer |
//! | 28 | `0x800E4740` | dSys | end-of-frame step: child `+0x14`, vtable `+0x28` |
//! | 29 | counter `r13-0x5318 += 1` | state | global frame counter |
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - volatile big-endian loads/stores and indirect calls.
//! * [`crate::engine::dsys`] - the four `dSys` step helpers and the
//!   `0x800E4820` thunk are translated there, not duplicated here.
//! * Everything else is called by absolute address. None of those callees is
//!   translated in this module; their roles are `// inferred:` from the
//!   evidence and marked as such.
//!
//! ## Start here
//!
//! [`fn_800e4e40`] reads top to bottom in the same order as the original. Each
//! numbered step in the body matches the table above. The three getters are
//! the values it reads at the boundaries between subsystem steps.
//!
//! ## Wave-20 support bodies
//!
//! | Address | Rust function | Role | Confidence |
//! |---|---|---|---|
//! | `0x800D08A0` | [`fn_800d08a0`] | renderer vtable slot `+0x0C` (frame step 27) | high |
//! | `0x800C9A10` | [`fn_800c9a10`] | construct and register the static heap object | high |
//! | `0x800E2AD0` | [`fn_800e2ad0`] | build two 0x0C-stride vertex arrays | high |
//!
//! All three are `translated`; the only inferred parts are the `r3`
//! this-pointer of the renderer call and the role of the second vertex array.
//! Evidence and the full record are in `docs/COVERAGE_WAVE20_ENGINE.md`.
//!
//! ## Wave-26 support bodies
//!
//! Wave 26 adds the two scene-transition position wrappers that feed
//! `0x800E2220` / `0x800E2340`, plus five helpers of the 0x0C-stride vertex
//! array whose builder is [`fn_800e2ad0`]:
//!
//! | Address | Rust function | Role | Confidence |
//! |---|---|---|---|
//! | `0x800E22F0` | [`scene_transition_position_a`] | object-relative position into the first sink | high |
//! | `0x800E23C0` | [`scene_transition_position_b`] | the same into the second sink | high |
//! | `0x800E2C70` | [`vertex_finalize_indices`] | fill the second array and its position word | high |
//! | `0x800E2D00` | [`vertex_set`] | store one vertex triple | high |
//! | `0x800E2D30` | [`vertex_count`] | count the builder's elements | high |
//! | `0x800E2D60` | [`vertex_at`] | address one vertex | high |
//! | `0x800E2D70` | [`curve_weight_neg_square`] | `(c - v)^2 * scale` | high |
//! | `0x800E2D90` | [`curve_weight_product`] | `(c - v) * v + scale` | high |
//!
//! Evidence and the full record are in `docs/COVERAGE_WAVE26_ENGINE.md`.
//!
//! ## Glossary
//!
//! * **DOL** - the Wii executable container (`sys/main.dol`).
//! * **r13 / SDA** - the PowerPC small-data base register and the globals
//!   addressed through it. `0x800041C0` sets `r13 = 0x8042F680`, so
//!   `r13 - 0x5318` is the absolute address `0x8042A368`.
//! * **MEM1 / MEM2** - the Wii's 24 MiB main RAM and the extra 64 MiB.
//! * **GX** - the Wii GPU; **AX** - the Wii audio DSP.
//! * **`dSys`** - the engine's root object (`dSys_c`); see
//!   [`crate::engine::dsys`].
//!
//! ## Status
//!
//! [`fn_800e4e40`] is `translated` with `high` confidence for the call order
//! and the two counters. The *roles* of most callees are inferred; they are
//! documented as opaque addresses, never given invented symbol names. The
//! three getters are `translated` with `high` confidence.
//!
//! The wave-20 trio is also `translated` with `high` confidence; see the
//! per-function notes for the two inferred details.

use crate::engine::dsys;
use crate::platform;

/// Small-data base register `r13` for main-`DOL` code.
///
/// Observed: `0x800041C0` loads `0x8042F680` into `r13` (`lis`/`ori` at
/// `0x80004244`). Every `r13 - N` global below is resolved against it.
const SDA_BASE: usize = 0x8042_F680;

/// `r13 - 0x53E8` = `0x8042A298`: pointer to the **main game object**.
///
/// `0x801073B0` allocates it (0x1C4 bytes) and constructs it with the game
/// title string `"New SUPER MARIO BROS. Wii"` (at `0x80321C20`). The frame
/// reads its byte at `+4` through `0x80107850`.
///
/// `// inferred:` "main game object" from the title-string constructor.
const G_MAIN_OBJECT: usize = SDA_BASE - 0x53E8; // 0x8042A298

/// `r13 - 0x53CC` = `0x8042A2B4`: pointer to the **display manager**.
///
/// Created by `0x801092F0` (0x1F8-byte allocation). Its state word is at
/// `+0x1D0`; the frame compares it with `2` twice. `0x80109660` is its state
/// machine and `0x80109730`/`0x801097F0` are its draw steps.
///
/// `// inferred:` "display manager" from the state-machine callees.
const G_DISPLAY_MANAGER: usize = SDA_BASE - 0x53CC; // 0x8042A2B4

/// `r13 - 0x53D4` = `0x8042A2AC`: pointer to the **2-D layout object**.
///
/// Created by `0x80107EC0` (0x40-byte allocation). The frame calls virtual
/// slots on the child pointer stored at `layout + 4`.
///
/// `// inferred:` "layout object" from `0x80107F10`/`0x80108130`.
const G_LAYOUT_OBJECT: usize = SDA_BASE - 0x53D4; // 0x8042A2AC

/// `r13 - 0x5318` = `0x8042A368`: the **global frame counter**.
///
/// Incremented as the last statement of every frame. `0x8015FE40` clears it
/// together with [`G_SCENE_COUNTER`], so the pair is the frame/scene clock.
const G_FRAME_COUNTER: usize = SDA_BASE - 0x5318; // 0x8042A368

/// `r13 - 0x5314` = `0x8042A36C`: the **scene/transition counter**.
///
/// Incremented only when the task list at [`G_TASK_LIST`] reports `2`
/// (finished). Cleared with [`G_FRAME_COUNTER`] by `0x8015FE40`.
const G_SCENE_COUNTER: usize = SDA_BASE - 0x5314; // 0x8042A36C

/// `r13 - 0x55E0` = `0x8042A0A0`: header of the **engine task list**.
///
/// `0x8015F620` treats it as `{ table: *const fn(u32), count: u16, index: u16 }`
/// and returns `2` when the list is exhausted or empty. That return value is
/// the frame's scene-transition trigger.
const G_TASK_LIST: usize = SDA_BASE - 0x55E0; // 0x8042A0A0

/// `r13 - 0x5728` = `0x80429F58`: pointer slot passed to `0x800B64E0`.
///
/// `0x800B64E0` calls `*slot` when it is non-null, then `0x80163280`.
/// `// unknown:` what the slot holds.
const G_5728_SLOT: usize = SDA_BASE - 0x5728; // 0x80429F58

/// `r13 - 0x5730` = `0x80429F50`: base of a four-pointer array.
///
/// `0x800B5AB0` walks `base + i*4` for `i in 0..4` and calls `0x800B5C60` on
/// each word. `// unknown:` the object type.
const G_5730_ARRAY: usize = SDA_BASE - 0x5730; // 0x80429F50

/// `r13 - 0x5240` = `0x8042A440`: object whose virtual slot `+0x1C` is called
/// by `0x800B0B00` at the end of the task-list block.
///
/// `// unknown:` the object type.
const G_5240_OBJECT: usize = SDA_BASE - 0x5240; // 0x8042A440

/// `r13 - 0x55F4` = `0x8042A08C`: pointer to the `dSys_c` object.
///
/// Set by `0x800E48D0` to `0x80371AA0`; the `dSys` helpers in
/// [`crate::engine::dsys`] dereference it. Kept here for the module-level
/// documentation; the frame itself only reaches it through those helpers.
const G_D_SYS_PTR: usize = SDA_BASE - 0x55F4; // 0x8042A08C

// ---------------------------------------------------------------------------
// Callees outside this module.
//
// Each is called by absolute address; none is translated here. The role line
// is `// inferred:` unless the evidence proves it.
// ---------------------------------------------------------------------------

/// `0x80107850(obj)` - returns the byte at `obj + 4`.
///
/// Observed body: `return *(undefined1 *)(param_1 + 4);`. The frame uses it as
/// the main object's start-of-frame flag.
const FN_80107850: usize = 0x8010_7850;

/// `0x80109340()` - returns `*(u32*)(r13 - 0x53CC)`, the display manager.
///
/// Observed body is a single small-data load. Called four times per frame so
/// the pointer is re-read after each subsystem step.
const FN_80109340: usize = 0x8010_9340;

/// `0x80107F10()` - returns `*(u32*)(r13 - 0x53D4)`, the layout object.
///
/// Observed body is a single small-data load.
const FN_80107F10: usize = 0x8010_7F10;

/// `0x80164D50()` - audio step: calls `0x802590F0(*(u32*)(r13 - 0x52B4))`.
///
/// `// inferred:` audio/AX frame step from the `0x80164xxx` cluster.
const FN_80164D50: usize = 0x8016_4D50;

/// `0x800D0610()` - 3-D view setup for four players/views.
///
/// The body updates four 8-word records at `0x80370D10`, reads the four view
/// objects at `0x80377C88..0x80377C94`, and contains paired-single/GQR math.
/// `// inferred:` 3-D camera/view setup.
const FN_800D0610: usize = 0x800D_0610;

/// `0x80164EC0()` - audio step: `0x8025EFA0`, then `0x802D2E50` when either
/// `r13-0x52AC` or `r13-0x52A4` is non-zero.
///
/// `// inferred:` audio/AX frame step.
const FN_80164EC0: usize = 0x8016_4EC0;

/// `0x800B64E0(slot)` - calls `*slot` if non-null, then `0x80163280`.
///
/// `// unknown:` the slot's target.
const FN_800B64E0: usize = 0x800B_64E0;

/// `0x800B0B20()` - `0x8016DAA0()` then `0x80163280()`.
///
/// `// inferred:` resource/task service tick, paired with `0x800B64E0`.
const FN_800B0B20: usize = 0x800B_0B20;

/// `0x80109730(display)` - when `display+0x1D0 == 2`, runs `0x801092E0`,
/// `0x80109780(display)` and `0x80174FA0()`.
///
/// `// inferred:` display-manager mode-2 reset path.
const FN_80109730: usize = 0x8010_9730;

/// `0x80108130(layout)` - calls virtual slot `+0x1C` on `*(layout + 4)`.
///
/// `// inferred:` layout draw step.
const FN_80108130: usize = 0x8010_8130;

/// `0x80107460(main)` - large 3-D draw for the main game object.
///
/// The body builds view/projection records (`0x802BFF30`, `0x802BF3D0`,
/// `0x802BFF70(.., 0x280, 0x1C8)`) and calls `0x80106EE0`/`0x80106F80`/
/// `0x80106FA0`/`0x80107170`/`0x80107050`.
/// `// inferred:` scene draw.
const FN_80107460: usize = 0x8010_7460;

/// `0x801097F0(display)` - post-draw effect, gated on a float field at
/// `display+0x1E0` against an r2-relative constant.
///
/// `// inferred:` display post-draw effect.
const FN_801097F0: usize = 0x8010_97F0;

/// `0x80163170()` - calls `0x80228DD0(&0x80377AB0, 0)`.
///
/// `// unknown:` the service behind `0x80377AB0`.
const FN_80163170: usize = 0x8016_3170;

/// `0x80164E70()` - drains the queue at `*(r13 - 0x52B4)`:
/// while `obj[0x39] != 0`, calls virtual slot `+0x38` with `(obj, count-1)`.
///
/// `// inferred:` audio voice/command queue drain.
const FN_80164E70: usize = 0x8016_4E70;

/// `0x801076B0(main)` - main-object update.
///
/// Reads `0x801CEF60()` into `main+0x1B0`, manages the `+4` flag, and calls
/// `0x80107F10`/`0x80107850` internally.
/// `// inferred:` per-frame game-object update.
const FN_801076B0: usize = 0x8010_76B0;

/// `0x801BE2F0(1)` - stores `1` in `*(u32*)(r13 - 0x4E20)` after consulting
/// `0x801DD3F0()`; returns the previous value.
///
/// `// unknown:` the state flag's meaning.
const FN_801BE2F0: usize = 0x801B_E2F0;

/// `0x80109660(display)` - display-manager state machine.
///
/// Handles states `1`, `2` and `3` and always ends with `0x80109EC0(display)`.
/// `// inferred:` display state machine.
const FN_80109660: usize = 0x8010_9660;

/// `0x801080F0(layout)` - `0x801084E0()` then virtual slot `+0x18` on
/// `*(layout + 4)`.
///
/// `// inferred:` layout update step.
const FN_801080F0: usize = 0x8010_80F0;

/// `0x800DC480()` - four-slot update.
///
/// Walks the four pointers at `*(r13 - 0x5658)`, calls `0x800DC970` on each
/// `+4`, and sets a flag through `0x801DFBD0(0/1)`.
/// `// inferred:` four-player update.
const FN_800DC480: usize = 0x800D_C480;

/// `0x80102140()` - for each non-null slot at `0x803741D0`, calls `0x800DDC50`.
///
/// Already translated as [`crate::engine::globals::fn_80102140`]; the frame
/// calls the address to keep this module a pure sequencer.
const FN_80102140: usize = 0x8010_2140;

/// `0x800B5AB0(array)` - calls `0x800B5C60(*(array + i*4))` for `i in 0..4`.
///
/// `// unknown:` the four objects.
const FN_800B5AB0: usize = 0x800B_5AB0;

/// `0x8006A130()` - controller/pad snapshot.
///
/// Reads `*(r13 - 0x51F8)` through `0x80196690`, `0x80196820`, `0x80196830`
/// and `0x801969B0`, then overrides three SDA bytes from the object at
/// `0x803741D0` when `0x801021B0()` is non-zero.
/// `// inferred:` input/pad state snapshot.
const FN_8006A130: usize = 0x8006_A130;

/// `0x8015F620(header, arg)` - runs a task table until one task returns
/// something other than `1`, or the list is exhausted.
///
/// Returns `2` when the list is exhausted/empty. Translated as
/// [`crate::subsystems::r80140_80180::run_task_list`]; the frame calls the
/// address for the same reason as `0x80102140`.
const FN_8015F620: usize = 0x8015_F620;

/// `0x800E1DE0()` - scene-transition trigger.
///
/// Gated on a byte and a `0x2EE` sentinel; calls `0x80162B20` and advances a
/// three-word history.
/// `// inferred:` scene change request.
const FN_800E1DE0: usize = 0x800E_1DE0;

/// `0x80162E10()` - resets up to five input/rumble channels through
/// `0x80162BB0`/`0x80163000` when their enable bits are clear.
///
/// `// inferred:` input/rumble reset after a scene change.
const FN_80162E10: usize = 0x8016_2E10;

/// `0x800B0B00()` - calls virtual slot `+0x1C` on `*(r13 - 0x5240)`.
///
/// `// unknown:` the object and slot.
const FN_800B0B00: usize = 0x800B_0B00;

/// `0x8016F410()` - calls virtual slot `+0xC` on
/// `*(*(r13 - 0x5220) + 0x10)`.
///
/// The frame reaches it through the 4-byte thunk at `0x800D08A0`; calling the
/// final target is equivalent.
/// `// inferred:` renderer end-of-frame step.
const FN_8016F410: usize = 0x8016_F410;

// ---------------------------------------------------------------------------
// Local helpers
// ---------------------------------------------------------------------------

/// Calls a PowerPC function with no arguments and keeps its `r3` result.
///
/// `crate::platform::call0` discards the result, but several getters in this
/// frame return a pointer (`0x80109340`, `0x80107F10`). This is the same
/// signature helper pattern used by `engine::startup`.
///
/// # Safety
/// `addr` must point at a PowerPC function returning `u32` with the Metrowerks
/// EABI, and the caller must be running on a PowerPC target.
unsafe fn call0_ret(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Increments a 32-bit counter stored at `addr`.
///
/// The original is `*(int *)addr = *(int *)addr + 1`; wrapping is the faithful
/// 32-bit PowerPC behaviour.
///
/// # Safety
/// `addr..addr+4` must be writable.
unsafe fn bump_u32(addr: usize) {
    platform::write_u32(addr, platform::read_u32(addr).wrapping_add(1));
}

// ---------------------------------------------------------------------------
// 0x800E4E40
// ---------------------------------------------------------------------------

/// Runs one game frame.
///
/// In plain English: read the main game object's start-of-frame flag and the
/// display mode, run the `dSys` begin step, tick audio, set up and draw the
/// 3-D scene, update the game object, service input, and bump the frame
/// counter. Two blocks are conditional on the display mode not being `2`; one
/// of them also requires the main object's flag to be clear.
///
/// # Original
/// - Ghidra: `FUN_800e4e40` at `0x800E4E40` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e4e40_FUN_800e4e40.c`
/// - Caller: `0x800C9F40` (`frame_loop`, in `engine::startup`)
///
/// # Arguments
/// None. All inputs come from SDA globals and the objects they point at.
///
/// # Returns
/// Nothing. Every call is a statement except the getter results cached below.
///
/// # Side effects
/// * reads and writes the globals listed on the constants above;
/// * runs the entire engine schedule (render, audio, input, actor updates);
/// * increments [`G_FRAME_COUNTER`] as its last statement, and
///   [`G_SCENE_COUNTER`] when the task list finishes;
/// * the display-mode comparisons re-read `+0x1D0` after the display state
///   machine may have changed it.
///
/// # Safety
/// Must run on the game thread after `0x800E48D0` has created the heaps and
/// the main/display/layout objects. All pointers read here must be valid; the
/// original performs no null checks on them.
///
/// # Confidence
/// high - all 360 bytes are accounted for and the call order is literal. The
/// roles of the opaque callees are `// inferred:` and are marked at each
/// constant.
pub unsafe fn fn_800e4e40() {
    // Step 1 - 0x800E4E40: the main object's byte at +4, through 0x80107850.
    let flag_before = platform::call1(FN_80107850, platform::read_u32(G_MAIN_OBJECT)) & 0xFF;

    // Step 2 - 0x800E4E48: display manager and its mode field at +0x1D0.
    // observed: the original computes `clz(mode - 2) >> 5` and tests it for
    // zero; that expression is 1 only when `mode == 2`, so the test is
    // `mode != 2`.
    let display = call0_ret(FN_80109340);
    let mode_before_is_not_2 = platform::read_i32(display as usize + 0x1D0) != 2;

    // Step 3 - 0x800E4E58: dSys begin-of-frame step.
    dsys::fn_800e46e0();

    // Step 4 - 0x800E4E60: audio step.
    platform::call0(FN_80164D50);

    // Step 5 - 0x800E4E68: 3-D view setup for the four views.
    platform::call0(FN_800D0610);

    // Step 6 - 0x800E4E70: audio step.
    platform::call0(FN_80164EC0);

    // Step 7 - 0x800E4E78: dSys mid-frame step.
    dsys::fn_800e4660();

    // Step 8 - 0x800E4E80: only outside display mode 2 and with the main flag
    // clear. 0x800B64E0 takes a pointer to a function-pointer slot.
    if mode_before_is_not_2 && flag_before == 0 {
        platform::call1(FN_800B64E0, platform::read_u32(G_5728_SLOT));
        platform::call0(FN_800B0B20);
    }

    // Step 9 - 0x800E4EA4: refresh the display pointer and run its mode-2
    // reset path. The original reloads r3 through 0x80109340 before the call;
    // 0x80109730's body reads `display + 0x1D0` again.
    let display = call0_ret(FN_80109340);
    platform::call1(FN_80109730, display);

    // Step 10 - 0x800E4EB4: refresh the layout object and call its draw slot.
    let layout = call0_ret(FN_80107F10);
    platform::call1(FN_80108130, layout);

    // Step 11 - 0x800E4EC0: draw the 3-D scene of the main object. The
    // original re-reads the main-object global here.
    platform::call1(FN_80107460, platform::read_u32(G_MAIN_OBJECT));

    // Step 12 - 0x800E4ECC: refresh the display pointer and run the post-draw
    // effect.
    let display = call0_ret(FN_80109340);
    platform::call1(FN_801097F0, display);

    // Step 13 - 0x800E4ED8: dSys flag step.
    dsys::fn_800e4680();

    // Step 14 - 0x800E4EE0: outside display mode 2, clear the 0x80377AB0
    // struct and drain the queue at r13-0x52B4.
    if mode_before_is_not_2 {
        platform::call0(FN_80163170);
        platform::call0(FN_80164E70);
    }

    // Step 15 - 0x800E4EF0: main-object update. The original re-reads the
    // global.
    platform::call1(FN_801076B0, platform::read_u32(G_MAIN_OBJECT));

    // Step 16 - 0x800E4EFC: second flag read. The original re-reads the
    // main-object pointer and keeps the first byte in a callee-saved register,
    // so this is an edge detector, not a plain state test.
    let flag_after = platform::call1(FN_80107850, platform::read_u32(G_MAIN_OBJECT)) & 0xFF;

    // Step 17 - 0x800E4F04: rising edge 0 -> non-0.
    if flag_after != 0 && flag_before == 0 {
        platform::call1(FN_801BE2F0, 1);
    }

    // Step 18 - 0x800E4F1C: refresh the display pointer and run the state
    // machine. It may change `+0x1D0`.
    let display = call0_ret(FN_80109340);
    platform::call1(FN_80109660, display);

    // Step 19 - 0x800E4F2C: refresh the layout object and run its update slot.
    let layout = call0_ret(FN_80107F10);
    platform::call1(FN_801080F0, layout);

    // Step 20 - 0x800E4F3C: re-read the display pointer and the mode. This is
    // a second, independent comparison: 0x80109660 above may have changed the
    // state from 1 to 3, so the value is not the same as in step 2.
    let display = call0_ret(FN_80109340);
    let mode_after_is_not_2 = platform::read_i32(display as usize + 0x1D0) != 2;

    // Step 21 - 0x800E4F50: four-slot (player) update.
    platform::call0(FN_800DC480);

    // Step 22 - 0x800E4F58: per-slot callback from engine::globals.
    platform::call0(FN_80102140);

    // Step 23 - 0x800E4F60: four-entry object loop.
    platform::call1(FN_800B5AB0, platform::read_u32(G_5730_ARRAY));

    // Step 24 - 0x800E4F68: controller/pad snapshot.
    platform::call0(FN_8006A130);

    // Step 25 - 0x800E4F70: task-list block. `2` means "list finished", which
    // triggers the scene transition and bumps the scene counter. 0x800B0B00
    // runs in both cases.
    if mode_after_is_not_2 && flag_after == 0 {
        let result = platform::call2(FN_8015F620, G_TASK_LIST as u32, 0) as i32;
        if result == 2 {
            platform::call0(FN_800E1DE0);
            platform::call0(FN_80162E10);
            bump_u32(G_SCENE_COUNTER);
        }
        platform::call0(FN_800B0B00);
    }

    // Step 26 - 0x800E4FB4: controller tick. The original reaches
    // 0x8006A320 through the 4-byte thunk at 0x800E4820.
    dsys::fn_800e4820();

    // Step 27 - 0x800E4FBC: renderer end-of-frame step (via the thunk at
    // 0x800D08A0 in the original).
    platform::call0(FN_8016F410);

    // Step 28 - 0x800E4FC4: dSys end-of-frame step.
    dsys::fn_800e4740();

    // Step 29 - 0x800E4FCC: the global frame counter, always last.
    bump_u32(G_FRAME_COUNTER);
}

// ---------------------------------------------------------------------------
// 0x80107850 / 0x80109340 / 0x80107F10 - the frame's three getters
// ---------------------------------------------------------------------------

/// Returns the main game object's start-of-frame flag byte.
///
/// This is the whole body of `0x80107850`: `return *(u8*)(object + 4);`. The
/// frame reads it before and after `0x801076B0` to detect a `0 -> non-0` edge.
///
/// # Original
/// - Ghidra: `FUN_80107850` at `0x80107850` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80107850_FUN_80107850.c`
///
/// # Arguments
/// - `object`: main game object pointer from `0x8042A298`.
///
/// # Returns
/// The byte at `object + 4`.
///
/// # Side effects
/// None; one byte read.
///
/// # Safety
/// `object..object+5` must be readable. The original performs no null check.
///
/// # Confidence
/// high - the body is a single load.
pub unsafe fn fn_80107850(object: u32) -> u8 {
    platform::read_u8(object as usize + 4)
}

/// Returns the display-manager pointer (`r13 - 0x53CC`).
///
/// The original is a single small-data load; the frame re-reads it after each
/// subsystem step because those steps may replace or clear the manager.
///
/// # Original
/// - Ghidra: `FUN_80109340` at `0x80109340` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80109340_FUN_80109340.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `*(u32*)0x8042A2B4` ([`G_DISPLAY_MANAGER`]).
///
/// # Side effects
/// None.
///
/// # Safety
/// The small-data slot must be mapped.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80109340() -> u32 {
    platform::read_u32(G_DISPLAY_MANAGER)
}

/// Returns the 2-D layout-object pointer (`r13 - 0x53D4`).
///
/// The original is a single small-data load; the frame re-reads it before the
/// layout draw and update steps.
///
/// # Original
/// - Ghidra: `FUN_80107F10` at `0x80107F10` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80107f10_FUN_80107f10.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `*(u32*)0x8042A2AC` ([`G_LAYOUT_OBJECT`]).
///
/// # Side effects
/// None.
///
/// # Safety
/// The small-data slot must be mapped.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_80107f10() -> u32 {
    platform::read_u32(G_LAYOUT_OBJECT)
}

// ===========================================================================
// Wave 20 - frame support in 0x800C9A10 / 0x800D08A0 / 0x800E2AD0
// ===========================================================================
//
// Three bodies that sit between the frame and the subsystems it drives:
//
// * `0x800D08A0` is the four-byte thunk the frame reaches at step 27; it is
//   the renderer's end-of-frame virtual call.
// * `0x800C9A10` registers the static heap object that `engine::dsys` builds in
//   `0x800E5400` and destroys in `0x800E5480`.
// * `0x800E2AD0` builds the two 0x0C-stride vertex arrays used by the scene
//   draw code.
//
// Their evidence is listed in `docs/COVERAGE_WAVE20_ENGINE.md`.

/// `r13 - 0x5220` -> `0x8042A460`: pointer to the renderer/display holder.
///
/// `+0x10` is the object whose vtable slot `+0x0C` ends the frame.
const G_RENDERER_5220: usize = SDA_BASE - 0x5220;

/// `0x80359470`: static storage of the heap/allocator object constructed by
/// `0x800E5400` and destroyed by `0x800E5480`.
const W_STATIC_HEAP_OBJECT: usize = 0x8035_9470;

/// `0x80359460`: link record filled by `0x802DC780` with the destructor and
/// the heap object.
const W_STATIC_HEAP_LINK: usize = 0x8035_9460;

/// `0x8031B010`: vtable installed at `object + 0` by [`fn_800e2ad0`].
const VTABLE_8031B010: u32 = 0x8031_B010;

/// `0x80015E30`: element type descriptor passed to `0x802DC7E0`.
const DESC_80015E30: usize = 0x8001_5E30;

/// `0x8000FBF0`: element constructor passed to `0x802DC7E0`.
const FN_8000FBF0: usize = 0x8000_FBF0;

/// `0x802B9250(size)`: aligned allocation on the current heap (returns `r3`).
const FN_802B9250: usize = 0x802B_9250;

/// `0x802DC7E0(buffer, descriptor, construct, size, count)`: array constructor.
const FN_802DC7E0: usize = 0x802D_C7E0;

/// `0x802DC780(object, destructor, link)`: registers a static destructor.
const FN_802DC780: usize = 0x802D_C780;

/// `0x800E5400(object)`: constructs the static heap object.
const FN_800E5400: usize = 0x800E_5400;

/// `0x800E5480(object, free)`: destroys the static heap object.
const FN_800E5480: usize = 0x800E_5480;

/// `0x80377DC8..0x80377DD0`: three words copied into every vertex by
/// [`fn_800e2ad0`]. `// unknown:` a constant vector, read from memory instead
/// of assumed to be zero.
const W_80377DC8: usize = 0x8037_7DC8;
const W_80377DCC: usize = 0x8037_7DCC;
const W_80377DD0: usize = 0x8037_7DD0;

/// Renderer end-of-frame step (the frame's `0x800D08A0` thunk).
///
/// In plain English: fetch the renderer holder, then call virtual slot `+0x0C`
/// on the object at `holder + 0x10`. The frame previously called the final
/// target `0x8016F410` directly; this body is the thunk itself.
///
/// # Original
/// - Ghidra: `thunk_FUN_8016f410` at `0x800D08A0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800d08a0_thunk_FUN_8016f410.c`
/// - Caller: [`fn_800e4e40`] step 27 (`0x800E4FBC`)
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call into the renderer.
///
/// # Safety
/// The renderer holder and its `+0x10` object must be live, and the vtable slot
/// must be a function pointer.
///
/// # Confidence
/// high - the two loads and the call are literal. `// inferred:` the object is
/// passed in `r3` (the register still holds it at the call site), matching the
/// vtable-call convention.
pub unsafe fn fn_800d08a0() {
    let holder = platform::read_u32(G_RENDERER_5220) as usize;
    let target = platform::read_u32(holder + 0x10) as usize;
    let entry = platform::read_u32(target + 0x0C) as usize;
    let _ = platform::call1(entry, target as u32);
}

/// Registers the static heap object with its destructor.
///
/// In plain English: construct the object in its static storage, then push it
/// onto the shutdown list with `0x800E5480` as its destructor.
///
/// # Original
/// - Ghidra: `FUN_800c9a10` at `0x800C9A10` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800c9a10_FUN_800c9a10.c`
///
/// # Arguments
/// None; the storage addresses are globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Constructs the 0x18-entry heap object at `0x80359470` and links it into the
/// static destructor list at `0x80359460`.
///
/// # Safety
/// Must run during start-up before any allocator use.
///
/// # Confidence
/// high - the constructor, destructor and link addresses are literal. The two
/// callee bodies live in [`crate::engine::dsys`] and are called by address.
pub unsafe fn fn_800c9a10() {
    let _ = platform::call1(FN_800E5400, W_STATIC_HEAP_OBJECT as u32);
    let _ = platform::call3(
        FN_802DC780,
        W_STATIC_HEAP_OBJECT as u32,
        FN_800E5480 as u32,
        W_STATIC_HEAP_LINK as u32,
    );
}

/// Builds a two-array vertex buffer.
///
/// In plain English: store the vertex count, the two vtable/tag bytes and the
/// class vtable; allocate the first array with `count` elements of `0x0C`
/// bytes and the second with `count * byte_6 + 1` elements; fill every element
/// of both arrays with the constant vector at `0x80377DC8..0x80377DD0`.
///
/// # Original
/// - Ghidra: `FUN_800e2ad0` at `0x800E2AD0` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2ad0_FUN_800e2ad0.c`
///
/// # Arguments
/// * `object` - storage for the header (0x10 bytes).
/// * `count` - vertex count, stored at `+4` as a halfword.
/// * `byte_6` - byte at `+6`; multiplies `count` for the second array size.
/// * `byte_7` - byte at `+7`, stored verbatim.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Two heap allocations and their element constructors, plus the two fill
/// loops.
///
/// # Safety
/// The heap must be live; `object` must be writable; the returned arrays must
/// stay reachable through `+8` / `+0xC`.
///
/// # Confidence
/// high - every size, loop bound and store is explicit. `// inferred:` the
/// second array is the strip/fan index buffer because its size is
/// `count * byte_6 + 1`.
pub unsafe fn fn_800e2ad0(object: usize, count: u16, byte_6: u8, byte_7: u8) -> usize {
    // observed: the halfword at +4 is written before the vtable word.
    platform::write_u16(object + 4, count);
    platform::write_u32(object, VTABLE_8031B010);
    platform::write_u8(object + 6, byte_6);
    platform::write_u8(object + 7, byte_7);

    let bytes_a = count as u32 * 0x0C + 0x10;
    let raw_a = platform::call1(FN_802B9250, bytes_a);
    let array_a = platform::call5(
        FN_802DC7E0,
        raw_a,
        DESC_80015E30 as u32,
        FN_8000FBF0 as u32,
        0x0C,
        count as u32,
    );
    platform::write_u32(object + 0x08, array_a);

    let count_b = count as u32 * byte_6 as u32 + 1;
    let raw_b = platform::call1(FN_802B9250, count_b * 0x0C + 0x10);
    let array_b = platform::call5(
        FN_802DC7E0,
        raw_b,
        DESC_80015E30 as u32,
        FN_8000FBF0 as u32,
        0x0C,
        count_b,
    );
    platform::write_u32(object + 0x0C, array_b);

    // First array: one constant vector per vertex.
    let mut offset = 0usize;
    let mut index = 0u32;
    while index < platform::read_u16(object + 4) as u32 {
        let base = platform::read_u32(object + 0x08) as usize + offset;
        platform::write_u32(base, platform::read_u32(W_80377DC8));
        platform::write_u32(base + 4, platform::read_u32(W_80377DCC));
        platform::write_u32(base + 8, platform::read_u32(W_80377DD0));
        offset += 0x0C;
        index += 1;
    }

    // Second array: the C re-reads both factors on every iteration.
    let mut offset = 0usize;
    let mut index = 0u32;
    while index < platform::read_u16(object + 4) as u32 * platform::read_u8(object + 6) as u32 + 1 {
        let base = platform::read_u32(object + 0x0C) as usize + offset;
        platform::write_u32(base, platform::read_u32(W_80377DC8));
        platform::write_u32(base + 4, platform::read_u32(W_80377DCC));
        platform::write_u32(base + 8, platform::read_u32(W_80377DD0));
        offset += 0x0C;
        index += 1;
    }

    object
}

// ===========================================================================
// Wave 26 - scene-transition positions and the vertex-array class
// (0x800E22F0, 0x800E23C0, 0x800E2C70..0x800E2D90)
// ===========================================================================
//
// Two small wrappers convert a 3-D position into the stack vector the scene
// transition expects (`0x800E2220` / `0x800E2340`), and five bodies finish the
// 0x0C-stride vertex-array class whose builder is [`fn_800e2ad0`]:
//
// | address | body | role |
// |---|---|---|
// | `0x800E22F0` | [`scene_transition_position_a`] | add the object position to a 2-D point |
// | `0x800E23C0` | [`scene_transition_position_b`] | the same wrapper for the second sink |
// | `0x800E2C70` | [`vertex_finalize_indices`] | compute three vertex positions and fill the tail |
// | `0x800E2D00` | [`vertex_set`] | store one vertex |
// | `0x800E2D30` | [`vertex_count`] | count the two arrays |
// | `0x800E2D60` | [`vertex_at`] | address one vertex |
// | `0x800E2D70` | [`curve_weight_neg_square`] | `(c - v)^2 * scale` |
// | `0x800E2D90` | [`curve_weight_product`] | `(c - v) * v + scale` |
//
// The `0x802DCD70`-family calls printed around these bodies are CodeWarrior
// save/restore helpers and are omitted, following the rest of this file.

/// Reads the big-endian word at `base + offset`.
#[inline]
unsafe fn read_word(base: usize, offset: usize) -> u32 {
    platform::read_u32(base + offset)
}

/// Writes the big-endian word `value` at `base + offset`.
#[inline]
unsafe fn write_word(base: usize, offset: usize, value: u32) {
    platform::write_u32(base + offset, value);
}

/// Reads an `f32` at `base + offset`.
#[inline]
unsafe fn read_float(base: usize, offset: usize) -> f32 {
    platform::read_f32(base + offset)
}

/// Reads the big-endian word at an absolute global address.
#[inline]
unsafe fn read_global_word(addr: usize) -> u32 {
    platform::read_u32(addr)
}

/// Reads the `f32` stored at an absolute SDA2 address.
#[inline]
unsafe fn read_const_f32(addr: usize) -> f32 {
    platform::read_f32(addr)
}

/// PowerPC `r2` small-data-2 base, set at `0x8000423C`.
///
/// The transition and curve helpers read their float constants through it.
const SDA2_BASE_26: usize = 0x8043_3080;

/// `r2 - 0x63F0` -> `0x8042CC90`: third component stored in the transition
/// vector. `// unknown:` the value is read from memory, not decoded.
const F32_SDA2_63F0: usize = SDA2_BASE_26 - 0x63F0;

/// `r2 - 0x63C8` -> `0x8042CCB8`: scale passed to `0x800E2DE0` by
/// [`vertex_finalize_indices`].
const F32_SDA2_63C8: usize = SDA2_BASE_26 - 0x63C8;

/// `r2 - 0x63C4` -> `0x8042CCBC`: centre of the two curve weights.
const F32_SDA2_63C4: usize = SDA2_BASE_26 - 0x63C4;

/// `r2 - 0x63C0` -> `0x8042CCC0`: scale of the two curve weights.
const F32_SDA2_63C0: usize = SDA2_BASE_26 - 0x63C0;

/// `0x800E2220(vector_out, position)`: first transition position sink.
const FN_800E2220: usize = 0x800E_2220;

/// `0x800E2340(vector_out, position)`: second transition position sink.
const FN_800E2340: usize = 0x800E_2340;

/// `0x800E2DE0(scale, src, src_end, dst, 0, 8)`: fills one vertex array.
const FN_800E2DE0: usize = 0x800E_2DE0;

/// Calls `func(f64, u32, u32, u32, u32, u32) -> u32`.
///
/// Used only for `0x800E2DE0`, whose first parameter is printed as a `double`
/// cast of a float constant. The reconstruction is semantic, not a bit-exact
/// ABI replay.
#[inline]
unsafe fn call_f64_u32x5(
    addr: usize,
    a: f64,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
) -> u32 {
    let function = crate::platform::guest_fn!(addr, fn(f64, u32, u32, u32, u32, u32) -> u32);
    function(a, b, c, d, e, f)
}

/// Adds the object's position to a 2-D point and sends it to the first sink.
///
/// In plain English: build `{ x + object[+0xAC], y + object[+0xB0], constant }`
/// and pass it to `0x800E2220`. The third component is a fixed SDA2 word, not
/// an argument.
///
/// # Original
/// - Ghidra: `FUN_800e22f0` at `0x800E22F0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e22f0_FUN_800e22f0.c`
///
/// # Arguments
/// * `x`, `y` - point offset before the object's own position is added.
/// * `sink` - first argument forwarded to `0x800E2220`.
/// * `object` - object whose `+0xAC` / `+0xB0` floats are its position.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One call; the sink decides what it does with the vector.
///
/// # Safety
/// `object + 0xB0` must be readable and the sink must accept the vector.
///
/// # Confidence
/// high - both additions, the constant word and the call order are literal.
pub unsafe fn scene_transition_position_a(x: f64, y: f64, sink: u32, object: usize) {
    let mut position = crate::platform::GuestLocal::new( [0u32; 3]);
    position[0] = ((x + read_float(object, 0xAC) as f64) as f32).to_bits();
    position[1] = ((y + read_float(object, 0xB0) as f64) as f32).to_bits();
    position[2] = read_global_word(F32_SDA2_63F0);
    let _ = platform::call2(FN_800E2220, sink, position.as_mut_ptr() as u32);
}

/// Adds the object's position to a 2-D point and sends it to the second sink.
///
/// Same arithmetic as [`scene_transition_position_a`], but the destination is
/// `0x800E2340`.
///
/// # Original
/// - Ghidra: `FUN_800e23c0` at `0x800E23C0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e23c0_FUN_800e23c0.c`
///
/// # Arguments
/// * `x`, `y` - point offset before the object's own position is added.
/// * `sink` - first argument forwarded to `0x800E2340`.
/// * `object` - object whose `+0xAC` / `+0xB0` floats are its position.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One call.
///
/// # Safety
/// Same as [`scene_transition_position_a`].
///
/// # Confidence
/// high - the body differs from the first wrapper only in its sink address.
pub unsafe fn scene_transition_position_b(x: f64, y: f64, sink: u32, object: usize) {
    let mut position = crate::platform::GuestLocal::new( [0u32; 3]);
    position[0] = ((x + read_float(object, 0xAC) as f64) as f32).to_bits();
    position[1] = ((y + read_float(object, 0xB0) as f64) as f32).to_bits();
    position[2] = read_global_word(F32_SDA2_63F0);
    let _ = platform::call2(FN_800E2340, sink, position.as_mut_ptr() as u32);
}

/// Fills the vertex builder's second array and copies its position word.
///
/// In plain English: call `0x800E2DE0` with the array base, its byte end
/// (`base + count * 0x0C`), the destination array and the fixed flags, then
/// copy the `+8` word of the first source vertex into the `+8` word of every
/// destination vertex. The destination count is `count * byte_6 + 1`.
///
/// # Original
/// - Ghidra: `FUN_800e2c70` at `0x800E2C70` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2c70_FUN_800e2c70.c`
///
/// # Arguments
/// * `builder` - 0x10-byte header built by [`fn_800e2ad0`]: `+4` count,
///   `+6` multiplier, `+8` source array, `+0xC` destination array.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Fills the destination array and writes one word per destination vertex.
///
/// # Safety
/// The arrays must be live and `count * byte_6 + 1` destinations must fit.
///
/// # Confidence
/// high - the count expressions, the call arguments and the stride are
/// literal. `// unknown:` the scale constant's numeric value.
pub unsafe fn vertex_finalize_indices(builder: usize) {
    let source = read_word(builder, 0x08) as usize;
    let count = platform::read_u16(builder + 4) as u32;
    let destination = read_word(builder, 0x0C) as usize;
    let _ = call_f64_u32x5(
        FN_800E2DE0,
        read_const_f32(F32_SDA2_63C8) as f64,
        source as u32,
        (source + count as usize * 0x0C) as u32,
        destination as u32,
        0,
        8,
    );

    let count_all = platform::read_u16(builder + 4) as u32
        * platform::read_u8(builder + 6) as u32
        + 1;
    let mut out = destination;
    let mut index = 0u32;
    while index < count_all {
        // observed: the C re-reads `source[2]` on every iteration, not once.
        write_word(out, 0x08, read_word(source, 0x08));
        out += 0x0C;
        index += 1;
    }
}

/// Stores one vertex word triple.
///
/// In plain English: write `source[0..2]` to `builder + 8 + index * 0x0C`.
///
/// # Original
/// - Ghidra: `FUN_800e2d00` at `0x800E2D00` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2d00_FUN_800e2d00.c`
///
/// # Arguments
/// * `builder` - vertex-array header; `+8` is the array base.
/// * `index` - vertex index.
/// * `source` - three words copied verbatim.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three word stores.
///
/// # Safety
/// The indexed vertex and the source triple must be writable/readable.
///
/// # Confidence
/// high - three literal stores.
pub unsafe fn vertex_set(builder: usize, index: u32, source: usize) {
    let vertex = read_word(builder, 0x08) as usize + index as usize * 0x0C;
    write_word(vertex, 0x00, read_word(source, 0x00));
    write_word(vertex, 0x04, read_word(source, 0x04));
    write_word(vertex, 0x08, read_word(source, 0x08));
}

/// Counts the vertices in the builder's two arrays.
///
/// In plain English: `count * byte_6 + 1` when the byte at `+7` is zero, else
/// `count * byte_6`.
///
/// # Original
/// - Ghidra: `FUN_800e2d30` at `0x800E2D30` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2d30_FUN_800e2d30.c`
///
/// # Arguments
/// * `builder` - vertex-array header.
///
/// # Returns
/// The element count as an unsigned 32-bit value.
///
/// # Side effects
/// None.
///
/// # Safety
/// `builder + 4..+7` must be readable.
///
/// # Confidence
/// high - one branch and two multiply-adds.
pub unsafe fn vertex_count(builder: usize) -> u32 {
    let count = platform::read_u16(builder + 4) as u32;
    let multiplier = platform::read_u8(builder + 6) as u32;
    if platform::read_u8(builder + 7) == 0 {
        count * multiplier + 1
    } else {
        count * multiplier
    }
}

/// Returns the address of one vertex in the builder's second array.
///
/// # Original
/// - Ghidra: `FUN_800e2d60` at `0x800E2D60` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2d60_FUN_800e2d60.c`
///
/// # Arguments
/// * `builder` - vertex-array header; `+0xC` is the array base.
/// * `index` - vertex index.
///
/// # Returns
/// `array + index * 0x0C`.
///
/// # Side effects
/// None.
///
/// # Safety
/// The returned address must stay inside the array.
///
/// # Confidence
/// high - one multiply-add.
pub unsafe fn vertex_at(builder: usize, index: u32) -> usize {
    read_word(builder, 0x0C) as usize + index as usize * 0x0C
}

/// Evaluates the first curve weight.
///
/// In plain English: `(centre - value)^2 * scale`, all in `f32`.
///
/// # Original
/// - Ghidra: `FUN_800e2d70` at `0x800E2D70` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2d70_FUN_800e2d70.c`
///
/// # Arguments
/// * `value` - the curve input.
///
/// # Returns
/// The weighted square as a `double` (the export returns `double`).
///
/// # Side effects
/// None.
///
/// # Safety
/// No pointers; the two SDA2 floats must be mapped.
///
/// # Confidence
/// high - one subtraction, one square and one scale; the constants are read by
/// address because their values are not in the export.
pub unsafe fn curve_weight_neg_square(value: f64) -> f64 {
    let difference = (read_const_f32(F32_SDA2_63C4) as f64 - value) as f32;
    (difference * difference * read_const_f32(F32_SDA2_63C0)) as f64
}

/// Evaluates the second curve weight.
///
/// In plain English: `(centre - value) * value + scale`.
///
/// # Original
/// - Ghidra: `FUN_800e2d90` at `0x800E2D90` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e2d90_FUN_800e2d90.c`
///
/// # Arguments
/// * `value` - the curve input.
///
/// # Returns
/// The product as a `double` (the export returns `double`).
///
/// # Side effects
/// None.
///
/// # Safety
/// No pointers; the two SDA2 floats must be mapped.
///
/// # Confidence
/// high - the exact cast order is preserved: the difference is rounded to
/// `f32` before the multiply, then the sum is rounded to `f32`.
pub unsafe fn curve_weight_product(value: f64) -> f64 {
    let difference = (read_const_f32(F32_SDA2_63C4) as f64 - value) as f32 as f64;
    let product = (difference * value) as f32;
    (product + read_const_f32(F32_SDA2_63C0)) as f64
}

// ===========================================================================
// Wave 273 lane E - untabled gap body in the 0x800E main-DOL band (`_E273`)
// ===========================================================================
//
// One prologue-shaped body that no documented extent covered and that the
// wave-266 sweep (`work/wave266/e_candidates.tsv`) listed as still untranslated.
// It was re-walked against `work/dol/main.elf` for this wave: the first word is
// `stwu r1,-0x10(r1)` + `mflr r0`, the recursive descent ends on a single
// `blr` at `0x800E2C6C`, and no extent in `src/` starts or overlaps the address
// ([`fn_800e2ad0`] ends at `0x800E2C2C`, one zero word follows, and
// [`vertex_finalize_indices`] starts at `0x800E2C70`).
//
// | address | extent | bytes | role |
// |---|---|---|---|
// | `0x800E2C30` | `0x800E2C30..0x800E2C70` | 64 | deleting destructor, slot 0 of the vertex-buffer vtable `0x8031B010` |

/// `0x802B9280(object)`: the shared deallocator used by the release helpers.
const W273_FN_802B9280: usize = 0x802B_9280;

/// Deletes the two-array vertex buffer when the release count is positive.
///
/// The body is the scalar-release shape used throughout the engine
/// ([`crate::ui::hud::fn_80038040`]): only `this != 0 && release > 0` frees the
/// storage through `0x802B9280(this)`; the receiver is returned either way. The
/// vtable slot identifies it as the deleting destructor of the class whose
/// constructor is [`fn_800e2ad0`] (`0x8031B018` is slot 0 of `0x8031B010`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800E2C30`.
/// - Extent: `0x800E2C30..0x800E2C70` (64 bytes), ending at the `blr` at
///   `0x800E2C6C`; the zero word at `0x800E2C2C` precedes it and
///   [`vertex_finalize_indices`] starts immediately at `0x800E2C70`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: data word `0x8031B018`, slot 0 of the vtable `0x8031B010`
///   ([`VTABLE_8031B010`]) installed by [`fn_800e2ad0`]. Not inside a pointer
///   run.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0xC(r1)`; `mr r31,r3`; `bc 12,2`
///   (`beq` when null); `cmpi cr0,0,r4,0`; `bc 4,1` (`blt` when the count is
///   not positive); `bl 0x802B9280`; `mr r3,r31`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: vertex buffer returned unchanged and freed only on the guarded path.
/// - `release`: signed count; only positive values free `this`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Frees `this` through `0x802B9280` when both guards pass.
///
/// # Safety
/// A non-null `this` with a positive `release` must be the header allocated by
/// [`fn_800e2ad0`] and accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards, the free call and the returned receiver are literal;
/// the class link is the vtable slot, not a symbol.
pub unsafe fn fn_800e2c30(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        let _ = platform::call1(W273_FN_802B9280, this as u32);
    }
    this
}
