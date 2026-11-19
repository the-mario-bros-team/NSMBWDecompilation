# Coverage - Wave 111 (runtime::dvd, the bit-coded LZ block decoder)

## How to read this file

The `runtime::dvd` lane owns the whole `0x801D0000` 64 KiB page of `main.dol`,
which is much wider than the disc band. Every body below is non-disc code that
happens to share the page; `src/runtime/dvd.rs` is the only writable file
assigned to the bucket, so the bodies live there.

`high` means every store, mask and branch in the body is literal and its role is
proven; `medium` means the instruction sequence is literal but at least one
field role or callee identity is inferred; `low` means only the store sequence
is certain.

This pass started from exactly the two addresses Wave 109 left unclaimed:
`0x801D6F60` and `0x801DEB70`. One is translated here. The other is skipped for
a reason that Wave 111 could measure byte by byte on the raw instruction stream
in `work/dol/main.elf`, which is stronger than the export-only reason recorded
in `docs/COVERAGE_WAVE109_DVD.md`.

## Summary

| File | Module | Bodies | Bytes | High | Medium | Low |
|---|---|---:|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 1 | 2036 | 0 | 1 | 0 |

## What this pass covers

- **The bit-coded LZ block decoder, `0x801D6F60` (2036 bytes).** This is the
  third decoder that runs on the `0x801D6F20` context, beside the two-run
  decoder pair `0x801D68C0`/`0x801D6B60` and the arithmetic decoder
  `0x801D7D50`. It reads a 32-bit block header, builds two canonical code tables
  from bit-packed code lengths, then emits literals and back-references until
  the `+0x04` output count reaches zero. All stream state (`+0x88C`, `+0x8A4`,
  `+0x89C`, `+0x8A0`) is written back on every normal exit, so a caller can
  resume after an underrun; `0x802B8510` in the image is exactly such a
  completion probe (it calls the body with the context only and asks
  `result == 0`).

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D6F60` | `fn_801d6f60` | 2036 | medium |

## Notes on the recovered operands and stores

- **The header swap.** The export prints
  `(uVar7 << 0x18 | (uVar7 & 0xff00) << 8 | uVar7 >> 8 & 0xff00) >> 8`, which
  drops a byte and cannot be right. The listing at
  `0x801D7040`-`0x801D7050` is `rlwinm r3,r5,8,8,15` /
  `rlwinm r0,r5,24,16,23` / `rlwimi r3,r5,24,0,7` / `rlwimi r0,r5,8,24,31` /
  `or`, evaluated against `0x4ABCDEF0` gives `0xF0DEBC4A`: a plain 32-bit byte
  swap. The body therefore stores `word.swap_bytes()`, tests
  `swapped & 0xF0 == 0x40` for the tag nibble and takes `swapped >> 8` as the
  24-bit length, which is what `0x801D7054`-`0x801D706C` do.
- **The 32-bit field read.** `0x802DD210` is `__ashrdi3` (a signed 64-bit shift
  right, twelve instructions), not an SDK routine. The body inlines its
  arithmetic: the accumulator is refilled to at least 32 bits while the bits
  that leave the 32-bit register are kept in a second word, then
  `(extra:acc) >> (bits - 32)` is taken. A non-negative result is the
  end-of-stream marker at `0x801D70F8`-`0x801D7010`.
- **The two table budgets.** The 16-bit field at `+0x890` is read byte-swapped
  and expanded to `(value + 1) * 32 - 16` bits; the 8-bit field at `+0x894`
  expands to `(value + 1) * 32 - 8`. The bodies then read 9-bit and 5-bit code
  lengths while more than 8 (resp. 4) bits remain, discard the tail, and hand
  the built table to `fn_801d82e0` with depth 9 and 5. The field role beyond
  "bits available to the table" is `// unknown:` and is marked as such.
- **The tree roots.** `fn_801d6f20` stores `context + 0x0E` at `+0x88C`, which is
  the first node of the table at `+0x0C` (entry 0 is the count); every walk
  masks the node pointer with `!3` to recover the table base, and the distance
  tree root is `context + 0x80E` (table at `+0x80C`).
- **The match length extension.** A leaf byte of the distance walk is a bit
  width, not a value: widths above one read that many bits and OR back the
  implied leading one at `1 << (width - 1)`. Widths of zero or one keep the
  sign-extended leaf byte itself as the distance, exactly as the register reuse
  at `0x801D7688`-`0x801D769C` does.
- **The shared bit reader.** The nine inlined copies are the same
  `do { *acc = *acc << 8 | *in++; } while (--ctr)` refill plus one `srw`/mask;
  they are factored into the private helper `dvd_take_bits`, which advances the
  cursor and budget on `None` just like the original's error paths.

## Deliberate skips

One address is not claimed.

- `0x801DEB70` (2524 bytes) - the per-frame Wi-Fi reconnect/update tick. Wave
  109 skipped it because the export builds a twelve-word request descriptor from
  uninitialized stack slots and hides the `r4` half of `0x802DCEC4` behind
  `extraout_r4`. The instruction stream settles both halves of that claim:
  - The `extraout_r4` is a real second return word: `0x802DCEC4` is
    `__divdi3` (it sign-normalises `r3:r4` and `r5:r6`, counts leading zeros and
    runs the 64-bit divide), so `(now - stored) / (bus/4)` returns its quotient
    in `r3:r4` and `0x801DF248` tests the low word. That part is recoverable.
  - The descriptor is not. `0x801DF3D8`-`0x801DF444` copies twelve words from
    the frame area `r1+0x18`-`r1+0x44` to `r1+0x48`; the body writes only
    `r1+0x18` (word `0x10`), the **low byte** of `r1+0x1C` (zero), the **low
    half** of `r1+0x30` (one) and `r1+0x44` (zero). The bytes at `r1+0x1D`-`0x2F`
    and `r1+0x33`-`0x43` are never written anywhere in the function, and the
    callee `0x801DDD20` reads exactly the unwritten fields: `puVar8[7]`,
    `puVar8[9]`, `the high half of puVar8[8]` and `puVar8[0xB]` for command
    `0x17`, `puVar8[10]` for `0x15`, plus one payload byte from `puVar8 + 1`
    (the MSB of the half-initialised word) on every command. Only 11 of the 48
    descriptor bytes are defined, so any Rust body would have to invent 37 bytes
    of stack garbage that the callee then forwards to the network layer.

The skip is recorded here rather than padded with a guess.

## Verification

- The address and byte size match `work/ghidra/main-gb/export/functions.tsv`
  (1 row, 2036 bytes, 0 high, 1 medium, 0 low).
- The evidence file exists at the cited
  `work/ghidra/main-gb/export/801d6f60_FUN_801d6f60.c` path.
- The row has exactly one `pub unsafe fn fn_801d6f60` body in
  `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`; this was the bucket's
  remaining untranslated set.
- `node tools/check-lane-csv.mjs wave111` reports no problem for
  `ledger/agents/wave111-dvd.csv`.
- `cargo check --message-format=short` reports no diagnostic naming
  `src/runtime/dvd.rs`.
