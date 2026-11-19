# Wave 220, lane E - `src/resource/archive.rs`, addresses `0x802C6540..0x802C8BA4`

Ten entries of `work/ghidra/main-gb/export/functions.tsv` (`0x802C6540`,
`0x802C68E0`, `0x802C6AA0`, `0x802C6E80`, `0x802C8000`, `0x802C8150`,
`0x802C81D0`, `0x802C81F0`, `0x802C83D0`, `0x802C8B00`, rows 9592-9642) in the
`0x802C0000` band of the `resource::archive` ledger module. All ten are
translated; nothing was deferred.

Block file: `work/wave220/laneE_block.rs`
Ledger: `ledger/agents/wave220-laneE.csv`

| address | tsv size | real extent | name | confidence | role |
|---|---|---|---|---|---|
| `0x802C6540` | 16 | `0x802C6540..0x802C6550` (16) | [`fn_802c6540`] | high | tail-dispatch vtable slot `+0x08` |
| `0x802C68E0` | 320 | `0x802C68E0..0x802C6A20` (320) | [`fn_802c68e0`] | high | fill the 0x30-byte state record and hand it to slot `+0x0C` |
| `0x802C6AA0` | 60 | `0x802C6AA0..0x802C6ADC` (60) + 4 pad | [`fn_802c6aa0`] | high | install `0x80350338` at `+0x38` and run `0x802C6BF0` |
| `0x802C6E80` | 108 | `0x802C6E80..0x802C6EEC` (108) + 4 pad | [`fn_802c6e80`] | high | store and flush one quad (`0x80253C30`/`0x802C6F60`/`0x80253CB0`) |
| `0x802C8000` | 188 | `0x802C8000..0x802C80BC` (188) | [`fn_802c8000`] | high | install the `+0xEC` sink, two mode arms |
| `0x802C8150` | 120 | `0x802C8150..0x802C81C8` (120) + 4 pad | [`fn_802c8150`] | high | clear the global 4-bit mask, arm `+0xF0`, flush |
| `0x802C81D0` | 28 | `0x802C81D0..0x802C81EC` (28) + 4 pad | [`fn_802c81d0`] | high | tail-dispatch the virtual behind `+0xF0` |
| `0x802C81F0` | 420 | `0x802C81F0..0x802C8398` (420) + 8 pad | [`fn_802c81f0`] | high | construct the 0x28-byte-per-lane object |
| `0x802C83D0` | 64 | `0x802C83D0..0x802C8410` (64) | [`fn_802c83d0`] | high | deleting helper of the record |
| `0x802C8B00` | 164 | `0x802C8B00..0x802C8BA4` (164) | [`fn_802c8b00`] | medium | per-frame lane/mirror walk |

Total: 1488 bytes, 10 bodies, 0 deferrals.

## Extent checks

Every `functions.tsv` size in this lane matches the real extent; **none** is a
prologue artefact. Each was re-checked in `work/dol/main.elf` from the
prologue down to the closing `bcctr`/`bclr`:

* Seven bodies end on their `bclr` at `start + tsv - 4`; the words between that
  `bclr` and the next `functions.tsv` entry are alignment zero fill the tsv does
  not count (4 bytes after `0x802C6AA0`, `0x802C6E80` and `0x802C81D0`, 8 after
  `0x802C8150` and `0x802C81F0`, 12 after `0x802C8B00`).
* `0x802C6540` is 16 bytes and ends on `4e800420 bcctr`, not `bclr`: it is a
  pure vtable tail call (`lwz r12,0x0(r3)`; `lwz r12,0x8(r12)`; `mtspr 288,r12`;
  `bcctr 20,0`) and the next entry `0x802C6550` follows immediately.
* `0x802C81D0` is 28 bytes and likewise ends on `bcctr` (slot `+0x08` of the
  inner object's vtable), with one alignment word before `0x802C81F0`.
* `0x802C8000`, `0x802C83D0` and `0x802C8B00` close exactly on `bclr` with the
  tsv size covering the whole body: `188` bytes (`bclr` at `0x802C80B8`,
  alignment word at `0x802C80BC`), `64` bytes (`bclr` at `0x802C840C`, next
  entry at `0x802C8410`) and `164` bytes (`bclr` at `0x802C8BA0`, alignment
  words at `0x802C8BA4..0x802C8BAC`). `0x802C68E0` likewise (`320` bytes,
  `bclr` at `0x802C6A1C`, next entry at `0x802C6A20`).

## What the run is

`// inferred:` this is the teardown/rebuild half of one display-record family,
not archive reading. It sits directly after the wave-219 GX display-object band
(`0x802C0A80..0x802C378C`), before the `0x802C9A30` child factory that
`0x802C83A0` wraps, and its records use the `0xA4`-byte lane and `0x44`-byte
mirror strides already documented for the RLYT polygon/child records in waves
99 and 219. The exports prove only the offsets, the masks, the strides and the
callee addresses.

## Bodies

* `0x802C6540` forwards `r3` unchanged to slot `+0x08` of `*this`; nothing is
  written, so it is modelled as an indirect call, not a `vcall` helper.
* `0x802C68E0` clears all 0x30 bytes of the record, reads the mode byte from
  `0x802C6030`, reads a word from slot `+0x10`, copies four bytes from
  `0x802C6020` (the getter is re-invoked per byte, as in the image), clears
  `+0x10..+0x2C`, and finally calls slot `+0x0C` with `(this, record)`.
* `0x802C6AA0` writes `0x80350338` at `+0x38`, runs `0x802C6BF0` and returns
  `this`.
* `0x802C6E80` stores `(*(f32+0x10), *(f32+0x08)/*(f32+0x0C), *(f32+0x18),
  *(f32+0x1C))` into the sink through `0x80253C30`, builds a 64-byte payload
  with `0x802C6F60` and flushes it with `0x80253CB0`. The four leading
  arguments are `lfs`-loaded in the image, so the helper added here passes
  `f32`, not the `double` the Ghidra prototype claims.
* `0x802C8000` indexes the record array with a *12 stride, installs
  `0x802C8150` at `+0xEC`, rewrites the low two bits of `+0xE8`
  (`& 0xFFFFFFFE | 2` for `mode == 0`, `& 0xFFFFFFFD | 1` otherwise), calls
  `0x80257670`/`0x80257640` and rewrites bit 0 of the byte at `+0x0A`. The
  array base is reloaded from `*this` after the call in both arms, and the body
  does the same.
* `0x802C8150` clears the low nibble of the global halfword at
  `r13 - 0x472E` (`0x8042AF52`), dispatches [`fn_802c81d0`], flushes through
  `0x802D2E50`, then re-arms `0x802C7A90` only if the nibble was non-zero. The
  re-arm value is the `neg`/`or`/`rlwinm ...,1,31,31` idiom, i.e. `0` or `1`;
  Ghidra renders it as `-(...) >> 0x1f`, which would be `-1`, so the body uses
  the derived `0`/`1` and the prose records the divergence.
* `0x802C81D0` reads `link = *(this+0xF0)`, then tail-calls slot `+0x08` of
  `*(*(link)+8)` with `(*(link), u16(link+8))`. The export's `(int*)link + 2`
  is `link + 8` and agrees with the `lhz r4,0x8(r4)` load.
* `0x802C81F0` writes the `0x80350370` vtable, the `CONCAT11(first, second)`
  halfword at both `+0x04` and `+0x06`, `third` at `+0x1C`, `0x40` at `+0x1E`
  and `0xFFFF` at `+0x24`, then allocates four blocks: a 0x1C-byte child built
  by `0x802CCE10`, the `third * min(first, 8) * 0x40` scratch block at `+0x10`,
  the `first * 0xA4 + 0x10` table built by `0x802DC7E0` at `+0x08`, and the
  `second * 0x0C` mirror table at `+0x0C`. The two init loops write the lane
  index/zero halfword and call `0x802C97F0`, then clear byte `+0x08` of each
  0x0C-byte mirror entry. It closes with slot `+0x1C` and returns `this`.
* `0x802C83D0` frees the object with `0x802B9280` only when it is non-null and
  `deleting > 0`, and returns `this` in every case. The image passes no fresh
  arguments, so `r3` is still the object.
* `0x802C8B00` is the per-frame walk: bit `0x10` of `+0x1E` selects one
  four-word upload through `0x802C7370(owner+0x2878, block, (s16)(this+0x24))`,
  then `0x802C8AC0(this, owner)` commits the pair, then one `0x802CA140` call
  per lane forwarding `*(this+8) + n*0xA4` and `*(owner+0x287C) + n*0x44`.

## Deferrals

None.

## Verification

The block was appended to a scratch copy of `src/`, `Cargo.toml` and
`Cargo.lock` in `%TEMP%\opencode\w220laneE`; `cargo check --lib` there finished
with exit code 0 and no new warnings.
