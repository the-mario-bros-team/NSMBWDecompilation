# Rust reconstruction style

## Source of truth

- Ghidra C (Gekko_Broadway): `work/ghidra/main-gb/export/<addr>_<name>.c`
- Function index: `work/ghidra/main-gb/export/functions.tsv`
- Evidence is not deleted: every Rust function cites its address and file.

## Rules

1. One Rust module per subsystem; no edits outside your assigned lane.
2. Keep the original control flow. Do not "simplify" away observable writes.
3. `unsafe` is expected; use `crate::platform` helpers (`read_u32`, `write_u32`,
   `call0..call3`) instead of raw pointer arithmetic where possible.
4. Preserve raw addresses in comments; never invent symbol names silently.
   Mark inferences:
   - `// observed:` from the Ghidra output
   - `// inferred:` naming/role guessed from call sites/strings
   - `// unknown:` unresolved
5. No external crates. `#![no_std]` crate.
6. Stub policy: an unresolved call may be `platform::call0(0x80123456)` with an
   `// unknown:` comment. Do not invent behavior.
7. Ledger row per translated function:
   `module,address,name,status,confidence,evidence,rust_file`
   status: `translated` | `partial` | `stub`; confidence: `high|medium|low`.
8. Do not run Ghidra, install tools, or modify `opencode.json`.

## Naming

- Keep `fn_80xxxxxx` when the role is unknown.
- Use a descriptive name only when the evidence is strong (`memcpy`,
  `memset`, `flush_cache`).
- A descriptive name must keep the address in its doc comment.

## Human readability (mandatory)

Someone with no context must be able to open any file and understand it.

1. **Module header** (`//!`): what this module covers, the original address
   range, why it exists, its dependencies, and a "start here" entry point.
2. **Function doc** (`///`): plain-English behavior first, then:
   - `# Original`: Ghidra name, address, size, evidence path
   - `# Arguments` / `# Returns`: meaning, not just types
   - `# Side effects`: memory writes, MMIO, calls with observable effects
   - `# Safety`: pointer/range requirements (`unsafe` functions)
   - `# Confidence`: `high | medium | low` + one-line reason
3. **No unexplained magic.** Every hard-coded address, bitmask or table gets a
   comment naming what it is, or `// unknown: ...`.
4. **Names over addresses in signatures.** Prefer `dst`, `src`, `len`; use
   `raw_0x...` only when the role is genuinely unknown.
5. **Glossary terms** (DOL, REL, U8, BRRES, GX, AX, MEM1, r13) are linked or
   defined at first use; see `docs/GLOSSARY.md`.
6. **Coverage doc** starts with a "How to read this file" section and a table
   (address, name, role, status, confidence, evidence).
7. Do not leave commented-out code or dead experiments. Mark partial functions
   with `// partial:` and what is missing.

If a translation cannot be explained in one plain sentence, it is not done.

