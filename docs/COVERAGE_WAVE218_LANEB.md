# Wave 218, lane B - `0x800FBE60..0x800FC470` (`engine::dsys`)

Twelve bodies of the progress/save cluster that sits between the course-table
helpers of Wave 217 lane G (`0x800F8770..0x800F8F70`) and the shell-byte family
that Wave 218 lane C takes from `0x800FC4F0` upwards. All twelve are
`translated`; nothing is deferred.

Block file: `work/wave218/laneB_block.rs`
Ledger: `ledger/agents/wave218-laneB.csv`

## Bodies

| address | size (tsv) | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x800FBE60` | 112 | `0x800FBE60..0x800FBED0` (112) | [`fn_800fbe60`] | medium | reset the row, then its seven `0xB0`-byte records |
| `0x800FBED0` | 92 | `0x800FBED0..0x800FBF2C` (92) + 4 pad | [`fn_800fbed0`] | high | row resolve, then one `0x800CE410` record write |
| `0x800FBF30` | 120 | `0x800FBF30..0x800FBFA8` (120) + 8 pad | [`fn_800fbf30`] | high | last set flag of the first eight |
| `0x800FBFB0` | 64 | `0x800FBFB0..0x800FBFF0` (64) | [`fn_800fbfb0`] | high | shell byte behind the flag cursor, and not `7` |
| `0x800FBFF0` | 72 | `0x800FBFF0..0x800FC038` (72) + 8 pad | [`fn_800fbff0`] | high | the inverse predicate, minus shell `7` |
| `0x800FC0A0` | 156 | `0x800FC0A0..0x800FC13C` (156) + 4 pad | [`fn_800fc0a0`] | medium | scan a sheet for stage class `8` |
| `0x800FC140` | 80 | `0x800FC140..0x800FC190` (80) | [`fn_800fc140`] | high | byte query `0x800CE450(row, a, b)` |
| `0x800FC190` | 92 | `0x800FC190..0x800FC1EC` (92) + 4 pad | [`fn_800fc190`] | high | void query `0x800CE430(row, a, b, c)` |
| `0x800FC1F0` | 212 | `0x800FC1F0..0x800FC2C4` (212) + 12 pad | [`fn_800fc1f0`] | high | dominant-axis sector code `0..6` of two points |
| `0x800FC390` | 112 | `0x800FC390..0x800FC400` (112) | [`fn_800fc390`] | high | three times the present-slot count |
| `0x800FC400` | 112 | `0x800FC400..0x800FC470` (112) | [`fn_800fc400`] | high | bank total against three per present slot |
| `0x800FC470` | 4 | `0x800FC470..0x800FC474` (4) | [`fn_800fc470`] | high | `b 0x800FC400` thunk |

Every size was re-checked in `work/dol/main.elf` from the prologue down to the
closing `bclr` and agrees with
`work/ghidra/main-gb/export/functions.tsv`:

* `0x800FBE60`, `0x800FBFB0`, `0x800FC140`, `0x800FC390` and `0x800FC400` end
  exactly on their `bclr` (the next entry begins on the next word).
* `0x800FBED0`, `0x800FC0A0`, `0x800FC190` and `0x800FC1F0` end 4 bytes short of
  the next entry, `0x800FBF30`/`0x800FBFF0` 8 bytes short: alignment zero words
  that the tsv does not count.
* `0x800FC470` has no `bclr` at all - it is the single `4bffff90 b 0x800fc400`
  branch, and its 4 bytes are the whole slot up to `0x800FC474`.

Not part of this lane and left untouched:

* `0x800FC040` (92 bytes, `0x800FC040..0x800FC09C`) is the shell-code select
  that `ledger/functions.csv` already carries as `game::bases` `fn_80770970`
  (the rel-`D` overlay shares the slot), so it is not re-translated here.
* `0x800FC2D0` (152 bytes, `0x800FC2D0..0x800FC368`) and `0x800FC370`
  (20 bytes, the `cntlzw(byte - 8)` shell test) sit between
  [`fn_800fc1f0`] and [`fn_800fc390`] but have **no** `functions.tsv` entry
  and no ledger row yet, so they are outside this lane's twelve. They were read
  in full while checking the extents; the parent wave may want to assign them.
* `0x800FC4F0` and upwards are Wave 218 lane C.

## Byte weight

112 + 92 + 120 + 64 + 72 + 156 + 80 + 92 + 212 + 112 + 112 + 4 = **1,228 bytes**.

## Evidence notes

### `0x800E0380(bank, -1)` at the head of five bodies

`0x800FBE60`, `0x800FBED0`, `0x800FBF30`, `0x800FC140`, `0x800FC190` and
`0x800FC400` all open with `lwz r3,0xffffa9c0(r13)` (`r13 - 0x5640` =
`0x8042A040`) followed by `li r4,-1` / `bl 0x800E0380`; the row handle that comes
back in `r3` is then the first argument of the row query. `0x800FBE60` is the one
place where the handle is dropped (`r3` is rebuilt from `r29` before the next
`bl`), so that call is reproduced with its result ignored.

### `0x800FBE60` - the seven-word key table

The loop is a `do`/`while` over `uVar1 = 0..6` (`cmpli cr0,30,r30,0x7` /
`bc 12,0` back) that walks `&DAT_802f3440` four bytes at a time and calls
`0x800FBED0(param_1, key, 0xB0)`. The seven words are read from the image at run
time instead of being inlined, because nothing in the export or the neighbouring
bodies pins their values. `// inferred:` they are per-record ids of the row.

### `0x800FBF30` - which of the two probes keeps the index

`0x800CE350(row, index, 1)` returns a byte (`rlwinm r0,r3,0,24,31`); the
accumulator starts at `0` and is overwritten every time the probe answers
non-zero, so the function returns the **last** set index in `0..7`, and `0` when
all eight are clear. `0x800FBFB0` then treats that value as an integer cursor
(`cmp cr0,r3,r4`, both signed) against the byte at `r13 - 0x5433`
(`0x8042A24D`, already [`G_SHELL_BYTE_5433_C`]).

### `0x800FC0A0` - the sheet walk

`iVar1 = *(r13 - 0x54F4) + *(*(r13 - 0x54F4) + 0x338C) * 0x16518` is built with
`lis r3,1` / `addi r0,r3,25880` (`0x16518`) and `mullw`; the count `+0x33A0` is
loaded **once**, before the loop, and the record base is `+0x3394` (the same
layout as the Wave 217 lane G bodies). Inside the loop `r3` is left untouched by
`0x800F56B0`'s caller, so `0x800F9530` receives the record pointer
(`record + index * 0x1C0 + 0x18`, i.e. its code string) and `0x800F9840`
receives that call's `r3` unchanged. The class test is `cmpi cr0,3,r3,8`
(signed).

### `0x800FC1F0` - why the ties go to the horizontal branch

The image is `fcmpo` after `frsp(fabs(d))` for each difference, followed by
`cror` and either `beq` or `bne`: `|dx| == T1` jumps **into** the horizontal
branch (`0x800FC22C`), and `|dz| != T1` jumps **out** of it (`0x800FC240`). The
condition is therefore `T1 <= |dx| || T1 <= |dz|`. Inside it,
`bc 4,1` (branch-if-false of the `GT` bit) sends `|dx| <= |dz|` to the `dz` arm,
which compares `dz` with `T2` (`2` when `dz <= T2`, else `3`); the `dx` arm
compares `dx` with `T2` (`5`/`4`). The remaining arm is the vertical one:
`T1 < |dy|` gives `0`/`1` from `dy <= T2`, and the untouched default is `6`.
That the result really is used as a sector code is proven by
[`fn_800f6cd0`], which calls this body to replace a negative mode argument.

### `0x800FC400` - the `subfc`/`adde` comparison

```text
lwz r3,-0x5640(r13) / li r4,-1 / bl 0x800E0380
cmpli cr0,30,r30,0x9  / bc 12,1 -> li r3,0
mr r4,r30 / bl 0x800CE190            ; r31 = total
mr r3,r30 / bl 0x800FC390            ; r3   = needed
srawi r5,r31,31                      ; r5 = total  >> 31 (0 or -1)
rlwinm r4,r3,1,31,31                 ; r4 = needed >> 31 (0 or +1)
subfc r0,r3,r31                      ; CA = total >= needed (unsigned)
adde  r3,r5,r4                       ; r3 = r5 + r4 + CA
```

Ghidra prints the same value in the form
`((int)uVar2 >> 0x1f) + ((uint)(uVar3 <= uVar2) - ((int)uVar3 >> 0x1f))`, because
`rlwinm r4,r3,1,31,31` extracts bit 31 of `needed` and `-(needed >> 31)` is `0`
or `1` for exactly the same inputs; the reconstruction writes the `adde` form.
For the reachable domain (counts come from `0x800CE190` and from `3 * present`
with `present <= 0x2A`) the result is `1` when `total >= needed`, otherwise `0`.

## Verification

`src/`, `Cargo.toml` and `Cargo.lock` were copied to a scratch directory, the
block was appended to the copy's `src/engine/dsys.rs` and `cargo check --lib`
finished with exit code `0` (the same 23 pre-existing `ui::menu` cast warnings as
the unmodified tree, none from `engine::dsys`).

## Deferrals

None. All twelve bodies were read in full from `work/dol/main.elf` and are
reproduced with every branch, mask and argument.
