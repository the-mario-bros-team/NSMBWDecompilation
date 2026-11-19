# Coverage — `0x80260000..0x802ED9E0` (`r80260_802ed`)

## How to read this file

This is the status page for one **address lane** of the `main.dol`
reconstruction. It answers three questions:

1. **What is in this range?** — *Range inventory*: how many function entries
   exist, how many are eligible, and how they are distributed.
2. **What did this pass translate, and where did it go?** — *Translated
   functions*: one row per function with address, Rust name, plain-English role,
   status, confidence, destination module and the Ghidra evidence file.
3. **What should the next pass do?** — *Promotion status*, *Clusters*,
   *Next targets* and *Uncertainties*, so the next agent neither redoes this
   work nor over-trusts it.

Column meanings:

- **Status** — `translated` = full control flow and every call accounted for;
  `partial` = entry/exit and major effects, body marked `// partial:`; `stub` =
  address only.
- **Confidence** — strength of the *evidence*, not code quality:
  `high` = the bytes/flow are explicit and cross-checked; `medium` = the flow is
  explicit but a role, table or API identity is inferred; `low` = mostly
  guesswork.
- **Evidence** — path under `work/ghidra/main-gb/export/`; that C file is the
  source of truth.

**Promotion status: all 20 translated functions have been promoted** into
semantic modules — `src/resource/archive.rs` (5), `src/game/player.rs` (13) and
`src/audio/seq.rs` (2). The staging file `src/subsystems/r80260_802ed.rs` is now
a tombstone that records the move and holds no functions. The machine-readable
ledger is `ledger/agents/r80260_802ed.csv`, whose rows now name the destination
module and file. Addresses are **virtual** addresses of the unrelocated
`main.dol` (image base `0x80004000`), not file offsets. To re-read
the shipped bytes: the main text section is file offset `0x27C0` for virtual
address `0x80006780`, so `file = 0x27C0 + (addr - 0x80006780)`.

Inline markers in the Rust (see `docs/RUST_STYLE.md`): `// observed:` = visible
in the Ghidra output, `// inferred:` = role guessed from callers/shape,
`// unknown:` = unresolved, `// partial:` = deliberately incomplete.

## Range inventory

Source: `work/ghidra/main-gb/export/functions.tsv`, filtered by **entry** address
(`0x80260000 <= addr < 0x802ED9E0`), after removing the two engine-lane skips
listed in `docs/LANES.md` row 15.

| Metric | Count |
|---|---:|
| Function entries in range | 1942 |
| Entries with size `< 16` bytes (**excluded**, address-only thunks/jump tables) | 264 |
| Eligible entries (size `>= 16`) | 1678 |
| Translated by this pass | 20 |
| Promoted to semantic modules | 20 |
| Still staged (translated, not promoted) | 0 |
| Coverage of eligible entries | 1.2 % |

Distribution of entries by 64 KiB block:

| Block | Entries |
|---|---:|
| `0x80260000..0x8026FFFF` | 260 |
| `0x80270000..0x8027FFFF` | 276 |
| `0x80280000..0x8028FFFF` | 158 |
| `0x80290000..0x8029FFFF` | 85 |
| `0x802A0000..0x802AFFFF` | 102 |
| `0x802B0000..0x802BFFFF` | 306 |
| `0x802C0000..0x802CFFFF` | 198 |
| `0x802D0000..0x802DFFFF` | 354 |
| `0x802E0000..0x802ED9DF` | 203 |
| **Total** | **1942** |

Skips: `0x802E9300` (148 bytes) and `0x802E9398` (144 bytes) belong to the
engine-startup lane and are **not** translated here.

### Excluded `< 16`-byte entries

These are address-only entries: 4-, 8- and 12-byte veneers, jump/branch thunks
and shared-`blr` helpers. A translation would only record a jump target, so they
are not translated. The ones adjacent to this pass's cluster are:

`8026F620, 8026F630, 80270100, 80270110, 802756D0, 80275700, 80275710,
80275720, 80275760..80275840, 80275ED0*, 802DCD40..802DCD70,
802DCD8C..802DCDBC`

(`*` = called by translated code but still an address-only helper; see
*Uncertainties*.) The remaining 214 addresses are listed exhaustively in
`functions.tsv`; most sit in the `0x802B..0x802D` library region that this pass
did not classify.

### The `0x802DCD40` save/restore family

The `0x802DCD40..0x802DCDBC` entries deserve a note because they look like 26
four-byte functions and are not:

```text
0x802DCD40  stw r14,-0x48(r11) ... stw r31,-4(r11) ; blr
0x802DCD6C  stw r25,-0x1C(r11) ... stw r31,-4(r11) ; blr
0x802DCD8C  lwz r14,-0x48(r11) ... lwz r31,-4(r11) ; blr
0x802DCDB8  lwz r25,-0x1C(r11) ... lwz r31,-4(r11) ; blr
```

Each entry point falls through to the shared `blr`, so a function can save or
restore exactly the callee-saved registers it needs. Callers enter at the lowest
register they clobber, e.g. `bl 0x802DCD6C` saves `r25..r31`. Ghidra mis-splits
these into 4-byte functions and models each `bl` as returning a 64-bit value in
`r3:r4`; that is why several C files in this cluster appear to unpack a `u64`
after a call. The Rust reads the incoming arguments directly and documents the
omitted spill at each site.

## Translated functions

| Address | Rust | Role (plain English) | Status | Confidence | Promoted to | Evidence |
|---|---|---|---|---|---|---|
| `0x8026F2D0` | `fn_8026f2d0` | decode one packed value (byte / 16-bit / VLQ / random range / table index) | translated | high | `resource/archive.rs` | `8026f2d0_FUN_8026f2d0.c` |
| `0x8026F770` | `fn_8026f770` | open archive file + normalise its path | translated | high | `resource/archive.rs` | `8026f770_FUN_8026f770.c` |
| `0x8026F930` | `fn_8026f930` | close archive file + reset path | translated | high | `resource/archive.rs` | `8026f930_FUN_8026f930.c` |
| `0x8026FC50` | `fn_8026fc50` | read the index block (`+0x124` @ `+0x120`) | translated | high | `resource/archive.rs` | `8026fc50_FUN_8026fc50.c` |
| `0x8026FDB0` | `fn_8026fdb0` | stream read, clamped to declared length | translated | medium | `resource/archive.rs` | `8026fdb0_FUN_8026fdb0.c` |
| `0x80270000` | `fn_80270000` | reserve a 32-byte-aligned pool window | translated | high | `game/player.rs` | `80270000_FUN_80270000.c` |
| `0x80270060` | `fn_80270060` | flush pool range + rewind cursor | translated | medium | `game/player.rs` | `80270060_FUN_80270060.c` |
| `0x80270120` | `fn_80270120` | construct one player slot (two `OSAlarm`s) | translated | high | `game/player.rs` | `80270120_FUN_80270120.c` |
| `0x802701A0` | `fn_802701a0` | start slot: cancel alarms, notify, request 1 | translated | high | `game/player.rs` | `802701a0_FUN_802701a0.c` |
| `0x80270260` | `fn_80270260` | stop slot: cancel alarms, notify, request 3 | translated | high | `game/player.rs` | `80270260_FUN_80270260.c` |
| `0x80270330` | `fn_80270330` | dispatch pending request 1/2/3 | translated | high | `game/player.rs` | `80270330_FUN_80270330.c` |
| `0x80270410` | `fn_80270410` | per-frame slot update + alarm edges | **partial** | medium | `game/player.rs` | `80270410_FUN_80270410.c` |
| `0x80270720` | `fn_80270720` | completion A: apply result, clear busy | translated | high | `game/player.rs` | `80270720_FUN_80270720.c` |
| `0x80270830` | `fn_80270830` | completion B: apply result, clear busy | translated | high | `game/player.rs` | `80270830_FUN_80270830.c` |
| `0x80270A10` | `fn_80270a10` | alarm A handler (lock + get user data) | translated | medium | `game/player.rs` | `80270a10_FUN_80270a10.c` |
| `0x80270A60` | `fn_80270a60` | alarm B handler: cancel A, clear flags | translated | high | `game/player.rs` | `80270a60_FUN_80270a60.c` |
| `0x80270B70` | `fn_80270b70` | slot address = table + index·0xA8 + 0x38 | translated | high | `game/player.rs` | `80270b70_FUN_80270b70.c` |
| `0x80270B80` | `fn_80270b80` | start periodic 0x80270C20 table tick | translated | medium | `game/player.rs` | `80270b80_FUN_80270b80.c` |
| `0x80270D20` | `fn_80270d20` | validate `RSEQ` magic/version, split offsets | translated | high | `audio/seq.rs` | `80270d20_FUN_80270d20.c` |
| `0x80270DA0` | `fn_80270da0` | find named entry in a table | translated | medium | `audio/seq.rs` | `80270da0_FUN_80270da0.c` |

Totals: **19 translated, 1 partial, 0 stubs**; **14 high, 6 medium, 0 low**.
**All 20 are promoted; 0 translated functions remain staged.**

## Clusters

### Cluster A — archive file reader (`0x8026F2D0..0x8026FDB0`)

Five functions that own an "archive reader" object:

| Object offset | Meaning (inferred) |
|---|---|
| `+0x008` | 0x100-byte stored path |
| `+0x108` | data-cursor sub-object (`+0x28` = buffer + 8) |
| `+0x14C` | 0x30-byte file object handed to the runtime file layer |
| `+0x1D8` | "file is open" byte |
| `+0x120` / `+0x124` | index-block offset / size |
| `+0x170` / `+0x174` | two length terms summed into a limit |

`0x8026F770` and `0x8026F930` are the open/close pair; `0x8026FC50` reads the
index block; `0x8026FDB0` reads streamed data with an EOF clamp; `0x8026F2D0`
decodes the compact values those blocks contain. `0x8026F2D0` is the one that
proves the reader carries sequence data: its `width == 4` case is the
"random 16-bit value scaled into `[lo, hi]`" idiom used for randomised
pitch/volume fields.

### Cluster B — four-slot player table (`0x80270000..0x80270B80`)

`0x803FC990` is the table header, `0x803FC9C8` (= header + 0x38) is slot 0, and
the stride is `0xA8`; `0x80270AD0` lazily clears and numbers the four slots.

| Slot offset | Meaning (inferred) |
|---|---|
| `+0x00` | started flag |
| `+0x01` | "busy this update" flag |
| `+0x02` | enable the parameter scan |
| `+0x03` | one-shot pending flag |
| `+0x04` | completion callback enabled |
| `+0x05` | async operation in flight |
| `+0x06` / `+0x07` | alarm A / alarm B armed |
| `+0x08` | state (0/2/4/6 idle, 1/3/5 queued) |
| `+0x0C` / `+0x10` | request / fallback request |
| `+0x14` | 0x20-byte parameter block |
| `+0x34` | slot index 0..=3 (also the async handle) |
| `+0x38` | completion callback |
| `+0x40` / `+0x70` | two `OSAlarm` objects |
| `+0xA0` | `u64` time-base snapshot |

Flow: `fn_802701a0`/`fn_80270260` write a request → `fn_80270330` maps it to
`0x801E1E20(handle, mode, completion)` → the completion lands in
`fn_80270720`/`fn_80270830` (or the un-translated `0x80270920`) → the periodic
`tick` (`0x80270B80` → `0x80270C20`) calls `fn_80270330` for every slot and
`fn_80270410` for slots in state 4.

`fn_80270000`/`fn_80270060` are a separate pool pair used by that machinery to
stage buffers; they are grouped here because they sit in the same function run
and are called from the same update path.

### Cluster C — `RSEQ` header and lookup (`0x80270D20..0x80270DA0`)

`fn_80270d20` validates the `0x52534551` (`"RSEQ"`) magic plus the `>= 0x100`
version word and returns `(base, base + *(u32 *)(base + 0x10))`. `fn_80270da0`
walks the archive name table (count at `+0x08`, relative offsets from `+0x0C`,
entries with `len` at `+0x0C`, name bytes at `+0x10`, payload at `+0x08`) and
returns the payload of the first `strncmp` match. `0x80270D20` is the anchor
that identifies the whole lane as NintendoWare sound/sequence code; see
`docs/GLOSSARY.md` for NW4R.

### Rest of the range (not classified by this pass)

The lane is far larger than this cluster. Block densities from the inventory
table show three very dense areas — `0x802B0000..0x802BFFFF` (306 entries),
`0x802D0000..0x802DFFFF` (354, including the save/restore family) and
`0x802E0000..0x802ED9DF` (203) — and a sparse one at
`0x80290000..0x8029FFFF` (85). This pass deliberately did not guess subsystem
names for them. The only pattern noted while sampling is that `0x8026ABA0` and
`0x8026B6D0` both start with an 84-byte size query followed by a 124-byte setup
routine (`0x8026AD50` / `0x8026B880`), which looks like two instantiations of
one class. That is unverified and is listed as a next target, not as a claim.

## Promotion status

The promotion pass moved every translated function out of the range module into
its semantic home. Nothing was copied: the definitions, doc comments and
`# Original` blocks exist only at the destinations.

| Cluster | Functions | Destination file | Ledger `module` |
|---|---:|---|---|
| A — archive file reader + packed values | 5 | `src/resource/archive.rs` | `resource/archive` |
| B — four-slot player table, pool, alarms | 13 | `src/game/player.rs` | `game/player` |
| C — `RSEQ` header + name lookup | 2 | `src/audio/seq.rs` | `audio/seq` |

- `src/subsystems/r80260_802ed.rs` is now a tombstone: a `//!` header with the
  cluster→destination map and no functions.
- `ledger/agents/r80260_802ed.csv` rows carry the destination `module` and
  `rust_file`.
- `// partial:` `fn_80270410` is still partial **at its destination**
  (`game/player`); the reason (unresolved `extraout_r4`) is unchanged.

### What remains staged

**No translated function.** The range module holds none, and the only remaining
work in this lane is translation of the untranslated entries listed under *Next
targets*. For the neighbours of this cluster the intended homes are already
clear:

| Untranslated | Destination it would join |
|---|---|
| `0x80270AD0`, `0x80270C20`, `0x80270920` | `game/player.rs` |
| `0x8026FD00`, `0x8026FB90`, `0x802756E0`, `0x80275CC0`, `0x80275ED0` | `resource/archive.rs` |
| `0x8026ABA0`, `0x8026B6D0`, `0x8026C3C0`, `0x8026D180`, `0x802B..` block | unclassified — must stay in staging until the class is identified |

### Follow-up for integration

The generated ledger view is already current for this lane: `ledger/functions.csv`,
`ledger/SUMMARY.md` and `docs/FUNCTION_INDEX.md` were regenerated after the CSV
update and list the three destination modules (`resource/archive` 5,
`game/player` 13, `audio/seq` 2).

`docs/COVERAGE.md` still has a hand-maintained module table with a single
`r80260_802ed` row; the integration pass should replace that row with the three
destination modules. (Its "Rust files" count also needs refreshing for the whole
tree, not just this lane.)

## Next targets

Ordered by value to this module, not by address.

| Target | Why |
|---|---|
| `0x80270AD0` (152 B) | lazy initialiser of the four-slot table; called by three translated functions and explains `0x803FC9C8`/index stamping |
| `0x80270C20` (248 B) | the periodic tick that drives `fn_80270330`/`fn_80270410`; its C is already understood (4-slot loop, `0x801A2050` parameter fetch) |
| `0x80270920` (232 B) | completion C, the near-duplicate of `fn_80270830` (states 6/3, mode 0) |
| `0x8026FD00` (176 B) | near-duplicate of `fn_8026fc50` using offsets `+0x118`/`+0x11C` |
| `0x8026FB90` (188 B) | archive header loader called by `fn_8026f770` |
| `0x802756E0`, `0x80275CC0`, `0x80275ED0` | path/cursor helpers called by cluster A (`0x80275ED0` is 12 B, below the size floor) |
| `0x8026ABA0`, `0x8026B6D0`, `0x8026C3C0`, `0x8026D180` | look like instantiations of one class (84-byte size query, then a 124-byte setup); identify the class before translating |
| `0x802B0000..0x802BFFFF` | densest unclassified block in the lane |

## Uncertainties

1. **All API identities are inferred.** The image has no symbol table. The
   file-layer names (`0x801D95B0` open, `0x801D9850` close, `0x801D8BD0` seek,
   `0x801D89F0` read, `0x801DAA70` current directory), the interrupt pair
   (`0x801B1140`/`0x801B1180`), the alarm API (`0x801AB2E0`, `0x801AB540`,
   `0x801AB5B0`, `0x801AB640`, `0x801ABA70/80`) and the cache operation
   (`0x801AC4A0`) were named from their call shape and returned values. No
   Nintendo symbol is claimed.
2. **`fn_80270410` is partial for one value.** The C passes `extraout_r4` to
   `0x801AB540` in the falling-edge arm — a register the decompiler could not
   resolve. The callee (`801ab540_FUN_801ab540.c`) never reads that parameter,
   and the Rust passes `0` with a `// partial:` marker.
3. **`fn_80270410` scans 20 words.** The caller fills only `0x28` bytes (10
   words) via `0x801A2050`, yet the scan is 2 × 10 words. The second half reads
   uninitialised stack in the original; reproduced as-is.
4. **Alarm timings.** Alarm A uses `(OS_BUS_CLOCK >> 2) * 0x1E0`, i.e.
   `OSSecondsToTicks(480)` (eight minutes). Alarm B, on the falling edge, uses
   `OSSecondsToTicks(1)`. The long timeout has no visible consumer in this
   cluster: alarm A's handler (`fn_80270a10`) only reads the user data, so why
   the slot holds an eight-minute alarm is still open.
5. **`fn_80270a10` reads and discards the alarm's user data.** That is what the
   bytes do (`0x80270A34`), but it has no visible effect; it may be a debug
   leftover. Not "simplified away".
6. **`fn_80270da0`'s table layout** is inferred from the offsets alone (length
   at `+0x0C`, name at `+0x10`, payload at `+0x08`). A future pass with a real
   `RSEQ`/`RSAR` file can confirm it.
7. **Signedness of the `width == 5` table entry** in `fn_8026f2d0` comes from
   Ghidra typing the pointer `short *`; the sign extension is an inference.
8. **Callee argument recovery.** Ghidra drops one argument at
   `0x80271CE0(param_3)` in `fn_8026f2d0`; the index is still in `r4` at the call
   site. The Rust passes it explicitly and marks the inference inline.
