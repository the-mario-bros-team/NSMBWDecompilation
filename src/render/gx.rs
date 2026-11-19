//! GX immediate-mode quad emission.
//!
//! **What lives here.** The three quad helpers of the first translation lane:
//! [`fn_8000ebf0`] writes four corners (three `f32` position components each,
//! plus two optional texture bytes) straight into the GX write-gather FIFO;
//! [`fn_8000eeb0`] and [`fn_8000ef20`] copy two 2D corner points onto the stack,
//! run the quad fill, emit the corners and flush.
//!
//! **Original addresses.** `0x8000EBF0`, `0x8000EEB0`, `0x8000EF20`, all inside
//! the `0x80005000..0x80014000` lane (`src/subsystems/r80005_80014.rs` before
//! promotion).
//!
//! **Why it exists.** GX (the Wii GPU) is driven through a command FIFO at
//! `0xCC008000`; every `stfs`/`stb` to that address is a vertex attribute
//! consumed by the GPU, not ordinary memory. This module isolates that MMIO
//! stream so the rest of the tree can build quads without touching the FIFO.
//! The vertex grids and vector math that feed these calls live in
//! [`crate::render::geometry`].
//!
//! **Dependencies.** `crate::platform` for big-endian stores and indirect
//! calls. `0x801C5570` (inferred `GXBegin`), `0x801C47D0` and `0x801C9A80`
//! (state) are still opaque and called by address; the quad fill
//! `0x8000E910` is translated as [`crate::render::geometry::fn_8000e910`], and
//! `0x8000EDD0` (flush) is not translated yet; see
//! `docs/COVERAGE_r80005_80014.md`.
//!
//! **Start here.** [`fn_8000ebf0`] - it contains the corner tables and the
//! FIFO write loop.
//!
//! **Glossary.** GX = Wii GPU command interface; FIFO = the write-gather
//! command stream at `0xCC008000`. See `docs/GLOSSARY.md`.

use crate::platform;
use crate::platform::{call3, call6};

// ---------------------------------------------------------------------------
// Cross-lane callees (opaque here).
//
// Each constant is the entry address of a function owned by another layer.
// They are declared so a reader can see which external effects a translation
// has without leaving this file.
// ---------------------------------------------------------------------------

/// GX immediate helper used before the quad writes: `(0x80, 0, 4)`.
/// `0x80` is `GX_QUADS` and `4` the vertex count, i.e. this is a `GXBegin`
/// equivalent (`// inferred:` - the arguments and the following FIFO writes
/// match that role exactly).
///
/// Ghidra: 801c5570 FUN_801c5570.c
const FN_801C5570: usize = 0x801C_5570;

/// GX state call with six integers `(0, 0, 0, 0x1E, 0, 0x7D)`.
///
/// Ghidra: 801c47d0 FUN_801c47d0.c
const FN_801C47D0: usize = 0x801C_47D0;

/// GX call with three arguments `(param, 0x1E, 0)`.
///
/// Ghidra: 801c9a80 FUN_801c9a80.c
const FN_801C9A80: usize = 0x801C_9A80;

/// Flushes/advances the quad filled by [`FN_8000E910`] (takes no arguments).
///
/// Ghidra: 8000edd0 FUN_8000edd0.c (in this range, not yet translated)
const FN_8000EDD0: usize = 0x8000_EDD0;

/// Fills the quad/vertex scratch used by [`fn_8000ebf0`]:
/// `(translate, flag, uv_flag, &scale, &offset)`, where `translate` travels in
/// `f1` and the four words in `r3..r6`.
///
/// Ghidra: 8000e910 FUN_8000e910.c (translated as
/// [`crate::render::geometry::fn_8000e910`])
const FN_8000E910: usize = 0x8000_E910;

// ---------------------------------------------------------------------------
// Image constants read through r2 (small data area 2).
//
// Addresses are absolute; the original instruction is `lfs f, disp(r2)` with
// `r2 = 0x80433080`. Values were read out of `work/dol/main.elf`.
// ---------------------------------------------------------------------------

/// `r2-0x7F68` - one, used for quad corners.
const ONE_F32: usize = 0x8042_B118;
/// `r2-0x7F64` - zero, used for quad corners.
const ZERO_F32_B: usize = 0x8042_B11C;

/// GX write-gather FIFO. Every `stfs`/`stb` to this address is consumed by the
/// GPU command processor (`0xCC008000` is the standard Wii GX FIFO).
const GX_FIFO: usize = 0xCC00_8000;

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

/// Calls a PowerPC function `(f32, u32, u32, u32, u32) -> u32`.
///
/// The float travels in `f1` and does not consume an integer slot; the four
/// words follow in `r3..r6`. This is the shape of the `0x8000E910` call.
///
/// # Safety
/// `addr` must accept one float plus four integer arguments.
#[inline]
unsafe fn call_f32_u32x4(addr: usize, f: f32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    let function: extern "C" fn(f32, u32, u32, u32, u32) -> u32 = core::mem::transmute(addr);
    function(f, a, b, c, d)
}

/// Emits one textured quad (four corners) through the GX FIFO.
///
/// Each corner writes three `f32` position components to `0xCC008000`
/// (GX write-gather FIFO) and, when `uv` is non-zero, two more bytes. The
/// four corners are the unit square. `flip` selects the vertex order:
/// `0` starts at `(0,1)` and walks down the left edge; non-zero starts at
/// `(0,0)` and walks along the bottom edge.
///
/// `0x801C5570(0x80, 0, 4)` is the "begin quads, vertex format 0, 4
/// vertices" call (`// inferred:` GXBegin with `GX_QUADS`).
///
/// # Original
/// - Ghidra: `FUN_8000EBF0` at `0x8000EBF0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000ebf0_FUN_8000ebf0.c`
///
/// # Arguments
/// * `flip` - `0` uses the `(0,1)`-first order, non-zero the `(0,0)`-first
///   order (observed at `0x8000EC20`: `cmpwi r30,0; beq` picks the block at
///   `0x8000ECF0`).
/// * `uv` - when non-zero, two extra bytes per corner are emitted; when zero
///   those stores are skipped (the original branches on `r31`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// 12 floats + up to 8 bytes to the GX FIFO; one call to `0x801C5570`.
///
/// # Safety
/// Must run on the game thread with the GX FIFO writable (real hardware or a
/// matching emulator); this is `MMIO`, not ordinary memory.
///
/// # Confidence
/// medium - the corner table and stores were reconstructed from
/// `0x8000EC24..0x8000EDB4`; the meaning of the two trailing bytes is
/// `// unknown:`.
pub unsafe fn fn_8000ebf0(flip: u32, uv: u32) {
    // observed 0x8000EC14..0x8000EC18.
    call3(FN_801C5570, 0x80, 0, 4);

    let zero = read_f32(ZERO_F32_B);
    let one = read_f32(ONE_F32);

    // observed 0x8000EC20: `beq 0x8000ECF0` - the zero case is the second
    // block in memory, so the tables are written in that order here.
    let (corners, uvs): ([(f32, f32, f32); 4], [(u8, u8); 4]) = if flip == 0 {
        (
            [(zero, one, zero), (one, one, zero), (one, zero, zero), (zero, zero, zero)],
            [(0, 0), (1, 0), (1, 1), (0, 1)],
        )
    } else {
        (
            [(zero, zero, zero), (one, zero, zero), (one, one, zero), (zero, one, zero)],
            [(0, 0), (1, 0), (1, 1), (0, 1)],
        )
    };

    let mut i = 0;
    while i < 4 {
        let (x, y, z) = corners[i];
        write_f32(GX_FIFO, x);
        write_f32(GX_FIFO, y);
        write_f32(GX_FIFO, z);
        if uv != 0 {
            platform::write_u8(GX_FIFO, uvs[i].0);
            platform::write_u8(GX_FIFO, uvs[i].1);
        }
        i += 1;
    }
}

/// Emits a quad from two 2D corner points (no texture bytes).
///
/// # Original
/// - Ghidra: `FUN_8000EEB0` at `0x8000EEB0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000eeb0_FUN_8000eeb0.c`
/// - observed: the export prints four arguments, but `f1` is never written
///   before `0x8000E910`, so the wrapper forwards its own `f1` (`translate`).
///
/// # Arguments
/// * `translate` - `f1`, forwarded unchanged to [`FN_8000E910`] (the `z`
///   translation).
/// * `flip` - forwarded to [`FN_8000E910`] (corner-order selector).
/// * `uv` - forwarded to [`fn_8000ebf0`] (texture-byte selector).
/// * `bottom_left` - 2 floats.
/// * `top_left` - 2 floats.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Builds two 2-float temporaries, then calls `0x8000E910`, `0x8000EBF0`
/// and `0x8000EDD0`.
///
/// # Safety
/// Both points readable for 8 bytes; GX FIFO writable.
///
/// # Confidence
/// high - the body is two copies plus three calls; the assembly order matches.
pub unsafe fn fn_8000eeb0(
    translate: f32,
    flip: u32,
    uv: u32,
    bottom_left: usize,
    top_left: usize,
) {
    let mut a = [0u32; 2];
    let mut b = [0u32; 2];
    b[0] = platform::read_u32(top_left);
    b[1] = platform::read_u32(top_left + 4);
    a[0] = platform::read_u32(bottom_left);
    a[1] = platform::read_u32(bottom_left + 4);

    call_f32_u32x4(
        FN_8000E910,
        translate,
        flip,
        uv,
        a.as_mut_ptr() as u32,
        b.as_mut_ptr() as u32,
    );
    fn_8000ebf0(flip, uv);
    platform::call0(FN_8000EDD0);
}

/// Emits a quad from two 2D points, forcing `uv = 1` and two extra GX state
/// calls before the corners.
///
/// # Original
/// - Ghidra: `FUN_8000EF20` at `0x8000EF20` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8000ef20_FUN_8000ef20.c`
///
/// # Arguments
/// * `translate` - `f1`, forwarded unchanged to `0x8000E910`.
/// * `flip` - forwarded to the quad builder.
/// * `param_2` - forwarded to `0x801C9A80` as its first argument.
/// * `bottom_left` - 2 floats.
/// * `top_left` - 2 floats.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8000E910(translate, flip, 1, ...)`, `0x801C47D0(0,0,0,0x1E,0,0x7D)`,
/// `0x801C9A80(param_2,0x1E,0)`, then `0x8000EBF0(flip,1)` and
/// `0x8000EDD0`.
///
/// # Safety
/// Both points readable for 8 bytes; GX FIFO writable.
///
/// # Confidence
/// high - the argument words are loaded literally
/// (`0x1E`=30, `0x7D`=125) and the call order is visible in
/// `0x8000EF34..0x8000EFA4`; `f1` is never written, so it is forwarded.
pub unsafe fn fn_8000ef20(
    translate: f32,
    flip: u32,
    param_2: u32,
    bottom_left: usize,
    top_left: usize,
) {
    let mut a = [0u32; 2];
    let mut b = [0u32; 2];
    b[0] = platform::read_u32(top_left);
    b[1] = platform::read_u32(top_left + 4);
    a[0] = platform::read_u32(bottom_left);
    a[1] = platform::read_u32(bottom_left + 4);

    // observed 0x8000EF34: r4 = 1, so the quad builder is told "uv present".
    call_f32_u32x4(
        FN_8000E910,
        translate,
        flip,
        1,
        a.as_mut_ptr() as u32,
        b.as_mut_ptr() as u32,
    );
    call6(FN_801C47D0, 0, 0, 0, 0x1E, 0, 0x7D);
    call3(FN_801C9A80, param_2, 0x1E, 0);
    fn_8000ebf0(flip, 1);
    platform::call0(FN_8000EDD0);
}
