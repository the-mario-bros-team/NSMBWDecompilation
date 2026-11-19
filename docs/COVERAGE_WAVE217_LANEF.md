# Wave 217, lane F - `src/engine/dsys.rs`, addresses `0x800FADB0..0x800FB270`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` (rows 3392-3401),
all in the `0x800F0000` bucket of the `engine::dsys` ledger module. All ten are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x800FADB0` | 92 | 92 (`blr` at `0x800FAE08`, 4 B padding) | state `1`/`3`, else the `0x100` bit |
| `0x800FAE10` | 44 | 44 (`blr` at `0x800FAE38`, 4 B padding) | state `4` (`cntlzw` fold) |
| `0x800FAE40` | 60 | 60 (`blr` at `0x800FAE78`, 4 B padding) | state in `1..=4` (`orc` range test) |
| `0x800FAE80` | 240 | 240 (`blr` at `0x800FAF6C`, no padding) | state `2`+bit `0x10` or state `4`+bit `0x80` |
| `0x800FAF70` | 204 | 204 (`blr` at `0x800FB038`, 4 B padding) | state `3`, or `0x100` bit + context `3/3/2` |
| `0x800FB040` | 260 | 260 (`blr` at `0x800FB140`, 4 B padding) | per-state arms for `2`/`3`/`4` |
| `0x800FB150` | 80 | 80 (`blr` at `0x800FB19C`) | bit `0x80` |
| `0x800FB1A0` | 80 | 80 (`blr` at `0x800FB1EC`) | bit `0x100` |
| `0x800FB1F0` | 80 | 80 (`blr` at `0x800FB23C`) | bit `0x10` |
| `0x800FB240` | 80 | 80 (`blr` at `0x800FB28C`) | bit `0x20` |

No `functions.tsv` size in this lane is a prologue artefact: each one was
checked against the prologue and the closing `blr` in `work/dol/main.elf`.

Bytes translated: 1220.

## What the run is

A single query cluster over a per-scene progress-flag bank:

* `0x800E0380(*(r13 - 0x5640), -1)` selects the current 0x980-byte block
  (`*(i8*)(bank + 0x26)` names it). `r13 - 0x5640` = `0x8042A040`, added as
  `G_PROGRESS_BANK_F`; it is the first use of that global in this module.
* `0x800CE3A0(record, group, index)` reads
  `record + group * 0xA8 + index * 4 + 0x6C`.
* `0x800CE3C0(record, group, index, mask)` is the boolean bit test of that word
  (`(word & mask) != 0`); the four `0x800FB1x0` wrappers are the tests for
  `0x10`, `0x20`, `0x80` and `0x100` and return the callee's `r3`, although
  Ghidra types them `void`.
* `0x800FA8C0(group, index)` folds the same four bits into the state `1..4`.

Every probe first rejects `group != *(G_TEXT_CONTEXT + 0x50)`, so only the block
the message context points at is queried.

## Notes on the evidence

* `0x800FA8C0`, `0x800FA9C0`, `0x800FAD60`, `0x800CE3A0` and `0x800CE3C0` are
  outside this lane's ten and are called through address constants suffixed
  `_F`. `0x800E0380` already existed in the module as `FN_800E0380` and is
  reused.
* Ghidra prints `0x800FAE10` and `0x800FAE40` as `(void)` and `0x800FAD60` /
  `0x800FADB0` with no parameters. The disassembly shows the argument registers
  are never written before the `bl 0x800FA8C0`, so the caller's arguments reach
  the state probe unchanged; the bodies are modelled with those parameters and
  the doc blocks say so.
* `0x800FAE40` is kept as the original wrapping arithmetic
  `((3 | ~(state-1)) - ((3 - (state-1)) >> 1)) >> 31`; the `X31_412` word is
  `orc r3,r3,r4`. Over the values the probe can return the expression is the
  range test `state in 1..=4`.
* `0x800FAF70` reloads `*(r13 - 0x5704)` after the `0x800FB1A0` call before the
  `+0x60`/`+0x54`/`+0x50` compares; the Rust body reloads it too.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
lane block was appended to the copy's `src/engine/dsys.rs`, and
`cargo check --lib` finished with exit code 0 (only the pre-existing warnings of
other modules). No source file in the repository was modified by this lane.
