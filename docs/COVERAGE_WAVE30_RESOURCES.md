# Coverage — wave 30 resource registry and archive additions

## How to read this file

This pass continues the archive-resource work from
`docs/COVERAGE_WAVE4_REGISTRY.md`, `docs/COVERAGE_WAVE15_RESOURCES.md` and
`docs/COVERAGE_WAVE24_RESOURCES.md`. It is organised by the flow, not by
address:

1. **Scope** — what was added, where it lives, and how big it is.
2. **Function table** — one row per new body: address, Rust name,
   plain-English role, status, confidence and Ghidra evidence file.
3. **Cluster notes** — the two flows the rows belong to:
   - the archive heap and Yaz0 wrapper (`0x802B8A70..0x802B9290`),
   - the EGG mount-stream base class (`0x802BA2B0..0x802BA800`).
4. **Unknowns** — object ownership, hook contracts and helper roles that are
   inferred from call sites.
5. **Verification** — compiler, doc linter and ledger.

Addresses are **virtual addresses of the unrelocated `main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. Status and confidence follow
`docs/READING_GUIDE.md`: `translated` means the complete recorded control flow
is present; `high` confidence means the bytes and call order are explicit, while
`medium` means one hidden argument or callee contract is inferred.

## Scope

| metric | value |
|---|---:|
| functions added | 22 |
| bytes of Ghidra function size | 2,572 |
| `translated` / `partial` / `stub` | 22 / 0 / 0 |
| high confidence | 20 |
| medium confidence | 2 |
| low confidence | 0 |
| new staging modules | 0 |
| files written | `src/resource/registry.rs`, `src/resource/archive.rs`, this document, `ledger/agents/wave30-resources.csv` |

The named roles divide as follows. **Member enumeration and archive lookup**
were already complete before this wave: the U8 iterator at
`0x8019F9B0..0x801A0040` lives in `src/resource/bres.rs` and the EGG archive
object at `0x802B7AC0..0x802B7E50` lives in `src/resource/archive.rs`.
**Holder lifecycle, mount completion and
compressed access** still ended in opaque addresses, so this wave translates the
two layers those flows call.

The selection covers the two layers that the previous resource passes still
called only by address:

- `0x802B8A70..0x802B9290` — the **archive heap**. Wave 15's
  `arc_object_read_entry` called `0x802B8CC0` and `0x802B8F70`; wave 24's mount
  executor called `0x802B8CC0` through the EGG stream base. This pass supplies
  the allocator, the free path, the owner lookup, the swap/CacheStore pair and
  the Yaz0 wrapper constructor.
- `0x802BA2B0..0x802BA800` — the **EGG mount-stream base class**. Wave 24
  documented `mount_stream_construct` (`0x8016B1C0`) and
  `mount_stream_manager_init` (`0x8016B220`) but deferred their base class as
  "outside this lane". This pass adds the base constructors, the destructors,
  the thread lookup, the global list and the thread-switch callback.

## Function table

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x802B8A70` | `lz_stream_object_construct` | Bind a Yaz0 decoder to a wrapper | translated | high | `802b8a70_FUN_802b8a70.c` |
| `0x802B8AD0` | `lz_stream_object_delete` | Guarded wrapper free | translated | medium | `802b8ad0_FUN_802b8ad0.c` |
| `0x802B8CC0` | `arc_heap_allocate` | Allocate on the thread's archive heap | translated | high | `802b8cc0_FUN_802b8cc0.c` |
| `0x802B8E80` | `arc_heap_find_by_user` | Find an archive heap by owner word | translated | high | `802b8e80_FUN_802b8e80.c` |
| `0x802B8F20` | `arc_heap_previous` | Read a heap's previous owner | translated | high | `802b8f20_FUN_802b8f20.c` |
| `0x802B8F30` | `arc_heap_current` | Current thread's archive heap | translated | medium | `802b8f30_FUN_802b8f30.c` |
| `0x802B8F70` | `arc_heap_free` | Free one block to an archive heap | translated | high | `802b8f70_FUN_802b8f70.c` |
| `0x802B9010` | `arc_heap_release_clients` | Drain a heap's client list | translated | high | `802b9010_FUN_802b9010.c` |
| `0x802B9170` | `arc_heap_exchange_current` | Swap the current heap under a mutex | translated | high | `802b9170_FUN_802b9170.c` |
| `0x802B91D0` | `arc_heap_store_current` | Swap and write the slot back | translated | high | `802b91d0_FUN_802b91d0.c` |
| `0x802B9280` | `arc_heap_free_object` | Free through the thread's heap | translated | high | `802b9280_FUN_802b9280.c` |
| `0x802BA2B0` | `mount_stream_default_align` | Stream allocation alignment `0x80` | translated | high | `802ba2b0_FUN_802ba2b0.c` |
| `0x802BA2D0` | `mount_stream_base_construct` | Base stream without a thread | translated | high | `802ba2d0_FUN_802ba2d0.c` |
| `0x802BA330` | `mount_stream_base_destroy` | Destroy the base stream | translated | high | `802ba330_FUN_802ba330.c` |
| `0x802BA3B0` | `mount_stream_base_init` | Allocate stack/thread and start it | translated | high | `802ba3b0_FUN_802ba3b0.c` |
| `0x802BA480` | `mount_stream_borrow` | Stream over a caller-owned thread | translated | high | `802ba480_FUN_802ba480.c` |
| `0x802BA500` | `mount_stream_destroy` | Stop and free a threaded stream | translated | high | `802ba500_FUN_802ba500.c` |
| `0x802BA5C0` | `mount_stream_find` | Find a stream by thread | translated | high | `802ba5c0_FUN_802ba5c0.c` |
| `0x802BA630` | `mount_stream_list_init` | Create the list and switch callback | translated | high | `802ba630_FUN_802ba630.c` |
| `0x802BA670` | `mount_stream_apply_heap` | Install or clear the heap override | translated | high | `802ba670_FUN_802ba670.c` |
| `0x802BA710` | `mount_stream_switch` | Thread-switch heap juggling | translated | medium | `802ba710_FUN_802ba710.c` |
| `0x802BA800` | `mount_stream_init_queue` | Message ring and list publish | translated | high | `802ba800_FUN_802ba800.c` |

## Cluster notes

### Archive heap and Yaz0 wrapper (`0x802B8A70..0x802B9290`)

The archive object (`0x802B7B00`) and the U8 handle (`0x802B7380`) never call
`operator new` directly. Every member buffer goes through `0x802B8CC0`, which
resolves a **per-thread heap** and calls that heap's virtual allocate slot:

```text
arc_heap_allocate(size, align, heap)
  ├─ current = *(r13 - 0x4870)                       archive heap SDA word
  ├─ thread  = OSGetCurrentThread()                  0x801B4B30
  ├─ stream  = 0x802BA5C0(thread)                    mount stream lookup
  ├─ if stream && stream[+0x3C]: heap = current = stream[+0x3C]
  ├─ expected = *(r13 - 0x4868)
  │    └─ mismatch -> 0x8015F730 x3, hook r13-0x4864, 0x802B9080, return 0
  ├─ heap = heap ?: current
  └─ heap->vtable[+0x14](heap, size, align), then hook r13-0x4860
```

`0x802B8F70` is the mirror free: when no heap is supplied it resolves the owner
with `0x801D3CF0` and `0x802B8E80`, runs the optional free hook (`r13 - 0x485C`)
with `{arg, block}`, then calls the heap's `+0x18` slot. `0x802B9280` is the
one-argument wrapper every deleting destructor uses.

The heap list at `0x80423880` is protected by the mutex at `0x80423890` and
gated by the ready byte `r13 - 0x486C`; `0x802B8E80` compares node `+0x10`.
`0x802B9170` swaps the current heap under that mutex; `0x802B91D0` performs the
same swap without the mutex and adds the data-cache store for the four-byte SDA
slot, which is the form the scheduler callback needs.

`0x802B8A70` is the small wrapper that wave 24's `lz_stream_object_init` /
`lz_stream_object_feed` drive: it stores the decoder at `+0x14`, installs
vtable `0x8034FC80` at `+0x10`, stores the context at `+0x18`, and calls the
decoder's `+0x10` slot to attach the wrapper. `0x802B8AD0` is its deleting
wrapper.

### Mount-stream base (`0x802BA2B0..0x802BA800`)

`0x8016B1C0` (`mount_stream_construct`) calls `0x802BA3B0` first and then writes
the concrete stream vtable `0x80329728`. `0x802BA3B0` is the EGG base
constructor:

```text
mount_stream_base_init(this, stack, ring, priority, heap)
  ├─ this->vtable = 0x8034FE30
  ├─ heap ?: *(r13 - 0x4870)
  ├─ this[+0x04] = heap
  ├─ stack &= ~0x1F; this[+0x38] = stack
  ├─ this[+0x34] = arc_heap_allocate(stack, 0x20, heap)
  ├─ this[+0x08] = arc_heap_allocate(0x318, 0x20, heap)   OSThread
  ├─ OSCreateThread(thread, 0x802BA860, this, stack+size, size, prio, 1)
  ├─ this[+0x3C] = 0; this[+0x40] = 0
  └─ mount_stream_init_queue(this, ring)
       ├─ this[+0x30] = ring
       ├─ this[+0x2C] = arc_heap_allocate(ring << 2, 4, 0)
       ├─ OSInitMessageQueue(this + 0x0C, ring, ring)
       └─ NW4R list append 0x804238A8
```

The base (`0x802BA2D0`/`0x802BA330`, vtable `0x8034FE20`) is the same object
without a queue or thread; it registers itself on the owning heap's client list
at `heap + 0x28`. `0x802BA480` borrows an existing `OSThread`: it copies the
thread's stack window (`+0x304` top minus `+0x308` current) into the stream and
still publishes a queue.

`0x802BA670` is the heap override that `mount_stream_manager_init` applies for
the global DVD heap. On the stream's own thread it swaps the current archive
heap through `0x802B91D0` and caches the old value at `+0x40`; on any other
thread it only caches the value. `0x802BA630` installs `0x802BA710` as the OS
thread-switch callback and saves the previous callback at `r13 - 0x4840`.
`0x802BA710` resolves both switched threads to streams, calls the outgoing
stream's `+0x14` slot (resume), swaps both cached heaps, calls the incoming
stream's `+0x10` slot (suspend), then forwards the pair to the saved callback.

## Unknowns

1. **`0x801D3CF0` is not an exported function name.** Its body searches an
   interval tree (`0x80395DA0`) for the block containing a pointer. The heap
   code uses its nonzero result as the owner word for `0x802B8E80`, so the Rust
   names it the "execution context" resolver and marks the contract medium.
2. **The heap's allocate/free slots are inferred from argument shapes.**
   `+0x14` receives `(heap, size, align)` and `+0x18` receives `(heap, block)`;
   the heap objects themselves are built by `0x802B9360`/`0x802B9F30`, which are
   still outside this lane.
3. **The Yaz0 wrapper's `+0x10` slot is an attach call.** The wrapper stores the
   decoder and context before the call, so the slot is named "attach"; its body
   is not exported.
4. **`0x802BA710`'s two arguments come from a register-save thunk.**
   `0x802DCD74` is the CodeWarrior prologue, not a call; the high/low halves are
   the outgoing and incoming thread pointers. The callback's restore calls to
   `0x802B91D0` drop their argument in the C, so the cached `+0x40` value is
   passed and the inference is called out in the body doc.
5. **`0x802BA670`'s clear branch restores `+0x40`.** The C prints
   `FUN_802b91d0()` with no operand while the surrounding code has already
   loaded the cached value; the Rust passes that value and marks the pass
   medium.
6. **No Nintendo class symbols are claimed.** `EGG::Stream`, `mHeap::FrmHeap`
   and the archive allocator keep address-anchored descriptions; `0x8034FE20`,
   `0x8034FE30` and `0x8034FC80` are vtable addresses, not symbol names.

## Verification

- `cargo check` — exit 0. No diagnostic points at either file in this lane.
- `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
- Ledger: `ledger/agents/wave30-resources.csv` (22 rows: 11
  `resource::archive`, 11 `resource::registry`).
- Body count: `src/resource/archive.rs` 38 -> 49 public bodies;
  `src/resource/registry.rs` 60 -> 71. New bodies: **22**.

## Promotion status

All 22 bodies are in their semantic homes, `src/resource/registry.rs` and
`src/resource/archive.rs`. No row lives in `src/subsystems/`. No `lib.rs`,
`mod.rs`, `Cargo.toml`, `platform.rs`, generated doc or other lane file was
changed.
