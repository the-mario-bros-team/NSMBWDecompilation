# Coverage - Wave 39 (runtime::dvd pass 3)

## Summary

| File | Module | Bodies | Bytes | High | Medium |
|---|---|---:|---:|---:|---:|
| `src/runtime/dvd.rs` | `runtime::dvd` | 30 | 4652 | 10 | 17 |

## What this pass covers

- Heap-list primitives (`0x801D5420`-`0x801D56C0`): the `+0x3C`/`+0x40`
  list counter, alignment formula, vtable-slot thunks, the four-word init,
  stride-list insert/remove/next helpers.
- DSP mailbox and task queue (`0x801D56E0`-`0x801D61A0`): the
  `0xCC005000` mailbox accessors, one-shot init, init-latch getter,
  insert/boot, pending selection, empty log hook, IRQ-handler dispatcher,
  ten-word task sender, priority-list insert/remove. All share the task
  globals `r13-0x4C50`/`-0x4C4C`/`-0x4C54` and `r13-0x4C5C`/`-0x4C60`.
- Auxiliary IOS wrappers (`0x801D62E0`-`0x801D6780`): seven sync
  ioctl-vector wrappers sharing [`SDA_DVD_PROBE_FD`] (`r13-0x6020`) with
  the previously translated probe pair `0x801D6230`/`0x801D6290`.

## Functions

| address | function | bytes | confidence |
|---|---|---:|---|
| `0x801D5420` | `fn_801d5420` | 120 | high |
| `0x801D54A0` | `fn_801d54a0` | 36 | high |
| `0x801D5540` | `fn_801d5540` | 16 | low |
| `0x801D5550` | `fn_801d5550` | 16 | low |
| `0x801D5560` | `fn_801d5560` | 28 | low |
| `0x801D55C0` | `fn_801d55c0` | 24 | high |
| `0x801D55E0` | `fn_801d55e0` | 112 | high |
| `0x801D5650` | `fn_801d5650` | 108 | high |
| `0x801D56C0` | `fn_801d56c0` | 32 | high |
| `0x801D56E0` | `fn_801d56e0` | 16 | medium |
| `0x801D56F0` | `fn_801d56f0` | 16 | medium |
| `0x801D5700` | `fn_801d5700` | 20 | medium |
| `0x801D5720` | `fn_801d5720` | 20 | medium |
| `0x801D5740` | `fn_801d5740` | 192 | medium |
| `0x801D5800` | `fn_801d5800` | 8 | high |
| `0x801D5810` | `fn_801d5810` | 112 | medium |
| `0x801D5880` | `fn_801d5880` | 188 | medium |
| `0x801D5940` | `fn_801d5940` | 80 | medium |
| `0x801D5990` | `fn_801d5990` | 1068 | medium |
| `0x801D5DC0` | `fn_801d5dc0` | 420 | high |
| `0x801D5F70` | `fn_801d5f70` | 396 | medium |
| `0x801D6100` | `fn_801d6100` | 152 | high |
| `0x801D61A0` | `fn_801d61a0` | 136 | high |
| `0x801D62E0` | `fn_801d62e0` | 148 | medium |
| `0x801D6380` | `fn_801d6380` | 276 | medium |
| `0x801D64A0` | `fn_801d64a0` | 164 | medium |
| `0x801D6550` | `fn_801d6550` | 236 | medium |
| `0x801D6640` | `fn_801d6640` | 152 | medium |
| `0x801D66E0` | `fn_801d66e0` | 152 | medium |
| `0x801D6780` | `fn_801d6780` | 208 | medium |

## Deliberate skips

- `0x801D6230`/`0x801D6290` (probe open/close): already translated in
  Wave 13 as `fn_801d6230`/`fn_801d6290`; not repeated.
- Decompressor cluster `0x801D68C0`/`0x801D6B60`/`0x801D6F20`/`0x801D6F60`
  (inflate state plus Huffman tables): a different queue/tag object from
  the task/IOS band, left for a later pass.
- Anything clearly input (from `0x801DDAC0` onward): input lane;
  untouched.
- Early SI band `0x801C0020`-`0x801C9E20`: not DVD/NAND; untouched.

## Verification

- Every address and byte size matches `functions.tsv`.
- Every evidence file exists at the cited path.
- Every row has exactly one body in `src/runtime/dvd.rs`.
- No address was already in `ledger/functions.csv`.
- `cargo check` is free of errors for `src/runtime/dvd.rs`.
