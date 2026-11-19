# Wave 156 — `d_enemiesNP` closure audit (read-only)

Answer to one question: **after wave 155, which `d_enemiesNP` functions are still
untranslated, and why?**

Read-only lane. No Rust file, ledger CSV or existing document was modified; the
only file written is this report. `tools/merge-ledger.mjs`,
`tools/check-lane-csv.mjs` and `scripts/check.ps1` were not run.

## Snapshot

| item | value |
| --- | --- |
| `src/game/enemies.rs` | sha256 `cf8207f35b471af42284c025f9a526d776b95c378f2c5efb836df5535e32893a`, 200,624 lines, mtime `2026-11-13 09:58:18` |
| `ledger/functions.csv` | 3,547 `game::enemies` rows, mtime `2026-11-13 09:41:57` (unchanged all audit) |
| `work/ghidra/rel-d_enemiesNP/export/functions.tsv` | 3,555 rows (+1 header) |
| raw stream | `work/rels/elf/d_enemiesNP.elf`, file offset = `address - 0x80A00000 + 0x1000`, sha256 `d1c48633795029606047f7061fa7162699da5637dc705df1b3d4fca6fe8de564` |

**The module is a moving target.** `enemies.rs` was rewritten twice while this
audit ran: 200,110 lines / 3,476 `fn_<addr>` bodies at the start, 200,351 at
09:54:02, 200,624 at 09:58:18. Two of the eight wave-155 skips were implemented by
other lanes during that window (`0x80A22820` and `0x80A10880`, both still
**unledgered**). All figures below are the 09:58:18 snapshot; the wave-155
boundary set is kept in [Closed during the audit](#closed-during-the-audit), so
the "after wave 155" question is still answered exactly.

## How the 3,555 TSV rows divide

Every TSV row is accounted for, exactly once, at 09:58:18:

| set | rows | notes |
| --- | ---: | --- |
| `fn_<addr>` bodies in `enemies.rs` | 3,478 | 3,476 ledgered + 2 unledgered (`fn_80a22820`, `fn_80a10880`) |
| ledger `game::enemies` rows with a semantic name, implemented under that name | 71 | all 71 are `translated`; e.g. `entry`, `register_buku_states`, `alloc_buku_actor` |
| **no body, no ledger row** | **6** | the subject of this report |
| total | 3,555 | |

Cross-checks:

- `ledger/functions.csv` claims **nothing** untranslated: of its 3,547 rows, 3,476
  are named `fn_<addr>` and **0** of those lack a body; the other 71 are renamed
  rows and all 71 names resolve to a real `fn`/`const` definition in
  `enemies.rs` (checked one by one — none is a doc-comment-only mention).
- 0 ledger addresses are absent from the TSV. The only gaps are the two new
  bodies with no ledger row.
- The `partial` status (93 rows) is a confidence label, not a missing body: every
  one of those addresses has an implementation.
- Coordinator items: **`fn_80a22820` and `fn_80a10880` have bodies but no
  `ledger/functions.csv` row.** Not fixed here — read-only lane.

## The six remaining bodies

Sizes are the TSV `size` column. `instr` is `size / 4`; all 2,746 instructions
decode to known PowerPC32/Gekko opcodes with **0 undecoded words**.

| # | address | TSV size | instr | calls | frame | what the body does | reconstructible now | blocker |
| ---: | --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| 1 | `0x80A3A4A0` | 2,620 | 655 | 30 | 544 | Fire Snake 150-segment body manager | **yes** | none — needs a dedicated raw walk |
| 2 | `0x80A3DEA0` | 2,940 | 735 | 39 | 112 | 16-case constructor / state dispatcher | **yes** | none — needs a dedicated raw walk |
| 3 | `0x80A7BEC0` | 1,464 | 366 | 40 | 160 | walker execute, three clip sequences | **yes** | none — needs a full raw walk |
| 4 | `0x80A92F40` | 1,172 | 293 | 23 | 96 | cloud-family frame step with player tracking | **yes** | none — ordinary lane |
| 5 | `0x80A9B3C0` | 1,196 | 299 | 14 | 128 | `killer_rot_houdai` per-slot state step | **yes** | none — ordinary lane |
| 6 | `0x80B04E90` | 1,592 | 398 | 8 | 64 | rail / chain direction-byte buffer builder | **yes** | none — needs a full raw walk |

Total 10,984 bytes. None of the six is blocked. Every blocker previously recorded
in `COVERAGE_WAVE115..155`, `ANALYSIS_WAVE127_ENEMIES.md` and the module header
was re-tested against the raw stream and either does not hold or is a
size/effort statement rather than missing evidence. See
[Blockers checked and dismissed](#blockers-checked-and-dismissed).

### `0x80A3A4A0` — Fire Snake 150-segment body manager (2,620 B)

Export: `work/ghidra/rel-d_enemiesNP/export/80a3a4a0_FUN_80a3a4a0.c` (950 lines).

A 544-byte frame that saves `f14..f31` as 18 `psq_st` pairs, calls `0x802DCD40`,
then runs two passes:

1. a four-player pass over `+0x7AF0[4]` gated by five `0x80056CF0` state probes
   (`0x53`, `0x2B`, `0xC`, `0x2E`, `0x29`), each arm running `0x8015F140` with a
   different constant quartet; the order counter `+0x7B04` advances while
   `(< 0x2F) && (< 500) && ((0x8042A200 & 3) == 0)`;
2. a 150-iteration (`0x96`) loop over the segment state at `+0x2CC0`, whose arms
   are `0`/`1` (entry), `2`/`3` (body build), `10` (launch). The build writes
   positions `+0x4430`, colours `+0x1D20`/`+0x24F0`, mesh index `+0x7310` and
   scale `+0x5B0`, and runs a nearest-of-four-player test (`0x802BE470` distance
   against the `+0x7AF0` threshold) before copying `+0x82DC..`/`+0x5BA0..` for
   the launch. The tail commits with `0x801C0BD0`/`0x8016A170` after segment
   `0x95`.

Why it is reconstructible: 655 standard instructions, 0 undecoded; the 40 `psq`
words are 18 `f14..f31` save/restore pairs (`W=1`, `I=0`) plus four body
single-element loads; every constant is a readable float in the image. This is a
size problem, not an evidence problem.

### `0x80A3DEA0` — 16-case constructor / state dispatcher (2,940 B)

Export: `.../80a3dea0_FUN_80a3dea0.c` (417 lines).

Copies the home position and decodes the kind word at `this+4`
(`>>0x14 & 0xF` → `+0x718`, `& 0xF` → `+0xA7C`, `>>0x18` → `+0x71C`,
`>>0x10 & 0xF` → `+0x348`, `>>8 & 0xF` → `+0xA80`/`+0xA95`), sets the `+0xAA4`/
`+0xAB4` flags, calls `0x80064680(0x178, this)`, walks the `0x801625B0`/
`0x801625D0` chain to offset the position by `0x80B2C540`, snaps the ground with
`+0xB4 = 0x80B2C548`/`0x80B2C544` by `+0x38F`, swaps the `+0xA4C` owner with
`0x80161800`/`0x80161750`, builds the frame with `0x80014820`/`0x800146B0`,
installs `FUN_80a3f360` at `+0x6D4`/`+0x578`, probes with `0x80097700` into
`+0x45D`, and switches on `+0xA7C` across 16 cases that select state records
(`&DAT_80b706e8`, `0x80353930`, `0x803537f0`, `0x80353830`..`0x80353930`,
`&DAT_80b70828`, `&DAT_80b707A8`, `&DAT_80b70868`, `&DAT_80b70968`,
`&DAT_80b709A8`, `&DAT_80b709E8`). It ends by setting `+0x318`/`+0x31C` speed
and `+0x320` scale from `0x80B2C558..0x80B2C564`.

The jump table is in the shipped artifact and complete: 16 words at `0x80B3EE94`
= `80a3e20c 80a3e734 80a3e768 80a3e988 80a3e7a8 80a3e978 80a3e944 80a3e7ec
80a3e67c 80a3e844 80a3e8a8 80a3e8cc 80a3e910 80a3e20c 80a3e65c 80a3e4a4` — every
target lies inside the 2,940-byte body.

### `0x80A7BEC0` — walker execute (1,464 B)

Export: `.../80a7bec0_FUN_80a7bec0.c` (295 lines).

`0x802DCD54` + `FUN_80a79df0`, then the animation state machine over the
`+0xE9A`/`+0xEA0`/`+0xEA1`/`+0xEA2`/`+0xEA3`/`+0xEA6`/`+0xEA8` bytes. The clip
slot is built from three stride tables over `+0x348`, `+0xEA1`, `+0xEA2`,
`+0xEA3` (`0x24`/`0x12`/`6`/`2`), i.e. `walk_ed_BR_m` at `0x80B2DE5C` and
`walk_st_AR_m` at `0x80B2DE58`. Landing fires the smoke effect
`0x8016C890(s_Wm_en_landsmoke_80b4c8a4, 0, &pos, 0, &0x80B73158)`; frame advance
goes through `0x80168080`/`0x80169020`/`0x80168FE0`/`0x80169F80`/`0x801680E0`; the
rest delegates to translated siblings `FUN_80a79cc0`, `FUN_80a7a030`,
`FUN_80a7a170`, `FUN_80a7a2c0`, `FUN_80a7a360`, `FUN_80a7a3c0`, `FUN_80a7ab90`,
`FUN_80a7ae80`.

Both name tables are present in the image (read: `0x80B2DE58` →
`0x80B4C63C 0x80B4C64C 0x80B4C63C 0x80B4C65C ...`, `0x80B2DE5C` →
`0x80B4C64C 0x80B4C63C 0x80B4C65C ...`). The `GQR0` blocks are the four
`f28..f31` save/restore pairs at `0x80A7BED4..0x80A7BEEC` / `0x80A7C444..0x80A7C45C`.

### `0x80A92F40` — cloud-family frame step with player tracking (1,172 B)

Export: `.../80a92f40_FUN_80a92f40.c` (201 lines). **No `psq` at all, no `unaff_`
at all** — the cleanest export of the six.

`0x802DCD74`; `+0x1C3` (`0x70C`) chooses `0x8005F900` vs `0x8005FA60` on the
`+0x726` byte; no player → `setState(&0x80B74038)`; otherwise `FUN_80a922c0`, and
when the player's `+0x1008` equals `*this` it clamps `+0xDC`/`+0xE0`/`+0xE4`
against `0x80B2EAE0`/`0x80B2EA7C`, polls `0x80070CB0`/`0x80070720` (slope 3 snaps
y from the player with the `0x43300000` double idiom), copies positions with
`0x8004A7B0`, applies the `0x60`/`2`/`0x80000`/`0x100000` player flags, selects
speed/mode from `0x80B2EB20..0x80B2EB48`, sets the texture with
`0x8015F340(+0x102, table, 0x200/0x400/0x800)`, runs the three base updates and
clamps against `0x80429E00+0x38/+0x3C/+0x40/+0x44`, then the virtual
`(+0x548)->+0x1C`.

The three unledgered `main.dol` helpers (`0x8004A7B0`, `0x80056CF0`, `0x80064D10`)
are called by address exactly as they already are elsewhere in `enemies.rs`; that
is a naming gap, not a reconstruction blocker.

### `0x80A9B3C0` — `killer_rot_houdai` per-slot state step (1,196 B)

Export: `.../80a9b3c0_FUN_80a9b3c0.c` (186 lines). **No `psq` at all.**

`0x802DCD70` returns the actor base in the high word and the slot index in the low
word, so `slot` can simply be the second parameter. Stage byte `+0xC88` (0 = init,
`0xFF` = done), slot flag mask `+0xC80`, angle array `+0xC38`, timers
`+0xC50`/`+0xC48`, full-angle table `0x80B2EFDC`. The armed arm spawns through
`FUN_80a9af80` and `FUN_80aaa360(&pos, flag<<4|2|slot<<8, &out, 0x20)`, stores the
handle at `+0xA04 + slot*0x20 + idx*8 + flag*4`, plays `0x80198C30(0x80429D60,
0x1BD, &vec, 0)`, sets the effect `0x80168FE0(0x80B2EFE4, +0x744 + slot*0x38)` and
marks `+0xC58 + slot`. The neighbour arm scans slots ±1 with `0x802DCBC0` +
`countLeadingZeros` and calls `FUN_80a9b1b0` with the descriptor triples at
`0x80B51CC8..0x80B51CF4`, whose callback is the local `0x80A9B870` (verified in
the image).

### `0x80B04E90` — rail / chain direction-byte buffer builder (1,592 B)

Export: `.../80b04e90_FUN_80b04e90.c` (242 lines). **No `psq` at all.**

`0x802DCD5C`; `0x800D90C0(word >> 4 & 0xF)` selects a `0x3B0`-stride record;
`p = base + (word >> 8 & 0xFF0) + *(rec+0x3C) + u16*0x10`. Pass 1 accumulates
`|(p[8]>>4)-(p[0]>>4)| + |(p[9]>>4)-(p[1]>>4)|` over `count-1` segments with
`0x802DE514` into `+0x2B20`; `0x802B9250(sum+2)` allocates the buffer into
`+0x2B1C`. Pass 2 writes one direction-byte pair per segment (x code `0`/`3`/`4`
for zero/negative/positive, y code `0`/`1`/`2`), with two 8-at-a-time unrolled
fill arms, then two NUL terminators. `0x802DCDA8` returns.

The only care needed is the two unrolled fill arms — plain run-length fills, the
same shape as other already-translated fill helpers. Its caller already keeps it
by address: `fn_80b04a30` does `call1(0x80B0_4E90, actor)`.

## Blockers checked and dismissed

Each recorded blocker was re-tested against `work/rels/elf/d_enemiesNP.elf`. None
of them survives.

1. **"`psq` against GQR4, programmed outside this REL"** (`ANALYSIS_WAVE127_ENEMIES.md:192`,
   for `0x80A3A4A0`). False. All 40 `psq` words in the body have `I = 0` (GQR0);
   there is not one GQR3/GQR4 reference. 36 are the `f14..f31` save/restore pairs
   with `W = 1` (single-float store), and the four body loads at `0x80A3A954`,
   `0x80A3A9B0`, `0x80A3A9CC`, `0x80A3A9EC` are `psq_l f0, d(r3), 0, 0` where only
   `f0` is consumed.
2. **"Loop-carried `r0` overwritten by the sprite-byte packer"**
   (`COVERAGE_WAVE141_ENEMIES.md:186`, for `0x80A3A4A0`). Does not hold. The raw at
   `0x80A3A740..0x80A3A750` is `lwz r3,0x2CC0(r26)` / `addi r0,r3,-2` /
   `cmpli 0,r0,1` / `bc 4,1,0x80A3A930`: the dispatch value is a fresh load from
   the actor field, and the export's `if (state - 2U < 2)` is faithful.
3. **"`unaff_GQR0` paired-single damage"** (module header line 90992; also
   `COVERAGE_WAVE152/153/154/155`). This is a decompiler artefact of `psq_st`/
   `psq_l` with `W = 1, I = 0`, i.e. a single-float spill. Evidence: (a) in all
   six remaining exports the only `unaff_*` registers are GQR0 — the `unaff_`
   count equals the `GQRn` count in each of the three files that has any;
   (b) 196 of the 201 exports in this module that contain `unaff_GQR0` are already
   translated; (c) every `psq` site in the six bodies is a prologue/epilogue
   `f14..f31` save/restore except the four single-element loads above.
4. **"Truncated export"** (`ANALYSIS_WAVE127_ENEMIES.md:192-193`). The two exports
   are 950 and 417 lines and cover every case; the 22 "removing unreachable block"
   warnings in `0x80A3A4A0` all fall inside the two `psq` ranges
   (`0x80A3AE30..0x80A3AEBC`, `0x80A3A954..0x80A3A9EC`), and the single warning in
   `0x80A3DEA0` is its own `0x80A3EA00` `f31` restore.
5. **"Export prints a callee with no arguments"** (`ANALYSIS_WAVE127_ENEMIES.md:120`
   and elsewhere). Not damage. Every one of the six starts
   `stwu r1,-N(r1)` / `mflr r0` / `stw r0,N+4(r1)` / `addi r11,r1,N-16` /
   `<spill>` / `bl 0x802DCD{40,54,5C,70}` and never writes `r3` before that call —
   these are zero-argument member functions, and `void f(void)` is the correct
   signature.
6. **"`0x80A9B3C0`: `GQR0` plus a 32-word table copy"** (module header line
   141164). Stale. The body contains zero `psq` instructions and zero GQR
   references; the nearby `0x80A969C0`/`0x80A96DC0` diagnosis does not apply.
7. **Size, not ambiguity.** The wave-155 note ("the rest are size, not ambiguity")
   is confirmed: 0 of 2,746 instructions fail to decode, every referenced data
   table is present in the shipped artifact (jump table 16/16, three name arrays,
   two clip tables, band constants at `0x80B2EFDC`, descriptor table at
   `0x80B51CC8`), and every callee is either a `main.dol` address this file
   already calls by address or a translated sibling.

## Closed during the audit

These are the two of the eight wave-155 skips that other lanes implemented while
this audit ran. Both are still **unledgered** (`ledger/functions.csv` has no
`game::enemies,<addr>` row for either).

### `0x80A22820` — `staffRollBlock` model/animation binder (700 B), closed 09:54

Wave 155 called it "the only one where the remaining doubt is a single arithmetic
idiom". The landed body resolves exactly that idiom literally from the raw —
`rlwinm 28,0,3` / `rlwinm 1,31,31` / `subf` / `rlwinm 4,0,31` / `add` /
`xoris 0x8000` / `lfd` / `fsubs`, i.e. `16 * (low nibble) - 15 * bit30`, plus the
`srawi 4`/`addze` signed divide for the variant index — and records "all 175
instructions decode".

Independent check from this lane: the raw seed sequence at
`0x80A22880..0x80A228C4` is exactly that. The export's
`(iVar4 * 0x10 | ...) ^ 0x80000000` rendering was the misleading part, not the
instruction stream — the same failure mode (`psq` spill noise plus a mis-typed
fixed-point→double sequence) that the six remaining exports show.

### `0x80A10880` — segmented-trail actor frame handler (940 B), closed 09:58

This was the body wave 152 "deliberately left out" and waves 153/154/155 carried
forward. The landed body confirms this lane's reading of the export field by
field: `bl 0x802DCD74`, the `(+0x544)->+0x2C` slot, the `0x80B35230` per-side
speed halved by `0x80B2AE58`, the `0x80072BC0`/`0x8006FA40` pair, the
`+0x5C4..+0x5CC` history copy, `fn_80a10410`, the `0x801611C0` heading with the
`+0x348` negation and `0x800B7510` commit, the `0x80075270`/`0x80075560` ground
pair with `+0x934`/`+0x935`, the `0x800929F0`/`0x80198C30`/`0x80078410` landing
triple, and the `+0x540`/`+0x238` node loop with the `+0x928`/`+0x92C` `0x1C`-stride
ring. It records "all 235 instructions decode".

The wave-152 claim that "a port would need the whole `0x1C` record layout proved
first" was already satisfiable at wave 156: the layout is exercised by the
translated `fn_80a10410` in the same file (same `+0x928`/`+0x92C` pair, same
`0x1C` stride, same field offsets).

Together these two closures demonstrate, on the two bodies whose blockers were
most often re-quoted, that the module's remaining work is transcription effort
against a fully decodable stream rather than missing evidence.

## Conclusion

**Yes — six bodies remain (`0x80A3A4A0`, `0x80A3DEA0`, `0x80A7BEC0`,
`0x80A92F40`, `0x80A9B3C0`, `0x80B04E90`, 10,984 bytes) and every one of them is
honestly reconstructible from the shipped artifact; none is blocked by missing
evidence.**

Practical split for the coordinator:

- ordinary lane budget — `0x80A92F40` (293 instructions, cleanest export),
  `0x80A9B3C0` (299);
- dedicated full raw walk — `0x80A7BEC0` (366), `0x80B04E90` (398),
  `0x80A3A4A0` (655), `0x80A3DEA0` (735).

Deferred, not blocked: the only genuinely open question in this module after
wave 155 is budget. The only ledger defects are the two unledgered bodies
(`fn_80a22820`, `fn_80a10880`). Because other lanes were closing rows during this
audit, re-run the TSV/Rust diff before starting a wave — the six above were
present at 09:58:18 and may shrink again.

## What this lane did not do

- No Rust edit, no ledger write, no merge, no `check-lane-csv`, no
  `scripts/check.ps1`.
- No change to any existing document; the stale notes cited above (module header
  line 141164, `ANALYSIS_WAVE127_ENEMIES.md`, the wave-141/152/155 coverage pages)
  are left as written and corrected only here.
