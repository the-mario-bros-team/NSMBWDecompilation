//! `0x802E9300` / `0x802E9398`: the MetroTRK debugger's boot entry points.
//!
//! **These are not the game loop.** The real frame loop is
//! [`crate::engine::startup::frame_loop`] (`0x800C9F40` calls `0x800E4E40`).
//! This file exists because the DOL entry can hand the machine to these two
//! functions instead of to the game, and a reader of the boot chain needs to
//! know where that branch goes.
//!
//! ## What this module covers
//!
//! Original address range `0x802E14F4..0x802ED9CC`, i.e. the whole
//! `0x802E0000` bucket: statically linked C-runtime string helpers at the low
//! end, the MetroTRK debugger in the middle, and the interrupt / exception
//! knobs at the high end. The file started with the two debug-boot entries
//! below; later waves added the clusters around them. The two 148/144-byte
//! entry points sit inside the MetroTRK cluster:
//!
//! | address | size | name here | role |
//! |---|---|---|---|
//! | `0x802E9300` | 148 B | [`fn_802e9300`] | debug boot, external interrupts **off**, hardware id from the caller |
//! | `0x802E9398` | 144 B | [`fn_802e9398`] | debug boot, external interrupts **on**, hardware id fixed to 2 |
//!
//! Both save the whole CPU state, ask `0x802E96E4` to select the target
//! hardware, then tail-call the debugger main loop `0x802E8E54`.
//!
//! ## Wave 36 - MetroTRK core (`0x802E8E54..0x802E9EAC`)
//!
//! Wave 36 adds 28 more bodies from the same MetroTRK cluster, immediately
//! around the boot entries above: the debugger main wrapper [`fn_802e8e54`]
//! and dispatch loop [`fn_802e8e90`], the address-map / init-flag helpers
//! ([`fn_802e9430`], [`fn_802e95cc`]), the handler-table trampolines
//! ([`fn_802e9820`], [`fn_802e9874`], [`fn_802e98a0`], [`fn_802e98b4`],
//! [`fn_802e98f0`], [`fn_802e992c`], [`fn_802e9940`]), the print / copy
//! helpers ([`fn_802e9954`], [`fn_802e996c`]), and the two-slot event queue
//! ([`fn_802e9a50`], [`fn_802e9a74`], [`fn_802e9b00`], [`fn_802e9bcc`],
//! [`fn_802e9bec`], [`fn_802e9c98`], [`fn_802e9cbc`], [`fn_802e9e08`],
//! [`fn_802e9e34`], [`fn_802e9e74`], [`fn_802e9e7c`], [`fn_802e9e84`]).
//! Together with the two boot entries the module covered 30 bodies at that
//! point in `0x802E8E54..0x802E9EAC` plus the two boot entries at `0x802E9300`
//! / `0x802E9398` inside that span.
//!
//! The untranslated machinery that still surrounds this cluster is called
//! by absolute address: the unaligned byte copy `0x802E8F7C`, the dispatcher
//! `0x802E91E0`, the bulk-flush `0x802E9498`, the reply reader `0x802E9CC8`
//! / `0x802E9D3C`.
//!
//! ## Wave 57 - UART ring buffer, message-buffer pool and platform knobs
//!
//! Wave 57 adds 30 bodies. The first group is the debug port's UART ring
//! buffer `0x802E886C..0x802E8D14`: the case-insensitive compare, buffer
//! constructor, blocking write / read, the interrupt-masked critical
//! section pair, and the small read-out wrappers. The second group is the
//! three 0x88C-byte TRK message buffers at `0x80425720` and the one-word
//! platform knobs at `0x802EA820..0x802ED9CC` (transmit, buffer
//! allocation / release, and the `0x80427230` / `0x80427238` getters and
//! setters used by the interrupt path). After Wave 57 the module held 60
//! bodies.
//!
//! ## Wave 68 - C string helpers and the TRK payload accessors
//!
//! Wave 68 adds 29 bodies in three clusters, all inside the same bucket.
//! First, the statically linked C runtime at the bottom of the bucket: the
//! byte-string routines `strncpy` / `strcat` / `strncat` / `strcmp` /
//! `strncmp` / `strchr` / `strrchr` / `strstr` (`0x802E19F8..0x802E1C88`) and
//! their wide-character twins plus the 2-byte-element copy helper
//! `0x802E21CC` (`0x802E4324..0x802E44CC`). Second, the byte-buffer append
//! callback `0x802E14F4`, which is the narrow twin of the wide append
//! `0x802E4324`. Third, the TRK message-payload accessors
//! `0x802EAA88..0x802EAFA8` - read bytes out, write a byte / word /
//! word-pair / word-array in, read a word / byte with the big-endian swap -
//! plus the event-buffer release `0x802E9BE4` and the four `0x80`
//! notification senders `0x802EB098..0x802EB1FC`. The module now holds 89
//! bodies.
//!
//! ## Wave 82 - storage primitives, request dispatcher and breakpoint knobs
//!
//! Wave 82 adds 25 bodies, all inside the same bucket. The low end gains the
//! two unaligned storage primitives `0x802E8F7C` (byte copy) and `0x802E90B8`
//! (byte fill) that the Wave 57 / 68 bodies already call by address, plus the
//! C-runtime routines `strcpy` `0x802E1938`, `wmemchr` `0x802E21D4` and the
//! `atoi` wrapper `0x802E2108`. The message layer gains the command dispatcher
//! `0x802E91E0`, the 8-byte payload append `0x802EABE8`, the four block
//! request handlers `0x802EB268` (read memory), `0x802EB49C` (write memory),
//! `0x802EB6AC` (read registers) and `0x802EB8A8` (write registers), the
//! continue / breakpoint / status / serial-option handlers
//! `0x802EBB48..0x802EBF48`, and the exception teardown `0x802EA4C4`. The high
//! end gains the request state machine `0x802ED2F4`, the `blr` patch helper
//! `0x802ED8B4`, the thread-index walk `0x802ED934`, the cache-window test
//! `0x802EC1B0`, the `0x91` support reply `0x802ED028` and the three
//! breakpoint knobs `0x802ED0C4` / `0x802ED1E8` / `0x802ED270`. After Wave 82
//! the module holds 114 bodies.
//!
//! ## Wave 91 - MSL `printf` / `strtol` support and the TRK reply path
//!
//! Wave 91 adds 25 bodies, all inside the same bucket. The C-runtime end gains
//! the MSL formatter and parser: the shared integer parser `0x802E1CF4`,
//! the 32-bit and 64-bit radix converters `0x802E278C` / `0x802E29D0`, the
//! decimal rounding helper `0x802E032C` and its byte-identical twin
//! `0x802E30E0`, the `printf` entry points `0x802E1560` / `0x802E15DC` /
//! `0x802E1664` with their append callback `0x802E149C`, the stream
//! orientation helper `0x802E44F8`, and the DMA-pool / flush pair
//! `0x802E4570` / `0x802E4648`. The debugger end gains the devkit selection
//! `0x802E96E4`, the segment writeback `0x802E9498`, the empty `0x802E981C`,
//! the reply dispatcher `0x802E99C0`, the reply reader `0x802E9D3C`, the
//! request sender `0x802EA094`, the D0/D1 chunk transfer `0x802E9EA0`, the
//! three D2/D3/D4 senders `0x802EA1BC` / `0x802EA2D4` / `0x802EA3B8`, the
//! breakpoint-reset thunk `0x802E942C` and the two eight-byte flush
//! forwarders `0x802EC0E4` / `0x802EC0EC`. Three Wave 82 call-by-address
//! sites now call the translated bodies directly: [`trk_boot`] calls
//! [`fn_802e96e4`], [`fn_802eb1fc`] calls [`fn_802e9498`], and
//! [`fn_802ed2f4`] calls the four request senders. After Wave 91 the module
//! holds 139 bodies.
//!
//! ## Wave 95 - MSL math kernels and the last TRK transfers
//!
//! Wave 95 adds 25 bodies, all inside the same bucket. The MSL cluster gains
//! fourteen routines: the trivial `copysign` `0x802E7F90`, `ceil` / `floor`
//! (`0x802E7E44` / `0x802E8084`), `frexp` `0x802E81D4`, `ldexp` `0x802E825C`,
//! `modf` `0x802E83C8`, the `log` / `log1p` pair `0x802E5490` / `0x802E51DC`,
//! four inverse-tangent kernels (`0x802E613C`, `0x802E7904`, `0x802E79C4`,
//! `0x802E7C14`), the software square root `0x802E8620` and the `acos` body
//! `0x802E46A0`. Their `.sdata2` constants are addressed through the
//! [`sda2`] helper, so the bodies read the shipped image instead of embedding
//! guessed numbers; the decompiler's `GQR0` probes are documented as omitted
//! paired-single artifacts on the two bodies that have them.
//!
//! The debugger end gains nine bodies: the two `0x802EC014` / `0x802EC07C`
//! transport guards, the `0x802EC0F4` trap-stub transfer, the `0x802ECDCC`
//! status trigger, the `0x802ECE28` `0x90` reply builder, the GPR / SPR block
//! transfers `0x802EC4C4` / `0x802EC6F8`, and the two instruction-patch
//! builders `0x802ED520` / `0x802ED5FC`. Two four-byte tails,
//! [`fn_802e8618`] and [`fn_802e8868`], forward to the now-translated
//! `0x802E5490` and `0x802E8620`. After Wave 95 the module holds 164 bodies.
//!
//! ## Wave 99 - the sine / cosine / tangent wrappers and the last TRK transfers
//!
//! Wave 99 adds 20 bodies, all inside the same bucket, and the module now holds
//! 184 bodies. The MSL cluster gains seven routines: the three wrappers
//! `0x802E7FBC` (`cos`), `0x802E84C4` (`sin`) and `0x802E8590` (`tan`), the
//! `scalbn` body `0x802E4660` that `pow` uses, the `rem_pio2` range reduction
//! `0x802E5DB8` they all share, the two-branch `atan` `0x802E496C` and the
//! `atan2` quadrant reconstruction `0x802E4C04`. Four four-byte tails
//! ([`fn_802e8608`], [`fn_802e860c`], [`fn_802e8610`], [`fn_802e8614`]) forward
//! into that group. The `fmod` kernel `0x802E4E5C` completes the arithmetic set.
//! Note that the two kernels Wave 95 labelled inverse-tangent tails
//! (`0x802E613C` / `0x802E7904`) are the MSL `__kernel_cos` / `__kernel_sin`
//! pair: the new wrappers call them in exactly those roles.
//!
//! The debugger end gains five bodies: the unaligned block copy `0x802EC2A8`,
//! the mapped block transfer `0x802EC374`, the `mfspr` / `mtspr` patch builder
//! `0x802ED6EC` and the two eight-byte-per-index transfers `0x802EC5BC` /
//! `0x802EC85C`. The format end gains the two variadic `sprintf` entries
//! `0x802E16E8` / `0x802E17DC`, and `0x802EA6BC` clears the two exception
//! flags. [`fn_802e861c`] stays untranslated because its target `0x802E55A4`
//! (`pow`) calls the `sqrt` thunk with an argument list Ghidra dropped.
//!
//! ## Wave 105 - the wide-format parser, driver and `%a` converter
//!
//! Wave 105 adds four bodies, all inside the same bucket, and the module now
//! holds 193 bodies. The wide `printf` family is the twin of the narrow one
//! that Wave 104 landed: the format-spec parser `0x802E21FC`, the driver
//! `0x802E3998` and the `%a` / `%A` hexadecimal-float converter `0x802E2C8C`.
//! With them the wide driver calls all three directly instead of by absolute
//! address; only the two `mbstowcs` / `mbtowc` helpers (`0x802DEDFC` /
//! `0x802DED94`) stay outside this lane. The fourth body is `0x802E9CC8`, the
//! MetroTRK "reply pending" flag setter, which is `low` confidence: its
//! 116-byte extent contains three blocks Ghidra removed as unreachable, so
//! only the two identical stores and the zero return are represented.
//!
//! ## Wave 106 - the privileged leaves, FPSCR pair, cache flush and SPR dump
//!
//! Wave 106 adds seven bodies and leaves three. Five are the routines the
//! earlier waves could only call by absolute address because Ghidra prints an
//! empty body for them: the MSR leaves `0x802EC1A0` (`mfmsr r3; blr`) and
//! `0x802EC1A8` (`mtmsr r3; blr`); the FPSCR accessors `0x802ED6A4`
//! (`mffs f31` + `stfd f31,0(r3)`, preserving `f31` through a `psq_st` /
//! `psq_l` pair) and `0x802ED6C8` (`lfd f31,0(r3)` + `mtfsf 255,f31`); and the
//! unconditional trap gate `0x802E890C` (`twi 31,r0,0`). The other two are the
//! cache-range publish `0x802E8E1C` (`dcbst` / `dcbf` / `sync` / `icbi` over an
//! 8-byte step) and the 452-byte SPR dump `0x802EA4F8`, which reads 96
//! registers into `0x804273E8..0x80427568`. Instruction words were read from
//! `work/dol/main.elf` wherever the decompiler dropped a block.
//!
//! The bodies emit the original instruction under
//! `#[cfg(target_arch = "powerpc")]`; the host twins are documented stand-ins
//! so that `cargo check` succeeds on the development machine, the same pattern
//! used by `boot::cache` and `runtime::os`. After Wave 106 the module holds 200
//! bodies. See `docs/COVERAGE_WAVE106_MAIN_LOOP.md`.
//!
//! ## Wave 111 - the Payne-Hanek tail
//!
//! Wave 111 adds one body, the 5816-byte `0x802E624C` that the earlier waves
//! could only reach through `platform::call6`: the MSL `__kernel_rem_pio2`
//! range reduction that [`fn_802e5db8`] uses above `2^20 * pi/2`. The
//! 8-way-unrolled convolution loops of the export are landed as the loops
//! Ghidra itself reconstructed, the 24-bit `2/pi` chunks come from
//! `ipio2 = 0x802FE030`, the term count from the shipped `init_jk` table at
//! `0x802FE1B8` (`{2, 3, 4, 6}` - not the fdlibm `{3, 4, 4, 6}`), the `pi/2`
//! chunks from `0x802FE1C8`, and the eight `.sdata2` scratch constants from
//! `0x8042FA78..0x8042FAB0`. [`fn_802e5db8`] now calls the body directly
//! instead of by address. After Wave 111 the module holds 201 bodies and the
//! bucket's remaining two members are the exception prologue / epilogue at the
//! top of the range.
//! See `docs/COVERAGE_WAVE111_MAIN_LOOP.md`.
//!
//! ## Wave 113 - the two exception frames close the bucket
//!
//! Wave 113 adds the bucket's last two addresses, both `partial`:
//! `0x802EC9CC` (744-byte exception prologue, [`fn_802ec9cc`]) and
//! `0x802ECCB4` (280-byte exception epilogue, [`fn_802eccb4`]). They save and
//! restore every GPR plus `LR` / `CTR` / `XER` / `CR` / `MSR` / `DAR` /
//! `DSISR`, so their bodies carry the ordinary C part - the `MSR & !0x8002`
//! mask, the exception-number and dispatch-flag stores, the nested-exception
//! ladder, the 3 / 4 / 5 event codes and the calls into the translated TRK
//! bodies - and list each register-only step at its address. The shared tail
//! `0x802ECBFC`, a third entry point inside the prologue's extent, is
//! [`exception_dispatch_tail`]. The context trampoline `0x802E9624` now calls
//! the prologue directly instead of `call8_u32`, and [`fn_802ea4c4`] calls the
//! epilogue directly; the `0x0FE00000` compare in the tail is the
//! `twi 31,r0,0` encoding of [`fn_802e890c`]. After Wave 113 the module holds
//! 204 bodies over all 203 addresses of the bucket (`fn_802e890c` has a target
//! and a host twin). See `docs/COVERAGE_WAVE113_MAIN_LOOP.md`.
//!
//! **r13 / r2 note.** The bodies read SDA globals through the register bases
//! named in `docs/GLOSSARY.md` (`0x800041C0` sets `r13 = 0x8042F680`,
//! `r2 = 0x80433080`). The constants below give the resolved absolute
//! addresses, e.g. `r13 - 0x4660 = 0x8042B020`.
//!
//! ## Why the boot entry reaches them
//!
//! ```text
//! boot type 2 -> 0x802E9300(hardware id 0)
//! boot type 3 -> 0x802E9300(hardware id 1)
//! boot type 4 -> 0x80004030 sets a flag; later -> 0x802E9398()
//! ```
//!
//! A retail disc boot uses none of these, so nothing here runs in normal play.
//!
//! ## Why this file is partial
//!
//! The two boot entries begin with `stmw`/`lmw`/`mfcr`/`mfmsr`/`mtmsr`/`mtspr`,
//! i.e. they save and restore the live machine state and replace the stack
//! pointer. Since Wave 106 the file does allow one-instruction stand-ins (the
//! `#[cfg(target_arch = "powerpc")]` helpers at the end of the file), but the
//! boot prologue still replaces `r1` and restores `r0..r31`, which needs a
//! whole-function assembly block rather than a leaf helper, so it stays
//! documented instruction by instruction instead of executed. What *is*
//! translated is the part that is a normal call sequence: the SPR dump, the
//! hardware-id query and the branch on its result.
//!
//! The bucket's last two members, the exception prologue `0x802EC9CC` and
//! epilogue `0x802ECCB4`, were the same kind of case: their Ghidra exports
//! print the `stmw` / `lmw` register runs as independent `in_rNN` field stores,
//! print `mtlr` / `mtspr` as global writes, and print each `rfi` teardown as a
//! `returnFromInterrupt(msr, srr0)` pseudo-call. Wave 113 decoded the
//! instruction words from `work/dol/main.elf` and landed the ordinary C part as
//! [`fn_802ec9cc`] / [`fn_802eccb4`] (`partial`); the register save / restore
//! and the three exception returns are documented at the addresses they occupy
//! rather than executed. `docs/COVERAGE_WAVE113_MAIN_LOOP.md` holds the full
//! decoded listing and the save maps.
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - indirect calls by absolute address.
//! * The MetroTRK bodies in this file: the boot entries, the debugger main
//!   loop ([`fn_802e8e54`]) and the hardware selection ([`fn_802e96e4`]).
//! * The MetroTRK SPR dump [`fn_802ea4f8`], which fills the SPR block at
//!   `0x804273E8`; Wave 106 translated it, so the boot path can call it.
//!
//! ## Start here
//!
//! Read [`fn_802e9300`], then [`fn_802e9398`] and the shared [`trk_boot`].
//!
//! ## Glossary (first use)
//!
//! * **MetroTRK** - Nintendo's target-resident debugger (the "TRK" in
//!   `devkitPPC`'s `TRK` library); it talks to a host debugger over USB/serial.
//!   It ships inside the game executable and is inert on a retail boot.
//! * **GPR / SPR** - general-purpose registers `r0..r31`; special-purpose
//!   registers (time base, `SRR1`, `HID0`, memory-management and cache control).
//! * **CR** - condition register; **MSR** - machine state register.
//! * **MSR.EE** - the "external interrupt enable" bit of the MSR (`0x8000`).
//! * **NDEV / GDEV** - the two devkit hardware revisions the debugger supports.
//! * **DOL** - the Wii executable container (`sys/main.dol`).

use crate::platform;

/// Start of the saved GPR/CR block, written by both entry points before
/// anything else.
///
/// `// inferred:` the base of MetroTRK's `TRKCPUState`-style save area. It is in
/// BSS inside the image (`0x80351680..0x8042FC3C`), so it is zero at boot.
///
/// # Original
/// - Ghidra: `FUN_802e9300` at `0x802e9300` (`DAT_80427240` in the C output)
/// - Evidence: `work/ghidra/main-gb/export/802e9300_FUN_802e9300.c`
/// - Observed: `0x802E9308 lis r3,0x8042` / `0x802E930C ori r3,r3,0x7240`
pub const TRK_GPR_CONTEXT: usize = 0x8042_7240;

/// Start of the saved SPR block, written by `0x802EA4F8`.
///
/// Covers `0x804273E8..0x8042756C` (the last store, `DAT_80427568` in the Ghidra
/// output, is the `DMAL` slot). Contents in order:
/// `SR0..SR15`, `TBU/TBL`, `HID0/1`, `SRR1`, `PVR`, `IBAT0..3`, `DBAT0..3`,
/// cache state, `SDR1`, `DAR`, `DSISR`, `SPRG0..3`, `DEC`, `IABR`, `EAR`,
/// `DABR`, `PMC1..4`, `SIA`, `MMCR0/1`, `ICTC`, `L2CR`, `SRR0`, `DC_DAT`,
/// `UMMCR0/1`, `UPMC1..4`, `USIA`, `HID2`, `WPAR`, `DMAU`, `DMAL`.
///
/// # Original
/// - Ghidra: `FUN_802ea4f8` at `0x802ea4f8` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea4f8_FUN_802ea4f8.c`
pub const TRK_SPR_CONTEXT: usize = 0x8042_73E8;

/// Stack top both entry points install before handing over to the debugger.
///
/// `0x80441C40` is above the DOL image end (`0x8042FC3C`), i.e. ordinary MEM1
/// that the linker never allocated. The value is hard-coded in the original.
///
/// # Original
/// - Ghidra: `FUN_802e9300` at `0x802e9300`
/// - Observed: `0x802E9368 lis r1,0x8044` / `0x802E936C ori r1,r1,0x1C40`
pub const TRK_STACK_TOP: usize = 0x8044_1C40;

/// `0x802E96E4` - selects the debugger's target hardware and installs the
/// message-handler table at `0x804256D0..0x804256F4`.
///
/// Wave 91 translated the body as [`fn_802e96e4`]; [`trk_boot`] calls it
/// directly now.
///
/// # Original
/// - Ghidra: `FUN_802e96e4` at `0x802e96e4` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e96e4_FUN_802e96e4.c`


/// `0x802EA4F8` - copies every readable SPR into [`TRK_SPR_CONTEXT`].
///
/// # Original
/// - Ghidra: `FUN_802ea4f8` at `0x802ea4f8` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea4f8_FUN_802ea4f8.c`
/// - `// unknown:` `r3` holds the post-`mtmsr` MSR value at the call site, but
///   the Ghidra body never reads its argument.
/// Wave 106 translated the body as [`fn_802ea4f8`], so [`trk_boot`] calls it
/// directly; the constant is kept for reference.
const TRK_DUMP_SPRS: usize = 0x802E_A4F8;

/// `0x802E8E54` - debugger main: initialise the transport (`0x802E9BEC`), print
/// `"MetroTRK for Revolution v0.4"` (`0x802E9CBC`) and dispatch host messages in
/// `0x802E8E90` until message type 1 arrives.
///
/// # Original
/// - Ghidra: `FUN_802e8e54` at `0x802e8e54` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8e54_FUN_802e8e54.c`
const TRK_MAIN: usize = 0x802E_8E54;

/// Body of one real game frame, reached through
/// [`crate::engine::startup::frame_loop`].
///
/// Listed in this file only so a reader cannot mistake the two MetroTRK entry
/// points for the main loop.
///
/// # Original
/// - Ghidra: `FUN_800e4e40` at `0x800e4e40` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800e4e40_FUN_800e4e40.c`
pub const GAME_FRAME_BODY: usize = 0x800E_4E40;

/// Hardware id meaning "GDEV", the fallback devkit revision.
///
/// `// inferred:` from `0x802E96E4`: the id-2 branch prints nothing and returns
/// success, which is what "already the default" looks like.
///
/// # Original
/// - Observed: `0x802E9404 li r3,2` in `0x802E9398`
const HW_ID_GDEV: u32 = 2;

/// Field offsets of the 0x1A8-byte save block at [`TRK_GPR_CONTEXT`].
///
/// # Original
/// - Ghidra: `FUN_802e9300` at `0x802e9300` / `FUN_802e9398` at `0x802e9398`
/// - Evidence: `work/ghidra/main-gb/export/802e9300_FUN_802e9300.c`
/// - `// unknown:` the bytes from `0x8C` to `0x1A8` are not written by either
///   entry point; the SPR block at `0x804273E8` is written by `0x802EA4F8`.
pub mod context_offset {
    /// `stmw r0,0(r3)`: `r0..r31`, 32 words. `+0x04` receives the stack pointer
    /// and `+0x0C` the first argument.
    pub const GPR: usize = 0x00;
    /// `stw r4,0x80(r3)` where `r4 = mfspr LR`.
    pub const LR: usize = 0x80;
    /// Same value as [`LR`]; this is the slot the failure path reloads into LR
    /// before `blr`, i.e. the resume address.
    pub const PC: usize = 0x84;
    /// `stw r4,0x88(r3)` where `r4 = mfcr`.
    pub const CR: usize = 0x88;
    /// End of the written area (start of the `// unknown:` gap).
    pub const WRITTEN_LEN: usize = 0x8C;
}

// ---------------------------------------------------------------------------
// Wave 36 - MetroTRK core constants.
// ---------------------------------------------------------------------------
//
// `main.dol` has no symbol table in the shipped image. The `FN_*` addresses
// below are the call targets named by the Ghidra bodies in
// `work/ghidra/main-gb/export/802e8e54_FUN_802e8e54.c` and friends; the
// `G_*` / `EVENT_*` / `HANDLER_SLOT_*` addresses are the SDA/BSS globals
// those bodies read or write. `r13 = 0x8042F680` and `r2 = 0x80433080`
// are from `docs/GLOSSARY.md` (`0x800041C0`).

/// `0x8042B020` - `r13 - 0x4660`, TRK init-result word written by
/// [`fn_802e8e54`] (and re-written by the teardown call `0x802E9C98`).
///
/// # Original
/// - Ghidra: `*(int *)((unaff_r13) + -0x4660)` in `802e8e54_FUN_802e8e54.c`
const G_TRK_INIT_RESULT: usize = 0x8042_B020;

/// `0x8042B028` - `r13 - 0x4658`, TRK address-base word read by
/// [`fn_802e9430`] and re-seeded to `0xE0000000` by [`fn_802e95cc`].
///
/// # Original
/// - Ghidra: `*(uint *)((unaff_r13) + -0x4658)` in `802e9430_FUN_802e9430.c`
const G_TRK_ADDR_BASE: usize = 0x8042_B028;

/// `0x8042B030` - `r13 - 18000` (`0x4650`). Byte flag that gates the
/// indirect call in [`fn_802e9874`].
///
/// # Original
/// - Ghidra: `*(char *)((unaff_r13) + -18000)` in `802e9874_FUN_802e9874.c`
/// - Observed: `-18000 == -0x4650`, so `0x8042F680 - 18000 = 0x8042B030`
const G_TRK_GUARD_BYTE: usize = 0x8042_B030;

/// `0x8042B040` - `r13 - 0x4640`, TRK context pointer slot shared by
/// [`fn_802e9bec`] (as argument to `0x802E9820`) and [`fn_802e8e90`]
/// (double-dereferenced to poll a byte).
///
/// # Original
/// - Ghidra: `(unaff_r13) + -0x4640` in `802e9bec_FUN_802e9bec.c` /
///   `802e8e90_FUN_802e8e90.c`
const G_TRK_CONTEXT_PTR: usize = 0x8042_B040;

/// `0x8042FBC0` - `r2 - 0x34C0`, four-byte source word copied by
/// [`fn_802e996c`].
///
/// # Original
/// - Ghidra: `(unaff_r2) + -0x34c0` in `802e996c_FUN_802e996c.c`
/// - Observed: `r2 = 0x80433080` per `docs/GLOSSARY.md`, so
///   `0x80433080 - 0x34C0 = 0x8042FBC0`
const G_R2_SRC_34C0: usize = 0x8042_FBC0;

/// `0x80427230` - byte-set word written `1` by [`fn_802e95cc`].
///
/// # Original
/// - Ghidra: `DAT_80427230` in `802e95cc_FUN_802e95cc.c`
const G_80427230: usize = 0x8042_7230;

/// `0x80427224` - word overwritten by [`fn_802e95cc`] with the (indeterminate)
/// return register of `0x802EC1A0`.
///
/// # Original
/// - Ghidra: `DAT_80427224` in `802e95cc_FUN_802e95cc.c`
const G_80427224: usize = 0x8042_7224;

/// `0x80427198` / `0x80427234` - the saved-context block and the "request in
/// flight" byte the two exception frames use.
///
/// The block at `0x80427198` holds `r0..r31` (+0..+0x7C), LR (+0x80), CTR
/// (+0x84), XER (+0x88), MSR (+0x8C), DAR (+0x90) and DSISR (+0x94); the byte
/// at `0x80427234` is the flag the prologue, the epilogue and the shared tail
/// at `0x802ECBFC` set, test and clear.
///
/// # Original
/// - Ghidra: `DAT_80427198` / `_DAT_80427234` in `802ec9cc_FUN_802ec9cc.c` and
///   `802eccb4_FUN_802eccb4.c`
const G_80427198: usize = 0x8042_7198;
const G_80427234: usize = 0x8042_7234;

/// `0x8042B058` - halfword exception number the prologue stores and the tail
/// re-reads.
///
/// # Original
/// - Ghidra: `DAT_8042b058` in `802ec9cc_FUN_802ec9cc.c`
const G_8042B058: usize = 0x8042_B058;

/// `0x80427478` flag word read by [`fn_802e9430`] (`(flag & 3) != 0`
/// selects the pass-through branch).
///
/// # Original
/// - Ghidra: `DAT_80427478` in `802e9430_FUN_802e9430.c`
const G_FLAG_80427478: usize = 0x8042_7478;

/// Handler-table slots installed by `0x802E96E4` at
/// `0x804256D0..0x804256F4` (see module header). Each slot holds one
/// function pointer, dereferenced by the trampolines below.
///
/// # Original
/// - Ghidra: `DAT_804256d0` .. `DAT_804256f4` in the `802e98*` / `802e99*`
///   exports
const HANDLER_SLOT_804256D0: usize = 0x8042_56D0;
const HANDLER_SLOT_804256D4: usize = 0x8042_56D4;
const HANDLER_SLOT_804256DC: usize = 0x8042_56DC;
const HANDLER_SLOT_804256E0: usize = 0x8042_56E0;
const HANDLER_SLOT_804256E4: usize = 0x8042_56E4;
const HANDLER_SLOT_804256E8: usize = 0x8042_56E8;
const HANDLER_SLOT_804256F0: usize = 0x8042_56F0;
const HANDLER_SLOT_804256F4: usize = 0x8042_56F4;

/// Event-queue state, all four globals shared by [`fn_802e9a50`],
/// [`fn_802e9a74`], [`fn_802e9b00`] and [`fn_802e9e34`].
///
/// # Original
/// - Ghidra: `DAT_804256f8` (count), `DAT_804256fc` (head),
///   `DAT_80425718` (sequence), `UNK_80425700` (two 12-byte slots)
const EVENT_COUNT: usize = 0x8042_56F8;
const EVENT_HEAD: usize = 0x8042_56FC;
const EVENT_SEQ: usize = 0x8042_5718;
const EVENT_SLOTS: usize = 0x8042_5700;

/// Format / string addresses named as `DAT_...` / `s_...` in the exports.
///
/// # Original
/// - `&DAT_803514d5` in `802e9954_FUN_802e9954.c`
/// - `s_MetroTRK_for_Revolution_v0_4_80351500` in `802e9cbc_FUN_802e9cbc.c`
/// - `s_MetroTRK___Event_Queue_full_803514e0` in `802e9b00_FUN_802e9b00.c`
const FMT_803514D5: usize = 0x8035_14D5;
const STR_BANNER_80351500: usize = 0x8035_1500;
const STR_QUEUE_FULL_803514E0: usize = 0x8035_14E0;

const FN_802E9BEC: usize = 0x802E_9BEC;
const FN_802E9CBC: usize = 0x802E_9CBC;
const FN_802E8E90: usize = 0x802E_8E90;
const FN_802E9C98: usize = 0x802E_9C98;
const FN_802E9CC8: usize = 0x802E_9CC8;
const FN_802E9A50: usize = 0x802E_9A50;
const FN_802EA884: usize = 0x802E_A884;
const FN_802E996C: usize = 0x802E_996C;
const FN_802E9E74: usize = 0x802E_9E74;
const FN_802E95CC: usize = 0x802E_95CC;
const FN_802E9820: usize = 0x802E_9820;
const FN_802ED91C: usize = 0x802E_D91C;
const FN_802E9E7C: usize = 0x802E_9E7C;
const FN_802E9954: usize = 0x802E_9954;
const FN_802E9A74: usize = 0x802E_9A74;
const FN_802E9E08: usize = 0x802E_9E08;
const FN_802E9D3C: usize = 0x802E_9D3C;
const FN_802E9E34: usize = 0x802E_9E34;
const FN_802E9B00: usize = 0x802E_9B00;
const FN_802E9BCC: usize = 0x802E_9BCC;
const FN_802ECDCC: usize = 0x802E_CDCC;
const FN_802EA940: usize = 0x802E_A940;
const FN_802E91E0: usize = 0x802E_91E0;
const FN_802E9BE4: usize = 0x802E_9BE4;
const FN_802ED4E8: usize = 0x802E_D4E8;
const FN_802EA4C4: usize = 0x802E_A4C4;
const FN_802ED2F4: usize = 0x802E_D2F4;
const FN_802E8F7C: usize = 0x802E_8F7C;
const FN_802E9624: usize = 0x802E_9624;
const FN_802EC1A0: usize = 0x802E_C1A0;
/// `0x802EC9CC` - the exception prologue. Wave 113 translated its C-level part
/// as [`fn_802ec9cc`], so the eight-argument forward in [`fn_802e9624`] calls
/// it directly; kept for reference.
///
/// # Original
/// - Ghidra: `FUN_802ec9cc` at `0x802ec9cc` (744 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec9cc_FUN_802ec9cc.c`
const FN_802EC9CC: usize = 0x802E_C9CC;
const FN_801AC560: usize = 0x801A_C560;
const FN_801AC470: usize = 0x801A_C470;
const FN_8015F730: usize = 0x8015_F730;
const FN_801B3CC0: usize = 0x801B_3CC0;
const FN_801B4BB0: usize = 0x801B_4BB0;

/// Calls a PowerPC function with no arguments but captures the `r3`/`eax`
/// return word, which [`platform::call0`] discards.
///
/// `crate::platform` stops at [`platform::call7`] and provides `call0` with
/// no return slot, so the wave-36 init / poll wrappers get this local
/// adapter. The signature pattern matches `engine::frame::call0_ret`.
///
/// # Safety
/// `addr` must point at a PowerPC function returning a word in `r3`.
#[inline]
unsafe fn call0_ret(addr: usize) -> u32 {
    let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
    f()
}

/// Calls a PowerPC function with eight integer arguments.
///
/// `crate::platform` stops at [`platform::call7`]; the eight-word ABI shape is
/// kept here for reference even though Wave 113 closed the only call site (the
/// `0x802EC9CC` forward in [`fn_802e9624`]) with a direct call.
///
/// # Safety
/// `addr` must point at a PowerPC function taking eight words in `r3`..`r9`
/// plus stack slots (PowerPC EABI). Argument order is the Ghidra call order.
#[inline]
#[allow(dead_code)]
unsafe fn call8_u32(
    addr: usize,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    let func: extern "C" fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32 =
        core::mem::transmute(addr);
    func(a, b, c, d, e, f, g, h)
}

/// Debug-boot entry, external interrupts **off**: saves the CPU state, selects
/// the debugger hardware and hands over to the debugger's main loop.
///
/// In plain English: "a debugger boot was requested; freeze the registers where
/// they are, tell the debugger code which devkit this is, then let it talk to
/// the host - the game never starts."
///
/// The `hardware_id` arrives in `r5`, not `r3` (`0x802E9370 or r3,r5,r5`); the
/// DOL entry passes 0 for boot type 2 and 1 for boot type 3.
///
/// # Original
/// - Ghidra: `FUN_802e9300` at `0x802e9300` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9300_FUN_802e9300.c`
/// - Call site: `0x800040E4..0x800040F0` in `80004050_entry.c`
///   (`mtspr LR,0x802E9300` + `bclr`, i.e. an indirect call)
/// - Instruction check: `work/dol/main.elf`, `0x802E9300..0x802E9394`
///
/// # Arguments
/// * `hardware_id` - `r5`: 0 or 1 from the boot-info boot type. 1 = NDEV,
///   2 = GDEV, anything else is rejected by `0x802E96E4`.
///
/// # Returns
/// Never, in practice: the success path tail-calls the debugger loop. The
/// `rc == 1` path returns to the saved resume address instead.
///
/// # Side effects
/// * writes `r0..r31`, LR and CR to `0x80427240`;
/// * writes the whole SPR bank to `0x804273E8`;
/// * clears `MSR.EE`, sets `SRR1` to the entry MSR, disables the L2 cache
///   (`L2CR = 0`) and the data breakpoint (`spr1013 = 0`);
/// * replaces the stack pointer with `0x80441C40`;
/// * programs the debugger's hardware handlers and then runs the debugger.
///
/// # Safety
/// Destroys the caller's machine state; only valid as a boot path.
///
/// # Confidence
/// high for the control flow and the MetroTRK identity (strings, handler table,
/// tail call); medium for the hardware-id numbering. Marked `partial`: the
/// register save/restore, the `MSR`/`SRR1` writes and the `lmw` resume live in
/// [`trk_boot`] as documented machine state that cannot be expressed in Rust, so
/// the body is the call sequence with those steps elided.
pub unsafe fn fn_802e9300(hardware_id: u32) {
    // canonical: trk_boot holds the shared 0x802E9300/0x802E9398 body.
    trk_boot(hardware_id, /* keep_interrupts_enabled = */ false);
}

/// Debug-boot entry, external interrupts **on**, hardware id fixed to GDEV (2).
///
/// In plain English: same as [`fn_802e9300`], but the machine is left with
/// interrupts enabled and the devkit is assumed to be the GDEV revision.
///
/// The DOL entry reaches this only when `0x80004040()` returns 1, i.e. when
/// boot type 4 previously ran `0x80004030` and set the flag.
///
/// # Original
/// - Ghidra: `FUN_802e9398` at `0x802e9398` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9398_FUN_802e9398.c`
/// - Call site: `0x800041A0..0x800041A4` in `80004050_entry.c`
/// - Differences from `0x802E9300`: `0x802E93D4` has no `xori` (MSR.EE stays
///   set) and `0x802E9404 li r3,2`
///
/// # Arguments
/// None. The hardware id is the constant 2.
///
/// # Returns
/// Never, in practice - same shape as [`fn_802e9300`].
///
/// # Side effects
/// Same as [`fn_802e9300`], except that `MSR.EE` is left enabled.
///
/// # Safety
/// Destroys the caller's machine state; only valid as a boot path.
///
/// # Confidence
/// high - the body is `0x802E9300` with two instructions changed (`mtmsr` keeps
/// `MSR.EE`, hardware id fixed to 2). Marked `partial` for the same documented
/// machine-state gap as [`fn_802e9300`]: the register save/restore, `MSR`/`SRR1`
/// writes and `lmw` resume in [`trk_boot`] are listed, not executed.
pub unsafe fn fn_802e9398() {
    // canonical: trk_boot holds the shared 0x802E9300/0x802E9398 body.
    trk_boot(HW_ID_GDEV, /* keep_interrupts_enabled = */ true);
}

/// Shared body of [`fn_802e9300`] and [`fn_802e9398`].
///
/// Only two things differ between the originals, and both are passed in:
/// `keep_interrupts_enabled` selects `mtmsr (MSR | 0x8000)` (`0x802E93D4`)
/// instead of `mtmsr ((MSR | 0x8000) ^ 0x8000)` (`0x802E933C`), and
/// `hardware_id` selects `li r3,2` (`0x802E9404`) instead of `or r3,r5,r5`
/// (`0x802E9370`).
///
/// # Original
/// - Ghidra: `FUN_802e9300` at `0x802e9300` (148 bytes) and `FUN_802e9398` at
///   `0x802e9398` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9300_FUN_802e9300.c`,
///   `work/ghidra/main-gb/export/802e9398_FUN_802e9398.c`
///
/// # Arguments
/// * `hardware_id` - value for the `0x802E96E4` query.
/// * `keep_interrupts_enabled` - whether the MSR write leaves `MSR.EE` set.
///
/// # Returns
/// Nothing; the normal path does not come back.
///
/// # Side effects
/// See [`fn_802e9300`]. The translated part calls `0x802EA4F8` (SPR dump),
/// `0x802E96E4` (hardware selection) and, on success, `0x802E8E54` (debugger).
///
/// # Safety
/// Takes over the machine; only valid as a boot path.
///
/// # Confidence
/// high for the untranslated register steps (each is listed with its address)
/// and for the branch; the `rc == 1` path is a known oddity - see below.
unsafe fn trk_boot(hardware_id: u32, keep_interrupts_enabled: bool) {
    // `keep_interrupts_enabled` is only observable in the `mtmsr` below, which
    // cannot be expressed in Rust; keep the parameter meaningful for readers.
    let _ = keep_interrupts_enabled;

    // partial: 0x802E9300..0x802E9348 (0x802E9398..0x802E93E0) is machine state.
    // Written out so the untranslated part is auditable:
    //   addi  r1,r1,-4 / stw r3,0(r1)   stash the arg while r3 is reused
    //   lis/ori r3,0x80427240           &TRK_GPR_CONTEXT
    //   stmw  r0,0(r3)                  save r0..r31
    //   lwz   r4,0(r1) / addi r1,r1,4
    //   stw   r1,4(r3)                  context.GPR[1] = stack pointer
    //   stw   r4,12(r3)                 context.GPR[3] = original r3
    //   mfspr r4,LR / stw r4,0x80(r3) / stw r4,0x84(r3)   (LR and resume PC)
    //   mfcr  r4    / stw r4,0x88(r3)                     (CR)
    //   mfmsr r4
    //   ori   r3,r4,0x8000            set MSR.EE
    //   [xori r3,r3,0x8000]           clear MSR.EE -- only in 0x802E9300
    //   mtmsr r3
    //   mtspr SRR1,r4                 SRR1 = the MSR we entered with
    // Missing: all of the above (registers, CR, MSR, SRR1). Not representable
    // without inline assembly.

    // observed 0x802E934C / 0x802E93E0: `bl 0x802EA4F8` - dump the SPRs.
    fn_802ea4f8();

    // partial: 0x802E9358..0x802E936C restores r0..r31 with `lmw`, zeroes
    // L2CR and spr1013 (L2 cache off, data breakpoint off) and sets
    // r1 = 0x80441C40. Missing: the register/SP replacement.

    // observed 0x802E9370 / 0x802E9404: r3 = hardware id, then `bl 0x802E96E4`.
    let rc = fn_802e96e4(hardware_id);

    // observed 0x802E9378..0x802E937C (0x802E940C..0x802E9410):
    //   cmpi cr0,r3,1 / bne -> debugger loop.
    if rc == 1 {
        // partial: 0x802E9380..0x802E938C / 0x802E9414..0x802E9420:
        //   lwz r4,0x84(r3) / mtspr LR,r4 / lmw r0,0(r3) / blr
        // i.e. "resume at the saved PC with the saved registers". The catch:
        // r3 is the *return value* 1 here, not a context pointer, so the load
        // would fault. Only an invalid hardware id (boot type 2 passes 0) can
        // reach it.
        // unknown: dormant/broken path in the shipped binary. No observable
        // behaviour to reconstruct.
        return;
    }

    // observed 0x802E9390 / 0x802E9424: `b 0x802E8E54` (tail call).
    platform::call0(TRK_MAIN);
}

// ---------------------------------------------------------------------------
// Wave 36 - MetroTRK core bodies (`0x802E8E54..0x802E9E84`).
// ---------------------------------------------------------------------------

/// Debugger main wrapper: run the init sequence, and on success print the
/// banner and run the dispatch loop.
///
/// In plain English: "initialise the debugger; if initialisation succeeded,
/// announce it and enter the message loop; either way record the final code
/// in the TRK result word."
///
/// # Original
/// - Ghidra: `FUN_802e8e54` at `0x802e8e54` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8e54_FUN_802e8e54.c`
///
/// # Arguments
/// None. The callee addresses and the result word are globals.
///
/// # Returns
/// Nothing. The init code is stored to [`G_TRK_INIT_RESULT`].
///
/// # Side effects
/// * writes [`G_TRK_INIT_RESULT`] twice (init code, then teardown code);
/// * runs the whole init / dispatch / teardown sequence via absolute calls.
///
/// # Safety
/// Only valid in the debugger-boot context; must not run during normal play.
///
/// # Confidence
/// high - the store / test / two-call sequence is literal in the export.
pub unsafe fn fn_802e8e54() {
    let rc = call0_ret(FN_802E9BEC);
    platform::write_i32(G_TRK_INIT_RESULT, rc as i32);
    if rc == 0 {
        platform::call0(FN_802E9CBC);
        platform::call0(FN_802E8E90);
    }
    let rc2 = call0_ret(FN_802E9C98);
    platform::write_u32(G_TRK_INIT_RESULT, rc2);
}

/// Debugger dispatch loop: dequeue events until a type-1 event ends the loop.
///
/// In plain English: "pull events from the queue forever; when nothing is
/// queued, either idle (poll `0x802E9E08`) or, if the previous iteration
/// idled and the context byte is zero, try the idle service `0x802ED4E8`;
/// when an event is queued, route it by type (3/4 to `0x802ECDCC`, 2 to the
/// message dispatcher `0x802E91E0`, 1 exits, 5 to `0x802ED2F4`) and then
/// release it via `0x802E9BE4`."
///
/// # Original
/// - Ghidra: `FUN_802e8e90` at `0x802e8e90` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8e90_FUN_802e8e90.c`
///
/// # Arguments
/// None. Events come from the queue at [`EVENT_SLOTS`].
///
/// # Returns
/// Nothing. Runs until a type-1 event arrives.
///
/// # Side effects
/// Consumes the event queue and runs the per-type handlers by absolute
/// address.
///
/// # Safety
/// Must run inside the MetroTRK debugger context with the queue initialised.
///
/// # Confidence
/// low - the queue / branch skeleton is literal, but the export leaves
/// `local_10` uninitialised for `0x802EA940(local_10)` and calls
/// `0x802E91E0` with no explicit argument (the register still holds the
/// buffer result on PowerPC). Both are modelled as written; the role of the
/// `**(char**)` context poll is `// inferred:` from the double dereference.
pub unsafe fn fn_802e8e90() {
    let mut done = false;
    let mut idled = false;
    // observed: `int local_18[2]` scratch event (8 bytes) on the stack.
    let mut event = [0_u32; 2];
    // unknown: the export passes an uninitialised `local_10` to 0x802EA940;
    // modelled as a zero word so the call sequence is literal.
    let local_10: u32 = 0;
    while !done {
        let dequeued = platform::call1(FN_802E9A74, event.as_mut_ptr() as u32);
        if dequeued == 0 {
            if idled {
                let ptr = platform::read_u32(G_TRK_CONTEXT_PTR) as usize;
                if platform::read_u8(ptr) == 0 {
                    let rc = call0_ret(FN_802ED4E8);
                    if rc == 0 {
                        platform::call0(FN_802EA4C4);
                    }
                    idled = false;
                    continue;
                }
            }
            idled = true;
            platform::call0(FN_802E9E08);
        } else {
            idled = false;
            let ev_type = event[0];
            // observed: `(local_18[0] - 3U) < 2`, unsigned.
            if ev_type.wrapping_sub(3) < 2 {
                let _ = platform::call1(FN_802ECDCC, event.as_mut_ptr() as u32);
            } else if ev_type == 2 {
                let _ = platform::call1(FN_802EA940, local_10);
                // observed: `FUN_802e91e0()` with no explicit argument.
                platform::call0(FN_802E91E0);
            } else if ev_type == 1 {
                done = true;
            } else if ev_type == 5 {
                platform::call0(FN_802ED2F4);
            }
            let _ = platform::call1(FN_802E9BE4, event.as_mut_ptr() as u32);
        }
    }
}

/// Map an address through the TRK window: pass through inside the
/// `GAR` window, otherwise tag it `0x8` / `0x9` depending on the range.
///
/// In plain English: "if the address is inside the current `0x4000`-byte
/// window at [`G_TRK_ADDR_BASE`] and the `0x80427478` flag has its low bits
/// set, return it unchanged; otherwise return it with the high nibble forced
/// to `0x8` for low addresses and `0x9` for mid addresses, or unchanged for
/// high addresses."
///
/// # Original
/// - Ghidra: `FUN_802e9430` at `0x802e9430` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9430_FUN_802e9430.c`
///
/// # Arguments
/// * `addr` - the address to map (`param_1`).
///
/// # Returns
/// The mapped address (see side effects for the window rule).
///
/// # Side effects
/// None. Reads [`G_TRK_ADDR_BASE`] and [`G_FLAG_80427478`].
///
/// # Safety
/// Caller must pass a target address; no pointers are dereferenced.
///
/// # Confidence
/// medium - the range cascade is literal; the window base is `r13 - 0x4658`
/// resolved via `docs/GLOSSARY.md` and the flag's role is `// inferred:`.
pub unsafe fn fn_802e9430(addr: u32) -> u32 {
    let base = platform::read_u32(G_TRK_ADDR_BASE);
    if base <= addr
        && addr < base.wrapping_add(0x4000)
        && (platform::read_u32(G_FLAG_80427478) & 3) != 0
    {
        return addr;
    }
    if addr < 0x3000000 {
        return addr & 0x3fff_ffff | 0x8000_0000;
    }
    if addr < 0x1000_0000 {
        return addr;
    }
    if 0x1bff_ffff < addr {
        return addr;
    }
    addr & 0x3fff_ffff | 0x9000_0000
}

/// Seed the TRK state: mark initialised and reset the address base.
///
/// In plain English: "set the initialised byte, run `0x802EC1A0`, reset the
/// address-base word to `0xE0000000` and return 0."
///
/// # Original
/// - Ghidra: `FUN_802e95cc` at `0x802e95cc` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e95cc_FUN_802e95cc.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * writes `1` to [`G_80427230`];
/// * writes the `MSR` image read by [`fn_802ec1a0`] to [`G_80427224`];
/// * writes `0xE0000000` to [`G_TRK_ADDR_BASE`].
///
/// # Safety
/// Must run in the debugger context; writes to BSS/SDA globals.
///
/// # Confidence
/// high - the three stores and the `bl 0x802EC1A0` at `0x802E95EC` are literal;
/// Wave 106 translated the callee, so the stored value is the machine-state
/// register rather than an unknown return register.
pub unsafe fn fn_802e95cc() -> u32 {
    platform::write_u32(G_80427230, 1);
    let reg = fn_802ec1a0();
    platform::write_u32(G_80427224, reg);
    platform::write_u32(G_TRK_ADDR_BASE, 0xe000_0000);
    0
}

/// Trampoline: forward `(0, 0, 0)` to `0x801B3CC0`.
///
/// In plain English: "call `0x801B3CC0(0, 0, 0)` and return."
///
/// # Original
/// - Ghidra: `FUN_802e9614` at `0x802e9614` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9614_FUN_802e9614.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One absolute call to `0x801B3CC0`.
///
/// # Safety
/// Caller must be in the debugger context; the callee's own requirements apply.
///
/// # Confidence
/// high - the three-zero-arg statement call is literal.
pub unsafe fn fn_802e9614() {
    let _ = platform::call3(FN_801B3CC0, 0, 0, 0);
}

/// Forward an object's saved arguments to [`fn_802ec9cc`], or reload them from
/// the object when the flag bit is set.
///
/// In plain English: "read the halfword flags at `object + 0x1A2`; if bit 1
/// is set, clear it and snapshot the five words at `object + 0x14..0x28`
/// over the caller-supplied tail; then call the exception prologue with
/// (`arg_a`, `object + 0x19C`, first, second, tail) where first/second are
/// either the flag word / `flags & 2` (flag clear) or the two snapshots
/// (flag set)."
///
/// # Original
/// - Ghidra: `FUN_802e9624` at `0x802e9624` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9624_FUN_802e9624.c`
///
/// # Arguments
/// * `object` - pointer to the TRK connection object (`param_1`).
/// * `arg_a` - forwarded as the first argument to [`fn_802ec9cc`].
/// * `_arg_b`, `_arg_c` - `param_3` / `param_4`; `// unknown:` the export
///   never forwards them (only the snapshot values `v1` / `v2` are passed).
/// * `arg_d..arg_g` - `param_5..param_8`; forwarded unless the flag branch
///   overwrites them with the object snapshots.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * clears `flags & ~2` at `object + 0x1A2` when the flag was set;
/// * calls [`fn_802ec9cc`] with eight arguments.
///
/// # Safety
/// `object` and `object + 0x1A2` must be readable/writable for `0x2C` bytes.
///
/// # Confidence
/// medium - the flag test, snapshot offsets and eight-argument forward are
/// literal; the object's field roles are `// inferred:` from the offsets.
pub unsafe fn fn_802e9624(
    object: usize,
    arg_a: u32,
    _arg_b: u32,
    _arg_c: u32,
    mut arg_d: u32,
    mut arg_e: u32,
    mut arg_f: u32,
    mut arg_g: u32,
) {
    let flags = platform::read_u16(object + 0x1A2);
    let mut v1 = flags as u32;
    let mut v2 = (flags & 2) as u32;
    if (flags & 2) != 0 {
        platform::write_u16(object + 0x1A2, flags & 0xfffd);
        v1 = platform::read_u32(object + 0x14);
        v2 = platform::read_u32(object + 0x18);
        arg_d = platform::read_u32(object + 0x1C);
        arg_e = platform::read_u32(object + 0x20);
        arg_f = platform::read_u32(object + 0x24);
        arg_g = platform::read_u32(object + 0x28);
    }
    let _ = fn_802ec9cc(
        arg_a,
        platform::read_u32(object + 0x19C),
        v1,
        v2,
        arg_d,
        arg_e,
        arg_f,
        arg_g,
    );
}

/// Connection callback: service `0x801B4BB0()`, then forward to
/// [`fn_802e9624`] with `0x500`.
///
/// In plain English: "run the low-level service once, then run the
/// saved-argument forward on the second argument."
///
/// # Original
/// - Ghidra: `FUN_802e96ac` at `0x802e96ac` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e96ac_FUN_802e96ac.c`
///
/// # Arguments
/// * `_unused` - `param_1`; ignored by the body.
/// * `connection` - `param_2`, forwarded as the object to [`fn_802e9624`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801B4BB0()` then `0x802E9624(connection, 0x500)`.
///
/// # Safety
/// `connection` must be a valid TRK object for [`fn_802e9624`].
///
/// # Confidence
/// medium - the two calls are literal; only `r3`/`r4` are set for the
/// eight-argument callee (the remaining registers carry over on PowerPC, as
/// modelled by [`platform::call2`]), and `param_1`'s role is `// unknown:`.
pub unsafe fn fn_802e96ac(_unused: u32, connection: usize) {
    platform::call0(FN_801B4BB0);
    let _ = platform::call2(FN_802E9624, connection as u32, 0x500);
}

/// Install the debugger handlers: invoke the two function pointers installed
/// at `0x804256D0` / `0x804256E8` and return 0.
///
/// In plain English: "call the first installed handler with this context
/// and the address of [`fn_802e96ac`], then call the second installed
/// handler with no arguments."
///
/// # Original
/// - Ghidra: `FUN_802e9820` at `0x802e9820` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9820_FUN_802e9820.c`
///
/// # Arguments
/// * `_unused0`, `_unused1` - `param_1` / `param_2`; ignored by the body.
/// * `context` - `param_3`, forwarded to the first handler.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Two indirect calls through the handler-table slots.
///
/// # Safety
/// The slots must hold valid function pointers (installed by `0x802E96E4`).
///
/// # Confidence
/// medium - the indirect sequence and the `fn_802e96ac` address passed as
/// the second argument are literal; the handler roles are `// inferred:`.
pub unsafe fn fn_802e9820(_unused0: u32, _unused1: u32, context: usize) -> u32 {
    let first = platform::read_u32(HANDLER_SLOT_804256D0) as usize;
    let _ = platform::call2(first, context as u32, 0x802E_96AC);
    let second = platform::read_u32(HANDLER_SLOT_804256E8) as usize;
    platform::call0(second);
    0
}

/// Guarded indirect call: run the handler at `0x804256D4` unless disabled.
///
/// In plain English: "if the TRK guard byte is non-zero, or no handler is
/// installed, return; otherwise call the installed handler."
///
/// # Original
/// - Ghidra: `FUN_802e9874` at `0x802e9874` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9874_FUN_802e9874.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call through [`HANDLER_SLOT_804256D4`] when armed.
///
/// # Safety
/// Guard byte must be mapped; the slot must hold a valid pointer or null.
///
/// # Confidence
/// medium - the two guards and the indirect call are literal; Ghidra warns
/// "Could not recover jumptable" here, and the guard's role is `// inferred:`.
pub unsafe fn fn_802e9874() {
    if platform::read_u8(G_TRK_GUARD_BYTE) != 0 {
        return;
    }
    let handler = platform::read_u32(HANDLER_SLOT_804256D4) as usize;
    if handler == 0 {
        return;
    }
    platform::call0(handler);
}

/// Unconditional indirect call through the handler slot at `0x804256DC`.
///
/// In plain English: "call whatever handler is installed in the slot."
///
/// # Original
/// - Ghidra: `FUN_802e98a0` at `0x802e98a0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e98a0_FUN_802e98a0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call through [`HANDLER_SLOT_804256DC`].
///
/// # Safety
/// The slot must hold a valid function pointer.
///
/// # Confidence
/// medium - the single indirect call is literal; the callee identity is
/// `// inferred:` (Ghidra warns "Could not recover jumptable").
pub unsafe fn fn_802e98a0() {
    let handler = platform::read_u32(HANDLER_SLOT_804256DC) as usize;
    platform::call0(handler);
}

/// Signed poll through `0x804256E0`: return `0` when the handler returns
/// zero, `-1` otherwise.
///
/// In plain English: "call the installed handler and report success as 0
/// and failure as -1."
///
/// # Original
/// - Ghidra: `FUN_802e98b4` at `0x802e98b4` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e98b4_FUN_802e98b4.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` if the handler returned zero, `-1` otherwise.
///
/// # Side effects
/// One indirect call through [`HANDLER_SLOT_804256E0`].
///
/// # Safety
/// The slot must hold a valid function pointer.
///
/// # Confidence
/// medium - the `(int)(-v | v) >> 31` sequence is reproduced literally
/// (it is `0`/`-1`); the callee identity is `// inferred:`.
pub unsafe fn fn_802e98b4() -> i32 {
    let handler = platform::read_u32(HANDLER_SLOT_804256E0) as usize;
    let v = call0_ret(handler) as i32;
    // observed: `(int)(-uVar1 | uVar1) >> 0x1f` in the export.
    (v.wrapping_neg() | v) >> 31
}

/// Signed poll through `0x804256E4`: return `0` when the handler returns
/// zero, `-1` otherwise.
///
/// In plain English: same as [`fn_802e98b4`], through the next slot.
///
/// # Original
/// - Ghidra: `FUN_802e98f0` at `0x802e98f0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e98f0_FUN_802e98f0.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// `0` if the handler returned zero, `-1` otherwise.
///
/// # Side effects
/// One indirect call through [`HANDLER_SLOT_804256E4`].
///
/// # Safety
/// The slot must hold a valid function pointer.
///
/// # Confidence
/// medium - the `(int)(-v | v) >> 31` sequence is reproduced literally;
/// the callee identity is `// inferred:`.
pub unsafe fn fn_802e98f0() -> i32 {
    let handler = platform::read_u32(HANDLER_SLOT_804256E4) as usize;
    let v = call0_ret(handler) as i32;
    // observed: `(int)(-uVar1 | uVar1) >> 0x1f` in the export.
    (v.wrapping_neg() | v) >> 31
}

/// Unconditional indirect call through the handler slot at `0x804256F4`.
///
/// In plain English: "call whatever handler is installed in the slot."
///
/// # Original
/// - Ghidra: `FUN_802e992c` at `0x802e992c` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e992c_FUN_802e992c.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call through [`HANDLER_SLOT_804256F4`].
///
/// # Safety
/// The slot must hold a valid function pointer.
///
/// # Confidence
/// medium - the single indirect call is literal; the callee identity is
/// `// inferred:` (Ghidra warns "Could not recover jumptable").
pub unsafe fn fn_802e992c() {
    let handler = platform::read_u32(HANDLER_SLOT_804256F4) as usize;
    platform::call0(handler);
}

/// Unconditional indirect call through the handler slot at `0x804256F0`.
///
/// In plain English: "call whatever handler is installed in the slot."
///
/// # Original
/// - Ghidra: `FUN_802e9940` at `0x802e9940` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9940_FUN_802e9940.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call through [`HANDLER_SLOT_804256F0`].
///
/// # Safety
/// The slot must hold a valid function pointer.
///
/// # Confidence
/// medium - the single indirect call is literal; the callee identity is
/// `// inferred:` (Ghidra warns "Could not recover jumptable").
pub unsafe fn fn_802e9940() {
    let handler = platform::read_u32(HANDLER_SLOT_804256F0) as usize;
    platform::call0(handler);
}

/// Print helper: call the two-argument print `0x8015F730` with the fixed
/// format at `0x803514D5` and one value.
///
/// In plain English: "print the value through the shared format string."
///
/// # Original
/// - Ghidra: `FUN_802e9954` at `0x802e9954` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9954_FUN_802e9954.c`
///
/// # Arguments
/// * `value` - `param_1`; the second print argument (a string pointer when
///   called from [`fn_802e9cbc`]).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One absolute call to `0x8015F730` (the printf-style helper).
///
/// # Safety
/// `value` must match the format at [`FMT_803514D5`]; callers pass a string
/// pointer or a number accordingly.
///
/// # Confidence
/// high - the `FUN_8015f730(&DAT_803514d5, param_1)` sequence is literal;
/// `// inferred:` the helper is printf-style.
pub unsafe fn fn_802e9954(value: u32) {
    let _ = platform::call2(FN_8015F730, FMT_803514D5 as u32, value);
}

/// Copy the four-byte word at `r2 - 0x34C0` to `0x801A9BA4` and service the
/// two cache/`OS` helpers on it.
///
/// In plain English: "copy one word from the SDA source to the fixed buffer,
/// then run `0x801AC560` and `0x801AC470` over the four bytes."
///
/// # Original
/// - Ghidra: `FUN_802e996c` at `0x802e996c` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e996c_FUN_802e996c.c`
///
/// # Arguments
/// None. The addresses are immediates / SDA-derived.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * writes four bytes at `0x801A9BA4`;
/// * calls `0x801AC560` and `0x801AC470` on that buffer.
///
/// # Safety
/// Both the source (`0x8042FBC0`) and the buffer must be mapped.
///
/// # Confidence
/// medium - the copy length, destination and two service calls are literal;
/// the `r2` base comes from `docs/GLOSSARY.md` and the service roles are
/// `// inferred:`.
pub unsafe fn fn_802e996c() {
    let _ = platform::call3(FN_802E8F7C, 0x801A_9BA4, G_R2_SRC_34C0 as u32, 4);
    let _ = platform::call2(FN_801AC560, 0x801A_9BA4, 4);
    let _ = platform::call2(FN_801AC470, 0x801A_9BA4, 4);
}

/// Reset the event queue: empty it and restart the sequence counter.
///
/// In plain English: "set the queue count and head to zero and the
/// sequence to `0x100`."
///
/// # Original
/// - Ghidra: `FUN_802e9a50` at `0x802e9a50` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9a50_FUN_802e9a50.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Writes [`EVENT_COUNT`], [`EVENT_HEAD`] and [`EVENT_SEQ`].
///
/// # Safety
/// Writes to the queue globals; caller must be in the debugger context.
///
/// # Confidence
/// high - the three stores (`0`, `0`, `0x100`) are literal.
pub unsafe fn fn_802e9a50() -> u32 {
    platform::write_u32(EVENT_COUNT, 0);
    platform::write_u32(EVENT_HEAD, 0);
    platform::write_u32(EVENT_SEQ, 0x100);
    0
}

/// Dequeue one event: copy the head slot to `dst` and advance the ring.
///
/// In plain English: "if the queue is non-empty, copy the twelve bytes at
/// the head slot to `dst`, advance the head (wrapping `2 -> 0`) and
/// decrement the count; return 1, else return 0."
///
/// # Original
/// - Ghidra: `FUN_802e9a74` at `0x802e9a74` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9a74_FUN_802e9a74.c`
///
/// # Arguments
/// * `dst` - 12-byte destination buffer (`param_1`).
///
/// # Returns
/// `1` if an event was copied, `0` if the queue was empty.
///
/// # Side effects
/// * copies one slot from [`EVENT_SLOTS`];
/// * updates [`EVENT_HEAD`] / [`EVENT_COUNT`].
///
/// # Safety
/// `dst..dst+12` must be writable; the queue globals must be mapped.
///
/// # Confidence
/// high - the guard, the 12-byte copy, the head/count update order and the
/// `2 -> 0` wrap are literal.
pub unsafe fn fn_802e9a74(dst: usize) -> u32 {
    if platform::read_i32(EVENT_COUNT) > 0 {
        let head = platform::read_u32(EVENT_HEAD);
        let slot = (EVENT_SLOTS as u32).wrapping_add(head.wrapping_mul(0xC));
        let _ = platform::call3(FN_802E8F7C, dst as u32, slot, 0xC);
        let head_next = head.wrapping_add(1);
        platform::write_u32(EVENT_HEAD, head_next);
        let count = platform::read_i32(EVENT_COUNT);
        platform::write_i32(EVENT_COUNT, count.wrapping_sub(1));
        if head_next == 2 {
            platform::write_u32(EVENT_HEAD, 0);
        }
        1
    } else {
        0
    }
}

/// Enqueue one event at `(head + count) % 2`, stamped with the next
/// sequence number.
///
/// In plain English: "if the queue already holds two events, print the
/// 'Event Queue full' message and return `0x100`; otherwise copy the
/// twelve-byte event into the next ring slot, store the current sequence
/// in the slot's second word, bump the sequence (wrapping `0x100` after
/// overflow) and the count, and return 0."
///
/// # Original
/// - Ghidra: `FUN_802e9b00` at `0x802e9b00` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9b00_FUN_802e9b00.c`
///
/// # Arguments
/// * `event` - 12-byte event to enqueue (`param_1`).
///
/// # Returns
/// `0` on success, `0x100` when the queue is full.
///
/// # Side effects
/// * copies one slot in [`EVENT_SLOTS`];
/// * writes the slot sequence word (`slot + 4`);
/// * updates [`EVENT_COUNT`] / [`EVENT_SEQ`];
/// * prints via `0x8015F730` when full.
///
/// # Safety
/// `event..event+12` must be readable; the queue globals must be mapped.
///
/// # Confidence
/// high - the full / ring / sequence logic is literal. The export's
/// `((head + count) >> 31 ...)` slot expression is `0` in this domain
/// (counts are `0..2`), so the slot is `(head + count) & 1`, reproduced
/// literally below.
pub unsafe fn fn_802e9b00(event: usize) -> u32 {
    if platform::read_i32(EVENT_COUNT) == 2 {
        let _ = platform::call1(FN_8015F730, STR_QUEUE_FULL_803514E0 as u32);
        0x100
    } else {
        let head = platform::read_i32(EVENT_HEAD);
        let count = platform::read_i32(EVENT_COUNT);
        // observed: `iVar1 = (head + count) >> 0x1f` then
        // `((head + count & 1 ^ -iVar1) + iVar1) * 0xc`.
        let sum = head.wrapping_add(count);
        let shift = sum >> 31;
        let slot_idx = ((sum & 1) ^ (-shift)).wrapping_add(shift);
        let slot_addr = (EVENT_SLOTS as i32).wrapping_add(slot_idx.wrapping_mul(0xC));
        let _ = platform::call3(FN_802E8F7C, slot_addr as u32, event as u32, 0xC);
        platform::write_u32((slot_addr + 4) as usize, platform::read_u32(EVENT_SEQ));
        let seq_next = platform::read_u32(EVENT_SEQ).wrapping_add(1);
        platform::write_u32(EVENT_SEQ, seq_next);
        if (seq_next as i32) < 0x100 {
            platform::write_u32(EVENT_SEQ, 0x100);
        }
        let count_next = platform::read_i32(EVENT_COUNT).wrapping_add(1);
        platform::write_i32(EVENT_COUNT, count_next);
        0
    }
}

/// Fill a twelve-byte event triple: `(value, 0, -1)`.
///
/// In plain English: "store `value` in the first word, zero the second and
/// `0xFFFFFFFF` in the third."
///
/// # Original
/// - Ghidra: `FUN_802e9bcc` at `0x802e9bcc` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9bcc_FUN_802e9bcc.c`
///
/// # Arguments
/// * `dst` - 12-byte task buffer (`param_1`).
/// * `value` - first word (`param_2`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes three words at `dst`.
///
/// # Safety
/// `dst..dst+12` must be writable.
///
/// # Confidence
/// high - the three stores are literal.
pub unsafe fn fn_802e9bcc(dst: usize, value: u32) {
    platform::write_u32(dst, value);
    platform::write_u32(dst + 4, 0);
    platform::write_u32(dst + 8, 0xffff_ffff);
}

/// Debugger init sequence: run the steps in order and return the first
/// non-zero code, or 0.
///
/// In plain English: "call `0x802E9CC8`, then `0x802E9A50`, then
/// `0x802EA884`, then unconditionally `0x802E996C`, then `0x802E9E74`, then
/// `0x802E95CC`, each only if the previous returned 0; finally, if all were
/// 0, install handlers via `0x802E9820(1, 0, 0x8042B040)`, forward the
/// context word to `0x802ED91C`, and propagate any error."
///
/// # Original
/// - Ghidra: `FUN_802e9bec` at `0x802e9bec` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9bec_FUN_802e9bec.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The first non-zero init code, or 0.
///
/// # Side effects
/// Runs the seven calls above in order; the `0x802E996C` step runs even
/// when the previous step failed.
///
/// # Safety
/// Must run during debugger start-up before the dispatch loop.
///
/// # Confidence
/// high - the unconditionally-called middle step and the error-propagation
/// tail are literal; the context address is `r13 - 0x4640` resolved via
/// `docs/GLOSSARY.md`.
pub unsafe fn fn_802e9bec() -> u32 {
    let mut rc = call0_ret(FN_802E9CC8);
    if rc == 0 {
        rc = call0_ret(FN_802E9A50);
    }
    if rc == 0 {
        rc = call0_ret(FN_802EA884);
    }
    platform::call0(FN_802E996C);
    if rc == 0 {
        rc = call0_ret(FN_802E9E74);
    }
    if rc == 0 {
        rc = call0_ret(FN_802E95CC);
    }
    if rc == 0 {
        let rc2 = platform::call3(FN_802E9820, 1, 0, G_TRK_CONTEXT_PTR as u32);
        let ctx_word = platform::read_u32(G_TRK_CONTEXT_PTR);
        let _ = platform::call1(FN_802ED91C, ctx_word);
        if rc2 != 0 {
            rc = rc2;
        }
    }
    rc
}

/// Teardown wrapper: run `0x802E9E7C` and return 0.
///
/// In plain English: "call the teardown stub and return 0."
///
/// # Original
/// - Ghidra: `FUN_802e9c98` at `0x802e9c98` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9c98_FUN_802e9c98.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// One absolute call to `0x802E9E7C`.
///
/// # Safety
/// Must run at debugger shutdown.
///
/// # Confidence
/// high - the single call and the constant return are literal.
pub unsafe fn fn_802e9c98() -> u32 {
    platform::call0(FN_802E9E7C);
    0
}

/// Print the debugger banner "MetroTRK for Revolution v0.4" via
/// [`fn_802e9954`].
///
/// In plain English: "print the banner string."
///
/// # Original
/// - Ghidra: `FUN_802e9cbc` at `0x802e9cbc` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9cbc_FUN_802e9cbc.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One call to `0x802E9954` with the string at `0x80351500`.
///
/// # Safety
/// The banner string must be mapped (it is in `.rodata`).
///
/// # Confidence
/// high - the single string call is literal.
pub unsafe fn fn_802e9cbc() {
    let _ = platform::call1(FN_802E9954, STR_BANNER_80351500 as u32);
}

/// Idle poll: read one reply id and, when it is not `-1`, post the
/// type-2 task for it.
///
/// In plain English: "if `0x802E9D3C` returns something other than `-1`,
/// build the type-2 queue task for that id."
///
/// # Original
/// - Ghidra: `FUN_802e9e08` at `0x802e9e08` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9e08_FUN_802e9e08.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x802E9D3C`, then `0x802E9E34(id)` unless the id is `-1`.
///
/// # Safety
/// Must run with the reply path initialised.
///
/// # Confidence
/// high - the `!= -1` guard and the forward are literal.
pub unsafe fn fn_802e9e08() {
    let id = call0_ret(FN_802E9D3C);
    if (id as i32) != -1 {
        let _ = platform::call1(FN_802E9E34, id);
    }
}

/// Build a `(2, 0, id)` queue task on the stack and enqueue it.
///
/// In plain English: "fill a twelve-byte task with `(2, 0, -1)` via
/// `0x802E9BCC`, overwrite the third word with `id`, then enqueue it via
/// `0x802E9B00`."
///
/// # Original
/// - Ghidra: `FUN_802e9e34` at `0x802e9e34` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9e34_FUN_802e9e34.c`
///
/// # Arguments
/// * `id` - `param_1`; the third word of the queued task.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x802E9BCC` (fill) then `0x802E9B00` (enqueue) on a stack task.
///
/// # Safety
/// The stack task is 12 bytes; callee requirements apply.
///
/// # Confidence
/// high - the fill / overwrite / enqueue order is literal. The host
/// `task` array stands in for the export's `auStack_18` / `local_10` pair,
/// matching how `engine::display` passes stack buffers by truncated pointer.
pub unsafe fn fn_802e9e34(id: u32) {
    // observed: `undefined1 auStack_18[8]` + `undefined4 local_10` (12 bytes total).
    let mut task = [0_u32; 3];
    let _ = platform::call2(FN_802E9BCC, task.as_mut_ptr() as u32, 2);
    // observed: `local_10 = param_1` (overwrites the -1 from 0x802E9BCC).
    task[2] = id;
    let _ = platform::call1(FN_802E9B00, task.as_ptr() as u32);
}

/// Init-step stub: return 0.
///
/// In plain English: "return 0."
///
/// # Original
/// - Ghidra: `FUN_802e9e74` at `0x802e9e74` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9e74_FUN_802e9e74.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the body is a single `return 0`.
pub unsafe fn fn_802e9e74() -> u32 {
    0
}

/// Teardown-step stub: return 0.
///
/// In plain English: "return 0."
///
/// # Original
/// - Ghidra: `FUN_802e9e7c` at `0x802e9e7c` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9e7c_FUN_802e9e7c.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the body is a single `return 0`.
pub unsafe fn fn_802e9e7c() -> u32 {
    0
}

/// Length of a NUL-terminated string (`strlen`).
///
/// In plain English: "walk the bytes from `ptr` until the first zero byte
/// and return the count."
///
/// # Original
/// - Ghidra: `FUN_802e9e84` at `0x802e9e84` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9e84_FUN_802e9e84.c`
///
/// # Arguments
/// * `ptr` - `param_1`; pointer to the string bytes.
///
/// # Returns
/// The number of bytes before the first zero byte.
///
/// # Side effects
/// None. Reads bytes from `ptr`.
///
/// # Safety
/// `ptr` must point at a NUL-terminated byte string.
///
/// # Confidence
/// high - the pre-increment loop is a literal `strlen`.
pub unsafe fn fn_802e9e84(ptr: usize) -> i32 {
    // observed: `pcVar2 = param_1 - 1; iVar1 = -1; do { pcVar2++; iVar1++; }`
    let mut cur = ptr.wrapping_sub(1);
    let mut len: i32 = -1;
    loop {
        cur = cur.wrapping_add(1);
        len = len.wrapping_add(1);
        if platform::read_u8(cur) == 0 {
            return len;
        }
    }
}

// ---------------------------------------------------------------------------
// Wave 57 - UART ring buffer, message-buffer pool and platform knobs.
// ---------------------------------------------------------------------------
//
// The debug port keeps one 0x500-byte ring at `0x804256B0` whose bytes live
// in `0x804251B0`, and three 0x88C-byte message buffers starting at
// `0x80425720`. The SDA offsets below resolve against `r13 = 0x8042F680`
// (`docs/GLOSSARY.md`), so `r13 - 0x4668` is `0x8042B018`.

/// `0x8042B018` - `r13 - 0x4668`. Non-zero once the debug serial port has
/// been opened; every blocking read / write checks it first and fails with
/// `-10001` (`0xFFFFD8EF`) when it is zero.
const G_DEBUG_PORT_READY: usize = 0x8042_B018;

/// `0x8042B048` - `r13 - 0x4638`. 16-bit transmit sequence number stamped
/// into every outbound packet at `+0x12`.
const G_TX_SEQUENCE: usize = 0x8042_B048;

/// `0x8042B054` - `r13 - 0x462c`. Word returned by [`fn_802eb090`].
///
/// `// unknown:` the export only shows the load; callers compare it against 0.
const G_8042B054: usize = 0x8042_B054;

/// `0x8042B060` - `r13 - 0x4620`. One-byte flag with a getter / setter pair
/// at [`fn_802ed9c4`] / [`fn_802ed9cc`].
const G_8042B060: usize = 0x8042_B060;

/// `0x804256B0` - the UART ring-buffer control block (24 bytes plus the
/// interrupt-mask word), initialised by [`fn_802e8be8`].
const TRK_RING_STRUCT: usize = 0x8042_56B0;

/// `0x804251B0` - storage behind [`TRK_RING_STRUCT`], 0x500 bytes.
const TRK_RING_DATA: usize = 0x8042_51B0;

/// Ring capacity in bytes (`0x802E8B2C` passes `0x500` to the constructor).
const TRK_RING_CAPACITY: u32 = 0x500;

/// `0x80425720` - base of the three TRK message buffers. Each slot is
/// `0x88C` bytes: a state word, two length words, then 0x880 bytes of data.
const TRK_BUFFER_POOL: usize = 0x8042_5720;

/// Distance between two message-buffer slots (`0x223` words).
const TRK_BUFFER_STRIDE: usize = 0x223 * 4;

/// Payload capacity of one message-buffer slot (`0x880`).
const TRK_BUFFER_DATA_MAX: u32 = 0x880;

/// `0x80427238` - word set by [`fn_802ed91c`]; `0x802ECCB4` tests it as a
/// pointer before building the interrupt-time task.
const G_80427238: usize = 0x8042_7238;

/// `0x804272C0` - word returned by [`fn_802ed2e4`] (the watchdog /
/// exception-handler selector written by the interrupt path).
const G_804272C0: usize = 0x8042_72C0;

/// `0x80350E68` - pointer to the current `OSFont`-style table. `+0x10` is the
/// 256-entry character-translation table used by [`fn_802e886c`].
const PTR_CHAR_TABLE_80350E68: usize = 0x8035_0E68;

/// Format strings named in the message-buffer exports.
const STR_WRITE_UARTN_80351568: usize = 0x8035_1568;
const STR_NO_BUFFER_80351590: usize = 0x8035_1590;

const FN_801B1140: usize = 0x801B_1140;
const FN_801B1180: usize = 0x801B_1180;
const FN_80228358: usize = 0x8022_8358;
const FN_802283B4: usize = 0x8022_83B4;
const FN_80228408: usize = 0x8022_8408;
const FN_802284AC: usize = 0x8022_84AC;
const FN_8022852C: usize = 0x8022_852C;
const FN_80228640: usize = 0x8022_8640;
const FN_80004364: usize = 0x8000_4364;
const FN_802E90B8: usize = 0x802E_90B8;
const FN_802E98F0: usize = 0x802E_98F0;

/// Compares two NUL-terminated strings through the font's character table.
///
/// In plain English: "walk both strings one byte at a time, translating each
/// byte through the current font table, and return `-1`, `0` or `1`."
///
/// # Original
/// - Ghidra: `FUN_802e886c` at `0x802e886c` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e886c_FUN_802e886c.c`
///
/// # Arguments
/// * `lhs` - `param_1`; left-hand string.
/// * `rhs` - `param_2`; right-hand string.
///
/// # Returns
/// `-1` when `lhs` sorts first, `1` when `rhs` sorts first, `0` when equal.
///
/// # Side effects
/// Reads both strings and the table at `*(0x80350E68) + 0x10`.
///
/// # Safety
/// Both pointers must address NUL-terminated byte strings, and
/// `0x80350E68` must hold a table pointer with 256 mapped bytes.
///
/// # Confidence
/// medium - the loop and comparisons are literal; the table's exact contents
/// are not in the export, so only its indexing is certain.
pub unsafe fn fn_802e886c(lhs: usize, rhs: usize) -> i32 {
    // observed: both bytes are `char` (signed) in the export, and only values
    // with `(uint)(int)c < 0x100` are translated, i.e. the ASCII range.
    let table = platform::read_u32(PTR_CHAR_TABLE_80350E68).wrapping_add(0x10) as usize;
    let mut a = lhs;
    let mut b = rhs;
    loop {
        let mut ca = platform::read_u8(a) as i8;
        a = a.wrapping_add(1);
        if (ca as i32 as u32) < 0x100 {
            ca = platform::read_u8(table.wrapping_add(ca as usize)) as i8;
        }
        let mut cb = platform::read_u8(b) as i8;
        b = b.wrapping_add(1);
        if (cb as i32 as u32) < 0x100 {
            cb = platform::read_u8(table.wrapping_add(cb as usize)) as i8;
        }
        if ca < cb {
            return -1;
        }
        if cb < ca {
            return 1;
        }
        if ca == 0 {
            return 0;
        }
    }
}

/// Initialises the UART ring buffer at `0x804256B0`.
///
/// In plain English: "bring up the debug serial port's low-level driver, then
/// point the ring control block at its 0x500-byte storage."
///
/// # Original
/// - Ghidra: `FUN_802e892c` at `0x802e892c` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e892c_FUN_802e892c.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * calls `0x80228358` (serial port setup);
/// * writes the six-word control block at [`TRK_RING_STRUCT`].
///
/// # Safety
/// Must run once, before any read or write of the ring.
///
/// # Confidence
/// medium - the constructor call and the `0x500` capacity are literal, but the
/// decompiler recovered no arguments for `0x80228358` even though its own body
/// takes two.
pub unsafe fn fn_802e892c() -> u32 {
    platform::call0(FN_80228358);
    let _ = fn_802e8be8(TRK_RING_STRUCT, TRK_RING_DATA as u32, TRK_RING_CAPACITY);
    0
}

/// Blocks until `len` bytes have been read from the debug port and stores them
/// in the ring buffer.
///
/// In plain English: "while the ring holds fewer than `len` bytes, read a block
/// from the port and append it; then copy the requested bytes out to the
/// caller's buffer."
///
/// # Original
/// - Ghidra: `FUN_802e899c` at `0x802e899c` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e899c_FUN_802e899c.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination for the `len` bytes.
/// * `len` - `param_2`; byte count wanted.
///
/// # Returns
/// 0 on success, `-10001` (`0xFFFFD8EF`) when the port is closed, or the
/// port-reader error code.
///
/// # Side effects
/// Reads the UART, appends into [`TRK_RING_STRUCT`], then drains it into `dst`.
///
/// # Safety
/// `dst` must have room for `len` bytes; the port must be initialised.
///
/// # Confidence
/// medium - the loop shape is literal; `0x802284AC` is called with `param_2`
/// although its own body only accepts the count it is given.
pub unsafe fn fn_802e899c(dst: usize, len: u32) -> i32 {
    if platform::read_u32(G_DEBUG_PORT_READY) == 0 {
        return -0x2711;
    }
    // observed: `undefined1 auStack_518[1284]` in the export.
    let mut scratch = [0_u8; 1284];
    let mut code: i32 = 0;
    loop {
        let queued = fn_802e8be0(TRK_RING_STRUCT);
        if queued >= len {
            break;
        }
        code = 0;
        let got = call0_ret(FN_80228408);
        if got != 0 {
            code = platform::call2(FN_802284AC, scratch.as_mut_ptr() as u32, len) as i32;
            if code == 0 {
                let _ = fn_802e8c0c(TRK_RING_STRUCT, scratch.as_ptr() as usize, got);
            }
        }
    }
    if code == 0 {
        let _ = fn_802e8d14(TRK_RING_STRUCT, dst as u32, len);
    }
    code
}

/// Writes `len` bytes to the debug port, looping until all are accepted.
///
/// In plain English: "while bytes remain, hand a chunk to the UART writer and
/// advance by however many it took."
///
/// # Original
/// - Ghidra: `FUN_802e8a50` at `0x802e8a50` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8a50_FUN_802e8a50.c`
///
/// # Arguments
/// * `src` - `param_1`; start of the bytes to send.
/// * `len` - `param_2`; byte count.
///
/// # Returns
/// 0 on success, `-10001` (`0xFFFFD8EF`) when the port is closed.
///
/// # Side effects
/// Calls `0x8022852C` once per chunk; that callee drives the UART FIFO.
///
/// # Safety
/// `src` must hold `len` readable bytes.
///
/// # Confidence
/// medium - the loop is literal, but the export types `0x8022852C` as `void`
/// while the loop tests its `r3`, so the return value is inferred.
pub unsafe fn fn_802e8a50(src: usize, len: u32) -> u32 {
    if platform::read_u32(G_DEBUG_PORT_READY) == 0 {
        return 0xFFFF_D8EF;
    }
    let mut cur = src;
    let mut remaining = len as i32;
    while remaining > 0 {
        let sent = platform::call2(FN_8022852C, cur as u32, remaining as u32) as i32;
        if sent == 0 {
            break;
        }
        cur = cur.wrapping_add(sent as usize);
        remaining -= sent;
    }
    0
}

/// Flushes the serial port (thin wrapper over `0x80228640`).
///
/// In plain English: "ask the low-level serial driver to push everything out,
/// then report success."
///
/// # Original
/// - Ghidra: `FUN_802e8ac4` at `0x802e8ac4` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8ac4_FUN_802e8ac4.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Calls `0x80228640`; the export's prototype shows no arguments.
///
/// # Safety
/// The serial driver must be initialised.
///
/// # Confidence
/// high - a single call plus `return 0`.
pub unsafe fn fn_802e8ac4() -> u32 {
    platform::call0(FN_80228640);
    0
}

/// Reads whatever the debug port has and appends it to the ring buffer.
///
/// In plain English: "if bytes are waiting, read them into a stack buffer and
/// queue them for the protocol layer; an empty port is not an error."
///
/// # Original
/// - Ghidra: `FUN_802e8b0c` at `0x802e8b0c` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8b0c_FUN_802e8b0c.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The number of bytes queued, 0 when the port was empty, or `-10009`
/// (`-0x2719`) when the port read failed.
///
/// # Side effects
/// Reads the UART and writes [`TRK_RING_STRUCT`].
///
/// # Safety
/// The ring and the port must be initialised; the stack buffer is 1284 bytes
/// in the export while the port may report up to 0x1FFF, so the shipped code
/// trusts the driver's count.
///
/// # Confidence
/// high - the three-way branch is literal.
pub unsafe fn fn_802e8b0c() -> i32 {
    // observed: `undefined1 auStack_508[1284]` in the export.
    let mut scratch = [0_u8; 1284];
    let mut count = call0_ret(FN_80228408) as i32;
    if count < 1 {
        return 0;
    }
    let rc = platform::call2(FN_802284AC, scratch.as_mut_ptr() as u32, count as u32) as i32;
    if rc == 0 {
        let _ = fn_802e8c0c(TRK_RING_STRUCT, scratch.as_ptr() as usize, count as u32);
    } else {
        count = -0x2719;
    }
    count
}

/// Re-initialises the serial driver (thin wrapper over `0x802283B4`).
///
/// In plain English: "reset the low-level serial port and report success."
///
/// # Original
/// - Ghidra: `FUN_802e8b80` at `0x802e8b80` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8b80_FUN_802e8b80.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Calls `0x802283B4`, which installs the serial interrupt handler.
///
/// # Safety
/// Must run with interrupts available in the debugger context.
///
/// # Confidence
/// high - a single call plus `return 0`.
pub unsafe fn fn_802e8b80() -> u32 {
    platform::call0(FN_802283B4);
    0
}

/// Enters the ring buffer's critical section and saves the interrupt state.
///
/// In plain English: "mask interrupts and remember what they were."
///
/// # Original
/// - Ghidra: `FUN_802e8ba8` at `0x802e8ba8` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8ba8_FUN_802e8ba8.c`
///
/// # Arguments
/// * `lock_word` - `param_1`; address that receives the saved MSR-derived
///   state (the `+0x18` word of [`TRK_RING_STRUCT`]).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801B1140` (interrupt disable) and stores its result.
///
/// # Safety
/// `lock_word` must be a writable aligned word; callers must pair it with
/// [`fn_802e8bd8`].
///
/// # Confidence
/// medium - the store is literal; the callee is identified as the
/// OSDisableInterrupts-shaped primitive from its `MSR` body, not a symbol.
pub unsafe fn fn_802e8ba8(lock_word: usize) {
    let state = call0_ret(FN_801B1140);
    platform::write_u32(lock_word, state);
}

/// Leaves the ring buffer's critical section.
///
/// In plain English: "restore the interrupt state saved by the matching
/// [`fn_802e8ba8`]."
///
/// # Original
/// - Ghidra: `FUN_802e8bd8` at `0x802e8bd8` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8bd8_FUN_802e8bd8.c`
///
/// # Arguments
/// * `lock_word` - `param_1`; word holding the saved state.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls `0x801B1180` (interrupt restore).
///
/// # Safety
/// Must follow a matching [`fn_802e8ba8`] on the same word.
///
/// # Confidence
/// medium - see [`fn_802e8ba8`]; the load / call sequence is literal.
pub unsafe fn fn_802e8bd8(lock_word: usize) {
    let _ = platform::call1(FN_801B1180, platform::read_u32(lock_word));
}

/// Number of bytes currently queued in the ring buffer.
///
/// In plain English: "return the fill counter at `+0x10`."
///
/// # Original
/// - Ghidra: `FUN_802e8be0` at `0x802e8be0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8be0_FUN_802e8be0.c`
///
/// # Arguments
/// * `ring` - `param_1`; ring control block.
///
/// # Returns
/// The `+0x10` word: bytes written by the port and not yet read.
///
/// # Side effects
/// Reads memory only.
///
/// # Safety
/// `ring` must point at an initialised control block.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_802e8be0(ring: usize) -> u32 {
    platform::read_u32(ring + 0x10)
}

/// Fills in the ring-buffer control block and returns the address just past it.
///
/// In plain English: "point the read and write cursors at the same storage,
/// record the capacity and free space, and hand back the lock word after it."
///
/// # Original
/// - Ghidra: `FUN_802e8be8` at `0x802e8be8` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8be8_FUN_802e8be8.c`
///
/// # Arguments
/// * `ring` - `param_1`; control block to initialise.
/// * `base` - `param_2`; storage address (`0x804251B0` at the only call site).
/// * `capacity` - `param_3`; storage size in bytes (`0x500`).
///
/// # Returns
/// `ring + 0x18`, the word the lock helpers use.
///
/// # Side effects
/// Writes `ring[0]`, `ring[1]`, `ring[2]` = `base`, `ring[3]` = capacity,
/// `ring[4]` = 0 and `ring[5]` = capacity (word indices).
///
/// # Safety
/// `ring` must address 24 writable bytes.
///
/// # Confidence
/// high - the six stores and the returned pointer are literal.
pub unsafe fn fn_802e8be8(ring: usize, base: u32, capacity: u32) -> usize {
    // observed: `param_1[2] = param_2; param_1[3] = param_3; *param_1 = param_2;`
    // `param_1[1] = param_2; param_1[4] = 0; param_1[5] = param_3;`
    platform::write_u32(ring + 0x08, base);
    platform::write_u32(ring + 0x0C, capacity);
    platform::write_u32(ring + 0x00, base);
    platform::write_u32(ring + 0x04, base);
    platform::write_u32(ring + 0x10, 0);
    platform::write_u32(ring + 0x14, capacity);
    ring + 0x18
}

/// Appends `len` bytes to the ring buffer, wrapping once at the end of storage.
///
/// In plain English: "copy into the free tail of the ring; if the write runs
/// off the end, finish at the base. Then advance the cursors."
///
/// # Original
/// - Ghidra: `FUN_802e8c0c` at `0x802e8c0c` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8c0c_FUN_802e8c0c.c`
///
/// # Arguments
/// * `ring` - `param_1`; control block.
/// * `src` - `param_2`; bytes to append.
/// * `len` - `param_3`; byte count.
///
/// # Returns
/// 0 on success, `-1` when the free counter at `+0x14` is smaller than `len`.
///
/// # Side effects
/// * masks interrupts through [`fn_802e8ba8`] / [`fn_802e8bd8`];
/// * copies with `memcpy` (`0x80004364`);
/// * updates the write cursor `+0x04`, free `+0x14` and used `+0x10`.
///
/// # Safety
/// `src` must hold `len` bytes; `ring` must be initialised.
///
/// # Confidence
/// high - every load, store and branch is literal in the export.
pub unsafe fn fn_802e8c0c(ring: usize, src: usize, len: u32) -> u32 {
    if platform::read_u32(ring + 0x14) < len {
        return 0xFFFF_FFFF;
    }
    fn_802e8ba8(ring + 0x18);
    let write_ptr = platform::read_u32(ring + 0x04);
    let base = platform::read_u32(ring + 0x08);
    let capacity = platform::read_u32(ring + 0x0C);
    // observed: `uVar3 = *(int *)(param_1 + 0xc) - (iVar2 - *(int *)(param_1 + 8))`
    let tail_free = capacity.wrapping_sub(write_ptr.wrapping_sub(base));
    if tail_free < len {
        // observed: `FUN_80004364` is the 668-byte `memcpy`.
        platform::call3(FN_80004364, write_ptr, src as u32, tail_free);
        platform::call3(
            FN_80004364,
            base,
            src.wrapping_add(tail_free as usize) as u32,
            len - tail_free,
        );
        platform::write_u32(ring + 0x04, base.wrapping_add(len).wrapping_sub(tail_free));
    } else {
        platform::call3(FN_80004364, write_ptr, src as u32, len);
        platform::write_u32(ring + 0x04, write_ptr.wrapping_add(len));
    }
    if platform::read_u32(ring + 0x0C) == platform::read_u32(ring + 0x04).wrapping_sub(base) {
        platform::write_u32(ring + 0x04, base);
    }
    platform::write_u32(ring + 0x14, platform::read_u32(ring + 0x14).wrapping_sub(len));
    platform::write_u32(ring + 0x10, platform::read_u32(ring + 0x10).wrapping_add(len));
    fn_802e8bd8(ring + 0x18);
    0
}

/// Removes `len` bytes from the ring buffer into the caller's buffer.
///
/// In plain English: "if enough bytes are queued, copy them out of the ring
/// (wrapping at the end of storage) and advance the read cursor."
///
/// # Original
/// - Ghidra: `FUN_802e8d14` at `0x802e8d14` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8d14_FUN_802e8d14.c`
///
/// # Arguments
/// * `ring` - `param_1`; control block.
/// * `dst` - `param_2`; destination buffer.
/// * `len` - `param_3`; byte count.
///
/// # Returns
/// 0 on success, `-1` when the used counter at `+0x10` is smaller than `len`.
///
/// # Side effects
/// * masks interrupts through [`fn_802e8ba8`] / [`fn_802e8bd8`];
/// * copies with `memcpy` (`0x80004364`);
/// * updates the read cursor `+0x00`, free `+0x14` and used `+0x10`.
///
/// # Safety
/// `dst` must hold `len` bytes; `ring` must be initialised.
///
/// # Confidence
/// high - every load, store and branch is literal in the export.
pub unsafe fn fn_802e8d14(ring: usize, dst: u32, len: u32) -> u32 {
    if platform::read_u32(ring + 0x10) < len {
        return 0xFFFF_FFFF;
    }
    fn_802e8ba8(ring + 0x18);
    let read_ptr = platform::read_u32(ring + 0x00);
    let base = platform::read_u32(ring + 0x08);
    let capacity = platform::read_u32(ring + 0x0C);
    let tail_used = capacity.wrapping_sub(read_ptr.wrapping_sub(base));
    if len < tail_used {
        platform::call3(FN_80004364, dst, read_ptr, len);
        platform::write_u32(ring + 0x00, read_ptr.wrapping_add(len));
    } else {
        platform::call3(FN_80004364, dst, read_ptr, tail_used);
        platform::call3(
            FN_80004364,
            dst.wrapping_add(tail_used),
            base,
            len - tail_used,
        );
        platform::write_u32(ring + 0x00, base.wrapping_add(len).wrapping_sub(tail_used));
    }
    if platform::read_u32(ring + 0x0C) == platform::read_u32(ring + 0x00).wrapping_sub(base) {
        platform::write_u32(ring + 0x00, base);
    }
    platform::write_u32(ring + 0x14, platform::read_u32(ring + 0x14).wrapping_add(len));
    platform::write_u32(ring + 0x10, platform::read_u32(ring + 0x10).wrapping_sub(len));
    fn_802e8bd8(ring + 0x18);
    0
}

/// Transmits one message buffer over the UART with a fresh sequence number.
///
/// In plain English: "stamp the buffer with the next sequence number, hand it
/// to the UART writer, and complain if the writer fails."
///
/// # Original
/// - Ghidra: `FUN_802ea820` at `0x802ea820` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea820_FUN_802ea820.c`
///
/// # Arguments
/// * `buf` - `param_1`; one [`TRK_BUFFER_POOL`] slot.
///
/// # Returns
/// Always 0; a writer failure is only printed.
///
/// # Side effects
/// * writes the 16-bit sequence at `+0x12` and increments
///   [`G_TX_SEQUENCE`];
/// * calls `0x802E98F0` (the handler slot at `0x804256E4`) and, on a non-zero
///   result, prints `"MetroTRK : TRK_WriteUARTN returned %ld"`.
///
/// # Safety
/// `buf` must be an allocated slot with `+0x04` holding the payload length.
///
/// # Confidence
/// high - the stamp, increment and error print are literal.
pub unsafe fn fn_802ea820(buf: usize) -> u32 {
    let mut seq = platform::read_u16(G_TX_SEQUENCE) as i16;
    if seq == 0 {
        seq = 1;
    }
    platform::write_u16(buf + 0x12, seq as u16);
    platform::write_u16(G_TX_SEQUENCE, seq.wrapping_add(1) as u16);
    let rc = platform::call2(
        FN_802E98F0,
        buf.wrapping_add(0x0C) as u32,
        platform::read_u32(buf + 0x04),
    );
    if rc != 0 {
        let _ = platform::call2(FN_8015F730, STR_WRITE_UARTN_80351568 as u32, rc);
    }
    0
}

/// Marks all three message buffers free.
///
/// In plain English: "clear the in-use word at the start of each of the three
/// 0x88C-byte slots."
///
/// # Original
/// - Ghidra: `FUN_802ea884` at `0x802ea884` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea884_FUN_802ea884.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Writes 0 to `0x80425720`, `0x80425FAC` and `0x80426838`, i.e. the state
/// word of slots 0, 1 and 2.
///
/// # Safety
/// Only valid at init / teardown, when no buffer is in flight.
///
/// # Confidence
/// high - three stores; the three addresses are exactly `base + n*0x88C`.
pub unsafe fn fn_802ea884() -> u32 {
    platform::write_u32(TRK_BUFFER_POOL, 0);
    platform::write_u32(TRK_BUFFER_POOL + TRK_BUFFER_STRIDE, 0);
    platform::write_u32(TRK_BUFFER_POOL + 2 * TRK_BUFFER_STRIDE, 0);
    0
}

/// Allocates one of the three message buffers.
///
/// In plain English: "scan the three slots for a free one, mark it in use, and
/// return both its index and address; if all are busy, print an error."
///
/// # Original
/// - Ghidra: `FUN_802ea8a4` at `0x802ea8a4` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea8a4_FUN_802ea8a4.c`
///
/// # Arguments
/// * `index_out` - `param_1`; receives the slot index (0..2).
/// * `addr_out` - `param_2`; receives the slot address.
///
/// # Returns
/// 0 on success, `0x300` when no slot is free.
///
/// # Side effects
/// * resets the chosen slot via [`fn_802ea98c`];
/// * stores `1` in its state word;
/// * prints `"MetroTRK : ERROR : No buffer available"` on failure.
///
/// # Safety
/// Both out-pointers must be writable words.
///
/// # Confidence
/// high - the three-iteration scan and both out-stores are literal.
pub unsafe fn fn_802ea8a4(index_out: usize, addr_out: usize) -> u32 {
    let mut status: u32 = 0x300;
    platform::write_u32(addr_out, 0);
    for index in 0..3_u32 {
        let slot = fn_802ea940(index);
        if platform::read_u32(slot) == 0 {
            // observed: the caller passes `keep_data = 1`, so the payload
            // bytes are left as they are and only the two length words clear.
            fn_802ea98c(slot, 1);
            platform::write_u32(slot, 1);
            status = 0;
            platform::write_u32(addr_out, slot as u32);
            platform::write_u32(index_out, index);
            break;
        }
    }
    if status == 0x300 {
        let _ = platform::call1(FN_8015F730, STR_NO_BUFFER_80351590 as u32);
    }
    status
}

/// Address of message-buffer slot `index`.
///
/// In plain English: "slot `0` is `0x80425720`, each next one is 0x88C bytes
/// further on; anything past slot 2 is null."
///
/// # Original
/// - Ghidra: `FUN_802ea940` at `0x802ea940` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea940_FUN_802ea940.c`
///
/// # Arguments
/// * `index` - `param_1`; slot number.
///
/// # Returns
/// The slot address, or 0 when `index >= 3`.
///
/// # Side effects
/// None.
///
/// # Safety
/// The result must be treated as valid only when non-zero.
///
/// # Confidence
/// high - one compare, one multiply-add.
pub unsafe fn fn_802ea940(index: u32) -> usize {
    if index < 3 {
        TRK_BUFFER_POOL + index as usize * TRK_BUFFER_STRIDE
    } else {
        0
    }
}

/// Releases a message-buffer slot.
///
/// In plain English: "clear the in-use word for slot `index`, ignoring the
/// sentinel `-1` and out-of-range values."
///
/// # Original
/// - Ghidra: `FUN_802ea964` at `0x802ea964` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea964_FUN_802ea964.c`
///
/// # Arguments
/// * `index` - `param_1`; slot number or `0xFFFFFFFF`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 0 to the slot's state word.
///
/// # Safety
/// None beyond a valid `index` value.
///
/// # Confidence
/// high - both guards and the store are literal.
pub unsafe fn fn_802ea964(index: u32) {
    if index == 0xFFFF_FFFF {
        return;
    }
    if index > 2 {
        return;
    }
    platform::write_u32(TRK_BUFFER_POOL + index as usize * TRK_BUFFER_STRIDE, 0);
}

/// Resets a message-buffer slot's cursors and optionally its payload.
///
/// In plain English: "zero the two length words; when `keep_data` is 0 also
/// wipe the 0x880-byte payload area."
///
/// # Original
/// - Ghidra: `FUN_802ea98c` at `0x802ea98c` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea98c_FUN_802ea98c.c`
///
/// # Arguments
/// * `buf` - `param_1`; slot base.
/// * `keep_data` - `param_2`; non-zero means leave the payload alone.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `+0x04` and `+0x08`; when `keep_data == 0`, calls the `memset`
/// `0x802E90B8` over `buf + 0x0C` for `0x880` bytes.
///
/// # Safety
/// `buf` must address a full 0x88C-byte slot.
///
/// # Confidence
/// high - the two stores and the conditional wipe are literal.
pub unsafe fn fn_802ea98c(buf: usize, keep_data: u32) {
    platform::write_u32(buf + 0x04, 0);
    platform::write_u32(buf + 0x08, 0);
    if keep_data != 0 {
        return;
    }
    let _ = platform::call3(FN_802E90B8, buf.wrapping_add(0x0C) as u32, 0, TRK_BUFFER_DATA_MAX);
}

/// Sets the payload length of a message-buffer slot.
///
/// In plain English: "record `len` as both the used and the high-water length,
/// rejecting anything above 0x880."
///
/// # Original
/// - Ghidra: `FUN_802ea9b4` at `0x802ea9b4` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea9b4_FUN_802ea9b4.c`
///
/// # Arguments
/// * `buf` - `param_1`; slot base.
/// * `len` - `param_2`; requested payload length.
///
/// # Returns
/// 0 on success, `0x301` when `len > 0x880`.
///
/// # Side effects
/// Writes `+0x08` and, when larger, `+0x04`.
///
/// # Safety
/// `buf` must address a message-buffer slot.
///
/// # Confidence
/// high - the bound and the two stores are literal.
pub unsafe fn fn_802ea9b4(buf: usize, len: u32) -> u32 {
    if len < TRK_BUFFER_DATA_MAX + 1 {
        platform::write_u32(buf + 0x08, len);
        if platform::read_u32(buf + 0x04) < len {
            platform::write_u32(buf + 0x04, len);
        }
        0
    } else {
        0x301
    }
}

/// Appends bytes to a message-buffer slot's payload.
///
/// In plain English: "copy into the end of the 0x880-byte payload area,
/// truncating to what still fits, and report truncation with `0x301`."
///
/// # Original
/// - Ghidra: `FUN_802ea9e4` at `0x802ea9e4` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea9e4_FUN_802ea9e4.c`
///
/// # Arguments
/// * `buf` - `param_1`; slot base.
/// * `src` - `param_2`; bytes to append.
/// * `len` - `param_3`; byte count.
///
/// # Returns
/// 0 on success, `0x301` when the payload had to be truncated (or was full).
///
/// # Side effects
/// * copies with `0x802E8F7C` for multi-byte runs;
/// * updates `+0x08` (used) and `+0x04` (high-water).
///
/// # Safety
/// `src` must hold `len` bytes; `buf` must be a message-buffer slot.
///
/// # Confidence
/// high - the clamp, the single-byte special case and the two stores are
/// literal.
pub unsafe fn fn_802ea9e4(buf: usize, src: usize, len: u32) -> u32 {
    let mut status: u32 = 0;
    let mut count = len;
    if count == 0 {
        return 0;
    }
    let used = platform::read_u32(buf + 0x08);
    if TRK_BUFFER_DATA_MAX - used < count {
        status = 0x301;
        count = TRK_BUFFER_DATA_MAX - used;
    }
    let dst = buf.wrapping_add(used as usize).wrapping_add(0x0C);
    if count == 1 {
        platform::write_u8(dst, platform::read_u8(src));
    } else {
        platform::call3(FN_802E8F7C, dst as u32, src as u32, count);
    }
    let new_used = platform::read_u32(buf + 0x08).wrapping_add(count);
    platform::write_u32(buf + 0x08, new_used);
    platform::write_u32(buf + 0x04, new_used);
    status
}

/// Returns the word at `r13 - 0x462C`.
///
/// In plain English: "hand back the SDA word the debug interrupt path gates
/// on."
///
/// # Original
/// - Ghidra: `FUN_802eb090` at `0x802eb090` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb090_FUN_802eb090.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The value of [`G_8042B054`].
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the load is literal; the word's role is inferred from
/// `0x802EC0F4`, which treats non-zero as "exception handler installed".
pub unsafe fn fn_802eb090() -> u32 {
    platform::read_u32(G_8042B054)
}

/// Returns the watchdog / exception-handler selector word.
///
/// In plain English: "read `0x804272C0`."
///
/// # Original
/// - Ghidra: `FUN_802ed2e4` at `0x802ed2e4` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed2e4_FUN_802ed2e4.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The value of [`G_804272C0`].
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one load.
pub unsafe fn fn_802ed2e4() -> u32 {
    platform::read_u32(G_804272C0)
}

/// Reads the `0x80427230` platform knob.
///
/// In plain English: "return the word the debugger uses to decide whether the
/// request path is enabled."
///
/// # Original
/// - Ghidra: `FUN_802ed4e8` at `0x802ed4e8` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed4e8_FUN_802ed4e8.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The value of [`G_80427230`].
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one load; the same word is written by [`fn_802e95cc`].
pub unsafe fn fn_802ed4e8() -> u32 {
    platform::read_u32(G_80427230)
}

/// Writes the `0x80427230` platform knob.
///
/// In plain English: "store `value` in the request-path enable word."
///
/// # Original
/// - Ghidra: `FUN_802ed4f8` at `0x802ed4f8` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed4f8_FUN_802ed4f8.c`
///
/// # Arguments
/// * `value` - `param_1`; new word value.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes [`G_80427230`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_802ed4f8(value: u32) {
    platform::write_u32(G_80427230, value);
}

/// Enables the `0x80427230` request path and reports success.
///
/// In plain English: "set the enable word to 1 and return 0."
///
/// # Original
/// - Ghidra: `FUN_802ed508` at `0x802ed508` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed508_FUN_802ed508.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Writes `1` to [`G_80427230`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one store plus `return 0`.
pub unsafe fn fn_802ed508() -> u32 {
    platform::write_u32(G_80427230, 1);
    0
}

/// Writes the `0x80427238` word used by the interrupt path.
///
/// In plain English: "store the handler pointer the exception entry tests."
///
/// # Original
/// - Ghidra: `FUN_802ed91c` at `0x802ed91c` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed91c_FUN_802ed91c.c`
///
/// # Arguments
/// * `value` - `param_1`; pointer or 0.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes [`G_80427238`]; `0x802ECCB4` dereferences it when non-zero.
///
/// # Safety
/// When non-zero, `value` must be a valid function pointer.
///
/// # Confidence
/// high - one store; the pointer use is visible in `0x802ECCB4`.
pub unsafe fn fn_802ed91c(value: u32) {
    platform::write_u32(G_80427238, value);
}

/// Forces the high bit of a value (physical-address marker).
///
/// In plain English: "return `value | 0x80000000`, the cached/physical address
/// tag the Wii uses for MMIO."
///
/// # Original
/// - Ghidra: `FUN_802ed92c` at `0x802ed92c` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed92c_FUN_802ed92c.c`
///
/// # Arguments
/// * `value` - `param_1`; address or size.
///
/// # Returns
/// `value | 0x80000000`.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one `ori`.
pub unsafe fn fn_802ed92c(value: u32) -> u32 {
    value | 0x8000_0000
}

/// Writes the one-byte flag at `r13 - 0x4620`.
///
/// In plain English: "store a byte flag that the debugger interrupt code
/// reads."
///
/// # Original
/// - Ghidra: `FUN_802ed9c4` at `0x802ed9c4` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed9c4_FUN_802ed9c4.c`
///
/// # Arguments
/// * `value` - `param_1`; byte value.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one byte at [`G_8042B060`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one byte store.
pub unsafe fn fn_802ed9c4(value: u8) {
    platform::write_u8(G_8042B060, value);
}

/// Reads the one-byte flag at `r13 - 0x4620`.
///
/// In plain English: "return the byte flag written by [`fn_802ed9c4`]."
///
/// # Original
/// - Ghidra: `FUN_802ed9cc` at `0x802ed9cc` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed9cc_FUN_802ed9cc.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The byte at [`G_8042B060`].
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - one byte load.
pub unsafe fn fn_802ed9cc() -> u8 {
    platform::read_u8(G_8042B060)
}

// ---------------------------------------------------------------------------
// Wave 68 - C string helpers and TRK payload accessors.
// ---------------------------------------------------------------------------
//
// The routines in the first two groups are MSL's byte- and wide-character
// string primitives. They are ordinary leaf loops over caller-owned memory,
// reproduced here with `platform::read_u8` / `read_u16`; the original aligned
// word-at-a-time fast paths are called out per body. The third group is the
// MetroTRK message-payload accessor set that Wave 57 named as the next step.

/// `0x8042B038` - `r13 - 0x4648`. Non-zero selects big-endian byte order on
/// the wire; zero makes the payload writers emit little-endian words.
///
/// # Original
/// - Ghidra: `*(int *)((unaff_r13) + -0x4648)` in `802eab18_FUN_802eab18.c`
/// - Observed: `0x8042F680 - 0x4648 = 0x8042B038` (`r13` per `docs/GLOSSARY.md`)
/// - `// inferred:` the MetroTRK "big-endian host" flag; the writers below
///   reverse each word exactly when it is zero.
const G_TRK_BIG_ENDIAN: usize = 0x8042_B038;

/// `0x8042B050` - `r13 - 0x4630`. Notification sequence counter; each sender
/// bumps it by two and puts `old + 1` in the packet header.
///
/// # Original
/// - Ghidra: `*(int *)((unaff_r13) + -0x4630)` in `802eb098_FUN_802eb098.c`
/// - Observed: `0x8042F680 - 0x4630 = 0x8042B050`
const G_TRK_NOTIFY_SEQ: usize = 0x8042_B050;

/// `0x802E3998` - the generic bounded-format driver that takes the append
/// callback in `r3` and the callback context in `r4`.
const FN_802E3998: usize = 0x802E_3998;

/// `0x802E4324` - the wide-character append callback passed to
/// [`FN_802E3998`] by [`fn_802e4390`].
const FN_802E4324: usize = 0x802E_4324;

/// `0x802E9498` - flushes the loaded code segments to memory; translated in
/// Wave 91 as [`fn_802e9498`], which [`fn_802eb1fc`] now calls directly.

/// Copy at most `len` bytes and NUL-pad the rest (MSL `strncpy`).
///
/// In plain English: "copy characters until the source ends or `len` bytes are
/// used, then fill the remainder with zeroes."
///
/// # Original
/// - Ghidra: `FUN_802e19f8` at `0x802e19f8` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e19f8_FUN_802e19f8.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination buffer.
/// * `src` - `param_2`; NUL-terminated source.
/// * `len` - `param_3`; capacity in bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes up to `len` bytes at `dst`.
///
/// # Safety
/// `src` must hold `len` readable bytes or an earlier NUL; `dst..dst+len` must
/// be writable. Overlap is undefined, as in C.
///
/// # Confidence
/// high - the copy-then-zero split is literal in the export.
pub unsafe fn fn_802e19f8(dst: usize, src: usize, len: u32) {
    let mut i = 0_u32;
    while i < len {
        let c = platform::read_u8(src.wrapping_add(i as usize));
        platform::write_u8(dst.wrapping_add(i as usize), c);
        i += 1;
        if c == 0 {
            break;
        }
    }
    while i < len {
        platform::write_u8(dst.wrapping_add(i as usize), 0);
        i += 1;
    }
}

/// Append a NUL-terminated string (MSL `strcat`).
///
/// In plain English: "walk to the end of `dst`, then copy `src` there,
/// including its terminator."
///
/// # Original
/// - Ghidra: `FUN_802e1a3c` at `0x802e1a3c` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1a3c_FUN_802e1a3c.c`
///
/// # Arguments
/// * `dst` - `param_1`; NUL-terminated destination.
/// * `src` - `param_2`; NUL-terminated source.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `strlen(src) + 1` bytes starting at the old terminator of `dst`.
///
/// # Safety
/// `dst` must have room for its own text plus `src`; both must be readable.
///
/// # Confidence
/// high - the end scan and the copy loop are literal.
pub unsafe fn fn_802e1a3c(dst: usize, src: usize) {
    let mut end = dst;
    while platform::read_u8(end) != 0 {
        end = end.wrapping_add(1);
    }
    let mut s = src;
    loop {
        let c = platform::read_u8(s);
        platform::write_u8(end, c);
        if c == 0 {
            return;
        }
        end = end.wrapping_add(1);
        s = s.wrapping_add(1);
    }
}

/// Append at most `len` characters (MSL `strncat`).
///
/// In plain English: "walk to the end of `dst`, copy at most `len` characters
/// from `src`, stop early at the source terminator, and always leave a
/// terminator behind."
///
/// # Original
/// - Ghidra: `FUN_802e1a68` at `0x802e1a68` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1a68_FUN_802e1a68.c`
///
/// # Arguments
/// * `dst` - `param_1`; NUL-terminated destination.
/// * `src` - `param_2`; NUL-terminated source.
/// * `len` - `param_3`; maximum characters to append.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes up to `len + 1` bytes at the end of `dst`.
///
/// # Safety
/// `dst` must have `len + 1` bytes free from its terminator.
///
/// # Confidence
/// high - the end scan, bounded copy and final terminator are literal.
pub unsafe fn fn_802e1a68(dst: usize, src: usize, len: u32) {
    let mut end = dst;
    while platform::read_u8(end) != 0 {
        end = end.wrapping_add(1);
    }
    let mut i = 0_u32;
    while i < len {
        let c = platform::read_u8(src.wrapping_add(i as usize));
        platform::write_u8(end, c);
        if c == 0 {
            return;
        }
        end = end.wrapping_add(1);
        i += 1;
    }
    platform::write_u8(end, 0);
}

/// Compare two NUL-terminated byte strings (MSL `strcmp`).
///
/// In plain English: "return the first difference, or 0 when both strings end
/// together."
///
/// # Original
/// - Ghidra: `FUN_802e1ab4` at `0x802e1ab4` (284 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1ab4_FUN_802e1ab4.c`
/// - `// observed:` the export first aligns both pointers, then compares a
///   word at a time with the `0xFEFEFEFF` / `0x80808080` zero-byte test. The
///   same bytes are compared here one at a time; the result is identical for
///   every pair of valid C strings.
///
/// # Arguments
/// * `lhs` - `param_1`; first string.
/// * `rhs` - `param_2`; second string.
///
/// # Returns
/// `lhs_byte - rhs_byte` at the first difference, or 0 when equal.
///
/// # Side effects
/// None.
///
/// # Safety
/// Both pointers must address NUL-terminated strings.
///
/// # Confidence
/// high - every comparison and the two exit conditions are literal; only the
/// word-at-a-time unrolling is collapsed.
pub unsafe fn fn_802e1ab4(lhs: usize, rhs: usize) -> i32 {
    let mut i = 0_usize;
    loop {
        let a = platform::read_u8(lhs.wrapping_add(i)) as i32;
        let b = platform::read_u8(rhs.wrapping_add(i)) as i32;
        if a != b {
            return a - b;
        }
        if a == 0 {
            return 0;
        }
        i += 1;
    }
}

/// Compare at most `len` characters (MSL `strncmp`).
///
/// In plain English: "compare character by character, stopping at the first
/// difference, at the shared terminator, or after `len` characters."
///
/// # Original
/// - Ghidra: `FUN_802e1bd0` at `0x802e1bd0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1bd0_FUN_802e1bd0.c`
///
/// # Arguments
/// * `lhs` - `param_1`; first string.
/// * `rhs` - `param_2`; second string.
/// * `len` - `param_3`; maximum characters to compare.
///
/// # Returns
/// `lhs_byte - rhs_byte` at the first difference, otherwise 0.
///
/// # Side effects
/// None.
///
/// # Safety
/// Both pointers must hold `len` readable bytes or an earlier NUL.
///
/// # Confidence
/// high - the counter, comparison and both exits are literal.
pub unsafe fn fn_802e1bd0(lhs: usize, rhs: usize, len: u32) -> i32 {
    let mut i = 0_u32;
    while i < len {
        let a = platform::read_u8(lhs.wrapping_add(i as usize)) as i32;
        let b = platform::read_u8(rhs.wrapping_add(i as usize)) as i32;
        if a != b {
            return a - b;
        }
        if a == 0 {
            return 0;
        }
        i += 1;
    }
    0
}

/// Find the first occurrence of a byte (MSL `strchr`).
///
/// In plain English: "scan for `c`; searching for 0 finds the terminator."
///
/// # Original
/// - Ghidra: `FUN_802e1c10` at `0x802e1c10` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1c10_FUN_802e1c10.c`
///
/// # Arguments
/// * `s` - `param_1`; NUL-terminated string.
/// * `c` - `param_2`; byte to find.
///
/// # Returns
/// Pointer to the match, or 0 when absent.
///
/// # Side effects
/// None.
///
/// # Safety
/// `s` must be NUL-terminated.
///
/// # Confidence
/// high - the scan order and the `c == 0` rule are literal.
pub unsafe fn fn_802e1c10(s: usize, c: u8) -> usize {
    let mut p = s;
    loop {
        let ch = platform::read_u8(p);
        if ch == c {
            return p;
        }
        if ch == 0 {
            return 0;
        }
        p = p.wrapping_add(1);
    }
}

/// Find the last occurrence of a byte (MSL `strrchr`).
///
/// In plain English: "remember every match while scanning to the terminator,
/// then return the last one."
///
/// # Original
/// - Ghidra: `FUN_802e1c40` at `0x802e1c40` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1c40_FUN_802e1c40.c`
///
/// # Arguments
/// * `s` - `param_1`; NUL-terminated string.
/// * `c` - `param_2`; byte to find.
///
/// # Returns
/// Pointer to the last match; searching for 0 returns the terminator; 0 when
/// absent.
///
/// # Side effects
/// None.
///
/// # Safety
/// `s` must be NUL-terminated.
///
/// # Confidence
/// high - the `last` accumulator and the final `c == 0` test are literal.
pub unsafe fn fn_802e1c40(s: usize, c: u8) -> usize {
    let mut p = s;
    let mut last = 0_usize;
    loop {
        let ch = platform::read_u8(p);
        if ch == 0 {
            break;
        }
        if ch == c {
            last = p;
        }
        p = p.wrapping_add(1);
    }
    if last != 0 {
        return last;
    }
    if c != 0 {
        return 0;
    }
    p
}

/// Find the first occurrence of a substring (MSL `strstr`).
///
/// In plain English: "scan the haystack for the needle's first character and
/// compare the rest; an empty needle matches immediately."
///
/// # Original
/// - Ghidra: `FUN_802e1c88` at `0x802e1c88` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1c88_FUN_802e1c88.c`
///
/// # Arguments
/// * `haystack` - `param_1`; NUL-terminated text to search.
/// * `needle` - `param_2`; NUL-terminated pattern, may be null or empty.
///
/// # Returns
/// Pointer to the first match, or 0 when absent.
///
/// # Side effects
/// None.
///
/// # Safety
/// Both pointers must be valid; a null `needle` is explicitly allowed.
///
/// # Confidence
/// high - the null / empty guards, outer scan and inner compare are literal.
pub unsafe fn fn_802e1c88(haystack: usize, needle: usize) -> usize {
    if needle == 0 {
        return haystack;
    }
    let first = platform::read_u8(needle);
    if first == 0 {
        return haystack;
    }
    let mut p = haystack;
    loop {
        let ch = platform::read_u8(p);
        if ch == 0 {
            return 0;
        }
        if ch == first {
            let mut i = 0_usize;
            loop {
                let n = platform::read_u8(needle.wrapping_add(i));
                if platform::read_u8(p.wrapping_add(i)) != n {
                    break;
                }
                if n == 0 {
                    return p;
                }
                i += 1;
            }
        }
        p = p.wrapping_add(1);
    }
}

/// Append at most `len` bytes to a bounded byte buffer (the narrow append
/// callback used by `0x802E0BF0`).
///
/// In plain English: "copy what still fits at the buffer's write cursor and
/// advance it; always report success."
///
/// # Original
/// - Ghidra: `FUN_802e14f4` at `0x802e14f4` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e14f4_FUN_802e14f4.c`
///
/// # Arguments
/// * `buf` - `param_1`; three-word state: base at `+0x00`, limit at `+0x04`,
///   cursor at `+0x08`.
/// * `src` - `param_2`; bytes to append.
/// * `count` - `param_3`; requested byte count.
///
/// # Returns
/// Always 1.
///
/// # Side effects
/// Copies with `0x80004364`, then adds the copied count to `buf + 0x08`.
///
/// # Safety
/// `buf` must be a readable/writable three-word state and `base + cursor` must
/// have room for the copied bytes.
///
/// # Confidence
/// high - the clamp, the memcpy and the cursor update are literal.
pub unsafe fn fn_802e14f4(buf: usize, src: usize, count: u32) -> u32 {
    let used = platform::read_u32(buf + 0x08);
    let limit = platform::read_u32(buf + 0x04);
    let mut n = limit.wrapping_sub(used);
    if used.wrapping_add(count) <= limit {
        n = count;
    }
    let dst = platform::read_u32(buf).wrapping_add(used);
    platform::call3(FN_80004364, dst, src as u32, n);
    platform::write_u32(
        buf + 0x08,
        platform::read_u32(buf + 0x08).wrapping_add(n),
    );
    1
}

/// Copy `count` 2-byte elements (the wide-character memcpy wrapper at
/// `0x802E21CC`).
///
/// In plain English: "copy `count` 16-bit characters by copying `count * 2`
/// bytes."
///
/// # Original
/// - Ghidra: `FUN_802e21cc` at `0x802e21cc` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e21cc_FUN_802e21cc.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination.
/// * `src` - `param_2`; source.
/// * `count` - `param_3`; element count, not bytes.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One call to the byte memcpy `0x80004364` with `count << 1`.
///
/// # Safety
/// `src` must hold `count * 2` readable bytes; `dst` as many writable ones.
///
/// # Confidence
/// high - the single tail call and the left shift are literal.
pub unsafe fn fn_802e21cc(dst: usize, src: usize, count: u32) {
    platform::call3(FN_80004364, dst as u32, src as u32, count.wrapping_shl(1));
}

/// Append wide characters to a bounded wide buffer (the callback used by
/// [`fn_802e4390`]).
///
/// In plain English: "copy as many wide characters as fit at the write cursor
/// and advance it."
///
/// # Original
/// - Ghidra: `FUN_802e4324` at `0x802e4324` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4324_FUN_802e4324.c`
///
/// # Arguments
/// * `buf` - `param_1`; three-word state: base pointer at `+0x00`, limit at
///   `+0x04`, cursor at `+0x08`.
/// * `src` - `param_2`; wide characters to append.
/// * `count` - `param_3`; requested character count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_802e21cc`] for the copy and adds the copied count to `+0x08`.
///
/// # Safety
/// `base + cursor * 2` must have room for the copied characters.
///
/// # Confidence
/// high - the unsigned clamp and the cursor update are literal.
pub unsafe fn fn_802e4324(buf: usize, src: usize, count: u32) {
    let used = platform::read_u32(buf + 0x08);
    let limit = platform::read_u32(buf + 0x04);
    let mut n = limit.wrapping_sub(used);
    if used.wrapping_add(count) <= limit {
        n = count;
    }
    let dst = (platform::read_u32(buf) as usize).wrapping_add((used as usize) * 2);
    fn_802e21cc(dst, src, n);
    platform::write_u32(
        buf + 0x08,
        platform::read_u32(buf + 0x08).wrapping_add(n),
    );
}

/// Bounded wide-character formatting (the wide twin of `vsnprintf`).
///
/// In plain English: "run the shared format driver over a `{dst, count}`
/// state, terminate the output, and report `-1` when it was truncated."
///
/// # Original
/// - Ghidra: `FUN_802e4390` at `0x802e4390` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4390_FUN_802e4390.c`
///
/// # Arguments
/// * `dst` - `param_1`; output buffer of `count` wide characters.
/// * `count` - `param_2`; capacity in wide characters; must be at least 1.
/// * `arg_a`, `arg_b` - `param_3` / `param_4`; forwarded untouched to the
///   format driver `0x802E3998`.
///
/// # Returns
/// The driver's character count when it fit, otherwise `-1`.
///
/// # Side effects
/// Writes a terminating 0 wide character at `dst + ret` (or at the last slot
/// on truncation), and appends through [`fn_802e4324`].
///
/// # Safety
/// `dst..dst+count` must be writable and `count > 0`; the forwarded arguments
/// must satisfy `0x802E3998`.
///
/// # Confidence
/// high - the three-word state, the driver call and both terminator branches
/// are literal. The `count == 0` underflow is preserved as in the original.
pub unsafe fn fn_802e4390(dst: usize, count: u32, arg_a: u32, arg_b: u32) -> i32 {
    let mut state = [0_u32; 3];
    state[0] = dst as u32;
    state[1] = count;
    state[2] = 0;
    let mut ret = platform::call5(
        FN_802E3998,
        FN_802E4324 as u32,
        state.as_mut_ptr() as u32,
        arg_a,
        arg_b,
        0,
    ) as i32;
    if ret >= 0 {
        if (ret as u32) < count {
            platform::write_u16(dst.wrapping_add((ret as usize) * 2), 0);
        } else {
            ret = -1;
            platform::write_u16(
                dst.wrapping_add((count as usize) * 2).wrapping_sub(2),
                0,
            );
        }
    }
    ret
}

/// Length of a NUL-terminated wide string (MSL `wcslen`).
///
/// In plain English: "count 16-bit characters until the terminator."
///
/// # Original
/// - Ghidra: `FUN_802e441c` at `0x802e441c` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e441c_FUN_802e441c.c`
///
/// # Arguments
/// * `s` - `param_1`; wide string.
///
/// # Returns
/// Number of characters before the terminator.
///
/// # Side effects
/// None.
///
/// # Safety
/// `s` must be NUL-terminated and 2-byte aligned.
///
/// # Confidence
/// high - one increment and one compare in the loop.
pub unsafe fn fn_802e441c(s: usize) -> i32 {
    let mut len = 0_usize;
    while platform::read_u16(s.wrapping_add(len * 2)) != 0 {
        len += 1;
    }
    len as i32
}

/// Copy a NUL-terminated wide string (MSL `wcscpy`).
///
/// In plain English: "copy 16-bit characters through the terminator."
///
/// # Original
/// - Ghidra: `FUN_802e4438` at `0x802e4438` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4438_FUN_802e4438.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination.
/// * `src` - `param_2`; wide source.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `wcslen(src) + 1` 16-bit characters at `dst`.
///
/// # Safety
/// Both pointers must be 2-byte aligned; `dst` must have room for the copy.
///
/// # Confidence
/// high - the copy-until-zero loop is literal.
pub unsafe fn fn_802e4438(dst: usize, src: usize) {
    let mut i = 0_usize;
    loop {
        let c = platform::read_u16(src.wrapping_add(i * 2));
        platform::write_u16(dst.wrapping_add(i * 2), c);
        if c == 0 {
            return;
        }
        i += 1;
    }
}

/// Copy at most `len` wide characters and zero-pad the rest (MSL `wcsncpy`).
///
/// In plain English: "copy until the source ends or `len` characters are
/// used, then fill the remainder with zero wide characters."
///
/// # Original
/// - Ghidra: `FUN_802e4454` at `0x802e4454` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4454_FUN_802e4454.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination.
/// * `src` - `param_2`; wide source.
/// * `len` - `param_3`; capacity in wide characters.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes exactly `len` 16-bit characters at `dst`.
///
/// # Safety
/// Both pointers must be 2-byte aligned; `dst` must hold `len` characters.
///
/// # Confidence
/// high - the copy / pad split is literal.
pub unsafe fn fn_802e4454(dst: usize, src: usize, len: u32) {
    let mut i = 0_u32;
    while i < len {
        let c = platform::read_u16(src.wrapping_add(i as usize * 2));
        platform::write_u16(dst.wrapping_add(i as usize * 2), c);
        i += 1;
        if c == 0 {
            break;
        }
    }
    while i < len {
        platform::write_u16(dst.wrapping_add(i as usize * 2), 0);
        i += 1;
    }
}

/// Compare two NUL-terminated wide strings (MSL `wcscmp`).
///
/// In plain English: "return the first 16-bit difference, or 0 when both end
/// together."
///
/// # Original
/// - Ghidra: `FUN_802e4498` at `0x802e4498` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4498_FUN_802e4498.c`
///
/// # Arguments
/// * `lhs` - `param_1`; first wide string.
/// * `rhs` - `param_2`; second wide string.
///
/// # Returns
/// `lhs_char - rhs_char` at the first difference, otherwise 0.
///
/// # Side effects
/// None.
///
/// # Safety
/// Both pointers must be 2-byte aligned and NUL-terminated.
///
/// # Confidence
/// high - the compare and both exits are literal; characters are unsigned.
pub unsafe fn fn_802e4498(lhs: usize, rhs: usize) -> i32 {
    let mut i = 0_usize;
    loop {
        let a = platform::read_u16(lhs.wrapping_add(i * 2)) as i32;
        let b = platform::read_u16(rhs.wrapping_add(i * 2)) as i32;
        if a != b {
            return a - b;
        }
        if a == 0 {
            return 0;
        }
        i += 1;
    }
}

/// Find the first occurrence of a wide character (MSL `wcschr`).
///
/// In plain English: "scan for `c`; searching for 0 finds the terminator."
///
/// # Original
/// - Ghidra: `FUN_802e44cc` at `0x802e44cc` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e44cc_FUN_802e44cc.c`
///
/// # Arguments
/// * `s` - `param_1`; wide string.
/// * `c` - `param_2`; character to find.
///
/// # Returns
/// Pointer to the match, or 0 when absent.
///
/// # Side effects
/// None.
///
/// # Safety
/// `s` must be 2-byte aligned and NUL-terminated.
///
/// # Confidence
/// high - the scan order and the `c == 0` rule are literal.
pub unsafe fn fn_802e44cc(s: usize, c: u16) -> usize {
    let mut p = s;
    loop {
        let ch = platform::read_u16(p);
        if ch == c {
            return p;
        }
        if ch == 0 {
            return 0;
        }
        p = p.wrapping_add(2);
    }
}

/// Release the message buffer referenced by an event's third word.
///
/// In plain English: "the task event carries a buffer index at `+0x08`; free
/// that slot."
///
/// # Original
/// - Ghidra: `FUN_802e9be4` at `0x802e9be4` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9be4_FUN_802e9be4.c`
///
/// # Arguments
/// * `event` - `param_1`; 12-byte task event from the queue.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Clears the in-use word of the slot named at `event + 0x08`, via
/// [`fn_802ea964`].
///
/// # Safety
/// `event + 0x08` must be readable and hold a valid slot index or -1.
///
/// # Confidence
/// high - one load and one tail call; [`fn_802e8e90`] already passes its
/// event scratch here.
pub unsafe fn fn_802e9be4(event: usize) {
    fn_802ea964(platform::read_u32(event + 0x08));
}

/// Read bytes out of a message-buffer payload.
///
/// In plain English: "copy up to the number of valid bytes left, advance the
/// read cursor, and report `0x302` when the request was longer than what
/// remained."
///
/// # Original
/// - Ghidra: `FUN_802eaa88` at `0x802eaa88` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eaa88_FUN_802eaa88.c`
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot. `+0x04` is the valid length,
///   `+0x08` the read cursor, `+0x0C` the payload.
/// * `dst` - `param_2`; destination bytes.
/// * `count` - `param_3`; requested byte count.
///
/// # Returns
/// 0 on a full read, `0x302` when the payload had fewer bytes than requested
/// (the request is then truncated).
///
/// # Side effects
/// Copies with `0x802E8F7C` and adds the copied count to the cursor.
///
/// # Safety
/// `dst` must have `count` writable bytes; `buf` must be a valid slot.
///
/// # Confidence
/// high - the zero-count exit, the unsigned clamp and the cursor update are
/// literal.
pub unsafe fn fn_802eaa88(buf: usize, dst: usize, count: u32) -> u32 {
    if count == 0 {
        return 0;
    }
    let used = platform::read_u32(buf + 0x08);
    let available = platform::read_u32(buf + 0x04).wrapping_sub(used);
    let mut n = count;
    let mut status = 0_u32;
    if available < n {
        status = 0x302;
        n = available;
    }
    let src = buf.wrapping_add(used as usize).wrapping_add(0x0C);
    platform::call3(FN_802E8F7C, dst as u32, src as u32, n);
    platform::write_u32(
        buf + 0x08,
        platform::read_u32(buf + 0x08).wrapping_add(n),
    );
    status
}

/// Append one word to a message-buffer payload in wire byte order.
///
/// In plain English: "write four bytes, reversed when the connection is not
/// big-endian, clamp at the 0x880-byte payload limit, and report `0x301` on
/// truncation."
///
/// # Original
/// - Ghidra: `FUN_802eab18` at `0x802eab18` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eab18_FUN_802eab18.c`
/// - `// observed:` the export builds the four bytes in `local_14..local_11`
///   only when `r13 - 0x4648 == 0`; otherwise it copies the word unchanged.
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `value` - `param_2`; word to append.
///
/// # Returns
/// 0 on success, `0x301` when the payload was truncated.
///
/// # Side effects
/// Writes 0..4 payload bytes and sets both `+0x04` and `+0x08` to the new
/// length.
///
/// # Safety
/// `buf` must be a valid slot and `+0x08 <= 0x880`.
///
/// # Confidence
/// high - both byte-order branches, the one-byte special case and the two
/// stores are literal.
pub unsafe fn fn_802eab18(buf: usize, value: u32) -> u32 {
    let big_endian = platform::read_u32(G_TRK_BIG_ENDIAN) != 0;
    let bytes = if big_endian {
        value.to_be_bytes()
    } else {
        value.to_le_bytes()
    };
    let used = platform::read_u32(buf + 0x08);
    let mut status = 0_u32;
    let mut n = 4_u32;
    let room = TRK_BUFFER_DATA_MAX.wrapping_sub(used);
    if room < 4 {
        status = 0x301;
        n = room;
    }
    let dst = buf.wrapping_add(used as usize).wrapping_add(0x0C);
    if n == 1 {
        platform::write_u8(dst, bytes[0]);
    } else {
        platform::call3(FN_802E8F7C, dst as u32, bytes.as_ptr() as u32, n);
    }
    let new_used = platform::read_u32(buf + 0x08).wrapping_add(n);
    platform::write_u32(buf + 0x08, new_used);
    platform::write_u32(buf + 0x04, new_used);
    status
}

/// Append a byte string to a message-buffer payload.
///
/// In plain English: "write bytes one at a time until done, or stop with
/// `0x301` when the 0x880-byte payload is full."
///
/// # Original
/// - Ghidra: `FUN_802eacdc` at `0x802eacdc` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eacdc_FUN_802eacdc.c`
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `src` - `param_2`; bytes to append.
/// * `count` - `param_3`; byte count.
///
/// # Returns
/// 0 on success, `0x301` if the payload filled up before `count` bytes.
///
/// # Side effects
/// For each stored byte, increments `+0x08` and the valid length at `+0x04`.
///
/// # Safety
/// `src` must hold `count` bytes; `buf` must be a valid slot.
///
/// # Confidence
/// high - the per-byte bound test and the two counters are literal.
pub unsafe fn fn_802eacdc(buf: usize, src: usize, count: u32) -> i32 {
    let mut status = 0_i32;
    let mut i = 0_u32;
    while i < count {
        let used = platform::read_u32(buf + 0x08);
        if used < TRK_BUFFER_DATA_MAX {
            let value = platform::read_u8(src.wrapping_add(i as usize));
            platform::write_u8(
                buf.wrapping_add(used as usize).wrapping_add(0x0C),
                value,
            );
            platform::write_u32(buf + 0x08, used.wrapping_add(1));
            platform::write_u32(
                buf + 0x04,
                platform::read_u32(buf + 0x04).wrapping_add(1),
            );
            status = 0;
        } else {
            status = 0x301;
        }
        i += 1;
    }
    status
}

/// Append an array of words to a message-buffer payload.
///
/// In plain English: "for each word, write four bytes in wire order, clamped
/// at the payload limit."
///
/// # Original
/// - Ghidra: `FUN_802ead40` at `0x802ead40` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ead40_FUN_802ead40.c`
/// - `// observed:` the status word is re-zeroed at the top of every
///   iteration, so only the last word's truncation survives.
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `words` - `param_2`; array of words.
/// * `count` - `param_3`; word count.
///
/// # Returns
/// 0, or `0x301` when the last iteration could not store all four bytes.
///
/// # Side effects
/// Writes payload bytes and updates `+0x08` / `+0x04`.
///
/// # Safety
/// `words` must hold `count` readable words; `buf` must be a valid slot.
///
/// # Confidence
/// high - the per-word byte-order branch, clamp and counter update are
/// literal.
pub unsafe fn fn_802ead40(buf: usize, words: usize, count: u32) -> i32 {
    let big_endian = platform::read_u32(G_TRK_BIG_ENDIAN) != 0;
    let mut status = 0_i32;
    let mut i = 0_u32;
    while i < count {
        let value = platform::read_u32(words.wrapping_add(i as usize * 4));
        let bytes = if big_endian {
            value.to_be_bytes()
        } else {
            value.to_le_bytes()
        };
        let used = platform::read_u32(buf + 0x08);
        status = 0;
        let mut n = 4_u32;
        let room = TRK_BUFFER_DATA_MAX.wrapping_sub(used);
        if room < 4 {
            status = 0x301;
            n = room;
        }
        let dst = buf.wrapping_add(used as usize).wrapping_add(0x0C);
        if n == 1 {
            platform::write_u8(dst, bytes[0]);
        } else {
            platform::call3(FN_802E8F7C, dst as u32, bytes.as_ptr() as u32, n);
        }
        let new_used = platform::read_u32(buf + 0x08).wrapping_add(n);
        platform::write_u32(buf + 0x08, new_used);
        platform::write_u32(buf + 0x04, new_used);
        i += 1;
    }
    status
}

/// Read eight payload bytes and byte-swap them into the caller's buffer when
/// the connection is little-endian.
///
/// In plain English: "read a 64-bit field; in little-endian mode reverse its
/// eight bytes before handing it back."
///
/// # Original
/// - Ghidra: `FUN_802eae30` at `0x802eae30` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eae30_FUN_802eae30.c`
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `dst` - `param_2`; 8-byte destination.
///
/// # Returns
/// 0 on success, `0x302` when fewer than eight bytes remained.
///
/// # Side effects
/// Advances the read cursor; writes up to 8 bytes at `dst`.
///
/// # Safety
/// `dst` must be 8 bytes writable; `buf` must be a valid slot.
///
/// # Confidence
/// high - the temp-buffer choice and the exact reverse order are literal.
pub unsafe fn fn_802eae30(buf: usize, dst: usize) -> i32 {
    let big_endian = platform::read_u32(G_TRK_BIG_ENDIAN) != 0;
    let mut tmp = [0_u8; 8];
    let target = if big_endian {
        dst
    } else {
        tmp.as_mut_ptr() as usize
    };
    let used = platform::read_u32(buf + 0x08);
    let available = platform::read_u32(buf + 0x04).wrapping_sub(used);
    let mut status = 0_i32;
    let mut n = 8_u32;
    if available < 8 {
        status = 0x302;
        n = available;
    }
    let src = buf.wrapping_add(used as usize).wrapping_add(0x0C);
    platform::call3(FN_802E8F7C, target as u32, src as u32, n);
    platform::write_u32(
        buf + 0x08,
        platform::read_u32(buf + 0x08).wrapping_add(n),
    );
    if !big_endian && status == 0 {
        let mut i = 0_usize;
        while i < 8 {
            platform::write_u8(dst.wrapping_add(i), tmp[7 - i]);
            i += 1;
        }
    }
    status
}

/// Read `count` payload bytes one at a time.
///
/// In plain English: "copy one byte per iteration, reporting `0x302` for every
/// iteration that found the payload empty."
///
/// # Original
/// - Ghidra: `FUN_802eaf10` at `0x802eaf10` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eaf10_FUN_802eaf10.c`
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `dst` - `param_2`; destination bytes.
/// * `count` - `param_3`; byte count.
///
/// # Returns
/// 0, or `0x302` when the last iteration found the payload empty.
///
/// # Side effects
/// Advances the read cursor by the number of bytes copied (0 or 1 per step).
///
/// # Safety
/// `dst` must have `count` writable bytes; `buf` must be a valid slot.
///
/// # Confidence
/// high - the one-byte copy and the empty/full test are literal.
pub unsafe fn fn_802eaf10(buf: usize, dst: usize, count: u32) -> i32 {
    let mut status = 0_i32;
    let mut i = 0_u32;
    while i < count {
        let used = platform::read_u32(buf + 0x08);
        status = 0;
        let empty = platform::read_u32(buf + 0x04) == used;
        if empty {
            status = 0x302;
        }
        let n = if empty { 0 } else { 1 };
        let src = buf.wrapping_add(used as usize).wrapping_add(0x0C);
        platform::call3(
            FN_802E8F7C,
            dst.wrapping_add(i as usize) as u32,
            src as u32,
            n,
        );
        platform::write_u32(
            buf + 0x08,
            platform::read_u32(buf + 0x08).wrapping_add(n),
        );
        i += 1;
    }
    status
}

/// Read `count` payload words and byte-swap each into the caller's array.
///
/// In plain English: "read a word at a time; in little-endian mode reverse
/// its four bytes into the destination."
///
/// # Original
/// - Ghidra: `FUN_802eafa8` at `0x802eafa8` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eafa8_FUN_802eafa8.c`
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `dst` - `param_2`; destination word array.
/// * `count` - `param_3`; word count.
///
/// # Returns
/// 0, or `0x302` when the last word found fewer than four bytes.
///
/// # Side effects
/// Advances the read cursor and writes up to `count * 4` bytes at `dst`.
///
/// # Safety
/// `dst` must hold `count * 4` writable bytes; `buf` must be a valid slot.
///
/// # Confidence
/// high - the temp-buffer choice, clamp and reverse order are literal.
pub unsafe fn fn_802eafa8(buf: usize, dst: usize, count: u32) -> i32 {
    let big_endian = platform::read_u32(G_TRK_BIG_ENDIAN) != 0;
    let mut status = 0_i32;
    let mut i = 0_u32;
    while i < count {
        let mut tmp = [0_u8; 4];
        let target = if big_endian {
            dst.wrapping_add(i as usize * 4)
        } else {
            tmp.as_mut_ptr() as usize
        };
        let used = platform::read_u32(buf + 0x08);
        let available = platform::read_u32(buf + 0x04).wrapping_sub(used);
        status = 0;
        let mut n = 4_u32;
        if available < 4 {
            status = 0x302;
            n = available;
        }
        let src = buf.wrapping_add(used as usize).wrapping_add(0x0C);
        platform::call3(FN_802E8F7C, target as u32, src as u32, n);
        platform::write_u32(
            buf + 0x08,
            platform::read_u32(buf + 0x08).wrapping_add(n),
        );
        if !big_endian && status == 0 {
            let d = dst.wrapping_add(i as usize * 4);
            platform::write_u8(d, tmp[3]);
            platform::write_u8(d.wrapping_add(1), tmp[2]);
            platform::write_u8(d.wrapping_add(2), tmp[1]);
            platform::write_u8(d.wrapping_add(3), tmp[0]);
        }
        i += 1;
    }
    status
}

/// Send the `0x80` "connect" notification and enable the request path.
///
/// In plain English: "set the request-path flag, build a 0x40-byte header
/// with type `0x80` and the next sequence number, and send it."
///
/// # Original
/// - Ghidra: `FUN_802eb098` at `0x802eb098` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb098_FUN_802eb098.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * writes 1 to [`G_8042B054`];
/// * bumps [`G_TRK_NOTIFY_SEQ`] by 2 and stamps `old + 1` at header `+0x0C`;
/// * sends 0x40 bytes through the handler trampoline [`fn_802e98f0`].
///
/// # Safety
/// The handler slot behind `0x802E98F0` must be installed.
///
/// # Confidence
/// medium - the header fields and sequence arithmetic are literal; the
/// `0x80` command name is `// inferred:` from the MetroTRK notification set.
pub unsafe fn fn_802eb098() -> u32 {
    platform::write_u32(G_8042B054, 1);
    let mut header = [0_u8; 0x40];
    let base = header.as_mut_ptr() as usize;
    platform::call3(FN_802E90B8, base as u32, 0, 0x40);
    platform::write_u32(
        base + 0x0C,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(1),
    );
    platform::write_u8(base + 0x04, 0x80);
    platform::write_u32(base, 0x40);
    platform::write_u8(base + 0x08, 0);
    platform::write_u32(
        G_TRK_NOTIFY_SEQ,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(2),
    );
    let _ = platform::call2(FN_802E98F0, base as u32, 0x40);
    0
}

/// Send the `0x80` "disconnect" notification, disable the request path and
/// queue the local teardown event.
///
/// In plain English: "clear the request-path flag, send the same 0x40-byte
/// `0x80` header, then post event 1 so the dispatcher shuts down."
///
/// # Original
/// - Ghidra: `FUN_802eb108` at `0x802eb108` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb108_FUN_802eb108.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * writes 0 to [`G_8042B054`];
/// * sends 0x40 bytes through [`fn_802e98f0`];
/// * fills a 12-byte event with [`fn_802e9bcc`] and queues it with
///   [`fn_802e9b00`].
///
/// # Safety
/// The handler slot behind `0x802E98F0` must be installed.
///
/// # Confidence
/// medium - the flag store, header and event calls are literal; the command
/// name is `// inferred:`.
pub unsafe fn fn_802eb108() -> u32 {
    platform::write_u32(G_8042B054, 0);
    let mut header = [0_u8; 0x40];
    let base = header.as_mut_ptr() as usize;
    platform::call3(FN_802E90B8, base as u32, 0, 0x40);
    platform::write_u32(
        base + 0x0C,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(1),
    );
    platform::write_u8(base + 0x04, 0x80);
    platform::write_u32(base, 0x40);
    platform::write_u8(base + 0x08, 0);
    platform::write_u32(
        G_TRK_NOTIFY_SEQ,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(2),
    );
    let _ = platform::call2(FN_802E98F0, base as u32, 0x40);
    let mut event = [0_u32; 4];
    fn_802e9bcc(event.as_mut_ptr() as usize, 1);
    let _ = fn_802e9b00(event.as_ptr() as usize);
    0
}

/// Send the `0x80` "continue" notification and re-enter the connection.
///
/// In plain English: "build the same 0x80 header, send it, then run the
/// connection step `0x802E9614`."
///
/// # Original
/// - Ghidra: `FUN_802eb190` at `0x802eb190` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb190_FUN_802eb190.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * bumps [`G_TRK_NOTIFY_SEQ`] by 2 and sends 0x40 bytes;
/// * calls [`fn_802e9614`].
///
/// # Safety
/// The handler slot behind `0x802E98F0` must be installed.
///
/// # Confidence
/// medium - the identical header code and the tail call are literal; the
/// command name is `// inferred:`.
pub unsafe fn fn_802eb190() -> u32 {
    let mut header = [0_u8; 0x40];
    let base = header.as_mut_ptr() as usize;
    platform::call3(FN_802E90B8, base as u32, 0, 0x40);
    platform::write_u32(
        base + 0x0C,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(1),
    );
    platform::write_u8(base + 0x04, 0x80);
    platform::write_u32(base, 0x40);
    platform::write_u8(base + 0x08, 0);
    platform::write_u32(
        G_TRK_NOTIFY_SEQ,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(2),
    );
    let _ = platform::call2(FN_802E98F0, base as u32, 0x40);
    fn_802e9614();
    0
}

/// Send the `0x80` "reset" notification and flush the code segments.
///
/// In plain English: "build the same 0x80 header, send it, then flush the
/// cached code with `0x802E9498`."
///
/// # Original
/// - Ghidra: `FUN_802eb1fc` at `0x802eb1fc` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb1fc_FUN_802eb1fc.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * bumps [`G_TRK_NOTIFY_SEQ`] by 2 and sends 0x40 bytes;
/// * calls [`fn_802e9498`], which writes back and invalidates the loaded
///   code segments.
///
/// # Safety
/// The handler slot behind `0x802E98F0` must be installed, and the segment
/// table used by `0x802E9498` must be initialised.
///
/// # Confidence
/// medium - the header code and the flush call are literal; the command name
/// is `// inferred:`.
pub unsafe fn fn_802eb1fc() -> u32 {
    let mut header = [0_u8; 0x40];
    let base = header.as_mut_ptr() as usize;
    platform::call3(FN_802E90B8, base as u32, 0, 0x40);
    platform::write_u32(
        base + 0x0C,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(1),
    );
    platform::write_u8(base + 0x04, 0x80);
    platform::write_u32(base, 0x40);
    platform::write_u8(base + 0x08, 0);
    platform::write_u32(
        G_TRK_NOTIFY_SEQ,
        platform::read_u32(G_TRK_NOTIFY_SEQ).wrapping_add(2),
    );
    let _ = platform::call2(FN_802E98F0, base as u32, 0x40);
    fn_802e9498();
    0
}

// ---------------------------------------------------------------------------
// Wave 82 - the unaligned storage primitives, the request dispatcher, the
// message handlers and the TRK breakpoint / watchpoint knobs.
// ---------------------------------------------------------------------------
//
// Everything below is reached from MetroTRK. The `FN_*` constants name the
// callees that this pass did **not** translate (they are still called by
// absolute address); the `G_*` constants are the SDA / .sbss words the bodies
// read. `r13 = 0x8042F680`, so `r13 - 0x4698 = 0x8042AFE8` (the C library
// `errno` slot) and `r13 - 0x4630 = 0x8042B050` (already [`G_TRK_NOTIFY_SEQ`]).

/// `0x8042AFE8` - `r13 - 0x4698`, MSL `errno`.
///
/// # Original
/// - Ghidra: `*(undefined4 *)(unaff_r13 + -0x4698)` in `802e2108_FUN_802e2108.c`
const G_ERRNO: usize = 0x8042_AFE8;

/// `0x802E1CF4` - the MSL integer parser shared by `atoi` / `strtol`. It takes
/// the radix, the clamp value, a one-character fetch callback, its context and
/// three output words (scratch, sign, status).
///
/// # Original
/// - Ghidra: `FUN_802e1cf4` at `0x802e1cf4` (1044 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1cf4_FUN_802e1cf4.c`
const FN_802E1CF4: usize = 0x802E_1CF4;

/// `0x802E18B0` - the fetch callback handed to [`FN_802E1CF4`] by
/// [`fn_802e2108`]: reads one `char` from the cursor word.
///
/// # Original
/// - Ghidra: `&LAB_802e18b0` in `802e2108_FUN_802e2108.c`
const FN_802E18B0: usize = 0x802E_18B0;

/// `0x8022863C` - callee of [`fn_802e8ae8`]; `// unknown:` the export does not
/// show the body (it is outside this bucket).
const FN_8022863C: usize = 0x8022_863C;

/// `0x802E9940` / `0x802E992C` - the two handler-table trampolines re-armed by
/// the interrupt teardown [`fn_802ea4c4`].
const FN_802E9940: usize = 0x802E_9940;
const FN_802E992C: usize = 0x802E_992C;

/// `0x802E8E1C` - `dcbst` / `dcbf` / `sync` / `icbi` cache-range flush.
///
/// Wave 106 translated the body as [`fn_802e8e1c`], so the callers below use it
/// directly; the constant is kept for reference.
///
/// # Original
/// - Ghidra: `FUN_802e8e1c` at `0x802e8e1c` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8e1c_FUN_802e8e1c.c`
const FN_802E8E1C: usize = 0x802E_8E1C;

/// `0x802E9EA0` - the chunked request/reply transfer loop used by the D0/D1
/// block requests. Wave 91 translated it as [`fn_802e9ea0`], so the D0/D1
/// branch below calls it directly.
///
/// # Original
/// - Ghidra: `FUN_802e9ea0` at `0x802e9ea0` (500 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9ea0_FUN_802e9ea0.c`

/// D2 / D3 / D4 request senders. Wave 68 skipped their bodies; Wave 91
/// translated all three as [`fn_802ea1bc`], [`fn_802ea2d4`] and
/// [`fn_802ea3b8`], which the ladder below calls directly.

/// `0x802EC374` - copies a byte block between the host buffer and a target
/// address (direction selected by its last argument). Used by the read/write
/// memory and register requests.
const FN_802EC374: usize = 0x802E_C374;

/// `0x802EC4C4` / `0x802EC5BC` / `0x802EC6F8` / `0x802EC85C` - the four
/// register-block transfers selected by the register-set byte (`0` = GPR,
/// `1` = FPR, `2` = SPR, `3` = MSR/CR).
const FN_802EC4C4: usize = 0x802E_C4C4;
const FN_802EC5BC: usize = 0x802E_C5BC;
const FN_802EC6F8: usize = 0x802E_C6F8;
const FN_802EC85C: usize = 0x802E_C85C;

/// `0x80351620` .. `0x8035162C` - the 16-byte saved interrupt context that the
/// block transfers bracket themselves with (saved on entry, restored on exit).
const G_80351620: usize = 0x8035_1620;
const G_80351628: usize = 0x8035_1628;

/// Watchpoint / breakpoint state words used by [`fn_802ed0c4`],
/// [`fn_802ed1e8`], [`fn_802ed270`] and [`fn_802ed2f4`].
///
/// * `0x804270D8` - "single-step / watch armed" flag;
/// * `0x804270DC` - comparison kind (0 = count down, 1 = address range,
///   0x10 = single-step);
/// * `0x804270E0` - remaining step count;
/// * `0x804270E4` / `0x804270E8` - low / high address of the watched range;
/// * `0x80427438` - the HID0-style control word the handlers toggle
///   (`0x400` = ICMP enable, `0x8000` / `0x400` mask pair);
/// * `0x80427538` - low half = the last exception type (`0xD00` = data
///   breakpoint).
const G_804270D8: usize = 0x8042_70D8;
const G_804270DC: usize = 0x8042_70DC;
const G_804270E0: usize = 0x8042_70E0;
const G_804270E4: usize = 0x8042_70E4;
const G_804270E8: usize = 0x8042_70E8;
const G_80427438: usize = 0x8042_7438;
const G_80427538: usize = 0x8042_7538;

/// The exception-frame words read by [`fn_802ed2f4`]: `0x80427250` request
/// code, `0x80427254` buffer pointer, `0x80427258` byte count, `0x8042724C`
/// reply status, `0x804272C0` saved PC.
const G_8042724C: usize = 0x8042_724C;
const G_80427250: usize = 0x8042_7250;
const G_80427254: usize = 0x8042_7254;
const G_80427258: usize = 0x8042_7258;

/// `0x80427188` - the data word handed to a patched function by
/// [`fn_802ed8b4`]; it sits in the saved-GPR block at [`TRK_GPR_CONTEXT`].
const G_80427188: usize = 0x8042_7188;

/// `0x800000DC` / `0x800000E4` - the OS thread-queue head and the current
/// thread pointer, walked by [`fn_802ed934`].
///
/// `// inferred:` the 0x2FC stride and the `0x80000000` tag are not in the
/// export; the walk itself is.
const G_800000DC: usize = 0x8000_00DC;
const G_800000E4: usize = 0x8000_00E4;

/// `0x802FE308` / `0x802FE30C` - the cached address window, and
/// `0x802FE310` / `0x802FE314` the per-mode "window present" flags, tested by
/// [`fn_802ec1b0`].
///
/// # Original
/// - Ghidra: `DAT_802fe308` .. `DAT_802fe314` in `802ec1b0_FUN_802ec1b0.c`
const G_CACHE_LO: usize = 0x802F_E308;
const G_CACHE_HI: usize = 0x802F_E30C;
const G_CACHE_FLAG_READ: usize = 0x802F_E310;
const G_CACHE_FLAG_WRITE: usize = 0x802F_E314;

/// The three format strings printed by the serial-option handler
/// [`fn_802ebf48`].
///
/// # Original
/// - Ghidra: `s_MetroTRK_Option___SerialIO___803515f0`,
///   `s_Disable_80351617`, `s_Enable_8035160f` in `802ebf48_FUN_802ebf48.c`
const STR_OPTION_803515F0: usize = 0x8035_15F0;
const STR_DISABLE_80351617: usize = 0x8035_1617;
const STR_ENABLE_8035160F: usize = 0x8035_160F;

/// Copy a NUL-terminated string (MSL `strcpy`).
///
/// In plain English: "copy bytes through the terminator."
///
/// # Original
/// - Ghidra: `FUN_802e1938` at `0x802e1938` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1938_FUN_802e1938.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination buffer.
/// * `src` - `param_2`; NUL-terminated source.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `strlen(src) + 1` bytes at `dst`.
///
/// # Safety
/// `dst` must be large enough and must not overlap `src` in a way that
/// clobbers unread source bytes.
///
/// # Confidence
/// high - the export's aligned word-at-a-time fast path writes exactly the
/// same bytes as this loop (same collapse as `strcmp` in Wave 68).
pub unsafe fn fn_802e1938(dst: usize, src: usize) {
    let mut i = 0_usize;
    loop {
        let c = platform::read_u8(src.wrapping_add(i));
        platform::write_u8(dst.wrapping_add(i), c);
        if c == 0 {
            return;
        }
        i += 1;
    }
}

/// Parse a decimal string with MSL `atoi` semantics, including the `errno`
/// and saturation behaviour.
///
/// In plain English: "run the shared integer parser in base 10 with a
/// `LONG_MAX` clamp, apply the sign, and report `ERANGE` plus the saturated
/// value on overflow."
///
/// # Original
/// - Ghidra: `FUN_802e2108` at `0x802e2108` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e2108_FUN_802e2108.c`
///
/// # Arguments
/// * `s` - `param_1`; NUL-terminated decimal string.
///
/// # Returns
/// The parsed value; `0x7FFFFFFF` or `0x80000000` when the parser overflows.
///
/// # Side effects
/// Writes `0x22` (`ERANGE`) to [`G_ERRNO`] on overflow.
///
/// # Safety
/// `s` must point at a readable NUL-terminated string.
///
/// # Confidence
/// medium - the clamp, the sign test and the `ERANGE` store are literal; the
/// two status words mean "sign" and "stopped on NUL", which is inferred from
/// the arithmetic (`// inferred:`).
pub unsafe fn fn_802e2108(s: usize) -> u32 {
    let mut cursor = s as u32;
    let mut scratch: u32 = 0;
    let mut sign: u32 = 0;
    let mut status = [0_u32; 2];
    let mut value = platform::call7(
        FN_802E1CF4,
        10,
        0x7FFF_FFFF,
        FN_802E18B0 as u32,
        &mut cursor as *mut u32 as u32,
        &mut scratch as *mut u32 as u32,
        &mut sign as *mut u32 as u32,
        status.as_mut_ptr() as u32,
    );
    if status[0] == 0
        && (sign != 0 || value < 0x8000_0000)
        && (sign == 0 || value < 0x8000_0001)
    {
        if sign != 0 {
            value = value.wrapping_neg();
        }
    } else {
        platform::write_u32(G_ERRNO, 0x22);
        let bump = ((sign.wrapping_neg() | sign) as i32 >> 31) as u32;
        value = 0x7FFF_FFFF_u32.wrapping_sub(bump);
    }
    value
}

/// Find a 16-bit character in a counted wide array (MSL `wmemchr`).
///
/// In plain English: "scan up to `count` wide characters and return the first
/// match, or null."
///
/// # Original
/// - Ghidra: `FUN_802e21d4` at `0x802e21d4` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e21d4_FUN_802e21d4.c`
///
/// # Arguments
/// * `s` - `param_1`; wide array.
/// * `ch` - `param_2`; character, compared as 16 bits.
/// * `count` - `param_3`; element count.
///
/// # Returns
/// Address of the match, or `0`.
///
/// # Side effects
/// None.
///
/// # Safety
/// `s` must hold `count` readable 16-bit elements.
///
/// # Confidence
/// high - the count guard, the compare and the 2-byte stride are literal.
pub unsafe fn fn_802e21d4(s: usize, ch: u32, count: u32) -> usize {
    let mut cur = s;
    let mut left = count;
    loop {
        if left == 0 {
            return 0;
        }
        if platform::read_u16(cur) as u32 == ch {
            return cur;
        }
        cur = cur.wrapping_add(2);
        left -= 1;
    }
}

/// Tail-call the platform reset hook and report success.
///
/// In plain English: "call `0x8022863C`, ignore its result, return 0."
///
/// # Original
/// - Ghidra: `FUN_802e8ae8` at `0x802e8ae8` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8ae8_FUN_802e8ae8.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Whatever `0x8022863C` does; `// unknown:` the callee's body is outside this
/// bucket.
///
/// # Safety
/// The callee must not require arguments the caller cannot supply.
///
/// # Confidence
/// medium - the call and the zero return are literal; the callee's role is
/// unknown.
pub unsafe fn fn_802e8ae8() -> u32 {
    platform::call0(FN_8022863C);
    0
}

/// Copy `len` bytes with unaligned loads and stores.
///
/// In plain English: "copy byte by byte, ascending."
///
/// # Original
/// - Ghidra: `FUN_802e8f7c` at `0x802e8f7c` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8f7c_FUN_802e8f7c.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination address (may be unaligned).
/// * `src` - `param_2`; source address (may be unaligned).
/// * `len` - `param_3`; byte count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes at `dst`.
///
/// # Safety
/// `src..src+len` readable, `dst..dst+len` writable.
///
/// # Confidence
/// high - the export's two-bytes-per-pass read-modify-write sequence touches
/// the same bytes in the same order as this loop; only the word-sized accesses
/// are collapsed.
pub unsafe fn fn_802e8f7c(dst: usize, src: usize, len: u32) {
    let mut i = 0_u32;
    while i < len {
        let byte = platform::read_u8(src.wrapping_add(i as usize));
        platform::write_u8(dst.wrapping_add(i as usize), byte);
        i += 1;
    }
}

/// Fill `len` bytes with an unaligned byte pattern (MSL `memset` with no
/// return value).
///
/// In plain English: "store the low byte of `value` `len` times."
///
/// # Original
/// - Ghidra: `FUN_802e90b8` at `0x802e90b8` (296 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e90b8_FUN_802e90b8.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination address.
/// * `value` - `param_2`; only the low 8 bits are stored.
/// * `len` - `param_3`; byte count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes `len` bytes at `dst`.
///
/// # Safety
/// `dst..dst+len` must be writable.
///
/// # Confidence
/// high - `param_2 & 0xff` is masked in the export and every store writes it;
/// the four-at-a-time unrolled path writes the same bytes.
pub unsafe fn fn_802e90b8(dst: usize, value: u32, len: u32) {
    let byte = (value & 0xFF) as u8;
    let mut i = 0_u32;
    while i < len {
        platform::write_u8(dst.wrapping_add(i as usize), byte);
        i += 1;
    }
}

/// Dispatch one MetroTRK message to its handler, selected by the command byte
/// at `+0x10`.
///
/// In plain English: "reset the message cursor and switch on the command byte;
/// an unknown command reports `0x500`."
///
/// # Original
/// - Ghidra: `FUN_802e91e0` at `0x802e91e0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e91e0_FUN_802e91e0.c`
///
/// # Arguments
/// * `buf` - `param_1`; the 0x88C-byte TRK message buffer.
///
/// # Returns
/// The handler's status word, or `0x500` for an unknown command.
///
/// # Side effects
/// Resets the buffer cursor through [`fn_802ea9b4`] and runs one handler.
///
/// # Safety
/// `buf` must be a valid message buffer whose command byte is readable.
///
/// # Confidence
/// high - every case value and target is literal in the export.
pub unsafe fn fn_802e91e0(buf: usize) -> u32 {
    fn_802ea9b4(buf, 0);
    match platform::read_u8(buf + 0x10) {
        1 => fn_802eb098(),
        2 => fn_802eb108(),
        3 => fn_802eb190(),
        7 => fn_802eb1fc(),
        0x10 => fn_802eb268(buf),
        0x11 => fn_802eb49c(buf),
        0x12 => fn_802eb6ac(buf),
        0x13 => fn_802eb8a8(buf),
        0x17 => fn_802ebf48(buf),
        0x18 => fn_802ebb48(buf),
        0x19 => fn_802ebc0c(buf),
        0x1A => fn_802ebe90(buf),
        _ => 0x500,
    }
}

/// Append one 64-bit value as eight bytes in the connection's byte order.
///
/// In plain English: "serialise `hi:lo` in wire order and append it to the
/// message payload, clamping to the buffer."
///
/// # Original
/// - Ghidra: `FUN_802eabe8` at `0x802eabe8` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eabe8_FUN_802eabe8.c`
///
/// # Arguments
/// * `buf` - `param_1`; message-buffer slot.
/// * `_reg` - `param_2`; never read by the body (the callers pass an
///   uninitialised register: Ghidra prints `extraout_r4`). Kept in the
///   signature because it occupies `r4`.
/// * `hi` - `param_3`; high word of the value.
/// * `lo` - `param_4`; low word of the value.
///
/// # Returns
/// 0, or `0x301` when fewer than eight bytes of room remained.
///
/// # Side effects
/// Appends up to 8 bytes at `+0x0C + used` and updates `+0x08` / `+0x04`.
///
/// # Safety
/// `buf` must be a valid 0x88C-byte slot.
///
/// # Confidence
/// high - the two-word local layout and both byte orders are literal; the
/// unused `r4` is visible as unreferenced in the export.
pub unsafe fn fn_802eabe8(buf: usize, _reg: u32, hi: u32, lo: u32) -> u32 {
    let value = ((hi as u64) << 32) | lo as u64;
    let bytes = if platform::read_u32(G_TRK_BIG_ENDIAN) == 0 {
        value.to_le_bytes()
    } else {
        value.to_be_bytes()
    };
    let used = platform::read_u32(buf + 0x08);
    let mut status = 0_u32;
    let mut n = 8_u32;
    let room = TRK_BUFFER_DATA_MAX.wrapping_sub(used);
    if room < 8 {
        status = 0x301;
        n = room;
    }
    let dst = buf.wrapping_add(used as usize).wrapping_add(0x0C);
    if n == 1 {
        platform::write_u8(dst, bytes[0]);
    } else {
        fn_802e8f7c(dst, bytes.as_ptr() as usize, n);
    }
    let new_used = platform::read_u32(buf + 0x08).wrapping_add(n);
    platform::write_u32(buf + 0x08, new_used);
    platform::write_u32(buf + 0x04, new_used);
    status
}

/// Re-arm the debugger's interrupt handlers and report success.
///
/// In plain English: "clear the single-step hook, re-install the two handler
/// trampolines, call the exception prologue, and return 0."
///
/// # Original
/// - Ghidra: `FUN_802ea4c4` at `0x802ea4c4` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea4c4_FUN_802ea4c4.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Clears [`G_80427230`] through [`fn_802ed4f8`], runs the two trampolines
/// `0x802E9940` / `0x802E992C` and the exception epilogue [`fn_802eccb4`].
///
/// # Safety
/// The handler slots must be initialised.
///
/// # Confidence
/// high - the four calls and the zero return are literal.
pub unsafe fn fn_802ea4c4() -> u32 {
    fn_802ed4f8(0);
    platform::call0(FN_802E9940);
    let _ = fn_802eccb4();
    platform::call0(FN_802E992C);
    0
}

/// Decide whether a target byte range lies inside the cached address window.
///
/// In plain English: "if the window is present, accept ranges that fall inside
/// it; recursively trim a range that straddles both edges."
///
/// # Original
/// - Ghidra: `FUN_802ec1b0` at `0x802ec1b0` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec1b0_FUN_802ec1b0.c`
///
/// # Arguments
/// * `start` - `param_1`; first address.
/// * `len` - `param_2`; byte count (`0` wraps the end address).
/// * `which` - `param_3`; 0 = read window, 1 = write window.
///
/// # Returns
/// 0 when the whole range is inside the window, `0x700` otherwise.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure arithmetic except for the four global reads.
///
/// # Confidence
/// high - the wrap test, the two window bounds and the flag pair are literal.
pub unsafe fn fn_802ec1b0(start: u32, len: u32, which: u32) -> i32 {
    let end = start.wrapping_add(len).wrapping_sub(1);
    if end < start {
        return 0x700;
    }
    if start > platform::read_u32(G_CACHE_HI) || platform::read_u32(G_CACHE_LO) > end {
        return 0x700;
    }
    if which == 0 && platform::read_u32(G_CACHE_FLAG_READ) == 0 {
        return 0x700;
    }
    if which == 1 && platform::read_u32(G_CACHE_FLAG_WRITE) == 0 {
        return 0x700;
    }
    let mut result = 0;
    let low = platform::read_u32(G_CACHE_LO);
    if start < low {
        result = fn_802ec1b0(start, low - start, which);
    }
    let high = platform::read_u32(G_CACHE_HI);
    if result == 0 && high < end {
        result = fn_802ec1b0(high, end.wrapping_sub(high), which);
    }
    result
}

/// Send the `0x91` "support" reply: the current exception PC and status word.
///
/// In plain English: "read four bytes from the saved context address, build a
/// `0x91` header carrying the PC and the status halfword, and send it."
///
/// # Original
/// - Ghidra: `FUN_802ed028` at `0x802ed028` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed028_FUN_802ed028.c`
///
/// # Arguments
/// * `buf` - `param_1`; the message buffer the reply is appended to.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies four bytes out of the target at `0x80351620`, prints a 0x40-byte
/// header and appends it through [`fn_802eacdc`].
///
/// # Safety
/// `buf` must be a valid slot and the block-transfer helper must be able to
/// read the target address.
///
/// # Confidence
/// medium - the header fields and the copy are literal; the roles of
/// `0x80351620` (PC) and `0x80351628` (status) are `// inferred:` from the
/// exception-frame layout.
pub unsafe fn fn_802ed028(buf: usize) {
    let mut msg = [0_u8; 0x40];
    let base = msg.as_mut_ptr() as usize;
    fn_802e90b8(base, 0, 0x40);
    platform::write_u32(base, 0x40);
    platform::write_u8(base + 0x04, 0x91);
    let pc = platform::read_u32(G_80351620);
    platform::write_u32(base + 0x08, pc);
    let mut count = 4_u32;
    let mut value = 0_u32;
    platform::call5(
        FN_802EC374,
        &mut value as *mut u32 as u32,
        pc,
        &mut count as *mut u32 as u32,
        0,
        1,
    );
    platform::write_u32(base + 0x10, platform::read_u16(G_80351628) as u32);
    platform::write_u32(base + 0x0C, value);
    fn_802eacdc(buf, base, 0x40);
}

/// Re-evaluate the data-breakpoint condition after an exception.
///
/// In plain English: "when the breakpoint is armed and the exception was a
/// data breakpoint, decide from the step counter / watched range whether it
/// should re-fire; either clear the control word or re-arm it and step."
///
/// # Original
/// - Ghidra: `FUN_802ed0c4` at `0x802ed0c4` (292 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed0c4_FUN_802ed0c4.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The value of [`G_804270D8`] when the function returns.
///
/// # Side effects
/// * writes [`G_804270D8`], [`G_80427438`], [`G_804270E0`];
/// * calls [`fn_802ed4f8`] on the re-arm path.
///
/// # Safety
/// Only global reads and writes; the exception frame must be initialised.
///
/// # Confidence
/// high - every mask (`0xFFFFFBFF` / `0x8000`, `0xFFFF7BFF` / `0x400`) and
/// both comparison branches are literal.
pub unsafe fn fn_802ed0c4() -> i32 {
    if platform::read_u32(G_804270D8) != 0 {
        let mut disarm = true;
        if (platform::read_u32(G_80427538) & 0xFFFF) == 0xD00 {
            let kind = platform::read_u32(G_804270DC);
            if kind == 0 {
                if platform::read_u32(G_804270E0) != 0 {
                    disarm = false;
                }
            } else if kind == 1
                && platform::read_u32(G_804270E4) <= platform::read_u32(G_804272C0)
                && platform::read_u32(G_804272C0) <= platform::read_u32(G_804270E8)
            {
                disarm = false;
            }
        }
        if disarm {
            platform::write_u32(G_804270D8, 0);
            platform::write_u32(
                G_80427438,
                (platform::read_u32(G_80427438) & 0xFFFF_FBFF) | 0x8000,
            );
        } else {
            platform::write_u32(G_804270D8, 1);
            platform::write_u32(
                G_80427438,
                (platform::read_u32(G_80427438) & 0xFFFF_7BFF) | 0x400,
            );
            let kind = platform::read_u32(G_804270DC);
            if kind == 0 || kind == 0x10 {
                platform::write_u32(
                    G_804270E0,
                    platform::read_u32(G_804270E0).wrapping_sub(1),
                );
            }
            fn_802ed4f8(0);
        }
    }
    platform::read_u32(G_804270D8) as i32
}

/// Arm a "step down to address" breakpoint.
///
/// In plain English: "if the caller asked for a single step, arm it and record
/// the address below which it stops; otherwise report `0x703`."
///
/// # Original
/// - Ghidra: `FUN_802ed1e8` at `0x802ed1e8` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed1e8_FUN_802ed1e8.c`
///
/// # Arguments
/// * `param_1` - `param_1`; the address the step starts from.
/// * `param_2` - `param_2`; 0 = arm, anything else = reject.
///
/// # Returns
/// 0 when armed, `0x703` otherwise.
///
/// # Side effects
/// Writes [`G_804270DC`] (0), [`G_80427438`], [`G_804270D8`] and
/// [`G_804270E0`], then calls [`fn_802ed4f8`].
///
/// # Safety
/// None beyond the global writes.
///
/// # Confidence
/// high - the two mask constants, the `param_1 - 1` store and the `0x703`
/// rejection are literal.
pub unsafe fn fn_802ed1e8(param_1: u32, param_2: u32) -> u32 {
    if param_2 != 0 {
        return 0x703;
    }
    platform::write_u32(G_804270DC, 0);
    platform::write_u32(
        G_80427438,
        (platform::read_u32(G_80427438) & 0xFFFF_7FFF) | 0x400,
    );
    platform::write_u32(G_804270D8, 1);
    platform::write_u32(G_804270E0, param_1.wrapping_sub(1));
    fn_802ed4f8(0);
    0
}

/// Arm a watched-address-range breakpoint.
///
/// In plain English: "record the low and high address and arm the comparator;
/// only the 'arm' mode is accepted."
///
/// # Original
/// - Ghidra: `FUN_802ed270` at `0x802ed270` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed270_FUN_802ed270.c`
///
/// # Arguments
/// * `low` - `param_1`; range start.
/// * `high` - `param_2`; range end.
/// * `mode` - `param_3`; 0 = arm, anything else = reject.
///
/// # Returns
/// 0 when armed, `0x703` otherwise.
///
/// # Side effects
/// Writes [`G_804270DC`] (1), [`G_80427438`], [`G_804270D8`],
/// [`G_804270E4`], [`G_804270E8`], then calls [`fn_802ed4f8`].
///
/// # Safety
/// None beyond the global writes.
///
/// # Confidence
/// high - the range words, the same two masks as [`fn_802ed1e8`] and the
/// `0x703` rejection are literal.
pub unsafe fn fn_802ed270(low: u32, high: u32, mode: i32) -> u32 {
    if mode != 0 {
        return 0x703;
    }
    platform::write_u32(G_804270DC, 1);
    platform::write_u32(
        G_80427438,
        (platform::read_u32(G_80427438) & 0xFFFF_7FFF) | 0x400,
    );
    platform::write_u32(G_804270D8, 1);
    platform::write_u32(G_804270E4, low);
    platform::write_u32(G_804270E8, high);
    fn_802ed4f8(0);
    0
}

/// Patch a `blr` into the caller-supplied code block, flush it and call it.
///
/// In plain English: "write the PowerPC `blr` opcode at `code + 0x24`, flush
/// 0x28 bytes of that block, then call it with `param_1` and a data pointer."
///
/// # Original
/// - Ghidra: `FUN_802ed8b4` at `0x802ed8b4` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed8b4_FUN_802ed8b4.c`
///
/// # Arguments
/// * `param_1` - `param_1`; the value handed to the patched function.
/// * `code` - `param_2`; the code block to patch and call.
/// * `_mode` - `param_3`; present at every call site but never read here.
///
/// # Returns
/// Always 0 (the patched function's result is discarded).
///
/// # Side effects
/// Writes the instruction `0x4E800020` at `code + 0x24`, flushes
/// `code..code+0x28`, then calls `code(param_1, 0x80427188)`.
///
/// # Safety
/// `code` must point at writable, executable memory holding at least 0x28
/// bytes of prepared instructions.
///
/// # Confidence
/// high - the opcode constant, the flush length and the indirect call with
/// `&DAT_80427188` are literal.
pub unsafe fn fn_802ed8b4(param_1: u32, code: usize, _mode: u32) -> u32 {
    platform::write_u32(code + 0x24, 0x4E80_0020);
    fn_802e8e1c(code as u32, 0x28);
    platform::call2(code, param_1, G_80427188 as u32);
    0
}

/// Report how many threads are queued ahead of the current thread.
///
/// In plain English: "walk the thread queue from the head; count the entries
/// and record the index of the current thread."
///
/// # Original
/// - Ghidra: `FUN_802ed934` at `0x802ed934` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed934_FUN_802ed934.c`
///
/// # Arguments
/// * `out_count` - `param_1`; receives the thread count (at least 1).
/// * `out_index` - `param_2`; receives the current thread's index, or 0 when
///   it was not found.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes both output words.
///
/// # Safety
/// Both pointers must be 4-byte writable; the queue must be consistent.
///
/// # Confidence
/// medium - the queue walk, the `0x2FC` stride and the three early exits are
/// literal; the `0x80000000` tag and the pointer roles are `// inferred:`
/// from the OS thread-queue layout.
pub unsafe fn fn_802ed934(out_count: usize, out_index: usize) {
    platform::write_u32(out_count, 1);
    platform::write_u32(out_index, 0);
    let mut cur = platform::read_u32(G_800000DC);
    if cur == 0xFFFF_FFFF || cur == 0 || cur == 0x8000_0000 {
        return;
    }
    let current = platform::read_u32(G_800000E4);
    let mut index = 0_u32;
    while cur != 0 {
        if cur == current {
            platform::write_u32(out_index, index);
        }
        index += 1;
        cur = platform::read_u32(cur.wrapping_add(0x2FC) as usize) | 0x8000_0000;
        if cur == 0xFFFF_FFFF || cur == 0 || cur == 0x8000_0000 {
            break;
        }
    }
    platform::write_u32(out_count, index);
}

/// Drive the pending debugger request selected by the exception-frame request
/// code.
///
/// In plain English: "look at the pending request code and finish it: D0/D1
/// transfer a block to or from the host, D2/D3/D4 read or write one word, and
/// anything else posts a 4-byte event."
///
/// # Original
/// - Ghidra: `FUN_802ed2f4` at `0x802ed2f4` (500 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed2f4_FUN_802ed2f4.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The request status, or 0 for the "post an event" path.
///
/// # Side effects
/// * updates [`G_8042724C`] with the new request status;
/// * advances the saved PC [`G_804272C0`] by 4 for every handled request;
/// * may flush a cache range through `0x802E8E1C`;
/// * may queue a local event through [`fn_802e9bcc`] / [`fn_802e9b00`].
///
/// # Safety
/// The exception frame (`0x80427240..0x804272D0`) must be populated and the
/// buffer pointer at [`G_80427254`] must be valid for the request size.
///
/// # Confidence
/// medium - the dispatch ladder, the status handshake and the PC advance are
/// literal; the four request senders were translated in Wave 91 and are now
/// called directly.
pub unsafe fn fn_802ed2f4() -> i32 {
    let ptr = platform::read_u32(G_80427254) as usize;
    let request = platform::read_u32(G_8042724C);
    let mut status: i32 = 0;
    let mut result: i32;
    if request.wrapping_sub(0xD0) < 2 {
        result = fn_802e9ea0(
            platform::read_u32(G_80427250),
            platform::read_u32(G_80427258),
            platform::read_u32(G_80427254) as usize,
            &mut status as *mut i32 as usize,
            1,
            (request.wrapping_sub(0xD1).leading_zeros() >> 5) as i32,
        );
        if status == 0 && result != 0 {
            status = 1;
        }
        platform::write_u32(G_8042724C, status as u32);
        if request == 0xD1 {
            fn_802e8e1c(
                platform::read_u32(G_80427258),
                platform::read_u32(ptr) as i32,
            );
        }
    } else if request == 0xD2 {
        result = fn_802ea1bc(
            platform::read_u32(G_80427250),
            platform::read_u32(G_80427254) & 0xFF,
            platform::read_u32(G_80427258) as usize,
            &mut status as *mut i32 as usize,
        );
        if status == 0 && result != 0 {
            status = 1;
        }
        platform::write_u32(G_8042724C, status as u32);
    } else if request == 0xD3 {
        result = fn_802ea2d4(
            platform::read_u32(G_80427250),
            &mut status as *mut i32 as usize,
        );
        if status == 0 && result != 0 {
            status = 1;
        }
        platform::write_u32(G_8042724C, status as u32);
    } else if request == 0xD4 {
        let mut word = platform::read_u32(ptr);
        result = fn_802ea3b8(
            platform::read_u32(G_80427250),
            &mut word as *mut u32 as usize,
            (platform::read_u32(G_80427258) & 0xFF) as u8,
            &mut status as *mut i32 as usize,
        );
        if status == 0 && result != 0 {
            status = 1;
        }
        platform::write_u32(G_8042724C, status as u32);
        platform::write_u32(ptr, word);
    } else {
        let mut event = [0_u32; 1];
        fn_802e9bcc(event.as_mut_ptr() as usize, 4);
        fn_802e9b00(event.as_mut_ptr() as usize);
        return 0;
    }
    platform::write_u32(
        G_804272C0,
        platform::read_u32(G_804272C0).wrapping_add(4),
    );
    result
}

/// Handle the `0x10` "read memory" request.
///
/// In plain English: "copy the requested block out of the target, then send a
/// `0x40`-byte header followed by the data; on failure send a `0x80` header
/// carrying the mapped error code."
///
/// # Original
/// - Ghidra: `FUN_802eb268` at `0x802eb268` (564 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb268_FUN_802eb268.c`
///
/// # Arguments
/// * `buf` - `param_1`; message buffer holding the request (`+0x14` flags,
///   `+0x18` length, `+0x1C` target address).
///
/// # Returns
/// 0 after sending the reply, or the transport status from [`fn_802ea820`].
///
/// # Side effects
/// Reads target memory through `0x802EC374`, appends the header and payload to
/// `buf`, and advances [`G_TRK_NOTIFY_SEQ`].
///
/// # Safety
/// `buf` must hold a complete request; the requested target range must be
/// readable.
///
/// # Confidence
/// medium - the flag tests, the header fields and the error ladder are literal;
/// the exact meaning of flag bits 1 / 3 / 6 is `// inferred:` from the
/// neighbouring handlers.
pub unsafe fn fn_802eb268(buf: usize) -> u32 {
    let flags = platform::read_u8(buf + 0x14);
    let addr = platform::read_u32(buf + 0x1C);
    let mut count = platform::read_u16(buf + 0x18) as u32;
    let mut data = [0_u8; 0x830];
    if (flags & 2) != 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x12);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        return 0;
    }
    let mut result = platform::call5(
        FN_802EC374,
        data.as_mut_ptr() as u32,
        addr,
        &mut count as *mut u32 as u32,
        ((flags & 8) == 0) as u32,
        1,
    ) as i32;
    fn_802ea98c(buf, 0);
    if result == 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, count.wrapping_add(0x40));
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0);
        platform::write_u32(h + 0x0C, seq);
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(1));
        let _ = fn_802ea9e4(buf, h, 0x40);
        let src = if (flags & 0x40) == 0 {
            data.as_ptr() as usize
        } else {
            (data.as_ptr() as usize).wrapping_add((addr as usize) & 0x1F)
        };
        result = fn_802ea9e4(buf, src, count) as i32;
    }
    if result == 0 {
        return fn_802ea820(buf);
    }
    let code = match result {
        0x700 => 0x13_u8,
        0x702 => 0x15,
        0x704 => 0x21,
        0x705 => 0x22,
        0x706 => 0x20,
        _ => 3,
    };
    let mut hdr = [0_u8; 0x40];
    let h = hdr.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(h, 0x40);
    platform::write_u8(h + 0x04, 0x80);
    platform::write_u8(h + 0x08, code);
    platform::write_u32(h + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
    0
}

/// Handle the `0x11` "write memory" request.
///
/// In plain English: "read the payload block out of the message, copy it into
/// the target, then acknowledge with a `0x40`-byte `0x80` header."
///
/// # Original
/// - Ghidra: `FUN_802eb49c` at `0x802eb49c` (528 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb49c_FUN_802eb49c.c`
///
/// # Arguments
/// * `buf` - `param_1`; message buffer holding the request.
///
/// # Returns
/// 0 after sending the reply, or the transport status from [`fn_802ea820`].
///
/// # Side effects
/// Writes target memory through `0x802EC374` and appends the reply header to
/// `buf`.
///
/// # Safety
/// `buf` must hold a complete request; the requested target range must be
/// writable.
///
/// # Confidence
/// medium - the cursor move to `0x40`, the payload read, the copy and the
/// acknowledgement are literal; the flag bit roles are `// inferred:`.
pub unsafe fn fn_802eb49c(buf: usize) -> u32 {
    let flags = platform::read_u8(buf + 0x14);
    let addr = platform::read_u32(buf + 0x1C);
    let mut count = platform::read_u16(buf + 0x18) as u32;
    let mut data = [0_u8; 0x830];
    if (flags & 2) != 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x12);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        return 0;
    }
    fn_802ea9b4(buf, 0x40);
    let _ = fn_802eaa88(buf, data.as_mut_ptr() as usize, count);
    let mut result = platform::call5(
        FN_802EC374,
        data.as_ptr() as u32,
        addr,
        &mut count as *mut u32 as u32,
        ((flags & 8) == 0) as u32,
        0,
    ) as i32;
    fn_802ea98c(buf, 0);
    if result == 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0);
        platform::write_u32(h + 0x0C, seq);
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(1));
        result = fn_802ea9e4(buf, h, 0x40) as i32;
    }
    if result == 0 {
        return fn_802ea820(buf);
    }
    let code = match result {
        0x700 => 0x13_u8,
        0x702 => 0x15,
        0x704 => 0x21,
        0x705 => 0x22,
        0x706 => 0x20,
        _ => 3,
    };
    let mut hdr = [0_u8; 0x40];
    let h = hdr.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(h, 0x40);
    platform::write_u8(h + 0x04, 0x80);
    platform::write_u8(h + 0x08, code);
    platform::write_u32(h + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
    0
}

/// Handle the `0x12` "read registers" request.
///
/// In plain English: "send the reply header first, then append the four
/// register blocks (GPR, FPR, SPR, MSR/CR); a failure becomes an error reply."
///
/// # Original
/// - Ghidra: `FUN_802eb6ac` at `0x802eb6ac` (508 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb6ac_FUN_802eb6ac.c`
///
/// # Arguments
/// * `buf` - `param_1`; message buffer holding the request.
///
/// # Returns
/// 0 after sending the reply, or the transport status from [`fn_802ea820`].
///
/// # Side effects
/// Appends the 0x468-byte reply (0x40 header + 0x400 register image),
/// advancing [`G_TRK_NOTIFY_SEQ`] by one.
///
/// # Safety
/// `buf` must have room for the 0x468-byte reply.
///
/// # Confidence
/// medium - the four block calls, their register counts (0x24 / 0x21 / 0x60 /
/// 0x1F) and the error ladder are literal; the export does not show a fill for
/// this header, so the buffer is zeroed here (`// observed:`).
pub unsafe fn fn_802eb6ac(buf: usize) -> u32 {
    if platform::read_u16(buf + 0x1C) < platform::read_u16(buf + 0x18) {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x14);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        return 0;
    }
    let mut hdr = [0_u8; 0x40];
    let h = hdr.as_mut_ptr() as usize;
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(h, 0x468);
    platform::write_u8(h + 0x04, 0x80);
    platform::write_u8(h + 0x08, 0);
    platform::write_u32(h + 0x0C, seq);
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(1));
    fn_802ea98c(buf, 0);
    fn_802eacdc(buf, h, 0x40);
    let mut scratch = [0_u32; 2];
    let sp = scratch.as_mut_ptr() as u32;
    let mut result = platform::call5(FN_802EC4C4, 0, 0x24, buf as u32, sp, 1) as i32;
    if result == 0 {
        result = platform::call5(FN_802EC5BC, 0, 0x21, buf as u32, sp, 1) as i32;
    }
    if result == 0 {
        result = platform::call5(FN_802EC6F8, 0, 0x60, buf as u32, sp, 1) as i32;
    }
    if result == 0 {
        result = platform::call5(FN_802EC85C, 0, 0x1F, buf as u32, sp, 1) as i32;
    }
    if result == 0 {
        return fn_802ea820(buf);
    }
    let code = match result {
        0x703 => 0x12_u8,
        0x701 => 0x14,
        0x702 => 0x15,
        0x704 => 0x21,
        0x705 => 0x22,
        0x706 => 0x20,
        _ => 3,
    };
    let mut err = [0_u8; 0x40];
    let e = err.as_mut_ptr() as usize;
    fn_802e90b8(e, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(e, 0x40);
    platform::write_u8(e + 0x04, 0x80);
    platform::write_u8(e + 0x08, code);
    platform::write_u32(e + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, e as u32, 0x40);
    0
}

/// Handle the `0x13` "write registers" request.
///
/// In plain English: "pick the register block named by the set byte at `+0x14`
/// and copy the payload into it, then acknowledge."
///
/// # Original
/// - Ghidra: `FUN_802eb8a8` at `0x802eb8a8` (672 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eb8a8_FUN_802eb8a8.c`
///
/// # Arguments
/// * `buf` - `param_1`; message buffer holding the request.
///
/// # Returns
/// 0 after sending the reply, or the transport status from [`fn_802ea820`].
///
/// # Side effects
/// Writes the target register image and appends one reply header.
///
/// # Safety
/// `buf` must hold a complete request; the register block must be writable.
///
/// # Confidence
/// medium - the set selection (0..3 -> GPR/FPR/SPR/MSR-CR), the short-range
/// error (`0x14`) and the status ladder are literal; the block identities are
/// `// inferred:` from the register counts.
pub unsafe fn fn_802eb8a8(buf: usize) -> u32 {
    let set = platform::read_u8(buf + 0x14);
    let first = platform::read_u16(buf + 0x18);
    let last = platform::read_u16(buf + 0x1C);
    fn_802ea9b4(buf, 0);
    if last < first {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x14);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        return 0;
    }
    fn_802ea9b4(buf, 0x40);
    let mut scratch = [0_u32; 2];
    let sp = scratch.as_mut_ptr() as u32;
    let mut result = match set {
        0 => platform::call5(FN_802EC4C4, first as u32, last as u32, buf as u32, sp, 0) as i32,
        1 => platform::call5(FN_802EC5BC, first as u32, last as u32, buf as u32, sp, 0) as i32,
        2 => platform::call5(FN_802EC6F8, first as u32, last as u32, buf as u32, sp, 0) as i32,
        3 => platform::call5(FN_802EC85C, first as u32, last as u32, buf as u32, sp, 0) as i32,
        _ => 0x703,
    };
    fn_802ea98c(buf, 0);
    if result == 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0);
        platform::write_u32(h + 0x0C, seq);
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(1));
        result = fn_802ea9e4(buf, h, 0x40) as i32;
    }
    if result == 0 {
        return fn_802ea820(buf);
    }
    let code = if result == 0x702 {
        0x15_u8
    } else if result < 0x702 {
        if result == 0x302 {
            2
        } else if result < 0x302 {
            3
        } else if result > 0x700 {
            0x14
        } else {
            3
        }
    } else if result == 0x705 {
        0x22
    } else if result < 0x705 {
        if result < 0x704 {
            0x12
        } else {
            0x21
        }
    } else if result < 0x707 {
        0x20
    } else {
        3
    };
    let mut err = [0_u8; 0x40];
    let e = err.as_mut_ptr() as usize;
    fn_802e90b8(e, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(e, 0x40);
    platform::write_u8(e + 0x04, 0x80);
    platform::write_u8(e + 0x08, code);
    platform::write_u32(e + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, e as u32, 0x40);
    0
}

/// Handle the `0x18` "continue" request.
///
/// In plain English: "acknowledge with `0x16` when the target is not stopped,
/// otherwise acknowledge and re-arm the interrupt handlers."
///
/// # Original
/// - Ghidra: `FUN_802ebb48` at `0x802ebb48` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ebb48_FUN_802ebb48.c`
///
/// # Arguments
/// * `_buf` - the caller's `r3`; the export's body never reads it.
///
/// # Returns
/// 0, or the value from [`fn_802ea4c4`] on the resume path.
///
/// # Side effects
/// Sends one `0x80` header and, when resuming, re-arms the handlers.
///
/// # Safety
/// The handler slot behind `0x802E98F0` must be installed.
///
/// # Confidence
/// high - both headers, the sequence arithmetic and the [`fn_802ed4e8`] gate
/// are literal.
pub unsafe fn fn_802ebb48(_buf: usize) -> u32 {
    if fn_802ed4e8() == 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x16);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        0
    } else {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        fn_802ea4c4()
    }
}

/// Handle the `0x19` "set / clear breakpoint" request.
///
/// In plain English: "validate the requested address against the stopped PC,
/// acknowledge, then arm either a single-step countdown or a watched range."
///
/// # Original
/// - Ghidra: `FUN_802ebc0c` at `0x802ebc0c` (644 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ebc0c_FUN_802ebc0c.c`
///
/// # Arguments
/// * `buf` - `param_1`; message buffer holding the request (`+0x14` set byte,
///   `+0x18` address byte, `+0x1C` low word, `+0x20` high word).
///
/// # Returns
/// 0 after the reply, or the arming result from [`fn_802ed1e8`] /
/// [`fn_802ed270`].
///
/// # Side effects
/// Sends one `0x80` header and writes the watchpoint globals.
///
/// # Safety
/// `buf` must hold a complete request.
///
/// # Confidence
/// medium - the set tests, the PC range check and the two arming calls are
/// literal; the `0x10`/`0x11` set values are `// inferred:` from the
/// `leading_zeros` trick the compiler used to turn them into 0 / 1.
pub unsafe fn fn_802ebc0c(buf: usize) -> u32 {
    fn_802ea9b4(buf, 0);
    let set = platform::read_u8(buf + 0x14);
    let low = platform::read_u32(buf + 0x1C);
    let high = platform::read_u32(buf + 0x20);
    let address = platform::read_u8(buf + 0x18) as u32;
    if set != 0 && set != 0x10 {
        if set != 1 && set != 0x11 {
            let mut hdr = [0_u8; 0x40];
            let h = hdr.as_mut_ptr() as usize;
            fn_802e90b8(h, 0, 0x40);
            let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
            platform::write_u32(h, 0x40);
            platform::write_u8(h + 0x04, 0x80);
            platform::write_u8(h + 0x08, 0x12);
            platform::write_u32(h + 0x0C, seq.wrapping_add(1));
            platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
            let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
            return 0;
        }
        let pc = fn_802ed2e4();
        if pc < low || high < pc {
            let mut hdr = [0_u8; 0x40];
            let h = hdr.as_mut_ptr() as usize;
            fn_802e90b8(h, 0, 0x40);
            let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
            platform::write_u32(h, 0x40);
            platform::write_u8(h + 0x04, 0x80);
            platform::write_u8(h + 0x08, 0x11);
            platform::write_u32(h + 0x0C, seq.wrapping_add(1));
            platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
            let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
            return 0;
        }
    } else if address == 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x11);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        return 0;
    }
    if fn_802ed4e8() == 0 {
        let mut hdr = [0_u8; 0x40];
        let h = hdr.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
        platform::write_u32(h, 0x40);
        platform::write_u8(h + 0x04, 0x80);
        platform::write_u8(h + 0x08, 0x16);
        platform::write_u32(h + 0x0C, seq.wrapping_add(1));
        platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
        let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
        return 0;
    }
    let mut ack = [0_u8; 0x40];
    let a = ack.as_mut_ptr() as usize;
    fn_802e90b8(a, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(a, 0x40);
    platform::write_u8(a + 0x04, 0x80);
    platform::write_u8(a + 0x08, 0);
    platform::write_u32(a + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, a as u32, 0x40);
    if set == 0 || set == 0x10 {
        fn_802ed1e8(address, set.wrapping_sub(0x10).leading_zeros() >> 5)
    } else {
        fn_802ed270(
            low,
            high,
            (set.wrapping_sub(0x11).leading_zeros() >> 5) as i32,
        )
    }
}

/// Handle the `0x1A` "get breakpoint status" request.
///
/// In plain English: "ask `0x802ED508` for the last breakpoint result and
/// reply with the mapped code."
///
/// # Original
/// - Ghidra: `FUN_802ebe90` at `0x802ebe90` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ebe90_FUN_802ebe90.c`
///
/// # Arguments
/// * `_buf` - the caller's `r3`; the export's body never reads it.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Sends one `0x80` header carrying the mapped code.
///
/// # Safety
/// The handler slot behind `0x802E98F0` must be installed.
///
/// # Confidence
/// high - the four-way code mapping and the header fields are literal.
pub unsafe fn fn_802ebe90(_buf: usize) -> u32 {
    let code = match fn_802ed508() {
        0 => 0_u8,
        0x704 => 0x21,
        0x705 => 0x22,
        0x706 => 0x20,
        _ => 1,
    };
    let mut hdr = [0_u8; 0x40];
    let h = hdr.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(h, 0x40);
    platform::write_u8(h + 0x04, 0x80);
    platform::write_u8(h + 0x08, code);
    platform::write_u32(h + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
    0
}

/// Handle the `0x17` "serial option" request.
///
/// In plain English: "print the MetroTRK serial option, apply it, then
/// acknowledge with an empty `0x80` header."
///
/// # Original
/// - Ghidra: `FUN_802ebf48` at `0x802ebf48` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ebf48_FUN_802ebf48.c`
///
/// # Arguments
/// * `buf` - `param_1`; message buffer (`+0x14` option id, `+0x18` value).
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// * prints one or two lines through `0x8015F730`;
/// * stores the new value through [`fn_802ed9c4`];
/// * sends one `0x80` header and bumps [`G_TRK_NOTIFY_SEQ`] by two.
///
/// # Safety
/// `buf` must hold a complete request.
///
/// # Confidence
/// high - the option test, the three literal strings, the setter call and the
/// header are all literal.
pub unsafe fn fn_802ebf48(buf: usize) -> u32 {
    let value = platform::read_u8(buf + 0x18);
    if platform::read_u8(buf + 0x14) == 1 {
        platform::call1(FN_8015F730, STR_OPTION_803515F0 as u32);
        if value == 0 {
            platform::call1(FN_8015F730, STR_DISABLE_80351617 as u32);
        } else {
            platform::call1(FN_8015F730, STR_ENABLE_8035160F as u32);
        }
        fn_802ed9c4(value);
    }
    let mut hdr = [0_u8; 0x40];
    let h = hdr.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    let seq = platform::read_u32(G_TRK_NOTIFY_SEQ);
    platform::write_u32(h, 0x40);
    platform::write_u8(h + 0x04, 0x80);
    platform::write_u8(h + 0x08, 0);
    platform::write_u32(h + 0x0C, seq.wrapping_add(1));
    platform::write_u32(G_TRK_NOTIFY_SEQ, seq.wrapping_add(2));
    let _ = platform::call2(FN_802E98F0, h as u32, 0x40);
    0
}

// ---------------------------------------------------------------------------
// Wave 91 - the MSL `printf` / `strtol` support code and the TRK reply path.
// ---------------------------------------------------------------------------
//
// `0x802E032C..0x802E1CF4` is the statically linked MSL number parser and
// formatter; `0x802E278C..0x802E4660` are its stream-state helpers;
// `0x802E942C..0x802EA3B8` is the MetroTRK request/reply layer that the Wave 82
// state machine [`fn_802ed2f4`] drives. The `FN_*` constants below name the
// callees this pass did **not** translate; they are still called by absolute
// address.

/// `0x80350E68` - pointer to the MSL ctype descriptor.
///
/// The descriptor holds two table pointers: `+0x08` the 16-bit class map and
/// `+0x0C` the lowercase -> uppercase map. Both are read by [`fn_802e1cf4`].
///
/// # Original
/// - Ghidra: `PTR_DAT_80350e68` in `802e1cf4_FUN_802e1cf4.c`
const PTR_CTYPE_80350E68: usize = 0x8035_0E68;

/// `0x802DE614` - the MSL `fwrite` core: copies `count` elements of `size`
/// bytes into a stream object and returns the elements written.
///
/// # Original
/// - Ghidra: `FUN_802de614` at `0x802de614` (420 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802de614_FUN_802de614.c`
const FN_802DE614: usize = 0x802D_E614;

/// `0x802E0BF0` - the buffered `printf` driver. It takes the append callback,
/// the callback context, the format string, the argument list and a mode word.
///
/// # Original
/// - Ghidra: `FUN_802e0bf0` at `0x802e0bf0` (2220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e0bf0_FUN_802e0bf0.c`
const FN_802E0BF0: usize = 0x802E_0BF0;

/// `0x80350A68` - the shared `stdout` stream object.
///
/// # Original
/// - Ghidra: `&DAT_80350a68` in `802e1560_FUN_802e1560.c`
const G_STDOUT_80350A68: usize = 0x8035_0A68;

/// `0x8015F730` - the debugger's line printer (`MetroTRK : ...` strings);
/// declared once at the top of the file, used throughout this pass.
///
/// # Original
/// - Ghidra: `FUN_8015f730` at `0x8015f730`
/// - Evidence: `work/ghidra/main-gb/export/8015f730_FUN_8015f730.c`

/// Read one entry of the MSL 16-bit character-class map.
///
/// `// inferred:` the mask passed by the callers selects "space" (`0x100`),
/// "decimal digit" (`8`) and "letter" (`1`); the map itself lives outside this
/// bucket.
unsafe fn ctype_flags(c: u32) -> u16 {
    // observed: every ctype read in the export is guarded by `uVar3 < 0x100`;
    // characters outside the byte range get no class bits.
    if c >= 0x100 {
        return 0;
    }
    let table = platform::read_u32(PTR_CTYPE_80350E68 + 8) as usize;
    platform::read_u16(table + (c as usize) * 2)
}

/// Map one character through the MSL lowercase -> uppercase table.
unsafe fn ctype_upper(c: u32) -> u32 {
    let table = platform::read_u32(PTR_CTYPE_80350E68 + 0x0C) as usize;
    platform::read_u8(table + c as usize) as u32
}

/// Write one digit position of a decimal `printf` field: round the digit,
/// carry into the more significant positions, and rewrite the exponent.
///
/// In plain English: "round the number stored as decimal digits at
/// `buf+5 .. buf+4+digits` starting at position `digit`, then either store the
/// rewritten tail or, on a full carry, bump the exponent and restart the
/// mantissa at `1`."
///
/// # Original
/// - Ghidra: `FUN_802e032c` at `0x802e032c` (296 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e032c_FUN_802e032c.c`
///
/// # Arguments
/// * `buf` - `param_1`; the digit block. `+2` is a 16-bit exponent, `+4` the
///   digit count (a byte) and `+5` the first digit.
/// * `digit` - `param_2`; zero-based position to round, from the start.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * rewrites `buf+5 ..` and possibly the exponent at `buf+2`;
/// * writes `buf+4` (digit count) and `buf+5` (leading digit).
///
/// # Safety
/// `buf` must hold `digit+5` writable bytes and a valid digit count.
///
/// # Confidence
/// medium - every store and the carry arithmetic are literal; the meaning
/// "round the decimal mantissa" is `// inferred:` from the exponent/count
/// layout.
pub unsafe fn fn_802e032c(buf: usize, digit: i32) {
    if digit >= 0 {
        let digits = platform::read_u8(buf + 4) as i32;
        if digits <= digit {
            return;
        }
        let at = buf.wrapping_add(digit as usize);
        let mut p = at + 5;
        // `(uint)(char)(*(char *)(iVar4 + 5) + -0x30)`: round up if >= 5.
        let mut carry = (platform::read_u8(p) as i8).wrapping_sub(0x30) as u32;
        let mut remaining = digit;
        if carry == 5 {
            let mut q = buf + digits as usize + 5;
            loop {
                q = q.wrapping_sub(1);
                if q <= p || platform::read_u8(q) != 0x30 {
                    break;
                }
            }
            carry = if q == p {
                (platform::read_u8(at + 4) & 1) as u32
            } else {
                1
            };
        } else {
            // observed: `((int)(uVar5 ^ 5) >> 1) - ((uVar5 ^ 5) & uVar5) >> 0x1f`
            // is the branch-free "digit >= 5" test.
            let x = carry ^ 5;
            carry = ((((x as i32) >> 1) as u32).wrapping_sub(x & carry)) >> 31;
        }
        while remaining != 0 {
            p = p.wrapping_sub(1);
            let ch = (platform::read_u8(p) as i8).wrapping_add(carry as i8);
            let value = ch.wrapping_sub(0x30) as u32;
            let x = value ^ 9;
            carry = ((((x as i32) >> 1) as u32).wrapping_sub(x & value)) >> 31;
            carry = carry.wrapping_neg();
            if carry == 0 && value != 0 {
                platform::write_u8(p, ch as u8);
                break;
            }
            remaining -= 1;
        }
        if carry != 0 {
            platform::write_u8(buf + 4, 1);
            let exponent = platform::read_u16(buf + 2);
            platform::write_u16(buf + 2, exponent.wrapping_add(1));
            platform::write_u8(buf + 5, 0x31);
            return;
        }
        if remaining != 0 {
            platform::write_u8(buf + 4, remaining as u8);
            return;
        }
    }
    platform::write_u16(buf + 2, 0);
    platform::write_u8(buf + 4, 1);
    platform::write_u8(buf + 5, 0x30);
}

/// The narrow append callback the `printf` driver uses for a `FILE`-style
/// stream: write `len` bytes and report the destination on success.
///
/// In plain English: "hand the bytes to `fwrite`; return the stream cookie if
/// every byte was written, else 0."
///
/// # Original
/// - Ghidra: `FUN_802e149c` at `0x802e149c` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e149c_FUN_802e149c.c`
///
/// # Arguments
/// * `stream` - `param_1`; the stream object handed to `0x802DE614`.
/// * `src` - `param_2`; bytes to append.
/// * `len` - `param_3`; byte count.
///
/// # Returns
/// `stream` when the full count was written, otherwise 0.
///
/// # Side effects
/// Writes `len` bytes into the stream (buffer or file).
///
/// # Safety
/// `src..src+len` must be readable and `stream` a valid stream object.
///
/// # Confidence
/// medium - the call and the `written != len` test are literal; that the
/// returned word is the cookie (rather than the byte count) is `// inferred:`
/// from the callback shape.
pub unsafe fn fn_802e149c(stream: u32, src: u32, len: i32) -> u32 {
    let written = platform::call4(FN_802DE614, src, 1, len as u32, stream) as i32;
    if written == len {
        stream
    } else {
        0
    }
}

/// `printf` into `stdout`, forcing the stream to byte orientation first.
///
/// In plain English: "if `stdout` is not wide-oriented, run the format driver
/// and return its result; otherwise report failure."
///
/// # Original
/// - Ghidra: `FUN_802e1560` at `0x802e1560` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1560_FUN_802e1560.c`
///
/// # Arguments
/// * `fmt` - `param_1`; the format string.
/// * `args` - `param_2`; the `va_list` pointer built by the caller.
///
/// # Returns
/// The driver's result, or `0xFFFFFFFF` when the stream is wide-oriented.
///
/// # Side effects
/// Writes to `stdout` and clears its wide-orientation flag through
/// [`fn_802e44f8`].
///
/// # Safety
/// `fmt` and `args` must describe a valid format call.
///
/// # Confidence
/// medium - the orientation test, the callback and the driver call are literal;
/// the stream identity is `// inferred:` from the `&DAT_80350A68` argument.
pub unsafe fn fn_802e1560(fmt: u32, args: u32) -> u32 {
    if fn_802e44f8(G_STDOUT_80350A68, -1) < 0 {
        platform::call5(
            FN_802E0BF0,
            fn_802e149c as *const () as usize as u32,
            G_STDOUT_80350A68 as u32,
            fmt,
            args,
            0,
        )
    } else {
        0xFFFF_FFFF
    }
}

/// Bounded `snprintf` into a caller buffer.
///
/// In plain English: "format into `buf` with `size` bytes of room and always
/// NUL-terminate; on overflow, terminate the last byte instead."
///
/// # Original
/// - Ghidra: `FUN_802e15dc` at `0x802e15dc` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e15dc_FUN_802e15dc.c`
///
/// # Arguments
/// * `buf` - `param_1`; destination (may be 0 to only measure).
/// * `size` - `param_2`; capacity in bytes.
/// * `fmt` - `param_3`; format string.
/// * `args` - `param_4`; `va_list` pointer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes at most `size` bytes at `buf`, including the terminator.
///
/// # Safety
/// `buf..buf+size` must be writable when `buf != 0`.
///
/// # Confidence
/// high - the three-word descriptor, the driver call and both terminator
/// branches are literal.
pub unsafe fn fn_802e15dc(buf: usize, size: u32, fmt: u32, args: u32) {
    // observed: the export builds {buf, size, 0} on the stack and passes its
    // address as the append context.
    let mut desc = [0_u32; 3];
    desc[0] = buf as u32;
    desc[1] = size;
    desc[2] = 0;
    let written = platform::call5(
        FN_802E0BF0,
        fn_802e14f4 as *const () as usize as u32,
        desc.as_mut_ptr() as u32,
        fmt,
        args,
        0,
    );
    if buf != 0 {
        if written < size {
            platform::write_u8(buf.wrapping_add(written as usize), 0);
        } else if size != 0 {
            platform::write_u8(buf.wrapping_add(size as usize - 1), 0);
        }
    }
}

/// Unbounded `sprintf` into a caller buffer.
///
/// In plain English: "format into `buf` through a descriptor whose length word
/// is `-1` (unbounded), then terminate at the reported count."
///
/// # Original
/// - Ghidra: `FUN_802e1664` at `0x802e1664` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1664_FUN_802e1664.c`
///
/// # Arguments
/// * `buf` - `param_1`; destination (may be 0).
/// * `fmt` - `param_2`; format string.
/// * `args` - `param_3`; `va_list` pointer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the formatted string plus a terminator at `buf`.
///
/// # Safety
/// `buf` must have room for the formatted text.
///
/// # Confidence
/// medium - the descriptor stores and both terminator branches are literal; the
/// `-1` length and the `buf-2` store are the published MSL `sprintf` shape
/// (`// inferred:`).
pub unsafe fn fn_802e1664(buf: usize, fmt: u32, args: u32) {
    let mut desc = [0_u32; 3];
    desc[0] = buf as u32;
    desc[1] = 0xFFFF_FFFF;
    desc[2] = 0;
    let written = platform::call5(
        FN_802E0BF0,
        fn_802e14f4 as *const () as usize as u32,
        desc.as_mut_ptr() as u32,
        fmt,
        args,
        0,
    ) as i32;
    if buf != 0 {
        if written == -1 {
            platform::write_u8(buf.wrapping_sub(2), 0);
        } else {
            platform::write_u8(buf.wrapping_add(written as usize), 0);
        }
    }
}

/// The MSL integer parser shared by `atoi` / `strtol`.
///
/// In plain English: "read digits from the fetch callback in the requested
/// base, honouring an optional sign, `0x` / `0` prefixes and leading spaces;
/// accumulate the value, record overflow, and report the index of the last
/// consumed character."
///
/// # Original
/// - Ghidra: `FUN_802e1cf4` at `0x802e1cf4` (1044 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e1cf4_FUN_802e1cf4.c`
///
/// # Arguments
/// * `base` - `param_1`; radix 2..36, or 0 to auto-detect from the prefix.
/// * `limit` - `param_2`; maximum number of characters to consume.
/// * `fetch` - `param_3`; `fetch(ctx, 0, 0)` reads the next character,
///   `fetch(ctx, c, 1)` pushes `c` back.
/// * `ctx` - `param_4`; cursor object handed to `fetch`.
/// * `index_out` - `param_5`; receives the index of the last character, or 0.
/// * `negative_out` - `param_6`; set to 1 when a `-` sign was consumed.
/// * `overflow_out` - `param_7`; set to 1 when the value saturated.
///
/// # Returns
/// The accumulated value (unmodified by the sign).
///
/// # Side effects
/// * one read or push-back call on the callback per character;
/// * writes the three output words.
///
/// # Safety
/// `fetch` must be a valid callback and the three output words writable.
///
/// # Confidence
/// medium - the state machine, the overflow tests and the terminator are
/// literal; the state numbers `0x20` / `0x40` mean "had digits" / "no digits",
/// which is `// inferred:` from the `& 0x34` test, and the export's
/// `fetch(ctx, 0, 0)` calls drop nothing else.
pub unsafe fn fn_802e1cf4(
    mut base: u32,
    limit: i32,
    fetch: usize,
    ctx: u32,
    index_out: usize,
    negative_out: usize,
    overflow_out: usize,
) -> u32 {
    let mut consumed: i32 = 0; // iVar8
    let mut state: u32 = 1; // uVar9
    let mut spaces: i32 = 0; // iVar7
    let mut value: u32 = 0; // uVar6
    let mut before_mul: u32 = 0; // uVar5
    platform::write_u32(overflow_out, 0);
    platform::write_u32(negative_out, 0);
    let mut c: u32; // uVar3
    if (base as i32) < 0 || base == 1 || (base as i32) > 0x24 || limit < 1 {
        state = 0x40;
        // observed: the invalid-base path pushes `param_2` back. Never taken by
        // `atoi`, which always passes base 10 and a large limit.
        c = limit as u32;
    } else {
        consumed = 1;
        c = platform::call3(fetch, ctx, 0, 0);
    }
    if base != 0 {
        before_mul = u32::MAX / base;
    }
    loop {
        // observed: `(int)param_2 < iVar8 || uVar3 == 0xffffffff ||
        // (uVar9 & 0x60) != 0` ends the scan.
        if limit < consumed || c == 0xFFFF_FFFF || (state & 0x60) != 0 {
            if (state & 0x34) == 0 {
                value = 0;
                platform::write_u32(index_out, 0);
            } else {
                platform::write_u32(index_out, (consumed + spaces - 1) as u32);
            }
            // observed: `(*param_3)(param_4, uVar3, 1)` -- push the last
            // character back into the cursor.
            let _ = platform::call3(fetch, ctx, c, 1);
            return value;
        }
        if state == 1 {
            if (ctype_flags(c) & 0x100) != 0 {
                c = platform::call3(fetch, ctx, 0, 0);
                spaces += 1;
            } else {
                if c == 0x2B {
                    consumed += 1;
                    c = platform::call3(fetch, ctx, 0, 0);
                } else if c == 0x2D {
                    consumed += 1;
                    c = platform::call3(fetch, ctx, 0, 0);
                    platform::write_u32(negative_out, 1);
                }
                state = 2;
            }
        } else if state == 2 {
            if (base == 0 || base == 0x10) && c == 0x30 {
                state = 4;
                consumed += 1;
                c = platform::call3(fetch, ctx, 0, 0);
            } else {
                state = 8;
            }
        } else if state == 4 {
            if c == 0x58 || c == 0x78 {
                base = 0x10;
                state = 8;
                consumed += 1;
                c = platform::call3(fetch, ctx, 0, 0);
            } else {
                if base == 0 {
                    base = 8;
                }
                state = 0x10;
            }
        } else {
            // states 8 / 0x10: one digit of the accumulated number.
            if base == 0 {
                base = 10;
            }
            if before_mul == 0 {
                before_mul = u32::MAX / base;
            }
            let digit: u32;
            if (ctype_flags(c) & 8) != 0 {
                let d = c.wrapping_sub(0x30);
                if (d as i32) < (base as i32) {
                    digit = d;
                } else {
                    // observed: decimal class but `digit >= base`. The export
                    // sets 0x20 here and then maps every non-0x10 state to
                    // 0x40, so this ends the scan without a value.
                    state = 0x40;
                    continue;
                }
            } else {
                let upper = if c < 0x100 { ctype_upper(c) } else { c };
                let d = upper.wrapping_sub(0x37);
                if (ctype_flags(c) & 1) != 0 && (d as i32) < (base as i32) {
                    digit = d;
                } else {
                    state = if state == 0x10 { 0x20 } else { 0x40 };
                    continue;
                }
            }
            if before_mul < value {
                platform::write_u32(overflow_out, 1);
            }
            // observed: `-(uVar6 * param_1) - 1 < uVar4`
            if value.wrapping_mul(base).wrapping_neg().wrapping_sub(1) < digit {
                platform::write_u32(overflow_out, 1);
            }
            value = value.wrapping_mul(base).wrapping_add(digit);
            state = 0x10;
            consumed += 1;
            c = platform::call3(fetch, ctx, 0, 0);
        }
    }
}

/// `0x801A9EE0` - reads the CPU's HID-style flag word tested by
/// [`fn_802e4570`].
///
/// # Original
/// - Ghidra: `FUN_801a9ee0` at `0x801a9ee0`
/// - Evidence: `work/ghidra/main-gb/export/801a9ee0_FUN_801a9ee0.c`
const FN_801A9EE0: usize = 0x801A_9EE0;

/// `0x80227C70` / `0x80227CC0` - the OS cache/DMA range helpers: the first
/// reserves `0xE100` bytes of DMA buffer space once, the second validates a
/// range before a transfer.
///
/// # Original
/// - Ghidra: `FUN_80227c70`, `FUN_80227cc0`
/// - Evidence: `work/ghidra/main-gb/export/80227c70_FUN_80227c70.c`,
///   `work/ghidra/main-gb/export/80227cc0_FUN_80227cc0.c`
const FN_80227C70: usize = 0x8022_7C70;
const FN_80227CC0: usize = 0x8022_7CC0;

/// `0x802EC07C` - the cache-flush wrapper Wave 82 left untranslated because its
/// fifth argument is undefined at the call site.
///
/// # Original
/// - Ghidra: `FUN_802ec07c` at `0x802ec07c` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec07c_FUN_802ec07c.c`
const FN_802EC07C: usize = 0x802E_C07C;

/// `0x8042AFF0` - `r13 - 0x4690`: "the DMA pool was reserved" flag.
///
/// # Original
/// - Ghidra: `*(int *)(unaff_r13 + -0x4690)` in `802e4570_FUN_802e4570.c`
const G_8042AFF0: usize = 0x8042_AFF0;

/// `0x8042B000` - `r13 - 0x4680`: the optional flush callback invoked by
/// [`fn_802e4648`].
///
/// # Original
/// - Ghidra: `*(code **)(unaff_r13 + -0x4680)` in `802e4648_FUN_802e4648.c`
const G_8042B000: usize = 0x8042_B000;

/// Convert an unsigned integer to wide digits for `printf` (`%d` / `%u` /
/// `%o` / `%x`), including sign, `0x` prefix and zero padding.
///
/// In plain English: "write the digits backwards from `end-2`, honouring the
/// conversion letter at `+6`, then prepend the sign, the `0x` prefix and the
/// zero padding the spec asks for; return the first digit."
///
/// # Original
/// - Ghidra: `FUN_802e278c` at `0x802e278c` (580 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e278c_FUN_802e278c.c`
///
/// # Arguments
/// * `value` - `param_1`; the number to format (already widened to 32 bits).
/// * `end` - `param_2`; one past the last wide character of the field.
/// * `spec` - `param_3`; the MSL format record: `+0` justification (`2` =
///   right), `+1` sign mode, `+3` `#` flag, `+6` conversion (a wide char),
///   `+8` minimum width, `+0xC` current field width.
///
/// # Returns
/// The address of the first character, or 0 when the field would exceed 510
/// wide characters.
///
/// # Side effects
/// Writes the digit string, its prefix and up to `width` zeros backwards from
/// `end-2`.
///
/// # Safety
/// `end` must point past a writable wide buffer and `spec` at a valid record.
///
/// # Confidence
/// medium - the digit loop, the prefixes and the padding are literal; the
/// record offsets are `// inferred:` from the three formatter bodies that
/// share them.
pub unsafe fn fn_802e278c(mut value: u32, end: u32, spec: u32) -> u32 {
    let sp = spec as usize;
    let mut cursor = end.wrapping_sub(2);
    platform::write_u16(cursor as usize, 0);
    let conv = platform::read_u16(sp + 6);
    if value == 0
        && platform::read_u32(sp + 0xC) == 0
        && (platform::read_u8(sp + 3) == 0 || conv != 0x6F)
    {
        return cursor;
    }
    let mut base: u32 = 0;
    let mut negative = false;
    if conv == 100 || conv == 0x69 {
        base = 10;
        if (value as i32) < 0 {
            if value != 0x8000_0000 {
                value = value.wrapping_neg();
            }
            negative = true;
        }
    } else if conv == 0x6F {
        platform::write_u8(sp + 1, 0);
        base = 8;
    } else if conv == 0x75 {
        platform::write_u8(sp + 1, 0);
        base = 10;
    } else if conv == 0x78 || conv == 0x58 {
        platform::write_u8(sp + 1, 0);
        base = 16;
    }
    // unknown: any other conversion letter leaves `base` at 0; the original
    // would divide by zero, so the formatter never calls it that way.
    let mut digits: i32 = 0;
    let mut before_last: u32;
    loop {
        before_last = cursor;
        let digit = value.wrapping_sub((value / base).wrapping_mul(base));
        value /= base;
        let ch = if (digit as i32) < 10 {
            (digit as i32 + 0x30) as u16
        } else if platform::read_u16(sp + 6) == 0x78 {
            (digit as i32 + 0x57) as u16
        } else {
            (digit as i32 + 0x37) as u16
        };
        cursor = before_last.wrapping_sub(2);
        platform::write_u16(cursor as usize, ch);
        digits += 1;
        if value == 0 {
            break;
        }
    }
    if base == 8 && platform::read_u8(sp + 3) != 0 && platform::read_u16(cursor as usize) != 0x30
    {
        cursor = before_last.wrapping_sub(4);
        platform::write_u16(cursor as usize, 0x30);
        digits += 2;
    }
    if platform::read_u8(sp) == 2 {
        let mut width = platform::read_u32(sp + 8);
        if negative || platform::read_u8(sp + 1) != 0 {
            width = width.wrapping_sub(1);
        }
        if base == 0x10 && platform::read_u8(sp + 3) != 0 {
            width = width.wrapping_sub(2);
        }
        platform::write_u32(sp + 0xC, width);
    }
    if platform::read_i32(sp + 0xC) + (end as i32).wrapping_sub(cursor as i32) / 2 < 0x1FE {
        while digits < platform::read_i32(sp + 0xC) {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x30);
            digits += 1;
        }
        if base == 0x10 && platform::read_u8(sp + 3) != 0 {
            platform::write_u16(cursor.wrapping_sub(2) as usize, platform::read_u16(sp + 6));
            cursor = cursor.wrapping_sub(4);
            platform::write_u16(cursor as usize, 0x30);
        }
        if negative {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x2D);
        } else if platform::read_u8(sp + 1) == 1 {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x2B);
        } else if platform::read_u8(sp + 1) == 2 {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x20);
        }
        cursor
    } else {
        0
    }
}

/// The same conversion as [`fn_802e278c`], for 64-bit values.
///
/// In plain English: "write the 64-bit number's digits backwards from
/// `end-2`, then prepend sign, prefix and zero padding exactly like the 32-bit
/// version."
///
/// # Original
/// - Ghidra: `FUN_802e29d0` at `0x802e29d0` (700 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e29d0_FUN_802e29d0.c`
///
/// # Arguments
/// * `hi` - `param_1`; high word of the value (and its sign).
/// * `lo` - `param_2`; low word of the value.
/// * `end` - `param_3`; one past the last wide character of the field.
/// * `spec` - `param_4`; the MSL format record, see [`fn_802e278c`].
///
/// # Returns
/// The address of the first character, or 0 on field overflow.
///
/// # Side effects
/// Writes the digit string, its prefix and up to `width` zeros backwards from
/// `end-2`.
///
/// # Safety
/// `end` must point past a writable wide buffer and `spec` at a valid record.
///
/// # Confidence
/// medium - the loop and the prefix/padding logic are literal; the export
/// dispatches the division to the MSL 64-bit helpers `0x802DCFFC` (quotient)
/// and `0x802DCDD8` (remainder) and `src/platform.rs` can only forward a
/// 32-bit result, so the same quotient and remainder are computed inline.
pub unsafe fn fn_802e29d0(hi: u32, lo: u32, end: u32, spec: u32) -> u32 {
    let sp = spec as usize;
    let mut value = ((hi as u64) << 32) | lo as u64;
    let mut cursor = end.wrapping_sub(2);
    platform::write_u16(cursor as usize, 0);
    let conv = platform::read_u16(sp + 6);
    if value == 0
        && platform::read_u32(sp + 0xC) == 0
        && (platform::read_u8(sp + 3) == 0 || conv != 0x6F)
    {
        return cursor;
    }
    let mut base: u32 = 0;
    let mut negative = false;
    if conv == 100 || conv == 0x69 {
        base = 10;
        if (hi as i32) < 0 {
            if value != 0x8000_0000_0000_0000 {
                value = value.wrapping_neg();
            }
            negative = true;
        }
    } else if conv == 0x6F {
        platform::write_u8(sp + 1, 0);
        base = 8;
    } else if conv == 0x75 {
        platform::write_u8(sp + 1, 0);
        base = 10;
    } else if conv == 0x78 || conv == 0x58 {
        platform::write_u8(sp + 1, 0);
        base = 16;
    }
    let b = base as u64;
    let mut digits: i32 = 0;
    let mut before_last: u32;
    loop {
        before_last = cursor;
        let digit = value.wrapping_sub((value / b).wrapping_mul(b));
        value /= b;
        let ch = if (digit as i32) < 10 {
            (digit as i32 + 0x30) as u16
        } else if platform::read_u16(sp + 6) == 0x78 {
            (digit as i32 + 0x57) as u16
        } else {
            (digit as i32 + 0x37) as u16
        };
        cursor = before_last.wrapping_sub(2);
        platform::write_u16(cursor as usize, ch);
        digits += 1;
        if value == 0 {
            break;
        }
    }
    if base == 8 && platform::read_u8(sp + 3) != 0 && platform::read_u16(cursor as usize) != 0x30
    {
        cursor = before_last.wrapping_sub(4);
        platform::write_u16(cursor as usize, 0x30);
        digits += 2;
    }
    if platform::read_u8(sp) == 2 {
        let mut width = platform::read_u32(sp + 8);
        if negative || platform::read_u8(sp + 1) != 0 {
            width = width.wrapping_sub(1);
        }
        if base == 0x10 && platform::read_u8(sp + 3) != 0 {
            width = width.wrapping_sub(2);
        }
        platform::write_u32(sp + 0xC, width);
    }
    if platform::read_i32(sp + 0xC) + (end as i32).wrapping_sub(cursor as i32) / 2 < 0x1FE {
        while digits < platform::read_i32(sp + 0xC) {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x30);
            digits += 1;
        }
        if base == 0x10 && platform::read_u8(sp + 3) != 0 {
            platform::write_u16(cursor.wrapping_sub(2) as usize, platform::read_u16(sp + 6));
            cursor = cursor.wrapping_sub(4);
            platform::write_u16(cursor as usize, 0x30);
        }
        if negative {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x2D);
        } else if platform::read_u8(sp + 1) == 1 {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x2B);
        } else if platform::read_u8(sp + 1) == 2 {
            cursor = cursor.wrapping_sub(2);
            platform::write_u16(cursor as usize, 0x20);
        }
        cursor
    } else {
        0
    }
}

/// Second copy of the decimal rounding helper at [`fn_802e032c`].
///
/// In plain English: identical to [`fn_802e032c`]; the compiler emitted the
/// same 296-byte body twice, once for the `printf` and once for the `scanf`
/// side of the MSL formatter.
///
/// # Original
/// - Ghidra: `FUN_802e30e0` at `0x802e30e0` (296 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e30e0_FUN_802e30e0.c`
/// - Observed: the export is byte-identical to
///   `work/ghidra/main-gb/export/802e032c_FUN_802e032c.c` except for the
///   function name.
///
/// # Arguments
/// * `buf` - `param_1`; the digit block (`+2` exponent, `+4` count, `+5` digits).
/// * `digit` - `param_2`; position to round.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Same stores as [`fn_802e032c`].
///
/// # Safety
/// Same as [`fn_802e032c`].
///
/// # Confidence
/// high - the body is the same instruction sequence as `0x802E032C`.
pub unsafe fn fn_802e30e0(buf: usize, digit: i32) {
    fn_802e032c(buf, digit);
}

/// Read or set the orientation of an MSL stream.
///
/// In plain English: "if the stream is byte-oriented, record the requested
/// orientation and return the request; if it is already oriented, report
/// which one; an unopened stream reports 0."
///
/// # Original
/// - Ghidra: `FUN_802e44f8` at `0x802e44f8` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e44f8_FUN_802e44f8.c`
///
/// # Arguments
/// * `stream` - `param_1`; stream object; `+4` holds the flag word.
/// * `mode` - `param_2`; negative = byte, positive = wide, 0 = query.
///
/// # Returns
/// `mode` after the call, `1` / `-1` when the stream was already wide / byte
/// oriented, or 0 for a null / unopened stream.
///
/// # Side effects
/// Writes `stream+4` (bits `0x1C00000` select the orientation).
///
/// # Safety
/// `stream` must be 0 or a valid stream object.
///
/// # Confidence
/// high - the two-bit orientation field, the three masks and every return are
/// literal; the "orientation" reading is `// inferred:` from the MSL stream
/// layout.
pub unsafe fn fn_802e44f8(stream: usize, mode: i32) -> i32 {
    if stream == 0 {
        return 0;
    }
    let flags = platform::read_u32(stream + 4);
    if (flags >> 0x16) & 7 == 0 {
        return 0;
    }
    let orientation = (flags >> 0x14) & 3;
    if orientation == 0 {
        if mode < 1 {
            if mode < 0 {
                platform::write_u32(stream + 4, (flags & 0xFFCF_FFFF) | 0x0010_0000);
            }
        } else {
            platform::write_u32(stream + 4, (flags & 0xFFCF_FFFF) | 0x0020_0000);
        }
        mode
    } else if orientation == 2 {
        1
    } else if orientation == 1 {
        -1
    } else {
        mode
    }
}

/// Flush a caller buffer through the cache/DMA path, reserving the DMA pool
/// the first time.
///
/// In plain English: "if interrupts are off and the DMA pool is not reserved
/// yet, reserve it; then validate the buffer range and hand the flush to
/// `0x802EC07C`. A busy flag or a failed reservation reports 1."
///
/// # Original
/// - Ghidra: `FUN_802e4570` at `0x802e4570` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4570_FUN_802e4570.c`
///
/// # Arguments
/// * `param_1` - `param_1`; buffer base passed through to the flush.
/// * `param_2` - `param_2`; range length or address passed through.
/// * `param_3` - `param_3`; pointer to the length word; cleared on failure.
/// * `param_4` - `param_4`; flush mode passed through.
///
/// # Returns
/// 0 when the flush was issued, 1 when it was refused.
///
/// # Side effects
/// * may set [`G_8042AFF0`] (DMA pool reserved);
/// * may clear `*param_3`;
/// * forwards to `0x802EC07C`.
///
/// # Safety
/// `param_3` must be a writable word pointer.
///
/// # Confidence
/// medium - the flag tests, the reservation and the two failure returns are
/// literal; the callee roles are `// inferred:` from their arguments.
pub unsafe fn fn_802e4570(param_1: u32, param_2: u32, param_3: usize, param_4: u32) -> u32 {
    let flags = call0_ret(FN_801A9EE0);
    if (flags & 0x2000_0000) == 0 {
        let mut rc: i32 = 0;
        if platform::read_u32(G_8042AFF0) == 0 {
            rc = platform::call1(FN_80227C70, 0xE100) as i32;
            if rc == 0 {
                platform::write_u32(G_8042AFF0, 1);
            }
        }
        if rc != 0 {
            return 1;
        }
        rc = platform::call2(FN_80227CC0, param_2, platform::read_u32(param_3)) as i32;
        if rc != 0 {
            platform::write_u32(param_3, 0);
            return 1;
        }
    }
    platform::call4(FN_802EC07C, param_1, param_2, param_3 as u32, param_4);
    0
}

/// Call the optional flush callback registered at [`G_8042B000`].
///
/// In plain English: "if a callback is registered, call it and return;
/// otherwise do nothing."
///
/// # Original
/// - Ghidra: `FUN_802e4648` at `0x802e4648` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4648_FUN_802e4648.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One indirect call; the export notes the call target could not be resolved
/// as a jump table, so it is emitted as a call.
///
/// # Safety
/// The slot must hold 0 or a valid function pointer.
///
/// # Confidence
/// medium - the null test and the indirect call are literal; the callback's
/// arguments are not visible in the export.
pub unsafe fn fn_802e4648() {
    let callback = platform::read_u32(G_8042B000);
    if callback == 0 {
        return;
    }
    // unknown: the export renders the tail branch as `(**(code **)(...))()`;
    // no arguments are visible at this call site.
    platform::call0(callback as usize);
}

/// `0x8042B028` - `r13 - 0x4658`: the base of the region covered by the third
/// data BAT, tested by [`fn_802e9498`].
///
/// # Original
/// - Ghidra: `*(uint *)(unaff_r13 + -0x4658)` in `802e9498_FUN_802e9498.c`
const G_8042B028: usize = 0x8042_B028;

/// `0x80427478` - the saved `DBAT3U` value written by [`TRK_SPR_CONTEXT`].
///
/// # Original
/// - Ghidra: `DAT_80427478` in `802e9498_FUN_802e9498.c`
const G_80427478: usize = 0x8042_7478;

/// `0x8042B030` - `r13 - 0x4650`: the one-byte "hardware selected" flag cleared
/// by [`fn_802e96e4`].
///
/// # Original
/// - Ghidra: `*(undefined1 *)(unaff_r13 + -18000)` in `802e96e4_FUN_802e96e4.c`
const G_8042B030: usize = 0x8042_B030;

/// `0x803513B8` - the fifteen physical addresses of the code segments that
/// [`fn_802e9498`] writes back to memory.
///
/// # Original
/// - Ghidra: `&DAT_803513b8` in `802e9498_FUN_802e9498.c`
const G_SEGMENT_TABLE_803513B8: usize = 0x8035_13B8;

/// `0x800046E4` - the cached image of the segment table that the flush copies
/// from (`UNK_800046e4` in the export).
///
/// # Original
/// - Ghidra: `&UNK_800046e4` in `802e9498_FUN_802e9498.c`
const G_IMAGE_SEGMENTS_800046E4: usize = 0x8000_46E4;

/// `0x804256D0..0x804256F4` - the MetroTRK handler table installed by
/// [`fn_802e96e4`].
///
/// # Original
/// - Ghidra: `DAT_804256d0 .. DAT_804256f4` in `802e96e4_FUN_802e96e4.c`
const G_TRK_HANDLER_TABLE: usize = 0x8042_56D0;

/// Interior entry points stored into the handler table by [`fn_802e96e4`].
///
/// # Original
/// - Ghidra: `&LAB_802e8968`, `&LAB_802e8970`, `&LAB_802e8994` in
///   `802e96e4_FUN_802e96e4.c`
const LAB_802E8968: usize = 0x802E_8968;
const LAB_802E8970: usize = 0x802E_8970;
const LAB_802E8994: usize = 0x802E_8994;

/// The six status lines printed by [`fn_802e96e4`].
///
/// # Original
/// - Ghidra: `s_Devkit_set_to____ld_803513f8`,
///   `s_MetroTRK___Sizeof_Reply____ld_by_8035140d`,
///   `s_MetroTRK___Set_to_NDEV_hardware_80351432`,
///   `s_MetroTRK___Set_to_UNKNOWN_hardwa_80351453`,
///   `s_MetroTRK___Invalid_hardware_ID_p_8035147e`,
///   `s_MetroTRK___Defaulting_to_GDEV_Ha_803514ad` in
///   `802e96e4_FUN_802e96e4.c`
const STR_DEVKIT_803513F8: usize = 0x8035_13F8;
const STR_REPLY64_8035140D: usize = 0x8035_140D;
const STR_NDEV_80351432: usize = 0x8035_1432;
const STR_UNKNOWN_HW_80351453: usize = 0x8035_1453;
const STR_INVALID_HW_8035147E: usize = 0x8035_147E;
const STR_GDEV_DEFAULT_803514AD: usize = 0x8035_14AD;

/// Re-arm the breakpoint handler: tail-call the 4-byte thunk's target.
///
/// In plain English: "branch to `0x802E9874`, which resets the TRK breakpoint
/// hook."
///
/// # Original
/// - Ghidra: `thunk_FUN_802e9874` at `0x802e942c` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e942c_thunk_FUN_802e9874.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever [`fn_802e9874`] does (it clears the breakpoint hook word).
///
/// # Safety
/// Only valid in the debugger context.
///
/// # Confidence
/// high - the only instruction is a branch to `0x802E9874`, which is already
/// translated as [`fn_802e9874`].
pub unsafe fn fn_802e942c() {
    fn_802e9874();
}

/// Write the three code segments back to physical memory and invalidate their
/// cache lines.
///
/// In plain English: "for each of the fifteen segment entries, if the mask bit
/// is set (except entry 4), translate the cached address to its physical alias
/// when it falls outside the BAT window, copy 256 bytes from the image and
/// flush that line."
///
/// # Original
/// - Ghidra: `FUN_802e9498` at `0x802e9498` (308 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9498_FUN_802e9498.c`
///
/// # Arguments
/// None; the segment table and mask are globals.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// * copies up to 15 x 256 bytes into translated physical addresses;
/// * calls `0x802E8F7C` (byte copy) and `0x802E8E1C` (`dcbst` / `sync` / `icbi`).
///
/// # Safety
/// The segment table must hold valid cached addresses with image copies at
/// `0x800046E4 + address`.
///
/// # Confidence
/// medium - the mask walk, the address translation and the two calls are
/// literal; the low-memory mask alias (`0x44` vs `0x80000044`) is kept exactly
/// as the export prints it, marked `// unknown:`.
pub unsafe fn fn_802e9498() {
    let window = platform::read_u32(G_8042B028);
    let dbat3u = platform::read_u32(G_80427478);
    let in_window =
        window < 0x45 && 0x44 < window.wrapping_add(0x4000) && (dbat3u & 3) != 0;
    // unknown: the export reads either absolute address 0x44 or 0x80000044;
    // the two are the same physical word through different address spaces.
    let mask_addr = if in_window { 0x44 } else { 0x8000_0044 };
    let mask = platform::read_u32(mask_addr);
    let mut index = 0_u32;
    let mut entry = G_SEGMENT_TABLE_803513B8;
    while index < 0xF {
        if (mask & (1 << index)) != 0 && index != 4 {
            let cached = platform::read_u32(entry);
            let mut physical = cached;
            let base = platform::read_u32(G_8042B028);
            if cached < base
                || base.wrapping_add(0x4000) <= cached
                || (platform::read_u32(G_80427478) & 3) == 0
            {
                if cached < 0x0300_0000 {
                    physical = (cached & 0x3FFF_FFFF) | 0x8000_0000;
                } else if cached > 0x0FFF_FFFF && cached < 0x1C00_0000 {
                    physical = (cached & 0x3FFF_FFFF) | 0x9000_0000;
                }
            }
            fn_802e8f7c(
                physical as usize,
                G_IMAGE_SEGMENTS_800046E4.wrapping_add(cached as usize),
                0x100,
            );
            fn_802e8e1c(physical, 0x100);
        }
        index += 1;
        entry += 4;
    }
}

/// Select the MetroTRK target hardware and install the matching handler table.
///
/// In plain English: "announce the requested devkit, then either install the
/// NDEV handler set (id 1), accept id 2 as already correct, or complain and
/// ask the caller to fall back to GDEV."
///
/// # Original
/// - Ghidra: `FUN_802e96e4` at `0x802e96e4` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e96e4_FUN_802e96e4.c`
///
/// # Arguments
/// * `hardware_id` - `param_1`; 1 = NDEV, 2 = GDEV, anything else is invalid.
///
/// # Returns
/// 0 for id 1 / 2, 1 for an invalid id (the caller then uses GDEV).
///
/// # Side effects
/// * clears [`G_8042B030`];
/// * prints one to four status lines through `0x8015F730`;
/// * for id 1, writes nine handler pointers into [`G_TRK_HANDLER_TABLE`].
///
/// # Safety
/// Only valid during debugger boot, before any host message arrives.
///
/// # Confidence
/// high - every string, table slot and return value is literal.
pub unsafe fn fn_802e96e4(hardware_id: u32) -> u32 {
    let _ = platform::call2(FN_8015F730, STR_DEVKIT_803513F8 as u32, hardware_id);
    let _ = platform::call2(FN_8015F730, STR_REPLY64_8035140D as u32, 0x40);
    platform::write_u8(G_8042B030, 0);
    if hardware_id == 2 {
        return 0;
    }
    if hardware_id == 1 {
        let _ = platform::call1(FN_8015F730, STR_NDEV_80351432 as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE, fn_802e892c as *const () as usize as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x04, fn_802e8b80 as *const () as usize as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x08, LAB_802E8968 as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x0C, fn_802e8b0c as *const () as usize as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x10, fn_802e899c as *const () as usize as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x14, fn_802e8a50 as *const () as usize as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x18, LAB_802E8970 as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x1C, LAB_802E8994 as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x20, fn_802e8ac4 as *const () as usize as u32);
        platform::write_u32(G_TRK_HANDLER_TABLE + 0x24, fn_802e8ae8 as *const () as usize as u32);
        return 0;
    }
    let _ = platform::call2(FN_8015F730, STR_UNKNOWN_HW_80351453 as u32, hardware_id);
    let _ = platform::call1(FN_8015F730, STR_INVALID_HW_8035147E as u32);
    let _ = platform::call1(FN_8015F730, STR_GDEV_DEFAULT_803514AD as u32);
    1
}

/// Empty helper the exception prologue calls before saving the context.
///
/// In plain English: "return immediately; the four-byte body is a bare
/// `blr`."
///
/// # Original
/// - Ghidra: `FUN_802e981c` at `0x802e981c` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e981c_FUN_802e981c.c`
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
/// None.
///
/// # Confidence
/// high - the whole function is one `blr`.
pub unsafe fn fn_802e981c() {}

/// Send the `0x90` reply (or the plain status reply) for the current request
/// and release both message buffers.
///
/// In plain English: "take the request buffer, fill the reply through the
/// `0x90` handler or the status handler, send it, then release the reply and
/// request slots."
///
/// # Original
/// - Ghidra: `FUN_802e99c0` at `0x802e99c0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e99c0_FUN_802e99c0.c`
///
/// # Arguments
/// * `command` - `param_1`; `0x90` selects the full status reply, anything
///   else the short one.
///
/// # Returns
/// 0 on success, otherwise the allocation / send error code.
///
/// # Side effects
/// Allocates and releases one of the three TRK message buffers and sends a
/// reply through [`fn_802e98f0`]'s transport.
///
/// # Safety
/// Only valid while a request is in flight.
///
/// # Confidence
/// high - the two-way branch, the two send calls and the two releases are
/// literal.
pub unsafe fn fn_802e99c0(command: u32) -> u32 {
    let mut index: u32 = 0;
    let mut buffer: u32 = 0;
    let mut rc = fn_802ea8a4(
        &mut index as *mut u32 as usize,
        &mut buffer as *mut u32 as usize,
    );
    if rc == 0 {
        if command == 0x90 {
            platform::call1(FN_802ECE28, buffer);
        } else {
            fn_802ed028(buffer as usize);
        }
        let mut reply: u32 = 0;
        rc = fn_802ea094(buffer as usize, &mut reply as *mut u32 as usize) as u32;
        if rc == 0 {
            fn_802ea964(reply);
        }
        fn_802ea964(index);
    }
    rc
}

/// Read one complete reply from the host into a fresh message buffer.
///
/// In plain English: "while the transport has a message, take a buffer, read
/// the 0x40-byte header and then the rest of the payload; return the buffer
/// index, or -1 when nothing arrived."
///
/// # Original
/// - Ghidra: `FUN_802e9d3c` at `0x802e9d3c` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9d3c_FUN_802e9d3c.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// The buffer index of the reply, or `-1` on error / no reply.
///
/// # Side effects
/// * consumes one transport message and allocates a message buffer;
/// * on a short read, releases the buffer again.
///
/// # Safety
/// Only valid while the debugger owns the transport.
///
/// # Confidence
/// medium - the header/payload split and the two releases are literal; the
/// transport read `0x802E98B4` drops its two arguments in the export, so the
/// table slot is called here with the buffer and length.
pub unsafe fn fn_802e9d3c() -> u32 {
    let mut index: u32 = 0xFFFF_FFFF;
    // observed: the export calls `FUN_802e98a0` and tests its result, but that
    // wrapper renders as `void`; the value comes from the installed handler.
    let poll = platform::read_u32(HANDLER_SLOT_804256DC) as usize;
    if (call0_ret(poll) as i32) < 1 {
        return index;
    }
    let mut buffer: u32 = 0;
    let channel = fn_802ea8a4(
        &mut index as *mut u32 as usize,
        &mut buffer as *mut u32 as usize,
    );
    let _ = fn_802ea9b4(buffer as usize, 0);
    let mut header = [0_u32; 0x40];
    let handler = platform::read_u32(HANDLER_SLOT_804256E0) as usize;
    let mut rc = platform::call2(handler, header.as_mut_ptr() as u32, 0x40) as i32;
    if rc == 0 {
        rc = fn_802eacdc(buffer as usize, header.as_mut_ptr() as usize, 0x40);
        if rc == 0 && header[0] > 0x40 {
            let mut payload = [0_u8; 2188];
            let handler = platform::read_u32(HANDLER_SLOT_804256E0) as usize;
            rc = platform::call2(handler, payload.as_mut_ptr() as u32, header[0]) as i32;
            if rc == 0 {
                let _ = fn_802eacdc(
                    buffer as usize,
                    payload.as_mut_ptr() as usize,
                    header[0],
                );
            } else {
                fn_802ea964(index);
                index = 0xFFFF_FFFF;
            }
        }
    } else {
        // observed: the export releases `uVar2`, the allocation result.
        fn_802ea964(channel);
        index = 0xFFFF_FFFF;
    }
    index
}

/// `0x802EC0F4` - the cache-flush trampoline that both [`fn_802ec0e4`] and
/// [`fn_802ec0ec`] forward to.
///
/// # Original
/// - Ghidra: `FUN_802ec0f4` at `0x802ec0f4` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec0f4_FUN_802ec0f4.c`
const FN_802EC0F4: usize = 0x802E_C0F4;

/// `0x802ECE28` - the `0x90` status reply builder (512 bytes). It fills the
/// 0x4E8-byte status message from the saved GPR/SPR frame; Wave 91 did not
/// translate it, so [`fn_802e99c0`] calls it by address.
///
/// # Original
/// - Ghidra: `FUN_802ece28` at `0x802ece28` (512 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ece28_FUN_802ece28.c`
const FN_802ECE28: usize = 0x802E_CE28;

/// The two error lines printed by [`fn_802ea094`].
///
/// # Original
/// - Ghidra: `s_MetroTRK___bad_reply_size__ld_80351520`,
///   `s_MetroTRK___failed_in_RequestSend_8035153f` in
///   `802ea094_FUN_802ea094.c`
const STR_BAD_REPLY_80351520: usize = 0x8035_1520;
const STR_FAILED_REQUEST_8035153F: usize = 0x8035_153F;

/// Transfer a block between the host and the target in `0xD0` / `0xD1` chunks
/// of at most 0x800 bytes.
///
/// In plain English: "loop over the request length, allocate a message buffer,
/// build the `D0` (read) or `D1` (write) header plus its payload chunk, send
/// it, wait for the reply, and copy the acknowledged length back."
///
/// # Original
/// - Ghidra: `FUN_802e9ea0` at `0x802e9ea0` (500 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9ea0_FUN_802e9ea0.c`
///
/// # Arguments
/// * `request` - `param_1`; the request code stored at header `+8`.
/// * `data` - `param_2`; the target buffer (0 is an error).
/// * `len_out` - `param_3`; in: byte count, out: bytes transferred.
/// * `result_out` - `param_4`; receives the low byte of the ack status.
/// * `poll` - `param_5`; 0 = fire-and-forget, else wait for the reply.
/// * `write` - `param_6`; 0 = `D0` (target -> host), 1 = `D1` (host -> target).
///
/// # Returns
/// 0 on success, 2 for a bad argument, otherwise the transport error.
///
/// # Side effects
/// * allocates and releases one message buffer per chunk;
/// * sends `0x40`-byte headers plus the payload through [`fn_802eacdc`];
/// * rewrites `*len_out` and `*result_out`.
///
/// # Safety
/// `data`, `len_out` and `result_out` must be valid for the request.
///
/// # Confidence
/// medium - the chunking, the two header bytes and the ack handling are
/// literal; the export reads the reply through an uninitialised register on
/// the failure path (`unaff_r22`), which is guarded here and marked
/// `// unknown:`.
pub unsafe fn fn_802e9ea0(
    request: u32,
    data: u32,
    len_out: usize,
    result_out: usize,
    poll: i32,
    write: i32,
) -> i32 {
    if data == 0 || platform::read_u32(len_out) == 0 {
        return 2;
    }
    platform::write_u32(result_out, 0);
    let mut offset: u32 = 0;
    let mut rc: i32 = 0;
    let mut stop = false;
    while !stop
        && offset < platform::read_u32(len_out)
        && rc == 0
        && platform::read_u32(result_out) == 0
    {
        let mut header = [0_u32; 0x10];
        let h = header.as_mut_ptr() as usize;
        fn_802e90b8(h, 0, 0x40);
        let mut chunk = 0x800;
        let remaining = platform::read_u32(len_out).wrapping_sub(offset);
        if remaining < 0x801 {
            chunk = remaining;
        }
        platform::write_u8(h + 0x04, if write != 0 { 0xD1 } else { 0xD0 });
        platform::write_u32(h, if write == 0 { chunk + 0x40 } else { 0x40 });
        platform::write_u16(h + 0x0C, chunk as u16);
        platform::write_u32(h + 0x08, request);
        let mut index: u32 = 0;
        let mut channel: u32 = 0;
        let _ = fn_802ea8a4(
            &mut index as *mut u32 as usize,
            &mut channel as *mut u32 as usize,
        );
        rc = fn_802eacdc(channel as usize, h, 0x40);
        if write == 0 && rc == 0 {
            rc = fn_802eacdc(
                channel as usize,
                data.wrapping_add(offset) as usize,
                chunk,
            );
        }
        if rc == 0 {
            if poll == 0 {
                rc = fn_802ea820(channel as usize) as i32;
            } else {
                let mut reply = [0_u32; 2];
                rc = fn_802ea094(channel as usize, reply.as_mut_ptr() as usize);
                // unknown: on failure the export reads an uninitialised
                // register for the reply buffer; no observable behaviour.
                if rc == 0 {
                    let reply_buf = fn_802ea940(reply[0]);
                    let acked = platform::read_u32(reply_buf + 0x1C);
                    let ack_len = platform::read_u16(reply_buf + 0x20) as u32;
                    if write != 0 && ack_len <= chunk {
                        let _ = fn_802ea9b4(reply_buf, 0x40);
                        rc = fn_802eaf10(reply_buf, data.wrapping_add(offset) as usize, ack_len);
                        if rc == 0x302 {
                            rc = 0;
                        }
                    }
                    if ack_len != chunk {
                        stop = true;
                        chunk = ack_len;
                    }
                    platform::write_u32(result_out, acked & 0xFF);
                    fn_802ea964(reply[0]);
                }
            }
        }
        fn_802ea964(index);
        offset = offset.wrapping_add(chunk);
    }
    platform::write_u32(len_out, offset);
    rc
}

/// Send one request and wait for its reply.
///
/// In plain English: "write the header, then read replies until one has a
/// command byte above `0x7F`; release the buffer if the header is short, the
/// type is not `0x80` or the reply code is not 0."
///
/// # Original
/// - Ghidra: `FUN_802ea094` at `0x802ea094` (296 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea094_FUN_802ea094.c`
///
/// # Arguments
/// * `port` - `param_1`; the message buffer to transmit.
/// * `reply_index_out` - `param_2`; receives the reply buffer index, or -1.
///
/// # Returns
/// 0 on success, `0x800` when the request failed.
///
/// # Side effects
/// * sends the buffer through [`fn_802ea820`];
/// * consumes replies with [`fn_802e9d3c`] and releases rejected ones;
/// * prints `"MetroTRK : bad reply size"` / `"failed in RequestSend"`.
///
/// # Safety
/// Only valid while the transport is owned by the debugger.
///
/// # Confidence
/// medium - the poll loop, the four header checks and both prints are literal;
/// the export drops the index argument of `0x802EA940` on the retry path, so
/// the value just stored in `reply_index_out` is used here.
pub unsafe fn fn_802ea094(port: usize, reply_index_out: usize) -> i32 {
    let mut rc = fn_802ea820(port) as i32;
    if rc == 0 {
        let mut buf: usize;
        loop {
            let mut id: i32;
            loop {
                id = fn_802e9d3c() as i32;
                platform::write_u32(reply_index_out, id as u32);
                if id != -1 {
                    break;
                }
            }
            // observed: `FUN_802ea940()` without the index argument.
            buf = fn_802ea940(platform::read_u32(reply_index_out));
            let _ = fn_802ea9b4(buf, 0);
            if platform::read_u8(buf + 0x10) > 0x7F {
                break;
            }
            fn_802e9e34(id as u32);
            platform::write_u32(reply_index_out, 0xFFFF_FFFF);
        }
        if platform::read_u32(reply_index_out) != 0xFFFF_FFFF {
            let mut bad = platform::read_u32(buf + 4) < 0x40;
            if bad {
                let _ = platform::call1(FN_8015F730, STR_BAD_REPLY_80351520 as u32);
            } else if platform::read_u8(buf + 0x10) != 0x80 || platform::read_u8(buf + 0x14) != 0 {
                bad = true;
            }
            if bad {
                fn_802ea964(platform::read_u32(reply_index_out));
                platform::write_u32(reply_index_out, 0xFFFF_FFFF);
            }
        }
    }
    if platform::read_u32(reply_index_out) == 0xFFFF_FFFF {
        let _ = platform::call1(FN_8015F730, STR_FAILED_REQUEST_8035153F as u32);
        rc = 0x800;
    }
    rc
}

/// `D2` request: read one register of the stopped context.
///
/// In plain English: "send a `D2` header naming the string to read and the
/// register number, then store the 32-bit reply value."
///
/// # Original
/// - Ghidra: `FUN_802ea1bc` at `0x802ea1bc` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea1bc_FUN_802ea1bc.c`
///
/// # Arguments
/// * `request` - `param_1`; NUL-terminated request string, usually a register
///   name.
/// * `reg` - `param_2`; low byte of the register selector.
/// * `out` - `param_3`; receives the 32-bit value at reply `+0x14`.
/// * `status_out` - `param_4`; receives the ack status at reply `+0x1C`.
///
/// # Returns
/// 0 on success, otherwise the transport error.
///
/// # Side effects
/// Allocates and releases one message buffer and writes two output words.
///
/// # Safety
/// `request` must be a NUL-terminated string and both outputs writable.
///
/// # Confidence
/// medium - the header bytes, the length computation and both reply reads are
/// literal; the register/string roles are `// inferred:` from the `D2` handler
/// Wave 82 documented.
pub unsafe fn fn_802ea1bc(request: u32, reg: u32, out: usize, status_out: usize) -> i32 {
    let mut header = [0_u32; 0x10];
    let h = header.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    platform::write_u32(out, 0);
    platform::write_u8(h + 0x04, 0xD2);
    platform::write_u32(h, fn_802e9e84(request as usize) as u32 + 0x41);
    platform::write_u8(h + 0x08, reg as u8);
    platform::write_u16(h + 0x0C, (fn_802e9e84(request as usize) as u32 + 1) as u16);
    let mut index: u32 = 0;
    let mut channel: u32 = 0;
    let _ = fn_802ea8a4(
        &mut index as *mut u32 as usize,
        &mut channel as *mut u32 as usize,
    );
    let mut rc = fn_802eacdc(channel as usize, h, 0x40);
    if rc == 0 {
        let count = fn_802e9e84(request as usize) as u32 + 1;
        rc = fn_802eacdc(channel as usize, request as usize, count);
    }
    if rc == 0 {
        platform::write_u32(status_out, 0);
        let mut reply = [0_u32; 2];
        rc = fn_802ea094(channel as usize, reply.as_mut_ptr() as usize);
        let mut reply_buf: usize = 0;
        if rc == 0 {
            reply_buf = fn_802ea940(reply[0]);
        }
        platform::write_u32(status_out, platform::read_u32(reply_buf + 0x1C));
        platform::write_u32(out, platform::read_u32(reply_buf + 0x14));
        fn_802ea964(reply[0]);
    }
    fn_802ea964(index);
    rc
}

/// `D3` request: write one register of the stopped context.
///
/// In plain English: "send a `D3` header whose payload is the 32-bit value,
/// then store the ack status from the reply."
///
/// # Original
/// - Ghidra: `FUN_802ea2d4` at `0x802ea2d4` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea2d4_FUN_802ea2d4.c`
///
/// # Arguments
/// * `value` - `param_1`; the word written to the register.
/// * `status_out` - `param_2`; receives the ack status at reply `+0x1C`.
///
/// # Returns
/// 0 on success, otherwise the transport error.
///
/// # Side effects
/// Allocates and releases one message buffer and writes one output word.
///
/// # Safety
/// `status_out` must be writable.
///
/// # Confidence
/// medium - the `0xD3` command, the `0x40` length and the reply read are
/// literal; the register/value roles are `// inferred:`.
pub unsafe fn fn_802ea2d4(value: u32, status_out: usize) -> i32 {
    let mut header = [0_u32; 0x10];
    let h = header.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    platform::write_u8(h + 0x04, 0xD3);
    platform::write_u32(h, 0x40);
    platform::write_u32(h + 0x08, value);
    let mut index: u32 = 0;
    let mut channel: u32 = 0;
    let mut rc = fn_802ea8a4(
        &mut index as *mut u32 as usize,
        &mut channel as *mut u32 as usize,
    ) as i32;
    if rc == 0 {
        rc = fn_802eacdc(channel as usize, h, 0x40);
    }
    if rc == 0 {
        platform::write_u32(status_out, 0);
        let mut reply = [0_u32; 2];
        rc = fn_802ea094(channel as usize, reply.as_mut_ptr() as usize);
        if rc == 0 {
            let reply_buf = fn_802ea940(reply[0]);
            platform::write_u32(status_out, platform::read_u32(reply_buf + 0x1C));
        }
        fn_802ea964(reply[0]);
    }
    fn_802ea964(index);
    rc
}

/// `D4` request: write one memory word with a byte count and a flag.
///
/// In plain English: "send a `D4` header carrying the address, the current
/// word and the byte count, then store the new word and the ack status the
/// reply carries."
///
/// # Original
/// - Ghidra: `FUN_802ea3b8` at `0x802ea3b8` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea3b8_FUN_802ea3b8.c`
///
/// # Arguments
/// * `address` - `param_1`; target address.
/// * `word` - `param_2`; in: current word, out: the reply word at `+0x24`.
/// * `count` - `param_3`; byte count stored at header `+0x10`.
/// * `status_out` - `param_4`; receives the ack status at reply `+0x1C`.
///
/// # Returns
/// 0 on success, otherwise the transport error.
///
/// # Side effects
/// Allocates and releases one message buffer and writes both output words.
///
/// # Safety
/// `word` and `status_out` must be writable.
///
/// # Confidence
/// medium - the `0xD4` command, the two reply reads and the failure stores are
/// literal; the address/word roles are `// inferred:` from the request body.
pub unsafe fn fn_802ea3b8(address: u32, word: usize, count: u8, status_out: usize) -> i32 {
    let mut header = [0_u32; 0x10];
    let h = header.as_mut_ptr() as usize;
    fn_802e90b8(h, 0, 0x40);
    platform::write_u8(h + 0x04, 0xD4);
    platform::write_u32(h, 0x40);
    platform::write_u32(h + 0x0C, platform::read_u32(word));
    platform::write_u32(h + 0x08, address);
    platform::write_u8(h + 0x10, count);
    let mut index: u32 = 0;
    let mut channel: u32 = 0;
    let mut rc = fn_802ea8a4(
        &mut index as *mut u32 as usize,
        &mut channel as *mut u32 as usize,
    ) as i32;
    if rc == 0 {
        rc = fn_802eacdc(channel as usize, h, 0x40);
    }
    if rc == 0 {
        platform::write_u32(status_out, 0);
        platform::write_u32(word, 0xFFFF_FFFF);
        let mut reply = [0_u32; 2];
        rc = fn_802ea094(channel as usize, reply.as_mut_ptr() as usize);
        if rc == 0 {
            let reply_buf = fn_802ea940(reply[0]);
            if reply_buf != 0 {
                platform::write_u32(status_out, platform::read_u32(reply_buf + 0x1C));
                platform::write_u32(word, platform::read_u32(reply_buf + 0x24));
            }
        }
        fn_802ea964(reply[0]);
    }
    fn_802ea964(index);
    rc
}

/// Flush the cache range prepared by the caller (first entry point).
///
/// In plain English: "forward the four arguments to `0x802EC0F4`."
///
/// # Original
/// - Ghidra: `FUN_802ec0e4` at `0x802ec0e4` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec0e4_FUN_802ec0e4.c`
///
/// # Arguments
/// * `a`..`d` - `param_1`..`param_4`; forwarded unchanged.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Whatever `0x802EC0F4` does; it is still untranslated (its fifth argument is
/// undefined at the call site).
///
/// # Safety
/// Same requirements as `0x802EC0F4`.
///
/// # Confidence
/// medium - the export shows only the forward; that the arguments pass through
/// is `// inferred:` from the caller `fn_802ec014`, which supplies four.
pub unsafe fn fn_802ec0e4(a: u32, b: u32, c: u32, d: u32) {
    platform::call4(FN_802EC0F4, a, b, c, d);
}

/// Flush the cache range prepared by the caller (second entry point).
///
/// In plain English: identical to [`fn_802ec0e4`]; the compiler emitted the
/// same 8-byte forward twice.
///
/// # Original
/// - Ghidra: `FUN_802ec0ec` at `0x802ec0ec` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec0ec_FUN_802ec0ec.c`
///
/// # Arguments
/// * `a`..`d` - `param_1`..`param_4`; forwarded unchanged.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Same as [`fn_802ec0e4`].
///
/// # Safety
/// Same as [`fn_802ec0e4`].
///
/// # Confidence
/// medium - the body is the same eight-byte forward as `0x802EC0E4`.
pub unsafe fn fn_802ec0ec(a: u32, b: u32, c: u32, d: u32) {
    platform::call4(FN_802EC0F4, a, b, c, d);
}

// ---------------------------------------------------------------------------
// Wave 95 - MSL math kernels and the remaining TRK transfers.
// ---------------------------------------------------------------------------
//
// The MSL section below resolves two register bases defined in
// `docs/GLOSSARY.md`: `r2 = 0x80433080` (the `.sdata2` constant pool) and
// `r13 = 0x8042F680` (the small-data area). Ghidra renders every constant-pool
// load as `*(double *)(unaff_r2 - N)`; `sda2`/`sda13` turn those offsets back
// into absolute addresses so the bodies stay literal instead of embedding
// guessed numeric values. The values themselves remain in the shipped image,
// which is exactly where the original reads them from.

/// Resolve a `r2`-relative `.sdata2` constant address (`r2 = 0x80433080`).
const fn sda2(off: i32) -> usize {
    0x8043_3080usize.wrapping_add_signed(off as isize)
}

/// Resolve a `r13`-relative small-data address (`r13 = 0x8042F680`).
const fn sda13(off: i32) -> usize {
    0x8042_F680usize.wrapping_add_signed(off as isize)
}

/// Reads a big-endian `f64` from two consecutive words at `addr`.
///
/// # Original
/// Host-side helper; models the `lfd` pair that the MSL math kernels use for
/// their `.sdata2` constants.
///
/// # Safety
/// `addr..addr+8` must be readable in the target's address space.
unsafe fn read_f64(addr: usize) -> f64 {
    let hi = platform::read_u32(addr) as u64;
    let lo = platform::read_u32(addr + 4) as u64;
    f64::from_bits((hi << 32) | lo)
}

/// Writes a big-endian `f64` to two consecutive words at `addr`.
///
/// # Original
/// Host-side helper; models the `stfd` pair used for the `modf` output word.
///
/// # Safety
/// `addr..addr+8` must be writable in the target's address space.
unsafe fn write_f64(addr: usize, value: f64) {
    let bits = value.to_bits();
    platform::write_u32(addr, (bits >> 32) as u32);
    platform::write_u32(addr + 4, bits as u32);
}

/// Copies the sign of `sign_of` onto the magnitude of `magnitude`.
///
/// This is the MSL `copysign` kernel: only the two high words are touched, so
/// a NaN payload in the low word survives unchanged.
///
/// # Original
/// - Ghidra: `FUN_802e7f90` at `0x802e7f90` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e7f90_FUN_802e7f90.c`
///
/// # Arguments
/// * `magnitude` - `param_1`; supplies the magnitude and the low 32 bits.
/// * `sign_of` - `param_2`; supplies only bit 63 (the sign).
///
/// # Returns
/// `magnitude` with `sign_of`'s sign bit.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the two masks and the `CONCAT44` are literal.
pub unsafe fn fn_802e7f90(magnitude: f64, sign_of: f64) -> f64 {
    let m = magnitude.to_bits();
    let s = sign_of.to_bits();
    f64::from_bits((m & 0x7FFF_FFFF_FFFF_FFFF) | (s & 0x8000_0000_0000_0000))
}

/// Rounds `x` upward to an integral value (MSL `ceil`).
///
/// In plain English: "if `x` already has no fraction, return it; otherwise
/// clear the fraction bits in the direction that increases the value."
///
/// # Original
/// - Ghidra: `FUN_802e7e44` at `0x802e7e44` (328 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e7e44_FUN_802e7e44.c`
///
/// # Arguments
/// * `x` - `param_1`; the value to round.
///
/// # Returns
/// The smallest integer not less than `x`; infinities and NaNs pass through.
///
/// # Side effects
/// None. The `0.5-eps` tests only select the `-0.0` / `1.0` edge results.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the bit masks and the two boundary constants at `r2-0x3528` /
/// `r2-0x3530` are literal; the `ceil` role is `// inferred:` from the
/// positive-add / negative-truncate split, which is the exact twin of the
/// `floor` body at [`fn_802e8084`].
pub unsafe fn fn_802e7e44(x: f64) -> f64 {
    let mut bits = x.to_bits();
    let mut hi = (bits >> 32) as u32;
    let mut lo = bits as u32;
    let raw_exp = (hi >> 20) & 0x7FF;
    let exponent = (raw_exp as i32).wrapping_sub(0x3FF);
    let bias = read_f64(sda2(-0x3530));
    let edge = read_f64(sda2(-0x3528));

    if exponent < 0x34 {
        if exponent < 0x14 {
            if exponent < 0 {
                if edge < bias + x {
                    if x < 0.0 {
                        hi = 0x8000_0000;
                        lo = 0;
                    } else if bits != 0 {
                        hi = 0x3FF0_0000;
                        lo = 0;
                    }
                }
            } else {
                let mask = 0x000F_FFFF_u32 >> (exponent & 0x3F);
                if lo == 0 && (hi & mask) == 0 {
                    return x;
                }
                if edge < bias + x {
                    if (hi as i32) > 0 {
                        hi = hi.wrapping_add(0x0010_0000 >> (exponent & 0x3F));
                    }
                    hi &= !mask;
                    lo = 0;
                }
            }
        } else {
            let mask = 0xFFFF_FFFF_u32 >> (raw_exp as i32 - 0x413);
            if (lo & mask) == 0 {
                return x;
            }
            if edge < bias + x {
                let mut low = lo;
                if (hi as i32) > 0 {
                    if exponent == 0x14 {
                        hi = hi.wrapping_add(1);
                    } else {
                        low = lo.wrapping_add(1 << (0x34 - exponent));
                        if low < lo {
                            hi = hi.wrapping_add(1);
                        }
                    }
                }
                lo = low & !mask;
            }
        }
    } else if exponent == 0x400 {
        return x + x;
    }

    bits = ((hi as u64) << 32) | lo as u64;
    f64::from_bits(bits)
}

/// Rounds `x` downward to an integral value (MSL `floor`).
///
/// In plain English: "if `x` already has no fraction, return it; otherwise
/// clear the fraction bits in the direction that decreases the value."
///
/// # Original
/// - Ghidra: `FUN_802e8084` at `0x802e8084` (332 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8084_FUN_802e8084.c`
///
/// # Arguments
/// * `x` - `param_1`; the value to round.
///
/// # Returns
/// The largest integer not greater than `x`; infinities and NaNs pass through.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the body is the exact twin of [`fn_802e7e44`] with the sign tests
/// swapped; every mask and constant is literal.
pub unsafe fn fn_802e8084(x: f64) -> f64 {
    let mut bits = x.to_bits();
    let mut hi = (bits >> 32) as u32;
    let mut lo = bits as u32;
    let raw_exp = (hi >> 20) & 0x7FF;
    let exponent = (raw_exp as i32).wrapping_sub(0x3FF);
    let bias = read_f64(sda2(-0x3518));
    let edge = read_f64(sda2(-0x3510));

    if exponent < 0x34 {
        if exponent < 0x14 {
            if exponent < 0 {
                if edge < bias + x {
                    if x < 0.0 {
                        if bits != 0 {
                            hi = 0xBFF0_0000;
                            lo = 0;
                        }
                    } else {
                        hi = 0;
                        lo = 0;
                    }
                }
            } else {
                let mask = 0x000F_FFFF_u32 >> (exponent & 0x3F);
                if lo == 0 && (hi & mask) == 0 {
                    return x;
                }
                if edge < bias + x {
                    if x < 0.0 {
                        hi = hi.wrapping_add(0x0010_0000 >> (exponent & 0x3F));
                    }
                    hi &= !mask;
                    lo = 0;
                }
            }
        } else {
            let mask = 0xFFFF_FFFF_u32 >> (raw_exp as i32 - 0x413);
            if (lo & mask) == 0 {
                return x;
            }
            if edge < bias + x {
                let mut low = lo;
                if x < 0.0 {
                    if exponent == 0x14 {
                        hi = hi.wrapping_add(1);
                    } else {
                        low = lo.wrapping_add(1 << (0x34 - exponent));
                        if low < lo {
                            hi = hi.wrapping_add(1);
                        }
                    }
                }
                lo = low & !mask;
            }
        }
    } else if exponent == 0x400 {
        return x + x;
    }

    bits = ((hi as u64) << 32) | lo as u64;
    f64::from_bits(bits)
}

/// Splits `x` into a normalized fraction and a power-of-two exponent.
///
/// In plain English: "return `m` in `[0.5,1)` and store `e` such that
/// `x == m * 2^e`; subnormals are pre-scaled by `2^54` first."
///
/// # Original
/// - Ghidra: `FUN_802e81d4` at `0x802e81d4` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e81d4_FUN_802e81d4.c`
///
/// # Arguments
/// * `x` - `param_1`; the value to split.
/// * `out_exp` - `param_2`; receives the exponent. Left at 0 for `0`, Inf and
///   NaN.
///
/// # Returns
/// The normalized fraction, or `x` unchanged when it is not finite.
///
/// # Side effects
/// Writes the exponent word.
///
/// # Safety
/// `out_exp` must point at a writable 32-bit word.
///
/// # Confidence
/// medium - the two masks, the `-0x36` subnormal bias and the `0x3fe` exponent
/// rebase are literal; the `frexp` role is `// inferred:` from the classic
/// `[0.5,1)` normalization sequence.
pub unsafe fn fn_802e81d4(x: f64, out_exp: usize) -> f64 {
    platform::write_u32(out_exp, 0);
    let mut value = x;
    let mut bits = value.to_bits();
    let mut hi = (bits >> 32) as u32;
    let mut abs = hi & 0x7FFF_FFFF;

    if abs < 0x7FF0_0000 && bits & 0x7FFF_FFFF_FFFF_FFFF != 0 {
        if abs < 0x0010_0000 {
            value *= read_f64(sda2(-0x3508));
            bits = value.to_bits();
            hi = (bits >> 32) as u32;
            abs = hi & 0x7FFF_FFFF;
            platform::write_u32(out_exp, (-0x36_i32) as u32);
        }
        let mantissa = f64::from_bits(
            ((((hi as u64) << 32) | (bits & 0xFFFF_FFFF)) & 0x800F_FFFF_FFFF_FFFF)
                | 0x3FE0_0000_0000_0000,
        );
        let old = platform::read_u32(out_exp) as i32;
        let new_exp = ((abs as i32) >> 20).wrapping_add(old).wrapping_sub(0x3FE);
        platform::write_u32(out_exp, new_exp as u32);
        return mantissa;
    }

    x
}

/// `0x802DF42C` - the MSL floating-point environment query called by
/// [`fn_802e825c`]. It is outside this bucket and not translated here.
///
/// # Original
/// - Ghidra: `FUN_802df42c` at `0x802df42c`
/// - Evidence: `work/ghidra/main-gb/export/802df42c_FUN_802df42c.c`
const FN_802DF42C: usize = 0x802D_F42C;

/// Scales `x` by `2^exp` (MSL `ldexp` / `scalbn`).
///
/// In plain English: "fold `exp` into the exponent field; when the result
/// would underflow or overflow, multiply by a power-of-two constant instead."
///
/// # Original
/// - Ghidra: `FUN_802e825c` at `0x802e825c` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e825c_FUN_802e825c.c`
///
/// # Arguments
/// * `x` - `param_1`; the significand.
/// * `exp` - `param_2`; the power of two to apply.
///
/// # Returns
/// `x * 2^exp`, with the MSL overflow / underflow constants applied when the
/// result leaves the normal range.
///
/// # Side effects
/// None beyond the call to [`FN_802DF42C`]. The original returns early when that
/// query reports fewer than three active FPU modes.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - every constant load, exponent rebase and overflow branch is
/// literal; the `ldexp` role and the [`FN_802DF42C`] guard are `// inferred:`
/// from the call shape and the `-50000` / `0xc351` clamping constants.
pub unsafe fn fn_802e825c(x: f64, exp: i32) -> f64 {
    let mut result = x;
    if (call0_ret(FN_802DF42C) as i32) < 3 {
        return result;
    }
    if read_f64(sda2(-0x3500)) == result {
        return result;
    }

    let mut bits = result.to_bits();
    let mut hi = (bits >> 32) as u32;
    let mut raw_exp = (hi >> 20) & 0x7FF;
    let mut sign_source = hi;
    let mut scaled = result;

    if raw_exp == 0 {
        if result == 0.0 {
            return result;
        }
        scaled *= read_f64(sda2(-0x34F8));
        bits = scaled.to_bits();
        hi = (bits >> 32) as u32;
        raw_exp = ((hi >> 20) & 0x7FF).wrapping_sub(0x36);
        sign_source = hi;
        if exp < -50000 {
            return read_f64(sda2(-0x34F0)) * scaled;
        }
    }

    let low = bits & 0xFFFF_FFFF;
    if raw_exp == 0x7FF {
        result = scaled + scaled;
    } else {
        let target = raw_exp as i32 + exp;
        if target < 0x7FF {
            if target < 1 {
                if target < -0x35 {
                    if exp < 0xC351 {
                        result = fn_802e7f90(read_f64(sda2(-0x34F0)), scaled);
                        result = read_f64(sda2(-0x34F0)) * result;
                    } else {
                        result = fn_802e7f90(read_f64(sda2(-0x34E8)), scaled);
                        result = read_f64(sda2(-0x34E8)) * result;
                    }
                } else {
                    let rebuilt = (sign_source & 0x800F_FFFF)
                        | ((target + 0x36) as u32).wrapping_mul(0x0010_0000);
                    result = f64::from_bits(((rebuilt as u64) << 32) | low);
                    result *= read_f64(sda2(-0x34E0));
                }
            } else {
                let rebuilt = (sign_source & 0x800F_FFFF)
                    | (target as u32).wrapping_mul(0x0010_0000);
                result = f64::from_bits(((rebuilt as u64) << 32) | low);
            }
        } else {
            result = fn_802e7f90(read_f64(sda2(-0x34E8)), scaled);
            result = read_f64(sda2(-0x34E8)) * result;
        }
    }
    result
}

/// Splits `x` into its integral and fractional parts (MSL `modf`).
///
/// In plain English: "write the signed integral part through `out_int` and
/// return the fraction with the same sign."
///
/// # Original
/// - Ghidra: `FUN_802e83c8` at `0x802e83c8` (252 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e83c8_FUN_802e83c8.c`
///
/// # Arguments
/// * `x` - `param_1`; the value to split.
/// * `out_int` - `param_2`; receives the integral part as two big-endian words.
///
/// # Returns
/// `x - integral`, i.e. the fraction with `x`'s sign.
///
/// # Side effects
/// Writes 8 bytes at `out_int`.
///
/// # Safety
/// `out_int` must be 8-byte writable.
///
/// # Confidence
/// high - the exponent cuts, the register-level masks and the two-word store of
/// the integral part are literal.
pub unsafe fn fn_802e83c8(x: f64, out_int: usize) -> f64 {
    let bits = x.to_bits();
    let hi = (bits >> 32) as u32;
    let lo = bits as u32;
    let raw_exp = (hi >> 20) & 0x7FF;
    let exponent = (raw_exp as i32).wrapping_sub(0x3FF);

    if exponent < 0x14 {
        if exponent < 0 {
            platform::write_u32(out_int, hi & 0x8000_0000);
            platform::write_u32(out_int + 4, 0);
            x
        } else {
            let mask = 0x000F_FFFF_u32 >> (exponent & 0x3F);
            if lo == 0 && (hi & mask) == 0 {
                write_f64(out_int, x);
                f64::from_bits(((hi & 0x8000_0000) as u64) << 32)
            } else {
                platform::write_u32(out_int, hi & !mask);
                platform::write_u32(out_int + 4, 0);
                x - read_f64(out_int)
            }
        }
    } else if exponent < 0x34 {
        let mask = 0xFFFF_FFFF_u32 >> (raw_exp as i32 - 0x413);
        if (lo & mask) == 0 {
            write_f64(out_int, x);
            f64::from_bits(((hi & 0x8000_0000) as u64) << 32)
        } else {
            platform::write_u32(out_int, hi);
            platform::write_u32(out_int + 4, lo & !mask);
            x - read_f64(out_int)
        }
    } else {
        write_f64(out_int, x);
        f64::from_bits(((hi & 0x8000_0000) as u64) << 32)
    }
}

/// Evaluates the inverse-tangent tail polynomial for a small argument.
///
/// In plain English: "return `atan(x)` corrected by the caller-supplied
/// residual `y`, using the odd minimax series in `x^2`."
///
/// # Original
/// - Ghidra: `FUN_802e613c` at `0x802e613c` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e613c_FUN_802e613c.c`
///
/// # Arguments
/// * `x` - `param_1`; the reduced argument.
/// * `y` - `param_2`; the residual the caller peeled off during reduction.
///
/// # Returns
/// The corrected angle in radians.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the series, the `0x3fd33333` / `0x3fe90001` cuts and the two
/// compensation forms are literal; the inverse-tangent role is `// inferred:`
/// from the `pi/2` constant and the odd polynomial.
pub unsafe fn fn_802e613c(x: f64, y: f64) -> f64 {
    let abs = ((x.to_bits() >> 32) as u32) & 0x7FFF_FFFF;
    if abs < 0x3E40_0000 && (x as i32) == 0 {
        return read_f64(sda2(-0x3650));
    }

    let x2 = x * x;
    let poly = x2
        * (read_f64(sda2(-0x3648))
            + x2
                * (read_f64(sda2(-0x3640))
                    + x2
                        * (read_f64(sda2(-0x3638))
                            + x2
                                * (read_f64(sda2(-0x3630))
                                    + x2
                                        * (read_f64(sda2(-0x3628))
                                            + read_f64(sda2(-0x3620)) * x2)))));
    if abs < 0x3FD3_3333 {
        read_f64(sda2(-0x3650)) - (read_f64(sda2(-0x3618)) * x2 - (x2 * poly - x * y))
    } else {
        let correction = if abs < 0x3FE9_0001 {
            f64::from_bits((abs.wrapping_sub(0x0020_0000) as u64) << 32)
        } else {
            read_f64(sda2(-0x3610))
        };
        (read_f64(sda2(-0x3650)) - correction)
            - ((read_f64(sda2(-0x3618)) * x2 - correction) - (x2 * poly - x * y))
    }
}

/// Evaluates the `atan` polynomial and either returns it or applies the
/// caller's residual correction.
///
/// In plain English: "if `x` is not tiny, form the odd series and add or
/// subtract the `y` correction selected by `mode`."
///
/// # Original
/// - Ghidra: `FUN_802e7904` at `0x802e7904` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e7904_FUN_802e7904.c`
///
/// # Arguments
/// * `x` - `param_1`; the reduced angle.
/// * `y` - `param_2`; the residual used when `mode != 0`.
/// * `mode` - `param_3`; 0 selects the ordinary series, non-zero the corrected
///   form.
///
/// # Returns
/// The angle approximation.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the threshold, the five coefficients and both branches are
/// literal; the `atan` role is `// inferred:` from the `x + x^3 * P(x^2)`
/// shape.
pub unsafe fn fn_802e7904(x: f64, y: f64, mode: u32) -> f64 {
    let mut value = x;
    let abs = ((x.to_bits() >> 32) as u32) & 0x7FFF_FFFF;
    if abs > 0x3E3F_FFFF || (x as i32) != 0 {
        let x2 = x * x;
        let x3 = x2 * x;
        let poly = read_f64(sda2(-0x35C8))
            + x2 * (read_f64(sda2(-0x35C0))
                + x2 * (read_f64(sda2(-0x35B8))
                    + x2 * (read_f64(sda2(-0x35B0)) + read_f64(sda2(-0x35A8)) * x2)));
        if mode == 0 {
            value = x + x3 * (read_f64(sda2(-0x35A0)) + x2 * poly);
        } else {
            value = x
                - ((x2 * (read_f64(sda2(-0x3598)) * y - x3 * poly) - y)
                    - read_f64(sda2(-0x35A0)) * x3);
        }
    }
    value
}

/// Forms the `2^52 + n` double the MSL kernels use to convert a signed integer
/// to `f64`, then subtracts the `2^52` bias the caller supplies.
///
/// # Original
/// Host-side helper; models the repeated
/// `(double)CONCAT44(0x43300000, v ^ 0x80000000)` idiom.
///
/// # Safety
/// None.
#[inline]
unsafe fn signed_to_f64_bits(value: i32) -> f64 {
    f64::from_bits(0x4330_0000_0000_0000 | ((value as u32 ^ 0x8000_0000) as u64))
}

/// Computes `log(1+x)` for a normalised argument (MSL `log1p` kernel).
///
/// In plain English: "reduce `x` to `m-1` near zero, evaluate the `atanh`
/// series for the small / large mantissa cases, and add back the exponent
/// scaled `ln 2` when one was peeled off."
///
/// # Original
/// - Ghidra: `FUN_802e51dc` at `0x802e51dc` (692 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e51dc_FUN_802e51dc.c`
///
/// # Arguments
/// * `x` - `param_1`; the reduced argument.
///
/// # Returns
/// `log(1+x)`, with the MSL `errno` / `-inf` handling for the zero and
/// negative-zero inputs.
///
/// # Side effects
/// Writes [`G_ERRNO`] (`0x21`, `EDOM`) for a negative input.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - all ten coefficient loads, the two `0x43300000` rebuilds and the
/// `0x6147a` / `0x6b851` symmetric range test are literal; the `log1p` role is
/// `// inferred:` from the `atanh` series and the `ln 2` add-back.
pub unsafe fn fn_802e51dc(x: f64) -> f64 {
    let mut shift = 0_i32;
    let mut value = x;
    let mut bits = value.to_bits();
    let mut hi = (bits >> 32) as u32;

    if (hi as i32) < 0x0010_0000 {
        if value == 0.0 {
            return read_f64(sda2(-0x3868)) / read_f64(sda13(-0x4678));
        }
        if value < 0.0 {
            platform::write_u32(G_ERRNO, 0x21);
            return (value - value) / read_f64(sda13(-0x4678));
        }
        shift = -0x36;
        value *= read_f64(sda2(-0x3860));
        bits = value.to_bits();
        hi = (bits >> 32) as u32;
    }

    if (hi as i32) < 0x7FF0_0000 {
        let exponent = (hi as i32) >> 20;
        let mantissa = hi & 0x000F_FFFF;
        let adjusted = mantissa.wrapping_add(0x0009_5F64);
        let rebuilt = mantissa | ((adjusted & 0x0010_0000) ^ 0x3FF0_0000);
        let local = f64::from_bits(
            ((rebuilt as u64) << 32) | (bits & 0xFFFF_FFFF),
        ) - read_f64(sda2(-0x3858));
        let power = shift + exponent - 0x3FF + (adjusted >> 20) as i32;

        if (mantissa.wrapping_add(2) & 0x000F_FFFF) < 3 {
            let mut result = read_f64(sda13(-0x4678));
            if local == result {
                if power != 0 {
                    let scaled =
                        signed_to_f64_bits(power) - read_f64(sda2(-0x37F0));
                    result = read_f64(sda2(-0x3850)) * scaled
                        + read_f64(sda2(-0x3848)) * scaled;
                }
            } else {
                result = (read_f64(sda2(-0x3840)) - read_f64(sda2(-0x3838)) * local)
                    * local
                    * local;
                if power == 0 {
                    result = local - result;
                } else {
                    let scaled =
                        signed_to_f64_bits(power) - read_f64(sda2(-0x37F0));
                    result = read_f64(sda2(-0x3850)) * scaled
                        - ((result - read_f64(sda2(-0x3848)) * scaled) - local);
                }
            }
            result
        } else {
            let t = local / (read_f64(sda2(-0x3830)) + local);
            let t2 = t * t;
            let t4 = t2 * t2;
            let scaled = signed_to_f64_bits(power) - read_f64(sda2(-0x37F0));
            let poly = t2
                * (read_f64(sda2(-0x3828))
                    + t4 * (read_f64(sda2(-0x3820))
                        + t4 * (read_f64(sda2(-0x3818)) + read_f64(sda2(-0x3810)) * t4)))
                + t4
                    * (read_f64(sda2(-0x3808))
                        + t4 * (read_f64(sda2(-0x3800))
                            + read_f64(sda2(-0x37F8)) * t4));
            if (((mantissa.wrapping_sub(0x0006_147A)) | 0x0006_B851_u32.wrapping_sub(mantissa))
                as i32)
                < 1
            {
                if power == 0 {
                    local - t * (local - poly)
                } else {
                    read_f64(sda2(-0x3850)) * scaled
                        - ((t * (local - poly) - read_f64(sda2(-0x3848)) * scaled) - local)
                }
            } else {
                let tail = read_f64(sda2(-0x3840)) * local * local;
                if power == 0 {
                    local - (tail - t * (tail + poly))
                } else {
                    read_f64(sda2(-0x3850)) * scaled
                        - ((tail
                            - (t * (tail + poly) + read_f64(sda2(-0x3848)) * scaled))
                            - local)
                }
            }
        }
    } else {
        value + value
    }
}

/// Computes the natural logarithm of `x` (MSL `log`).
///
/// In plain English: "split `x = m * 2^k` with `m` in `[1,2)`, ask
/// [`fn_802e51dc`] for `log(m)`, and add `k * ln 2`."
///
/// # Original
/// - Ghidra: `FUN_802e5490` at `0x802e5490` (276 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e5490_FUN_802e5490.c`
///
/// # Arguments
/// * `x` - `param_1`; the positive input.
///
/// # Returns
/// `log(x)`, or the MSL `-inf` / `NaN` result for zero and negatives.
///
/// # Side effects
/// Writes [`G_ERRNO`] (`0x21`) for zero and negative inputs.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the subnormal pre-scale, the exponent split and the three
/// `ln 2`-weighted terms are literal; the `log` role is `// inferred:` from the
/// `[1,2)` normalization and the call to the `log1p` kernel.
pub unsafe fn fn_802e5490(x: f64) -> f64 {
    let mut shift = 0_i32;
    let mut value = x;
    let mut bits = value.to_bits();
    let mut hi = (bits >> 32) as u32;

    if (hi as i32) < 0x0010_0000 {
        if value == 0.0 {
            let zero = read_f64(sda2(-0x37E8));
            platform::write_u32(G_ERRNO, 0x21);
            return zero / read_f64(sda13(-0x4670));
        }
        if value < 0.0 {
            platform::write_u32(G_ERRNO, 0x21);
            return (value - value) / read_f64(sda13(-0x4670));
        }
        shift = -0x36;
        value *= read_f64(sda2(-0x37E0));
        bits = value.to_bits();
        hi = (bits >> 32) as u32;
    }

    if (hi as i32) < 0x7FF0_0000 {
        let exponent = shift + ((hi as i32) >> 20) - 0x3FF;
        let round = exponent >> 31;
        let rebuilt = (hi & 0x000F_FFFF)
            | (((round + 0x3FF) as u32).wrapping_mul(0x0010_0000));
        let mantissa = f64::from_bits(((rebuilt as u64) << 32) | (bits & 0xFFFF_FFFF));
        let k = signed_to_f64_bits(exponent - round) - read_f64(sda2(-0x37C0));
        let log_m = fn_802e51dc(mantissa);
        read_f64(sda2(-0x37D8)) * k
            + read_f64(sda2(-0x37D0)) * log_m
            + read_f64(sda2(-0x37C8)) * k
    } else {
        value + value
    }
}

/// Evaluates the `atan2` tail polynomial after quadrant reduction.
///
/// In plain English: "reduce `(y,x)` to the first octant, evaluate the odd
/// polynomial, then rebuild the angle with the quadrant table."
///
/// # Original
/// - Ghidra: `FUN_802e79c4` at `0x802e79c4` (592 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e79c4_FUN_802e79c4.c`
///
/// # Arguments
/// * `y` - `param_1`; the numerator.
/// * `x` - `param_2`; the denominator.
/// * `quadrant` - `param_3`; the quadrant selector (`0xffffffff` is the
///   special `-0/0` case).
///
/// # Returns
/// The reduced angle, or the `0` / `-0` / `Inf` edge result.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// low - the arithmetic, the `0x3e300000` / `0x3fe59427` thresholds and every
/// coefficient load are literal, but the decompiler's `GQR0` probe at the top
/// and bottom is omitted: it only models the paired-single quantized-load
/// conversion and does not change the result. The `atan2` role is
/// `// inferred:` from the constants (`pi/4`, `pi/2`).
pub unsafe fn fn_802e79c4(y: f64, x: f64, quadrant: u32) -> f64 {
    let mut num = y;
    let mut den = x;
    let hi = (num.to_bits() >> 32) as u32;
    let abs = hi & 0x7FFF_FFFF;

    if abs < 0x3E30_0000 && (num as i32) == 0 {
        if quadrant == 0xFFFF_FFFF && num == 0.0 {
            num = read_f64(sda2(-0x3590)) / num.abs();
        } else if quadrant != 1 {
            num = read_f64(sda2(-0x3588)) / num;
        }
    } else {
        let mut reduced = num;
        if abs > 0x3FE5_9427 {
            if num < 0.0 {
                num = -num;
                den = -den;
            }
            reduced = read_f64(sda2(-0x3578)) - den;
            den = read_f64(sda2(-0x3570));
            reduced = (read_f64(sda2(-0x3580)) - num) + reduced;
        }

        let sq = reduced * reduced;
        let fourth = sq * sq;
        let poly = den
            + sq
                * (den
                    + sq * reduced
                        * (read_f64(0x802F_E210)
                            + fourth
                                * (read_f64(0x802F_E220)
                                    + fourth
                                        * (read_f64(0x802F_E230)
                                            + fourth
                                                * (read_f64(0x802F_E240)
                                                    + fourth
                                                        * (read_f64(0x802F_E250)
                                                            + fourth * read_f64(0x802F_E260)))))
                            + sq
                                * (read_f64(0x802F_E218)
                                    + fourth
                                        * (read_f64(0x802F_E228)
                                            + fourth
                                                * (read_f64(0x802F_E238)
                                                    + fourth
                                                        * (read_f64(0x802F_E248)
                                                            + fourth
                                                                * (read_f64(0x802F_E258)
                                                                    + fourth
                                                                        * read_f64(0x802F_E268))))))))
            + read_f64(0x802F_E208) * sq * reduced;

        let angle = reduced + poly;

        if abs < 0x3FE5_9428 {
            if quadrant != 1 {
                let inv = read_f64(sda2(-0x3588)) / angle;
                let hi_angle = f64::from_bits(angle.to_bits() & 0xFFFF_FFFF_0000_0000);
                let hi_inv = f64::from_bits(inv.to_bits() & 0xFFFF_FFFF_0000_0000);
                num = hi_inv
                    + inv
                        * (read_f64(sda2(-0x3590)) + hi_inv * hi_angle
                            + hi_inv * (poly - (hi_angle - reduced)));
            } else {
                num = angle;
            }
        } else {
            let k = signed_to_f64_bits(quadrant as i32) - read_f64(sda2(-0x3560));
            let sign = signed_to_f64_bits(1 - (((hi >> 30) & 2) as i32))
                - read_f64(sda2(-0x3560));
            num = sign
                * (k - read_f64(sda2(-0x3568))
                    * (reduced - ((angle * angle) / (angle + k) - poly)));
        }
    }
    num
}

/// Evaluates the range-reduced inverse-tangent polynomial and quadrant table.
///
/// In plain English: "map `x` into one of four octants, evaluate the odd
/// polynomial, and look the octant's `pi` offset up in the table at
/// `0x802FE270`."
///
/// # Original
/// - Ghidra: `FUN_802e7c14` at `0x802e7c14` (560 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e7c14_FUN_802e7c14.c`
///
/// # Arguments
/// * `x` - `param_1`; the angle argument.
///
/// # Returns
/// The angle in radians; `+/-pi` for very large finite inputs and `NaN` for
/// NaNs.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the four octant cuts, the ten coefficient loads, the `0x802FE270`
/// offset table and the `+/-pi` tail are literal; the inverse-tangent role is
/// `// inferred:` from the `atan`-style `r - r*P(r^2)` form.
pub unsafe fn fn_802e7c14(x: f64) -> f64 {
    let bits = x.to_bits();
    let hi = (bits >> 32) as u32;
    let lo = bits as u32;
    let abs = hi & 0x7FFF_FFFF;

    if abs < 0x4410_0000 {
        let mut octant;
        let mut reduced = x;
        if abs < 0x3FDC_0000 {
            if abs < 0x3E20_0000 && read_f64(sda2(-0x3558)) < read_f64(sda2(-0x3550)) + x {
                return x;
            }
            octant = -1;
        } else {
            reduced = x.abs();
            if abs < 0x3FF3_0000 {
                if abs < 0x3FE6_0000 {
                    octant = 0;
                    reduced = (read_f64(sda2(-0x3548)) * reduced - read_f64(sda2(-0x3550)))
                        / (read_f64(sda2(-0x3548)) + reduced);
                } else {
                    octant = 1;
                    reduced =
                        (reduced - read_f64(sda2(-0x3550))) / (read_f64(sda2(-0x3550)) + reduced);
                }
            } else if abs < 0x4003_8000 {
                octant = 2;
                reduced = (reduced - read_f64(sda2(-0x3540)))
                    / (read_f64(sda2(-0x3550)) + read_f64(sda2(-0x3540)) * reduced);
            } else {
                octant = 3;
                reduced = read_f64(sda2(-0x3538)) / reduced;
            }
        }

        let sq = reduced * reduced;
        let fourth = sq * sq;
        let even = fourth
            * (read_f64(0x802F_E2B8)
                + fourth
                    * (read_f64(0x802F_E2C8)
                        + fourth
                            * (read_f64(0x802F_E2D8)
                                + fourth
                                    * (read_f64(0x802F_E2E8)
                                        + fourth * read_f64(0x802F_E2F8)))));
        let odd = sq
            * (read_f64(0x802F_E2B0)
                + fourth
                    * (read_f64(0x802F_E2C0)
                        + fourth
                            * (read_f64(0x802F_E2D0)
                                + fourth
                                    * (read_f64(0x802F_E2E0)
                                        + fourth
                                            * (read_f64(0x802F_E2F0)
                                                + fourth * read_f64(0x802F_E300))))));

        if octant < 0 {
            return reduced - reduced * (odd + even);
        }
        let base = read_f64(0x802F_E270 + octant as usize * 8);
        let tail = read_f64(0x802F_E290 + octant as usize * 8);
        let result = base - ((reduced * (odd + even) - tail) - reduced);
        if x >= 0.0 {
            result
        } else {
            -result
        }
    } else {
        if abs == 0x7FF0_0000 {
            if lo != 0 {
                return x + x;
            }
        } else if abs > 0x7FEF_FFFF {
            return x + x;
        }
        if (hi as i32) < 1 {
            -read_f64(0x802F_E288) - read_f64(0x802F_E2A8)
        } else {
            read_f64(0x802F_E288) + read_f64(0x802F_E2A8)
        }
    }
}

/// `0x80429BB8` - the MSL `NaN` constant returned by [`fn_802e8620`] for a
/// negative input.
///
/// # Original
/// - Observed: `(double)DAT_80429bb8` in `802e8620_FUN_802e8620.c`
const D_8042_9BB8: usize = 0x8042_9BB8;

/// Software square root using integer digit-by-digit extraction.
///
/// In plain English: "normalize the significand, run the restoring square-root
/// loop over 52 fraction bits, then rebuild the exponent as `(e-1023)/2`."
///
/// # Original
/// - Ghidra: `FUN_802e8620` at `0x802e8620` (584 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8620_FUN_802e8620.c`
///
/// # Arguments
/// * `x` - `param_1`; the radicand.
///
/// # Returns
/// `sqrt(x)`; `NaN` for negative inputs and `errno = 0x21`.
///
/// # Side effects
/// Writes [`G_ERRNO`] (`0x21`, `EDOM`) for a negative input.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the normalization loops, the two restoring passes and the final
/// exponent assembly are literal; the `sqrt` role is `// inferred:` from the
/// digit-by-digit remainder recurrence.
pub unsafe fn fn_802e8620(x: f64) -> f64 {
    let bits = x.to_bits();
    let mut hi = (bits >> 32) as u32;
    let mut lo = bits as u32;

    if (hi & 0x7FF0_0000) == 0x7FF0_0000 {
        platform::write_u32(G_ERRNO, 0x21);
        return x + x * x;
    }
    if (hi as i32) < 1 {
        if bits & 0x7FFF_FFFF_FFFF_FFFF == 0 {
            return x;
        }
        if x < 0.0 {
            platform::write_u32(G_ERRNO, 0x21);
            return read_f64(D_8042_9BB8);
        }
    }

    let mut exponent = (hi as i32) >> 20;
    if exponent == 0 {
        exponent = 0;
        let mut frac = lo;
        while hi == 0 {
            exponent -= 0x15;
            hi = frac >> 11;
            frac <<= 21;
        }
        let mut shift = 0_u32;
        while (hi & 0x0010_0000) == 0 {
            shift += 1;
            hi <<= 1;
        }
        lo = frac << shift;
        exponent -= shift as i32 - 1;
        hi |= frac.wrapping_shr(0x20 - shift);
    }

    let mut significand = (hi & 0x000F_FFFF) | 0x0010_0000;
    if (exponent as u32).wrapping_sub(0x3FF) & 1 != 0 {
        significand = significand.wrapping_mul(2).wrapping_sub((lo as i32 >> 31) as u32);
        lo = lo.wrapping_mul(2);
    }

    let mut remainder = significand.wrapping_mul(2) as i32 - ((lo as i32) >> 31);
    let mut quotient_hi = 0_u32;
    let mut root_hi = 0_i32;
    let mut quotient_lo = 0_u32;
    let mut exponent_bits = 0_u32;

    let mut bit = 0x0020_0000_u32;
    let mut shifted;
    while bit != 0 {
        shifted = lo.wrapping_mul(2);
        let trial = root_hi + bit as i32;
        if trial <= remainder {
            root_hi = trial + bit as i32;
            remainder -= trial;
            exponent_bits = exponent_bits.wrapping_add(bit);
        }
        remainder = (((lo & 0x7FFF_FFFF) >> 30) as i32).wrapping_add(remainder.wrapping_mul(2));
        lo = shifted;
        bit >>= 1;
    }

    let mut bit = 0x8000_0000_u32;
    let mut fraction = lo.wrapping_mul(2);
    while bit != 0 {
        let trial = quotient_hi.wrapping_add(bit);
        let mut next_root = root_hi;
        if root_hi < remainder || (root_hi == remainder && trial <= fraction) {
            quotient_hi = trial.wrapping_add(bit);
            if (trial & 0x8000_0000) == 0x8000_0000 && (quotient_hi & 0x8000_0000) == 0 {
                next_root = root_hi + 1;
            }
            remainder -= root_hi;
            if fraction < trial {
                remainder -= 1;
            }
            fraction = fraction.wrapping_sub(trial);
            quotient_lo = quotient_lo.wrapping_add(bit);
        }
        remainder = remainder.wrapping_mul(2).wrapping_sub((fraction as i32) >> 31);
        fraction = fraction.wrapping_mul(2);
        root_hi = next_root;
        bit >>= 1;
    }

    if remainder != 0 || fraction != 0 {
        if quotient_lo == 0xFFFF_FFFF {
            quotient_lo = 0;
            exponent_bits = exponent_bits.wrapping_add(1);
        } else {
            quotient_lo = quotient_lo.wrapping_add(quotient_lo & 1);
        }
    }
    quotient_lo >>= 1;
    if exponent_bits & 1 == 1 {
        quotient_lo |= 0x8000_0000;
    }

    let high_word = ((exponent_bits as i32) >> 1) as u32
        + 0x3FE0_0000
        + (((exponent - 0x3FF) as u32).wrapping_mul(0x0008_0000) & 0xFFF0_0000);
    f64::from_bits(((high_word as u64) << 32) | quotient_lo as u64)
}

// ---------------------------------------------------------------------------
// Wave 95 - remaining TRK cache / register-transfer entry points.
// ---------------------------------------------------------------------------

/// `0x802E890C` - the TRK trap gate reached by [`fn_802ec0f4`].
///
/// Wave 106 translated the eight bytes as [`fn_802e890c`] (`twi 31,r0,0`), so
/// the caller uses the body directly; the constant is kept for reference.
///
/// # Original
/// - Ghidra: `FUN_802e890c` at `0x802e890c` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e890c_FUN_802e890c.c`
const FN_802E890C: usize = 0x802E_890C;

/// `0x804270C8` / `0x804270C9` - the two "SPR range overlapped a save slot"
/// flags set by [`fn_802ec6f8`].
const G_804270C8: usize = 0x8042_70C8;
const G_804270C9: usize = 0x8042_70C9;

/// `0x80351624` / `0x8035162C` - the middle and last words of the saved
/// interrupt context saved by the register-block transfers.
const G_80351624: usize = 0x8035_1624;
const G_8035162C: usize = 0x8035_162C;

/// Flush the cache range through the first entry point, unless the debugger
/// says the transport is busy.
///
/// In plain English: "when `0x802ED9CC` says the path is usable, forward the
/// range to `0x802EC0E4`/`0x802EC0F4`; otherwise report `1`."
///
/// # Original
/// - Ghidra: `FUN_802ec014` at `0x802ec014` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec014_FUN_802ec014.c`
///
/// # Arguments
/// * `_param_1` - `param_1`; the export never reads it.
/// * `param_2`..`param_4` - forwarded to the flush.
///
/// # Returns
/// The flush result, or `1` when the transport is busy.
///
/// # Side effects
/// Same as [`fn_802ec0f4`].
///
/// # Safety
/// Same as [`fn_802ec0f4`].
///
/// # Confidence
/// high - the `0x802ED9CC` guard and the four forwarded arguments are literal.
pub unsafe fn fn_802ec014(_param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32 {
    if fn_802ed9cc() == 0 {
        1
    } else {
        platform::call4(FN_802EC0F4, 0, param_2, param_3, param_4)
    }
}

/// Flush the cache range through the second entry point.
///
/// In plain English: the exact twin of [`fn_802ec014`]; the first forwarded
/// word is `1` instead of `0`.
///
/// # Original
/// - Ghidra: `FUN_802ec07c` at `0x802ec07c` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec07c_FUN_802ec07c.c`
///
/// # Arguments
/// * `_param_1` - `param_1`; unused.
/// * `param_2`..`param_4` - forwarded to the flush.
///
/// # Returns
/// The flush result, or `1` when the transport is busy.
///
/// # Side effects
/// Same as [`fn_802ec0f4`].
///
/// # Safety
/// Same as [`fn_802ec0f4`].
///
/// # Confidence
/// high - byte-for-byte the same shape as `0x802EC014`, with the literal `1`.
pub unsafe fn fn_802ec07c(_param_1: u32, param_2: u32, param_3: u32, param_4: u32) -> u32 {
    if fn_802ed9cc() == 0 {
        1
    } else {
        platform::call4(FN_802EC0F4, 1, param_2, param_3, param_4)
    }
}

/// Runs the TRK cache flush through the exception-handler gate.
///
/// In plain English: "if a handler is installed, let `0x802E890C` run the
/// transfer and translate its byte result into the TRK status code."
///
/// # Original
/// - Ghidra: `FUN_802ec0f4` at `0x802ec0f4` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec0f4_FUN_802ec0f4.c`
///
/// # Arguments
/// * `param_1` - `param_1`; transferred to the trap stub.
/// * `param_2` - `param_2`; transferred to the trap stub.
/// * `param_3` - `param_3`; pointer to the length/word the stub may update.
/// * `param_4` - `param_4`; transferred to the trap stub.
/// * `param_5` - `param_5`; the mode word used by the stub.
///
/// # Returns
/// `0` on success, `2` for the stub's retry result, `1` otherwise or when no
/// handler is installed.
///
/// # Side effects
/// May replace the word at `*param_3`.
///
/// # Safety
/// `param_3` must be a writable word pointer.
///
/// # Confidence
/// medium - the handler test, the local word copy-back and the three-way result
/// mapping are literal; the stub's role is `// inferred:` from the TRK
/// exception-handler convention.
pub unsafe fn fn_802ec0f4(
    param_1: u32,
    param_2: u32,
    param_3: usize,
    param_4: u32,
    param_5: u32,
) -> u32 {
    if fn_802eb090() == 0 {
        return 1;
    }
    let mut word = platform::read_u32(param_3);
    let result = fn_802e890c(
        param_5,
        param_1,
        &mut word as *mut u32 as u32,
        param_2,
    );
    platform::write_u32(param_3, word);
    match result {
        0 => 0,
        2 => 2,
        _ => 1,
    }
}

/// Sends the `0x90` status reply when the request is a continue / step and no
/// breakpoint condition is armed.
///
/// In plain English: "for request types 3 and 4, clear the step flag and post
/// the status block."
///
/// # Original
/// - Ghidra: `FUN_802ecdcc` at `0x802ecdcc` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ecdcc_FUN_802ecdcc.c`
///
/// # Arguments
/// * `request` - `param_1`; pointer to the request word.
///
/// # Returns
/// The result of [`fn_802e99c0`], or 0 when the request is not handled.
///
/// # Side effects
/// May call [`fn_802ed4f8`] and send a reply.
///
/// # Safety
/// `request` must be a readable word pointer.
///
/// # Confidence
/// high - the `(type - 3) < 2` test, the breakpoint check and the `0x90`
/// command are literal.
pub unsafe fn fn_802ecdcc(request: usize) -> u32 {
    let mut result = 0;
    if platform::read_u32(request).wrapping_sub(3) < 2 && fn_802ed0c4() == 0 {
        fn_802ed4f8(1);
        result = fn_802e99c0(0x90);
    }
    result
}

/// Builds and sends the `0x4E8`-byte `0x90` status reply.
///
/// In plain English: "fill a 0x40-byte header from the saved context, post the
/// 32 GPRs and the five status words, then post the 0x400-byte saved code
/// window."
///
/// # Original
/// - Ghidra: `FUN_802ece28` at `0x802ece28` (512 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ece28_FUN_802ece28.c`
///
/// # Arguments
/// * `channel` - `param_1`; the open TRK channel.
///
/// # Returns
/// 0 on success, otherwise the first failing send result.
///
/// # Side effects
/// Writes the header buffer, reads the saved GPR / SPR context, and sends via
/// [`fn_802eacdc`], [`fn_802eab18`] and [`fn_802ea9e4`].
///
/// # Safety
/// The saved context words must be readable; `channel` must be an open TRK
/// channel.
///
/// # Confidence
/// medium - every offset, the `0x20`-word GPR loop, the five status words and
/// the 0x400-byte code copy are literal; the reply-field roles are
/// `// inferred:` from the `0x90` status protocol.
pub unsafe fn fn_802ece28(channel: usize) -> i32 {
    let mut header = [0_u32; 0x10];
    let base = header.as_mut_ptr() as usize;
    fn_802e90b8(base, 0, 0x40);
    platform::write_u32(base, 0x4E8);
    platform::write_u8(base + 0x04, 0x90);
    platform::write_u32(base + 0x08, platform::read_u32(G_804272C0));

    let mut count = 0_u32;
    let mut index = 0_u32;
    fn_802ed934(&mut count as *mut u32 as usize, &mut index as *mut u32 as usize);
    platform::write_u32(base + 0x14, count);

    let status = fn_802ed92c(0xE4);
    platform::write_u32(base + 0x18, platform::read_u32(status as usize));

    let mut saved_word = 0_u32;
    let mut saved_len = 4_u32;
    platform::call5(
        FN_802EC374,
        &mut saved_word as *mut u32 as u32,
        platform::read_u32(G_804272C0),
        &mut saved_len as *mut u32 as u32,
        0,
        1,
    );
    platform::write_u32(base + 0x10, platform::read_u32(G_80427538) & 0xFFFF);
    platform::write_u32(base + 0x0C, saved_word);

    let mut result = fn_802eacdc(channel, base, 0x40) as u32;

    if result == 0 {
        let mut gpr = TRK_GPR_CONTEXT;
        for _ in 0..0x20 {
            let _ = fn_802eab18(channel, platform::read_u32(gpr));
            gpr += 4;
        }
        let _ = fn_802eab18(channel, platform::read_u32(G_804272C0));
        let _ = fn_802eab18(channel, platform::read_u32(G_804272C0 + 4));
        let _ = fn_802eab18(channel, platform::read_u32(G_804272C0 + 8));
        let _ = fn_802eab18(channel, platform::read_u32(G_804272C0 + 0xC));
        result = fn_802eab18(channel, platform::read_u32(G_804272C0 + 0x10));
    }

    if result == 0 {
        for offset in [0xD4_u32, 0xD8, 0xDC, 0xE0, 0xE4] {
            let slot = fn_802ed92c(offset);
            result = fn_802eab18(channel, platform::read_u32(slot as usize));
        }
    }

    if result == 0 {
        let mut payload = [0_u32; 0x100];
        let mut length = 0x400_u32;
        platform::call5(
            FN_802EC374,
            payload.as_mut_ptr() as u32,
            platform::read_u32(G_804272C0) & 0xFFFF_FC00,
            &mut length as *mut u32 as u32,
            0,
            1,
        );
        fn_802ea9e4(channel, payload.as_mut_ptr() as usize, 0x400);
    }

    result as i32
}

/// Copies a GPR range between the saved context and a host buffer.
///
/// In plain English: "for register indices below 0x25, copy `count` words
/// between `param_3` and `0x80427240 + index*4`."
///
/// # Original
/// - Ghidra: `FUN_802ec4c4` at `0x802ec4c4` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec4c4_FUN_802ec4c4.c`
///
/// # Arguments
/// * `first` - `param_1`; first register index.
/// * `last` - `param_2`; last register index (must be below `0x25`).
/// * `buffer` - `param_3`; the host-side word buffer.
/// * `out_len` - `param_4`; receives the byte count, or 0 on failure.
/// * `direction` - `param_5`; 0 reads the context into the buffer, non-zero
///   writes the buffer into the context.
///
/// # Returns
/// 0 on success, `0x701` for a bad range, `0x702` when the context flag is set.
///
/// # Side effects
/// Saves and restores `0x80351620..0x8035162C`; may write `out_len`; copies
/// words through [`fn_802eafa8`] / [`fn_802ead40`].
///
/// # Safety
/// `buffer` must hold `count` words and `out_len` must be writable.
///
/// # Confidence
/// medium - the range test, the byte-count formula, the save/restore and the
/// `0x702` sticky flag are literal; the register-set roles are `// inferred:`
/// from the `0x80427240` base.
pub unsafe fn fn_802ec4c4(
    first: i32,
    last: u32,
    buffer: u32,
    out_len: usize,
    direction: i32,
) -> u32 {
    let saved20 = platform::read_u32(G_80351620);
    let saved24 = platform::read_u32(G_80351624);
    let saved28 = platform::read_u32(G_80351628);
    let saved2c = platform::read_u32(G_8035162C);
    platform::write_u32(
        G_8035162C,
        (saved2c & 0xFF00_0000) | ((saved2c & 0x0000_FFFF) << 8),
    );

    let mut result;
    if last < 0x25 {
        let count = (last as i32).wrapping_sub(first).wrapping_add(1);
        platform::write_u32(out_len, (count as u32).wrapping_mul(4));
        let address = TRK_GPR_CONTEXT.wrapping_add((first as usize).wrapping_mul(4));
        result = if direction == 0 {
            fn_802eafa8(buffer as usize, address, count as u32) as u32
        } else {
            fn_802ead40(buffer as usize, address, count as u32) as u32
        };
        if (platform::read_u32(G_8035162C) >> 16) & 0xFF != 0 {
            platform::write_u32(out_len, 0);
            result = 0x702;
        }
    } else {
        result = 0x701;
    }

    platform::write_u32(G_80351620, saved20);
    platform::write_u32(G_80351624, saved24);
    platform::write_u32(G_80351628, saved28);
    platform::write_u32(G_8035162C, saved2c);
    result
}

/// Copies an SPR range between the saved context and a host buffer.
///
/// In plain English: "for SPR indices below 0x61, copy `count` words between
/// `param_3` and `0x804273E8 + index*4`, setting the two overlap flags when
/// the range covers a slot that the transfer itself uses."
///
/// # Original
/// - Ghidra: `FUN_802ec6f8` at `0x802ec6f8` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec6f8_FUN_802ec6f8.c`
///
/// # Arguments
/// * `first` - `param_1`; first SPR index.
/// * `last` - `param_2`; last SPR index (must be below `0x61`).
/// * `buffer` - `param_3`; the host-side word buffer.
/// * `out_len` - `param_4`; receives the byte count, or 0 on failure.
/// * `direction` - `param_5`; 0 reads the context into the buffer, non-zero
///   writes the buffer into the context.
///
/// # Returns
/// 0 on success, `0x701` for a bad range, `0x702` when the context flag is set;
/// when `first > last` the original returns `first` unchanged.
///
/// # Side effects
/// May set [`G_804270C8`] / [`G_804270C9`], saves and restores
/// `0x80351620..0x8035162C`.
///
/// # Safety
/// `buffer` must hold `count` words and `out_len` must be writable.
///
/// # Confidence
/// medium - the range tests, the two byte-address bounds and the save/restore
/// are literal; the SPR base is the documented [`TRK_SPR_CONTEXT`], and the
/// overlap flags are `// inferred:` to protect the live exception frame.
pub unsafe fn fn_802ec6f8(
    first: u32,
    last: u32,
    buffer: u32,
    out_len: usize,
    direction: i32,
) -> u32 {
    let saved20 = platform::read_u32(G_80351620);
    let saved24 = platform::read_u32(G_80351624);
    let saved28 = platform::read_u32(G_80351628);
    let saved2c = platform::read_u32(G_8035162C);
    platform::write_u32(
        G_8035162C,
        (saved2c & 0xFF00_0000) | ((saved2c & 0x0000_FFFF) << 8),
    );

    let mut result;
    if last < 0x61 {
        platform::write_u32(out_len, 0);
        // The original leaves `param_1` untouched when `first > last` and
        // returns it at the end.
        result = first;
        if first <= last {
            let span = last - first;
            let count = span.wrapping_add(1);
            platform::write_u32(out_len, count.wrapping_mul(4));
            let address = TRK_SPR_CONTEXT.wrapping_add((first as usize).wrapping_mul(4));
            let end = address.wrapping_add((span as usize).wrapping_mul(4));
            if direction == 0 {
                if address < 0x8042_742D && 0x8042_7427 < end {
                    platform::write_u8(G_804270C8, 1);
                }
                if address < 0x8042_74B9 && 0x8042_74B7 < end {
                    platform::write_u8(G_804270C9, 1);
                }
                result = fn_802eafa8(buffer as usize, address, count) as u32;
            } else {
                result = fn_802ead40(buffer as usize, address, count) as u32;
            }
        }
        if (platform::read_u32(G_8035162C) >> 16) & 0xFF != 0 {
            platform::write_u32(out_len, 0);
            result = 0x702;
        }
    } else {
        result = 0x701;
    }

    platform::write_u32(G_80351620, saved20);
    platform::write_u32(G_80351624, saved24);
    platform::write_u32(G_80351628, saved28);
    platform::write_u32(G_8035162C, saved2c);
    result
}

/// Builds the ten-word `mfspr`-style patch used by the register transfers.
///
/// In plain English: "fill a 0x28-byte code block with the two caller-selected
/// words plus eight constant words from `0x802FE320`, then run it through
/// `0x802ED8B4`."
///
/// # Original
/// - Ghidra: `FUN_802ed520` at `0x802ed520` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed520_FUN_802ed520.c`
///
/// # Arguments
/// * `param_1` - `param_1`; the value handed to the patched function.
/// * `spr` - `param_2`; the SPR field packed into the generated opcode.
/// * `mode` - `param_3`; 0 emits the `mfspr` form, non-zero the `mtspr` form.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes and executes the patched block through [`fn_802ed8b4`].
///
/// # Safety
/// The patched block needs executable memory, which `0x802ED8B4` supplies.
///
/// # Confidence
/// medium - the two opcode words and the eight copied constants are literal;
/// the `mfspr` / `mtspr` roles are `// inferred:` from the `0x3a6` / `0x2a6`
/// extended opcodes.
pub unsafe fn fn_802ed520(param_1: u32, spr: u32, mode: i32) {
    let mut code = [0_u32; 10];
    for (index, slot) in code.iter_mut().enumerate().skip(2) {
        *slot = platform::read_u32(0x802F_E320 + (index - 2) * 4);
    }
    if mode == 0 {
        code[1] = ((spr & 0xFE0) << 6)
            | 0x7C80_0000
            | ((spr & 0x1F) << 16)
            | 0x3A6;
        code[0] = 0x8083_0000;
    } else {
        code[1] = 0x9083_0000;
        code[0] = ((spr & 0xFE0) << 6)
            | 0x7C80_0000
            | ((spr & 0x1F) << 16)
            | 0x2A6;
    }
    fn_802ed8b4(param_1, code.as_mut_ptr() as usize, 0);
}

/// Builds the nine-word `mtspr` / `mfspr` patch used by the register
/// transfers.
///
/// In plain English: "pack `param_2` into the first opcode and copy eight
/// constant words from `0x802FE344`, then run the block through
/// `0x802ED8B4`."
///
/// # Original
/// - Ghidra: `FUN_802ed5fc` at `0x802ed5fc` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed5fc_FUN_802ed5fc.c`
///
/// # Arguments
/// * `param_1` - `param_1`; the value handed to the patched function.
/// * `spr` - `param_2`; the SPR field packed into the first word.
/// * `mode` - `param_3`; selects the `0xE003` or `0xF003` opcode form.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes and executes the patched block through [`fn_802ed8b4`].
///
/// # Safety
/// Same as [`fn_802ed520`].
///
/// # Confidence
/// medium - the shifted field, the two opcode constants and the eight copied
/// words are literal; the instruction roles are `// inferred:`.
pub unsafe fn fn_802ed5fc(param_1: u32, spr: i32, mode: i32) {
    let mut code = [0_u32; 10];
    code[0] = ((spr as u32) << 21) | 0xE003_0000;
    for (index, slot) in code.iter_mut().enumerate().skip(1) {
        *slot = platform::read_u32(0x802F_E344 + (index - 1) * 4);
    }
    if mode != 0 {
        code[0] = ((spr as u32) << 21) | 0xF003_0000;
    }
    fn_802ed8b4(param_1, code.as_mut_ptr() as usize, mode as u32);
}

/// Tail branch to the MSL `log` body at `0x802E5490`.
///
/// The four-byte function is a single unconditional branch; Ghidra inlined the
/// target into the export, so the body here is a direct call.
///
/// # Original
/// - Ghidra: `thunk_FUN_802e5490` at `0x802e8618` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8618_thunk_FUN_802e5490.c`
///
/// # Arguments
/// * `x` - `param_1`; forwarded unchanged.
///
/// # Returns
/// `log(x)`.
///
/// # Side effects
/// Same as [`fn_802e5490`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the export is the target body verbatim and the call site is a
/// four-byte branch.
pub unsafe fn fn_802e8618(x: f64) -> f64 {
    fn_802e5490(x)
}

/// Tail branch to the software square root at `0x802E8620`.
///
/// # Original
/// - Ghidra: `thunk_FUN_802e8620` at `0x802e8868` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8868_thunk_FUN_802e8620.c`
///
/// # Arguments
/// * `x` - `param_1`; forwarded unchanged.
///
/// # Returns
/// `sqrt(x)`.
///
/// # Side effects
/// Same as [`fn_802e8620`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the export is the target body verbatim and the call site is a
/// four-byte branch.
pub unsafe fn fn_802e8868(x: f64) -> f64 {
    fn_802e8620(x)
}

/// Computes arccosine with the MSL range-splitting polynomial (MSL `acos`).
///
/// In plain English: "for `|x| < 0.5` return `pi/2` minus the odd arcsine
/// series; for the remaining in-domain values reduce around `+/-1` and call the
/// software square root; outside `[-1,1]` set `errno` and return `NaN`."
///
/// # Original
/// - Ghidra: `FUN_802e46a0` at `0x802e46a0` (716 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e46a0_FUN_802e46a0.c`
///
/// # Arguments
/// * `x` - `param_1`; the cosine value.
///
/// # Returns
/// `acos(x)` in radians, or `NaN` when `|x| > 1`.
///
/// # Side effects
/// Writes [`G_ERRNO`] (`0x21`, `EDOM`) when the argument is out of domain.
///
/// # Safety
/// None.
///
/// # Confidence
/// low - every coefficient load, threshold and the two reduction branches are
/// literal, but the decompiler's `GQR0` probe at the top and bottom is omitted;
/// it only models the paired-single quantized-load conversion and does not
/// change the result. The `acos` role is `// inferred:` from the `[-1,1]`
/// domain test, the `pi/2` tiny-input result and the `sqrt`-based reduction.
pub unsafe fn fn_802e46a0(x: f64) -> f64 {
    let bits = x.to_bits();
    let hi = (bits >> 32) as u32;
    let abs = hi & 0x7FFF_FFFF;

    if abs < 0x3FF0_0000 {
        if abs < 0x3FE0_0000 {
            if abs < 0x3C60_0001 {
                read_f64(sda2(-0x39C0))
            } else {
                let x2 = x * x;
                let numerator = x2
                    * (read_f64(sda2(-0x39B8))
                        + x2
                            * (read_f64(sda2(-0x39B0))
                                + x2
                                    * (read_f64(sda2(-0x39A8))
                                        + x2
                                            * (read_f64(sda2(-0x39A0))
                                                + x2
                                                    * (read_f64(sda2(-0x3998))
                                                        + read_f64(sda2(-0x3990)) * x2)))));
                let denominator = read_f64(sda2(-0x3988))
                    + x2
                        * (read_f64(sda2(-0x3980))
                            + x2
                                * (read_f64(sda2(-0x3978))
                                    + x2
                                        * (read_f64(sda2(-0x3970))
                                            + read_f64(sda2(-0x3968)) * x2)));
                read_f64(sda2(-0x39C0))
                    - (x - (read_f64(sda2(-0x3960)) - x * (numerator / denominator)))
            }
        } else if x < 0.0 {
            let t = read_f64(sda2(-0x3958)) * (read_f64(sda2(-0x3988)) + x);
            let p4 = read_f64(sda2(-0x39A0))
                + t * (read_f64(sda2(-0x3998)) + read_f64(sda2(-0x3990)) * t);
            let p3 = read_f64(sda2(-0x39A8)) + t * p4;
            let p2 = read_f64(sda2(-0x39B0)) + t * p3;
            let den = read_f64(sda2(-0x3988))
                + t
                    * (read_f64(sda2(-0x3980))
                        + t * (read_f64(sda2(-0x3978))
                            + t * (read_f64(sda2(-0x3970))
                                + read_f64(sda2(-0x3968)) * t)));
            let p1 = t * (read_f64(sda2(-0x39B8)) + t * p2);
            let root = fn_802e8620(t);
            read_f64(sda2(-0x39C8))
                - read_f64(sda2(-0x3950)) * (root + ((p1 / den) * root - read_f64(sda2(-0x3960))))
        } else {
            let t = read_f64(sda2(-0x3958)) * (read_f64(sda2(-0x3988)) - x);
            let root = fn_802e8620(t);
            let hi_root = f64::from_bits(root.to_bits() & 0xFFFF_FFFF_0000_0000);
            let numerator = t
                * (read_f64(sda2(-0x39B8))
                    + t
                        * (read_f64(sda2(-0x39B0))
                            + t
                                * (read_f64(sda2(-0x39A8))
                                    + t
                                        * (read_f64(sda2(-0x39A0))
                                            + t
                                                * (read_f64(sda2(-0x3998))
                                                    + read_f64(sda2(-0x3990)) * t)))));
            let denominator = read_f64(sda2(-0x3988))
                + t
                    * (read_f64(sda2(-0x3980))
                        + t * (read_f64(sda2(-0x3978))
                            + t * (read_f64(sda2(-0x3970))
                                + read_f64(sda2(-0x3968)) * t)));
            read_f64(sda2(-0x3950))
                * (hi_root
                    + (t - hi_root * hi_root) / (root + hi_root)
                    + (numerator / denominator) * root)
        }
    } else {
        let low = bits as u32;
        if abs == 0x3FF0_0000 && low == 0 {
            if x < 0.0 {
                read_f64(sda2(-0x39C8))
            } else {
                read_f64(sda2(-0x39D0))
            }
        } else {
            platform::write_u32(G_ERRNO, 0x21);
            read_f64(D_8042_9BB8)
        }
    }
}

// ---------------------------------------------------------------------------
// Wave 99 - the sine / cosine / tangent wrappers, the huge-argument range
// reduction and three TRK block-transfer helpers.
// ---------------------------------------------------------------------------

/// `0x802E624C` - the Payne-Hanek reduction tail that [`fn_802e5db8`] calls for
/// arguments too large for the three-coefficient reduction.
///
/// `// inferred:` the six register arguments are `(double *x, double *y, int e0,
/// int nx, int prec, const int *ipio2)`, the classic
/// `__kernel_rem_pio2` signature; the call site matches that shape exactly.
///
/// # Original
/// - Ghidra: `FUN_802e624c` at `0x802e624c` (5816 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e624c_FUN_802e624c.c`
pub const FN_802E624C: usize = 0x802E_624C;

/// `0x802EC1A8` - the `mtmsr` leaf that pairs with [`fn_802ec1a0`].
///
/// Wave 106 translated the body as [`fn_802ec1a8`]; the constant is kept for
/// reference.
///
/// # Original
/// - Ghidra: `FUN_802ec1a8` at `0x802ec1a8` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec1a8_FUN_802ec1a8.c`
pub const FN_802EC1A8: usize = 0x802E_C1A8;

/// `0x802ED6A4` - the `mffs`-to-memory leaf used by [`fn_802ed6ec`].
///
/// Wave 106 translated the body as [`fn_802ed6a4`]; the constant is kept for
/// reference.
///
/// # Original
/// - Ghidra: `FUN_802ed6a4` at `0x802ed6a4` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed6a4_FUN_802ed6a4.c`
pub const FN_802ED6A4: usize = 0x802E_D6A4;

/// `0x802ED6C8` - the FPSCR image installer used by [`fn_802ed6ec`].
///
/// Wave 106 translated the body as [`fn_802ed6c8`]; the constant is kept for
/// reference.
///
/// # Original
/// - Ghidra: `FUN_802ed6c8` at `0x802ed6c8` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed6c8_FUN_802ed6c8.c`
pub const FN_802ED6C8: usize = 0x802E_D6C8;

/// Scales a double by a power of two (MSL `scalbn`).
///
/// In plain English: "split `x` into a fraction and a binary exponent, add `n`
/// to that exponent, then rebuild the value."
///
/// # Original
/// - Ghidra: `FUN_802e4660` at `0x802e4660` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4660_FUN_802e4660.c`
///
/// # Arguments
/// * `x` - the double in `f1`; Ghidra dropped it from the signature because the
///   body only forwards it to `frexp`.
/// * `n` - `param_1`; the binary exponent to add.
///
/// # Returns
/// `x * 2^n`, with the subnormal / overflow handling of [`fn_802e81d4`] and
/// [`fn_802e825c`].
///
/// # Side effects
/// None outside the two-word stack slot that receives the `frexp` exponent.
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the `frexp` call, the exponent add on the stack slot and the `ldexp`
/// call are literal; only the dropped double parameter is `// inferred:` from
/// the register it must occupy.
pub unsafe fn fn_802e4660(x: f64, n: i32) -> f64 {
    let mut exponent: i32 = 0;
    let fraction = fn_802e81d4(x, &mut exponent as *mut i32 as usize);
    fn_802e825c(fraction, exponent.wrapping_add(n))
}

/// Reduces a double to `[-pi/4, pi/4]` and reports the quadrant (MSL
/// `__ieee754_rem_pio2`).
///
/// In plain English: "subtract the right multiple of `pi/2` from `x`, return
/// the quadrant `n` in `[-3,3]`, and store the reduced value plus its
/// correction as two doubles."
///
/// # Original
/// - Ghidra: `FUN_802e5db8` at `0x802e5db8` (900 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e5db8_FUN_802e5db8.c`
///
/// # Arguments
/// * `x` - `param_1`; the angle in radians.
/// * `_pio2` - the word every caller loads from `.sdata2` before the call
///   (`sda2(-0x3520)` / `-0x34d8` / `-0x34d0`). The body never reads it; its
///   role is `// unknown:`, but the value is forwarded unchanged so the call
///   site stays byte-for-byte equivalent.
/// * `out` - `param_2`; receives the reduced value at `out + 0` and the
///   correction at `out + 8`.
///
/// # Returns
/// The quadrant: `0` for `|x| < pi/4`, `+/-1` for the `pi/2` band and an
/// integer multiple of `pi/2` otherwise.
///
/// # Side effects
/// Writes the two doubles through `out`.
///
/// # Safety
/// `out..out+16` must be writable and 8-byte aligned for `f64` stores.
///
/// # Confidence
/// medium - the four range cuts, every coefficient load, the `npio2_hw` table
/// compare at `0x802FE138` and the sign fixups are literal; the huge-argument
/// branch builds the three-word `tx` array and calls [`fn_802e624c`] with the
/// `while (tx[nx-1] == 0) nx--` scan that the export renders as a malformed
/// pointer walk, so that scan is `// inferred:` from the classic
/// `__kernel_rem_pio2` shape.
pub unsafe fn fn_802e5db8(x: f64, _pio2: f64, out: usize) -> i32 {
    let bits = x.to_bits();
    let hi = (bits >> 32) as u32;
    let abs = hi & 0x7FFF_FFFF;

    if abs < 0x3FE9_21FC {
        write_f64(out, x);
        write_f64(out + 8, read_f64(sda2(-0x36A8)));
        return 0;
    }

    if abs < 0x4002_D97C {
        // One `pi/2` step; the sign of `x` selects the subtraction direction.
        if (hi as i32) < 1 {
            let mut t = read_f64(sda2(-0x36A0)) + x;
            if abs == 0x3FF9_21FB {
                let chunk = read_f64(sda2(-0x3688));
                t += read_f64(sda2(-0x3690));
                let high = chunk + t;
                write_f64(out, high);
                write_f64(out + 8, chunk + (t - high));
            } else {
                let chunk = read_f64(sda2(-0x3698));
                let high = chunk + t;
                write_f64(out, high);
                write_f64(out + 8, chunk + (t - high));
            }
            return -1;
        }
        let mut t = x - read_f64(sda2(-0x36A0));
        if abs == 0x3FF9_21FB {
            let chunk = read_f64(sda2(-0x3688));
            t -= read_f64(sda2(-0x3690));
            let high = t - chunk;
            write_f64(out, high);
            write_f64(out + 8, (t - high) - chunk);
        } else {
            let chunk = read_f64(sda2(-0x3698));
            let high = t - chunk;
            write_f64(out, high);
            write_f64(out + 8, (t - high) - chunk);
        }
        return 1;
    }

    if abs < 0x4139_21FC {
        // `|x| < 2^20 * pi/2`: pick the multiple from a linear estimate and
        // peel up to three `pi/2` chunks off in decreasing precision.
        let exponent = (abs as i32) >> 20;
        let mut quadrant =
            (read_f64(sda2(-0x3680)) + read_f64(sda2(-0x3678)) * x.abs()) as i32;
        let scaled = signed_to_f64_bits(quadrant) - read_f64(sda2(-0x3658));
        let mut correction = read_f64(sda2(-0x3698)) * scaled;
        let mut reduced = x.abs() - read_f64(sda2(-0x36A0)) * scaled;
        // `npio2_hw` at `0x802FE138`: exact `pi/2` multiples, no correction.
        if quadrant < 0x20
            && abs != platform::read_u32(0x802F_E138 + (quadrant as usize - 1) * 4)
        {
            write_f64(out, reduced - correction);
        } else {
            write_f64(out, reduced - correction);
            if 0x10 < exponent - ((platform::read_u32(out) >> 20) & 0x7FF) as i32 {
                let high = read_f64(sda2(-0x3690)) * scaled;
                let middle = reduced - high;
                correction = read_f64(sda2(-0x3688)) * scaled - ((reduced - middle) - high);
                write_f64(out, middle - correction);
                reduced = middle;
                if 0x31 < exponent - ((platform::read_u32(out) >> 20) & 0x7FF) as i32 {
                    let high = read_f64(sda2(-0x3670)) * scaled;
                    reduced = middle - high;
                    correction = read_f64(sda2(-0x3668)) * scaled - ((middle - reduced) - high);
                    write_f64(out, reduced - correction);
                }
            }
        }
        let low = (reduced - read_f64(out)) - correction;
        write_f64(out + 8, low);
        if x < 0.0 {
            write_f64(out, -read_f64(out));
            write_f64(out + 8, -low);
            quadrant = -quadrant;
        }
        return quadrant;
    }

    if abs < 0x7FF0_0000 {
        // Huge but finite: hand three 24-bit digits to the Payne-Hanek tail.
        let e0 = exponent_of(abs) - 0x416;
        let z_hi = (abs as i32).wrapping_sub(e0.wrapping_mul(0x0010_0000)) as u32;
        let z = f64::from_bits(((z_hi as u64) << 32) | (bits & 0xFFFF_FFFF));
        let digit = z as i32;
        let tx0 = signed_to_f64_bits(digit) - read_f64(sda2(-0x3658));
        let rest = read_f64(sda2(-0x3660)) * (z - tx0);
        let word = rest as u32;
        let tx1 = signed_to_f64_bits(word as i32) - read_f64(sda2(-0x3658));
        let tx2 = read_f64(sda2(-0x3660)) * (rest - tx1);
        let mut tx = [tx0, tx1, tx2];
        let mut nx = 3_usize;
        while nx > 1 && tx[nx - 1] == 0.0 {
            nx -= 1;
        }
        // observed: `bl 0x802e624c` with `r3 = tx`, `r4 = ty`, `r5 = e0`,
        // `r6 = nx`, `r7 = 2` (64-bit) and `r8 = 0x802fe030`.
        let mut quadrant = fn_802e624c(
            tx.as_mut_ptr() as usize,
            out,
            e0,
            nx as i32,
            2,
            0x802F_E030,
        );
        if x < 0.0 {
            quadrant = -quadrant;
            write_f64(out, -read_f64(out));
            write_f64(out + 8, -read_f64(out + 8));
        }
        return quadrant;
    }

    write_f64(out, x - x);
    write_f64(out + 8, x - x);
    0
}

/// Computes cosine (MSL `cos`).
///
/// In plain English: "for `|x| < pi/4` evaluate the cosine series directly;
/// otherwise reduce by `pi/2` and pick cosine or sine with the sign that the
/// quadrant demands."
///
/// # Original
/// - Ghidra: `FUN_802e7fbc` at `0x802e7fbc` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e7fbc_FUN_802e7fbc.c`
///
/// # Arguments
/// * `x` - `param_1`; the angle in radians.
///
/// # Returns
/// `cos(x)`; `NaN` for infinities and NaNs.
///
/// # Side effects
/// None outside the 16-byte reduction buffer on the stack.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the `0x3FE921FC` / `0x7FF00000` cuts and the four quadrant cases
/// are literal; the small-argument call is rendered with no arguments, so the
/// `(x, 0.0)` argument pair is `// inferred:` from the `__kernel_cos` contract
/// of [`fn_802e613c`].
pub unsafe fn fn_802e7fbc(x: f64) -> f64 {
    let abs = ((x.to_bits() >> 32) as u32) & 0x7FFF_FFFF;
    if abs < 0x3FE9_21FC {
        return fn_802e613c(x, 0.0);
    }
    if abs < 0x7FF0_0000 {
        let mut reduced = [0.0_f64; 2];
        let quadrant =
            fn_802e5db8(x, read_f64(sda2(-0x3520)), reduced.as_mut_ptr() as usize) & 3;
        return match quadrant {
            0 => fn_802e613c(reduced[0], reduced[1]),
            1 => -fn_802e7904(reduced[0], reduced[1], 1),
            2 => -fn_802e613c(reduced[0], reduced[1]),
            _ => fn_802e7904(reduced[0], reduced[1], 1),
        };
    }
    x - x
}

/// Computes sine (MSL `sin`).
///
/// In plain English: "for `|x| < pi/4` evaluate the sine series directly;
/// otherwise reduce by `pi/2` and pick sine or cosine with the quadrant sign."
///
/// # Original
/// - Ghidra: `FUN_802e84c4` at `0x802e84c4` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e84c4_FUN_802e84c4.c`
///
/// # Arguments
/// * `x` - `param_1`; the angle in radians.
///
/// # Returns
/// `sin(x)`; `NaN` for infinities and NaNs.
///
/// # Side effects
/// None outside the 16-byte reduction buffer on the stack.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the cuts and the four quadrant cases are literal; the
/// small-argument call keeps only the integer argument, so `(x, 0.0, 0)` is
/// `// inferred:` from the `__kernel_sin` contract of [`fn_802e7904`].
pub unsafe fn fn_802e84c4(x: f64) -> f64 {
    let abs = ((x.to_bits() >> 32) as u32) & 0x7FFF_FFFF;
    if abs < 0x3FE9_21FC {
        return fn_802e7904(x, 0.0, 0);
    }
    if abs < 0x7FF0_0000 {
        let mut reduced = [0.0_f64; 2];
        let quadrant =
            fn_802e5db8(x, read_f64(sda2(-0x34D8)), reduced.as_mut_ptr() as usize) & 3;
        return match quadrant {
            0 => fn_802e7904(reduced[0], reduced[1], 1),
            1 => fn_802e613c(reduced[0], reduced[1]),
            2 => -fn_802e7904(reduced[0], reduced[1], 1),
            _ => -fn_802e613c(reduced[0], reduced[1]),
        };
    }
    x - x
}

/// Computes tangent (MSL `tan`).
///
/// In plain English: "for `|x| < pi/4` evaluate the tangent kernel directly;
/// otherwise reduce by `pi/2` and invert the result for odd quadrants."
///
/// # Original
/// - Ghidra: `FUN_802e8590` at `0x802e8590` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8590_FUN_802e8590.c`
///
/// # Arguments
/// * `x` - `param_1`; the angle in radians.
///
/// # Returns
/// `tan(x)`; `NaN` for infinities and NaNs.
///
/// # Side effects
/// None outside the 16-byte reduction buffer on the stack.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the cuts, the `(quadrant & 1) * -2 + 1` sign expression and the
/// kernel call are literal; the argument pair of the small-argument call is
/// `// inferred:` from the `__kernel_tan` contract of [`fn_802e79c4`].
pub unsafe fn fn_802e8590(x: f64) -> f64 {
    let abs = ((x.to_bits() >> 32) as u32) & 0x7FFF_FFFF;
    if abs < 0x3FE9_21FC {
        return fn_802e79c4(x, 0.0, 1);
    }
    if abs < 0x7FF0_0000 {
        let mut reduced = [0.0_f64; 2];
        let quadrant =
            fn_802e5db8(x, read_f64(sda2(-0x34D0)), reduced.as_mut_ptr() as usize) & 3;
        let odd = ((quadrant & 1).wrapping_mul(-2).wrapping_add(1)) as u32;
        return fn_802e79c4(reduced[0], reduced[1], odd);
    }
    x - x
}

/// Four-byte branch to the arccosine body.
///
/// In plain English: "this entry point is `b 0x802E46A0`, so it is `acos`."
///
/// # Original
/// - Ghidra: `thunk_FUN_802e46a0` at `0x802e8608` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8608_thunk_FUN_802e46a0.c`
///
/// # Arguments
/// * `x` - `param_1`; forwarded unchanged to [`fn_802e46a0`].
///
/// # Returns
/// `acos(x)`.
///
/// # Side effects
/// Same as [`fn_802e46a0`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the export is the target body verbatim and the entry is a four-byte
/// branch.
pub unsafe fn fn_802e8608(x: f64) -> f64 {
    fn_802e46a0(x)
}

/// Copies a byte range between two addresses with read-modify-write word
/// accesses (TRK unaligned block copy).
///
/// In plain English: "copy `len` bytes one at a time by shifting each source
/// byte into its position inside the destination word."
///
/// # Original
/// - Ghidra: `FUN_802ec2a8` at `0x802ec2a8` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec2a8_FUN_802ec2a8.c`
///
/// # Arguments
/// * `dst` - `param_1`; destination address.
/// * `src` - `param_2`; source address.
/// * `len` - `param_3`; byte count.
/// * `store_msr` - `param_4`; MSR image installed before the destination store
///   (`// inferred:` from the `msr | 0x10` value the caller builds).
/// * `load_msr` - `param_5`; MSR image installed before the source load.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the destination bytes through read-modify-write word stores, and
/// installs / restores the MSR through [`fn_802ec1a0`] / [`fn_802ec1a8`]. The
/// two `sync` instructions are documented but not emitted, because the body
/// models only the observable word stores.
///
/// # Safety
/// Both ranges must be readable / writable word-aligned windows in the target
/// address space.
///
/// # Confidence
/// medium - the mask arithmetic, the two MSR installs and the final restore are
/// literal; the MSR roles are `// inferred:` from the one call site that passes
/// `msr` and `msr | MSR.EE`.
pub unsafe fn fn_802ec2a8(dst: u32, src: u32, len: i32, store_msr: u32, load_msr: u32) {
    let saved_msr = fn_802ec1a0();
    let mut d = dst;
    let mut s = src;
    let mut remaining = len;
    while remaining != 0 {
        fn_802ec1a8(load_msr);
        let word = platform::read_u32((s & !3) as usize);
        // observed: `sync` between the load and the store.
        fn_802ec1a8(store_msr);
        let target = (d & !3) as usize;
        let dst_shift = 8 * (3 - (d & 3));
        let src_shift = 8 * (3 - (s & 3));
        let mask = 0xFF_u32 << dst_shift;
        let value = (word >> src_shift) & 0xFF;
        let old = platform::read_u32(target);
        platform::write_u32(target, (old & !mask) | (mask & (value << dst_shift)));
        // observed: a second `sync` after the store.
        remaining -= 1;
        s = s.wrapping_add(1);
        d = d.wrapping_add(1);
    }
    fn_802ec1a8(saved_msr);
}

/// Moves a block between a host buffer and a mapped target address (TRK block
/// transfer).
///
/// In plain English: "map the source address, refuse the transfer when the
/// cache-window test fails, copy with [`fn_802ec2a8`], then flush both windows."
///
/// # Original
/// - Ghidra: `FUN_802ec374` at `0x802ec374` (336 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec374_FUN_802ec374.c`
///
/// # Arguments
/// * `target` - `param_1`; the host buffer address.
/// * `source` - `param_2`; the target address to map.
/// * `len` - `param_3`; pointer to the word holding the byte count.
/// * `_unused` - `param_4`; never read by the body (both call sites pass 0).
/// * `size` - `param_5`; zero selects the mapped-buffer direction, non-zero the
///   reverse direction.
///
/// # Returns
/// 0 on success, `0x702` when the saved-context flag was set, or the
/// cache-window test result.
///
/// # Side effects
/// Saves and restores `0x80351620..0x8035162C`, may zero `*len`, copies through
/// [`fn_802ec2a8`] and flushes through `0x802E8E1C`.
///
/// # Safety
/// `len` must be a writable word; the two buffers must cover `*len` bytes.
///
/// # Confidence
/// medium - the range test, the save / restore bracket, the
/// `countLeadingZeros(size) >> 5` selector and the `0x702` flag are literal;
/// the reverse-direction call drops its fifth argument in the export, so
/// `load_msr = store_msr = msr` is `// inferred:` from the forward call.
pub unsafe fn fn_802ec374(
    target: u32,
    source: u32,
    len: usize,
    _unused: u32,
    size: i32,
) -> i32 {
    let saved20 = platform::read_u32(G_80351620);
    let saved24 = platform::read_u32(G_80351624);
    let saved28 = platform::read_u32(G_80351628);
    let saved2c = platform::read_u32(G_8035162C);
    platform::write_u32(
        G_8035162C,
        (saved2c & 0xFF00_0000) | ((saved2c & 0x0000_FFFF) << 8),
    );

    let mapped = fn_802e9430(source);
    let which = (size as u32).leading_zeros() >> 5;
    let mut result = fn_802ec1b0(mapped, platform::read_u32(len), which);
    if result == 0 {
        let msr = fn_802ec1a0();
        if size == 0 {
            fn_802ec2a8(
                mapped,
                target,
                platform::read_u32(len) as i32,
                msr | (platform::read_u32(G_80427438) & 0x10),
                msr,
            );
            fn_802e8e1c(mapped, platform::read_u32(len) as i32);
            if source != mapped {
                fn_802e8e1c(source, platform::read_u32(len) as i32);
            }
        } else {
            // observed: the export passes only four arguments here.
            fn_802ec2a8(
                target,
                mapped,
                platform::read_u32(len) as i32,
                msr,
                msr,
            );
        }
    } else {
        platform::write_u32(len, 0);
    }

    if (platform::read_u32(G_8035162C) >> 16) & 0xFF != 0 {
        platform::write_u32(len, 0);
        result = 0x702;
    }
    platform::write_u32(G_80351620, saved20);
    platform::write_u32(G_80351624, saved24);
    platform::write_u32(G_80351628, saved28);
    platform::write_u32(G_8035162C, saved2c);
    result
}

/// Builds the `mfspr` / `mtspr` patch block for one special-purpose register
/// and runs it with [`fn_802ed8b4`].
///
/// In plain English: "fill a ten-word code block with the table template for
/// SPR `spr`, patch in the read or write form, then branch into it."
///
/// # Original
/// - Ghidra: `FUN_802ed6ec` at `0x802ed6ec` (456 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed6ec_FUN_802ed6ec.c`
///
/// # Arguments
/// * `buf` - `param_1`; the value handed to the patched code (a TRK buffer).
/// * `spr` - `param_2`; the SPR number; `0x20` selects `mffs` and `0x21` the
///   FPSCR read / write pair.
/// * `write` - `param_3`; 0 builds the read form, non-zero the write form.
///
/// # Returns
/// The value [`fn_802ed8b4`] returns (always 0).
///
/// # Side effects
/// Writes the stack code block, calls it, and for `spr == 0x20` executes
/// [`fn_802ed6c8`] (install the FPSCR from `buf`) or [`fn_802ed6a4`]
/// (store the FPSCR into `buf`).
///
/// # Safety
/// The patched block must be executable; `buf` must be a valid argument for the
/// generated instruction sequence.
///
/// # Confidence
/// medium - the two constant tables (`0x802FE320..0x802FE33C` and
/// `0x802FE368..0x802FE38C`), the three opcode words, the `spr << 0x15` field
/// and the copy-back for the write form are literal; the `spr == 0x20` branch
/// ends in a self-assignment that Ghidra could not resolve and that is left as
/// a comment. The two FPSCR callees now receive `buf`, their `r3` argument in
/// the original.
pub unsafe fn fn_802ed6ec(buf: usize, spr: u32, write: i32) -> u32 {
    // Template the `mtspr` forms need; indices 0 / 1 are overwritten below.
    let mut words = [0_u32; 10];
    let mut i = 0;
    while i < 10 {
        words[i] = platform::read_u32(0x802F_E368 + i * 4);
        i += 1;
    }

    if spr < 0x20 {
        words[0] = (spr << 0x15)
            | if write != 0 {
                0xD803_0000
            } else {
                0xC803_0000
            };
        return fn_802ed8b4(buf as u32, words.as_mut_ptr() as usize, write as u32);
    }

    if spr == 0x20 {
        if write == 0 {
            fn_802ed6c8(buf);
        } else {
            fn_802ed6a4(buf);
        }
        platform::write_u32(buf, 0);
        // observed: `param_1[1] = param_1[1]` in the export, a self-assignment
        // that stores no new value.
        return 0;
    }

    if spr == 0x21 {
        if write == 0 {
            platform::write_u32(buf, platform::read_u32(buf + 4));
        }
        let mut j = 0;
        while j < 8 {
            words[2 + j] = platform::read_u32(0x802F_E320 + j * 4);
            j += 1;
        }
        if write == 0 {
            words[0] = 0x8083_0000;
            words[1] = 0x7C9E_FBA6;
        } else {
            words[0] = 0x7C9E_FAA6;
            words[1] = 0x9083_0000;
        }
        let result = fn_802ed8b4(buf as u32, words.as_mut_ptr() as usize, write as u32);
        if write != 0 {
            platform::write_u32(buf + 4, platform::read_u32(buf));
            platform::write_u32(buf, 0);
        }
        return result;
    }

    0
}

/// Returns the biased exponent field of an already-masked high word.
///
/// # Original
/// Host-side helper; mirrors the `(int)uVar1 >> 0x14` shifts in
/// `802e5db8_FUN_802e5db8.c`.
///
/// # Safety
/// None.
#[inline]
fn exponent_of(abs_hi: u32) -> i32 {
    (abs_hi as i32) >> 20
}

/// Computes arctangent with the MSL two-branch series (MSL `atan`).
///
/// In plain English: "for `|x| < 0.5` evaluate the odd rational series; for
/// `0.5 <= |x| < 1` reduce with a square root and a `pi/2` correction; outside
/// the domain return `+/-pi/2` or `NaN`."
///
/// # Original
/// - Ghidra: `FUN_802e496c` at `0x802e496c` (664 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e496c_FUN_802e496c.c`
///
/// # Arguments
/// * `x` - `param_1`; the tangent value.
///
/// # Returns
/// `atan(x)` in radians; `+/-pi/2 * x` for `|x| == 1` and `NaN` (with
/// `errno = 0x21`) outside `[-1, 1]`.
///
/// # Side effects
/// Writes [`G_ERRNO`] (`0x21`, `EDOM`) for `|x| > 1`.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - every coefficient load, the three magnitude cuts and both
/// reduction forms are literal; the decompiler's `GQR0` probes are omitted
/// paired-single artifacts, and the `x*x` that the tiny-input path needs is
/// `// inferred:` because Ghidra removed the unreachable block that set it.
pub unsafe fn fn_802e496c(x: f64) -> f64 {
    let hi = (x.to_bits() >> 32) as u32;
    let abs = hi & 0x7FFF_FFFF;
    let mut result;

    if abs < 0x3FF0_0000 {
        if abs < 0x3FE0_0000 {
            if abs < 0x3E40_0000
                && read_f64(sda2(-0x3930)) < read_f64(sda2(-0x3938)) + x
            {
                // tiny input: `huge + x > 1.0` makes the result exactly `x`.
                return x;
            }
            let x2 = x * x;
            let numerator = x2
                * (read_f64(sda2(-0x3928))
                    + x2
                        * (read_f64(sda2(-0x3920))
                            + x2
                                * (read_f64(sda2(-0x3918))
                                    + x2
                                        * (read_f64(sda2(-0x3910))
                                            + x2
                                                * (read_f64(sda2(-0x3908))
                                                    + read_f64(sda2(-0x3900)) * x2)))));
            let denominator = read_f64(sda2(-0x3930))
                + x2
                    * (read_f64(sda2(-0x38F8))
                        + x2
                            * (read_f64(sda2(-0x38F0))
                                + x2
                                    * (read_f64(sda2(-0x38E8))
                                        + read_f64(sda2(-0x38E0)) * x2)));
            result = x + x * (numerator / denominator);
        } else {
            // 0.5 <= |x| < 1: reduce through `sqrt` and a `pi/2` offset.
            let t = read_f64(sda2(-0x38D8)) * (read_f64(sda2(-0x3930)) - x.abs());
            let p5 = read_f64(sda2(-0x3910))
                + t * (read_f64(sda2(-0x3908)) + read_f64(sda2(-0x3900)) * t);
            let p4 = read_f64(sda2(-0x3918)) + t * p5;
            let p3 = read_f64(sda2(-0x3920)) + t * p4;
            let denominator = read_f64(sda2(-0x3930))
                + t
                    * (read_f64(sda2(-0x38F8))
                        + t * (read_f64(sda2(-0x38F0))
                            + t * (read_f64(sda2(-0x38E8))
                                + read_f64(sda2(-0x38E0)) * t)));
            let numerator = t * (read_f64(sda2(-0x3928)) + t * p3);
            let root = fn_802e8620(t);
            let root_hi = f64::from_bits(root.to_bits() & 0xFFFF_FFFF_0000_0000);
            if abs < 0x3FEF_3333 {
                let scale = read_f64(sda2(-0x38D0));
                result = read_f64(sda2(-0x38C8))
                    - ((scale * root * (numerator / denominator)
                        - (read_f64(sda2(-0x3940))
                            - scale * ((t - root_hi * root_hi) / (root + root_hi))))
                        - (read_f64(sda2(-0x38C8)) - scale * root_hi));
            } else {
                result = read_f64(sda2(-0x3948))
                    - (read_f64(sda2(-0x38D0))
                        * (root + root * (numerator / denominator))
                        - read_f64(sda2(-0x3940)));
            }
            if (hi as i32) < 1 {
                result = -result;
            }
        }
    } else if abs == 0x3FF0_0000 && (x.to_bits() as u32) == 0 {
        result = read_f64(sda2(-0x3948)) * x + read_f64(sda2(-0x3940)) * x;
    } else {
        platform::write_u32(G_ERRNO, 0x21);
        result = read_f64(D_8042_9BB8);
    }
    result
}

/// Computes `atan2(y, x)` by reducing to [`fn_802e7c14`] and fixing the
/// quadrant.
///
/// In plain English: "return the angle of the point `(x, y)` from the signs and
/// magnitudes, with the exact `0`, `pi/2` and `pi` results for the axis and
/// infinity cases."
///
/// # Original
/// - Ghidra: `FUN_802e4c04` at `0x802e4c04` (592 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4c04_FUN_802e4c04.c`
///
/// # Arguments
/// * `y` - `param_1`; the ordinate.
/// * `x` - `param_2`; the abscissa.
///
/// # Returns
/// The angle in `(-pi, pi]`; `NaN` when either input is a NaN.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - the quadrant word, every special-case table index and the two
/// `pi` compensations are literal; the `atan` call is rendered without
/// arguments in the `x == 1.0` branch, so its `y` argument is `// inferred:`
/// from the `|y/x|` call on the general path.
pub unsafe fn fn_802e4c04(y: f64, x: f64) -> f64 {
    let y_bits = y.to_bits();
    let x_bits = x.to_bits();
    let y_hi = (y_bits >> 32) as u32;
    let x_hi = (x_bits >> 32) as u32;
    let y_lo = y_bits as u32;
    let x_lo = x_bits as u32;
    let abs_y = y_hi & 0x7FFF_FFFF;
    let abs_x = x_hi & 0x7FFF_FFFF;
    let y_lsb = (y_lo | y_lo.wrapping_neg()) >> 31;
    let x_lsb = (x_lo | x_lo.wrapping_neg()) >> 31;

    if (abs_x | x_lsb) < 0x7FF0_0001 && (abs_y | y_lsb) < 0x7FF0_0001 {
        let quadrant = (y_hi >> 31) | ((x_hi >> 30) & 2);
        if x_hi == 0x3FF0_0000 && x_lo == 0 {
            return fn_802e7c14(y);
        }
        if abs_y == 0 && y_lo == 0 {
            return match quadrant {
                0 | 1 => y,
                2 => read_f64(sda2(-0x38C0)),
                _ => read_f64(sda2(-0x38B8)),
            };
        }
        let mut result;
        if abs_x == 0 && x_lo == 0 {
            result = if y < 0.0 {
                read_f64(sda2(-0x38B0))
            } else {
                read_f64(sda2(-0x38A8))
            };
            return result;
        }
        if abs_x == 0x7FF0_0000 {
            if abs_y == 0x7FF0_0000 {
                return match quadrant {
                    0 => read_f64(sda2(-0x38A0)),
                    1 => read_f64(sda2(-0x3898)),
                    2 => read_f64(sda2(-0x3890)),
                    _ => read_f64(sda2(-0x3888)),
                };
            }
            return match quadrant {
                0 => read_f64(sda2(-0x3880)),
                1 => read_f64(sda2(-0x3878)),
                2 => read_f64(sda2(-0x38C0)),
                _ => read_f64(sda2(-0x38B8)),
            };
        }
        if abs_y == 0x7FF0_0000 {
            result = if y < 0.0 {
                read_f64(sda2(-0x38B0))
            } else {
                read_f64(sda2(-0x38A8))
            };
        } else {
            let difference = (abs_y.wrapping_sub(abs_x) as i32) >> 20;
            result = if difference < 0x3D {
                if y < 0.0 && difference < -0x3C {
                    read_f64(sda2(-0x3880))
                } else {
                    fn_802e7c14((y / x).abs())
                }
            } else {
                read_f64(sda2(-0x38A8))
            };
            match quadrant {
                0 => {}
                1 => result = -result,
                2 => {
                    result = read_f64(sda2(-0x38C0))
                        - (result - read_f64(sda2(-0x3870)))
                }
                _ => {
                    result = (result - read_f64(sda2(-0x3870)))
                        - read_f64(sda2(-0x38C0))
                }
            }
        }
        result
    } else {
        y + x
    }
}

/// Four-byte branch to the `atan` body.
///
/// In plain English: "this entry point is `b 0x802E496C`, so it is `atan`."
///
/// # Original
/// - Ghidra: `thunk_FUN_802e496c` at `0x802e860c` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e860c_thunk_FUN_802e496c.c`
///
/// # Arguments
/// * `x` - `param_1`; forwarded unchanged to [`fn_802e496c`].
///
/// # Returns
/// `atan(x)`.
///
/// # Side effects
/// Same as [`fn_802e496c`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the export is the target body verbatim and the entry is a four-byte
/// branch.
pub unsafe fn fn_802e860c(x: f64) -> f64 {
    fn_802e496c(x)
}

/// Four-byte branch to the `atan2` body.
///
/// In plain English: "this entry point is `b 0x802E4C04`, so it is `atan2`."
///
/// # Original
/// - Ghidra: `thunk_FUN_802e4c04` at `0x802e8610` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8610_thunk_FUN_802e4c04.c`
///
/// # Arguments
/// * `y` - `param_1`; the ordinate.
/// * `x` - `param_2`; the abscissa.
///
/// # Returns
/// `atan2(y, x)`.
///
/// # Side effects
/// Same as [`fn_802e4c04`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the export is the target body verbatim and the entry is a four-byte
/// branch.
pub unsafe fn fn_802e8610(y: f64, x: f64) -> f64 {
    fn_802e4c04(y, x)
}

/// Four-byte branch to the `fmod` body.
///
/// In plain English: "this entry point is `b 0x802E4E5C`, so it is `fmod`."
///
/// # Original
/// - Ghidra: `thunk_FUN_802e4e5c` at `0x802e8614` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8614_thunk_FUN_802e4e5c.c`
///
/// # Arguments
/// * `x` - `param_1`; the dividend, forwarded to [`fn_802e4e5c`].
/// * `y` - `param_2`; the divisor.
///
/// # Returns
/// `fmod(x, y)`.
///
/// # Side effects
/// Same as [`fn_802e4e5c`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - the export is the target body verbatim and the entry is a four-byte
/// branch.
pub unsafe fn fn_802e8614(x: f64, y: f64) -> f64 {
    fn_802e4e5c(x, y)
}

/// Computes the floating-point remainder `x - n*y` for integral `n` (MSL
/// `fmod`).
///
/// In plain English: "normalise both mantissas, subtract the divisor from the
/// dividend one bit at a time, and reassemble the remainder with the dividend's
/// sign."
///
/// # Original
/// - Ghidra: `FUN_802e4e5c` at `0x802e4e5c` (896 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e4e5c_FUN_802e4e5c.c`
///
/// # Arguments
/// * `x` - `param_1`; the dividend.
/// * `y` - `param_2`; the divisor.
///
/// # Returns
/// `fmod(x, y)`, signed like `x`; `NaN` when `y` is zero or either input is
/// infinite / NaN.
///
/// # Side effects
/// None.
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - every threshold, the two normalisation loops, the bit-by-bit
/// subtraction and the final reassembly are literal; the decompiler's
/// `GQR0` probe is an omitted paired-single artifact. The shifts go through
/// [`srw`] so that the large counts behave like the PowerPC `srw` opcode the
/// original emits.
pub unsafe fn fn_802e4e5c(x: f64, y: f64) -> f64 {
    let x_bits = x.to_bits();
    let y_bits = y.to_bits();
    let x_hi = (x_bits >> 32) as u32;
    let y_hi = (y_bits >> 32) as u32;
    let mut x_lo = x_bits as u32;
    let mut y_lo = y_bits as u32;
    let abs_x = x_hi & 0x7FFF_FFFF;
    let abs_y = y_hi & 0x7FFF_FFFF;
    let sign_x = x_hi & 0x8000_0000;
    let lsb_y = (y_lo | y_lo.wrapping_neg()) >> 31;

    // `0x802FDFF0` is a two-double table holding `+0.0` / `-0.0`.
    let zero = |sign: u32| read_f64(0x802F_DFF0 + (sign >> 28) as usize);

    if (abs_y == 0 && y_lo == 0)
        || (abs_x as i32) > 0x7FEF_FFFF
        || (abs_y | lsb_y) > 0x7FF0_0000
    {
        return (x * y) / (x * y);
    }

    if (abs_x as i32) <= (abs_y as i32) {
        if (abs_x as i32) < (abs_y as i32) || x_lo < y_lo {
            return x;
        }
        if x_lo == y_lo {
            return zero(sign_x);
        }
    }

    // Exponent of the dividend.
    let mut ex_x;
    if abs_x < 0x0010_0000 {
        if abs_x == 0 {
            ex_x = -0x413;
            let mut probe = x_lo;
            while (probe as i32) > 0 {
                ex_x -= 1;
                probe = probe.wrapping_shl(1);
            }
        } else {
            ex_x = -0x3FE;
            let mut probe = abs_x.wrapping_shl(11) as i32;
            while probe > 0 {
                ex_x -= 1;
                probe = probe.wrapping_shl(1);
            }
        }
    } else {
        ex_x = ((abs_x as i32) >> 20) - 0x3FF;
    }

    // Exponent of the divisor.
    let mut ex_y;
    if abs_y < 0x0010_0000 {
        if abs_y == 0 {
            ex_y = -0x413;
            let mut probe = y_lo;
            while (probe as i32) > 0 {
                ex_y -= 1;
                probe = probe.wrapping_shl(1);
            }
        } else {
            ex_y = -0x3FE;
            let mut probe = y_hi.wrapping_shl(11) as i32;
            while probe > 0 {
                ex_y -= 1;
                probe = probe.wrapping_shl(1);
            }
        }
    } else {
        ex_y = ((abs_y as i32) >> 20) - 0x3FF;
    }

    // Normalise the dividend mantissa into `hi_x:x_lo`.
    let mut hi_x;
    if ex_x < -0x3FE {
        let shift = -ex_x - 0x3FE;
        if shift < 0x20 {
            hi_x = x_lo.wrapping_shr(0x20_u32.wrapping_sub(shift as u32));
            x_lo = x_lo.wrapping_shl(shift as u32);
            hi_x |= abs_x.wrapping_shl(shift as u32);
        } else {
            hi_x = x_lo.wrapping_shl((-ex_x - 0x41E) as u32);
            x_lo = 0;
        }
    } else {
        hi_x = (abs_x & 0x000F_FFFF) | 0x0010_0000;
    }

    // Normalise the divisor mantissa into `hi_y:y_lo`.
    let mut hi_y;
    if ex_y < -0x3FE {
        let shift = -ex_y - 0x3FE;
        if shift < 0x20 {
            hi_y = y_lo.wrapping_shr(0x20_u32.wrapping_sub(shift as u32));
            y_lo = y_lo.wrapping_shl(shift as u32);
            hi_y |= abs_y.wrapping_shl(shift as u32);
        } else {
            hi_y = y_lo.wrapping_shl((-ex_y - 0x41E) as u32);
            y_lo = 0;
        }
    } else {
        hi_y = (abs_y & 0x000F_FFFF) | 0x0010_0000;
    }

    let mut difference = ex_x - ex_y;
    let tail_shift = difference + 2;
    loop {
        if difference == 0 {
            let mut word = hi_x.wrapping_sub(hi_y);
            if word == 0
                && srw(x_lo, tail_shift) == srw(y_lo, tail_shift)
            {
                return zero(sign_x);
            }
            if x_lo < y_lo {
                word = word.wrapping_sub(1);
            }
            if (word as i32) >= 0 {
                x_lo = x_lo.wrapping_sub(y_lo);
                hi_x = word;
            }
            if hi_x == 0 && x_lo == 0 {
                return zero(sign_x);
            }
            while (hi_x as i32) < 0x0010_0000 {
                let carry = ((x_lo as i32) >> 31) as u32;
                x_lo = x_lo.wrapping_mul(2);
                ex_y -= 1;
                hi_x = hi_x.wrapping_mul(2).wrapping_sub(carry);
            }
            if ex_y < -0x3FE {
                let count = -ex_y - 0x3FE;
                let (high, low);
                if (count as i32) < 0x15 {
                    low = x_lo.wrapping_shr(count as u32)
                        | hi_x.wrapping_shl(0x20_u32.wrapping_sub(count as u32));
                    high = ((hi_x as i32) >> (count & 0x3F)) as u32;
                } else if (count as i32) < 0x20 {
                    high = sign_x;
                    low = hi_x.wrapping_shl(0x20_u32.wrapping_sub(count as u32))
                        | x_lo.wrapping_shr(count as u32);
                } else {
                    high = sign_x;
                    low = ((hi_x as i32) >> ((-ex_y - 0x41E) & 0x3F)) as u32;
                }
                return f64::from_bits(
                    (((high | sign_x) as u64) << 32) | low as u64,
                );
            }
            let high = hi_x.wrapping_sub(0x0010_0000)
                | (((ex_y + 0x3FF) as u32).wrapping_mul(0x0010_0000))
                | sign_x;
            return f64::from_bits(((high as u64) << 32) | x_lo as u64);
        }

        let mut word = hi_x.wrapping_sub(hi_y);
        let mut low = x_lo.wrapping_sub(y_lo);
        if word == 0 && srw(x_lo, tail_shift) == srw(y_lo, tail_shift) {
            return zero(sign_x);
        }
        if x_lo < y_lo {
            word = word.wrapping_sub(1);
        }
        if (word as i32) < 0 {
            hi_x = hi_x
                .wrapping_mul(2)
                .wrapping_sub(((x_lo as i32) >> 31) as u32);
            low = x_lo;
        } else {
            if word == 0 && low == 0 {
                return zero(sign_x);
            }
            hi_x = word
                .wrapping_mul(2)
                .wrapping_sub(((low as i32) >> 31) as u32);
        }
        x_lo = low.wrapping_mul(2);
        difference -= 1;
    }
}

/// Models the PowerPC `srw` shift the decompiled `>>` stands for.
///
/// `srw` uses `count & 63` and produces 0 for every count of 32 or more, which
/// differs from Rust's `>>` (masked to 31). The normalisation tests in
/// [`fn_802e4e5c`] rely on that behaviour for large exponent differences.
///
/// # Original
/// Host-side helper; models the `srw` instruction emitted by
/// `802e4e5c_FUN_802e4e5c.c`.
///
/// # Safety
/// None.
#[inline]
fn srw(value: u32, count: i32) -> u32 {
    let shift = (count as u32) & 63;
    if shift >= 32 {
        0
    } else {
        value >> shift
    }
}

/// Formats a bounded buffer through the MSL formatter with a fresh `va_list`
/// (the variadic `snprintf` entry).
///
/// In plain English: "save the incoming argument registers into a `va_list`,
/// run the formatter into `buf`, then NUL-terminate the result."
///
/// # Original
/// - Ghidra: `FUN_802e16e8` at `0x802e16e8` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e16e8_FUN_802e16e8.c`
///
/// # Arguments
/// * `buf` - `param_9` (`r3`); the destination buffer.
/// * `size` - `param_10` (`r4`); the buffer capacity.
/// * `format` - `param_11` (`r5`); the format string.
/// * `gpr` - `param_12..16` (`r6..r10`); the remaining integer / pointer
///   arguments. The original receives them in registers, so they are modelled
///   as an explicit array.
/// * `fpr` - `param_1..8` (`f1..f8`); the floating-point arguments.
/// * `stack` - the caller's stack-argument words, addressed by the `va_list`
///   overflow pointer (`r1+152` in the original).
///
/// # Returns
/// The byte count the formatter produced, or -1 on a write failure.
///
/// # Side effects
/// Writes the formatted text and the terminator into `buf`; may call the append
/// callback `0x802E14F4` through the formatter.
///
/// # Safety
/// `buf` must cover `size` bytes and `stack` must hold the caller's words.
///
/// # Confidence
/// high - the save areas, the `0x03000000` `va_list` word (gpr index 3, fpr
/// index 0), the `r1+152` overflow pointer, the formatter call and the two
/// termination branches are literal; the `$cr1` guard only skips a save no
/// format on this path reads.
pub unsafe fn fn_802e16e8(
    buf: usize,
    size: u32,
    format: u32,
    gpr: [u32; 5],
    fpr: [f64; 8],
    stack: [u32; 8],
) -> u32 {
    // -0x28: the append context {buffer, capacity}; -0x20: padding;
    // -0x1c: the va_list {register word, overflow area, register save area}.
    let mut block = [0_u32; 6];
    block[0] = buf as u32;
    block[1] = size;
    block[2] = 0;
    block[3] = 0x0300_0000;

    // -0x88: the eight saved GPRs (r3..r10).
    let mut gpr_area = [0_u32; 8];
    gpr_area[0] = buf as u32;
    gpr_area[1] = size;
    gpr_area[2] = format;
    gpr_area[3..8].copy_from_slice(&gpr);

    // -0x88: the eight saved GPRs (r3..r10) immediately followed by the eight
    // saved FPRs (f1..f8); the formatter reaches the FPR area as
    // `register_save + 0x20`.
    let mut save_area = [0_u32; 24];
    save_area[0] = buf as u32;
    save_area[1] = size;
    save_area[2] = format;
    save_area[3..8].copy_from_slice(&gpr);
    let mut index = 0;
    while index < 8 {
        let slot = save_area.as_mut_ptr().add(8 + index * 2) as usize;
        write_f64(slot, fpr[index]);
        index += 1;
    }
    let mut overflow = stack;
    block[4] = overflow.as_mut_ptr() as u32;
    block[5] = save_area.as_mut_ptr() as u32;

    let written = platform::call5(
        FN_802E0BF0,
        FN_802E14F4 as u32,
        block.as_mut_ptr() as u32,
        format,
        block.as_mut_ptr().add(3) as u32,
        0,
    );

    if buf != 0 {
        if written < size {
            platform::write_u8(buf + written as usize, 0);
        } else if size != 0 {
            platform::write_u8(buf + size as usize - 1, 0);
        }
    }
    written
}

/// Formats an unbounded buffer through the MSL formatter with a fresh
/// `va_list` (the variadic `sprintf` entry).
///
/// In plain English: "save the incoming argument registers, run the formatter
/// with an unlimited append context, then NUL-terminate the result."
///
/// # Original
/// - Ghidra: `FUN_802e17dc` at `0x802e17dc` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e17dc_FUN_802e17dc.c`
///
/// # Arguments
/// * `buf` - `param_9` (`r3`); the destination buffer.
/// * `format` - `param_10` (`r4`); the format string.
/// * `gpr` - `param_11..16` (`r5..r10`); the remaining integer / pointer
///   arguments.
/// * `fpr` - `param_1..8` (`f1..f8`); the floating-point arguments.
/// * `stack` - the caller's stack-argument words, addressed by the `va_list`
///   overflow pointer (`r1+168` in the original).
///
/// # Returns
/// The byte count the formatter produced, or -1 on a write failure.
///
/// # Side effects
/// Writes the formatted text and the terminator into `buf`.
///
/// # Safety
/// `buf` must have room for the whole expansion and `stack` must hold the
/// caller's words.
///
/// # Confidence
/// high - the `-1` capacity word, the `0x02000000` `va_list` word, the
/// `r1+168` overflow pointer and the two termination branches are literal; the
/// `buf - 2` store on the error path is copied from the export verbatim.
pub unsafe fn fn_802e17dc(
    buf: usize,
    format: u32,
    gpr: [u32; 6],
    fpr: [f64; 8],
    stack: [u32; 8],
) -> i32 {
    // -0x38: the three-word append context {buffer, -1, 0}; -0x2c: the
    // va_list {register word, overflow area, register save area}.
    let mut block = [0_u32; 6];
    block[0] = buf as u32;
    block[1] = 0xFFFF_FFFF;
    block[2] = 0;
    block[3] = 0x0200_0000;
    let mut overflow = stack;
    block[4] = overflow.as_mut_ptr() as u32;

    // -0x98: the eight saved GPRs (r3..r10) immediately followed by the eight
    // saved FPRs (f1..f8).
    let mut save_area = [0_u32; 24];
    save_area[0] = buf as u32;
    save_area[1] = format;
    save_area[2..8].copy_from_slice(&gpr);
    let mut index = 0;
    while index < 8 {
        let slot = save_area.as_mut_ptr().add(8 + index * 2) as usize;
        write_f64(slot, fpr[index]);
        index += 1;
    }
    block[5] = save_area.as_mut_ptr() as u32;

    let written = platform::call5(
        FN_802E0BF0,
        FN_802E14F4 as u32,
        block.as_mut_ptr() as u32,
        format,
        block.as_mut_ptr().add(3) as u32,
        0,
    ) as i32;

    if buf != 0 {
        if written == -1 {
            platform::write_u8(buf - 2, 0);
        } else {
            platform::write_u8(buf + written as usize, 0);
        }
    }
    written
}

/// `0x802E14F4` - the byte-buffer append callback the formatter writes through.
///
/// # Original
/// - Ghidra: `FUN_802e14f4` at `0x802e14f4` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e14f4_FUN_802e14f4.c`
pub const FN_802E14F4: usize = 0x802E_14F4;

/// Clears the two "exception taken" flags the interrupt handlers test.
///
/// In plain English: "reset `0x804270C8` and `0x804270C9` to zero."
///
/// # Original
/// - Ghidra: `FUN_802ea6bc` at `0x802ea6bc` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea6bc_FUN_802ea6bc.c`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes [`G_804270C8`] and [`G_804270C9`].
///
/// # Safety
/// None.
///
/// # Confidence
/// medium - both byte stores are literal and the twin of the same pair in
/// [`fn_802eccb4`]; `// partial:` the export attributes only these two stores
/// to the function although `functions.tsv` gives it a 356-byte extent, so the
/// remaining bytes are not covered by any instruction in the decompilation.
pub unsafe fn fn_802ea6bc() {
    platform::write_u8(G_804270C8, 0);
    platform::write_u8(G_804270C9, 0);
}

/// Reads or writes a GPR-style eight-byte group for each index in a range.
///
/// In plain English: "for every index from `first` to `last`, build the
/// `mfspr` / `mtspr` patch for it and move one 8-byte group between the TRK
/// buffer and the target."
///
/// # Original
/// - Ghidra: `FUN_802ec5bc` at `0x802ec5bc` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec5bc_FUN_802ec5bc.c`
///
/// # Arguments
/// * `first` - `param_1`; first index.
/// * `last` - `param_2`; last index (must be below `0x22`).
/// * `buffer` - `param_3`; the TRK payload buffer.
/// * `out_len` - `param_4`; receives the byte count, or 0 on failure.
/// * `direction` - `param_5`; 0 reads the target into the payload, non-zero
///   writes the payload into the target.
///
/// # Returns
/// 0 on success, `0x701` for a bad range, `0x702` when the context flag is set.
///
/// # Side effects
/// Saves and restores `0x80351620..0x8035162C`, sets MSR bit `0x2000` through
/// [`fn_802ec1a8`] for the duration, executes the patched block through
/// [`fn_802ed6ec`] and appends to the payload through [`fn_802eabe8`].
///
/// # Safety
/// `out_len` must be writable and `buffer` must accept the appended groups.
///
/// # Confidence
/// medium - the range test, the `+8` byte accounting, the MSR bit, the
/// save / restore bracket and the `0x702` flag are literal; the dropped fourth
/// argument of [`fn_802eabe8`] is unused by that body, so 0 is passed.
pub unsafe fn fn_802ec5bc(
    first: u32,
    last: u32,
    buffer: u32,
    out_len: usize,
    direction: i32,
) -> u32 {
    let mut result;
    if last < 0x22 {
        let saved20 = platform::read_u32(G_80351620);
        let saved24 = platform::read_u32(G_80351624);
        let saved28 = platform::read_u32(G_80351628);
        let saved2c = platform::read_u32(G_8035162C);
        platform::write_u32(
            G_8035162C,
            (saved2c & 0xFF00_0000) | ((saved2c & 0x0000_FFFF) << 8),
        );

        let msr = fn_802ec1a0();
        fn_802ec1a8(msr | 0x2000);
        platform::write_u32(out_len, 0);
        result = 0;
        let mut index = first;
        while index <= last && result == 0 {
            let mut payload = [0_u32; 2];
            if direction == 0 {
                fn_802eae30(buffer as usize, payload.as_mut_ptr() as usize);
                result = fn_802ed6ec(payload.as_mut_ptr() as usize, index, 0);
            } else {
                fn_802ed6ec(payload.as_mut_ptr() as usize, index, direction);
                result = fn_802eabe8(buffer as usize, 0, payload[0], payload[1]);
            }
            index = index.wrapping_add(1);
            platform::write_u32(out_len, platform::read_u32(out_len).wrapping_add(8));
        }
        if (platform::read_u32(G_8035162C) >> 16) & 0xFF != 0 {
            platform::write_u32(out_len, 0);
            result = 0x702;
        }

        platform::write_u32(G_80351620, saved20);
        platform::write_u32(G_80351624, saved24);
        platform::write_u32(G_80351628, saved28);
        platform::write_u32(G_8035162C, saved2c);
    } else {
        result = 0x701;
    }
    result
}

/// Reads or writes eight bytes per SPR index over a range, through the
/// `mfspr` / `mtspr` patch blocks.
///
/// In plain English: "flush the FPU state with three patch blocks, then for
/// every index from `first` to `last` transfer one 8-byte group between the TRK
/// buffer and the target."
///
/// # Original
/// - Ghidra: `FUN_802ec85c` at `0x802ec85c` (368 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec85c_FUN_802ec85c.c`
///
/// # Arguments
/// * `first` - `param_1`; first index.
/// * `last` - `param_2`; last index (must be below `0x20`).
/// * `buffer` - `param_3`; the TRK payload buffer.
/// * `out_len` - `param_4`; receives the byte count, or 0 on failure.
/// * `direction` - `param_5`; 0 reads the target into the payload, non-zero
///   writes the payload into the target.
///
/// # Returns
/// 0 on success, `0x701` for a bad range, `0x702` when the context flag is set.
///
/// # Side effects
/// Executes three [`fn_802ed520`] blocks (`spr` `0x398` write, `0x398` read,
/// `0x390` write), saves and restores `0x80351620..0x8035162C`, and appends to
/// the payload through [`fn_802eabe8`].
///
/// # Safety
/// `out_len` must be writable and `buffer` must accept the appended groups.
///
/// # Confidence
/// medium - the range test, the three patch blocks, the `+8` accounting and the
/// save / restore bracket are literal; the first block's `| 0xA0000000` is
/// overwritten by the second builder (the export shows it anyway) and the
/// payload words are read back from the stack slot [`fn_802ed5fc`] fills.
pub unsafe fn fn_802ec85c(
    first: u32,
    last: u32,
    buffer: u32,
    out_len: usize,
    direction: i32,
) -> u32 {
    let mut result;
    if last < 0x20 {
        let saved20 = platform::read_u32(G_80351620);
        let saved24 = platform::read_u32(G_80351624);
        let saved28 = platform::read_u32(G_80351628);
        let saved2c = platform::read_u32(G_8035162C);
        platform::write_u32(
            G_8035162C,
            (saved2c & 0xFF00_0000) | ((saved2c & 0x0000_FFFF) << 8),
        );

        let mut patch = [0_u32; 10];
        fn_802ed520(patch.as_mut_ptr() as u32, 0x398, 1);
        patch[0] |= 0xA000_0000;
        fn_802ed520(patch.as_mut_ptr() as u32, 0x398, 0);
        patch[0] = 0;
        fn_802ed520(patch.as_mut_ptr() as u32, 0x390, 0);

        platform::write_u32(out_len, 0);
        result = 0;
        let mut index = first;
        while index <= last && result == 0 {
            let mut payload = [0_u32; 2];
            if direction == 0 {
                fn_802eae30(buffer as usize, payload.as_mut_ptr() as usize);
                // observed: the export assigns the void call's result; the
                // builder returns nothing, so the loop keeps `result == 0`.
                fn_802ed5fc(payload.as_mut_ptr() as u32, index as i32, 0);
            } else {
                fn_802ed5fc(payload.as_mut_ptr() as u32, index as i32, direction);
                result = fn_802eabe8(buffer as usize, 0, payload[0], payload[1]);
            }
            index = index.wrapping_add(1);
            platform::write_u32(out_len, platform::read_u32(out_len).wrapping_add(8));
        }
        if (platform::read_u32(G_8035162C) >> 16) & 0xFF != 0 {
            platform::write_u32(out_len, 0);
            result = 0x702;
        }

        platform::write_u32(G_80351620, saved20);
        platform::write_u32(G_80351624, saved24);
        platform::write_u32(G_80351628, saved28);
        platform::write_u32(G_8035162C, saved2c);
    } else {
        result = 0x701;
    }
    result
}

// ---------------------------------------------------------------------------
// Wave 104 - the `pow` kernel, its tail and the format-conversion routines.
// ---------------------------------------------------------------------------
//
// Wave 104 finished the last reconstructible members of the `0x802E0000`
// bucket: the MSL `pow` body the `pow` tail forwards to, and the two `__cvt`
// helpers that the `printf` drivers call. Everything else that is still
// untranslated in the bucket is either a live-register save/restore
// (`mfspr`, `mffs`, `mfmsr`), a cache-instruction loop or an instruction
// sequence Ghidra renders as removed code; those are listed in
// `docs/COVERAGE_WAVE104_MAIN_LOOP.md`.

/// Keeps only the high 32 bits of `value`, zeroing the low word.
///
/// This is the export's `(ulonglong)x & 0xffffffff00000000` mask, the
/// "high part" of the double-double splits the MSL math kernels use.
///
/// # Original
/// Host-side helper; no binary address. Models the high-word copy the export
/// writes as a `CONCAT44(x, 0)` cast.
///
/// # Confidence
/// high - mechanical mask of the IEEE-754 bit pattern.
fn high_word(value: f64) -> f64 {
    f64::from_bits(value.to_bits() & 0xFFFF_FFFF_0000_0000)
}

/// Raises `x` to the power `y` (MSL `pow`).
///
/// In plain English: "reduce `|x|` to a significand and a base-2 exponent,
/// evaluate the `log2` polynomial, multiply that by `y`, then rebuild the
/// result through the `exp2` polynomial" - with explicit shortcuts for zero,
/// one, infinity and NaN, and the integer test that decides whether a negative
/// base is legal.
///
/// # Original
/// - Ghidra: `FUN_802e55a4` at `0x802e55a4` (2068 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e55a4_FUN_802e55a4.c`
///
/// # Arguments
/// * `x` - `param_1`; the base, in `f1`.
/// * `y` - `param_2`; the exponent, in `f2`.
///
/// # Returns
/// `x` raised to `y`, with the MSL `errno` / `+inf` / `0` results for the
/// domain and range edges (a negative base with a non-integer exponent writes
/// `EDOM` and returns the `0x80429BB8` NaN).
///
/// # Side effects
/// Writes [`G_ERRNO`] (`0x21`, `EDOM`) when a negative base meets a
/// non-integer exponent.
///
/// # Safety
/// None; the two callees are the bodies in this file.
///
/// # Confidence
/// medium - every cut, polynomial, table index and sign fixup is literal;
/// `// inferred:` two arguments are not visible in the export, so they are
/// reconstructed from the call sites: the `thunk_FUN_802e8620()` with no
/// argument is `sqrt(x)` (the `|x|` fast path for `y == 0.5`), and
/// `FUN_802e4660()` with no argument is `scalbn(result, shift)` (the
/// underflow repair). The `ldexpf` / `GQR0` probes at the top and bottom are
/// paired-single artifacts of the decompiler and are omitted, as on the
/// Wave 95 / 99 kernels.
pub unsafe fn fn_802e55a4(x: f64, y: f64) -> f64 {
    let x_bits = x.to_bits();
    let y_bits = y.to_bits();
    let x_hi = (x_bits >> 32) as u32;
    let x_lo = x_bits as u32;
    let x_abs_hi = x_hi & 0x7FFF_FFFF;
    let y_hi = (y_bits >> 32) as u32;
    let y_lo = y_bits as u32;
    let y_abs_hi = y_hi & 0x7FFF_FFFF;
    let x_negative = (x_bits as i64) < 0;
    let y_negative = (y_bits as i64) < 0;

    // `.sdata2` constants, in the order the export first loads them.
    let one = read_f64(sda2(-0x37B8));
    let zero = read_f64(sda2(-0x37B0));
    let huge = read_f64(sda2(-0x37A8));

    // y is +-0: the result is exactly 1.
    if (y_bits & 0x7FFF_FFFF_0000_0000) == 0 && y_lo == 0 {
        return one;
    }

    // A NaN or an infinity in either argument: the result is `x + y`.
    let x_finite = x_abs_hi < 0x7FF0_0001 && (x_abs_hi != 0x7FF0_0000 || x_lo == 0);
    let y_finite = y_abs_hi < 0x7FF0_0001 && (y_abs_hi != 0x7FF0_0000 || y_lo == 0);
    if !(x_finite && y_finite) {
        return x + y;
    }

    // `parity` is the export's `iVar2`: 0 = non-integer, 1 = odd integer,
    // 2 = even integer. It is only computed for a negative base.
    let mut parity = 0_i32;
    if x_negative {
        if y_abs_hi < 0x4340_0000 {
            if y_abs_hi > 0x3FEF_FFFF {
                let exponent = (y_abs_hi as i32 >> 20) - 0x3FF;
                if exponent < 0x15 {
                    let shift = (0x14 - exponent) as u32 & 0x3F;
                    let quotient = (y_abs_hi as i32 >> shift) as u32;
                    if y_lo == 0 && y_abs_hi == quotient.wrapping_shl(shift) {
                        parity = 2 - (quotient & 1) as i32;
                    }
                } else {
                    let shift = (0x34 - exponent) as u32 & 0x3F;
                    let quotient = y_lo >> shift;
                    if y_lo == quotient.wrapping_shl(shift) {
                        parity = 2 - (quotient & 1) as i32;
                    }
                }
            }
        } else {
            parity = 2;
        }
    }

    // Integral `y` with a zero low word: the 0, 1, 2 and 1/2 shortcuts.
    if y_lo == 0 {
        if y_abs_hi == 0x7FF0_0000 {
            // |y| is infinite.
            if x_abs_hi == 0x3FF0_0000 && x_lo == 0 {
                return y - y;
            }
            if x_abs_hi < 0x3FF0_0000 {
                return if y_negative { -y } else { zero };
            }
            return if y_negative { zero } else { y };
        }
        if y_abs_hi == 0x3FF0_0000 {
            // |y| == 1.
            return if y_negative { one / x } else { x };
        }
        if y_hi == 0x4000_0000 {
            // y == 2.0.
            return x * x;
        }
        if y_hi == 0x3FE0_0000 && !x_negative {
            // y == 0.5 with a non-negative base.
            return fn_802e8620(x);
        }
    }

    let mut magnitude = x.abs();

    // |x| is 0, 1 or infinite, and the low word is zero.
    if x_lo == 0
        && (x_abs_hi == 0x7FF0_0000
            || (x_bits & 0x7FFF_FFFF_0000_0000) == 0
            || x_abs_hi == 0x3FF0_0000)
    {
        if y_negative {
            magnitude = one / magnitude;
        }
        if x_negative {
            if x_abs_hi == 0x3FF0_0000 && parity == 0 {
                // `1^y` with a non-integer exponent is NaN.
                magnitude = (magnitude - magnitude) / (magnitude - magnitude);
            } else if parity == 1 {
                magnitude = -magnitude;
            }
        }
        return magnitude;
    }

    // Negative base, non-integer exponent: domain error.
    if x_negative && parity == 0 {
        platform::write_u32(G_ERRNO, 0x21);
        return read_f64(D_8042_9BB8);
    }

    // `head` / `tail` are the export's `dStack_90` / `dVar10` pair: the
    // high and low halves of `y * log2(|x|)`, or of `y * log1p(x - 1)`.
    let head: f64;
    let tail: f64;

    if y_abs_hi < 0x41E0_0001 {
        // --- main path: normalise |x| and evaluate log2 ---------------------
        let mut exponent_bias = 0_i32;
        let mut mantissa_hi = x_abs_hi;
        if x_abs_hi < 0x0010_0000 {
            // Subnormal: scale by 2^54 first.
            exponent_bias = -0x35;
            let scaled = magnitude * read_f64(sda2(-0x3770));
            magnitude = scaled;
            mantissa_hi = (scaled.to_bits() >> 32) as u32;
        }

        let fraction = mantissa_hi & 0x000F_FFFF;
        let mut normalized_hi = fraction | 0x3FF0_0000;
        exponent_bias += (mantissa_hi as i32) >> 20;
        let mut table_exponent = exponent_bias.wrapping_sub(0x3FF) as u32;
        let table_index: usize;
        if fraction < 0x0003_988F {
            table_index = 0;
        } else if fraction < 0x000B_B67A {
            table_index = 1;
        } else {
            // Round up into the next binade.
            normalized_hi = normalized_hi.wrapping_sub(0x0010_0000);
            table_index = 0;
            table_exponent = exponent_bias.wrapping_sub(0x3FE) as u32;
        }

        let normalized_lo = magnitude.to_bits() as u32;
        let normalized =
            f64::from_bits(((normalized_hi as u64) << 32) | normalized_lo as u64);

        // The three-entry log2 table at `0x802FE000` / `0x802FE010` /
        // `0x802FE020` (stride 8).
        let at_low = read_f64(0x802F_E000 + table_index * 8);
        let at_mid = read_f64(0x802F_E010 + table_index * 8);
        let at_high = read_f64(0x802F_E020 + table_index * 8);

        let inverse = one / (normalized + at_low);
        let reduced = (normalized - at_low) * inverse;
        let split_base = f64::from_bits(
            ((((((normalized_hi as i32) >> 1) as u32) | 0x2000_0000_u32)
                .wrapping_add((table_index as u32).wrapping_mul(0x0004_0000))
                .wrapping_add(0x0008_0000)) as u64)
                << 32
                | (zero.to_bits() & 0xFFFF_FFFF),
        );
        let reduced_hi = high_word(reduced);
        let squared = reduced * reduced;

        let mut low_part = inverse
            * (((normalized - at_low) - reduced_hi * split_base)
                - reduced_hi * (normalized - (split_base - at_low)));

        // log2 polynomial: six coefficients at `-0x3768..-0x3740`.
        let poly = squared
            * squared
            * (read_f64(sda2(-0x3768))
                + squared
                    * (read_f64(sda2(-0x3760))
                        + squared
                            * (read_f64(sda2(-0x3758))
                                + squared
                                    * (read_f64(sda2(-0x3750))
                                        + squared
                                            * (read_f64(sda2(-0x3748))
                                                + read_f64(sda2(-0x3740)) * squared)))))
            + low_part * (reduced_hi + reduced);

        let split = high_word(read_f64(sda2(-0x3738)) + reduced_hi * reduced_hi + poly);
        low_part = low_part * split
            + (poly - ((split - read_f64(sda2(-0x3738))) - reduced_hi * reduced_hi))
                * reduced;
        let mid = high_word(reduced_hi * split + low_part);
        let exp_scaled = read_f64(sda2(-0x3730)) * mid;
        let exp_extra = at_high
            + read_f64(sda2(-0x3728)) * mid
            + read_f64(sda2(-0x3720))
                * (low_part - (mid - reduced_hi * split));

        let exponent_f64 = signed_to_f64_bits(table_exponent as i32)
            - read_f64(sda2(-14000));
        head = high_word(exponent_f64 + exp_scaled + exp_extra + at_mid);
        tail = exp_extra - (((head - exponent_f64) - at_mid) - exp_scaled);
    } else {
        // --- y too large for the log path: range shortcuts ------------------
        if y_abs_hi > 0x43F0_0000 {
            if x_abs_hi < 0x3FF0_0000 {
                return if y_negative { huge } else { zero };
            }
            if (y_hi as i32) < 1 {
                return zero;
            }
            return huge;
        }
        if x_abs_hi < 0x3FEF_FFFF {
            return if y_negative { huge } else { zero };
        }
        if x_abs_hi > 0x3FF0_0000 {
            return if (y_hi as i32) < 1 { zero } else { huge };
        }

        // `x` is within one ulp of 1: evaluate log1p(x - 1) instead.
        let delta = x - one;
        let linear = read_f64(sda2(-0x3788)) * delta;
        let correction = read_f64(sda2(-0x3780)) * delta
            - read_f64(sda2(-0x3778))
                * delta
                * delta
                * (read_f64(sda2(-0x37A0))
                    - delta
                        * (read_f64(sda2(-0x3798))
                            - read_f64(sda2(-0x3790)) * delta));
        head = high_word(linear + correction);
        tail = correction - (head - linear);
    }

    // Apply the sign of a negative base with an odd exponent.
    let mut sign_scale = one;
    if x_negative && parity == 1 {
        sign_scale = read_f64(sda2(-0x3718));
    }

    // `y * log2(|x|)` in double-double: high word of `y` plus its low part.
    let y_high = f64::from_bits(y_bits & 0xFFFF_FFFF_0000_0000);
    let product_hi = y_high * head;
    let mut product_lo = head * (y - y_high) + y * tail;
    let product = product_lo + product_hi;
    let mut product_hi_word = (product.to_bits() >> 32) as u32;

    // Range cuts just below 2^1024 and around the smallest subnormal.
    if (product.to_bits() as i64) < 0x4090_0000_0000_0000 {
        if (product_hi_word & 0x7FFF_FFFF) > 0x4090_CBFF {
            if product != -1075.0 {
                return read_f64(sda2(-0x3700)) * read_f64(sda2(-0x3700)) * sign_scale;
            }
            if product_lo <= -1075.0 - product_hi {
                return read_f64(sda2(-0x3700)) * read_f64(sda2(-0x3700)) * sign_scale;
            }
        }
    } else {
        if product != 1024.0 {
            return read_f64(sda2(-0x3710)) * read_f64(sda2(-0x3710)) * sign_scale;
        }
        if 1024.0 - product_hi < read_f64(sda2(-0x3708)) + product_lo {
            return read_f64(sda2(-0x3710)) * read_f64(sda2(-0x3710)) * sign_scale;
        }
    }

    // Peel the integer part of the exponent into `shift_count`.
    let mut shift_count = 0_i32;
    if (product_hi_word & 0x7FFF_FFFF) > 0x3FE0_0000 {
        let round_shift = ((product_hi_word >> 20) & 0x7FF).wrapping_sub(0x3FE) & 0x3F;
        product_hi_word = product_hi_word.wrapping_add(0x0010_0000_u32 >> round_shift);
        let exponent = (((product_hi_word & 0x7FFF_FFFF) as i32) >> 20) as u32;
        let exponent = exponent.wrapping_sub(0x3FF);
        let split = f64::from_bits(
            (((product_hi_word & !(0x000F_FFFF_u32 >> (exponent & 0x3F))) as u64) << 32)
                | (one.to_bits() & 0xFFFF_FFFF),
        );
        shift_count = ((((product_hi_word & 0x000F_FFFF) | 0x0010_0000) as i32)
            >> (0x14_u32.wrapping_sub(exponent) & 0x3F)) as i32;
        if (product.to_bits() as i64) < 0 {
            shift_count = -shift_count;
        }
        product_lo -= split;
    }

    // --- exp2 polynomial ---------------------------------------------------
    let frac_hi = high_word(product_lo + product_hi);
    let t1 = read_f64(sda2(-0x36F8)) * frac_hi;
    let t2 = read_f64(sda2(-0x36F0)) * (product_lo - (frac_hi - product_lo))
        + read_f64(sda2(-0x36E8)) * frac_hi;
    let exp_arg = t1 + t2;
    let square = exp_arg * exp_arg;
    let exp_lo = t2 - (exp_arg - t1);
    let exp_poly = exp_arg
        - square
            * (read_f64(sda2(-0x36E0))
                + square
                    * (read_f64(sda2(-0x36D8))
                        + square
                            * (read_f64(sda2(-0x36D0))
                                + square
                                    * (read_f64(sda2(-0x36C8))
                                        + read_f64(sda2(-0x36C0)) * square))));
    let exp_value = one
        - (((exp_arg * exp_poly) / (exp_poly - read_f64(sda2(-0x36B8)))
            - (exp_lo + exp_arg * exp_lo))
            - exp_arg);

    // Re-apply the peeled exponent, falling back to `scalbn` on underflow.
    let exp_hi = (exp_value.to_bits() >> 32) as u32;
    let biased = exp_hi.wrapping_add((shift_count as u32).wrapping_mul(0x0010_0000));
    let scaled = if ((biased as i32) >> 20) < 1 {
        fn_802e4660(exp_value, shift_count)
    } else {
        f64::from_bits(((biased as u64) << 32) | (exp_value.to_bits() & 0xFFFF_FFFF))
    };

    sign_scale * scaled
}

/// Four-byte tail of [`fn_802e55a4`] (`b 0x802E55A4`).
///
/// # Original
/// - Ghidra: `thunk_FUN_802e55a4` at `0x802e861c` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e861c_thunk_FUN_802e55a4.c`
///
/// # Arguments
/// * `x`, `y` - forwarded unchanged to [`fn_802e55a4`].
///
/// # Returns
/// `x` raised to `y`.
///
/// # Side effects
/// Same as [`fn_802e55a4`].
///
/// # Safety
/// None.
///
/// # Confidence
/// high - `functions.tsv` gives the entry point four bytes and the export is
/// the shared `pow` body, i.e. a single `b 0x802E55A4`.
pub unsafe fn fn_802e861c(x: f64, y: f64) -> f64 {
    fn_802e55a4(x, y)
}

/// Formats one `double` into a narrow-character buffer (MSL `__cvt`).
///
/// In plain English: "convert the value to 32 decimal digits with
/// `FUN_802DE370`, trim the trailing zeros, then lay the digits out lazily
/// backwards from `buf_end` according to the `%g` / `%e` / `%f` conversion and
/// the precision in the spec block."
///
/// # Original
/// - Ghidra: `FUN_802e0454` at `0x802e0454` (1948 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e0454_FUN_802e0454.c`
///
/// # Arguments
/// * `value` - `param_1`; the number, in `f1`.
/// * `buf_end` - `param_2`; one past the last byte the caller reserved. The
///   routine writes backwards from `buf_end - 1` and returns the first byte.
/// * `spec` - `param_3`; the 16-byte spec block the parser filled:
///   `+1` sign mode (0 none, 1 `+`, 2 space), `+3` alternate form,
///   `+5` conversion character, `+0xC` precision (word).
///
/// # Returns
/// The first byte of the finished string, or `0` when the field would exceed
/// `0x1FD` characters or the conversion is not `e` / `E` / `f` / `F` / `g` /
/// `G`.
///
/// # Side effects
/// Writes the digit tail of `out` through [`fn_802e032c`], writes the finished
/// text backwards into `buf_end`, and may rewrite `spec+5` (`g` -> `e`) and
/// `spec+0xC` (the effective precision).
///
/// # Safety
/// `buf_end` must have as many writable bytes below it as the field needs;
/// `spec` must be the 16-byte block and `value` finite (the `INF` / `NAN`
/// spellings are handled, but the caller passes the raw double).
///
/// # Confidence
/// medium - the four branches, every copy loop and both `FUN_802e032c`
/// rounding positions are literal. `// inferred:` the second argument of the
/// two one-argument `FUN_802e032c` calls is the rounding position
/// (`precision` and `precision + 1`), recovered from the `cmpw` operands at
/// `0x802e0794` / `0x802e0850`. The eight-byte unrolled fills are written as
/// byte loops; they store the same bytes.
pub unsafe fn fn_802e0454(value: f64, buf_end: u32, spec: usize) -> u32 {
    let decimal_point = platform::read_u8(platform::read_u32(0x8035_0C80) as usize);
    let precision = platform::read_i32(spec + 0xC);
    if precision > 0x1FD {
        return 0;
    }

    // `local_48`: the dtoa request (`+2` = 32 significant digits).
    let mut dtoa_in = [0_u8; 4];
    dtoa_in[3] = 0x20;
    // `local_44`: sign at `+0`, exponent at `+2`, digit count at `+4`,
    // digits at `+5`.
    let mut out = [0_u8; 48];
    platform::call2(
        0x802D_E370,
        dtoa_in.as_mut_ptr() as u32,
        out.as_mut_ptr() as u32,
    );

    // Trim trailing zeros, keeping at least one digit.
    let mut scan = 5 + out[4] as usize;
    while out[4] > 1 {
        scan -= 1;
        if out[scan] != b'0' {
            break;
        }
        out[4] -= 1;
        let exponent = i16::from_be_bytes([out[2], out[3]]) + 1;
        let bytes = exponent.to_be_bytes();
        out[2] = bytes[0];
        out[3] = bytes[1];
    }

    if out[5] == b'0' {
        out[2] = 0;
        out[3] = 0;
    } else if out[5] == b'I' {
        let upper = (ctype_flags(platform::read_u8(spec + 5) as u32) & 0x200) != 0;
        if value < read_f64(sda2(-0x39E0)) {
            let dst = buf_end.wrapping_sub(5);
            fn_802e1938(
                dst as usize,
                if upper { 0x802F_D9C0 } else { 0x802F_D9C5 },
            );
            return dst;
        }
        let dst = buf_end.wrapping_sub(4);
        fn_802e1938(
            dst as usize,
            if upper { 0x802F_D9CA } else { 0x802F_D9CE },
        );
        return dst;
    } else if out[5] == b'N' {
        let upper = (ctype_flags(platform::read_u8(spec + 5) as u32) & 0x200) != 0;
        if out[0] == 0 {
            let dst = buf_end.wrapping_sub(4);
            fn_802e1938(
                dst as usize,
                if upper { 0x802F_D9DC } else { 0x802F_D9E0 },
            );
            return dst;
        }
        let dst = buf_end.wrapping_sub(5);
        fn_802e1938(
            dst as usize,
            if upper { 0x802F_D9D2 } else { 0x802F_D9D7 },
        );
        return dst;
    }

    // `%g` / `%e` / `%f` selection. `out[2..4]` becomes the position of the
    // decimal point relative to the first digit (the export's `local_42`,
    // which `FUN_802e032c` may then bump on a carry).
    let mut conv = platform::read_u8(spec + 5);
    let mut exponent = i16::from_be_bytes([out[2], out[3]]) as i32 + out[4] as i32 - 1;
    let stored = (exponent as i16).to_be_bytes();
    out[2] = stored[0];
    out[3] = stored[1];
    let mut exponent_form = false;

    if conv == b'g' || conv == b'G' {
        if precision < out[4] as i32 {
            fn_802e032c(out.as_mut_ptr() as usize, precision);
        }
        exponent = i16::from_be_bytes([out[2], out[3]]) as i32;
        if exponent < -4 || precision <= exponent {
            if platform::read_u8(spec + 3) == 0 {
                platform::write_i32(spec + 0xC, out[4] as i32 - 1);
            } else {
                platform::write_i32(spec + 0xC, precision - 1);
            }
            conv = if conv == b'g' { b'e' } else { b'E' };
            platform::write_u8(spec + 5, conv);
            exponent_form = true;
        } else if platform::read_u8(spec + 3) == 0 {
            let mut n = out[4] as i32 - (exponent + 1);
            if n < 0 {
                n = 0;
            }
            platform::write_i32(spec + 0xC, n);
        } else {
            platform::write_i32(spec + 0xC, precision - (exponent + 1));
        }
    } else if conv == b'e' || conv == b'E' {
        exponent_form = true;
    } else if conv != b'f' && conv != b'F' {
        return buf_end.wrapping_sub(1);
    }

    let precision = platform::read_i32(spec + 0xC);
    let mut cursor = buf_end.wrapping_sub(1);
    platform::write_u8(cursor as usize, 0);

    if exponent_form {
        // `LAB_802e0848`: `d.dddde+XX`.
        if precision + 1 < out[4] as i32 {
            fn_802e032c(out.as_mut_ptr() as usize, precision + 1);
        }
        let mut exp = i16::from_be_bytes([out[2], out[3]]) as i32;
        let mut exp_sign = b'+';
        if exp < 0 {
            exp = -exp;
            exp_sign = b'-';
        }
        let mut count = 0;
        while exp != 0 || count < 2 {
            let digit = (exp % 10) as u8;
            exp /= 10;
            cursor -= 1;
            platform::write_u8(cursor as usize, digit + b'0');
            count += 1;
        }
        cursor -= 1;
        platform::write_u8(cursor as usize, exp_sign);
        cursor -= 1;
        platform::write_u8(cursor as usize, conv);

        if 0x1FD < precision + (buf_end.wrapping_sub(cursor) as i32) {
            return 0;
        }
        if (out[4] as i32) < precision + 1 {
            let mut pad = (precision + 2) - out[4] as i32;
            while {
                pad -= 1;
                pad != 0
            } {
                cursor -= 1;
                platform::write_u8(cursor as usize, b'0');
            }
        }
        let digits = out[4] as usize;
        let mut src = 5 + digits;
        for _ in 0..digits {
            src -= 1;
            cursor -= 1;
            platform::write_u8(cursor as usize, out[src]);
        }
        if precision != 0 || platform::read_u8(spec + 3) != 0 {
            cursor -= 1;
            platform::write_u8(cursor as usize, decimal_point);
        }
        let lead = cursor - 1;
        platform::write_u8(lead as usize, out[5]);
        if out[0] != 0 {
            platform::write_u8((cursor - 2) as usize, b'-');
            return cursor.wrapping_sub(2) as u32;
        }
        match platform::read_u8(spec + 1) {
            1 => {
                platform::write_u8((cursor - 2) as usize, b'+');
                cursor.wrapping_sub(2) as u32
            }
            2 => {
                platform::write_u8((cursor - 2) as usize, b' ');
                cursor.wrapping_sub(2) as u32
            }
            _ => cursor.wrapping_sub(1) as u32,
        }
    } else {
        // `dddd.dddd` / `%f`.
        let mut fraction =
            out[4] as i32 - i16::from_be_bytes([out[2], out[3]]) as i32 - 1;
        if fraction < 0 {
            fraction = 0;
        }
        if precision < fraction {
            fn_802e032c(
                out.as_mut_ptr() as usize,
                out[4] as i32 - (fraction - precision),
            );
            fraction = out[4] as i32 - i16::from_be_bytes([out[2], out[3]]) as i32 - 1;
            if fraction < 0 {
                fraction = 0;
            }
        }
        let mut whole = i16::from_be_bytes([out[2], out[3]]) as i32 + 1;
        if whole < 0 {
            whole = 0;
        }
        if 0x1FD < whole + fraction {
            return 0;
        }

        // Fraction: pad, copy the available digits, then zero-fill.
        let available = out[4] as usize;
        let mut src = 5 + available;
        let mut copied = 0_i32;
        for _ in 0..(precision - fraction) {
            cursor -= 1;
            platform::write_u8(cursor as usize, b'0');
        }
        while copied < fraction && (copied as usize) < available {
            src -= 1;
            cursor -= 1;
            platform::write_u8(cursor as usize, out[src]);
            copied += 1;
        }
        for _ in copied..fraction {
            cursor -= 1;
            platform::write_u8(cursor as usize, b'0');
        }

        if precision != 0 || platform::read_u8(spec + 3) != 0 {
            cursor -= 1;
            platform::write_u8(cursor as usize, decimal_point);
        }
        if whole == 0 {
            cursor -= 1;
            platform::write_u8(cursor as usize, b'0');
        } else {
            for _ in 0..(whole - available as i32) {
                cursor -= 1;
                platform::write_u8(cursor as usize, b'0');
            }
            let take = whole - (whole - available as i32).max(0);
            for _ in 0..take {
                src -= 1;
                cursor -= 1;
                platform::write_u8(cursor as usize, out[src]);
            }
        }

        if out[0] == 0 {
            match platform::read_u8(spec + 1) {
                1 => {
                    cursor -= 1;
                    platform::write_u8(cursor as usize, b'+');
                }
                2 => {
                    cursor -= 1;
                    platform::write_u8(cursor as usize, b' ');
                }
                _ => {}
            }
        } else {
            cursor -= 1;
            platform::write_u8(cursor as usize, b'-');
        }
        cursor as u32
    }
}

/// Wide-character ctype lookup (`*(u16 *)(*(u32 *)(0x80350E68 + 0x14) + 2*c)`).
///
/// # Original
/// Host-side helper; no binary address. Models the wide ctype table pointer the
/// `printf` family reaches through `PTR_DAT_80350e68 + 0x14`.
///
/// # Confidence
/// high - the pointer chain and the `& 0x200` uppercase bit are literal in
/// every wide conversion (`0x80350DF0` -> `0x802FD9E8` in the shipped image).
unsafe fn wide_ctype_flags(c: u32) -> u16 {
    if c >= 0x100 {
        return 0;
    }
    let table = platform::read_u32(PTR_CTYPE_80350E68 + 0x14) as usize;
    platform::read_u16(table + (c as usize) * 2)
}

/// Formats one `double` for the wide-character `printf` driver (MSL `__cvt`).
///
/// In plain English: the same decimal layout as [`fn_802e0454`], but the text
/// is first built backwards in a narrow scratch buffer and then widened into
/// the caller's `u16` buffer with the MSL `mbstowcs` helper `0x802DEDFC`.
///
/// # Original
/// - Ghidra: `FUN_802e3208` at `0x802e3208` (1936 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e3208_FUN_802e3208.c`
///
/// # Arguments
/// * `value` - `param_1`; the number, in `f1`.
/// * `buf_end` - `param_2`; one past the caller's wide buffer. The widened
///   string lands at `buf_end - 2 * len - 2`.
/// * `spec` - `param_3`; the spec block. The wide parser stores the
///   conversion character at `+6` (16 bits), the sign mode at `+1`, the
///   alternate-form flag at `+3` and the precision at `+0xC`.
///
/// # Returns
/// The first wide character of the finished string, or `0` on the same
/// conditions as [`fn_802e0454`].
///
/// # Side effects
/// Rewrites the digit block through [`fn_802e30e0`], writes the narrow
/// scratch buffer and the widened result, and may rewrite `spec+6` / `spec+0xC`.
///
/// # Safety
/// `buf_end` must have room for the field; `spec` must be the wide spec block.
///
/// # Confidence
/// medium - the body is the wide twin of [`fn_802e0454`] and every branch,
/// copy and rounding position matches it; `// inferred:` the `0x1FD` length
/// guards are written as `precision + bytes_written`, the form the narrow
/// twin uses and the one the `subf` / `add` pair at `0x802E3658` implements.
/// The eight-byte unrolled fills are byte loops here.
pub unsafe fn fn_802e3208(value: f64, buf_end: u32, spec: usize) -> u32 {
    let decimal_point = platform::read_u8(platform::read_u32(0x8035_0C80) as usize);
    let precision = platform::read_i32(spec + 0xC);
    if precision > 0x1FD {
        return 0;
    }

    let mut dtoa_in = [0_u8; 4];
    dtoa_in[3] = 0x20;
    let mut out = [0_u8; 48];
    platform::call2(
        0x802D_E370,
        dtoa_in.as_mut_ptr() as u32,
        out.as_mut_ptr() as u32,
    );

    let mut scan = 5 + out[4] as usize;
    while out[4] > 1 {
        scan -= 1;
        if out[scan] != b'0' {
            break;
        }
        out[4] -= 1;
        let exponent = i16::from_be_bytes([out[2], out[3]]) + 1;
        let bytes = exponent.to_be_bytes();
        out[2] = bytes[0];
        out[3] = bytes[1];
    }

    let conv_wide = platform::read_u16(spec + 6);
    if out[5] == b'0' {
        out[2] = 0;
        out[3] = 0;
    } else if out[5] == b'I' {
        let upper = (wide_ctype_flags(conv_wide as u32) & 0x200) != 0;
        if value < read_f64(sda2(-0x39D8)) {
            let dst = buf_end.wrapping_sub(10);
            fn_802e4438(
                dst as usize,
                if upper { 0x8035_12FC } else { 0x8035_1306 },
            );
            return dst;
        }
        let dst = buf_end.wrapping_sub(8);
        fn_802e4438(
            dst as usize,
            if upper { 0x8035_1310 } else { 0x8035_1318 },
        );
        return dst;
    } else if out[5] == b'N' {
        let upper = (wide_ctype_flags(conv_wide as u32) & 0x200) != 0;
        if out[0] == 0 {
            let dst = buf_end.wrapping_sub(8);
            fn_802e4438(
                dst as usize,
                if upper { 0x8035_1334 } else { 0x8035_133C },
            );
            return dst;
        }
        let dst = buf_end.wrapping_sub(10);
        fn_802e4438(
            dst as usize,
            if upper { 0x8035_1320 } else { 0x8035_132A },
        );
        return dst;
    }

    // The narrow scratch buffer the finished text is built in; `local_23f` is
    // its digit area and `auStack_218` its body.
    let mut tmp = [0_u8; 512];
    let mut cursor = tmp.len() - 1;
    tmp[cursor] = 0;

    let mut conv = platform::read_u16(spec + 6) as u8;
    let mut exponent =
        i16::from_be_bytes([out[2], out[3]]) as i32 + out[4] as i32 - 1;
    let stored = (exponent as i16).to_be_bytes();
    out[2] = stored[0];
    out[3] = stored[1];
    let mut exponent_form = false;

    if conv == b'g' || conv == b'G' {
        if precision < out[4] as i32 {
            fn_802e30e0(out.as_mut_ptr() as usize, precision);
        }
        exponent = i16::from_be_bytes([out[2], out[3]]) as i32;
        if exponent < -4 || precision <= exponent {
            if platform::read_u8(spec + 3) == 0 {
                platform::write_i32(spec + 0xC, out[4] as i32 - 1);
            } else {
                platform::write_i32(spec + 0xC, precision - 1);
            }
            conv = if conv == b'g' { b'e' } else { b'E' };
            let wide_conv = conv as u16;
            platform::write_u16(spec + 6, wide_conv);
            exponent_form = true;
        } else if platform::read_u8(spec + 3) == 0 {
            let mut n = out[4] as i32 - (exponent + 1);
            if n < 0 {
                n = 0;
            }
            platform::write_i32(spec + 0xC, n);
        } else {
            platform::write_i32(spec + 0xC, precision - (exponent + 1));
        }
    } else if conv == b'e' || conv == b'E' {
        exponent_form = true;
    } else if conv != b'f' && conv != b'F' {
        return widen(tmp.as_ptr().add(cursor) as u32, buf_end);
    }

    let precision = platform::read_i32(spec + 0xC);

    if exponent_form {
        if precision + 1 < out[4] as i32 {
            fn_802e30e0(out.as_mut_ptr() as usize, precision + 1);
        }
        let mut exp = i16::from_be_bytes([out[2], out[3]]) as i32;
        let mut exp_sign = b'+';
        if exp < 0 {
            exp = -exp;
            exp_sign = b'-';
        }
        let mut count = 0;
        while exp != 0 || count < 2 {
            let digit = (exp % 10) as u8;
            exp /= 10;
            cursor -= 1;
            tmp[cursor] = digit + b'0';
            count += 1;
        }
        cursor -= 1;
        tmp[cursor] = exp_sign;
        cursor -= 1;
        tmp[cursor] = conv;

        let written = (tmp.len() - 1 - cursor) as i32;
        if 0x1FD < precision + written {
            return 0;
        }
        if (out[4] as i32) < precision + 1 {
            let mut pad = (precision + 2) - out[4] as i32;
            while {
                pad -= 1;
                pad != 0
            } {
                cursor -= 1;
                tmp[cursor] = b'0';
            }
        }
        let digits = out[4] as usize;
        let mut src = 5 + digits;
        for _ in 0..digits {
            src -= 1;
            cursor -= 1;
            tmp[cursor] = out[src];
        }
        if precision != 0 || platform::read_u8(spec + 3) != 0 {
            cursor -= 1;
            tmp[cursor] = decimal_point;
        }
        cursor -= 1;
        tmp[cursor] = out[5];
        if out[0] != 0 {
            cursor -= 1;
            tmp[cursor] = b'-';
        } else {
            match platform::read_u8(spec + 1) {
                1 => {
                    cursor -= 1;
                    tmp[cursor] = b'+';
                }
                2 => {
                    cursor -= 1;
                    tmp[cursor] = b' ';
                }
                _ => {}
            }
        }
        widen(tmp.as_ptr().add(cursor) as u32, buf_end)
    } else {
        let mut fraction =
            out[4] as i32 - i16::from_be_bytes([out[2], out[3]]) as i32 - 1;
        if fraction < 0 {
            fraction = 0;
        }
        if precision < fraction {
            fn_802e30e0(
                out.as_mut_ptr() as usize,
                out[4] as i32 - (fraction - precision),
            );
            fraction = out[4] as i32 - i16::from_be_bytes([out[2], out[3]]) as i32 - 1;
            if fraction < 0 {
                fraction = 0;
            }
        }
        let mut whole = i16::from_be_bytes([out[2], out[3]]) as i32 + 1;
        if whole < 0 {
            whole = 0;
        }
        if 0x1FD < whole + fraction {
            return 0;
        }

        let available = out[4] as usize;
        let mut src = 5 + available;
        let mut copied = 0_i32;
        for _ in 0..(precision - fraction) {
            cursor -= 1;
            tmp[cursor] = b'0';
        }
        while copied < fraction && (copied as usize) < available {
            src -= 1;
            cursor -= 1;
            tmp[cursor] = out[src];
            copied += 1;
        }
        for _ in copied..fraction {
            cursor -= 1;
            tmp[cursor] = b'0';
        }

        if precision != 0 || platform::read_u8(spec + 3) != 0 {
            cursor -= 1;
            tmp[cursor] = decimal_point;
        }
        if whole == 0 {
            cursor -= 1;
            tmp[cursor] = b'0';
        } else {
            for _ in 0..(whole - available as i32) {
                cursor -= 1;
                tmp[cursor] = b'0';
            }
            let take = whole - (whole - available as i32).max(0);
            for _ in 0..take {
                src -= 1;
                cursor -= 1;
                tmp[cursor] = out[src];
            }
        }

        if out[0] == 0 {
            match platform::read_u8(spec + 1) {
                1 => {
                    cursor -= 1;
                    tmp[cursor] = b'+';
                }
                2 => {
                    cursor -= 1;
                    tmp[cursor] = b' ';
                }
                _ => {}
            }
        } else {
            cursor -= 1;
            tmp[cursor] = b'-';
        }
        widen(tmp.as_ptr().add(cursor) as u32, buf_end)
    }
}

/// Widen a finished narrow field into the caller's `u16` buffer.
///
/// This is the export's tail
/// `dst = buf_end - 2 * strlen(p) - 2; mbstowcs(dst, p, strlen(p))`.
///
/// # Original
/// Host-side glue for [`fn_802e3208`]; `0x802DEDFC` is the MSL `mbstowcs`.
///
/// # Returns
/// `dst`, the first wide character of the converted field.
///
/// # Safety
/// `dst` must have `len` writable `u16` slots.
unsafe fn widen(narrow: u32, buf_end: u32) -> u32 {
    let mut len = 0_u32;
    while platform::read_u8(narrow.wrapping_add(len) as usize) != 0 {
        len += 1;
    }
    let dst = buf_end.wrapping_sub(len.wrapping_mul(2)).wrapping_sub(2);
    platform::call3(0x802D_EDFC, dst, narrow, len);
    dst
}

// Addresses of the MSL helpers the two `printf` drivers call by absolute
// address. None of them is in the `0x802E0000` bucket, so they stay stubs.
const FN_802DC6B8: usize = 0x802D_C6B8; // va_arg(ap, size) -> pointer
const FN_802DC69C: usize = 0x802D_C69C; // strlen
const FN_802DF4B8: usize = 0x802D_F4B8; // narrow format-spec parser
const FN_802DF040: usize = 0x802D_F040; // bounded string scan
const FN_802DFA74: usize = 0x802D_FA74; // u32 -> narrow digits
const FN_802DFCB4: usize = 0x802D_FCB4; // u64 -> narrow digits
const FN_802DFF64: usize = 0x802D_FF64; // double -> narrow digits
const FN_802DEEBC: usize = 0x802D_EEBC; // wcstombs-like wide -> narrow
const FN_802E4438_WIDE_STRCPY: usize = 0x802E_4438;
const FN_802E21FC_WIDE_PARSER: usize = 0x802E_21FC;
const FN_802E2C8C_WIDE_HEX: usize = 0x802E_2C8C;
const FN_802E278C_WIDE_U32: usize = 0x802E_278C;
const FN_802E29D0_WIDE_U64: usize = 0x802E_29D0;
const FN_802E441C_WIDE_STRLEN: usize = 0x802E_441C;
const FN_802E44CC_WIDE_STRCHR: usize = 0x802E_44CC;
const FN_802E30E0_WIDE_ROUND: usize = 0x802E_30E0;

/// `0x80429BB0` - `r13 - 0x5AD0`, the empty wide string in BSS.
///
/// # Original
/// - Observed: `iVar3 = unaff_r13 + -0x5ad0` in `802e0bf0_FUN_802e0bf0.c`
const EMPTY_WIDE_8042_9BB0: usize = 0x8042_9BB0;

/// Emits the rest of a malformed format string (the export's `caseD_26`).
///
/// # Original
/// Host-side helper for [`fn_802e0bf0`] / [`fn_802e3998`]; models the
/// `strlen` + one callback call the two drivers jump to when a conversion
/// fails. The address `0x802DC69C` is the MSL `strlen`.
///
/// # Returns
/// The new character count, or `-1` when the callback reports an error.
///
/// # Safety
/// `from` must point at a NUL-terminated string in the format buffer.
unsafe fn emit_rest_of_format(
    callback: u32,
    stream: u32,
    from: u32,
    count: i32,
) -> i32 {
    let len = platform::call1(FN_802DC69C, from) as i32;
    if len == 0 {
        return count + len;
    }
    if platform::call3(callback as usize, stream, from, len as u32) != 0 {
        return count + len;
    }
    -1
}

/// The narrow-character `printf` engine (MSL `__printf` core).
///
/// In plain English: "walk the format string, copy the literal runs to the
/// output callback, parse each `%` spec, fetch the argument out of the
/// `va_list`, convert it with the matching MSL routine, then emit it with the
/// width / padding rules."
///
/// # Original
/// - Ghidra: `FUN_802e0bf0` at `0x802e0bf0` (2220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e0bf0_FUN_802e0bf0.c`
///
/// # Arguments
/// * `callback` - `param_1`; the output function, called as
///   `callback(stream, bytes, len)` and returning non-zero on success.
/// * `stream` - `param_2`; opaque first argument for the callback.
/// * `fmt` - `param_3`; the NUL-terminated narrow format string.
/// * `ap` - `param_4`; the `va_list` cursor.
/// * `bounded` - `param_5`; non-zero selects the `snprintf` behaviour, where a
///   null `%s` / `%ls` argument and `%n` are errors.
///
/// # Returns
/// The number of characters the format produces, or `-1` when the callback
/// fails or a bounded conversion overflows.
///
/// # Side effects
/// Calls `callback` once per literal run / conversion / padding byte, writes
/// count through `%n`, and fills the `spec` block.
///
/// # Safety
/// `callback` must be a valid `extern "C"` function with the three-argument
/// shape, `fmt` readable, `ap` a valid argument cursor.
///
/// # Confidence
/// medium - every case, the width / padding loop and the `%n` store are
/// literal. `// inferred:` the spec block is the 16-byte record the parser
/// writes: byte 0 pad mode, +1 sign mode, +2 precision given, +3 alternate
/// form, +4 type code, +5 conversion character, +8 width, +12 precision. The
/// conversions are called by absolute address because they are outside this
/// bucket.
pub unsafe fn fn_802e0bf0(
    callback: u32,
    stream: u32,
    fmt: usize,
    ap: u32,
    bounded: i32,
) -> i32 {
    let mut count = 0_i32;
    let mut cursor = fmt;
    let mut spec = [0_u8; 16];
    let mut scratch = [0_u8; 524];
    let base = scratch.as_mut_ptr() as usize;
    let scratch_end = base + 512; // the export's `auStack_51 + 1`

    loop {
        if platform::read_u8(cursor) == 0 {
            return count;
        }
        let percent = fn_802e1c10(cursor, 0x25);
        if percent == 0 {
            let len = platform::call1(FN_802DC69C, cursor as u32) as i32;
            if len == 0 {
                return count + len;
            }
            if platform::call3(callback as usize, stream, cursor as u32, len as u32) != 0 {
                return count + len;
            }
            return -1;
        }

        count += (percent - cursor) as i32;
        if percent != cursor
            && platform::call3(callback as usize, stream, cursor as u32, (percent - cursor) as u32) == 0
        {
            return -1;
        }

        cursor = platform::call3(
            FN_802DF4B8,
            percent as u32,
            ap,
            spec.as_mut_ptr() as u32,
        ) as usize;

        let flags = spec[0];
        let alternative = spec[3];
        let conv = spec[5];
        let type_code = spec[4];
        let precision = i32::from_be_bytes([spec[12], spec[13], spec[14], spec[15]]);
        let width = i32::from_be_bytes([spec[8], spec[9], spec[10], spec[11]]);

        let mut text;
        let mut len;

        match conv {
            b'%' => {
                scratch[0] = b'%';
                text = base;
                len = 1;
            }
            b'A' | b'a' => {
                let slot = platform::call2(FN_802DC6B8, ap, 3) as usize;
                let value = read_f64(slot);
                let mut copy = [0_u8; 16];
                core::ptr::copy_nonoverlapping(spec.as_ptr(), copy.as_mut_ptr(), 16);
                let produced = call_f64_2(
                    FN_802DFF64,
                    value,
                    (scratch_end + 1) as u32,
                    copy.as_mut_ptr() as u32,
                ) as usize;
                if produced == 0 {
                    return emit_rest_of_format(
                        callback,
                        stream,
                        percent as u32,
                        count,
                    );
                }
                text = produced;
                len = (scratch_end - 1 - produced) as i32;
            }
            b'E' | b'F' | b'G' | b'e' | b'f' | b'g' => {
                let slot = platform::call2(FN_802DC6B8, ap, 3) as usize;
                let value = read_f64(slot);
                let mut copy = [0_u8; 16];
                core::ptr::copy_nonoverlapping(spec.as_ptr(), copy.as_mut_ptr(), 16);
                let produced = call_f64_2(
                    0x802E_0454,
                    value,
                    (scratch_end + 1) as u32,
                    copy.as_mut_ptr() as u32,
                ) as usize;
                if produced == 0 {
                    return emit_rest_of_format(
                        callback,
                        stream,
                        percent as u32,
                        count,
                    );
                }
                text = produced;
                len = (scratch_end - 1 - produced) as i32;
            }
            b'X' | b'o' | b'u' | b'x' => {
                let value = fetch_integer(ap, type_code);
                let mut copy = [0_u8; 16];
                core::ptr::copy_nonoverlapping(spec.as_ptr(), copy.as_mut_ptr(), 16);
                let (produced, _wide) = convert_integer(
                    value,
                    type_code,
                    (scratch_end + 1) as u32,
                    copy.as_mut_ptr() as u32,
                );
                if produced == 0 {
                    return emit_rest_of_format(
                        callback,
                        stream,
                        percent as u32,
                        count,
                    );
                }
                text = produced;
                len = (scratch_end - 1 - produced) as i32;
            }
            b'd' | b'i' => {
                let value = fetch_integer(ap, type_code);
                let mut copy = [0_u8; 16];
                core::ptr::copy_nonoverlapping(spec.as_ptr(), copy.as_mut_ptr(), 16);
                let (produced, _wide) = convert_integer(
                    value,
                    type_code,
                    (scratch_end + 1) as u32,
                    copy.as_mut_ptr() as u32,
                );
                if produced == 0 {
                    return emit_rest_of_format(
                        callback,
                        stream,
                        percent as u32,
                        count,
                    );
                }
                text = produced;
                len = (scratch_end - 1 - produced) as i32;
            }
            b'c' => {
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                scratch[0] = platform::read_u8(slot);
                text = base;
                len = 1;
            }
            b'n' => {
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                let out = platform::read_u32(slot) as usize;
                if bounded != 0 {
                    // observed: the export calls `FUN_802e4648(0, 0, -1)`.
                    fn_802e4648();
                    return -1;
                }
                match type_code {
                    0 | 3 | 7 | 8 => platform::write_i32(out, count),
                    2 => platform::write_u16(out, count as u16),
                    4 | 6 => {
                        platform::write_i32(out + 4, count);
                        platform::write_i32(out, count >> 31);
                    }
                    // observed: types 1 (hh) and 5 (w) store nothing.
                    _ => {}
                }
                continue;
            }
            b's' => {
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                let argument = platform::read_u32(slot) as usize;
                if type_code == 5 {
                    let mut wide = argument;
                    if bounded != 0 && wide == 0 {
                        fn_802e4648();
                        return -1;
                    }
                    if wide == 0 {
                        wide = EMPTY_WIDE_8042_9BB0;
                    }
                    let produced =
                        platform::call3(FN_802DEEBC, base as u32, wide as u32, 0x200) as i32;
                    if produced < 0 {
                        return emit_rest_of_format(
                            callback,
                            stream,
                            percent as u32,
                            count,
                        );
                    }
                    text = base;
                } else {
                    if bounded != 0 && argument == 0 {
                        fn_802e4648();
                        return -1;
                    }
                    text = if argument == 0 {
                        0x802F_D9E4 // the empty string ""
                    } else {
                        argument
                    };
                }
                if alternative == 0 {
                    if spec[2] == 0 {
                        len = platform::call1(FN_802DC69C, text as u32) as i32;
                    } else {
                        let end = platform::call3(
                            FN_802DF040,
                            text as u32,
                            0,
                            precision as u32,
                        ) as usize;
                        len = if end != 0 {
                            (end - text) as i32
                        } else {
                            precision
                        };
                    }
                } else {
                    // `// inferred:` a counted string: first byte is the length.
                    len = platform::read_u8(text) as i32;
                    text += 1;
                    if spec[2] != 0 && precision < len {
                        len = precision;
                    }
                }
            }
            _ => {
                return emit_rest_of_format(callback, stream, percent as u32, count);
            }
        }

        // Width and padding.
        let mut field_len = len;
        let mut remaining = len;
        if flags != 0 {
            let pad = if flags == 2 { b'0' } else { b' ' };
            let first = platform::read_u8(text);
            if (first == b'+' || first == b'-' || first == b' ')
                && pad == b'0'
            {
                if platform::call3(callback as usize, stream, text as u32, 1) == 0 {
                    return -1;
                }
                text += 1;
                remaining = field_len - 1;
            }
            if flags == 2 && (conv == b'a' || conv == b'A') {
                if remaining < 2 {
                    return -1;
                }
                if platform::call3(callback as usize, stream, text as u32, 2) == 0 {
                    return -1;
                }
                remaining -= 2;
                text += 2;
            }
            while field_len < width {
                let byte = pad;
                if platform::call3(
                    callback as usize,
                    stream,
                    &byte as *const u8 as u32,
                    1,
                ) == 0
                {
                    return -1;
                }
                field_len += 1;
            }
        }
        if remaining != 0
            && platform::call3(callback as usize, stream, text as u32, remaining as u32) == 0
        {
            return -1;
        }
        if flags == 0 {
            while field_len < width {
                let byte = b' ';
                if platform::call3(
                    callback as usize,
                    stream,
                    &byte as *const u8 as u32,
                    1,
                ) == 0
                {
                    return -1;
                }
                field_len += 1;
            }
        }
        count += field_len;
    }
}

/// One integer argument fetched through the `va_list` cursor.
///
/// Returns the two words the radix converters take: for the `ll` / `j` type
/// codes both words are read from the argument slot, for everything else the
/// high word is zero and the low word carries the type-code truncation the
/// export applies (`(short)` for `h`, `(char)` for `hh`, sign-extended).
///
/// # Original
/// Host-side helper for [`fn_802e0bf0`] / [`fn_802e3998`]; models the
/// `FUN_802dc6b8` calls and the type-code masks in both drivers.
///
/// # Safety
/// `ap` must be a valid `va_list` cursor.
unsafe fn fetch_integer(ap: u32, type_code: u8) -> (u32, u32) {
    match type_code {
        4 | 6 => {
            let slot = platform::call2(FN_802DC6B8, ap, 2) as usize;
            (platform::read_u32(slot), platform::read_u32(slot + 4))
        }
        _ => {
            let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
            let raw = platform::read_u32(slot);
            let value = match type_code {
                2 => (raw as u16 as i16 as i32) as u32,
                1 => (raw as u8 as i8 as i32) as u32,
                _ => raw,
            };
            (value, 0)
        }
    }
}

/// Converts one integer argument into the narrow digit buffer.
///
/// # Original
/// Host-side helper for [`fn_802e0bf0`]; models the two call shapes
/// `FUN_802dfcb4(hi, lo, end, spec)` (type codes 4 / 6) and
/// `FUN_802dfa74(value, end, spec)` (everything else). The signedness is the
/// conversion character, which the spec block already carries, so both
/// `%d` and `%x` take the same call.
///
/// # Returns
/// `(start, true)` for the 64-bit call and `(start, false)` otherwise; `start`
/// is `0` when the conversion failed.
///
/// # Safety
/// `end` must address a writable scratch tail and `spec` a valid spec block.
unsafe fn convert_integer(
    value: (u32, u32),
    type_code: u8,
    end: u32,
    spec: u32,
) -> (usize, bool) {
    if type_code == 4 || type_code == 6 {
        (
            platform::call4(FN_802DFCB4, value.0, value.1, end, spec) as usize,
            true,
        )
    } else {
        (platform::call3(FN_802DFA74, value.0, end, spec) as usize, false)
    }
}

/// Calls a PowerPC function taking a `double` in `f1` and two integers.
///
/// The MSL conversion routines (`FUN_802dff64`, [`fn_802e0454`]) take the
/// value in `f1` and the buffer / spec pointers in `r3` / `r4`; this crate
/// has no such helper in [`crate::platform`], so the narrow wrapper lives here.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f64_2(addr: usize, x: f64, a: u32, b: u32) -> u32 {
    let f: extern "C" fn(f64, u32, u32) -> u32 = core::mem::transmute(addr);
    f(x, a, b)
}

// ---------------------------------------------------------------------------
// Wave 105 - the wide printf parser, driver and %a converter, and the reply flag.
// ---------------------------------------------------------------------------
//
// The wide `printf` family is the twin of the narrow one that Wave 104 landed
// just above: `fn_802e3998` is the driver (the twin of [`fn_802e0bf0`]),
// `fn_802e21fc` is its format-spec parser (the twin of `0x802DF4B8`) and
// `fn_802e2c8c` is the `%a` / `%A` converter (the twin of the `%e` / `%f` /
// `%g` converter [`fn_802e0454`]). The three now form a closed set inside this
// file: the driver calls all three directly instead of by absolute address.

/// Formats one `double` as a wide `%a` / `%A` hexadecimal float (MSL `__cvt_hex`).
///
/// In plain English: "run the decimal conversion only to learn the sign and
/// whether the value is zero / infinity / NaN; for a finite non-zero value,
/// rebuild the mantissa as hexadecimal nibbles from the raw IEEE-754 bits and
/// hang a decimal `p`-exponent on the end."
///
/// # Original
/// - Ghidra: `FUN_802e2c8c` at `0x802e2c8c` (1108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e2c8c_FUN_802e2c8c.c`
///
/// # Arguments
/// * `value` - `param_1`; the number, in `f1`.
/// * `buf_end` - `param_2`; one past the caller's wide buffer. The field is
///   built backwards from `buf_end - 2` and the first unit is returned.
/// * `spec` - `param_3`; the wide spec block from [`fn_802e21fc`]:
///   `+0` pad mode, `+1` sign mode (0 none, 1 `+`, 2 space), `+2` precision
///   given, `+3` alternate form, `+6` conversion character (16 bits),
///   `+0xC` precision (word).
///
/// # Returns
/// The first wide unit of the finished field, or `0` when the precision is at
/// least `0x1FE` (the caller's buffer cannot hold the field).
///
/// # Side effects
/// Writes one eight-byte `double` request and a 48-byte digit record through
/// `FUN_802DE370`, writes the field backwards into `buf_end`, and calls
/// [`fn_802e278c`] to render the binary exponent in decimal.
///
/// # Safety
/// `buf_end` must have room for `precision + 8` wide units and `spec` must be
/// a valid 16-byte record.
///
/// # Confidence
/// medium - every branch, the eight special-case string addresses and the
/// nibble extraction are literal. `// inferred:` `spec+6 == 'A'` selects the
/// uppercase spelling (the export tests `0x41`), and the constant compared
/// against `ABS(value)` is the shipped `.sdata2` word at `r2-0x39D8`, read
/// from the image rather than guessed. The decompiler's `GQR0` probe and the
/// byte-identical second spec copy at `local_68` (nothing reads it) are
/// documented as omitted paired-single / dead-store artifacts.
pub unsafe fn fn_802e2c8c(value: f64, buf_end: u32, spec: u32) -> u32 {
    let sp = spec as usize;
    // `PTR_DAT_80350c80` -> the locale decimal-point character.
    let decimal_point =
        platform::read_u8(platform::read_u32(0x8035_0C80) as usize) as u16;
    let precision = platform::read_i32(sp + 0xC);
    if precision >= 0x1FE {
        return 0;
    }
    let conv = platform::read_u16(sp + 6);

    // `local_80`: dtoa request; bytes `+2..+4` = 32 significant digits.
    let mut dtoa_in = [0_u8; 4];
    dtoa_in[3] = 0x20;
    // `local_58`: sign at `+0`, decimal exponent at `+2`, digit count at `+4`,
    // first digit at `+5`.
    let mut out = [0_u8; 48];
    call_f64_2(
        0x802D_E370,
        value,
        dtoa_in.as_mut_ptr() as u32,
        out.as_mut_ptr() as u32,
    );

    let negative = out[0] != 0;
    let first_digit = out[5];

    if first_digit == b'0' || first_digit == b'I' || first_digit == b'N' {
        if first_digit == b'0' {
            // observed: the zero case first clears the decimal exponent. The
            // value is never read again, so the store goes through the
            // volatile helper to keep the write (and silence the dead-store
            // lint).
            platform::write_u8(out.as_mut_ptr().add(2) as usize, 0);
            platform::write_u8(out.as_mut_ptr().add(3) as usize, 0);
        }
        let (unsigned_src, signed_src) = if first_digit == b'0' {
            if conv == 0x41 {
                (0x8035_12EC, 0x8035_12D8) // "0X0" / "-0X0"
            } else {
                (0x8035_12F4, 0x8035_12E2) // "0x0" / "-0x0"
            }
        } else if first_digit == b'I' {
            if conv == 0x41 {
                (0x8035_1310, 0x8035_12FC) // "INF" / "-INF"
            } else {
                (0x8035_1318, 0x8035_1306) // "inf" / "-inf"
            }
        } else if conv == 0x41 {
            (0x8035_1334, 0x8035_1320) // "NAN" / "-NAN"
        } else {
            (0x8035_133C, 0x8035_132A) // "nan" / "-nan"
        };

        // Four units without a sign, five with one.
        let (dst, src) = if negative {
            (buf_end.wrapping_sub(10), signed_src)
        } else {
            (buf_end.wrapping_sub(8), unsigned_src)
        };
        fn_802e4438(dst as usize, src);
        return dst;
    }

    // The decimal `p` exponent goes through the wide `%d` converter.
    let mut exponent_spec = [0_u8; 16];
    exponent_spec[0] = 1; // `0x01010000`: pad mode 1
    exponent_spec[1] = 1; // always show the exponent sign
    exponent_spec[6] = b'd'; // decimal digits
    exponent_spec[15] = 1; // precision 1
    let exponent_field = ((value.to_bits() >> 52) & 0x7FF) as u32;
    // observed: `(exp - 0x3FF) & ((int)(-exp | exp) >> 31)` is the same as
    // "subtract the bias, but keep 0 for a zero exponent field".
    let unbiased = if exponent_field != 0 {
        exponent_field.wrapping_sub(0x3FF)
    } else {
        0
    };
    let mut cursor = fn_802e278c(unbiased, buf_end, exponent_spec.as_mut_ptr() as u32);

    // `p` / `P` goes just before the first exponent digit.
    cursor = cursor.wrapping_sub(2);
    platform::write_u16(cursor as usize, if conv == 0x61 { 0x70 } else { 0x50 });

    // One hexadecimal nibble per requested fraction digit. The export reads
    // the saved `double` byte-wise as `local_80[idx - 8]`; here the same byte
    // comes straight out of the big-endian bit pattern.
    let bits = value.to_bits();
    let mut remaining = precision;
    let mut bit = (precision as u32).wrapping_mul(4).wrapping_add(0xB);
    while remaining > 0 {
        let digit = if bit < 0x40 {
            let index = (bit >> 3) as usize;
            let shift = 7 - (bit & 7);
            let byte = ((bits >> (8 * (7 - index))) & 0xFF) as u32;
            let mut nibble = (byte >> shift) & 0xFF;
            if (bit & !7) != (bit.wrapping_sub(4) & !7) {
                // The export's `(&bStack_89)[index]` is the byte just above
                // the selected one; index 0 would fall outside the `double`.
                let previous = if index >= 1 {
                    ((bits >> (8 * (8 - index))) & 0xFF) as u32
                } else {
                    0
                };
                nibble |= ((previous << 8) >> shift) & 0xFF;
            }
            nibble & 0xF
        } else {
            0
        };
        let unit = if digit < 10 {
            digit as u16 + 0x30
        } else if conv == 0x61 {
            digit as u16 + 0x57 // 'a' - 10
        } else {
            digit as u16 + 0x37 // 'A' - 10
        };
        cursor = cursor.wrapping_sub(2);
        platform::write_u16(cursor as usize, unit);
        bit = bit.wrapping_sub(4);
        remaining -= 1;
    }

    if precision != 0 || platform::read_u8(sp + 3) != 0 {
        cursor = cursor.wrapping_sub(2);
        platform::write_u16(cursor as usize, decimal_point);
    }

    // The leading hex digit: `0` for a zero magnitude, `1` for everything
    // else. `r2 - 0x39D8` is the constant the export compares against.
    let magnitude = f64::from_bits(value.to_bits() & 0x7FFF_FFFF_FFFF_FFFF);
    let lead = if magnitude == read_f64(sda2(-0x39D8)) {
        0x30
    } else {
        0x31
    };
    platform::write_u16(cursor.wrapping_sub(2) as usize, lead);
    platform::write_u16(
        cursor.wrapping_sub(4) as usize,
        if conv == 0x61 { 0x78 } else { 0x58 },
    );
    let mut start = cursor.wrapping_sub(6);
    platform::write_u16(start as usize, 0x30);

    if negative {
        start = cursor.wrapping_sub(8);
        platform::write_u16(start as usize, 0x2D);
    } else {
        match platform::read_u8(sp + 1) {
            1 => {
                start = cursor.wrapping_sub(8);
                platform::write_u16(start as usize, 0x2B);
            }
            2 => {
                start = cursor.wrapping_sub(8);
                platform::write_u16(start as usize, 0x20);
            }
            _ => {}
        }
    }
    start
}

/// Parses one `%` field of a wide format string (MSL `__parse_format`).
///
/// In plain English: "skip the flag characters, read the width and precision
/// (both may be `*`), read the optional length modifier, then classify the
/// conversion character and reject the combinations that MSL does not allow
/// by writing `0xFFFF` into the conversion slot."
///
/// # Original
/// - Ghidra: `FUN_802e21fc` at `0x802e21fc` (1424 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e21fc_FUN_802e21fc.c`
///
/// # Arguments
/// * `fmt` - `param_1`; points at the `%` that starts the field.
/// * `ap` - `param_2`; the `va_list` cursor, used when a width or precision
///   is `*`.
/// * `spec` - `param_3`; receives the 16-byte spec block:
///   `+0` pad mode (0 left, 1 default, 2 zero), `+1` sign mode, `+2` precision
///   given, `+3` alternate form, `+4` length code, `+6` conversion character
///   (16 bits), `+8` width, `+0xC` precision.
///
/// # Returns
/// The wide character just past the conversion character.
///
/// # Side effects
/// Writes the spec block; consumes up to two `va_arg` slots for `*` widths.
///
/// # Safety
/// `fmt` must be a NUL-terminated wide string and `spec` 16 writable bytes.
///
/// # Confidence
/// medium - the flag, width, precision and length loops and every rejection
/// test are literal. `// inferred:` the length code starts at 0 (the export
/// reads an uninitialised stack word that every surrounding test treats as
/// "no modifier") and the `#` flag keeps its byte when a precision is given
/// (the export's `CONCAT11(1, (byte)local_28)` re-stores the same byte).
pub unsafe fn fn_802e21fc(fmt: u32, ap: u32, spec: u32) -> u32 {
    let sp = spec as usize;
    let mut out = [0_u8; 16];
    out[0] = 1; // `local_28 = 0x01000000`: default (non-zero) pad mode

    let mut cursor = fmt.wrapping_add(2); // skip a possible UTF-16 BOM
    let mut ch = platform::read_u16(cursor as usize) as u32;
    let mut width = 0_i32;
    let mut precision = 0_i32;

    if ch == 0x25 {
        // The `%` conversion carries no flags and no argument.
        out[6] = 0;
        out[7] = 0x25;
        platform::write_u32(sp + 8, 0);
        platform::write_u32(sp + 0xC, 0);
        copy_spec(&out, sp);
        return fmt.wrapping_add(4);
    }

    // Flag characters.
    loop {
        let mut recognised = true;
        match ch {
            0x2D => out[0] = 0,                        // '-'
            0x2B => out[1] = 1,                        // '+'
            0x20 => {
                // A space is weaker than an explicit '+'.
                if out[1] != 1 {
                    out[1] = 2;
                }
            }
            0x23 => out[3] = 1,                        // '#'
            0x30 => {
                // '0' only replaces the pad mode when no '-' was seen.
                if out[0] != 0 {
                    out[0] = 2;
                }
            }
            _ => recognised = false,
        }
        if !recognised {
            break;
        }
        cursor = cursor.wrapping_add(2);
        ch = platform::read_u16(cursor as usize) as u32;
    }

    // Field width.
    if ch == 0x2A {
        let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
        width = platform::read_u32(slot) as i32;
        if width < 0 {
            width = width.wrapping_neg();
            out[0] = 0;
        }
        cursor = cursor.wrapping_add(2);
        ch = platform::read_u16(cursor as usize) as u32;
    } else {
        while wide_ctype_flags(ch) & 8 != 0 {
            width = (ch as i32)
                .wrapping_add(width.wrapping_mul(10))
                .wrapping_sub(0x30);
            cursor = cursor.wrapping_add(2);
            ch = platform::read_u16(cursor as usize) as u32;
        }
    }

    if width < 0x1FE {
        let mut probe = cursor;
        if ch == 0x2E {
            // '.': the precision is given.
            probe = cursor.wrapping_add(2);
            ch = platform::read_u16(probe as usize) as u32;
            out[2] = 1;
            if ch == 0x2A {
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                precision = platform::read_u32(slot) as i32;
                if precision < 0 {
                    // A negative `*` precision is treated as "not given".
                    out[2] = 0;
                }
                probe = cursor.wrapping_add(4);
                ch = platform::read_u16(probe as usize) as u32;
            } else {
                while wide_ctype_flags(ch) & 8 != 0 {
                    precision = (ch as i32)
                        .wrapping_add(precision.wrapping_mul(10))
                        .wrapping_sub(0x30);
                    probe = probe.wrapping_add(2);
                    ch = platform::read_u16(probe as usize) as u32;
                }
            }
        }

        // Length modifier.
        let mut length: u8 = 0;
        let mut has_length = true;
        if ch == 0x68 {
            // 'h' -> short, 'hh' -> char
            length = 2;
            if platform::read_u16(probe.wrapping_add(2) as usize) == 0x68 {
                length = 1;
                probe = probe.wrapping_add(2);
                ch = 0x68;
            }
        } else if ch == 0x6C {
            // 'l' -> int, 'll' -> long long
            length = 3;
            if platform::read_u16(probe.wrapping_add(2) as usize) == 0x6C {
                length = 4;
                probe = probe.wrapping_add(2);
                ch = 0x6C;
            }
        } else if ch == 0x4C {
            length = 9; // 'L' -> long double
        } else if ch == 0x6A {
            length = 6; // 'j' -> intmax_t
        } else if ch == 0x74 {
            length = 8; // 't' -> ptrdiff_t
        } else if ch == 0x7A {
            length = 7; // 'z' -> size_t
        } else {
            has_length = false;
        }
        if has_length {
            probe = probe.wrapping_add(2);
            ch = platform::read_u16(probe as usize) as u32;
        }

        let mut conv = ch;
        // Rejections are `0xFFFF` in the conversion slot.
        match conv {
            0x41 | 0x61 => {
                // 'A' / 'a'
                if out[2] == 0 {
                    precision = 0xD;
                }
                if matches!(length, 6 | 7 | 8) || matches!(length, 1 | 2) || length == 4 {
                    conv = 0xFFFF;
                }
            }
            0x46 | 0x66 => {
                // 'F' / 'f'
                if matches!(length, 6 | 7 | 8) || length == 2 || length == 4 {
                    conv = 0xFFFF;
                } else if out[2] == 0 {
                    precision = 6;
                }
            }
            0x47 | 0x67 => {
                // 'G' / 'g' falls through into the 'e' rules.
                if precision == 0 {
                    precision = 1;
                }
                if matches!(length, 6 | 7 | 8) || matches!(length, 1 | 2) || length == 4 {
                    conv = 0xFFFF;
                } else if out[2] == 0 {
                    precision = 6;
                }
            }
            0x45 | 0x65 => {
                // 'E' / 'e'
                if matches!(length, 6 | 7 | 8) || matches!(length, 1 | 2) || length == 4 {
                    conv = 0xFFFF;
                } else if out[2] == 0 {
                    precision = 6;
                }
            }
            0x58 | 0x64 | 0x69 | 0x6F | 0x75 | 0x78 => {
                // X d i o u x
                if length == 9 {
                    length = 4;
                }
                if out[2] == 0 {
                    precision = 1;
                } else if out[0] == 2 {
                    out[0] = 1;
                }
            }
            0x63 => {
                // 'c': only the wide modifier is valid.
                if length == 3 {
                    length = 5;
                } else if out[2] != 0 || length != 0 {
                    conv = 0xFFFF;
                }
            }
            0x6E => {
                // 'n' never takes a long double.
                if length == 9 {
                    length = 4;
                }
            }
            0x70 => {
                // 'p' is `%#x` with a precision of 8.
                length = 3;
                out[3] = 1;
                conv = 0x78;
                precision = 8;
            }
            0x73 => {
                // 's': only the wide modifier is valid.
                if length == 3 {
                    length = 5;
                } else if length != 0 {
                    conv = 0xFFFF;
                }
            }
            _ => conv = 0xFFFF,
        }

        out[4] = length;
        out[5] = 0;
        out[6] = (conv >> 8) as u8;
        out[7] = conv as u8;
        platform::write_u32(sp + 8, width as u32);
        platform::write_u32(sp + 0xC, precision as u32);
        copy_spec(&out, sp);
        return probe.wrapping_add(2);
    }

    // Width alone is already too large for the caller's buffer.
    out[6] = 0xFF;
    out[7] = 0xFF;
    platform::write_u32(sp + 8, width as u32);
    platform::write_u32(sp + 0xC, 0);
    copy_spec(&out, sp);
    cursor.wrapping_add(2)
}

/// Copies the 16-byte spec record built on the stack to its destination.
///
/// # Original
/// Host-side helper for [`fn_802e21fc`]; models the four `*param_3 = ...`
/// stores that close every path of the export.
///
/// # Safety
/// `sp` must have 16 writable bytes.
unsafe fn copy_spec(out: &[u8; 16], sp: usize) {
    core::ptr::copy_nonoverlapping(out.as_ptr(), sp as *mut u8, 16);
}

/// Renders one wide `%s` field that could not be read back to the format text.
///
/// # Original
/// Host-side helper for [`fn_802e3998`]; models the export's
/// `caseD_26` handler, which measures the rest of the format with
/// `FUN_802e441c` (the wide `strlen`) and emits it.
///
/// # Returns
/// The new character count, or `-1` when the callback reports an error.
///
/// # Safety
/// `from` must point at a NUL-terminated wide string.
unsafe fn emit_rest_of_format_wide(callback: u32, stream: u32, from: u32, count: i32) -> i32 {
    let len = fn_802e441c(from as usize);
    if len == 0 {
        return count + len;
    }
    if platform::call3(callback as usize, stream, from, len as u32) != 0 {
        return count + len;
    }
    -1
}

/// Fetches one integer argument for the wide driver.
///
/// # Original
/// Host-side helper for [`fn_802e3998`]; the `ll` / `j` type codes (4 / 6)
/// read two words, everything else one, and only the `short` code (2) is
/// truncated - zero-extended for the unsigned conversions, sign-extended for
/// `%d` / `%i`.
///
/// # Safety
/// `ap` must be a valid `va_list` cursor.
unsafe fn fetch_wide_integer(ap: u32, type_code: u8, signed: bool) -> (u32, u32) {
    match type_code {
        4 | 6 => {
            let slot = platform::call2(FN_802DC6B8, ap, 2) as usize;
            (platform::read_u32(slot), platform::read_u32(slot + 4))
        }
        _ => {
            let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
            let raw = platform::read_u32(slot);
            let value = if type_code == 2 && signed {
                (raw as u16 as i16 as i32) as u32
            } else if type_code == 2 {
                raw & 0xFFFF
            } else {
                raw
            };
            (value, 0)
        }
    }
}

/// The wide-character `printf` engine (MSL `__wprintf` core).
///
/// In plain English: "walk the wide format string, copy the literal runs to
/// the output callback, parse each `%` field with [`fn_802e21fc`], fetch the
/// argument out of the `va_list`, convert it with the matching wide MSL
/// routine, then emit it with the width / padding rules."
///
/// # Original
/// - Ghidra: `FUN_802e3998` at `0x802e3998` (2444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e3998_FUN_802e3998.c`
///
/// # Arguments
/// * `callback` - `param_1`; the output function, called as
///   `callback(stream, units, len)` with a wide `u16` buffer, returning
///   non-zero on success.
/// * `stream` - `param_2`; opaque first argument for the callback.
/// * `fmt` - `param_3`; the NUL-terminated wide format string.
/// * `ap` - `param_4`; the `va_list` cursor.
/// * `bounded` - `param_5`; non-zero selects the `swprintf` behaviour, where a
///   null `%s` / `%ls` argument and `%n` are errors.
///
/// # Returns
/// The number of wide characters the format produces, or `-1` when the
/// callback fails or a bounded conversion overflows.
///
/// # Side effects
/// Calls `callback` once per literal run / conversion / padding unit, writes
/// the count through `%n`, and fills the spec block.
///
/// # Safety
/// `callback` must be a valid `extern "C"` function with the three-argument
/// shape, `fmt` readable, `ap` a valid argument cursor.
///
/// # Confidence
/// medium - the body is the wide twin of [`fn_802e0bf0`] and every case, the
/// padding loop and the `%n` store match it instruction for instruction. The
/// conversions now call the bodies in this file ([`fn_802e2c8c`],
/// [`fn_802e3208`], [`fn_802e278c`], [`fn_802e29d0`]) instead of absolute
/// addresses. `// inferred:` the narrow `%s` branch reads its first byte from
/// the string pointer (the export names a stale register there) and the two
/// `mbstowcs` / `mbtowc` helpers stay call-by-address targets because they sit
/// outside this bucket.
pub unsafe fn fn_802e3998(
    callback: u32,
    stream: u32,
    fmt: usize,
    ap: u32,
    bounded: i32,
) -> i32 {
    let mut count = 0_i32;
    let mut cursor = fmt;
    let mut spec = [0_u8; 16];
    // The wide scratch buffer: 512 units, filled backwards from `+1024`.
    let mut scratch = [0_u16; 512];
    let base = scratch.as_mut_ptr() as usize;
    let scratch_end = base + 1024; // the export's `auStack_52 + 2`
    let text_end = base + 1022; // the export's `auStack_52`: the NUL slot

    loop {
        if platform::read_u16(cursor as usize) == 0 {
            return count;
        }
        let percent = fn_802e44cc(cursor, 0x25);
        if percent == 0 {
            let len = fn_802e441c(cursor);
            if len == 0 {
                return count + len;
            }
            if platform::call3(callback as usize, stream, cursor as u32, len as u32) != 0 {
                return count + len;
            }
            return -1;
        }

        let run = ((percent - cursor) / 2) as i32;
        count += run;
        if run != 0
            && platform::call3(callback as usize, stream, cursor as u32, run as u32) == 0
        {
            return -1;
        }

        cursor = fn_802e21fc(percent as u32, ap, spec.as_mut_ptr() as u32) as usize;

        let flags = spec[0];
        let precision_given = spec[2];
        let alternative = spec[3];
        let type_code = spec[4];
        let conv = u16::from_be_bytes([spec[6], spec[7]]) as u32;
        let width = i32::from_be_bytes([spec[8], spec[9], spec[10], spec[11]]);
        let precision = i32::from_be_bytes([spec[12], spec[13], spec[14], spec[15]]);

        let mut text;
        let mut len;

        match conv {
            0x25 => {
                scratch[0] = b'%' as u16;
                text = base;
                len = 1;
            }
            0x41 | 0x61 => {
                // 'A' / 'a'
                let slot = platform::call2(FN_802DC6B8, ap, 3) as usize;
                let value = read_f64(slot);
                let mut copy = [0_u8; 16];
                copy_spec(&spec, copy.as_mut_ptr() as usize);
                let produced =
                    fn_802e2c8c(value, scratch_end as u32, copy.as_mut_ptr() as u32) as usize;
                if produced == 0 {
                    return emit_rest_of_format_wide(callback, stream, percent as u32, count);
                }
                text = produced;
                len = ((text_end - produced) / 2) as i32;
            }
            0x45 | 0x46 | 0x47 | 0x65 | 0x66 | 0x67 => {
                // E F G e f g
                let slot = platform::call2(FN_802DC6B8, ap, 3) as usize;
                let value = read_f64(slot);
                let mut copy = [0_u8; 16];
                copy_spec(&spec, copy.as_mut_ptr() as usize);
                let produced =
                    fn_802e3208(value, scratch_end as u32, copy.as_mut_ptr() as usize) as usize;
                if produced == 0 {
                    return emit_rest_of_format_wide(callback, stream, percent as u32, count);
                }
                text = produced;
                len = ((text_end - produced) / 2) as i32;
            }
            0x58 | 0x6F | 0x75 | 0x78 => {
                // X o u x
                let value = fetch_wide_integer(ap, type_code, false);
                let mut copy = [0_u8; 16];
                copy_spec(&spec, copy.as_mut_ptr() as usize);
                let produced = if type_code == 4 || type_code == 6 {
                    fn_802e29d0(
                        value.0,
                        value.1,
                        scratch_end as u32,
                        copy.as_mut_ptr() as u32,
                    )
                } else {
                    fn_802e278c(value.0, scratch_end as u32, copy.as_mut_ptr() as u32)
                } as usize;
                if produced == 0 {
                    return emit_rest_of_format_wide(callback, stream, percent as u32, count);
                }
                text = produced;
                len = ((text_end - produced) / 2) as i32;
            }
            0x64 | 0x69 => {
                // 'd' / 'i'
                let value = fetch_wide_integer(ap, type_code, true);
                let mut copy = [0_u8; 16];
                copy_spec(&spec, copy.as_mut_ptr() as usize);
                let produced = if type_code == 4 || type_code == 6 {
                    fn_802e29d0(
                        value.0,
                        value.1,
                        scratch_end as u32,
                        copy.as_mut_ptr() as u32,
                    )
                } else {
                    fn_802e278c(value.0, scratch_end as u32, copy.as_mut_ptr() as u32)
                } as usize;
                if produced == 0 {
                    return emit_rest_of_format_wide(callback, stream, percent as u32, count);
                }
                text = produced;
                len = ((text_end - produced) / 2) as i32;
            }
            0x63 => {
                // 'c': the wide modifier stores the unit directly, the narrow
                // one goes through `mbtowc` (`0x802DED94`).
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                if type_code == 5 {
                    scratch[0] = platform::read_u32(slot) as u16;
                    text = base;
                    len = 1;
                } else {
                    let mut mb = [0_u8; 2];
                    mb[0] = platform::read_u8(slot);
                    len = platform::call3(
                        0x802D_ED94,
                        base as u32,
                        mb.as_ptr() as u32,
                        1,
                    ) as i32;
                    text = base;
                }
            }
            0x6E => {
                // 'n': store the character count so far.
                if bounded != 0 {
                    // observed: the export calls `FUN_802e4648(0, 0, -1)`.
                    fn_802e4648();
                    return -1;
                }
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                let out = platform::read_u32(slot) as usize;
                match type_code {
                    0 | 3 | 7 | 8 => platform::write_i32(out, count),
                    2 => platform::write_u16(out, count as u16),
                    4 | 6 => {
                        platform::write_i32(out + 4, count);
                        platform::write_i32(out, count >> 31);
                    }
                    _ => {}
                }
                continue;
            }
            0x73 => {
                // 's'
                let slot = platform::call2(FN_802DC6B8, ap, 1) as usize;
                let argument = platform::read_u32(slot) as usize;
                if type_code == 5 {
                    if bounded != 0 && argument == 0 {
                        fn_802e4648();
                        return -1;
                    }
                    let wide = if argument == 0 {
                        EMPTY_WIDE_8042_9BB0
                    } else {
                        argument
                    };
                    if alternative == 0 {
                        if precision_given == 0 {
                            len = fn_802e441c(wide);
                        } else {
                            let end =
                                fn_802e21d4(wide, 0, precision as u32) as usize;
                            len = if end != 0 {
                                ((end - wide) / 2) as i32
                            } else {
                                precision
                            };
                        }
                        text = wide;
                    } else {
                        // `#`: the first unit is a one-byte length prefix,
                        // and the text starts after it.
                        len = (platform::read_u16(wide) as u32 & 0xFF) as i32;
                        text = wide + 2;
                        if precision_given != 0 && precision < len {
                            len = precision;
                        }
                    }
                } else {
                    if bounded != 0 && argument == 0 {
                        fn_802e4648();
                        return -1;
                    }
                    let narrow = if argument == 0 {
                        0x802F_DFE8 // the empty string ""
                    } else {
                        argument
                    };
                    let mut source = narrow;
                    let wanted = if alternative == 0 {
                        if precision_given == 0 {
                            platform::call1(FN_802DC69C, narrow as u32) as i32
                        } else {
                            let end = platform::call3(
                                FN_802DF040,
                                narrow as u32,
                                0,
                                precision as u32,
                            ) as usize;
                            if end != 0 {
                                (end - narrow) as i32
                            } else {
                                precision
                            }
                        }
                    } else {
                        // `// inferred:` a counted string: the first byte is
                        // the length, and the text starts after it (the export
                        // names a stale register for the read, but its narrow
                        // twin at `0x802E0BF0` advances the pointer).
                        let mut first = platform::read_u8(narrow) as i32;
                        source = narrow + 1;
                        if precision_given != 0 && precision < first {
                            first = precision;
                        }
                        first
                    };
                    let produced = platform::call3(
                        0x802D_EDFC,
                        base as u32,
                        source as u32,
                        wanted as u32,
                    ) as i32;
                    if produced < 0 {
                        return emit_rest_of_format_wide(
                            callback,
                            stream,
                            percent as u32,
                            count,
                        );
                    }
                    text = base;
                    len = produced;
                }
            }
            _ => {
                return emit_rest_of_format_wide(callback, stream, percent as u32, count);
            }
        }

        // Width and padding, in wide units.
        let mut field_len = len;
        let mut remaining = len;
        if flags != 0 {
            let pad = if flags == 2 { b'0' as u16 } else { b' ' as u16 };
            let first = platform::read_u16(text);
            if (first == 0x2B || first == 0x2D || first == 0x20) && pad == b'0' as u16 {
                if platform::call3(callback as usize, stream, text as u32, 1) == 0 {
                    return -1;
                }
                text += 2;
                remaining = field_len - 1;
            }
            if flags == 2 && (conv == 0x61 || conv == 0x41) {
                // `0x` / `0X` after a zero-padded hexadecimal float prefix.
                if remaining < 2 {
                    return -1;
                }
                if platform::call3(callback as usize, stream, text as u32, 2) == 0 {
                    return -1;
                }
                remaining -= 2;
                text += 4;
            }
            while field_len < width {
                let unit = pad;
                if platform::call3(
                    callback as usize,
                    stream,
                    &unit as *const u16 as u32,
                    1,
                ) == 0
                {
                    return -1;
                }
                field_len += 1;
            }
        }
        if remaining != 0
            && platform::call3(callback as usize, stream, text as u32, remaining as u32) == 0
        {
            return -1;
        }
        if flags == 0 {
            while field_len < width {
                let unit = b' ' as u16;
                if platform::call3(
                    callback as usize,
                    stream,
                    &unit as *const u16 as u32,
                    1,
                ) == 0
                {
                    return -1;
                }
                field_len += 1;
            }
        }
        count += field_len;
    }
}

/// Sets the "reply pending" flag and reports success.
///
/// In plain English: "store 1 into the word at `r13 - 0x4648` and return 1."
///
/// # Original
/// - Ghidra: `FUN_802e9cc8` at `0x802e9cc8` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e9cc8_FUN_802e9cc8.c`
/// - observed: `li r6,1` / `stw r6,-0x4648(r13)` at `0x802E9CE0`/`0x802E9CF8`,
///   then two `lis`/`cmplwi` probes (`0xEDCC0000` vs `0x5678` and
///   `0x87AA0000` vs `0x3412`) at `0x802E9CFC..0x802E9D2C`. Both compare a
///   high-half constant with a zero-extended 16-bit immediate and can never be
///   equal on Broadway, so the image always falls through, skips the second
///   store and reaches `li r3,1` at `0x802E9D30`.
///
/// # Arguments
/// None.
///
/// # Returns
/// One.
///
/// # Side effects
/// Writes `1` to `0x8042B038` (`r13 - 0x4648`) and leaves the dead four-byte
/// scratch word at `r1+8` alone.
///
/// # Safety
/// None; the target word is in the image's BSS.
///
/// # Confidence
/// high - every instruction in the 116-byte extent is accounted for; the
/// removed-block warnings only covered the two constant branches, which are
/// resolved above.
pub unsafe fn fn_802e9cc8() -> u32 {
    // The dead r1+8 scratch word (0x12/0x56/0x86/0x78) is never read back.
    platform::write_u32(sda13(-0x4648), 1);
    1
}

// ---------------------------------------------------------------------------
// Wave 106 - privileged leaves, FPSCR accessors, cache flush and SPR dump.
// ---------------------------------------------------------------------------
//
// Each helper below is a one-instruction stand-in, not a recovered function.
// The PowerPC build emits the original instruction; the host twin is documented
// as empty/zero so that `cargo check` works on the development machine (the
// same rule `boot::cache` and `runtime::os` follow).

/// Reads one special-purpose register with `mfspr`.
///
/// # Original
/// Host-side helper; no binary address. `SPR` is the register number from the
/// PowerPC register map (for example `1008` = `HID0`, `912` = `GQR0`).
///
/// # Safety
/// Privileged registers can only be read in supervisor state; the caller must
/// know the register is readable in its context.
///
/// # Confidence
/// high - one instruction, no side effects.
#[cfg(target_arch = "powerpc")]
unsafe fn read_spr<const SPR: u32>() -> u32 {
    let value: u32;
    core::arch::asm!("mfspr {0}, {1}", out(reg) value, const SPR, options(nostack));
    value
}

/// Host twin of `read_spr`; reports zero.
///
/// # Confidence
/// high for the signature, `low` for the value (target-only instruction).
#[cfg(not(target_arch = "powerpc"))]
unsafe fn read_spr<const SPR: u32>() -> u32 {
    0
}

/// Writes one special-purpose register with `mtspr`.
///
/// # Original
/// Host-side helper; no binary address. `SPR` is the register number from the
/// PowerPC register map (for example `26` = `SRR0`, `912` = `GQR0`).
///
/// # Safety
/// Privileged registers can only be written in supervisor state; the caller
/// must know the value is legal for the register and the current context.
///
/// # Confidence
/// high - one instruction, no side effects beyond the register write.
#[cfg(target_arch = "powerpc")]
unsafe fn write_spr<const SPR: u32>(value: u32) {
    core::arch::asm!("mtspr {0}, {1}", const SPR, in(reg) value, options(nostack));
}

/// Host twin of `write_spr`; does nothing.
///
/// # Confidence
/// high for the signature, `low` for the effect (target-only instruction).
#[cfg(not(target_arch = "powerpc"))]
unsafe fn write_spr<const SPR: u32>(_value: u32) {}

/// Reads one segment register with `mfsr`.
///
/// # Original
/// Host-side helper; no binary address. `SR` is the segment number `0..15`.
///
/// # Safety
/// Supervisor only.
///
/// # Confidence
/// high - one instruction.
#[cfg(target_arch = "powerpc")]
unsafe fn read_sr<const SR: u32>() -> u32 {
    let value: u32;
    core::arch::asm!("mfsr {0}, {1}", out(reg) value, const SR, options(nostack));
    value
}

/// Host twin of `read_sr`; reports zero.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn read_sr<const SR: u32>() -> u32 {
    0
}

/// Reads one half of the time base with `mftb`.
///
/// # Original
/// Host-side helper; no binary address. `TBR` is `268` (`TBL`) or `269`
/// (`TBU`).
///
/// # Safety
/// Readable in user mode on Gekko, but the value is only meaningful when the
/// two halves are read back to back.
///
/// # Confidence
/// high - one instruction.
#[cfg(target_arch = "powerpc")]
unsafe fn read_tbr<const TBR: u32>() -> u32 {
    let value: u32;
    core::arch::asm!("mftb {0}, {1}", out(reg) value, const TBR, options(nostack));
    value
}

/// Host twin of `read_tbr`; reports zero.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn read_tbr<const TBR: u32>() -> u32 {
    0
}

/// Reads the machine-state register with `mfmsr`.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Safety
/// Supervisor only.
///
/// # Confidence
/// high - one instruction.
#[cfg(target_arch = "powerpc")]
unsafe fn read_msr() -> u32 {
    let msr: u32;
    core::arch::asm!("mfmsr {0}", out(reg) msr, options(nostack));
    msr
}

/// Host twin of `read_msr`; reports zero.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn read_msr() -> u32 {
    0
}

/// Writes the machine-state register with `mtmsr`.
///
/// # Original
/// Host-side helper; no binary address.
///
/// # Safety
/// Supervisor only, and the new image immediately changes translation,
/// interrupt and FPU enable state.
///
/// # Confidence
/// high - one instruction.
#[cfg(target_arch = "powerpc")]
unsafe fn write_msr(msr: u32) {
    core::arch::asm!("mtmsr {0}", in(reg) msr, options(nostack));
}

/// Host twin of `write_msr`; does nothing.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn write_msr(_msr: u32) {}

/// Writes back, flushes, synchronises and invalidates one cache block.
///
/// # Original
/// Host-side helper; the four instructions `dcbst`, `dcbf`, `sync`, `icbi` of
/// `0x802E8E1C` (`0x7C00286C`, `0x7C0028AC`, `0x7C0004AC`, `0x7C002FAC`).
///
/// # Safety
/// `addr` need not be aligned; the instruction only touches cache state.
///
/// # Confidence
/// high - instruction order matches the export.
#[cfg(target_arch = "powerpc")]
unsafe fn cache_block_publish(addr: u32) {
    core::arch::asm!(
        "dcbst 0, {0}",
        "dcbf 0, {0}",
        "sync",
        "icbi 0, {0}",
        addr = in(reg) addr,
    );
}

/// Host twin of `cache_block_publish`; does nothing.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn cache_block_publish(_addr: u32) {}

/// Discards prefetched instructions with `isync`.
///
/// # Original
/// Host-side helper; `0x4C00012C` at the end of `0x802E8E1C`.
///
/// # Confidence
/// high.
#[cfg(target_arch = "powerpc")]
unsafe fn instruction_synchronize() {
    core::arch::asm!("isync");
}

/// Host twin of `instruction_synchronize`; does nothing.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn instruction_synchronize() {}

/// Stores the eight-byte `mffs` image at `out`, preserving `f31`.
///
/// # Original
/// Host-side helper; the nine instructions of `0x802ED6A4` (`0x9421FFC0`,
/// `0xDBE10010`, `0xF3E10020`, `0xFFE0048E`, `0xDBE30000`, `0xE3E10020`,
/// `0xCBE10010`, `0x38210040`, `0x4E800020`).
///
/// # Safety
/// `out` must be an 8-byte writable address.
///
/// # Confidence
/// high - the sequence is literal and closes with `addi r1` / `blr`.
#[cfg(target_arch = "powerpc")]
unsafe fn fpscr_image_to(out: usize) {
    core::arch::asm!(
        "stwu 1, -0x40(1)",
        "stfd 31, 0x10(1)",
        "psq_st 31, 0x20(1), 0, 0",
        "mffs 31",
        "stfd 31, 0({out})",
        "psq_l 31, 0x20(1), 0, 0",
        "lfd 31, 0x10(1)",
        "addi 1, 1, 0x40",
        out = in(reg) out,
    );
}

/// Host twin of `fpscr_image_to`; does nothing.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn fpscr_image_to(_out: usize) {}

/// Installs the eight-byte FPSCR image at `src`, preserving `f31`.
///
/// # Original
/// Host-side helper; the nine instructions of `0x802ED6C8` (`0x9421FFC0`,
/// `0xDBE10010`, `0xF3E10020`, `0xCBE30000`, `0xFDFEFD8E`, `0xE3E10020`,
/// `0xCBE10010`, `0x38210040`, `0x4E800020`). Ghidra removed the `lfd` /
/// `mtfsf` block as unreachable, so the words come from `work/dol/main.elf`.
///
/// # Safety
/// `src` must be a readable 8-byte address holding an FPSCR image.
///
/// # Confidence
/// high - `0xFDFEFD8E` decodes as `mtfsf 255,f31` (XO 711, FLM `0xFF`,
/// FRB 31), matching the `lfd f31,0(r3)` immediately before it.
#[cfg(target_arch = "powerpc")]
unsafe fn fpscr_image_from(src: usize) {
    core::arch::asm!(
        "stwu 1, -0x40(1)",
        "stfd 31, 0x10(1)",
        "psq_st 31, 0x20(1), 0, 0",
        "lfd 31, 0({src})",
        "mtfsf 255, 31",
        "psq_l 31, 0x20(1), 0, 0",
        "lfd 31, 0x10(1)",
        "addi 1, 1, 0x40",
        src = in(reg) src,
    );
}

/// Host twin of `fpscr_image_from`; does nothing.
#[cfg(not(target_arch = "powerpc"))]
unsafe fn fpscr_image_from(_src: usize) {}

/// Raises the debugger's unconditional trap gate.
///
/// In plain English: "trap to the exception handler, which services the request
/// and resumes the caller with a status byte in `r3`."
///
/// # Original
/// - Ghidra: `FUN_802e890c` at `0x802e890c` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e890c_FUN_802e890c.c`
/// - Instructions: `0x0FE00000` = `twi 31,r0,0`, `0x4E800020` = `blr`
///
/// # Arguments
/// - `arg1`..`arg4`: `r3`..`r6`. The trap reads none of them; the exception
///   handler reads the saved context, and the only caller [`fn_802ec0f4`]
///   passes a mode word, two words and a buffer address this way.
///
/// # Returns
/// The status byte the handler leaves in `r3` (0 = success, 2 = retry).
///
/// # Side effects
/// Raises a trap exception. `TO = 31` makes the trap unconditional.
///
/// # Safety
/// A trap handler must be installed; on a retail boot the exception is fatal.
/// PowerPC build only.
///
/// # Confidence
/// medium - the instruction words and the argument / result convention are
/// literal in the export and in [`fn_802ec0f4`]; that the handler writes the
/// result back into the interrupted `r3` is `// inferred:` from that calling
/// convention, because the handler `0x802EC9CC` is only partly translated (its
/// register teardown is not).
#[cfg(target_arch = "powerpc")]
pub unsafe fn fn_802e890c(arg1: u32, arg2: u32, arg3: u32, arg4: u32) -> u8 {
    let status: u32;
    // observed: 0x0FE00000 = twi 31,r0,0 (TO = 31 traps on every condition).
    core::arch::asm!(
        "twi 31, 0, 0",
        inlateout("r3") arg1 => status,
        in("r4") arg2,
        in("r5") arg3,
        in("r6") arg4,
        options(nostack),
    );
    status as u8
}

/// Host twin of [`fn_802e890c`]; records the call and reports success.
///
/// # Original
/// - Same address `0x802e890c`; the host cannot raise a PowerPC trap.
///
/// # Confidence
/// high for the encoding; the host return value is a stand-in.
#[cfg(not(target_arch = "powerpc"))]
pub unsafe fn fn_802e890c(_arg1: u32, _arg2: u32, _arg3: u32, _arg4: u32) -> u8 {
    0
}

/// Publishes `addr..addr+size` to instruction memory.
///
/// In plain English: "align down to the cache step, fold the skipped lead
/// bytes into the count, then write back, flush, synchronise and invalidate
/// every 8-byte step of the range, ending with `isync`."
///
/// # Original
/// - Ghidra: `FUN_802e8e1c` at `0x802e8e1c` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e8e1c_FUN_802e8e1c.c`
///
/// # Arguments
/// - `addr`: `param_1`; first byte of the range.
/// - `size`: `param_2`; signed byte count.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `dcbst`, `dcbf`, `sync` and `icbi` per 8-byte step, then one `isync`.
///
/// # Safety
/// `addr`/`size` must describe memory the caller may publish. The loop always
/// runs at least once, including for `size <= 0`, exactly as the `do`/`while`
/// does.
///
/// # Confidence
/// high - the mask `0xFFFFFFF1`, the `subf` / `add` adjustment, the four
/// instructions, the 8-byte step and the signed `bge` bound are literal in the
/// export; the instruction words at `0x802E8E2C..0x802E8E4C` agree.
pub unsafe fn fn_802e8e1c(addr: u32, size: i32) {
    // observed: `lis r5,0xFFFF / ori r5,r5,0xFFF1 / and r5,r5,r3`.
    const START_MASK: u32 = 0xFFFF_FFF1;
    // observed: `addic r5,r5,8` / `addic. r4,r4,-8`.
    const STEP: u32 = 8;

    let mut p = addr & START_MASK;
    // observed: `subf r3,r5,r3; add r4,r4,r3`.
    let mut left = size.wrapping_add(addr.wrapping_sub(p) as i32);
    loop {
        cache_block_publish(p);
        p = p.wrapping_add(STEP);
        left = left.wrapping_sub(STEP as i32);
        if left < 0 {
            break;
        }
    }
    instruction_synchronize();
}

/// Copies every readable segment register, SPR and time-base half into the
/// debugger's dump block.
///
/// In plain English: "read the 96 registers the debugger reports, in this exact
/// order, into `0x804273E8..0x80427568`."
///
/// # Original
/// - Ghidra: `FUN_802ea4f8` at `0x802ea4f8` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ea4f8_FUN_802ea4f8.c`
///
/// # Arguments
/// None. The original overwrites `r2` with the block base `0x80427240` and
/// stores through it; the absolute destination of every store is listed below.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 96 words at `0x804273E8..0x80427568`. `0x80427538` is not written.
///
/// # Safety
/// Privileged register reads; supervisor state required. The destination is the
/// image's BSS, so it is writable.
///
/// # Confidence
/// medium - the store order and destination addresses are literal in the
/// export, and the SPR / SR numbers are those Ghidra printed for each source
/// (`in_sr0`, `in_TBLr`, `in_IBAT0U`, `in_spr233`, ...); the unnamed
/// `in_sprNNN` sources are numbered from the hex in their Ghidra names. The
/// `unaff_GQR*` sources are GQR0..7, which the export leaves unattributed but
/// the instruction stream at `0x802EA684..0x802EA6B4` reads as SPRs 912..919.
pub unsafe fn fn_802ea4f8() {
    // observed: `lis r2,0x8042 / ori r2,r2,0x7240`; each `stmw` stores a
    // consecutive register run, so the order below is the address order.
    platform::write_u32(0x804273E8, read_sr::<0>()); // sr0
    platform::write_u32(0x804273EC, read_sr::<1>()); // sr1
    platform::write_u32(0x804273F0, read_sr::<2>()); // sr2
    platform::write_u32(0x804273F4, read_sr::<3>()); // sr3
    platform::write_u32(0x804273F8, read_sr::<4>()); // sr4
    platform::write_u32(0x804273FC, read_sr::<5>()); // sr5
    platform::write_u32(0x80427400, read_sr::<6>()); // sr6
    platform::write_u32(0x80427404, read_sr::<7>()); // sr7
    platform::write_u32(0x80427408, read_sr::<8>()); // sr8
    platform::write_u32(0x8042740C, read_sr::<9>()); // sr9
    platform::write_u32(0x80427410, read_sr::<10>()); // sr10
    platform::write_u32(0x80427414, read_sr::<11>()); // sr11
    platform::write_u32(0x80427418, read_sr::<12>()); // sr12
    platform::write_u32(0x8042741C, read_sr::<13>()); // sr13
    platform::write_u32(0x80427420, read_sr::<14>()); // sr14
    platform::write_u32(0x80427424, read_sr::<15>()); // sr15
    platform::write_u32(0x80427428, read_tbr::<268>()); // TBL
    platform::write_u32(0x8042742C, read_tbr::<269>()); // TBU
    platform::write_u32(0x80427430, read_spr::<1008>()); // HID0 = 0x3f0
    platform::write_u32(0x80427434, read_spr::<1009>()); // HID1 = 0x3f1
    platform::write_u32(0x80427438, read_spr::<27>()); // SRR1 = 0x1b
    platform::write_u32(0x8042743C, read_spr::<287>()); // PVR = 0x11f
    platform::write_u32(0x80427440, read_spr::<528>()); // IBAT0U = 0x210
    platform::write_u32(0x80427444, read_spr::<529>()); // IBAT0L = 0x211
    platform::write_u32(0x80427448, read_spr::<530>()); // IBAT1U = 0x212
    platform::write_u32(0x8042744C, read_spr::<531>()); // IBAT1L = 0x213
    platform::write_u32(0x80427450, read_spr::<532>()); // IBAT2U = 0x214
    platform::write_u32(0x80427454, read_spr::<533>()); // IBAT2L = 0x215
    platform::write_u32(0x80427458, read_spr::<534>()); // IBAT3U = 0x216
    platform::write_u32(0x8042745C, read_spr::<535>()); // IBAT3L = 0x217
    platform::write_u32(0x80427460, read_spr::<536>()); // DBAT0U = 0x218
    platform::write_u32(0x80427464, read_spr::<537>()); // DBAT0L = 0x219
    platform::write_u32(0x80427468, read_spr::<538>()); // DBAT1U = 0x21a
    platform::write_u32(0x8042746C, read_spr::<539>()); // DBAT1L = 0x21b
    platform::write_u32(0x80427470, read_spr::<540>()); // DBAT2U = 0x21c
    platform::write_u32(0x80427474, read_spr::<541>()); // DBAT2L = 0x21d
    platform::write_u32(0x80427478, read_spr::<542>()); // DBAT3U = 0x21e
    platform::write_u32(0x8042747C, read_spr::<543>()); // DBAT3L = 0x21f
    platform::write_u32(0x80427480, read_spr::<560>()); // IC_CSR = 0x230
    platform::write_u32(0x80427484, read_spr::<561>()); // IC_ADR = 0x231
    platform::write_u32(0x80427488, read_spr::<562>()); // IC_DAT = 0x232
    platform::write_u32(0x8042748C, read_spr::<563>()); // spr233 = 0x233
    platform::write_u32(0x80427490, read_spr::<564>()); // spr234 = 0x234
    platform::write_u32(0x80427494, read_spr::<565>()); // spr235 = 0x235
    platform::write_u32(0x80427498, read_spr::<566>()); // spr236 = 0x236
    platform::write_u32(0x8042749C, read_spr::<25>()); // SDR1 = 0x19
    platform::write_u32(0x804274A0, read_spr::<19>()); // DAR = 0x13
    platform::write_u32(0x804274A4, read_spr::<18>()); // DSISR = 0x12
    platform::write_u32(0x804274A8, read_spr::<272>()); // SPRG0 = 0x110
    platform::write_u32(0x804274AC, read_spr::<273>()); // SPRG1 = 0x111
    platform::write_u32(0x804274B0, read_spr::<274>()); // SPRG2 = 0x112
    platform::write_u32(0x804274B4, read_spr::<275>()); // SPRG3 = 0x113
    platform::write_u32(0x804274B8, read_spr::<22>()); // DEC = 0x16
    platform::write_u32(0x804274BC, read_spr::<1010>()); // IABR = 0x3f2
    platform::write_u32(0x804274C0, read_spr::<282>()); // EAR = 0x11a
    platform::write_u32(0x804274C4, read_spr::<1013>()); // DABR = 0x3f5
    platform::write_u32(0x804274C8, read_spr::<953>()); // PMC1 = 0x3b9
    platform::write_u32(0x804274CC, read_spr::<954>()); // PMC2 = 0x3ba
    platform::write_u32(0x804274D0, read_spr::<957>()); // PMC3 = 0x3bd
    platform::write_u32(0x804274D4, read_spr::<958>()); // PMC4 = 0x3be
    platform::write_u32(0x804274D8, read_spr::<955>()); // SIA = 0x3bb
    platform::write_u32(0x804274DC, read_spr::<952>()); // MMCR0 = 0x3b8
    platform::write_u32(0x804274E0, read_spr::<956>()); // MMCR1 = 0x3bc
    platform::write_u32(0x804274E4, read_spr::<567>()); // spr237 = 0x237
    platform::write_u32(0x804274E8, read_spr::<568>()); // DC_CST = 0x238
    platform::write_u32(0x804274EC, read_spr::<569>()); // DC_ADR = 0x239
    platform::write_u32(0x804274F0, read_spr::<1019>()); // ICTC = 0x3fb
    platform::write_u32(0x804274F4, read_spr::<1017>()); // L2CR = 0x3f9
    platform::write_u32(0x804274F8, read_spr::<26>()); // SRR0 = 0x1a
    platform::write_u32(0x804274FC, read_spr::<570>()); // DC_DAT = 0x23a
    platform::write_u32(0x80427500, read_spr::<936>()); // UMMCR0 = 0x3a8
    platform::write_u32(0x80427504, read_spr::<937>()); // UPMC1 = 0x3a9
    platform::write_u32(0x80427508, read_spr::<938>()); // UPMC2 = 0x3aa
    platform::write_u32(0x8042750C, read_spr::<939>()); // USIA = 0x3ab
    platform::write_u32(0x80427510, read_spr::<940>()); // UMMCR1 = 0x3ac
    platform::write_u32(0x80427514, read_spr::<941>()); // UPMC3 = 0x3ad
    platform::write_u32(0x80427518, read_spr::<942>()); // UPMC4 = 0x3ae
    platform::write_u32(0x8042751C, read_spr::<571>()); // spr23b = 0x23b
    platform::write_u32(0x80427520, read_spr::<572>()); // spr23c = 0x23c
    platform::write_u32(0x80427524, read_spr::<573>()); // spr23d = 0x23d
    platform::write_u32(0x80427528, read_spr::<574>()); // spr23e = 0x23e
    platform::write_u32(0x8042752C, read_spr::<575>()); // spr23f = 0x23f
    platform::write_u32(0x80427530, read_spr::<920>()); // HID2 = 0x398
    platform::write_u32(0x80427534, read_spr::<1011>()); // spr3f3 = 0x3f3
    // observed: 0x80427538 is skipped by the original.
    platform::write_u32(0x8042753C, read_spr::<912>()); // GQR0 = 0x390
    platform::write_u32(0x80427540, read_spr::<913>()); // GQR1 = 0x391
    platform::write_u32(0x80427544, read_spr::<914>()); // GQR2 = 0x392
    platform::write_u32(0x80427548, read_spr::<915>()); // GQR3 = 0x393
    platform::write_u32(0x8042754C, read_spr::<916>()); // GQR4 = 0x394
    platform::write_u32(0x80427550, read_spr::<917>()); // GQR5 = 0x395
    platform::write_u32(0x80427554, read_spr::<918>()); // GQR6 = 0x396
    platform::write_u32(0x80427558, read_spr::<919>()); // GQR7 = 0x397
    platform::write_u32(0x8042755C, read_spr::<920>()); // HID2 = 0x398
    platform::write_u32(0x80427560, read_spr::<921>()); // WPAR = 0x399
    platform::write_u32(0x80427564, read_spr::<922>()); // DMAU = 0x39a
    platform::write_u32(0x80427568, read_spr::<923>()); // DMAL = 0x39b
}

/// Returns the machine-state register image.
///
/// In plain English: "execute `mfmsr r3` and hand the register back."
///
/// # Original
/// - Ghidra: `FUN_802ec1a0` at `0x802ec1a0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec1a0_FUN_802ec1a0.c`
/// - Instructions: `0x7C6000A6` = `mfmsr r3`, `0x4E800020` = `blr`
///
/// # Arguments
/// None.
///
/// # Returns
/// The complete 32-bit `MSR` image.
///
/// # Side effects
/// None.
///
/// # Safety
/// Supervisor only; the body is the original leaf routine.
///
/// # Confidence
/// high - the export is empty because Ghidra drops the privileged read, but the
/// two instruction words decode unambiguously (`XO` 83, `rD` 3).
pub unsafe fn fn_802ec1a0() -> u32 {
    read_msr()
}

/// Installs the machine-state register image in `msr`.
///
/// In plain English: "execute `mtmsr r3`."
///
/// # Original
/// - Ghidra: `FUN_802ec1a8` at `0x802ec1a8` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec1a8_FUN_802ec1a8.c`
/// - Instructions: `0x7C600124` = `mtmsr r3`, `0x4E800020` = `blr`
///
/// # Arguments
/// - `msr`: `r3`; the new `MSR` image.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Immediately changes translation, interrupt and FPU enable state.
///
/// # Safety
/// Supervisor only, and the write must leave the machine able to continue
/// fetching this code.
///
/// # Confidence
/// high - the two words decode as `mtmsr r3` (`XO` 146) and `blr`.
pub unsafe fn fn_802ec1a8(msr: u32) {
    write_msr(msr)
}

/// Stores the eight-byte FPSCR image at `out`.
///
/// In plain English: "save `f31`, read the FPSCR into it with `mffs`, store the
/// resulting double to `*out`, then restore `f31`."
///
/// # Original
/// - Ghidra: `FUN_802ed6a4` at `0x802ed6a4` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed6a4_FUN_802ed6a4.c`
/// - Instructions: `stwu`, `stfd f31`, `psq_st f31`, `mffs f31`,
///   `stfd f31,0(r3)`, `psq_l f31`, `lfd f31`, `addi r1`, `blr`
///
/// # Arguments
/// - `out`: `r3`; the eight-byte destination (the TRK reply buffer).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes eight bytes at `out`. `f31` is preserved, including its
/// paired-single quantization, through the `psq_st` / `psq_l` pair.
///
/// # Safety
/// `out` must be an 8-byte writable address.
///
/// # Confidence
/// high - the nine words are literal; the decompiler's `ldexpf(GQR0)` probes
/// are paired-single artifacts of the `psq_st` / `psq_l` save, not calls.
pub unsafe fn fn_802ed6a4(out: usize) {
    fpscr_image_to(out);
}

/// Installs the eight-byte FPSCR image at `src`.
///
/// In plain English: "save `f31`, load the image from `*src` into it, copy it
/// into the FPSCR with `mtfsf`, then restore `f31`."
///
/// # Original
/// - Ghidra: `FUN_802ed6c8` at `0x802ed6c8` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ed6c8_FUN_802ed6c8.c`
/// - Instructions: `stwu`, `stfd f31`, `psq_st f31`, `lfd f31,0(r3)`,
///   `mtfsf 255,f31`, `psq_l f31`, `lfd f31`, `addi r1`, `blr`
///
/// # Arguments
/// - `src`: `r3`; the eight-byte image to install.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Replaces the FPSCR (exception enables, rounding and flush modes).
///
/// # Safety
/// `src` must be an 8-byte readable address. The caller is responsible for
/// installing a legal FPSCR image.
///
/// # Confidence
/// medium - Ghidra removed the `lfd` / `mtfsf` block as unreachable and shows
/// only the GQR probes, so the body is recovered from the raw words; `0xFDFEFD8E`
/// decodes as `mtfsf 255,f31` (XO 711), which fixes both the source and the
/// field mask.
pub unsafe fn fn_802ed6c8(src: usize) {
    fpscr_image_from(src);
}

// ---------------------------------------------------------------------------
// Wave 111 - the Payne-Hanek range-reduction tail.
// ---------------------------------------------------------------------------
//
// This is the last member of the `0x802E0000` bucket whose Ghidra export still
// describes its own instruction stream: the `__kernel_rem_pio2` kernel that
// `sin` / `cos` / `tan` reach for arguments above `2^20 * pi/2`. The two
// exception-frame routines at the top of the bucket stay documented rather
// than translated; `docs/COVERAGE_WAVE111_MAIN_LOOP.md` records why.

/// Reduces a huge angle with the 2/pi bit table (MSL `__kernel_rem_pio2`).
///
/// In plain English: "multiply the 24-bit chunks of `x` by the matching chunks
/// of `2/pi` taken from `ipio2`, take the low three bits of the integer part as
/// the quadrant, and write the fraction back as one, two or three doubles."
///
/// The routine never forms `(2/pi)*x`. It keeps `q[0..=jk]`, the 24-bit chunks
/// of the product, in `f64`, distils them into the integer array `iq[]`, and
/// only then rebuilds the fraction with `scalbn` and the eight shipped `PIo2`
/// chunks. That is why the operation count does not grow with the exponent.
///
/// # Original
/// - Ghidra: `FUN_802e624c` at `0x802e624c` (5816 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802e624c_FUN_802e624c.c`
/// - Constant tables: `init_jk` at `0x802FE1B8` (`{2, 3, 4, 6}` in the image,
///   *not* the fdlibm `{3, 4, 4, 6}`), `PIo2` at `0x802FE1C8` (eight doubles).
///
/// # Arguments
/// - `x`: `param_1` / `r3`; address of the `nx` 24-bit digits of `|x|`, each
///   held in a `f64` (`x[0] * 2^e0` reconstructs the original).
/// - `y`: `param_2` / `r4`; receives the reduced value and its corrections.
/// - `e0`: `param_3`; exponent of `x[0]`.
/// - `nx`: `param_4`; number of valid digits in `x`.
/// - `prec`: `param_5`; `0` = 24-bit, `1` = 53-bit, `2` = 64-bit,
///   `3` = 113-bit output precision. The one caller in this file uses `2`.
/// - `ipio2`: `param_6` / `r8`; address of the 24-bit chunks of `2/pi`. The
///   caller passes `0x802FE030`.
///
/// # Returns
/// The quadrant `n & 7`: the low three bits of the integer part of `(2/pi)*x`.
/// The sign of the reduced value is carried by `n` itself (`0..3` positive,
/// `4..7` negative), which is why the callers negate on `n < 0`.
///
/// # Side effects
/// Writes `y[0]`, `y[1]` and (for `prec == 3`) `y[2]`. Nothing else leaves the
/// stack frame: `iq[20]`, `f[20]`, `q[20]` and `fq[20]` are local.
///
/// # Safety
/// `x` must hold `nx` readable doubles, `y` must have `prec == 3 ? 3 : 2`
/// writable doubles, and `ipio2` must hold at least `jv + jz + k` words of the
/// 2/pi table. The two tables at `0x802FE1B8` / `0x802FE1C8` must be intact.
///
/// # Confidence
/// medium - every operand, table index, rounding step and store is literal in
/// the export, and the `init_jk` / `PIo2` contents were read from
/// `work/dol/main.elf`. The compiler unrolled each convolution eight ways and
/// re-derived the loop counts as `int` range guards; those guards are always
/// true for the 20-element arrays, so the bodies below are the loops Ghidra
/// itself reconstructed. Two points are `// inferred:` - the `scalbn` argument
/// order at the `n` computation (Ghidra prints the caller's leftover `r4` as a
/// second argument) and the `(double)(int32_t)w` conversions, which the export
/// spells as the `CONCAT44(0x43300000, w ^ 0x80000000) - (2^52 + 2^31)` magic
/// word pair; the subtrahend was read from the image at `0x8042FAB0`.
pub unsafe fn fn_802e624c(
    x: usize,
    y: usize,
    e0: i32,
    nx: i32,
    prec: i32,
    ipio2: usize,
) -> i32 {
    /// `(double)(int32_t)w` through the MSL magic-word sequence, i.e.
    /// `f64::from_bits(0x43300000_00000000 | (w ^ 0x80000000)) - (2^52 + 2^31)`.
    #[inline]
    fn word_to_f64(w: u32, two52p31: f64) -> f64 {
        f64::from_bits(0x4330_0000_0000_0000 | ((w ^ 0x8000_0000) as u64)) - two52p31
    }

    // observed: the ten `.sdata2` constants, resolved through `r2 = 0x80433080`.
    let zero = read_f64(sda2(-0x3608));
    let one = read_f64(sda2(-0x35D8));
    let two24 = read_f64(sda2(-0x35F8));
    let twon24 = read_f64(sda2(-0x3600));
    let two52p31 = read_f64(sda2(-0x35D0)); // 2^52 + 2^31 = 4503601774854144.0
    let eighth = read_f64(sda2(-0x35E8));
    let eight = read_f64(sda2(-0x35F0));
    let half = read_f64(sda2(-0x35E0));

    // observed: `lwz r3, (0x802fe1b8 + prec*4)` - MSL `init_jk[prec]`.
    let jk = platform::read_u32(0x802F_E1B8 + prec as usize * 4) as i32;
    let jx = nx - 1;

    // observed: `jv = (e0-3)/24` clamped at zero, then `q0 = e0 - 24*(jv+1)`.
    // Ghidra's `(x/24) + (x>>31)` / `- (x>>31)` pair is the compiler's signed
    // division; for every `e0 >= 3` it is plain truncation, and everything
    // below zero is clamped, so the two forms agree.
    let mut jv = (e0 - 3) / 24;
    if jv < 0 {
        jv = 0;
    }
    let mut q0 = e0 - 24 * (jv + 1);

    let mut f = [zero; 20];
    let mut q = [zero; 20];
    let mut fq = [zero; 20];
    let mut iq = [0u32; 20];

    // observed: `for (i=0,j=jv-jx; i<=jx+jk; i++,j++)` with `zero` for j < 0.
    let m = jx + jk;
    let mut j = jv - jx;
    let mut i = 0;
    while i <= m {
        f[i as usize] = if j < 0 {
            zero
        } else {
            word_to_f64(platform::read_u32(ipio2 + j as usize * 4), two52p31)
        };
        i += 1;
        j += 1;
    }

    // observed: `for(i=0;i<=jk;i++) { for(j=0,fw=0;j<=jx;j++) fw += x[j]*f[jx+i-j]; }`
    i = 0;
    while i <= jk {
        let mut fw = zero;
        let mut j = 0;
        while j <= jx {
            fw += read_f64(x + j as usize * 8) * f[(jx + i - j) as usize];
            j += 1;
        }
        q[i as usize] = fw;
        i += 1;
    }

    let mut jz = jk;
    let mut n: u32;
    let mut ih: i32;
    let mut z: f64;

    // `recompute:` - the extension loop below can jump back here with a larger
    // `jz`, so the distill / quadrant step is a real loop in the binary.
    loop {
        // observed: distill `q[]` into 24-bit chunks, high chunk first.
        z = q[jz as usize];
        let mut i = 0;
        let mut j = jz;
        while j > 0 {
            let fw = word_to_f64((twon24 * z) as i32 as u32, two52p31);
            iq[i as usize] = (z - two24 * fw) as i32 as u32;
            z = q[(j - 1) as usize] + fw;
            i += 1;
            j -= 1;
        }

        // observed: `z = scalbn(z,q0); z -= 8.0*floor(z*0.125); n = (int)z;`
        z = fn_802e4660(z, q0);
        let mut fw = fn_802e8084(eighth * z);
        z -= eight * fw;
        n = z as i32 as u32;
        z -= word_to_f64(n, two52p31);
        ih = 0;

        // observed: the three ways to recover the high bits that `(int)z` lost.
        let last = iq[(jz - 1) as usize];
        if q0 > 0 {
            let high = (last as i32 >> (24 - q0)) as u32;
            iq[(jz - 1) as usize] = last.wrapping_sub(high << (24 - q0));
            n = n.wrapping_add(high);
            ih = iq[(jz - 1) as usize] as i32 >> (23 - q0);
        } else if q0 == 0 {
            ih = last as i32 >> 23;
        } else if z >= half {
            ih = 2;
        }

        // observed: `if(ih>0) { n += 1; ... compute 1-q ... }`
        if ih > 0 {
            n = n.wrapping_add(1);
            let mut carry = false;
            let mut i = 0;
            while i < jz {
                let word = iq[i as usize];
                if carry {
                    iq[i as usize] = 0x00FF_FFFF - word;
                } else if word != 0 {
                    iq[i as usize] = 0x0100_0000 - word;
                    carry = true;
                }
                i += 1;
            }
            if q0 == 1 {
                iq[(jz - 1) as usize] &= 0x007F_FFFF;
            } else if q0 == 2 {
                iq[(jz - 1) as usize] &= 0x003F_FFFF;
            }
            if ih == 2 {
                z = one - z;
                if carry {
                    z -= fn_802e4660(one, q0);
                }
            }
        }

        // observed: "check if recomputation is needed" - only when z == 0.
        if z == zero {
            let mut any = 0u32;
            let mut i = jz - 1;
            while i >= jk {
                any |= iq[i as usize];
                i -= 1;
            }
            if any == 0 {
                // `k` = number of extra 2/pi terms the cancellation swallowed.
                let mut k = 1;
                while iq[(jk - k) as usize] == 0 {
                    k += 1;
                }
                let mut i = jz + 1;
                while i <= jz + k {
                    f[(jx + i) as usize] = word_to_f64(
                        platform::read_u32(ipio2 + (jv + i) as usize * 4),
                        two52p31,
                    );
                    let mut fw = zero;
                    let mut j = 0;
                    while j <= jx {
                        fw += read_f64(x + j as usize * 8) * f[(jx + i - j) as usize];
                        j += 1;
                    }
                    q[i as usize] = fw;
                    i += 1;
                }
                jz += k;
                continue;
            }
        }
        break;
    }

    // observed: drop the now-zero high chunks, or split the fraction again.
    if z == zero {
        jz -= 1;
        q0 -= 24;
        while iq[jz as usize] == 0 {
            jz -= 1;
            q0 -= 24;
        }
    } else {
        z = fn_802e4660(z, -q0);
        if z >= two24 {
            let fw = (twon24 * z) as i32 as u32;
            iq[jz as usize] = (z - two24 * word_to_f64(fw, two52p31)) as i32 as u32;
            jz += 1;
            q0 += 24;
            iq[jz as usize] = fw;
        } else {
            iq[jz as usize] = z as i32 as u32;
        }
    }

    // observed: `fw = scalbn(one,q0); q[i] = fw*(double)iq[i]; fw *= twon24;`
    let mut fw = fn_802e4660(one, q0);
    let mut i = jz;
    while i >= 0 {
        q[i as usize] = fw * word_to_f64(iq[i as usize], two52p31);
        fw *= twon24;
        i -= 1;
    }

    // observed: `fq[jz-i] = sum PIo2[k]*q[i+k]` over the shipped `PIo2` table.
    i = jz;
    while i >= 0 {
        let mut fw = zero;
        let mut k = 0;
        while k <= jk && k <= jz - i {
            fw += read_f64(0x802F_E1C8 + k as usize * 8) * q[(i + k) as usize];
            k += 1;
        }
        fq[(jz - i) as usize] = fw;
        i -= 1;
    }

    // observed: `switch(prec)` - compress `fq[]` into `y[]`.
    if prec == 1 || prec == 2 {
        let mut fw = zero;
        let mut i = jz;
        while i >= 0 {
            fw += fq[i as usize];
            i -= 1;
        }
        write_f64(y, if ih == 0 { fw } else { -fw });
        fw = fq[0] - fw;
        let mut i = 1;
        while i <= jz {
            fw += fq[i as usize];
            i += 1;
        }
        write_f64(y + 8, if ih == 0 { fw } else { -fw });
    } else if prec == 0 {
        let mut fw = zero;
        let mut i = jz;
        while i >= 0 {
            fw += fq[i as usize];
            i -= 1;
        }
        write_f64(y, if ih == 0 { fw } else { -fw });
    } else if prec == 3 {
        // observed: two `fq[i-1]+fq[i]` renormalisation passes.
        let mut i = jz;
        while i > 0 {
            let fw = fq[(i - 1) as usize] + fq[i as usize];
            fq[i as usize] += fq[(i - 1) as usize] - fw;
            fq[(i - 1) as usize] = fw;
            i -= 1;
        }
        let mut i = jz;
        while i > 1 {
            let fw = fq[(i - 1) as usize] + fq[i as usize];
            fq[i as usize] += fq[(i - 1) as usize] - fw;
            fq[(i - 1) as usize] = fw;
            i -= 1;
        }
        let mut fw = zero;
        let mut i = jz;
        while i >= 2 {
            fw += fq[i as usize];
            i -= 1;
        }
        if ih == 0 {
            write_f64(y, fq[0]);
            write_f64(y + 8, fq[1]);
            write_f64(y + 16, fw);
        } else {
            write_f64(y, -fq[0]);
            write_f64(y + 8, -fq[1]);
            write_f64(y + 16, -fw);
        }
    }

    (n & 7) as i32
}

// ---------------------------------------------------------------------------
// Wave 113 - the two exception frames at the top of the bucket.
// ---------------------------------------------------------------------------
//
// `0x802EC9CC` (prologue) and `0x802ECCB4` (epilogue) are the MetroTRK
// exception entry points. The external interrupt reaches the prologue through
// `0x802E96AC` -> `0x802E9624` -> `0x802EC9CC`; the handler re-arm path reaches
// the epilogue through `0x802EA4C4` -> `0x802ECCB4`. Both replace `r1` and save
// or restore every GPR plus LR / CTR / XER / CR / MSR / DAR / DSISR, so neither
// can be executed from Rust: a body that performed the real save / restore
// would have to be a whole-function `asm!` block, which this file's policy
// reserves for single-instruction leaves (see the Wave 106 section). What *is*
// reconstructed below is the ordinary C part: the MSR mask, the exception
// number and dispatch-flag stores, the nested-exception ladder, the event
// codes and the calls into the translated TRK bodies. Every register-only step
// is listed at the address it occupies, and both rows are `partial`.

/// Shared dispatch tail of [`fn_802ec9cc`] and [`fn_802eccb4`].
///
/// In plain English: "if a debugger request is already in flight, clear the
/// in-flight flag and hand the saved context back; otherwise classify the
/// exception, read the faulting instruction for a program or trace exception,
/// and queue a 3 / 4 / 5 event."
///
/// This is the block at `0x802ECBFC`. It sits inside the 744-byte extent of
/// [`fn_802ec9cc`] but is a third entry point of that routine: the prologue
/// branches to it at `0x802ECB5C` and the epilogue at `0x802ECDC8`, which is
/// why both Ghidra exports contain the same event block.
///
/// # Original
/// - Ghidra: `LAB_802ecbfc`, folded into `FUN_802ec9cc`; evidence
///   `work/ghidra/main-gb/export/802ec9cc_FUN_802ec9cc.c`
/// - Instructions: `0x802ECBFC..0x802ECCA4`
///
/// # Arguments
/// None. The context block base `0x80427198` and the TRK globals are absolute.
///
/// # Returns
/// `0x80427198` when the dispatch flag was set, otherwise the queue status from
/// [`fn_802e9b00`] (`0` queued, `0x100` when the queue was full).
///
/// # Side effects
/// * clears the dispatch byte `0x80427234`;
/// * for `0x700` / `0xD00` exceptions, reads four bytes at the saved PC through
///   [`fn_802ec374`];
/// * writes a three-word event through [`fn_802e9bcc`] and queues it through
///   [`fn_802e9b00`].
///
/// # Safety
/// `0x804272C0` must hold the saved PC and the TRK event queue must be mapped.
///
/// # Confidence
/// medium - the flag test, the `0x700` / `0xD00` test, the compare against
/// `0x0FE00000` (the `twi 31,r0,0` encoding, see [`fn_802e890c`]) and the
/// 3 / 4 / 5 codes are literal in `0x802ECC10..0x802ECCA0`.
unsafe fn exception_dispatch_tail() -> u32 {
    // observed 0x802ECC10..0x802ECC24: `lwz r0,0x9C(r3)` with r3 = 0x80427198,
    // then a store of zero and a branch to the frame teardown at 0x802ECCA4.
    if platform::read_u32(G_80427234) != 0 {
        platform::write_u32(G_80427234, 0);
        return G_80427198 as u32;
    }

    // observed 0x802ECC30..0x802ECC38: `lwz r0,0x2F8(r3)` with r3 = 0x80427240
    // and `rlwinm r0,r0,0,16,31` - the low half of the duplicated exception
    // number.
    let exception = platform::read_u16(G_80427538) as u32;
    let code;
    if exception == 0x700 || exception == 0xD00 {
        // observed 0x802ECC48..0x802ECC6C: read the instruction word at the
        // saved PC into a four-byte stack slot. The call is
        // `fn_802ec374(&word, [0x804272C0], &len, 0, 1)`.
        let mut word: u32 = 0;
        let mut len: u32 = 4;
        fn_802ec374(
            &mut word as *mut u32 as usize as u32,
            platform::read_u32(G_804272C0),
            &mut len as *mut u32 as usize,
            0,
            1,
        );
        // observed 0x802ECC70..0x802ECC90: `addis r0,r3,-4064` followed by
        // `cmpli r0,0` is an equality test against 0x0FE00000.
        code = if word == 0x0FE0_0000 { 5 } else { 3 };
    } else {
        // observed 0x802ECC90: `li r4,4`.
        code = 4;
    }

    // observed 0x802ECC94..0x802ECCA0: `li r3,0x10(r1)`, `bl 0x802E9BCC`,
    // `li r3,0x10(r1)`, `bl 0x802E9B00`; the second return is the r3 the
    // epilogue hands back.
    let mut event = [0_u32; 3];
    fn_802e9bcc(event.as_mut_ptr() as usize, code);
    fn_802e9b00(event.as_mut_ptr() as usize)
}

/// MetroTRK exception prologue: mask interrupts, record the exception and send
/// program / trace / external exceptions to the debugger.
///
/// In plain English: "turn interrupts off, record the exception number, and if
/// the exception arrived while the debugger was already handling one, resume
/// the interrupted code at once; otherwise save the whole register file and the
/// SPRs, mark the debugger busy, and queue the event."
///
/// The routine has three entry points inside its 744-byte extent: itself,
/// `0x802ECB60` (the nested-exception return) and `0x802ECBFC` (the shared
/// dispatch tail, [`exception_dispatch_tail`]). Ghidra folded the last two into
/// this body.
///
/// # Original
/// - Ghidra: `FUN_802ec9cc` at `0x802ec9cc` (744 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ec9cc_FUN_802ec9cc.c`
/// - Caller: `0x802E9624` (translated as [`fn_802e9624`]) via `0x802E96AC`,
///   which is the external-interrupt path and always passes `0x500`.
/// - Instructions: `0x802EC9CC..0x802ECB5C`, `0x802ECB60..0x802ECBF8`,
///   `0x802ECBFC..0x802ECCA4`
///
/// # Arguments
/// * `exception` - `r3`; the exception number (`0x500` external interrupt,
///   `0x700` program, `0xD00` trace).
/// * `resume_pc` - `r4`; the saved PC the preceding context trampoline passes
///   in. The original stashes it in `SRR1` at `0x802EC9D0`; the two fast
///   returns are the only users.
/// * `_r5`..`_r10` - `r5`..`r10`; stored into the exception frame at
///   `0x80427254..0x80427268`, part of the untranslated save.
///
/// # Returns
/// The shared tail's value (`0x80427198` or the [`fn_802e9b00`] queue status).
/// The `rfi` paths do not return to this caller; they report 0 here and the
/// machine-state teardown is documented in place.
///
/// # Side effects
/// * halfword exception number at `0x8042B058`, twice at `0x80427538`;
/// * installs the `0x80427224` MSR image with `MSR.EE` / `MSR.RI` cleared;
/// * sets the dispatch byte `0x80427234` and the in-exception byte
///   `0x8035162C`, and sets byte `0x8035162D` on the nested return;
/// * stores the current `SRR0` at `0x80351620` and may advance it by four;
/// * calls `0x802E981C` (empty), [`fn_802ea4f8`] (SPR dump),
///   [`fn_802ec374`] and [`exception_dispatch_tail`].
///
/// # Safety
/// Called only from the TRK exception vector: the frame at `0x80427240` must be
/// writable, the context block at `0x80427198` must be populated, and
/// `0x80427238` must hold the handler pointer.
///
/// # Confidence
/// low - the MSR mask, the stores, the comparison ladder, the event codes and
/// every call are literal in the decoded words, but the register save / restore
/// (`0x802EC9CC..0x802EC9E0`, `0x802ECABC..0x802ECAF4`,
/// `0x802ECB0C..0x802ECB5C`) and all three `rfi` teardowns are machine state
/// with no Rust representation, so the exits return instead of executing them.
/// `// inferred:` `resume_pc` is the frame word at `object + 0x19C` forwarded by
/// [`fn_802e9624`]. The body reads live `SRR0` at `0x802ECB6C` where the
/// original reads back the `r2` its entry stash put there, because that stash
/// is part of the untranslated register half.
pub unsafe fn fn_802ec9cc(
    exception: u32,
    resume_pc: u32,
    _r5: u32,
    _r6: u32,
    _r7: u32,
    _r8: u32,
    _r9: u32,
    _r10: u32,
) -> u32 {
    // partial: 0x802EC9CC..0x802EC9E0 stashes r2 in SRR0, r4 in SRR1, SPRG3 in
    // r4 and the CR image in SPRG3: `mtspr SRR0,r2`, `mtspr SRR1,r4`,
    // `mfspr r4,SPRG3`, `mfcr r2`, `mtspr SPRG3,r2`. `resume_pc` is the r4
    // half of that pair.
    let _ = resume_pc;

    // observed 0x802EC9E0..0x802EC9FC: `lwz r2,0x8C(0x80427198)` loads the MSR
    // image at 0x80427224, `ori 0x8002` then `xori 0x8002` clears EE and RI,
    // and `sync; mtmsr r2; sync` installs it.
    fn_802ec1a8(platform::read_u32(G_80427224) & !0x8002);

    // observed 0x802ECA00..0x802ECA08: `sth r3,0(r2)` with r2 = 0x8042B058.
    platform::write_u16(G_8042B058, exception as u16);

    if exception == 0x500 {
        // partial: 0x802ECA14..0x802ECA34 saves LR at 0x8042766C, calls the
        // empty 0x802E981C and restores LR.
        fn_802e981c();

        // observed 0x802ECA38..0x802ECA60: the first byte of the handler the
        // pointer at 0x80427238 names, and the in-exception byte at
        // 0x8035162C, decide whether the interrupt returns straight away.
        let handler = platform::read_u32(G_80427238) as usize;
        if platform::read_u8(handler) == 0 || platform::read_u8(G_8035162C) == 1 {
            // partial: 0x802ECA78..0x802ECA90 reloads CR from 0x80427178, r3
            // from 0x804270FC and r2 from 0x804270F8, then `rfi`.
            return 0;
        }
        // observed 0x802ECA64..0x802ECA74: `stb 1,0x9C(0x80427198)` and branch
        // to the common test at 0x802ECA94.
        platform::write_u8(G_80427234, 1);
    }

    // observed 0x802ECA94..0x802ECAB0: reload the exception halfword and test
    // the in-exception byte at 0x8035162C.
    if platform::read_u8(G_8035162C) != 0 {
        // observed 0x802ECB60..0x802ECB74: store the exception at 0x80351628
        // and the current SRR0 at 0x80351620. The original's SRR0 is the value
        // its entry stash (`mtspr SRR0,r2`, not executed above) put there; the
        // body reads the live register instead.
        platform::write_u16(G_80351628, exception as u16);
        platform::write_u32(G_80351620, read_spr::<26>());

        // observed 0x802ECB78..0x802ECBC8: these exceptions consumed the
        // faulting instruction, so the return advances SRR0 by four; every
        // other exception retries the same instruction.
        let nested = platform::read_u16(G_80351628) as u32;
        if nested == 0x200
            || nested == 0x300
            || nested == 0x400
            || nested == 0x600
            || nested == 0x700
            || nested == 0x800
            || nested == 0x1000
            || nested == 0x1100
            || nested == 0x1200
            || nested == 0x1300
        {
            // observed 0x802ECBCC..0x802ECBD4: `mfspr r3,SRR0; addi r3,4;
            // mtspr SRR0,r3`.
            write_spr::<26>(read_spr::<26>().wrapping_add(4));
        }
        // observed 0x802ECBD8..0x802ECBE4: `li r3,1; stb r3,13(0x80351620)`.
        platform::write_u8(G_8035162C + 1, 1);
        // partial: 0x802ECBE8..0x802ECBF8 restores CR from SPRG3, r2 from
        // SPRG1 and r3 from SPRG2, then `rfi`.
        return 0;
    }

    // partial: 0x802ECAB4..0x802ECAF4 builds the exception frame at 0x80427240:
    // r0 at +0, r1 at +4, SPRG1 at +8, SPRG3 at +0xC, r4..r31 at +0x10, and
    // SRR0 (which the entry put r2 in), LR, SPRG3 (the CR image), CTR and XER
    // at +0x80..+0x90.
    //
    // observed 0x802ECACC..0x802ECAD0: the exception number twice at 0x80427538.
    platform::write_u16(G_80427538, exception as u16);
    platform::write_u16(G_80427538 + 2, exception as u16);

    // observed 0x802ECAF8: `bl 0x802EA4F8` dumps the SPRs.
    fn_802ea4f8();

    // observed 0x802ECAFC..0x802ECB08: `li r3,1; stb r3,12(0x80351620)`.
    platform::write_u8(G_8035162C, 1);

    // partial: 0x802ECB0C..0x802ECB5C restores the previous context block at
    // 0x80427198 (MSR, LR, CTR, XER, DSISR, DAR, r3..r31, r0, r1, r2) and
    // branches to the shared tail at 0x802ECBFC.
    exception_dispatch_tail()
}

/// MetroTRK exception epilogue: save the machine state, mask interrupts and
/// either hand the exception to the installed handler or resume the interrupted
/// code.
///
/// In plain English: "park the whole register file and the exception SPRs in
/// the context block, turn interrupts off, and if a handler is installed mark a
/// request in flight and go to the shared dispatch tail; otherwise clear the
/// in-exception state and leave through `rfi`."
///
/// # Original
/// - Ghidra: `FUN_802eccb4` at `0x802eccb4` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802eccb4_FUN_802eccb4.c`
/// - Caller: `0x802EA4C4` (translated as [`fn_802ea4c4`]) re-arms the handlers
///   through this routine.
///
/// # Arguments
/// None. `r3`..`r10` are stored into the context block by the untranslated
/// save; Ghidra lists them only because the caller leaves them live.
///
/// # Returns
/// The shared tail's value (`0x80427198` or the [`fn_802e9b00`] queue status)
/// when the handler path is taken, otherwise 0. The `rfi` path does not return
/// to this caller.
///
/// # Side effects
/// * fills `0x80427198..0x8042722C` with r0..r31, LR, CTR, XER, MSR, DAR and
///   DSISR;
/// * installs `MSR & !0x8002` through [`fn_802ec1a8`];
/// * sets the dispatch byte `0x80427234`, or clears the in-exception byte
///   `0x8035162C` and calls [`fn_802ea6bc`];
/// * may queue an event through [`exception_dispatch_tail`].
///
/// # Safety
/// Only valid on the TRK exception path: the block at `0x80427198` must be
/// writable and `0x80427238` must hold the handler pointer.
///
/// # Confidence
/// low - the handler test, the `MSR & !0x8002` mask, the callee wiring and the
/// tail branch are literal; `// partial:` the `stmw` / `lmw` register runs and
/// both `rfi` teardowns are machine state and are documented in place instead
/// of executed.
pub unsafe fn fn_802eccb4() -> u32 {
    // partial: 0x802ECCB4..0x802ECD00 saves r0..r31 with `stmw r0,0`, MSR at
    // +0x8C, LR at +0x80, CTR at +0x84, XER at +0x88, DAR at +0x90 and DSISR
    // at +0x94 into 0x80427198.

    // observed 0x802ECCF0..0x802ECD00: `li r1,0x8002; nor r1,r1,r1` builds
    // ~0x8002, `mfmsr r3; and r3,r3,r1; mtmsr r3` clears EE and RI.
    fn_802ec1a8(fn_802ec1a0() & !0x8002);

    // observed 0x802ECD04..0x802ECD18: test the first byte of the handler the
    // pointer at 0x80427238 names.
    let handler = platform::read_u32(G_80427238) as usize;
    if platform::read_u8(handler) != 0 {
        // observed 0x802ECD1C..0x802ECD2C: `stb 1,0x9C(0x80427198)` marks the
        // request in flight, then the branch at 0x802ECD2C reloads the saved
        // context and reaches 0x802ECBFC.
        platform::write_u8(G_80427234, 1);
        // partial: 0x802ECD78..0x802ECDC8 restores the MSR, LR, CTR, XER,
        // DSISR, DAR, r3..r31, r0, r1 and r2 from 0x80427198 and branches to
        // the shared tail.
        exception_dispatch_tail()
    } else {
        // observed 0x802ECD30..0x802ECD40: clear the in-exception byte and the
        // two exception flags.
        platform::write_u8(G_8035162C, 0);
        fn_802ea6bc();
        // partial: 0x802ECD44..0x802ECD74 restores r27..r31 and SRR0 / LR /
        // CR / CTR / XER from 0x80427240, then r3..r31, r0, r1 and r2, and
        // leaves through `rfi`.
        0
    }
}

