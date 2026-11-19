# Coverage - Wave 33 (`d_enemiesNP`)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/game/enemies.rs` | `game::enemies` | 26 | 3820 | 17 | 9 |

Address range `0x80AA1BF0..0x80AA37B0`, pass 27.

## What this pass covers

- **`daEnKanon_c` tail** (`0x80AA1BF0..0x80AA2CB0`, 18 bodies). The two
  node-arm callbacks its state table installs (`0x80AA1BF0`/`0x80AA1C50`) and
  the two extra callback words of the death records (`0x80AA1CB0`/`0x80AA1CC0`);
  the child-chain walker `0x80AA1DB0` and the name-suffix comparator
  `0x80AA1E90`; the two `0x802DCBFC` state-destructor forwarders
  (`0x80AA1F50`/`0x80AA1F80`); the model transform `0x80AA21E0` and the
  `karon_head` binder `0x80AA2310`; the landing-smoke spawner `0x80AA2580`
  and the touch-sound request `0x80AA2620`; and the state steps
  `0x80AA26C0` (frame integration plus dual side probe), `0x80AA28C0`
  (attach), `0x80AA29B0` (spin), `0x80AA2A80` (bounce), `0x80AA2B40`
  (homing) and `0x80AA2CB0` (placement).
- **`keronpa` plant family head** (`0x80AA3190..0x80AA37B0`, 8 bodies). The
  family's node-arm callback `0x80AA3190` and two destructor forwarders
  `0x80AA32B0`/`0x80AA32E0`; the `0xA58`-byte allocator `0x80AA3310` and the
  `g3d/keronpa.brres` binder `0x80AA3340`; the per-side clip binder
  `0x80AA34E0`; and the two position-trail samplers
  `0x80AA3780`/`0x80AA37B0`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x80AA1BF0` | `fn_80aa1bf0` | 88 | high |
| `0x80AA1C50` | `fn_80aa1c50` | 92 | high |
| `0x80AA1CB0` | `fn_80aa1cb0` | 12 | high |
| `0x80AA1CC0` | `fn_80aa1cc0` | 12 | high |
| `0x80AA1DB0` | `fn_80aa1db0` | 224 | high |
| `0x80AA1E90` | `fn_80aa1e90` | 136 | medium |
| `0x80AA1F50` | `fn_80aa1f50` | 48 | medium |
| `0x80AA1F80` | `fn_80aa1f80` | 48 | medium |
| `0x80AA21E0` | `fn_80aa21e0` | 292 | medium |
| `0x80AA2310` | `fn_80aa2310` | 276 | high |
| `0x80AA2580` | `fn_80aa2580` | 148 | high |
| `0x80AA2620` | `fn_80aa2620` | 160 | high |
| `0x80AA26C0` | `fn_80aa26c0` | 464 | medium |
| `0x80AA28C0` | `fn_80aa28c0` | 224 | high |
| `0x80AA29B0` | `fn_80aa29b0` | 160 | high |
| `0x80AA2A80` | `fn_80aa2a80` | 152 | high |
| `0x80AA2B40` | `fn_80aa2b40` | 332 | high |
| `0x80AA2CB0` | `fn_80aa2cb0` | 76 | high |
| `0x80AA3190` | `fn_80aa3190` | 88 | high |
| `0x80AA32B0` | `fn_80aa32b0` | 48 | medium |
| `0x80AA32E0` | `fn_80aa32e0` | 48 | medium |
| `0x80AA3310` | `fn_80aa3310` | 48 | medium |
| `0x80AA3340` | `fn_80aa3340` | 412 | high |
| `0x80AA34E0` | `fn_80aa34e0` | 152 | medium |
| `0x80AA3780` | `fn_80aa3780` | 40 | high |
| `0x80AA37B0` | `fn_80aa37b0` | 40 | high |

## Verification

- Every address and byte size matches
  `work/ghidra/rel-d_enemiesNP/export/functions.tsv`.
- Every evidence file exists at the cited path under
  `work/ghidra/rel-d_enemiesNP/export/`.
- Every row has exactly one `pub unsafe fn fn_<addr>` body in
  `src/game/enemies.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check --message-format=short` is free of errors for
  `src/game/enemies.rs`.

## Deliberately skipped

These neighbours were read and left out because no honest body is possible from
the export alone.

- `0x80AA0080` - `halt_baddata` trap thunk; not a function.
- `0x80AA1030` - calls the `0x80AA0080` trap, so it cannot be reconstructed.
- `0x80AA0530`, `0x80AA0C20` - paired-single `ps_*` bodies using GQR0 and
  `ldexpf`; the Ghidra C loses the paired-single semantics.
- `0x80AA35E0`, `0x80AA37E0`, `0x80AA3A90` - GQR0 paired-single helpers.
- `0x80AA38D0` - GQR0 helper whose four `f64` arguments do not correspond to a
  provable signature.
- `0x80AA39D0` - `halt_baddata` trap thunk.
- `0x80AA3580`, `0x80AA3960`, `0x80AA3A10`, `0x80AA3B70` - outside the selected
  26-body window; left for a later pass so the `keronpa` tail can be taken as
  one contiguous run.
