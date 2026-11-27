# Rewriting NSMBW in Rust: the plan

The goal: New Super Mario Bros. Wii as native Rust, running with no emulated Wii, playing exactly like the original. Repo: the-mario-bros-team/NSMBWDecompilation (Rust reconstruction in `src/`, playable port in `crates/`).

Written 2026-11-25. Whoever works on this (Claude, Codex or anyone) does the phases in order, keeps the **Progress** table at the end true, and hands off through this file and the branch `rust-guest-memory`.

## Where it stands (measured 2026-11-25, repo at `246bc22`)

| | Done | Total |
|---|---|---|
| Functions written in Rust (`src/`) | 22,281 transcribed + 1,219 with real names | about 24,700 (Ghidra's 20,252 + 2,457 found later; **2,198 not written yet**) |
| Of those, ever run or checked against the game | 0 | all |
| Real data types | 19 structs | thousands of classes |
| Names the recomp's `MAP.txt` already knows, main program | 8,958 of the 9,195 still called `fn_<address>` | |
| REL functions (enemies, bosses, objects) | written against placeholder addresses (`0x80A00000` base) | 9,938, which need translating to real addresses before any name or check applies |

Name sources available here: the recomp's `MAP.txt` (16,064 names; 13,838 are C++ signatures, which also give each function's class and argument types), a separate symbol analysis (`symbols_E1_merged_v13.tsv`, 23,305 reviewed names), RootCubed's symbol list and hashes (they prove a guessed name). The community NSMBW-Decomp has no license: read it for understanding only, copy nothing (no names, field names or layouts). The same goes for any source whose license is unknown.

## The one rule

**Nothing counts as done until the running game proves it.** The port in `crates/` runs the original code, so it is the referee. Every Rust function that replaces an original one must give byte-identical guest memory over the 1-1 hash script (98 hashes, 13,194,609,027 instructions) and the all-courses scripts, plus a per-call lockstep check on its real inputs. Code that has never run doesn't get marked done, however good it looks. That's exactly what went wrong with the 20,000 functions written so far.

## Phase 0: make the Rust runnable inside the port (do this first)

Without it, nothing in Phase 1 can be checked.
1. `platform.rs` reads and writes guest memory (`guest_mem::Mem`, big-endian) instead of treating Wii addresses as PC pointers.
2. Provide guest-address calls through the port dispatcher, including mixed integer and floating-point registers, explicit EABI stack arguments, and temporary guest buffers.
3. Provide a verified path from ELF analysis addresses to a module, section, and offset, then resolve the location through the live guest module list.
4. A switch table: run the Rust body instead of the recompiled one for chosen addresses. Lockstep mode compares registers and touched memory after each call (PORT_PLAN 2.2's validator).
5. **Checks.** Run the seeded 1-1 replay with native `memcpy` and compare all 98 RAM hashes. Separately run `cargo test --profile quick -p hle-os synthetic_native_actor_matches_98_checkpoints -- --ignored`. The actor fixture generates its own boot files and requires one native dispatch, a matching CPU snapshot, and a matching RAM hash at every checkpoint.

The user approved this runtime verification scope on 2026-11-26. Track bulk migration of existing source calls and REL placeholders after Phase 0. Track original-title in-game actor parity separately.

## Phase 1: every function and data structure understood with real types

1. **Names.** Merge the sources above into one table: address → name, where it came from, proof. Main program first (8,958 names ready), then the RELs after Phase 0.3. Demangle the C++ signatures into class and argument types.
2. **Class list.** From the signatures, vtables, the sizes passed to `operator new`, and the actor profile table: every class, its size and its base class.
3. **Layouts.** For each class, field offsets and types from how its functions use them. Order: the base chain every actor shares (`fBase_c` → `dBase_c` → `dBaseActor_c` → `dActor_c` → `dActorState_c` → `dEn_c`), then the player, then enemies by how often courses use them, then bosses, world map, menus.
4. **Typed bodies.** Rewrite functions to use the types (`enemy.records[i]` instead of `actor + 0x6B4 + i * 0xA4`), still on guest memory. Every batch passes the gate.
5. **The missing 2,198 functions,** written directly in typed form.

## Phase 2: prove all of it

Extend the gate past 1-1: every course in all nine worlds (35 already have hash scripts), bosses, world maps, menus, 4 players. Then switch **all** game logic to Rust at once and play through. Milestone: the game's own code is 100% Rust; the Wii SDK and engine libraries still run recompiled.

## Phase 3: replace the Wii engine layers

One layer at a time, each checked by its own output (GX digests, audio hashes) and the gameplay hashes:
1. Math (PSMTX/PSVEC) and string and memory helpers in native Rust.
2. OS threads, alarms and heaps on host primitives.
3. Graphics: GX at the API level onto the port's renderers (as the recomp does), then NW4R models and layouts drawn natively.
4. Sound (AX, NW4R sound) onto a host mixer; disc and NAND onto files.

## Phase 4: native data, no emulated Wii

Move objects off big-endian guest memory into native Rust structs, one subsystem at a time, once all code touching them is Rust. File loaders read the disc's formats (U8, BRRES, BRLYT, BRSTM) straight into native structures. When guest memory is gone, the emulator is gone, and that's where it gets much lighter than the recomp.

## Speed in the meantime

Phases 0-2 run at about the port's current speed. In parallel, `docs/PORT_SPEED_COMPARISON.md`'s steps 1-4 (translate every function, registers in locals, cheaper memory, fewer budget checks) make the port itself fast now.

## Working with more than one AI

- One worker per module at a time, on its own branch. Write which module you hold in the Progress table before starting.
- Bulk work (names, typed rewrites, missing functions) can go to any model **only once Phase 0's gate exists**. The gate catches mistakes, not the model.
- Hard parts need the strongest reviewer: the type system design, base classes, the player, anything the gate flags that isn't obvious.
- Handing off: update the Progress table, with what's done, what's checked, and what's next, and push the branch. A prompt to the next AI should just point here.

## Commits, pushes and the rules for every worker

- **Branch:** all of this lives on `rust-guest-memory`. Never push to `main`, and never tag or release; merging to `main` is the owner's call.
- **Commit when a step passes the gate, and push it.** Any AI may push this branch (the owner's standing permission, 2026-11-27). Name the step and the gate result in the message, e.g. `Phase 1.4: dEn_c typed bodies (gate: 98/98, lockstep 256/256)`, and update the Progress table in the same commit. Work that hasn't passed stays local.
- **Pull before you start and push before you stop.** One AI at a time; write the module you hold in the Progress table.
- **Never commit game files:** `main.dol`, RELs, RAM dumps, saves, recompiled output, the `work/` exports. They come from the owner's own disc.
- **Public text:** no private email addresses, tokens, LAN addresses or machine details in files or messages.
- **Unlicensed sources:** names withheld in `docs/rust-plan/phase1-name-table.tsv` (provenance `withheld: ...`) were found only in the community NSMBW-Decomp or another source of unknown license; derive them from the binary before using them. Field names in the actor layouts should be re-checked the same way.
- **Ask the owner first** before anything that could hurt performance or that changes gameplay.

## Rough size

These are sessions of about 5 hours, with parallel agents. The later phases are the least certain.

| Phase | Sessions |
|---|---|
| 0 | 2-4 |
| 1 | names 1-2; types and typed bodies 50-120 |
| 2 | 15-40 |
| 3 | 40-100 |
| 4 | 40-100 |
| **All** | **about 150-370**, roughly 4-12 months at 1-2 sessions a day |

## Progress (keep this true)

| Step | State | Who | Notes |
|---|---|---|---|
| Plan | written 2026-11-25 | Claude | |
| Phase 0 | complete 2026-11-26 | Codex | Approved runtime scope verified on local `rust-guest-memory` from `246bc22`. The disc-free synthetic actor passed all 98 CPU and RAM checkpoints with one native dispatch per checkpoint. The seeded 1-1 replay matched all 98 RAM hashes with 740,425 native memcpy commits. Bulk source migration and original-title actor parity are tracked separately. |
| Phase 0.1 guest-memory adapter | implemented and verified | Codex | The opt-in `guest-memory` feature uses the same `System` RAM for platform loads, stores, and synchronous register calls. Platform unit tests and a live `System` probe passed on Ubuntu WSL. |
| Phase 0.2 guest call registers | shared bridge implemented and verified | Codex | `with_guest_runtime` connects `call0`–`call7`, `call_u64`, and typed `call_registers` to `System`. The typed entry uses r3–r10 and f1–f8 and captures r3, r4, and f1. Tests covered register limits, a wide return, caller restoration, and mixed arguments at a resolved REL entry. The rebuilt seeded 1-1 replay matched all 98 RAM hashes with native memcpy active. The bridge covers the shared helpers and additional OS sites. `call_frame_with_scratch` supports bounded guest buffers and explicit EABI stack arguments. Synthetic calls verified input, copy-back, mixed spills, and fault restoration. The bridge now covers another seven OS call sites. |
| Phase 0.3 live REL resolver | implemented and verified | Codex | `System::resolve_rel_address` uses live module IDs, original section indices and section-relative offsets. Synthetic tests covered unload, bounds, executable alignment, and mixed-register calls through a resolved entry. A field-7000 RAM capture resolved 4 executable and 20 data sections across 4 modules. The seeded replay still matched all 98 hashes with native memcpy active. `node tools/rel2elf.mjs --resolve-address <summary.json> <0xaddress>...` converts ELF analysis addresses to module, section and offset. Synthetic converter output verified code, data, BSS, and ambiguous or unmapped addresses. The platform exposes the lookup and `call_rel_registers`, which resolves and calls within one borrowed runtime. The end-to-end fixture used ELF tuples for a mixed-register guest call, read data and BSS, and proved unloaded code is rejected. The metadata-only `--address-layout` mode accepts raw or LZ11-compressed modules. All four local RELs matched the live capture on 24 sections and 48 endpoint lookups. Source placeholder migration is tracked separately. |
| Phase 0 synthetic native actor | verified | Codex | The disc-free test `synthetic_native_actor_matches_98_checkpoints` uses the native registry and asserts one native actor dispatch per checkpoint. All 98 CPU snapshots, RAM hashes, actor state values and retired instruction counts matched the interpreter. |
| Phase 0 host build | x86-64 builds passed | Codex | Rust 1.97.1 built both default and `guest-memory` configurations. All four adapter unit tests passed. Unsupported PowerPC-only entries have explicit host stubs. |
| Phase 0 `memcpy` dispatcher | initial byte-only experiment superseded | Codex | The initial seeded run differed at 97 of 98 RAM checkpoints. The original matched the byte-copy destination in 50 observed calls and changed registers and guest timebase in all 50. The guarded native checkpoint is tracked below. |
| Phase 0 per-call validator | tested in the live interpreter | Codex | `NSMBW_RUST_MEMCPY=validate` diagnoses the initial byte-copy primitive. Seven comparisons matched RAM and differed on registers and timing. One crossed a hardware slice and was inconclusive. The validator replay matched all 98 baseline hashes, the reference instruction count, and all 358 logged positions. |
| Phase 0 native memcpy parity | passed 2026-11-25 | Codex | `NSMBW_RUST_MEMCPY=1` on the seeded USA revision 1 interpreter replay matched all 98 RAM hashes, all 358 positions, and 13,194,609,027 guest instructions. The hook made 740,425 live commits covering 16,106,843 bytes. |
| Phase 1.1 evidence inventory | verified 2026-11-26 | Codex | Accounted for 20,250 ledger rows across the main executable and four RELs. Exact local module hashes and three lifecycle anchors per REL establish the offset crosswalk. The inventory records 12,784 rows with name candidates, 1,007 with recorded signatures, eight spelling conflicts, and 126 rows supported by placement alone. The five Ghidra inventory exports remain absent. Outputs: `docs/rust-plan/phase1-ledger-evidence.jsonl` and `docs/rust-plan/phase1-evidence-verification.json`. |
| Phase 1.1 name table | generated 2026-11-26 | assistant | `phase1-name-table.tsv` and `.jsonl` give one chosen name per ledger row, 12,772 of 20,250, with provenance and confidence. 9,926 agree across sources, 2,846 come from one source, eight conflicts are kept unresolved, and 7,470 have no name. Cross-check: all 10,243 main-program rows matched by address to `symbols_E1_merged_v13.tsv`; the 10,000 chosen names match it exactly, 0 differ. Nothing was applied to Rust sources: a name is a candidate, not proof of function identity. Byte-identical regeneration. |
| Phase 1.2 class list | generated 2026-11-26 | assistant | `phase1-class-list.tsv` lists 1,246 class candidates from the demangled signatures, with namespace, method count, resolved/unresolved names, and modules. 8,639 ledger functions sit in a class, 1,024 in a free-function namespace (`nw4r::ef`, `dWmLib`, `dGameCom`, ...), 10,587 are free. Namespace grouping uses naming-convention heuristics and is a candidate list, not a confirmed hierarchy; sizes and base classes are not yet derived. |
| Phase 1.3 actor base chain | verified 2026-11-26 | assistant | Layouts for `fBase_c` (0x64), `dBase_c` (0x70), `dBaseActor_c` (0x128), `dActor_c` (0x394), `dActorState_c` (0x3D4), `dActorMultiState_c` (0x414) verified by decoding their constructors from `main.dol` (SMNE01 rev 1), not from a header: every field offset/width and every sub-object constructor call is checked, sizes chain end-to-end, 0 fail 0 warn. Outputs: `docs/rust-plan/phase1-actor-base-layout.md` and `.gate.txt`; layout data and gate in `docs/rust-plan/tools/actor-base-layout.json` and `docs/rust-plan/tools/layout-gate.cjs`. **Correction:** `dEn_c` does not derive from `dActorState_c`; it derives from `dActorMultiState_c`, and its own fields start at 0x414. `dEn_c` fields before 0x47C are verified; the tail is header-only. |
| Phase 0 toolchain pin | verified 2026-11-26 | assistant | Rust 1.99.1 rebuilt the port on Windows (4 jobs, BelowNormal). The seeded 1-1 replay matched all 98 RAM hashes, 13,194,609,027 instructions, 358 player positions, and 740,425 native memcpy calls / 16,106,843 bytes. `rust-toolchain.toml` pins 1.99.1. |
| Phase 1 layout views | generated 2026-11-26 | assistant | One JSON per class in `layouts/actor/`; `tools/gen-layout-views.mjs` emits `src/game/layouts.rs`: a 32-bit guest pointer plus big-endian accessors, `SIZE`/`OFFSET_*` constants, `as_<base>`, and a size/offset test per class. The seven base-chain classes compile and their 7 tests pass. No `#[repr(C)]` shadow structs; Phase 4 will reuse the same JSON. |
| Phase 1 evidence binary-first | implemented 2026-11-26 | assistant | `tools/verify-ledger.mjs` now accepts binary evidence: `dol2elf` builds `main.dol`, `rel2elf --address-layout` maps the four RELs, and `disasm-ppc` proves sampled addresses. All 20,250 rows are evidenced; the five absent Ghidra exports no longer fail the gate. |
| Phase 0.4 switch table | implemented and gated 2026-11-26 | assistant | `hle-os` has a `NativeBody` registry; new `native-hooks` crate registers `fBase_c::__ct` (0x80161AD0) from `nsmbw` when `NSMBW_RUST_BODIES` is set. `1` = lockstep (original runs, body validated on a shadow). `accounted` = body runs on live pre-call state, exact CPU/cycles taken from the original run on the shadow, RAM compared outside the stack/scratch windows and the original's RAM restored on any difference. `direct` is unaccounted and still desynchronizes. `ShadowRuntime` resolves REL addresses from shadow RAM; `NSMBW_RUST_BODIES_VALIDATE` sets the call budget (default 16). Seeded 1-1 in both modes: 98/98 hashes, 13,194,609,027 instructions, 740,425 memcpy calls / 16,106,843 bytes, **16/16 bodies validated, 0 failed**. |
| Phase 1.4 first gated slice | verified 2026-11-26 | assistant | Two `dActor_c` frame hooks converted to `DActor` views and registered: `preExecute` (`0x800643F0`: mDisableMask 0x38E, mKillIfDisabled 0x366, mRideCheck 0x2DC) and `preDraw` (`0x80064540`: mKind 0x38C, mEatState 0x36C, mActorProperties 0x120). `NSMBW_RUST_BODIES_SET` isolates one address. Lockstep 1-1 per body: 98/98 hashes, 13,194,609,027 instructions, **256/256 validated, 0 failed** for each. The combined run surfaced a real defect in `fBase_c::__ct`: one call differs at `0x80359939` (tree/global write), so that body needs fixing before it can be switched. |
