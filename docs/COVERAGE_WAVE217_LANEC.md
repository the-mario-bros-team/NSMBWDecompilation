# Wave 217, lane C - `0x800F9270..0x800F99F0` (`engine::dsys`)

The four-character stage-code family: it turns the numeric stage index kept in
ten global edge words into the `"W<world><slot>"` code used by the res/object
loaders, and it parses such a code back into an index. The block sits directly
above Wave 217 lane B's player-sheet run (`0x800F8770..0x800F9078`) and below the
`0x800F9A60` stage-code table builder. All ten bodies are `translated`; the two
`medium` entries are `medium` because of the role of the walked object names, not
because of missing control flow.

Block file: `work/wave217/laneC_block.rs`
Ledger: `ledger/agents/wave217-laneC.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800F9270` | 68 | `0x800F9270..0x800F92B4` (68) | [`fn_800f9270`] | high | copy the four bytes at `+1`, terminate |
| `0x800F92C0` | 68 | `0x800F92C0..0x800F9304` (68) | [`fn_800f92c0`] | high | copy the four bytes at `+5`, terminate |
| `0x800F9310` | 96 | `0x800F9310..0x800F9370` (96) | [`fn_800f9310`] | high | build the `'R'` record (`1 + 4 + 4 + 1`) |
| `0x800F9370` | 436 | `0x800F9370..0x800F9524` (436) | [`fn_800f9370`] | high | format `"W<world><slot>"` from the stage index |
| `0x800F9530` | 232 | `0x800F9530..0x800F9618` (232) | [`fn_800f9530`] | high | parse a `"W.."` code back into an index |
| `0x800F9620` | 268 | `0x800F9620..0x800F972C` (268) | [`fn_800f9620`] | medium | resolve the code from the `cobKinoko*` objects |
| `0x800F9730` | 260 | `0x800F9730..0x800F9834` (260) | [`fn_800f9730`] | medium | sum the trailing name digits, base `25` |
| `0x800F9840` | 228 | `0x800F9840..0x800F9924` (228) | [`fn_800f9840`] | high | classify the index into group `0..10` / `11` |
| `0x800F9970` | 128 | `0x800F9970..0x800F99F0` (128) | [`fn_800f9970`] | high | code kind `0..4` from bytes 0 and 2 |
| `0x800F99F0` | 100 | `0x800F99F0..0x800F9A54` (100) | [`fn_800f99f0`] | high | chain node whose `+4` byte equals the tag |

Every size was re-checked against the prologue and the closing `bclr` in
`work/dol/main.elf` and agrees with
`work/ghidra/main-gb/export/functions.tsv`:

* `0x800F9270`, `0x800F92C0`, `0x800F9310`, `0x800F9370`, `0x800F9530`,
  `0x800F9620`, `0x800F9730`, `0x800F9840`, `0x800F9970` and `0x800F99F0` all
  end exactly on their `bclr`; the tsv counts the `bclr` plus the alignment
  words that follow it up to the next 16-byte boundary (`blr` at
  `0x800F92B0 + 4`, `0x800F9300 + 4`, `0x800F936C + 4`, `0x800F9520 + 4`,
  `0x800F9614 + 4`, `0x800F9728 + 4`, `0x800F9830 + 4`, `0x800F9920 + 4`,
  `0x800F99EC + 4`, `0x800F9A50 + 4`).
* `0x800F9840` and `0x800F9970` are leaves with no stack frame at all: the
  first ends in the shared `li r4,0xB` / `li r4,0xA` tail, the second in a
  chain of six early `bclr` exits.

## Byte weight

68 + 68 + 96 + 436 + 232 + 268 + 260 + 228 + 128 + 100 = **1,884 bytes**.

## Evidence notes

### The global edge words (`r2 - 0x6124 .. - 0x60F8`)

`0x8042CF5C` holds eleven words that the `r2` (float/constant) base addresses
directly: `0, 0, 20, 21, 23, 25, 32, 35, 36, 37, 38, 40` for offsets
`-0x6124, -0x6120, -0x611C, -0x6118, -0x6114, -0x6110, -0x610C, -0x6108,
-0x6104, -0x6100, -0x60FC, -0x60F8`. They are cumulative stage-count edges: the
same word is printed by [`fn_800f9370`] and consumed by [`fn_800f9530`] and
[`fn_800f9840`], and `-0x6120` (value `0`) is the world base of the digit arm.
Every body reads them at run time, so no constant is invented.

### `0x800F9370` - the 11-word `ctr` table

The dispatch is `lwzx` off the word table at `0x8031C0C0` (not `0x8032C0C0`:
`addi r4,r4,-16192` is applied to `lis r4,0x8032`). The table is

```
0x800F93D8 0x800F940C 0x800F9428 0x800F9448 0x800F9464
0x800F9474 0x800F9490 0x800F94F8 0x800F94AC 0x800F94C8 0x800F94E4
```

which is exactly the arm order the block uses. The guard at `0x800F93B8` is
`cmpli cr0,r3,0xA` + `bc 12,1` (branch if **greater**), so classification `11`
takes the `0x800F94F8` default and returns `0`. Ghidra's `switch` labels the arms
in a different textual order but names the same letters.

### `0x800F9530` - the 41-word `ctr` table

Same shape: `0x8031C0EC`, selector `(s8)code[2] - '0'`, guard
`cmpli cr0,r0,0x28` + `bc 12,1`. Entries `0..9` all point at `0x800F958C` (the
digit arm), `0x11/0x13/0x17/0x1B/0x23/0x24/0x27/0x28` are the only other live
entries and every remaining slot points at the `0x800F95FC` exit, which returns
the `-1` preset. The digit arm re-issues the same `0x802E2108(code + 3)` call the
head already made; the block reuses the first result (identical `strlen`
arguments, no side effects).

### `0x802DCD60` / `0x802DCD70` / `0x802DCDAC` / `0x802DCDBC` - save/restore stubs

`0x800F9620` opens with `addi r11,r1,0x30` + `bl 0x802DCD70` and closes with
`bl 0x802DCDBC`; `0x800F9730` uses `0x802DCD60` and `0x802DCDAC`. Both pairs are
the Metrowerks outlined `_savegpr_26` / `_restgpr_26` (and `_22`) fragments: ten
`stw`/`lwz` of the callee-saved GPRs at `r11-relative` slots followed by `bclr`.
They preserve `r3`/`r4`, so Ghidra's `uVar6 = FUN_802dcd70()` and
`uVar1 = FUN_802dcd60()` are really the incoming arguments. That is what fixes
the real signatures of the two bodies:

* `0x800F9620(param_1 = stage, param_2 = dst)` - `r29 = r3 - 25` and
  `r26 = r4 = dst`, then `0x802E19F8(dst, name, 4)`.
* `0x800F9730(param_1 = code)` - `mr r4,r3` right after the stub, so the
  `0x80100530` lookup key is the function's own first argument.

Ghidra types both `void`; `0x800F9730` returns `r25` (`0x800F981C` sets `r3`
before the restore), which `0x800F9530`'s `'K'` arm consumes.

### `0x800F9730` - the `0x1F` clamp

`0x800F97E4..0x800F97F4` is `subfc r0,r3,r30` (`r30 = 0x1F`), `rlwinm` of the
sign bit, `subfe r0,r31,r0` (`r31 = 0`), `andc r0,r3,r0`: the value is added
when `(u32)value <= 0x1F` or when it is negative, and dropped otherwise. For a
signed variable that is exactly `if value <= 0x1F { sum += value }`, which is
what the block writes; the clamp is a `min`-style range test on the digit, not a
separate range guard.

### `0x800F9620` / `0x800F9730` - the three `cobKinoko*` keys

`0x8031C090` holds three name pointers: `0x8031C060 "cobKinokoRed"`,
`0x8031C070 "cobKinoko1up"`, `0x8031C080 "cobKinokoStar"`. Both bodies walk
three keys (`cmpi cr0,22,r22,3` in the inner loop, `cmpi cr0,27,r27,3` over the
key table), which is why the block iterates `slot` over `0..3`. The walked nodes
carry names through the self-relative `+8` offset (the existing
[`node_next_a`] helper models exactly that idiom) and `0x800F9620` also follows
`+0x5C` to the sub-record it copies from.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory
(`%TEMP%\opencode\w217C`), the block was appended to the copy's
`src/engine/dsys.rs` and `cargo check --lib` exits `0` with the same 23
pre-existing warnings as the unmodified tree (none of them from `engine::dsys`).

## Deferrals

None. Every one of the ten bodies was read in full from `work/dol/main.elf`, and
no body needed a guessing step: where the export was silent (the save/restore
stubs, the two jump tables, the `switch` order) the disassembly decided it.
