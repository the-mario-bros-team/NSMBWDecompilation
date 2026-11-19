# Wave 220 lane D coverage (`resource::archive`)

Slice: the ten `resource::archive` bodies at `0x802C6A20`, `0x802C6DD0`,
`0x802C6EF0`, `0x802C6FF0`, `0x802C7370`, `0x802C7A50`, `0x802C7A90`,
`0x802C7B30`, `0x802C8450` and `0x802C84F0` (1532 bytes of `functions.tsv`
extent, all ten verified as the real extent). All ten are translated; nothing
was deferred.

Bodies live in `work/wave220/laneD_block.rs` and are appended to
`src/resource/archive.rs` by the merge step; ledger rows are in
`ledger/agents/wave220-laneD.csv` (`module = resource::archive`).

This is the view/projection and text-row corner of the `0x802C0000` band. It
picks up where the wave-219 texture/GX lanes stop: `0x802C6DD0` installs the
four `r13 - 0x47C8..-0x47BC` view terms, `0x802C6FF0`/`0x802C7B30` turn a
source rectangle plus those terms into the seven-word projection array,
`0x802C7370` fills the nine glyph slots of a text row, `0x802C7A50`/
`0x802C7A90` are the render-target begin/end pair, and `0x802C8450`/
`0x802C84F0` are the deleting destructor and default-record reset of the
layout object whose vtable is `0x80350370`.

## Translated

| Address | Bytes | Rust body | Summary |
| --- | --- | --- | --- |
| `0x802C6A20` | 16 | `fn_802c6a20` | Loads `*(this + 0x00)`, loads slot `+0x14` and tail-branches to it with the incoming registers untouched. `// partial:` only `r3` is visible in the export, so the slot's remaining arguments are not modelled. |
| `0x802C6DD0` | 28 | `fn_802c6dd0` | Four `stfs` into `0x8042AEB8`, `0x8042AEBC`, `0x8042AEC0`, `0x8042AEC4` (the compiler uses two `addi` bases plus `+4` displacements). |
| `0x802C6EF0` | 104 | `fn_802c6ef0` | `0x802C7190(owner, &a, &b, &c, &d)` derives four corner terms; then `0x80253C70(a, b, c, d, owner[+0x18], owner[+0x1C], holder)` stores them into the draw object and sets its `0x40` quad flag. |
| `0x802C6FF0` | 188 | `fn_802c6ff0` | With `w = +8`, `h = +0xC`, `gx = 1/(0.5*w)`, `gy = 1/(0.5*h)`: `out[0]=0`; `out[1]=+0x14/(+0x28*(w/h))`; `out[2]=+0x20*gx`; `out[3]=+0x14/+0x2C`; `out[4]=+0x24*gy`; `out[5]=-+0x18/(+0x1C-+0x18)`; `out[6]=+0x1C*out[5]`; then tail-calls `0x802C7B30(out, A, B, C*gx, D*gy)` with the four view terms. |
| `0x802C7370` | 488 | `fn_802c7370` | Row descriptor `{u16 limit at +0, u16 count at +2, `0xC`-stride rows at +0xC}`, holder pointer at `r4`. `stop = min(table[+2], (*holder)[+0x3C])`; a non-negative `start` overrides it with `start+1`. Per row: build the `{table, row}` record, look the row up with `0x802425E0`, bail out with `0` when either word is null. Then for each of the `u8` glyph codes at `+0x3C`: `0xFFFF` writes the `0xFF` marker; otherwise `0x80242530(holder, code)` gives `+0x10` for the ascending slot and, when `+0x1C & 8`, `+0x14` for the descending slot. Unused slots through the descending cursor get `0xFF`. Slot 8 comes from `0x802423F0(holder, row[+0x18])[+0x10]`, or `0xFF` when `+0x14` is zero. Returns `1`. |
| `0x802C7A50` | 60 | `fn_802c7a50` | Stores the `u16` mode at `0x8042AED4` and the target at `0x8042AED0`; `0x801C0500(source, 0x80424698)` installs the transform; `0x802D1DA0()` resets the 2D draw state. |
| `0x802C7A90` | 124 | `fn_802c7a90` | Reads `0x8042AED8` and feeds bits 0/1/2 to `0x802D2F20`/`0x802D2F80`/`0x802D2FE0`; `0x802D3040(0, 0)`; when `finish != 0` and the target is non-null, calls virtual slot `+0x0C` of the target's `+0x38` sub-object with the target as `r3`. First argument is unused. |
| `0x802C7B30` | 64 | `fn_802c7b30` | `terms[+4] *= 1/a`; `terms[+0xC] *= 1/b`; `terms[+8] += c`; `terms[+0x10] += d`. The two divisions are computed before the multiplies, as the `fdivs`/`fmuls` pairs do. |
| `0x802C8450` | 160 | `fn_802c8450` | Null-safe deleting destructor. Installs vtable `0x80350370`, calls the child at `+0x14` through its `+0x18` slot with `(child, 1)`, frees `+0x10` with `0x802B9290`, `delete[]`s `+0x08` with the `0x802C8410` element destructor, frees `+0x0C`, and frees `this` with `0x802B9280` when `flags > 0`. Returns `this`. |
| `0x802C84F0` | 300 | `fn_802c84f0` | For `i < obj[+4]`: `0x802C9AF0(obj[+8] + i*0xA4)` and, from the second record on, clear bit 0 of its `+0x68` halfword. For `j < obj[+5]`: write `0x646464FF` at `obj[+0xC] + j*0xC` and the word at `0x80429B5C` at `+4`. Copy `00 00 00 FF` to `+0x18..+0x1B`, clear `+0x1D`, and mask `+0x1E` with `0xFFE0`. |

## Sizes and extents

Every `# Original` block keeps the canonical `main-gb` `functions.tsv` value,
and every value was re-checked against the prologue head and the closing
control transfer in `work/dol/main.elf`:

| address | tsv | first instruction | closing transfer | extent |
| --- | --- | --- | --- | --- |
| `0x802C6A20` | 16 | `lwz r12,0(r3)` (leaf) | `bcctr` at `0x802C6A2C` | 16 |
| `0x802C6DD0` | 28 | `addi r4,r13,-18376` (leaf) | `blr` at `0x802C6DE8` | 28 |
| `0x802C6EF0` | 104 | `stwu r1,-0x20` | `blr` at `0x802C6F54` | 104 |
| `0x802C6FF0` | 188 | `lfs f3,8(r3)` (leaf) | `b 0x802C7B30` at `0x802C70A8` | 188 |
| `0x802C7370` | 488 | `stwu r1,-0x40` | `blr` at `0x802C7554` | 488 |
| `0x802C7A50` | 60 | `stwu r1,-0x10` | `blr` at `0x802C7A88` | 60 |
| `0x802C7A90` | 124 | `stwu r1,-0x10` | `blr` at `0x802C7B08` | 124 |
| `0x802C7B30` | 64 | `lfs f8,-0x3C98(r2)` (leaf) | `blr` at `0x802C7B6C` | 64 |
| `0x802C8450` | 160 | `stwu r1,-0x10` | `blr` at `0x802C84EC` | 160 |
| `0x802C84F0` | 300 | `stwu r1,-0x20` | `blr` at `0x802C8618` | 300 |

Two entries close with a tail branch instead of `blr`: `0x802C6A20` is the
16-byte virtual thunk (`bcctr`) and `0x802C6FF0` ends in `b 0x802C7B30`. In
both cases the next `functions.tsv` entry starts exactly at `start + size`, so
the tsv size is the real extent, not a prologue artefact.

## Reading notes

* `0x802C7370` and its caller `0x802C8B00` decompile a `__savegprlr_20` /
  `__restgprlr_20` pair as a 64-bit return. The stub at `0x802DCD58` only
  stores `r20..r31` and returns, so the caller's `r3`/`r4` survive: the
  "high word" is the first argument (the row table) and the "low word" is the
  second (the holder pointer). The body keeps the export's argument order but
  corrects that aliasing in the prose.
* `0x802C7A50`'s target word `0x8042AED0` is already bound by earlier waves as
  `G_ARC_HUD_OWNER_A`; this lane reuses that constant instead of redefining
  it, and documents the render-target role in place.
* All constants added here use the `_D` suffix and were checked against
  `src/resource/archive.rs` for duplicates. Values read from the image:
  `r2 - 0x3CBC = 0.0f`, `r2 - 0x3CB8 = 0.5f`, `r2 - 0x3CC8 = 1.0f`,
  `r2 - 0x3C98 = 1.0f`, `r2 - 0x3C70 = 0x646464FF`, `r2 - 0x3E40 = 00 00 00 FF`,
  `r13 - 0x5B24 = 0x80429B5C -> 0x80429B58` (`"--\0\0LOBJ"`).
* The two `0x802C8450`/`0x802C84F0` bodies are vtable slots `+0x18` and `+0x1C`
  of `0x80350370`; the slot addresses were read from the image
  (`0x80350388` and `0x8035038C`).

## Deferrals

None.

## Verification

* Extent check: `work/dol/main.elf` decoded for all ten entries; table above.
* Scratch crate: `src/`, `Cargo.toml` and `Cargo.lock` copied to
  `%TEMP%\opencode\w220laneD`, `laneD_block.rs` appended to the copy's
  `src/resource/archive.rs`, then `cargo check --lib` -> exit 0 (26 existing
  warnings elsewhere in the crate, none from the appended block).
