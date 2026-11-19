# Audit - Wave 241 (body-gap closure, batch 13)

## 1. Every ledger row now provably has a body - and the gate enforces it

`0x801AE0A0` (`runtime::init`, previously `stub` with no body) was decoded from
the image:

```
lis  r5,0x8000
lwz  r4,0x30F0(r5)      ; latest boot/loader record
cmplw cr0,r4,r5         ; r4 >= 0x80000000 ?
blt  +0xC               ; no -> zero path
li   r5,0x1C
bl   0x80004364         ; memcpy(dst, record, 0x1C)
li   r0,0
stw  r0,0(r3)           ; zero path: clears ONLY word 0
blr
```

Wave 240's reading was right except that the zero path clears only the first
word, not all 0x1C bytes. The record's writer `0x801AE840` allocates 0x1C bytes,
sets word0 = 1, word1 = tick, word2 = accumulated size, words 3-5 = params,
word6 = per-section handle, and publishes it to `0x800030F0`. So the function is
a getter for the latest loader/boot descriptor; its role is marked inferred.

With that body, `node tools/verify-ledger.mjs` reports **20,250 rows, 0
duplicates, 0 missing, `bodies.ok: true`, gaps 0** (19,062 `fn_<addr>` bodies +
1,188 semantic bodies). `scripts/check.ps1` now runs
`verify-ledger.mjs --enforce-bodies`, so the `0x806E7F50`/`0x801AE0A0` class of
gap cannot recur silently.

## 2. Bodies translated (54)

| Lane | Source | Bodies | Bytes |
|---|---|---:|---:|
| B | `WM_PLAYER_VTABLE` 170-183 | 12 | 1,864 |
| C | `dEn_c` 123-132 | 10 | 808 |
| D | WorldMap vtable 174-187 | 10 | 2,448 |
| E | player-class vtable 163-188 | 12 | ~600 |
| F | `0x803502F8` band, 10 slots | 10 | 3,516 |

Notable: the player-class batch was eleven 4-byte hooks plus one 560-byte motion
refresh (animation height x `0.3f` easing when `+0x10D4` bit 0 is set). The
archive band's five `0x80350370` row-table codecs include a 1,528-byte mode-0
loader and a 964-byte mode-2 interpolator.

## 3. Image facts recorded

- `0x802C8D10`'s mode-1 eight-times-unrolled copy is **unreachable** (row count
  capped at 8).
- Both packed-row paths place the packed base with the **unclamped** header
  count while clamping the record/row loops.
- `dEn_c` slot 128 is a tail branch to `0x800954D0`; slots 129/131/132 have real
  `lwz r12,r12+off` dispatchers, the other seven are table-only.
- `WM_PLAYER_VTABLE` 170-183 is the mount/dismount controller family.
- The `0x80350230` family is complete (36/36).

## 4. Process note

`.\scripts\check.ps1` is blocked by machine execution policy; it must be run as
`powershell -NoProfile -ExecutionPolicy Bypass -File .\scripts\check.ps1`.
Lane F skipped the gate while other lanes were writing; the coordinator ran it
after all lanes settled: **exit 0**.

## Verification

- `cargo check` exit 0; `doc-lint` 0 violations across 68 files.
- `scripts/check.ps1` exit 0 with body enforcement armed.
- Ledger 20,250 rows, 0 duplicates, 0 staging.
