# Wave 220, lane F - `src/resource/archive.rs`, addresses `0x802C6040..0x802C8410`

Ten entries of `work/ghidra/main-gb/export/functions.tsv`, all in the
`0x802C6000` run of the `resource::archive` ledger module. All ten are
translated; none deferred.

| address | tsv size | real extent | body |
|---|---|---|---|
| `0x802C6040` | 60 | 60 (`blr` at `0x802C6078`, 4 B padding) | construct the `0x803502F8` record and run its field initialiser |
| `0x802C65D0` | 108 | 108 (`blr` at `0x802C6638`, 4 B padding) | deleting destructor of the `0x80350318` record |
| `0x802C6640` | 24 | 20 (`blr` at `0x802C6650`, 8 B padding) | set bit 2 of the `+0x04` flag byte |
| `0x802C6BF0` | 116 | 116 (`blr` at `0x802C6C60`, 4 B padding) | copy one 0x36-byte record |
| `0x802C6D60` | 20 | 20 (two tail branches, no `blr`) | tail-dispatch on the record's `+0x00` word |
| `0x802C7190` | 356 | 356 (`blr` at `0x802C72F0`, 4 B padding) | emit the four edges of a screen rectangle |
| `0x802C78F0` | 340 | 340 (`blr` at `0x802C7A40`, 4 B padding) | construct the `0x80350350` screen record |
| `0x802C7C70` | 184 | 184 (`blr` at `0x802C7D24`, 4 B padding) | draw one textured quad from a queried descriptor |
| `0x802C7E30` | 184 | 184 (`blr` at `0x802C7EE4`, 4 B padding) | deleting destructor of the `0x80350360` record |
| `0x802C8410` | 64 | 64 (`blr` at `0x802C844C`, no padding) | null-guarded conditional `operator delete` wrapper |

One `functions.tsv` size in this lane is a prologue artefact: `0x802C6640`
closes with `blr` at `0x802C6650`, so the body is 20 bytes and
`0x802C6654..0x802C6657` is alignment padding; `functions.tsv` records the
padded 24. `0x802C6D60` has no `blr` at all: it ends in two tail branches
(`0x802C6D6C` -> `0x802C6E50`, `0x802C6D70` -> `0x802C6E20`) and the last
instruction ends exactly at `start + 20`. Every remaining entry ends in a `blr`
at `start + size - 4`.

Bytes translated: 1456 (`functions.tsv` weight); 1452 by real extent.

## What the run is

The `0x802C6000` band continues the record half of the overlay whose
immediate-mode GX half is the wave-219 lane in this same file. The ten bodies
fall into four groups.

**Record construction (`0x802C6040`, `0x802C78F0`).** `0x802C6040` writes the
`0x803502F8` vtable and calls the field initialiser at `0x802C6080`, which
copies the four bytes at `r2-0x3E3C` into `+0x04..+0x07`, stores the float at
`r2-0x3CD8` into `+0x08`, `+0x0C`, `+0x10` and `+0x14`, and zeroes `+0x18`
(word) and `+0x1C` (byte). `0x802C78F0` is the bigger screen record: vtable
`0x80350350` at `+0x1C`, extent `0x280 x 0x210`, `+0x0C/+0x0E = 0x20/0x100`,
`+0x14/+0x16 = 0x260/0x340`, then three virtual queries of the manager at
`r13-0x4838` (slots `+0x08`, `+0x08`, `+0x10`) that override the extent halves
and the `+0x04` word, and finally `+0x10 = 3 * 0x801C6BA0(w, h, 6, 0, 0)`,
i.e. three times the texture-buffer size of the record's extent.

**Teardown (`0x802C65D0`, `0x802C7E30`, `0x802C8410`).** All three follow the
same `guard -> optional operator delete` shape. `0x802C65D0` installs the
`0x80350318` vtable and removes the `0x802C64F0` callback from the handler
table word at `+0x0C` through `0x802DCB10`. `0x802C7E30` installs the
`0x80350360` vtable at `+0x08`, then drains a 12-byte-stride member array:
`0x80249AC0` on each entry's `+0x04` word, clear it, and after the loop free
the array with `0x802B9290` and zero `*(this)`. `0x802C8410` only forwards to
`operator delete`.

**Small field helpers (`0x802C6640`, `0x802C6BF0`, `0x802C6D60`).**
`0x802C6640` is the "set once" form of bit 2 in the `+0x04` flag byte
(`rlwinm.` test, early `blr`, otherwise `ori 4`). `0x802C6BF0` copies two
words, eleven singles and one halfword - the raw instructions use
`lfs`/`stfs` for `+0x08..+0x30` where the export prints word copies, so the
copy is typed as float and reproduces the same bits. `0x802C6D60` tests
`*(this)` and tail-branches; `r3` still holds `this`, which the export loses.

**Geometry and drawing (`0x802C7190`, `0x802C7C70`).** `0x802C7190` emits the
four edges of a rectangle from the record's origin `+0x20`/`+0x24`, size
`+0x08`/`+0x0C` and scales `+0x28`/`+0x2C`, plus two SDA float pairs at
`r13-0x47C0..-0x47C4` (`0x8042AEC0..0x8042AEC4`). `0x802C7340`, the body just
after this lane, seeds those four slots with `1.0`, `1.0`, `0.0`, `0.0`, so
they read as a default-neutral scale/offset pair; mode 0 uses the `0.5`/`-0.5`
coefficients at `r2-0x3CB8`/`r2-0x3CA8`, mode 1 the mirrored edge form with
`1.0` and `-1.0` at `r2-0x3CC8`/`r2-0x3CB0`, and any other mode value writes
nothing. `0x802C7C70` queries `0x8023EB30`, and only when it reports a hit
programs the texture attributes with `0x801C6D90` and the coordinate scale
with `0x801C6FF0` (`f3 = r2-0x3C80 = 0.0`).

## Notes on fidelity

* **`__savegprlr_27` artefact (`0x802C7C70`, `0x802C7E30`).** Both bodies open
  with `addi r11,r1,<frame>` / `bl 0x802DCD74` and close with
  `bl 0x802DCDC0`. `0x802DCD74` stores `r27..r31` at `r11-0x14..r11-4` and
  returns without touching `r3`/`r4`. The exports model it as
  `uVar5 = FUN_802dcd74()` returning a `u64`, which invents two arguments and
  swaps them: `0x802C7C70` passes the **second** raw argument to `0x8023EB30`
  (`mr r3,r4` right after the save helper) and the **first** to both GX calls
  (saved in `r27`), the opposite of what the export prints. Both bodies take
  their arguments from the prologue (`mr r27,r3` / `mr r28,r4`), and the save
  and restore helpers are not reproduced as calls.
* **`0x802C7C70` scratch block.** The seven query outputs are stack slots
  `r1+0x08`, `+0x0A`, `+0x0C`, `+0x10`, `+0x14`, `+0x18` and `+0x1C`. The saved
  `r27..r31` occupy `r1+0x2C..r1+0x3C`, so the writable window is
  `r1+0x08..r1+0x2C`; the body models it as one 0x24-byte buffer and reads the
  fields back with the same offsets, which is also why the export's
  `local_24[9]` (36 bytes from `+0x18`) cannot be literal.
* **`0x802C78F0` repeated query.** The compiler re-emits the `+0x08` slot call
  twice per source read (once for the test, once for the value) and the `+0x10`
  slot call likewise; the body keeps the same call counts rather than caching,
  because the slot is live code and the export also shows two calls.
* **`0x802C78F0` byte split.** Its `+0x04` word is stored as four separate
  big-endian bytes (`local_18`, `uStack_17`, `uStack_16`, `uStack_15`), which
  is byte-for-byte `write_u32(this + 0x04, word)` on the target.
* **`0x802C6D60` (medium).** The test `*(this) == 0` and both tail targets are
  literal; the forwarded `r3` is read off the raw branches, not from the
  export, which shows two argument-less calls. Confidence is medium for that
  reason only.

## Verification

Scratch copy at `%TEMP%\opencode\wave220_laneF` (`src/`, `Cargo.toml`,
`Cargo.lock`), this block appended to `src/resource/archive.rs`, then
`cargo check --lib`:

```
warning: `nsmbw` (lib) generated 26 warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 18.24s
EXIT=0
```

The 26 warnings are pre-existing and none points into this block.
