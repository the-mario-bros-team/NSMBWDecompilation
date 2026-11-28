//! Node arrays and object construction.
//!
//! **What lives here.**
//! * [`init_150_node_array`] - clears the 150-entry node record (`u8[150]`,
//!   two `f32[150]` lanes, one `Vec3[150]` lane) and links the object into the
//!   scene.
//! * [`init_500_node_array`] - clears the 500-entry node record (six scalar
//!   lanes, `Vec3[500]`, `u16[500]`, `u8[500]`) and paints two per-node colour
//!   words with `0xFFFFFF00`.
//! * [`fn_8000f460`] - constructor: base constructor, then vtable and class
//!   flag.
//! * [`fn_8000f4b0`] - sized factory: allocate `index*4 + 0xE8` bytes, run the
//!   constructor, apply the label.
//! * [`fn_8000f270`] - constructor variant that prepares the `+8` sub-object
//!   first and stores a word at `+0x24`.
//! * [`fn_8000f570`] - per-state dispatch guarded by a class flag bit, with
//!   two virtual-call loops.
//! * [`fn_8000fbf0`] - "free when the signed release flag is positive" helper.
//! * [`fn_80014200`] .. [`fn_80014970`] - actor release helpers and an
//!   intrusive doubly-linked node list (append, remove, clear, destroy).
//! * [`fn_800156b0`], [`fn_80015c10`] - per-frame node position/fade ticks
//!   used by the coin/warning HUD scenes.
//! * [`fn_800190f0`], [`fn_80019360`], [`fn_80019380`] - scene-node list
//!   registration flags.
//! * [`fn_8001e0b0`], [`fn_8001e100`] - actor hit-dispatch predicates.
//!
//! **Original addresses.** `0x80008440`, `0x800099D0`, `0x8000F270`,
//! `0x8000F460`, `0x8000F4B0`, `0x8000F570`, `0x8000FBF0` - all inside the
//! `0x80005000..0x80014000` lane (`src/subsystems/r80005_80014.rs` before
//! promotion).
//!
//! **Wave 13 additions.** This file also owns the actor-band translations from
//! the main.dol range `0x80014000..0x80040000`: the release/list framework at
//! `0x80014200..0x80014970`, the coin-node animation pair
//! (`0x800156B0`/`0x80015C10`), the scene-node list helpers at
//! `0x800190F0..0x80019380`, and the hit predicates at
//! `0x8001E0B0`/`0x8001E100`. Their evidence is the same export directory;
//! each function's `# Original` block names its own file.
//!
//! **Wave 17 additions.** The same actor band also contributes the warning and
//! error scene destructors/state steps at `0x80017060..0x80017FD0`, the scene
//! node teardown/registration helpers at `0x80019080..0x80019390`, the
//! classic-controller scene constructor/builder at
//! `0x80019990`/`0x80019C10`, and the `"Wm_en_hit"` actor state steps at
//! `0x8001DF60..0x8001E850`. See `docs/COVERAGE_WAVE17_ACTOR.md`.
//!
//! **Why it exists.** Both initialisers are the largest clean functions of the
//! lane and reveal their object layouts by construction: every array tiles the
//! next with zero gaps, which is a strong self-check on the offsets. The
//! constructor/factory pair is the only object creation in the lane. Actors
//! built on the shared base initialiser `0x80164FD0` are the consumers.
//!
//! **Dependencies.** `crate::platform`; the opaque actor/scene callees are
//! documented beside their constants: `0x80164FD0` (actor base init),
//! `0x8016A2A0`/`0x8016A0F0` (model flags), `0x802C7C70` (scene sub-object),
//! `0x80258710` (base constructor) and `0x80257640` (label).
//!
//! **Start here.** [`init_150_node_array`] for the smaller record, then
//! [`fn_8000f460`] for the construction path. For the Wave 13 block, start at
//! [`fn_80014820`] (list append) and its three companions, then
//! [`fn_80015c10`] for the coin-pane tick. For Wave 17, start at
//! [`fn_80017060`] (simplest warning teardown), then [`fn_80017bb0`] (the
//! error-window state step) and [`fn_8001e1c0`] (the `Wm_en_hit` state).
//!
//! **Glossary.** "Node array" is a flat per-node record owned by the actor, not
//! a linked structure. See `docs/GLOSSARY.md`.

use crate::platform;
use crate::platform::{call1, call2, call3, call4, call5, call6};
use crate::game::layouts::DActor;

// ---------------------------------------------------------------------------
// Cross-lane callees (opaque here).
//
// Each constant is the *entry* address of a function owned by another lane.
// They are declared so a reader can see which external effects a translation
// has without leaving this file.
// ---------------------------------------------------------------------------

/// Actor/scene base initialiser (`this, param, 0`) returning 1 on success.
/// `0x80007510` is a 4-byte tail thunk to this address.
///
/// Ghidra: 80164fd0 FUN_80164fd0.c ; thunk at 80007510_thunk_FUN_80164fd0.c
const FN_80164FD0: usize = 0x8016_4FD0;

/// Applies two model/instance flags: `(this, a, b)`.
///
/// Ghidra: 8016a2a0 FUN_8016a2a0.c
const FN_8016A2A0: usize = 0x8016_A2A0;

/// Sets one model/instance flag: `(this, flag, value)`.
///
/// Ghidra: 8016a0f0 FUN_8016a0f0.c
const FN_8016A0F0: usize = 0x8016_A0F0;

/// Configures a sub-object built by [`FN_80164FD0`]:
/// `(sub, &vec2, a, b, c, d)`.
///
/// Ghidra: 802c7c70 FUN_802c7c70.c
const FN_802C7C70: usize = 0x802C_7C70;

/// Base-class constructor forwarding three arguments:
/// `(this, a, b, c)`. Stores `b`/`c` at `+0xDC`/`+0xE0` and ORs
/// `0x60000000` into `+0xCC`.
///
/// Ghidra: 80258710 FUN_80258710.c
const FN_80258710: usize = 0x8025_8710;

/// Applies a "type/label" word to an object: `(this, value)`.
///
/// Ghidra: 80257640 FUN_80257640.c
const FN_80257640: usize = 0x8025_7640;

/// Sets up a sub-object at `this+8` before the actor base initialiser:
/// `(sub, param, 0x20)`.
///
/// Ghidra: 8016a690 FUN_8016a690.c
const FN_8016A690: usize = 0x8016_A690;

/// Creates/flushes a render or resource queue with one selector word.
///
/// Ghidra: 8024d5e0 FUN_8024d5e0.c
const FN_8024D5E0: usize = 0x8024_D5E0;

/// Returns the current render/transform context word.
///
/// Ghidra: 80164b30 FUN_80164b30.c
const FN_80164B30: usize = 0x8016_4B30;

/// Copies a transform/state block into caller storage: `(context, out)`.
///
/// Ghidra: 802540b0 FUN_802540b0.c
const FN_802540B0: usize = 0x8025_40B0;

/// Frees an object when the caller's release flag is positive.
///
/// Ghidra: 802b9280 FUN_802b9280.c
const FN_802B9280: usize = 0x802B_9280;

// ---------------------------------------------------------------------------
// Wave 13 actor-band callees (entry addresses in `0x80014000..0x80040000`).
//
// These are entry points owned by other translation lanes. They are named so
// a reader can see each caller's external effects without leaving this file.
// ---------------------------------------------------------------------------

/// Release-if-positive helper: frees the object only when the second argument
/// is positive.
///
/// `0x80014240` calls it with a zero flag, so in this image the guarded free
/// is skipped and the call has no visible effect.
///
/// Ghidra: 8015f800 FUN_8015f800.c
const FN_8015F800: usize = 0x8015_F800;

/// Reads the byte state selector at `this + 0x0A`.
///
/// This 8-byte leaf is below the 16-byte task minimum, so it is called by
/// address instead of translated.
///
/// Ghidra: 800146c0 FUN_800146c0.c
const FN_800146C0: usize = 0x8001_46C0;

/// Base constructor used by `fn_80014970`: installs vtable `0x80329670` at
/// `+0x00` and clears `+0x04`.
///
/// Ghidra: 8016a010 FUN_8016a010.c
const FN_8016A010: usize = 0x8016_A010;

/// Fills the 216-byte table at `this + 0x1B4` with ids `0..0x14`, then id
/// `0x19` and the `0xFF` terminator.
///
/// Ghidra: 801c3fe0 FUN_801c3fe0.c
const FN_801C3FE0: usize = 0x801C_3FE0;

/// Initialises the array at `this + 0x04` with ids `9..0x14`.
///
/// Ghidra: 801c4700 FUN_801c4700.c
const FN_801C4700: usize = 0x801C_4700;

/// Applies the 8-byte entries at `this + 0x1B4` to the renderer bitfields
/// until the `0xFF` terminator.
///
/// Ghidra: 801c3a30 FUN_801c3a30.c
const FN_801C3A30: usize = 0x801C_3A30;

/// Applies the 16-byte entries at `this + 0x04` to the renderer fields until
/// the `0xFF` terminator; selector `0` is forwarded to the callee.
///
/// Ghidra: 801c4250 FUN_801c4250.c
const FN_801C4250: usize = 0x801C_4250;

/// Applies the two-bit value stored at `this + 0x2D0` to the renderer field
/// at `0x254` and marks that state dirty.
///
/// Ghidra: 801c5860 FUN_801c5860.c
const FN_801C5860: usize = 0x801C_5860;

/// Computes four viewport-derived words (two origins, two extents) into
/// `this + 0x2C0..+0x2CC` from the global renderer object.
///
/// Ghidra: 801c9cd0 FUN_801c9cd0.c (a 4-byte thunk forwards to this address).
const FN_801C9CD0: usize = 0x801C_9CD0;

/// Prepares the block at `this + 0x28C` from the global renderer state.
///
/// Ghidra: 801c9900 FUN_801c9900.c
const FN_801C9900: usize = 0x801C_9900;

/// Prepares the block at `this + 0x2A8` from the global renderer state.
///
/// Ghidra: 801c9c40 FUN_801c9c40.c
const FN_801C9C40: usize = 0x801C_9C40;

/// Packs the two-bit value from the global renderer field into the word at
/// `this + 0x2D0`.
///
/// Ghidra: 801c5890 FUN_801c5890.c
const FN_801C5890: usize = 0x801C_5890;

/// Commits the block at `this + 0x28C` through `0x801C98B0` and sets renderer
/// state bit 0.
///
/// Ghidra: 802d31d0 FUN_802d31d0.c
const FN_802D31D0: usize = 0x802D_31D0;

/// Commits the six floats at `this + 0x2A8` through `0x801C9C10` and sets
/// renderer state bit 1.
///
/// Ghidra: 802d3200 FUN_802d3200.c
const FN_802D3200: usize = 0x802D_3200;

/// Commits the four words at `this + 0x2C0` through `0x801C9C60` and sets
/// renderer state bit 2.
///
/// Ghidra: 802d3230 FUN_802d3230.c
const FN_802D3230: usize = 0x802D_3230;

/// Sets the global default byte `0x804248CC` through `0x801C8E10`; the
/// argument is the byte stored at `+0x2D4`.
///
/// Ghidra: 802d2f20 FUN_802d2f20.c
const FN_802D2F20: usize = 0x802D_2F20;

/// Sets the global default byte `0x804248CD` through `0x801C8E40`; the
/// argument is the byte stored at `+0x2D5`.
///
/// Ghidra: 802d2f80 FUN_802d2f80.c
const FN_802D2F80: usize = 0x802D_2F80;

/// Sets the global default byte `0x804248CE` through `0x801C8F90`; the
/// argument is the byte stored at `+0x2D6`.
///
/// Ghidra: 802d2fe0 FUN_802d2fe0.c
const FN_802D2FE0: usize = 0x802D_2FE0;

/// Calls the global state poll `0x801DC8B0` with a 1-byte buffer and returns
/// the resulting byte.
///
/// Ghidra: 801dd1d0 FUN_801dd1d0.c (the call at `0x800156E0` goes through a
/// 4-byte thunk at `0x800B54B0`). The callee takes no arguments.
const FN_801DD1D0: usize = 0x801D_D1D0;

/// Adjusts a two-float point in place with the global camera/screen state:
/// `(point2)`. `// inferred:` the exact projection role from its call site.
///
/// Ghidra: 800b33d0 FUN_800b33d0.c
const FN_800B33D0: usize = 0x800B_33D0;

/// Inserts a node into the global sorted list `0x80377AB0`: `(node)`.
///
/// Ghidra: 80163850 FUN_80163850.c (the call at `0x800190F0` goes through a
/// 4-byte thunk at `0x80006EA0`).
const FN_80163850: usize = 0x8016_3850;

// ---------------------------------------------------------------------------
// Wave 17 callees (warning scenes and the "Wm_en_hit" actor).
//
// Every constant is an entry address owned by another lane. Byte-level
// evidence for each call site is in the function doc that uses it; the roles
// below are one-line summaries of the exported C, not recovered symbols.
// ---------------------------------------------------------------------------

/// Release-if-positive helper: frees `obj` only when `release > 0`, then
/// returns `obj`. The warning-scene destructors call it with `0`, so in this
/// image it only performs the null check.
///
/// Ghidra: 8015f9f0 FUN_8015f9f0.c
const FN_8015F9F0: usize = 0x8015_F9F0;

/// Two-stage sub-object cleanup: `(block, release)`. When `block` is non-null
/// it clears `block + 0xAC` through `0x80006AF0` and `block` through
/// `0x80006E30`, then frees `block` only when `release > 0`.
///
/// Ghidra: 800c89a0 FUN_800c89a0.c
const FN_800C89A0: usize = 0x800C_89A0;

/// Returns zero when the layout animation at `(pane_group, -1)` has finished;
/// non-zero while it is still running. `// inferred:` from every caller's
/// `== 0` test before advancing a state.
///
/// Ghidra: 800c9610 FUN_800c9610.c
const FN_800C9610: usize = 0x800C_9610;

/// Plays/starts one layout animation by index: `(pane_group, index)`.
///
/// Ghidra: 800c9490 FUN_800c9490.c
const FN_800C9490: usize = 0x800C_9490;

/// Starts or stops one layout animation slot: `(pane_group, index, mode)`.
///
/// Ghidra: 800c92f0 FUN_800c92f0.c
const FN_800C92F0: usize = 0x800C_92F0;

/// Rebuilds the layout panes after a bind step: `(pane_group)`.
///
/// Ghidra: 800c9560 FUN_800c9560.c
const FN_800C9560: usize = 0x800C_9560;

/// Registers a transform/animation pair with a counter:
/// `(owner, time, id, link, flag)`.
///
/// Ghidra: 800c9a60 FUN_800c9a60.c
const FN_800C9A60: usize = 0x800C_9A60;

/// Returns the current layout/context word (no arguments).
///
/// Ghidra: 800cdc60 FUN_800cdc60.c
const FN_800CDC60: usize = 0x800C_DC60;

/// Resets the global HUD owner `0x8042A2C8`: `(owner, mode)`.
///
/// Ghidra: 8010c780 FUN_8010c780.c
const FN_8010C780: usize = 0x8010_C780;

/// Plays one sound/effect id through a global handle: `(handle, id, flag)`.
///
/// Ghidra: 80195380 FUN_80195380.c
const FN_80195380: usize = 0x8019_5380;

/// Loads an archive and binds it to a pane group: `(group, path, mode)`.
/// Returns the bound object, or null when the archive is missing.
///
/// Ghidra: 800c8cb0 FUN_800c8cb0.c
const FN_800C8CB0: usize = 0x800C_8CB0;

/// Binds a name table to a pane group: `(group, names, count)`.
///
/// Ghidra: 800c8fb0 FUN_800c8fb0.c
const FN_800C8FB0: usize = 0x800C_8FB0;

/// Binds a pane table plus its data table: `(group, panes, data, count)`.
///
/// Ghidra: 800c90f0 FUN_800c90f0.c
const FN_800C90F0: usize = 0x800C_90F0;

/// Returns the root object of a pane group: `(group)`.
///
/// Ghidra: 80007220 FUN_80007220.c
const FN_80007220: usize = 0x8000_7220;

/// Looks up a named object in a pane group: `(group, name)`. The exported
/// helper at this address is a thunk to `0x80007320`.
///
/// Ghidra: 800c8a90 thunk_FUN_80007320.c
const FN_800C8A90: usize = 0x800C_8A90;

/// Binds three position/animation records: `(group, table_a, table_b, id,
/// count)`.
///
/// Ghidra: 800c8f20 FUN_800c8f20.c
const FN_800C8F20: usize = 0x800C_8F20;

/// Binds a pane array into caller storage: `(group, names, out, count)`.
///
/// Ghidra: 800c8dd0 FUN_800c8dd0.c
const FN_800C8DD0: usize = 0x800C_8DD0;

/// Finishes pane setup with two mode words: `(group, a, b)`.
///
/// Ghidra: 800c93d0 FUN_800c93d0.c
const FN_800C93D0: usize = 0x800C_93D0;

/// Commits the pane group after binding: `(group)`.
///
/// Ghidra: 800c9500 FUN_800c9500.c
const FN_800C9500: usize = 0x800C_9500;

/// Constructs the render/transform block at `(out, a, b, c)`.
///
/// Ghidra: 8015fc90 FUN_8015fc90.c
const FN_8015FC90: usize = 0x8015_FC90;

/// Constructs a small sub-object: `(out)`.
///
/// Ghidra: 800c8950 FUN_800c8950.c
const FN_800C8950: usize = 0x800C_8950;

/// Returns the global controller/vibration handler (no arguments).
///
/// Ghidra: 80107f10 FUN_80107f10.c
const FN_80107F10: usize = 0x8010_7F10;

/// Enables or disables vibration on a handler: `(handler, on)`.
///
/// Ghidra: 80108160 FUN_80108160.c
const FN_80108160: usize = 0x8010_8160;

/// Copies/converts a position block: `(out, position)`.
///
/// Ghidra: 8006a4b0 FUN_8006a4b0.c
const FN_8006A4B0: usize = 0x8006_A4B0;

/// Spawns a named effect at a position: `(name, arg, position, a, b)`.
///
/// Ghidra: 8016c890 FUN_8016c890.c
const FN_8016C890: usize = 0x8016_C890;

/// Plays an effect/sound id with a position: `(handle, id, position, flag)`.
///
/// Ghidra: 80197f00 FUN_80197f00.c
const FN_80197F00: usize = 0x8019_7F00;

// ---------------------------------------------------------------------------
// Image constants read through r2 (small data area 2).
//
// Addresses are absolute; the original instruction is `lfs f, disp(r2)` with
// `r2 = 0x80433080`. Values were read out of `work/dol/main.elf`:
//   0x8042B0A0 = 0x00000000 (0.0f)     0x8042B0D0 = 0xFFFFFF00
// ---------------------------------------------------------------------------

/// `r2-0x7FE0` - zero, used to clear floats.
const ZERO_F32_A: usize = 0x8042_B0A0;
/// `r2-0x7FB0` - per-node colour word, image value `0xFFFFFF00`
/// (channels are 0xFF,0xFF,0xFF,0x00 - `// unknown:` which channel is alpha).
const NODE_COLOR_U32: usize = 0x8042_B0D0;

/// Vtable installed by the constructor at `0x8000F460`. Ghidra:
/// `&DAT_802fe448`.
const VTABLE_802FE448: u32 = 0x802F_E448;

/// Vtable installed by the constructor at `0x80014970`. Ghidra:
/// `&DAT_802ff690`.
const VTABLE_802FF690: u32 = 0x802F_F690;

/// `r2-0x7EFC` - image value `0x3F800000`, `f32 1.0`.
const SDA_ONE_F32: usize = 0x8042_B184;

/// `r2-0x7EF8` - image value `0x38C90FDB`, `f32 0.0000958738` = pi/32768.
/// It converts the signed 16-bit angle at `+0x1C` into radians.
const SDA_ANGLE_SCALE: usize = 0x8042_B188;

/// `r2-0x7ED8` - image value `0x00000000`, `f32 0.0`.
const SDA_ZERO_F32: usize = 0x8042_B1A8;

/// `r2-0x7ED0` - image value `0x3F000000`, `f32 0.5`.
const SDA_HALF_F32: usize = 0x8042_B1B0;

/// `r2-0x7E60` - two `f32` values `1.0`, `-1.0`, selected by the state flag
/// of the `Wm_en_hit` actor. The same table is used for the scale sign and
/// for the state-step sign.
const SDA_SIGN_TABLE: usize = 0x8042_B220;

/// `r2-0x7E54` - image value `0x00000000`, `f32 0.0`.
const SDA_ZERO_F32_B: usize = 0x8042_B22C;

/// `r2-0x7E4C` - image value `0x3FE00000`, `f32 1.75` (X scale base).
const SDA_ACTOR_SCALE_X: usize = 0x8042_B234;

/// `r2-0x7E48` - image value `0x3EB33333`, `f32 0.35` (height factor).
const SDA_ACTOR_HEIGHT_FACTOR: usize = 0x8042_B238;

/// `r2-0x7E44` - image value `0x40700000`, `f32 3.75` (Y scale).
const SDA_ACTOR_SCALE_Y: usize = 0x8042_B23C;

/// `r2-0x7E40` - image value `0x45ABE000`, `f32 5504.0` (effect height).
const SDA_ACTOR_EFFECT_Y: usize = 0x8042_B240;

/// `r2-0x7E3C` - image value `0x40300000`, `f32 2.75` (Y scale for the
/// state-1 vector).
const SDA_ACTOR_SCALE_Y_ALT: usize = 0x8042_B244;

/// `r13-0x5730` - pointer to the per-player controller record array; the
/// active player index is at `0x8042A464` and each slot is a pointer. The
/// record's `+0x1C` word holds the button bits tested by `fn_80017bb0`.
const SDA_CONTROLLER_TABLE: usize = 0x8042_9F50;

/// `r13-0x521C` - signed index of the active player slot.
const SDA_ACTIVE_PLAYER: usize = 0x8042_A464;

/// `r13-0x51F8` - global sound/effect handle used by the warning scenes.
const SDA_WARNING_SOUND_HANDLE: usize = 0x8042_A488;

/// `r13-0x5218` - pointer to the allocation interface whose virtual slot
/// `+0x20` reserves `0x900` bytes for the warning object.
const SDA_WARNING_ALLOCATOR: usize = 0x8042_A468;

/// `r13-0x6F4E` - `u16` global tested before enabling vibration; zero means
/// vibration is allowed.
const SDA_VIBRATION_LOCK: usize = 0x8042_8732;

/// `r13-0x5924` - global sound/effect handle used by the `Wm_en_hit` actor.
const SDA_ACTOR_EFFECT_HANDLE: usize = 0x8042_9D5C;

/// `r13-0x53B8` - global HUD owner passed to `0x8010C780`.
const SDA_HUD_OWNER: usize = 0x8042_A2C8;

/// String `"Wm_en_hit"` at `0x803017C0`, used as the effect name by the
/// `Wm_en_hit` actor state steps.
const STR_WM_EN_HIT: u32 = 0x8030_17C0;

/// Empty BSS event objects used as the argument of the warning scenes'
/// vtable slot `+0x18` (`(state, event)`).
const EVENT_803520F8: u32 = 0x8035_20F8;
const EVENT_80352178: u32 = 0x8035_2178;
const EVENT_803521F8: u32 = 0x8035_21F8;
const EVENT_80352238: u32 = 0x8035_2238;
const EVENT_803522F8: u32 = 0x8035_22F8;
const EVENT_80352338: u32 = 0x8035_2338;
const EVENT_80352378: u32 = 0x8035_2378;

// ---------------------------------------------------------------------------
// Classic-controller warning scene tables (`0x803009E8` is the base of the
// string block; the constants are absolute because they are passed to opaque
// layout helpers).
// ---------------------------------------------------------------------------

/// `"extensionControllerClasic/extensionControllerClasic.arc"`.
const ARC_CLASSIC_CONTROLLER: u32 = 0x8030_0AC8;
/// `"extensionControllerClasic_05.brlyt"`.
const BRLYT_CLASSIC_CONTROLLER: u32 = 0x8030_0B00;
/// Three-name table (pane names).
const PANES_CLASSIC_3: u32 = 0x8030_0A74;
/// Three-pane table bound with [`PANES_CLASSIC_3`].
const PANES_CLASSIC_3B: u32 = 0x8030_0A8C;
/// Data table passed as the third argument of `0x800C90F0`.
const DATA_802EE018: u32 = 0x802E_E018;
/// `"T_player"` name used by the classic-controller scene.
const NAME_T_PLAYER: u32 = 0x8030_0B24;
/// Two-name table at `r13-0x7EF0` (`0x80427790`).
const TABLE_80427790: u32 = 0x8042_7790;
/// Float table at `r2-0x7EA8` (`0x8042B1D8`).
const TABLE_8042B1D8: u32 = 0x8042_B1D8;
/// Three-name table at `r13-0x7EE8` (`0x80427798`).
const TABLE_80427798: u32 = 0x8042_7798;
/// Empty BSS event object for the classic-controller scene.
const EVENT_803523F8: u32 = 0x8035_23F8;

/// Vtables and sub-vtables installed by `fn_80019990`.
const VT_80300B30: u32 = 0x8030_0B30;
const VT_80300B40: u32 = 0x8030_0B40;
const VT_80300B70: u32 = 0x8030_0B70;
const VT_802FEAF8: u32 = 0x802F_EAF8;
const VT_80300BA0: u32 = 0x8030_0BA0;
const VT_80300BB8: u32 = 0x8030_0BB8;

/// Global array of pane-group owners indexed by the word at `this + 0x218`.
const SDA_PANE_GROUP_TABLE: usize = 0x8037_7C88;

/// `r13-0x5200` - pointer to a global frame object; its `u16` at `+0x06` is
/// the counter consumed by `fn_800156b0`. `// unknown:` owner and reset site.
const SDA_FRAME_COUNTER_PTR: usize = 0x8042_A480;

/// Three consecutive default bytes (`0x804248CC..0x804248CE`) copied into the
/// `+0x2D4..+0x2D6` fields by `fn_80014380`. The region is BSS at boot, so the
/// translation reads it at runtime instead of assuming zero.
const SDA_DEFAULT_FLAGS: usize = 0x8042_48CC;

// ---------------------------------------------------------------------------
// Local helpers.
// ---------------------------------------------------------------------------

/// Reads a big-endian `f32` word without changing its bit pattern.
#[inline]
unsafe fn read_f32(addr: usize) -> f32 {
    f32::from_bits(platform::read_u32(addr))
}

/// Writes a big-endian `f32` word without changing its bit pattern.
#[inline]
unsafe fn write_f32(addr: usize, value: f32) {
    platform::write_u32(addr, value.to_bits());
}

// ===========================================================================
// 1. Node-array initialisers
// ===========================================================================

/// Initialises a 150-entry node array: one `Vec3`, two `f32` lanes, one `u8`
/// lane per entry, all zeroed; then links the object into the scene.
///
/// The object layout recovered from the loop is contiguous:
///
/// | offset | contents |
/// |---|---|
/// | `+0x038` | `u8[150]` |
/// | `+0x0D0` | `f32[150]` |
/// | `+0x328` | `f32[150]` |
/// | `+0x580` | `Vec3[150]` (12 bytes each, so ends at `+0xC88`) |
///
/// The compiler unrolled the loop eight times and peeled six entries; the loop
/// below reproduces the same 150 updates.
///
/// # Original
/// - Ghidra: `FUN_80008440` at `0x80008440` (632 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80008440_FUN_80008440.c`
///
/// # Arguments
/// * `this` - object to initialise (see table above).
/// * `point` - pointer to one word copied into the model sub-object.
/// * `pose` - signed animation/pose index; negative means "no pose".
/// * `arg_a` - forwarded as the second word of the negative-pose branch.
/// * `arg_b` - scene/model parameter forwarded to `0x80164FD0`.
///
/// # Returns
/// `1` on success, `0` when the base initialiser `0x80164FD0` fails.
///
/// # Side effects
/// Calls `0x80164FD0`, `0x802C7C70`, `0x8016A2A0`, `0x8016A0F0`; performs
/// 150 × (3 + 1 + 1) = 750 `f32` stores plus 150 `u8` stores.
///
/// # Safety
/// `this` must be valid up to `+0xC88`. `point` readable for 4 bytes.
///
/// # Confidence
/// high for the loop and layout; medium for the role of `pose`/`arg_a`.
pub unsafe fn init_150_node_array(
    this: usize,
    point: usize,
    pose: i32,
    arg_a: u32,
    arg_b: u32,
) -> u32 {
    // observed 0x80008474: `bl 0x80007510`, a 4-byte thunk to 0x80164FD0.
    let mut local = crate::platform::GuestLocal::new( [0u32; 2]);
    if call3(FN_80164FD0, this as u32, arg_b, 0) == 0 {
        return 0;
    }

    // observed 0x80008488..0x80008498: load *point, then
    // `0x802C7C70(this+8, &local, 0, 0, 1, 1)`.
    local[0] = platform::read_u32(point);
    call6(FN_802C7C70, (this + 8) as u32, local.as_mut_ptr() as u32, 0, 0, 1, 1);

    // observed 0x800084E8: branch on the signed pose index.
    if pose < 0 {
        call3(FN_8016A2A0, this as u32, 0, arg_a);
        call3(FN_8016A0F0, this as u32, 6, 1);
    } else {
        call3(FN_8016A2A0, this as u32, pose as u32, 0);
        call3(FN_8016A0F0, this as u32, 7, 1);
    }

    // observed 0x80008478..0x800084B4: the zero word comes from 0x8042B0A0.
    let zero = read_f32(ZERO_F32_A);

    // The compiled loop body is unrolled 8x (`iVar2 += 0x60`, `iVar4 += 0x20`,
    // `iVar6 += 8`) with a six-entry peeled tail after the 18 iterations.
    let mut i = 0;
    while i < 150 {
        write_f32(this + 0x580 + i * 12, zero);
        write_f32(this + 0x584 + i * 12, zero);
        write_f32(this + 0x588 + i * 12, zero);
        write_f32(this + 0x328 + i * 4, zero);
        write_f32(this + 0xd0 + i * 4, zero);
        platform::write_u8(this + 0x38 + i, 0);
        i += 1;
    }

    1
}

/// Initialises a 500-entry node array: clears six scalar lanes and one `Vec3`
/// lane, and paints two per-node colour words with `0xFFFFFF00`.
///
/// The object layout recovered from the loop is contiguous and very regular:
///
/// | offset | contents | ends at |
/// |---|---|---|
/// | `+0x0FC8` | `f32[500]` | `+0x17B8` |
/// | `+0x1F68` | `f32[500]` | `+0x2738` |
/// | `+0x2738` | `u32[500]` (colour `0xFFFFFF00`) | `+0x2F08` |
/// | `+0x2F08` | `u32[500]` (colour `0xFFFFFF00`) | `+0x36D8` |
/// | `+0x36D8` | `f32[500]` | `+0x3EA8` |
/// | `+0x3EA8` | `f32[500]` | `+0x4678` |
/// | `+0x4678` | `Vec3[500]` (12 bytes each) | `+0x5DE8` |
/// | `+0x5DE8` | `u16[500]` | `+0x61D0` |
/// | `+0x61D0` | `u8[500]` | `+0x63C4` |
///
/// The compiler unrolled two entries per iteration (`0xFA` = 250 iterations),
/// which is why the pseudocode writes two entries per pass; the loop below is
/// the same 500 updates.
///
/// # Original
/// - Ghidra: `FUN_800099D0` at `0x800099D0` (504 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800099d0_FUN_800099d0.c`
///
/// # Arguments
/// * `this` - object to initialise (table above).
/// * `point` - pointer to one word copied into the model sub-object.
/// * `pose` - signed animation/pose index; negative means "no pose".
/// * `arg_a` - forwarded as the second word of the negative-pose branch.
/// * `arg_b` - scene/model parameter forwarded to `0x80164FD0`.
///
/// # Returns
/// `1` on success, `0` when `0x80164FD0` fails.
///
/// # Side effects
/// Calls `0x80164FD0`, `0x802C7C70`, `0x8016A2A0`, `0x8016A0F0`; performs
/// 500 × (3 + 4 + 2) = 4,500 word stores, 500 `u16` stores and 500 `u8`
/// stores.
///
/// # Safety
/// `this` must be valid up to `+0x63C4`. `point` readable for 4 bytes.
///
/// # Confidence
/// high for the layout (the arrays tile each other exactly, which is a strong
/// self-check); medium for the role of the colour words.
pub unsafe fn init_500_node_array(
    this: usize,
    point: usize,
    pose: i32,
    arg_a: u32,
    arg_b: u32,
) -> u32 {
    // observed 0x800099FC: `bl 0x80007510` (thunk to 0x80164FD0).
    let mut local = crate::platform::GuestLocal::new( [0u32; 2]);
    if call3(FN_80164FD0, this as u32, arg_b, 0) == 0 {
        return 0;
    }

    local[0] = platform::read_u32(point);
    call6(FN_802C7C70, (this + 8) as u32, local.as_mut_ptr() as u32, 0, 0, 1, 1);

    if pose < 0 {
        call3(FN_8016A2A0, this as u32, 0, arg_a);
        call3(FN_8016A0F0, this as u32, 6, 1);
    } else {
        call3(FN_8016A2A0, this as u32, pose as u32, 0);
        call3(FN_8016A0F0, this as u32, 7, 1);
    }

    let zero = read_f32(ZERO_F32_A);
    let colour = platform::read_u32(NODE_COLOR_U32);

    let mut i = 0;
    while i < 500 {
        write_f32(this + 0x4678 + i * 12, zero);
        write_f32(this + 0x467C + i * 12, zero);
        write_f32(this + 0x4680 + i * 12, zero);
        write_f32(this + 0x3EA8 + i * 4, zero);
        write_f32(this + 0x36D8 + i * 4, zero);
        write_f32(this + 0x1F68 + i * 4, zero);
        write_f32(this + 0x0FC8 + i * 4, zero);
        platform::write_u32(this + 0x2738 + i * 4, colour);
        platform::write_u32(this + 0x2F08 + i * 4, colour);
        platform::write_u16(this + 0x5DE8 + i * 2, 0);
        platform::write_u8(this + 0x61D0 + i, 0);
        i += 1;
    }

    1
}

// ===========================================================================
// 2. Object construction
// ===========================================================================

/// Constructor: runs the base constructor, then installs the vtable and a
/// class flag.
///
/// The body forwards its three arguments to `0x80258710` (which stores two of
/// them at `+0xDC`/`+0xE0`), writes `&DAT_802FE448` at `+0x00`, and rewrites
/// the flag word at `+0xCC`: clear bit 30 (`0x40000000`), set bit 29
/// (`0x20000000`).
///
/// # Original
/// - Ghidra: `FUN_8000F460` at `0x8000F460` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f460_FUN_8000f460.c`
///
/// # Arguments
/// * `this` - new object.
/// * `a`, `b`, `c` - forwarded to `0x80258710`; `b`/`c` are stored at
///   `+0xDC`/`+0xE0` by that constructor.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x80258710`; three word stores (vtable + flag word).
///
/// # Safety
/// `this` must be a valid object of at least `0xD0` bytes.
///
/// # Confidence
/// high for the stores; medium for the vtable's class identity (no RTTI name
/// is available in the image).
pub unsafe fn fn_8000f460(this: usize, a: u32, b: u32, c: u32) -> u32 {
    // observed 0x8000F474: `bl 0x80258710` with r4..r6 still holding the
    // incoming arguments.
    call4(FN_80258710, this as u32, a, b, c);

    // observed 0x8000F478..0x8000F484.
    platform::write_u32(this, VTABLE_802FE448);

    // observed 0x8000F488..0x8000F490:
    //   oris  r0,r0,0x2000      ; set   0x20000000
    //   rlwinm r0,r0,0,2,0      ; clear 0x40000000
    let flags = platform::read_u32(this + 0xCC);
    platform::write_u32(this + 0xCC, (flags & 0xBFFF_FFFF) | 0x2000_0000);

    this as u32
}

/// Allocates and constructs a variable-size object through the owner's
/// allocator, then applies a label.
///
/// The size is `index * 4 + 0xE8` bytes (232 + 4 per index); the object is
/// constructed by [`fn_8000f460`] with `(owner, new + 0xE8, index)` as the
/// forwarded arguments, which places the per-index array right after the
/// fixed 0xE8-byte header. On success the label is applied with `0x80257640`.
///
/// # Original
/// - Ghidra: `FUN_8000F4B0` at `0x8000F4B0` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f4b0_FUN_8000f4b0.c`
///
/// # Arguments
/// * `owner` - object owning the allocator at `[owner+0x14]`.
/// * `size_out` - optional pointer (0 = none); receives the requested size.
/// * `index` - count-like selector; also forwarded to the constructor.
/// * `label` - word applied to the new object with `0x80257640`.
///
/// # Returns
/// The new object, or `0` when allocation fails.
///
/// # Side effects
/// One virtual call to allocator slot `+0x14` (`alloc(owner, size, 4)`), the
/// constructor, then `0x80257640`.
///
/// # Safety
/// `owner` must be valid and `[owner+0x14]` a valid allocator object with a
/// readable vtable; `size_out` either 0 or writable.
///
/// # Confidence
/// medium - allocation math and call order are exact; the allocator
/// interface is inferred from the virtual slot.
pub unsafe fn fn_8000f4b0(owner: usize, size_out: u32, index: i32, label: u32) -> u32 {
    // observed 0x8000F4B8: `rlwinm r7,r5,2,0,29` then `addi r0,r7,0xEB` and
    // `rlwinm r0,r0,0,0,29` - the mask is a no-op because 0xE8 is aligned.
    let size = (index as u32).wrapping_mul(4).wrapping_add(0xE8);

    if size_out != 0 {
        platform::write_u32(size_out as usize, size);
    }

    // observed 0x8000F4F0..0x8000F508: load allocator from [owner+0x14], take
    // vtable slot +0x14, call it as `(allocator, size, 4)`.
    let allocator = platform::read_u32(owner + 0x14);
    let vtable = platform::read_u32(allocator as usize);
    let allocate = platform::read_u32(vtable as usize + 0x14);

    let memory = call3(allocate as usize, allocator, size, 4);
    if memory == 0 {
        return 0;
    }

    // observed 0x8000F52C..0x8000F534: constructor(owner, new+0xE8, index).
    let object = fn_8000f460(
        memory as usize,
        owner as u32,
        memory.wrapping_add(0xE8),
        index as u32,
    );

    // observed 0x8000F538..0x8000F548.
    if object != 0 {
        call2(FN_80257640, object, label);
    }

    object
}

// ===========================================================================
// 3. Actor base-constructor variants and release helpers
// ===========================================================================

/// Calls a no-argument function that returns `u32`.
#[inline]
unsafe fn call0_r(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Calls a PowerPC function with six `f32` arguments in `f1..f13`.
///
/// `crate::platform::call6` passes integer registers; the original
/// `0x802D3200` call uses floating registers, so this local helper keeps the
/// argument class correct.
#[inline]
unsafe fn call6_f32(
    addr: usize,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
) {
    let func = crate::platform::guest_fn!(addr, fn(f32, f32, f32, f32, f32, f32));
    func(a, b, c, d, e, f)
}

/// A 48-byte, 8-byte-aligned scratch block used by [`fn_8000f570`].
///
/// The original builds a 48-byte transform/state block on the stack and passes
/// its address to `0x802540B0` and to the per-entry callbacks.
#[repr(C, align(8))]
struct Scratch48([u8; 48]);

impl Scratch48 {
    #[inline]
    fn new() -> Self {
        Self([0u8; 48])
    }

    #[inline]
    fn addr(&mut self) -> u32 {
        self.0.as_mut_ptr() as u32
    }
}

/// Constructor variant that first prepares a sub-object at `this+8`, then runs
/// the shared actor base initialiser and stores one word at `+0x24`.
///
/// The body is byte-identical to `0x8000F370`; the two addresses differ only in
/// which translation unit's inline copy the compiler emitted.
///
/// # Original
/// - Ghidra: `FUN_8000F270` at `0x8000F270` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f270_FUN_8000f270.c`
///
/// # Arguments
/// * `this` - new object; `this+8` is the model/scene sub-object.
/// * `param_2` - word stored at `+0x24` when construction succeeds (role
///   `// unknown:`).
/// * `param_3` - forwarded to the sub-object setup `0x8016A690`.
///
/// # Returns
/// `true` when `0x80164FD0` reports success, `false` otherwise.
///
/// # Side effects
/// Calls `0x8016A690(this+8, param_3, 0x20)` and
/// `0x80164FD0(this, this+8, 0)`; on success writes `+0x24`.
///
/// # Safety
/// `this` must be valid up to `+0x28` and acceptable to both callees.
///
/// # Confidence
/// high - the call order, operands and conditional store are explicit.
pub unsafe fn fn_8000f270(this: usize, param_2: u32, param_3: u32) -> bool {
    // observed 0x8000F280: `0x8016A690(this + 8, param_3, 0x20)`.
    call3(FN_8016A690, (this + 8) as u32, param_3, 0x20);

    // observed 0x8000F290: `bl 0x80007510`, the shared 4-byte thunk to
    // 0x80164FD0; r4 is `this + 8`.
    let ok = call3(FN_80164FD0, this as u32, (this + 8) as u32, 0);

    if ok != 0 {
        // observed 0x8000F2A0: store param_2 at +0x24 only on success.
        platform::write_u32(this + 0x24, param_2);
    }

    ok != 0
}

/// Dispatches one of two per-state update paths, guarded by a class flag bit.
///
/// The first guard tests bit `kind-1` of the flag word at `+0xCC` using the
/// PowerPC `slw` shift semantics (count masked to 6 bits). If the bit is clear
/// and the entry count at `+0xE4` is non-zero:
///
/// * `kind == 5` calls the caller-supplied virtual object once with the two
///   flag bits at 29/30 as booleans;
/// * `kind == 6` flushes `0x7FF`, captures the current transform context,
///   builds a 48-byte block through `0x802540B0`, and walks the `+0xDC` pointer
///   array. Each entry receives four virtual calls (modes 2, 1, 4, 6); after
///   the first entry a local flag bit 2 suppresses the mode-2 call.
///
/// `_param_3` is not read by the original.
///
/// # Original
/// - Ghidra: `FUN_8000F570` at `0x8000F570` (428 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f570_FUN_8000f570.c`
///
/// # Arguments
/// * `this` - actor owning the flag word `+0xCC`, count `+0xE4` and pointer
///   array `+0xDC`.
/// * `kind` - selector; only `5` and `6` have bodies, `// inferred:` from the
///   two mode values and the two virtual call shapes.
/// * `_param_3` - unused by the original body.
/// * `vtable_owner` - object whose vtable slot `+0x0C` is used for `kind == 5`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8024D5E0`, `0x80164B30`, `0x802540B0`, and up to four virtual
/// calls per entry through each entry's vtable slot `+0x0C`.
///
/// # Safety
/// `this+0xCC/+0xDC/+0xE4` must be readable; when the guard passes, the
/// `+0xDC` array must hold `+0xE4` valid object pointers whose first word is a
/// vtable with a function at `+0x0C`. `vtable_owner` must be valid for the
/// `kind == 5` branch.
///
/// # Confidence
/// high for the flow and all operands; medium for the `kind` values' meaning.
pub unsafe fn fn_8000f570(
    this: usize,
    kind: u32,
    _param_3: u32,
    vtable_owner: usize,
) {
    let flags = platform::read_u32(this + 0xCC);

    // observed: `param_2 < 9 && ((1 << (param_2 - 1)) & flags) != 0`.
    // The PowerPC `slw` masks the count to six bits; a count >= 32 yields zero.
    let shift = kind.wrapping_sub(1) & 0x3F;
    let bit = if shift >= 32 { 0 } else { 1u32 << shift };
    let already_set = kind < 9 && (bit & flags) != 0;

    if already_set {
        return;
    }
    if platform::read_u32(this + 0xE4) == 0 {
        return;
    }

    let mut local_44 = crate::platform::GuestLocal::<u32>::new( 0);

    if kind == 5 {
        // observed: `(**(code **)(*param_4 + 0x0C))(param_4, this,
        // (flags >> 29) & 1, (flags >> 30) & 1)`.
        let vtable = platform::read_u32(vtable_owner) as usize;
        let entry = platform::read_u32(vtable + 0x0C) as usize;
        platform::call4(
            entry,
            vtable_owner as u32,
            this as u32,
            (flags >> 0x1D) & 1,
            (flags >> 0x1E) & 1,
        );
        return;
    }

    if kind != 6 {
        return;
    }

    // observed: flush, capture the current context, then build the block.
    let _ = platform::call1(FN_8024D5E0, 0x7FF);
    let mut context = crate::platform::GuestLocal::new( call0_r(FN_80164B30));
    let mut block = Scratch48::new();
    let _ = platform::call2(
        FN_802540B0,
        (context.as_mut_ptr() as *mut u32) as u32,
        block.addr(),
    );

    let count = platform::read_u32(this + 0xE4);
    let array = platform::read_u32(this + 0xDC) as usize;

    let mut index: u32 = 0;
    let mut offset: u32 = 0;
    while index < count {
        // observed: `piVar2 = *(int **)(*(int *)(this + 0xDC) + iVar4)`.
        let entry_object = platform::read_u32(array + offset as usize) as usize;

        if (*local_44 & 4) == 0 {
            let vtable = platform::read_u32(entry_object) as usize;
            let call = platform::read_u32(vtable + 0x0C) as usize;
            platform::call4(call, entry_object as u32, 2, 0, 0);
        }

        let vtable = platform::read_u32(entry_object) as usize;
        let call = platform::read_u32(vtable + 0x0C) as usize;
        platform::call4(call, entry_object as u32, 1, 0, 0);

        let vtable = platform::read_u32(entry_object) as usize;
        let call = platform::read_u32(vtable + 0x0C) as usize;
        platform::call4(call, entry_object as u32, 4, 0, block.addr());

        let vtable = platform::read_u32(entry_object) as usize;
        let call = platform::read_u32(vtable + 0x0C) as usize;
        platform::call4(
            call,
            entry_object as u32,
            6,
            0,
            (local_44.as_mut_ptr() as *mut u32) as u32,
        );

        offset = offset.wrapping_add(4);
        *local_44 |= 4;
        index += 1;
    }
}

/// Conditionally frees an object in the CodeWarrior "release if positive"
/// shape and returns the pointer unchanged.
///
/// The object is freed only when the pointer is non-null **and** the signed
/// second argument is positive, exactly like the sibling helpers at
/// `0x8000FE90`, `0x8000FED0`, `0x8000FFB0` and `0x8000FFF0`.
///
/// # Original
/// - Ghidra: `FUN_8000FBF0` at `0x8000FBF0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000fbf0_FUN_8000fbf0.c`
///
/// # Arguments
/// * `this` - object pointer, returned unchanged.
/// * `release` - signed release flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`, so callers can chain the release.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// A non-null `this` with a positive `release` must be a heap object that
/// `0x802B9280` can free.
///
/// # Confidence
/// high - the two guards and the return are explicit.
pub unsafe fn fn_8000fbf0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        // observed: Ghidra prints `FUN_802b9280()` without operands; sibling
        // release helpers in `engine/display.rs` pass the object in r3.
        let _ = call1(FN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// 4. Actor release helpers and intrusive node list (0x80014200..0x80014970)
// ===========================================================================
//
// This block is a small C++ subsystem recovered from its own code:
//
// * four "delete when the signed release flag is positive" helpers, emitted
//   once per translation unit (`0x80014200`, `0x80014240`, `0x80014340`,
//   `0x800145F0`);
// * a registration/unregistration pair for the sub-objects of a scene node
//   (`0x80014380`, `0x80014480`);
// * four state-scaled accessors (`0x80014640`..`0x800147E0`);
// * an intrusive doubly-linked list (`0x80014820`..`0x80014910`), followed by
//   an adjacent vtable constructor (`0x80014970`) whose class is not named.
//
// List layout used by the four list functions:
//
// | offset | contents |
// |---|---|
// | `+0x00` | `u32` element count |
// | `+0x04` | pointer to the first node (the head) |
// | `+0x08` | pointer to the last node (the tail) |
//
// Node layout:
//
// | offset | contents |
// |---|---|
// | `+0x00` | next node toward the head |
// | `+0x04` | previous node toward the tail |

/// Releases `this` when the pointer is non-null and `release` is positive.
///
/// This is the smallest member of the CodeWarrior "release if positive"
/// family. The pointer is returned unchanged so callers can chain deletion.
///
/// # Original
/// - Ghidra: `FUN_80014200` at `0x80014200` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014200_FUN_80014200.c`
///
/// # Arguments
/// * `this` - object pointer, returned unchanged.
/// * `release` - signed release flag; freeing happens only when it is `> 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// A non-null `this` with positive `release` must be a heap object that
/// `0x802B9280` can free.
///
/// # Confidence
/// high - the two guards and the return are explicit.
pub unsafe fn fn_80014200(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        // observed 0x80014218: `bl 0x802b9280` with r3 still `this`.
        let _ = call1(FN_802B9280, this as u32);
    }
    this
}

/// Releases `this` after calling the release-if-positive helper.
///
/// Unlike [`fn_80014200`], the helper call is unconditional once the object
/// exists: only the free itself depends on `release`. The helper is called
/// with a zero flag, so its own guarded free is skipped in this image.
///
/// # Original
/// - Ghidra: `FUN_80014240` at `0x80014240` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014240_FUN_80014240.c`
///
/// # Arguments
/// * `this` - object pointer, returned unchanged.
/// * `release` - signed release flag; freeing happens only when it is `> 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x8015F800(this, 0)` for every non-null object, then optionally
/// `0x802B9280(this)`.
///
/// # Safety
/// `this` must be valid for `0x8015F800`; when `release > 0` it must also be
/// a heap object that `0x802B9280` can free.
///
/// # Confidence
/// high - the byte stream shows the cleanup call guarded only by `this != 0`.
pub unsafe fn fn_80014240(this: usize, release: i32) -> usize {
    if this != 0 {
        // observed 0x8001425C..0x80014260.
        let _ = call2(FN_8015F800, this as u32, 0);
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Releases `this` when the pointer is non-null and `release` is positive.
///
/// Byte-identical to [`fn_80014200`]; the two addresses are separate inline
/// copies emitted for different translation units.
///
/// # Original
/// - Ghidra: `FUN_80014340` at `0x80014340` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014340_FUN_80014340.c`
///
/// # Arguments
/// * `this` - object pointer, returned unchanged.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` when both guards pass.
///
/// # Safety
/// A non-null `this` with positive `release` must be freeable.
///
/// # Confidence
/// high.
pub unsafe fn fn_80014340(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        let _ = call1(FN_802B9280, this as u32);
    }
    this
}

/// Registers the sub-objects selected by `mask` and records the mask in the
/// object's flag word.
///
/// Each bit of `mask` chooses one constructor-family call; the calls are made
/// in bit order. The final store is an OR, not an assignment: already-set
/// registration bits survive.
///
/// # Original
/// - Ghidra: `FUN_80014380` at `0x80014380` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014380_FUN_80014380.c`
///
/// # Arguments
/// * `this` - scene node owning the sub-objects at `+0x04`, `+0x1B4`,
///   `+0x28C`, `+0x2A8`, `+0x2C0..+0x2CC`, `+0x2D0`.
/// * `mask` - bit set of registration steps (`1,2,4,8,0x10,0x20,0x40,0x80,
///   0x100`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to nine registration calls, up to three byte stores from the
/// `0x804248CC` defaults, and `flags |= mask` at `+0x00`.
///
/// # Safety
/// `this` must be valid up to `+0x2D6` and the selected sub-objects must be
/// acceptable to their callees.
///
/// # Confidence
/// medium - the mask and call order are exact; the role of each sub-object is
/// inferred from its constructor/destructor pair.
pub unsafe fn fn_80014380(this: usize, mask: u32) {
    // observed 0x80014390..0x800143D8: bit 0, sub-object at +0x1B4.
    if (mask & 1) != 0 {
        let _ = call1(FN_801C3FE0, (this + 0x1B4) as u32);
    }
    // observed 0x800143DC..0x800143F4: bit 1, array at +0x04, selector 0.
    if (mask & 2) != 0 {
        let _ = call2(FN_801C4700, 0, (this + 4) as u32);
    }
    // observed 0x800143F8..0x80014410: bit 2, block at +0x28C.
    if (mask & 4) != 0 {
        let _ = call1(FN_801C9900, (this + 0x28C) as u32);
    }
    // observed 0x80014414..0x8001442C: bit 3, block at +0x2A8.
    if (mask & 8) != 0 {
        let _ = call1(FN_801C9C40, (this + 0x2A8) as u32);
    }
    // observed 0x80014430..0x80014448: bit 4, pointer at +0x2D0.
    if (mask & 0x10) != 0 {
        let _ = call1(FN_801C5890, (this + 0x2D0) as u32);
    }
    // observed 0x8001444C..0x80014474: bit 5, four words +0x2C0..+0x2CC.
    if (mask & 0x20) != 0 {
        let _ = call4(
            FN_801C9CD0,
            (this + 0x2C0) as u32,
            (this + 0x2C4) as u32,
            (this + 0x2C8) as u32,
            (this + 0x2CC) as u32,
        );
    }
    // observed 0x80014478..0x800144B8: bits 6..8 copy default bytes from the
    // SDA block at 0x804248CC into +0x2D4, +0x2D5 and +0x2D6.
    if (mask & 0x40) != 0 {
        let value = platform::read_u8(SDA_DEFAULT_FLAGS);
        platform::write_u8(this + 0x2D4, value);
    }
    if (mask & 0x80) != 0 {
        let value = platform::read_u8(SDA_DEFAULT_FLAGS + 1);
        platform::write_u8(this + 0x2D5, value);
    }
    if (mask & 0x100) != 0 {
        let value = platform::read_u8(SDA_DEFAULT_FLAGS + 2);
        platform::write_u8(this + 0x2D6, value);
    }
    // observed 0x800144C0..0x800144C8: `or` the mask into the flag word.
    let flags = platform::read_u32(this);
    platform::write_u32(this, flags | mask);
}

/// Unregisters the sub-objects selected by the flag word at `this + 0x00`,
/// then clears the flag word.
///
/// This is the teardown mirror of [`fn_80014380`]. The bits are processed in a
/// different order (`1,2,0x10,4,8,0x20,0x40,0x80,0x100`) and every call has a
/// matching applier or finaliser.
///
/// # Original
/// - Ghidra: `FUN_80014480` at `0x80014480` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014480_FUN_80014480.c`
///
/// # Arguments
/// * `this` - scene node whose flag word selects the live sub-objects.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to nine teardown calls, a six-float submission from `+0x2A8`, three
/// boolean finalisers from the bytes at `+0x2D4..+0x2D6`, and a zero store at
/// `+0x00`.
///
/// # Safety
/// `this` must be valid up to `+0x2D6`; every set bit must refer to a
/// sub-object that was constructed by [`fn_80014380`].
///
/// # Confidence
/// medium - control flow and operands are exact; the callee roles are inferred
/// from their names in the constructor/destructor pair.
pub unsafe fn fn_80014480(this: usize) {
    // observed: the instruction stream reloads `*this` before every test
    // (`lwz r0,0(r31)` at 0x80014494, 0x800144A8, 0x800144C0, ...), so a
    // callee that changes the flag word is observed by later branches.
    if (platform::read_u32(this) & 1) != 0 {
        let _ = call1(FN_801C3A30, (this + 0x1B4) as u32);
    }
    if (platform::read_u32(this) & 2) != 0 {
        let _ = call2(FN_801C4250, 0, (this + 4) as u32);
    }
    if (platform::read_u32(this) & 0x10) != 0 {
        let pointer = platform::read_u32(this + 0x2D0);
        let _ = call1(FN_801C5860, pointer);
    }
    if (platform::read_u32(this) & 4) != 0 {
        let _ = call1(FN_802D31D0, (this + 0x28C) as u32);
    }
    // observed 0x800144F8..0x80014534: six single-precision arguments.
    if (platform::read_u32(this) & 8) != 0 {
        call6_f32(
            FN_802D3200,
            read_f32(this + 0x2A8),
            read_f32(this + 0x2AC),
            read_f32(this + 0x2B0),
            read_f32(this + 0x2B4),
            read_f32(this + 0x2B8),
            read_f32(this + 0x2BC),
        );
    }
    // observed 0x80014538..0x80014550.
    if (platform::read_u32(this) & 0x20) != 0 {
        let _ = call4(
            FN_802D3230,
            platform::read_u32(this + 0x2C0),
            platform::read_u32(this + 0x2C4),
            platform::read_u32(this + 0x2C8),
            platform::read_u32(this + 0x2CC),
        );
    }
    // observed 0x80014554..0x800145A0: `-(u32)byte >> 31` is `byte != 0`.
    if (platform::read_u32(this) & 0x40) != 0 {
        let value = (platform::read_u8(this + 0x2D4) != 0) as u32;
        let _ = call1(FN_802D2F20, value);
    }
    if (platform::read_u32(this) & 0x80) != 0 {
        let value = (platform::read_u8(this + 0x2D5) != 0) as u32;
        let _ = call1(FN_802D2F80, value);
    }
    if (platform::read_u32(this) & 0x100) != 0 {
        let value = (platform::read_u8(this + 0x2D6) != 0) as u32;
        let _ = call1(FN_802D2FE0, value);
    }

    // observed 0x800145A4..0x800145AC.
    platform::write_u32(this, 0);
}

/// Releases `this` when the pointer is non-null and `release` is positive.
///
/// Byte-identical to [`fn_80014200`]; this third copy is emitted for the
/// translation unit that owns the scene-node registration pair.
///
/// # Original
/// - Ghidra: `FUN_800145F0` at `0x800145F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800145f0_FUN_800145f0.c`
///
/// # Arguments
/// * `this` - object pointer, returned unchanged.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` when both guards pass.
///
/// # Safety
/// A non-null `this` with positive `release` must be freeable.
///
/// # Confidence
/// high.
pub unsafe fn fn_800145f0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        let _ = call1(FN_802B9280, this as u32);
    }
    this
}

/// Returns `1` when the byte at `this + 0x0B` is non-zero, else `0`.
///
/// # Original
/// - Ghidra: `FUN_80014640` at `0x80014640` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014640_FUN_80014640.c`
///
/// # Arguments
/// * `this` - record with the flag byte at `+0x0B`.
///
/// # Returns
/// `0` or `1`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `this + 0x0B` must be readable.
///
/// # Confidence
/// high - the original is `lbz; neg; srwi 31`.
pub unsafe fn fn_80014640(this: usize) -> u32 {
    (platform::read_u8(this + 0x0B) != 0) as u32
}

/// Stores a three-float position into `this + 0x0C..+0x14`.
///
/// The decompiler shows `double` arguments because the ELF import models the
/// PowerPC floating registers as doubles. The instructions store single
/// precision (`stfs`), so the Rust body takes `f32`.
///
/// # Original
/// - Ghidra: `FUN_800146B0` at `0x800146B0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800146b0_FUN_800146b0.c`
///
/// # Arguments
/// * `x`, `y`, `z` - position components; the original prototype places them
///   first, in `f1..f3`.
/// * `this` - record to update.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three `f32` stores at `+0x0C`, `+0x10`, `+0x14`.
///
/// # Safety
/// `this` must be writable through `+0x17`.
///
/// # Confidence
/// high - the stores and offsets are explicit.
pub unsafe fn fn_800146b0(x: f32, y: f32, z: f32, this: usize) {
    write_f32(this + 0x0C, x);
    write_f32(this + 0x10, y);
    write_f32(this + 0x14, z);
}

/// Initialises one node record: state `2`, a float value, a `u16` id and a
/// cleared byte.
///
/// # Original
/// - Ghidra: `FUN_800146F0` at `0x800146F0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800146f0_FUN_800146f0.c`
///
/// # Arguments
/// * `value` - stored at `+0x18` (first source argument, `f1`).
/// * `this` - record to initialise.
/// * `id` - stored at `+0x1C` (third source argument, `r4`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Byte `+0x0A = 2`, `f32 +0x18`, `u16 +0x1C`, byte `+0x1E = 0`.
///
/// # Safety
/// `this` must be writable through `+0x1E`.
///
/// # Confidence
/// high.
pub unsafe fn fn_800146f0(value: f32, this: usize, id: u16) {
    platform::write_u8(this + 0x0A, 2);
    write_f32(this + 0x18, value);
    platform::write_u16(this + 0x1C, id);
    platform::write_u8(this + 0x1E, 0);
}

/// Returns the state-scaled float at `+0x18` for states 1 and 2, otherwise
/// `1.0`.
///
/// # Original
/// - Ghidra: `FUN_80014710` at `0x80014710` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014710_FUN_80014710.c`
///
/// # Arguments
/// * `this` - record; its state byte is read through `0x800146C0`.
///
/// # Returns
/// `*(f32 *)(this + 0x18)` when the state is 1 or 2, else the SDA constant
/// `1.0` at `0x8042B184`.
///
/// # Side effects
/// Calls `0x800146C0`; reads one float.
///
/// # Safety
/// `this + 0x18` must be readable when the state selects the value.
///
/// # Confidence
/// high for the control flow; medium for the meaning of the three states.
pub unsafe fn fn_80014710(this: usize) -> f32 {
    // observed 0x80014720: call with r3 still `this`.
    let state = call1(FN_800146C0, this as u32) as u8;
    if state == 1 || state == 2 {
        // observed: `lfs f1, 0x18(r31)`.
        read_f32(this + 0x18)
    } else {
        // observed: `lfs f1, -0x7EFC(r2)`.
        read_f32(SDA_ONE_F32)
    }
}

/// Converts the signed 16-bit angle at `+0x1C` to radians, or returns `0`.
///
/// The state comes from [`fn_800147a0`]; only states 2 and 3 carry an angle.
/// The original converts via the `0x43300000` double trick and then multiplies
/// by `pi/32768` in single precision.
///
/// # Original
/// - Ghidra: `FUN_80014750` at `0x80014750` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014750_FUN_80014750.c`
///
/// # Arguments
/// * `this` - record read by [`fn_800147a0`].
///
/// # Returns
/// `(angle as f32) * 0.0000958738`, or `0.0` for the other states.
///
/// # Side effects
/// Calls `0x800147A0`; no writes.
///
/// # Safety
/// `this` must be acceptable to `0x800147A0`.
///
/// # Confidence
/// high - the conversion constant at `0x8042B188` is `pi/32768` and the final
/// multiply is `fmuls`.
pub unsafe fn fn_80014750(this: usize) -> f32 {
    // observed 0x80014758: `bl 0x800147a0` with r3 = this.
    let raw = fn_800147a0(this);
    // observed: `extsh` then `oris ...,0x8000`, the standard signed-int to
    // double bit pattern; the final scaling is single precision.
    let angle = raw as i16;
    (angle as f32) * read_f32(SDA_ANGLE_SCALE)
}

/// Returns the signed 16-bit angle at `+0x1C` for states 2 and 3, else `0`.
///
/// # Original
/// - Ghidra: `FUN_800147A0` at `0x800147A0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800147a0_FUN_800147a0.c`
///
/// # Arguments
/// * `this` - record with the state byte and the angle at `+0x1C`.
///
/// # Returns
/// Sign-extended `i16` angle, or `0`.
///
/// # Side effects
/// Calls `0x800146C0`.
///
/// # Safety
/// `this + 0x1C` must be readable when the state matches.
///
/// # Confidence
/// high.
pub unsafe fn fn_800147a0(this: usize) -> i32 {
    let state = call1(FN_800146C0, this as u32) as u8;
    if state == 2 || state == 3 {
        // observed: `lha r3, 0x1C(r31)`.
        platform::read_u16(this + 0x1C) as i16 as i32
    } else {
        0
    }
}

/// Returns the byte at `+0x1E` for state 3, else `0`.
///
/// # Original
/// - Ghidra: `FUN_800147E0` at `0x800147E0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800147e0_FUN_800147e0.c`
///
/// # Arguments
/// * `this` - record with the state byte and the payload byte at `+0x1E`.
///
/// # Returns
/// The payload byte, or `0`.
///
/// # Side effects
/// Calls `0x800146C0`.
///
/// # Safety
/// `this + 0x1E` must be readable when the state is 3.
///
/// # Confidence
/// high.
pub unsafe fn fn_800147e0(this: usize) -> u8 {
    let state = call1(FN_800146C0, this as u32) as u8;
    if state == 3 {
        platform::read_u8(this + 0x1E)
    } else {
        0
    }
}

/// Appends one node to the tail of an intrusive doubly-linked list.
///
/// The list stores its count at `+0x00`, head at `+0x04` and tail at `+0x08`.
/// A node stores its head-ward link at `+0x00` and tail-ward link at `+0x04`.
///
/// # Original
/// - Ghidra: `FUN_80014820` at `0x80014820` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014820_FUN_80014820.c`
///
/// # Arguments
/// * `list` - list header (`count`, `head`, `tail`).
/// * `node` - node to link; its two pointer words are overwritten.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four pointer/count word writes.
///
/// # Safety
/// `list` must be valid through `+0x08`, `node` through `+0x07`, and `node`
/// must not already be linked into this list.
///
/// # Confidence
/// high - the branches and link updates are explicit.
pub unsafe fn fn_80014820(list: usize, node: usize) {
    let tail = platform::read_u32(list + 0x08) as usize;
    if tail == 0 {
        // observed 0x80014830..0x80014840: empty list, node is both ends.
        platform::write_u32(list + 0x08, node as u32);
        platform::write_u32(list + 0x04, node as u32);
    } else {
        // observed 0x80014844..0x80014858.
        platform::write_u32(node, tail as u32);
        platform::write_u32(tail + 0x04, node as u32);
        platform::write_u32(list + 0x08, node as u32);
    }
    // observed 0x8001485C..0x80014868.
    let count = platform::read_u32(list);
    platform::write_u32(list, count.wrapping_add(1));
}

/// Unlinks one node from an intrusive doubly-linked list.
///
/// The two neighbours are patched around `node`, the list head/tail are
/// repaired at the ends, and the count is decremented. The removed node's own
/// links are cleared where the original clears them (not both unconditionally).
///
/// # Original
/// - Ghidra: `FUN_80014860` at `0x80014860` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014860_FUN_80014860.c`
///
/// # Arguments
/// * `list` - list header.
/// * `node` - linked node to remove.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Up to five pointer writes plus the count decrement.
///
/// # Safety
/// `node` must be currently linked in `list`.
///
/// # Confidence
/// high.
pub unsafe fn fn_80014860(list: usize, node: usize) {
    let next = platform::read_u32(node) as usize;
    let prev = platform::read_u32(node + 0x04) as usize;

    if next == 0 {
        // observed 0x80014880: removing the head.
        platform::write_u32(list + 0x04, prev as u32);
    } else {
        platform::write_u32(next + 0x04, prev as u32);
        platform::write_u32(node, 0);
    }

    if prev == 0 {
        // observed 0x8001489C: removing the tail.
        platform::write_u32(list + 0x08, next as u32);
    } else {
        platform::write_u32(prev, next as u32);
        platform::write_u32(node + 0x04, 0);
    }

    // observed 0x800148A0..0x800148AC.
    let count = platform::read_u32(list);
    platform::write_u32(list, count.wrapping_sub(1));
}

/// Removes every node from an intrusive list.
///
/// The original loop loads the tail, calls [`fn_80014860`] with that node and
/// repeats until the tail word is zero.
///
/// # Original
/// - Ghidra: `FUN_800148B0` at `0x800148B0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800148b0_FUN_800148b0.c`
///
/// # Arguments
/// * `list` - list header to empty.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_80014860`] once per node; clears head, tail and count.
///
/// # Safety
/// Every linked node must be valid for the unlink writes.
///
/// # Confidence
/// high.
pub unsafe fn fn_800148b0(list: usize) {
    loop {
        // observed 0x800148D0..0x800148D8: load tail and test it.
        let tail = platform::read_u32(list + 0x08) as usize;
        if tail == 0 {
            break;
        }
        // observed 0x800148C8..0x800148CC: r4 already holds the tail.
        fn_80014860(list, tail);
    }
}

/// Destroys a list object: empties it when non-empty, then optionally frees it.
///
/// # Original
/// - Ghidra: `FUN_80014910` at `0x80014910` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014910_FUN_80014910.c`
///
/// # Arguments
/// * `this` - list object, returned unchanged.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls [`fn_800148b0`] when the count is non-zero and `0x802B9280(this)` when
/// `release > 0`.
///
/// # Safety
/// `this` must be a valid list header or zero; a positive `release` must be
/// freeable.
///
/// # Confidence
/// high.
pub unsafe fn fn_80014910(this: usize, release: i32) -> usize {
    if this != 0 {
        if platform::read_u32(this) != 0 {
            fn_800148b0(this);
        }
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Constructs an object of the class whose vtable is `0x802FF690`.
///
/// The body is the standard CodeWarrior constructor sequence: run the base
/// constructor, overwrite the vtable, clear one trailing word. The class name
/// is not recovered, and the layout is *not* proven to be the intrusive-list
/// header used by the preceding functions.
///
/// # Original
/// - Ghidra: `FUN_80014970` at `0x80014970` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80014970_FUN_80014970.c`
///
/// # Arguments
/// * `this` - object to construct.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x8016A010(this)`, then stores the vtable `0x802FF690` at `+0x00`
/// and clears the word at `+0x08`. (The base constructor already clears
/// `+0x04`.)
///
/// # Safety
/// `this` must be writable through `+0x08`.
///
/// # Confidence
/// high for the stores; low for the class identity (no RTTI name and no
/// caller in main.dol).
pub unsafe fn fn_80014970(this: usize) -> usize {
    // observed 0x80014984: Ghidra prints no argument, but r3 is still `this`.
    let _ = call1(FN_8016A010, this as u32);
    platform::write_u32(this, VTABLE_802FF690);
    platform::write_u32(this + 0x08, 0);
    this
}

// ===========================================================================
// 5. Per-frame node animation (0x800156B0, 0x80015C10)
// ===========================================================================

/// Updates a four-float interpolation record and its active byte.
///
/// When the state poll at `0x801DD1D0` reports a non-zero byte, the body reads
/// the global `u16` counter through `0x8042A480 + 0x06` and derives a
/// half-speed step between the record's two boundary floats. The four values
/// are always written, even on the zero-byte path (where they are zero).
///
/// # Original
/// - Ghidra: `FUN_800156B0` at `0x800156B0` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800156b0_FUN_800156b0.c`
///
/// # Arguments
/// * `this` - record with boundaries at `+0x1C4`/`+0x1C8` and output at
///   `+0x94..+0xA4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls the no-argument state poll; reads one global pointer and one `u16`;
/// writes four floats and one byte.
///
/// # Safety
/// `this` must be writable through `+0xA4`; the global pointer must be valid.
///
/// # Confidence
/// medium - the arithmetic is exact, but the Ghidra C inverts the branch and
/// the `+0xA4` boolean; this body follows the instruction stream.
pub unsafe fn fn_800156b0(this: usize) {
    let zero = read_f32(SDA_ZERO_F32);
    // observed 0x800156E0: call through the 0x800B54B0 thunk; no arguments.
    let active = call0_r(FN_801DD1D0) as u8 != 0;

    let mut local_24 = zero;
    let mut local_20 = zero;
    let mut local_1c = zero;
    let mut flag = 0u8;

    // observed 0x800156E8: `beq` skips the block when the poll byte is zero.
    if active {
        let counter_object = platform::read_u32(SDA_FRAME_COUNTER_PTR) as usize;
        let counter = platform::read_u16(counter_object + 0x06) as f32;
        local_1c = read_f32(this + 0x1C8);
        local_20 = read_f32(this + 0x1C4);
        // observed: 0.5 * (counter - this[+0x1C8]), computed in single
        // precision after the 0x43300000 integer conversion.
        local_24 = read_f32(SDA_HALF_F32) * (counter - read_f32(this + 0x1C8));
        flag = 1;
    }

    // observed 0x8001573C..0x80015760: common store block.
    write_f32(this + 0x94, zero);
    write_f32(this + 0x98, local_24);
    write_f32(this + 0x9C, local_20);
    write_f32(this + 0xA0, local_1c);
    platform::write_u8(this + 0xA4, flag);
}

/// Advances a coin-pane fade: step the scale, clamp the screen point and push
/// it into the pane's transform.
///
/// The body exits immediately when bit 0 of the pane's visibility byte at
/// `+0xBB` is clear. Otherwise it:
///
/// 1. compares the step at `+0x1A8` with zero;
/// 2. adds the step to the offset at `+0x1B8` and subtracts the decrement at
///    `+0x1B0` from the step;
/// 3. computes `x`/`y` from `+0x19C`/`+0x1A0` plus the offset, clamped by the
///    maximum at `+0x1F4`;
/// 4. passes the pair to `0x800B33D0` and writes the result to the pane at
///    `+0x2C`/`+0x30`/`+0x34`.
///
/// # Original
/// - Ghidra: `FUN_80015C10` at `0x80015C10` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80015c10_FUN_80015c10.c`
///
/// # Arguments
/// * `this` - HUD node with the pane pointer at `+0x1D4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800B33D0(point2)`; writes three floats to the pane and two to
/// the node.
///
/// # Safety
/// `this` must be valid through `+0x1F4`; `*(this + 0x1D4)` must point to a
/// pane whose `+0xBB` byte and `+0x2C..+0x34` floats are accessible.
///
/// # Confidence
/// high for the flow and offsets; medium for the projection role of
/// `0x800B33D0`.
pub unsafe fn fn_80015c10(this: usize) {
    let pane = platform::read_u32(this + 0x1D4) as usize;
    // observed 0x80015C1C..0x80015C2C: `lbz +0xBB; rlwinm 0,31,31; beq`.
    if (platform::read_u8(pane + 0xBB) & 1) == 0 {
        return;
    }

    let zero = read_f32(SDA_ZERO_F32);
    let mut step = read_f32(this + 0x1A8);
    // observed 0x80015C34..0x80015C5C.
    if zero < step {
        let offset = read_f32(this + 0x1B8);
        write_f32(this + 0x1B8, offset + step);
        let decrement = read_f32(this + 0x1B0);
        write_f32(this + 0x1A8, step - decrement);
    } else {
        write_f32(this + 0x1A8, zero);
    }

    // observed 0x80015C60..0x80015C80: build the 2-float point.
    let mut point = crate::platform::GuestLocal::new( [0.0f32; 2]);
    point[0] = read_f32(this + 0x19C);
    point[1] = read_f32(this + 0x1A0) + read_f32(this + 0x1B8);
    let maximum = read_f32(this + 0x1F4);
    if maximum <= point[1] {
        point[1] = maximum;
    }

    // observed 0x80015C84..0x80015C88.
    let _ = call1(FN_800B33D0, point.as_mut_ptr() as u32);

    // observed 0x80015C8C..0x80015CB8: reload the pane pointer, then store.
    let pane = platform::read_u32(this + 0x1D4) as usize;
    write_f32(pane + 0x2C, point[0]);
    write_f32(pane + 0x30, point[1]);
    write_f32(pane + 0x34, zero);
}

// ===========================================================================
// 6. Scene-node list registration (0x800190F0..0x80019380)
// ===========================================================================

/// Inserts a scene node into the global sorted list when its enable byte is
/// set.
///
/// The enable byte is `+0x1E5`; the list link sits at `+0x04`. The callee
/// `0x80163850` inserts by the sort key at `link + 0x0C`.
///
/// # Original
/// - Ghidra: `FUN_800190F0` at `0x800190F0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800190f0_FUN_800190f0.c`
///
/// # Arguments
/// * `this` - scene node.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80163850(this + 4)` when `+0x1E5` is non-zero.
///
/// # Safety
/// `this + 4` must be a valid list link when the byte is set.
///
/// # Confidence
/// high for the guard and call; medium for the exact enable/disable meaning
/// of `+0x1E5`.
pub unsafe fn fn_800190f0(this: usize) {
    // observed 0x800190F0..0x80019100: `lbz +0x1E5; beqlr`.
    if platform::read_u8(this + 0x1E5) == 0 {
        return;
    }
    // observed: `addi r3, r3, 4; b 0x80006EA0` (thunk to 0x80163850).
    let _ = call1(FN_80163850, (this + 4) as u32);
}

/// Marks a scene node as registered and stores its list link value.
///
/// # Original
/// - Ghidra: `FUN_80019360` at `0x80019360` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019360_FUN_80019360.c`
///
/// # Arguments
/// * `this` - scene node.
/// * `link` - word stored at `+0x1E8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `u32 +0x1E8 = link`, byte `+0x1E5 = 1`, byte `+0x1E6 = 1`.
///
/// # Safety
/// `this` must be writable through `+0x1EB`.
///
/// # Confidence
/// high.
pub unsafe fn fn_80019360(this: usize, link: u32) {
    platform::write_u32(this + 0x1E8, link);
    platform::write_u8(this + 0x1E5, 1);
    platform::write_u8(this + 0x1E6, 1);
}

/// Sets the two pending-update bytes of a scene node.
///
/// # Original
/// - Ghidra: `FUN_80019380` at `0x80019380` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019380_FUN_80019380.c`
///
/// # Arguments
/// * `this` - scene node.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Byte `+0x1E6 = 1`, byte `+0x1E7 = 1`.
///
/// # Safety
/// `this` must be writable through `+0x1E7`.
///
/// # Confidence
/// high.
pub unsafe fn fn_80019380(this: usize) {
    platform::write_u8(this + 0x1E6, 1);
    platform::write_u8(this + 0x1E7, 1);
}

// ===========================================================================
// 7. Actor hit dispatch (0x8001E0B0, 0x8001E100)
// ===========================================================================

/// Accepts a hit when the hit record has kind `3` and the actor's virtual
/// slot `+0x158` reports success.
///
/// `param_2` is part of the original three-argument shape but is never read.
///
/// # Original
/// - Ghidra: `FUN_8001E0B0` at `0x8001E0B0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e0b0_FUN_8001e0b0.c`
///
/// # Arguments
/// * `this` - actor; its vtable is at `+0x60`.
/// * `_param_2` - unused by the original body.
/// * `hit` - hit record; byte `+0x2D` is its kind.
///
/// # Returns
/// `1` when the kind is `3` and the virtual call returns non-zero, else `0`.
///
/// # Side effects
/// One virtual call through `vtable + 0x158` with `(this, hit)`.
///
/// # Safety
/// `this + 0x60` and `hit + 0x2D` must be readable; the vtable slot must be a
/// valid function.
///
/// # Confidence
/// high for the dispatch; medium for the meaning of kind `3`.
pub unsafe fn fn_8001e0b0(this: usize, _param_2: u32, hit: usize) -> u32 {
    // observed 0x8001E0C0..0x8001E0CC.
    if platform::read_u8(hit + 0x2D) == 3 {
        let vtable = platform::read_u32(this + 0x60) as usize;
        let entry = platform::read_u32(vtable + 0x158) as usize;
        // observed: `bctrl` with r3 = this and r4 = hit.
        if call2(entry, this as u32, hit as u32) != 0 {
            return 1;
        }
    }
    0
}

/// Accepts a hit when the hit record has kind `3` and the actor's virtual
/// slot `+0x158` reports success.
///
/// Byte-identical to [`fn_8001e0b0`]; the second copy belongs to a different
/// actor class translation unit.
///
/// # Original
/// - Ghidra: `FUN_8001E100` at `0x8001E100` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e100_FUN_8001e100.c`
///
/// # Arguments
/// * `this` - actor; its vtable is at `+0x60`.
/// * `_param_2` - unused by the original body.
/// * `hit` - hit record; byte `+0x2D` is its kind.
///
/// # Returns
/// `1` when the kind is `3` and the virtual call returns non-zero, else `0`.
///
/// # Side effects
/// One virtual call through `vtable + 0x158` with `(this, hit)`.
///
/// # Safety
/// `this + 0x60` and `hit + 0x2D` must be readable; the vtable slot must be a
/// valid function.
///
/// # Confidence
/// high for the dispatch; medium for the meaning of kind `3`.
pub unsafe fn fn_8001e100(this: usize, _param_2: u32, hit: usize) -> u32 {
    if platform::read_u8(hit + 0x2D) == 3 {
        let vtable = platform::read_u32(this + 0x60) as usize;
        let entry = platform::read_u32(vtable + 0x158) as usize;
        if call2(entry, this as u32, hit as u32) != 0 {
            return 1;
        }
    }
    0
}

// ===========================================================================
// 8. Warning-scene teardown helpers (0x80017060..0x80018EA0)
//
// These are the emitted destructor/release helpers of the error, nunchuk and
// classic-controller warning scenes. Each takes `(this, release)` and frees
// `this` only when `release > 0`; the bigger ones first release a sub-object
// or clear a pane group. The second argument of `0x8015F9F0` is always zero,
// so that call only performs its non-null guard in this image.
// ===========================================================================

/// Releases a warning-scene object when the signed release flag is positive.
///
/// # Original
/// - Ghidra: `FUN_80017060` at `0x80017060` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017060_FUN_80017060.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Calls the allocator's free (`0x802B9280`) when `this != 0 && release > 0`.
///
/// # Safety
/// `this` must be a live allocation when it is non-null.
///
/// # Confidence
/// high - the guards and the free call match the instruction stream.
pub unsafe fn fn_80017060(this: usize, release: i32) -> usize {
    // observed 0x80017060: `cmpwi r3,0; beq`; then `cmpwi r4,0; ble`.
    if this != 0 && release > 0 {
        // observed: r3 still holds `this` at the call.
        let _ = call1(FN_802B9280, this as u32);
    }
    this
}

/// Releases a warning-scene object after offering its `+0x18` sub-object to
/// the guarded release helper.
///
/// # Original
/// - Ghidra: `FUN_800170A0` at `0x800170A0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800170a0_FUN_800170a0.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Calls `0x8015F9F0(this + 0x18, 0)` (a no-op release in this image) and
/// frees `this` when `release > 0`.
///
/// # Safety
/// `this + 0x18` must be readable when `this` is non-null.
///
/// # Confidence
/// high.
pub unsafe fn fn_800170a0(this: usize, release: i32) -> usize {
    if this != 0 {
        // observed 0x800170C4: `addic. r3, r3, 0x18` - the sub-object guard.
        let sub = this + 0x18;
        if sub != 0 {
            let _ = call2(FN_8015F9F0, sub as u32, 0);
        }
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Releases the error-window object: guarded `+0x1B8` sub-object, pane-group
/// cleanup at `+0x08`, then the optional free.
///
/// # Original
/// - Ghidra: `FUN_80017170` at `0x80017170` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017170_FUN_80017170.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Calls `0x8015F9F0(this + 0x1B8, 0)`, `0x800C89A0(this + 8, -1)`, and
/// frees `this` when `release > 0`.
///
/// # Safety
/// `this` must be valid through `+0x1BC` when non-null.
///
/// # Confidence
/// high for the flow; medium for the exact meaning of the pane-group block.
pub unsafe fn fn_80017170(this: usize, release: i32) -> usize {
    if this != 0 {
        // observed 0x80017194: `addic. r0, r3, 0x1A0` (guard on this + 0x1A0).
        let guard = this + 0x1A0;
        if guard != 0 {
            let sub = this + 0x1B8;
            if sub != 0 {
                let _ = call2(FN_8015F9F0, sub as u32, 0);
            }
        }
        // observed 0x800171B0: `addi r3, r30, 8; li r4, -1`.
        let _ = call2(FN_800C89A0, (this + 8) as u32, 0xFFFF_FFFF);
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Releases a nunchuk/classic warning object after its base release helper.
///
/// # Original
/// - Ghidra: `FUN_80018BB0` at `0x80018BB0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80018bb0_FUN_80018bb0.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Calls `0x8015F800(this, 0)` for every non-null object, then frees `this`
/// when `release > 0`.
///
/// # Safety
/// `this` must be a live allocation when non-null.
///
/// # Confidence
/// high.
pub unsafe fn fn_80018bb0(this: usize, release: i32) -> usize {
    if this != 0 {
        // observed 0x80018BD4: `li r4,0; bl 0x8015F800`.
        let _ = call2(FN_8015F800, this as u32, 0);
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Releases a warning object when the signed release flag is positive.
///
/// This is the minimal form used by the nunchuk scene; it has no sub-object
/// teardown step.
///
/// # Original
/// - Ghidra: `FUN_80018D50` at `0x80018D50` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80018d50_FUN_80018d50.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Frees `this` when `this != 0 && release > 0`.
///
/// # Safety
/// `this` must be a live allocation when non-null.
///
/// # Confidence
/// high.
pub unsafe fn fn_80018d50(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        // observed 0x80018D74: r3 still holds `this`.
        let _ = call1(FN_802B9280, this as u32);
    }
    this
}

/// Releases a warning object after offering its `+0x18` sub-object to the
/// guarded release helper.
///
/// # Original
/// - Ghidra: `FUN_80018DD0` at `0x80018DD0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80018dd0_FUN_80018dd0.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Calls `0x8015F9F0(this + 0x18, 0)` and frees `this` when `release > 0`.
///
/// # Safety
/// `this + 0x18` must be readable when `this` is non-null.
///
/// # Confidence
/// high.
pub unsafe fn fn_80018dd0(this: usize, release: i32) -> usize {
    if this != 0 {
        let sub = this + 0x18;
        if sub != 0 {
            let _ = call2(FN_8015F9F0, sub as u32, 0);
        }
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Releases a warning object: guarded `+0x1B4` sub-object, pane-group cleanup
/// at `+0x04`, then the optional free.
///
/// # Original
/// - Ghidra: `FUN_80018EA0` at `0x80018EA0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80018ea0_FUN_80018ea0.c`
///
/// # Arguments
/// * `this` - object to release; may be null.
/// * `release` - signed release flag.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// Calls `0x8015F9F0(this + 0x1B4, 0)`, `0x800C89A0(this + 4, -1)`, and
/// frees `this` when `release > 0`.
///
/// # Safety
/// `this` must be valid through `+0x1B8` when non-null.
///
/// # Confidence
/// high for the flow; medium for the pane-group role.
pub unsafe fn fn_80018ea0(this: usize, release: i32) -> usize {
    if this != 0 {
        // observed 0x80018EC4: `addic. r0, r3, 0x19C` (guard on this + 0x19C).
        let guard = this + 0x19C;
        if guard != 0 {
            let sub = this + 0x1B4;
            if sub != 0 {
                let _ = call2(FN_8015F9F0, sub as u32, 0);
            }
        }
        // observed 0x80018EE0: `addi r3, r30, 4; li r4, -1`.
        let _ = call2(FN_800C89A0, (this + 4) as u32, 0xFFFF_FFFF);
        if release > 0 {
            let _ = call1(FN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 9. Warning-scene state steps (0x80017BB0..0x80017FD0)
// ===========================================================================

/// Chooses the error-window state from the active player's buttons and
/// signals a transition through the scene vtable.
///
/// When the "force" byte `+0x22F` is zero, the body reads the active player's
/// controller record and maps bit 3 of its `+0x1C` word to state `1` and bit 2
/// to state `0`. A changed state is recorded at `+0x224` (with the old value
/// at `+0x228`), plays sound `0x78` and sends event `0x803520F8`.
///
/// The path taken when the force byte is set (or the state did not change)
/// asks the global allocation interface for `0x900` bytes; on success it
/// records the previous state, sends event `0x80352178`, and enables vibration
/// for state `3` when the global lock is clear.
///
/// # Original
/// - Ghidra: `FUN_80017BB0` at `0x80017BB0` (292 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017bb0_FUN_80017bb0.c`
///
/// # Arguments
/// * `this` - error-window object; state at `+0x224`, previous at `+0x228`,
///   flags at `+0x22F`, event target at `+0x1A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x224`/`+0x228`; calls `0x80195380`, vtable slot `+0x18`, the
/// allocator slot `+0x20`, `0x80107F10` and `0x80108160`.
///
/// # Safety
/// `this` must be valid through `+0x22F`; the controller table, allocator
/// pointer and sound handle globals must be initialised.
///
/// # Confidence
/// medium - the field roles are inferred from the state transitions; the
/// three-load pointer chain at `0x80017BE4..0x80017BEC` follows the
/// instruction stream (Ghidra prints one indirection fewer).
pub unsafe fn fn_80017bb0(this: usize) {
    let current = platform::read_i32(this + 0x224);

    // observed 0x80017BC4..0x80017BD8: `lbz +0x22F; beq` into the check block.
    if platform::read_u8(this + 0x22F) == 0 {
        let mut next = current;
        let table = platform::read_u32(SDA_CONTROLLER_TABLE) as usize;
        let index = platform::read_i32(SDA_ACTIVE_PLAYER) as usize;
        // observed 0x80017BE4..0x80017BEC: `lwzx` (slot pointer), `lwz +4`
        // (controller record pointer), then `lwz +0x1C` (button word).
        let slot = platform::read_u32(table + index * 4) as usize;
        let record = platform::read_u32(slot + 4) as usize;
        let flags = platform::read_u32(record + 0x1C);
        if flags & 8 != 0 {
            next = 1;
        }
        if flags & 4 != 0 {
            next = 0;
        }

        if next != current {
            platform::write_i32(this + 0x228, current);
            platform::write_i32(this + 0x224, next);
            let _ = call3(
                FN_80195380,
                platform::read_u32(SDA_WARNING_SOUND_HANDLE),
                0x78,
                1,
            );
            let vtable = platform::read_u32(this + 0x1A0) as usize;
            let notify = platform::read_u32(vtable + 0x18) as usize;
            let _ = call2(notify, (this + 0x1A0) as u32, EVENT_803520F8);
            return;
        }
    }

    // observed 0x80017C58: allocator interface slot +0x20, size 0x900.
    let allocator = platform::read_u32(SDA_WARNING_ALLOCATOR) as usize;
    let allocator_vtable = platform::read_u32(allocator) as usize;
    let allocate = platform::read_u32(allocator_vtable + 0x20) as usize;
    if call2(allocate, allocator as u32, 0x900) != 0 {
        platform::write_i32(this + 0x228, platform::read_i32(this + 0x224));
        let vtable = platform::read_u32(this + 0x1A0) as usize;
        let notify = platform::read_u32(vtable + 0x18) as usize;
        let _ = call2(notify, (this + 0x1A0) as u32, EVENT_80352178);

        // observed 0x80017C9C..0x80017CB0: u16 lock == 0 and state == 3.
        if platform::read_u16(SDA_VIBRATION_LOCK) == 0
            && platform::read_i32(this + 0x220) == 3
        {
            let handler = call0_r(FN_80107F10);
            let _ = call2(FN_80108160, handler, 1);
        }
    }
}

/// Selects the error-window animation from the previous state and restarts
/// the pane group.
///
/// # Original
/// - Ghidra: `FUN_80017D00` at `0x80017D00` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017d00_FUN_80017d00.c`
///
/// # Arguments
/// * `this` - error-window object; previous state at `+0x228`, force byte at
///   `+0x22F`, pane group at `+0x08`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8010C780(global 0x8042A2C8, 0)` and plays animation
/// `previous + 7` or `9` through `0x800C9490`.
///
/// # Safety
/// `this + 0x08` must be an initialised pane group.
///
/// # Confidence
/// high for the flow; medium for the animation indices' meaning.
pub unsafe fn fn_80017d00(this: usize) {
    // observed 0x80017D18: `lwz r3, -0x53B8(r13); li r4, 0`.
    let _ = call2(
        FN_8010C780,
        platform::read_u32(SDA_HUD_OWNER),
        0,
    );

    // observed 0x80017D20..0x80017D48: byte +0x22F selects 9 or previous + 7.
    let animation = if platform::read_u8(this + 0x22F) == 0 {
        platform::read_i32(this + 0x228) + 7
    } else {
        9
    };
    let _ = call2(FN_800C9490, (this + 8) as u32, animation as u32);
}

/// Asks the pane-group owner for `0x900` bytes and sends the "finished" event
/// when the request succeeds.
///
/// # Original
/// - Ghidra: `FUN_80017EE0` at `0x80017EE0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017ee0_FUN_80017ee0.c`
///
/// # Arguments
/// * `this` - error-window object; owner index at `+0x218`, event target at
///   `+0x1A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls vtable slot `+0x20` of the owner selected from the global table
/// `0x80377C88`; on success calls vtable slot `+0x18` with event
/// `0x803521F8`.
///
/// # Safety
/// The global table and the selected owner must be valid.
///
/// # Confidence
/// medium - the owner table's element type is inferred from the `lwzx`.
pub unsafe fn fn_80017ee0(this: usize) {
    let index = platform::read_i32(this + 0x218) as usize;
    let owner = platform::read_u32(SDA_PANE_GROUP_TABLE + index * 4) as usize;
    let vtable = platform::read_u32(owner) as usize;
    let allocate = platform::read_u32(vtable + 0x20) as usize;
    let allocated = call2(allocate, owner as u32, 0x900);
    if allocated != 0 {
        let state_vtable = platform::read_u32(this + 0x1A0) as usize;
        let notify = platform::read_u32(state_vtable + 0x18) as usize;
        let _ = call2(notify, (this + 0x1A0) as u32, EVENT_803521F8);
    }
}

/// Starts the error-window close animation and arms a 30-frame timer.
///
/// # Original
/// - Ghidra: `FUN_80017F70` at `0x80017F70` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017f70_FUN_80017f70.c`
///
/// # Arguments
/// * `this` - error-window object; pane group at `+0x08`, timer at `+0x21C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Plays sound `0x81`, starts animation `0x11`, stops slot `0x12`, and stores
/// `0x1E` at `+0x21C`.
///
/// # Safety
/// The pane group and the global sound handle must be valid.
///
/// # Confidence
/// high.
pub unsafe fn fn_80017f70(this: usize) {
    let _ = call3(
        FN_80195380,
        platform::read_u32(SDA_WARNING_SOUND_HANDLE),
        0x81,
        1,
    );
    let _ = call2(FN_800C9490, (this + 8) as u32, 0x11);
    let _ = call3(FN_800C92F0, (this + 8) as u32, 0x12, 0);
    platform::write_i32(this + 0x21C, 0x1E);
}

/// Counts down the close timer and sends the "closed" event when it reaches
/// zero and the pane animation has finished.
///
/// # Original
/// - Ghidra: `FUN_80017FD0` at `0x80017FD0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80017fd0_FUN_80017fd0.c`
///
/// # Arguments
/// * `this` - error-window object; timer at `+0x21C`, pane group at `+0x08`,
///   event target at `+0x1A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the decremented timer or calls vtable slot `+0x18` with event
/// `0x80352238`.
///
/// # Safety
/// The pane group and event target must be valid.
///
/// # Confidence
/// high.
pub unsafe fn fn_80017fd0(this: usize) {
    if call2(FN_800C9610, (this + 8) as u32, 0xFFFF_FFFF) != 0 {
        return;
    }
    let timer = platform::read_i32(this + 0x21C);
    if timer == 0 {
        let vtable = platform::read_u32(this + 0x1A0) as usize;
        let notify = platform::read_u32(vtable + 0x18) as usize;
        let _ = call2(notify, (this + 0x1A0) as u32, EVENT_80352238);
    } else {
        platform::write_i32(this + 0x21C, timer - 1);
    }
}

// ===========================================================================
// 10. Scene-node list and pane helpers (0x80019080..0x80019390)
// ===========================================================================

/// Removes a registered scene node from its pane group and both lists.
///
/// The body runs only when the enable byte `+0x1E5` is set. It calls the
/// scene vtable slot `+0x10` with `this`, detaches the pane group at
/// `this + 4` through `0x800C9560`, then calls the pane vtable slot `+0x10`
/// with `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_80019080` at `0x80019080` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019080_FUN_80019080.c`
///
/// # Arguments
/// * `this` - scene node; vtable at `+0x19C`, pane vtable at `+0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two virtual calls plus `0x800C9560(this + 4)`.
///
/// # Safety
/// `this + 4`, `+0x0C` and `+0x19C` must be valid when the byte is set.
///
/// # Confidence
/// medium - the virtual slot roles are inferred; `r3` for the first call is
/// `this` (Ghidra omits the argument).
pub unsafe fn fn_80019080(this: usize) {
    // observed 0x80019094: `lbz +0x1E5; beq`.
    if platform::read_u8(this + 0x1E5) != 0 {
        let scene_vtable = platform::read_u32(this + 0x19C) as usize;
        let detach_scene = platform::read_u32(scene_vtable + 0x10) as usize;
        // observed 0x800190A0..0x800190AC: r3 is still `this` at `bctrl`.
        let _ = call1(detach_scene, this as u32);

        let _ = call1(FN_800C9560, (this + 4) as u32);

        let pane_vtable = platform::read_u32(this + 0x0C) as usize;
        let detach_pane = platform::read_u32(pane_vtable + 0x10) as usize;
        let _ = call1(detach_pane, (this + 4) as u32);
    }
}

/// Registers a scene node with the pane manager and sends the pane event.
///
/// # Original
/// - Ghidra: `FUN_80019140` at `0x80019140` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019140_FUN_80019140.c`
///
/// # Arguments
/// * `this` - scene node; pane owner at `+0x1DC`, link word at `+0x1E8`,
///   scene vtable at `+0x19C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800CDC60`, `0x800C9A60(owner, time, 0x68, link + 4, 0)` and the
/// scene vtable slot `+0x18` with event `0x80352338`.
///
/// # Safety
/// `this + 0x1E8` and the pane owner must be valid.
///
/// # Confidence
/// medium - the `0x68` selector and event object are recorded but not named.
pub unsafe fn fn_80019140(this: usize) {
    // observed 0x80019154: `bl 0x800CDC60`.
    let time = call0_r(FN_800CDC60);
    let link = platform::read_u32(this + 0x1E8) as usize;
    // observed 0x80019158..0x80019174: five integer arguments.
    let _ = call5(
        FN_800C9A60,
        platform::read_u32(this + 0x1DC),
        time,
        0x68,
        (link + 4) as u32,
        0,
    );

    let vtable = platform::read_u32(this + 0x19C) as usize;
    let notify = platform::read_u32(vtable + 0x18) as usize;
    let _ = call2(notify, (this + 0x19C) as u32, EVENT_80352338);
}

/// Sends the nunchuk close event once the pane animation has finished.
///
/// # Original
/// - Ghidra: `FUN_80019200` at `0x80019200` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019200_FUN_80019200.c`
///
/// # Arguments
/// * `this` - warning object; pane group at `+0x04`, event target at `+0x19C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls vtable slot `+0x18` with event `0x80352378` when
/// `0x800C9610(this + 4, -1)` returns zero.
///
/// # Safety
/// The pane group and event target must be valid.
///
/// # Confidence
/// high.
pub unsafe fn fn_80019200(this: usize) {
    if call2(FN_800C9610, (this + 4) as u32, 0xFFFF_FFFF) != 0 {
        return;
    }
    let vtable = platform::read_u32(this + 0x19C) as usize;
    let notify = platform::read_u32(vtable + 0x18) as usize;
    let _ = call2(notify, (this + 0x19C) as u32, EVENT_80352378);
}

/// Sends the nunchuk start event once the pane animation has finished.
///
/// # Original
/// - Ghidra: `FUN_800192F0` at `0x800192F0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800192f0_FUN_800192f0.c`
///
/// # Arguments
/// * `this` - warning object; pane group at `+0x04`, event target at `+0x19C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls vtable slot `+0x18` with event `0x803522F8` when
/// `0x800C9610(this + 4, -1)` returns zero.
///
/// # Safety
/// The pane group and event target must be valid.
///
/// # Confidence
/// high.
pub unsafe fn fn_800192f0(this: usize) {
    if call2(FN_800C9610, (this + 4) as u32, 0xFFFF_FFFF) != 0 {
        return;
    }
    let vtable = platform::read_u32(this + 0x19C) as usize;
    let notify = platform::read_u32(vtable + 0x18) as usize;
    let _ = call2(notify, (this + 0x19C) as u32, EVENT_803522F8);
}

/// Attaches an interface object to a scene node when the interface accepts
/// the request.
///
/// The virtual slot `+0x0C` of `interface` is called with `interface`; when it
/// returns zero the interface is stored at `this + 0x0C` and `this + 4` is
/// returned, otherwise the function returns null.
///
/// # Original
/// - Ghidra: `FUN_80019390` at `0x80019390` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019390_FUN_80019390.c`
///
/// # Arguments
/// * `this` - scene node.
/// * `interface` - candidate interface object with a vtable at `+0x00`.
///
/// # Returns
/// `this + 4` on success, `0` when the interface rejects.
///
/// # Side effects
/// Writes the interface pointer at `this + 0x0C`; one virtual call.
///
/// # Safety
/// `interface` must have a valid vtable with a callable slot `+0x0C`.
///
/// # Confidence
/// medium - the interface role is inferred from the call ordering.
pub unsafe fn fn_80019390(this: usize, interface: usize) -> usize {
    let vtable = platform::read_u32(interface) as usize;
    let accept = platform::read_u32(vtable + 0x0C) as usize;
    if call1(accept, interface as u32) == 0 {
        platform::write_u32(this + 0x0C, interface as u32);
        this + 4
    } else {
        0
    }
}

// ===========================================================================
// 11. Classic-controller warning scene construction (0x80019990, 0x80019C10)
// ===========================================================================

/// Constructs the classic-controller warning scene object.
///
/// The constructor installs the base vtable `0x80300B30`, builds the pane
/// helper at `+0x04`, fills the state block at `+0x19C..+0x1B0`, constructs
/// the sub-object at `+0x1B4`, clears the loaded byte and the link word, and
/// replaces the state vtable with the "constructed" one `0x80300B40`.
///
/// # Original
/// - Ghidra: `FUN_80019990` at `0x80019990` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019990_FUN_80019990.c`
///
/// # Arguments
/// * `this` - object to construct.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Installs six vtables/pointers, calls `0x800C8950(this + 4)` and
/// `0x8015FC90(this + 0x1B4, this + 0x1A0, this + 0x1A4, 0x803523F8)`, and
/// writes bytes `+0x1E4` and words `+0x1E8`.
///
/// # Safety
/// `this` must be writable through `+0x1EC`.
///
/// # Confidence
/// high for the stores; medium for the class identity (no RTTI name).
pub unsafe fn fn_80019990(this: usize) -> usize {
    // observed 0x800199B0..0x800199B8.
    platform::write_u32(this, VT_80300B30);
    let _ = call1(FN_800C8950, (this + 4) as u32);

    // observed 0x800199E4..0x80019A08: four vtable/pointer words, self and 0.
    platform::write_u32(this + 0x19C, VT_80300B70);
    platform::write_u32(this + 0x1A0, VT_802FEAF8);
    platform::write_u32(this + 0x1A4, VT_80300BA0);
    platform::write_u32(this + 0x1A8, VT_80300BB8);
    platform::write_u32(this + 0x1AC, this as u32);
    platform::write_u32(this + 0x1B0, 0);

    // observed 0x80019A0C: four arguments, the last is the BSS event object.
    let _ = call4(
        FN_8015FC90,
        (this + 0x1B4) as u32,
        (this + 0x1A0) as u32,
        (this + 0x1A4) as u32,
        EVENT_803523F8,
    );

    // observed 0x80019A10..0x80019A24.
    platform::write_u8(this + 0x1E4, 0);
    platform::write_u32(this + 0x19C, VT_80300B40);
    platform::write_u32(this + 0x1E8, 0);
    this
}

/// Builds the classic-controller warning scene: loads the archive and layout,
/// binds three pane sets, resolves the root and `T_player` panes, and marks
/// the scene loaded.
///
/// The body returns `1` when the loaded byte `+0x1E4` is already set, `0` when
/// the archive cannot be bound, and `1` after a successful build. On success
/// it also clears bit 0 of the root pane's visibility byte `+0xBB` and clears
/// `+0x1E5`/`+0x1E7`.
///
/// # Original
/// - Ghidra: `FUN_80019C10` at `0x80019C10` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80019c10_FUN_80019c10.c`
///
/// # Arguments
/// * `this` - classic-controller scene object; pane group at `+0x04`,
///   loaded byte at `+0x1E4`, root pane at `+0x1D8`.
///
/// # Returns
/// `1` when loaded/ready, `0` when the archive is missing.
///
/// # Side effects
/// Binds `extensionControllerClasic/extensionControllerClasic.arc` and
/// `extensionControllerClasic_05.brlyt`, calls the pane helpers
/// `0x800C8FB0`/`0x800C90F0`/`0x800C8F20`/`0x800C8DD0`/`0x800C93D0`/
/// `0x800C9560`/`0x800C9500`, two vtable slots (`+0x14`, `+0x10`), and writes
/// the pane and scene fields.
///
/// # Safety
/// `this` must be writable through `+0x1E7`; the archive path must exist.
///
/// # Confidence
/// high for the call sequence and string tables; medium for the meaning of
/// the `0x68`/`0x97` constants and the pane tables.
pub unsafe fn fn_80019c10(this: usize) -> u32 {
    // observed 0x80019C30: `lbz +0x1E4; beq` → return 1 when already loaded.
    if platform::read_u8(this + 0x1E4) != 0 {
        return 1;
    }

    // observed 0x80019C44..0x80019C50: (group, arc path, 0).
    let archive = call3(
        FN_800C8CB0,
        (this + 4) as u32,
        ARC_CLASSIC_CONTROLLER,
        0,
    );
    if archive == 0 {
        return 0;
    }

    // observed 0x80019C64..0x80019C7C: pane vtable slot +0x14, brlyt + 0.
    let layout_vtable = platform::read_u32(this + 0x0C) as usize;
    let bind_layout = platform::read_u32(layout_vtable + 0x14) as usize;
    let _ = call3(
        bind_layout,
        (this + 4) as u32,
        BRLYT_CLASSIC_CONTROLLER,
        0,
    );

    // observed 0x80019C80..0x80019CA4: two pane tables and the data table.
    let _ = call3(FN_800C8FB0, (this + 4) as u32, PANES_CLASSIC_3, 3);
    let _ = call4(
        FN_800C90F0,
        (this + 4) as u32,
        PANES_CLASSIC_3B,
        DATA_802EE018,
        3,
    );

    // observed 0x80019CA8..0x80019CC0: root pane and T_player lookup.
    let root = call1(FN_80007220, (this + 4) as u32);
    platform::write_u32(this + 0x1D8, root);
    let player = call2(FN_800C8A90, (this + 4) as u32, NAME_T_PLAYER);
    platform::write_u32(this + 0x1DC, player);

    // observed 0x80019CC4..0x80019CEC.
    let _ = call5(
        FN_800C8F20,
        (this + 4) as u32,
        TABLE_80427790,
        TABLE_8042B1D8,
        0x68,
        2,
    );
    let _ = call4(
        FN_800C8DD0,
        (this + 4) as u32,
        TABLE_80427798,
        (this + 0x1E0) as u32,
        1,
    );

    // observed 0x80019CF0..0x80019D20.
    let _ = call3(FN_800C93D0, (this + 4) as u32, 0, 0);
    let _ = call1(FN_800C9560, (this + 4) as u32);
    let commit_vtable = platform::read_u32(this + 0x0C) as usize;
    let commit = platform::read_u32(commit_vtable + 0x10) as usize;
    let _ = call1(commit, (this + 4) as u32);
    let _ = call1(FN_800C9500, (this + 4) as u32);

    // observed 0x80019D24..0x80019D50.
    platform::write_u8(this + 0x10, 0x97);
    let root_pane = platform::read_u32(this + 0x1D8) as usize;
    platform::write_u8(
        root_pane + 0xBB,
        platform::read_u8(root_pane + 0xBB) & 0xFE,
    );
    platform::write_u8(this + 0x1E5, 0);
    platform::write_u8(this + 0x1E7, 0);
    platform::write_u8(this + 0x1E4, 1);
    1
}

// ===========================================================================
// 12. "Wm_en_hit" actor state steps (0x8001DF60..0x8001E850)
//
// A small actor family whose state steps write `+0x3F0` (variant flag) and
// `+0x3F4` (next state), scale the actor through vtable slot `+0x150`, and
// spawn the "Wm_en_hit" effect at the actor position. `fn_8001df60` is the
// per-frame dispatch body; the remaining five are the state entry points.
// ===========================================================================

/// Shared tail of the three hit-state entry points.
///
/// Writes the variant flag, builds the 3-float scale vector
/// `[(1.75 + 0.35*|height|) * ±1, y_scale, 0.0]`, calls vtable slot `+0x150`,
/// spawns the `"Wm_en_hit"` effect at `+0x1D8..+0x1E0`, converts the actor
/// position through `0x8006A4B0`, plays effect `0x1A4`, and stores
/// `next_state` at `+0x3F4`.
///
/// This is the common body of `0x8001E1C0`, `0x8001E2E0` and `0x8001E400`;
/// it has no address of its own.
///
/// # Arguments
/// * `this` - actor; scale target and effect fields.
/// * `other` - the hit record's owner (`hit + 4`); its `+0xE8` height feeds
///   the scale.
/// * `flag` - 0 or 1 selector written to `+0x3F0`.
/// * `y_scale` - `3.75` for `0x8001E1C0`, `2.75` for the other two.
/// * `scale_mode` - third argument of the scale call (`0x1000`/`0xC00`).
/// * `next_state` - value stored at `+0x3F4`.
///
/// # Returns
/// `1` (the original returns a constant success value).
///
/// # Safety
/// `this`, `other` and both vtables must be valid.
///
/// # Confidence
/// high for the arithmetic: `fabs` + `frsp` produce `|height|`, the `lfsx`
/// selects the ±1 table entry, and the two `fmuls`/`fadds` order matches
/// Ghidra's expression.
unsafe fn actor_hit_state_step(
    this: usize,
    other: usize,
    flag: u32,
    y_scale: f32,
    scale_mode: u32,
    next_state: u32,
) -> u32 {
    platform::write_u32(this + 0x3F0, flag);

    // observed: `fabs f2`/`frsp f5` for |height|, `lfsx` from the ±1 table.
    let mut scale = crate::platform::GuestLocal::new( [0.0f32; 3]);
    scale[0] = (read_f32(SDA_ACTOR_SCALE_X)
        + read_f32(SDA_ACTOR_HEIGHT_FACTOR)
            * DActor { addr: other as u32 }.as_d_base_actor().speed().x().abs())
        * read_f32(SDA_SIGN_TABLE + flag as usize * 4);
    scale[1] = y_scale;
    scale[2] = read_f32(SDA_ZERO_F32_B);

    let vtable = platform::read_u32(this + 0x60) as usize;
    let set_scale = platform::read_u32(vtable + 0x150) as usize;
    let _ = call3(
        set_scale,
        this as u32,
        scale.as_mut_ptr() as u32,
        scale_mode,
    );

    // observed: the effect position is +0x1D8/+0x1DC with 5504.0 as Z.
    let mut effect_position = crate::platform::GuestLocal::new( [0.0f32; 3]);
    effect_position[0] = read_f32(this + 0x1D8);
    effect_position[1] = read_f32(this + 0x1DC);
    effect_position[2] = read_f32(SDA_ACTOR_EFFECT_Y);
    let _ = call5(
        FN_8016C890,
        STR_WM_EN_HIT,
        0,
        effect_position.as_ptr() as u32,
        0,
        0,
    );

    // observed: 0x20-byte stack block filled from the actor position.
    let mut position = crate::platform::GuestLocal::new( [0u8; 0x20]);
    let pos_ptr = DActor { addr: this as u32 }.as_d_base_actor().pos().addr;
    let _ = call2(FN_8006A4B0, position.as_mut_ptr() as u32, pos_ptr);
    let _ = call4(
        FN_80197F00,
        platform::read_u32(SDA_ACTOR_EFFECT_HANDLE),
        0x1A4,
        position.as_ptr() as u32,
        0,
    );

    platform::write_u32(this + 0x3F4, next_state);
    1
}

/// Per-frame "Wm_en_hit" actor dispatch: swaps the two scene records when
/// they disagree, then drives the hit reaction.
///
/// # Original
/// - Ghidra: `FUN_8001DF60` at `0x8001DF60` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001df60_FUN_8001df60.c`
/// - Removed: the modelled `0x802DCD74`/`0x802DCDC0` calls. They are the
///   `_savegpr_27`/`_restgpr_27` compiler helpers (register spill/restore
///   only); the pair is the incoming `r3`/`r4`.
///
/// # Arguments
/// * `param_1`: first scene record (`r3`), the higher half of the Ghidra pair.
/// * `param_2`: second scene record (`r4`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets bit 1 of the first record's `+0xA1` and calls vtable slots `+0x6C`,
/// `+0x164` and `+0x168`.
///
/// # Safety
/// Both scene records and their actor pointers must be valid.
///
/// # Confidence
/// medium - the record roles are inferred from field use; the marker/string
/// comparison follows the instruction stream.
pub unsafe fn fn_8001df60(param_1: usize, param_2: usize) {
    let first = param_1;
    let second = param_2;

    // observed 0x8001DF74: `lwz r0, 0x34(r4); rlwinm 0,12,12; beq`.
    if (platform::read_u32(second + 0x34) & 0x0008_0000) != 0 {
        let second_actor = platform::read_u32(second + 4) as usize;
        let first_actor = platform::read_u32(first + 4) as usize;
        let mut pending = true;

        // observed 0x8001DF94: `lbz 0x38C`; markers 1 and 2 share one body.
        let marker = DActor { addr: second_actor as u32 }.kind();
        if marker == 1 || marker == 2 {
            let second_vtable = platform::read_u32(second_actor + 0x60) as usize;
            let second_name = platform::read_u32(second_vtable + 0x6C) as usize;
            let second_char = call1(second_name, second_actor as u32);
            let second_char = platform::read_u8(second_char as usize);

            let first_vtable = platform::read_u32(first_actor + 0x60) as usize;
            let first_name = platform::read_u32(first_vtable + 0x6C) as usize;
            let first_char = call1(first_name, first_actor as u32);
            if platform::read_u8(first_char as usize) == second_char {
                pending = false;
            }
        } else if (DActor { addr: second_actor as u32 }.as_d_base_actor().flags_120() & 0x80) != 0 {
            pending = false;
        }

        if pending {
            // observed 0x8001E050: set bit 1 of the first record's +0xA1.
            platform::write_u8(first + 0xA1, platform::read_u8(first + 0xA1) | 2);

            let vtable = platform::read_u32(first_actor + 0x60) as usize;
            let accept = platform::read_u32(vtable + 0x164) as usize;
            let _ = call2(accept, first_actor as u32, second as u32);
            let finish = platform::read_u32(vtable + 0x168) as usize;
            let _ = call1(finish, first_actor as u32);
        }
    }
}

/// First hit-state entry: variant flag from the two `+0xAC` values, X scale
/// `(1.75 + 0.35*|height|) * ±1`, scale mode `0x1000`, next state `2`.
///
/// # Original
/// - Ghidra: `FUN_8001E1C0` at `0x8001E1C0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e1c0_FUN_8001e1c0.c`
///
/// # Arguments
/// * `this` - actor.
/// * `hit` - hit record; its `+0x04` word points at the other actor.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// See [`actor_hit_state_step`]; additionally compares `this + 0xAC` with the
/// other actor's `+0xAC`.
///
/// # Safety
/// `hit + 4` must point at a readable actor.
///
/// # Confidence
/// high - the comparison polarity is confirmed by the `fcmpo`/`cror 2,1,2`/
/// `bne` sequence: the branch is taken exactly when `this + 0xAC < other +
/// 0xAC`.
pub unsafe fn fn_8001e1c0(this: usize, hit: usize) -> u32 {
    let other = platform::read_u32(hit + 4) as usize;
    // observed 0x8001E1E4..0x8001E1FC: less-than selects the `1` store.
    let this_pos = DActor { addr: this as u32 }.as_d_base_actor().pos();
    let other_pos = DActor { addr: other as u32 }.as_d_base_actor().pos();
    let flag = if this_pos.x() < other_pos.x() { 1 } else { 0 };
    actor_hit_state_step(
        this,
        other,
        flag,
        read_f32(SDA_ACTOR_SCALE_Y),
        0x1000,
        2,
    )
}

/// Second hit-state entry: variant flag from the other actor's height,
/// scale mode `0x1000`, next state `3`.
///
/// # Original
/// - Ghidra: `FUN_8001E2E0` at `0x8001E2E0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e2e0_FUN_8001e2e0.c`
///
/// # Arguments
/// * `this` - actor.
/// * `hit` - hit record; its `+0x04` word points at the other actor.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// See [`actor_hit_state_step`]; the flag is `1` when the other actor's
/// `+0xE8` height is negative.
///
/// # Safety
/// `hit + 4` must point at a readable actor.
///
/// # Confidence
/// high - the `fcmpo`/`cror`/`bne` sequence selects the `1` store exactly
/// when the height is less than zero.
pub unsafe fn fn_8001e2e0(this: usize, hit: usize) -> u32 {
    let other = platform::read_u32(hit + 4) as usize;
    // observed 0x8001E30C: the branch is taken when height < 0.0.
    let flag = if read_f32(other + 0xE8) < 0.0 { 1 } else { 0 };
    actor_hit_state_step(
        this,
        other,
        flag,
        read_f32(SDA_ACTOR_SCALE_Y_ALT),
        0x1000,
        3,
    )
}

/// Third hit-state entry: same flag as [`fn_8001e2e0`], scale mode `0xC00`,
/// next state `4`.
///
/// # Original
/// - Ghidra: `FUN_8001E400` at `0x8001E400` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e400_FUN_8001e400.c`
///
/// # Arguments
/// * `this` - actor.
/// * `hit` - hit record; its `+0x04` word points at the other actor.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// See [`actor_hit_state_step`].
///
/// # Safety
/// `hit + 4` must point at a readable actor.
///
/// # Confidence
/// high - same comparison sequence as [`fn_8001e2e0`].
pub unsafe fn fn_8001e400(this: usize, hit: usize) -> u32 {
    let other = platform::read_u32(hit + 4) as usize;
    let flag = if read_f32(other + 0xE8) < 0.0 { 1 } else { 0 };
    actor_hit_state_step(
        this,
        other,
        flag,
        read_f32(SDA_ACTOR_SCALE_Y_ALT),
        0x0C00,
        4,
    )
}

/// Fourth hit-state entry: X scale `1.75 * ±1`, Y scale `2.75`, scale mode
/// `0x600`, next state `1`.
///
/// The selector is the inverse of the byte at `+0x348`: `flag = byte ^ 1`.
///
/// # Original
/// - Ghidra: `FUN_8001E520` at `0x8001E520` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e520_FUN_8001e520.c`
///
/// # Arguments
/// * `this` - actor.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// Writes `+0x3F0` and `+0x3F4`; calls vtable slot `+0x150` with the 3-float
/// vector and `0x600`.
///
/// # Safety
/// `this + 0x60` must be a valid vtable.
///
/// # Confidence
/// high for the arithmetic and stores; medium for the meaning of `+0x348`.
pub unsafe fn fn_8001e520(this: usize) -> u32 {
    // observed 0x8001E548: `lbz +0x348; xori 1`.
    let flag = (platform::read_u8(this + 0x348) ^ 1) as u32;
    platform::write_u32(this + 0x3F0, flag);

    let mut scale = crate::platform::GuestLocal::new( [0.0f32; 3]);
    scale[0] = read_f32(SDA_ACTOR_SCALE_X) * read_f32(SDA_SIGN_TABLE + flag as usize * 4);
    scale[1] = read_f32(SDA_ACTOR_SCALE_Y_ALT);
    scale[2] = read_f32(SDA_ZERO_F32_B);

    let vtable = platform::read_u32(this + 0x60) as usize;
    let set_scale = platform::read_u32(vtable + 0x150) as usize;
    let _ = call3(set_scale, this as u32, scale.as_mut_ptr() as u32, 0x600);
    platform::write_u32(this + 0x3F4, 1);
    1
}

/// Forwards one value to the actor's virtual slot `+0x3F4` with a zero third
/// argument.
///
/// Ghidra could not recover the jump table at `0x8001E86C` and treated the
/// tail branch as an indirect call; the instruction stream loads the slot
/// from the actor vtable and branches through it with
/// `(actor, value, 0)`.
///
/// # Original
/// - Ghidra: `FUN_8001E850` at `0x8001E850` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8001e850_FUN_8001e850.c`
///
/// # Arguments
/// * `value` - forwarded as the second argument (`r4`).
/// * `actor` - object whose vtable slot `+0x3F4` is invoked (`r3`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call through `actor`'s vtable.
///
/// # Safety
/// `actor + 0x60` must point at a valid vtable.
///
/// # Confidence
/// medium - the slot is exact, the callee's identity is not recovered.
pub unsafe fn fn_8001e850(value: u32, actor: usize) {
    let vtable = platform::read_u32(actor + 0x60) as usize;
    let entry = platform::read_u32(vtable + 0x3F4) as usize;
    // observed 0x8001E850: r3 = actor, r4 = value, r5 = 0.
    let _ = call3(entry, actor as u32, value, 0);
}

// ===========================================================================
// Wave 273 lane E - untabled gap body in the 0x8001 main-DOL band (`_E273`)
// ===========================================================================
//
// One prologue-shaped body that no documented extent covered and that the
// wave-266 sweep (`work/wave266/e_candidates.tsv`) listed as still untranslated.
// It was re-walked against `work/dol/main.elf` for this wave: the first word is
// `stwu r1,-0x10(r1)` + `mflr r0`, the recursive descent ends on a single
// `blr` at `0x80014B70`, and no extent in `src/` starts or overlaps the address
// (the two zero words at `0x80014B28`/`0x80014B2C` precede it and the eight
// padding bytes at `0x80014B74..0x80014B80` follow it).
//
// | address | extent | bytes | role |
// |---|---|---|---|
// | `0x80014B30` | `0x80014B30..0x80014B74` | 68 | constructor for the class whose vtable is `0x802FF670` |

/// `0x802FF670`: vtable installed at `this + 0` by [`fn_80014b30`]. Its two
/// RTTI header words are zero; the methods are `0x80014B80`, `0x8000F720`,
/// `0x8016A090`, `0x8016A0E0`, `0x80014BF0` and `0x80014EF0`.
const W273_VTABLE_802FF670: u32 = 0x802F_F670;

/// Constructs an object of the class whose vtable is `0x802FF670`.
///
/// The body is the standard CodeWarrior constructor sequence, identical to the
/// sibling [`fn_80014970`] except for the vtable word: run the base constructor
/// `0x8016A010`, overwrite the vtable at `+0x00`, clear the word at `+0x08`.
/// The class name is not recovered; the vtable sits directly before the
/// `0x802FF690` table installed by [`fn_80014970`] in the same data block.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80014B30`.
/// - Extent: `0x80014B30..0x80014B74` (68 bytes), ending at the `blr` at
///   `0x80014B70`; two zero words precede it and eight zero bytes follow at
///   `0x80014B74..0x80014B80`.
/// - Reach: no `b` or `bl` in the executable segments targets it and no data
///   word names it.
/// - Table: the vtable `0x802FF670` that it installs is not itself named by a
///   `lis`/`addi` pair inside translated code; the class has no caller in
///   `main.dol`.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r3`; `bl 0x8016A010`; `lis r3,0x8030`;
///   `li r0,0`; `addi r3,r3,-0x990` (`0x802FF670`); `stw r3,0(r31)`;
///   `mr r3,r31`; `stw r0,8(r31)`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: object to construct; at least `0x0C` writable bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the base constructor, writes the vtable word and clears `+0x08`.
///
/// # Safety
/// `this` must be writable through `+0x08` and accepted by `0x8016A010`.
///
/// # Confidence
/// high - the call, the address pair and both stores are literal; low for the
/// class identity (no RTTI name and no caller in `main.dol`).
pub unsafe fn fn_80014b30(this: usize) -> usize {
    // observed 0x80014B44: Ghidra prints no argument, but r3 is still `this`.
    let _ = call1(FN_8016A010, this as u32);
    platform::write_u32(this, W273_VTABLE_802FF670);
    platform::write_u32(this + 0x08, 0);
    this
}
