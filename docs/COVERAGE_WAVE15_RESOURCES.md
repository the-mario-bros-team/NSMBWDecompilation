# Coverage — wave 15 resource registry and archive additions

## How to read this file

This pass continues the archive-resource work from
`docs/COVERAGE_WAVE4_REGISTRY.md` and `docs/COVERAGE_WAVE7_RESOURCES.md`. It is
organised by the flow, not by address:

1. **Scope** — what was added, where it lives, and how big it is.
2. **Function table** — one row per new body: address, Rust name,
   plain-English role, status, confidence and Ghidra evidence file.
3. **Cluster notes** — the two flows the rows belong to:
   - the mount-command queue behind `info_c` (`0x8016B440..0x8016BF70`),
   - the EGG member-read path and the U8 iterator close
     (`0x802B7ED0..0x802B8150`, `0x801A0040`).
4. **Unknowns** — callbacks, decoder slots and helpers called by address only.
5. **Verification** — compiler, doc linter and ledger.

Addresses are **virtual addresses of the unrelocated `main.dol`** (image base
`0x80004000`), not file offsets. Evidence paths are relative to
`work/ghidra/main-gb/export/`. Status and confidence follow
`docs/READING_GUIDE.md`: `translated` means the complete recorded control flow
is present; `high` confidence means the bytes and call order are explicit, while
`medium` means one callee identity or hidden argument is inferred.

## Scope

| metric | value |
|---|---:|
| functions added | 25 |
| bytes of Ghidra function size | 3,024 |
| `translated` / `partial` / `stub` | 25 / 0 / 0 |
| high confidence | 20 |
| medium confidence | 5 |
| low confidence | 0 |
| new staging modules | 0 |
| files written | `src/resource/registry.rs`, `src/resource/archive.rs`, this document, `ledger/agents/wave15-resources.csv` |

The selection covers two address bands:

- `0x8016B440..0x8016BF70` — the mount-command objects and queue that the
  registry only called by address in wave 4: allocation/free, done/waited
  handshake, list submit/remove/run, the base command, the memory-mount factory
  used by `res_info_set`, its executor, and its archive getter used by
  `res_info_cleanup`;
- `0x802B7ED0..0x802B8150` — the EGG archive member-read path that the mount
  executor drives, plus the no-op U8 directory-iterator close at `0x801A0040`.

## Function table

| Address | Rust | Role (plain English) | Status | Confidence | Evidence |
|---|---|---|---|---|---|
| `0x8016B440` | `mount_cmd_destroy_wait` | Wait for a command and optionally free it | translated | high | `8016b440_FUN_8016b440.c` |
| `0x8016B4A0` | `mount_cmd_alloc` | Allocate a command from the mount heap | translated | high | `8016b4a0_FUN_8016b4a0.c` |
| `0x8016B4B0` | `mount_cmd_free` | Return a command to the mount heap | translated | high | `8016b4b0_FUN_8016b4b0.c` |
| `0x8016B4D0` | `mount_cmd_mark_done` | Set the done byte and wake the waiter | translated | high | `8016b4d0_FUN_8016b4d0.c` |
| `0x8016B4F0` | `mount_cmd_wait_done` | Sleep until the done byte is set | translated | high | `8016b4f0_FUN_8016b4f0.c` |
| `0x8016B540` | `mount_queue_init` | Initialise the global mount queue | translated | high | `8016b540_FUN_8016b540.c` |
| `0x8016B590` | `mount_queue_wake` | Wake the queue consumer | translated | high | `8016b590_FUN_8016b590.c` |
| `0x8016B5A0` | `mount_queue_sleep` | Sleep on the queue | translated | high | `8016b5a0_FUN_8016b5a0.c` |
| `0x8016B5B0` | `mount_queue_peek` | Read the pending head | translated | high | `8016b5b0_FUN_8016b5b0.c` |
| `0x8016B5C0` | `mount_queue_submit` | Run inline or append and wake | translated | high | `8016b5c0_FUN_8016b5c0.c` |
| `0x8016B670` | `mount_queue_remove` | Unlink one command from the queue | translated | high | `8016b670_FUN_8016b670.c` |
| `0x8016B730` | `mount_cmd_run` | Call one command's execute slot | translated | medium | `8016b730_FUN_8016b730.c` |
| `0x8016B770` | `mount_queue_drain` | Infinite consumer loop | translated | high | `8016b770_FUN_8016b770.c` |
| `0x8016B7C0` | `mount_cmd_base_construct` | Write the base command header | translated | high | `8016b7c0_FUN_8016b7c0.c` |
| `0x8016B7E0` | `mount_cmd_destroying_delete` | Virtual deleting destructor | translated | high | `8016b7e0_FUN_8016b7e0.c` |
| `0x8016BA20` | `mount_mem_cmd_construct` | Construct the memory-mount command | translated | high | `8016ba20_FUN_8016ba20.c` |
| `0x8016BCC0` | `mount_mem_cmd_create` | Create and queue a memory mount | translated | high | `8016bcc0_FUN_8016bcc0.c` |
| `0x8016BD60` | `mount_mem_cmd_execute` | Read, mount and publish the archive | translated | medium | `8016bd60_FUN_8016bd60.c` |
| `0x8016BE60` | `mount_mem_cmd_archive` | Read the command's mounted archive | translated | high | `8016be60_FUN_8016be60.c` |
| `0x8016BEF0` | `mount_arc_cmd_construct` | Construct the archive-mount command | translated | high | `8016bef0_FUN_8016bef0.c` |
| `0x8016BF70` | `mount_arc_cmd_create` | Create and queue an archive mount | translated | high | `8016bf70_FUN_8016bf70.c` |
| `0x802B7ED0` | `arc_object_open_entry` | Open an entry on a temporary object and read it | translated | medium | `802b7ed0_FUN_802b7ed0.c` |
| `0x802B7F80` | `arc_object_read_entry` | Allocate and fill one member buffer | translated | medium | `802b7f80_FUN_802b7f80.c` |
| `0x802B8150` | `arc_object_load_member` | Stream one member through a decoder | translated | medium | `802b8150_FUN_802b8150.c` |
| `0x801A0040` | `arc_close_dir_stub` | No-op U8 iterator close, returns 1 | translated | high | `801a0040_FUN_801a0040.c` |

## Cluster notes

### Mount command queue (`0x8016B440..0x8016BF70`)

The registry's `info_c` never touches the DVD layer directly. It stores the
result of the factories below; the queue owns the asynchronous handshake:

```text
res_info_set(info, arc, folder, dir, heap)
  └─ 0x8016BCC0 mount_mem_cmd_create(path, dir, heap)
       ├─ 0x8016BC30 probe path -> entry + tag
       ├─ 0x8016B4A0 alloc 0x1C
       ├─ 0x8016BA20 mount_mem_cmd_construct
       └─ 0x8016B5C0 submit(queue, cmd)
            ├─ current-thread == consumer -> cmd->vtable[3](cmd)
            └─ else lock, append at +0x1C, unlock, 0x8016B590 wake

consumer (0x8016B770)
  └─ 0x8016B5A0 sleep(queue+0x20)
       loop: 0x8016B5B0 peek
             0x8016B670 remove
             0x8016B730 cmd->vtable[3](cmd)

0x8016BD60 execute
  ├─ 0x8016B2A0 read entry (eight arguments)
  ├─ 0x802B75B0 mount bytes as a U8 archive
  ├─ on failure: 0x802B9280 free the temporary read buffer
  ├─ store archive at +0x10
  └─ 0x8016B4D0 done byte + wake queue+0x28

res_info_cleanup(info)
  └─ 0x8016BE60 archive = *(*(cmd + 0x10) + 0x18)
       then 0x8016B7E0 -> vtable[2](cmd, 1) -> 0x8016B440 wait + free
```

The queue object layout is `{mutex @0x00, head @0x18, tail @0x1C,
work thread-queue @0x20, done thread-queue @0x28}`. `0x8016B540` publishes it at
`r13 - 0x5254` (`0x8042A42C`); `0x8016B4A0`/`4B0` use the heap word at
`r13 - 0x5260` (`0x8042A420`).

### EGG member read (`0x802B7ED0..0x802B8150`)

After the mount executor has opened a disc entry, the read path is:

```text
0x802B7ED0 open_entry(entry, dst, heap, dir, offset, len_out, size_out)
  ├─ 0x802B7B00 construct 0xF8-byte temporary archive object
  ├─ 0x802B7C30 open the entry
  ├─ 0x802B7F80 read_entry(object, dst, ...)
  │    ├─ vtable+0x1C -> member size
  │    ├─ 0x802B8CC0 allocate aligned_size - offset when dst is null
  │    ├─ offset != 0 -> 0x801CAB20 probe read 0x20 + 0x801AC440 invalidate
  │    ├─ 0x801CAB20 read aligned_size - offset, retry on negative
  │    └─ *length_out = read length
  └─ 0x802B7B50 destroy (deleting flag -1)
```

`0x802B8150` is the decoder-aware variant: it reads the compressed member into
a temporary allocation, hands it to decoder slot `+0x14`, frees the temporary,
allocates the destination from the returned expanded size, then feeds the member
through the decoder in `chunk`-sized reads (slot `+0x0C`) until the decoder
reports completion or a read fails.

## Unknowns

1. **The mount executor is called by address.** `0x8016B2A0` is not part of
   either destination file. Its eight-argument order
   `(entry, 0, heap, direction, 0, size_out, 0, mode)` is reconstructed from its
   own body and its registry caller, so `mount_mem_cmd_execute` is `medium`.
2. **The archive decoder object is opaque.** In `0x802B8150`, slots `+0x08`,
   `+0x0C`, `+0x10` and `+0x14` are called in the recorded order, but their
   names (destination setup, feed, position/size, input) are inferred only from
   argument shapes. The allocator object's `+0x14`/`+0x18` slots are the
   matching allocate/free pair.
3. **The archive-read hook at `r13 - 0x4878` is unexported.**
   `0x802B7F80` calls it with a six-word context before the read; its body is not
   in the export, so only the context layout is reproduced.
4. **`0x8016B730`'s virtual-call `this` is inferred.** Ghidra prints the
   indirect call with no arguments; the command pointer used to fetch the vtable
   is passed as `this`, matching the ABI and the queue's use of the slot.
5. **`0x8016B770`'s remove argument is inferred.** The C prints
   `FUN_8016b670(param_1)` but the node is still in `r4`; the body removes
   `local_18[0]`, so the Rust call passes `(queue, node)`.
6. **No Nintendo class symbols are claimed.** `info_c`, `dRes_c` and the EGG
   archive object keep descriptive names from the earlier waves; the
   mount-command names are address-anchored descriptions, not symbol claims.

## Verification

- `cargo check` — exit 0. The remaining warnings are all in other lanes
  (`src/runtime/bt.rs`, `src/runtime/input.rs`); neither file in this lane
  produced a diagnostic.
- `node tools/doc-lint.mjs src` — `ok: true`, 68 files, 0 violations.
- Ledger: `ledger/agents/wave15-resources.csv` (25 rows: 21
  `resource::registry`, 4 `resource::archive`).

## Promotion status

All 25 bodies are in their semantic homes, `src/resource/registry.rs` and
`src/resource/archive.rs`. No row lives in `src/subsystems/`. No `lib.rs`,
`mod.rs`, `Cargo.toml`, `platform.rs`, generated doc or other lane file was
changed.
