//! Path/rail nodes and the course-select cursor helpers in the same band.
//!
//! **What lives here.** Four related clusters:
//!
//! * **Node-collection accessors** (`0x8000D190..0x8000D660`). Every one takes
//!   an owner object, reads a manager pointer from `[this+4]`, dynamically
//!   casts it to the type described by `0x802F8FF0`, then reads or writes a
//!   node through the collection API at `0x8023B800..0x8023B8C0`. They are
//!   [`fn_8000d190`], [`fn_8000d260`], [`fn_8000d3b0`], [`fn_8000d5b0`] and
//!   [`fn_8000d660`].
//! * **Rail / follower state** (`0x8000DE70..0x8000E840`). A follower object
//!   reserves `+0x30..+0xC36`: an anchor point, up to 255 12-byte polyline
//!   points, a speed and an encoded point count. [`fn_8000df60`] builds the
//!   polyline, [`fn_8000de70`] and [`fn_8000e840`] initialise followers, and
//!   [`fn_8000e030`] sums a rail length.
//! * **Record writers** (`0x8000D0A0`, `0x8000EFC0`, `0x8000F0F0`).
//!   [`fn_8000efc0`] clears a 0x20-byte route record; [`fn_8000f0f0`] is the
//!   single-value twin of the node writer [`fn_8000d260`]; [`fn_8000d0a0`]
//!   stores a three-word vector at a fixed record stride.
//! * **Wave-11 band additions** (`0x8000D7E0..0x800126A0`). The follower
//!   transform seed, two actor-attach wrappers, the course-select cursor
//!   constructor/deleting-destructor family, and the per-slot cursor state
//!   writers. They live here because wave 11 assigned the whole
//!   `0x8000C000..0x80014000` band to this module; the path/rail cluster above
//!   is the file's original subject.
//!
//! **Original addresses.** `0x8000D0A0`, `0x8000D190`, `0x8000D260`,
//! `0x8000D3B0`, `0x8000D5B0`, `0x8000D660`, `0x8000D7E0`, `0x8000DE70`,
//! `0x8000DF60`, `0x8000E030`, `0x8000E840`, `0x8000EFC0`, `0x8000F0F0`,
//! `0x8000F370`, `0x8000F800`, `0x8000FC30`, `0x8000FE90`, `0x8000FED0`,
//! `0x8000FF10`, `0x8000FFB0`, `0x8000FFF0`, `0x80010030`, `0x80010070`,
//! `0x800100D0`, `0x800105C0`, `0x800108C0`, `0x80010F40`, `0x80011020`,
//! `0x80011120`, `0x80011180`, `0x800112C0`, `0x80011310`, `0x800113E0`,
//! `0x80011470`, `0x800114E0`, `0x800115C0`, `0x800126A0` - all inside the
//! `0x8000C000..0x80014000` band.
//!
//! **Why it exists.** Actors that follow a rail share this state machine. The
//! node API itself (`0x8023xxxx`) and the iterator API (`0x8025Cxxx`) are
//! owned by other layers, so the calls stay opaque; the argument order and the
//! iteration are exact. The cursor cluster is the course-select counterpart of
//! the same "per-slot object plus state byte" pattern and has no other
//! destination module in this band.
//!
//! **Dependencies.** `crate::platform`; [`crate::render::geometry::vec2_differs_from`]
//! for the duplicate-point test; and the opaque callees documented beside their
//! constants (`0x80168400` collection lookup, `0x8023xxxx` node payloads,
//! `0x8025Cxxx` iterators, `0x80164FD0` actor base, `0x8016A2A0`/`0x8016A0F0`
//! model flags, `0x802C7C70`/`0x802C7B70` scene sub-objects, `0x800C9490`
//! cursor show/hide, `0x800C9A60` cursor animation).
//!
//! **Start here.** [`fn_8000df60`] - the polyline builder shows the full
//! follower layout in one function. For the wave-11 additions, start at
//! [`fn_8000fc30`], which builds every cursor slot that the later state
//! writers address by constant offset.
//!
//! **Glossary.** A "node" is an element of the game's route/collection
//! container; its concrete type is not recovered. `+0x3C` is a relative offset
//! from an element to its node payload (0 = absent). See `docs/GLOSSARY.md`.

use crate::platform;
use crate::platform::{call1, call2, call3, call4, call5, call6};
use crate::render::geometry::vec2_differs_from;

// ---------------------------------------------------------------------------
// Cross-lane callees (opaque here).
//
// Each constant is the *entry* address of a function owned by another lane.
// They are declared so a reader can see which external effects a translation
// has without leaving this file.
// ---------------------------------------------------------------------------

/// Returns a collection handle for a context object (argument varies by call
/// site: see [`fn_8000d260`] and [`fn_8000d5b0`]).
///
/// Ghidra: 80168400 FUN_80168400.c
const FN_80168400: usize = 0x8016_8400;

/// Number of elements in the collection described by a handle (takes
/// `&mut handle`).
///
/// Ghidra: 8023b8c0 FUN_8023b8c0.c
const FN_8023B8C0: usize = 0x8023_B8C0;

/// Element `i` of a collection (takes `&mut handle`, index).
///
/// Ghidra: 8023b860 FUN_8023b860.c
const FN_8023B860: usize = 0x8023_B860;

/// Looks an element up by key in a collection (takes `&mut handle`, key).
///
/// Ghidra: 8023b800 FUN_8023b800.c
const FN_8023B800: usize = 0x8023_B800;

/// Evaluates a path node into the caller's output word.
///
/// Ghidra: 8023d0a0 FUN_8023d0a0.c
const FN_8023D0A0: usize = 0x8023_D0A0;

/// Evaluates a node pointer into a 2-float result (`node, param, out`).
///
/// Ghidra: 8023d300 FUN_8023d300.c
const FN_8023D300: usize = 0x8023_D300;

/// Writes a 2-float result through a node pointer (`node, param, value`).
///
/// Ghidra: 8023d3c0 FUN_8023d3c0.c
const FN_8023D3C0: usize = 0x8023_D3C0;

/// Evaluates a node into four bytes (`node, param, byte_out`).
///
/// Ghidra: 8023d460 FUN_8023d460.c
const FN_8023D460: usize = 0x8023_D460;

/// Writes a 2-float result through a node pointer (variant of
/// [`FN_8023D3C0`] used by `0x8000D3B0`).
///
/// Ghidra: 8023d520 FUN_8023d520.c
const FN_8023D520: usize = 0x8023_D520;

/// Releases/finishes a node handle (`&mut handle`, 0).
///
/// Ghidra: 8023ca80 FUN_8023ca80.c
const FN_8023CA80: usize = 0x8023_CA80;

/// Builds a node iterator over an object at an index: `(scratch, obj, index)`.
///
/// Ghidra: 8025c660 FUN_8025c660.c
const FN_8025C660: usize = 0x8025_C660;

/// Returns the element at `index` from an iterator built by
/// [`FN_8025C660`], storing a handle into the caller's word.
///
/// Ghidra: 8025c4c0 FUN_8025c4c0.c
const FN_8025C4C0: usize = 0x8025_C4C0;

/// Returns the element at `index` from a node iterator (used with
/// [`FN_8023D3C0`]/[`FN_8023D520`]).
///
/// Ghidra: 8025c560 FUN_8025c560.c
const FN_8025C560: usize = 0x8025_C560;

/// Sets a signed selector on an iterator built by [`FN_8025C960`].
///
/// Ghidra: 8025c8e0 FUN_8025c8e0.c
const FN_8025C8E0: usize = 0x8025_C8E0;

/// Builds an iterator for another node flavour: `(scratch, obj, param)`.
///
/// Ghidra: 8025c960 FUN_8025c960.c
const FN_8025C960: usize = 0x8025_C960;

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

/// Supplies a path/route to the follower sub-object at `+8`:
/// `(sub, path)`.
///
/// Ghidra: 802c7b70 FUN_802c7b70.c
const FN_802C7B70: usize = 0x802C_7B70;

/// Prepares the 0x20-byte sub-object at `obj+8`:
/// `(obj+8, source, 0x20)`.
///
/// Ghidra: 8016a690 FUN_8016a690.c
const FN_8016A690: usize = 0x8016_A690;

/// Constructs the shared course-select base embedded at `+8` of every cursor
/// object. Called once from [`fn_8000fc30`].
///
/// Ghidra: 800c8950 FUN_800c8950.c
const FN_800C8950: usize = 0x800C_8950;

/// Installs one cursor slot object:
/// `(slot+0x18, slot+4, slot+8, descriptor)`.
///
/// Ghidra: 8015fc90 FUN_8015fc90.c
const FN_8015FC90: usize = 0x8015_FC90;

/// CodeWarrior `operator delete`-shaped release; the pointer is in `r3`, and
/// the callee zeroes `r4` itself before tail-calling the allocator.
///
/// Ghidra: 802b9280 FUN_802b9280.c (20 bytes; `addi r4,r0,0; b 802b8f70`)
const FN_802B9280: usize = 0x802B_9280;

/// Partial destructor called as `(object, 0)` by the deleting destructors.
///
/// Ghidra: 8015f9f0 FUN_8015f9f0.c
const FN_8015F9F0: usize = 0x8015_F9F0;

/// Bool-returning course-select query; `0x800B54B0` is a 4-byte tail thunk to
/// this address and the callee ignores `r3`.
///
/// Ghidra: 801dd1d0 FUN_801dd1d0.c ; thunk at 800b54b0_FUN_800b54b0.c
const FN_801DD1D0: usize = 0x801D_D1D0;

/// Hides one cursor element: `(base+8, index)`.
///
/// Ghidra: 800c9490 FUN_800c9490.c
const FN_800C9490: usize = 0x800C_9490;

/// Tests one cursor element: `(base+8, index) -> non-zero when present`.
///
/// Ghidra: 800c9610 FUN_800c9610.c
const FN_800C9610: usize = 0x800C_9610;

/// Sets one cursor element's state: `(base+8, index, value)`.
///
/// Ghidra: 800c92f0 FUN_800c92f0.c
const FN_800C92F0: usize = 0x800C_92F0;

/// Shows one cursor element: `(base+8, index)`.
///
/// Ghidra: 800c9380 FUN_800c9380.c
const FN_800C9380: usize = 0x800C_9380;

/// Starts one cursor animation: `(pane, stamp, 4, action, 0)`.
///
/// Ghidra: 800c9a60 FUN_800c9a60.c
const FN_800C9A60: usize = 0x800C_9A60;

/// Returns the course-select frame stamp from `[0x80369CB4]`. The function
/// ignores `r3`, so the local helper [`call0_r`] is used.
///
/// Ghidra: 800cdc60 FUN_800cdc60.c
const FN_800CDC60: usize = 0x800C_DC60;

/// `r13-0x5200`: pointer to the course-select player/controller record. The
/// record's inner pointer at `[+0]` has a `u16` count at `+6`.
const G_CURSOR_PLAYERS: usize = 0x8042_A480;

/// `r13-0x5A84`: singleton cursor object written by [`fn_8000fc30`].
const G_CURSOR_OBJECT: usize = 0x8042_9BFC;

/// `r13-0x5730`: course manager; its `[+4][+8]` slot is the selected course
/// id compared by [`fn_800126a0`].
const G_COURSE_MANAGER: usize = 0x8042_9F50;

/// `r13-0x5704`: course state object whose byte `+0x3D9` mirrors whether the
/// selected course id is non-zero.
const G_COURSE_STATE: usize = 0x8042_9F7C;

/// Vtable installed at `+0` of the cursor object by [`fn_8000fc30`].
const VTABLE_CURSOR: u32 = 0x802F_EA28;

/// Final first word of each embedded cursor slot object (`0x802FEA38`).
const VTABLE_CURSOR_ALT: u32 = 0x802F_EA38;

/// First word of each embedded cursor slot object (`0x802FEA68`).
const VTABLE_CURSOR_SLOT_A: u32 = 0x802F_EA68;

/// Second word of each embedded cursor slot object (`0x802FEAF8`).
const VTABLE_CURSOR_SLOT_B: u32 = 0x802F_EAF8;

/// Third word of each embedded cursor slot object (`0x802FEA98`).
const VTABLE_CURSOR_SLOT_C: u32 = 0x802F_EA98;

/// Fourth word of each embedded cursor slot object (`0x802FEAB0`).
const VTABLE_CURSOR_SLOT_D: u32 = 0x802F_EAB0;

/// Slot bases inside the cursor object, as built by [`fn_8000fc30`]. Eight
/// slots of 0x24 bytes each; the last ends at `+0x368`.
const CURSOR_SLOT_BASES: [usize; 8] = [
    0x1A0, 0x1DC, 0x218, 0x254, 0x290, 0x2CC, 0x308, 0x344,
];

/// Runtime-filled descriptors passed to the slot installer `0x8015FC90`, one
/// per slot in [`CURSOR_SLOT_BASES`] order.
///
/// `// unknown:` all eight are zero in the shipped image and are filled by the
/// course-select scene; the concrete object type is not recovered.
const CURSOR_SLOT_DESCRIPTORS: [u32; 8] = [
    0x8035_16B8,
    0x8035_17B8,
    0x8035_18B8,
    0x8035_19B8,
    0x8035_1AB8,
    0x8035_1BB8,
    0x8035_1CB8,
    0x8035_1DB8,
];

/// Runtime cursor state words passed to the slot virtual method at `+0x18`.
///
/// `// unknown:` zero in the shipped image; they select visible/hidden or
/// normal/alternate animation state and are named by address only.
const CURSOR_DESC_803516B8: u32 = 0x8035_16B8;
const CURSOR_DESC_803516F8: u32 = 0x8035_16F8;
const CURSOR_DESC_80351738: u32 = 0x8035_1738;
const CURSOR_DESC_80351778: u32 = 0x8035_1778;
const CURSOR_DESC_803517B8: u32 = 0x8035_17B8;
const CURSOR_DESC_803517F8: u32 = 0x8035_17F8;
const CURSOR_DESC_80351838: u32 = 0x8035_1838;
const CURSOR_DESC_80351878: u32 = 0x8035_1878;
const CURSOR_DESC_803518F8: u32 = 0x8035_18F8;

/// `r2-0x7F2C`: `0.0f`, the initial value of the four cursor transform
/// fields written by [`fn_800105c0`].
const F32_ZERO_8042B154: usize = 0x8042_B154;

/// `r2-0x7F28`: `0.5f`, the scale applied to the player-count term.
const F32_HALF_8042B158: usize = 0x8042_B158;

/// `r2-0x7F24`: `330.0f`, the x-axis origin used by [`fn_800105c0`].
const F32_330_8042B15C: usize = 0x8042_B15C;

/// `r2-0x7F20`: `640.0f`, the y-axis extent used by [`fn_800105c0`].
const F32_640_8042B160: usize = 0x8042_B160;

/// `r2-0x7F9C`: zero word copied into the two follower transform slots by
/// [`fn_8000d7e0`].
const U32_ZERO_8042B0E4: usize = 0x8042_B0E4;

/// `r2-0x7F98`: `8.0f`, the default follower speed passed to
/// [`fn_8000de70`] by [`fn_8000d7e0`].
const F32_SPEED_8042B0E8: usize = 0x8042_B0E8;

/// `r2-0x7F10`: four player-presence bits, `{1, 2, 4, 8}`. Indexed by the
/// signed player number in [`fn_80010f40`].
const PLAYER_BIT_TABLE_8042B170: usize = 0x8042_B170;

/// `r2-0x7F80` - zero, the starting value of the rail-length sum.
const ZERO_F32_C: usize = 0x8042_B100;

/// Virtual "is this object of the type described by ..." test. The original
/// loads the vtable word at `[obj]`, takes slot `+0x08`, and calls it with
/// `(obj, &descriptor)`; the result is `!= 0` when the cast succeeds.
const VSLOT_IS_INSTANCE_OF: usize = 0x08;

/// Type descriptor word passed to [`VSLOT_IS_INSTANCE_OF`] by the path-node
/// accessors in this range. Ghidra: `&DAT_802f8ff0`.
const TYPE_DESCRIPTOR_802F8FF0: u32 = 0x802F_8FF0;

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

/// A 64-byte, 8-byte-aligned scratch block. The original functions build
/// callee-owned iterators/temporaries on the stack (52-64 bytes); the layout
/// inside is private to those callees, so only the size and alignment matter.
#[repr(C, align(8))]
struct Scratch64([u8; 64]);

impl Scratch64 {
    #[inline]
    fn new() -> Self {
        Self([0u8; 64])
    }

    /// Address of the block, as passed in `r3`/`r4`.
    #[inline]
    fn addr(&mut self) -> u32 {
        self.0.as_mut_ptr() as u32
    }
}

/// Runs the "dynamic cast" idiom shared by the path-node accessors in this
/// range: load `[obj]` (vtable), load slot `+0x08`, call it with
/// `(obj, &type_descriptor)`, and return the raw result.
///
/// # Safety
/// `obj` must point at an object whose first word is a readable vtable with at
/// least three entries.
unsafe fn is_instance_of(obj: u32, type_descriptor: u32) -> u32 {
    let vtable = platform::read_u32(obj as usize);
    let slot = platform::read_u32(vtable as usize + VSLOT_IS_INSTANCE_OF);
    let mut descriptor = type_descriptor;
    platform::call2(slot as usize, obj, &mut descriptor as *mut u32 as u32)
}

/// Calls a function with no declared arguments and keeps the word it returns
/// in `r3`. [`platform::call0`] intentionally discards the result; the
/// course-select helpers here need it.
///
/// # Safety
/// `addr` must point to a function with this ABI.
#[inline]
unsafe fn call0_r(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Calls virtual slot `slot` of `obj` with only the receiver in `r3`.
///
/// The original loads `[obj]`, then `[vtable+slot]`, then branches through
/// `ctr` while leaving `r3 = obj`; this helper models exactly that.
///
/// # Safety
/// `obj` must point at a readable vtable whose `slot` entry is a valid
/// function.
unsafe fn vslot_call0(obj: u32, slot: usize) -> u32 {
    let vtable = platform::read_u32(obj as usize);
    let entry = platform::read_u32(vtable as usize + slot);
    call1(entry as usize, obj)
}

/// Calls virtual slot `slot` of `obj` with `(obj, arg)` in `r3`/`r4`.
///
/// # Safety
/// `obj` must point at a readable vtable whose `slot` entry is a valid
/// function accepting `(obj, arg)`.
unsafe fn vslot_call2(obj: u32, slot: usize, arg: u32) -> u32 {
    let vtable = platform::read_u32(obj as usize);
    let entry = platform::read_u32(vtable as usize + slot);
    call2(entry as usize, obj, arg)
}

// ===========================================================================
// 1. Path-node accessors
//
// These five functions share one shape: take an owner object, recover a
// "manager" from `[this+4]`, dynamically cast it to the type described by
// 0x802F8FF0, then walk a node collection exported by 0x8023B8xx and write a
// point through 0x8023D3xx. The node API itself lives in another lane, so the
// calls stay opaque; the argument order and iteration are exact.
// ===========================================================================

/// Stores `value` on the first node of the collection attached to `this`.
///
/// # Original
/// - Ghidra: `FUN_8000D190` at `0x8000D190` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d190_FUN_8000d190.c`
///
/// # Arguments
/// * `this` - owner object; the manager pointer is `[this+4]`.
/// * `key` - passed to the node iterator (`0x8025C660`).
/// * `value` - word written by `0x8023D0A0` through the returned element.
///
/// # Returns
/// Nothing. All failure paths simply skip the write.
///
/// # Side effects
/// One virtual call (the cast), then `0x8025C660`, `0x8025C4C0`,
/// `0x8023D0A0`.
///
/// # Safety
/// `[this+4]` must be null or a valid object with a readable vtable; if the
/// cast succeeds, the node calls must accept it.
///
/// # Confidence
/// medium - control flow and argument order are exact; the node API's meaning
/// comes from its own pseudocode.
pub unsafe fn fn_8000d190(this: usize, key: u32, value: u32) {
    let manager = platform::read_u32(this + 4);

    // observed 0x8000D1B8..0x8000D1F0: cast manager, keep it only if the
    // virtual "is instance" slot returns non-zero.
    let mut cast_ok = false;
    if manager != 0 && is_instance_of(manager, TYPE_DESCRIPTOR_802F8FF0) != 0 {
        cast_ok = true;
    }
    let target = if cast_ok { manager } else { 0 };

    if target != 0 {
        let mut iterator = Scratch64::new();
        // observed 0x8000D20C..0x8000D234.
        call3(FN_8025C660, iterator.addr(), target, key);
        let mut element = call2(FN_8025C4C0, iterator.addr(), 0);
        call2(FN_8023D0A0, &mut element as *mut u32 as u32, value);
    }
}

/// Evaluates every node of a collection and applies a point to it.
///
/// For each element the body:
/// 1. resolves the element's relative offset at `[element+0x3C]` (0 means
///    "null node"),
/// 2. asks `0x8023D300` for the node's 2-float point,
/// 3. builds an iterator element for `this` at the same index and asks
///    `0x8025C560` for a value,
/// 4. writes the point through `0x8023D3C0`,
/// 5. releases the handle with `0x8023CA80`.
///
/// # Original
/// - Ghidra: `FUN_8000D260` at `0x8000D260` (332 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d260_FUN_8000d260.c`
///
/// # Arguments
/// * `this` - owner object (manager at `[this+4]`).
/// * `param_2` - forwarded to `0x8023D300`/`0x8023D3C0`.
/// * `param_3` - forwarded to `0x8025C560`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes through the node API; calls `0x80168400`, `0x8023B8C0`,
/// `0x8023B860`, `0x8025C660`, `0x8025C560`, `0x8023D300`, `0x8023D3C0`,
/// `0x8023CA80`.
///
/// # Safety
/// `this` must be a valid owner; the collection and node functions must accept
/// `param_2`/`param_3` as passed.
///
/// # Confidence
/// medium - the loop and the two stack temporaries are exact; the node API is
/// opaque.
pub unsafe fn fn_8000d260(this: usize, param_2: u32, param_3: u32) {
    // observed 0x8000D288: `bl 0x80168400` with r3 still holding `this`.
    let mut collection = call1(FN_80168400, this as u32);

    let manager = platform::read_u32(this + 4);
    let mut cast_ok = false;
    if manager != 0 && is_instance_of(manager, TYPE_DESCRIPTOR_802F8FF0) != 0 {
        cast_ok = true;
    }
    let target = if cast_ok { manager } else { 0 };

    if target != 0 {
        let mut index: u32 = 0;
        loop {
            // observed 0x8000D37C..0x8000D388: re-read the count each pass and
            // stop when `index >= count` (`cmpw r30,r3; blt body`).
            let count = call1(FN_8023B8C0, &mut collection as *mut u32 as u32);
            if index >= count {
                break;
            }

            // observed 0x8000D2F8..0x8000D31C: resolve the element, follow its
            // `+0x3C` relative offset, then advance by the 0x20-byte header.
            let element = call2(FN_8023B860, &mut collection as *mut u32 as u32, index);
            let rel = platform::read_u32(element as usize + 0x3C);
            let mut node = if rel != 0 { element + rel } else { 0 };
            node += 0x20;

            let mut point = [0u32; 2];
            call3(FN_8023D300, &mut node as *mut u32 as u32, param_2, point.as_mut_ptr() as u32);

            let mut iterator = Scratch64::new();
            call3(FN_8025C660, iterator.addr(), target, index);
            let mut value = call2(FN_8025C560, iterator.addr(), param_3);

            // observed 0x8000D350..0x8000D368: the C reuses the local that
            // held the node pointer for the returned value; the two are
            // separate variables here.
            let mut resolved = point[0];
            call3(FN_8023D3C0, &mut value as *mut u32 as u32, param_2, &mut resolved as *mut u32 as u32);

            call2(FN_8023CA80, &mut value as *mut u32 as u32, 0);
            index += 1;
        }
    }
}

/// `0x8000D3B0` - same walk as [`fn_8000d260`], using the second node
/// evaluator pair (`0x8023D460` to read, `0x8023D520` to write).
///
/// The two functions differ only in those two callees; everything else
/// (collection handle, cast, index loop, scratch sizes) is byte-identical.
///
/// # Original
/// - Ghidra: `FUN_8000D3B0` at `0x8000D3B0` (332 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d3b0_FUN_8000d3b0.c`
///
/// # Arguments
/// * `this` - owner object (manager at `[this+4]`).
/// * `param_2` - forwarded to the node evaluators.
/// * `param_3` - forwarded to `0x8025C560`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Same call set as [`fn_8000d260`] with `0x8023D460`/`0x8023D520` instead of
/// `0x8023D300`/`0x8023D3C0`.
///
/// # Safety
/// Same contract as [`fn_8000d260`].
///
/// # Confidence
/// medium - see [`fn_8000d260`].
pub unsafe fn fn_8000d3b0(this: usize, param_2: u32, param_3: u32) {
    let mut collection = call1(FN_80168400, this as u32);

    let manager = platform::read_u32(this + 4);
    let mut cast_ok = false;
    if manager != 0 && is_instance_of(manager, TYPE_DESCRIPTOR_802F8FF0) != 0 {
        cast_ok = true;
    }
    let target = if cast_ok { manager } else { 0 };

    if target != 0 {
        let mut index: u32 = 0;
        loop {
            let count = call1(FN_8023B8C0, &mut collection as *mut u32 as u32);
            if index >= count {
                break;
            }

            let element = call2(FN_8023B860, &mut collection as *mut u32 as u32, index);
            let rel = platform::read_u32(element as usize + 0x3C);
            let mut node = if rel != 0 { element + rel } else { 0 };
            node += 0x20;

            let mut point = [0u32; 2];
            call3(FN_8023D460, &mut node as *mut u32 as u32, param_2, point.as_mut_ptr() as u32);

            let mut iterator = Scratch64::new();
            call3(FN_8025C660, iterator.addr(), target, index);
            let mut value = call2(FN_8025C560, iterator.addr(), param_3);

            let mut resolved = point[0];
            call3(FN_8023D520, &mut value as *mut u32 as u32, param_2, &mut resolved as *mut u32 as u32);

            call2(FN_8023CA80, &mut value as *mut u32 as u32, 0);
            index += 1;
        }
    }
}

/// Looks a node up by key and returns its first four bytes.
///
/// # Original
/// - Ghidra: `FUN_8000D5B0` at `0x8000D5B0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d5b0_FUN_8000d5b0.c`
///
/// # Arguments
/// * `out` - 4 bytes written (`// unknown:` interpreted as a colour/state
///   word - the callee returns four separate bytes).
/// * `context` - argument handed to `0x80168400` to build the collection
///   handle (the caller's scene/object).
/// * `key` - lookup key for `0x8023B800`.
/// * `param_4` - forwarded to `0x8023D460`.
///
/// # Returns
/// Always `1` (the original's `addi r3,r0,1`).
///
/// # Side effects
/// Four `stb`s to `out`; calls `0x80168400`, `0x8023B800`, `0x8023D460`.
///
/// # Safety
/// `out` writable for 4 bytes; `context`/`key` must be accepted by the node
/// API.
///
/// # Confidence
/// medium - the byte copy is exact; the meaning of the four bytes is unknown.
pub unsafe fn fn_8000d5b0(out: usize, context: u32, key: u32, param_4: u32) -> u32 {
    let mut collection = call1(FN_80168400, context);

    // observed 0x8000D5EC..0x8000D608: find by key, follow the relative
    // offset at +0x3C, then add the 0x20-byte header.
    let element = call2(FN_8023B800, &mut collection as *mut u32 as u32, key);
    let rel = platform::read_u32(element as usize + 0x3C);
    let node = if rel != 0 { element + rel } else { 0 };
    let mut node = node + 0x20;

    let mut bytes = [0u8; 4];
    call3(FN_8023D460, &mut node as *mut u32 as u32, param_4, bytes.as_mut_ptr() as u32);

    // observed 0x8000D61C..0x8000D63C: four independent byte stores.
    platform::write_u8(out, bytes[0]);
    platform::write_u8(out + 1, bytes[1]);
    platform::write_u8(out + 2, bytes[2]);
    platform::write_u8(out + 3, bytes[3]);

    1
}

/// Selects a node from `this`'s collection and sets a sign-derived selector.
///
/// The last expression in the original is
/// `(-param_3 | param_3) >> 31`, i.e. "1 when the signed value is negative,
/// 0 otherwise" (`neg r0,r29; or r0,r0,r29; rlwinm r4,r0,1,31,31`).
///
/// # Original
/// - Ghidra: `FUN_8000D660` at `0x8000D660` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d660_FUN_8000d660.c`
///
/// # Arguments
/// * `this` - owner object (manager at `[this+4]`).
/// * `param_2` - forwarded to `0x8025C960`.
/// * `param_3` - signed value; its sign becomes the selector `0` or `1`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8025C960`, `0x8025C8E0`; one virtual cast.
///
/// # Safety
/// Same contract as [`fn_8000d190`].
///
/// # Confidence
/// medium - exact control flow; the selector's consumer is opaque.
pub unsafe fn fn_8000d660(this: usize, param_2: u32, param_3: i32) {
    let manager = platform::read_u32(this + 4);
    let mut cast_ok = false;
    if manager != 0 && is_instance_of(manager, TYPE_DESCRIPTOR_802F8FF0) != 0 {
        cast_ok = true;
    }
    let target = if cast_ok { manager } else { 0 };

    if target != 0 {
        let mut iterator = Scratch64::new();
        call3(FN_8025C960, iterator.addr(), target, param_2);

        // observed 0x8000D6EC..0x8000D6FC: neg/or/sign-bit.
        let folded = (param_3 as u32).wrapping_neg() | param_3 as u32;
        let selector = folded >> 31;
        call2(FN_8025C8E0, iterator.addr(), selector);
    }
}

// ===========================================================================
// 2. Rail / follower initialisation
//
// The objects below reserve 0xC30..0xD30 bytes for the follower state and
// store a polyline of 12-byte points at +0x3C..+0xC30 (255 points max, the
// count at +0xC34 is "points * 2 + 2").
// ===========================================================================

/// Builds the follower polyline from a point array, dropping near-duplicates.
///
/// The object keeps:
/// * `+0x30/+0x34` - first point `x`/`y` (the "spawn" anchor),
/// * `+0x38` - first point's `u16` third field,
/// * `+0x3C..+0xC30` - up to 255 points of `{f32 x, f32 y, u16 z, u16 pad}`,
/// * `+0xC34` - encoded point count (`2 * points + 2`), 0 when no point was
///   appended.
///
/// The walk copies `first` into the anchor fields, then appends every following
/// point that [`vec2_differs_from`] considers distinct from the previous
/// accepted one (tolerance `0.1` at `0x8042B0F0`). Appending stops early when
/// the 255-point buffer is full.
///
/// # Original
/// - Ghidra: `FUN_8000DF60` at `0x8000DF60` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000df60_FUN_8000df60.c`
///
/// # Arguments
/// * `this` - follower object.
/// * `first` - first point of the source array.
/// * `end` - one-past-the-last point (exclusive bound, as in the original
///   `cmpw r30,r26; blt`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the anchor (3 words) and up to 255 * 12 bytes; calls
/// [`vec2_differs_from`].
///
/// # Safety
/// `first..end` must be a readable array of 12-byte points; `this` must be
/// valid up to `+0xC36`.
///
/// # Confidence
/// high - the assembly loop (`0x8000DF74..0x8000E024`) was checked
/// statement by statement; Ghidra's `undefined8` return from `0x802DCD6C` was
/// a prologue-save artefact, not a real call.
pub unsafe fn fn_8000df60(this: usize, first: usize, end: usize) {
    // observed 0x8000DF78: `sth r0,0xC34(r3)` - clear the encoded count.
    platform::write_u16(this + 0xC34, 0);

    // observed 0x8000DF84..0x8000DFAC: copy the anchor point.
    write_f32(this + 0x30, read_f32(first));
    write_f32(this + 0x34, read_f32(first + 4));
    platform::write_u16(this + 0x38, platform::read_u16(first + 8));

    let buffer_end = this + 0xC30;
    let mut out = this + 0x3C;
    let mut count: i32 = 0;
    let mut accepted = first;
    let mut cursor = first + 12;

    while cursor < end {
        if vec2_differs_from(accepted, cursor) {
            write_f32(out, read_f32(cursor));
            write_f32(out + 4, read_f32(cursor + 4));
            platform::write_u16(out + 8, platform::read_u16(cursor + 8));
            accepted = cursor;
            count += 2;
            out += 12;
            // observed 0x8000DFE8..0x8000DFF0: stop when the buffer is full.
            if out == buffer_end {
                break;
            }
        }
        cursor += 12;
    }

    // observed 0x8000E000..0x8000E00C: `if (count != 0) *(+0xC34) = count+2`.
    if count != 0 {
        platform::write_u16(this + 0xC34, (count + 2) as u16);
    }
}

/// Initialises a follower object that walks the polyline in
/// [`fn_8000df60`] using speed stored at `+0xC30`.
///
/// # Original
/// - Ghidra: `FUN_8000DE70` at `0x8000DE70` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000de70_FUN_8000de70.c`
///
/// # Arguments
/// * `this` - follower object.
/// * `point` - pointer to one word copied into the model sub-object.
/// * `speed` - stored as `f32` at `+0xC30` (the original receives it in `f1`,
///   which Ghidra prints as `double param_1`).
/// * `pose` - signed pose index; negative means "no pose".
/// * `arg_a` - forwarded in the negative-pose branch.
/// * `arg_b` - scene/model parameter forwarded to `0x80164FD0`.
///
/// # Returns
/// `1` on success, `0` when `0x80164FD0` fails.
///
/// # Side effects
/// Stores speed and a zero `u16`; calls `0x80164FD0`, `0x802C7C70`,
/// `0x8016A2A0`, `0x8016A0F0`.
///
/// # Safety
/// `this` valid up to `+0xC36`; `point` readable for 4 bytes.
///
/// # Confidence
/// medium - register use was checked against the assembly; only the
/// meaning of the model flags is inferred.
pub unsafe fn fn_8000de70(
    this: usize,
    point: usize,
    speed: f32,
    pose: i32,
    arg_a: u32,
    arg_b: u32,
) -> u32 {
    // observed 0x8000DEA8/0x8000DEAC.
    write_f32(this + 0xC30, speed);
    platform::write_u16(this + 0xC34, 0);

    // observed 0x8000DEB0: `bl 0x80007510` (thunk to 0x80164FD0).
    let mut local = [0u32; 2];
    if call3(FN_80164FD0, this as u32, arg_b, 0) == 0 {
        return 0;
    }

    // observed 0x8000DEC4..0x8000DEE4: `0x802C7C70(this+8, &local, 1, 0, 1, 1)`.
    local[0] = platform::read_u32(point);
    call6(FN_802C7C70, (this + 8) as u32, local.as_mut_ptr() as u32, 1, 0, 1, 1);

    // observed 0x8000DEE8: `cmpwi r5,0; blt` - signed pose.
    if pose < 0 {
        call3(FN_8016A2A0, this as u32, 0, arg_a);
        call3(FN_8016A0F0, this as u32, 6, 1);
    } else {
        call3(FN_8016A2A0, this as u32, pose as u32, 0);
        call3(FN_8016A0F0, this as u32, 7, 1);
    }

    1
}

/// Initialises a follower object whose polyline is supplied by the scene
/// helper `0x802C7B70` instead of a point array.
///
/// # Original
/// - Ghidra: `FUN_8000E840` at `0x8000E840` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000e840_FUN_8000e840.c`
///
/// # Arguments
/// * `this` - follower object.
/// * `path` - argument forwarded to `0x802C7B70` as the path/route object.
/// * `pose` - signed pose index; negative means "no pose".
/// * `arg_a` - forwarded in the negative-pose branch.
/// * `arg_b` - scene/model parameter forwarded to `0x80164FD0`.
///
/// # Returns
/// `1` on success, `0` when `0x80164FD0` fails.
///
/// # Side effects
/// Clears the `u8` at `+0x38`; calls `0x80164FD0`, `0x802C7B70`,
/// `0x8016A2A0`, `0x8016A0F0`.
///
/// # Safety
/// `this` valid up to `+0xC36`; `path` accepted by `0x802C7B70`.
///
/// # Confidence
/// medium - mirror image of [`fn_8000de70`] with a different path source.
pub unsafe fn fn_8000e840(
    this: usize,
    path: u32,
    pose: i32,
    arg_a: u32,
    arg_b: u32,
) -> u32 {
    // observed 0x8000E878: `stb r0,0x38(r3)`.
    platform::write_u8(this + 0x38, 0);

    if call3(FN_80164FD0, this as u32, arg_b, 0) == 0 {
        return 0;
    }

    // observed 0x8000E894..0x8000E898: `0x802C7B70(this+8, path)`.
    call2(FN_802C7B70, (this + 8) as u32, path);

    if pose < 0 {
        call3(FN_8016A2A0, this as u32, 0, arg_a);
        call3(FN_8016A0F0, this as u32, 6, 1);
    } else {
        call3(FN_8016A2A0, this as u32, pose as u32, 0);
        call3(FN_8016A0F0, this as u32, 7, 1);
    }

    1
}

/// Sums one `f32` field of an array of 8-byte entries, starting from 0.0.
///
/// Used by the rail code to total segment lengths: `count` is the byte at
/// `+0x38`, the array pointer is `+0x30`, and each entry's first float is
/// added (stride 8).
///
/// # Original
/// - Ghidra: `FUN_8000E030` at `0x8000E030` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000e030_FUN_8000e030.c`
///
/// # Arguments
/// * `this` - object with `count: u8` at `+0x38` and `array: *f32` at `+0x30`.
///
/// # Returns
/// The sum of `array[i].x` for `i in 0..count`, starting at `0.0f`.
///
/// # Side effects
/// None beyond reads.
///
/// # Safety
/// `[this+0x30]` must point at `count` readable entries of 8 bytes.
///
/// # Confidence
/// high for the loop; included as a layout helper for the rail record (it is
/// called from `0x8000F950`, which is a next target, not from the translated
/// set here).
pub unsafe fn fn_8000e030(this: usize) -> f32 {
    let count = platform::read_u8(this + 0x38) as usize;
    let base = platform::read_u32(this + 0x30) as usize;
    let mut sum = read_f32(ZERO_F32_C); // observed: `lfs f1,-0x7F80(r2)` = 0.0f
    let end = base + count * 8;
    let mut p = base;
    while p < end {
        sum += read_f32(p);
        p += 8;
    }
    sum
}

// ===========================================================================
// 3. Record writers and node-collection helper
//
// These three functions do not form one subsystem, but they are the remaining
// clean bodies of the path/route band. `fn_8000f0f0` is byte-for-byte the same
// node-writer shape as `fn_8000d260` above, so it belongs with the path family
// even though its address falls in the neighbouring actor band.
// ===========================================================================

/// Initialises a 0x20-byte route/node record: two seed bytes, two seed
/// halfwords, then zeroes and a constant `0x20` terminator.
///
/// The recovered layout is exactly 32 bytes:
///
/// | offset | width | value |
/// |---|---|---|
/// | `+0x00` | byte | `b0` |
/// | `+0x01` | byte | `b1` |
/// | `+0x02` | halfword | `h2` |
/// | `+0x04` | halfword | `h4` |
/// | `+0x06` | halfword | `0` |
/// | `+0x08` | byte | `0` |
/// | `+0x09` | byte | `0` |
/// | `+0x0A` | halfword | `0` |
/// | `+0x0C` | word | `0` |
/// | `+0x10..+0x19` | ten bytes | `0` |
/// | `+0x1A` | halfword | `0` |
/// | `+0x1C` | word | `0x20` |
///
/// # Original
/// - Ghidra: `FUN_8000EFC0` at `0x8000EFC0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000efc0_FUN_8000efc0.c`
///
/// # Arguments
/// * `record` - writable 0x20-byte record.
/// * `b0`, `b1` - first two seed bytes (role `// unknown:`).
/// * `h2`, `h4` - seed halfwords (role `// unknown:`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// 17 stores covering all 32 bytes; no calls.
///
/// # Safety
/// `record` must be writable for 0x20 bytes.
///
/// # Confidence
/// high - the body is a straight-line sequence of stores; only the four seed
/// field roles are unknown.
pub unsafe fn fn_8000efc0(record: usize, b0: u8, b1: u8, h2: u16, h4: u16) {
    platform::write_u8(record, b0);
    platform::write_u8(record + 1, b1);
    platform::write_u16(record + 2, h2);
    platform::write_u16(record + 4, h4);
    platform::write_u16(record + 6, 0);
    platform::write_u8(record + 8, 0);
    platform::write_u8(record + 9, 0);
    platform::write_u16(record + 0x0A, 0);
    platform::write_u32(record + 0x0C, 0);
    // observed: ten individual byte stores at +0x10..+0x19.
    platform::write_u8(record + 0x10, 0);
    platform::write_u8(record + 0x11, 0);
    platform::write_u8(record + 0x12, 0);
    platform::write_u8(record + 0x13, 0);
    platform::write_u8(record + 0x14, 0);
    platform::write_u8(record + 0x15, 0);
    platform::write_u8(record + 0x16, 0);
    platform::write_u8(record + 0x17, 0);
    platform::write_u8(record + 0x18, 0);
    platform::write_u8(record + 0x19, 0);
    platform::write_u16(record + 0x1A, 0);
    platform::write_u32(record + 0x1C, 0x20);
}

/// Writes one word into every element of a node collection.
///
/// This is the same collection walk as [`fn_8000d3b0`], reduced to a single
/// value: for each element it resolves the node payload through the element's
/// `+0x3C` relative offset, adds the 0x20-byte node header, and calls the
/// node writer `0x8023D3C0` with the caller's second argument and word value.
///
/// # Original
/// - Ghidra: `FUN_8000F0F0` at `0x8000F0F0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f0f0_FUN_8000f0f0.c`
///
/// # Arguments
/// * `handle_addr` - address of the caller's collection handle word. The node
///   API takes this address and reads the handle through it; each iteration
///   re-reads the count the same way.
/// * `param_2` - forwarded to `0x8023D3C0` (selector/parameter).
/// * `param_3` - readable pointer whose word is re-read on every iteration and
///   passed to the node writer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8023B8C0`, `0x8023B860`, `0x8023D3C0` and `0x8023CA80` once per
/// element; writes through the node API.
///
/// # Safety
/// `handle_addr` must point at a valid collection handle, `param_3` must be
/// readable for one word, and each element's `+0x3C` offset must resolve to a
/// valid node payload.
///
/// # Confidence
/// high - the loop and the four calls are explicit; the node API itself is
/// opaque.
pub unsafe fn fn_8000f0f0(handle_addr: usize, param_2: u32, param_3: usize) {
    let mut index: u32 = 0;
    loop {
        // observed: `FUN_8023b8c0(param_1)` - the count is re-read on every
        // iteration through the caller's handle address.
        let count = call1(FN_8023B8C0, handle_addr as u32);
        if index >= count {
            break;
        }

        let element = call2(FN_8023B860, handle_addr as u32, index);
        let rel = platform::read_u32(element as usize + 0x3C);
        let mut node = if rel != 0 {
            element.wrapping_add(rel)
        } else {
            0
        };
        node = node.wrapping_add(0x20);

        // observed: `local_18 = *param_3` on every iteration, then
        // `0x8023D3C0(&node, param_2, &local_18)`.
        let mut point = platform::read_u32(param_3);
        call3(
            FN_8023D3C0,
            &mut node as *mut u32 as u32,
            param_2,
            &mut point as *mut u32 as u32,
        );
        call2(FN_8023CA80, &mut node as *mut u32 as u32, 0);
        index += 1;
    }
}

/// Stores a three-word vector at a fixed stride inside a large record.
///
/// The destination is
/// `this + index_a * 0x3390 + index_b * 0xC + 0x48`. The `0x3390` stride and the
/// `0xC` element stride are observed constants; `// unknown:` which subsystem
/// owns the record (the values look like per-player/per-node vectors, but no
/// caller in the export proves it).
///
/// # Original
/// - Ghidra: `FUN_8000D0A0` at `0x8000D0A0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d0a0_FUN_8000d0a0.c`
///
/// # Arguments
/// * `this` - base of the fixed-stride record.
/// * `value` - readable pointer to three words copied verbatim.
/// * `index_a` - record index (stride `0x3390`).
/// * `index_b` - vector index (stride `0xC`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three word stores; no calls.
///
/// # Safety
/// `value` must be readable for 12 bytes and the computed destination writable
/// for 12 bytes.
///
/// # Confidence
/// high for the arithmetic; low for the record's meaning.
pub unsafe fn fn_8000d0a0(this: usize, value: usize, index_a: i32, index_b: i32) {
    let offset = index_a
        .wrapping_mul(0x3390)
        .wrapping_add(index_b.wrapping_mul(0xC)) as isize;
    let dst = (this as isize + offset + 0x48) as usize;
    platform::write_u32(dst, platform::read_u32(value));
    platform::write_u32(dst + 4, platform::read_u32(value + 4));
    platform::write_u32(dst + 8, platform::read_u32(value + 8));
}

// ===========================================================================
// 4. Follower transform seed and actor-attach wrappers
//
// These three bodies sit between the path-node and rail clusters. `d7e0`
// seeds the follower's per-frame transform fields and then re-enters the
// rail initialiser; `f370`/`f800` are the actor-base wrappers that the same
// object family uses.
// ===========================================================================

/// Seeds two follower transform slots and starts the rail follower with the
/// default speed.
///
/// # Original
/// - Ghidra: `FUN_8000D7E0` at `0x8000D7E0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d7e0_FUN_8000d7e0.c`
///
/// # Arguments
/// * `this` - follower object; `+0xD14` and `+0xD18` are written.
/// * `point` - readable pointer to the first 12-byte rail point. Only its
///   first word is copied into the local one-word buffer.
/// * `pose` - forwarded to [`fn_8000de70`] in `r5`.
/// * `arg_a` - forwarded to [`fn_8000de70`] in `r6`.
/// * `arg_b` - forwarded to [`fn_8000de70`] in `r7`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes two zero words at `+0xD14`/`+0xD18`, then calls [`fn_8000de70`]
/// with speed `8.0f`. That callee stores the speed and clears the rail count.
///
/// # Safety
/// `this` must be valid up to `+0xD18`; `point` must be readable for four
/// bytes; the forwarded arguments must be acceptable to [`fn_8000de70`].
///
/// # Confidence
/// medium - the body and constants are exact. Ghidra shows only `(this,
/// point)`, because the original leaves `r5..r7` untouched for
/// [`fn_8000de70`]; they are exposed here as explicit parameters.
pub unsafe fn fn_8000d7e0(
    this: usize,
    point: usize,
    pose: i32,
    arg_a: u32,
    arg_b: u32,
) {
    // observed 0x8000D7F0..0x8000D814: both fields receive the same zero word.
    let zero = platform::read_u32(U32_ZERO_8042B0E4);
    platform::write_u32(this + 0xD14, zero);
    platform::write_u32(this + 0xD18, zero);

    // observed 0x8000D81C..0x8000D830: copy one word, then `bl 0x8000DE70`
    // with f1 = 8.0f and r3/r4 as below.
    let mut local = [platform::read_u32(point)];
    fn_8000de70(
        this,
        local.as_mut_ptr() as usize,
        read_f32(F32_SPEED_8042B0E8),
        pose,
        arg_a,
        arg_b,
    );
}

/// Prepares the model sub-object at `+8`, runs the shared actor base
/// initialiser, and stores one word at `+0x24` on success.
///
/// This is the sibling copy of `0x8000F270` in `game::actor`; the two bodies
/// are byte-identical and differ only in which translation unit emitted them.
///
/// # Original
/// - Ghidra: `FUN_8000F370` at `0x8000F370` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f370_FUN_8000f370.c`
///
/// # Arguments
/// * `this` - object whose `+8` sub-object is prepared.
/// * `param_2` - word stored at `+0x24` when the base initialiser succeeds
///   (`// unknown:` role).
/// * `param_3` - forwarded to `0x8016A690`.
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
pub unsafe fn fn_8000f370(this: usize, param_2: u32, param_3: u32) -> bool {
    // observed 0x8000F384..0x8000F398: r4 = param_3, r5 = 0x20, r3 = this+8.
    call3(FN_8016A690, (this + 8) as u32, param_3, 0x20);

    // observed 0x8000F3A8: `bl 0x80007510`, the 4-byte thunk to 0x80164FD0.
    let ok = call3(FN_80164FD0, this as u32, (this + 8) as u32, 0);

    if ok != 0 {
        // observed 0x8000F3BC: `stw r31,0x24(r30)`.
        platform::write_u32(this + 0x24, param_2);
    }

    ok != 0
}

/// Returns the object pointed to by virtual slot `+0x14`, advanced by four
/// bytes.
///
/// # Original
/// - Ghidra: `FUN_8000F800` at `0x8000F800` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f800_FUN_8000f800.c`
///
/// # Arguments
/// * `this` - object whose first word is a vtable with a callable entry at
///   `+0x14`.
///
/// # Returns
/// The callee's `r3` plus 4. The original keeps the virtual receiver in `r3`
/// and only adjusts the returned pointer.
///
/// # Side effects
/// One virtual call through `[this][+0x14]`.
///
/// # Safety
/// `this` must point at a readable vtable whose `+0x14` entry is a valid
/// function returning a usable pointer.
///
/// # Confidence
/// high - the two loads, the `ctr` branch and the `+4` adjustment are exact;
/// only the returned object's type is unknown.
pub unsafe fn fn_8000f800(this: usize) -> u32 {
    // observed 0x8000F80C..0x8000F818: `lwz r12,0(r3); lwz r12,0x14(r12);
    // bcctr` with r3 still holding `this`.
    let result = vslot_call0(this as u32, 0x14);

    // observed 0x8000F820: `addi r3,r3,4`.
    result.wrapping_add(4)
}

// ===========================================================================
// 5. Course-select cursor object lifecycle
//
// `0x8000FC30` constructs one object with eight embedded 0x24-byte slots at
// +0x1A0..+0x368. The eight small bodies after it are its deleting-destructor
// family: each returns the pointer unchanged and frees it only when the signed
// second argument is positive.
// ===========================================================================

/// Builds the course-select cursor object and registers it as the singleton.
///
/// The object starts with vtable `0x802FEA28` and a shared base at `+8`. Eight
/// slots are then installed at the offsets in [`CURSOR_SLOT_BASES`]; each slot
/// receives four vtable words, a back-pointer to the owner, a zero word and a
/// call to `0x8015FC90`, after which the slot's first word is replaced by
/// `0x802FEA38`. The body ends by clearing `+0x435` and storing the object in
/// `r13-0x5A84`. The original emits the eight blocks unrolled; the loop below
/// preserves their order and every store.
///
/// # Original
/// - Ghidra: `FUN_8000FC30` at `0x8000FC30` (540 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000fc30_FUN_8000fc30.c`
///
/// # Arguments
/// * `this` - object storage supplied by the caller; the body does not
///   allocate it.
///
/// # Returns
/// Nothing. The original leaves `this` in `r3` for the C++ constructor
/// convention, but Ghidra declares a `void` function.
///
/// # Side effects
/// Writes the object header, every slot field, one byte and the singleton
/// pointer; calls `0x800C8950` once and `0x8015FC90` eight times.
///
/// # Safety
/// `this` must be writable for at least `0x436` bytes and the descriptor
/// objects passed to `0x8015FC90` must be acceptable to it.
///
/// # Confidence
/// high - all offsets and data addresses were checked against the raw
/// instruction words; only the slot object type is inferred.
pub unsafe fn fn_8000fc30(this: usize) {
    // observed 0x8000FC44..0x8000FC58: install the first vtable, then
    // `0x800C8950(this + 8)`.
    platform::write_u32(this, VTABLE_CURSOR);
    call1(FN_800C8950, (this + 8) as u32);

    // observed 0x8000FC84..0x8000FE24: eight identical slot installations.
    for (i, &slot_base) in CURSOR_SLOT_BASES.iter().enumerate() {
        let base = this + slot_base;
        platform::write_u32(base, VTABLE_CURSOR_SLOT_A);
        platform::write_u32(base + 4, VTABLE_CURSOR_SLOT_B);
        platform::write_u32(base + 8, VTABLE_CURSOR_SLOT_C);
        platform::write_u32(base + 0x0C, VTABLE_CURSOR_SLOT_D);
        platform::write_u32(base + 0x10, this as u32);
        platform::write_u32(base + 0x14, 0);
        call4(
            FN_8015FC90,
            (base + 0x18) as u32,
            (base + 4) as u32,
            (base + 8) as u32,
            CURSOR_SLOT_DESCRIPTORS[i],
        );
        platform::write_u32(base, VTABLE_CURSOR_ALT);
    }

    // observed 0x8000FE30..0x8000FE34: clear the flag byte and publish the
    // singleton at r13-0x5A84.
    platform::write_u8(this + 0x435, 0);
    platform::write_u32(G_CURSOR_OBJECT, this as u32);
}

/// Deleting destructor: frees the object when the signed flag is positive and
/// returns the pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_8000FE90` at `0x8000FE90` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000fe90_FUN_8000fe90.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
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
/// high - the two guards and the return are exact; the body is identical to
/// `0x8000FBF0` in `game::actor`.
pub unsafe fn fn_8000fe90(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}

/// Deleting destructor; second emitted copy of `0x8000FE90`.
///
/// # Original
/// - Ghidra: `FUN_8000FED0` at `0x8000FED0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000fed0_FUN_8000fed0.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// Same contract as [`fn_8000fe90`].
///
/// # Confidence
/// high - byte-identical to [`fn_8000fe90`].
pub unsafe fn fn_8000fed0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}

/// Deleting destructor that first runs the partial destructor `0x8015F9F0`.
///
/// # Original
/// - Ghidra: `FUN_8000FF10` at `0x8000FF10` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000ff10_FUN_8000ff10.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x8015F9F0(this, 0)` whenever `this` is non-null, then
/// `0x802B9280(this)` when `release > 0`.
///
/// # Safety
/// `this` must be a valid object for `0x8015F9F0` and, when freed, for
/// `0x802B9280`.
///
/// # Confidence
/// high - both calls and the signed guard are explicit.
pub unsafe fn fn_8000ff10(this: usize, release: i32) -> usize {
    if this != 0 {
        call2(FN_8015F9F0, this as u32, 0);
        if release > 0 {
            call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Deleting destructor: frees the object when the signed flag is positive and
/// returns the pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_8000FFB0` at `0x8000FFB0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000ffb0_FUN_8000ffb0.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// Same contract as [`fn_8000fe90`].
///
/// # Confidence
/// high - byte-identical to [`fn_8000fe90`].
pub unsafe fn fn_8000ffb0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}

/// Deleting destructor: frees the object when the signed flag is positive and
/// returns the pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_8000FFF0` at `0x8000FFF0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000fff0_FUN_8000fff0.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// Same contract as [`fn_8000fe90`].
///
/// # Confidence
/// high - byte-identical to [`fn_8000fe90`].
pub unsafe fn fn_8000fff0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}

/// Deleting destructor: frees the object when the signed flag is positive and
/// returns the pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_80010030` at `0x80010030` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80010030_FUN_80010030.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// Same contract as [`fn_8000fe90`].
///
/// # Confidence
/// high - byte-identical to [`fn_8000fe90`].
pub unsafe fn fn_80010030(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}

/// Deleting destructor that first releases the sub-object at `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_80010070` at `0x80010070` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80010070_FUN_80010070.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x8015F9F0(this+0x18, 0)` when both pointers are non-null, then
/// `0x802B9280(this)` when `release > 0`.
///
/// # Safety
/// `this+0x18` must be a valid object for `0x8015F9F0`; the owner must be
/// freeable when `release > 0`.
///
/// # Confidence
/// high - the `addic. r3,r3,24` guard and the two calls are exact.
pub unsafe fn fn_80010070(this: usize, release: i32) -> usize {
    if this != 0 {
        if this + 0x18 != 0 {
            call2(FN_8015F9F0, (this + 0x18) as u32, 0);
        }
        if release > 0 {
            call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Deleting destructor that first releases the sub-object at `+0x18`.
///
/// The original tests `this` twice (`0x800100F0` and `0x800100F4`); the second
/// test can never fail once the first passed, so it has no observable effect.
///
/// # Original
/// - Ghidra: `FUN_800100D0` at `0x800100D0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800100d0_FUN_800100d0.c`
///
/// # Arguments
/// * `this` - object pointer; may be null.
/// * `release` - signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x8015F9F0(this+0x18, 0)` when both pointers are non-null, then
/// `0x802B9280(this)` when `release > 0`.
///
/// # Safety
/// Same contract as [`fn_80010070`].
///
/// # Confidence
/// high - the duplicate guard is the only difference from [`fn_80010070`].
pub unsafe fn fn_800100d0(this: usize, release: i32) -> usize {
    if this != 0 {
        if this != 0 {
            if this + 0x18 != 0 {
                call2(FN_8015F9F0, (this + 0x18) as u32, 0);
            }
        }
        if release > 0 {
            call1(FN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 6. Course-select cursor state writers
//
// Every function below addresses the same object built by [`fn_8000fc30`]:
// `+0x08` is the shared base, `+0x1A0`/`+0x1DC`/`+0x218` are the slot objects,
// and `+0x434..+0x44E` are per-player state bytes. The slot virtual method at
// `+0x18` receives `(slot, descriptor)`; the descriptor constants at
// `0x803516B8..0x803518F8` are runtime-filled and select the visible or
// alternate state.
// ===========================================================================

/// Computes and stores the cursor transform fields at `+0x98..+0xA8`.
///
/// When the bool query `0x801DD1D0` returns zero, the x field is derived from
/// the player count: `0.5 * (count - 330.0)`, while the y/z extents become
/// `640.0`/`330.0`. Otherwise all four floats stay at `0.0`. The last byte is
/// set to `1` on the computed path and `0` otherwise.
///
/// # Original
/// - Ghidra: `FUN_800105C0` at `0x800105C0` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800105c0_FUN_800105c0.c`
///
/// # Arguments
/// * `this` - cursor object; `+0x98..+0xA8` are written.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Reads the global player record and calls `0x801DD1D0`; writes five fields.
///
/// # Safety
/// `[0x8042A480]` must point at a readable record whose first word points at a
/// structure with a readable `u16` at `+6`.
///
/// # Confidence
/// high for the arithmetic (the constants at `0x8042B154..0x8042B160` were
/// read from the image); medium for the player-record meaning.
pub unsafe fn fn_800105c0(this: usize) {
    // observed 0x800105C8..0x800105EC: the four floats start at 0.0 and the
    // flag byte starts at 0.
    let zero = read_f32(F32_ZERO_8042B154);
    let mut local_24 = zero;
    let mut local_20 = zero;
    let mut local_1c = zero;

    // observed 0x800105F0: `bl 0x800B54B0`, a tail thunk to 0x801DD1D0.
    let computed = call0_r(FN_801DD1D0) == 0;

    if computed {
        // observed 0x800105FC..0x80010644: `u16` count at `[[r13-0x5200]]+6`,
        // then `0.5 * (count - 330.0)` and the 640.0/330.0 extents.
        let players = platform::read_u32(G_CURSOR_PLAYERS) as usize;
        let record = platform::read_u32(players) as usize;
        let count = platform::read_u16(record + 6) as f32;
        local_20 = read_f32(F32_640_8042B160);
        local_1c = read_f32(F32_330_8042B15C);
        local_24 = read_f32(F32_HALF_8042B158) * (count - local_1c);
    }

    // observed 0x8001065C..0x8001066C: store the four floats and the flag.
    write_f32(this + 0x98, zero);
    write_f32(this + 0x9C, local_24);
    write_f32(this + 0xA0, local_20);
    write_f32(this + 0xA4, local_1c);
    platform::write_u8(this + 0xA8, computed as u8);
}

/// Advances or backs the cursor position by 20 and mirrors it into eight
/// element bytes.
///
/// If `+0x430` is zero the position moves down by 20 and clamps at 0;
/// otherwise it moves up by 20 and clamps at 255. The resulting byte is then
/// written to `+0xB8` of the eight objects at `+0x384`, `+0x3F0`, `+0x388`,
/// `+0x3F4`, `+0x38C`, `+0x3F8`, `+0x390` and `+0x3FC`.
///
/// # Original
/// - Ghidra: `FUN_800108C0` at `0x800108C0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800108c0_FUN_800108c0.c`
///
/// # Arguments
/// * `this` - cursor object.
///
/// # Returns
/// Nothing; returns early when `+0x42C` already equals `+0x430`.
///
/// # Side effects
/// One word store at `+0x42C`, eight byte stores through the element pointers.
/// The original emits the eight `stb` groups individually; the loop preserves
/// their order.
///
/// # Safety
/// Each element pointer read from `+0x384..+0x3FC` must be valid up to
/// `+0xB8`.
///
/// # Confidence
/// high - the signed clamps and all eight offsets are explicit in the
/// instructions.
pub unsafe fn fn_800108c0(this: usize) {
    let current = platform::read_i32(this + 0x42C);
    let target = platform::read_i32(this + 0x430);
    if current == target {
        return;
    }

    // observed 0x800108D0..0x80010904: two signed clamp directions.
    let new_value = if target == 0 {
        (current - 20).max(0)
    } else {
        (current + 20).min(255)
    };
    platform::write_i32(this + 0x42C, new_value);

    // observed 0x80010908..0x80010948: eight `stb ...,0xB8(...)`.
    let byte = new_value as u8;
    for &offset in &[0x384usize, 0x3F0, 0x388, 0x3F4, 0x38C, 0x3F8, 0x390, 0x3FC] {
        let element = platform::read_u32(this + offset) as usize;
        platform::write_u8(element + 0xB8, byte);
    }
}

/// Tests whether player `index` is present in the cursor bit mask at `+0x434`.
///
/// The mask table at `0x8042B170` is `{1, 2, 4, 8}`. The original computes
/// `(-(mask & bit) | (mask & bit)) >> 31`, which is 1 exactly when the AND is
/// non-zero.
///
/// # Original
/// - Ghidra: `FUN_80010F40` at `0x80010F40` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80010f40_FUN_80010f40.c`
///
/// # Arguments
/// * `this` - cursor object with the bit mask at `+0x434`.
/// * `index` - player slot, observed as 0..3. No bounds check exists in the
///   original.
///
/// # Returns
/// `1` when the player bit is set, `0` otherwise.
///
/// # Side effects
/// None beyond reads.
///
/// # Safety
/// `index` must be 0..3 so the table read stays inside the four-byte mask.
///
/// # Confidence
/// high - the table bytes and the neg/or/sign-bit sequence were checked in the
/// image.
pub unsafe fn fn_80010f40(this: usize, index: u32) -> u32 {
    let bit = platform::read_u8(PLAYER_BIT_TABLE_8042B170 + index as usize);
    let present = platform::read_u8(this + 0x434) & bit;
    (present != 0) as u32
}

/// Shows or hides the first cursor slot according to the `+0x437` flag.
///
/// When the flag is clear the function asks `0x800C9610` whether element 2 is
/// present and, if not, sends descriptor `0x80351738` to the slot's virtual
/// method. When the flag is set it hides element 2 first and then sends
/// descriptor `0x80351778`.
///
/// # Original
/// - Ghidra: `FUN_80011020` at `0x80011020` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80011020_FUN_80011020.c`
///
/// # Arguments
/// * `this` - cursor object; the first slot starts at `+0x1A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call `0x800C9490`/`0x800C9610` and always calls the slot virtual method
/// at `+0x18` on one path.
///
/// # Safety
/// `+0x1A0` must be a valid slot object whose vtable has a callable `+0x18`
/// entry.
///
/// # Confidence
/// high - the branch order and both descriptor addresses are explicit.
pub unsafe fn fn_80011020(this: usize) {
    if platform::read_u8(this + 0x437) == 0 {
        if call2(FN_800C9610, (this + 8) as u32, 2) == 0 {
            vslot_call2((this + 0x1A0) as u32, 0x18, CURSOR_DESC_80351738);
        }
    } else {
        call2(FN_800C9490, (this + 8) as u32, 2);
        vslot_call2((this + 0x1A0) as u32, 0x18, CURSOR_DESC_80351778);
    }
}

/// Marks the first cursor slot ready and resets its state.
///
/// # Original
/// - Ghidra: `FUN_80011120` at `0x80011120` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80011120_FUN_80011120.c`
///
/// # Arguments
/// * `this` - cursor object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Sets bytes `+0x449 = 1` and `+0x437 = 0`; calls
/// `0x800C9490(this+8, 5)` and `0x800C92F0(this+8, 8, 0)`.
///
/// # Safety
/// `this` must be valid through `+0x449`.
///
/// # Confidence
/// high - four stores/calls, all operands explicit.
pub unsafe fn fn_80011120(this: usize) {
    platform::write_u8(this + 0x449, 1);
    platform::write_u8(this + 0x437, 0);
    call2(FN_800C9490, (this + 8) as u32, 5);
    call3(FN_800C92F0, (this + 8) as u32, 8, 0);
}

/// Shows or hides the first cursor slot according to the `+0x436` flag.
///
/// When the flag is clear the function clears `+0x438` and sends descriptor
/// `0x803516B8` after confirming element 8 is absent; when set it hides
/// element 8 and sends `0x803516F8`.
///
/// # Original
/// - Ghidra: `FUN_80011180` at `0x80011180` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80011180_FUN_80011180.c`
///
/// # Arguments
/// * `this` - cursor object; the first slot starts at `+0x1A0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May call `0x800C9490`/`0x800C9610`; writes `+0x438`; calls the slot
/// virtual method at `+0x18`.
///
/// # Safety
/// `+0x1A0` must be a valid slot object with a callable `+0x18` entry.
///
/// # Confidence
/// high - the flag, the element index and both descriptors are explicit.
pub unsafe fn fn_80011180(this: usize) {
    if platform::read_u8(this + 0x436) == 0 {
        if call2(FN_800C9610, (this + 8) as u32, 8) == 0 {
            platform::write_u8(this + 0x438, 0);
            vslot_call2((this + 0x1A0) as u32, 0x18, CURSOR_DESC_803516B8);
        }
    } else {
        call2(FN_800C9490, (this + 8) as u32, 8);
        vslot_call2((this + 0x1A0) as u32, 0x18, CURSOR_DESC_803516F8);
    }
}

/// Clears the group flag and applies the `+0x43D` state to elements 0 and 1.
///
/// # Original
/// - Ghidra: `FUN_800112C0` at `0x800112C0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800112c0_FUN_800112c0.c`
///
/// # Arguments
/// * `this` - cursor object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x43A = 0`; calls `0x800C92F0(this+8, 0, +0x43D)` and the same
/// for element 1.
///
/// # Safety
/// `this` must be valid through `+0x43D`.
///
/// # Confidence
/// high - both calls and the stored value are explicit.
pub unsafe fn fn_800112c0(this: usize) {
    platform::write_u8(this + 0x43A, 0);
    // observed 0x800112E0 and 0x800112EC: the byte is loaded again before the
    // second call, so it is re-read here as well.
    call3(
        FN_800C92F0,
        (this + 8) as u32,
        0,
        platform::read_u8(this + 0x43D) as u32,
    );
    call3(
        FN_800C92F0,
        (this + 8) as u32,
        1,
        platform::read_u8(this + 0x43D) as u32,
    );
}

/// Shows or hides the second cursor slot according to the `+0x43B` flag.
///
/// # Original
/// - Ghidra: `FUN_80011310` at `0x80011310` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80011310_FUN_80011310.c`
///
/// # Arguments
/// * `this` - cursor object; the second slot starts at `+0x1DC`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800C9490`/`0x800C9610` on elements 0 and 1 and sends descriptor
/// `0x80351838` or `0x80351878` to the slot virtual method at `+0x18`.
///
/// # Safety
/// `+0x1DC` must be a valid slot object with a callable `+0x18` entry.
///
/// # Confidence
/// high - the two-element test and both descriptors are explicit.
pub unsafe fn fn_80011310(this: usize) {
    if platform::read_u8(this + 0x43B) == 0 {
        if call2(FN_800C9610, (this + 8) as u32, 0) == 0 {
            if call2(FN_800C9610, (this + 8) as u32, 1) == 0 {
                vslot_call2((this + 0x1DC) as u32, 0x18, CURSOR_DESC_80351838);
            }
        }
    } else {
        call2(FN_800C9490, (this + 8) as u32, 0);
        call2(FN_800C9490, (this + 8) as u32, 1);
        vslot_call2((this + 0x1DC) as u32, 0x18, CURSOR_DESC_80351878);
    }
}

/// Shows cursor elements 3 and 4.
///
/// # Original
/// - Ghidra: `FUN_800113E0` at `0x800113E0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800113e0_FUN_800113e0.c`
///
/// # Arguments
/// * `this` - cursor object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800C9380(this+8, 3)` and `0x800C9380(this+8, 4)`.
///
/// # Safety
/// `this+8` must be the shared cursor base expected by `0x800C9380`.
///
/// # Confidence
/// high - two calls, two literal indices.
pub unsafe fn fn_800113e0(this: usize) {
    call2(FN_800C9380, (this + 8) as u32, 3);
    call2(FN_800C9380, (this + 8) as u32, 4);
}

/// Clears the second group flag and resets elements 3, 4, 6 and 7.
///
/// # Original
/// - Ghidra: `FUN_80011470` at `0x80011470` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80011470_FUN_80011470.c`
///
/// # Arguments
/// * `this` - cursor object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x43B = 0`; calls `0x800C9490` for 3 and 4, then
/// `0x800C92F0` for 6 and 7 with zero.
///
/// # Safety
/// `this+8` must be the shared cursor base expected by both callees.
///
/// # Confidence
/// high - four calls and the cleared byte are explicit.
pub unsafe fn fn_80011470(this: usize) {
    platform::write_u8(this + 0x43B, 0);
    call2(FN_800C9490, (this + 8) as u32, 3);
    call2(FN_800C9490, (this + 8) as u32, 4);
    call3(FN_800C92F0, (this + 8) as u32, 6, 0);
    call3(FN_800C92F0, (this + 8) as u32, 7, 0);
}

/// Shows or hides the second cursor slot according to the `+0x43A` flag.
///
/// # Original
/// - Ghidra: `FUN_800114E0` at `0x800114E0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800114e0_FUN_800114e0.c`
///
/// # Arguments
/// * `this` - cursor object; the second slot starts at `+0x1DC`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800C9490`/`0x800C9610` on elements 6 and 7; writes `+0x43C` on
/// the normal path; sends descriptor `0x803517B8` or `0x803517F8` to the slot
/// virtual method at `+0x18`.
///
/// # Safety
/// `+0x1DC` must be a valid slot object with a callable `+0x18` entry.
///
/// # Confidence
/// high - both element tests and both descriptors are explicit.
pub unsafe fn fn_800114e0(this: usize) {
    if platform::read_u8(this + 0x43A) == 0 {
        if call2(FN_800C9610, (this + 8) as u32, 6) == 0 {
            if call2(FN_800C9610, (this + 8) as u32, 7) == 0 {
                platform::write_u8(this + 0x43C, 0);
                vslot_call2((this + 0x1DC) as u32, 0x18, CURSOR_DESC_803517B8);
            }
        }
    } else {
        call2(FN_800C9490, (this + 8) as u32, 6);
        call2(FN_800C9490, (this + 8) as u32, 7);
        vslot_call2((this + 0x1DC) as u32, 0x18, CURSOR_DESC_803517F8);
    }
}

/// Shows the third cursor slot when the group is ready and player 0 is absent
/// or another member is present.
///
/// # Original
/// - Ghidra: `FUN_800115C0` at `0x800115C0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800115c0_FUN_800115c0.c`
///
/// # Arguments
/// * `this` - cursor object; the third slot starts at `+0x218`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_80010f40`] only when `+0x43F` is zero; on the accepted path
/// sends descriptor `0x803518F8` to the slot virtual method at `+0x18`.
///
/// # Safety
/// `+0x218` must be a valid slot object with a callable `+0x18` entry.
///
/// # Confidence
/// high - the short-circuit order is visible in the instructions.
pub unsafe fn fn_800115c0(this: usize) {
    if platform::read_u8(this + 0x43F) == 0 {
        // observed 0x800115E4..0x800115F8: the `||` short-circuits, so the
        // `+0x43E` byte is read only when player 0 is absent.
        let show = fn_80010f40(this, 0) != 0 || platform::read_u8(this + 0x43E) != 0;
        if show {
            vslot_call2((this + 0x218) as u32, 0x18, CURSOR_DESC_803518F8);
        }
    }
}

/// Mirrors the selected course id into `+0x404` and restarts two cursor
/// animations when it changes.
///
/// The new id is read from `[[r13-0x5730]+4]+8`. On a change the body stores
/// it at `+0x404`, reads a frame stamp from `0x800CDC60`, sets the course-state
/// byte `+0x3D9` to `id != 0`, and calls `0x800C9A60` for the two panes at
/// `+0x3CC` and `+0x3D0` with actions `0x0A`/`0x0E` (depending on `+0x445`)
/// and `0x0F`.
///
/// # Original
/// - Ghidra: `FUN_800126A0` at `0x800126A0` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800126a0_FUN_800126a0.c`
///
/// # Arguments
/// * `this` - cursor object.
///
/// # Returns
/// Nothing; returns early when the id is unchanged.
///
/// # Side effects
/// Writes `+0x404`, the global `+0x3D9` byte, and starts two pane animations.
///
/// # Safety
/// `[0x80429F50]` and its `+4` pointer must be readable, and both pane
/// pointers at `+0x3CC`/`+0x3D0` must be valid animation targets.
///
/// # Confidence
/// high - the id comparison, branch order and all five call arguments were
/// checked in the instructions.
pub unsafe fn fn_800126a0(this: usize) {
    let manager = platform::read_u32(G_COURSE_MANAGER) as usize;
    let source = platform::read_u32(manager + 4) as usize;
    let course_id = platform::read_i32(source + 8);

    if course_id != platform::read_i32(this + 0x404) {
        platform::write_i32(this + 0x404, course_id);

        // observed 0x800126D4: the stamp helper ignores r3 and returns the
        // global frame counter.
        let stamp = call0_r(FN_800CDC60);
        let state = platform::read_u32(G_COURSE_STATE) as usize;
        platform::write_u8(state + 0x3D9, (course_id != 0) as u8);

        // observed 0x800126F4..0x80012754: `+0x445 == 0` selects action 10,
        // otherwise action 14; the second pane always uses action 15.
        let action = if platform::read_u8(this + 0x445) == 0 {
            0x0A
        } else {
            0x0E
        };
        call5(FN_800C9A60, (this + 0x3CC) as u32, stamp, 4, action, 0);
        call5(FN_800C9A60, (this + 0x3D0) as u32, stamp, 4, 0x0F, 0);
    }
}

// ===========================================================================
// Wave 276 lane E (`_E276`) - gap batch 6, two deleting destructors
// ===========================================================================
//
// Two untabled gap bodies from the wave-266 candidate set, re-verified against
// `work/dol/main.elf` with the current documented extents. Selection:
// `work/wave276/e_pick.mjs`; ELF re-walk and disassembly: `e_verify.mjs` and
// `e_verify_stdout.txt`. Both are byte-identical to the already-translated
// deleting destructors in this module and sit directly before one of them.

// ---------------------------------------------------------------------------
// 0x8000FE50 - deleting destructor, copy before 0x8000FE90
// ---------------------------------------------------------------------------

/// Deleting destructor: frees the object when the signed flag is positive and
/// returns the pointer unchanged.
///
/// Byte-identical to [`fn_8000fe90`], which starts at `0x8000FE90` immediately
/// after this body. It is the third emitted copy of the same C++ deleting
/// destructor in this band.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8000FE50`.
/// - Extent: `0x8000FE50..0x8000FE90` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x8000FE8C`; one zero word precedes it at `0x8000FE4C` and
///   the next body (`fn_8000fe90`) starts at `0x8000FE90`.
/// - Reach: no `b` or `bl` targets it; the isolated word `0x802FEAD0` (first
///   word of a `{code, 0, 0}` descriptor) names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq to the return); `cmpi cr0,0,r4,0`;
///   `bc 4,1,+0x08` (branch when `release <= 0`); `bl 0x802B9280`;
///   `mr r3,r31`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: object pointer; may be null.
/// - `release`: signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// A non-null `this` with a positive `release` must be a heap object that
/// `0x802B9280` can free.
///
/// # Confidence
/// high - the two guards, the free and the return are exact and match
/// [`fn_8000fe90`] word for word.
pub unsafe fn fn_8000fe50(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}

// ---------------------------------------------------------------------------
// 0x8000FF70 - deleting destructor, copy before 0x8000FFB0
// ---------------------------------------------------------------------------

/// Deleting destructor; another emitted copy of `0x8000FE90`.
///
/// Same contract as [`fn_8000fe50`], placed directly before [`fn_8000ffb0`]
/// (which starts at `0x8000FFB0`). The descriptor table at `0x802FEB20` holds
/// `0x8000FF70` at its first word and `0x8000FFB0` at `+0x18`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8000FF70`.
/// - Extent: `0x8000FF70..0x8000FFB0` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x8000FFAC`; two zero words precede it at
///   `0x8000FF68..0x8000FF70` and the next body starts at `0x8000FFB0`.
/// - Reach: no `b` or `bl` targets it; the isolated word `0x802FEB20` (first
///   word of a `{code, 0, 0}` descriptor) names it.
/// - Observed: the same sequence as [`fn_8000fe50`], with `bl 0x802B9280`
///   guarded by `this != 0` (`bc 12,2`) and `release > 0` (`bc 4,1`).
///
/// # Arguments
/// - `this`: object pointer; may be null.
/// - `release`: signed flag; freeing happens only when `release > 0`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Calls `0x802B9280(this)` on the guarded path.
///
/// # Safety
/// Same contract as [`fn_8000fe50`].
///
/// # Confidence
/// high - byte-identical to [`fn_8000fe50`].
pub unsafe fn fn_8000ff70(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        call1(FN_802B9280, this as u32);
    }
    this
}
