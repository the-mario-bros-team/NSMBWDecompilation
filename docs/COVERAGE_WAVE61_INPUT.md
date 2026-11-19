# Coverage — Wave 61 (`runtime::input`, HID-host state block and API builders)

## How to read this file

1. **Summary** gives the before/after totals for the `0x801F0000` band and the
   per-file numbers.
2. **What this pass covers** describes the three shapes of code added: the HID
   state block, the method-table plumbing, and the `0x1700`..`0x170E` request
   builders.
3. The **function table** lists all 30 bodies in address order with the byte
   count re-read from `work/ghidra/main-gb/export/functions.tsv`.
4. **Deliberate skips** records what was left out and why.

Status vocabulary (`docs/READING_GUIDE.md`): `translated` means the whole
control flow and every call in the export are accounted for; all 30 rows are
`translated`. Function names stay `fn_<addr>` because the image has no symbol
table for this layer; the doc comments carry the roles and `// inferred:` /
`// unknown:` markers repeat them at the code.

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/input.rs` | `runtime::input` | 30 | 2,404 | 19 | 11 |

Band inventory (`[0x801F0000, 0x80200000)`):

| Item | Count |
|---|---:|
| Functions in the band | 268 |
| Untranslated before this pass | 163 |
| Selected and translated by this pass | **30** |
| Untranslated in the band after this pass | 133 |

## What this pass covers

The pass fills the layer between the GKI pool / BTA timer code translated in
waves 18, 34 and 50 and the `bta_hh` state machine at `0x801FF970`. Three
shapes of code repeat:

- **HID-host state block (`0x803CDDE0`).** `0x801F9490` clears the whole
  0x4C-byte block with one `memset`, so the reset (`fn_801f8ed0`), the queue
  drain (`fn_801f9a50`) and the idle restore (`fn_801f9b00`) all write the same
  fields: block word `+0`, four configuration bytes `+0x10..+0x13`, the
  callback `0x803CDE00`, the flag byte `0x803CDE08`, the state byte
  `0x803CDE09` (`5` idle / `4` connected, matching `0x801F9490`) and the two
  GKI lists `0x803CDE0C`/`0x803CDE1C` whose 16-bit counts sit at `+8`
  (`0x803CDE14`/`0x803CDE24`), proven by the already-translated
  [`fn_801f78f0`] / [`fn_801f77b0`] list layout.
- **Method-table plumbing (`r13-0x4B00`).** `fn_801f9e10` installs
  `0x802F6D30` in `r13-0x4B00` and the `r13-0x5D70` object in `r13-0x4AFC`.
  Slots `+0`, `+4`, `+8` and `+0xC` are then called with literal argument
  lists by `fn_801f9b70` / `fn_801f9be0` / `fn_801f9b50`; `fn_801f9ef0`
  reverses the bring-up and `fn_801f9f60` rebuilds the queues.
- **API request builders (`0x1700`..`0x170E`).** Same shape as the wave-50 DM
  builders: allocate with the GKI `getbuf` `0x801F6FB0`, zero the buffer,
  write the 16-bit event id at `+0`, fill the payload and post with
  `0x801FA850`. Covered: `0x1700` (`fn_801ff6c4`), `0x1701`
  (`fn_801ff660`), `0x1708` (`fn_801ff778`), `0x170A` (`fn_801ff8b0`),
  `0x170C` (`fn_801ff5a0`), `0x170D` (`fn_801ff628`), `0x170E`
  (`fn_801ff91c`). The demultiplexer `fn_801ffcc4` maps every other id to a
  handle through the `0x803D1B5C` table and calls the state machine; the name
  table `fn_801ffe10` returns the `0x80347Dxx` diagnostic strings.

Cross-layer facts reused: `0x801F6FB0` is the GKI `getbuf`, `0x801FA850` posts
to the HID-host queue, `0x801F7240` frees a payload, `0x801F8760`/`0x801F8730`
are the GKI lock pair, `0x801F78F0` pops a list head, `0x801AB2E0`/
`0x801AB5B0`/`0x801AB640` are the OS alarm init/arm/stop trio already used by
WPAD init, and `r13-0x4AE8` is [`SDA_BTA_TRACE_LEVEL`], read by
[`bta_trace_level`] and now written by `fn_801fa908`.

## Functions

| Address | Function | Bytes | Confidence |
|---|---|---:|---|
| `0x801F62C0` | `fn_801f62c0` | 12 | high |
| `0x801F6A40` | `fn_801f6a40` | 16 | high |
| `0x801F6A50` | `fn_801f6a50` | 16 | high |
| `0x801F6A60` | `fn_801f6a60` | 8 | medium |
| `0x801F7A60` | `fn_801f7a60` | 8 | high |
| `0x801F7A70` | `fn_801f7a70` | 28 | medium |
| `0x801F7A90` | `fn_801f7a90` | 16 | high |
| `0x801F8020` | `fn_801f8020` | 20 | high |
| `0x801F8720` | `fn_801f8720` | 8 | high |
| `0x801F8EC0` | `fn_801f8ec0` | 12 | medium |
| `0x801F8ED0` | `fn_801f8ed0` | 148 | medium |
| `0x801F9A50` | `fn_801f9a50` | 172 | medium |
| `0x801F9B00` | `fn_801f9b00` | 68 | high |
| `0x801F9B50` | `fn_801f9b50` | 32 | medium |
| `0x801F9B70` | `fn_801f9b70` | 100 | medium |
| `0x801F9BE0` | `fn_801f9be0` | 32 | medium |
| `0x801F9C20` | `fn_801f9c20` | 44 | medium |
| `0x801F9E10` | `fn_801f9e10` | 220 | medium |
| `0x801F9EF0` | `fn_801f9ef0` | 52 | high |
| `0x801F9F60` | `fn_801f9f60` | 64 | high |
| `0x801FA908` | `fn_801fa908` | 8 | high |
| `0x801FF5A0` | `fn_801ff5a0` | 136 | high |
| `0x801FF628` | `fn_801ff628` | 56 | high |
| `0x801FF660` | `fn_801ff660` | 100 | high |
| `0x801FF6C4` | `fn_801ff6c4` | 180 | high |
| `0x801FF778` | `fn_801ff778` | 144 | high |
| `0x801FF8B0` | `fn_801ff8b0` | 108 | high |
| `0x801FF91C` | `fn_801ff91c` | 84 | high |
| `0x801FFCC4` | `fn_801ffcc4` | 332 | medium |
| `0x801FFE10` | `fn_801ffe10` | 180 | high |

Totals: **30 translated**, 0 partial, 0 stubs; 19 high / 11 medium / 0 low;
2,404 bytes.

## Corrections and caveats

1. **`fn_801f9e10` polls bare `blr` stubs.** `0x801F8630` and `0x801F8660`
   decompile to `return;` (4-byte thunks, on the wave-34/50 skip list). The
   two spin loops (`start flag`, `0x801FCF30`) still call them with `0` and
   `100`/`2000`, exactly as the export shows; the missing yield is recorded in
   the body comment and in the `# Safety` block.
2. **`fn_801f9b50` indirect arguments.** The export renders the `+0xC` slot
   call with no arguments. The ABI leaves `event` in `r3` and `value` in `r4`,
   so the Rust passes both; confidence stays `medium`.
3. **`fn_801ffcc4` and `0x801FFEC4`.** `0x801FFEC4` is a split-prologue
   function (`FUN_802dcd74`), so its one argument and its byte result are
   recovered from the call site rather than from its own body. The
   demultiplexer is otherwise literal and stays `medium`.
4. **`fn_801f8ed0` block word width.** Ghidra gives no width for `0x803CDDE0`
   (the only other reference is `memset(&DAT_803cdde0, 0, 0x4C)` in
   `0x801F9490`). The Rust uses a 32-bit store and marks the field `unknown:`.
5. **`fn_801f9a50` list counts.** The loop tests read the 16-bit `+8` count of
   each list object, not a word: the layout is fixed by the already-translated
   `fn_801f78f0` / `fn_801f77b0`.
6. **No staging.** All bodies live in `src/runtime/input.rs`; no other Rust
   file, generated doc or `ledger/functions.csv` was touched.

## Deliberate skips

- **`0x801F9D10`–`0x801F9DD0` (7 functions, 20 B each).** These are the seven
  `BTA_TRACE_*` entry points, already called by address from the HID-host
  layer. Each body is `if (r13-0x4AF8 != 0) return; FUN_801f9c50();` and
  forwards *all* incoming registers to the variadic printer `0x801F9C50`; call
  sites use two to six arguments. Ghidra recovers no parameters, so a Rust
  signature would have to invent an arity. Left untranslated.
- **`0x801F8E80` (60 B).** Computes `(handle - 8) - 0x803CDDBA`, a global whose
  only reference in the whole export is this line, so its width cannot be
  proven. Left untranslated rather than guess between a byte and a word.
- **`0x801F9830` (8 B).** Body is a single `return 0`, but `0x801F8820` calls
  it with three arguments and consumes the result as a report length; the
  decompiled body is a split artifact. Left untranslated.
- **`0x801FF808` (144 B).** Uses `FUN_802DCD70` and an `extraout_r4` as a live
  register pair (the same split-frame family as wave 50's `0x801FD0E0`).
  `0x801FFEC4` (316 B) uses the sibling artifact `FUN_802DCD74` and is skipped
  for the same reason; `fn_801ffcc4` and `fn_801f9a50` call it by address.
- **`0x801FF970` (852 B).** A private forwarding stub
  (`unsafe fn fn_801ff970`) already exists in the file for the callers; the
  real state machine is not translated, so no ledger row is claimed.
- **The rest of the band** — BTU bring-up `0x801F0020`–`0x801F35C0`, the
  HID/SDP state handlers `0x801FBB40`–`0x801FD83C`, the subsystem bring-up
  `0x801FAB34`–`0x801FDC34` and the message plumbing
  `0x801FDE58`–`0x801FF308` — remains for later passes.

## Verification

- Every address and byte size in the table was re-read from
  `work/ghidra/main-gb/export/functions.tsv` for this pass (total 2,404 bytes).
- Every evidence file exists at the cited `work/ghidra/` path.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/runtime/input.rs`.
- No address was already in `ledger/functions.csv`; the 163-entry
  untranslated list for the band was derived from it first.
- `cargo check --message-format=short` reports no errors; the only warning in
  this file is the existing-style "direct cast of function item into an
  integer" at the two older call sites (lines 5267 and 10463).
- `node tools/check-lane-csv.mjs wave61` prints `OK` and reports
  `wave61-input.csv: 30 rows`.

## Ledger

Machine-readable rows: `ledger/agents/wave61-input.csv` (30 rows, module
`runtime::input`, address order as the table above, header
`module,address,name,status,confidence,evidence,rust_file`).
