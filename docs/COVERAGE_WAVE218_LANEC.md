# Wave 218, lane C - `src/engine/dsys.rs`, addresses `0x800FC4F0..0x800FCDB0`

Twelve entries of `work/ghidra/main-gb/export/functions.tsv` in the `0x800F0000`
bucket of the `engine::dsys` ledger module. All twelve are translated; none
deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800FC4F0` | 104 | 104 (`blr` at `0x800FC554`, no padding) | seed -> lookup -> bind with rotated arguments |
| `0x800FC810` | 20 | 20 (`blr` at `0x800FC820`) | current shell byte is `5` |
| `0x800FC830` | 20 | 20 (`blr` at `0x800FC840`) | current shell byte is `7` |
| `0x800FC940` | 36 | 36 (`blr` at `0x800FC960`) | eight-word style table `0x802F3420`, `0x2A` fallback |
| `0x800FC970` | 72 | 72 (`blr` at `0x800FC9B4`) | table code is one of `1`, `2`, `4` |
| `0x800FC9C0` | 44 | 44 (`blr` at `0x800FC9E8`) | pad count `>= 2` |
| `0x800FCA20` | 44 | 44 (`blr` at `0x800FCA48`) | state of `(shell, 0x26)` is `0` |
| `0x800FCA50` | 136 | 136 (`blr` at `0x800FCAD4`) | four-way shell gate |
| `0x800FCAF0` | 88 | 88 (`blr` at `0x800FCB44`) | per-shell variant answer |
| `0x800FCCA0` | 244 | 244 (`blr` at `0x800FCD90`) | `'*'` slot re-roll and style shift |
| `0x800FCDA0` | 4 | 4 (single `b 0x800F7400`, no `blr`) | thunk into the shell-idle predicate |
| `0x800FCDB0` | 12 | 12 (`blr` at `0x800FCDB8`) | clear the `0x8042A164` latch |

No `functions.tsv` size in this lane is a prologue artefact: each was checked
against the prologue and the closing `blr` in `work/dol/main.elf` (or, for
`0x800FCDA0`, against the unconditional branch and the word of padding that
follows it). Bytes translated: **824**.

## What the run is

Three groups sharing one object, the message/shell context at
[`G_TEXT_CONTEXT`] (`0x80429F7C`, already in the module):

1. **Model binding.** `0x800FC4F0` builds a one-word lookup seed with
   `0x80168400`, resolves `param_3` through it with `0x80164F00`, and binds the
   result with `0x80168230`. The last call's arguments are rotated: the
   container is `param_2`, the destination is `param_1`, and `param_1` is the
   *third* word. The body reserves a 12-byte stack slot (`local_18[3]` in the
   export) but only initialises its first word, which the module models as a
   three-word array for fidelity.

2. **Shell queries.** `0x800FC810` / `0x800FC830` are the `byte == 5` and
   `byte == 7` tests on [`G_SHELL_BYTE_5433_C`] (`0x8042A24D`), both written as
   the Metrowerks `cntlzw(x) >> 5` idiom. `0x800FC940` reads the eight-word
   table at `0x802F3420` with a *signed* `cmpi` bound of 8 and a
   `rlwinm r0,r3,2,0,29` index scale; `0x800FC970` feeds that code to
   `0x800FAD60` (the `1/2/4` "seen" test translated in an earlier wave) and
   short-circuits the `0x2A` sentinel. `0x800FCA20`, `0x800FCA50` and
   `0x800FCAF0` combine `0x800FAE40` state probes with the pad count from
   `0x8005FFB0` and the shell byte.

3. **Slot re-roll.** `0x800FCCA0` only acts when `context[+0x395 + index] ==
   '*'` (`0x2A`). It counts the enabled slots with `0x800FCE60`, then draws a
   bounded random ordinal (`0x800B2EE0`) and maps it through `0x800FCF30`
   until the mapped style is new; the loop is a `do`/`while (count > 1)` with an
   early exit on a changed style. The finish shifts the old `+0x395` byte into
   `+0x39F`, or writes `'*'` to both when the scan ran into the context's
   `+0x54` sentinel, clears `+0x3A9`, and calls `0x800FCDB0`, which clears the
   SDA byte at `r13 - 0x551C` (`0x8042A164`).

4. **Thunk.** `0x800FCDA0` is exactly `b 0x800F7400`; its export is filed as
   `800fcda0_thunk_FUN_800f7400.c` and reproduces the callee body because Ghidra
   followed the branch. It is translated as a direct call to
   [`fn_800f7400`], which takes no arguments.

## Notes on the evidence

* `0x800FC970`'s `bl 0x800FC940` has no `mr r3,...` in front of it, so Ghidra
  prints the callee as `FUN_800fc940()`. `r3` is still `param_1` (`mr r31,r3`
  precedes it), so the index is forwarded. The doc block records this.
* `0x800FCCA0`'s `bl 0x800FCE60` likewise receives `param_1` in `r3`; the
  export prints no arguments there either.
* The initial value of the export's `uVar2` is uninitialised; the Rust body
  yields the value directly out of the `loop`, so no initialiser is invented.
* New address constants in this lane are suffixed `_C`:
  `DAT_802F3420_C`, `FN_8005FFB0_C`, `FN_800B2EE0_C`, `FN_800FCE60_C`,
  `FN_800FCF30_C`, `FN_80164F00_C`, `FN_80168230_C` and the SDA byte
  `G_SLOT_LATCH_551C_C`. Existing `FN_80168400`, [`G_TEXT_CONTEXT`],
  [`G_SHELL_BYTE_5433_C`], [`fn_800fad60`], [`fn_800fae40`] and
  [`fn_800f7400`] are reused; nothing is redefined.

## Verification

`cargo check --lib` was run in a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` with this lane's block appended to the copy's
`src/engine/dsys.rs`: exit 0, no warning or error referencing `dsys.rs`.
