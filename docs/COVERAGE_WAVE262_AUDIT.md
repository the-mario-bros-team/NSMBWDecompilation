# Audit - Wave 262 (ninth vtable, polarity bug fixed)

## 1. Two more vtables complete

| Table | Slots |
|---|---:|
| `0x80303AA4` (`pakkun` derived) | **183/183** |
| `0x80303F50` (`pakkun` base) | **182/182** |

Re-measured twin shape: **166 shared** slot-index words, **16 differing**
(`+0x40`, `+0x22C`, `+0x278..+0x2A4`, `+0x2C4`, `+0x2D0`), 17 targets unique to
the derived table and 16 to the base.

**Nine vtables are complete**: 275, 278, 311, 45, 306, 235, 265, 183, 182.

## 2. Polarity bug in `fn_800244d0` fixed

The `fcmpo cr0,f1,f0` / `cror 2,1,2` / `bc 4,2` idiom sets CR0[EQ] to
`f1 >= f0` and branches when that is **false**, i.e. when `f1` is negative or
unordered - and that arm stores 1. The body read it as "non-zero"
(`above > 0.0 || above < 0.0`), which is wrong for positives and for zero.

Corrected to `!(above >= 0.0) as u32`, which matches the repo convention used in
the wave-227 NaN audit.

## 3. Bodies

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| A | `0x80306400` next 12 | 12 | 4,940 |
| B | `0x80303104` next 12 | 12 | ~2,000 |
| C | `0x8030241c` next 12 | 12 | 2,332 |
| D | `pakkun` twin remainder (closes both) | 42 | ~3,000 |
| E | `0x80304e18` first 10 | 10 | ~1,500 |

Coverage after the wave: `0x80306400` 152/193 (41 uncovered),
`0x80303104` 170/187 (3 prologue), `0x8030241c` 161/199 (38),
`0x80304e18` 153/186 (14 prologue).

## 4. Findings

- `fn_802de124` (wave 261) needed Rust `/ 2`, not `>> 1`: the image uses the
  truncating signed divide idiom, so `-63` is `-31`.
- Wave-204's `F32_SDA2_7D54` is documented as a zero vector but the shipped word
  is `1.3f`; an accurately named constant was added (no body reads the old one).
- `0x80306400` is an **uninstalled** vtable (`daEnTogezo_c`): no image builds or
  stores its base.
- `0x8030241c` is confirmed as a C++ vtable by ten `d_enemiesNP` relocations
  naming its header.
- Lane F audited 60 wave-261 bodies: all words, slots, extents, ownership and
  censuses reproduce; 9 doc corrections (six D extents had excluded the trailing
  `blr`, D total 2,164 -> 2,188).

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations.
- `check-dupe-bodies` exit 0.
- `scripts/check.ps1` exit 0.
