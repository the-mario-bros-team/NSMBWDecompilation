# PORT PLAN — a runnable, fully playable NSMBW on Windows (Rust + Vulkan)

Written 2026-11-16, after wave 293. This is a plan, not a status report.

## 0. Why a new architecture is needed

The existing crate (`src/`, ~52 MB) is a source-level reconstruction. It cannot
run as a game, for three structural reasons:

- `src/platform.rs` dereferences Wii virtual addresses (`0x80xxxxxx`) as host
  pointers, which crashes on the first read on Windows.
- 22,203 call sites call other functions **by Wii address**
  (`platform::callN(0x801C6AF0, ...)`) through `transmute`, and the helpers pass
  only integers (float arguments in `f1..` are lost).
- Nothing replaces the hardware the game drives: GX (graphics), AX/DSP (audio),
  DVD, Wiimote over Bluetooth, NAND, the OS scheduler, interrupts, VI, and IOS.

In addition, no body has ever been executed, so its behavioural correctness is
unknown. Making those 20,000 bodies runnable would require fixing the memory
model, every call site, and every behavioural bug found along the way.

**Decision: the runtime runs the game's own machine code, translated
mechanically.** The CPU code comes from the shipped `main.dol` and the four RELs,
first through an interpreter and then through a static recompiler that emits
Rust. The hardware is replaced by Rust high-level emulation (HLE) at a small
number of well-defined boundaries. Graphics go to Vulkan.

The existing work is still useful, as **analysis** rather than as runtime code:

- The ledger (20,252 addresses), the gap-function corpus and the pointer-table
  survey seed the recompiler's function list and indirect-branch targets.
- Measured extents and recovered jump tables become recompiler hints.
- Names and `# Original` blocks become a symbol map, so crash logs show
  `OSLink (0x801B1F20)` instead of a bare address.
- Identified SDK entry points (`OSLink` `0x801B1F20`, `OSCreateThread`
  `0x801B5130`, and others in `src/runtime/*`) are the hook points HLE needs.

Recommendation: **pause the translation waves.** Each wave adds bodies that the
runtime will not execute. Resume them later only if you want idiomatic Rust
replacements for specific functions (mods, fixes), and validate each one against
the recompiled original.

## 1. Inputs and environment (checked on this PC)

| Item | State |
|---|---|
| Rust | `cargo 1.97.1`, `rustc 1.97.1` |
| GPU | NVIDIA GeForce RTX 5070 Ti (Vulkan 1.3 capable); `vulkan-1.dll` present |
| Vulkan SDK | **not installed** (`VULKAN_SDK` unset). Needed only for validation layers during development. |
| Dolphin | user data present (`%APPDATA%\Dolphin Emulator`); used as the reference implementation and for FIFO logs |
| `work/dol/main.elf`, `work/rels/*.rel`, `work/rels/elf/*.elf` | present |
| Extracted disc filesystem (`sys/`, `files/`) | present at `E:\mariobroswii` (SMNE01 rev 1), set in `port.toml`. `nsmbw-port --check` confirmed it is byte-identical to `work/dol/main.elf` and `work/rels/*.rel`. Other regions and revisions fail the check: the PAL disc (SMNP01) differs from `text0` onward. |

**Legal boundary:** every game byte (code, `fst.bin`, assets) is read at runtime
from the user's own extracted disc. Recompiler output is derived from
Nintendo's code: generate it locally into a git-ignored folder and never publish
it. Dolphin is GPL-2.0+: read it as a reference for behaviour, and do not copy
its code unless the port is also licensed under the GPL.

## 2. Architecture

```
            +-------------------------- frontend (winit window, config, menus) ---+
            |                                                                     |
 user's     |   guest-cpu          guest-mem             hle-os                   |
 extracted  |   - interpreter      - MEM1 24 MiB         - boot (low-mem globals) |
 disc  ---> |   - recompiled fns   - MEM2 64 MiB         - threads (coroutines)   |
            |   - dispatch table   - MMIO router         - interrupts, timebase   |
            |        |                  |                - OSLink hook (REL map)  |
            |        v                  v                                         |
            |   hle-ios (IPC regs 0xCD000000)    gx (WGPipe 0xCC008000, CP/PE)    |
            |   - /dev/fs  -> host save dir      - FIFO decoder                  |
            |   - /dev/di  -> extracted files    - vertex loader, XF, BP state   |
            |   - /dev/es, /dev/stm, stubs       - texture decode + cache         |
            |   - /dev/usb/oh1 -> emulated BT    - EFB/XFB model                  |
            |       HID Wiimotes <- input        |                                 |
            |                                    v                                 |
            |   dsp-ax (DSP mailbox 0xCC005000)  vk-render (ash): TEV ubershader,  |
            |   - AX ucode HLE mixer -> cpal     pipelines, present to swapchain   |
            +---------------------------------------------------------------------+
```

### 2.1 Cargo workspace

The existing `nsmbw` library stays as it is and remains the only
`default-members` entry. A plain `cargo check` (and `scripts/check.ps1`)
therefore still checks only the reconstruction. Build the port with
`cargo build -p frontend`. Crates are added when their milestone starts, not as
empty placeholders; P0 added only `crates/frontend`.

| Crate | Role |
|---|---|
| `crates/guest-mem` | Big-endian MEM1/MEM2, cached and uncached mirrors (`0x8/0xC`, `0x9/0xD`), physical masking for DMA, locked-cache scratch at `0xE0000000`, MMIO routing for `0xCC00xxxx`/`0xCD00xxxx`. |
| `crates/guest-cpu` | `Cpu` context (GPRs, FPRs as paired `f64`, CR, XER, LR, CTR, FPSCR, GQRs, MSR, SRR0/1), instruction decoder, interpreter, and the address → host-function dispatch table. |
| `crates/ppc-recomp` | Build-time tool: reads `main.elf` and the REL ELFs plus the ledger's function list, and writes Rust into `gen/` (git-ignored). |
| `crates/game-recomp` | Thin crate that `include!`s the generated shards and registers them in the dispatch table. |
| `crates/hle-os` | HLE boot, thread coroutines, exception and interrupt delivery, decrementer and timebase, `OSLink` hook, symbol map. |
| `crates/hle-ios` | The IOS devices listed in 2.4 (request handling in guest memory, NAND folder, disc reads). |
| `crates/hw` | Hardware registers: PI interrupt cause/mask and CPU FIFO, IPC registers, VI timing, DSP/ARAM/AI, EXI (RTC, SRAM), SI, and the CP/PE registers in front of `gx`. Added in P2; `dsp-ax` takes over its audio stub. |
| `crates/gx` | FIFO decode, CP/XF/BP register state, vertex loader, transform and lighting (on the CPU for now), texture decoders, encoder and cache, TMEM. Outputs draw batches, EFB copies and per-draw records for frame interpolation (`interp`); reads and writes Dolphin FIFO logs (`dff`). Free of Vulkan types, so it can be unit-tested. |
| `crates/vk-render` | `ash` + `gpu-allocator`: the TEV ubershader (WGSL, compiled by `naga`; indirect texturing, fog, Z textures, destination alpha by dual-source blending), pipelines and their on-disk cache, textures, EFB render target, the EFB copy pass (format conversion, RAM writeback, CPU peeks), XFB images, swapchain present with V-Sync on or off, an overlay, PNG screenshots, and a headless mode. |
| `crates/dsp-ax` | DSP mailbox model, AX microcode HLE mixer, AI output through `cpal`. |
| `crates/input` | Keyboard and `gilrs` gamepads mapped to Wiimote state (buttons, sideways layout, shake, tilt), up to 4 players. |
| `crates/frontend` | The binary: the game on its own thread, the `winit` window with frame pacing and interpolation, display settings (`settings.toml`, hotkeys), the performance monitor, FIFO-log replay, screenshot and trace tools. |

Shaders: write GLSL and compile it to SPIR-V at build time with `naga`, which is
pure Rust and needs no Vulkan SDK. Install the Vulkan SDK anyway for the
validation layers.

### 2.2 CPU: interpreter first, then the recompiler

**The interpreter comes first** because it is:
1. the quickest path to booting the game,
2. the correctness oracle for the recompiler (lockstep comparison), and
3. a permanent fallback for any address the recompiler did not cover, such as
   code reached only through computed jumps.

Instruction semantics need particular care in these places:

- Integer: carry and overflow (`addc/adde/subfe/addze`, `srawi` carry), `rlwinm`
  and `rlwimi` masks, `cntlzw`, `divw` edge cases (result undefined but
  deterministic on hardware), and `lwarx`/`stwcx.`.
- Floating point: single-precision operations round through `f32` the way
  `frsp` does. `fmadd`-family instructions are fused (`f64::mul_add`, then round).
  `fres` and `frsqrte` need the hardware estimate tables, not exact division,
  because physics code can depend on them. `fctiwz` truncates. `fsel` is exact.
- Paired singles: `ps_*` arithmetic, `ps_merge*`, `ps_sum*`, and `psq_l`/`psq_st`
  with GQR quantisation (u8/s8/u16/s16 scaled). The stock Ghidra language could
  not decode these, so expect them in hot code.
- Cache operations: `dcbz` **must** zero 32 bytes. `dcbf`, `dcbst`, `dcbi`,
  `icbi` and `sync` are no-ops, except where `dcbi` interacts with the locked
  cache's DMA.
- `mftb` and the decrementer derive from a guest cycle counter, with the
  timebase at 60.75 MHz (bus clock divided by 4).

**Recompiler design.** It emits one Rust function per guest function:
`fn f_802b3b00(c: &mut Cpu, m: &mut Mem)`.

- Function list: Ghidra `functions.tsv` (all images), the gap corpus, and the
  pointer-table targets from the ledger. Extents use the descent rules from the
  handoff, section 4 (`bcctrl` falls through, and so on).
- Control flow inside a function: a `loop { match block { ... } }` state
  machine over basic blocks. It is simple, always correct, and LLVM optimises
  it well.
- Direct `bl` becomes a direct Rust call. Indirect calls (`bctrl` through
  vtables, function pointers) go through the dispatch table, and miss to the
  interpreter.
- Jump tables (`bctr`) become a `match` when the table is recovered; otherwise
  the interpreter handles them.
- Interrupt and timing checkpoints sit at function entry and on loop
  back-edges: subtract an instruction-count budget, and when it runs out,
  service timers and interrupts.
- REL code: the guest's own `OSLink` runs normally and patches guest memory. A
  hook after `OSLink` records each module's section load bases. Recompiled REL
  code reads relocated values (`lis/addi` pairs, cross-module `bl`) through
  `rel_base(module, section) + addend`, using the ELF relocations, so it is
  correct wherever the heap places the module.
- Output is sharded into about 64 modules for parallel and incremental
  compilation. If `rustc` compile times become unworkable, the fallback is a
  Cranelift JIT, which is still pure Rust.

**Lockstep validator:** run each recompiled function and the interpreter from the
same snapshot, then compare registers and dirtied memory. Run it at boot over
every function the game executes, and on random inputs for leaf functions.

### 2.3 OS layer (`hle-os`)

- **Boot:** skip the apploader. Load the DOL sections and `fst.bin` into MEM1 the
  way the apploader would, populate the low-memory globals (arena low/high,
  memory sizes, console type, bus and CPU clocks, FST pointer, MEM2 bounds, IOS
  version), then enter at the DOL entry point (`__start`).
- **Threads:** the Wii OS does its own context switches through
  `OSSaveContext`/`OSLoadContext`/`rfi`, which are among the "machine-state"
  partials in the ledger. In the interpreter these work naturally, since they
  only swap register state. In recompiled code each guest thread runs on its
  own stackful coroutine (`corosensei`) on one host thread, which is
  deterministic and matches the single-core Broadway. `OSLoadContext(ctx)`
  resumes the coroutine that owns `ctx`.
- **Interrupts:** at a checkpoint, save an `OSContext` exactly as the vector stub
  would, then run the guest handler (`__OSDispatchInterrupt` path). Sources: VI
  retrace, PE finish/token, DSP/AI, IPC (IOS replies), decrementer, and the
  EXI/SI stubs.
- **Timing:** the guest cycle counter advances by instructions executed. Host
  vsync paces VI retrace at 59.94 Hz, and the guest catches up to it.

### 2.4 IOS layer (`hle-ios`) — one boundary: the IPC registers at `0xCD000000`

| Device | Behaviour |
|---|---|
| `/dev/di` | Disc reads, served from the extracted `files/` tree by mapping partition offsets back through `fst.bin`. |
| `/dev/fs` + NAND paths | A host folder (`saves/nand/`). The save file goes in the title's data directory. `SYSCONF` and `setting.txt` are generated with sane defaults (language, 16:9 flag, sensor bar). |
| `/dev/es` | Title ID, ticket views and IOS version: canned answers. |
| `/dev/stm/*` | Reset and power events, wired to window close. |
| `/dev/usb/oh1/57e/305` | Emulated Bluetooth: HCI, L2CAP, and the Wiimote HID report protocol (documented on WiiBrew). It feeds up to four virtual Wiimotes from `crates/input`, with no game-symbol knowledge needed. |
| `/dev/net/*`, WiiConnect24, SD | Report "absent"; the game handles this. |

Bring-up shortcut: before Bluetooth works, hook `KPADRead` once it is located
in the image, so testing can start earlier. Treat this as temporary.

### 2.5 Graphics (`gx` + `vk-render`) — the largest piece

1. **Capture:** guest writes to the write-gather pipe (`0xCC008000`) append to a
   command stream. CP/PI FIFO registers and watermark interrupts are modelled
   enough for the SDK's FIFO handling. PE token and finish raise interrupts,
   since `GXDrawDone` waits on them.
2. **Decode:** NOP, `LOAD_CP_REG`, `LOAD_XF_REG`, `LOAD_INDX_A..D`, `CALL_DL`
   (display lists read from guest memory), `INVL_VC`, `LOAD_BP_REG`, and all
   draw primitives (quads, triangles, strips, fans, lines, line strips, points).
3. **Vertex loader:** VAT and VCD formats (direct, index8, index16; component
   types and fractional bits), converted on the CPU into one host vertex format
   and uploaded per batch.
4. **XF in the vertex shader:** position, normal and texture matrix memory,
   projection and viewport, 2 colour channels × 8 lights, and up to 8 texgens
   including post-transform matrices.
5. **TEV in a fragment ubershader** driven by packed BP state in a uniform or
   storage buffer: 16 stages, colour and alpha combiners, swap tables, konst
   colours, 4 indirect stages (NSMBW water and lava distortion), alpha compare,
   z-texture, fog, and destination alpha. Specialised pipelines can come later,
   behind a cache, if performance needs them.
6. **Textures:** CPU decode of I4, I8, IA4, IA8, RGB565, RGB5A3, RGBA8 and CMPR
   to RGBA8, plus C4/C8/C14X2 with TLUT palettes. The cache is keyed by address,
   format and content hash, and covers mips, LOD bias, wrap, clamp, mirror and
   filtering.
7. **Raster state:** blending, logic ops, depth, culling, scissor and scissor
   offset, colour and alpha update masks.
8. **EFB/XFB:** the EFB is an offscreen target at internal resolution × scale.
   EFB-to-texture copies (with the format conversions, including Z) feed effects
   and screen wipes. EFB-to-XFB copies are virtual: VI presents whichever copy
   matches the XFB address it is pointed at on retrace. CPU EFB peeks and pokes
   get a slow readback path.

**Verified independently of the CPU work:** record Dolphin FIFO logs (`.dff`) of
representative scenes (title, world map, 1-1, underground, water, lava, ghost
house, airship, boss, 4-player), replay them through `gx` + `vk-render`, and
compare against Dolphin screenshots at native resolution. This lets graphics
progress in parallel with sections 2.2–2.4.

### 2.6 Audio (`dsp-ax`)

The game uploads AX microcode to the DSP through the mailbox (`0xCC005000`).
Identify it by hash and run an HLE AX mixer every 5 ms frame:

- parameter blocks from guest memory
- ADPCM and PCM8/16 decode
- polyphase sample-rate conversion
- volume and pan ramps
- the aux A/B/C buses that the game's effect callbacks use
- AI DMA output at 32 kHz, resampled to the device rate for `cpal`

NW4R `snd` (BRSAR and streamed BRSTM music) is guest code and works on top of
this once DVD and AX work. The Wiimote speaker can be silent at first.

## 3. Milestones (each ends with a check you can run)

Estimates assume one experienced developer working full-time with agent help.
They are honest ranges, not promises; graphics accuracy is the main source of
variance.

| # | Milestone | Done when | Estimate |
|---|---|---|---|
| P0 | Workspace and inputs | `cargo run -p frontend -- --check` finds the extracted disc, `main.dol`, the RELs and `fst.bin`, and opens a Vulkan window that clears to a colour, with no validation errors | 2–4 days |
| P1 | Guest core | Instruction test suite passes (≥1 test per opcode form, with special cases for the list in 2.2). The interpreter runs `__start` through `OSInit` until the first wait on emulated hardware. (Reaching `main` moved to P2: `OSInit` itself waits on DSP/ARAM, and IPC comes after that.) | 3–5 weeks |
| P2 | Boot HLE | With GX and DSP stubbed: the game reaches its main loop and VI retrace runs at 60 Hz; save-folder files are created; the log shows the REL modules linked through the `OSLink` hook | 4–6 weeks |
| P3 | GX → Vulkan (parallel with P1–P2) | All reference FIFO logs replay, each within a set PSNR threshold of Dolphin's frame, with no validation errors | 10–16 weeks |
| P4 | Title to World 1-1 | Live game: title screen, file select, world map, and 1-1 played to the flagpole on keyboard and gamepad, through emulated Bluetooth Wiimotes | 4–6 weeks |
| P5 | Audio | Music and SFX in 1-1 and the world map; streamed BRSTM loops cleanly; an A/B comparison against a Dolphin audio dump has no audible defects | 3–5 weeks |
| P6 | Recompiler | Every function executed during a scripted run passes lockstep; 60 fps in 4-player 1-1 with headroom; the interpreter fallback rate is logged and near zero | 6–10 weeks |
| P7 | Whole game | Completion checklist (section 4) passes on a fresh save | 8–16 weeks |
| P8 | Polish | 16:9, internal resolution scaling, controller remapping UI, borderless fullscreen, a one-folder release build (code and assets still read from the user's disc) | 2–4 weeks |

Sequenced with P3 running in parallel: roughly **9–15 months** to "entirely
playable".

## 4. Definition of "entirely playable"

All on a fresh save, through the normal UI:

- [ ] Boot through the strap and warning screens, file select, and a new game
- [ ] Worlds 1–8: every level, tower, castle, airship, ghost house and cannon
- [ ] Every boss fight (Larry through Bowser and the final sequence) and the credits
- [ ] World 9 unlocked via Star Coins and completed
- [ ] Toad houses and their minigames
- [ ] Save, quit, reload: progress, Star Coins and lives persist
- [ ] 2–4 players simultaneously, with bubbles, deaths and rejoining
- [ ] Shake (spin, pick up, carry), tilt platforms, and propeller suit flights work from the mapped controls
- [ ] Super Guide and hint movies play
- [ ] 60 fps with no audio dropouts during a full 4-player level
- [ ] No validation-layer errors and no interpreter-fallback spikes in a full playthrough log

## 5. Risks and mitigations

| Risk | Mitigation |
|---|---|
| GX edge cases (indirect textures, EFB copy formats, TEV precision) | FIFO-log regression suite from day one; Dolphin as a behaviour reference |
| Float divergence (estimates, fused ops, paired-single quantisation) | Exact estimate tables; lockstep against the interpreter; per-op unit tests against values captured in Dolphin |
| Thread and interrupt semantics | Coroutines plus checkpoint delivery; interpreter mode as the simple, trusted fallback |
| Recompiler compile times on ~1M instructions | Sharding, `opt-level=2` for generated crates, and the interpreter during development; Cranelift JIT as a fallback |
| Indirect branches the recompiler missed | Dispatch misses go to the interpreter and are logged, then fed back as new function seeds |
| Bluetooth HID emulation is larger than expected | Temporary `KPADRead` hook so gameplay testing is not blocked |
| Timing-sensitive game code (loading, audio sync) | Cycle-budget tuning; DSP and AI interrupts at their real cadence |

## 6. P0 status (2026-11-16)

Run from `D:\nsmbw`:

- `cargo run -p frontend -- --check` checks the game folder named in
  `port.toml` (or `--game <folder>`): `boot.bin` ID and revision, the DOL
  header, the DOL sections against `work/dol/main.elf`, every `fst.bin` entry
  against `files/`, and the four `*.rel.LZ` modules (LZ11-decompressed) against
  `work/rels`. **Passes** for `E:\mariobroswii`, and fails as intended for the
  PAL disc.
- `cargo run -p frontend -- --frames 300` opens a 1280×720 window on the RTX
  5070 Ti, clears it every frame through a FIFO swapchain, and exits. **Passes**:
  300 frames presented.

Still open:

1. Install the Vulkan SDK, then re-run `--frames 300` in a debug build. P0 is
   done when it reports 0 validation errors **with the layer on**. Today it
   prints `validation layer: not available`, so the "0 errors" it reports means
   nothing yet.
2. Resizing and minimising rebuild the swapchain in code, but have not been
   exercised by hand.
3. Record the first Dolphin FIFO logs (title screen, 1-1) into `work/fifo/` for P3.

## 7. P1 status (2026-11-16)

Added `crates/guest-mem` and `crates/guest-cpu`, plus `--boot` in the frontend.

- **Memory:** MEM1/MEM2 with their cached, uncached and physical mirrors, the
  locked cache at `0xE0000000`, and MMIO routing for `0xCC/0xCD/0xC8`.
- **Interpreter coverage:** every integer, branch, CR, load/store (including
  string, byte-reversed and `lwarx`/`stwcx.`), system (`sc`, `rfi`,
  `mfspr`/`mtspr` with timebase and decrementer), FP and paired-single
  instruction, plus `psq_*` quantisation and locked-cache DMA.
- **FP behaviour:** `lfs`/`stfs` are bit-exact (NaN payloads, denormals), FMA is
  fused, and the frC input of single-precision multiplies is rounded to 25 bits.
- **Tests:** `cargo test -p guest-mem -p guest-cpu` runs 26 tests, all passing.
  They target carries, masks, CR logic, divide edge cases, FP conversions,
  quantisation and DMA.
- **Known approximation:** `fres`/`frsqrte` use exact maths, not the hardware
  estimate tables (see 2.2).
- **Boot:** `cargo run --release -p frontend -- --boot` loads the DOL and FST,
  sets up low memory, and runs from `__start`. It completes `__init_hardware`
  and `__init_data`, then enters `OSInit` (`0x801AA800`),
  where the OS prints, through its own report routine:
  `L2 cache initialized`, `Exceptions initialized...`, and the RVL_SDK EXI and
  SI build banners (`Feb 27 2009`).
- **First hardware wait:** `0x801AC248..50` polls DSP CSR `0xCC00500A` for bit
  `0x20` (ARAM DMA done) inside the audio-system init. That is the start of P2.
- **Speed:** about 32 M instructions/s in a release build, roughly 4% of
  Broadway's 729 MHz. That is fine for boot bring-up but far too slow to play.
  P2–P4 will want a decoded-block cache in the interpreter, and P6 (the
  recompiler) is required for full speed.
- **Low-memory values** in `crates/frontend/src/boot.rs` follow Dolphin's
  direct-boot setup as recalled. P2 checks each one against what `OSInit`
  actually reads.

## 8. P2 status (2026-11-17)

Added `crates/hw`, `crates/hle-ios` and `crates/hle-os`. The boot loader moved
from the frontend into `hle-os`; `--boot` now only reports.

- **Run loop** (`hle-os`): runs 500-instruction slices, then advances the
  hardware and delivers an external (0x500) or decrementer (0x900) exception
  when MSR[EE] is set.
  - **Idle skipping:** when the scheduler's idle loop (`0x801B4FF0`) finds an
    empty run queue (`0x8042A708`), the loop jumps to the next video line
    instead of spinning. About 97% of guest time is skipped this way.
  - **`OSLink` hook:** logs each REL's section addresses.
- **Hardware** (`hw`):
  - PI interrupt cause and mask.
  - VI line timing: 525 lines at 29.97 Hz, with the four display interrupts.
  - The IPC registers, with ack then reply, as IOS does.
  - DSP: ARAM DMA (into MEM2, as Dolphin does on the Wii), the boot-ROM and
    task-boot mail handshakes, and AX command lists answered with a yield
    mail after 2000 cycles. AI DMA interrupts at the real 48 kHz cadence.
  - EXI: the RTC and SRAM chip; memory-card slots report empty.
  - SI: every GameCube port answers "no response".
  - CP/PE stubs plus a gather-pipe command walker:
    - It tracks vertex formats and display-list calls, and raises the PE
      draw-done and token interrupts.
    - It only parses bursts written while the CPU FIFO is linked to the GPU,
      so display lists being recorded are skipped.
    - The FIFO write pointer advances per 32-byte burst, at write time.
- **IOS** (`hle-ios`):
  - `/dev/fs` and NAND files in `saves/nand/`, plus generated `SYSCONF` and
    `setting.txt`.
  - `/dev/es`: title ID, title dir, ticket view, consumption.
  - `/dev/stm`: the event hook is held, as on hardware.
  - `/dev/di`: disc reads mapped through `fst.bin` to the extracted files;
    the trace names each file.
  - `/dev/usb/oh1/57e/305`: an HCI controller with no remote devices. It
    returns command-complete and command-status events, and inquiry finds
    nobody. `/dev/net/*` opens fail, which the game accepts.
- **Tests:** `cargo test -p guest-mem -p guest-cpu -p hle-ios -p hw -p hle-os`
  runs 32 tests, all passing. New this milestone: the `SYSCONF` and
  `setting.txt` formats, NAND path containment, the gather-pipe walker, DSP
  mail handshakes and ARAM DMA.
- **Result:** run `cargo run --release -p frontend -- --boot --steps 20000000000`
  (about 27 s of guest time; the limit counts guest cycles).
  - **Startup:** `main` is reached after 9.8M instructions. Every SDK library
    initialises and prints its banner (GX, VI, WPAD, KPAD, AI, AX, DSP, NW4R
    G3D/LYT/SND/EF), and the Bluetooth stack comes up.
  - **RELs:** all four are linked: `d_profile`, `d_bases`, `d_enemies`, `d_en_boss`.
  - **Loading:** the game loads the strap screen (`wiiStrap.arc`), fonts,
    messages, the HOME-button menu, the sound archive, scene wipes and effects.
  - **Frame loop:** it then settles into its frame loop. The game thread is in
    `frame_loop` → NW4R layout drawing.
  - **Rates:** VI retraces run at 59.9 per guest second, with about 2 draw-done
    syncs and 1.5k draw commands per frame. The host runs about 180 retraces
    per second, 3× real time, at 60–75 M instructions/s.
- **Not done (P2 criteria):**
  1. The game's own save file is not created yet. NSMBW writes it after the
     strap screen, which waits for a button press, so this needs the
     emulated Wiimotes (P4). Only `SYSCONF` and `setting.txt` exist in
     `saves/nand/` so far.
  2. 60 Hz is reached in guest time, not paced to the host clock. Real-time
     pacing belongs with the window loop (P3/P4).
- **Open approximations:**
  - The AX microcode only acknowledges command lists. It writes no mixed
    audio and no voice-state updates (P5). Streams that wait on voice
    progress may stall.
  - The low-memory boot values are still unverified one by one. The boot
    succeeds with them.

## 9. P3 status (2026-11-17, first pass)

Added `crates/gx` and `crates/vk-render`. The frontend's P0 clear-screen
presenter is gone: with no mode flag, `nsmbw-port` now runs the game in a
window, one video field per host frame, paced to 59.94 Hz.

- **`gx`** replaces the P2 command walker in `hw`.
  - **Commands:** decodes every FIFO command, including display-list calls
    and indexed XF loads. The BP mask register is honoured.
  - **Register state:** CP (VCD, VAT, arrays), XF (matrices, lights,
    texgens, viewport, projection), BP (TEV, PE, textures, TLUT loads into a
    TMEM model).
  - **Vertex loader:** all component formats; direct, index8 and index16
    attributes; NBT with three indices.
  - **Transform, on the CPU:**
    - position and normal matrices, projection and viewport;
    - both colour channels with eight lights (diffuse functions, spot and
      specular attenuation);
    - texgens with post-transform matrices.

    The output is already in Vulkan clip space for a 640×528 EFB. This
    differs from the plan in 2.5, which put it in the vertex shader; it can
    move there later behind the same output.
  - **Primitives:** quads, triangles, strips and fans become triangle lists.
    Lines and points become screen-space quads.
  - **Textures:** all ten formats, with TLUTs and mip levels. A cache keyed
    by address, format and a content hash makes each texture decode and
    upload once.
  - **Copies:** EFB copies, both to XFB and to texture, with clears.
- **`vk-render`:**
  - **Ubershader:** one WGSL pair, compiled to SPIR-V at startup by `naga`
    (no SDK needed). The fragment stage runs the TEV in integer maths from
    each draw's packed BP state, found by instance index:
    - 16 stages, swap tables, konst colours, compare modes;
    - clamp and scale;
    - alpha test with both comparisons.
  - **Pipelines:** keyed by blend mode. Cull, depth and scissor are dynamic
    (Vulkan 1.3, dynamic rendering).
  - **Textures and samplers:** bound as descriptor arrays, rewritten per
    frame.
  - **EFB and XFB:** the EFB is RGBA8 + D32 at `--scale` × native (default
    2). XFB copies are kept per guest address, and the one VI register
    0x1C points at is presented, letterboxed to 16:9.
  - **Screenshots:** `--screenshot` saves that XFB as a PNG.
- **Result:**
  - **Strap screen:** renders correctly at 1× and 2×, including the fade-in
    from black and the animated second panel.
  - **Steady state:** about 9 draws per field. The host runs about 117
    fields/s unpaced, and 59.94 when paced.
  - **Commands:**
    ```
    cargo run --release -p frontend -- --frames 400 --unpaced --screenshot shot.png
    cargo run --release -p frontend -- --frames 300   # 0 validation errors reported
    ```
  - The game draws the strap screen identically whether `SYSCONF` says 4:3
    or 16:9. (Because the SDK rejected that `SYSCONF`; fixed in 11.2.)
- **Tests:**

  ```
  cargo test -p guest-mem -p guest-cpu -p hle-ios -p hw -p hle-os -p gx -p vk-render
  ```

  runs 39 tests, all passing. New this milestone:
  - texture formats (I8, RGB5A3, RGBA8, CMPR) and the texture cache;
  - the vertex loader (direct and indexed);
  - viewport mapping;
  - the FIFO decoder and BP mask;
  - a WGSL validation test for the ubershader.
- **P3's criterion (reference FIFO logs replayed against Dolphin) is not met
  yet.** No `.dff` logs exist (P0 item 3), the validation layer is not
  installed (P0 item 1), and every screen after the strap needs input (P4).
- **Not implemented yet** (each needs a scene that uses it to verify; all eleven are done in section 10):
  1. Indirect textures (the state is already packed for the shader). Water
     and lava need these.
  2. Fog.
  3. Constant and destination alpha. This needs dual-source blending.
  4. Z textures and z-freeze.
  5. Logic ops other than clear, copy and no-op.
  6. Per-channel clear masks.
  7. TMEM preloading (BP 0x60–0x63).
  8. Emboss texgens, which are approximated.
  9. EFB copy format conversion: intensity, Z and 6-bit formats.
  10. EFB copies stay on the GPU and are not written to RAM, so CPU reads
      of a copy's destination see stale bytes.
  11. CPU EFB peeks and pokes.

## 10. P3 second pass: the GX features, speed, and display settings (2026-11-17)

### 10.1 Interpreter speed

Measured with Windows' sampling profiler (`xperf`, a new `profiling` build
profile with debug info): nearly all host time was the interpreter. Fetch and
bookkeeping in `Cpu::step` took 27%, decode and simple instructions 33%,
loads and stores 14%. GX, hardware and rendering were under 1% together.

- **Guest memory:** one buffer for MEM1, MEM2 and the locked cache, with a
  4 MiB page table and inlined fast paths. Slow paths (MMIO, edges) are cold
  functions.
- **Decoded-block cache** (`guest-cpu/src/blocks.rs`):
  - Straight runs of up to 32 instructions are decoded once into
    (handler, word) pairs and checked against memory on every entry, so
    rewritten code is decoded again.
  - Blocks end at branches and before the run loop's special PCs (the idle
    loop and `OSLink`).
  - The windowed game uses it (`System::run_fast`); `--boot` keeps the
    per-instruction path for its instrumentation.
- **Dedicated handlers** (`fast.rs`) cover the ~40 most common instruction
  forms, skipping `execute`'s large prologue and two-level dispatch.
  `tests/fast_paths.rs` checks each one against the interpreter on random
  states.
- **Multiply-add:** `fmadd`-family instructions use the host FMA
  instruction; `f64::mul_add` without it is a software routine.

| Path | Before | After |
|---|---|---|
| Game window (`--frames 60 --unpaced`) | ~87 M instructions/s (estimated) | 170–195 M/s |
| `--boot` report (instrumented) | 74 M/s | 77 M/s |
| Tight synthetic loops (`tests/speed.rs`) | 80–120 M/s | 116–228 M/s |

The strap screen still matches the P3 reference pixel for pixel, and
`--boot` runs the same instruction count. **This is still not full speed for
gameplay.** A busy field (about 12 M instructions, as during boot loading)
takes about 60 ms, so expect 15–20 fps where the game is busy until the
recompiler (P6).

### 10.2 GX features

All eleven items section 9 left open are built, each with a test:

1. **Indirect texturing:** lookups with scales, the three static and dynamic
   matrices, bias, wrapping, add-previous, and alpha bump (normalised too).
2. **Fog:** all functions, perspective and orthographic, with range
   adjustment.
3. **Destination alpha:** dual-source blending. The EFB gets the constant
   alpha while blending uses the TEV's.
4. **Z textures:** u8, u16 and u24, add and replace. **Z-freeze:** the depth
   plane of the last unfrozen triangle.
5. **Logic ops:** all sixteen, through Vulkan's logic ops.
6. **Per-channel clear masks:** a masked full-rectangle draw.
7. **TMEM preloading** (BP 0x60–0x63), including RGBA8's split banks.
   Preloaded textures are read from TMEM.
8. **Emboss texgens:** the light direction in tangent space.
9. **EFB copy formats:** a copy pass converts to every copy format,
   intensity and depth copies included, with an exact 2×2 box filter. This
   also fixes a decoding bug: the copy format bits were read unscrambled.
10. **RAM writeback:** each texture copy is also rendered at native size and
    read back; the game thread encodes it into RAM in the copy's texture
    format (`gx::texture::encode`), and the GPU copy stays valid for those
    bytes.
11. **CPU EFB peeks and pokes** at physical `0x08000000`. Pokes are ordered
    with the gather-pipe data. Peeks read a snapshot the renderer refreshes
    each frame while the game keeps peeking.

Also: the texture-coordinate scale registers (BP 0x30–0x3F) now apply to
every lookup.

`vk-render/tests/gx_features.rs` drives hand-built command streams through
`gx` and a headless renderer on the GPU and reads back the EFB. It covers:
- fog, XOR logic op, destination alpha;
- the alpha-only clear, a poke, z-freeze, a Z texture;
- an indirect lookup shifted by four texels;
- an RGB565 copy written back as `F8 00`.

**Caveat:** RAM writeback and EFB peeks arrive one to two frames late,
because the GPU runs after the field. Exact timing would need a GPU sync at
every copy.

### 10.3 P3's check: FIFO-log replay

```
nsmbw-port --replay log.dff [--frame n] [--scale 1] [--screenshot out.png] --compare dolphin.png [--min-psnr 30]
```

- **Format:** `gx::dff` reads Dolphin's FIFO-log format (versions 1–6), laid
  out as in Dolphin's `FifoDataFile.cpp`.
- **Replay:** the starting state is loaded the way `FifoPlayer::LoadRegisters`
  and `ClearEfb` do it. Each frame's RAM updates are applied at their FIFO
  positions, and EFB copies are written back to RAM.
- **Result:** the chosen frame's XFB is captured, and the exit code fails
  below the PSNR threshold or on validation errors.
- **Tested** with a synthetic log written by `gx::dff::write`.

**Still to do for the formal criterion:**
1. Record logs and frame dumps in Dolphin (Graphics > Advanced > Dump
   Frames; Tools > FIFO Player > Record).
2. Install the Vulkan SDK for the validation layer.

The scenes worth recording (title, 1-1, water, lava, world map) need P4's
input to reach in this port, but not in Dolphin.

### 10.4 Frame rate and display settings

These follow the design of WitherBlitz's NSMB-Wii-Recompilation (aurora's
`frame_interpolation.cpp`, its display settings), rewritten for this
interpreter-based port.

- **Game thread:** the game runs on its own thread, paced to 59.94 Hz, and
  hands each field's GX output to the render thread. Renderer results (RAM
  writebacks, EFB snapshots) go back over a channel.
- **Frame rates:** 60 (native), 90, 120, 144, 165, 180, 240, 300, 360, or
  unlocked (`--fps`, or F5/F6).
  - Above 60, frames are drawn one field behind real time, between the two
    newest fields.
  - Draws are paired by identity and nearest screen position. Large jumps
    don't pair.
  - Position matrices are interpolated rigidly: quaternion rotation, linear
    scale and translation, with a linear fallback for shear. Projection and
    viewport are blended.
  - The game still advances 59.94 times a second.
- **Measured on this PC** (RTX 5070 Ti):
  - 90–240 fps hold their target.
  - 300 and 360 reach about 270, because each present costs about 2 ms in
    the driver's present path.
  - Unlocked runs at about 570.
- **Settings and hotkeys:**
  - V-Sync (F7; off uses mailbox).
  - Borderless fullscreen (F11).
  - Resolution 1–8× or `native` (next start).
  - Aspect 16:9 or 4:3 (F9): the picture changes at once; the game reads
    `SYSCONF` `IPL.AR` at its next start.
  - Performance monitor (F3): FPS, game FPS, CPU and render milliseconds,
    and a frame-time graph.
  - Battery saver (F4): draws every other field.
  - F1 lists the keys. Changes are saved to `settings.toml`.
- **Process:** on Windows, the process runs at above-normal priority with
  power throttling off. Fields slower than 20 ms are logged. The Vulkan
  pipeline cache is kept in `cache/pipelines.bin`.

### 10.5 Features from NSMB-Wii-Recompilation, mapped to this plan

That port is a static recompilation built on WiiCompiled with aurora/Dawn
(C/C++). Its code cannot be dropped into this Rust port, so its features are
rebuilt here following its design. Windows first.

| Feature there | Here |
|---|---|
| Frame interpolation, V-Sync, fullscreen, battery saver, performance monitor | Done (10.4). The monitor has no size slider yet. |
| Aspect 4:3 / 16:9 | Done. Applies at the next start, as there. |
| Fill, 16:10, ultrawide (Hor+, via the widened `EGG::Screen` canvas) | P8: a game-side hook like their `dynamic_aspect.cpp`. Needs P4 to verify in levels. |
| Resolution 0.5×–4× and Native | Integer 1–8× and native done; 0.5× not. |
| Graphics on a second core, above-normal priority, drop reports, pipeline cache | Done. |
| Direct3D 12/11 renderers, Android, Linux builds | Android started (16); the others not planned now (Windows, Vulkan 1.3). |
| Keyboard, mouse, controllers, rebinding, combinations, Space Smart Spin, mouse as gyro, key prompts, rumble, real Wiimotes, Bluetooth passthrough | P4 (`crates/input`, emulated Bluetooth Wiimotes); rebinding UI in P8. |
| Clicking the game's menus and map | P8 (needs input plus game hooks). |
| Gameplay options (character pick, autosave, exit course, no time limit, infinite lives, power-up) | P8, as game hooks once P4 makes them testable. |
| Settings window, F10 bar | For now hotkeys and `settings.toml`; an in-game window in P8. |
| Spinning coins, depth-blur toggle | P8 (needs the game's draws identified in levels). |
| LAN and Tailscale lockstep multiplayer | New milestone after P7 (P9). The deterministic core already fits input-only lockstep. |
| HD texture pack | P8: replacement textures keyed by `gx`'s texture hash. |
| Languages, Newer Super Mario Bros. Wii, disc image picker, update button | Languages with P4 (`SYSCONF` `IPL.LNG`); the rest after P7. |

### 10.6 Tests and checks

```
cargo test -p guest-mem -p guest-cpu -p hle-ios -p hw -p hle-os -p gx -p vk-render -p frontend -p input
cargo test --release -p guest-cpu --test speed -- --ignored --nocapture   # interpreter throughput
nsmbw-port --frames 400 --unpaced --screenshot shot.png                   # identical to docs/img/p3_strap.png with aspect = "4:3" (see 11.2)
nsmbw-port --frames 300 --fps 144                                         # interpolated
```

`scripts/check.ps1` and `--check` print OK.

## 11. P4 status (2026-11-17): done

Added `crates/input` and the emulated Wii Remotes. **P4's criterion is
met:** `scripts/1-1.press` plays from boot through the title, file select,
the opening, the World 1 map and World 1-1 to the flagpole and back to the
map with 1-1 cleared, all through the emulated Bluetooth Wii Remote
(11.10). With GX on its own threads (11.8), every scene measured runs at
full speed and 240 fps. Still open: gamepads (`gilrs`) are untested on
real hardware; the keyboard and the scripted remote are tested.

### 11.1 Emulated Bluetooth Wii Remotes

- **Controller** (`hle-ios/src/bt.rs`): HCI commands and events, ACL data,
  and L2CAP signalling for up to four remotes.
  - A remote pages the Wii once page scan is on. The Wii accepts, reads the
    remote's name, features, version and clock offset, and authenticates.
  - The remote then opens the HID control (PSM 0x11) and interrupt
    (PSM 0x13) channels, and both sides configure them.
  - Every ACL packet from the Wii is answered with Number Of Completed
    Packets.
  - If the Wii disconnects a remote, it stays off until a button is pressed.
- **Remote** (`hle-ios/src/wiimote.rs`): the HID report protocol.
  - Output reports: LEDs, reporting mode, status, memory and register reads
    and writes, IR, and speaker.
  - Input reports: status, read data, acknowledgements, and every data
    mode.
  - EEPROM: Dolphin's IR and accelerometer calibration.
  - Accelerometer: the remote is held sideways and flat. Tilt rolls gravity
    towards Y; shaking swings all three axes at 6 Hz.
  - Not attached or not modelled: extension controllers and MotionPlus
    (reads fail as "unavailable"), IR points (reported empty), speaker
    output.
  - Reports run at 200 per second in continuous mode, as Dolphin's do.
- **Pairing:** at every start the remotes are written to `SYSCONF`
  `BT.DINF` (Dolphin's addresses). Read Stored Link Key returns a key per
  remote.

### 11.2 Fixes found on the way (they affected every scene)

1. **`SYSCONF` was rejected.** The generator wrote a null byte after each
   array item. The SDK's parser (reconstructed at `0x801DBEB0`) checks that
   each item ends where the next begins, so it threw the whole file away
   and used defaults.
   - Effects: aspect ratio, language and the paired-remote list were never
     read. That is why section 9 found the strap screen identical at 4:3
     and 16:9.
   - Fixed, plus `nand::sysconf_valid`; a stale file is regenerated.
   - With `aspect = "4:3"` the strap screen is still pixel-identical to
     `docs/img/p3_strap.png`. At 16:9 (the default) the game now draws its
     anamorphic widescreen layout.
2. **No FP-unavailable exception.** The OS saves FPU registers lazily: it
   clears MSR[FP] on a thread switch and relies on exception 0x800 to swap
   them. Without it, threads saw each other's FPRs.
   - Seen on the title screen: an angle loop got `f0` = 0.9 instead of
     65536.0 and spun forever, freezing the game.
   - Now an FP instruction with MSR[FP] clear traps (interpreter and block
     cache), with a test.
3. **Save files:** the title's data directory is created at start, and
   ReadDirectory on a file returns "invalid" (-101), which the SDK takes to
   mean "exists". The game now creates `wiimj2d.sav` and `banner.bin`, which
   closes P2's open item 1.
4. **AX voices progress:** each 3 ms frame, the command list's `0x0004`
   command gives the voice list.
   - Every running voice moves through its samples at its rate, wraps at
     its loop point, or stops at its end; the position and running flag go
     back to its PB.
   - Streams now refill at the real rate.
   - Still no mixing or output (P5).

### 11.3 Game functions answered on the host

`hle-os` answers some functions on the host:

| Address | What | Why |
|---|---|---|
| `0x801A3450`, `0x801A3880`, `0x801A48F0`, `0x801A5C10`, `0x801A6580`, `0x801A6D30`, `0x801A7A80`, `0x801A88B0`, `0x801A93F0` | AX aux effect callbacks return at once, only with `NSMBW_AUXFX=off` | Since P5 they run (reverb natively, `0x8026C210` below) and the DSP mixes their output back; turning them off also leaves the aux returns out. |
| `0x8026C210` | `nw4r::snd::FxReverbHi::UpdateBuffer`, computed natively (13) | Interpreted, the effects added 35% to the guest instructions of the run to 1-1; with the reverb native, 3% (most of it the delay, still interpreted). Ported from the recomp's `axfx_reverb_native.cpp`. |
| `0x801AC440/70/A0` | DC range loops return at once | Cache operations do nothing in this emulator. |
| `0x802E8620` | fdlibm `sqrt`, computed natively | Correctly rounded either way; skipped while the FPU is off. |
| `0x802CD4B0` | `EGG::LightTextureManager::drawAndCaptureTexture` skipped when nothing it reads changed (11.9) | A speed-up, not a workaround: kept after P5. `NSMBW_LIGHTTEX=off` turns it off. |
| `0x802E1AB4` | MSL `strcmp`, computed natively | Same result (difference of the first unequal bytes); 2% of guest time, mostly scene loads comparing effect names. Kept after P5. |

### 11.4 Controls (defaults from WitherBlitz's NSMB-Wii-Recompilation)

| Wii Remote (sideways) | Keyboard | Xbox-layout gamepad |
|---|---|---|
| D-pad | arrows | D-pad, left stick |
| 2 (jump) | X, Space | A |
| 1 (run, fire) | Z, Left Shift | B, X |
| Shake (spin, pick up) | C, Ctrl | Y, LB, RB |
| Tilt | Q / E | LT / RT, right stick |
| A | B | — |
| + / − / HOME | Enter or Esc / Tab or − / Home | Start / Back / Guide |

- The keyboard is player 1. Gamepads take players 1–4 in the order they
  connect, and a player's remote switches on with their first device.
- Not yet: Space Smart Spin, rebinding, rumble to gamepads, mouse as gyro.

Scripted input for tests: `--press <frame>:<buttons>[:<frames>]`, plus
`--shot <frame>:<png>` and `--trace-ios`. `--profile-guest` prints the
hottest guest functions. `--log-fps` prints, once a second, the frames
shown, the game's speed, and the emulation and render times.

```
nsmbw-port --frames 7200 --unpaced --press 600:a --press 1550:2 --press 2600:2 --press 3000:2 --press 3400:2 --press 5150:down:900 --press 6100:2 --press 6300:2 --press 6700:2
```

With an existing save this runs, in order:
1. the strap screen and "Hold the Wii Remote sideways";
2. the title screen;
3. file select and Select Players (1);
4. the opening cutscene (Peach's birthday, the airship);
5. the castle: Mario walks out (the remote's D-pad down is screen right);
6. the World 1 map;
7. World 1-1, where the script's jumps run Mario into the first Goomba.

On a fresh save, "save data has been created" adds an OK prompt after
step 1, which shifts the later frame numbers.

### 11.5 Speed (unpaced, host time per field; 16.7 ms is full speed)

| Scene | Start of P4 | After the first pass | Now |
|---|---|---|---|
| Strap screen | 2–3 ms | 2–3 ms | 2–3 ms |
| Title screen | frozen (FP bug) | 11–13 ms | 11–13 ms (full speed) |
| Opening cutscene (11k draws, 77k vertices per field) | 60–76 ms | 47 ms | 27 ms |
| World map | — | 30–35 ms | 28–29 ms |
| **World 1-1** | — | — | **14 ms (full speed)** |

What changed, roughly in order of effect:

1. **Indexed geometry.** `gx` keeps each draw's vertices once and emits
   triangle-list indices (`Output::indices`); `vk-render` draws indexed,
   with the indices after the vertices in the frame's buffer. `Draw` ranges
   are index ranges; `DrawRecord` ranges stay vertex ranges.
2. **No per-vertex allocation.** The vertex loader borrows attribute bytes
   instead of allocating per attribute; triangle vertices are transformed
   straight into the output.
3. **Recycled output buffers.** The renderer hands each field's `Output`
   back (`ToEmu::Recycle`), so its vectors keep their capacity.
4. **Frame-interpolation records only when interpolating** (`ToEmu::Interpolate`;
   switched between fields so `model` stays aligned with `vertices`).
5. **Per-draw XF setup** (lights, channel colours, post matrices) decoded
   once and cached by an XF generation that position and texture matrix
   loads do not bump.
6. **Interpreter:**
   - Blocks are checked by per-page write generations kept by guest memory
     (stores and `ram_mut` bump pages that hold decoded code), not by
     re-reading their instructions; a test covers stores and DMA.
   - A 256-bit filter skips the special-address checks for almost every
     block.
7. **EFB copies to RAM are off by default** (`efb_copies_to_ram` in
   `settings.toml`; Dolphin's default is the same). They cost a readback,
   an encode and a hash per copy, about 15 per field.

**A bug found on the way:** written back late, a copy could land on memory
the game had already reused, corrupting it. Some runs faulted at
`0x802E1AB8`; with writebacks off, runs were identical. Each copy now
carries a hash of its destination at copy time (`CopyTex::ram_hash`), and
a writeback is applied only if that memory is unchanged. In one run 64,795
were applied and 3,423 skipped.

### 11.6 Game speed capped at 100%, and 240 fps (second speed pass)

Measured in real time (paced) with `--fps 240 --log-fps` at EFB 2x in a
1280x720 window:

| Scene | Before | Now |
|---|---|---|
| World 1-1 | game 100%, shown ~110 fps | game 100%, shown 240 fps |
| World map, opening cutscene | game ~50%, shown ~70 fps | game ~50%, shown 240 fps |
| After a slow stretch | game up to 106% while catching up | never above 100% |

1. **Pacing:** a late field no longer makes the game thread run the next
   fields back to back to catch up (it did for up to six fields, so the
   game briefly ran at 106%). Lateness over half a field restarts the
   schedule from now. No field starts before its slot, so the game cannot
   run faster than 59.94 fields a second.
   - Later (section 14): a field up to two fields late (33 ms, a scene
     load's slow field) keeps the schedule, and the next fields skip their
     sleep until it is met again; later than that restarts from now. This
     is not the old catch-up: it never makes up more than two fields, and
     still no field starts before its slot, so over anything longer than a
     couple of fields the game never runs faster than real time. A field's
     frame keeps its scheduled time (`GameFrame::time`), so interpolated
     frames stay evenly placed during the catch-up. The per-second game
     speed reading can show up to ~103% just after such a field.
2. **GPU time per frame** (world map, timestamp queries): 3.65 ms became
   1.2 ms, with identical pixels (world map frame 5700, and the 4:3 strap
   screen).
   - The ubershader indexed local arrays by run-time values (texture
     coordinates and their derivatives, TEV registers, the konst table).
     GPUs keep such arrays in slow memory. They are now plain values chosen
     by `switch`; this was most of the gain.
   - Draws whose depth is the rasterised one (no z-freeze, no Z texture)
     use a fragment entry point without `frag_depth`
     (`fs_early`, pipeline key bit `EARLY_DEPTH`), so the GPU can test
     depth before shading.
3. **Render thread per displayed frame:**
   - Pairing draws for interpolation (`gx::interp::pair`) took 11 ms per
     field pair. Each record's screen position is now computed once, and a
     group whose draws come in the same order and barely move is paired in
     order: about 1 ms.
   - Interpolated vertices are written straight into the frame's mapped
     vertex buffer (`Renderer::render_with`, `interp::interpolate_into`),
     saving a copy of every vertex (11 MB per frame on the world map).
   - Retired textures and buffers are freed when the frame that last used
     them completes, instead of waiting for the whole GPU each time one is
     retired (1.4 ms per frame in 1-1).
   - Immediate present mode was tried and changed nothing: the present
     waited for the GPU, which the shader work fixed.

**Then still slow (fixed in 11.8): game speed on the world map and in
cutscenes (~50%).** GX decoding took about 15 ms of each 30 ms field on
the game thread:
vertex loading 5.4 ms, transform and lighting 6.4 ms, interpolation
records 2.1 ms, shading state 1.2 ms. The interpreter takes the rest.
Both of the field's PE finishes fall at its edges (one at the start, one
after all GX work), so a GX thread would overlap almost the whole field.
Next steps were, in order: a GX thread (done, 11.8); the vertex transform
in the vertex shader (plan 2.5, step 4); a display-list cache with write
generations from DC flushes and DMA; the recompiler (P6).

### 11.7 What carries over from NSMB-Wii-Recompilation's optimisations

That port is the same build (SMNE01 rev 1), and its `projects/nsmbw/MAP.txt`
names every function: the hooks above were found with it. Reviewed for this
port:

| There | Here |
|---|---|
| Vertex decode, transform and lighting on the GPU (aurora's generated shaders) | To do: 11.6's second step |
| Interpolation once per game frame; displayed frames only re-blend matrices | Partly done (pairing once per field pair, written straight to the GPU); the rest comes with the GPU transform |
| GX on its own thread (16 KB chunks, waits only where needed) | Done, as two threads (11.8) |
| `EGG::LightTextureManager::drawAndCaptureTexture` (0x802CD4B0) skipped when its inputs are unchanged | Done (11.9) |
| Write generations from DC flushes and DMA; texture hashes and a display-list cache keyed on them | To do, with the GX thread |
| Battery saver that skips the game's own drawing (`dGraph_c::painter`) every other frame | To do (P8); ours skips only the host render |
| Adaptive interpolation (fewer inserted frames while the game is late) | Not needed now that 240 holds |
| Native `strcmp`; `breffSearchName`/`breftSearchName` | `strcmp` done; the name searches later (load spikes only) |
| Merged gather-pipe stores, bigger GX chunks | Later (with the GX thread) |
| Native `nw4r::lyt` quad drawing, ~150 native GX functions | After P6 |
| Translator optimisations (register residency, leaf inlining, GQR hoisting) | P6 started (14); these are still to do |
| Draw merging, pipeline cache, bind-group cache, above-normal priority, async readback | Already here, or not needed |
| Native AX reverb and mixing | P5 (the callbacks are stubbed until then) |
| Exact-product FMA shortcut | Not needed: the host FMA instruction is used |

### 11.8 GX on its own threads

GX decoding now runs beside the interpreter, in two stages:

1. **Scanner, on the game thread** (`gx::thread::Scanner`): walks the FIFO
   stream for command lengths and raises the PE events (finish, tokens,
   with the BP write mask) at the same point inline decoding did. It also
   rewrites the commands that read RAM by address: a display list's
   commands are copied inline, and indexed XF loads become plain XF writes
   with the words as they are now. About 1 ms per field on the world map.
2. **Decode thread** (`gx` thread): the `Gpu` as before (register state,
   vertex loading, textures, TMEM, EFB copies). It reads vertex arrays and
   textures from guest RAM through `guest_mem::RamView`, a read-only view
   that shares RAM's allocation, as the real GPU reads RAM.
3. **Transform thread** (`gx-transform`, `gx/src/back.rs`): transform,
   lighting, the z-freeze plane, interpolation records and building the
   field's `Output`. It keeps its own XF state, fed every XF write in
   stream order, so it never reads RAM. The decode stage sends it ops in
   batches of 1,024.

**Synchronisation:** the game thread waits for the decode thread whenever
the scanner raised a PE event (twice per field on the world map), and for
both threads when it takes a field's output. At a finish the GPU has read
everything sent before it, as on hardware, so the game may reuse those
buffers. Emulated time does not move while waiting, so runs stay
deterministic.

**Checks:**
- Unpaced to 1-1, inline (`--gx-inline`) and threaded give the same guest
  instruction count, and screenshots at frames 1000, 2500, 3500, 4500,
  5700, 6500 and 6800 are byte-identical to each other and to the build
  before the split.
- `gx::thread::tests::threaded_matches_inline` runs a stream with a display
  list, an indexed XF load and masked tokens both ways.
- The 4:3 strap screen still matches `docs/img/p3_strap.png`.

**Interpolating, a new field is only prepared:** its textures are uploaded
and its copy targets made, but it is not drawn on its own, since the
in-between frames draw it anyway (unless its EFB is read back: copies to
RAM, CPU peeks, the last frame). `--shot` then saves the next in-between
frame, which is what the window shows.

| Scene (paced, `--fps 240`) | Before 11.6 | After 11.6 | Now |
|---|---|---|---|
| World map, opening cutscene | 50%, 70 fps | 50%, 240 fps | **100%, 240 fps** (game thread 14–18 ms) |
| World 1-1 | 100%, 110 fps | 100%, 240 fps | 100%, 240 fps (game thread ~10 ms) |

Unpaced to 1-1, the game thread's total time fell from 124.7 s (inline) to
80.3 s. Short dips remain at scene loads (title to file select, entering
the map or a level), where the guest decompresses and builds the scene.

**What carries over from the recomp next:** a display-list cache and the
GPU vertex transform (11.7); the light-texture skip is done (11.9).

### 11.9 Light textures redrawn only when they change

Every frame, courses and the world map redraw their light textures (the
normal-sphere maps their models are lit with) into a corner of the EFB and
copy each one out, around a save and restore of the EFB. While the lights
hold still that is the same picture each time.

`hle_os::light_texture` (after the recomp's `light_texture_cache.cpp`)
hooks `EGG::LightTextureManager::drawAndCaptureTexture` (0x802CD4B0). It
hashes what the call reads:
- its float arguments;
- the manager, its screen and its light textures;
- their light set, light objects and ambient colours;
- the EGG statics 0x8042AEC0..0x8042AF58.

Objects are hashed whole as heap blocks (the expanded heap's `UD` header
gives the size). When the key matches the manager's last call, the hook
only sets the manager's drawn-this-frame bit and returns. The last call's
EFB copies are still the textures at those addresses, because the GX
texture cache keeps a copy until its address is copied to again or its RAM
changes. The hook keeps the call's own early-outs (light textures off,
already drawn this frame, none to draw), which then run the game's code.

**Checks:**
- Unpaced to 1-1, 4,905 of 5,549 calls were skipped. Guest instructions
  fell from 8.72 to 8.25 billion, and screenshots at frames 1000, 2500,
  3500, 4500, 5700, 6500 and 6800 are byte-identical with the hook on and
  off.
- With it off, the instruction count is the same as before the hook.
- `light_texture::tests::repeats_are_skipped_until_a_light_changes`.
- When the window closes, the frontend prints how many redraws were
  skipped.

**Effect** (paced, `--fps 240`, game-thread time per field):

| Scene | Off | On |
|---|---|---|
| World map | 16.0–16.1 ms; seconds at 92–97% | 14.6–15.3 ms; never below 98% |
| World 1-1 | 11.1 ms | 10.1 ms |

The only dips left are while a scene loads.

### 11.10 World 1-1 to the flagpole (P4's check)

```
nsmbw-port --unpaced --press-file scripts/1-1.press --frames 9700
```

- `--press-file` reads `--press` entries from a file (spaces or lines,
  `#` comments). `scripts/1-1.press` explains each part of the run.
- Run: hold right and run (`down+1` on the sideways remote) from frame
  6300 and hop for 20 frames every 40, with changes where the rhythm
  failed:
  - an extra hop before the first Koopa;
  - no hops under the brick row with a Goomba on it;
  - a 4-frame tap onto the hilltop step under a ? block (a full jump
    hits the block and drops Mario back);
  - hops timed for the first pit and for the two groups of three Goombas.
- Mario reaches the flagpole at about frame 8880 with all five lives, the
  course is cleared at 9150, and by 9700 he is back on the World 1 map
  with 1-1 cleared and the path to 1-2 open. Screenshots:
  `docs/img/p4/1-1_start.png`, `1-1_flagpole.png`,
  `1-1_course_clear.png`, `1-1_back_on_map.png`.
- Unpaced runs are deterministic, so the file replays exactly (the replay
  matched the attempt's positions at the flagpole). Paced runs are not:
  renderer feedback (EFB snapshots) reaches the game at host-dependent
  times.
- `--watch-player` prints player 1's position every 10 frames. It finds
  the player through `daPyMng_c`'s IDs (0x80354E10) and
  `fManager_c::searchBaseByID`'s lists (0x80377A70), and reads the actor's
  position at +0xAC. The run was scripted from these numbers and the
  screenshots before each death.


## 12. P9: online multiplayer with drop-in and a camera per player (plan, 2026-11-18)

Replaces the restart-based lockstep netplay of NSMB-Wii-Recompilation
(`runtime/src/netplay`, read for ideas only).

### 12.1 Decisions (WitherBlitz, 2026-11-18)

| Question | Decision |
|---|---|
| Where players are | **Online first**: two PCs over LAN or Tailscale. Each PC shows its own player's view. Local split-screen later, on the same camera hooks. |
| When a player can join | **Any time**, mid-level too. The host sends its whole emulator state; nobody restarts. |
| The game's shared camera | **Active around everyone.** The game's own camera becomes a box around all players, so enemies, coins and blocks near any player run. No bubbling or pushing for being off-screen. Bubbles from A, deaths and pits stay. Each PC draws a normal-size view centred on its player. |
| Auto-scroll, boss rooms, zone changes | **The game's rules there.** In auto-scrolling zones and boss rooms both views follow the game's camera, with its bubbles and pushes. Pipes and doors to another zone move everyone, as now (one zone is loaded at a time). |

### 12.2 How it works

1. **Lockstep on inputs.** Every PC runs the whole game. Each field, every
   PC sends its local remote's input for field *n + delay*; field *n* runs
   once all inputs for it are in. The guest never sees anything else from
   the network, so the games stay identical.
2. **No host-dependent feedback in a net session.** Paced runs are not
   deterministic only because the renderer feeds the game: EFB copies
   written back to RAM, and EFB snapshots for CPU peeks. In a net session
   writebacks are off and peeks are answered without the renderer. Each PC
   also draws a different view, so this is needed anyway.
3. **Join by save state.** A joiner connects; at the next field boundary
   the host serialises its state (CPU, RAM, hardware, IOS with the
   Bluetooth stack, HLE OS, GX register state, NAND files) and sends it
   compressed. Both continue in lockstep from that field, and the joiner's
   remote switches on. The joiner writes saves to a session folder, never
   over its own.
4. **Desync check.** Every 120 fields each PC sends a hash of MEM1 and
   MEM2; a mismatch stops the session with the field number.
5. **Camera (12.1).** The game's camera (the scroll and zoom state in
   `dBg_c` and the screen in `dBgParameter_c`) is replaced, outside
   auto-scroll and boss zones, by a box around all players, so actor
   activation, sprite spawning and the off-screen checks cover everyone.
   Each player's view is drawn by running the draw pass once per player
   with the screen moved onto that player. Every PC runs every view, so
   the guest stays identical; each PC renders only its own player's. The
   frame (`dSys_c::execute`, 0x800E4E40) draws with `dGraph_c::painter`
   (0x800B64E0) and then runs `fManager_c::mainLoop` (0x80162E10), whose
   last pass is the actors' draw walk.

### 12.3 Phases (each ends with a check)

| # | Phase | Done when |
|---|---|---|
| M1 | Scripted remotes 2–4 (`--press p2:…`) | A two-player script joins player 2 on the World 1 map without restarting, and both play 1-1 on one PC with the game's shared camera |
| M2 | Net-safe determinism | With feedback off, a paced run gives the same RAM hashes per field as an unpaced one |
| M3 | Save states | Save at field *f*, load in a new process, run to *g*: RAM hash and screenshot match a run that never stopped |
| M4 | Lockstep netplay and joining | Host and joiner on 127.0.0.1: the joiner connects on the map and mid-level, and the hashes agree every 120 fields to the end of the run |
| M5 | Mid-level join | The joiner's player appears in the running level |
| M6 | Camera per player | Each PC's view is centred on its player; enemies and coins near either player work; two players finish 1-1 online |

The solo path must not change: `scripts/1-1.press` still reaches the
flagpole and the 4:3 strap screen still matches `docs/img/p3_strap.png`.

### 12.4 Status

**M1 (scripted remotes): done.** `--press p2:<frame>:<buttons>` drives
remote 2 (p1–p4); a remote switches on at its first scripted press
(`p2:5100:` with no buttons) and stays on. `--watch-player` prints every
player found in `daPyMng_c`'s ID list (0x80354E10, one word per player).
- A connected remote does not add a player by itself. The game adds one
  through the World Map menu (+, then Add/Drop Players): the new remote
  gets a slot with a character picked, and pressing 2 on it brings the
  player in. Checked from a saved map state: `5200:plus 5260:left 5300:2`,
  then `p2:5420:2` brought Luigi onto the map.

**M3 (save states): done.**
- `guest_mem::snap` is one walk over each type's fields that either writes
  them or reads them back in place (`Snapshot`), so saving and loading
  cannot disagree. Implemented for the CPU, RAM, hardware (PI, VI, CP/PE,
  IPC, DSP/AI, EXI, SI), IOS (open files relative to the NAND root, the
  Bluetooth stack, the Wii Remotes), the HLE OS (linked modules, the
  light-texture keys) and GX (the scanner, and the decoder's BP, CP, XF,
  TEV and TMEM, resent to the transform thread on load).
- Left out on purpose: decoded blocks (loading counts as a write to every
  code page), statistics, the renderer's EFB, host textures. Saving forgets
  the light-texture keys on the saving machine too, so both redraw them.
- The RTC now runs on emulated time from its power-on value (`hw::exi`),
  so it is the same on every machine in a session.
- `--save-state <f>:<file>`, `--load-state <file>`, `--hash-every <n>`
  (prints a 64-bit hash of RAM), `--dump-ram <f>:<file>`. Screenshot
  frames (`--shot`) are now field numbers, so they carry on after a load.
- **Check:** saved at field 6400 in 1-1 (88 MB uncompressed), loaded in a
  new process: RAM hashes match the run that never stopped at every 100
  fields to 7000, and screenshots at 6500 and 6990 are byte-identical.

**M4 (lockstep and joining): done for two machines.** `crates/frontend/src/net.rs`.
- `--host <port>` plays remote 1 and listens; `--join <host>:<port>`
  joins at any time. At its next field the host saves its machine and
  sends it with its NAND files (deflated); the joiner writes the files to
  `saves/net-join/nand`, boots, loads, and plays the next free remote from
  two fields later. Input delay: 2 fields once someone has joined, none
  for a solo host.
- Every 120 fields each machine sends a RAM hash; a mismatch stops with
  the field number. A joiner leaving switches its remote off at a field
  every machine agrees on; the host leaving ends the session.
- In a session nothing from the renderer reaches the game (`--no-feedback`,
  implied): EFB writebacks are dropped and peeks read zero. A full 1-1 run
  makes no EFB peeks, and writebacks are off by default, so the solo game
  does not change.
- **Checks** (two processes on 127.0.0.1, unpaced):
  - joined during boot (field 304, 15.8 MB, 0.5 s): hashes agree to 1984;
  - joined on the World 1 map (`--wait-join 5100`, 22.3 MB, 0.6 s to
    build): both went into 1-1 together; hashes agree at every 300 fields
    to 6600, and screenshots at 5400 (map) and 6790 (1-1) are
    byte-identical between host and joiner;
  - the host's hashes before the join equal a plain solo run's.

**M6 (camera per player): started; paused 2026-11-18.**
- Approach chosen (keeps the game identical on every PC): the game's own
  camera is to become a box around every player (activation, spawning and
  the off-screen checks then cover everyone), and each PC re-projects the
  world draws onto its own player at normal size. Only the GX transform
  changes per PC, never guest state.
- **Done, the per-PC view** (`gx::xf::View`, `GpuPort::set_view`,
  `--view-player <n>`, `--view-zoom <z>`; a net session uses its own
  remote). Each field the game thread reads the game camera and the
  player, and the transform half swaps the world projection for a window
  centred on the player and clamped inside the camera. Checked: in 1-1 with
  `--view-zoom 2` the world is drawn 2x around Mario while the HUD stays.
  Known limit: background layers move with the world (no parallax fix-up)
  when a view is away from the camera's centre.
- Findings (1-1, field 6500):
  - `dBg_c::m_bg_p` = *(r13 − 0x58B0) (r13 = 0x8042F680). Camera top-left
    at +0x8FE88/+0x8FE8C, size at +0x8FE78/+0x8FE7C (471.8 × 258.6, y up),
    zoom at +0x9000C (0.866; size = base 408.7 × 224 ÷ zoom). Zone size
    used to cap the zoom: +0x8FEA0/+0x8FEA4 (6595.6 × 385 in 1-1).
  - `dBgParameter_c::ms_Instance_p` = *(r13 − 0x5880): camera left/top at
    +0x38/+0x3C, size +0x40/+0x44, centre +0x48/+0x4C.
  - The world projection is orthographic and centred: `2/width`,
    `2/height` of the camera, depth −5.00025e-5, −1.0000501. Other passes:
    light textures (2/32), the HUD (608 × 456), a top strip (640 × 72), and
    2 draws at 832 × 456 that would collide with the world's when the
    camera is 832 × 456.
  - Each frame `dBg_c::calcScroll` (0x8007A6C0) is called from 0x80155988
    with the look-at point (r4 = 0x8043F9EC) and writes the camera left at
    0x8007A94C. The copy to `dBgParameter_c` +0x38/+0x3C is at
    0x801566F0/0x80156700 (called from 0x80156654). Both are in main.dol,
    so they can be hooked like the others in `hle-os`.
  - The multiplayer zoom (0x8007AC40) steps between three zoom levels
    (min/mid/max tables, spread and shrink lines) and is capped so the
    camera never exceeds the zone's height. A box around far-apart players
    therefore cannot come from the zoom; the plan is to set the camera
    rectangle (width and height separately) after the copy at 0x801566F0,
    since each PC re-projects anyway.

**Next steps when this resumes:**
1. M6: hook after the camera update: outside auto-scroll and boss zones,
   widen the camera rectangle (dBg_c and dBgParameter_c) to the players'
   bounding box plus half a normal screen, and keep the view at the
   players-together size. Then check bubbles and pushes stop, and that
   enemies near the far player run.
2. M5: mid-level join. `daPyMng_c::create` (0x8005EEE0) and
   `isCreateBalloon` (0x80061110) look like the way to bring the joiner in
   inside a bubble, at a field both machines agree on.
3. M2's paced check (paced `--no-feedback` hashes equal unpaced), and
   two players through 1-1 to the flagpole online.
4. Not yet re-run after these changes: `scripts/1-1.press` to the flagpole,
   the 4:3 strap screenshot, `--check`, `scripts/check.ps1`. The changes
   leave the solo path's guest code alone (the M3 and M4 runs reproduced
   solo hashes), apart from the RTC now running on emulated time.

## 13. P5 status (2026-11-18)

Developed in a copy while the multiplayer session edited this folder, then
merged back (three-way, with `git merge-file`).

**Done:**
- **AX microcode on the host** (`crates/hw/src/ax.rs`, the newer AXWii
  command layout NSMBW uses, found by dumping its lists):
  - setup;
  - adding the last frame's surround into L/R;
  - processing the voice list;
  - aux A/B/C exchange with the effect callbacks;
  - compressor (ignored);
  - output (surround as `s32`, the stereo frame as right then left,
    big-endian);
  - Wii Remote speakers (silent).
- **Voices:**
  - ADPCM, PCM16 and PCM8 through an accelerator model;
  - at its end address the accelerator always moves to the loop address
    (with the loop predictor; streams keep their history), and a voice that
    does not loop then stops;
  - 4-point cubic SRC (`src_type` 0), linear (1) or none (2);
  - volume envelope, one-pole low-pass, Q14 biquad;
  - the twelve bus channels the mixer-control word selects (bits 0-4 main,
    16-20 aux A, 21-25 aux B, 26-30 aux C: L, R, L/R ramp, S, S ramp), with
    per-sample volume ramps, written back to the PB with the de-pop values.
- **Effects:**
  - the game's aux callbacks run again, and their output is mixed back;
  - `FxReverbHi::UpdateBuffer` runs natively (`crates/hle-os/src/reverb.rs`,
    ported from the recomp);
  - `NSMBW_AUXFX=off` skips both.
- **AI DMA:** each block plays from the address latched when it started;
  its samples go to `Dsp::audio` at 32 kHz.
- **Frontend** (`crates/frontend/src/audio.rs`, `cpal` 0.15):
  - the default output device, resampled linearly;
  - a 100 ms pre-buffer, at most 250 ms queued;
  - dropouts counted;
  - `--wav <file>` records what was played;
  - `--no-audio` turns output off;
  - unpaced runs are silent.
- **The "is sound playing?" hook is gone.** `SE_VOC_MA_PLAYER_DECIDE` is a
  sequence (`nw4r::snd::detail::SeqSound`) that waits for its note's voice.
  `AxVoice::IsPlayFinished` (0x80261E70) counts a voice finished when its
  current address, as the AX library copies it back to the CPU's `AXVPB`,
  lies in NW4R's zero buffer: the first 0x200 nibbles of `AxManager`'s
  zero buffer (`0x803FC810 + 4`). NW4R makes that buffer a one-shot's loop
  address. The accelerator used to leave a stopped voice at its end address,
  so the sequence never ended. Found by logging `isSndPlaying`'s results,
  naming the sound object's vtable, and comparing the DSP's and the CPU's
  PB copies in a `--dump-ram` image (`__AXPB` at `r13 - 0x5078` =
  0x8042A608; `AXVPB` array 0x80385E20, index at +0x18).

**Checks:**
- Unit tests:
  - a PCM16 voice reaches the left channel, then stops with its address at
    its loop address;
  - ADPCM header skipping.
- `scripts/1-1.press` reaches the flagpole and returns to the map with the
  hook removed, with effects on and with them off.
- The music's spectral peaks sit on exact notes (C4 261 Hz, G3 196, D3 147,
  C5 523), so decoding and pitch are right.
- With effects on, 1-1's sound differs by about 4% RMS from effects off;
  the title and map send nothing to the aux buses.
- Paced at 240 fps with sound and effects, the world map runs at 100.1%.
  Dropouts happen only at boot and scene loads, where the game itself runs
  below 100%.

| Run to 1-1's flagpole (unpaced) | Guest instructions | Host time |
|---|---|---|
| Effects off | 12.04 billion | 110 s |
| Effects interpreted | 16.19 billion | 134 s |
| Reverb native | 12.38 billion | 113 s |

**Still to do:**
1. P5's formal check: an A/B comparison against a Dolphin audio dump. Use
   `--wav` on our side; the recording has to come from Dolphin.
2. The delay effect (`AXFXDelayExpCallback`) is still interpreted: 2.5% of
   guest samples in the run to 1-1. Port it natively if a scene needs it.
3. Not modelled: initial time delay, the compressor, DPL2 output, Wii
   Remote speakers, and the polyphase coefficients (cubic interpolation
   instead).

## 14. P6 status (2026-11-18, first pass)

Ahead-of-time Rust, one function per guest function, as 2.2 planned (chosen
over a Cranelift JIT on 2026-11-18). The design is adapted to this port,
which runs the game's own OS: thread switches and interrupts happen at any
instruction through guest exceptions, and save states and online lockstep
need the whole machine in `Cpu` and `Mem` at every slice end.

**Pieces:**
- `crates/ppc-recomp` (the generator) reads three inputs:
  - a RAM image taken after the RELs are linked (`--dump-ram`);
  - the recomp project's `projects/nsmbw/MAP.txt` for function starts (the
    same build; the RELs load at its addresses here too);
  - a code profile (`--code-profile`, every address an interpreter block
    started at).

  It generates every function the profile touches into
  `crates/game-recomp/gen/`.
- `crates/game-recomp` holds the function table and the hook list. Its
  16 shard crates (`shards/s00`..`s15`) are contiguous address ranges that
  cargo builds in parallel. Build scripts write empty stubs when nothing is
  generated, so the port builds without them and interprets everything.
- `guest_cpu::recomp` is the runtime: `Rt` (entry table, code checks,
  budget), `call`, `call_index`, `call_indirect` and `tail`.
- `hle_os::System::run_fast` is the dispatcher.

**How generated code runs:**
- Every instruction calls the interpreter's own handler with a constant
  word:
  - `guest_cpu::fast` handlers are always inlined;
  - `interp::op_main` (the body of `execute`) is always inlined;
  - the other groups are `#[inline]`.

  The two engines therefore share one definition of each instruction.
  Branches, calls and returns are native code.
- A function can be entered at any of its block starts (a `match` on `pc`).
  Blocks are emitted as nested labeled blocks in groups of 128, so a jump
  forward is a plain `break`; loops go back through the `match`.
- A call is a Rust call, after which the caller continues only if `pc` is
  its return address. Anything else unwinds to the dispatcher, with nothing
  lost: guest state is all in `Cpu`/`Mem`. This covers thread switches,
  hooks, unknown jumps, an interrupt and the end of a field. Calls deeper
  than 400 unwind too.
- After every branch, generated code sets `pc` to the destination (and
  `lr` first, for linking forms) and checks the slice budget (`slice`). A
  spent slice ends right there through `Rt`'s slice-end callback
  (`hle_os::Slices::end_slice`: advance the hardware, deliver interrupts).
  The code carries on natively unless an interrupt was taken or the field
  is over.
- `mftb` and `mfspr`/`mtspr` of DEC or TB see the exact cycle count.
- Generated code stops before `sc` and `rfi`, which the interpreter runs
  (`mtmsr` too until the second pass, which runs it in generated code and
  ends its block there). With the FPU off, a block with FP instructions is
  left to the interpreter.
- Generated code never runs through a hook address (`hle_os::SPECIAL`). The
  generated list must match it, or the code is not used.
- **Code checks:** guest memory counts writes to code pages (`code_epoch`).
  A function checks one number on entry; when it changed, it compares its
  pages' write generations, and if those changed, its words. A function
  whose code changed is interpreted until the code matches again (REL
  loading, patches).

**The slice rule (both engines):** a slice ends at the first instruction
after a branch (`guest_cpu::is_branch`: `b`, `bc`, `bclr`, `bcctr`, taken or
not) once `SLICE` (500) cycles have passed, or when the scheduler goes idle.
- The interpreter's blocks end at branches, so `System::run_field` checks
  after each block that ended on one (`run_block` reports it).
- Generated code checks after each of its branches, so it never has to stop
  mid-block, and a slice end does not unwind it.
- `System::run_field(limit)` replaces `run_fast`: it runs until the field
  ends (a VI retrace) or `limit`. The stop is decided at a slice end in
  `Slices`, which the dispatcher and generated code share.
- `--profile-guest` samples the PC at each slice end there.
- `run_slice` (`--boot`'s per-instruction path) still runs exactly `n`
  instructions.

This replaced a first version in which slices were exactly `SLICE`
instructions: generated code then stopped before any block that did not
fit, and unwound to the dispatcher at every slice end. Timing changed with
the rule. 1-1 still reaches the flagpole at the same positions (x 6704.0 at
frame 8880, course clear at 9150), with the script unchanged.

**Lockstep check:** `--hash-every 100` over the whole 1-1 script (9700
frames). Recompiled (99.3% of instructions) and `--interpreter` runs give
identical RAM hashes at all 98 points and the same instruction count
(13,194,609,027). This is stronger than per-function lockstep: any
difference in any executed function would show up in memory. (With the
first slice rule, both also matched: 13,194,504,867 instructions.)

**Speed** (1-1 script, unpaced, no V-Sync; host time in `run_field` net of
waiting for the GX thread):

| Step | Recompiled | Interpreter |
|---|---|---|
| First build (one crate, `pc` dispatch per block) | 115.5 s total | 168 s total |
| Group handlers inlinable, 16 shards | 89.8 s total | |
| Thin LTO, step at slice edges | 87.6 s net | 138.7 s net |
| Nested labeled blocks | 77.9 s net | |
| Leaner RAM fast path, inlined FP conversions | 74.1 s net | |
| Slices end after branches, in place (no unwinding) | **65.7 s net** (83.0 s total) | 137.7 s net (139.2 s total) |

Recompiled code is now about 2.1× faster than the interpreter on the game
thread. Waiting for the GX thread grew to 17.3 s of the 83.0 s, so the GX
threads are becoming the limit in heavy scenes. Profile of the game thread
at 87.6 s, before slices ended in place:
- generated code 41.7 s;
- the dispatcher 8.4 s (table lookups that miss the cache);
- generic handlers still out of line 7.7 s (since inlined);
- AX mixing and reverb 2.6 s;
- the GX scanner 1.5 s.

**Build time:**

| Setup | Full build |
|---|---|
| One crate, fat LTO | 50 min |
| One crate, thin LTO | 46 min |
| 16 shards, fat LTO | 25 min (21 of them in the fat-LTO link) |
| 16 shards, thin LTO | 4 m 16 s |

The release profile now uses thin LTO.

**Regenerating:**

```
nsmbw-port --unpaced --press-file scripts/1-1.press --frames 9700 --code-profile profile.txt --dump-ram 9690:ram.bin
cargo run --release -p ppc-recomp -- --ram ram.bin --map MAP.txt --profile profile.txt
cargo build --release -p frontend
```

From Git Bash, set `MSYS_NO_PATHCONV=1` for `--dump-ram` with a drive
path. `--interpreter` turns generated code off. The exit report prints the
share of instructions run by generated code and the time spent waiting for
the GX thread. The generated code comes from the user's disc, so it is not
part of the source tree.

**Still to do:**
1. **Cheaper memory accesses:** a store is still about 20 host instructions
   (page table, code-page flags, reloads of `Mem`'s fields after each
   store).
2. **Coverage:** profile more of the game (other worlds, menus, bosses) so
   the 0.7% interpreted stays small, P7 included.
3. **P6's check in 4-player 1-1** (60 fps with headroom) needs a
   4-player script; single-player 1-1 was already full speed.
4. **Translator optimisations** from 11.7 (register residency, leaf
   inlining, GQR hoisting).
5. **The GX threads** (vertex loading, transform) now hold up the game
   thread for about a fifth of its time in the 1-1 script: the vertex
   transform on the GPU (11.6) is the next step there.

### Second pass: engine speed and the build-test loop (2026-11-19)

Work in its own worktree (branch `claude/engine-speed-aa0f93`), under the
user's rule of at most 4 of the 16 hardware threads. Every change below
passed the 1-1 hash script (98 points, recompiled and `--interpreter`,
13,194,609,027 instructions) before it was committed.

**The last first-pass commit's untested changes** (flat guest memory,
`fast::Raw` accesses for r1/r2/r13-relative D-form loads and stores, the
always-inlined FP conversions, `recomp::at` without the CPU): verified.
Recompiled and interpreter runs match at all 98 points with the same
instruction count. (An interpreter run made in the shared checkout just
before diverged from field 100 on, with 14.0 billion instructions; it did
not reproduce here and was most likely another session's settings or
saves at the time.)

**Measuring on a shared machine.** Other sessions build and run the port
at the same time, and wall time moved by up to 25% between identical
runs. Three tools came out of that:
- the exit report also prints the game thread's own CPU time
  (`GetThreadTimes`) and the wait for the GX thread's output at the end
  of each field, which `run_field`'s timer leaves out (about 0.5-1 s);
- `--headless` (with `--frames`) runs the game thread without a window or
  renderer and stops after the same fields a windowed run gets through
  (9701 for `--frames 9700`), so its hashes and instruction count compare
  directly; it needs no Vulkan (the console session lost its Vulkan
  driver for a while during this work) and takes no host input;
- a benchmark from a save state at field 6000 (just before 1-1) over 2000
  fields, run alternately with the two builds being compared, best of
  three. Saving a state resets the light-texture keys, so runs from a
  state are deterministic but leave the continuous run's hashes after the
  state's field; they compare builds with each other, not with the
  reference.

**Build times** (4 jobs, this PC, other sessions building too):

| Build | Time |
|---|---|
| Full release build, dependencies included | 12 m 51 s |
| of which the 16 shards | 70-113 s each, about 5.7 min in all |
| of which the final thin-LTO link (`frontend`) | 6 m 21 s |
| Release after a frontend-only change | 5 m 59 s |
| Without cross-crate LTO, after a frontend change | 16 s |
| Without cross-crate LTO, after an hle-os change | 20 s |

Thin LTO re-optimises every module of the generated code at the final link,
so any change at all costs about 6 minutes; the shards themselves never
rebuild for changes outside guest-cpu and guest-mem. Without cross-crate
LTO the port runs about a third slower (game thread 108.7 s of CPU time
against 80.7 s), so the release profile keeps it, and a new `quick`
profile (release without cross-crate LTO, `target/quick`) serves checks,
whose results do not depend on optimisation.

**Profile of the game thread** (release build, xperf at 1 kHz, 79 s of
CPU time): generated code 58%; the dispatcher (`run_field`) 8.9%, mostly
cache misses in the entry table (the page of `Rt::table`, then `fns`);
gather-pipe writes through `Mmio::write` 2.3%; the reverb HLE 2.0%; AX
mixing 1.7%; `Rt::enter_slow` 1.6%; ntdll 6% and the C runtime 4% (callers
to be found). In generated code, prologue pushes alone took 7% of its
samples: each call saved 8 general registers and xmm6..xmm15 (LLVM keeps
float constants there) under the Windows calling convention, and every
call went through a binary search of the function's `match` on `pc`. The
code is large: 80 MB of `.text` for 534K guest instructions, about 150
bytes each. `psq_l`/`psq_st` called `powi` for the GQR scale even for float
data. The GX decode thread (54 s of CPU) spends 33% in `Layout::read`,
11% in `Gpu::run`, 7% each in `Layout::new` (twice per draw) and texture
lookups (hashing every texture used, once per field), 5% reading arrays
through a virtual call per attribute, and 13% in the kernel; the transform
thread (35 s) is 74% `Xf::transform`.

**Changes:**

1. **Generated functions use the `sysv64` calling convention** (`GuestFn`,
   `GUEST_FN_ABI`): no callee-saved vector registers, two fewer saved
   general ones, and the 16-byte result in registers. A call entered at
   the function's start skips its `match`. Stack and small-data accesses
   cannot fail (`Mem::raw_read_u32` and co. return values; without the
   flat mapping the checked fallback is out of line and panics on no
   RAM), so they carry no error path. The binary shrank from 86 to 65 MB.
   Benchmark (game-thread CPU, best of three): 22.86 s against 23.75 s.
2. **Exact GQR scales, an entry cache, code tracking by line.** `psq_l`/
   `psq_st` build 2^n from its bits instead of calling `powi`. `Rt::lookup`
   goes through a direct-mapped cache of 8192 entries. Memory marks code
   per 32-byte line as well as per page, and a write to a code page that
   touches no code line (the OS globals beside the exception vectors)
   leaves its generation alone. Dispatcher 8.9% -> 6.9% of the game
   thread (plus 0.8% of cache misses), `Rt::enter_slow` 1.6% -> gone;
   benchmark 13.17 s against 13.53 s (a quieter machine by then).
3. **Allocation churn in the GX FIFO path** (about 7% of the game thread,
   found with stack sampling): the scanner allocated twice per 32-byte
   gather-pipe burst, display lists were copied before scanning, and
   `Hw::flush_gather` took a new burst vector every slice. On the decode
   thread, each VAT's vertex layout is now kept until the next CP load,
   and indexed arrays are looked up once per draw instead of through a
   virtual call per attribute. RAM hashes and GX output digests
   (`--hash-gx`, new) identical; benchmark 19.5 s against 21.0-21.4 s in
   like-for-like pairs.
4. **`mtmsr` runs in generated code** (it was an island: generated code
   unwound before it, the dispatcher stepped it and climbed back up the
   guest stack). It ends its block, so the next block re-checks MSR[FP]
   as before. Recompiled share 99.34% -> 99.57%; single-stepped
   instructions 67 -> 36 million; dispatcher iterations 635 -> 529
   million.
5. **Tail calls instead of unwinding** (the biggest step). Temporary
   counters showed the dispatcher entering recompiled code 487 million
   times in the 1-1 script, once per 27 guest instructions, nearly all
   at 0x802dcd48..0x802dcdd0: CodeWarrior's `_savegpr_N`/`_restgpr_N`
   (and `_save_fpr`/`_rest_fpr`), which most prologues and epilogues
   call. The map lists each entry point as a one-instruction function
   that falls through into the next; falling off a function's end
   returned with `pc` there, so every native frame unwound, and the
   dispatcher ran the rest one instruction per entry and climbed back up
   the guest stack. Code continuing in another function's entry is now a
   tail call, and a `bctr` out of the function an indirect tail call
   (`tail_indirect`). Hook addresses are no longer table entries, so an
   indirect call or jump to one always reaches the dispatcher. The exit
   report now counts the dispatcher's entries: 487 million -> 16.7
   million. Whole script: `run_field` 65.7 s, 51.3 s net of GX waits
   (it was 83-105 s, 61-82 s net). Benchmark: 11.73 s against 14.70 s.
6. **GX batches recycled.** The transform thread dropped each applied
   batch, so the decode thread grew every new one's vectors from nothing
   (150 bytes per vertex): page faults and heap calls were about a third
   of the decode thread on the world map, where it is what the game
   waits for. Applied batches now go back emptied (`Sink::spare`).
   World map (fields 6000-6240): GX waits 1.92 -> 0.87 s, `run_field`
   16.5 -> 10.7 ms per field.

**Tried and not kept** (verified identical, but no gain worth the code):
- `Cpu::spr` as a fixed array (one load per GQR read instead of a vector
  access with a bounds check) and `psq_l`/`psq_st` with the integer GQR
  types out of line: 4% less machine code, no measurable change.
- The reverb HLE working on one slice over all its buffers (a new
  `Mem::data_mut`): about 0.6 points of the game thread, because its
  loop is float maths and strided delay-line accesses, not the checks.
- Without cross-crate LTO (see the build times above): a third slower.
7. **Paced: the field's GX output is collected after the sleep.** The
   game thread used to wait at each field's end for the transform thread
   to finish the output (with interpolation records above 60 fps), then
   sleep until the field's slot ended. It now asks for the output
   (`GpuPort::request_output`), sleeps, and collects it
   (`finish_output`); the frame's display time was already the slot's
   end. The channel to the window holds four fields instead of two.
   Paced from field 6000 (world map, then 1-1), two other builds running:
   end-of-field waits 0.27/0.76/0.87 s at 60/240/360 fps became
   0.02/0.18/0.08 s, and the world map holds 100% at 240 and 360 fps (it
   was 92-98%; at 60 fps it was already 98-100%).
8. **30 fps.** `--fps 30` (F5/F6; the battery saver shares the path) shows
   every other field with the game at 59.94. The battery saver skipped a
   field only if it made no EFB-to-texture copy, which in courses and on
   the world map is never, so it showed 60 fps. A skipped field now runs
   its draws up to its last texture copy, and its clears and copies after
   that, but not the draws after it, and uploads only the vertices those
   draws use. Paced: 30.0 fps shown, game 100% throughout. Screenshots of
   fields 6201, 6461 and 6501 are byte-identical at 30 and 60 fps.

9. **Priorities.** The process stays at normal priority (it raised itself
   to above normal); inside it the game thread and both GX threads run
   above normal (`win::raise_this_thread`, run first by each GX thread),
   so a window drawing hundreds of frames a second does not slow
   emulation, and the rest of the PC stays responsive.
10. **Hooks called from recompiled code.** The hooks that replace a whole
   function (return at once, the reverb, `strcmp`, `sqrt`) became one
   function, `hle_os::replace`, that `System::before` uses and that
   recompiled code calls through `Rt::set_hook` at a call, jump or
   fall-through to a hook address, instead of unwinding. A hook that
   declines changes nothing, so the dispatcher then decides the same way;
   the light-texture, idle and `OSLink` hooks stay in the dispatcher. The
   hook list is unchanged. Dispatcher entries 16.7 -> 5.3 million; scene
   loads (`strcmp` runs about 5 million times in the script) 30-45 ms ->
   17-32 ms per field.
11. **Vertex loader, part 1.** `Layout::read` called `comp` (not inlined)
   for every component; `comps` picks the format once per attribute. GX
   waits on the world map down 25-40%, whole script 12.0 -> 8.6 s.
12. **`strcmp` reads each string's RAM once** (`ram_from`) instead of a
   byte at a time through `Mem::ram`, which was 10% of the game thread
   during the opening cutscene's load.
13. **Vertex loader, part 2.** Vertices are read straight into the batch
   (`Layout::read_into` over a template): built on the stack and copied
   out, every vertex stalled on reading back its own fresh stores. World
   map GX waits another 20% lower.

After 13, headless: `run_field` for the whole script 48.1 s, of which
6.7 s waiting for the GX thread. Paced at 360 fps over the whole script,
with two other builds running: game speed median 100%, minimum 97%; only
the opening cutscene's load (field 3121, 29 ms of guest string and
resource searches) and the course clear (fields 9406-9413) leave a second
below 99%. Shown fps (the renderer, branch `render-400`) dipped to 252.

14. **Catch-up of at most two fields** (11.6). A paced field up to 33 ms
   late keeps the schedule; the next fields skip their sleep until it is
   met. Later than that, the schedule restarts from now as before. Paced
   at 360 fps over the whole script on a quiet machine: game speed 100% in
   every second (minimum and median); after step 13 the opening
   cutscene's load and the course clear left seconds at 97-98%. Shown fps: median 360,
   minimum 286 (this branch's renderer; `render-400` has its own). With a
   build starting during the run, seconds dropped to 80-94% at fields
   that are never slow headless, with 80-150 ms render stalls: other
   load, which the catch-up does not cover and should not.
15. **Exit report: slow fields.** Every run, headless too, ends with how
   many fields took longer than 16.7 ms to emulate and the eight slowest.
   1-1 headless: 10 fields, all boot (0-36), the opening cutscene's load
   (3121, 3122, 3128, 18-25 ms) and 9407 (19 ms).
16. **Four players** (`scripts/1-1-4p.press`, `--frames 11000`). Remotes
   2-4 switch on at field 2000; Select Players picks 4 and the three
   press OK. With four players the World 1 map comes at ~3600, without
   the opening cutscene. All four run 1-1 on 1-1.press's inputs (with hops
   every 40 frames after), die and come back in bubbles, and Luigi takes
   the flagpole at ~10250; by 11000 all four are back on the map.
   - Lockstep: recompiled and interpreter identical at all 111 hash points
     and 19,912,994,930 instructions, headless and windowed. (A windowed
     run with `--shot` and `--watch-player` ran 11.8 million more
     instructions, 4 more light-texture redraws, with the same RAM at all
     111 points: renderer timing reaching the game, not the engine.)
   - P6's check, 60 fps with headroom in 4-player 1-1: paced at 60 fps,
     game 100% (the lowest second 98%, a 59-field second), emulation
     8.2 ms per field on average in 1-1 and 12.9 ms in its worst second;
     headless, only boot and the map load after Start Game (3524-3525,
     26-30 ms) take over 16.7 ms.
   - Paced at 360 fps from field 6290 on a quiet machine: game 100% in
     every second; 5 fields over 16.7 ms (the level's start and the map
     load after the course clear), none waiting on GX. The same run with
     three builds going took 48-51 s of game-thread CPU instead of 29.6 s
     and dipped to 81-95%: other processes sharing the cores, not the
     emulator. Timing runs need a quiet machine (`bench.lock`, and no
     builds).
17. **Fields that wait for GX** are shown in the slow-field report (each
   field's GX wait), which is how the above was told apart.
18. **Vertex transform in place.** `Xf::transform` built each 144-byte
   vertex on the stack and returned it: its texture coordinates, stored a
   float at a time, were copied out with 32-byte loads that cannot take
   their data from those stores, and every vertex paid a call's prologue,
   bounds checks on XF memory, and one or two calls to `light_channel`.
   `Xf::transform_into` writes a draw's vertices into the output in place,
   with the vertex body and `light_channel` inlined. The course clear has
   80,000 vertices a field (4-player 1-1 25,000-30,000): 80-85 -> 62-69 ns
   a vertex before inlining `light_channel`; with it, the game thread's
   end-of-field wait for GX output at the course clear fell from 1.8 to
   0.45 ms a field (headless, from field 9380). GX digests identical.

With the frame rate unlocked (`--fps unlocked`, this branch's renderer),
the whole 1-1 script shows a median of 645 fps (minimum 303) with the
game at 100% (median; the lowest second, 98%, is a 59-field second at
boot).

**Result** (1-1 hash script, windowed, the first build of this pass
against the last, alternating, best of two, same machine state):

| | Before | After |
|---|---|---|
| `run_field` (host time) | 89.6 s | 58.5 s |
| of which waiting for the GX thread | 20.6 s | 12.9 s |
| net (the game thread's own work) | 69.0 s | 45.6 s (-34%) |
| game thread CPU time | 70.3 s | 47.3 s |
| dispatcher entries into recompiled code | (~560 million) | 16.7 million |
| interpreter, net | 156.8 s | 149.0 s (-5%) |

The binary went from 86 to 65 MB. Every step kept all 98 hashes and the
instruction count (13,194,609,027) in both engines, headless and
windowed.

**The build-test loop now:**
- `cargo build --profile quick -p frontend` for checks: release without
  cross-crate LTO; a change outside guest-cpu, guest-mem and the
  generated code relinks in 16-20 s (release: about 6 minutes).
- `--headless --frames N` for hash checks: no window or renderer, no
  Vulkan needed, the same fields and instruction count as a windowed run.
- `--hash-gx` with `--hash-every`: digests of the GX output, to check
  that a GX change leaves what the renderer gets alone.
- A state at field 6000 (`--save-state 6000:st6000.bin`, then
  `--load-state st6000.bin --frames 2000`) makes a 15-30 s benchmark;
  compare builds alternately, by game-thread CPU (the exit report).
- Since 2026-11-20 builds run through `D:\nsmbw-p6\cargo-limited.ps1`
  (2 jobs, below-normal priority, one build on the PC at a time), so a
  full rebuild takes 15-30 minutes including waits for other sessions;
  the timings above were taken at 4 jobs.

**Still to do (engine):**
1. **Scene loads** still have single fields of 18-29 ms (the opening
   cutscene's load: guest `strncmp`, `breftSearchName`, `getResInfo`,
   `ARCConvertPathToEntrynum`), which step 14 makes up. Native versions
   of those searches would remove them; they would be new hooks, left
   for later.
2. **The GX threads** in heavy scenes: the transform is still about
   60 ns a vertex (normalising and dividing for lighting and texture
   coordinates; doing several vertices at once with SIMD, or the transform
   on the GPU, 11.6, would be the next step); the decode thread's
   `Layout::read_into`, and the 150-byte `RawVertex` per vertex.
3. **Generated code** is still about 120 bytes per guest instruction:
   checked loads and stores inline their slow paths, every branch checks
   the slice budget, and loops go back through the `match` on `pc`.
4. **Interrupts and thread switches** still unwind and climb (16.7
   million dispatcher entries); a native return stack would be needed.

## 15. 400 fps with the game at 100% (frames shown, 2026-11-20)

Goal (WitherBlitz, 2026-11-20): 400 frames shown a second with the game
capped at 100%. The engine's side (game speed) is section 14's second
pass; this is the window thread's side: frames shown between fields
(11.6) at 400 a second, and nothing on that thread holding up the game.

**Check:** the whole 1-1 script (and `scripts/1-1-4p.press`) paced at
`--fps 400 --no-vsync --log-fps`, recompiled code, on a quiet machine
(no build or other emulator running: the build and bench locks in
`D:\nsmbw-p6`). `--log-fps` prints, every second, frames shown, game
speed, and the window thread's time taking fields in, preparing them, and
showing frames.

**What changed:**
1. **Frame interpolation on the GPU.** Frames shown between two fields
   rebuilt every vertex on the CPU (11 MB a frame on the world map: 2.7 ms
   of window thread per frame, 310-350 fps at most). Now, once per pair of
   fields, the newer field goes to the GPU in one buffer (vertices,
   indices, and `gx::interp::Moves`: model positions, each matrix pair
   split into quaternions, scales and translations, and each draw's two
   projections and viewports). The vertex shader blends them at the
   fraction `t`, a push constant, as `interp::interpolate_into` does (a
   test runs the shader's maths against it). Draw states and descriptors
   are written again per frame slot only when the field or the textures
   change. `Renderer::prepare`, `render_prepared`.
2. **Less to prepare per field.** The world map has about 80,700 vertices
   in 11,600 draws a field. Draws that did not move are left out of the
   moves (their vertices are already right); each distinct camera pair
   and matrix pair is built once; pairing walks records sorted by
   identity; records keep their matrices in one list per field (the
   transform thread allocated one `Vec` per record). Prepared-field
   buffers are reused. The pairing and moves are built by an `interp`
   thread as soon as a field arrives, so the window thread only uploads.
3. **Per frame shown:** draws set only the dynamic state that changed;
   the wait for the next frame sleeps on a high-resolution waitable timer
   and spins only the last 250 us (it spun 1.5 ms).
4. **Frames driven every turn of the event loop** (`about_to_wait`), not
   by redraw requests: Windows delivers those as paint messages and held
   them back to ~9 ms (or far longer while the window was covered),
   which capped one run at ~110 fps.
5. **Nothing on the window thread waits on the outside world:**
   - The title no longer shows live numbers: setting it blocked the
     window thread 15-40 ms every few seconds (overlays and accessibility
     tools hook it). They are in the performance monitor (F3) and
     `--log-fps`.
   - Lines printed while the game runs go through an output thread
     (`crate::say`). A kernel trace (xperf: context switches with
     stacks, and file I/O) of a run that stalled for 0.3-1 s at scene
     loads showed the window thread blocked on writes to its redirected
     log on this PC's laptop hard drive, each flushing the NTFS journal
     and the disk cache (38-350 ms apiece, 18 s over the run); a console
     in selection mode blocks writes the same way. The game thread then
     waited on the full frame channel.
6. `--fps 400` (and in the F5/F6 cycle, with engine-speed's 30).

**Measured** (World 1 map, unlocked frame rate, no V-Sync): 310-350 fps
shown at 2.7 ms of window thread per frame before; ~600 after 1-2. At
`--fps 400`, recompiled: frames shown ~400 at 98-101% game; window thread
about 25-40% busy on the map (it was 80%), 15-20% in 1-1.

**Result** (render-400 on engine-speed's a2dbf3f, recompiled code
regenerated, quiet machine, `--fps 400 --no-vsync`, per-second readings):

| Run | Seconds | Frames shown min / median | Game min / median |
|---|---|---|---|
| Whole 1-1 script (9,700 fields) | 159 | 385 / 400 | 97% / 100% |
| Four players, `scripts/1-1-4p.press` (11,000 fields) | 181 | 389 / 400 | 97% / 100% |

A one-second window holds 59 or 60 fields, so 97-98% is the reading's
grain, not a slowdown. The only second under 388 fps was the opening
cutscene's load (field ~3161, 384.7 fps). Instruction counts are the
lockstep references' (13,194,609,027 and 19,912,994,930).

**Lessons for measuring:** other sessions' builds and game runs on this
PC (CPU and GPU) and output redirected to the hard drive both produced
dips that looked like engine problems. Take timings with the build and
bench locks held, and log through a pipe or to the SSD.

## 16. Android (first milestone, 2026-11-20)

Goal (WitherBlitz, 2026-11-20): an Android version (arm64, Android 9 and
up, Vulkan) with the app shell, touch controls and Android features of
WitherBlitz's NSMB-Wii-Recompilation, whose Java app is adapted here.
Branch `claude/android-7bd682`.

**Pieces:**
- `crates/android` builds `libnsmbw.so` (package `nsmbw-android`; empty on
  other targets). It runs the desktop game thread unchanged: `emu.rs`,
  `net.rs` and `say.rs` are included from `crates/frontend/src` with
  `#[path]`. Its own `audio` (AAudio through the `ndk` crate, the same
  queue and resampling as the `cpal` one) and `win` (thread priority nice
  -8, thread CPU time) stand in for the frontend modules of those names.
  stdout and stderr go to logcat (`adb logcat -s nsmbw`).
- A render thread takes each field and presents it at the game's own 60
  fps, as the desktop window does at 60 (`play.rs`'s native path; no frame
  interpolation yet). The renderer is kept for the whole run: Android
  destroys the surface whenever the app leaves the screen, so
  `vk_render::Renderer` gained `detach_window` and `attach_window` (only the
  surface and swapchain go; textures stay). A display turned from its
  natural orientation (a landscape game on a portrait phone) gets an
  upright swapchain with the turned extent, and the compositor turns it.
- `android/` is a Gradle project (Android Gradle Plugin 8.13.2, Gradle
  8.13, JDK 17, no AndroidX), app id `com.wither.nsmbwport` so it installs
  beside the recomp's `com.wither.nsmbw`. From the recomp's app (its own
  code; the SDL Java files are not used):
  - the start menu (Play, Language, Help, Settings, Edit Touch Controls,
    Skip Start Menu, and importing the extracted disc as a folder through
    the system picker or as a zip, with the recomp's SMNE01 rev 1 and
    file-list checks) and the Settings screen, cut to the settings this
    port has (Video: resolution, V-Sync, aspect; Controls: touch, opacity,
    tilt, shake, Reverse Tilt, vibration; Language);
  - the on-screen sideways Wii Remote (`TouchControlsView`: D-pad, 1, 2, A,
    Shake, + and -, opacity, vibration) and its layout editor with profiles;
    it hides while a controller is used and a tap brings it back;
  - tilt and shake from the phone's gravity and accelerometer sensors
    (`MotionInput`), with Reverse Tilt on by default;
  - controllers as Wii Remotes 1-4 with the desktop's Xbox-layout defaults
    (`ControllerInput`, as `crates/input`);
  - the game's rumble on the phone's vibrator;
  - the Back gesture's menu (Start Menu, Edit Touch Controls, Quit Game);
  - leaving the app pauses the game (no fields are taken, so the game
    thread stops at its next one; sound plays silence).
- The game's language is the Wii's (`SYSCONF` `IPL.LNG`, set at each start
  with the aspect ratio): English, French or Spanish from the USA disc;
  Italian menus keep the game in English.

**Build** (this PC: through `D:\nsmbw-p6\cargo-limited.ps1` and
`android\gradle-limited.ps1`, which take the build lock):

```
rustup target add aarch64-linux-android        # once
powershell -NoProfile -File android\build.ps1 [-Profile quick] [-Install]
```

`build.ps1` finds the NDK (`ANDROID_NDK_HOME`, else the newest under
`%LOCALAPPDATA%\Android\Sdk\ndk`), builds `libnsmbw.so` for API 28 with
the NDK's clang as linker, stages it in `android\native\jniLibs`
(ignored), and runs Gradle offline: `android\app\build\outputs\apk\debug\app-debug.apk`.
Set `CARGO_TARGET_DIR` to keep the arm64 build off a full drive.
`.cargo/config.toml` now gives `target-cpu=x86-64-v3` to the Windows
target only, and `generic` to arm64 (a worktree under the main checkout
also reads the main checkout's config).

**Run:** install the APK, open NSMBW Port, import the folder Dolphin's
Extract Entire Disc made (or a zip of it), and tap Play. For testing over
adb, a debug build's storage can be filled directly:

```
adb push <disc>\sys <disc>\files <disc>\tmd.bin /data/local/tmp/nsmbw-game/
adb shell run-as com.wither.nsmbwport sh -c "mkdir -p files/game && cp -r /data/local/tmp/nsmbw-game/* files/game/"
adb shell rm -r /data/local/tmp/nsmbw-game
```

(From Git Bash, set `MSYS_NO_PATHCONV=1` first.) The game needs `sys/`,
`files/` and `tmd.bin`, which the import checks for.

For silent test runs, a file named `no-audio` in the app's files starts
the game without sound (as `--no-audio` does on the PC); remove it to
hear the game again:

```
adb shell run-as com.wither.nsmbwport touch files/no-audio
adb shell run-as com.wither.nsmbwport rm files/no-audio
```

**Checked** (2026-11-20) on WitherBlitz's Galaxy SM-S948U (Android 16,
Snapdragon 8 Elite Gen 5, Adreno 840, 3120x1440 at 120 Hz), debug APK,
release `libnsmbw.so`, interpreter only, EFB 1x, V-Sync on:
- the start menu finds the imported game and passes its checks; Play
  boots it: strap screen, title screen, Select a File and Select
  Players, all through the on-screen remote's A, 2 and 1 (logcat shows
  each button reaching Wii Remote 1);
- the picture is upright, 16:9 between bars, the remote drawn over it;
- the game runs at full speed: 98-100%, 7-9 ms of emulation and about
  2 ms of rendering a field on the strap and title screens; 11-16 ms and
  2-4 ms in the title's attract demo (courses), at 96-100% (before the
  swapchain fix below: 10-21 ms of rendering and 78-90% there). After an
  hour of runs the phone throttles and emulation rises to 23-28 ms
  (60-65%);
- leaving the app (Home) stops the game (no fields run while away) and
  coming back carries on, drawing to the new surface; the Back gesture
  opens the menu; an error from the game or the GPU shows in a dialog with
  Start Menu and Quit Game.

Every 10 s the render thread logs the game's speed, the emulation and
render times, the textures held and the GPU memory in use.

**Fixed with the validation layer** (Khronos's Android build, release
1.4.363.1, packed into the APK by `build.ps1 -ValidationLayer <.so>` and
switched on by a file named `validation` in the app's files; a release
build enables the layer when `NSMBW_VALIDATION` is set; a repeated message
is printed 20 times, then at powers of two):
- **The swapchain was rebuilt at every present.** An upright swapchain on
  a turned display is reported suboptimal each time; that is now ignored
  for a swapchain the renderer chose to keep upright. Rendering fell from
  5-21 ms to 2-4 ms a field, and the attract demo holds 96-100%.
- **Draws used cull and depth state that a copy had invalidated.** After
  an EFB copy (its own pipeline) a clear restarted the EFB pass without
  forgetting the cached dynamic state, so the next draws did not set it
  again. Desktop drivers keep the old state; the specification and Adreno
  do not.
- **Image views were made for images that are only copied** (XFBs, the
  palette), which is not allowed; they get none now.
- Still reported, not fixed: `naga`'s SPIR-V gives function-local
  variables of buffer-layout struct types `Offset` decorations
  (VUID-StandaloneSpirv-None-10684); drivers accept it.

**Worked around, cause unknown: the device was lost in the attract
demo's snow course**, about 250 fields in (field ~7,130 of a run; once at
~8,270), with Adreno GPU-fault snapshots in logcat and then
`VK_ERROR_DEVICE_LOST`. The validation layer reports nothing there. What
32 runs on the phone established (temporary switches in the app's files,
since removed):
- Skipping the draws that use indirect texturing (the lit 3D models,
  which sample the 32x32 light-texture copies) removes it. Removing the
  indirect lookups, the indirect commands, or the copies does not, or
  only moves it.
- It needs the fragment shader's screen derivatives (`dpdx`, `dpdy`) to
  be in use. Clean: `textureSampleLevel` at level 0; `textureSampleGrad`
  with zero or constant gradients taken from a uniform (four runs, each
  past field 11,800). Faulting: the real derivatives passed on, clamped
  (to 8, to 0.001), zeroed unless under 64 (by `if`), or only added to
  the coordinate of a level-0 fetch; all 27 runs with the derivatives in
  use faulted.
- It is not a bad value or index: it stays with the coordinates and
  gradients made finite by their bits, the shading index clamped to the
  buffer, every texture and sampler slot forced valid without a branch,
  samplers limited to level 0 or to their image's levels, robust buffer
  and image access, and `naga` clamping local array indices.

So on Android the ubershader samples at the base level
(`shaders::ubershader_source`), which leaves the derivatives unused: no
mipmaps there (this game's textures are mostly drawn near 1:1). Desktop
is unchanged. With it the attract demo ran 6 minutes (19,082 fields,
through the snow course and round the loop) without a fault. To find the cause: which of the eight coordinates'
derivatives matters, the same scene on another Adreno and on a Mali, and
the kernel's GPU fault line (needs a bug report or root).

**Not yet:** recompiled code (`ppc-recomp` writes the host's calling
convention, `extern "sysv64"`, into `gen/`, which arm64 cannot compile; the
Android build interprets everything), the flat guest-memory mapping
(`guest-mem/flat.rs` has only the Windows one; Android would use
`memfd_create` and `mmap`), frame interpolation above 60 fps, Fill Screen
(the recomp's Hor+ widening; Fill shows 16:9 here), tapping the game's
menus, the Power-Up button, the quick-save backup when leaving the app, and
the in-game settings window (built on another branch; options will meet
through a shared settings file).
