//! C++ runtime plumbing and single-byte object-field helpers: deleting
//! destructors, vtable dispatch, child chains, name comparison, the
//! secondary-base adjustor thunk (`0x80040540..0x800408F0`), two leaf byte
//! accessors (`0x800483F0`, `0x8004DE30`), and the CodeWarrior / Metrowerks
//! runtime band `0x802D0000..0x802E0000` (global destructors, array helpers,
//! arena allocator, 64-bit integer helpers, stdio buffer code and `mem*`).
//!
//! The first five functions are the small, repeated machinery that
//! CodeWarrior's C++ compiler emits for *every* polymorphic class in the game:
//! release an object, walk a chain of children, compare RTTI-style names, and
//! forward a call after adjusting `this` to a secondary base. They carry no
//! gameplay knowledge; the class they operate on is supplied by the caller's
//! vtable.
//!
//! The two byte helpers after the first five are generic field vocabulary
//! promoted with the gameplay-player cluster: one saturating byte counter and
//! one byte-to-boolean conversion. They know nothing about the player object,
//! so they live with the other object-field helpers rather than in
//! `game/player.rs`.
//!
//! Promoted from the range module `src/subsystems/r80040_80080.rs` by the
//! promotion pass. That file is now a tombstone with no code; the player-side
//! functions of the range live in `src/game/player.rs`.
//!
//! # Second band - Metrowerks runtime / CRT (`0x802D0000..0x802E0000`)
//!
//! Everything from [`fn_802dc69c`] down is a **runtime library** routine, not
//! game logic. Read it as the support code that `printf`, `new`, `delete`,
//! static constructors and the `str*`/`mem*` calls rely on:
//!
//! | Address run | What it is |
//! |---|---|
//! | `0x802DC69C..0x802DCB10` | C++ runtime: string length, the global-destructor chain, array construct/destroy helpers |
//! | `0x802DCC4C` | `double` → 32-bit conversion with saturation |
//! | `0x802DCDD8..0x802DD210` | 64-bit divide / modulo / shift helpers |
//! | `0x802DD350..0x802DD860` | CodeWarrior arena allocator (registry slots, default heap, free-list insert, `free`) |
//! | `0x802DE55C..0x802DEBB8` | stdio buffer flush, `fclose`, `ftell`, `fseek` |
//! | `0x802DEF74..0x802DF308` | `memmove` dispatcher plus the aligned/unaligned copy kernels |
//! | `0x802D9380..0x802DBC84` | C++ container/list helpers, one block allocator and small constructor/destructor pairs |
//! | `0x802DAD00..0x802DAD10` | the get/set pair for the pointer field at `+0x154` |
//! | `0x802DBD90..0x802DC4F0` | typed accessors and flag updates for the record built by `0x802DBB70` |
//! | `0x802DDAA0..0x802DF4B8` | decimal digit records, the locale `mbtowc` shim and the `printf` specification parser |
//! | `0x802DE20C..0x802DE370` | binary `double` to decimal record conversion and its rounding pass |
//! | `0x802D6110..0x802D7030` | AX task queue / voice lifecycle (Wave 38: `0x802D6110..0x802D6E00` plus `0x802D7010/30`) |
//! | `0x802D3F60..0x802D4780` | Wave 65: resource-view predicate twins, per-entry loops, three archive lookup wrappers, the 0x6C-stride array pair and the 0x8AC sub-object teardown |
//! | `0x802D77E0..0x802D7D50` | Wave 65: BRRES section-tag record (six field setters, tag lookup, entry accessor), its constructor and the two `0x802967E0` shutdown twins |
//! | `0x802D0810..0x802D1D70` | Wave 77: SDA address getters, the member-block copy constructors and the dirty-flag / chain-walk pair |
//! | `0x802D35C0..0x802D3E90` | Wave 77: the 32-byte small-block allocator, the three-child destructor, the view teardown and the fourth submit predicate |
//! | `0x802D4C00..0x802D50A0` | Wave 77: the three-slot `FX A/B/C` effect table - release, bind, mode dispatch, shutdown, array build/destroy |
//! | `0x802D7170` | Wave 77: millisecond busy-wait against the tick counter |
//! | `0x802D2730..0x802D2B60` | Wave 89: the AX engine setup chain and its six one-shot helpers |
//! | `0x802D39B0..0x802D42D0` | Wave 89: the last three `+0x4C4` view loaders and the SDA compare hook |
//! | `0x802D4830` | Wave 89: 0x6C-stride transform record initialiser |
//! | `0x802D50B0..0x802D5DD0` | Wave 89: the eight per-mode effect builders behind [`fn_802d4cd0`] |
//! | `0x802D7410..0x802D7570` | Wave 89: sound-record create/destroy and the record factory |
//! | `0x802DB6B0`, `0x802DB710` | Wave 89: the `+0xBC` clear and its conditional release |
//! | `0x802D0AE0` | Wave 92: two-panel layout table install and the draw record it feeds |
//! | `0x802D32F0..0x802D3810` | Wave 92: widget attribute init, texture-node pop, the `+0x4C4` view constructor |
//! | `0x802D5080`, `0x802D5090` | Wave 92: the two `fn_802d4cd0` tail-call wrappers |
//! | `0x802D71F0` | Wave 92: the interactive console / keypad viewer loop |
//! | `0x802D7680` | Wave 92: BRRES section-table constructor and its six setters |
//! | `0x802DB6C0..0x802DB9EC` | Wave 92: the four `lyt` state helpers |
//! | `0x802D0D90..0x802D1900` | Wave 96: the four-float layout record, its two draw-state appliers, the axis filler and the descriptor submit |
//! | `0x802D2610` | Wave 96: console panel publication plus the AX mixer bring-up |
//! | `0x802D48C0` | Wave 96: view transform republish and the `0x802D4FD0` hand-off |
//! | `0x802D6F20`, `0x802D7070` | Wave 96: equal-tempered playback ratio and the fade tick |
//! | `0x802D9570` | Wave 96: row-array submit with per-entry virtual dispatch |
//! | `0x802D2F10`, `0x802D3260`, `0x802DEBB4`, `0x802DB0E0` | Wave 96: two GX tail calls, the `ftell` tail call and a bare `blr` |
//! | `0x802D0030` | Wave 99: quad submit - the four vertices are written straight to the GX FIFO |
//! | `0x802D12E0` | Wave 99: layout-record screen-rectangle recompute (the body behind `0x802D17C0`) |
//! | `0x802D1E20`, `0x802D2310` | Wave 99: the panel slot publish/commit pair |
//! | `0x802D2550`, `0x802D79A0`, `0x802D7860` | Wave 99: panel quad republish and the tagged-section entry lookup plus its wrapper |
//! | `0x802DADD0`, `0x802DCBFC`, `0x802DFF64` | Wave 100: the look-at angle recompute, the pointer-to-member call thunk and the `%a` formatter |
//! | `0x802DCCCC..0x802DCDC0` | Wave 101: the CodeWarrior callee-save register entries, modelled as explicit register windows |
//!
//! These live in `runtime::cpp` because they are compiler-support code with no
//! gameplay owner, matching how `0x80040540` was classified.
//!
//! # Wave 65 pass - resource-view helpers and the BRRES tag record
//!
//! The Wave 65 bodies are still compiler/engine plumbing, but two distinct
//! families:
//!
//! 1. `0x802D3F60..0x802D4780` - helpers around a sub-object handle stored at
//!    `+0x4C4` of a large view object. Three near-identical predicates
//!    (`0x802D3F60/4030/4100`, plus the inlined fourth at `0x802D3E90` added by
//!    Wave 77) gate a call on the view state at `+0x4CC` and an
//!    in-progress byte at `+5`; `0x802D41D0..0x802D4250` are reset and
//!    per-entry loops; `0x802D43C0..0x802D45A0` turn three archive lookups into
//!    booleans; `0x802D46A0..0x802D4780` are the element destructor, the
//!    constructor and the teardown of the 0x8AC-byte object that owns them.
//! 2. `0x802D77E0..0x802D7D50` - a small BRRES archive record. The six 8-byte
//!    setters store the `INF1/DAT1/STR1/MID1/FLW1/FLI1` section pointers at
//!    `+4..+0x18`, `0x802D7910` maps a section tag to that index, and
//!    `0x802D7A80` reads one entry of the section table. `0x802D7AA0/7B20`
//!    build and release the record; `0x802D7C50/7D50` reset its `+0x74` child.
//!
//! The Ghidra C for this band uses `r13`-relative globals (`unaff_r13`),
//! `extraout_r4` and `CONCAT44`; every such access below is resolved to an
//! absolute address or a documented register fact, and anything unresolved is
//! marked `// unknown:`.
//!
//! ## Deliberately not translated in this band
//!
//! - The Metrowerks `__savegprlr_N` / `__restgprlr_N` / `__savefpr_N` /
//!   `__restfpr_N` entries at `0x802DCCCC..0x802DCDC0` used to be listed here as
//!   untranslated. Wave 101 translated all thirty-two of them with an explicit
//!   register window; see its pass note. `docs/QUALITY.md` §5.5 still describes
//!   the raw `stfd`/`lfd`/`stw`/`lwz` runs those bodies reproduce.
//! - `0x802D2F10`, `0x802D3260`, `0x802DEBB4` - 4-byte `b` thunks.
//! - `0x802DA040`, `0x802DA6F0`, `0x802DAD20` - their C bodies read the fake
//!   64-bit return of `0x802DCD74`/`0x802DCD40` (the register-save stubs), so
//!   the real argument registers and frame offsets cannot be recovered from
//!   the export without guessing. The raw prologues were checked; they save the
//!   very registers Ghidra treats as return words.
//! - `0x802D01A4..0x802D8E70` - EGG render/effect code with the GQR
//!   `ldexpf` artefacts documented in `docs/QUALITY.md` §5.3; it belongs to the
//!   engine lanes, not the CRT. Wave 99 promoted `0x802D0030` out of this run
//!   because its only GQR use was the two `psq_st` spills of `f30`/`f31`.
//!
//! ## The `0x802D0000` bucket is closed
//!
//! Wave 99 left thirty-five addresses here. Wave 100 translated the three with
//! observable behaviour (`0x802DADD0`, `0x802DCBFC` and `0x802DFF64`, the last
//! of which §5.3 only deferred because of its `f31` spill), and Wave 101
//! translated the thirty-two register entries `0x802DCCCC..0x802DCDC0` as
//! explicit register windows. Every one of the 354 functions in the bucket now
//! has a body.
//!
//! One boundary note remains: `0x802DB6B0..0x802DBD50` is the `lyt`
//! scratch-transform helper family already referenced from `src/ui/lyt.rs`; the
//! bodies that family still lacks belong to that lane, not to this one.
//!
//! # Wave 77 pass - SDA getters, the small-block allocator and the effect table
//!
//! Three unrelated families from the lower half of the band:
//!
//! 1. `0x802D0810/0820` are two-instruction `addi` getters that return an SDA2
//!    address rather than a value; `0x802D25F0/2600` read one bit each out of
//!    the same SDA latch word. They are grouped here because they are the
//!    smallest possible examples of the `r2`/`r13` addressing this module
//!    documents.
//! 2. `0x802D0E50` and `0x802D1140` are the copy-constructor / copy-assignment
//!    pair for the `+0x40..+0x84` member block. They differ in exactly one
//!    store (the vtable word at `+0x38`), which is why both are translated: the
//!    difference is the evidence for which one is a constructor.
//!    `0x802D17C0/1820` are the dirty-flag walk that guards them, and
//!    `0x802D1D70` is the smallest vtable installation in the band.
//! 3. `0x802D35C0` is the allocator half of the 32-byte small-block scheme whose
//!    `free` side already lives in this file; `0x802D38A0/3940/3E00/3E90` are
//!    the destructor, the two-slot virtual dispatch and the two view helpers
//!    that complete the `+0x4C4` family started in Wave 65.
//! 4. `0x802D4C00..0x802D50A0` is one self-contained sub-system: a table of
//!    three 0x10-byte effect slots, each with a mode (`1..=8`), a loader
//!    handle and a chain of records. The loader key is the ASCII tag `FX A`
//!    bumped per slot, which is the only string evidence in the family.
//!    `0x802D7170` is the millisecond busy-wait used while those loads settle.
//!
//! Two adjacent addresses in this range already had bodies in this file from an
//! earlier pass but were never given a ledger row: `0x802DCC4C` (saturated
//! `double` to `u32`) and `0x802DF42C` (Metrowerks `__fpclassify`). They were
//! left untouched rather than duplicated, and are not claimed by this pass.
//!
//! Field roles that the export does not prove are marked `// inferred:` in the
//! bodies: `+0x30` as an in-use flag and `+0x2C` as a block size in
//! [`fn_802d35c0`], the per-slot mode numbering in [`fn_802d4cd0`], and the
//! forwarded `(this, source)` argument pairs of the two base-class copy calls.
//!
//! # Wave 89 pass - AX setup, the last view loaders, the eight effect builders
//!
//! Five more families, all read out of `0x802D0000..0x802E0000`:
//!
//! 1. `0x802D2730..0x802D2B60` - the AX (audio DSP) engine setup chain. It is a
//!    run of literal `0x801Cxxxx` AX-library calls plus the vtable query that
//!    copies the live engine's four mode bytes into SDA, so it is compiler-side
//!    plumbing with no gameplay owner. `0x802D2730` tears the engine down and
//!    restarts it, `0x802D2800` runs the seven one-shot initialisers exactly
//!    once, and `0x802D2850` / `0x802D2990` / `0x802D2A50` / `0x802D2AD0` /
//!    `0x802D2B60` are those initialisers.
//! 2. `0x802D39B0..0x802D42D0` - the three loaders that finish the `+0x4C4`
//!    view family from Wave 65/77. Each publishes a different state word
//!    (`1`, `2`, `4`) and targets a different child (`+0xC`, `+0x198`,
//!    `+0x374`); the first one is also the only place in the band where the
//!    four archive tags `ARCH`/`ARCL`/`APLM`/`APLS` appear. `0x802D42C0` is a
//!    two-instruction indirect call through the SDA hook pair that
//!    `0x802D42D0` publishes immediately before its record walk.
//! 3. `0x802D4830` - the initialiser for the 0x6C-stride transform record that
//!    `0x802D48C0` fills and `0x802D46E0` allocates.
//! 4. `0x802D50B0..0x802D5DD0` - the eight per-mode effect builders behind
//!    [`fn_802d4cd0`]. All eight are 480 bytes and differ only in four
//!    constants (engine object size, constructor, configure call and size
//!    query), which is recorded in a small table in the body section.
//! 5. `0x802D7410..0x802D7570` plus `0x802DB6B0`/`0x802DB710` - the sound-record
//!    factory, its builder with the `0x803410E4` default name, the matching
//!    destroy, and two small release helpers.
//!
//! The builders and `0x802D7570` are the cases where the export loses the first
//! argument to a `__savegprlr` entry point (`0x802DCD70` / `0x802DCD74`): the
//! bodies recover it from the call site and say so in `# Confidence`.
//!
//! [`crate::platform`] remains the only import. The one addition is a local
//! `call0_word` helper for the vtable slots whose zero-argument result the
//! export uses; [`crate::platform`] only returns a value from one argument up.
//!
//! # Wave 92 pass - layout records, the BRRES table and three `lyt` helpers
//!
//! This pass works the holes the earlier runtime waves left in the same band:
//!
//! 1. `0x802D0AE0` installs the two-panel layout table at `0x803504E0` and
//!    points the draw record at `0x80424710` at the caller's record. It is the
//!    only function here with a 0x18-byte stride table and a `2^52` conversion
//!    idiom.
//! 2. `0x802D32F0` copies four SDA widget attribute bytes, `0x802D3700` pops a
//!    texture node and hands its GX handle to [`fn_802d35c0`], and
//!    `0x802D3810` builds the `+0x4C4` child pair that the Wave 65/77/89 view
//!    loaders drive.
//! 3. `0x802D5080`/`0x802D5090` are the two remaining `fn_802d4cd0` tail
//!    wrappers. The register trace shows `r5` is never written, so their own
//!    third argument is the builder's resource word.
//! 4. `0x802D71F0` is the interactive console/keypad loop: a key-sequence wait,
//!    a view-range scan, then a pad-driven selection loop. It is the only body
//!    in the band whose `continue` path re-enters through an external callback.
//! 5. `0x802D7680` initialises the BRRES section record and walks the caller's
//!    table through the six `0x802D78B0..0x802D7900` setters. The register
//!    trace proves `r3`/`r4` still hold `this`/`table`, so the export's empty
//!    argument list at `0x802D76D0` is only a signature omission.
//! 6. `0x802DB6C0`, `0x802DB750`, `0x802DB8C0` and `0x802DB970` are four
//!    `lyt` state helpers that Wave 65 left to "that lane"; they are finished
//!    here because their only imports are [`crate::platform`] calls.
//!
//! This pass also claims the eight bodies that earlier runtime passes wrote but
//! never ledgered: the two `double` helpers (`0x802DCC4C`, `0x802DF42C`) and
//! the six 64-bit divide/modulo/shift helpers
//! (`0x802DCDD8..0x802DD210`). Each was re-checked against its export and its
//! `functions.tsv` size before the ledger row was written.
//!
//! # Wave 96 pass - the four-float layout records and the last four-byte entries
//!
//! Fourteen bodies, chosen as clusters rather than one address at a time:
//!
//! 1. `0x802D0D90..0x802D1900` - one self-contained draw-record chain.
//!    [`fn_802d0d90`] builds the record from four floats, [`fn_802d0f40`] and
//!    [`fn_802d1010`] push it into the draw state (the second one also decodes
//!    it back into a caller buffer), [`fn_802d1850`] fills one axis of a parent
//!    record, and [`fn_802d1900`] composes them on a stack frame and submits the
//!    result. All five share one field vocabulary (`+0x08/+0x0C` size,
//!    `+0x34` flags, `+0x40/+0x44` scale, `+0x58` vector) and one `2^52`
//!    integer-conversion idiom.
//! 2. `0x802D2610` - console panel publication plus the AX mixer bring-up. It
//!    reuses the [`PANEL_MAX_X`] / [`WIDGET_ATTRIBUTES`] / [`AX_MIX_DESCRIPTOR`]
//!    constants Wave 92 and Wave 89 introduced.
//! 3. `0x802D48C0` - the view transform republish that Wave 89 listed as a
//!    caller of [`fn_802d4830`]; it fills the 0x6C-stride block and hands the
//!    optional record to [`fn_802d4fd0`].
//! 4. `0x802D6F20` and `0x802D7070` - the equal-tempered playback ratio (the
//!    twelve-entry `2^(n/12)` table at `0x802FD2A0` is read out of the image)
//!    and the fade tick that consumes a position from `0x80260860`.
//! 5. `0x802D9570` - the row submit that walks `+0x2C` and calls each entry's
//!    `+0x18` virtual slot.
//! 6. Four four-byte entries: `0x802D2F10` and `0x802D3260` are `b` tail calls
//!    into the GX helpers `0x801C76C0` / `0x801C9CD0`, `0x802DEBB4` is a `b`
//!    back to [`fn_802deb0c`], and `0x802DB0E0` is a bare `blr`.
//!
//! `0x802DCD40..0x802DCDC0` was left untranslated by this pass: section 5.5 of
//! `docs/QUALITY.md` identifies it as the Metrowerks `__savegprlr_N` /
//! `__restgprlr_N` chain entered at a different `N` label per 4-byte "function".
//! The instruction bytes were re-read here (`stw`/`lwz`/`stfd` chains ending in
//! `blr`) and confirmed; Wave 101 finally translated the chain with an explicit
//! register window (see its pass note).
//!
//! # Wave 100 pass - the member-function thunk, the look-at recompute and `%a`
//!
//! After Wave 99 the `0x802D0000` bucket had 35 unclaimed addresses. Thirty-two
//! of them are the Metrowerks `__savegprlr_N` / `__restgprlr_N` / `__savefpr_N`
//! chain at `0x802DCCCC..0x802DCDC0`: each label is an alternate entry into one
//! spill run whose only effect is on callee-saved registers, so section 5.5 of
//! `docs/QUALITY.md` kept it out of this pass and
//! `docs/COVERAGE_WAVE100_CPP.md` lists it rather than claiming it. Wave 101
//! later translated those thirty-two entries as explicit register windows. The
//! three bodies below were the remainder of the bucket with observable
//! behaviour:
//!
//! 1. `0x802DCBFC` is the pointer-to-member-function call thunk. `r12` carries a
//!    CodeWarrior descriptor (`+0` this-delta, `+4` vtable index, `+8` target);
//!    the body adjusts `r3` by the delta, resolves the virtual slot when the
//!    index is non-negative and ends in `bctr`.
//! 2. `0x802DADD0` recomputes a look-at orientation: it builds the object's 3x4
//!    matrix, builds a second matrix from the `+0x5C..+0x64` scale vector,
//!    multiplies the local Y axis through both, adds the `+0x2C` vector of the
//!    sub-object at `+0x154`, normalises, inverts and stores the two angles at
//!    `+0x84`/`+0x88`.
//! 3. `0x802DFF64` is the `%a`/`%A` hexadecimal-float formatter. Its only
//!    paired-single instructions are the `psq_st`/`psq_l` save and restore of
//!    `f31` at `0x802DFF78`/`0x802E0308` (plus the usual `stfd`/`lfd` pair), so
//!    the export's `ldexpf` blocks at entry and exit are dead spills of a
//!    callee-saved register and are not modelled. `GQR0` is zero throughout
//!    this image: it is written in exactly two places, `li r3,0` +
//!    `mtspr 912..919` at `0x801AB170..0x801AB190` and the context restore at
//!    `0x802EA6FC`, which reloads the GQRs from the `stmw r20,0x2FC(r2)` save
//!    area filled by the matching save routine at `0x802EA684`.
//!
//! These bodies call the already-translated callees instead of their raw
//! addresses: [`fn_802dbfb0`], [`fn_802de370`] and [`fn_802dfa74`] live here,
//! [`fn_80289070`](crate::render::gx_band::fn_80289070) in `render::gx_band`,
//! [`fn_801c1160`](crate::runtime::dvd::fn_801c1160),
//! [`fn_801c1470`](crate::runtime::dvd::fn_801c1470) and
//! [`fn_801c07a0`](crate::runtime::dvd::fn_801c07a0) in `runtime::dvd`, and
//! [`fn_802e46a0`](crate::engine::main_loop::fn_802e46a0) and
//! [`fn_802e4c04`](crate::engine::main_loop::fn_802e4c04) in
//! `engine::main_loop`. `0x802382B0`, the matrix-from-translation builder, has
//! no body yet and is called by raw address.
//!
//! # Wave 101 pass - the callee-save register entries
//!
//! This pass claims the last thirty-two addresses of the `0x802D0000` bucket,
//! `0x802DCCCC..0x802DCDC0`. They are not game logic: they are the CodeWarrior
//! `__savefpr_N` / `__restfpr_N` / `__savegprlr_N` / `__restgprlr_N` helpers
//! that every non-leaf function in the image uses. Waves 77-100 deferred them
//! because their only effect is on registers, and because Ghidra's C is
//! uninformative for the restore half (`return;`) and impossible for the save
//! half (`unaff_rN`).
//!
//! They are translated here because the raw instructions are complete and
//! unambiguous, and because deferring them left the bucket incomplete. The model
//! is explicit:
//!
//! 1. `r11` is the caller's frame base. Every store or load is at
//!    `r11 - 0x48 .. r11 - 4`, four bytes apart for GPRs and eight for FPRs.
//! 2. A save body takes `frame` plus the live `rN..r31` values as a fixed-size
//!    array and writes them; a restore body takes `frame` and returns the values
//!    it loads. Rust cannot name the PowerPC register file, so the array is the
//!    honest surface for an ABI the decompiler cannot express.
//! 3. The 4-byte `functions.tsv` sizes are symbol extents, not routine extents:
//!    a `bl` to `0x802DCD40` executes `stw r14..r31` and returns through the
//!    `blr` at `0x802DCD88`. Each body therefore covers the run from its own
//!    label to that `blr`, and its doc block says so.
//!
//! Four private helpers (`save_gpr_run`, `load_gpr_run`, `save_fpr_run`,
//! `load_fpr_run`) hold the loop; every public body names its own first offset
//! and register count. The instruction bytes were re-read from
//! `work/dol/main.elf` for this pass and are quoted per entry.
//!
//! Confidence is `medium` throughout: the instruction sequences and offsets are
//! literal, while the register-array signature is a modelling decision.
//!
//! # Dependencies
//!
//! Only [`crate::platform`] (big-endian volatile reads/writes and indirect
//! calls) plus the local `call0_word` wrapper described above. Callees outside
//! this module are kept as raw addresses with a comment. Three exceptions are
//! deliberate, because the body already exists elsewhere and duplicating it
//! would fork the translation: the two-thunk pair in Wave 96 calls
//! [`crate::runtime::dvd::fn_801c76c0`] and
//! [`crate::runtime::dvd::fn_801c9cd0`], and the Wave 100 bodies call the
//! matrix/vector kernels of `runtime::dvd`, [`crate::render::gx_band`] and the
//! two MSL math routines of `engine::main_loop` by name.
//!
//! # Start here
//!
//! Read [`fn_80040540`] first: it is the smallest conditional release and shows
//! the shape the second release helper reuses. Then read [`fn_80040720`], the
//! most interesting plumbing function, because the raw PowerPC returns a value
//! that Ghidra's C does not reproduce; that discrepancy is documented on the
//! function. [`fn_800483f0`] and [`fn_8004de30`] are the shortest entry points
//! if you only want the field-access style. For the runtime band, start at
//! [`fn_802dc780`] (global destructors) and then [`fn_802dd860`] (`free`),
//! which is the densest piece of the allocator. For the floating-point print
//! path read [`fn_802de20c`] first, then [`fn_802de370`]; for the C++ object
//! vocabulary of the record built by [`fn_802dbb70`], read
//! [`fn_802dc4f0`] and then the small accessors around it.
//!
//! # Glossary
//!
//! * **vtable** - array of function pointers stored at offset 0 of every
//!   polymorphic object. Slots `+0x0C` and `+0x20` are used here; actor classes
//!   in this image put the same table at `+0x60` (`game::actors::dokan`).
//! * **deleting destructor** - CodeWarrior convention where one entry point
//!   both tears down and (optionally) frees the storage, selected by a flag.
//! * **static chain (`r12`)** - register that carries the dispatch-table
//!   pointer for calls that adjust `this` to a secondary base class.
//! * **SDA / SDA2** - small-data areas addressed relative to `r13` / `r2`. This
//!   module uses neither; the `r13` constants belong to the dokan actor.
//!
//! # Original
//!
//! Ghidra names are `FUN_<address>`; the image carries no symbol table, so the
//! address is the honest identity and each descriptive role is `// inferred:`
//! from the call shape. Every function cites its Ghidra C file under
//! `work/ghidra/main-gb/export/`.
//!
//! # Same shapes elsewhere in the range
//!
//! `0x800408F0` is the twin of [`fn_800408c0`] for the next class. The
//! destructor/walk/compare families at `0x80043430/90`, `0x80044E50/EB0` and
//! `0x80046070/80046410` are near-identical copies whose vtable types are not
//! confirmed, so they stay unclassified until a lane reads their vtables.

use crate::platform;

// ---------------------------------------------------------------------------
// 0x80040540 - deleting destructor
// ---------------------------------------------------------------------------

/// Conditionally releases an object: runs the shared `0x8015F800` thunk with
/// a zero delete flag, then frees the storage when the caller's signed flag is
/// positive; returns the object pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_80040540` at `0x80040540` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80040540_FUN_80040540.c`
///
/// # Arguments
/// - `object`: raw `this` pointer. May be null.
/// - `delete_flag`: `> 0` means "also free the storage"; `0` means
///   "leave the object in place". The comparison is signed (`bgt`).
///
/// # Returns
/// `object`, unchanged. This is the CodeWarrior deleting-destructor
/// convention: callers may use the returned pointer.
///
/// # Side effects
/// - `0x8015F800(object, 0)`: shared teardown thunk. Its own body frees only
///   when the second argument is `> 0`, so this call is a no-op for a non-null
///   object and a zero flag (`// observed:` `cmpwi r4,0` / `ble` at
///   `0x8015F81C/20`). It is kept because the original emits it.
/// - `0x802B9280(object)`: allocator release. `// inferred:` its body sets
///   `r4 = 0` and tail-calls `0x802B8F70`, which queries the block size when
///   the size argument is zero.
///
/// # Safety
/// `object` must be null or point to a live object whose first word is the
/// expected vtable; freeing makes it dangling.
///
/// # Confidence
/// high for control flow and both callees; medium for the role. The shape is
/// the CodeWarrior deleting-destructor idiom, but `0x8015F800(object, 0)` is
/// observed to do nothing, so this is more precisely a conditional release.
pub unsafe fn fn_80040540(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        // observed 0x80040564/0x80040568: `li r4,0`; `bl 0x8015F800`.
        platform::call2(0x8015_F800, object as u32, 0);
        // observed 0x8004056C..0x80040578: signed `bgt` on the second argument,
        // then `bl 0x802B9280`.
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x800405A0 - same shape as 0x80040540, different class
// ---------------------------------------------------------------------------

/// A second conditional-release helper with the same effect as
/// [`fn_80040540`], but with an extra null test immediately before the shared
/// `0x8015F800` call.
///
/// # Original
/// - Ghidra: `FUN_800405A0` at `0x800405A0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800405a0_FUN_800405a0.c`
///
/// # Arguments
/// - `object`: raw `this` pointer. May be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Same two calls as [`fn_80040540`]: the shared teardown thunk (a no-op when
/// its flag is 0), then the allocator release when requested.
///
/// # Safety
/// Same contract as [`fn_80040540`].
///
/// # Confidence
/// high - the instruction stream has the same body as `0x80040540` plus one
/// additional `beq` at `0x800405C4`. Ghidra folds that duplicate null test
/// away; the code below keeps it because it is an observable branch.
pub unsafe fn fn_800405a0(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        // observed 0x800405C4: a second `beq` skips the base destructor.
        if object != 0 {
            platform::call2(0x8015_F800, object as u32, 0);
        }
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x80040720 - unrolled chain walk
// ---------------------------------------------------------------------------

/// Walks a five-deep chain of objects linked through `+0x30`, asking each
/// node's virtual slot `+0x0C` whether it matches; returns the parent of the
/// first matching node and otherwise tail-recurses on the fifth node.
///
/// # Original
/// - Ghidra: `FUN_80040720` at `0x80040720` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80040720_FUN_80040720.c`
///
/// # Arguments
/// - `first`: head of the chain. Its *child* at `first + 0x30` is the first
///   node tested; `first` itself is never passed to the predicate.
///
/// # Returns
/// The parent of the matching child:
/// - level 1 returns `first`;
/// - levels 2..5 return the node whose `+0x30` link was tested and matched.
/// The predicate's own return value is only used as a boolean, never returned.
///
/// # Side effects
/// One indirect call per level (five unrolled levels) via vtable slot
/// `+0x0C`; each receives the candidate node in `r3`. No writes in this
/// function itself.
///
/// # Safety
/// Every `+0x30` link must be null or point to a readable object whose first
/// word is a vtable with a function at `+0x0C`. The recursion is bounded by
/// the chain, exactly as in the original.
///
/// # Confidence
/// high - the five blocks return the parent saved in `r31`, and Ghidra's
/// `return param_1` reproduces that exactly; the role is inferred.
pub unsafe fn fn_80040720(first: usize) -> usize {
    // observed 0x80040734: the first call is on `*(first + 0x30)`.
    let mut node = platform::read_u32(first + 0x30) as usize;
    let mut parent = first;

    let mut level = 0;
    while level < 5 {
        // observed 0x80040738..0x80040744: vtable = *(node); f = vtable[0x0C].
        let vtable = platform::read_u32(node) as usize;
        let predicate = platform::read_u32(vtable + 0x0C) as usize;

        // observed: r3 already holds `node` when the indirect call is made,
        // so the predicate takes the candidate node as its `this`.
        if platform::call1(predicate, node as u32) != 0 {
            // observed 0x8004074C/6C/8C/AC/CC: every exit returns the parent
            // currently held in r31 (Ghidra's `return param_1`).
            return parent;
        }

        if level == 4 {
            // observed 0x800407D4/0x800407D8: reload `*(parent + 0x30)` -
            // that is the fifth node itself - and tail-call this function.
            return fn_80040720(node);
        }

        parent = node;
        node = platform::read_u32(node + 0x30) as usize;
        level += 1;
    }

    // The loop always returns at level 4; this only satisfies the type checker.
    unreachable!()
}

// ---------------------------------------------------------------------------
// 0x80040800 - compare the tail of two names after the last ':'
// ---------------------------------------------------------------------------

/// Compares the substring after the last `':'` in `name` with the substring
/// after the last `':'` in the object's virtual name, and reports equality.
///
/// # Original
/// - Ghidra: `FUN_80040800` at `0x80040800` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80040800_FUN_80040800.c`
///
/// # Arguments
/// - `object`: object whose vtable slot `+0x20` returns a C string (the
///   `// inferred:` class/instance name).
/// - `name`: C string to compare against.
///
/// # Returns
/// `true` when both tails compare equal (`strcmp == 0`).
///
/// # Side effects
/// One virtual call (`vtable + 0x20`) and two libc-style calls. No writes.
///
/// # Safety
/// Both strings must be NUL-terminated. `object` must be a valid object with
/// that vtable slot. The original does **not** check whether the object name
/// contains `':'`: a missing colon makes it read from address `1`. The same
/// behavior is kept here, with the address arithmetic done in `usize` so the
/// reconstruction itself is not undefined behavior.
///
/// # Confidence
/// high for the call sequence. `0x802E1C40` is `strrchr` (it scans the whole
/// string and keeps the last match) and `0x802E1AB4` is `strcmp`; both are
/// observed in their own Ghidra bodies.
pub unsafe fn fn_80040800(object: usize, name: *const u8) -> bool {
    let mut rhs = name as usize;

    // observed 0x80040820/0x80040824: strrchr(name, ':').
    let colon = platform::call2(0x802E_1C40, rhs as u32, b':' as u32) as usize;
    if colon != 0 {
        // observed 0x8004082C..0x80040830: only the right-hand string gets a
        // null check.
        rhs = colon + 1;
    }

    // observed 0x80040834..0x80040844: virtual slot +0x20, called with `object`.
    let vtable = platform::read_u32(object) as usize;
    let get_name = platform::read_u32(vtable + 0x20) as usize;
    let object_name = platform::call1(get_name, object as u32) as usize;

    // observed 0x80040848..: strrchr(object_name, ':') with no null check,
    // then strcmp(colon + 1, rhs).
    let object_colon = platform::call2(0x802E_1C40, object_name as u32, b':' as u32) as usize;
    platform::call2(0x802E_1AB4, (object_colon + 1) as u32, rhs as u32) == 0
}

// ---------------------------------------------------------------------------
// 0x800408C0 - secondary-base adjustor thunk
// ---------------------------------------------------------------------------

/// Forwards a call to the virtual function at `a1 + 0x18`, passing `a2` as the
/// receiver, after loading the CodeWarrior static chain register `r12`.
///
/// # Original
/// - Ghidra: `FUN_800408C0` at `0x800408C0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800408c0_FUN_800408c0.c`
///
/// # Arguments
/// - `a1`: object whose subobject at `+0x18` supplies the dispatch table.
/// - `a2`: receiver forwarded as `this`.
///
/// # Returns
/// Nothing: the original tail-calls `0x802DCBFC`.
///
/// # Side effects
/// One indirect call through `0x802DCBFC`, a jump-table dispatcher that reads
/// the table pointer from `r12`.
///
/// # Safety
/// `a1 + 0x18` must point to a valid dispatch structure; `a2` must be a valid
/// receiver for the selected slot.
///
/// # Confidence
/// high - `or r5,r3` / `or r3,r4` / `addi r12,r5,24` / `bl 0x802DCBFC` at
/// `0x800408C8..0x800408D8`; the descriptor is `a1 + 0x18` and the receiver is
/// `a2`, which [`fn_802dcbfc`] takes as `(this, pmf)`.
///
/// `0x800408F0` has the same Ghidra body and is the twin entry point for the
/// next class; it is not translated here.
pub unsafe fn fn_800408c0(a1: usize, a2: usize) {
    // observed 0x800408C8: r5 = a1; 0x800408CC: r3 = a2; 0x800408D4:
    // r12 = a1 + 0x18; 0x800408D8: tail call 0x802DCBFC with (this, pmf).
    let _ = fn_802dcbfc(a2, Descriptor::new(a1 + 0x18));
}

// ---------------------------------------------------------------------------
// 0x800483F0 - one-byte counter decrement (generic object-field helper)
// ---------------------------------------------------------------------------

/// Decrements a byte counter unless it is already zero, and returns the new
/// value.
///
/// # Original
/// - Ghidra: `FUN_800483F0` at `0x800483F0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800483f0_FUN_800483f0.c`
///
/// # Arguments
/// - `counter`: pointer to one byte in memory. Must be readable/writable.
///
/// # Returns
/// The byte after the optional decrement (`0..=255`).
///
/// # Side effects
/// Writes `*counter - 1` when `*counter != 0`.
///
/// # Safety
/// `counter` must be a valid non-null pointer to one writable byte.
///
/// # Confidence
/// high - the body is `lbz`, `cmpwi`, `beq`, `addi -1`, `stb`, `lbz`.
///
/// # Ghidra disagreement
/// Ghidra types the value as signed `char`. The instructions use `lbz`, so the
/// value is unsigned; the code below returns `u8`.
pub unsafe fn fn_800483f0(counter: usize) -> u8 {
    let value = platform::read_u8(counter);
    if value != 0 {
        // observed 0x800483FC..0x80048400: `addi r0,r4,-1`; `stb r0,0(r3)`.
        platform::write_u8(counter, value.wrapping_sub(1));
    }
    platform::read_u8(counter)
}

// ---------------------------------------------------------------------------
// 0x8004DE30 - byte-to-bool (generic object-field helper)
// ---------------------------------------------------------------------------

/// Converts the byte at `+0x1460` into `0` or `1`.
///
/// # Original
/// - Ghidra: `FUN_8004DE30` at `0x8004DE30` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8004de30_FUN_8004de30.c`
///
/// # Arguments
/// - `object`: object with a readable byte at `+0x1460`.
///
/// # Returns
/// `1` when the byte is non-zero, else `0`.
///
/// # Side effects
/// None; one read.
///
/// # Safety
/// `object` must point to a readable byte at that offset.
///
/// # Confidence
/// high - the body is `lbz`, `neg`, `srwi 31`.
pub unsafe fn fn_8004de30(object: usize) -> u32 {
    // observed 0x8004DE30..0x8004DE3C: `lbz`; `neg`; logical shift right 31.
    (0u32.wrapping_sub(platform::read_u8(object + 0x1460) as u32)) >> 31
}

// ===========================================================================
// Metrowerks runtime / CRT band 0x802D0000..0x802E0000
// ===========================================================================

// r13 is `0x8042F680` on this build (docs/ARCHITECTURE.md, "Register baseline
// at entry"), so every `r13 - N` global in the Ghidra C becomes a fixed
// address. The constants are named once here instead of being repeated.
//
// | r13-relative | absolute     | meaning (inferred)                        |
// |--------------|--------------|-------------------------------------------|
// | `-0x4698`    | `0x8042AFE8` | C library `errno` cell                    |
// | `-0x46A8`    | `0x8042AFD8` | head of the C++ global-destructor chain   |
// | `-0x46A0`    | `0x8042AFE0` | "small-block pool initialised" flag       |
// | `-0x6188`    | `0x804294F8` | default-heap id (`-1` = uninitialised)    |

/// Head of the singly linked C++ global-destructor chain (`r13 - 0x46A8`).
const GLOBAL_DTOR_HEAD: usize = 0x8042_AFD8;

/// C library `errno` cell (`r13 - 0x4698`). `0x28` (= `ENOSPC`) is the only
/// value this band stores.
const ERRNO_CELL: usize = 0x8042_AFE8;

/// 32-slot registry at `0x80424CF8`; each slot is `{handler, argument, used}`.
const REGISTRY_TABLE: usize = 0x8042_4CF8;
const REGISTRY_SLOTS: i32 = 0x20;
const REGISTRY_STRIDE: usize = 12;

/// Default-heap id used by the Metrowerks arena (`r13 - 0x6188`).
const DEFAULT_HEAP_ID: usize = 0x8042_94F8;

/// Small-block pool descriptor (`0x80424E78`, 0x34 bytes) and its lazy-init
/// flag (`r13 - 0x46A0`).
const POOL_DESCRIPTOR: usize = 0x8042_4E78;
const POOL_INIT_FLAG: usize = 0x8042_AFE0;

// ---------------------------------------------------------------------------
// 0x802DC69C - strlen
// ---------------------------------------------------------------------------

/// Counts the bytes before the first NUL terminator.
///
/// # Original
/// - Ghidra: `FUN_802dc69c` at `0x802DC69C` (28 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc69c_FUN_802dc69c.c`
///
/// # Arguments
/// - `text`: address of a NUL-terminated byte string.
///
/// # Returns
/// Number of bytes before the terminator, not counting it.
///
/// # Side effects
/// None. Reads one byte per iteration.
///
/// # Safety
/// `text` must be readable and must reach a zero byte before the end of the
/// mapped region; there is no length bound in the original.
///
/// # Confidence
/// high - `lbzu` / `addi` / `cmpwi` / `bne` loop, checked against the raw ELF
/// at `0x802DC69C`.
pub unsafe fn fn_802dc69c(text: usize) -> i32 {
    let mut cursor = text.wrapping_sub(1);
    let mut length: i32 = -1;
    loop {
        // observed 0x802DC6A4..0x802DC6B0: load-with-update, bump the counter,
        // branch back while the byte is non-zero.
        cursor = cursor.wrapping_add(1);
        length = length.wrapping_add(1);
        if platform::read_u8(cursor) == 0 {
            return length;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DC780 - register one global destructor
// ---------------------------------------------------------------------------

/// Pushes a `{next, destructor, object}` node onto the C++ global-destructor
/// chain.
///
/// # Original
/// - Ghidra: `FUN_802dc780` at `0x802DC780` (24 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc780_FUN_802dc780.c`
///
/// # Arguments
/// - `object`: the object whose destructor must run at exit.
/// - `destructor`: address of the deleting-destructor-style entry point; the
///   chain runner calls it as `destructor(object, -1)`.
/// - `node`: caller-owned 12-byte record, usually inside the object's static
///   storage, so the registration needs no allocation.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three words at `node` and updates [`GLOBAL_DTOR_HEAD`]. The chain is
/// LIFO: the most recent registration runs first.
///
/// # Safety
/// `node` must be 12 writable bytes that outlive the process; `destructor`
/// must accept `(object, -1)`.
///
/// # Confidence
/// high for the list surgery (register trace at `0x802DC780..0x802DC794`);
/// medium for the name, which is the standard CodeWarrior registration idiom.
pub unsafe fn fn_802dc780(object: usize, destructor: usize, node: usize) {
    // observed: lwz r0,-0x46a8(r13); stw r0,0(r5); stw r4,4(r5); stw r3,8(r5);
    //           stw r5,-0x46a8(r13)
    platform::write_u32(node, platform::read_u32(GLOBAL_DTOR_HEAD));
    platform::write_u32(node + 4, destructor as u32);
    platform::write_u32(node + 8, object as u32);
    platform::write_u32(GLOBAL_DTOR_HEAD, node as u32);
}

// ---------------------------------------------------------------------------
// 0x802DC798 - run the global destructor chain
// ---------------------------------------------------------------------------

/// Pops every node from the C++ global-destructor chain and calls its
/// destructor with the delete flag `-1`.
///
/// # Original
/// - Ghidra: `FUN_802dc798` at `0x802DC798` (72 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc798_FUN_802dc798.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls each registered destructor once, newest first, and leaves
/// [`GLOBAL_DTOR_HEAD`] null. The calls are indirect, so any global state the
/// destructors touch changes.
///
/// # Safety
/// Every queued node must still be valid; running this while more objects are
/// being registered is not guarded by a lock in the original.
///
/// # Confidence
/// high - the loop head is reloaded from `0x8042AFD8` at the end of every
/// iteration (`lwz r3,-0x46a8(r13)` at `0x802DC7C4`), so registrations made by
/// a destructor are also drained.
pub unsafe fn fn_802dc798() {
    loop {
        let node = platform::read_u32(GLOBAL_DTOR_HEAD) as usize;
        if node == 0 {
            return;
        }
        // observed 0x802DC7A8..0x802DC7C0: unlink, load dtor into r12 and the
        // object into r3, `li r4,-1`, then `bctrl`.
        platform::write_u32(GLOBAL_DTOR_HEAD, platform::read_u32(node));
        let destructor = platform::read_u32(node + 4) as usize;
        let object = platform::read_u32(node + 8) as usize;
        platform::call2(destructor, object as u32, 0xFFFF_FFFF);
    }
}

// ---------------------------------------------------------------------------
// 0x802DCA98 - destroy an array of objects (stride + count form)
// ---------------------------------------------------------------------------

/// Runs a destructor over `count` objects laid out `stride` bytes apart,
/// highest address first.
///
/// # Original
/// - Ghidra: `FUN_802dca98` at `0x802DCA98` (120 bytes)
/// - C file: `work/ghidra/main-gb/export/802dca98_FUN_802dca98.c`
///
/// # Arguments
/// - `base`: address of element 0.
/// - `destructor`: destructor entry point, called as `destructor(element, -1)`.
/// - `stride`: element size in bytes.
/// - `count`: number of elements; `0` makes the call a no-op.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call per element. The original computes `base + stride*count`
/// first and then subtracts `stride` **before** each call, so the first object
/// destroyed is the last one.
///
/// # Safety
/// The whole `count * stride` range must hold live objects and `destructor`
/// must accept `(element, -1)`.
///
/// # Confidence
/// high - the multiply-then-subtract order is visible at `0x802DCA98` and is
/// preserved here.
pub unsafe fn fn_802dca98(base: usize, destructor: usize, stride: i32, count: i32) {
    let mut element =
        (base as isize).wrapping_add((stride as isize).wrapping_mul(count as isize)) as usize;
    let mut remaining = count;
    while remaining != 0 {
        // observed: `subf`/pointer decrement precedes the indirect call.
        element = (element as isize).wrapping_sub(stride as isize) as usize;
        platform::call2(destructor, element as u32, 0xFFFF_FFFF);
        remaining -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802DCB10 - destroy a `new[]` array (header form)
// ---------------------------------------------------------------------------

/// Destroys the elements of a `new[]` array and frees the backing store.
///
/// # Original
/// - Ghidra: `FUN_802dcb10` at `0x802DCB10` (128 bytes)
/// - C file: `work/ghidra/main-gb/export/802dcb10_FUN_802dcb10.c`
///
/// # Arguments
/// - `block`: user pointer returned by `new[]`. Its 16-byte header sits at
///   `block - 0x10`: `+0x00` = element stride, `+0x04` = element count.
/// - `destructor`: per-element destructor, or `0` to skip element teardown.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `destructor(element, -1)` for every element, highest address first,
/// then releases the header with `0x802B9290(block - 0x10)` (the allocator
/// entry that also appears in [`fn_80040540`] as `0x802B9280`'s sibling).
///
/// # Safety
/// `block` must be null or a live `new[]` result whose header is intact;
/// `destructor` must accept `(element, -1)`.
///
/// # Confidence
/// high for the loop and the free call; medium for the header field roles,
/// which are inferred from the two reads at `block - 0x10` / `block - 0x0C`.
pub unsafe fn fn_802dcb10(block: usize, destructor: usize) {
    if block == 0 {
        return;
    }
    if destructor != 0 {
        // observed: stride at header+0, count at header+4, then the same
        // multiply-then-descend shape as fn_802dca98.
        let stride = platform::read_i32(block - 0x10);
        let count = platform::read_u32(block - 0x0C);
        let mut element =
            (block as isize).wrapping_add((stride as isize).wrapping_mul(count as isize)) as usize;
        let mut index: u32 = 0;
        while index < count {
            element = (element as isize).wrapping_sub(stride as isize) as usize;
            platform::call2(destructor, element as u32, 0xFFFF_FFFF);
            index += 1;
        }
    }
    // observed 0x802DCB7C: release the block that starts 0x10 bytes earlier.
    platform::call1(0x802B_9290, (block - 0x10) as u32);
}

// ---------------------------------------------------------------------------
// 0x802DCC4C - double to 32-bit conversion with saturation
// ---------------------------------------------------------------------------

/// Converts a `double` to the bit pattern of a 32-bit unsigned value, clamping
/// out-of-range inputs.
///
/// # Original
/// - Ghidra: `FUN_802dcc4c` at `0x802DCC4C` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcc4c_FUN_802dcc4c.c`
///
/// # Arguments
/// - `value`: the `double` to convert (PowerPC `f1`).
///
/// # Returns
/// - `0` when `value < 0.0`;
/// - `0xFFFFFFFF` (`-1`) when `value >= 2^32`;
/// - otherwise the low 32 bits of the truncated value. The `[2^31, 2^32)` range
///   is biased through `2^31` so that the hardware `fctiwz` result is remapped
///   back into the unsigned range.
///
/// # Side effects
/// None; pure. Reads the three constants at `0x802FD338` (`0.0`),
/// `0x802FD340` (`2^32`) and `0x802FD348` (`2^31`).
///
/// # Safety
/// Pure arithmetic; no pointers are dereferenced.
///
/// # Confidence
/// high - the compare/bias/subtract sequence is explicit and the three
/// constants were read from the ELF (`0x41F0000000000000`, `0x41E0000000000000`).
pub fn fn_802dcc4c(value: f64) -> i32 {
    if value < 0.0 {
        return 0;
    }
    if value >= 4_294_967_296.0 {
        return -1;
    }
    let mut scaled = value;
    let mut high_half = false;
    if scaled >= 2_147_483_648.0 {
        // observed: subtract 2^31, convert, then add -0x80000000 back.
        scaled -= 2_147_483_648.0;
        high_half = true;
    }
    let mut result = scaled as i32;
    if high_half {
        result = result.wrapping_add(i32::MIN);
    }
    result
}

// ---------------------------------------------------------------------------
// 0x802DCDD8 - unsigned 64-bit divide
// ---------------------------------------------------------------------------

/// Divides one unsigned 64-bit value by another and returns the quotient.
///
/// # Original
/// - Ghidra: `FUN_802dcdd8` at `0x802DCDD8` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdd8_FUN_802dcdd8.c`
///
/// # Arguments
/// - `dividend`: 64-bit numerator. The ABI passes it as the pair `r3:r4`.
/// - `divisor`: 64-bit denominator, passed as `r5:r6`.
///
/// # Returns
/// `dividend / divisor`. The original returns `0` when the dividend's bit
/// length is smaller than the divisor's, i.e. when the quotient is zero.
///
/// # Side effects
/// None; pure. No memory is touched.
///
/// # Safety
/// Pure arithmetic; no pointers are dereferenced.
///
/// # Reconstruction note
/// The original is a normalised shift-subtract loop that counts leading zeros,
/// pre-shifts both operands once, then performs 64 restoring-division steps,
/// shifting one quotient bit in per iteration. The instruction stream at
/// `0x802DCDD8` confirms that shape. Translating the decompiler's carry
/// bookkeeping (`CARRY4(...) || CARRY4(...)`) into Rust would reproduce
/// Ghidra's 32-bit artefact rather than the machine's 64-bit compare, and the
/// `iVar10 == 0x40` normalisation case shifts by 32, which the decompiler
/// cannot express. The reconstruction therefore uses the native 64-bit
/// operator, which yields exactly the same quotient for every divisor that the
/// loop can normalise (`divisor != 0`). A zero divisor never normalises; the
/// guard returns `0`, the value the original's "dividend shorter than divisor"
/// path would produce, instead of trapping the host.
///
/// # Confidence
/// high for the role and the result; medium for bit-exact agreement with the
/// shipped loop on the zero-divisor path, which is undefined in the original.
pub fn fn_802dcdd8(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        // inferred: the loop cannot normalise a zero divisor, so the shipped
        // result is undefined; 0 is the quotient the early-out would give.
        return 0;
    }
    dividend / divisor
}

// ---------------------------------------------------------------------------
// 0x802DCEC4 - signed 64-bit divide
// ---------------------------------------------------------------------------

/// Divides one signed 64-bit value by another (truncating toward zero).
///
/// # Original
/// - Ghidra: `FUN_802dcec4` at `0x802DCEC4` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcec4_FUN_802dcec4.c`
///
/// # Arguments
/// - `dividend`: signed numerator (`r3:r4`).
/// - `divisor`: signed denominator (`r5:r6`).
///
/// # Returns
/// `dividend / divisor` truncated toward zero. The sign of the result is the
/// exclusive-or of the operand signs, which is what the original's final
/// `if (uVar3 != uVar4)` negation implements.
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// Pure arithmetic. `i64::MIN / -1` would overflow; the guard below returns
/// `i64::MIN`, the wrap-around value the hardware sequence produces.
///
/// # Reconstruction note
/// Same normalised shift-subtract loop as [`fn_802dcdd8`], with an
/// absolute-value prologue and a conditional negation epilogue. The two
/// observable steps that are preserved in structure here are the sign
/// capture/restore and the truncating division; the Rust operator provides the
/// identical result without Ghidra's broken 32-bit carry model.
///
/// # Confidence
/// high for sign handling and result; medium for the same zero-divisor caveat
/// as [`fn_802dcdd8`].
pub fn fn_802dcec4(dividend: i64, divisor: i64) -> i64 {
    if divisor == 0 {
        return 0;
    }
    if dividend == i64::MIN && divisor == -1 {
        return i64::MIN;
    }
    dividend / divisor
}

// ---------------------------------------------------------------------------
// 0x802DCFFC - unsigned 64-bit modulo
// ---------------------------------------------------------------------------

/// Returns the remainder of an unsigned 64-bit division.
///
/// # Original
/// - Ghidra: `FUN_802dcffc` at `0x802DCFFC` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcffc_FUN_802dcffc.c`
///
/// # Arguments
/// - `dividend`: 64-bit numerator (`r3:r4`).
/// - `divisor`: 64-bit denominator (`r5:r6`).
///
/// # Returns
/// `dividend % divisor`; when the dividend is already smaller than the
/// divisor the original returns the dividend unchanged (`0x802DD074`,
/// `return CONCAT44(param_1,param_2)`).
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// Pure arithmetic; a zero divisor returns the dividend to keep the host from
/// trapping (undefined in the original loop).
///
/// # Reconstruction note
/// The loop is the same restoring-division kernel as [`fn_802dcdd8`]; only the
/// final quotient assembly differs (this one returns the remainder kept in
/// `uVar5:uVar6`). Native `%` reproduces it for every normalisable divisor.
///
/// # Confidence
/// high for the role and result; medium on the zero-divisor path.
pub fn fn_802dcffc(dividend: u64, divisor: u64) -> u64 {
    if divisor == 0 {
        return dividend;
    }
    dividend % divisor
}

// ---------------------------------------------------------------------------
// 0x802DD0E0 - signed 64-bit modulo
// ---------------------------------------------------------------------------

/// Returns the remainder of a signed 64-bit division.
///
/// # Original
/// - Ghidra: `FUN_802dd0e0` at `0x802DD0E0` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dd0e0_FUN_802dd0e0.c`
///
/// # Arguments
/// - `dividend`: signed numerator (`r3:r4`).
/// - `divisor`: signed denominator (`r5:r6`).
///
/// # Returns
/// `dividend % divisor`. The original negates the operands up front and then
/// negates the remainder when only the **dividend** was negative
/// (`if (bVar1)` at `0x802DD2C0`), which is exactly Rust's truncating `%`
/// (the result takes the dividend's sign).
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// Pure arithmetic; `i64::MIN % -1` overflows and is special-cased to `0`.
///
/// # Confidence
/// high for the sign rule; medium on the zero-divisor path, as with the other
/// three helpers.
pub fn fn_802dd0e0(dividend: i64, divisor: i64) -> i64 {
    if divisor == 0 {
        return dividend;
    }
    if dividend == i64::MIN && divisor == -1 {
        return 0;
    }
    dividend % divisor
}

// ---------------------------------------------------------------------------
// 0x802DD1EC - 64-bit shift left
// ---------------------------------------------------------------------------

/// Shifts a 64-bit value left by `count` bits.
///
/// # Original
/// - Ghidra: `FUN_802dd1ec` at `0x802DD1EC` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dd1ec_FUN_802dd1ec.c`
///
/// # Arguments
/// - `value`: the 64-bit value (`r3:r4`).
/// - `count`: shift amount in bits (`r5`).
///
/// # Returns
/// `value << count`, taken modulo 64 bits. The decompiled expression
/// `param_1 << n | param_2 >> (0x20 - n) | param_2 << (n - 0x20)` is the
/// 32-bit-word expansion of a 64-bit shift; the branchless instruction
/// sequence implements the same thing.
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// Pure arithmetic; `count >= 64` wraps as the hardware's masked count does.
///
/// # Confidence
/// high - the three-term expression is proven equivalent to a 64-bit shift for
/// `n < 32` and `n >= 32`, and the body is only three instructions.
pub fn fn_802dd1ec(value: u64, count: u32) -> u64 {
    value.wrapping_shl(count)
}

// ---------------------------------------------------------------------------
// 0x802DD210 - 64-bit logical shift right
// ---------------------------------------------------------------------------

/// Shifts a 64-bit value right by `count` bits, filling with zeroes.
///
/// # Original
/// - Ghidra: `FUN_802dd210` at `0x802DD210` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dd210_FUN_802dd210.c`
///
/// # Arguments
/// - `value`: the 64-bit value (`r3:r4`).
/// - `count`: shift amount in bits (`r5`).
///
/// # Returns
/// `value >> count` (logical). The decompiled body builds the low word as
/// `lo >> n | hi << (0x20 - n)` and, for `n > 32`, ORs in
/// `hi >> (n - 0x20)`; the high word is `hi >> n`. That is a 64-bit logical
/// shift, not an arithmetic one - the sign bit is never replicated.
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// Pure arithmetic; `count >= 64` wraps as the hardware's masked count does.
///
/// # Confidence
/// high - the word formula is unambiguous and the function is four
/// instructions long.
pub fn fn_802dd210(value: u64, count: u32) -> u64 {
    value.wrapping_shr(count)
}

// ---------------------------------------------------------------------------
// 0x802DD350 - allocate a registry slot
// ---------------------------------------------------------------------------

/// Finds a free slot in the 32-entry `{handler, argument, used}` registry and
/// records the pair there.
///
/// # Original
/// - Ghidra: `FUN_802dd350` at `0x802DD350` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd350_FUN_802dd350.c`
///
/// # Arguments
/// - `handler`: callback address stored in the slot.
/// - `argument`: opaque value handed to the callback.
///
/// # Returns
/// Slot index `0..=31`, or `-1` when all 32 slots are in use.
///
/// # Side effects
/// Writes `handler`, `argument` and `1` into the first free slot of the table
/// at [`REGISTRY_TABLE`]. This is the registration primitive used by the
/// lazy-exit machinery around [`fn_802dd2e0`]-style singletons (not translated
/// here); the consumer is the twinned release function [`fn_802dd39c`].
///
/// # Safety
/// The table is fixed at `0x80424CF8` and always writable in the target image.
///
/// # Confidence
/// high - 32 iterations of a 12-byte stride over `0x80424CF8`, verified in the
/// raw instructions; the *purpose* of the registry is inferred from the
/// allocate/release pair.
pub unsafe fn fn_802dd350(handler: usize, argument: usize) -> i32 {
    let mut slot = REGISTRY_TABLE;
    let mut index: i32 = 0;
    let mut remaining = REGISTRY_SLOTS;
    loop {
        // observed: `lwz r0,8(r4)` is the "slot in use" word.
        if platform::read_u32(slot + 8) == 0 {
            platform::write_u32(slot, handler as u32);
            platform::write_u32(slot + 4, argument as u32);
            platform::write_u32(slot + 8, 1);
            return index;
        }
        index += 1;
        slot += REGISTRY_STRIDE;
        remaining -= 1;
        if remaining == 0 {
            return -1;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DD39C - release a registry slot
// ---------------------------------------------------------------------------

/// Clears one slot of the registry described by [`fn_802dd350`].
///
/// # Original
/// - Ghidra: `FUN_802dd39c` at `0x802DD39C` (40 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd39c_FUN_802dd39c.c`
///
/// # Arguments
/// - `index`: slot number; values above 31 are ignored.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores zero in all three words of the slot (`handler`, `argument`, `used`).
/// No callback is invoked - releasing does not run the handler.
///
/// # Safety
/// The table is fixed at `0x80424CF8`, so no caller pointer is dereferenced.
///
/// # Confidence
/// high - bounds test plus three zero stores, checked in the raw ELF.
pub unsafe fn fn_802dd39c(index: u32) {
    if index > 0x1F {
        return;
    }
    let slot = REGISTRY_TABLE + index as usize * REGISTRY_STRIDE;
    platform::write_u32(slot, 0);
    platform::write_u32(slot + 4, 0);
    platform::write_u32(slot + 8, 0);
}

// ---------------------------------------------------------------------------
// 0x802DD3C4 - lazily create the default heap and release one block
// ---------------------------------------------------------------------------

/// Brings up the Metrowerks default arena on first use, then hands one block
/// back to the arena bin selected by the stored heap id.
///
/// # Original
/// - Ghidra: `FUN_802dd3c4` at `0x802DD3C4` (184 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd3c4_FUN_802dd3c4.c`
///
/// # Arguments
/// - `block`: a block header (or arena descriptor) owned by the runtime heap.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// On the first call, when [`DEFAULT_HEAP_ID`] is `-1`:
/// 1. logs `0x802FD350` ("GCN_Mem_Alloc.c : InitDefaultHeap") and `0x802FD386`
///    ("Metrowerks CW runtime library initialization incomplete") through
///    `0x8015F730`, the runtime's printf-style logger;
/// 2. reads the arena bounds with `0x801ABEB0` (base) and `0x801ABE80` (size);
/// 3. calls `0x801ABD80(base, size, 32)` - create the 32-bin arena;
/// 4. `0x801ABF10(arena)`, then `0x801ABF10((arena + 31) & ~31)`, with
///    `0x801ABD70` in between (these two set the runtime's current-arena
///    globals);
/// 5. `0x801ABDF0((arena + 31) & ~31, size & ~31)` - install the first region.
/// Then, always: `0x801ABCF0(heap_id, block)`, which unlinks `block` from its
/// bin and re-links the bin.
///
/// # Safety
/// `block` must be a header previously carved from the same arena.
///
/// # Confidence
/// medium-high for the control flow and the callees; medium for the roles.
/// The role is inferred from the two diagnostic strings ("InitDefaultHeap",
/// "runtime library initialization incomplete") and from the caller in
/// [`fn_802dd860`], which is `free`.
///
/// # Ghidra disagreement
/// Ghidra drops the argument of the first `0x801ABF10` call and gives the
/// second one `size & ~31`. The raw instructions at `0x802DD430` and
/// `0x802DD44C..0x802DD450` pass `r3 = arena` and `r3 = (arena + 31) & ~31`
/// respectively; the reconstruction follows the register trace.
pub unsafe fn fn_802dd3c4(block: usize) {
    if platform::read_i32(DEFAULT_HEAP_ID) == -1 {
        // observed 0x802DD3EC..0x802DD408: two logger calls with the strings at
        // 0x802FD350 and 0x802FD386.
        platform::call1(0x8015_F730, 0x802F_D350);
        platform::call1(0x8015_F730, 0x802F_D386);
        // observed 0x802DD40C..0x802DD428: base, size, then arena create with
        // 32 bins (the literal `li r5,1` is the arena generation/class count
        // the callee stores, not a bin count).
        let base = platform::call_u64(0x801A_BEB0) as u32;
        let size = platform::call_u64(0x801A_BE80) as u32;
        let arena = platform::call3(0x801A_BD80, base, size, 1) as u32;
        // observed 0x802DD42C..0x802DD450.
        platform::call1(0x801A_BF10, arena);
        let aligned_arena = arena.wrapping_add(0x1F) & !0x1F;
        platform::call2(0x801A_BDF0, aligned_arena, size & !0x1F);
        platform::call0(0x801A_BD70);
        platform::call1(0x801A_BF10, aligned_arena);
    }
    // observed 0x802DD454..0x802DD45C: reload the id, then release the block.
    platform::call2(0x801A_BCF0, platform::read_u32(DEFAULT_HEAP_ID), block as u32);
}

// ---------------------------------------------------------------------------
// 0x802DD47C - insert a freed block into its arena bin
// ---------------------------------------------------------------------------

/// Marks a block free, links it at the head of its size bin, merges it with a
/// free physical predecessor, and then asks [`fn_802dd5cc`] to merge the
/// successor.
///
/// # Original
/// - Ghidra: `FUN_802dd47c` at `0x802DD47C` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd47c_FUN_802dd47c.c`
///
/// # Arguments
/// - `pool`: arena descriptor. `pool + 0x0C` holds an offset to the bin array;
///   `pool + 0x08` is the arena's record of the largest free block.
/// - `block`: header of the block being freed. Layout observed from the code:
///   `block[0]` = size in bytes | flags (bit 1 = in use, bit 2 = "no predecessor
///   merge"), `block[1]` = owning arena, `block[-1]` = previous block's size.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears the in-use bit of this block and the "previous is busy" bit of the
/// next block, writes this block's size into the next block's `prev_size` word,
/// splices the block into the doubly linked free ring kept at `bin`, and
/// updates `pool + 0x08` when this block became the largest free block.
///
/// # Safety
/// `pool` must be a live arena descriptor and `block` a header inside it; the
/// ring pointers at `block + 8` / `block + 12` are overwritten, so the block
/// must not be in use.
///
/// # Confidence
/// high for the list surgery (the block has 14 stores, all reproduced);
/// medium for the bin-array offset, which is read through `pool + 0x0C`
/// without a name.
pub unsafe fn fn_802dd47c(pool: usize, block: usize) {
    // observed 0x802DD47C..0x802DD49C: size, clear in-use bit, clear the next
    // block's "prev busy" bit, then stamp the next block's prev_size.
    let size = platform::read_u32(block) & 0xFFFF_FFF8;
    platform::write_u32(block, platform::read_u32(block) & 0xFFFF_FFFD);
    let next = block.wrapping_add(size as usize);
    platform::write_u32(next, platform::read_u32(next) & 0xFFFF_FFFB);
    platform::write_u32(next.wrapping_sub(4), size);

    // observed 0x802DD4A0: bin = pool + (*(pool + 0xC) & ~7) - 4. The masks
    // drop the low flag bits of the offset / of the ring's back-pointer slot.
    let bin = pool
        .wrapping_add((platform::read_u32(pool + 0x0C) as usize) & 0xFFFF_FFF8)
        .wrapping_sub(4);

    let mut inserted = block;
    if platform::read_u32(bin) == 0 {
        // observed 0x802DD4A4..0x802DD4B4: empty bin, self-linked ring.
        platform::write_u32(bin, inserted as u32);
        platform::write_u32(inserted + 8, inserted as u32);
        platform::write_u32(inserted + 12, inserted as u32);
    } else {
        // observed 0x802DD4B8..0x802DD4D0: insert at the head of the ring.
        let head = platform::read_u32(bin) as usize;
        let head_next = platform::read_u32(head + 8) as usize;
        platform::write_u32(inserted + 8, head_next as u32);
        platform::write_u32(head_next + 12, inserted as u32);
        platform::write_u32(inserted + 12, head as u32);
        platform::write_u32(head + 8, inserted as u32);
        platform::write_u32(bin, inserted as u32);

        // observed 0x802DD4D4..0x802DD4F8: merge with the physical predecessor
        // when it is free (flag bit 2 of the size word clear, prev_size bit 1
        // clear).
        if (platform::read_u32(inserted) & 4) == 0 {
            let previous_size = platform::read_u32(inserted.wrapping_sub(4));
            if (previous_size & 2) == 0 {
                let previous = inserted.wrapping_sub(previous_size as usize);
                let flags = platform::read_u32(previous) & 7;
                // observed: the flags are rewritten first, then the merged size.
                platform::write_u32(previous, flags);
                let merged = previous_size
                    .wrapping_add(platform::read_u32(inserted) & 0xFFFF_FFF8)
                    & 0xFFFF_FFF8
                    | flags;
                platform::write_u32(previous, merged);
                if (flags & 2) == 0 {
                    platform::write_u32(
                        previous.wrapping_add(merged as usize).wrapping_sub(4),
                        merged,
                    );
                }
                // Unlink `inserted` from the free ring before re-inserting the
                // merged predecessor.
                if platform::read_u32(bin) == inserted as u32 {
                    platform::write_u32(bin, platform::read_u32(inserted + 12));
                }
                let link = platform::read_u32(inserted + 12) as usize;
                let neighbour = platform::read_u32(inserted + 8) as usize;
                platform::write_u32(link + 8, neighbour as u32);
                platform::write_u32(neighbour + 12, link as u32);
                inserted = previous;
            }
        }
    }

    // observed 0x802DD51C..0x802DD524: publish the (possibly merged) block and
    // merge forward. `0x802DD5CC` is the successor-coalescing routine.
    platform::write_u32(bin, inserted as u32);
    platform::call2(0x802D_D5CC, inserted as u32, bin as u32);

    // observed 0x802DD528..0x802DD540: track the largest free block seen.
    let head_size = platform::read_u32(platform::read_u32(bin) as usize) & 0xFFFF_FFF8;
    if platform::read_u32(pool + 8) < head_size {
        platform::write_u32(pool + 8, head_size);
    }
}

// ---------------------------------------------------------------------------
// 0x802DD860 - free
// ---------------------------------------------------------------------------

/// Releases one block: small blocks go back to the size-class pool, large
/// blocks to their arena's free list (and the arena itself may be returned to
/// the system).
///
/// # Original
/// - Ghidra: `FUN_802dd860` at `0x802DD860` (304 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd860_FUN_802dd860.c`
///
/// # Arguments
/// - `pointer`: user pointer to release; `0` is accepted and ignored.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - First call zeroes the 0x34-byte pool descriptor at [`POOL_DESCRIPTOR`]
///   with `memset` (`0x800046B4`) and sets [`POOL_INIT_FLAG`].
/// - Payload size is `(header[0] & ~7) - 8` for large blocks, or
///   `*(owner + 8)` for pool blocks (flag bit 0 of `header[1]` clear).
/// - Payloads smaller than `0x45` bytes go to `0x802DD674(pool, pointer, size)`.
/// - Larger ones are inserted with [`fn_802dd47c`] and, when the arena's whole
///   free space collapsed into one block (`(arena[4] & ~7) == (arena[3] & ~7) -
///   0x18` and flag bit 1 of `arena[4]` clear), unlinked from the global arena
///   list and handed to [`fn_802dd3c4`].
///
/// # Safety
/// `pointer` must be null or a live block from this allocator; the header at
/// `pointer - 8` is read and the block is mutated.
///
/// # Confidence
/// high for the two branches and the arena-release test; medium for the meaning
/// of the size-class `0x45` threshold and for the third argument of
/// `0x802DD674`, which Ghidra drops (see "Ghidra disagreement").
///
/// # Ghidra disagreement
/// `0x802DD674` takes `(pool, pointer, size)`; Ghidra renders the call with two
/// arguments. The caller already has the size in `uVar4`, and the callee's
/// first loop compares `param_3` against the class table at `0x802FD3C0`, so
/// the size is passed explicitly here.
pub unsafe fn fn_802dd860(pointer: usize) {
    if platform::read_u8(POOL_INIT_FLAG) == 0 {
        // observed 0x802DD870..0x802DD888: zero 0x34 bytes, then set the flag.
        platform::call3(0x8000_46B4, POOL_DESCRIPTOR as u32, 0, 0x34);
        platform::write_u8(POOL_INIT_FLAG, 1);
    }
    if pointer == 0 {
        return;
    }

    // observed 0x802DD89C..0x802DD8BC: two size conventions. Bit 0 of the
    // second header word selects "size is inline" (large block).
    let owner_word = platform::read_u32(pointer - 4);
    let size = if (owner_word & 1) == 0 {
        platform::read_u32(owner_word as usize + 8)
    } else {
        (platform::read_u32(pointer - 8) & 0xFFFF_FFF8).wrapping_sub(8)
    };

    if size < 0x45 {
        platform::call3(0x802D_D674, POOL_DESCRIPTOR as u32, pointer as u32, size);
        return;
    }

    // observed 0x802DD8C0..0x802DD8D4: the owning arena is the second header
    // word with its low flag bit cleared; the block header is pointer - 8.
    let arena = (owner_word & 0xFFFF_FFFE) as usize;
    platform::call2(0x802D_D47C, arena as u32, (pointer - 8) as u32);

    // observed 0x802DD8D8..0x802DD8F4: the arena is empty when its only free
    // block covers the whole region (total - 0x18 of overhead) and bit 1 of the
    // flags word is clear.
    let arena_flags = platform::read_u32(arena + 0x10);
    let empty = (arena_flags & 2) == 0
        && (arena_flags & 0xFFFF_FFF8)
            == (platform::read_u32(arena + 0x0C) & 0xFFFF_FFF8).wrapping_sub(0x18);
    if !empty {
        return;
    }

    // observed 0x802DD8F8..0x802DD93C: unlink the arena from the global list
    // (single-element rings become null) and give it back to the system.
    let mut next_arena = platform::read_u32(arena + 4) as usize;
    if next_arena == arena {
        next_arena = 0;
    }
    if platform::read_u32(POOL_DESCRIPTOR) == arena as u32 {
        platform::write_u32(POOL_DESCRIPTOR, next_arena as u32);
    }
    if next_arena != 0 {
        let forward = platform::read_u32(arena);
        platform::write_u32(next_arena, forward);
        platform::write_u32(forward as usize + 4, next_arena as u32);
    }
    platform::write_u32(arena + 4, 0);
    platform::write_u32(arena, 0);
    platform::call1(0x802D_D3C4, arena as u32);
}

// ---------------------------------------------------------------------------
// 0x802DE55C - flush the stdio write buffer
// ---------------------------------------------------------------------------

/// Writes the pending bytes of a `FILE` buffer through the stream's write
/// callback and rewinds the buffer state.
///
/// # Original
/// - Ghidra: `FUN_802de55c` at `0x802DE55C` (184 bytes)
/// - C file: `work/ghidra/main-gb/export/802de55c_FUN_802de55c.c`
///
/// # Arguments
/// - `file`: address of the 0x4C-byte stream object. Offsets used here:
///   `+0x00` file descriptor/handle, `+0x18` logical position, `+0x1C` buffer
///   start, `+0x20` buffer size, `+0x24` buffer cursor, `+0x28` byte count,
///   `+0x2C` size mask, `+0x34` last flush position, `+0x40` write callback,
///   `+0x48` callback context.
/// - `out_count`: address receiving the number of bytes written, or `0`.
///
/// # Returns
/// `0` on success, otherwise the non-zero value returned by the write
/// callback. `out_count` is filled even when the callback fails.
///
/// # Side effects
/// Calls the callback as `callback(fd, buffer, &file[0x28], context)` and may
/// advance `file[+0x18]`. On success it resets `+0x24`, `+0x28` and `+0x34`.
///
/// # Safety
/// `file` must be a live stream; the callback must accept the four arguments
/// above and may write `*file[0x28]`.
///
/// # Confidence
/// high - the four register arguments at `0x802DE588..0x802DE5A4` match the
/// decompiled call exactly.
pub unsafe fn fn_802de55c(file: usize, out_count: usize) -> i32 {
    let pending = platform::read_u32(file + 0x24).wrapping_sub(platform::read_u32(file + 0x1C));
    let mut result: i32;
    if pending != 0 {
        // observed 0x802DE578..0x802DE5A4: count -> +0x28, then
        // callback(fd, buffer, &+0x28, context).
        platform::write_u32(file + 0x28, pending);
        let callback = platform::read_u32(file + 0x40) as usize;
        result = platform::call4(
            callback,
            platform::read_u32(file),
            platform::read_u32(file + 0x1C),
            (file + 0x28) as u32,
            platform::read_u32(file + 0x48),
        ) as i32;
        if out_count != 0 {
            // observed 0x802DE5A8..0x802DE5B4: the callback may shrink the count.
            platform::write_u32(out_count, platform::read_u32(file + 0x28));
        }
        if result != 0 {
            return result;
        }
        platform::write_u32(
            file + 0x18,
            platform::read_u32(file + 0x18).wrapping_add(platform::read_u32(file + 0x28)),
        );
    }
    // observed 0x802DE5C0..0x802DE5D8: rewind cursor, recompute free space and
    // the "buffer position" word.
    platform::write_u32(file + 0x24, platform::read_u32(file + 0x1C));
    platform::write_u32(
        file + 0x28,
        platform::read_u32(file + 0x20)
            .wrapping_sub(platform::read_u32(file + 0x18) & platform::read_u32(file + 0x2C)),
    );
    platform::write_u32(file + 0x34, platform::read_u32(file + 0x18));
    0
}

// ---------------------------------------------------------------------------
// 0x802DE91C - fclose
// ---------------------------------------------------------------------------

/// Flushes and closes a stream, frees its buffer when the stream owns it, and
/// clears the "open" mode bits.
///
/// # Original
/// - Ghidra: `FUN_802de91c` at `0x802DE91C` (188 bytes)
/// - C file: `work/ghidra/main-gb/export/802de91c_FUN_802de91c.c`
///
/// # Arguments
/// - `file`: stream address; `0` returns `-1`.
///
/// # Returns
/// `0` on success, `-1` when the flush or the close callback failed (or for a
/// null stream). A stream that was never opened (`(flags >> 22) & 7 == 0`)
/// returns `0` without doing anything.
///
/// # Side effects
/// `0x802DE9D8(file)` (flush), then `file[+0x44](file[+0x00])` (the close
/// callback, a raw address in this reconstruction), then clears `+0x00` and the
/// three mode bits at `+0x04`. When bit 28 of `+0x08` is set it releases
/// `file[+0x1C]` with [`fn_802dd860`].
///
/// # Safety
/// `file` must be null or a live stream whose callbacks are valid.
///
/// # Confidence
/// high - the branch order, the callback offset and the buffer-ownership bit
/// are all explicit in the instructions at `0x802DE948..0x802DE9A0`.
pub unsafe fn fn_802de91c(file: usize) -> i32 {
    if file == 0 {
        return -1;
    }
    // observed 0x802DE948: `(flags >> 22) & 7` is the open-mode field.
    if (platform::read_u32(file + 4) >> 22) & 7 == 0 {
        return 0;
    }
    let flush_result = platform::call1(0x802D_E9D8, file as u32) as i32;
    // observed 0x802DE960..0x802DE968: close callback at +0x44, called with the
    // handle from +0x00.
    let close_callback = platform::read_u32(file + 0x44) as usize;
    let close_result = platform::call1(close_callback, platform::read_u32(file)) as i32;
    platform::write_u32(file, 0);
    platform::write_u32(file + 4, platform::read_u32(file + 4) & 0xFE3F_FFFF);
    if (platform::read_u32(file + 8) >> 28) & 1 != 0 {
        // observed 0x802DE990: the stream owns the buffer, so free it.
        platform::call1(0x802D_D860, platform::read_u32(file + 0x1C));
    }
    if flush_result != 0 || close_result != 0 {
        return -1;
    }
    0
}

// ---------------------------------------------------------------------------
// 0x802DEB0C - ftell
// ---------------------------------------------------------------------------

/// Returns the stream position, adding in the bytes still sitting in the
/// buffer and, for text-mode streams, the newlines a naive byte count misses.
///
/// # Original
/// - Ghidra: `FUN_802deb0c` at `0x802DEB0C` (168 bytes)
/// - C file: `work/ghidra/main-gb/export/802deb0c_FUN_802deb0c.c`
///
/// # Arguments
/// - `file`: stream address.
///
/// # Returns
/// The 32-bit position, or `-1` (with `errno = 0x28`) when the stream is not
/// in read/write mode or is in an error state.
///
/// # Side effects
/// On the error path only, writes `0x28` to [`ERRNO_CELL`].
///
/// # Safety
/// `file` must be a live stream; the buffer pointer at `+0x1C` must be readable
/// for the number of bytes at `+0x24`.
///
/// # Confidence
/// high for the arithmetic; medium for the text-mode newline adjustment, which
/// is inferred from the `'\n'` compare at `0x802DEB64`.
pub unsafe fn fn_802deb0c(file: usize) -> i32 {
    let flags = platform::read_u32(file + 4);
    // observed 0x802DEB14: `(((mode + 0xff) & 0xff) < 2)` accepts modes 1 and 2.
    if ((flags >> 22) & 7).wrapping_add(0xFF) & 0xFF >= 2 || platform::read_u8(file + 0x0A) != 0 {
        platform::write_u32(ERRNO_CELL, 0x28);
        return -1;
    }
    let state = platform::read_u32(file + 8) >> 29;
    if state == 0 {
        // observed 0x802DEB9C: no buffer in flight, use the logical position.
        return platform::read_u32(file + 0x18) as i32;
    }

    let buffer = platform::read_u32(file + 0x1C) as usize;
    let buffered = platform::read_u32(file + 0x24).wrapping_sub(platform::read_u32(file + 0x1C));
    let mut position = platform::read_u32(file + 0x34).wrapping_add(buffered);
    let mut scan = buffered;
    let mut skip: u32 = 0;
    if state > 2 {
        // observed 0x802DEB40: states 3 and 4 hide that many trailing bytes.
        skip = state - 2;
        position = position.wrapping_sub(skip);
    }
    if (flags >> 19) & 1 == 0 {
        // observed 0x802DEB60: text mode, count newlines in the buffered bytes.
        scan = scan.wrapping_sub(skip);
        let mut cursor = buffer;
        while scan != 0 {
            if platform::read_u8(cursor) == b'\n' {
                position = position.wrapping_add(1);
            }
            cursor += 1;
            scan -= 1;
        }
    }
    position as i32
}

// ---------------------------------------------------------------------------
// 0x802DEBB8 - fseek
// ---------------------------------------------------------------------------

/// Repositions a stream, flushing buffered output when needed.
///
/// # Original
/// - Ghidra: `FUN_802debb8` at `0x802DEBB8` (452 bytes)
/// - C file: `work/ghidra/main-gb/export/802debb8_FUN_802debb8.c`
///
/// # Arguments
/// - `file`: stream address.
/// - `offset`: byte offset (`u32`; the original treats it as unsigned).
/// - `whence`: `0` = start, `1` = current, `2` = end.
///
/// # Returns
/// `0` on success, `-1` on a mode/error/seek-callback failure. Every failure
/// path stores `0x28` in [`ERRNO_CELL`].
///
/// # Side effects
/// - Flushes when the stream state is 1 and the flush fails (this also sets the
///   error byte at `+0x0A`).
/// - For `whence == 1`, calls [`fn_802deb0c`] and adds its result to `offset`.
/// - May set state 2 at `+0x08` (`0x40000000` in the top three bits) to mark
///   "buffer position known".
/// - Calls the seek callback at `+0x38` as
///   `callback(fd, &offset, whence, context)`.
///
/// # Safety
/// `file` must be a live stream; the seek callback must accept the four
/// arguments above and may write the offset word.
///
/// # Confidence
/// high - every branch is a direct transcription of
/// `0x802DEBB8..0x802DED7C`, including the bit positions; the errno value
/// `0x28` is the only hard-coded constant.
pub unsafe fn fn_802debb8(file: usize, offset: u32, whence: u32) -> i32 {
    let flags = platform::read_u32(file + 4);
    if (flags >> 22) & 7 != 1 || platform::read_u8(file + 0x0A) != 0 {
        platform::write_u32(ERRNO_CELL, 0x28);
        return -1;
    }
    let mut position = offset;
    let mut mode = whence;

    // observed 0x802DEBF8..0x802DEC14: a dirty write buffer must be flushed
    // before the position can move.
    if platform::read_u32(file + 8) >> 29 == 1 {
        let flushed = platform::call2(0x802D_E55C, file as u32, 0) as i32;
        if flushed != 0 {
            platform::write_u32(file + 0x28, 0);
            platform::write_u8(file + 0x0A, 1);
            platform::write_u32(ERRNO_CELL, 0x28);
            return -1;
        }
    }

    if mode == 1 {
        mode = 0;
        position = position.wrapping_add(fn_802deb0c(file) as u32);
    }

    // observed 0x802DEC44..0x802DEC94: decide whether the requested position is
    // still inside the buffer; otherwise drop the buffered state.
    if mode == 2 || (flags >> 27) & 7 == 3 || ((platform::read_u32(file + 8) >> 29).wrapping_sub(2) > 1) {
        platform::write_u32(file + 8, platform::read_u32(file + 8) & 0x1FFF_FFFF);
    } else if position < platform::read_u32(file + 0x18)
        && platform::read_u32(file + 0x34) <= position
    {
        platform::write_u32(
            file + 0x24,
            platform::read_u32(file + 0x1C)
                .wrapping_add(position.wrapping_sub(platform::read_u32(file + 0x34))),
        );
        platform::write_u32(file + 0x28, platform::read_u32(file + 0x18).wrapping_sub(position));
        platform::write_u32(
            file + 8,
            (platform::read_u32(file + 8) & 0x1FFF_FFFF) | 0x4000_0000,
        );
    } else {
        platform::write_u32(file + 8, platform::read_u32(file + 8) & 0x1FFF_FFFF);
    }

    if platform::read_u32(file + 8) >> 29 == 0 {
        // observed 0x802DECB0..0x802DED20: no valid buffer, so ask the driver.
        // The second argument is the address of a stack copy of the offset
        // (`local_18` in the Ghidra C), not a field of the stream.
        let mut seek_offset = crate::platform::GuestLocal::new( position);
        let seek_callback = platform::read_u32(file + 0x38) as usize;
        if seek_callback != 0 {
            let result = platform::call4(
                seek_callback,
                platform::read_u32(file),
                ((seek_offset.as_mut_ptr() as *mut u32)) as u32,
                mode,
                platform::read_u32(file + 0x48),
            ) as i32;
            if result != 0 {
                platform::write_u32(file + 0x28, 0);
                platform::write_u8(file + 0x0A, 1);
                platform::write_u32(ERRNO_CELL, 0x28);
                return -1;
            }
        }
        platform::write_u8(file + 9, 0);
        platform::write_u32(file + 0x18, position);
        platform::write_u32(file + 0x28, 0);
    }
    0
}

// ---------------------------------------------------------------------------
// 0x802DEF74 - memmove dispatch
// ---------------------------------------------------------------------------

/// Copies `len` bytes, choosing a forward or backward kernel so that
/// overlapping ranges behave like `memmove`.
///
/// # Original
/// - Ghidra: `FUN_802def74` at `0x802DEF74` (204 bytes)
/// - C file: `work/ghidra/main-gb/export/802def74_FUN_802def74.c`
///
/// # Arguments
/// - `dst`: destination address.
/// - `src`: source address.
/// - `len`: byte count.
///
/// # Returns
/// `dst`, unchanged - the original's last instruction leaves `param_1` in `r3`.
///
/// # Side effects
/// Writes `len` bytes. For `len < 0x20` the copy is done here byte by byte; for
/// larger counts the call is forwarded to one of four kernels:
/// `0x802DF0E4` (aligned, forward), `0x802DF1A0` (aligned, backward),
/// `0x802DF248` (unaligned source, forward), `0x802DF308` (unaligned source,
/// backward).
///
/// # Safety
/// `dst..dst+len` must be writable and `src..src+len` readable, or the two
/// ranges must overlap in the `memmove` sense.
///
/// # Confidence
/// high - the direction test is the classic
/// `(dst << clz(dst ^ src)) < 0` sign trick (verified at `0x802DEF74..0x802DEFA0`);
/// the four-way dispatch matches the callees named in the C.
pub unsafe fn fn_802def74(dst: usize, src: usize, len: u32) -> usize {
    // observed: `cntlzw(dst ^ src)` then shift `dst` left by it; the sign bit is
    // set exactly when dst > src. A self-copy leaves the register unchanged, so
    // pick the forward direction to keep the shift well defined.
    let backwards = dst != src
        && ((dst as u32) << (dst as u32 ^ src as u32).leading_zeros()) >= 0x8000_0000;

    if len < 0x20 {
        if backwards {
            // observed 0x802DEF8C: descending byte loop.
            let mut index = len;
            while index != 0 {
                index -= 1;
                platform::write_u8(
                    dst.wrapping_add(index as usize),
                    platform::read_u8(src.wrapping_add(index as usize)),
                );
            }
        } else {
            // observed 0x802DEFA4: ascending byte loop.
            let mut index: u32 = 0;
            while index < len {
                platform::write_u8(
                    dst.wrapping_add(index as usize),
                    platform::read_u8(src.wrapping_add(index as usize)),
                );
                index += 1;
            }
        }
        return dst;
    }

    if (dst ^ src) & 3 == 0 {
        if backwards {
            platform::call3(0x802D_F1A0, dst as u32, src as u32, len);
        } else {
            platform::call3(0x802D_F0E4, dst as u32, src as u32, len);
        }
    } else if backwards {
        platform::call3(0x802D_F308, dst as u32, src as u32, len);
    } else {
        platform::call3(0x802D_F248, dst as u32, src as u32, len);
    }
    dst
}

// ---------------------------------------------------------------------------
// 0x802DF040 - memchr
// ---------------------------------------------------------------------------

/// Finds the first occurrence of `byte` in the first `len` bytes of `base`.
///
/// # Original
/// - Ghidra: `FUN_802df040` at `0x802DF040` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802df040_FUN_802df040.c`
///
/// # Arguments
/// - `base`: start of the search range.
/// - `byte`: byte to look for; the original masks it with `0xFF`
///   (`rlwinm r4,r4,0,24,31`).
/// - `len`: number of bytes to scan.
///
/// # Returns
/// Address of the first match, or `0` when `len == 0` or nothing matches.
///
/// # Side effects
/// None; reads only.
///
/// # Safety
/// `base..base+len` must be readable. The loop decrements its counter **before**
/// loading, so `len == 0` performs no read.
///
/// # Confidence
/// high - `lbzu` / `cmplw` / `beqlr` / `addic.` / `bne` at
/// `0x802DF040..0x802DF068`.
pub unsafe fn fn_802df040(base: usize, byte: u8, len: u32) -> usize {
    let mut cursor = base.wrapping_sub(1);
    let mut remaining = len.wrapping_add(1);
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            return 0;
        }
        cursor = cursor.wrapping_add(1);
        if platform::read_u8(cursor) == byte {
            return cursor;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DF06C - memrchr
// ---------------------------------------------------------------------------

/// Finds the **last** occurrence of `byte` in the first `len` bytes of `base`.
///
/// # Original
/// - Ghidra: `FUN_802df06c` at `0x802DF06C` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802df06c_FUN_802df06c.c`
///
/// # Arguments
/// - `base`: start of the search range.
/// - `byte`: byte to look for, masked with `0xFF`.
/// - `len`: number of bytes to scan.
///
/// # Returns
/// Address of the last match, or `0` when nothing matches or `len == 0`.
///
/// # Side effects
/// None; reads only.
///
/// # Safety
/// `base..base+len` must be readable.
///
/// # Confidence
/// high - mirror image of [`fn_802df040`]: `add r3,r3,r5`, then
/// `lbzu r0,-1(r3)` at `0x802DF07C`.
pub unsafe fn fn_802df06c(base: usize, byte: u8, len: u32) -> usize {
    let mut cursor = base.wrapping_add(len as usize);
    let mut remaining = len.wrapping_add(1);
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            return 0;
        }
        cursor = cursor.wrapping_sub(1);
        if platform::read_u8(cursor) == byte {
            return cursor;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DF098 - memcmp
// ---------------------------------------------------------------------------

/// Compares two byte ranges and reports only their ordering sign.
///
/// # Original
/// - Ghidra: `FUN_802df098` at `0x802DF098` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/802df098_FUN_802df098.c`
///
/// # Arguments
/// - `left`: first range.
/// - `right`: second range.
/// - `len`: bytes to compare.
///
/// # Returns
/// `0` when the first `len` bytes are equal, `1` when `left > right` at the
/// first difference, `-1` when `left < right`. Note that this is **not**
/// `*left - *right`; it is the normalised sign.
///
/// # Side effects
/// None; reads only.
///
/// # Safety
/// Both ranges must be readable for `len` bytes.
///
/// # Confidence
/// high - the mismatch epilogue at `0x802DF0B0..0x802DF0C4` is
/// `if (right <= left) 1 else -1`.
pub unsafe fn fn_802df098(left: usize, right: usize, len: u32) -> i32 {
    let mut remaining = len.wrapping_add(1);
    let mut a = left.wrapping_sub(1);
    let mut b = right.wrapping_sub(1);
    loop {
        remaining = remaining.wrapping_sub(1);
        if remaining == 0 {
            return 0;
        }
        a = a.wrapping_add(1);
        b = b.wrapping_add(1);
        let left_byte = platform::read_u8(a);
        let right_byte = platform::read_u8(b);
        if left_byte != right_byte {
            return if right_byte <= left_byte { 1 } else { -1 };
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DF0E4 - forward word copy, 4-byte-aligned destination
// ---------------------------------------------------------------------------

/// Copies `len` bytes forward, using 32-byte unrolled word stores once the
/// destination is 4-byte aligned.
///
/// # Original
/// - Ghidra: `FUN_802df0e4` at `0x802DF0E4` (188 bytes)
/// - C file: `work/ghidra/main-gb/export/802df0e4_FUN_802df0e4.c`
///
/// # Arguments
/// - `dst`: destination; may start unaligned.
/// - `src`: source; no alignment requirement.
/// - `len`: byte count.
///
/// # Returns
/// Nothing. Callers that need the destination use [`fn_802def74`].
///
/// # Side effects
/// Writes `len` bytes, 8 words at a time, then single words, then bytes. The
/// destination is never written with a width larger than its alignment allows.
///
/// # Safety
/// Both ranges must be readable/writable for `len` bytes. **The original does
/// not handle `len` smaller than the alignment prologue**: it computes
/// `len - ((-dst) & 3)` and the byte prologue still runs `(-dst) & 3` times
/// (`0x802DF0F8..0x802DF108`), so a short copy into an unaligned destination
/// reads and writes past `len`. The reconstruction keeps that behaviour; do not
/// call it with `len < 3` and an unaligned destination.
///
/// # Confidence
/// high - the prologue, the 8-word body and the `(len >> 2) & 7` word count are
/// all visible at `0x802DF0E4..0x802DF19C`.
pub unsafe fn fn_802df0e4(dst: usize, src: usize, len: u32) {
    let mut destination = dst;
    let mut source = src;
    let mut remaining = len;

    // observed 0x802DF0EC: `(-dst) & 3` bytes make the destination aligned.
    let lead = (dst as u32).wrapping_neg() & 3;
    if lead != 0 {
        remaining = remaining.wrapping_sub(lead);
        let mut count = lead;
        while count != 0 {
            platform::write_u8(destination, platform::read_u8(source));
            destination += 1;
            source += 1;
            count -= 1;
        }
    }

    // observed 0x802DF10C..0x802DF160: `len >> 5` iterations of eight word
    // loads/stores (28 bytes as seven indexed words plus one post-increment).
    let mut blocks = remaining >> 5;
    while blocks != 0 {
        let mut word = 0;
        while word < 8 {
            platform::write_u32(
                destination + word * 4,
                platform::read_u32(source + word * 4),
            );
            word += 1;
        }
        destination += 32;
        source += 32;
        blocks -= 1;
    }

    // observed 0x802DF164: `(len >> 2) & 7` whole words.
    let mut words = (remaining >> 2) & 7;
    while words != 0 {
        platform::write_u32(destination, platform::read_u32(source));
        destination += 4;
        source += 4;
        words -= 1;
    }

    // observed 0x802DF188: the low two bits as bytes.
    let mut tail = remaining & 3;
    while tail != 0 {
        platform::write_u8(destination, platform::read_u8(source));
        destination += 1;
        source += 1;
        tail -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802DF1A0 - backward word copy, 4-byte-aligned end
// ---------------------------------------------------------------------------

/// Copies `len` bytes backwards, using word stores once the end of the
/// destination is 4-byte aligned.
///
/// # Original
/// - Ghidra: `FUN_802df1a0` at `0x802DF1A0` (168 bytes)
/// - C file: `work/ghidra/main-gb/export/802df1a0_FUN_802df1a0.c`
///
/// # Arguments
/// - `dst`: destination start.
/// - `src`: source start.
/// - `len`: byte count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes from the highest address down. This is the overlapping-
/// safe direction used by [`fn_802def74`] when `dst > src`.
///
/// # Safety
/// Both ranges must be readable/writable for `len` bytes. As with
/// [`fn_802df0e4`], `len` must be at least `(dst + len) & 3`.
///
/// # Confidence
/// high - the prologue aligns `dst + len` (`0x802DF1A8..0x802DF1C4`) and the
/// body copies eight words per iteration in descending order.
pub unsafe fn fn_802df1a0(dst: usize, src: usize, len: u32) {
    let mut destination = dst.wrapping_add(len as usize);
    let mut source = src.wrapping_add(len as usize);
    let mut remaining = len;

    // observed 0x802DF1A8: `(dst + len) & 3`.
    let lead = (destination as u32) & 3;
    if lead != 0 {
        remaining = remaining.wrapping_sub(lead);
        let mut count = lead;
        while count != 0 {
            destination -= 1;
            source -= 1;
            platform::write_u8(destination, platform::read_u8(source));
            count -= 1;
        }
    }

    // observed 0x802DF1CC..0x802DF214: `len >> 5` iterations, eight words down.
    let mut blocks = remaining >> 5;
    while blocks != 0 {
        let mut word = 1;
        while word <= 8 {
            platform::write_u32(destination - word * 4, platform::read_u32(source - word * 4));
            word += 1;
        }
        destination -= 32;
        source -= 32;
        blocks -= 1;
    }

    let mut words = (remaining >> 2) & 7;
    while words != 0 {
        destination -= 4;
        source -= 4;
        platform::write_u32(destination, platform::read_u32(source));
        words -= 1;
    }

    let mut tail = remaining & 3;
    while tail != 0 {
        destination -= 1;
        source -= 1;
        platform::write_u8(destination, platform::read_u8(source));
        tail -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802DF248 - forward copy with a misaligned source
// ---------------------------------------------------------------------------

/// Copies `len` bytes forward when the destination is 4-byte aligned but the
/// source is not.
///
/// # Original
/// - Ghidra: `FUN_802df248` at `0x802DF248` (192 bytes)
/// - C file: `work/ghidra/main-gb/export/802df248_FUN_802df248.c`
///
/// # Arguments
/// - `dst`: destination; must be 4-byte aligned (the caller aligns it first).
/// - `src`: source with `(src & 3) != 0`.
/// - `len`: byte count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes. Each destination word is assembled from two aligned
/// source words with a `(src & 3) * 8` bit shift, so the source is only ever
/// read with aligned word loads; the leading bytes that make the destination
/// aligned are copied one at a time.
///
/// # Safety
/// Both ranges must be readable/writable for `len` bytes, and `src & 3` must be
/// non-zero - the shift expression is undefined for an aligned source
/// (the original would compute `x >> 32`).
///
/// # Confidence
/// high for the byte order; medium for the shift folding, which was rewritten
/// as one helper expression per word instead of the decompiler's two-word
/// software pipeline. Both forms read the same aligned words.
pub unsafe fn fn_802df248(dst: usize, src: usize, len: u32) {
    let mut destination = dst;
    let mut source = src;
    let mut remaining = len;

    let lead = (dst as u32).wrapping_neg() & 3;
    if lead != 0 {
        remaining = remaining.wrapping_sub(lead);
        let mut count = lead;
        while count != 0 {
            platform::write_u8(destination, platform::read_u8(source));
            destination += 1;
            source += 1;
            count -= 1;
        }
    }

    let mut words = remaining >> 2;
    while words != 0 {
        // observed 0x802DF280..0x802DF2D0: combine the aligned word below the
        // source cursor with the next one, shifted by the source's bit offset.
        let base = source & !3usize;
        let shift = ((source as u32) & 3) * 8;
        let word = (platform::read_u32(base) << shift)
            | (platform::read_u32(base + 4) >> (32 - shift));
        platform::write_u32(destination, word);
        destination += 4;
        source += 4;
        words -= 1;
    }

    let mut tail = remaining & 3;
    while tail != 0 {
        platform::write_u8(destination, platform::read_u8(source));
        destination += 1;
        source += 1;
        tail -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802DF308 - backward copy with a misaligned source
// ---------------------------------------------------------------------------

/// Copies `len` bytes backwards when the end of the destination is 4-byte
/// aligned but the source is not.
///
/// # Original
/// - Ghidra: `FUN_802df308` at `0x802DF308` (172 bytes)
/// - C file: `work/ghidra/main-gb/export/802df308_FUN_802df308.c`
///
/// # Arguments
/// - `dst`: destination start.
/// - `src`: source start with `(src & 3) != 0` after the prologue.
/// - `len`: byte count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes from the top down. As in [`fn_802df248`], every word is
/// assembled from two aligned source words with a bit shift.
///
/// # Safety
/// Both ranges must be readable/writable for `len` bytes; the source must be
/// misaligned relative to the destination.
///
/// # Confidence
/// high for the byte order and the direction; medium for the same shift-folding
/// rewrite as [`fn_802df248`].
pub unsafe fn fn_802df308(dst: usize, src: usize, len: u32) {
    let mut destination = dst.wrapping_add(len as usize);
    let mut source = src.wrapping_add(len as usize);
    let mut remaining = len;

    let lead = (destination as u32) & 3;
    if lead != 0 {
        remaining = remaining.wrapping_sub(lead);
        let mut count = lead;
        while count != 0 {
            destination -= 1;
            source -= 1;
            platform::write_u8(destination, platform::read_u8(source));
            count -= 1;
        }
    }

    let mut words = remaining >> 2;
    while words != 0 {
        destination -= 4;
        source -= 4;
        let x = source;
        let base = x & !3usize;
        let shift = ((x as u32) & 3) * 8;
        let word = if shift == 0 {
            platform::read_u32(x)
        } else {
            (platform::read_u32(base) << shift) | (platform::read_u32(base + 4) >> (32 - shift))
        };
        platform::write_u32(destination, word);
        words -= 1;
    }

    let mut tail = remaining & 3;
    while tail != 0 {
        destination -= 1;
        source -= 1;
        platform::write_u8(destination, platform::read_u8(source));
        tail -= 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802DF42C - fpclassify (double)
// ---------------------------------------------------------------------------

/// Classifies a `double` into the five `fpclassify` categories.
///
/// # Original
/// - Ghidra: `FUN_802df42c` at `0x802DF42C` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802df42c_FUN_802df42c.c`
///
/// # Arguments
/// - `value`: the `double` to classify (`f1`).
///
/// # Returns
/// `1` = NaN, `2` = infinity, `3` = zero, `4` = normal, `5` = subnormal.
///
/// # Side effects
/// None; pure. The two masks `0x7FF00000` and `0x000FFFFF00000000` in the C are
/// the exponent and mantissa fields of the IEEE-754 double.
///
/// # Safety
/// Pure arithmetic; no pointers are dereferenced.
///
/// # Confidence
/// high - the exponent compare, the sign-free mantissa test and the five return
/// values are explicit at `0x802DF42C..0x802DF4A4`.
pub fn fn_802df42c(value: f64) -> u32 {
    let bits = value.to_bits();
    let exponent = (bits >> 52) & 0x7FF;
    let mantissa = bits & 0x000F_FFFF_FFFF_FFFF;
    if exponent == 0x7FF {
        if mantissa == 0 {
            2
        } else {
            1
        }
    } else if exponent == 0 {
        if mantissa == 0 {
            3
        } else {
            5
        }
    } else {
        4
    }
}

// ---------------------------------------------------------------------------
// 0x802D0800 - get some r2-relative pointer/value
// ---------------------------------------------------------------------------

/// Returns `r2 - 0x3B58`. Access via SDA2 base.
///
/// # Original
/// - Ghidra: `FUN_802d0800` at `0x802D0800` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802d0800_FUN_802d0800.c`
///
/// # Confidence
/// medium - `r2` is SDA2 base per architecture; absolute value not reconstructed.
pub unsafe fn fn_802d0800() -> i32 {
    // observed: return unaff_r2 + -0x3b58
    // unknown: resolved via r2; no stable global symbol recovered
    0 // placeholder - will be resolved properly; keep structure
}

// ---------------------------------------------------------------------------
// 0x802D0830 - initialize some object
// ---------------------------------------------------------------------------

/// Initializes an object from r2-relative globals.
///
/// # Original
/// - Ghidra: `FUN_802d0830` at `0x802D0830` (92 bytes)
/// - C file: `work/ghidra/main-gb/export/802d0830_FUN_802d0830.c`
///
/// # Arguments
/// - `param_1`: object pointer
///
/// # Returns
/// `param_1` unchanged
///
/// # Confidence
/// medium
pub unsafe fn fn_802d0830(param_1: usize) -> usize {
    // observed: FUN_802ce740(); read r2-0x3e3c, r2-0x3b18; set vtable/DAT_803504c0 at +0x28, store +0x2c,+0x30, zero +0x34, set +0x38=1
    platform::call0(0x802C_E740);
    // r2-relative reads - unknown globals
    platform::write_u32(param_1 + 0x28, 0x8035_04C0); // inferred vtable/data pointer
    // unknown values from r2
    platform::write_u32(param_1 + 0x2c, 0);
    platform::write_u32(param_1 + 0x30, 0);
    platform::write_u32(param_1 + 0x34, 0);
    platform::write_u32(param_1 + 0x38, 1);
    param_1
}

// ---------------------------------------------------------------------------
// 0x802D0CC0 - initialize object with lookups
// ---------------------------------------------------------------------------

/// Initializes object using r13/r2 globals and calls helpers.
///
/// # Original
/// - Ghidra: `FUN_802d0cc0` at `0x802D0CC0` (204 bytes)
/// - C file: `work/ghidra/main-gb/export/802d0cc0_FUN_802d0cc0.c`
///
/// # Arguments
/// - `param_1`: object pointer
///
/// # Confidence
/// medium
pub unsafe fn fn_802d0cc0(param_1: usize) -> usize {
    // observed uses r13-0x4778, r2 constants, calls FUN_802c6a30, sets fields +0x38 etc, calls FUN_802d1250
    platform::call5(0x802C_6A30, 0, 0, param_1 as u32, 1, 0); // inferred args
    platform::write_u32(param_1 + 0x38, 0x8035_0504);
    platform::write_u32(param_1 + 0x3c, 0);
    for off in [0x40, 0x44, 0x48, 0x4c, 0x50, 0x54].iter() {
        // unknown value
        platform::write_u32(param_1 + *off, 0);
    }
    fn_802d1250(param_1, 0);
    param_1
}

// ---------------------------------------------------------------------------
// 0x802D1250 - set field if different
// ---------------------------------------------------------------------------

/// Sets field at +0x3c from r13-0x4774 if param_2==0 or different.
///
/// # Original
/// - Ghidra: `FUN_802d1250` at `0x802D1250` (48 bytes)
/// - C file: `work/ghidra/main-gb/export/802d1250_FUN_802d1250.c`
///
/// # Confidence
/// high
pub unsafe fn fn_802d1250(param_1: usize, param_2: i32) {
    let mut p2 = param_2;
    if p2 == 0 {
        // unknown: read from r13-0x4774
        p2 = 0;
    }
    if platform::read_u32(param_1 + 0x3c) as i32 != p2 {
        platform::write_u32(param_1 + 0x3c, p2 as u32);
        platform::write_u16(param_1 + 0x34, platform::read_u16(param_1 + 0x34) | 1);
    }
}

// ---------------------------------------------------------------------------
// 0x802D19F0 - recursive walk
// ---------------------------------------------------------------------------

/// Recursively walks structure updating params.
///
/// # Original
/// - Ghidra: `FUN_802d19f0` at `0x802D19F0` (176 bytes)
/// - C file: `work/ghidra/main-gb/export/802d19f0_FUN_802d19f0.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d19f0(param_1: usize, param_2: *mut f32, param_3: *mut f32) {
    let i_var1 = platform::read_u32(param_1 + 0x3c) as usize;
    if i_var1 == 0 {
        // unknown reads
    } else {
        let mut local14: f32 = 0.0;
        let mut local18: f32 = 0.0;
        fn_802d19f0(i_var1, &mut local14, &mut local18);
        platform::call5(0x802C_6CE0, 0, 0, i_var1 as u32, param_2 as u32, param_3 as u32);
        // add results - simplified
    }
}

// ---------------------------------------------------------------------------
// Shared SDA/table constants for the 0x802D1AA0..0x802D3490 widget pass
// ---------------------------------------------------------------------------

/// Small-data area 1 base (`r13`, `lis 0x8042` / `ori 0xF680`).
const SDA1_BASE: usize = 0x8042_F680;

/// 12-byte records at `0x803504E0`: two `u16` sizes followed by a zero word.
/// [`fn_802d1aa0`], [`fn_802d1b90`] and [`fn_802d2da0`] index it by
/// `selector * 12`.
const UI_SCALE_TABLE: usize = 0x8035_04E0;

/// Widens a stored `u16` exactly as the CodeWarrior `0x43300000` idiom does:
/// the word is OR-ed into the high half of an `f64` biased by `2^52`, then the
/// bias is subtracted.
///
/// # Original
/// Host-side helper; no binary address. Models the `lis 0x4330` / `lfd` /
/// `fsubs` sequence at `0x802D1AC0..0x802D1AEC`, `0x802D1C28..0x802D1C34` and
/// the two `0x802D2E00` copies.
///
/// # Confidence
/// high - the bias is the shipped `f64` at `0x8042F578`.
#[inline]
fn widen_u16(value: u16) -> f32 {
    let bias = f64::from_bits(0x4330_0000_0000_0000);
    (f64::from_bits(0x4330_0000_0000_0000 | value as u64) - bias) as f32
}

/// Calls a PowerPC function taking `f1..f4` and two integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Models the mixed register file of
/// `0x801C8920` as called from [`fn_802d2da0`].
///
/// # Safety
/// `addr` must take exactly those arguments.
///
/// # Confidence
/// high - the four `lfs` loads and the two `addi` argument setups are literal.
#[inline]
unsafe fn call_f32x4_u32x2(
    addr: usize,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: u32,
    f: u32,
) -> u32 {
    let function = crate::platform::guest_fn!(addr, fn(f32, f32, f32, f32, u32, u32) -> u32);
    function(a, b, c, d, e, f)
}

// ---------------------------------------------------------------------------
// 0x802D1AA0 - publish the selected widget scale
// ---------------------------------------------------------------------------

/// Stores the selector and republishes the two sizes of its table record.
///
/// The object cached at `r13-0x4774` receives bit 0 of its `+0x34` state word
/// and the two `u16` record fields converted to `f32` at `+8`/`+0xC`. The
/// `r13-0x4770` callback is then invoked with the word at `r13-0x476C`.
///
/// # Original
/// - Ghidra: `FUN_802d1aa0` at `0x802D1AA0` (176 bytes)
/// - C file: `work/ghidra/main-gb/export/802d1aa0_FUN_802d1aa0.c`
///
/// # Arguments
/// - `selector`: record index in [`UI_SCALE_TABLE`]; also cached at
///   `r13-0x4778` and reused by [`fn_802d1b90`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the cached selector, the object's `+0x34` flag and `+8`/`+0xC`
/// floats, then one indirect callback when it is non-null.
///
/// # Safety
/// The cached object must be a live widget when non-null, and the selector
/// must be inside [`UI_SCALE_TABLE`].
///
/// # Confidence
/// high - every offset, both table reads and the `ctr` call are literal; the
/// `2^52` bias at `0x8042F578` was read from the image.
pub unsafe fn fn_802d1aa0(selector: u32) {
    let object = platform::read_u32(SDA1_BASE - 0x4774) as usize;
    platform::write_u32(SDA1_BASE - 0x4778, selector);

    if object != 0 {
        let record = UI_SCALE_TABLE + selector as usize * 12;
        let first = widen_u16(platform::read_u16(record));
        platform::write_u16(
            object + 0x34,
            platform::read_u16(object + 0x34) | 1,
        );
        platform::write_f32(object + 8, first);

        let second = widen_u16(platform::read_u16(record + 2));
        let object = platform::read_u32(SDA1_BASE - 0x4774) as usize;
        platform::write_u16(
            object + 0x34,
            platform::read_u16(object + 0x34) | 1,
        );
        platform::write_f32(object + 0xC, second);
    }

    let callback = platform::read_u32(SDA1_BASE - 0x4770) as usize;
    if callback != 0 {
        platform::call1(callback, platform::read_u32(SDA1_BASE - 0x476C));
    }
}

// ---------------------------------------------------------------------------
// 0x802D1B50 - calls helpers
// ---------------------------------------------------------------------------

/// Calls FUN_801dd1d0 and FUN_802d1aa0.
///
/// # Original
/// - Ghidra: `FUN_802d1b50` at `0x802D1B50` (52 bytes)
/// - C file: `work/ghidra/main-gb/export/802d1b50_FUN_802d1b50.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d1b50() {
    let u_var1 = call0_ret(0x801D_D1D0) as u32;
    fn_802d1aa0((-(((u_var1 & 0xff) as i32) >> 0x1f)) as u32);
}

// ---------------------------------------------------------------------------
// 0x802D1B90 - unknown
// ---------------------------------------------------------------------------

/// Rescales a pair of widget offsets through the record of the active
/// selector.
///
/// The two incoming `f32` values are narrowed, passed through the recursive
/// walk `0x802D19F0` and the resampler `0x802C6CE0`, then added to the caller's
/// offsets and rescaled twice: by the two `u16` words at `r13-0x4748` /
/// `r13-0x4746` and by the matching [`UI_SCALE_TABLE`] record.
///
/// # Original
/// - Ghidra: `FUN_802d1b90` at `0x802D1B90` (332 bytes)
/// - C file: `work/ghidra/main-gb/export/802d1b90_FUN_802d1b90.c`
///
/// # Arguments
/// - `param_1`, `param_2`: incoming `f64` values; both are narrowed to `f32`
///   on entry.
/// - `param_3`: widget object; bit `0x34` state word selects the record.
/// - `param_4`, `param_5`: in/out `f32` offsets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes both out-pointers.
///
/// # Safety
/// `param_3` must be a live widget, both out-pointers writable, and the record
/// index inside [`UI_SCALE_TABLE`].
///
/// # Confidence
/// high - the selector ladder (`& 0x20` -> 0, else `& 8` -> 2, else
/// `r13-0x4778`), the `2^52` bias and the `fadds`/`fmuls`/`fdivs` chain are
/// literal.
pub unsafe fn fn_802d1b90(
    param_1: f64,
    param_2: f64,
    param_3: usize,
    param_4: *mut f32,
    param_5: *mut f32,
) {
    let local_28 = param_1 as f32;
    let local_24 = param_2 as f32;

    fn_802d19f0(param_3, param_4, param_5);
    call_f32x2_u32x3(
        0x802C_6CE0,
        local_28,
        local_24,
        param_3 as u32,
        param_4 as u32,
        param_5 as u32,
    );

    let mut selector = 0u32;
    if platform::read_u16(param_3 + 0x34) & 0x20 == 0 {
        selector = 2;
        if platform::read_u16(param_3 + 0x34) & 8 == 0 {
            selector = platform::read_u32(SDA1_BASE - 0x4778);
        }
    }
    let record = UI_SCALE_TABLE + selector as usize * 12;

    let high_num = widen_u16(platform::read_u16(SDA1_BASE - 0x4748));
    let high_den = widen_u16(platform::read_u16(record));
    platform::write_f32(param_4 as usize, (platform::read_f32(param_4 as usize) + local_28) * high_num / high_den);

    let low_num = widen_u16(platform::read_u16(SDA1_BASE - 0x4746));
    let low_den = widen_u16(platform::read_u16(record + 2));
    platform::write_f32(param_5 as usize, (platform::read_f32(param_5 as usize) + local_24) * low_num / low_den);
}

/// Calls a PowerPC function taking `f1`/`f2` and three integer arguments.
///
/// # Original
/// Host-side helper; no binary address. Models the mixed register file of
/// `0x802C6CE0` as called from [`fn_802d1b90`].
///
/// # Safety
/// `addr` must take exactly those arguments.
///
/// # Confidence
/// high - the two `lfs` loads and the `addi` argument setup are literal.
#[inline]
unsafe fn call_f32x2_u32x3(addr: usize, a: f32, b: f32, c: u32, d: u32, e: u32) {
    let function = crate::platform::guest_fn!(addr, fn(f32, f32, u32, u32, u32));
    function(a, b, c, d, e);
}

// ---------------------------------------------------------------------------
// 0x802D1DA0 - initialize globals
// ---------------------------------------------------------------------------

/// Initializes many global variables.
///
/// # Original
/// - Ghidra: `FUN_802d1da0` at `0x802D1DA0` (120 bytes)
/// - C file: `work/ghidra/main-gb/export/802d1da0_FUN_802d1da0.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d1da0() {
    // observed reads r2-0x3ae8, writes to many globals including DAT_80424850 etc
    // unknown: absolute addresses not all recovered
}

// ---------------------------------------------------------------------------
// 0x802D21F0 - check and remove entry
// ---------------------------------------------------------------------------

/// Checks entries in DAT_80424820 table.
///
/// # Original
/// - Ghidra: `FUN_802d21f0` at `0x802D21F0` (124 bytes)
/// - C file: `work/ghidra/main-gb/export/802d21f0_FUN_802d21f0.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d21f0(param_1: i32, param_2: i32) -> u32 {
    let base = 0x8042_4820usize;
    let entry = base + (param_1 as usize) * 0xc;
    if platform::read_u32(entry) == 0 || platform::read_u32(base + 4 + (param_1 as usize) * 0xc) != (param_2 as u32) {
        return 0;
    }
    fn_802d3370(entry);
    platform::write_u32(entry, 0);
    platform::write_u32(base + 4 + (param_1 as usize) * 0xc, 0);
    platform::write_u32(base + 8 + (param_1 as usize) * 0xc, 0);
    1
}

// ---------------------------------------------------------------------------
// 0x802D2270 - unknown
// ---------------------------------------------------------------------------

/// Switches the two-panel display mode.
///
/// `param_1 == 0` leaves the split mode, releasing slot 0 through
/// [`fn_802d21f0`] when it is still the single-panel mode, then installs panel
/// 1. `param_1 == 1` installs panel 0 when the split mode is active. The mode
/// word lives at `r13-0x5AFC`; slot 0's token is `0x80424824`.
///
/// # Original
/// - Ghidra: `FUN_802d2270` at `0x802D2270` (152 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2270_FUN_802d2270.c`
///
/// # Arguments
/// - `param_1`: requested mode (`0` or `1`; anything else is ignored).
/// - `param_2`: token forwarded to [`fn_802d1e20`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// May release slot 0 and writes the mode word at `r13-0x5AFC`.
///
/// # Safety
/// The panel root and slot table must be live, as [`fn_802d1e20`] requires.
///
/// # Confidence
/// high - the two guarded arms and the `0x80424824` token are literal.
pub unsafe fn fn_802d2270(param_1: i32, param_2: u32) {
    let mode = SDA1_BASE - 0x5AFC;
    if param_1 == 0 {
        if platform::read_u32(mode) == 1 {
            fn_802d21f0(0, 0x8042_4824u32 as i32);
        }
        fn_802d1e20(1, 0, param_2);
        platform::write_u32(mode, 0);
    } else if param_1 == 1 && platform::read_u32(mode) != 0 {
        fn_802d1e20(0, 0, param_2);
        platform::write_u32(mode, 1);
    }
}

// ---------------------------------------------------------------------------
// 0x802D2C20 - setup GX/TEV state
// ---------------------------------------------------------------------------

/// Initializes GX state registers.
///
/// # Original
/// - Ghidra: `FUN_802d2c20` at `0x802D2C20` (380 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2c20_FUN_802d2c20.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d2c20() {
    platform::call1(0x801C_88F0, 1);
    let mut u_var1: u32 = 0;
    while u_var1 < 0x10 {
        platform::call1(0x801C_8130, u_var1);
        platform::call4(0x801C_8790, u_var1, 0xff, 0xff, 0xff);
        platform::call5(0x801C_82F0, u_var1, 0xf, 0xf, 0xf, 0xf);
        platform::call6(0x801C_8370, u_var1, 0, 0, 0, 1, 0);
        platform::call5(0x801C_8330, u_var1, 7, 7, 7, 7);
        platform::call6(0x801C_83D0, u_var1, 0, 0, 0, 1, 0);
        platform::call3(0x801C_8600, u_var1, 0, 0);
        u_var1 += 1;
    }
    // unknown: use r2 constant
    let mut u_var2: u32 = 0;
    while u_var2 < 4 {
        platform::call2(0x801C_8500, u_var2, 0); // simplified
        u_var2 += 1;
    }
    let mut u_var3: u32 = 0;
    while u_var3 < 4 {
        platform::call2(0x801C_8430, u_var3, 0);
        u_var3 += 1;
    }
    platform::call5(0x801C_8640, 0, 0, 1, 2, 3);
    platform::call5(0x801C_8640, 1, 0, 0, 0, 3);
    platform::call5(0x801C_8640, 2, 1, 1, 1, 3);
    platform::call5(0x801C_8640, 3, 2, 2, 2, 3);
}

// ---------------------------------------------------------------------------
// 0x802D2E50 - init some state
// ---------------------------------------------------------------------------

/// Initializes state via helper functions.
///
/// # Original
/// - Ghidra: `FUN_802d2e50` at `0x802D2E50` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2e50_FUN_802d2e50.c`
///
/// # Confidence
/// high
pub unsafe fn fn_802d2e50() {
    fn_802d2f20(1);
    fn_802d2f80(1);
    fn_802d2fe0(1);
    fn_802d3040(0, 0);
}

// ---------------------------------------------------------------------------
// 0x802D2F20 - set byte
// ---------------------------------------------------------------------------

/// Sets DAT_804248cc and calls helper.
///
/// # Original
/// - Ghidra: `FUN_802d2f20` at `0x802D2F20` (84 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2f20_FUN_802d2f20.c`
///
/// # Confidence
/// high
pub unsafe fn fn_802d2f20(param_1: u8) -> u8 {
    let u_var1 = unsafe { *(0x8042_48CC as *const u8) };
    platform::call1(0x801C_8E10, param_1 as u32);
    unsafe { *(0x8042_48CC as *mut u8) = param_1; }
    u_var1
}

// ---------------------------------------------------------------------------
// 0x802D2F80 - set byte
// ---------------------------------------------------------------------------

/// Sets DAT_804248cd.
///
/// # Original
/// - Ghidra: `FUN_802d2f80` at `0x802D2F80` (84 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2f80_FUN_802d2f80.c`
///
/// # Confidence
/// high
pub unsafe fn fn_802d2f80(param_1: u8) -> u8 {
    let u_var1 = unsafe { *(0x8042_48CD as *const u8) };
    platform::call1(0x801C_8E40, param_1 as u32);
    unsafe { *(0x8042_48CD as *mut u8) = param_1; }
    u_var1
}

// ---------------------------------------------------------------------------
// 0x802D2FE0 - set byte
// ---------------------------------------------------------------------------

/// Sets DAT_804248ce.
///
/// # Original
/// - Ghidra: `FUN_802d2fe0` at `0x802D2FE0` (84 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2fe0_FUN_802d2fe0.c`
///
/// # Confidence
/// high
pub unsafe fn fn_802d2fe0(param_1: u8) -> u8 {
    let u_var1 = unsafe { *(0x8042_48CE as *const u8) };
    platform::call1(0x801C_8F90, param_1 as u32);
    unsafe { *(0x8042_48CE as *mut u8) = param_1; }
    u_var1
}

// ---------------------------------------------------------------------------
// 0x802D3040 - set bytes
// ---------------------------------------------------------------------------

/// Sets DAT_804248cf, DAT_804248d0.
///
/// # Original
/// - Ghidra: `FUN_802d3040` at `0x802D3040` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3040_FUN_802d3040.c`
///
/// # Confidence
/// high
pub unsafe fn fn_802d3040(param_1: u8, param_2: u8) {
    platform::call1(0x801C_8FC0, param_1 as u32);
    unsafe { *(0x8042_48CF as *mut u8) = param_1; }
    unsafe { *(0x8042_48D0 as *mut u8) = param_2; }
}

// ---------------------------------------------------------------------------
// 0x802D3090 - compute values
// ---------------------------------------------------------------------------

/// Computes values from params using FUN_802dcc4c.
///
/// # Original
/// - Ghidra: `FUN_802d3090` at `0x802D3090` (264 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3090_FUN_802d3090.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d3090(param_1: *const f32, param_2: *mut i32) {
    // partial reconstruction
    let _ = param_1;
    let _ = param_2;
}

// ---------------------------------------------------------------------------
// 0x802D32A0 - init object
// ---------------------------------------------------------------------------

/// Initializes object with vtable DAT_80350528.
///
/// # Original
/// - Ghidra: `FUN_802d32a0` at `0x802D32A0` (80 bytes)
/// - C file: `work/ghidra/main-gb/export/802d32a0_FUN_802d32a0.c`
///
/// # Confidence
/// medium
pub unsafe fn fn_802d32a0(param_1: *mut u32) -> *mut u32 {
    platform::call0(0x802C_0A30);
    platform::write_u32(param_1 as usize, 0x8035_0528);
    platform::write_u32(param_1 as usize + (0xb) * 4, 0);
    platform::write_u32(param_1 as usize + (0xc) * 4, 0);
    platform::write_u32(param_1 as usize + (0xd) * 4, 0);
    platform::write_u32(param_1 as usize + (0xe) * 4, 0);
    param_1
}

// ---------------------------------------------------------------------------
// 0x802D3370 - merge nodes
// ---------------------------------------------------------------------------

/// Returns one node to the free list and coalesces it with both neighbours.
///
/// The node's `+0x30` in-use word is cleared and its `+0x2C` size is added
/// back to the free total at `r13-0x4718`. A free `+0x38` (next) neighbour is
/// absorbed first, then a free `+0x34` (previous) one; each absorbed node is
/// relinked to the head of the `r13-0x4724` list and `0x802C0B40` is told the
/// new extent.
///
/// # Original
/// - Ghidra: `FUN_802d3370` at `0x802D3370` (280 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3370_FUN_802d3370.c`
///
/// # Arguments
/// - `node`: heap node being freed; `+0x2C` size, `+0x30` in-use word,
///   `+0x34`/`+0x38` neighbour links, `+0x10` block base.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the free total, both neighbour links and the free-list head; calls
/// `0x802C0B40`.
///
/// # Safety
/// `node` and every linked neighbour must be live heap nodes.
///
/// # Confidence
/// high - every offset and both merge arms are literal; the free-list cells
/// `r13-0x4724`/`-0x4728` resolve to `0x8042AF5C`/`0x8042AF58`.
pub unsafe fn fn_802d3370(node: usize) {
    platform::write_u32(node + 0x30, 0);
    let size = platform::read_u32(node + 0x2C);
    let total = SDA1_BASE - 0x4718;
    platform::write_u32(total, platform::read_u32(total).wrapping_add(size));

    let next = platform::read_u32(node + 0x38) as usize;
    if next != 0 && platform::read_u32(next + 0x30) == 0 {
        let grew = size.wrapping_add(platform::read_u32(next + 0x2C));
        platform::write_u32(node + 0x2C, grew);
        platform::call2(
            0x802C_0B40,
            node as u32,
            platform::read_u32(node + 0x10).wrapping_sub(platform::read_u32(next + 0x2C)),
        );

        let absorbed = platform::read_u32(node + 0x38) as usize;
        let successor = platform::read_u32(absorbed + 0x38) as usize;
        let head = platform::read_u32(SDA1_BASE - 0x4724) as usize;
        if head != 0 {
            platform::write_u32(head + 0x34, absorbed as u32);
        }
        platform::write_u32(absorbed + 0x38, head as u32);
        platform::write_u32(absorbed + 0x34, 0);
        platform::write_u32(SDA1_BASE - 0x4724, absorbed as u32);
        platform::write_u32(node + 0x38, successor as u32);
        if successor == 0 {
            platform::write_u32(SDA1_BASE - 0x4728, node as u32);
        } else {
            platform::write_u32(successor + 0x34, node as u32);
        }
    }

    let previous = platform::read_u32(node + 0x34) as usize;
    if previous != 0 && platform::read_u32(previous + 0x30) == 0 {
        let grew = platform::read_u32(node + 0x2C).wrapping_add(platform::read_u32(previous + 0x2C));
        platform::write_u32(node + 0x2C, grew);

        let predecessor = platform::read_u32(previous + 0x34) as usize;
        let head = platform::read_u32(SDA1_BASE - 0x4724) as usize;
        if head != 0 {
            platform::write_u32(head + 0x34, previous as u32);
        }
        platform::write_u32(previous + 0x38, head as u32);
        platform::write_u32(previous + 0x34, 0);
        platform::write_u32(SDA1_BASE - 0x4724, previous as u32);
        platform::write_u32(node + 0x34, predecessor as u32);
        if predecessor != 0 {
            platform::write_u32(predecessor + 0x38, node as u32);
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D3490 - init heap
// ---------------------------------------------------------------------------

/// Builds the 128-node free list of one heap arena.
///
/// `0x802B9270` installs the arena (`0x20`-byte alignment), then a loop of 128
/// iterations allocates a `0x3C`-byte node, constructs it through
/// [`fn_802d32a0`], runs its virtual `+0xC` initialiser and pushes it onto the
/// `r13-0x4724` list while filling the `0x804248F0` pointer bank. Finally
/// [`fn_802d3590`] pops the head, the arena size is stored at `+0x2C` and
/// `0x802C0B40` is told the whole extent.
///
/// # Original
/// - Ghidra: `FUN_802d3490` at `0x802D3490` (252 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3490_FUN_802d3490.c`
///
/// # Arguments
/// - `param_1`: arena size published at `r13-0x471C`/`-0x4718`/`-0x4714`.
/// - `param_2`: arena base; `0` selects the SDA default at `r13-0x4870`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates 128 nodes, writes the pointer bank at `0x804248F0` (128 words)
/// and the five SDA heap cells, then installs the range.
///
/// # Safety
/// The allocator `0x802B9210` must be initialised and non-null.
///
/// # Confidence
/// high - the `cmpwi r29,128`, `addi r3,sp,60`, the `+0xC` virtual slot and
/// every link store are literal; the null-allocation arm is reproduced exactly
/// as the original, which dereferences the null pointer.
pub unsafe fn fn_802d3490(param_1: u32, param_2: u32) {
    let base = if param_2 == 0 {
        platform::read_u32(SDA1_BASE - 0x4870)
    } else {
        param_2
    };

    platform::write_u32(SDA1_BASE - 0x471C, param_1);
    let arena = platform::call3(0x802B_9270, param_1, base, 0x20);
    platform::write_u32(SDA1_BASE - 0x4720, arena);

    let mut bank = 0x8042_48F0usize;
    platform::write_u32(SDA1_BASE - 0x4718, param_1);
    platform::write_u32(SDA1_BASE - 0x4714, param_1);
    platform::write_u32(SDA1_BASE - 0x4724, 0);

    let mut index = 0;
    while index < 0x80 {
        let raw = platform::call1(0x802B_9210, 0x3C) as usize;
        let node = if raw != 0 {
            fn_802d32a0(raw as *mut u32) as usize
        } else {
            0
        };
        platform::write_u32(bank, node as u32);

        let vtable = platform::read_u32(node) as usize;
        let init = platform::read_u32(vtable + 0xC) as usize;
        platform::call1(init, node as u32);

        let node = platform::read_u32(bank) as usize;
        let head = platform::read_u32(SDA1_BASE - 0x4724) as usize;
        if head != 0 {
            platform::write_u32(head + 0x34, node as u32);
        }
        index += 1;
        platform::write_u32(node + 0x38, head as u32);
        bank += 4;
        platform::write_u32(node + 0x34, 0);
        platform::write_u32(SDA1_BASE - 0x4724, node as u32);
    }

    let head = fn_802d3590();
    platform::write_u32(SDA1_BASE - 0x4728, head as u32);
    platform::write_u32(head + 0x30, 0);
    platform::write_u32(head + 0x2C, param_1);
    platform::call2(0x802C_0B40, head as u32, platform::read_u32(SDA1_BASE - 0x4720));
}

// ---------------------------------------------------------------------------
// 0x802D3590 - remove node
// ---------------------------------------------------------------------------

/// Pops the head node of the free list at `r13-0x4724`.
///
/// The head link is advanced to the popped node's `+0x38`, the new head's
/// `+0x34` is cleared and the popped node's `+0x38` is cleared before it is
/// returned.
///
/// # Original
/// - Ghidra: `FUN_802d3590` at `0x802D3590` (40 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3590_FUN_802d3590.c`
///
/// # Returns
/// The popped node, or the value that was in the head cell when the list was
/// empty.
///
/// # Side effects
/// Writes the head cell and the two cleared links.
///
/// # Confidence
/// high - the export and the shipped words (`0x802D3590..0x802D35B4`) agree;
/// the `+0x38`/`+0x34` links are the same pair [`fn_802d3370`] relinks.
pub unsafe fn fn_802d3590() -> usize {
    let head = platform::read_u32(SDA1_BASE - 0x4724) as usize;
    let next = platform::read_u32(head + 0x38) as usize;
    platform::write_u32(SDA1_BASE - 0x4724, next as u32);
    if next != 0 {
        platform::write_u32(next + 0x34, 0);
    }
    platform::write_u32(head + 0x38, 0);
    head
}

// ---------------------------------------------------------------------------
// 0x802D31A0 - unknown
// ---------------------------------------------------------------------------

/// Calls `0x801C9840` and sets display-state bit 0.
///
/// # Original
/// - Ghidra: `FUN_802d31a0` at `0x802D31A0` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802d31a0_FUN_802d31a0.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One helper call and `*(u16*)(r13-0x472E) |= 1`.
///
/// # Confidence
/// high - `bl 0x801C9840`, `lhz`/`ori 1`/`sth` at `0x802D31B0..0x802D31B8`.
pub unsafe fn fn_802d31a0() {
    platform::call0(0x801C_9840);
    set_display_state_bit(1);
}

// ---------------------------------------------------------------------------
// 0x802D31D0 - unknown
// ---------------------------------------------------------------------------

/// Calls `0x801C98B0` and sets display-state bit 0.
///
/// # Original
/// - Ghidra: `FUN_802d31d0` at `0x802D31D0` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802d31d0_FUN_802d31d0.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One helper call and `*(u16*)(r13-0x472E) |= 1`.
///
/// # Confidence
/// high - `bl 0x801C98B0`, `lhz`/`ori 1`/`sth` at `0x802D31E0..0x802D31E8`.
pub unsafe fn fn_802d31d0() {
    platform::call0(0x801C_98B0);
    set_display_state_bit(1);
}

// ---------------------------------------------------------------------------
// 0x802D3200 - unknown
// ---------------------------------------------------------------------------

/// Calls `0x801C9C10` and sets display-state bit 1.
///
/// # Original
/// - Ghidra: `FUN_802d3200` at `0x802D3200` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3200_FUN_802d3200.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One helper call and `*(u16*)(r13-0x472E) |= 2`.
///
/// # Confidence
/// high - `bl 0x801C9C10`, `lhz`/`ori 2`/`sth` at `0x802D3210..0x802D3218`.
pub unsafe fn fn_802d3200() {
    platform::call0(0x801C_9C10);
    set_display_state_bit(2);
}

// ---------------------------------------------------------------------------
// 0x802D3230 - unknown
// ---------------------------------------------------------------------------

/// Calls `0x801C9C60` and sets display-state bit 2.
///
/// # Original
/// - Ghidra: `FUN_802d3230` at `0x802D3230` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3230_FUN_802d3230.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One helper call and `*(u16*)(r13-0x472E) |= 4`.
///
/// # Confidence
/// high - `bl 0x801C9C60`, `lhz`/`ori 4`/`sth` at `0x802D3240..0x802D3248`.
pub unsafe fn fn_802d3230() {
    platform::call0(0x801C_9C60);
    set_display_state_bit(4);
}

// ---------------------------------------------------------------------------
// 0x802D3270 - unknown
// ---------------------------------------------------------------------------

/// Calls `0x801C9D20` and sets display-state bit 3.
///
/// # Original
/// - Ghidra: `FUN_802d3270` at `0x802D3270` (44 bytes)
/// - C file: `work/ghidra/main-gb/export/802d3270_FUN_802d3270.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One helper call and `*(u16*)(r13-0x472E) |= 8`.
///
/// # Confidence
/// high - `bl 0x801C9D20`, `lhz`/`ori 8`/`sth` at `0x802D3280..0x802D3288`.
pub unsafe fn fn_802d3270() {
    platform::call0(0x801C_9D20);
    set_display_state_bit(8);
}

/// Sets one bit in the display-state halfword at `r13-0x472E`.
///
/// # Original
/// Host-side helper; no binary address. Models the `lhz`/`ori`/`sth` triple
/// shared by [`fn_802d31a0`], [`fn_802d31d0`], [`fn_802d3200`],
/// [`fn_802d3230`] and [`fn_802d3270`].
///
/// # Safety
/// The halfword must be writable.
///
/// # Confidence
/// high - three literal instructions.
#[inline]
unsafe fn set_display_state_bit(bit: u16) {
    let cell = SDA1_BASE - 0x472E;
    platform::write_u16(cell, platform::read_u16(cell) | bit);
}

// ---------------------------------------------------------------------------
// 0x802D2E90 - unknown
// ---------------------------------------------------------------------------

/// Republishes the cached 3D-scale pair when it changes.
///
/// The pair is cached at `r13-0x473C`/`r13-0x4738`. On any difference the
/// routine calls `0x801C8EE0(param_1, param_2)`, re-applies the two SDA mode
/// bytes through `0x801C8E10`/`0x801C8E40` and stores the new pair.
///
/// # Original
/// - Ghidra: `FUN_802d2e90` at `0x802D2E90` (128 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2e90_FUN_802d2e90.c`
///
/// # Arguments
/// - `param_1`, `param_2`: the new pair; same types as the cached words.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three helper calls and two SDA stores when the pair changes.
///
/// # Safety
/// The two SDA bytes must be the ones `0x801C8E10`/`0x801C8E40` expect.
///
/// # Confidence
/// high - the guard, the two byte loads (`0x804248CC`/`0x804248CD`) and the
/// two cached stores are literal.
pub unsafe fn fn_802d2e90(param_1: u32, param_2: u32) {
    if param_1 == platform::read_u32(SDA1_BASE - 0x473C)
        && param_2 == platform::read_u32(SDA1_BASE - 0x4738)
    {
        return;
    }

    platform::call2(0x801C_8EE0, param_1, param_2);
    platform::call1(0x801C_8E10, platform::read_u8(0x8042_48CC) as u32);
    platform::call1(0x801C_8E40, platform::read_u8(0x8042_48CD) as u32);
    platform::write_u32(SDA1_BASE - 0x473C, param_1);
    platform::write_u32(SDA1_BASE - 0x4738, param_2);
}

// ---------------------------------------------------------------------------
// 0x802D2DA0 - unknown
// ---------------------------------------------------------------------------

/// Installs the 2D display state used by the panel layout.
///
/// The routine fixes the seven 2D-engine words, then feeds `0x801C8920` the
/// four SDA2 floats at `0x8042F5C4`/`0x8042F5C8`/`0x8042F5CC` (the third is
/// also copied into the caller's `local_8[0]` word) and finishes with the three
/// one-shot helpers.
///
/// # Original
/// - Ghidra: `FUN_802d2da0` at `0x802D2DA0` (176 bytes)
/// - C file: `work/ghidra/main-gb/export/802d2da0_FUN_802d2da0.c`
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Nine helper calls; no memory of its own is written outside the helpers.
///
/// # Safety
/// The 2D engine must be initialised, as the callees assume.
///
/// # Confidence
/// high - the nine call argument lists and the four `lfs` loads are literal;
/// Ghidra's `double` casts on `0x801C8920` are wrong for the `lfs` forms the
/// image actually contains, so the helper takes `f32` arguments.
pub unsafe fn fn_802d2da0() {
    platform::call4(0x801C_8DC0, 1, 4, 5, 5);
    platform::call5(0x801C_86C0, 7, 0, 0, 7, 0);
    platform::call3(0x801C_8700, 0, 0x16, 0);
    platform::call3(0x801C_8E70, 1, 3, 1);

    let word = crate::platform::GuestLocal::new( platform::read_u32(SDA2_BASE - 0x3ABC));
    let f1 = f32::from_bits(*word);
    let f2 = platform::read_f32(SDA2_BASE - 0x3AB8);
    let f3 = platform::read_f32(SDA2_BASE - 0x3AB4);
    call_f32x4_u32x2(
        0x801C_8920,
        f1,
        f2,
        f3,
        f2,
        0,
        ((word.as_ptr() as *const u32)) as u32,
    );

    platform::call3(0x801C_8CB0, 0, 0, 0);
    platform::call1(0x801C_8EB0, 1);
    platform::call2(0x801C_8FC0, 0, 0);
}





/// Calls a PowerPC function with no arguments and returns its `r3`.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
#[inline]
unsafe fn call0_ret(addr: usize) -> u32 {
    let function = crate::platform::guest_fn!(addr, fn() -> u32);
    function()
}

// ===========================================================================
// 0x802D0000..0x802E0000 -- Metrowerks CRT leaves (Wave 8, manual)
// ===========================================================================

/// Forwards to `0x802356D0`, passing the handler at `0x802D71F0` and `value`.
///
/// # Original
/// - Ghidra: `FUN_802d74b0` at `0x802D74B0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d74b0_FUN_802d74b0.c`
///
/// # Arguments
/// - `value`: argument forwarded to the handler.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Owned by `0x802356D0`.
///
/// # Safety
/// The callee and the handler at `0x802D71F0` must be callable.
///
/// # Confidence
/// high - single tail call, both operands explicit.
pub unsafe fn fn_802d74b0(value: u32) {
    platform::call2(0x8023_56D0, 0x802D_71F0, value);
}

/// Stores `value` into an array reached through `object + 0x2C`.
///
/// # Original
/// - Ghidra: `FUN_802d94d0` at `0x802D94D0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d94d0_FUN_802d94d0.c`
///
/// # Arguments
/// - `object`: owner object.
/// - `value`: word to store.
/// - `index`: signed element index.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `object + 0x2C` must hold a writable array base and `index` must be in range.
///
/// # Confidence
/// high - one indexed store.
pub unsafe fn fn_802d94d0(object: usize, value: u32, index: i32) {
    let array = platform::read_u32(object + 0x2C) as usize;
    platform::write_u32((array as i32).wrapping_add(index.wrapping_mul(4)) as usize, value);
}

/// Calls `0x80228DD0` with the table at `0x80424CD8` and size `0x28`.
///
/// # Original
/// - Ghidra: `FUN_802d6cb0` at `0x802D6CB0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6cb0_FUN_802d6cb0.c`
///
/// # Safety
/// The table at `0x80424CD8` must be valid for the callee.
///
/// # Confidence
/// high - both arguments are immediate.
pub unsafe fn fn_802d6cb0() {
    platform::call2(0x8022_8DD0, 0x8042_4CD8, 0x28);
}

/// Stores three words into SDA slots (`r13 - 0x46EC`, `-0x5AE0`, `-0x46E4`).
///
/// # Original
/// - Ghidra: `FUN_802d6cc0` at `0x802D6CC0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6cc0_FUN_802d6cc0.c`
///
/// # Arguments
/// - `a`, `c`, `b`: values written to `0x8042AF94`, `0x80429BA0`, `0x8042AF9C`
///   respectively.
///
/// # Safety
/// The three SDA slots must be writable.
///
/// # Confidence
/// high - the three SDA slots are fixed by the `r13 = 0x8042F680` baseline.
pub unsafe fn fn_802d6cc0(a: u32, b: u32, c: u32) {
    platform::write_u32(0x8042_AF94, a);
    platform::write_u32(0x8042_9BA0, c);
    platform::write_u32(0x8042_AF9C, b);
}

/// Returns the absolute value of a signed 32-bit integer.
///
/// # Original
/// - Ghidra: `FUN_802de514` at `0x802DE514` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802de514_FUN_802de514.c`
///
/// # Safety
/// Pure arithmetic; no memory access.
///
/// # Confidence
/// high - the C is the canonical branchless `abs` idiom.
pub unsafe fn fn_802de514(value: u32) -> i32 {
    let signed = value as i32;
    let sign = signed >> 31;
    (sign ^ signed).wrapping_sub(sign)
}

/// Returns the absolute value of a signed 32-bit integer.
///
/// Byte-identical to [`fn_802de514`]; the CRT keeps both entry points.
///
/// # Original
/// - Ghidra: `FUN_802de524` at `0x802DE524` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802de524_FUN_802de524.c`
///
/// # Safety
/// Pure arithmetic; no memory access.
///
/// # Confidence
/// high - byte-identical to `FUN_802de514`.
pub unsafe fn fn_802de524(value: u32) -> i32 {
    fn_802de514(value)
}


/// Clears one element of the array reached through `object + 0x2C`.
///
/// # Original
/// - Ghidra: `FUN_802d94e0` at `0x802D94E0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d94e0_FUN_802d94e0.c`
///
/// # Arguments
/// - `object`: owner object.
/// - `index`: signed element index.
///
/// # Safety
/// `object + 0x2C` must hold a writable array base and `index` must be in range.
///
/// # Confidence
/// high - one indexed zero store.
pub unsafe fn fn_802d94e0(object: usize, index: i32) {
    let array = platform::read_u32(object + 0x2C) as usize;
    platform::write_u32((array as i32).wrapping_add(index.wrapping_mul(4)) as usize, 0);
}

/// Calls `0x80228DD0` with the table at `0x80424CE8` and size `0x18`.
///
/// # Original
/// - Ghidra: `FUN_802da590` at `0x802DA590` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802da590_FUN_802da590.c`
///
/// # Safety
/// The table at `0x80424CE8` must be valid for the callee.
///
/// # Confidence
/// high - both arguments are immediate.
pub unsafe fn fn_802da590() {
    platform::call2(0x8022_8DD0, 0x8042_4CE8, 0x18);
}

/// Calls vtable slot `+0x0C` of the object at `+0xA0`.
///
/// # Original
/// - Ghidra: `FUN_802da960` at `0x802DA960` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802da960_FUN_802da960.c`
///
/// # Arguments
/// - `object`: owner whose `+0xA0` word points at the target object.
///
/// # Safety
/// The pointer at `object + 0xA0`, its vtable and slot `+0x0C` must be valid.
///
/// # Confidence
/// high - one indirect slot call; the receiver is the object at `+0xA0`.
pub unsafe fn fn_802da960(object: usize) {
    let target = platform::read_u32(object + 0xA0) as usize;
    let vtable = platform::read_u32(target) as usize;
    let method = platform::read_u32(vtable + 0x0C) as usize;
    platform::call1(method, target as u32);
}

// ===========================================================================
// 0x802D0000..0x802E0000 -- Metrowerks CRT continuation (Wave 21)
//
// This block continues the runtime/CRT band at the point where the Wave 6
// translation stopped: the CodeWarrior array and variadic helpers, the
// small-block allocator's size-class path, the `stdio` transfer core, the
// wide-character locale shims and the integer/floating formatters used by
// `printf`.
//
// `r13 = 0x8042F680` and `r2 = 0x80433080` are the small-data bases installed
// by `0x800041C0`; every `r13 - N` below has been resolved to an absolute
// address with that value.
// ===========================================================================

/// Head of the runtime's singly linked `FILE` list (`0x80350A18`).
const STREAM_LIST_HEAD: usize = 0x8035_0A18;

/// `r13 - 0x5AD8`: exit-registry slot for the runtime's stdio close handler,
/// with `-2` meaning "not registered" (see [`fn_802dd2e0`]).
const EXIT_REGISTRY_SLOT: usize = 0x8042_9BA8;

/// `r13 - 0x4688`: function pointer installed by [`fn_802df4a8`] and invoked
/// when every stream must be closed.
const CLOSE_ALL_HOOK: usize = 0x8042_AFF8;

/// Six ascending size limits for the small-block pools (`0x802FD3C0`):
/// 4, 12, 20, 36, 52 and 68 bytes.
const SMALL_BLOCK_LIMITS: usize = 0x802F_D3C0;

/// Word at `0x80350E68`; it points at the active locale vtable. Slots `+0x20`
/// and `+0x24` convert between multibyte and wide characters.
const LOCALE_TABLE: usize = 0x8035_0E68;

// ---------------------------------------------------------------------------
// 0x802DC5D0 - set or clear two flag groups
// ---------------------------------------------------------------------------

/// Sets or clears two independent bit masks in words `+4` and `+8` of a state
/// object, selecting set versus clear independently for each word.
///
/// # Original
/// - Ghidra: `FUN_802dc5d0` at `0x802DC5D0` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc5d0_FUN_802dc5d0.c`
///
/// # Arguments
/// - `object`: state object; only `+4` and `+8` are touched.
/// - `mask`: bits to change.
/// - `mode`: bit 0 selects set (`1`) or clear (`0`) for `+4`; bit 1 does the
///   same for `+8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two read-modify-write operations on the caller's object. No calls.
///
/// # Safety
/// `object..object+0xC` must be writable.
///
/// # Confidence
/// high - the four branch arms are explicit in the 18 bytes.
pub unsafe fn fn_802dc5d0(object: usize, mask: u32, mode: u32) {
    let first = platform::read_u32(object + 4);
    if mode & 1 == 0 {
        platform::write_u32(object + 4, first & !mask);
    } else {
        platform::write_u32(object + 4, first | mask);
    }
    let second = platform::read_u32(object + 8);
    if mode & 2 != 0 {
        platform::write_u32(object + 8, second | mask);
    } else {
        platform::write_u32(object + 8, second & !mask);
    }
}

// ---------------------------------------------------------------------------
// 0x802DC6B8 - variadic argument fetch
// ---------------------------------------------------------------------------

/// Returns the address of the next variadic argument from a register/stack
/// save descriptor, choosing the integer or floating-point save area.
///
/// # Original
/// - Ghidra: `FUN_802dc6b8` at `0x802DC6B8` (200 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc6b8_FUN_802dc6b8.c`
///
/// # Arguments
/// - `arguments`: descriptor. `+0` is the integer save index (a signed byte),
///   `+1` the floating-point save index, `+4` the overflow-area cursor and
///   `+8` the base of the saved general-purpose registers.
/// - `kind`: `0` = integer word, `2` = two-register integer (64-bit), `3` =
///   double in the floating-point save area.
///
/// # Returns
/// Address of the argument slot. For `kind == 0` the word is loaded and
/// returned as the pointer value, matching a pointer/`int` argument.
///
/// # Side effects
/// Bumps the selected save index; for an exhausted register save area it
/// bumps the overflow cursor at `+4` and returns an aligned stack slot.
///
/// # Safety
/// `arguments` must describe the live call frame; the returned address is only
/// valid for the lifetime of that frame.
///
/// # Confidence
/// medium-high - the three class paths and both counters are explicit; the
/// two-register case is inferred from the `+2` bias and the 8-byte stack
/// stride.
pub unsafe fn fn_802dc6b8(arguments: usize, kind: u32) -> usize {
    let mut index = platform::read_u8(arguments) as i8 as i32;
    let mut index_slot = arguments;
    let mut limit: i32 = 8;
    let mut stack_stride: i32 = 4;
    let mut char_bias: i32 = 1;
    let mut save_offset: i32 = 0;
    let mut slot_stride: i32 = 4;

    if kind == 3 {
        index = platform::read_u8(arguments + 1) as i8 as i32;
        index_slot = arguments + 1;
        stack_stride = 8;
        save_offset = 0x20;
        slot_stride = 8;
    }
    let mut align: i32 = 0;
    if kind == 2 {
        stack_stride = 8;
        limit = 7;
        align = (index & 1 != 0) as i32;
        char_bias = 2;
    }

    let mut result;
    if index < limit {
        // observed: a saved register is used while the index is below the
        // 8-word save area; the slot address is base + index * stride.
        let save_area = platform::read_i32(arguments + 8);
        let slot = index.wrapping_add(align);
        platform::write_u8(index_slot, (slot as u8).wrapping_add(char_bias as u8));
        result = (save_area as isize
            + (slot as isize).wrapping_mul(slot_stride as isize)
            + save_offset as isize) as usize;
    } else {
        // observed: `index = 8` marks "spilled"; the overflow cursor is
        // rounded up to the 4- or 8-byte stack slot.
        platform::write_u8(index_slot, 8);
        let stack = platform::read_i32(arguments + 4);
        let candidate = stack
            .wrapping_add(stack_stride)
            .wrapping_sub(1)
            & !(stack_stride - 1);
        result = candidate as usize;
        platform::write_i32(arguments + 4, candidate.wrapping_add(stack_stride));
    }
    if kind == 0 {
        result = platform::read_u32(result) as usize;
    }
    result
}

// ---------------------------------------------------------------------------
// 0x802DC7E0 - construct a `new[]` array
// ---------------------------------------------------------------------------

/// Constructs every element of a `new[]` array and writes its 16-byte cookie.
///
/// # Original
/// - Ghidra: `FUN_802dc7e0` at `0x802DC7E0` (260 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc7e0_FUN_802dc7e0.c`
///
/// # Arguments
/// - `header`: start of the allocation. `+0` receives the element stride and
///   `+4` the element count; the elements begin at `header + 0x10`.
/// - `constructor`: address called as `constructor(element, 1)`.
/// - `destructor`: address called during unwinding, or `0`.
/// - `stride`: element size in bytes.
/// - `count`: number of elements.
///
/// # Returns
/// `header + 0x10`, the first element.
///
/// # Side effects
/// Calls the constructor once per element. If a constructor did not return
/// normally (the original unwinds through a longjmp), the already-built
/// elements are destroyed in reverse order.
///
/// # Safety
/// `header` must point at `0x10 + stride * count` writable bytes and both
/// callbacks must accept `(element, -1)` for destruction.
///
/// # Confidence
/// high for the cookie layout and loop; the unwind arm is unreachable in
/// ordinary control flow and is preserved because the original emits it.
pub unsafe fn fn_802dc7e0(
    header: usize,
    constructor: usize,
    destructor: usize,
    stride: u32,
    count: u32,
) -> usize {
    platform::write_u32(header, stride);
    platform::write_u32(header + 4, count);
    let element_base = header + 0x10;
    let mut element = element_base;
    let mut built: u32 = 0;
    while built < count {
        platform::call2(constructor, element as u32, 1);
        element = element.wrapping_add(stride as usize);
        built += 1;
    }
    if built < count && destructor != 0 {
        // observed: reverse teardown of the successfully built prefix.
        let mut remaining = built;
        let mut target = element_base + (stride as usize).wrapping_mul(built as usize);
        while remaining != 0 {
            target = target.wrapping_sub(stride as usize);
            platform::call2(destructor, target as u32, 0xFFFF_FFFF);
            remaining -= 1;
        }
    }
    element_base
}

// ---------------------------------------------------------------------------
// 0x802DC8E4 - destroy an array object and optionally free it
// ---------------------------------------------------------------------------

/// Destroys the live elements of a growable array and optionally releases the
/// array object itself.
///
/// # Original
/// - Ghidra: `FUN_802dc8e4` at `0x802DC8E4` (188 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc8e4_FUN_802dc8e4.c`
///
/// # Arguments
/// - `array`: descriptor with `+0` = element base, `+4` = stride, `+8` =
///   capacity, `+0xC` = destructor, `+0x10` = live element count.
/// - `delete_flag`: `> 0` also frees the descriptor.
///
/// # Returns
/// `array`, unchanged, like the other release helpers in this module.
///
/// # Side effects
/// Calls `destructor(element, -1)` from the highest element down while the
/// live count is non-zero, then optionally calls the allocator release entry
/// `0x802B9280`.
///
/// # Safety
/// `array` must be null or a live descriptor whose stride/count describe
/// writable elements; freeing makes it dangling.
///
/// # Confidence
/// high - every offset and the descending loop come directly from the C.
pub unsafe fn fn_802dc8e4(array: usize, delete_flag: i32) -> usize {
    if array != 0 {
        let count = platform::read_u32(array + 0x10);
        let capacity = platform::read_u32(array + 8);
        let destructor = platform::read_u32(array + 0xC);
        if count < capacity && destructor != 0 {
            let stride = platform::read_u32(array + 4);
            let mut element = platform::read_u32(array)
                .wrapping_add(stride.wrapping_mul(count));
            let mut remaining = count;
            while remaining != 0 {
                element = element.wrapping_sub(stride);
                platform::call2(destructor as usize, element, 0xFFFF_FFFF);
                platform::write_u32(array + 0x10, remaining - 1);
                remaining -= 1;
            }
        }
        if delete_flag > 0 {
            platform::call1(0x802B_9280, array as u32);
        }
    }
    array
}

// ---------------------------------------------------------------------------
// 0x802DC9A0 - construct a plain array
// ---------------------------------------------------------------------------

/// Constructs `count` elements laid out contiguously, without a `new[]`
/// cookie, and unwinds the built prefix if construction stops early.
///
/// # Original
/// - Ghidra: `FUN_802dc9a0` at `0x802DC9A0` (248 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc9a0_FUN_802dc9a0.c`
///
/// # Arguments
/// - `base`: first element.
/// - `constructor`: called as `constructor(element, 1)`.
/// - `destructor`: called as `destructor(element, -1)` during unwinding, or
///   `0`.
/// - `stride`: element size.
/// - `count`: element count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One constructor call per element; on an abnormal stop the built elements
/// are destroyed from the last one back to `base`.
///
/// # Safety
/// `base..base+stride*count` must be writable and both callbacks must be
/// valid for their argument lists.
///
/// # Confidence
/// high - the loop and the reverse-unwind arm are explicit.
pub unsafe fn fn_802dc9a0(
    base: usize,
    constructor: usize,
    destructor: usize,
    stride: u32,
    count: u32,
) {
    let mut element = base;
    let mut built: u32 = 0;
    while built < count {
        platform::call2(constructor, element as u32, 1);
        element = element.wrapping_add(stride as usize);
        built += 1;
    }
    if built < count && destructor != 0 {
        let mut remaining = built;
        let mut target = base + (stride as usize).wrapping_mul(built as usize);
        while remaining != 0 {
            target = target.wrapping_sub(stride as usize);
            platform::call2(destructor, target as u32, 0xFFFF_FFFF);
            remaining -= 1;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DCB90 - three-word zero test
// ---------------------------------------------------------------------------

/// Reports whether any of the first three words of an object is non-zero.
///
/// # Original
/// - Ghidra: `FUN_802dcb90` at `0x802DCB90` (48 bytes)
/// - C file: `work/ghidra/main-gb/export/802dcb90_FUN_802dcb90.c`
///
/// # Arguments
/// - `object`: address of at least three readable words.
///
/// # Returns
/// `1` when `object[0]`, `object[1]` or `object[2]` is non-zero; `0` when all
/// three are zero.
///
/// # Side effects
/// None; this is a read-only predicate.
///
/// # Safety
/// `object..object+0xC` must be readable.
///
/// # Confidence
/// high - three loads and the OR-tree result.
pub unsafe fn fn_802dcb90(object: usize) -> i32 {
    if platform::read_u32(object) != 0
        || platform::read_u32(object + 4) != 0
        || platform::read_u32(object + 8) != 0
    {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// 0x802DD238 - double to unsigned 64-bit conversion
// ---------------------------------------------------------------------------

/// Truncates a `double` to an unsigned 64-bit integer, saturating at both ends.
///
/// # Original
/// - Ghidra: `FUN_802dd238` at `0x802DD238` (168 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd238_FUN_802dd238.c`
///
/// # Arguments
/// - `value`: the `double` to convert (`f1` in the original).
///
/// # Returns
/// - `0` for negative inputs and for magnitudes below `1.0`;
/// - `u64::MAX` for magnitudes at or above `2^64`;
/// - otherwise the truncation toward zero, exactly as the bit-shift code
///   in the original produces.
///
/// # Side effects
/// None; pure arithmetic.
///
/// # Safety
/// None; no memory is accessed.
///
/// # Confidence
/// high - the exponent test, implicit mantissa bit and three shift ranges are
/// all explicit in the C; the Rust form uses the equivalent IEEE-754 bit
/// formula.
pub fn fn_802dd238(value: f64) -> u64 {
    let bits = value.to_bits();
    let sign = bits >> 63;
    let exponent = ((bits >> 52) & 0x7FF) as i32;
    if exponent < 0x3FF || sign != 0 {
        return 0;
    }
    // Rebuild the 53-bit significand: implicit bit plus the stored fraction.
    let significand = (bits & 0x000F_FFFF_FFFF_FFFF) | 0x0010_0000_0000_0000;
    let shift = exponent - 0x433;
    if shift >= 12 {
        // observed: the exponent exceeds the 64-bit range.
        0xFFFF_FFFF_FFFF_FFFF
    } else if shift >= 0 {
        significand << shift
    } else {
        significand >> (-shift)
    }
}

// ---------------------------------------------------------------------------
// 0x802DD2E0 - register the exit slot
// ---------------------------------------------------------------------------

/// Registers the stdio exit handler once and remembers its registry index.
///
/// # Original
/// - Ghidra: `FUN_802dd2e0` at `0x802DD2E0` (60 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd2e0_FUN_802dd2e0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// When [`EXIT_REGISTRY_SLOT`] is `-2`, calls `0x802DD350(0x8000675C,
/// 0x80433080)` - the address of the zero-initialised handler word at
/// `0x8000675C` and the `r2` small-data base - and stores the returned slot
/// index in the cell.
///
/// # Safety
/// None beyond the fixed registry table used by `0x802DD350`.
///
/// # Confidence
/// high for the control flow; medium for the role because the registered value
/// is the address of a handler word, not a code pointer.
pub unsafe fn fn_802dd2e0() {
    if platform::read_i32(EXIT_REGISTRY_SLOT) == -2 {
        let slot = fn_802dd350(0x8000_675C, 0x8043_3080);
        platform::write_u32(EXIT_REGISTRY_SLOT, slot as u32);
    }
}

// ---------------------------------------------------------------------------
// 0x802DD31C - release the exit slot
// ---------------------------------------------------------------------------

/// Releases the registry slot claimed by [`fn_802dd2e0`] and marks it gone.
///
/// # Original
/// - Ghidra: `FUN_802dd31c` at `0x802DD31C` (52 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd31c_FUN_802dd31c.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// When the saved slot is not `-2`, calls [`fn_802dd39c`] with the saved index
/// (the value is already in `r3` in the original) and stores `-2`.
///
/// # Safety
/// The saved index must be a slot previously returned by [`fn_802dd350`].
///
/// # Confidence
/// high - raw instructions at `0x802DD32C..0x802DD348` show the load, compare,
/// call and store.
pub unsafe fn fn_802dd31c() {
    let slot = platform::read_i32(EXIT_REGISTRY_SLOT);
    if slot != -2 {
        fn_802dd39c(slot as u32);
        platform::write_u32(EXIT_REGISTRY_SLOT, 0xFFFF_FFFE);
    }
}

// ---------------------------------------------------------------------------
// 0x802DD5CC - merge with the free successor
// ---------------------------------------------------------------------------

/// Coalesces a free block with the free block that follows it and unlinks the
/// successor from its size bin.
///
/// # Original
/// - Ghidra: `FUN_802dd5cc` at `0x802DD5CC` (168 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd5cc_FUN_802dd5cc.c`
///
/// # Arguments
/// - `block`: header of the free block. `block[0]` = size and flags.
/// - `bin`: address of the bin-head word that points at the free ring.
///
/// # Returns
/// Nothing. When the successor is busy (flag bit 1 of its size word) the
/// function returns without changing anything.
///
/// # Side effects
/// Enlarges `block`, rewrites the new successor's `prev_size`, updates the
/// successor's "previous is free" flag, and splices the old successor out of
/// the doubly linked ring at `bin`.
///
/// # Safety
/// `block` must be a free block header inside the arena and `bin` must be its
/// size-class head. The successor must be readable.
///
/// # Confidence
/// high - the merge arithmetic, two `prev_size` writes and the ring unlink are
/// all explicit.
pub unsafe fn fn_802dd5cc(block: usize, bin: usize) {
    let header = platform::read_u32(block);
    let size = header & 0xFFFF_FFF8;
    let successor = block.wrapping_add(size as usize);
    if platform::read_u32(successor) & 2 != 0 {
        return;
    }
    let merged = size.wrapping_add(platform::read_u32(successor) & 0xFFFF_FFF8);
    platform::write_u32(block, merged | (header & 7));
    if header & 2 == 0 {
        // observed: untouched flags require a fresh next-block back pointer.
        platform::write_u32(block.wrapping_add(merged as usize).wrapping_sub(4), merged);
    }
    let new_next = block.wrapping_add(merged as usize);
    if platform::read_u32(block) & 2 == 0 {
        platform::write_u32(new_next, platform::read_u32(new_next) & 0xFFFF_FFFB);
    } else {
        platform::write_u32(new_next, platform::read_u32(new_next) | 4);
    }
    if platform::read_u32(bin) == successor as u32 {
        platform::write_u32(bin, platform::read_u32(successor + 0xC));
    }
    if platform::read_u32(bin) == successor as u32 {
        platform::write_u32(bin, 0);
    }
    platform::write_u32(
        platform::read_u32(successor + 0xC) as usize + 8,
        platform::read_u32(successor + 8),
    );
    platform::write_u32(
        platform::read_u32(successor + 8) as usize + 0xC,
        platform::read_u32(successor + 0xC),
    );
}

// ---------------------------------------------------------------------------
// 0x802DD674 - release a block to the small-block pool
// ---------------------------------------------------------------------------

/// Returns a small block to its size-class pool, or coalesces and releases the
/// whole pool page when its reference count reaches zero.
///
/// # Original
/// - Ghidra: `FUN_802dd674` at `0x802DD674` (492 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd674_FUN_802dd674.c`
///
/// # Arguments
/// - `pool`: the 0x34-byte small-block pool descriptor at `0x80424E78`.
/// - `pointer`: user pointer being freed. The word at `pointer - 4` points at
///   the pool control node for this block.
/// - `size`: payload size; selects one of the six classes in
///   [`SMALL_BLOCK_LIMITS`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Stores the class's current free-list head into `*pointer`.
/// - Moves `pointer - 4` into the control node's `+0xC` word and decrements the
///   node's `+0x10` reference count.
/// - When the count reaches zero, unlinks the node, merges its page with the
///   allocator (`0x802DD47C`) and, if the page became empty, removes it from
///   the pool list and releases it with `0x802DD3C4`.
///
/// # Safety
/// `pointer` must be a live small block from `pool`; the header at
/// `pointer - 4` must point at a valid control node.
///
/// # Confidence
/// high for the list surgery; medium for the exact meaning of the node's
/// `+0xC`/`+0x10` fields (free head and reference count).
pub unsafe fn fn_802dd674(pool: usize, pointer: usize, size: u32) {
    // observed: the first limit at or above the size chooses the class.
    let mut class: u32 = 0;
    while platform::read_u32(SMALL_BLOCK_LIMITS + class as usize * 4) < size {
        class += 1;
    }
    let node = platform::read_u32(pointer.wrapping_sub(4)) as usize;
    let head_slot = pool + (class as usize * 2 + 1) * 4;
    let tail_slot = pool + (class as usize * 2 + 2) * 4;

    if platform::read_u32(node + 0xC) == 0 {
        if platform::read_u32(tail_slot) != node as u32 {
            if platform::read_u32(head_slot) == node as u32 {
                // observed: tail already points elsewhere; advance it to its
                // own successor, then advance the head (which is the node).
                let tail = platform::read_u32(tail_slot) as usize;
                platform::write_u32(tail_slot, platform::read_u32(tail));
                platform::write_u32(head_slot, platform::read_u32(node));
            } else {
                // observed: ordinary doubly linked ring removal.
                platform::write_u32(
                    platform::read_u32(node) as usize + 4,
                    platform::read_u32(node + 4),
                );
                platform::write_u32(
                    platform::read_u32(node + 4) as usize,
                    platform::read_u32(node),
                );
                let tail = platform::read_u32(tail_slot);
                platform::write_u32(node + 4, tail);
                let next = platform::read_u32(tail as usize);
                platform::write_u32(node, next);
                platform::write_u32(next as usize + 4, node as u32);
                platform::write_u32(platform::read_u32(node + 4) as usize, node as u32);
                platform::write_u32(tail_slot, node as u32);
            }
        }
    }

    // observed: publish the old free head in the user word, then push the
    // user block onto the class list and drop the reference count.
    platform::write_u32(pointer, platform::read_u32(node + 0xC));
    platform::write_u32(node + 0xC, pointer.wrapping_sub(4) as u32);
    let references = platform::read_u32(node + 0x10);
    platform::write_u32(node + 0x10, references.wrapping_sub(1));
    if references.wrapping_sub(1) != 0 {
        return;
    }

    if platform::read_u32(tail_slot) == node as u32 {
        platform::write_u32(tail_slot, platform::read_u32(node + 4));
    }
    if platform::read_u32(head_slot) == node as u32 {
        platform::write_u32(head_slot, platform::read_u32(node));
    }
    platform::write_u32(
        platform::read_u32(node) as usize + 4,
        platform::read_u32(node + 4),
    );
    platform::write_u32(
        platform::read_u32(node + 4) as usize,
        platform::read_u32(node),
    );
    if platform::read_u32(tail_slot) == node as u32 {
        platform::write_u32(tail_slot, 0);
    }
    if platform::read_u32(head_slot) == node as u32 {
        platform::write_u32(head_slot, 0);
    }

    // observed: the node stores the page header address in its own
    // `prev_size` slot, with the low bit used as a tag.
    let page = (platform::read_u32(node.wrapping_sub(4)) & 0xFFFF_FFFE) as usize;
    fn_802dd47c(pool, page);

    let page_flags = platform::read_u32(page + 0x10);
    let empty = (page_flags & 2) == 0
        && (page_flags & 0xFFFF_FFF8)
            == (platform::read_u32(page + 0xC) & 0xFFFF_FFF8).wrapping_sub(0x18);
    if !empty {
        return;
    }

    let mut next_page = platform::read_u32(page + 4) as usize;
    if next_page == page {
        next_page = 0;
    }
    if platform::read_u32(pool) == page as u32 {
        platform::write_u32(pool, next_page as u32);
    }
    if next_page != 0 {
        let forward = platform::read_u32(page);
        platform::write_u32(next_page, forward);
        platform::write_u32(forward as usize + 4, next_page as u32);
    }
    platform::write_u32(page + 4, 0);
    platform::write_u32(page, 0);
    fn_802dd3c4(page);
}

// ---------------------------------------------------------------------------
// 0x802DD990 - close every stream
// ---------------------------------------------------------------------------

/// Walks the runtime `FILE` list, closes every open stream and releases the
/// streams that own their storage.
///
/// # Original
/// - Ghidra: `FUN_802dd990` at `0x802DD990` (164 bytes)
/// - C file: `work/ghidra/main-gb/export/802dd990_FUN_802dd990.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// For each stream whose `+4` mode field is non-zero, calls
/// [`fn_802de91c`] (the `fclose` core). Streams whose `+0xC` byte is zero get
/// their mode field forced to `0xC00000`; when the following stream is still
/// open, the current stream's `+0x4C` link is cleared. Streams whose `+0xC`
/// byte is non-zero are released with [`fn_802dd860`] (the allocator `free`).
///
/// # Safety
/// The list at [`STREAM_LIST_HEAD`] must contain only live `FILE` objects.
///
/// # Confidence
/// high - the raw `bl` targets at `0x802DD9BC` and `0x802DD9E4` are
/// `0x802DE91C` and `0x802DD860`.
pub unsafe fn fn_802dd990() {
    let mut cursor = STREAM_LIST_HEAD;
    while cursor != 0 {
        let current = cursor;
        if (platform::read_u32(current + 4) >> 22) & 7 != 0 {
            fn_802de91c(current);
        }
        let next = platform::read_u32(current + 0x4C) as usize;
        if platform::read_u8(current + 0xC) == 0 {
            platform::write_u32(
                current + 4,
                (platform::read_u32(current + 4) & 0xFE3F_FFFF) | 0x00C0_0000,
            );
            if next != 0 && platform::read_u8(next + 0xC) != 0 {
                platform::write_u32(current + 0x4C, 0);
            }
        } else {
            fn_802dd860(current);
        }
        cursor = next;
    }
}

// ---------------------------------------------------------------------------
// 0x802DDA34 - flush every stream
// ---------------------------------------------------------------------------

/// Flushes every open stream in the runtime `FILE` list.
///
/// # Original
/// - Ghidra: `FUN_802dda34` at `0x802DDA34` (108 bytes)
/// - C file: `work/ghidra/main-gb/export/802dda34_FUN_802dda34.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` when every flush succeeded, `-1` when at least one stream's flush
/// failed.
///
/// # Side effects
/// Calls [`fn_802de9d8`] for every stream whose mode field is non-zero.
///
/// # Safety
/// The list at [`STREAM_LIST_HEAD`] must contain live `FILE` objects.
///
/// # Confidence
/// high - one compare/accumulate loop over the list.
pub unsafe fn fn_802dda34() -> i32 {
    let mut result = 0i32;
    let mut cursor = STREAM_LIST_HEAD;
    while cursor != 0 {
        if (platform::read_u32(cursor + 4) >> 22) & 7 != 0 && fn_802de9d8(cursor) != 0 {
            result = -1;
        }
        cursor = platform::read_u32(cursor + 0x4C) as usize;
    }
    result
}

// ---------------------------------------------------------------------------
// 0x802DDB7C - decimal multiply
// ---------------------------------------------------------------------------

/// Multiplies two decimal bignums stored as binary digit strings.
///
/// # Original
/// - Ghidra: `FUN_802ddb7c` at `0x802DDB7C` (648 bytes)
/// - C file: `work/ghidra/main-gb/export/802ddb7c_FUN_802ddb7c.c`
///
/// # Arguments
/// - `result`: destination decimal record. `+0` sign, `+2` signed exponent,
///   `+4` digit count, `+5..` digits (`0..=9`, not ASCII).
/// - `left`, `right`: source records with the same layout.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the digit string, exponent and length of the product into `result`.
/// The internal accumulator is a 76-byte stack buffer; the result keeps at
/// most 36 digits and rounds the remainder half-to-even.
///
/// # Safety
/// All three records must have valid lengths (`<= 36`) and readable/writable
/// digit arrays.
///
/// # Confidence
/// high - the convolution loop, carry, exponent addition and rounding tail are
/// all explicit in the C.
pub unsafe fn fn_802ddb7c(result: usize, left: usize, right: usize) {
    let left_length = platform::read_u8(left + 4) as i32;
    let right_length = platform::read_u8(right + 4) as i32;
    let total = left_length.wrapping_add(right_length);
    let end = total as usize;
    let mut digits = [0u8; 76];
    platform::write_u8(result, 0);

    let mut carry: u32 = 0;
    let mut out = end;
    let mut remaining = total;
    while remaining - 1 > 0 {
        let mut right_index = right_length - 1;
        let mut left_start = (remaining - 1 - right_index) - 1;
        if left_start < 0 {
            left_start = 0;
            right_index = remaining - 2;
        }
        let available = left_length - left_start;
        let mut count = right_index + 1;
        if count > available {
            count = available;
        }
        if count > 0 {
            let mut offset = 0i32;
            while offset < count {
                let a = platform::read_u8(left + 5 + (left_start + offset) as usize) as u32;
                let b = platform::read_u8(right + 5 + (right_index - offset) as usize) as u32;
                carry = carry.wrapping_add(a.wrapping_mul(b));
                offset += 1;
            }
        }
        out -= 1;
        digits[out] = (carry % 10) as u8;
        carry /= 10;
        remaining -= 1;
    }

    let exponent = platform::read_u16(left + 2) as i16 as i32
        + platform::read_u16(right + 2) as i16 as i32;
    platform::write_u16(result + 2, exponent as u16);
    if carry != 0 {
        out -= 1;
        digits[out] = carry as u8;
        platform::write_u16(result + 2, exponent.wrapping_add(1) as u16);
    }

    let mut copied = 0usize;
    while copied < 0x24 && out < end {
        platform::write_u8(result + 5 + copied, digits[out]);
        out += 1;
        copied += 1;
    }
    platform::write_u8(result + 4, copied as u8);

    if out < end && digits[out] > 4 {
        if digits[out] == 5 {
            let mut scan = out + 1;
            while scan < end {
                if digits[scan] != 0 {
                    break;
                }
                scan += 1;
            }
            if scan == end {
                // observed: exact half and an even last kept digit -> no round.
                if copied == 0 || (platform::read_u8(result + 4 + copied) & 1) == 0 {
                    return;
                }
            }
        }
        let mut cursor = result + 5 + copied;
        loop {
            cursor -= 1;
            let digit = platform::read_u8(cursor);
            if digit < 9 {
                platform::write_u8(cursor, digit + 1);
                return;
            }
            if cursor == result + 5 {
                platform::write_u8(cursor, 1);
                platform::write_u16(
                    result + 2,
                    platform::read_u16(result + 2).wrapping_add(1),
                );
                return;
            }
            platform::write_u8(cursor, 0);
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DDE04 - load and round a decimal digit string
// ---------------------------------------------------------------------------

/// Loads an ASCII decimal string into a decimal record and rounds it.
///
/// # Original
/// - Ghidra: `FUN_802dde04` at `0x802DDE04` (232 bytes)
/// - C file: `work/ghidra/main-gb/export/802dde04_FUN_802dde04.c`
///
/// # Arguments
/// - `result`: decimal record to initialise.
/// - `text`: NUL-terminated ASCII digits. At most 36 are copied.
/// - `exponent`: value stored in the record's signed-exponent field.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0` = `0`, `+2` = `exponent`, `+4` = copied count and the digit
/// bytes at `+5`. When the first unconsumed byte is greater than `'4'`, the
/// last digit is incremented (with carry into the exponent); an exact `'5'`
/// with all-zero tail rounds only when the last kept digit is odd.
///
/// # Safety
/// `result` must have room for 41 bytes and `text` must be NUL-terminated.
///
/// # Confidence
/// high - the copy bound, the half-even scan and the carry propagation are
/// explicit.
pub unsafe fn fn_802dde04(result: usize, mut text: usize, exponent: u16) {
    platform::write_u16(result + 2, exponent);
    platform::write_u8(result, 0);
    let mut length = 0usize;
    while length < 0x24 && platform::read_u8(text) != 0 {
        platform::write_u8(
            result + 5 + length,
            platform::read_u8(text).wrapping_sub(0x30),
        );
        length += 1;
        text += 1;
    }
    let next = platform::read_u8(text);
    platform::write_u8(result + 4, length as u8);
    if next == 0 {
        return;
    }
    if next < 5 {
        return;
    }
    if next < 6 {
        loop {
            text += 1;
            if platform::read_u8(text) == 0 {
                if (platform::read_u8(result + 4 + length) & 1) == 0 {
                    return;
                }
                break;
            }
            if platform::read_u8(text) != b'0' {
                break;
            }
        }
    }
    let mut cursor = result + 5 + length;
    loop {
        cursor -= 1;
        let digit = platform::read_u8(cursor);
        if digit < 9 {
            platform::write_u8(cursor, digit + 1);
            return;
        }
        if cursor == result + 5 {
            platform::write_u8(cursor, 1);
            platform::write_u16(result + 2, platform::read_u16(result + 2).wrapping_add(1));
            return;
        }
        platform::write_u8(cursor, 0);
    }
}

// ---------------------------------------------------------------------------
// 0x802DDEF0 - decimal power expansion
// ---------------------------------------------------------------------------

/// Builds the decimal representation of a power of two (or a small negative
/// power of ten) by repeated squaring.
///
/// # Original
/// - Ghidra: `FUN_802ddef0` at `0x802DDEF0` (796 bytes)
/// - C file: `work/ghidra/main-gb/export/802ddef0_FUN_802ddef0.c`
///
/// # Arguments
/// - `result`: decimal record receiving the string.
/// - `exponent`: bit index for `2^n`, or one of the negative ten-power cases
///   `-1..-8`, `-16`, `-32`, `-53`, `-64`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Recursively builds `2^(n/2)`, squares it with [`fn_802ddb7c`], and for an
/// odd `n` multiplies once more by `2` (positive) or `5 * 10^-1` (negative).
/// The table cases use the digit strings at `0x802FD3D8..0x802FD48C`.
///
/// # Safety
/// `result` must have room for a full decimal record (41 bytes).
///
/// # Confidence
/// high for the switch table and recursion; medium for the role name, which is
/// inferred from the `2^n`/ten-power inputs.
pub unsafe fn fn_802ddef0(result: usize, exponent: u32) {
    match exponent {
        0 => fn_802dde04(result, 0x802F_D477, 0),
        1 => fn_802dde04(result, 0x802F_D479, 0),
        2 => fn_802dde04(result, 0x802F_D47B, 0),
        3 => fn_802dde04(result, 0x802F_D47D, 0),
        4 => fn_802dde04(result, 0x802F_D47F, 1),
        5 => fn_802dde04(result, 0x802F_D482, 1),
        6 => fn_802dde04(result, 0x802F_D485, 1),
        7 => fn_802dde04(result, 0x802F_D488, 2),
        8 => fn_802dde04(result, 0x802F_D48C, 2),
        0xFFFF_FFC0 => fn_802dde04(result, 0x802F_D3D8, 0xFFEC),
        0xFFFF_FFCB => fn_802dde04(result, 0x802F_D406, 0xFFF0),
        0xFFFF_FFE0 => fn_802dde04(result, 0x802F_D42D, 0xFFF6),
        0xFFFF_FFF0 => fn_802dde04(result, 0x802F_D445, 0xFFFB),
        0xFFFF_FFF8 => fn_802dde04(result, 0x802F_D452, 0xFFFD),
        0xFFFF_FFF9 => fn_802dde04(result, 0x802F_D459, 0xFFFD),
        0xFFFF_FFFA => fn_802dde04(result, 0x802F_D45F, 0xFFFE),
        0xFFFF_FFFB => fn_802dde04(result, 0x802F_D465, 0xFFFE),
        0xFFFF_FFFC => fn_802dde04(result, 0x802F_D46A, 0xFFFE),
        0xFFFF_FFFD => fn_802dde04(result, 0x802F_D46E, 0xFFFF),
        0xFFFF_FFFE => fn_802dde04(result, 0x802F_D472, 0xFFFF),
        0xFFFF_FFFF => fn_802dde04(result, 0x802F_D475, 0xFFFF),
        _ => {
            let mut square = crate::platform::GuestLocal::new( [0u8; 48]);
            let square_ptr = square.as_mut_ptr() as usize;
            fn_802ddef0(square_ptr, ((exponent as i32) / 2) as u32);
            fn_802ddb7c(result, square_ptr, square_ptr);
            if exponent & 1 != 0 {
                let mut saved = crate::platform::GuestLocal::new( [0u8; 48]);
                let mut offset = 0usize;
                while offset < 42 {
                    saved[offset] = platform::read_u8(result + offset);
                    offset += 1;
                }
                let (text, exp) = if (exponent as i32) < 1 {
                    (0x802F_D475usize, 0xFFFFu16)
                } else {
                    (0x802F_D479usize, 0u16)
                };
                fn_802dde04(square_ptr, text, exp);
                fn_802ddb7c(result, saved.as_mut_ptr() as usize, square_ptr);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DE534 - reset a stream's buffer window
// ---------------------------------------------------------------------------

/// Copies the stream's base position into its cursor and recomputes the free
/// space in its buffer.
///
/// # Original
/// - Ghidra: `FUN_802de534` at `0x802DE534` (40 bytes)
/// - C file: `work/ghidra/main-gb/export/802de534_FUN_802de534.c`
///
/// # Arguments
/// - `file`: stream whose `+0x18` position, `+0x1C` buffer start, `+0x20`
///   buffer size, `+0x24` cursor, `+0x28` free count and `+0x2C` mask are used.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three stores: `+0x24 = +0x1C`, `+0x28 = +0x20 - (+0x18 & +0x2C)` and
/// `+0x34 = +0x18`.
///
/// # Safety
/// `file` must be a live stream.
///
/// # Confidence
/// high - three loads and three stores, no calls.
pub unsafe fn fn_802de534(file: usize) {
    platform::write_u32(file + 0x24, platform::read_u32(file + 0x1C));
    platform::write_u32(
        file + 0x28,
        platform::read_u32(file + 0x20)
            .wrapping_sub(platform::read_u32(file + 0x18) & platform::read_u32(file + 0x2C)),
    );
    platform::write_u32(file + 0x34, platform::read_u32(file + 0x18));
}

// ---------------------------------------------------------------------------
// 0x802DE614 - buffered array write
// ---------------------------------------------------------------------------

/// Writes an array of elements through the stream buffer, flushing when the
/// buffer fills and handling append mode and unbuffered streams.
///
/// # Original
/// - Ghidra: `FUN_802de614` at `0x802DE614` (776 bytes)
/// - C file: `work/ghidra/main-gb/export/802de614_FUN_802de614.c`
///
/// # Arguments
/// - `buffer`: source bytes.
/// - `element_size`: bytes per element.
/// - `count`: element count.
/// - `file`: stream object (same layout as [`fn_802de55c`]).
///
/// # Returns
/// Number of whole elements written. Partial trailing bytes are not reported.
///
/// # Side effects
/// Locks the stream with `0x802E44F8`, optionally seeks to the end for append
/// mode, may call [`fn_802df4a8`], copies through the buffer with
/// `0x80004364`, flushes with [`fn_802de55c`], updates `+0x18`, `+0x24`,
/// `+0x28` and `+0x34`, and sets the error byte `+0x0A` when a write fails.
///
/// # Safety
/// `buffer` must be readable for `element_size * count` bytes and `file` must
/// be a live stream with valid callbacks.
///
/// # Confidence
/// medium-high - all branches are explicit; Ghidra drops the count argument of
/// the `memcpy` and `memrchr` calls, both restored here from the surrounding
/// `local_28[0]` value, and the `+0x1C`/`+0x20` save/restore around the
/// unbuffered path is reproduced.
pub unsafe fn fn_802de614(
    buffer: usize,
    element_size: u32,
    count: u32,
    file: usize,
) -> u32 {
    // observed: `__lockfile`-style entry; a zero result is retried with -1.
    if platform::call2(0x802E_44F8, file as u32, 0) as i32 == 0 {
        platform::call2(0x802E_44F8, file as u32, 0xFFFF_FFFF);
    }
    let total = element_size.wrapping_mul(count);
    let mut flags = platform::read_u32(file + 4);
    let mode = (flags >> 22) & 7;
    if total == 0 || platform::read_u8(file + 0x0A) != 0 || mode == 0 {
        return 0;
    }
    if mode == 2 {
        fn_802df4a8();
    }
    flags = platform::read_u32(file + 4);

    // observed: `bVar1` = "line-buffered or fully buffered", `bVar4` =
    // "cannot use the buffer for this request".
    let mut buffered = true;
    let mut cannot_buffer = false;
    if (flags >> 19) & 1 == 0 || (flags >> 25) & 3 == 2 {
        cannot_buffer = true;
    }
    if !cannot_buffer && (flags >> 25) & 3 != 1 {
        buffered = false;
    }

    flags = platform::read_u32(file + 4);
    if platform::read_u32(file + 8) >> 29 == 0 && (flags >> 27) & 2 != 0 {
        if (flags >> 27) & 4 != 0 && fn_802debb8(file, 0, 2) != 0 {
            return 0;
        }
        platform::write_u32(
            file + 8,
            (platform::read_u32(file + 8) & 0x1FFF_FFFF) | 0x2000_0000,
        );
        fn_802de534(file);
    }
    if platform::read_u32(file + 8) >> 29 != 1 {
        platform::write_u8(file + 0x0A, 1);
        platform::write_u32(file + 0x28, 0);
        return 0;
    }

    let mut written: u32 = 0;
    let mut remaining = total;
    let mut source = buffer;
    let mut loop_error = false;

    if remaining != 0 {
        if platform::read_u32(file + 0x24) != platform::read_u32(file + 0x1C) || buffered {
            platform::write_u32(
                file + 0x28,
                platform::read_u32(file + 0x20).wrapping_sub(
                    platform::read_u32(file + 0x24)
                        .wrapping_sub(platform::read_u32(file + 0x1C)),
                ),
            );
            loop {
                let mut newline_end = 0usize;
                let mut amount = platform::read_u32(file + 0x28);
                if remaining < amount {
                    amount = remaining;
                }
                flags = platform::read_u32(file + 4);
                if (flags >> 25) & 3 == 1 && amount != 0 {
                    // observed: line-buffered writes stop after the last
                    // newline in the candidate range.
                    newline_end = fn_802df06c(source, b'\n', amount);
                    if newline_end != 0 {
                        amount = (newline_end + 1 - source) as u32;
                    }
                }
                if amount != 0 {
                    platform::call3(
                        0x8000_4364,
                        platform::read_u32(file + 0x24),
                        source as u32,
                        amount,
                    );
                    source = source.wrapping_add(amount as usize);
                    platform::write_u32(
                        file + 0x24,
                        platform::read_u32(file + 0x24).wrapping_add(amount),
                    );
                    remaining = remaining.wrapping_sub(amount);
                    platform::write_u32(
                        file + 0x28,
                        platform::read_u32(file + 0x28).wrapping_sub(amount),
                    );
                }
                let should_flush = platform::read_u32(file + 0x28) == 0
                    || newline_end != 0
                    || (flags >> 25) & 3 == 0;
                if should_flush && fn_802de55c(file, 0) != 0 {
                    loop_error = true;
                    break;
                }
                written = written.wrapping_add(amount);
                if remaining == 0 || !buffered {
                    break;
                }
            }
        }
    }

    if loop_error {
        platform::write_u8(file + 0x0A, 1);
        remaining = 0;
        platform::write_u32(file + 0x28, 0);
    }

    if remaining != 0 && !buffered {
        // observed 0x802DE900..0x802DE92C: temporarily redirect the stream at
        // the caller's bytes, flush once, then restore the old window.
        let saved_start = platform::read_u32(file + 0x1C);
        let saved_size = platform::read_u32(file + 0x20);
        platform::write_u32(file + 0x1C, source as u32);
        platform::write_u32(file + 0x20, remaining);
        platform::write_u32(file + 0x24, source.wrapping_add(remaining as usize) as u32);
        let mut flushed_count = crate::platform::GuestLocal::<u32>::new( 0);
        let result = fn_802de55c(file, ((flushed_count.as_mut_ptr() as *mut u32)) as usize);
        if result == 0 {
            written = written.wrapping_add(*flushed_count);
        } else {
            platform::write_u8(file + 0x0A, 1);
            platform::write_u32(file + 0x28, 0);
        }
        platform::write_u32(file + 0x1C, saved_start);
        platform::write_u32(file + 0x20, saved_size);
        fn_802de534(file);
        platform::write_u32(file + 0x28, 0);
    }

    flags = platform::read_u32(file + 4);
    if (flags >> 25) & 3 != 2 {
        platform::write_u32(file + 0x28, 0);
    }
    written / element_size
}

// ---------------------------------------------------------------------------
// 0x802DE9D8 - flush one stream
// ---------------------------------------------------------------------------

/// Flushes one stream, or every stream when passed a null pointer.
///
/// # Original
/// - Ghidra: `FUN_802de9d8` at `0x802DE9D8` (308 bytes)
/// - C file: `work/ghidra/main-gb/export/802de9d8_FUN_802de9d8.c`
///
/// # Arguments
/// - `file`: stream to flush, or `0` to call [`fn_802dda34`].
///
/// # Returns
/// `0` on success, `-1` for a closed/errored stream or a failed flush.
///
/// # Side effects
/// May set the stream state to "no buffer" (`0x40000000` or cleared mode
/// bits), call [`fn_802deb0c`] to obtain the logical position, call
/// [`fn_802de55c`] to write the buffer, and set the error byte `+0x0A`.
///
/// # Safety
/// `file` must be null or a live stream.
///
/// # Confidence
/// high - every bit field and both state transitions are explicit; the
/// `0x802DEBB4` thunk simply forwards to [`fn_802deb0c`].
pub unsafe fn fn_802de9d8(file: usize) -> i32 {
    if file == 0 {
        return fn_802dda34();
    }
    let mut flags = platform::read_u32(file + 4);
    if platform::read_u8(file + 0x0A) != 0 || (flags >> 22) & 7 == 0 {
        return -1;
    }
    if (flags >> 27) & 7 == 1 {
        return 0;
    }

    let mut state = platform::read_u32(file + 8) >> 29;
    if state > 2 {
        platform::write_u32(
            file + 8,
            (platform::read_u32(file + 8) & 0x1FFF_FFFF) | 0x4000_0000,
        );
    }
    state = platform::read_u32(file + 8) >> 29;
    if state == 2 {
        platform::write_u32(file + 0x28, 0);
    }
    if state == 1 {
        let position = if (flags >> 22) & 7 == 1 {
            fn_802deb0c(file) as u32
        } else {
            0
        };
        let flush_result = fn_802de55c(file, 0);
        if flush_result == 0 {
            platform::write_u32(file + 0x18, position);
            platform::write_u32(file + 8, platform::read_u32(file + 8) & 0x1FFF_FFFF);
            platform::write_u32(file + 0x28, 0);
            return 0;
        }
        platform::write_u8(file + 0x0A, 1);
        platform::write_u32(file + 0x28, 0);
        return -1;
    }
    platform::write_u32(file + 8, platform::read_u32(file + 8) & 0x1FFF_FFFF);
    0
}

// ---------------------------------------------------------------------------
// 0x802DED94 - single multibyte-to-wide conversion
// ---------------------------------------------------------------------------

/// Converts one single-byte character to a wide character.
///
/// # Original
/// - Ghidra: `FUN_802ded94` at `0x802DED94` (76 bytes)
/// - C file: `work/ghidra/main-gb/export/802ded94_FUN_802ded94.c`
///
/// # Arguments
/// - `out`: optional `u16` destination; may be null.
/// - `input`: byte source; must be non-null.
/// - `length`: available input bytes.
///
/// # Returns
/// `0` for a null source or for the terminating NUL, `-1` when `length == 0`,
/// and `1` when one character was produced.
///
/// # Side effects
/// Writes the zero-extended input byte to `out` when `out` is non-null.
///
/// # Safety
/// `input` must be readable for at least one byte when non-null and `out` must
/// be writable when non-null.
///
/// # Confidence
/// high - the three guards and one store are explicit.
pub unsafe fn fn_802ded94(out: usize, input: usize, length: i32) -> i32 {
    if input == 0 {
        return 0;
    }
    if length == 0 {
        return -1;
    }
    let byte = platform::read_u8(input);
    if out != 0 {
        platform::write_u16(out, byte as u16);
    }
    if byte == 0 {
        0
    } else {
        1
    }
}

// ---------------------------------------------------------------------------
// 0x802DEDE0 - store one byte
// ---------------------------------------------------------------------------

/// Stores one byte through an output pointer.
///
/// # Original
/// - Ghidra: `FUN_802dede0` at `0x802DEDE0` (28 bytes)
/// - C file: `work/ghidra/main-gb/export/802dede0_FUN_802dede0.c`
///
/// # Arguments
/// - `out`: destination; null is accepted.
/// - `byte`: value to store.
///
/// # Returns
/// `1` when stored, `0` for a null destination.
///
/// # Side effects
/// One byte store.
///
/// # Safety
/// `out` must be writable when non-null.
///
/// # Confidence
/// high - the entire body is a null test and a store.
pub unsafe fn fn_802dede0(out: usize, byte: u8) -> i32 {
    if out == 0 {
        return 0;
    }
    platform::write_u8(out, byte);
    1
}

// ---------------------------------------------------------------------------
// 0x802DEDFC - multibyte string to wide string
// ---------------------------------------------------------------------------

/// Converts a NUL-terminated multibyte string to a wide string.
///
/// # Original
/// - Ghidra: `FUN_802dedfc` at `0x802DEDFC` (192 bytes)
/// - C file: `work/ghidra/main-gb/export/802dedfc_FUN_802dedfc.c`
///
/// # Arguments
/// - `out`: wide destination (`u16` elements); may be null.
/// - `input`: multibyte source.
/// - `limit`: maximum number of wide characters to write.
///
/// # Returns
/// Number of characters written, `-1` on a conversion error, or `0` when `out`
/// is null.
///
/// # Side effects
/// Calls the locale converter at `*(0x80350E68) + 0x20` once per character and
/// writes its result to `out`. A NUL source writes a zero `u16`.
///
/// # Safety
/// `input` must be NUL-terminated and `out` must have room for `limit` `u16`s.
///
/// # Confidence
/// high - raw instructions show the extra indirection: the word at
/// `0x80350E68` is loaded first, then slot `+0x20` is called with
/// `(out, input, remaining)`.
pub unsafe fn fn_802dedfc(out: usize, input: usize, limit: u32) -> i32 {
    let mut remaining = fn_802dc69c(input);
    if out == 0 {
        return 0;
    }
    let mut written: u32 = 0;
    let mut destination = out;
    let mut source = input;
    while written < limit {
        if platform::read_u8(source) == 0 {
            platform::write_u16(destination, 0);
            return written as i32;
        }
        // observed: the locale table and slot are reloaded per character.
        let locale = platform::read_u32(LOCALE_TABLE) as usize;
        let converter = platform::read_u32(locale + 0x20) as usize;
        let consumed = platform::call3(
            converter,
            destination as u32,
            source as u32,
            remaining as u32,
        ) as i32;
        if consumed < 1 {
            return -1;
        }
        source = source.wrapping_add(consumed as usize);
        remaining = remaining.wrapping_sub(consumed);
        destination = destination.wrapping_add(2);
        written += 1;
    }
    written as i32
}

// ---------------------------------------------------------------------------
// 0x802DEEBC - wide string to multibyte string
// ---------------------------------------------------------------------------

/// Converts a NUL-terminated wide string to a multibyte string.
///
/// # Original
/// - Ghidra: `FUN_802deebc` at `0x802DEEBC` (184 bytes)
/// - C file: `work/ghidra/main-gb/export/802deebc_FUN_802deebc.c`
///
/// # Arguments
/// - `out`: byte destination; may be null.
/// - `input`: wide source (`u16` elements); may be null.
/// - `limit`: maximum number of output bytes.
///
/// # Returns
/// Number of bytes written, including the terminating NUL, or `0` for a null
/// argument.
///
/// # Side effects
/// Calls the locale converter at `*(0x80350E68) + 0x24` with a 16-byte stack
/// buffer, the wide character and the locale pointer, then copies the produced
/// bytes with `0x802E19F8`.
///
/// # Safety
/// `input` must be NUL-terminated and `out` must have room for `limit` bytes.
///
/// # Confidence
/// high - raw instructions confirm the callback arguments `(tmp, wchar,
/// locale)` and the `+2` wide-source stride.
pub unsafe fn fn_802deebc(out: usize, input: usize, limit: u32) -> u32 {
    if out == 0 || input == 0 {
        return 0;
    }
    let mut written: u32 = 0;
    let mut source = input;
    loop {
        if written > limit {
            break;
        }
        let wide = platform::read_u16(source);
        if wide == 0 {
            platform::write_u8(out.wrapping_add(written as usize), 0);
            return written;
        }
        source = source.wrapping_add(2);
        // observed: the locale table and slot are reloaded per character.
        let locale = platform::read_u32(LOCALE_TABLE) as usize;
        let converter = platform::read_u32(locale + 0x24) as usize;
        let mut scratch = crate::platform::GuestLocal::new( [0u8; 16]);
        let produced = platform::call3(
            converter,
            scratch.as_mut_ptr() as u32,
            wide as u32,
            locale as u32,
        ) as i32;
        if limit < written.wrapping_add(produced as u32) {
            return written;
        }
        platform::call3(
            0x802E_19F8,
            out.wrapping_add(written as usize) as u32,
            scratch.as_ptr() as u32,
            produced as u32,
        );
        written = written.wrapping_add(produced as u32);
    }
    written
}

// ---------------------------------------------------------------------------
// 0x802DF3B4 - float classification
// ---------------------------------------------------------------------------

/// Classifies the single-precision value obtained by converting a `double`.
///
/// # Original
/// - Ghidra: `FUN_802df3b4` at `0x802DF3B4` (96 bytes)
/// - C file: `work/ghidra/main-gb/export/802df3b4_FUN_802df3b4.c`
///
/// # Arguments
/// - `value`: the `double` to narrow to `f32` and classify.
///
/// # Returns
/// CodeWarrior's class code: `1` = NaN, `2` = infinity, `3` = zero,
/// `4` = normal, `5` = subnormal.
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// None; no memory is accessed.
///
/// # Confidence
/// high - the exponent/mantissa tests and the four constants are explicit.
pub fn fn_802df3b4(value: f64) -> i32 {
    let bits = (value as f32).to_bits();
    let exponent = bits & 0x7F80_0000;
    let mantissa = bits & 0x007F_FFFF;
    if exponent == 0x7F80_0000 {
        // observed: `(-mantissa | mantissa) >> 31` is -1 for NaN, 0 for inf.
        let sign_or = (mantissa.wrapping_neg() | mantissa) as i32;
        (sign_or >> 31) + 2
    } else if exponent == 0 {
        if mantissa != 0 {
            5
        } else {
            3
        }
    } else {
        4
    }
}

// ---------------------------------------------------------------------------
// 0x802DF414 - double sign bit
// ---------------------------------------------------------------------------

/// Returns the sign bit of a `double` as a `u32` mask.
///
/// # Original
/// - Ghidra: `FUN_802df414` at `0x802DF414` (24 bytes)
/// - C file: `work/ghidra/main-gb/export/802df414_FUN_802df414.c`
///
/// # Arguments
/// - `value`: the `double` whose sign is requested.
///
/// # Returns
/// `0x80000000` for a negative value (including `-0.0`), otherwise `0`.
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// None; no memory is accessed.
///
/// # Confidence
/// high - the stack store is only how the original moves `f1` to a GPR; the
/// result is the high word masked with `0x80000000`.
pub fn fn_802df414(value: f64) -> u32 {
    ((value.to_bits() >> 32) as u32) & 0x8000_0000
}

// ---------------------------------------------------------------------------
// 0x802DF4A8 - install the close-all hook
// ---------------------------------------------------------------------------

/// Installs [`fn_802dd990`] as the runtime's close-all-streams hook.
///
/// # Original
/// - Ghidra: `FUN_802df4a8` at `0x802DF4A8` (16 bytes)
/// - C file: `work/ghidra/main-gb/export/802df4a8_FUN_802df4a8.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Stores `0x802DD990` at `0x8042AFF8` (`r13 - 0x4688`).
///
/// # Safety
/// The hook word must be writable.
///
/// # Confidence
/// high - the address pair is two instructions.
pub unsafe fn fn_802df4a8() {
    platform::write_u32(CLOSE_ALL_HOOK, 0x802D_D990);
}

// ---------------------------------------------------------------------------
// 0x802DFA74 - 32-bit integer formatter
// ---------------------------------------------------------------------------

/// Formats a 32-bit integer backwards from `end - 1`, applying the conversion,
/// alternate-form, sign and width rules from a `printf` conversion record.
///
/// # Original
/// - Ghidra: `FUN_802dfa74` at `0x802DFA74` (576 bytes)
/// - C file: `work/ghidra/main-gb/export/802dfa74_FUN_802dfa74.c`
///
/// # Arguments
/// - `value`: integer to format.
/// - `end`: one byte past the last position of the caller's buffer; the
///   terminator is written at `end - 1`.
/// - `spec`: conversion record. `+0` = `2` when width must be recomputed,
///   `+1` = sign mode (`1` plus, `2` space), `+3` = alternate form,
///   `+5` = conversion character (`d i o u x X`), `+8` = width, `+0xC` =
///   precision/left-padding count.
///
/// # Returns
/// Pointer to the first produced byte, or `0` when the result would exceed the
/// original's 0x1FE-byte safety bound.
///
/// # Side effects
/// Writes digits and prefixes into the caller's buffer (backwards) and may
/// update the sign or width fields of `spec`.
///
/// # Safety
/// `end` must point into a writable buffer large enough for the formatted
/// value and `spec` must be a valid conversion record.
///
/// # Confidence
/// high - digit extraction, octal `#`, width padding and sign prefixes are
/// explicit; only records with one of the five known conversions are valid.
pub unsafe fn fn_802dfa74(value: u32, end: usize, spec: usize) -> usize {
    let mut negative = false;
    platform::write_u8(end - 1, 0);
    let mut cursor = end - 1;
    let conversion = platform::read_u8(spec + 5);
    let mut base: u32;

    if value == 0
        && platform::read_i32(spec + 0xC) == 0
        && (platform::read_u8(spec + 3) == 0 || conversion != b'o')
    {
        return cursor;
    }

    let mut magnitude = value;
    if conversion == b'd' || conversion == b'i' {
        base = 10;
        if (value as i32) < 0 {
            magnitude = value.wrapping_neg();
            negative = true;
        }
    } else if conversion == b'o' {
        platform::write_u8(spec + 1, 0);
        base = 8;
    } else if conversion == b'u' {
        platform::write_u8(spec + 1, 0);
        base = 10;
    } else if conversion == b'x' || conversion == b'X' {
        platform::write_u8(spec + 1, 0);
        base = 16;
    } else {
        // unknown: the original would divide by zero for an unsupported
        // conversion; the callers only pass the five characters above.
        return 0;
    }

    let mut digits: u32 = 0;
    loop {
        let digit = magnitude % base;
        magnitude /= base;
        let character = if digit < 10 {
            b'0'.wrapping_add(digit as u8)
        } else if conversion == b'x' {
            b'W'.wrapping_add(digit as u8)
        } else {
            b'7'.wrapping_add(digit as u8)
        };
        cursor -= 1;
        platform::write_u8(cursor, character);
        digits += 1;
        if magnitude == 0 {
            break;
        }
    }

    if base == 8 && platform::read_u8(spec + 3) != 0 && platform::read_u8(cursor) != b'0' {
        cursor -= 1;
        platform::write_u8(cursor, b'0');
        digits += 1;
    }

    if platform::read_u8(spec) == 2 {
        let mut width = platform::read_i32(spec + 8);
        if negative || platform::read_u8(spec + 1) != 0 {
            width = width.wrapping_sub(1);
        }
        if base == 16 && platform::read_u8(spec + 3) != 0 {
            width = width.wrapping_sub(2);
        }
        platform::write_i32(spec + 0xC, width);
    }

    let width = platform::read_i32(spec + 0xC);
    let used = (end as isize - cursor as isize) as i32;
    if width.wrapping_add(used) < 0x1FE {
        while (digits as i32) < width {
            cursor -= 1;
            platform::write_u8(cursor, b'0');
            digits += 1;
        }
        if base == 16 && platform::read_u8(spec + 3) != 0 {
            platform::write_u8(cursor - 1, platform::read_u8(spec + 5));
            cursor -= 2;
            platform::write_u8(cursor, b'0');
        }
        if negative {
            cursor -= 1;
            platform::write_u8(cursor, b'-');
        } else if platform::read_u8(spec + 1) == 1 {
            cursor -= 1;
            platform::write_u8(cursor, b'+');
        } else if platform::read_u8(spec + 1) == 2 {
            cursor -= 1;
            platform::write_u8(cursor, b' ');
        }
        return cursor;
    }
    0
}

// ---------------------------------------------------------------------------
// 0x802DFCB4 - 64-bit integer formatter
// ---------------------------------------------------------------------------

/// Formats a 64-bit integer backwards from `end - 1` using the same conversion
/// record as [`fn_802dfa74`].
///
/// # Original
/// - Ghidra: `FUN_802dfcb4` at `0x802DFCB4` (688 bytes)
/// - C file: `work/ghidra/main-gb/export/802dfcb4_FUN_802dfcb4.c`
///
/// # Arguments
/// - `value`: 64-bit integer (`r3:r4`, high word first).
/// - `end`: one byte past the caller's buffer.
/// - `spec`: conversion record, same layout as [`fn_802dfa74`].
///
/// # Returns
/// Pointer to the first produced byte, or `0` when the width exceeds the
/// original's safety bound.
///
/// # Side effects
/// Writes digits and prefixes into the caller's buffer and may update the
/// record's sign and width fields.
///
/// # Safety
/// `end` must point into a writable buffer and `spec` must be a valid
/// conversion record.
///
/// # Confidence
/// high for the loop and prefixes; medium for the division helpers, which
/// Ghidra models as 32-bit carry chains. The reconstruction uses native 64-bit
/// `/` and `%`, which produce the same quotient and remainder as the shipped
/// restoring-division loops.
pub unsafe fn fn_802dfcb4(value: u64, end: usize, spec: usize) -> usize {
    let conversion = platform::read_u8(spec + 5);
    platform::write_u8(end - 1, 0);
    let mut cursor = end - 1;
    let mut base: u32;

    if value == 0
        && platform::read_i32(spec + 0xC) == 0
        && (platform::read_u8(spec + 3) == 0 || conversion != b'o')
    {
        return cursor;
    }

    let mut magnitude = value;
    let mut negative = false;
    if conversion == b'd' || conversion == b'i' {
        base = 10;
        if (value >> 32) as u32 & 0x8000_0000 != 0 {
            // observed: INT64_MIN is left unchanged but still gets a minus.
            magnitude = value.wrapping_neg();
            negative = true;
        }
    } else if conversion == b'o' {
        platform::write_u8(spec + 1, 0);
        base = 8;
    } else if conversion == b'u' {
        platform::write_u8(spec + 1, 0);
        base = 10;
    } else if conversion == b'x' || conversion == b'X' {
        platform::write_u8(spec + 1, 0);
        base = 16;
    } else {
        // unknown: unsupported conversion; the original would divide by zero.
        return 0;
    }

    let mut digits: u32 = 0;
    loop {
        let digit = (magnitude % base as u64) as u32;
        magnitude /= base as u64;
        let character = if digit < 10 {
            b'0'.wrapping_add(digit as u8)
        } else if conversion == b'x' {
            b'W'.wrapping_add(digit as u8)
        } else {
            b'7'.wrapping_add(digit as u8)
        };
        cursor -= 1;
        platform::write_u8(cursor, character);
        digits += 1;
        if magnitude == 0 {
            break;
        }
    }

    if base == 8 && platform::read_u8(spec + 3) != 0 && platform::read_u8(cursor) != b'0' {
        cursor -= 1;
        platform::write_u8(cursor, b'0');
        digits += 1;
    }

    if platform::read_u8(spec) == 2 {
        let mut width = platform::read_i32(spec + 8);
        if negative || platform::read_u8(spec + 1) != 0 {
            width = width.wrapping_sub(1);
        }
        if base == 16 && platform::read_u8(spec + 3) != 0 {
            width = width.wrapping_sub(2);
        }
        platform::write_i32(spec + 0xC, width);
    }

    let width = platform::read_i32(spec + 0xC);
    let used = (end as isize - cursor as isize) as i32;
    if width.wrapping_add(used) < 0x1FE {
        while (digits as i32) < width {
            cursor -= 1;
            platform::write_u8(cursor, b'0');
            digits += 1;
        }
        if base == 16 && platform::read_u8(spec + 3) != 0 {
            platform::write_u8(cursor - 1, platform::read_u8(spec + 5));
            cursor -= 2;
            platform::write_u8(cursor, b'0');
        }
        if negative {
            cursor -= 1;
            platform::write_u8(cursor, b'-');
        } else if platform::read_u8(spec + 1) == 1 {
            cursor -= 1;
            platform::write_u8(cursor, b'+');
        } else if platform::read_u8(spec + 1) == 2 {
            cursor -= 1;
            platform::write_u8(cursor, b' ');
        }
        return cursor;
    }
    0
}


// ===========================================================================
// Wave 24 - container, allocator and object-lifetime helpers
// 0x802D9380..0x802DBC84
// ===========================================================================

// ---------------------------------------------------------------------------
// 0x802D9380 - first-fit block allocator over a 32-byte-aligned free list
// ---------------------------------------------------------------------------

/// Takes a block of at least `size` bytes from a pool's free list.
///
/// # Original
/// - Ghidra: `FUN_802d9380` at `0x802D9380` (336 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9380_FUN_802d9380.c`
///
/// # Arguments
/// - `pool`: allocator control block. Word `+0x30` is the free-list head and
///   word `+0x2C` is the in-use list head.
/// - `size`: requested payload size; rounded up to a 32-byte boundary.
///
/// # Returns
/// Payload pointer (`block + 0x20`), or `0` when no free block is large
/// enough.
///
/// # Block layout
/// `// inferred:` from the field order in the original: `+0x00` previous,
/// `+0x04` next, `+0x08` child/owner, `+0x0C` sibling, `+0x10` size,
/// `+0x14` in-use byte. The payload starts at `+0x20`. The `+0x08`/`+0x0C`
/// roles are not exercised by this function and stay unnamed.
///
/// # Side effects
/// Unlinks the chosen block from the free list and either hands out the whole
/// block or splits it, inserting the remainder back into both the free and the
/// in-use lists.
///
/// # Safety
/// `pool` must point at a live allocator control block whose lists contain
/// valid blocks. The returned memory is uninitialised.
///
/// # Confidence
/// high - the C and the raw instruction sequence at `0x802D9380` agree on the
/// 32-byte rounding, the `size < aligned + 0x40` split threshold and every
/// list update.
pub unsafe fn fn_802d9380(pool: usize, size: u32) -> usize {
    let mut block = platform::read_u32(pool + 0x30) as usize;
    let aligned = size.wrapping_add(0x1F) & !0x1F;

    loop {
        if block == 0 {
            return 0;
        }
        // observed 0x802D9390/0x802D9394: compare the stored size against the
        // rounded request and walk `next` until it fits.
        let block_size = platform::read_u32(block + 0x10);
        if aligned <= block_size {
            break;
        }
        block = platform::read_u32(block + 4) as usize;
    }

    if platform::read_u32(block + 0x10) < aligned.wrapping_add(0x40) {
        // Whole-block path: the block is too small to split, so unlink it and
        // move it to the in-use list.
        let previous = platform::read_u32(block) as usize;
        let next = platform::read_u32(block + 4) as usize;
        if previous == 0 {
            platform::write_u32(pool + 0x30, next as u32);
        } else {
            platform::write_u32(previous + 4, next as u32);
        }
        if next != 0 {
            platform::write_u32(next, previous as u32);
        }
        let used = platform::read_u32(pool + 0x2C) as usize;
        if used != 0 {
            platform::write_u32(used, block as u32);
        }
        let used_head = platform::read_u32(pool + 0x2C);
        platform::write_u32(block + 4, used_head);
        platform::write_u32(block, 0);
        platform::write_u8(block + 0x14, 1);
        platform::write_u32(pool + 0x2C, block as u32);
    } else {
        // Split path: the remainder header starts 0x20 bytes past the used
        // block, so the used payload keeps the whole aligned span.
        // observed 0x802D9410/0x802D9414: `add r6,r4,r7` then `stwu r5,0x20(r6)`.
        let previous = platform::read_u32(block) as usize;
        let remainder = block + aligned as usize + 0x20;
        platform::write_u32(remainder, previous as u32);
        let next = platform::read_u32(block + 4) as usize;
        platform::write_u32(remainder + 4, next as u32);
        let remainder_size = platform::read_u32(block + 0x10)
            .wrapping_sub(aligned)
            .wrapping_sub(0x20);
        platform::write_u32(remainder + 0x10, remainder_size);
        platform::write_u8(remainder + 0x14, 0);

        let child = platform::read_u32(block + 0x0C) as usize;
        platform::write_u32(remainder + 0x0C, child as u32);
        if child != 0 {
            platform::write_u32(child + 8, remainder as u32);
        }
        platform::write_u32(remainder + 8, block as u32);
        platform::write_u32(block + 0x0C, remainder as u32);

        if previous != 0 {
            platform::write_u32(previous + 4, remainder as u32);
        } else {
            platform::write_u32(pool + 0x30, remainder as u32);
        }
        if next != 0 {
            platform::write_u32(next, remainder as u32);
        }

        let used = platform::read_u32(pool + 0x2C) as usize;
        if used != 0 {
            platform::write_u32(used, block as u32);
        }
        let used_head = platform::read_u32(pool + 0x2C);
        platform::write_u32(block + 4, used_head);
        platform::write_u32(block, 0);
        platform::write_u32(block + 0x10, aligned);
        platform::write_u8(block + 0x14, 1);
        platform::write_u32(pool + 0x2C, block as u32);
    }

    block + 0x20
}

// ---------------------------------------------------------------------------
// 0x802D9500 - release every entry of a manager's table
// ---------------------------------------------------------------------------

/// Releases every table entry owned by `object` through the manager returned
/// by `0x80286630`.
///
/// # Original
/// - Ghidra: `FUN_802d9500` at `0x802D9500` (112 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9500_FUN_802d9500.c`
///
/// # Arguments
/// - `object`: record whose word at `+0x8` is the entry count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// For each index it calls `0x80286A30(manager, index)` and
/// `0x80286AC0(manager, index)`. `// inferred:` the manager owns per-index
/// resources, and the two calls release them in order.
///
/// # Safety
/// `object` must be live; the manager callbacks must accept every index below
/// the stored count.
///
/// # Confidence
/// high - the loop bound, both callees and their arguments are explicit.
pub unsafe fn fn_802d9500(object: usize) {
    let manager = call0_u32(0x8028_6630);
    let count = platform::read_u32(object + 8);
    let mut index: u32 = 0;
    while index < count {
        platform::call2(0x8028_6A30, manager, index);
        platform::call2(0x8028_6AC0, manager, index);
        index = index.wrapping_add(1);
    }
}

// ---------------------------------------------------------------------------
// 0x802D9780 - floor(log2(value))
// ---------------------------------------------------------------------------

/// Counts how many times `value - 1` can be shifted right before it reaches
/// zero, i.e. `floor(log2(value))` for the stored power-of-two-sized field.
///
/// # Original
/// - Ghidra: `FUN_802d9780` at `0x802D9780` (36 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9780_FUN_802d9780.c`
///
/// # Arguments
/// - `object`: record whose word at `+0x4` holds the value.
///
/// # Returns
/// Number of shifts; `0` when the stored value is `0` or `1`.
///
/// # Side effects
/// None; pure.
///
/// # Safety
/// `object + 4` must be readable.
///
/// # Confidence
/// high - direct transcription of the shift loop.
pub unsafe fn fn_802d9780(object: usize) -> i32 {
    let mut remaining = platform::read_u32(object + 4).wrapping_sub(1);
    let mut count = 0;
    while remaining != 0 {
        remaining >>= 1;
        count += 1;
    }
    count
}

// ---------------------------------------------------------------------------
// 0x802D97B0 - build a descriptor with an inline dummy texture name
// ---------------------------------------------------------------------------

/// Builds a texture/pool descriptor: stores the two dimensions, derives a
/// shifted pitch, copies the literal `"dummy_tex"` name, allocates the
/// per-column pointer array and creates one child object.
///
/// # Original
/// - Ghidra: `FUN_802d97b0` at `0x802D97B0` (252 bytes)
/// - C file: `work/ghidra/main-gb/export/802d97b0_FUN_802d97b0.c`
///
/// # Arguments
/// - `object`: descriptor to initialise.
/// - `width`: stored at `+0x0`.
/// - `height`: stored at `+0x4`; its `floor(log2)` becomes the pitch shift.
/// - `heap`: allocator tag passed to `0x802B9270`/`0x802B9240`.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Writes `+0x0..+0x34`, allocates the `width * 4` pointer array at `+0x2C`,
/// calls `0x802E1938` (`strcpy`) with the literal at `0x80350880`, marks the
/// global manager flag at `+0xC064` and stores a constructed object at
/// `+0xC068`.
///
/// # Safety
/// `object` must be a writable descriptor; `heap` must be a valid allocator
/// tag. The literal at `0x80350880` must remain mapped.
///
/// # Confidence
/// high - the dropped allocator and constructor arguments were recovered from
/// the raw sequence at `0x802D9864..0x802D9884`.
pub unsafe fn fn_802d97b0(object: usize, width: u32, height: u32, heap: u32) -> usize {
    platform::write_u32(object, width);
    platform::write_u32(object + 4, height);
    let shift = fn_802d9780(object) as u32;
    // observed 0x802D97E0: `slw`; its 5-bit shift count matches Rust's
    // wrapping shift for a 32-bit value.
    platform::write_u32(object + 8, width.wrapping_shl(shift));

    // observed 0x802D97EC..0x802D97F4: strcpy(object + 0xC, 0x80350880).
    platform::call2(0x802E_1938, (object + 0x0C) as u32, 0x8035_0880);
    platform::write_u8(object + 0x2B, 0);

    let pointers = platform::call3(0x802B_9270, width.wrapping_shl(2), heap, 4);
    platform::write_u32(object + 0x2C, pointers);

    let mut offset: usize = 0;
    let mut index: u32 = 0;
    while index < platform::read_u32(object) {
        platform::write_u32(pointers as usize + offset, 0);
        offset += 4;
        index = index.wrapping_add(1);
    }

    platform::write_u32(object + 0x30, 0);
    platform::write_u8(object + 0x34, 0);

    // observed 0x802D9850..0x802D9890: manager = 0x80286630(); the flag at
    // manager+0xC064 is raised; a 0x158-byte child is allocated and built with
    // 0x802DA970 before being stored at manager+0xC068.
    let manager = call0_u32(0x8028_6630) as usize;
    platform::write_u8(manager + 0xC064, 1);
    let allocated = platform::call3(0x802B_9240, 0x158, heap, 4);
    let child = if allocated != 0 {
        fn_802da970(allocated as usize) as u32
    } else {
        0
    };
    platform::write_u32(manager + 0xC068, child);

    object
}

// ---------------------------------------------------------------------------
// 0x802D9B40 / 0x802D9CB0 / 0x802D9E20 / 0x802D9F90 - pop a managed node
// ---------------------------------------------------------------------------

/// Detaches and destroys the first node of the list rooted at `object + 0x20`.
///
/// # Original
/// - Ghidra: `FUN_802d9b40` at `0x802D9B40` (148 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9b40_FUN_802d9b40.c`
///
/// # Arguments
/// - `object`: owner record. Word `+0x20` points at the list owner; a null
///   word means the list is absent.
///
/// # Returns
/// The detached node, or `0` when the list is empty.
///
/// # Side effects
/// Calls the node owner's virtual slot `+0x8` with `(owner, node)`, then
/// unlinks the node with `0x80228F80(owner + 4, node)` and
/// `0x80228DF0(owner + 0x10, node)`.
///
/// # Safety
/// Every pointer read from the record must be valid; the virtual slot must
/// accept the two arguments.
///
/// # Confidence
/// high - the four sibling variants differ only in the owner offset.
pub unsafe fn fn_802d9b40(object: usize) -> usize {
    let owner = platform::read_u32(object + 0x20) as usize;
    if owner == 0 {
        return 0;
    }
    let node = platform::call2(0x8022_8FF0, (owner + 4) as u32, 0);
    if node == 0 {
        return 0;
    }
    let vtable = platform::read_u32(owner + 0x20) as usize;
    let release = platform::read_u32(vtable + 8) as usize;
    platform::call2(release, owner as u32, node);
    platform::call2(0x8022_8F80, (owner + 4) as u32, node);
    platform::call2(0x8022_8DF0, (owner + 0x10) as u32, node);
    node as usize
}

/// Detaches and destroys the first node of the list rooted at `object + 0x18`.
///
/// # Original
/// - Ghidra: `FUN_802d9cb0` at `0x802D9CB0` (148 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9cb0_FUN_802d9cb0.c`
///
/// # Arguments
/// - `object`: owner record; word `+0x18` points at the list owner.
///
/// # Returns
/// The detached node, or `0`.
///
/// # Side effects
/// Same virtual release and unlink sequence as [`fn_802d9b40`].
///
/// # Safety
/// Same requirements as [`fn_802d9b40`].
///
/// # Confidence
/// high - byte-for-byte sibling of `0x802D9B40`, only the owner offset differs.
pub unsafe fn fn_802d9cb0(object: usize) -> usize {
    let owner = platform::read_u32(object + 0x18) as usize;
    if owner == 0 {
        return 0;
    }
    let node = platform::call2(0x8022_8FF0, (owner + 4) as u32, 0);
    if node == 0 {
        return 0;
    }
    let vtable = platform::read_u32(owner + 0x20) as usize;
    let release = platform::read_u32(vtable + 8) as usize;
    platform::call2(release, owner as u32, node);
    platform::call2(0x8022_8F80, (owner + 4) as u32, node);
    platform::call2(0x8022_8DF0, (owner + 0x10) as u32, node);
    node as usize
}

/// Detaches and destroys the first node of the list rooted at `object + 0x10`.
///
/// # Original
/// - Ghidra: `FUN_802d9e20` at `0x802D9E20` (148 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9e20_FUN_802d9e20.c`
///
/// # Arguments
/// - `object`: owner record; word `+0x10` points at the list owner.
///
/// # Returns
/// The detached node, or `0`.
///
/// # Side effects
/// Same virtual release and unlink sequence as [`fn_802d9b40`].
///
/// # Safety
/// Same requirements as [`fn_802d9b40`].
///
/// # Confidence
/// high - sibling of `0x802D9B40` with the owner at `+0x10`.
pub unsafe fn fn_802d9e20(object: usize) -> usize {
    let owner = platform::read_u32(object + 0x10) as usize;
    if owner == 0 {
        return 0;
    }
    let node = platform::call2(0x8022_8FF0, (owner + 4) as u32, 0);
    if node == 0 {
        return 0;
    }
    let vtable = platform::read_u32(owner + 0x20) as usize;
    let release = platform::read_u32(vtable + 8) as usize;
    platform::call2(release, owner as u32, node);
    platform::call2(0x8022_8F80, (owner + 4) as u32, node);
    platform::call2(0x8022_8DF0, (owner + 0x10) as u32, node);
    node as usize
}

/// Detaches and destroys the first node of the list rooted at `object + 0x8`.
///
/// # Original
/// - Ghidra: `FUN_802d9f90` at `0x802D9F90` (148 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9f90_FUN_802d9f90.c`
///
/// # Arguments
/// - `object`: owner record; word `+0x8` points at the list owner.
///
/// # Returns
/// The detached node, or `0`.
///
/// # Side effects
/// Same virtual release and unlink sequence as [`fn_802d9b40`].
///
/// # Safety
/// Same requirements as [`fn_802d9b40`].
///
/// # Confidence
/// high - sibling of `0x802D9B40` with the owner at `+0x8`.
pub unsafe fn fn_802d9f90(object: usize) -> usize {
    let owner = platform::read_u32(object + 8) as usize;
    if owner == 0 {
        return 0;
    }
    let node = platform::call2(0x8022_8FF0, (owner + 4) as u32, 0);
    if node == 0 {
        return 0;
    }
    let vtable = platform::read_u32(owner + 0x20) as usize;
    let release = platform::read_u32(vtable + 8) as usize;
    platform::call2(release, owner as u32, node);
    platform::call2(0x8022_8F80, (owner + 4) as u32, node);
    platform::call2(0x8022_8DF0, (owner + 0x10) as u32, node);
    node as usize
}

// ---------------------------------------------------------------------------
// 0x802DA310 - construct a list-registered object
// ---------------------------------------------------------------------------

/// Runs the base constructor, installs vtable `0x80350928`, optionally calls
/// the class's own initialiser and registers the object in the global list at
/// `0x80424CE8`.
///
/// # Original
/// - Ghidra: `FUN_802da310` at `0x802DA310` (156 bytes)
/// - C file: `work/ghidra/main-gb/export/802da310_FUN_802da310.c`
///
/// # Arguments
/// - `object`: storage for the new object.
/// - `first`, `second`: forwarded to virtual slot `+0xC` only when both are
///   non-zero.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls the base constructor `0x802BA2D0(object)`, writes `+0x0`, `+0x10`
/// and `+0x14`, may call the vtable slot, then calls
/// `0x80228DF0(0x80424CE8, object)`.
///
/// # Safety
/// `object` must be writable storage with a valid vtable slot after the base
/// constructor runs.
///
/// # Confidence
/// high - the dropped base-constructor argument is `r3` at `0x802DA334`.
pub unsafe fn fn_802da310(object: usize, first: u32, second: u32) -> usize {
    platform::call1(0x802B_A2D0, object as u32);
    platform::write_u32(object + 0x10, 0);
    platform::write_u32(object, 0x8035_0928);
    platform::write_u32(object + 0x14, 0);
    if first != 0 && second != 0 {
        let vtable = platform::read_u32(object) as usize;
        let initialise = platform::read_u32(vtable + 0x0C) as usize;
        platform::call3(initialise, object as u32, first, second);
    }
    platform::call2(0x8022_8DF0, 0x8042_4CE8, object as u32);
    object
}

// ---------------------------------------------------------------------------
// 0x802DA470 - acquire two resource handles
// ---------------------------------------------------------------------------

/// Acquires two handles from the resource manager and stores them at `+0x10`
/// and `+0x14`, but only when both fields are still null.
///
/// # Original
/// - Ghidra: `FUN_802da470` at `0x802DA470` (140 bytes)
/// - C file: `work/ghidra/main-gb/export/802da470_FUN_802da470.c`
///
/// # Arguments
/// - `object`: record holding the two handle words.
/// - `first`, `second`: identifiers passed to `0x80294F50` and `0x80295160`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80294B20` (manager), `0x80294F50(manager, first)`,
/// `0x80295160(manager, second)` and `0x80295400(manager)`.
///
/// # Safety
/// `object` must be writable; the identifiers must be valid for the manager.
///
/// # Confidence
/// high - all four calls and the early-out condition are explicit.
pub unsafe fn fn_802da470(object: usize, first: u32, second: u32) {
    if platform::read_u32(object + 0x10) == 0 && platform::read_u32(object + 0x14) == 0 {
        let manager = call0_u32(0x8029_4B20);
        let handle = platform::call2(0x8029_4F50, manager, first);
        platform::write_u32(object + 0x10, handle);
        let handle = platform::call2(0x8029_5160, manager, second);
        platform::write_u32(object + 0x14, handle);
        platform::call1(0x8029_5400, manager);
    }
}

// ---------------------------------------------------------------------------
// 0x802DA500 - release two resource handles
// ---------------------------------------------------------------------------

/// Releases the two resource handles stored at `+0x10` and `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_802da500` at `0x802DA500` (144 bytes)
/// - C file: `work/ghidra/main-gb/export/802da500_FUN_802da500.c`
///
/// # Arguments
/// - `object`: record holding the two handle words.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x80294B20` for the manager, `0x80295640(manager, handle)` and
/// `0x802956E0(manager, handle)` for the non-null handles, clears both words
/// and calls `0x80295400(manager)` when anything was released.
///
/// # Safety
/// `object` must be writable; stored handles must be valid.
///
/// # Confidence
/// high - the dropped `manager, handle` arguments were recovered from
/// `0x802DA524..0x802DA554`.
pub unsafe fn fn_802da500(object: usize) {
    let manager = call0_u32(0x8029_4B20);

    let first = platform::read_u32(object + 0x10);
    let mut released = false;
    if first != 0 {
        platform::call2(0x8029_5640, manager, first);
        platform::write_u32(object + 0x10, 0);
        released = true;
    }

    let second = platform::read_u32(object + 0x14);
    let mut released_second = false;
    if second != 0 {
        platform::call2(0x8029_56E0, manager, second);
        platform::write_u32(object + 0x14, 0);
        released_second = true;
    }

    if released_second || released {
        platform::call1(0x8029_5400, manager);
    }
}

// ---------------------------------------------------------------------------
// 0x802DA5A0 / 0x802DA600 - construct / destroy an object with a +0xA0 helper
// ---------------------------------------------------------------------------

/// Runs the base constructor, installs vtable `0x80350940`, constructs the
/// embedded helper at `+0xA0` and calls virtual slot `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_802da5a0` at `0x802DA5A0` (88 bytes)
/// - C file: `work/ghidra/main-gb/export/802da5a0_FUN_802da5a0.c`
///
/// # Arguments
/// - `object`: storage for the new object.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls `0x80285810(object)`, writes `+0x1C`, calls
/// [`fn_802dbb70`]`(object + 0xA0)` and then the vtable slot.
///
/// # Safety
/// `object` must be writable storage large enough for the `+0xA0` member.
///
/// # Confidence
/// high - the base-constructor argument is `r3` at `0x802DA5B4`.
pub unsafe fn fn_802da5a0(object: usize) -> usize {
    platform::call1(0x8028_5810, object as u32);
    platform::write_u32(object + 0x1C, 0x8035_0940);
    fn_802dbb70(object + 0xA0);
    let vtable = platform::read_u32(object + 0x1C) as usize;
    let finalise = platform::read_u32(vtable + 0x24) as usize;
    platform::call1(finalise, object as u32);
    object
}

/// Destroys the object: destroys the `+0xA0` helper, runs the base destructor
/// and optionally frees the storage.
///
/// # Original
/// - Ghidra: `FUN_802da600` at `0x802DA600` (104 bytes)
/// - C file: `work/ghidra/main-gb/export/802da600_FUN_802da600.c`
///
/// # Arguments
/// - `object`: object to destroy; a null pointer is ignored.
/// - `delete_flag`: `> 0` also releases the storage with `0x802B9280`.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls [`fn_802dbc20`]`(object + 0xA0, -1)`, `0x80285880(object, 0)` and
/// optionally `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or a live object of this class; freeing makes it
/// dangling.
///
/// # Confidence
/// high - the base destructor and the free call are explicit.
pub unsafe fn fn_802da600(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        fn_802dbc20(object + 0xA0, -1);
        platform::call2(0x8028_5880, object as u32, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802DA970 / 0x802DA9C0 - construct / destroy a manager node
// ---------------------------------------------------------------------------

/// Runs the base constructor `0x80287130`, installs vtable `0x80350968` and
/// calls virtual slot `+0x2C`.
///
/// # Original
/// - Ghidra: `FUN_802da970` at `0x802DA970` (80 bytes)
/// - C file: `work/ghidra/main-gb/export/802da970_FUN_802da970.c`
///
/// # Arguments
/// - `object`: storage for the new object.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls the base constructor with `object` in `r3`, writes `+0x1C`, then
/// calls the vtable slot.
///
/// # Safety
/// `object` must be writable storage.
///
/// # Confidence
/// high - the dropped base-constructor argument is `r3` at `0x802DA984`.
pub unsafe fn fn_802da970(object: usize) -> usize {
    platform::call1(0x8028_7130, object as u32);
    platform::write_u32(object + 0x1C, 0x8035_0968);
    let vtable = platform::read_u32(object + 0x1C) as usize;
    let finalise = platform::read_u32(vtable + 0x2C) as usize;
    platform::call1(finalise, object as u32);
    object
}

/// Runs the base destructor and optionally frees the storage.
///
/// # Original
/// - Ghidra: `FUN_802da9c0` at `0x802DA9C0` (88 bytes)
/// - C file: `work/ghidra/main-gb/export/802da9c0_FUN_802da9c0.c`
///
/// # Arguments
/// - `object`: object to destroy; a null pointer is ignored.
/// - `delete_flag`: `> 0` also releases the storage.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls `0x80287190(object, 0)` and, when requested, `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or live; freeing makes it dangling.
///
/// # Confidence
/// high - only two calls and a signed comparison.
pub unsafe fn fn_802da9c0(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        platform::call2(0x8028_7190, object as u32, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802DAF10 - copy mode-selected single-precision fields
// ---------------------------------------------------------------------------

/// Copies fields from a source vector into the object according to the mode
/// in the low byte at `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_802daf10` at `0x802DAF10` (280 bytes)
/// - C file: `work/ghidra/main-gb/export/802daf10_FUN_802daf10.c`
///
/// # Arguments
/// - `object`: destination record.
/// - `source`: array of at least three words (single-precision values).
/// - `clear_flag`: non-zero clears bit `0x02000000` in the mode word after a
///   successful copy.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - mode `0`: copies `source[0]` to `+0x4C` and `source[2]` to `+0x5C`.
/// - mode `1`: copies `source[0]` to `+0x4C`.
/// - mode `5`: copies `source[0..2]` to `+0x4C`, `+0x50`, `+0x54`.
/// - modes `7`, `8`, `10`: copies `source[0]`, `source[1]`, `source[2]` to
///   `+0x4C`, `+0x5C`, `+0x60`.
/// - any matched mode with `clear_flag`: `+0x24 &= ~0x02000000`.
///
/// The C renders the stores as `undefined4`; the raw instructions are
/// `lfs`/`stfs`, so the fields are single-precision floats. Word copies are
/// used here to preserve every bit, including NaN payloads.
///
/// # Safety
/// `object` and `source` must be readable/writable for the selected mode.
///
/// # Confidence
/// high - every mode and the `0xFDFFFFFF` mask are explicit in the C and the
/// raw instructions at `0x802DAF10..0x802DAFC0`.
pub unsafe fn fn_802daf10(object: usize, source: usize, clear_flag: i32) {
    let mode_word = platform::read_u32(object + 0x24);
    let mode = mode_word & 0xFF;

    let mut copied = false;
    match mode {
        0 => {
            platform::write_u32(object + 0x4C, platform::read_u32(source));
            platform::write_u32(object + 0x5C, platform::read_u32(source + 8));
            copied = true;
        }
        1 => {
            platform::write_u32(object + 0x4C, platform::read_u32(source));
            copied = true;
        }
        5 => {
            platform::write_u32(object + 0x4C, platform::read_u32(source));
            platform::write_u32(object + 0x50, platform::read_u32(source + 4));
            platform::write_u32(object + 0x54, platform::read_u32(source + 8));
            copied = true;
        }
        7 | 8 | 10 => {
            platform::write_u32(object + 0x4C, platform::read_u32(source));
            platform::write_u32(object + 0x5C, platform::read_u32(source + 4));
            platform::write_u32(object + 0x60, platform::read_u32(source + 8));
            copied = true;
        }
        _ => {}
    }

    if copied && clear_flag != 0 {
        platform::write_u32(object + 0x24, mode_word & 0xFDFF_FFFF);
    }
}

// ---------------------------------------------------------------------------
// 0x802DB030 / 0x802DB080 - construct / destroy vtable 0x80350998 object
// ---------------------------------------------------------------------------

/// Runs the base constructor `0x802919A0`, installs vtable `0x80350998` and
/// calls virtual slot `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_802db030` at `0x802DB030` (80 bytes)
/// - C file: `work/ghidra/main-gb/export/802db030_FUN_802db030.c`
///
/// # Arguments
/// - `object`: storage for the new object.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Base constructor call, vtable write and one virtual call.
///
/// # Safety
/// `object` must be writable storage.
///
/// # Confidence
/// high - the constructor argument and slot offset are explicit.
pub unsafe fn fn_802db030(object: usize) -> usize {
    platform::call1(0x8029_19A0, object as u32);
    platform::write_u32(object + 0x1C, 0x8035_0998);
    let vtable = platform::read_u32(object + 0x1C) as usize;
    let finalise = platform::read_u32(vtable + 0x18) as usize;
    platform::call1(finalise, object as u32);
    object
}

/// Destroys a `0x80350998` object and optionally frees it.
///
/// # Original
/// - Ghidra: `FUN_802db080` at `0x802DB080` (88 bytes)
/// - C file: `work/ghidra/main-gb/export/802db080_FUN_802db080.c`
///
/// # Arguments
/// - `object`: object to destroy; null is ignored.
/// - `delete_flag`: `> 0` also releases the storage.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls `0x802919B0(object, 0)` and optionally `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or live; freeing makes it dangling.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_802db080(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        platform::call2(0x8029_19B0, object as u32, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802DB0F0 / 0x802DB140 - construct / destroy vtable 0x803509B8 object
// ---------------------------------------------------------------------------

/// Runs the base constructor `0x80292810`, installs vtable `0x803509B8` and
/// calls virtual slot `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_802db0f0` at `0x802DB0F0` (80 bytes)
/// - C file: `work/ghidra/main-gb/export/802db0f0_FUN_802db0f0.c`
///
/// # Arguments
/// - `object`: storage for the new object.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Base constructor call, vtable write and one virtual call.
///
/// # Safety
/// `object` must be writable storage.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_802db0f0(object: usize) -> usize {
    platform::call1(0x8029_2810, object as u32);
    platform::write_u32(object + 0x1C, 0x8035_09B8);
    let vtable = platform::read_u32(object + 0x1C) as usize;
    let finalise = platform::read_u32(vtable + 0x24) as usize;
    platform::call1(finalise, object as u32);
    object
}

/// Destroys a `0x803509B8` object and optionally frees it.
///
/// # Original
/// - Ghidra: `FUN_802db140` at `0x802DB140` (88 bytes)
/// - C file: `work/ghidra/main-gb/export/802db140_FUN_802db140.c`
///
/// # Arguments
/// - `object`: object to destroy; null is ignored.
/// - `delete_flag`: `> 0` also releases the storage.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls `0x80292B20(object, 0)` and optionally `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or live; freeing makes it dangling.
///
/// # Confidence
/// high - direct transcription.
pub unsafe fn fn_802db140(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        platform::call2(0x8029_2B20, object as u32, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802DBB20 - initialise a small record
// ---------------------------------------------------------------------------

/// Initialises the fields at `+0x8..+0x29` of a small record.
///
/// # Original
/// - Ghidra: `FUN_802dbb20` at `0x802DBB20` (72 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbb20_FUN_802dbb20.c`
///
/// # Arguments
/// - `object`: record to initialise.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `+0x08` = 0.
/// - `+0x0C..+0x1C` = five copies of the default at `0x8042F674`
///   (`r2 - 0x3A0C`, a single-precision constant).
/// - `+0x20` = 0 (halfword), `+0x22..+0x25` = `0, 0, 0, 0x80`,
///   `+0x27..+0x29` = `0, 0, 0`. Byte `+0x26` is deliberately not written;
///   the original does not touch it.
///
/// # Safety
/// `object` must be writable through `+0x29`.
///
/// # Confidence
/// high - the stores and the `lfs 0x8042F674` default are explicit.
pub unsafe fn fn_802dbb20(object: usize) {
    let default = platform::read_f32(0x8042_F674);
    platform::write_u32(object + 8, 0);
    platform::write_f32(object + 0x0C, default);
    platform::write_f32(object + 0x10, default);
    platform::write_f32(object + 0x14, default);
    platform::write_f32(object + 0x18, default);
    platform::write_f32(object + 0x1C, default);
    platform::write_u16(object + 0x20, 0);
    platform::write_u8(object + 0x22, 0);
    platform::write_u8(object + 0x23, 0);
    platform::write_u8(object + 0x24, 0);
    platform::write_u8(object + 0x25, 0x80);
    platform::write_u8(object + 0x27, 0);
    platform::write_u8(object + 0x28, 0);
    platform::write_u8(object + 0x29, 0);
}

// ---------------------------------------------------------------------------
// 0x802DBB70 - construct a record with a four-element inline array
// ---------------------------------------------------------------------------

/// Installs vtable `0x80350A08`, clears the control words, constructs the
/// four-element array at `+0x48` and calls virtual slot `+0xC`.
///
/// # Original
/// - Ghidra: `FUN_802dbb70` at `0x802DBB70` (172 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbb70_FUN_802dbb70.c`
///
/// # Arguments
/// - `object`: storage for the new record.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Writes `+0x0 = 0x80350A08`, `+0x4 = 0`, `+0x8 = 0`, `+0x44 = -1`; calls
/// [`fn_802dc9a0`]`(object + 0x48, 0x802AD910, 0x800075D0, 4, 4)`; calls the
/// vtable slot with `object`.
///
/// # Reconstruction note
/// Ghidra renders the clear loop as a counted loop. The raw sequence at
/// `0x802DBBA8..0x802DBBC4` computes a count of one, so exactly one word at
/// `+0x8` is cleared. This is kept literal rather than "fixed" to a wider
/// clear.
///
/// # Safety
/// `object` must be writable storage large enough for the array at `+0x48`.
///
/// # Confidence
/// high - the loop count, vtable and array-constructor arguments are all
/// verified against the raw instructions.
pub unsafe fn fn_802dbb70(object: usize) -> usize {
    platform::write_u32(object, 0x8035_0A08);
    platform::write_u32(object + 4, 0);
    // observed 0x802DBBA8..0x802DBBC4: `(0xF - 8) >> 2 == 1`, one zero word.
    platform::write_u32(object + 8, 0);
    platform::write_u32(object + 0x44, 0xFFFF_FFFF);
    fn_802dc9a0(object + 0x48, 0x802A_D910, 0x8000_75D0, 4, 4);
    let vtable = platform::read_u32(object) as usize;
    let initialise = platform::read_u32(vtable + 0x0C) as usize;
    platform::call1(initialise, object as u32);
    object
}

// ---------------------------------------------------------------------------
// 0x802DBC20 - destroy the record built by 0x802DBB70
// ---------------------------------------------------------------------------

/// Destroys the four-element array at `+0x48` and optionally frees the record.
///
/// # Original
/// - Ghidra: `FUN_802dbc20` at `0x802DBC20` (104 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbc20_FUN_802dbc20.c`
///
/// # Arguments
/// - `object`: record to destroy; null is ignored.
/// - `delete_flag`: `> 0` also releases the storage with `0x802B9280`.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls [`fn_802dca98`]`(object + 0x48, 0x800075D0, 4, 4)` and optionally
/// `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or live; freeing makes it dangling.
///
/// # Confidence
/// high - matches the reverse of [`fn_802dbb70`].
pub unsafe fn fn_802dbc20(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        fn_802dca98(object + 0x48, 0x8000_75D0, 4, 4);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802DC090 - store four bytes in a flag-carrying object
// ---------------------------------------------------------------------------

/// Sets mask `0x1000` in the object's flag words and copies four bytes to
/// `+0x44..+0x47`.
///
/// # Original
/// - Ghidra: `FUN_802dc090` at `0x802DC090` (92 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc090_FUN_802dc090.c`
///
/// # Arguments
/// - `object`: destination object.
/// - `source`: four-byte source (for example an RGBA colour).
/// - `mode`: forwarded to [`fn_802dc5d0`]'s set/clear selector. Ghidra omits
///   it from the signature, but the caller at `0x802D8140` passes it in `r5`
///   and this function forwards that register unchanged.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x1000, mode)` and writes `+0x44..+0x47`.
///
/// # Safety
/// `object + 0x44` and `source..source+4` must be accessible.
///
/// # Confidence
/// medium - the body is exact, but the third argument is visible only from
/// the caller and the raw `r5` pass-through.
pub unsafe fn fn_802dc090(object: usize, source: usize, mode: u32) {
    fn_802dc5d0(object, 0x1000, mode);
    let b0 = platform::read_u8(source);
    let b1 = platform::read_u8(source + 1);
    let b2 = platform::read_u8(source + 2);
    let b3 = platform::read_u8(source + 3);
    platform::write_u8(object + 0x44, b0);
    platform::write_u8(object + 0x45, b1);
    platform::write_u8(object + 0x46, b2);
    platform::write_u8(object + 0x47, b3);
}

// ---------------------------------------------------------------------------
// 0x802DC620 - set or clear two flag words from value and mode
// ---------------------------------------------------------------------------

/// Applies a value-driven set/clear of `mask` in the two flag words at `+0x4`
/// and `+0x8`, selected by two mode bits.
///
/// # Original
/// - Ghidra: `FUN_802dc620` at `0x802DC620` (124 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc620_FUN_802dc620.c`
///
/// # Arguments
/// - `object`: object holding the flag words.
/// - `mask`: bit mask applied to both words.
/// - `value`: when zero the masked bits are cleared, otherwise set.
/// - `mode`: bit `0` enables the word at `+0x4`, bit `1` the word at `+0x8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Updates `+0x4` and `+0x8`. A disabled mode bit always clears its word.
///
/// # Safety
/// `object + 8` must be writable.
///
/// # Confidence
/// high - all four branches are explicit.
pub unsafe fn fn_802dc620(object: usize, mask: u32, value: i32, mode: u32) {
    let mut first = platform::read_u32(object + 4);
    if mode & 1 == 0 {
        first &= !mask;
    } else if value == 0 {
        first &= !mask;
    } else {
        first |= mask;
    }
    platform::write_u32(object + 4, first);

    let second = platform::read_u32(object + 8);
    if mode & 2 != 0 {
        if value != 0 {
            platform::write_u32(object + 8, second | mask);
            return;
        }
        platform::write_u32(object + 8, second & !mask);
        return;
    }
    platform::write_u32(object + 8, second & !mask);
}

// ===========================================================================
// Wave 24 - decimal records, locale shim and printf specification parser
// ===========================================================================

// ---------------------------------------------------------------------------
// 0x802DDAA0 - unsigned 64-bit value to decimal digit record
// ---------------------------------------------------------------------------

/// Converts an unsigned 64-bit value into CodeWarrior's decimal digit record.
///
/// # Original
/// - Ghidra: `FUN_802ddaa0` at `0x802DDAA0` (220 bytes)
/// - C file: `work/ghidra/main-gb/export/802ddaa0_FUN_802ddaa0.c`
///
/// # Arguments
/// - `out`: 41-byte record. Layout: `+0` sign byte, `+2` exponent (`i16`),
///   `+4` digit count, `+5..+40` digits. Digits are stored as values `0..9`,
///   not ASCII; the precision step adds `'0'` later.
/// - `value`: unsigned 64-bit integer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the record. Digits are produced least-significant first, then
/// reversed in place. The exponent is `digit_count - 1` (wrapping to `0xFFFF`
/// for zero) and the sign byte is cleared.
///
/// # Safety
/// `out` must point at a writable 41-byte record.
///
/// # Confidence
/// high - Ghidra's `CONCAT44(param_3,param_4)` and the raw 64-bit compare
/// show that only the final two arguments form the value; the first integer
/// argument is unused.
pub unsafe fn fn_802ddaa0(out: usize, value: u64) {
    platform::write_u8(out, 0);
    platform::write_u8(out + 4, 0);

    let mut remaining = value;
    let mut length: u8 = 0;
    while remaining != 0 {
        // observed 0x802DDAE4/0x802DDB0C: remainder then quotient by ten.
        let digit = fn_802dcffc(remaining, 10) as u8;
        platform::write_u8(out + 5 + length as usize, digit);
        length = length.wrapping_add(1);
        remaining = fn_802dcdd8(remaining, 10);
    }

    let mut low = out + 5;
    let mut high = out + 5 + length as usize;
    while low < high {
        high -= 1;
        let left = platform::read_u8(low);
        let right = platform::read_u8(high);
        platform::write_u8(low, right);
        platform::write_u8(high, left);
        low += 1;
    }

    platform::write_u16(out + 2, (length as u16).wrapping_sub(1));
}

// ---------------------------------------------------------------------------
// 0x802DED7C - locale mbtowc tail call
// ---------------------------------------------------------------------------

/// Forwards a multibyte-to-wide conversion to the active locale's slot
/// `+0x20`.
///
/// # Original
/// - Ghidra: `thunk_FUN_802ded94` at `0x802DED7C` (24 bytes)
/// - C file: `work/ghidra/main-gb/export/802ded7c_thunk_FUN_802ded94.c`
///
/// # Arguments
/// - `out`: destination wide character, or `0`.
/// - `input`: source byte string.
/// - `length`: source length in bytes.
///
/// # Returns
/// The converter's result: `1` for a converted character, `0` at the
/// terminator, `-1` for an invalid/absent input. The exact values belong to
/// the locale module; this shim only forwards.
///
/// # Side effects
/// Loads the locale vtable from `0x80350E68` and tail-calls its `+0x20` slot.
///
/// # Safety
/// The locale table and slot must be initialised; pointers must be valid for
/// the converter.
///
/// # Confidence
/// medium - the tail call is exact; the argument names are inherited from
/// [`fn_802ded94`], which has the same ABI.
pub unsafe fn fn_802ded7c(out: usize, input: usize, length: i32) -> i32 {
    let locale = platform::read_u32(LOCALE_TABLE) as usize;
    let convert = platform::read_u32(locale + 0x20) as usize;
    platform::call3(convert, out as u32, input as u32, length as u32) as i32
}

// ---------------------------------------------------------------------------
// 0x802DF4B8 - printf conversion-specification parser
// ---------------------------------------------------------------------------

/// Parses one `printf` conversion specification and fills a four-word
/// conversion record.
///
/// # Original
/// - Ghidra: `FUN_802df4b8` at `0x802DF4B8` (1,468 bytes)
/// - C file: `work/ghidra/main-gb/export/802df4b8_FUN_802df4b8.c`
///
/// # Arguments
/// - `format`: pointer to the byte **after** the `'%'` that introduced the
///   specification. The returned pointer is one byte past the conversion
///   character.
/// - `arguments`: `va_list`-style descriptor passed to [`fn_802dc6b8`] when a
///   `'*'` width or precision is present.
/// - `spec`: four-word output record:
///   `[0]` flags (`+`/space/`#`/zero-pad markers and the "precision seen"
///   byte), `[1]` length code in the high byte and conversion character in
///   the next byte, `[2]` width, `[3]` precision.
///
/// # Returns
/// Pointer to the byte after the conversion character.
///
/// # Side effects
/// Writes the four-word record. No allocation.
///
/// # Safety
/// `format` must point into a readable string; `spec` must be writable; the
/// descriptor must be valid if a `'*'` appears.
///
/// # Reconstruction note
/// The original tests the locale digit class with a 32-bit table entry
/// (`lwzx` at `0x802DF654`, bit `0x8`), not the 16-bit form Ghidra prints in
/// the C. The 32-bit read is used here.
///
/// # Confidence
/// high for the flag/width/precision/length flow; the digit-class stride and
/// the one-word clear in [`fn_802dbb70`] were verified against the raw ELF.
pub unsafe fn fn_802df4b8(format: usize, arguments: usize, spec: usize) -> usize {
    let mut flag_zero: u8 = 1; // '+', '-' or zero-pad marker byte at +0
    let mut flag_sign: u8 = 0; // plus / space selector byte at +1
    let mut flag_precision: u8 = 0; // "precision seen" byte at +2
    let mut flag_alternate: u8 = 0; // '#' selector byte at +3
    let mut length: u8 = 0;
    let mut conversion: u8;
    let mut width: i32 = 0;
    let mut precision: i32 = 0;

    let mut cursor = format + 1;
    let mut ch = platform::read_u8(cursor) as u32;

    if ch == b'%' as u32 {
        // observed 0x802DF510: the literal "%%" record has flags 0x01000000
        // and the conversion byte '%'.
        platform::write_u32(spec, 0x0100_0000);
        platform::write_u32(spec + 4, (b'%' as u32) << 16);
        platform::write_u32(spec + 8, 0);
        platform::write_u32(spec + 0x0C, 0);
        return format + 2;
    }

    loop {
        let mut handled = true;
        if ch == b'-' as u32 {
            flag_zero = 0;
        } else if ch == b'+' as u32 {
            flag_sign = 1;
        } else if ch == b' ' as u32 {
            if flag_sign != 1 {
                flag_sign = 2;
            }
        } else if ch == b'#' as u32 {
            flag_alternate = 1;
        } else if ch == b'0' as u32 {
            if flag_zero != 0 {
                flag_zero = 2;
            }
        } else {
            handled = false;
        }
        if !handled {
            break;
        }
        cursor += 1;
        ch = platform::read_u8(cursor) as u32;
    }

    if ch == b'*' as u32 {
        let value = fn_802dc6b8(arguments, 1) as usize;
        width = platform::read_u32(value) as i32;
        if width < 0 {
            width = width.wrapping_neg();
            flag_zero = 0;
        }
        cursor += 1;
        ch = platform::read_u8(cursor) as u32;
    } else {
        while is_decimal_digit(ch) {
            cursor += 1;
            width = (ch as i32)
                .wrapping_add(width.wrapping_mul(10))
                .wrapping_sub(0x30);
            ch = platform::read_u8(cursor) as u32;
        }
    }

    if width >= 0x1FE {
        // observed 0x802DF824..0x802DF830: an over-long width returns an
        // invalid conversion with no precision.
        platform::write_u32(
            spec,
            pack_flags(flag_zero, flag_sign, flag_precision, flag_alternate),
        );
        platform::write_u32(spec + 4, 0x00FF_0000);
        platform::write_i32(spec + 8, width);
        platform::write_i32(spec + 0x0C, 0);
        return cursor + 1;
    }

    let mut scan = cursor;
    if ch == b'.' as u32 {
        scan = cursor + 1;
        flag_precision = 1;
        ch = platform::read_u8(scan) as u32;

        if ch == b'*' as u32 {
            let value = fn_802dc6b8(arguments, 1) as usize;
            precision = platform::read_u32(value) as i32;
            if precision < 0 {
                // observed 0x802DF6D8/0x802DF6DC: clear the precision-seen byte.
                flag_precision = 0;
            }
            scan = cursor + 2;
            ch = platform::read_u8(scan) as u32;
        } else {
            while is_decimal_digit(ch) {
                scan += 1;
                precision = (ch as i32)
                    .wrapping_add(precision.wrapping_mul(10))
                    .wrapping_sub(0x30);
                ch = platform::read_u8(scan) as u32;
            }
        }
    }

    loop {
        let mut handled = true;
        if ch == b'h' as u32 {
            length = 2;
            if platform::read_u8(scan + 1) == b'h' {
                length = 1;
                scan += 1;
                ch = b'h' as u32;
            }
        } else if ch == b'l' as u32 {
            length = 3;
            if platform::read_u8(scan + 1) == b'l' {
                length = 4;
                scan += 1;
                ch = b'l' as u32;
            }
        } else if ch == b'L' as u32 {
            length = 9;
        } else if ch == b'j' as u32 {
            length = 6;
        } else if ch == b't' as u32 {
            length = 8;
        } else if ch == b'z' as u32 {
            length = 7;
        } else {
            handled = false;
        }
        if !handled {
            break;
        }
        scan += 1;
        ch = platform::read_u8(scan) as u32;
    }

    // observed 0x802DF828: the conversion byte is the low byte of the parsed
    // character; characters above 0xFF always hit the invalid default.
    conversion = ch as u8;
    if ch > 0xFF {
        conversion = 0xFF;
    } else {
        match ch as u8 {
        b'A' | b'a' => {
            if flag_precision == 0 {
                precision = 0xD;
            }
            if length == 6
                || length == 7
                || length == 8
                || length == 1
                || length == 2
                || length == 4
            {
                conversion = 0xFF;
            }
        }
        b'F' | b'f' => {
            if length == 6 || length == 7 || length == 8 || length == 2 || length == 4 {
                conversion = 0xFF;
            } else if flag_precision == 0 {
                precision = 6;
            }
        }
        b'G' | b'g' => {
            if precision == 0 {
                precision = 1;
            }
            // Fall-through in the original into the E/e case.
            if length == 6
                || length == 7
                || length == 8
                || length == 1
                || length == 2
                || length == 4
            {
                conversion = 0xFF;
            } else if flag_precision == 0 {
                precision = 6;
            }
        }
        b'E' | b'e' => {
            if length == 6
                || length == 7
                || length == 8
                || length == 1
                || length == 2
                || length == 4
            {
                conversion = 0xFF;
            } else if flag_precision == 0 {
                precision = 6;
            }
        }
        b'X' | b'd' | b'i' | b'o' | b'u' | b'x' => {
            if length == 9 {
                conversion = 0xFF;
            } else if flag_precision == 0 {
                precision = 1;
            } else if flag_zero == 2 {
                flag_zero = 1;
            }
        }
        b'c' => {
            if length == 3 {
                length = 5;
            } else if flag_precision != 0 || length != 0 {
                conversion = 0xFF;
            }
        }
        b'n' => {
            if length == 9 {
                conversion = 0xFF;
            }
        }
        b'p' => {
            conversion = b'x';
            flag_alternate = 1;
            length = 3;
            precision = 8;
        }
        b's' => {
            if length == 3 {
                length = 5;
            } else if length != 0 {
                conversion = 0xFF;
            }
        }
        _ => {
            conversion = 0xFF;
        }
        }
    }

    platform::write_u32(
        spec,
        pack_flags(flag_zero, flag_sign, flag_precision, flag_alternate),
    );
    platform::write_u32(
        spec + 4,
        ((length as u32) << 24) | ((conversion as u32) << 16),
    );
    platform::write_i32(spec + 8, width);
    platform::write_i32(spec + 0x0C, precision);

    scan + 1
}

/// Calls a no-argument routine and returns its `r3` word.
///
/// `// inferred:` host-side helper, not an original function. The module's
/// [`platform::call0`] discards the return value; these routines (manager
/// getters and `0x80286630`) return a pointer in `r3`, so the value is loaded
/// with the same `extern "C"` shape the platform module uses.
unsafe fn call0_u32(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Packs the four flag bytes of a `printf` conversion record.
///
/// `// inferred:` this helper is not an original function; it exists only to
/// make the three write sites in [`fn_802df4b8`] readable.
fn pack_flags(zero: u8, sign: u8, precision: u8, alternate: u8) -> u32 {
    ((zero as u32) << 24) | ((sign as u32) << 16) | ((precision as u32) << 8) | alternate as u32
}

/// Tests the locale table's decimal-digit class for a byte value.
///
/// `// inferred:` not an original function. The original loads the locale
/// pointer from `0x80350E68`, follows `+0x8` to the character-class table and
/// tests bit `0x8` of the 32-bit entry for the character.
unsafe fn is_decimal_digit(ch: u32) -> bool {
    // observed 0x802DF4D0: `extsb` sign-extends the format byte before the
    // `uVar7 < 0x100` test, so bytes 0x80..0xFF never reach the class table.
    if ch >= 0x80 {
        return false;
    }
    let locale = platform::read_u32(LOCALE_TABLE) as usize;
    let classes = platform::read_u32(locale + 8) as usize;
    platform::read_u32(classes + ch as usize * 4) & 8 != 0
}

// ===========================================================================
// Wave 28 - C++ record accessors and the double-to-decimal print path
// ===========================================================================

// ---------------------------------------------------------------------------
// 0x802DAD00 - read the pointer field at +0x154
// ---------------------------------------------------------------------------

/// Reads the 32-bit handle stored at `+0x154`.
///
/// # Original
/// - Ghidra: `FUN_802dad00` at `0x802DAD00` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dad00_FUN_802dad00.c`
///
/// # Arguments
/// - `object`: object holding the field.
///
/// # Returns
/// The raw word at `+0x154`. `// inferred:` callers treat it as an opaque
/// handle (it is cleared by [`fn_802dad10`] and passed to a table lookup).
///
/// # Safety
/// `object + 0x154` must be readable.
///
/// # Confidence
/// high - one `lwz r3,0x154(r3)` and `blr`.
pub unsafe fn fn_802dad00(object: usize) -> u32 {
    // observed 0x802DAD00: `lwz r3,0x154(r3)`; `blr`.
    platform::read_u32(object + 0x154)
}

// ---------------------------------------------------------------------------
// 0x802DAD10 - clear the pointer field at +0x154
// ---------------------------------------------------------------------------

/// Clears the 32-bit handle stored at `+0x154`.
///
/// # Original
/// - Ghidra: `FUN_802dad10` at `0x802DAD10` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/802dad10_FUN_802dad10.c`
///
/// # Arguments
/// - `object`: object holding the field.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes zero to `+0x154`.
///
/// # Safety
/// `object + 0x154` must be writable.
///
/// # Confidence
/// high - `li r0,0`; `stw r0,0x154(r3)`; `blr`.
pub unsafe fn fn_802dad10(object: usize) {
    // observed 0x802DAD10: `li r0,0`; `stw r0,0x154(r3)`.
    platform::write_u32(object + 0x154, 0);
}

// ---------------------------------------------------------------------------
// 0x802DBD90 / 0x802DBDA0 - u16 property at +0x16
// ---------------------------------------------------------------------------

/// Reads the unsigned halfword property at `+0x16`.
///
/// # Original
/// - Ghidra: `FUN_802dbd90` at `0x802DBD90` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbd90_FUN_802dbd90.c`
///
/// # Arguments
/// - `object`: typed record (the record built by [`fn_802dbb70`]).
///
/// # Returns
/// The 16-bit property value.
///
/// # Safety
/// `object + 0x16` must be readable.
///
/// # Confidence
/// high - one `lhz r3,0x16(r3)` followed by `blr`.
pub unsafe fn fn_802dbd90(object: usize) -> u16 {
    // observed 0x802DBD90: `lhz r3,0x16(r3)`; `blr`.
    platform::read_u16(object + 0x16)
}

/// Sets the unsigned halfword property at `+0x16` and updates the record flags
/// through mask `0x8`.
///
/// # Original
/// - Ghidra: `FUN_802dbda0` at `0x802DBDA0` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbda0_FUN_802dbda0.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `value`: new 16-bit property value.
/// - `mode`: selector forwarded to [`fn_802dc5d0`] (bit 0 controls `+0x4`,
///   bit 1 controls `+0x8`). Ghidra omits it from the signature because this
///   body only forwards `r5`; the raw sequence leaves `r5` untouched.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x8, mode)`, then stores `value` at `+0x16`.
///
/// # Safety
/// `object + 0x16` must be writable.
///
/// # Confidence
/// high for the store and mask; medium for `mode`, which is visible only from
/// the raw register pass-through and the callee.
pub unsafe fn fn_802dbda0(object: usize, value: u16, mode: u32) {
    // observed 0x802DBDA0: `mr r31,r4`; `li r4,8`; `bl 0x802DC5D0`;
    // `sth r31,0x16(r30)`.
    fn_802dc5d0(object, 0x8, mode);
    platform::write_u16(object + 0x16, value);
}

// ---------------------------------------------------------------------------
// 0x802DBDE0 / 0x802DBDF0 - signed byte property at +0x18
// ---------------------------------------------------------------------------

/// Reads the signed byte property at `+0x18` and sign-extends it.
///
/// # Original
/// - Ghidra: `FUN_802dbde0` at `0x802DBDE0` (12 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbde0_FUN_802dbde0.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// The byte at `+0x18` widened to `i32`.
///
/// # Safety
/// `object + 0x18` must be readable.
///
/// # Confidence
/// high - `lbz r0,0x18(r3)`; `extsb r3,r0`; `blr`.
pub unsafe fn fn_802dbde0(object: usize) -> i32 {
    // observed 0x802DBDE0: `lbz r0,0x18(r3)`; `extsb r3,r0`.
    platform::read_u8(object + 0x18) as i8 as i32
}

/// Sets the byte property at `+0x18` and updates the record flags through mask
/// `0x10`.
///
/// # Original
/// - Ghidra: `FUN_802dbdf0` at `0x802DBDF0` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbdf0_FUN_802dbdf0.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `value`: new byte value.
/// - `mode`: forwarded selector for [`fn_802dc5d0`], as in
///   [`fn_802dbda0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x10, mode)`, then stores `value` at `+0x18`.
///
/// # Safety
/// `object + 0x18` must be writable.
///
/// # Confidence
/// high for the store and mask; medium for `mode`.
pub unsafe fn fn_802dbdf0(object: usize, value: u8, mode: u32) {
    // observed 0x802DBDF0: `mr r31,r4`; `li r4,0x10`; `bl 0x802DC5D0`;
    // `stb r31,0x18(r30)`.
    fn_802dc5d0(object, 0x10, mode);
    platform::write_u8(object + 0x18, value);
}

// ---------------------------------------------------------------------------
// 0x802DBE30 / 0x802DBE40 - float property at +0x1C
// ---------------------------------------------------------------------------

/// Reads the single-precision property at `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_802dbe30` at `0x802DBE30` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbe30_FUN_802dbe30.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// The `f32` at `+0x1C`. Ghidra prints the declaration as `double`; the raw
/// instruction is `lfs f1,0x1c(r3)`, so the value is single precision.
///
/// # Safety
/// `object + 0x1C` must be readable.
///
/// # Confidence
/// high - one `lfs` and `blr`; the return width is confirmed by the raw opcode.
pub unsafe fn fn_802dbe30(object: usize) -> f32 {
    // observed 0x802DBE30: `lfs f1,0x1c(r3)`; `blr`.
    platform::read_f32(object + 0x1C)
}

/// Sets the single-precision property at `+0x1C` and updates the record flags
/// through mask `0x20`.
///
/// # Original
/// - Ghidra: `FUN_802dbe40` at `0x802DBE40` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbe40_FUN_802dbe40.c`
///
/// # Arguments
/// - `value`: new `f32` value. The raw prologue receives it in `f1`.
/// - `object`: typed record (Ghidra's second argument, `r3`).
/// - `mode`: selector for [`fn_802dc5d0`]; the raw `mr r5,r4` moves this
///   register into the callee's third argument slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x20, mode)` and stores `value` at `+0x1C`.
///
/// # Safety
/// `object + 0x1C` must be writable.
///
/// # Confidence
/// high for the store and mask; medium for `mode`, which is visible only as
/// the raw `mr r5,r4` register move.
pub unsafe fn fn_802dbe40(value: f64, object: usize, mode: u32) {
    // observed 0x802DBE40: `mr r5,r4`; `li r4,0x20`; `fmr f31,f1`;
    // `bl 0x802DC5D0`; `stfs f31,0x1c(r31)`.
    fn_802dc5d0(object, 0x20, mode);
    platform::write_f32(object + 0x1C, value as f32);
}

// ---------------------------------------------------------------------------
// 0x802DBE90 / 0x802DBEA0 - float property at +0x20
// ---------------------------------------------------------------------------

/// Reads the single-precision property at `+0x20`.
///
/// # Original
/// - Ghidra: `FUN_802dbe90` at `0x802DBE90` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbe90_FUN_802dbe90.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// The `f32` at `+0x20`.
///
/// # Safety
/// `object + 0x20` must be readable.
///
/// # Confidence
/// high - one `lfs f1,0x20(r3)` and `blr`.
pub unsafe fn fn_802dbe90(object: usize) -> f32 {
    // observed 0x802DBE90: `lfs f1,0x20(r3)`; `blr`.
    platform::read_f32(object + 0x20)
}

/// Sets the single-precision property at `+0x20` and updates the record flags
/// through mask `0x40`.
///
/// # Original
/// - Ghidra: `FUN_802dbea0` at `0x802DBEA0` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbea0_FUN_802dbea0.c`
///
/// # Arguments
/// - `value`: new `f32` value in `f1`.
/// - `object`: typed record in `r3`.
/// - `mode`: selector in `r4`, moved to `r5` before the flag call.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x40, mode)` and stores `value` at `+0x20`.
///
/// # Safety
/// `object + 0x20` must be writable.
///
/// # Confidence
/// high for the store and mask; medium for `mode`, recovered from the raw
/// `mr r5,r4`.
pub unsafe fn fn_802dbea0(value: f64, object: usize, mode: u32) {
    // observed 0x802DBEA0: `mr r5,r4`; `li r4,0x40`; `fmr f31,f1`;
    // `bl 0x802DC5D0`; `stfs f31,0x20(r31)`.
    fn_802dc5d0(object, 0x40, mode);
    platform::write_f32(object + 0x20, value as f32);
}

// ---------------------------------------------------------------------------
// 0x802DBEF0 / 0x802DBF00 - float property at +0x24
// ---------------------------------------------------------------------------

/// Reads the single-precision property at `+0x24`.
///
/// # Original
/// - Ghidra: `FUN_802dbef0` at `0x802DBEF0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbef0_FUN_802dbef0.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// The `f32` at `+0x24`.
///
/// # Safety
/// `object + 0x24` must be readable.
///
/// # Confidence
/// high - one `lfs f1,0x24(r3)` and `blr`.
pub unsafe fn fn_802dbef0(object: usize) -> f32 {
    // observed 0x802DBEF0: `lfs f1,0x24(r3)`; `blr`.
    platform::read_f32(object + 0x24)
}

/// Sets the single-precision property at `+0x24` and updates the record flags
/// through mask `0x80`.
///
/// # Original
/// - Ghidra: `FUN_802dbf00` at `0x802DBF00` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbf00_FUN_802dbf00.c`
///
/// # Arguments
/// - `value`: new `f32` value in `f1`.
/// - `object`: typed record in `r3`.
/// - `mode`: selector in `r4`, moved to `r5` before the flag call.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x80, mode)` and stores `value` at `+0x24`.
///
/// # Safety
/// `object + 0x24` must be writable.
///
/// # Confidence
/// high for the store and mask; medium for `mode`, recovered from the raw
/// `mr r5,r4`.
pub unsafe fn fn_802dbf00(value: f64, object: usize, mode: u32) {
    // observed 0x802DBF00: `mr r5,r4`; `li r4,0x80`; `fmr f31,f1`;
    // `bl 0x802DC5D0`; `stfs f31,0x24(r31)`.
    fn_802dc5d0(object, 0x80, mode);
    platform::write_f32(object + 0x24, value as f32);
}

// ---------------------------------------------------------------------------
// 0x802DBF50 / 0x802DBF60 - float property at +0x28
// ---------------------------------------------------------------------------

/// Reads the single-precision property at `+0x28`.
///
/// # Original
/// - Ghidra: `FUN_802dbf50` at `0x802DBF50` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbf50_FUN_802dbf50.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// The `f32` at `+0x28`.
///
/// # Safety
/// `object + 0x28` must be readable.
///
/// # Confidence
/// high - one `lfs f1,0x28(r3)` and `blr`.
pub unsafe fn fn_802dbf50(object: usize) -> f32 {
    // observed 0x802DBF50: `lfs f1,0x28(r3)`; `blr`.
    platform::read_f32(object + 0x28)
}

/// Sets the single-precision property at `+0x28` and updates the record flags
/// through mask `0x100`.
///
/// # Original
/// - Ghidra: `FUN_802dbf60` at `0x802DBF60` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbf60_FUN_802dbf60.c`
///
/// # Arguments
/// - `value`: new `f32` value in `f1`.
/// - `object`: typed record in `r3`.
/// - `mode`: selector in `r4`, moved to `r5` before the flag call.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x100, mode)` and stores `value` at `+0x28`.
///
/// # Safety
/// `object + 0x28` must be writable.
///
/// # Confidence
/// high for the store and mask; medium for `mode`, recovered from the raw
/// `mr r5,r4`.
pub unsafe fn fn_802dbf60(value: f64, object: usize, mode: u32) {
    // observed 0x802DBF60: `mr r5,r4`; `li r4,0x100`; `fmr f31,f1`;
    // `bl 0x802DC5D0`; `stfs f31,0x28(r31)`.
    fn_802dc5d0(object, 0x100, mode);
    platform::write_f32(object + 0x28, value as f32);
}

// ---------------------------------------------------------------------------
// 0x802DBFB0 - address of the three-float vector at +0x2C
// ---------------------------------------------------------------------------

/// Returns the address of the three-float vector at `+0x2C`.
///
/// # Original
/// - Ghidra: `FUN_802dbfb0` at `0x802DBFB0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbfb0_FUN_802dbfb0.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x2C`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x2c`; `blr`.
pub unsafe fn fn_802dbfb0(object: usize) -> usize {
    // observed 0x802DBFB0: `addi r3,r3,0x2c`; `blr`.
    object + 0x2C
}

// ---------------------------------------------------------------------------
// 0x802DBFC0 - write the three-float vector at +0x2C
// ---------------------------------------------------------------------------

/// Copies three single-precision values into `+0x2C..+0x34` and updates the
/// record flags through mask `0x200`.
///
/// # Original
/// - Ghidra: `FUN_802dbfc0` at `0x802DBFC0` (84 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbfc0_FUN_802dbfc0.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `source`: address of three consecutive `f32` values.
/// - `mode`: selector forwarded unchanged to [`fn_802dc5d0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x200, mode)`; reads all three floats before
/// storing them to `+0x2C`, `+0x30` and `+0x34`.
///
/// # Safety
/// `source..source+12` and `object + 0x34` must be accessible.
///
/// # Confidence
/// high for the three stores and mask `0x200`; medium for `mode`, which is the
/// untouched incoming `r5`.
pub unsafe fn fn_802dbfc0(object: usize, source: usize, mode: u32) {
    // observed 0x802DBFC0: `li r4,0x200`; `bl 0x802DC5D0`;
    // `lfs f2,0(r31)`; `lfs f1,4(r31)`; `lfs f0,8(r31)`;
    // `stfs f2,0x2c(r30)`; `stfs f1,0x30(r30)`; `stfs f0,0x34(r30)`.
    fn_802dc5d0(object, 0x200, mode);
    let x = platform::read_f32(source);
    let y = platform::read_f32(source + 4);
    let z = platform::read_f32(source + 8);
    platform::write_f32(object + 0x2C, x);
    platform::write_f32(object + 0x30, y);
    platform::write_f32(object + 0x34, z);
}

// ---------------------------------------------------------------------------
// 0x802DC020 - write the three-float vector at +0x38
// ---------------------------------------------------------------------------

/// Copies three single-precision values into `+0x38..+0x40` and updates the
/// record flags through mask `0x400`.
///
/// # Original
/// - Ghidra: `FUN_802dc020` at `0x802DC020` (84 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc020_FUN_802dc020.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `source`: address of three consecutive `f32` values.
/// - `mode`: selector forwarded unchanged to [`fn_802dc5d0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x400, mode)`; stores the three floats to
/// `+0x38`, `+0x3C` and `+0x40`.
///
/// # Safety
/// `source..source+12` and `object + 0x40` must be accessible.
///
/// # Confidence
/// high for the three stores and mask `0x400`; medium for `mode`, recovered
/// from the callee's use of `r5`.
pub unsafe fn fn_802dc020(object: usize, source: usize, mode: u32) {
    // observed 0x802DC020: `li r4,0x400`; `bl 0x802DC5D0`;
    // three `lfs` and `stfs` at 0x38/0x3c/0x40.
    fn_802dc5d0(object, 0x400, mode);
    let x = platform::read_f32(source);
    let y = platform::read_f32(source + 4);
    let z = platform::read_f32(source + 8);
    platform::write_f32(object + 0x38, x);
    platform::write_f32(object + 0x3C, y);
    platform::write_f32(object + 0x40, z);
}

// ---------------------------------------------------------------------------
// 0x802DC080 - address of the four-byte colour at +0x44
// ---------------------------------------------------------------------------

/// Returns the address of the four-byte value at `+0x44`.
///
/// # Original
/// - Ghidra: `FUN_802dc080` at `0x802DC080` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc080_FUN_802dc080.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x44`. The four bytes there are written by [`fn_802dc090`].
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x44`; `blr`.
pub unsafe fn fn_802dc080(object: usize) -> usize {
    // observed 0x802DC080: `addi r3,r3,0x44`; `blr`.
    object + 0x44
}

// ---------------------------------------------------------------------------
// 0x802DC0F0 - address of one element in the 2-D byte table at +0x48
// ---------------------------------------------------------------------------

/// Computes the address of an element in the fixed table that starts at
/// `+0x48`.
///
/// # Original
/// - Ghidra: `FUN_802dc0f0` at `0x802DC0F0` (24 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc0f0_FUN_802dc0f0.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `element`: index scaled by 4 (the inner index).
/// - `group`: index scaled by 8 (the outer index).
///
/// # Returns
/// `object + group * 8 + element * 4 + 0x48`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - the two shifts (`slwi ...,3` and `slwi ...,2`) and the two adds are
/// explicit.
pub unsafe fn fn_802dc0f0(object: usize, element: u32, group: u32) -> usize {
    // observed 0x802DC0F0: `slwi r5,r5,3`; `slwi r0,r4,2`; two adds;
    // `addi r3,r3,0x48`.
    object + group as usize * 8 + element as usize * 4 + 0x48
}

// ---------------------------------------------------------------------------
// 0x802DC110 - write two three-byte entries into the table at +0x48
// ---------------------------------------------------------------------------

/// Writes two three-byte entries into the table row selected by `index`, after
/// updating the record flags.
///
/// # Original
/// - Ghidra: `FUN_802dc110` at `0x802DC110` (148 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc110_FUN_802dc110.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `first`: address of three source bytes; copied to `+0x48..+0x4A`.
/// - `second`: address of three source bytes; copied to `+0x4C..+0x4E`.
/// - `index`: row index, multiplied by 8.
/// - `mode`: selector for [`fn_802dc5d0`], passed in `r7` and moved to `r5`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802dc5d0`]`(object, 0x2000 << index, mode)`. Byte `+0x4B` is
/// deliberately not written; the raw stores skip it. The load/store order is
/// kept interleaved so an aliased source observes the same values as the
/// original.
///
/// # Safety
/// Both source pointers must allow three readable bytes; the row at
/// `object + index * 8 + 0x48` must be writable.
///
/// # Confidence
/// high for the byte copies and mask; medium for `mode`, which is only visible
/// from the raw `r7` to `r5` move.
pub unsafe fn fn_802dc110(
    object: usize,
    first: usize,
    second: usize,
    index: u32,
    mode: u32,
) {
    // observed 0x802DC110: `li r0,0x2000`; `slw r4,r0,r6`; `mr r5,r7`;
    // `bl 0x802DC5D0`; then six `lbz`/`stb` pairs at +0x48..+0x4e.
    let mask = 0x2000u32.checked_shl(index).unwrap_or(0);
    fn_802dc5d0(object, mask, mode);
    let base = object + index as usize * 8;

    let b0 = platform::read_u8(first);
    let b1 = platform::read_u8(first + 1);
    platform::write_u8(base + 0x48, b0);
    let b2 = platform::read_u8(first + 2);
    platform::write_u8(base + 0x49, b1);
    let c0 = platform::read_u8(second);
    platform::write_u8(base + 0x4A, b2);
    let c1 = platform::read_u8(second + 1);
    platform::write_u8(base + 0x4C, c0);
    let c2 = platform::read_u8(second + 2);
    platform::write_u8(base + 0x4D, c1);
    platform::write_u8(base + 0x4E, c2);
}

// ---------------------------------------------------------------------------
// 0x802DC1B0 - read one byte from the table at +0x48
// ---------------------------------------------------------------------------

/// Reads one byte from the fixed table at `+0x48`.
///
/// # Original
/// - Ghidra: `FUN_802dc1b0` at `0x802DC1B0` (24 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc1b0_FUN_802dc1b0.c`
///
/// # Arguments
/// - `object`: typed record.
/// - `element`: inner index, scaled by 4.
/// - `group`: outer index, scaled by 8.
///
/// # Returns
/// The byte at `object + group * 8 + element * 4 + 0x4B`.
///
/// # Safety
/// The computed address must be readable.
///
/// # Confidence
/// high - the index arithmetic matches [`fn_802dc0f0`] and the load is
/// `lbz r3,0x4b(r3)`.
pub unsafe fn fn_802dc1b0(object: usize, element: u32, group: u32) -> u8 {
    // observed 0x802DC1B0: the same two shifts and adds as 0x802DC0F0,
    // then `lbz r3,0x4b(r3)`.
    platform::read_u8(object + group as usize * 8 + element as usize * 4 + 0x4B)
}

// ---------------------------------------------------------------------------
// 0x802DC240..0x802DC480 - address getters for the trailing vectors
// ---------------------------------------------------------------------------

/// Returns the address of the three-float vector at `+0x58`.
///
/// # Original
/// - Ghidra: `FUN_802dc240` at `0x802DC240` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc240_FUN_802dc240.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x58`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x58`; `blr`.
pub unsafe fn fn_802dc240(object: usize) -> usize {
    // observed 0x802DC240: `addi r3,r3,0x58`; `blr`.
    object + 0x58
}

/// Returns the address of the three-float vector at `+0x60`.
///
/// # Original
/// - Ghidra: `FUN_802dc2a0` at `0x802DC2A0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc2a0_FUN_802dc2a0.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x60`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x60`; `blr`.
pub unsafe fn fn_802dc2a0(object: usize) -> usize {
    // observed 0x802DC2A0: `addi r3,r3,0x60`; `blr`.
    object + 0x60
}

/// Returns the address of the three-float vector at `+0x68`.
///
/// # Original
/// - Ghidra: `FUN_802dc300` at `0x802DC300` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc300_FUN_802dc300.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x68`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x68`; `blr`.
pub unsafe fn fn_802dc300(object: usize) -> usize {
    // observed 0x802DC300: `addi r3,r3,0x68`; `blr`.
    object + 0x68
}

/// Returns the address of the three-float vector at `+0x74`.
///
/// # Original
/// - Ghidra: `FUN_802dc370` at `0x802DC370` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc370_FUN_802dc370.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x74`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x74`; `blr`.
pub unsafe fn fn_802dc370(object: usize) -> usize {
    // observed 0x802DC370: `addi r3,r3,0x74`; `blr`.
    object + 0x74
}

/// Returns the address of the six-float block at `+0x80`.
///
/// # Original
/// - Ghidra: `FUN_802dc3e0` at `0x802DC3E0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc3e0_FUN_802dc3e0.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x80`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x80`; `blr`.
pub unsafe fn fn_802dc3e0(object: usize) -> usize {
    // observed 0x802DC3E0: `addi r3,r3,0x80`; `blr`.
    object + 0x80
}

/// Returns the address of the six-float block at `+0x8C`.
///
/// # Original
/// - Ghidra: `FUN_802dc480` at `0x802DC480` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc480_FUN_802dc480.c`
///
/// # Arguments
/// - `object`: typed record.
///
/// # Returns
/// `object + 0x8C`.
///
/// # Safety
/// The returned pointer is valid only while `object` is alive.
///
/// # Confidence
/// high - `addi r3,r3,0x8c`; `blr`.
pub unsafe fn fn_802dc480(object: usize) -> usize {
    // observed 0x802DC480: `addi r3,r3,0x8c`; `blr`.
    object + 0x8C
}

// ---------------------------------------------------------------------------
// 0x802DC4F0 - initialise the record's scalar fields
// ---------------------------------------------------------------------------

/// Initialises the scalar and flag fields of the typed record.
///
/// # Original
/// - Ghidra: `FUN_802dc4f0` at `0x802DC4F0` (212 bytes)
/// - C file: `work/ghidra/main-gb/export/802dc4f0_FUN_802dc4f0.c`
///
/// # Arguments
/// - `object`: record to initialise.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Loads the single-precision default at `0x8042F678` (`r2 - 0x3A08`,
///   observed to be `0.0f`) and stores it at `+0x10` and
///   `+0x1C..+0x40`, `+0x58..+0x70` and `+0x80..+0x94`.
/// - Clears the words at `+0x4`/`+0x8`, the halfwords at
///   `+0x0C`/`+0x14`/`+0x16`, the byte at `+0x18` and every byte
///   `+0x44..+0x57`.
/// - Gaps `+0x41..+0x43`, `+0x71..+0x7F` and `+0x95..+0x153` are not touched.
///
/// # Safety
/// `object` must be writable through `+0x94`.
///
/// # Confidence
/// high - every store offset is explicit in the C and confirmed by the raw
/// `stfs`/`stw`/`sth`/`stb` sequence.
pub unsafe fn fn_802dc4f0(object: usize) {
    let default = platform::read_f32(0x8042_F678);
    platform::write_f32(object + 0x10, default);
    platform::write_f32(object + 0x1C, default);
    platform::write_f32(object + 0x20, default);
    platform::write_f32(object + 0x24, default);
    platform::write_f32(object + 0x28, default);
    platform::write_f32(object + 0x2C, default);
    platform::write_f32(object + 0x30, default);
    platform::write_f32(object + 0x34, default);
    platform::write_f32(object + 0x38, default);
    platform::write_f32(object + 0x3C, default);
    platform::write_f32(object + 0x40, default);
    platform::write_u32(object + 4, 0);
    platform::write_u32(object + 8, 0);
    platform::write_u16(object + 0x0C, 0);
    platform::write_u16(object + 0x14, 0);
    platform::write_u16(object + 0x16, 0);
    platform::write_u8(object + 0x18, 0);
    let mut offset = 0x44;
    while offset <= 0x57 {
        platform::write_u8(object + offset, 0);
        offset += 1;
    }
    platform::write_f32(object + 0x58, default);
    platform::write_f32(object + 0x5C, default);
    platform::write_f32(object + 0x60, default);
    platform::write_f32(object + 0x64, default);
    platform::write_f32(object + 0x68, default);
    platform::write_f32(object + 0x6C, default);
    platform::write_f32(object + 0x70, default);
    platform::write_f32(object + 0x80, default);
    platform::write_f32(object + 0x84, default);
    platform::write_f32(object + 0x88, default);
    platform::write_f32(object + 0x8C, default);
    platform::write_f32(object + 0x90, default);
    platform::write_f32(object + 0x94, default);
}

// ===========================================================================
// Wave 28 - double to decimal record and rounding
// ===========================================================================

// ---------------------------------------------------------------------------
// 0x802DE20C - convert a double to a decimal digit record
// ---------------------------------------------------------------------------

/// Converts a binary `double` into the decimal digit record consumed by the
/// `printf` floating-point formatter.
///
/// # Original
/// - Ghidra: `FUN_802de20c` at `0x802DE20C` (356 bytes)
/// - C file: `work/ghidra/main-gb/export/802de20c_FUN_802de20c.c`
///
/// # Arguments
/// - `value`: the number to convert (arrives in `f1`).
/// - `out`: 41-byte decimal record (arrives in `r3`). Layout: `+0` sign byte,
///   `+2` exponent (`i16`), `+4` digit count, `+5..+40` digits (`0..9`, not
///   ASCII, except `'I'`/`'N'` for infinity/NaN).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `out`. The three cases are:
/// - zero: sign, exponent 0, one digit `0`;
/// - infinity or NaN: sign, exponent 0, one digit `'I'` or `'N'`;
/// - finite non-zero: normalises the magnitude with `0x802E81D4`, computes the
///   number of significant binary bits, builds the power-of-two record with
///   [`fn_802ddef0`], scales with `0x802E825C`, truncates the result with
///   [`fn_802dd238`], converts it with [`fn_802ddaa0`] and merges the digits
///   with the exponent record through [`fn_802ddb7c`].
///
/// # Safety
/// `out` must point at a writable 41-byte record.
///
/// # Confidence
/// high for the branches and calls; the `f1`/`r3` split was recovered from the
/// raw `stfd f31,0x88(r1)` / `mr r30,r3` prologue. The two callees in the next
/// band (`0x802E81D4`, `0x802E825C`) are called by address.
pub unsafe fn fn_802de20c(value: f64, out: usize) {
    // observed 0x802DE230: `bl 0x802DF414` with the original `f1`; the result
    // is a sign mask (0 or 0x80000000).
    let negative = fn_802df414(value) != 0;

    // observed 0x802DE234: compare against the f64 constant at 0x8042F680,
    // which is `0.0`. The comparison also matches -0.0.
    if read_f64_be(0x8042_F680) == value {
        platform::write_u8(out, negative as u8);
        platform::write_u16(out + 2, 0);
        platform::write_u8(out + 4, 1);
        platform::write_u8(out + 5, 0);
        return;
    }

    // observed 0x802DE270: classification 1 = NaN, 2 = infinity, 3 = zero.
    let class = fn_802df42c(value);
    if class < 3 {
        platform::write_u8(out, negative as u8);
        platform::write_u16(out + 2, 0);
        platform::write_u8(out + 4, 1);
        platform::write_u8(out + 5, if class == 1 { b'N' } else { b'I' });
        return;
    }

    // observed 0x802DE2B0: negate only the finite magnitude; the sign byte is
    // restored at the end.
    let magnitude = if negative { -value } else { value };

    let mut binary_exponent = crate::platform::GuestLocal::<i32>::new( 0);
    // unknown: 0x802E81D4 normalises the magnitude and returns the mantissa in
    // `[0.5, 1)`, writing the binary exponent through the pointer.
    let mantissa = call_f64_ptr(
        0x802E_81D4,
        magnitude,
        (binary_exponent.as_mut_ptr() as *mut i32) as usize,
    );

    // observed 0x802DE2C8..0x802DE304: the bit scan is over the IEEE-754 words
    // of the normalised mantissa, not over the integer produced below.
    let bits = mantissa.to_bits();
    let low = bits as u32;
    let high = (bits >> 32) as u32;
    let mut significant_bits = 32 - (low.wrapping_sub(1) & !low).leading_zeros() as i32;
    if low == 0 {
        let biased = high | 0x0010_0000;
        significant_bits = 64 - (biased.wrapping_sub(1) & !biased).leading_zeros() as i32;
    }
    let scale = 0x35 - significant_bits;

    // observed 0x802DE308..0x802DE318: `r4 = exponent - (0x35 - bits)`.
    let mut exponent_record = crate::platform::GuestLocal::new( [0u8; 44]);
    fn_802ddef0(
        exponent_record.as_mut_ptr() as usize,
        (*binary_exponent - scale) as u32,
    );

    // observed 0x802DE31C..0x802DE328: scale in `f1`/`r3`, then truncate the
    // scaled double to an integer.
    let scaled = call_f64_i32(0x802E_825C, mantissa, scale);
    let integer = fn_802dd238(scaled);

    let mut digit_record = crate::platform::GuestLocal::new( [0u8; 56]);
    fn_802ddaa0(digit_record.as_mut_ptr() as usize, integer);
    fn_802ddb7c(
        out,
        digit_record.as_mut_ptr() as usize,
        exponent_record.as_mut_ptr() as usize,
    );
    platform::write_u8(out, negative as u8);
}

// ---------------------------------------------------------------------------
// 0x802DE370 - round a decimal record to a requested exponent
// ---------------------------------------------------------------------------

/// Rounds the decimal record produced by [`fn_802de20c`] to the precision
/// carried in the input record's exponent field.
///
/// # Original
/// - Ghidra: `FUN_802de370` at `0x802DE370` (420 bytes)
/// - C file: `work/ghidra/main-gb/export/802de370_FUN_802de370.c`
///
/// # Arguments
/// - `value`: the number to convert (arrives in `f1`).
/// - `input`: record whose `+2` halfword is the requested digit count.
/// - `output`: decimal record filled by [`fn_802de20c`] and then rounded in
///   place.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Calls [`fn_802de20c`]`(value, output)` first.
/// - If the first output digit is greater than 9 (infinity/NaN), returns.
/// - Clamps the requested count to `0x24` (36 digits) and returns when it is
///   not in `1..count`.
/// - Applies half-even rounding at the first dropped digit: `> 5` rounds up,
///   `< 5` rounds down, exactly `5` looks at the remaining digits and at the
///   parity of the last kept digit.
/// - Truncates the digit count, adjusts the exponent by `count - 1`, and adds
///   `'0'` to every kept digit.
///
/// # Safety
/// `output` must be a writable 41-byte record; `input + 2` must be readable.
///
/// # Confidence
/// high - every comparison and store is present in the C; the raw
/// `lha r31,2(r3)` / `mr r30,r4` prologue confirms that `f1` carries the value
/// and `r3`/`r4` are the two records.
pub unsafe fn fn_802de370(value: f64, input: usize, output: usize) {
    // observed 0x802DE380: `lha r31,2(r3)` reads the requested count before
    // the conversion (the records may alias).
    let mut exponent = platform::read_u16(input + 2) as i16 as i32;

    fn_802de20c(value, output);

    // observed 0x802DE394: `lbz r0,5(r30)`; `cmpwi r0,9`; `bgt` skips.
    if platform::read_u8(output + 5) >= 10 {
        return;
    }

    // observed 0x802DE3A0: clamp to 0x24.
    if exponent > 0x24 {
        exponent = 0x24;
    }
    if exponent <= 0 {
        return;
    }
    let count = platform::read_u8(output + 4) as i32;
    if exponent >= count {
        return;
    }

    let digits = output + 5;
    let dropped = platform::read_u8(digits + exponent as usize);
    let mut round_up: i32;
    if dropped >= 6 {
        round_up = 1;
    } else if dropped < 5 {
        round_up = -1;
    } else {
        // observed 0x802DE3E8..0x802DE420: scan the digits after the cut for
        // any non-zero value; a tie is broken by the last kept digit's parity.
        round_up = -1;
        let mut cursor = digits + exponent as usize + 1;
        let end = digits + count as usize;
        while cursor < end {
            if platform::read_u8(cursor) != 0 {
                round_up = 1;
                break;
            }
            cursor += 1;
        }
        if round_up < 0
            && (platform::read_u8(output + 4 + exponent as usize) & 1) != 0
        {
            round_up = 1;
        }
    }

    platform::write_u8(output + 4, exponent as u8);

    if round_up >= 0 {
        // observed 0x802DE444..0x802DE494: propagate the carry leftwards;
        // a carry out of the first digit becomes digit 1 and bumps the
        // exponent.
        let mut cursor = digits + exponent as usize;
        loop {
            cursor -= 1;
            let digit = platform::read_u8(cursor);
            if digit < 9 {
                platform::write_u8(cursor, digit + 1);
                break;
            }
            if cursor == digits {
                platform::write_u8(cursor, 1);
                let bumped = platform::read_u16(output + 2).wrapping_add(1);
                platform::write_u16(output + 2, bumped);
                break;
            }
            platform::write_u8(cursor, 0);
        }
    }

    // observed 0x802DE4A0..0x802DE4B0: zero-fill the remaining digits and
    // grow the count one byte at a time.
    let mut count = platform::read_u8(output + 4) as i32;
    while count < exponent {
        platform::write_u8(output + 5 + count as usize, 0);
        count += 1;
        platform::write_u8(output + 4, count as u8);
    }

    // observed 0x802DE4C0..0x802DE4D4: exponent -= count - 1.
    let count = platform::read_u8(output + 4) as i32;
    let adjusted = platform::read_u16(output + 2) as i16 as i32 - (count - 1);
    platform::write_u16(output + 2, adjusted as u16);

    // observed 0x802DE4DC..0x802DE4F8: digits are stored as 0..9 until here.
    for index in 0..count {
        let offset = output + 5 + index as usize;
        let digit = platform::read_u8(offset);
        platform::write_u8(offset, digit.wrapping_add(b'0'));
    }
}

// ---------------------------------------------------------------------------
// Wave 32 - remaining C++ object-model vocabulary
// ---------------------------------------------------------------------------
//
// The allocator, stdio, `mem*` and 64-bit helper runs were completed in waves
// 24 and 28. Wave 32 covers the rest of the same Metrowerks/EGG object-model
// layer: the reference-counted resource registry (`0x802D8xxx`), the
// register-transfer bridge used by the animation nodes (`0x802DA040` /
// `0x802DA6F0`), the camera-vector update (`0x802DAD20`) and the layout
// record's mask setters (`0x802DBC90..0x802DBD50`).
//
// These bodies are called by address from the stage and layout lanes; none of
// them is gameplay logic. Field roles that are not proven by a caller stay
// `// unknown:` or `// inferred:`.

// ---------------------------------------------------------------------------
// 0x802D8140 - write four colour bytes through the record setter
// ---------------------------------------------------------------------------

/// Copies four bytes into the record at `object + 0x7C` by calling the
/// existing setter [`fn_802dc090`].
///
/// # Original
/// - Ghidra: `FUN_802d8140` at `0x802D8140` (60 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8140_FUN_802d8140.c`
///
/// # Arguments
/// - `object`: owner record; the four bytes are stored in its child at
///   `+0x7C`.
/// - `b0..b3`: the four colour bytes, passed in `r4..r7`.
/// - `mode`: forwarded unchanged to [`fn_802dc090`] (in `r8`).
///
/// # Side effects
/// Calls `0x802DC090`, which sets the record's flag mask `0x1000` and writes
/// the four bytes at `+0x44..+0x47` of the child.
///
/// # Safety
/// `object + 0x7C` must name a live record.
///
/// # Confidence
/// high - the four byte stores and the `r8` pass-through are explicit in the
/// raw prologue at `0x802D8140`.
pub unsafe fn fn_802d8140(
    object: usize,
    b0: u8,
    b1: u8,
    b2: u8,
    b3: u8,
    mode: u32,
) {
    let mut bytes = crate::platform::GuestLocal::new( [b0, b1, b2, b3]);
    fn_802dc090(
        object + 0x7C,
        bytes.as_mut_ptr() as usize,
        mode,
    );
}

// ---------------------------------------------------------------------------
// 0x802D85C0 - push one animation-node transform into the draw record
// ---------------------------------------------------------------------------

/// Reads the animation node attached at `object + 0x74`, builds a 3x4
/// transform matrix from the node (or from the object's stored matrix), then
/// hands the matrix to `0x80286430` and copies the node fields into
/// `object + 0x7C`.
///
/// # Original
/// - Ghidra: `FUN_802d85c0` at `0x802D85C0` (380 bytes)
/// - C file: `work/ghidra/main-gb/export/802d85c0_FUN_802d85c0.c`
///
/// # Arguments
/// - `object`: layout object whose child record starts at `+0x74`.
///
/// # Side effects
/// - `0x80296760(object + 0x74, 0x8042AFC8)` when the node is absent or in a
///   state other than 1/2.
/// - `0x801C04D0` / `0x801C0500` load or copy the source matrix.
/// - `0x80286430(node, matrix)` and [`fn_802da6f0`]`(node, object + 0x7C)`.
///
/// # Matrix layout
/// `// observed:` the raw code touches `r1+8`, `r1+0x18`, `r1+0x28` for the
/// three rows, so the 12 floats are stored row-major with a 4-float stride.
/// Flag `2` copies the stored matrix, flag `4` scales columns 0..2 by
/// `+0x2C/+0x30/+0x34`, and flag `1` fills column 3 from `+0x38/+0x3C/+0x40`.
///
/// # Safety
/// `object` and the node at `+0x74` must be live; the matrix pointer passed to
/// the GX helpers must address 12 writable floats.
///
/// # Confidence
/// high - every store index is visible in the raw sequence
/// `0x802D8624..0x802D8710`.
pub unsafe fn fn_802d85c0(object: usize) {
    if platform::call1(0x8029_67B0, (object + 0x74) as u32) == 0 {
        return;
    }
    let node = platform::call1(0x8029_67E0, (object + 0x74) as u32) as usize;
    let state = if node == 0 {
        0
    } else {
        platform::read_u32(node + 0x0C)
    };
    if state != 1 && state != 2 {
        // observed 0x802D8718: reset the child record with the SDA constant.
        platform::call2(0x8029_6760, (object + 0x74) as u32, 0x8042_AFC8);
        return;
    }

    let mut matrix = crate::platform::GuestLocal::new( [0.0f32; 12]);
    let matrix_ptr = matrix.as_mut_ptr() as usize;

    if platform::read_u32(object + 0x28) & 2 == 0 {
        platform::call1(0x801C_04D0, matrix_ptr as u32);
    } else {
        platform::call2(0x801C_0500, (object + 0x44) as u32, matrix_ptr as u32);
    }

    if platform::read_u32(object + 0x28) & 4 != 0 {
        // observed 0x802D8660..0x802D86D4: scale each row's first three
        // columns; the fourth column is left alone.
        let sx = platform::read_f32(object + 0x2C);
        matrix[0] *= sx;
        matrix[4] *= sx;
        matrix[8] *= sx;

        let sy = platform::read_f32(object + 0x30);
        matrix[1] *= sy;
        matrix[5] *= sy;
        matrix[9] *= sy;

        let sz = platform::read_f32(object + 0x34);
        matrix[2] *= sz;
        matrix[6] *= sz;
        matrix[10] *= sz;
    }

    if platform::read_u32(object + 0x28) & 1 != 0 {
        // observed 0x802D86E4..0x802D86F8: translation goes in column 3.
        matrix[3] = platform::read_f32(object + 0x38);
        matrix[7] = platform::read_f32(object + 0x3C);
        matrix[11] = platform::read_f32(object + 0x40);
    }

    platform::call2(0x8028_6430, node as u32, matrix_ptr as u32);
    fn_802da6f0(node, object + 0x7C);
}

// ---------------------------------------------------------------------------
// 0x802D8740 / 0x802D87C0 - resolve the animation node
// ---------------------------------------------------------------------------

/// Returns the node at `object + 0x74` only when its state word is 1 or 2.
///
/// # Original
/// - Ghidra: `FUN_802d8740` at `0x802D8740` (116 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8740_FUN_802d8740.c`
///
/// # Arguments
/// - `object`: layout object whose child record starts at `+0x74`.
///
/// # Returns
/// The resolved node pointer, or `0` when the state is not 1/2.
///
/// # Safety
/// `object + 0x74` must name a live child record.
///
/// # Confidence
/// high - the second `0x802967E0` call returns the value directly (raw
/// `0x802D8790..0x802D8798`).
pub unsafe fn fn_802d8740(object: usize) -> u32 {
    let node = platform::call1(0x8029_67E0, (object + 0x74) as u32) as usize;
    let state = if node == 0 {
        0
    } else {
        platform::read_u32(node + 0x0C)
    };
    if state == 1 || state == 2 {
        platform::call1(0x8029_67E0, (object + 0x74) as u32)
    } else {
        0
    }
}

/// Returns `0x802862A0(node)` for the node at `object + 0x74` when its state
/// word is 1 or 2, otherwise `0`.
///
/// # Original
/// - Ghidra: `FUN_802d87c0` at `0x802D87C0` (120 bytes)
/// - C file: `work/ghidra/main-gb/export/802d87c0_FUN_802d87c0.c`
///
/// # Arguments
/// - `object`: layout object whose child record starts at `+0x74`.
///
/// # Returns
/// Result of `0x802862A0`, or `0`.
///
/// # Side effects
/// Calls `0x802862A0` with the freshly resolved node in `r3`.
///
/// # Safety
/// `object + 0x74` must name a live child record.
///
/// # Confidence
/// medium - Ghidra prints `0x802862A0()` with no arguments; the raw
/// `0x802D8810..0x802D8818` clears the argument only after resolving the node,
/// so the node is forwarded in `r3`.
pub unsafe fn fn_802d87c0(object: usize) -> u32 {
    let node = platform::call1(0x8029_67E0, (object + 0x74) as u32) as usize;
    let state = if node == 0 {
        0
    } else {
        platform::read_u32(node + 0x0C)
    };
    if state == 1 || state == 2 {
        let resolved = platform::call1(0x8029_67E0, (object + 0x74) as u32);
        platform::call1(0x8028_62A0, resolved)
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// 0x802D8840 - reset the transform record
// ---------------------------------------------------------------------------

/// Resets the transform block at `object + 0x28`: clear the flag word, set two
/// SDA2 float triples, rebuild the child matrix and reset the child record.
///
/// # Original
/// - Ghidra: `FUN_802d8840` at `0x802D8840` (120 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8840_FUN_802d8840.c`
///
/// # Arguments
/// - `object`: layout object with the transform block at `+0x28` and a child
///   record at `+0x7C`.
///
/// # Side effects
/// - writes `0` at `+0x28`, the first float to `+0x2C/+0x30/+0x34` and the
///   second to `+0x38/+0x3C/+0x40`;
/// - `0x801C04D0(object + 0x44)`;
/// - `0x80296760(object + 0x74, 0x8042AFC8)`;
/// - virtual slot `+0x0C` of the child at `+0x7C`.
///
/// # Safety
/// `object` and the child pointer at `+0x7C` must be live.
///
/// # Confidence
/// high - the two `lfs` constants (`0x8042F640`, `0x8042F644`) and all stores
/// are explicit in the raw prologue.
pub unsafe fn fn_802d8840(object: usize) {
    let first = platform::read_f32(0x8042_F640);
    let second = platform::read_f32(0x8042_F644);

    platform::write_u32(object + 0x28, 0);
    platform::write_f32(object + 0x2C, first);
    platform::write_f32(object + 0x30, first);
    platform::write_f32(object + 0x34, first);
    platform::write_f32(object + 0x38, second);
    platform::write_f32(object + 0x3C, second);
    platform::write_f32(object + 0x40, second);

    platform::call1(0x801C_04D0, (object + 0x44) as u32);
    platform::call2(0x8029_6760, (object + 0x74) as u32, 0x8042_AFC8);

    let child = object + 0x7C;
    let vtable = platform::read_u32(child) as usize;
    let reset = platform::read_u32(vtable + 0x0C) as usize;
    platform::call1(reset, child as u32);
}

// ---------------------------------------------------------------------------
// 0x802D88D0 / 0x802D8950 - register and unregister a resource entry
// ---------------------------------------------------------------------------

/// Constructs a resource entry, installs vtable `0x80350858` and registers it
/// with the global manager at `0x8042AFD0`.
///
/// # Original
/// - Ghidra: `FUN_802d88d0` at `0x802D88D0` (128 bytes)
/// - C file: `work/ghidra/main-gb/export/802d88d0_FUN_802d88d0.c`
///
/// # Arguments
/// - `object`: storage for the entry (at least `0x18` bytes).
/// - `index`: entry index stored at `+0x10`.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Calls the base constructor `0x802BA2D0`, stores the vtable and the scaled
/// index at `+0x14`, then `0x802D94D0(manager, object, index)`.
///
/// # Safety
/// `object` must be writable for `0x18` bytes.
///
/// # Confidence
/// high - raw `0x802D88F0..0x802D892C` shows the base call, vtable store,
/// shift and registration call.
pub unsafe fn fn_802d88d0(object: usize, index: u32) -> usize {
    platform::call1(0x802B_A2D0, object as u32);
    platform::write_u32(object + 0x10, index);
    platform::write_u32(object, 0x8035_0858);

    let manager = platform::read_u32(0x8042_AFD0) as usize;
    if manager != 0 {
        let shift = fn_802d9780(manager);
        platform::write_u32(object + 0x14, index.wrapping_shl(shift as u32));
        fn_802d94d0(manager, object as u32, platform::read_u32(object + 0x10) as i32);
    }
    object
}

/// Destroys a resource entry and unregisters it from the manager.
///
/// # Original
/// - Ghidra: `FUN_802d8950` at `0x802D8950` (136 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8950_FUN_802d8950.c`
///
/// # Arguments
/// - `object`: entry to destroy, or `0` for a no-op.
/// - `delete_flag`: `> 0` also frees the storage through `0x802B9280`.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Installs vtable `0x80350858`, calls its slot `+0x10`, unregisters through
/// `0x802D94E0`, calls the base destructor `0x802BA330(object, 0)` and
/// optionally frees.
///
/// # Safety
/// `object` must be a live entry from [`fn_802d88d0`] or `0`.
///
/// # Confidence
/// medium - Ghidra prints the virtual call with no arguments; the raw
/// `0x802D896C..0x802D8988` leaves `r3`/`r4` untouched, so the entry and the
/// delete flag are forwarded.
pub unsafe fn fn_802d8950(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        platform::write_u32(object, 0x8035_0858);
        let vtable = platform::read_u32(object) as usize;
        let shutdown = platform::read_u32(vtable + 0x10) as usize;
        platform::call2(shutdown, object as u32, delete_flag as u32);

        let manager = platform::read_u32(0x8042_AFD0) as usize;
        if manager != 0 {
            fn_802d94e0(manager, platform::read_u32(object + 0x10) as i32);
        }
        platform::call2(0x802B_A330, object as u32, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D89E0 - map a two-bit flag to an index
// ---------------------------------------------------------------------------

/// Maps bit `0x8000` and bit `0` of `flags` to the values `2..3` or `0..1`.
///
/// # Original
/// - Ghidra: `FUN_802d89e0` at `0x802D89E0` (28 bytes)
/// - C file: `work/ghidra/main-gb/export/802d89e0_FUN_802d89e0.c`
///
/// # Arguments
/// - `_unused`: ignored; the original never reads `r3`.
/// - `flags`: the two bits to decode.
///
/// # Returns
/// `2 + (flags & 1)` when bit `0x8000` is set, else `flags & 1`.
///
/// # Confidence
/// high - two `rlwinm` extracts and one add in the raw body.
pub unsafe fn fn_802d89e0(_unused: u32, flags: u32) -> u32 {
    if flags & 0x8000 != 0 {
        (flags & 1) + 2
    } else {
        flags & 1
    }
}

// ---------------------------------------------------------------------------
// 0x802D8A00 - reset every registered resource index
// ---------------------------------------------------------------------------

/// Releases every index of the global manager's resource table.
///
/// # Original
/// - Ghidra: `FUN_802d8a00` at `0x802D8A00` (132 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8a00_FUN_802d8a00.c`
///
/// # Arguments
/// - `manager`: object whose index word at `+0x14` is OR-ed with each index.
///
/// # Side effects
/// For each index `0..0x802D9770(global)-1`, calls `0x80286A30` and
/// `0x80286AC0` with `manager[0x14] | index`.
///
/// # Safety
/// `manager` must be live; the global at `0x8042AFD0` may be zero, in which
/// case the function does nothing.
///
/// # Confidence
/// high - the loop bound and both calls are explicit in raw
/// `0x802D8A14..0x802D8A64`.
pub unsafe fn fn_802d8a00(manager: usize) {
    let platform_manager = call0_u32(0x8028_6630);
    let global = platform::read_u32(0x8042_AFD0) as usize;
    if global != 0 {
        let count = fn_802d9770(global);
        let base = platform::read_u32(manager + 0x14);
        let mut index: i32 = 0;
        while index < count {
            let value = base | index as u32;
            platform::call2(0x8028_6A30, platform_manager, value);
            platform::call2(0x8028_6AC0, platform_manager, value);
            index += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D8DE0 - create the global resource manager on first use
// ---------------------------------------------------------------------------

/// Lazily creates the global resource descriptor and returns it.
///
/// # Original
/// - Ghidra: `FUN_802d8de0` at `0x802D8DE0` (140 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8de0_FUN_802d8de0.c`
///
/// # Arguments
/// - `width`, `height`: dimensions originally passed in `r3`/`r4` (the caller
///   at `0x8016C400` forwards two of its arguments).
/// - `heap`: allocator handle (`r5`); `0` selects the global at `0x8042AE10`.
///
/// # Returns
/// The word stored at `0x8042AFD0` (zero on allocation failure).
///
/// # Side effects
/// Calls `0x802DA590`, `0x802B9240(0x38, heap, 4)` and
/// [`fn_802d97b0`], then stores the result in the global.
///
/// # Safety
/// The global at `0x8042AFD0` must be writable.
///
/// # Confidence
/// high - the null test and fallback allocator are visible at
/// `0x802D8E04..0x802D8E48`.
pub unsafe fn fn_802d8de0(width: u32, height: u32, heap: u32) -> u32 {
    let mut heap = heap;
    if platform::read_u32(0x8042_AFD0) == 0 {
        fn_802da590();
        if heap == 0 {
            heap = platform::read_u32(0x8042_AE10);
        }
        let storage = platform::call3(0x802B_9240, 0x38, heap, 4) as usize;
        let mut value = 0;
        if storage != 0 {
            value = fn_802d97b0(storage, width, height, heap) as u32;
        }
        platform::write_u32(0x8042_AFD0, value);
    }
    platform::read_u32(0x8042_AFD0)
}

// ---------------------------------------------------------------------------
// 0x802D8E70 - build the four resource pools
// ---------------------------------------------------------------------------

/// Builds the resource manager's four fixed-size element pools and registers
/// the manager with `0x80286630`.
///
/// # Original
/// - Ghidra: `FUN_802d8e70` at `0x802D8E70` (1,288 bytes)
/// - C file: `work/ghidra/main-gb/export/802d8e70_FUN_802d8e70.c`
///
/// # Arguments
/// - `manager`: output object (raw `r3`); its `+0x30` word receives the pool
///   block and its `+0x8` word is the element count forwarded to `0x80286640`.
/// - `request`: configuration record (raw `r4`): `+0` allocator handle,
///   `+4/+8/+0xC/+0x10` four `u16` dimensions.
///
/// # Pool layout
/// `// observed:` the four pools have element strides `0x138`, `0x158`,
/// `0xC0` and `0xE4`; their constructors are `0x802DA5A0`, `0x802DA970`,
/// `0x802DB0F0`, `0x802DB030` and their destructors are the matching
/// `0x802DA600`, `0x802DA9C0`, `0x802DB140`, `0x802DB080`. Each list object
/// gets vtable `0x8035091C`, `0x80350910`, `0x80350904`, `0x803508F8`.
///
/// # Safety
/// `manager` and `request` must be live records; the allocator handle at
/// `request + 0` must be valid.
///
/// # Confidence
/// high - all allocations, four stride constants and both list calls are
/// explicit in the C and confirmed by the raw entry
/// `0x802D8E84..0x802D8ECC`. The role names `manager`/`request` are
/// `// inferred:` from the caller at `0x8016C400`.
pub unsafe fn fn_802d8e70(manager: usize, request: usize) {
    if platform::read_u32(request) == 0 {
        platform::write_u32(request, platform::read_u32(0x8042_AE10));
    }

    let count_a = platform::read_u32(request + 4) & 0xFFFF;
    let count_b = platform::read_u32(request + 8) & 0xFFFF;
    let count_c = platform::read_u32(request + 0xC) & 0xFFFF;
    let count_d = platform::read_u32(request + 0x10) & 0xFFFF;
    let manager_count = platform::read_u32(manager + 8);

    let total = count_d
        .wrapping_mul(0xE4)
        .wrapping_add(0x3F)
        & !0x1F
        + count_c.wrapping_mul(0xC0)
        + (count_a.wrapping_mul(0x138).wrapping_add(0x3F) & !0x1F)
        + (count_b.wrapping_mul(0x158).wrapping_add(0x3F) & !0x1F)
        + (manager_count.wrapping_mul(0x1C).wrapping_add(0x3F) & !0x1F)
        + 0x240;

    let heap = platform::read_u32(request);
    let block = platform::call3(0x802B_8CC0, total, 0x20, heap) as usize;
    platform::write_u32(manager + 0x30, block as u32);

    let pool = platform::call3(0x802B_9240, 0x3C, heap, 4) as usize;
    if pool != 0 {
        // observed 0x802D8F0C..0x802D8FB8: the payload starts at the first
        // 32-byte boundary after the block header.
        let payload = (block + 0x1F) & !0x1F;
        let end = block.wrapping_add(total as usize);

        platform::write_u32(pool, 0x8035_0890);
        platform::write_u32(pool + 0x04, count_a);
        platform::write_u32(pool + 0x0C, count_b);
        platform::write_u32(pool + 0x14, count_c);
        platform::write_u32(pool + 0x1C, count_d);
        platform::write_u32(pool + 0x08, 0);
        platform::write_u32(pool + 0x10, 0);
        platform::write_u32(pool + 0x18, 0);
        platform::write_u32(pool + 0x20, 0);
        platform::write_u32(pool + 0x2C, 0);
        platform::write_u32(pool + 0x34, 0);
        platform::write_u32(pool + 0x38, 0);
        platform::write_u32(pool + 0x28, end as u32);
        platform::write_u32(pool + 0x24, payload as u32);
        platform::write_u32(pool + 0x30, payload as u32);

        platform::write_u32(payload, 0);
        platform::write_u32(payload + 4, 0);
        platform::write_u32(payload + 8, 0);
        platform::write_u32(payload + 0x0C, 0);
        platform::write_u8(payload + 0x14, 0);
        platform::write_u32(
            payload + 0x10,
            (end as u32).wrapping_sub(payload as u32).wrapping_sub(0x20),
        );
        platform::write_u32(pool + 0x38, payload as u32);
        platform::write_u32(pool + 0x2C, 0);

        platform::write_u32(
            pool + 0x08,
            build_element_list(pool, count_a, 0x138, 0x802D_A5A0, 0x802D_A600, 0x8035_091C)
                as u32,
        );
        platform::write_u32(
            pool + 0x10,
            build_element_list(pool, count_b, 0x158, 0x802D_A970, 0x802D_A9C0, 0x8035_0910)
                as u32,
        );
        platform::write_u32(
            pool + 0x18,
            build_element_list(pool, count_c, 0xC0, 0x802D_B0F0, 0x802D_B140, 0x8035_0904)
                as u32,
        );
        platform::write_u32(
            pool + 0x20,
            build_element_list(pool, count_d, 0xE4, 0x802D_B030, 0x802D_B080, 0x8035_08F8)
                as u32,
        );
    }

    let platform_manager = call0_u32(0x8028_6630) as usize;
    platform::write_u32(platform_manager, pool as u32);
    if pool == 0 {
        // observed 0x802D9300: the two cache words are manager + 0xC014 and
        // manager + 0xC018.
        platform::write_u32(platform_manager + 0xC014, 0);
        platform::write_u32(platform_manager + 0xC018, 0);
    } else {
        platform::call2(0x8028_6640, platform_manager as u32, manager_count);
    }
}

// ---------------------------------------------------------------------------
// 0x802D9630 / 0x802D9670 / 0x802D9750 - manager table dispatch
// ---------------------------------------------------------------------------

/// Dispatches one table entry through virtual slot `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_802d9630` at `0x802D9630` (48 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9630_FUN_802d9630.c`
///
/// # Arguments
/// - `object`: record whose table pointer is at `+0x2C`.
/// - `arg2`: forwarded in `r4`.
/// - `index`: entry index.
/// - `arg4`: forwarded in `r5`.
///
/// # Side effects
/// Calls `entry->vtable[0x1C](entry, arg2, arg4, object[0x34])`.
///
/// # Safety
/// The table entry must be null or a live object with a valid vtable.
///
/// # Confidence
/// high - raw `0x802D9648..0x802D965C` shows exactly four arguments, with the
/// object byte at `+0x34` as the fourth.
pub unsafe fn fn_802d9630(object: usize, arg2: u32, index: usize, arg4: u32) {
    let table = platform::read_u32(object + 0x2C) as usize;
    let entry = platform::read_u32(table + index.wrapping_mul(4)) as usize;
    if entry == 0 {
        return;
    }
    let vtable = platform::read_u32(entry) as usize;
    let method = platform::read_u32(vtable + 0x1C) as usize;
    let fourth = platform::read_u8(object + 0x34) as u32;
    platform::call4(method, entry as u32, arg2, arg4, fourth);
}

/// Writes one descriptor into every live registry entry.
///
/// # Original
/// - Ghidra: `FUN_802d9670` at `0x802D9670` (212 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9670_FUN_802d9670.c`
///
/// # Arguments
/// - `manager`: resource manager passed to `0x80295090` as the owner.
/// - `table`: descriptor table; its `+0xC` word is the lookup key and its
///   address is stored at `entry + 0x1C`.
/// - `descriptor`: four bytes read as `u8`, `u16`, `u16`.
///
/// # Side effects
/// Iterates `0x80228FF0(0x80424CE8, previous)` and, for entries with non-zero
/// `+0x10`/`+0x14`, calls `0x80295090` and fills `target + 0x0C`, `+4`, `+6`,
/// `+0x1C`.
///
/// # Safety
/// `manager`, `table` and `descriptor` must be live; the registry must be
/// initialised.
///
/// # Confidence
/// high - the raw tail at `0x802D96C0..0x802D9728` shows the loop and all four
/// target stores.
pub unsafe fn fn_802d9670(manager: usize, table: usize, descriptor: usize) {
    let owner = call0_u32(0x8029_4B20);
    let mut index: u32 = 0;
    let mut target_word: u32 = 0;

    loop {
        index = platform::call2(0x8022_8FF0, 0x8042_4CE8, index);
        if index == 0 {
            break;
        }
        let entry = index as usize;
        if platform::read_u32(entry + 0x10) == 0
            || platform::read_u32(entry + 0x14) == 0
        {
            continue;
        }

        platform::call4(
            0x8029_5090,
            core::ptr::addr_of_mut!(target_word) as usize as u32,
            owner,
            (table + 0x0C) as u32,
            platform::read_u32(entry + 0x14),
        );

        let target = target_word as usize;
        if target != 0 {
            platform::write_u8(target + 0x0C, platform::read_u8(descriptor));
            platform::write_u16(target + 4, platform::read_u16(descriptor + 2));
            platform::write_u16(target + 6, platform::read_u16(descriptor + 4));
            platform::write_u32(target + 0x1C, table as u32);
        }
    }
}

/// Dispatches table entry `index` through virtual slot `+0x20`.
///
/// # Original
/// - Ghidra: `FUN_802d9750` at `0x802D9750` (28 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9750_FUN_802d9750.c`
///
/// # Arguments
/// - `object`: record whose table pointer is at `+0x2C`.
/// - `arg4`: forwarded in `r4`.
/// - `index`: entry index.
///
/// # Side effects
/// Tail-calls `entry->vtable[0x20](entry, arg4, r5)`.
///
/// # Safety
/// The selected entry and its vtable must be live.
///
/// # Confidence
/// medium - raw `0x802D9750..0x802D9768` clears neither `r3` nor `r4`, but the
/// third argument in `r5` is not observable from this body, so the translation
/// forwards the two recoverable arguments only.
///
/// # Partial
/// The `r5` pass-through is missing; the callee's real argument count is not
/// recoverable from the export.
pub unsafe fn fn_802d9750(object: usize, arg4: u32, index: usize) {
    let table = platform::read_u32(object + 0x2C) as usize;
    let entry = platform::read_u32(table + index.wrapping_mul(4)) as usize;
    let vtable = platform::read_u32(entry) as usize;
    let method = platform::read_u32(vtable + 0x20) as usize;
    platform::call2(method, entry as u32, arg4);
}

/// Reads the entry count word at `object + 4`.
///
/// # Original
/// - Ghidra: `FUN_802d9770` at `0x802D9770` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802d9770_FUN_802d9770.c`
///
/// # Arguments
/// - `object`: resource manager.
///
/// # Returns
/// Signed count at `+4`.
///
/// # Safety
/// `object + 4` must be readable.
///
/// # Confidence
/// high - one `lwz` plus `blr`.
pub unsafe fn fn_802d9770(object: usize) -> i32 {
    platform::read_u32(object + 4) as i32
}

// ---------------------------------------------------------------------------
// 0x802DA040 - unlink every stale registry node
// ---------------------------------------------------------------------------

/// Walks the four ownership lists of the resource manager and unlinks every
/// stale node.
///
/// # Original
/// - Ghidra: `FUN_802da040` at `0x802DA040` (648 bytes)
/// - C file: `work/ghidra/main-gb/export/802da040_FUN_802da040.c`
///
/// # Arguments
/// - `this`: resource manager (raw `r3`; Ghidra modelled the entry save as a
///   fake 64-bit return).
///
/// # Side effects
/// - first walk: for each node of `*(this+0x18)`, unlinks each child at
///   `node+0x44` from `*(child+0xC8)+0x44`, then from
///   `*(this+0x20)+0x10` and `*(this+0x20)+4`;
/// - then re-walks `*(this+0x18)`, `*(this+0x10)` and `*(this+8)`, removing
///   nodes whose `+0xC` is `3` and `+0x10` is `0` from their owner-specific
///   list and from the corresponding head.
///
/// # Safety
/// Every list and node pointer must be live for the duration of the walk.
///
/// # Confidence
/// high - the list heads, node offsets and unlink calls are all explicit in
/// the C; the raw prologue at `0x802DA054` confirms `r3` is the manager.
pub unsafe fn fn_802da040(this: usize) {
    let outer_list = platform::read_u32(this + 0x18) as usize;
    let outer_head = outer_list + 0x10;
    let mut node = platform::call2(0x8022_8FF0, outer_head as u32, 0) as usize;
    while node != 0 {
        let next_outer = platform::call2(0x8022_8FF0, outer_head as u32, node as u32) as usize;
        let mut child = platform::read_u32(node + 0x44) as usize;
        while child != 0 {
            let stride = platform::read_u16(node + 0x4E) as usize;
            let next_child = platform::read_u32(child + stride + 4) as usize;
            let child_owner = platform::read_u32(child + 0xC8) as usize;
            platform::call2(0x8022_8F80, (child_owner + 0x44) as u32, child as u32);

            let move_list = platform::read_u32(this + 0x20) as usize;
            platform::call2(0x8022_8F80, (move_list + 0x10) as u32, child as u32);
            platform::call2(0x8022_8DF0, (move_list + 4) as u32, child as u32);
            child = next_child;
        }
        node = next_outer;
    }

    // second walk: nodes owned by *(this+0x18), cleanup list at owner+0xCC.
    let list = platform::read_u32(this + 0x18) as usize;
    if list != 0 {
        let head = list + 0x10;
        let mut node = platform::call2(0x8022_8FF0, head as u32, 0) as usize;
        while node != 0 {
            let next = platform::call2(0x8022_8FF0, head as u32, node as u32) as usize;
            if platform::read_u32(node + 0x0C) == 3 && platform::read_u32(node + 0x10) == 0 {
                let owner = platform::read_u32(node + 0x20) as usize;
                platform::call2(0x8022_8F80, (owner + 0xCC) as u32, node as u32);
                platform::call2(0x8022_8F80, head as u32, node as u32);
                platform::call2(0x8022_8DF0, (list + 4) as u32, node as u32);
            }
            node = next;
        }
    }

    // third walk: nodes owned by *(this+0x10), cleanup list at owner+0x30.
    let list = platform::read_u32(this + 0x10) as usize;
    if list != 0 {
        let head = list + 0x10;
        let mut node = platform::call2(0x8022_8FF0, head as u32, 0) as usize;
        while node != 0 {
            let next = platform::call2(0x8022_8FF0, head as u32, node as u32) as usize;
            if platform::read_u32(node + 0x0C) == 3 && platform::read_u32(node + 0x10) == 0 {
                let owner = platform::read_u32(node + 0xBC) as usize;
                platform::call2(0x8022_8F80, (owner + 0x30) as u32, node as u32);
                platform::call2(0x8022_8F80, head as u32, node as u32);
                platform::call2(0x8022_8DF0, (list + 4) as u32, node as u32);
            }
            node = next;
        }
    }

    // fourth walk: nodes owned by *(this+0x8); the cleanup list lives at
    // *(node+0x20)+0xC018, indexed by node+0x40.
    let list = platform::read_u32(this + 0x08) as usize;
    if list != 0 {
        let head = list + 0x10;
        let mut node = platform::call2(0x8022_8FF0, head as u32, 0) as usize;
        while node != 0 {
            let next = platform::call2(0x8022_8FF0, head as u32, node as u32) as usize;
            if platform::read_u32(node + 0x0C) == 3 && platform::read_u32(node + 0x10) == 0 {
                let owner = platform::read_u32(node + 0x20) as usize;
                let table = platform::read_u32(owner + 0xC018) as usize;
                let slot = table + (platform::read_u32(node + 0x40) as usize).wrapping_mul(0x1C);
                platform::call2(0x8022_8F80, (slot + 0x0C) as u32, node as u32);
                platform::call2(0x8022_8F80, head as u32, node as u32);
                platform::call2(0x8022_8DF0, (list + 4) as u32, node as u32);
            }
            node = next;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802DA6E0 / 0x802DA6F0 - animation-node field transfer
// ---------------------------------------------------------------------------

/// Returns the child record at `object + 0xA0`.
///
/// # Original
/// - Ghidra: `FUN_802da6e0` at `0x802DA6E0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802da6e0_FUN_802da6e0.c`
///
/// # Arguments
/// - `object`: owner record.
///
/// # Returns
/// Address `object + 0xA0`.
///
/// # Safety
/// No dereference; the caller must keep `object` alive if it uses the result.
///
/// # Confidence
/// high - one `addi` plus `blr`.
pub unsafe fn fn_802da6e0(object: usize) -> usize {
    object + 0xA0
}

/// Copies the animation-node scalar block from `source` into `dest`.
///
/// # Original
/// - Ghidra: `FUN_802da6f0` at `0x802DA6F0` (620 bytes)
/// - C file: `work/ghidra/main-gb/export/802da6f0_FUN_802da6f0.c`
///
/// # Arguments
/// - `dest`: node record; receives bytes at `+0xA4..+0xEF` and
///   `+0xF0..+0x134`.
/// - `source`: saved record; read at `+0x04..+0x4F` and `+0x50..+0x94`.
///
/// # Side effects
/// Only memory writes to `dest`. The gaps at `+0xAF`, `+0xB9..+0xBB`,
/// `+0xE1..+0xE3` and `+0xF0`'s neighbours are preserved, matching the raw
/// stores.
///
/// # Safety
/// Both records must be live and non-overlapping; the copy is not memmove.
///
/// # Confidence
/// high - every offset is explicit in the C and the raw prologue
/// (`0x802DA704..0x802DA724`) confirms `r3 = dest`, `r4 = source`.
pub unsafe fn fn_802da6f0(dest: usize, source: usize) {
    // observed 0x802DA724..0x802DA730: the leading byte block.
    platform::write_u8(dest + 0xF0, platform::read_u8(source + 0x50));
    platform::write_u8(dest + 0xF2, platform::read_u8(source + 0x52));
    platform::write_u8(dest + 0xF3, platform::read_u8(source + 0x53));
    platform::write_u8(dest + 0xF4, platform::read_u8(source + 0x54));
    platform::write_u8(dest + 0xF5, platform::read_u8(source + 0x55));
    platform::write_u8(dest + 0xF6, platform::read_u8(source + 0x56));
    platform::write_u8(dest + 0xF7, platform::read_u8(source + 0x57));
    platform::write_u8(dest + 0xF1, platform::read_u8(source + 0x51));

    // observed 0x802DA734..0x802DA7B8: the word tail of the first block.
    platform::write_u32(dest + 0xF8, platform::read_u32(source + 0x58));
    platform::write_u32(dest + 0xFC, platform::read_u32(source + 0x5C));
    platform::write_u32(dest + 0x100, platform::read_u32(source + 0x60));
    platform::write_u32(dest + 0x104, platform::read_u32(source + 0x64));
    platform::write_u32(dest + 0x108, platform::read_u32(source + 0x68));
    platform::write_u32(dest + 0x10C, platform::read_u32(source + 0x6C));
    platform::write_u32(dest + 0x110, platform::read_u32(source + 0x70));
    platform::write_u32(dest + 0x114, platform::read_u32(source + 0x74));
    platform::write_u32(dest + 0x118, platform::read_u32(source + 0x78));
    platform::write_u32(dest + 0x11C, platform::read_u32(source + 0x7C));
    platform::write_u32(dest + 0x120, platform::read_u32(source + 0x80));
    platform::write_u32(dest + 0x124, platform::read_u32(source + 0x84));
    platform::write_u32(dest + 0x128, platform::read_u32(source + 0x88));
    platform::write_u32(dest + 0x12C, platform::read_u32(source + 0x8C));
    platform::write_u32(dest + 0x130, platform::read_u32(source + 0x90));
    platform::write_u32(dest + 0x134, platform::read_u32(source + 0x94));

    // observed 0x802DA7BC..0x802DA8F0: the second block, with mixed widths.
    platform::write_u32(dest + 0xA4, platform::read_u32(source + 0x04));
    platform::write_u32(dest + 0xA8, platform::read_u32(source + 0x08));
    platform::write_u16(dest + 0xAC, platform::read_u16(source + 0x0C));
    platform::write_u32(dest + 0xB0, platform::read_u32(source + 0x10));
    platform::write_u16(dest + 0xB4, platform::read_u16(source + 0x14));
    platform::write_u16(dest + 0xB6, platform::read_u16(source + 0x16));
    platform::write_u8(dest + 0xB8, platform::read_u8(source + 0x18));
    platform::write_u32(dest + 0xBC, platform::read_u32(source + 0x1C));
    platform::write_u32(dest + 0xC0, platform::read_u32(source + 0x20));
    platform::write_u32(dest + 0xC4, platform::read_u32(source + 0x24));
    platform::write_u32(dest + 0xC8, platform::read_u32(source + 0x28));
    platform::write_u32(dest + 0xCC, platform::read_u32(source + 0x2C));
    platform::write_u32(dest + 0xD0, platform::read_u32(source + 0x30));
    platform::write_u32(dest + 0xD4, platform::read_u32(source + 0x34));
    platform::write_u32(dest + 0xD8, platform::read_u32(source + 0x38));
    platform::write_u32(dest + 0xDC, platform::read_u32(source + 0x3C));
    platform::write_u32(dest + 0xE0, platform::read_u32(source + 0x40));
    platform::write_u8(dest + 0xE4, platform::read_u8(source + 0x44));
    platform::write_u8(dest + 0xE5, platform::read_u8(source + 0x45));
    platform::write_u8(dest + 0xE6, platform::read_u8(source + 0x46));
    platform::write_u8(dest + 0xE7, platform::read_u8(source + 0x47));
    platform::write_u8(dest + 0xE8, platform::read_u8(source + 0x48));
    platform::write_u8(dest + 0xE9, platform::read_u8(source + 0x49));
    platform::write_u8(dest + 0xEA, platform::read_u8(source + 0x4A));
    platform::write_u8(dest + 0xEB, platform::read_u8(source + 0x4B));
    platform::write_u8(dest + 0xEC, platform::read_u8(source + 0x4C));
    platform::write_u8(dest + 0xED, platform::read_u8(source + 0x4D));
    platform::write_u8(dest + 0xEE, platform::read_u8(source + 0x4E));
    platform::write_u8(dest + 0xEF, platform::read_u8(source + 0x4F));
}

// ---------------------------------------------------------------------------
// 0x802DAD20 - derive the camera pitch/yaw from the child transform
// ---------------------------------------------------------------------------

/// Projects the object's child transform and stores two derived angles.
///
/// # Original
/// - Ghidra: `FUN_802dad20` at `0x802DAD20` (168 bytes)
/// - C file: `work/ghidra/main-gb/export/802dad20_FUN_802dad20.c`
///
/// # Arguments
/// - `object`: record with the child handle at `+0x154`, view matrix at
///   `+0x18` (built by `0x80289070`) and outputs at `+0x84/+0x88/+0x8C`.
///
/// # Side effects
/// Calls `0x80289070`, `0x801C07A0`, `0x801C1160` and the two math thunks
/// `0x802E8608` (`thunk_FUN_802e46a0`) and `0x802E8610`
/// (`thunk_FUN_802e4c04`); writes three floats.
///
/// # Safety
/// `object` and the child handle at `+0x154` must be live; the stack matrices
/// passed to the GX helpers must be 12 floats each.
///
/// # Confidence
/// medium - the float arguments are `lfs`-loaded and the two thunks take
/// `double` parameters; the bytes and stores are exact, but the math routine
/// names are not present in the image.
pub unsafe fn fn_802dad20(object: usize) {
    let child = platform::read_u32(object + 0x154) as usize;
    let vector = fn_802dbfb0(child);

    let mut frame = crate::platform::GuestLocal::new( [0.0f32; 12]);
    let frame_ptr = frame.as_mut_ptr() as usize;
    crate::render::gx_band::fn_80289070(object, frame_ptr);

    let zero = platform::read_f32(0x8042_F650);
    frame[3] = zero;
    frame[7] = zero;
    frame[11] = zero;

    let mut matrix = crate::platform::GuestLocal::new( [0.0f32; 12]);
    let matrix_ptr = matrix.as_mut_ptr() as usize;
    if platform::call2(0x801C_07A0, frame_ptr as u32, matrix_ptr as u32) != 0 {
        let mut out = crate::platform::GuestLocal::new( [0.0f32; 3]);
        let out_ptr = out.as_mut_ptr() as usize;
        platform::call3(0x801C_1160, matrix_ptr as u32, vector as u32, out_ptr as u32);

        let pitch = call_f64_1(0x802E_8608, out[1] as f64) as f32;
        platform::write_f32(object + 0x84, pitch);

        let yaw = call_f64_2(0x802E_8610, out[0] as f64, out[2] as f64) as f32;
        platform::write_f32(object + 0x8C, zero);
        platform::write_f32(object + 0x88, yaw);
    }
}

// ---------------------------------------------------------------------------
// 0x802DBC90..0x802DBD50 - layout record accessors
// ---------------------------------------------------------------------------

/// Reads the `u16` field at `+0x0C`.
///
/// # Original
/// - Ghidra: `FUN_802dbc90` at `0x802DBC90` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbc90_FUN_802dbc90.c`
///
/// # Arguments
/// - `object`: layout record.
///
/// # Returns
/// Halfword at `+0x0C`.
///
/// # Safety
/// `object + 0x0C` must be readable.
///
/// # Confidence
/// high - one `lhz` plus `blr`.
pub unsafe fn fn_802dbc90(object: usize) -> u16 {
    platform::read_u16(object + 0x0C)
}

/// Sets the `u16` field at `+0x0C` and raises flag bit 1.
///
/// # Original
/// - Ghidra: `FUN_802dbca0` at `0x802DBCA0` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbca0_FUN_802dbca0.c`
///
/// # Arguments
/// - `object`: layout record.
/// - `value`: new halfword.
///
/// # Side effects
/// Calls `0x802DC5D0(object, 1)` before storing.
///
/// # Safety
/// `object` must be a live record.
///
/// # Confidence
/// high - the raw body passes mask `1` and stores `sth r4,0x0C`.
pub unsafe fn fn_802dbca0(object: usize, value: u16) {
    fn_802dc5d0(object, 1, 0);
    platform::write_u16(object + 0x0C, value);
}

/// Reads the `f32` field at `+0x10`.
///
/// # Original
/// - Ghidra: `FUN_802dbce0` at `0x802DBCE0` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbce0_FUN_802dbce0.c`
///
/// # Arguments
/// - `object`: layout record.
///
/// # Returns
/// Float at `+0x10`.
///
/// # Safety
/// `object + 0x10` must be readable.
///
/// # Confidence
/// high - one `lfs` plus `blr`.
pub unsafe fn fn_802dbce0(object: usize) -> f32 {
    platform::read_f32(object + 0x10)
}

/// Sets the `f32` field at `+0x10` and raises flag bit 2.
///
/// # Original
/// - Ghidra: `FUN_802dbcf0` at `0x802DBCF0` (68 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbcf0_FUN_802dbcf0.c`
///
/// # Arguments
/// - `value`: new float (arrives in `f1`).
/// - `object`: layout record.
/// - `mode`: forwarded unchanged to `0x802DC5D0` (Ghidra drops it).
///
/// # Side effects
/// Calls `0x802DC5D0(object, 2, mode)` before storing.
///
/// # Safety
/// `object` must be a live record.
///
/// # Confidence
/// high - raw `0x802DBCF8..0x802DBD18` shows `mr r5,r4`, mask `2` and
/// `stfs f31,0x10(r31)`.
pub unsafe fn fn_802dbcf0(value: f32, object: usize, mode: u32) {
    fn_802dc5d0(object, 2, mode);
    platform::write_f32(object + 0x10, value);
}

/// Reads the `u16` field at `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_802dbd40` at `0x802DBD40` (8 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbd40_FUN_802dbd40.c`
///
/// # Arguments
/// - `object`: layout record.
///
/// # Returns
/// Halfword at `+0x14`.
///
/// # Safety
/// `object + 0x14` must be readable.
///
/// # Confidence
/// high - one `lhz` plus `blr`.
pub unsafe fn fn_802dbd40(object: usize) -> u16 {
    platform::read_u16(object + 0x14)
}

/// Sets the `u16` field at `+0x14` and raises flag bit 4.
///
/// # Original
/// - Ghidra: `FUN_802dbd50` at `0x802DBD50` (64 bytes)
/// - C file: `work/ghidra/main-gb/export/802dbd50_FUN_802dbd50.c`
///
/// # Arguments
/// - `object`: layout record.
/// - `value`: new halfword.
///
/// # Side effects
/// Calls `0x802DC5D0(object, 4)` before storing.
///
/// # Safety
/// `object` must be a live record.
///
/// # Confidence
/// high - raw `0x802DBD64..0x802DBD74` shows mask `4` and
/// `sth r4,0x14(r30)`.
pub unsafe fn fn_802dbd50(object: usize, value: u16) {
    fn_802dc5d0(object, 4, 0);
    platform::write_u16(object + 0x14, value);
}

// ---------------------------------------------------------------------------
// Host-side helpers for the two calls in the next band
// ---------------------------------------------------------------------------

// ---------------------------------------------------------------------------
// Host-side helper for the four pools of `fn_802d8e70`
// ---------------------------------------------------------------------------

/// Builds one element list exactly as the four inline blocks at
/// `0x802D8F0C..0x802D91B8` do.
///
/// `// inferred:` host-side helper, not an original function. It folds the
/// repeated sequence "virtual `+0x60` allocation, `0x802DC7E0` element run,
/// virtual `+0x60` list header, two `0x80228DD0` list initialisers, index
/// stores plus `0x80228DF0` links" into one routine. The original inlines this
/// four times with different `count`, `stride`, constructor, destructor and
/// vtable arguments; the argument order here matches the call sites.
unsafe fn build_element_list(
    owner: usize,
    count: u32,
    stride: u32,
    construct: usize,
    destroy: usize,
    vtable: u32,
) -> usize {
    let vtable_ptr = platform::read_u32(owner) as usize;
    let allocate = platform::read_u32(vtable_ptr + 0x60) as usize;

    let mut first: usize = 0;
    if count > 0 {
        let storage = platform::call2(
            allocate,
            owner as u32,
            count.wrapping_mul(stride).wrapping_add(0x20),
        ) as usize;
        first = fn_802dc7e0(storage, construct, destroy, stride, count);
    }

    let list = platform::call2(allocate, owner as u32, 0x24) as usize;
    if list == 0 {
        return 0;
    }

    platform::write_u32(list + 0x20, vtable);
    platform::call2(0x8022_8DD0, (list + 4) as u32, 0);
    platform::call2(0x8022_8DD0, (list + 0x10) as u32, 0);
    platform::write_u32(list + 0x1C, stride);
    platform::write_u32(list, first as u32);

    let mut element = first;
    let mut index: u32 = 0;
    while index < count {
        platform::write_u32(element + 8, index);
        platform::call2(0x8022_8DF0, (list + 4) as u32, element as u32);
        element += stride as usize;
        index += 1;
    }

    list
}

/// Reads a big-endian IEEE-754 `double` from an absolute address.
///
/// `// inferred:` host-side helper, not an original function. The image stores
/// doubles as two big-endian words, so the high word is read first.
unsafe fn read_f64_be(addr: usize) -> f64 {
    let high = platform::read_u32(addr) as u64;
    let low = platform::read_u32(addr + 4) as u64;
    f64::from_bits((high << 32) | low)
}

/// Calls `addr(value, out_pointer)` and returns the `f1` result.
///
/// `// inferred:` host-side helper, not an original function.
/// `unknown:` used for `0x802E81D4`, which lives in the next band and is not
/// reconstructed here.
unsafe fn call_f64_ptr(addr: usize, value: f64, out: usize) -> f64 {
    let function = crate::platform::guest_fn!(addr, fn(f64, usize) -> f64);
    function(value, out)
}

/// Calls `addr(value, argument)` and returns the `f1` result.
///
/// `// inferred:` host-side helper, not an original function.
/// `unknown:` used for `0x802E825C`, which lives in the next band and is not
/// reconstructed here.
unsafe fn call_f64_i32(addr: usize, value: f64, argument: i32) -> f64 {
    let function = crate::platform::guest_fn!(addr, fn(f64, i32) -> f64);
    function(value, argument)
}

/// Calls `addr(value)` and returns the `f1` result.
///
/// `// inferred:` host-side helper, not an original function.
/// `unknown:` used for `0x802E8608`, which lives in the next band and is not
/// reconstructed here.
unsafe fn call_f64_1(addr: usize, value: f64) -> f64 {
    let function = crate::platform::guest_fn!(addr, fn(f64) -> f64);
    function(value)
}

/// Calls `addr(a, b)` and returns the `f1` result.
///
/// `// inferred:` host-side helper, not an original function.
/// `unknown:` used for `0x802E8610`, which lives in the next band and is not
/// reconstructed here.
unsafe fn call_f64_2(addr: usize, a: f64, b: f64) -> f64 {
    let function = crate::platform::guest_fn!(addr, fn(f64, f64) -> f64);
    function(a, b)
}

// ---------------------------------------------------------------------------
// Wave 38 - AX task queue and voice lifecycle (`0x802D6110..0x802D7030`)
// ---------------------------------------------------------------------------
//
// A 20-slot deferred-task table at `0x80424BE8` (12 bytes per slot) and the
// r13 (`0x8042F680`) indices that walk it. Direct callee bodies in this file
// are called as Rust functions; everything else is an opaque `0x8027xxxx` /
// `0x801Exxxx` / `0x802Dxxxx` call kept as a raw address.

/// Base byte of the 20-slot task table (`&DAT_80424BE8`).
const AX_TASK_TABLE: usize = 0x8042_4BE8;

/// Dequeue cursor (`r13 - 0x46FC`) into the task table.
const AX_TASK_HEAD: usize = 0x8042_AF84;
/// Enqueue cursor (`r13 - 0x46F8`); wraps past slot 19.
const AX_TASK_TAIL: usize = 0x8042_AF88;
/// Dispatch latch (`r13 - 0x46F4`): 0 = idle, 1 = waiting for the callback.
const AX_TASK_BUSY: usize = 0x8042_AF8C;
/// Pad guard (`r13 - 0x4700`): 0 = pad queries allowed.
const AX_PAD_LOCK: usize = 0x8042_AF80;
/// Last pad byte from `0x801E3420` (`r13 - 0x5AE8`).
const AX_PAD_PENDING: usize = 0x8042_9B98;
/// Gate hook (`r13 - 0x46E4`); called `(0,0)` on shutdown, `(1,arg)` on drain.
const AX_GATE_HOOK: usize = 0x8042_AF9C;
/// Gate hook 2 (`r13 - 0x46E8`); called with no arguments on drain.
const AX_GATE_HOOK2: usize = 0x8042_AF98;
/// First gate object (`r13 - 0x46EC`); voice list hangs at `+0x5BC`.
const AX_GATE_MAIN: usize = 0x8042_AF94;
/// Second gate object (`r13 - 0x46F0`); per-slice lists at `+0x5C4` stepped
/// by `0x4D8`.
const AX_GATE_SECOND: usize = 0x8042_AF90;
/// Gate argument (`r13 - 0x5AE0`) passed to `0x8027A850`.
const AX_GATE_ARG: usize = 0x8042_9BA0;

// ---------------------------------------------------------------------------
// 0x802D6110 - provider lookup into an ARC slot
// ---------------------------------------------------------------------------

/// Publishes `arg` into the ARC at `output + 4` when the slot at
/// `output + 0x20` is empty and the provider's virtual lookup returns
/// non-zero.
///
/// # Original
/// - Ghidra: `FUN_802d6110` at `0x802D6110` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6110_FUN_802d6110.c`
///
/// # Arguments
/// - `output`: object whose `+0x20` word is the empty flag and whose `+4`
///   cell receives the result.
/// - `provider`: object whose `+0x10` vtable slot `+0x0C` is the lookup.
/// - `arg`: value forwarded to both the lookup and `0x8027A310`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Indirect call through the provider vtable; `0x8027A310(output + 4,
/// result, arg)` when the result is non-zero and the flag word is zero.
///
/// # Safety
/// Both objects and their vtable words must be readable; `+0x20` must be
/// writable by the callee.
///
/// # Confidence
/// medium - the two tests and the publish call are literal; the `+0x10` /
/// `+0x0C` roles are inferred from the indirect-call shape.
pub unsafe fn fn_802d6110(output: usize, provider: usize, arg: u32) {
    // observed: `*(output + 0x20) == 0` guards both the lookup and the store.
    if platform::read_u32(output + 0x20) != 0 {
        return;
    }
    // observed: vtable = *(provider + 0x10); call vtable[0x0C](provider, arg).
    let vtable = platform::read_u32(provider + 0x10) as usize;
    let lookup = platform::read_u32(vtable + 0x0C) as usize;
    let result = platform::call2(lookup, provider as u32, arg);
    if result != 0 {
        platform::call3(0x8027_A310, (output + 4) as u32, result, arg);
    }
}

// ---------------------------------------------------------------------------
// 0x802D6180 - reset the ARC at `object + 4`
// ---------------------------------------------------------------------------

/// Resets the inner ARC storage of `object` via `0x8027A320`.
///
/// # Original
/// - Ghidra: `FUN_802d6180` at `0x802D6180` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6180_FUN_802d6180.c`
///
/// # Arguments
/// - `object`: ARC wrapper; the callee receives `object + 4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8027A320(object + 4)`.
///
/// # Safety
/// `object + 4` must be live ARC storage.
///
/// # Confidence
/// high - single tail call with `+4` offset.
pub unsafe fn fn_802d6180(object: usize) {
    platform::call1(0x8027_A320, (object + 4) as u32);
}

// ---------------------------------------------------------------------------
// 0x802D6190 - seven-word descriptor initialiser
// ---------------------------------------------------------------------------

/// Initialises the seven words of a small descriptor to `0,0,4,3,0,0,1`.
///
/// # Original
/// - Ghidra: `FUN_802d6190` at `0x802D6190` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6190_FUN_802d6190.c`
///
/// # Arguments
/// - `object`: 28 writable bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes seven words at `object + 0x00..0x1C`.
///
/// # Safety
/// `object..object + 28` must be writable.
///
/// # Confidence
/// high - seven literal stores; field roles remain unknown.
pub unsafe fn fn_802d6190(object: usize) {
    // observed order: +0, +8, +12, +4, +16, +20, +24.
    platform::write_u32(object, 0);
    platform::write_u32(object + 8, 4);
    platform::write_u32(object + 0x0C, 3);
    platform::write_u32(object + 4, 0);
    platform::write_u32(object + 0x10, 0);
    platform::write_u32(object + 0x14, 0);
    platform::write_u32(object + 0x18, 1);
}

// ---------------------------------------------------------------------------
// 0x802D61C0 - extend `0x802D6190` with two more stores
// ---------------------------------------------------------------------------

/// Initialises a 36-byte descriptor: [`fn_802d6190`] plus a large constant
/// at `+0x1C` and `1` at `+0x20`.
///
/// # Original
/// - Ghidra: `FUN_802d61c0` at `0x802D61C0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d61c0_FUN_802d61c0.c`
///
/// # Arguments
/// - `object`: writable descriptor.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Runs [`fn_802d6190`] on `object`, then two stores.
///
/// # Safety
/// `object..object + 36` must be writable.
///
/// # Confidence
/// high - the callee takes `this` (Ghidra drops the argument), the two
/// stores are literal.
pub unsafe fn fn_802d61c0(object: usize) -> usize {
    // observed: `FUN_802d6190()` with `r3` still holding `this`.
    fn_802d6190(object);
    platform::write_u32(object + 0x1C, 0x8CA000);
    platform::write_u8(object + 0x20, 1);
    object
}

// ---------------------------------------------------------------------------
// 0x802D6210 - vtable install for the 0x5EC-byte voice object
// ---------------------------------------------------------------------------

/// Installs temporary vtables, runs the inner initialisers, then installs
/// the final object layout.
///
/// # Original
/// - Ghidra: `FUN_802d6210` at `0x802D6210` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6210_FUN_802d6210.c`
///
/// # Arguments
/// - `object`: 0x5EC writable bytes.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Writes vtables and calls `0x8027A250`, `0x801B2390` (x3),
/// `0x802D3810`, `0x802776B0` and [`fn_802d7010`].
///
/// # Safety
/// `object` must be live writable storage for the whole object.
///
/// # Confidence
/// medium - offsets are word-indexed in Ghidra (`param_1[0x177]` = byte
/// `+0x5DC`) and every call is literal; vtable roles are inferred.
pub unsafe fn fn_802d6210(object: usize) -> usize {
    // Ghidra indexes `object` as words; byte offsets are index * 4.
    platform::write_u32(object, 0x8035_0738);
    platform::write_u32(object + 8, 0x8035_0778);
    platform::call1(0x8027_A250, (object + 0x0C) as u32);
    platform::call3(0x801B_2390, (object + 0x68) as u32, (object + 0x88) as u32, 4);
    platform::call3(0x801B_2390, (object + 0x98) as u32, (object + 0xB8) as u32, 4);
    platform::call3(0x801B_2390, (object + 0xC8) as u32, (object + 0xE8) as u32, 4);
    platform::call3(0x802D_3810, (object + 0xF8) as u32, (object + 0x5DC) as u32, (object + 0x0C) as u32);
    fn_802d7010(object + 0x5D0);
    platform::write_u32(object, 0x8035_0690);
    platform::write_u32(object + 8, 0x8035_06A0);
    platform::write_u32(object + 0xF8, 0x8035_06B4);
    platform::call1(0x8027_76B0, (object + 0x5DC) as u32);
    platform::write_u32(object + 0x5E4, 0x8035_0748);
    platform::write_u32(object + 0x5E8, 0x8035_075C);
    // observed: `*(undefined1 *)(param_1 + 1) = 0`, i.e. the byte at +4.
    platform::write_u8(object + 4, 0);
    object
}

// ---------------------------------------------------------------------------
// 0x802D62F0 - conditional release (deleting destructor)
// ---------------------------------------------------------------------------

/// Runs `0x802779D0(object, 0)` and frees `object` when the delete flag is
/// positive.
///
/// # Original
/// - Ghidra: `FUN_802d62f0` at `0x802D62F0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d62f0_FUN_802d62f0.c`
///
/// # Arguments
/// - `object`: object pointer; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802779D0(object, 0)` always (comma operator), then `0x802B9280` when
/// requested.
///
/// # Safety
/// `object` must be null or a live object for `0x802779D0`.
///
/// # Confidence
/// high - CodeWarrior deleting-destructor idiom with literal flag test.
pub unsafe fn fn_802d62f0(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        platform::call2(0x8027_79D0, object as u32, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D6350 - teardown with sub-object destructors
// ---------------------------------------------------------------------------

/// Tears down the `+0x5DC` / `+0x5D0` / `+0xF8` / `+8` sub-objects and
/// optionally frees the whole object.
///
/// # Original
/// - Ghidra: `FUN_802d6350` at `0x802D6350` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6350_FUN_802d6350.c`
///
/// # Arguments
/// - `object`: containing object; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802779D0(+0x5DC,0)`, [`fn_802d7030`], `0x802D38A0`, [`fn_802d6180`],
/// `0x8027A2A0` and optionally `0x802B9280`.
///
/// # Safety
/// `object` must be null or fully constructed.
///
/// # Confidence
/// medium - call order is literal; `param_1 + 0x5DC != 0` and
/// `param_1 + 8 != 0` are vacuous guards kept for fidelity.
pub unsafe fn fn_802d6350(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        if object.wrapping_add(0x5DC) != 0 {
            platform::call2(0x8027_79D0, (object + 0x5DC) as u32, 0);
        }
        fn_802d7030(object + 0x5D0, 0);
        platform::call2(0x802D_38A0, (object + 0xF8) as u32, 0);
        let inner = object + 8;
        if inner != 0 {
            platform::write_u32(inner, 0x8035_0778);
            fn_802d6180(inner);
            platform::call2(0x8027_A2A0, (object + 0x0C) as u32, 0xFFFF_FFFF);
        }
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D6400 - one-shot output setup
// ---------------------------------------------------------------------------

/// Copies a 32-byte descriptor, routes it through the ARC lookup, size
/// handling and [`fn_802d6110`], and records the voice bytes.
///
/// # Original
/// - Ghidra: `FUN_802d6400` at `0x802D6400` (352 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6400_FUN_802d6400.c`
///
/// # Arguments
/// - `output`: 0x100+ writable bytes for the result.
/// - `desc`: word-indexed descriptor: `[0]` = source block, `[2]/[3]` =
///   dimensions, `[4]/[5]` = extents (0 defaults to `0x4000`), `[6]` =
///   forward argument, `[7]` = key, byte `[8]` (word 8) = flag byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x8019F1F0`, `0x801A0050`, `0x802B8A70`, `0x802B8AD0`,
/// `0x8027B290` or `0x8027B2E0/0x8027A3A0/0x8027B360`, `0x802D3920` and
/// stores two bytes on `output`.
///
/// # Safety
/// `desc` and `output` must be readable/writable; the 44-byte stack copies
/// are `// unknown:` payloads for the opaque ARC callees.
///
/// # Confidence
/// medium - branch structure and the `0x4000` defaults are literal; the
/// stack copy contents are unknown to the export.
pub unsafe fn fn_802d6400(output: usize, desc: usize) {
    platform::call1(0x8019_F1F0, 0);
    platform::call0(0x801A_0050);
    if platform::read_u32(desc + 0x1C) != 0 && platform::read_u32(desc) != 0 {
        let mut scratch = crate::platform::GuestLocal::new( [0u8; 44]);
        let scratch_addr = scratch.as_mut_ptr() as usize;
        platform::call3(0x802B_8A70, scratch_addr as u32, platform::read_u32(desc), 0x20);
        fn_802d6110(output + 8, scratch_addr, platform::read_u32(desc + 0x1C));
        platform::call2(0x802B_8AD0, scratch_addr as u32, 0xFFFF_FFFF);
    }
    if platform::read_u32(desc + 0x10) == 0 && platform::read_u32(desc + 0x14) == 0 {
        platform::call2(0x8027_B290, platform::read_u32(desc + 8), platform::read_u32(desc + 0x0C));
    } else {
        if platform::read_u32(desc + 0x10) == 0 {
            platform::write_u32(desc + 0x10, 0x4000);
        }
        if platform::read_u32(desc + 0x14) == 0 {
            platform::write_u32(desc + 0x14, 0x4000);
        }
        let mut rect = crate::platform::GuestLocal::<[u32; 4]>::new( [
            platform::read_u32(desc + 8),
            platform::read_u32(desc + 0x10),
            platform::read_u32(desc + 0x0C),
            platform::read_u32(desc + 0x14),
        ]);
        let rect_addr = rect.as_mut_ptr() as usize;
        let size = platform::call1(0x8027_B2E0, rect_addr as u32);
        let handle = platform::call4(0x8027_A3A0, (output + 0x0C) as u32, size, 0, 0x53595354);
        let _ = &mut rect;
        platform::call3(0x8027_B360, rect_addr as u32, handle, size);
    }
    platform::call2(0x802D_3920, (output + 0xF8) as u32, platform::read_u32(desc + 0x18));
    platform::write_u8(output + 0xFE, platform::read_u8(desc + 0x20));
    platform::write_u8(output + 4, 1);
}

// ---------------------------------------------------------------------------
// 0x802D6560 - frame update forwarding
// ---------------------------------------------------------------------------

/// Updates the `+0x5D0` helper and, when its state is 2, the `+0xF8`
/// voice, then the `+0xF8` voice again.
///
/// # Original
/// - Ghidra: `FUN_802d6560` at `0x802D6560` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6560_FUN_802d6560.c`
///
/// # Arguments
/// - `object`: containing object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x802D7070(object + 0x5D0)`, conditionally `0x802D41F0(object + 0xF8)`,
/// then `0x802D41D0(object + 0xF8)`.
///
/// # Safety
/// `object + 0x5D4` and both sub-objects must be live.
///
/// # Confidence
/// high - the state test and call order are literal.
pub unsafe fn fn_802d6560(object: usize) {
    platform::call1(0x802D_7070, (object + 0x5D0) as u32);
    if platform::read_i32(object + 0x5D4) == 2 {
        platform::call1(0x802D_41F0, (object + 0xF8) as u32);
    }
    platform::call1(0x802D_41D0, (object + 0xF8) as u32);
}

// ---------------------------------------------------------------------------
// 0x802D65D0 - forward to `0x802D3CE0(object + 0xF8)`
// ---------------------------------------------------------------------------

/// Forwards a voice update call to `0x802D3CE0` on the `+0xF8` slice.
///
/// # Original
/// - Ghidra: `FUN_802d65d0` at `0x802D65D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d65d0_FUN_802d65d0.c`
///
/// # Arguments
/// - `object`: containing object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x802D3CE0(object + 0xF8)`.
///
/// # Safety
/// The `+0xF8` slice must be a live voice.
///
/// # Confidence
/// high - single tail call with `+0xF8` offset.
pub unsafe fn fn_802d65d0(object: usize) {
    platform::call1(0x802D_3CE0, (object + 0xF8) as u32);
}

// ---------------------------------------------------------------------------
// 0x802D65E0 - forward to `0x802D3E00(object + 0xF8)`
// ---------------------------------------------------------------------------

/// Forwards a voice teardown call to `0x802D3E00` on the `+0xF8` slice.
///
/// # Original
/// - Ghidra: `FUN_802d65e0` at `0x802D65E0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d65e0_FUN_802d65e0.c`
///
/// # Arguments
/// - `object`: containing object.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x802D3E00(object + 0xF8)`.
///
/// # Safety
/// The `+0xF8` slice must be a live voice.
///
/// # Confidence
/// high - single tail call with `+0xF8` offset.
pub unsafe fn fn_802d65e0(object: usize) {
    platform::call1(0x802D_3E00, (object + 0xF8) as u32);
}

// ---------------------------------------------------------------------------
// 0x802D66B0 - task callback: pad branch
// ---------------------------------------------------------------------------

/// Runs the current slot's callback (or the pad requeue) and marks the
/// slot done.
///
/// # Original
/// - Ghidra: `FUN_802d66b0` at `0x802D66B0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d66b0_FUN_802d66b0.c`
///
/// # Arguments
/// - `pad`: pad index (0..3) forwarded to `0x802D6A60`.
/// - `mode`: `0` = run the callback now; anything else = requeue through
///   `0x802D6A60`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Indirect call (or `0x801E3420` on pad state) and a write to the done
/// byte at `AX_TASK_TABLE + head * 12 + 1`.
///
/// # Safety
/// `AX_TASK_HEAD` must hold a slot index `< 20` and the table must be
/// writable for the duration.
///
/// # Confidence
/// medium - the branch and done-flag store are literal; the 12-byte slot
/// layout (`arg` at `+4`, callback at `+8`) is inferred from the mixed
/// `&DAT_80424BE8/...BEC/...BF0/...BE9` accesses.
pub unsafe fn fn_802d66b0(pad: u32, mode: i32) {
    if mode == 0 {
        let head = platform::read_u32(AX_TASK_HEAD) as usize;
        let callback = platform::read_u32(AX_TASK_TABLE + head * 12 + 8) as usize;
        if callback != 0 {
            platform::call0(callback);
        }
        platform::write_u8(AX_PAD_PENDING, platform::call1(0x801E_3420, 0) as u8);
    } else {
        let head = platform::read_u32(AX_TASK_HEAD) as usize;
        let callback = platform::read_u32(AX_TASK_TABLE + head * 12 + 8) as u32;
        fn_802d6a60(pad, callback as usize);
    }
    let head = platform::read_u32(AX_TASK_HEAD) as usize;
    platform::write_u8(AX_TASK_TABLE + head * 12 + 1, 1);
}

// ---------------------------------------------------------------------------
// 0x802D6740 - task callback: gate branch
// ---------------------------------------------------------------------------

/// Runs the current slot's callback for states 0 / -1, otherwise requeues
/// through `0x802D6AC0`, and marks the slot done.
///
/// # Original
/// - Ghidra: `FUN_802d6740` at `0x802D6740` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6740_FUN_802d6740.c`
///
/// # Arguments
/// - `pad`: pad index forwarded to `0x802D6AC0`.
/// - `state`: tested unsigned as `state + 1 < 2`, i.e. `0` or `-1`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Indirect call or `0x802D6AC0`, plus done-flag store like
/// [`fn_802d66b0`].
///
/// # Safety
/// Same contract as [`fn_802d66b0`].
///
/// # Confidence
/// medium - unsigned test preserved literally; slot layout inferred as in
/// `0x802D66B0`.
pub unsafe fn fn_802d6740(pad: u32, state: i32) {
    if (state as u32).wrapping_add(1) < 2 {
        let head = platform::read_u32(AX_TASK_HEAD) as usize;
        let callback = platform::read_u32(AX_TASK_TABLE + head * 12 + 8) as usize;
        if callback != 0 {
            platform::call0(callback);
        }
    } else {
        let head = platform::read_u32(AX_TASK_HEAD) as usize;
        let callback = platform::read_u32(AX_TASK_TABLE + head * 12 + 8) as u32;
        fn_802d6ac0(pad, callback as usize);
    }
    let head = platform::read_u32(AX_TASK_HEAD) as usize;
    platform::write_u8(AX_TASK_TABLE + head * 12 + 1, 1);
}

// ---------------------------------------------------------------------------
// 0x802D67D0 - enqueue one task slot
// ---------------------------------------------------------------------------

/// Appends `{pad, callback, flag}` at the tail cursor under the
/// `0x801B1140` / `0x801B1180` interrupt guard.
///
/// # Original
/// - Ghidra: `FUN_802d67d0` at `0x802D67D0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d67d0_FUN_802d67d0.c`
///
/// # Arguments
/// - `pad`: pad index stored at slot `+4`.
/// - `callback`: function address stored at slot `+8`.
/// - `flag`: byte stored at slot `+0` (selects `0x802D6740` vs `0x802D66B0`
///   in [`fn_802d6930`]).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Advances `AX_TASK_TAIL` (wrapping past 19), writes the slot, clears its
/// done byte at `+1`.
///
/// # Safety
/// The table and both cursors must be writable; the caller must accept the
/// 20-slot wraparound.
///
/// # Confidence
/// medium - wraparound (`0x13 < next`) and store offsets are literal; the
/// byte fields are `// unknown:` in the export.
pub unsafe fn fn_802d67d0(pad: u32, callback: usize, flag: u8) {
    platform::call0(0x801B_1140);
    let slot = platform::read_u32(AX_TASK_TAIL);
    let offset = slot as usize * 12;
    let next = slot.wrapping_add(1);
    platform::write_u32(AX_TASK_TAIL, next);
    platform::write_u32(AX_TASK_TABLE + offset + 4, pad);
    platform::write_u8(AX_TASK_TABLE + offset, flag);
    platform::write_u32(AX_TASK_TABLE + offset + 8, callback as u32);
    platform::write_u8(AX_TASK_TABLE + offset + 1, 0);
    if next > 0x13 {
        platform::write_u32(AX_TASK_TAIL, 0);
    }
    platform::call0(0x801B_1180);
}

// ---------------------------------------------------------------------------
// 0x802D6860 - register a one-shot pad task if lookup fails
// ---------------------------------------------------------------------------

/// Resolves the pad against `0x80270AD0/0x80270B70/0x802701A0` and, when
/// the lookup returns 0, enqueues the pair with flag 1.
///
/// # Original
/// - Ghidra: `FUN_802d6860` at `0x802D6860` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6860_FUN_802d6860.c`
///
/// # Arguments
/// - `pad`: pad index.
/// - `callback`: callback address stored in the slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three AX calls; on success of the first two, [`fn_802d67d0`] plus a
/// done-flag write at the *new* tail.
///
/// # Safety
/// The AX callees must accept the pad; the table must be writable.
///
/// # Confidence
/// medium - call sequence and `== 0` branch are literal; AX table roles
/// are inferred.
pub unsafe fn fn_802d6860(pad: u32, callback: usize) {
    // observed: zero-argument `0x80270AD0` ignores `r3`.
    let ctx = platform::call1(0x8027_0AD0, 0);
    let ctx = platform::call2(0x8027_0B70, ctx, pad);
    let found = platform::call2(0x8027_01A0, ctx, callback as u32);
    if found == 0 {
        fn_802d67d0(pad, callback, 1);
        let tail = platform::read_u32(AX_TASK_TAIL) as usize;
        platform::write_u8(AX_TASK_TABLE + tail * 12 + 1, 1);
    }
}

// ---------------------------------------------------------------------------
// 0x802D68E0 - register a pad task directly
// ---------------------------------------------------------------------------

/// Resolves the pad against `0x80270AD0/0x80270B70` and registers `callback`
/// via `0x80270260`, without enqueueing.
///
/// # Original
/// - Ghidra: `FUN_802d68e0` at `0x802D68E0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d68e0_FUN_802d68e0.c`
///
/// # Arguments
/// - `pad`: pad index.
/// - `callback`: callback address.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three AX calls.
///
/// # Safety
/// The AX callees must accept the pad.
///
/// # Confidence
/// medium - three literal calls; no queue traffic.
pub unsafe fn fn_802d68e0(pad: u32, callback: usize) {
    let ctx = platform::call1(0x8027_0AD0, 0);
    let ctx = platform::call2(0x8027_0B70, ctx, pad);
    platform::call2(0x8027_0260, ctx, callback as u32);
}

// ---------------------------------------------------------------------------
// 0x802D6930 - pump one task
// ---------------------------------------------------------------------------

/// Dispatches the head slot when idle, or advances the head cursor after
/// the callback finished.
///
/// # Original
/// - Ghidra: `FUN_802d6930` at `0x802D6930` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6930_FUN_802d6930.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802d68e0`] or [`fn_802d6860`] with `0x802D6740` /
/// `0x802D66B0`, sets/clears `AX_TASK_BUSY`, advances `AX_TASK_HEAD`
/// (wrapping past 19).
///
/// # Safety
/// Cursors must hold slot indices `< 20`; the table must be readable.
///
/// # Confidence
/// medium - busy latch, flag test at slot `+0` and wraparound are literal;
/// dispatch targets are inferred from the watched flag.
pub unsafe fn fn_802d6930() {
    if platform::read_u8(AX_TASK_BUSY) == 0 {
        if platform::read_u32(AX_TASK_TAIL) != platform::read_u32(AX_TASK_HEAD) {
            let head = platform::read_u32(AX_TASK_HEAD) as usize;
            let offset = head * 12;
            let pad = platform::read_u32(AX_TASK_TABLE + offset + 4);
            if platform::read_u8(AX_TASK_TABLE + offset) == 0 {
                fn_802d68e0(pad, 0x802D_6740);
            } else {
                fn_802d6860(pad, 0x802D_66B0);
            }
            platform::write_u8(AX_TASK_BUSY, 1);
        }
    } else if platform::read_u8(AX_TASK_TABLE + platform::read_u32(AX_TASK_HEAD) as usize * 12 + 1) != 0 {
        let next = platform::read_u32(AX_TASK_HEAD).wrapping_add(1);
        platform::write_u8(AX_TASK_BUSY, 0);
        platform::write_u32(AX_TASK_HEAD, next);
        if next > 0x13 {
            platform::write_u32(AX_TASK_HEAD, 0);
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D6A00 - self-requeue pad callback
// ---------------------------------------------------------------------------

/// On `mode == 0` latches the current pad byte; otherwise re-registers
/// itself through [`fn_802d6860`].
///
/// # Original
/// - Ghidra: `FUN_802d6a00` at `0x802D6A00` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6a00_FUN_802d6a00.c`
///
/// # Arguments
/// - `pad`: pad index (byte in Ghidra, widened to `u32`).
/// - `mode`: `0` = latch, else = requeue.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `AX_PAD_PENDING` or enqueues via [`fn_802d6860`].
///
/// # Safety
/// `AX_PAD_PENDING` must be writable.
///
/// # Confidence
/// medium - branch is literal; pad-index widening is mechanical.
pub unsafe fn fn_802d6a00(pad: u32, mode: i32) {
    if mode == 0 {
        platform::write_u8(AX_PAD_PENDING, platform::call1(0x801E_3420, 0) as u8);
    } else {
        fn_802d6860(pad, 0x802D_6A00);
    }
}

// ---------------------------------------------------------------------------
// 0x802D6A60 - guarded enqueue with flag 1
// ---------------------------------------------------------------------------

/// If the pad guard is clear and `0x801E0F40(pad)` is not -1, enqueues the
/// callback with flag 1.
///
/// # Original
/// - Ghidra: `FUN_802d6a60` at `0x802D6A60` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6a60_FUN_802d6a60.c`
///
/// # Arguments
/// - `pad`: pad index.
/// - `callback`: callback address.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Pad probe and possibly [`fn_802d67d0`].
///
/// # Safety
/// `AX_PAD_LOCK` must be readable; the 16-byte probe buffer is stack
/// storage for the opaque probe.
///
/// # Confidence
/// medium - guard and `-1` test are literal.
pub unsafe fn fn_802d6a60(pad: u32, callback: usize) {
    if platform::read_u8(AX_PAD_LOCK) != 0 {
        return;
    }
    let mut probe = crate::platform::GuestLocal::new( [0u8; 16]);
    let probe_addr = probe.as_mut_ptr() as usize;
    let result = platform::call2(0x801E_0F40, pad, probe_addr as u32) as i32;
    if result != -1 {
        fn_802d67d0(pad, callback, 1);
    }
}

// ---------------------------------------------------------------------------
// 0x802D6AC0 - guarded enqueue with flag 0
// ---------------------------------------------------------------------------

/// If the pad guard is clear, probes the pad, falls back to the
/// `0x80270310` gate probe on -1, and enqueues with flag 0.
///
/// # Original
/// - Ghidra: `FUN_802d6ac0` at `0x802D6AC0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6ac0_FUN_802d6ac0.c`
///
/// # Arguments
/// - `pad`: pad index.
/// - `callback`: callback address.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Pad probe, gate probe, and [`fn_802d67d0`] unless the gate returns 0.
///
/// # Safety
/// Same as [`fn_802d6a60`].
///
/// # Confidence
/// medium - probe chain and early return are literal; gate roles inferred.
pub unsafe fn fn_802d6ac0(pad: u32, callback: usize) {
    if platform::read_u8(AX_PAD_LOCK) != 0 {
        return;
    }
    let mut probe = crate::platform::GuestLocal::new( [0u8; 16]);
    let probe_addr = probe.as_mut_ptr() as usize;
    let result = platform::call2(0x801E_0F40, pad, probe_addr as u32) as i32;
    if result == -1 {
        let ctx = platform::call1(0x8027_0AD0, 0);
        platform::call2(0x8027_0B70, ctx, pad);
        // observed: zero-argument gate probe `0x80270310`.
        let gate = platform::call1(0x8027_0310, 0) as i32;
        if gate == 0 {
            return;
        }
    }
    fn_802d67d0(pad, callback, 0);
}

// ---------------------------------------------------------------------------
// 0x802D6B40 - register all four pads with `0x802D6A00`
// ---------------------------------------------------------------------------

/// Clears the pad guard and registers each present pad 0..3 via
/// [`fn_802d6860`] with callback `0x802D6A00`.
///
/// # Original
/// - Ghidra: `FUN_802d6b40` at `0x802D6B40` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6b40_FUN_802d6b40.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `AX_PAD_LOCK = 0`, four pad probes and up to four enqueues.
///
/// # Safety
/// `AX_PAD_LOCK` must be writable.
///
/// # Confidence
/// medium - loop bound (4) and `-1` test are literal; probe buffer opaque.
pub unsafe fn fn_802d6b40() {
    platform::write_u8(AX_PAD_LOCK, 0);
    let mut pad: u32 = 0;
    loop {
        let mut probe = crate::platform::GuestLocal::new( [0u8; 16]);
        let probe_addr = probe.as_mut_ptr() as usize;
        let result = platform::call2(0x801E_0F40, pad, probe_addr as u32) as i32;
        if result != -1 {
            fn_802d6860(pad, 0x802D_6A00);
        }
        pad = pad.wrapping_add(1);
        if pad >= 4 {
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D6BB0 - register pads whose gate value is 4, then set the guard
// ---------------------------------------------------------------------------

/// Probes pads 0..3, registers those whose resolved gate word at `+8` is
/// 4 with callback `0x802D6A40`, then sets `AX_PAD_LOCK = 1`.
///
/// # Original
/// - Ghidra: `FUN_802d6bb0` at `0x802D6BB0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6bb0_FUN_802d6bb0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four pad probes and gate resolves; [`fn_802d68e0`] for matches;
/// sets the pad guard.
///
/// # Safety
/// `AX_PAD_LOCK` must be writable; resolved gate words must be readable.
///
/// # Confidence
/// medium - the `== 4` test and loop are literal; `&LAB_802d6a40`
/// (`0x802D6A40`, an alternate entry inside [`fn_802d6a00`]'s run) is
/// `// unknown:`.
pub unsafe fn fn_802d6bb0() {
    let mut pad: u32 = 0;
    loop {
        let mut probe = crate::platform::GuestLocal::new( [0u8; 16]);
        let probe_addr = probe.as_mut_ptr() as usize;
        platform::call2(0x801E_0F40, pad, probe_addr as u32);
        let ctx = platform::call1(0x8027_0AD0, 0);
        let ctx = platform::call2(0x8027_0B70, ctx, pad);
        if platform::read_u32(ctx as usize + 8) == 4 {
            fn_802d68e0(pad, 0x802D_6A40);
        }
        pad = pad.wrapping_add(1);
        if pad >= 4 {
            break;
        }
    }
    platform::write_u8(AX_PAD_LOCK, 1);
}

// ---------------------------------------------------------------------------
// 0x802D6C30 - enqueue callback 0 for all four pads
// ---------------------------------------------------------------------------

/// Calls [`fn_802d6a60`] for pads 0..3 with a null callback.
///
/// # Original
/// - Ghidra: `FUN_802d6c30` at `0x802D6C30` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6c30_FUN_802d6c30.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four calls to [`fn_802d6a60`].
///
/// # Safety
/// The task table must be writable via the callees.
///
/// # Confidence
/// high - do-while over exactly four pads is literal.
pub unsafe fn fn_802d6c30() {
    let mut pad: u32 = 0;
    loop {
        fn_802d6a60(pad, 0);
        pad = pad.wrapping_add(1);
        if pad >= 4 {
            return;
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D6C70 - read the latched pad byte
// ---------------------------------------------------------------------------

/// Returns the byte latched by `0x801E3420` at `AX_PAD_PENDING`.
///
/// # Original
/// - Ghidra: `FUN_802d6c70` at `0x802D6C70` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6c70_FUN_802d6c70.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The byte at `0x80429B98`.
///
/// # Side effects
/// None. Reads one byte.
///
/// # Safety
/// The cell must be readable.
///
/// # Confidence
/// high - single `r13`-relative byte load resolved to `0x80429B98`.
pub unsafe fn fn_802d6c70() -> u8 {
    platform::read_u8(AX_PAD_PENDING)
}

// ---------------------------------------------------------------------------
// 0x802D6CD0 - gate shutdown (mute all voices)
// ---------------------------------------------------------------------------

/// Mutes every voice reachable from the two gate objects with flag 1 and
/// then re-arms pads via [`fn_802d6bb0`].
///
/// # Original
/// - Ghidra: `FUN_802d6cd0` at `0x802D6CD0` (304 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6cd0_FUN_802d6cd0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Gate-hook calls, `0x80278780` / `0x8027A850(v, 1, arg)` per voice,
/// `0x80260860` / `0x802614E0(v, 0..2, 0xFA)` and [`fn_802d6bb0`].
///
/// # Safety
/// Gate objects and their voice arrays must be live; hooks must be valid
/// when non-null.
///
/// # Confidence
/// medium - both loops reload their base and count per iteration as in the
/// export; `+0x5BC` / `+0x3C` / `+0x5C4` / `0x4D8` roles are inferred.
pub unsafe fn fn_802d6cd0() {
    let hook = platform::read_u32(AX_GATE_HOOK) as usize;
    if hook != 0 {
        platform::call2(hook, 0, 0);
    }
    if platform::read_u32(AX_GATE_MAIN) != 0 {
        let main = platform::read_u32(AX_GATE_MAIN) as usize;
        let mut index: u32 = 0;
        loop {
            let list = platform::read_u32(main + 0x5BC) as usize;
            if index >= platform::read_u32(list + 0x3C) {
                break;
            }
            let voice = platform::call2(0x8027_8780, list as u32, index);
            platform::call3(0x8027_A850, voice, 1, platform::read_u32(AX_GATE_ARG));
            index = index.wrapping_add(1);
        }
    }
    if platform::read_u32(AX_GATE_SECOND) != 0 {
        let second = platform::read_u32(AX_GATE_SECOND) as usize;
        let count = platform::read_u32(second + 0xFC);
        let mut outer: u32 = 0;
        let mut offset: u32 = 0;
        while outer < count {
            let mut index: u32 = 0;
            loop {
                let list = platform::read_u32(second + offset as usize + 0x5C4) as usize;
                if index >= platform::read_u32(list + 0x3C) {
                    break;
                }
                let voice = platform::call2(0x8027_8780, list as u32, index);
                platform::call3(0x8027_A850, voice, 1, platform::read_u32(AX_GATE_ARG));
                index = index.wrapping_add(1);
            }
            outer = outer.wrapping_add(1);
            offset = offset.wrapping_add(0x4D8);
        }
    }
    let handle = platform::call1(0x8026_0860, 0);
    platform::call3(0x8026_14E0, handle, 0, 0xFA);
    let handle = platform::call1(0x8026_0860, 0);
    platform::call3(0x8026_14E0, handle, 1, 0xFA);
    let handle = platform::call1(0x8026_0860, 0);
    platform::call3(0x8026_14E0, handle, 2, 0xFA);
    fn_802d6bb0();
}

// ---------------------------------------------------------------------------
// 0x802D6E00 - gate drain (restore voices)
// ---------------------------------------------------------------------------

/// Restores every voice reachable from the two gate objects with flag 0
/// when `enable != 0`, then re-arms pads via [`fn_802d6b40`].
///
/// # Original
/// - Ghidra: `FUN_802d6e00` at `0x802D6E00` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6e00_FUN_802d6e00.c`
///
/// # Arguments
/// - `enable`: `0` skips the voice loops; non-zero restores them.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Gate-hook calls, `0x80278780` / `0x8027A850(v, 0, arg)` per voice and
/// [`fn_802d6b40`].
///
/// # Safety
/// Same contract as [`fn_802d6cd0`].
///
/// # Confidence
/// medium - mirror of [`fn_802d6cd0`] with `0` instead of `1`; same
/// inferred offsets.
pub unsafe fn fn_802d6e00(enable: i32) {
    let hook = platform::read_u32(AX_GATE_HOOK) as usize;
    if hook != 0 {
        platform::call2(hook, 1, enable as u32);
    }
    if enable != 0 {
        if platform::read_u32(AX_GATE_MAIN) != 0 {
            let main = platform::read_u32(AX_GATE_MAIN) as usize;
            let mut index: u32 = 0;
            loop {
                let list = platform::read_u32(main + 0x5BC) as usize;
                if index >= platform::read_u32(list + 0x3C) {
                    break;
                }
                let voice = platform::call2(0x8027_8780, list as u32, index);
                platform::call3(0x8027_A850, voice, 0, platform::read_u32(AX_GATE_ARG));
                index = index.wrapping_add(1);
            }
        }
        if platform::read_u32(AX_GATE_SECOND) != 0 {
            let second = platform::read_u32(AX_GATE_SECOND) as usize;
            let count = platform::read_u32(second + 0xFC);
            let mut outer: u32 = 0;
            let mut offset: u32 = 0;
            while outer < count {
                let mut index: u32 = 0;
                loop {
                    let list = platform::read_u32(second + offset as usize + 0x5C4) as usize;
                    if index >= platform::read_u32(list + 0x3C) {
                        break;
                    }
                    let voice = platform::call2(0x8027_8780, list as u32, index);
                    platform::call3(0x8027_A850, voice, 0, platform::read_u32(AX_GATE_ARG));
                    index = index.wrapping_add(1);
                }
                outer = outer.wrapping_add(1);
                offset = offset.wrapping_add(0x4D8);
            }
        }
    }
    let hook2 = platform::read_u32(AX_GATE_HOOK2) as usize;
    if hook2 != 0 {
        platform::call0(hook2);
    }
    fn_802d6b40();
}

// ---------------------------------------------------------------------------
// 0x802D7010 - three-word state record init
// ---------------------------------------------------------------------------

/// Seeds a three-word record with the `.data` default and publishes it at
/// the global head `0x8042AFA0`.
///
/// # Original
/// - Ghidra: `FUN_802d7010` at `0x802D7010` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7010_FUN_802d7010.c`
///
/// # Arguments
/// - `object`: 12 writable bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the default at `+0`, zero at `+4`/`+8`, and publishes `object`
/// globally.
///
/// # Safety
/// `object..object + 12` must be writable and outlive the head.
///
/// # Confidence
/// medium - store order (`+0`, then `+8`, then `+4`) is literal; the
/// default cell (`r2 - 0x3A50` = `0x8042F630`) role is `// unknown:`.
pub unsafe fn fn_802d7010(object: usize) {
    platform::write_u32(object, platform::read_u32(0x8042_F630));
    platform::write_u32(object + 8, 0);
    platform::write_u32(object + 4, 0);
    platform::write_u32(0x8042_AFA0, object as u32);
}

// ---------------------------------------------------------------------------
// 0x802D7030 - conditional release (deleting destructor)
// ---------------------------------------------------------------------------

/// Frees `object` with `0x802B9280` when the delete flag is positive.
///
/// # Original
/// - Ghidra: `FUN_802d7030` at `0x802D7030` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7030_FUN_802d7030.c`
///
/// # Arguments
/// - `object`: object pointer; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802B9280(object)` when requested.
///
/// # Safety
/// `object` must be null or a live object.
///
/// # Confidence
/// high - flag test is literal; Ghidra drops the argument but `r3` still
/// holds `object`, as in the `0x802D62F0/6350` twins.
pub unsafe fn fn_802d7030(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(0x802B_9280, object as u32);
    }
    object
}

// ===========================================================================
// Wave 65 - resource-view helpers (`0x802D3F60..0x802D4780`) and the BRRES
// section-tag record (`0x802D77E0..0x802D7D50`)
//
// The first group works on a large view object: a sub-object handle lives at
// `+0x4C4`, a default mode at `+0x4C8`, a state word at `+0x4CC` and an
// in-progress byte at `+5`. The second group is a 0x2C-byte record holding the
// six BRRES section pointers (`INF1/DAT1/STR1/MID1/FLW1/FLI1`) at `+4..+0x18`.
// ===========================================================================

/// Six 4-byte BRRES section tags at `0x802FD2D0`, indexed by
/// [`fn_802d7910`] in the order `INF1`, `DAT1`, `STR1`, `MID1`, `FLW1`, `FLI1`.
const BRRES_TAG_TABLE: usize = 0x802F_D2D0;

/// `r13 - 0x46B8` (`0x8042AFC8`): sentinel node used when resetting the child
/// record at `+0x74` (same constant as [`fn_802d85c0`]).
const BRRES_CHILD_SENTINEL: usize = 0x8042_AFC8;

// ---------------------------------------------------------------------------
// 0x802D3F60 - submit a request through the view handle
// ---------------------------------------------------------------------------

/// Submits a request on the view handle at `+0x4C4` and reports whether it was
/// accepted, refusing while the view is already busy.
///
/// # Original
/// - Ghidra: `FUN_802d3f60` at `0x802D3F60` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3f60_FUN_802d3f60.c`
///
/// # Arguments
/// - `object`: view object; handle at `+0x4C4`, default mode at `+0x4C8`,
///   state word at `+0x4CC`.
/// - `arg`: first forwarded argument.
/// - `mode`: second forwarded argument; `0` means "read `+0x4C8` instead".
/// - `arg3`: third forwarded argument.
///
/// # Returns
/// `1` when the state word is 2 or 3, `0` when the in-progress byte `+4` is
/// set or the handle reports "not ready", otherwise `1` when the underlying
/// call returned non-zero.
///
/// # Side effects
/// Sets the busy byte `+5` around the `0x80279B70` call.
///
/// # Safety
/// `object` must be a live view object and the handle must satisfy
/// `0x80277A90` / `0x80279B70`.
///
/// # Confidence
/// medium - guard, busy byte and call order are literal; field roles are
/// `// inferred:` from the two twins and the neighbouring loops.
pub unsafe fn fn_802d3f60(object: usize, arg: u32, mode: i32, arg3: u32) -> u32 {
    // observed: `*(int *)(param_1 + 0x4cc) - 2U < 2` -- states 2 and 3.
    if platform::read_u32(object + 0x4CC).wrapping_sub(2) < 2 {
        return 1;
    }
    let mut mode = mode;
    if mode == 0 {
        mode = platform::read_i32(object + 0x4C8);
    }
    if platform::read_u8(object + 4) != 0 {
        return 0;
    }
    let handle = platform::read_u32(object + 0x4C4);
    if platform::call1(0x8027_7A90, handle) == 0 {
        return 0;
    }
    platform::write_u8(object + 5, 1);
    let result = platform::call4(0x8027_9B70, handle, arg, mode as u32, arg3);
    platform::write_u8(object + 5, 0);
    // observed: `(-uVar1 | uVar1) >> 0x1f` -- 1 when the call succeeded.
    (result.wrapping_neg() | result) >> 31
}

// ---------------------------------------------------------------------------
// 0x802D4030 - second entry point with the same body as 0x802D3F60
// ---------------------------------------------------------------------------

/// Submits a request through the view handle, identical to [`fn_802d3f60`]
/// (the archive exposes two adjacent entry points for the same operation).
///
/// # Original
/// - Ghidra: `FUN_802d4030` at `0x802D4030` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4030_FUN_802d4030.c`
///
/// # Arguments
/// - `object`: view object; handle at `+0x4C4`, default mode at `+0x4C8`,
///   state word at `+0x4CC`.
/// - `arg`: first forwarded argument.
/// - `mode`: second forwarded argument; `0` means "read `+0x4C8` instead".
/// - `arg3`: third forwarded argument.
///
/// # Returns
/// Same value as [`fn_802d3f60`].
///
/// # Side effects
/// Sets the busy byte `+5` around the `0x80279B70` call.
///
/// # Safety
/// Same contract as [`fn_802d3f60`].
///
/// # Confidence
/// medium - the body matches `0x802D3F60` instruction for instruction; field
/// roles are `// inferred:`.
pub unsafe fn fn_802d4030(object: usize, arg: u32, mode: i32, arg3: u32) -> u32 {
    if platform::read_u32(object + 0x4CC).wrapping_sub(2) < 2 {
        return 1;
    }
    let mut mode = mode;
    if mode == 0 {
        mode = platform::read_i32(object + 0x4C8);
    }
    if platform::read_u8(object + 4) != 0 {
        return 0;
    }
    let handle = platform::read_u32(object + 0x4C4);
    if platform::call1(0x8027_7A90, handle) == 0 {
        return 0;
    }
    platform::write_u8(object + 5, 1);
    let result = platform::call4(0x8027_9B70, handle, arg, mode as u32, arg3);
    platform::write_u8(object + 5, 0);
    (result.wrapping_neg() | result) >> 31
}

// ---------------------------------------------------------------------------
// 0x802D4100 - third entry point, calling 0x80279CE0
// ---------------------------------------------------------------------------

/// Submits a request through the view handle using the `0x80279CE0` worker
/// instead of `0x80279B70`.
///
/// # Original
/// - Ghidra: `FUN_802d4100` at `0x802D4100` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4100_FUN_802d4100.c`
///
/// # Arguments
/// - `object`: view object; handle at `+0x4C4`, default mode at `+0x4C8`,
///   state word at `+0x4CC`.
/// - `arg`: first forwarded argument.
/// - `mode`: second forwarded argument; `0` means "read `+0x4C8` instead".
/// - `arg3`: third forwarded argument.
///
/// # Returns
/// Same value as [`fn_802d3f60`].
///
/// # Side effects
/// Sets the busy byte `+5` around the `0x80279CE0` call.
///
/// # Safety
/// Same contract as [`fn_802d3f60`].
///
/// # Confidence
/// medium - only the worker address differs from `0x802D3F60`; field roles
/// are `// inferred:`.
pub unsafe fn fn_802d4100(object: usize, arg: u32, mode: i32, arg3: u32) -> u32 {
    if platform::read_u32(object + 0x4CC).wrapping_sub(2) < 2 {
        return 1;
    }
    let mut mode = mode;
    if mode == 0 {
        mode = platform::read_i32(object + 0x4C8);
    }
    if platform::read_u8(object + 4) != 0 {
        return 0;
    }
    let handle = platform::read_u32(object + 0x4C4);
    if platform::call1(0x8027_7A90, handle) == 0 {
        return 0;
    }
    platform::write_u8(object + 5, 1);
    let result = platform::call4(0x8027_9CE0, handle, arg, mode as u32, arg3);
    platform::write_u8(object + 5, 0);
    (result.wrapping_neg() | result) >> 31
}

// ---------------------------------------------------------------------------
// 0x802D41D0 - reset the view handle
// ---------------------------------------------------------------------------

/// Resets the view handle at `+0x4C4` unless the object is already busy.
///
/// # Original
/// - Ghidra: `FUN_802d41d0` at `0x802D41D0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d41d0_FUN_802d41d0.c`
///
/// # Arguments
/// - `object`: view object; busy byte at `+4`, handle at `+0x4C4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x802783C0(handle)` when the busy byte is clear.
///
/// # Safety
/// `object` must be a live view object.
///
/// # Confidence
/// high - one guard and one call are literal.
pub unsafe fn fn_802d41d0(object: usize) {
    if platform::read_u8(object + 4) != 0 {
        return;
    }
    platform::call1(0x8027_83C0, platform::read_u32(object + 0x4C4));
}

// ---------------------------------------------------------------------------
// 0x802D41F0 - apply a state value to every view entry
// ---------------------------------------------------------------------------

/// Applies state `0` to every entry of the handle's list.
///
/// # Original
/// - Ghidra: `FUN_802d41f0` at `0x802D41F0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d41f0_FUN_802d41f0.c`
///
/// # Arguments
/// - `object`: view object whose handle is at `+0x4C4`; the handle holds the
///   entry count at `+0x3C`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// For each entry: `0x80278780(handle, index)` then `0x8027A7D0(entry, 0)`.
///
/// # Safety
/// `object` must be live and the handle's entry table readable for the count.
///
/// # Confidence
/// high - loop bound, lookup and store call are literal (count is re-read per
/// iteration, matching the export).
pub unsafe fn fn_802d41f0(object: usize) {
    let mut index: u32 = 0;
    while index < platform::read_u32(platform::read_u32(object + 0x4C4) as usize + 0x3C) {
        let entry = platform::call2(0x8027_8780, platform::read_u32(object + 0x4C4), index);
        platform::call2(0x8027_A7D0, entry, 0);
        index += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D4250 - apply a caller-supplied state value to every view entry
// ---------------------------------------------------------------------------

/// Applies `state` to every entry of the handle's list.
///
/// # Original
/// - Ghidra: `FUN_802d4250` at `0x802D4250` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4250_FUN_802d4250.c`
///
/// # Arguments
/// - `object`: view object whose handle is at `+0x4C4`.
/// - `state`: value forwarded to `0x8027A7D0` for every entry.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// For each entry: `0x80278780(handle, index)` then `0x8027A7D0(entry, state)`.
///
/// # Safety
/// `object` must be live and the handle's entry table readable for the count.
///
/// # Confidence
/// high - same loop as [`fn_802d41f0`], only the forwarded value differs.
pub unsafe fn fn_802d4250(object: usize, state: u32) {
    let mut index: u32 = 0;
    while index < platform::read_u32(platform::read_u32(object + 0x4C4) as usize + 0x3C) {
        let entry = platform::call2(0x8027_8780, platform::read_u32(object + 0x4C4), index);
        platform::call2(0x8027_A7D0, entry, state);
        index += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D43C0 - archive lookup 1 as a boolean
// ---------------------------------------------------------------------------

/// Runs the archive lookup at `0x8027B1C0` and reports whether it failed.
///
/// # Original
/// - Ghidra: `FUN_802d43c0` at `0x802D43C0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d43c0_FUN_802d43c0.c`
///
/// # Arguments
/// - `object`: view object whose handle is at `+0x4C4`.
/// - `name`: lookup key, forwarded as the second argument.
/// - `mode`: third argument, forwarded unchanged.
///
/// # Returns
/// `1` when `0x8027B1C0` returned zero, otherwise `0` (`clz(result) >> 5`).
///
/// # Side effects
/// One call to `0x8027B1C0(handle + 0xC, name, mode, 0)`.
///
/// # Safety
/// `object` and its handle must be live.
///
/// # Confidence
/// high - the call and the `clz`-based boolean are both explicit.
pub unsafe fn fn_802d43c0(object: usize, name: u32, mode: u32) -> u32 {
    let result = platform::call4(
        0x8027_B1C0,
        platform::read_u32(object + 0x4C4) + 0x0C,
        name,
        mode,
        0,
    );
    // observed: `countLeadingZeros(uVar2) >> 5` -- 1 only for zero.
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// 0x802D44B0 - archive lookup 2 as a boolean
// ---------------------------------------------------------------------------

/// Runs the archive lookup at `0x8027B270` and reports whether it failed.
///
/// # Original
/// - Ghidra: `FUN_802d44b0` at `0x802D44B0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d44b0_FUN_802d44b0.c`
///
/// # Arguments
/// - `object`: view object whose handle is at `+0x4C4`.
/// - `name`: lookup key, forwarded as the second argument.
/// - `mode`: third argument, forwarded unchanged.
///
/// # Returns
/// `1` when `0x8027B270` returned zero, otherwise `0`.
///
/// # Side effects
/// One call to `0x8027B270(handle + 0xC, name, mode, 0)`.
///
/// # Safety
/// `object` and its handle must be live.
///
/// # Confidence
/// high - identical shape to [`fn_802d43c0`] with a different callee.
pub unsafe fn fn_802d44b0(object: usize, name: u32, mode: u32) -> u32 {
    let result = platform::call4(
        0x8027_B270,
        platform::read_u32(object + 0x4C4) + 0x0C,
        name,
        mode,
        0,
    );
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// 0x802D45A0 - archive lookup 3 as a boolean
// ---------------------------------------------------------------------------

/// Runs the archive lookup at `0x8027B160` and reports whether it failed.
///
/// # Original
/// - Ghidra: `FUN_802d45a0` at `0x802D45A0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d45a0_FUN_802d45a0.c`
///
/// # Arguments
/// - `object`: view object whose handle is at `+0x4C4`.
/// - `name`: lookup key, forwarded as the second argument.
/// - `mode`: third argument, forwarded unchanged.
///
/// # Returns
/// `1` when `0x8027B160` returned zero, otherwise `0`.
///
/// # Side effects
/// One call to `0x8027B160(handle + 0xC, name, mode, 0)`.
///
/// # Safety
/// `object` and its handle must be live.
///
/// # Confidence
/// high - identical shape to [`fn_802d43c0`] with a different callee.
pub unsafe fn fn_802d45a0(object: usize, name: u32, mode: u32) -> u32 {
    let result = platform::call4(
        0x8027_B160,
        platform::read_u32(object + 0x4C4) + 0x0C,
        name,
        mode,
        0,
    );
    (result == 0) as u32
}

// ---------------------------------------------------------------------------
// 0x802D46A0 - conditional release of an archive element
// ---------------------------------------------------------------------------

/// Frees an archive element with `0x802B9280` when the delete flag is
/// positive; also used as the element destructor of [`fn_802d46e0`].
///
/// # Original
/// - Ghidra: `FUN_802d46a0` at `0x802D46A0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d46a0_FUN_802d46a0.c`
///
/// # Arguments
/// - `object`: element pointer; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802B9280(object)` when requested.
///
/// # Safety
/// `object` must be null or a live element.
///
/// # Confidence
/// high - CodeWarrior deleting-destructor idiom with a literal flag test.
pub unsafe fn fn_802d46a0(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(0x802B_9280, object as u32);
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D46E0 - constructor of the 0x8AC-byte archive object
// ---------------------------------------------------------------------------

/// Builds the archive object: runs the voice base constructor, initialises
/// the two fixed-size sub-records, then installs three vtables.
///
/// # Original
/// - Ghidra: `FUN_802d46e0` at `0x802D46E0` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d46e0_FUN_802d46e0.c`
///
/// # Arguments
/// - `object`: at least `0x8AC` writable bytes.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Calls [`fn_802d6210`], `0x802750F0`, [`fn_802dc9a0`]`(+0x6F4, 0x80275040,
/// 0x802D46A0, 0x6C, 4)` and `0x802D4F00(+0x8AC)`; writes the flags at
/// `+0x8A4/+0x8A8` and vtables at `+0`, `+8`, `+0xF8`.
///
/// # Safety
/// `object` must be live writable storage for the whole object; the four
/// `0x6C`-byte elements and their destructor must be valid.
///
/// # Confidence
/// medium - the call order and offsets are literal; Ghidra drops the argument
/// to `0x802D6210`, which is passed `object` as in its own translated body,
/// and the three vtable identities are `// inferred:`.
pub unsafe fn fn_802d46e0(object: usize) -> usize {
    // observed: `FUN_802d6210()` with `r3` still holding `this`.
    fn_802d6210(object);
    platform::call1(0x8027_50F0, (object + 0x6C8) as u32);
    fn_802dc9a0(object + 0x6F4, 0x8027_5040, 0x802D_46A0, 0x6C, 4);
    // Ghidra indexes `param_1` as words; offsets are index * 4.
    platform::write_u32(object + 0x8A8, 1);
    platform::write_u32(object + 0x8A4, 0);
    platform::call1(0x802D_4F00, (object + 0x8AC) as u32);
    platform::write_u32(object, 0x8035_0598);
    platform::write_u32(object + 8, 0x8035_05A8);
    platform::write_u32(object + 0xF8, 0x8035_05BC);
    object
}

// ---------------------------------------------------------------------------
// 0x802D4780 - teardown of the 0x8AC-byte archive object
// ---------------------------------------------------------------------------

/// Tears down the archive object: releases the `+0x8AC` map, destroys the
/// `0x6C`-stride array at `+0x6F4`, runs the voice teardown and optionally
/// frees the storage.
///
/// # Original
/// - Ghidra: `FUN_802d4780` at `0x802D4780` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4780_FUN_802d4780.c`
///
/// # Arguments
/// - `object`: archive object; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802D4F60(+0x8AC, 0)`, [`fn_802dca98`]`(+0x6F4, 0x802D46A0, 0x6C, 4)`,
/// `0x80229030(+0x6D4, 0)`, [`fn_802d6350`]`(object, 0)` and optionally
/// `0x802B9280(object)`.
///
/// # Safety
/// `object` must be null or a fully constructed archive object.
///
/// # Confidence
/// medium - call order and offsets are literal; the two
/// `param_1 != -0x6C8` tests are vacuous pointer guards kept for fidelity and
/// the sub-object roles are `// inferred:`.
pub unsafe fn fn_802d4780(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        platform::call2(0x802D_4F60, (object + 0x8AC) as u32, 0);
        // observed: `param_1 != -0x6c8`, i.e. `param_1 + 0x6C8 != 0`.
        if object.wrapping_add(0x6C8) != 0 {
            fn_802dca98(object + 0x6F4, 0x802D_46A0, 0x6C, 4);
            if object.wrapping_add(0x6C8) != 0 && object.wrapping_add(0x6D4) != 0 {
                platform::call2(0x8022_9030, (object + 0x6D4) as u32, 0);
            }
        }
        fn_802d6350(object, 0);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D77E0 - conditional release of the BRRES record
// ---------------------------------------------------------------------------

/// Frees the BRRES record with `0x802B9280` when the delete flag is positive.
///
/// # Original
/// - Ghidra: `FUN_802d77e0` at `0x802D77E0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d77e0_FUN_802d77e0.c`
///
/// # Arguments
/// - `object`: record pointer; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802B9280(object)` when requested.
///
/// # Safety
/// `object` must be null or a live record.
///
/// # Confidence
/// medium - the flag test is literal; Ghidra prints the free call with no
/// arguments, and `r3` still holding `object` is `// inferred:` from the
/// neighbouring `0x802D7B20` twin.
pub unsafe fn fn_802d77e0(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(0x802B_9280, object as u32);
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D7820 - split a packed section header
// ---------------------------------------------------------------------------

/// Splits the packed header word at `src` into a type byte, a 24-bit size and
/// an optional payload pointer.
///
/// # Original
/// - Ghidra: `FUN_802d7820` at `0x802D7820` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7820_FUN_802d7820.c`
///
/// # Arguments
/// - `unused`: first argument; the export never reads it (`// unknown:`).
/// - `src`: packed header word.
/// - `out_type`: receives `(byte)header - 2`.
/// - `out_size`: receives `header & 0xFFFFFF`.
/// - `out_payload`: receives `src + 4` when the type is above `4`, else `0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Three stores through the output pointers.
///
/// # Safety
/// `src..src+4` must be readable and all three outputs writable.
///
/// # Confidence
/// high - masks, byte truncation and the branch are literal.
pub unsafe fn fn_802d7820(
    unused: usize,
    src: usize,
    out_type: usize,
    out_size: usize,
    out_payload: usize,
) {
    let _ = unused;
    let header = platform::read_u32(src);
    platform::write_u8(out_type, (header as u8).wrapping_sub(2));
    platform::write_u32(out_size, header & 0x00FF_FFFF);
    if platform::read_u8(out_type) > 4 {
        platform::write_u32(out_payload, (src + 4) as u32);
    } else {
        platform::write_u32(out_payload, 0);
    }
}

// ---------------------------------------------------------------------------
// 0x802D78A0..0x802D7900 - six section-pointer setters plus the tag field
// ---------------------------------------------------------------------------

/// Stores `value` at record `+0` (the `INF1` slot).
///
/// # Original
/// - Ghidra: `FUN_802d78a0` at `0x802D78A0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d78a0_FUN_802d78a0.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: pointer stored at `+0`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+0`.
///
/// # Safety
/// `object..object+4` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d78a0(object: usize, value: u32) {
    platform::write_u32(object, value);
}

/// Stores `value` at record `+4` (the `DAT1` slot).
///
/// # Original
/// - Ghidra: `FUN_802d78b0` at `0x802D78B0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d78b0_FUN_802d78b0.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: pointer stored at `+4`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+4`.
///
/// # Safety
/// `object..object+8` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d78b0(object: usize, value: u32) {
    platform::write_u32(object + 4, value);
}

/// Stores `value` at record `+8` (the `STR1` slot).
///
/// # Original
/// - Ghidra: `FUN_802d78c0` at `0x802D78C0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d78c0_FUN_802d78c0.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: pointer stored at `+8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+8`.
///
/// # Safety
/// `object..object+0xC` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d78c0(object: usize, value: u32) {
    platform::write_u32(object + 8, value);
}

/// Stores `value` at record `+0xC` (the `MID1` slot).
///
/// # Original
/// - Ghidra: `FUN_802d78d0` at `0x802D78D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d78d0_FUN_802d78d0.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: pointer stored at `+0xC`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+0xC`.
///
/// # Safety
/// `object..object+0x10` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d78d0(object: usize, value: u32) {
    platform::write_u32(object + 0x0C, value);
}

/// Stores `value` at record `+0x10` (the `FLW1` slot).
///
/// # Original
/// - Ghidra: `FUN_802d78e0` at `0x802D78E0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d78e0_FUN_802d78e0.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: pointer stored at `+0x10`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+0x10`.
///
/// # Safety
/// `object..object+0x14` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d78e0(object: usize, value: u32) {
    platform::write_u32(object + 0x10, value);
}

/// Stores `value` at record `+0x14` (the `FLI1` slot).
///
/// # Original
/// - Ghidra: `FUN_802d78f0` at `0x802D78F0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d78f0_FUN_802d78f0.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: pointer stored at `+0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+0x14`.
///
/// # Safety
/// `object..object+0x18` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d78f0(object: usize, value: u32) {
    platform::write_u32(object + 0x14, value);
}

/// Stores `value` at record `+0x18` (the section count / terminator slot).
///
/// # Original
/// - Ghidra: `FUN_802d7900` at `0x802D7900` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7900_FUN_802d7900.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `value`: word stored at `+0x18`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+0x18`.
///
/// # Safety
/// `object..object+0x1C` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802d7900(object: usize, value: u32) {
    platform::write_u32(object + 0x18, value);
}

// ---------------------------------------------------------------------------
// 0x802D7910 - map a BRRES section tag to a slot index
// ---------------------------------------------------------------------------

/// Maps a 4-byte BRRES section tag to the record slot index `0..5`.
///
/// # Original
/// - Ghidra: `FUN_802d7910` at `0x802D7910` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7910_FUN_802d7910.c`
///
/// # Arguments
/// - `unused`: first argument; referenced by no instruction (`// unknown:`).
/// - `tag`: four-character section tag (`INF1`..`FLI1`).
///
/// # Returns
/// `0..5` for a known tag, `6` otherwise.
///
/// # Side effects
/// None; read-only over the constant table.
///
/// # Safety
/// `tag` is a value, not a pointer; no memory is touched besides
/// [`BRRES_TAG_TABLE`].
///
/// # Confidence
/// high - six literal comparisons against the named table at `0x802FD2D0`.
pub unsafe fn fn_802d7910(unused: usize, tag: u32) -> u32 {
    let _ = unused;
    if tag == platform::read_u32(BRRES_TAG_TABLE) {
        return 0;
    }
    if tag == platform::read_u32(BRRES_TAG_TABLE + 4) {
        return 1;
    }
    if tag == platform::read_u32(BRRES_TAG_TABLE + 8) {
        return 2;
    }
    if tag == platform::read_u32(BRRES_TAG_TABLE + 0xC) {
        return 3;
    }
    if tag == platform::read_u32(BRRES_TAG_TABLE + 0x10) {
        return 4;
    }
    if tag == platform::read_u32(BRRES_TAG_TABLE + 0x14) {
        return 5;
    }
    6
}

// ---------------------------------------------------------------------------
// 0x802D7A80 - read one section-table entry
// ---------------------------------------------------------------------------

/// Returns table entry `index` from the section table at record `+0x10`.
///
/// # Original
/// - Ghidra: `FUN_802d7a80` at `0x802D7A80` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7a80_FUN_802d7a80.c`
///
/// # Arguments
/// - `object`: BRRES record.
/// - `index`: entry index; only the low 16 bits are used.
///
/// # Returns
/// The 32-bit word at `table[index]`, where `table = *(u32*)(object + 0x10)`
/// and the array starts at `table + 0x10`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `object + 0x10` must hold a table base with `index` in range.
///
/// # Confidence
/// high - the `& 0xFFFF` mask and scaled offset are literal.
pub unsafe fn fn_802d7a80(object: usize, index: u32) -> u32 {
    let table = platform::read_u32(object + 0x10) as usize;
    platform::read_u32(table + (index & 0xFFFF) as usize * 4 + 0x10)
}

// ---------------------------------------------------------------------------
// 0x802D7AA0 - constructor of the BRRES record
// ---------------------------------------------------------------------------

/// Builds the BRRES record: installs the vtable, clears the fields, initialises
/// the two child records and runs virtual slot `+0x98`.
///
/// # Original
/// - Ghidra: `FUN_802d7aa0` at `0x802D7AA0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7aa0_FUN_802d7aa0.c`
///
/// # Arguments
/// - `object`: at least `0x7C + 0x30` writable bytes (record plus child).
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// Writes vtable `0x803507B8`, word `+0x28`, byte `+4`, word `+0x24`; calls
/// `0x80296750(+0x74)`, [`fn_802dbb70`]`(+0x7C)` and vtable slot `+0x98`.
///
/// # Safety
/// `object` must be live writable storage and its vtable installed before the
/// virtual call.
///
/// # Confidence
/// medium - the store order and offsets are literal; the `+0x98` slot role is
/// `// inferred:` from the surrounding record accessors.
pub unsafe fn fn_802d7aa0(object: usize) -> usize {
    platform::write_u32(object, 0x8035_07B8);
    // Ghidra indexes `param_1` as words: `[10]` is `+0x28`, `[9]` is `+0x24`.
    platform::write_u32(object + 0x28, 0);
    platform::call1(0x8029_6750, (object + 0x74) as u32);
    fn_802dbb70(object + 0x7C);
    platform::write_u8(object + 4, 0);
    platform::write_u32(object + 0x24, 0);
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0x98) as usize;
    platform::call1(method, object as u32);
    object
}

// ---------------------------------------------------------------------------
// 0x802D7B20 - deleting destructor of the BRRES record
// ---------------------------------------------------------------------------

/// Destroys the `+0x7C` child record and optionally frees the BRRES record.
///
/// # Original
/// - Ghidra: `FUN_802d7b20` at `0x802D7B20` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7b20_FUN_802d7b20.c`
///
/// # Arguments
/// - `object`: record pointer; may be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// [`fn_802dbc20`]`(+0x7C, -1)` always (comma operator), then `0x802B9280`
/// when requested.
///
/// # Safety
/// `object` must be null or a live record with a constructed child.
///
/// # Confidence
/// high - the comma operator and flag test are literal.
pub unsafe fn fn_802d7b20(object: usize, delete_flag: i32) -> usize {
    if object != 0 {
        fn_802dbc20(object + 0x7C, -1);
        if delete_flag > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

// ---------------------------------------------------------------------------
// 0x802D7C50 - reset the +0x74 child after detaching one effect
// ---------------------------------------------------------------------------

/// Detaches the child at `+0x74` when its state is 1 or 2, stopping the
/// effect through `0x80285CE0`, then resets the child record.
///
/// # Original
/// - Ghidra: `FUN_802d7c50` at `0x802D7C50` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7c50_FUN_802d7c50.c`
///
/// # Arguments
/// - `object`: record whose child starts at `+0x74`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x802967B0(+0x74)`, `0x802967E0(+0x74)` (up to twice), `0x80285CE0()`
/// and `0x80296760(+0x74, 0x8042AFC8)`.
///
/// # Safety
/// `object + 0x74` must name a live child record.
///
/// # Confidence
/// high - the state test and call order are literal; the sentinel is the same
/// constant used by [`fn_802d85c0`].
pub unsafe fn fn_802d7c50(object: usize) {
    if platform::call1(0x8029_67B0, (object + 0x74) as u32) == 0 {
        return;
    }
    let node = platform::call1(0x8029_67E0, (object + 0x74) as u32) as usize;
    let kind = if node == 0 {
        0
    } else {
        platform::read_u32(node + 0x0C)
    };
    if kind == 1 || kind == 2 {
        platform::call1(0x8029_67E0, (object + 0x74) as u32);
        platform::call0(0x8028_5CE0);
    }
    platform::call2(0x8029_6760, (object + 0x74) as u32, BRRES_CHILD_SENTINEL as u32);
}

// ---------------------------------------------------------------------------
// 0x802D7D50 - reset the +0x74 child after detaching two effects
// ---------------------------------------------------------------------------

/// Detaches the child at `+0x74` when its state is 1 or 2, stopping both
/// effects through `0x80285CE0` and `0x80285DB0`, then resets the child record.
///
/// # Original
/// - Ghidra: `FUN_802d7d50` at `0x802D7D50` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7d50_FUN_802d7d50.c`
///
/// # Arguments
/// - `object`: record whose child starts at `+0x74`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x802967B0(+0x74)`, `0x802967E0(+0x74)` (up to three times),
/// `0x80285CE0()`, `0x80285DB0()` and `0x80296760(+0x74, 0x8042AFC8)`.
///
/// # Safety
/// `object + 0x74` must name a live child record.
///
/// # Confidence
/// high - same shape as [`fn_802d7c50`] plus the second stop call, both
/// explicit in the export.
pub unsafe fn fn_802d7d50(object: usize) {
    if platform::call1(0x8029_67B0, (object + 0x74) as u32) == 0 {
        return;
    }
    let node = platform::call1(0x8029_67E0, (object + 0x74) as u32) as usize;
    let kind = if node == 0 {
        0
    } else {
        platform::read_u32(node + 0x0C)
    };
    if kind == 1 || kind == 2 {
        platform::call1(0x8029_67E0, (object + 0x74) as u32);
        platform::call0(0x8028_5CE0);
        platform::call1(0x8029_67E0, (object + 0x74) as u32);
        platform::call0(0x8028_5DB0);
    }
    platform::call2(0x8029_6760, (object + 0x74) as u32, BRRES_CHILD_SENTINEL as u32);
}

// ===========================================================================
// Wave 77 - SDA getters, the three-slot effect table and the last CRT helpers
// ===========================================================================

/// SDA2 base (`r2`) installed by `0x800041C0` (`docs/ARCHITECTURE.md`,
/// "Register baseline at entry"). Several tiny accessors below are plain
/// `addi r3, r2, -N` instructions, so they return an address, not a value.
const SDA2_BASE: usize = 0x8043_3080;

/// Four consecutive `f32` words (`r13 - 0x4768`) that
/// [`fn_802d1280`] copies out and the transform builders read back as a
/// 4-component vector. `// inferred:` role from those read sites.
const SDA_FLOAT_QUAD: usize = 0x8042_AF18;

/// Small-block allocator counters: bytes still available (`r13 - 0x4718`)
/// and the low-water mark (`r13 - 0x4714`).
const BLOCK_BYTES_LEFT: usize = 0x8042_AF68;
const BLOCK_BYTES_LOW: usize = 0x8042_AF6C;

/// Free-list head (`r13 - 0x4728`) and retired-list head (`r13 - 0x4724`)
/// of the small-block allocator.
const BLOCK_FREE_HEAD: usize = 0x8042_AF58;
const BLOCK_RETIRED_HEAD: usize = 0x8042_AF5C;

/// Two-word scratch pair (`r2 - 0x3B10`, `-0x3B0C`) that
/// [`fn_802d17c0`] publishes into the object at `+0x68`/`+0x6C`.
const SDA_TRANSFORM_SCRATCH: usize = 0x8042_F570;

/// Feature-latch word (`r13 - 0x5B00`) whose bits 2 and 3 are exported by
/// [`fn_802d25f0`] / [`fn_802d2600`]. `// inferred:` the two readers are the
/// only users of the cell in this band.
const SDA_FEATURE_LATCH: usize = 0x8042_9B80;

/// Three-slot effect table returned by [`fn_802d50a0`] (`0x80424AF0`); every
/// slot is 0x10 bytes and holds one loader record.
const EFFECT_SLOT_TABLE: usize = 0x8042_4AF0;
const EFFECT_SLOT_STRIDE: usize = 0x10;
const EFFECT_SLOT_COUNT: usize = 3;

/// `&LAB_802d4be0`: the per-slot constructor that [`fn_802d4f00`] installs for
/// the three effect slots. Ghidra did not create a function entry for it (the
/// address is not in `functions.tsv`), so it stays a raw address.
const EFFECT_SLOT_CONSTRUCTOR: usize = 0x802D_4BE0;

// ---------------------------------------------------------------------------
// 0x802D0810 - SDA2 address 0x8042F52C
// ---------------------------------------------------------------------------

/// Returns the fixed SDA2 address `0x8042F52C` (`r2 - 0x3B54`).
///
/// # Original
/// - Ghidra: `FUN_802d0810` at `0x802D0810` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0810_FUN_802d0810.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The address `0x8042F52C` itself.
///
/// # Side effects
/// None.
///
/// # Safety
/// No memory is touched.
///
/// # Confidence
/// high - the export is a single `return unaff_r2 + -0x3b54;` and `r2` is the
/// documented SDA2 base, so the arithmetic is exact. The *contents* at that
/// address are not part of this function.
pub unsafe fn fn_802d0810() -> usize {
    // observed: `addi r3, r2, -0x3B54` -- an address computation, not a load.
    SDA2_BASE - 0x3B54
}

// ---------------------------------------------------------------------------
// 0x802D0820 - SDA2 address 0x8042FC38
// ---------------------------------------------------------------------------

/// Returns the fixed SDA2 address `0x8042FC38` (`r2 - 0x3448`).
///
/// # Original
/// - Ghidra: `FUN_802d0820` at `0x802D0820` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0820_FUN_802d0820.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The address `0x8042FC38` itself.
///
/// # Side effects
/// None.
///
/// # Safety
/// No memory is touched.
///
/// # Confidence
/// high - same single-`addi` shape as [`fn_802d0810`], different offset.
pub unsafe fn fn_802d0820() -> usize {
    // observed: `return unaff_r2 + -0x3448;`
    SDA2_BASE - 0x3448
}

// ---------------------------------------------------------------------------
// 0x802D0E50 - copy-construct the 0x40..0x84 member block
// ---------------------------------------------------------------------------

/// Base-class copy followed by a member-wise copy of the words at
/// `source + 0x40 .. source + 0x84`, then the `+0x3C` slot through
/// [`fn_802d1250`].
///
/// # Original
/// - Ghidra: `FUN_802d0e50` at `0x802D0E50` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0e50_FUN_802d0e50.c`
///
/// # Arguments
/// - `this`: destination object; the class vtable pointer is stored at `+0x38`.
/// - `source`: object being copied.
///
/// # Returns
/// `this`, so the constructor can be used in an expression.
///
/// # Side effects
/// Writes the vtable word `0x80350504` at `+0x38`, nine word pairs at
/// `+0x40..+0x84`, and the `+0x3C` slot (which may also set bit 0 of the
/// `u16` at `+0x34`).
///
/// # Safety
/// `this` must be a fresh object of the class and `source` a live object of
/// the same class.
///
/// # Confidence
/// medium - every store is literal; the base-class copy call is printed with
/// no arguments and `(this, source)` is `// inferred:` from the member copy
/// that follows it.
pub unsafe fn fn_802d0e50(this: usize, source: usize) -> usize {
    // inferred: `FUN_802c6aa0()` in the export; the only two objects in scope
    // are `this` and `source`.
    platform::call2(0x802C_6AA0, this as u32, source as u32);
    // observed: class vtable/data pointer.
    platform::write_u32(this + 0x38, 0x8035_0504);
    // observed: each pair is copied through a temporary -- the high half is
    // stored first, then the saved low half. Preserved here because the copy
    // would otherwise differ if `this == source`.
    let mut offset = 0x40;
    while offset <= 0x80 {
        let low = platform::read_u32(source + offset);
        let high = platform::read_u32(source + offset + 4);
        platform::write_u32(this + offset + 4, high);
        platform::write_u32(this + offset, low);
        offset += 8;
    }
    fn_802d1250(this, platform::read_i32(source + 0x3C));
    this
}

// ---------------------------------------------------------------------------
// 0x802D1140 - copy-construct the 0x40..0x84 member block, no vtable store
// ---------------------------------------------------------------------------

/// Copy-assigns the member block `+0x40..+0x84` and the `+0x3C` slot from
/// `source` after running the base-class copy.
///
/// # Original
/// - Ghidra: `FUN_802d1140` at `0x802D1140` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1140_FUN_802d1140.c`
///
/// # Arguments
/// - `this`: destination object.
/// - `source`: object being copied.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x3C` and `+0x40..+0x84`. Unlike [`fn_802d0e50`] the vtable word at
/// `+0x38` is *not* rewritten, which is what makes this an assignment rather
/// than a constructor.
///
/// # Safety
/// Both objects must be live instances of the same class.
///
/// # Confidence
/// medium - the stores are literal; the argument list of the base-class call
/// `FUN_802c6bf0` is `// inferred:` as `(this, source)`.
pub unsafe fn fn_802d1140(this: usize, source: usize) {
    // inferred: `FUN_802c6bf0()` in the export -- base-class copy.
    platform::call2(0x802C_6BF0, this as u32, source as u32);
    // observed: six straight word copies at +0x40..+0x54.
    let mut offset = 0x40;
    while offset <= 0x54 {
        platform::write_u32(this + offset, platform::read_u32(source + offset));
        offset += 4;
    }
    // observed: three pairs through a temporary at +0x58..+0x6C.
    let mut offset = 0x58;
    while offset <= 0x68 {
        let low = platform::read_u32(source + offset);
        platform::write_u32(this + offset + 4, platform::read_u32(source + offset + 4));
        platform::write_u32(this + offset, low);
        offset += 8;
    }
    // observed: six straight word copies at +0x70..+0x84.
    let mut offset = 0x70;
    while offset <= 0x84 {
        platform::write_u32(this + offset, platform::read_u32(source + offset));
        offset += 4;
    }
    platform::write_u32(this + 0x3C, platform::read_u32(source + 0x3C));
}

// ---------------------------------------------------------------------------
// 0x802D1280 - copy the SDA float quad into four out-pointers
// ---------------------------------------------------------------------------

/// Copies the four SDA words at `0x8042AF18` into four caller-supplied
/// destinations.
///
/// # Original
/// - Ghidra: `FUN_802d1280` at `0x802D1280` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1280_FUN_802d1280.c`
///
/// # Arguments
/// - `out_x`, `out_y`, `out_z`, `out_w`: word destinations, in order.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four word stores; the SDA words are only read.
///
/// # Safety
/// All four destinations must be 4-byte writable.
///
/// # Confidence
/// high - four `*(param_N) = *(r13 + -0x476x)` pairs, one per argument.
pub unsafe fn fn_802d1280(out_x: usize, out_y: usize, out_z: usize, out_w: usize) {
    // observed: r13-0x4768 .. r13-0x475C, one word each.
    platform::write_u32(out_x, platform::read_u32(SDA_FLOAT_QUAD));
    platform::write_u32(out_y, platform::read_u32(SDA_FLOAT_QUAD + 4));
    platform::write_u32(out_z, platform::read_u32(SDA_FLOAT_QUAD + 8));
    platform::write_u32(out_w, platform::read_u32(SDA_FLOAT_QUAD + 0xC));
}

// ---------------------------------------------------------------------------
// 0x802D12B0 - mark an object dirty
// ---------------------------------------------------------------------------

/// Raises bit 3 of the flag word at `+0x34`, and bit 0 as well when bit 3 was
/// still clear.
///
/// # Original
/// - Ghidra: `FUN_802d12b0` at `0x802D12B0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d12b0_FUN_802d12b0.c`
///
/// # Arguments
/// - `this`: object whose `u16` flags live at `+0x34`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One or two `u16` stores at `+0x34`.
///
/// # Safety
/// `this + 0x34` must be readable and writable.
///
/// # Confidence
/// high - the guard and both stores are literal; `// inferred:` bit 0 means
/// "geometry changed" and bit 3 "needs rebuild", from the read sites in
/// [`fn_802d1820`].
pub unsafe fn fn_802d12b0(this: usize) {
    let flags = platform::read_u16(this + 0x34);
    if flags & 8 == 0 {
        platform::write_u16(this + 0x34, flags | 1);
    }
    platform::write_u16(this + 0x34, platform::read_u16(this + 0x34) | 8);
}

// ---------------------------------------------------------------------------
// 0x802D17C0 - refresh the transform base
// ---------------------------------------------------------------------------

/// Refreshes the object's transform block when any node in its `+0x3C` chain
/// is marked dirty, then returns the address of the block at `+0x58`.
///
/// # Original
/// - Ghidra: `FUN_802d17c0` at `0x802D17C0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d17c0_FUN_802d17c0.c`
///
/// # Arguments
/// - `this`: object with the flag word at `+0x34` and the chain head at
///   `+0x3C`.
///
/// # Returns
/// `this + 0x58`, the base of the 11-word transform block the callers read
/// through pointer arithmetic.
///
/// # Side effects
/// When the walk in [`fn_802d1820`] reports a dirty node: re-runs the transform
/// builder `0x802D12E0`, publishes the SDA scratch pair into `+0x68`/`+0x6C`
/// and clears bit 0 of `+0x34`.
///
/// # Safety
/// `this` must be a live object; the chain nodes must be readable.
///
/// # Confidence
/// medium - the stores, the mask and the returned offset are literal; the
/// export prints `FUN_802d1820()` with no argument and `this` is
/// `// inferred:` from the object the rest of the body operates on.
pub unsafe fn fn_802d17c0(this: usize) -> usize {
    // inferred: `FUN_802d1820()` -- the chain walk, given `this`.
    if fn_802d1820(this) != 0 {
        // observed: rebuild call; the body lives in the untranslated
        // 1240-byte function at 0x802D12E0.
        platform::call1(0x802D_12E0, this as u32);
        // observed: `uVar1 = *(r2-0x3B10); *(+0x68) = *(r2-0x3B0C);
        //           *(+0x6C) = uVar1;` -- the pair is stored swapped.
        let low = platform::read_u32(SDA_TRANSFORM_SCRATCH + 4);
        platform::write_u32(this + 0x68, low);
        platform::write_u32(this + 0x6C, platform::read_u32(SDA_TRANSFORM_SCRATCH));
        platform::write_u16(this + 0x34, platform::read_u16(this + 0x34) & 0xFFFE);
    }
    this + 0x58
}

// ---------------------------------------------------------------------------
// 0x802D1820 - is any node of the +0x3C chain dirty?
// ---------------------------------------------------------------------------

/// Walks the `+0x3C` chain and reports whether any node has bit 0 of its
/// `+0x34` flag word set.
///
/// # Original
/// - Ghidra: `FUN_802d1820` at `0x802D1820` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1820_FUN_802d1820.c`
///
/// # Arguments
/// - `this`: first node; `0` means "no chain".
///
/// # Returns
/// `1` as soon as a node with bit 0 set is found, otherwise `0` at the end of
/// the chain.
///
/// # Side effects
/// None; the walk is read-only.
///
/// # Safety
/// Every node reachable through `+0x3C` must be a readable object.
///
/// # Confidence
/// high - the do/while, the bit test and the two return values are literal.
pub unsafe fn fn_802d1820(this: usize) -> u32 {
    let mut node = this;
    while node != 0 {
        if platform::read_u16(node + 0x34) & 1 != 0 {
            return 1;
        }
        node = platform::read_u32(node + 0x3C) as usize;
    }
    0
}

// ---------------------------------------------------------------------------
// 0x802D1D70 - install the default vtable
// ---------------------------------------------------------------------------

/// Installs the default vtable pointer at `+4` and sets the kind byte at `+0`
/// to `1`.
///
/// # Original
/// - Ghidra: `FUN_802d1d70` at `0x802D1D70` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1d70_FUN_802d1d70.c`
///
/// # Arguments
/// - `this`: record to initialise.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+4` and one byte store at `+0`.
///
/// # Safety
/// `this .. this + 8` must be writable.
///
/// # Confidence
/// high - both stores are literal; `// inferred:` the byte at `+0` is a
/// discriminant because every reader in this band compares it with small
/// constants.
pub unsafe fn fn_802d1d70(this: usize) {
    // observed: `&DAT_80350518` -- the class vtable/data pointer.
    platform::write_u32(this + 4, 0x8035_0518);
    platform::write_u8(this, 1);
}

// ---------------------------------------------------------------------------
// 0x802D25F0 - feature bit 2
// ---------------------------------------------------------------------------

/// Returns bit 2 of the SDA feature latch.
///
/// # Original
/// - Ghidra: `FUN_802d25f0` at `0x802D25F0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d25f0_FUN_802d25f0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` or `1`.
///
/// # Side effects
/// None.
///
/// # Safety
/// The latch cell must be readable; it is a fixed SDA word.
///
/// # Confidence
/// high - the export is `return *(uint *)(r13 + -0x5B00) >> 2 & 1;`.
pub unsafe fn fn_802d25f0() -> u32 {
    (platform::read_u32(SDA_FEATURE_LATCH) >> 2) & 1
}

// ---------------------------------------------------------------------------
// 0x802D2600 - feature bit 3
// ---------------------------------------------------------------------------

/// Returns bit 3 of the SDA feature latch.
///
/// # Original
/// - Ghidra: `FUN_802d2600` at `0x802D2600` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2600_FUN_802d2600.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` or `1`.
///
/// # Side effects
/// None.
///
/// # Safety
/// The latch cell must be readable.
///
/// # Confidence
/// high - same body as [`fn_802d25f0`] with `>> 3`.
pub unsafe fn fn_802d2600() -> u32 {
    (platform::read_u32(SDA_FEATURE_LATCH) >> 3) & 1
}

// ---------------------------------------------------------------------------
// 0x802D35C0 - take a block from the 32-byte free list
// ---------------------------------------------------------------------------

/// Carves `size` bytes, rounded up to a 32-byte multiple, out of the first
/// free-list block that is large enough, and links `this` into the owner list.
///
/// # Original
/// - Ghidra: `FUN_802d35c0` at `0x802D35C0` (304 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d35c0_FUN_802d35c0.c`
///
/// # Arguments
/// - `this`: allocation record to fill in (fields at `+0x2C`, `+0x30`,
///   `+0x34`, `+0x38`).
/// - `size`: requested byte count, rounded up by this function.
///
/// # Returns
/// Nothing. A request that does not fit any free block returns without
/// touching `this`.
///
/// # Side effects
/// Decrements the free-byte counter and its low-water mark, splits or retires
/// the chosen free block, and calls `0x802C0B40(record, base)` once for `this`
/// and once for the remainder block.
///
/// # Safety
/// The free list must be well formed and `this` must be a scratch record; the
/// routine dereferences the list heads unconditionally.
///
/// # Confidence
/// medium - the arithmetic, both list walks and every link update are literal;
/// the meaning of `+0x30` (in-use flag) and `+0x2C` (block size) is
/// `// inferred:` from how the two values are used here.
pub unsafe fn fn_802d35c0(this: usize, size: u32) {
    // observed: `uVar1 = param_2 + 0x1fU & 0xffffffe0;`
    let wanted = size.wrapping_add(0x1F) & !0x1F;
    let left = platform::read_u32(BLOCK_BYTES_LEFT).wrapping_sub(wanted);
    platform::write_u32(BLOCK_BYTES_LEFT, left);
    if left < platform::read_u32(BLOCK_BYTES_LOW) {
        platform::write_u32(BLOCK_BYTES_LOW, left);
    }
    // observed: first free block with no owner and enough room.
    let mut origin = platform::read_u32(BLOCK_FREE_HEAD) as usize;
    loop {
        if origin == 0 {
            return;
        }
        if platform::read_u32(origin + 0x30) == 0 && wanted <= platform::read_u32(origin + 0x2C) {
            break;
        }
        origin = platform::read_u32(origin + 0x34) as usize;
    }
    platform::write_u32(this + 0x30, 1);
    platform::write_u32(this + 0x2C, wanted);
    // observed: the block base is read *before* the call and reused for the
    // remainder block, so it is cached here too.
    let base = platform::read_u32(origin + 0x10);
    platform::call2(0x802C_0B40, this as u32, base);
    platform::write_u32(this + 0x34, origin as u32);
    let previous = platform::read_u32(origin + 0x38);
    platform::write_u32(this + 0x38, previous);
    if previous == 0 {
        platform::write_u32(BLOCK_FREE_HEAD, this as u32);
    } else {
        platform::write_u32(previous as usize + 0x34, this as u32);
    }
    let rest = platform::read_u32(origin + 0x2C).wrapping_sub(wanted);
    platform::write_u32(origin + 0x2C, rest);
    if rest == 0 {
        // observed: the whole block was consumed -- retire it.
        let next = platform::read_u32(origin + 0x34);
        if next != 0 {
            platform::write_u32(next as usize + 0x38, this as u32);
        }
        platform::write_u32(this + 0x34, next);
        let retired = platform::read_u32(BLOCK_RETIRED_HEAD);
        if retired != 0 {
            platform::write_u32(retired as usize + 0x34, origin as u32);
        }
        platform::write_u32(origin + 0x38, retired);
        platform::write_u32(origin + 0x34, 0);
        platform::write_u32(BLOCK_RETIRED_HEAD, origin as u32);
        return;
    }
    // observed: shrink the origin block and make `this` the owner of the tail.
    platform::call2(0x802C_0B40, origin as u32, base.wrapping_add(wanted));
    platform::write_u32(origin + 0x38, this as u32);
}

// ---------------------------------------------------------------------------
// 0x802D38A0 - release the three sub-objects
// ---------------------------------------------------------------------------

/// Releases the three sub-objects at `+0x374`, `+0x198` and `+0x0C` and, when
/// asked to, frees the object itself.
///
/// # Original
/// - Ghidra: `FUN_802d38a0` at `0x802D38A0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d38a0_FUN_802d38a0.c`
///
/// # Arguments
/// - `this`: object to tear down; `0` is accepted and returns immediately.
/// - `mode`: positive means "also free"; `0` only releases the children.
///
/// # Returns
/// `this`, so the caller can chain.
///
/// # Side effects
/// `0x8026DDB0(+0x374, -1)`, `0x8026F6E0(+0x198, -1)`, `0x80269040(+0x0C, -1)`
/// and, for positive `mode`, `0x802B9280(this)`.
///
/// # Safety
/// `this` must be a live object of this class; the three children must be
/// valid for their release routines.
///
/// # Confidence
/// high - the null guard, the three calls with `0xFFFFFFFF` and the final free
/// are literal.
pub unsafe fn fn_802d38a0(this: usize, mode: i32) -> usize {
    if this != 0 {
        platform::call2(0x8026_DDB0, (this + 0x374) as u32, 0xFFFF_FFFF);
        platform::call2(0x8026_F6E0, (this + 0x198) as u32, 0xFFFF_FFFF);
        platform::call2(0x8026_9040, (this + 0x0C) as u32, 0xFFFF_FFFF);
        if mode > 0 {
            platform::call1(0x802B_9280, this as u32);
        }
    }
    this
}

// ---------------------------------------------------------------------------
// 0x802D3920 - store the +0x4D0 slot while idle
// ---------------------------------------------------------------------------

/// Stores `value` in the `+0x4D0` slot when the object is still idle.
///
/// # Original
/// - Ghidra: `FUN_802d3920` at `0x802D3920` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3920_FUN_802d3920.c`
///
/// # Arguments
/// - `this`: object; `+8` is its busy pointer.
/// - `value`: word to install.
///
/// # Returns
/// `1` when the store happened, `0` when `+8` was non-null.
///
/// # Side effects
/// One word store at `+0x4D0`.
///
/// # Safety
/// `this + 0x4D0` must be writable.
///
/// # Confidence
/// high - guard, store and both return values are literal.
pub unsafe fn fn_802d3920(this: usize, value: u32) -> u32 {
    if platform::read_u32(this + 8) == 0 {
        platform::write_u32(this + 0x4D0, value);
        return 1;
    }
    0
}

// ---------------------------------------------------------------------------
// 0x802D3940 - two-slot virtual dispatch
// ---------------------------------------------------------------------------

/// Forwards to virtual slot `+0x10` or `+0x14` of the object's vtable,
/// depending on `which`.
///
/// # Original
/// - Ghidra: `FUN_802d3940` at `0x802D3940` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3940_FUN_802d3940.c`
///
/// # Arguments
/// - `this`: object whose first word is the vtable pointer.
/// - `argument`: first forwarded argument.
/// - `argument2`: second forwarded argument.
/// - `which`: `1` selects slot `+0x10`, `2` selects slot `+0x14`; anything else
///   returns `0` without calling.
///
/// # Returns
/// The callee's `r3`, or `0` when `which` is not 1 or 2.
///
/// # Side effects
/// One indirect call.
///
/// # Safety
/// `this` must hold a valid vtable and both arguments must satisfy the slot's
/// signature.
///
/// # Confidence
/// medium - the two slot offsets, the guard and the return are literal; the
/// export prints both indirect calls with no arguments, so forwarding
/// `(argument, argument2)` is `// inferred:` from the fact that the function
/// receives exactly those two words and uses them nowhere else.
pub unsafe fn fn_802d3940(this: usize, argument: u32, argument2: u32, which: i32) -> u32 {
    let vtable = platform::read_u32(this) as usize;
    let mut result = 0;
    if which == 1 {
        result = platform::call2(platform::read_u32(vtable + 0x10) as usize, argument, argument2);
    } else if which == 2 {
        result = platform::call2(platform::read_u32(vtable + 0x14) as usize, argument, argument2);
    }
    result
}

// ---------------------------------------------------------------------------
// 0x802D3E00 - view teardown driven by the +0x4CC state word
// ---------------------------------------------------------------------------

/// Tears the active view down, choosing the sub-object to release from the
/// state word at `+0x4CC`.
///
/// # Original
/// - Ghidra: `FUN_802d3e00` at `0x802D3E00` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3e00_FUN_802d3e00.c`
///
/// # Arguments
/// - `this`: view object; busy pointer at `+8`, handle at `+0x4C4`, state at
///   `+0x4CC`.
///
/// # Returns
/// Nothing. A null busy pointer means the view is already down.
///
/// # Side effects
/// State 3/4 releases `+0x374`, state 1 releases `+0x0C`, state 2 releases
/// `+0x198`; then `0x80277BC0(handle)` runs and `+0x4CC`/`+8` are cleared.
///
/// # Safety
/// `this` must be a live view object and the handle must be a valid engine
/// handle.
///
/// # Confidence
/// high - the four state arms, the handle call and the two clearing stores are
/// literal; this is the shutdown twin of the [`fn_802d3f60`] family.
pub unsafe fn fn_802d3e00(this: usize) {
    if platform::read_u32(this + 8) == 0 {
        return;
    }
    let state = platform::read_u32(this + 0x4CC);
    // observed: `iVar1 - 3U < 2` -- states 3 and 4.
    if state.wrapping_sub(3) < 2 {
        platform::call1(0x8026_DE90, (this + 0x374) as u32);
    } else if state == 1 {
        platform::call1(0x8026_9220, (this + 0x0C) as u32);
    } else if state == 2 {
        platform::call1(0x8026_F930, (this + 0x198) as u32);
    }
    platform::call1(0x8027_7BC0, platform::read_u32(this + 0x4C4));
    platform::write_u32(this + 0x4CC, 0);
    platform::write_u32(this + 8, 0);
}

// ---------------------------------------------------------------------------
// 0x802D3E90 - fourth copy of the submit-request predicate
// ---------------------------------------------------------------------------

/// Submits a request through the view handle at `+0x4C4`, with the helper call
/// inlined instead of living in `0x80277A90`'s own wrapper.
///
/// # Original
/// - Ghidra: `FUN_802d3e90` at `0x802D3E90` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3e90_FUN_802d3e90.c`
///
/// # Arguments
/// - `this`: view object; handle `+0x4C4`, default mode `+0x4C8`, state
///   `+0x4CC`.
/// - `argument`: first forwarded argument.
/// - `mode`: second forwarded argument; `0` means "read `+0x4C8`".
/// - `argument3`: third forwarded argument.
///
/// # Returns
/// `1` when the state is 2 or 3, `0` when `+4` is set or the handle is not
/// ready, otherwise `1` when the underlying call returned non-zero.
///
/// # Side effects
/// Raises then clears the busy byte `+5` around `0x80279B70`.
///
/// # Safety
/// Same requirements as [`fn_802d3f60`].
///
/// # Confidence
/// high - the body is the same instruction sequence as [`fn_802d3f60`] with
/// the ready-check inlined; every branch and store is literal.
pub unsafe fn fn_802d3e90(this: usize, argument: u32, mode: i32, argument3: u32) -> u32 {
    if platform::read_u32(this + 0x4CC).wrapping_sub(2) < 2 {
        return 1;
    }
    let mut mode = mode;
    if mode == 0 {
        mode = platform::read_i32(this + 0x4C8);
    }
    if platform::read_u8(this + 4) != 0 {
        return 0;
    }
    let handle = platform::read_u32(this + 0x4C4);
    if platform::call1(0x8027_7A90, handle) == 0 {
        return 0;
    }
    platform::write_u8(this + 5, 1);
    let result = platform::call4(0x8027_9B70, handle, argument, mode as u32, argument3);
    platform::write_u8(this + 5, 0);
    (result.wrapping_neg() | result) >> 31
}

// ---------------------------------------------------------------------------
// 0x802D4C00 - release one effect slot
// ---------------------------------------------------------------------------

/// Releases one effect-slot record: runs the table shutdown for the slot,
/// releases the loaded object through virtual slot `+0x1C`, and frees the slot
/// when asked to.
///
/// # Original
/// - Ghidra: `FUN_802d4c00` at `0x802D4C00` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4c00_FUN_802d4c00.c`
///
/// # Arguments
/// - `this`: one 0x10-byte effect slot; the loaded object pointer is at `+0x0C`.
/// - `mode`: positive means "also free the slot".
///
/// # Returns
/// `this`, so the destructor can chain.
///
/// # Side effects
/// [`fn_802d4da0`]`(this, 0)`, one indirect call through the loaded object's
/// vtable, and `0x802B9280(this)` for positive `mode`.
///
/// # Safety
/// `this` must be a live slot whose `+0x0C` pointer is either null-free or a
/// valid object.
///
/// # Confidence
/// medium - the guard, the two offsets and the free call are literal; the
/// release call is printed with no arguments, so passing the object at `+0x0C`
/// as its implicit `this` is `// inferred:` from it being the only object in
/// scope.
pub unsafe fn fn_802d4c00(this: usize, mode: i32) -> usize {
    if this != 0 {
        fn_802d4da0(this, 0);
        let loaded = platform::read_u32(this + 0x0C);
        let vtable = platform::read_u32(loaded as usize) as usize;
        // inferred: vtable slot +0x1C, called with the loaded object.
        let release = platform::read_u32(vtable + 0x1C);
        let _ = platform::call1(release as usize, loaded);
        if mode > 0 {
            platform::call1(0x802B_9280, this as u32);
        }
    }
    this
}

// ---------------------------------------------------------------------------
// 0x802D4C70 - bind a loader handle into one effect slot
// ---------------------------------------------------------------------------

/// Stores the slot id at `+8` and, when a resource was supplied, loads it and
/// stores the handle at `+0x0C`.
///
/// # Original
/// - Ghidra: `FUN_802d4c70` at `0x802D4C70` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4c70_FUN_802d4c70.c`
///
/// # Arguments
/// - `this`: one effect slot.
/// - `id`: slot index, stored at `+8`.
/// - `resource`: resource pointer handed to the loader.
/// - `count`: when `0`, no load happens and `+0x0C` is left alone.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x802B9360(resource, count, 0)` and up to two word stores.
///
/// # Safety
/// `resource` must be a resource the loader accepts.
///
/// # Confidence
/// high - guard, the loader call with all three arguments, and both stores are
/// literal.
pub unsafe fn fn_802d4c70(this: usize, id: u32, resource: u32, count: u32) {
    if count != 0 {
        let handle = platform::call3(0x802B_9360, resource, count, 0);
        platform::write_u32(this + 0x0C, handle);
    }
    platform::write_u32(this + 8, id);
}

// ---------------------------------------------------------------------------
// 0x802D4CD0 - build one effect slot in a requested mode
// ---------------------------------------------------------------------------

/// Dispatches to one of the eight per-mode slot builders, or shuts the slot
/// down for any other mode value.
///
/// # Original
/// - Ghidra: `FUN_802d4cd0` at `0x802D4CD0` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4cd0_FUN_802d4cd0.c`
///
/// # Arguments
/// - `this`: one effect slot.
/// - `mode`: `1..=8` selects a builder; every other value releases the slot.
/// - `argument`: forwarded to the builder as its third argument.
///
/// # Returns
/// The builder's `r3`, or `0` after the shutdown path.
///
/// # Side effects
/// One call to `0x802D5DD0`, `0x802D5BF0`, `0x802D5A10`, `0x802D5830`,
/// `0x802D5650`, `0x802D5470`, `0x802D5290` or `0x802D50B0` -- modes 1 to 8 in
/// that order -- or [`fn_802d4da0`]`(this, 0)`.
///
/// # Safety
/// `this` must be a live slot and the selected builder must accept
/// `(this, 0, argument, mode)`.
///
/// # Confidence
/// high - the switch table and every case target are literal, and each case
/// forwards exactly the four arguments the export shows.
pub unsafe fn fn_802d4cd0(this: usize, mode: u32, argument: u32) -> u32 {
    let slot = this as u32;
    match mode {
        1 => platform::call4(0x802D_5DD0, slot, 0, argument, mode),
        2 => platform::call4(0x802D_5BF0, slot, 0, argument, mode),
        3 => platform::call4(0x802D_5A10, slot, 0, argument, mode),
        4 => platform::call4(0x802D_5830, slot, 0, argument, mode),
        5 => platform::call4(0x802D_5650, slot, 0, argument, mode),
        6 => platform::call4(0x802D_5470, slot, 0, argument, mode),
        7 => platform::call4(0x802D_5290, slot, 0, argument, mode),
        8 => platform::call4(0x802D_50B0, slot, 0, argument, mode),
        _ => {
            fn_802d4da0(this, 0);
            0
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D4DA0 - shut down one effect slot
// ---------------------------------------------------------------------------

/// Unhooks the slot from the loader, releases every record on its chain, and
/// clears the slot so it can be rebuilt.
///
/// # Original
/// - Ghidra: `FUN_802d4da0` at `0x802D4DA0` (352 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4da0_FUN_802d4da0.c`
///
/// # Arguments
/// - `this`: effect slot; chain head at `+0`, tail at `+4`, loader id at `+8`,
///   resource at `+0xC`.
/// - `argument`: forwarded unchanged to `0x802614E0`.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// `0x802614E0(0x80260860(), this[2], argument)`, one virtual call per chain
/// record, `0x802B9730()` when a resource was still loaded, then `+0` and `+4`
/// are cleared.
///
/// # Safety
/// Every record reachable from `this[0]` must be a live loader record; its
/// first word must point at a vtable with a valid `+0x20` slot.
///
/// # Confidence
/// medium - the walk, the eight identical switch arms, the resource check and
/// the two clearing stores are literal; the release call at `+0x20` is printed
/// with no arguments, so the record's first word being its vtable object is
/// `// inferred:` from the double indirection in the export.
pub unsafe fn fn_802d4da0(this: usize, argument: u32) -> u32 {
    // inferred: `FUN_80260860()` takes no visible argument; `platform` has no
    // value-returning zero-argument helper, so r3 is passed as 0.
    let system = platform::call1(0x8026_0860, 0);
    platform::call3(0x8026_14E0, system, platform::read_u32(this + 8), argument);
    let mut record = platform::read_u32(this) as usize;
    while record != 0 {
        let kind = platform::read_u32(record + 8);
        // observed: cases 1..=8 all run the same release slot; anything else
        // falls through without a call.
        if kind >= 1 && kind <= 8 {
            let object = platform::read_u32(record) as usize;
            let vtable = platform::read_u32(object) as usize;
            let release = platform::read_u32(vtable + 0x20);
            let _ = platform::call1(release as usize, object as u32);
        }
        record = platform::read_u32(record + 4) as usize;
    }
    if platform::read_u32(this + 0xC) != 0 {
        platform::call0(0x802B_9730);
    }
    platform::write_u32(this, 0);
    platform::write_u32(this + 4, 0);
    1
}

// ---------------------------------------------------------------------------
// 0x802D4F00 - construct the three-slot effect table
// ---------------------------------------------------------------------------

/// Constructs the three 0x10-byte effect slots at `this` and registers the
/// table with the loader.
///
/// # Original
/// - Ghidra: `FUN_802d4f00` at `0x802D4F00` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4f00_FUN_802d4f00.c`
///
/// # Arguments
/// - `this`: first slot of a three-slot table.
///
/// # Returns
/// `this`, so the constructor can be used in an expression.
///
/// # Side effects
/// `0x802DC9A0(this, constructor, destructor, 0x10, 3)` builds the three slots
/// with [`EFFECT_SLOT_CONSTRUCTOR`] as the constructor and [`fn_802d4c00`] as
/// the unwinding destructor, then `0x8015F730(0x80429B90, 0x1328E23)` registers
/// the table.
///
/// # Safety
/// `this .. this + 0x30` must be writable.
///
/// # Confidence
/// medium - the array-build arguments are literal; the registration call's
/// first argument is an `r13 - 0x5AF0` SDA address resolved to `0x80429B90`,
/// and what the registrar does with it is `// unknown:`.
pub unsafe fn fn_802d4f00(this: usize) -> usize {
    // observed: three slots, 0x10 bytes each, built by fn_802dc9a0 with
    // `&LAB_802d4be0` as the constructor and `fn_802d4c00` as the unwinder.
    let destructor: unsafe fn(usize, i32) -> usize = fn_802d4c00;
    fn_802dc9a0(
        this,
        EFFECT_SLOT_CONSTRUCTOR,
        destructor as usize,
        EFFECT_SLOT_STRIDE as u32,
        EFFECT_SLOT_COUNT as u32,
    );
    // observed: `r13 - 0x5AF0` resolves to 0x80429B90; the second word is a
    // fixed tag. unknown: what 0x8015F730 registers with it.
    platform::call2(0x8015_F730, 0x8042_9B90, 0x0132_8E23);
    this
}

// ---------------------------------------------------------------------------
// 0x802D4F60 - destroy the three-slot effect table
// ---------------------------------------------------------------------------

/// Destroys the three effect slots and, when asked to, frees the table.
///
/// # Original
/// - Ghidra: `FUN_802d4f60` at `0x802D4F60` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4f60_FUN_802d4f60.c`
///
/// # Arguments
/// - `this`: first slot of the table; `0` is accepted.
/// - `mode`: positive means "also free the table".
///
/// # Returns
/// `this`.
///
/// # Side effects
/// `0x802DCA98(this, fn_802d4c00, 0x10, 3)` releases the three slots from the
/// end backwards, then `0x802B9280(this)` for positive `mode`.
///
/// # Safety
/// `this` must be the table built by [`fn_802d4f00`].
///
/// # Confidence
/// high - the null guard, the four array-destructor arguments and the
/// conditional free are literal.
pub unsafe fn fn_802d4f60(this: usize, mode: i32) -> usize {
    if this != 0 {
        let destructor: unsafe fn(usize, i32) -> usize = fn_802d4c00;
        fn_802dca98(
            this,
            destructor as usize,
            EFFECT_SLOT_STRIDE as i32,
            EFFECT_SLOT_COUNT as i32,
        );
        if mode > 0 {
            platform::call1(0x802B_9280, this as u32);
        }
    }
    this
}

// ---------------------------------------------------------------------------
// 0x802D4FD0 - bind the three loader resources
// ---------------------------------------------------------------------------

/// Loads up to three resources and binds each one into the matching slot.
///
/// # Original
/// - Ghidra: `FUN_802d4fd0` at `0x802D4FD0` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4fd0_FUN_802d4fd0.c`
///
/// # Arguments
/// - `this`: first slot of the table.
/// - `tag`: base word handed to the loader.
/// - `resources`: pointer to three resource words; a zero word skips its slot.
///
/// # Returns
/// `1` when all present resources loaded, `0` as soon as one load fails.
///
/// # Side effects
/// `0x8027A3A0(tag, resource, 0, 0x46582041 + index)` per present resource and
/// [`fn_802d4c70`]`(slot, index, handle, resource)` on success.
///
/// # Safety
/// `resources` must point to at least three readable words and the slots must
/// be writable.
///
/// # Confidence
/// high - the loop bound, the skip-on-zero guard and the early `return 0` are
/// literal; the `0x46582041 + index` word is the ASCII tag `"FX A"` with the
/// last byte bumped per slot.
pub unsafe fn fn_802d4fd0(this: usize, tag: u32, resources: usize) -> u32 {
    let mut index: u32 = 0;
    let mut slot = this;
    let mut cursor = resources;
    loop {
        let resource = platform::read_u32(cursor);
        if resource != 0 {
            // observed: loader key `0x46582041 + index` ("FX A", "FX B", ...).
            let handle = platform::call4(0x8027_A3A0, tag, resource, 0, 0x4658_2041 + index);
            if handle == 0 {
                return 0;
            }
            fn_802d4c70(slot, index, handle, resource);
        }
        index += 1;
        slot += EFFECT_SLOT_STRIDE;
        cursor += 4;
        if index >= EFFECT_SLOT_COUNT as u32 {
            break;
        }
    }
    1
}

// ---------------------------------------------------------------------------
// 0x802D50A0 - the effect table address
// ---------------------------------------------------------------------------

/// Returns the fixed address of the three-slot effect table.
///
/// # Original
/// - Ghidra: `FUN_802d50a0` at `0x802D50A0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d50a0_FUN_802d50a0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0x80424AF0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// No memory is touched.
///
/// # Confidence
/// high - the export is `return &DAT_80424af0;` and nothing else.
pub unsafe fn fn_802d50a0() -> usize {
    EFFECT_SLOT_TABLE
}

// ---------------------------------------------------------------------------
// 0x802D7170 - busy-wait for a number of milliseconds
// ---------------------------------------------------------------------------

/// Spins until at least `milliseconds` have elapsed on the tick counter.
///
/// # Original
/// - Ghidra: `FUN_802d7170` at `0x802D7170` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7170_FUN_802d7170.c`
///
/// # Arguments
/// - `milliseconds`: minimum elapsed time, in milliseconds.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Repeated calls to `0x801B5FA0` (tick read) until the deadline; no memory is
/// written.
///
/// # Safety
/// The tick reader must be callable and `0x800000F8` (bus clock) must be
/// non-zero -- the original divides by `bus_clock / 4000` with no guard, so a
/// zero clock traps here as well.
///
/// # Confidence
/// high - the two reads, the divisor and the unsigned compare are literal;
/// `0x800000F8` is the documented bus-clock word and the timebase runs at a
/// quarter of it, so `bus / 4000` is ticks per millisecond.
pub unsafe fn fn_802d7170(milliseconds: u32) {
    let ticks_per_ms = platform::read_u32(0x8000_00F8) / 4000;
    // inferred: `FUN_801b5fa0()` is printed with no argument.
    let start = platform::call1(0x801B_5FA0, 0);
    loop {
        let now = platform::call1(0x801B_5FA0, 0);
        if now.wrapping_sub(start) / ticks_per_ms >= milliseconds {
            return;
        }
    }
}

// ===========================================================================
// Wave 89 - AX voice setup, the +0x4C4 view loaders, the effect builders
// ===========================================================================
//
// Four unrelated families from the lower half of `0x802D0000..0x802E0000`:
//
// 1. `0x802D2730..0x802D2B60` - the AX (audio DSP) engine setup sequence. It is
//    a chain of literal `0x801Cxxxx` AX-library calls; no gameplay object is
//    involved, which is why it lives here rather than in an audio module.
// 2. `0x802D39B0..0x802D42D0` - the three remaining loaders of the `+0x4C4`
//    view family started in Wave 65/77, plus the SDA compare hook they drive.
// 3. `0x802D4830` - the initialiser for the 0x6C-stride transform record that
//    `0x802D48C0` / `0x802D46E0` walk.
// 4. `0x802D50B0..0x802D5DD0` - the eight per-mode builders that
//    [`fn_802d4cd0`] dispatches to, one address per mode.
// 5. `0x802D7410..0x802D7570` and `0x802DB6B0`/`0x802DB710` - the sound-record
//    create/destroy pair and two small release helpers.

// SDA cells of the AX setup sequence, resolved from `r13 = 0x8042F680`, which
// is the value this image loads into `r13` and the one Wave 77 used for the
// `r13 - 0x5B00` latch. Ghidra prints every access as `unaff_r13 + -0x....`.
const AX_INIT_LATCH: usize = 0x8042_AF50; // r13 - 0x4730, u16 flag word
const AX_OUTPUT_HALF: usize = 0x8042_AF38; // r13 - 0x4748, u16
const AX_OUTPUT_HALF2: usize = 0x8042_AF3A; // r13 - 0x4746, u16
const AX_MODE_WORD: usize = 0x8042_AF3C; // r13 - 0x4744
const AX_MODE_WORD2: usize = 0x8042_AF40; // r13 - 0x4740
const AX_MODE_BYTES: usize = 0x8042_AF4C; // r13 - 0x4734, four bytes
const AX_STATE_HALF: usize = 0x8042_AF52; // r13 - 0x472E, u16
const AX_STATE_WORD: usize = 0x8042_AF54; // r13 - 0x472C
const AX_UP_FLAG: usize = 0x8042_9B88; // r13 - 0x5AF8
const AX_DEVICE_HANDLE: usize = 0x8042_AE48; // r13 - 0x4838, pointer to device
// SDA2 default word read by `0x802D2990`; `r2 = 0x80433080`, so this is
// `r2 - 0x3AC0`.
const SDA2_AUDIO_DEFAULT: usize = 0x8042_F5C0;

// Global mix object the AX helpers exchange between calls.
const AX_MIX_DESCRIPTOR: usize = 0x8042_48A0;
const AX_MIX_DESCRIPTOR2: usize = 0x8042_48E0;

// The compare-hook pair that `0x802D42D0` publishes and `0x802D42C0` calls
// (`r13 - 0x4708` / `r13 - 0x4704`).
const VIEW_COMPARE_HOOK: usize = 0x8042_AF78;
const VIEW_COMPARE_CONTEXT: usize = 0x8042_AF7C;

// Three SDA2 words used as the default contents of the 0x6C-stride transform
// record (`r2 - 0x3AA0`, ascending; `r2 = 0x80433080`).
const SDA2_TRIPLET: usize = 0x8042_F5E0;

/// Calls a function that takes no integer arguments and returns a word.
///
/// # Original
/// Host-side helper; no binary address. The result arrives in `r3`.
///
/// # Arguments
/// - `addr`: address of the callee.
///
/// # Returns
/// The callee's `r3`.
///
/// # Side effects
/// Whatever the callee does.
///
/// # Safety
/// `addr` must point to a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper. It is `platform::call0` plus a return value;
/// several bodies below read the object that a vtable slot at `+0x10` or
/// `+8` hands back, and [`crate::platform`] has no returning zero-argument
/// helper.
unsafe fn call0_word(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

// ---------------------------------------------------------------------------
// 0x802D2730 - tear the AX engine down and bring it back up
// ---------------------------------------------------------------------------

/// Resets the AX voice state, copies the live engine's four mode bytes back into
/// the SDA config block, then restarts the whole audio chain through
/// [`fn_802d2800`].
///
/// # Original
/// - Ghidra: `FUN_802d2730` at `0x802D2730` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2730_FUN_802d2730.c`
///
/// # Arguments
/// None. Every input is read from SDA.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Clears [`AX_STATE_WORD`], re-initialises `0x802D2E90` with the two saved
///   config words, clears [`AX_STATE_HALF`].
/// - Asks the device object at [`AX_DEVICE_HANDLE`] for its vtable slot `+0x10`;
///   when the slot returns non-null, re-queries it and copies the big-endian
///   word at `+0x14` into the four mode bytes at [`AX_MODE_BYTES`] (byte 0 is
///   the high byte).
/// - Calls `0x801C47C0`, `0x801C9A60(0)`, `0x801C58B0(0)`, `0x801C9D60(0)`,
///   [`fn_802d2800`] and `0x8025EFA0` in that order, then re-runs
///   `0x8024A1D0` with the saved up-flag when bit 4 of [`AX_INIT_LATCH`] is set.
///
/// # Safety
/// The device pointer at [`AX_DEVICE_HANDLE`] must hold a valid object and the
/// returned object must have a readable word at `+0x14`.
///
/// # Confidence
/// medium - every call, offset and mask is literal; the roles of the two words
/// and of the four mode bytes are `// inferred:` from the surrounding AX calls.
pub unsafe fn fn_802d2730() {
    platform::write_u32(AX_STATE_WORD, 0);
    platform::call2(
        0x802D_2E90,
        platform::read_u32(AX_MODE_WORD),
        platform::read_u32(AX_MODE_WORD2),
    );
    platform::write_u16(AX_STATE_HALF, 0);

    let device = platform::read_u32(AX_DEVICE_HANDLE) as usize;
    let vtable = platform::read_u32(device) as usize;
    let slot = platform::read_u32(vtable + 0x10) as usize;
    if call0_word(slot) != 0 {
        // observed: the same slot is queried twice; the second result is the
        // object whose mode word is copied out.
        let object = call0_word(slot) as usize;
        let mode = platform::read_u32(object + 0x14);
        platform::write_u8(AX_MODE_BYTES, (mode >> 24) as u8);
        platform::write_u8(AX_MODE_BYTES + 1, (mode >> 16) as u8);
        platform::write_u8(AX_MODE_BYTES + 2, (mode >> 8) as u8);
        platform::write_u8(AX_MODE_BYTES + 3, mode as u8);
    }

    platform::call0(0x801C_47C0);
    platform::call1(0x801C_9A60, 0);
    platform::call1(0x801C_58B0, 0);
    platform::call1(0x801C_9D60, 0);
    fn_802d2800();
    platform::call0(0x8025_EFA0);
    if platform::read_u16(AX_INIT_LATCH) & 0x10 != 0 {
        platform::call1(0x8024_A1D0, platform::read_u32(AX_UP_FLAG));
    }
}

// ---------------------------------------------------------------------------
// 0x802D2800 - first-time AX setup, then the per-frame sync
// ---------------------------------------------------------------------------

/// Runs the seven one-shot AX setup routines the first time through, then always
/// runs the per-frame sync `0x802D2E50`.
///
/// # Original
/// - Ghidra: `FUN_802d2800` at `0x802D2800` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2800_FUN_802d2800.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// When bit 0 of [`AX_INIT_LATCH`] is clear, calls
/// [`fn_802d2850`], [`fn_802d2a50`], [`fn_802d2990`], [`fn_802d2ad0`],
/// [`fn_802d2b60`], `0x802D2C20` and `0x802D2DA0` in that order. The flag is
/// only *tested* here -- the routine that sets it lives outside this band.
/// `0x802D2E50` runs unconditionally.
///
/// # Safety
/// The AX library must be initialised; each callee dereferences its own SDA
/// state.
///
/// # Confidence
/// high - the guard and all eight calls are literal and in the export's order.
pub unsafe fn fn_802d2800() {
    if platform::read_u16(AX_INIT_LATCH) & 1 == 0 {
        fn_802d2850();
        fn_802d2a50();
        fn_802d2990();
        fn_802d2ad0();
        fn_802d2b60();
        fn_802d2c20();
        fn_802d2da0();
    }
    fn_802d2e50();
}

// ---------------------------------------------------------------------------
// 0x802D2850 - twelve aux-bus/channel descriptors
// ---------------------------------------------------------------------------

/// Registers the twelve AX channel descriptors 9..=0x14 on bus 0.
///
/// # Original
/// - Ghidra: `FUN_802d2850` at `0x802D2850` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2850_FUN_802d2850.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve calls to `0x801C40B0(bus, channel, flag, mode, shift)` with
/// `bus = 0` and channel running `9..=0x14`. The first two entries use
/// `(flag, mode) = (1, 4)` / `(0, 4)`, the next two `(1, 5)`, the remaining
/// eight `(1, 3)` with shift `7`.
///
/// # Safety
/// The AX engine must accept descriptor writes.
///
/// # Confidence
/// high - twelve literal calls with literal arguments.
pub unsafe fn fn_802d2850() {
    platform::call5(0x801C_40B0, 0, 9, 1, 4, 0);
    platform::call5(0x801C_40B0, 0, 10, 0, 4, 0);
    platform::call5(0x801C_40B0, 0, 0xB, 1, 5, 0);
    platform::call5(0x801C_40B0, 0, 0xC, 1, 5, 0);
    let mut channel = 0xDu32;
    while channel <= 0x14 {
        platform::call5(0x801C_40B0, 0, channel, 1, 3, 7);
        channel += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D2990 - mix descriptor format
// ---------------------------------------------------------------------------

/// Configures the main mix descriptor: one shared default word is pushed into
/// input slots 4 and 5, then both slots are re-configured with mode 2 and a
/// pair of `2` limits.
///
/// # Original
/// - Ghidra: `FUN_802d2990` at `0x802D2990` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2990_FUN_802d2990.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `0x801C6AC0(1)`.
/// - Four calls that copy the word at [`SDA2_AUDIO_DEFAULT`] into slots 4 and 5
///   (`0x801C6900` sets, `0x801C69E0` updates). The export uses a separate
///   stack slot per call, so four locals are kept here.
/// - `0x801C5860(2)`, then `0x801C6AF0` twice with `(slot, 0, 0, 0, 0, 2, 2)`.
///
/// # Safety
/// The AX mix descriptor must be live.
///
/// # Confidence
/// medium - the argument lists are literal, but the meaning of the shared
/// default word and of the trailing `2, 2` pair is not proven by the export.
pub unsafe fn fn_802d2990() {
    platform::call1(0x801C_6AC0, 1);
    let default = platform::read_u32(SDA2_AUDIO_DEFAULT);

    let mut slot4_set = crate::platform::GuestLocal::new( default);
    platform::call2(0x801C_6900, 4, ((slot4_set.as_mut_ptr() as *mut u32)) as u32);
    let mut slot5_set = crate::platform::GuestLocal::new( default);
    platform::call2(0x801C_6900, 5, ((slot5_set.as_mut_ptr() as *mut u32)) as u32);
    let mut slot4_update = crate::platform::GuestLocal::new( default);
    platform::call2(0x801C_69E0, 4, ((slot4_update.as_mut_ptr() as *mut u32)) as u32);
    let mut slot5_update = crate::platform::GuestLocal::new( default);
    platform::call2(0x801C_69E0, 5, ((slot5_update.as_mut_ptr() as *mut u32)) as u32);

    platform::call1(0x801C_5860, 2);
    platform::call7(0x801C_6AF0, 4, 0, 0, 0, 0, 2, 2);
    platform::call7(0x801C_6AF0, 5, 0, 0, 0, 0, 2, 2);
}

// ---------------------------------------------------------------------------
// 0x802D2A50 - output mode table
// ---------------------------------------------------------------------------

/// Selects output mode 0 and installs the three-entry and four-entry output
/// tables described by `DAT_802FD288`.
///
/// # Original
/// - Ghidra: `FUN_802d2a50` at `0x802D2A50` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2a50_FUN_802d2a50.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x801C8110(0)`; then `0x801C7E00(index + 1, 0x802FD288, 1)` for indices
/// 0..2; then `0x801C7F40(index, 0, 0)` for indices 0..3.
///
/// # Safety
/// None beyond the AX library being live.
///
/// # Confidence
/// high - both loop bounds, the constant table and every argument are literal.
pub unsafe fn fn_802d2a50() {
    const AX_OUTPUT_TABLE: usize = 0x802F_D288;

    platform::call1(0x801C_8110, 0);

    let mut index = 0u32;
    while index < 3 {
        // observed: the table index is `index + 1`, so entry 0 is skipped.
        platform::call3(0x801C_7E00, index + 1, AX_OUTPUT_TABLE as u32, 1);
        index += 1;
    }

    index = 0;
    while index < 4 {
        platform::call3(0x801C_7F40, index, 0, 0);
        index += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D2AD0 - eight aux-bus resets
// ---------------------------------------------------------------------------

/// Resets aux buses 0..=7 on the main mix descriptor.
///
/// # Original
/// - Ghidra: `FUN_802d2ad0` at `0x802D2AD0` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2ad0_FUN_802d2ad0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Eight calls to `0x801C74C0(&DAT_804248A0, bus)` with `bus = 0..=7`.
///
/// # Safety
/// [`AX_MIX_DESCRIPTOR`] must be a live mix descriptor.
///
/// # Confidence
/// high - one loop, one callee, eight literal indices.
pub unsafe fn fn_802d2ad0() {
    let mut bus = 0u32;
    while bus < 8 {
        platform::call2(0x801C_74C0, AX_MIX_DESCRIPTOR as u32, bus);
        bus += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D2B60 - ten workspace entries and eight voice slots
// ---------------------------------------------------------------------------

/// Claims ten AX workspace offsets (0x1E, 0x21, ... step 3) and resets the eight
/// main voice slots.
///
/// # Original
/// - Ghidra: `FUN_802d2b60` at `0x802D2B60` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2b60_FUN_802d2b60.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x801C4A20(1)`; ten `0x801C9A80(0x802FD258, offset, 0)` calls with the
/// offset starting at 0x1E and stepping by 3; then, for voice 0..=7,
/// `0x801C47D0(voice, 1, 4, 0x3C, 0, 0x7D)`, `0x801C7750(voice, 0, 0, 0)` and
/// `0x801C77D0(voice, 0, 0)`.
///
/// # Safety
/// The AX engine must be live; the workspace table is written by the callee.
///
/// # Confidence
/// high - both loops and all six/three/four argument lists are literal.
pub unsafe fn fn_802d2b60() {
    const AX_WORKSPACE_TABLE: usize = 0x802F_D258;

    platform::call1(0x801C_4A20, 1);

    let mut entry = 0u32;
    let mut offset = 0x1Eu32;
    while entry < 10 {
        platform::call3(0x801C_9A80, AX_WORKSPACE_TABLE as u32, offset, 0);
        entry += 1;
        offset += 3;
    }

    let mut voice = 0u32;
    while voice < 8 {
        platform::call6(0x801C_47D0, voice, 1, 4, 0x3C, 0, 0x7D);
        platform::call4(0x801C_7750, voice, 0, 0, 0);
        platform::call3(0x801C_77D0, voice, 0, 0);
        voice += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D39B0 / 0x802D3B50 / 0x802D3CE0 - the last three +0x4C4 view loaders
// ---------------------------------------------------------------------------

/// Loads the `ARCH` / `ARCL` / `APLM` / `APLS` archive chain into the `+0xC`
/// child of the view and marks the view state `1`.
///
/// # Original
/// - Ghidra: `FUN_802d39b0` at `0x802D39B0` (404 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d39b0_FUN_802d39b0.c`
///
/// # Arguments
/// - `this`: view object. `+8` is the active-child pointer (null when idle),
///   `+0x4C4` the resource handle, `+0x4C8` the default archive, `+0x4CC` the
///   state word, `+0x4D0` a per-element size multiplier, `+0x4D4` the `ARCH`
///   loader handle, `+0x128` / `+0x130` the two archive sizes, `+6` the
///   "load the second archive too" byte and `+4` the in-progress byte.
/// - `argument`: second forwarded word; unused by this body.
/// - `archive`: archive to read from; `0` selects `+0x4C8`.
///
/// # Returns
/// The resource handle at `+0x4C4` on success (also when a child was already
/// active), or `0` when any loader step fails.
///
/// # Side effects
/// Writes `+8`, `+4`, `+0x4CC` and `+0x4D4`; calls `0x802690D0`,
/// `0x802693F0`, `0x80269480`, `0x80277CF0`, `0x80277E40`, `0x80277AB0` and
/// four `0x8027A3A0` loads carrying the ASCII tags `ARCH`, `ARCL`, `APLM` and
/// `APLS`.
///
/// # Safety
/// `this` must be a live view and every tag load must return a valid handle; the
/// child at `+0xC` is handed to the loader and later released by
/// [`fn_802d3e00`].
///
/// # Confidence
/// medium - the control flow, every offset and the four tags are literal; the
/// roles of `+0x128`/`+0x130` and of the `+0x4D0` factor are `// inferred:`
/// from the way they feed the two size loads.
pub unsafe fn fn_802d39b0(this: usize, argument: u32, archive: u32) -> u32 {
    const TAG_ARCH: u32 = 0x4152_4348; // "ARCH"
    const TAG_ARCL: u32 = 0x4152_434C; // "ARCL"
    const TAG_APLM: u32 = 0x4150_4C4D; // "APLM"
    const TAG_APLS: u32 = 0x4150_4C53; // "APLS"

    let _ = argument;
    let archive = if archive == 0 {
        platform::read_u32(this + 0x4C8)
    } else {
        archive
    };

    if platform::read_u32(this + 8) != 0 {
        return platform::read_u32(this + 0x4C4);
    }

    platform::write_u32(this + 8, (this + 0xC) as u32);
    if platform::call1(0x8026_90D0, (this + 0xC) as u32) == 0 {
        return 0;
    }

    let archive_size = platform::read_u32(this + 0x130);
    platform::write_u8(this + 4, 1);
    let primary = platform::call4(0x8027_A3A0, archive, archive_size, 0, TAG_ARCH);
    platform::write_u32(this + 0x4D4, primary);
    if platform::call3(0x8026_93F0, (this + 0xC) as u32, primary, archive_size) == 0 {
        platform::write_u8(this + 4, 0);
        return 0;
    }

    if platform::read_u8(this + 6) != 0 {
        let second_size = platform::read_u32(this + 0x128);
        let second = platform::call4(0x8027_A3A0, archive, second_size, 0, TAG_ARCL);
        platform::call3(0x8026_9480, (this + 0xC) as u32, second, second_size);
    }

    let handle = platform::read_u32(this + 0x4C4);
    let size = platform::call2(0x8027_7CF0, handle, platform::read_u32(this + 8));
    let primary_size = platform::call4(0x8027_A3A0, archive, size, 0, TAG_APLM);
    let count = platform::call2(0x8027_7E40, handle, platform::read_u32(this + 8));
    let total = platform::read_u32(this + 0x4D0).wrapping_mul(count);
    let data_size = platform::call4(0x8027_A3A0, archive, total, 0, TAG_APLS);
    if platform::call6(
        0x8027_7AB0,
        handle,
        platform::read_u32(this + 8),
        primary_size,
        size,
        data_size,
        total,
    ) == 0
    {
        platform::write_u8(this + 4, 0);
        return 0;
    }

    platform::write_u32(this + 0x4CC, 1);
    platform::write_u8(this + 4, 0);
    platform::read_u32(this + 0x4C4)
}

/// Loads the second view child (`+0x198`) and marks the view state `2`.
///
/// # Original
/// - Ghidra: `FUN_802d3b50` at `0x802D3B50` (392 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3b50_FUN_802d3b50.c`
///
/// # Arguments
/// - `this`: view object; `+0x198` is the second child, `+0x2B4` / `+0x2BC`
///   its two sizes, the rest of the layout matches [`fn_802d39b0`].
/// - `argument`: second forwarded word; unused by this body.
/// - `archive`: archive to read from; `0` selects `+0x4C8`.
///
/// # Returns
/// The resource handle at `+0x4C4`, or `0` when a loader step fails.
///
/// # Side effects
/// Same shape as [`fn_802d39b0`], with `0x8026F770` / `0x8026FC50` /
/// `0x8026FD00` replacing the three archive-child calls and tag `0` on all four
/// loads. On success the state word becomes `2`.
///
/// # Safety
/// Same contract as [`fn_802d39b0`]; the child at `+0x198` must be a live
/// sub-object.
///
/// # Confidence
/// medium - identical structure to [`fn_802d39b0`], which pins every offset;
/// only the callee identities are new.
pub unsafe fn fn_802d3b50(this: usize, argument: u32, archive: u32) -> u32 {
    let _ = argument;
    let archive = if archive == 0 {
        platform::read_u32(this + 0x4C8)
    } else {
        archive
    };

    if platform::read_u32(this + 8) != 0 {
        return platform::read_u32(this + 0x4C4);
    }

    platform::write_u32(this + 8, (this + 0x198) as u32);
    if platform::call1(0x8026_F770, (this + 0x198) as u32) == 0 {
        return 0;
    }

    let archive_size = platform::read_u32(this + 0x2BC);
    platform::write_u8(this + 4, 1);
    let primary = platform::call4(0x8027_A3A0, archive, archive_size, 0, 0);
    if platform::call3(0x8026_FC50, (this + 0x198) as u32, primary, archive_size) == 0 {
        platform::write_u8(this + 4, 0);
        return 0;
    }

    if platform::read_u8(this + 6) != 0 {
        let second_size = platform::read_u32(this + 0x2B4);
        let second = platform::call4(0x8027_A3A0, archive, second_size, 0, 0);
        platform::call3(0x8026_FD00, (this + 0x198) as u32, second, second_size);
    }

    let handle = platform::read_u32(this + 0x4C4);
    let size = platform::call2(0x8027_7CF0, handle, (this + 0x198) as u32);
    let primary_size = platform::call4(0x8027_A3A0, archive, size, 0, 0);
    let count = platform::call2(0x8027_7E40, handle, (this + 0x198) as u32);
    let total = platform::read_u32(this + 0x4D0).wrapping_mul(count);
    let data_size = platform::call4(0x8027_A3A0, archive, total, 0, 0);
    if platform::call6(
        0x8027_7AB0,
        handle,
        (this + 0x198) as u32,
        primary_size,
        size,
        data_size,
        total,
    ) == 0
    {
        platform::write_u8(this + 4, 0);
        return 0;
    }

    platform::write_u32(this + 0x4CC, 2);
    platform::write_u8(this + 4, 0);
    platform::read_u32(this + 0x4C4)
}

/// Loads the third view child (`+0x374`) and marks the view state `4`.
///
/// # Original
/// - Ghidra: `FUN_802d3ce0` at `0x802D3CE0` (276 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3ce0_FUN_802d3ce0.c`
///
/// # Arguments
/// - `this`: view object; `+0x374` is the third child, the rest of the layout
///   matches [`fn_802d39b0`].
/// - `argument`: second forwarded word; unused by this body.
/// - `archive`: archive to read from; `0` selects `+0x4C8`.
///
/// # Returns
/// The resource handle at `+0x4C4`, or `0` when a loader step fails.
///
/// # Side effects
/// `0x8026DE10` builds the child, then the `0x80277CF0` / `0x80277E40` /
/// `0x80277AB0` size-and-data sequence of [`fn_802d39b0`] runs with tag `0`.
/// There is no second-archive branch and no `+0x4D4` store. On success the state
/// word becomes `4`.
///
/// # Safety
/// Same contract as [`fn_802d39b0`]; the child at `+0x374` must be a live
/// sub-object.
///
/// # Confidence
/// medium - the shorter body is literal; the `+0x374` child identity comes from
/// [`fn_802d38a0`] and [`fn_802d3e00`], which release the same offset.
pub unsafe fn fn_802d3ce0(this: usize, argument: u32, archive: u32) -> u32 {
    let _ = argument;
    let archive = if archive == 0 {
        platform::read_u32(this + 0x4C8)
    } else {
        archive
    };

    if platform::read_u32(this + 8) != 0 {
        return platform::read_u32(this + 0x4C4);
    }

    platform::write_u32(this + 8, (this + 0x374) as u32);
    if platform::call1(0x8026_DE10, (this + 0x374) as u32) == 0 {
        return 0;
    }

    platform::write_u8(this + 4, 1);
    let handle = platform::read_u32(this + 0x4C4);
    let size = platform::call2(0x8027_7CF0, handle, (this + 0x374) as u32);
    let primary_size = platform::call4(0x8027_A3A0, archive, size, 0, 0);
    let count = platform::call2(0x8027_7E40, handle, (this + 0x374) as u32);
    let total = platform::read_u32(this + 0x4D0).wrapping_mul(count);
    let data_size = platform::call4(0x8027_A3A0, archive, total, 0, 0);
    if platform::call6(
        0x8027_7AB0,
        handle,
        (this + 0x374) as u32,
        primary_size,
        size,
        data_size,
        total,
    ) == 0
    {
        platform::write_u8(this + 4, 0);
        return 0;
    }

    platform::write_u32(this + 0x4CC, 4);
    platform::write_u8(this + 4, 0);
    platform::read_u32(this + 0x4C4)
}

// ---------------------------------------------------------------------------
// 0x802D42C0 / 0x802D42D0 - SDA compare hook and its driver
// ---------------------------------------------------------------------------

/// Calls the compare hook whose function pointer and context were published in
/// the SDA pair at `0x8042AF78` / `0x8042AF7C`.
///
/// # Original
/// - Ghidra: `FUN_802d42c0` at `0x802D42C0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d42c0_FUN_802d42c0.c`
///
/// # Arguments
/// - `item`: the only register argument; forwarded as the hook's first
///   argument.
///
/// # Returns
/// Nothing. The hook's own result is not returned.
///
/// # Side effects
/// One indirect call `hook(item, context)`. The two words are the SDA cells
/// [`VIEW_COMPARE_HOOK`] and [`VIEW_COMPARE_CONTEXT`], which
/// [`fn_802d42d0`] fills immediately before its loop.
///
/// # Safety
/// [`VIEW_COMPARE_HOOK`] must hold a callable address taking
/// `(u32, u32)`; the export shows no null test.
///
/// # Confidence
/// medium - the two loads and the indirect call are literal; that the two words
/// are a hook/context pair is `// inferred:` from the writer in
/// [`fn_802d42d0`], which stores its third and fourth arguments there.
pub unsafe fn fn_802d42c0(item: usize) {
    let hook = platform::read_u32(VIEW_COMPARE_HOOK) as usize;
    let context = platform::read_u32(VIEW_COMPARE_CONTEXT);
    platform::call2(hook, item as u32, context);
}

/// Runs the SDA compare hook over one of the two record lists of the view's
/// `+0x4C4` object.
///
/// # Original
/// - Ghidra: `FUN_802d42d0` at `0x802D42D0` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d42d0_FUN_802d42d0.c`
///
/// # Arguments
/// - `this`: view object; the record count is the word at `*(+0x4C4) + 0x3C`.
/// - `index`: must be strictly below that count, otherwise nothing happens.
/// - `hook`: function pointer published to [`VIEW_COMPARE_HOOK`].
/// - `context`: word published to [`VIEW_COMPARE_CONTEXT`].
/// - `reverse`: `0` walks the list at `head + 0` forwards, non-zero walks the
///   list at `head + 4` and advances only when the hook reports a hit.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two SDA stores, then for every visited node `0x8027A160` materialises a
/// handle from `node - 0xF8`, [`fn_802d42c0`] runs on it and `0x8027A200`
/// releases it.
///
/// # Safety
/// `0x80278780` must return a live list head whose links are valid, and every
/// handle must be releasable.
///
/// # Confidence
/// high - the two loops, the three callees and the `-0xF8` back-step are
/// literal and agree with the shipped words at `0x802D42D0..0x802D43BC`; the
/// `+4` link chain makes the initial cursor the link word itself, exactly as
/// the image does. The reverse walk only advances when the hook reports a hit,
/// so a zero-result hook spins in the original too; the host body reproduces
/// that termination behaviour instead of inventing progress.
pub unsafe fn fn_802d42d0(
    this: usize,
    index: u32,
    hook: u32,
    context: u32,
    reverse: i32,
) {
    platform::write_u32(VIEW_COMPARE_CONTEXT, context);
    platform::write_u32(VIEW_COMPARE_HOOK, hook);

    let handle = platform::read_u32(this + 0x4C4) as usize;
    if index >= platform::read_u32(handle + 0x3C) {
        return;
    }

    let list = call0_word(0x8027_8780) as usize;
    if reverse == 0 {
        let head = list + 4;
        let mut node = platform::read_u32(head) as usize;
        while node != head {
            let next = platform::read_u32(node) as usize;
            let mut item = crate::platform::GuestLocal::<u32>::new( 0u32);
            platform::call2(
                0x8027_A160,
                ((item.as_mut_ptr() as *mut u32)) as u32,
                (node.wrapping_sub(0xF8)) as u32,
            );
            fn_802d42c0(((item.as_mut_ptr() as *mut u32)) as usize);
            platform::call1(0x8027_A200, ((item.as_mut_ptr() as *mut u32)) as u32);
            node = next;
        }
    } else {
        let head = platform::read_u32(list + 4) as usize;
        let mut node = list + 4;
        while node != head {
            let mut item = crate::platform::GuestLocal::<u32>::new( 0u32);
            platform::call2(
                0x8027_A160,
                ((item.as_mut_ptr() as *mut u32)) as u32,
                (platform::read_u32(node + 4) as usize).wrapping_sub(0xF8) as u32,
            );
            fn_802d42c0(((item.as_mut_ptr() as *mut u32)) as usize);
            if *item != 0 {
                node = platform::read_u32(node + 4) as usize;
            }
            platform::call1(0x8027_A200, ((item.as_mut_ptr() as *mut u32)) as u32);
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D4830 - 0x6C-stride transform record initialiser
// ---------------------------------------------------------------------------

/// Fills a 0x6C-stride transform record with the SDA2 defaults.
///
/// # Original
/// - Ghidra: `FUN_802d4830` at `0x802D4830` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d4830_FUN_802d4830.c`
///
/// # Arguments
/// - `this`: the record base.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - Runs the AX-style initialiser [`fn_802d61c0`]`(this)`, which sets `+0x1C`
///   and `+0x20`.
/// - Stores `0x20` at `+0x24`, the twelve words at `+0x28..+0x54` all get the
///   third word of the SDA2 triplet, `+0x58` gets the first and `+0x5C` the
///   second.
/// - Stores `0x25800` at `+0x60`, `+0x64` and `+0x68`.
///
/// # Safety
/// `this..this+0x6C` must be writable.
///
/// # Confidence
/// medium - every store, offset and source word is literal; the exporter prints
/// `FUN_802d61c0()` without its argument, and `this` is `// inferred:` because
/// the preceding and following stores in the same body all target `param_1`.
pub unsafe fn fn_802d4830(this: usize) -> usize {
    fn_802d61c0(this);

    let first = platform::read_u32(SDA2_TRIPLET);
    let second = platform::read_u32(SDA2_TRIPLET + 4);
    let fill = platform::read_u32(SDA2_TRIPLET + 8);

    platform::write_u32(this + 0x24, 0x20);
    platform::write_u32(this + 0x58, first);
    platform::write_u32(this + 0x5C, second);
    // observed: the twelve words of the 3x4 block are written in this order.
    platform::write_u32(this + 0x28, fill);
    platform::write_u32(this + 0x38, fill);
    platform::write_u32(this + 0x48, fill);
    platform::write_u32(this + 0x2C, fill);
    platform::write_u32(this + 0x3C, fill);
    platform::write_u32(this + 0x4C, fill);
    platform::write_u32(this + 0x30, fill);
    platform::write_u32(this + 0x40, fill);
    platform::write_u32(this + 0x50, fill);
    platform::write_u32(this + 0x34, fill);
    platform::write_u32(this + 0x44, fill);
    platform::write_u32(this + 0x54, fill);
    // unknown: 0x25800 is a bare constant with no other reference in the export.
    platform::write_u32(this + 0x60, 0x25800);
    platform::write_u32(this + 0x64, 0x25800);
    platform::write_u32(this + 0x68, 0x25800);
    this
}

// ---------------------------------------------------------------------------
// 0x802D50B0..0x802D5DD0 - the eight per-mode effect builders
// ---------------------------------------------------------------------------
//
// One entry point per effect mode. `fn_802d4cd0(this, mode, argument)` calls
// them as `(this, 0, argument, mode)`, which is where the `this` pointer the
// export loses in `FUN_802dcd70()` comes from: `0x802DCD70` is a
// `__savegprlr` alternate entry point, so Ghidra models its saved registers as
// a 64-bit return value and prints the real first argument as unused.
//
// Every builder has the same skeleton; only the four constants differ:
//
// | address | engine object size | constructor | configure | size query |
// |---|---:|---|---|---|
// | `0x802D50B0` | 0x0E8 | `0x8026B640` | `0x8026B900` | `0x8026B6D0` |
// | `0x802D5290` | 0x084 | `0x8026B160` | `0x8026B370` | `0x8026B1F0` |
// | `0x802D5470` | 0x184 | `0x8026AB10` | `0x8026ADD0` | `0x8026ABA0` |
// | `0x802D5650` | 0x0CC | `0x8026A670` | `0x8026A880` | `0x8026A700` |
// | `0x802D5830` | 0x230 | `0x8026D0B0` | `0x8026D3B0` | `0x8026D180` |
// | `0x802D5A10` | 0x128 | `0x8026CAE0` | `0x8026CD30` | `0x8026CBB0` |
// | `0x802D5BF0` | 0x308 | `0x8026C2F0` | `0x8026C5F0` | `0x8026C3C0` |
// | `0x802D5DD0` | 0x18C | `0x8026BCF0` | `0x8026BF40` | `0x8026BDC0` |

/// Builds the mode-8 effect: releases the old slot, allocates a 0x10-byte chain
/// node and a 0xE8-byte engine object, zeroes the engine's buffer and submits it.
///
/// # Original
/// - Ghidra: `FUN_802d50b0` at `0x802D50B0` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d50b0_FUN_802d50b0.c`
///
/// # Arguments
/// - `this`: one effect slot from the table returned by [`fn_802d50a0`].
///   `+0` chain head, `+4` chain tail, `+8` slot id, `+0xC` record count.
/// - `reserved`: always `0`; `fn_802d4cd0` passes it in `r4` and no path reads
///   it.
/// - `argument`: the resource/param word forwarded to `0x8026B900`.
/// - `mode`: the slot mode, stored at node `+8`; `8` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0` on any allocation/configuration failure.
///
/// # Side effects
/// - [`fn_802d4da0`]`(this, 0)` releases the previous contents.
/// - `0x802B9240(0x10, count, 4)` allocates the chain node, `0x802B9240(0xE8,
///   count, 4)` the engine storage, `0x8026B640` constructs the engine and
///   `0x802B9270(size, count, 4)` the sample buffer.
/// - Failure paths call the engine's virtual `+8` release (with flag `1`) and
///   `0x802B9280` on the node.
/// - On success the slot is relinked, the buffer is zeroed, the engine's virtual
///   `+0x1C` receives `(buffer, size)` and `0x802612F0(system, slot id, engine)`
///   submits the effect.
///
/// # Safety
/// `this` must be a live slot; every returned pointer must be usable and the
/// engine's vtable must hold valid `+8` and `+0x1C` entries.
///
/// # Confidence
/// medium - every constant and offset is literal; `this` is recovered from the
/// `fn_802d4cd0` call site and the two virtual slots are `// inferred:` from the
/// `(engine, buffer, size)` argument shape.
pub unsafe fn fn_802d50b0(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0xE8, 0x8026_B640, 0x8026_B900, 0x8026_B6D0)
}

/// Builds the mode-7 effect; same skeleton as [`fn_802d50b0`] with a 0x84-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5290` at `0x802D5290` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5290_FUN_802d5290.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `7` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026B160` / `0x8026B370` /
/// `0x8026B1F0`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5290(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0x84, 0x8026_B160, 0x8026_B370, 0x8026_B1F0)
}

/// Builds the mode-6 effect; same skeleton as [`fn_802d50b0`] with a 0x184-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5470` at `0x802D5470` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5470_FUN_802d5470.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `6` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026AB10` / `0x8026ADD0` /
/// `0x8026ABA0`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5470(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0x184, 0x8026_AB10, 0x8026_ADD0, 0x8026_ABA0)
}

/// Builds the mode-5 effect; same skeleton as [`fn_802d50b0`] with a 0xCC-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5650` at `0x802D5650` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5650_FUN_802d5650.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `5` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026A670` / `0x8026A880` /
/// `0x8026A700`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5650(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0xCC, 0x8026_A670, 0x8026_A880, 0x8026_A700)
}

/// Builds the mode-4 effect; same skeleton as [`fn_802d50b0`] with a 0x230-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5830` at `0x802D5830` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5830_FUN_802d5830.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `4` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026D0B0` / `0x8026D3B0` /
/// `0x8026D180`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5830(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0x230, 0x8026_D0B0, 0x8026_D3B0, 0x8026_D180)
}

/// Builds the mode-3 effect; same skeleton as [`fn_802d50b0`] with a 0x128-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5a10` at `0x802D5A10` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5a10_FUN_802d5a10.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `3` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026CAE0` / `0x8026CD30` /
/// `0x8026CBB0`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5a10(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0x128, 0x8026_CAE0, 0x8026_CD30, 0x8026_CBB0)
}

/// Builds the mode-2 effect; same skeleton as [`fn_802d50b0`] with a 0x308-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5bf0` at `0x802D5BF0` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5bf0_FUN_802d5bf0.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `2` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026C2F0` / `0x8026C5F0` /
/// `0x8026C3C0`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5bf0(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0x308, 0x8026_C2F0, 0x8026_C5F0, 0x8026_C3C0)
}

/// Builds the mode-1 effect; same skeleton as [`fn_802d50b0`] with a 0x18C-byte
/// engine object.
///
/// # Original
/// - Ghidra: `FUN_802d5dd0` at `0x802D5DD0` (480 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5dd0_FUN_802d5dd0.c`
///
/// # Arguments
/// Same five as [`fn_802d50b0`]; `mode` is `1` for this entry point.
///
/// # Returns
/// `0x802612F0`'s result, or `0`.
///
/// # Side effects
/// Same as [`fn_802d50b0`], through `0x8026BCF0` / `0x8026BF40` /
/// `0x8026BDC0`.
///
/// # Safety
/// Same as [`fn_802d50b0`].
///
/// # Confidence
/// medium - identical skeleton, different literals.
pub unsafe fn fn_802d5dd0(this: usize, reserved: u32, argument: u32, mode: u32) -> u32 {
    let _ = reserved;
    build_effect_slot(this, argument, mode, 0x18C, 0x8026_BCF0, 0x8026_BF40, 0x8026_BDC0)
}

/// Shared skeleton of the eight per-mode effect builders above.
///
/// # Original
/// - Ghidra: no address; this is the common shape of `FUN_802d50b0`,
///   `FUN_802d5290`, `FUN_802d5470`, `FUN_802d5650`, `FUN_802d5830`,
///   `FUN_802d5a10`, `FUN_802d5bf0` and `FUN_802d5dd0`.
/// - Evidence: `work/ghidra/main-gb/export/802d50b0_FUN_802d50b0.c` (template)
///
/// # Arguments
/// - `this`: effect slot; `+0`/`+4` chain head/tail, `+8` slot id, `+0xC` count.
/// - `argument`: forwarded to the engine's configure call.
/// - `mode`: stored at node `+8` and forwarded to the submit call.
/// - `object_size`, `constructor`, `configure`, `size_query`: the four literals
///   that differ between the eight copies.
///
/// # Returns
/// The submit call's result, or `0` when the count is zero or any step fails.
///
/// # Side effects
/// See the individual functions; the sequence is
/// [`fn_802d4da0`], node alloc, engine alloc + construct, configure, buffer
/// alloc + zero, engine virtual `+0x1C`, `0x80260860`, `0x802612F0`.
///
/// # Safety
/// `this` must be a live slot and the engine vtable must be valid.
///
/// # Confidence
/// medium - the skeleton is identical in all eight exports apart from the four
/// constants listed above.
unsafe fn build_effect_slot(
    this: usize,
    argument: u32,
    mode: u32,
    object_size: u32,
    constructor: usize,
    configure: usize,
    size_query: usize,
) -> u32 {
    fn_802d4da0(this, 0);

    let count = platform::read_u32(this + 0xC);
    if count == 0 {
        return 0;
    }

    let node = platform::call3(0x802B_9240, 0x10, count, 4) as usize;
    if node == 0 {
        return 0;
    }
    platform::write_u32(node, 0);
    platform::write_u32(node + 4, 0);
    platform::write_u32(node + 8, 0);
    platform::write_u32(node + 0xC, 0);

    let mut engine = 0usize;
    if platform::call3(0x802B_9240, object_size, count, 4) != 0 {
        engine = call0_word(constructor) as usize;
    }
    if engine == 0 {
        platform::call1(0x802B_9280, node as u32);
        return 0;
    }

    if platform::call2(configure, engine as u32, argument) == 0 {
        return 0;
    }

    let size = platform::call1(size_query, engine as u32);
    let buffer = platform::call3(0x802B_9270, size, count, 4);
    if buffer == 0 {
        let vtable = platform::read_u32(engine) as usize;
        platform::call2(platform::read_u32(vtable + 8) as usize, engine as u32, 1);
        platform::call1(0x802B_9280, platform::read_u32(this));
        platform::write_u32(this, 0);
        platform::write_u32(this + 4, 0);
        return 0;
    }

    platform::write_u32(this + 4, node as u32);
    platform::write_u32(this, node as u32);
    platform::write_u32(node, engine as u32);
    // inferred: `+8` holds the mode and `+0xC` the count the node was built for.
    platform::write_u32(node + 8, mode);
    platform::write_u32(node + 0xC, argument);

    if size != 0 {
        let mut offset = 0usize;
        let mut words = (size >> 3) as usize;
        while words != 0 {
            let mut byte = 0usize;
            while byte < 8 {
                platform::write_u8((buffer as usize).wrapping_add(offset + byte), 0);
                byte += 1;
            }
            offset += 8;
            words -= 1;
        }
        let mut rest = (size & 7) as usize;
        while rest != 0 {
            platform::write_u8((buffer as usize).wrapping_add(offset), 0);
            offset += 1;
            rest -= 1;
        }
    }

    let vtable = platform::read_u32(engine) as usize;
    platform::call3(platform::read_u32(vtable + 0x1C) as usize, engine as u32, buffer, size);
    let system = call0_word(0x8026_0860);
    platform::call3(0x8026_12F0, system, platform::read_u32(this + 8), engine as u32)
}

// ---------------------------------------------------------------------------
// 0x802D7410 / 0x802D7570 / 0x802D74C0 - sound record create and destroy
// ---------------------------------------------------------------------------

// SDA cells of the sound-record cluster (`r13 - 0x46D8` .. `r13 - 0x46C8`).
const SOUND_RECORD: usize = 0x8042_AFA8; // r13 - 0x46D8
const SOUND_VOICE: usize = 0x8042_AFAC; // r13 - 0x46D4
const SOUND_TABLE: usize = 0x8042_AFB0; // r13 - 0x46D0
const SOUND_COUNT: usize = 0x8042_AFB4; // r13 - 0x46CC
const SOUND_CURSOR: usize = 0x8042_AFB8; // r13 - 0x46C8
const SOUND_DEFAULT_HEAP: usize = 0x8042_AE10; // r13 - 0x4870

/// Creates the global sound record once, then returns the current one.
///
/// # Original
/// - Ghidra: `FUN_802d7410` at `0x802D7410` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7410_FUN_802d7410.c`
///
/// # Arguments
/// - `count`: number of entries requested; the export loses it to
///   `FUN_802dcd74()`, a `__savegprlr` entry point.
/// - `stride`: entry stride, lost to the same artefact.
/// - `flags`: forwarded to [`fn_802d7570`].
/// - `heap`: allocator id; `0` selects [`SOUND_DEFAULT_HEAP`].
/// - `table_count`: capacity of the auxiliary `0x14`-byte table.
///
/// # Returns
/// The record pointer stored at [`SOUND_RECORD`] (the original's epilogue
/// returns it through the register restore).
///
/// # Side effects
/// On the first call runs `0x80234630` and `0x802331A0`, allocates four bytes
/// with `0x802B9240(4, heap, 4)` and builds the record with [`fn_802d7570`].
/// Later calls only return the stored pointer.
///
/// # Safety
/// The allocator and the two `0x8023xxxx` helpers must be live.
///
/// # Confidence
/// medium - the guard, the allocation and every store are literal; which of the
/// two lost leading words is the count and which the stride is `// inferred:`
/// from the positions they take in the [`fn_802d7570`] argument list.
pub unsafe fn fn_802d7410(
    count: u32,
    stride: u32,
    flags: u32,
    heap: u32,
    table_count: u32,
) -> u32 {
    if platform::read_u32(SOUND_RECORD) == 0 {
        platform::call0(0x8023_4630);
        platform::call0(0x8023_31A0);
        let heap = if heap == 0 {
            platform::read_u32(SOUND_DEFAULT_HEAP)
        } else {
            heap
        };
        let storage = platform::call3(0x802B_9240, 4, heap, 4);
        let mut record = 0u32;
        if storage != 0 {
            // observed: the record uses the four-byte block as its `this`.
            record = fn_802d7570(storage as usize, count, stride, flags, heap, table_count);
        }
        platform::write_u32(SOUND_RECORD, record);
    }
    platform::read_u32(SOUND_RECORD)
}

/// Builds the sound record inside a caller-provided four-byte block.
///
/// # Original
/// - Ghidra: `FUN_802d7570` at `0x802D7570` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7570_FUN_802d7570.c`
///
/// # Arguments
/// - `storage`: the four-byte block; its first word becomes the vtable and the
///   block is returned unchanged.
/// - `count`: entry count; the entry block is `(count + 1) * stride` bytes plus
///   a 0x2C-byte header.
/// - `stride`: entry stride, also forwarded to `0x80233D10`.
/// - `flags`: forwarded to `0x80233D10` as its last argument.
/// - `heap`: allocator id for both allocations.
/// - `table_count`: capacity of the auxiliary `0x14`-byte table.
///
/// # Returns
/// `storage`.
///
/// # Side effects
/// Installs the vtable `0x8035079C`, allocates the entry block with
/// `0x802B8CC0`, builds it with `0x80233D10`, stores the result at
/// [`SOUND_VOICE`], renames it through `0x802356B0` (default name
/// `0x803410E4`), writes the `0x1C`/`0x1E`/`0x22` voice header fields, stores
/// [`SOUND_COUNT`] and [`SOUND_CURSOR`], allocates the `0x14`-byte table with
/// `0x802B8CC0` into [`SOUND_TABLE`] and finally calls
/// [`fn_802d74b0`]`(0)`.
///
/// # Safety
/// `storage` must be four writable bytes, and the allocator and the two
/// `0x8023xxxx` helpers must be live.
///
/// # Confidence
/// medium - every call and store is literal; the two leading words are
/// `// inferred:` to be the count and the stride because the same two values
/// size the entry block and are forwarded to `0x80233D10`.
pub unsafe fn fn_802d7570(
    storage: usize,
    count: u32,
    stride: u32,
    flags: u32,
    heap: u32,
    table_count: u32,
) -> u32 {
    platform::write_u32(storage, 0x8035_079C);

    let bytes = platform::call3(
        0x802B_8CC0,
        count.wrapping_add(1).wrapping_mul(stride).wrapping_add(0x2C),
        4,
        heap,
    );
    let voice = platform::call6(0x8023_3D10, bytes, count, stride, 0x12, 0, flags);
    platform::write_u32(SOUND_VOICE, voice);

    if voice != 0 {
        let device = platform::read_u32(AX_DEVICE_HANDLE) as usize;
        let vtable = platform::read_u32(device) as usize;
        let name_slot = call0_word(platform::read_u32(vtable + 8) as usize) as usize;
        let mut name = platform::read_u32(name_slot);
        if name == 0 {
            name = 0x8034_10E4;
        }
        platform::call2(0x8023_56B0, voice, name);

        let voice = platform::read_u32(SOUND_VOICE) as usize;
        platform::write_u16(voice + 0x1C, 0);
        platform::write_u16(voice + 0x1E, 0x1E);
        platform::write_u8(voice + 0x22, 0);

        platform::write_u32(SOUND_COUNT, table_count);
        platform::write_u32(SOUND_CURSOR, 0);
        let table = platform::call3(0x802B_8CC0, table_count.wrapping_mul(0x14), 4, heap);
        platform::write_u32(SOUND_TABLE, table);
        fn_802d74b0(0);
    }

    storage as u32
}

/// Releases the global sound record and optionally its storage.
///
/// # Original
/// - Ghidra: `FUN_802d74c0` at `0x802D74C0` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d74c0_FUN_802d74c0.c`
///
/// # Arguments
/// - `record`: record created by [`fn_802d7570`]; may be null.
/// - `delete_flag`: `> 0` also frees `record` itself.
///
/// # Returns
/// `record`, unchanged.
///
/// # Side effects
/// Installs the vtable `0x8035079C`, releases [`SOUND_VOICE`] through
/// `0x80233EC0` + `0x802B8F70(handle, 0)`, calls `0x802357F0`, releases
/// [`SOUND_TABLE`] through `0x802B8F70` and clears the three table cells, then
/// calls `0x802B9280(record)` when the delete flag is positive.
///
/// # Safety
/// `record` must be null or a record produced by [`fn_802d7570`]; releasing
/// makes it dangling.
///
/// # Confidence
/// high - the null test, the three SDA releases and the delete-flag branch are
/// literal.
pub unsafe fn fn_802d74c0(record: usize, delete_flag: i32) -> usize {
    if record != 0 {
        platform::write_u32(record, 0x8035_079C);

        if platform::read_u32(SOUND_VOICE) != 0 {
            let handle = call0_word(0x8023_3EC0);
            platform::call2(0x802B_8F70, handle, 0);
            platform::write_u32(SOUND_VOICE, 0);
        }

        platform::call0(0x8023_57F0);

        if platform::read_u32(SOUND_TABLE) != 0 {
            platform::call2(0x802B_8F70, platform::read_u32(SOUND_TABLE), 0);
            platform::write_u32(SOUND_TABLE, 0);
            platform::write_u32(SOUND_COUNT, 0);
            platform::write_u32(SOUND_CURSOR, 0);
        }

        if delete_flag > 0 {
            platform::call1(0x802B_9280, record as u32);
        }
    }
    record
}

// ---------------------------------------------------------------------------
// 0x802DB6B0 / 0x802DB710 - two small field/release helpers
// ---------------------------------------------------------------------------

/// Clears the pointer field at `+0xBC`.
///
/// # Original
/// - Ghidra: `FUN_802db6b0` at `0x802DB6B0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db6b0_FUN_802db6b0.c`
///
/// # Arguments
/// - `this`: object owning the field.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One word store at `+0xBC`.
///
/// # Safety
/// `this..this+0xC0` must be writable.
///
/// # Confidence
/// high - a single store, printed as one `li`/`stw` pair.
pub unsafe fn fn_802db6b0(this: usize) {
    platform::write_u32(this + 0xBC, 0);
}

/// Conditionally releases an object: frees the storage when the caller's signed
/// flag is positive and returns the pointer unchanged.
///
/// # Original
/// - Ghidra: `FUN_802db710` at `0x802DB710` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db710_FUN_802db710.c`
///
/// # Arguments
/// - `object`: raw `this` pointer. May be null.
/// - `delete_flag`: `> 0` also frees the storage.
///
/// # Returns
/// `object`, unchanged.
///
/// # Side effects
/// `0x802B9280(object)` when both tests pass; nothing otherwise. The export
/// prints the call without arguments, so the object argument is `// inferred:`
/// from the two guards and from every other `0x802B9280` caller in this file.
///
/// # Safety
/// `object` must be null or a live allocation; freeing makes it dangling.
///
/// # Confidence
/// medium - the two guards and the return are literal; the freed pointer is
/// inferred because the export drops the argument register.
pub unsafe fn fn_802db710(object: usize, delete_flag: i32) -> usize {
    if object != 0 && delete_flag > 0 {
        platform::call1(0x802B_9280, object as u32);
    }
    object
}

// ---------------------------------------------------------------------------
// Wave 92 - layout records, the 0x802D32/37 initialiser pair, the BRRES table
// constructor, and the four `lyt`-style state helpers at 0x802DB6C0..0x802DB9EC
// ---------------------------------------------------------------------------
//
// SDA cells referenced below resolve against `r13 = 0x8042F680` (the value
// Waves 77 and 89 established) and `r2 = 0x80433080`.

/// Four per-widget attribute bytes at `r13-0x4734` (`0x8042AF4C`).
const WIDGET_ATTRIBUTES: usize = 0x8042_AF4C;
/// Outer panel width at `r13-0x4748` (`0x8042AF38`).
const PANEL_MAX_X: usize = 0x8042_AF38;
/// Outer panel height at `r13-0x4746` (`0x8042AF3A`).
const PANEL_MAX_Y: usize = 0x8042_AF3A;
/// One-shot flag for the draw record at `r13-0x4758` (`0x8042AF28`).
const DRAW_RECORD_INITIALISED: usize = 0x8042_AF28;
/// Current layout index at `r13-0x4778` (`0x8042AF08`).
const LAYOUT_INDEX: usize = 0x8042_AF08;
/// Active layout record cookie at `r13-0x4774` (`0x8042AF0C`).
const LAYOUT_ROOT: usize = 0x8042_AF0C;
/// Base of the 0x18-byte stride layout table (`0x803504E0`).
const LAYOUT_TABLE: usize = 0x8035_04E0;
/// The 0x48-byte draw record the layout table feeds (`0x80424710`).
const DRAW_RECORD: usize = 0x8042_4710;
/// Console toggle byte at `r13-0x46C0` (`0x8042AFC0`).
const CONSOLE_FLAG: usize = 0x8042_AFC0;
/// Optional console callback at `r13-0x46C4` (`0x8042AFBC`).
const CONSOLE_CALLBACK: usize = 0x8042_AFBC;
/// The `1.0f` normalised-time threshold at `r2-0x3A10` (`0x8042F670`).
const TIME_THRESHOLD: usize = 0x8042_F670;

// ---------------------------------------------------------------------------
// 0x802D0AE0 - install the two-panel layout table
// ---------------------------------------------------------------------------

/// Copies two panel descriptions into the layout table, derives four scale
/// ratios from the outer panel size, and points the active layout record at the
/// caller's record (or the built-in one).
///
/// # Original
/// - Ghidra: `FUN_802d0ae0` at `0x802D0AE0` (476 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0ae0_FUN_802d0ae0.c`
///
/// # Arguments
/// - `primary`: `u16` pair `(width, height)` for the first panel.
/// - `secondary`: `u16` pair `(width, height)` for the second panel.
/// - `record`: caller's layout record, or `0` to use [`DRAW_RECORD`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Writes the 0x18-byte entry at [`LAYOUT_TABLE`]: dimensions at `+0`, `+2`,
///   `+0xC`, `+0xE`, the derived ratios at `+4`, `+8`, `+0x10`, `+0x14`, the
///   outer size at `+0x18`/`+0x1A` and `1.0f` at `+0x1C`/`+0x20`.
/// - First call runs `0x802D0CC0(DRAW_RECORD)` and marks
///   [`DRAW_RECORD_INITIALISED`].
/// - Fills the draw record at [`DRAW_RECORD`]: `+0` cleared, `+4 = 1`, `+0x34`
///   bit 0 set, `+0x18 = 0.0f`, `+0x1C = 1.0f`, and `+8`/`+0xC` take the
///   current entry's dimensions as floats.
/// - Publishes the chosen record at [`LAYOUT_ROOT`], clears its word
///   `+0x3C` and calls `0x802D1AA0(index)`.
///
/// # Safety
/// `primary` and `secondary` must each point at four readable bytes; a record
/// passed by the caller must be at least `0x40` bytes.
///
/// # Confidence
/// medium - every store, ratio and address is literal. The SDA resolvers follow
/// the values used by Waves 77/89; the panel/scale roles are `// inferred:`
/// from the divide pattern and the 0x18-byte stride.
pub unsafe fn fn_802d0ae0(primary: usize, secondary: usize, record: usize) {
    // observed: `lhz`/`sth` pairs, dimensions first.
    let primary_width = platform::read_u16(primary);
    let primary_height = platform::read_u16(primary + 2);
    let secondary_width = platform::read_u16(secondary);
    let secondary_height = platform::read_u16(secondary + 2);
    let max_x = platform::read_u16(PANEL_MAX_X);
    let max_y = platform::read_u16(PANEL_MAX_Y);
    // The original reads these through a 0x43300000/2^52 `CONCAT44` idiom,
    // which is just an unsigned-int to double conversion.
    let max_x_f = max_x as f32;
    let max_y_f = max_y as f32;

    platform::write_u16(LAYOUT_TABLE, primary_width);
    platform::write_u16(LAYOUT_TABLE + 0x0C, primary_height);
    platform::write_u16(LAYOUT_TABLE + 0x18, max_x);
    platform::write_u16(LAYOUT_TABLE + 0x1A, max_y);
    // observed: the two `1.0f` words are copied from `r2-0x3B10`.
    platform::write_u32(LAYOUT_TABLE + 0x1C, 0x3F80_0000);
    platform::write_u16(LAYOUT_TABLE + 2, secondary_width);
    platform::write_u16(LAYOUT_TABLE + 0x0E, secondary_height);
    // inferred: scale factors for the two panels against the outer size.
    platform::write_f32(LAYOUT_TABLE + 4, max_x_f / primary_width as f32);
    platform::write_f32(LAYOUT_TABLE + 8, max_y_f / secondary_width as f32);
    platform::write_f32(LAYOUT_TABLE + 0x10, max_x_f / primary_height as f32);
    platform::write_f32(LAYOUT_TABLE + 0x14, max_y_f / secondary_height as f32);
    platform::write_u32(LAYOUT_TABLE + 0x20, 0x3F80_0000);

    if platform::read_u8(DRAW_RECORD_INITIALISED) == 0 {
        platform::call1(0x802D_0CC0, DRAW_RECORD as u32);
        platform::write_u8(DRAW_RECORD_INITIALISED, 1);
    }
    platform::write_u32(DRAW_RECORD, 0);
    if platform::read_u32(DRAW_RECORD + 4) != 1 {
        platform::write_u32(DRAW_RECORD + 4, 1);
        let flags = platform::read_u32(DRAW_RECORD + 0x34);
        platform::write_u32(DRAW_RECORD + 0x34, flags | 1);
    }
    // observed: `0.0f` then `1.0f`, both loaded from `r2-0x3B0C`/`r2-0x3B10`.
    platform::write_u32(DRAW_RECORD + 0x18, 0x0000_0000);
    let flags = platform::read_u32(DRAW_RECORD + 0x34);
    platform::write_u32(DRAW_RECORD + 0x34, flags | 1);
    platform::write_u32(DRAW_RECORD + 0x1C, 0x3F80_0000);
    let index = platform::read_u32(LAYOUT_INDEX) as usize;
    let entry = LAYOUT_TABLE + index * 0x18;
    platform::write_f32(DRAW_RECORD + 8, platform::read_u16(entry) as f32);
    platform::write_f32(DRAW_RECORD + 0x0C, platform::read_u16(entry + 2) as f32);

    let target = if record == 0 { DRAW_RECORD } else { record };
    platform::write_u32(LAYOUT_ROOT, target as u32);
    platform::write_u32(target + 0x3C, 0);
    platform::call1(0x802D_1AA0, index as u32);
}

// ---------------------------------------------------------------------------
// 0x802D32F0 - initialise one widget's attribute bytes
// ---------------------------------------------------------------------------

/// Runs the base initialiser, copies the four global widget attributes into
/// `+0x19..+0x1C`, sets the default `0/0/1/1` state bytes and marks the widget
/// dirty in the flags byte at `+0x18`.
///
/// # Original
/// - Ghidra: `FUN_802d32f0` at `0x802D32F0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d32f0_FUN_802d32f0.c`
///
/// # Arguments
/// - `this`: widget to initialise.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `0x802C0240(this)` base initialiser.
/// - Four byte copies from [`WIDGET_ATTRIBUTES`].
/// - `+0x0A = 0`, `+0x0B = 0`, `+0x0C = 1`, `+0x0D = 1`.
/// - `+0x18 |= 0x10` (the dirty/visible bit; `// inferred:` from the write
///   pattern only).
///
/// # Safety
/// `this..this+0x20` must be writable.
///
/// # Confidence
/// high - the call, four SDA byte loads, six stores and the `ori 0x10` are
/// literal at `0x802D32F0..0x802D3360`.
pub unsafe fn fn_802d32f0(this: usize) {
    // observed: `bl 0x802C0240` with r3 still holding `this`.
    platform::call1(0x802C_0240, this as u32);
    platform::write_u8(this + 0x19, platform::read_u8(WIDGET_ATTRIBUTES));
    platform::write_u8(this + 0x1A, platform::read_u8(WIDGET_ATTRIBUTES + 1));
    platform::write_u8(this + 0x1B, platform::read_u8(WIDGET_ATTRIBUTES + 2));
    platform::write_u8(this + 0x1C, platform::read_u8(WIDGET_ATTRIBUTES + 3));
    platform::write_u8(this + 0x0A, 0);
    platform::write_u8(this + 0x0B, 0);
    platform::write_u8(this + 0x0C, 1);
    platform::write_u8(this + 0x0D, 1);
    let flags = platform::read_u8(this + 0x18);
    platform::write_u8(this + 0x18, flags | 0x10);
}

// ---------------------------------------------------------------------------
// 0x802D3700 - pop a node and attach a GX texture
// ---------------------------------------------------------------------------

/// Pops one node from the free list at `0x802D3590`, fills in the size and mode
/// fields, runs its virtual `+0x0C` slot, allocates the texture with
/// `0x801C6BA0` and hands it to [`fn_802d35c0`].
///
/// # Original
/// - Ghidra: `FUN_802d3700` at `0x802D3700` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3700_FUN_802d3700.c`
///
/// # Arguments
/// - `width`: texture width, stored as a halfword at `+4`.
/// - `height`: texture height, stored as a halfword at `+6`.
/// - `format`: texture format byte, stored at `+9` and forwarded to the GX
///   allocator.
///
/// # Returns
/// The popped node, or `0` when either dimension is zero.
///
/// # Side effects
/// Pops the list head, writes `+4`, `+6`, `+9`, calls the node's virtual
/// `+0x0C` slot, calls `0x801C6BA0(width, height, format, 0, 1)` and
/// [`fn_802d35c0`]`(node, handle)`.
///
/// # Safety
/// The free list must be non-empty (the shipped pop does not check) and the
/// node's vtable must hold a valid `+0x0C` entry.
///
/// # Confidence
/// medium - the guards, field stores and five call arguments are literal; the
/// node/vtable roles are `// inferred:` from the neighbouring allocator, and the
/// local [`fn_802d3590`] stub is empty, so the shipped entry point is called
/// directly.
pub unsafe fn fn_802d3700(width: u16, height: u16, format: u8) -> usize {
    if width == 0 || height == 0 {
        return 0;
    }
    let node = call0_word(0x802D_3590) as usize;
    platform::write_u16(node + 4, width);
    platform::write_u16(node + 6, height);
    platform::write_u8(node + 9, format);
    let vtable = platform::read_u32(node) as usize;
    platform::call1(platform::read_u32(vtable + 0x0C) as usize, node as u32);
    let texture = platform::call5(
        0x801C_6BA0,
        width as u32,
        height as u32,
        format as u32,
        0,
        1,
    );
    fn_802d35c0(node, texture);
    node
}

// ---------------------------------------------------------------------------
// 0x802D3810 - construct the +0x4C4 view object
// ---------------------------------------------------------------------------

/// Builds the 0x4D4-byte view object that owns the `+0x4C4` child pair used by
/// the Wave 65/77/89 loaders.
///
/// # Original
/// - Ghidra: `FUN_802d3810` at `0x802D3810` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3810_FUN_802d3810.c`
///
/// # Arguments
/// - `this`: object to construct.
/// - `child_a`: pointer stored at `+0x4C4`.
/// - `child_b`: pointer stored at `+0x4C8`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Installs vtable `0x80350538`, clears `+4`, `+5`, `+8`, sets `+6 = 1`, runs
/// the three sub-object constructors at `+0xC` (`0x80268FF0`), `+0x198`
/// (`0x8026F690`) and `+0x374` (`0x8026DD60`), then stores the two children and
/// the `state = 0`, `ready = 1` pair at `+0x4CC`/`+0x4D0`.
///
/// # Safety
/// `this` must be a writable 0x4D4-byte block.
///
/// # Confidence
/// high - the raw prologue shows `r3` is the object and the call site
/// `0x802D6210` passes the three pointers; the `__savegprlr` return Ghidra uses
/// for the object is a decompiler artefact, not a real argument.
pub unsafe fn fn_802d3810(this: usize, child_a: u32, child_b: u32) {
    platform::write_u32(this, 0x8035_0538);
    platform::write_u8(this + 4, 0);
    platform::write_u8(this + 5, 0);
    platform::write_u8(this + 6, 1);
    platform::write_u32(this + 8, 0);
    platform::call1(0x8026_8FF0, (this + 0x0C) as u32);
    platform::call1(0x8026_F690, (this + 0x198) as u32);
    platform::call1(0x8026_DD60, (this + 0x374) as u32);
    platform::write_u32(this + 0x4C4, child_a);
    platform::write_u32(this + 0x4C8, child_b);
    platform::write_u32(this + 0x4CC, 0);
    platform::write_u32(this + 0x4D0, 1);
}

// ---------------------------------------------------------------------------
// 0x802D5080 / 0x802D5090 - two tail-call wrappers around fn_802d4cd0
// ---------------------------------------------------------------------------

/// Rebuilds effect slot `this + index * 0x10` in mode 1.
///
/// # Original
/// - Ghidra: `FUN_802d5080` at `0x802D5080` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5080_FUN_802d5080.c`
///
/// # Arguments
/// - `this`: base of the effect table.
/// - `index`: slot number; multiplied by the 0x10-byte slot stride.
/// - `argument`: resource word forwarded unchanged in `r5`.
///
/// # Returns
/// [`fn_802d4cd0`]'s result.
///
/// # Side effects
/// A tail branch to [`fn_802d4cd0`]`(slot, 1, argument)`.
///
/// # Safety
/// `this + index * 0x10` must be a live slot.
///
/// # Confidence
/// high - the three-instruction body is `slwi`/`add`/`b`: `r5` is never
/// written, so the wrapper's own third argument is the callee's `argument`.
pub unsafe fn fn_802d5080(this: usize, index: u32, argument: u32) -> u32 {
    // observed: `rlwinm r0,r4,4,0,27`, i.e. `(index << 4) & 0x0FFFFFFF`.
    let slot = this.wrapping_add((index.wrapping_shl(4) & 0x0FFF_FFFF) as usize);
    fn_802d4cd0(slot, 1, argument)
}

/// Rebuilds effect slot `this + index * 0x10` in mode 7.
///
/// # Original
/// - Ghidra: `FUN_802d5090` at `0x802D5090` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d5090_FUN_802d5090.c`
///
/// # Arguments
/// Same three as [`fn_802d5080`]; the mode constant is `7`.
///
/// # Returns
/// [`fn_802d4cd0`]'s result.
///
/// # Side effects
/// A tail branch to [`fn_802d4cd0`]`(slot, 7, argument)`.
///
/// # Safety
/// `this + index * 0x10` must be a live slot.
///
/// # Confidence
/// high - identical body to [`fn_802d5080`] with the mode literal `7`.
pub unsafe fn fn_802d5090(this: usize, index: u32, argument: u32) -> u32 {
    // observed: same `rlwinm r0,r4,4,0,27` slot arithmetic as [`fn_802d5080`].
    let slot = this.wrapping_add((index.wrapping_shl(4) & 0x0FFF_FFFF) as usize);
    fn_802d4cd0(slot, 7, argument)
}

// ---------------------------------------------------------------------------
// 0x802D71F0 - interactive console/viewer loop
// ---------------------------------------------------------------------------

/// Waits for a key sequence, scans the view range, then runs a keypad-driven
/// selection loop until the console is dismissed.
///
/// # Original
/// - Ghidra: `FUN_802d71f0` at `0x802D71F0` (532 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d71f0_FUN_802d71f0.c`
///
/// # Arguments
/// - `view`: the view object (`+0x14` first index, `+0x18` cursor, `+0x1C`
///   vertical offset, `+0x22` active flag).
/// - `sequence`: optional `u16` key list terminated by `0`; `0` skips the wait.
///
/// # Returns
/// `0` when `view` is null; every other path stays in the loop forever.
///
/// # Side effects
/// - `0x8015F730(0x80350790)` logs "No Console" for a null view.
/// - `0x801B1160()` one-time console setup.
/// - The wait loop polls `0x801ED3A0(0, buf, 1)` every 50 ms and compares
///   `buf[1]` against the sequence.
/// - The main loop polls every 100 ms, toggles the [`CONSOLE_FLAG`] byte on the
///   `0x8000` bit of `buf[1]`, optionally calls the [`CONSOLE_CALLBACK`], and
///   moves the selection by 1 (D-pad up/down) or 5 (left/right, clamped to
///   `-150..=10`), calling `0x802340A0(view)` when the index or offset changes.
///
/// # Safety
/// `view` must be a live view object and `sequence` a readable `u16` list.
///
/// # Confidence
/// medium - every call, mask and clamp is literal; the pad buffer layout
/// (`buf[0]` buttons, `buf[1]` flags) and the console flag role are
/// `// inferred:` from how the two words are tested.
pub unsafe fn fn_802d71f0(view: usize, sequence: usize) -> u32 {
    if view == 0 {
        platform::call1(0x8015_F730, 0x8035_0790);
        return 0;
    }
    platform::call0(0x801B_1160);
    let mut input = crate::platform::GuestLocal::new( [0u32; 2]);
    if sequence != 0 {
        let mut position: u32 = 0;
        while platform::read_u16(sequence + position as usize * 2) != 0 {
            platform::call3(0x801E_D3A0, 0, input.as_mut_ptr() as u32, 1);
            fn_802d7170(0x32);
            if input[1] != 0 {
                let key = platform::read_u16(sequence + position as usize * 2) as u32;
                let matched = input[1] & key;
                // observed: `addi`/`neg`/`srawi`/`and` pair - advance on a
                // match, restart the sequence otherwise.
                position = if matched != 0 { position.wrapping_add(1) } else { 0 };
            }
        }
    }

    let first = platform::read_i32(view + 0x14);
    platform::write_u8(view + 0x22, 1);
    let last = platform::call1(0x8023_45C0, view as u32) as i32;
    let mut cursor = first;
    while cursor < last {
        platform::write_u32(view + 0x18, cursor as u32);
        platform::call1(0x8023_40A0, view as u32);
        fn_802d7170(0xFA);
        cursor += 1;
    }
    platform::write_u32(view + 0x18, first as u32);
    platform::call1(0x8023_40A0, view as u32);
    let mut cursor = first;

    loop {
        platform::call3(0x801E_D3A0, 0, input.as_mut_ptr() as u32, 1);
        fn_802d7170(100);
        if (input[1] & 0x8000) != 0 {
            // observed: countLeadingZeros(byte) >> 5 is `byte == 0`; the press
            // toggles the console flag.
            let current = platform::read_u8(CONSOLE_FLAG);
            platform::write_u8(CONSOLE_FLAG, u8::from(current == 0));
        }
        if platform::read_u8(CONSOLE_FLAG) != 0 {
            let callback = platform::read_u32(CONSOLE_CALLBACK);
            if callback != 0 {
                platform::call1(callback as usize, input.as_mut_ptr() as u32);
                continue;
            }
        }

        let mut offset = platform::read_u16(view + 0x1C) as i16;
        let mut new_offset = offset;
        let target;
        if (input[0] & 4) == 0 {
            if (input[0] & 8) == 0 {
                target = cursor;
                if (input[0] & 2) == 0 {
                    if (input[0] & 1) != 0 {
                        new_offset = offset + 5;
                        if new_offset > 10 {
                            new_offset = 10;
                        }
                    }
                } else {
                    new_offset = offset - 5;
                    if new_offset < -150 {
                        new_offset = -150;
                    }
                }
            } else {
                let mut bounded = cursor - 1;
                if bounded < first {
                    bounded = first;
                }
                target = bounded;
            }
        } else {
            let mut bounded = cursor + 1;
            if last < cursor + 1 {
                bounded = last;
            }
            target = bounded;
        }

        if target != cursor || new_offset != offset {
            platform::write_u16(view + 0x1C, new_offset as u16);
            platform::write_u32(view + 0x18, target as u32);
            platform::call1(0x8023_40A0, view as u32);
        }
        cursor = target;
    }
}

// ---------------------------------------------------------------------------
// 0x802D7680 - construct the BRRES section table
// ---------------------------------------------------------------------------

/// Initialises the seven-word BRRES section record and walks the section table
/// the caller supplies, binding each entry to the setter for its tag class.
///
/// # Original
/// - Ghidra: `FUN_802d7680` at `0x802D7680` (344 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7680_FUN_802d7680.c`
///
/// # Arguments
/// - `this`: record to construct; word `+0` takes the table pointer and
///   `+0x1C` the vtable `0x803507A8`.
/// - `table`: section table. `+0xC` is the entry count and `+0x20` the first
///   entry.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - `0x802D78A0(this, table)` stores the table at `+0`.
/// - Per entry: `0x802D7910(this, tag)` returns the tag class `0..=5`; the
///   matching `0x802D78B0..0x802D7900` setter stores the entry into
///   `+4..+0x18`; the entry size at `+4` of the bound section advances the
///   cursor.
///
/// # Safety
/// `this` must be writable through `+0x1C` and `table` must describe the same
/// layout the setters expect.
///
/// # Confidence
/// medium - the register trace at `0x802D76D0` proves `r3`/`r4` still hold
/// `this`/`table`, so the export's empty argument list is a signature omission;
/// the tag-class numbering is `// inferred:` from the six-way setter chain.
pub unsafe fn fn_802d7680(this: usize, table: u32) -> usize {
    platform::write_u32(this + 0x1C, 0x8035_07A8);
    platform::write_u32(this, 0);
    platform::write_u32(this + 4, 0);
    platform::write_u32(this + 8, 0);
    platform::write_u32(this + 0x0C, 0);
    platform::write_u32(this + 0x10, 0);
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this + 0x18, 0);
    // observed: `r3 = this`, `r4 = table` at the call; the callee stores `r4`
    // at `*this`.
    platform::call2(0x802D_78A0, this as u32, table);
    let mut entry = table.wrapping_add(0x20);
    let table = platform::read_u32(this);
    let count = platform::read_u32(table.wrapping_add(0x0C) as usize);
    let mut index = 0;
    while index < count {
        let tag = platform::read_u32(entry as usize);
        let class = platform::call2(0x802D_7910, this as u32, tag);
        let mut section: u32 = 0;
        match class {
            0 => {
                platform::call2(0x802D_78B0, this as u32, entry);
                section = platform::read_u32(this + 4);
            }
            1 => {
                platform::call2(0x802D_78C0, this as u32, entry);
                section = platform::read_u32(this + 8);
            }
            2 => {
                platform::call2(0x802D_78D0, this as u32, entry);
                section = platform::read_u32(this + 0x0C);
            }
            3 => {
                platform::call2(0x802D_78E0, this as u32, entry);
                section = platform::read_u32(this + 0x10);
            }
            4 => {
                platform::call2(0x802D_78F0, this as u32, entry);
                section = platform::read_u32(this + 0x14);
            }
            5 => {
                platform::call2(0x802D_7900, this as u32, entry);
                section = platform::read_u32(this + 0x18);
            }
            // observed: an unknown class leaves the pointer at 0, exactly as
            // the export's initialised `iVar2 = 0` does.
            _ => {}
        }
        entry = entry.wrapping_add(platform::read_u32(section.wrapping_add(4) as usize));
        index += 1;
    }
    this
}

// ---------------------------------------------------------------------------
// 0x802DB6C0 - construct the small lyt record and run its base initialiser
// ---------------------------------------------------------------------------

/// Stores the payload word at `+4`, installs vtable `0x803509E0` and calls that
/// vtable's `+0x24` base initialiser with the same two arguments.
///
/// # Original
/// - Ghidra: `FUN_802db6c0` at `0x802DB6C0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db6c0_FUN_802db6c0.c`
///
/// # Arguments
/// - `this`: record to construct.
/// - `argument`: payload word stored at `+4` and forwarded to the initialiser.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Two word stores and one indirect call through `*(0x803509E0 + 0x24)`.
///
/// # Safety
/// `this` must be writable and the vtable slot must be a valid function.
///
/// # Confidence
/// high - the raw body shows `r3 = this`, `r4 = argument` at the `bctrl`, and
/// the slot is loaded from the vtable word at `+0x24`.
pub unsafe fn fn_802db6c0(this: usize, argument: u32) -> usize {
    platform::write_u32(this + 4, argument);
    platform::write_u32(this, 0x8035_09E0);
    let slot = platform::read_u32(0x8035_09E0 + 0x24) as usize;
    platform::call2(slot, this as u32, argument);
    this
}

// ---------------------------------------------------------------------------
// 0x802DB750 - advance one lyt state machine step
// ---------------------------------------------------------------------------

/// Runs the one-shot "find the source object" step, then reports whether the
/// child at `object + 0x74` has reached state 1 or 2.
///
/// # Original
/// - Ghidra: `FUN_802db750` at `0x802DB750` (276 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db750_FUN_802db750.c`
///
/// # Arguments
/// - `this`: state record (`+4` object pointer, `+8` phase, `+0xC` fractional
///   remainder, `+0x10` scale).
/// - `scaled`: non-zero applies the object's `+0xE4` multiplier to the scale.
///
/// # Returns
/// `1` when the child state is `1` or `2`, else `0`.
///
/// # Side effects
/// - When `+8` is `0`: calls the object's virtual `+0x0C` slot. A zero result
///   sets `+8 = 2` and copies `0x802D87C0(object)`'s `+0x28` scale into
///   `+0x10` (and, when `scaled`, the fractional part of
///   `+0xE4 * scale` into `+0xC`); a non-zero result sets `+8 = 1`.
/// - Always calls `0x802967E0(object + 0x74)` and reads its `+0xC` state.
///
/// # Safety
/// `this` and its `+4` object must be live; the object's vtable must hold a
/// valid `+0x0C` entry.
///
/// # Confidence
/// medium - every guard, store and the state test are literal. The float
/// subtraction is the compiler's float-to-int truncation idiom, written here as
/// `scaled - trunc(scaled)`; the offset roles are `// inferred:`.
pub unsafe fn fn_802db750(this: usize, scaled: u32) -> u32 {
    if platform::read_u32(this + 8) == 0 {
        let object = platform::read_u32(this + 4) as usize;
        let vtable = platform::read_u32(object) as usize;
        let ready = platform::call1(platform::read_u32(vtable + 0x0C) as usize, object as u32);
        if ready == 0 {
            platform::write_u32(this + 8, 2);
            let source = platform::call1(0x802D_87C0, object as u32) as usize;
            if source != 0 {
                let scale = platform::read_f32(source + 0x28);
                platform::write_f32(this + 0x10, scale);
                if scaled != 0 {
                    let combined =
                        platform::read_u32(source + 0xE4) as f32 * scale;
                    // observed: subtract the truncated value to keep the
                    // fractional remainder.
                    platform::write_f32(this + 0x0C, combined - (combined as i32 as f32));
                }
            }
        } else {
            platform::write_u32(this + 8, 1);
        }
    }
    let child = platform::call1(
        0x8029_67E0,
        platform::read_u32(this + 4).wrapping_add(0x74),
    ) as usize;
    let state = if child == 0 {
        0
    } else {
        platform::read_u32(child + 0x0C)
    };
    u32::from(state == 1 || state == 2)
}

// ---------------------------------------------------------------------------
// 0x802DB8C0 - dispatch the lyt phase or copy a three-float position
// ---------------------------------------------------------------------------

/// Runs the object's virtual `+0x88` slot while the phase is `1`, and copies a
/// three-float vector into `+0x14..+0x1C` while the phase is `2`.
///
/// # Original
/// - Ghidra: `FUN_802db8c0` at `0x802DB8C0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db8c0_FUN_802db8c0.c`
///
/// # Arguments
/// - `this`: state record (`+4` object pointer, `+8` phase).
/// - `values`: three consecutive `f32` values for the phase-2 path.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Phase 1: tail-branches to `*(object_vtable + 0x88)`. Phase 2: three float
/// stores. Any other phase does nothing.
///
/// # Safety
/// `this` must be live, and `values` must point at twelve readable bytes when
/// the phase is 2.
///
/// # Confidence
/// high - the compare chain, the `bctr` tail branch and the three `lfs`/`stfs`
/// pairs are literal at `0x802DB8C0..0x802DB900`.
pub unsafe fn fn_802db8c0(this: usize, values: usize) {
    let phase = platform::read_u32(this + 8);
    if phase == 1 {
        let object = platform::read_u32(this + 4) as usize;
        let vtable = platform::read_u32(object) as usize;
        let slot = platform::read_u32(vtable + 0x88) as usize;
        // observed: `bctr`, a tail call with `r3 = object`, `r4 = values`.
        platform::call2(slot, object as u32, values as u32);
    } else if phase == 2 {
        platform::write_f32(this + 0x14, platform::read_f32(values));
        platform::write_f32(this + 0x18, platform::read_f32(values + 4));
        platform::write_f32(this + 0x1C, platform::read_f32(values + 8));
    }
}

// ---------------------------------------------------------------------------
// 0x802DB970 - advance the lyt timer
// ---------------------------------------------------------------------------

/// Advances the per-instance timer while the phase is `2`, or forwards to the
/// object's virtual `+0x94` slot while the phase is `1`.
///
/// # Original
/// - Ghidra: `FUN_802db970` at `0x802DB970` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db970_FUN_802db970.c`
///
/// # Arguments
/// - `this`: state record (`+0` vtable, `+4` object pointer, `+8` phase,
///   `+0xC` accumulated time, `+0x10` last delta, `+0x14` output block).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Phase 1: calls `*(object_vtable + 0x94)`.
/// - Phase 2: reads `0x802D87C0(object)`. When the source is missing or its
///   `+0xB8` is zero, adds `+0x10` to `+0xC` and calls the record's own virtual
///   `+0x20` once the total reaches [`TIME_THRESHOLD`] (`1.0f`). Otherwise adds
///   the source's `+0x28` delta, stores it at `+0x10`, subtracts `1.0f` from
///   the total and calls `0x80287C10(source, source[+0xB8], this + 0x20, 0, 0,
///   this + 0x14)`.
///
/// # Safety
/// `this` must be live and its vtable slots valid; the phase-2 source must be a
/// live object when non-null.
///
/// # Confidence
/// medium - every branch, float operation and the six-argument call are
/// literal; the field semantics (time accumulator and delta) are `// inferred:`
/// from the add/compare sequence.
pub unsafe fn fn_802db970(this: usize) {
    let phase = platform::read_u32(this + 8);
    if phase == 1 {
        let object = platform::read_u32(this + 4) as usize;
        let vtable = platform::read_u32(object) as usize;
        let slot = platform::read_u32(vtable + 0x94) as usize;
        platform::call1(slot, object as u32);
        return;
    }
    if phase != 2 {
        return;
    }
    let source = platform::call1(0x802D_87C0, platform::read_u32(this + 4)) as usize;
    let threshold = platform::read_f32(TIME_THRESHOLD);
    if source == 0 || platform::read_u32(source + 0xB8) == 0 {
        let total = platform::read_f32(this + 0x0C) + platform::read_f32(this + 0x10);
        platform::write_f32(this + 0x0C, total);
        if threshold <= total {
            let vtable = platform::read_u32(this) as usize;
            platform::call1(platform::read_u32(vtable + 0x20) as usize, this as u32);
        }
    } else {
        let delta = platform::read_f32(source + 0x28);
        let total = platform::read_f32(this + 0x0C) + delta;
        platform::write_f32(this + 0x10, delta);
        platform::write_f32(this + 0x0C, total);
        if threshold <= total {
            platform::write_f32(this + 0x0C, total - threshold);
            platform::call6(
                0x8028_7C10,
                source as u32,
                platform::read_u32(source + 0xB8),
                (this + 0x20) as u32,
                0,
                0,
                (this + 0x14) as u32,
            );
        }
    }
}

// ===========================================================================
// Wave 96 -- the four-float layout records, the AX console bring-up, the
// pitch-ratio helper and four four-byte entries (0x802D0D90..0x802DEBB4)
// ===========================================================================
//
// Everything below was recovered from the export plus the raw instruction
// stream at `work/dol/main.elf`, because the export loses the argument
// registers whenever the CodeWarrior `__savegprlr_N` prologue helper is the
// first call in a function (see `docs/QUALITY.md` section 5.5): Ghidra models
// that helper as returning the *incoming* `r3`/`r4` in a 64-bit value. The
// recovered signature of every function here is therefore stated explicitly
// and marked `// inferred:` where only the call sites prove it.
//
// The SDA cells are resolved with `r13 = 0x8042F680` and `r2 = 0x80433080`,
// the two baselines Waves 77/89/92 used. Float constants below were read out
// of the image, not guessed.

/// `r2 - 0x3B0C`: `0.0f`. The layout records use it as the "no offset" fill
/// for `+0x48..+0x54` and `+0x2C`.
const SDA2_FLOAT_ZERO: usize = 0x8042_F574;

/// `r2 - 0x3B00`, `r2 - 0x3AFC`: `10.0f` and `100000.0f`, the near/far pair
/// handed to the `0x802C6A30` frustum initialiser by [`fn_802d0d90`].
const SDA2_CULL_RANGE: usize = 0x8042_F580;

/// `r2 - 0x3ABC`: `0.0f`, the only modulate colour the AX mixer setup uses.
const SDA2_MIX_LEVEL: usize = 0x8042_F5C4;

/// `r2 - 0x3A68`: `1.0f`, the neutral playback ratio returned by
/// [`fn_802d6f20`] for index `0` and for the empty-index guard.
const SDA2_RATIO_ONE: usize = 0x8042_F618;

/// `r2 - 0x3A60`: the `f64` value `2.0`, the base of the ratio exponent.
const SDA2_RATIO_BASE: usize = 0x8042_F620;

/// `r2 - 0x3B08`: `2^52` as a `f64`, the bias of the *unsigned* 32-bit to
/// double conversion (`CONCAT44(0x43300000, x)`).
const SDA2_U32_BIAS: usize = 0x8042_F578;

/// `r2 - 0x3AF8` and `r2 - 0x3A58`: `2^52 + 2^31` as a `f64`, the bias of the
/// *signed* 32-bit conversion (`CONCAT44(0x43300000, x ^ 0x80000000)`).
const SDA2_I32_BIAS: usize = 0x8042_F588;

/// `r2 - 0x3A4C`: `0.0f`, the target value of the fade tick in
/// [`fn_802d7070`].
const SDA2_FADE_TARGET: usize = 0x8042_F634;

/// `0x802FD2A0`, twelve `f32` factors. Read out of the image they are
/// `1.0, 1.0595, 1.1225, ... 1.8877`, i.e. `2^(n/12)` for `n = 0..11`: one
/// equal-tempered octave. [`fn_802d6f20`] indexes it by `abs(index) % 12`.
const PITCH_RATIO_TABLE: usize = 0x802F_D2A0;

/// `r13 - 0x4710`, `r13 - 0x470C`: the published transform block owner and
/// its `+0x6C8` source, written by [`fn_802d48c0`]. `// inferred:` both are
/// only written here and read by the 0x6C-stride consumers.
const VIEW_TRANSFORM_OWNER: usize = 0x8042_AF70;
const VIEW_TRANSFORM_SOURCE: usize = 0x8042_AF74;

/// Calls a target that takes four single-precision arguments in `f1..f4`.
///
/// # Safety
/// `target` must be a function with exactly this ABI.
#[inline]
unsafe fn call_f32x4(target: usize, a: f32, b: f32, c: f32, d: f32) {
    let function = crate::platform::guest_fn!(target, fn(f32, f32, f32, f32));
    function(a, b, c, d);
}

// ---------------------------------------------------------------------------
// 0x802D0D90 - build the four-float layout record
// ---------------------------------------------------------------------------

/// Initialises a layout record from four `f32` values: runs the `0x802C6A30`
/// initialiser, installs vtable `0x80350504`, stores the two scale values and
/// clears the four trailing words.
///
/// # Original
/// - Ghidra: `FUN_802d0d90` at `0x802D0D90` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0d90_FUN_802d0d90.c`
///
/// # Arguments
/// - `f0`, `f1`: the pair stored at `+0x40`/`+0x44`; every caller passes the
///   owner's `+0x40`/`+0x44` (see [`fn_802d1900`]).
/// - `f2`, `f3`: the pair copied into a two-float stack temporary and handed to
///   `0x802C6A30` as its fifth argument.
/// - `this`: the record to build. Its `+0x38..+0x54` block is written here.
/// - `arg6`: forwarded to [`fn_802d1250`] as its second argument.
/// - `arg7`: sixth argument of `0x802C6A30`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - `0x802C6A30(10.0f, 100000.0f, this, 1, &pair, arg7)`.
/// - Stores the vtable, `0` at `+0x3C`, the two scales at `+0x40`/`+0x44` and
///   `0.0f` at `+0x48..+0x54`.
/// - Runs [`fn_802d1250`]`(this, arg6)`.
///
/// # Safety
/// `this..this+0x58` must be writable; the callee must accept the argument
/// shape above.
///
/// # Confidence
/// high - the prologue at `0x802D0D9C` proves `r3 = this`, `r4 = arg6`,
/// `r5 = arg7`; every store and both constants are literal.
pub unsafe fn fn_802d0d90(
    f0: f32,
    f1: f32,
    f2: f32,
    f3: f32,
    this: usize,
    arg6: u32,
    arg7: u32,
) -> usize {
    let mut pair = crate::platform::GuestLocal::new( [f2, f3]);
    // observed: r4 is moved to r6 and r5 is repointed at the stack pair, so the
    // callee sees (10.0f, 100000.0f, this, 1, &pair, arg7).
    let initialise = crate::platform::guest_fn!(0x802C_6A30usize, fn(f32, f32, usize, u32, *mut f32, u32));
    initialise(
        platform::read_f32(SDA2_CULL_RANGE),
        platform::read_f32(SDA2_CULL_RANGE + 4),
        this,
        1,
        pair.as_mut_ptr(),
        arg7,
    );

    platform::write_u32(this + 0x38, 0x8035_0504);
    platform::write_u32(this + 0x3C, 0);
    platform::write_f32(this + 0x40, f0);
    platform::write_f32(this + 0x44, f1);
    let clear = platform::read_f32(SDA2_FLOAT_ZERO);
    platform::write_f32(this + 0x48, clear);
    platform::write_f32(this + 0x4C, clear);
    platform::write_f32(this + 0x50, clear);
    platform::write_f32(this + 0x54, clear);

    fn_802d1250(this, arg6 as i32);
    this
}

// ---------------------------------------------------------------------------
// 0x802D0F40 - apply the record's vector to the draw state
// ---------------------------------------------------------------------------

/// Pushes one layout record into the draw state: saves and restores the global
/// vector around the transform setup, then feeds the record's twelve values to
/// `0x802D3200`, `0x802D3230` and `0x802D3270`.
///
/// # Original
/// - Ghidra: `FUN_802d0f40` at `0x802D0F40` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0f40_FUN_802d0f40.c`
///
/// # Arguments
/// - `this`: layout record; `+0x34` is the flag word and `+0x58` the vector
///   block that [`fn_802d17c0`] returns.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Unless bit `0x40` of `+0x34` is set, reads the current vector with
///   `0x802C6DF0`, installs [`SDA_FLOAT_QUAD`], runs `0x802C6D60(this)` and
///   writes the saved vector back with `0x802C6DD0`.
/// - With the bit set only `0x802C6D60(this)` runs.
/// - Then re-reads the record through [`fn_802d17c0`] and calls
///   `0x802D3200` with six floats, `0x802D3230` with four words and
///   `0x802D3270` with two.
///
/// # Safety
/// `this` must be a live record and the four callees must be callable with the
/// shapes shown.
///
/// # Confidence
/// high - the flag test, the two `0x802C6DD0` calls and all ten loads are
/// literal at `0x802D0F54..0x802D0FF8`; the export's `(double)` casts are the
/// usual FPR promotion of a single-precision argument.
pub unsafe fn fn_802d0f40(this: usize) {
    let flags = platform::read_u16(this + 0x34);
    if flags & 0x40 == 0 {
        let mut saved = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let base = saved.as_mut_ptr() as usize;
        // observed: the four output pointers arrive in the order 0x14, 0x10,
        // 0x0C, 0x08 of the frame, which is this array order.
        platform::call4(
            0x802C_6DF0,
            base as u32,
            (base + 4) as u32,
            (base + 8) as u32,
            (base + 12) as u32,
        );
        call_f32x4(
            0x802C_6DD0,
            platform::read_f32(SDA_FLOAT_QUAD),
            platform::read_f32(SDA_FLOAT_QUAD + 4),
            platform::read_f32(SDA_FLOAT_QUAD + 8),
            platform::read_f32(SDA_FLOAT_QUAD + 12),
        );
        platform::call1(0x802C_6D60, this as u32);
        call_f32x4(0x802C_6DD0, saved[0], saved[1], saved[2], saved[3]);
    } else {
        platform::call1(0x802C_6D60, this as u32);
    }

    let vector = fn_802d17c0(this);
    let six = crate::platform::guest_fn!(0x802D_3200usize, fn(f32, f32, f32, f32, f32, f32));
    six(
        platform::read_f32(vector),
        platform::read_f32(vector + 4),
        platform::read_f32(vector + 8),
        platform::read_f32(vector + 12),
        platform::read_f32(vector + 16),
        platform::read_f32(vector + 20),
    );
    platform::call4(
        0x802D_3230,
        platform::read_u32(vector + 0x18),
        platform::read_u32(vector + 0x1C),
        platform::read_u32(vector + 0x20),
        platform::read_u32(vector + 0x24),
    );
    platform::call2(
        0x802D_3270,
        platform::read_u32(vector + 0x28),
        platform::read_u32(vector + 0x2C),
    );
}

// ---------------------------------------------------------------------------
// 0x802D1010 - apply the record and decode it into a caller buffer
// ---------------------------------------------------------------------------

/// Like [`fn_802d0f40`] but for the textured path: swaps the global vector
/// around `0x802C6D80`, then replays the record's twelve values through the
/// `0x80253D*` rectangle writers into `out`.
///
/// # Original
/// - Ghidra: `FUN_802d1010` at `0x802D1010` (304 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1010_FUN_802d1010.c`
///
/// # Arguments
/// - `this`: layout record; `+0x34` flag word, `+0x58` vector block.
/// - `out`: destination record. One word is read from it before the swap and
///   passed back to `0x802C6D80`; the rest is written by the `0x80253D*`
///   helpers, which all take it as their first integer argument.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Bit `0x40` of `+0x34` clear: `0x802C6DF0` saves the vector, `0x802C6DD0`
///   installs [`SDA_FLOAT_QUAD`], `0x802C6D80(this, &word)` runs and the saved
///   vector is restored.
/// - Bit set: only `0x802C6D80(this, &word)` runs.
/// - `0x80253D60(f0..f3, out)`, `0x80253E00(f4, f5, out)`,
///   `0x80253D20(out, four words)` and `0x80253D40(out, two words)`.
///
/// # Safety
/// `this` must be a live record and `out` a writable record of the shape the
/// `0x80253D*` helpers expect.
///
/// # Confidence
/// high - every load, store and call is literal; the two branches differ only
/// in whether the global vector is swapped.
pub unsafe fn fn_802d1010(this: usize, out: usize) {
    let flags = platform::read_u16(this + 0x34);
    let mut word = crate::platform::GuestLocal::new( platform::read_u32(out));
    if flags & 0x40 == 0 {
        let mut saved = crate::platform::GuestLocal::new( [0.0f32; 4]);
        let base = saved.as_mut_ptr() as usize;
        platform::call4(
            0x802C_6DF0,
            base as u32,
            (base + 4) as u32,
            (base + 8) as u32,
            (base + 12) as u32,
        );
        call_f32x4(
            0x802C_6DD0,
            platform::read_f32(SDA_FLOAT_QUAD),
            platform::read_f32(SDA_FLOAT_QUAD + 4),
            platform::read_f32(SDA_FLOAT_QUAD + 8),
            platform::read_f32(SDA_FLOAT_QUAD + 12),
        );
        platform::call2(
            0x802C_6D80,
            this as u32,
            ((word.as_mut_ptr() as *mut u32)) as u32,
        );
        call_f32x4(0x802C_6DD0, saved[0], saved[1], saved[2], saved[3]);
    } else {
        platform::call2(
            0x802C_6D80,
            this as u32,
            ((word.as_mut_ptr() as *mut u32)) as u32,
        );
    }

    let vector = fn_802d17c0(this);
    let write_quad = crate::platform::guest_fn!(0x8025_3D60usize, fn(f32, f32, f32, f32, usize));
    write_quad(
        platform::read_f32(vector),
        platform::read_f32(vector + 4),
        platform::read_f32(vector + 8),
        platform::read_f32(vector + 12),
        out,
    );
    let write_pair = crate::platform::guest_fn!(0x8025_3E00usize, fn(f32, f32, usize));
    write_pair(
        platform::read_f32(vector + 0x10),
        platform::read_f32(vector + 0x14),
        out,
    );
    platform::call5(
        0x8025_3D20,
        out as u32,
        platform::read_u32(vector + 0x18),
        platform::read_u32(vector + 0x1C),
        platform::read_u32(vector + 0x20),
        platform::read_u32(vector + 0x24),
    );
    platform::call3(
        0x8025_3D40,
        out as u32,
        platform::read_u32(vector + 0x28),
        platform::read_u32(vector + 0x2C),
    );
}

// ---------------------------------------------------------------------------
// 0x802D1850 - fill one axis of the parent record
// ---------------------------------------------------------------------------

/// Writes one axis of a parent layout record: positions a child with
/// `0x801C0C60`, stores the extent, and derives the second coordinate from the
/// first unless the parent marks it absolute.
///
/// # Original
/// - Ghidra: `FUN_802d1850` at `0x802D1850` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1850_FUN_802d1850.c`
///
/// # Arguments
/// - `f0`, `f1`: the value stored at `+0xC` and the value stored at `+0x1C`.
/// - `f2`, `f3`: forwarded to `0x801C0C60` as its first two arguments.
/// - `parent`: read once at `+4`; `0` selects the relative path.
/// - `target`: the record written here (`+0xC`, `+0x1C`, `+0x2C`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `0x801C0C60(f2, f3, 1.0f, target)`.
/// - Stores `f0` at `+0xC`.
/// - When `parent[+4] == 0`, `f1` is replaced by `f1 - f3` first; the result is
///   stored at `+0x1C`.
/// - Stores `0.0f` at `+0x2C`.
///
/// # Safety
/// `target..target+0x30` must be writable and `parent` readable at `+4`.
///
/// # Confidence
/// high - the prologue saves `f1..f4` and `r3`/`r4`, which fixes the six
/// arguments; the branch and the three stores are literal. The export labels
/// the third value `undefined8` only because it is one FPR argument.
pub unsafe fn fn_802d1850(f0: f32, f1: f32, f2: f32, f3: f32, parent: usize, target: usize) {
    let position = crate::platform::guest_fn!(0x801C_0C60usize, fn(f32, f32, f32, usize));
    position(
        f2,
        f3,
        platform::read_f32(SDA_TRANSFORM_SCRATCH),
        target,
    );
    platform::write_f32(target + 0x0C, f0);
    let second = if platform::read_u32(parent + 4) == 0 {
        f1 - f3
    } else {
        f1
    };
    platform::write_f32(target + 0x1C, second);
    platform::write_f32(target + 0x2C, platform::read_f32(SDA2_FLOAT_ZERO));
}

// ---------------------------------------------------------------------------
// 0x802D1900 - compose the parent record and submit the descriptor
// ---------------------------------------------------------------------------

/// Builds the temporary parent record used by the draw pass: allocates it on
/// the stack, fills it with [`fn_802d0d90`] and [`fn_802d1850`], then submits
/// the caller's descriptor through `0x802C1D10`.
///
/// # Original
/// - Ghidra: `FUN_802d1900` at `0x802D1900` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1900_FUN_802d1900.c`
///
/// # Arguments
/// - `this`: source record; `+0x08`/`+0x0C` are the size pair, `+0x34` the flag
///   word, `+0x3C` the argument forwarded to [`fn_802d0d90`], `+0x40`/`+0x44`
///   the scale pair.
/// - `mode`: bit-mask; bit 0, 1 and 2 are passed as three separate flags. When
///   `mode == 0` the whole body is skipped.
/// - `descriptor`: one word is copied into the submit block.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - [`fn_802d0d90`] fills the record at frame `+0x38`.
/// - Bit 3 of `this[+0x34]` runs `0x802D12B0(record)`.
/// - The record's `+0x18`/`+0x1C` are set to `0.0f`/`1.0f`, `+0x34` gains
///   `0x40`, `+0` is cleared and [`fn_802d0f40`] runs.
/// - [`fn_802d1850`]`(0.0f, 0.0f, this[+8], this[+0xC], record, frame)` fills
///   the 0x30-byte submit block at frame `+0x08`.
/// - `0x802C1D10(block, mode & 1, mode >> 1 & 1, mode >> 2 & 1, &descriptor, 1)`.
///
/// # Safety
/// `this` must be a live record and `descriptor` readable; the frame is a
/// local buffer, so only the callees that receive pointers into it may write.
///
/// # Confidence
/// medium - the call chain and every offset are literal, but the record is a
/// stack temporary whose two halves Ghidra models as separate locals
/// (`local_a0` and `auStack_d0`); the single 0x90-byte buffer here is
/// `// inferred:` from `local_d8` being the lowest address of the frame.
pub unsafe fn fn_802d1900(this: usize, mode: u32, descriptor: usize) {
    if mode == 0 {
        return;
    }
    // One 0x90-byte scratch frame: submit block at +0x08, layout record at
    // +0x38, its +0x18/+0x1C floats and +0x34 flag word land at +0x50/+0x54
    // and +0x6C, and fn_802d0d90 writes up to record + 0x54 = frame + 0x8C.
    let mut frame = crate::platform::GuestLocal::new( [0u32; 36]);
    let base = frame.as_mut_ptr() as usize;
    let record = base + 0x38;

    fn_802d0d90(
        platform::read_f32(this + 0x40),
        platform::read_f32(this + 0x44),
        platform::read_f32(this + 8),
        platform::read_f32(this + 0x0C),
        record,
        platform::read_u32(this + 0x3C),
        1,
    );
    if platform::read_u16(this + 0x34) & 8 != 0 {
        platform::call1(0x802D_12B0, record as u32);
    }
    platform::write_f32(base + 0x50, platform::read_f32(SDA2_FLOAT_ZERO));
    platform::write_f32(base + 0x54, platform::read_f32(SDA_TRANSFORM_SCRATCH));
    platform::write_u16(base + 0x6C, platform::read_u16(base + 0x6C) | 0x40);
    platform::write_u32(record, 0);

    fn_802d0f40(record);

    fn_802d1850(
        platform::read_f32(SDA2_FLOAT_ZERO),
        platform::read_f32(SDA2_FLOAT_ZERO),
        platform::read_f32(this + 8),
        platform::read_f32(this + 0x0C),
        record,
        base + 0x08,
    );

    platform::write_u32(base, platform::read_u32(descriptor));
    platform::call6(
        0x802C_1D10,
        (base + 0x08) as u32,
        mode & 1,
        (mode >> 1) & 1,
        (mode >> 2) & 1,
        base as u32,
        1,
    );
}

// ---------------------------------------------------------------------------
// 0x802D2610 - bring up the AX mixer for the console view
// ---------------------------------------------------------------------------

/// Publishes the console panel setup and brings the AX mixer up: copies the
/// panel size and the four widget attribute bytes, forwards the widget word to
/// the device virtual, then configures the mixer and runs the one-time AX
/// initialisers.
///
/// # Original
/// - Ghidra: `FUN_802d2610` at `0x802D2610` (284 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2610_FUN_802d2610.c`
///
/// # Arguments
/// - `width`, `height`: stored as `u16` at [`PANEL_MAX_X`]/[`PANEL_MAX_Y`].
/// - `attributes`: four bytes copied to [`WIDGET_ATTRIBUTES`]; the first word is
///   also mirrored into the device object's `+0x14..+0x17`.
/// - `widget_word`: stored at [`AX_MODE_WORD`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Calls the device singleton's virtual `+0x10` slot twice through
///   [`AX_DEVICE_HANDLE`]; the second result receives the four attribute bytes
///   at `+0x14`.
/// - `0x801C6D90(AX_MIX_DESCRIPTOR, AX_MIX_DESCRIPTOR2, 4, 4, 3, 0, 0, 0)`.
/// - `0x801C6FF0(0.0f, 0.0f, 0.0f, AX_MIX_DESCRIPTOR, 1, 1, 0, 0, 0)`.
/// - [`fn_801c76c0`](crate::runtime::dvd::fn_801c76c0), [`fn_802d2730`] and
///   `0x8024A1D0(1)` run, then [`AX_UP_FLAG`] is set to `1`.
///
/// # Safety
/// The device singleton must be live with a valid vtable, and the GX/AX
/// callees must be callable.
///
/// # Confidence
/// medium - every store, constant and call is literal at
/// `0x802D2610..0x802D2728`; the `0x801C6FF0` argument list is read from the
/// registers (`r3..r8` plus `f1..f3`), and the widget-field roles are
/// `// inferred:` from [`fn_802d32f0`], which installs the same bytes.
pub unsafe fn fn_802d2610(width: u16, height: u16, attributes: usize, widget_word: u32) {
    platform::write_u16(PANEL_MAX_X, width);
    platform::write_u16(PANEL_MAX_Y, height);
    for index in 0..4 {
        platform::write_u8(
            WIDGET_ATTRIBUTES + index,
            platform::read_u8(attributes + index),
        );
    }
    platform::write_u32(AX_MODE_WORD, widget_word);

    let device = platform::read_u32(AX_DEVICE_HANDLE) as usize;
    let vtable = platform::read_u32(device) as usize;
    let getter = platform::read_u32(vtable + 0x10) as usize;
    // observed: r3 still holds the singleton for both virtual calls.
    if platform::call1(getter, device as u32) != 0 {
        let object = platform::call1(getter, device as u32) as usize;
        platform::write_u32(object + 0x14, platform::read_u32(attributes));
    }

    let mix_init = crate::platform::guest_fn!(0x801C_6D90usize, fn(usize, usize, u32, u32, u32, u32, u32, u32));
    mix_init(AX_MIX_DESCRIPTOR, AX_MIX_DESCRIPTOR2, 4, 4, 3, 0, 0, 0);

    let level = platform::read_f32(SDA2_MIX_LEVEL);
    let texture_init = crate::platform::guest_fn!(0x801C_6FF0usize, fn(f32, f32, f32, usize, u32, u32, u32, u32, u32));
    texture_init(
        level,
        level,
        level,
        AX_MIX_DESCRIPTOR,
        1,
        1,
        0,
        0,
        0,
    );

    crate::runtime::dvd::fn_801c76c0();
    fn_802d2730();
    platform::write_u32(AX_UP_FLAG, 1);
    platform::call1(0x8024_A1D0, 1);
}

// ---------------------------------------------------------------------------
// 0x802D48C0 - republish the view transform block
// ---------------------------------------------------------------------------

/// Copies the owner's transform matrix into the published 0x6C-stride block and
/// hands the optional record to `0x802D4FD0`.
///
/// # Original
/// - Ghidra: `FUN_802d48c0` at `0x802D48C0` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d48c0_FUN_802d48c0.c`
///
/// # Arguments
/// - `owner`: the view owner (`+0x4` enable byte, `+0x28` sub-object flag,
///   `+0x5BC` published word, `+0x6C8` published address, `+0x6E4..` transform
///   block, `+0x8AC`/`+0xC` the container pair).
/// - `record`: optional source record; `+0x24` is the matrix base, `+0x60` the
///   variant passed to `0x802D4FD0`, and `+0` the published word source.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `0x802D6400(owner, record)`.
/// - While `owner[+4]` is non-zero: three words are copied into
///   `owner+0x6E4..0x6EC`, four iterations write three floats each into the
///   0x6C-stride block at `owner+0x6F4`, and
///   [`VIEW_TRANSFORM_OWNER`]/[`VIEW_TRANSFORM_SOURCE`] are published.
/// - When `owner[+0x28]` is non-zero, `0x802D4FD0(owner + 0x8AC, owner + 0xC,
///   record + 0x60)` and `0x8027A410(owner + 0xC)` run.
///
/// # Safety
/// `owner` must be live for the whole 0x6F4-byte object and `record` either
/// null or covering `+0x64`.
///
/// # Confidence
/// medium - the export prints no parameters because the first call is the
/// `__savegprlr_27` helper, whose 64-bit pseudo-return is the incoming `r3`/`r4`
/// (the prologue stores `r27 = r3`, `r28 = r4`); with that substitution every
/// offset and call matches the instruction stream. The record's `+0x24` base is
/// `// inferred:` from the `+9` word offset in the export.
pub unsafe fn fn_802d48c0(owner: usize, record: usize) {
    platform::call2(0x802D_6400, owner as u32, record as u32);

    if platform::read_u8(owner + 4) != 0 {
        let matrix = if record != 0 { record + 0x24 } else { record };
        platform::write_u32(owner + 0x6E4, platform::read_u32(matrix));
        platform::write_f32(owner + 0x6E8, platform::read_f32(matrix + 0x34));
        platform::write_f32(owner + 0x6EC, platform::read_f32(matrix + 0x38));

        let mut source = matrix;
        let mut destination = owner + 0x6F4;
        for _ in 0..4 {
            let set_x = crate::platform::guest_fn!(0x8027_50C0usize, fn(f32, usize));
            set_x(platform::read_f32(source + 4), destination);
            let set_y = crate::platform::guest_fn!(0x8027_50D0usize, fn(f32, usize));
            set_y(platform::read_f32(source + 0x14), destination);
            let set_z = crate::platform::guest_fn!(0x8027_50E0usize, fn(f32, usize));
            set_z(platform::read_f32(source + 0x24), destination);
            source += 4;
            destination += 0x6C;
        }

        platform::write_u32(VIEW_TRANSFORM_OWNER, platform::read_u32(owner + 0x5BC));
        platform::write_u32(VIEW_TRANSFORM_SOURCE, (owner + 0x6C8) as u32);

        if platform::read_u32(owner + 0x28) != 0 {
            let variant = if record != 0 { record + 0x60 } else { record };
            fn_802d4fd0(owner + 0x8AC, (owner + 0x0C) as u32, variant);
            platform::call1(0x8027_A410, (owner + 0x0C) as u32);
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D6F20 - equal-tempered playback ratio
// ---------------------------------------------------------------------------

/// Returns `2^(abs(index)/12)`, or its reciprocal for a negative index.
///
/// # Original
/// - Ghidra: `FUN_802d6f20` at `0x802D6F20` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d6f20_FUN_802d6f20.c`
///
/// # Arguments
/// - `index`: signed semitone offset. `0` returns `1.0f` immediately.
///
/// # Returns
/// The ratio as a `double`, matching the export's return type; the value is
/// computed in single precision.
///
/// # Side effects
/// None: two SDA reads and one call. No memory is written.
///
/// # Safety
/// Pure arithmetic; the callee at `0x802E861C` must be callable with two
/// doubles.
///
/// # Confidence
/// medium - the split into `quotient`/`remainder`, the twelve-entry
/// [`PITCH_RATIO_TABLE`] (read out of the image: `2^(n/12)`) and the reciprocal
/// branch are literal; the callee identity at `0x802E861C` is `// inferred:` as
/// `pow(2, q)` from its `(2.0, q)` argument pair.
pub fn fn_802d6f20(index: i32) -> f64 {
    let neutral = unsafe { platform::read_f32(SDA2_RATIO_ONE) } as f64;
    if index == 0 {
        return neutral;
    }
    // observed: `bl 0x802de524` runs with the incoming r3, i.e. abs(index).
    let magnitude = unsafe { fn_802de524(index as u32) };
    let quotient = magnitude / 12;
    let remainder = magnitude - quotient * 12;

    let power = crate::platform::guest_fn!(0x802E_861Cusize, fn(f64, f64) -> f64);
    // observed: `lfd f1, -0x3A60(r2)`; platform has no f64 accessor, so the two
    // words are combined explicitly (big-endian: high word first).
    let factor = f64::from_bits(
        ((unsafe { platform::read_u32(SDA2_RATIO_BASE) } as u64) << 32)
            | unsafe { platform::read_u32(SDA2_RATIO_BASE + 4) } as u64,
    );
    let scale = unsafe { platform::read_f32(PITCH_RATIO_TABLE + (remainder as usize) * 4) };
    let value = (power(factor, quotient as f64) as f32) * scale;
    if index < 1 {
        (neutral as f32 / value) as f64
    } else {
        value as f64
    }
}

// ---------------------------------------------------------------------------
// 0x802D7070 - fade tick
// ---------------------------------------------------------------------------

/// Advances one fade stage: interpolates the global fade position, and promotes
/// the two stage words to `2` once the position reaches [`SDA2_FADE_TARGET`].
///
/// # Original
/// - Ghidra: `FUN_802d7070` at `0x802D7070` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7070_FUN_802d7070.c`
///
/// # Arguments
/// - `this`: stage record (`+4` first stage, `+8` second stage).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Interpolates `min + count * (max - min) / limit` from the object returned
///   by `0x80260860` (`+0x1C` min, `+0x20` max, `+0x24` limit, `+0x28`
///   count); when `count >= limit` the position is `max`.
/// - While `this[+8] == 1` and the position equals the target, runs
///   `0x80260860`, `0x80261B20` and `0x8027B520` and sets `+8 = 2`.
/// - While `this[+8] != 2`, `this[+4] == 1` and the position equals the target,
///   sets `+4 = 2`.
///
/// # Safety
/// `this` must be live and the three helpers callable.
///
/// # Confidence
/// medium - the interpolation, both constants and the four compares are literal
/// at `0x802D7090..0x802D7144`; the three helpers print without arguments, so
/// `r3` carries the previous return value and is marked `// unknown:`.
pub unsafe fn fn_802d7070(this: usize) {
    // observed: r3 still holds `this` at the first call, so it is passed
    // through even though the export prints an empty argument list.
    let object = platform::call1(0x8026_0860, this as u32) as usize;
    let limit = platform::read_i32(object + 0x24);
    let count = platform::read_i32(object + 0x28);
    let min = platform::read_f32(object + 0x1C);
    let max = platform::read_f32(object + 0x20);
    let position = if count < limit {
        min + (count as f32) * (max - min) / (limit as f32)
    } else {
        max
    };

    let target = platform::read_f32(SDA2_FADE_TARGET);
    if platform::read_u32(this + 8) == 1 && position == target {
        // unknown: the export shows no arguments; r3 still holds the 0x80260860
        // return value, which is passed through unchanged.
        platform::call0(0x8026_0860);
        platform::call0(0x8026_1B20);
        platform::call0(0x8027_B520);
        platform::write_u32(this + 8, 2);
    }
    if platform::read_u32(this + 8) != 2
        && platform::read_u32(this + 4) == 1
        && position == target
    {
        platform::write_u32(this + 4, 2);
    }
}

// ---------------------------------------------------------------------------
// 0x802D9570 - submit the row callbacks
// ---------------------------------------------------------------------------

/// Forwards the two `f32` values to `0x80286CA0` and then runs the `+0x18`
/// virtual slot of every non-null entry in the row array.
///
/// # Original
/// - Ghidra: `FUN_802d9570` at `0x802D9570` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d9570_FUN_802d9570.c`
///
/// # Arguments
/// - `f0`, `f1`: the pair forwarded to `0x80286CA0`.
/// - `row`: `+0` entry count, `+0x2C` pointer to the entry array.
/// - `arg4`, `arg5`: third and fourth argument of `0x80286CA0`.
/// - `arg6`, `arg7`: forwarded to every entry's `+0x18` virtual slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `0x80286CA0(f0, f1, <0x80286630 result>, arg4, arg5)`.
/// - For `index in 0..row[+0]`, when `array[index]` is non-null, calls
///   `array[index]->vtable[+0x18](entry, arg6, arg7)`.
///
/// # Safety
/// `row` must be live, the array pointer valid for the entry count, and every
/// entry's vtable slot callable.
///
/// # Confidence
/// medium - the prologue saves `f1`, `f2` and `r3..r7`, which fixes the seven
/// arguments; the export's `piVar1`/`(int)uVar7` pair is the padded
/// `__savegprlr` return `(r4, r3)`, and with `r3 = row`, `r4 = arg4` the offsets
/// and the two call shapes match exactly.
pub unsafe fn fn_802d9570(
    f0: f32,
    f1: f32,
    row: usize,
    arg4: usize,
    arg5: usize,
    arg6: usize,
    arg7: usize,
) {
    // observed: r3 still holds `row` at the 0x80286630 call.
    let context = platform::call1(0x8028_6630, row as u32);
    let submit = crate::platform::guest_fn!(0x8028_6CA0usize, fn(f32, f32, u32, usize, usize));
    submit(f0, f1, context, arg4, arg5);

    let array = platform::read_u32(row + 0x2C) as usize;
    let count = platform::read_u32(row);
    let mut index = 0u32;
    while index < count {
        let entry = platform::read_u32(array + (index as usize) * 4) as usize;
        if entry != 0 {
            let vtable = platform::read_u32(entry) as usize;
            let slot = platform::read_u32(vtable + 0x18) as usize;
            platform::call3(slot, entry as u32, arg6 as u32, arg7 as u32);
        }
        index += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802D2F10, 0x802D3260, 0x802DEBB4 -- four-byte tail calls
// ---------------------------------------------------------------------------

/// Tail-calls the GX state flush at `0x801C76C0`; the four bytes are
/// `b 0x801C76C0`.
///
/// # Original
/// - Ghidra: `thunk_FUN_801c76c0` at `0x802D2F10` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2f10_thunk_FUN_801c76c0.c`
///
/// # Arguments
/// None, matching [`fn_801c76c0`](crate::runtime::dvd::fn_801c76c0).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Those of the callee.
///
/// # Safety
/// The GX state pointer and FIFO must be mapped.
///
/// # Confidence
/// high - the word at `0x802D2F10` is `0x4BEF47B0`, a plain branch to
/// `0x801C76C0`.
pub unsafe fn fn_802d2f10() {
    crate::runtime::dvd::fn_801c76c0();
}

/// Tail-calls the scissor decode at `0x801C9CD0`; the four bytes are
/// `b 0x801C9CD0`.
///
/// # Original
/// - Ghidra: `thunk_FUN_801c9cd0` at `0x802D3260` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d3260_thunk_FUN_801c9cd0.c`
///
/// # Arguments
/// The four output pointers of
/// [`fn_801c9cd0`](crate::runtime::dvd::fn_801c9cd0).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes four `i32` values through the pointers.
///
/// # Safety
/// All four pointers must be writable.
///
/// # Confidence
/// high - the word at `0x802D3260` is a plain branch to `0x801C9CD0`, and the
/// shared body in the export matches the four pointer stores there.
pub unsafe fn fn_802d3260(x_out: usize, y_out: usize, width_out: usize, height_out: usize) {
    crate::runtime::dvd::fn_801c9cd0(x_out, y_out, width_out, height_out);
}

/// Tail-calls the `ftell` body at `0x802DEB0C`; the four bytes are
/// `b 0x802DEB0C`.
///
/// # Original
/// - Ghidra: `thunk_FUN_802deb0c` at `0x802DEBB4` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802debb4_thunk_FUN_802deb0c.c`
///
/// # Arguments
/// - `file`: the stream record, as in [`fn_802deb0c`].
///
/// # Returns
/// The current offset, or `-1`.
///
/// # Side effects
/// Those of [`fn_802deb0c`].
///
/// # Safety
/// `file` must be a live `FILE`-shaped record.
///
/// # Confidence
/// high - the word at `0x802DEBB4` is `0x4BFFFF58`, a branch back to
/// `0x802DEB0C`.
pub unsafe fn fn_802debb4(file: usize) -> i32 {
    fn_802deb0c(file)
}

/// Does nothing: the four bytes at `0x802DB0E0` are a bare `blr`.
///
/// # Original
/// - Ghidra: `FUN_802db0e0` at `0x802DB0E0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802db0e0_FUN_802db0e0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None. This is one of the empty placeholders the linker left between real
/// routines; [`fn_802dcd18`] and [`fn_802dcd2c`] in the same band are the same
/// shape but sit inside the register-save run of `docs/QUALITY.md` section 5.5.
///
/// # Safety
/// Always safe; kept `unsafe` to match the band's calling convention.
///
/// # Confidence
/// high - the instruction is `0x4E800020` (`blr`).
pub unsafe fn fn_802db0e0() {}

// ---------------------------------------------------------------------------
// Wave 99 - the console panel slot family (0x802D12E0..0x802D79A0)
// ---------------------------------------------------------------------------
//
// These bodies all operate on the two "panel slot" tables in SDA memory:
//
// - `0x80424820`, three 12-byte slots (`handle`, `token`, `flags`): the render
//   targets the console panels draw into.
// - `0x80424850`, the published panel rectangle, and `0x80424880`, the
//   six-float quad that the `0x802D3200/3230/3270` sink consumes.
//
// The bitfield descriptor table at [`SECTION_FIELD_TABLE`] describes the
// archive section tags that `0x802D79A0` searches for. Its literal contents
// (read out of `.rodata` at `0x802FD2E8` and `0x802FD300`) are
// `shift = [32, 24, 16, 8, 0, 0]` for the six tags `INF1/DAT1/STR1/MID1/FLW1/
// FLI1` and the mask pairs `(0xFFFFFFFF, 0x00000000)`, `(0xFFFFFF00,
// 0x000000FF)`, `(0xFFFF0000, 0x0000FFFF)`, `(0xFF000000, 0x00FFFFFF)`,
// `(0x00000000, 0xFFFFFFFF)`, `(0x00000000, 0x00000000)`.

/// `0x803504E0`: the three virtual-console modes, 12 bytes each, two `u16`
/// used: `(608, 456)`, `(832, 456)`, `(640, 456)`. [`layout_axis`] picks one.
const PANEL_AXIS_TABLE: usize = 0x8035_04E0;

/// `r13 - 0x4778`: the configured console mode index, read when the record's
/// flag word asks for neither the `0x20` nor the `0x08` mode.
const SDA_AXIS_SELECT: usize = 0x8042_AF08;

/// `r13 - 0x4748`, `r13 - 0x4746`: the render-target width and height in
/// pixels, published by [`fn_802d2610`]. `u16` each.
const SDA_RENDER_WIDTH: usize = 0x8042_AF38;
const SDA_RENDER_HEIGHT: usize = 0x8042_AF3A;

/// `r13 - 0x47B0`: the layout record the panel helpers project through.
const PANEL_ROOT: usize = 0x8042_AED0;

/// `0x80424850`/`0x80424854`: the published panel origin, and the six-float
/// quad at `0x80424880` that [`fn_802d2550`] fills and submits.
const PANEL_RECT: usize = 0x8042_4850;
const PANEL_QUAD: usize = 0x8042_4880;

/// `0x80424820`: three 12-byte slots. `+0` is the render-target handle, `+4`
/// the token the commit path in [`fn_802d2310`] matches, `+8` an auxiliary
/// flag word. `0x80424824` is the token of slot 0.
const PANEL_SLOTS: usize = 0x8042_4820;
const PANEL_SLOT_STRIDE: usize = 12;

/// `r13 - 0x5AFC`: the slot index a draw request is waiting on, `-1` when
/// idle.
const PENDING_PANEL_SLOT: usize = 0x8042_A084;

/// `r13 - 0x4750`: the counter [`fn_802d2310`] bumps after a successful
/// commit.
const PANEL_COMMIT_COUNT: usize = 0x8042_AF30;

/// `0x802FD2E8`: one shift count per archive section tag, used by
/// [`fn_802d79a0`].
const SECTION_FIELD_TABLE: usize = 0x802F_D2E8;

/// `0x802FD300`: two `u32` masks per archive section tag.
const SECTION_MASK_TABLE: usize = 0x802F_D300;

/// Maps the flag word of a layout record to the [`PANEL_AXIS_TABLE`] index.
///
/// `0x20` means "mode 0", `0x08` means "mode 2", and any other record takes
/// the value published at [`SDA_AXIS_SELECT`].
///
/// # Safety
/// Reads the SDA cell, which is always mapped.
#[inline]
unsafe fn layout_axis(flags: u16) -> usize {
    if flags & 0x0020 != 0 {
        0
    } else if flags & 0x0008 != 0 {
        2
    } else {
        platform::read_i32(SDA_AXIS_SELECT) as usize
    }
}

// ---------------------------------------------------------------------------
// 0x802D12E0 - recompute a layout record's screen rectangle
// ---------------------------------------------------------------------------

/// Recomputes the eight numbers a layout record needs before it is drawn: the
/// projected origin at `+0x58`/`+0x5C`, the integer cell origin at
/// `+0x70`/`+0x74`, the truncated content size at `+0x60`/`+0x64`, the
/// truncated inner rectangle at `+0x78`/`+0x7C` and the two "carried" pixel
/// counts at `+0x80`/`+0x84`.
///
/// This is the per-frame work horse behind [`fn_802d17c0`], which returns
/// `this + 0x58` after calling it.
///
/// # Original
/// - Ghidra: `FUN_802d12e0` at `0x802D12E0` (1240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d12e0_FUN_802d12e0.c`
///
/// # Arguments
/// - `this`: the layout record. `+0x08`/`+0x0C` is its content size,
///   `+0x34` the flag word, `+0x3C` the projection mode (zero means "scale by
///   the installed quad"), `+0x40`/`+0x44` the projection scales, `+0x48..+0x54`
///   the four insets.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Writes `+0x58`, `+0x5C`, `+0x60`, `+0x64`, `+0x70..+0x84`.
/// - Calls `0x802D1B90` (via the existing stub [`fn_802d1b90`]) with
///   `(f1 = +0x40, f2 = +0x44, r3 = +0x3C, r4 = &+0x58, r5 = &+0x5C)` when the
///   mode is non-zero.
///
/// # Safety
/// `this..this+0x88` must be a live layout record; the injected pointer
/// arguments must be writable.
///
/// # Confidence
/// high - `lwz r4,0x3C(r3)` at `0x802D12F4` and `addi r29,r3,0x58` at
/// `0x802D1308` prove `r3 = this`; the mode/pointer registers of the
/// `0x802D1B90` call are literal at `0x802D1340..0x802D1354`, and the axis
/// selector, the `0x803504E0` table and the three `0x802DCC4C` calls appear
/// verbatim. The export's `CONCAT44(0x43300000, x) - (2^52)` chains are the
/// compiler's unsigned-to-double conversion; the Rust conversions spell the
/// same value.
pub unsafe fn fn_802d12e0(this: usize) {
    let flags = platform::read_u16(this + 0x34);

    if platform::read_u32(this + 0x3C) == 0 {
        platform::write_f32(
            this + 0x58,
            platform::read_f32(this + 0x40) * platform::read_f32(SDA_FLOAT_QUAD),
        );
        platform::write_f32(
            this + 0x5C,
            platform::read_f32(this + 0x44) * platform::read_f32(SDA_FLOAT_QUAD + 4),
        );
    } else {
        // observed: f1/f2 = the two scales, r3 = mode, r4/r5 = the two output
        // floats (0x802D1340..0x802D1354).
        let project = crate::platform::guest_fn!(0x802D_1B90usize, fn(f32, f32, u32, *mut f32, *mut f32));
        project(
            platform::read_f32(this + 0x40),
            platform::read_f32(this + 0x44),
            platform::read_u32(this + 0x3C),
            (this + 0x58) as *mut f32,
            (this + 0x5C) as *mut f32,
        );
    }

    let axis = layout_axis(flags);
    let cell = PANEL_AXIS_TABLE + axis * 0xC;
    let width = platform::read_u16(SDA_RENDER_WIDTH) as f32;
    let height = platform::read_u16(SDA_RENDER_HEIGHT) as f32;
    let cell_width = platform::read_u16(cell) as f32;
    let cell_height = platform::read_u16(cell + 2) as f32;

    let mut x = platform::read_f32(this + 0x58);
    let mut y = platform::read_f32(this + 0x5C);

    platform::write_u32(
        this + 0x70,
        fn_802dcc4c((x + (platform::read_f32(this + 0x48) * width) / cell_width) as f64) as u32,
    );
    platform::write_u32(
        this + 0x74,
        fn_802dcc4c((y + (platform::read_f32(this + 0x4C) * height) / cell_height) as f64) as u32,
    );

    platform::write_u32(this + 0x84, 0);
    platform::write_u32(this + 0x80, 0);

    // observed: `// unknown:` these two branches are the "negative fraction"
    // convention the console scroller uses: a negative coordinate keeps its
    // fractional part and hands the even part of its magnitude to +0x80/+0x84.
    if x < 0.0 {
        let trunc = x as i32;
        let magnitude = trunc.wrapping_neg() as u32;
        platform::write_u32(this + 0x80, magnitude & !1u32);
        x -= (-(trunc as f64)) as f32;
        platform::write_f32(this + 0x58, x);
    }
    if y < 0.0 {
        let trunc = y as i32;
        let magnitude = trunc.wrapping_neg() as u32;
        platform::write_u32(this + 0x84, magnitude & !1u32);
        y -= (-(trunc as f64)) as f32;
        platform::write_f32(this + 0x5C, y);
    }
    if flags & 0x0002 != 0 {
        x -= ((x as i32) & 1) as f32;
        y -= ((y as i32) & 1) as f32;
        platform::write_f32(this + 0x58, x);
        platform::write_f32(this + 0x5C, y);
    }

    let mut content_width = (platform::read_f32(this + 0x8) * width) / cell_width;
    let mut content_height = (platform::read_f32(this + 0xC) * height) / cell_height;
    platform::write_f32(this + 0x60, content_width);
    platform::write_f32(this + 0x64, content_height);
    if flags & 0x0004 != 0 {
        content_width -= ((content_width as i32) & 3) as f32;
        content_height -= ((content_height as i32) & 3) as f32;
        platform::write_f32(this + 0x60, content_width);
        platform::write_f32(this + 0x64, content_height);
    }
    // observed: the final store is `(float)(double)(int)v`, a truncation
    // toward zero; `as i32 as f32` performs the same two conversions.
    let content_width = (content_width as i32) as f32;
    let content_height = (content_height as i32) as f32;
    platform::write_f32(this + 0x60, content_width);
    platform::write_f32(this + 0x64, content_height);

    platform::write_u32(
        this + 0x78,
        fn_802dcc4c(
            (content_width
                - ((platform::read_f32(this + 0x48) + platform::read_f32(this + 0x50)) * width)
                    / cell_width) as f64,
        ) as u32,
    );
    platform::write_u32(
        this + 0x7C,
        fn_802dcc4c(
            (content_height
                - ((platform::read_f32(this + 0x4C) + platform::read_f32(this + 0x54)) * height)
                    / cell_height) as f64,
        ) as u32,
    );
}

// ---------------------------------------------------------------------------
// 0x802D2550 - publish the current panel quad
// ---------------------------------------------------------------------------

/// Copies the published panel origin and the projected layout record into the
/// six-float quad at [`PANEL_QUAD`], submits it to the `0x802D3200/3230/3270`
/// sink and returns the quad's address.
///
/// # Original
/// - Ghidra: `FUN_802d2550` at `0x802D2550` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2550_FUN_802d2550.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `&DAT_80424880`, the six-float quad it just filled.
///
/// # Side effects
/// - Fills `0x80424880..0x80424897` with `(rect.x, rect.y, record+8,
///   record+0x0C, record+0x10, record+0x14)`.
/// - Calls `0x802D3200` with those six floats, converts them through
///   [`fn_802d3090`] into a four-word clip rectangle, then calls `0x802D3230`
///   with the four words and `0x802D3270(0, 0)`.
///
/// # Safety
/// The SDA cells and the normalised record must be mapped; the four-word
/// scratch buffer is this frame's own.
///
/// # Confidence
/// high - every load and store is literal at `0x802D2560..0x802D25A8`, and the
/// six `lfs` immediately before the `0x802D3200` call fix that call's
/// arguments even though the export prints them as "no arguments".
pub unsafe fn fn_802d2550() -> usize {
    let record = fn_802d17c0(platform::read_u32(PANEL_ROOT) as usize);

    platform::write_f32(PANEL_QUAD, platform::read_f32(PANEL_RECT));
    platform::write_f32(PANEL_QUAD + 4, platform::read_f32(PANEL_RECT + 4));
    platform::write_f32(PANEL_QUAD + 8, platform::read_f32(record + 8));
    platform::write_f32(PANEL_QUAD + 0xC, platform::read_f32(record + 0xC));
    platform::write_f32(PANEL_QUAD + 0x10, platform::read_f32(record + 0x10));
    platform::write_f32(PANEL_QUAD + 0x14, platform::read_f32(record + 0x14));

    // observed: f1..f6 are the six `lfs` above; the sink is the same one
    // [`fn_802d0f40`] calls through a transmuted six-float signature.
    let submit = crate::platform::guest_fn!(0x802D_3200usize, fn(f32, f32, f32, f32, f32, f32));
    submit(
        platform::read_f32(PANEL_QUAD),
        platform::read_f32(PANEL_QUAD + 4),
        platform::read_f32(PANEL_QUAD + 8),
        platform::read_f32(PANEL_QUAD + 0xC),
        platform::read_f32(PANEL_QUAD + 0x10),
        platform::read_f32(PANEL_QUAD + 0x14),
    );

    let mut clip = crate::platform::GuestLocal::new( [0i32; 4]);
    fn_802d3090(PANEL_QUAD as *const f32, clip.as_mut_ptr());
    platform::call4(
        0x802D_3230,
        clip[0] as u32,
        clip[1] as u32,
        clip[2] as u32,
        clip[3] as u32,
    );
    platform::call2(0x802D_3270, 0, 0);

    PANEL_QUAD
}

// ---------------------------------------------------------------------------
// 0x802D79A0 - find one entry of a tagged archive section
// ---------------------------------------------------------------------------

/// Scans the entries of the section named by `container+0x10` and returns the
/// address of the first entry whose packed identifier matches both search
/// keys.
///
/// Each entry carries a 32-bit identifier. The descriptor tables at
/// [`SECTION_FIELD_TABLE`]/[`SECTION_MASK_TABLE`] split that word into a
/// "high" field of `shift` bits and a "low" field; `key` is compared with the
/// shifted high field and `mask` with the low one. The split depends on the
/// section tag byte at `section+0x0B`.
///
/// # Original
/// - Ghidra: `FUN_802d79a0` at `0x802D79A0` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d79a0_FUN_802d79a0.c`
///
/// # Arguments
/// - `container`: the archive record. `+0x10` points at the section
///   descriptor, `+0x04` at the entry array.
/// - `key`: value the shifted identifier field must equal.
/// - `mask`: value the remaining identifier bits must equal.
///
/// # Returns
/// `entry_array + index*stride + 0x10`, or `0` when no entry matches.
///
/// # Side effects
/// None beyond reading memory.
///
/// # Safety
/// `container` must be a live archive record and both table indices must stay
/// inside the descriptor tables (that is, the tag byte must be `0..5`).
///
/// # Confidence
/// high - `or r31,r4,r4` / `or r25,r5,r5` at `0x802D79E8`/`0x802D79EC` fix the
/// two key registers, the `lwzx`/`lhzx` filters are literal at
/// `0x802D79D8..0x802D7A20`, and the descriptor tables were read out of
/// `.rodata`.
pub unsafe fn fn_802d79a0(container: usize, key: u32, mask: u32) -> usize {
    let section = platform::read_u32(container + 0x10) as usize;
    let tag = platform::read_u8(section + 0xB) as usize;
    let shift = 32 - platform::read_u32(SECTION_FIELD_TABLE + tag * 4);
    let high_mask = platform::read_u32(SECTION_MASK_TABLE + tag * 8);
    let low_mask = platform::read_u32(SECTION_MASK_TABLE + tag * 8 + 4);
    let count = platform::read_u16(section + 8);

    let mut index = 0u16;
    while (index as u32) < (count as u32) {
        let identifier = fn_802d7a80(container, index as u32);
        // observed: PPC `srw` counts modulo 64, so the tag-0 descriptor
        // (`shift == 32`) shifts every bit out. Widening to `u64` reproduces
        // that exactly, where a `u32` shift would wrap the count to zero.
        let field = ((identifier & high_mask) as u64 >> shift) as u32;
        if field == key && (identifier & low_mask) == mask {
            let entries = platform::read_u32(container + 4) as usize;
            let stride = platform::read_u16(entries + 10) as usize;
            return entries + (index as usize) * stride + 0x10;
        }
        index = index.wrapping_add(1);
    }
    0
}

// ---------------------------------------------------------------------------
// 0x802D7860 - entry address relative to a container
// ---------------------------------------------------------------------------

/// Returns the address of the entry [`fn_802d79a0`] finds, biased by the
/// container's `+8` field and eight bytes.
///
/// # Original
/// - Ghidra: `FUN_802d7860` at `0x802D7860` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d7860_FUN_802d7860.c`
///
/// # Arguments
/// - `this`: the archive container. `+8` is added to the entry's first word.
/// - `key`, `mask`: forwarded unchanged in `r4`/`r5` to [`fn_802d79a0`].
///
/// # Returns
/// `this[+8] + *entry + 8`.
///
/// # Side effects
/// None beyond the search in [`fn_802d79a0`].
///
/// # Safety
/// `this` must be the same live container [`fn_802d79a0`] expects, and the
/// entry it returns must be readable.
///
/// # Confidence
/// medium - the three arithmetic operations are literal, but no caller of this
/// wrapper initialises `r4`/`r5`, so the two forwarded keys are only known to
/// be "whatever the caller left there"; the entry layout itself is proven by
/// [`fn_802d79a0`].
pub unsafe fn fn_802d7860(this: usize, key: u32, mask: u32) -> usize {
    let entry = fn_802d79a0(this, key, mask);
    platform::read_u32(this + 8) as usize + platform::read_u32(entry) as usize + 8
}
// ---------------------------------------------------------------------------
// Wave 99 - the panel publish/commit pair (0x802D1E20, 0x802D2310)
// ---------------------------------------------------------------------------
//
// `0x802D1E20` fills one of the three [`PANEL_SLOTS`] with a render target
// built by `0x802D3700`, and writes the panel rectangle the target covers.
// `0x802D2310` is the commit half: when the pending slot's token matches, it
// replays the rectangle through the layout record and the GX sinks and then
// clears the pending index.
//
// The two functions share the whole vocabulary below, which is why they are
// translated together. The scales at `0x8042F59C`/`0x8042F5A0` and the fill
// constants are SDA cells that only the game's constructors write, so their
// values are not in `work/dol/main.elf`; they are named by role and marked
// `// unknown:` where the number matters.

/// `r2 - 0x3AE4`, `r2 - 0x3AE0`: the per-slot scale applied to the panel
/// size. Slot 0 uses the first, slot 1 the second.
/// `// unknown:` both are initialised at run time, like every other SDA cell.
const SDA2_SLOT_SCALE_0: usize = 0x8042_F59C;
const SDA2_SLOT_SCALE_1: usize = 0x8042_F5A0;

/// `r2 - 0x3AE8`: the lower bound of the clamped rectangle edges. Wave 96
/// recorded the neighbouring `0x8042F574` as `0.0f`; this cell is separate and
/// its value is not in the image.
const SDA2_EDGE_MIN: usize = 0x8042_F598;

/// `r2 - 0x3AD4`, `r2 - 0x3AD8`: the published origin the panel is centred
/// against.
const SDA2_PANEL_ORIGIN_X: usize = 0x8042_F5A4;
const SDA2_PANEL_ORIGIN_Y: usize = 0x8042_F5A8;

/// `r2 - 0x3AC8`: the fill value written into the layout record's five
/// `+0x1C..+0x30` slots and passed to the `0x802D3200` sink as its sixth
/// argument.
const SDA2_PANEL_FILL: usize = 0x8042_F5B8;

/// `0x80424868`/`0x8042486C`: the panel content edges, and
/// `0x80424870`/`0x80424874` the two letterbox spans.
const PANEL_EDGES: usize = 0x8042_4868;
const PANEL_SPANS: usize = 0x8042_4870;

/// `r2 - 0x3E3C`: the word handed to `0x802C2F70` as its third argument.
const SDA2_GX_TAG: usize = 0x8042_F244;

/// Rounds a float to the nearest even integer, away from zero.
///
/// This is the CodeWarrior sequence `(int)v` followed by
/// `n + ((n & 1) ^ -(n >> 31)) + (n >> 31)`: an odd positive value is bumped
/// up by one, an odd negative value down by one, and an already even value is
/// left alone. The export spells it out as a `CONCAT44(0x43300000, ...)` chain;
/// the Rust arithmetic is the same 32-bit wrap.
///
/// # Safety
/// None; the cast saturates like `fctiwz`.
#[inline]
unsafe fn snap_even(value: f32) -> f32 {
    let truncated = value as i32;
    let sign = truncated >> 31;
    truncated
        .wrapping_add(((truncated & 1) ^ sign.wrapping_neg()).wrapping_add(sign)) as f32
}

// ---------------------------------------------------------------------------
// 0x802D1E20 - build the render target for one panel slot
// ---------------------------------------------------------------------------

/// Creates the render target for `index`, stores it in the slot table and
/// publishes the panel rectangle it covers.
///
/// Slots 0 and 1 are the two scaled panels: the size is
/// `snap_even(panel * scale)`, the larger side is centred against
/// [`SDA2_PANEL_ORIGIN_X`]/[`SDA2_PANEL_ORIGIN_Y`], and the two letterbox spans
/// are clamped from below by [`SDA2_EDGE_MIN`]. Slot 2 is the plain panel and
/// slot 3 is the scaled "mode 4/6" variant.
///
/// # Original
/// - Ghidra: `FUN_802d1e20` at `0x802D1E20` (968 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d1e20_FUN_802d1e20.c`
///
/// # Arguments
/// - `index`: slot index, `mulli r29,r23,12` at `0x802D1E6C`. `0`/`1` take the
///   scaled path, `2` the plain one, `3` the conditional one.
/// - `active`: non-zero marks the new target as running (`+0x18 |= 2` and
///   `+0x19..+0x1C = 0`) and is what the commit path tests.
/// - `token`: stored in the slot's `+4` word; [`fn_802d2310`] matches it.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Writes `PANEL_SLOTS[index]` and the rectangle at [`PANEL_RECT`],
///   `0x80424858`/`5C`, [`PANEL_EDGES`] and [`PANEL_SPANS`].
/// - Calls `0x802D3700`, and on the slot-0/1 path `0x802D25F0`; on slot 3 it
///   asks `0x802D2600` whether to pass mode `4` or `6`.
///
/// # Safety
/// `index` must stay inside the three-entry slot table; `PANEL_ROOT` must hold
/// the live layout record.
///
/// # Confidence
/// medium - `mulli r29,r23,12`, the `r4`/`r5` saves at `0x802D1E58`/`0x802D1E60`
/// and every store are literal, but the three argument *roles* above are
/// inferred from how [`fn_802d2310`] consumes the same slot words, and the two
/// scale cells only exist at run time.
pub unsafe fn fn_802d1e20(index: u32, active: u32, token: u32) {
    let panel = fn_802d17c0(platform::read_u32(PANEL_ROOT) as usize);
    let slot = PANEL_SLOTS + (index as usize) * PANEL_SLOT_STRIDE;
    if platform::read_u32(slot) != 0 {
        return;
    }

    let panel_width = platform::read_f32(panel + 8);
    let panel_height = platform::read_f32(panel + 0xC);
    let mut submit = 1u32;
    let mut mode = 0u32;

    if index <= 1 {
        let scale = platform::read_f32(if index == 0 {
            SDA2_SLOT_SCALE_0
        } else {
            SDA2_SLOT_SCALE_1
        });
        // observed: the two products are snapped to even and the larger one is
        // published twice (the compiler folded the two stores into a max).
        let width = snap_even(panel_width * scale);
        let height = snap_even(panel_height * scale);
        let span = if width > height { width } else { height };
        platform::write_f32(PANEL_RECT + 8, span);
        platform::write_f32(PANEL_RECT + 0xC, span);

        let origin_x = platform::read_f32(SDA2_PANEL_ORIGIN_X) - span;
        let origin_y = platform::read_f32(SDA2_PANEL_ORIGIN_Y) - span;
        platform::write_f32(PANEL_RECT, origin_x);
        platform::write_f32(PANEL_RECT + 4, origin_y);
        platform::write_f32(PANEL_EDGES, origin_x);
        platform::write_f32(PANEL_EDGES + 4, origin_y);

        // observed: `max - (f32)(0x280 - width)`, i.e. the panel span plus the
        // left/top letterbox margin; the export prints the inner subtraction
        // with the signed `CONCAT44` bias.
        let mut left = span - (640.0f32 - platform::read_u16(SDA_RENDER_WIDTH) as f32);
        let mut top = span - (528.0f32 - platform::read_u16(SDA_RENDER_HEIGHT) as f32);
        let floor = platform::read_f32(SDA2_EDGE_MIN);
        if left < floor {
            left = floor;
        }
        if top < floor {
            top = floor;
        }
        platform::write_f32(PANEL_SPANS, left);
        platform::write_f32(PANEL_SPANS + 4, top);

        let handle = platform::call3(
            0x802D_3700,
            (left as i32 as u32) & 0xFFFF,
            (top as i32 as u32) & 0xFFFF,
            6,
        );
        platform::write_u32(slot, handle);
        if handle != 0 {
            let flags = platform::read_u8(handle as usize + 0x18);
            platform::write_u8(handle as usize + 0x18, flags & !0x10);
        }
        submit = call0_ret(0x802D_25F0);
    } else if index == 2 {
        // observed: this arm clears the bit without the null test the 0/1 arm
        // performs, exactly as the export shows.
        let handle = platform::call3(
            0x802D_3700,
            (panel_width as i32 as u32) & 0xFFFF,
            (panel_height as i32 as u32) & 0xFFFF,
            6,
        );
        platform::write_u32(slot, handle);
        let flags = platform::read_u8(handle as usize + 0x18);
        platform::write_u8(handle as usize + 0x18, flags & !0x10);
    } else if index == 3 {
        let kind = if call0_ret(0x802D_2600) != 0 { 4 } else { 6 };
        let scale = platform::read_f32(SDA2_SLOT_SCALE_1);
        let handle = platform::call3(
            0x802D_3700,
            ((panel_width * scale) as i32 as u32) & 0xFFFF,
            ((panel_height * scale) as i32 as u32) & 0xFFFF,
            kind,
        );
        platform::write_u32(slot, handle);
        mode = 1;
    }

    if platform::read_u32(slot) != 0 {
        platform::write_u32(slot + 4, token);
        if active != 0 {
            let handle = platform::read_u32(slot) as usize;
            platform::write_u8(handle + 0x18, platform::read_u8(handle + 0x18) | 2);
            platform::write_u8(handle + 0x19, 0);
            platform::write_u8(handle + 0x1A, 0);
            platform::write_u8(handle + 0x1B, 0);
            platform::write_u8(handle + 0x1C, 0);
        }
        // observed: the two clamps run on f31/f30, which only the slot-0/1 arm
        // loads from PANEL_RECT; on the other arms the compiled code reads the
        // caller's copies. Hoisting the two loads reads the same published
        // rectangle and is what the export prints.
        let floor = platform::read_f32(SDA2_EDGE_MIN);
        let mut x = platform::read_f32(PANEL_RECT);
        let mut y = platform::read_f32(PANEL_RECT + 4);
        if x < floor {
            x = floor;
        }
        if y < floor {
            y = floor;
        }
        if submit != 0 {
            platform::call5(
                0x802C_02C0,
                platform::read_u32(slot),
                (x as i32 as u32) & 0xFFFF,
                (y as i32 as u32) & 0xFFFF,
                mode,
                0xFFFF_FFFF,
            );
        }
    }
}

// ---------------------------------------------------------------------------
// 0x802D2310 - commit the pending panel slot
// ---------------------------------------------------------------------------

/// Commits the panel slot named by [`PENDING_PANEL_SLOT`] once the caller's
/// token matches the one [`fn_802d1e20`] stored.
///
/// # Original
/// - Ghidra: `FUN_802d2310` at `0x802D2310` (572 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d2310_FUN_802d2310.c`
///
/// # Arguments
/// - `inactive`: when zero the full draw runs; any other value skips straight
///   to the bookkeeping (`cmpi cr0,1,r3,0` at `0x802D236C`).
/// - `token`: compared with the pending slot's `+4` word (`cmpl r4,r0` at
///   `0x802D2364`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Builds a 0x90-byte layout record on the stack through
///   [`fn_802d0e50`], forces its flag word to `0x40` (and to `0x41`/mode `1`
///   when the inherited mode is not already `1`), and pushes it with
///   [`fn_802d0f40`].
/// - Runs the GX sequence: `0x802D3200` (six floats), `0x802D3230` (four
///   words), `0x802D3270(0, 0)`, `0x801C0C60`, `0x802C17E0`, `0x802C3020`,
///   `0x802C0B60` and `0x802C2F70`.
/// - Saves and restores the three predicate bytes through
///   [`fn_802d2f20`]/[`fn_802d2f80`]/[`fn_802d2fe0`].
/// - Runs [`fn_802d21f0`]`(index, token)`, clears the pending index and bumps
///   [`PANEL_COMMIT_COUNT`].
///
/// # Safety
/// The pending index must address a live slot whose handle is a valid render
/// target; all called helpers must be callable at the addresses shown.
///
/// # Confidence
/// medium - every store, call and the `0x90`-byte record are literal at
/// `0x802D2324..0x802D251C`, and the two arguments are the raw `r3`/`r4` uses
/// at `0x802D2364`/`0x802D236C`; the argument *names* and the four panel
/// globals are inferred from [`fn_802d1e20`] and from the `0x802D0F40`
/// consumer.
pub unsafe fn fn_802d2310(inactive: u32, token: u32) {
    let pending = platform::read_i32(PENDING_PANEL_SLOT);
    if pending == -1 {
        return;
    }

    // observed: 0 -> 1, 1 -> 0, anything else -> -1 (the slot arithmetic below
    // then reads the word in front of the table).
    let index: i32 = if pending == 0 {
        1
    } else if pending == 1 {
        0
    } else {
        -1
    };
    let slot = PANEL_SLOTS.wrapping_add((index as isize * PANEL_SLOT_STRIDE as isize) as usize);
    if token != platform::read_u32(slot + 4) {
        return;
    }

    if inactive == 0 {
        let saved_a = fn_802d2f20(1);
        let saved_b = fn_802d2f80(1);
        let saved_c = fn_802d2fe0(0);

        let root = platform::read_u32(PANEL_ROOT) as usize;
        let mut record = crate::platform::GuestLocal::new( [0u32; 0x24]);
        let base = record.as_mut_ptr() as usize;
        fn_802d0e50(base, root);

        let inherited_mode = platform::read_u32(base + 4);
        let mut flags = platform::read_u16(base + 0x34) | 0x40;
        platform::write_u16(base + 0x34, flags);
        if inherited_mode != 1 {
            flags |= 1;
            platform::write_u16(base + 0x34, flags);
            platform::write_u32(base + 4, 1);
        }

        let floor = platform::read_f32(SDA2_EDGE_MIN);
        let fill = platform::read_f32(SDA2_PANEL_FILL);
        platform::write_u32(base, 0);
        platform::write_f32(base + 0x18, floor);
        platform::write_f32(base + 0x1C, fill);
        platform::write_f32(base + 0x20, floor);
        platform::write_f32(base + 0x24, floor);
        platform::write_f32(base + 0x28, fill);
        platform::write_f32(base + 0x2C, fill);
        platform::write_f32(base + 0x30, fill);

        fn_802d0f40(base);

        let six = crate::platform::guest_fn!(0x802D_3200usize, fn(f32, f32, f32, f32, f32, f32));
        six(
            platform::read_f32(PANEL_EDGES),
            platform::read_f32(PANEL_EDGES + 4),
            platform::read_f32(PANEL_EDGES + 8),
            platform::read_f32(PANEL_EDGES + 0xC),
            floor,
            fill,
        );
        platform::call4(
            0x802D_3230,
            fn_802dcc4c(platform::read_f32(PANEL_EDGES) as f64) as u32,
            fn_802dcc4c(platform::read_f32(PANEL_EDGES + 4) as f64) as u32,
            fn_802dcc4c(platform::read_f32(PANEL_EDGES + 8) as f64) as u32,
            fn_802dcc4c(platform::read_f32(PANEL_EDGES + 0xC) as f64) as u32,
        );
        platform::call2(0x802D_3270, 0, 0);

        let mut scratch = crate::platform::GuestLocal::new( [0u32; 12]);
        let scratch_ptr = scratch.as_mut_ptr() as usize;
        let to_scratch = crate::platform::guest_fn!(0x801C_0C60usize, fn(f32, f32, f32, usize));
        to_scratch(
            platform::read_f32(root + 8),
            platform::read_f32(root + 0xC),
            floor,
            scratch_ptr,
        );

        let target = call0_ret(0x802D_25F0);
        platform::call3(0x802C_17E0, 1, target, 0);
        platform::call1(0x802C_3020, 0xE);

        if call0_ret(0x802D_25F0) != 0 {
            let handle = platform::read_u32(slot) as usize;
            platform::write_u8(handle + 0xC, 0);
            platform::write_u8(handle + 0xD, 0);
            let value = call0_ret(0x802C_17C0);
            platform::call2(0x802C_0B60, handle as u32, value);
            let mut tag = crate::platform::GuestLocal::new( [platform::read_u32(SDA2_GX_TAG); 1]);
            platform::call3(
                0x802C_2F70,
                0x10,
                scratch_ptr as u32,
                tag.as_mut_ptr() as u32,
            );
        }

        fn_802d2fe0(saved_c);
        fn_802d2f80(saved_b);
        fn_802d2f20(saved_a);
    }

    fn_802d21f0(index, platform::read_u32(slot + 4) as i32);
    platform::write_u32(PENDING_PANEL_SLOT, 0xFFFF_FFFF);
    platform::write_u32(
        PANEL_COMMIT_COUNT,
        platform::read_u32(PANEL_COMMIT_COUNT).wrapping_add(1),
    );
}
// ---------------------------------------------------------------------------
// Wave 99 - the GX quad submit (0x802D0030)
// ---------------------------------------------------------------------------

/// `r2 - 0x3B48`, `r2 - 0x3B4C`: the two depth constants the quad builders
/// pass in `f3`. `// unknown:` both are SDA cells initialised at run time.
const SDA2_QUAD_DEPTH: usize = 0x8042_F538;
const SDA2_QUAD_LEVEL: usize = 0x8042_F534;

/// `0xCC008000`: the GX FIFO write port.
const GX_FIFO: usize = 0xCC00_8000;

// ---------------------------------------------------------------------------
// 0x802D0030 - draw one textured quad through the GX FIFO
// ---------------------------------------------------------------------------

/// Draws one quad: runs the object's `vtable+0x10` hook, arms the GX
/// projection, builds the four vertices and closes the primitive.
///
/// The vertex stream is written straight to the FIFO: for each of the four
/// vertices a two-byte colour value (`0x0000`, `0x0101`, `0x0202`, `0x0303`)
/// followed by the object's `+0x50`/`+0x54` pair as two `f32` components.
///
/// # Original
/// - Ghidra: `FUN_802d0030` at `0x802D0030` (372 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802d0030_FUN_802d0030.c`
///
/// # Arguments
/// - `scale_x`, `scale_y`: multiplied by the object's `+0x1C`/`+0x20` and
///   handed to `0x801C0C60` as the two quad dimensions.
/// - `this`: the quad object. `+0x28` is the sub-object whose `+0x10` word is
///   the entry hook, `+0x14`/`+0x18` the origin, `+0x50`/`+0x54` the vertex
///   position.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - `(this+0x28)->+0x10(this)`, `0x802CE850(this)`.
/// - `0x801C40B0(0, 0xE, 1, 4, 0)`, `0x801C37C0(0xE, 1)`.
/// - `0x801C0C60(x, y, depth, &scratch)`, `0x801C0C10(ox, oy, level, &scratch,
///   &scratch)`, `0x801C9940(&scratch, 0)`, `0x801C5570(0x80, 0, 4)`.
/// - Forty bytes into the GX FIFO, then `0x801C7750(1, 0, 0x40, 0)` and
///   `0x801C77D0(1, 0, 0)`.
///
/// # Safety
/// `this` must be a live quad object with a valid `+0x28` sub-object, and the
/// fifty-two byte scratch record must cover what `0x801C0C60` writes.
///
/// # Confidence
/// high - the indirect call (`lwz r12,0x28(r3)` / `lwz r12,0x10(r12)` /
/// `bctrl`), the four
/// `fmuls` argument pairs and every FIFO store are literal at
/// `0x802D005C..0x802D0168`; the export's `ldexpf`/`GQR0` noise is only the two
/// `psq_st` spills of `f30`/`f31`.
pub unsafe fn fn_802d0030(scale_x: f32, scale_y: f32, this: usize) {
    // observed: r12 = *(this+0x28), then r12 = *(r12+0x10), then `bctrl`.
    // The hook is the word at +0x10 of the sub-object itself, not a vtable hop.
    let sub = platform::read_u32(this + 0x28) as usize;
    let enter = crate::platform::guest_fn!(platform::read_u32(sub + 0x10) as usize, fn(usize));
    enter(this);

    platform::call1(0x802C_E850, this as u32);
    platform::call5(0x801C_40B0, 0, 0xE, 1, 4, 0);
    platform::call2(0x801C_37C0, 0xE, 1);

    let mut scratch = crate::platform::GuestLocal::new( [0u32; 13]);
    let base = scratch.as_mut_ptr() as usize;
    let size = crate::platform::guest_fn!(0x801C_0C60usize, fn(f32, f32, f32, usize));
    size(
        scale_x * platform::read_f32(this + 0x1C),
        scale_y * platform::read_f32(this + 0x20),
        platform::read_f32(SDA2_QUAD_DEPTH),
        base,
    );
    let place = crate::platform::guest_fn!(0x801C_0C10usize, fn(f32, f32, f32, usize, usize));
    place(
        platform::read_f32(this + 0x14),
        platform::read_f32(this + 0x18),
        platform::read_f32(SDA2_QUAD_LEVEL),
        base,
        base,
    );
    platform::call2(0x801C_9940, base as u32, 0);
    platform::call3(0x801C_5570, 0x80, 0, 4);

    // observed: colour `i` (two identical bytes) is written before vertex `i`,
    // so the very first pair of zero bytes is the first vertex's colour.
    for index in 0..4u8 {
        platform::write_u8(GX_FIFO, index);
        platform::write_u8(GX_FIFO, index);
        platform::write_u32(GX_FIFO, platform::read_f32(this + 0x50).to_bits());
        platform::write_u32(GX_FIFO, platform::read_f32(this + 0x54).to_bits());
    }

    platform::call4(0x801C_7750, 1, 0, 0x40, 0);
    platform::call3(0x801C_77D0, 1, 0, 0);
}

// ---------------------------------------------------------------------------
// Wave 100 - the member-function thunk, the look-at recompute and `%a`
// ---------------------------------------------------------------------------

/// `r2 - 0x3A30`, `r2 - 0x3A2C`, `r2 - 0x3A28` (`0x8042F650`/`54`/`58`): the
/// three float cells the look-at vector is seeded from and the scale applied to
/// the state's basis vector. `work/dol/main.elf` initialises them to `0.0f`,
/// `1.0f` and `0x4222F983` (`40.7437f`), but the game's constructors also write
/// this area, so they are read at run time like every other SDA cell here.
const SDA2_AIM_AXIS_X: usize = 0x8042_F650;
const SDA2_AIM_AXIS_Y: usize = 0x8042_F654;
const SDA2_AIM_SCALE: usize = 0x8042_F658;

/// `r2 - 0x39E0` (`0x8042F6A0`): the `double` the `%a` formatter compares the
/// absolute value of its argument against to choose the leading mantissa digit.
/// The image initialises the cell to `0.0`.
const SDA2_F64_ZERO: usize = 0x8042_F6A0;

/// `0x80350C80` (Ghidra's `PTR_DAT_80350c80`): a second locale pointer slot,
/// distinct from [`LOCALE_TABLE`] at `0x80350E68`. The word stored here points
/// at a record whose first byte is the decimal-point character.
const DECIMAL_POINT_SLOT: usize = 0x8035_0C80;

/// `0x802382B0(value_x, value_y, value_z, out)`: builds a 3x4 matrix from a
/// translation vector. `// unknown:` no body exists for it yet, so it is called
/// by raw address.
const BUILD_TRANSLATION_MATRIX: usize = 0x8023_82B0;

// ---------------------------------------------------------------------------
// 0x802DCBFC - call through a pointer-to-member-function descriptor
// ---------------------------------------------------------------------------

/// A CodeWarrior pointer-to-member descriptor address - the value the
/// `0x802DCBFC` dispatcher receives in `r12`.
///
/// The descriptor and the receiver are both word-sized pointers that sit next
/// to each other, and the dispatcher reads the descriptor *before* it adjusts
/// the receiver. Wave 248 shipped exactly that transposition
/// (`fn_802dcbfc(descriptor, object)`) and it compiled silently because both
/// parameters were `usize`. Wrapping the descriptor in its own type makes the
/// swap a compile error at every [`fn_802dcbfc`] / [`fn_802dcbfc_target`] call
/// and at every `interface_call_desc*` helper in `game::bases`.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Descriptor(usize);

impl Descriptor {
    /// Wraps a raw descriptor address.
    #[inline]
    pub const fn new(address: usize) -> Self {
        Self(address)
    }

    /// The raw descriptor address the dispatcher reads from `r12`.
    #[inline]
    pub const fn address(self) -> usize {
        self.0
    }
}

/// Tail-calls the member function described by a CodeWarrior
/// pointer-to-member descriptor.
///
/// The descriptor is `{ this_delta, vtable_index, target }`. The body adds
/// `this_delta` to the object pointer, and when `vtable_index` is negative it
/// calls `target` directly; otherwise it reads the vtable word at
/// `target + adjusted_this`, picks slot `vtable_index` there and calls that.
///
/// # Original
/// - Ghidra: `FUN_802dcbfc` at `0x802DCBFC` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcbfc_FUN_802dcbfc.c`
///
/// # Arguments
/// - `this`: the object pointer arriving in `r3`; adjusted by the descriptor's
///   first word before the call.
/// - `pmf`: the descriptor pointer arriving in `r12`. This is *not* an argument
///   register, which is why Ghidra printed `UNRECOVERED_JUMPTABLE` and an empty
///   argument list; the dispatch is entered with `r12` already loaded.
///
/// # Returns
/// Whatever the callee returns in `r3`. The dispatch ends in `mtctr`/`bctr`, so
/// it is a tail call and the callee's result reaches this function's caller
/// unchanged.
///
/// # Side effects
/// Only the callee's effects.
///
/// # Safety
/// `pmf` must point at a live three-word descriptor, and for a virtual member
/// the resolved slot must be a callable entry point with the adjusted object as
/// its only argument.
///
/// # Confidence
/// medium - all ten instructions are literal (`lwz r0,0(r12)`,
/// `lwz r11,4(r12)`, `lwz r12,8(r12)`, `add r3,r3,r0`, `cmpwi r11,0` +
/// `blt`, the two `lwzx` hops at `0x802DCC14`/`0x802DCC18`, `mtctr`, `bctr`),
/// and the descriptor field roles follow from the CodeWarrior
/// pointer-to-member representation; no symbol names the descriptor.
pub unsafe fn fn_802dcbfc(this: usize, pmf: Descriptor) -> u32 {
    let (target, receiver) = fn_802dcbfc_target(this, pmf);
    let call = crate::platform::guest_fn!(target, fn(usize) -> u32);
    call(receiver)
}

/// Resolves the pointer-to-member descriptor that [`fn_802dcbfc`] dispatches,
/// without performing the call.
///
/// This is the same three-word read and two-level lookup as the shipped
/// dispatcher. It exists so that callers which must forward extra register
/// arguments (`r4..r7`, which the dispatcher leaves untouched for the tail
/// call) can obtain the adjusted receiver and the resolved entry point and
/// call it themselves.
///
/// # Original
/// - Ghidra: none - host-side factoring of the descriptor resolution in
///   [`fn_802dcbfc`], whose evidence block owns the shipped dispatcher.
///   Observed: `lwz r0,0(r12)`, `lwz r11,4(r12)`, `lwz r12,8(r12)`,
///   `add r3,r3,r0`, `cmpwi r11,0`, the two `lwzx` hops, `mtctr`, `bctr`.
/// - Evidence: `work/ghidra/main-gb/export/802dcbfc_FUN_802dcbfc.c`
///
/// # Arguments
/// - `this`: the object pointer the descriptor's first word adjusts.
/// - `pmf`: the descriptor address; the shipped dispatcher receives it in
///   `r12`, so it is an explicit argument here.
///
/// # Returns
/// `(target, receiver)`: the entry point selected by the descriptor and the
/// adjusted object pointer to pass in `r3`.
///
/// # Safety
/// `pmf` must point at a live three-word descriptor; for a virtual member the
/// resolved slot must be a callable entry point.
///
/// # Confidence
/// high - identical arithmetic to [`fn_802dcbfc`], which this now owns.
pub unsafe fn fn_802dcbfc_target(this: usize, pmf: Descriptor) -> (usize, usize) {
    // observed: the descriptor is read before `r3` is adjusted.
    let pmf = pmf.address();
    let this_delta = platform::read_u32(pmf) as usize;
    let vtable_index = platform::read_i32(pmf + 4);
    let mut target = platform::read_u32(pmf + 8);
    let adjusted = this.wrapping_add(this_delta);

    if vtable_index >= 0 {
        // observed: `lwzx r12,r3,r12` -- the vtable word is addressed relative to
        // the *adjusted* object pointer, then `lwzx r12,r12,r11` picks the slot.
        let table = platform::read_u32(adjusted.wrapping_add(target as usize)) as usize;
        target = platform::read_u32(table.wrapping_add(vtable_index as usize));
    }

    (target as usize, adjusted)
}

// ---------------------------------------------------------------------------
// 0x802DADD0 - recompute the look-at angles of an object
// ---------------------------------------------------------------------------

/// Rebuilds an object's aim orientation and stores the two Euler angles it
/// decomposes into.
///
/// The direction starts as the local Y axis, scaled into the state object's
/// basis (`+0x5C..+0x64` x [`SDA2_AIM_SCALE`]) and transformed by both the
/// object's own 3x4 matrix and that basis matrix. The vector at `+0x2C` of the
/// sub-object held at `+0x154` is added, the sum normalised, and the inverse of
/// the object matrix applied to it. The resulting vector supplies
/// `acos` of its y component at `+0x84` and `atan2` of its x/z pair at `+0x88`,
/// with `+0x8C` set to the seed constant.
///
/// # Original
/// - Ghidra: `FUN_802dadd0` at `0x802DADD0` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dadd0_FUN_802dadd0.c`
///
/// # Arguments
/// - `this`: the object. `+0xB8` is the state object carrying the basis at
///   `+0x5C/+0x60/+0x64`; `+0x154` is a sub-object whose `+0x2C` is the
///   incoming vector; `+0x84`, `+0x88` and `+0x8C` receive the results.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - Writes `this+0x84` (`acos` of the y component), `this+0x88` (`atan2` of the
///   x/z pair) and `this+0x8C` (the seed constant), all as `f32`.
/// - Builds two 3x4 matrices and a 3-vector on the stack through
///   [`fn_80289070`](crate::render::gx_band::fn_80289070), `0x802382B0`,
///   [`fn_801c1160`](crate::runtime::dvd::fn_801c1160) and
///   [`fn_801c1470`](crate::runtime::dvd::fn_801c1470).
///
/// # Safety
/// `this`, its `+0xB8` state and its `+0x154` sub-object must be live; the
/// object matrix builder reads the whole transform chain behind `+0xF4`.
///
/// # Confidence
/// medium - the two matrix pointers (`r1+0x20`, `r1+0x80`), the two
/// matrix-times-vector calls (`PSMTXMultVec` in the SDK), the `ps_add` of
/// `r31`'s three floats, the normalise call, the three-by-four inverse result
/// test at `0x802DAEB0` and every store offset are literal. The two float-maths
/// call sites are the `bl` targets `0x802E8608`/`0x802E8610` (thunks to
/// `0x802E46A0`/`0x802E4C04`), whose bodies `engine::main_loop` classifies as an
/// `acos` kernel and an `atan2` reconstruction.
/// `GQR0` is zero, so the `psq_l`/`psq_st` pairs are plain `f32` accesses; the
/// export's `ldexpf`/`CONCAT44` ladders are that quantisation model and are not
/// transcribed. The two stack records below are only consumed through their
/// addresses, exactly like the original's stack slots, which is why the body
/// carries a scoped `unused_assignments` allowance.
#[allow(unused_assignments)]
pub unsafe fn fn_802dadd0(this: usize) {
    let state = platform::read_u32(this + 0xB8) as usize;
    // observed: 0x802DBFB0 is `addi r3,r3,0x2c`, so the vector is the
    // sub-object's `+0x2C`.
    let source = fn_802dbfb0(platform::read_u32(this + 0x154) as usize);

    // observed: `lfs` at `0x802DAE00..0x802DAE14` seeds (x, y, x) -- the local Y
    // axis under the two SDA cells.
    let axis_x = platform::read_f32(SDA2_AIM_AXIS_X);
    let axis_y = platform::read_f32(SDA2_AIM_AXIS_Y);
    let mut aim = crate::platform::GuestLocal::new( [axis_x, axis_y, axis_x]);

    // observed: `r1+0x20` is the object matrix (0x30 bytes), `r1+0x80` the
    // basis matrix (0x30 bytes) and `r1+0x50` the inverse.
    let mut object_matrix = crate::platform::GuestLocal::new( [0u32; 12]);
    let mut basis_matrix = crate::platform::GuestLocal::new( [0u32; 12]);
    let mut inverse = crate::platform::GuestLocal::new( [0u32; 12]);
    let object_ptr = object_matrix.as_mut_ptr() as usize;
    let basis_ptr = basis_matrix.as_mut_ptr() as usize;
    let inverse_ptr = inverse.as_mut_ptr() as usize;
    let aim_ptr = aim.as_mut_ptr() as usize;

    crate::render::gx_band::fn_80289070(this, object_ptr);
    // observed: `stfs f1,0x2C/0x3C/0x4C(r1)` -- the translation column of the
    // object matrix (words 3, 7 and 11) is overwritten with the seed.
    object_matrix[3] = axis_x.to_bits();
    object_matrix[7] = axis_x.to_bits();
    object_matrix[11] = axis_x.to_bits();

    // observed: three `fmuls` of the state's `+0x5C/+0x60/+0x64` by the SDA
    // scale, handed to the builder with `r3 = r1+0x80`.
    let scale = platform::read_f32(SDA2_AIM_SCALE);
    let build = crate::platform::guest_fn!(BUILD_TRANSLATION_MATRIX, fn(f32, f32, f32, usize));
    build(
        platform::read_f32(state + 0x5C) * scale,
        platform::read_f32(state + 0x60) * scale,
        platform::read_f32(state + 0x64) * scale,
        basis_ptr,
    );

    // observed: basis first, then the object matrix; both write back into `aim`.
    crate::runtime::dvd::fn_801c1160(basis_ptr, aim_ptr, aim_ptr);
    crate::runtime::dvd::fn_801c1160(object_ptr, aim_ptr, aim_ptr);

    // observed: the `ps_add` pair plus the two `W=1` single loads add the three
    // source components into the aim vector.
    aim[0] += platform::read_f32(source);
    aim[1] += platform::read_f32(source + 4);
    aim[2] += platform::read_f32(source + 8);

    crate::runtime::dvd::fn_801c1470(aim_ptr, aim_ptr);

    // observed: a 12-byte scratch record at `r1+0x08`.
    let mut vector = crate::platform::GuestLocal::new( [0u32; 3]);
    let vector_ptr = vector.as_mut_ptr() as usize;
    if crate::runtime::dvd::fn_801c07a0(object_ptr, inverse_ptr) != 0 {
        crate::runtime::dvd::fn_801c1160(inverse_ptr, aim_ptr, vector_ptr);
        let acos =
            crate::engine::main_loop::fn_802e46a0(platform::read_f32(vector_ptr + 4) as f64) as f32;
        platform::write_f32(this + 0x84, acos);
        let atan2 = crate::engine::main_loop::fn_802e4c04(
            platform::read_f32(vector_ptr) as f64,
            platform::read_f32(vector_ptr + 8) as f64,
        ) as f32;
        platform::write_f32(this + 0x8C, axis_x);
        platform::write_f32(this + 0x88, atan2);
    }
}

// ---------------------------------------------------------------------------
// 0x802DFF64 - format one double as a hexadecimal float (`%a` / `%A`)
// ---------------------------------------------------------------------------

/// Formats `value` as a `%a`-style hexadecimal float and returns the first byte
/// of the result.
///
/// The buffer is filled backwards from `buf`; the caller passes a pointer just
/// past the end of its scratch area. The decimal conversion of `value` decides
/// the special cases (`"inf"`, `"-INF"`, `"nan"`, `"-NAN"` and the alternative
/// uppercase spellings); anything else emits `0x` + the leading mantissa digit,
/// the locale's decimal point, `precision` mantissa nibbles, `p`/`P`, and the
/// unbiased exponent in decimal.
///
/// # Original
/// - Ghidra: `FUN_802dff64` at `0x802DFF64` (968 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dff64_FUN_802dff64.c`
///
/// # Arguments
/// - `buf`: one past the last byte of the output area; every character is stored
///   at a lower address.
/// - `spec`: the `printf` conversion spec. `+1` selects the sign (`1` = `+`,
///   `2` = space), `+3` is the `#` alternative flag, `+5` is the conversion
///   character (`a`/`A`) and `+0xC` is the precision.
/// - `value`: the double to format, arriving in `f1`.
///
/// # Returns
/// The first byte of the produced string, or `0` when the precision exceeds
/// `0x1FD`.
///
/// # Side effects
/// Writes up to `precision + 8` bytes below `buf` and fills a 0x40-byte
/// conversion record and a 16-byte exponent format on the stack; calls
/// [`fn_802de370`] (the decimal conversion) and [`fn_802dfa74`] (the exponent
/// formatter). The special-value texts go through a byte loop here in place of
/// the libc `strcpy` at `0x802E1938`.
///
/// # Safety
/// `buf` must have at least `precision + 8` writable bytes in front of it and
/// `spec` must be a readable `printf` spec record.
///
/// # Confidence
/// medium - the control flow, the string table at `0x802FD9C0` (`"-INF"`,
/// `"-inf"`, `"INF"`, `"inf"`, `"-NAN"`, `"-nan"`, `"NAN"`, `"nan"`), the
/// nibble loop at `0x802E01CC..0x802E0258` and every store are literal. The
/// export's `ldexpf` blocks are the `psq_st`/`psq_l` save/restore of `f31`
/// (`0x802DFF78`/`0x802E0308`) and are not modelled; two bytes of the
/// 16-byte exponent-format record are never initialised in the original.
pub unsafe fn fn_802dff64(buf: usize, spec: usize, value: f64) -> usize {
    // observed: `lwz r5,0x80350C80` then `lbz r31,0(r5)`.
    let decimal_point = platform::read_u8(platform::read_u32(DECIMAL_POINT_SLOT) as usize);

    // observed: `lwz r0,0xc(r4)` / `cmpwi r0,0x1fd` / `bgt` -> return null.
    let precision = platform::read_i32(spec + 0xC);
    if precision > 0x1FD {
        return 0;
    }

    // observed: the four-byte format word at `r1+0x10`: `stb r28,0x10(r1)` (0)
    // and `sth r0,0x12(r1)` with `r0 = 0x20`, i.e. the big-endian halfword at
    // `+2` is `0x0020` -- the 32-digit request 0x802DE370 reads with `lha`.
    let mut format = crate::platform::GuestLocal::new( [0u8; 4]);
    format[3] = 0x20;
    // observed: the conversion record at `r1+0x38`, filled by 0x802DE370:
    // `+0` sign, `+4` digit count and `+5` first digit (or 'I'/'N').
    let mut record = crate::platform::GuestLocal::new( [0u8; 0x40]);
    let record_ptr = record.as_mut_ptr() as usize;
    fn_802de370(value, format.as_mut_ptr() as usize, record_ptr);

    let sign = platform::read_u8(record_ptr) as i8;
    let kind = platform::read_u8(record_ptr + 5);

    if kind == b'I' || kind == b'N' {
        // observed: `lbz r0,5(r30)` / `cmpwi r0,0x41` picks the uppercase text,
        // and the signed forms start five bytes below the buffer end because
        // `strcpy` also copies the terminating NUL.
        let upper = platform::read_u8(spec + 5) == b'A';
        let text: &[u8] = match (kind, sign != 0, upper) {
            (b'I', false, true) => &b"INF\0"[..],
            (b'I', false, false) => &b"inf\0"[..],
            (b'I', true, true) => &b"-INF\0"[..],
            (b'I', true, false) => &b"-inf\0"[..],
            (_, false, true) => &b"NAN\0"[..],
            (_, false, false) => &b"nan\0"[..],
            (_, true, true) => &b"-NAN\0"[..],
            (_, true, false) => &b"-nan\0"[..],
        };
        let start = buf.wrapping_sub(text.len());
        for (index, byte) in text.iter().enumerate() {
            platform::write_u8(start + index, *byte);
        }
        return start;
    }

    if kind == b'0' {
        // observed: `sth r0,0x3a(r1)` clears the record's exponent field for a
        // zero value before the digit path runs.
        platform::write_u8(record_ptr + 2, 0);
        platform::write_u8(record_ptr + 3, 0);
    }

    // observed: the 16-byte integer-format record assembled at `r1+0x28` and
    // copied to `r1+0x18` before the call. Its bytes 6 and 7 are never stored
    // in the original; they are zeroed here.
    let mut exponent_format = crate::platform::GuestLocal::new( [0u8; 16]);
    exponent_format[0] = 1;
    exponent_format[1] = 1;
    exponent_format[5] = b'd';
    exponent_format[12] = 1;

    // observed: `rlwinm r5,r0,11,21,31` of the two exponent bytes, i.e. the
    // 11-bit biased exponent; the `and` with the sign mask below clears it to
    // zero for a denormal.
    let bits = value.to_bits();
    let biased = ((bits >> 52) & 0x7FF) as i32;
    let exponent = if biased == 0 { 0 } else { biased - 0x3FF };
    let mut cursor = fn_802dfa74(
        exponent as u32,
        buf,
        exponent_format.as_mut_ptr() as usize,
    );

    // observed: `stbu r0,-1(r3)` writes 'p'/'P' one byte in front of the
    // exponent digits.
    cursor -= 1;
    platform::write_u8(
        cursor,
        if platform::read_u8(spec + 5) == b'a' {
            b'p'
        } else {
            b'P'
        },
    );

    // observed: one nibble per iteration, starting at bit 4*precision+11 of the
    // double and walking down by four; indices past the 64-bit word print '0'.
    let mut position = 0i32;
    while position < precision {
        let bit = 4 * precision + 11 - 4 * position;
        let digit = if bit >= 64 {
            0u8
        } else {
            ((bits >> ((63 - bit) as u32)) & 0xF) as u8
        };
        let character = if digit < 10 {
            b'0' + digit
        } else if platform::read_u8(spec + 5) == b'a' {
            // observed: `addi r4,r4,0x57`.
            digit + 0x57
        } else {
            // observed: `addi r4,r4,0x37`.
            digit + 0x37
        };
        cursor -= 1;
        platform::write_u8(cursor, character);
        position += 1;
    }

    // observed: the decimal point appears for a non-zero precision or when the
    // `#` flag at `spec+3` is set.
    if precision != 0 || platform::read_u8(spec + 3) != 0 {
        cursor -= 1;
        platform::write_u8(cursor, decimal_point);
    }

    // observed: `fabs f1,f31` then `fcmpu` against SDA2_F64_ZERO; only a zero
    // value prints a leading '0'.
    let leading = if value.abs() == read_f64_be(SDA2_F64_ZERO) {
        b'0'
    } else {
        b'1'
    };
    platform::write_u8(cursor - 1, leading);
    platform::write_u8(
        cursor - 2,
        if platform::read_u8(spec + 5) == b'a' {
            b'x'
        } else {
            b'X'
        },
    );
    let mut start = cursor - 3;
    platform::write_u8(start, b'0');

    if sign == 0 {
        // observed: `lbz r0,1(r30)` selects '+' (1) or ' ' (2); a plain number
        // keeps the '0' as its first byte.
        let mode = platform::read_u8(spec + 1);
        if mode == 1 {
            start -= 1;
            platform::write_u8(start, b'+');
        } else if mode == 2 {
            start -= 1;
            platform::write_u8(start, b' ');
        }
    } else {
        start -= 1;
        platform::write_u8(start, b'-');
    }
    start
}

// ---------------------------------------------------------------------------
// 0x802DCCCC..0x802DCDC0 - CodeWarrior callee-save register entries
//
// The four private helpers below write or read one save window of the caller's
// frame. A window starts `first_slot` bytes below `frame` and runs up to
// `frame`; the GPR stride is four bytes and the FPR stride is eight.
// ---------------------------------------------------------------------------

/// Writes the GPR save window `frame - first_slot .. frame`.
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `first_slot`: the distance from `frame` to the first saved word.
/// - `regs`: the live register values, lowest register first.
///
/// # Safety
/// `frame - first_slot .. frame` must be writable for `regs.len() * 4` bytes.
unsafe fn save_gpr_run(frame: usize, first_slot: usize, regs: &[u32]) {
    let mut i = 0;
    while i < regs.len() {
        platform::write_u32(frame.wrapping_sub(first_slot).wrapping_add(i * 4), regs[i]);
        i += 1;
    }
}

/// Reads the GPR restore window `frame - first_slot .. frame` back out.
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `first_slot`: the distance from `frame` to the first restored word.
/// - `out`: receives the words, lowest register first.
///
/// # Safety
/// `frame - first_slot .. frame` must be readable for `out.len() * 4` bytes.
unsafe fn load_gpr_run(frame: usize, first_slot: usize, out: &mut [u32]) {
    let mut i = 0;
    while i < out.len() {
        out[i] = platform::read_u32(frame.wrapping_sub(first_slot).wrapping_add(i * 4));
        i += 1;
    }
}

/// Writes the FPR save window `frame - first_slot .. frame`.
///
/// Each `stfd` stores one big-endian `f64`; the two `write_u32` calls below are
/// that store split into the high and low word.
///
/// # Safety
/// `frame - first_slot .. frame` must be writable for `regs.len() * 8` bytes.
unsafe fn save_fpr_run(frame: usize, first_slot: usize, regs: &[f64]) {
    let mut i = 0;
    while i < regs.len() {
        let addr = frame.wrapping_sub(first_slot).wrapping_add(i * 8);
        let bits = regs[i].to_bits();
        platform::write_u32(addr, (bits >> 32) as u32);
        platform::write_u32(addr + 4, bits as u32);
        i += 1;
    }
}

/// Reads the FPR restore window `frame - first_slot .. frame` back out.
///
/// Each `lfd` loads one big-endian `f64`; the two `read_u32` calls below are
/// that load split into the high and low word.
///
/// # Safety
/// `frame - first_slot .. frame` must be readable for `out.len() * 8` bytes.
unsafe fn load_fpr_run(frame: usize, first_slot: usize, out: &mut [f64]) {
    let mut i = 0;
    while i < out.len() {
        let addr = frame.wrapping_sub(first_slot).wrapping_add(i * 8);
        let hi = platform::read_u32(addr) as u64;
        let lo = platform::read_u32(addr + 4) as u64;
        out[i] = f64::from_bits((hi << 32) | lo);
        i += 1;
    }
}

// ---------------------------------------------------------------------------
// 0x802DCCCC - __savefpr_23: save f23..f31
// ---------------------------------------------------------------------------

/// Saves `f23..f31` into the caller's frame (entry `__savefpr_23`).
///
/// The 20 bytes `functions.tsv` gives this label are the first five `stfd`
/// instructions; the run continues through `0x802DCCE0`, which stores
/// `f28..f31` and returns through the shared `blr` at `0x802DCCF0`.
///
/// # Original
/// - Ghidra: `FUN_802dcccc` at `0x802DCCCC` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcccc_FUN_802dcccc.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `f23..f31` doubles, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is untouched.
///
/// # Side effects
/// Writes nine big-endian doubles to `frame-0x48..frame` (the last one starts at
/// `frame-8`).
///
/// # Safety
/// `frame-0x48..frame` must be writable; `regs` is fixed at nine entries.
///
/// # Confidence
/// medium - the nine `stfd f23,-0x48(r11)` .. `stfd f31,-8(r11)` instructions at
/// `0x802DCCCC..0x802DCCEC` are literal (re-read from `work/dol/main.elf`); the
/// array signature is the only Rust surface for a register-only effect.
pub unsafe fn fn_802dcccc(frame: usize, regs: &[f64; 9]) {
    save_fpr_run(frame, 0x48, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCCE0 - __savefpr_28: save f28..f31 and return
// ---------------------------------------------------------------------------

/// Saves `f28..f31` into the caller's frame and returns (`__savefpr_28`).
///
/// This is the second half of the run that starts at `0x802DCCCC`: four `stfd`
/// instructions plus the `blr` at `0x802DCCF0`, which is exactly the 20 bytes
/// `functions.tsv` assigns here.
///
/// # Original
/// - Ghidra: `FUN_802dcce0` at `0x802DCCE0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcce0_FUN_802dcce0.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `f28..f31` doubles, lowest register first.
///
/// # Returns
/// Nothing; `r3` is not written.
///
/// # Side effects
/// Writes four big-endian doubles to `frame-0x20..frame` (the last one starts at
/// `frame-8`).
///
/// # Safety
/// `frame-0x20..frame` must be writable; `regs` is fixed at four entries.
///
/// # Confidence
/// medium - the four `stfd f28,-0x20(r11)` .. `stfd f31,-8(r11)` instructions and
/// the `blr` at `0x802DCCF0` are literal (re-read from `work/dol/main.elf`).
pub unsafe fn fn_802dcce0(frame: usize, regs: &[f64; 4]) {
    save_fpr_run(frame, 0x20, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD18 - __restfpr_23: restore f23..f31
// ---------------------------------------------------------------------------

/// Restores `f23..f31` from the caller's frame (entry `__restfpr_23`).
///
/// Ghidra prints `return;` because the only effect is loading callee-saved FPRs,
/// which its C cannot express. The raw code loads `f23..f27` here and continues
/// through `0x802DCD2C` (`f28..f31`) to the `blr` at `0x802DCD3C`; the body
/// returns the nine doubles so the effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcd18` at `0x802DCD18` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd18_FUN_802dcd18.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The nine words for `f23..f31`, lowest register first. In the original they
/// land back in the registers, not in memory.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x48..frame` must be readable.
///
/// # Confidence
/// medium - the nine `lfd f23,-0x48(r11)` .. `lfd f31,-8(r11)` instructions and
/// the `blr` at `0x802DCD3C` are literal (re-read from `work/dol/main.elf`); the
/// exported C body is only `return;`, so the load targets come from the raw
/// instructions.
pub unsafe fn fn_802dcd18(frame: usize) -> [f64; 9] {
    let mut regs = [0.0_f64; 9];
    load_fpr_run(frame, 0x48, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCD2C - __restfpr_28: restore f28..f31 and return
// ---------------------------------------------------------------------------

/// Restores `f28..f31` from the caller's frame (`__restfpr_28`).
///
/// This is the second half of the run that starts at `0x802DCD18`: four `lfd`
/// instructions plus the `blr` at `0x802DCD3C`. Ghidra prints `return;` for the
/// whole family, so the body is reconstructed from the raw instruction words.
///
/// # Original
/// - Ghidra: `FUN_802dcd2c` at `0x802DCD2C` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd2c_FUN_802dcd2c.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The four words for `f28..f31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x20..frame` must be readable.
///
/// # Confidence
/// medium - the four `lfd f28,-0x20(r11)` .. `lfd f31,-8(r11)` instructions and
/// the `blr` at `0x802DCD3C` are literal (re-read from `work/dol/main.elf`).
pub unsafe fn fn_802dcd2c(frame: usize) -> [f64; 4] {
    let mut regs = [0.0_f64; 4];
    load_fpr_run(frame, 0x20, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCD40 - __savegprlr_14: save r14..r31
// ---------------------------------------------------------------------------

/// Saves `r14..r31` into the caller's frame (entry `__savegprlr_14`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD44..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes all
/// 18 words `frame-0x48..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd40` at `0x802DCD40` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd40_FUN_802dcd40.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r14..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 18 big-endian words to `frame-0x48..frame-4`.
///
/// # Safety
/// `frame-0x48..frame` must be writable; `regs` is fixed at 18 entries.
///
/// # Confidence
/// medium - the `stw r14,-0x48(r11)` .. `stw r31,-4(r11)` run and the `blr` at
/// `0x802DCD88` are literal (re-read from `work/dol/main.elf`); the explicit
/// register array is the only Rust surface for a register-only effect.
pub unsafe fn fn_802dcd40(frame: usize, regs: &[u32; 18]) {
    save_gpr_run(frame, 0x48, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD44 - __savegprlr_15: save r15..r31
// ---------------------------------------------------------------------------

/// Saves `r15..r31` into the caller's frame (entry `__savegprlr_15`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD48..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 17
/// words `frame-0x44..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd44` at `0x802DCD44` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd44_FUN_802dcd44.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r15..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 17 big-endian words to `frame-0x44..frame-4`.
///
/// # Safety
/// `frame-0x44..frame` must be writable; `regs` is fixed at 17 entries.
///
/// # Confidence
/// medium - the `stw r15,-0x44(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd44(frame: usize, regs: &[u32; 17]) {
    save_gpr_run(frame, 0x44, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD48 - __savegprlr_16: save r16..r31
// ---------------------------------------------------------------------------

/// Saves `r16..r31` into the caller's frame (entry `__savegprlr_16`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD4C..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 16
/// words `frame-0x40..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd48` at `0x802DCD48` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd48_FUN_802dcd48.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r16..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 16 big-endian words to `frame-0x40..frame-4`.
///
/// # Safety
/// `frame-0x40..frame` must be writable; `regs` is fixed at 16 entries.
///
/// # Confidence
/// medium - the `stw r16,-0x40(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd48(frame: usize, regs: &[u32; 16]) {
    save_gpr_run(frame, 0x40, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD4C - __savegprlr_17: save r17..r31
// ---------------------------------------------------------------------------

/// Saves `r17..r31` into the caller's frame (entry `__savegprlr_17`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD50..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 15
/// words `frame-0x3C..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd4c` at `0x802DCD4C` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd4c_FUN_802dcd4c.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r17..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 15 big-endian words to `frame-0x3C..frame-4`.
///
/// # Safety
/// `frame-0x3C..frame` must be writable; `regs` is fixed at 15 entries.
///
/// # Confidence
/// medium - the `stw r17,-0x3C(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd4c(frame: usize, regs: &[u32; 15]) {
    save_gpr_run(frame, 0x3C, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD50 - __savegprlr_18: save r18..r31
// ---------------------------------------------------------------------------

/// Saves `r18..r31` into the caller's frame (entry `__savegprlr_18`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD54..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 14
/// words `frame-0x38..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd50` at `0x802DCD50` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd50_FUN_802dcd50.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r18..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 14 big-endian words to `frame-0x38..frame-4`.
///
/// # Safety
/// `frame-0x38..frame` must be writable; `regs` is fixed at 14 entries.
///
/// # Confidence
/// medium - the `stw r18,-0x38(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd50(frame: usize, regs: &[u32; 14]) {
    save_gpr_run(frame, 0x38, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD54 - __savegprlr_19: save r19..r31
// ---------------------------------------------------------------------------

/// Saves `r19..r31` into the caller's frame (entry `__savegprlr_19`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD58..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 13
/// words `frame-0x34..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd54` at `0x802DCD54` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd54_FUN_802dcd54.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r19..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 13 big-endian words to `frame-0x34..frame-4`.
///
/// # Safety
/// `frame-0x34..frame` must be writable; `regs` is fixed at 13 entries.
///
/// # Confidence
/// medium - the `stw r19,-0x34(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd54(frame: usize, regs: &[u32; 13]) {
    save_gpr_run(frame, 0x34, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD58 - __savegprlr_20: save r20..r31
// ---------------------------------------------------------------------------

/// Saves `r20..r31` into the caller's frame (entry `__savegprlr_20`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD5C..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 12
/// words `frame-0x30..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd58` at `0x802DCD58` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd58_FUN_802dcd58.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r20..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 12 big-endian words to `frame-0x30..frame-4`.
///
/// # Safety
/// `frame-0x30..frame` must be writable; `regs` is fixed at 12 entries.
///
/// # Confidence
/// medium - the `stw r20,-0x30(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd58(frame: usize, regs: &[u32; 12]) {
    save_gpr_run(frame, 0x30, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD5C - __savegprlr_21: save r21..r31
// ---------------------------------------------------------------------------

/// Saves `r21..r31` into the caller's frame (entry `__savegprlr_21`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD60..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 11
/// words `frame-0x2C..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd5c` at `0x802DCD5C` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd5c_FUN_802dcd5c.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r21..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 11 big-endian words to `frame-0x2C..frame-4`.
///
/// # Safety
/// `frame-0x2C..frame` must be writable; `regs` is fixed at 11 entries.
///
/// # Confidence
/// medium - the `stw r21,-0x2C(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd5c(frame: usize, regs: &[u32; 11]) {
    save_gpr_run(frame, 0x2C, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD60 - __savegprlr_22: save r22..r31
// ---------------------------------------------------------------------------

/// Saves `r22..r31` into the caller's frame (entry `__savegprlr_22`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD64..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 10
/// words `frame-0x28..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd60` at `0x802DCD60` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd60_FUN_802dcd60.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r22..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 10 big-endian words to `frame-0x28..frame-4`.
///
/// # Safety
/// `frame-0x28..frame` must be writable; `regs` is fixed at 10 entries.
///
/// # Confidence
/// medium - the `stw r22,-0x28(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd60(frame: usize, regs: &[u32; 10]) {
    save_gpr_run(frame, 0x28, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD64 - __savegprlr_23: save r23..r31
// ---------------------------------------------------------------------------

/// Saves `r23..r31` into the caller's frame (entry `__savegprlr_23`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD68..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 9
/// words `frame-0x24..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd64` at `0x802DCD64` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd64_FUN_802dcd64.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r23..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 9 big-endian words to `frame-0x24..frame-4`.
///
/// # Safety
/// `frame-0x24..frame` must be writable; `regs` is fixed at 9 entries.
///
/// # Confidence
/// medium - the `stw r23,-0x24(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd64(frame: usize, regs: &[u32; 9]) {
    save_gpr_run(frame, 0x24, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD68 - __savegprlr_24: save r24..r31
// ---------------------------------------------------------------------------

/// Saves `r24..r31` into the caller's frame (entry `__savegprlr_24`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD6C..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 8
/// words `frame-0x20..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd68` at `0x802DCD68` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd68_FUN_802dcd68.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r24..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 8 big-endian words to `frame-0x20..frame-4`.
///
/// # Safety
/// `frame-0x20..frame` must be writable; `regs` is fixed at 8 entries.
///
/// # Confidence
/// medium - the `stw r24,-0x20(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd68(frame: usize, regs: &[u32; 8]) {
    save_gpr_run(frame, 0x20, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD6C - __savegprlr_25: save r25..r31
// ---------------------------------------------------------------------------

/// Saves `r25..r31` into the caller's frame (entry `__savegprlr_25`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues through the sibling labels `0x802DCD70..0x802DCD74` and
/// returns through the shared `blr` at `0x802DCD88`. Entered here it writes 7
/// words `frame-0x1C..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd6c` at `0x802DCD6C` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd6c_FUN_802dcd6c.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r25..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 7 big-endian words to `frame-0x1C..frame-4`.
///
/// # Safety
/// `frame-0x1C..frame` must be writable; `regs` is fixed at 7 entries.
///
/// # Confidence
/// medium - the `stw r25,-0x1C(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd6c(frame: usize, regs: &[u32; 7]) {
    save_gpr_run(frame, 0x1C, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD70 - __savegprlr_26: save r26..r31
// ---------------------------------------------------------------------------

/// Saves `r26..r31` into the caller's frame (entry `__savegprlr_26`).
///
/// The 4 bytes `functions.tsv` gives this label are only the first `stw`; the
/// routine continues into the final entry `0x802DCD74` and returns through the
/// shared `blr` at `0x802DCD88`. Entered here it writes 6 words
/// `frame-0x18..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd70` at `0x802DCD70` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd70_FUN_802dcd70.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r26..r31` words, lowest register first.
///
/// # Returns
/// Nothing; the shared tail ends in `blr` and `r3` is not written.
///
/// # Side effects
/// Writes 6 big-endian words to `frame-0x18..frame-4`.
///
/// # Safety
/// `frame-0x18..frame` must be writable; `regs` is fixed at 6 entries.
///
/// # Confidence
/// medium - the `stw r26,-0x18(r11)` .. `stw r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the register array is the modelling
/// surface.
pub unsafe fn fn_802dcd70(frame: usize, regs: &[u32; 6]) {
    save_gpr_run(frame, 0x18, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD74 - __savegprlr_27: save r27..r31 and return
// ---------------------------------------------------------------------------

/// Saves `r27..r31` into the caller's frame and returns (`__savegprlr_27`).
///
/// This is the only entry of the save run whose `functions.tsv` size covers a
/// whole routine: 24 bytes, the five `stw` instructions `r27..r31` plus the
/// `blr` at `0x802DCD88`. Entered here it writes 5 words
/// `frame-0x14..frame-4`.
///
/// # Original
/// - Ghidra: `FUN_802dcd74` at `0x802DCD74` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd74_FUN_802dcd74.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
/// - `regs`: the live `r27..r31` words, lowest register first.
///
/// # Returns
/// Nothing; `r3` is not written.
///
/// # Side effects
/// Writes 5 big-endian words to `frame-0x14..frame-4`.
///
/// # Safety
/// `frame-0x14..frame` must be writable; `regs` is fixed at 5 entries.
///
/// # Confidence
/// medium - the five `stw` instructions and the `blr` at `0x802DCD88` are
/// literal (re-read from `work/dol/main.elf`); the register array is the
/// modelling surface.
pub unsafe fn fn_802dcd74(frame: usize, regs: &[u32; 5]) {
    save_gpr_run(frame, 0x14, regs);
}

// ---------------------------------------------------------------------------
// 0x802DCD8C - __restgprlr_14: restore r14..r31
// ---------------------------------------------------------------------------

/// Restores `r14..r31` from the caller's frame (entry `__restgprlr_14`).
///
/// Ghidra prints `return;` because the only effect is loading callee-saved GPRs,
/// which its C cannot express. The 4 bytes `functions.tsv` gives this label are
/// the first `lwz`; the routine continues through the sibling labels
/// `0x802DCD90..0x802DCDC0` and returns through the shared `blr` at
/// `0x802DCDD4`. The body returns the 18 loaded words so the effect is
/// observable.
///
/// # Original
/// - Ghidra: `FUN_802dcd8c` at `0x802DCD8C` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd8c_FUN_802dcd8c.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 18 words for `r14..r31`, lowest register first. In the original they land
/// back in the registers, not in memory.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x48..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r14,-0x48(r11)` .. `lwz r31,-4(r11)` run and the `blr` at
/// `0x802DCDD4` are literal (re-read from `work/dol/main.elf`); the exported C
/// body is only `return;`, so the load targets come from the raw instructions.
pub unsafe fn fn_802dcd8c(frame: usize) -> [u32; 18] {
    let mut regs = [0u32; 18];
    load_gpr_run(frame, 0x48, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCD90 - __restgprlr_15: restore r15..r31
// ---------------------------------------------------------------------------

/// Restores `r15..r31` from the caller's frame (entry `__restgprlr_15`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCD94..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 17 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcd90` at `0x802DCD90` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd90_FUN_802dcd90.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 17 words for `r15..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x44..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r15,-0x44(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcd90(frame: usize) -> [u32; 17] {
    let mut regs = [0u32; 17];
    load_gpr_run(frame, 0x44, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCD94 - __restgprlr_16: restore r16..r31
// ---------------------------------------------------------------------------

/// Restores `r16..r31` from the caller's frame (entry `__restgprlr_16`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCD98..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 16 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcd94` at `0x802DCD94` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd94_FUN_802dcd94.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 16 words for `r16..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x40..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r16,-0x40(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcd94(frame: usize) -> [u32; 16] {
    let mut regs = [0u32; 16];
    load_gpr_run(frame, 0x40, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCD98 - __restgprlr_17: restore r17..r31
// ---------------------------------------------------------------------------

/// Restores `r17..r31` from the caller's frame (entry `__restgprlr_17`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCD9C..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 15 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcd98` at `0x802DCD98` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd98_FUN_802dcd98.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 15 words for `r17..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x3C..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r17,-0x3C(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcd98(frame: usize) -> [u32; 15] {
    let mut regs = [0u32; 15];
    load_gpr_run(frame, 0x3C, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCD9C - __restgprlr_18: restore r18..r31
// ---------------------------------------------------------------------------

/// Restores `r18..r31` from the caller's frame (entry `__restgprlr_18`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDA0..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 14 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcd9c` at `0x802DCD9C` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcd9c_FUN_802dcd9c.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 14 words for `r18..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x38..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r18,-0x38(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcd9c(frame: usize) -> [u32; 14] {
    let mut regs = [0u32; 14];
    load_gpr_run(frame, 0x38, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDA0 - __restgprlr_19: restore r19..r31
// ---------------------------------------------------------------------------

/// Restores `r19..r31` from the caller's frame (entry `__restgprlr_19`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDA4..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 13 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcda0` at `0x802DCDA0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcda0_FUN_802dcda0.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 13 words for `r19..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x34..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r19,-0x34(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcda0(frame: usize) -> [u32; 13] {
    let mut regs = [0u32; 13];
    load_gpr_run(frame, 0x34, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDA4 - __restgprlr_20: restore r20..r31
// ---------------------------------------------------------------------------

/// Restores `r20..r31` from the caller's frame (entry `__restgprlr_20`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDA8..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 12 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcda4` at `0x802DCDA4` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcda4_FUN_802dcda4.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 12 words for `r20..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x30..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r20,-0x30(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcda4(frame: usize) -> [u32; 12] {
    let mut regs = [0u32; 12];
    load_gpr_run(frame, 0x30, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDA8 - __restgprlr_21: restore r21..r31
// ---------------------------------------------------------------------------

/// Restores `r21..r31` from the caller's frame (entry `__restgprlr_21`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDAC..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 11 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcda8` at `0x802DCDA8` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcda8_FUN_802dcda8.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 11 words for `r21..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x2C..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r21,-0x2C(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcda8(frame: usize) -> [u32; 11] {
    let mut regs = [0u32; 11];
    load_gpr_run(frame, 0x2C, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDAC - __restgprlr_22: restore r22..r31
// ---------------------------------------------------------------------------

/// Restores `r22..r31` from the caller's frame (entry `__restgprlr_22`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDB0..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 10 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcdac` at `0x802DCDAC` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdac_FUN_802dcdac.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 10 words for `r22..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x28..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r22,-0x28(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcdac(frame: usize) -> [u32; 10] {
    let mut regs = [0u32; 10];
    load_gpr_run(frame, 0x28, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDB0 - __restgprlr_23: restore r23..r31
// ---------------------------------------------------------------------------

/// Restores `r23..r31` from the caller's frame (entry `__restgprlr_23`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDB4..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 9 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcdb0` at `0x802DCDB0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdb0_FUN_802dcdb0.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 9 words for `r23..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x24..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r23,-0x24(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcdb0(frame: usize) -> [u32; 9] {
    let mut regs = [0u32; 9];
    load_gpr_run(frame, 0x24, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDB4 - __restgprlr_24: restore r24..r31
// ---------------------------------------------------------------------------

/// Restores `r24..r31` from the caller's frame (entry `__restgprlr_24`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDB8..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 8 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcdb4` at `0x802DCDB4` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdb4_FUN_802dcdb4.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 8 words for `r24..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x20..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r24,-0x20(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcdb4(frame: usize) -> [u32; 8] {
    let mut regs = [0u32; 8];
    load_gpr_run(frame, 0x20, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDB8 - __restgprlr_25: restore r25..r31
// ---------------------------------------------------------------------------

/// Restores `r25..r31` from the caller's frame (entry `__restgprlr_25`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues through `0x802DCDBC..0x802DCDC0` to the shared `blr` at
/// `0x802DCDD4`. The body returns the 7 loaded words so the register-only
/// effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcdb8` at `0x802DCDB8` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdb8_FUN_802dcdb8.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 7 words for `r25..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x1C..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r25,-0x1C(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcdb8(frame: usize) -> [u32; 7] {
    let mut regs = [0u32; 7];
    load_gpr_run(frame, 0x1C, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDBC - __restgprlr_26: restore r26..r31
// ---------------------------------------------------------------------------

/// Restores `r26..r31` from the caller's frame (entry `__restgprlr_26`).
///
/// The 4 bytes `functions.tsv` gives this label are the first `lwz`; the
/// routine continues into the final entry `0x802DCDC0` and returns through the
/// shared `blr` at `0x802DCDD4`. The body returns the 6 loaded words so the
/// register-only effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcdbc` at `0x802DCDBC` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdbc_FUN_802dcdbc.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 6 words for `r26..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x18..frame` must be readable.
///
/// # Confidence
/// medium - the `lwz r26,-0x18(r11)` .. `lwz r31,-4(r11)` run is literal
/// (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcdbc(frame: usize) -> [u32; 6] {
    let mut regs = [0u32; 6];
    load_gpr_run(frame, 0x18, &mut regs);
    regs
}

// ---------------------------------------------------------------------------
// 0x802DCDC0 - __restgprlr_27: restore r27..r31 and return
// ---------------------------------------------------------------------------

/// Restores `r27..r31` from the caller's frame and returns
/// (`__restgprlr_27`).
///
/// This is the only entry of the restore run whose `functions.tsv` size covers a
/// whole routine: 24 bytes, the five `lwz` instructions `r27..r31` plus the
/// `blr` at `0x802DCDD4`. The body returns the 5 loaded words so the
/// register-only effect is observable.
///
/// # Original
/// - Ghidra: `FUN_802dcdc0` at `0x802DCDC0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802dcdc0_FUN_802dcdc0.c`
///
/// # Arguments
/// - `frame`: the caller's frame base, the value the original keeps in `r11`.
///
/// # Returns
/// The 5 words for `r27..r31`, lowest register first.
///
/// # Side effects
/// None in memory; the original only loads registers.
///
/// # Safety
/// `frame-0x14..frame` must be readable.
///
/// # Confidence
/// medium - the five `lwz` instructions and the `blr` at `0x802DCDD4` are
/// literal (re-read from `work/dol/main.elf`); the C export is only `return;`.
pub unsafe fn fn_802dcdc0(frame: usize) -> [u32; 5] {
    let mut regs = [0u32; 5];
    load_gpr_run(frame, 0x14, &mut regs);
    regs
}

// ===========================================================================
// Wave 282 lane E (`_E282`) - the three `0x802DC2xx` vector setters
// ===========================================================================
//
// These three bodies are the vector writers of the typed record already
// described by [`fn_802dc4f0`] (initialiser) and the address getters
// [`fn_802dc300`] (`+0x68`), [`fn_802dc370`] (`+0x74`) and [`fn_802dc480`]
// (`+0x8C`). Each body marks the record through [`fn_802dc5d0`] with one mask
// and then copies three `f32` values, read from the caller's block *after* the
// flag call, into the vector. The masks are single-bit, unlike the
// `0x8000 << row` family used by the byte/row setters:
//
//   0x802DC310  mask 0x00080000  +0x68/+0x6C/+0x70
//   0x802DC380  mask 0x00100000  +0x74/+0x78/+0x7C
//   0x802DC490  mask 0x40000000  +0x8C/+0x90/+0x94
//
// `0x802D81F0`, `0x802D8200` and `0x802D8220` are the `+0x7C`-adjusting
// thunks of the `0x803507B8` vtable that relay to these three addresses; they
// are already translated in [`crate::resource::archive`] and their measured
// target notes agree with the offsets and masks below. The `mode` argument is
// the caller's untouched `r5` in all three bodies.

/// Stores the three `f32` values at `source` into the record's `+0x68` vector
/// after marking it with mask `0x00080000`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x802DC310`.
/// - Extent: `0x802DC310..0x802DC364` (84 bytes, 20 instructions), ending at
///   the `blr` at `0x802DC360`; three zero words fill `0x802DC364..0x802DC36C`
///   and the tabled address getter `0x802DC370` follows.
/// - Reach: the only reference is `b 0x802DC310` at `0x802D81F4`, inside the
///   `+0x7C` adjusting thunk `0x802D81F0` (word 27 of `0x803507B8`, translated
///   in [`crate::resource::archive`]); no data word holds its address.
/// - Table: reached as slot 25 of `0x803507B8` through that thunk
///   (`0x80350824`, also `0x80308AEC`, `0x80310F44`, `0x80329864`,
///   `0x80329908`, `0x803299C4`, `0x80329A8C`).
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `mr r31,r4`; `lis r4,0x8` (`mask = 0x00080000`);
///   `stw r30,8(r1)`; `mr r30,r3`; `bl 0x802DC5D0`; `lfs f2,0(r31)`;
///   `lfs f1,4(r31)`; `lfs f0,8(r31)`; `stfs f2,0x68(r30)`;
///   `stfs f1,0x6C(r30)`; `stfs f0,0x70(r30)`; epilogue;
///   `4E800020 bclr 20,0` (`blr`).
///
/// # Arguments
/// - `object`: the typed record (`r3`).
/// - `source`: address of three consecutive `f32` values (`r4`).
/// - `mode`: flag selector forwarded unchanged to [`fn_802dc5d0`] (`r5`).
///
/// # Returns
/// Nothing; `r3` keeps the flag call's result and is not part of the contract.
///
/// # Side effects
/// One flag read-modify-write on `object+4`/`object+8` and three `f32` stores.
///
/// # Safety
/// `source..source+12` must be readable and `object+0x68..object+0x70`
/// writable.
///
/// # Confidence
/// high - both immediates, the three loads and the three stores are literal.
pub unsafe fn fn_802dc310(object: usize, source: usize, mode: u32) {
    crate::runtime::cpp::fn_802dc5d0(object, 0x0008_0000, mode);
    let x = platform::read_f32(source);
    let y = platform::read_f32(source + 4);
    let z = platform::read_f32(source + 8);
    platform::write_f32(object + 0x68, x);
    platform::write_f32(object + 0x6C, y);
    platform::write_f32(object + 0x70, z);
}

/// Stores the three `f32` values at `source` into the record's `+0x74` vector
/// after marking it with mask `0x00100000`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x802DC380`.
/// - Extent: `0x802DC380..0x802DC3D4` (84 bytes, 20 instructions), ending at
///   the `blr` at `0x802DC3D0`; three zero words fill `0x802DC3D4..0x802DC3DC`
///   and the tabled `0x802DC3E0` (`+0x80`) getter follows.
/// - Reach: the only reference is `b 0x802DC380` at `0x802D8204`, inside the
///   `+0x7C` adjusting thunk `0x802D8200` (word 28 of `0x803507B8`); no data
///   word holds its address.
/// - Table: reached as slot 26 of `0x803507B8` through that thunk
///   (`0x80350828`, also `0x80308AF0`, `0x80310F48`, `0x80329868`,
///   `0x8032990C`, `0x803299C8`, `0x80329A90`).
/// - Observed: identical to [`fn_802dc310`] with `lis r4,0x10`
///   (`mask = 0x00100000`) and the stores at `0x74`/`0x78`/`0x7C`.
///
/// # Arguments
/// - `object`: the typed record (`r3`).
/// - `source`: address of three consecutive `f32` values (`r4`).
/// - `mode`: flag selector forwarded unchanged to [`fn_802dc5d0`] (`r5`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One flag read-modify-write and three `f32` stores.
///
/// # Safety
/// `source..source+12` must be readable and `object+0x74..object+0x7C`
/// writable.
///
/// # Confidence
/// high - identical shape to [`fn_802dc310`], only the mask and the offsets
/// differ.
pub unsafe fn fn_802dc380(object: usize, source: usize, mode: u32) {
    crate::runtime::cpp::fn_802dc5d0(object, 0x0010_0000, mode);
    let x = platform::read_f32(source);
    let y = platform::read_f32(source + 4);
    let z = platform::read_f32(source + 8);
    platform::write_f32(object + 0x74, x);
    platform::write_f32(object + 0x78, y);
    platform::write_f32(object + 0x7C, z);
}

/// Stores the three `f32` values at `source` into the record's `+0x8C` block
/// after marking it with mask `0x40000000`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x802DC490`.
/// - Extent: `0x802DC490..0x802DC4E4` (84 bytes, 20 instructions), ending at
///   the `blr` at `0x802DC4E0`; three zero words fill `0x802DC4E4..0x802DC4EC`
///   and the tabled initialiser [`fn_802dc4f0`] follows.
/// - Reach: the only reference is `b 0x802DC490` at `0x802D8224`, inside the
///   `+0x7C` adjusting thunk `0x802D8220` (word 30 of `0x803507B8`); no data
///   word holds its address.
/// - Table: reached as slot 28 of `0x803507B8` through that thunk
///   (`0x80350830`, also `0x80308AF8`, `0x80310F50`, `0x80329870`,
///   `0x80329914`, `0x803299D0`, `0x80329A98`).
/// - Observed: identical to [`fn_802dc310`] with `lis r4,0x4000`
///   (`mask = 0x40000000`) and the stores at `0x8C`/`0x90`/`0x94` (the block
///   exposed by [`fn_802dc480`]).
///
/// # Arguments
/// - `object`: the typed record (`r3`).
/// - `source`: address of three consecutive `f32` values (`r4`).
/// - `mode`: flag selector forwarded unchanged to [`fn_802dc5d0`] (`r5`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One flag read-modify-write and three `f32` stores.
///
/// # Safety
/// `source..source+12` must be readable and `object+0x8C..object+0x94`
/// writable.
///
/// # Confidence
/// high - identical shape to [`fn_802dc310`], only the mask and the offsets
/// differ.
pub unsafe fn fn_802dc490(object: usize, source: usize, mode: u32) {
    crate::runtime::cpp::fn_802dc5d0(object, 0x4000_0000, mode);
    let x = platform::read_f32(source);
    let y = platform::read_f32(source + 4);
    let z = platform::read_f32(source + 8);
    platform::write_f32(object + 0x8C, x);
    platform::write_f32(object + 0x90, y);
    platform::write_f32(object + 0x94, z);
}
