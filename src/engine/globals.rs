//! Engine-global state and small-data helpers.
//!
//! **What this module is.** The recovered global-state layer of main-`DOL`:
//! small-data accessors and cleanup helpers for the four object slots at
//! `0x803741D0`, the 42-column bit-mask table at `0x803741F0`, the seven-byte
//! effect reset, and the packed-identifier builder at `0x80102A60`.
//!
//! **Original range.** `0x80100270..0x801092F0` (47 functions, 5,024 bytes).
//! The first pass translated twenty-two entries, thirteen promoted from
//! `src/subsystems/r80100_80140.rs` clusters A-C and nine previously
//! untranslated entries from the `0x801015A0..0x80102A80` window:
//!
//! * `0x801015A0`, `0x801015E0`, `0x80101690`, `0x801016B0` - record accessors;
//! * `0x801017D0`, `0x801018C0` - the `0x80373D60` display/mode state resets;
//! * `0x80102260` - the mode dispatcher;
//! * `0x801027C0` - the challenge/flag scan;
//! * `0x80102A80` - the 10x4 course-record loader.
//!
//! This pass adds twenty-five previously untranslated engine-global entries:
//!
//! * `0x80100270..0x80100720` - course/scene record lookups using the `0xBF8`
//!   record stride and the `0x16518` course stride;
//! * `0x80100830..0x80100940` - render-state publication and the
//!   `r13 - 0x54E0` linked-list scans;
//! * `0x80100F70..0x80101090` - mode predicate, per-slot effect pass and mask
//!   query;
//! * `0x801017B0` - 16-bit id to pointer lookup;
//! * `0x801072F0..0x80107410` - main-object creation and the `0x80374920` boot
//!   barrier;
//! * `0x80107EC0`, `0x80108F10`, `0x80109260`, `0x801092F0` - layout/display
//!   object and display-manager lifetimes.
//!
//! The WorldMap effect-slot table (`0x80102F90..0x80103ED0`, cluster D) and the
//! r2/float-ABI helpers (cluster E) remain staged there until
//! `src/game/worldmap.rs` is available.
//!
//! **Why it exists.** These functions own the small-data base `r13` and the
//! engine-wide object/bit state that other subsystems read. They form the
//! stable base that later gameplay code can call.
//!
//! **Dependencies.** Only [`crate::platform`] (`read_u8`, `read_u16`,
//! `read_i32`, `read_u32`, `read_f32`, `write_u8`, `write_u32`,
//! `write_f32`, `call0..call4`) plus the local six-argument [`call6`] and
//! float-first [`call_f32_3`] helpers. No other lane's module is required.
//!
//! **Start here.** [`fn_80102220`] is the top-level teardown path for the four
//! object slots; [`fn_80102750`] is the bit-table query; [`fn_80102a60`] is the
//! pure packed-identifier builder; [`fn_80102260`] is the mode dispatcher that
//! ties the state resets and scene transitions together.
//!
//! **Evidence.** `work/ghidra/main-gb/export/functions.tsv` and the
//! per-function C files cited in every `# Original` block below.
//!
//! **Small-data base.** Ghidra prints `unaff_r13` for main-`DOL` globals.
//! `0x800041C0` contains `addis r13,0,0x8042` / `ori r13,r13,0xF680` (bytes at
//! ELF offset 0x444), so `r13 = 0x8042F680`. [`SDA_BASE`] is shared with the
//! staged WorldMap/float helpers in `r80100_80140`. The second small-data base
//! is `r2 = 0x80433080` ([`SDA2_BASE`]); the float state in [`fn_80100830`] is
//! addressed relative to it.
//!
//! **Status.** All forty-seven functions are `translated`. Roles marked
//! `// inferred:` or `// unknown:` are not proven; see
//! `docs/COVERAGE_r80100_80140.md`, `docs/COVERAGE_WAVE7_ENGINE.md` and
//! `docs/COVERAGE_WAVE7_ENGINE2.md`.

use crate::platform;

// ---------------------------------------------------------------------------
// Shared small-data addresses
// ---------------------------------------------------------------------------

/// Small-data base register r13 for main-`DOL` code.
///
/// Observed: `0x800041C0` contains `3D A0 80 42 61 AD F6 80` =
/// `addis r13,0,0x8042` / `ori r13,r13,0xF680` / `blr`.
///
/// The remaining staged helpers in `src/subsystems/r80100_80140.rs` import
/// this constant instead of defining a second copy.
pub(crate) const SDA_BASE: usize = 0x8042_F680;

/// `r13 - 0x54BC` (absolute `0x8042A1C4`): a table of 32-bit words.
/// Role unknown; [`fn_80101950`] is its only accessor in this pass.
const SDA_54BC_WORDS: usize = SDA_BASE - 0x54BC;

/// `r13 - 0x54B8` (absolute `0x8042A1C8`): pointer to an object with at least
/// 0x11D8 bytes. The field at `+0x11D4` is read by [`fn_80101960`].
const SDA_54B8_OBJECT: usize = SDA_BASE - 0x54B8;

/// `r13 - 0x5458` (absolute `0x8042A228`): slot that receives a pointer chosen
/// from the table at [`PTR_TABLE_802F4A18`].
const SDA_5458_CURRENT: usize = SDA_BASE - 0x5458;

/// `r13 - 0x5454` (absolute `0x8042A22C`): lazily-created pool pointer.
const SDA_5454_POOL: usize = SDA_BASE - 0x5454;

/// `r13 - 0x545A` (absolute `0x8042A226`): byte flag set after the four object
/// slots have been destroyed.
const SDA_545A_FLAG: usize = SDA_BASE - 0x545A;

/// `r13 - 0x5428` (absolute `0x8042A258`): byte cleared by the reset helper.
const SDA_5428_BYTE: usize = SDA_BASE - 0x5428;

/// `r13 - 0x5426` / `r13 - 0x5425` (absolute `0x8042A25A` / `0x8042A25B`).
const SDA_5426_BYTE: usize = SDA_BASE - 0x5426;
const SDA_5425_BYTE: usize = SDA_BASE - 0x5425;

/// `r13 - 0x5424` / `r13 - 0x5423` (absolute `0x8042A25C` / `0x8042A25D`).
const SDA_5424_BYTE: usize = SDA_BASE - 0x5424;
const SDA_5423_BYTE: usize = SDA_BASE - 0x5423;

/// `r13 - 0x663B` (absolute `0x80429045`) and `r13 - 0x663A`
/// (absolute `0x80429046`): two bytes reset to 0xFF / 0x00.
const SDA_663B_BYTE: usize = SDA_BASE - 0x663B;
const SDA_663A_BYTE: usize = SDA_BASE - 0x663A;

/// `0x803741B0`: an array of 32-bit values indexed by [`fn_80101fd0`].
/// Exact element count unknown; no bounds check appears in the callee.
const ARRAY_803741B0: usize = 0x8037_41B0;

/// `0x803741D0`: four 32-bit object pointers. The cleanup helpers in this
/// module operate on exactly four entries at `+0, +4, +8, +0xC`.
const OBJECT_SLOTS_803741D0: usize = 0x8037_41D0;
const OBJECT_SLOT_COUNT: i32 = 4;

/// `0x803741F0`: a table of 42 (`0x2A`) 32-bit bit-masks per row; one row is
/// `0xA8` bytes. [`fn_80102750`] tests one bit-mask in this table.
const BITS_803741F0: usize = 0x8037_41F0;
const BITS_ROW_BYTES: usize = 0xA8;
const BITS_ROW_COUNT: u32 = 0x2A;

/// `0x802F4A18`: array of 32-bit code/data pointers selected by index.
/// Role unknown; [`fn_80101bc0`] stores the selected pointer into SDA.
const PTR_TABLE_802F4A18: usize = 0x802F_4A18;

/// `0x802F4A24`: data object passed as the fourth argument of the six-argument
/// allocator/factory call in [`fn_801021d0`]. Size/layout unknown.
const DATA_802F4A24: usize = 0x802F_4A24;

/// `0x8016E7F0`, `0x8008F310`, `0x800DDC50`: callees outside this lane.
/// Names and full behaviour are unknown; each is called through
/// [`crate::platform`] or [`call6`] with the argument count shown by Ghidra.
const FUN_8008F310: usize = 0x8008_F310;
const FUN_800DDC50: usize = 0x800D_DC50;
const FUN_8016E7F0: usize = 0x8016_E7F0;

// ---------------------------------------------------------------------------
// Addresses added by the mode/state pass (0x801015A0..0x80102A80)
// ---------------------------------------------------------------------------

/// `0x80373D60`: byte/word state block rewritten by [`fn_801017d0`] and
/// [`fn_801018c0`], then consumed by the layout/display code.
///
/// The block is addressed through these individual field constants because the
/// original mixes byte and word stores at unaligned-looking offsets. The field
/// meanings are `// unknown:`; the names describe only the observed writes.
const STATE_80373D60: usize = 0x8037_3D60;
const STATE_65: usize = STATE_80373D60 + 0x05;
const STATE_66: usize = STATE_80373D60 + 0x06;
const STATE_67: usize = STATE_80373D60 + 0x07;
const STATE_68: usize = STATE_80373D60 + 0x08;
const STATE_6C: usize = STATE_80373D60 + 0x0C;
const STATE_6D: usize = STATE_80373D60 + 0x0D;
const STATE_6E: usize = STATE_80373D60 + 0x0E;
const STATE_6F: usize = STATE_80373D60 + 0x0F;

/// `0x803741BC`: byte array indexed by the word at [`SDA_5488_INDEX`].
/// `// unknown:` the array length; the original performs no check.
const ARRAY_803741BC: usize = 0x8037_41BC;

/// `0x802F49D0`: table of two-byte rows selected by [`ARRAY_803741BC`].
const TABLE_802F49D0: usize = 0x802F_49D0;

/// `0x802F49F0`: table of 32-bit course/effect values indexed by a byte.
const TABLE_802F49F0: usize = 0x802F_49F0;

/// `0x8031D8B4`: table of record pointers used by [`fn_801015a0`].
const TABLE_8031D8B4: usize = 0x8031_D8B4;

/// `0x80315B90`: mode/state object; its word at `+8` is compared with `4`.
const OBJECT_80315B90: usize = 0x8031_5B90;

/// `0x80354E60` / `0x80354E90`: four-entry index table and the argument table
/// it selects from, used by [`fn_80102260`] before the mode change.
const TABLE_80354E60: usize = 0x8035_4E60;
const TABLE_80354E90: usize = 0x8035_4E90;

/// `r13 - 0x5488` (absolute `0x8042A1F8`): word index into
/// [`ARRAY_803741BC`].
const SDA_5488_INDEX: usize = SDA_BASE - 0x5488;

/// `r13 - 0x547C` (absolute `0x8042A204`): current engine mode word.
const SDA_547C_MODE: usize = SDA_BASE - 0x547C;

/// `r13 - 0x54D0` (absolute `0x8042A1B0`): byte set to `1` by both resets.
const SDA_54D0_BYTE: usize = SDA_BASE - 0x54D0;

/// `r13 - 0x6674` (absolute `0x8042900C`): mode word written by the resets and
/// by [`fn_80102260`].
const SDA_6674_MODE: usize = SDA_BASE - 0x6674;

/// `r13 - 0x5700` (absolute `0x80429F80`): bit flags tested by
/// [`fn_80102260`] (`0x10`, `0x20`).
const SDA_5700_FLAGS: usize = SDA_BASE - 0x5700;

/// `r13 - 0x5704` (absolute `0x80429F7C`): pointer to the current course/
/// challenge object.
const SDA_5704_OBJECT: usize = SDA_BASE - 0x5704;

/// `r13 - 0x56D0` (absolute `0x80429FB0`): object updated on the flag-path of
/// [`fn_80102260`].
const SDA_56D0_OBJECT: usize = SDA_BASE - 0x56D0;

/// `r13 - 0x5432` (absolute `0x8042A24E`): byte forwarded to `0x80102690`.
const SDA_5432_BYTE: usize = SDA_BASE - 0x5432;

/// `r13 - 0x5640` (absolute `0x8042A040`): pointer passed to `0x800E0380`.
const SDA_5640_OBJECT: usize = SDA_BASE - 0x5640;

/// `r13 - 0x54E3` / `r13 - 0x54E4` (absolute `0x8042A19D` / `0x8042A19C`):
/// one-shot "flag changed" bytes set by [`fn_801027c0`].
const SDA_54E3_BYTE: usize = SDA_BASE - 0x54E3;
const SDA_54E4_BYTE: usize = SDA_BASE - 0x54E4;

/// Callees outside this lane, called by absolute address exactly as the
/// evidence shows. None of them is reconstructed here.
const FUN_8005FFB0: usize = 0x8005_FFB0;
const FUN_800B0DB0: usize = 0x800B_0DB0;
const FUN_800B5040: usize = 0x800B_5040;
const FUN_800BBB70: usize = 0x800B_BB70;
const FUN_800BBC50: usize = 0x800B_BC50;
const FUN_800BBC60: usize = 0x800B_BC60;
const FUN_800CE490: usize = 0x800C_E490;
const FUN_800CE4D0: usize = 0x800C_E4D0;
const FUN_800CE510: usize = 0x800C_E510;
const FUN_800CE540: usize = 0x800C_E540;
const FUN_800CE560: usize = 0x800C_E560;
const FUN_800CE940: usize = 0x800C_E940;
const FUN_800CEB80: usize = 0x800C_EB80;
const FUN_800E0380: usize = 0x800E_0380;
const FUN_800E1E60: usize = 0x800E_1E60;
const FUN_800FAE40: usize = 0x800F_AE40;
const FUN_800FB1F0: usize = 0x800F_B1F0;
const FUN_800FB240: usize = 0x800F_B240;
const FUN_800FC400: usize = 0x800F_C400;
const FUN_800FC970: usize = 0x800F_C970;
const FUN_800FCDB0: usize = 0x800F_CDB0;
const FUN_800FD3B0: usize = 0x800F_D3B0;
const FUN_80102690: usize = 0x8010_2690;

// ---------------------------------------------------------------------------
// Addresses added by the engine-global block (0x80100270..0x801092F0)
// ---------------------------------------------------------------------------

/// Second small-data base register `r2` (`0x80433080`).
///
/// Observed: `0x800041C0` sets `r2 = 0x80433080`; the float constants below
/// are addressed as `r2 - k`. The arithmetic is `/f` (single precision).
const SDA2_BASE: usize = 0x8043_3080;

/// `r13 - 0x6838` / `r13 - 0x6830` / `r13 - 0x6828` (absolute `0x80428E48`,
/// `0x80428E50`, `0x80428E58`): three small static buffers handed to the
/// formatter at `0x802E16E8` by [`fn_80100720`].
const SDA_6838_BUFFER: usize = SDA_BASE - 0x6838;
const SDA_6830_BUFFER: usize = SDA_BASE - 0x6830;
const SDA_6828_BUFFER: usize = SDA_BASE - 0x6828;

/// `r13 - 0x5433` (absolute `0x8042A24D`): byte compared with `2` when
/// [`fn_80100720`] is called with mode `1`.
const SDA_5433_BYTE: usize = SDA_BASE - 0x5433;

/// `r13 - 0x54E0` (absolute `0x8042A1A0`): head of a linked list. Each node
/// stores the next pointer at `+0x184`; [`fn_80100900`] and [`fn_80100940`]
/// test the bytes at `+0x124` and `+0x18C`.
const SDA_54E0_LIST: usize = SDA_BASE - 0x54E0;

/// `r13 - 0x54F8` (absolute `0x8042A188`): word copied from the `r2` constant
/// pool by [`fn_80100830`].
const SDA_54F8_WORD: usize = SDA_BASE - 0x54F8;

/// `r13 - 0x54F0` (absolute `0x8042A190`): one-shot byte that guards the
/// `0x8031C9B0` float reset in [`fn_80100830`].
const SDA_54F0_BYTE: usize = SDA_BASE - 0x54F0;

/// `r13 - 0x5658` (absolute `0x8042A028`): four object pointers at `+4`,
/// `+8`, `+0xC`, `+0x10`, consumed by [`fn_80100f90`].
const SDA_5658_ARRAY: usize = SDA_BASE - 0x5658;

/// `r13 - 0x5200` (absolute `0x8042A480`): pointer to an object whose first
/// word points at a header; [`fn_80108f10`] reads the header's `+4`/`+6`
/// halfwords.
const SDA_5200_OBJECT: usize = SDA_BASE - 0x5200;

/// `r13 - 0x53F8` (absolute `0x8042A288`): word passed as the third argument
/// of [`fn_80107410`] by [`fn_801073b0`].
const SDA_53F8_WORD: usize = SDA_BASE - 0x53F8;

/// `r13 - 0x53E8` (absolute `0x8042A298`): slot that receives the main game
/// object allocated by [`fn_801073b0`].
const SDA_53E8_OBJECT: usize = SDA_BASE - 0x53E8;

/// `r13 - 0x53E4` (absolute `0x8042A29C`): boot-completion byte written by
/// [`fn_801072f0`] and polled by [`fn_80107340`].
const SDA_53E4_BYTE: usize = SDA_BASE - 0x53E4;

/// `r13 - 0x53E0` (absolute `0x8042A2A0`): word returned by
/// [`fn_80107340`] once the boot barrier opens.
const SDA_53E0_WORD: usize = SDA_BASE - 0x53E0;

/// `r13 - 0x53D4` (absolute `0x8042A2AC`): slot for the 0x40-byte layout
/// object created by [`fn_80107ec0`].
const SDA_53D4_OBJECT: usize = SDA_BASE - 0x53D4;

/// `r13 - 0x53D0` (absolute `0x8042A2B0`): lazily-created display record
/// owned by [`fn_80109260`].
const SDA_53D0_OBJECT: usize = SDA_BASE - 0x53D0;

/// `r13 - 0x53CC` (absolute `0x8042A2B4`): slot for the 0x1F8-byte display
/// manager created by [`fn_801092f0`].
const SDA_53CC_OBJECT: usize = SDA_BASE - 0x53CC;

/// `0x8031C8B0`: render-state block. `+0xC` receives a word and `+0x18`,
/// `+0x1C`, `+0x20` receive three floats from the `r2` pool.
const STATE_8031C8B0: usize = 0x8031_C8B0;

/// `0x8031C9B0`: second render-state block. `+0x20`, `+0x24`, `+0x28`
/// receive one float when [`SDA_54F0_BYTE`] is still zero.
const STATE_8031C9B0: usize = 0x8031_C9B0;

/// `0x80373D30`: data argument of the `0x802DC780` push in [`fn_80100830`].
const DATA_80373D30: usize = 0x8037_3D30;

/// `0x8031D39C`: base of the eight-byte parameter table selected by the
/// second argument of [`fn_80100f90`]. Each row holds an `/f` at `+0x26C`, a
/// signed halfword at `+0x270` and a byte at `+0x272`.
const TABLE_8031D39C: usize = 0x8031_D39C;

/// `0x80377C88`: table of 32-bit entries indexed by the object word at `+4`;
/// the selected entry is the first integer argument of `0x802BC9A0`.
const TABLE_80377C88: usize = 0x8037_7C88;

/// `0x80354E80`: mask table indexed by the word at [`TABLE_80354E60`];
/// [`fn_80101090`] returns its low bit.
const MASK_TABLE_80354E80: usize = 0x8035_4E80;

/// `0x80320840`: 16-bit-indexed table of 32-bit values returned by
/// [`fn_801017b0`].
const TABLE_80320840: usize = 0x8032_0840;

/// `0x80374920`: object passed to the boot helper `0x801CF580`.
const DATA_80374920: usize = 0x8037_4920;

/// `0x80321C20`: `"New SUPER MARIO BROS. Wii"` passed to
/// [`fn_80107410`].
const STR_80321C20: usize = 0x8032_1C20;

// Callees outside this lane, called by absolute address exactly as the
// evidence shows. None of them is reconstructed here.
const FUN_800DC990: usize = 0x800D_C990;
const FUN_800F0EC0: usize = 0x800F_0EC0;
const FUN_800F53A0: usize = 0x800F_53A0;
const FUN_800F56B0: usize = 0x800F_56B0;
const FUN_801006E0: usize = 0x8010_06E0;
const FUN_801008E0: usize = 0x8010_08E0;
const FUN_801072E0: usize = 0x8010_72E0;
const FUN_80107360: usize = 0x8010_7360;
const FUN_80107410: usize = 0x8010_7410;
const FUN_80107890: usize = 0x8010_7890;
const FUN_80107920: usize = 0x8010_7920;
const FUN_80107F20: usize = 0x8010_7F20;
const FUN_80108F40: usize = 0x8010_8F40;
const FUN_80109350: usize = 0x8010_9350;
const FUN_80168400: usize = 0x8016_8400;
const FUN_801CF580: usize = 0x801C_F580;
const FUN_8023B2A0: usize = 0x8023_B2A0;
const FUN_8023B360: usize = 0x8023_B360;
const FUN_802BC9A0: usize = 0x802B_C9A0;
const FUN_802B9240: usize = 0x802B_9240;
const FUN_802D3700: usize = 0x802D_3700;
const FUN_802DC69C: usize = 0x802D_C69C;
const FUN_802DC780: usize = 0x802D_C780;
const FUN_802E16E8: usize = 0x802E_16E8;
const FUN_802E1BD0: usize = 0x802E_1BD0;

// ---------------------------------------------------------------------------
// Local call helper
// ---------------------------------------------------------------------------

/// Calls `addr` with six 32-bit integer arguments (`r3..r8`).
///
/// `platform::call0..call3` stop at three arguments and `platform.rs` is outside
/// this lane, so the six-argument form used by `0x8016E7F0` lives here.
///
/// # Safety
/// `addr` must be a PowerPC function with the Metrowerks EABI signature
/// `u32(u32, u32, u32, u32, u32, u32)`.
unsafe fn call6(
    addr: usize,
    a1: u32,
    a2: u32,
    a3: u32,
    a4: u32,
    a5: u32,
    a6: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32) -> u32);
    f(a1, a2, a3, a4, a5, a6)
}

/// Calls `addr` with no arguments and keeps the `r3` result.
///
/// `platform::call0` discards the result, but `0x8005FFB0` and `0x801ABE90` /
/// `0x801ABE60` return values that the callers consume.
///
/// # Safety
/// `addr` must point at a PowerPC function returning `u32`.
unsafe fn call0_ret(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Calls `addr` with one single-precision float and three integer arguments.
///
/// The PowerPC EABI places `value` in `f1` and `a`/`b`/`c` in `r3`/`r4`/`r5`.
/// This helper is used only by [`fn_80100f90`] for `0x802BC9A0`, whose Ghidra
/// body drops the float and the last two integers.
///
/// # Safety
/// `addr` must point at a PowerPC function with the EABI signature
/// `void(f32, u32, i32, u8)`.
unsafe fn call_f32_3(addr: usize, value: f32, a: u32, b: i32, c: u8) {
    let f = crate::platform::guest_fn!(addr, fn(f32, u32, i32, u8));
    f(value, a, b, c)
}

// ---------------------------------------------------------------------------
// Cluster A: small-data and four-slot global state
// ---------------------------------------------------------------------------

/// Returns one 32-bit word from the small-data table at `r13 - 0x54BC`.
///
/// `// inferred:` a plain table getter. No caller was inspected, so the table's
/// element type and meaning are unknown.
///
/// # Original
/// - Ghidra: `FUN_80101950` at `0x80101950` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101950_FUN_80101950.c`
///
/// # Arguments
/// - `index`: signed element index. Multiplied by 4 and added to
///   `0x8042A1C4`; no bounds check is performed.
///
/// # Returns
/// The big-endian 32-bit word at `0x8042A1C4 + index * 4`.
///
/// # Side effects
/// None; this is a pure read from game memory.
///
/// # Safety
/// The computed address must be mapped and readable. The caller must keep
/// `index` inside the table; the original does not validate it.
///
/// # Confidence
/// high - the Ghidra body is a single indexed load with no ambiguity.
pub unsafe fn fn_80101950(index: i32) -> u32 {
    let addr = (SDA_54BC_WORDS as isize + (index as isize) * 4) as usize;
    platform::read_u32(addr)
}

/// Returns the 32-bit field at `object + 0x11D4`, or zero when the object
/// pointer is null.
///
/// The null check is on the SDA global, not on the field: a present object with
/// a zero field also returns zero. Both cases are intentionally collapsed in
/// the Ghidra body.
///
/// # Original
/// - Ghidra: `FUN_80101960` at `0x80101960` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101960_FUN_80101960.c`
///
/// # Arguments
/// None; the object pointer comes from `0x8042A1C8` (`r13 - 0x54B8`).
///
/// # Returns
/// `0` when the global pointer is null, otherwise the value at `+0x11D4`.
///
/// # Side effects
/// None; reads `0x8042A1C8` and the pointed-to object.
///
/// # Safety
/// If the global is non-null it must point to at least `0x11D8` readable bytes.
///
/// # Confidence
/// high - direct transcription; the role of the object is unknown.
pub unsafe fn fn_80101960() -> i32 {
    let owner = platform::read_u32(SDA_54B8_OBJECT);
    if owner == 0 {
        return 0;
    }
    // observed: field offset 0x11D4; its role is unknown.
    let value = platform::read_u32(owner as usize + 0x11D4) as i32;
    if value != 0 {
        return value;
    }
    0
}

/// Stores the pointer at `0x802F4A18[index]` into the small-data slot at
/// `r13 - 0x5458`.
///
/// `// inferred:` selects the current handler/object by index. The table holds
/// 32-bit addresses (the original is a 32-bit PowerPC).
///
/// # Original
/// - Ghidra: `FUN_80101BC0` at `0x80101BC0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101bc0_FUN_80101bc0.c`
///
/// # Arguments
/// - `index`: signed index into `0x802F4A18`; no bounds check.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the selected 32-bit pointer to `0x8042A228`.
///
/// # Safety
/// `0x802F4A18 + index * 4` must be readable and the loaded value must be a
/// valid pointer for later consumers.
///
/// # Confidence
/// high for the store; medium for the "select current object" role.
pub unsafe fn fn_80101bc0(index: i32) {
    let table_addr = (PTR_TABLE_802F4A18 as isize + (index as isize) * 4) as usize;
    let selected = platform::read_u32(table_addr);
    platform::write_u32(SDA_5458_CURRENT, selected);
}

/// Stores `value` into the 32-bit array element at `0x803741B0 + index * 4`.
///
/// `// inferred:` a setter for an array whose length is not encoded in the
/// callee. No bounds check appears.
///
/// # Original
/// - Ghidra: `FUN_80101FD0` at `0x80101FD0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101fd0_FUN_80101fd0.c`
///
/// # Arguments
/// - `index`: signed element index, scaled by 4.
/// - `value`: 32-bit value to store.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one 32-bit word into the `0x803741B0` array.
///
/// # Safety
/// The destination address must be writable and inside the array; the original
/// performs no range check.
///
/// # Confidence
/// high - single indexed store.
pub unsafe fn fn_80101fd0(index: i32, value: u32) {
    let addr = (ARRAY_803741B0 as isize + (index as isize) * 4) as usize;
    platform::write_u32(addr, value);
}

/// `0x8016E4F0(heap)` - heap-swap thunk to `0x802B9170`.
const W289_FN_8016E4F0: usize = 0x8016_E4F0;

/// `0x802B9210(size)` - size-class allocator on the current heap.
const W289_FN_802B9210: usize = 0x802B_9210;

/// Builds the `0xF0`-byte connection object for one player slot.
///
/// In plain English: swap the heap selector to `heap`, allocate `0xF0` bytes and
/// run [`crate::engine::display::fn_800dd950`] with `index` as its owner field,
/// store the result in the fourth-slot table at [`OBJECT_SLOTS_803741D0`], then
/// restore the previous heap. The store happens even when the allocation
/// failed, so a failed slot is written as null.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80101FF0`.
/// - Extent: `0x80101FF0..0x80102060` (112 bytes, 28 words), ending at the
///   `blr` at `0x8010205C`; it sits between [`fn_80101fd0`] and
///   [`fn_80102060`], which reads the same slots.
/// - Reach: no `b`/`bl` in the main image; `d_basesNP` calls it from
///   `0x807B20BC` (the loop over the four slots) with `r3 = *(0x80377C48)` and
///   `r4 = index`. No data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `stw r30,0x8(r1)`; `mr r30,r4`;
///   `bl 0x8016E4F0`; `mr r31,r3`; `li r3,0xF0`; `bl 0x802B9210`;
///   `cmpi cr0,0,r3,0`; `mr r5,r3`; `41820010 bc 12,2` (`beq`, skip the
///   constructor); `mr r4,r30`; `bl 0x800DD950`; `mr r5,r3`;
///   `lis r4,0x8037`; `rlwinm r0,r30,2,0,29`; `addi r4,r4,0x41D0`;
///   `mr r3,r31`; `stwx r5,r4,r0`; `bl 0x8016E4F0`; epilogue; `blr`.
/// - Decoder note: the first `0x8016E4F0` call consumes the incoming `r3`, so
///   `heap` is a real first argument; `r4` is the slot index (`mr r30,r4`).
///
/// # Arguments
/// - `heap`: heap selector installed for the allocation and restored afterwards.
/// - `index`: slot number in `0..4`; not bounds-checked in the original.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Swaps the heap selector twice, allocates `0xF0` bytes and writes the slot.
///
/// # Safety
/// `heap` must be allocator-accepted and `index` must name a writable word in
/// the four-slot table.
///
/// # Confidence
/// high - the size, constructor pair, index shift and slot table are literal.
pub unsafe fn fn_80101ff0(heap: u32, index: i32) {
    let previous = platform::call1(W289_FN_8016E4F0, heap);
    let raw = platform::call1(W289_FN_802B9210, 0xF0) as usize;
    let mut object = raw;
    if raw != 0 {
        object = crate::engine::display::fn_800dd950(raw, index as u32);
    }
    let slot = (OBJECT_SLOTS_803741D0 as isize + (index as isize) * 4) as usize;
    platform::write_u32(slot, object as u32);
    platform::call1(W289_FN_8016E4F0, previous);
}

/// Tries to destroy one of the four object slots at `0x803741D0`.
///
/// Calls `0x8008F310(object + 0x40)`. If that call returns zero the slot is
/// considered still busy and the function returns 0 *without* clearing it.
/// Otherwise it reloads the slot pointer, invokes the virtual destructor at
/// `vtable + 8` with `(object, 1)`, clears the slot and returns 1.
///
/// # Original
/// - Ghidra: `FUN_80102060` at `0x80102060` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102060_FUN_80102060.c`
///
/// # Arguments
/// - `index`: slot number in `0..4`. The original does not bounds-check it.
///
/// # Returns
/// `1` when the slot is empty after the call, `0` when the busy test failed.
///
/// # Side effects
/// - calls `0x8008F310(object + 0x40)`;
/// - may call the virtual function `*(vtable + 8)(object, 1)`;
/// - may write zero to `0x803741D0 + index * 4`.
///
/// # Safety
/// The slot pointer, its vtable, `vtable + 8` and `object + 0x40` must all be
/// valid. The virtual callee must accept `(this, u32)` and return normally.
///
/// # Confidence
/// high - the control flow and both calls are explicit in the evidence.
pub unsafe fn fn_80102060(index: i32) -> u32 {
    let slot_addr = (OBJECT_SLOTS_803741D0 as isize + (index as isize) * 4) as usize;
    if platform::read_u32(slot_addr) == 0 {
        return 1;
    }

    let object = platform::read_u32(slot_addr);
    // observed: the busy gate reads object + 0x40; field role unknown.
    let busy_test = platform::call1(FUN_8008F310, object + 0x40);

    // observed: `-1 < (int)(-uVar1 | uVar1)` is a sign-bit test. The bit is
    // clear only when `busy_test == 0`, i.e. the object is still busy.
    let sign_test = busy_test.wrapping_neg() | busy_test;
    if -1i32 < sign_test as i32 {
        return 0;
    }

    // observed: the C reloads the slot after 0x8008F310, so the callee may have
    // changed or cleared it. Preserve that second read.
    let object = platform::read_u32(slot_addr);
    if object != 0 {
        let vtable = platform::read_u32(object as usize);
        // observed: virtual slot at vtable + 8, called as (object, 1);
        // inferred: deleting destructor.
        let destructor = platform::read_u32(vtable as usize + 8);
        platform::call2(destructor as usize, object, 1);
    }
    platform::write_u32(slot_addr, 0);
    1
}

/// Destroys all four object slots at `0x803741D0`.
///
/// Stops at the first slot for which [`fn_80102060`] returns 0.
///
/// # Original
/// - Ghidra: `FUN_801020F0` at `0x801020F0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801020f0_FUN_801020f0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` if all four slots were processed, `0` if one was still busy.
///
/// # Side effects
/// Same as [`fn_80102060`], applied in index order until a failure.
///
/// # Safety
/// Same requirements as [`fn_80102060`] for every visited slot.
///
/// # Confidence
/// high - direct loop over four indices.
pub unsafe fn fn_801020f0() -> u32 {
    let mut index: i32 = 0;
    loop {
        let result = fn_80102060(index);
        if result == 0 {
            return 0;
        }
        index += 1;
        if index >= OBJECT_SLOT_COUNT {
            return 1;
        }
    }
}

/// Calls `0x800DDC50` once for every non-null object slot at `0x803741D0`.
///
/// `// unknown:` Ghidra shows `FUN_800DDC50()` with no arguments. The slot
/// pointer is the obvious candidate for `r3`, but the decompiled body does not
/// prove it. This translation therefore performs `call0`, matching the
/// evidence exactly; see the coverage doc's uncertainty list.
///
/// # Original
/// - Ghidra: `FUN_80102140` at `0x80102140` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102140_FUN_80102140.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800DDC50` up to four times.
///
/// # Safety
/// `0x800DDC50` must be callable with no reconstructed arguments and must not
/// require state that this caller does not establish.
///
/// # Confidence
/// medium - loop and slot test are clear; the missing argument is a Ghidra
/// limitation, so the call is literal rather than ABI-complete.
pub unsafe fn fn_80102140() {
    let mut index: i32 = 0;
    loop {
        let addr = (OBJECT_SLOTS_803741D0 as isize + (index as isize) * 4) as usize;
        if platform::read_u32(addr) != 0 {
            platform::call0(FUN_800DDC50);
        }
        index += 1;
        if index >= OBJECT_SLOT_COUNT {
            return;
        }
    }
}

/// Returns 1 when the first object slot at `0x803741D0` is non-null.
///
/// The Ghidra expression `(-x | x) >> 31` extracts the sign of `-x | x`, which
/// is set exactly when `x != 0`.
///
/// # Original
/// - Ghidra: `FUN_801021B0` at `0x801021B0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801021b0_FUN_801021b0.c`
///
/// # Arguments
/// None; it reads `0x803741D0`.
///
/// # Returns
/// `1` if the 32-bit word at `0x803741D0` is non-zero, otherwise `0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `0x803741D0` must be mapped. No pointer dereference is performed.
///
/// # Confidence
/// high - the bit expression is fully explicit.
pub unsafe fn fn_801021b0() -> u32 {
    let value = platform::read_u32(OBJECT_SLOTS_803741D0);
    (value.wrapping_neg() | value) >> 31
}

/// Lazily creates the pool object stored at `r13 - 0x5454`.
///
/// If the SDA slot is already non-null this function does nothing. Otherwise it
/// calls the six-argument factory `0x8016E7F0` with
/// `(param_2 / param_3, param_3, param_1, 0x802F4A24, 0x20, 4)` and stores the
/// result.
///
/// # Original
/// - Ghidra: `FUN_801021D0` at `0x801021D0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801021d0_FUN_801021d0.c`
///
/// # Arguments
/// - `param_1`: opaque value forwarded as argument 3 of the factory.
/// - `param_2`: unsigned numerator; divided by `param_3`.
/// - `param_3`: unsigned divisor and factory argument 2. Must be non-zero;
///   PowerPC `divw`/`divwu` leaves the result undefined for zero, and the
///   original performs no check.
///
/// # Returns
/// Nothing. The created pointer (or the existing one) lives at `0x8042A22C`.
///
/// # Side effects
/// May call `0x8016E7F0` and write its return value to `0x8042A22C`.
///
/// # Safety
/// `param_3` must be non-zero. `0x8016E7F0` must accept the six arguments shown
/// and return a valid pool pointer or zero.
///
/// # Confidence
/// high for control flow and arguments; medium for the "pool" role (inferred
/// from the one-time-null check pattern).
pub unsafe fn fn_801021d0(param_1: u32, param_2: u32, param_3: u32) {
    if platform::read_u32(SDA_5454_POOL) != 0 {
        return;
    }

    // observed: first argument is `param_2 / param_3` (unsigned division).
    // unknown: the final 0x20 and 4 are forwarded verbatim; their meanings
    // (likely element size/count) are not established by this call site.
    let created = call6(
        FUN_8016E7F0,
        param_2 / param_3,
        param_3,
        param_1,
        DATA_802F4A24 as u32,
        0x20,
        4,
    );
    platform::write_u32(SDA_5454_POOL, created);
}

/// If any object slot is populated, destroys all four and sets the
/// "slots destroyed" byte at `r13 - 0x545A`.
///
/// # Original
/// - Ghidra: `FUN_80102220` at `0x80102220` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102220_FUN_80102220.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_801020f0`] and writes `1` to `0x8042A226` when the first slot is
/// non-null. The write happens even if one of the later slots refuses to be
/// destroyed, exactly as in the original.
///
/// # Safety
/// Same as [`fn_801020f0`].
///
/// # Confidence
/// high - three-step body is fully explicit.
pub unsafe fn fn_80102220() {
    if fn_801021b0() != 0 {
        fn_801020f0();
        platform::write_u8(SDA_545A_FLAG, 1);
    }
}

// ---------------------------------------------------------------------------
// Cluster B: bit-mask table and small-data reset
// ---------------------------------------------------------------------------

/// Tests whether `mask` shares any bit with one 32-bit entry of the table at
/// `0x803741F0`.
///
/// The entry address is `0x803741F0 + row * 0xA8 + column * 4`. `column` must
/// be below `0x2A`; otherwise the function returns 0 without reading memory.
///
/// # Original
/// - Ghidra: `FUN_80102750` at `0x80102750` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102750_FUN_80102750.c`
///
/// # Arguments
/// - `row`: signed row index; each row is 0xA8 bytes (42 words).
/// - `column`: unsigned column index; valid range `0..0x2A`.
/// - `mask`: bit-mask to test.
///
/// # Returns
/// `1` if `mask & table[row][column] != 0`, otherwise `0` (including the
/// out-of-range column case).
///
/// # Side effects
/// None.
///
/// # Safety
/// For `column < 0x2A`, the computed table address must be readable. `row` is
/// not range-checked by the original.
///
/// # Confidence
/// high - the address formula and bit test are explicit.
pub unsafe fn fn_80102750(row: i32, column: u32, mask: u32) -> u32 {
    if column < BITS_ROW_COUNT {
        let addr = (BITS_803741F0 as isize
            + (row as isize) * BITS_ROW_BYTES as isize
            + (column as isize) * 4) as usize;
        let entry = platform::read_u32(addr);
        let overlap = mask & entry;
        return (overlap.wrapping_neg() | overlap) >> 31;
    }
    0
}

/// Resets six small-data bytes used by the WorldMap/effect state.
///
/// The order is observable because two destination bytes are copied from
/// source bytes that are cleared later.
///
/// # Original
/// - Ghidra: `FUN_80102790` at `0x80102790` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102790_FUN_80102790.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// In order:
/// 1. `0x80429045 = 0xFF`
/// 2. `0x80429046 = 0x00`
/// 3. `0x8042A258 = 0x00`
/// 4. `0x8042A25B = 0x8042A25A`
/// 5. `0x8042A25D = 0x8042A25C`
/// 6. `0x8042A25A = 0x00`
/// 7. `0x8042A25C = 0x00`
///
/// # Safety
/// All seven byte addresses are fixed SDA locations and must be writable.
///
/// # Confidence
/// high - the seven stores are listed explicitly in the evidence.
pub unsafe fn fn_80102790() {
    platform::write_u8(SDA_663B_BYTE, 0xFF);
    platform::write_u8(SDA_663A_BYTE, 0x00);
    platform::write_u8(SDA_5428_BYTE, 0x00);
    platform::write_u8(SDA_5425_BYTE, platform::read_u8(SDA_5426_BYTE));
    platform::write_u8(SDA_5423_BYTE, platform::read_u8(SDA_5424_BYTE));
    platform::write_u8(SDA_5426_BYTE, 0x00);
    platform::write_u8(SDA_5424_BYTE, 0x00);
}

// ---------------------------------------------------------------------------
// Cluster C: packed identifier
// ---------------------------------------------------------------------------

/// Packs three small fields into one 32-bit value.
///
/// Layout: bits 0-7 from `low`, bits 8-15 from `high`, bits 4-7 from `mid`.
/// The `mid` and `low` fields overlap in bits 4-7; the OR order in the original
/// is `low`, then `high << 8`, then `mid << 4`, so a non-zero `mid` can change
/// bits 4-7. Preserve that exact expression.
///
/// # Original
/// - Ghidra: `FUN_80102A60` at `0x80102A60` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102a60_FUN_80102a60.c`
///
/// # Arguments
/// - `low`: value masked to 8 bits.
/// - `mid`: value shifted left 4.
/// - `high`: value shifted left 8.
///
/// # Returns
/// `(low & 0xFF) | (high << 8) | (mid << 4)`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure arithmetic; no memory access.
///
/// # Confidence
/// high - the expression is literal. The field meanings are unknown.
pub unsafe fn fn_80102a60(low: u32, mid: i32, high: i32) -> u32 {
    (low & 0xFF) | ((high as u32) << 8) | ((mid as u32) << 4)
}

// ---------------------------------------------------------------------------
// Cluster F: record accessors (0x801015A0..0x801016B0)
// ---------------------------------------------------------------------------

/// Returns the record pointer selected by two fields of an owner object.
///
/// The owner holds a signed index at `+0x1A8` and a row number at `+0x158`.
/// The index selects a base pointer from the table at `0x8031D8B4`; the row
/// number selects an 8-byte entry inside that base.
///
/// # Original
/// - Ghidra: `FUN_801015A0` at `0x801015A0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801015a0_FUN_801015a0.c`
///
/// # Arguments
/// - `param_1`: owner object with at least `0x1AC` readable bytes.
///
/// # Returns
/// `table[index] + row * 8`, or `0` when `index < 0`.
///
/// # Side effects
/// None; two owner reads and one table read.
///
/// # Safety
/// `param_1` must be readable at `+0x158` and `+0x1A8`; a non-negative index
/// must be inside the table and the selected base pointer must be readable at
/// the computed offset.
///
/// # Confidence
/// high - the signed-index branch and the address formula are explicit.
pub unsafe fn fn_801015a0(param_1: u32) -> u32 {
    let index = platform::read_i32(param_1 as usize + 0x1A8);
    if index < 0 {
        return 0;
    }
    let base = platform::read_u32(TABLE_8031D8B4 + (index as usize) * 4);
    let row = platform::read_u32(param_1 as usize + 0x158);
    base.wrapping_add(row.wrapping_mul(8))
}

/// Returns the first word of the record selected by [`fn_801015a0`].
///
/// Returns `0xFFFFFFFF` when the owner's byte at `+0x165` is zero or when the
/// record pointer is null.
///
/// # Original
/// - Ghidra: `FUN_801015E0` at `0x801015E0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801015e0_FUN_801015e0.c`
///
/// # Arguments
/// - `param_1`: owner object forwarded unchanged to [`fn_801015a0`].
///
/// # Returns
/// `*(u32*)fn_801015a0(param_1)` or `0xFFFFFFFF`.
///
/// # Side effects
/// None; reads the owner, the table and the selected record.
///
/// # Safety
/// Same as [`fn_801015a0`]; the selected record must also be readable at `+0`.
///
/// # Confidence
/// high - the original calls `0x801015A0` without rewriting `r3`, so the same
/// owner pointer is forwarded (verified in the instruction stream).
pub unsafe fn fn_801015e0(param_1: u32) -> u32 {
    let record = fn_801015a0(param_1);
    if platform::read_u8(param_1 as usize + 0x165) == 0 || record == 0 {
        return 0xFFFF_FFFF;
    }
    platform::read_u32(record as usize)
}

/// Returns the second word of the record selected by [`fn_801015a0`].
///
/// Unlike [`fn_801015e0`] there is no owner-byte gate: only a null record
/// pointer produces `0xFFFFFFFF`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80101630`.
/// - Extent: `0x80101630..0x80101664` (52 bytes, 13 instructions), ending at
///   the `blr` at `0x80101660`; one zero word precedes it at `0x8010162C` and
///   three follow before the next body at `0x80101670`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: no data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `bl 0x801015A0`; `cmpi cr0,0,r3,0`; `bc 12,2` (`beq`) to `li r3,-1`;
///   otherwise `lwz r3,0x4(r3)`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `param_1`: owner object forwarded unchanged to [`fn_801015a0`].
///
/// # Returns
/// `*(u32*)(fn_801015a0(param_1) + 4)`, or `0xFFFFFFFF` when the record
/// pointer is null.
///
/// # Side effects
/// None; reads the owner, the table and the selected record.
///
/// # Safety
/// Same as [`fn_801015a0`]; the selected record must also be readable at
/// `+4`.
///
/// # Confidence
/// high - the single call, the null test and the `+4` load are literal.
pub unsafe fn fn_80101630(param_1: u32) -> u32 {
    let record = fn_801015a0(param_1);
    if record == 0 {
        return 0xFFFF_FFFF;
    }
    platform::read_u32(record as usize + 4)
}

/// Stores four words in the order the compiler emitted: `[0]`, `[3]`, `[1]`,
/// `[2]`.
///
/// # Original
/// - Ghidra: `FUN_80101690` at `0x80101690` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101690_FUN_80101690.c`
///
/// # Arguments
/// - `dst`: destination record (`r3`).
/// - `a`: stored at `dst + 0` (`r4`).
/// - `b`: stored at `dst + 8` (`r5`).
/// - `c`: stored at `dst + 0xC` (`r6`).
/// - `d`: stored at `dst + 4` (`r7`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four 32-bit stores.
///
/// # Safety
/// `dst..dst+0x10` must be writable.
///
/// # Confidence
/// high - the four `stw` instructions are explicit; the callers' argument
/// mapping is documented at [`fn_801016b0`].
pub unsafe fn fn_80101690(dst: u32, a: u32, b: u32, c: u32, d: u32) {
    platform::write_u32(dst as usize, a);
    platform::write_u32(dst as usize + 4, d);
    platform::write_u32(dst as usize + 8, b);
    platform::write_u32(dst as usize + 0xC, c);
}

/// Scans four `0x10`-byte records for a negative word at `+0x168` and fills the
/// first match through [`fn_80101690`].
///
/// The original only writes `r3` for the call, so `r4`..`r7` are inherited
/// from this function's caller and become the four stored words. The one known
/// caller (`0x800F8F70`) passes `(owner, 0x29, param, 0, 0x80)`, which the
/// five-argument Rust signature makes explicit.
///
/// # Original
/// - Ghidra: `FUN_801016B0` at `0x801016B0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801016b0_FUN_801016b0.c`
///
/// # Arguments
/// - `param_1`: owner of the four-record array at `+0x168`.
/// - `arg_2`..`arg_5`: forwarded to [`fn_80101690`] (`r4`..`r7` in the
///   original). The known caller uses `0x29`, its own `param_1`, `0`, `0x80`.
///
/// # Returns
/// `1` when a matching record was filled, `0` when all four records were
/// non-negative.
///
/// # Side effects
/// May write four words into `owner + 0x168 + i*0x10`.
///
/// # Safety
/// The owner must be readable for four records and the selected record must be
/// writable.
///
/// # Confidence
/// medium - the loop and the inherited-argument behaviour are observed, but the
/// stored values' meaning is unknown.
pub unsafe fn fn_801016b0(
    param_1: u32,
    arg_2: u32,
    arg_3: u32,
    arg_4: u32,
    arg_5: u32,
) -> u32 {
    let mut record = param_1;
    let mut index: u32 = 0;
    let mut remaining: i32 = 4;
    loop {
        if platform::read_i32(record as usize + 0x168) < 0 {
            let dst = param_1
                .wrapping_add(index.wrapping_mul(0x10))
                .wrapping_add(0x168);
            fn_80101690(dst, arg_2, arg_3, arg_4, arg_5);
            return 1;
        }
        record = record.wrapping_add(0x10);
        index += 1;
        remaining -= 1;
        if remaining == 0 {
            return 0;
        }
    }
}

// ---------------------------------------------------------------------------
// Cluster G: display/mode state resets (0x801017D0, 0x801018C0)
// ---------------------------------------------------------------------------

/// Rewrites the `0x80373D60` state block for the scene that is being entered.
///
/// When `param_1` is non-zero the two state bytes at `+0xC`/`+0xD` come from a
/// two-byte row selected by `ARRAY_803741BC[SDA_5488_INDEX]`, and the block's
/// kind word is set to `3`. Otherwise the pair becomes `0`/`0x27` and the kind
/// is `2`. Both cases copy the pair to `+0xE`/`+0xF`, publish the block to the
/// layout code (`0x800B0DB0(0)`) and request scene `6` through `0x800E1E60`.
///
/// # Original
/// - Ghidra: `FUN_801017D0` at `0x801017D0` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801017d0_FUN_801017d0.c`
///
/// # Arguments
/// - `param_1`: non-zero selects the table-driven pair and kind `3`.
/// - `param_2`: forwarded twice to `0x800E1E60(6, param_2, param_2)`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Eight state-block writes, one call to `0x800B0DB0`, one SDA byte store and
/// one scene request.
///
/// # Safety
/// `SDA_5488_INDEX` must select a valid byte of `ARRAY_803741BC`, which must
/// select a valid row of `0x802F49D0`. `0x800E1E60` must accept the three
/// arguments shown.
///
/// # Confidence
/// high - every store, comparison and call is explicit.
pub unsafe fn fn_801017d0(param_1: i32, param_2: u32) {
    if param_1 != 0 {
        let index = platform::read_u32(SDA_5488_INDEX) as usize;
        let row = platform::read_u8(ARRAY_803741BC + index) as usize;
        platform::write_u8(STATE_6C, platform::read_u8(TABLE_802F49D0 + row * 2));
        platform::write_u8(
            STATE_6D,
            platform::read_u8(TABLE_802F49D0 + row * 2 + 1),
        );
        platform::write_u32(STATE_68, 3);
        platform::write_u8(STATE_67, 1);
    } else {
        platform::write_u8(STATE_6C, 0);
        platform::write_u8(STATE_6D, 0x27);
        platform::write_u32(STATE_68, 2);
        platform::write_u8(STATE_67, 0);
    }

    platform::write_u8(STATE_6E, platform::read_u8(STATE_6C));
    platform::write_u8(STATE_6F, platform::read_u8(STATE_6D));
    platform::write_u8(STATE_66, 0);
    platform::write_u8(STATE_65, 0xFF);

    platform::call1(FUN_800B0DB0, 0);
    platform::write_u8(SDA_54D0_BYTE, 1);
    platform::call3(FUN_800E1E60, 6, param_2, param_2);
}

/// Resets the `0x80373D60` state block to its neutral values and requests
/// scene `6`.
///
/// `+0x68` is cleared as a word, `+0x67`/`+0x66` as bytes, `+0x65` becomes
/// `0xFF`, and the two state pairs become `0`/`0x28`. The block is published
/// with `0x800B0DB0(1)`, the SDA byte at `0x8042A1B0` is set, the mode word at
/// `0x8042900C` becomes `2`, and `0x800E1E60(6, 0, 0)` runs last.
///
/// # Original
/// - Ghidra: `FUN_801018C0` at `0x801018C0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801018c0_FUN_801018c0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Eight state-block writes, one call to `0x800B0DB0`, two SDA stores and one
/// scene request.
///
/// # Safety
/// The state block and both SDA words must be writable; `0x800E1E60` must
/// accept the arguments shown.
///
/// # Confidence
/// high - the store order and every call are explicit.
pub unsafe fn fn_801018c0() {
    platform::write_u32(STATE_68, 0);
    platform::write_u8(STATE_67, 0);
    platform::write_u8(STATE_66, 0);
    platform::write_u8(STATE_65, 0xFF);
    platform::write_u8(STATE_6C, 0);
    platform::write_u8(STATE_6D, 0x28);
    platform::write_u8(STATE_6E, 0);
    platform::write_u8(STATE_6F, 0x28);

    platform::call1(FUN_800B0DB0, 1);
    platform::write_u8(SDA_54D0_BYTE, 1);
    platform::write_u32(SDA_6674_MODE, 2);
    platform::call3(FUN_800E1E60, 6, 0, 0);
}

// ---------------------------------------------------------------------------
// Cluster H: mode dispatcher (0x80102260)
// ---------------------------------------------------------------------------

/// Dispatches a mode/course change across three mutually exclusive paths.
///
/// The three paths are selected by the current mode word at
/// `r13 - 0x547C` (`2`/`3`), by bit `0x10` of the flags at `r13 - 0x5700`,
/// and by the object state read from `0x80315B98`. Every path ends in a scene
/// request (`0x800E1E60`) or, on the flag path, in the four-object update and
/// the `0x2C7`/`9` request pair.
///
/// `0x802DCD74` / `0x802DCDC0` in the Ghidra output are Metrowerks
/// save/restore-register helpers, not calls; they are omitted (see
/// `docs/COVERAGE_r80005_80014.md` §"Ghidra artefact"). The values Ghidra
/// prints as the helper's return are the saved `r29`/`r28` parameters.
///
/// # Original
/// - Ghidra: `FUN_80102260` at `0x80102260` (576 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102260_FUN_80102260.c`
///
/// # Arguments
/// - `param_1`: requested scene/mode id (compared with `10`, `8`, `3`).
/// - `param_2`: scene argument, forwarded to `0x800E1E60`.
/// - `param_3`: mode value stored at `r13 - 0x6674`; also gates the `0x2C7`
///   request on the flag path.
/// - `param_4`: forwarded to `0x800B0DB0` before any branch.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May write the mode word, the course object's `+0x370`, the flag byte at
/// `r13 - 0x5432` consumers' inputs, and may run scene requests, the
/// four-object update (`0x800CE940`) and `0x800B5040`.
///
/// # Safety
/// The course object at `r13 - 0x5704` must be readable at `+0x370` when the
/// table path is taken; `0x80315B90` and its `+8` word must be mapped; all
/// callees must accept the argument counts shown.
///
/// # Confidence
/// medium - every branch is reconstructed from the instruction stream, but the
/// save-register decompiler artefact required manual recovery of `param_1`/
/// `param_2`, and several callee roles are inferred.
pub unsafe fn fn_80102260(param_1: u32, param_2: u32, param_3: i32, param_4: u32) {
    platform::call1(FUN_800B0DB0, param_4);

    let mode = platform::read_u32(SDA_547C_MODE);
    if mode.wrapping_sub(2) <= 1 {
        platform::write_u32(SDA_6674_MODE, 2);
        if param_1 == 10 || param_1 == 8 {
            platform::call3(FUN_800E1E60, param_1, param_2, 0);
        } else {
            fn_801017d0(0, 0);
        }
    } else if (platform::read_u32(SDA_5700_FLAGS) & 0x10) == 0 {
        if platform::read_u32(OBJECT_80315B90 + 8) == 4 {
            platform::write_u32(SDA_6674_MODE, 2);
            fn_801018c0();
        } else {
            platform::write_u32(SDA_6674_MODE, param_3 as u32);

            let course = platform::read_u32(SDA_54B8_OBJECT);
            let course_byte_120c = platform::read_u8(course as usize + 0x120C);
            let course_byte_120d = platform::read_u8(course as usize + 0x120D);

            if mode == 1
                && param_3 == 1
                && course_byte_120c == 7
                && course_byte_120d == 0x17
            {
                platform::call3(FUN_800E1E60, 8, 2, 0);
            } else if param_1 == 3 {
                let table_value = platform::read_u32(
                    TABLE_802F49F0 + (course_byte_120c as usize) * 4,
                );
                if course_byte_120d == 0x23 && param_3 == 0 && table_value != 10 {
                    let course_object = platform::read_u32(SDA_5704_OBJECT);
                    platform::write_u32(course_object as usize + 0x370, 3);
                    platform::call3(FUN_80102690, table_value & 0xFF, 0, 0);
                } else {
                    platform::call3(
                        FUN_80102690,
                        course_byte_120c as u32,
                        platform::read_u8(SDA_5432_BYTE) as u32,
                        0,
                    );
                }
            } else {
                platform::call3(FUN_800E1E60, param_1, param_2, 0);
            }
        }
    } else {
        let object = platform::read_u32(SDA_56D0_OBJECT);
        if object != 0 {
            let mut index: u32 = 0;
            loop {
                let slot = platform::read_u32(TABLE_80354E60 + (index as usize) * 4) as usize;
                let argument = platform::read_u32(TABLE_80354E90 + slot * 4);
                platform::call3(FUN_800CE940, object, argument, index);
                index += 1;
                if index >= 4 {
                    break;
                }
            }
        }

        platform::write_u32(SDA_6674_MODE, 2);

        if param_3 == 0 {
            let value = call0_ret(FUN_8005FFB0);
            let inverted = value ^ 1;
            let expression = ((inverted as i32) >> 1)
                .wrapping_sub((inverted & value) as i32);
            if expression < 0 {
                platform::call3(FUN_800E1E60, 0x2C7, 0, 0);
                return;
            }
        }

        platform::call0(FUN_800B5040);
        if (platform::read_u32(SDA_5700_FLAGS) & 0x20) == 0 {
            if object != 0 {
                platform::call1(FUN_800CEB80, object);
            }
            platform::call3(FUN_800E1E60, 0x2C7, 0, 0);
        } else {
            platform::call3(FUN_800E1E60, 9, 0, 0);
        }
    }
}

// ---------------------------------------------------------------------------
// Cluster I: challenge/flag scan (0x801027C0)
// ---------------------------------------------------------------------------

/// Re-evaluates the challenge bits in the byte at `state + 2`.
///
/// The function keeps four one-shot bits: `0x02`, `0x04`, `0x08` and `0x10`,
/// plus `0x20`. It tests the tables behind `0x800FB1F0`/`0x800FB240`,
/// `0x800FC400`, `0x800FD3B0` and `0x800FAE40`, publishes "bit changed" bytes
/// at `r13 - 0x54E3` / `r13 - 0x54E4`, and finally sets `0x20` when every
/// check passed.
///
/// # Original
/// - Ghidra: `FUN_801027C0` at `0x801027C0` (640 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801027c0_FUN_801027c0.c`
///
/// # Arguments
/// None; the state object comes from `0x800E0380(*(r13-0x5640), -1)`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * reads `0x800FC970`, `0x80102750`, `0x800FB1F0`, `0x800FB240`,
///   `0x800FC400`, `0x800FD3B0` and `0x800FAE40` thousands of times;
/// * writes bits in `state + 2`;
/// * writes the two one-shot bytes at `0x8042A19D` / `0x8042A19C`.
///
/// # Safety
/// The state object must be writable at `+2`; all table callbacks must accept
/// the argument counts shown.
///
/// # Confidence
/// high - the four nestings, all bit masks and both one-shot writes are
/// explicit. What each bit gates is unknown.
pub unsafe fn fn_801027c0() {
    let state = platform::call2(
        FUN_800E0380,
        platform::read_u32(SDA_5640_OBJECT),
        0xFFFF_FFFF,
    );

    if (platform::read_u8(state as usize + 2) & 0x02) == 0 {
        if platform::call1(FUN_800FC970, 7) != 0 {
            let flags = platform::read_u8(state as usize + 2);
            platform::write_u8(state as usize + 2, flags | 0x02);
        }
    }

    if (platform::read_u8(state as usize + 2) & 0x04) == 0 {
        let mut all_ok = true;
        let mut i: u32 = 0;
        loop {
            let mut j: u32 = 0;
            loop {
                if fn_80102750(i as i32, j, 2) != 0
                    && platform::call2(FUN_800FB1F0, i, j) == 0
                    && platform::call2(FUN_800FB240, i, j) == 0
                {
                    all_ok = false;
                }
                j += 1;
                if j >= 0x2A {
                    break;
                }
            }
            i += 1;
            if i >= 8 {
                break;
            }
        }
        if all_ok {
            let flags = platform::read_u8(state as usize + 2);
            platform::write_u8(state as usize + 2, flags | 0x04);
        }
    }

    let mut ok = true;
    if (platform::read_u8(state as usize + 2) & 0x08) == 0 {
        let mut all_present = true;
        let mut i: u32 = 0;
        loop {
            if platform::call1(FUN_800FC400, i) == 0 {
                all_present = false;
                ok = false;
            }
            i += 1;
            if i >= 8 {
                break;
            }
        }
        if all_present {
            let flags = platform::read_u8(state as usize + 2);
            platform::write_u8(state as usize + 2, flags | 0x08);
        }
    }

    platform::write_u8(SDA_54E3_BYTE, 0);
    let before_10 = platform::read_u8(state as usize + 2);
    platform::write_u8(state as usize + 2, before_10 & !0x10);
    if platform::call1(FUN_800FC400, 8) == 0 {
        ok = false;
    } else {
        if (before_10 & 0x10) == 0 {
            platform::write_u8(SDA_54E3_BYTE, 1);
        }
        let flags = platform::read_u8(state as usize + 2);
        platform::write_u8(state as usize + 2, flags | 0x10);
    }

    platform::write_u8(SDA_54E4_BYTE, 0);
    let before_20 = platform::read_u8(state as usize + 2);
    platform::write_u8(state as usize + 2, before_20 & !0x20);

    let mut i: u32 = 0;
    loop {
        let mut j: u32 = 0;
        loop {
            if platform::call2(FUN_800FD3B0, i, j) == 0 {
                ok = false;
            }
            j += 1;
            if j >= 0x2A {
                break;
            }
        }
        i += 1;
        if i >= 9 {
            break;
        }
    }

    i = 0;
    loop {
        if fn_80102750(i as i32, 0x23, 0x40) != 0
            && platform::call2(FUN_800FAE40, i, 0x23) == 0
        {
            ok = false;
        }
        i += 1;
        if i >= 8 {
            break;
        }
    }

    if ok {
        if (before_20 & 0x20) == 0 {
            platform::write_u8(SDA_54E4_BYTE, 1);
        }
        let flags = platform::read_u8(state as usize + 2);
        platform::write_u8(state as usize + 2, flags | 0x20);
    }
}

// ---------------------------------------------------------------------------
// Cluster J: course-record loader (0x80102A80)
// ---------------------------------------------------------------------------

/// Loads the 10x4 course record into the current course object.
///
/// It copies three header bytes (`+3`, `+4`, `+5`) to `+0x3C`, `+0x40`,
/// `+0x44`, sets `+0x48` to `-1`, publishes two `0x800CE560` values through
/// `0x800BBC50`/`0x800BBC60`, stores a boolean from record byte `+7` at
/// `+0x380`, then walks the 10x4 table. Each cell is packed into a three-word
/// local (`u8`, sign-extended `char`, `u8`) and handed to `0x800BBB70`; the row
/// byte from `0x800CE540` goes to `+0x395 + i`, a `'*'` marker to
/// `+0x3A9 + i`, and `0x2A` to `+0x39F + i`.
///
/// `0x802DCD6C` / `0x802DCDB8` are Metrowerks save/restore-register helpers,
/// not calls; they are omitted.
///
/// # Original
/// - Ghidra: `FUN_80102A80` at `0x80102A80` (400 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80102a80_FUN_80102a80.c`
///
/// # Arguments
/// None; the record comes from `0x800E0380(*(r13-0x5640), -1)` and the course
/// object from `r13 - 0x5704`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the course object's `+0x3C..+0x48`, `+0x380`, `+0x395..+0x39E`,
/// `+0x39F..+0x3A8` and `+0x3A9..+0x3B2`, the SDA byte at `r13 - 0x5428`, and
/// calls `0x800FCDB0` last.
///
/// # Safety
/// The record and course object must be readable/writable for the offsets
/// above; `0x800BBB70` must accept a pointer to the three-word cell.
///
/// # Confidence
/// medium - the decompiler dropped the first argument of `0x800CE560`; the
/// instruction stream shows `r3` still holds the record, and the signed-char
/// extension of `0x800CE4D0` was recovered from `extsb`.
pub unsafe fn fn_80102a80() {
    let record = platform::call2(
        FUN_800E0380,
        platform::read_u32(SDA_5640_OBJECT),
        0xFFFF_FFFF,
    );
    let course = platform::read_u32(SDA_5704_OBJECT);

    platform::write_u32(
        course as usize + 0x3C,
        platform::read_u8(record as usize + 3) as u32,
    );
    platform::write_u32(
        course as usize + 0x40,
        platform::read_u8(record as usize + 4) as u32,
    );
    platform::write_u32(
        course as usize + 0x44,
        platform::read_u8(record as usize + 5) as u32,
    );
    platform::write_u32(course as usize + 0x48, 0xFFFF_FFFF);

    let first = platform::call1(FUN_800CE560, record);
    platform::call3(FUN_800BBC50, course, 4, first);
    let second = platform::call1(FUN_800CE560, record);
    platform::call3(FUN_800BBC60, course, 4, second);

    let marker = platform::read_u8(record as usize + 7) as u32;
    platform::write_u8(
        course as usize + 0x380,
        ((marker.wrapping_neg() | marker) >> 31) as u8,
    );

    let mut i: u32 = 0;
    loop {
        let mut j: u32 = 0;
        loop {
            let first_byte = platform::call3(FUN_800CE490, record, i, j) & 0xFF;
            let signed = (platform::call3(FUN_800CE4D0, record, i, j) as u8 as i8) as i32;
            let third_byte = platform::call3(FUN_800CE510, record, i, j) & 0xFF;
            let mut cell = crate::platform::GuestLocal::new( [first_byte, signed as u32, third_byte]);
            platform::call4(
                FUN_800BBB70,
                course,
                i,
                j,
                cell.as_mut_ptr() as u32,
            );
            j += 1;
            if j >= 4 {
                break;
            }
        }

        let row_byte = platform::call2(FUN_800CE540, record, i) as u8;
        let base = course.wrapping_add(i);
        platform::write_u8(base as usize + 0x395, row_byte);

        let again = platform::call2(FUN_800CE540, record, i) as u8;
        platform::write_u8(
            base as usize + 0x3A9,
            if again == 0x2A { 1 } else { 0 },
        );

        i += 1;
        platform::write_u8(base as usize + 0x39F, 0x2A);
        if i >= 10 {
            break;
        }
    }

    platform::write_u8(SDA_5428_BYTE, 0);
    platform::call0(FUN_800FCDB0);
}

// ---------------------------------------------------------------------------
// Cluster K: course/scene record lookups (0x80100200..0x80100720)
// ---------------------------------------------------------------------------

/// Resolves one scene anchor `(scene, key)` into a three-word point.
///
/// The row index is `i32 *(scene + 0x338C)`; the 192-record table of the row
/// starts at `scene + 0x5C7F4 + row * 0x3F08`. [`crate::engine::dsys::fn_800f32f0`]
/// finds the `0x54`-byte record whose `+0x50` word equals `key` and returns
/// `record + 4`; the body then forwards `record + 8` to [`fn_80100270`], which
/// writes the three words at `dst`. This is the `(dst, scene, key)` shape the
/// many `game::bases` callers describe as an anchor-to-position lookup.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80100200`.
/// - Extent: `0x80100200..0x80100264` (100 bytes, 25 words), ending at the
///   `blr` at `0x80100260`; three zero words follow at
///   `0x80100264..0x80100270` and the next body starts at `0x80100270`.
/// - Reach: no `b`/`bl` and no data word in the main image; `d_basesNP` calls
///   it from `78` sites and `d_en_bossNP` from `10` (`0x8075BE24` ..
///   `0x80C56D3C`), which is why the `game::bases` prose names it directly.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r4`; `stw r30,0x8(r1)`; `mr r30,r3`;
///   `3C7F0006 addis r3,r31,6` (`scene + 0x60000`);
///   `8004338C lwz r0,0x338C(r4)`; `mr r4,r5`; `1C003F08 mulli r0,r0,0x3F08`;
///   `7C630214 add r3,r3,r0`; `3863C7F4 addi r3,r3,-0x380C` (net
///   `+0x5C7F4`); `bl 0x800F32F0`; `mr r5,r3`; `mr r3,r30`; `mr r4,r31`;
///   `38A50004 addi r5,r5,4`; `bl 0x80100270`; epilogue; `4E800020 blr`.
/// - Decoder note: `addis r3,r31,6` and the negative `addi` fold the high and
///   low halves of `+0x5C7F4`; it is not a `lis` page, so the effective base
///   offset is the sum, not `0x60000`.
/// - Evidence: `work/wave286/e_dis.txt`, `work/wave286/e_refs.txt`.
///
/// # Arguments
/// - `dst`: destination for the three words (`r3`).
/// - `scene`: scene object; `+0x338C` is the row index (`r4`).
/// - `key`: record key looked up through `+0x50` (`r5`).
///
/// # Returns
/// Nothing; the last callee's `r3` is not part of the contract.
///
/// # Side effects
/// One record lookup and a 12-byte write at `dst`.
///
/// # Safety
/// `scene` must be readable at `+0x338C`, the selected row table must hold 192
/// readable records and `dst` must be writable for 12 bytes.
///
/// # Confidence
/// medium - the arithmetic, the two calls and the `+4` bias are literal; the
/// field roles are the ones the `game::bases` call sites already use.
pub unsafe fn fn_80100200(dst: u32, scene: u32, key: u32) {
    let index = platform::read_i32(scene as usize + 0x338C);
    let table = scene
        .wrapping_add((index as u32).wrapping_mul(0x3F08))
        .wrapping_add(0x5C7F4);
    let record = crate::engine::dsys::fn_800f32f0(table, key);
    fn_80100270(dst, scene, record.wrapping_add(4));
}

/// Writes the three-word record produced by `0x801006E0` into `dst`.
///
/// The stores are emitted in the order `dst[1]`, `dst[0]`, `dst[2]`. The record
/// lives in a stack temporary, so the order only matters if `dst` aliases one
/// of the arguments forwarded to the callee.
///
/// # Original
/// - Ghidra: `FUN_80100270` at `0x80100270` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100270_FUN_80100270.c`
///
/// # Arguments
/// - `dst`: destination for the three 32-bit words.
/// - `a`, `b`: forwarded unchanged to `0x801006E0` together with `&record`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 12 bytes at `dst`; the callee may update course state.
///
/// # Safety
/// `dst` must be writable for 12 bytes and the callee must accept the
/// forwarded arguments.
///
/// # Confidence
/// high - the instruction stream matches the decompiled body.
pub unsafe fn fn_80100270(dst: u32, a: u32, b: u32) {
    let mut record = crate::platform::GuestLocal::new( [0u32; 3]);
    platform::call3(
        FUN_801006E0,
        a,
        b,
        record.as_mut_ptr() as u32,
    );
    platform::write_u32(dst as usize + 4, record[1]);
    platform::write_u32(dst as usize, record[0]);
    platform::write_u32(dst as usize + 8, record[2]);
}

/// Tail-calls the course-string table lookup at `0x800F53A0`.
///
/// The function computes
/// `scene + *(i32*)(scene + 0x338C) * 0x16518 + 0x3394` and passes only that
/// pointer. The two zero words between the load and the arithmetic are inline
/// data skipped by the branch at `0x801002C4`.
///
/// # Original
/// - Ghidra: `FUN_801002c0` at `0x801002C0` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801002c0_FUN_801002c0.c`
///
/// # Arguments
/// - `scene`: pointer to the current scene object.
///
/// # Returns
/// Nothing; the callee's integer result is discarded.
///
/// # Side effects
/// Whatever `0x800F53A0` performs on the selected string table.
///
/// # Safety
/// `scene` must be readable at `+0x338C` and the computed table pointer must
/// address a valid `0x800F53A0` table.
///
/// # Confidence
/// high - direct transcription; only the callee's role is unknown.
pub unsafe fn fn_801002c0(scene: u32) {
    let index = platform::read_i32(scene as usize + 0x338C);
    let table = (scene as isize + (index as isize) * 0x16518 + 0x3394) as u32;
    platform::call1(FUN_800F53A0, table);
}

/// Returns the first course string with `'W'` at byte 0 and `'S'` at byte 2.
///
/// The count comes from `scene + index * 0x16518 + 0x33A0` and each entry from
/// `0x800F56B0(table, i)`, which adds `i * 0x1C0 + 0x18` to the table base
/// `scene + index * 0x16518 + 0x3394`.
///
/// # Original
/// - Ghidra: `FUN_801002f0` at `0x801002F0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801002f0_FUN_801002f0.c`
///
/// # Arguments
/// - `scene`: pointer to the current scene object.
///
/// # Returns
/// The matching entry pointer, or `0` when the loop exhausts the count.
///
/// # Side effects
/// None; both callees are pure lookups.
///
/// # Safety
/// `scene` and the selected course table must be mapped for the offsets the
/// loop reads.
///
/// # Confidence
/// high - the loop and the two byte tests are exact.
pub unsafe fn fn_801002f0(scene: u32) -> u32 {
    let stride = platform::read_i32(scene as usize + 0x338C);
    let base = (scene as isize + (stride as isize) * 0x16518) as usize;
    let count = platform::read_i32(base + 0x33A0);

    let mut i: i32 = 0;
    while i < count {
        let entry = platform::call2(
            FUN_800F56B0,
            (base + 0x3394) as u32,
            i as u32,
        );
        if platform::read_u8(entry as usize) == 0x57
            && platform::read_u8(entry as usize + 2) == 0x53
        {
            return entry;
        }
        i += 1;
    }
    0
}

/// Returns the first nonzero result of [`fn_801005c0`] over the scene entries.
///
/// The count is the signed word at `scene + 0x3390`; each candidate index is
/// passed straight through.
///
/// # Original
/// - Ghidra: `FUN_80100530` at `0x80100530` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100530_FUN_80100530.c`
///
/// # Arguments
/// - `scene`: pointer to the current scene object.
/// - `a`: string/name forwarded to `0x801005C0`.
/// - `b`: optional context forwarded to `0x801005C0`.
///
/// # Returns
/// The first matching entry pointer, or `0` when no index matches.
///
/// # Side effects
/// Each candidate calls `0x801005C0`, which may run a comparison callback.
///
/// # Safety
/// `scene` must be readable at `+0x3390` and the per-entry lookup must be
/// valid for every index below the count.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_80100530(scene: u32, a: u32, b: u32) -> u32 {
    let count = platform::read_i32(scene as usize + 0x3390);
    let mut i: i32 = 0;
    while i < count {
        let found = fn_801005c0(scene, a, i, b);
        if found != 0 {
            return found;
        }
        i += 1;
    }
    0
}

/// Finds one entry in the `0xBF8`-byte scene record addressed by an index.
///
/// The handle comes from `0x80168400(scene + index * 0xBF8 + 0x1A8)`. The
/// string length is `0x802DC69C(a)`. The start index is `0` when `context` is
/// null; otherwise it is `*(context[0] + 0xC) + 1` when `context[0]` is
/// non-null, or `1` when it is null.
///
/// At each step the list count is `0x8023B360(&handle)`; the candidate is
/// `0x8023B2A0(&handle, index)`, and its compared node is
/// `candidate + *(u32*)(candidate + 8)` when that offset is nonzero, else `0`.
/// `0x802E1BD0(node, a, length)` returns `0` for a match.
///
/// # Original
/// - Ghidra: `FUN_801005c0` at `0x801005C0` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801005c0_FUN_801005c0.c`
///
/// # Arguments
/// - `scene`: pointer to the current scene object.
/// - `a`: zero-terminated name to compare.
/// - `index`: scene-record index scaled by `0xBF8`.
/// - `context`: optional object whose first word supplies the start index.
///
/// # Returns
/// The matching list element pointer, or `0` when no element matches.
///
/// # Side effects
/// Calls `0x80168400`, `0x8023B360`, `0x8023B2A0` and `0x802E1BD0`.
///
/// # Safety
/// `scene`, `a` and `context` must satisfy the offsets above; the handle
/// returned by `0x80168400` must be usable by the list helpers.
///
/// # Confidence
/// medium - the decompiler drops the `0x802DCD70`/`0x802DCDBC` save/restore
/// calls and the `&handle` argument; both were recovered from the instruction
/// stream.
pub unsafe fn fn_801005c0(scene: u32, a: u32, index: i32, context: u32) -> u32 {
    let record = (scene as isize + (index as isize) * 0xBF8 + 0x1A8) as u32;
    let mut handle = crate::platform::GuestLocal::new( platform::call1(FUN_80168400, record));
    let length = platform::call1(FUN_802DC69C, a);

    let mut start: u32 = 0;
    if context != 0 {
        let first = platform::read_u32(context as usize);
        start = if first != 0 {
            platform::read_u32(first as usize + 0xC) + 1
        } else {
            1
        };
    }

    let mut element: u32 = 0;
    let mut matched = false;
    loop {
        let handle_ptr = (handle.as_mut_ptr() as *mut u32) as u32;
        let count = platform::call1(FUN_8023B360, handle_ptr);
        if count <= start {
            break;
        }

        element = platform::call2(FUN_8023B2A0, handle_ptr, start);
        let offset = platform::read_u32(element as usize + 8);
        let node = if offset != 0 {
            element.wrapping_add(offset)
        } else {
            0
        };
        if platform::call3(FUN_802E1BD0, node, a, length) == 0 {
            matched = true;
            break;
        }
        start += 1;
    }

    if matched {
        element
    } else {
        0
    }
}

/// Tail-calls the scene-record getter at `0x800F0EC0`.
///
/// It computes `scene + *(i32*)(scene + 0x338C) * 0xBF8 + 0x1A0` and forwards
/// `a` and `b` in `r4`/`r5`. Ghidra prints a one-argument prototype because
/// this function itself does not touch `r4`/`r5`, but the callee does use
/// them.
///
/// # Original
/// - Ghidra: `FUN_801006e0` at `0x801006E0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801006e0_FUN_801006e0.c`
///
/// # Arguments
/// - `scene`: pointer to the current scene object.
/// - `a`, `b`: forwarded unchanged to `0x800F0EC0`.
///
/// # Returns
/// Nothing; the callee's result is discarded.
///
/// # Side effects
/// Whatever `0x800F0EC0` performs on the selected record.
///
/// # Safety
/// `scene` must be readable at `+0x338C` and the computed record must be valid.
///
/// # Confidence
/// medium - the forwarded arguments are recovered from the unchanged `r4`/`r5`
/// registers rather than from Ghidra's prototype.
pub unsafe fn fn_801006e0(scene: u32, a: u32, b: u32) {
    let index = platform::read_i32(scene as usize + 0x338C);
    let record = (scene as isize + (index as isize) * 0xBF8 + 0x1A0) as u32;
    platform::call3(FUN_800F0EC0, record, a, b);
}

/// Formats course entries and appends them until `0x800F53A0` reports failure.
///
/// The mode selects one of three static buffers:
/// `0` uses `r13 - 0x6838`; `1` uses `r13 - 0x6838` when the byte at
/// `r13 - 0x5433` equals `2`, otherwise `r13 - 0x6830`; `2` uses
/// `r13 - 0x6828`; any other mode skips formatting. Each buffer is filled by
/// `0x802E16E8(&scratch, 8, buffer, i)` and then passed to
/// `0x800F53A0(scene + index * 0x16518 + 0x3394, &scratch)`.
///
/// # Original
/// - Ghidra: `FUN_80100720` at `0x80100720` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100720_FUN_80100720.c`
///
/// # Arguments
/// - `scene`: pointer to the current scene object.
/// - `mode`: buffer/format selector (`0`, `1`, `2`, or anything else).
///
/// # Returns
/// The index at which the append failed. The search starts at index `0`, so a
/// failure on the very first entry returns `0`.
///
/// # Side effects
/// Fills and consumes the selected static buffer, calls `0x802E16E8` and
/// mutates the course string table through `0x800F53A0`.
///
/// # Safety
/// `scene` and the selected buffer must be valid for the durations above.
///
/// # Confidence
/// high - the three buffer paths and the loop exit condition are exact.
pub unsafe fn fn_80100720(scene: u32, mode: i32) -> i32 {
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 8]);
    let mut i: i32 = 0;

    loop {
        let buffer = match mode {
            0 => SDA_6838_BUFFER,
            1 => {
                if platform::read_u8(SDA_5433_BYTE) == 2 {
                    SDA_6838_BUFFER
                } else {
                    SDA_6830_BUFFER
                }
            }
            2 => SDA_6828_BUFFER,
            _ => 0,
        };

        if buffer != 0 {
            platform::call4(
                FUN_802E16E8,
                scratch.as_mut_ptr() as u32,
                8,
                buffer as u32,
                i as u32,
            );
        }

        let stride = platform::read_i32(scene as usize + 0x338C);
        let table = (scene as isize + (stride as isize) * 0x16518 + 0x3394) as u32;
        let result = platform::call2(
            FUN_800F53A0,
            table,
            scratch.as_mut_ptr() as u32,
        ) as i32;
        if result < 0 {
            return i;
        }
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// Cluster L: state queries and small global resets (0x80100830..0x801017B0)
// ---------------------------------------------------------------------------

/// Publishes the `0x8031C8B0` render state and the one-shot `0x8031C9B0`
/// reset.
///
/// Three `/f` values from the `r2` pool (`0x8042D0A0`, `0x8042D0A4`,
/// `0x8042D0A8`) are stored at `0x8031C8B0 + 0x18..+0x20`, and the word at
/// `r2 - 0x6114` is stored at `+0xC`. The function then pushes the callback
/// `0x801008E0` with data `0x80373D30` through `0x802DC780(0, ...)`, copies
/// `r2 - 0x60FC` into `r13 - 0x54F8`, and, while the byte at
/// `r13 - 0x54F0` is zero, stores the float at `r2 - 0x5FD4` into
/// `0x8031C9B0 + 0x20..+0x28` and sets that byte to `1`.
///
/// The Ghidra body also shows three `stfs` spills to the stack; they are dead
/// stores emitted by the compiler and are omitted here.
///
/// # Original
/// - Ghidra: `FUN_80100830` at `0x80100830` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100830_FUN_80100830.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `0x8031C8B0 + 0xC..+0x20`, `0x8031C9B0 + 0x20..+0x28`,
/// `r13 - 0x54F8` and `r13 - 0x54F0`; calls `0x802DC780`.
///
/// # Safety
/// Both state blocks and the `r2` pool addresses must be readable/writable.
///
/// # Confidence
/// high - all live stores and the guard match the instruction stream.
pub unsafe fn fn_80100830() {
    platform::write_u32(
        STATE_8031C8B0 + 0x0C,
        platform::read_u32(SDA2_BASE - 0x6114),
    );

    let f2 = platform::read_f32(SDA2_BASE - 0x5FE0);
    let f1 = platform::read_f32(SDA2_BASE - 0x5FDC);
    let f0 = platform::read_f32(SDA2_BASE - 0x5FD8);
    platform::write_f32(STATE_8031C8B0 + 0x18, f2);
    platform::write_f32(STATE_8031C8B0 + 0x1C, f1);
    platform::write_f32(STATE_8031C8B0 + 0x20, f0);

    platform::call3(FUN_802DC780, 0, FUN_801008E0 as u32, DATA_80373D30 as u32);

    platform::write_u32(SDA_54F8_WORD, platform::read_u32(SDA2_BASE - 0x60FC));

    if platform::read_u8(SDA_54F0_BYTE) == 0 {
        let value = platform::read_f32(SDA2_BASE - 0x5FD4);
        platform::write_f32(STATE_8031C9B0 + 0x20, value);
        platform::write_f32(STATE_8031C9B0 + 0x24, value);
        platform::write_f32(STATE_8031C9B0 + 0x28, value);
        platform::write_u8(SDA_54F0_BYTE, 1);
    }
}

/// Tests whether any node in the `r13 - 0x54E0` list has both flag bytes set.
///
/// The walk starts at the list head itself and follows `+0x184`. A node counts
/// when its byte at `+0x124` and its byte at `+0x18C` are both nonzero.
///
/// # Original
/// - Ghidra: `FUN_80100900` at `0x80100900` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100900_FUN_80100900.c`
///
/// # Arguments
/// None; the head comes from `r13 - 0x54E0`.
///
/// # Returns
/// `1` when a node satisfies both bytes, otherwise `0`.
///
/// # Side effects
/// None; read-only list walk.
///
/// # Safety
/// Every reachable node must be readable at `+0x124`, `+0x184` and `+0x18C`.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_80100900() -> u32 {
    let mut node = platform::read_u32(SDA_54E0_LIST);
    while node != 0 {
        if platform::read_u8(node as usize + 0x124) != 0
            && platform::read_u8(node as usize + 0x18C) != 0
        {
            return 1;
        }
        node = platform::read_u32(node as usize + 0x184);
    }
    0
}

/// Counts the nodes after the `r13 - 0x54E0` list head whose `+0x124` byte is
/// nonzero.
///
/// The head itself is skipped: the walk starts at `head + 0x184`.
///
/// # Original
/// - Ghidra: `FUN_80100940` at `0x80100940` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100940_FUN_80100940.c`
///
/// # Arguments
/// None; the head comes from `r13 - 0x54E0`.
///
/// # Returns
/// The number of matching nodes.
///
/// # Side effects
/// None; read-only list walk.
///
/// # Safety
/// The head must be readable at `+0x184` and every reachable node at `+0x124`
/// and `+0x184`.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_80100940() -> i32 {
    let mut count: i32 = 0;
    let head = platform::read_u32(SDA_54E0_LIST);
    let mut node = platform::read_u32(head as usize + 0x184);
    while node != 0 {
        if platform::read_u8(node as usize + 0x124) != 0 {
            count += 1;
        }
        node = platform::read_u32(node as usize + 0x184);
    }
    count
}

/// Returns `1` for the two effect modes `7` and `8`, otherwise `0`.
///
/// # Original
/// - Ghidra: `FUN_80100f70` at `0x80100F70` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100f70_FUN_80100f70.c`
///
/// # Arguments
/// - `mode`: integer mode selector.
///
/// # Returns
/// `1` when `mode == 7 || mode == 8`, else `0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// This function only compares integers and has no memory preconditions.
///
/// # Confidence
/// high - exact comparison chain.
pub unsafe fn fn_80100f70(mode: i32) -> u32 {
    if mode == 7 || mode == 8 {
        1
    } else {
        0
    }
}

/// Runs the per-slot effect/update pass selected by an eight-byte table row.
///
/// The current course object is `r13 - 0x5704`. For each of its four word
/// slots at `+0x384`, when the slot equals `3` the function first calls
/// `0x800DC990` with the matching entry from the four-pointer array at
/// `r13 - 0x5658` (`+4`, `+8`, `+0xC`, `+0x10`), then repeats the walk and,
/// when the selected object's byte at `+0x55` is nonzero, invokes
/// `0x802BC9A0(float, 0x80377C88[object[+4]], s16, u8)`.
///
/// The float, signed halfword and byte all come from one row of the table at
/// `0x8031D39C + n * 8`: `/f` at `+0x26C`, halfword at `+0x270`, byte at
/// `+0x272`. The row therefore depends on `n`, not on the slot index.
///
/// # Original
/// - Ghidra: `FUN_80100f90` at `0x80100F90` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80100f90_FUN_80100f90.c`
///
/// # Arguments
/// - `caller_arg`: forwarded in `r3` by the wrappers but never read here.
/// - `n`: row selector for the parameter table at `0x8031D39C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x800DC990` up to four times and `0x802BC9A0` up to four times.
///
/// # Safety
/// The course object, the `r13 - 0x5658` array and the selected table row
/// must satisfy the offsets above. `0x802BC9A0` is entered through an `f32`
/// argument that Ghidra's prototype drops.
///
/// # Confidence
/// medium - the argument count and the `f32` ABI were recovered from the
/// instruction stream; the callee's meaning is unknown.
pub unsafe fn fn_80100f90(caller_arg: u32, n: i32) {
    let _ = caller_arg;
    let course = platform::read_u32(SDA_5704_OBJECT);

    let mut i: u32 = 0;
    let mut array_offset: u32 = 0;
    let mut slot = course;
    while i < 4 {
        if platform::read_u32(slot as usize + 0x384) == 3 {
            let array = platform::read_u32(SDA_5658_ARRAY);
            let object = platform::read_u32(
                array.wrapping_add(array_offset).wrapping_add(4) as usize,
            );
            platform::call1(FUN_800DC990, object);
        }
        i += 1;
        array_offset += 4;
        slot = slot.wrapping_add(4);
    }

    let table = (TABLE_8031D39C as isize + (n as isize) * 8) as usize;
    let value = platform::read_f32(table + 0x26C);
    let half = platform::read_u16(table + 0x270) as i16 as i32;
    let byte = platform::read_u8(table + 0x272);

    let mut i: u32 = 0;
    let mut array_offset: u32 = 0;
    let mut slot = course;
    while i < 4 {
        if platform::read_u32(slot as usize + 0x384) == 3 {
            let array = platform::read_u32(SDA_5658_ARRAY);
            let object = platform::read_u32(
                array.wrapping_add(array_offset).wrapping_add(4) as usize,
            );
            if platform::read_u8(object as usize + 0x55) != 0 {
                let index = platform::read_u32(object as usize + 4) as usize;
                let entry = platform::read_u32(TABLE_80377C88 + index * 4);
                call_f32_3(FUN_802BC9A0, value, entry, half, byte);
            }
        }
        i += 1;
        array_offset += 4;
        slot = slot.wrapping_add(4);
    }
}

/// Wrapper that runs [`fn_80100f90`] with table row `3`.
///
/// # Original
/// - Ghidra: `FUN_80101070` at `0x80101070` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101070_FUN_80101070.c`
///
/// # Arguments
/// - `caller_arg`: forwarded unchanged to `0x80100F90`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Same as [`fn_80100f90`].
///
/// # Safety
/// Same as [`fn_80100f90`].
///
/// # Confidence
/// high - one `li r4,3` followed by a tail branch.
pub unsafe fn fn_80101070(caller_arg: u32) {
    fn_80100f90(caller_arg, 3);
}

/// Wrapper that runs [`fn_80100f90`] with table row `2`.
///
/// # Original
/// - Ghidra: `FUN_80101080` at `0x80101080` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101080_FUN_80101080.c`
///
/// # Arguments
/// - `caller_arg`: forwarded unchanged to `0x80100F90`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Same as [`fn_80100f90`].
///
/// # Safety
/// Same as [`fn_80100f90`].
///
/// # Confidence
/// high - one `li r4,2` followed by a tail branch.
pub unsafe fn fn_80101080(caller_arg: u32) {
    fn_80100f90(caller_arg, 2);
}

/// Returns the low bit of the mask selected by the `0x80354E60` index.
///
/// The word at `0x80354E60` is scaled by four and indexes the table at
/// `0x80354E80`; the mask's bit 0 is the result.
///
/// # Original
/// - Ghidra: `FUN_80101090` at `0x80101090` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80101090_FUN_80101090.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` or `1`, the low bit of `0x80354E80[*(0x80354E60)]`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Both tables must be readable at the selected index.
///
/// # Confidence
/// high - single indexed load and mask.
pub unsafe fn fn_80101090() -> u32 {
    let index = platform::read_u32(TABLE_80354E60) as usize;
    platform::read_u32(MASK_TABLE_80354E80 + index * 4) & 1
}

/// Returns one word from the table at `0x80320840` using a 16-bit index.
///
/// # Original
/// - Ghidra: `FUN_801017b0` at `0x801017B0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801017b0_FUN_801017b0.c`
///
/// # Arguments
/// - `index`: only the low 16 bits are used.
///
/// # Returns
/// The 32-bit value at `0x80320840 + (index & 0xFFFF) * 4`.
///
/// # Side effects
/// None.
///
/// # Safety
/// The computed address must be readable; the original performs no range
/// check.
///
/// # Confidence
/// high - direct indexed load.
pub unsafe fn fn_801017b0(index: u32) -> u32 {
    let element = (index & 0xFFFF) as usize;
    platform::read_u32(TABLE_80320840 + element * 4)
}

// ---------------------------------------------------------------------------
// Cluster M: main/layout/display object lifetimes (0x801072F0..0x801092F0)
// ---------------------------------------------------------------------------

/// Starts the boot helper `0x801CF580` and clears the barrier byte first.
///
/// The byte at `r13 - 0x53E4` is cleared, then `0x801CF580(0x80374920,
/// 0x801072E0)` runs. The return value is the truth value of the callee's
/// result (`(-r | r) >> 31`).
///
/// # Original
/// - Ghidra: `FUN_801072f0` at `0x801072F0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801072f0_FUN_801072f0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `1` when `0x801CF580` returns nonzero, else `0`.
///
/// # Side effects
/// Clears `r13 - 0x53E4` and invokes the boot helper.
///
/// # Safety
/// `0x801CF580` must accept the two constant arguments.
///
/// # Confidence
/// high - the barrier store and the sign-bit reduction are exact.
pub unsafe fn fn_801072f0() -> u32 {
    platform::write_u8(SDA_53E4_BYTE, 0);
    let result = platform::call2(
        FUN_801CF580,
        DATA_80374920 as u32,
        FUN_801072E0 as u32,
    );
    if result != 0 {
        1
    } else {
        0
    }
}

/// Waits for the boot barrier byte and returns the word behind it.
///
/// Spins while the byte at `r13 - 0x53E4` is zero, then reads
/// `r13 - 0x53E0`.
///
/// # Original
/// - Ghidra: `FUN_80107340` at `0x80107340` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80107340_FUN_80107340.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The word at `r13 - 0x53E0`.
///
/// # Side effects
/// Busy-waits until another context sets `r13 - 0x53E4`.
///
/// # Safety
/// The caller must arrange for the barrier byte to become nonzero, otherwise
/// this function never returns.
///
/// # Confidence
/// high - exact spin loop and load.
pub unsafe fn fn_80107340() -> u32 {
    while platform::read_u8(SDA_53E4_BYTE) == 0 {
        core::hint::spin_loop();
    }
    platform::read_u32(SDA_53E0_WORD)
}

/// Clears the main-object header and its two trailing words.
///
/// Calls `0x80107890(object + 8)`, then writes zero to `object[0]`, the byte
/// at `+4`, and the words at `+0x1B0` and `+0x1B4`.
///
/// # Original
/// - Ghidra: `FUN_80107360` at `0x80107360` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80107360_FUN_80107360.c`
///
/// # Arguments
/// - `object`: object to reset.
///
/// # Returns
/// `object`, the original pointer.
///
/// # Side effects
/// Calls `0x80107890` and writes four fields.
///
/// # Safety
/// `object` must be writable through `+0x1B7` and acceptable to `0x80107890`.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_80107360(object: u32) -> u32 {
    platform::call1(FUN_80107890, object + 8);
    platform::write_u32(object as usize, 0);
    platform::write_u8(object as usize + 4, 0);
    platform::write_u32(object as usize + 0x1B0, 0);
    platform::write_u32(object as usize + 0x1B4, 0);
    object
}

/// Allocates and initialises the 0x1C4-byte main game object.
///
/// The object comes from `0x802B9240(0x1C4, heap, 4)`. When non-null it is
/// reset by `0x80107360`; the resulting pointer is stored at `r13 - 0x53E8`
/// and passed to `0x80107410` with the title `0x80321C20`
/// (`"New SUPER MARIO BROS. Wii"`), the word at `r13 - 0x53F8` and `heap`.
///
/// # Original
/// - Ghidra: `FUN_801073b0` at `0x801073B0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801073b0_FUN_801073b0.c`
///
/// # Arguments
/// - `heap`: allocator passed as the second argument of `0x802B9240`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates memory, resets the object, stores it at `r13 - 0x53E8` and calls
/// `0x80107410`.
///
/// # Safety
/// `heap` must be a valid Metrowerks heap handle and the allocation must be
/// usable for 0x1C4 bytes.
///
/// # Confidence
/// high - allocation size, argument order and the constant title are exact.
pub unsafe fn fn_801073b0(heap: u32) {
    let allocated = platform::call3(FUN_802B9240, 0x1C4, heap, 4);
    let object = if allocated != 0 {
        platform::call1(FUN_80107360, allocated)
    } else {
        0
    };
    platform::write_u32(SDA_53E8_OBJECT, object);

    platform::call4(
        FUN_80107410,
        object,
        STR_80321C20 as u32,
        platform::read_u32(SDA_53F8_WORD),
        heap,
    );
}

/// Initialises the main-object header fields.
///
/// Calls `0x80107920(object + 8)`, stores `value` at `object[0]`, and clears
/// the byte at `+4`, the byte at `+0x1B8` and the word at `+0x1BC`. The second
/// argument (`title`) and the fourth argument (`heap`) are not read by the
/// body; they are part of the call shape used by [`fn_801073b0`].
///
/// # Original
/// - Ghidra: `FUN_80107410` at `0x80107410` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80107410_FUN_80107410.c`
///
/// # Arguments
/// - `object`: main object to initialise.
/// - `_title`: unused pointer to `"New SUPER MARIO BROS. Wii"`.
/// - `value`: word stored at `object[0]`.
/// - `_heap`: unused allocator handle.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80107920` and writes four fields.
///
/// # Safety
/// `object` must be writable through `+0x1BF` and acceptable to `0x80107920`.
///
/// # Confidence
/// high for the writes; medium for the unused `title`/`heap` parameters,
/// which are present in the call but absent from the callee body.
pub unsafe fn fn_80107410(
    object: u32,
    _title: u32,
    value: u32,
    _heap: u32,
) {
    platform::call1(FUN_80107920, object + 8);
    platform::write_u32(object as usize, value);
    platform::write_u8(object as usize + 4, 0);
    platform::write_u8(object as usize + 0x1B8, 0);
    platform::write_u32(object as usize + 0x1BC, 0);
}

/// Allocates and initialises the 0x40-byte layout object.
///
/// The object comes from `0x802B9240(0x40, heap, 4)`. When non-null it is
/// initialised by `0x80107F20(object, heap)`, and either way the result is
/// stored at `r13 - 0x53D4`.
///
/// # Original
/// - Ghidra: `FUN_80107ec0` at `0x80107EC0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80107ec0_FUN_80107ec0.c`
///
/// # Arguments
/// - `heap`: allocator passed to `0x802B9240` and to `0x80107F20`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates 0x40 bytes, optionally initialises them, and writes
/// `r13 - 0x53D4`.
///
/// # Safety
/// `heap` must be a valid Metrowerks heap handle.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_80107ec0(heap: u32) {
    let allocated = platform::call3(FUN_802B9240, 0x40, heap, 4);
    let object = if allocated != 0 {
        platform::call2(FUN_80107F20, allocated, heap)
    } else {
        0
    };
    platform::write_u32(SDA_53D4_OBJECT, object);
}

/// Creates the display record from the two halfwords in the current header.
///
/// The object is `r13 - 0x5200`, its header is the object's first word, and
/// the call is `0x802D3700(u16 header[+4], u16 header[+6], 6)`.
///
/// # Original
/// - Ghidra: `FUN_80108f10` at `0x80108F10` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80108f10_FUN_80108f10.c`
///
/// # Arguments
/// None; the source object comes from `r13 - 0x5200`.
///
/// # Returns
/// The value returned by `0x802D3700` (a handle/pointer; Ghidra prints
/// `void`, but the caller stores `r3`).
///
/// # Side effects
/// Calls `0x802D3700`.
///
/// # Safety
/// The object pointer and its header must be readable at the offsets above.
///
/// # Confidence
/// medium - the return value is recovered from the caller's store, not from
/// Ghidra's `void` prototype.
pub unsafe fn fn_80108f10() -> u32 {
    let owner = platform::read_u32(SDA_5200_OBJECT);
    let header = platform::read_u32(owner as usize);
    let first = platform::read_u16(header as usize + 4) as u32;
    let second = platform::read_u16(header as usize + 6) as u32;
    platform::call3(FUN_802D3700, first, second, 6)
}

/// Lazily creates the display record and forwards it to `0x80108F40`.
///
/// When `r13 - 0x53D0` is zero, [`fn_80108f10`] is called and its result is
/// stored in the slot. Then, if the slot is nonzero, `0x80108F40(record)` is
/// called. Ghidra's body drops the argument to `0x80108F40`; the instruction
/// stream leaves the record in `r3`.
///
/// # Original
/// - Ghidra: `FUN_80109260` at `0x80109260` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80109260_FUN_80109260.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May create and store the record at `r13 - 0x53D0` and may call
/// `0x80108F40`.
///
/// # Safety
/// The display header read by [`fn_80108f10`] must be valid; `0x80108F40`
/// must accept the record pointer.
///
/// # Confidence
/// medium - the dropped `0x80108F40` argument is recovered from `r3`.
pub unsafe fn fn_80109260() {
    if platform::read_u32(SDA_53D0_OBJECT) == 0 {
        let created = fn_80108f10();
        platform::write_u32(SDA_53D0_OBJECT, created);
    }

    let record = platform::read_u32(SDA_53D0_OBJECT);
    if record != 0 {
        platform::call1(FUN_80108F40, record);
    }
}

/// Allocates and initialises the 0x1F8-byte display manager.
///
/// The object comes from `0x802B9240(0x1F8, heap, 4)`. When non-null it is
/// initialised by `0x80109350(object, heap)`, and either way the result is
/// stored at `r13 - 0x53CC`.
///
/// # Original
/// - Ghidra: `FUN_801092f0` at `0x801092F0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801092f0_FUN_801092f0.c`
///
/// # Arguments
/// - `heap`: allocator passed to `0x802B9240` and to `0x80109350`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates 0x1F8 bytes, optionally initialises them, and writes
/// `r13 - 0x53CC`.
///
/// # Safety
/// `heap` must be a valid Metrowerks heap handle.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_801092f0(heap: u32) {
    let allocated = platform::call3(FUN_802B9240, 0x1F8, heap, 4);
    let object = if allocated != 0 {
        platform::call2(FUN_80109350, allocated, heap)
    } else {
        0
    };
    platform::write_u32(SDA_53CC_OBJECT, object);
}

// ===========================================================================
// Wave 282 lane E (`_E282`) - the three player resource-name release loops
// ===========================================================================
//
// `0x80101CE0`, `0x80101DC0` and `0x80101EA0` are the free-all counterparts of
// the three append bodies at `0x80101C60`, `0x80101D40` and `0x80101E20`
// (still untabled). Each append body allocates a copy of the name it is given,
// stores it in its word array and bumps the matching counter; each body here
// walks the same counter down and frees every stored string. The three
// (counter, array) pairs are
//
//   0x8042A20C / 0x80373D88   append 0x80101C60
//   0x8042A210 / 0x80373DC8   append 0x80101D40
//   0x8042A214 / 0x803740E8   append 0x80101E20
//
// `0x807B1C20` (the base-REL stage/player mount,
// [`crate::game::bases::fn_807b1c20`]) consumes the first two lists by
// appending their names to the resource manager at `0x8042A038 + 4`; the third
// list is not read there and its owner is `// unknown:`.

/// `r13 - 0x5474` -> `0x8042A20C`: live-entry count of the first
/// resource-name list at [`E282_NAMES_80373D88`].
///
/// observed: `lwz r0,-0x5474(r13)` / `stw r0,-0x5474(r13)` in
/// [`fn_80101ce0`], and the matching `lwz r0,-0x5474(r13)` increment in the
/// append body `0x80101C60`.
const E282_COUNT_8042A20C: usize = SDA_BASE - 0x5474;

/// `r13 - 0x5470` -> `0x8042A210`: live-entry count of the second
/// resource-name list at [`E282_NAMES_80373DC8`].
const E282_COUNT_8042A210: usize = SDA_BASE - 0x5470;

/// `r13 - 0x546C` -> `0x8042A214`: live-entry count of the third
/// resource-name list at [`E282_NAMES_803740E8`].
const E282_COUNT_8042A214: usize = SDA_BASE - 0x546C;

/// `0x80373D88`: word array of owned name strings, grown by `0x80101C60`.
///
/// observed: `lis r31,0x8037` / `addi r31,r31,0x3D88` in [`fn_80101ce0`].
const E282_NAMES_80373D88: usize = 0x8037_3D88;

/// `0x80373DC8`: word array of owned name strings, grown by `0x80101D40`.
const E282_NAMES_80373DC8: usize = 0x8037_3DC8;

/// `0x803740E8`: word array of owned name strings, grown by `0x80101E20`.
const E282_NAMES_803740E8: usize = 0x8037_40E8;

/// `0x802B9290(block)` - frees one block through the thread archive heap.
///
/// Already reconstructed as [`crate::resource::archive::fn_802b9290`]; the
/// call is made through the address because that body takes `u32`.
const E282_FN_802B9290: usize = 0x802B_9290;

/// Releases every name stored in the first player resource-name list.
///
/// The loop is a pre-decrement drain: while the counter at
/// [`E282_COUNT_8042A20C`] is non-zero, decrement it and free the string it then
/// indexes. There is no null test on the element because the append body never
/// stores a null word.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80101CE0`.
/// - Extent: `0x80101CE0..0x80101D34` (84 bytes, 20 instructions), ending at
///   the `blr` at `0x80101D30`; three zero words fill `0x80101D34..0x80101D3C`
///   and the next body starts at `0x80101D40`.
/// - Reach: no `b`, `bl` or `bc` in the executable segments names it and no
///   data word holds its address; the append body `0x80101C60` is its
///   structural counterpart.
/// - Table: none.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `lis r31,0x8037`; `addi r31,r31,15752`;
///   `b 0x80101D14`; body `lwz r3,-0x5474(r13)`; `addi r0,r3,-1`;
///   `stw r0,-0x5474(r13)`; `rlwinm r0,r0,2,0,29`; `lwzx r3,r31,r0`;
///   `bl 0x802B9290`; head `lwz r0,-0x5474(r13)`; `cmpi cr0,0,r0,0`;
///   `4082FFE0 bc 4,2` (`bne`, back to the body); epilogue;
///   `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// None; the counter and the array are absolute globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Frees one heap string per entry and leaves the counter at zero.
///
/// # Safety
/// Every word in `0x80373D88..0x80373D88+counter*4` must be a block from the
/// archive heap accepted by `0x802B9290`.
///
/// # Confidence
/// high - the counter, array base, index scale and free call are literal; the
/// "player resource name" role is `// inferred:` from
/// [`crate::game::bases::fn_807b1c20`].
pub unsafe fn fn_80101ce0() {
    loop {
        let count = platform::read_i32(E282_COUNT_8042A20C);
        if count == 0 {
            break;
        }
        let index = count - 1;
        platform::write_i32(E282_COUNT_8042A20C, index);
        let entry = platform::read_u32(E282_NAMES_80373D88 + index as usize * 4);
        platform::call1(E282_FN_802B9290, entry);
    }
}

/// Releases every name stored in the second player resource-name list.
///
/// Same pre-decrement drain as [`fn_80101ce0`], over
/// [`E282_COUNT_8042A210`] / [`E282_NAMES_80373DC8`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80101DC0`.
/// - Extent: `0x80101DC0..0x80101E14` (84 bytes, 20 instructions), ending at
///   the `blr` at `0x80101E10`; three zero words fill `0x80101E14..0x80101E1C`
///   and the next body starts at `0x80101E20`.
/// - Reach: no branch and no data word in the main image names it.
/// - Table: none.
/// - Observed: identical to [`fn_80101ce0`] with `lis r31,0x8037` /
///   `addi r31,r31,15816` (`0x80373DC8`), `-0x5470(r13)` and the same
///   `bl 0x802B9290`; `4082FFE0 bc 4,2` closes the loop.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Frees one heap string per entry and leaves the counter at zero.
///
/// # Safety
/// Every word in `0x80373DC8..0x80373DC8+counter*4` must be a live archive-heap
/// block.
///
/// # Confidence
/// high - identical shape to [`fn_80101ce0`], only the pair differs.
pub unsafe fn fn_80101dc0() {
    loop {
        let count = platform::read_i32(E282_COUNT_8042A210);
        if count == 0 {
            break;
        }
        let index = count - 1;
        platform::write_i32(E282_COUNT_8042A210, index);
        let entry = platform::read_u32(E282_NAMES_80373DC8 + index as usize * 4);
        platform::call1(E282_FN_802B9290, entry);
    }
}

/// Releases every name stored in the third player resource-name list.
///
/// Same pre-decrement drain as [`fn_80101ce0`], over
/// [`E282_COUNT_8042A214`] / [`E282_NAMES_803740E8`]. This is the list whose
/// consumer is `// unknown:`; nothing in the main image reads it.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80101EA0`.
/// - Extent: `0x80101EA0..0x80101EF4` (84 bytes, 20 instructions), ending at
///   the `blr` at `0x80101EF0`; three zero words fill `0x80101EF4..0x80101EFC`
///   and the next body starts at `0x80101F00`.
/// - Reach: no branch and no data word in the main image names it.
/// - Table: none.
/// - Observed: identical to [`fn_80101ce0`] with `lis r31,0x8037` /
///   `addi r31,r31,16616` (`0x803740E8`), `-0x546C(r13)` and the same
///   `bl 0x802B9290`; `4082FFE0 bc 4,2` closes the loop.
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Frees one heap string per entry and leaves the counter at zero.
///
/// # Safety
/// Every word in `0x803740E8..0x803740E8+counter*4` must be a live archive-heap
/// block.
///
/// # Confidence
/// high - identical shape to [`fn_80101ce0`], only the pair differs.
pub unsafe fn fn_80101ea0() {
    loop {
        let count = platform::read_i32(E282_COUNT_8042A214);
        if count == 0 {
            break;
        }
        let index = count - 1;
        platform::write_i32(E282_COUNT_8042A214, index);
        let entry = platform::read_u32(E282_NAMES_803740E8 + index as usize * 4);
        platform::call1(E282_FN_802B9290, entry);
    }
}

// ===========================================================================
// Wave 291 lane E (`_E291`) - one scene-chain walk and the first two
// resource-name append bodies
// ===========================================================================
//
// Re-picked by `work/wave291/e_pick.mjs` from `work/wave266/e_candidates.tsv`
// after the 247 covered candidates and the three deferred addresses were
// removed. All three are 116 bytes, start with `9421xxxx` (`stwu r1,-N(r1)`),
// reproduce their extent with `bad = 0`, end on one `blr`, have no overlap and
// are not pointer-run interiors.
//
// The two append bodies are the exact writers of the counters and arrays that
// [`fn_80101ce0`] and [`fn_80101dc0`] drain; the third list's append body at
// `0x80101E20` stays untabled. `0x80100430` is the neighbour of
// [`fn_801002f0`]/[`fn_80100530`]/[`fn_801005c0`] on the same `0x338C` course
// row.

/// `0x800F0F30` - binds/keys the name at a `+0x1A0` record. Its guest return
/// is the `0x80164F00` lookup answer, needed by [`fn_80100430`]'s loop.
///
/// The dSys body models the same call but declares no return value, so the
/// value is taken through the address exactly as the existing
/// `FN_800F0F30_C` call site in `engine::dsys` does.
const W291_FN_800F0F30: usize = 0x800F_0F30;

/// `0x800DFCE0` - resolves the next `+0x1A8` record against the previous
/// answer. The display body also models no return value.
const W291_FN_800DFCE0: usize = 0x800D_FCE0;

// ---------------------------------------------------------------------------
// 0x80100430 - count the `+0x1A8` chain of the selected course row
// ---------------------------------------------------------------------------

/// Walks the selected course row's `+0x1A8` chain and returns how many links
/// were taken.
///
/// The first probe targets `+0x1A0` with the caller's second argument; the
/// loop then, while the last answer is non-negative, resolves `+0x1A8`
/// against that answer and counts the step.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80100430`.
/// - Extent: `0x80100430..0x801004A4` (116 bytes, 29 words), ending at the
///   `blr` at `0x801004A0`; one zero word fills `0x801004A4..0x801004A8`.
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x8076FCA8`
///   and `0x8076FCB8`.
/// - Table: none.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `stw r30,8(r1)`; `mr r30,r3`;
///   `lwz r0,0x338C(r3)`; `mulli r0,r0,0xBF8`; `add r3,r3,r0`;
///   `addi r3,r3,0x1A0`; `bl 0x800F0F30`; `li r31,0`; `b 0x80100480`;
///   body `lwz r0,0x338C(r30)`; `mr r4,r3`; `addi r31,r31,1`;
///   `mulli r0,r0,0xBF8`; `add r3,r30,r0`; `addi r3,r3,0x1A8`;
///   `bl 0x800DFCE0`; head `cmpi cr0,r3,0`; `4080FFE0 bc 4,0`;
///   `mr r3,r31`; epilogue; `4E800020 blr`.
/// - Polarity: `4080FFE0` is `bc 4,0` (`bf cr0.lt`), i.e. the loop repeats
///   while the signed answer is **not** negative (`>= 0`).
/// - Decoder note: `mulli` is the `0xBF8` record stride; the two `addi` words
///   use the same selected row each iteration, so the row index is re-read
///   instead of being cached.
///
/// # Arguments
/// * `scene` - course/scene object with the selected row at `+0x338C`.
/// * `arg` - second argument forwarded to the first `0x800F0F30` probe.
///
/// # Returns
/// The number of `0x800DFCE0` steps taken before a negative answer.
///
/// # Side effects
/// One first probe plus the chain resolutions.
///
/// # Safety
/// `scene + 0x338C` must be readable and the selected row must contain
/// readable `+0x1A0`/`+0x1A8` records.
///
/// # Confidence
/// medium - the stride, the two offsets, the loop guard and the counter are
/// literal; the two callees are the same functions `engine::dsys` and
/// `engine::display` already model, and the chain's semantic name is inferred
/// from the row use in [`fn_801005c0`].
pub unsafe fn fn_80100430(scene: u32, arg: u32) -> u32 {
    let row = platform::read_i32(scene as usize + 0x338C);
    let first = (scene as isize + row as isize * 0xBF8 + 0x1A0) as u32;
    let mut answer = platform::call2(W291_FN_800F0F30, first, arg);

    let mut count: u32 = 0;
    while (answer as i32) >= 0 {
        let row = platform::read_i32(scene as usize + 0x338C);
        let record = (scene as isize + row as isize * 0xBF8 + 0x1A8) as u32;
        answer = platform::call2(W291_FN_800DFCE0, record, answer);
        count += 1;
    }
    count
}

// ---------------------------------------------------------------------------
// 0x80101C60 - append a name to the first resource-name list
// ---------------------------------------------------------------------------

/// Copies `name` into the first resource-name list and returns the previous
/// entry count.
///
/// The body reads the live count, allocates `strlen(name) + 1` bytes, stores
/// the copy at `0x80373D88[count]`, copies the string, then increments
/// `0x8042A20C`. The counter/array pair is the one [`fn_80101ce0`] drains.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80101C60`.
/// - Extent: `0x80101C60..0x80101CD4` (116 bytes, 29 words), ending at the
///   `blr` at `0x80101CD0`; one zero word fills `0x80101CD4..0x80101CD8`.
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x807B25D4`,
///   `0x807B25F8`, `0x807B261C` and `0x807B2640`.
/// - Table: the `0x80373D88` word array, indexed by `0x8042A20C`.
/// - Observed: `9421FFE0 stwu r1,-0x20(r1)`; `mflr r0`; `stw r0,0x24(r1)`;
///   `stw r31,0x1C(r1)`; `lis r31,0x8037`; `addi r31,r31,15752`
///   (`0x80373D88`); `stw r30,0x18(r1)`; `stw r29,0x14(r1)`; `mr r29,r3`;
///   `lwz r0,r13,-21620` (`0x8042A20C`); `rlwinm r30,r0,2,0,29`;
///   `bl 0x802DC69C`; `addi r3,r3,1`; `rlwinm r0,r3,1,0,30`;
///   `subf r3,r3,r0`; `bl 0x802B9250`; `stwx r3,r31,r30`;
///   `mr r4,r29`; `bl 0x802E1938`; `lwz r3,r13,-21620`;
///   `addi r0,r3,1`; `stw r0,r13,-21620`; epilogue; `4E800020 blr`.
/// - Polarity: no branch.
/// - Decoder note: `lis r31,0x8037` + `addi r31,r31,15752` is positive, so it
///   folds to `0x80373D88`. The `rlwinm 1,0,30` + `subf` pair is the
///   `2 * (len + 1) - (len + 1)` identity, i.e. the `len + 1` allocation size.
///
/// # Arguments
/// * `name` - NUL-terminated source string (`r3`/`r29`).
///
/// # Returns
/// The entry count **before** the append, because the `lwz r3` that reloads
/// the counter is the last write to `r3`.
///
/// # Side effects
/// One allocation, one array store, one string copy and one counter store.
///
/// # Safety
/// `name` must be NUL-terminated and the counter must stay below the array's
/// capacity.
///
/// # Confidence
/// high - the counter, the array fold, the allocation size and the copy are
/// literal; the list identity is fixed by [`fn_80101ce0`].
pub unsafe fn fn_80101c60(name: u32) -> u32 {
    let index = platform::read_u32(E282_COUNT_8042A20C);
    let length = platform::call1(FUN_802DC69C, name);
    let size = length.wrapping_add(1);
    let copy = crate::resource::archive::fn_802b9250(size);
    platform::write_u32(E282_NAMES_80373D88 + index as usize * 4, copy);
    crate::engine::main_loop::fn_802e1938(copy as usize, name as usize);
    let count = platform::read_u32(E282_COUNT_8042A20C);
    platform::write_u32(E282_COUNT_8042A20C, count.wrapping_add(1));
    count
}

// ---------------------------------------------------------------------------
// 0x80101D40 - append a name to the second resource-name list
// ---------------------------------------------------------------------------

/// The second-list twin of [`fn_80101c60`]: the copy lands at
/// `0x80373DC8[count]` and the counter is `0x8042A210`. The pair is drained
/// by [`fn_80101dc0`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80101D40`.
/// - Extent: `0x80101D40..0x80101DB4` (116 bytes, 29 words), ending at the
///   `blr` at `0x80101DB0`; one zero word fills `0x80101DB4..0x80101DB8`.
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x807B2698`
///   and `0x807B26CC`.
/// - Table: the `0x80373DC8` word array, indexed by `0x8042A210`.
/// - Observed: the same prologue; `lis r31,0x8037`; `addi r31,r31,15816`
///   (`0x80373DC8`); `lwz r0,r13,-21616` (`0x8042A210`);
///   `rlwinm r30,r0,2,0,29`; `bl 0x802DC69C`; `addi r3,r3,1`;
///   `rlwinm r0,r3,1,0,30`; `subf r3,r3,r0`; `bl 0x802B9250`;
///   `stwx r3,r31,r30`; `mr r4,r29`; `bl 0x802E1938`;
///   `lwz r3,r13,-21616`; `addi r0,r3,1`; `stw r0,r13,-21616`;
///   epilogue; `4E800020 blr`.
/// - Polarity: no branch.
/// - Decoder note: `addi r31,r31,15816` is `0x3DC8`, so the base is
///   `0x80373DC8`; altogether the body is byte-for-byte the first-list append
///   with the two SDA/array words changed.
///
/// # Arguments
/// * `name` - NUL-terminated source string (`r3`/`r29`).
///
/// # Returns
/// The entry count before the append.
///
/// # Side effects
/// One allocation, one array store, one string copy and one counter store.
///
/// # Safety
/// `name` must be NUL-terminated and the counter must stay below the array's
/// capacity.
///
/// # Confidence
/// high - the only differences from [`fn_80101c60`] are the documented
/// counter/array addresses.
pub unsafe fn fn_80101d40(name: u32) -> u32 {
    let index = platform::read_u32(E282_COUNT_8042A210);
    let length = platform::call1(FUN_802DC69C, name);
    let size = length.wrapping_add(1);
    let copy = crate::resource::archive::fn_802b9250(size);
    platform::write_u32(E282_NAMES_80373DC8 + index as usize * 4, copy);
    crate::engine::main_loop::fn_802e1938(copy as usize, name as usize);
    let count = platform::read_u32(E282_COUNT_8042A210);
    platform::write_u32(E282_COUNT_8042A210, count.wrapping_add(1));
    count
}

// ===========================================================================
// Wave 292 lane E (`_E292`) - the third resource-name append and the counted
// scene-chain walk
// ===========================================================================
//
// Re-picked by `work/wave292/e_pick.mjs` from `work/wave266/e_candidates.tsv`
// after the 259 candidates covered by waves 266..291 and the three deferred
// addresses were removed. Both bodies are 116 bytes, start with `9421xxxx`
// (`stwu r1,-N(r1)`), reproduce their extent with `bad = 0`, end on exactly
// one `blr`, do not overlap an extent and are not pointer-run interiors.
//
// `0x80101E20` is the writer of the third counter/array pair that
// [`fn_80101ea0`] drains, completing the append/drain family of
// `0x80101C60`/`0x80101D40`. `0x801004B0` is [`fn_80100430`]'s counted twin
// on the same `0x338C` course row.

// ---------------------------------------------------------------------------
// 0x80101E20 - append a name to the third resource-name list
// ---------------------------------------------------------------------------

/// Copies `name` into the third resource-name list and returns the previous
/// entry count.
///
/// The body reads the live count, allocates `strlen(name) + 1` bytes, stores
/// the copy at `0x803740E8[count]`, copies the string, then increments
/// `0x8042A214`. The pair is drained by [`fn_80101ea0`], which owns exactly
/// this array and counter.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x80101E20`.
/// - Extent: `0x80101E20..0x80101E94` (116 bytes, 29 words), ending at the
///   `blr` at `0x80101E90`; one zero word fills `0x80101E94..0x80101E98`.
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x807B6DA0`.
/// - Table: the `0x803740E8` word array, indexed by `0x8042A214`.
/// - Observed: `9421FFE0 stwu r1,-0x20(r1)`; `mflr r0`; `stw r0,0x24(r1)`;
///   `stw r31,0x1C(r1)`; `lis r31,0x8037`; `addi r31,r31,16616`
///   (`0x803740E8`); `stw r30,0x18(r1)`; `stw r29,0x14(r1)`; `mr r29,r3`;
///   `lwz r0,r13,-21612` (`0x8042A214`); `rlwinm r30,r0,2,0,29`;
///   `bl 0x802DC69C`; `addi r3,r3,1`; `rlwinm r0,r3,1,0,30`;
///   `subf r3,r3,r0`; `bl 0x802B9250`; `stwx r3,r31,r30`;
///   `mr r4,r29`; `bl 0x802E1938`; `lwz r3,r13,-21612`;
///   `addi r0,r3,1`; `stw r0,r13,-21612`; epilogue; `4E800020 blr`.
/// - Polarity: no branch.
/// - Decoder note: `addi r31,r31,16616` is `0x40E8`, so the array base is
///   `0x803740E8`; the body is byte-for-byte [`fn_80101c60`] with only the
///   array and counter words changed.
///
/// # Arguments
/// * `name` - NUL-terminated source string (`r3`/`r29`).
///
/// # Returns
/// The entry count **before** the append (`r3` is reloaded from the counter
/// after the copy).
///
/// # Side effects
/// One allocation, one array store, one string copy and one counter store.
///
/// # Safety
/// `name` must be NUL-terminated and the counter must stay below the array's
/// capacity.
///
/// # Confidence
/// high - the counter, the array fold, the allocation size and the copy are
/// literal; the list identity is fixed by [`fn_80101ea0`].
pub unsafe fn fn_80101e20(name: u32) -> u32 {
    let index = platform::read_u32(E282_COUNT_8042A214);
    let length = platform::call1(FUN_802DC69C, name);
    let size = length.wrapping_add(1);
    let copy = crate::resource::archive::fn_802b9250(size);
    platform::write_u32(E282_NAMES_803740E8 + index as usize * 4, copy);
    crate::engine::main_loop::fn_802e1938(copy as usize, name as usize);
    let count = platform::read_u32(E282_COUNT_8042A214);
    platform::write_u32(E282_COUNT_8042A214, count.wrapping_add(1));
    count
}

// ---------------------------------------------------------------------------
// 0x801004B0 - walk at most `count` links of the selected course row's
// `+0x1A8` chain
// ---------------------------------------------------------------------------

/// Counted twin of [`fn_80100430`]: probes the row's `+0x1A0` record, then
/// resolves `+0x1A8` against the previous answer at most `count` times.
///
/// The loop stops when the last answer is negative **or** the remaining count
/// reaches zero; each iteration decrements the counter after the resolve,
/// exactly as the `addi r31,r31,-1` before the loop head does.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x801004B0`.
/// - Extent: `0x801004B0..0x80100528` (120 bytes, 30 words), ending at the
///   `blr` at `0x80100524`; the next body starts immediately at
///   `0x80100530` (`fn_80100530`).
/// - Reach: no `b` and no data word; `d_basesNP` calls it from `0x8076A968`,
///   `0x8076FDE8` and `0x8076FF4C`.
/// - Table: none.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r5`; `stw r30,8(r1)`; `mr r30,r3`;
///   `lwz r0,0x338C(r3)`; `mulli r0,r0,0xBF8`; `add r3,r3,r0`;
///   `addi r3,r3,0x1A0`; `bl 0x800F0F30`; `b 0x80100500`;
///   loop body `lwz r0,0x338C(r30)`; `mr r4,r3`; `mulli r0,r0,0xBF8`;
///   `add r3,r30,r0`; `addi r3,r3,0x1A8`; `bl 0x800DFCE0`;
///   `addi r31,r31,-1`; head `cmpi cr0,r3,0`; `4180000C bc 12,0`
///   (`bt cr0.lt`, exit when the answer is negative); `cmpi cr0,r31,0`;
///   `4181FFD8 bc 12,1` (`bt cr0.gt`, repeat while the count is positive);
///   epilogue; `4E800020 blr`.
/// - Polarity: `4180000C` is `bc 12,0` (`bt cr0.lt`) so a negative answer
///   leaves the loop; `4181FFD8` is `bc 12,1` (`bt cr0.gt`) so the loop
///   repeats only while the signed counter is **greater than zero**.
/// - Decoder note: `mulli` is the same `0xBF8` row stride as
///   [`fn_80100430`]; the row index is re-read on every iteration instead of
///   being cached. `4E800421` does not occur; both callees are reached by
///   direct `bl`.
///
/// # Arguments
/// * `scene` - course/scene object with the selected row at `+0x338C`.
/// * `arg` - second argument forwarded to the first `0x800F0F30` probe.
/// * `count` - signed iteration cap; the loop runs at most this many
///   resolves. Zero or negative skips the loop after the first probe.
///
/// # Returns
/// The last `r3` produced by the probe/resolve pair (negative when the chain
/// ended, otherwise the last accepted answer).
///
/// # Side effects
/// One first probe plus at most `count` chain resolutions.
///
/// # Safety
/// `scene + 0x338C` must be readable and the selected row must contain
/// readable `+0x1A0`/`+0x1A8` records.
///
/// # Confidence
/// high - the stride, offsets, two guards and the post-resolve decrement are
/// literal; both callees are the ones [`fn_80100430`] already documents.
pub unsafe fn fn_801004b0(scene: u32, arg: u32, count: i32) -> u32 {
    let row = platform::read_i32(scene as usize + 0x338C);
    let first = (scene as isize + row as isize * 0xBF8 + 0x1A0) as u32;
    let mut answer = platform::call2(W291_FN_800F0F30, first, arg);
    let mut remaining = count;

    loop {
        if (answer as i32) < 0 || remaining <= 0 {
            break;
        }
        let row = platform::read_i32(scene as usize + 0x338C);
        let record = (scene as isize + row as isize * 0xBF8 + 0x1A8) as u32;
        answer = platform::call2(W291_FN_800DFCE0, record, answer);
        remaining -= 1;
    }
    answer
}
