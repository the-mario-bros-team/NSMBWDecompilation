# Reading guide (start here)

This repository reconstructs **New SUPER MARIO BROS. Wii** (SMNE01, build
r37675) as readable Rust. It is a source reconstruction, not a playable port
yet. Nothing here is original Nintendo source; every function is derived from
the shipped binary and cites its evidence.

## The three layers

| Layer | Location | What it is |
|---|---|---|
| Evidence | `work/ghidra/main-gb/export/` | Ghidra C pseudocode + `functions.tsv`; immutable reference |
| Reconstruction | `src/` | Rust, organized by system; the authoritative output |
| Navigation | `docs/`, `ledger/` | architecture, call graphs, coverage, confidence |

Never edit `work/` to make the Rust look nicer. If Rust and C disagree, the C
(and the original bytes) wins until proven otherwise.

## Where to start

1. `docs/ARCHITECTURE.md` — the machine and the boot flow.
2. `docs/BOOT_CHAIN.md` — entry point to `main`, function by function.
3. `src/boot/entry.rs` — the first translated function.
4. `ledger/functions.csv` — address/name/status/confidence/evidence index.
5. `docs/GLOSSARY.md` — Wii/NSMBW vocabulary.

`src/subsystems/r*.rs` are **unclassified extraction ranges** produced by
parallel lanes. They are evidence-organized by address, not by meaning. Do not
start there; use them through the coverage docs and the ledger.

## How to read one function

```rust
/// Copies `len` bytes from `src` to `dst`, then flushes the written range from
/// the data cache and invalidates the corresponding instruction cache lines.
///
/// # Original
/// - Ghidra: `FUN_80004364` at `0x80004364` (668 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80004364_FUN_80004364.c`
///
/// # Confidence
/// high — matches the canonical CodeWarrior `memcpy` shape.
pub unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) { ... }
```

Read the doc comment first, then the `# Original` block to jump to evidence.
Addresses are **virtual** (image base `0x80004000`); they are not file offsets.

## Naming

| Form | Meaning |
|---|---|
| `memcpy`, `memset`, `flush_cache` | role proven by behavior/callers |
| `fn_801b8970` | role not proven; address is the identity |
| `dBase_c`, `nw4hbm::lyt` | recovered RTTI/string name |

## Confidence and status

- **translated** — full control flow, all calls accounted for
- **partial** — entry/exit and major effects, body marked `// partial:`
- **stub** — address/role only, behavior unknown
- **high / medium / low** — strength of evidence, not code quality

## Checks

```powershell
cargo check                  # syntax/type check (no linker needed)
node tools/doc-lint.mjs src  # readability gate: module/function docs + evidence
```

The Windows port lives behind `docs/PORT_WINDOWS.md`; do not mix host code into
the reconstruction modules.

## Legal

Use your own game dump. No game assets or extracted content are redistributed
by this repository.
