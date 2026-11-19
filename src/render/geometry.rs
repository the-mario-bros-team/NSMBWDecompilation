//! Object construction, vertex grids, float copies and 2D vector math.
//!
//! **What lives here.** Nine clusters, all from the `0x80005000..0x8000FFFF`
//! band of `main.dol`:
//!
//! * **Geometry primitives** - [`store_four_lane_vertices`] writes one index of
//!   four parallel `Vec3` arrays; [`copy_12_floats`] moves twelve floats into a
//!   model slot; [`fn_8000b320`] publishes a single 2-float point; and
//!   [`vec2_sub`], [`vec2_differs_from`], [`vec2_length`] are the 2D helpers
//!   that [`crate::game::path`] reuses to drop duplicate polyline points.
//! * **Scene sub-object attachment** - [`fn_8000aa70`], [`fn_8000aec0`] and
//!   [`fn_8000b370`] validate an owner through `0x80164FD0`, push one or two
//!   2-float points into the sub-objects at `+0x08`/`+0x28`, then select a
//!   state (selector `6` for "detached", `7` for "attached").
//! * **Screen-space rectangle** - [`fn_80007520`] opens a `1000x1000` GX draw
//!   surface and writes the six rect floats of the resulting object.
//! * **Named-material registration** - [`fn_80007610`] registers the twenty
//!   small-data entries at `0x80427688` in a render resource and sets its
//!   `+0x30` scale word.
//! * **Object construction / destruction** - the `0x80006870..0x80006F70`
//!   cluster ([`fn_80006870`], [`fn_800068b0`], [`fn_800069a0`],
//!   [`fn_80006a20`], [`fn_80006aa0`], [`fn_80006af0`], [`fn_80006b60`],
//!   [`fn_80006bf0`], [`fn_80006c50`], [`fn_80006cd0`], [`fn_80006df0`],
//!   [`fn_80006e30`], [`fn_80006f70`], [`fn_80006fa0`]). All install a vtable
//!   and free through `0x802B9280`; the class identities are not recoverable
//!   because the images carry no RTTI names for them.
//! * **Global collection lookups** - [`fn_80007300`], [`fn_80007320`],
//!   [`fn_800073d0`], [`fn_80007470`] walk the linked list returned by a
//!   virtual slot and compare each node against one small-data anchor.
//! * **Band-completion leaves** - the seven sub-16-byte entries the first pass
//!   skipped: [`fn_80006790`] (empty), [`fn_800067a0`] (frame-callback
//!   drain), [`fn_80006a40`] (inline-member pointer), [`fn_80006be0`]
//!   (resource-create wrapper), [`fn_80006ea0`] (sorted-list insert),
//!   [`fn_80007220`] (child getter) and [`fn_80007510`] (actor-init thunk).
//!   Three are 4-byte tail branches; their bodies are one forwarder each.
//! * **View transform and rail polyline** - [`fn_8000d740`] builds the
//!   (`// inferred:`) sine/cosine view words at `+0xD0C..+0xD28`; [`fn_8000d820`]
//!   forwards to it and then to the rail builder. [`fn_8000f950`] tessellates a
//!   two-point curve, accumulates segment lengths and emits `{x, y, u16}`
//!   records; [`fn_8000fba0`] is its wrapper into
//!   [`crate::game::path::fn_8000df60`].
//!   [`fn_8000e910`] fills one quad's matrix from the `u16` fields of the
//!   global render object, [`fn_8000edf0`] drives it with two lazy-initialised
//!   2-float constants, and [`fn_8000edd0`] / [`fn_8000f2e0`] are 20-byte
//!   virtual-slot forwarders (`+0x38`/`+0x0C` and `+0x24`/`+0x18`).
//! * **GX ribbon and particle emitters** - the four entries the earlier passes
//!   deferred because their Ghidra exports dropped the middle of the FIFO
//!   loops. Wave 110 decoded them from the raw words instead:
//!   [`fn_80007de0`] draws 200 rotated billboard quads from the record array at
//!   `+0x780`, [`fn_8000c5c0`] draws a four-corner ribbon strip from the arrays
//!   at `+0x48`/`+0x33D8`/`+0x6768`/`+0x9AF8`, [`fn_8000ca50`] draws the
//!   six-vertex variant of that strip, and [`fn_8000e060`] draws one long
//!   triangle strip made of the same four arrays plus a rounded end cap.
//!
//! **Original addresses.** `0x80006790`, `0x800067A0`, `0x80006870`,
//! `0x800068B0`, `0x800069A0`, `0x80006A20`, `0x80006A40`, `0x80006AA0`,
//! `0x80006AF0`, `0x80006B60`, `0x80006BE0`, `0x80006BF0`, `0x80006C50`,
//! `0x80006CD0`, `0x80006DF0`, `0x80006E30`, `0x80006EA0`, `0x80006F70`,
//! `0x80006FA0`, `0x80007220`, `0x80007230`, `0x80007300`, `0x80007320`,
//! `0x800073D0`, `0x80007470`, `0x80007510`, `0x80007520`, `0x800075D0`,
//! `0x80007610`, `0x8000AA70`, `0x8000AEC0`, `0x8000AE50`, `0x8000B320`,
//! `0x8000B370`, plus the out-of-band `0x8000D0D0`, `0x8000D740`,
//! `0x8000D820`, `0x8000D870`, `0x8000D8A0`, `0x8000E910`, `0x8000EDD0`,
//! `0x8000EDF0`, `0x8000F2E0`, `0x8000F900`, `0x8000F950`, `0x8000FBA0` that
//! were promoted here by the geometry lanes, plus
//! `0x80007DE0`, `0x8000C5C0`, `0x8000CA50` and `0x8000E060`, which wave 110
//! translated from the raw instruction stream after the earlier passes had
//! rejected their Ghidra exports (those exports drop every vertex store except
//! one per loop iteration).
//!
//! **Why it exists.** These are the arithmetic, storage and object-plumbing
//! primitives that the quad, rail and scene code share. `main.dol` has no
//! symbol table, so every name here is either the Ghidra `FUN_<address>` name or
//! a description explicitly marked `// inferred:`.
//!
//! **Dependencies.** `crate::platform` for big-endian loads/stores and indirect
//! calls, `crate::boot::entry::SDA1_BASE` for the `r13` small-data base,
//! [`crate::render::gx::fn_8000ebf0`] for the quad corner emitter and
//! [`crate::game::path::fn_8000df60`] for the follower polyline consumer.
//! All cross-lane callees are called by absolute address and documented at
//! their constants below; nothing is invented.
//!
//! **Start here.** [`store_four_lane_vertices`] for the vertex layout, then
//! [`fn_8000b370`] for the scene attachment, then [`vec2_sub`] for the math.
//!
//! **Out-of-band kernels.** The Wave 228 lane F block at the end of the file
//! (`0x80254BE0`..`0x80256F84`) translates 38 of the 42 pointers in the six
//! seven-entry nw4r scene-kernel tables at `0x802F8EF0`..`0x802F8F98`; the other
//! four slots already live in [`crate::runtime::os`]. Every kernel writes the
//! same eight-float two-column transform block, which is why they belong beside
//! the `0x80253xxx` matrix helpers already translated here.
//!
//! **Glossary.** `Vec3` here means three consecutive big-endian `f32` values
//! (`x`, `y`, `z`), 12 bytes; it is a layout description, not a Rust type.
//! `SDA` = small-data area, the PowerPC `r13`-relative globals. `GX` = the Wii
//! GPU. See `docs/GLOSSARY.md`.

use crate::boot::entry::SDA1_BASE;
use crate::platform;
use crate::platform::call6;

// ---------------------------------------------------------------------------
// Cross-lane callees (opaque here).
//
// Each constant is the entry address of a function owned by another layer.
// They are declared so a reader can see which external effects a translation
// has without leaving this file. Where a callee's own Ghidra export gave a
// signature, it is quoted; otherwise the argument shape is the one observed at
// the call site.
// ---------------------------------------------------------------------------

/// Configures a sub-object built by `0x80164FD0`:
/// `(sub, &vec2, a, b, c, d)`. The four flags select which components are
/// copied; this file always passes `1` for "enabled" and `0`/`1` for "flip".
///
/// Ghidra: 802c7c70 `FUN_802c7c70.c`
const FN_802C7C70: usize = 0x802C_7C70;

/// Double-precision helper applied to a squared 2D distance; the image-wide
/// idiom (`sqrt`) is a float that is widened again.
///
/// Ghidra: 802be470 `FUN_802be470.c`
/// `// inferred:` square root - the callee narrows its result to `f32` and the
/// only caller feeds it `dx*dx + dy*dy`.
const FN_802BE470: usize = 0x802B_E470;

/// `(this, key, 0) -> 0/1`: validates an owner object before the scene
/// sub-objects may be written. Every caller in this file tests the result.
///
/// Ghidra: 80164fd0 `FUN_80164fd0.c` (`undefined4 FUN_80164fd0(int,int,undefined1*)`)
const FN_80164FD0: usize = 0x8016_4FD0;

/// `()`: runs the registered frame callbacks, drains the list at
/// `0x80377AB0`, then calls the matching teardown helper.
///
/// This is the target of the 4-byte tail branch at `0x800067A0`. Ghidra's
/// export for the thunk address contains this body because it followed the
/// branch.
///
/// Ghidra: 80163280 `FUN_80163280.c`
const FN_80163280: usize = 0x8016_3280;

/// `(node)`: inserts `node` into the list at `0x80377AB0`, ordered by the
/// byte at `node + 0x0C`.
///
/// This is the target of the 4-byte tail branch at `0x80006EA0`. Ghidra's
/// export for the thunk address contains this body because it followed the
/// branch.
///
/// Ghidra: 80163850 `FUN_80163850.c`
const FN_80163850: usize = 0x8016_3850;

/// `(object, 0, value)` / `(object, value, 0)`: applies one of two state
/// payloads. Called before the selector write below in every attachment.
///
/// Ghidra: 8016a2a0 `FUN_8016a2a0.c` (`void FUN_8016a2a0(int,undefined4,undefined4)`)
const FN_8016A2A0: usize = 0x8016_A2A0;

/// `(object, selector, enable)`: writes one selector/enable pair. Ghidra's
/// export declares one argument, but both call sites set `r4`/`r5`, so the
/// three-argument form below is the observed ABI.
///
/// Ghidra: 8016a0f0 `FUN_8016a0f0.c`
const FN_8016A0F0: usize = 0x8016_A0F0;

/// `(this, handler, 0, &byte) -> u32`: queues a one-byte message for the code
/// address `handler`. Returns `1` when the queue accepted it.
///
/// Ghidra: 80163e10 `FUN_80163e10.c`
const FN_80163E10: usize = 0x8016_3E10;

/// `(sub)`: installs the sub-object vtable for the `0x802FE410` family.
///
/// Ghidra: 802b6620 `FUN_802b6620.c`
const FN_802B6620: usize = 0x802B_6620;

/// `(sub, 0)`: tears down a member object.
///
/// Ghidra: 80229030 `FUN_80229030.c`
const FN_80229030: usize = 0x8022_9030;

/// `(sub, 0)`: tears down the `0x802FE410` member object.
///
/// Ghidra: 802b63b0 `FUN_802b63b0.c`
const FN_802B63B0: usize = 0x802B_63B0;

/// `(this)`: the raw `operator delete` used by every destructor in this file.
///
/// Ghidra: 802b9280 `FUN_802b9280.c`
const FN_802B9280: usize = 0x802B_9280;

/// `(sub)`: installs vtable `0x80310E40` and clears four fields.
///
/// Ghidra: 8008f140 `FUN_8008f140.c`
const FN_8008F140: usize = 0x8008_F140;

/// `(sub, -1)`: releases the `0x8008F140` sub-object.
///
/// Ghidra: 8008f170 `FUN_8008f170.c`
const FN_8008F170: usize = 0x8008_F170;

/// `(sub, name_or_index, byte, allocator) -> handle`: creates the named
/// sub-resource at `sub`.
///
/// Ghidra: 8008f1b0 `FUN_8008f1b0.c`
const FN_8008F1B0: usize = 0x8008_F1B0;

/// `(this, handle)`: binds a created sub-resource to its owner. Ghidra declares
/// two arguments; the original also leaves `r5 = 0x80427680` (the small-data
/// name block) but the callee never reads it.
///
/// Ghidra: 80163660 `FUN_80163660.c`
const FN_80163660: usize = 0x8016_3660;

/// `(this)`: clears `this+8` and releases `*(this+4)`. Ghidra's call site shows
/// no arguments, but `r3` is live at the call, so the real form is one argument.
///
/// Ghidra: 80163690 `FUN_80163690.c`
const FN_80163690: usize = 0x8016_3690;

/// `(a, b, factory, 0x20, 4) -> handle`: builds the 0x20-byte description used
/// by the global resource at `r13-0x5AC0`.
///
/// Ghidra: 8016e500 `FUN_8016e500.c`
const FN_8016E500: usize = 0x8016_E500;

/// `(size, handle, align) -> object`: allocator call used by the global
/// factory.
///
/// Ghidra: 802b9240 `FUN_802b9240.c`
const FN_802B9240: usize = 0x802B_9240;

/// `(object) -> object`: constructs the object in place and returns it. Ghidra's
/// call site shows no arguments, but `r3` holds the freshly allocated block.
///
/// Ghidra: 8016a5e0 `FUN_8016a5e0.c`
const FN_8016A5E0: usize = 0x8016_A5E0;

/// `(object, handle, 0x20)`: finishes initialisation of the global resource.
///
/// Ghidra: 8016a690 `FUN_8016a690.c`
const FN_8016A690: usize = 0x8016_A690;

/// `(sub) -> bool`: releases the `0x8008F140` sub-object's payload.
///
/// Ghidra: 8008f2b0 `FUN_8008f2b0.c`
const FN_8008F2B0: usize = 0x8008_F2B0;

/// `(sub, -1)`: tears down the `0x802B4D30` member object.
///
/// Ghidra: 802b4db0 `FUN_802b4db0.c`
const FN_802B4DB0: usize = 0x802B_4DB0;

/// `(sub, 0)`: tears down the `0x803294D8` member object.
///
/// Ghidra: 802acb80 `FUN_802acb80.c`
const FN_802ACB80: usize = 0x802A_CB80;

/// `(sub)`: installs the `0x803294D8` member vtable.
///
/// Ghidra: 802acb40 `FUN_802acb40.c`
const FN_802ACB40: usize = 0x802A_CB40;

/// `(sub)`: installs the `0x8008F140`-adjacent member vtable.
///
/// Ghidra: 802b4d30 `FUN_802b4d30.c`
const FN_802B4D30: usize = 0x802B_4D30;

/// The 4-byte tail thunk at `0x8000B54B0` (`b 0x801DD1D0`): a "is the console
/// in the 16:9 / progressive state?" predicate. Called directly, as the thunk
/// rule in `docs/COVERAGE_r80005_80014.md` requires.
///
/// Ghidra: 800b54b0 `thunk_FUN_801dd1d0.c`
const FN_801DD1D0: usize = 0x801D_D1D0;

/// `(out_floats, source)`: reads four floats out of a render object.
///
/// Ghidra: 802ad520 `FUN_802ad520.c` (`void FUN_802ad520(float*,int)`)
const FN_802AD520: usize = 0x802A_D520;

/// `(out, a, b, c, d, e, f) -> status`: opens a draw surface. The literal
/// argument shape at the single call site is
/// `(arg, 1, &word, 1000, 1000, 0x80, 0x80, 2, 2)`.
///
/// Ghidra: 801645f0 `FUN_801645f0.c` (nine parameters; typed `void` there but
/// the caller tests `r3`, so the return value is real)
const FN_801645F0: usize = 0x8016_45F0;

/// `(id_or_texture) -> handle`: resolves a texture/surface object. The single
/// argument `0x1F` selects the built-in surface.
///
/// Ghidra: 80164b20 `FUN_80164b20.c`
const FN_80164B20: usize = 0x8016_4B20;

/// `(surface, x0, y0, x1, y1, u0, v0)`: writes the surface rect and UV pair
/// (`+0xBC..+0xC4`, `+0xB4`/`+0xB8`) and marks the object dirty.
///
/// Ghidra: 80253c70 `FUN_80253c70.c` (`(double,double,double,double,double,double,int*)`)
const FN_80253C70: usize = 0x8025_3C70;

/// `(surface, x0, y0, x1, y1)`: writes the surface rect only.
///
/// Ghidra: 80253d00 `FUN_80253d00.c` (`(double,double,double,double,int*)`)
const FN_80253D00: usize = 0x8025_3D00;

/// `(0)`: closes the draw surface opened by [`FN_801645F0`].
///
/// Ghidra: 801644f0 `FUN_801644f0.c`
const FN_801644F0: usize = 0x8016_44F0;

/// `(a, b, c, d, e, f) -> status`: looks a render resource up by six register
/// arguments. The single call site forwards its own `r3..r8` unchanged.
///
/// Ghidra: 80164770 `FUN_80164770.c`
const FN_80164770: usize = 0x8016_4770;

/// `(source) -> resource`: resolves the resource that owns `source`.
///
/// Ghidra: 80164b70 `FUN_80164b70.c`
const FN_80164B70: usize = 0x8016_4B70;

/// `(table, name)`: looks `name` up in `table` and, when it is missing, allocates
/// and inserts a new 0x80-byte node. Ghidra types it `void` and drops both
/// arguments; its body reads `r3` as the table and `r4` as the name.
///
/// Ghidra: 802cd050 `FUN_802cd050.c`
const FN_802CD050: usize = 0x802C_D050;

/// `(a) -> a`: begin/end bracket around a batch of resource edits (the same
/// address is called before and after the twenty registrations). A 4-byte tail
/// thunk to `0x802B9170`.
///
/// Ghidra: 8016e4f0 `thunk_FUN_802b9170.c`
const FN_802B9170: usize = 0x802B_9170;

// ---------------------------------------------------------------------------
// Image constants read through r2 (small data area 2).
//
// Addresses are absolute; the original instruction is `lfs f, disp(r2)` with
// `r2 = 0x80433080`. Values were read out of `work/dol/main.elf`:
//   0x8042B080 = 0.0f          0x8042B084 = 0.7307692f (19/26)
//   0x8042B088 = 1.0f          0x8042B098 = 0.5f
//   0x8042B09C = -0.5f         0x8042B0A0 = 0.0f
//   0x8042B0A4 = 1.0f          0x8042B0F0 = 0.1f
// ---------------------------------------------------------------------------

/// `r2-0x7F90` - duplicate-point tolerance, image value `0.1f`.
const DUP_TOLERANCE_F32: usize = 0x8042_B0F0;

/// `r2-0x8000` - zero, initial value of the `+0x88`/`+0x8C` fields.
const F32_ZERO_8042B080: usize = 0x8042_B080;

/// `r2-0x7FFC` - `0.7307692f` (`19/26`), initial value of the `+0x74` field.
const F32_RATIO_8042B084: usize = 0x8042_B084;

/// `r2-0x7FF8` - one, initial value of the `+0x78` field.
const F32_ONE_8042B088: usize = 0x8042_B088;

/// `r2-0x7FE8` - `0.5f`, first/second rect corner coordinate.
const F32_HALF_8042B098: usize = 0x8042_B098;

/// `r2-0x7FE4` - `-0.5f`, the mirrored corner coordinate.
const F32_NEG_HALF_8042B09C: usize = 0x8042_B09C;

/// `r2-0x7FE0` - zero, the first UV coordinate.
const F32_ZERO_8042B0A0: usize = 0x8042_B0A0;

/// `r2-0x7FDC` - one, the second UV coordinate and the `+0x30` scale word.
const F32_ONE_8042B0A4: usize = 0x8042_B0A4;

// ---------------------------------------------------------------------------
// Local helpers.
// ---------------------------------------------------------------------------

/// Address of the `r13` small-data slot `offset` bytes below the base.
///
/// `r13` is pinned to [`SDA1_BASE`] (`0x8042F680`) by `__init_registers` at
/// `0x80004244` (`lis r13,0x8042` + `ori r13,r13,0xF680`), so `r13 - N` is
/// written `sda1(N)` throughout this file.
#[inline]
fn sda1(offset: usize) -> usize {
    SDA1_BASE - offset
}

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

/// Calls a function taking and returning a `double` (PowerPC `f1`).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f64(addr: usize, value: f64) -> f64 {
    let f: extern "C" fn(f64) -> f64 = core::mem::transmute(addr);
    f(value)
}

/// Calls a function with one pointer and six `double` arguments, matching
/// `0x80253C70` (`r3`, then `f1..f6`).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f64x6_ptr(
    addr: usize,
    ptr: u32,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
) {
    let func: extern "C" fn(u32, f64, f64, f64, f64, f64, f64) =
        core::mem::transmute(addr);
    func(ptr, a, b, c, d, e, f)
}

/// Calls a function with one pointer and four `double` arguments, matching
/// `0x80253D00` (`r3`, then `f1..f4`).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f64x4_ptr(addr: usize, ptr: u32, a: f64, b: f64, c: f64, d: f64) {
    let func: extern "C" fn(u32, f64, f64, f64, f64) = core::mem::transmute(addr);
    func(ptr, a, b, c, d)
}

/// Calls a function with nine integer arguments (`r3..r10` plus one stack
/// slot). `crate::platform` stops at [`platform::call7`], so the ninth argument
/// needs its own helper.
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call9(addr: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32, h: u32, i: u32) -> u32 {
    let func: extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(addr);
    func(a, b, c, d, e, f, g, h, i)
}

/// Calls a function with no arguments and keeps the `r3` result.
///
/// [`platform::call0`] discards the register, but several leaf predicates in
/// this band return a status in `r3` while taking nothing.
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call0_u32(addr: usize) -> u32 {
    let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
    f()
}

/// Reads the function pointer stored in `object`'s vtable at byte `slot`.
///
/// # Safety
/// `object` must hold a readable vtable pointer and the table must have at
/// least `slot + 4` readable bytes.
#[inline]
unsafe fn vtable_entry(object: usize, slot: usize) -> usize {
    platform::read_u32(platform::read_u32(object) as usize + slot) as usize
}

/// Builds the two-word temporary that the original copies one point into.
///
/// Only the first word is written; the second stays uninitialised, exactly as
/// on the original stack frame. The 2-float writer `0x802C7C70` therefore sees
/// an unspecified second component, which the attachment wrappers pass through
/// unchanged.
///
/// # Safety
/// `src` must be readable for 4 bytes.
#[inline]
unsafe fn point_temp(src: usize) -> core::mem::MaybeUninit<[u32; 2]> {
    let mut temp = core::mem::MaybeUninit::<[u32; 2]>::uninit();
    core::ptr::write(temp.as_mut_ptr() as *mut u32, platform::read_u32(src));
    temp
}

// ===========================================================================
// 1. Vector helpers
// ===========================================================================

/// Subtracts two 2D points: `out = b - a`, component by component.
///
/// This is the only vector operation the range's quad/rail code needs, so it
/// is kept as a public building block.
///
/// # Original
/// - Ghidra: `FUN_8000D870` at `0x8000D870` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d870_FUN_8000d870.c`
///
/// # Arguments
/// * `out` - 2 floats written (`x` then `y`).
/// * `a` - first point (`x` at `+0`, `y` at `+4`).
/// * `b` - second point.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two stores. No calls.
///
/// # Safety
/// All three pointers must be valid for the accesses shown; `out` may alias
/// neither, but the original does not check.
///
/// # Confidence
/// high - the instruction stream is four `lfs`, two `fsubs`, two `stfs`.
pub unsafe fn vec2_sub(out: usize, a: usize, b: usize) {
    // observed 0x8000D87C..0x8000D88C: load a.x/a.y and b.x/b.y, subtract
    // b - a, store x at +0 then y at +4.
    let bx = read_f32(b);
    let by = read_f32(b + 4);
    let ax = read_f32(a);
    let ay = read_f32(a + 4);
    write_f32(out + 4, by - ay);
    write_f32(out, bx - ax);
}

/// Returns whether two 2D points differ by more than the global tolerance.
///
/// The predicate is "**not** inside the tolerance box":
/// `true` when `|dx| >= eps` or `|dy| >= eps`, where `eps` is the `f32` at
/// `0x8042B0F0` (image value `0.1`). It is used to drop duplicate rail points.
///
/// # Original
/// - Ghidra: `FUN_8000D8A0` at `0x8000D8A0` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d8a0_FUN_8000d8a0.c`
///
/// # Arguments
/// * `a` - first 2D point.
/// * `b` - second 2D point.
///
/// # Returns
/// `true` when the points are farther apart than the tolerance on either axis.
///
/// # Side effects
/// Two stack stores (the local delta). No calls except [`vec2_sub`].
///
/// # Safety
/// `a` and `b` must each be readable for 8 bytes.
///
/// # Confidence
/// high - the body is this exact four-comparison shape. The compiled form uses
/// `fcmpo` + `cror` (see `0x8000D8C4..0x8000D914`), so a NaN input takes the
/// "differs" branch; the comparisons below reproduce the C pseudocode.
pub unsafe fn vec2_differs_from(a: usize, b: usize) -> bool {
    let mut delta = [0u32; 2];
    vec2_sub(delta.as_mut_ptr() as usize, a, b);
    let dx = f32::from_bits(delta[0]);
    let dy = f32::from_bits(delta[1]);
    let eps = read_f32(DUP_TOLERANCE_F32);

    // observed: `if (dx < eps) { if (-eps < dx) { if (dy < eps) ...
    // Only the all-inside case returns false.
    !(dx < eps && -eps < dx && dy < eps && -eps < dy)
}

/// Length of the 2D delta `b - a`.
///
/// The original squares each component in single precision, adds them, widens
/// the sum to `double`, and calls `0x802BE470` (a `sqrt`-shaped helper that
/// narrows its result back to `f32`). The `double` result is what callers keep.
///
/// # Original
/// - Ghidra: `FUN_8000F900` at `0x8000F900` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f900_FUN_8000f900.c`
///
/// # Arguments
/// * `a` - first 2D point.
/// * `b` - second 2D point.
///
/// # Returns
/// `sqrt((b.x-a.x)^2 + (b.y-a.y)^2)` as `f64` (`// inferred:` the helper at
/// `0x802BE470` is a square root; its body forwards to `0x802E8620` and
/// narrows through `f32`).
///
/// # Side effects
/// Two stack stores of the deltas. One call.
///
/// # Safety
/// `a` and `b` must each be readable for 8 bytes.
///
/// # Confidence
/// high for the arithmetic; medium for the name of `0x802BE470`.
pub unsafe fn vec2_length(a: usize, b: usize) -> f64 {
    let dx = read_f32(b) - read_f32(a);
    let dy = read_f32(b + 4) - read_f32(a + 4);
    // observed 0x8000F924..0x8000F934: fmuls/fadds in single precision.
    let sum_of_squares = dx * dx + dy * dy;
    call_f64(FN_802BE470, sum_of_squares as f64)
}

// ===========================================================================
// 2. Vertex-grid storage
// ===========================================================================

/// Stores four `Vec3`s (one per lane) at the same index in four strided arrays.
///
/// The destination object owns four parallel arrays of 3-float vertices:
/// lane `k` starts at `object + 0x48 + k*0x3390`, and each vertex is 12 bytes
/// (`x` at `+0`, `y` at `+4`, `z` at `+8`). This function writes index `index`
/// of all four lanes from twelve consecutive floats in `src`.
///
/// The "four lanes at one index" shape is why the write order interleaves the
/// components (`x` of all lanes, then `y`, then `z`) in the compiled code; the
/// per-lane loop below performs exactly the same twelve stores.
///
/// # Original
/// - Ghidra: `FUN_8000D0D0` at `0x8000D0D0` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d0d0_FUN_8000d0d0.c`
///
/// # Arguments
/// * `object` - base of the four-lane structure.
/// * `src` - 12 floats: lane 0 `x,y,z`, lane 1 `x,y,z`, lane 2, lane 3.
/// * `index` - element index inside each lane (`index*12` bytes).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// 12 `f32` stores. No calls.
///
/// # Safety
/// `object` must be valid up to `+0x48 + 3*0x3390 + index*12 + 12`; `src` must
/// be readable for 48 bytes. The original does no bounds check - the caller
/// owns the index range.
///
/// # Confidence
/// high - the stride `0x3390`, base `0x48` and 12-byte vertex are all visible
/// in `addi r6,r3,72`, `mulli r10,r5,12` and the four `stfs` groups.
pub unsafe fn store_four_lane_vertices(object: usize, src: usize, index: u32) {
    /// Distance between the four parallel vertex arrays, in bytes
    /// (observed: `addis r5,r6,1` + `stfs f0,0x3390(r9)`).
    const LANE_STRIDE: usize = 0x3390;
    /// First lane's `x` component.
    const LANES_BASE: usize = 0x48;

    let mut lane = 0;
    while lane < 4 {
        let dst = object + LANES_BASE + lane * LANE_STRIDE + index as usize * 12;
        let s = src + lane * 12;
        write_f32(dst, read_f32(s));
        write_f32(dst + 4, read_f32(s + 4));
        write_f32(dst + 8, read_f32(s + 8));
        lane += 1;
    }
}

/// Copies twelve consecutive `f32` values (48 bytes) to `dst + 0x28`.
///
/// The destination layout at `+0x28..+0x58` is twelve words; the source is
/// three `Vec4`s or four `Vec3`s depending on the owner (`// unknown:` which -
/// the copy itself is layout-agnostic).
///
/// # Original
/// - Ghidra: `FUN_8000AE50` at `0x8000AE50` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000ae50_FUN_8000ae50.c`
///
/// # Arguments
/// * `dst` - object base; words `+0x28 .. +0x54` are overwritten.
/// * `src` - 12 floats.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve `stfs` stores. No calls.
///
/// # Safety
/// `dst` valid for writes in `0x28..0x58`, `src` readable for 48 bytes.
///
/// # Confidence
/// high - twelve `lfs`/`stfs` pairs, offsets `0x00..0x2C` to `0x28..0x54`,
/// nothing else in the body.
pub unsafe fn copy_12_floats(dst: usize, src: usize) {
    let mut i = 0;
    while i < 12 {
        write_f32(dst + 0x28 + i * 4, read_f32(src + i * 4));
        i += 1;
    }
}

/// Sets the 2-float point of the sub-object at `object + 0x08 + index*0x20`.
///
/// The sub-object is produced by the scene helper `0x802C7C70`, which takes the
/// destination, a pointer to the value, and four flag words. This wrapper
/// always passes `(0, 0, 1, 1)` - i.e. "copy one 2-float point, both axes".
///
/// # Original
/// - Ghidra: `FUN_8000B320` at `0x8000B320` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000b320_FUN_8000b320.c`
///
/// # Arguments
/// * `object` - owner of the sub-object array.
/// * `index` - element index; stride is 0x20 bytes.
/// * `point` - readable 2 floats (only the first word is loaded here).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x802C7C70`, which writes into the sub-object.
///
/// # Safety
/// `object + 8 + index*0x20` must be a valid sub-object for `0x802C7C70`;
/// `point` readable for 8 bytes.
///
/// # Confidence
/// high for the address arithmetic (`slwi r0,r4,5` + `addi r3,r3,8`); the
/// effect of `0x802C7C70` is taken from its own pseudocode.
///
/// # Placement note
/// This belongs to the vertex-grid cluster because it publishes one 2-float
/// point into the same sub-object layout that `copy_12_floats` fills; it has no
/// path/rail behaviour of its own.
pub unsafe fn fn_8000b320(object: usize, index: u32, point: usize) {
    let mut value = [0u32; 2];
    value[0] = platform::read_u32(point);
    call6(
        FN_802C7C70,
        (object + index as usize * 0x20 + 8) as u32,
        value.as_mut_ptr() as u32,
        0,
        0,
        1,
        1,
    );
}

// ===========================================================================
// 3. Scene sub-object attachment (0x8000AA70, 0x8000AEC0, 0x8000B370)
//
// The three wrappers share one skeleton:
//
//   if (0x80164FD0(owner, key, 0) == 0) return 0;
//   0x802C7C70(owner + 0x08, &point_a, f0, f1, 1, 1);
//   [0x802C7C70(owner + 0x28, &point_b, f0, f1, 1, 1);]
//   if (state < 0) { 0x8016A2A0(owner, 0, state_value); 0x8016A0F0(owner, 6, 1); }
//   else           { 0x8016A2A0(owner, state, 0);        0x8016A0F0(owner, 7, 1); }
//   return 1;
//
// Selector 6 therefore means "no state value" and 7 "state value present".
// The flags passed to 0x802C7C70 are `(copy, flip, 1, 1)`: the second flag is
// the only one that ever changes between the attachment variants.
// ===========================================================================

/// Validates an owner, attaches one 2-float point at `owner + 0x08` and selects
/// the owner's state.
///
/// `fn_8000aec0` at `0x8000AEC0` is the byte-identical twin of this function
/// (same 220 bytes, same constants; only the call displacements differ), so it
/// is translated separately in [`fn_8000aec0`] rather than shared.
///
/// # Original
/// - Ghidra: `FUN_8000AA70` at `0x8000AA70` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000aa70_FUN_8000aa70.c`
///
/// # Arguments
/// * `owner` - scene object validated by `0x80164FD0`.
/// * `point` - source of the 2-float point; only its first word is copied.
/// * `state` - negative means "detached" (selector `6`), otherwise the value
///   stored verbatim with selector `7`.
/// * `state_value` - payload used only in the detached case.
/// * `key` - second argument of the `0x80164FD0` validity check.
///
/// # Returns
/// `1` when the point was attached, `0` when the owner failed validation.
///
/// # Side effects
/// Writes the sub-object at `owner + 0x08` through `0x802C7C70`, then two state
/// calls on `owner`.
///
/// # Safety
/// `owner` must be a valid scene object; `point` readable for 4 bytes.
///
/// # Confidence
/// high - the control flow, the flags `(0,0,1,1)` and the `6`/`7` selectors are
/// all read directly from the instruction stream (`0x8000AAA4..0x8000AB28`).
pub unsafe fn fn_8000aa70(owner: u32, point: usize, state: i32, state_value: u32, key: u32) -> u32 {
    if platform::call3(FN_80164FD0, owner, key, 0) == 0 {
        return 0;
    }

    let mut value = point_temp(point);
    call6(
        FN_802C7C70,
        owner + 8,
        value.as_mut_ptr() as u32,
        0,
        0,
        1,
        1,
    );

    if state < 0 {
        platform::call3(FN_8016A2A0, owner, 0, state_value);
        platform::call3(FN_8016A0F0, owner, 6, 1);
    } else {
        platform::call3(FN_8016A2A0, owner, state as u32, 0);
        platform::call3(FN_8016A0F0, owner, 7, 1);
    }

    1
}

/// Twin of [`fn_8000aa70`] at `0x8000AEC0`.
///
/// The bytes are identical apart from the intra-function branch displacements,
/// so this is a second entry point with the same behaviour: validate, attach one
/// 2-float point at `+0x08`, select state `6`/`7`.
///
/// # Original
/// - Ghidra: `FUN_8000AEC0` at `0x8000AEC0` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000aec0_FUN_8000aec0.c`
///
/// # Arguments
/// Same as [`fn_8000aa70`].
///
/// # Returns
/// `1` on success, `0` when `0x80164FD0` rejects the owner.
///
/// # Side effects
/// Same as [`fn_8000aa70`].
///
/// # Safety
/// `owner` must be a valid scene object; `point` readable for 4 bytes.
///
/// # Confidence
/// high - instruction-for-instruction identical to `0x8000AA70`
/// (`0x8000AEF4..0x8000AF78`).
pub unsafe fn fn_8000aec0(owner: u32, point: usize, state: i32, state_value: u32, key: u32) -> u32 {
    if platform::call3(FN_80164FD0, owner, key, 0) == 0 {
        return 0;
    }

    let mut value = point_temp(point);
    call6(
        FN_802C7C70,
        owner + 8,
        value.as_mut_ptr() as u32,
        0,
        0,
        1,
        1,
    );

    if state < 0 {
        platform::call3(FN_8016A2A0, owner, 0, state_value);
        platform::call3(FN_8016A0F0, owner, 6, 1);
    } else {
        platform::call3(FN_8016A2A0, owner, state as u32, 0);
        platform::call3(FN_8016A0F0, owner, 7, 1);
    }

    1
}

/// Validates an owner, attaches two 2-float points and records a mode word.
///
/// This is the richer sibling of [`fn_8000aa70`]: it writes both the `+0x08`
/// and the `+0x28` sub-objects and stores a mode/counter pair at
/// `+0xCE90..+0xCE9A` (a `0x44C` constant, the mode, and two zero halfwords).
///
/// The mode selects which of the two sub-objects is flipped:
/// * mode `0` - neither;
/// * mode `4` - the first (`+0x08`);
/// * any other value - the second (`+0x28`).
///
/// # Original
/// - Ghidra: `FUN_8000B370` at `0x8000B370` (444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000b370_FUN_8000b370.c`
///
/// # Arguments
/// * `owner` - scene object validated by `0x80164FD0`.
/// * `mode` - sub-object flip selector (`0`, `4` or other) and the value
///   stored at `owner + 0xCE94`.
/// * `point_a` - source of the point for the `+0x08` sub-object.
/// * `point_b` - source of the point for the `+0x28` sub-object.
/// * `state` - negative means "detached" (selector `6`), otherwise the value
///   stored with selector `7`.
/// * `state_value` - payload used only in the detached case.
/// * `key` - second argument of the `0x80164FD0` validity check.
///
/// # Returns
/// `1` when both points were attached, `0` when the owner is invalid.
///
/// # Side effects
/// Four `0x802C7C70` calls at most, two state calls, and four stores in
/// `+0xCE90..+0xCE9C`.
///
/// # Safety
/// `owner` must be valid up to `+0xCE9C`; both point sources readable for
/// 4 bytes each.
///
/// # Confidence
/// high - the dispatch chain is `cmpi r27,0` / `cmpi r27,4`
/// (`0x8000B3B8..0x8000B4A4`) and every flag word was read from the `r5..r8`
/// setup before each `bl 0x802C7C70`.
///
/// # Ghidra artefact
/// The C prints `uVar5 = FUN_802dcd70()` and then uses `iVar1`/`iVar4` as if
/// they came from it. `0x802DCD70` is a Metrowerks save-register helper, not a
/// call: the two "halves" are really `owner` (`r26`) and `mode` (`r27`). See
/// "Ghidra artefact" in `docs/COVERAGE_r80005_80014.md`.
pub unsafe fn fn_8000b370(
    owner: u32,
    mode: i32,
    point_a: usize,
    point_b: usize,
    state: i32,
    state_value: u32,
    key: u32,
) -> u32 {
    if platform::call3(FN_80164FD0, owner, key, 0) == 0 {
        return 0;
    }

    let mut a = point_temp(point_a);
    let mut b = point_temp(point_b);

    if mode == 0 {
        call6(
            FN_802C7C70,
            owner + 8,
            a.as_mut_ptr() as u32,
            1,
            0,
            1,
            1,
        );
        call6(
            FN_802C7C70,
            owner + 0x28,
            b.as_mut_ptr() as u32,
            1,
            0,
            1,
            1,
        );
    } else if mode == 4 {
        call6(
            FN_802C7C70,
            owner + 8,
            a.as_mut_ptr() as u32,
            1,
            1,
            1,
            1,
        );
        call6(
            FN_802C7C70,
            owner + 0x28,
            b.as_mut_ptr() as u32,
            1,
            0,
            1,
            1,
        );
    } else {
        call6(
            FN_802C7C70,
            owner + 8,
            a.as_mut_ptr() as u32,
            1,
            0,
            1,
            1,
        );
        call6(
            FN_802C7C70,
            owner + 0x28,
            b.as_mut_ptr() as u32,
            1,
            1,
            1,
            1,
        );
    }

    if state < 0 {
        platform::call3(FN_8016A2A0, owner, 0, state_value);
        platform::call3(FN_8016A0F0, owner, 6, 1);
    } else {
        platform::call3(FN_8016A2A0, owner, state as u32, 0);
        platform::call3(FN_8016A0F0, owner, 7, 1);
    }

    // observed 0x8000B4F4..0x8000B510: `addis r4,r26,1` then the four stores
    // at -0x3170..-0x3166, i.e. owner + 0xCE90..+0xCE9A.
    let owner = owner as usize;
    platform::write_u32(owner + 0xCE90, 0x44C);
    platform::write_i32(owner + 0xCE94, mode);
    platform::write_u16(owner + 0xCE98, 0);
    platform::write_u16(owner + 0xCE9A, 0);

    1
}

// ===========================================================================
// 4. Screen-space rectangle (0x80007520)
// ===========================================================================

/// Opens a `1000x1000` GX draw surface and writes its rect/UV quad.
///
/// The sequence is:
/// 1. `0x801645F0(arg, 1, &word, 1000, 1000, 0x80, 0x80, 2, 2)` - opens the
///    surface; a zero status aborts and returns `0`.
/// 2. `0x80164B20(0x1F)` - resolves the built-in surface object (stored in the
///    same stack word, so the word is reused as the handle).
/// 3. `0x80253C70` writes six floats `(0.5, -0.5, -0.5, 0.5, 0.0, 1.0)` - the
///    rect corners followed by the UV pair.
/// 4. `0x80253D00` writes the four rect corners `(0.5, 0.5, 0.5, 0.5)`.
/// 5. `0x801644F0(0)` closes the surface.
///
/// # Original
/// - Ghidra: `FUN_80007520` at `0x80007520` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007520_FUN_80007520.c`
///
/// # Arguments
/// * `arg` - first argument of the surface open; role `// unknown:` (forwarded
///   verbatim).
///
/// # Returns
/// `1` when the surface was opened and configured, `0` when the open failed.
///
/// # Side effects
/// Allocates/opens a GX surface, writes its rect fields, closes it.
///
/// # Safety
/// The GX surface helpers expect the renderer to be initialised.
///
/// # Confidence
/// medium - the argument shapes and float constants are exact
/// (`0x80007544..0x800075B8`), but what `0x801645F0`'s nine arguments mean and
/// whether the rect is a scissor, viewport or texture quad is not resolved.
pub unsafe fn fn_80007520(arg: u32) -> u32 {
    // observed 0x80007538: r5 points at a stack word initialised to zero; the
    // callee may replace it with an output value.
    let mut open_word = [0u32; 1];

    let status = call9(
        FN_801645F0,
        arg,
        1,
        open_word.as_mut_ptr() as u32,
        1000,
        1000,
        0x80,
        0x80,
        2,
        2,
    );
    if status == 0 {
        return 0;
    }

    // observed 0x80007570/0x80007580: the 0x1F lookup result goes into a
    // *second* stack word (r1+0x14) that is then passed as the surface pointer.
    // The original never reuses the open-call word.
    let mut surface_word = [0u32; 1];
    surface_word[0] = platform::call1(FN_80164B20, 0x1F);

    call_f64x6_ptr(
        FN_80253C70,
        surface_word.as_mut_ptr() as u32,
        read_f32(F32_HALF_8042B098) as f64,
        read_f32(F32_NEG_HALF_8042B09C) as f64,
        read_f32(F32_NEG_HALF_8042B09C) as f64,
        read_f32(F32_HALF_8042B098) as f64,
        read_f32(F32_ZERO_8042B0A0) as f64,
        read_f32(F32_ONE_8042B0A4) as f64,
    );

    let half = read_f32(F32_HALF_8042B098) as f64;
    call_f64x4_ptr(
        FN_80253D00,
        surface_word.as_mut_ptr() as u32,
        half,
        half,
        half,
        half,
    );

    platform::call1(FN_801644F0, 0);
    1
}

// ===========================================================================
// 5. Named-material registration (0x80007610)
// ===========================================================================

/// Registers the twenty small-data entries of a render resource.
///
/// The six incoming arguments are forwarded unchanged to `0x80164770`, which
/// validates the resource slot; a zero status aborts with `0`. The function
/// then resolves the resource through `0x80164B70`, brackets a batch of twenty
/// `0x802CD050` calls with `0x802B9170`, and finally writes `1.0f` at
/// `*(resource + 0x08) + 0x30`.
///
/// The twenty arguments are consecutive 8-byte entries starting at `r13-0x7FF8`
/// (`0x80427688`, inside the small-data block that `0x80163660` also points at).
/// `0x802CD050` looks each one up by name and allocates a 0x80-byte node when it
/// is missing, so this is a material/mesh registration pass.
///
/// # Original
/// - Ghidra: `FUN_80007610` at `0x80007610` (396 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007610_FUN_80007610.c`
///
/// # Arguments
/// * `p1..p5` - forwarded verbatim to `0x80164770`; roles `// unknown:`.
/// * `p6` - sixth argument of `0x80164770` and the key passed to `0x80164B70`.
///
/// # Returns
/// `1` on success, `0` when either lookup fails.
///
/// # Side effects
/// Up to twenty `0x802CD050` calls (each may allocate), two `0x802B9170`
/// calls, and one `f32` store at `*(resource+8)+0x30`.
///
/// # Safety
/// `p6` must be a key accepted by `0x80164B70`; the returned resource must be
/// readable.
///
/// # Confidence
/// medium - every call target, the 20-iteration count (`r13-0x7FF8` ..
/// `r13-0x7F60`, stride 8) and the `+0x30` store are instruction-exact; the
/// meaning of the six forwarded arguments is not resolved.
///
/// # Ghidra artefact
/// Ghidra prints `in_r8` for the sixth argument because it only tracked `r3`;
/// the raw `or r29,r8,r8` at `0x80007628` shows the real parameter register.
pub unsafe fn fn_80007610(
    p1: u32,
    p2: u32,
    p3: u32,
    p4: u32,
    p5: u32,
    p6: u32,
) -> u32 {
    // observed 0x80007634: `bl 0x80164770` with r3..r8 still holding all six
    // incoming arguments.
    if platform::call6(FN_80164770, p1, p2, p3, p4, p5, p6) == 0 {
        return 0;
    }

    let resource = platform::call1(FN_80164B70, p6);
    if resource == 0 {
        return 0;
    }

    let table = platform::read_u32(resource as usize + 0x14);
    let token = platform::call1(FN_802B9170, p1);

    let mut i = 0;
    while i < 20 {
        // observed: `addi r4,r13,-32760` for i = 0, then +8 per iteration up
        // to `addi r4,r13,-32608` for i = 19.
        let entry = sda1(0x7FF8 - i * 8);
        platform::call2(FN_802CD050, table, entry as u32);
        i += 1;
    }

    platform::call1(FN_802B9170, token);

    let target = platform::read_u32(resource as usize + 8) as usize;
    write_f32(target + 0x30, read_f32(F32_ONE_8042B0A4));

    1
}

// ===========================================================================
// 6. Object construction and destruction (0x80006870..0x80006F70)
//
// Two object families appear here:
//   * the `0x802FE400`/`0x802FE410` family (0x800068B0, 0x800069A0) and its
//     `0x802FE3F0` extension (0x80006AA0, 0x80006AF0), which owns a
//     `0x8008F140` sub-object at `+0xC0` and a resource handle at `+0xBC`;
//   * the `0x802FE3D8`/`0x803294D8` family (0x80006CD0, 0x80006E30), which owns
//     a `0x802B4D30` sub-object at `+0x30`.
// Every destructor takes `(this, free_flag)`, frees only when `free_flag > 0`,
// and returns `this`; that shape is the Metrowerks deleting destructor.
// ===========================================================================

/// Queues a one-byte message for the code label at `0x80006850`.
///
/// `0x80006850` is an unlisted 32-byte leaf that copies its byte argument to
/// `this + 0xB8` and `this + 0xB9` and returns `0`; `0x80163E10` is the queue
/// helper that invokes it. Ghidra names the target `LAB_80006850` because it is
/// a message handler, not a separately indexed function.
///
/// # Original
/// - Ghidra: `FUN_80006870` at `0x80006870` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006870_FUN_80006870.c`
///
/// # Arguments
/// * `this` - object whose `+0xB8`/`+0xB9` bytes the handler writes.
/// * `value` - the byte carried by the message.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x80163E10` call, which may run the handler immediately.
///
/// # Safety
/// `this` must be a valid object of the handler's class.
///
/// # Confidence
/// high - the call shape `(r3, 0x80006850, 0, &byte)` is visible at
/// `0x80006880..0x80006890`, and the handler's body was read at `0x80006850`.
pub unsafe fn fn_80006870(this: u32, value: u8) {
    // observed 0x80006884: `stb r4,8(r1)`; the remaining 7 bytes of the
    // 8-byte slot stay uninitialised, as in the original frame.
    let mut message = core::mem::MaybeUninit::<[u8; 8]>::uninit();
    core::ptr::write(message.as_mut_ptr() as *mut u8, value);
    platform::call4(
        FN_80163E10,
        this,
        0x8000_6850,
        0,
        message.as_mut_ptr() as u32,
    );
}

/// Constructs the `0x802FE400` object: clears two words, installs the vtable,
/// builds the `0x802FE410` member at `+0x0C`, then runs virtual slot `+0x0C`.
///
/// # Original
/// - Ghidra: `FUN_800068B0` at `0x800068B0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800068b0_FUN_800068b0.c`
///
/// # Arguments
/// * `this` - storage for the new object (at least `+0x10` bytes).
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Three word stores, one `0x802B6620` call on `this + 0x0C`, and one virtual
/// call through the freshly installed vtable slot `+0x0C`.
///
/// # Safety
/// `this` must be writable for the fields shown and must be a valid object by
/// the time the virtual slot runs.
///
/// # Confidence
/// high - `stw`/`addi` sequence `0x800068D8..0x80006908`; the virtual dispatch
/// is `lwz r12,0(r30); lwz r12,12(r12); bcctr`.
pub unsafe fn fn_800068b0(this: usize) -> usize {
    platform::write_u32(this + 4, 0);
    platform::write_u32(this + 8, 0);
    platform::write_u32(this, 0x802F_E400);
    platform::call1(FN_802B6620, (this + 0x0C) as u32);
    platform::write_u32(this + 0x0C, 0x802F_E410);

    let slot = vtable_entry(this, 0x0C);
    platform::call1(slot, this as u32);

    this
}

/// Destroys a `0x802FE400` object, optionally releasing its storage.
///
/// The member at `+0x0C` is torn down first; the test `this + 0x30 != 0` is
/// always true for a non-null `this`, which is why it appears as a redundant
/// duplicate branch in the instruction stream (`0x800069C8..0x800069E0`).
///
/// # Original
/// - Ghidra: `FUN_800069A0` at `0x800069A0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800069a0_FUN_800069a0.c`
///
/// # Arguments
/// * `this` - object to destroy; a null pointer is a no-op.
/// * `free_flag` - positive values also release `this` through `0x802B9280`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Two member teardown calls (`0x80229030` on `+0x30`, `0x802B63B0` on `+0x0C`)
/// and an optional free.
///
/// # Safety
/// `this` must be null or a valid object of this class.
///
/// # Confidence
/// high - every branch was decoded (`beq`/`bne` at `0x800069C4`,
/// `0x800069CC`, `0x800069D0`, `0x800069D8`, `0x800069F4`).
pub unsafe fn fn_800069a0(this: usize, free_flag: i32) -> usize {
    if this != 0 {
        if this + 0x0C != 0 {
            // observed 0x800069C8/0x800069D0: two identical `beq` tests on the
            // same `addic. r3,r31,0x0C` result; this one is on `+0x30`.
            if this + 0x30 != 0 {
                platform::call2(FN_80229030, (this + 0x30) as u32, 0);
            }
            platform::call2(FN_802B63B0, (this + 0x0C) as u32, 0);
        }
        if free_flag > 0 {
            platform::call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Tail-dispatches virtual slot `+0x0C` of the member at `this + 0x0C`.
///
/// The original loads `r12 = *member`, then `r12 = *(r12 + 0x0C)`, sets
/// `r6 = 0` and jumps. `r4`/`r5` are forwarded unchanged, so this entry keeps
/// the two incoming argument registers.
///
/// # Original
/// - Ghidra: `FUN_80006A20` at `0x80006A20` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006a20_FUN_80006a20.c`
///
/// # Arguments
/// * `this` - owner of the member at `+0x0C`.
/// * `a`, `b` - forwarded to the virtual slot in `r4`/`r5`.
///
/// # Returns
/// The virtual slot's `r3`.
///
/// # Side effects
/// One indirect call that may do anything.
///
/// # Safety
/// `this + 0x0C` must hold a valid object with a vtable of at least `+0x10`.
///
/// # Confidence
/// high - `lwzu r12,12(r3)` / `lwz r12,12(r12)` / `bcctr` at
/// `0x80006A20..0x80006A30`; Ghidra's "could not recover jumptable" warning is
/// the tail call, not missing code.
pub unsafe fn fn_80006a20(this: usize, a: u32, b: u32) -> u32 {
    let member = this + 0x0C;
    let slot = vtable_entry(member, 0x0C);
    platform::call4(slot, member as u32, a, b, 0)
}

/// Constructs the `0x802FE3F0` object: base constructor, own vtable, a cleared
/// handle word at `+0xBC` and a `0x8008F140` sub-object at `+0xC0`.
///
/// # Original
/// - Ghidra: `FUN_80006AA0` at `0x80006AA0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006aa0_FUN_80006aa0.c`
///
/// # Arguments
/// * `this` - storage for the new object (at least `+0xC4` bytes).
///
/// # Returns
/// `this`.
///
/// # Side effects
/// One call to [`fn_800068b0`] on the same object, two stores, one
/// `0x8008F140` call. Ghidra prints `FUN_800068b0()` without arguments because
/// it models the call as taking none; `r3` is still `this` at `0x80006AB4`.
///
/// # Safety
/// `this` must be writable for the fields shown.
///
/// # Confidence
/// high - `or r31,r3,r3` before the call proves the receiver, and the field
/// offsets are `0`/`0xBC`/`0xC0` in the instruction stream.
pub unsafe fn fn_80006aa0(this: usize) -> usize {
    fn_800068b0(this);
    platform::write_u32(this, 0x802F_E3F0);
    platform::write_u32(this + 0xBC, 0);
    platform::call1(FN_8008F140, (this + 0xC0) as u32);
    this
}

/// Destroys a `0x802FE3F0` object, optionally releasing its storage.
///
/// # Original
/// - Ghidra: `FUN_80006AF0` at `0x80006AF0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006af0_FUN_80006af0.c`
///
/// # Arguments
/// * `this` - object to destroy; null is a no-op.
/// * `free_flag` - positive values also release `this`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// `0x8008F170(this + 0xC0, -1)`, the base destructor [`fn_800069a0`] with a
/// zero free flag, then an optional free.
///
/// # Safety
/// `this` must be null or a valid object of this class.
///
/// # Confidence
/// high - `addi r4,r0,-1` / `bl 0x8008f170`, the `bl 0x800069a0` with `r4 = 0`
/// and the `bgt` on `param_2` are all explicit.
pub unsafe fn fn_80006af0(this: usize, free_flag: i32) -> usize {
    if this != 0 {
        platform::call2(FN_8008F170, (this + 0xC0) as u32, 0xFFFF_FFFF);
        fn_800069a0(this, 0);
        if free_flag > 0 {
            platform::call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Creates the resource behind `+0xBC` once, using the global allocator.
///
/// The allocator is the word at `r13-0x5AC0` (`0x80429BC0`), slot `+0x14`. The
/// third argument is truncated to a byte (`clrlwi r5,r5,24`) before the call,
/// so `param_3` is a `u8`. On success the new handle is also bound with
/// `0x80163660`.
///
/// # Original
/// - Ghidra: `FUN_80006B60` at `0x80006B60` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006b60_FUN_80006b60.c`
///
/// # Arguments
/// * `this` - owner object; `+0xBC` caches the handle.
/// * `param_2` - second argument of `0x8008F1B0` (role `// unknown:`).
/// * `param_3` - byte argument of `0x8008F1B0`.
///
/// # Returns
/// `1` when a handle exists afterwards, `0` when creation failed.
///
/// # Side effects
/// May allocate; writes `+0xBC`; one `0x80163660` call.
///
/// # Safety
/// `this` must be a valid object of this class.
///
/// # Confidence
/// high - `lwz r6,-23232(r13)` + `lwz r6,20(r6)` gives the allocator slot, and
/// `clrlwi r5,r5,24` pins the byte argument.
pub unsafe fn fn_80006b60(this: usize, param_2: u32, param_3: u8) -> u32 {
    if platform::read_u32(this + 0xBC) != 0 {
        return 1;
    }

    // observed 0x80006B88: `lwz r6,-23232(r13)` -> r13-0x5AC0 = 0x80429BC0.
    let allocator_owner = platform::read_u32(sda1(0x5AC0)) as usize;
    let allocator = platform::read_u32(allocator_owner + 0x14);

    let handle = platform::call4(
        FN_8008F1B0,
        (this + 0xC0) as u32,
        param_2,
        param_3 as u32,
        allocator,
    );
    platform::write_u32(this + 0xBC, handle);

    if handle == 0 {
        return 0;
    }

    // observed 0x80006BB0: `addi r5,r13,-32768` -> 0x80427680 (SDA names).
    platform::call2(FN_80163660, this as u32, handle);

    1
}

/// Releases the resource at `+0xBC` and the sub-object at `+0xC0`.
///
/// The release goes through virtual slot `+0x10` of the object at
/// `r13-0x5AC0` (`0x80429BC0`), i.e. the same global that owns the allocator
/// used by [`fn_80006b60`].
///
/// # Original
/// - Ghidra: `FUN_80006BF0` at `0x80006BF0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006bf0_FUN_80006bf0.c`
///
/// # Arguments
/// * `this` - owner object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x80163690(this)`, one virtual call with `(owner, handle)`, a `+0xBC` clear
/// and `0x8008F2B0(this + 0xC0)`.
///
/// # Safety
/// `this` must be a valid object of this class.
///
/// # Confidence
/// high - the `lwz r3,-23232(r13)` / `lwz r12,16(r3)` / `lwz r12,16(r12)` chain
/// is explicit at `0x80006C14..0x80006C28`.
///
/// # Ghidra artefact
/// Ghidra prints `FUN_80163690()` with no argument; `r3` still holds `this`, and
/// `0x80163690`'s own export declares `(int param_1)`.
pub unsafe fn fn_80006bf0(this: usize) {
    if platform::read_u32(this + 0xBC) != 0 {
        platform::call1(FN_80163690, this as u32);

        let owner = platform::read_u32(sda1(0x5AC0)) as usize;
        let slot = vtable_entry(owner + 0x10, 0x10);
        platform::call2(slot, owner as u32, platform::read_u32(this + 0xBC));

        platform::write_u32(this + 0xBC, 0);
    }
    platform::call1(FN_8008F2B0, (this + 0xC0) as u32);
}

/// Global factory for the object at `r13-0x5AC0`.
///
/// It builds the 0x20-byte description through `0x8016E500`, allocates the
/// 0x1C-byte object with `0x802B9240`, constructs it with `0x8016A5E0`, stores
/// the result in the global and finishes with `0x8016A690`. The function always
/// returns `1`; a failed allocation stores a null global but still returns `1`.
///
/// # Original
/// - Ghidra: `FUN_80006C50` at `0x80006C50` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006c50_FUN_80006c50.c`
///
/// # Arguments
/// * `param_1`, `param_2` - swapped into the first two arguments of
///   `0x8016E500` (the original does `r3 = param_2`, `r4 = param_1`).
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Allocates and constructs the global object, writes `r13-0x5AC0`.
///
/// # Safety
/// The allocator must be initialised; the caller must run on the target.
///
/// # Confidence
/// high - the register swap is `or r0,r3,r3` / `or r3,r4,r4` /
/// `or r4,r0,r0` at `0x80006C64..0x80006C74`.
pub unsafe fn fn_80006c50(param_1: u32, param_2: u32) -> u32 {
    let handle = platform::call5(
        FN_8016E500,
        param_2,
        param_1,
        0x802F_E3A0,
        0x20,
        4,
    );
    let object = platform::call3(FN_802B9240, 0x1C, handle, 4);

    let value = if object != 0 {
        platform::call1(FN_8016A5E0, object)
    } else {
        0
    };

    platform::write_u32(sda1(0x5AC0), value);
    platform::call3(FN_8016A690, value, handle, 0x20);

    1
}

/// Constructs the `0x802FE3D8` object.
///
/// Fields set, in order: byte `+0x0C = 0x80`, vtable `+0x08 = 0x802FE3D8`,
/// member `+0x10` built by `0x802ACB40` and re-vtabled to `0x803294D8`, member
/// `+0x30` built by `0x802B4D30`, `+0x84 = 0`, `+0x88`/`+0x8C = 0.0f`,
/// `+0xA4 = 0`. When the region query `0x801DD1D0` returns `1`, `+0x74` is set
/// to `0.7307692f`, `+0x78` to `1.0f` and bit `0x20` is set in the byte at
/// `+0x80`. Finally `+0xA0 = 0`.
///
/// # Original
/// - Ghidra: `FUN_80006CD0` at `0x80006CD0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006cd0_FUN_80006cd0.c`
///
/// # Arguments
/// * `this` - storage for the new object (at least `+0xA8` bytes).
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Two member constructors, one region query, and the field stores listed above.
///
/// # Safety
/// `this` must be writable for `+0xA8` bytes.
///
/// # Confidence
/// high - every constant was resolved from the image:
/// `0x8042B080 = 0.0f`, `0x8042B084 = 0.7307692f`, `0x8042B088 = 1.0f`.
///
/// # Ghidra artefact
/// `0x802DCD70`-style save helpers do not appear here, but `unaff_r2` does: the
/// `lfs f0,-32768(r2)` loads at `0x80006D1C`, `0x80006D48` and `0x80006D4C`
/// are the three constants above. The two `stfs` to `8(r1)`/`12(r1)` at
/// `0x80006D54`/`0x80006D58` only fill dead stack slots and are omitted.
pub unsafe fn fn_80006cd0(this: usize) -> usize {
    platform::write_u8(this + 0x0C, 0x80);
    platform::write_u32(this + 8, 0x802F_E3D8);
    platform::call1(FN_802ACB40, (this + 0x10) as u32);
    platform::write_u32(this + 0x10, 0x8032_94D8);
    platform::call1(FN_802B4D30, (this + 0x30) as u32);

    let zero = read_f32(F32_ZERO_8042B080);
    platform::write_u32(this + 0x84, 0);
    write_f32(this + 0x88, zero);
    write_f32(this + 0x8C, zero);
    platform::write_u32(this + 0xA4, 0);

    if call0_u32(FN_801DD1D0) as u8 == 1 {
        write_f32(this + 0x74, read_f32(F32_RATIO_8042B084));
        write_f32(this + 0x78, read_f32(F32_ONE_8042B088));
        let flags = platform::read_u8(this + 0x80);
        platform::write_u8(this + 0x80, flags | 0x20);
    }

    platform::write_u8(this + 0xA0, 0);
    this
}

/// Bare deleting destructor: frees `this` when `free_flag > 0`, then returns it.
///
/// `fn_800075d0` at `0x800075D0` is the byte-identical twin of this function.
///
/// # Original
/// - Ghidra: `FUN_80006DF0` at `0x80006DF0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006df0_FUN_80006df0.c`
///
/// # Arguments
/// * `this` - object to release; null is a no-op.
/// * `free_flag` - positive values call `0x802B9280(this)`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Optionally one free.
///
/// # Safety
/// `this` must be null or a pointer returned by this class's allocator.
///
/// # Confidence
/// high - the body is two `cmpi` + `bl 0x802b9280`.
pub unsafe fn fn_80006df0(this: usize, free_flag: i32) -> usize {
    if this != 0 && free_flag > 0 {
        platform::call1(FN_802B9280, this as u32);
    }
    this
}

/// Destroys a `0x802FE3D8` object, optionally releasing its storage.
///
/// # Original
/// - Ghidra: `FUN_80006E30` at `0x80006E30` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006e30_FUN_80006e30.c`
///
/// # Arguments
/// * `this` - object to destroy; null is a no-op.
/// * `free_flag` - positive values also release `this`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// `0x802B4DB0(this + 0x30, -1)`, `0x802ACB80(this + 0x10, 0)` and an optional
/// free.
///
/// # Safety
/// `this` must be null or a valid object of this class.
///
/// # Confidence
/// high - `addi r3,r3,48` (the `+0x30` member) and `addic. r3,r30,16` (the
/// non-null test on `+0x10`) are explicit at `0x80006E58`/`0x80006E60`.
pub unsafe fn fn_80006e30(this: usize, free_flag: i32) -> usize {
    if this != 0 {
        platform::call2(FN_802B4DB0, (this + 0x30) as u32, 0xFFFF_FFFF);
        if this + 0x10 != 0 {
            platform::call2(FN_802ACB80, (this + 0x10) as u32, 0);
        }
        if free_flag > 0 {
            platform::call1(FN_802B9280, this as u32);
        }
    }
    this
}

/// Tail-dispatches virtual slot `+0x38` of the member at `this + 0x10` with the
/// low bit of `+0xA4` as the single argument.
///
/// # Original
/// - Ghidra: `FUN_80006F70` at `0x80006F70` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006f70_FUN_80006f70.c`
///
/// # Arguments
/// * `this` - owner of the member at `+0x10` and the flag word at `+0xA4`.
///
/// # Returns
/// The virtual slot's `r3`.
///
/// # Side effects
/// One indirect call that may do anything.
///
/// # Safety
/// `this + 0x10` must hold a valid object with a vtable of at least `+0x3C`.
///
/// # Confidence
/// high - `lwz r0,164(r3)` + `clrlwi r0,r0,31.` + `lwzu r12,16(r3)` +
/// `lwz r12,56(r12)` at `0x80006F70..0x80006F90`.
pub unsafe fn fn_80006f70(this: usize) -> u32 {
    let flag = platform::read_u32(this + 0xA4) & 1;
    let member = this + 0x10;
    let slot = vtable_entry(member, 0x38);
    platform::call2(slot, member as u32, flag)
}

/// Copies four floats out of the `+0x10` member and pushes them into the
/// `+0x30` member through virtual slot `+0x30`.
///
/// `0x802AD520(&local, this + 0x10)` fills four floats; they are stored at
/// `+0x64`, `+0x68`, `+0x6C` and `+0x70` (Ghidra prints `+100`, `+0x68`,
/// `+0x6c`, `+0x70` - `100` is decimal for `0x64`). The virtual slot then
/// receives `(this + 0x10, this + 0x30)`.
///
/// # Original
/// - Ghidra: `FUN_80006FA0` at `0x80006FA0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006fa0_FUN_80006fa0.c`
///
/// # Arguments
/// * `this` - owner of both members.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `0x802AD520` call, four `f32` stores, one virtual call.
///
/// # Safety
/// Both members must be valid objects; the virtual slot must exist.
///
/// # Confidence
/// high - the four `lfs`/`stfs` pairs at `0x80006FC0..0x80006FE4` and the
/// `lwz r12,48(r12)` dispatch at `0x80006FE8` are explicit.
pub unsafe fn fn_80006fa0(this: usize) {
    let mut values = [0u32; 4];
    platform::call2(FN_802AD520, values.as_mut_ptr() as u32, (this + 0x10) as u32);

    write_f32(this + 0x64, f32::from_bits(values[0]));
    write_f32(this + 0x68, f32::from_bits(values[1]));
    write_f32(this + 0x6C, f32::from_bits(values[2]));
    write_f32(this + 0x70, f32::from_bits(values[3]));

    let member = this + 0x10;
    let slot = vtable_entry(member, 0x30);
    platform::call2(slot, member as u32, (this + 0x30) as u32);
}

/// Attaches a candidate child object and notifies the two member objects.
///
/// The function first checks whether the child pointer at `+0x20` is already
/// present. If it is, the object is considered attached and the function
/// returns `1`. Otherwise the candidate is `param_3`, falling back to the
/// pointer at `+0x84`; a null candidate is rejected with `0`.
///
/// The candidate is activated through [`fn_80006a20`] (`param_2` is forwarded
/// as its third argument), then the member at `+0x10` receives the activation
/// result through virtual slot `+0x0C` together with `candidate + 0x0C`. If
/// that slot returns non-zero, the member at `+8` is notified through virtual
/// slot `+0x10` with `this`. The slot result is returned.
///
/// # Original
/// - Ghidra: `FUN_80007230` at `0x80007230` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007230_FUN_80007230.c`
///
/// # Arguments
/// * `this` - owner object; reads `+0x20`, `+0x84`, `+0x10` and `+8`.
/// * `param_2` - forwarded to [`fn_80006a20`] in `r5` (role `// unknown:`).
/// * `param_3` - preferred candidate child; `0` selects `*(this + 0x84)`.
///
/// # Returns
/// `1` when `+0x20` was already non-null, otherwise the result of the
/// `+0x10` virtual slot; `0` when the candidate is null or activation fails.
///
/// # Side effects
/// One [`fn_80006a20`] call, one virtual `+0x0C` call, and on success one
/// virtual `+0x10` call. No direct memory writes.
///
/// # Safety
/// `this` must be a valid object; `param_3` (or `+0x84`) must point at an
/// object acceptable to [`fn_80006a20`], and both members must have the
/// vtables described above.
///
/// # Confidence
/// high - the branch tree and all three call sites were decoded from the raw
/// words at `0x80007230..0x800072F0`, and they match the Ghidra C exactly.
pub unsafe fn fn_80007230(this: usize, param_2: u32, param_3: u32) -> u32 {
    // observed 0x8000724C: `lwz r0,32(r3)` + `cmpwi r0,0`; a non-null child
    // at +0x20 means "already attached".
    if platform::read_u32(this + 0x20) != 0 {
        return 1;
    }

    // observed 0x80007260..0x80007278: use r5, else the fallback at +0x84.
    let mut candidate = param_3;
    if candidate == 0 {
        candidate = platform::read_u32(this + 0x84);
    }
    if candidate == 0 {
        return 0;
    }

    // observed 0x8000727C..0x80007294: `mr r5,r4; mr r3,r31; li r4,0;
    // bl 0x80006A20`, then `cmpwi r3,0` / `mr r4,r3`.
    let mut result = fn_80006a20(candidate as usize, 0, param_2);
    if result == 0 {
        return 0;
    }

    // observed 0x800072A0..0x800072B4: `lwz r12,16(r30)` (the +0x10 member's
    // vtable), slot +0x0C, r3 = this+0x10, r4 = activation result,
    // r5 = candidate+0x0C.
    let member = this + 0x10;
    let slot = vtable_entry(member, 0x0C);
    result = platform::call3(slot, member as u32, result, candidate + 0x0C);

    if result != 0 {
        // observed 0x800072C4..0x800072D4: `lwz r12,8(r30)`, slot +0x10,
        // r3 = this.
        let owner = this + 8;
        let notify = vtable_entry(owner, 0x10);
        platform::call1(notify, this as u32);
    }

    result
}

// ===========================================================================
// 7. Global collection lookups (0x80007300..0x80007470)
//
// All three lookups share one shape:
//
//   object = virtual_slot_3C(this->+0x20, arg, 1);
//   if (object == 0) return 0;
//   for (node = virtual_slot_0C(object); node; node = *node)
//       if (node == ANCHOR) return object;
//   return 0;
//
// The anchors are three adjacent words in the small-data area
// (`r13-0x48A8`, `r13-0x48B0`, `r13-0x48A0`), i.e. the list heads of three
// sibling collections.
// ===========================================================================

/// Tail-dispatches virtual slot `+0x3C` of the object at `this + 0x20` with
/// `(child, arg, 1)`.
///
/// # Original
/// - Ghidra: `FUN_80007300` at `0x80007300` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007300_FUN_80007300.c`
///
/// # Arguments
/// * `this` - owner of the child pointer at `+0x20`.
/// * `arg` - forwarded in `r4`.
///
/// # Returns
/// The virtual slot's `r3`.
///
/// # Side effects
/// One indirect call that may do anything.
///
/// # Safety
/// `*(this + 0x20)` must be a valid object with a vtable of at least `+0x40`.
///
/// # Confidence
/// high - `lwz r3,32(r3)` / `addi r5,r0,1` / `lwz r12,60(r12)` / `bcctr`.
pub unsafe fn fn_80007300(this: usize, arg: u32) -> u32 {
    let child = platform::read_u32(this + 0x20) as usize;
    let slot = vtable_entry(child, 0x3C);
    platform::call3(slot, child as u32, arg, 1)
}

/// Returns `this`'s collection object when the small-data node `r13-0x48A8` is
/// on its list, otherwise `0`.
///
/// # Original
/// - Ghidra: `FUN_80007320` at `0x80007320` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007320_FUN_80007320.c`
///
/// # Arguments
/// * `this` - owner passed to [`fn_80007300`].
/// * `arg` - forwarded to [`fn_80007300`] in `r4`; role `// unknown:`.
///
/// # Returns
/// The collection object, or `0` when the lookup or the search fails.
///
/// # Side effects
/// One [`fn_80007300`] call, one virtual `+0x0C` call and a linked-list walk.
///
/// # Safety
/// The collection object's `+0x0C` slot must return a valid node chain.
///
/// # Confidence
/// high - the two `cmpi r3,0` tests, the `cmplw r3,r31` node comparison and the
/// `b 0x80007374` loop back-edge are explicit at `0x80007338..0x80007384`.
///
/// # Ghidra artefact
/// Ghidra prints two arguments for `FUN_80007300()`; the original forwards its
/// own `r3` and `r4` unchanged, which is why the second argument is kept here.
pub unsafe fn fn_80007320(this: usize, arg: u32) -> usize {
    let object = fn_80007300(this, arg);
    if object == 0 {
        return 0;
    }

    // observed 0x80007344: `addi r31,r13,-18600` -> r13-0x48A8 = 0x8042ADD8.
    let anchor = sda1(0x48A8);

    let mut node = platform::call1(vtable_entry(object as usize, 0x0C), object);
    while node != 0 {
        if node as usize == anchor {
            return object as usize;
        }
        node = platform::read_u32(node as usize);
    }

    0
}

/// Same search as [`fn_80007320`], against the anchor at `r13-0x48B0`
/// (`0x8042ADD0`).
///
/// # Original
/// - Ghidra: `FUN_800073D0` at `0x800073D0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800073d0_FUN_800073d0.c`
///
/// # Arguments
/// * `this` - owner passed to [`fn_80007300`].
/// * `arg` - forwarded to [`fn_80007300`] in `r4`.
///
/// # Returns
/// The collection object, or `0`.
///
/// # Side effects
/// Identical to [`fn_80007320`].
///
/// # Safety
/// Same as [`fn_80007320`].
///
/// # Confidence
/// high - `addi r31,r13,-18608` at `0x800073F4` pins the anchor; the rest is
/// instruction-for-instruction the same as `0x80007320`.
pub unsafe fn fn_800073d0(this: usize, arg: u32) -> usize {
    let object = fn_80007300(this, arg);
    if object == 0 {
        return 0;
    }

    let anchor = sda1(0x48B0);

    let mut node = platform::call1(vtable_entry(object as usize, 0x0C), object);
    while node != 0 {
        if node as usize == anchor {
            return object as usize;
        }
        node = platform::read_u32(node as usize);
    }

    0
}

/// Same search as [`fn_80007320`], against the anchor at `r13-0x48A0`
/// (`0x8042ADE0`).
///
/// # Original
/// - Ghidra: `FUN_80007470` at `0x80007470` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007470_FUN_80007470.c`
///
/// # Arguments
/// * `this` - owner passed to [`fn_80007300`].
/// * `arg` - forwarded to [`fn_80007300`] in `r4`.
///
/// # Returns
/// The collection object, or `0`.
///
/// # Side effects
/// Identical to [`fn_80007320`].
///
/// # Safety
/// Same as [`fn_80007320`].
///
/// # Confidence
/// high - `addi r31,r13,-18592` at `0x80007494` pins the anchor; the rest is
/// instruction-for-instruction the same as `0x80007320`.
pub unsafe fn fn_80007470(this: usize, arg: u32) -> usize {
    let object = fn_80007300(this, arg);
    if object == 0 {
        return 0;
    }

    let anchor = sda1(0x48A0);

    let mut node = platform::call1(vtable_entry(object as usize, 0x0C), object);
    while node != 0 {
        if node as usize == anchor {
            return object as usize;
        }
        node = platform::read_u32(node as usize);
    }

    0
}

/// Bare deleting destructor: frees `this` when `free_flag > 0`, then returns it.
///
/// Byte-identical twin of [`fn_80006df0`] at a second entry point.
///
/// # Original
/// - Ghidra: `FUN_800075D0` at `0x800075D0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800075d0_FUN_800075d0.c`
///
/// # Arguments
/// * `this` - object to release; null is a no-op.
/// * `free_flag` - positive values call `0x802B9280(this)`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Optionally one free.
///
/// # Safety
/// `this` must be null or a pointer returned by this class's allocator.
///
/// # Confidence
/// high - identical instruction words to `0x80006DF0` apart from the
/// displacement of the single `bl 0x802b9280`.
pub unsafe fn fn_800075d0(this: usize, free_flag: i32) -> usize {
    if this != 0 && free_flag > 0 {
        platform::call1(FN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// 8. Band-completion leaves (0x80006790..0x80007510)
//
// Wave 11 translated every entry in the band that was at least 16 bytes and
// skipped seven smaller ones. They are collected here so the band's Ghidra
// index has no unexplained gap. Four are ordinary leaves; three are 4-byte
// tail branches (`b target`) whose Ghidra export shows the *target* body
// because Ghidra followed the branch. Each forwarder calls its target by
// address and keeps the original argument registers.
// ===========================================================================

/// Empty leaf: the complete original body is the single instruction `blr`.
///
/// # Original
/// - Ghidra: `FUN_80006790` at `0x80006790` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006790_FUN_80006790.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Safety
/// Always safe to call; it touches no memory and no registers beyond the
/// return.
///
/// # Confidence
/// high - `0x4E800020` is the only word in the body.
pub unsafe fn fn_80006790() {}

/// Tail branch to [`FN_80163280`]: drains the frame-callback list.
///
/// The original instruction is `b 0x80163280` at `0x800067A0`; the branch
/// does not set LR, so the target returns directly to the thunk's caller. The
/// Rust makes the same call explicitly.
///
/// # Original
/// - Ghidra: `thunk_FUN_80163280` at `0x800067A0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800067a0_thunk_FUN_80163280.c`
///   (Ghidra followed the branch and exported the target body)
///
/// # Arguments
/// None (the target reads no arguments in its own export).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever the target does: it runs the registered frame callbacks, walks
/// the list at `0x80377AB0` through virtual slot `+0x0C`, and calls the
/// matching teardown helper.
///
/// # Safety
/// The target must be callable in the current runtime state and its list must
/// be initialised.
///
/// # Confidence
/// high - the raw word at `0x800067A0` is `0x4815CAE0`, which decodes to
/// `b 0x80163280`.
pub unsafe fn fn_800067a0() {
    // observed 0x800067A0: `b 0x80163280` (tail branch, LK = 0).
    platform::call0(FN_80163280);
}

/// Builds the inline-member pointer at `this + 0x04`.
///
/// The original computes `this + 0x0C` and stores it at `this + 0x04`, the
/// standard Metrowerks idiom for "point the member at its inline storage".
///
/// # Original
/// - Ghidra: `FUN_80006A40` at `0x80006A40` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006a40_FUN_80006a40.c`
///
/// # Arguments
/// * `this` - object that owns the member; writes `+0x04`, reads `+0x0C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One `u32` store at `this + 0x04`.
///
/// # Safety
/// `this` must be valid for at least `0x10` bytes.
///
/// # Confidence
/// high - `addi r0,r3,12` / `stw r0,4(r3)` / `blr` are the whole body.
pub unsafe fn fn_80006a40(this: usize) {
    // observed 0x80006A40: `addi r0,r3,0x0C` + `stw r0,4(r3)`.
    platform::write_u32(this + 4, (this + 0x0C) as u32);
}

/// Tail branch to [`fn_80006b60`] with the third argument forced to `0`.
///
/// # Original
/// - Ghidra: `FUN_80006BE0` at `0x80006BE0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006be0_FUN_80006be0.c`
///
/// # Arguments
/// * `this` - owner object, forwarded in `r3`.
/// * `param_2` - second argument of [`fn_80006b60`], forwarded in `r4`.
///
/// # Returns
/// The target's result: `1` when a handle exists afterwards, `0` on failure.
///
/// # Side effects
/// Same as [`fn_80006b60`]: may allocate and writes `+0xBC`.
///
/// # Safety
/// Same as [`fn_80006b60`]: `this` must be a valid object of its class.
///
/// # Confidence
/// high - `li r5,0` + `b 0x80006B60`; Ghidra prints a `void` result because
/// it models the thunk from the caller side, but `r3` is the target's result.
pub unsafe fn fn_80006be0(this: usize, param_2: u32) -> u32 {
    // observed 0x80006BE0: `li r5,0` + `b 0x80006B60`.
    fn_80006b60(this, param_2, 0)
}

/// Tail branch to [`FN_80163850`]: inserts a node into the sorted list at
/// `0x80377AB0`.
///
/// # Original
/// - Ghidra: `thunk_FUN_80163850` at `0x80006EA0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80006ea0_thunk_FUN_80163850.c`
///   (Ghidra followed the branch and exported the target body)
///
/// # Arguments
/// * `node` - forwarded in `r3`; the target compares `node + 0x0C` with each
///   existing node's key.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The target inserts `node`, which rewrites list links.
///
/// # Safety
/// `node` must be a valid list node whose `+0x0C` key byte is readable.
///
/// # Confidence
/// high - the raw word at `0x80006EA0` is `0x4815C9B0`, which decodes to
/// `b 0x80163850`.
pub unsafe fn fn_80006ea0(node: u32) {
    // observed 0x80006EA0: `b 0x80163850` (tail branch, LK = 0).
    platform::call1(FN_80163850, node);
}

/// Returns the child pointer stored at `this + 0x20`.
///
/// This is the getter paired with [`fn_80007230`] (which tests the same word)
/// and [`fn_80007300`] (which dispatches through it).
///
/// # Original
/// - Ghidra: `FUN_80007220` at `0x80007220` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007220_FUN_80007220.c`
///
/// # Arguments
/// * `this` - object that owns the child pointer.
///
/// # Returns
/// The raw word at `this + 0x20` (`0` when no child is attached).
///
/// # Side effects
/// None.
///
/// # Safety
/// `this` must be valid for at least `0x24` bytes.
///
/// # Confidence
/// high - `lwz r3,32(r3)` / `blr` are the whole body.
pub unsafe fn fn_80007220(this: usize) -> u32 {
    // observed 0x80007220: `lwz r3,0x20(r3)`.
    platform::read_u32(this + 0x20)
}

/// Tail branch to the actor/scene base initialiser [`FN_80164FD0`].
///
/// The thunk is 4 bytes (`b 0x80164FD0`). Most callers in the image call the
/// target directly; this entry is kept for completeness and forwards all
/// three argument registers unchanged.
///
/// # Original
/// - Ghidra: `thunk_FUN_80164fd0` at `0x80007510` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007510_thunk_FUN_80164fd0.c`
///   (Ghidra followed the branch and exported the target body)
///
/// # Arguments
/// * `a`, `b`, `c` - the three arguments of [`FN_80164FD0`], forwarded in
///   `r3`, `r4` and `r5`.
///
/// # Returns
/// The target's `r3`: `1` on success, `0` when validation fails.
///
/// # Side effects
/// Same as [`FN_80164FD0`]: builds and writes the sub-object at `a + 4`.
///
/// # Safety
/// Same as [`FN_80164FD0`]: `a` must be a valid owner and `c` a readable
/// scratch buffer (or null).
///
/// # Confidence
/// high - the raw word at `0x80007510` is `0x4815DAC0`, which decodes to
/// `b 0x80164fd0`.
pub unsafe fn fn_80007510(a: u32, b: u32, c: u32) -> u32 {
    // observed 0x80007510: `b 0x80164FD0` (tail branch, LK = 0).
    platform::call3(FN_80164FD0, a, b, c)
}

// ===========================================================================
// 9. View transform, quad matrix and rail polyline (0x8000D740..0x8000FBA0)
//
// Wave 105 finished the reconstructible entries of the `0x80000000` bucket with
// Ghidra C as the source of truth. Wave 110 then added the four entries whose
// exports drop blocks in the middle of the GX FIFO loops (`0x80007DE0`,
// `0x8000C5C0`, `0x8000CA50`, `0x8000E060`); those were decoded instruction by
// instruction from `work/dol/main.elf`, so their section below cites both the
// export and the raw word range.
//
// The `0x802D0E50`/`0x802D0F40` pair fills and submits one GX draw descriptor;
// `0x801C0C60`/`0x801C0C10` compose the 48-byte model matrix; `0x802DCC4C`
// converts a float to a saturating `int` (which the rail code stores as `u16`).
// These are called by address, so the roles above are `// inferred:` from the
// call shape, not from a symbol table.
// ===========================================================================

/// 8-bit-phase waveform evaluator, sibling of [`FN_80237BD0`]; both index a
/// 256-entry table at `0x802F74A0` after reducing the angle modulo a full turn.
///
/// Ghidra: 80237c40 `FUN_80237c40.c`
const FN_80237C40: usize = 0x8023_7C40;

/// Second 8-bit-phase evaluator. The pair is `// inferred:` to be the sine and
/// cosine of the same phase, because `0x8000D740` calls both with one angle.
///
/// Ghidra: 80237bd0 `FUN_80237bd0.c`
const FN_80237BD0: usize = 0x8023_7BD0;

/// `(f1 step, start, end, out, close_flag, samples) -> out_end`: tessellates
/// the 3-float curve through `start`/`start+0x18` into `out` and returns the
/// first free output slot. `// inferred:` from `0x8000F950`'s call shape.
///
/// Ghidra: 800e2f10 `FUN_800e2f10.c`
const FN_800E2F10: usize = 0x800E_2F10;

/// `(f1 value) -> i32`: saturating float-to-int conversion (clamps to `0` and
/// `-1`, then truncates). The rail code masks the result to 16 bits.
///
/// Ghidra: 802dcc4c `FUN_802dcc4c.c`
const FN_802DCC4C: usize = 0x802D_CC4C;

/// `(descriptor, render_object)`: fills the GX draw descriptor from the global
/// render object at `r13-0x47B0` (`0x8042AED0`). `// inferred:` descriptor
/// fill; the callee body is owned by the GX lane.
///
/// Ghidra: 802d0e50 `FUN_802d0e50.c`
const FN_802D0E50: usize = 0x802D_0E50;

/// `(descriptor)`: submits the descriptor filled by [`FN_802D0E50`].
///
/// Ghidra: 802d0f40 `FUN_802d0f40.c`
const FN_802D0F40: usize = 0x802D_0F40;

/// `(f1 x, f2 y, f3 z, matrix)`: applies a translation to the 48-byte matrix.
///
/// Ghidra: 801c0c60 `FUN_801c0c60.c`
const FN_801C0C60: usize = 0x801C_0C60;

/// `(f1 x, f2 y, f3 z, matrix, scratch)`: concatenates one more translation.
/// The caller passes the same matrix twice, so `scratch` aliases the output.
///
/// Ghidra: 801c0c10 `FUN_801c0c10.c`
const FN_801C0C10: usize = 0x801C_0C10;

/// `(matrix, 0)`: loads the matrix into the GX transform slots.
///
/// Ghidra: 801c9940 `FUN_801c9940.c`
const FN_801C9940: usize = 0x801C_9940;

/// `(matrix, 0)`: companion load performed after [`FN_801C9940`].
///
/// Ghidra: 801c99c0 `FUN_801c99c0.c`
const FN_801C99C0: usize = 0x801C_99C0;

/// `()`: resets the GX transform state before the descriptor is built.
///
/// Ghidra: 801c4070 `FUN_801c4070.c`
const FN_801C4070: usize = 0x801C_4070;

/// `(state, value)`: writes one GX state register.
///
/// Ghidra: 801c37c0 `FUN_801c37c0.c`
const FN_801C37C0: usize = 0x801C_37C0;

/// `(a, b, c, d, e)`: GX state command with five literal operands.
///
/// Ghidra: 801c40b0 `FUN_801c40b0.c`
const FN_801C40B0: usize = 0x801C_40B0;

/// Absolute addresses of the `r2` image constants used below (`r2` is pinned to
/// `0x80433080` by `__init_registers`).
///
/// * `0x8042B0E0` = `1/256` (angle-to-phase scale)
/// * `0x8042B0E4` = `0.0`
/// * `0x8042B118` = `1.0`
/// * `0x8042B11C` = `0.0`
/// * `0x8042B120` = `20000.0`
/// * `0x8042B124` = `-10000.0`
/// * `0x8042B128` = `0.5`
/// * `0x8042B138` = `0.5`
/// * `0x8042B13C` = `0.125`
/// * `0x8042B144` = `0.1`
/// * `0x8042B148` = `128.0`
const F32_PHASE_SCALE: usize = 0x8042_B0E0;
const F32_ZERO_B0E4: usize = 0x8042_B0E4;
const F32_ONE_B118: usize = 0x8042_B118;
const F32_ZERO_B11C: usize = 0x8042_B11C;
const F32_20000_B120: usize = 0x8042_B120;
const F32_NEG_10000_B124: usize = 0x8042_B124;
const F32_HALF_B128: usize = 0x8042_B128;
const F32_HALF_B138: usize = 0x8042_B138;
const F32_EIGHTH_B13C: usize = 0x8042_B13C;
const F32_TENTH_B144: usize = 0x8042_B144;
const F32_128_B148: usize = 0x8042_B148;

/// The render object that owns the quad matrices, `r13-0x47B0`.
const SDA_RENDER_OBJECT: usize = 0x47B0;

/// Lazy-init flag/word pairs. `0x5AA8` is shared between the `0x80351690`
/// fill block of [`fn_8000e910`] and the `r13-0x5AA0` pair of
/// [`fn_8000edf0`]; both initialise it to `1.0`, so the shared flag is benign.
const SDA_FILL_FLAG: usize = 0x5AA8;
const SDA_E910_PAIR_FLAG: usize = 0x5AB7;
const SDA_E910_PAIR: usize = 0x5AB0;
const SDA_EDF0_PAIR_FLAG: usize = 0x5A98;
const SDA_EDF0_PAIR: usize = 0x5A90;
const SDA_EDF0_FIRST: usize = 0x5AA0;

/// Global scratch block filled with `1.0` by [`fn_8000e910`] (`0x80351690`).
const GLOBAL_FILL_BLOCK: usize = 0x8035_1690;

/// Calls `addr(value: f32) -> f32` (single-precision in `f1`).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f32(addr: usize, value: f32) -> f32 {
    let f: extern "C" fn(f32) -> f32 = core::mem::transmute(addr);
    f(value)
}

/// Calls `addr(value: f64) -> i32` (`0x802DCC4C`'s ABI).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f64_to_i32(addr: usize, value: f64) -> i32 {
    let f: extern "C" fn(f64) -> i32 = core::mem::transmute(addr);
    f(value)
}

/// Calls `addr(f1, f2, f3, ptr)` (`0x801C0C60`'s ABI).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f64x3_ptr(addr: usize, a: f64, b: f64, c: f64, ptr: usize) {
    let f: extern "C" fn(f64, f64, f64, usize) = core::mem::transmute(addr);
    f(a, b, c, ptr);
}

/// Calls `addr(f1, f2, f3, ptr, ptr)` (`0x801C0C10`'s ABI).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f64x3_ptr2(addr: usize, a: f64, b: f64, c: f64, p1: usize, p2: usize) {
    let f: extern "C" fn(f64, f64, f64, usize, usize) = core::mem::transmute(addr);
    f(a, b, c, p1, p2);
}

/// Calls `addr(f1, a, b, c, d, e) -> u32` (`0x800E2F10`'s ABI).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f32_5u32(
    addr: usize,
    f1: f32,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
) -> u32 {
    let f: extern "C" fn(f32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(addr);
    f(f1, a, b, c, d, e)
}

/// Virtual-slot tail call through a table stored at `object + table_offset`.
///
/// Reads `[object + table_offset]` as the table pointer, then `[table + slot]`
/// as the code address and calls it with `receiver` in `r3`.
///
/// # Safety
/// `object` and the table must be readable at the listed offsets.
#[inline]
unsafe fn tail_call_via_member(object: usize, table_offset: usize, slot: usize, receiver: u32) -> u32 {
    let table = platform::read_u32(object + table_offset) as usize;
    let code = platform::read_u32(table + slot) as usize;
    platform::call1(code, receiver)
}

/// `void`: forwards to the virtual slot `+0x0C` of the object reached through
/// `r13-0x47B0`'s `+0x38` member.
///
/// # Original
/// - Ghidra: `FUN_8000EDD0` at `0x8000EDD0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000edd0_FUN_8000edd0.c`
///
/// # Arguments
/// None. The receiver is the global render object itself.
///
/// # Returns
/// Nothing in the Ghidra prototype; the `bctr` tail branch forwards the
/// callee's `r3` to this function's caller.
///
/// # Side effects
/// Whatever the virtual slot does. Ghidra warns that it could not recover the
/// jump table at `0x8000EDE0`; the raw words are
/// `lwz r3,-0x47B0(r13); lwz r12,0x38(r3); lwz r12,0xC(r12); mtctr r12; bctr`.
///
/// # Safety
/// The global render object and its `+0x38` table must be initialised.
///
/// # Confidence
/// high - the 20-byte body is five instructions and each operand is literal.
pub unsafe fn fn_8000edd0() {
    let global = platform::read_u32(sda1(SDA_RENDER_OBJECT));
    tail_call_via_member(global as usize, 0x38, 0x0C, global);
}

/// `void`: forwards to the virtual slot `+0x18` of the member at `this + 0x24`.
///
/// # Original
/// - Ghidra: `FUN_8000F2E0` at `0x8000F2E0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f2e0_FUN_8000f2e0.c`
///
/// # Arguments
/// * `this` - object that owns the member pointer at `+0x24`.
///
/// # Returns
/// Nothing in the Ghidra prototype; `bctr` forwards the callee's `r3`.
///
/// # Side effects
/// The virtual call's effects. Raw body:
/// `lwz r3,0x24(r3); lwz r12,0(r3); lwz r12,0x18(r12); mtctr r12; bctr`.
///
/// # Safety
/// `this + 0x24` must hold a readable object pointer with a vtable.
///
/// # Confidence
/// high - five instructions, no branch ambiguity.
pub unsafe fn fn_8000f2e0(this: usize) {
    let member = platform::read_u32(this + 0x24);
    platform::call1(vtable_entry(member as usize, 0x18), member);
}

/// Builds the per-actor view words at `this + 0xD0C .. +0xD28` from a scale and
/// a 16-bit angle.
///
/// The angle is first biased by `0x4000` (a quarter turn in the image's
/// 1/65536-turn convention), converted to `f32`, and scaled by `1/256` before
/// the two table evaluators are called. The first result is stored at `+0xD0C`,
/// the second at `+0xD10`, its negation at `+0xD20`, and the scale itself at
/// `+0xD24`/`+0xD28`. `+0xD1C` receives the image constant `0.0`.
///
/// # Original
/// - Ghidra: `FUN_8000D740` at `0x8000D740` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d740_FUN_8000d740.c`
///
/// # Arguments
/// * `scale` - `f1`, the transform scale (also written to `+0xD24`/`+0xD28`).
/// * `this` - `r3`, object that owns the `+0xD0C` block.
/// * `angle` - `r4` (low 16 bits), the phase biased by `0x4000`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Six `f32` stores and two evaluator calls.
///
/// # Safety
/// `this` must be valid through `+0xD2C`.
///
/// # Confidence
/// medium - the stores and both calls are literal; the evaluator pair is
/// `// inferred:` to be sine/cosine from the shared angle and the matrix-like
/// output layout, and the `psq_l` conversion is modelled as an unsigned 16-bit
/// read (Ghidra types the argument `short` because it sees the `sth`).
pub unsafe fn fn_8000d740(scale: f32, this: usize, angle: u16) {
    // observed 0x8000D75C..0x8000D76C: `addi r31,r4,0x4000`, `sth r31,0xA(r1)`,
    // then `psq_l f1,0(r4),0,0`; the GQR converts the stored u16 to f32.
    let phase = ((angle.wrapping_add(0x4000)) as f32) * read_f32(F32_PHASE_SCALE);

    let first = call_f32(FN_80237C40, phase);
    let second = call_f32(FN_80237BD0, phase);

    write_f32(this + 0xD0C, scale * first);
    write_f32(this + 0xD1C, read_f32(F32_ZERO_B0E4));
    write_f32(this + 0xD10, scale * second);
    // observed 0x8000D7A8: `fneg f2,f1` where `f1 = scale * second`.
    write_f32(this + 0xD20, -(scale * second));
    write_f32(this + 0xD24, scale);
    write_f32(this + 0xD28, scale);
}

/// Rebuilds the view words and then drives the rail polyline builder with them.
///
/// The raw function forwards its own `f1` and `r4` unchanged to
/// [`fn_8000d740`], then calls [`fn_8000fba0`] with
/// `(this, this+0xD0C, this+0xD24, this+0xD24, this+0xCA0, 0.0)`.
///
/// # Original
/// - Ghidra: `FUN_8000D820` at `0x8000D820` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000d820_FUN_8000d820.c`
///
/// # Arguments
/// * `this` - `r3`, the object whose `+0xD0C` view block is rebuilt.
/// * `angle` - `r4`, forwarded to [`fn_8000d740`].
/// * `scale` - `f1`, forwarded to [`fn_8000d740`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// All of [`fn_8000d740`] plus the polyline work of [`fn_8000fba0`].
///
/// # Safety
/// `this` must be valid through `+0xD2C` and its polyline arrays.
///
/// # Confidence
/// high - the argument registers are literal at `0x8000D838..0x8000D850`;
/// Ghidra drops `f1`/`r4` from its prototype because the callee has no
/// recovered signature, not because they are unused.
pub unsafe fn fn_8000d820(this: usize, angle: u16, scale: f32) {
    fn_8000d740(scale, this, angle);
    fn_8000fba0(
        this,
        this + 0xD0C,
        this + 0xD24,
        this + 0xD24,
        this + 0xCA0,
        read_f32(F32_ZERO_B0E4),
    );
}

/// Builds and submits one quad's transform matrix from the global render
/// object and two 2-float vectors.
///
/// The `u16` fields at `global + 8` and `global + 0xC` are converted through
/// int and scaled by `param_4[0]`/`param_4[1]`; `0x801C0C60` seeds the 48-byte
/// matrix, `0x801C0C10` concatenates the translation, and the matrix is then
/// loaded into the GX transform slots.
///
/// # Original
/// - Ghidra: `FUN_8000E910` at `0x8000E910` (732 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000e910_FUN_8000e910.c`
///
/// # Arguments
/// * `translate` - `f1`, the `z` translation handed to `0x801C0C10`.
/// * `flag` - `r3`, selects the degenerate (`0`) or half-scale (`1`) path.
/// * `uv_flag` - `r4`, enables the extra `0xD` GX state pair when non-zero.
/// * `scale` - `r5`, pointer to the two scale floats that multiply the global
///   object's `u16` fields.
/// * `offset` - `r6`, pointer to the 2-float translation applied last.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The GX preamble, the lazy init of the `0x80351690` block and the
/// `r13-0x5AB0` pair, one descriptor fill/submit and the matrix compose/load
/// calls, using 56-byte (descriptor) and 48-byte (matrix) stack temporaries.
///
/// # Safety
/// Both pointers must be readable for 8 bytes; the GX FIFO must be writable.
///
/// # Confidence
/// medium - every store and call operand was decoded from the raw words, but
/// `0x801C0C60`/`0x801C0C10`/`0x802D0E50` belong to other lanes and their roles
/// are `// inferred:` from the argument shapes. The `2^52` integer-conversion
/// idiom is reproduced as a `u16` cast.
pub unsafe fn fn_8000e910(
    translate: f32,
    flag: u32,
    uv_flag: u32,
    scale: usize,
    offset: usize,
) {
    let global = platform::read_u32(sda1(SDA_RENDER_OBJECT)) as usize;

    // observed 0x8000E948..0x8000E998: the fixed GX preamble, plus the
    // optional 0xD pair.
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call5(FN_801C40B0, 0, 9, 1, 4, 0);
    if uv_flag != 0 {
        platform::call2(FN_801C37C0, 0x0D, 1);
        platform::call5(FN_801C40B0, 0, 0x0D, 1, 0, 0);
    }

    // observed 0x8000E99C..0x8000E9EC: lazy global fill blocks.
    if platform::read_u8(sda1(SDA_FILL_FLAG)) == 0 {
        let one = read_f32(F32_ONE_B118);
        write_f32(GLOBAL_FILL_BLOCK, one);
        write_f32(GLOBAL_FILL_BLOCK + 4, one);
        write_f32(GLOBAL_FILL_BLOCK + 8, one);
        platform::write_u8(sda1(SDA_FILL_FLAG), 1);
    }
    if platform::read_u8(sda1(SDA_E910_PAIR_FLAG)) == 0 {
        let zero = read_f32(F32_ZERO_B11C);
        write_f32(sda1(SDA_E910_PAIR), zero);
        write_f32(sda1(SDA_E910_PAIR) + 4, zero);
        platform::write_u8(sda1(SDA_E910_PAIR_FLAG), 1);
    }

    // observed 0x8000E9F0: `addi r3,r1,0x40`; `0x802D0E50(&desc, global)`.
    let mut desc = [0u32; 14];
    let desc_ptr = desc.as_mut_ptr() as usize;
    platform::call2(FN_802D0E50, desc_ptr as u32, global as u32);

    // observed 0x8000E9FC..0x8000EA48: bit 0x40 plus the flag-dependent 0x41.
    let mut flags = platform::read_u16(desc_ptr + 0x34) | 0x40;
    if flag == 0 {
        if platform::read_u32(desc_ptr + 0x04) != 0 {
            platform::write_u32(desc_ptr + 0x04, 0);
            flags |= 0x01;
        }
    } else if platform::read_u32(desc_ptr + 0x04) != 1 {
        platform::write_u32(desc_ptr + 0x04, 1);
        flags |= 0x01;
    }
    platform::write_u16(desc_ptr + 0x34, flags);

    // observed 0x8000EA4C..0x8000EA9C: descriptor field overrides + submit.
    platform::write_u32(desc_ptr, 0);                                  // +0x00
    platform::write_u32(desc_ptr + 0x18, read_f32(F32_ONE_B118).to_bits());
    platform::write_u32(desc_ptr + 0x1C, read_f32(F32_20000_B120).to_bits());
    platform::write_u32(desc_ptr + 0x20, platform::read_u32(sda1(SDA_E910_PAIR)));
    platform::write_u32(desc_ptr + 0x24, platform::read_u32(sda1(SDA_E910_PAIR) + 4));
    platform::write_u32(desc_ptr + 0x28, platform::read_u32(GLOBAL_FILL_BLOCK));
    platform::write_u32(desc_ptr + 0x2C, platform::read_u32(GLOBAL_FILL_BLOCK + 4));
    platform::write_u32(desc_ptr + 0x30, platform::read_u32(GLOBAL_FILL_BLOCK + 8));
    platform::call1(FN_802D0F40, desc_ptr as u32);

    // observed 0x8000EAA0..0x8000EB10: `(u16)` of the two float fields at
    // `global+8` / `global+0xC`, scaled by `scale[0]` / `scale[1]` (r5/r30).
    let x = read_f32(scale)
        * ((read_f32(global + 0x08) as i32) as u16 as f32);
    let y = read_f32(scale + 4)
        * ((read_f32(global + 0x0C) as i32) as u16 as f32);

    let mut matrix = [0u32; 12];
    let matrix_ptr = matrix.as_mut_ptr() as usize;

    // observed 0x8000EAFC..0x8000EB10.
    call_f64x3_ptr(
        FN_801C0C60,
        x as f64,
        y as f64,
        read_f32(F32_ONE_B118) as f64,
        matrix_ptr,
    );

    if flag == 0 {
        // observed 0x8000EB1C..0x8000EB30: `(0, 0, translate)`.
        call_f64x3_ptr2(
            FN_801C0C10,
            0.0,
            0.0,
            translate as f64,
            matrix_ptr,
            matrix_ptr,
        );
    } else {
        // observed 0x8000EB50..0x8000EB78: `(0.5*-x, 0.5*-y, translate)`.
        let half = read_f32(F32_HALF_B128);
        call_f64x3_ptr2(
            FN_801C0C10,
            (half * -x) as f64,
            (half * -y) as f64,
            translate as f64,
            matrix_ptr,
            matrix_ptr,
        );
    }

    // observed 0x8000EB7C..0x8000EB90: `(0, 0, -10000)` in both paths.
    call_f64x3_ptr2(
        FN_801C0C10,
        0.0,
        0.0,
        read_f32(F32_NEG_10000_B124) as f64,
        matrix_ptr,
        matrix_ptr,
    );

    // observed 0x8000EB94..0x8000EBB0: apply the caller's 2-float offset (r6)
    // and load the finished matrix.
    call_f64x3_ptr2(
        FN_801C0C10,
        read_f32(offset) as f64,
        read_f32(offset + 4) as f64,
        0.0,
        matrix_ptr,
        matrix_ptr,
    );
    platform::call2(FN_801C9940, matrix_ptr as u32, 0);
    platform::call2(FN_801C99C0, matrix_ptr as u32, 0);
}

/// Lazily initialises two 2-float constants, fills one quad's matrices, emits
/// the quad corners and flushes it.
///
/// The two pairs are `r13-0x5AA0`/`-0x5A9C` and `r13-0x5A90`/`-0x5A8C`; their
/// one-byte "done" flags are `r13-0x5AA8` and `r13-0x5A98`. The first pair is
/// set to `1.0` and the second to `0.0` on first use.
///
/// # Original
/// - Ghidra: `FUN_8000EDF0` at `0x8000EDF0` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000edf0_FUN_8000edf0.c`
///
/// # Arguments
/// * `a` - `r3`, first argument of [`fn_8000e910`] and
///   [`crate::render::gx::fn_8000ebf0`].
/// * `b` - `r4`, second argument of both.
/// * `translate` - `f1`, forwarded unchanged to [`fn_8000e910`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four SDA floats, two SDA flags, the quad matrix and the GX FIFO.
///
/// # Safety
/// The GX FIFO must be writable; the global render object must be live.
///
/// # Confidence
/// high - the two lazy-init blocks and the three calls are literal; `f1` is
/// never written before the call, which is why Ghidra prints no float argument.
pub unsafe fn fn_8000edf0(a: usize, b: usize, translate: f32) {
    if platform::read_u8(sda1(SDA_FILL_FLAG)) == 0 {
        let one = read_f32(F32_ONE_B118);
        write_f32(sda1(SDA_EDF0_FIRST), one);
        write_f32(sda1(SDA_EDF0_FIRST) + 4, one);
        platform::write_u8(sda1(SDA_FILL_FLAG), 1);
    }
    if platform::read_u8(sda1(SDA_EDF0_PAIR_FLAG)) == 0 {
        let zero = read_f32(F32_ZERO_B11C);
        write_f32(sda1(SDA_EDF0_PAIR), zero);
        write_f32(sda1(SDA_EDF0_PAIR) + 4, zero);
        platform::write_u8(sda1(SDA_EDF0_PAIR_FLAG), 1);
    }

    // observed 0x8000EE54..0x8000EE80: `local_10` = pair A, `local_18` = pair B.
    let mut pair_a = [0u32; 2];
    pair_a[0] = platform::read_u32(sda1(SDA_EDF0_FIRST));
    pair_a[1] = platform::read_u32(sda1(SDA_EDF0_FIRST) + 4);
    let mut pair_b = [0u32; 2];
    pair_b[0] = platform::read_u32(sda1(SDA_EDF0_PAIR));
    pair_b[1] = platform::read_u32(sda1(SDA_EDF0_PAIR) + 4);

    fn_8000e910(
        translate,
        a as u32,
        b as u32,
        pair_a.as_mut_ptr() as usize,
        pair_b.as_mut_ptr() as usize,
    );
    crate::render::gx::fn_8000ebf0(a as u32, b as u32);
    fn_8000edd0();
}

/// Tessellates a two-point curve, accumulates its segment lengths and emits
/// `{x, y, u16}` polyline records.
///
/// The output points live at `this + 0xC38` (8 bytes each) and their cumulative
/// lengths at `this + 0xC80` (4 bytes each). The `u16` of every record is the
/// height in 1/128 units: `phase * 128` for a flat line, or
/// `(length/total * span + phase) * 128` when the line is long enough to
/// interpolate. A closing record is appended after the outer loop.
///
/// # Original
/// - Ghidra: `FUN_8000F950` at `0x8000F950` (580 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000f950_FUN_8000f950.c`
///
/// # Arguments
/// * `scale` - `f1`, initial height phase (the "scale" forwarded by
///   [`fn_8000fba0`]).
/// * `start` - `r3`, first curve point (the object base).
/// * `begin` - `r4`, first 3-float control point.
/// * `limit` - `r5`, pointer whose low two floats are the step vector; the
///   outer loop runs while `begin + 8*i < limit - 0x10`.
/// * `pivot` - `r6`, 2-float vector used for the initial half-height.
/// * `out` - `r7`, first output record.
///
/// # Returns
/// `r3`: the slot after the closing record.
///
/// # Side effects
/// Writes 12-byte `{x, y, u16}` records at `out`, plus `this+0xC38`
/// (tessellator output) and `this+0xC80` (cumulative lengths).
///
/// # Safety
/// `start` must own the `0xC38`/`0xC80` arrays; `begin`/`limit`/`pivot`/`out`
/// must be readable/writable for the ranges the loops touch.
///
/// # Confidence
/// medium - the raw words determine the two loops and every store, but
/// `0x800E2F10` (tessellator) and `0x802DCC4C` (saturating float-to-int) are
/// other lanes' helpers, so their roles are `// inferred:` from the call shape.
pub unsafe fn fn_8000f950(
    scale: f32,
    start: usize,
    begin: usize,
    limit: usize,
    pivot: usize,
    out: usize,
) -> usize {
    // observed 0xF9A4..0xF9D0: `f31 = 0.5`, `f28 = f1`,
    // `f30 = f1 - 0.5*pivot[0]`, `f29 = 0.125`, `f27 = 128`, `f26 = 0.1`.
    let half = read_f32(F32_HALF_B138);
    let mut phase = scale;
    let mut base = scale - half * read_f32(pivot);
    let span = half * (read_f32(limit) + read_f32(limit + 4));
    let sample_step = read_f32(F32_EIGHTH_B13C);
    let height_scale = read_f32(F32_128_B148);
    let flat_threshold = read_f32(F32_TENTH_B144);
    let mut cursor = begin;
    let stop = limit.wrapping_sub(0x10);
    let mut dst = out;
    let mut last_point;

    // observed 0xF9D8: the body is entered unconditionally (the test is the
    // `blt 0xF9D8` back-edge at 0xFB14), so this is a do/while.
    loop {
        // observed 0xF9E0..0xF9F0: `(0.125, r24, r24+0x18, this+0xC38, 1, 8)`.
        let end = call_f32_5u32(
            FN_800E2F10,
            sample_step,
            cursor as u32,
            (cursor + 0x18) as u32,
            (start + 0xC38) as u32,
            1,
            8,
        ) as usize;

        // observed 0xFA1C..0xFA44: the first pair is skipped (`b 0xFA3C`), so
        // the length array starts with the segment `[0]..[1]`.
        let mut prev = start + 0xC38;
        let mut cur = prev + 8;
        let mut lengths = start + 0xC80;
        let mut total = 0.0f32;
        while cur < end {
            total += vec2_length(prev, cur) as f32;
            write_f32(lengths, total);
            prev += 8;
            cur += 8;
            lengths += 4;
        }

        // observed 0xFA50/0xFA54/0xFAB4: both paths decrement the end pointer
        // by one record and then enter a do/while, so the last point is left
        // to the closing record at 0xFB18.
        let last = end.wrapping_sub(8);
        let mut point = start + 0xC38;
        if total >= flat_threshold {
            write_f32(dst, read_f32(point));
            write_f32(dst + 4, read_f32(point + 4));
            platform::write_u16(dst + 8, call_f64_to_i32(
                FN_802DCC4C,
                (phase * height_scale) as f64,
            ) as u16);
            dst += 12;

            point += 8;
            let mut length_ptr = start + 0xC80;
            loop {
                write_f32(dst, read_f32(point));
                write_f32(dst + 4, read_f32(point + 4));
                let fraction = read_f32(length_ptr) / total;
                platform::write_u16(dst + 8, call_f64_to_i32(
                    FN_802DCC4C,
                    ((fraction * span + phase) * height_scale) as f64,
                ) as u16);
                dst += 12;
                point += 8;
                length_ptr += 4;
                if point >= last {
                    break;
                }
            }
        } else {
            // observed 0xFA54..0xFA84: `f28 *= 128`, then flat records.
            phase *= height_scale;
            loop {
                write_f32(dst, read_f32(point));
                write_f32(dst + 4, read_f32(point + 4));
                platform::write_u16(dst + 8, call_f64_to_i32(
                    FN_802DCC4C,
                    phase as f64,
                ) as u16);
                dst += 12;
                point += 8;
                if point >= last {
                    break;
                }
            }
        }
        last_point = point;

        // observed 0xFAF8..0xFB14: `f30 += limit[0]`, `f28 += f30`, then the
        // `blt` back-edge.
        base += read_f32(limit);
        phase += base;
        cursor += 8;
        if cursor >= stop {
            break;
        }
    }

    // observed 0xFB18..0xFB34: the closing record is emitted once, after the
    // outer loop, and copies the final `r29` (`point` above).
    write_f32(dst, read_f32(last_point));
    write_f32(dst + 4, read_f32(last_point + 4));
    platform::write_u16(dst + 8, call_f64_to_i32(
        FN_802DCC4C,
        (phase * height_scale) as f64,
    ) as u16);
    dst += 12;

    dst
}

/// Wrapper: build the rail points with [`fn_8000f950`], then feed them to the
/// follower polyline in [`crate::game::path`].
///
/// Raw body: forward `r3..r7` and `f1` to `0x8000F950`, keep its `r3`, then
/// call `0x8000DF60(r3, r7, result)`.
///
/// # Original
/// - Ghidra: `FUN_8000FBA0` at `0x8000FBA0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000fba0_FUN_8000fba0.c`
///
/// # Arguments
/// * `a` - `r3`, the object base and first argument of `0x8000DF60`.
/// * `b`, `c`, `d` - `r4..r6`, forwarded to [`fn_8000f950`].
/// * `e` - `r7`, output buffer and second argument of `0x8000DF60`.
/// * `scale` - `f1`, forwarded to [`fn_8000f950`].
///
/// # Returns
/// Nothing (the Ghidra prototype is `void`; the final `r3` is discarded).
///
/// # Side effects
/// Everything [`fn_8000f950`] does, plus the follower rebuild.
///
/// # Safety
/// Same as [`fn_8000f950`], plus `e` must be a valid follower record.
///
/// # Confidence
/// high - the body is two `mr`s and two `bl`s; the `r3` bridge between them is
/// literal at `0x8000FBC0`.
pub unsafe fn fn_8000fba0(
    a: usize,
    b: usize,
    c: usize,
    d: usize,
    e: usize,
    scale: f32,
) {
    let end = fn_8000f950(scale, a, b, c, d, e);
    crate::game::path::fn_8000df60(a, e, end);
}

// ===========================================================================
// 10. GX ribbon and particle emitters (0x80007DE0..0x8000E840)
//
// These four entries were the last untranslated functions of the
// `0x80000000` bucket. Every earlier pass rejected them because Ghidra's C
// export loses whole blocks inside the vertex loops: the `psq_l`/`psq_st`
// prologues become `ldexpf`/`unaff_GQR` noise and the FIFO loops show one
// store per iteration where the raw stream writes four or six vertices. This
// pass decoded the raw instruction words in `work/dol/main.elf` instead, so
// the bodies below follow the machine code literally, including the store
// order inside each `GXBegin`/`GXEnd` pair.
//
// All four take the object pointer in `r3`; the `bl 0x802DCD4x/5x/6x` at the
// top of each body is one of the compiler's `__savegprlr_N` helpers, not a
// context getter, so `r3` is still the caller's `this` when the helper
// returns. The matching `__restgprlr_N` call in the epilogue is omitted: Rust
// restores its own callee-saved registers.
//
// The image's GX calls are owned by the `render::gx` band. They are invoked by
// absolute address and described only by what this file observes: the argument
// registers and whether the call reads or writes the FIFO.
// ===========================================================================

/// GX write-gather FIFO, the destination of every vertex store below.
const GX_FIFO: usize = 0xCC00_8000;

/// `(ptr, 0/1)`: resets one of the three GX array/stream slots the effect
/// object keeps at `+0x08`, `+0x28` and `+0x48`. `// inferred:` array reset -
/// the callee writes the slot's base/stride words and the second argument
/// selects the "ready" value.
///
/// Ghidra: 801c74c0 `FUN_801c74c0.c`
const FN_801C74C0: usize = 0x801C_74C0;

// `FN_801C4070`, `FN_801C37C0`, `FN_801C40B0` and `FN_801C9940` are declared
// in section 9 above and reused here.

/// `()`: begins a render object (`0x80164F50`'s band).
///
/// Ghidra: 80164f50 `FUN_80164f50.c`
const FN_80164F50: usize = 0x8016_4F50;

/// `(mode)`: GX pipeline/CP setup call.
///
/// Ghidra: 801c6ac0 `FUN_801c6ac0.c`
const FN_801C6AC0: usize = 0x801C_6AC0;

/// `(seven literals)`: GX transform/CP setup call. Only `0x8000CA50` uses it.
///
/// Ghidra: 801c6af0 `FUN_801c6af0.c`
const FN_801C6AF0: usize = 0x801C_6AF0;

/// `(index)`: selects the active GX texture/tev stage count.
///
/// Ghidra: 801c4a20 `FUN_801c4a20.c`
const FN_801C4A20: usize = 0x801C_4A20;

/// `(stage, a, b, c, d, e)`: per-stage texture setup call.
///
/// Ghidra: 801c47d0 `FUN_801c47d0.c`
const FN_801C47D0: usize = 0x801C_47D0;

/// `(count)`: GX texture-stage commit.
///
/// Ghidra: 801c88f0 `FUN_801c88f0.c`
const FN_801C88F0: usize = 0x801C_88F0;

/// `(mode)`: GX channel/lighting setup.
///
/// Ghidra: 801c8110 `FUN_801c8110.c`
const FN_801C8110: usize = 0x801C_8110;

/// `(a, b, c, d, e)`: GX TEV stage-order command.
///
/// Ghidra: 801c8640 `FUN_801c8640.c`
const FN_801C8640: usize = 0x801C_8640;

/// `(vertex_format, attribute, component_count, type)`: writes one GX vertex
/// attribute descriptor (`POS` is attribute `0`, `TEX0` is `1`).
///
/// Ghidra: 801c8790 `FUN_801c8790.c`
const FN_801C8790: usize = 0x801C_8790;

/// `(stage, a, b, c, d)`: TEV colour-combiner inputs.
///
/// Ghidra: 801c82f0 `FUN_801c82f0.c`
const FN_801C82F0: usize = 0x801C_82F0;

/// `(stage, a, b, c, d, e)`: TEV alpha-combiner inputs.
///
/// Ghidra: 801c8370 `FUN_801c8370.c`
const FN_801C8370: usize = 0x801C_8370;

/// `(stage, a, b, c, d)`: TEV colour-combiner operation.
///
/// Ghidra: 801c8330 `FUN_801c8330.c`
const FN_801C8330: usize = 0x801C_8330;

/// `(stage, a, b, c, d, e)`: TEV alpha-combiner operation.
///
/// Ghidra: 801c83d0 `FUN_801c83d0.c`
const FN_801C83D0: usize = 0x801C_83D0;

/// `(mode)`: GX blend-mode setup.
///
/// Ghidra: 801c8eb0 `FUN_801c8eb0.c`
const FN_801C8EB0: usize = 0x801C_8EB0;

/// `(a, b, c, d)`: GX blend factor setup.
///
/// Ghidra: 801c8dc0 `FUN_801c8dc0.c`
const FN_801C8DC0: usize = 0x801C_8DC0;

/// `(a, b, c)`: GX logic-op/blend setup.
///
/// Ghidra: 801c8e70 `FUN_801c8e70.c`
const FN_801C8E70: usize = 0x801C_8E70;

/// `(a, b, c, d, e)`: GX cull/compare setup.
///
/// Ghidra: 801c86c0 `FUN_801c86c0.c`
const FN_801C86C0: usize = 0x801C_86C0;

/// `(r3 = 0, f1..f4 = colour, pointer)`: GX fog/clear-colour setup with four
/// equal components. The pointer receives a temporary word the callee uses as
/// scratch.
///
/// Ghidra: 801c8920 `FUN_801c8920.c`
const FN_801C8920: usize = 0x801C_8920;

/// `(a, b, c)`: GX fog-range setup.
///
/// Ghidra: 801c8cb0 `FUN_801c8cb0.c`
const FN_801C8CB0: usize = 0x801C_8CB0;

/// `(mode)`: GX z-buffer/compare setup.
///
/// Ghidra: 801c5860 `FUN_801c5860.c`
const FN_801C5860: usize = 0x801C_5860;

/// `(mode)`: GX alpha-test setup.
///
/// Ghidra: 801c8f90 `FUN_801c8f90.c`
const FN_801C8F90: usize = 0x801C_8F90;

/// `(channel, &packed_colour)`: writes one GX colour register from the four
/// bytes at the pointer. `0x801C8430`'s export confirms the read
/// (`_DAT_cc008000 = *param_2 >> 4 & 0xff000 | ...`).
///
/// Ghidra: 801c8430 `FUN_801c8430.c`
const FN_801C8430: usize = 0x801C_8430;

/// `(primitive, vertex_format, vertex_count)`: opens one GX primitive.
///
/// Ghidra: 801c5570 `FUN_801c5570.c`
const FN_801C5570: usize = 0x801C_5570;

/// `(0)`: closes the transform block opened after the model matrix.
///
/// Ghidra: 801c9a60 `FUN_801c9a60.c`
const FN_801C9A60: usize = 0x801C_9A60;

/// `(mode)`: companion of the previous call, used by the C5C0/CA50 pair.
///
/// Ghidra: 801c9d60 `FUN_801c9d60.c`
const FN_801C9D60: usize = 0x801C_9D60;

/// `(matrix, matrix)`: normalises/prepares the 48-byte matrix in place.
/// Only `0x80007DE0` calls it.
///
/// Ghidra: 801c07a0 `FUN_801c07a0.c`
const FN_801C07A0: usize = 0x801C_07A0;

/// `() -> u32`: returns the view/camera matrix handle stored next to the
/// caller's stack slot; the emitters pass it to [`FN_802540B0`].
///
/// Ghidra: 80164b30 `FUN_80164b30.c`
const FN_80164B30: usize = 0x8016_4B30;

/// `(handle, matrix)`: copies/converts the view matrix into a 48-byte buffer.
///
/// Ghidra: 802540b0 `FUN_802540b0.c`
const FN_802540B0: usize = 0x8025_40B0;

/// `(object, matrix)`: builds the object's 48-byte model matrix.
///
/// Ghidra: 8016a190 `FUN_8016a190.c`
const FN_8016A190: usize = 0x8016_A190;

/// `(src, src2, dst)`: concatenates two 48-byte matrices.
///
/// Ghidra: 801c0540 `FUN_801c0540.c`
const FN_801C0540: usize = 0x801C_0540;

// `r2`-relative constants. `r2` is pinned to `0x80433080` by
// `__init_registers`, so each absolute address below is the one the raw
// instruction prints.
const F32_HALF_B098: usize = 0x8042_B098; //  0.5
const F32_NEG_HALF_B09C: usize = 0x8042_B09C; // -0.5
const F32_ZERO_B0A0: usize = 0x8042_B0A0; //  0.0
const F32_TENTH_B0B0: usize = 0x8042_B0B0; //  0.1
const F32_150_B0B4: usize = 0x8042_B0B4; //  150.0
const F32_FOUR_B0B8: usize = 0x8042_B0B8; //  4.0
const F32_PHASE_SCALE_B0BC: usize = 0x8042_B0BC; //  1/256
const F32_EIGHT_B0D4: usize = 0x8042_B0D4; //  8.0
const F32_32_B0D8: usize = 0x8042_B0D8; //  32.0
const F32_ZERO_B100: usize = 0x8042_B100; //  0.0
const F32_16_B104: usize = 0x8042_B104; //  16.0
const F32_PHASE_SCALE_B108: usize = 0x8042_B108; //  1/256
const F32_512_B10C: usize = 0x8042_B10C; //  512.0

/// Calls `addr(f1, f2, f3, f4, r3, ptr)` (`0x801C8920`'s ABI).
///
/// # Safety
/// `addr` must be a PowerPC function with this exact ABI.
#[inline]
unsafe fn call_f32x4_ptr(
    addr: usize,
    arg3: u32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    ptr: usize,
) {
    let f: extern "C" fn(f32, f32, f32, f32, u32, usize) = core::mem::transmute(addr);
    f(a, b, c, d, arg3, ptr);
}

/// Writes one `{f32 x, f32 y, u8 s, u8 t}` ribbon vertex to the GX FIFO.
///
/// # Safety
/// The GX FIFO must be writable (hardware register block).
#[inline]
unsafe fn write_uv8_vertex(x: f32, y: f32, s: u8, t: u8) {
    write_f32(GX_FIFO, x);
    write_f32(GX_FIFO, y);
    platform::write_u8(GX_FIFO, s);
    platform::write_u8(GX_FIFO, t);
}

/// Writes one `{f32 x, f32 y, s16 s, s16 t}` ribbon vertex to the GX FIFO.
///
/// # Safety
/// The GX FIFO must be writable (hardware register block).
#[inline]
unsafe fn write_uv16_vertex(x: f32, y: f32, s: u16, t: u16) {
    write_f32(GX_FIFO, x);
    write_f32(GX_FIFO, y);
    platform::write_u16(GX_FIFO, s);
    platform::write_u16(GX_FIFO, t);
}

/// Writes one `{f32 x, f32 y, f32 z, s16 s0, s16 t0, s16 s1, s16 t1}` ribbon
/// vertex to the GX FIFO.
///
/// # Safety
/// The GX FIFO must be writable (hardware register block).
#[inline]
unsafe fn write_xyz_uv16_vertex(
    x: f32,
    y: f32,
    z: f32,
    s0: u16,
    t0: u16,
    s1: u16,
    t1: u16,
) {
    write_f32(GX_FIFO, x);
    write_f32(GX_FIFO, y);
    write_f32(GX_FIFO, z);
    platform::write_u16(GX_FIFO, s0);
    platform::write_u16(GX_FIFO, t0);
    platform::write_u16(GX_FIFO, s1);
    platform::write_u16(GX_FIFO, t1);
}

/// Writes one `{f32 x, f32 y, f32 z, u8 r, u8 g, u8 b, u8 a, s16 s0, s16 t0,
/// s16 s1, s16 t1}` ribbon vertex to the GX FIFO.
///
/// # Safety
/// The GX FIFO must be writable (hardware register block).
#[inline]
unsafe fn write_xyz_color_uv16_vertex(
    x: f32,
    y: f32,
    z: f32,
    r: u8,
    g: u8,
    b: u8,
    a: u8,
    s0: u16,
    t0: u16,
    s1: u16,
    t1: u16,
) {
    write_f32(GX_FIFO, x);
    write_f32(GX_FIFO, y);
    write_f32(GX_FIFO, z);
    platform::write_u8(GX_FIFO, r);
    platform::write_u8(GX_FIFO, g);
    platform::write_u8(GX_FIFO, b);
    platform::write_u8(GX_FIFO, a);
    platform::write_u16(GX_FIFO, s0);
    platform::write_u16(GX_FIFO, t0);
    platform::write_u16(GX_FIFO, s1);
    platform::write_u16(GX_FIFO, t1);
}

/// Draws the 200-slot billboard ribbon of one effect object.
///
/// The record array at `this + 0x780` holds three `f32` per entry: the centre
/// `x`, the centre `y` and a radius factor. For record `i` the radius is
/// `record.z * (this[+0x68] + 0.1 * (i & 7))`. Each record becomes one
/// `GXBegin(0x80, 0, 4)` quad whose corners are
/// `(record.x + r*sin, record.y + r*cos)` at the phases `0`, `+0x4000`,
/// `+0x8000`, `+0xC000` of the 16-bit angle at `this + 0x1270`, with the
/// quadrant UVs `(0,0)`, `(1,0)`, `(1,1)`, `(0,1)`.
///
/// Two overrides apply. The first eleven records use the fixed radius `150.0`
/// and force the alpha byte of both colours to `30` when the mode byte at
/// `+0x1272` is zero. From record `190` on the radius is multiplied by `4.0`
/// and the array at `+0x48` is reset. When the mode byte is non-zero the array
/// is instead selected by `i % 3` (`+0x28`, `+0x48`, `+0x08` for remainders
/// `0`, `1`, `2`).
///
/// # Original
/// - Ghidra: `FUN_80007de0` at `0x80007DE0` (1632 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80007de0_FUN_80007de0.c`
/// - Raw words: `0x80007DE0..0x8000843C` (408 instructions)
///
/// # Arguments
/// * `this` - `r3`, the effect object; it owns the `200 * 12` record array at
///   `+0x780`, the phase at `+0x1270`, the mode byte at `+0x1272`, the scale at
///   `+0x68` and the two packed colours at `+0x70`/`+0x74`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The GX state block (twenty-two setup calls, one fog colour, three
/// fixed-function calls and two colour registers), two 48-byte matrix scratch
/// blocks, and 200 quads of four `{f32 x, f32 y, u8 s, u8 t}` vertices in the
/// FIFO.
///
/// # Safety
/// `this` must be valid through `+0x1272`; the FIFO must be writable.
///
/// # Confidence
/// medium - every branch, store and call operand is literal in the raw words;
/// the `0x80237BD0`/`0x80237C40` pair and the `psq_l` phase conversion are
/// modelled as in [`fn_8000d740`], and every `0x801Cxxxx` callee belongs to the
/// GX band, so its exact API name is `// inferred:`.
pub unsafe fn fn_80007de0(this: usize) {
    let zero = read_f32(F32_ZERO_B0A0);
    let phase_scale = read_f32(F32_PHASE_SCALE_B0BC);

    // observed 0x80007E50..0x80007FC4: fixed two-stage GX preamble.
    platform::call2(FN_801C74C0, (this + 0x08) as u32, 0);
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call2(FN_801C37C0, 0x0D, 1);
    platform::call5(FN_801C40B0, 0, 9, 0, 4, 0);
    platform::call5(FN_801C40B0, 0, 0x0D, 1, 0, 0);
    platform::call0(FN_80164F50);
    platform::call1(FN_801C6AC0, 0);
    platform::call1(FN_801C4A20, 1);
    platform::call6(FN_801C47D0, 0, 1, 4, 0x3C, 0, 0x7D);
    platform::call1(FN_801C88F0, 1);
    platform::call1(FN_801C8110, 0);
    platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    platform::call4(FN_801C8790, 0, 0, 0, 4);
    platform::call5(FN_801C82F0, 0, 4, 2, 8, 0x0F);
    platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
    platform::call5(FN_801C8330, 0, 7, 1, 4, 7);
    platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
    platform::call1(FN_801C8EB0, 0);
    platform::call4(FN_801C8DC0, 1, 4, 5, 0x0F);
    platform::call3(FN_801C8E70, 0, 3, 0);
    platform::call5(FN_801C86C0, 4, 0, 1, 4, 0);

    // observed 0x80007FC8..0x80008004: four copies of 0.0 as the fog colour,
    // then the fixed-function block.
    let mut zero_word = 0u32;
    call_f32x4_ptr(
        FN_801C8920,
        0,
        zero,
        zero,
        zero,
        zero,
        &mut zero_word as *mut u32 as usize,
    );
    platform::call3(FN_801C8CB0, 0, 0, 0);
    platform::call1(FN_801C5860, 0);
    platform::call1(FN_801C8F90, 1);

    // observed 0x8000800C..0x80008030: the object's two packed colours.
    let color0 = platform::read_u32(this + 0x70);
    platform::call2(FN_801C8430, 1, &color0 as *const u32 as u32);
    let color1 = platform::read_u32(this + 0x74);
    platform::call2(FN_801C8430, 2, &color1 as *const u32 as u32);

    // observed 0x80008034..0x80008090: view matrix -> scratch, normalise it,
    // then concatenate the object matrix and load the result.
    let view = call0_u32(FN_80164B30);
    let mut base = [0u32; 12];
    let mut model = [0u32; 12];
    let base_ptr = base.as_mut_ptr() as usize;
    let model_ptr = model.as_mut_ptr() as usize;
    platform::call2(FN_802540B0, view, base_ptr as u32);
    platform::call2(FN_801C07A0, base_ptr as u32, base_ptr as u32);
    write_f32(base_ptr + 0x0C, zero);
    write_f32(base_ptr + 0x1C, zero);
    write_f32(base_ptr + 0x2C, zero);
    platform::call2(FN_8016A190, this as u32, model_ptr as u32);
    platform::call3(FN_801C0540, model_ptr as u32, base_ptr as u32, model_ptr as u32);
    platform::call2(FN_801C9940, model_ptr as u32, 0);
    platform::call1(FN_801C9A60, 0);

    let mode = platform::read_u8(this + 0x1272);
    let mut record = this + 0x780;
    for index in 0..200u32 {
        // observed 0x800080DC..0x8000811C: the 2^52 conversion idiom yields
        // `index & 7`; `r17` is the record pointer.
        let step = (index & 7) as f32;
        let mut radius = read_f32(record + 8)
            * (read_f32(this + 0x68) + read_f32(F32_TENTH_B0B0) * step);

        let mut color0 = platform::read_u32(this + 0x70);
        let mut color1 = platform::read_u32(this + 0x74);

        if mode == 0 {
            if index <= 10 {
                // observed 0x8000812C..0x80008160: fixed radius, alpha 30.
                radius = read_f32(F32_150_B0B4);
                color0 = (color0 & 0xFFFF_FF00) | 0x1E;
                color1 = (color1 & 0xFFFF_FF00) | 0x1E;
                platform::call2(FN_801C8430, 1, &color0 as *const u32 as u32);
                platform::call2(FN_801C8430, 2, &color1 as *const u32 as u32);
            } else if index >= 12 {
                // observed 0x80008164..0x80008198: records 12.. reset +0x28.
                platform::call2(FN_801C74C0, (this + 0x28) as u32, 0);
                platform::call2(FN_801C8430, 1, &color0 as *const u32 as u32);
                platform::call2(FN_801C8430, 2, &color1 as *const u32 as u32);
            }
        } else {
            // observed 0x800081B8..0x80008210: the remainder selects the
            // array; the raw compares 0 -> +0x28, 1 -> +0x48, 2 -> +0x08.
            match index % 3 {
                0 => {
                    platform::call2(FN_801C74C0, (this + 0x28) as u32, 0);
                }
                1 => {
                    platform::call2(FN_801C74C0, (this + 0x48) as u32, 0);
                }
                _ => {
                    platform::call2(FN_801C74C0, (this + 0x08) as u32, 0);
                }
            }
        }

        // observed 0x8000819C..0x800081B4: the last ten records grow 4x.
        if mode == 0 && index >= 190 {
            radius *= read_f32(F32_FOUR_B0B8);
            platform::call2(FN_801C74C0, (this + 0x48) as u32, 0);
        }

        // observed 0x80008214..0x80008238: the unrotated fallback corners are
        // the unit square scaled by the radius (±0.5).
        let mut sin0 = read_f32(F32_NEG_HALF_B09C);
        let mut cos0 = read_f32(F32_HALF_B098);
        let mut sin1 = read_f32(F32_HALF_B098);
        let mut cos1 = read_f32(F32_HALF_B098);
        let mut sin2 = read_f32(F32_HALF_B098);
        let mut cos2 = read_f32(F32_NEG_HALF_B09C);
        let mut sin3 = read_f32(F32_NEG_HALF_B09C);
        let mut cos3 = read_f32(F32_NEG_HALF_B09C);

        if index <= 10 {
            // observed 0x8000823C..0x8000831C: four phases, sine first.
            let angle = platform::read_u16(this + 0x1270);
            let quarter = angle.wrapping_add(0x4000);
            let half = angle.wrapping_add(0x8000);
            let three_quarters = angle.wrapping_add(0xC000);
            sin0 = call_f32(FN_80237BD0, angle as f32 * phase_scale);
            cos0 = call_f32(FN_80237C40, angle as f32 * phase_scale);
            sin1 = call_f32(FN_80237BD0, quarter as f32 * phase_scale);
            cos1 = call_f32(FN_80237C40, quarter as f32 * phase_scale);
            sin2 = call_f32(FN_80237BD0, half as f32 * phase_scale);
            cos2 = call_f32(FN_80237C40, half as f32 * phase_scale);
            sin3 = call_f32(FN_80237BD0, three_quarters as f32 * phase_scale);
            cos3 = call_f32(FN_80237C40, three_quarters as f32 * phase_scale);
        }

        // observed 0x80008320..0x800083C4: one quad, four uv8 vertices.
        let x = read_f32(record);
        let y = read_f32(record + 4);
        platform::call3(FN_801C5570, 0x80, 0, 4);
        write_uv8_vertex(x + radius * sin0, y + radius * cos0, 0, 0);
        write_uv8_vertex(x + radius * sin1, y + radius * cos1, 1, 0);
        write_uv8_vertex(x + radius * sin2, y + radius * cos2, 1, 1);
        write_uv8_vertex(x + radius * sin3, y + radius * cos3, 0, 1);
        record += 12;
    }
}

/// Draws the four-corner ribbon strip of one effect object.
///
/// The four parallel `Vec3` arrays at `+0x48`, `+0x33D8`, `+0x6768` and
/// `+0x9AF8` each hold `count` records (12 bytes each, `count` at `+0xCE90`).
/// Each index emits one `GXBegin(0x80, 0, 4)` quad built from those four
/// positions, two texture coordinates per vertex and the packed colours at
/// `+0xCE88`/`+0xCE8C`. The first two corners use texture 0 `(0, 0)` /
/// `(63, 0)` and the second pair `(0, 1023)` / `(63, 1023)`; texture 1 carries
/// the longitudinal scroll from the `s16` angle at `+0xCE98`, plus `1023` on
/// the far pair. The last two positions drop their `y` by the image constant
/// `32.0`, which folds the ribbon into a box section.
///
/// # Original
/// - Ghidra: `FUN_8000c5c0` at `0x8000C5C0` (1168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000c5c0_FUN_8000c5c0.c`
/// - Raw words: `0x8000C5C0..0x8000CA4C` (292 instructions)
///
/// # Arguments
/// * `this` - `r3`, the effect object; it owns the four position arrays, the
///   count at `+0xCE90`, the angle at `+0xCE98` and the two colours at
///   `+0xCE88`/`+0xCE8C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The GX state block, a 48-byte view matrix, and `count` quads of four
/// `{f32 x, f32 y, f32 z, s16 s0, s16 t0, s16 s1, s16 t1}` vertices.
///
/// # Safety
/// `this` must be valid through `+0xCE9A` and the four `count * 12` arrays.
///
/// # Confidence
/// medium - the raw words fix the loop, the four positions, the seven stores per
/// vertex and the count; the `psq_l` phase conversion is modelled as an
/// unsigned 16-bit read and every `0x801Cxxxx` callee is another lane's GX
/// helper, so the callee identity is `// inferred:`.
pub unsafe fn fn_8000c5c0(this: usize) {
    let zero = read_f32(F32_ZERO_B0A0);

    // observed 0x8000C5D8..0x8000C818: two-stage GX preamble.
    platform::call2(FN_801C74C0, (this + 0x08) as u32, 0);
    platform::call2(FN_801C74C0, (this + 0x28) as u32, 1);
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call2(FN_801C37C0, 0x0D, 1);
    platform::call2(FN_801C37C0, 0x0E, 1);
    platform::call5(FN_801C40B0, 0, 9, 1, 4, 0);
    platform::call5(FN_801C40B0, 0, 0x0D, 1, 3, 10);
    platform::call5(FN_801C40B0, 0, 0x0E, 1, 3, 10);
    platform::call0(FN_80164F50);
    platform::call1(FN_801C6AC0, 0);
    platform::call1(FN_801C4A20, 2);
    platform::call6(FN_801C47D0, 0, 1, 4, 0x3C, 0, 0x7D);
    platform::call6(FN_801C47D0, 1, 1, 5, 0x3C, 0, 0x7D);
    platform::call1(FN_801C88F0, 2);
    platform::call1(FN_801C8110, 0);
    platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    platform::call4(FN_801C8790, 0, 0, 0, 0xFF);
    platform::call5(FN_801C82F0, 0, 0x0F, 0x0F, 0x0F, 8);
    platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
    platform::call5(FN_801C8330, 0, 7, 7, 7, 4);
    platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
    platform::call4(FN_801C8790, 1, 1, 1, 0xFF);
    platform::call5(FN_801C82F0, 1, 4, 2, 8, 0);
    platform::call6(FN_801C8370, 1, 0, 0, 0, 1, 0);
    platform::call5(FN_801C8330, 1, 4, 7, 7, 0);
    platform::call6(FN_801C83D0, 1, 0, 0, 0, 1, 0);
    platform::call1(FN_801C8EB0, 0);
    platform::call4(FN_801C8DC0, 1, 4, 5, 0x0F);
    platform::call3(FN_801C8E70, 1, 3, 1);
    platform::call5(FN_801C86C0, 4, 0, 1, 4, 0);
    platform::call1(FN_801C9D60, 1);

    // observed 0x8000C820..0x8000C860.
    let mut zero_word = 0u32;
    call_f32x4_ptr(
        FN_801C8920,
        0,
        zero,
        zero,
        zero,
        zero,
        &mut zero_word as *mut u32 as usize,
    );
    platform::call3(FN_801C8CB0, 0, 0, 0);
    platform::call1(FN_801C5860, 0);
    platform::call1(FN_801C8F90, 1);

    // observed 0x8000C864..0x8000C890: colours at +0xCE88/+0xCE8C.
    let color0 = platform::read_u32(this + 0xCE88);
    platform::call2(FN_801C8430, 1, &color0 as *const u32 as u32);
    let color1 = platform::read_u32(this + 0xCE8C);
    platform::call2(FN_801C8430, 2, &color1 as *const u32 as u32);

    // observed 0x8000C894..0x8000C8E4: view matrix and model-matrix concat.
    let view = call0_u32(FN_80164B30);
    let mut base = [0u32; 12];
    let mut model = [0u32; 12];
    let base_ptr = base.as_mut_ptr() as usize;
    let model_ptr = model.as_mut_ptr() as usize;
    platform::call2(FN_802540B0, view, base_ptr as u32);
    write_f32(base_ptr + 0x0C, zero);
    write_f32(base_ptr + 0x1C, zero);
    write_f32(base_ptr + 0x2C, zero);
    platform::call2(FN_8016A190, this as u32, model_ptr as u32);
    platform::call3(FN_801C0540, model_ptr as u32, base_ptr as u32, model_ptr as u32);
    platform::call2(FN_801C9940, model_ptr as u32, 0);
    platform::call1(FN_801C9A60, 0);

    let count = platform::read_i32(this + 0xCE90);
    if count <= 0 {
        platform::call1(FN_801C9D60, 0);
        return;
    }

    let angle = platform::read_u16(this + 0xCE98);
    let lift = read_f32(F32_32_B0D8);
    let mut corner0 = this + 0x48;
    let mut corner1 = this + 0x33D8;
    let mut corner2 = this + 0x6768;
    let mut corner3 = this + 0x9AF8;

    for index in 0..count as u32 {
        // observed 0x8000C91C..0x8000C938: the shift chain reduces to the
        // zero texture coordinate and the constant (0x80, 4) for every index
        // below 2^28; it is kept literal so the arithmetic stays visible.
        let bit30 = (index >> 30) & 1;
        let base = index.wrapping_shl(28).wrapping_sub(bit30);
        let half = base.wrapping_shl(4);
        let edge = half.wrapping_add(bit30).wrapping_shl(6) & 0x03FF_FFFF;
        let primitive = base.wrapping_add(128);
        let vertices = half.wrapping_add(bit30).wrapping_add(4);
        let far = edge.wrapping_add(0x3F);

        platform::call3(FN_801C5570, primitive, 0, vertices);

        // observed 0x8000C944..0x8000C9CC: near pair, texture 0 at v = 0.
        write_xyz_uv16_vertex(
            read_f32(corner0),
            read_f32(corner0 + 4),
            read_f32(corner0 + 8),
            edge as u16,
            0,
            edge as u16,
            angle,
        );
        write_xyz_uv16_vertex(
            read_f32(corner1),
            read_f32(corner1 + 4),
            read_f32(corner1 + 8),
            far as u16,
            0,
            far as u16,
            angle,
        );

        // observed 0x8000C9D4..0x8000CA04: far pair, `y -= 32.0`,
        // texture 0 at v = 1023.
        write_xyz_uv16_vertex(
            read_f32(corner2),
            read_f32(corner2 + 4) - lift,
            read_f32(corner2 + 8),
            far as u16,
            1023,
            far as u16,
            angle.wrapping_add(1023),
        );
        write_xyz_uv16_vertex(
            read_f32(corner3),
            read_f32(corner3 + 4) - lift,
            read_f32(corner3 + 8),
            edge as u16,
            1023,
            edge as u16,
            angle.wrapping_add(1023),
        );

        corner0 += 12;
        corner1 += 12;
        corner2 += 12;
        corner3 += 12;
    }

    platform::call1(FN_801C9D60, 0);
}

/// Draws the six-vertex ribbon strip of one effect object.
///
/// This is the taller sibling of [`fn_8000c5c0`]. The same four `Vec3` arrays
/// (`+0x48`, `+0x33D8`, `+0x9AF8`, `+0x6768`) and the same count at `+0xCE90`
/// drive it, but each index emits one `GXBegin(0x98, 0, 6)` triangle strip of
/// six vertices: the near pair at texture-0 `v = 0`, the middle pair at
/// `v = 511`, and a far pair whose `y` is lowered by the image constant `8.0`
/// at `v = 1023`. The horizontal texture coordinate advances by
/// `170 * (index % 6)`; the second texture coordinate scrolls with the `s16`
/// angle at `+0xCE98`.
///
/// # Original
/// - Ghidra: `FUN_8000ca50` at `0x8000CA50` (1444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000ca50_FUN_8000ca50.c`
/// - Raw words: `0x8000CA50..0x8000CFF0` (361 instructions)
///
/// # Arguments
/// * `this` - `r3`, the effect object; it owns the four position arrays, the
///   count at `+0xCE90`, the angle at `+0xCE98` and the colours at
///   `+0xCE88`/`+0xCE8C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The GX state block, a 48-byte view matrix, and `count` strips of six
/// `{f32 x, f32 y, f32 z, u8 r, u8 g, u8 b, u8 a, s16 s0, s16 t0, s16 s1, s16 t1}`
/// vertices.
///
/// # Safety
/// `this` must be valid through `+0xCE9A` and the four `count * 12` arrays.
///
/// # Confidence
/// medium - the raw words fix the loop, all six vertices and every colour and
/// texture store; the callees are other lanes' GX helpers and the `psq_l`
/// phase conversion is modelled as an unsigned 16-bit read.
pub unsafe fn fn_8000ca50(this: usize) {
    let zero = read_f32(F32_ZERO_B0A0);

    // observed 0x8000CA68..0x8000CCE8: three-stage GX preamble.
    platform::call2(FN_801C74C0, (this + 0x08) as u32, 0);
    platform::call2(FN_801C74C0, (this + 0x28) as u32, 1);
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call2(FN_801C37C0, 0x0B, 1);
    platform::call2(FN_801C37C0, 0x0D, 1);
    platform::call2(FN_801C37C0, 0x0E, 1);
    platform::call5(FN_801C40B0, 0, 9, 1, 4, 0);
    platform::call5(FN_801C40B0, 0, 0x0B, 1, 5, 0);
    platform::call5(FN_801C40B0, 0, 0x0D, 1, 3, 10);
    platform::call5(FN_801C40B0, 0, 0x0E, 1, 3, 10);
    platform::call0(FN_80164F50);
    platform::call1(FN_801C6AC0, 1);
    platform::call7(FN_801C6AF0, 4, 0, 1, 1, 0, 0, 2);
    platform::call1(FN_801C4A20, 2);
    platform::call6(FN_801C47D0, 0, 1, 4, 0x3C, 0, 0x7D);
    platform::call6(FN_801C47D0, 1, 1, 5, 0x3C, 0, 0x7D);
    platform::call1(FN_801C88F0, 2);
    platform::call1(FN_801C8110, 0);
    platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    platform::call4(FN_801C8790, 0, 0, 0, 4);
    platform::call5(FN_801C82F0, 0, 0x0F, 0x0F, 0x0F, 8);
    platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
    platform::call5(FN_801C8330, 0, 7, 7, 7, 5);
    platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
    platform::call4(FN_801C8790, 1, 1, 1, 0xFF);
    platform::call5(FN_801C82F0, 1, 4, 2, 8, 0);
    platform::call6(FN_801C8370, 1, 0, 0, 0, 1, 0);
    platform::call5(FN_801C8330, 1, 7, 7, 7, 0);
    platform::call6(FN_801C83D0, 1, 0, 0, 0, 1, 0);
    platform::call1(FN_801C8EB0, 0);
    platform::call4(FN_801C8DC0, 1, 4, 5, 0x0F);
    platform::call3(FN_801C8E70, 1, 3, 1);
    platform::call5(FN_801C86C0, 4, 0, 1, 4, 0);
    platform::call1(FN_801C9D60, 1);

    // observed 0x8000CCF4..0x8000CD34.
    let mut zero_word = 0u32;
    call_f32x4_ptr(
        FN_801C8920,
        0,
        zero,
        zero,
        zero,
        zero,
        &mut zero_word as *mut u32 as usize,
    );
    platform::call3(FN_801C8CB0, 0, 0, 0);
    platform::call1(FN_801C5860, 0);
    platform::call1(FN_801C8F90, 1);

    // observed 0x8000CD38..0x8000CD64: colours at +0xCE88/+0xCE8C.
    let color0 = platform::read_u32(this + 0xCE88);
    platform::call2(FN_801C8430, 1, &color0 as *const u32 as u32);
    let color1 = platform::read_u32(this + 0xCE8C);
    platform::call2(FN_801C8430, 2, &color1 as *const u32 as u32);

    // observed 0x8000CD68..0x8000CDB8: view matrix and model-matrix concat.
    let view = call0_u32(FN_80164B30);
    let mut base = [0u32; 12];
    let mut model = [0u32; 12];
    let base_ptr = base.as_mut_ptr() as usize;
    let model_ptr = model.as_mut_ptr() as usize;
    platform::call2(FN_802540B0, view, base_ptr as u32);
    write_f32(base_ptr + 0x0C, zero);
    write_f32(base_ptr + 0x1C, zero);
    write_f32(base_ptr + 0x2C, zero);
    platform::call2(FN_8016A190, this as u32, model_ptr as u32);
    platform::call3(FN_801C0540, model_ptr as u32, base_ptr as u32, model_ptr as u32);
    platform::call2(FN_801C9940, model_ptr as u32, 0);
    platform::call1(FN_801C9A60, 0);

    let count = platform::read_i32(this + 0xCE90);
    if count <= 0 {
        platform::call1(FN_801C9D60, 0);
        return;
    }

    let angle = platform::read_u16(this + 0xCE98);
    let lift = read_f32(F32_EIGHT_B0D4);
    let mut near0 = this + 0x48;
    let mut near1 = this + 0x33D8;
    let mut far = this + 0x9AF8;
    let mut middle = this + 0x6768;

    for index in 0..count as u32 {
        // observed 0x8000CE00..0x8000CE24: `index % 6` via the `0x2AAAAAAB`
        // multiply, then `170 * remainder`.
        let u0 = (index % 6) * 170;
        let u1 = u0 + 169;

        platform::call3(FN_801C5570, 0x98, 0, 6);

        // observed 0x8000CE28..0x8000CE64: near pair, texture 0 at v = 0.
        write_xyz_color_uv16_vertex(
            read_f32(near0),
            read_f32(near0 + 4),
            read_f32(near0 + 8),
            0,
            0,
            0,
            0xFF,
            u0 as u16,
            0,
            u0 as u16,
            angle,
        );
        write_xyz_color_uv16_vertex(
            read_f32(near1),
            read_f32(near1 + 4),
            read_f32(near1 + 8),
            0,
            0,
            0,
            0xFF,
            u1 as u16,
            0,
            u1 as u16,
            angle,
        );

        // observed 0x8000CE68..0x8000CEE4: middle pair, white,
        // texture 0 at v = 511.
        write_xyz_color_uv16_vertex(
            read_f32(far),
            read_f32(far + 4),
            read_f32(far + 8),
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            u0 as u16,
            511,
            u0 as u16,
            angle.wrapping_add(511),
        );
        write_xyz_color_uv16_vertex(
            read_f32(middle),
            read_f32(middle + 4),
            read_f32(middle + 8),
            0xFF,
            0xFF,
            0xFF,
            0xFF,
            u1 as u16,
            511,
            u1 as u16,
            angle.wrapping_add(511),
        );

        // observed 0x8000CEEC..0x8000CF64: far pair, `y -= 8.0`, alpha 0,
        // texture 0 at v = 1023.
        write_xyz_color_uv16_vertex(
            read_f32(far),
            read_f32(far + 4) - lift,
            read_f32(far + 8),
            0xFF,
            0xFF,
            0xFF,
            0,
            u0 as u16,
            1023,
            u0 as u16,
            angle.wrapping_add(1023),
        );
        write_xyz_color_uv16_vertex(
            read_f32(middle),
            read_f32(middle + 4) - lift,
            read_f32(middle + 8),
            0xFF,
            0xFF,
            0xFF,
            0,
            u1 as u16,
            1023,
            u1 as u16,
            angle.wrapping_add(1023),
        );

        near0 += 12;
        near1 += 12;
        far += 12;
        middle += 12;
    }

    platform::call1(FN_801C9D60, 0);
}

/// Draws the ribbon plus rounded end cap of one effect object.
///
/// The function only runs when the byte at `+0x38` (the record count) is at
/// least two. It opens a single `GXBegin(0x98, 0, (count + 9) * 2)` triangle
/// strip and then writes every vertex in one sequence: the first cross-section
/// from the angle of record 0, one two-vertex cross-section per following
/// record (the centre advances by that record's length along the angle, and the
/// next angle is the midpoint of the two record angles plus a quarter turn),
/// eight two-vertex rings of a rounded end cap, and a final closing pair. The
/// record array is at `this + 0x30` (8 bytes per record: `f32` length, `s16`
/// angle); the cross-section radius is `16.0`, the cap width comes from
/// `this + 0x34`, and the longitudinal texture coordinate is the remaining rail
/// length scaled by `512.0` and saturated to `u16`.
///
/// # Original
/// - Ghidra: `FUN_8000e060` at `0x8000E060` (2016 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000e060_FUN_8000e060.c`
/// - Raw words: `0x8000E060..0x8000E83C` (504 instructions)
///
/// # Arguments
/// * `this` - `r3`, the effect object; it owns the record array at `+0x30`,
///   the cap width at `+0x34`, the record count at `+0x38`, and the packed
///   colours at `+0x28`/`+0x2C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// The GX state block, a 48-byte view matrix, and one long triangle strip of
/// `(count + 9) * 2` `{f32 x, f32 y, s16 s, s16 t}` vertices in the FIFO.
///
/// # Safety
/// `this` must be valid through `+0x38`; the record array must hold `count`
/// readable 8-byte records; the FIFO must be writable.
///
/// # Confidence
/// medium - the raw words give the strip length, every vertex and the cap
/// arithmetic; [`crate::game::path::fn_8000e030`] is this repository's
/// rail-length helper, and the `0x801Cxxxx` callees plus the `psq_l` phase
/// conversion are modelled as `// inferred:`.
pub unsafe fn fn_8000e060(this: usize) {
    let count = platform::read_u8(this + 0x38);
    if count < 2 {
        return;
    }

    let zero = read_f32(F32_ZERO_B100);
    let sixteen = read_f32(F32_16_B104);
    let phase_scale = read_f32(F32_PHASE_SCALE_B108);
    let tex_scale = read_f32(F32_512_B10C);

    // observed 0x8000E0DC..0x8000E248: two-stage GX preamble.
    platform::call2(FN_801C74C0, (this + 0x08) as u32, 0);
    platform::call0(FN_801C4070);
    platform::call2(FN_801C37C0, 9, 1);
    platform::call2(FN_801C37C0, 0x0D, 1);
    platform::call5(FN_801C40B0, 0, 9, 0, 4, 0);
    platform::call5(FN_801C40B0, 0, 0x0D, 1, 2, 0x0E);
    platform::call0(FN_80164F50);
    platform::call1(FN_801C6AC0, 0);
    platform::call1(FN_801C4A20, 1);
    platform::call6(FN_801C47D0, 0, 1, 4, 0x3C, 0, 0x7D);
    platform::call1(FN_801C88F0, 1);
    platform::call1(FN_801C8110, 0);
    platform::call5(FN_801C8640, 0, 0, 1, 2, 3);
    platform::call4(FN_801C8790, 0, 0, 0, 4);
    platform::call5(FN_801C82F0, 0, 0x0F, 8, 2, 4);
    platform::call5(FN_801C8330, 0, 7, 4, 1, 7);
    platform::call6(FN_801C8370, 0, 0, 0, 0, 1, 0);
    platform::call6(FN_801C83D0, 0, 0, 0, 0, 1, 0);
    platform::call1(FN_801C8EB0, 0);
    platform::call4(FN_801C8DC0, 1, 4, 5, 0x0F);
    platform::call3(FN_801C8E70, 1, 3, 1);
    platform::call5(FN_801C86C0, 4, 0, 1, 4, 0);

    // observed 0x8000E24C..0x8000E288.
    let mut zero_word = 0u32;
    call_f32x4_ptr(
        FN_801C8920,
        0,
        zero,
        zero,
        zero,
        zero,
        &mut zero_word as *mut u32 as usize,
    );
    platform::call3(FN_801C8CB0, 0, 0, 0);
    platform::call1(FN_801C5860, 2);
    platform::call1(FN_801C8F90, 1);

    // observed 0x8000E290..0x8000E2B4: colours at +0x28/+0x2C.
    let color0 = platform::read_u32(this + 0x28);
    platform::call2(FN_801C8430, 1, &color0 as *const u32 as u32);
    let color1 = platform::read_u32(this + 0x2C);
    platform::call2(FN_801C8430, 2, &color1 as *const u32 as u32);

    // observed 0x8000E2B8..0x8000E308: view matrix and model-matrix concat.
    let view = call0_u32(FN_80164B30);
    let mut base = [0u32; 12];
    let mut model = [0u32; 12];
    let base_ptr = base.as_mut_ptr() as usize;
    let model_ptr = model.as_mut_ptr() as usize;
    platform::call2(FN_802540B0, view, base_ptr as u32);
    write_f32(base_ptr + 0x0C, zero);
    write_f32(base_ptr + 0x1C, zero);
    write_f32(base_ptr + 0x2C, zero);
    platform::call2(FN_8016A190, this as u32, model_ptr as u32);
    platform::call3(FN_801C0540, model_ptr as u32, base_ptr as u32, model_ptr as u32);
    platform::call2(FN_801C9940, model_ptr as u32, 0);
    platform::call1(FN_801C9A60, 0);

    // observed 0x8000E30C..0x8000E330: the rail length left to draw.
    let mut remaining = crate::game::path::fn_8000e030(this) + sixteen;
    let records = platform::read_u32(this + 0x30) as usize;
    let end = records + count as usize * 8;
    let angle = platform::read_u16(this + 0xCE98);
    let mut length = read_f32(records);
    let mut cursor = records + 8;
    let mut center_x = zero;
    let mut center_y = zero;

    // observed 0x8000E334..0x8000E3AC: the strip is opened once, then the
    // first cross-section is written from the phase `angle + 0x4000`.
    let first_angle = angle.wrapping_add(0x4000);
    let first_cos = call_f32(FN_80237C40, first_angle as f32 * phase_scale);
    let first_sin = call_f32(FN_80237BD0, first_angle as f32 * phase_scale);
    let first_x = sixteen * first_cos;
    let first_y = sixteen * first_sin;
    platform::call3(FN_801C5570, 0x98, 0, ((count as u32 + 9) << 1) & 0x1FFFE);
    let mut t = call_f64_to_i32(FN_802DCC4C, (remaining * tex_scale) as f64) as u16;
    write_uv16_vertex(-first_x, -first_y, 0x4000, t);
    write_uv16_vertex(first_x, first_y, 0, t);

    // observed 0x8000E420..0x8000E53C: one cross-section per remaining record.
    let mut edge_angle = angle;
    while cursor < end {
        remaining -= length;
        let next_angle = platform::read_u16(cursor + 4);
        let cos_a = call_f32(FN_80237C40, edge_angle as f32 * phase_scale);
        let sin_a = call_f32(FN_80237BD0, edge_angle as f32 * phase_scale);
        let mid_x = center_x + length * cos_a;
        let mid_y = center_y + length * sin_a;
        let midpoint = edge_angle.wrapping_add(
            edge_angle.wrapping_sub(next_angle) >> 1,
        );
        edge_angle = midpoint.wrapping_add(0x4000);
        center_x = mid_x;
        center_y = mid_y;

        let cos_b = call_f32(FN_80237C40, edge_angle as f32 * phase_scale);
        let sin_b = call_f32(FN_80237BD0, edge_angle as f32 * phase_scale);
        let across_x = sixteen * cos_b;
        let across_y = sixteen * sin_b;
        t = call_f64_to_i32(FN_802DCC4C, (remaining * tex_scale) as f64) as u16;
        write_uv16_vertex(mid_x - across_x, mid_y - across_y, 0x4000, t);
        write_uv16_vertex(mid_x + across_x, mid_y + across_y, 0, t);

        length = read_f32(cursor);
        cursor += 8;
    }

    // observed 0x8000E544..0x8000E6CC: eight rings closing the end.
    let width = read_f32(this + 0x34);
    let last_angle = platform::read_u16(cursor - 4);
    let cap_radius = (length - width) * read_f32(F32_HALF_B098);
    let tie_angle = last_angle.wrapping_add(0x4000);
    let mut ring_angle = 0x7000u16;
    for _ in 0..8 {
        let cos_ring = call_f32(FN_80237C40, ring_angle as f32 * phase_scale);
        let ring = cap_radius * cos_ring + cap_radius;
        let ring_remaining = remaining - ring;

        let cos_last = call_f32(FN_80237C40, last_angle as f32 * phase_scale);
        let sin_last = call_f32(FN_80237BD0, last_angle as f32 * phase_scale);
        let point_x = ring * cos_last + center_x;
        let point_y = ring * sin_last + center_y;

        let sin_ring = call_f32(FN_80237BD0, ring_angle as f32 * phase_scale);
        let ring2 = width * sin_ring + sixteen;
        let cos_tie = call_f32(FN_80237C40, tie_angle as f32 * phase_scale);
        let sin_tie = call_f32(FN_80237BD0, tie_angle as f32 * phase_scale);
        let across_x = ring2 * cos_tie;
        let across_y = ring2 * sin_tie;

        t = call_f64_to_i32(FN_802DCC4C, (ring_remaining * tex_scale) as f64) as u16;
        write_uv16_vertex(point_x - across_x, point_y - across_y, 0x4000, t);
        write_uv16_vertex(point_x + across_x, point_y + across_y, 0, t);

        center_x = point_x;
        center_y = point_y;
        ring_angle = ring_angle.wrapping_sub(0x1000);
    }

    // observed 0x8000E6D0..0x8000E7CC: the closing pair.
    let end_cos = call_f32(FN_80237C40, last_angle as f32 * phase_scale);
    let end_sin = call_f32(FN_80237BD0, last_angle as f32 * phase_scale);
    let closing_x = center_x + sixteen * end_cos;
    let closing_y = center_y + sixteen * end_sin;
    let tie_cos = call_f32(FN_80237C40, tie_angle as f32 * phase_scale);
    let tie_sin = call_f32(FN_80237BD0, tie_angle as f32 * phase_scale);
    let close_x = sixteen * tie_cos;
    let close_y = sixteen * tie_sin;
    write_uv16_vertex(closing_x - close_x, closing_y - close_y, 0x4000, 0);
    write_uv16_vertex(closing_x + close_x, closing_y + close_y, 0, 0);
}

// ===========================================================================
// Wave 228 lane F - nw4r scene-class kernel tables `0x802F8EF0`..`0x802F8F98`
// ===========================================================================
//
// Six seven-entry pointer tables sit back to back in `main.dol`:
//
// | table | slots | dispatch site |
// |---|---|---|
// | `0x802F8EF0` | 7 | [`crate::runtime::os::fn_802556b0`], `direct != 0` |
// | `0x802F8F0C` | 7 | [`crate::runtime::os::fn_802556b0`], `direct == 0` |
// | `0x802F8F28` | 7 | [`crate::runtime::os::fn_802562b0`], `direct != 0` |
// | `0x802F8F44` | 7 | [`crate::runtime::os::fn_802562b0`], `direct == 0` |
// | `0x802F8F60` | 7 | [`crate::runtime::os::fn_80256f90`], `direct != 0` |
// | `0x802F8F7C` | 7 | [`crate::runtime::os::fn_80256f90`], `direct == 0` |
//
// Each wrapper indexes its pair with the gather-format selector
// `(format >> 1) & 7` of the draw packet; selector `7` means "no kernel" and
// returns `0` without a call. All 42 slots carry distinct code pointers, and
// every kernel has the same signature: `void kernel(float* dst, const float*
// src)`.
//
// `dst` is a two-column, four-float transform block:
//
// | offset | meaning |
// |---|---|
// | `+0x00`..`+0x0C` | column 0; `+0x0C` is its translation component |
// | `+0x10`..`+0x1C` | column 1; `+0x1C` is its translation component |
//
// `src` is five floats: `[0]`/`[1]` scale the two columns, `[2]` is the
// rotation angle, `[3]`/`[4]` are the two translation/pivot offsets. The
// angle is multiplied by `0.7111111` before it reaches `0x80237CA0`, i.e. the
// stored angle is in degrees and the sine table uses 256 steps per turn.
//
// Four slots are already translated in [`crate::runtime::os`] because they also
// have `functions.tsv` rows: `0x80254EC0` (table 0 slot 0), `0x80255AA0` and
// `0x80255BC0` (table 1 slots 1 and 0) and `0x80256990` (table 2 slot 5).
// The 38 kernels below have no `functions.tsv` row and no Ghidra export; each
// one was decoded from `work/dol/main.elf` (disassembly:
// `work/wave228/laneF_disasm.txt`).
//
// **Why they are here.** `src/render/geometry.rs` already owns the neighbouring
// nw4r matrix helpers `0x80253C70`, `0x80253D00` and `0x802540B0`; the
// dispatch wrappers stay in `runtime::os` where Wave 97 translated them.
//
// **Dependencies.** [`read_f32`]/[`write_f32`] and one call to `0x80237CA0`,
// the table-driven sine/cosine pair.
// ---------------------------------------------------------------------------

/// `0x80237CA0`: table-driven sine/cosine pair.
///
/// The 16-byte entry at `0x80237CA0` is the public shape of a `0x74`-byte
/// interpolating kernel in the same bucket: it range-reduces the angle, walks the
/// `0x802F74A0` table with `0xC0`-step steps and writes the interpolated sine
/// through its first pointer and the cosine through its second.
///
/// # Returns
/// `(sin, cos)` as single-precision floats.
///
/// # Safety
/// Must be the real `0x80237CA0` entry point.
#[inline]
unsafe fn host_sincos(angle: f64) -> (f32, f32) {
    let mut sin = 0.0f32;
    let mut cos = 0.0f32;
    let f: extern "C" fn(f64, *mut f32, *mut f32) = core::mem::transmute(0x8023_7ca0usize);
    f(angle, &mut sin as *mut f32, &mut cos as *mut f32);
    (sin, cos)
}

// The three small-data constant blocks the kernels read through `r2`
// (`r2 = 0x80433080`). The three angle steps are distinct addresses with the
// same value, `0.7111111f` = 64/90, so a degree angle reaches the sine table as
// `degrees * 256 / 360`.
const F32_ZERO_A_228: usize = 0x8042_E738; //  0.0
const F32_ONE_A_228: usize = 0x8042_E73C; //  1.0
const F32_ANGLE_STEP_A_228: usize = 0x8042_E740; //  0.7111111
const F32_HALF_A_228: usize = 0x8042_E744; //  0.5
const F32_NEG_HALF_A_228: usize = 0x8042_E748; // -0.5
const F32_ZERO_B_228: usize = 0x8042_E750; //  0.0
const F32_ONE_B_228: usize = 0x8042_E754; //  1.0
const F32_ANGLE_STEP_B_228: usize = 0x8042_E758; //  0.7111111
const F32_ZERO_C_228: usize = 0x8042_E760; //  0.0
const F32_HALF_C_228: usize = 0x8042_E764; //  0.5
const F32_ONE_C_228: usize = 0x8042_E768; //  1.0
const F32_ANGLE_STEP_C_228: usize = 0x8042_E76C; //  0.7111111
const F32_NEG_HALF_C_228: usize = 0x8042_E770; // -0.5

/// Writes a two-column scale block and nothing else.
///
/// `dst[0]` is `src[0]`, `dst[5]` is `src[1]`, and both translation components
/// become one minus `src[1]`; the other six entries are zero.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254BE0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254BE0..0x80254C18` (56 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 6 of the seven-entry kernel table at `0x802F8EF0`
///   (table A0), reached from `fn_802556b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254be0(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_A_228);
    let one = read_f32(F32_ONE_A_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    write_f32(dst, s0);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, zero);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, s1);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one - s1);
}

/// Writes the unit-square rotation about `(0.5, 0.5)`.
///
/// Both columns are the sine/cosine pair and both translation components are
/// `0.5 -+ 0.5*sin - 0.5*cos`, i.e. the image moves about the centre of a unit
/// square. The source scales and offsets are ignored.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254C20` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254C20..0x80254CB4` (148 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 5 of the seven-entry kernel table at `0x802F8EF0`
///   (table A0), reached from `fn_802556b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254c20(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_A_228);
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = half * cos;
    let t2 = half * sin;
    let t3 = half - t1;
    write_f32(dst, cos);
    write_f32(dst + 0x4, sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, t3 - t2);
    write_f32(dst + 0x10, -sin);
    write_f32(dst + 0x14, cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, t3 + t2);
}

/// Writes a pure translation block.
///
/// The linear part is the identity and the two translation components are
/// `-src[3]` and `src[4]`; the angle and both scales are ignored.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254CC0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254CC0..0x80254CF8` (56 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 3 of the seven-entry kernel table at `0x802F8EF0`
///   (table A0), reached from `fn_802556b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254cc0(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_A_228);
    let one = read_f32(F32_ONE_A_228);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, one);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, -s3);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, one);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, s4);
}

/// Writes the rotated, scaled, centred block (both axes).
///
/// `src[0]`/`src[1]` scale the two columns, `src[2]` is the rotation angle and
/// `src[3]`/`src[4]` the two offsets subtracted from the `(0.5, 0.5)` pivot.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254D00` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254D00..0x80254DBC` (188 bytes); the
///   closing `bclr 20,0` is the last word, then 4 padding bytes.
/// - Table: slot 4 of the seven-entry kernel table at `0x802F8EF0`
///   (table A0), reached from `fn_802556b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254d00(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_A_228);
    let one = read_f32(F32_ONE_A_228);
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let neg_half = read_f32(F32_NEG_HALF_A_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * cos;
    let t2 = s1 * sin;
    let t3 = s0 * cos;
    let t4 = s0 * sin;
    write_f32(dst, s0 * cos);
    write_f32(dst + 0x4, s0 * sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, neg_half * (t4 + t3 - s0));
    write_f32(dst + 0x10, -t2);
    write_f32(dst + 0x14, s1 * cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one + half * (t2 - t1 - s1));
}

/// Writes the centred rotation with the two source offsets applied.
///
/// Same sine/cosine layout as [`fn_80254c20`] but the offsets `src[3]`/`src[4]`
/// are added to the two translation components; the column scales are ignored.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254DC0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254DC0..0x80254E70` (176 bytes); the
///   closing `bclr 20,0` is the last word.
/// - Table: slot 1 of the seven-entry kernel table at `0x802F8EF0`
///   (table A0), reached from `fn_802556b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254dc0(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_A_228);
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = half * cos;
    let t2 = half * sin;
    let t3 = half - t1;
    write_f32(dst, cos);
    write_f32(dst + 0x4, sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, t3 - t2 - s3);
    write_f32(dst + 0x10, -sin);
    write_f32(dst + 0x14, cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, s4 + (t3 + t2));
}

/// Writes the pivot scale block.
///
/// `src[0]`/`src[1]` scale the two columns and `src[3]`/`src[4]` are the pivot
/// offsets; there is no sine/cosine term.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254E70` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254E70..0x80254EC0` (80 bytes); the
///   closing `bclr 20,0` is the last word, then 224 padding bytes.
/// - Table: slot 2 of the seven-entry kernel table at `0x802F8EF0`
///   (table A0), reached from `fn_802556b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254e70(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_A_228);
    let one = read_f32(F32_ONE_A_228);
    let s0 = read_f32(src);
    let s3 = read_f32(src + 0xc);
    let s1 = read_f32(src + 0x4);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, s0);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, -s0 * s3);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, s1);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one + s1 * (s4 - one));
}

/// Scales both columns of the block in place.
///
/// `src[0]` multiplies column 0 and `src[1]` column 1; the second translation
/// component is pulled back towards one (`1 + m7*s1 - s1`).
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80254FA0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80254FA0..0x80255018` (120 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 6 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80254fa0(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_A_228);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m0 = read_f32(dst);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m2 = read_f32(dst + 0x8);
    let m3 = read_f32(dst + 0xc);
    let m5 = read_f32(dst + 0x14);
    let m6 = read_f32(dst + 0x18);
    let m7 = read_f32(dst + 0x1c);
    write_f32(dst + 0x10, m4 * s1);
    write_f32(dst, m0 * s0);
    write_f32(dst + 0x4, m1 * s0);
    write_f32(dst + 0x8, m2 * s0);
    write_f32(dst + 0xc, m3 * s0);
    write_f32(dst + 0x14, m5 * s1);
    write_f32(dst + 0x18, m6 * s1);
    write_f32(dst + 0x1c, one + m7 * s1 - s1);
}

/// Rotates the block in place about the `(0.5, 0.5)` pivot.
///
/// Both columns are rotated by the sine/cosine pair and the two translation
/// components are re-derived from the `0.5` pivot, so the image spins about the
/// centre of a unit square.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255020` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255020..0x80255150` (304 bytes); the
///   closing `bclr 20,0` is the last word.
/// - Table: slot 5 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255020(dst: usize, src: usize) {
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t7 = half * cos;
    let t9 = half - t7;
    let t10 = half * sin;
    write_f32(dst, cos * m0 + sin * m4);
    write_f32(dst + 0x10, -sin * m0 + cos * m4);
    write_f32(dst + 0x4, cos * m1 + sin * m5);
    write_f32(dst + 0x14, -sin * m1 + cos * m5);
    write_f32(dst + 0x8, cos * m2 + sin * m6);
    write_f32(dst + 0x18, -sin * m2 + cos * m6);
    write_f32(dst + 0xc, t9 + (cos * m3 + sin * m7) - t10);
    write_f32(dst + 0x1c, t10 + (t9 + (-sin * m3 + cos * m7)));
}

/// Adds the two source offsets to the translation components in place.
///
/// `dst[3] -= src[3]` and `dst[7] += src[4]`; the six linear entries are
/// untouched.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255150` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255150..0x80255178` (40 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 3 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255150(dst: usize, src: usize) {
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    write_f32(dst + 0xc, m3 + -s3);
    write_f32(dst + 0x1c, m7 + s4);
}

/// Rotates the block in place and re-derives both translation components.
///
/// Uses both source column scales and both pivot offsets: the linear part is
/// rotated by the sine/cosine pair and the two translation components become
/// `0.5*scale +/- (rotated pivot)`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255180` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255180..0x80255314` (404 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 4 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255180(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_A_228);
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let s1 = read_f32(src + 0x4);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * sin;
    let t2 = s1 * cos;
    let t4 = -t1;
    let t5 = m7 - half;
    let t6 = m3 - half;
    let t7 = s0 * cos;
    let t8 = s0 * sin;
    write_f32(dst + 0x1c, one + (t4 * t6 + t2 * t5 - half * s1));
    write_f32(dst + 0x10, t4 * m0 + t2 * m4);
    write_f32(dst, t7 * m0 + t8 * m4);
    write_f32(dst + 0x4, t7 * m1 + t8 * m5);
    write_f32(dst + 0x14, t4 * m1 + t2 * m5);
    write_f32(dst + 0x8, t7 * m2 + t8 * m6);
    write_f32(dst + 0x18, t4 * m2 + t2 * m6);
    write_f32(dst + 0xc, half * s0 + (t7 * t6 + t8 * t5));
}

/// Rotates the block in place and applies the two source offsets.
///
/// The linear part is rotated by the sine/cosine pair, then `src[3]`/`src[4]`
/// are folded into the two translation components together with the `0.5` pivot
/// terms.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255320` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255320..0x80255470` (336 bytes); the
///   closing `bclr 20,0` is the last word.
/// - Table: slot 1 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255320(dst: usize, src: usize) {
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let neg_half = read_f32(F32_NEG_HALF_A_228);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t8 = neg_half * cos;
    let t9 = half * sin;
    let t10 = t9 - half;
    write_f32(dst, cos * m0 + sin * m4);
    write_f32(dst + 0x10, -sin * m0 + cos * m4);
    write_f32(dst + 0x4, cos * m1 + sin * m5);
    write_f32(dst + 0x14, -sin * m1 + cos * m5);
    write_f32(dst + 0x8, cos * m2 + sin * m6);
    write_f32(dst + 0x18, -sin * m2 + cos * m6);
    write_f32(dst + 0xc, t8 + (cos * m3 + sin * m7) - t10 - s3);
    write_f32(dst + 0x1c, s4 + (t10 + (t8 + (-sin * m3 + cos * m7))));
}

/// Scales both columns of the block in place and applies the two source offsets.
///
/// `src[0]`/`src[1]` scale the two columns, `src[3]`/`src[4]` are the offsets;
/// the angle is not read.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255470` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255470..0x802554F8` (136 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 2 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255470(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_A_228);
    let m0 = read_f32(dst);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m2 = read_f32(dst + 0x8);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m6 = read_f32(dst + 0x18);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, m0 * s0);
    write_f32(dst + 0x4, m1 * s0);
    write_f32(dst + 0x8, m2 * s0);
    write_f32(dst + 0xc, s0 * (m3 - s3));
    write_f32(dst + 0x10, m4 * s1);
    write_f32(dst + 0x14, m5 * s1);
    write_f32(dst + 0x18, m6 * s1);
    write_f32(dst + 0x1c, one + s1 * (m7 + s4 - one));
}

/// Full in-place update: scales, rotates and translates the block.
///
/// Both source scales are applied to the two columns, the sine/cosine pair
/// rotates them, and the `(0.5, 0.5)` pivot plus the two source offsets rebuild
/// the translation components.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255500` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255500..0x802556A8` (424 bytes); the
///   closing `bclr 20,0` is the last word, then 584 padding bytes.
/// - Table: slot 0 of the seven-entry kernel table at `0x802F8F0C`
///   (table A1), reached from `fn_802556b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255500(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_A_228);
    let angle_step = read_f32(F32_ANGLE_STEP_A_228);
    let half = read_f32(F32_HALF_A_228);
    let s0 = read_f32(src);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let s3 = read_f32(src + 0xc);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * sin;
    let t2 = m7 - half;
    let t4 = -t1;
    let t6 = s1 * cos;
    let t7 = s0 * cos;
    let t11 = s0 * sin;
    write_f32(dst, t7 * m0 + t11 * m4);
    write_f32(dst + 0x10, t4 * m0 + t6 * m4);
    write_f32(dst + 0x1c, one + (t4 * (half + m3) + t6 * t2 - (half - s4) * s1));
    write_f32(dst + 0x4, t7 * m1 + t11 * m5);
    write_f32(dst + 0x8, t7 * m2 + t11 * m6);
    write_f32(dst + 0x14, t4 * m1 + t6 * m5);
    write_f32(dst + 0x18, t4 * m2 + t6 * m6);
    write_f32(dst + 0xc, (half - s3) * s0 + (t7 * (m3 - half) + t11 * t2));
}

/// Writes the `0x802F8F28` variant of the two-column scale block.
///
/// Same shape as [`fn_80254be0`] but from the `B` constant block; only
/// `dst[0]`, `dst[5]` and the second translation component are non-zero.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802558F0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x802558F0..0x80255928` (56 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 6 of the seven-entry kernel table at `0x802F8F28`
///   (table B0), reached from `fn_802562b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_802558f0(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_B_228);
    let one = read_f32(F32_ONE_B_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    write_f32(dst, s0);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, zero);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, s1);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one - s1);
}

/// Writes the `0x802F8F28` variant of the centred rotation.
///
/// The two columns are `(cos, -sin)` and `(sin, cos)` and the two translation
/// components are `sin` and `1 - cos`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255930` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255930..0x802559B8` (136 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 5 of the seven-entry kernel table at `0x802F8F28`
///   (table B0), reached from `fn_802562b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255930(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_B_228);
    let one = read_f32(F32_ONE_B_228);
    let angle_step = read_f32(F32_ANGLE_STEP_B_228);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    write_f32(dst, cos);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0x4, -sin);
    write_f32(dst + 0xc, sin);
    write_f32(dst + 0x10, sin);
    write_f32(dst + 0x14, cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one - cos);
}

/// Writes the `0x802F8F28` pure translation block.
///
/// Identity linear part, `-src[3]` and `src[4]` as the two translation
/// components.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802559C0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x802559C0..0x802559F8` (56 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 3 of the seven-entry kernel table at `0x802F8F28`
///   (table B0), reached from `fn_802562b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_802559c0(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_B_228);
    let one = read_f32(F32_ONE_B_228);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, one);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, -s3);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, one);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, s4);
}

/// Writes the rotated, scaled block from the `B` constant block.
///
/// Column scales `src[0]`/`src[1]`, rotation `src[2]`; the two translation
/// components are `-sin`-scaled and `1 - cos`-scaled rather than offset by a
/// pivot.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255A00` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255A00..0x80255AA0` (160 bytes); the
///   closing `bclr 20,0` is the last word, then 208 padding bytes.
/// - Table: slot 4 of the seven-entry kernel table at `0x802F8F28`
///   (table B0), reached from `fn_802562b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255a00(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_B_228);
    let one = read_f32(F32_ONE_B_228);
    let angle_step = read_f32(F32_ANGLE_STEP_B_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * cos;
    let t3 = s0 * sin;
    write_f32(dst, s0 * cos);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0x4, -t3);
    write_f32(dst + 0xc, s0 * sin);
    write_f32(dst + 0x10, s1 * sin);
    write_f32(dst + 0x14, s1 * cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one + -t1);
}

/// Writes the `0x802F8F28` pivot scale block.
///
/// `src[0]`/`src[1]` scale the two columns, `src[3]`/`src[4]` are the pivot
/// offsets; no angle term.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255B70` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255B70..0x80255BC0` (80 bytes); the
///   closing `bclr 20,0` is the last word, then 208 padding bytes.
/// - Table: slot 2 of the seven-entry kernel table at `0x802F8F28`
///   (table B0), reached from `fn_802562b0` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255b70(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_B_228);
    let one = read_f32(F32_ONE_B_228);
    let s0 = read_f32(src);
    let s3 = read_f32(src + 0xc);
    let s1 = read_f32(src + 0x4);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, s0);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, -s0 * s3);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, s1);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, one + s1 * (s4 - one));
}

/// Scales both columns of the block in place (`B` constants).
///
/// `src[0]` scales column 0, `src[1]` column 1, and the second translation
/// component is pulled back towards one.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255C90` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255C90..0x80255D08` (120 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 6 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255c90(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_B_228);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m0 = read_f32(dst);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m2 = read_f32(dst + 0x8);
    let m3 = read_f32(dst + 0xc);
    let m5 = read_f32(dst + 0x14);
    let m6 = read_f32(dst + 0x18);
    let m7 = read_f32(dst + 0x1c);
    write_f32(dst + 0x10, m4 * s1);
    write_f32(dst, m0 * s0);
    write_f32(dst + 0x4, m1 * s0);
    write_f32(dst + 0x8, m2 * s0);
    write_f32(dst + 0xc, m3 * s0);
    write_f32(dst + 0x14, m5 * s1);
    write_f32(dst + 0x18, m6 * s1);
    write_f32(dst + 0x1c, one + s1 * (m7 - one));
}

/// Rotates the block in place in the opposite sense.
///
/// The linear part is `(cos, +sin)` / `(-sin, cos)` about the `B` constant
/// block, and the two translation components are rebuilt from `sin`, `cos` and
/// the existing entries.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255D10` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255D10..0x80255E20` (272 bytes); the
///   closing `bclr 20,0` is the last word.
/// - Table: slot 5 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255d10(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_B_228);
    let angle_step = read_f32(F32_ANGLE_STEP_B_228);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    write_f32(dst, cos * m0 - sin * m4);
    write_f32(dst + 0x10, sin * m0 + cos * m4);
    write_f32(dst + 0x4, cos * m1 - sin * m5);
    write_f32(dst + 0x14, sin * m1 + cos * m5);
    write_f32(dst + 0x8, cos * m2 - sin * m6);
    write_f32(dst + 0x18, sin * m2 + cos * m6);
    write_f32(dst + 0xc, sin + (cos * m3 - sin * m7));
    write_f32(dst + 0x1c, one + (sin * m3 + cos * m7 - cos));
}

/// Adds the two source offsets to the translation components in place (`B` constants).
///
/// `dst[3] -= src[3]` and `dst[7] += src[4]`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255E20` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255E20..0x80255E44` (36 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 3 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255e20(dst: usize, src: usize) {
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    write_f32(dst + 0xc, m3 - s3);
    write_f32(dst + 0x1c, m7 + s4);
}

/// Rotates the block in place and folds in the two source offsets.
///
/// Both source column scales act on the linear part, the sine/cosine pair
/// rotates it, and the two translation components become `sin + cos*pivot`
/// forms built from `src[3]`/`src[4]`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255E50` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255E50..0x80255F8C` (316 bytes); the
///   closing `bclr 20,0` is the last word, then 4 padding bytes.
/// - Table: slot 4 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255e50(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_B_228);
    let angle_step = read_f32(F32_ANGLE_STEP_B_228);
    let s1 = read_f32(src + 0x4);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s0 = read_f32(src);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * cos;
    let t2 = s1 * sin;
    let t3 = s0 * cos;
    let t4 = s0 * sin;
    write_f32(dst + 0x1c, one + (t2 * m3 + t1 * m7 - t1));
    write_f32(dst, t3 * m0 - t4 * m4);
    write_f32(dst + 0xc, t4 + (t3 * m3 - t4 * m7));
    write_f32(dst + 0x10, t2 * m0 + t1 * m4);
    write_f32(dst + 0x4, t3 * m1 - t4 * m5);
    write_f32(dst + 0x14, t2 * m1 + t1 * m5);
    write_f32(dst + 0x8, t3 * m2 - t4 * m6);
    write_f32(dst + 0x18, t2 * m2 + t1 * m6);
}

/// Rotates the block in place and re-derives the translation from the source offsets.
///
/// Uses the `src[3]`/`src[4]`-shifted pivot: the linear part is rotated in the
/// `(cos, +sin)` / `(-sin, cos)` sense and the two translation components are
/// `sin`/`1 - cos` biased.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80255F90` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80255F90..0x802560BC` (300 bytes); the
///   closing `bclr 20,0` is the last word, then 4 padding bytes.
/// - Table: slot 1 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80255f90(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_B_228);
    let angle_step = read_f32(F32_ANGLE_STEP_B_228);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t7 = m3 - s3;
    let t13 = m7 + s4;
    write_f32(dst, cos * m0 - sin * m4);
    write_f32(dst + 0x10, sin * m0 + cos * m4);
    write_f32(dst + 0x4, cos * m1 - sin * m5);
    write_f32(dst + 0x14, sin * m1 + cos * m5);
    write_f32(dst + 0x8, cos * m2 - sin * m6);
    write_f32(dst + 0x18, sin * m2 + cos * m6);
    write_f32(dst + 0xc, sin + (cos * t7 - sin * t13));
    write_f32(dst + 0x1c, one + (sin * t7 + cos * t13 - cos));
}

/// Scales both columns of the block in place and applies the two source offsets (`B` constants).
///
/// `src[0]`/`src[1]` scale the two columns, `src[3]`/`src[4]` are the offsets.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802560C0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x802560C0..0x80256148` (136 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 2 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_802560c0(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_B_228);
    let m0 = read_f32(dst);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m2 = read_f32(dst + 0x8);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m6 = read_f32(dst + 0x18);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, m0 * s0);
    write_f32(dst + 0x4, m1 * s0);
    write_f32(dst + 0x8, m2 * s0);
    write_f32(dst + 0xc, s0 * (m3 - s3));
    write_f32(dst + 0x10, m4 * s1);
    write_f32(dst + 0x14, m5 * s1);
    write_f32(dst + 0x18, m6 * s1);
    write_f32(dst + 0x1c, one + s1 * (m7 + s4 - one));
}

/// Full in-place update for the `0x802F8F44` table.
///
/// Both column scales, the rotation and the two source offsets are folded in;
/// the two translation components are rebuilt from the
/// `src[3]`/`src[4]`-shifted pivot.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256150` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256150..0x802562AC` (348 bytes); the
///   closing `bclr 20,0` is the last word, then 612 padding bytes.
/// - Table: slot 0 of the seven-entry kernel table at `0x802F8F44`
///   (table B1), reached from `fn_802562b0` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256150(dst: usize, src: usize) {
    let one = read_f32(F32_ONE_B_228);
    let angle_step = read_f32(F32_ANGLE_STEP_B_228);
    let s0 = read_f32(src);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * cos;
    let t2 = s1 * sin;
    let t3 = s0 * cos;
    let t4 = m3 - s3;
    let t5 = s0 * sin;
    let t6 = m7 + s4;
    write_f32(dst + 0xc, t5 + (t3 * t4 - t5 * t6));
    write_f32(dst, t3 * m0 - t5 * m4);
    write_f32(dst + 0x1c, one + (t2 * t4 + t1 * t6 - t1));
    write_f32(dst + 0x10, t2 * m0 + t1 * m4);
    write_f32(dst + 0x4, t3 * m1 - t5 * m5);
    write_f32(dst + 0x14, t2 * m1 + t1 * m5);
    write_f32(dst + 0x8, t3 * m2 - t5 * m6);
    write_f32(dst + 0x18, t2 * m2 + t1 * m6);
}

/// Writes the `0x802F8F60` two-column scale block.
///
/// `dst[0]` is `src[0]`, `dst[5]` is `src[1]` and both translation components
/// become `0.5 - src[n]`. The original multiplies the shared `0.5` by the
/// shared `1.0` first.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256510` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256510..0x80256558` (72 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 6 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256510(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let one = read_f32(F32_ONE_C_228);
    let half = read_f32(F32_HALF_C_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    write_f32(dst, s0);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, half * (one - s0));
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, s1);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, half * (one - s1));
}

/// Writes the centred rotation with `-0.5`-biased translation.
///
/// Columns `(cos, sin)` / `(-sin, cos)`; the two translation components are
/// `-0.5*cos + sin - 1` and `0.5*sin + cos - 1`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256560` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256560..0x8025660C` (172 bytes); the
///   closing `bclr 20,0` is the last word, then 4 padding bytes.
/// - Table: slot 5 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256560(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let one = read_f32(F32_ONE_C_228);
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let neg_half = read_f32(F32_NEG_HALF_C_228);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    write_f32(dst, cos);
    write_f32(dst + 0x4, sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, neg_half * (cos + sin - one));
    write_f32(dst + 0x10, -sin);
    write_f32(dst + 0x14, cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, neg_half * (-sin + cos - one));
}

/// Writes the `0x802F8F60` pure translation block.
///
/// Identity linear part, `-src[3]` and `src[4]` translation.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256610` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256610..0x80256648` (56 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 3 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256610(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let one = read_f32(F32_ONE_C_228);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, one);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, -s3);
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, one);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, s4);
}

/// Writes the rotated, scaled block from the `C` constant block.
///
/// Column scales `src[0]`/`src[1]`, rotation `src[2]`, and the two translation
/// components are `-0.5*(scaled cos) + scaled sin - 1` forms.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256650` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256650..0x80256704` (180 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 4 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256650(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let one = read_f32(F32_ONE_C_228);
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let neg_half = read_f32(F32_NEG_HALF_C_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * sin;
    let t2 = s0 * sin;
    let t3 = s0 * cos;
    let t4 = s1 * cos;
    let t5 = -t1;
    write_f32(dst, s0 * cos);
    write_f32(dst + 0x4, s0 * sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0x10, -t1);
    write_f32(dst + 0xc, neg_half * (t3 + t2 - one));
    write_f32(dst + 0x14, s1 * cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, neg_half * (t5 + t4 - one));
}

/// Writes the rotation about the source pivot, biased by `0.5`.
///
/// Columns `(cos, sin)` / `(-sin, cos)`; the two translation components are
/// `0.5 +/- (rotated (src[3]+0.5, src[4]-0.5))`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256710` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256710..0x802567DC` (204 bytes); the
///   closing `bclr 20,0` is the last word, then 4 padding bytes.
/// - Table: slot 1 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256710(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let half = read_f32(F32_HALF_C_228);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = half + s3;
    let t3 = s4 - half;
    write_f32(dst, cos);
    write_f32(dst + 0x4, sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, half + (-cos * t1 + sin * t3));
    write_f32(dst + 0x10, -sin);
    write_f32(dst + 0x14, cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, half + (sin * t1 + cos * t3));
}

/// Writes the `C` pivot scale block.
///
/// `src[0]`/`src[1]` scale the two columns and `src[3]`/`src[4]` are the
/// `0.5`-biased pivot offsets.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802567E0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x802567E0..0x80256838` (88 bytes); the
///   closing `bclr 20,0` is the last word, then 8 padding bytes.
/// - Table: slot 2 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_802567e0(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let half = read_f32(F32_HALF_C_228);
    let s0 = read_f32(src);
    let s3 = read_f32(src + 0xc);
    let s1 = read_f32(src + 0x4);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, s0);
    write_f32(dst + 0x4, zero);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0xc, half + -s0 * (half + s3));
    write_f32(dst + 0x10, zero);
    write_f32(dst + 0x14, s1);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x1c, half + s1 * (s4 - half));
}

/// Writes the `C` full block: scale, rotation and `0.5` pivot.
///
/// Both scales act on the two columns, the sine/cosine pair rotates them and
/// the two translation components are rebuilt from a `0.5 + src[3]` / `src[4] -
/// 0.5` pivot.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256840` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256840..0x8025690C` (204 bytes); the
///   closing `bclr 20,0` is the last word, then 4 padding bytes.
/// - Table: slot 0 of the seven-entry kernel table at `0x802F8F60`
///   (table C0), reached from `fn_80256f90` with a non-zero `direct`
///   argument. The wrapper indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256840(dst: usize, src: usize) {
    let zero = read_f32(F32_ZERO_C_228);
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let half = read_f32(F32_HALF_C_228);
    let s0 = read_f32(src);
    let s1 = read_f32(src + 0x4);
    let s3 = read_f32(src + 0xc);
    let s4 = read_f32(src + 0x10);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s0 * cos;
    let t2 = s0 * sin;
    let t3 = s1 * cos;
    let t4 = s1 * sin;
    let t5 = half + s3;
    let t7 = s4 - half;
    write_f32(dst, s0 * cos);
    write_f32(dst + 0x4, s0 * sin);
    write_f32(dst + 0x8, zero);
    write_f32(dst + 0x14, s1 * cos);
    write_f32(dst + 0x18, zero);
    write_f32(dst + 0x10, -t4);
    write_f32(dst + 0x1c, half + (t4 * t5 + t3 * t7));
    write_f32(dst + 0xc, half + (-t1 * t5 + t2 * t7));
}

/// Scales both columns of the block in place and re-centres the translation (`C` constants).
///
/// `src[0]`/`src[1]` scale the two columns and both translation components are
/// pulled back towards `0.5`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256910` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256910..0x80256990` (128 bytes); the
///   closing `bclr 20,0` is the last word, then 304 padding bytes.
/// - Table: slot 6 of the seven-entry kernel table at `0x802F8F7C`
///   (table C1), reached from `fn_80256f90` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256910(dst: usize, src: usize) {
    let half = read_f32(F32_HALF_C_228);
    let m0 = read_f32(dst);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m2 = read_f32(dst + 0x8);
    let m3 = read_f32(dst + 0xc);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m6 = read_f32(dst + 0x18);
    let m7 = read_f32(dst + 0x1c);
    write_f32(dst, m0 * s0);
    write_f32(dst + 0x4, m1 * s0);
    write_f32(dst + 0x8, m2 * s0);
    write_f32(dst + 0xc, half + s0 * (m3 - half));
    write_f32(dst + 0x10, m4 * s1);
    write_f32(dst + 0x14, m5 * s1);
    write_f32(dst + 0x18, m6 * s1);
    write_f32(dst + 0x1c, half + s1 * (m7 - half));
}

/// Adds the two source offsets to the translation components in place (`C` constants).
///
/// `dst[3] -= src[3]` and `dst[7] += src[4]`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256AC0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256AC0..0x80256AE4` (36 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 3 of the seven-entry kernel table at `0x802F8F7C`
///   (table C1), reached from `fn_80256f90` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256ac0(dst: usize, src: usize) {
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    write_f32(dst + 0xc, m3 - s3);
    write_f32(dst + 0x1c, m7 + s4);
}

/// Rotates the block in place about the `(0.5, 0.5)` pivot (`C` constants).
///
/// The linear part is rotated by the sine/cosine pair and the two translation
/// components are rebuilt as `0.5 +/- (rotated (m3-0.5, m7-0.5))`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256AF0` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256AF0..0x80256C34` (324 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 4 of the seven-entry kernel table at `0x802F8F7C`
///   (table C1), reached from `fn_80256f90` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256af0(dst: usize, src: usize) {
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let half = read_f32(F32_HALF_C_228);
    let s0 = read_f32(src);
    let m3 = read_f32(dst + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s1 = read_f32(src + 0x4);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * sin;
    let t2 = s0 * sin;
    let t3 = s0 * cos;
    let t4 = -t1;
    let t5 = m3 - half;
    let t6 = m7 - half;
    let t7 = s1 * cos;
    write_f32(dst + 0xc, half + (t3 * t5 + t2 * t6));
    write_f32(dst + 0x1c, half + (t4 * t5 + t7 * t6));
    write_f32(dst, t3 * m0 + t2 * m4);
    write_f32(dst + 0x10, t4 * m0 + t7 * m4);
    write_f32(dst + 0x4, t3 * m1 + t2 * m5);
    write_f32(dst + 0x14, t4 * m1 + t7 * m5);
    write_f32(dst + 0x8, t3 * m2 + t2 * m6);
    write_f32(dst + 0x18, t4 * m2 + t7 * m6);
}

/// Rotates the block in place and applies the two source offsets (`C` constants).
///
/// The linear part is rotated in the `(cos, +sin)` / `(-sin, cos)` sense and
/// the two translation components become `0.5 +/-` forms of the
/// `src[3]`/`src[4]`-shifted pivot.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256C40` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256C40..0x80256D80` (320 bytes); the
///   closing `bclr 20,0` is the last word.
/// - Table: slot 1 of the seven-entry kernel table at `0x802F8F7C`
///   (table C1), reached from `fn_80256f90` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256c40(dst: usize, src: usize) {
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let half = read_f32(F32_HALF_C_228);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t8 = m3 - s3;
    let t15 = m7 + s4;
    let t17 = t8 - half;
    let t18 = t15 - half;
    write_f32(dst, cos * m0 + sin * m4);
    write_f32(dst + 0x10, -sin * m0 + cos * m4);
    write_f32(dst + 0x4, cos * m1 + sin * m5);
    write_f32(dst + 0x14, -sin * m1 + cos * m5);
    write_f32(dst + 0x8, cos * m2 + sin * m6);
    write_f32(dst + 0x18, -sin * m2 + cos * m6);
    write_f32(dst + 0xc, half + (cos * t17 + sin * t18));
    write_f32(dst + 0x1c, half + (-sin * t17 + cos * t18));
}

/// Scales both columns of the block in place and applies the two source offsets (`C` constants).
///
/// `src[0]`/`src[1]` scale the two columns and the two translation components
/// are rebuilt from the `C` `0.5` pivot and the `src[3]`/`src[4]` offsets.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256D80` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256D80..0x80256E14` (148 bytes); the
///   closing `bclr 20,0` is the last word, then 12 padding bytes.
/// - Table: slot 2 of the seven-entry kernel table at `0x802F8F7C`
///   (table C1), reached from `fn_80256f90` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256d80(dst: usize, src: usize) {
    let half = read_f32(F32_HALF_C_228);
    let m0 = read_f32(dst);
    let s0 = read_f32(src);
    let m1 = read_f32(dst + 0x4);
    let m2 = read_f32(dst + 0x8);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m4 = read_f32(dst + 0x10);
    let s1 = read_f32(src + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m6 = read_f32(dst + 0x18);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    write_f32(dst, m0 * s0);
    write_f32(dst + 0x4, m1 * s0);
    write_f32(dst + 0x8, m2 * s0);
    write_f32(dst + 0xc, half + s0 * (m3 - s3 - half));
    write_f32(dst + 0x10, m4 * s1);
    write_f32(dst + 0x14, m5 * s1);
    write_f32(dst + 0x18, m6 * s1);
    write_f32(dst + 0x1c, m7 * (half + s1 * (m7 + s4 - half)));
}

/// Full in-place update for the `0x802F8F7C` table.
///
/// Both column scales, the rotation and the two source offsets are folded in;
/// the two translation components are rebuilt from a `(m3-src[3], m7+src[4])`
/// pivot centred on `0.5`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80256E20` and
///   the export tree has no file for it; the body was read from
///   `work/dol/main.elf`.
/// - Extent: `0x80256E20..0x80256F84` (356 bytes); the
///   closing `bclr 20,0` is the last word.
/// - Table: slot 0 of the seven-entry kernel table at `0x802F8F7C`
///   (table C1), reached from `fn_80256f90` with `direct == 0`. The wrapper
///   indexes the table with `(format >> 1) & 7`.
///
/// # Arguments
/// * `dst` - eight-float two-column transform block.
///   The block is read and written in place.
/// * `src` - five floats: two column scales, the angle in degrees, two
///   translation/pivot offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Float stores into `dst` and one call to `0x80237CA0`.
///
/// # Safety
/// `dst` must be writable for 32 bytes and `src` readable for 20.
///
/// # Confidence
/// high - straight-line `lfs`/`stfs` single-precision arithmetic only.
pub unsafe fn fn_80256e20(dst: usize, src: usize) {
    let angle_step = read_f32(F32_ANGLE_STEP_C_228);
    let half = read_f32(F32_HALF_C_228);
    let s0 = read_f32(src);
    let m0 = read_f32(dst);
    let m4 = read_f32(dst + 0x10);
    let m3 = read_f32(dst + 0xc);
    let s3 = read_f32(src + 0xc);
    let m7 = read_f32(dst + 0x1c);
    let s4 = read_f32(src + 0x10);
    let s1 = read_f32(src + 0x4);
    let m1 = read_f32(dst + 0x4);
    let m5 = read_f32(dst + 0x14);
    let m2 = read_f32(dst + 0x8);
    let m6 = read_f32(dst + 0x18);
    let s2 = read_f32(src + 0x8);
    let angle = angle_step * s2;
    let (sin, cos) = host_sincos(angle as f64);
    let t1 = s1 * sin;
    let t2 = m3 - s3;
    let t3 = s0 * sin;
    let t4 = s0 * cos;
    let t5 = -t1;
    let t6 = t2 - half;
    let t7 = m7 + s4;
    let t8 = s1 * cos;
    let t10 = t7 - half;
    write_f32(dst, t4 * m0 + t3 * m4);
    write_f32(dst + 0xc, half + (t4 * t6 + t3 * t10));
    write_f32(dst + 0x1c, half + (t5 * t6 + t8 * t10));
    write_f32(dst + 0x10, t5 * m0 + t8 * m4);
    write_f32(dst + 0x4, t4 * m1 + t3 * m5);
    write_f32(dst + 0x14, t5 * m1 + t8 * m5);
    write_f32(dst + 0x8, t4 * m2 + t3 * m6);
    write_f32(dst + 0x18, t5 * m2 + t8 * m6);
}
