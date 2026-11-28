//! Archive-resource registry (`dRes_c` / `info_c`) - `0x800DE970..0x800DF840`
//! plus the mount-command layer at `0x8016B440..0x8016BF70`.
//!
//! ## What this module covers
//!
//! This is the game's **resource-archive registry**: a fixed-stride table of
//! 0x40-byte archive holders (`info_c`), each of which owns one mounted U8
//! archive ([`crate::resource::archive`]) and the list of its members. The
//! functions here create the table, look a holder up by name, refcount it,
//! finish a pending DVD mount, build the member-pointer array, and hand typed
//! member pointers back to the rest of the game.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x800DE970` | 124 B | [`res_info_destroy`] | wait for the DVD command, clean up, optionally free |
//! | `0x800DE9F0` | 212 B | [`res_info_cleanup`] | adopt the command's archive, free heap/binaries, unmount |
//! | `0x800DEAD0` | 196 B | [`res_info_set`] | build `/<folder>/<name>.arc` and start a mount |
//! | `0x800DEBA0` | 312 B | [`res_info_set_res`] | finish a mount and build the member table |
//! | `0x800DEDB0` | 408 B | [`res_info_load_res`] | allocate `mpFiles` and enumerate the archive |
//! | `0x800DEF50` | 20 B | [`dres_construct`] | zero the registry header |
//! | `0x800DEF70` | 116 B | [`dres_destroy`] | destroy the holder array and free the registry |
//! | `0x800DEFF0` | 136 B | [`dres_init`] | allocate and construct the holder array |
//! | `0x800DF080` | 168 B | [`dres_set_res`] | find-or-create a holder and bump its refcount |
//! | `0x800DF1E0` | 140 B | [`dres_get_res`] | member pointer + size by path (logging open) |
//! | `0x800DF270` | 292 B | [`dres_get_res_compressed`] | member pointer + size/compression, `.LZ` fallback |
//! | `0x800DF3A0` | 16 B | [`dres_copy_res`] | copy one uncompressed member |
//! | `0x800DF3B0` | 16 B | [`dres_copy_res_compressed`] | copy or LZ-decompress one member |
//! | `0x800DF3C0` | 128 B | [`dres_get_res_silently`] | member pointer by path, no warning |
//! | `0x800DF440` | 148 B | [`dres_get_res_silently_size`] | member pointer + size, no warning |
//! | `0x800DF4E0` | 132 B | [`dres_sync_all_res`] | advance one pending mount and report progress |
//! | `0x800DF600` | 52 B | [`dres_new_res_info`] | first free holder (refcount zero) |
//! | `0x800DF680` | 96 B | [`dres_mng_construct`] | construct the singleton and install 300 holders |
//! | `0x800DF6E0` | 100 B | [`dres_mng_destroy`] | clear the singleton and destroy the registry |
//! | `0x800DF7D0` | 112 B | [`dres_mng_set_res_array`] | schedule an array of archive names |
//! | `0x800DF840` | 28 B | [`dres_mng_set_res`] | register one folder/name pair through the manager |
//!
//! Three functions of the same family already live in
//! [`crate::resource::bres`]: `0x800DF570` (`getResInfo`), `0x800DF640`
//! (`getResInfoLoaded`) and `0x800DF180` (the `g3d/*.brres` wrapper). The code
//! below calls the first two instead of duplicating them.
//!
//! ## Mount-command cluster (wave 15)
//!
//! The registry never talks to the DVD layer directly: a holder stores the
//! result of the two factories below. Wave 15 supplied the command objects and
//! the queue that the earlier registry bodies only called by address.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x8016B440` | 96 B | [`mount_cmd_destroy_wait`] | wait for a command, then free it |
//! | `0x8016B4A0` | 12 B | [`mount_cmd_alloc`] | allocate a command from the mount heap |
//! | `0x8016B4B0` | 8 B | [`mount_cmd_free`] | return a command to the mount heap |
//! | `0x8016B4D0` | 20 B | [`mount_cmd_mark_done`] | set the done byte and wake the waiter |
//! | `0x8016B4F0` | 68 B | [`mount_cmd_wait_done`] | sleep until the done byte is set |
//! | `0x8016B540` | 80 B | [`mount_queue_init`] | initialise the global queue |
//! | `0x8016B590` | 8 B | [`mount_queue_wake`] | wake the queue consumer |
//! | `0x8016B5A0` | 8 B | [`mount_queue_sleep`] | sleep on the queue |
//! | `0x8016B5B0` | 8 B | [`mount_queue_peek`] | read the pending head |
//! | `0x8016B5C0` | 168 B | [`mount_queue_submit`] | run inline or append and wake |
//! | `0x8016B670` | 184 B | [`mount_queue_remove`] | unlink one command |
//! | `0x8016B730` | 52 B | [`mount_cmd_run`] | call the command's execute slot |
//! | `0x8016B770` | 76 B | [`mount_queue_drain`] | infinite consumer loop |
//! | `0x8016B7C0` | 28 B | [`mount_cmd_base_construct`] | base command header |
//! | `0x8016B7E0` | 28 B | [`mount_cmd_destroying_delete`] | virtual deleting destructor |
//! | `0x8016BA20` | 116 B | [`mount_mem_cmd_construct`] | construct the memory-mount command |
//! | `0x8016BCC0` | 160 B | [`mount_mem_cmd_create`] | create + queue a memory mount |
//! | `0x8016BD60` | 248 B | [`mount_mem_cmd_execute`] | read, mount and publish the archive |
//! | `0x8016BE60` | 24 B | [`mount_mem_cmd_archive`] | read the command's mounted archive |
//! | `0x8016BEF0` | 120 B | [`mount_arc_cmd_construct`] | construct the archive-mount command |
//! | `0x8016BF70` | 156 B | [`mount_arc_cmd_create`] | create + queue an archive mount |
//!
//! `0x8016B7E0` is the exact address `res_info_cleanup` called before this pass;
//! `0x8016BCC0` is the factory `res_info_set` called. Their bodies now live
//! next to their only resource-registry callers.
//!
//! ## Mount lookup, stream manager and path probe (wave 24)
//!
//! The wave-15 queue above needs three more layers to be usable end to end:
//! the table that maps a magic/extension tag to a mounted archive object, the
//! threaded stream that executes reads, and the path probe that translates a
//! plain disc path into a DVD entry number.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x8016B090` | 12 B | [`mount_archive_table_set`] | publish the archive table bounds |
//! | `0x8016B0A0` | 16 B | [`mount_archive_table_init`] | install the shipped one-slot table |
//! | `0x8016B0B0` | 56 B | [`mount_archive_find`] | find an archive object by tag byte |
//! | `0x8016B0F0` | 64 B | [`mount_archive_acquire`] | call the found object's acquire slot |
//! | `0x8016B130` | 56 B | [`mount_archive_release`] | call the found object's release slot |
//! | `0x8016B170` | 8 B | [`mount_thread`] | read the mount worker thread |
//! | `0x8016B180` | 8 B | [`mount_default_heap`] | read the default mount heap |
//! | `0x8016B1C0` | 76 B | [`mount_stream_construct`] | construct the threaded read stream |
//! | `0x8016B220` | 124 B | [`mount_stream_manager_init`] | create the stream and resume its thread |
//! | `0x8016B2A0` | 344 B | [`mount_execute`] | read one entry, optionally through the decoder |
//! | `0x8016B400` | 8 B | [`mount_entry_lookup`] | probe a path without a tag output |
//! | `0x8016B800` | 88 B | [`mount_call_cmd_destroy`] | wait for a call command, then free |
//! | `0x8016B860` | 100 B | [`mount_call_cmd_construct`] | construct a "call one function" command |
//! | `0x8016B8D0` | 120 B | [`mount_call_cmd_create`] | create and queue that command |
//! | `0x8016BAA0` | 364 B | [`mount_probe_path_variants`] | try every registered extension |
//! | `0x8016BC10` | 8 B | [`mount_set_path_fallback`] | enable/disable the fallback probe |
//! | `0x8016BC20` | 8 B | [`mount_path_fallback`] | read the fallback flag |
//! | `0x8016BC30` | 128 B | [`mount_probe_path`] | path -> DVD entry, then fallback |
//!
//! ## Mount stream base (wave 30)
//!
//! Wave 24 translated the stream manager and executor but stopped at the EGG
//! `Stream` base they construct. This pass adds the base class at
//! `0x802BA2B0..0x802BA800`, so the thread, stack, message queue and heap
//! override that `mount_stream_manager_init` installs are explicit.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x802BA2B0` | 8 B | [`mount_stream_default_align`] | alignment constant `0x80` |
//! | `0x802BA2D0` | 84 B | [`mount_stream_base_construct`] | base stream, no thread |
//! | `0x802BA330` | 116 B | [`mount_stream_base_destroy`] | destroy the base stream |
//! | `0x802BA3B0` | 208 B | [`mount_stream_base_init`] | allocate stack/thread and start it |
//! | `0x802BA480` | 116 B | [`mount_stream_borrow`] | stream over a caller-owned thread |
//! | `0x802BA500` | 188 B | [`mount_stream_destroy`] | stop and free a threaded stream |
//! | `0x802BA5C0` | 100 B | [`mount_stream_find`] | find a stream by thread |
//! | `0x802BA630` | 60 B | [`mount_stream_list_init`] | create the list + switch callback |
//! | `0x802BA670` | 156 B | [`mount_stream_apply_heap`] | install/clear the heap override |
//! | `0x802BA710` | 236 B | [`mount_stream_switch`] | thread-switch heap juggling |
//! | `0x802BA800` | 92 B | [`mount_stream_init_queue`] | message ring + list publish |
//!
//! ## Stream/frame-heap wrapper family (wave 39)
//!
//! Wave 38 published the heaps themselves; this pass adds the per-object
//! wrappers that put a frame heap around one allocation. A single stream
//! object (constructed over the EGG stream base) is published in
//! `r13 - 0x5298`, and the heap-view family attaches an EGG heap at `+0x14`
//! with the heap's `+0x0C`/`+0x10`/`+0x1C` slots driving the kind test,
//! attach and detach.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x8016A480` | 72 B | [`fn_8016a480`] | construct stream base + publish singleton |
//! | `0x8016A5D0` | 8 B | [`fn_8016a5d0`] | read the singleton |
//! | `0x8016A5E0` | 76 B | [`fn_8016a5e0`] | LZ-view init with the singleton |
//! | `0x8016A630` | 88 B | [`fn_8016a630`] | LZ-view deleting destructor |
//! | `0x8016A690` | 80 B | [`fn_8016a690`] | attach a heap to the view |
//! | `0x8016A700` | 56 B | [`fn_8016a700`] | allocate + construct the singleton |
//! | `0x8016A740` | 60 B | [`fn_8016a740`] | alternate view constructor |
//! | `0x8016A780` | 108 B | [`fn_8016a780`] | alternate view deleter |
//! | `0x8016A7F0` | 132 B | [`fn_8016a7f0`] | create frame heap + attach |
//! | `0x8016A880` | 92 B | [`fn_8016a880`] | detach heap back to the singleton |
//! | `0x8016A8E0` | 116 B | [`fn_8016a8e0`] | adjust frame heap + report size |
//! | `0x8016A960` | 76 B | [`fn_8016a960`] | open frame heap + save current |
//! | `0x8016A9B0` | 100 B | [`fn_8016a9b0`] | restore heap + adjust |
//!
//! ## Sound-bank tables and element accessors (wave 40)
//!
//! The 0x80160000 bucket is only partly registry code. This pass takes the
//! table layer that sits directly above the wave-39 stream wrappers, plus the
//! accessor set for one `0x38`-stride array element. The five intrusive lists
//! at `0x80377A10..0x80377A6C` and the eight-bucket hash table at `0x80377A70`
//! use the same node shape as [`mount_archive_find`]; the stream view they read
//! through is the LZ view from wave 39.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x801628C0` | 92 B | [`fn_801628c0`] | zeroed default-heap allocation |
//! | `0x80162920` | 12 B | [`fn_80162920`] | free on the default heap |
//! | `0x80162930` | 120 B | [`fn_80162930`] | update pass + queue a finished bank |
//! | `0x801629B0` | 112 B | [`fn_801629b0`] | first bank with `+0x0A == 0` |
//! | `0x80162A50` | 20 B | [`fn_80162a50`] | stash four construction arguments |
//! | `0x80162B40` | 92 B | [`fn_80162b40`] | ordered insert by `u16` key |
//! | `0x80162BB0` | 116 B | [`fn_80162bb0`] | free a hash-bucket list |
//! | `0x80162C30` | 44 B | [`fn_80162c30`] | find a bucket node by descriptor word |
//! | `0x80162CA0` | 72 B | [`fn_80162ca0`] | unlink a bank from its list |
//! | `0x80162CF0` | 16 B | [`fn_80162cf0`] | low three bits of the word behind `+0x3C` |
//! | `0x80162D00` | 72 B | [`fn_80162d00`] | bank id -> descriptor payload |
//! | `0x80162D50` | 84 B | [`fn_80162d50`] | `u16` key -> payload |
//! | `0x80162DB0` | 84 B | [`fn_80162db0`] | `u8` sub-key -> payload |
//! | `0x80162E10` | 192 B | [`fn_80162e10`] | free the bank lists by teardown mask |
//! | `0x80162ED0` | 288 B | [`fn_80162ed0`] | reset the five lists and hash table |
//! | `0x80163000` | 132 B | [`fn_80163000`] | free one bank list |
//! | `0x80163090` | 104 B | [`fn_80163090`] | find by the `u16` payload key |
//! | `0x80163100` | 104 B | [`fn_80163100`] | find by the `u8` payload sub-key |
//! | `0x80163170` | 16 B | [`fn_80163170`] | reset the sorted `ut::List` |
//! | `0x80163180` | 148 B | [`fn_80163180`] | build the LZ view through wave 39 |
//! | `0x80163280` | 120 B | [`fn_80163280`] | virtual draw pass over the sorted list |
//! | `0x801634C0` | 8 B | [`fn_801634c0`] | read the published LZ view |
//! | `0x80168060` | 28 B | [`fn_80168060`] | element class tag |
//! | `0x80168080` | 20 B | [`fn_80168080`] | element vtable slot `+0x20` |
//! | `0x801680A0` | 20 B | [`fn_801680a0`] | element vtable slot `+0x1C` |
//! | `0x801680C0` | 20 B | [`fn_801680c0`] | element vtable slot `+0x2C` |
//! | `0x801680E0` | 20 B | [`fn_801680e0`] | element vtable slot `+0x28` |
//!
//! ## Handle, intrusive list and context wrappers (wave 41)
//!
//! Wave 40 left several small leaf routines in the 0x80160000 bucket
//! untranslated because they looked like bare wrappers. This pass takes them in
//! five contiguous runs: the 0x48-byte **handle** whose vtable calls and SDA
//! scratch fields sit at `0x80160020..0x801602D0`; the handle's lock helpers
//! at `0x80160930..0x801609D0`; the **doubly-linked list primitives** at
//! `0x80160F10..0x80161020` that wave 40 previously called only by address;
//! the **live-context facade** at `0x80164B10..0x80164D50`; and the
//! `0x38`-stride **element dispatch trampolines** at `0x801662E0..0x80166360`
//! that forward into the wave-40 accessor set.
//!
//! List nodes use the same `{prev, next}` shape as [`mount_archive_find`]:
//! `+0x00` is the previous node, `+0x04` the next node, and a list header is
//! `{head, tail}`. The live-context facade reads the object published in
//! `r13 - 0x52B4`; the state table starts at `+0xF8` with `0x20` slots of
//! `0x10C` bytes each, and `+0xF4` holds the current slot index.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80160020` | 36 B | [`fn_80160020`] | probe flag, dispatch `+0x20` |
//! | `0x80160080` | 132 B | [`fn_80160080`] | construct the 0x48-byte handle |
//! | `0x80160110` | 104 B | [`fn_80160110`] | close and optionally free a handle |
//! | `0x80160190` | 156 B | [`fn_80160190`] | publish + allocate the handle buffer |
//! | `0x80160240` | 132 B | [`fn_80160240`] | lower the two recorded minima |
//! | `0x801602D0` | 40 B | [`fn_801602d0`] | sum `u16` half-words |
//! | `0x80160930` | 96 B | [`fn_80160930`] | release lock and optionally free |
//! | `0x80160990` | 52 B | [`fn_80160990`] | acquire the lock word |
//! | `0x801609D0` | 56 B | [`fn_801609d0`] | release and clear the lock word |
//! | `0x80160F10` | 80 B | [`fn_80160f10`] | insert a node after a peer |
//! | `0x80160F60` | 128 B | [`fn_80160f60`] | unlink a node |
//! | `0x80160FE0` | 64 B | [`fn_80160fe0`] | append a node at the tail |
//! | `0x80161020` | 64 B | [`fn_80161020`] | insert a node at the head |
//! | `0x80164B10` | 8 B | [`fn_80164b10`] | read the live context |
//! | `0x80164B20` | 12 B | [`fn_80164b20`] | read one context state |
//! | `0x80164B30` | 8 B | [`fn_80164b30`] | read the current state |
//! | `0x80164B40` | 12 B | [`fn_80164b40`] | read the current slot index |
//! | `0x80164B60` | 12 B | [`fn_80164b60`] | context `+0x2878` address |
//! | `0x80164B70` | 16 B | [`fn_80164b70`] | slot object by index |
//! | `0x80164B90` | 124 B | [`fn_80164b90`] | close one slot + clear aux bit |
//! | `0x80164C50` | 128 B | [`fn_80164c50`] | key lookup in a slot's map |
//! | `0x80164CD0` | 128 B | [`fn_80164cd0`] | shut down one slot + context |
//! | `0x80164D50` | 8 B | [`fn_80164d50`] | request context teardown |
//! | `0x801662E0` | 16 B | [`fn_801662e0`] | element dispatch `+0x20` |
//! | `0x801662F0` | 16 B | [`fn_801662f0`] | element value set |
//! | `0x80166300` | 16 B | [`fn_80166300`] | element dispatch `+0x2C` |
//! | `0x80166310` | 16 B | [`fn_80166310`] | element dispatch `+0x28` |
//! | `0x80166320` | 16 B | [`fn_80166320`] | element range test |
//! | `0x80166340` | 20 B | [`fn_80166340`] | element `+0x34` byte |
//! | `0x80166360` | 20 B | [`fn_80166360`] | element `+0x28` float |
//!
//! ## Context vtable, element wrapper and table reset (wave 45)
//!
//! The 0x80160000 bucket still holds the small object lifecycle helpers that
//! surround the live-context facade. This pass takes three contiguous groups
//! plus the four-entry table reset:
//! the **live-context vtable wrappers** at `0x80164E50..0x80164F50`, the
//! **element wrapper constructors/destructors** at `0x801693A0..0x8016A3A0`,
//! the four identical **copy-on-success frees** at
//! `0x8016AF90..0x8016B050`, and the **matrix/table reset** helpers at
//! `0x8016F150..0x8016F7D0`.
//!
//! The element wrappers use the same `+0x30` owner chain as
//! [`fn_80169c20`]'s neighbours: `+4` is the nested block, `+0x24`/`+0x28`
//! the element pair, and `+0x2C` the virtual target. The table helpers clear
//! `0x18`-byte slots from `0x80377D08`, the same entries that
//! [`fn_8016f460`] indexes at `0x18` stride.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80164E50` | 32 B | [`fn_80164e50`] | context vtable `+0x34` with count |
//! | `0x80164E70` | 80 B | [`fn_80164e70`] | pop context objects via `+0x38` |
//! | `0x80164EC0` | 60 B | [`fn_80164ec0`] | update when either SDA slot is set |
//! | `0x80164F00` | 76 B | [`fn_80164f00`] | looked-up `+0xC` word or `-1` |
//! | `0x80164F50` | 68 B | [`fn_80164f50`] | reset channel 0 then 16 channels |
//! | `0x801693A0` | 76 B | [`fn_801693a0`] | release the `+0x24` buffer |
//! | `0x80169560` | 84 B | [`fn_80169560`] | construct `0x80329658` sub-object |
//! | `0x801695C0` | 64 B | [`fn_801695c0`] | copy-on-success free |
//! | `0x80169AC0` | 92 B | [`fn_80169ac0`] | forward one element, reset owner |
//! | `0x80169C20` | 92 B | [`fn_80169c20`] | destroy element buffer + owner |
//! | `0x80169C90` | 64 B | [`fn_80169c90`] | advance unfinished nested object |
//! | `0x80169CD0` | 68 B | [`fn_80169cd0`] | construct `0x80329638` wrapper |
//! | `0x80169D20` | 104 B | [`fn_80169d20`] | destroy wrapper and optionally free |
//! | `0x8016A1A0` | 120 B | [`fn_8016a1a0`] | set the two-state `+4` flag |
//! | `0x8016A2A0` | 72 B | [`fn_8016a2a0`] | apply a value to the audio handle |
//! | `0x8016A2F0` | 68 B | [`fn_8016a2f0`] | construct `0x80329688` object |
//! | `0x8016A340` | 88 B | [`fn_8016a340`] | reset and optionally free |
//! | `0x8016A3A0` | 180 B | [`fn_8016a3a0`] | create the `+4` handle by mode |
//! | `0x8016AF90` | 64 B | [`fn_8016af90`] | copy-on-success free |
//! | `0x8016AFD0` | 64 B | [`fn_8016afd0`] | copy-on-success free |
//! | `0x8016B010` | 64 B | [`fn_8016b010`] | copy-on-success free |
//! | `0x8016B050` | 64 B | [`fn_8016b050`] | copy-on-success free |
//! | `0x8016F150` | 56 B | [`fn_8016f150`] | fill a 12-word matrix |
//! | `0x8016F500` | 68 B | [`fn_8016f500`] | clear one `0x18`-byte table slot |
//! | `0x8016F550` | 60 B | [`fn_8016f550`] | clear all four slots |
//! | `0x8016F5D0` | 100 B | [`fn_8016f5d0`] | start a slot task, clear on `-1` |
//! | `0x8016F6E0` | 64 B | [`fn_8016f6e0`] | copy-on-success free |
//! | `0x8016F7D0` | 92 B | [`fn_8016f7d0`] | init entry, mark both bytes live |
//!
//! ## List, sound-object, element and interpolator leaves (wave 52)
//!
//! Wave 45 finished the context and table resets; this pass takes the leaf
//! routines that sit between them in the `0x80160000` bucket: the **intrusive
//! list walk/append/remove set** at `0x80161700..0x80161C40`, the **sound
//! object** calls that forward through `+0x10`/`+0x94` at
//! `0x80163A60..0x801644D0`, the **element cast/teardown** pair at
//! `0x80168100..0x8016A090`, and the **interpolator value/state** leaves at
//! `0x80168B40..0x80168D80`.
//!
//! The list nodes are `{next, prev, payload, key}` (`+0x00`, `+0x04`,
//! `+0x08`, `+0x0C`); the key lookup and unlink calls are the same shape as
//! the wave-40 sound-bank lists. The element wrappers keep the cast target at
//! `+0x04` and reuse the wave-40 vtable idiom. The interpolator keeps its
//! current value at `+0x04`, its step at `+0x0C`, and the two reset `f32`
//! words from `r2 - 0x53D8` / `r2 - 0x53D4`.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80161700` | 60 B | [`fn_80161700`] | first payload in a node chain |
//! | `0x80161750` | 160 B | [`fn_80161750`] | append a node and store its key |
//! | `0x80161800` | 180 B | [`fn_80161800`] | unlink a node if its key matches |
//! | `0x801618C0` | 92 B | [`fn_801618c0`] | clear a whole node list |
//! | `0x80161920` | 140 B | [`fn_80161920`] | pop array entries via `+0x10` |
//! | `0x801619B0` | 188 B | [`fn_801619b0`] | advance the `u16` state machine |
//! | `0x80161A70` | 88 B | [`fn_80161a70`] | run until the state leaves `1` |
//! | `0x80161C40` | 120 B | [`fn_80161c40`] | unlink a sound bank's list + free |
//! | `0x80163A60` | 36 B | [`fn_80163a60`] | forward the `+0x94` bit to `+0x38` |
//! | `0x80163A90` | 108 B | [`fn_80163a90`] | query transform and call `+0x30` |
//! | `0x80163C70` | 196 B | [`fn_80163c70`] | create a child and publish it |
//! | `0x80163E10` | 72 B | [`fn_80163e10`] | map a recursive result to a flag |
//! | `0x80163FB0` | 60 B | [`fn_80163fb0`] | release `+0x14` and return `1` |
//! | `0x80164080` | 152 B | [`fn_80164080`] | call `+0x5C` on matching nodes |
//! | `0x80164120` | 144 B | [`fn_80164120`] | apply a double to matching nodes |
//! | `0x801642C0` | 228 B | [`fn_801642c0`] | store a float via two key lookups |
//! | `0x80164460` | 108 B | [`fn_80164460`] | re-apply value and set enable bit |
//! | `0x801644D0` | 24 B | [`fn_801644d0`] | unpack descriptor, forward value |
//! | `0x80168100` | 116 B | [`fn_80168100`] | element teardown + optional free |
//! | `0x80168180` | 168 B | [`fn_80168180`] | cast `+0x04` and install it |
//! | `0x80168230` | 80 B | [`fn_80168230`] | cast then copy three words out |
//! | `0x8016A030` | 96 B | [`fn_8016a030`] | destroy `+0x04` child + optional free |
//! | `0x8016A090` | 68 B | [`fn_8016a090`] | release the `+0x04` child |
//! | `0x80168B40` | 60 B | [`fn_80168b40`] | construct with two reset constants |
//! | `0x80168B90` | 32 B | [`fn_80168b90`] | restore the five value fields |
//! | `0x80168CB0` | 24 B | [`fn_80168cb0`] | is the value back at reset? |
//! | `0x80168D10` | 112 B | [`fn_80168d10`] | buffer view + two published words |
//! | `0x80168D80` | 88 B | [`fn_80168d80`] | reset interpolator + optional free |
//!
//! ## Bucket stragglers: tree lists, actor state and element class twins (wave 64)
//!
//! Wave 52 closed the registry-adjacent leaves; this pass takes the next
//! contiguous run of reachable `0x80160000` code that the earlier waves left
//! behind: the **tree/list primitive pair** at `0x80161580`/`0x80161620`, the
//! **actor enable/disable state machine** at `0x80161EF0..0x80162B00`, the
//! **NW4R sorted-list insert** at `0x80163850`, and **three instantiations of
//! the `0x38`-stride element class** at `0x80165820..0x80167AB0` (size query,
//! counted size query, child release, per-element release and deleting
//! destructor) plus the shared base teardown at `0x80167EC0`. Four small
//! query/init leaves at `0x8016C6F0..0x8016FF60` close the pass.
//!
//! Tree nodes are `{parent +0x00, first child +0x04, prev +0x08, next +0x0C}`.
//! The element class keeps its native child at `+0x04`, its heap block at
//! `+0x08`, the `0x38`-stride array at `+0x28` and its child chain at `+0x14`.
//! The three class instantiations differ only in the allocator query address
//! (`0x80243D20`, `0x80245500`, `0x80246B00`) and the vtable word, so they are
//! written as explicit twins rather than a parameterised helper.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80161530` | 48 B | [`fn_80161530`] | zero a four-word list header |
//! | `0x80161580` | 136 B | [`fn_80161580`] | append a node under a parent |
//! | `0x80161620` | 152 B | [`fn_80161620`] | unlink a childless node |
//! | `0x80161EF0` | 104 B | [`fn_80161ef0`] | queue gate for the actor |
//! | `0x801622E0` | 552 B | [`fn_801622e0`] | run the enable/disable state machine |
//! | `0x80162510` | 116 B | [`fn_80162510`] | mark a node and its child chain |
//! | `0x80162B00` | 24 B | [`fn_80162b00`] | resolve a node's `+0x10` descriptor |
//! | `0x80163850` | 144 B | [`fn_80163850`] | sorted-list insert by `+0x0C` |
//! | `0x801651F0` | 132 B | [`fn_801651f0`] | forward a value to `+0x04` and rebuild |
//! | `0x80165820` | 124 B | [`fn_80165820`] | class-A instance size |
//! | `0x80165A80` | 92 B | [`fn_80165a80`] | class-A child release |
//! | `0x80165B50` | 224 B | [`fn_80165b50`] | class-A counted size |
//! | `0x80165E70` | 108 B | [`fn_80165e70`] | class-A deleting destructor |
//! | `0x80166280` | 96 B | [`fn_80166280`] | release one class-A element |
//! | `0x80166420` | 124 B | [`fn_80166420`] | class-B instance size |
//! | `0x80166680` | 92 B | [`fn_80166680`] | class-B child release |
//! | `0x80166750` | 224 B | [`fn_80166750`] | class-B counted size |
//! | `0x80166A70` | 108 B | [`fn_80166a70`] | class-B deleting destructor |
//! | `0x80166E80` | 96 B | [`fn_80166e80`] | release one class-B element |
//! | `0x80167000` | 124 B | [`fn_80167000`] | class-C instance size |
//! | `0x80167270` | 92 B | [`fn_80167270`] | class-C child release |
//! | `0x80167340` | 224 B | [`fn_80167340`] | class-C counted size |
//! | `0x801676A0` | 108 B | [`fn_801676a0`] | class-C deleting destructor |
//! | `0x80167AB0` | 96 B | [`fn_80167ab0`] | release one class-C element |
//! | `0x80167EC0` | 108 B | [`fn_80167ec0`] | shared base destructor |
//! | `0x80167F30` | 100 B | [`fn_80167f30`] | release the `+0x04`/`+0x08` fields |
//! | `0x8016C6F0` | 64 B | [`fn_8016c6f0`] | conditional free, owner A |
//! | `0x8016CFD0` | 192 B | [`fn_8016cfd0`] | copy a live descriptor into `+0x7C` |
//! | `0x8016DCA0` | 64 B | [`fn_8016dca0`] | conditional free, owner B |
//! | `0x8016FF60` | 92 B | [`fn_8016ff60`] | initialise a UI entry |
//!
//! ## Element entry, cast and list-release layer (wave 78)
//!
//! Wave 64 wrote the class size queries, destructors and per-element release
//! twins, but left every body they reached by absolute address unwritten. This
//! pass supplies those bodies: the **cast helper family** at
//! `0x80168400..0x80168520`, the **class teardown / per-entry release** pairs at
//! `0x80165EE0`, `0x80166AE0`, `0x80167710` and their `0x80166xxx`/`0x80167940`
//! twins, the **sound-bank list transition** at `0x80161DA0`, the **three-row
//! transform installers** at `0x80161E40..0x80162220`, and the
//! `0x80377C3C` **intrusive-list teardown** at `0x8016C4A0`.
//!
//! Two translator notes apply to the whole section. The Metrowerks
//! `0x802DCD5C/64/68/6C/70/74` stubs return the incoming `r3:r4` pair, so the
//! dropped arguments are spelled out as parameters (the wave-38 rule). The
//! `p = *(int **)(this + 4); local = &type; (*p->vtbl[2])(p, &local)` idiom is
//! the EGG `dynamic_cast<T>`; it is factored into `w78_cast_to`.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80161DA0` | 144 B | [`fn_80161da0`] | sound-bank list transition by stage |
//! | `0x80161E40` | 168 B | [`fn_80161e40`] | install three transform rows |
//! | `0x80162020` | 168 B | [`fn_80162020`] | install the second row block |
//! | `0x80162120` | 168 B | [`fn_80162120`] | install the third row block |
//! | `0x80162220` | 168 B | [`fn_80162220`] | install the last row block |
//! | `0x80163D40` | 208 B | [`fn_80163d40`] | depth-first walk with two predicates |
//! | `0x80164990` | 164 B | [`fn_80164990`] | allocate one auxiliary slot |
//! | `0x80165520` | 184 B | [`fn_80165520`] | resolve the entry child twice |
//! | `0x801655E0` | 216 B | [`fn_801655e0`] | cast child, run two slots |
//! | `0x801656C0` | 180 B | [`fn_801656c0`] | cast holder, forward to `0x801655E0` |
//! | `0x80165EE0` | 236 B | [`fn_80165ee0`] | class-A array teardown |
//! | `0x80166110` | 188 B | [`fn_80166110`] | release one class-A entry |
//! | `0x80166AE0` | 236 B | [`fn_80166ae0`] | class-B array teardown |
//! | `0x80166D10` | 188 B | [`fn_80166d10`] | release one class-B entry |
//! | `0x80167710` | 236 B | [`fn_80167710`] | class-C array teardown |
//! | `0x80167940` | 188 B | [`fn_80167940`] | release one class-C entry |
//! | `0x80167FA0` | 188 B | [`fn_80167fa0`] | round size and create the heap |
//! | `0x80168280` | 348 B | [`fn_80168280`] | file a node by its kind |
//! | `0x80168400` | 128 B | [`fn_80168400`] | child `+0xE8` word |
//! | `0x80168520` | 188 B | [`fn_80168520`] | clear `+0x08`, forward value |
//! | `0x8016C4A0` | 240 B | [`fn_8016c4a0`] | empty and destroy the active list |
//! | `0x8016C850` | 64 B | [`fn_8016c850`] | submit three words, flush the buffer |
//! | `0x8016CEA0` | 168 B | [`fn_8016cea0`] | re-submit a registered request |
//! | `0x8016EDD0` | 88 B | [`fn_8016edd0`] | format and submit one message id |
//! | `0x8016EEF0` | 88 B | [`fn_8016eef0`] | second message-id formatter |
//!
//! ## Random steps, interpolators and layout leaves (wave 89)
//!
//! When this pass started the `0x80160000` page still held 101 bodies. They were
//! the residue every earlier wave had declined: the `r2` float helpers, the
//! Metrowerks register-save pair, and the GQR/`ldexpf` blocks. This pass takes
//! the part that can be reconstructed without guessing:
//!
//! * the **LCG random helpers** at `0x80161460..0x801614E0`. The state word is
//!   `*state = *state * 1664525 + 1013904223`; both constants are literal in
//!   `main.elf` at `0x80429448`/`0x8042944C`, so the step is not guessed. The
//!   two float variants build a mantissa with `| 0x3F800000` and subtract
//!   `1.0`/`1.5`, which yields `[0,1)` and `[-0.5,0.5)`.
//! * the **interpolator / tween cluster** at `0x801636C0..0x801638E0` and
//!   `0x80168C10`. Their `r2` constants resolve, by reading `main.elf` at
//!   `0x8042DC30..0x8042DCB8`, to `0.0`, `1.0` and `0.7307692` (19/26); they are
//!   spelled out as named constants rather than left as bare `r2` loads.
//! * the **layout / actor leaves** at `0x80163FF0..0x8016D8A0`: the actor
//!   constructor at `0x80161AD0`, the `0x80377C3C` retire pass at `0x8016C5C0`,
//!   the auxiliary-slot release at `0x80164A40`, the four `nw4r::lyt` draw twins
//!   at `0x801685E0..0x801689B0`, and the record-tail clear at `0x8016D8A0`.
//!
//! The wave-78 Metrowerks rule is reused with one correction that makes the
//! group tractable: `FUN_802dcd5c/64/68/6c/70/74` are **prologue fragments**
//! (they only spill `r21..r31` and fall through), so the "64-bit return" the
//! export shows is the enclosing function's own incoming `r3:r4`. `hi(uVar)` is
//! therefore the function's **first** parameter and `lo(uVar)` its **second**.
//! The bodies below spell those parameters out instead of modelling a 64-bit
//! call. The matching `0x802DCDB0..BC` epilogue fragments are not emitted as
//! calls, for the same reason.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80161060` | 80 B | [`fn_80161060`] | two-step message-object composition |
//! | `0x801610B0` | 100 B | [`fn_801610b0`] | build a context, then compose it |
//! | `0x80161460` | 40 B | [`fn_80161460`] | LCG step + `mulhi` scale |
//! | `0x80161490` | 68 B | [`fn_80161490`] | LCG step, scale to `[0,1)` |
//! | `0x801614E0` | 68 B | [`fn_801614e0`] | LCG step, scale to `[-0.5,0.5)` |
//! | `0x80161AD0` | 368 B | [`fn_80161ad0`] | construct an actor node |
//! | `0x801636C0` | 148 B | [`fn_801636c0`] | advance a looping/clamped tween |
//! | `0x801637F0` | 96 B | [`fn_801637f0`] | tween end-of-range predicate |
//! | `0x801638E0` | 184 B | [`fn_801638e0`] | construct a tween with three constants |
//! | `0x80163FF0` | 132 B | [`fn_80163ff0`] | find an entry by name |
//! | `0x80164A40` | 204 B | [`fn_80164a40`] | release one auxiliary slot |
//! | `0x80164FD0` | 244 B | [`fn_80164fd0`] | create the deferred-load object |
//! | `0x80168480` | 156 B | [`fn_80168480`] | `index`th child of an element |
//! | `0x801685E0` | 304 B | [`fn_801685e0`] | draw one child with a tint |
//! | `0x80168710` | 364 B | [`fn_80168710`] | draw all children with a tint |
//! | `0x80168880` | 304 B | [`fn_80168880`] | draw one child, second path |
//! | `0x801689B0` | 364 B | [`fn_801689b0`] | draw all children, second path |
//! | `0x80168C10` | 148 B | [`fn_80168c10`] | step a cubic tween |
//! | `0x80169F80` | 116 B | [`fn_80169f80`] | attach a pane to its parent |
//! | `0x8016AB90` | 68 B | [`fn_8016ab90`] | construct with the `0x80329700` vtable |
//! | `0x8016C400` | 160 B | [`fn_8016c400`] | submit one layout request |
//! | `0x8016C5C0` | 300 B | [`fn_8016c5c0`] | retire finished list entries |
//! | `0x8016C890` | 132 B | [`fn_8016c890`] | stack-allocate and dispatch |
//! | `0x8016CAB0` | 176 B | [`fn_8016cab0`] | dispatch one registered request |
//! | `0x8016CD40` | 352 B | [`fn_8016cd40`] | register a request object |
//! | `0x8016D8A0` | 52 B | [`fn_8016d8a0`] | clear the record tail |
//! | `0x8016E230` | 356 B | [`fn_8016e230`] | oriented-box containment test |
//!
//! ## Why there is no `bcsv` module
//!
//! Wave 4 originally scoped this file as a BCSV/table parser. After checking
//! the shipped image and assets, there is **no BCSV container in this build**:
//!
//! * `sys/main.dol` contains no `BCSV` magic (`42 43 53 56`) anywhere;
//! * the 21 `.csv` members indexed in `work/arcs/index.json` are plain text
//!   (WorldMap route/point tables and `HomeButton2/home.csv`), not BCSV rows;
//! * no `.bcsv` path or extension appears in `work/dol/strings.txt`.
//!
//! The table code that *does* exist in this address band is the archive-resource
//! registry below. It is the closest real "header parse / field lookup / typed
//! accessor / row iteration" cluster: the registry header is
//! `{mpArcInfos, u16 mNumArcs, mpCallback}`, a row is one 0x40-byte `info_c`,
//! lookup is by name, and the typed accessors return raw member bytes, sizes and
//! compression types. This module therefore reconstructs the registry instead
//! of inventing a BCSV parser that the binary does not have.
//!
//! ## Table model
//!
//! `dRes_c` (the registry object):
//!
//! | offset | meaning |
//! |---|---|
//! | `+0x00` | `mpArcInfos`, pointer to the first 0x40-byte holder |
//! | `+0x04` | `u16 mNumArcs`, number of holders |
//! | `+0x08` | `mpCallback`, optional loader callback |
//!
//! `info_c` (one 0x40-byte holder):
//!
//! | offset | meaning |
//! |---|---|
//! | `+0x00` | `char mName[0x20]` |
//! | `+0x20` | `u16 mRefCount` |
//! | `+0x24` | pending DVD mount command |
//! | `+0x28` | mounted archive object |
//! | `+0x2C` | heap used to mount the archive |
//! | `+0x30` | raw archive binary |
//! | `+0x34` | archive size |
//! | `+0x38` | frame heap holding the loaded members |
//! | `+0x3C` | `u8 **mpFiles`, member-pointer array |
//!
//! ## Start here
//!
//! [`dres_set_res`] is the entry point for "make this archive available": it
//! looks up a holder, creates one, mounts the archive, and increments the
//! refcount. [`res_info_set_res`] + [`res_info_load_res`] show how the member
//! table is built; [`dres_get_res_silently`] is the shortest read-side example.
//!
//! ## Dependencies
//!
//! * [`crate::platform`] - big-endian loads/stores and absolute calls.
//! * [`crate::resource::bres`] - the three already-translated helpers
//!   `fn_800df570`, `fn_800df640`, `fn_802b76c0`, `fn_802b7760`, `fn_802b7770`.
//! * [`crate::resource::archive`] - the EGG archive object and member-read
//!   bodies that [`mount_execute`] drives.
//! * [`crate::runtime::dvd`] - the translated
//!   `dvd_convert_path_to_entrynum_801ca680` path probe.
//! * Callees outside this lane are called by absolute address and are **not**
//!   translated here: the heap layer (`0x8016E740`, `0x8016E760`, `0x8016E980`,
//!   `0x8016E9B0`, `0x802B8CC0`, `0x802B8F70`, `0x802B9240`, `0x802B9250`,
//!   `0x802B9280`), the archive enumerator (`0x802B7830`, `0x802B78A0`,
//!   `0x802B7A40`), the stream base/thread helpers (`0x802BA3B0`,
//!   `0x802BA670`, `0x801B5860`), the cache write-back `0x801AC500`, the LZ
//!   pair (`0x801D8150`, `0x801D8190`), the Metrowerks helpers
//!   (`0x802DC780`, `0x802DC7E0`, `0x802DCB10`, `0x802DC69C`, `0x802E1C40`),
//!   `snprintf` (`0x802E16E8`), `strncpy` (`0x802E19F8`), `memcpy`
//!   (`0x80004364`), the OS mutex/thread-queue primitives (`0x801B2E20`,
//!   `0x801B2E60`, `0x801B2F40`, `0x801B4B20`, `0x801B4B30`, `0x801B5C90`,
//!   `0x801B5D80`) and `VIWaitForRetrace` (`0x801BCCF0`).
//!
//! Wave 40 adds the sound-bank table callees, also called by absolute address:
//! the remaining intrusive-list primitives (`0x801616C0`, `0x80161700`,
//! `0x80161E40`), the Metrowerks critical-section
//! and hash helpers (`0x802DCB90`, `0x802DCBFC`, `0x802DC9A0`), the EGG object
//! factory (`0x802B9430`), `memset`
//! (`0x800046B4`), the draw setup `0x80163220` and the element tag query
//! `0x80249C90`. Wave 41 gives `0x80160F10`, `0x80160F60` and `0x80160FE0`
//! real bodies below, so the wave-40 bodies still reach them through the same
//! absolute-address constants but the implementation is now in this file.
//! The `0x802DCD44`/`0x802DCD90` pair that wave 40 also listed here is not a
//! pair of lock helpers: both are fragments of the compiler's register-save
//! table, and wave 291 removed them (`fn_80162ed0`).
//!
//! `// inferred:` the class roles and field names are cross-checked against the
//! vtable at `0x80319550` and the call sites of `0x800DF180`/`0x800DF3C0`; the
//! image has no symbol table, so no Nintendo symbol is claimed.
//!
//! ## Layout-slot builders and the control page (wave 93)
//!
//! Wave 89 left the prologue-pair functions of the `0x80160000` page alone
//! because the decompiler shows their first two parameters only as the result
//! of a register-save fragment. Wave 93 reads them back out the same way wave
//! 89 read `0x8016E500`/`0x8016E7F0`: the fragment returns the caller's own
//! `r3:r4`, so the two words become explicit parameters. That makes the whole
//! `0x80163E60..0x80167C40` layout-group family (one class constructor, one
//! per-element builder and one "commit" twin per class) translatable.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80160800` | 164 B | [`fn_80160800`] | release a handle's reader |
//! | `0x80161180` | 52 B | [`fn_80161180`] | angle-table lookup |
//! | `0x80163520` | 320 B | [`fn_80163520`] | draw a sound-group description |
//! | `0x80163E60` | 332 B | [`fn_80163e60`] | copy a description into a group |
//! | `0x801641B0` | 260 B | [`fn_801641b0`] | recursive match probe |
//! | `0x80164500` | 236 B | [`fn_80164500`] | create the reader view |
//! | `0x801650D0` | 288 B | [`fn_801650d0`] | build one text record |
//! | `0x801658A0` | 284 B | [`fn_801658a0`] | build one layout group |
//! | `0x801659C0` | 180 B | [`fn_801659c0`] | commit one group element |
//! | `0x80165C30` | 572 B | [`fn_80165c30`] | build a group of `n` children |
//! | `0x80165FD0` | 320 B | [`fn_80165fd0`] | commit child `n` |
//! | `0x801664A0` | 284 B | [`fn_801664a0`] | second-class group builder |
//! | `0x801665C0` | 180 B | [`fn_801665c0`] | second-class element commit |
//! | `0x80166830` | 572 B | [`fn_80166830`] | second-class group of `n` |
//! | `0x80166BD0` | 320 B | [`fn_80166bd0`] | second-class child commit |
//! | `0x80167080` | 300 B | [`fn_80167080`] | third-class group builder |
//! | `0x801671B0` | 180 B | [`fn_801671b0`] | third-class element commit |
//! | `0x80167420` | 636 B | [`fn_80167420`] | third-class group of `n` |
//! | `0x80167800` | 320 B | [`fn_80167800`] | third-class child commit |
//! | `0x80167C40` | 280 B | [`fn_80167c40`] | four-argument group builder |
//! | `0x80167D60` | 232 B | [`fn_80167d60`] | clear and re-key one element |
//! | `0x8016C820` | 4 B | [`fn_8016c820`] | empty dispatch leaf |
//! | `0x8016D640` | 464 B | [`fn_8016d640`] | register/update an actor record |
//! | `0x8016DAA0` | 60 B | [`fn_8016daa0`] | reset the text page |
//! | `0x8016DAE0` | 288 B | [`fn_8016dae0`] | install a control object |
//! | `0x80161F60` | 188 B | [`fn_80161f60`] | detach a record node |
//!
//! The four classes differ only in their constructor, cast type word, element
//! vtable and commit twin; the bodies keep those literals so the mapping can be
//! checked against `functions.tsv` and the exports.
//!
//! ## Angle, transform and render-state leaves (wave 97)
//!
//! Wave 93 declined the last 48 bodies of the `0x80160000` page because every
//! one of them touches the floating-point register file, and
//! [`crate::platform`] only fills integer registers. This pass takes the 25
//! that can be reconstructed once the float ABI is spelled out with the local
//! `extern "C"` transmute helpers `src/engine/dsys.rs` and `src/audio/ax.rs`
//! already use. The remaining 23 stay documented skips.
//!
//! Four clusters come out of it:
//!
//! * the **angle pair** `0x80160D10..0x80160EA0` (3 bodies): a step-toward
//!   helper with an `FLT_EPSILON` snap, and the horizontal/vertical angle
//!   reporters that feed `0x801611C0`;
//! * the **render-state commit family** `0x801654B0..0x801690E0` (9 bodies):
//!   the shared `0x80168F10` push, the base commit plus its four per-class
//!   twins (their reference count is converted with the `2^52 + 2^31` bias),
//!   the `+0x1C` slot helper and the two range predicates that read the
//!   element's `+0x04` object;
//! * the **texture and transform leaves** `0x80169290..0x8016EE30` (6
//!   bodies): the two texture builders, their forwarder, and the X/Y/Z
//!   rotation-matrix builders that scale a signed halfword angle by `1/256`
//!   before the sine and cosine helpers;
//! * the **record, scale and GX leaves** (7 bodies): the animation-record fill,
//!   the `0x80163FF0` lookup that feeds it, the scale record, the two
//!   random-rate scalers, the text-object factory and the GX clear reset.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80160D10` | 368 B | [`fn_80160d10`] | step a position toward a target |
//! | `0x80160E80` | 28 B | [`fn_80160e80`] | horizontal angle between two points |
//! | `0x80160EA0` | 104 B | [`fn_80160ea0`] | vertical angle between two points |
//! | `0x801613A0` | 88 B | [`fn_801613a0`] | scale a word by the random rate |
//! | `0x80161400` | 56 B | [`fn_80161400`] | scale a float by the random rate |
//! | `0x80163220` | 88 B | [`fn_80163220`] | reset the GX clear colour |
//! | `0x80163760` | 100 B | [`fn_80163760`] | fill one animation record |
//! | `0x801643B0` | 176 B | [`fn_801643b0`] | look up an entry and start its record |
//! | `0x80164770` | 308 B | [`fn_80164770`] | create one text object |
//! | `0x801654B0` | 104 B | [`fn_801654b0`] | base element commit |
//! | `0x80165AE0` | 104 B | [`fn_80165ae0`] | first-class element commit |
//! | `0x801666E0` | 104 B | [`fn_801666e0`] | second-class element commit |
//! | `0x801672D0` | 104 B | [`fn_801672d0`] | third-class element commit |
//! | `0x80167E50` | 104 B | [`fn_80167e50`] | four-argument element commit |
//! | `0x80168BD0` | 60 B | [`fn_80168bd0`] | reset a scale record |
//! | `0x80168F10` | 196 B | [`fn_80168f10`] | push one element's render state |
//! | `0x80168FE0` | 60 B | [`fn_80168fe0`] | run the `+0x1C` slot, store a scale |
//! | `0x80169020` | 192 B | [`fn_80169020`] | end-of-range predicate |
//! | `0x801690E0` | 312 B | [`fn_801690e0`] | range-containment predicate |
//! | `0x80169290` | 268 B | [`fn_80169290`] | create a texture and its draw block |
//! | `0x801693F0` | 68 B | [`fn_801693f0`] | present a texture from its record |
//! | `0x80169440` | 276 B | [`fn_80169440`] | draw and present one texture |
//! | `0x8016EBF0` | 188 B | [`fn_8016ebf0`] | X-axis rotation matrix |
//! | `0x8016ED10` | 188 B | [`fn_8016ed10`] | Y-axis rotation matrix |
//! | `0x8016EE30` | 188 B | [`fn_8016ee30`] | Z-axis rotation matrix |
//!
//! The four commit twins are the `0x80165AE0`/`0x801666E0`/`0x801672D0`/
//! `0x80167E50` addresses wave 93's layout-group bodies still call directly, so
//! those constants now name functions in this same file. `fn_80168f10` is the
//! shared callee they all reach.
//!
//! ## Bucket completion: tail thunks, record builders and GX leaves (wave 103)
//!
//! Wave 97 recorded 23 skips on this page; nineteen of them are bodies below.
//! Each skip reason was re-checked against the instruction bytes in
//! `work/dol/main.elf`, and three of them did not hold:
//!
//! * Eight of the 23 are 4-byte leaves: three `blr` and five `b <target>` tail
//!   branches. Ghidra exports the *target's* body under the thunk's address,
//!   which made them look like duplicates of already-translated functions. The
//!   instruction word of every one is quoted in its doc block.
//! * The `0x802DCD58..0x802DCDBC` "calls" at the head of several bodies are the
//!   Metrowerks register-save thunks. They only spill `r21..r31` into the
//!   caller's frame and never write `r3`/`r4`, so the `undefined8` Ghidra shows
//!   as their result is the incoming argument pair. That turns `unaff_r24..r31`
//!   and `extraout_r4`/`extraout_f1` back into ordinary parameters.
//! * No `psq_l`/`psq_st` in these bodies carries a quantised load type; the
//!   `f26..f31` pairs are plain saves. The single-float branch is the real one.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80160AD0` | 4 B | [`fn_80160ad0`] | tail branch to `memset` |
//! | `0x80161740` | 4 B | [`fn_80161740`] | empty leaf |
//! | `0x80161CC0` | 180 B | [`fn_80161cc0`] | four-way descriptor dispatch |
//! | `0x80162110` | 4 B | [`fn_80162110`] | empty leaf |
//! | `0x80162A70` | 132 B | [`fn_80162a70`] | build one bank from the factory table |
//! | `0x801644F0` | 4 B | [`fn_801644f0`] | empty leaf |
//! | `0x801645F0` | 380 B | [`fn_801645f0`] | create the two text-object tables |
//! | `0x80165280` | 548 B | [`fn_80165280`] | rebuild and re-attach a scene object |
//! | `0x80169600` | 1068 B | [`fn_80169600`] | blend one matrix entry |
//! | `0x80169B20` | 244 B | [`fn_80169b20`] | allocate a bank's entry array |
//! | `0x80169D90` | 340 B | [`fn_80169d90`] | load a resource and attach it |
//! | `0x8016AD30` | 532 B | [`fn_8016ad30`] | draw the background quad |
//! | `0x8016B950` | 4 B | [`fn_8016b950`] | tail branch to the command factory |
//! | `0x8016BCB0` | 4 B | [`fn_8016bcb0`] | tail branch to the path probe |
//! | `0x8016E3A0` | 268 B | [`fn_8016e3a0`] | test a box against six planes |
//! | `0x8016E4F0` | 4 B | [`fn_8016e4f0`] | tail branch to the heap switch |
//! | `0x8016E8E0` | 4 B | [`fn_8016e8e0`] | tail branch to the frame allocator |
//! | `0x8016EBA0` | 68 B | [`fn_8016eba0`] | pack twelve floats |
//! | `0x8016FD30` | 228 B | [`fn_8016fd30`] | allocate and fill a name record |
//!
//! `0x8016F220` was also read this pass and looked reconstructible in
//! principle, but its four-player loop pushed the measured size past what could
//! be verified field by field; it was left as a skip together with
//! `0x801611C0`, `0x8016DEC0` and `0x8016FA10`. All four are bodies below as of
//! wave 110.
//!
//! ## Bucket closure: quadrant table, player frame and text pages (wave 110)
//!
//! The four skips left on this page are bodies below, so the `0x80160000`
//! bucket has no untranslated address left. Wave 103's reasons were re-checked
//! against the instruction words in `work/dol/main.elf` and none of them held:
//!
//! * `0x801611C0` is reached by a **tail branch** from `0x80160E80`, so its
//!   `f1`/`f2` are single-precision `fsubs` results, not the doubles the export
//!   prints. Its one-argument table lookups are pairs whose second member the
//!   decompiler dropped; the listing names it in each branch.
//! * `0x8016DEC0`, `0x8016F220` and `0x8016FA10` keep all their state in `r13`
//!   slots and BSS tables, which the export prints as `unaff_r13`. The listing
//!   gives every slot a concrete offset. One of them (`r13 - 0x5208`) is also
//!   written by [`fn_8016f640`] and read by [`fn_8016f660`]; the `1`/`3`
//!   comparisons here show it is a small count, not an object pointer.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x801611C0` | 436 B | [`fn_801611c0`] | quadrant angle from two direction components |
//! | `0x8016DEC0` | 880 B | [`fn_8016dec0`] | rebuild a box's corners, face directions and bounds |
//! | `0x8016F220` | 484 B | [`fn_8016f220`] | advance the four-player table one frame |
//! | `0x8016FA10` | 604 B | [`fn_8016fa10`] | draw and age one text page |
//!
//! ## Status
//!
//! Four hundred and twenty-three functions, all `translated`; none is a stub.
//! Wave 7 added the thin
//! `0x800DF840` manager wrapper; wave 15 added the 21-body mount-command queue
//! at `0x8016B440..0x8016BF70`; wave 24 added the 18-body archive table, stream
//! manager, executor and path probe at `0x8016B090..0x8016BC30`; wave 30 added
//! the 11-body EGG stream base at `0x802BA2B0..0x802BA800`; wave 38 added the
//! 14-body mount/archive heap layer at `0x8016E500..0x8016EB50`; wave 39
//! added the 13-body stream/frame-heap wrapper family at
//! `0x8016A480..0x8016A9B0`; wave 40 added the 22-body sound-bank table layer at
//! `0x801628C0..0x801634C0` and the five-body `0x38`-stride element accessor
//! set at `0x80168060..0x801680E0`; wave 41 added the 30-body handle, list,
//! live-context and element-dispatch group at
//! `0x80160020..0x80166360`; wave 44 (manual, while the free model was
//! exhausted) added the 20-body leaf-wrapper group at
//! `0x80161380..0x8016F830`, plus 17 more leaf wrappers at
//! `0x80161560..0x8016A130`, plus four virtual-dispatch wrappers at
//! `0x801636A0..0x8016F410` and 15 lookup/copy helpers at
//! `0x801616C0..0x8016F460`; wave 45 added the 28-body context-vtable,
//! element-wrapper, copy-on-success and table-reset group at
//! `0x80164E50..0x8016F7D0`; wave 52 added the 28-body list, sound-object,
//! element and interpolator leaves at `0x80161700..0x80168D80`; wave 64 added
//! the 30-body tree-list, actor-state, sorted-insert and element-class twin
//! group at `0x80161530..0x8016FF60`; wave 78 added the 25-body class teardown,
//! cast-helper, row-install and list-release layer at
//! `0x80161DA0..0x8016EEF0`, for three hundred and twenty-two functions in
//! total; wave 89 added the 27-body random, tween and layout group at
//! `0x80161060..0x8016E230`, for three hundred and forty-nine functions in
//! total; wave 93 added the 26-body layout-slot, angle-table and control-page
//! group at `0x80160800..0x8016DAE0`; wave 97 added the 25-body angle,
//! transform and render-state group at `0x80160D10..0x8016EE30`; wave 103 added
//! the 19-body tail-thunk, record-builder and GX-leaf group at
//! `0x80160AD0..0x8016FD30`; wave 110 added the last four bodies at
//! `0x801611C0..0x8016FA10`, for four hundred and twenty-three functions in
//! total.
//! `0x80165EE0`, `0x80166AE0`, `0x80167710` and `0x80168520` are now real
//! bodies, so the wave-64 constants that still name them (`W64_FN_*`) are
//! addresses of functions in this same file, and `0x80165280`,
//! `0x801611C0`, `0x8016DEC0`, `0x8016F220` and `0x8016FA10` all have bodies
//! now: no address of the `0x80160000` page is left untranslated.
//! The functions
//! at `0x800DF760` (vtable-only callback) and `0x800DF750` (4-byte empty thunk)
//! remain below the size floor and are listed as next targets in
//! `docs/COVERAGE_WAVE4_REGISTRY.md`. `// partial:` none.

use crate::boot::entry::SDA2_BASE;
use crate::engine::startup::SDA_BASE;
use crate::game::bases::{interface_call_desc, interface_call_desc2, Descriptor};
use crate::platform;
use crate::resource::archive;
use crate::resource::bres;
use crate::runtime::dvd;

// ===========================================================================
// dRes_c / info_c field offsets.
// ===========================================================================

/// `dRes_c` -> `info_c *mpArcInfos` (start of the 0x40-byte holder array).
const DRES_ARC_INFOS: usize = 0x00;
/// `dRes_c` -> `u16 mNumArcs` (holder count).
const DRES_NUM_ARCS: usize = 0x04;
/// `dRes_c` -> `callback_c *mpCallback`.
const DRES_CALLBACK: usize = 0x08;

/// `info_c` -> `char mName[0x20]`.
const INFO_NAME: usize = 0x00;
/// `info_c` -> `u16 mRefCount`.
const INFO_REFCOUNT: usize = 0x20;
/// `info_c` -> pending `mDvd_mountMemArchive_c *`.
const INFO_DVD_CMD: usize = 0x24;
/// `info_c` -> mounted `EGG::Archive *`.
const INFO_ARCHIVE: usize = 0x28;
/// `info_c` -> heap the archive was mounted into.
const INFO_MOUNT_HEAP: usize = 0x2C;
/// `info_c` -> raw archive bytes.
const INFO_ARC_BINARY: usize = 0x30;
/// `info_c` -> archive size.
const INFO_ARCHIVE_SIZE: usize = 0x34;
/// `info_c` -> frame heap holding the member data.
const INFO_DATA_HEAP: usize = 0x38;
/// `info_c` -> `u8 **mpFiles`.
const INFO_FILES: usize = 0x3C;

/// Distance between two `info_c` rows.
const INFO_STRIDE: usize = 0x40;

/// DVD mount command: "done" byte at `+0x08`.
const DVD_CMD_DONE: usize = 0x08;
/// DVD mount command: mounted archive pointer at `+0x10`.
const DVD_CMD_ARCHIVE: usize = 0x10;
/// DVD mount command: mount heap at `+0x14`.
const DVD_CMD_HEAP: usize = 0x14;
/// DVD mount command: archive size at `+0x18`.
const DVD_CMD_SIZE: usize = 0x18;

// ===========================================================================
// Globals and read-only tables.
// ===========================================================================

/// `r13 - 0x5648` -> `0x8042A038`: the `dResMng_c` singleton set by
/// [`dres_mng_construct`] and cleared by [`dres_mng_destroy`].
const G_RES_MNG_INSTANCE: usize = SDA_BASE - 0x5648;

/// `r13 - 0x5650` -> `0x8042A030`: optional "archive scheduled" hook
/// (`dRes_c::mSetCallback`); called by [`res_info_set`] after a successful
/// mount request.
const G_SET_CALLBACK: usize = SDA_BASE - 0x5650;

/// `0x80377C48`: `mHeap::g_gameHeaps[MEM_EXP_HEAP_ALLOC_FAST]`-style default
/// heap used by [`res_info_set_res`] when the mount heap is null.
const G_GAME_HEAP_DEFAULT: usize = 0x8037_7C48;

/// `0x80319550`: `dResMng_c` vtable installed by [`dres_mng_construct`].
const RES_MNG_VTABLE: usize = 0x8031_9550;

/// `0x80319560`: `resCallback_c` vtable installed at `dResMng_c + 0x10`.
const RES_CALLBACK_VTABLE: usize = 0x8031_9560;

/// `0x80319528`: format string `"/%s/%s.arc"` (folder, archive name).
const ARC_PATH_FORMAT: usize = 0x8031_9528;

/// `0x80319534`: debug name `"dRes_c::info_c::mDataHeap"` passed to the frame
/// heap factory.
const DATA_HEAP_NAME: usize = 0x8031_9534;

/// `0x80427F78`: format string `"%s.LZ"` used by the compressed accessor.
const LZ_SUFFIX_FORMAT: usize = 0x8042_7F78;

/// `0x800DE940`: element constructor handed to the Metrowerks
/// `__construct_array` helper by [`dres_init`].
///
/// `// unknown:` Ghidra has no separate function entry for this label; it is
/// an in-object constructor, not a named routine. It is passed through by
/// address exactly as the C does.
const RES_INFO_CTOR: usize = 0x800D_E940;

/// `0x800DE970`: element destructor handed to Metrowerks `__destroy_arr` and
/// `__construct_array`. It is the same routine as [`res_info_destroy`].
const RES_INFO_DTOR: usize = 0x800D_E970;

/// `0x800DEDA0`: static `searchCallback_c` thunk handed to
/// [`ARCHIVE_SEARCH_INSIDE`] by [`res_info_load_res`].
///
/// `// unknown:` the thunk is not a separate Ghidra export; the address is the
/// literal `&DAT_800deda0` in the caller's C.
const SEARCH_CALLBACK: usize = 0x800D_EDA0;

/// Number of holders installed by `dResMng_c`'s constructor.
const MAX_ARCHIVES: u32 = 300;

// ===========================================================================
// Callees outside this lane (called by absolute address).
// ===========================================================================

/// `0x80004364` - `memcpy(dst, src, len)`.
const MEMCPY: usize = 0x8000_4364;

/// `0x8016B7E0` - destroy a finished DVD mount command object.
const DVD_CMD_DESTROY: usize = 0x8016_B7E0;

/// `0x8016BCC0` - `mDvd_mountMemArchive_c::create(path, allocDir, heap)`.
const DVD_MOUNT_CREATE: usize = 0x8016_BCC0;

/// `0x8016BE60` - return the raw archive bytes owned by a mount command.
const DVD_GET_ARC_BINARY: usize = 0x8016_BE60;

/// `0x8016E740` - destroy a frame heap (`mHeap::destroyFrmHeap`).
const HEAP_DESTROY_FRM: usize = 0x8016_E740;

/// `0x8016E760` - adjust a frame heap (`mHeap::adjustFrmHeap`).
const HEAP_ADJUST_FRM: usize = 0x8016_E760;

/// `0x8016E980` - restore the heap that was current before the frame heap.
const HEAP_RESTORE_CURRENT: usize = 0x8016_E980;

/// `0x8016E9B0` - create a frame heap on the current heap:
/// `(size, parent, name, align, option)`.
const HEAP_CREATE_FRM_TO_CURRENT: usize = 0x8016_E9B0;

/// `0x801AC500` - `DCStoreRangeNoSync(address, length)`.
const DC_STORE_RANGE_NO_SYNC: usize = 0x801A_C500;

/// `0x801BCCF0` - `VIWaitForRetrace()`.
const VI_WAIT_FOR_RETRACE: usize = 0x801B_CCF0;

/// `0x801D8150` - `CXGetUncompressedSize(LZ data)`.
const CX_GET_UNCOMPRESSED_SIZE: usize = 0x801D_8150;

/// `0x801D8190` - `CXUncompressLZ(src, dst)` (two arguments, no length).
const CX_UNCOMPRESS_LZ: usize = 0x801D_8190;

/// `0x802B7680` - `EGG::Archive::unmount()`; releases the archive object and
/// destroys it when its internal reference count reaches zero.
const ARCHIVE_UNMOUNT: usize = 0x802B_7680;

/// `0x802B7830` - `EGG::Archive::countFile()`.
const ARCHIVE_COUNT_FILE: usize = 0x802B_7830;

/// `0x802B78A0` - `EGG::Archive::getFileArray(files, count)`.
const ARCHIVE_GET_FILE_ARRAY: usize = 0x802B_78A0;

/// `0x802B7A40` - `EGG::Archive::searchInside(callback, user)`.
const ARCHIVE_SEARCH_INSIDE: usize = 0x802B_7A40;

/// `0x802B8F70` - `EGG::Heap::free(ptr, heap)` (explicit-heap variant).
const HEAP_FREE_EXPLICIT: usize = 0x802B_8F70;

/// `0x802B9250` - default allocator (`operator new(size)`).
const HEAP_ALLOC: usize = 0x802B_9250;

/// `0x802B9280` - default deallocator (`operator delete(ptr)`).
const HEAP_DELETE: usize = 0x802B_9280;

/// `0x802DC7E0` - Metrowerks `__construct_array(ptr, ctor, dtor, size, count)`.
const CONSTRUCT_ARRAY: usize = 0x802D_C7E0;

/// `0x802DCB10` - Metrowerks `__destroy_arr(ptr, dtor, size, count)`.
const DESTROY_ARRAY: usize = 0x802D_CB10;

/// `0x802E16E8` - `snprintf(dst, size, format, ...)`.
const SNPRINTF: usize = 0x802E_16E8;

/// `0x802E19F8` - `strncpy(dst, src, len)`.
const STRNCPY: usize = 0x802E_19F8;

// ===========================================================================
// Mount-command queue (the DVD layer behind `info_c`): 0x8016B440..0x8016BF70
//
// These are the objects `res_info_set` creates (0x8016BCC0) and
// `res_info_cleanup` consumes (0x8016B7E0 / 0x8016BE60). They were previously
// called by address only; wave 15 gives the queue and both command kinds bodies.
// ===========================================================================

/// `r13 - 0x5254` -> `0x8042A42C`: the active mount-queue manager.
const G_MOUNT_QUEUE: usize = SDA_BASE - 0x5254;

/// `r13 - 0x5260` -> `0x8042A420`: heap used by [`mount_cmd_alloc`] /
/// [`mount_cmd_free`].
const G_MOUNT_HEAP: usize = SDA_BASE - 0x5260;

/// `r13 - 0x525C` -> `0x8042A424`: default heap for a mount command whose
/// `+0x14` heap word is zero.
const G_MOUNT_DEFAULT_HEAP: usize = SDA_BASE - 0x525C;

/// `r13 - 0x5268` -> `0x8042A418`: thread id of the queue consumer.
const G_MOUNT_THREAD: usize = SDA_BASE - 0x5268;

/// `r13 - 0x6218` -> `0x80429468`: default command mode byte.
const G_MOUNT_DEFAULT_MODE: usize = SDA_BASE - 0x6218;

/// `0x8032977C` - vtable of the base mount command.
const MOUNT_CMD_BASE_VTABLE: usize = 0x8032_977C;

/// `0x80329754` - vtable of the "mount into memory" command created by
/// [`mount_mem_cmd_create`].
const MOUNT_MEM_VTABLE: usize = 0x8032_9754;

/// `0x80329740` - vtable of the "mount as archive" command created by
/// [`mount_arc_cmd_create`].
const MOUNT_ARC_VTABLE: usize = 0x8032_9740;

/// `0x802B8CC0` - EGG heap allocation `(size, alignment, heap)`.
const EGG_ALLOC_ALIGNED: usize = 0x802B_8CC0;

/// `0x801B2E20` - `OSInitMutex(mutex)`.
const OS_INIT_MUTEX: usize = 0x801B_2E20;

/// `0x801B4B20` - `OSInitThreadQueue(queue)`.
const OS_INIT_THREAD_QUEUE: usize = 0x801B_4B20;

/// `0x801B5C90` - `OSSleepThread(queue)`.
const OS_SLEEP_THREAD: usize = 0x801B_5C90;

/// `0x801B5D80` - `OSWakeupThread(queue)`.
const OS_WAKEUP_THREAD: usize = 0x801B_5D80;

/// `0x801B4B30` - `OSGetCurrentThread()`.
const OS_GET_CURRENT_THREAD: usize = 0x801B_4B30;

/// `0x801B2E60` - `OSLockMutex(mutex)`.
const OS_MUTEX_LOCK: usize = 0x801B_2E60;

/// `0x801B2F40` - `OSUnlockMutex(mutex)`.
const OS_MUTEX_UNLOCK: usize = 0x801B_2F40;

/// `0x8016BC30` - path probe; the translation is [`mount_probe_path`].
///
/// Retained as an address because the wave-15 command factory still calls it
/// directly.
const MOUNT_PROBE: usize = 0x8016_BC30;

/// `0x8016B2A0` - mount executor; the translation is [`mount_execute`].
///
/// Retained as an address because the wave-15 memory-mount command still calls
/// it directly with eight arguments.
const MOUNT_EXEC: usize = 0x8016_B2A0;

// ---------------------------------------------------------------------------
// Mount archive table, stream manager and path probe (wave 24).
// ---------------------------------------------------------------------------

/// `r13 - 0x5270` -> `0x8042A410`: first slot of the mount-archive table.
///
/// Each slot holds one `u32` pointer to an archive-manager object whose `+0x04`
/// byte is the tag and whose `+0x08` word is the extension string.
const G_MOUNT_ARCHIVE_TABLE: usize = SDA_BASE - 0x5270;
/// `r13 - 0x526C` -> `0x8042A414`: one past the last table slot.
const G_MOUNT_ARCHIVE_TABLE_END: usize = SDA_BASE - 0x526C;
/// `r13 - 0x621C` -> `0x80429464`: static slot installed by
/// [`mount_archive_table_init`]; it contains `0x80377AF0` in the image.
const MOUNT_ARCHIVE_TABLE_STATIC: usize = SDA_BASE - 0x621C;
/// `r13 - 0x6218` -> `0x80429468`: one past the static slot.
const MOUNT_ARCHIVE_TABLE_STATIC_END: usize = SDA_BASE - 0x6218;
/// `r13 - 0x6214` -> `0x8042946C`: chunk size passed to the decoder
/// (`0x00010000` in the image).
const G_MOUNT_CHUNK: usize = SDA_BASE - 0x6214;
/// `r13 - 0x5258` -> `0x8042A428`: enable the extension-fallback path probe.
const G_MOUNT_PATH_FALLBACK: usize = SDA_BASE - 0x5258;
/// `r13 - 0x5257` -> `0x8042A429`: the global archive object is constructed.
const G_GLOBAL_ARCHIVE_READY: usize = SDA_BASE - 0x5257;
/// `r13 - 0x5264` -> `0x8042A41C`: the threaded mount-stream object.
const G_MOUNT_STREAM: usize = SDA_BASE - 0x5264;
/// `r13 - 0x5228` -> `0x8042A458`: optional global DVD heap context installed
/// by `0x8016EB50`; when present it is given to the new stream.
const G_MOUNT_DVD_HEAP: usize = SDA_BASE - 0x5228;

/// `0x80377B60`: process-wide EGG archive object created by [`mount_execute`].
const G_GLOBAL_ARCHIVE: usize = 0x8037_7B60;
/// `0x80377B50`: static storage used when registering the archive destructor.
const G_GLOBAL_ARCHIVE_REG: usize = 0x8037_7B50;
/// `0x80377BD0` (`G_GLOBAL_ARCHIVE + 0x70`): decoded-size word published by the
/// decoder path of [`mount_execute`].
const G_GLOBAL_ARCHIVE_SIZE: usize = 0x8037_7BD0;
/// `0x802B7B50` - EGG archive-object destructor registered for the global
/// archive.
const GLOBAL_ARCHIVE_DTOR: usize = 0x802B_7B50;

/// `0x80329728` - vtable of the threaded mount-stream object.
const MOUNT_STREAM_VTABLE: usize = 0x8032_9728;
/// `0x80329768` - vtable of the "call one function" mount command.
const MOUNT_CALL_CMD_VTABLE: usize = 0x8032_9768;

/// `0x802BA3B0` - construct the threaded stream base:
/// `(this, buffer_size, 0, priority, 0)`. It allocates the read buffer and an
/// OS thread and stores the thread pointer at `this + 0x08`.
const MOUNT_STREAM_CTOR: usize = 0x802B_A3B0;
/// `0x802BA670` - set the stream's heap override: `(stream, heap)`. The second
/// argument is the heap object, not an allocation size.
const MOUNT_STREAM_SET_HEAP: usize = 0x802B_A670;
/// `0x802B9240` - `operator new(size, heap, alignment)`.
const MOUNT_OBJECT_ALLOC: usize = 0x802B_9240;
/// `0x801B5860` - `OSResumeThread(thread)`: decrements the suspend count and
/// schedules the thread when it reaches zero.
const OS_RESUME_THREAD: usize = 0x801B_5860;
/// `0x802DC780` - Metrowerks global-destructor registration
/// `(object, destructor, storage)`.
const REGISTER_GLOBAL_OBJECT: usize = 0x802D_C780;
/// `0x801D4A90` - return the size word stored immediately before an allocation.
const ALLOC_SIZE_PREFIX: usize = 0x801D_4A90;
/// `0x802DC69C` - `strlen`-shaped: length of a NUL-terminated C string.
const STRLEN: usize = 0x802D_C69C;
/// `0x802E1C40` - `strrchr`-shaped: last occurrence of a byte in a C string.
const STR_RCHR: usize = 0x802E_1C40;

// ---------------------------------------------------------------------------
// Wave 52: list/state-machine, sound-object, element and interpolator leaves
// (`0x80161700..0x8016A340`).
// ---------------------------------------------------------------------------

/// `0x80161740` - the empty guard routine every list helper calls when one of
/// its arguments is null or zero. It returns immediately.
const LIST_BAD_ARG: usize = 0x8016_1740;

/// `0x802AD520` - write four words describing the transform of the object at
/// `this + 0x10` into the caller's buffer.
const FN_TRANSFORM_QUERY: usize = 0x802A_D520;

/// `0x801636A0` - three-argument jumptable dispatch into virtual slot `+0xC`
/// of the object at `this + 4`. Called by address because the slot is a
/// jumptable-with-too-many-branches in the export.
const FN_DISPATCH_3: usize = 0x8016_36A0;

/// `0x80163D40` - recursive "apply to every child" helper used by
/// [`fn_80163e10`]. Not translated: it wraps a Metrowerks critical section.
const FN_RECURSIVE_APPLY: usize = 0x8016_3D40;

/// `0x801641B0` - recursive key lookup over a node tree, returning the matched
/// entry pointer or zero.
const FN_KEY_LOOKUP: usize = 0x8016_41B0;

/// `0x802B9290` - release one allocation through `0x802B8F70(ptr, 0)`.
const HEAP_RELEASE: usize = 0x802B_9290;

/// `0x80167EC0` - reset/destroy the interpolator object at `this`.
const FN_INTERP_RESET: usize = 0x8016_7EC0;

/// `0x8025B420` - install `cast` into the output object with mode `1` and
/// `arg`; returns the callee's status.
const FN_CAST_INSTALL: usize = 0x8025_B420;

/// `0x802F8FD8` - type-info descriptor handed to the cast query at vtable
/// slot `+0x08`.
const DESCRIPTOR_802F8FD8: usize = 0x802F_8FD8;

/// `0x80329600` - data word holding the function pointer invoked by the
/// element destructor [`fn_80168100`].
const ELEMENT_DTOR_SLOT: usize = 0x8032_9600;

/// `0x803295F0` - vtable installed by [`fn_80168100`] before the teardown.
const ELEMENT_DTOR_VTABLE: usize = 0x8032_95F0;

/// `0x80329670` - vtable installed by [`fn_8016a030`].
const ELEMENT_WRAPPER_VTABLE: usize = 0x8032_9670;

/// `0x803295D8` - base vtable installed by [`fn_80168d10`] (also the one
/// [`fn_80167ec0`] restores).
const INTERP_BASE_VTABLE: usize = 0x8032_95D8;

/// `0x80329610` - vtable installed by [`fn_80168b40`].
const INTERP_VALUE_VTABLE: usize = 0x8032_9610;

/// `0x80329620` - vtable installed by [`fn_80168d10`] after its sub-object is
/// constructed.
const INTERP_ACTIVE_VTABLE: usize = 0x8032_9620;

/// `r2 - 0x53D8` - one of the two `f32` constants the interpolator resets to
/// (stored at `+0x04`, `+0x10`, `+0x14` and `+0x18`).
const INTERP_VALUE_A: usize = SDA2_BASE - 0x53D8;

/// `r2 - 0x53D4` - the second reset `f32` constant (stored at `+0x08`,
/// `+0x14` and `+0x18`).
const INTERP_VALUE_B: usize = SDA2_BASE - 0x53D4;

/// `r2 - 0x53D0` - default `f32` copied into `+0x28`/`+0x30` by
/// [`fn_80168d10`].
const INTERP_VALUE_C: usize = SDA2_BASE - 0x53D0;

// ===========================================================================
// info_c lifecycle: 0x800DE970..0x800DEDB0
// ===========================================================================

/// Destroys one archive holder after its pending DVD command finishes.
///
/// The original busy-waits on the command's `mDone` byte, calls
/// [`res_info_cleanup`], and then frees the holder when `deleting` is positive.
/// A null holder returns `0` unchanged.
///
/// # Original
/// - Ghidra: `FUN_800de970` at `0x800DE970` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de970_FUN_800de970.c`
/// - observed: `cVar1 = *(char *)(*(int *)(param_1 + 0x24) + 8); while
///   (cVar1 == '\0') { FUN_801bccf0(); ... } FUN_800de9f0(param_1);`
///
/// # Arguments
/// - `this`: address of a 0x40-byte `info_c`.
/// - `deleting`: `> 0` to free the holder after cleanup (virtual deleting
///   destructor flag).
///
/// # Returns
/// `this`, or `0` when `this` is null.
///
/// # Side effects
/// Waits for a retrace, unmounts and frees the holder's resources, and may
/// free `this`.
///
/// # Safety
/// `this` must be null or a live `info_c`; its command pointer must be null or
/// a live mount command. The loop has no timeout in the original.
///
/// # Confidence
/// high - the wait loop, cleanup call, conditional free and return are explicit.
pub unsafe fn res_info_destroy(this: usize, deleting: i32) -> usize {
    if this == 0 {
        return 0;
    }
    let cmd = platform::read_u32(this + INFO_DVD_CMD);
    while platform::read_u8(cmd as usize + DVD_CMD_DONE) == 0 {
        platform::call0(VI_WAIT_FOR_RETRACE);
    }
    res_info_cleanup(this);
    if deleting > 0 {
        platform::call1(HEAP_DELETE, this as u32);
    }
    this
}

/// Releases everything an archive holder owns.
///
/// If a DVD command is pending and done, its archive, binary, mount heap and
/// size are adopted into the holder and the command is destroyed. Then the
/// member heap and member array are released, the archive is unmounted, and the
/// raw archive binary is freed. Returns `0` while the DVD command is still
/// running.
///
/// # Original
/// - Ghidra: `FUN_800de9f0` at `0x800DE9F0` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800de9f0_FUN_800de9f0.c`
/// - observed: `iVar2 = *(int *)(param_1 + 0x24);` then the `+0x28/+0x2c/
///   +0x30/+0x34` copies, `FUN_8016b7e0()`, `FUN_802b7680(+0x28)`,
///   `FUN_802b8f70(+0x30,0)`.
///
/// # Arguments
/// - `this`: address of a live 0x40-byte `info_c`.
///
/// # Returns
/// `1` after cleanup, or `0` when the pending DVD command has not completed.
///
/// # Side effects
/// Frees the frame heap and archive binary, unmounts the archive, and clears
/// the corresponding holder fields.
///
/// # Safety
/// `this` must point at a live `info_c`; a non-null command pointer must be a
/// live mount command.
///
/// # Confidence
/// high - every store/load and both free paths are visible.
pub unsafe fn res_info_cleanup(this: usize) -> u32 {
    let cmd = platform::read_u32(this + INFO_DVD_CMD);
    if cmd != 0 {
        if platform::read_u8(cmd as usize + DVD_CMD_DONE) == 0 {
            return 0;
        }
        platform::write_u32(
            this + INFO_ARCHIVE,
            platform::read_u32(cmd as usize + DVD_CMD_ARCHIVE),
        );
        platform::write_u32(
            this + INFO_ARC_BINARY,
            platform::call1(DVD_GET_ARC_BINARY, cmd),
        );
        platform::write_u32(
            this + INFO_MOUNT_HEAP,
            platform::read_u32(cmd as usize + DVD_CMD_HEAP),
        );
        platform::write_u32(
            this + INFO_ARCHIVE_SIZE,
            platform::read_u32(cmd as usize + DVD_CMD_SIZE),
        );
        platform::call1(DVD_CMD_DESTROY, cmd);
        platform::write_u32(this + INFO_DVD_CMD, 0);
    }

    let archive = platform::read_u32(this + INFO_ARCHIVE);
    if archive != 0 {
        let data_heap = platform::read_u32(this + INFO_DATA_HEAP);
        if data_heap != 0 {
            platform::call1(HEAP_DESTROY_FRM, data_heap);
            platform::write_u32(this + INFO_DATA_HEAP, 0);
        }
        platform::call1(ARCHIVE_UNMOUNT, archive);
        platform::write_u32(this + INFO_ARCHIVE, 0);
        platform::write_u32(this + INFO_FILES, 0);
    }

    let binary = platform::read_u32(this + INFO_ARC_BINARY);
    if binary != 0 {
        platform::call2(HEAP_FREE_EXPLICIT, binary, 0);
        platform::write_u32(this + INFO_ARC_BINARY, 0);
    }
    1
}

/// Builds `/<folder>/<name>.arc` and schedules the memory archive mount.
///
/// The path is built into a 0x50-byte stack buffer, handed to
/// `mDvd_mountMemArchive_c::create`, and the command pointer is stored at
/// `info_c + 0x24`. On success the archive name is copied into the holder and
/// the optional global hook [`G_SET_CALLBACK`] is notified.
///
/// # Original
/// - Ghidra: `FUN_800dead0` at `0x800DEAD0` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dead0_FUN_800dead0.c`
/// - observed: `FUN_802e16e8(auStack_68,0x50,s___s__s_arc_80319528,param_3,
///   param_2); iVar1 = FUN_8016bcc0(auStack_68,param_4,param_5);
///   FUN_802e19f8(param_1,param_2,0x20);`
///
/// # Arguments
/// - `this`: address of a free `info_c`.
/// - `arc_name`: archive name (`param_2`).
/// - `folder`: containing folder (`param_3`).
/// - `alloc_dir`: allocation direction passed to the DVD layer.
/// - `heap`: heap for the mount, or 0 for the default.
///
/// # Returns
/// `1` when the mount command was created, `0` otherwise.
///
/// # Side effects
/// Writes `this + 0x24` and `this + 0x00..0x20`, and starts an asynchronous
/// DVD mount.
///
/// # Safety
/// `this` must be a writable free holder; `arc_name`/`folder` must be readable
/// C strings; `heap` must be null or a live EGG heap.
///
/// # Confidence
/// high - format arguments, mount call and name copy are explicit.
pub unsafe fn res_info_set(
    this: usize,
    arc_name: usize,
    folder: usize,
    alloc_dir: u32,
    heap: usize,
) -> u32 {
    let mut path = crate::platform::GuestLocal::new( [0u8; 0x50]);
    let path_addr = path.as_mut_ptr() as u32;
    platform::call5(
        SNPRINTF,
        path_addr,
        0x50,
        ARC_PATH_FORMAT as u32,
        folder as u32,
        arc_name as u32,
    );
    let cmd = platform::call3(DVD_MOUNT_CREATE, path_addr, alloc_dir, heap as u32);
    platform::write_u32(this + INFO_DVD_CMD, cmd);
    if cmd == 0 {
        return 0;
    }
    platform::call3(STRNCPY, this as u32, arc_name as u32, 0x20);
    let hook = platform::read_u32(G_SET_CALLBACK);
    if hook != 0 {
        platform::call2(hook as usize, path_addr, heap as u32);
    }
    1
}

/// Finishes a pending archive mount and builds its member table.
///
/// If the holder has no archive yet, this consumes a completed DVD command:
/// it adopts the archive pointer, raw binary, mount heap and size, destroys the
/// command, creates the member-data frame heap, calls [`res_info_load_res`],
/// restores the previous heap, and writes the loaded range back to memory.
///
/// # Original
/// - Ghidra: `FUN_800deba0` at `0x800DEBA0` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800deba0_FUN_800deba0.c`
/// - observed: `if (*(int *)(param_1 + 0x28) == 0) { ... if (*(char *)(iVar2 +
///   8) == '\0') return 1; ... FUN_8016e9b0(0xffffffff, iVar2, ...);
///   FUN_800dedb0(param_1,param_2); FUN_8016e980();
///   FUN_8016e760(...); FUN_801ac500(...); }`
///
/// # Arguments
/// - `this`: address of a live `info_c`.
/// - `callback`: optional loader callback address, or 0.
///
/// # Returns
/// `-1` on error, `1` while the DVD command is still running, `0` on success
/// (including "already loaded").
///
/// # Side effects
/// Adopts the command's archive, allocates the data heap and member array,
/// enumerates the archive, writes the heap range back to main memory.
///
/// # Safety
/// `this` must be a live holder; `callback` must be null or implement the
/// two-virtual-method callback ABI.
///
/// # Confidence
/// high - every branch and call is explicit; only the EGG helper identities are
/// inferred from their argument shapes.
pub unsafe fn res_info_set_res(this: usize, callback: usize) -> i32 {
    if platform::read_u32(this + INFO_ARCHIVE) == 0 {
        let cmd = platform::read_u32(this + INFO_DVD_CMD);
        if cmd == 0 {
            return -1;
        }
        if platform::read_u8(cmd as usize + DVD_CMD_DONE) == 0 {
            return 1;
        }
        platform::write_u32(
            this + INFO_ARCHIVE,
            platform::read_u32(cmd as usize + DVD_CMD_ARCHIVE),
        );
        platform::write_u32(
            this + INFO_ARC_BINARY,
            platform::call1(DVD_GET_ARC_BINARY, cmd),
        );
        platform::write_u32(
            this + INFO_MOUNT_HEAP,
            platform::read_u32(cmd as usize + DVD_CMD_HEAP),
        );
        platform::write_u32(
            this + INFO_ARCHIVE_SIZE,
            platform::read_u32(cmd as usize + DVD_CMD_SIZE),
        );
        platform::call1(DVD_CMD_DESTROY, cmd);
        platform::write_u32(this + INFO_DVD_CMD, 0);

        if platform::read_u32(this + INFO_ARCHIVE) == 0 {
            return -1;
        }

        let mount_heap = platform::read_u32(this + INFO_MOUNT_HEAP);
        let heap = if mount_heap != 0 {
            mount_heap
        } else {
            platform::read_u32(G_GAME_HEAP_DEFAULT)
        };
        let data_heap = platform::call5(
            HEAP_CREATE_FRM_TO_CURRENT,
            0xffff_ffff,
            heap,
            DATA_HEAP_NAME as u32,
            0x20,
            0,
        );
        platform::write_u32(this + INFO_DATA_HEAP, data_heap);
        if data_heap == 0 {
            return -1;
        }

        let status = res_info_load_res(this, callback);
        platform::call0(HEAP_RESTORE_CURRENT);
        platform::call1(HEAP_ADJUST_FRM, data_heap);
        if status == -1 {
            return -1;
        }

        let heap_base = platform::read_u32(this + INFO_DATA_HEAP) as usize;
        let end = platform::read_u32(platform::read_u32(heap_base + 0x10) as usize + 0x1c)
            as usize;
        platform::call2(
            DC_STORE_RANGE_NO_SYNC,
            heap_base as u32,
            (end - heap_base) as u32,
        );
    }
    0
}

/// Allocates `mpFiles` and enumerates every member of the holder's archive.
///
/// With a null callback the archive's file array is filled directly. With a
/// callback, the callback's `init(name)` virtual is called first and a
/// `searchCallback_c` context (`{callback, files, count, 0, '    '}`) is passed
/// to `EGG::Archive::searchInside`, which asks the callback to `execute` each
/// file.
///
/// # Original
/// - Ghidra: `FUN_800dedb0` at `0x800DEDB0` (408 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800dedb0_FUN_800dedb0.c`
/// - observed: `uVar1 = FUN_802b7830(+0x28); iVar2 = FUN_802b9250(uVar1 << 2);
///   ... FUN_802b78a0(+0x28, +0x3c, uVar1);` and the callback arm with
///   `local_18 = 0x20202020; FUN_802b7a40(+0x28,&DAT_800deda0,&local_28);`
///
/// # Arguments
/// - `this`: address of a live `info_c` whose archive is mounted.
/// - `callback`: optional loader callback address, or 0.
///
/// # Returns
/// `-1` when the member array cannot be allocated, otherwise `0`.
///
/// # Side effects
/// Allocates `count * 4` bytes, writes `info_c + 0x3C`, and invokes the
/// callback for every archive entry.
///
/// # Safety
/// `this` must hold a mounted archive; `callback` must be null or valid.
///
/// # Confidence
/// high - the allocation, zero-fill, both enumeration arms and the callback
/// context layout are explicit.
pub unsafe fn res_info_load_res(this: usize, callback: usize) -> i32 {
    let archive = platform::read_u32(this + INFO_ARCHIVE);
    let count = platform::call1(ARCHIVE_COUNT_FILE, archive);
    let files = platform::call1(HEAP_ALLOC, count << 2);
    platform::write_u32(this + INFO_FILES, files);
    if files == 0 {
        return -1;
    }

    let base = files as usize;
    for index in 0..count as usize {
        platform::write_u32(base + index * 4, 0);
    }

    if callback == 0 {
        platform::call3(ARCHIVE_GET_FILE_ARRAY, archive, files, count);
    } else {
        let vtable = platform::read_u32(callback) as usize;
        platform::call2(
            platform::read_u32(vtable + 8) as usize,
            callback as u32,
            this as u32,
        );
        // `searchCallback_c`: mpCallback, mpFiles, mNumFiles, mFileIdx,
        // mFolderSig (four spaces).
        let mut search = crate::platform::GuestLocal::new( [callback as u32, files, count, 0, 0x2020_2020]);
        platform::call3(
            ARCHIVE_SEARCH_INSIDE,
            archive,
            SEARCH_CALLBACK as u32,
            search.as_mut_ptr() as u32,
        );
    }
    0
}

// ===========================================================================
// dRes_c registry lifetime: 0x800DEF50..0x800DEFF0
// ===========================================================================

/// Zeroes the three-word registry header (`mpArcInfos`, `mNumArcs`,
/// `mpCallback`).
///
/// # Original
/// - Ghidra: `FUN_800def50` at `0x800DEF50` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800def50_FUN_800def50.c`
/// - observed: `*param_1 = 0; *(undefined2 *)(param_1 + 1) = 0; param_1[2] = 0;`
///
/// # Arguments
/// - `this`: address of a `dRes_c` header.
///
/// # Side effects
/// Writes `+0x00` (u32), `+0x04` (u16) and `+0x08` (u32).
///
/// # Safety
/// `this` must be writable for 0x0C bytes.
///
/// # Confidence
/// high - three stores, no branches.
pub unsafe fn dres_construct(this: usize) {
    platform::write_u32(this + DRES_ARC_INFOS, 0);
    platform::write_u16(this + DRES_NUM_ARCS, 0);
    platform::write_u32(this + DRES_CALLBACK, 0);
}

/// Destroys every holder in the registry and releases the holder array.
///
/// The array is destroyed through Metrowerks `__destroy_arr` with the
/// `info_c` destructor at `0x800DE970` and a 0x40-byte stride, then the header
/// is zeroed. When `deleting` is positive the registry object itself is freed.
///
/// # Original
/// - Ghidra: `FUN_800def70` at `0x800DEF70` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800def70_FUN_800def70.c`
/// - observed: `FUN_802dcb10(*param_1,FUN_800de970); *param_1 = 0;
///   *(undefined2 *)(param_1 + 1) = 0; if (0 < param_2) FUN_802b9280(param_1);`
///
/// # Arguments
/// - `this`: address of a `dRes_c`, or 0.
/// - `deleting`: `> 0` to free the registry object.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Runs every holder destructor and frees the holder array; may free `this`.
///
/// # Safety
/// `this` must be null or a live registry whose array has `mNumArcs` valid
/// 0x40-byte holders.
///
/// # Confidence
/// high - both calls and all three clears are explicit; the missing count and
/// stride operands are inferred from the header layout and the `0x40` stride.
pub unsafe fn dres_destroy(this: usize, deleting: i32) -> usize {
    if this == 0 {
        return 0;
    }
    let infos = platform::read_u32(this + DRES_ARC_INFOS);
    if infos != 0 {
        platform::call4(
            DESTROY_ARRAY,
            infos,
            RES_INFO_DTOR as u32,
            INFO_STRIDE as u32,
            platform::read_u16(this + DRES_NUM_ARCS) as u32,
        );
        platform::write_u32(this + DRES_ARC_INFOS, 0);
        platform::write_u16(this + DRES_NUM_ARCS, 0);
    }
    if deleting > 0 {
        platform::call1(HEAP_DELETE, this as u32);
    }
    this
}

/// Allocates and constructs the fixed-size holder array.
///
/// The allocation is `max_count * 0x40 + 0x10` bytes; the extra 0x10 is the
/// Metrowerks array cookie. `__construct_array` runs the in-object constructor
/// [`RES_INFO_CTOR`] and the destructor [`RES_INFO_DTOR`] (which is the same
/// `0x800DE970` routine used by [`dres_destroy`]). On success the header gets
/// the array pointer, the `u16` count and the callback.
///
/// # Original
/// - Ghidra: `FUN_800deff0` at `0x800DEFF0` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800deff0_FUN_800deff0.c`
/// - observed: `uVar1 = FUN_802b9250((param_2 & 0xffff) * 0x40 + 0x10);
///   iVar2 = FUN_802dc7e0(uVar1,&LAB_800de940,FUN_800de970,0x40,param_2);`
///
/// # Arguments
/// - `this`: address of a `dRes_c` header.
/// - `max_count`: holder count; only the low 16 bits size the allocation.
/// - `callback`: callback pointer stored at `+0x08`.
///
/// # Returns
/// `true` when `__construct_array` returned a non-null array.
///
/// # Side effects
/// Allocates the holder array, runs each element constructor, writes the
/// header.
///
/// # Safety
/// `this` must be writable; `max_count` must match the game's fixed limit when
/// the returned registry is used.
///
/// # Confidence
/// high - allocation size, constructor/destructor arguments and stride are
/// explicit; `0x800DE940` itself is an unexported in-object constructor.
pub unsafe fn dres_init(this: usize, max_count: u32, callback: usize) -> bool {
    let bytes = (max_count & 0xffff) * (INFO_STRIDE as u32) + 0x10;
    let base = platform::call1(HEAP_ALLOC, bytes);
    let array = platform::call5(
        CONSTRUCT_ARRAY,
        base,
        RES_INFO_CTOR as u32,
        RES_INFO_DTOR as u32,
        INFO_STRIDE as u32,
        max_count,
    );
    platform::write_u32(this + DRES_ARC_INFOS, array);
    if array != 0 {
        platform::write_u16(this + DRES_NUM_ARCS, max_count as u16);
        platform::write_u32(this + DRES_CALLBACK, callback as u32);
    }
    array != 0
}

// ===========================================================================
// dRes_c lookup / acquire: 0x800DF080..0x800DF600
// ===========================================================================

/// Finds or creates the holder for `arc_name` and increments its refcount.
///
/// The lookup is the existing slot scan in
/// [`crate::resource::bres::fn_800df570`] (refcount nonzero + `strcmp`). On a
/// miss, [`dres_new_res_info`] finds a free row and [`res_info_set`] starts the
/// mount; a failed mount cleans the row up again. A success increments the
/// `u16` at `info_c + 0x20`.
///
/// # Original
/// - Ghidra: `FUN_800df080` at `0x800DF080` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df080_FUN_800df080.c`
/// - observed: `iVar1 = FUN_800df570(); if (iVar1 == 0) { iVar1 =
///   FUN_800df600(...); ... iVar3 = FUN_800dead0(iVar1,...);
///   if (iVar3 == 0) FUN_800de9f0(iVar1); } *(short *)(iVar1 + 0x20) =
///   *(short *)(iVar1 + 0x20) + 1;`
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
/// - `arc_name`: archive name to find or create.
/// - `folder`: containing folder for a new mount.
/// - `alloc_dir`: allocation direction for a new mount.
/// - `heap`: mount heap, or 0 for the default.
///
/// # Returns
/// `1` on success, `0` when no free row exists or the mount cannot start.
///
/// # Side effects
/// May start a DVD mount, fill a holder, and increments its refcount.
///
/// # Safety
/// `this` must be a live registry; `arc_name`/`folder` must be C strings.
///
/// # Confidence
/// high - the lookup, create, cleanup-on-failure and refcount increment are
/// explicit; Ghidra lost the lookup arguments, recovered from `0x800DF570`.
pub unsafe fn dres_set_res(
    this: usize,
    arc_name: usize,
    folder: usize,
    alloc_dir: u32,
    heap: usize,
) -> u32 {
    let mut slot = bres::fn_800df570(this, arc_name);
    if slot == 0 {
        slot = dres_new_res_info(this);
        if slot == 0 {
            return 0;
        }
        let created = res_info_set(slot as usize, arc_name, folder, alloc_dir, heap);
        if created == 0 {
            res_info_cleanup(slot as usize);
            return 0;
        }
    }
    let address = slot as usize;
    let refcount = platform::read_u16(address + INFO_REFCOUNT);
    platform::write_u16(address + INFO_REFCOUNT, refcount.wrapping_add(1));
    1
}

/// Releases one archive holder found by name.
///
/// The counterpart of [`dres_set_res`]: [`bres::fn_800df570`] resolves
/// `(table, name)` to a `0x40`-byte holder, the refcount at
/// [`INFO_REFCOUNT`] is decremented as a `u16`, and when it reaches zero
/// [`res_info_cleanup`] tears the holder down. A miss returns `0` without
/// touching anything.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800DF130`.
/// - Extent: `0x800DF130..0x800DF17C` (76 bytes, 17 instructions), ending at the
///   `blr` at `0x800DF178`; two zero words precede it at
///   `0x800DF128..0x800DF130` and the next body (`0x800DF180`) starts
///   immediately.
/// - Reach: no `b` or `bl` in the executable segments targets it; no data word
///   names it.
/// - Table: none.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `bl 0x800DF570`; `cmpi cr0,0,r3,0`; `bc 4,2` (`bne`) to the decrement;
///   `li r3,0`; `b` to the epilogue; `lhz r4,0x20(r3)`; `addi r0,r4,-1`;
///   `sth r0,0x20(r3)`; `rlwinm. r0,r0,0,16,31`; `bc 4,2` (`bne`) over the
///   cleanup; `bl 0x800DE9F0`; `li r3,1`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `table`: address of the `{u32 base, u16 count}` pair read by
///   [`bres::fn_800df570`].
/// - `name`: NUL-terminated archive name.
///
/// # Returns
/// `1` after a successful decrement, `0` when the name has no live holder.
///
/// # Side effects
/// Decrements the holder refcount and, at zero, releases the holder's archive
/// through [`res_info_cleanup`].
///
/// # Safety
/// `table`, `name` and the resolved holder must satisfy
/// [`bres::fn_800df570`]'s contract; a holder that reaches zero must still be
/// live for [`res_info_cleanup`].
///
/// # Confidence
/// high - the lookup, the wrap, the zero test and both literal return values
/// are explicit; the `u16` masking is the `rlwinm` pair.
pub unsafe fn fn_800df130(table: usize, name: usize) -> u32 {
    let slot = bres::fn_800df570(table, name) as usize;
    if slot == 0 {
        return 0;
    }
    let count = platform::read_u16(slot + INFO_REFCOUNT).wrapping_sub(1);
    platform::write_u16(slot + INFO_REFCOUNT, count);
    if count == 0 {
        let _ = res_info_cleanup(slot);
    }
    1
}

/// Returns the first holder whose refcount is zero.
///
/// # Original
/// - Ghidra: `FUN_800df600` at `0x800DF600` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df600_FUN_800df600.c`
/// - observed: `uVar1 = *(ushort *)(param_1 + 1); iVar2 = *param_1;` then a
///   scan with a `0x40` stride testing `*(short *)(iVar2 + 0x20) == 0`.
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
///
/// # Returns
/// Address of a free 0x40-byte holder, or `0` when every row is referenced.
///
/// # Side effects
/// None; read-only scan.
///
/// # Safety
/// The registry array must hold `mNumArcs` readable 0x40-byte rows.
///
/// # Confidence
/// high - stride, count and free test are explicit.
pub unsafe fn dres_new_res_info(this: usize) -> u32 {
    let base = platform::read_u32(this + DRES_ARC_INFOS) as usize;
    let count = platform::read_u16(this + DRES_NUM_ARCS) as u32;
    for index in 0..count {
        let slot = base + index as usize * INFO_STRIDE;
        if platform::read_u16(slot + INFO_REFCOUNT) == 0 {
            return slot as u32;
        }
    }
    0
}

/// Advances one holder whose DVD mount has completed.
///
/// The scan starts at row 0 and stops at the first holder with a non-null DVD
/// command whose [`res_info_set_res`] status is positive (still in progress).
/// The original returns `1` in that case and `0` after the whole table.
///
/// # Original
/// - Ghidra: `FUN_800df4e0` at `0x800DF4E0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df4e0_FUN_800df4e0.c`
/// - observed: `if ((*(int *)(iVar3 + 0x24) != 0) && (iVar1 =
///   FUN_800deba0(iVar3,param_1[2]), 0 < iVar1)) break;` with a 0x40 stride.
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
///
/// # Returns
/// `1` when a pending holder was found and advanced, otherwise `0`.
///
/// # Side effects
/// May finish a mount, allocate the member table and run the loader callback.
///
/// # Safety
/// The registry and every row must be live; the stored callback must be null
/// or valid.
///
/// # Confidence
/// high - the loop, stride and early exit are explicit.
pub unsafe fn dres_sync_all_res(this: usize) -> u32 {
    let mut slot = platform::read_u32(this + DRES_ARC_INFOS) as usize;
    let count = platform::read_u16(this + DRES_NUM_ARCS) as u32;
    let callback = platform::read_u32(this + DRES_CALLBACK);
    for _ in 0..count {
        if platform::read_u32(slot + INFO_DVD_CMD) != 0 {
            let status = res_info_set_res(slot, callback as usize);
            if status > 0 {
                return 1;
            }
        }
        slot += INFO_STRIDE;
    }
    0
}

// ===========================================================================
// Typed member accessors: 0x800DF1E0..0x800DF440
// ===========================================================================

/// Opens a member by path and returns its data pointer plus size.
///
/// This is the logging accessor: [`bres::fn_802b76c0`] calls `arc_open`, which
/// emits the "file not found" warning on a miss. `size_out` is written even on
/// failure (zero).
///
/// # Original
/// - Ghidra: `FUN_800df1e0` at `0x800DF1E0` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df1e0_FUN_800df1e0.c`
/// - observed: `iVar1 = FUN_800df640(); ... uVar2 = FUN_802b76c0(*(int *)(iVar1
///   + 0x28),param_3,&local_18); *param_4 = local_14;`
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
/// - `arc_name`: archive name for [`bres::fn_800df640`].
/// - `res_path`: member path inside the archive.
/// - `size_out`: address that receives the member size (never null in the
///   original).
///
/// # Returns
/// Member data pointer, or `0` when the holder, archive or member is missing.
///
/// # Side effects
/// Writes four bytes at `size_out`; may log a missing member.
///
/// # Safety
/// `this`/`arc_name` must satisfy [`bres::fn_800df640`]; `res_path` must be a
/// C string; `size_out` must be writable for four bytes.
///
/// # Confidence
/// high - the size word is `FileInfo + 4`, and the call/`*size` pair is
/// explicit.
pub unsafe fn dres_get_res(
    this: usize,
    arc_name: usize,
    res_path: usize,
    size_out: usize,
) -> u32 {
    let mut info = crate::platform::GuestLocal::new( [0u32; 2]);
    let mut data = 0u32;
    let slot = bres::fn_800df640(this, arc_name);
    if slot != 0 {
        let archive = platform::read_u32(slot as usize + INFO_ARCHIVE);
        if archive != 0 {
            data = bres::fn_802b76c0(
                archive as usize,
                res_path,
                info.as_mut_ptr() as usize,
            );
        }
    }
    platform::write_u32(size_out, info[1]);
    data
}

/// Opens a member, falling back to `<path>.LZ`, and returns size/compression.
///
/// The uncompressed name is tried first through
/// [`bres::fn_802b7760`]/[`bres::fn_802b7770`]. If it is absent, `snprintf`
/// appends `.LZ`, the lookup is retried, and on a hit the first byte's high
/// nibble becomes the compression type while
/// [`CX_GET_UNCOMPRESSED_SIZE`] supplies the size. This is the accessor used by
/// callers that must know whether to run [`CX_UNCOMPRESS_LZ`].
///
/// # Original
/// - Ghidra: `FUN_800df270` at `0x800DF270` (292 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df270_FUN_800df270.c`
/// - observed: `FUN_802e16e8(auStack_120,0x100,unaff_r13 + -0x7708,param_3);
///   ... *param_5 = *pbVar4 & 0xf0; *param_4 = FUN_801d8150(pbVar4);` and the
///   non-LZ arm `*param_5 = 0; *param_4 = local_124;`
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
/// - `arc_name`: archive name.
/// - `res_path`: member path.
/// - `size_out`: address for the uncompressed size, or 0.
/// - `compression_out`: address for the compression type, or 0.
///
/// # Returns
/// Member data pointer, or `0` on a miss.
///
/// # Side effects
/// Writes optional size/compression outputs; may call `snprintf`.
///
/// # Safety
/// `this`/`arc_name` must satisfy [`bres::fn_800df640`]; `res_path` must be a
/// C string; the output pointers must be null or writable.
///
/// # Confidence
/// high - the fallback, size and compression writes are explicit; the LZ format
/// string at `0x80427F78` is the literal `"%s.LZ"` read from the image.
pub unsafe fn dres_get_res_compressed(
    this: usize,
    arc_name: usize,
    res_path: usize,
    size_out: usize,
    compression_out: usize,
) -> u32 {
    let mut data = 0u32;
    let slot = bres::fn_800df640(this, arc_name);
    if slot != 0 {
        let archive = platform::read_u32(slot as usize + INFO_ARCHIVE);
        if archive != 0 {
            let mut entry = bres::fn_802b7760(archive as usize, res_path);
            if entry < 0 {
                let mut compressed_path = crate::platform::GuestLocal::new( [0u8; 0x100]);
                platform::call4(
                    SNPRINTF,
                    compressed_path.as_mut_ptr() as u32,
                    0x100,
                    LZ_SUFFIX_FORMAT as u32,
                    res_path as u32,
                );
                entry = bres::fn_802b7760(
                    archive as usize,
                    compressed_path.as_ptr() as usize,
                );
                if entry >= 0 {
                    data = bres::fn_802b7770(archive as usize, entry as u32, 0);
                    if data != 0 {
                        if compression_out != 0 {
                            let kind = platform::read_u8(data as usize) & 0xf0;
                            platform::write_u32(compression_out, kind as u32);
                        }
                        if size_out != 0 {
                            let size =
                                platform::call1(CX_GET_UNCOMPRESSED_SIZE, data);
                            platform::write_u32(size_out, size);
                        }
                    }
                }
            } else {
                let mut info = crate::platform::GuestLocal::new( [0u32; 2]);
                data = bres::fn_802b7770(
                    archive as usize,
                    entry as u32,
                    info.as_mut_ptr() as usize,
                );
                if data != 0 {
                    if compression_out != 0 {
                        platform::write_u32(compression_out, 0);
                    }
                    if size_out != 0 {
                        platform::write_u32(size_out, info[1]);
                    }
                }
            }
        }
    }
    data
}

/// Returns a member pointer by path without logging on a miss.
///
/// # Original
/// - Ghidra: `FUN_800df3c0` at `0x800DF3C0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df3c0_FUN_800df3c0.c`
/// - observed: `iVar2 = FUN_802b7760(iVar1,param_3); if (-1 < iVar2)
///   uVar3 = FUN_802b7770(iVar1,iVar2,0);`
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
/// - `arc_name`: archive name.
/// - `res_path`: member path.
///
/// # Returns
/// Member data pointer, or `0` on a miss.
///
/// # Side effects
/// None visible; no warning is emitted.
///
/// # Safety
/// `this`/`arc_name` must satisfy [`bres::fn_800df640`]; `res_path` a C string.
///
/// # Confidence
/// high - the index lookup and null-on-miss path are explicit.
pub unsafe fn dres_get_res_silently(
    this: usize,
    arc_name: usize,
    res_path: usize,
) -> u32 {
    let mut data = 0u32;
    let slot = bres::fn_800df640(this, arc_name);
    if slot != 0 {
        let archive = platform::read_u32(slot as usize + INFO_ARCHIVE);
        if archive != 0 {
            let entry = bres::fn_802b7760(archive as usize, res_path);
            if entry >= 0 {
                data = bres::fn_802b7770(archive as usize, entry as u32, 0);
            }
        }
    }
    data
}

/// Returns a member pointer plus size without logging on a miss.
///
/// `size_out` is written even when the member is absent (zero).
///
/// # Original
/// - Ghidra: `FUN_800df440` at `0x800DF440` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df440_FUN_800df440.c`
/// - observed: `iVar2 = FUN_802b7760(iVar1,param_3); -1 < iVar2; ...
///   uVar4 = FUN_802b7770(iVar1,iVar2,&local_28); uVar3 = local_24;
///   *param_4 = uVar3;`
///
/// # Arguments
/// - `this`: address of a live `dRes_c`.
/// - `arc_name`: archive name.
/// - `res_path`: member path.
/// - `size_out`: address receiving the size.
///
/// # Returns
/// Member data pointer, or `0` on a miss.
///
/// # Side effects
/// Writes four bytes at `size_out`.
///
/// # Safety
/// Same pointer requirements as [`dres_get_res_silently`], plus a writable
/// `size_out`.
///
/// # Confidence
/// high - the output word is `FileInfo + 4` and is written unconditionally.
pub unsafe fn dres_get_res_silently_size(
    this: usize,
    arc_name: usize,
    res_path: usize,
    size_out: usize,
) -> u32 {
    let mut size = 0u32;
    let mut data = 0u32;
    let slot = bres::fn_800df640(this, arc_name);
    if slot != 0 {
        let archive = platform::read_u32(slot as usize + INFO_ARCHIVE);
        if archive != 0 {
            let entry = bres::fn_802b7760(archive as usize, res_path);
            if entry >= 0 {
                let mut info = crate::platform::GuestLocal::new( [0u32; 2]);
                data = bres::fn_802b7770(
                    archive as usize,
                    entry as u32,
                    info.as_mut_ptr() as usize,
                );
                if data != 0 {
                    size = info[1];
                }
            }
        }
    }
    platform::write_u32(size_out, size);
    data
}

/// Copies one uncompressed member.
///
/// # Original
/// - Ghidra: `FUN_800df3a0` at `0x800DF3A0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df3a0_FUN_800df3a0.c`
/// - observed: `FUN_80004364(param_2,param_1);` - Ghidra drops the third
///   argument, which is still in `r5` at the tail call.
///
/// # Arguments
/// - `from`: source bytes.
/// - `to`: destination bytes.
/// - `size`: byte count (PowerPC `r5`).
///
/// # Side effects
/// Writes `size` bytes at `to`.
///
/// # Safety
/// `from..from+size` readable and `to..to+size` writable, non-overlapping in
/// the sense required by `memcpy`.
///
/// # Confidence
/// high - the tail call is `memcpy`; only the register-only length is inferred
/// from the ABI and the public `copyRes(from,to,size)` shape.
pub unsafe fn dres_copy_res(from: usize, to: usize, size: u32) {
    platform::call3(MEMCPY, to as u32, from as u32, size);
}

/// Copies or LZ-decompresses one member, depending on `compression`.
///
/// A zero compression type is a plain [`dres_copy_res`]; any other value calls
/// `CXUncompressLZ(from, to)` with no explicit length.
///
/// # Original
/// - Ghidra: `FUN_800df3b0` at `0x800DF3B0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df3b0_FUN_800df3b0.c`
/// - observed: `if (in_r6 == 0) { FUN_800df3a0(); return; }
///   FUN_801d8190();`
///
/// # Arguments
/// - `from`: source bytes.
/// - `to`: destination bytes.
/// - `size`: byte count for the uncompressed arm.
/// - `compression`: 0 for a plain copy, nonzero for LZ.
///
/// # Side effects
/// Writes decompressed or copied bytes at `to`.
///
/// # Safety
/// `from`/`to` must be valid for the selected operation; `size` only applies
/// to the uncompressed arm.
///
/// # Confidence
/// high - the `r6` test and both tail calls are explicit; `0x801D8190` is
/// `CXUncompressLZ` (P1 `0x801D82D0` shifted by the E1 `-0x140` delta).
pub unsafe fn dres_copy_res_compressed(
    from: usize,
    to: usize,
    size: u32,
    compression: u32,
) {
    if compression == 0 {
        dres_copy_res(from, to, size);
    } else {
        platform::call2(CX_UNCOMPRESS_LZ, from as u32, to as u32);
    }
}

// ===========================================================================
// dResMng_c singleton: 0x800DF680..0x800DF840
// ===========================================================================

/// Constructs the `dResMng_c` singleton and installs its 300 holders.
///
/// The object starts with the manager vtable, an embedded `dRes_c` at `+0x04`,
/// and the callback vtable at `+0x10`. The constructor stores the object in
/// [`G_RES_MNG_INSTANCE`] and calls [`dres_init`] with `MAX_ARCHIVES`.
///
/// # Original
/// - Ghidra: `FUN_800df680` at `0x800DF680` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df680_FUN_800df680.c`
/// - observed: `*param_1 = &DAT_80319550; FUN_800def50(param_1 + 1);
///   param_1[4] = &DAT_80319560; *(r13-0x5648) = param_1;
///   FUN_800deff0(param_1 + 1,300,param_1 + 4);`
///
/// # Arguments
/// - `this`: address of the `dResMng_c` object.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes the two vtables, the singleton global, and allocates the holder
/// array.
///
/// # Safety
/// `this` must be writable for at least 0x14 bytes and remain alive.
///
/// # Confidence
/// high - every store and call is explicit; the vtable addresses are named
/// from the `dResMng_c`/`resCallback_c` layout.
pub unsafe fn dres_mng_construct(this: usize) -> usize {
    platform::write_u32(this, RES_MNG_VTABLE as u32);
    dres_construct(this + 0x04);
    platform::write_u32(this + 0x10, RES_CALLBACK_VTABLE as u32);
    platform::write_u32(G_RES_MNG_INSTANCE, this as u32);
    dres_init(this + 0x04, MAX_ARCHIVES, this + 0x10);
    this
}

/// Clears the `dResMng_c` singleton and destroys its registry.
///
/// # Original
/// - Ghidra: `FUN_800df6e0` at `0x800DF6E0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df6e0_FUN_800df6e0.c`
/// - observed: `if (param_1 != 0) { *(r13-0x5648) = 0;
///   FUN_800def70(param_1 + 4,0xffffffff); if (0 < param_2) FUN_802b9280(param_1); }`
///
/// # Arguments
/// - `this`: address of the singleton, or 0.
/// - `deleting`: `> 0` to free the manager object.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Clears [`G_RES_MNG_INSTANCE`], destroys every holder, and may free `this`.
///
/// # Safety
/// `this` must be null or the live singleton.
///
/// # Confidence
/// high - the global clear, embedded-registry destroy and conditional free are
/// explicit.
pub unsafe fn dres_mng_destroy(this: usize, deleting: i32) -> usize {
    if this == 0 {
        return 0;
    }
    platform::write_u32(G_RES_MNG_INSTANCE, 0);
    dres_destroy(this + 0x04, -1);
    if deleting > 0 {
        platform::call1(HEAP_DELETE, this as u32);
    }
    this
}

/// `dResMng_c` slot `+0x1C`: validate and bind one `"g3d "` resource.
///
/// The only word that names this body is `0x8031956C`, slot `+0x1C` of
/// [`RES_MNG_VTABLE`]. The third argument is checked against the four-character
/// resource tag `0x67336420` (`"g3d "`); when it matches, the BRRES image in
/// the second argument is validated with
/// [`crate::resource::bres::brres_check_revision`], bound with
/// [`crate::resource::bres::brres_bind_all`] and then handed to the deferred
/// bind/check [`crate::resource::bres::brres_bind_check`] through a second copy
/// of the same image word. The second argument is returned unchanged.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x800DF760`.
/// - Extent: `0x800DF760..0x800DF7C8` (104 bytes, 26 words), ending at the
///   `blr` at `0x800DF7C4`; two zero words follow at
///   `0x800DF7C8..0x800DF7D0` and the next body starts at `0x800DF7D0`.
/// - Reach: no `b`/`bl` in any image; the data word `0x8031956C` (slot `+0x1C`
///   of `0x80319550`) names it, which is why this is the vtable-only callback
///   listed in the module header.
/// - Observed: `9421FFE0 stwu r1,-0x20(r1)`; `mflr r0`; `stw r0,0x24(r1)`;
///   `3C0598CD addis r0,r5,0x98CD`; `28006420 cmpli cr0,r0,0x6420`;
///   `stw r31,0x1C(r1)`; `mr r31,r4`; `40820034 bc 4,2` (bne, return);
///   `stw r4,0xC(r1)`; `addi r3,r1,0xC`; `bl 0x8023A860`;
///   `cmpi cr0,r3,0`; `41820020 bc 12,2` (beq, return);
///   `addi r3,r1,0xC`; `bl 0x8023A590`; `lwz r0,0xC(r1)`;
///   `addi r3,r1,0xC`; `stw r0,0x8(r1)`; `addi r4,r1,0x8`;
///   `bl 0x8023A350`; `mr r3,r31`; epilogue; `4E800020 blr`.
/// - Decoder note: `addis r0,r5,0x98CD` + `cmpli 0x6420` is the compiler's
///   folded `r5 == 0x67336420` (adding `0x98CD0000` wraps the high half to
///   zero because `0x6733 + 0x98CD = 0x10000`); the constant is the ASCII tag
///   `"g3d "`.
/// - Evidence: `work/wave286/e_dis.txt`, `work/wave286/e_refs.txt`.
///
/// # Arguments
/// - `_manager`: `dResMng_c` receiver; the body never reads it.
/// - `image`: BRRES image pointer, copied into the stack word the three BRRES
///   helpers receive (`r4`).
/// - `kind`: resource tag, accepted only for `0x67336420` (`r5`).
///
/// # Returns
/// `image`, on both the accepted and the rejected path.
///
/// # Side effects
/// One validate pass, one binding pass and one deferred bind/check pass; the
/// helpers may cache TEX0/palette pointers inside the image.
///
/// # Safety
/// `image` must address a readable BRRES image with complete node tables.
///
/// # Confidence
/// medium - the guard, both stack copies and all three calls are literal; the
/// callback's role is inferred from the BRRES helpers it forwards to and from
/// the `dResMng_c` vtable slot that names it.
pub unsafe fn fn_800df760(_manager: usize, image: u32, kind: u32) -> u32 {
    if kind == 0x6733_6420 {
        let mut slot = crate::platform::GuestLocal::new( [image]);
        let slot_addr = slot.as_mut_ptr() as usize;
        if bres::brres_check_revision(slot_addr) {
            bres::brres_bind_all(slot_addr);
            let copy = crate::platform::GuestLocal::new( [platform::read_u32(slot_addr)]);
            bres::brres_bind_check(slot_addr, copy.as_ptr() as usize);
        }
    }
    image
}

/// Schedules an array of archive names on the embedded `dRes_c`.
///
/// Each name is registered through [`dres_set_res`] with the same containing
/// folder, a zero allocation direction and the caller's heap.
///
/// # Original
/// - Ghidra: `FUN_800df7d0` at `0x800DF7D0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df7d0_FUN_800df7d0.c`
/// - observed: `for (iVar1 = 0; iVar1 < param_4; iVar1 = iVar1 + 1) {
///   FUN_800df080(...,*param_3,...,0,param_5); param_3 = param_3 + 1; }`
///
/// # Arguments
/// - `this`: address of the `dResMng_c` singleton.
/// - `path`: containing folder for every archive.
/// - `names`: address of a `u32` array of C-string pointers.
/// - `count`: number of names.
/// - `heap`: mount heap, or 0 for the default.
///
/// # Side effects
/// Starts one mount per name and bumps each holder's refcount.
///
/// # Safety
/// `this` must be the live singleton; `names` must hold `count` readable
/// pointers; each name and `path` must be C strings.
///
/// # Confidence
/// high - the loop bound, pointer advance and argument order are explicit.
pub unsafe fn dres_mng_set_res_array(
    this: usize,
    path: usize,
    names: usize,
    count: u32,
    heap: usize,
) {
    for index in 0..count {
        let name = platform::read_u32(names + index as usize * 4);
        dres_set_res(this + 0x04, name as usize, path, 0, heap);
    }
}

// ===========================================================================
// dResMng_c thin setter: 0x800DF840
// ===========================================================================

/// Registers one archive through the manager's embedded `dRes_c`.
///
/// This is a one-call wrapper: it forwards `folder` and `name` to
/// [`dres_set_res`] on the registry at `this + 0x04`, with allocation
/// direction `0` and the caller's heap. The source order is easy to misread:
/// Ghidra numbers the arguments `(manager, folder, name, heap)` while
/// `dres_set_res` takes `(registry, name, folder, alloc_dir, heap)`.
///
/// # Original
/// - Ghidra: `FUN_800df840` at `0x800DF840` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df840_FUN_800df840.c`
/// - observed: `FUN_800df080(param_1 + 4,param_3,param_2,0,param_4);`
/// - Call site `0x8015BE60` passes `(singleton, "__US_save_banner",
///   "save_banner", 0)`, confirming "second argument = folder".
///
/// # Arguments
/// - `this`: address of the live `dResMng_c` singleton.
/// - `folder`: containing folder C string (`param_2`).
/// - `name`: archive name C string (`param_3`).
/// - `heap`: mount heap, or 0 for the default.
///
/// # Side effects
/// Starts a mount when the holder is new and increments the holder refcount;
/// see [`dres_set_res`].
///
/// # Safety
/// `this` must be the live singleton whose embedded registry was initialised by
/// [`dres_mng_construct`]; the two strings must be readable and `heap` must be
/// null or a live EGG heap.
///
/// # Confidence
/// high - the whole body is one call and the argument permutation is confirmed
/// by a shipped call site.
pub unsafe fn dres_mng_set_res(this: usize, folder: usize, name: usize, heap: usize) {
    dres_set_res(this + 0x04, name, folder, 0, heap);
}

// ===========================================================================
// Mount-command lifetime and queue: 0x8016B440..0x8016B770
//
// `res_info_set` allocates one of these commands and `res_info_cleanup` waits
// for it and reads its archive. A single global manager
// ([`G_MOUNT_QUEUE`]) owns a mutex at `+0x00`, a pending-command head/tail at
// `+0x18`/`+0x1C`, and two OS thread queues at `+0x20` (work available) and
// `+0x28` (command finished).
// ===========================================================================

/// Base-command deleting destructor: wait for the command to finish, then free.
///
/// The destructor installs the base vtable, waits for the command's done byte,
/// and releases the object when the deleting flag is positive. The wait is on
/// the queue's done thread-queue, so it always runs after the worker has
/// published its result.
///
/// # Original
/// - Ghidra: `FUN_8016b440` at `0x8016B440` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b440_FUN_8016b440.c`
/// - observed: `*param_1 = &DAT_8032977c; FUN_8016b4f0(); if (0 < param_2)
///   FUN_8016b4b0(param_1);`
/// - `// inferred:` the two helper calls omit their argument in the C; `this`
///   is still in `r3`, so both receive `this`.
///
/// # Arguments
/// - `this`: command object, or 0.
/// - `deleting`: `> 0` frees the object after the wait.
///
/// # Returns
/// `this` unchanged.
///
/// # Safety
/// A non-null `this` must be a live command whose `+8` done byte is written by
/// another thread; the wait has no timeout in the original.
///
/// # Confidence
/// high - the wait, vtable store and free guard are explicit.
pub unsafe fn mount_cmd_destroy_wait(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, MOUNT_CMD_BASE_VTABLE as u32);
        mount_cmd_wait_done(this);
        if deleting > 0 {
            mount_cmd_free(this);
        }
    }
    this
}

/// Allocates one mount command from the global mount heap.
///
/// # Original
/// - Ghidra: `FUN_8016b4a0` at `0x8016B4A0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b4a0_FUN_8016b4a0.c`
/// - observed: `FUN_802b8cc0(param_1,0xfffffffc,*(undefined4 *)(r13 + -0x5260));`
/// - `// inferred:` the C prints `void`, but every caller (for example
///   `0x8016BCC0`) stores the result, so the allocation pointer is returned.
///
/// # Arguments
/// - `size`: byte count of the command object (`0x1C` or `0x20` at the two
///   shipped call sites).
///
/// # Returns
/// The allocated object, or 0.
///
/// # Side effects
/// Allocates from [`G_MOUNT_HEAP`].
///
/// # Safety
/// The mount heap must already be initialised; `size` is not checked.
///
/// # Confidence
/// high - one explicit call; only the return type is recovered from call sites.
pub unsafe fn mount_cmd_alloc(size: u32) -> u32 {
    platform::call3(
        EGG_ALLOC_ALIGNED,
        size,
        0xFFFF_FFFC,
        platform::read_u32(G_MOUNT_HEAP),
    )
}

/// Releases one mount command to the global mount heap.
///
/// # Original
/// - Ghidra: `FUN_8016b4b0` at `0x8016B4B0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b4b0_FUN_8016b4b0.c`
/// - observed: `FUN_802b8f70(param_1,*(undefined4 *)(r13 + -0x5260));`
///
/// # Arguments
/// - `this`: command object previously returned by [`mount_cmd_alloc`].
///
/// # Safety
/// `this` must be a live allocation from the mount heap and must not be used
/// afterwards.
///
/// # Confidence
/// high - one explicit free call.
pub unsafe fn mount_cmd_free(this: usize) {
    platform::call2(
        HEAP_FREE_EXPLICIT,
        this as u32,
        platform::read_u32(G_MOUNT_HEAP),
    );
}

/// Marks a command finished and wakes the thread waiting on the done queue.
///
/// # Original
/// - Ghidra: `FUN_8016b4d0` at `0x8016B4D0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b4d0_FUN_8016b4d0.c`
/// - observed: `*(undefined1 *)(param_1 + 8) = 1;
///   FUN_801b5d80(*(int *)(r13 + -0x5254) + 0x28);`
///
/// # Arguments
/// - `this`: the command that just completed.
///
/// # Side effects
/// Writes `+0x08` and wakes [`G_MOUNT_QUEUE`]`+0x28`.
///
/// # Safety
/// The global queue must have been initialised by [`mount_queue_init`].
///
/// # Confidence
/// high - one store and one wakeup.
pub unsafe fn mount_cmd_mark_done(this: usize) {
    platform::write_u8(this + 0x08, 1);
    let queue = platform::read_u32(G_MOUNT_QUEUE) as usize;
    platform::call1(OS_WAKEUP_THREAD, (queue + 0x28) as u32);
}

/// Waits until the command's done byte at `+0x08` becomes nonzero.
///
/// The original sleeps on the global queue's done thread-queue. There is no
/// timeout and no cancellation path.
///
/// # Original
/// - Ghidra: `FUN_8016b4f0` at `0x8016B4F0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b4f0_FUN_8016b4f0.c`
/// - observed: `while (*(char *)(param_1 + 8) == '\0') {
///   FUN_801b5c90(*(int *)(r13 + -0x5254) + 0x28); }`
///
/// # Arguments
/// - `this`: the command to wait for.
///
/// # Safety
/// Must be called with the global queue initialised and another thread able to
/// run [`mount_cmd_mark_done`].
///
/// # Confidence
/// high - the loop and the queue offset are explicit.
pub unsafe fn mount_cmd_wait_done(this: usize) {
    while platform::read_u8(this + 0x08) == 0 {
        let queue = platform::read_u32(G_MOUNT_QUEUE) as usize;
        platform::call1(OS_SLEEP_THREAD, (queue + 0x28) as u32);
    }
}

/// Initialises the global mount-command queue.
///
/// The object layout used by this cluster is:
///
/// | offset | meaning |
/// |---|---|
/// | `+0x00` | mutex locked by [`mount_queue_submit`] |
/// | `+0x18` | pending head |
/// | `+0x1C` | pending tail |
/// | `+0x20` | "work available" thread queue |
/// | `+0x28` | "command finished" thread queue |
///
/// # Original
/// - Ghidra: `FUN_8016b540` at `0x8016B540` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b540_FUN_8016b540.c`
/// - observed: `+0x18 = 0; +0x1c = 0; FUN_801b2e20(); FUN_801b4b20(+0x20);
///   FUN_801b4b20(+0x28); *(r13 + -0x5254) = param_1;`
/// - `// inferred:` `FUN_801b2e20` prints no argument but still has `this` in
///   `r3`; the mutex is at `+0x00`, matching the lock in `0x8016B5C0`.
///
/// # Arguments
/// - `this`: writable queue object (at least `0x2C` bytes).
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Initialises the mutex and two thread queues and publishes the object in
/// [`G_MOUNT_QUEUE`].
///
/// # Safety
/// `this` must remain alive and must not be used by another thread until this
/// returns.
///
/// # Confidence
/// high - all stores/calls are explicit; the mutex offset is inferred from the
/// lock call.
pub unsafe fn mount_queue_init(this: usize) -> usize {
    platform::write_u32(this + 0x18, 0);
    platform::write_u32(this + 0x1C, 0);
    platform::call1(OS_INIT_MUTEX, this as u32);
    platform::call1(OS_INIT_THREAD_QUEUE, (this + 0x20) as u32);
    platform::call1(OS_INIT_THREAD_QUEUE, (this + 0x28) as u32);
    platform::write_u32(G_MOUNT_QUEUE, this as u32);
    this
}

/// Wakes a thread sleeping on the queue's "work available" condition.
///
/// # Original
/// - Ghidra: `FUN_8016b590` at `0x8016B590` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b590_FUN_8016b590.c`
/// - observed: `FUN_801b5d80(param_1 + 0x20);`
///
/// # Arguments
/// - `this`: the global queue object.
///
/// # Confidence
/// high - one wakeup on `+0x20`.
pub unsafe fn mount_queue_wake(this: usize) {
    platform::call1(OS_WAKEUP_THREAD, (this + 0x20) as u32);
}

/// Sleeps on the queue's "work available" condition.
///
/// # Original
/// - Ghidra: `FUN_8016b5a0` at `0x8016B5A0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b5a0_FUN_8016b5a0.c`
/// - observed: `FUN_801b5c90(param_1 + 0x20);`
///
/// # Arguments
/// - `this`: the global queue object.
///
/// # Confidence
/// high - one sleep on `+0x20`.
pub unsafe fn mount_queue_sleep(this: usize) {
    platform::call1(OS_SLEEP_THREAD, (this + 0x20) as u32);
}

/// Returns the pending head of the mount queue.
///
/// # Original
/// - Ghidra: `FUN_8016b5b0` at `0x8016B5B0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b5b0_FUN_8016b5b0.c`
/// - observed: `return *(undefined4 *)(param_1 + 0x18);`
///
/// # Arguments
/// - `this`: the global queue object.
///
/// # Returns
/// The first pending command, or 0.
///
/// # Confidence
/// high - one load.
pub unsafe fn mount_queue_peek(this: usize) -> u32 {
    platform::read_u32(this + 0x18)
}

/// Submits a command to the queue, or runs it inline on the queue thread.
///
/// The original compares [`G_MOUNT_THREAD`] with `OSGetCurrentThread`; equal
/// means "we are the consumer", so the command's vtable slot `+0x0C` runs
/// immediately. Otherwise the command is appended under the queue mutex and the
/// consumer is woken.
///
/// # Original
/// - Ghidra: `FUN_8016b5c0` at `0x8016B5C0` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b5c0_FUN_8016b5c0.c`
/// - observed: `iVar1 = FUN_8016b170(); iVar2 = FUN_801b4b30();
///   if (iVar2 == iVar1) (**(code **)(*param_2 + 0xc))(param_2);
///   else { lock; append at +0x1c; unlock; 0x8016B590(param_1); }`
///
/// # Arguments
/// - `this`: the global queue object.
/// - `node`: command to submit; its `+4` next pointer is cleared.
///
/// # Side effects
/// May run `node` synchronously, otherwise links it into the queue and wakes
/// the consumer.
///
/// # Safety
/// `node` must be a live command; the queue must be initialised.
///
/// # Confidence
/// high - the comparison, both branches and the list stores are explicit.
pub unsafe fn mount_queue_submit(this: usize, node: usize) {
    let queue_thread = platform::read_u32(G_MOUNT_THREAD);
    let current = call0_u32(OS_GET_CURRENT_THREAD);
    if current == queue_thread {
        let vtable = platform::read_u32(node) as usize;
        platform::call1(platform::read_u32(vtable + 0x0C) as usize, node as u32);
    } else {
        platform::call1(OS_MUTEX_LOCK, this as u32);
        platform::write_u32(node + 0x04, 0);
        let tail = platform::read_u32(this + 0x1C) as usize;
        if tail == 0 {
            platform::write_u32(this + 0x1C, node as u32);
            platform::write_u32(this + 0x18, node as u32);
        } else {
            platform::write_u32(tail + 0x04, node as u32);
            platform::write_u32(this + 0x1C, node as u32);
        }
        platform::call1(OS_MUTEX_UNLOCK, this as u32);
        mount_queue_wake(this);
    }
}

/// Removes one command from the pending list.
///
/// The scan follows the `+0x04` next pointer. Removing the only node clears
/// both head and tail; removing the tail updates the tail to the previous node.
/// A command that is not found leaves the list unchanged, but the mutex is
/// still unlocked and the consumer is still woken, exactly as the original.
///
/// # Original
/// - Ghidra: `FUN_8016b670` at `0x8016B670` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b670_FUN_8016b670.c`
/// - observed: the `+0x18` head walk, single-node clear, tail update and the
///   trailing `FUN_801b2f40` + `FUN_8016b590`.
///
/// # Arguments
/// - `this`: the global queue object.
/// - `node`: command to unlink.
///
/// # Side effects
/// Rewrites list links, unlocks the queue mutex, wakes the consumer.
///
/// # Safety
/// `this` must be the live queue and `node` must be a node that may be in it.
///
/// # Confidence
/// high - the full unlink logic is explicit.
pub unsafe fn mount_queue_remove(this: usize, node: usize) {
    platform::call1(OS_MUTEX_LOCK, this as u32);
    let mut previous = 0usize;
    let mut current = platform::read_u32(this + 0x18) as usize;
    while current != 0 {
        if current == node {
            if previous == 0 {
                if platform::read_u32(this + 0x18) == platform::read_u32(this + 0x1C) {
                    platform::write_u32(this + 0x18, 0);
                    platform::write_u32(this + 0x1C, 0);
                } else {
                    platform::write_u32(this + 0x18, platform::read_u32(current + 0x04));
                }
            } else {
                platform::write_u32(previous + 0x04, platform::read_u32(current + 0x04));
                if platform::read_u32(this + 0x1C) == current as u32 {
                    platform::write_u32(this + 0x1C, previous as u32);
                }
            }
        }
        previous = current;
        current = platform::read_u32(current + 0x04) as usize;
    }
    platform::call1(OS_MUTEX_UNLOCK, this as u32);
    mount_queue_wake(this);
}

/// Runs one queued command through its vtable slot `+0x0C`.
///
/// # Original
/// - Ghidra: `FUN_8016b730` at `0x8016B730` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b730_FUN_8016b730.c`
/// - observed: `(**(code **)(*(int *)*param_1 + 0xc))(); return 0;`
/// - `// inferred:` `param_1` points at a slot holding the command pointer, and
///   the command is still in `r3` when the virtual slot runs.
///
/// # Arguments
/// - `slot`: address of a word holding the command address.
///
/// # Returns
/// Always 0.
///
/// # Side effects
/// Runs the command's virtual `execute` slot; the body is command-specific.
///
/// # Safety
/// `slot` must hold a live command whose vtable `+0x0C` is callable.
///
/// # Confidence
/// medium - the pointer chain and call are explicit; the `this` argument is
/// inferred from the ABI.
pub unsafe fn mount_cmd_run(slot: usize) -> u32 {
    let node = platform::read_u32(slot) as usize;
    let vtable = platform::read_u32(node) as usize;
    platform::call1(platform::read_u32(vtable + 0x0C) as usize, node as u32);
    0
}

/// Consumer loop: wait for work, then run every pending command.
///
/// The original is an infinite loop with no exit; it sleeps on the queue's
/// work condition, drains the list (remove then run), and repeats.
///
/// # Original
/// - Ghidra: `FUN_8016b770` at `0x8016B770` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b770_FUN_8016b770.c`
/// - observed: `do { FUN_8016b5a0(param_1,param_2); while (true) {
///   local_18[0] = FUN_8016b5b0(param_1); if (local_18[0] == 0) break;
///   FUN_8016b670(param_1); FUN_8016b730(local_18); } param_2 = 0; } while (true);`
/// - `// inferred:` the C drops the node argument to `FUN_8016b670`; `r4` is
///   still `local_18[0]`, so the remove call receives `(queue, node)`.
///
/// # Arguments
/// - `this`: the global queue object.
///
/// # Side effects
/// Never returns; runs command bodies submitted by other threads.
///
/// # Safety
/// The queue must be initialised and this loop must be the only consumer.
///
/// # Confidence
/// high - the loop structure and list operations are explicit.
pub unsafe fn mount_queue_drain(this: usize) {
    loop {
        mount_queue_sleep(this);
        loop {
            let node = mount_queue_peek(this);
            if node == 0 {
                break;
            }
            mount_queue_remove(this, node as usize);
            let mut slot = crate::platform::GuestLocal::new( node);
            mount_cmd_run((slot.as_mut_ptr() as *mut u32) as usize);
        }
    }
}

// ---------------------------------------------------------------------------
// Wave 275 lane E - untabled gap body after the consumer loop (`_E275`)
// ---------------------------------------------------------------------------

/// Runs the mount-queue consumer and then reports success.
///
/// The body forwards its first argument unchanged to the infinite
/// [`mount_queue_drain`] loop, so the trailing `return 0` is unreachable in the
/// shipped image; it is kept because the instruction stream ends on it.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8016B190`.
/// - Extent: `0x8016B190..0x8016B1B4` (36 bytes, 9 instructions), ending at the
///   `blr` at `0x8016B1B0`; two zero words precede it at
///   `0x8016B188..0x8016B190` and two follow before `0x8016B1C0`.
/// - Reach: one tail `b` at `0x8016B214` targets it; no `bl` does.
/// - Table: no data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `bl 0x8016B770`; `lwz r0,0x14(r1)`; `li r3,0`; `mtlr r0`;
///   `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: the global mount-command queue object.
///
/// # Returns
/// `0`, if the callee ever returned.
///
/// # Side effects
/// Delegates to [`mount_queue_drain`], which never returns.
///
/// # Safety
/// The queue must be initialised and this must be its only consumer.
///
/// # Confidence
/// high - one argument-forwarding call and a literal `li r3,0`.
pub unsafe fn fn_8016b190(this: usize) -> u32 {
    mount_queue_drain(this);
    0
}

// ===========================================================================
// Mount archive table and lookup: 0x8016B090..0x8016B180
//
// The table is a small array of pointers to archive-manager objects. Each
// manager carries a tag byte at +0x04 and a file-extension string at +0x08.
// The shipped image installs one entry; the lookup bodies are nevertheless
// generic over the table bounds.
// ===========================================================================

/// Publishes the two bounds of the mount-archive table.
///
/// # Original
/// - Ghidra: `FUN_8016b090` at `0x8016B090` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b090_FUN_8016b090.c`
/// - observed: `*(r13 - 0x5270) = param_1; *(r13 - 0x526c) = param_2;`
///
/// # Arguments
/// - `start`: address of the first slot (a `u32 *` array).
/// - `end`: address one past the last slot.
///
/// # Side effects
/// Writes the two SDA words read by [`mount_archive_find`].
///
/// # Safety
/// `start..end` must remain readable for the lifetime of the mount system and
/// every slot must hold a pointer to a live manager object.
///
/// # Confidence
/// high - two stores, no branches.
pub unsafe fn mount_archive_table_set(start: u32, end: u32) {
    platform::write_u32(G_MOUNT_ARCHIVE_TABLE, start);
    platform::write_u32(G_MOUNT_ARCHIVE_TABLE_END, end);
}

/// Installs the shipped one-slot archive table.
///
/// `0x8008F110` calls this immediately after [`mount_stream_manager_init`]. The
/// static slot at `0x80429464` contains `0x80377AF0`, the process-wide archive
/// manager whose extension is `".szs"` (the string at `0x80429470`).
///
/// # Original
/// - Ghidra: `FUN_8016b0a0` at `0x8016B0A0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b0a0_FUN_8016b0a0.c`
/// - observed: `FUN_8016b090(r13 + -0x621c, r13 + -0x6218);`
/// - `// observed:` the image bytes at `0x80429464` are `f0 7a 37 80`
///   (little-endian pointer `0x80377AF0`); `0x80429468` starts the next object.
///
/// # Side effects
/// Calls [`mount_archive_table_set`] with the static slot bounds.
///
/// # Safety
/// Must run before any mount command resolves a path.
///
/// # Confidence
/// high - one call and the static table bytes are visible in the image.
pub unsafe fn mount_archive_table_init() {
    mount_archive_table_set(
        MOUNT_ARCHIVE_TABLE_STATIC as u32,
        MOUNT_ARCHIVE_TABLE_STATIC_END as u32,
    );
}

/// Finds the archive manager with the requested tag byte.
///
/// # Original
/// - Ghidra: `FUN_8016b0b0` at `0x8016B0B0` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b0b0_FUN_8016b0b0.c`
/// - observed: walks `r13 - 0x5270` to `r13 - 0x526c` in 4-byte steps and
///   compares `*(u8 *)(*slot + 4)` with `param_1`.
///
/// # Arguments
/// - `tag`: tag byte to match.
///
/// # Returns
/// The matching manager object, or 0 when the table has no such entry.
///
/// # Side effects
/// None; read-only scan.
///
/// # Safety
/// The table bounds and every object pointer must be valid.
///
/// # Confidence
/// high - the bounds, stride and byte test are explicit.
pub unsafe fn mount_archive_find(tag: u8) -> u32 {
    let mut slot = platform::read_u32(G_MOUNT_ARCHIVE_TABLE) as usize;
    let end = platform::read_u32(G_MOUNT_ARCHIVE_TABLE_END) as usize;
    loop {
        if slot == end {
            return 0;
        }
        let object = platform::read_u32(slot) as usize;
        if platform::read_u8(object + 0x04) == tag {
            return object as u32;
        }
        slot += 4;
    }
}

/// Runs the found manager's acquire virtual (`vtable + 0x0C`).
///
/// The mount executor uses the returned object as the decoder for
/// [`archive::arc_object_load_member`].
///
/// # Original
/// - Ghidra: `FUN_8016b0f0` at `0x8016B0F0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b0f0_FUN_8016b0f0.c`
/// - observed: `piVar1 = FUN_8016b0b0(); if (piVar1 == 0) return 0;
///   return (**(code **)(*piVar1 + 0xc))();`
/// - `// inferred:` the virtual call still has the manager in `r3`, so it is
///   passed as `this`.
///
/// # Arguments
/// - `tag`: tag byte of the manager.
///
/// # Returns
/// The virtual slot's result, or 0 when no manager matches.
///
/// # Side effects
/// Runs the manager's acquire method.
///
/// # Safety
/// A matching manager must implement slot `+0x0C` with this ABI.
///
/// # Confidence
/// medium - the call and return are explicit; only the hidden `this` argument
/// and the slot's meaning are inferred.
pub unsafe fn mount_archive_acquire(tag: u8) -> u32 {
    let object = mount_archive_find(tag) as usize;
    if object == 0 {
        return 0;
    }
    let vtable = platform::read_u32(object) as usize;
    platform::call1(
        platform::read_u32(vtable + 0x0C) as usize,
        object as u32,
    )
}

/// Runs the found manager's release virtual (`vtable + 0x10`).
///
/// # Original
/// - Ghidra: `FUN_8016b130` at `0x8016B130` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b130_FUN_8016b130.c`
/// - observed: `piVar1 = FUN_8016b0b0(); if (piVar1 != 0)
///   (**(code **)(*piVar1 + 0x10))();`
/// - `// inferred:` the manager pointer is still in `r3` for the virtual call.
///
/// # Arguments
/// - `tag`: tag byte of the manager that was acquired.
///
/// # Side effects
/// Runs the manager's release method.
///
/// # Safety
/// Must pair with [`mount_archive_acquire`] on the same tag.
///
/// # Confidence
/// medium - one call; the hidden `this` argument is inferred from the ABI.
pub unsafe fn mount_archive_release(tag: u8) {
    let object = mount_archive_find(tag) as usize;
    if object != 0 {
        let vtable = platform::read_u32(object) as usize;
        platform::call1(
            platform::read_u32(vtable + 0x10) as usize,
            object as u32,
        );
    }
}

/// Returns the mount worker's OS thread.
///
/// # Original
/// - Ghidra: `FUN_8016b170` at `0x8016B170` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b170_FUN_8016b170.c`
/// - observed: `return *(undefined4 *)(r13 + -0x5268);`
///
/// # Returns
/// The `OSThread *` stored by [`mount_stream_manager_init`].
///
/// # Confidence
/// high - one load.
pub unsafe fn mount_thread() -> u32 {
    platform::read_u32(G_MOUNT_THREAD)
}

/// Returns the default heap used by mount commands.
///
/// # Original
/// - Ghidra: `FUN_8016b180` at `0x8016B180` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b180_FUN_8016b180.c`
/// - observed: `return *(undefined4 *)(r13 + -0x525c);`
///
/// # Returns
/// The heap pointer installed from the third argument of
/// [`mount_stream_manager_init`].
///
/// # Confidence
/// high - one load.
pub unsafe fn mount_default_heap() -> u32 {
    platform::read_u32(G_MOUNT_DEFAULT_HEAP)
}

// ===========================================================================
// Mount stream manager and executor: 0x8016B1C0..0x8016B2A0
// ===========================================================================

/// Constructs the threaded mount-stream object.
///
/// The base constructor (`0x802BA3B0`) allocates a 0x4000-byte read buffer and
/// an OS thread, and stores the thread pointer at `+0x08`. This wrapper then
/// installs the mount subclass vtable.
///
/// # Original
/// - Ghidra: `FUN_8016b1c0` at `0x8016B1C0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b1c0_FUN_8016b1c0.c`
/// - observed: `FUN_802ba3b0(param_1,0x4000,0,param_2,0);
///   *param_1 = &DAT_80329728; return param_1;`
///
/// # Arguments
/// - `this`: writable 0x4C-byte stream object.
/// - `priority`: OS thread priority passed to the base constructor.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Allocates the stream buffer and thread, then writes [`MOUNT_STREAM_VTABLE`].
///
/// # Safety
/// `this` must remain alive and must not be resumed before the constructor
/// returns.
///
/// # Confidence
/// high - all five base-constructor arguments and the vtable store are visible.
pub unsafe fn mount_stream_construct(this: usize, priority: u32) -> usize {
    platform::call5(
        MOUNT_STREAM_CTOR,
        this as u32,
        0x4000,
        0,
        priority,
        0,
    );
    platform::write_u32(this, MOUNT_STREAM_VTABLE as u32);
    this
}

/// Creates the global mount stream and starts its worker thread.
///
/// The global heaps are published first, then a 0x4C-byte stream is allocated
/// from `object_heap`, constructed with `priority`, and stored in
/// [`G_MOUNT_STREAM`]. Its thread pointer is copied to [`G_MOUNT_THREAD`]. If a
/// global DVD heap context exists, the stream's heap override is set to it, and
/// the thread is finally resumed.
///
/// # Original
/// - Ghidra: `FUN_8016b220` at `0x8016B220` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b220_FUN_8016b220.c`
/// - observed: `*(r13 - 0x525c) = param_3; *(r13 - 0x5260) = param_2;
///   iVar1 = FUN_802b9240(0x4c,param_4,4); ... iVar2 = FUN_8016b1c0(iVar1,param_1);
///   *(r13 - 0x5264) = iVar2; *(r13 - 0x5268) = *(iVar2 + 8); ... FUN_801b5860(...);`
/// - `// observed:` the only call site, `0x8008F110`, sets `r6 = 0` before the
///   call, so `object_heap` is null and the allocator uses the current heap.
///
/// # Arguments
/// - `priority`: OS thread priority for the stream worker.
/// - `mount_heap`: heap used by mount commands (`G_MOUNT_HEAP`).
/// - `default_heap`: fallback heap (`G_MOUNT_DEFAULT_HEAP`).
/// - `object_heap`: heap for the 0x4C-byte stream, or 0 for the current heap.
///
/// # Side effects
/// Writes four SDA globals, allocates and constructs the stream, may set its
/// heap override, and resumes its thread.
///
/// # Safety
/// The allocation must succeed in the original (there is no null check before
/// `*(stream + 8)`); the three heap arguments must be valid EGG heaps or 0.
///
/// # Confidence
/// high - the disassembly confirms the four incoming registers and the hidden
/// `r6 = 0` at the call site; the stream base's internals are outside this lane.
pub unsafe fn mount_stream_manager_init(
    priority: u32,
    mount_heap: u32,
    default_heap: u32,
    object_heap: u32,
) {
    platform::write_u32(G_MOUNT_DEFAULT_HEAP, default_heap);
    platform::write_u32(G_MOUNT_HEAP, mount_heap);

    let memory = platform::call3(MOUNT_OBJECT_ALLOC, 0x4C, object_heap, 4);
    let stream = if memory != 0 {
        mount_stream_construct(memory as usize, priority) as u32
    } else {
        0
    };
    platform::write_u32(G_MOUNT_STREAM, stream);

    let thread = platform::read_u32(stream as usize + 0x08);
    platform::write_u32(G_MOUNT_THREAD, thread);

    let dvd_heap = platform::read_u32(G_MOUNT_DVD_HEAP);
    if dvd_heap != 0 {
        platform::call2(MOUNT_STREAM_SET_HEAP, stream, dvd_heap);
    }

    platform::call1(OS_RESUME_THREAD, thread);
}

/// Reads one disc entry, optionally through a compressed-stream decoder.
///
/// `mode == 0` is the direct path: [`archive::arc_object_open_entry`] builds a
/// temporary archive object and returns the raw member buffer.
///
/// A nonzero `mode` selects the decoder path. The low byte names the archive
/// manager acquired through [`mount_archive_acquire`]; a single global EGG
/// archive object is constructed lazily (and registered for destructor
/// processing), the entry is opened, and
/// [`archive::arc_object_load_member`] streams it through the decoder in
/// [`G_MOUNT_CHUNK`]-sized pieces. The decoded buffer is flushed from the data
/// cache using the allocation size stored at `buffer - 0x0C`. Both output words
/// receive the size published at `G_GLOBAL_ARCHIVE + 0x70`.
///
/// # Original
/// - Ghidra: `FUN_8016b2a0` at `0x8016B2A0` (344 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b2a0_FUN_8016b2a0.c`
/// - observed: the `param_8 == 0` branch calls `FUN_802b7ed0`; the decoder branch
///   calls `FUN_8016b0f0`, `FUN_802b7b00`, `FUN_802dc780`, `FUN_802b7c30`,
///   `FUN_802b8150`, `FUN_8016b130`, `FUN_802b7d10`, `FUN_801d4a90` and
///   `FUN_801ac500`, then writes `*param_7` and `*param_6`.
/// - `// observed:` the disassembly passes `r9 = 0` as the decoder's
///   `identifier` and takes the chunk from `0x8042946C` (`0x10000`).
///
/// # Arguments
/// - `entry`: DVD entry number from [`mount_probe_path`].
/// - `dst`: destination buffer, or 0 to allocate one.
/// - `heap`: EGG heap/allocator object used for the read.
/// - `direction`: allocation direction (`1` flips the two alignments).
/// - `offset`: byte offset inside the member.
/// - `length_out`: optional output for the produced length.
/// - `size_out`: optional output for the member size.
/// - `mode`: 0 for a direct read, otherwise the decoder tag in the low byte.
///
/// # Returns
/// The buffer address, or 0 when the direct read fails.
///
/// # Side effects
/// Opens/closes a global archive object, calls the decoder and cache flush, and
/// writes the optional outputs.
///
/// # Safety
/// All pointer arguments must satisfy the called archive functions; `mode`'s
/// low byte must name a registered manager for the decoder path.
///
/// # Confidence
/// high - every branch, call and output store is explicit; the global archive
/// object is the same one `archive.rs` documents at `+0x70`.
pub unsafe fn mount_execute(
    entry: u32,
    dst: usize,
    heap: usize,
    direction: u32,
    offset: u32,
    length_out: usize,
    size_out: usize,
    mode: u32,
) -> u32 {
    if mode == 0 {
        return archive::arc_object_open_entry(
            entry,
            dst,
            heap as u32,
            direction,
            offset,
            length_out,
            size_out,
        );
    }

    let tag = (mode & 0xff) as u8;
    let decoder = mount_archive_acquire(tag);

    if platform::read_u8(G_GLOBAL_ARCHIVE_READY) == 0 {
        archive::arc_object_construct(G_GLOBAL_ARCHIVE);
        platform::call3(
            REGISTER_GLOBAL_OBJECT,
            G_GLOBAL_ARCHIVE as u32,
            GLOBAL_ARCHIVE_DTOR as u32,
            G_GLOBAL_ARCHIVE_REG as u32,
        );
        platform::write_u8(G_GLOBAL_ARCHIVE_READY, 1);
    }

    archive::arc_object_open(G_GLOBAL_ARCHIVE, entry);
    let result = archive::arc_object_load_member(
        G_GLOBAL_ARCHIVE,
        decoder as usize,
        dst,
        heap,
        direction,
        offset,
        0,
        platform::read_u32(G_MOUNT_CHUNK),
        0,
        0,
    );

    mount_archive_release(tag);
    let size = platform::read_u32(G_GLOBAL_ARCHIVE_SIZE);
    archive::arc_object_close(G_GLOBAL_ARCHIVE);

    let allocation_size = platform::call1(ALLOC_SIZE_PREFIX, result);
    platform::call2(DC_STORE_RANGE_NO_SYNC, result, allocation_size);

    // observed: the C writes `*param_7` first, then `*param_6`.
    if size_out != 0 {
        platform::write_u32(size_out, size);
    }
    if length_out != 0 {
        platform::write_u32(length_out, size);
    }
    result
}

// ===========================================================================
// Path probe: 0x8016B400, 0x8016BAA0, 0x8016BC10..0x8016BC30
// ===========================================================================

/// Probes a disc path without returning its tag byte.
///
/// # Original
/// - Ghidra: `FUN_8016b400` at `0x8016B400` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b400_FUN_8016b400.c`
/// - observed: `FUN_8016bc30(param_1,0);` (tail call)
///
/// # Arguments
/// - `path`: NUL-terminated disc path.
///
/// # Returns
/// The DVD entry number, or -1 when no path variant resolves.
///
/// # Safety
/// `path` must be a readable C string.
///
/// # Confidence
/// high - one tail call with a null tag pointer.
pub unsafe fn mount_entry_lookup(path: usize) -> i32 {
    mount_probe_path(path, 0)
}

// ---------------------------------------------------------------------------
// Wave 275 lane E - untabled gap body beside the path probe (`_E275`)
// ---------------------------------------------------------------------------

/// Reports whether a disc path resolves to an archive entry.
///
/// The body probes the path through [`mount_entry_lookup`] and folds
/// `entry + 1` with the standard `(x | -x) >> 31` sign test, i.e. it returns
/// `1` exactly when the probe did not report `-1`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8016B410`.
/// - Extent: `0x8016B410..0x8016B440` (48 bytes, 12 instructions), ending at
///   the `blr` at `0x8016B43C`; two zero words precede it at
///   `0x8016B408..0x8016B410` and the next body starts at `0x8016B440`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: no data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `bl 0x8016B400`; `subfic r4,r3,-1`; `addi r0,r3,1`;
///   `or r0,r4,r0` (`// observed:` the printed operand order is the
///   disassembler's; the encoded form is `or r0,r4,r0`);
///   `rlwinm r3,r0,1,31,31` (the sign-fold shift); epilogue; `blr`.
///
/// # Arguments
/// - `path`: NUL-terminated disc path.
///
/// # Returns
/// `1` when the path exists, `0` when [`mount_entry_lookup`] returns `-1`.
///
/// # Side effects
/// Only the callee's path-table reads.
///
/// # Safety
/// `path` must be a readable NUL-terminated string.
///
/// # Confidence
/// high - the probe call, the `entry + 1` fold and the return are literal; the
/// `+ 1` fold is the same predicate already used by `fn_807b29a0` and
/// `fn_807b6d60` in `game::bases`.
pub unsafe fn fn_8016b410(path: usize) -> u32 {
    let entry = mount_entry_lookup(path);
    let plus_one = entry.wrapping_add(1);
    ((plus_one.wrapping_neg() | plus_one) as u32) >> 31
}

/// Resolves a path by appending or replacing each registered extension.
///
/// The path is copied into a 0x100-byte buffer. The first pass appends each
/// manager's extension at the end of the path. If none matches, the buffer is
/// truncated at the last `/` (or kept whole when there is none), the extension
/// at the last `.` is replaced by each registered extension, and the lookup is
/// retried. The tag byte of the matching manager is written to `tag_out`.
///
/// # Original
/// - Ghidra: `FUN_8016baa0` at `0x8016BAA0` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016baa0_FUN_8016baa0.c`
/// - observed: the `strncpy(buffer, path, 0x100)` copy, the buffer-end guard
///   `local_19`, the `strrchr` pair at `0x802E1C40`, the table walk, and the
///   final `*tag_out = tag`.
/// - `// inferred:` the save stub `0x802DCD70` returns the incoming `r3:r4`, so
///   the real signature is `(path, tag_out)`.
///
/// # Arguments
/// - `path`: NUL-terminated disc path.
/// - `tag_out`: optional writable byte for the matched manager's tag.
///
/// # Returns
/// The DVD entry number, or -1 when no variant resolves.
///
/// # Side effects
/// Writes the optional tag byte; uses a 0x100-byte stack buffer.
///
/// # Safety
/// `path` must be readable; `tag_out` must be null or writable.
///
/// # Confidence
/// high - the two passes and both `strrchr` calls are explicit; only the save
/// stub's register pair needs the ABI reading above.
pub unsafe fn mount_probe_path_variants(path: usize, tag_out: usize) -> i32 {
    let mut buffer = crate::platform::GuestLocal::new( [0u8; 0x100]);
    let base = buffer.as_mut_ptr() as usize;
    platform::call3(STRNCPY, base as u32, path as u32, 0x100);

    let mut entry = -1i32;
    let mut tag = 0u8;

    // observed: `local_19` is the last byte of the 256-byte buffer; a non-NUL
    // there means the source did not fit and the lookup is skipped.
    if platform::read_u8(base + 0xFF) == 0 {
        let length = platform::call1(STRLEN, base as u32) as usize;
        let mut cursor = base + length;

        let table = platform::read_u32(G_MOUNT_ARCHIVE_TABLE) as usize;
        let table_end = platform::read_u32(G_MOUNT_ARCHIVE_TABLE_END) as usize;

        let mut slot = table;
        while slot != table_end {
            let object = platform::read_u32(slot) as usize;
            let extension = platform::read_u32(object + 0x08);
            let remaining = 0x100 - (cursor - base);
            platform::call3(
                STRNCPY,
                cursor as u32,
                extension,
                remaining as u32,
            );
            let found = dvd::dvd_convert_path_to_entrynum_801ca680(base as *const u8);
            if found != -1 {
                tag = platform::read_u8(object + 0x04);
                entry = found;
                break;
            }
            slot += 4;
        }

        if entry == -1 {
            platform::write_u8(cursor, 0);

            let mut dot = platform::call2(STR_RCHR, base as u32, b'/' as u32) as usize;
            if dot == 0 {
                dot = base;
            }
            dot = platform::call2(STR_RCHR, dot as u32, b'.' as u32) as usize;

            if dot != 0 {
                let mut slot = table;
                while slot != table_end {
                    let object = platform::read_u32(slot) as usize;
                    let extension = platform::read_u32(object + 0x08);
                    let remaining = 0x100 - (dot - base);
                    platform::call3(
                        STRNCPY,
                        dot as u32,
                        extension,
                        remaining as u32,
                    );
                    let found =
                        dvd::dvd_convert_path_to_entrynum_801ca680(base as *const u8);
                    if found != -1 {
                        tag = platform::read_u8(object + 0x04);
                        entry = found;
                        break;
                    }
                    slot += 4;
                }
                if entry == -1 {
                    platform::write_u8(dot, 0);
                }
            }
        }
    }

    if entry != -1 && tag_out != 0 {
        platform::write_u8(tag_out, tag);
    }
    entry
}

/// Enables or disables the extension-fallback path probe.
///
/// `0x8008F110` enables it once the mount stream is initialised.
///
/// # Original
/// - Ghidra: `FUN_8016bc10` at `0x8016BC10` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bc10_FUN_8016bc10.c`
/// - observed: `*(undefined1 *)(r13 + -0x5258) = param_1;`
///
/// # Arguments
/// - `value`: byte stored in the flag (the shipped caller writes 1).
///
/// # Side effects
/// Writes [`G_MOUNT_PATH_FALLBACK`].
///
/// # Confidence
/// high - one store.
pub unsafe fn mount_set_path_fallback(value: u8) {
    platform::write_u8(G_MOUNT_PATH_FALLBACK, value);
}

/// Returns the extension-fallback flag.
///
/// # Original
/// - Ghidra: `FUN_8016bc20` at `0x8016BC20` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bc20_FUN_8016bc20.c`
/// - observed: `return *(undefined1 *)(r13 + -0x5258);`
///
/// # Returns
/// The flag byte written by [`mount_set_path_fallback`].
///
/// # Confidence
/// high - one load.
pub unsafe fn mount_path_fallback() -> u32 {
    platform::read_u8(G_MOUNT_PATH_FALLBACK) as u32
}

/// Resolves a disc path to a DVD entry number.
///
/// The direct lookup runs first. On a miss, and only when
/// [`mount_path_fallback`] is nonzero, the extension/variant pass
/// [`mount_probe_path_variants`] runs. On a direct hit with a non-null tag
/// output, the tag is cleared to 0.
///
/// # Original
/// - Ghidra: `FUN_8016bc30` at `0x8016BC30` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bc30_FUN_8016bc30.c`
/// - observed: `iVar1 = FUN_801ca680(); if (iVar1 == -1) { iVar2 =
///   FUN_8016bc20(); if (iVar2 != 0) iVar1 = FUN_8016baa0(param_1,param_2); }
///   else if (param_2 != 0) *param_2 = 0; return iVar1;`
///
/// # Arguments
/// - `path`: NUL-terminated disc path.
/// - `tag_out`: optional writable byte for the fallback tag.
///
/// # Returns
/// The DVD entry number, or -1 when neither lookup resolves.
///
/// # Side effects
/// May write `tag_out` (0 on a direct hit, the manager tag on a fallback hit).
///
/// # Safety
/// `path` must be a readable C string; `tag_out` must be null or writable.
///
/// # Confidence
/// high - both branches and the tag clear are explicit; the direct lookup is
/// the translated `crate::runtime::dvd` function.
pub unsafe fn mount_probe_path(path: usize, tag_out: usize) -> i32 {
    let mut entry = dvd::dvd_convert_path_to_entrynum_801ca680(path as *const u8);
    if entry == -1 {
        if mount_path_fallback() != 0 {
            entry = mount_probe_path_variants(path, tag_out);
        }
    } else if tag_out != 0 {
        platform::write_u8(tag_out, 0);
    }
    entry
}

// ===========================================================================
// "Call one function" mount command: 0x8016B800..0x8016B8D0
// ===========================================================================

/// Deleting destructor for the "call one function" command.
///
/// The base destructor runs with the non-deleting flag `0`, then the object is
/// returned to the mount heap when `deleting` is positive.
///
/// # Original
/// - Ghidra: `FUN_8016b800` at `0x8016B800` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b800_FUN_8016b800.c`
/// - observed: `if ((param_1 != 0) && (FUN_8016b440(param_1,0), 0 < param_2))
///   { FUN_8016b4b0(param_1); } return param_1;`
///
/// # Arguments
/// - `this`: command object, or 0.
/// - `deleting`: `> 0` frees the object after the base destructor.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Waits for the command, then may free it.
///
/// # Safety
/// A non-null `this` must be a live command.
///
/// # Confidence
/// high - the null guard, wait, guard and free are explicit.
pub unsafe fn mount_call_cmd_destroy(this: usize, deleting: i32) -> usize {
    if this != 0 {
        mount_cmd_destroy_wait(this, 0);
        if deleting > 0 {
            mount_cmd_free(this);
        }
    }
    this
}

/// Constructs a command that calls one function with one argument.
///
/// The queued execute body (`0x8016B960`, not exported) loads `+0x0C` as a
/// function pointer, passes `+0x10`, stores the result at `+0x14`, and marks
/// the command done.
///
/// # Original
/// - Ghidra: `FUN_8016b860` at `0x8016B860` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b860_FUN_8016b860.c`
/// - observed: `FUN_8016b7c0(); *param_1 = &DAT_80329768; param_1[3] = param_2;
///   param_1[4] = param_3; param_1[5] = 0;`
///
/// # Arguments
/// - `this`: writable 0x18-byte command object.
/// - `callback`: function pointer invoked by the queue worker.
/// - `argument`: single argument passed to `callback`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes the base header, the subclass vtable and the three fields.
///
/// # Safety
/// `this` must be writable for 0x18 bytes; `callback` must accept one `u32`.
///
/// # Confidence
/// high - the constructor, vtable and three field stores are explicit.
pub unsafe fn mount_call_cmd_construct(
    this: usize,
    callback: u32,
    argument: u32,
) -> usize {
    mount_cmd_base_construct(this);
    platform::write_u32(this, MOUNT_CALL_CMD_VTABLE as u32);
    platform::write_u32(this + 0x0C, callback);
    platform::write_u32(this + 0x10, argument);
    platform::write_u32(this + 0x14, 0);
    this
}

/// Allocates and queues a "call one function" command.
///
/// # Original
/// - Ghidra: `FUN_8016b8d0` at `0x8016B8D0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b8d0_FUN_8016b8d0.c`
/// - observed: `iVar1 = FUN_8016b4a0(0x18); iVar2 = 0; if (iVar1 != 0)
///   iVar2 = FUN_8016b860(iVar1,param_1,param_2); if (iVar2 != 0)
///   FUN_8016b5c0(*(r13 + -0x5254),iVar2); return iVar2;`
///
/// # Arguments
/// - `callback`: function pointer invoked by the queue worker.
/// - `argument`: single argument passed to `callback`.
///
/// # Returns
/// The queued command, or 0 when allocation fails.
///
/// # Side effects
/// Allocates from the mount heap and submits the command to the global queue.
///
/// # Safety
/// The mount queue and heap must be initialised; `callback` must be callable
/// with one `u32`.
///
/// # Confidence
/// high - allocation size, construction and submit are explicit.
pub unsafe fn mount_call_cmd_create(callback: u32, argument: u32) -> u32 {
    let mut command = 0u32;
    let memory = mount_cmd_alloc(0x18);
    if memory != 0 {
        command = mount_call_cmd_construct(memory as usize, callback, argument) as u32;
    }
    if command != 0 {
        let queue = platform::read_u32(G_MOUNT_QUEUE) as usize;
        mount_queue_submit(queue, command as usize);
    }
    command
}

// ===========================================================================
// Mount-command constructors and create entry points: 0x8016B7C0..0x8016BF70
// ===========================================================================

/// Constructs the base mount command.
///
/// Writes [`MOUNT_CMD_BASE_VTABLE`], clears the next pointer at `+0x04` and the
/// done byte at `+0x08`.
///
/// # Original
/// - Ghidra: `FUN_8016b7c0` at `0x8016B7C0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b7c0_FUN_8016b7c0.c`
/// - observed: `*param_1 = &DAT_8032977c; param_1[1] = 0;
///   *(undefined1 *)(param_1 + 2) = 0;`
///
/// # Arguments
/// - `this`: writable command object (at least `0x0C` bytes).
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be writable for the command header.
///
/// # Confidence
/// high - three literal stores.
pub unsafe fn mount_cmd_base_construct(this: usize) -> usize {
    platform::write_u32(this, MOUNT_CMD_BASE_VTABLE as u32);
    platform::write_u32(this + 0x04, 0);
    platform::write_u8(this + 0x08, 0);
    this
}

/// Virtual deleting destructor for a queued command.
///
/// Calls the object's vtable slot `+0x08` with the deleting flag `1`. The
/// target is the object's own deleting destructor; for the base command that is
/// [`mount_cmd_destroy_wait`].
///
/// # Original
/// - Ghidra: `FUN_8016b7e0` at `0x8016B7E0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b7e0_FUN_8016b7e0.c`
/// - observed: `if (param_1 == (int *)0x0) return; (**(code **)(*param_1 + 8))(param_1,1);`
/// - `// inferred:` the slot's arguments are explicit in the C, unlike the
///   destructor calls elsewhere in this cluster.
///
/// # Arguments
/// - `this`: command object, or 0.
///
/// # Side effects
/// Runs the virtual destructor, which may free the object.
///
/// # Safety
/// A non-null `this` must be a live command object.
///
/// # Confidence
/// high - null guard, slot offset and deleting flag are explicit.
pub unsafe fn mount_cmd_destroying_delete(this: usize) {
    if this == 0 {
        return;
    }
    let vtable = platform::read_u32(this) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x08) as usize,
        this as u32,
        1,
    );
}

/// Constructs a "mount into memory" command.
///
/// The base constructor runs first, then [`MOUNT_MEM_VTABLE`] is installed.
/// `flags == 0` selects the global default mode byte. The remaining fields are
/// cleared, and the entry number at `+0x0C` starts at `-1` ("none").
///
/// # Original
/// - Ghidra: `FUN_8016ba20` at `0x8016BA20` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ba20_FUN_8016ba20.c`
/// - observed: `FUN_8016b7c0(); *param_1 = &DAT_80329754; if (param_2 == 0)
///   param_2 = *(byte *)(r13 + -0x6218); ... param_1[3] = 0xffffffff;`
/// - `// inferred:` `FUN_8016b7c0` prints no argument but receives `this` in
///   `r3`.
///
/// # Arguments
/// - `this`: writable 0x1C-byte command object.
/// - `flags`: mode byte, or 0 for the default.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be writable for the full command and must not be freed while
/// queued.
///
/// # Confidence
/// high - every store is explicit; only the hidden base-constructor argument is
/// inferred.
pub unsafe fn mount_mem_cmd_construct(this: usize, flags: u32) -> usize {
    mount_cmd_base_construct(this);
    platform::write_u32(this, MOUNT_MEM_VTABLE as u32);
    let mode = if flags == 0 {
        platform::read_u8(G_MOUNT_DEFAULT_MODE) as u32
    } else {
        flags
    };
    platform::write_u8(this + 0x09, mode as u8);
    platform::write_u8(this + 0x0A, 0);
    platform::write_u32(this + 0x0C, 0xFFFF_FFFF);
    platform::write_u32(this + 0x10, 0);
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this + 0x18, 0);
    this
}

/// Creates and queues a "mount into memory" command for `path`.
///
/// This is exactly the factory [`res_info_set`] calls by address. It resolves
/// the path to a disc entry, allocates a 0x1C-byte command, records the entry,
/// heap and tag, and submits it to the mount queue.
///
/// # Original
/// - Ghidra: `FUN_8016bcc0` at `0x8016BCC0` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bcc0_FUN_8016bcc0.c`
/// - observed: `iVar1 = thunk_FUN_8016bc30(param_1,local_18);
///   if (iVar1 != -1) { iVar2 = FUN_8016b4a0(0x1c); ... FUN_8016ba20(iVar2,param_2);
///   ... +0xc = iVar1; +0x14 = param_3; +10 = local_18[0];
///   FUN_8016b5c0(*(r13 + -0x5254),iVar3); }`
///
/// # Arguments
/// - `path`: NUL-terminated disc path.
/// - `flags`: command mode byte (0 selects the default).
/// - `heap`: heap used for the eventual buffer, or 0 for the default.
///
/// # Returns
/// The queued command, or 0 when the path is unknown or allocation fails.
///
/// # Side effects
/// Starts an asynchronous mount and may enqueue work.
///
/// # Safety
/// `path` must be a readable C string; the queue must be initialised.
///
/// # Confidence
/// high - the probe, allocation, field stores and submit are explicit.
pub unsafe fn mount_mem_cmd_create(path: usize, flags: u32, heap: u32) -> u32 {
    let mut tag = crate::platform::GuestLocal::new( [0u8; 8]);
    let entry = platform::call2(
        MOUNT_PROBE,
        path as u32,
        tag.as_mut_ptr() as u32,
    );
    if entry == 0xFFFF_FFFF {
        return 0;
    }

    let mut command = 0u32;
    let memory = mount_cmd_alloc(0x1C);
    if memory != 0 {
        command = mount_mem_cmd_construct(memory as usize, flags) as u32;
    }
    if command != 0 {
        platform::write_u32(command as usize + 0x0C, entry);
        platform::write_u32(command as usize + 0x14, heap);
        platform::write_u8(command as usize + 0x0A, tag[0]);
        let queue = platform::read_u32(G_MOUNT_QUEUE) as usize;
        mount_queue_submit(queue, command as usize);
    }
    command
}

/// Executes a queued "mount into memory" command.
///
/// The worker resolves the recorded entry through `0x8016B2A0`, mounts the
/// resulting bytes as a U8 archive through [`bres::fn_802b75b0`], stores the
/// archive object at `+0x10`, frees the temporary buffer on failure, and marks
/// the command done.
///
/// # Original
/// - Ghidra: `FUN_8016bd60` at `0x8016BD60` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bd60_FUN_8016bd60.c`
/// - observed: the `+0x14` heap fallback, the `0x8016B2A0` eight-argument call,
///   `FUN_802b75b0(iVar1,iVar4,uVar2)`, the failure free at `0x802B9280`, and
///   the final `FUN_8016b4d0(param_1)`.
///
/// # Arguments
/// - `this`: the live command object.
///
/// # Returns
/// `1` when `+0x10` holds a mounted archive, `0` otherwise.
///
/// # Side effects
/// Reads the disc, mounts the archive, may free the temporary buffer, and wakes
/// the done queue.
///
/// # Safety
/// The command must have been created by [`mount_mem_cmd_create`]; the registry
/// then owns the returned archive via `info_c + 0x28`.
///
/// # Confidence
/// medium - the call order and every store are explicit, but the eight
/// arguments of `0x8016B2A0` are reconstructed from its own body.
pub unsafe fn mount_mem_cmd_execute(this: usize) -> u32 {
    let stored_heap = platform::read_u32(this + 0x14);
    let heap = if stored_heap != 0 {
        stored_heap
    } else {
        platform::read_u32(G_MOUNT_DEFAULT_HEAP)
    };

    let direction = if platform::read_u8(this + 0x09) == 1 {
        1
    } else {
        2
    };
    let loaded = call8_u32(
        MOUNT_EXEC,
        platform::read_u32(this + 0x0C),
        0,
        heap,
        direction,
        0,
        (this + 0x18) as u32,
        0,
        platform::read_u8(this + 0x0A) as u32,
    ) as i32;

    let mut archive = 0u32;
    if loaded != 0 {
        let flags = if platform::read_u8(this + 0x09) == 1 {
            4
        } else {
            0xFFFF_FFFC
        };
        archive = bres::fn_802b75b0(loaded as usize, heap, flags);
    }

    if archive == 0 {
        if loaded != 0 {
            platform::call1(HEAP_DELETE, loaded as u32);
        }
        platform::write_u32(this + 0x18, 0);
    } else {
        platform::write_u32(this + 0x10, archive);
    }

    mount_cmd_mark_done(this);
    (platform::read_u32(this + 0x10) != 0) as u32
}

/// Returns the archive mounted by a finished "mount into memory" command.
///
/// The mounted archive object is at `+0x10`; its raw U8 handle is the word at
/// `archive + 0x18`. [`res_info_cleanup`] uses this through the address
/// `0x8016BE60`.
///
/// # Original
/// - Ghidra: `FUN_8016be60` at `0x8016BE60` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016be60_FUN_8016be60.c`
/// - observed: `if (*(int *)(param_1 + 0x10) == 0) return 0;
///   return *(undefined4 *)(*(int *)(param_1 + 0x10) + 0x18);`
///
/// # Arguments
/// - `this`: a command that has completed.
///
/// # Returns
/// The raw archive pointer, or 0 when nothing was mounted.
///
/// # Safety
/// `this` must be a completed command; a non-null `+0x10` must be a live
/// archive object.
///
/// # Confidence
/// high - both loads are explicit.
pub unsafe fn mount_mem_cmd_archive(this: usize) -> u32 {
    let mounted = platform::read_u32(this + 0x10) as usize;
    if mounted == 0 {
        return 0;
    }
    platform::read_u32(mounted + 0x18)
}

/// Constructs a "mount as archive" command.
///
/// Identical to [`mount_mem_cmd_construct`] except for [`MOUNT_ARC_VTABLE`] and
/// the extra cleared word at `+0x1C` (the archive path/mode word used by
/// [`mount_arc_cmd_create`]).
///
/// # Original
/// - Ghidra: `FUN_8016bef0` at `0x8016BEF0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bef0_FUN_8016bef0.c`
/// - observed: `FUN_8016b7c0(); *param_1 = &DAT_80329740; ... param_1[7] = 0;`
///
/// # Arguments
/// - `this`: writable 0x20-byte command object.
/// - `flags`: mode byte, or 0 for the default.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be writable for the full command.
///
/// # Confidence
/// high - every store is explicit; only the hidden base-constructor argument is
/// inferred.
pub unsafe fn mount_arc_cmd_construct(this: usize, flags: u32) -> usize {
    mount_cmd_base_construct(this);
    platform::write_u32(this, MOUNT_ARC_VTABLE as u32);
    let mode = if flags == 0 {
        platform::read_u8(G_MOUNT_DEFAULT_MODE) as u32
    } else {
        flags
    };
    platform::write_u8(this + 0x09, mode as u8);
    platform::write_u8(this + 0x0A, 0);
    platform::write_u32(this + 0x0C, 0xFFFF_FFFF);
    platform::write_u32(this + 0x10, 0);
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this + 0x18, 0);
    platform::write_u32(this + 0x1C, 0);
    this
}

/// Creates and queues a "mount as archive" command for `path`.
///
/// The companion of [`mount_mem_cmd_create`], but it allocates 0x20 bytes and
/// stores the extra word at `+0x1C`. The shipped `dRes_c` registry uses the
/// memory variant; this one is kept because the same queue dispatches it and it
/// shares the mount object lifecycle.
///
/// # Original
/// - Ghidra: `FUN_8016bf70` at `0x8016BF70` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bf70_FUN_8016bf70.c`
/// - observed: `iVar1 = thunk_FUN_8016bc30(param_1,local_18);
///   if (iVar1 == -1) iVar3 = 0; else { iVar2 = FUN_8016b4a0(0x20); ...
///   FUN_8016bef0(iVar2,param_2); ... +0xc = iVar1; +0x1c = param_3;
///   +10 = local_18[0]; FUN_8016b5c0(*(r13 + -0x5254),iVar3); }`
///
/// # Arguments
/// - `path`: NUL-terminated disc path.
/// - `flags`: command mode byte (0 selects the default).
/// - `extra`: value stored at `+0x1C` (the archive-mode word).
///
/// # Returns
/// The queued command, or 0 on failure.
///
/// # Side effects
/// Starts an asynchronous mount and may enqueue work.
///
/// # Safety
/// `path` must be a readable C string; the queue must be initialised.
///
/// # Confidence
/// high - the probe, allocation, field stores and submit are explicit.
pub unsafe fn mount_arc_cmd_create(path: usize, flags: u32, extra: u32) -> u32 {
    let mut tag = crate::platform::GuestLocal::new( [0u8; 8]);
    let entry = platform::call2(
        MOUNT_PROBE,
        path as u32,
        tag.as_mut_ptr() as u32,
    );
    if entry == 0xFFFF_FFFF {
        return 0;
    }

    let mut command = 0u32;
    let memory = mount_cmd_alloc(0x20);
    if memory != 0 {
        command = mount_arc_cmd_construct(memory as usize, flags) as u32;
    }
    if command != 0 {
        platform::write_u32(command as usize + 0x0C, entry);
        platform::write_u32(command as usize + 0x1C, extra);
        platform::write_u8(command as usize + 0x0A, tag[0]);
        let queue = platform::read_u32(G_MOUNT_QUEUE) as usize;
        mount_queue_submit(queue, command as usize);
    }
    command
}

// ===========================================================================
// Mount stream base class: 0x802BA2B0..0x802BA800
//
// `mount_stream_construct` (0x8016B1C0) and `mount_stream_manager_init`
// (0x8016B220) build a threaded read stream through the routines below. They are
// the EGG `Stream` base: the object owns a message queue, a stack buffer, an
// `OSThread` and an optional archive-heap override. Wave 24 documented the
// manager and executor that call them but left these bodies opaque; wave 30
// supplies the class itself.
//
// Object layout (`// inferred:` from the stores in `0x802BA3B0`/`0x802BA800`):
//
// | offset | meaning |
// |---|---|
// | `+0x00` | vtable (`0x8034FE20` base, `0x8034FE30` stream) |
// | `+0x04` | archive heap that owns the stream |
// | `+0x08` | `OSThread *` created by `OSCreateThread` |
// | `+0x0C` | `OSMessageQueue` (0x20 bytes) |
// | `+0x2C` | message ring buffer (`count << 2`) |
// | `+0x30` | ring capacity in messages |
// | `+0x34` | stack buffer address |
// | `+0x38` | stack size, 32-byte aligned |
// | `+0x40` | heap override restored by the switch callback |
//
// Start here: [`mount_stream_base_init`] is the constructor that
// `mount_stream_construct` calls; [`mount_stream_apply_heap`] is the function
// `mount_stream_manager_init` calls for the DVD heap.
// ===========================================================================

/// `0x8034FE20`: vtable of the stream base (no thread, no queue).
const EGG_STREAM_BASE_VTABLE: usize = 0x8034_FE20;
/// `0x8034FE30`: vtable of the threaded stream/borrowed-buffer stream.
const EGG_STREAM_VTABLE: usize = 0x8034_FE30;
/// `0x802BA860`: entry point passed to `OSCreateThread` by
/// [`mount_stream_base_init`].
const EGG_STREAM_THREAD_ENTRY: usize = 0x802B_A860;

/// `0x804238A8`: NW4R list of live streams; stride 0x44 from `0x80228DD0`.
const G_MOUNT_STREAM_LIST: usize = 0x8042_38A8;
/// `r13 - 0x4840`: previous thread-switch callback saved by
/// [`mount_stream_list_init`].
const G_MOUNT_SWITCH_CALLBACK: usize = SDA_BASE - 0x4840;
/// `r13 - 0x4870` -> `0x8042A2C0`: archive heap used by the current thread
/// (same SDA word as `G_ARC_HEAP_CURRENT` in [`crate::resource::archive`]).
const G_ARC_HEAP_CURRENT: usize = SDA_BASE - 0x4870;
/// Stream field: cached archive-heap override.
const STREAM_HEAP_OVERRIDE: usize = 0x40;
/// Stream field: `OSThread *`.
const STREAM_THREAD: usize = 0x08;
/// Stream field: stack buffer.
const STREAM_STACK: usize = 0x34;
/// Stream field: stack size.
const STREAM_STACK_SIZE: usize = 0x38;
/// Stream field: message ring.
const STREAM_RING: usize = 0x2C;
/// Stream field: message-queue sub-object.
const STREAM_QUEUE: usize = 0x0C;
/// Stream field: ring capacity.
const STREAM_RING_COUNT: usize = 0x30;
/// Size of the `OSThread` allocated by [`mount_stream_base_init`].
const OS_THREAD_SIZE: usize = 0x318;

/// `0x801B5130` - `OSCreateThread(thread, entry, arg, stackEnd, stackSize, prio, detached)`.
const FN_OS_CREATE_THREAD: usize = 0x801B_5130;
/// `0x801B4B40` - `// inferred:` `OSIsThreadTerminated(thread)`; the body reads
/// the thread state at `+0x2C8` and returns 1 for the finished states.
const FN_OS_THREAD_TERMINATED: usize = 0x801B_4B40;
/// `0x801B4B70` / `0x801B4BB0` - scheduler lock/unlock.
const FN_OS_SCHED_LOCK: usize = 0x801B_4B70;
const FN_OS_SCHED_UNLOCK: usize = 0x801B_4BB0;
/// `0x801B4820` - `// inferred:` `OSSetThreadSwitchCallback(callback)`; the body
/// swaps the word at `r13 - 0x6150`, substitutes the default `0x801B4810`, and
/// returns the previous callback (or 0 for the default).
const FN_OS_SET_SWITCH_CALLBACK: usize = 0x801B_4820;
/// `0x801B57C0` / `0x801B5490` - `OSDetachThread` / `OSCancelThread`.
const FN_OS_DETACH_THREAD: usize = 0x801B_57C0;
const FN_OS_CANCEL_THREAD: usize = 0x801B_5490;
/// `0x80228DD0` - NW4R list constructor (`list, stride`).
const FN_UT_LIST_INIT: usize = 0x8022_8DD0;
/// `0x80228DF0` - NW4R list append (`list, node`).
const FN_UT_LIST_APPEND: usize = 0x8022_8DF0;
/// `0x80228F80` - NW4R list remove (`list, node`).
const FN_UT_LIST_REMOVE: usize = 0x8022_8F80;
/// `0x80228FF0` - NW4R list iterator (`list, previous -> node`).
const FN_UT_LIST_NEXT: usize = 0x8022_8FF0;

/// Returns the alignment used by the stream base when it allocates.
///
/// # Original
/// - Ghidra: `FUN_802ba2b0` at `0x802BA2B0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba2b0_FUN_802ba2b0.c`
/// - observed: `return 0x80;`
///
/// # Returns
/// `0x80` bytes.
///
/// # Confidence
/// high - literal constant return.
pub unsafe fn mount_stream_default_align() -> u32 {
    0x80
}

/// Constructs the stream base that owns no thread or queue.
///
/// Writes the base vtable `0x8034FE20`, resolves the calling thread's archive
/// heap through [`crate::resource::archive::arc_heap_current`], stores it at
/// `+0x04`, and registers `this` on that heap's client list (`heap + 0x28`).
///
/// # Original
/// - Ghidra: `FUN_802ba2d0` at `0x802BA2D0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba2d0_FUN_802ba2d0.c`
/// - observed: `*param_1 = &DAT_8034fe20; iVar1 = FUN_802b8f30();
///   param_1[1] = iVar1; if (iVar1 != 0) FUN_80228df0(iVar1 + 0x28,param_1);`
///
/// # Arguments
/// - `this`: stream object (at least `0x44` bytes).
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Installs the vtable, stores the owning heap and appends to its client list.
///
/// # Safety
/// `this` must be writable and the current archive heap must be initialised
/// before the object is destroyed.
///
/// # Confidence
/// high - every store and the list append are explicit.
pub unsafe fn mount_stream_base_construct(this: usize) -> usize {
    platform::write_u32(this, EGG_STREAM_BASE_VTABLE as u32);
    let heap = archive::arc_heap_current();
    platform::write_u32(this + 0x04, heap);
    if heap != 0 {
        platform::call2(FN_UT_LIST_APPEND, heap + 0x28, this as u32);
    }
    this
}

/// Destroys the stream base, optionally freeing the object.
///
/// Clears the vtable, unlinks `this` from its owning heap's client list, and
/// frees the object through [`crate::resource::archive::arc_heap_free_object`]
/// when `deleting` is positive. A null object is accepted.
///
/// # Original
/// - Ghidra: `FUN_802ba330` at `0x802BA330` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba330_FUN_802ba330.c`
/// - observed: `*param_1 = &DAT_8034fe20; if (param_1[1] != 0)
///   FUN_80228f80(param_1[1] + 0x28,param_1); if (0 < param_2) FUN_802b9280(param_1);`
///
/// # Arguments
/// - `this`: stream address, or 0.
/// - `deleting`: `> 0` to free the object.
///
/// # Returns
/// `this` unchanged.
///
/// # Safety
/// A non-null `this` must be a constructed base stream whose heap is still live.
///
/// # Confidence
/// high - explicit guard, unlink and free.
pub unsafe fn mount_stream_base_destroy(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, EGG_STREAM_BASE_VTABLE as u32);
        let heap = platform::read_u32(this + 0x04) as usize;
        if heap != 0 {
            platform::call2(FN_UT_LIST_REMOVE, heap as u32 + 0x28, this as u32);
        }
        if deleting > 0 {
            archive::arc_heap_free_object(this as u32);
        }
    }
    this
}

/// Initialises the threaded stream base.
///
/// Writes the stream vtable `0x8034FE30`, picks `default_heap` (or the current
/// archive heap when 0), 32-byte-aligns the requested stack size, allocates the
/// stack buffer and a fresh `OSThread` from that heap, creates the thread with
/// `0x802BA860` as entry point and `this` as its argument, clears the two
/// bookkeeping words, and publishes the stream through
/// [`mount_stream_init_queue`].
///
/// # Original
/// - Ghidra: `FUN_802ba3b0` at `0x802BA3B0` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba3b0_FUN_802ba3b0.c`
/// - observed: `*param_1 = &DAT_8034fe30; if (param_5 == 0)
///   param_5 = *(r13 - 0x4870); param_1[1] = param_5;
///   param_1[0xe] = param_2 & 0xffffffe0; param_1[0xd] = FUN_802b8cc0(size,0x20,heap);
///   param_1[2] = FUN_802b8cc0(0x318,0x20,heap);
///   FUN_801b5130(thread,&LAB_802ba860,param_1,stack+size,size,param_4,1);
///   FUN_802ba800(param_1,param_3);`
///
/// # Arguments
/// - `this`: stream object (at least `0x44` bytes).
/// - `stack_size`: requested stack size; rounded down to 32 bytes.
/// - `ring_count`: number of message slots in the stream queue.
/// - `priority`: `OSCreateThread` priority.
/// - `default_heap`: archive heap to charge, or 0 for the current heap.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Allocates two blocks, creates an OS thread and initialises the queue.
///
/// # Safety
/// `this` must be writable and unreferenced; the heap must be able to serve
/// both allocations, and `stack_size` must leave room for the OS frame.
///
/// # Confidence
/// high - allocation sizes, thread arguments and call order are explicit;
/// `param_1[1]`'s hidden third argument to `0x802BA800` is ignored by its body.
pub unsafe fn mount_stream_base_init(
    this: usize,
    stack_size: u32,
    ring_count: u32,
    priority: u32,
    default_heap: usize,
) -> usize {
    platform::write_u32(this, EGG_STREAM_VTABLE as u32);
    let heap = if default_heap != 0 {
        default_heap
    } else {
        platform::read_u32(G_ARC_HEAP_CURRENT) as usize
    };
    platform::write_u32(this + 0x04, heap as u32);
    let aligned_stack = stack_size & 0xFFFF_FFE0;
    platform::write_u32(this + STREAM_STACK_SIZE, aligned_stack);
    let stack = archive::arc_heap_allocate(aligned_stack, 0x20, heap);
    platform::write_u32(this + STREAM_STACK, stack);
    let thread = archive::arc_heap_allocate(OS_THREAD_SIZE as u32, 0x20, heap);
    platform::write_u32(this + STREAM_THREAD, thread);
    platform::call7(
        FN_OS_CREATE_THREAD,
        thread,
        EGG_STREAM_THREAD_ENTRY as u32,
        this as u32,
        stack + aligned_stack,
        aligned_stack,
        priority,
        1,
    );
    platform::write_u32(this + 0x3C, 0);
    platform::write_u32(this + 0x40, 0);
    mount_stream_init_queue(this, ring_count);
    this
}

/// Initialises a stream over an already-created thread.
///
/// Used when the caller owns the OS thread and stack (for example a DVD worker
/// created elsewhere). Instead of allocating, it copies the thread's stack
/// window (`thread + 0x304` top, `thread + 0x308` current position) into the
/// stream's `+0x38`/`+0x34` fields and then publishes the stream.
///
/// # Original
/// - Ghidra: `FUN_802ba480` at `0x802BA480` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba480_FUN_802ba480.c`
/// - observed: `*param_1 = &DAT_8034fe30; param_1[1] = 0; param_1[2] = param_2;
///   param_1[0xe] = *(param_2 + 0x304) - *(param_2 + 0x308);
///   param_1[0xd] = *(param_2 + 0x308); FUN_802ba800(param_1,param_3);`
///
/// # Arguments
/// - `this`: stream object to initialise.
/// - `thread`: live `OSThread` whose stack the stream borrows.
/// - `ring_count`: message-ring capacity.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `thread` must outlive the stream; the stream must not outlive the thread's
/// stack.
///
/// # Confidence
/// high - field arithmetic and init call are explicit.
pub unsafe fn mount_stream_borrow(this: usize, thread: usize, ring_count: u32) -> usize {
    platform::write_u32(this, EGG_STREAM_VTABLE as u32);
    platform::write_u32(this + 0x04, 0);
    platform::write_u32(this + STREAM_THREAD, thread as u32);
    let stack_size = platform::read_u32(thread + 0x304)
        .wrapping_sub(platform::read_u32(thread + 0x308));
    platform::write_u32(this + STREAM_STACK_SIZE, stack_size);
    platform::write_u32(this + STREAM_STACK, platform::read_u32(thread + 0x308));
    platform::write_u32(this + 0x3C, 0);
    platform::write_u32(this + 0x40, 0);
    mount_stream_init_queue(this, ring_count);
    this
}

/// Destroys a threaded stream and frees its allocations.
///
/// Clears the vtable, unlinks the stream from [`G_MOUNT_STREAM_LIST`], and when
/// the stream owns a heap (`+0x04` non-zero) it terminates the OS thread (detach
/// plus cancel when the thread is not already terminated), releases the stack
/// buffer and the `OSThread` to that heap, releases the `+0x2C` object to the
/// current heap, and finally frees the stream itself when `deleting` is
/// positive.
///
/// # Original
/// - Ghidra: `FUN_802ba500` at `0x802BA500` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba500_FUN_802ba500.c`
/// - observed: `FUN_80228f80(&DAT_804238a8,param_1); if (param_1[1] != 0) {
///   if (FUN_801b4b40(param_1[2]) == 0) { FUN_801b57c0(param_1[2]);
///   FUN_801b5490(param_1[2]); } FUN_802b8f70(param_1[0xd],param_1[1]);
///   FUN_802b8f70(param_1[2],param_1[1]); }
///   FUN_802b8f70(param_1[0xb],0); if (0 < param_2) FUN_802b9280(param_1);`
///
/// # Arguments
/// - `this`: stream address, or 0.
/// - `deleting`: `> 0` to free the stream object.
///
/// # Returns
/// `this` unchanged.
///
/// # Safety
/// The stream must not be running user work; `thread` must be a valid thread
/// object if the stream owns a heap.
///
/// # Confidence
/// high - branch structure and every free are explicit.
pub unsafe fn mount_stream_destroy(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, EGG_STREAM_VTABLE as u32);
        platform::call2(FN_UT_LIST_REMOVE, G_MOUNT_STREAM_LIST as u32, this as u32);
        let heap = platform::read_u32(this + 0x04) as usize;
        if heap != 0 {
            let thread = platform::read_u32(this + STREAM_THREAD) as usize;
            if platform::call1(FN_OS_THREAD_TERMINATED, thread as u32) == 0 {
                platform::call1(FN_OS_DETACH_THREAD, thread as u32);
                platform::call1(FN_OS_CANCEL_THREAD, thread as u32);
            }
            archive::arc_heap_free(platform::read_u32(this + STREAM_STACK), heap);
            archive::arc_heap_free(thread as u32, heap);
        }
        archive::arc_heap_free(platform::read_u32(this + STREAM_RING), 0);
        if deleting > 0 {
            archive::arc_heap_free_object(this as u32);
        }
    }
    this
}

/// Finds the live stream whose owning thread matches `thread`.
///
/// Walks [`G_MOUNT_STREAM_LIST`] with the NW4R iterator and compares each node's
/// `+0x08` (`OSThread *`) with `thread`.
///
/// # Original
/// - Ghidra: `FUN_802ba5c0` at `0x802BA5C0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba5c0_FUN_802ba5c0.c`
/// - observed: `do { n = FUN_80228FF0(&DAT_804238a8,n); if (n == 0)
///   return 0; } while (*(int *)(n + 8) != param_1); return n;`
///
/// # Arguments
/// - `thread`: thread pointer to look up.
///
/// # Returns
/// The stream node, or 0.
///
/// # Confidence
/// high - explicit list walk.
pub unsafe fn mount_stream_find(thread: u32) -> u32 {
    let mut node = 0u32;
    loop {
        node = platform::call2(FN_UT_LIST_NEXT, G_MOUNT_STREAM_LIST as u32, node);
        if node == 0 {
            return 0;
        }
        if platform::read_u32(node as usize + STREAM_THREAD) == thread {
            return node;
        }
    }
}

/// Initialises the global stream list and installs the switch callback.
///
/// Builds [`G_MOUNT_STREAM_LIST`] with a 0x44-byte stride, then replaces the OS
/// thread-switch callback with `0x802BA710` and saves the previous callback in
/// [`G_MOUNT_SWITCH_CALLBACK`]. The callback swaps archive-heap overrides when
/// control moves between streams.
///
/// # Original
/// - Ghidra: `FUN_802ba630` at `0x802BA630` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba630_FUN_802ba630.c`
/// - observed: `FUN_80228dd0(&DAT_804238a8,0x44);
///   uVar1 = FUN_801b4820(FUN_802ba710); *(r13 - 0x4840) = uVar1;`
///
/// # Side effects
/// Creates the process-wide list and installs a scheduler callback.
///
/// # Safety
/// Must run once before any stream is created; the callback must remain valid.
///
/// # Confidence
/// high - both calls and the store are explicit; `0x801B4820` is identified as
/// the switch-callback setter by its body.
pub unsafe fn mount_stream_list_init() {
    platform::call2(FN_UT_LIST_INIT, G_MOUNT_STREAM_LIST as u32, 0x44);
    let previous = platform::call1(FN_OS_SET_SWITCH_CALLBACK, 0x802B_A710);
    platform::write_u32(G_MOUNT_SWITCH_CALLBACK, previous);
}

/// Applies or clears a stream's archive-heap override for the current thread.
///
/// When called on the stream's own thread, the current archive heap is switched
/// to `heap` (or restored to the value cached at `+0x40` when `heap` is 0) and
/// the cached word is updated. On any other thread the override is only stored
/// at `+0x40` for the switch callback to apply later. The scheduler lock is held
/// across the decision.
///
/// # Original
/// - Ghidra: `FUN_802ba670` at `0x802BA670` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba670_FUN_802ba670.c`
/// - observed: `FUN_801b4b70(); if (FUN_801b4b30() == *(param_1 + 8)) {
///   if (param_2 == 0) { if (*(param_1 + 0x40) != 0) { FUN_802b91d0();
///   *(param_1 + 0x40) = 0; } } else { if (*(param_1 + 0x40) == 0)
///   *(param_1 + 0x40) = *(r13 - 0x4870); FUN_802b91d0(param_2); } }
///   else *(param_1 + 0x40) = param_2; FUN_801b4bb0();`
/// - `// inferred:` the dropped argument of `0x802B91D0` in the clear branch is
///   the cached `+0x40` value being restored.
///
/// # Arguments
/// - `this`: stream object.
/// - `heap`: archive heap to install, or 0 to restore the cached one.
///
/// # Safety
/// `this` must be a live stream; `heap` must be 0 or a live archive heap.
///
/// # Confidence
/// high for the branch layout; medium for the restored argument in the clear
/// branch.
pub unsafe fn mount_stream_apply_heap(this: usize, heap: usize) {
    platform::call0(FN_OS_SCHED_LOCK);
    let owner = call0_u32(OS_GET_CURRENT_THREAD);
    let thread = platform::read_u32(this + STREAM_THREAD);
    if owner == thread {
        let cached = platform::read_u32(this + STREAM_HEAP_OVERRIDE) as usize;
        if heap == 0 {
            if cached != 0 {
                archive::arc_heap_store_current(cached);
                platform::write_u32(this + STREAM_HEAP_OVERRIDE, 0);
            }
        } else {
            if cached == 0 {
                platform::write_u32(
                    this + STREAM_HEAP_OVERRIDE,
                    platform::read_u32(G_ARC_HEAP_CURRENT),
                );
            }
            archive::arc_heap_store_current(heap);
        }
    } else {
        platform::write_u32(this + STREAM_HEAP_OVERRIDE, heap as u32);
    }
    platform::call0(FN_OS_SCHED_UNLOCK);
}

/// Thread-switch callback that applies each stream's cached heap override.
///
/// Receives the outgoing and incoming thread pointers (the CodeWarrior
/// register-save pair), resolves both to stream objects, caches/restores the
/// archive heap around the switch, and then forwards the pair to the previous
/// callback saved by [`mount_stream_list_init`]. The outgoing stream's resume
/// slot `+0x14` runs before its heap is swapped; the incoming stream's suspend
/// slot `+0x10` runs after.
///
/// # Original
/// - Ghidra: `FUN_802ba710` at `0x802BA710` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba710_FUN_802ba710.c`
/// - observed: `FUN_802DCD74` is the register save; high/low are the two thread
///   arguments. Both are resolved with `FUN_802ba5c0`; each stream with a
///   non-zero `+0x40` restores it through `FUN_802b91d0` and stores the previous
///   current heap back into `+0x40`.
///
/// # Arguments
/// - `outgoing`: thread being switched away from, or 0.
/// - `incoming`: thread being switched to, or 0.
///
/// # Safety
/// Runs from the scheduler callback with interrupts in the OS-defined state;
/// the stream list and heaps must be live.
///
/// # Confidence
/// medium - the callback arguments come from the save thunk, and the dropped
/// `0x802B91D0` argument is inferred from the surrounding store.
pub unsafe fn mount_stream_switch(outgoing: u32, incoming: u32) {
    let previous_stream = if outgoing != 0 {
        mount_stream_find(outgoing) as usize
    } else {
        0
    };
    let next_stream = if incoming != 0 {
        mount_stream_find(incoming) as usize
    } else {
        0
    };

    if previous_stream != 0 {
        let resume = platform::read_u32(platform::read_u32(previous_stream) as usize + 0x14);
        platform::call1(resume as usize, previous_stream as u32);
        let cached = platform::read_u32(previous_stream + STREAM_HEAP_OVERRIDE) as usize;
        if cached != 0 {
            let current = platform::read_u32(G_ARC_HEAP_CURRENT);
            archive::arc_heap_store_current(cached);
            platform::write_u32(previous_stream + STREAM_HEAP_OVERRIDE, current);
        }
    }

    if next_stream != 0 {
        let cached = platform::read_u32(next_stream + STREAM_HEAP_OVERRIDE) as usize;
        if cached != 0 {
            let current = platform::read_u32(G_ARC_HEAP_CURRENT);
            archive::arc_heap_store_current(cached);
            platform::write_u32(next_stream + STREAM_HEAP_OVERRIDE, current);
        }
        let suspend = platform::read_u32(platform::read_u32(next_stream) as usize + 0x10);
        platform::call1(suspend as usize, next_stream as u32);
    }

    let previous = platform::read_u32(G_MOUNT_SWITCH_CALLBACK) as usize;
    if previous != 0 {
        platform::call2(previous, outgoing, incoming);
    }
}

/// Publishes a stream: allocates its message ring, initialises the queue and
/// appends it to the global stream list.
///
/// # Original
/// - Ghidra: `FUN_802ba800` at `0x802BA800` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802ba800_FUN_802ba800.c`
/// - observed: `*(param_1 + 0x30) = param_2; uVar1 = FUN_802b8cc0(param_2 << 2,4);
///   *(param_1 + 0x2c) = uVar1; FUN_801b2390(param_1 + 0xc,uVar1,param_1[0x30]);
///   FUN_80228df0(&DAT_804238a8,param_1);`
///
/// # Arguments
/// - `this`: stream object.
/// - `ring_count`: number of messages the queue can hold.
///
/// # Side effects
/// Allocates `ring_count * 4` bytes, initialises `OSMessageQueue` at `+0x0C`
/// and appends the stream to [`G_MOUNT_STREAM_LIST`].
///
/// # Safety
/// `this` must be writable and not yet in the list; `ring_count` must be
/// non-zero.
///
/// # Confidence
/// high - allocation size, queue init and list append are explicit.
pub unsafe fn mount_stream_init_queue(this: usize, ring_count: u32) {
    platform::write_u32(this + STREAM_RING_COUNT, ring_count);
    let ring = archive::arc_heap_allocate(ring_count << 2, 4, 0);
    platform::write_u32(this + STREAM_RING, ring);
    crate::runtime::os::init_message_queue(
        this + STREAM_QUEUE,
        ring as usize,
        platform::read_u32(this + STREAM_RING_COUNT),
    );
    platform::call2(FN_UT_LIST_APPEND, G_MOUNT_STREAM_LIST as u32, this as u32);
}

// ===========================================================================
// Mount/archive heap layer (frame-heap save/restore, game-heap table,
// mount heap slots): 0x8016E740..0x8016EB50
//
// This is the heap machinery every registry mount runs inside of. A *frame
// heap* wraps the archive work: [`fn_8016e9b0`] builds one around the current
// allocation and [`fn_8016e9f0`] publishes the three indexed game heaps into
// the table `dres` indices read through [`G_GAME_HEAP_DEFAULT`]. Three named
// slots (`r13 - 0x522C/0x5230/0x5234`) and the mount DVD-heap slot
// [`G_MOUNT_DVD_HEAP`]) hold the remaining process-wide heaps. The two big
// `@0x8016E500`/`@0x8016E630` and `@0x8016E7F0` routines are the
// "allocate inside the current EGG heap" constructors `info_c`'s data heap is
// built with; `E9B0` is a thin wrapper over `E630`.
// ===========================================================================

/// `r13 - 0x5238`: single save word for the current heap across a frame-heap
/// wrap/save/restore cycle. Written by [`fn_8016e970`], read by
/// [`fn_8016e980`].
const G_FRAME_HEAP_SAVE: usize = SDA_BASE - 0x5238;
/// `r13 - 0x522C`: named heap slot published by [`fn_8016eb20`].
const G_HEAP_SLOT_522C: usize = SDA_BASE - 0x522C;
/// `r13 - 0x5230`: named heap slot published by [`fn_8016eaf0`].
const G_HEAP_SLOT_5230: usize = SDA_BASE - 0x5230;
/// `r13 - 0x5234`: named heap slot published by [`fn_8016eac0`].
const G_HEAP_SLOT_5234: usize = SDA_BASE - 0x5234;
/// `r13 - 0x61F0` byte: which entry of the game-heap table is the "current"
/// heap; compared against the index in [`fn_8016e9f0`].
const G_GAME_HEAP_CURRENT_INDEX: usize = SDA_BASE - 0x61F0;
/// `0x802F5A40`: three-word table of heap-descriptor addresses indexed by the
/// [`fn_8016e9f0`] index (`&DAT_802f5a40 + index * 4`).
const GAME_HEAP_DESCRIPTORS: usize = 0x802F_5A40;

/// `0x802B9360` - EGG allocator variant used by [`fn_8016e500`]:
/// `(buffer, heap_base, flags) -> object`.
const FN_EGG_ALLOC_A: usize = 0x802B_9360;
/// `0x802B9840` - EGG allocator variant used by [`fn_8016e630`].
const FN_EGG_ALLOC_B: usize = 0x802B_9840;
/// `0x802B9B80` - EGG allocator used by the read-path constructor
/// [`fn_8016e7f0`].
const FN_EGG_ALLOC_C: usize = 0x802B_9B80;
/// `0x8016E8E0` - 4-byte thunk to `0x802B9DF0` used by [`fn_8016e7f0`] to get
/// the raw allocation buffer.
const FN_EGG_ALLOC_BUF_THUNK: usize = 0x8016_E8E0;
/// `0x802B9170` - restore the saved current heap (called through the
/// `0x8016E4F0` thunk by [`fn_8016e980`]).
const FN_EGG_HEAP_RESTORE: usize = 0x802B_9170;
/// `0x802BA2B0` - allocate the mount DVD-heap object used by
/// [`fn_8016eb50`] (no arguments; its result seeds `0x802BA000`).
const FN_DVD_HEAP_ALLOC: usize = 0x802B_A2B0;
/// `0x802BA000` - construct the mount DVD-heap object on an allocation:
/// `(allocation, arg)`.
const FN_DVD_HEAP_CONSTRUCT: usize = 0x802B_A000;

/// Allocates inside the current EGG heap and seeds the object override word.
///
/// The raw 64-bit result of the register-save stub `0x802DCD70` supplies two
/// values: the high word selects the alignment path (`-1` means "use the
/// heap's own vtable `+0x28` allocator", otherwise the `0x88`-biased cursor
/// from the already-translated [`crate::ui::lyt::fn_8016e610`]) and the low
/// word is the heap, falling back to [`G_ARC_HEAP_CURRENT`] when zero. The
/// size is clamped to `0x20` minimum. The created object is stored through
/// the heap's `+0x14` factory slot, optionally receiving the caller's
/// `user_param` at `+0x34`.
///
/// # Original
/// - Ghidra: `FUN_8016e500` at `0x8016E500` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e500_FUN_8016e500.c`
/// - Removed: the modelled `0x802DCDBC` publish call at the tail. `0x802DCDBC`
///   is the `_restgpr0_26` epilogue fragment (`lwz r26..r31` then return), not
///   a function; the matching `0x802DCD70` prologue fragment is already
///   spelled out as the `base`/`heap` parameters.
/// - observed: `uVar6 = FUN_802dcd70(); iVar2 = (int)((ulonglong)uVar6 >> 0x20);
///   piVar4 = (int *)uVar6; if (piVar4 == 0) { piVar4 = *(r13 - 0x4870); }
///   ... uVar1 = (iVar2 == -1) ? (**(piVar4 + 0x28))(piVar4,param_4)
///   : FUN_8016e610(iVar2,param_4); iVar2 = (**(piVar4 + 0x14))(piVar4,uVar1,param_4);
///   ... if (iVar5 == 0) (**(piVar4 + 0x18))(piVar4,iVar2);
///   else if (param_3 != 0) *(int *)(iVar5 + 0x34) = param_3;
///   FUN_802dcdbc(iVar5);`
///
/// # Arguments
/// - `base`: alignment base (the dropped `r3` word). `-1` means "let the
///   heap's own `+0x28` allocator pick the buffer"; any other value is handed
///   to [`crate::ui::lyt::fn_8016e610`].
/// - `heap`: EGG heap (the dropped `r4` word), or 0 for the current heap
///   [`G_ARC_HEAP_CURRENT`].
/// - `user_param`: value stored at the created object's `+0x34`, or 0.
/// - `size`: allocation size; clamped to `0x20` when below.
/// - `flags`: flag word masked by [`crate::ui::lyt::fn_8016e4b0`].
///
/// # Side effects
/// Allocates inside the current EGG heap and may write `+0x34` of the new
/// object.
///
/// # Safety
/// The heap must be live; `size` is not range-checked.
///
/// # Confidence
/// medium - the branch, clamp and all stores are literal; the meaning of the
/// two register-pair words and the `+0x28`/`+0x14`/`+0x18` vtable slots is
/// inferred from the allocator family around `0x802B9360`.
/// `// observed:` `uVar6 = FUN_802dcd70()` re-reads the dropped `r3:r4` (the
/// arguments above), so it is spelled out as two parameters instead of a
/// `call_u64`.
pub unsafe fn fn_8016e500(
    base: i32,
    heap: usize,
    user_param: u32,
    size: u32,
    flags: u32,
) -> u32 {
    let mut heap = heap;
    let mut size = size;
    if heap == 0 {
        heap = platform::read_u32(G_ARC_HEAP_CURRENT) as usize;
    }
    if size < 0x20 {
        size = 0x20;
    }
    // observed: `iVar2 == -1` selects the vtable allocator slot.
    let base_ptr = if base == -1 {
        let vtable = platform::read_u32(heap) as usize;
        platform::call2(platform::read_u32(vtable + 0x28) as usize, heap as u32, size)
    } else {
        crate::ui::lyt::fn_8016e610(base, size as i32) as u32
    };
    // observed: create through the heap `+0x14` factory slot.
    let vtable = platform::read_u32(heap) as usize;
    let object = platform::call3(
        platform::read_u32(vtable + 0x14) as usize,
        heap as u32,
        base_ptr,
        size,
    );
    let mut result = 0u32;
    if object != 0 {
        let filtered = crate::ui::lyt::fn_8016e4b0(flags);
        result = platform::call3(FN_EGG_ALLOC_A, object, base_ptr, filtered as u32);
        if result == 0 {
            platform::call2(
                platform::read_u32(vtable + 0x18) as usize,
                heap as u32,
                object,
            );
        } else if user_param != 0 {
            platform::write_u32(result as usize + 0x34, user_param);
        }
    }
    result
}

/// Allocates inside the current EGG heap (`0x802B9840` variant).
///
/// Same register-pair / clamp / factory skeleton as [`fn_8016e500`], but the
/// alignment cursor comes from the `0x80`-biased helper
/// [`crate::ui::lyt::fn_8016e7d0`] instead of `fn_8016e610`, and the object
/// is created through `0x802B9840`.
///
/// # Original
/// - Ghidra: `FUN_8016e630` at `0x8016E630` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e630_FUN_8016e630.c`
/// - Removed: the modelled `0x802DCDBC` publish call at the tail; it is the
///   `_restgpr0_26` epilogue fragment, not a function (see [`fn_8016e500`]).
/// - observed: identical to `0x8016E500` except `FUN_8016e7d0(iVar2,param_4)`
///   and `FUN_802b9840(iVar2,uVar1,uVar3)`.
///
/// # Arguments
/// - `base`: alignment base (the dropped `r3` word); see [`fn_8016e500`].
/// - `heap`: EGG heap (the dropped `r4` word), or 0 for the current heap.
/// - `user_param`: value stored at the created object's `+0x34`, or 0.
/// - `size`: allocation size; clamped to `0x20` when below.
/// - `flags`: flag word filtered by [`crate::ui::lyt::fn_8016e4b0`].
///
/// # Side effects
/// Allocates inside the current EGG heap and may write the `+0x34` word.
///
/// # Safety
/// Same as [`fn_8016e500`].
///
/// # Confidence
/// medium - identical skeleton to `fn_8016e500`; only the two callees differ.
pub unsafe fn fn_8016e630(
    base: i32,
    heap: usize,
    user_param: u32,
    size: u32,
    flags: u32,
) -> u32 {
    let mut heap = heap;
    let mut size = size;
    if heap == 0 {
        heap = platform::read_u32(G_ARC_HEAP_CURRENT) as usize;
    }
    if size < 0x20 {
        size = 0x20;
    }
    let vtable = platform::read_u32(heap) as usize;
    let base_ptr = if base == -1 {
        platform::call2(platform::read_u32(vtable + 0x28) as usize, heap as u32, size)
    } else {
        crate::ui::lyt::fn_8016e7d0(base, size as i32) as u32
    };
    let object = platform::call3(
        platform::read_u32(vtable + 0x14) as usize,
        heap as u32,
        base_ptr,
        size,
    );
    let mut result = 0u32;
    if object != 0 {
        let filtered = crate::ui::lyt::fn_8016e4b0(flags);
        result = platform::call3(FN_EGG_ALLOC_B, object, base_ptr, filtered as u32);
        if result == 0 {
            platform::call2(
                platform::read_u32(vtable + 0x18) as usize,
                heap as u32,
                object,
            );
        } else if user_param != 0 {
            platform::write_u32(result as usize + 0x34, user_param);
        }
    }
    result
}

/// Read-path equivalent of [`fn_8016e630`]'s allocation.
///
/// The allocation buffer comes from the `0x8016E8E0` thunk (`0x802B9DF0`)
/// rather than the heap vtable `+0x28`, the register-pair stub is `0x802DCD68`,
/// and the object is created through `0x802B9B80`.
///
/// # Original
/// - Ghidra: `FUN_8016e7f0` at `0x8016E7F0` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e7f0_FUN_8016e7f0.c`
/// - Removed: the modelled `0x802DCDB4` publish call at the tail. It is the
///   `_restgpr0_24` epilogue fragment (`lwz r24..r31` then return); the
///   `0x802DCD68` prologue fragment is the `base`/`heap_arg` parameter pair.
/// - observed: `uVar6 = FUN_802dcd68(); uVar1 = high;
///   if (param_3 == 0) param_3 = *(r13 - 0x4870); ...
///   uVar2 = thunk_FUN_802b9df0(uVar1,(int)uVar6,param_5);
///   ... iVar3 = (**(param_3 + 0x14))(param_3,uVar2,param_5);
///   iVar5 = FUN_802b9b80(iVar3,uVar2,uVar1,param_5,uVar4);
///   if (iVar5 == 0) (**(param_3 + 0x18))(param_3,iVar3);
///   else if (param_4 != 0) *(int *)(iVar5 + 0x34) = param_4;
///   FUN_802dcdb4(iVar5);`
///
/// # Arguments
/// - `base`: alignment base (the dropped `r3` word; also fed to the buffer
///   thunk and to `0x802B9B80` as `uVar1`).
/// - `heap_arg`: second dropped word (the thunk's second word).
/// - `heap`: EGG heap, or 0 to use the current heap [`G_ARC_HEAP_CURRENT`].
/// - `user_param`: value stored at the created object's `+0x34`, or 0.
/// - `size`: allocation size; clamped to `0x20` when below.
/// - `flags`: flag word filtered by [`crate::ui::lyt::fn_8016e4b0`].
///
/// # Side effects
/// Advances through the read allocator; writes `+0x34` on success.
///
/// # Safety
/// Same heap requirements as [`fn_8016e500`].
///
/// # Confidence
/// medium - the `(buffer, object, size)` shape and all branches are literal;
/// the thunk at `0x8016E8E0` keeps its address for the inferred role.
/// `// observed:` the register-pair stub `0x802DCD68` re-reads `r3:r4` (the
/// arguments above).
pub unsafe fn fn_8016e7f0(
    base: u32,
    heap_arg: u32,
    heap: usize,
    user_param: u32,
    size: u32,
    flags: u32,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(G_ARC_HEAP_CURRENT) as usize
    } else {
        heap
    };
    let mut size = size;
    if size < 0x20 {
        size = 0x20;
    }
    // observed: `thunk_FUN_802b9df0(uVar1, (int)uVar6, param_5)` - buffer alloc.
    let buffer = platform::call3(
        FN_EGG_ALLOC_BUF_THUNK,
        base,
        heap_arg,
        size,
    );
    let vtable = platform::read_u32(heap) as usize;
    let object = platform::call3(
        platform::read_u32(vtable + 0x14) as usize,
        heap as u32,
        buffer,
        size,
    );
    let mut result = 0u32;
    if object != 0 {
        let filtered = crate::ui::lyt::fn_8016e4b0(flags);
        result = platform::call5(
            FN_EGG_ALLOC_C,
            object,
            buffer,
            base,
            size,
            filtered as u32,
        );
        if result == 0 {
            platform::call2(
                platform::read_u32(vtable + 0x18) as usize,
                heap as u32,
                object,
            );
        } else if user_param != 0 {
            platform::write_u32(result as usize + 0x34, user_param);
        }
    }
    result
}

/// Destroys a frame heap through its vtable slot `+0x1C`.
///
/// This is the body behind the [`HEAP_DESTROY_FRM`] address that
/// `res_info_cleanup` calls; a null heap returns immediately.
///
/// # Original
/// - Ghidra: `FUN_8016e740` at `0x8016E740` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e740_FUN_8016e740.c`
/// - observed: `if (param_1 == 0) return; (**(param_1 + 0x1c))();`
///
/// # Arguments
/// - `this`: frame heap object, or 0.
///
/// # Side effects
/// Runs the heap's virtual destroy slot.
///
/// # Safety
/// A non-null `this` must be a live frame heap whose vtable `+0x1C` accepts
/// the object as its receiver.
///
/// # Confidence
/// high - null guard and the indirect call are literal; the slot's role is the
/// registry's own name for this address (`HEAP_DESTROY_FRM`).
pub unsafe fn fn_8016e740(this: usize) {
    if this == 0 {
        return;
    }
    let vtable = platform::read_u32(this) as usize;
    platform::call1(platform::read_u32(vtable + 0x1C) as usize, this as u32);
}

/// Records the current heap in the frame save slot.
///
/// Called before a frame heap is installed so [`fn_8016e980`] can restore it.
/// `res_info_set_res` uses the create/restore pair around its archive load.
///
/// # Original
/// - Ghidra: `FUN_8016e970` at `0x8016E970` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e970_FUN_8016e970.c`
/// - observed: `*(r13 - 0x5238) = *(r13 - 0x4870);`
///
/// # Side effects
/// Writes [`G_FRAME_HEAP_SAVE`].
///
/// # Confidence
/// high - one load, one store.
pub unsafe fn fn_8016e970() {
    let current = platform::read_u32(G_ARC_HEAP_CURRENT);
    platform::write_u32(G_FRAME_HEAP_SAVE, current);
}

/// Restores the heap saved by [`fn_8016e970`] and clears the slot.
///
/// # Original
/// - Ghidra: `FUN_8016e980` at `0x8016E980` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e980_FUN_8016e980.c`
/// - observed: `FUN_802b9170(*(r13 - 0x5238)); *(r13 - 0x5238) = 0;`
///
/// # Side effects
/// Calls the heap-restore helper with the saved heap, then clears
/// [`G_FRAME_HEAP_SAVE`].
///
/// # Confidence
/// high - both operations are explicit.
pub unsafe fn fn_8016e980() {
    let saved = platform::read_u32(G_FRAME_HEAP_SAVE);
    platform::call1(FN_EGG_HEAP_RESTORE, saved);
    platform::write_u32(G_FRAME_HEAP_SAVE, 0);
}

/// Creates a frame heap on the current heap and records the previous one.
///
/// This is the body behind [`HEAP_CREATE_FRM_TO_CURRENT`], called by
/// `res_info_set_res` with a heap-hint/size/name argument block that Ghidra
/// drops. The save stub consumes that block; here it is passed through as the
/// five dropped words the original actually pushes (the C prints five
/// parameters for this address at its call sites).
///
/// # Original
/// - Ghidra: `FUN_8016e9b0` at `0x8016E9B0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e9b0_FUN_8016e9b0.c`
/// - observed: `iVar1 = FUN_8016e630(); if (iVar1 != 0) {
///   uVar2 = thunk_FUN_802b9170(); *(r13 - 0x5238) = uVar2; } return iVar1;`
///   - `// inferred:` the five dropped arguments (size/heap/name/align/option)
///   are still live at the tail call, exactly as `res_info_set_res` passes
///   them (`0xffffffff`, heap, `DATA_HEAP_NAME`, `0x20`, `0`).
///
/// # Returns
/// The created frame heap, or 0.
///
/// # Side effects
/// Creates a heap through [`fn_8016e630`] and writes [`G_FRAME_HEAP_SAVE`]
/// with the previous current heap when creation succeeds.
///
/// # Safety
/// The current heap must be live; the dropped block must match the five-word
/// heap-creation shape used by `res_info_set_res`.
///
/// # Confidence
/// medium - the create/save/return skeleton is literal; the five dropped
/// arguments and `0x802B9170`'s "previous heap" role are inferred from the
/// `res_info_set_res` call site (which documents the same address).
pub unsafe fn fn_8016e9b0(
    base: i32,
    heap: u32,
    name: u32,
    size: u32,
    flags: u32,
) -> u32 {
    let created = fn_8016e630(base, heap as usize, name, size, flags);
    if created != 0 {
        let previous = platform::call1(FN_EGG_HEAP_RESTORE, 0);
        platform::write_u32(G_FRAME_HEAP_SAVE, previous);
    }
    created
}

/// Creates and publishes game heap `param_1` in the global heap table.
///
/// Only indexes `1` and `2` are valid (`param_1 != 0 && param_1 < 3`); any
/// other index returns 0 without touching the table. The heap comes from the
/// factory at `0x8016E8F0` with the descriptor in
/// [`GAME_HEAP_DESCRIPTORS`]; the entry is stored in the `0x80377C48` table,
/// and when the index matches the current selector byte at
/// [`G_GAME_HEAP_CURRENT_INDEX`] it also becomes the current-head heap.
///
/// # Original
/// - Ghidra: `FUN_8016e9f0` at `0x8016E9F0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e9f0_FUN_8016e9f0.c`
/// - observed: `if ((param_1 != 0) && (param_1 < 3)) {
///   uVar3 = FUN_8016e8f0(param_2,param_3,*(0x802f5a40 + param_1 * 4));
///   bVar1 = *(r13 - 0x61f0); (&DAT_80377c48)[param_1] = uVar3;
///   if (param_1 == bVar1) DAT_80377c48 = (&DAT_80377c48)[param_1];
///   uVar3 = (&DAT_80377c48)[param_1]; } else uVar3 = 0; return uVar3;`
///
/// # Arguments
/// - `index`: game-heap slot index; only `1` and `2` ever succeed.
/// - `param_2`, `param_3`: the two words the `0x8016E8F0` factory takes.
///
/// # Returns
/// The created heap, or 0.
///
/// # Side effects
/// Writes the 3-entry table at `0x80377C48` and may repoint its first entry.
///
/// # Safety
/// `index` outside `1..3` is rejected (returns 0); the two factory words must
/// fit the `0x8016E8F0` factory shape.
///
/// # Confidence
/// high - the guard, the factory call, the table store and the index compare
/// are all literal; the table roles are the registry's own names for the same
/// `0x80377C48` global.
pub unsafe fn fn_8016e9f0(index: u32, param_2: u32, param_3: u32) -> u32 {
    if index == 0 || index >= 3 {
        return 0;
    }
    let descriptor = platform::read_u32(GAME_HEAP_DESCRIPTORS + index as usize * 4);
    let heap = crate::ui::lyt::fn_8016e8f0(param_2, param_3, descriptor as i32);
    platform::write_u32(G_GAME_HEAP_DEFAULT + index as usize * 4, heap);
    let current = platform::read_u8(G_GAME_HEAP_CURRENT_INDEX);
    if index == current as u32 {
        platform::write_u32(G_GAME_HEAP_DEFAULT, heap);
    }
    platform::read_u32(G_GAME_HEAP_DEFAULT + index as usize * 4)
}



/// Wrapper for game heap `1` ([`fn_8016e9f0`] with index 1).
///
/// # Original
/// - Ghidra: `FUN_8016eaa0` at `0x8016EAA0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eaa0_FUN_8016eaa0.c`
/// - observed: `FUN_8016e9f0(1,param_1,param_2);`
///
/// # Arguments
/// - `param_1`, `param_2`: forwarded to [`fn_8016e9f0`].
///
/// # Confidence
/// high - one tail call.
pub unsafe fn fn_8016eaa0(param_1: u32, param_2: u32) {
    fn_8016e9f0(1, param_1, param_2);
}

/// Wrapper for game heap `2` ([`fn_8016e9f0`] with index 2).
///
/// # Original
/// - Ghidra: `FUN_8016eab0` at `0x8016EAB0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eab0_FUN_8016eab0.c`
/// - observed: `FUN_8016e9f0(2,param_1,param_2);`
///
/// # Arguments
/// - `param_1`, `param_2`: forwarded to [`fn_8016e9f0`].
///
/// # Confidence
/// high - one tail call.
pub unsafe fn fn_8016eab0(param_1: u32, param_2: u32) {
    fn_8016e9f0(2, param_1, param_2);
}

/// Publishes the `0x80329B58` heap in the first named mount-heap slot.
///
/// # Original
/// - Ghidra: `FUN_8016eac0` at `0x8016EAC0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eac0_FUN_8016eac0.c`
/// - observed: `uVar1 = FUN_8016e8f0(param_1,param_2,&DAT_80329b58);
///   *(r13 - 0x5234) = uVar1;`
///
/// # Arguments
/// - `param_1`, `param_2`: forwarded to the `0x8016E8F0` factory.
///
/// # Side effects
/// Writes [`G_HEAP_SLOT_5234`].
///
/// # Confidence
/// high - one factory call and one store; the slot/descriptor names are the
/// observed addresses.
pub unsafe fn fn_8016eac0(param_1: u32, param_2: u32) {
    let heap = crate::ui::lyt::fn_8016e8f0(param_1, param_2, 0x8032_9B58u32 as i32);
    platform::write_u32(G_HEAP_SLOT_5234, heap);
}

/// Publishes the `0x80329B90` heap in the second named mount-heap slot.
///
/// # Original
/// - Ghidra: `FUN_8016eaf0` at `0x8016EAF0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eaf0_FUN_8016eaf0.c`
/// - observed: `uVar1 = FUN_8016e8f0(param_1,param_2,&DAT_80329b90);
///   *(r13 - 0x5230) = uVar1;`
///
/// # Arguments
/// - `param_1`, `param_2`: forwarded to the `0x8016E8F0` factory.
///
/// # Side effects
/// Writes [`G_HEAP_SLOT_5230`].
///
/// # Confidence
/// high - one factory call and one store.
pub unsafe fn fn_8016eaf0(param_1: u32, param_2: u32) {
    let heap = crate::ui::lyt::fn_8016e8f0(param_1, param_2, 0x8032_9B90u32 as i32);
    platform::write_u32(G_HEAP_SLOT_5230, heap);
}

/// Publishes the `0x80329BC0` heap in the third named mount-heap slot.
///
/// # Original
/// - Ghidra: `FUN_8016eb20` at `0x8016EB20` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eb20_FUN_8016eb20.c`
/// - observed: `uVar1 = FUN_8016e8f0(param_1,param_2,&DAT_80329bc0);
///   *(r13 - 0x522c) = uVar1;`
///
/// # Arguments
/// - `param_1`, `param_2`: forwarded to the `0x8016E8F0` factory.
///
/// # Side effects
/// Writes [`G_HEAP_SLOT_522C`].
///
/// # Confidence
/// high - one factory call and one store.
pub unsafe fn fn_8016eb20(param_1: u32, param_2: u32) {
    let heap = crate::ui::lyt::fn_8016e8f0(param_1, param_2, 0x8032_9BC0u32 as i32);
    platform::write_u32(G_HEAP_SLOT_522C, heap);
}

/// Creates the mount DVD-heap object and publishes it in [`G_MOUNT_DVD_HEAP`].
///
/// The object is allocated by `0x802BA2B0` and constructed by `0x802BA000`
/// with the caller's `param_1`; the descriptor address `0x80329BF0` is then
/// written to its `+0x34` word. `mount_stream_manager_init` installs this heap
/// on the mount stream when it exists (see [`G_MOUNT_DVD_HEAP`]).
///
/// # Original
/// - Ghidra: `FUN_8016eb50` at `0x8016EB50` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eb50_FUN_8016eb50.c`
/// - observed: `uVar1 = FUN_802ba2b0(); iVar2 = FUN_802ba000(uVar1,param_1);
///   *(r13 - 0x5228) = iVar2; *(iVar2 + 0x34) = &DAT_80329bf0;
///   return *(r13 - 0x5228);`
///
/// # Arguments
/// - `param_1`: first word of the `0x802BA000` constructor block.
///
/// # Returns
/// The created heap object.
///
/// # Side effects
/// Allocates and constructs the object and writes [`G_MOUNT_DVD_HEAP`].
///
/// # Safety
/// The mount-stream base layer (`0x802BAxxx`) must be initialised; `param_1`
/// must fit the constructor block.
///
/// # Confidence
/// high - the two calls, the global store and the `+0x34` descriptor store are
/// literal; the two callee names are by address.
pub unsafe fn fn_8016eb50(param_1: u32) -> u32 {
    let allocation = call0_u32(FN_DVD_HEAP_ALLOC);
    let object = platform::call2(FN_DVD_HEAP_CONSTRUCT, allocation, param_1);
    platform::write_u32(G_MOUNT_DVD_HEAP, object);
    platform::write_u32(object as usize + 0x34, 0x8032_9BF0);
    object
}

// ===========================================================================
// Stream/frame-heap wrapper family (wave 39)
//
// The wave-38 heap layer (0x8016E500..0x8016EB50) publishes heaps, but the
// per-object wrappers that put a frame heap around one allocation live here.
// A single stream object (constructed by [`fn_8016a480`] over the EGG stream
// base, [`mount_stream_base_construct`]) is published in
// [`G_HF_STREAM_SINGLETON`]; the `dSys` initialisation installs it through
// [`fn_8016a700`]. The heap-view family below attaches an EGG heap to a small
// (0x38-byte) view object: `+0x14` holds the heap, `+0x18` the user/size
// word, and the heap's vtable slots `+0x0C` (kind test), `+0x10` (attach) and
// `+0x1C` (detach) drive the switch.
// ===========================================================================

/// `r13 - 0x5298`: the stream singleton published by [`fn_8016a480`] and read
/// by the heap-view family.
const G_HF_STREAM_SINGLETON: usize = SDA_BASE - 0x5298;
/// Heap-view field: the attached EGG heap.
const HF_HEAP: usize = 0x14;
/// Heap-view field: the user/size word stored by [`fn_8016a690`].
const HF_USER_PARAM: usize = 0x18;
/// `0x803296D0`: vtable installed by [`fn_8016a480`] on the stream singleton.
const HF_STREAM_BASE_VTABLE: usize = 0x8032_96D0;
/// `0x803296BC`: heap-view vtable installed by [`fn_8016a5e0`] after the LZ
/// stream-object construction overwrites `+0x10`.
const HF_LZ_VIEW_VTABLE_A: usize = 0x8032_96BC;
/// `0x803296A8`: heap-view vtable installed by [`fn_8016a740`] and
/// [`fn_8016a780`].
const HF_LZ_VIEW_VTABLE_B: usize = 0x8032_96A8;

/// Constructs the stream singleton over the EGG stream base and publishes it.
///
/// The base constructor ([`mount_stream_base_construct`], `0x802BA2D0`) runs
/// first, then the `u16` at `+0x1C` is zeroed, [`HF_STREAM_BASE_VTABLE`] is
/// installed, and the object is stored in [`G_HF_STREAM_SINGLETON`].
///
/// # Original
/// - Ghidra: `FUN_8016a480` at `0x8016A480` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a480_FUN_8016a480.c`
/// - observed: `FUN_802ba2d0(); *(undefined2 *)(param_1 + 7) = 0;
///   *param_1 = &DAT_803296d0; *(r13 - 0x5298) = param_1;`
///
/// # Arguments
/// - `this`: writable stream object (at least `0x1E` bytes; the shipped
///   factory allocates `0x38`).
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Constructs the stream base, installs the vtable, and publishes the object
/// in [`G_HF_STREAM_SINGLETON`].
///
/// # Safety
/// `this` must be writable and must remain alive; the archive-heap list used
/// by the base constructor must be initialised.
///
/// # Confidence
/// high - the base call, the `u16` clear, the vtable store and the singleton
/// publish are literal in the export.
pub unsafe fn fn_8016a480(this: usize) -> usize {
    mount_stream_base_construct(this);
    platform::write_u16(this + 0x1C, 0);
    platform::write_u32(this, HF_STREAM_BASE_VTABLE as u32);
    platform::write_u32(G_HF_STREAM_SINGLETON, this as u32);
    this
}

/// Returns the stream singleton published by [`fn_8016a480`].
///
/// # Original
/// - Ghidra: `FUN_8016a5d0` at `0x8016A5D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a5d0_FUN_8016a5d0.c`
/// - observed: `return *(undefined4 *)(r13 + -0x5298);`
///
/// # Returns
/// The singleton address, or 0 before [`fn_8016a700`] runs.
///
/// # Confidence
/// high - one SDA load.
pub unsafe fn fn_8016a5d0() -> u32 {
    platform::read_u32(G_HF_STREAM_SINGLETON)
}

/// Initialises an LZ-stream view with the singleton as its decoder.
///
/// Runs [`archive::lz_stream_object_construct`] (`0x802B8A70`) with the
/// singleton from [`fn_8016a5d0`] as the decoder, then overwrites `+0x10`
/// with [`HF_LZ_VIEW_VTABLE_A`].
///
/// # Original
/// - Ghidra: `FUN_8016a5e0` at `0x8016A5E0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a5e0_FUN_8016a5e0.c`
/// - observed: `uVar1 = FUN_8016a5d0(); FUN_802b8a70(param_1,uVar1,4);
///   *(param_1 + 0x10) = &DAT_803296bc;`
///
/// # Arguments
/// - `this`: writable heap-view object.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the LZ construction (which attaches the singleton to `this`), then
/// overwrites `+0x10` with [`HF_LZ_VIEW_VTABLE_A`].
///
/// # Safety
/// The singleton must already be installed, and the decoder's `+0x10` slot
/// must accept the `(decoder, this)` attach call.
///
/// # Confidence
/// high - the getter, the construction call and the overwrite are literal;
/// the `0x802B8A70` role is the already-translated
/// [`archive::lz_stream_object_construct`].
pub unsafe fn fn_8016a5e0(this: usize) -> usize {
    let singleton = fn_8016a5d0() as usize;
    archive::lz_stream_object_construct(this, singleton, 4);
    platform::write_u32(this + 0x10, HF_LZ_VIEW_VTABLE_A as u32);
    this
}

/// Deleting destructor for the LZ-stream view.
///
/// Calls [`archive::lz_stream_object_delete`] (`0x802B8AD0`) with the
/// non-deleting flag `0`, then frees the object through the default deleter
/// ([`HEAP_DELETE`], `0x802B9280`) when `deleting` is positive.
///
/// # Original
/// - Ghidra: `FUN_8016a630` at `0x8016A630` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a630_FUN_8016a630.c`
/// - observed: `if ((param_1 != 0) && (FUN_802b8ad0(param_1,0), 0 < param_2))
///   { FUN_802b9280(param_1); } return param_1;`
///
/// # Arguments
/// - `this`: view object, or 0.
/// - `deleting`: `> 0` frees the object.
///
/// # Returns
/// `this` unchanged.
///
/// # Safety
/// A non-null `this` must be a live view object; the delete runs only for a
/// live heap.
///
/// # Confidence
/// high - the null guard, the comma-ordered delete call and the conditional
/// free are explicit.
pub unsafe fn fn_8016a630(this: usize, deleting: i32) -> usize {
    if this != 0 {
        archive::lz_stream_object_delete(this, 0);
        if deleting > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Attaches an EGG heap to a heap-view object.
///
/// A zero `heap` selects the current archive heap ([`G_ARC_HEAP_CURRENT`]).
/// The word lands at `+0x14`, `user_param` at `+0x18`, and the heap's
/// `+0x10` vtable slot runs with `(heap, this)`. Always returns `1`.
///
/// # Original
/// - Ghidra: `FUN_8016a690` at `0x8016A690` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a690_FUN_8016a690.c`
/// - observed: `if (param_2 == 0) param_2 = *(r13 - 0x4870);
///   *(param_1 + 0x18) = param_3; *(param_1 + 0x14) = param_2;
///   (**(param_2 + 0x10))(param_2,param_1); return 1;`
///
/// # Arguments
/// - `this`: writable heap-view object.
/// - `heap`: EGG heap to attach, or 0 for the current heap.
/// - `user_param`: word stored at `+0x18`.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// Writes `+0x14`/`+0x18` and runs the heap's attach slot.
///
/// # Safety
/// The heap (or the current heap) must be live and its `+0x10` slot must
/// implement the `(heap, view)` attach ABI.
///
/// # Confidence
/// high - the fallback, both stores and the two-argument virtual call are
/// literal.
pub unsafe fn fn_8016a690(this: usize, heap: usize, user_param: u32) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(G_ARC_HEAP_CURRENT) as usize
    } else {
        heap
    };
    platform::write_u32(this + HF_USER_PARAM, user_param);
    platform::write_u32(this + HF_HEAP, heap as u32);
    let vtable = platform::read_u32(heap) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x10) as usize,
        heap as u32,
        this as u32,
    );
    1
}

/// Allocates the 0x38-byte heap-view object and constructs the singleton.
///
/// The allocation runs through `0x802B9240` (`operator new(size, heap, 4)`)
/// on the caller's heap; a non-null result is constructed with
/// [`fn_8016a480`]. The `dSys` initialisation calls this with the game-heap
/// table value `0x80377C48`.
///
/// # Original
/// - Ghidra: `FUN_8016a700` at `0x8016A700` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a700_FUN_8016a700.c`
/// - observed: `iVar1 = FUN_802b9240(0x38,param_1,4); if (iVar1 != 0)
///   FUN_8016a480();`
///
/// # Arguments
/// - `heap`: EGG heap the `0x38`-byte object is allocated from.
///
/// # Side effects
/// Allocates on `heap` and, on success, constructs and publishes the
/// singleton.
///
/// # Safety
/// `heap` must be a live EGG heap; a null allocation is silently accepted
/// by the original.
///
/// # Confidence
/// high - the size, the null guard and the construct call are explicit; the
/// heap role comes from the `dSys` call site (`0x800E48D0`).
pub unsafe fn fn_8016a700(heap: usize) {
    let object = platform::call3(MOUNT_OBJECT_ALLOC, 0x38, heap as u32, 4);
    if object != 0 {
        fn_8016a480(object as usize);
    }
}

/// Constructs the alternate heap-view family.
///
/// Runs [`fn_8016a5e0`] (the LZ-view init) and then installs
/// [`HF_LZ_VIEW_VTABLE_B`], overwriting the `+0x10` word set by the init.
///
/// # Original
/// - Ghidra: `FUN_8016a740` at `0x8016A740` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a740_FUN_8016a740.c`
/// - observed: `FUN_8016a5e0(); *(param_1 + 0x10) = &DAT_803296a8;
///   return param_1;`
/// - `// inferred:` the init call prints no argument; `this` is still in
///   `r3`, so it receives `this`.
///
/// # Arguments
/// - `this`: writable heap-view object.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the LZ-view init, then writes [`HF_LZ_VIEW_VTABLE_B`] at `+0x10`.
///
/// # Safety
/// Same requirements as [`fn_8016a5e0`].
///
/// # Confidence
/// high - the init call and the overwrite are explicit; only the hidden
/// init argument is inferred from the ABI.
pub unsafe fn fn_8016a740(this: usize) -> usize {
    fn_8016a5e0(this);
    platform::write_u32(this + 0x10, HF_LZ_VIEW_VTABLE_B as u32);
    this
}

/// Deleting destructor for the alternate heap-view family.
///
/// Installs [`HF_LZ_VIEW_VTABLE_B`], detaches through [`fn_8016a880`], runs
/// the LZ delete with flag `0` ([`fn_8016a630`]), and frees the object when
/// `deleting` is positive.
///
/// # Original
/// - Ghidra: `FUN_8016a780` at `0x8016A780` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a780_FUN_8016a780.c`
/// - observed: `if (param_1 != 0) { *(param_1 + 0x10) = &DAT_803296a8;
///   FUN_8016a880(); FUN_8016a630(param_1,0); if (0 < param_2)
///   FUN_802b9280(param_1); } return param_1;`
///
/// # Arguments
/// - `this`: view object, or 0.
/// - `deleting`: `> 0` frees the object.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Detaches the view's heap, runs the LZ delete, and may free `this`.
///
/// # Safety
/// A non-null `this` must be a constructed view object whose heap detach is
/// safe (see [`fn_8016a880`]).
///
/// # Confidence
/// high - every step and guard is literal; the detach call's hidden argument
/// is `this` as still in `r3`.
pub unsafe fn fn_8016a780(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::write_u32(this + 0x10, HF_LZ_VIEW_VTABLE_B as u32);
        fn_8016a880(this);
        fn_8016a630(this, 0);
        if deleting > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Creates a frame heap and attaches it to a heap view.
///
/// The register-pair stub re-reads the two leading words `(this, base)`:
/// the view first detaches ([`fn_8016a880`]), a heap is created with
/// [`fn_8016e630`] and attached ([`fn_8016a690`]) on success. Returns whether
/// a heap was created. The shipped callers (`0x80044460`, `0x80115D10`) pass
/// `(view, base, heap, name, size, flags)` with `base` of `0x2000` or `-1`.
///
/// # Original
/// - Ghidra: `FUN_8016a7f0` at `0x8016A7F0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a7f0_FUN_8016a7f0.c`
/// - Removed: the modelled `0x802DCDBC` publish call with the `ok` boolean.
///   `0x802DCDBC` is the `_restgpr0_26` epilogue fragment, not a function;
///   the `0x802DCD70` prologue fragment is the `(this, base)` parameter pair.
/// - observed: `uVar2 = FUN_802dcd70(); FUN_8016a880();
///   iVar1 = FUN_8016e630((int)uVar2,param_3,param_4,param_5,param_6);
///   if (iVar1 != 0) FUN_8016a690((int)((ulonglong)uVar2 >> 0x20),iVar1,
///   param_5); FUN_802dcdbc(iVar1 != 0);`
///
/// # Arguments
/// - `this`: heap-view object (the detached-then-attached view).
/// - `base`: alignment base word (the second register-pair word).
/// - `heap`: EGG heap handed to [`fn_8016e630`].
/// - `name`: user word handed to [`fn_8016e630`].
/// - `size`: size handed to [`fn_8016e630`] and stored as the view's
///   `+0x18` word.
/// - `flags`: flag word handed to [`fn_8016e630`].
///
/// # Returns
/// `1` when a heap was created and attached, else `0`.
///
/// # Side effects
/// Detaches the view's previous heap, then creates and attaches a new frame
/// heap.
///
/// # Safety
/// `this` must be a live view; `heap` must satisfy [`fn_8016e630`]; the
/// view's old heap must be detachable.
///
/// # Confidence
/// medium - the call order and stores are literal; the six-word signature and
/// the boolean return are inferred from the register-pair shape and the
/// shipped callers.
pub unsafe fn fn_8016a7f0(
    this: usize,
    base: i32,
    heap: usize,
    name: u32,
    size: u32,
    flags: u32,
) -> u32 {
    fn_8016a880(this);
    let created = fn_8016e630(base, heap, name, size, flags);
    if created != 0 {
        fn_8016a690(this, created as usize, size);
    }
    (created != 0) as u32
}

/// Detaches the view's heap back to the singleton.
///
/// When `+0x14` is not the singleton, the heap's own `+0x1C` slot runs (the
/// detach), and `+0x14` is reset to the singleton from [`fn_8016a5d0`].
///
/// # Original
/// - Ghidra: `FUN_8016a880` at `0x8016A880` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a880_FUN_8016a880.c`
/// - observed: `piVar1 = FUN_8016a5d0(); if (*(param_1 + 0x14) != piVar1)
///   { (**(**(param_1 + 0x14) + 0x1c))(); *(param_1 + 0x14) = piVar1; }`
///
/// # Arguments
/// - `this`: heap-view object.
///
/// # Side effects
/// Runs the old heap's detach slot and restores the singleton into `+0x14`.
///
/// # Safety
/// A `+0x14` heap that is not the singleton must be live - the original
/// dereferences it without a null check.
///
/// # Confidence
/// medium - the branch, getter and store are literal; the virtual-call
/// receiver follows the [`fn_8016e740`] convention (still in `r3`).
pub unsafe fn fn_8016a880(this: usize) {
    let singleton = fn_8016a5d0() as usize;
    let heap = platform::read_u32(this + HF_HEAP) as usize;
    if heap != singleton {
        let vtable = platform::read_u32(heap) as usize;
        platform::call1(platform::read_u32(vtable + 0x1C) as usize, heap as u32);
        platform::write_u32(this + HF_HEAP, singleton as u32);
    }
}

/// Adjusts the view's frame heap and reports the remaining size.
///
/// When the view's heap is already the singleton, returns `0`. Otherwise a
/// heap whose `+0x0C` kind does not answer `2` is discarded (treated as
/// null), and the survivor is adjusted through [`crate::ui::lyt::fn_8016e760`]
/// (`0x8016E760`, the frame-heap adjust wrapper [`HEAP_ADJUST_FRM`]).
///
/// # Original
/// - Ghidra: `FUN_8016a8e0` at `0x8016A8E0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a8e0_FUN_8016a8e0.c`
/// - observed: `piVar1 = FUN_8016a5d0(); piVar4 = *(param_1 + 0x14);
///   if (piVar4 == piVar1) uVar2 = 0; else { if ((piVar4 == 0) ||
///   (iVar3 = (**(piVar4 + 0xc))(piVar4), iVar3 != 2)) piVar4 = 0;
///   uVar2 = FUN_8016e760(piVar4); } return uVar2;`
///
/// # Arguments
/// - `this`: heap-view object.
///
/// # Returns
/// The adjusted size, or `0` when the heap is the singleton.
///
/// # Side effects
/// Runs the heap's `+0x0C` kind probe and, through the adjust wrapper, its
/// `+0x2C` size probe.
///
/// # Safety
/// A non-singleton `+0x14` heap must be live with a callable `+0x0C` slot.
///
/// # Confidence
/// high - the singleton early-out, the null/short-circuit kind test and the
/// adjust call are explicit; the `0x8016E760` identity is the
/// already-translated UI-layer wrapper.
pub unsafe fn fn_8016a8e0(this: usize) -> u32 {
    let singleton = fn_8016a5d0() as usize;
    let mut heap = platform::read_u32(this + HF_HEAP) as usize;
    if heap == singleton {
        return 0;
    }
    if heap != 0 {
        let vtable = platform::read_u32(heap) as usize;
        let kind = platform::call1(platform::read_u32(vtable + 0x0C) as usize, heap as u32);
        if kind != 2 {
            heap = 0;
        }
    } else {
        heap = 0;
    }
    crate::ui::lyt::fn_8016e760(heap)
}

/// Opens a frame heap around one allocation and saves the previous one.
///
/// Forwards the six words to [`fn_8016a7f0`]; on success the current heap is
/// recorded ([`fn_8016e970`], `0x8016E970`) and the view's heap address is
/// stored as the current heap through `0x802B9170` (the restore call behind
/// the `0x8016E4F0` thunk). Returns whether the frame heap was created. The
/// shipped callers pass `(view, base, heap, name, size, flags)` and later close
/// with [`fn_8016a9b0`].
///
/// # Original
/// - Ghidra: `FUN_8016a960` at `0x8016A960` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a960_FUN_8016a960.c`
/// - observed: `iVar1 = FUN_8016a7f0(); if (iVar1 != 0) { FUN_8016e970();
///   thunk_FUN_802b9170(*(param_1 + 0x14)); } return iVar1 != 0;`
///
/// # Arguments
/// - `this`: heap-view object.
/// - `base`: alignment base word (see [`fn_8016a7f0`]).
/// - `heap`: EGG heap for the frame.
/// - `name`: user word for [`fn_8016e630`].
/// - `size`: size word.
/// - `flags`: flag word.
///
/// # Returns
/// `1` when the frame heap was opened, else `0`.
///
/// # Side effects
/// Creates and attaches a frame heap, saves the old current heap in
/// [`G_FRAME_HEAP_SAVE`], and sets the view's heap as current.
///
/// # Safety
/// Same heap requirements as [`fn_8016a7f0`]; the caller must pair this with
/// [`fn_8016a9b0`].
///
/// # Confidence
/// medium - the forward, the save and the current-heap store are literal;
/// the boolean return follows the [`fn_8016a7f0`] composition note.
pub unsafe fn fn_8016a960(
    this: usize,
    base: i32,
    heap: usize,
    name: u32,
    size: u32,
    flags: u32,
) -> u32 {
    let created = fn_8016a7f0(this, base, heap, name, size, flags);
    if created != 0 {
        fn_8016e970();
        platform::call1(
            FN_EGG_HEAP_RESTORE,
            platform::read_u32(this + HF_HEAP),
        );
    }
    created
}

/// Closes the frame heap opened by [`fn_8016a960`].
///
/// Restores the saved current heap ([`fn_8016e980`], `0x8016E980`) and runs
/// the frame-heap adjust ([`crate::ui::lyt::fn_8016e760`]) on the view's
/// heap, discarding it first when the heap is null or its `+0x0C` kind does
/// not answer `2`.
///
/// # Original
/// - Ghidra: `FUN_8016a9b0` at `0x8016A9B0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a9b0_FUN_8016a9b0.c`
/// - observed: `FUN_8016e980(); piVar2 = *(param_1 + 0x14);
///   if ((piVar2 == 0) || (iVar1 = (**(piVar2 + 0xc))(piVar2), iVar1 != 2))
///   piVar2 = 0; FUN_8016e760(piVar2);`
///
/// # Arguments
/// - `this`: heap-view object whose frame heap was opened by [`fn_8016a960`].
///
/// # Side effects
/// Restores the saved current heap and adjusts the view's frame heap.
///
/// # Safety
/// Must only run after a successful [`fn_8016a960`]; a non-null heap must be
/// live with a callable `+0x0C` slot.
///
/// # Confidence
/// high - the restore, the null/short-circuit kind test and the adjust call
/// are literal.
pub unsafe fn fn_8016a9b0(this: usize) {
    fn_8016e980();
    let mut heap = platform::read_u32(this + HF_HEAP) as usize;
    if heap != 0 {
        let vtable = platform::read_u32(heap) as usize;
        let kind = platform::call1(platform::read_u32(vtable + 0x0C) as usize, heap as u32);
        if kind != 2 {
            heap = 0;
        }
    }
    crate::ui::lyt::fn_8016e760(heap);
}

// ===========================================================================
// Sound-bank table consumers (wave 40)
//
// The registry's `info_c` table is mirrored, one level up, by a second family
// of tables at `0x80377A10`: five intrusive doubly linked lists plus one
// eight-bucket hash table at `0x80377A70`. Every node carries its `next` at
// `+0x04`; the hash buckets point at a descriptor through `+0x08` while the
// bank lists point at their payload through `+0x10`. The low key of a bank
// list node is the `u16` at `payload + 0x08` (or the `u8` at `payload + 0x0E`
// for the secondary search), which is exactly the shape [`mount_archive_find`]
// uses for the mounted-archive table.
//
// The stream side of the same layer is built by [`fn_80163180`]: it allocates a
// `0x1C`-byte LZ view through the wave-39 constructor [`fn_8016a5e0`], attaches
// the archive heap with [`fn_8016a690`], and publishes the view in
// `r13 - 0x52C0` (read back by [`fn_801634c0`]) and `r13 - 0x48B8`.
//
// The second cluster below (`0x80168060..0x801680E0`) is the accessor set for
// one `0x38`-stride array element: the element wrapper keeps its native object
// at `+0x04`, and each function forwards to one fixed vtable slot. The
// container that indexes these elements is `0x80166280..0x80166360`, which
// stays out of scope here because it is another lane's layout object.
// ===========================================================================

/// `r13 - 0x52C8`: pointer to the bank-factory table indexed by a `u16` id.
const G_SB_BANK_TABLE: usize = SDA_BASE - 0x52C8;
/// `r13 - 0x52C0`: the LZ-stream view created by [`fn_80163180`].
const G_SB_STREAM_VIEW: usize = SDA_BASE - 0x52C0;
/// `r13 - 0x48B8`: second publication of the same view, read by consumers.
const G_SB_STREAM_VIEW_CONSUMER: usize = SDA_BASE - 0x48B8;
/// `r13 - 0x6228`: current bank-group id; `2` selects the pending-list path.
const G_SB_CURRENT_GROUP: usize = SDA_BASE - 0x6228;
/// `r13 - 0x52D0`: teardown bitmask; a clear bit means "group still live".
const G_SB_TEARDOWN_FLAGS: usize = SDA_BASE - 0x52D0;
/// `r13 - 0x52E8`: argument stash, `u16` bank id (see [`fn_80162a50`]).
const G_SB_ARG_ID: usize = SDA_BASE - 0x52E8;
/// `r13 - 0x52E4`: argument stash, third word.
const G_SB_ARG_C: usize = SDA_BASE - 0x52E4;
/// `r13 - 0x52E0`: argument stash, flag byte.
const G_SB_ARG_FLAG: usize = SDA_BASE - 0x52E0;
/// `r13 - 0x52DC`: argument stash, second word.
const G_SB_ARG_B: usize = SDA_BASE - 0x52DC;

/// `0x80377A10`: bank list 1 (`u16` key at `payload + 0x08`).
const SB_LIST_1: usize = 0x8037_7A10;
/// `0x80377A20`: bank list 2, the list [`fn_80162930`] appends pending banks to.
const SB_LIST_2: usize = 0x8037_7A20;
/// `0x80377A34`: bank list 3.
const SB_LIST_3: usize = 0x8037_7A34;
/// `0x80377A48`: bank list 4.
const SB_LIST_4: usize = 0x8037_7A48;
/// `0x80377A5C`: bank list 5.
const SB_LIST_5: usize = 0x8037_7A5C;
/// `0x80377A70`: the eight-bucket hash table (`0x20` bytes per bucket).
const SB_HASH_BUCKETS: usize = 0x8037_7A70;
/// `0x80377AB0`: the `NW4R` `ut::List` walked by [`fn_80163280`].
const SB_SORTED_LIST: usize = 0x8037_7AB0;
/// `0x80329460`: contiguous descriptor block copied into the five bank lists by
/// [`fn_80162ed0`] (`0x3C` bytes, three header words per list).
const SB_LIST_DESCRIPTORS: usize = 0x8032_9460;
/// `0x803294A0`: descriptor installed at `object + 0x34` by [`fn_80163180`].
const SB_OBJECT_DESCRIPTOR: usize = 0x8032_94A0;
/// `0x80162FF0`: hash-table callback installed by [`fn_80162ed0`].
const FN_SB_HASH_CALLBACK: usize = 0x8016_2FF0;

/// `0x800046B4` - `// inferred:` `memset(dst, value, len)`; the three-argument
/// call in [`fn_801628c0`] matches that shape.
const FN_MEMSET: usize = 0x8000_46B4;
/// `0x80160F60` - unlink one node from a bank list.
const FN_SB_LIST_UNLINK: usize = 0x8016_0F60;
/// `0x80160F10` - insert a node after a `prev` node (0 inserts at the head).
const FN_SB_LIST_INSERT: usize = 0x8016_0F10;
/// `0x80160FE0` - append a node at the list tail.
const FN_SB_LIST_APPEND: usize = 0x8016_0FE0;
/// `0x801616C0` - next node in a bank list.
const FN_SB_LIST_NEXT: usize = 0x8016_16C0;
/// `0x80161700` - end-of-list marker for a bank list.
const FN_SB_LIST_END: usize = 0x8016_1700;
/// `0x80161E40` - per-frame bank-list update pass; `// inferred:` it runs on the
/// bank object it is handed.
const FN_SB_LIST_UPDATE: usize = 0x8016_1E40;
/// `0x802B9430` - EGG heap-object factory `(size, parent, align)`.
const FN_SB_OBJECT_NEW: usize = 0x802B_9430;
/// `0x802AB4F0` - `// inferred:` acquire the bank system before building a view.
const FN_SB_SYSTEM_ACQUIRE: usize = 0x802A_B4F0;
/// `0x80163220` - GX setup for the bank draw pass; it stays untranslated here.
const FN_SB_DRAW_SETUP: usize = 0x8016_3220;
/// `0x802DCB90` - Metrowerks critical-section enter.
const FN_CRITICAL_ENTER: usize = 0x802D_CB90;
/// `0x802DC9A0` - Metrowerks hash-table initialiser `(table, callback, 0, 8, 8)`.
const FN_HASH_INIT: usize = 0x802D_C9A0;

/// Allocates a zeroed block from the game's default heap.
///
/// The allocation runs through the alignment-aware EGG allocator with a `-4`
/// alignment token and [`G_GAME_HEAP_DEFAULT`] as the heap. A non-null result
/// is zeroed with the three-argument [`FN_MEMSET`].
///
/// # Original
/// - Ghidra: `FUN_801628c0` at `0x801628C0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801628c0_FUN_801628c0.c`
/// - observed: `iVar1 = FUN_802b8cc0(param_1,0xfffffffc,DAT_80377c48);
///   if (iVar1 != 0) { FUN_800046b4(iVar1,0,param_1); } return iVar1;`
///
/// # Arguments
/// - `size`: byte count of the block.
///
/// # Returns
/// The zeroed block, or 0 when the allocation failed.
///
/// # Side effects
/// Allocates from the default game heap and writes `size` zero bytes.
///
/// # Safety
/// The default game heap must be initialised.
///
/// # Confidence
/// high - the allocator arguments, the null guard and the `memset` are literal.
pub unsafe fn fn_801628c0(size: u32) -> u32 {
    let block = platform::call3(EGG_ALLOC_ALIGNED, size, 0xFFFF_FFFC, G_GAME_HEAP_DEFAULT as u32);
    if block != 0 {
        platform::call3(FN_MEMSET, block, 0, size);
    }
    block
}

/// Returns one allocation to the game's default heap.
///
/// # Original
/// - Ghidra: `FUN_80162920` at `0x80162920` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162920_FUN_80162920.c`
/// - observed: `FUN_802b8f70(param_1,DAT_80377c48);`
///
/// # Arguments
/// - `ptr`: block previously returned by [`fn_801628c0`].
///
/// # Side effects
/// Frees `ptr` on the default game heap.
///
/// # Safety
/// `ptr` must be a live allocation from the same heap and must not be used
/// again afterwards.
///
/// # Confidence
/// high - a single two-argument free.
pub unsafe fn fn_80162920(ptr: u32) {
    platform::call2(HEAP_FREE_EXPLICIT, ptr, G_GAME_HEAP_DEFAULT as u32);
}

/// Runs the per-frame update and queues a finished bank.
///
/// The update pass runs first. A bank whose `+0x0A`, `+0x0B` and `+0x0C` bytes
/// are all zero is either marked "ready" (`+0x0D = 1`) when the current group
/// is `2`, or appended to [`SB_LIST_2`] through `node + 0x24`.
///
/// # Original
/// - Ghidra: `FUN_80162930` at `0x80162930` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162930_FUN_80162930.c`
/// - observed: `FUN_80161e40(); if ((byte[0xb] == 0) && (byte[0xc] == 0) &&
///   (byte[0xa] == 0)) { if (r13 - 0x6228 == 2) byte[0xd] = 1;
///   else FUN_80160fe0(&DAT_80377a20,param_1 + 0x24); }`
///
/// # Arguments
/// - `this`: bank object.
///
/// # Side effects
/// Runs the update pass, may set `+0x0D`, or links `this + 0x24` into
/// [`SB_LIST_2`].
///
/// # Safety
/// `this` must be a live bank object and its `+0x24` list node must be free.
///
/// # Confidence
/// medium - the three-byte test and both branches are literal; the update
/// call's receiver is not printed by Ghidra and is passed as `this`.
pub unsafe fn fn_80162930(this: usize) {
    platform::call1(FN_SB_LIST_UPDATE, this as u32);
    let queued = platform::read_u8(this + 0x0B) == 0
        && platform::read_u8(this + 0x0C) == 0
        && platform::read_u8(this + 0x0A) == 0;
    if queued {
        if platform::read_u32(G_SB_CURRENT_GROUP) == 2 {
            platform::write_u8(this + 0x0D, 1);
        } else {
            platform::call2(FN_SB_LIST_APPEND, SB_LIST_2 as u32, (this + 0x24) as u32);
        }
    }
}

/// Finds the first bank in `this`'s list whose `+0x0A` byte is zero.
///
/// The scan starts at `this + 0x14` and stops at the end marker returned by
/// [`FN_SB_LIST_END`] on `this + 0x10`; each node's payload is read from
/// `node + 0x10`.
///
/// # Original
/// - Ghidra: `FUN_801629b0` at `0x801629B0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801629b0_FUN_801629b0.c`
/// - observed: `iVar1 = FUN_80161700(param_1 + 0x10); for (iVar2 = *(param_1 +
///   0x14); (iVar2 != 0 && (iVar2 != iVar1)); iVar2 = FUN_801616c0(iVar2)) {
///   if (*(char *)(*(int *)(iVar2 + 0x10) + 10) == 0) return *(iVar2 + 0x10); }`
///
/// # Arguments
/// - `this`: bank object whose list starts at `+0x14`.
///
/// # Returns
/// The first payload with `+0x0A == 0`, or 0.
///
/// # Safety
/// `this` must be a live bank object and every list node must be readable.
///
/// # Confidence
/// medium - the loop and both field offsets are literal; the receiver is a
/// hidden `this` that Ghidra drops at the call sites.
pub unsafe fn fn_801629b0(this: usize) -> u32 {
    let end = platform::call1(FN_SB_LIST_END, (this + 0x10) as u32);
    let mut node = platform::read_u32(this + 0x14);
    while node != 0 && node != end {
        let payload = platform::read_u32(node as usize + 0x10) as usize;
        if platform::read_u8(payload + 0x0A) == 0 {
            return payload as u32;
        }
        node = platform::call1(FN_SB_LIST_NEXT, node);
    }
    0
}

/// Stashes the four bank-construction arguments in the small-data area.
///
/// `fn_80162a70` calls this before and after the factory callback so the
/// callback can read the pending id and its two words plus a flag byte from
/// fixed SDA slots. The trailing call uses `(0, 0, 0, 0)` to clear the stash.
///
/// # Original
/// - Ghidra: `FUN_80162a50` at `0x80162A50` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162a50_FUN_80162a50.c`
/// - observed: `*(r13 - 0x52e4) = param_3; *(r13 - 0x52e8) = param_1;
///   *(r13 - 0x52e0) = param_4; *(r13 - 0x52dc) = param_2;`
///
/// # Arguments
/// - `id`: `u16` bank id.
/// - `arg_b`: second word (pointer or count at the call sites).
/// - `arg_c`: third word.
/// - `flag`: flag byte.
///
/// # Side effects
/// Writes the four SDA slots above.
///
/// # Confidence
/// high - four stores with literal offsets.
pub unsafe fn fn_80162a50(id: u16, arg_b: u32, arg_c: u32, flag: u8) {
    platform::write_u32(G_SB_ARG_C, arg_c);
    platform::write_u16(G_SB_ARG_ID, id);
    platform::write_u8(G_SB_ARG_FLAG, flag);
    platform::write_u32(G_SB_ARG_B, arg_b);
}

/// Inserts a node into a bank list, keeping the `u16` key at `+0x0C` ascending.
///
/// An empty list takes the node through the tail-append helper; otherwise the
/// node goes before the first entry with a greater key, or after the last entry
/// with a key less than or equal to it.
///
/// # Original
/// - Ghidra: `FUN_80162b40` at `0x80162B40` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162b40_FUN_80162b40.c`
/// - observed: `iVar2 = *param_1; if (param_2 == 0) return 0; if (iVar2 == 0)
///   { uVar1 = FUN_80160fe0(); return uVar1; } if (*(ushort *)(param_2 + 0xc) <
///   *(ushort *)(iVar2 + 0xc)) { uVar1 = FUN_80160f10(param_1,param_2,0);
///   return uVar1; } do { iVar2 = *(iVar2 + 4); if (iVar2 == 0) break; }
///   while (*(ushort *)(iVar2 + 0xc) <= *(ushort *)(param_2 + 0xc));
///   uVar1 = FUN_80160f10(); return uVar1;`
///
/// # Arguments
/// - `list`: list header (head at `+0x00`).
/// - `node`: node to insert (key at `+0x0C`).
///
/// # Returns
/// 1 when inserted, 0 for a null node.
///
/// # Side effects
/// Relinks the list.
///
/// # Safety
/// `list` must be a live header and `node` must not already be linked.
///
/// # Confidence
/// medium - the ordering test and both helper calls are literal; Ghidra drops
/// the arguments of the two helpers on the branch edges, and the predecessor
/// (`iVar2`) is the inferred third argument of the final insert.
pub unsafe fn fn_80162b40(list: usize, node: usize) -> u32 {
    let mut cursor = platform::read_u32(list) as usize;
    if node == 0 {
        return 0;
    }
    if cursor == 0 {
        return platform::call2(FN_SB_LIST_APPEND, list as u32, node as u32);
    }
    if platform::read_u16(node + 0x0C) < platform::read_u16(cursor + 0x0C) {
        return platform::call3(FN_SB_LIST_INSERT, list as u32, node as u32, 0);
    }
    loop {
        cursor = platform::read_u32(cursor + 4) as usize;
        if cursor == 0 {
            break;
        }
        if platform::read_u16(node + 0x0C) < platform::read_u16(cursor + 0x0C) {
            break;
        }
    }
    platform::call3(FN_SB_LIST_INSERT, list as u32, node as u32, cursor as u32)
}

/// Frees every payload of one hash-bucket list.
///
/// The critical section is entered on `list + 8`, then each node's `+0x08`
/// word is released and the list is walked through `node + 0x04`.
///
/// # Original
/// - Ghidra: `FUN_80162bb0` at `0x80162BB0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162bb0_FUN_80162bb0.c`
/// - observed: `iVar1 = FUN_802dcb90(param_1 + 2); if (iVar1 != 0) { iVar1 =
///   *param_1; while (iVar1 != 0) { iVar2 = *(iVar1 + 4);
///   FUN_802dcbfc(*(iVar1 + 8)); iVar1 = iVar2; } } return 1;`
///
/// # Arguments
/// - `list`: bucket header (head at `+0x00`, lock word at `+0x08`).
///
/// # Returns
/// `1`.
///
/// # Side effects
/// Frees each node payload and leaves the nodes themselves linked.
///
/// # Safety
/// Every payload word must be a live allocation from the matching heap.
///
/// # Confidence
/// high - the enter guard, the walk and the free are literal.
pub unsafe fn fn_80162bb0(list: usize) -> u32 {
    if platform::call1(FN_CRITICAL_ENTER, (list + 8) as u32) != 0 {
        let mut node = platform::read_u32(list);
        while node != 0 {
            let next = platform::read_u32(node as usize + 4);
            interface_call_desc(Descriptor::new(list + 8), platform::read_u32(node as usize + 8));
            node = next;
        }
    }
    1
}

/// Finds a hash-bucket node whose descriptor's first word equals `key`.
///
/// # Original
/// - Ghidra: `FUN_80162c30` at `0x80162C30` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162c30_FUN_80162c30.c`
/// - observed: `iVar1 = *param_1; while( true ) { if (iVar1 == 0) return 0;
///   if (**(int **)(iVar1 + 8) == param_2) break; iVar1 = *(iVar1 + 4); }
///   return iVar1;`
///
/// # Arguments
/// - `bucket`: bucket header (head at `+0x00`).
/// - `key`: word compared against `*(*(node + 8))`.
///
/// # Returns
/// The matching node, or 0.
///
/// # Confidence
/// high - the loop, the double indirection and both offsets are literal.
pub unsafe fn fn_80162c30(bucket: usize, key: u32) -> u32 {
    let mut node = platform::read_u32(bucket);
    while node != 0 {
        let descriptor = platform::read_u32(node as usize + 0x08) as usize;
        if platform::read_u32(descriptor) == key {
            return node;
        }
        node = platform::read_u32(node as usize + 0x04);
    }
    0
}

/// Unlinks a bank object from its list and clears the owner pointer.
///
/// # Original
/// - Ghidra: `FUN_80162ca0` at `0x80162CA0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162ca0_FUN_80162ca0.c`
/// - observed: `if (*(int *)(param_1 + 8) != 0) {
///   FUN_80160f60(*(int *)(param_1 + 8) + 0x54,param_1);
///   *(undefined4 *)(param_1 + 8) = 0; }`
///
/// # Arguments
/// - `this`: bank object holding its list node pointer at `+0x08`.
///
/// # Side effects
/// Unlinks the node at `owner + 0x54` and zeroes `+0x08`.
///
/// # Safety
/// A non-null owner must be a live list and `this` must be its member node.
///
/// # Confidence
/// high - guard, unlink and clear are literal.
pub unsafe fn fn_80162ca0(this: usize) {
    let owner = platform::read_u32(this + 0x08);
    if owner != 0 {
        platform::call2(FN_SB_LIST_UNLINK, owner + 0x54, this as u32);
        platform::write_u32(this + 0x08, 0);
    }
}

/// Returns the low three bits of the word behind `this + 0x3C`.
///
/// # Original
/// - Ghidra: `FUN_80162cf0` at `0x80162CF0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162cf0_FUN_80162cf0.c`
/// - observed: `return **(uint **)(param_1 + 0x3c) & 7;`
///
/// # Returns
/// A value in `0..=7`.
///
/// # Confidence
/// high - one double indirection and a literal mask.
pub unsafe fn fn_80162cf0(this: usize) -> u32 {
    platform::read_u32(platform::read_u32(this + 0x3C) as usize) & 7
}

/// Looks a bank id up in the eight-bucket hash table.
///
/// The bucket is `(id & 7) * 0x20` bytes into [`SB_HASH_BUCKETS`]; a hit returns
/// the node's `+0x08` descriptor payload word.
///
/// # Original
/// - Ghidra: `FUN_80162d00` at `0x80162D00` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162d00_FUN_80162d00.c`
/// - observed: `iVar1 = FUN_80162c30(&DAT_80377a70 + (param_1 & 7) * 8,param_1);
///   if (iVar1 == 0) uVar2 = 0; else uVar2 = *(undefined4 *)(iVar1 + 8);`
///
/// # Arguments
/// - `id`: bank id; the low three bits select the bucket.
///
/// # Returns
/// The descriptor word, or 0.
///
/// # Confidence
/// medium - the bucket selection and payload read are literal; the `* 8`
/// pointer scale is Ghidra's `undefined4` arithmetic, i.e. `0x20` bytes.
pub unsafe fn fn_80162d00(id: u32) -> u32 {
    let bucket = SB_HASH_BUCKETS + (id & 7) as usize * 0x20;
    let node = fn_80162c30(bucket, id);
    if node == 0 {
        0
    } else {
        platform::read_u32(node as usize + 0x08)
    }
}

/// Looks a `u16` key up in [`SB_LIST_1`] and returns the node payload.
///
/// A non-null `node` starts the search after that node instead of at the head.
///
/// # Original
/// - Ghidra: `FUN_80162d50` at `0x80162D50` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162d50_FUN_80162d50.c`
/// - observed: `if (param_2 == 0) param_2 = 0; else param_2 = param_2 + 0x10;
///   iVar1 = FUN_80163090(&DAT_80377a10,param_1,param_2); if (iVar1 == 0)
///   uVar2 = 0; else uVar2 = *(undefined4 *)(iVar1 + 0x10);`
///
/// # Arguments
/// - `key`: `u16` key.
/// - `node`: optional start node.
///
/// # Returns
/// The payload word, or 0.
///
/// # Confidence
/// high - both paths, the helper call and the payload read are literal.
pub unsafe fn fn_80162d50(key: u32, node: u32) -> u32 {
    let start = if node == 0 { 0 } else { node + 0x10 };
    let found = fn_80163090(SB_LIST_1, key, start);
    if found == 0 {
        0
    } else {
        platform::read_u32(found as usize + 0x10)
    }
}

/// Looks a `u8` sub-key up in [`SB_LIST_1`] and returns the node payload.
///
/// # Original
/// - Ghidra: `FUN_80162db0` at `0x80162DB0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162db0_FUN_80162db0.c`
/// - observed: `if (param_2 == 0) param_2 = 0; else param_2 = param_2 + 0x10;
///   iVar1 = FUN_80163100(&DAT_80377a10,param_1,param_2); if (iVar1 == 0)
///   uVar2 = 0; else uVar2 = *(undefined4 *)(iVar1 + 0x10);`
///
/// # Arguments
/// - `key`: `u8` sub-key.
/// - `node`: optional start node.
///
/// # Returns
/// The payload word, or 0.
///
/// # Confidence
/// high - mirror of [`fn_80162d50`] with the byte-key search.
pub unsafe fn fn_80162db0(key: u32, node: u32) -> u32 {
    let start = if node == 0 { 0 } else { node + 0x10 };
    let found = fn_80163100(SB_LIST_1, key, start);
    if found == 0 {
        0
    } else {
        platform::read_u32(found as usize + 0x10)
    }
}

/// Frees the payloads of all five bank lists according to the teardown mask.
///
/// A clear bit in [`G_SB_TEARDOWN_FLAGS`] means the group is still live, so its
/// list is freed while [`G_SB_CURRENT_GROUP`] is set to that group's id. The
/// last group uses [`fn_80163000`]; the other four use [`fn_80162bb0`]. The
/// group id is cleared to 0 at the end.
///
/// # Original
/// - Ghidra: `FUN_80162e10` at `0x80162E10` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162e10_FUN_80162e10.c`
/// - observed: `if ((flags & 8) == 0) { r13 - 0x6228 = 4;
///   FUN_80162bb0(&DAT_80377a5c); } if ((flags & 2) == 0) { ... = 2;
///   FUN_80162bb0(&DAT_80377a20); } if ((flags & 4) == 0) { ... = 3;
///   FUN_80162bb0(&DAT_80377a34); } if ((flags & 0x10) == 0) { ... = 5;
///   FUN_80162bb0(&DAT_80377a48); } if ((flags & 1) == 0) { ... = 1;
///   FUN_80163000(&DAT_80377a10); } r13 - 0x6228 = 0;`
///
/// # Side effects
/// Releases the selected lists and rewrites [`G_SB_CURRENT_GROUP`].
///
/// # Confidence
/// high - five independent bit tests and their literal list addresses.
pub unsafe fn fn_80162e10() {
    let flags = platform::read_u32(G_SB_TEARDOWN_FLAGS);
    if flags & 0x08 == 0 {
        platform::write_u32(G_SB_CURRENT_GROUP, 4);
        fn_80162bb0(SB_LIST_5);
    }
    if flags & 0x02 == 0 {
        platform::write_u32(G_SB_CURRENT_GROUP, 2);
        fn_80162bb0(SB_LIST_2);
    }
    if flags & 0x04 == 0 {
        platform::write_u32(G_SB_CURRENT_GROUP, 3);
        fn_80162bb0(SB_LIST_3);
    }
    if flags & 0x10 == 0 {
        platform::write_u32(G_SB_CURRENT_GROUP, 5);
        fn_80162bb0(SB_LIST_4);
    }
    if flags & 0x01 == 0 {
        platform::write_u32(G_SB_CURRENT_GROUP, 1);
        fn_80163000(SB_LIST_1);
    }
    platform::write_u32(G_SB_CURRENT_GROUP, 0);
}

/// Resets the five bank lists and the hash table.
///
/// Each list gets a zero head and three header words copied from the contiguous
/// descriptor block [`SB_LIST_DESCRIPTORS`]. The hash table is then initialised
/// with eight buckets and eight entries per bucket through [`FN_HASH_INIT`].
///
/// # Original
/// - Ghidra: `FUN_80162ed0` at `0x80162ED0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162ed0_FUN_80162ed0.c`
/// - Removed: the modelled `0x802DCD44`/`0x802DCD90` "lock"/"unlock" calls.
///   Those addresses are the `_savegpr0_15` and `_restgpr0_15` fragments of the
///   compiler's register-save table (the `bl` at `0x80162EE0` and the `bl` at
///   `0x80162FDC`); they only spill and restore `r15..r31`.
/// - observed: `FUN_802dcd44(); DAT_80377a10 = 0; DAT_80377a14 =
///   DAT_80329460; ... DAT_80377a6c = PTR_FUN_80329498;
///   FUN_802dc9a0(&DAT_80377a70,&LAB_80162ff0,0,8,8); FUN_802dcd90();`
///
/// # Side effects
/// Overwrites `0x80377A10..0x80377A6C` and reinitialises the hash table.
///
/// # Confidence
/// high - every store and the hash initialiser are literal.
pub unsafe fn fn_80162ed0() {
    platform::write_u32(SB_LIST_1, 0);
    platform::write_u32(SB_LIST_1 + 0x04, platform::read_u32(SB_LIST_DESCRIPTORS));
    platform::write_u32(SB_LIST_1 + 0x08, platform::read_u32(SB_LIST_DESCRIPTORS + 0x04));
    platform::write_u32(SB_LIST_1 + 0x0C, platform::read_u32(SB_LIST_DESCRIPTORS + 0x08));
    platform::write_u32(SB_LIST_2, 0);
    platform::write_u32(SB_LIST_2 + 0x04, 0);
    platform::write_u32(SB_LIST_2 + 0x08, platform::read_u32(SB_LIST_DESCRIPTORS + 0x0C));
    platform::write_u32(SB_LIST_2 + 0x0C, platform::read_u32(SB_LIST_DESCRIPTORS + 0x10));
    platform::write_u32(SB_LIST_2 + 0x10, platform::read_u32(SB_LIST_DESCRIPTORS + 0x14));
    platform::write_u32(SB_LIST_3, 0);
    platform::write_u32(SB_LIST_3 + 0x04, 0);
    platform::write_u32(SB_LIST_3 + 0x08, platform::read_u32(SB_LIST_DESCRIPTORS + 0x18));
    platform::write_u32(SB_LIST_3 + 0x0C, platform::read_u32(SB_LIST_DESCRIPTORS + 0x1C));
    platform::write_u32(SB_LIST_3 + 0x10, platform::read_u32(SB_LIST_DESCRIPTORS + 0x20));
    platform::write_u32(SB_LIST_4, 0);
    platform::write_u32(SB_LIST_4 + 0x04, 0);
    platform::write_u32(SB_LIST_4 + 0x08, platform::read_u32(SB_LIST_DESCRIPTORS + 0x24));
    platform::write_u32(SB_LIST_4 + 0x0C, platform::read_u32(SB_LIST_DESCRIPTORS + 0x28));
    platform::write_u32(SB_LIST_4 + 0x10, platform::read_u32(SB_LIST_DESCRIPTORS + 0x2C));
    platform::write_u32(SB_LIST_5, 0);
    platform::write_u32(SB_LIST_5 + 0x04, 0);
    platform::write_u32(SB_LIST_5 + 0x08, platform::read_u32(SB_LIST_DESCRIPTORS + 0x30));
    platform::write_u32(SB_LIST_5 + 0x0C, platform::read_u32(SB_LIST_DESCRIPTORS + 0x34));
    platform::write_u32(SB_LIST_5 + 0x10, platform::read_u32(SB_LIST_DESCRIPTORS + 0x38));
    platform::call5(
        FN_HASH_INIT,
        SB_HASH_BUCKETS as u32,
        FN_SB_HASH_CALLBACK as u32,
        0,
        8,
        8,
    );
}

/// Releases the payloads of a bank list.
///
/// The critical section is entered on `list + 4`; the list is then walked with
/// [`FN_SB_LIST_NEXT`] and each node's `+0x10` payload is freed.
///
/// # Original
/// - Ghidra: `FUN_80163000` at `0x80163000` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163000_FUN_80163000.c`
/// - observed: `iVar1 = FUN_802dcb90(param_1 + 1); if (iVar1 != 0) { iVar1 =
///   *param_1; while (iVar1 != 0) { iVar2 = FUN_801616c0(iVar1);
///   FUN_802dcbfc(*(iVar1 + 0x10)); iVar1 = iVar2; } } return 1;`
///
/// # Arguments
/// - `list`: bank list header (head at `+0x00`, lock word at `+0x04`).
///
/// # Returns
/// `1`.
///
/// # Side effects
/// Frees every node payload.
///
/// # Safety
/// Every payload word must be a live allocation.
///
/// # Confidence
/// high - the enter guard, the walk and the free are literal.
pub unsafe fn fn_80163000(list: usize) -> u32 {
    if platform::call1(FN_CRITICAL_ENTER, (list + 4) as u32) != 0 {
        let mut node = platform::read_u32(list);
        while node != 0 {
            let next = platform::call1(FN_SB_LIST_NEXT, node);
            interface_call_desc(Descriptor::new(list + 4), platform::read_u32(node as usize + 0x10));
            node = next;
        }
    }
    1
}

/// Finds a bank node by the `u16` key at `payload + 0x08`.
///
/// # Original
/// - Ghidra: `FUN_80163090` at `0x80163090` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163090_FUN_80163090.c`
/// - observed: `if (param_3 == 0) iVar1 = *param_1; else iVar1 =
///   FUN_801616c0(param_3); while( true ) { if (iVar1 == 0) return 0;
///   if (*(ushort *)(*(int *)(iVar1 + 0x10) + 8) == param_2) break;
///   iVar1 = FUN_801616c0(); } return iVar1;`
///
/// # Arguments
/// - `list`: list header.
/// - `key`: `u16` key to match.
/// - `start`: node to start after, or 0 for the head.
///
/// # Returns
/// The matching node, or 0.
///
/// # Confidence
/// high - the start selection, the key compare and the walk are literal.
pub unsafe fn fn_80163090(list: usize, key: u32, start: u32) -> u32 {
    let mut node = if start == 0 {
        platform::read_u32(list)
    } else {
        platform::call1(FN_SB_LIST_NEXT, start)
    };
    loop {
        if node == 0 {
            return 0;
        }
        let payload = platform::read_u32(node as usize + 0x10) as usize;
        if platform::read_u16(payload + 0x08) == key as u16 {
            return node;
        }
        node = platform::call1(FN_SB_LIST_NEXT, node);
    }
}

/// Finds a bank node by the `u8` sub-key at `payload + 0x0E`.
///
/// # Original
/// - Ghidra: `FUN_80163100` at `0x80163100` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163100_FUN_80163100.c`
/// - observed: `if (param_3 == 0) iVar1 = *param_1; else iVar1 =
///   FUN_801616c0(param_3); while( true ) { if (iVar1 == 0) return 0;
///   if (*(byte *)(*(int *)(iVar1 + 0x10) + 0xe) == param_2) break;
///   iVar1 = FUN_801616c0(); } return iVar1;`
///
/// # Arguments
/// - `list`: list header.
/// - `key`: `u8` sub-key to match.
/// - `start`: node to start after, or 0 for the head.
///
/// # Returns
/// The matching node, or 0.
///
/// # Confidence
/// high - mirror of [`fn_80163090`] with the byte key.
pub unsafe fn fn_80163100(list: usize, key: u32, start: u32) -> u32 {
    let mut node = if start == 0 {
        platform::read_u32(list)
    } else {
        platform::call1(FN_SB_LIST_NEXT, start)
    };
    loop {
        if node == 0 {
            return 0;
        }
        let payload = platform::read_u32(node as usize + 0x10) as usize;
        if platform::read_u8(payload + 0x0E) == key as u8 {
            return node;
        }
        node = platform::call1(FN_SB_LIST_NEXT, node);
    }
}

/// Reinitialises the sorted `NW4R` list used by [`fn_80163280`].
///
/// # Original
/// - Ghidra: `FUN_80163170` at `0x80163170` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163170_FUN_80163170.c`
/// - observed: `FUN_80228dd0(&DAT_80377ab0,0);`
///
/// # Side effects
/// Resets [`SB_SORTED_LIST`] to empty.
///
/// # Confidence
/// high - one two-argument call with literal arguments.
pub unsafe fn fn_80163170() {
    platform::call2(FN_UT_LIST_INIT, SB_SORTED_LIST as u32, 0);
}

/// Builds the resource view that the bank layer reads its archives through.
///
/// An EGG object is created with `(param_2, param_1, 4)` and stamped with
/// [`SB_OBJECT_DESCRIPTOR`] at `+0x34`; a `0x1C`-byte LZ view is then allocated
/// through [`fn_8016a5e0`] (wave 39) and the same object is attached as its heap
/// with [`fn_8016a690`] and user word `4`. The view is published in
/// [`G_SB_STREAM_VIEW`] and [`G_SB_STREAM_VIEW_CONSUMER`] before the sorted list
/// is reset. Always returns `1`.
///
/// # Original
/// - Ghidra: `FUN_80163180` at `0x80163180` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163180_FUN_80163180.c`
/// - observed: `FUN_802ab4f0(); iVar1 = FUN_802b9430(param_2,param_1,4);
///   *(iVar1 + 0x34) = &DAT_803294a0; iVar2 = FUN_802b9240(0x1c,iVar1,4);
///   uVar3 = 0; if (iVar2 != 0) uVar3 = FUN_8016a5e0();
///   *(r13 - 0x52c0) = uVar3; FUN_8016a690(uVar3,iVar1,4);
///   *(r13 - 0x48b8) = *(r13 - 0x52c0); FUN_80163170(); return 1;`
///
/// # Arguments
/// - `param_1`: first word of the EGG object block.
/// - `param_2`: size passed to the object factory.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// Allocates two objects, constructs the LZ view, attaches the heap and writes
/// both SDA publication slots.
///
/// # Safety
/// The archive heap layer behind `0x802B9430`/`0x802B9240` must be initialised;
/// the factory's object must be at least `0x38` bytes.
///
/// # Confidence
/// medium - the allocation chain, the descriptor store and both SDA writes are
/// literal; Ghidra drops the receiver of the `0x8016A5E0` call, which is the
/// freshly allocated `0x1C`-byte block, as wave 39's constructor requires.
pub unsafe fn fn_80163180(param_1: u32, param_2: u32) -> u32 {
    platform::call0(FN_SB_SYSTEM_ACQUIRE);
    let object = platform::call3(FN_SB_OBJECT_NEW, param_2, param_1, 4) as usize;
    platform::write_u32(object + 0x34, SB_OBJECT_DESCRIPTOR as u32);
    let allocation = platform::call3(MOUNT_OBJECT_ALLOC, 0x1C, object as u32, 4);
    let view = if allocation == 0 {
        0
    } else {
        fn_8016a5e0(allocation as usize) as u32
    };
    platform::write_u32(G_SB_STREAM_VIEW, view);
    fn_8016a690(view as usize, object, 4);
    platform::write_u32(G_SB_STREAM_VIEW_CONSUMER, view);
    fn_80163170();
    1
}

/// Runs the bank-list draw pass and rebuilds the sorted list.
///
/// After the draw setup at `0x80163220`, every node of [`SB_SORTED_LIST`] is
/// visited through [`FN_UT_LIST_NEXT`] and its `+0x08` vtable slot `+0x0C` runs
/// with the node as receiver. The list is reset afterwards.
///
/// # Original
/// - Ghidra: `FUN_80163280` at `0x80163280` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163280_FUN_80163280.c`
/// - observed: `FUN_80163220(); for (iVar1 = FUN_80228ff0(&DAT_80377ab0,0);
///   iVar1 != 0; iVar1 = FUN_80228ff0(&DAT_80377ab0,iVar1)) {
///   (**(code **)(*(int *)(iVar1 + 8) + 0xc))(iVar1); } FUN_80163170();`
///
/// # Side effects
/// Runs one virtual call per node, then reinitialises [`SB_SORTED_LIST`].
///
/// # Safety
/// Every node's vtable word must be live and its `+0x0C` slot callable.
///
/// # Confidence
/// medium - the iteration and the virtual call are literal; the draw setup at
/// `0x80163220` is GX state and stays untranslated, and the `+0x0C` slot's
/// receiver is inferred from r3.
pub unsafe fn fn_80163280() {
    platform::call0(FN_SB_DRAW_SETUP);
    let mut node = platform::call2(FN_UT_LIST_NEXT, SB_SORTED_LIST as u32, 0);
    while node != 0 {
        let vtable = platform::read_u32(node as usize + 0x08) as usize;
        platform::call1(platform::read_u32(vtable + 0x0C) as usize, node);
        node = platform::call2(FN_UT_LIST_NEXT, SB_SORTED_LIST as u32, node);
    }
    fn_80163170();
}

/// Returns the LZ view published by [`fn_80163180`].
///
/// # Original
/// - Ghidra: `FUN_801634c0` at `0x801634C0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801634c0_FUN_801634c0.c`
/// - observed: `return *(undefined4 *)(r13 + -0x52c0);`
///
/// # Returns
/// The view address, or 0 before [`fn_80163180`] runs.
///
/// # Confidence
/// high - one SDA load.
pub unsafe fn fn_801634c0() -> u32 {
    platform::read_u32(G_SB_STREAM_VIEW)
}

// ===========================================================================
// 0x38-stride element accessors (wave 40)
//
// The container at `0x80166280..0x80166360` indexes a `0x38`-stride element
// array with its base at `+0x28`. Each element keeps its native object pointer
// at `+0x04`; the five wrappers below forward one fixed operation to that
// object's vtable. The container itself is a layout object owned elsewhere.
// ===========================================================================

/// `0x80249C90` - read the word at `object + 4`; see [`FN_ELEMENT_QUERY`].
const FN_ELEMENT_CLASS_TAG: usize = 0x8024_9C90;
/// Element field: the native object the wrappers dispatch to.
const ELEMENT_OBJECT: usize = 0x04;

/// Returns the element's class tag through `0x80249C90(object, 4)`.
///
/// # Original
/// - Ghidra: `FUN_80168060` at `0x80168060` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168060_FUN_80168060.c`
/// - observed: `if (*(int *)(param_1 + 4) == 0) return 0;
///   uVar1 = FUN_80249c90(*(int *)(param_1 + 4),4); return uVar1;`
///
/// # Arguments
/// - `element`: one `0x38`-byte array element.
///
/// # Returns
/// The tag word, or 0 when the element has no object.
///
/// # Confidence
/// high - the null guard, the object field and both call arguments are literal.
pub unsafe fn fn_80168060(element: usize) -> u32 {
    let object = platform::read_u32(element + ELEMENT_OBJECT);
    if object == 0 {
        return 0;
    }
    platform::call2(FN_ELEMENT_CLASS_TAG, object, 4)
}

/// Dispatches the element object's vtable slot `+0x20`.
///
/// # Original
/// - Ghidra: `FUN_80168080` at `0x80168080` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168080_FUN_80168080.c`
/// - observed: `(**(code **)(**(int **)(param_1 + 4) + 0x20))();`
///
/// # Arguments
/// - `element`: one `0x38`-byte array element.
///
/// # Side effects
/// Runs the object's `+0x20` slot.
///
/// # Safety
/// `element + 4` must hold a live object whose vtable has a callable `+0x20`.
///
/// # Confidence
/// medium - the slot and the receiver load are literal; Ghidra does not print
/// the arguments of the virtual call.
pub unsafe fn fn_80168080(element: usize) {
    let object = platform::read_u32(element + ELEMENT_OBJECT) as usize;
    let vtable = platform::read_u32(object) as usize;
    platform::call1(platform::read_u32(vtable + 0x20) as usize, object as u32);
}

/// Dispatches the element object's vtable slot `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_801680a0` at `0x801680A0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801680a0_FUN_801680a0.c`
/// - observed: `(**(code **)(**(int **)(param_1 + 4) + 0x1c))();`
///
/// # Arguments
/// - `element`: one `0x38`-byte array element.
///
/// # Side effects
/// Runs the object's `+0x1C` slot.
///
/// # Safety
/// `element + 4` must hold a live object whose vtable has a callable `+0x1C`.
///
/// # Confidence
/// medium - the slot and the receiver load are literal; the virtual call's
/// argument registers are not recovered.
pub unsafe fn fn_801680a0(element: usize) {
    let object = platform::read_u32(element + ELEMENT_OBJECT) as usize;
    let vtable = platform::read_u32(object) as usize;
    platform::call1(platform::read_u32(vtable + 0x1C) as usize, object as u32);
}

/// Dispatches the element object's vtable slot `+0x2C`.
///
/// # Original
/// - Ghidra: `FUN_801680c0` at `0x801680C0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801680c0_FUN_801680c0.c`
/// - observed: `(**(code **)(**(int **)(param_1 + 4) + 0x2c))();`
///
/// # Arguments
/// - `element`: one `0x38`-byte array element.
///
/// # Side effects
/// Runs the object's `+0x2C` slot.
///
/// # Safety
/// `element + 4` must hold a live object whose vtable has a callable `+0x2C`.
///
/// # Confidence
/// medium - the slot and the receiver load are literal; the virtual call's
/// argument registers are not recovered.
pub unsafe fn fn_801680c0(element: usize) {
    let object = platform::read_u32(element + ELEMENT_OBJECT) as usize;
    let vtable = platform::read_u32(object) as usize;
    platform::call1(platform::read_u32(vtable + 0x2C) as usize, object as u32);
}

/// Dispatches the element object's vtable slot `+0x28`.
///
/// # Original
/// - Ghidra: `FUN_801680e0` at `0x801680E0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801680e0_FUN_801680e0.c`
/// - observed: `(**(code **)(**(int **)(param_1 + 4) + 0x28))();`
///
/// # Arguments
/// - `element`: one `0x38`-byte array element.
///
/// # Side effects
/// Runs the object's `+0x28` slot.
///
/// # Safety
/// `element + 4` must hold a live object whose vtable has a callable `+0x28`.
///
/// # Confidence
/// medium - the slot and the receiver load are literal; the virtual call's
/// argument registers are not recovered.
pub unsafe fn fn_801680e0(element: usize) {
    let object = platform::read_u32(element + ELEMENT_OBJECT) as usize;
    let vtable = platform::read_u32(object) as usize;
    platform::call1(platform::read_u32(vtable + 0x28) as usize, object as u32);
}

// ===========================================================================
// 0x80160000 bucket: handle, list, context and element dispatch (wave 41)
// ===========================================================================

/// `0x80328AE8`: vtable installed at `handle + 0x0C` by [`fn_80160080`].
const HANDLE_VTABLE: usize = 0x8032_8AE8;

/// `0x8015FEF0` - base-class constructor run first by [`fn_80160080`].
const FN_HANDLE_BASE_CTOR: usize = 0x8015_FEF0;

/// `0x8015FE50` - base-class destructor run by [`fn_80160110`] and
/// [`fn_80160930`].
const FN_HANDLE_BASE_DTOR: usize = 0x8015_FE50;

/// `0x80160930` - close a lock object, then optionally free it.
const FN_HANDLE_LOCK_CLOSE: usize = 0x8016_0930;

/// `0x801609D0` - release the lock word installed by [`fn_80160990`].
const FN_HANDLE_LOCK_RELEASE: usize = 0x8016_09D0;

/// `0x802B7B00` - construct the block returned by the handle's `+0x14` slot.
///
/// `// inferred:` the routine installs `&DAT_8034FBE0` at `+0x00` and clears
/// `+0x04`; the caller passes the freshly allocated block.
const FN_HANDLE_BLOCK_CTOR: usize = 0x802B_7B00;

/// `0x802B9220` - identity helper (`return second argument`); used as a
/// placement hook by [`fn_80160190`].
const FN_HANDLE_PLACEMENT: usize = 0x802B_9220;

/// `0x80235710` - create the lock word used by the handle pair.
const FN_HANDLE_LOCK_NEW: usize = 0x8023_5710;

/// `0x802357A0` - destroy the lock word used by the handle pair.
const FN_HANDLE_LOCK_FREE: usize = 0x8023_57A0;

/// `r13 - 0x52FC` -> published handle object.
const G_HANDLE_SELF: usize = SDA_BASE - 0x52FC;
/// `r13 - 0x5304` -> buffer allocated by [`fn_80160190`].
const G_HANDLE_BUFFER: usize = SDA_BASE - 0x5304;
/// `r13 - 0x5308` -> remaining size of that buffer.
const G_HANDLE_REMAIN: usize = SDA_BASE - 0x5308;
/// `r13 - 0x52F8` -> smallest value reported by vtable slot `+0x28`.
const G_HANDLE_MIN_A: usize = SDA_BASE - 0x52F8;
/// `r13 - 0x52F4` -> smallest value reported by vtable slot `+0x24`.
const G_HANDLE_MIN_B: usize = SDA_BASE - 0x52F4;

/// `r13 - 0x52B4` -> live context object (vtable-bearing manager).
const G_CTX_MAIN: usize = SDA_BASE - 0x52B4;
/// `r13 - 0x52AC` -> array of per-index objects.
const G_CTX_SLOTS: usize = SDA_BASE - 0x52AC;
/// `r13 - 0x52A4` -> array of per-index auxiliary flag objects.
const G_CTX_AUX_SLOTS: usize = SDA_BASE - 0x52A4;

/// `0x80258BD0` - read one context state by id.
const FN_CTX_STATE_GET_BY_ID: usize = 0x8025_8BD0;
/// `0x80258C30` - read the current context state.
const FN_CTX_STATE_GET_CURRENT: usize = 0x8025_8C30;
/// `0x802590A0` - close the context (`vtable + 0x0C(1, ...)`).
const FN_CTX_CLOSE: usize = 0x8025_90A0;
/// `0x802590F0` - request context teardown (`vtable + 0x0C(2, ...)`).
const FN_CTX_TEARDOWN: usize = 0x8025_90F0;
/// `0x802C6640` - set the "aux closed" bit (bit 2) on one aux object.
const FN_CTX_AUX_MARK: usize = 0x802C_6640;

/// `0x802CDD00` - fill a 16-byte local key from `*(param + 4)`.
///
/// `// inferred:` the callee copies/key-initialises the stack buffer before the
/// map lookup; its exact key encoding is outside this lane.
const FN_CTX_KEY_INIT: usize = 0x802C_DD00;
/// `0x802CD2D0` - map lookup by key.
const FN_CTX_KEY_FIND: usize = 0x802C_D2D0;
/// `0x802CD1F0` - map lookup/store using the prepared 16-byte key.
const FN_CTX_KEY_STORE: usize = 0x802C_D1F0;

/// `0x80168FE0` - set an element's `+0x30` float through vtable slot `+0x1C`.
const FN_ELEMENT_VALUE_SET: usize = 0x8016_8FE0;
/// `0x80169020` - element range test against its `+0x2C` limit.
const FN_ELEMENT_TEST: usize = 0x8016_9020;

/// `element` -> owner object at `+0x28` (the base of the 0x38-byte array).
const ELEMENT_OWNER: usize = 0x28;
/// Distance between two 0x38-byte array elements.
const ELEMENT_STRIDE: usize = 0x38;

/// Probes a handle: if its first half-word is zero, dispatch vtable slot `+0x20`.
///
/// # Original
/// - Ghidra: `FUN_80160020` at `0x80160020` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160020_FUN_80160020.c`
/// - observed: `if (*param_1 == 0) return (**(code **)(*(int *)(param_1 + 6) + 0x20))(); return 1;`
///
/// # Arguments
/// - `this`: handle object. Its first `u16` is a state flag and the word at
///   `+0x0C` is the object whose vtable is dispatched.
///
/// # Returns
/// The virtual slot's result when the flag is zero, otherwise `1`.
///
/// # Side effects
/// May run an external virtual method with observable effects.
///
/// # Safety
/// `this` must point at a live handle and `this + 0x0C` must hold a live
/// object whose `+0x20` slot is callable.
///
/// # Confidence
/// medium - the guard, offset and slot are literal; Ghidra prints the virtual
/// call with no argument list, so the receiver is inferred as `this`.
pub unsafe fn fn_80160020(this: usize) -> u32 {
    if platform::read_u16(this) == 0 {
        let object = platform::read_u32(this + 0x0C) as usize;
        let vtable = platform::read_u32(object) as usize;
        return platform::call1(platform::read_u32(vtable + 0x20) as usize, this as u32);
    }
    1
}

/// Constructs the 0x48-byte handle around a caller-supplied owner and value.
///
/// # Original
/// - Ghidra: `FUN_80160080` at `0x80160080` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160080_FUN_80160080.c`
/// - observed: `FUN_8015fef0(); vtable = &DAT_80328ae8; +0x20 = param_2;
///   +0x24 = 0; +0x25 = 0; +0x26 = 0; +0x30 = param_3;`
///
/// # Arguments
/// - `this`: storage for the handle; the caller allocates 0x48 bytes.
/// - `owner`: object stored at `+0x20`.
/// - `value`: word stored at `+0x30`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the base constructor, installs [`HANDLE_VTABLE`] at `+0x0C`, and
/// clears `+0x14`, `+0x18`, `+0x1C`, `+0x24`, `+0x25`, `+0x26`, `+0x28`,
/// `+0x2C` and `+0x34`.
///
/// # Safety
/// `this` must be writable for at least 0x38 bytes and `owner`/`value` must be
/// valid for the handle's lifetime.
///
/// # Confidence
/// high - every store and the vtable address are literal in the export.
pub unsafe fn fn_80160080(this: usize, owner: u32, value: u32) -> usize {
    platform::call1(FN_HANDLE_BASE_CTOR, this as u32);
    platform::write_u32(this + 0x0C, HANDLE_VTABLE as u32);
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this + 0x18, 0);
    platform::write_u32(this + 0x1C, 0);
    platform::write_u32(this + 0x20, owner);
    platform::write_u8(this + 0x24, 0);
    platform::write_u8(this + 0x25, 0);
    platform::write_u16(this + 0x26, 0);
    platform::write_u32(this + 0x28, 0);
    platform::write_u32(this + 0x2C, 0);
    platform::write_u32(this + 0x30, value);
    platform::write_u32(this + 0x34, 0);
    this
}

/// Closes a handle's lock object and optionally returns it to the heap.
///
/// # Original
/// - Ghidra: `FUN_80160110` at `0x80160110` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160110_FUN_80160110.c`
/// - observed: `FUN_80160930(param_1 + 0x34, 0xffffffff); FUN_8015fe50(param_1, 0);
///   if (0 < param_2) FUN_802b9280(param_1);`
///
/// # Arguments
/// - `this`: handle returned by [`fn_80160080`].
/// - `deleting`: when positive, free the handle after destruction.
///
/// # Returns
/// `this` unchanged (C returns the pointer even after freeing).
///
/// # Side effects
/// Closes the state object at `+0x34`, runs the base destructor, and may free
/// `this` through the default deallocator.
///
/// # Safety
/// `this` must be null or a live handle. After a positive `deleting` the
/// pointer must not be used again.
///
/// # Confidence
/// high - the null guard, both calls and the size test are literal.
pub unsafe fn fn_80160110(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::call2(FN_HANDLE_LOCK_CLOSE, (this + 0x34) as u32, 0xFFFF_FFFF);
        platform::call2(FN_HANDLE_BASE_DTOR, this as u32, 0);
        if deleting > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Publishes a handle and initialises its scratch buffer and two minima.
///
/// # Original
/// - Ghidra: `FUN_80160190` at `0x80160190` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160190_FUN_80160190.c`
/// - observed: `r13-0x52fc = param_1; buf = (*vtbl[0x14])(param_1, 0xd0, 4);
///   r13-0x5304 = buf; if (FUN_802b9220(0xd0, buf) != 0) FUN_802b7b00();
///   r13-0x5308 = 0xd0; r13-0x52f8 = (*vtbl[0x28])(param_1, 4);
///   r13-0x52f4 = (*vtbl[0x24])(param_1);`
///
/// # Arguments
/// - `this`: live handle object (`0x48` bytes) whose vtable supplies the
///   allocator and the two measurement slots.
///
/// # Side effects
/// Publishes `this` in [`G_HANDLE_SELF`], allocates a `0xD0`-byte buffer into
/// [`G_HANDLE_BUFFER`], constructs it, records the remaining size in
/// [`G_HANDLE_REMAIN`] and stores the two slot readings in
/// [`G_HANDLE_MIN_A`] / [`G_HANDLE_MIN_B`].
///
/// # Safety
/// `this` must point at a live object with a callable vtable and the SDA slots
/// must not be used by another handle concurrently.
///
/// # Confidence
/// medium - all offsets and calls are literal, but the vtable slot arguments
/// other than `this` are inferred from the surrounding handle code.
pub unsafe fn fn_80160190(this: usize) {
    platform::write_u32(G_HANDLE_SELF, this as u32);
    let vtable = platform::read_u32(this) as usize;
    let buffer = platform::call3(
        platform::read_u32(vtable + 0x14) as usize,
        this as u32,
        0xD0,
        4,
    );
    platform::write_u32(G_HANDLE_BUFFER, buffer);
    let constructed = platform::call2(FN_HANDLE_PLACEMENT, 0xD0, buffer);
    if constructed != 0 {
        platform::call1(FN_HANDLE_BLOCK_CTOR, constructed);
    }
    platform::write_u32(G_HANDLE_REMAIN, 0xD0);
    let min_a = platform::call2(
        platform::read_u32(vtable + 0x28) as usize,
        this as u32,
        4,
    );
    platform::write_u32(G_HANDLE_MIN_A, min_a);
    let min_b = platform::call1(platform::read_u32(vtable + 0x24) as usize, this as u32);
    platform::write_u32(G_HANDLE_MIN_B, min_b);
}

/// Lowers the two recorded minima when `this + 0x30` is the published handle.
///
/// # Original
/// - Ghidra: `FUN_80160240` at `0x80160240` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160240_FUN_80160240.c`
/// - observed: `piVar2 = *(int **)(param_1 + 0x30); if (piVar2 == r13-0x52fc) { ... }`
///
/// # Arguments
/// - `this`: object whose `+0x30` word may name a handle.
///
/// # Side effects
/// Calls vtable slots `+0x28` and `+0x24` on the handle and replaces
/// [`G_HANDLE_MIN_A`] / [`G_HANDLE_MIN_B`] when the new value is smaller.
///
/// # Safety
/// `this` must be readable and `this + 0x30`, when non-null, must be a live
/// handle with those vtable slots.
///
/// # Confidence
/// medium - the comparison and min updates are literal; the second virtual
/// call's receiver is inferred because Ghidra prints an empty argument list.
pub unsafe fn fn_80160240(this: usize) {
    let object = platform::read_u32(this + 0x30) as usize;
    if object == platform::read_u32(G_HANDLE_SELF) as usize {
        let vtable = platform::read_u32(object) as usize;
        let value_a =
            platform::call2(platform::read_u32(vtable + 0x28) as usize, object as u32, 4);
        if value_a < platform::read_u32(G_HANDLE_MIN_A) {
            platform::write_u32(G_HANDLE_MIN_A, value_a);
        }
        let object_again = platform::read_u32(this + 0x30) as usize;
        let vtable_again = platform::read_u32(object_again) as usize;
        let value_b =
            platform::call1(platform::read_u32(vtable_again + 0x24) as usize, object_again as u32);
        if value_b < platform::read_u32(G_HANDLE_MIN_B) {
            platform::write_u32(G_HANDLE_MIN_B, value_b);
        }
    }
}

/// Sums a run of big-endian half-words.
///
/// # Original
/// - Ghidra: `FUN_801602d0` at `0x801602d0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801602d0_FUN_801602d0.c`
/// - observed: `for (; param_2 != 0; param_2 += -2) iVar2 += *param_1++;`
///
/// # Arguments
/// - `words`: first half-word of the run.
/// - `byte_len`: number of bytes to consume; the loop assumes a multiple of 2.
///
/// # Returns
/// The 32-bit sum of the `byte_len / 2` unsigned half-words (wrapping, as the
/// PowerPC `add` does).
///
/// # Safety
/// `words .. words + byte_len` must be readable.
///
/// # Confidence
/// high - a single half-word load and add in a counted loop.
pub unsafe fn fn_801602d0(words: usize, byte_len: i32) -> i32 {
    let mut sum = 0i32;
    let mut cursor = words;
    let mut remaining = byte_len;
    while remaining != 0 {
        sum = sum.wrapping_add(platform::read_u16(cursor) as i32);
        cursor += 2;
        remaining -= 2;
    }
    sum
}

/// Closes a handle's lock object and optionally returns it to the heap.
///
/// This is the pointer-taking variant used by the handle destructor; it only
/// closes when the first word (the lock) is non-zero.
///
/// # Original
/// - Ghidra: `FUN_80160930` at `0x80160930` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160930_FUN_80160930.c`
/// - observed: `if (param_1 != 0) { if (*param_1 != 0) FUN_801609d0();
///   if (0 < param_2) FUN_802b9280(param_1); }`
///
/// # Arguments
/// - `this`: pointer to a handle tail; word `+0x00` is the lock word.
/// - `deleting`: when positive, free `this`.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Releases the lock and may free `this`.
///
/// # Safety
/// `this` must be null or writable for one word; after a positive `deleting`
/// it must not be used again.
///
/// # Confidence
/// high - the guards, lock test and free call are literal.
pub unsafe fn fn_80160930(this: usize, deleting: i32) -> usize {
    if this != 0 {
        if platform::read_u32(this) != 0 {
            platform::call1(FN_HANDLE_LOCK_RELEASE, this as u32);
        }
        if deleting > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Acquires a lock word and stores it in front of the handle tail.
///
/// # Original
/// - Ghidra: `FUN_80160990` at `0x80160990` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160990_FUN_80160990.c`
/// - observed: `uVar1 = FUN_80235710(param_1 + 1); *param_1 = uVar1;`
///
/// # Arguments
/// - `this`: two-word object; the lock is created from the word at `+0x04`.
///
/// # Side effects
/// Stores the new lock word at `+0x00`.
///
/// # Safety
/// `this .. this + 8` must be readable/writable and `+0x04` must be a valid
/// lock descriptor.
///
/// # Confidence
/// medium - the layout and store are literal; the lock factory's role is
/// inferred from its paired release in [`fn_801609d0`].
pub unsafe fn fn_80160990(this: usize) {
    let lock = platform::call1(FN_HANDLE_LOCK_NEW, (this + 4) as u32);
    platform::write_u32(this, lock);
}

/// Releases the lock word at `+0x00` and clears it.
///
/// # Original
/// - Ghidra: `FUN_801609d0` at `0x801609d0` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801609d0_FUN_801609d0.c`
/// - observed: `FUN_802357a0(*param_1); *param_1 = 0;`
///
/// # Arguments
/// - `this`: two-word object whose `+0x00` word holds the lock.
///
/// # Side effects
/// Destroys the lock and zeroes the word.
///
/// # Safety
/// `this` must be writable and `+0x00` must hold a lock created by
/// [`fn_80160990`] (or zero; the callee is responsible for that policy).
///
/// # Confidence
/// high - the release call, the load and the clear are literal.
pub unsafe fn fn_801609d0(this: usize) {
    platform::call1(FN_HANDLE_LOCK_FREE, platform::read_u32(this));
    platform::write_u32(this, 0);
}

/// Inserts `node` directly after `previous` in a doubly-linked list.
///
/// A null `previous` means "insert at the head" and is delegated to
/// [`fn_80161020`], which is how Ghidra prints the tail call.
///
/// # Original
/// - Ghidra: `FUN_80160f10` at `0x80160f10` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160f10_FUN_80160f10.c`
/// - observed: `if (param_3 == 0) return FUN_80161020(); node->next = prev->next;
///   node->prev = prev; prev->next = node; fix tail/peer;`
///
/// # Arguments
/// - `list`: `{head, tail}` header.
/// - `node`: node to link; `+0x00` previous, `+0x04` next.
/// - `previous`: node to insert after, or zero for the head path.
///
/// # Returns
/// `1` when linked, `0` when `node` is null.
///
/// # Side effects
/// Rewrites up to four node/header words.
///
/// # Safety
/// All three pointers must belong to the same list and `node` must be unlinked.
///
/// # Confidence
/// high - every link update is literal; only the call argument list is elided
/// by Ghidra on the head path.
pub unsafe fn fn_80160f10(list: usize, node: usize, previous: usize) -> u32 {
    if previous == 0 {
        return fn_80161020(list, node);
    }
    if node != 0 {
        platform::write_u32(node + 0x04, platform::read_u32(previous + 0x04));
        platform::write_u32(node, previous as u32);
        platform::write_u32(previous + 0x04, node as u32);
        let next = platform::read_u32(node + 0x04) as usize;
        if next == 0 {
            platform::write_u32(list + 0x04, node as u32);
        } else {
            platform::write_u32(next, node as u32);
        }
        return 1;
    }
    0
}

/// Unlinks `node` from a doubly-linked list and clears its links.
///
/// # Original
/// - Ghidra: `FUN_80160f60` at `0x80160f60` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160f60_FUN_80160f60.c`
/// - observed: `if (node->prev == 0) { if (node == list->head) list->head = node->next; }
///   else node->prev->next = node->next; ... node->prev = node->next = 0;`
///
/// # Arguments
/// - `list`: `{head, tail}` header.
/// - `node`: linked node to remove.
///
/// # Returns
/// `1` when removed, `0` when `node` is null.
///
/// # Side effects
/// Rewrites the neighbours and both link words of `node`.
///
/// # Safety
/// `node` must be linked in `list` (or at least be a valid node).
///
/// # Confidence
/// high - the four link cases and the two clears are literal.
pub unsafe fn fn_80160f60(list: usize, node: usize) -> u32 {
    if node != 0 {
        let previous = platform::read_u32(node) as usize;
        if previous == 0 {
            if node == platform::read_u32(list) as usize {
                platform::write_u32(list, platform::read_u32(node + 0x04));
            }
        } else {
            platform::write_u32(previous + 0x04, platform::read_u32(node + 0x04));
        }
        let next = platform::read_u32(node + 0x04) as usize;
        if next == 0 {
            if node == platform::read_u32(list + 0x04) as usize {
                platform::write_u32(list + 0x04, previous as u32);
            }
        } else {
            platform::write_u32(next, previous as u32);
        }
        platform::write_u32(node, 0);
        platform::write_u32(node + 0x04, 0);
        return 1;
    }
    0
}

/// Appends `node` at the tail of a doubly-linked list.
///
/// # Original
/// - Ghidra: `FUN_80160fe0` at `0x80160fe0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160fe0_FUN_80160fe0.c`
/// - observed: `if (list->tail == 0) list->head = node; else { list->tail->next = node;
///   node->prev = list->tail; } list->tail = node;`
///
/// # Arguments
/// - `list`: `{head, tail}` header.
/// - `node`: unlinked node to append.
///
/// # Returns
/// `1` when appended, `0` when `node` is null.
///
/// # Side effects
/// Updates the header tail and up to two node words.
///
/// # Safety
/// `node` must be unlinked and the header writable.
///
/// # Confidence
/// high - the empty-list and non-empty cases are literal.
pub unsafe fn fn_80160fe0(list: usize, node: usize) -> u32 {
    if node != 0 {
        let tail = platform::read_u32(list + 0x04);
        if tail == 0 {
            platform::write_u32(list, node as u32);
        } else {
            platform::write_u32(tail as usize + 0x04, node as u32);
            platform::write_u32(node, tail);
        }
        platform::write_u32(list + 0x04, node as u32);
        return 1;
    }
    0
}

/// Inserts `node` at the head of a doubly-linked list.
///
/// # Original
/// - Ghidra: `FUN_80161020` at `0x80161020` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161020_FUN_80161020.c`
/// - observed: `if (list->head == 0) list->tail = node; else { list->head->prev = node;
///   node->next = list->head; } list->head = node;`
///
/// # Arguments
/// - `list`: `{head, tail}` header.
/// - `node`: unlinked node to insert.
///
/// # Returns
/// `1` when inserted, `0` when `node` is null.
///
/// # Side effects
/// Updates the header head and up to two node words.
///
/// # Safety
/// `node` must be unlinked and the header writable.
///
/// # Confidence
/// high - both branches are literal.
pub unsafe fn fn_80161020(list: usize, node: usize) -> u32 {
    if node != 0 {
        let head = platform::read_u32(list);
        if head == 0 {
            platform::write_u32(list + 0x04, node as u32);
        } else {
            platform::write_u32(head as usize, node as u32);
            platform::write_u32(node + 0x04, head);
        }
        platform::write_u32(list, node as u32);
        return 1;
    }
    0
}

/// Returns the live context object published in `r13 - 0x52B4`.
///
/// # Original
/// - Ghidra: `FUN_80164b10` at `0x80164b10` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b10_FUN_80164b10.c`
/// - observed: `return *(undefined4 *)(unaff_r13 + -0x52b4);`
///
/// # Returns
/// The context pointer, or zero before the context is created.
///
/// # Safety
/// The SDA slot must not be written concurrently.
///
/// # Confidence
/// high - one SDA load.
pub unsafe fn fn_80164b10() -> u32 {
    platform::read_u32(G_CTX_MAIN)
}

/// Returns one context state by id.
///
/// # Original
/// - Ghidra: `FUN_80164b20` at `0x80164b20` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b20_FUN_80164b20.c`
/// - observed: `FUN_80258bd0(*(undefined4 *)(r13 + -0x52b4), param_1);`
///
/// # Arguments
/// - `state`: slot id in `0..=0x1F`.
///
/// # Returns
/// The state word stored for that id.
///
/// # Side effects
/// None; the SDA load and the lookup are read-only.
///
/// # Safety
/// The context from [`fn_80164b10`] must be live and the id in range.
///
/// # Confidence
/// high - the SDA load and the single call are literal.
pub unsafe fn fn_80164b20(state: u32) -> u32 {
    platform::call2(
        FN_CTX_STATE_GET_BY_ID,
        platform::read_u32(G_CTX_MAIN),
        state,
    )
}

/// Reads the context's current state (the result is discarded by the wrapper).
///
/// # Original
/// - Ghidra: `FUN_80164b30` at `0x80164b30` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b30_FUN_80164b30.c`
/// - observed: `FUN_80258c30(*(undefined4 *)(r13 + -0x52b4));`
///
/// # Side effects
/// Runs the read-only state lookup; the returned word is not stored.
///
/// # Safety
/// The live context must be non-null.
///
/// # Confidence
/// high - the SDA load and the single call are literal.
pub unsafe fn fn_80164b30() {
    platform::call1(
        FN_CTX_STATE_GET_CURRENT,
        platform::read_u32(G_CTX_MAIN),
    );
}

/// Returns the context's current slot index byte at `+0xF4`.
///
/// # Original
/// - Ghidra: `FUN_80164b40` at `0x80164b40` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b40_FUN_80164b40.c`
/// - observed: `return *(undefined1 *)(*(int *)(r13 + -0x52b4) + 0xf4);`
///
/// # Returns
/// The index of the active state slot.
///
/// # Safety
/// The live context must be readable.
///
/// # Confidence
/// high - one byte load at the literal offset.
pub unsafe fn fn_80164b40() -> u8 {
    platform::read_u8(platform::read_u32(G_CTX_MAIN) as usize + 0xF4)
}

/// Returns `context + 0x2878`, the start of a sub-object of the live context.
///
/// # Original
/// - Ghidra: `FUN_80164b60` at `0x80164b60` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b60_FUN_80164b60.c`
/// - observed: `return *(int *)(r13 + -0x52b4) + 0x2878;`
///
/// # Returns
/// The address `context + 0x2878` (zero-extended through the pointer when the
/// context is null).
///
/// # Safety
/// The returned address is only usable when the context is live and has at
/// least 0x2878 bytes.
///
/// # Confidence
/// high - one add at the literal offset.
pub unsafe fn fn_80164b60() -> usize {
    platform::read_u32(G_CTX_MAIN) as usize + 0x2878
}

/// Returns slot `index` from the per-context object array.
///
/// # Original
/// - Ghidra: `FUN_80164b70` at `0x80164b70` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b70_FUN_80164b70.c`
/// - observed: `return *(undefined4 *)(*(int *)(r13 + -0x52ac) + param_1 * 4);`
///
/// # Arguments
/// - `index`: slot index; no bounds check in the original.
///
/// # Returns
/// The slot word, or zero when the array entry is null.
///
/// # Safety
/// The array pointer in [`G_CTX_SLOTS`] must be non-null and `index` must be
/// inside the allocated array.
///
/// # Confidence
/// high - one indexed load.
pub unsafe fn fn_80164b70(index: i32) -> u32 {
    platform::read_u32(
        platform::read_u32(G_CTX_SLOTS) as usize + index as usize * 4,
    )
}

/// Asks slot `index` to close and clears bit 2 of its auxiliary flag byte.
///
/// # Original
/// - Ghidra: `FUN_80164b90` at `0x80164b90` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164b90_FUN_80164b90.c`
/// - observed: `if (array && (obj = array[index]) != 0) (*obj->vtable[0x28])();
///   if (aux && (e = aux[index]) != 0) e->byte[4] &= 0xfb;`
///
/// # Arguments
/// - `index`: slot index used for both arrays.
///
/// # Side effects
/// Runs the slot object's `+0x28` method and clears bit 2 in the auxiliary
/// entry's `+0x04` byte.
///
/// # Safety
/// Both array pointers may be null; when non-null, `index` must be in range
/// and the slot object's vtable callable.
///
/// # Confidence
/// medium - the null tests, offsets and mask are literal; the virtual call's
/// receiver is inferred because Ghidra prints no argument list.
pub unsafe fn fn_80164b90(index: i32) {
    let slots = platform::read_u32(G_CTX_SLOTS) as usize;
    if slots != 0 {
        let object = platform::read_u32(slots + index as usize * 4) as usize;
        if object != 0 {
            let vtable = platform::read_u32(object) as usize;
            platform::call1(platform::read_u32(vtable + 0x28) as usize, object as u32);
        }
    }
    let aux = platform::read_u32(G_CTX_AUX_SLOTS) as usize;
    if aux != 0 {
        let entry = platform::read_u32(aux + index as usize * 4) as usize;
        if entry != 0 {
            platform::write_u8(entry + 4, platform::read_u8(entry + 4) & 0xFB);
        }
    }
}

/// Looks up a key in the map owned by slot `index`'s object.
///
/// # Original
/// - Ghidra: `FUN_80164c50` at `0x80164c50` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164c50_FUN_80164c50.c`
/// - observed: `if (array == 0) return 0; FUN_802cdd00(local, *(param_2 + 4));
///   map = *(*(array + param_3 * 4) + 0x14); r = FUN_802cd2d0(map, param_1);
///   return FUN_802cd1f0(map, r, local);`
///
/// # Arguments
/// - `key`: lookup key passed straight to the map helper.
/// - `descriptor`: object whose `+0x04` word seeds the 16-byte local buffer.
/// - `index`: slot index; the slot object's `+0x14` word is the map.
///
/// # Returns
/// The map helper's result, or `0` when the slot array is absent.
///
/// # Side effects
/// Builds a local key and runs two map operations on the slot's map.
///
/// # Safety
/// The array/slot/map chain must be live and `descriptor + 4` must be a valid
/// key source.
///
/// # Confidence
/// medium - all offsets and the three calls are literal; the map helper roles
/// are inferred from call order and names are not claimed.
pub unsafe fn fn_80164c50(key: u32, descriptor: usize, index: i32) -> u32 {
    let slots = platform::read_u32(G_CTX_SLOTS) as usize;
    if slots == 0 {
        return 0;
    }
    let mut local_key = crate::platform::GuestLocal::new( [0u32; 4]);
    let key_ptr = local_key.as_mut_ptr() as u32;
    platform::call2(FN_CTX_KEY_INIT, key_ptr, platform::read_u32(descriptor + 4));
    let object = platform::read_u32(slots + index as usize * 4) as usize;
    let map = platform::read_u32(object + 0x14);
    let found = platform::call2(FN_CTX_KEY_FIND, map, key);
    platform::call3(FN_CTX_KEY_STORE, map, found, key_ptr)
}

/// Shuts down slot `index` and the live context.
///
/// # Original
/// - Ghidra: `FUN_80164cd0` at `0x80164cd0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164cd0_FUN_80164cd0.c`
/// - observed: `if (array && (obj = array[index])) (*obj->vtable[0x20])(obj, context);
///   if (aux && aux[index]) FUN_802c6640(aux[index]); FUN_802590a0(context);`
///
/// # Arguments
/// - `index`: slot index used for both arrays.
///
/// # Side effects
/// Runs the slot's `+0x20` method with the live context, marks the auxiliary
/// entry closed, then calls the context close routine.
///
/// # Safety
/// Both arrays may be null; when present, `index` must be in range and all
/// objects live.
///
/// # Confidence
/// medium - the guards, offsets and calls are literal; the auxiliary marker's
/// argument is inferred because Ghidra prints an empty argument list.
pub unsafe fn fn_80164cd0(index: i32) {
    let slots = platform::read_u32(G_CTX_SLOTS) as usize;
    if slots != 0 {
        let object = platform::read_u32(slots + index as usize * 4) as usize;
        if object != 0 {
            let vtable = platform::read_u32(object) as usize;
            platform::call2(
                platform::read_u32(vtable + 0x20) as usize,
                object as u32,
                platform::read_u32(G_CTX_MAIN),
            );
        }
    }
    let aux = platform::read_u32(G_CTX_AUX_SLOTS) as usize;
    if aux != 0 {
        let entry = platform::read_u32(aux + index as usize * 4);
        if entry != 0 {
            platform::call1(FN_CTX_AUX_MARK, entry);
        }
    }
    platform::call1(FN_CTX_CLOSE, platform::read_u32(G_CTX_MAIN));
}

/// Requests teardown of the live context.
///
/// # Original
/// - Ghidra: `FUN_80164d50` at `0x80164d50` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164d50_FUN_80164d50.c`
/// - observed: `FUN_802590f0(*(undefined4 *)(r13 + -0x52b4));`
///
/// # Side effects
/// Runs the context's `+0x0C` vtable method with request kind `2`.
///
/// # Safety
/// The live context from [`fn_80164b10`] must be non-null.
///
/// # Confidence
/// high - the SDA load and the single call are literal.
pub unsafe fn fn_80164d50() {
    platform::call1(FN_CTX_TEARDOWN, platform::read_u32(G_CTX_MAIN));
}

/// Dispatches the `0x38`-stride element at `index` to [`fn_80168080`].
///
/// # Original
/// - Ghidra: `FUN_801662e0` at `0x801662e0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801662e0_FUN_801662e0.c`
/// - observed: `FUN_80168080(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Side effects
/// Runs the element object's `+0x20` vtable slot through [`fn_80168080`].
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the owner load, stride and forward call are literal.
pub unsafe fn fn_801662e0(this: usize, index: i32) {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    fn_80168080(element);
}

/// Sets the `0x38`-stride element's value through the `0x80168FE0` wrapper.
///
/// # Original
/// - Ghidra: `FUN_801662f0` at `0x801662f0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801662f0_FUN_801662f0.c`
/// - observed: `FUN_80168fe0(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Side effects
/// [`FN_ELEMENT_VALUE_SET`] dispatches the element's `+0x1C` slot and writes
/// the float at `+0x30`.
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// medium - the address arithmetic is literal; the value argument lives in
/// `f1` and is not represented in Ghidra's call text, so only the element
/// pointer is forwarded here.
pub unsafe fn fn_801662f0(this: usize, index: i32) {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    platform::call1(FN_ELEMENT_VALUE_SET, element as u32);
}

/// Dispatches the `0x38`-stride element at `index` to [`fn_801680c0`].
///
/// # Original
/// - Ghidra: `FUN_80166300` at `0x80166300` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166300_FUN_80166300.c`
/// - observed: `FUN_801680c0(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Side effects
/// Runs the element object's `+0x2C` vtable slot through [`fn_801680c0`].
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the owner load, stride and forward call are literal.
pub unsafe fn fn_80166300(this: usize, index: i32) {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    fn_801680c0(element);
}

/// Dispatches the `0x38`-stride element at `index` to [`fn_801680e0`].
///
/// # Original
/// - Ghidra: `FUN_80166310` at `0x80166310` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166310_FUN_80166310.c`
/// - observed: `FUN_801680e0(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Side effects
/// Runs the element object's `+0x28` vtable slot through [`fn_801680e0`].
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the owner load, stride and forward call are literal.
pub unsafe fn fn_80166310(this: usize, index: i32) {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    fn_801680e0(element);
}

/// Runs the element range test at `0x80169020` for the element at `index`.
///
/// # Original
/// - Ghidra: `FUN_80166320` at `0x80166320` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166320_FUN_80166320.c`
/// - observed: `FUN_80169020(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Side effects
/// [`FN_ELEMENT_TEST`] reads the element's `+0x28`, `+0x2C` and `+0x34` fields
/// and returns whether the element is inside its range; the boolean is dropped
/// because this trampoline is declared `void`.
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// medium - the address arithmetic is literal; the trampoline discards the
/// callee's boolean result.
pub unsafe fn fn_80166320(this: usize, index: i32) {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    platform::call1(FN_ELEMENT_TEST, element as u32);
}

/// Stores a byte in the `0x38`-stride element's `+0x34` field.
///
/// # Original
/// - Ghidra: `FUN_80166340` at `0x80166340` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166340_FUN_80166340.c`
/// - observed: `*(undefined1 *)(*(int *)(param_1 + 0x28) + param_3 * 0x38 + 0x34) = param_2;`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `value`: byte to store (the element's mode/tag).
/// - `index`: element index.
///
/// # Side effects
/// Writes one byte at `element + 0x34`.
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the store with both scales is literal.
pub unsafe fn fn_80166340(this: usize, value: u8, index: i32) {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    platform::write_u8(element + 0x34, value);
}

/// Returns the `0x38`-stride element's `+0x28` float.
///
/// # Original
/// - Ghidra: `FUN_80166360` at `0x80166360` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166360_FUN_80166360.c`
/// - observed: `return (double)*(float *)(*(int *)(param_1 + 0x28) + param_2 * 0x38 + 0x28);`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Returns
/// The element's `+0x28` single-precision value (C widens it to `double` on
/// return; the stored field is `f32`).
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - one float load at the literal offset.
pub unsafe fn fn_80166360(this: usize, index: i32) -> f32 {
    let element = platform::read_u32(this + ELEMENT_OWNER) as usize
        + index as usize * ELEMENT_STRIDE;
    platform::read_f32(element + 0x28)
}

// ===========================================================================
// Local call helpers
// ===========================================================================

/// Calls a routine by absolute address with no arguments and keeps its result.
///
/// [`crate::platform::call0`] discards the return value; the thread-id query
/// used by [`mount_queue_submit`] needs it.
///
/// # Safety
/// `addr` must be the entry point of a function taking no arguments.
unsafe fn call0_u32(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Calls a routine by absolute address with eight arguments.
///
/// [`crate::platform`] exposes `call0`..`call7`; the mount executor at
/// [`MOUNT_EXEC`] takes eight.
///
/// # Safety
/// `addr` must accept eight `u32` arguments and keep the PowerPC ABI.
unsafe fn call8_u32(
    addr: usize,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
) -> u32 {
    let function = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32);
    function(a, b, c, d, e, f, g, h)
}

// ===========================================================================
// 0x80160000 bucket: leaf wrappers and small object helpers (wave 44, manual)
// ===========================================================================

/// `r13 - 0x52F0` -> `0x8042A390`: word stored by [`fn_80161380`].
///
/// `// unknown:` the global's role; no translated caller names it.
const G_80161380_TARGET: usize = SDA_BASE - 0x52F0;

/// `r13 - 0x5208` -> `0x8042A478`: instance installed by [`fn_8016f640`].
///
/// `// unknown:` the object's type; [`fn_8016f660`] only returns it.
const G_8016F640_INSTANCE: usize = SDA_BASE - 0x5208;

/// Stores a word in the `r13 - 0x52F0` global.
///
/// # Original
/// - Ghidra: `FUN_80161380` at `0x80161380` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161380_FUN_80161380.c`
///
/// # Arguments
/// - `value`: word to store.
///
/// # Side effects
/// Writes [`G_80161380_TARGET`].
///
/// # Safety
/// The SDA slot must be writable.
///
/// # Confidence
/// high - one literal SDA store; the global's role is `// unknown:`.
pub unsafe fn fn_80161380(value: u32) {
    platform::write_u32(G_80161380_TARGET, value);
}

/// Passes the `r13 - 0x52F0` global to `FUN_80161490`.
///
/// # Original
/// - Ghidra: `FUN_80161390` at `0x80161390` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161390_FUN_80161390.c`
///
/// # Side effects
/// Whatever `FUN_80161490` does with the SDA object.
///
/// # Safety
/// The SDA slot must name an object acceptable to `FUN_80161490`.
///
/// # Confidence
/// high - the literal load and tail call are the whole body.
pub unsafe fn fn_80161390() {
    platform::call1(0x8016_1490, G_80161380_TARGET as u32);
}

/// Returns the constant `1`.
///
/// # Original
/// - Ghidra: `FUN_80161d90` at `0x80161d90` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161d90_FUN_80161d90.c`
///
/// # Returns
/// `1`.
///
/// # Confidence
/// high - the export is a single `li r3,1; blr`.
pub unsafe fn fn_80161d90() -> u32 {
    1
}

/// Forwards three arguments to `FUN_80162a70` with a zero second argument.
///
/// # Original
/// - Ghidra: `FUN_80162b20` at `0x80162b20` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162b20_FUN_80162b20.c`
///
/// # Arguments
/// - `this`: first forwarded pointer.
/// - `a`: third argument.
/// - `b`: fourth argument.
///
/// # Safety
/// `this` and `FUN_80162a70`'s expectations must match.
///
/// # Confidence
/// high - one four-argument call with a literal zero.
pub unsafe fn fn_80162b20(this: usize, a: u32, b: u32) {
    platform::call4(0x8016_2A70, this as u32, 0, a, b);
}

/// Clears `this + 8` and passes the `this + 4` pointer to `FUN_802B6700`.
///
/// # Original
/// - Ghidra: `FUN_80163690` at `0x80163690` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163690_FUN_80163690.c`
///
/// # Arguments
/// - `this`: object whose `+4` pointer is released and `+8` word is cleared.
///
/// # Safety
/// `this` must be readable and writable; the `+4` pointer must be valid for
/// whatever `FUN_802B6700` does with it.
///
/// # Confidence
/// high - the store, load and call are literal.
pub unsafe fn fn_80163690(this: usize) {
    platform::write_u32(this + 8, 0);
    platform::call1(0x802B_6700, platform::read_u32(this + 4));
}

/// Stores one double-converted value at `object + 8` and `object + 0xC`.
///
/// # Original
/// - Ghidra: `FUN_801637d0` at `0x801637d0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801637d0_FUN_801637d0.c`
///
/// # Arguments
/// - `value`: value narrowed to `f32` by the original code.
/// - `object`: destination object.
///
/// # Side effects
/// Writes two `f32` fields.
///
/// # Safety
/// `object` must be writable for 16 bytes.
///
/// # Confidence
/// high - two literal `f32` stores with the same converted argument.
pub unsafe fn fn_801637d0(value: f64, object: usize) {
    platform::write_f32(object + 8, value as f32);
    platform::write_f32(object + 0xC, value as f32);
}

/// Stores one double-converted value at `object + 0x10`.
///
/// # Original
/// - Ghidra: `FUN_801637e0` at `0x801637e0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801637e0_FUN_801637e0.c`
///
/// # Arguments
/// - `value`: value narrowed to `f32` by the original code.
/// - `object`: destination object.
///
/// # Safety
/// `object + 0x10` must be writable.
///
/// # Confidence
/// high - one literal `f32` store.
pub unsafe fn fn_801637e0(value: f64, object: usize) {
    platform::write_f32(object + 0x10, value as f32);
}

/// Clears `this + 8` and calls `0x8016A090(this)`.
///
/// # Original
/// - Ghidra: `FUN_80168b20` at `0x80168b20` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168b20_FUN_80168b20.c`
/// - observed: the export prints `FUN_8016a090()` without arguments, but
///   `0x8016A090` dereferences `r3 + 4` and the wrapper never reloads `r3`, so
///   the callee receives `this`.
///
/// # Safety
/// `this + 8` must be writable; `0x8016A090` receives `this`.
///
/// # Confidence
/// high - the store, the untouched `r3` and the call are literal.
pub unsafe fn fn_80168b20(this: usize) {
    platform::write_u32(this + 8, 0);
    platform::call1(0x8016_A090, this as u32);
}

/// Clears the byte at `this + 0x1C` and calls `0x80168B90`.
///
/// # Original
/// - Ghidra: `FUN_80168b80` at `0x80168b80` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168b80_FUN_80168b80.c`
/// - observed: the export prints `FUN_80168b90()` without arguments, but
///   `0x80168B90` writes `r3 + 4..+0x18` and the wrapper never reloads `r3`.
///
/// # Safety
/// `this + 0x1C` must be writable; `0x80168B90` receives `this`.
///
/// # Confidence
/// high - the byte store, the untouched `r3` and the call are literal.
pub unsafe fn fn_80168b80(this: usize) {
    platform::write_u8(this + 0x1C, 0);
    platform::call1(0x8016_8B90, this as u32);
}

/// Installs `value` in the `r13 - 0x5208` global; on null, calls `0x8016F550`.
///
/// # Original
/// - Ghidra: `FUN_8016f640` at `0x8016f640` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f640_FUN_8016f640.c`
///
/// # Arguments
/// - `value`: instance pointer, or `0` to run the teardown routine.
///
/// # Side effects
/// Writes [`G_8016F640_INSTANCE`] before the null test.
///
/// # Safety
/// The SDA slot must be writable; `0x8016F550` must accept no arguments.
///
/// # Confidence
/// high - the store, null test and call are literal.
pub unsafe fn fn_8016f640(value: u32) {
    platform::write_u32(G_8016F640_INSTANCE, value);
    if value == 0 {
        platform::call0(0x8016_F550);
    }
}

/// Returns the `r13 - 0x5208` global.
///
/// # Original
/// - Ghidra: `FUN_8016f660` at `0x8016f660` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f660_FUN_8016f660.c`
///
/// # Returns
/// The word installed by [`fn_8016f640`].
///
/// # Safety
/// The SDA slot must be readable.
///
/// # Confidence
/// high - one literal SDA load.
pub unsafe fn fn_8016f660() -> u32 {
    platform::read_u32(G_8016F640_INSTANCE)
}

/// Stores `value` at `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_8016f830` at `0x8016f830` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f830_FUN_8016f830.c`
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - one literal store.
pub unsafe fn fn_8016f830(this: usize, value: u32) {
    platform::write_u32(this + 4, value);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_80168FE0`.
///
/// # Original
/// - Ghidra: `FUN_80166ef0` at `0x80166ef0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166ef0_FUN_80166ef0.c`
///
/// # Arguments
/// - `this`: object owning the element array at `+0x28`.
/// - `index`: element index.
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80166ef0(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_8FE0, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_801680C0`.
///
/// # Original
/// - Ghidra: `FUN_80166f00` at `0x80166f00` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166f00_FUN_80166f00.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80166f00(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_80C0, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_801680E0`.
///
/// # Original
/// - Ghidra: `FUN_80166f10` at `0x80166f10` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166f10_FUN_80166f10.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80166f10(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_80E0, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_80169020`.
///
/// # Original
/// - Ghidra: `FUN_80166f20` at `0x80166f20` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166f20_FUN_80166f20.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80166f20(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_9020, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_801690E0`.
///
/// # Original
/// - Ghidra: `FUN_80166f30` at `0x80166f30` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166f30_FUN_80166f30.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80166f30(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_90E0, element as u32);
}

/// Forwards `value` to `FUN_802574E0` with the `this + 4` handle and mode `0`.
///
/// # Original
/// - Ghidra: `FUN_8016a170` at `0x8016a170` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a170_FUN_8016a170.c`
///
/// # Safety
/// `this + 4` must hold a handle accepted by `FUN_802574E0`.
///
/// # Confidence
/// high - the `+4` load and three-argument call are literal.
pub unsafe fn fn_8016a170(this: usize, value: u32) {
    platform::call3(0x8025_74E0, platform::read_u32(this + 4), 0, value);
}

/// Forwards `value` to `FUN_802575D0` with the `this + 4` handle and mode `0`.
///
/// # Original
/// - Ghidra: `FUN_8016a180` at `0x8016a180` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a180_FUN_8016a180.c`
///
/// # Safety
/// `this + 4` must hold a handle accepted by `FUN_802575D0`.
///
/// # Confidence
/// high - the `+4` load and three-argument call are literal.
pub unsafe fn fn_8016a180(this: usize, value: u32) {
    platform::call3(0x8025_75D0, platform::read_u32(this + 4), 0, value);
}

/// Forwards `value` to `FUN_802575D0` with the `this + 4` handle and mode `2`.
///
/// # Original
/// - Ghidra: `FUN_8016a190` at `0x8016a190` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a190_FUN_8016a190.c`
///
/// # Safety
/// `this + 4` must hold a handle accepted by `FUN_802575D0`.
///
/// # Confidence
/// high - the `+4` load and three-argument call are literal.
pub unsafe fn fn_8016a190(this: usize, value: u32) {
    platform::call3(0x8025_75D0, platform::read_u32(this + 4), 2, value);
}

/// Zeroes the four words at `this`.
///
/// # Original
/// - Ghidra: `FUN_80161560` at `0x80161560` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161560_FUN_80161560.c`
///
/// # Safety
/// `this` must be writable for 16 bytes.
///
/// # Confidence
/// high - four consecutive literal zero stores.
pub unsafe fn fn_80161560(this: usize) {
    platform::write_u32(this, 0);
    platform::write_u32(this + 4, 0);
    platform::write_u32(this + 8, 0);
    platform::write_u32(this + 0xC, 0);
}

/// Writes the byte `value` at the `index`th element's `+0x34`.
///
/// # Original
/// - Ghidra: `FUN_80166f40` at `0x80166f40` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166f40_FUN_80166f40.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - literal `+0x28` array load, `0x38` stride and `+0x34` byte store.
pub unsafe fn fn_80166f40(this: usize, value: u8, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::write_u8(element + 0x34, value);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_80168080`.
///
/// # Original
/// - Ghidra: `FUN_80167b10` at `0x80167b10` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b10_FUN_80167b10.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80167b10(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_8080, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_80168FE0`.
///
/// # Original
/// - Ghidra: `FUN_80167b20` at `0x80167b20` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b20_FUN_80167b20.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80167b20(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_8FE0, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_801680C0`.
///
/// # Original
/// - Ghidra: `FUN_80167b30` at `0x80167b30` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b30_FUN_80167b30.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80167b30(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_80C0, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_801680E0`.
///
/// # Original
/// - Ghidra: `FUN_80167b40` at `0x80167b40` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b40_FUN_80167b40.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80167b40(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_80E0, element as u32);
}

/// Dispatches the `index`th `0x38`-byte element to `FUN_80169020`.
///
/// # Original
/// - Ghidra: `FUN_80167b50` at `0x80167b50` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b50_FUN_80167b50.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - the `+0x28` load, `0x38` stride and call are literal.
pub unsafe fn fn_80167b50(this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::call1(0x8016_9020, element as u32);
}

/// Writes the byte `value` at the `index`th element's `+0x34`.
///
/// # Original
/// - Ghidra: `FUN_80167b60` at `0x80167b60` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b60_FUN_80167b60.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - literal `+0x28` array load, `0x38` stride and `+0x34` byte store.
pub unsafe fn fn_80167b60(this: usize, value: u8, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::write_u8(element + 0x34, value);
}

/// Returns the `index`th element's `+0x28` value as an `f32`.
///
/// # Original
/// - Ghidra: `FUN_80167b80` at `0x80167b80` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167b80_FUN_80167b80.c`
///
/// # Returns
/// The stored `f32` (the C caller widens it to `double`).
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - one literal `f32` load at `+0x28`.
pub unsafe fn fn_80167b80(this: usize, index: i32) -> f32 {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::read_f32(element + 0x28)
}

/// Stores a double-converted value at the `index`th element's `+0x2C`.
///
/// # Original
/// - Ghidra: `FUN_80167ba0` at `0x80167ba0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167ba0_FUN_80167ba0.c`
///
/// # Safety
/// `this + 0x28` must name the array and `index` must be in range.
///
/// # Confidence
/// high - one literal `f32` store at `+0x2C`.
pub unsafe fn fn_80167ba0(value: f64, this: usize, index: i32) {
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    platform::write_f32(element + 0x2C, value as f32);
}

/// Calls `FUN_80168BD0` on `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_80169c80` at `0x80169c80` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169c80_FUN_80169c80.c`
///
/// # Safety
/// `this + 4` must be acceptable to `FUN_80168BD0`.
///
/// # Confidence
/// high - one literal address computation and call.
pub unsafe fn fn_80169c80(this: usize) {
    platform::call1(0x8016_8BD0, (this + 4) as u32);
}

/// Stores `value` at `this + 0x38`.
///
/// # Original
/// - Ghidra: `FUN_8016a000` at `0x8016a000` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a000_FUN_8016a000.c`
///
/// # Safety
/// `this + 0x38` must be writable.
///
/// # Confidence
/// high - one literal store.
pub unsafe fn fn_8016a000(this: usize, value: u32) {
    platform::write_u32(this + 0x38, value);
}

/// Installs the `0x80329670` vtable and clears `+4`.
///
/// # Original
/// - Ghidra: `FUN_8016a010` at `0x8016a010` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a010_FUN_8016a010.c`
///
/// # Safety
/// `this` must be writable for 8 bytes.
///
/// # Confidence
/// medium - the vtable address is literal but the class identity is inferred.
pub unsafe fn fn_8016a010(this: usize) {
    platform::write_u32(this, 0x8032_9670);
    platform::write_u32(this + 4, 0);
}

/// Sets the `+0x1C` byte to `1` and clears `+0x1D`.
///
/// # Original
/// - Ghidra: `FUN_80168bb0` at `0x80168bb0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168bb0_FUN_80168bb0.c`
///
/// # Safety
/// `this` must be writable at both byte offsets.
///
/// # Confidence
/// high - two literal byte stores.
pub unsafe fn fn_80168bb0(this: usize) {
    platform::write_u8(this + 0x1C, 1);
    platform::write_u8(this + 0x1D, 0);
}

/// Calls `FUN_802E1938` on `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_8016da50` at `0x8016da50` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016da50_FUN_8016da50.c`
///
/// # Safety
/// `this + 4` must be acceptable to `FUN_802E1938`.
///
/// # Confidence
/// high - one literal address computation and call.
pub unsafe fn fn_8016da50(this: usize) {
    platform::call1(0x802E_1938, (this + 4) as u32);
}

/// Stores `value` at `this + 10`.
///
/// # Original
/// - Ghidra: `FUN_8016dce0` at `0x8016dce0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dce0_FUN_8016dce0.c`
///
/// # Safety
/// `this + 10` must be writable for two bytes.
///
/// # Confidence
/// high - one literal half-word store.
pub unsafe fn fn_8016dce0(this: usize, value: u16) {
    platform::write_u16(this + 10, value);
}

/// Stores three double-converted values at `[this+4] + 0xDC..0xE4`.
///
/// # Original
/// - Ghidra: `FUN_8016a130` at `0x8016a130` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a130_FUN_8016a130.c`
///
/// # Safety
/// `this + 4` must hold a pointer writable through `+0xE4`.
///
/// # Confidence
/// high - three literal `f32` stores after the `+4` load.
pub unsafe fn fn_8016a130(x: f64, y: f64, z: f64, this: usize) {
    let base = platform::read_u32(this + 4) as usize;
    platform::write_f32(base + 0xDC, x as f32);
    platform::write_f32(base + 0xE0, y as f32);
    platform::write_f32(base + 0xE4, z as f32);
}

/// Calls virtual slot `+0xC` of the object at `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_801636a0` at `0x801636a0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801636a0_FUN_801636a0.c`
///
/// # Safety
/// `this + 4` must hold an object whose vtable has a callable `+0xC` slot.
///
/// # Confidence
/// medium - the double indirection and literal slot are clear, but the vtable
/// type is inferred.
pub unsafe fn fn_801636a0(this: usize) {
    let object = platform::read_u32(this + 4) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0xC) as usize;
    platform::call0(method);
}

/// Calls virtual slot `+0x20` of the object at `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_8016a0f0` at `0x8016a0f0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a0f0_FUN_8016a0f0.c`
///
/// # Safety
/// `this + 4` must hold an object whose vtable has a callable `+0x20` slot.
///
/// # Confidence
/// medium - the double indirection and literal slot are clear, but the vtable
/// type is inferred.
pub unsafe fn fn_8016a0f0(this: usize) {
    let object = platform::read_u32(this + 4) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0x20) as usize;
    platform::call0(method);
}

/// Calls virtual slot `+0x24` of the object at `this + 4`.
///
/// # Original
/// - Ghidra: `FUN_8016a110` at `0x8016a110` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a110_FUN_8016a110.c`
///
/// # Safety
/// `this + 4` must hold an object whose vtable has a callable `+0x24` slot.
///
/// # Confidence
/// medium - the double indirection and literal slot are clear, but the vtable
/// type is inferred.
pub unsafe fn fn_8016a110(this: usize) {
    let object = platform::read_u32(this + 4) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0x24) as usize;
    platform::call0(method);
}

/// Calls virtual slot `+0xC` of the `r13-0x5220` object's `+0x10` field.
///
/// # Original
/// - Ghidra: `FUN_8016f410` at `0x8016f410` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f410_FUN_8016f410.c`
///
/// # Safety
/// The SDA slot must name an object whose `+0x10` field points at a vtable
/// with a callable `+0xC` slot.
///
/// # Confidence
/// medium - the indirection chain is literal; the object type is inferred.
pub unsafe fn fn_8016f410() {
    let object = platform::read_u32(SDA_BASE - 0x5220) as usize;
    let vtable = platform::read_u32(object + 0x10) as usize;
    let method = platform::read_u32(vtable + 0xC) as usize;
    platform::call0(method);
}

/// Returns `this+4`, or calls `0x80161700` when that word is zero.
///
/// # Original
/// - Ghidra: `FUN_801616c0` at `0x801616c0` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801616c0_FUN_801616c0.c`
///
/// # Confidence
/// high - literal load, null test and fallback call.
pub unsafe fn fn_801616c0(this: usize) -> u32 {
    let value = platform::read_u32(this + 4);
    if value == 0 {
        call0_u32(0x8016_1700)
    } else {
        value
    }
}

/// Returns `1` when `+0xB` is zero and bit 1 of `+0xF` is clear.
///
/// # Original
/// - Ghidra: `FUN_801620e0` at `0x801620e0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801620e0_FUN_801620e0.c`
///
/// # Confidence
/// high - two literal byte tests and the boolean result.
pub unsafe fn fn_801620e0(this: usize) -> u32 {
    if platform::read_u8(this + 0xB) == 0 && (platform::read_u8(this + 0xF) & 2) == 0 {
        1
    } else {
        0
    }
}

/// Returns `1` when `+0xB` is zero and bit 3 of `+0xF` is clear.
///
/// # Original
/// - Ghidra: `FUN_801621e0` at `0x801621e0` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801621e0_FUN_801621e0.c`
///
/// # Confidence
/// high - two literal byte tests and the boolean result.
pub unsafe fn fn_801621e0(this: usize) -> u32 {
    if platform::read_u8(this + 0xB) == 0 && (platform::read_u8(this + 0xF) & 8) == 0 {
        1
    } else {
        0
    }
}

/// Returns `[[this+0x10]+0x10]`, or zero when `+0x10` is null.
///
/// # Original
/// - Ghidra: `FUN_80162590` at `0x80162590` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162590_FUN_80162590.c`
///
/// # Confidence
/// high - literal null test and fixed offset read.
pub unsafe fn fn_80162590(this: usize) -> u32 {
    let object = platform::read_u32(this + 0x10);
    if object != 0 {
        platform::read_u32(object as usize + 0x10)
    } else {
        0
    }
}

/// Returns `[[this+0x14]+0x10]`, or zero when `+0x14` is null.
///
/// # Original
/// - Ghidra: `FUN_801625b0` at `0x801625b0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801625b0_FUN_801625b0.c`
///
/// # Confidence
/// high - literal null test and fixed offset read.
pub unsafe fn fn_801625b0(this: usize) -> u32 {
    let object = platform::read_u32(this + 0x14);
    if object != 0 {
        platform::read_u32(object as usize + 0x10)
    } else {
        0
    }
}

/// Calls `0x801629B0` and returns whether the result is non-zero.
///
/// # Original
/// - Ghidra: `FUN_80162a20` at `0x80162a20` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162a20_FUN_80162a20.c`
///
/// # Confidence
/// high - the `(-x | x) >> 31` idiom is literal.
pub unsafe fn fn_80162a20() -> u32 {
    let value = call0_u32(0x8016_29B0);
    (value.wrapping_neg() | value) >> 31
}

/// Installs `value` at `+8` and calls `0x802B6680` with the `+4` pointer.
///
/// # Original
/// - Ghidra: `FUN_80163660` at `0x80163660` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163660_FUN_80163660.c`
///
/// # Confidence
/// high - the guard, store and call are literal.
pub unsafe fn fn_80163660(this: usize, value: u32) -> u32 {
    if platform::read_u32(this + 8) != 0 {
        return 0;
    }
    platform::write_u32(this + 8, value);
    platform::call1(0x802B_6680, platform::read_u32(this + 4))
}

/// Calls virtual slot `+0x14` of the object at `this + 8`, if non-null.
///
/// # Original
/// - Ghidra: `FUN_801683e0` at `0x801683e0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801683e0_FUN_801683e0.c`
///
/// # Confidence
/// medium - the null test and slot are literal; the vtable type is inferred.
pub unsafe fn fn_801683e0(this: usize) {
    let object = platform::read_u32(this + 8) as usize;
    if object == 0 {
        return;
    }
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0x14) as usize;
    platform::call0(method);
}

/// Copies three words from `source` to `[this+4] + 0xDC..0xE4`.
///
/// # Original
/// - Ghidra: `FUN_8016a150` at `0x8016a150` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a150_FUN_8016a150.c`
///
/// # Confidence
/// high - three literal word copies after the `+4` load.
pub unsafe fn fn_8016a150(this: usize, source: usize) {
    let base = platform::read_u32(this + 4) as usize;
    platform::write_u32(base + 0xDC, platform::read_u32(source));
    platform::write_u32(base + 0xE0, platform::read_u32(source + 4));
    platform::write_u32(base + 0xE4, platform::read_u32(source + 8));
}

/// Calls `0x8016C5C0` and then `0x8016C730`.
///
/// # Original
/// - Ghidra: `FUN_8016c7f0` at `0x8016c7f0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c7f0_FUN_8016c7f0.c`
///
/// # Confidence
/// high - two literal no-argument calls.
pub unsafe fn fn_8016c7f0() {
    platform::call0(0x8016_C5C0);
    platform::call0(0x8016_C730);
}

/// Forwards three arguments to `0x802D9570` after the `r13-0x46B0` handle.
///
/// # Original
/// - Ghidra: `FUN_8016c830` at `0x8016c830` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c830_FUN_8016c830.c`
///
/// # Confidence
/// high - the SDA load, five-argument call and literal zero are visible.
pub unsafe fn fn_8016c830(a: u32, b: u32, c: u32) {
    platform::call5(
        0x802D_9570,
        platform::read_u32(SDA_BASE - 0x46B0),
        a,
        b,
        c,
        0,
    );
}

/// Copies three words from `this+0xC/+0x1C/+0x2C` into `out`.
///
/// # Original
/// - Ghidra: `FUN_8016f130` at `0x8016f130` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f130_FUN_8016f130.c`
///
/// # Confidence
/// high - three literal loads and stores.
pub unsafe fn fn_8016f130(this: usize, out: usize) {
    platform::write_u32(out, platform::read_u32(this + 0xC));
    platform::write_u32(out + 4, platform::read_u32(this + 0x1C));
    platform::write_u32(out + 8, platform::read_u32(this + 0x2C));
}

/// Installs the `r13-0x4810` object at `r13-0x5220` and runs three helpers.
///
/// # Original
/// - Ghidra: `FUN_8016f1f0` at `0x8016f1f0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f1f0_FUN_8016f1f0.c`
///
/// # Confidence
/// high - the SDA copy and three literal calls are visible.
pub unsafe fn fn_8016f1f0() {
    platform::write_u32(
        SDA_BASE - 0x5220,
        platform::read_u32(SDA_BASE - 0x4810),
    );
    platform::call0(0x8016_F550);
    platform::call0(0x8016_F220);
    platform::call0(0x8016_F410);
}

/// Returns a table byte for a flagged `index`, or `0xFFFFFFFF`.
///
/// # Original
/// - Ghidra: `FUN_8016f460` at `0x8016f460` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f460_FUN_8016f460.c`
///
/// # Confidence
/// medium - offsets and stride are literal; the table's meaning is inferred.
pub unsafe fn fn_8016f460(index: usize) -> u32 {
    if platform::read_u8(SDA_BASE - 0x520C + index) != 0 {
        platform::read_u8(0x8037_7D1C + index * 0x18) as u32
    } else {
        0xFFFF_FFFF
    }
}

/// Calls virtual slot `+0xC` of the `r13-0x5250` object with two arguments.
///
/// # Original
/// - Ghidra: `FUN_8016c5a0` at `0x8016c5a0` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c5a0_FUN_8016c5a0.c`
///
/// # Confidence
/// medium - the indirection chain and slot are literal; the type is inferred.
pub unsafe fn fn_8016c5a0(a: u32, b: u32) {
    let object = platform::read_u32(SDA_BASE - 0x5250) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0xC) as usize;
    platform::call3(method, object as u32, a, b);
}

// ===========================================================================
// 0x80160000 bucket: vtable, element-wrapper and table-reset helpers
// (wave 45)
// ===========================================================================

/// Calls virtual slot `+0x34` of the live context with the context's count.
///
/// # Original
/// - Ghidra: `FUN_80164e50` at `0x80164e50` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164e50_FUN_80164e50.c`
/// - observed: `(**(*piVar1 + 0x34))(piVar1,piVar1[0x39],param_1);`
///
/// # Arguments
/// - `value`: forwarded unchanged as the virtual call's third argument.
///
/// # Side effects
/// Runs whatever the context's `+0x34` slot does.
///
/// # Safety
/// The `r13 - 0x52B4` slot must hold a context whose vtable has a callable
/// `+0x34` slot.
///
/// # Confidence
/// high - the SDA load, slot index and argument order are literal.
pub unsafe fn fn_80164e50(value: u32) {
    let object = platform::read_u32(SDA_BASE - 0x52B4) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0x34) as usize;
    platform::call3(
        method,
        object as u32,
        platform::read_u32(object + 0xE4),
        value,
    );
}

/// Pops every live-context object through virtual slot `+0x38`.
///
/// # Original
/// - Ghidra: `FUN_80164e70` at `0x80164e70` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164e70_FUN_80164e70.c`
/// - observed: `while (piVar1[0x39] != 0) { slot+0x38(piVar1, count-1); }`
///
/// # Side effects
/// Repeated virtual calls until the context reports a zero object count.
///
/// # Safety
/// The `r13 - 0x52B4` slot must hold a context whose vtable has a callable
/// `+0x38` slot.
///
/// # Confidence
/// high - the loop condition and decremented index are literal.
pub unsafe fn fn_80164e70() {
    let object = platform::read_u32(SDA_BASE - 0x52B4) as usize;
    while platform::read_u32(object + 0xE4) != 0 {
        let count = platform::read_u32(object + 0xE4);
        let vtable = platform::read_u32(object) as usize;
        let method = platform::read_u32(vtable + 0x38) as usize;
        platform::call2(method, object as u32, count - 1);
    }
}

/// Runs the two-context update helper when either SDA slot is non-empty.
///
/// # Original
/// - Ghidra: `FUN_80164ec0` at `0x80164ec0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164ec0_FUN_80164ec0.c`
///
/// # Side effects
/// Calls `0x8025EFA0` unconditionally; calls `0x802D2E50` when either
/// `r13 - 0x52AC` or `r13 - 0x52A4` is non-zero.
///
/// # Safety
/// Both SDA slots must be readable.
///
/// # Confidence
/// high - one unconditional call and one guarded call are the whole body.
pub unsafe fn fn_80164ec0() {
    platform::call0(0x8025_EFA0);
    if platform::read_u32(SDA_BASE - 0x52AC) != 0
        || platform::read_u32(SDA_BASE - 0x52A4) != 0
    {
        platform::call0(0x802D_2E50);
    }
}

/// Returns the `+0xC` word of the object behind `*this`, or `0xFFFFFFFF`.
///
/// # Original
/// - Ghidra: `FUN_80164f00` at `0x80164f00` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164f00_FUN_80164f00.c`
///
/// # Returns
/// `0xFFFFFFFF` when `*this` is null or the lookup at `0x8023B200` returns
/// null; otherwise the looked-up object's `+0xC` word.
///
/// # Safety
/// `this` must point at a readable word and the lookup callee must return a
/// readable object.
///
/// # Confidence
/// medium - the null tests and offset are literal, but the export shows the
/// lookup with no explicit argument so its input is inferred from the caller.
pub unsafe fn fn_80164f00(this: usize) -> u32 {
    if platform::read_u32(this) == 0 {
        return 0xFFFF_FFFF;
    }
    let object = call0_u32(0x8023_B200) as usize;
    if object == 0 {
        0xFFFF_FFFF
    } else {
        platform::read_u32(object + 0xC)
    }
}

/// Resets channel 0 and then all sixteen channels.
///
/// # Original
/// - Ghidra: `FUN_80164f50` at `0x80164f50` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164f50_FUN_80164f50.c`
///
/// # Side effects
/// Calls `0x801C8110(0)` once, then `0x801C8130(i)` for `i = 0..0xF`.
///
/// # Confidence
/// high - one literal call and a fixed sixteen-iteration loop.
pub unsafe fn fn_80164f50() {
    platform::call1(0x801C_8110, 0);
    for index in 0..0x10u32 {
        platform::call1(0x801C_8130, index);
    }
}

/// Releases the `+0x24` buffer and clears the `+0x24`/`+0x28` pair.
///
/// # Original
/// - Ghidra: `FUN_801693a0` at `0x801693a0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801693a0_FUN_801693a0.c`
///
/// # Side effects
/// When `+0x24` is non-null, calls `0x802B8F70(ptr, 0)` and zeroes both words.
///
/// # Safety
/// `this + 0x24` must be readable and, when non-null, the pointer must be a
/// value accepted by `0x802B8F70`.
///
/// # Confidence
/// high - the guard, call and two stores are literal.
pub unsafe fn fn_801693a0(this: usize) {
    let buffer = platform::read_u32(this + 0x24);
    if buffer != 0 {
        platform::call2(0x802B_8F70, buffer, 0);
        platform::write_u32(this + 0x24, 0);
        platform::write_u32(this + 0x28, 0);
    }
}

/// Constructs the `0x80329658` sub-object and zeroes four trailing words.
///
/// # Original
/// - Ghidra: `FUN_80169560` at `0x80169560` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169560_FUN_80169560.c`
///
/// # Arguments
/// - `this`: object to initialise.
///
/// # Returns
/// `this`, so the caller can chain the construction.
///
/// # Side effects
/// Installs the `0x80329658` vtable at `+0x00`, constructs the nested object
/// at `+4` via `0x80168B40`, and zeroes `+0x24..+0x30`.
///
/// # Safety
/// `this` must be writable through `+0x30`.
///
/// # Confidence
/// high - the vtable constant, nested call and four zero stores are literal.
pub unsafe fn fn_80169560(this: usize) -> usize {
    platform::write_u32(this, 0x8032_9658);
    platform::call1(0x8016_8B40, (this + 4) as u32);
    platform::write_u32(this + 0x24, 0);
    platform::write_u32(this + 0x28, 0);
    platform::write_u32(this + 0x2C, 0);
    platform::write_u32(this + 0x30, 0);
    this
}

/// Returns `this`, freeing it first when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_801695c0` at `0x801695c0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801695c0_FUN_801695c0.c`
///
/// # Side effects
/// Calls `0x802B9280(this)` when `this` is non-null and `count > 0`.
///
/// # Safety
/// A non-null `this` must be a block accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards and single call are literal.
pub unsafe fn fn_801695c0(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Forwards one element to a virtual `+0x14` slot and resets the owner.
///
/// # Original
/// - Ghidra: `FUN_80169ac0` at `0x80169ac0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169ac0_FUN_80169ac0.c`
///
/// # Arguments
/// - `this`: object owning the `+0x2C` target.
/// - `a`: passed through to the virtual call.
/// - `src`: points at the word copied into the one-word argument buffer.
///
/// # Side effects
/// When `+0x2C` is non-null, calls its vtable slot `+0x14`; then always calls
/// `0x80168BB0(this + 4)`.
///
/// # Safety
/// `src` must be readable; the `+0x2C` object must have a callable `+0x14`
/// slot.
///
/// # Confidence
/// high - the local buffer initialisation, slot index and trailing call are
/// literal.
pub unsafe fn fn_80169ac0(this: usize, a: u32, src: usize) {
    let object = platform::read_u32(this + 0x2C) as usize;
    if object != 0 {
        let local = crate::platform::GuestLocal::new( platform::read_u32(src));
        let vtable = platform::read_u32(object) as usize;
        let method = platform::read_u32(vtable + 0x14) as usize;
        platform::call3(
            method,
            object as u32,
            a,
            (local.as_ptr() as *const u32) as usize as u32,
        );
    }
    platform::call1(0x8016_8BB0, (this + 4) as u32);
}

/// Destroys the element buffer and the `+0x30` owner.
///
/// # Original
/// - Ghidra: `FUN_80169c20` at `0x80169c20` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169c20_FUN_80169c20.c`
///
/// # Side effects
/// Calls `0x80168B80(this + 4)`; when `+0x28` is non-null, calls the vtable
/// slot `+0x10` of `[[this+0x30]+0x10]`; then zeroes `+0x28` and `+0x30`.
///
/// # Safety
/// The `+0x30` chain must resolve to an object with a callable `+0x10` slot.
///
/// # Confidence
/// medium - the three loads and slot are literal; the owner's type is
/// inferred from the neighbouring element helpers.
pub unsafe fn fn_80169c20(this: usize) {
    platform::call1(0x8016_8B80, (this + 4) as u32);
    if platform::read_u32(this + 0x28) != 0 {
        let owner = platform::read_u32(this + 0x30) as usize;
        let object = platform::read_u32(owner + 0x10) as usize;
        let vtable = platform::read_u32(object) as usize;
        let method = platform::read_u32(vtable + 0x10) as usize;
        platform::call0(method);
    }
    platform::write_u32(this + 0x28, 0);
    platform::write_u32(this + 0x30, 0);
}

/// Advances the nested object at `+4` when it has not finished.
///
/// # Original
/// - Ghidra: `FUN_80169c90` at `0x80169c90` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169c90_FUN_80169c90.c`
///
/// # Side effects
/// Calls `0x80168CB0(this + 4)`; when it returns zero, calls
/// `0x80168C10(this + 4)`.
///
/// # Safety
/// `this + 4` must be a valid nested-object address.
///
/// # Confidence
/// high - the test and two literal call targets are explicit.
pub unsafe fn fn_80169c90(this: usize) {
    if platform::call1(0x8016_8CB0, (this + 4) as u32) == 0 {
        platform::call1(0x8016_8C10, (this + 4) as u32);
    }
}

/// Constructs the `0x80329638` wrapper around a `0x80169560` sub-object.
///
/// # Original
/// - Ghidra: `FUN_80169cd0` at `0x80169cd0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169cd0_FUN_80169cd0.c`
///
/// # Returns
/// `this`, so the caller can chain the construction.
///
/// # Side effects
/// Calls `0x8016A2F0(this)`, installs the vtable at `+0x00`, then constructs the
/// nested object at `+0xC`.
///
/// # Safety
/// `this` must be writable through `+0x30`.
///
/// # Confidence
/// high - the base construction, vtable store and nested call are literal.
pub unsafe fn fn_80169cd0(this: usize) -> usize {
    platform::call1(0x8016_A2F0, this as u32);
    platform::write_u32(this, 0x8032_9638);
    platform::call1(0x8016_9560, (this + 0xC) as u32);
    this
}

/// Destroys the wrapper and optionally frees it.
///
/// # Original
/// - Ghidra: `FUN_80169d20` at `0x80169d20` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169d20_FUN_80169d20.c`
///
/// # Side effects
/// When `this` is non-null: clears the nested object at `+0xC` through
/// `0x801695C0(..., 0xFFFFFFFF)`, calls `0x8016A340(this, 0)`, and frees
/// `this` through `0x802B9280` when `count > 0`.
///
/// # Safety
/// A non-null `this` must be a live wrapper object.
///
/// # Confidence
/// high - all three calls, their arguments and the guards are literal.
pub unsafe fn fn_80169d20(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::call2(0x8016_95C0, (this + 0xC) as u32, 0xFFFF_FFFF);
        platform::call2(0x8016_A340, this as u32, 0);
        if count > 0 {
            platform::call1(0x802B_9280, this as u32);
        }
    }
    this
}

/// Sets a two-state flag on the object at `+4`.
///
/// # Original
/// - Ghidra: `FUN_8016a1a0` at `0x8016a1a0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a1a0_FUN_8016a1a0.c`
///
/// # Arguments
/// - `this`: wrapper whose `+4` field holds the target object.
/// - `flag`: when zero, the `+0x20` slot is called a second time with `1`.
///
/// # Side effects
/// Calls `0x8016A0F0(this, 2, 0)`, then virtual slot `+0xC` with three
/// arguments, then `0x8016A0F0(this, 2, 1)` when `flag == 0`.
///
/// # Safety
/// The `+4` object's vtable must have callable `+0xC` and `+0x20` slots.
///
/// # Confidence
/// medium - the call sites are literal, but `0x8016A0F0` is a jumptable
/// routine whose own export hides the forwarded arguments.
pub unsafe fn fn_8016a1a0(this: usize, flag: i32) {
    platform::call3(0x8016_A0F0, this as u32, 2, 0);
    let object = platform::read_u32(this + 4) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0xC) as usize;
    platform::call4(method, object as u32, 1, 0, 0);
    if flag == 0 {
        platform::call3(0x8016_A0F0, this as u32, 2, 1);
    }
}

/// Applies a value to the `+4` handle through two volume helpers.
///
/// # Original
/// - Ghidra: `FUN_8016a2a0` at `0x8016a2a0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a2a0_FUN_8016a2a0.c`
///
/// # Arguments
/// - `this`: wrapper whose `+4` field holds the audio handle.
/// - `_unused`: the middle register argument; the export never reads it.
/// - `value`: forwarded to `0x80257670`.
///
/// # Side effects
/// Calls `0x80257640(handle)` and `0x80257670(handle, value)`.
///
/// # Safety
/// `+4` must hold a handle accepted by both callees.
///
/// # Confidence
/// high - both calls and the unused argument are literal.
pub unsafe fn fn_8016a2a0(this: usize, _unused: u32, value: u32) {
    let handle = platform::read_u32(this + 4);
    platform::call1(0x8025_7640, handle);
    platform::call2(0x8025_7670, handle, value);
}

/// Constructs the `0x80329688` object on top of `0x8016A010`.
///
/// # Original
/// - Ghidra: `FUN_8016a2f0` at `0x8016a2f0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a2f0_FUN_8016a2f0.c`
///
/// # Returns
/// `this`, so the caller can chain the construction.
///
/// # Side effects
/// Calls `0x8016A010(this)`, zeroes `+8`, and installs the vtable at `+0x00`.
///
/// # Safety
/// `this` must be writable through `+8`.
///
/// # Confidence
/// high - the base call, zero store and vtable store are literal.
pub unsafe fn fn_8016a2f0(this: usize) -> usize {
    platform::call1(0x8016_A010, this as u32);
    platform::write_u32(this + 8, 0);
    platform::write_u32(this, 0x8032_9688);
    this
}

/// Returns `this`, resetting and optionally freeing it.
///
/// # Original
/// - Ghidra: `FUN_8016a340` at `0x8016a340` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a340_FUN_8016a340.c`
///
/// # Side effects
/// When `this` is non-null, calls `0x80168100(this, 0)`; when `count > 0`,
/// frees `this` through `0x802B9280`.
///
/// # Safety
/// A non-null `this` must be a live object accepted by both callees.
///
/// # Confidence
/// high - the comma expression still implies one guarded reset call and one
/// guarded free.
pub unsafe fn fn_8016a340(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::call2(0x8016_8100, this as u32, 0);
        if count > 0 {
            platform::call1(0x802B_9280, this as u32);
        }
    }
    this
}

/// Creates the `+4` handle through one of two source helpers.
///
/// # Original
/// - Ghidra: `FUN_8016a3a0` at `0x8016a3a0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a3a0_FUN_8016a3a0.c`
///
/// # Arguments
/// - `this`: object whose `+4` handle is replaced.
/// - `src`: points at the one-word descriptor passed to the factory.
/// - `group`: sound group; `0` selects the `r13 - 0x52B8` default.
/// - `mode`: `0` uses the four-argument factory, non-zero the three-argument
///   variant.
/// - `arg4`: extra factory argument, used only in mode `0`.
/// - `out`: scratch buffer; `0` selects a twelve-byte local buffer.
///
/// # Returns
/// `1` when a handle was produced, `0` otherwise.
///
/// # Side effects
/// Stores the new handle at `+4` and, when non-null, sets both `0x7F` fields
/// through `0x80257640` / `0x80257670`.
///
/// # Safety
/// `src` must be readable and `out`, when non-zero, writable for twelve bytes.
///
/// # Confidence
/// medium - the branch tree and offsets are literal; the callees' exact
/// parameter roles come from the call shape rather than a symbol table.
pub unsafe fn fn_8016a3a0(
    this: usize,
    src: usize,
    group: u32,
    mode: u32,
    arg4: u32,
    out: usize,
) -> u32 {
    const DEFAULT_GROUP: usize = SDA_BASE - 0x52B8;

    let group = if group == 0 {
        platform::read_u32(DEFAULT_GROUP)
    } else {
        group
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 12]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };

    let mut descriptor = crate::platform::GuestLocal::<u32>::new(unsafe { core::mem::zeroed() });
    let handle = if mode == 0 {
        *descriptor = platform::read_u32(src);
        platform::call4(
            0x8025_A380,
            group,
            out as u32,
            (descriptor.as_ptr() as *const u32) as usize as u32,
            arg4,
        )
    } else {
        *descriptor = platform::read_u32(src);
        platform::call3(
            0x8025_CA20,
            group,
            out as u32,
            (descriptor.as_ptr() as *const u32) as usize as u32,
        )
    };

    platform::write_u32(this + 4, handle);
    let stored = platform::read_u32(this + 4);
    if stored != 0 {
        platform::call2(0x8025_7640, stored, 0x7F);
        platform::call2(0x8025_7670, stored, 0x7F);
    }
    (stored != 0) as u32
}

/// Returns `this`, freeing it first when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_8016af90` at `0x8016af90` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016af90_FUN_8016af90.c`
///
/// # Side effects
/// Calls `0x802B9280(this)` when `this` is non-null and `count > 0`.
///
/// # Safety
/// A non-null `this` must be a block accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards and single call are literal.
pub unsafe fn fn_8016af90(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Returns `this`, freeing it first when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_8016afd0` at `0x8016afd0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016afd0_FUN_8016afd0.c`
///
/// # Side effects
/// Calls `0x802B9280(this)` when `this` is non-null and `count > 0`.
///
/// # Safety
/// A non-null `this` must be a block accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards and single call are literal.
pub unsafe fn fn_8016afd0(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Returns `this`, freeing it first when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_8016b010` at `0x8016b010` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b010_FUN_8016b010.c`
///
/// # Side effects
/// Calls `0x802B9280(this)` when `this` is non-null and `count > 0`.
///
/// # Safety
/// A non-null `this` must be a block accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards and single call are literal.
pub unsafe fn fn_8016b010(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Returns `this`, freeing it first when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_8016b050` at `0x8016b050` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b050_FUN_8016b050.c`
///
/// # Side effects
/// Calls `0x802B9280(this)` when `this` is non-null and `count > 0`.
///
/// # Safety
/// A non-null `this` must be a block accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards and single call are literal.
pub unsafe fn fn_8016b050(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Fills a twelve-word matrix with the `r2 - 0x5358` constant.
///
/// # Original
/// - Ghidra: `FUN_8016f150` at `0x8016f150` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f150_FUN_8016f150.c`
///
/// # Side effects
/// Writes the same word to `this + 0x00..0x2C`.
///
/// # Safety
/// `this` must be writable for `0x30` bytes.
///
/// # Confidence
/// high - twelve identical stores of the same SDA2 word are literal.
pub unsafe fn fn_8016f150(this: usize) {
    let value = platform::read_u32(SDA2_BASE - 0x5358);
    platform::write_u32(this, value);
    platform::write_u32(this + 0x04, value);
    platform::write_u32(this + 0x08, value);
    platform::write_u32(this + 0x0C, value);
    platform::write_u32(this + 0x10, value);
    platform::write_u32(this + 0x14, value);
    platform::write_u32(this + 0x18, value);
    platform::write_u32(this + 0x1C, value);
    platform::write_u32(this + 0x20, value);
    platform::write_u32(this + 0x24, value);
    platform::write_u32(this + 0x28, value);
    platform::write_u32(this + 0x2C, value);
}

/// Clears one `0x18`-byte entry of the table at `0x80377D08`.
///
/// # Original
/// - Ghidra: `FUN_8016f500` at `0x8016f500` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f500_FUN_8016f500.c`
///
/// # Arguments
/// - `index`: table slot, multiplied by the `0x18` stride.
///
/// # Side effects
/// Zeroes the entry's five words and four bytes, plus the parallel byte in
/// `r13 - 0x520C`.
///
/// # Safety
/// `index` must be in the table's valid range.
///
/// # Confidence
/// high - the stride, all eight zero stores and the SDA byte are literal.
pub unsafe fn fn_8016f500(index: usize) {
    let entry = 0x8037_7D08 + index * 0x18;
    platform::write_u32(entry, 0);
    platform::write_u32(entry + 0x04, 0);
    platform::write_u32(entry + 0x08, 0);
    platform::write_u32(entry + 0x0C, 0);
    platform::write_u32(entry + 0x10, 0);
    platform::write_u8(entry + 0x14, 0);
    platform::write_u8(entry + 0x15, 0);
    platform::write_u8(entry + 0x16, 0);
    platform::write_u8(SDA_BASE - 0x520C + index, 0);
    platform::write_u8(entry + 0x17, 0);
}

/// Clears all four table entries.
///
/// # Original
/// - Ghidra: `FUN_8016f550` at `0x8016f550` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f550_FUN_8016f550.c`
///
/// # Side effects
/// Calls `0x8016F500(i)` for `i = 0..3`.
///
/// # Confidence
/// high - one literal call inside a fixed four-iteration loop.
pub unsafe fn fn_8016f550() {
    for index in 0..4usize {
        platform::call1(0x8016_F500, index as u32);
    }
}

/// Starts one table entry's task and clears it on failure.
///
/// # Original
/// - Ghidra: `FUN_8016f5d0` at `0x8016f5d0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f5d0_FUN_8016f5d0.c`
///
/// # Arguments
/// - `index`: table slot passed to the task starter.
///
/// # Returns
/// `0xFFFFFFFF` when the starter failed, otherwise the starter's result.
///
/// # Side effects
/// Calls `0x801E12C0(index, &table[index], 0x8016F590)`; on `-1` it clears the
/// slot through `0x8016F500`.
///
/// # Safety
/// `index` must be in range for both the table and the callback.
///
/// # Confidence
/// medium - the task-starter call shape is literal, but the identity of the
/// callback at `0x8016F590` is not recovered.
pub unsafe fn fn_8016f5d0(index: i32) -> i32 {
    let result = platform::call3(
        0x801E_12C0,
        index as u32,
        (0x8037_7D68 + index as usize * 0x18) as u32,
        0x8016_F590,
    ) as i32;
    if result == -1 {
        platform::call1(0x8016_F500, index as u32);
    }
    result
}

/// Returns `this`, freeing it first when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_8016f6e0` at `0x8016f6e0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f6e0_FUN_8016f6e0.c`
///
/// # Side effects
/// Calls `0x802B9280(this)` when `this` is non-null and `count > 0`.
///
/// # Safety
/// A non-null `this` must be a block accepted by `0x802B9280`.
///
/// # Confidence
/// high - the two guards and single call are literal.
pub unsafe fn fn_8016f6e0(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Initialises a table entry and marks both state bytes live.
///
/// # Original
/// - Ghidra: `FUN_8016f7d0` at `0x8016f7d0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f7d0_FUN_8016f7d0.c`
///
/// # Arguments
/// - `this`: entry being initialised.
/// - `_arg2`, `_arg3`: register arguments the export never reads.
/// - `arg4`: forwarded to `0x8016F830`.
///
/// # Side effects
/// Calls `0x8016FE90()`, `0x8016F830(this, arg4)`, `0x80228DD0(this + 8, 0x14)`,
/// then sets `+0x14` and `+0x15` to `1`.
///
/// # Safety
/// `this` must be writable through `+0x15`.
///
/// # Confidence
/// high - all three calls, the buffer size and the two byte stores are
/// literal.
pub unsafe fn fn_8016f7d0(this: usize, _arg2: u32, _arg3: u32, arg4: u32) {
    platform::call0(0x8016_FE90);
    platform::call2(0x8016_F830, this as u32, arg4);
    platform::call2(0x8022_8DD0, (this + 8) as u32, 0x14);
    platform::write_u8(this + 0x14, 1);
    platform::write_u8(this + 0x15, 1);
}

// ===========================================================================
// Intrusive list and small state machine: 0x80161700..0x80161A70 (wave 52)
//
// Nodes are `{next, prev, payload, key}` with `+0x00` next, `+0x04` prev and
// `+0x0C` the payload word. The state machine at the end of the run stores a
// `u16` state at `+0x00`, a `u16` cursor at `+0x02`, an entry array pointer at
// `+0x04` and a vtable at `+0x10`.
// ===========================================================================

/// Returns the first payload word found while walking a node chain.
///
/// The function checks `this` first and then follows the `+0x00` next links
/// until a node has a non-zero `+0x0C` word.
///
/// # Original
/// - Ghidra: `FUN_80161700` at `0x80161700` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161700_FUN_80161700.c`
/// - observed: `if (param_1[3] != 0) return param_1[3]; param_1 = *param_1;
///   while (true) { if (param_1 == 0) return 0; if (param_1[3] != 0) break;
///   param_1 = *param_1; } return param_1[3];`
///
/// # Arguments
/// - `node`: first node of the chain, or zero.
///
/// # Returns
/// The payload word, or `0` when the chain is exhausted.
///
/// # Safety
/// Every non-null node must be readable through `+0x0C`.
///
/// # Confidence
/// high - the two payload tests and the next-link walk are literal.
pub unsafe fn fn_80161700(node: usize) -> u32 {
    if platform::read_u32(node + 0x0C) != 0 {
        return platform::read_u32(node + 0x0C);
    }
    let mut current = platform::read_u32(node) as usize;
    loop {
        if current == 0 {
            return 0;
        }
        if platform::read_u32(current + 0x0C) != 0 {
            return platform::read_u32(current + 0x0C);
        }
        current = platform::read_u32(current) as usize;
    }
}

/// Appends `node` at the tail of a singly-linked list and stores its key.
///
/// A bad argument calls the empty guard [`LIST_BAD_ARG`] exactly as the
/// original does; the guard is a no-op, so the later dereference still happens.
///
/// # Original
/// - Ghidra: `FUN_80161750` at `0x80161750` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161750_FUN_80161750.c`
/// - observed: `if (param_1 == 0 || param_2 == 0 || param_3 == 0) FUN_80161740();
///   ... *param_2 = param_3; param_2[1] = 0;`
///
/// # Arguments
/// - `list`: address of the head pointer.
/// - `node`: node to append; `+0x00` key, `+0x04` next.
/// - `key`: key word stored at `node + 0x00`.
///
/// # Side effects
/// Links the node at the tail (or head) and clears its next word.
///
/// # Safety
/// `list` must be readable/writable for one word and `node` for eight bytes.
/// The guard does not make null pointers safe.
///
/// # Confidence
/// high - the guard, head path, tail walk and two stores are literal.
pub unsafe fn fn_80161750(list: usize, node: usize, key: u32) {
    if list == 0 || node == 0 || key == 0 {
        platform::call0(LIST_BAD_ARG);
    }
    if node == 0 {
        return;
    }
    let mut current = platform::read_u32(list) as usize;
    if current == 0 {
        platform::write_u32(list, node as u32);
    } else {
        loop {
            let next = platform::read_u32(current + 4) as usize;
            if next == 0 {
                platform::write_u32(current + 4, node as u32);
                break;
            }
            if next == node {
                return;
            }
            current = next;
        }
    }
    platform::write_u32(node, key);
    platform::write_u32(node + 4, 0);
}

/// Removes `node` from a singly-linked list when its key matches.
///
/// The key check is `*node == key`; the search then walks the `+0x04` links.
/// The node's two words are cleared only when it was actually found.
///
/// # Original
/// - Ghidra: `FUN_80161800` at `0x80161800` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161800_FUN_80161800.c`
/// - observed: `if ((param_2 != 0) && (*param_2 == param_3)) { ... *param_2 = 0;
///   param_2[1] = 0; }`
///
/// # Arguments
/// - `list`: address of the head pointer.
/// - `node`: node to unlink.
/// - `key`: expected key at `node + 0x00`.
///
/// # Side effects
/// Rewrites the predecessor's next word and zeroes both node words.
///
/// # Safety
/// `node` must be a valid node and, when found, be linked in `list`.
///
/// # Confidence
/// high - the key test, head case, walk and two clears are literal.
pub unsafe fn fn_80161800(list: usize, node: usize, key: u32) {
    if list == 0 || node == 0 || key == 0 {
        platform::call0(LIST_BAD_ARG);
    }
    if node == 0 || platform::read_u32(node) != key {
        return;
    }
    let mut current = platform::read_u32(list) as usize;
    if current == node {
        platform::write_u32(list, platform::read_u32(node + 4));
    } else {
        loop {
            let next = platform::read_u32(current + 4) as usize;
            if next == node {
                platform::write_u32(current + 4, platform::read_u32(node + 4));
                break;
            }
            if next == 0 {
                return;
            }
            current = next;
        }
    }
    platform::write_u32(node, 0);
    platform::write_u32(node + 4, 0);
}

/// Unlinks every node of a list and zeroes each node's two link words.
///
/// # Original
/// - Ghidra: `FUN_801618c0` at `0x801618c0` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801618c0_FUN_801618c0.c`
/// - observed: `puVar1 = *param_1; *param_1 = 0; while (puVar1 != 0) {
///   puVar2 = puVar1[1]; *puVar1 = 0; puVar1[1] = 0; puVar1 = puVar2; }`
///
/// # Arguments
/// - `list`: address of the head pointer; it is cleared first.
///
/// # Side effects
/// Writes the list head and every reachable node.
///
/// # Safety
/// Every node in the chain must be writable. A null `list` still dereferences
/// after the no-op guard.
///
/// # Confidence
/// high - the detach, walk and two clears are literal.
pub unsafe fn fn_801618c0(list: usize) {
    if list == 0 {
        platform::call0(LIST_BAD_ARG);
    }
    let mut current = platform::read_u32(list) as usize;
    platform::write_u32(list, 0);
    while current != 0 {
        let next = platform::read_u32(current + 4) as usize;
        platform::write_u32(current, 0);
        platform::write_u32(current + 4, 0);
        current = next;
    }
}

/// Pops the state machine's entry array from the end down to index zero.
///
/// Each entry is handed to virtual slot `+0x10` of the object at `+0x10`.
///
/// # Original
/// - Ghidra: `FUN_80161920` at `0x80161920` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161920_FUN_80161920.c`
/// - observed: `if ((*(int *)(param_1 + 2) != 0) && (*param_1 != 0)) {
///   param_1[1] = param_1[1] + -1; while (-1 < param_1[1]) {
///   (**(code **)(*(int *)(param_1 + 8) + 0x10))(param_1, array[index]); ... } }`
///
/// # Arguments
/// - `this`: state object; `+0x00` state, `+0x02` cursor, `+0x04` array,
///   `+0x10` vtable.
///
/// # Side effects
/// Calls the pop slot once per entry and leaves the cursor at `0xFFFF`.
///
/// # Safety
/// The array and vtable must be live for the cursor range.
///
/// # Confidence
/// medium - the loop and offsets are literal, but the slot's role is inferred
/// from the paired append in the neighbouring code.
pub unsafe fn fn_80161920(this: usize) {
    let array = platform::read_u32(this + 0x04) as usize;
    if array == 0 || platform::read_u16(this) == 0 {
        return;
    }
    let mut index = platform::read_u16(this + 0x02) as i32 - 1;
    platform::write_u16(this + 0x02, index as u16);
    while index >= 0 {
        let entry = platform::read_u32(array + index as usize * 4);
        let vtable = platform::read_u32(this + 0x10) as usize;
        let pop = platform::read_u32(vtable + 0x10) as usize;
        platform::call2(pop, this as u32, entry);
        index -= 1;
        platform::write_u16(this + 0x02, index as u16);
    }
}

/// Advances the state machine over an entry array until a non-zero entry or a
/// state change.
///
/// The virtual `+0x0C` slot is polled; while it returns `2` the cursor moves to
/// the next entry and the `+0x0E` word is cleared. With `force == 0` the loop
/// stops after one step.
///
/// # Original
/// - Ghidra: `FUN_801619b0` at `0x801619b0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801619b0_FUN_801619b0.c`
/// - observed: `if ((array == 0) || (*param_1 == 2)) iVar1 = 2; else { do {
///   if (array[index] == 0) break; sVar2 = vtable[0xC](param_1); *param_1 = sVar2;
///   if (sVar2 != 2) break; param_1[7] = 0; param_1[1]++; } while (param_2 != 0);
///   iVar1 = *param_1; }`
///
/// # Arguments
/// - `this`: state object (see [`fn_80161920`] for the layout).
/// - `force`: non-zero to keep advancing while the slot returns `2`.
///
/// # Returns
/// The current `u16` state (always `2` when the array is absent).
///
/// # Side effects
/// Stores the returned state at `+0x00`, moves the cursor and clears `+0x0E`.
///
/// # Safety
/// The array and vtable must be valid for the cursor range.
///
/// # Confidence
/// medium - the loop is literal; the slot and the `+0x0E` word are inferred
/// state-machine fields.
pub unsafe fn fn_801619b0(this: usize, force: u32) -> u32 {
    let array = platform::read_u32(this + 0x04) as usize;
    if array == 0 || platform::read_u16(this) == 2 {
        return 2;
    }
    let mut index = platform::read_u16(this + 0x02) as usize;
    loop {
        if platform::read_u32(array + index * 4) == 0 {
            break;
        }
        let vtable = platform::read_u32(this + 0x10) as usize;
        let step = platform::read_u32(vtable + 0x0C) as usize;
        let state = platform::call1(step, this as u32) as u16;
        platform::write_u16(this, state);
        if state != 2 {
            break;
        }
        platform::write_u16(this + 0x0E, 0);
        index += 1;
        platform::write_u16(this + 0x02, index as u16);
        if force == 0 {
            break;
        }
    }
    platform::read_u16(this) as u32
}

/// Runs the state machine until it leaves state `1`.
///
/// # Original
/// - Ghidra: `FUN_80161a70` at `0x80161a70` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161a70_FUN_80161a70.c`
/// - observed: `if (*param_1 == 1) { sVar1 = FUN_801619b0(param_1,0);
///   *param_1 = sVar1; } return *param_1 != 1;`
///
/// # Returns
/// `true` when the state is no longer `1`.
///
/// # Side effects
/// Stores the advanced state back into `this + 0x00`.
///
/// # Safety
/// `this` must be a live state object.
///
/// # Confidence
/// high - the guard, the call and the comparison are literal.
pub unsafe fn fn_80161a70(this: usize) -> bool {
    if platform::read_u16(this) == 1 {
        let state = fn_801619b0(this, 0) as u16;
        platform::write_u16(this, state);
    }
    platform::read_u16(this) != 1
}

/// Destroys the sound bank's list, restores the base vtable and optionally
/// frees the bank.
///
/// Every node reachable from `+0x54` is unlinked with [`fn_80162ca0`], the
/// header at `+0x54` is re-read each round (the unlink clears the owner's
/// pointer), and the base vtable is stored at `+0x60`.
///
/// # Original
/// - Ghidra: `FUN_80161c40` at `0x80161c40` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161c40_FUN_80161c40.c`
/// - observed: `iVar1 = *(int *)(param_1 + 0x54);
///   *(undefined **)(param_1 + 0x60) = &DAT_80329410;
///   while (iVar1 != 0) { FUN_80162ca0(iVar1); iVar1 = *(int *)(param_1 + 0x54); }
///   if (0 < param_2) fun_80162920(param_1);`
///
/// # Arguments
/// - `this`: sound-bank object, or zero.
/// - `deleting`: positive to free the bank after teardown.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Unlinks the whole `+0x54` list, overwrites the vtable at `+0x60` and may
/// free `this` through [`fn_80162920`].
///
/// # Safety
/// A non-null `this` must be a live bank whose `+0x54` list can be unlinked;
/// after a positive `deleting` it must not be used again.
///
/// # Confidence
/// high - the guard, vtable store, loop and both calls are literal and both
/// callees already have bodies in this file.
pub unsafe fn fn_80161c40(this: usize, deleting: i32) -> usize {
    if this != 0 {
        let mut node = platform::read_u32(this + 0x54) as usize;
        platform::write_u32(this + 0x60, 0x8032_9410);
        while node != 0 {
            fn_80162ca0(node);
            node = platform::read_u32(this + 0x54) as usize;
        }
        if deleting > 0 {
            fn_80162920(this as u32);
        }
    }
    this
}

// ===========================================================================
// Sound-object helpers: 0x80163A60..0x801644D0 (wave 52)
//
// `+0x10` holds the embedded object whose vtable is used for the virtual
// calls; the outer object keeps its flag word at `+0x94`, a circular list
// header at `+0x10` of the list owner, and a one-byte enable bit at `+0x0C`.
// ===========================================================================

/// Forwards the `+0x94` bit-0 flag to virtual slot `+0x38` of `this + 0x10`.
///
/// # Original
/// - Ghidra: `FUN_80163a60` at `0x80163a60` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163a60_FUN_80163a60.c`
/// - observed: `(**(code **)(*(int *)(param_1 + 0x10) + 0x38))
///   ((int *)(param_1 + 0x10), (*(uint *)(param_1 + 0x94) & 1) != 0);`
///
/// # Arguments
/// - `this`: outer object; `+0x10` embedded object, `+0x94` flag word.
///
/// # Side effects
/// Calls the embedded object's slot with itself and the boolean.
///
/// # Safety
/// `this + 0x10` must hold a vtable with a callable `+0x38` slot.
///
/// # Confidence
/// medium - the call is literal, but Ghidra printed it as an indirect jump.
pub unsafe fn fn_80163a60(this: usize) {
    let object = this + 0x10;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0x38) as usize;
    let enabled = (platform::read_u32(this + 0x94) & 1) != 0;
    platform::call2(method, object as u32, enabled as u32);
}

/// Reads the embedded transform, stores four words, then applies it.
///
/// # Original
/// - Ghidra: `FUN_80163a90` at `0x80163a90` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163a90_FUN_80163a90.c`
/// - observed: `FUN_802ad520(&local_18,param_1 + 0x10);
///   *(param_1 + 100) = local_18; ... (**(code **)(*(int *)(param_1 + 0x10)
///   + 0x30))(param_1 + 0x10,param_1 + 0x30);`
///
/// # Arguments
/// - `this`: outer object; `+0x10` embedded object, `+0x30` destination.
///
/// # Side effects
/// Writes `+0x64..0x70` from the query and calls virtual slot `+0x30`.
///
/// # Safety
/// The embedded object must be live and `this + 0x30` writable.
///
/// # Confidence
/// medium - the four stores and the call are literal; the query's output
/// layout is inferred from the four returned words.
pub unsafe fn fn_80163a90(this: usize) {
    let object = this + 0x10;
    let mut transform = crate::platform::GuestLocal::new( [0u32; 4]);
    platform::call2(
        FN_TRANSFORM_QUERY,
        transform.as_mut_ptr() as u32,
        object as u32,
    );
    platform::write_u32(this + 0x64, transform[0]);
    platform::write_u32(this + 0x68, transform[1]);
    platform::write_u32(this + 0x6C, transform[2]);
    platform::write_u32(this + 0x70, transform[3]);
    let vtable = platform::read_u32(object) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x30) as usize,
        object as u32,
        (this + 0x30) as u32,
    );
}

/// Creates a child through `0x801636A0` and publishes it through the embedded
/// object's `+0x0C` slot.
///
/// A non-zero `+0x20` word short-circuits with `1`. When `source` is zero the
/// fallback at `+0x84` is used; if that is also zero the result is `0`.
///
/// # Original
/// - Ghidra: `FUN_80163c70` at `0x80163c70` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163c70_FUN_80163c70.c`
/// - observed: `if (*(int *)(param_1 + 0x20) == 0) { if ((param_3 == 0) &&
///   (param_3 = *(int *)(param_1 + 0x84), param_3 == 0)) iVar1 = 0; else {
///   iVar1 = FUN_801636a0(param_3,0,param_2); ... } } else iVar1 = 1;`
///
/// # Arguments
/// - `this`: outer object; `+0x08` callback vtable, `+0x10` embedded object.
/// - `arg`: second argument of the dispatch call.
/// - `source`: optional source object; `+0x84` is the fallback.
///
/// # Returns
/// `1` when already busy, otherwise the slot's status, or `0` on failure.
///
/// # Side effects
/// Creates the child, calls virtual `+0x0C`, and on success calls the `+0x08`
/// object's `+0x10` slot.
///
/// # Safety
/// All objects must be live; the fallback at `+0x84` must be readable.
///
/// # Confidence
/// medium - control flow and offsets are literal; the dispatch target is a
/// jumptable whose argument list comes from the caller.
pub unsafe fn fn_80163c70(this: usize, arg: u32, source: usize) -> u32 {
    if platform::read_u32(this + 0x20) != 0 {
        return 1;
    }
    let mut parent = source;
    if parent == 0 {
        parent = platform::read_u32(this + 0x84) as usize;
        if parent == 0 {
            return 0;
        }
    }
    let child = platform::call3(FN_DISPATCH_3, parent as u32, 0, arg);
    if child == 0 {
        return 0;
    }
    let object = this + 0x10;
    let vtable = platform::read_u32(object) as usize;
    let result = platform::call3(
        platform::read_u32(vtable + 0x0C) as usize,
        object as u32,
        child,
        platform::read_u32(parent + 4),
    );
    if result != 0 {
        let callback = platform::read_u32(this + 8) as usize;
        platform::call1(
            platform::read_u32(callback + 0x10) as usize,
            this as u32,
        );
    }
    result
}

/// Applies an operation through `0x80163D40` and maps its result to a flag.
///
/// The final expression `(2 - r | r - 2) >> 31` is `1` exactly when the helper
/// returns something other than `2`.
///
/// # Original
/// - Ghidra: `FUN_80163e10` at `0x80163e10` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163e10_FUN_80163e10.c`
/// - observed: `iVar1 = FUN_80163d40(param_1, *(undefined4 *)(param_1 + 0x20),
///   param_2, param_3, param_4); return (2U - iVar1 | iVar1 - 2U) >> 0x1f;`
///
/// # Arguments
/// - `this`: outer object; `+0x20` is the first helper argument.
/// - `a`, `b`, `c`: forwarded arguments.
///
/// # Returns
/// `1` when the helper's result is not `2`, otherwise `0`.
///
/// # Safety
/// The object at `+0x20` and the three forwarded values must be valid for the
/// recursive helper.
///
/// # Confidence
/// medium - the arithmetic is literal; the helper itself is untranslated, so
/// its role is inferred.
pub unsafe fn fn_80163e10(this: usize, a: u32, b: u32, c: u32) -> u32 {
    let result = platform::call5(
        FN_RECURSIVE_APPLY,
        this as u32,
        platform::read_u32(this + 0x20),
        a,
        b,
        c,
    );
    (2u32.wrapping_sub(result) | result.wrapping_sub(2)) >> 31
}

/// Releases the pointer at `+0x14`, clears it and reports success.
///
/// # Original
/// - Ghidra: `FUN_80163fb0` at `0x80163fb0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163fb0_FUN_80163fb0.c`
/// - observed: `FUN_802b9290(*(undefined4 *)(param_1 + 0x14));
///   *(undefined4 *)(param_1 + 0x14) = 0; return 1;`
///
/// # Arguments
/// - `this`: object; `+0x14` holds an allocation or zero.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Frees the allocation and clears the field.
///
/// # Safety
/// The word at `+0x14` must be accepted by `0x802B9290`.
///
/// # Confidence
/// medium - the call, clear and literal `1` are visible; the release role is
/// inferred from `0x802B9290`'s own body.
pub unsafe fn fn_80163fb0(this: usize) -> u32 {
    platform::call1(HEAP_RELEASE, platform::read_u32(this + 0x14));
    platform::write_u32(this + 0x14, 0);
    1
}

/// Walks a circular list and calls virtual slot `+0x5C` on each matching node.
///
/// Each node stores the object at `+0x08`; [`FN_KEY_LOOKUP`] decides whether
/// the node matches, and the match's `+0x08` word becomes the second argument.
///
/// # Original
/// - Ghidra: `FUN_80164080` at `0x80164080` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164080_FUN_80164080.c`
/// - observed: `for (puVar2 = *(undefined4 **)(param_2 + 0x10);
///   puVar2 != (undefined4 *)(param_2 + 0x10); puVar2 = (undefined4 *)*puVar2) {
///   iVar1 = FUN_801641b0(param_1,puVar2[2],0); if (iVar1 != 0)
///   (**(code **)(*(int *)puVar2[2] + 0x5c))(...); }`
///
/// # Arguments
/// - `key`: lookup key.
/// - `list`: owner whose `+0x10` is the circular-list header.
/// - `value`: forwarded third argument.
///
/// # Side effects
/// One virtual call per matching node.
///
/// # Safety
/// The list must be well-formed and every node object live.
///
/// # Confidence
/// medium - the loop and four arguments are literal; the slot role is
/// inferred.
pub unsafe fn fn_80164080(key: u32, list: usize, value: u32) {
    let head = list + 0x10;
    let mut node = platform::read_u32(head) as usize;
    while node != head {
        let object = platform::read_u32(node + 8);
        let found = platform::call3(FN_KEY_LOOKUP, key, object, 0);
        if found != 0 {
            let vtable = platform::read_u32(object as usize) as usize;
            platform::call4(
                platform::read_u32(vtable + 0x5C) as usize,
                object,
                platform::read_u32(found as usize + 8),
                value,
                0,
            );
        }
        node = platform::read_u32(node) as usize;
    }
}

/// Applies a double value to every matching node of a circular list.
///
/// # Original
/// - Ghidra: `FUN_80164120` at `0x80164120` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164120_FUN_80164120.c`
/// - observed: `for (puVar3 = *(undefined4 **)(param_3 + 0x10); ...) {
///   uVar2 = puVar3[2]; iVar1 = FUN_801641b0(param_2,uVar2,0);
///   if (iVar1 != 0) FUN_801642c0(param_1,param_2,uVar2); }`
///
/// # Arguments
/// - `value`: the `f64` value forwarded to [`fn_801642c0`].
/// - `key`: lookup key.
/// - `list`: owner whose `+0x10` is the circular-list header.
///
/// # Side effects
/// One [`fn_801642c0`] call per matching node.
///
/// # Safety
/// The list and all node objects must be live.
///
/// # Confidence
/// medium - the loop and key lookup are literal; the value semantics come from
/// the callee.
pub unsafe fn fn_80164120(value: f64, key: u32, list: usize) {
    let head = list + 0x10;
    let mut node = platform::read_u32(head) as usize;
    while node != head {
        let object = platform::read_u32(node + 8);
        if platform::call3(FN_KEY_LOOKUP, key, object, 0) != 0 {
            fn_801642c0(value, key, object as usize);
        }
        node = platform::read_u32(node) as usize;
    }
}

/// Stores a float value into every object found by a two-level key search.
///
/// The first lookup is virtual `+0x58` on the tree itself. A second pass walks
/// `+0x64` children and queries each child's `+0x24` slot with the same key.
///
/// # Original
/// - Ghidra: `FUN_801642c0` at `0x801642c0` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801642c0_FUN_801642c0.c`
/// - observed: `iVar1 = (**(code **)(*param_3 + 0x58))(param_3,param_2 + 4);
///   if (iVar1 != 0) *(float *)(*(int *)(iVar1 + 8) + 0x10) = (float)param_1;
///   bVar3 = (**(code **)(*param_3 + 100))(param_3); for (...) { ... }`
///
/// # Arguments
/// - `value`: value narrowed to `f32` and stored at `+0x10` of each target.
/// - `key`: key word; the key data starts at `key + 4`.
/// - `tree`: object providing the `+0x58`, `+0x64` and `+0x6C` slots.
///
/// # Side effects
/// Up to `child_count + 1` float stores.
///
/// # Safety
/// The tree, children and matched records must be live; `key + 4` must be a
/// valid key buffer.
///
/// # Confidence
/// medium - the calls, offsets and float store are literal; the child slot
/// roles are inferred from the `+0x64` count and `+0x6C` accessor.
pub unsafe fn fn_801642c0(value: f64, key: u32, tree: usize) {
    let vtable = platform::read_u32(tree) as usize;
    let found = platform::call2(
        platform::read_u32(vtable + 0x58) as usize,
        tree as u32,
        key + 4,
    );
    if found != 0 {
        let record = platform::read_u32(found as usize + 8) as usize;
        platform::write_f32(record + 0x10, value as f32);
    }
    let count = platform::call1(
        platform::read_u32(vtable + 0x64) as usize,
        tree as u32,
    ) as u8;
    let mut index = 0u8;
    while index < count {
        let child = platform::call2(
            platform::read_u32(vtable + 0x6C) as usize,
            tree as u32,
            index as u32,
        ) as usize;
        let child_vtable = platform::read_u32(child) as usize;
        let match_ptr = platform::call2(
            platform::read_u32(child_vtable + 0x24) as usize,
            child as u32,
            key + 4,
        );
        if match_ptr != 0 {
            let record = platform::read_u32(match_ptr as usize + 8) as usize;
            platform::write_f32(record + 0x10, value as f32);
        }
        index += 1;
    }
}

/// Re-applies the owner's value to all its list nodes and records the enable
/// bit.
///
/// # Original
/// - Ghidra: `FUN_80164460` at `0x80164460` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164460_FUN_80164460.c`
/// - observed: `FUN_80164080(*(undefined4 *)(param_1 + 4), **(undefined4 **)
///   (param_1 + 8), param_2); if (param_2 == 0) byte &= 0xfe; else byte |= 1;`
///
/// # Arguments
/// - `this`: holder; `+0x04` key, `+0x08` pointer to the list owner, `+0x0C`
///   flags.
/// - `value`: forwarded to [`fn_80164080`]; also selects the flag value.
///
/// # Side effects
/// One walk over the list and one bit update at `+0x0C`.
///
/// # Safety
/// `this + 8` must point to the list owner.
///
/// # Confidence
/// high - the call, the two branches and the bit operations are literal.
pub unsafe fn fn_80164460(this: usize, value: u32) {
    fn_80164080(
        platform::read_u32(this + 4),
        platform::read_u32(platform::read_u32(this + 8) as usize) as usize,
        value,
    );
    if value == 0 {
        platform::write_u8(this + 0x0C, platform::read_u8(this + 0x0C) & 0xFE);
    } else {
        platform::write_u8(this + 0x0C, platform::read_u8(this + 0x0C) | 1);
    }
}

/// Unpacks a three-word descriptor and forwards it to [`fn_80164120`].
///
/// # Original
/// - Ghidra: `FUN_801644d0` at `0x801644d0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801644d0_FUN_801644d0.c`
/// - observed: `FUN_80164120((double)*(float *)(*param_1 + 8),param_1[1],
///   *(undefined4 *)param_1[2]);`
///
/// # Arguments
/// - `descriptor`: `{object, key, list_owner_ptr}`.
///
/// # Side effects
/// Calls [`fn_80164120`] with the object's `+0x08` float widened to `f64`.
///
/// # Safety
/// All three words must be valid and the object readable at `+0x08`.
///
/// # Confidence
/// high - the three loads and the widened argument are literal.
pub unsafe fn fn_801644d0(descriptor: usize) {
    let object = platform::read_u32(descriptor) as usize;
    let value = platform::read_f32(object + 8) as f64;
    let key = platform::read_u32(descriptor + 4);
    let list = platform::read_u32(platform::read_u32(descriptor + 8) as usize);
    fn_80164120(value, key, list as usize);
}

// ===========================================================================
// Element wrappers: 0x80168100..0x8016A340 (wave 52)
//
// These are the destructor/teardown pairs around the `0x38`-stride element
// accessor set from wave 40. `+0x04` is the casting target, `+0x14` the flag
// byte and `+0x38` the accessor index.
// ===========================================================================

/// Destroys the element object, runs the teardown slot, then optionally frees.
///
/// # Original
/// - Ghidra: `FUN_80168100` at `0x80168100` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168100_FUN_80168100.c`
/// - observed: `*param_1 = &DAT_803295f0; (*(code *)PTR_FUN_80329600)();
///   FUN_8016a030(param_1,0); if (0 < param_2) FUN_802b9280(param_1);`
///
/// # Arguments
/// - `this`: element object, or zero.
/// - `deleting`: positive to free after teardown.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Installs the teardown vtable, calls the function pointer stored at
/// `0x80329600`, runs [`fn_8016a030`] and may free `this`.
///
/// # Safety
/// `this` must be null or a live element; after a positive `deleting` it must
/// not be used again.
///
/// # Confidence
/// medium - every store and call is literal; the identity of the indirect slot
/// is unresolved.
pub unsafe fn fn_80168100(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, ELEMENT_DTOR_VTABLE as u32);
        let slot = platform::read_u32(ELEMENT_DTOR_SLOT) as usize;
        platform::call0(slot);
        fn_8016a030(this, 0);
        if deleting > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Casts the object at `+0x04` to the `0x802F8FD8` type and installs it.
///
/// # Original
/// - Ghidra: `FUN_80168180` at `0x80168180` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168180_FUN_80168180.c`
/// - observed: `piVar3 = *(int **)(param_1 + 4); ... local_18[0] = &DAT_802f8fd8;
///   iVar2 = (**(code **)(*piVar3 + 8))(piVar3,local_18); ...
///   FUN_8025b420(piVar3,param_3,1,param_2);`
///
/// # Arguments
/// - `this`: wrapper; `+0x04` is the object to cast.
/// - `arg`: fourth argument of the install call.
/// - `out`: output buffer passed as the third argument.
///
/// # Returns
/// The install call's status word (the export declares `void`, but the caller
/// tests the value, so the tail-called status is returned).
///
/// # Side effects
/// Runs the cast query and the install call.
///
/// # Safety
/// `out` must be writable for the install call; the object must be live.
///
/// # Confidence
/// medium - the query, the local descriptor and the four arguments are
/// literal; the `void` declaration hides the forwarded return value.
pub unsafe fn fn_80168180(this: usize, arg: u32, out: u32) -> u32 {
    let object = platform::read_u32(this + 4) as usize;
    let mut cast = 0usize;
    if object != 0 {
        let mut typeinfo = crate::platform::GuestLocal::new( [DESCRIPTOR_802F8FD8 as u32, 0u32]);
        let vtable = platform::read_u32(object) as usize;
        let matched = platform::call2(
            platform::read_u32(vtable + 8) as usize,
            object as u32,
            typeinfo.as_mut_ptr() as u32,
        );
        if matched != 0 {
            cast = object;
        }
    }
    platform::call4(FN_CAST_INSTALL, cast as u32, out, 1, arg)
}

/// Casts the wrapper's object and copies three words into `value`.
///
/// # Original
/// - Ghidra: `FUN_80168230` at `0x80168230` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168230_FUN_80168230.c`
/// - observed: `iVar1 = FUN_80168180(param_1,param_2,auStack_38);
///   if (iVar1 != 0) FUN_8016f130(auStack_38,param_3); return iVar1 != 0;`
///
/// # Arguments
/// - `this`: wrapper.
/// - `arg`: forwarded to [`fn_80168180`].
/// - `value`: three-word destination, written only on success.
///
/// # Returns
/// `true` when the cast/install succeeded.
///
/// # Side effects
/// Writes `value..value+8`.
///
/// # Safety
/// `value` must be writable for three words.
///
/// # Confidence
/// medium - the local buffer and both calls are literal; the callee's return
/// value is inferred from the test.
pub unsafe fn fn_80168230(this: usize, arg: u32, value: usize) -> bool {
    let mut out = crate::platform::GuestLocal::new( [0u32; 13]);
    let ok = fn_80168180(this, arg, out.as_mut_ptr() as u32);
    if ok != 0 {
        fn_8016f130(out.as_ptr() as usize, value);
    }
    ok != 0
}

/// Destroys the `+0x04` child and optionally frees the wrapper.
///
/// # Original
/// - Ghidra: `FUN_8016a030` at `0x8016a030` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a030_FUN_8016a030.c`
/// - observed: `*param_1 = &DAT_80329670; FUN_8016a090();
///   if (0 < param_2) FUN_802b9280(param_1);`
///
/// # Arguments
/// - `this`: wrapper, or zero.
/// - `deleting`: positive to free the wrapper.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Installs the wrapper vtable, destroys the child through [`fn_8016a090`] and
/// may free `this`.
///
/// # Safety
/// A non-null `this` must be a live wrapper; after a positive `deleting` it
/// must not be used again.
///
/// # Confidence
/// high - the vtable store, the call and the conditional free are literal.
pub unsafe fn fn_8016a030(this: usize, deleting: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, ELEMENT_WRAPPER_VTABLE as u32);
        fn_8016a090(this);
        if deleting > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Releases the child object stored at `+0x04` and clears the field.
///
/// # Original
/// - Ghidra: `FUN_8016a090` at `0x8016a090` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016a090_FUN_8016a090.c`
/// - observed: `if (*(int *)(param_1 + 4) != 0) {
///   FUN_80249ac0(*(int *)(param_1 + 4)); *(undefined4 *)(param_1 + 4) = 0; }`
///
/// # Arguments
/// - `this`: wrapper whose `+0x04` child is released.
///
/// # Side effects
/// Calls `0x80249AC0(child)` and zeroes `+0x04`.
///
/// # Safety
/// The child must be accepted by `0x80249AC0`.
///
/// # Confidence
/// high - the guard, release call and clear are literal.
pub unsafe fn fn_8016a090(this: usize) {
    let child = platform::read_u32(this + 4);
    if child != 0 {
        platform::call1(0x8024_9AC0, child);
        platform::write_u32(this + 4, 0);
    }
}

// ===========================================================================
// Interpolator leaves: 0x80168B40..0x80168D80 (wave 52)
//
// Object layout: `+0x04` current value, `+0x08` accumulator, `+0x0C` step,
// `+0x10`/`+0x14`/`+0x18` derived factors, `+0x1C`/`+0x1D` state bytes and
// `+0x28`/`+0x30` the two published values.
// ===========================================================================

/// Constructs the interpolator and stores the two reset constants.
///
/// # Original
/// - Ghidra: `FUN_80168b40` at `0x80168b40` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168b40_FUN_80168b40.c`
/// - observed: `*param_1 = &DAT_80329610; param_1[1] = uVar1; param_1[2] = uVar2;
///   ... *(undefined1 *)(param_1 + 7) = 0; *(undefined1 *)((int)param_1 + 0x1d) = 0;`
///
/// # Arguments
/// - `this`: interpolator object, writable for `0x1E` bytes.
///
/// # Side effects
/// Installs the vtable and fills `+0x04..0x18` plus two byte flags.
///
/// # Safety
/// `this` must be writable through `+0x1D`.
///
/// # Confidence
/// high - the vtable store, six constant stores and two byte clears are
/// literal.
pub unsafe fn fn_80168b40(this: usize) {
    let a = platform::read_u32(INTERP_VALUE_A);
    let b = platform::read_u32(INTERP_VALUE_B);
    platform::write_u32(this, INTERP_VALUE_VTABLE as u32);
    platform::write_u32(this + 0x04, a);
    platform::write_u32(this + 0x08, b);
    platform::write_u32(this + 0x0C, a);
    platform::write_u32(this + 0x10, a);
    platform::write_u32(this + 0x14, b);
    platform::write_u32(this + 0x18, b);
    platform::write_u8(this + 0x1C, 0);
    platform::write_u8(this + 0x1D, 0);
}

/// Restores the five value fields to the two reset constants.
///
/// # Original
/// - Ghidra: `FUN_80168b90` at `0x80168b90` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168b90_FUN_80168b90.c`
/// - observed: `*(param_1 + 4) = uVar2; *(param_1 + 8) = uVar1;
///   *(param_1 + 0x10) = uVar2; *(param_1 + 0x14) = uVar1;
///   *(param_1 + 0x18) = uVar1;`
///
/// # Arguments
/// - `this`: interpolator object.
///
/// # Side effects
/// Writes `+0x04`, `+0x08`, `+0x10`, `+0x14` and `+0x18`.
///
/// # Safety
/// `this` must be writable through `+0x18`.
///
/// # Confidence
/// high - the five loads/stores are literal.
pub unsafe fn fn_80168b90(this: usize) {
    let b = platform::read_u32(INTERP_VALUE_B);
    let a = platform::read_u32(INTERP_VALUE_A);
    platform::write_u32(this + 0x04, a);
    platform::write_u32(this + 0x08, b);
    platform::write_u32(this + 0x10, a);
    platform::write_u32(this + 0x14, b);
    platform::write_u32(this + 0x18, b);
}

/// Reports whether the interpolator is back at its reset value.
///
/// # Original
/// - Ghidra: `FUN_80168cb0` at `0x80168cb0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168cb0_FUN_80168cb0.c`
/// - observed: `return *(float *)(r2 + -0x53d8) == *(float *)(param_1 + 4);`
///
/// # Arguments
/// - `this`: interpolator object.
///
/// # Returns
/// `true` when `+0x04` bit-equals the `r2 - 0x53D8` constant.
///
/// # Safety
/// `this + 4` must be readable.
///
/// # Confidence
/// high - one comparison of two loaded words; bit equality is used instead of
/// a float compare, which differs only for NaN inputs.
pub unsafe fn fn_80168cb0(this: usize) -> bool {
    platform::read_u32(INTERP_VALUE_A) == platform::read_u32(this + 0x04)
}

/// Constructs the interpolator's buffer view and publishes two constants.
///
/// # Original
/// - Ghidra: `FUN_80168d10` at `0x80168d10` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168d10_FUN_80168d10.c`
/// - observed: `*param_1 = &DAT_803295d8; param_1[1] = 0; param_1[2] = 0;
///   FUN_8016a5e0(param_1 + 3); *param_1 = &DAT_80329620; param_1[10] = uVar1;
///   param_1[0xc] = uVar1; *(undefined1 *)(param_1 + 0xd) = 0;`
///
/// # Arguments
/// - `this`: object, writable through `+0x34`.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Constructs the LZ view at `+0x0C`, swaps the vtable and stores the
/// `r2 - 0x53D0` word at `+0x28` and `+0x30`.
///
/// # Safety
/// `this` must be writable and the LZ constructor must accept `this + 0x0C`.
///
/// # Confidence
/// medium - the stores and calls are literal; the vtable-swap idiom implies a
/// base-then-derived construction order.
pub unsafe fn fn_80168d10(this: usize) -> usize {
    platform::write_u32(this, INTERP_BASE_VTABLE as u32);
    platform::write_u32(this + 0x04, 0);
    platform::write_u32(this + 0x08, 0);
    fn_8016a5e0(this + 0x0C);
    let value = platform::read_u32(INTERP_VALUE_C);
    platform::write_u32(this, INTERP_ACTIVE_VTABLE as u32);
    platform::write_u32(this + 0x28, value);
    platform::write_u32(this + 0x30, value);
    platform::write_u8(this + 0x34, 0);
    this
}

/// Destroys the interpolator's buffer view and optionally frees the object.
///
/// # Original
/// - Ghidra: `FUN_80168d80` at `0x80168d80` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168d80_FUN_80168d80.c`
/// - observed: `if ((param_1 != 0) && (FUN_80167ec0(param_1,0), 0 < param_2)) {
///   FUN_802b9280(param_1); } return param_1;`
///
/// # Arguments
/// - `this`: interpolator object, or zero.
/// - `count`: positive to free after the reset.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Calls the reset at `0x80167EC0` and may free `this`.
///
/// # Safety
/// A non-null `this` must be a live interpolator; after a positive `count` it
/// must not be used again.
///
/// # Confidence
/// high - the guard, reset call and conditional free are literal.
pub unsafe fn fn_80168d80(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::call2(FN_INTERP_RESET, this as u32, 0);
        if count > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

// ===========================================================================
// 0x80160000 bucket stragglers (wave 64)
//
// Wave 52 finished the registry-adjacent list/state/interpolator leaves but
// left the rest of the `0x80160000` page untouched. This pass closes the
// remaining runs that can be written without guessing a constant:
//
// * tree/list primitives at `0x80161580`/`0x80161620` (parent, first child,
//   previous and next sibling at `+0x00`, `+0x04`, `+0x08`, `+0x0C`);
// * the actor enable/disable state machine at `0x80161EF0..0x80162B00`;
// * the NW4R sorted-list insert at `0x80163850` (`SB_SORTED_LIST`);
// * three instantiations of the `0x38`-stride element class at
//   `0x80165820..0x80167AB0` (size query, counted size query, release pair,
//   deleting destructor) plus the shared teardown at `0x80167EC0`;
// * small query/init leaves at `0x8016C6F0..0x8016FF60`.
//
// The three class instantiations differ only in the allocator query address
// and the vtable word, so they are kept as explicit twins rather than a
// parameterised helper: the original has no shared body.
// ===========================================================================

/// `0x8022_8E60` - `nw4r::ut::List::InsertBefore(list, next, node)`.
const W64_LIST_INSERT_BEFORE: usize = 0x8022_8E60;

/// `0x80168520` - cast `arg` into the element's `+0x04` child (sibling of
/// [`fn_80166280`]'s release path).
const W64_FN_80168520: usize = 0x8016_8520;

/// `0x80165280` - rebuild one keyed element entry; called by
/// [`fn_801651f0`].
const W64_FN_80165280: usize = 0x8016_5280;

/// `0x802B99C0` - return a block to the EGG `0x38`-element heap class `3`.
const W64_FN_802B99C0: usize = 0x802B_99C0;

/// `0x80249B40` - destroy the native object behind an element.
const W64_FN_80249B40: usize = 0x8024_9B40;

/// `0x80249AC0` - release the native object behind an element.
const W64_FN_80249AC0: usize = 0x8024_9AC0;

/// `0x80245500` - class-B element size query (`0, out, a, b, 0`).
const W64_FN_80245500: usize = 0x8024_5500;

/// `0x80245300` - class-B counted element size query.
const W64_FN_80245300: usize = 0x8024_5300;

/// `0x80243D20` - class-A element size query (`0, out, a, b, 0`).
const W64_FN_80243D20: usize = 0x8024_3D20;

/// `0x80243B30` - class-A counted element size query.
const W64_FN_80243B30: usize = 0x8024_3B30;

/// `0x80246B00` - class-C element size query (`0, out, a, b, 0`).
const W64_FN_80246B00: usize = 0x8024_6B00;

/// `0x80246900` - class-C counted element size query.
const W64_FN_80246900: usize = 0x8024_6900;

/// `0x80165EE0` - class-A element teardown (releases the `+0x28` array).
const W64_FN_80165EE0: usize = 0x8016_5EE0;

/// `0x80166AE0` - class-B element teardown.
const W64_FN_80166AE0: usize = 0x8016_6AE0;

/// `0x80167710` - class-C element teardown.
const W64_FN_80167710: usize = 0x8016_7710;

/// `0x8016FFC0` - store the descriptor word at `entry + 0x04`.
const W64_FN_8016FFC0: usize = 0x8016_FFC0;

/// `0x80170630` - construct the inner object: `*this = 0x801D4380(a, b, 0)`.
const W64_FN_80170630: usize = 0x8017_0630;

/// `0x802967B0` - "descriptor registered" predicate for `+0x74`.
const W64_FN_802967B0: usize = 0x8029_67B0;

/// `0x802967E0` - fetch the descriptor object registered for `+0x74`.
const W64_FN_802967E0: usize = 0x8029_67E0;

/// `0x802DA6F0` - copy a descriptor into the `+0x7C` field.
const W64_FN_802DA6F0: usize = 0x802D_A6F0;

/// `0x80162A70` - resolve a `u16` id against the global element table.
const W64_FN_80162A70: usize = 0x8016_2A70;

/// `0x80329518` - vtable installed by the class-A deleting destructor.
const W64_CLASS_A_VTABLE: usize = 0x8032_9518;

/// `0x80329548` - vtable installed by the class-B deleting destructor.
const W64_CLASS_B_VTABLE: usize = 0x8032_9548;

/// `0x80329578` - vtable installed by the class-C deleting destructor.
const W64_CLASS_C_VTABLE: usize = 0x8032_9578;

/// Zeroes a four-word list header and returns its address.
///
/// # Original
/// - Ghidra: `FUN_80161530` at `0x80161530` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161530_FUN_80161530.c`
/// - observed: `FUN_80161560(); return param_1;`
///
/// # Arguments
/// - `this`: address of the 16-byte header zeroed by [`fn_80161560`].
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Clears the four words at `+0x00..+0x0C`.
///
/// # Safety
/// `this` must be writable for 16 bytes.
///
/// # Confidence
/// high - the forwarding call and the return are literal, and the callee has a
/// body in this file.
pub unsafe fn fn_80161530(this: usize) -> usize {
    fn_80161560(this);
    this
}

/// Appends a leaf node to a root list or to a parent's child list.
///
/// A node is `{parent +0x00, first child +0x04, prev sibling +0x08,
/// next sibling +0x0C}`. With `parent == 0` the node is appended to the chain
/// rooted at `root`; otherwise it is stored under `parent` and its `+0x00`
/// word is set to that parent.
///
/// # Original
/// - Ghidra: `FUN_80161580` at `0x80161580` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161580_FUN_80161580.c`
/// - observed: `if (param_2 == 0) return 0; if (param_3 == 0) { ... } else {
///   *param_2 = param_3; iVar1 = *(int *)(param_3 + 4); ... } return 1;`
///
/// # Arguments
/// - `root`: address of the head word used when `parent` is zero.
/// - `node`: node to append; must be null or eight readable/writable bytes.
/// - `parent`: owning node, or zero to use `root`.
///
/// # Returns
/// `true` on success, `false` when `node` is null.
///
/// # Side effects
/// Writes the new link in the tail node, the node's `+0x08`/`+0x00` words and
/// (for the `root` case) the head word.
///
/// # Safety
/// `node` and every node already in the chain must be live; `parent`, when
/// non-zero, must be a live node.
///
/// # Confidence
/// high - the null guard, both append paths and the three stores are literal.
pub unsafe fn fn_80161580(root: usize, node: usize, parent: usize) -> bool {
    if node == 0 {
        return false;
    }
    if parent == 0 {
        let mut current = platform::read_u32(root) as usize;
        if current == 0 {
            platform::write_u32(root, node as u32);
        } else {
            loop {
                let next = platform::read_u32(current + 0x0C) as usize;
                if next == 0 {
                    platform::write_u32(current + 0x0C, node as u32);
                    platform::write_u32(node + 0x08, current as u32);
                    break;
                }
                current = next;
            }
        }
    } else {
        platform::write_u32(node, parent as u32);
        let mut current = platform::read_u32(parent + 0x04) as usize;
        if current == 0 {
            platform::write_u32(parent + 0x04, node as u32);
        } else {
            loop {
                let next = platform::read_u32(current + 0x0C) as usize;
                if next == 0 {
                    platform::write_u32(current + 0x0C, node as u32);
                    platform::write_u32(node + 0x08, current as u32);
                    break;
                }
                current = next;
            }
        }
    }
    true
}

/// Unlinks a childless node from its parent's child list.
///
/// The node is rejected when it still owns children (`+0x04` non-zero). The
/// head case clears `*root`; otherwise the parent's first-child word or the
/// previous sibling's next word is rewritten.
///
/// # Original
/// - Ghidra: `FUN_80161620` at `0x80161620` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161620_FUN_80161620.c`
/// - observed: `if (param_2[1] != 0) return 0; ... param_2[2] = 0;
///   param_2[3] = 0; *param_2 = 0; return 1;`
///
/// # Arguments
/// - `root`: address of the head word of the list the node may head.
/// - `node`: leaf node to unlink.
///
/// # Returns
/// `true` when unlinked, `false` for a null node or a node with children.
///
/// # Side effects
/// Repairs the neighbour links and clears the node's parent, prev and next
/// words (its child word is already zero).
///
/// # Safety
/// `node` must be a live leaf and either be linked in `root` or carry valid
/// neighbour words.
///
/// # Confidence
/// high - the guards, all four neighbour repairs and the three clears are
/// literal.
pub unsafe fn fn_80161620(root: usize, node: usize) -> bool {
    if node == 0 {
        return false;
    }
    if platform::read_u32(node + 0x04) != 0 {
        return false;
    }
    let prev = platform::read_u32(node + 0x08) as usize;
    let next = platform::read_u32(node + 0x0C) as usize;
    if prev == 0 {
        let parent = platform::read_u32(node) as usize;
        if parent == 0 {
            if node == platform::read_u32(root) as usize {
                platform::write_u32(root, next as u32);
            }
        } else {
            platform::write_u32(parent + 0x04, next as u32);
        }
    } else {
        platform::write_u32(prev + 0x0C, next as u32);
    }
    if next != 0 {
        platform::write_u32(next + 0x08, prev as u32);
    }
    platform::write_u32(node + 0x08, 0);
    platform::write_u32(node + 0x0C, 0);
    platform::write_u32(node, 0);
    true
}

/// Reports whether an actor may advance this frame.
///
/// The `+0x50` queue must be absent or finished ([`fn_80161a70`]), and the
/// `+0x14` lookup used by [`fn_801625b0`] must return zero.
///
/// # Original
/// - Ghidra: `FUN_80161ef0` at `0x80161ef0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161ef0_FUN_80161ef0.c`
/// - observed: `if ((*(int *)(param_1 + 0x50) == 0) ||
///   (iVar1 = FUN_80161a70(*(int *)(param_1 + 0x50)), iVar1 != 0)) {
///   iVar1 = FUN_801625b0(param_1); if (iVar1 == 0) uVar2 = 1; else uVar2 = 0; }`
///
/// # Arguments
/// - `this`: live actor object.
///
/// # Returns
/// `true` when the queue gate is open and the lookup is empty.
///
/// # Side effects
/// [`fn_80161a70`] may advance the queue state machine.
///
/// # Safety
/// `this` must be a live actor; a non-null `+0x50` must be a live state object.
///
/// # Confidence
/// medium - the branch and both callees are literal, but the `+0x50`/`+0x14`
/// roles are inferred from the neighbouring actor code.
pub unsafe fn fn_80161ef0(this: usize) -> bool {
    let queue = platform::read_u32(this + 0x50) as usize;
    if queue != 0 && !fn_80161a70(queue) {
        return false;
    }
    fn_801625b0(this) == 0
}

/// Advances the actor's enable/disable state machine and republishes its list
/// membership.
///
/// The `+0x0B` busy bit takes the short release path. Otherwise the peer
/// returned by [`fn_80162590`] drives the `+0x0F` bits 1 and 3; then state
/// `+0x0A` selects between the two published key pairs (`+0x30`/`+0x32` and
/// `+0x40`/`+0x42`) and the child chain at `+0x14` is marked through
/// [`fn_80162510`].
///
/// # Original
/// - Ghidra: `FUN_801622e0` at `0x801622e0` (552 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801622e0_FUN_801622e0.c`
/// - observed: `if (*(char *)(param_1 + 0xb) == '\0') { iVar1 = FUN_80162590();
///   ... if (*(char *)(param_1 + 10) == '\x01') { ... } } else { ... } return 1;`
///
/// # Arguments
/// - `this`: live actor object.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Toggles the `+0x0F` flags, moves entries between the wave-40 sound/actor
/// lists, rewrites the two published `u16` keys and marks every child at
/// `+0x14`.
///
/// # Safety
/// `this` must be a live actor; its `+0x14` chain and the list heads
/// [`SB_LIST_2`]..[`SB_LIST_5`] must be valid.
///
/// # Confidence
/// high - every branch, mask and store is literal; only the list roles are
/// inherited from the wave-40 bodies.
pub unsafe fn fn_801622e0(this: usize) -> u32 {
    if platform::read_u8(this + 0x0B) == 0 {
        let peer = fn_80162590(this) as usize;
        if peer != 0 {
            let peer_flags = platform::read_u8(peer + 0x0F);
            if (peer_flags & 1) == 0 && (peer_flags & 2) == 0 {
                if (platform::read_u8(this + 0x0F) & 2) != 0 {
                    let flags = platform::read_u8(this + 0x0F) & 0xFD;
                    platform::write_u8(this + 0x0F, flags);
                }
            } else {
                let flags = platform::read_u8(this + 0x0F) | 2;
                platform::write_u8(this + 0x0F, flags);
            }
            if (peer_flags & 4) == 0 && (peer_flags & 8) == 0 {
                if (platform::read_u8(this + 0x0F) & 8) != 0 {
                    let flags = platform::read_u8(this + 0x0F) & 0xF7;
                    platform::write_u8(this + 0x0F, flags);
                }
            } else {
                let flags = platform::read_u8(this + 0x0F) | 8;
                platform::write_u8(this + 0x0F, flags);
            }
        }
        match platform::read_u8(this + 0x0A) {
            1 => {
                if platform::read_u16(this + 0x32) != platform::read_u16(this + 0x30) {
                    fn_80160f60(SB_LIST_3, this + 0x24);
                    let key = platform::read_u16(this + 0x32);
                    platform::write_u16(this + 0x30, key);
                    fn_80162b40(SB_LIST_3, this + 0x24);
                }
                if platform::read_u16(this + 0x42) != platform::read_u16(this + 0x40) {
                    fn_80160f60(SB_LIST_4, this + 0x34);
                    let key = platform::read_u16(this + 0x42);
                    platform::write_u16(this + 0x40, key);
                    fn_80162b40(SB_LIST_4, this + 0x34);
                }
            }
            2 => {}
            _ => {
                if platform::read_u8(this + 0x0D) == 0 {
                    if platform::read_u8(this + 0x0C) != 0 {
                        platform::write_u8(this + 0x0C, 0);
                        fn_80162b40(SB_LIST_3, this + 0x24);
                        fn_80162b40(SB_LIST_4, this + 0x34);
                        platform::write_u8(this + 0x0A, 1);
                    }
                } else {
                    platform::write_u8(this + 0x0D, 0);
                    fn_80160fe0(SB_LIST_2, this + 0x24);
                }
            }
        }
    } else {
        platform::write_u8(this + 0x0B, 0);
        if platform::read_u8(this + 0x0A) == 1 {
            fn_80160f60(SB_LIST_3, this + 0x24);
            fn_80160f60(SB_LIST_4, this + 0x34);
        } else {
            fn_80160f60(SB_LIST_2, this + 0x24);
        }
        fn_80161020(SB_LIST_5, this + 0x24);
        platform::write_u8(this + 0x0A, 2);
        let mut node = platform::read_u32(this + 0x14) as usize;
        while node != 0 {
            fn_80162510(platform::read_u32(node + 0x10) as usize);
            node = platform::read_u32(node + 0x0C) as usize;
        }
    }
    1
}

/// Recursively marks an actor and its `+0x14` child chain as shutting down.
///
/// A node is skipped when `+0x0B` is already set or `+0x0A` is `2`; otherwise
/// `+0x0B` becomes `1`, the virtual slot `+0x38` of the object at `+0x60`
/// runs, and every `+0x14` child is visited in turn.
///
/// # Original
/// - Ghidra: `FUN_80162510` at `0x80162510` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162510_FUN_80162510.c`
/// - observed: `if ((*(char *)(param_1 + 0xb) == '\0') &&
///   (*(char *)(param_1 + 10) != '\x02')) { *(undefined1 *)(param_1 + 0xb) = 1;
///   (**(code **)(*(int *)(param_1 + 0x60) + 0x38))();
///   for (iVar1 = *(int *)(param_1 + 0x14); iVar1 != 0; iVar1 = *(int *)(iVar1 + 0xc)) { ... } }`
///
/// # Arguments
/// - `this`: actor node; the recursion depth follows the `+0x14` chain.
///
/// # Side effects
/// Sets `+0x0B`, runs one virtual call and recurses into each child's `+0x10`
/// payload.
///
/// # Safety
/// Every visited node and its `+0x60` vtable must be live; the chain must be
/// acyclic.
///
/// # Confidence
/// medium - the guards, bit store and recursion are literal; the virtual
/// receiver at `+0x38` is inferred from the unchanged `r3`.
pub unsafe fn fn_80162510(this: usize) {
    if platform::read_u8(this + 0x0B) != 0 || platform::read_u8(this + 0x0A) == 2 {
        return;
    }
    platform::write_u8(this + 0x0B, 1);
    let vtable = platform::read_u32(this + 0x60) as usize;
    let slot = platform::read_u32(vtable + 0x38) as usize;
    platform::call1(slot, this as u32);
    let mut node = platform::read_u32(this + 0x14) as usize;
    while node != 0 {
        fn_80162510(platform::read_u32(node + 0x10) as usize);
        node = platform::read_u32(node + 0x0C) as usize;
    }
}

/// Resolves the descriptor behind a node's `+0x10` payload.
///
/// # Original
/// - Ghidra: `FUN_80162b00` at `0x80162b00` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162b00_FUN_80162b00.c`
/// - observed: `if (param_2 == 0) return 0;
///   uVar1 = FUN_80162a70(param_1,param_2 + 0x10); return uVar1;`
///
/// # Arguments
/// - `this`: table owner word handed to the resolver.
/// - `node`: node whose `+0x10` payload is resolved, or zero.
///
/// # Returns
/// The resolver result, or `0` for a null node.
///
/// # Side effects
/// None beyond the resolver's own locks/factory.
///
/// # Safety
/// A non-null `node` must be readable through `+0x10`.
///
/// # Confidence
/// medium - the guard and the two call arguments are literal; the resolver's
/// body is still untranslated.
pub unsafe fn fn_80162b00(this: usize, node: usize) -> u32 {
    if node == 0 {
        return 0;
    }
    platform::call2(W64_FN_80162A70, this as u32, (node + 0x10) as u32)
}

/// Inserts a node into the global sorted list, ordered by the byte at `+0x0C`.
///
/// The walk uses the NW4R list iterator ([`FN_UT_LIST_NEXT`]) until the first
/// node with a larger key is found; the new node is inserted before it or
/// appended when the walk runs off the end.
///
/// # Original
/// - Ghidra: `FUN_80163850` at `0x80163850` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163850_FUN_80163850.c`
/// - observed: `iVar1 = FUN_80228ff0(&DAT_80377ab0,0); while( true ) {
///   if (iVar1 == 0) { FUN_80228df0(&DAT_80377ab0,param_1); return; }
///   if (*(byte *)(param_1 + 0xc) < *(byte *)(iVar1 + 0xc)) break;
///   iVar1 = FUN_80228ff0(&DAT_80377ab0); } FUN_80228e60(&DAT_80377ab0,iVar1,param_1);`
///
/// # Arguments
/// - `node`: node to insert; its `+0x0C` byte is the sort key.
///
/// # Side effects
/// Rewrites the [`SB_SORTED_LIST`] links.
///
/// # Safety
/// `node` must be a live NW4R list node and must not already be linked.
///
/// # Confidence
/// medium - the order test and the three list calls are literal; the iterator's
/// second argument is dropped by the export and restored from the list idiom.
pub unsafe fn fn_80163850(node: usize) {
    let mut current = platform::call2(FN_UT_LIST_NEXT, SB_SORTED_LIST as u32, 0) as usize;
    loop {
        if current == 0 {
            platform::call2(FN_UT_LIST_APPEND, SB_SORTED_LIST as u32, node as u32);
            return;
        }
        if platform::read_u8(node + 0x0C) < platform::read_u8(current + 0x0C) {
            break;
        }
        current = platform::call2(FN_UT_LIST_NEXT, SB_SORTED_LIST as u32, current as u32) as usize;
    }
    platform::call3(
        W64_LIST_INSERT_BEFORE,
        SB_SORTED_LIST as u32,
        current as u32,
        node as u32,
    );
}

/// Forwards a value to the element's `+0x04` child and rebuilds its keyed
/// entry.
///
/// The virtual slot `+0x0C` is called on `this` itself to obtain the value,
/// which is stored through `0x80168520`; the rebuilt entry is then produced by
/// `0x80165280` from the descriptor word at `arg3`.
///
/// # Original
/// - Ghidra: `FUN_801651f0` at `0x801651f0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801651f0_FUN_801651f0.c`
/// - observed: `uVar1 = (**(code **)(*param_1 + 0xc))();
///   FUN_80168520(param_2,uVar1); local_18[0] = *param_3;
///   FUN_80165280(param_1,param_2,local_18,param_4);`
///
/// # Arguments
/// - `this`: element wrapper whose vtable slot `+0x0C` is queried.
/// - `arg2`: destination object for the cast value.
/// - `arg3`: address of the descriptor word.
/// - `arg4`: forwarded to the entry rebuild.
///
/// # Side effects
/// Runs one virtual call, one cast/install and the keyed-entry rebuild.
///
/// # Safety
/// `this` must be a live wrapper, `arg3` readable for one word, and both
/// callees must accept the forwarded pointers.
///
/// # Confidence
/// medium - the call order and arguments are literal; the callee roles are
/// inferred from the neighbouring class-B bodies.
pub unsafe fn fn_801651f0(this: usize, arg2: u32, arg3: usize, arg4: u32) {
    let vtable = platform::read_u32(this) as usize;
    let slot = platform::read_u32(vtable + 0x0C) as usize;
    let value = platform::call1(slot, this as u32);
    platform::call2(W64_FN_80168520, arg2, value);
    let descriptor = crate::platform::GuestLocal::new( platform::read_u32(arg3));
    platform::call4(
        W64_FN_80165280,
        this as u32,
        arg2,
        (descriptor.as_ptr() as *const u32) as usize as u32,
        arg4,
    );
}

/// Returns the space needed for one class-A element instance.
///
/// The raw size comes from the constructor query at `0x80243D20`; when
/// `align` is non-zero the block is advanced by the EGG header allowance and
/// rounded up to `0x20` ([`crate::ui::lyt::fn_8016e7d0`]).
///
/// # Original
/// - Ghidra: `FUN_80165820` at `0x80165820` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165820_FUN_80165820.c`
/// - observed: `local_10[0] = 0; local_18 = *param_1; local_14 = *param_2;
///   FUN_80243d20(0,local_10,&local_14,&local_18,0); if (param_3 != 0) { ... }
///   return local_10[0];`
///
/// # Arguments
/// - `a`, `b`: addresses of the two constructor words.
/// - `align`: non-zero to add the aligned heap-header allowance.
///
/// # Returns
/// The element size in bytes.
///
/// # Side effects
/// None; the query writes only the local size word.
///
/// # Safety
/// Both addresses must be readable for one word.
///
/// # Confidence
/// high - the five call arguments and the alignment arithmetic are literal.
pub unsafe fn fn_80165820(a: usize, b: usize, align: i32) -> u32 {
    let mut size = crate::platform::GuestLocal::<u32>::new( 0);
    let first = crate::platform::GuestLocal::new( platform::read_u32(a));
    let mut second = crate::platform::GuestLocal::new( platform::read_u32(b));
    platform::call5(
        W64_FN_80243D20,
        0,
        (size.as_mut_ptr() as *mut u32) as usize as u32,
        (second.as_mut_ptr() as *mut u32) as usize as u32,
        (first.as_ptr() as *const u32) as usize as u32,
        0,
    );
    if align != 0 {
        let advanced = crate::ui::lyt::fn_8016e7d0(*size as i32, 0x20);
        *size = (advanced as u32).wrapping_add(0x1F) & 0xFFFF_FFE0;
    }
    *size
}

/// Releases the element's `+0x04` child and returns its buffer to heap class 3.
///
/// # Original
/// - Ghidra: `FUN_80165a80` at `0x80165a80` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165a80_FUN_80165a80.c`
/// - observed: `if (*(int **)(param_1 + 4) != (int *)0x0) {
///   (**(code **)(**(int **)(param_1 + 4) + 0x34))();
///   FUN_802b99c0(*(undefined4 *)(param_1 + 8),3);
///   *(undefined4 *)(param_1 + 4) = 0; }`
///
/// # Arguments
/// - `this`: element wrapper.
///
/// # Side effects
/// Runs virtual slot `+0x34` of the child, frees the `+0x08` buffer through
/// `0x802B99C0(buffer, 3)` and clears `+0x04`.
///
/// # Safety
/// A non-null `+0x04` must be a live EGG object and `+0x08` its heap block.
///
/// # Confidence
/// high - the guard, the virtual call, the heap-class argument and the clear
/// are literal.
pub unsafe fn fn_80165a80(this: usize) {
    let child = platform::read_u32(this + 0x04) as usize;
    if child != 0 {
        let vtable = platform::read_u32(child) as usize;
        let slot = platform::read_u32(vtable + 0x34) as usize;
        platform::call1(slot, child as u32);
        platform::call2(W64_FN_802B99C0, platform::read_u32(this + 0x08), 3);
        platform::write_u32(this + 0x04, 0);
    }
}

/// Returns the space needed for `count` class-A elements plus their table.
///
/// The base query is `0x80243B30`; a `0x38`-byte stride and the aligned size
/// from [`fn_80165820`] are added per entry, then the optional `0x20`
/// alignment allowance is applied.
///
/// # Original
/// - Ghidra: `FUN_80165b50` at `0x80165b50` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165b50_FUN_80165b50.c`
/// - observed: `local_1c[0] = 0; local_20 = *param_1;
///   FUN_80243b30(0,local_1c,&local_20,param_3);
///   local_1c[0] = local_1c[0] + (param_3 * 0x38 + 0x1fU & 0xffffffe0); ...
///   iVar1 = FUN_80165820(&local_24,&local_28,1);
///   local_1c[0] = local_1c[0] + param_3 * (iVar1 + 0x1fU & 0xffffffe0);`
///
/// # Arguments
/// - `a`, `b`: addresses of the two constructor words.
/// - `count`: number of elements.
/// - `align`: non-zero to add the aligned heap-header allowance.
///
/// # Returns
/// The total size in bytes.
///
/// # Side effects
/// None; only local size words are written.
///
/// # Safety
/// Both addresses must be readable for one word.
///
/// # Confidence
/// high - every term of the size formula is literal.
pub unsafe fn fn_80165b50(a: usize, b: usize, count: u32, align: i32) -> u32 {
    let mut total = crate::platform::GuestLocal::<u32>::new( 0);
    let mut first = crate::platform::GuestLocal::new( platform::read_u32(a));
    platform::call4(
        W64_FN_80243B30,
        0,
        (total.as_mut_ptr() as *mut u32) as usize as u32,
        (first.as_mut_ptr() as *mut u32) as usize as u32,
        count,
    );
    *total = total.wrapping_add(count.wrapping_mul(0x38).wrapping_add(0x1F) & 0xFFFF_FFE0);
    let element_a = crate::platform::GuestLocal::new( platform::read_u32(a));
    let element_b = crate::platform::GuestLocal::new( platform::read_u32(b));
    let element_size = fn_80165820(
        (element_a.as_ptr() as *const u32) as usize,
        (element_b.as_ptr() as *const u32) as usize,
        1,
    );
    *total = total.wrapping_add(
        count.wrapping_mul(element_size.wrapping_add(0x1F) & 0xFFFF_FFE0),
    );
    if align != 0 {
        let advanced = crate::ui::lyt::fn_8016e7d0(*total as i32, 0x20);
        *total = (advanced as u32).wrapping_add(0x1F) & 0xFFFF_FFE0;
    }
    *total
}

/// Deleting destructor for the class-A element.
///
/// Installs [`W64_CLASS_A_VTABLE`], runs the class teardown at `0x80165EE0`,
/// forwards to the shared base destructor [`fn_80167ec0`], then frees the
/// object when `count` is positive.
///
/// # Original
/// - Ghidra: `FUN_80165e70` at `0x80165e70` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165e70_FUN_80165e70.c`
/// - observed: `if (param_1 != (undefined4 *)0x0) { *param_1 = &DAT_80329518;
///   FUN_80165ee0(); FUN_80167ec0(param_1,0); if (0 < param_2) {
///   FUN_802b9280(param_1); } } return param_1;`
///
/// # Arguments
/// - `this`: element object, or zero.
/// - `count`: positive to free the object after teardown.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Writes the vtable word, releases the element array and may free `this`.
///
/// # Safety
/// A non-null `this` must be a live class-A element; after a positive `count`
/// it must not be used again.
///
/// # Confidence
/// medium - the vtable, two calls and the free are literal; the teardown
/// callee's body is still untranslated.
pub unsafe fn fn_80165e70(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, W64_CLASS_A_VTABLE as u32);
        platform::call1(W64_FN_80165EE0, this as u32);
        fn_80167ec0(this, 0);
        if count > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Releases one class-A element when it still owns a native object.
///
/// The entry is at `array + index * 0x38`; the object field is tested through
/// [`fn_80168060`] before virtual slot `+0x14` runs with the entry as receiver.
///
/// # Original
/// - Ghidra: `FUN_80166280` at `0x80166280` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166280_FUN_80166280.c`
/// - observed: `param_2 = param_2 * 0x38;
///   iVar1 = FUN_80168060(*(int *)(param_1 + 0x28) + param_2); if (iVar1 != 0) {
///   (**(code **)(*(int *)(param_2 + *(int *)(param_1 + 0x28)) + 0x14))(...); }`
///
/// # Arguments
/// - `this`: element owner whose `+0x28` word points at the array.
/// - `index`: element index, multiplied by the `0x38` stride.
///
/// # Side effects
/// Runs one virtual call on the element.
///
/// # Safety
/// `this + 0x28` must address an array with at least `index + 1` entries and
/// each entry's vtable word must be live.
///
/// # Confidence
/// high - the stride, the tag test and the virtual slot are literal.
pub unsafe fn fn_80166280(this: usize, index: u32) {
    let offset = index.wrapping_mul(0x38) as usize;
    let element = platform::read_u32(this + 0x28) as usize + offset;
    if fn_80168060(element) != 0 {
        let vtable = platform::read_u32(element) as usize;
        let slot = platform::read_u32(vtable + 0x14) as usize;
        platform::call1(slot, element as u32);
    }
}

/// Returns the space needed for one class-B element instance.
///
/// Identical to [`fn_80165820`] except for the constructor query address
/// (`0x80245500`).
///
/// # Original
/// - Ghidra: `FUN_80166420` at `0x80166420` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166420_FUN_80166420.c`
/// - observed: `FUN_80245500(0,local_10,&local_14,&local_18,0);
///   if (param_3 != 0) { iVar1 = FUN_8016e7d0(local_10[0],0x20); ... }`
///
/// # Arguments
/// - `a`, `b`: addresses of the two constructor words.
/// - `align`: non-zero to add the aligned heap-header allowance.
///
/// # Returns
/// The element size in bytes.
///
/// # Side effects
/// None; only the local size word is written.
///
/// # Safety
/// Both addresses must be readable for one word.
///
/// # Confidence
/// high - the five call arguments and the alignment arithmetic are literal.
pub unsafe fn fn_80166420(a: usize, b: usize, align: i32) -> u32 {
    let mut size = crate::platform::GuestLocal::<u32>::new( 0);
    let first = crate::platform::GuestLocal::new( platform::read_u32(a));
    let mut second = crate::platform::GuestLocal::new( platform::read_u32(b));
    platform::call5(
        W64_FN_80245500,
        0,
        (size.as_mut_ptr() as *mut u32) as usize as u32,
        (second.as_mut_ptr() as *mut u32) as usize as u32,
        (first.as_ptr() as *const u32) as usize as u32,
        0,
    );
    if align != 0 {
        let advanced = crate::ui::lyt::fn_8016e7d0(*size as i32, 0x20);
        *size = (advanced as u32).wrapping_add(0x1F) & 0xFFFF_FFE0;
    }
    *size
}

/// Releases the class-B element's `+0x04` child and returns its buffer to heap
/// class 3.
///
/// # Original
/// - Ghidra: `FUN_80166680` at `0x80166680` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166680_FUN_80166680.c`
/// - observed: `if (*(int **)(param_1 + 4) != (int *)0x0) {
///   (**(code **)(**(int **)(param_1 + 4) + 0x34))();
///   FUN_802b99c0(*(undefined4 *)(param_1 + 8),3);
///   *(undefined4 *)(param_1 + 4) = 0; }`
///
/// # Arguments
/// - `this`: element wrapper.
///
/// # Side effects
/// Runs virtual slot `+0x34` of the child, frees the `+0x08` buffer and clears
/// `+0x04`.
///
/// # Safety
/// A non-null `+0x04` must be a live EGG object and `+0x08` its heap block.
///
/// # Confidence
/// high - the guard, the virtual call, the heap-class argument and the clear
/// are literal.
pub unsafe fn fn_80166680(this: usize) {
    let child = platform::read_u32(this + 0x04) as usize;
    if child != 0 {
        let vtable = platform::read_u32(child) as usize;
        let slot = platform::read_u32(vtable + 0x34) as usize;
        platform::call1(slot, child as u32);
        platform::call2(W64_FN_802B99C0, platform::read_u32(this + 0x08), 3);
        platform::write_u32(this + 0x04, 0);
    }
}

/// Returns the space needed for `count` class-B elements plus their table.
///
/// Identical to [`fn_80165b50`] except for the base query address
/// (`0x80245300`) and the per-element query [`fn_80166420`].
///
/// # Original
/// - Ghidra: `FUN_80166750` at `0x80166750` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166750_FUN_80166750.c`
/// - observed: `FUN_80245300(0,local_1c,&local_20,param_3);
///   local_1c[0] = local_1c[0] + (param_3 * 0x38 + 0x1fU & 0xffffffe0); ...
///   iVar1 = FUN_80166420(&local_24,&local_28,1); ...`
///
/// # Arguments
/// - `a`, `b`: addresses of the two constructor words.
/// - `count`: number of elements.
/// - `align`: non-zero to add the aligned heap-header allowance.
///
/// # Returns
/// The total size in bytes.
///
/// # Side effects
/// None; only local size words are written.
///
/// # Safety
/// Both addresses must be readable for one word.
///
/// # Confidence
/// high - every term of the size formula is literal.
pub unsafe fn fn_80166750(a: usize, b: usize, count: u32, align: i32) -> u32 {
    let mut total = crate::platform::GuestLocal::<u32>::new( 0);
    let mut first = crate::platform::GuestLocal::new( platform::read_u32(a));
    platform::call4(
        W64_FN_80245300,
        0,
        (total.as_mut_ptr() as *mut u32) as usize as u32,
        (first.as_mut_ptr() as *mut u32) as usize as u32,
        count,
    );
    *total = total.wrapping_add(count.wrapping_mul(0x38).wrapping_add(0x1F) & 0xFFFF_FFE0);
    let element_a = crate::platform::GuestLocal::new( platform::read_u32(a));
    let element_b = crate::platform::GuestLocal::new( platform::read_u32(b));
    let element_size = fn_80166420(
        (element_a.as_ptr() as *const u32) as usize,
        (element_b.as_ptr() as *const u32) as usize,
        1,
    );
    *total = total.wrapping_add(
        count.wrapping_mul(element_size.wrapping_add(0x1F) & 0xFFFF_FFE0),
    );
    if align != 0 {
        let advanced = crate::ui::lyt::fn_8016e7d0(*total as i32, 0x20);
        *total = (advanced as u32).wrapping_add(0x1F) & 0xFFFF_FFE0;
    }
    *total
}

/// Deleting destructor for the class-B element.
///
/// Installs [`W64_CLASS_B_VTABLE`], runs the class teardown at `0x80166AE0`,
/// forwards to [`fn_80167ec0`], then frees the object when `count` is positive.
///
/// # Original
/// - Ghidra: `FUN_80166a70` at `0x80166a70` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166a70_FUN_80166a70.c`
/// - observed: `if (param_1 != (undefined4 *)0x0) { *param_1 = &DAT_80329548;
///   FUN_80166ae0(); FUN_80167ec0(param_1,0); if (0 < param_2) {
///   FUN_802b9280(param_1); } } return param_1;`
///
/// # Arguments
/// - `this`: element object, or zero.
/// - `count`: positive to free the object after teardown.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Writes the vtable word, releases the element array and may free `this`.
///
/// # Safety
/// A non-null `this` must be a live class-B element; after a positive `count`
/// it must not be used again.
///
/// # Confidence
/// medium - the vtable, two calls and the free are literal; the teardown
/// callee's body is still untranslated.
pub unsafe fn fn_80166a70(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, W64_CLASS_B_VTABLE as u32);
        platform::call1(W64_FN_80166AE0, this as u32);
        fn_80167ec0(this, 0);
        if count > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Releases one class-B element when it still owns a native object.
///
/// # Original
/// - Ghidra: `FUN_80166e80` at `0x80166e80` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166e80_FUN_80166e80.c`
/// - observed: `param_2 = param_2 * 0x38;
///   iVar1 = FUN_80168060(*(int *)(param_1 + 0x28) + param_2); if (iVar1 != 0) {
///   (**(code **)(*(int *)(param_2 + *(int *)(param_1 + 0x28)) + 0x14))(...); }`
///
/// # Arguments
/// - `this`: element owner whose `+0x28` word points at the array.
/// - `index`: element index, multiplied by the `0x38` stride.
///
/// # Side effects
/// Runs one virtual call on the element.
///
/// # Safety
/// `this + 0x28` must address an array with at least `index + 1` entries and
/// each entry's vtable word must be live.
///
/// # Confidence
/// high - the stride, the tag test and the virtual slot are literal.
pub unsafe fn fn_80166e80(this: usize, index: u32) {
    let offset = index.wrapping_mul(0x38) as usize;
    let element = platform::read_u32(this + 0x28) as usize + offset;
    if fn_80168060(element) != 0 {
        let vtable = platform::read_u32(element) as usize;
        let slot = platform::read_u32(vtable + 0x14) as usize;
        platform::call1(slot, element as u32);
    }
}

/// Returns the space needed for one class-C element instance.
///
/// Identical to [`fn_80165820`] except for the constructor query address
/// (`0x80246B00`).
///
/// # Original
/// - Ghidra: `FUN_80167000` at `0x80167000` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167000_FUN_80167000.c`
/// - observed: `FUN_80246b00(0,local_10,&local_14,&local_18,0);
///   if (param_3 != 0) { iVar1 = FUN_8016e7d0(local_10[0],0x20); ... }`
///
/// # Arguments
/// - `a`, `b`: addresses of the two constructor words.
/// - `align`: non-zero to add the aligned heap-header allowance.
///
/// # Returns
/// The element size in bytes.
///
/// # Side effects
/// None; only the local size word is written.
///
/// # Safety
/// Both addresses must be readable for one word.
///
/// # Confidence
/// high - the five call arguments and the alignment arithmetic are literal.
pub unsafe fn fn_80167000(a: usize, b: usize, align: i32) -> u32 {
    let mut size = crate::platform::GuestLocal::<u32>::new( 0);
    let first = crate::platform::GuestLocal::new( platform::read_u32(a));
    let mut second = crate::platform::GuestLocal::new( platform::read_u32(b));
    platform::call5(
        W64_FN_80246B00,
        0,
        (size.as_mut_ptr() as *mut u32) as usize as u32,
        (second.as_mut_ptr() as *mut u32) as usize as u32,
        (first.as_ptr() as *const u32) as usize as u32,
        0,
    );
    if align != 0 {
        let advanced = crate::ui::lyt::fn_8016e7d0(*size as i32, 0x20);
        *size = (advanced as u32).wrapping_add(0x1F) & 0xFFFF_FFE0;
    }
    *size
}

/// Releases the class-C element's `+0x04` child and returns its buffer to heap
/// class 3.
///
/// # Original
/// - Ghidra: `FUN_80167270` at `0x80167270` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167270_FUN_80167270.c`
/// - observed: `if (*(int **)(param_1 + 4) != (int *)0x0) {
///   (**(code **)(**(int **)(param_1 + 4) + 0x34))();
///   FUN_802b99c0(*(undefined4 *)(param_1 + 8),3);
///   *(undefined4 *)(param_1 + 4) = 0; }`
///
/// # Arguments
/// - `this`: element wrapper.
///
/// # Side effects
/// Runs virtual slot `+0x34` of the child, frees the `+0x08` buffer and clears
/// `+0x04`.
///
/// # Safety
/// A non-null `+0x04` must be a live EGG object and `+0x08` its heap block.
///
/// # Confidence
/// high - the guard, the virtual call, the heap-class argument and the clear
/// are literal.
pub unsafe fn fn_80167270(this: usize) {
    let child = platform::read_u32(this + 0x04) as usize;
    if child != 0 {
        let vtable = platform::read_u32(child) as usize;
        let slot = platform::read_u32(vtable + 0x34) as usize;
        platform::call1(slot, child as u32);
        platform::call2(W64_FN_802B99C0, platform::read_u32(this + 0x08), 3);
        platform::write_u32(this + 0x04, 0);
    }
}

/// Returns the space needed for `count` class-C elements plus their table.
///
/// Identical to [`fn_80165b50`] except for the base query address
/// (`0x80246900`) and the per-element query [`fn_80167000`].
///
/// # Original
/// - Ghidra: `FUN_80167340` at `0x80167340` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167340_FUN_80167340.c`
/// - observed: `FUN_80246900(0,local_1c,&local_20,param_3);
///   local_1c[0] = local_1c[0] + (param_3 * 0x38 + 0x1fU & 0xffffffe0); ...
///   iVar1 = FUN_80167000(&local_24,&local_28,1); ...`
///
/// # Arguments
/// - `a`, `b`: addresses of the two constructor words.
/// - `count`: number of elements.
/// - `align`: non-zero to add the aligned heap-header allowance.
///
/// # Returns
/// The total size in bytes.
///
/// # Side effects
/// None; only local size words are written.
///
/// # Safety
/// Both addresses must be readable for one word.
///
/// # Confidence
/// high - every term of the size formula is literal.
pub unsafe fn fn_80167340(a: usize, b: usize, count: u32, align: i32) -> u32 {
    let mut total = crate::platform::GuestLocal::<u32>::new( 0);
    let mut first = crate::platform::GuestLocal::new( platform::read_u32(a));
    platform::call4(
        W64_FN_80246900,
        0,
        (total.as_mut_ptr() as *mut u32) as usize as u32,
        (first.as_mut_ptr() as *mut u32) as usize as u32,
        count,
    );
    *total = total.wrapping_add(count.wrapping_mul(0x38).wrapping_add(0x1F) & 0xFFFF_FFE0);
    let element_a = crate::platform::GuestLocal::new( platform::read_u32(a));
    let element_b = crate::platform::GuestLocal::new( platform::read_u32(b));
    let element_size = fn_80167000(
        (element_a.as_ptr() as *const u32) as usize,
        (element_b.as_ptr() as *const u32) as usize,
        1,
    );
    *total = total.wrapping_add(
        count.wrapping_mul(element_size.wrapping_add(0x1F) & 0xFFFF_FFE0),
    );
    if align != 0 {
        let advanced = crate::ui::lyt::fn_8016e7d0(*total as i32, 0x20);
        *total = (advanced as u32).wrapping_add(0x1F) & 0xFFFF_FFE0;
    }
    *total
}

/// Deleting destructor for the class-C element.
///
/// Installs [`W64_CLASS_C_VTABLE`], runs the class teardown at `0x80167710`,
/// forwards to [`fn_80167ec0`], then frees the object when `count` is positive.
///
/// # Original
/// - Ghidra: `FUN_801676a0` at `0x801676a0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801676a0_FUN_801676a0.c`
/// - observed: `if (param_1 != (undefined4 *)0x0) { *param_1 = &DAT_80329578;
///   FUN_80167710(); FUN_80167ec0(param_1,0); if (0 < param_2) {
///   FUN_802b9280(param_1); } } return param_1;`
///
/// # Arguments
/// - `this`: element object, or zero.
/// - `count`: positive to free the object after teardown.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Writes the vtable word, releases the element array and may free `this`.
///
/// # Safety
/// A non-null `this` must be a live class-C element; after a positive `count`
/// it must not be used again.
///
/// # Confidence
/// medium - the vtable, two calls and the free are literal; the teardown
/// callee's body is still untranslated.
pub unsafe fn fn_801676a0(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, W64_CLASS_C_VTABLE as u32);
        platform::call1(W64_FN_80167710, this as u32);
        fn_80167ec0(this, 0);
        if count > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Releases one class-C element when it still owns a native object.
///
/// # Original
/// - Ghidra: `FUN_80167ab0` at `0x80167ab0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167ab0_FUN_80167ab0.c`
/// - observed: `param_2 = param_2 * 0x38;
///   iVar1 = FUN_80168060(*(int *)(param_1 + 0x28) + param_2); if (iVar1 != 0) {
///   (**(code **)(*(int *)(param_2 + *(int *)(param_1 + 0x28)) + 0x14))(...); }`
///
/// # Arguments
/// - `this`: element owner whose `+0x28` word points at the array.
/// - `index`: element index, multiplied by the `0x38` stride.
///
/// # Side effects
/// Runs one virtual call on the element.
///
/// # Safety
/// `this + 0x28` must address an array with at least `index + 1` entries and
/// each entry's vtable word must be live.
///
/// # Confidence
/// high - the stride, the tag test and the virtual slot are literal.
pub unsafe fn fn_80167ab0(this: usize, index: u32) {
    let offset = index.wrapping_mul(0x38) as usize;
    let element = platform::read_u32(this + 0x28) as usize + offset;
    if fn_80168060(element) != 0 {
        let vtable = platform::read_u32(element) as usize;
        let slot = platform::read_u32(vtable + 0x14) as usize;
        platform::call1(slot, element as u32);
    }
}

/// Shared base destructor for the three element classes.
///
/// Restores the base vtable [`INTERP_BASE_VTABLE`], releases the `+0x04`/`+0x08`
/// fields through [`fn_80167f30`], destroys the sub-object at `+0x0C` and frees
/// the object when `count` is positive.
///
/// # Original
/// - Ghidra: `FUN_80167ec0` at `0x80167ec0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167ec0_FUN_80167ec0.c`
/// - observed: `if (param_1 != (undefined4 *)0x0) { *param_1 = &DAT_803295d8;
///   FUN_80167f30(); FUN_8016a630(param_1 + 3,0xffffffff);
///   if (0 < param_2) { FUN_802b9280(param_1); } } return param_1;`
///
/// # Arguments
/// - `this`: derived element object, or zero.
/// - `count`: positive to free the object after teardown.
///
/// # Returns
/// `this` unchanged.
///
/// # Side effects
/// Writes the base vtable word, clears both child fields, destroys the
/// `+0x0C` view and may free `this`.
///
/// # Safety
/// A non-null `this` must be a live element object; after a positive `count`
/// it must not be used again.
///
/// # Confidence
/// high - the guard, vtable store, two teardown calls and the conditional free
/// are literal.
pub unsafe fn fn_80167ec0(this: usize, count: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, INTERP_BASE_VTABLE as u32);
        fn_80167f30(this);
        fn_8016a630(this + 0x0C, -1);
        if count > 0 {
            platform::call1(HEAP_DELETE, this as u32);
        }
    }
    this
}

/// Releases the element's `+0x04` and `+0x08` fields.
///
/// The `+0x04` native object is destroyed and released (`0x80249B40` then
/// `0x80249AC0`); the `+0x08` frame heap is destroyed through
/// [`fn_8016e740`]. Both fields are zeroed.
///
/// # Original
/// - Ghidra: `FUN_80167f30` at `0x80167f30` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167f30_FUN_80167f30.c`
/// - observed: `if (*(int *)(param_1 + 4) != 0) { FUN_80249b40(...);
///   FUN_80249ac0(...); *(undefined4 *)(param_1 + 4) = 0; }
///   if (*(int *)(param_1 + 8) != 0) { FUN_8016e740();
///   *(undefined4 *)(param_1 + 8) = 0; }`
///
/// # Arguments
/// - `this`: element object whose two child fields are released.
///
/// # Side effects
/// Two object teardowns, one heap teardown and two field clears.
///
/// # Safety
/// A non-null `+0x04` must be a native object accepted by `0x80249B40`/
/// `0x80249AC0`; a non-null `+0x08` must be a live frame heap.
///
/// # Confidence
/// medium - the guards, calls and clears are literal; the `0x8016E740`
/// argument is restored from the field just tested.
pub unsafe fn fn_80167f30(this: usize) {
    let first = platform::read_u32(this + 0x04);
    if first != 0 {
        platform::call1(W64_FN_80249B40, first);
        platform::call1(W64_FN_80249AC0, first);
        platform::write_u32(this + 0x04, 0);
    }
    let second = platform::read_u32(this + 0x08);
    if second != 0 {
        fn_8016e740(second as usize);
        platform::write_u32(this + 0x08, 0);
    }
}

/// Frees an object when the deleting flag is positive and returns it.
///
/// # Original
/// - Ghidra: `FUN_8016c6f0` at `0x8016c6f0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c6f0_FUN_8016c6f0.c`
/// - observed: `if ((param_1 != 0) && (0 < param_2)) { FUN_802b9280(); }
///   return param_1;`
///
/// # Arguments
/// - `this`: candidate object, or zero.
/// - `count`: positive to free the object.
///
/// # Returns
/// `this` unchanged.
///
/// # Safety
/// A non-null `this` must be a block accepted by the default deleter
/// [`HEAP_DELETE`], and must not be used again when `count > 0`.
///
/// # Confidence
/// high - the two guards, the call and the returned word are literal.
pub unsafe fn fn_8016c6f0(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(HEAP_DELETE, this as u32);
    }
    this
}

/// Reports whether the descriptor registered at `+0x74` is live and copies it
/// into `+0x7C`.
///
/// Repeated existence checks are performed; the descriptor's state word
/// (`+0x0C`) must be `1` or `2` before the copy runs.
///
/// # Original
/// - Ghidra: `FUN_8016cfd0` at `0x8016cfd0` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016cfd0_FUN_8016cfd0.c`
/// - observed: `iVar2 = FUN_802967b0(param_1 + 0x74); ... iVar2 = FUN_802967e0(...);
///   bVar1 = false; if ((iVar2 == 1) || (iVar2 == 2)) bVar1 = true;
///   if (bVar1) { ... FUN_802da6f0(iVar2,param_1 + 0x7c); uVar3 = 1; }`
///
/// # Arguments
/// - `this`: owner object; the descriptor register lives at `+0x74`.
///
/// # Returns
/// `1` when the descriptor was copied, otherwise `0`.
///
/// # Side effects
/// Copies the descriptor into `+0x7C` through `0x802DA6F0`.
///
/// # Safety
/// `this` must be a live owner; the registered descriptor must remain valid
/// across the three predicate calls.
///
/// # Confidence
/// medium - the predicate sequence, state test and copy are literal; the
/// descriptor field roles are inferred from the neighbouring UI code.
pub unsafe fn fn_8016cfd0(this: usize) -> u32 {
    let key = (this + 0x74) as u32;
    if platform::call1(W64_FN_802967B0, key) == 0 {
        return 0;
    }
    if platform::call1(W64_FN_802967B0, key) == 0 {
        return 0;
    }
    let object = platform::call1(W64_FN_802967E0, key) as usize;
    let state = if object == 0 {
        0
    } else {
        platform::read_u32(object + 0x0C)
    };
    if state != 1 && state != 2 {
        return 0;
    }
    let object = platform::call1(W64_FN_802967E0, key) as usize;
    if object == 0 {
        return 0;
    }
    platform::call2(W64_FN_802DA6F0, object as u32, (this + 0x7C) as u32);
    1
}

/// Frees an object when the deleting flag is positive and returns it.
///
/// Twin of [`fn_8016c6f0`] at the second UI owner address.
///
/// # Original
/// - Ghidra: `FUN_8016dca0` at `0x8016dca0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dca0_FUN_8016dca0.c`
/// - observed: `if ((param_1 != 0) && (0 < param_2)) { FUN_802b9280(); }
///   return param_1;`
///
/// # Arguments
/// - `this`: candidate object, or zero.
/// - `count`: positive to free the object.
///
/// # Returns
/// `this` unchanged.
///
/// # Safety
/// A non-null `this` must be a block accepted by [`HEAP_DELETE`], and must not
/// be used again when `count > 0`.
///
/// # Confidence
/// high - the two guards, the call and the returned word are literal.
pub unsafe fn fn_8016dca0(this: usize, count: i32) -> usize {
    if this != 0 && count > 0 {
        platform::call1(HEAP_DELETE, this as u32);
    }
    this
}

/// Initialises a UI entry: constructs the inner object, stores the descriptor
/// word, clears the 20-byte counter block and marks both state bytes.
///
/// # Original
/// - Ghidra: `FUN_8016ff60` at `0x8016ff60` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ff60_FUN_8016ff60.c`
/// - observed: `FUN_80170630(); FUN_8016ffc0(param_1,param_4);
///   FUN_80228dd0(param_1 + 8,0x14); *(undefined1 *)(param_1 + 0x14) = 1;
///   *(undefined1 *)(param_1 + 0x15) = 1;`
///
/// # Arguments
/// - `this`: entry being initialised.
/// - `arg2`, `arg3`: forwarded to the inner constructor `0x80170630`.
/// - `arg4`: descriptor word stored at `+0x04`.
///
/// # Side effects
/// Writes `+0x00` (inner object), `+0x04` (descriptor), clears `+0x08..+0x1B`
/// and sets `+0x14`/`+0x15`.
///
/// # Safety
/// `this` must be writable through `+0x1B`; `0x80170630` must accept the two
/// forwarded words.
///
/// # Confidence
/// medium - the call order, buffer size and two byte stores are literal; the
/// inner constructor's arguments are inferred from its own export.
pub unsafe fn fn_8016ff60(this: usize, arg2: u32, arg3: u32, arg4: u32) {
    platform::call3(W64_FN_80170630, this as u32, arg2, arg3);
    platform::call2(W64_FN_8016FFC0, this as u32, arg4);
    platform::call2(FN_UT_LIST_INIT, (this + 8) as u32, 0x14);
    platform::write_u8(this + 0x14, 1);
    platform::write_u8(this + 0x15, 1);
}

// ===========================================================================
// Element entry, cast and list-release layer (wave 78)
//
// The `0x80160000` page still held the *second half* of the `0x38`-stride
// element classes: the per-entry release, the whole-array teardown, the
// `dynamic_cast` helper family around `0x80168400`, and the sound-object leaf
// that forwards a typed child. These are exactly the bodies the wave-64
// destructors already called by absolute address (`W64_FN_80165EE0`,
// `W64_FN_80166AE0`, `W64_FN_80167710`), so this pass closes the loop instead
// of opening a new subsystem.
//
// Two idioms are worth knowing before reading the bodies:
//
// * `FUN_802dcd5c`/`64`/`68`/`6c`/`70`/`74` are Metrowerks **register-save
//   stubs**: they return the incoming `r3:r4` pair. The two dropped arguments
//   are spelled out as parameters, exactly as wave 38 did for
//   [`fn_8016e500`]. `FUN_802dcdb0`/`b4`/`b8`/`bc` are the matching
//   return-register stubs, so the function returns its value normally here.
// * `p = *(int **)(this + 4); local = &type; (*p->vtbl[2])(p, &local)` is the
//   EGG `dynamic_cast<T>`: the stack word holds the type-info pointer and its
//   **address** is the argument. It appears a dozen times below, so it is
//   factored into `w78_cast_to`.
//
// `r13` is [`SDA_BASE`], so every `r13 - 0xNNNN` global keeps that form.
// ===========================================================================

/// `SDA_BASE - 0x52B8` - default EGG heap used by the attach helpers (the same
/// SDA word as `DEFAULT_GROUP`, which is the sound-bank view of it).
const W78_DEFAULT_HEAP: usize = SDA_BASE - 0x52B8;

/// `SDA_BASE - 0x52A0` - frame heap passed as the fourth word to `0x8016E630`.
const W78_G_FRAME_HEAP: usize = SDA_BASE - 0x52A0;

/// `SDA_BASE - 0x52A8` - number of slots behind [`G_CTX_AUX_SLOTS`].
const W78_G_AUX_SLOT_COUNT: usize = SDA_BASE - 0x52A8;

/// `SDA_BASE - 0x524C` - handle that owns the `0x80377C3C` intrusive list.
const W78_G_ACTIVE_LIST: usize = SDA_BASE - 0x524C;

/// `0x80377C3C` - sentinel node *and* head link of that list.
const W78_LIST_SENTINEL: usize = 0x8037_7C3C;

/// `0x80377C40` - tail link of the same list.
const W78_LIST_TAIL: usize = 0x8037_7C40;

/// `0x80377C44` - element count of the same list.
const W78_LIST_COUNT: usize = 0x8037_7C44;

/// `0x802F8FD8` - type-info word of a generic layout element.
const W78_TYPE_ELEMENT: usize = 0x802F_8FD8;

/// `0x802F8588` - type-info word of a class-A element.
const W78_TYPE_CLASS_A: usize = 0x802F_8588;

/// `0x802F85F0` - type-info word of a class-B element.
const W78_TYPE_CLASS_B: usize = 0x802F_85F0;

/// `0x802F8658` - type-info word of a class-C element.
const W78_TYPE_CLASS_C: usize = 0x802F_8658;

/// `0x802F8FF0` - type-info word of the sound-object owner.
const W78_TYPE_OBJECT: usize = 0x802F_8FF0;

/// `0x802F86B4` - type-info word of the object updated by [`fn_801655e0`].
const W78_TYPE_UPDATE: usize = 0x802F_86B4;

/// `0x802F86C8` - type-info word of the holder forwarded by
/// [`fn_801656c0`].
const W78_TYPE_HOLDER: usize = 0x802F_86C8;

/// `0x803295A8` - heap/type descriptor word given to the `0x8016E630`
/// allocator by [`fn_80167fa0`].
const W78_HEAP_TAG: usize = 0x8032_95A8;

/// `0x80161CC0` - install three transform rows on the sound object.
const W78_FN_80161CC0: usize = 0x8016_1CC0;

/// `0x8016C9E0` - build the request packet handed to the layout dispatcher.
const W78_FN_8016C9E0: usize = 0x8016_C9E0;

/// `0x8016E630` - frame-heap allocator used by [`fn_80167fa0`].
const W78_FN_8016E630: usize = 0x8016_E630;

/// `0x8016ED10` - format one message id into a 60-byte stack packet.
const W78_FN_8016ED10: usize = 0x8016_ED10;

/// `0x8016EE30` - second formatter used by [`fn_8016eef0`].
const W78_FN_8016EE30: usize = 0x8016_EE30;

/// `0x801C0540` - submit a formatted message packet.
const W78_FN_801C0540: usize = 0x801C_0540;

/// `0x801C9D60` - flush the layout command buffer.
const W78_FN_801C9D60: usize = 0x801C_9D60;

/// `0x802481B0` - resolve one element id into the element container.
const W78_FN_802481B0: usize = 0x8024_81B0;

/// `0x802B9240` - allocate `size` bytes from an explicit heap.
const W78_FN_802B9240: usize = 0x802B_9240;

/// `0x802B9730` - destroy the owner of the `0x80377C3C` list.
const W78_FN_802B9730: usize = 0x802B_9730;

/// `0x802C6550` - construct one auxiliary slot object from a `u16` id.
const W78_FN_802C6550: usize = 0x802C_6550;

/// `0x802D9500` - flush the layout queue behind `r13 - 0x46B0`.
const W78_FN_802D9500: usize = 0x802D_9500;

/// `0x802D9630` - submit three words to the layout queue.
const W78_FN_802D9630: usize = 0x802D_9630;

/// Runs the EGG `dynamic_cast<T>` idiom shared by the element classes.
///
/// The original keeps the type-info pointer in a stack word and passes the
/// **address** of that word to the object's `+0x08` slot; the slot returns
/// non-zero when the object really is a `T`. A null object or a failed cast
/// yields `0`, which is exactly what the export does.
///
/// # Safety
/// `object` must be zero or point at a live object whose vtable word is
/// readable.
unsafe fn w78_cast_to(object: usize, type_info: usize) -> usize {
    if object == 0 {
        return 0;
    }
    let mut info = crate::platform::GuestLocal::new( type_info as u32);
    let vtable = platform::read_u32(object) as usize;
    let slot = platform::read_u32(vtable + 0x08) as usize;
    if platform::call2(slot, object as u32, (info.as_mut_ptr() as *mut u32) as usize as u32) != 0 {
        object
    } else {
        0
    }
}

/// Copies three consecutive big-endian words starting at `base` into `out`.
///
/// # Safety
/// `base` must be readable for three words.
unsafe fn w78_read_row3(out: &mut [u32; 3], base: usize) {
    let mut index = 0;
    while index < 3 {
        out[index] = platform::read_u32(base + index * 4);
        index += 1;
    }
}

/// Applies one sound-bank list transition to the bank object.
///
/// `stage == 2` unlinks the two list nodes at `+0x24`/`+0x34` and sets the
/// pending byte at `+0x0A`, unless the current sound group is already `3`, in
/// which case only the `+0x0C` "done" byte is set. `stage == 1` marks the
/// object and its whole child chain.
///
/// # Original
/// - Ghidra: `FUN_80161da0` at `0x80161da0` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161da0_FUN_80161da0.c`
/// - observed: `if (param_2 == 2) { FUN_80160f60(&DAT_80377a20,param_1 + 0x24);
///   if (*(int *)(unaff_r13 + -0x6228) == 3) { *(undefined1 *)(param_1 + 0xc) = 1; }
///   else { FUN_80162b40(&DAT_80377a34,param_1 + 0x24);
///   FUN_80162b40(&DAT_80377a48,param_1 + 0x34); *(undefined1 *)(param_1 + 10) = 1; } }
///   else if (param_2 == 1) { FUN_80162510(); }`
///
/// # Arguments
/// - `this`: bank object; `+0x24` and `+0x34` are its two list nodes, `+0x0A`
///   the pending byte and `+0x0C` the done byte.
/// - `stage`: `1` = mark the chain, `2` = run the transition; anything else
///   does nothing.
///
/// # Side effects
/// Unlinks or links the two nodes and writes one of the two state bytes.
///
/// # Safety
/// `this` must be a live bank object while the global sound-bank lists are
/// being edited.
///
/// # Confidence
/// medium - every call and byte store is literal; `FUN_80162510` takes `this`
/// although the export dropped the argument, and the three list identifiers
/// come from the wave-40 sound-bank map.
pub unsafe fn fn_80161da0(this: usize, stage: u32) {
    if stage == 2 {
        fn_80160f60(0x8037_7A20, this + 0x24);
        if platform::read_u32(G_SB_CURRENT_GROUP) == 3 {
            platform::write_u8(this + 0x0C, 1);
        } else {
            fn_80162b40(0x8037_7A34, this + 0x24);
            fn_80162b40(0x8037_7A48, this + 0x34);
            platform::write_u8(this + 0x0A, 1);
        }
    } else if stage == 1 {
        fn_80162510(this);
    }
}

/// Copies three 3-word transform rows out of the `0x80329350`..`0x80329368`
/// globals and hands them to `0x80161CC0` low row first.
///
/// # Original
/// - Ghidra: `FUN_80161e40` at `0x80161e40` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161e40_FUN_80161e40.c`
/// - observed: `local_38 = DAT_80329368; local_34 = DAT_8032936c;
///   local_30 = DAT_80329370; ... local_18 = DAT_80329358;
///   FUN_80161cc0(param_1,&local_20,&local_2c,&local_38);`
///
/// # Arguments
/// - `context`: forwarded unchanged as the first argument.
///
/// # Side effects
/// Calls `0x80161CC0` with three stack rows. This file leaves that callee
/// untranslated because its own export loses the argument registers.
///
/// # Safety
/// `context` must be what `0x80161CC0` expects; the three rows are stack
/// copies of read-only globals.
///
/// # Confidence
/// medium - the nine loads and the three pointers are literal; the identity of
/// `0x80161CC0` is inferred from its four sibling wrappers.
pub unsafe fn fn_80161e40(context: u32) {
    let mut low = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut middle = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut high = crate::platform::GuestLocal::new( [0u32; 3]);
    w78_read_row3(&mut low, 0x8032_9350);
    w78_read_row3(&mut middle, 0x8032_935C);
    w78_read_row3(&mut high, 0x8032_9368);
    platform::call4(
        W78_FN_80161CC0,
        context,
        low.as_mut_ptr() as usize as u32,
        middle.as_mut_ptr() as usize as u32,
        high.as_mut_ptr() as usize as u32,
    );
}

/// Same three-row install as [`fn_80161e40`] with the second row block.
///
/// # Original
/// - Ghidra: `FUN_80162020` at `0x80162020` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162020_FUN_80162020.c`
/// - observed: `local_38 = DAT_8032938c; ... local_2c = DAT_80329380;
///   local_28 = DAT_80329384; local_24 = DAT_80329388; ... local_20 = DAT_80329374;
///   ... FUN_80161cc0(param_1,&local_20,&local_2c,&local_38);`
///
/// # Arguments
/// - `context`: forwarded unchanged as the first argument.
///
/// # Side effects
/// Calls `0x80161CC0` with three stack rows.
///
/// # Safety
/// Same as [`fn_80161e40`].
///
/// # Confidence
/// medium - the loads and pointers are literal; the callee is inferred.
pub unsafe fn fn_80162020(context: u32) {
    let mut low = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut middle = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut high = crate::platform::GuestLocal::new( [0u32; 3]);
    w78_read_row3(&mut low, 0x8032_9374);
    w78_read_row3(&mut middle, 0x8032_9380);
    w78_read_row3(&mut high, 0x8032_938C);
    platform::call4(
        W78_FN_80161CC0,
        context,
        low.as_mut_ptr() as usize as u32,
        middle.as_mut_ptr() as usize as u32,
        high.as_mut_ptr() as usize as u32,
    );
}

/// Same three-row install as [`fn_80161e40`] with the third row block.
///
/// # Original
/// - Ghidra: `FUN_80162120` at `0x80162120` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162120_FUN_80162120.c`
/// - observed: `local_38 = DAT_803293b0; ... local_2c = DAT_803293a4;
///   local_20 = DAT_80329398; ... FUN_80161cc0(param_1,&local_20,&local_2c,&local_38);`
///
/// # Arguments
/// - `context`: forwarded unchanged as the first argument.
///
/// # Side effects
/// Calls `0x80161CC0` with three stack rows.
///
/// # Safety
/// Same as [`fn_80161e40`].
///
/// # Confidence
/// medium - the loads and pointers are literal; the callee is inferred.
pub unsafe fn fn_80162120(context: u32) {
    let mut low = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut middle = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut high = crate::platform::GuestLocal::new( [0u32; 3]);
    w78_read_row3(&mut low, 0x8032_9398);
    w78_read_row3(&mut middle, 0x8032_93A4);
    w78_read_row3(&mut high, 0x8032_93B0);
    platform::call4(
        W78_FN_80161CC0,
        context,
        low.as_mut_ptr() as usize as u32,
        middle.as_mut_ptr() as usize as u32,
        high.as_mut_ptr() as usize as u32,
    );
}

/// Same three-row install as [`fn_80161e40`] with the last row block.
///
/// # Original
/// - Ghidra: `FUN_80162220` at `0x80162220` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162220_FUN_80162220.c`
/// - observed: `local_38 = DAT_803293d4; ... local_2c = DAT_803293c8;
///   local_20 = DAT_803293bc; ... FUN_80161cc0(param_1,&local_20,&local_2c,&local_38);`
///
/// # Arguments
/// - `context`: forwarded unchanged as the first argument.
///
/// # Side effects
/// Calls `0x80161CC0` with three stack rows.
///
/// # Safety
/// Same as [`fn_80161e40`].
///
/// # Confidence
/// medium - the loads and pointers are literal; the callee is inferred.
pub unsafe fn fn_80162220(context: u32) {
    let mut low = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut middle = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut high = crate::platform::GuestLocal::new( [0u32; 3]);
    w78_read_row3(&mut low, 0x8032_93BC);
    w78_read_row3(&mut middle, 0x8032_93C8);
    w78_read_row3(&mut high, 0x8032_93D4);
    platform::call4(
        W78_FN_80161CC0,
        context,
        low.as_mut_ptr() as usize as u32,
        middle.as_mut_ptr() as usize as u32,
        high.as_mut_ptr() as usize as u32,
    );
}

/// Walks a nested element list depth-first until a predicate accepts a node.
///
/// Two callback words drive it. When `filter` is non-zero it is asked about the
/// node itself and a result of `1` prunes that subtree - the leaf predicate is
/// then not even called. Otherwise `leaf` is called with `(node, mode)` and a
/// non-zero result ends the walk. In both remaining cases the node's circular
/// child list at `+0x14` is visited; each link's `+0x04` word minus `4` is the
/// child payload.
///
/// # Original
/// - Ghidra: `FUN_80163d40` at `0x80163d40` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163d40_FUN_80163d40.c`
/// - observed: `if (((param_4 != (code *)0x0) && (iVar1 = (*param_4)(iVar2),
///   iVar1 != 1)) || (iVar3 = (*param_3)(iVar2,param_5), iVar3 == 0)) {
///   iVar1 = iVar2 + 0x14; while ((iVar1 != *(int *)(iVar2 + 0x14) &&
///   (iVar3 = FUN_80163d40(...), iVar3 == 0))) { iVar1 = *(int *)(iVar1 + 4); } }`
///
/// # Arguments
/// - `context`: first dropped word; only forwarded to the recursive call.
/// - `node`: object whose `+0x14` field starts the child list.
/// - `leaf`: predicate `(node, mode) -> u32`; non-zero stops the walk.
/// - `filter`: optional predicate `(node) -> u32`; `1` prunes the subtree.
/// - `mode`: forwarded to the leaf predicate.
///
/// # Returns
/// The predicate value that stopped the walk, or `0`.
///
/// # Side effects
/// Runs the two callbacks; writes nothing itself.
///
/// # Safety
/// `node` must be live and readable at `+0x14`, and both callbacks must accept
/// the receiver they are given. A non-zero `leaf` of zero still crashes, as in
/// the original.
///
/// # Confidence
/// medium - the control flow, the `-4` payload bias and the register-save stub
/// are literal; the two callback roles are inferred from their use.
pub unsafe fn fn_80163d40(
    context: u32,
    node: usize,
    leaf: usize,
    filter: usize,
    mode: u32,
) -> u32 {
    let mut result = 0u32;
    let mut descend = false;
    if filter != 0 {
        if platform::call1(filter, node as u32) != 1 {
            descend = true;
        }
    } else {
        result = platform::call2(leaf, node as u32, mode);
        if result == 0 {
            descend = true;
        }
    }
    if descend {
        let header = node + 0x14;
        let head = platform::read_u32(header) as usize;
        let mut link = header;
        while link != head {
            let child = platform::read_u32(link + 4) as usize;
            result = fn_80163d40(context, child.wrapping_sub(4), leaf, filter, mode);
            if result != 0 {
                break;
            }
            link = child;
        }
    }
    result
}

/// Allocates one auxiliary slot object and publishes it in the
/// `r13 - 0x52A4` table.
///
/// The index is rejected when it is negative, beyond the count at
/// `r13 - 0x52A8`, or already occupied. On success a `0x10`-byte block is taken
/// from `heap` and constructed by `0x802C6550(block, id)`.
///
/// # Original
/// - Ghidra: `FUN_80164990` at `0x80164990` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164990_FUN_80164990.c`
/// - observed: `if ((((int)param_3 < 0) || (*(uint *)(unaff_r13 + -0x52a8) <= param_3))
///   || (*(int *)(*(int *)(unaff_r13 + -0x52a4) + param_3 * 4) != 0)) { uVar1 = 0; }
///   else { iVar2 = FUN_802b9240(0x10,param_1,4); ... }`
///
/// # Arguments
/// - `heap`: EGG heap passed to the allocator.
/// - `id`: `u16` stored in the new slot object.
/// - `index`: table slot to fill.
///
/// # Returns
/// `1` when the slot was created, otherwise `0`.
///
/// # Side effects
/// Allocates a `0x10`-byte block and writes the slot pointer.
///
/// # Safety
/// The table behind [`G_CTX_AUX_SLOTS`] must hold at least `index + 1`
/// pointers and `heap` must be a live EGG heap.
///
/// # Confidence
/// medium - the guard, allocation and construction are literal; that the
/// `0x52A4` table is the per-index auxiliary slot table is inferred from the
/// wave-41 live-context bodies.
pub unsafe fn fn_80164990(heap: u32, id: u16, index: i32) -> u32 {
    let count = platform::read_u32(W78_G_AUX_SLOT_COUNT);
    if index < 0 || index as u32 >= count {
        return 0;
    }
    let table = platform::read_u32(G_CTX_AUX_SLOTS) as usize;
    if platform::read_u32(table + index as usize * 4) != 0 {
        return 0;
    }
    let block = platform::call3(W78_FN_802B9240, 0x10, heap, 4);
    let object = if block == 0 {
        0
    } else {
        platform::call2(W78_FN_802C6550, block, id as u32)
    };
    if object == 0 {
        return 0;
    }
    platform::write_u32(table + index as usize * 4, object);
    1
}

/// Builds the typed child object of one element entry through `0x802481B0`.
///
/// The element id word is resolved twice: first with the container word `0`,
/// then with `this + 0x0C`. The intermediate `fn_80167fa0` call rounds the
/// size word in the caller's buffer and creates the backing heap; the second
/// resolution result becomes the entry's `+0x04` child.
///
/// # Original
/// - Ghidra: `FUN_80165520` at `0x80165520` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165520_FUN_80165520.c`
/// - observed: `uVar4 = FUN_802dcd74(); iVar1 = high; if (param_4 == 0)
///   param_4 = *(r13 - 0x52b8); if (param_5 == 0) param_5 = auStack_20;
///   local_24 = *uVar4; FUN_802481b0(0,param_5,&local_24,param_3);
///   iVar2 = FUN_80167fa0(iVar1,param_4,param_5); if (iVar2 != 0) {
///   local_28 = *uVar4; uVar3 = FUN_802481b0(iVar1 + 0xc,auStack_20,&local_28,param_3);
///   *(undefined4 *)(iVar1 + 4) = uVar3; } FUN_802dcdc0(iVar2 != 0);`
///
/// # Arguments
/// - `this`: element entry being built.
/// - `key`: pointer to the element id word.
/// - `mode`: forwarded to both `0x802481B0` calls.
/// - `heap`: EGG heap, or `0` for [`W78_DEFAULT_HEAP`].
/// - `slot`: caller buffer of at least `0x20` bytes, or `0` for a local
///   scratch buffer.
///
/// # Returns
/// `true` when `fn_80167fa0` succeeded.
///
/// # Side effects
/// Runs `0x802481B0` twice, rewrites the size word of the buffer, allocates the
/// backing heap and stores the child at `+0x04`.
///
/// # Safety
/// `key` must be a readable word, `slot` (when non-zero) writable for `0x20`
/// bytes, and `heap` a live EGG heap.
///
/// # Confidence
/// medium - both calls, the default-heap substitution and the `+0x04` store are
/// literal; `FUN_802dcd74` is the register-save stub, so its two words are
/// spelled out as parameters.
pub unsafe fn fn_80165520(
    this: usize,
    key: usize,
    mode: u32,
    heap: usize,
    slot: usize,
) -> bool {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 0x20]);
    let out = if slot == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        slot
    };
    let mut value = crate::platform::GuestLocal::new( platform::read_u32(key));
    platform::call4(
        W78_FN_802481B0,
        0,
        out as u32,
        (value.as_mut_ptr() as *mut u32) as usize as u32,
        mode,
    );
    if fn_80167fa0(this, heap, out) == 0 {
        return false;
    }
    let mut value = crate::platform::GuestLocal::new( platform::read_u32(key));
    let child = platform::call4(
        W78_FN_802481B0,
        (this + 0x0C) as u32,
        out as u32,
        (value.as_mut_ptr() as *mut u32) as usize as u32,
        mode,
    );
    platform::write_u32(this + 0x04, child);
    true
}

/// Casts the child at `this + 4` to the update type and runs two virtual slots.
///
/// # Original
/// - Ghidra: `FUN_801655e0` at `0x801655e0` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801655e0_FUN_801655e0.c`
/// - observed: `piVar3 = *(int **)(param_2 + 4); ... local_28[0] = &DAT_802f86b4;
///   iVar2 = (**(code **)(*piVar3 + 8))(piVar3,local_28); ...
///   (**(code **)(*piVar3 + 0x48))(param_1,piVar3,param_3);
///   (**(code **)(*piVar3 + 0x3c))(piVar3,param_3,param_4);`
///
/// # Arguments
/// - `context`: forwarded as the first slot argument.
/// - `this`: element entry whose `+0x04` holds the raw child.
/// - `mode`: forwarded to both slots.
/// - `value`: forwarded to the second slot.
///
/// # Side effects
/// Two virtual calls on the cast child.
///
/// # Safety
/// The cast must succeed: the export dereferences the result without a null
/// guard, so a failed cast crashes exactly as the original does.
///
/// # Confidence
/// medium - the cast, both slots and their argument lists are literal; the slot
/// roles are unnamed in the image.
pub unsafe fn fn_801655e0(context: u32, this: usize, mode: u32, value: usize) {
    let child = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_UPDATE,
    );
    let vtable = platform::read_u32(child) as usize;
    platform::call3(
        platform::read_u32(vtable + 0x48) as usize,
        context,
        child as u32,
        mode,
    );
    platform::call3(
        platform::read_u32(vtable + 0x3C) as usize,
        child as u32,
        mode,
        value as u32,
    );
}

/// Casts the child at `holder + 4` to the holder type and forwards to
/// [`fn_801655e0`].
///
/// # Original
/// - Ghidra: `FUN_801656c0` at `0x801656c0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801656c0_FUN_801656c0.c`
/// - observed: `piVar3 = *(int **)(param_4 + 4); ... local_28[0] = &DAT_802f86c8;
///   ... FUN_801655e0(param_1,param_2,param_3,piVar3);`
///
/// # Arguments
/// - `context`: forwarded to [`fn_801655e0`].
/// - `this`: element entry forwarded to [`fn_801655e0`].
/// - `mode`: forwarded to [`fn_801655e0`].
/// - `holder`: object whose `+0x04` child is cast.
///
/// # Side effects
/// One cast plus the two virtual calls inside [`fn_801655e0`].
///
/// # Safety
/// `holder + 0x04` must be a null or live object; see [`fn_801655e0`] for the
/// unguarded cast result.
///
/// # Confidence
/// medium - the cast and the four-argument forward are literal; the type word
/// role is inferred.
pub unsafe fn fn_801656c0(context: u32, this: usize, mode: u32, holder: usize) {
    let value = w78_cast_to(
        platform::read_u32(holder + 0x04) as usize,
        W78_TYPE_HOLDER,
    );
    fn_801655e0(context, this, mode, value);
}

/// Class-A element teardown: releases every entry of the `+0x28` array and then
/// runs the shared teardown [`fn_80167f30`].
///
/// The array element count comes from the cast child's `+0x18` word and each
/// entry is one `0x38`-stride block whose vtable `+0x10` slot is called with
/// the entry itself as receiver.
///
/// # Original
/// - Ghidra: `FUN_80165ee0` at `0x80165ee0` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165ee0_FUN_80165ee0.c`
/// - observed: `if ((piVar4 != (int *)0x0) && (*(int *)(param_1 + 0x28) != 0)) {
///   iVar5 = piVar4[6]; ... for (iVar2 = 0; iVar2 < iVar5; iVar2 = iVar2 + 1) {
///   (**(code **)(*(int *)(iVar3 + *(int *)(param_1 + 0x28)) + 0x10))
///   (*(int *)(param_1 + 0x28) + iVar3); iVar3 = iVar3 + 0x38; }
///   *(undefined4 *)(param_1 + 0x28) = 0; } FUN_80167f30(param_1);`
///
/// # Arguments
/// - `this`: element object whose `+0x04` child owns the entry array and whose
///   `+0x28` word points at that array.
///
/// # Side effects
/// One virtual call per entry, clears `+0x28`, then runs [`fn_80167f30`].
///
/// # Safety
/// The `+0x04` cast must succeed and `+0x28` must point at `count` live
/// `0x38`-byte entries; after the call `+0x28` is zero.
///
/// # Confidence
/// medium - the loop, the stride and the virtual slot are literal; the type
/// word and the `+0x18` count field are inferred from the class layout.
pub unsafe fn fn_80165ee0(this: usize) {
    let array_object = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_CLASS_A,
    );
    let array = platform::read_u32(this + 0x28) as usize;
    if array_object != 0 && array != 0 {
        let count = platform::read_u32(array_object + 0x18);
        let mut index = 0u32;
        while index < count {
            let element = array + index as usize * 0x38;
            let vtable = platform::read_u32(element) as usize;
            platform::call1(platform::read_u32(vtable + 0x10) as usize, element as u32);
            index += 1;
        }
        platform::write_u32(this + 0x28, 0);
    }
    fn_80167f30(this);
}

/// Class-A per-entry release driven by the owner's `+0x40` virtual slot.
///
/// # Original
/// - Ghidra: `FUN_80166110` at `0x80166110` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166110_FUN_80166110.c`
/// - observed: `piVar3 = *(int **)(param_1 + 4); ... local_18[0] = &DAT_802f8588;
///   ... (**(code **)(*piVar3 + 0x40))(piVar3,param_2);
///   FUN_80165a80(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: element object whose `+0x04` is cast to the class-A type.
/// - `index`: entry index; the array base is `+0x28`, the stride `0x38`.
///
/// # Side effects
/// One virtual call on the owner and [`fn_80165a80`] on the entry.
///
/// # Safety
/// The cast must succeed, and the `+0x28` array must contain `index + 1`
/// entries.
///
/// # Confidence
/// medium - the slot, the stride and the release call are literal; the cast
/// target is inferred from the class-A destructor that installs the vtable.
pub unsafe fn fn_80166110(this: usize, index: u32) {
    let child = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_CLASS_A,
    );
    let vtable = platform::read_u32(child) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x40) as usize,
        child as u32,
        index,
    );
    let array = platform::read_u32(this + 0x28) as usize;
    fn_80165a80(array + index as usize * 0x38);
}

/// Class-B element teardown, twin of [`fn_80165ee0`].
///
/// # Original
/// - Ghidra: `FUN_80166ae0` at `0x80166ae0` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166ae0_FUN_80166ae0.c`
/// - observed: `local_18[0] = &DAT_802f85f0; ... iVar5 = piVar4[6]; ...
///   (**(code **)(*(int *)(iVar3 + *(int *)(param_1 + 0x28)) + 0x10))(...);
///   iVar3 = iVar3 + 0x38; } *(undefined4 *)(param_1 + 0x28) = 0;
///   } FUN_80167f30(param_1);`
///
/// # Arguments
/// - `this`: class-B element object.
///
/// # Side effects
/// One virtual call per entry, clears `+0x28`, runs [`fn_80167f30`].
///
/// # Safety
/// Same array requirements as [`fn_80165ee0`].
///
/// # Confidence
/// medium - identical to [`fn_80165ee0`] except for the type word.
pub unsafe fn fn_80166ae0(this: usize) {
    let array_object = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_CLASS_B,
    );
    let array = platform::read_u32(this + 0x28) as usize;
    if array_object != 0 && array != 0 {
        let count = platform::read_u32(array_object + 0x18);
        let mut index = 0u32;
        while index < count {
            let element = array + index as usize * 0x38;
            let vtable = platform::read_u32(element) as usize;
            platform::call1(platform::read_u32(vtable + 0x10) as usize, element as u32);
            index += 1;
        }
        platform::write_u32(this + 0x28, 0);
    }
    fn_80167f30(this);
}

/// Class-B per-entry release, twin of [`fn_80166110`].
///
/// # Original
/// - Ghidra: `FUN_80166d10` at `0x80166d10` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166d10_FUN_80166d10.c`
/// - observed: `local_18[0] = &DAT_802f85f0; ...
///   (**(code **)(*piVar3 + 0x40))(piVar3,param_2);
///   FUN_80166680(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: class-B element object.
/// - `index`: entry index.
///
/// # Side effects
/// One virtual call on the owner and [`fn_80166680`] on the entry.
///
/// # Safety
/// Same array requirements as [`fn_80166110`].
///
/// # Confidence
/// medium - identical to [`fn_80166110`] except for the type word and the
/// release callee.
pub unsafe fn fn_80166d10(this: usize, index: u32) {
    let child = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_CLASS_B,
    );
    let vtable = platform::read_u32(child) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x40) as usize,
        child as u32,
        index,
    );
    let array = platform::read_u32(this + 0x28) as usize;
    fn_80166680(array + index as usize * 0x38);
}

/// Class-C element teardown, twin of [`fn_80165ee0`].
///
/// # Original
/// - Ghidra: `FUN_80167710` at `0x80167710` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167710_FUN_80167710.c`
/// - observed: `local_18[0] = &DAT_802f8658; ... iVar5 = piVar4[6]; ...
///   (**(code **)(*(int *)(iVar3 + *(int *)(param_1 + 0x28)) + 0x10))(...);
///   iVar3 = iVar3 + 0x38; } *(undefined4 *)(param_1 + 0x28) = 0;
///   } FUN_80167f30(param_1);`
///
/// # Arguments
/// - `this`: class-C element object.
///
/// # Side effects
/// One virtual call per entry, clears `+0x28`, runs [`fn_80167f30`].
///
/// # Safety
/// Same array requirements as [`fn_80165ee0`].
///
/// # Confidence
/// medium - identical to [`fn_80165ee0`] except for the type word.
pub unsafe fn fn_80167710(this: usize) {
    let array_object = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_CLASS_C,
    );
    let array = platform::read_u32(this + 0x28) as usize;
    if array_object != 0 && array != 0 {
        let count = platform::read_u32(array_object + 0x18);
        let mut index = 0u32;
        while index < count {
            let element = array + index as usize * 0x38;
            let vtable = platform::read_u32(element) as usize;
            platform::call1(platform::read_u32(vtable + 0x10) as usize, element as u32);
            index += 1;
        }
        platform::write_u32(this + 0x28, 0);
    }
    fn_80167f30(this);
}

/// Class-C per-entry release, twin of [`fn_80166110`].
///
/// # Original
/// - Ghidra: `FUN_80167940` at `0x80167940` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167940_FUN_80167940.c`
/// - observed: `local_18[0] = &DAT_802f8658; ...
///   (**(code **)(*piVar3 + 0x40))(piVar3,param_2);
///   FUN_80167270(*(int *)(param_1 + 0x28) + param_2 * 0x38);`
///
/// # Arguments
/// - `this`: class-C element object.
/// - `index`: entry index.
///
/// # Side effects
/// One virtual call on the owner and [`fn_80167270`] on the entry.
///
/// # Safety
/// Same array requirements as [`fn_80166110`].
///
/// # Confidence
/// medium - identical to [`fn_80166110`] except for the type word and the
/// release callee.
pub unsafe fn fn_80167940(this: usize, index: u32) {
    let child = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_CLASS_C,
    );
    let vtable = platform::read_u32(child) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x40) as usize,
        child as u32,
        index,
    );
    let array = platform::read_u32(this + 0x28) as usize;
    fn_80167270(array + index as usize * 0x38);
}

/// Rounds the element buffer size and creates the `+0x08` backing heap object.
///
/// `0x8016E7D0` (the EGG alignment helper, already available as
/// [`crate::ui::lyt::fn_8016e7d0`]) is asked three times: the raw size, the
/// zero baseline, and the difference between them. The difference is rounded
/// and replaces the caller's size word, then the heap object is created by
/// `0x8016E630(delta, heap + 0x14, 0x803295A8, r13 - 0x52A0, 0)` and attached
/// at `+0x0C` through [`fn_8016a690`].
///
/// # Original
/// - Ghidra: `FUN_80167fa0` at `0x80167fa0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167fa0_FUN_80167fa0.c`
/// - observed: `iVar1 = FUN_8016e7d0(*param_3,0x20);
///   iVar2 = FUN_8016e7d0(0,0x20); iVar2 = (iVar1 + 0x1fU & 0xffffffe0) - iVar2;
///   iVar1 = FUN_8016e7d0(iVar2,0x20); *param_3 = iVar1 + 0x1fU & 0xffffffe0;
///   uVar3 = FUN_8016e630(iVar2,*(param_2 + 0x14),&DAT_803295a8,
///   *(r13 - 0x52a0),0); *(param_1 + 8) = uVar3;
///   FUN_8016a690(param_1 + 0xc,uVar3,0x20); return 1;`
///
/// # Arguments
/// - `this`: element object; `+0x08` receives the heap object and `+0x0C`
///   becomes its view.
/// - `heap`: EGG heap whose `+0x14` slot is the allocator.
/// - `size_ptr`: pointer to the size word, updated in place.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Writes the rounded size, allocates the heap object and attaches it.
///
/// # Safety
/// `heap` must be a live EGG heap and `size_ptr` must be a writable word.
///
/// # Confidence
/// medium - the three alignment calls and the allocator arguments are literal;
/// the `r13 - 0x52A0` frame-heap role is inferred from the surrounding layout
/// code.
pub unsafe fn fn_80167fa0(this: usize, heap: usize, size_ptr: usize) -> u32 {
    let raw = platform::read_u32(size_ptr) as i32;
    let sized = crate::ui::lyt::fn_8016e7d0(raw, 0x20);
    let baseline = crate::ui::lyt::fn_8016e7d0(0, 0x20);
    let delta = (sized.wrapping_add(0x1F) & !0x1F).wrapping_sub(baseline);
    let rounded = crate::ui::lyt::fn_8016e7d0(delta, 0x20);
    platform::write_u32(size_ptr, (rounded.wrapping_add(0x1F) & !0x1F) as u32);
    let object = platform::call5(
        W78_FN_8016E630,
        delta as u32,
        platform::read_u32(heap + 0x14),
        W78_HEAP_TAG as u32,
        platform::read_u32(W78_G_FRAME_HEAP),
        0,
    );
    platform::write_u32(this + 0x08, object);
    fn_8016a690(this + 0x0C, object as usize, 0x20);
    1
}

/// Files a child node into the sound-object owner by its kind.
///
/// A virtual `+0x0C` call classifies the node. Kind `5` is routed to the
/// `0x802F8FF0` owner, every other kind to the `0x802F8FD8` owner; a kind of
/// `0` also stores the node at the owner's `+0x08`. Both paths finish with the
/// same `+0x34` slot call carrying the node's `+0x04` word and the literal `6`.
///
/// # Original
/// - Ghidra: `FUN_80168280` at `0x80168280` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168280_FUN_80168280.c`
/// - observed: `iVar2 = (**(code **)(*param_2 + 0xc))(param_2);
///   if (iVar2 == 5) { ... local_14 = &DAT_802f8ff0; ...
///   (**(code **)(*piVar3 + 0x34))(piVar3,param_2[1],6); } else { ...
///   local_18 = &DAT_802f8fd8; ... if (iVar2 == 0) *(int **)(param_1 + 8) = param_2;
///   (**(code **)(*piVar3 + 0x34))(piVar3,param_2[1],6); }`
///
/// # Arguments
/// - `this`: owner object whose `+0x04` child is cast; `+0x08` receives
///   kind-`0` nodes.
/// - `node`: node being filed; `+0x04` is forwarded to the `+0x34` slot.
///
/// # Side effects
/// One classification call, one cast per branch, one `+0x34` slot call and the
/// optional `+0x08` store.
///
/// # Safety
/// Both the node vtable and the owner cast must be live; the export does not
/// guard the cast result.
///
/// # Confidence
/// medium - the two branches, the type words and the literal `6` are literal;
/// the node class meanings are inferred from the sound-object callers.
pub unsafe fn fn_80168280(this: usize, node: usize) {
    let node_vtable = platform::read_u32(node) as usize;
    let kind = platform::call1(
        platform::read_u32(node_vtable + 0x0C) as usize,
        node as u32,
    );
    let payload = platform::read_u32(node + 0x04);
    if kind == 5 {
        let owner = w78_cast_to(
            platform::read_u32(this + 0x04) as usize,
            W78_TYPE_OBJECT,
        );
        let vtable = platform::read_u32(owner) as usize;
        platform::call3(
            platform::read_u32(vtable + 0x34) as usize,
            owner as u32,
            payload,
            6,
        );
    } else {
        let owner = w78_cast_to(
            platform::read_u32(this + 0x04) as usize,
            W78_TYPE_ELEMENT,
        );
        if kind == 0 {
            platform::write_u32(this + 0x08, node as u32);
        }
        let vtable = platform::read_u32(owner) as usize;
        platform::call3(
            platform::read_u32(vtable + 0x34) as usize,
            owner as u32,
            payload,
            6,
        );
    }
}

/// Returns the `+0xE8` word of the owner's child cast to the generic type.
///
/// # Original
/// - Ghidra: `FUN_80168400` at `0x80168400` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168400_FUN_80168400.c`
/// - observed: `piVar3 = *(int **)(param_1 + 4); bVar1 = false;
///   if (piVar3 != (int *)0x0) { local_18[0] = &DAT_802f8fd8;
///   iVar2 = (**(code **)(*piVar3 + 8))(piVar3,local_18); ... }
///   if (!bVar1) { piVar3 = (int *)0x0; } return piVar3[0x3a];`
///
/// # Arguments
/// - `this`: owner object whose `+0x04` child is cast.
///
/// # Returns
/// The child's `+0xE8` word.
///
/// # Side effects
/// None.
///
/// # Safety
/// The cast must succeed. The export dereferences the result without a null
/// guard, so a failed cast reads address `0xE8`; that crash is preserved.
///
/// # Confidence
/// medium - the cast and the `+0xE8` read are literal; the field role is
/// inferred from the attach helpers that consume it.
pub unsafe fn fn_80168400(this: usize) -> u32 {
    let child = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_ELEMENT,
    );
    platform::read_u32(child + 0xE8)
}

/// Clears the `+0x08` back-pointer for a zero value and forwards it to the
/// generic element's `+0x3C` slot.
///
/// # Original
/// - Ghidra: `FUN_80168520` at `0x80168520` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168520_FUN_80168520.c`
/// - observed: `piVar3 = *(int **)(param_1 + 4); ... local_18[0] = &DAT_802f8fd8;
///   ... if (param_2 == 0) { *(undefined4 *)(param_1 + 8) = 0; }
///   (**(code **)(*piVar3 + 0x3c))(piVar3,param_2);`
///
/// # Arguments
/// - `this`: owner object whose `+0x04` child is cast.
/// - `value`: value forwarded to the slot; `0` also clears `+0x08`.
///
/// # Side effects
/// Optional `+0x08` clear plus one virtual call.
///
/// # Safety
/// The cast must succeed; the slot is called on the raw result.
///
/// # Confidence
/// medium - the guard, the clear and the slot are literal; the type word and
/// the field role are inferred.
pub unsafe fn fn_80168520(this: usize, value: u32) {
    let child = w78_cast_to(
        platform::read_u32(this + 0x04) as usize,
        W78_TYPE_ELEMENT,
    );
    if value == 0 {
        platform::write_u32(this + 0x08, 0);
    }
    let vtable = platform::read_u32(child) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x3C) as usize,
        child as u32,
        value,
    );
}

/// Empties and destroys the `0x80377C3C` intrusive list.
///
/// The list is `{head link 0x80377C3C, tail link 0x80377C40, count 0x80377C44}`
/// with a static sentinel node at `0x80377C3C` itself. The walk starts at the
/// tail link (or at the sentinel when the tail link is the sentinel), unlinks
/// every node in both directions, clears its two links, decrements the count and
/// finally destroys the owner handle from `r13 - 0x524C`.
///
/// # Original
/// - Ghidra: `FUN_8016c4a0` at `0x8016c4a0` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c4a0_FUN_8016c4a0.c`
/// - observed: `FUN_802d9500(*(r13 + -0x46b0)); if (*(int *)(r13 + -0x524c) != 0) {
///   puVar3 = &DAT_80377c3c; if (DAT_80377c40 != &DAT_80377c3c)
///   puVar3 = DAT_80377c40; while (puVar3 != &DAT_80377c3c) { ...
///   DAT_80377c44 = DAT_80377c44 + -1; puVar3 = puVar1; }
///   FUN_802b9730(*(r13 + -0x524c)); }`
///
/// # Side effects
/// Flushes the layout queue, unlinks every node, zeroes the two list links and
/// the count, then destroys the list owner.
///
/// # Safety
/// Every node in the list must be a live two-word link; after this call the
/// list is empty and the owner is gone.
///
/// # Confidence
/// medium - the unlink cases, the count and the two calls are literal; the
/// head/tail roles of `0x80377C3C`/`0x80377C40` are inferred from the
/// `ut::List` layout used by the wave-40 sound-bank tables.
pub unsafe fn fn_8016c4a0() {
    platform::call1(
        W78_FN_802D9500,
        platform::read_u32(SDA_BASE - 0x46B0),
    );
    if platform::read_u32(W78_G_ACTIVE_LIST) == 0 {
        return;
    }
    let sentinel = W78_LIST_SENTINEL;
    let tail = platform::read_u32(W78_LIST_TAIL) as usize;
    let mut node = if tail != sentinel { tail } else { sentinel };
    while node != sentinel {
        let raw_next = platform::read_u32(node + 4) as usize;
        let next = if raw_next == 0 { sentinel } else { raw_next };
        let previous = platform::read_u32(node) as usize;
        if previous == sentinel {
            platform::write_u32(W78_LIST_TAIL, raw_next as u32);
        } else {
            platform::write_u32(previous + 4, raw_next as u32);
        }
        if raw_next == sentinel {
            platform::write_u32(W78_LIST_SENTINEL, previous as u32);
        } else {
            platform::write_u32(raw_next, previous as u32);
        }
        platform::write_u32(node, 0);
        platform::write_u32(node + 4, 0);
        platform::write_u32(
            W78_LIST_COUNT,
            platform::read_u32(W78_LIST_COUNT).wrapping_sub(1),
        );
        node = next;
    }
    platform::call1(W78_FN_802B9730, platform::read_u32(W78_G_ACTIVE_LIST));
}

/// Forwards three words to `0x802D9630` behind the `r13 - 0x46B0` handle, then
/// flushes the layout command buffer with `0x801C9D60(0)`.
///
/// # Original
/// - Ghidra: `FUN_8016c850` at `0x8016c850` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c850_FUN_8016c850.c`
/// - observed: `FUN_802d9630(*(undefined4 *)(unaff_r13 + -0x46b0),param_1,param_2,
///   param_3); FUN_801c9d60(0);`
///
/// # Arguments
/// - `a`, `b`, `c`: words handed to `0x802D9630`.
///
/// # Side effects
/// One queue submission and one command-buffer flush.
///
/// # Safety
/// The `r13 - 0x46B0` handle must be live; the three words are passed through
/// unchanged.
///
/// # Confidence
/// high - the SDA load, the four-argument call and the literal zero are literal.
pub unsafe fn fn_8016c850(a: u32, b: u32, c: u32) {
    platform::call4(
        W78_FN_802D9630,
        platform::read_u32(SDA_BASE - 0x46B0),
        a,
        b,
        c,
    );
    platform::call1(W78_FN_801C9D60, 0);
}

/// Re-submits a layout request whose descriptor at `+0x74` is still registered.
///
/// When `0x802967B0` reports the descriptor at `+0x74`, a `0x38`-byte packet is
/// built by `0x8016C9E0(arg2, arg3, arg4, packet)` and handed to the object's
/// `+0x8C` slot; the `+0x94` slot then commits it.
///
/// # Original
/// - Ghidra: `FUN_8016cea0` at `0x8016cea0` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016cea0_FUN_8016cea0.c`
/// - observed: `iVar1 = FUN_802967b0(param_1 + 0x1d); if (iVar1 != 0) {
///   FUN_8016c9e0(param_2,param_3,param_4,auStack_48);
///   (**(code **)(*param_1 + 0x8c))(param_1,auStack_48);
///   (**(code **)(*param_1 + 0x94))(param_1); } return iVar1 != 0;`
///
/// # Arguments
/// - `this`: layout object; the descriptor register lives at `+0x74`
///   (`param_1 + 0x1d` in the `int *` export).
/// - `arg2`, `arg3`, `arg4`: forwarded to the packet builder.
///
/// # Returns
/// `true` when the descriptor was registered and the request was submitted.
///
/// # Side effects
/// Builds a `0x38`-byte stack packet and runs two virtual slots.
///
/// # Safety
/// `this` must be a live layout object with a valid vtable; the descriptor
/// must stay registered across the call, as in [`fn_8016cfd0`].
///
/// # Confidence
/// high - the predicate, the packet call, the two slots and the returned flag
/// are literal, and `+0x74` matches the already-translated [`fn_8016cfd0`].
pub unsafe fn fn_8016cea0(this: usize, arg2: u32, arg3: u32, arg4: u32) -> bool {
    if platform::call1(W64_FN_802967B0, (this + 0x74) as u32) == 0 {
        return false;
    }
    let mut packet = crate::platform::GuestLocal::new( [0u8; 0x38]);
    platform::call4(
        W78_FN_8016C9E0,
        arg2,
        arg3,
        arg4,
        packet.as_mut_ptr() as usize as u32,
    );
    let vtable = platform::read_u32(this) as usize;
    platform::call2(
        platform::read_u32(vtable + 0x8C) as usize,
        this as u32,
        packet.as_mut_ptr() as usize as u32,
    );
    platform::call1(platform::read_u32(vtable + 0x94) as usize, this as u32);
    true
}

/// Formats one message id into a 60-byte packet and submits it.
///
/// The `u16` behind `id` is copied to a stack word, turned into the packet by
/// `0x8016ED10`, and submitted through `0x801C0540` with `this` as both the
/// first and the third argument.
///
/// # Original
/// - Ghidra: `FUN_8016edd0` at `0x8016edd0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016edd0_FUN_8016edd0.c`
/// - observed: `if (*param_2 != 0) { local_48[0] = *param_2;
///   FUN_8016ed10(auStack_40,local_48); FUN_801c0540(param_1,auStack_40,param_1); }`
///
/// # Arguments
/// - `this`: destination handle, passed twice.
/// - `id`: pointer to the `u16` message id; a zero id does nothing.
///
/// # Side effects
/// Builds a `0x3C`-byte stack packet and submits it.
///
/// # Safety
/// `id` must be a readable half-word and the packet builder must accept a
/// 60-byte destination.
///
/// # Confidence
/// medium - the guard, the two calls and the buffer sizes are literal; the
/// packet format itself lives in the untranslated `0x8016ED10`.
pub unsafe fn fn_8016edd0(this: u32, id: usize) {
    let raw = platform::read_u16(id);
    if raw == 0 {
        return;
    }
    let mut value = crate::platform::GuestLocal::new( raw);
    let mut packet = crate::platform::GuestLocal::new( [0u8; 0x3C]);
    platform::call2(
        W78_FN_8016ED10,
        packet.as_mut_ptr() as usize as u32,
        (value.as_mut_ptr() as *mut u16) as usize as u32,
    );
    platform::call3(
        W78_FN_801C0540,
        this,
        packet.as_mut_ptr() as usize as u32,
        this,
    );
}

/// Same submit path as [`fn_8016edd0`] with the second packet formatter
/// (`0x8016EE30`).
///
/// # Original
/// - Ghidra: `FUN_8016eef0` at `0x8016eef0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eef0_FUN_8016eef0.c`
/// - observed: `if (*param_2 != 0) { local_48[0] = *param_2;
///   FUN_8016ee30(auStack_40,local_48); FUN_801c0540(param_1,auStack_40,param_1); }`
///
/// # Arguments
/// - `this`: destination handle, passed twice.
/// - `id`: pointer to the `u16` message id; a zero id does nothing.
///
/// # Side effects
/// Builds a `0x3C`-byte stack packet and submits it.
///
/// # Safety
/// Same as [`fn_8016edd0`].
///
/// # Confidence
/// medium - identical to [`fn_8016edd0`] except for the formatter address.
pub unsafe fn fn_8016eef0(this: u32, id: usize) {
    let raw = platform::read_u16(id);
    if raw == 0 {
        return;
    }
    let mut value = crate::platform::GuestLocal::new( raw);
    let mut packet = crate::platform::GuestLocal::new( [0u8; 0x3C]);
    platform::call2(
        W78_FN_8016EE30,
        packet.as_mut_ptr() as usize as u32,
        (value.as_mut_ptr() as *mut u16) as usize as u32,
    );
    platform::call3(
        W78_FN_801C0540,
        this,
        packet.as_mut_ptr() as usize as u32,
        this,
    );
}

// ===========================================================================
// Random steps, interpolators and layout leaves (wave 89)
//
// Everything below was still untranslated in the `0x80160000` page when this
// pass started. Three rules keep the group honest:
//
// * The `r2` float constants are *resolved*, not guessed: `r2` is
//   [`SDA2_BASE`] = `0x80433080`, and every `r2 - 0xNNNN` word below was read
//   back from `main.elf` at `SDA2_BASE - 0xNNNN`. The comment on each constant
//   states the address and the `f32` value found there.
// * `FUN_802dcd5c/64/68/6c/70/74` are Metrowerks **prologue fragments** (they
//   only spill `r21..r31` and fall through). The `undefined8` the decompiler
//   shows as their "return" is therefore the enclosing function's own incoming
//   `r3:r4`: `hi` is the first parameter and `lo` the second. Those two are
//   spelled out below. The `0x802DCDB0..BC` epilogue fragments are not emitted
//   as calls either.
// * Where the decompiler drops the receiver of an indirect call, the parameter
//   is added back only when the same object is passed explicitly by the
//   neighbouring slots; that is marked `// inferred:`.
// ===========================================================================

/// `1664525` (`0x0019660D`) - multiplier of the shared 32-bit LCG. The value is
/// literal in `main.elf` at `0x80429448`, which is `r13 - 0x6238`.
const W89_LCG_MUL: u32 = 0x0019_660D;

/// `1013904223` (`0x3C6EF35F`) - increment of the same LCG, literal at
/// `0x8042944C` (`r13 - 0x6234`).
const W89_LCG_ADD: u32 = 0x3C6E_F35F;

/// `r2 - 0x5458` (`0x8042DC28`) - `1.0f`, the mantissa bias subtracted by
/// [`fn_80161490`].
const W89_F32_MANTISSA_BIAS: usize = SDA2_BASE - 0x5458;

/// `r2 - 0x5454` (`0x8042DC2C`) - `1.5f`, the mantissa bias subtracted by
/// [`fn_801614e0`].
const W89_F32_HALF_BIAS: usize = SDA2_BASE - 0x5454;

/// `r2 - 0x53D8` (`0x8042DCA8`) - `0.0f`, the "at rest" value of the tween
/// helpers.
const W89_F32_TWEEN_ZERO: usize = SDA2_BASE - 0x53D8;

/// `r2 - 0x53D4` (`0x8042DCAC`) - `1.0f`, the "complete" value of the tween
/// helpers.
const W89_F32_TWEEN_ONE: usize = SDA2_BASE - 0x53D4;

/// `r2 - 0x5438` (`0x8042DC48`) - `0.7307692f` (19/26), the default gain
/// [`fn_801638e0`] parks at `+0x74`.
const W89_F32_GAIN_19_26: usize = SDA2_BASE - 0x5438;

/// `r2 - 0x544C` (`0x8042DC34`) - `1.0f`, the upper end of a clamped tween.
const W89_F32_ONE: usize = SDA2_BASE - 0x544C;

/// `r2 - 0x5450` (`0x8042DC30`) - `0.0f`, the lower end of a clamped tween.
const W89_F32_ZERO: usize = SDA2_BASE - 0x5450;

/// `r2 - 0x5378` (`0x8042DD08`) - `0.0f`, the idle word cleared at `+0x128`.
const W89_F32_RECORD_ZERO: usize = SDA2_BASE - 0x5378;

/// `0x80329410` - vtable installed at `+0x60` by [`fn_80161ad0`].
const W89_VT_ACTOR: usize = 0x8032_9410;

/// `0x803294C0` - first vtable installed by [`fn_801638e0`] at `+0x08`.
const W89_VT_TWEEN_A: usize = 0x8032_94C0;

/// `0x803294D8` - vtable that replaces `0x803294C0` at `+0x10`.
const W89_VT_TWEEN_B: usize = 0x8032_94D8;

/// `0x80329700` - vtable installed at `+0x00` by [`fn_8016ab90`].
const W89_VT_OBJECT: usize = 0x8032_9700;

/// `0x80329A14` - vtable of the request object built by [`fn_8016cd40`].
const W89_VT_REQUEST: usize = 0x8032_9A14;

/// `0x80329A20` - vtable of the stack object built by [`fn_8016c890`].
const W89_VT_STACK_REQUEST: usize = 0x8032_9A20;

/// `0x80377A10` - root of the actor tree the constructor inserts into.
const W89_ACTOR_ROOT: usize = 0x8037_7A10;

/// `0x80377A70` - base of the group table indexed by `FUN_80162CF0`.
const W89_GROUP_TABLE: usize = 0x8037_7A70;

/// `0x802F9018` - type-info word of the deferred-load object.
const W89_TYPE_DEFERRED: usize = 0x802F_9018;

/// `0x80161020` - link a node into the group table.
const W89_FN_80161020: usize = 0x8016_1020;

/// `0x802E1AB4` - string compare used by [`fn_80163ff0`]'s name scan.
const W89_FN_802E1AB4: usize = 0x802E_1AB4;

/// `0x8016D120` - retire one finished list entry.
const W89_FN_8016D120: usize = 0x8016_D120;

/// `0x80162590` - resolve the parent node of a freshly built actor.
const W89_FN_80162590: usize = 0x8016_2590;

/// `0x80162CF0` - group-table index of a list node.
const W89_FN_80162CF0: usize = 0x8016_2CF0;

/// `0x8016CAB0` - dispatch through a stack-allocated request object.
const W89_FN_8016CAB0: usize = 0x8016_CAB0;

/// `0x801C1450` - build the 16-byte message context of [`fn_80161060`].
const W89_FN_801C1450: usize = 0x801C_1450;

/// `0x801C13F0` - fold that context and two words into the final message.
const W89_FN_801C13F0: usize = 0x801C_13F0;

/// `0x801C1420` - build the 24-byte context of [`fn_801610b0`].
const W89_FN_801C1420: usize = 0x801C_1420;

/// `0x80168B90` - "tween finished" notification called by [`fn_80168c10`].
const W89_FN_80168B90: usize = 0x8016_8B90;

/// `0x801DD1D0` - read the shipped display-mode byte.
const W89_FN_801DD1D0: usize = 0x801D_D1D0;

/// `0x8016DC00` - base constructor of the `0x80329700` class.
const W89_FN_8016DC00: usize = 0x8016_DC00;

/// `0x8023B860` - element `i` of a child array.
const W89_FN_8023B860: usize = 0x8023_B860;

/// `0x8023B8C0` - element count of a child array.
const W89_FN_8023B8C0: usize = 0x8023_B8C0;

/// `0x8023CA80` - release the temporary built by `0x8023D3C0`/`0x8023D520`.
const W89_FN_8023CA80: usize = 0x8023_CA80;

/// `0x8023D3C0` - submit one child through the first draw path.
const W89_FN_8023D3C0: usize = 0x8023_D3C0;

/// `0x8023D520` - submit one child through the second draw path.
const W89_FN_8023D520: usize = 0x8023_D520;

/// `0x8025C560` - pack a draw entry into one word.
const W89_FN_8025C560: usize = 0x8025_C560;

/// `0x8025C660` - build one draw entry for a child.
const W89_FN_8025C660: usize = 0x8025_C660;

/// `0x8025EFB0` - construct the deferred-load reader.
const W89_FN_8025EFB0: usize = 0x8025_EFB0;

/// `0x80257640` / `0x80257670` - the two parameter writes of that reader.
const W89_FN_80257640: usize = 0x8025_7640;
const W89_FN_80257670: usize = 0x8025_7670;

/// `0x802ACB40` - construct the `+0x10` sub-object of [`fn_801638e0`].
const W89_FN_802ACB40: usize = 0x802A_CB40;

/// `0x802B4D30` - construct the `+0x30` sub-object of [`fn_801638e0`].
const W89_FN_802B4D30: usize = 0x802B_4D30;

/// `0x802B9280` - free a block from the default heap.
const W89_FN_802B9280: usize = 0x802B_9280;

/// `0x802D7AA0` / `0x802D7B20` - construct/destroy the stack request object.
const W89_FN_802D7AA0: usize = 0x802D_7AA0;
const W89_FN_802D7B20: usize = 0x802D_7B20;

/// `0x802D8DE0` - store two words into the pending request header.
const W89_FN_802D8DE0: usize = 0x802D_8DE0;

/// `0x802D8E70` - submit a five-word request block.
const W89_FN_802D8E70: usize = 0x802D_8E70;

/// `0x802DA310` - wrap an allocation in the request handle type.
const W89_FN_802DA310: usize = 0x802D_A310;

/// `0x802DA590` - flush the pending request queue.
const W89_FN_802DA590: usize = 0x802D_A590;

/// `0x80296750` / `0x80296760` - init/re-key the request key object.
const W89_FN_80296750: usize = 0x8029_6750;
const W89_FN_80296760: usize = 0x8029_6760;

/// `0x80164FA0` - completion callback handed to `0x8025EFB0`. It is an interior
/// label of `0x80164FD0`, not a function of its own, so it stays an address.
const W89_CB_80164FA0: usize = 0x8016_4FA0;

/// `SDA_BASE - 0x6230` - actor serial counter (initialised to `1` in the image).
const W89_G_SERIAL: usize = SDA_BASE - 0x6230;

/// Advances the shared 32-bit LCG and returns the new state.
///
/// The original folds the 64-bit product back into 32 bits by adding the high
/// half and the carry of the low half, which is exactly the low word of
/// `state * 1664525 + 1013904223`.
///
/// # Safety
/// `state` must point at a writable `u32`.
unsafe fn w89_lcg_step(state: usize) -> u32 {
    let next = (platform::read_u32(state) as u64 * W89_LCG_MUL as u64
        + W89_LCG_ADD as u64) as u32;
    platform::write_u32(state, next);
    next
}

/// Calls a PowerPC function with no arguments and returns its `r3`.
///
/// `platform` only offers [`platform::call0`], which discards `r3`; the display
/// mode query at `0x801DD1D0` has no arguments but a meaningful result, so the
/// same small wrapper the other lanes use is repeated here.
///
/// # Safety
/// `addr` must point to a function with this ABI.
unsafe fn w89_call0_ret(addr: usize) -> u32 {
    let function = crate::platform::guest_fn!(addr, fn() -> u32);
    function()
}

/// Composes one message in two steps.
///
/// `0x801C1450` fills a 16-byte context from `b`; `0x801C13F0` then folds that
/// context together with `a` and `c` into the final message.
///
/// # Original
/// - Ghidra: `FUN_80161060` at `0x80161060` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161060_FUN_80161060.c`
/// - observed: `FUN_801c1450(param_2,auStack_18);
///   FUN_801c13f0(auStack_18,param_1,param_3);`
///
/// # Arguments
/// - `a`: first message word, handed to `0x801C13F0` as its second argument.
/// - `b`: source word for the 16-byte context.
/// - `c`: third message word, handed to `0x801C13F0` as its third argument.
///
/// # Side effects
/// Builds a 16-byte stack context; `0x801C13F0` may then publish a message.
///
/// # Safety
/// Both callees must accept a host stack pointer, and `c` must stay valid for
/// the duration of the second call.
///
/// # Confidence
/// medium - both calls and the buffer size are literal, but the three words are
/// untyped in the export, so only their routing is claimed.
pub unsafe fn fn_80161060(a: u32, b: u32, c: u32) {
    let mut context = crate::platform::GuestLocal::new( [0u8; 0x10]);
    platform::call2(W89_FN_801C1450, b, context.as_mut_ptr() as usize as u32);
    platform::call3(
        W89_FN_801C13F0,
        context.as_mut_ptr() as usize as u32,
        a,
        c,
    );
}

/// Builds a 24-byte context, then composes a message from it.
///
/// `tag` is a two-word handle (`r3:r4`) that the original forwards to
/// [`fn_80161060`] unchanged, which is why it is one `u64` here.
///
/// # Original
/// - Ghidra: `FUN_801610b0` at `0x801610b0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801610b0_FUN_801610b0.c`
/// - observed: `FUN_801c1420(param_3,param_2,auStack_28);
///   FUN_80161060(param_1,param_2,auStack_28,param_4);`
///
/// # Arguments
/// - `tag`: the two-word handle, split back into `r3:r4` for the nested call.
/// - `a`: word forwarded as `fn_80161060`'s third argument.
/// - `b`: first source word for the 24-byte context.
/// - `_c`: fourth word of the export; the three-argument callee ignores it.
///
/// # Side effects
/// Builds a 24-byte stack context, then runs [`fn_80161060`].
///
/// # Safety
/// `a`/`b` must be acceptable to `0x801C1420` and `0x801C13F0`.
///
/// # Confidence
/// medium - the routing of `r3:r4` follows the wave-78 prologue-fragment rule;
/// the fourth word is provably unused by the nested three-argument call.
pub unsafe fn fn_801610b0(tag: u64, a: u32, b: u32, _c: u32) {
    let mut context = crate::platform::GuestLocal::new( [0u8; 0x18]);
    platform::call3(
        W89_FN_801C1420,
        b,
        a,
        context.as_mut_ptr() as usize as u32,
    );
    fn_80161060(tag as u32, (tag >> 32) as u32, a);
}

/// Advances the shared LCG and returns the high half of `state * range`.
///
/// This is the integer "scale a random state into `range`" helper: the new
/// state is stored back through `state` before the multiply-high.
///
/// # Original
/// - Ghidra: `FUN_80161460` at `0x80161460` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161460_FUN_80161460.c`
/// - observed: `uVar1 = *param_1 * *(r13 + -0x6238); uVar1 = uVar1 +
///   *(r13 + -0x6234) + (high half) + CARRY4(...); *param_1 = uVar1;
///   return (int)((ulonglong)uVar1 * (ulonglong)param_2 >> 0x20);`
///
/// # Arguments
/// - `state`: pointer to the 32-bit LCG state; updated in place.
/// - `range`: multiplier whose high half is returned.
///
/// # Returns
/// `high32(new_state * range)`.
///
/// # Side effects
/// Writes the new state through `state`.
///
/// # Safety
/// `state` must point at a writable `u32`.
///
/// # Confidence
/// high - both LCG constants are literal in `main.elf`, and the carry folding
/// is exactly a 32-bit truncation of the 64-bit sum.
pub unsafe fn fn_80161460(state: usize, range: u32) -> u32 {
    let next = w89_lcg_step(state);
    ((next as u64 * range as u64) >> 32) as u32
}

/// Advances the shared LCG and scales the new state into `[0, 1)`.
///
/// The original builds a float in `[1, 2)` by forcing the exponent bits
/// (`state >> 9 | 0x3F800000`) and then subtracts `1.0f` from `r2 - 0x5458`.
///
/// # Original
/// - Ghidra: `FUN_80161490` at `0x80161490` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161490_FUN_80161490.c`
/// - observed: `return (double)((float)(uVar2 >> 9 | 0x3f800000) - fVar1);`
///
/// # Arguments
/// - `state`: pointer to the 32-bit LCG state; updated in place.
///
/// # Returns
/// A value in `[0, 1)`. The C prototype says `double` because the soft-float
/// ABI widens `f1`; the value itself is an `f32`.
///
/// # Side effects
/// Writes the new state through `state`.
///
/// # Safety
/// `state` must point at a writable `u32`.
///
/// # Confidence
/// high - the mantissa trick and the `1.0f` constant at `0x8042DC28` are
/// literal.
pub unsafe fn fn_80161490(state: usize) -> f32 {
    let next = w89_lcg_step(state);
    f32::from_bits((next >> 9) | 0x3F80_0000) - platform::read_f32(W89_F32_MANTISSA_BIAS)
}

/// Advances the shared LCG and scales the new state into `[-0.5, 0.5)`.
///
/// Same mantissa trick as [`fn_80161490`], but the `r2 - 0x5454` bias is
/// `1.5f`, so the result is centred on zero.
///
/// # Original
/// - Ghidra: `FUN_801614e0` at `0x801614e0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801614e0_FUN_801614e0.c`
/// - observed: `return (double)((float)(uVar2 >> 9 | 0x3f800000) - fVar1);`
///
/// # Arguments
/// - `state`: pointer to the 32-bit LCG state; updated in place.
///
/// # Returns
/// A value in `[-0.5, 0.5)`.
///
/// # Side effects
/// Writes the new state through `state`.
///
/// # Safety
/// `state` must point at a writable `u32`.
///
/// # Confidence
/// high - identical to [`fn_80161490`] except for the `1.5f` bias at
/// `0x8042DC2C`.
pub unsafe fn fn_801614e0(state: usize) -> f32 {
    let next = w89_lcg_step(state);
    f32::from_bits((next >> 9) | 0x3F80_0000) - platform::read_f32(W89_F32_HALF_BIAS)
}

/// Constructs one actor node and links it into the actor tree.
///
/// The node carries three intrusive list headers at `+0x10`, `+0x20` and
/// `+0x40` (each `{self, next, prev, self}`-shaped), a serial word copied from
/// the global counter, and the sort key used by the group table. The counter is
/// bumped after the copy, and a wrap to `-1` traps in place.
///
/// # Original
/// - Ghidra: `FUN_80161ad0` at `0x80161ad0` (368 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161ad0_FUN_80161ad0.c`
/// - observed: `param_1[0x18] = &DAT_80329410; *param_1 = *(r13 + -0x6230);
///   ... FUN_80161530(param_1 + 4); ... FUN_80161580(&DAT_80377a10,
///   param_1 + 4, *(r13 + -0x52dc)); iVar2 = FUN_80162cf0(param_1 + 4); ...`
///
/// # Arguments
/// - `this`: destination node, at least `0x64` bytes.
///
/// # Returns
/// `this`, so the constructor composes like a C++ `__ct`.
///
/// # Side effects
/// Writes ~30 fields, bumps `r13 - 0x6230`, inserts the node into the
/// `0x80377A10` tree and the `0x80377A70` group table, and ORs two bits into
/// `+0x0F` when the resolved parent carries them.
///
/// # Safety
/// `this` must be a live, exclusively owned block; the tree roots must already
/// be initialised.
///
/// # Confidence
/// medium - every store and call is literal; the field *meanings* are inferred
/// from the twin constructors at `0x80161AD0`'s call sites, and the argument
/// list of the `0x8016DC00`-style base constructor is elided in the export.
pub unsafe fn fn_80161ad0(this: usize) -> usize {
    platform::write_u32(this + 0x60, W89_VT_ACTOR as u32);
    let serial = platform::read_u32(W89_G_SERIAL);
    platform::write_u32(this, serial);
    platform::write_u32(this + 0x04, platform::read_u32(SDA_BASE - 0x52E4));
    platform::write_u16(this + 0x08, platform::read_u16(SDA_BASE - 0x52E8));
    platform::write_u8(this + 0x0E, platform::read_u8(SDA_BASE - 0x52E0));
    fn_80161530(this + 0x10);
    platform::write_u32(this + 0x20, this as u32);
    platform::write_u32(this + 0x24, 0);
    platform::write_u32(this + 0x28, 0);
    platform::write_u32(this + 0x2C, this as u32);
    platform::write_u16(this + 0x30, 0);
    platform::write_u16(this + 0x32, 0);
    platform::write_u32(this + 0x34, 0);
    platform::write_u32(this + 0x38, 0);
    platform::write_u32(this + 0x3C, this as u32);
    platform::write_u16(this + 0x40, 0);
    platform::write_u16(this + 0x42, 0);
    platform::write_u32(this + 0x44, 0);
    platform::write_u32(this + 0x48, 0);
    platform::write_u32(this + 0x4C, this as u32);
    // observed: `+0x50` is skipped by the original; only `+0x54`/`+0x58` clear.
    platform::write_u32(this + 0x54, 0);
    platform::write_u32(this + 0x58, 0);
    let next_serial = platform::read_u32(W89_G_SERIAL).wrapping_add(1);
    platform::write_u32(W89_G_SERIAL, next_serial);
    if next_serial == 0xFFFF_FFFF {
        // observed: the original spins on an empty `do { } while (true)` when
        // the serial wraps; the trap is preserved rather than dropped.
        loop {}
    }
    fn_80161580(
        W89_ACTOR_ROOT,
        this + 0x10,
        platform::read_u32(SDA_BASE - 0x52DC) as usize,
    );
    let slot = platform::call1(W89_FN_80162CF0, (this + 0x10) as u32) as usize;
    fn_80161020(W89_GROUP_TABLE + slot * 8, this + 0x44);
    let table = platform::read_u32(SDA_BASE - 0x52C8) as usize;
    let kind = platform::read_u16(this + 0x08) as usize;
    let descriptor = platform::read_u32(table + kind * 4) as usize;
    if descriptor != 0 {
        let low = platform::read_u16(descriptor + 4);
        let high = platform::read_u16(descriptor + 6);
        platform::write_u16(this + 0x30, low);
        platform::write_u16(this + 0x32, low);
        platform::write_u16(this + 0x40, high);
        platform::write_u16(this + 0x42, high);
    }
    let parent = platform::call1(W89_FN_80162590, this as u32) as usize;
    if parent != 0 {
        let flags = platform::read_u8(parent + 0x0F);
        if flags & 0x03 != 0 {
            platform::write_u8(this + 0x0F, platform::read_u8(this + 0x0F) | 0x02);
        }
        if flags & 0x0C != 0 {
            platform::write_u8(this + 0x0F, platform::read_u8(this + 0x0F) | 0x08);
        }
    }
    this
}

/// Advances one looping or clamping tween by its own step.
///
/// The previous value is published at `+0x0C` first. Bit `2` of the mode byte
/// at `+0x14` selects the "sawtooth" form (subtract the step until the value
/// falls below it), bit `1` selects clamping at `1.0`/`0.0` instead of
/// wrapping through `+0x04`.
///
/// # Original
/// - Ghidra: `FUN_801636c0` at `0x801636c0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801636c0_FUN_801636c0.c`
/// - observed: `*(float *)(param_1 + 0xc) = fVar2; if ((bVar1 & 2) == 0) {
///   fVar2 = fVar2 + *(float *)(param_1 + 0x10); ... } else { ... }
///   *(float *)(param_1 + 8) = fVar2;`
///
/// # Arguments
/// - `this`: the tween; `+0x04` is its range, `+0x08` the value, `+0x0C` the
///   previous value, `+0x10` the step and `+0x14` the mode byte.
///
/// # Side effects
/// Writes `+0x0C` and `+0x08`.
///
/// # Safety
/// `this..this+0x18` must be a live tween.
///
/// # Confidence
/// high - every arithmetic step is literal and the two `r2` constants resolve
/// to `1.0f` (`0x8042DC34`) and `0.0f` (`0x8042DC30`).
pub unsafe fn fn_801636c0(this: usize) {
    let mode = platform::read_u8(this + 0x14);
    let mut value = platform::read_f32(this + 0x08);
    platform::write_f32(this + 0x0C, value);
    if mode & 0x02 == 0 {
        value += platform::read_f32(this + 0x10);
        if mode & 0x01 == 0 {
            if platform::read_f32(this + 0x04) <= value {
                value -= platform::read_f32(this + 0x04);
            }
        } else {
            let ceiling = platform::read_f32(this + 0x04) - platform::read_f32(W89_F32_ONE);
            if ceiling < value {
                value = ceiling;
            }
        }
    } else {
        let step = platform::read_f32(this + 0x10);
        if value < step {
            if mode & 0x01 == 0 {
                value += platform::read_f32(this + 0x04) - step;
            } else {
                value = platform::read_f32(W89_F32_ZERO);
            }
        } else {
            value -= step;
        }
    }
    platform::write_f32(this + 0x08, value);
}

/// Reports whether a tween has reached the end of its range.
///
/// Mode `1` is complete once `value >= range - 1`; mode `3` is complete once
/// `value <= 0`; any other mode is never complete.
///
/// # Original
/// - Ghidra: `FUN_801637f0` at `0x801637f0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801637f0_FUN_801637f0.c`
/// - observed: `if (*(char *)(param_1 + 0x14) == '\x01') return
///   *(float *)(param_1 + 4) - fConst <= *(float *)(param_1 + 8); if (... !=
///   '\x03') return false; return *(float *)(param_1 + 8) <= fConst;`
///
/// # Arguments
/// - `this`: the tween; `+0x04` range, `+0x08` value, `+0x14` mode byte.
///
/// # Returns
/// `true` when the tween has finished.
///
/// # Safety
/// `this..this+0x18` must be a live tween.
///
/// # Confidence
/// high - the two comparisons, the mode bytes and the `1.0f`/`0.0f` constants
/// are all literal.
pub unsafe fn fn_801637f0(this: usize) -> bool {
    let mode = platform::read_u8(this + 0x14);
    if mode == 1 {
        return platform::read_f32(this + 0x04) - platform::read_f32(W89_F32_ONE)
            <= platform::read_f32(this + 0x08);
    }
    if mode != 3 {
        return false;
    }
    platform::read_f32(this + 0x08) <= platform::read_f32(W89_F32_ZERO)
}

/// Builds a tween with the shipped defaults.
///
/// Installs two vtables, constructs the `+0x10` and `+0x30` sub-objects, parks
/// `0.0f` in `+0x88..+0x90`, and - only when `0x801DD1D0` reports mode `1` -
/// copies the `19/26` gain to `+0x74`, `1.0` to `+0x78` and sets bit `5` of
/// `+0x80`.
///
/// # Original
/// - Ghidra: `FUN_801638e0` at `0x801638e0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801638e0_FUN_801638e0.c`
/// - observed: `*(undefined1 *)(param_1 + 0xc) = 0x80;
///   *(undefined **)(param_1 + 8) = &DAT_803294c0; FUN_802acb40(...);
///   ... cVar2 = FUN_801dd1d0(); if (cVar2 == '\x01') { ... }`
///
/// # Arguments
/// - `this`: destination tween, at least `0x98` bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes the vtable words, constructs two sub-objects and may set the
/// display-mode defaults at `+0x74..+0x80`.
///
/// # Safety
/// `this` must be a live, exclusively owned block.
///
/// # Confidence
/// medium - every store is literal; the sub-object roles at `+0x10`/`+0x30`
/// are inferred from the vtable words they receive.
pub unsafe fn fn_801638e0(this: usize) -> usize {
    platform::write_u8(this + 0x0C, 0x80);
    platform::write_u32(this + 0x08, W89_VT_TWEEN_A as u32);
    platform::call1(W89_FN_802ACB40, (this + 0x10) as u32);
    platform::write_u32(this + 0x10, W89_VT_TWEEN_B as u32);
    platform::call1(W89_FN_802B4D30, (this + 0x30) as u32);
    let zero = platform::read_u32(W89_F32_ZERO);
    platform::write_u32(this + 0x84, 0);
    platform::write_u32(this + 0x88, zero);
    platform::write_u32(this + 0x8C, zero);
    platform::write_u32(this + 0x90, zero);
    platform::write_u32(this + 0x94, 0);
    if w89_call0_ret(W89_FN_801DD1D0) as u8 == 1 {
        platform::write_u32(this + 0x74, platform::read_u32(W89_F32_GAIN_19_26));
        platform::write_u32(this + 0x78, platform::read_u32(W89_F32_ONE));
        platform::write_u8(this + 0x80, platform::read_u8(this + 0x80) | 0x20);
    }
    this
}

/// Finds the 8-byte table entry whose object's `+0x18` name matches.
///
/// The container holds a count at `+0x18` and an array of 8-byte entries at
/// `+0x14`; each entry's first word points at an object whose name string
/// starts at `+0x18`. The scan stops at the first `strcmp`-equal name.
///
/// # Original
/// - Ghidra: `FUN_80163ff0` at `0x80163ff0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163ff0_FUN_80163ff0.c`
/// - observed: `iVar6 = *(int *)(iVar1 + 0x18); ... iVar5 = *(int *)(iVar1 +
///   0x14); iVar2 = FUN_802e1ab4((int)uVar8, *(int *)(iVar5 + iVar7) + 0x18);
///   if (iVar2 == 0) { iVar4 = iVar5 + iVar3 * 8; ... }`
///
/// # Arguments
/// - `container`: object with the count at `+0x18` and the array at `+0x14`.
/// - `name`: pointer to the NUL-terminated name to look for.
///
/// # Returns
/// Address of the matching 8-byte entry, or `0`.
///
/// # Safety
/// `container` and `name` must be live for the whole scan.
///
/// # Confidence
/// medium - the loop, the stride and the compare are literal; that
/// `0x802E1AB4` is a `strcmp` is inferred from its use here and in
/// [`fn_80161ad0`]'s neighbourhood.
pub unsafe fn fn_80163ff0(container: usize, name: u32) -> u32 {
    let count = platform::read_u32(container + 0x18) as i32;
    let array = platform::read_u32(container + 0x14) as usize;
    let mut index = 0i32;
    let mut offset = 0usize;
    let mut found = 0u32;
    while index < count {
        let entry = platform::read_u32(array + offset) as usize;
        if platform::call2(W89_FN_802E1AB4, name, (entry + 0x18) as u32) == 0 {
            found = (array + index as usize * 8) as u32;
            break;
        }
        offset += 8;
        index += 1;
    }
    found
}

/// Releases one auxiliary slot, or every slot when `index` is negative.
///
/// The slot table lives behind `r13 - 0x52A4` with its length at
/// `r13 - 0x52A8`. A negative index walks all slots, frees the table itself and
/// clears the pointer; a non-negative index releases only that slot and zeroes
/// its entry.
///
/// # Original
/// - Ghidra: `FUN_80164a40` at `0x80164a40` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164a40_FUN_80164a40.c`
/// - observed: `piVar2 = *(int **)(r13 + -0x52a4); if (piVar2 != 0) {
///   piVar1 = (int *)piVar2[param_1]; if (piVar1 != 0) { if (param_1 < 0) {
///   ... FUN_802b9280(*(r13 + -0x52a4)); *(r13 + -0x52a4) = 0; } else {
///   (**(code **)(*piVar1 + 0x18))(piVar1,1); piVar2[param_1] = 0; } } }`
///
/// # Arguments
/// - `index`: slot to release, or a negative value for "all slots".
///
/// # Side effects
/// Calls the `+0x18` release slot of each object, frees the table with
/// `0x802B9280` in the "all" case, and clears the global pointer.
///
/// # Safety
/// The table must be live and every non-null entry must be a releasable object.
///
/// # Confidence
/// medium - the guards, the release slot and the free are literal. The original
/// really does index the table with the (possibly negative) `index` for its
/// first guard; that out-of-range read is preserved on purpose.
pub unsafe fn fn_80164a40(index: i32) {
    let table = platform::read_u32(SDA_BASE - 0x52A4) as usize;
    if table == 0 {
        return;
    }
    // observed: the guard really does index with the (possibly negative) value;
    // the byte offset is computed modulo 2^32 like the original's `piVar2[i]`.
    let byte_offset = index.wrapping_mul(4) as u32 as usize;
    let slot = platform::read_u32(table.wrapping_add(byte_offset)) as usize;
    if slot == 0 {
        return;
    }
    if index < 0 {
        let count = platform::read_u32(SDA_BASE - 0x52A8) as usize;
        let mut cursor = table;
        let mut i = 0usize;
        while i < count {
            let object = platform::read_u32(cursor) as usize;
            if object != 0 {
                let release = platform::read_u32(platform::read_u32(object) as usize + 0x18) as usize;
                platform::call2(release, object as u32, 1);
            }
            cursor += 4;
            i += 1;
        }
        platform::call1(W89_FN_802B9280, table as u32);
        platform::write_u32(SDA_BASE - 0x52A4, 0);
    } else {
        let release = platform::read_u32(platform::read_u32(slot) as usize + 0x18) as usize;
        platform::call2(release, slot as u32, 1);
        platform::write_u32(table + index as usize * 4, 0);
    }
}

/// Creates the deferred-load object owned by `this`.
///
/// `0x8025EFB0` builds a reader over `heap`/`desc` with the completion callback
/// at `0x80164FA0`; the result is stored at `this + 4` and its two parameters
/// are set to `0x7F`. The reader is then cast to the `0x802F9018` interface and
/// the owner is stored at `+0xF0` of the cast result.
///
/// # Original
/// - Ghidra: `FUN_80164fd0` at `0x80164fd0` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164fd0_FUN_80164fd0.c`
/// - observed: `iVar2 = FUN_8025efb0(param_2,param_3,&LAB_80164fa0,1,1,0);
///   *(int *)(param_1 + 4) = iVar2; ... FUN_80257640(iVar2,0x7f);
///   FUN_80257670(...); ... piVar4[0x3c] = param_1;`
///
/// # Arguments
/// - `this`: owner object; the reader pointer lands at `+0x04`.
/// - `heap`: heap for the reader, or `0` for `r13 - 0x52B8`.
/// - `desc`: description block, or `0` for an 8-byte empty one.
///
/// # Returns
/// `1` on success, `0` when the reader could not be created.
///
/// # Side effects
/// Allocates the reader, writes `+0x04` of `this`, calls two parameter
/// setters, and stores `this` at `+0xF0` of the cast reader.
///
/// # Safety
/// `this` must be writable; a non-zero `desc` must be a readable block.
///
/// # Confidence
/// medium - all six arguments, the two setters and the `+0xF0` store are
/// literal; the cast target is inferred from the `0x802F9018` type word.
pub unsafe fn fn_80164fd0(this: usize, heap: usize, desc: usize) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut empty = crate::platform::GuestLocal::new( [0u8; 8]);
    let desc = if desc == 0 {
        empty.as_mut_ptr() as usize
    } else {
        desc
    };
    let reader = platform::call6(
        W89_FN_8025EFB0,
        heap as u32,
        desc as u32,
        W89_CB_80164FA0 as u32,
        1,
        1,
        0,
    ) as usize;
    platform::write_u32(this + 0x04, reader as u32);
    if reader == 0 {
        return 0;
    }
    platform::call2(W89_FN_80257640, reader as u32, 0x7F);
    platform::call2(W89_FN_80257670, reader as u32, 0x7F);
    let cast = w78_cast_to(platform::read_u32(this + 0x04) as usize, W89_TYPE_DEFERRED);
    // observed: the original stores through the cast result without a null
    // check; the unguarded store is preserved.
    platform::write_u32(cast + 0xF0, this as u32);
    1
}

/// Returns the `index`th child of an element.
///
/// The element is reached by casting `*(this + 4)` to the generic element type;
/// its child array header lives at `+0xE8` and `0x8023B860` walks it.
///
/// # Original
/// - Ghidra: `FUN_80168480` at `0x80168480` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168480_FUN_80168480.c`
/// - observed: `piVar3 = *(int **)(param_1 + 4); ... local_18 = &DAT_802f8fd8;
///   ... local_14[0] = piVar3[0x3a]; FUN_8023b860(local_14,param_2);`
///
/// # Arguments
/// - `this`: holder whose `+0x04` points at the element to inspect.
/// - `index`: child index.
///
/// # Returns
/// The child pointer, or `0` when the index is out of range.
///
/// # Safety
/// `this + 4` must name a castable element; a failed cast dereferences null,
/// exactly as in the original.
///
/// # Confidence
/// medium - the cast, the `+0xE8` read and the array call are literal; the
/// array semantics come from the already-translated `0x8023B860`.
pub unsafe fn fn_80168480(this: usize, index: u32) -> u32 {
    let element = w78_cast_to(platform::read_u32(this + 0x04) as usize, W78_TYPE_ELEMENT);
    let mut header = crate::platform::GuestLocal::new( platform::read_u32(element + 0xE8));
    platform::call2(
        W89_FN_8023B860,
        (header.as_mut_ptr() as *mut u32) as usize as u32,
        index,
    )
}

/// Draws one child of `this` with a tint, first path.
///
/// When the sound-object cast succeeds the draw entry is packed by
/// `0x8025C560`; otherwise the child element's own `+0x20` sub-object is used.
/// Both paths then call `0x8023D3C0` and release the temporary with `0x8023CA80`.
///
/// # Original
/// - Ghidra: `FUN_801685e0` at `0x801685e0` (304 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801685e0_FUN_801685e0.c`
/// - observed: `piVar4 = *(int **)(iVar2 + 4); ... local_5c =
///   FUN_80168480(iVar2,(int)uVar5); ... FUN_8023d3c0(&local_5c,param_3,
///   &local_64); FUN_8023ca80(&local_5c,0);`
///
/// # Arguments
/// - `this`: holder whose `+0x04` points at the element.
/// - `index`: child index.
/// - `arg3`: word forwarded as the second argument of the submit call.
/// - `color`: pointer to the tint word read before each submit.
/// - `arg5`: word forwarded to `0x8025C560`.
///
/// # Side effects
/// Builds a 0x54-byte draw entry on the stack and runs the submit/release pair.
///
/// # Safety
/// `this`, `color` and the child array must be live; a failed cast
/// dereferences null, exactly as in the original.
///
/// # Confidence
/// medium - the branch, both call chains and the `+0x3C` bias are literal; the
/// tint/entry roles are inferred from the `nw4r::lyt` draw call sites.
pub unsafe fn fn_801685e0(this: usize, index: u32, arg3: u32, color: usize, arg5: u32) {
    let child = w78_cast_to(platform::read_u32(this + 0x04) as usize, W78_TYPE_OBJECT);
    if child == 0 {
        let element = fn_80168480(this, index) as usize;
        let bias = platform::read_u32(element + 0x3C) as usize;
        let base = if bias == 0 { 0usize } else { element + bias };
        let mut target = crate::platform::GuestLocal::new( (base + 0x20) as u32);
        let mut tint = crate::platform::GuestLocal::new( platform::read_u32(color));
        platform::call3(
            W89_FN_8023D3C0,
            (target.as_mut_ptr() as *mut u32) as usize as u32,
            arg3,
            (tint.as_mut_ptr() as *mut u32) as usize as u32,
        );
        platform::call2(
            W89_FN_8023CA80,
            (target.as_mut_ptr() as *mut u32) as usize as u32,
            0,
        );
    } else {
        let mut entry = crate::platform::GuestLocal::new( [0u8; 0x54]);
        platform::call3(
            W89_FN_8025C660,
            entry.as_mut_ptr() as usize as u32,
            child as u32,
            index,
        );
        let mut packed = crate::platform::GuestLocal::new( platform::call2(
            W89_FN_8025C560,
            entry.as_mut_ptr() as usize as u32,
            arg5,
        ));
        let mut tint = crate::platform::GuestLocal::new( platform::read_u32(color));
        platform::call3(
            W89_FN_8023D3C0,
            (packed.as_mut_ptr() as *mut u32) as usize as u32,
            arg3,
            (tint.as_mut_ptr() as *mut u32) as usize as u32,
        );
        platform::call2(
            W89_FN_8023CA80,
            (packed.as_mut_ptr() as *mut u32) as usize as u32,
            0,
        );
    }
}

/// Draws every child of `this` with a tint, first path.
///
/// Same two branches as [`fn_801685e0`] but the child index comes from the
/// `+0xE8` array walk, and the count is re-read on every iteration.
///
/// # Original
/// - Ghidra: `FUN_80168710` at `0x80168710` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168710_FUN_80168710.c`
/// - observed: `local_54 = FUN_80168400(); ... for (uVar4 = 0; uVar3 =
///   FUN_8023b8c0(&local_54), uVar4 < uVar3; uVar4 = uVar4 + 1) { ... }`
///
/// # Arguments
/// - `this`: holder whose `+0x04` points at the element.
/// - `color`: tint word forwarded as the second submit argument.
/// - `pos`: pointer to the position/tint word read before each submit.
/// - `arg4`: word forwarded to `0x8025C560`.
///
/// # Side effects
/// Builds one 0x50-byte draw entry per child and runs the submit/release pair.
///
/// # Safety
/// `this`, `pos` and the child array must be live; a failed cast dereferences
/// null, exactly as in the original.
///
/// # Confidence
/// medium - the loop, both call chains and the `+0x3C` bias are literal; the
/// argument roles are inferred from [`fn_801685e0`]'s call sites.
pub unsafe fn fn_80168710(this: usize, color: u32, pos: usize, arg4: u32) {
    let mut header = crate::platform::GuestLocal::new( fn_80168400(this));
    let child = w78_cast_to(platform::read_u32(this + 0x04) as usize, W78_TYPE_OBJECT);
    let mut index = 0u32;
    while index < platform::call1(W89_FN_8023B8C0, (header.as_mut_ptr() as *mut u32) as usize as u32) {
        if child == 0 {
            let element = platform::call2(
                W89_FN_8023B860,
                (header.as_mut_ptr() as *mut u32) as usize as u32,
                index,
            ) as usize;
            let bias = platform::read_u32(element + 0x3C) as usize;
            let base = if bias == 0 { 0usize } else { element + bias };
            let mut target = crate::platform::GuestLocal::new( (base + 0x20) as u32);
            let mut tint = crate::platform::GuestLocal::new( platform::read_u32(pos));
            platform::call3(
                W89_FN_8023D3C0,
                (target.as_mut_ptr() as *mut u32) as usize as u32,
                color,
                (tint.as_mut_ptr() as *mut u32) as usize as u32,
            );
            platform::call2(
                W89_FN_8023CA80,
                (target.as_mut_ptr() as *mut u32) as usize as u32,
                0,
            );
        } else {
            let mut entry = crate::platform::GuestLocal::new( [0u8; 0x50]);
            platform::call3(
                W89_FN_8025C660,
                entry.as_mut_ptr() as usize as u32,
                child as u32,
                index,
            );
            let mut packed = crate::platform::GuestLocal::new( platform::call2(
                W89_FN_8025C560,
                entry.as_mut_ptr() as usize as u32,
                arg4,
            ));
            let mut tint = crate::platform::GuestLocal::new( platform::read_u32(pos));
            platform::call3(
                W89_FN_8023D3C0,
                (packed.as_mut_ptr() as *mut u32) as usize as u32,
                color,
                (tint.as_mut_ptr() as *mut u32) as usize as u32,
            );
            platform::call2(
                W89_FN_8023CA80,
                (packed.as_mut_ptr() as *mut u32) as usize as u32,
                0,
            );
        }
        index += 1;
    }
}

/// Draws one child of `this` with a tint, second path.
///
/// Identical to [`fn_801685e0`] except that the submit call is `0x8023D520`
/// instead of `0x8023D3C0`.
///
/// # Original
/// - Ghidra: `FUN_80168880` at `0x80168880` (304 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168880_FUN_80168880.c`
/// - observed: `local_5c = FUN_80168480(iVar2,(int)uVar5); ...
///   FUN_8023d520(&local_5c,param_3,&local_64); FUN_8023ca80(&local_5c,0);`
///
/// # Arguments
/// - `this`: holder whose `+0x04` points at the element.
/// - `index`: child index.
/// - `arg3`: word forwarded as the second argument of the submit call.
/// - `color`: pointer to the tint word read before each submit.
/// - `arg5`: word forwarded to `0x8025C560`.
///
/// # Side effects
/// Builds a 0x54-byte draw entry on the stack and runs the submit/release pair.
///
/// # Safety
/// Same as [`fn_801685e0`].
///
/// # Confidence
/// medium - the twin of [`fn_801685e0`] with one different callee.
pub unsafe fn fn_80168880(this: usize, index: u32, arg3: u32, color: usize, arg5: u32) {
    let child = w78_cast_to(platform::read_u32(this + 0x04) as usize, W78_TYPE_OBJECT);
    if child == 0 {
        let element = fn_80168480(this, index) as usize;
        let bias = platform::read_u32(element + 0x3C) as usize;
        let base = if bias == 0 { 0usize } else { element + bias };
        let mut target = crate::platform::GuestLocal::new( (base + 0x20) as u32);
        let mut tint = crate::platform::GuestLocal::new( platform::read_u32(color));
        platform::call3(
            W89_FN_8023D520,
            (target.as_mut_ptr() as *mut u32) as usize as u32,
            arg3,
            (tint.as_mut_ptr() as *mut u32) as usize as u32,
        );
        platform::call2(
            W89_FN_8023CA80,
            (target.as_mut_ptr() as *mut u32) as usize as u32,
            0,
        );
    } else {
        let mut entry = crate::platform::GuestLocal::new( [0u8; 0x54]);
        platform::call3(
            W89_FN_8025C660,
            entry.as_mut_ptr() as usize as u32,
            child as u32,
            index,
        );
        let mut packed = crate::platform::GuestLocal::new( platform::call2(
            W89_FN_8025C560,
            entry.as_mut_ptr() as usize as u32,
            arg5,
        ));
        let mut tint = crate::platform::GuestLocal::new( platform::read_u32(color));
        platform::call3(
            W89_FN_8023D520,
            (packed.as_mut_ptr() as *mut u32) as usize as u32,
            arg3,
            (tint.as_mut_ptr() as *mut u32) as usize as u32,
        );
        platform::call2(
            W89_FN_8023CA80,
            (packed.as_mut_ptr() as *mut u32) as usize as u32,
            0,
        );
    }
}

/// Draws every child of `this` with a tint, second path.
///
/// Identical to [`fn_80168710`] except that the submit call is `0x8023D520`.
///
/// # Original
/// - Ghidra: `FUN_801689b0` at `0x801689b0` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801689b0_FUN_801689b0.c`
/// - observed: `local_54 = FUN_80168400(); ... FUN_8023d520(&local_5c,
///   (int)uVar6,&local_64); FUN_8023ca80(&local_5c,0);`
///
/// # Arguments
/// - `this`: holder whose `+0x04` points at the element.
/// - `color`: tint word forwarded as the second submit argument.
/// - `pos`: pointer to the position/tint word read before each submit.
/// - `arg4`: word forwarded to `0x8025C560`.
///
/// # Side effects
/// Builds one 0x50-byte draw entry per child and runs the submit/release pair.
///
/// # Safety
/// Same as [`fn_80168710`].
///
/// # Confidence
/// medium - the twin of [`fn_80168710`] with one different callee.
pub unsafe fn fn_801689b0(this: usize, color: u32, pos: usize, arg4: u32) {
    let mut header = crate::platform::GuestLocal::new( fn_80168400(this));
    let child = w78_cast_to(platform::read_u32(this + 0x04) as usize, W78_TYPE_OBJECT);
    let mut index = 0u32;
    while index < platform::call1(W89_FN_8023B8C0, (header.as_mut_ptr() as *mut u32) as usize as u32) {
        if child == 0 {
            let element = platform::call2(
                W89_FN_8023B860,
                (header.as_mut_ptr() as *mut u32) as usize as u32,
                index,
            ) as usize;
            let bias = platform::read_u32(element + 0x3C) as usize;
            let base = if bias == 0 { 0usize } else { element + bias };
            let mut target = crate::platform::GuestLocal::new( (base + 0x20) as u32);
            let mut tint = crate::platform::GuestLocal::new( platform::read_u32(pos));
            platform::call3(
                W89_FN_8023D520,
                (target.as_mut_ptr() as *mut u32) as usize as u32,
                color,
                (tint.as_mut_ptr() as *mut u32) as usize as u32,
            );
            platform::call2(
                W89_FN_8023CA80,
                (target.as_mut_ptr() as *mut u32) as usize as u32,
                0,
            );
        } else {
            let mut entry = crate::platform::GuestLocal::new( [0u8; 0x50]);
            platform::call3(
                W89_FN_8025C660,
                entry.as_mut_ptr() as usize as u32,
                child as u32,
                index,
            );
            let mut packed = crate::platform::GuestLocal::new( platform::call2(
                W89_FN_8025C560,
                entry.as_mut_ptr() as usize as u32,
                arg4,
            ));
            let mut tint = crate::platform::GuestLocal::new( platform::read_u32(pos));
            platform::call3(
                W89_FN_8023D520,
                (packed.as_mut_ptr() as *mut u32) as usize as u32,
                color,
                (tint.as_mut_ptr() as *mut u32) as usize as u32,
            );
            platform::call2(
                W89_FN_8023CA80,
                (packed.as_mut_ptr() as *mut u32) as usize as u32,
                0,
            );
        }
        index += 1;
    }
}

/// Steps the cubic tween used by the `+0x04` interpolators.
///
/// The value at `+0x08` advances by the step at `+0x0C`. If the accumulator has
/// not reached `1.0` the three Hermite coefficients at `+0x10..+0x18` are
/// rebuilt from the remaining time and the new rate at `+0x04`; otherwise
/// `0x80168B90` is called and the step stops.
///
/// # Original
/// - Ghidra: `FUN_80168c10` at `0x80168c10` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168c10_FUN_80168c10.c`
/// - observed: `fVar3 = *(float *)(param_1 + 8) + *(float *)(param_1 + 0xc);
///   *(float *)(param_1 + 8) = fVar3; if (fVar2 <= fVar3) { FUN_80168b90();
///   return; } *(undefined1 *)(param_1 + 0x1d) = 1; ...`; the export omits the
///   argument, but `r3` still holds `this` at that call.
///
/// # Arguments
/// - `this`: the tween; `+0x04` rate, `+0x08` accumulator, `+0x0C` step,
///   `+0x10..+0x18` coefficients, `+0x1D` "active" byte.
///
/// # Side effects
/// Writes `+0x08`, `+0x1D`, `+0x04`, `+0x10`, `+0x14` and `+0x18`; may call
/// `0x80168B90`.
///
/// # Safety
/// `this..this+0x20` must be a live tween with a non-zero rate at `+0x04`.
///
/// # Confidence
/// medium - every term is literal, including the `-(s*i*i - 2i)` cubic; the
/// last two coefficients are computed from the same `1/sum` value twice in the
/// export and are factored here into one local.
pub unsafe fn fn_80168c10(this: usize) {
    let rate = platform::read_f32(this + 0x04);
    if platform::read_f32(W89_F32_TWEEN_ZERO) == rate {
        return;
    }
    let one = platform::read_f32(W89_F32_TWEEN_ONE);
    let accumulator = platform::read_f32(this + 0x08) + platform::read_f32(this + 0x0C);
    platform::write_f32(this + 0x08, accumulator);
    if one <= accumulator {
        platform::call1(W89_FN_80168B90, this as u32);
        return;
    }
    platform::write_u8(this + 0x1D, 1);
    let base = rate - accumulator * rate * accumulator;
    platform::write_f32(this + 0x04, base);
    let rest = one - base;
    let sum = base / rate + rest;
    let inverse = 1.0f32 / sum;
    let cubic = -(sum * inverse * inverse - (inverse + inverse));
    platform::write_f32(this + 0x10, (base / rate) * cubic);
    platform::write_f32(this + 0x14, rest * cubic);
    platform::write_f32(this + 0x18, rest * -cubic);
}

/// Attaches a pane to its parent, or reports that the parent is attached.
///
/// `0x80169C80` is called with the low word of the token only when the
/// descriptor's `+0x0C` slot reports "not attached"; `0x80168280` then files the
/// descriptor under the object.
///
/// # Original
/// - Ghidra: `FUN_80169f80` at `0x80169f80` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169f80_FUN_80169f80.c`
/// - observed: `iVar1 = (**(code **)(*param_3 + 0xc))(param_3); if (iVar1 ==
///   0) { FUN_80169c80(param_1,param_2 + 0xc); } FUN_80168280(param_2,param_3);`
///
/// # Arguments
/// - `token`: two-word token; only `r3` reaches the one-argument `0x80169C80`.
/// - `object`: destination object.
/// - `descriptor`: the pane/descriptor being attached.
///
/// # Side effects
/// One virtual query and, when it answers zero, one attach call plus one
/// filing call.
///
/// # Safety
/// `descriptor` must be a live object with a valid `+0x0C` slot.
///
/// # Confidence
/// medium - the three calls are literal; that `0x80169C80` takes one argument
/// is visible in its own export, so the second word of `param_1` is dropped.
pub unsafe fn fn_80169f80(token: u64, object: usize, descriptor: usize) {
    let query = platform::read_u32(platform::read_u32(descriptor) as usize + 0x0C) as usize;
    if platform::call1(query, descriptor as u32) == 0 {
        fn_80169c80(token as u32 as usize);
    }
    fn_80168280(object, descriptor);
}

/// Constructs an object with the `0x80329700` vtable.
///
/// The base constructor is called first, then the vtable is installed and the
/// display-mode byte is copied to `+0x14`.
///
/// # Original
/// - Ghidra: `FUN_8016ab90` at `0x8016ab90` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ab90_FUN_8016ab90.c`
/// - observed: `FUN_8016dc00(); *param_1 = &DAT_80329700; uVar1 =
///   FUN_801dd1d0(); *(undefined1 *)(param_1 + 5) = uVar1;`
///
/// # Arguments
/// - `this`: destination object.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Runs the base constructor and writes `+0x00` and `+0x14`.
///
/// # Safety
/// `this` must be a live, exclusively owned block.
///
/// # Confidence
/// medium - all three steps are literal, but the export shows no argument list
/// for `0x8016DC00`, so the base constructor is emitted as a zero-argument
/// call.
pub unsafe fn fn_8016ab90(this: usize) -> usize {
    platform::call0(W89_FN_8016DC00);
    platform::write_u32(this, W89_VT_OBJECT as u32);
    platform::write_u8(this + 0x14, w89_call0_ret(W89_FN_801DD1D0) as u8);
    this
}

/// Submits one layout request and publishes the resulting handle.
///
/// The five request words are copied into a stack block behind the two-word
/// heap tag, the block is queued with `0x802D8E70`, and a `0x20`-byte handle is
/// allocated from `heap` and wrapped by `0x802DA310`. The handle lands in
/// `r13 - 0x5250`.
///
/// # Original
/// - Ghidra: `FUN_8016c400` at `0x8016c400` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c400_FUN_8016c400.c`
/// - observed: `FUN_802d8de0(param_6,param_7,uVar2); local_30 = param_3; ...
///   FUN_802d8e70(*(r13 + -0x46b0),&local_38); FUN_802da590();
///   iVar1 = FUN_802b9240(0x20,uVar2,4); ... *(r13 + -0x5250) = uVar2;`
///
/// # Arguments
/// - `heap`: heap word, also the third argument of `0x802D8DE0`.
/// - `tag`: second word of the stack block.
/// - `arg3`, `arg4`, `arg5`: request words stored in the block.
/// - `arg6`, `arg7`: the two words forwarded to `0x802D8DE0`.
///
/// # Side effects
/// Queues a five-word block, flushes the queue, allocates and installs the
/// handle at `r13 - 0x5250`.
///
/// # Safety
/// `heap` must be a live EGG heap when the allocation runs.
///
/// # Confidence
/// medium - every store and call is literal; the two-word tag in the block is
/// reproduced from the prologue-fragment rule (first/second parameter).
pub unsafe fn fn_8016c400(
    heap: u32,
    tag: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
) {
    platform::call3(W89_FN_802D8DE0, arg6, arg7, heap);
    let mut block = crate::platform::GuestLocal::new( [heap, tag, arg3, arg4, arg5]);
    platform::call2(
        W89_FN_802D8E70,
        platform::read_u32(SDA_BASE - 0x46B0),
        block.as_mut_ptr() as usize as u32,
    );
    platform::call0(W89_FN_802DA590);
    let memory = platform::call3(W78_FN_802B9240, 0x20, heap, 4);
    let handle = if memory == 0 {
        0
    } else {
        platform::call3(W89_FN_802DA310, memory, 0, 0)
    };
    platform::write_u32(SDA_BASE - 0x5250, handle);
}

/// Retires every finished entry of the `0x80377C3C` list.
///
/// Each node's state is read through `0x802967E0(node + 0x14)`; states `1` and
/// `2` are handed to `0x8016D120` and stay linked, every other state is unlinked
/// in both directions, cleared and destroyed through its `+0x08` vtable.
///
/// # Original
/// - Ghidra: `FUN_8016c5c0` at `0x8016c5c0` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c5c0_FUN_8016c5c0.c`
/// - observed: `if (*(int *)(r13 + -0x524c) != 0) { puVar5 = &DAT_80377c3c; if
///   (DAT_80377c40 != &DAT_80377c3c) puVar5 = DAT_80377c40; while (...) { ...
///   DAT_80377c44 = DAT_80377c44 + -1; ... (**(code **)(puVar4[2] + 8))(puVar4,1); } }`
///
/// # Side effects
/// Unlinks and destroys finished nodes, decrements the count, and calls the
/// state dispatcher for the live ones.
///
/// # Safety
/// The list and the owner handle at `r13 - 0x524C` must be live.
///
/// # Confidence
/// medium - the walk, the two state values, the unlink stores and the destroy
/// slot are literal; the `0x80377C3C` head/tail roles are the same ones
/// [`fn_8016c4a0`] already uses.
pub unsafe fn fn_8016c5c0() {
    // observed: the export opens with the `0x802DCD70` register-save fragment,
    // which is the prologue of this function, not a call.
    if platform::read_u32(W78_G_ACTIVE_LIST) == 0 {
        return;
    }
    let sentinel = W78_LIST_SENTINEL;
    let tail = platform::read_u32(W78_LIST_TAIL) as usize;
    let mut node = if tail != sentinel { tail } else { sentinel };
    while node != sentinel {
        let next = platform::read_u32(node + 4);
        let state_object = platform::call1(W64_FN_802967E0, (node + 0x14) as u32) as usize;
        let state = if state_object == 0 {
            0
        } else {
            platform::read_u32(state_object + 0x0C)
        };
        if state == 1 || state == 2 {
            platform::call1(W89_FN_8016D120, node as u32);
        } else {
            let previous = platform::read_u32(node) as usize;
            let mut new_tail = next as usize;
            if previous != sentinel {
                platform::write_u32(previous + 4, next);
                new_tail = platform::read_u32(W78_LIST_TAIL) as usize;
            }
            platform::write_u32(W78_LIST_TAIL, new_tail as u32);
            if next as usize == sentinel {
                platform::write_u32(W78_LIST_SENTINEL, previous as u32);
            } else {
                platform::write_u32(next as usize, previous as u32);
            }
            platform::write_u32(node, 0);
            platform::write_u32(node + 4, 0);
            platform::write_u32(
                W78_LIST_COUNT,
                platform::read_u32(W78_LIST_COUNT).wrapping_sub(1),
            );
            if node != 0 {
                let destroy =
                    platform::read_u32(platform::read_u32(node + 8) as usize + 8) as usize;
                platform::call2(destroy, node as u32, 1);
            }
        }
        node = next as usize;
    }
}

/// Builds a request object on the stack and dispatches it.
///
/// `0x802D7AA0` constructs a 0x138-byte stack object, its vtable is replaced
/// with `0x80329A20`, and all five words of the caller's request are handed to
/// [`fn_8016cab0`] together with that object. `0x802D7B20` destroys the object
/// afterwards.
///
/// # Original
/// - Ghidra: `FUN_8016c890` at `0x8016c890` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c890_FUN_8016c890.c`
/// - observed: `FUN_802d7aa0(local_138); local_138[0] = &DAT_80329a20;
///   uVar1 = FUN_8016cab0(local_138,(int)(uVar2 >> 0x20),(int)uVar2,param_3,
///   param_4,param_5); FUN_802d7b20(local_138,0);`
///
/// # Arguments
/// - `a`, `b`: first two request words, forwarded as `r3`/`r4`.
/// - `c`, `d`, `e`: remaining request words forwarded to [`fn_8016cab0`].
///
/// # Side effects
/// Builds and destroys a 0x138-byte stack object and runs one dispatch.
///
/// # Safety
/// The dispatcher must not keep a pointer to the stack object after returning.
///
/// # Confidence
/// medium - the constructor, the vtable store, the six-argument dispatch and
/// the destructor are literal; the request words are untyped in the export.
pub unsafe fn fn_8016c890(a: u32, b: u32, c: u32, d: u32, e: u32) {
    let mut object = crate::platform::GuestLocal::new( [0u32; 0x138 / 4]);
    platform::call1(W89_FN_802D7AA0, object.as_mut_ptr() as usize as u32);
    object[0] = W89_VT_STACK_REQUEST as u32;
    platform::call6(
        W89_FN_8016CAB0,
        object.as_mut_ptr() as usize as u32,
        a,
        b,
        c,
        d,
        e,
    );
    platform::call2(W89_FN_802D7B20, object.as_mut_ptr() as usize as u32, 0);
}

/// Runs one registered request through the object's dispatch slots.
///
/// The `+0x9C` slot decides whether the request is still valid. If it is, a
/// 0x38-byte packet is built from `arg4..arg6` by `0x8016C9E0`, submitted
/// through `+0x8C` and committed through `+0x94`.
///
/// # Original
/// - Ghidra: `FUN_8016cab0` at `0x8016cab0` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016cab0_FUN_8016cab0.c`
/// - observed: `iVar1 = (**(code **)(*param_1 + 0x9c))(); if (iVar1 != 0) {
///   FUN_8016c9e0(param_4,param_5,param_6,auStack_48);
///   (**(code **)(*param_1 + 0x8c))(param_1,auStack_48);
///   (**(code **)(*param_1 + 0x94))(param_1); } return iVar1 != 0;`
///
/// # Arguments
/// - `this`: the request object.
/// - `_arg2`, `_arg3`: words present in the export but unused by the body.
/// - `arg4`, `arg5`, `arg6`: forwarded to the packet builder.
///
/// # Returns
/// `true` when the `+0x9C` slot accepted the request.
///
/// # Side effects
/// Builds a 0x38-byte stack packet and runs two virtual slots.
///
/// # Safety
/// `this` must be a live object with a valid vtable.
///
/// # Confidence
/// medium - the packet size, the two explicit slots and the return value are
/// literal. `// inferred:` the `+0x9C` predicate is called with `this`; the
/// export drops the receiver, but the neighbouring `+0x8C`/`+0x94` slots both
/// pass it explicitly.
pub unsafe fn fn_8016cab0(
    this: usize,
    _arg2: u32,
    _arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
) -> bool {
    let accepted = platform::call1(
        platform::read_u32(platform::read_u32(this) as usize + 0x9C) as usize,
        this as u32,
    );
    if accepted == 0 {
        return false;
    }
    let mut packet = crate::platform::GuestLocal::new( [0u8; 0x38]);
    platform::call4(
        W78_FN_8016C9E0,
        arg4,
        arg5,
        arg6,
        packet.as_mut_ptr() as usize as u32,
    );
    let submit = platform::read_u32(platform::read_u32(this) as usize + 0x8C) as usize;
    platform::call2(submit, this as u32, packet.as_mut_ptr() as usize as u32);
    let commit = platform::read_u32(platform::read_u32(this) as usize + 0x94) as usize;
    platform::call1(commit, this as u32);
    true
}

/// Submits one request and links the new object into the active list.
///
/// A 0x1C-byte object is allocated from the list's own heap, its vtable is set
/// to `0x80329A14`, two words of context are copied, the request is registered
/// with `0x80296750`, and the `+0xA0` slot of `this` performs the submission.
/// On success the object is appended to the `0x80377C3C` list; on failure it is
/// destroyed through its own `+0x08` slot.
///
/// # Original
/// - Ghidra: `FUN_8016cd40` at `0x8016cd40` (352 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016cd40_FUN_8016cd40.c`
/// - observed: `puVar2 = FUN_802b9240(0x1c,*(r13 + -0x524c),4); ...
///   iVar3 = (**(code **)(*piVar1 + 0xa0))(piVar1,(int)uVar5,param_3,param_4,
///   param_5,param_6); ... DAT_80377c44 = DAT_80377c44 + 1;`
///
/// # Arguments
/// - `this`: dispatcher object whose `+0xA0` slot submits the request.
/// - `arg2`: word forwarded as the second slot argument.
/// - `arg3`, `arg4`, `arg5`, `arg6`: remaining slot arguments.
/// - `arg7`, `arg8`: two context words stored at `+0x0C`/`+0x10`.
///
/// # Returns
/// Nothing; the C version returns `1`/`0` through the register stub only.
///
/// # Side effects
/// Allocates, registers, links or destroys one list node.
///
/// # Safety
/// `this` must be a live dispatcher and `r13 - 0x524C` must hold its heap.
///
/// # Confidence
/// medium - the allocation size, the vtable, the link stores and the destroy
/// slot are literal; the slot argument count comes from the explicit call.
pub unsafe fn fn_8016cd40(
    this: usize,
    arg2: u32,
    arg3: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    arg7: u32,
    arg8: u32,
) {
    let object = platform::call3(
        W78_FN_802B9240,
        0x1C,
        platform::read_u32(W78_G_ACTIVE_LIST),
        4,
    ) as usize;
    if object == 0 {
        return;
    }
    platform::write_u32(object + 0x08, W89_VT_REQUEST as u32);
    platform::write_u32(object, 0);
    platform::write_u32(object + 0x04, 0);
    platform::write_u32(object + 0x0C, arg7);
    platform::write_u32(object + 0x10, arg8);
    platform::call1(W89_FN_80296750, (object + 0x14) as u32);
    let submit = platform::read_u32(platform::read_u32(this) as usize + 0xA0) as usize;
    if platform::call6(submit, this as u32, arg2, arg3, arg4, arg5, arg6) == 0 {
        let destroy = platform::read_u32(platform::read_u32(object + 0x08) as usize + 8) as usize;
        platform::call2(destroy, object as u32, 1);
        return;
    }
    platform::call2(W89_FN_80296760, (object + 0x14) as u32, (this + 0x74) as u32);
    let sentinel = W78_LIST_SENTINEL;
    if platform::read_u32(W78_LIST_TAIL) == sentinel as u32 {
        // observed: empty list - both links point at the sentinel.
        platform::write_u32(object + 0x04, sentinel as u32);
        platform::write_u32(object, sentinel as u32);
        platform::write_u32(W78_LIST_TAIL, object as u32);
    } else {
        // observed: `*puVar2 = DAT_80377c3c` copies the current head link, and
        // `DAT_80377c3c[1]` is the tail word at `0x80377C40`.
        platform::write_u32(object, platform::read_u32(W78_LIST_SENTINEL));
        platform::write_u32(object + 0x04, sentinel as u32);
        platform::write_u32(W78_LIST_TAIL, object as u32);
    }
    platform::write_u32(
        W78_LIST_COUNT,
        platform::read_u32(W78_LIST_COUNT).wrapping_add(1),
    );
    platform::write_u32(W78_LIST_SENTINEL, object as u32);
}

/// Clears the tail fields of one 0x136-byte record.
///
/// # Original
/// - Ghidra: `FUN_8016d8a0` at `0x8016d8a0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d8a0_FUN_8016d8a0.c`
/// - observed: `*(undefined4 *)(param_1 + 0x128) = *(r2 + -0x5378);
///   *(undefined2 *)(param_1 + 300) = 0; ... *(undefined1 *)(param_1 + 0x131)
///   = 0x80; ...`
///
/// # Arguments
/// - `this`: record to clear.
///
/// # Side effects
/// Writes one word, one half-word and six bytes. `+0x132` is not touched, which
/// matches the original's byte-at-a-time stores.
///
/// # Safety
/// `this..this+0x136` must be writable.
///
/// # Confidence
/// high - all stores are literal and the `r2 - 0x5378` word is `0.0f` in
/// `main.elf`.
pub unsafe fn fn_8016d8a0(this: usize) {
    platform::write_u32(this + 0x128, platform::read_u32(W89_F32_RECORD_ZERO));
    platform::write_u16(this + 0x12C, 0);
    platform::write_u8(this + 0x12E, 0);
    platform::write_u8(this + 0x12F, 0);
    platform::write_u8(this + 0x130, 0);
    platform::write_u8(this + 0x131, 0x80);
    platform::write_u8(this + 0x133, 0);
    platform::write_u8(this + 0x134, 0);
    platform::write_u8(this + 0x135, 0);
}

/// Tests one point against the six plane pairs of an oriented box.
///
/// The point is transformed by the three rows at `+0x00`, `+0x10` and `+0x20`
/// and compared against the near/far bounds at `+0x70`/`+0x74` and the four
/// lateral bounds. The last comparison is written with the original's
/// `countLeadingZeros` idiom, which yields `1` when the point is on the inside
/// side of that plane and `0` when it is not.
///
/// # Original
/// - Ghidra: `FUN_8016e230` at `0x8016e230` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e230_FUN_8016e230.c`
/// - observed: `fVar5 = param_1[0xb] + param_1[10] * fVar3 + param_1[8] *
///   fVar1 + param_1[9] * fVar2; if (fVar5 - fVar4 <= param_1[0x1c]) { ...
///   uVar7 = countLeadingZeros((uint)(fVar4 < fVar1 * param_1[0x19] +
///   fVar5 * param_1[0x1a])); uVar7 = uVar7 >> 5; }`
///
/// # Arguments
/// - `this`: the box; rows at `+0x00`/`+0x10`/`+0x20`, bounds at `+0x70`.
/// - `view`: pointer to four floats `{x, y, z, w}`.
///
/// # Returns
/// `1` when the point is inside every plane, `0` otherwise.
///
/// # Side effects
/// None; the function only reads.
///
/// # Safety
/// `this + 0x78` and `view + 0x10` must be readable.
///
/// # Confidence
/// medium - every term and comparison is literal. The callers in the export
/// pass only one argument, but the body reads four floats through the second
/// pointer, so the second parameter is part of the real signature.
pub unsafe fn fn_8016e230(this: usize, view: usize) -> u32 {
    let x = platform::read_f32(view);
    let y = platform::read_f32(view + 0x04);
    let z = platform::read_f32(view + 0x08);
    let w = platform::read_f32(view + 0x0C);
    let depth = platform::read_f32(this + 0x2C)
        + platform::read_f32(this + 0x28) * z
        + platform::read_f32(this + 0x20) * x
        + platform::read_f32(this + 0x24) * y;
    if depth - w > platform::read_f32(this + 0x70) {
        return 0;
    }
    if platform::read_f32(this + 0x74) > depth + w {
        return 0;
    }
    let lateral = platform::read_f32(this + 0x0C)
        + platform::read_f32(this + 0x08) * z
        + platform::read_f32(this) * x
        + platform::read_f32(this + 0x04) * y;
    if lateral * platform::read_f32(this + 0x30) + depth * platform::read_f32(this + 0x38) > w {
        return 0;
    }
    if lateral * platform::read_f32(this + 0x40) + depth * platform::read_f32(this + 0x48) > w {
        return 0;
    }
    let vertical = platform::read_f32(this + 0x1C)
        + platform::read_f32(this + 0x18) * z
        + platform::read_f32(this + 0x10) * x
        + platform::read_f32(this + 0x14) * y;
    if vertical * platform::read_f32(this + 0x54) + depth * platform::read_f32(this + 0x58) > w {
        return 0;
    }
    let last = vertical * platform::read_f32(this + 0x64) + depth * platform::read_f32(this + 0x68);
    if w < last {
        0
    } else {
        1
    }
}

// ===========================================================================
// Layout-slot builders, list leaves and the control page (wave 93)
//
// This pass takes the last reconstructable bodies of the `0x80160000` page.
// Three conventions from wave 89 still apply, because nearly every function
// below depends on them:
//
// * `FUN_802dcd5c/64/68/6c/70/74` are Metrowerks **prologue fragments**. The
//   `undefined8` the decompiler shows as their "return" is the enclosing
//   function's own incoming `r3:r4`: `hi` is the first parameter and `lo` the
//   second. Every body below spells those two out and never emits the matching
//   `0x802DCDB0..BC` epilogue fragment as a call.
// * The decompiler drops the receiver of some virtual and direct calls. Where
//   the object is passed explicitly by the neighbouring slots, or is the
//   object the code just allocated, the receiver is added back and the doc
//   block says `// inferred: receiver`. This is the rule wave 89 introduced.
// * Every absolute address below is a constant with its original role; no
//   callee outside this file is renamed.
// ===========================================================================

/// `0x802B5C20` - element count of a sound-group description block.
const W93_FN_802B5C20: usize = 0x802B_5C20;
/// `0x802B5C40` - first element of that block (walked with a `0x14` stride).
const W93_FN_802B5C40: usize = 0x802B_5C40;
/// `0x802B5C60` - per-element size word.
const W93_FN_802B5C60: usize = 0x802B_5C60;
/// `0x802B5C80` - per-element flag word (`s16`).
const W93_FN_802B5C80: usize = 0x802B_5C80;
/// `0x802B5DA0` - build the `0x10`-byte sub-object of one element.
const W93_FN_802B5DA0: usize = 0x802B_5DA0;
/// `0x802B5B10` - adopt a prepared description block.
const W93_FN_802B5B10: usize = 0x802B_5B10;
/// `0x802AC830` - element `i` of a sound-bank description block.
const W93_FN_802AC830: usize = 0x802A_C830;
/// `0x802AD000` - build the transient draw object of a sound group.
const W93_FN_802AD000: usize = 0x802A_D000;
/// `0x802B7310` - register one built draw object.
const W93_FN_802B7310: usize = 0x802B_7310;
/// `0x802B9240` - allocate from the default heap.
const W93_FN_802B9240: usize = 0x802B_9240;
/// `0x802B9270` - allocate with an explicit heap and alignment.
const W93_FN_802B9270: usize = 0x802B_9270;
/// `0x802B9220` - bump-allocate one element from a slab.
const W93_FN_802B9220: usize = 0x802B_9220;
/// `0x802B99C0` - release a descriptor with a kind tag.
const W93_FN_802B99C0: usize = 0x802B_99C0;
/// `0x802B8F70` - free a block with an explicit size of zero.
const W93_FN_802B8F70: usize = 0x802B_8F70;
/// `0x801D5540` - reserve the element slab for a layout group.
const W93_FN_801D5540: usize = 0x801D_5540;
/// `0x801636A0` - forward a prepared description to a vtable slot. The local
/// [`fn_801636a0`] only models the receiver, so the three-word call site keeps
/// the address form.
const W93_FN_801636A0: usize = 0x8016_36A0;
/// `0x80165AE0` / `0x801666E0` / `0x801672D0` / `0x80167E50` - the four
/// "commit the finished layout group" twins.
const W93_FN_80165AE0: usize = 0x8016_5AE0;
const W93_FN_801666E0: usize = 0x8016_66E0;
const W93_FN_801672D0: usize = 0x8016_72D0;
const W93_FN_80167E50: usize = 0x8016_7E50;
/// `0x80243D20` / `0x80243B30` / `0x80245300` / `0x80245500` - the four
/// "create the layout view" constructors, one per element class.
const W93_FN_80243D20: usize = 0x8024_3D20;
const W93_FN_80243B30: usize = 0x8024_3B30;
const W93_FN_80245300: usize = 0x8024_5300;
const W93_FN_80245500: usize = 0x8024_5500;
/// `0x80246900` / `0x80246B00` - the two newer view constructors of that family.
const W93_FN_80246900: usize = 0x8024_6900;
const W93_FN_80246B00: usize = 0x8024_6B00;
/// `0x80242C30` - the four-argument variant used by the `0x80167D60` twin.
const W93_FN_80242C30: usize = 0x8024_2C30;
/// `0x80248990` - build a text/geometry record from two words.
const W93_FN_80248990: usize = 0x8024_8990;
/// `0x8024D4F0` - publish the active text context.
const W93_FN_8024D4F0: usize = 0x8024_D4F0;
/// `0x8024D5E0` - set the text render mode (`0x7FF`).
const W93_FN_8024D5E0: usize = 0x8024_D5E0;
/// `0x8025EEB0` - enable the reader's completion mode.
const W93_FN_8025EEB0: usize = 0x8025_EEB0;
/// `0x802589E0` - create the deferred reader over a `0x28`-byte header.
const W93_FN_802589E0: usize = 0x8025_89E0;
/// `0x802D87C0` - resolve the record that owns an actor.
const W93_FN_802D87C0: usize = 0x802D_87C0;
/// `0x80287C10` - submit an icon/record packet.
const W93_FN_80287C10: usize = 0x8028_7C10;
/// `0x802D1B50` - finish the pending text pass.
const W93_FN_802D1B50: usize = 0x802D_1B50;
/// `0x8016D4B0` - release the record built for an actor.
const W93_FN_8016D4B0: usize = 0x8016_D4B0;
/// `0x8016C960` - attach a record to the actor list.
const W93_FN_8016C960: usize = 0x8016_C960;
/// `0x8016C9E0` - build the `0x48`-byte draw block of an actor.
const W93_FN_8016C9E0: usize = 0x8016_C9E0;
/// `0x80168520` - copy a view into an element slot.
const W93_FN_80168520: usize = 0x8016_8520;
/// `0x801B2190` - `os_unlink`-style release of the handle's reader.
const W93_FN_801B2190: usize = 0x801B_2190;

/// `0x802F8588` - element type of the first layout-group class.
const W93_TYPE_802F8588: usize = 0x802F_8588;
/// `0x802F85A8` - type of the child element that class links at `+0x3C`.
const W93_TYPE_802F85A8: usize = 0x802F_85A8;
/// `0x802F85F0` - element type of the second layout-group class.
const W93_TYPE_802F85F0: usize = 0x802F_85F0;
/// `0x802F8610` - its child element type.
const W93_TYPE_802F8610: usize = 0x802F_8610;
/// `0x802F8658` - element type of the third layout-group class.
const W93_TYPE_802F8658: usize = 0x802F_8658;
/// `0x802F8678` - its child element type.
const W93_TYPE_802F8678: usize = 0x802F_8678;

/// `0x80329530` / `0x80329560` / `0x80329590` - vtables installed on the
/// `0x38`-byte elements built by the three group constructors.
const W93_VT_80329530: usize = 0x8032_9530;
const W93_VT_80329560: usize = 0x8032_9560;
const W93_VT_80329590: usize = 0x8032_9590;

/// `0x80377A5C` - second global intrusive list used by [`fn_80161f60`].
const W93_LIST_80377A5C: usize = 0x8037_7A5C;
/// `0x80377B1C` - sentinel head of the actor-record list built by
/// [`fn_8016d640`].
const W93_LIST_80377B1C: usize = 0x8037_7B1C;
/// `0x80377C30` - tail node of that list.
const W93_G_LIST_LAST: usize = 0x8037_7C30;
/// `0x80377C34` - head node of that list.
const W93_G_LIST_FIRST: usize = 0x8037_7C34;
/// `0x80377C38` - node count of that list.
const W93_G_LIST_COUNT: usize = 0x8037_7C38;

/// `r13 - 0x5200` - text-context pointer whose first word is the active context.
const W93_G_TEXT_CONTEXT: usize = SDA_BASE - 0x5200;
/// `r13 - 0x5240` - currently installed control object (read by
/// [`fn_8016daa0`] and [`fn_8016dae0`]).
const W93_G_CONTROL: usize = SDA_BASE - 0x5240;

/// `0x80328B48` - `u16` angle table indexed by the scaled ratio.
const W93_ANGLE_TABLE: usize = 0x8032_8B48;
/// `r2 - 0x546C` - scale applied to the ratio before the table lookup.
const W93_F32_ANGLE_SCALE: usize = SDA2_BASE - 0x546C;
/// `r2 - 0x5374` - threshold compared against an actor's `+0x128` word.
const W93_F32_RECORD_THRESHOLD: usize = SDA2_BASE - 0x5374;

/// Releases the reader of a handle and unlinks it from the live list.
///
/// The handle's `+0x14` owner is detached through its `+0x38` slot, the
/// `+0x34` lock block is reset, `os_unlink` on the owner decides whether the
/// release continues, and the reader at `+0x0C` reports how much of the shared
/// remaining size to give back. A pending `+0x18` object is discarded and
/// cleared.
///
/// # Original
/// - Ghidra: `FUN_80160800` at `0x80160800` (164 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160800_FUN_80160800.c`
/// - observed: `(**(code **)(*(int *)(param_1 + 0x14) + 0x38))();
///   FUN_801609d0(param_1 + 0x34); iVar1 = FUN_801b2190(...);
///   *(r13 - 0x5308) -= iVar1;`
///
/// # Arguments
/// - `this`: handle object; `+0x0C` reader, `+0x14` owner, `+0x18` pending
///   object, `+0x30` its vtable holder, `+0x34` lock block.
///
/// # Returns
/// `1` when the owner was unlinked, `0` when `os_unlink` refused.
///
/// # Side effects
/// Calls two virtual slots, subtracts from [`G_HANDLE_REMAIN`], and clears
/// `+0x18` after notifying it.
///
/// # Safety
/// `this` must be live and `+0x0C`/`+0x14` must point at their objects; the
/// deferred `+0x30` object is only dereferenced when `+0x18` is non-zero.
///
/// # Confidence
/// medium - every branch and store is literal; the two receivers are elided in
/// the export and are supplied from the owning fields (`// inferred:`).
pub unsafe fn fn_80160800(this: usize) -> u32 {
    let owner = platform::read_u32(this + 0x14) as usize;
    let detach = platform::read_u32(platform::read_u32(owner) as usize + 0x38) as usize;
    // inferred: receiver - the export shows `(... + 0x38)()` with no argument.
    platform::call1(detach, owner as u32);
    fn_801609d0(this + 0x34);
    if platform::call1(W93_FN_801B2190, owner as u32) == 0 {
        return 0;
    }
    let reader = platform::read_u32(this + 0x0C) as usize;
    let release = platform::read_u32(platform::read_u32(reader) as usize + 0x10) as usize;
    let released = platform::call1(release, this as u32) as i32;
    platform::write_u32(
        G_HANDLE_REMAIN,
        platform::read_u32(G_HANDLE_REMAIN).wrapping_sub(released as u32),
    );
    if platform::read_u32(this + 0x18) != 0 {
        let holder = platform::read_u32(this + 0x30) as usize;
        let discard = platform::read_u32(platform::read_u32(holder) as usize + 0x18) as usize;
        // inferred: receiver - the guard above keeps the holder alive.
        platform::call1(discard, holder as u32);
        platform::write_u32(this + 0x18, 0);
    }
    1
}

/// Returns one entry of the `0x80328B48` angle table for `atan2`-style input.
///
/// The ratio `param_1 / param_2` is rounded to `f32`, scaled by the `f32` at
/// `r2 - 0x546C`, truncated to a signed integer and used as a halfword index
/// into the table. Negative indices wrap below the table base, exactly as the
/// original `&DAT_80328b48 + index * 2` does.
///
/// # Original
/// - Ghidra: `FUN_80161180` at `0x80161180` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161180_FUN_80161180.c`
///
/// # Arguments
/// - `param_1`: numerator of the angle ratio.
/// - `param_2`: denominator of the angle ratio.
///
/// # Returns
/// The `u16` table entry (an angle in the caller's fixed-point unit).
///
/// # Safety
/// The table and the SDA2 scale word must be mapped; division by zero follows
/// the hardware `f64` result.
///
/// # Confidence
/// medium - the table lookup and both conversions are literal; the scale's
/// meaning is inferred from its use as the ratio multiplier.
pub unsafe fn fn_80161180(param_1: f64, param_2: f64) -> u16 {
    let scale = platform::read_f32(W93_F32_ANGLE_SCALE);
    let ratio = (param_1 / param_2) as f32;
    let index = (scale * ratio) as i32;
    platform::read_u16(W93_ANGLE_TABLE.wrapping_add((index as usize).wrapping_mul(2)))
}

/// Draws a sound-group description.
///
/// The element count and flag word come from the description at
/// `this + 4`. An empty description, or one with a non-zero flag, is handed to
/// the object's `+0x28` slot. Otherwise the object's `+0x10` word and the
/// description's `+0x04` word must be live, and each element is turned into a
/// draw object by `0x802AC830`/`0x802AD000`, submitted through the draw
/// object's `+0x10` slot, and registered by `0x802B7310`.
///
/// # Original
/// - Ghidra: `FUN_80163520` at `0x80163520` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163520_FUN_80163520.c`
/// - observed: `uVar11 = FUN_802dcd64(); ... uVar6 = FUN_802b5c20(iVar9);
///   sVar7 = FUN_802b5c80(iVar9); if ((uVar6 == 0) || (sVar7 != 0))
///   { uVar2 = (**(code **)(*piVar1 + 0x28))(piVar1,iVar9,param_3); }`
///
/// # Arguments
/// - `this`: draw target (the prologue's high word).
/// - `source`: description block (the prologue's low word).
/// - `arg`: forwarded to the draw slot.
///
/// # Returns
/// `1` when the element loop ran, otherwise the draw slot's result or `0`.
///
/// # Safety
/// `this` and `source` must be live; `source + 4` must point at the element
/// array when the loop path is taken.
///
/// # Confidence
/// high - the guard, the two stores and the loop are literal in the export.
pub unsafe fn fn_80163520(this: usize, source: usize, arg: u32) -> u32 {
    let count = platform::call1(W93_FN_802B5C20, source as u32) as u16;
    let flag = platform::call1(W93_FN_802B5C80, source as u32) as i16;
    if count == 0 || flag != 0 {
        let draw = platform::read_u32(platform::read_u32(this) as usize + 0x28) as usize;
        return platform::call3(draw, this as u32, source as u32, arg);
    }
    if platform::read_u32(this + 0x10) == 0 || platform::read_u32(source + 4) == 0 {
        return 0;
    }
    let mut element = platform::call1(W93_FN_802B5C40, source as u32);
    let mut index = 0u32;
    while index < count as u32 {
        let entry = platform::call2(
            W93_FN_802AC830,
            platform::read_u32(this + 0x14),
            element,
        );
        let size = platform::call1(W93_FN_802B5C60, source as u32);
        let sub = platform::call3(W93_FN_802B5DA0, source as u32, entry, size) as u16;
        let object = platform::call1(W93_FN_802AD000, this as u32) as usize;
        let submit = platform::read_u32(platform::read_u32(object) as usize + 0x10) as usize;
        platform::call4(
            submit,
            object as u32,
            platform::read_u32(source + 4),
            arg,
            sub as u32,
        );
        let size = platform::call1(W93_FN_802B5C60, source as u32);
        platform::call4(W93_FN_802B7310, entry, object as u32, size, 1);
        element = element.wrapping_add(0x14);
        index = index.wrapping_add(1);
    }
    1
}

/// Copies one sound-group description into a layout group.
///
/// The prepared block comes from the `0x801636A0` forwarder and is adopted at
/// `+0x04`. Depending on `mode`, the group is rendered either through the
/// owner's `+0x28` slot or through [`fn_80163520`]. The element count is
/// re-read from the adopted block, an eight-byte entry table is allocated and
/// each entry is filled with the description element and the value found by
/// walking the element's `+0x10` list.
///
/// # Original
/// - Ghidra: `FUN_80163e60` at `0x80163e60` (332 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163e60_FUN_80163e60.c`
/// - observed: `uVar10 = FUN_802dcd68(); ... uVar2 = FUN_801636a0(param_3,0,(int)uVar10);
///   ... if (param_5 == 0) (**(... + 0x28))(param_4,iVar1 + 4,...);
///   else FUN_80163520(param_4,iVar1 + 4,...);`
///
/// # Arguments
/// - `this`: layout group (the prologue's high word).
/// - `source`: prepared description (the prologue's low word).
/// - `context`: block whose `+0x04` word is forwarded by `0x801636A0`.
/// - `owner`: object owning the draw slot and the `+0x14` element array.
/// - `mode`: `0` selects the `+0x28` draw slot, non-zero selects
///   [`fn_80163520`].
///
/// # Returns
/// `1` after the entry table is built.
///
/// # Side effects
/// Allocates the eight-byte entry table at `+0x14`, writes the count at
/// `+0x18`, and walks each element's intrusive list.
///
/// # Safety
/// `this`, `context` and `owner` must be live; the list nodes are read until
/// the `+0x10` sentinel.
///
/// # Confidence
/// medium - the loop and all stores are literal; the prologue pair and the
/// elided forwarder arguments are documented above.
pub unsafe fn fn_80163e60(
    this: usize,
    source: usize,
    context: usize,
    owner: usize,
    mode: u32,
) -> u32 {
    let prepared = platform::call3(W93_FN_801636A0, context as u32, 0, source as u32);
    platform::call2(W93_FN_802B5B10, (this + 4) as u32, prepared);
    let target = platform::read_u32(context + 4);
    if mode == 0 {
        let draw = platform::read_u32(platform::read_u32(owner) as usize + 0x28) as usize;
        platform::call3(draw, owner as u32, (this + 4) as u32, target);
    } else {
        fn_80163520(owner, this + 4, target);
    }
    let count = platform::call1(W93_FN_802B5C20, (this + 4) as u32) as u16;
    platform::write_u32(this + 0x18, count as u32);
    let view = fn_801634c0() as usize;
    let table = platform::call3(
        W93_FN_802B9270,
        (count as u32) << 3,
        platform::read_u32(view + 0x14),
        4,
    );
    platform::write_u32(this + 0x14, table);
    let mut cursor = platform::call1(W93_FN_802B5C40, (this + 4) as u32);
    let mut index = 0u32;
    let mut offset = 0u32;
    loop {
        if platform::read_u32(this + 0x18) <= index {
            return 1;
        }
        let mut child_value = 0u32;
        let entry = platform::call2(W93_FN_802AC830, platform::read_u32(owner + 0x14), cursor);
        platform::write_u32(platform::read_u32(this + 0x14).wrapping_add(offset) as usize, entry);
        let mut node = platform::read_u32(entry as usize + 0x10) as usize;
        while node != entry as usize + 0x10 {
            let found = fn_801641b0(this, platform::read_u32(node + 8) as usize, 0);
            if found != 0 {
                child_value = platform::read_u32(found as usize + 8);
                break;
            }
            node = platform::read_u32(node) as usize;
        }
        cursor = cursor.wrapping_add(0x14);
        index = index.wrapping_add(1);
        let slot = platform::read_u32(this + 0x14).wrapping_add(offset);
        offset = offset.wrapping_add(8);
        platform::write_u32(slot as usize + 4, child_value);
    }
}

/// Recursively probes a tree for an element matching `this + 4`.
///
/// The tree's own `+0x58` slot is asked first. On a miss the element count is
/// re-read from `+0x64` and each child's `+0x24` slot is probed. When
/// `recurse` is non-zero the `+0x14` intrusive list is walked and each node is
/// retried with the node's payload (`node - 4`).
///
/// # Original
/// - Ghidra: `FUN_801641b0` at `0x801641b0` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801641b0_FUN_801641b0.c`
/// - observed: `uVar7 = FUN_802dcd74(); iVar1 = high; piVar4 = low;
///   iVar2 = (**(code **)(*piVar4 + 0x58))(piVar4,iVar1 + 4);`
///
/// # Arguments
/// - `this`: object whose `+0x04` word is the search key.
/// - `tree`: tree node carrying the `+0x58`/`+0x64`/`+0x6C` slots.
/// - `recurse`: non-zero to descend into the `+0x14` node list.
///
/// # Returns
/// The matching element pointer, or `0`.
///
/// # Safety
/// `this` and `tree` must be live; the node list is followed until its
/// self-referential sentinel.
///
/// # Confidence
/// medium - the slot indices and the recursive call are literal; the node
/// payload offset is inferred from the `node - 4` pointer arithmetic.
pub unsafe fn fn_801641b0(this: usize, tree: usize, recurse: u32) -> i32 {
    let probe_self = platform::read_u32(platform::read_u32(tree) as usize + 0x58) as usize;
    let mut result = platform::call2(probe_self, tree as u32, (this + 4) as u32) as i32;
    if result == 0 {
        let count_slot = platform::read_u32(platform::read_u32(tree) as usize + 0x64) as usize;
        let count = platform::call1(count_slot, tree as u32) as u8;
        let mut index = 0u8;
        while index < count {
            let child_slot = platform::read_u32(platform::read_u32(tree) as usize + 0x6C) as usize;
            let child = platform::call2(child_slot, tree as u32, index as u32) as usize;
            let probe_child = platform::read_u32(platform::read_u32(child) as usize + 0x24) as usize;
            result = platform::call2(probe_child, child as u32, (this + 4) as u32) as i32;
            if result != 0 {
                return result;
            }
            index = index.wrapping_add(1);
        }
        if recurse != 0 {
            let head = tree + 0x14;
            let mut node = platform::read_u32(head) as usize;
            while node != head {
                result = fn_801641b0(this, node.wrapping_sub(4), 1);
                if result != 0 {
                    return result;
                }
                node = platform::read_u32(node) as usize;
            }
        }
        result = 0;
    }
    result
}

/// Creates the LZ view, the `+0x28` header and the deferred reader of a page.
///
/// The `0x1c`-byte view is allocated from the prologue's high word, its LZ
/// wrapper is constructed by [`fn_8016a5e0`] and published in
/// `r13 - 0x52B8`. `0x802589E0` then builds the reader over a `0x28`-byte
/// stack block. Any failure frees the view through its `+0x10` vtable slot and
/// clears the published pointer.
///
/// # Original
/// - Ghidra: `FUN_80164500` at `0x80164500` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164500_FUN_80164500.c`
/// - observed: `uVar4 = FUN_802dcd74(); uVar3 = high; ... if (iVar1 != 0)
///   iVar2 = FUN_8016a5e0(); ... FUN_8016a690(iVar2,uVar3,0x20);`
///
/// # Arguments
/// - `this`: heap/owner word (the prologue's high word).
/// - `source`: reader key (the prologue's low word).
/// - `param_3`, `param_4`, `param_5`: forwarded to `0x802589E0`.
///
/// # Returns
/// `1` when the reader was created, `0` otherwise.
///
/// # Side effects
/// Allocates, publishes `r13 - 0x52B8` and `r13 - 0x52B4`, and frees the view
/// on failure.
///
/// # Safety
/// The heap word must be valid; the stack block is only touched by the callee.
///
/// # Confidence
/// medium - the allocation, three-argument init and failure path are literal;
/// the elided receiver of `0x8016A5E0` is supplied from the allocation.
pub unsafe fn fn_80164500(
    this: usize,
    source: usize,
    param_3: u32,
    param_4: u32,
    param_5: u32,
) -> u32 {
    let allocation = platform::call3(W93_FN_802B9240, 0x1C, this as u32, 4);
    let mut view = 0usize;
    if allocation != 0 {
        // inferred: receiver - the freshly allocated block is the view.
        view = fn_8016a5e0(allocation as usize);
    }
    platform::write_u32(W78_DEFAULT_HEAP, view as u32);
    if view == 0 {
        return 0;
    }
    fn_8016a690(view, this, 0x20);
    platform::call1(W93_FN_8025EEB0, 1);
    let context = platform::read_u32(W93_G_TEXT_CONTEXT) as usize;
    platform::call1(W93_FN_8024D4F0, platform::read_u32(context));
    let mut block = crate::platform::GuestLocal::new( [0u8; 0x28]);
    let reader = platform::call6(
        W93_FN_802589E0,
        platform::read_u32(W78_DEFAULT_HEAP),
        block.as_mut_ptr() as usize as u32,
        source as u32,
        param_3,
        param_4,
        param_5,
    ) as usize;
    platform::write_u32(G_CTX_MAIN, reader as u32);
    if reader == 0 {
        let view = platform::read_u32(W78_DEFAULT_HEAP) as usize;
        if view != 0 {
            let release = platform::read_u32(platform::read_u32(view + 0x10) as usize + 8) as usize;
            platform::call2(release, view as u32, 1);
        }
        platform::write_u32(W78_DEFAULT_HEAP, 0);
        return 0;
    }
    1
}

/// Builds one text record from a two-word key and installs it at `+0x04`.
///
/// The key words are copied into a local pair, `0x80248990` fills the caller's
/// output block, [`fn_80167fa0`] validates the record and a second
/// `0x80248990` call at `+0x0C` produces the child header. The child's `+0x30`
/// slot must accept the source word; otherwise the object's `+0x10` slot is
/// invoked and the build reports failure.
///
/// # Original
/// - Ghidra: `FUN_801650d0` at `0x801650d0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801650d0_FUN_801650d0.c`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word read twice.
/// - `spec`: specification word read twice.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: 36-byte output block, or `0` for a stack block.
///
/// # Returns
/// `1` when the child accepted the key, `0` otherwise.
///
/// # Safety
/// `this`, `source` and `spec` must be readable; `out` must be writable for
/// 36 bytes when non-zero.
///
/// # Confidence
/// medium - both record calls and the failure branch are literal; the receiver
/// of the final `+0x10` slot is the owner.
pub unsafe fn fn_801650d0(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 0x24]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    platform::call5(
        W93_FN_80248990,
        0,
        out as u32,
        (spec_word.as_mut_ptr() as *mut u32) as usize as u32,
        (source_word.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    );
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut pair_source = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut pair_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80248990,
        (this + 0x0C) as u32,
        scratch.as_mut_ptr() as usize as u32,
        (pair_spec.as_mut_ptr() as *mut u32) as usize as u32,
        (pair_source.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    let mut tail = crate::platform::GuestLocal::new( platform::read_u32(source));
    let accept = platform::read_u32(platform::read_u32(object) as usize + 0x30) as usize;
    if platform::call2(accept, object as u32, (tail.as_mut_ptr() as *mut u32) as usize as u32) == 0 {
        let discard = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(discard, this as u32);
        return 0;
    }
    platform::call3(W93_FN_80165AE0, this as u32, spec as u32, 4);
    1
}

/// Builds one layout group and commits its first element.
///
/// The key pair is folded by [`fn_80165820`] into the caller's output word,
/// [`fn_80167fa0`] validates it, and `0x80243D20` creates the child header at
/// `+0x04`. The child's `+0x30` slot accepts the source word; on success the
/// "commit group" twin `0x80165AE0` is called.
///
/// # Original
/// - Ghidra: `FUN_801658a0` at `0x801658a0` (284 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801658a0_FUN_801658a0.c`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: 36-byte output block, or `0` for a stack block.
///
/// # Returns
/// `1` on success, `0` when validation or the child's `+0x30` slot fails.
///
/// # Safety
/// Same requirements as [`fn_801650d0`].
///
/// # Confidence
/// medium - the fold call and every store are literal.
pub unsafe fn fn_801658a0(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 0x24]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let value = fn_80165820(
        (source_word.as_mut_ptr() as *mut u32) as usize,
        (spec_word.as_mut_ptr() as *mut u32) as usize,
        0,
    );
    platform::write_u32(out, value);
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut pair_source = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut pair_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80243D20,
        (this + 0x0C) as u32,
        0,
        (pair_spec.as_mut_ptr() as *mut u32) as usize as u32,
        (pair_source.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    let mut tail = crate::platform::GuestLocal::new( platform::read_u32(source));
    let accept = platform::read_u32(platform::read_u32(object) as usize + 0x30) as usize;
    if platform::call2(accept, object as u32, (tail.as_mut_ptr() as *mut u32) as usize as u32) == 0 {
        let discard = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(discard, this as u32);
        return 0;
    }
    platform::call3(W93_FN_80165AE0, this as u32, spec as u32, 4);
    1
}

/// Builds a layout group with `count` children and links every child.
///
/// [`fn_80165b50`] folds the key pair, `0x80243B30` creates the group header,
/// `0x801D5540` reserves `count * 0x38` bytes at `+0x28`, and each element is
/// bump-allocated from that slab. Every element is initialised by
/// [`fn_80168d10`], gets the class vtable, and is filled by
/// [`fn_801658a0`]. The first element is linked through `+0x3C`; the rest are
/// re-parented by [`fn_80165a80`].
///
/// # Original
/// - Ghidra: `FUN_80165c30` at `0x80165c30` (572 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165c30_FUN_80165c30.c`
/// - observed: `uVar11 = FUN_802dcd5c(); ... FUN_80168d10();
///   *puVar5 = &DAT_80329530; ... iVar6 = FUN_801658a0(iVar8,...);
///   if (iVar6 == 0) FUN_8016e740(...);`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: one-word output block, or `0` for a stack word.
/// - `count`: number of children to build.
///
/// # Returns
/// `1` when every child was built, `0` on the first failure.
///
/// # Side effects
/// Reserves the element slab at `+0x28`, initialises every element, and frees
/// the `+0x08` handle on failure.
///
/// # Safety
/// `this` must be writable; `count` drives the slab arithmetic and must not
/// exceed the caller's reservation.
///
/// # Confidence
/// medium - the slab size, loop bound and both cast type words are literal;
/// the receiver of `0x80168D10` is supplied from the allocation.
pub unsafe fn fn_80165c30(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
    count: u32,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u32; 1]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let value = fn_80165b50(
        (source_word.as_mut_ptr() as *mut u32) as usize,
        (spec_word.as_mut_ptr() as *mut u32) as usize,
        count,
        0,
    );
    platform::write_u32(out, value);
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut key = crate::platform::GuestLocal::new( platform::read_u32(source));
    let object = platform::call4(
        W93_FN_80243B30,
        (this + 0x0C) as u32,
        0,
        (key.as_mut_ptr() as *mut u32) as usize as u32,
        count,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    let size = count.wrapping_mul(0x38).wrapping_add(0x1F) & !0x1F;
    let table = platform::call2(W93_FN_801D5540, (this + 0x0C) as u32, size);
    platform::write_u32(this + 0x28, table);
    let group = w78_cast_to(platform::read_u32(this + 4) as usize, W93_TYPE_802F8588);
    let mut cursor = platform::read_u32(this + 0x28) as usize;
    let mut index = 0u32;
    while index < count {
        let element = platform::call2(W93_FN_802B9220, 0x38, cursor as u32) as usize;
        if element != 0 {
            // inferred: receiver - the block just taken from the slab.
            fn_80168d10(element);
            platform::write_u32(element, W93_VT_80329530 as u32);
        }
        let mut child_source = crate::platform::GuestLocal::new( platform::read_u32(source));
        let mut child_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
        let ok = fn_801658a0(
            cursor,
            (child_source.as_mut_ptr() as *mut u32) as usize,
            (child_spec.as_mut_ptr() as *mut u32) as usize,
            this + 0x0C,
            0,
        );
        if ok == 0 {
            fn_8016e740(platform::read_u32(this + 8) as usize);
            return 0;
        }
        if index == 0 {
            let child = w78_cast_to(platform::read_u32(cursor + 4) as usize, W93_TYPE_802F85A8);
            let link = platform::read_u32(platform::read_u32(group) as usize + 0x3C) as usize;
            platform::call3(link, group as u32, 0, child as u32);
        } else {
            fn_80165a80(cursor);
        }
        cursor = cursor.wrapping_add(0x38);
        index = index.wrapping_add(1);
    }
    1
}

/// Commits one child of an existing layout group.
///
/// The group's `+0x04` header is cast to the class type, its `+0x40` slot is
/// asked to prepare `index`, [`fn_801659c0`] fills the `index`th element, and
/// the child header is linked back through `+0x3C`.
///
/// # Original
/// - Ghidra: `FUN_80165fd0` at `0x80165fd0` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165fd0_FUN_80165fd0.c`
/// - observed: `FUN_801659c0(*(int *)(iVar3 + 0x28) + param_4 * 0x38,
///   (int)uVar6,local_30,param_5);`
///
/// # Arguments
/// - `this`: layout group (the prologue's high word).
/// - `param_2`: key word passed on to [`fn_801659c0`].
/// - `spec`: specification word copied into the one-word block.
/// - `index`: child index.
/// - `param_5`: forwarded to [`fn_801659c0`].
///
/// # Safety
/// The group header at `+0x04` must be castable and `index` must be below the
/// element count reserved by [`fn_80165c30`].
///
/// # Confidence
/// medium - the cast type words and slot indices are literal; a failed cast is
/// dereferenced, exactly as in the original.
pub unsafe fn fn_80165fd0(
    this: usize,
    param_2: u32,
    spec: usize,
    index: u32,
    param_5: u32,
) {
    let group = w78_cast_to(platform::read_u32(this + 4) as usize, W93_TYPE_802F8588);
    let prepare = platform::read_u32(platform::read_u32(group) as usize + 0x40) as usize;
    platform::call2(prepare, group as u32, index);
    let mut header = crate::platform::GuestLocal::new( [platform::read_u32(spec)]);
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    fn_801659c0(
        element,
        param_2,
        header.as_mut_ptr() as usize,
        param_5,
    );
    let child = w78_cast_to(platform::read_u32(element + 4) as usize, W93_TYPE_802F85A8);
    let link = platform::read_u32(platform::read_u32(group) as usize + 0x3C) as usize;
    platform::call3(link, group as u32, index, child as u32);
}

/// Second-class twin of [`fn_801658a0`].
///
/// Identical skeleton to [`fn_801658a0`]: [`fn_80166420`] folds the key pair,
/// `0x80245500` creates the child header, and `0x801666E0` commits the group.
/// Only the class constructor and the commit twin differ.
///
/// # Original
/// - Ghidra: `FUN_801664a0` at `0x801664a0` (284 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801664a0_FUN_801664a0.c`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: 36-byte output block, or `0` for a stack block.
///
/// # Returns
/// `1` on success, `0` when validation or the child's `+0x30` slot fails.
///
/// # Safety
/// Same requirements as [`fn_801650d0`].
///
/// # Confidence
/// medium - only the three callee addresses differ from [`fn_801658a0`].
pub unsafe fn fn_801664a0(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 0x24]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let value = fn_80166420(
        (source_word.as_mut_ptr() as *mut u32) as usize,
        (spec_word.as_mut_ptr() as *mut u32) as usize,
        0,
    );
    platform::write_u32(out, value);
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut pair_source = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut pair_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80245500,
        (this + 0x0C) as u32,
        0,
        (pair_spec.as_mut_ptr() as *mut u32) as usize as u32,
        (pair_source.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    let mut tail = crate::platform::GuestLocal::new( platform::read_u32(source));
    let accept = platform::read_u32(platform::read_u32(object) as usize + 0x30) as usize;
    if platform::call2(accept, object as u32, (tail.as_mut_ptr() as *mut u32) as usize as u32) == 0 {
        let discard = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(discard, this as u32);
        return 0;
    }
    platform::call3(W93_FN_801666E0, this as u32, spec as u32, 4);
    1
}

/// Second-class twin of [`fn_80165c30`].
///
/// [`fn_80166750`] folds the key pair, `0x80245300` creates the group header,
/// `0x801D5540` reserves the `0x38`-stride slab, and the elements get the
/// `0x80329560` vtable. Children are built by [`fn_801664a0`] and re-parented
/// by [`fn_80166680`].
///
/// # Original
/// - Ghidra: `FUN_80166830` at `0x80166830` (572 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166830_FUN_80166830.c`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: one-word output block, or `0` for a stack word.
/// - `count`: number of children to build.
///
/// # Returns
/// `1` when every child was built, `0` on the first failure.
///
/// # Safety
/// Same requirements as [`fn_80165c30`].
///
/// # Confidence
/// medium - the skeleton is identical to [`fn_80165c30`]; the class-specific
/// addresses and type words are literal.
pub unsafe fn fn_80166830(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
    count: u32,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u32; 1]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let value = fn_80166750(
        (source_word.as_mut_ptr() as *mut u32) as usize,
        (spec_word.as_mut_ptr() as *mut u32) as usize,
        count,
        0,
    );
    platform::write_u32(out, value);
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut key = crate::platform::GuestLocal::new( platform::read_u32(source));
    let object = platform::call4(
        W93_FN_80245300,
        (this + 0x0C) as u32,
        0,
        (key.as_mut_ptr() as *mut u32) as usize as u32,
        count,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    let size = count.wrapping_mul(0x38).wrapping_add(0x1F) & !0x1F;
    let table = platform::call2(W93_FN_801D5540, (this + 0x0C) as u32, size);
    platform::write_u32(this + 0x28, table);
    let group = w78_cast_to(platform::read_u32(this + 4) as usize, W93_TYPE_802F85F0);
    let mut cursor = platform::read_u32(this + 0x28) as usize;
    let mut index = 0u32;
    while index < count {
        let element = platform::call2(W93_FN_802B9220, 0x38, cursor as u32) as usize;
        if element != 0 {
            fn_80168d10(element);
            platform::write_u32(element, W93_VT_80329560 as u32);
        }
        let mut child_source = crate::platform::GuestLocal::new( platform::read_u32(source));
        let mut child_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
        let ok = fn_801664a0(
            cursor,
            (child_source.as_mut_ptr() as *mut u32) as usize,
            (child_spec.as_mut_ptr() as *mut u32) as usize,
            this + 0x0C,
            0,
        );
        if ok == 0 {
            fn_8016e740(platform::read_u32(this + 8) as usize);
            return 0;
        }
        if index == 0 {
            let child = w78_cast_to(platform::read_u32(cursor + 4) as usize, W93_TYPE_802F8610);
            let link = platform::read_u32(platform::read_u32(group) as usize + 0x3C) as usize;
            platform::call3(link, group as u32, 0, child as u32);
        } else {
            fn_80166680(cursor);
        }
        cursor = cursor.wrapping_add(0x38);
        index = index.wrapping_add(1);
    }
    1
}

/// Second-class twin of [`fn_80165fd0`].
///
/// # Original
/// - Ghidra: `FUN_80166bd0` at `0x80166bd0` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80166bd0_FUN_80166bd0.c`
///
/// # Arguments
/// - `this`: layout group (the prologue's high word).
/// - `param_2`: key word passed on to [`fn_801665c0`].
/// - `spec`: specification word copied into the one-word block.
/// - `index`: child index.
/// - `param_5`: forwarded to [`fn_801665c0`].
///
/// # Safety
/// Same requirements as [`fn_80165fd0`].
///
/// # Confidence
/// medium - only the type words and the child builder differ from
/// [`fn_80165fd0`].
pub unsafe fn fn_80166bd0(
    this: usize,
    param_2: u32,
    spec: usize,
    index: u32,
    param_5: u32,
) {
    let group = w78_cast_to(platform::read_u32(this + 4) as usize, W93_TYPE_802F85F0);
    let prepare = platform::read_u32(platform::read_u32(group) as usize + 0x40) as usize;
    platform::call2(prepare, group as u32, index);
    let mut header = crate::platform::GuestLocal::new( [platform::read_u32(spec)]);
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    fn_801665c0(
        element,
        param_2,
        header.as_mut_ptr() as usize,
        param_5,
    );
    let child = w78_cast_to(platform::read_u32(element + 4) as usize, W93_TYPE_802F8610);
    let link = platform::read_u32(platform::read_u32(group) as usize + 0x3C) as usize;
    platform::call3(link, group as u32, index, child as u32);
}

/// Third-class twin of [`fn_801658a0`].
///
/// [`fn_80167000`] folds the key pair, `0x80246B00` creates the child header,
/// and `0x801672D0` commits the group. Unlike the two earlier twins this one
/// checks the returned header before touching its `+0x30` slot.
///
/// # Original
/// - Ghidra: `FUN_80167080` at `0x80167080` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167080_FUN_80167080.c`
/// - observed: `piVar4 = (int *)FUN_80246b00(...); piVar1[1] = (int)piVar4;
///   if (piVar4 == (int *)0x0) { uVar2 = 0; }`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: 36-byte output block, or `0` for a stack block.
///
/// # Returns
/// `1` on success, `0` on a null header or a rejected key.
///
/// # Safety
/// Same requirements as [`fn_801650d0`].
///
/// # Confidence
/// high - the skeleton and the extra null test are literal in the export.
pub unsafe fn fn_80167080(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 0x24]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let value = fn_80167000(
        (source_word.as_mut_ptr() as *mut u32) as usize,
        (spec_word.as_mut_ptr() as *mut u32) as usize,
        0,
    );
    platform::write_u32(out, value);
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut pair_source = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut pair_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80246B00,
        (this + 0x0C) as u32,
        0,
        (pair_spec.as_mut_ptr() as *mut u32) as usize as u32,
        (pair_source.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    if object == 0 {
        return 0;
    }
    let mut tail = crate::platform::GuestLocal::new( platform::read_u32(source));
    let accept = platform::read_u32(platform::read_u32(object) as usize + 0x30) as usize;
    if platform::call2(accept, object as u32, (tail.as_mut_ptr() as *mut u32) as usize as u32) == 0 {
        let discard = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(discard, this as u32);
        return 0;
    }
    platform::call3(W93_FN_801672D0, this as u32, spec as u32, 4);
    1
}

/// Third-class twin of [`fn_80165c30`].
///
/// [`fn_80167340`] folds the key pair, `0x80246900` creates the group header,
/// `0x801D5540` reserves the slab at `+0x28`, and every element gets the
/// `0x80329590` vtable. Failure frees the `+0x08` handle through both
/// `0x8016E740` and `0x802B8F70`, unlike the earlier twins.
///
/// # Original
/// - Ghidra: `FUN_80167420` at `0x80167420` (636 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167420_FUN_80167420.c`
/// - observed: `iVar4 = FUN_80246900(piVar2 + 3,0,&local_48,param_6);
///   ... if (iVar6 == 0) { FUN_8016e740(piVar2[2]);
///   FUN_802b8f70(piVar2[2],0); }`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: one-word output block, or `0` for a stack word.
/// - `count`: number of children to build.
///
/// # Returns
/// `1` when every child was built, `0` on the first failure.
///
/// # Safety
/// Same requirements as [`fn_80165c30`].
///
/// # Confidence
/// medium - the extra null test around the slab pointer and the double free
/// are literal; the class addresses are literal too.
pub unsafe fn fn_80167420(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
    count: u32,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u32; 1]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let value = fn_80167340(
        (source_word.as_mut_ptr() as *mut u32) as usize,
        (spec_word.as_mut_ptr() as *mut u32) as usize,
        count,
        0,
    );
    platform::write_u32(out, value);
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut key = crate::platform::GuestLocal::new( platform::read_u32(source));
    let object = platform::call4(
        W93_FN_80246900,
        (this + 0x0C) as u32,
        0,
        (key.as_mut_ptr() as *mut u32) as usize as u32,
        count,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    if object == 0 {
        return 0;
    }
    let size = count.wrapping_mul(0x38).wrapping_add(0x1F) & !0x1F;
    let table = platform::call2(W93_FN_801D5540, (this + 0x0C) as u32, size);
    platform::write_u32(this + 0x28, table);
    if table == 0 {
        let discard = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(discard, this as u32);
        return 0;
    }
    let group = w78_cast_to(platform::read_u32(this + 4) as usize, W93_TYPE_802F8658);
    let mut cursor = platform::read_u32(this + 0x28) as usize;
    let mut index = 0u32;
    while index < count {
        let element = platform::call2(W93_FN_802B9220, 0x38, cursor as u32) as usize;
        if element != 0 {
            fn_80168d10(element);
            platform::write_u32(element, W93_VT_80329590 as u32);
        }
        let mut child_source = crate::platform::GuestLocal::new( platform::read_u32(source));
        let mut child_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
        let ok = fn_80167080(
            cursor,
            (child_source.as_mut_ptr() as *mut u32) as usize,
            (child_spec.as_mut_ptr() as *mut u32) as usize,
            this + 0x0C,
            0,
        );
        if ok == 0 {
            let handle = platform::read_u32(this + 8) as usize;
            fn_8016e740(handle);
            platform::call2(W93_FN_802B8F70, handle as u32, 0);
            return 0;
        }
        if index == 0 {
            let child = w78_cast_to(platform::read_u32(cursor + 4) as usize, W93_TYPE_802F8678);
            let link = platform::read_u32(platform::read_u32(group) as usize + 0x3C) as usize;
            platform::call3(link, group as u32, 0, child as u32);
        } else {
            fn_80167270(cursor);
        }
        cursor = cursor.wrapping_add(0x38);
        index = index.wrapping_add(1);
    }
    1
}

/// Third-class twin of [`fn_80165fd0`].
///
/// # Original
/// - Ghidra: `FUN_80167800` at `0x80167800` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167800_FUN_80167800.c`
///
/// # Arguments
/// - `this`: layout group (the prologue's high word).
/// - `param_2`: key word passed on to [`fn_801671b0`].
/// - `spec`: specification word copied into the one-word block.
/// - `index`: child index.
/// - `param_5`: forwarded to [`fn_801671b0`].
///
/// # Safety
/// Same requirements as [`fn_80165fd0`].
///
/// # Confidence
/// medium - only the type words and the child builder differ from
/// [`fn_80165fd0`].
pub unsafe fn fn_80167800(
    this: usize,
    param_2: u32,
    spec: usize,
    index: u32,
    param_5: u32,
) {
    let group = w78_cast_to(platform::read_u32(this + 4) as usize, W93_TYPE_802F8658);
    let prepare = platform::read_u32(platform::read_u32(group) as usize + 0x40) as usize;
    platform::call2(prepare, group as u32, index);
    let mut header = crate::platform::GuestLocal::new( [platform::read_u32(spec)]);
    let element = platform::read_u32(this + 0x28) as usize + index as usize * 0x38;
    fn_801671b0(
        element,
        param_2,
        header.as_mut_ptr() as usize,
        param_5,
    );
    let child = w78_cast_to(platform::read_u32(element + 4) as usize, W93_TYPE_802F8678);
    let link = platform::read_u32(platform::read_u32(group) as usize + 0x3C) as usize;
    platform::call3(link, group as u32, index, child as u32);
}

/// Four-argument twin of [`fn_801658a0`] using `0x80242C30`.
///
/// Unlike the other three twins the record builder keeps its implicit heap
/// argument at zero and does not take a count; the commit twin is
/// `0x80167E50`.
///
/// # Original
/// - Ghidra: `FUN_80167c40` at `0x80167c40` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167c40_FUN_80167c40.c`
/// - observed: `uVar2 = FUN_80242c30(0,param_5,&local_28,&local_2c);
///   ... piVar4 = (int *)FUN_80242c30(piVar1 + 3,auStack_24,&local_30,&local_34);`
///
/// # Arguments
/// - `this`: owner (the prologue's high word).
/// - `source`: source word (the prologue's low word).
/// - `spec`: specification word.
/// - `heap`: heap, or `0` for `r13 - 0x52B8`.
/// - `out`: 36-byte output block, or `0` for a stack block.
///
/// # Returns
/// `1` on success, `0` when validation or the child's `+0x30` slot fails.
///
/// # Safety
/// Same requirements as [`fn_801650d0`].
///
/// # Confidence
/// medium - both four-argument calls and the commit twin are literal.
pub unsafe fn fn_80167c40(
    this: usize,
    source: usize,
    spec: usize,
    heap: usize,
    out: usize,
) -> u32 {
    let heap = if heap == 0 {
        platform::read_u32(W78_DEFAULT_HEAP) as usize
    } else {
        heap
    };
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 0x24]);
    let out = if out == 0 {
        scratch.as_mut_ptr() as usize
    } else {
        out
    };
    let mut source_word = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    platform::call4(
        W93_FN_80242C30,
        0,
        out as u32,
        (spec_word.as_mut_ptr() as *mut u32) as usize as u32,
        (source_word.as_mut_ptr() as *mut u32) as usize as u32,
    );
    if fn_80167fa0(this, heap, out) == 0 {
        return 0;
    }
    let mut pair_source = crate::platform::GuestLocal::new( platform::read_u32(source));
    let mut pair_spec = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call4(
        W93_FN_80242C30,
        (this + 0x0C) as u32,
        scratch.as_mut_ptr() as usize as u32,
        (pair_spec.as_mut_ptr() as *mut u32) as usize as u32,
        (pair_source.as_mut_ptr() as *mut u32) as usize as u32,
    ) as usize;
    platform::write_u32(this + 4, object as u32);
    let mut tail = crate::platform::GuestLocal::new( platform::read_u32(source));
    let accept = platform::read_u32(platform::read_u32(object) as usize + 0x30) as usize;
    if platform::call2(accept, object as u32, (tail.as_mut_ptr() as *mut u32) as usize as u32) == 0 {
        let discard = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(discard, this as u32);
        return 0;
    }
    platform::call3(W93_FN_80167E50, this as u32, spec as u32, 4);
    1
}

/// One-element builder used by [`fn_80165fd0`].
///
/// The group's `+0x04` header is prepared by [`fn_80165a80`], the key is
/// folded by [`fn_80168400`], `0x80243D20` creates the child header, and the
/// `0x80165AE0` twin commits the element.
///
/// # Original
/// - Ghidra: `FUN_801659c0` at `0x801659c0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801659c0_FUN_801659c0.c`
/// - observed: `FUN_80165a80(); local_24 = FUN_80168400(param_2);
///   ... FUN_80165ae0(param_1,param_3,param_4);`
///
/// # Arguments
/// - `this`: element slot.
/// - `param_2`: key word.
/// - `spec`: specification word copied into a one-word block.
/// - `param_4`: forwarded to `0x80165AE0`.
///
/// # Side effects
/// Writes the child header at `+0x04` and calls its `+0x30` slot.
///
/// # Safety
/// `this` must be a writable element slot; `spec` must be readable.
///
/// # Confidence
/// medium - the receiver of the first call is elided in the export and is
/// supplied from `this` (`// inferred:`).
pub unsafe fn fn_801659c0(this: usize, param_2: u32, spec: usize, param_4: u32) {
    // inferred: receiver - the export shows `FUN_80165a80()` with no argument.
    fn_80165a80(this);
    let mut key = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let mut word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80243D20,
        (this + 0x0C) as u32,
        0,
        (word.as_mut_ptr() as *mut u32) as usize as u32,
        (key.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    );
    platform::write_u32(this + 4, object);
    let mut tail = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let accept = platform::read_u32(platform::read_u32(object as usize) as usize + 0x30) as usize;
    platform::call2(accept, object, (tail.as_mut_ptr() as *mut u32) as usize as u32);
    platform::call3(W93_FN_80165AE0, this as u32, spec as u32, param_4);
}

/// Second-class twin of [`fn_801659c0`].
///
/// # Original
/// - Ghidra: `FUN_801665c0` at `0x801665c0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801665c0_FUN_801665c0.c`
///
/// # Arguments
/// - `this`: element slot.
/// - `param_2`: key word.
/// - `spec`: specification word copied into a one-word block.
/// - `param_4`: forwarded to `0x801666E0`.
///
/// # Safety
/// Same requirements as [`fn_801659c0`].
///
/// # Confidence
/// medium - only the two callee addresses differ from [`fn_801659c0`].
pub unsafe fn fn_801665c0(this: usize, param_2: u32, spec: usize, param_4: u32) {
    // inferred: receiver - the export shows `FUN_80166680()` with no argument.
    fn_80166680(this);
    let mut key = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let mut word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80245500,
        (this + 0x0C) as u32,
        0,
        (word.as_mut_ptr() as *mut u32) as usize as u32,
        (key.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    );
    platform::write_u32(this + 4, object);
    let mut tail = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let accept = platform::read_u32(platform::read_u32(object as usize) as usize + 0x30) as usize;
    platform::call2(accept, object, (tail.as_mut_ptr() as *mut u32) as usize as u32);
    platform::call3(W93_FN_801666E0, this as u32, spec as u32, param_4);
}

/// Third-class twin of [`fn_801659c0`].
///
/// # Original
/// - Ghidra: `FUN_801671b0` at `0x801671b0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801671b0_FUN_801671b0.c`
///
/// # Arguments
/// - `this`: element slot.
/// - `param_2`: key word.
/// - `spec`: specification word copied into a one-word block.
/// - `param_4`: forwarded to `0x801672D0`.
///
/// # Safety
/// Same requirements as [`fn_801659c0`].
///
/// # Confidence
/// medium - only the two callee addresses differ from [`fn_801659c0`].
pub unsafe fn fn_801671b0(this: usize, param_2: u32, spec: usize, param_4: u32) {
    // inferred: receiver - the export shows `FUN_80167270()` with no argument.
    fn_80167270(this);
    let mut key = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let mut word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let object = platform::call5(
        W93_FN_80246B00,
        (this + 0x0C) as u32,
        0,
        (word.as_mut_ptr() as *mut u32) as usize as u32,
        (key.as_mut_ptr() as *mut u32) as usize as u32,
        0,
    );
    platform::write_u32(this + 4, object);
    let mut tail = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let accept = platform::read_u32(platform::read_u32(object as usize) as usize + 0x30) as usize;
    platform::call2(accept, object, (tail.as_mut_ptr() as *mut u32) as usize as u32);
    platform::call3(W93_FN_801672D0, this as u32, spec as u32, param_4);
}

/// Empty leaf reached through the layout dispatch table.
///
/// The original body is a bare `return`; it exists so the table keeps a
/// uniform entry for this slot.
///
/// # Original
/// - Ghidra: `FUN_8016c820` at `0x8016C820` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016c820_FUN_8016c820.c`
///
/// # Confidence
/// high - the export is literally `{ return; }`.
pub unsafe fn fn_8016c820() {}

/// Installs the control object and reports the previous state.
///
/// `0x8024D5E0` sets the text mode, [`fn_80164f50`] rebuilds the live page, and
/// the installed control's `+0x20` slot is ticked. When a control is already
/// installed, its `+0x10` state is read; states `2` and `3` veto the swap.
/// Otherwise the new object is stored in `r13 - 0x5240` and its `+0x0C` slot
/// is told the previous state (`0` or `1`).
///
/// # Original
/// - Ghidra: `FUN_8016dae0` at `0x8016dae0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dae0_FUN_8016dae0.c`
/// - observed: `iVar3 = (**(code **)(**(int **)(r13 + -0x5240) + 0x10))();
///   uVar2 = countLeadingZeros(iVar3 + -2);`
///
/// # Arguments
/// - `this`: candidate control object.
///
/// # Returns
/// `1` when the object was installed, `0` when an installed control is in
/// state `2` or `3`.
///
/// # Side effects
/// Writes `r13 - 0x5240` and calls two virtual slots.
///
/// # Safety
/// `this` must be a live control; an installed control must be readable.
///
/// # Confidence
/// medium - the state test and both stores are literal; the two `+0x10`
/// receivers are elided in the export and are supplied from the installed
/// object (`// inferred:`).
pub unsafe fn fn_8016dae0(this: usize) -> u32 {
    let installed = platform::read_u32(W93_G_CONTROL) as usize;
    if installed != 0 {
        let mut blocked = true;
        let state_slot = platform::read_u32(platform::read_u32(installed) as usize + 0x10) as usize;
        // inferred: receiver - the installed object.
        let state = platform::call1(state_slot, installed as u32);
        if state != 2 {
            let state = platform::call1(state_slot, installed as u32);
            if state != 3 {
                blocked = false;
            }
        }
        if blocked {
            return 0;
        }
    }
    let previous = if platform::read_u32(W93_G_CONTROL) == 0 {
        let tick = platform::read_u32(platform::read_u32(this) as usize + 0x10) as usize;
        platform::call1(tick, this as u32)
    } else {
        let object = platform::read_u32(W93_G_CONTROL) as usize;
        let state_slot = platform::read_u32(platform::read_u32(object) as usize + 0x10) as usize;
        platform::call1(state_slot, object as u32)
    };
    platform::write_u32(W93_G_CONTROL, this as u32);
    if previous == 0 || previous == 1 {
        let apply = platform::read_u32(platform::read_u32(this) as usize + 0x0C) as usize;
        platform::call2(apply, this as u32, previous);
    }
    1
}

/// Resets the text mode and ticks the installed control page.
///
/// `0x8024D5E0` switches the renderer to mode `0x7FF`, [`fn_80164f50`] clears
/// the live page, and the installed control's `+0x20` slot runs with no
/// arguments.
///
/// # Original
/// - Ghidra: `FUN_8016daa0` at `0x8016daa0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016daa0_FUN_8016daa0.c`
/// - observed: `FUN_8024d5e0(0x7ff); FUN_80164f50();
///   (**(code **)(**(int **)(unaff_r13 + -0x5240) + 0x20))();`
///
/// # Side effects
/// Changes the render mode and ticks the control page.
///
/// # Safety
/// `r13 - 0x5240` must hold a live control.
///
/// # Confidence
/// medium - the two direct calls are literal; the receiver of the `+0x20`
/// slot is the installed object (`// inferred:`).
pub unsafe fn fn_8016daa0() {
    platform::call1(W93_FN_8024D5E0, 0x7FF);
    fn_80164f50();
    let control = platform::read_u32(W93_G_CONTROL) as usize;
    let tick = platform::read_u32(platform::read_u32(control) as usize + 0x20) as usize;
    // inferred: receiver - the object whose vtable supplies `+0x20`.
    platform::call1(tick, control as u32);
}

/// Clears and re-keys one element of a layout group.
///
/// The element's own `+0x0C` slot is asked for its current key, the value is
/// copied by `0x80168520`, the `+0x04` header's `+0x34` slot is reset, and the
/// `+0x08` descriptor is released. `0x80242C30` then creates a replacement
/// header from a new key pair; the header's `+0x30` slot accepts it and
/// `0x80167E50` commits the element.
///
/// # Original
/// - Ghidra: `FUN_80167d60` at `0x80167d60` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167d60_FUN_80167d60.c`
/// - observed: `uVar1 = (**(code **)(*param_1 + 0xc))();
///   FUN_80168520(param_2,uVar1); (**(code **)(*(int *)param_1[1] + 0x34))();`
///
/// # Arguments
/// - `this`: element slot; `+0x04` header, `+0x08` descriptor.
/// - `param_2`: key word.
/// - `spec`: specification word.
/// - `param_4`: forwarded to `0x80167E50`.
///
/// # Safety
/// `this` must be live; both virtual receivers are elided in the export and
/// are supplied from the fields they belong to.
///
/// # Confidence
/// medium - the four calls and every store are literal; the two elided
/// receivers are inferred from the owning fields.
pub unsafe fn fn_80167d60(this: usize, param_2: u32, spec: usize, param_4: u32) {
    let vtable = platform::read_u32(this) as usize;
    let read = platform::read_u32(vtable + 0x0C) as usize;
    // inferred: receiver - the element itself.
    let value = platform::call1(read, this as u32);
    platform::call2(W93_FN_80168520, param_2, value);
    let header = platform::read_u32(this + 4) as usize;
    let reset = platform::read_u32(platform::read_u32(header) as usize + 0x34) as usize;
    // inferred: receiver - the `+0x04` header.
    platform::call1(reset, header as u32);
    platform::call2(W93_FN_802B99C0, platform::read_u32(this + 8), 3);
    let mut key_word = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let mut spec_word = crate::platform::GuestLocal::new( platform::read_u32(spec));
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 12]);
    let object = platform::call4(
        W93_FN_80242C30,
        (this + 0x0C) as u32,
        scratch.as_mut_ptr() as usize as u32,
        (spec_word.as_mut_ptr() as *mut u32) as usize as u32,
        (key_word.as_mut_ptr() as *mut u32) as usize as u32,
    );
    platform::write_u32(this + 4, object);
    let mut tail = crate::platform::GuestLocal::new( fn_80168400(param_2 as usize));
    let installed = platform::read_u32(this + 4) as usize;
    let accept = platform::read_u32(platform::read_u32(installed) as usize + 0x30) as usize;
    platform::call2(accept, installed as u32, (tail.as_mut_ptr() as *mut u32) as usize as u32);
    platform::call3(W93_FN_80167E50, this as u32, spec as u32, param_4);
}

/// Detaches one record from the list used by [`fn_80161f60`].
///
/// `0x80161620` unlinks the node at `+0x10` from the actor tree,
/// `0x80162CF0` resolves its group index, and the node at `+0x44`/`+0x24` is
/// unlinked from the group table and the second global list. The optional
/// `+0x5C` object and `+0x50` descriptor are released, then the node's own
/// `+0x60` vtable slot is asked to delete it.
///
/// # Original
/// - Ghidra: `FUN_80161f60` at `0x80161f60` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161f60_FUN_80161f60.c`
/// - observed: `if (param_2 == 2) { FUN_80161620(&DAT_80377a10,param_1 + 0x10);
///   iVar1 = FUN_80162cf0(param_1 + 0x10); ... }`
///
/// # Arguments
/// - `this`: record node.
/// - `mode`: only `2` performs the removal.
///
/// # Side effects
/// Unlinks three intrusive nodes and calls the node's deleting slot.
///
/// # Safety
/// `this` must be live and its intrusive links must be intact.
///
/// # Confidence
/// medium - the three unlink calls and the guard are literal; the two elided
/// receivers are supplied from the fields they guard.
pub unsafe fn fn_80161f60(this: usize, mode: u32) {
    if mode != 2 {
        return;
    }
    fn_80161620(W89_ACTOR_ROOT, this + 0x10);
    let index = platform::call1(W89_FN_80162CF0, (this + 0x10) as u32) as usize;
    fn_80160f60(W89_GROUP_TABLE + index * 8, this + 0x44);
    fn_80160f60(W93_LIST_80377A5C, this + 0x24);
    let owner = platform::read_u32(this + 0x5C) as usize;
    if owner != 0 {
        let release = platform::read_u32(platform::read_u32(owner) as usize + 0x1C) as usize;
        // inferred: receiver - the object stored at `+0x5C`.
        platform::call1(release, owner as u32);
    }
    let descriptor = platform::read_u32(this + 0x50);
    if descriptor != 0 {
        // inferred: receiver - the descriptor guarded by `+0x50`.
        fn_80161920(descriptor as usize);
    }
    if this != 0 {
        let discard = platform::read_u32(platform::read_u32(this + 0x60) as usize + 0x48) as usize;
        platform::call2(discard, this as u32, 1);
    }
}

/// Registers or updates the record attached to an actor.
///
/// A record already marked at `+0x11C` is first matched by name; when the
/// owning actor resolves to a drawable record and the `+0x128` word passes the
/// threshold, `0x80287C10` submits a copy of the caller's three-word packet.
/// Otherwise the record is created by `0x8016C960` and inserted at the tail of
/// the global list. The `0x48`-byte draw block is then built and handed to the
/// record's `+0x8C` slot before its `+0x94` slot completes the frame.
///
/// # Original
/// - Ghidra: `FUN_8016d640` at `0x8016d640` (464 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016d640_FUN_8016d640.c`
/// - observed: `uVar4 = FUN_802dcd70(); ... if (DAT_80377c34 == &DAT_80377b1c)
///   { piVar1[0x46] = ...; DAT_80377c34 = piVar1; } ... DAT_80377c38 += 1;`
///
/// # Arguments
/// - `this`: record object (the prologue's high word).
/// - `param_2`: actor key (the prologue's low word).
/// - `param_3`: forwarded to `0x8016C960`.
/// - `spec`: three-word packet copied before submission.
/// - `param_5`, `param_6`: forwarded to `0x8016C9E0`.
///
/// # Returns
/// `1` when the record is active, `0` when the owning actor has no drawable
/// record or the creation failed.
///
/// # Safety
/// `this` must be live; `spec` must be readable for twelve bytes.
///
/// # Confidence
/// medium - every branch, list write and slot index is literal; the three list
/// globals are named from their use as first/last/count.
pub unsafe fn fn_8016d640(
    this: usize,
    param_2: u32,
    param_3: u32,
    spec: usize,
    param_5: u32,
    param_6: u32,
) -> u32 {
    if platform::read_u8(this + 0x11C) != 0 {
        if platform::call1(W89_FN_802E1AB4, (this + 4) as u32) == 0 {
            let ready = platform::read_u32(platform::read_u32(this) as usize + 0xB8) as usize;
            if platform::call1(ready, this as u32) != 0 {
                let record = platform::call1(W93_FN_802D87C0, this as u32) as usize;
                if record == 0 || platform::read_u32(record + 0xB8) == 0 {
                    return 0;
                }
                if platform::read_f32(W93_F32_RECORD_THRESHOLD)
                    <= platform::read_f32(this + 0x128)
                {
                    let mut packet = crate::platform::GuestLocal::new( [
                        platform::read_u32(spec),
                        platform::read_u32(spec + 4),
                        platform::read_u32(spec + 8),
                    ]);
                    platform::call6(
                        W93_FN_80287C10,
                        record as u32,
                        platform::read_u32(record + 0xB8),
                        (this + 0x12C) as u32,
                        0,
                        0,
                        packet.as_mut_ptr() as usize as u32,
                    );
                }
                return 1;
            }
        } else {
            platform::call1(W93_FN_8016D4B0, this as u32);
        }
    }
    if platform::read_u8(this + 0x11C) == 0 {
        if platform::call3(W93_FN_8016C960, this as u32, param_2, param_3) == 0 {
            return 0;
        }
        let head = W93_LIST_80377B1C;
        if platform::read_u32(W93_G_LIST_FIRST) == head as u32 {
            platform::write_u32(this + 0x118, head as u32);
            platform::write_u32(this + 0x114, head as u32);
            platform::write_u32(W93_G_LIST_FIRST, this as u32);
        } else {
            platform::write_u32(this + 0x114, platform::read_u32(W93_G_LIST_LAST));
            platform::write_u32(this + 0x118, head as u32);
            let last = platform::read_u32(W93_G_LIST_LAST) as usize;
            platform::write_u32(last + 0x118, this as u32);
        }
        platform::write_u32(
            W93_G_LIST_COUNT,
            platform::read_u32(W93_G_LIST_COUNT).wrapping_add(1),
        );
        platform::write_u32(W93_G_LIST_LAST, this as u32);
        platform::write_u8(this + 0x11C, 1);
    }
    platform::write_u8(this + 0x11D, 1);
    let mut block = crate::platform::GuestLocal::new( [0u8; 0x48]);
    platform::call4(
        W93_FN_8016C9E0,
        spec as u32,
        param_5,
        param_6,
        block.as_mut_ptr() as usize as u32,
    );
    let draw = platform::read_u32(platform::read_u32(this) as usize + 0x8C) as usize;
    platform::call2(draw, this as u32, block.as_mut_ptr() as usize as u32);
    let finish = platform::read_u32(platform::read_u32(this) as usize + 0x94) as usize;
    platform::call1(finish, this as u32);
    1
}

// ===========================================================================
// Angle, transform and render-state leaves (wave 97)
//
// Wave 93 documented these as skips because every one of them touches the
// floating-point register file. That is a limitation of
// [`crate::platform`], not of the evidence: `call0..call7` only fill integer
// registers, so a callee that reads `f1..f3` cannot be reached through them.
//
// Four rules keep this pass honest:
//
// * Float arguments and float results use the same local typed-transmute
//   helpers `src/engine/dsys.rs` and `src/audio/ax.rs` already use for the
//   PowerPC EABI, whose float and integer register files are separate. The
//   helper docs state the exact argument shape of each callee.
// * The `r2` float constants are resolved, not guessed: `r2` is
//   [`SDA2_BASE`] = `0x80433080`, and every `r2 - 0xNNNN` word below was read
//   back from `work/dol/main.elf` at `SDA2_BASE - 0xNNNN`. The comment on each
//   constant states the address and the `f32`/`f64` value found there.
// * The paired-single `GQR` blocks Ghidra prints around these bodies are
//   decompiler artefacts: `unaff_GQR0`/`unaff_GQR3` are never assigned by the
//   function, so the `ldexpf` branches are dead. They are not reproduced. Only
//   [`fn_8016ebf0`], [`fn_8016ed10`] and [`fn_8016ee30`] really read a
//   quantised value; their `// inferred:` note says which width is assumed.
// * Where the decompiler drops the receiver of an indirect call, the object is
//   supplied only when the neighbouring slots name the same object; that is
//   marked `// inferred:`.
// ===========================================================================

/// `r2 - 0x5480` (`0x8042DC00`) - `FLT_EPSILON` (`1.1920929e-7`), the
/// "vectors already coincide" threshold of [`fn_80160d10`].
const W97_F32_EPS_5480: usize = SDA2_BASE - 0x5480;

/// `r2 - 0x547C` (`0x8042DC04`) - `0.0f`, the "no step requested" sentinel of
/// [`fn_80160d10`].
const W97_F32_ZERO_547C: usize = SDA2_BASE - 0x547C;

/// `r2 - 0x5420` (`0x8042DC60`) - the `f64` `2^52 + 2^31`, the bias of the
/// "unsigned word to `f64`" idiom. The same eight bytes are duplicated at
/// `r2 - 0x5410`, `-0x5400`, `-0x53F0` and `-0x53E0`; all five read back
/// identical in `main.elf`.
const W97_F64_UNSIGNED_BIAS: usize = SDA2_BASE - 0x5420;

/// `r2 - 0x5428` (`0x8042DC58`) - `1.0f`, first of the two constants every
/// element-commit twin forwards to [`fn_80168f10`]. Duplicated at `-0x5418`,
/// `-0x5408`, `-0x53F8` and `-0x53E8`.
const W97_F32_ONE_5428: usize = SDA2_BASE - 0x5428;

/// `r2 - 0x5424` (`0x8042DC5C`) - `-1.0f`, the second forwarded constant.
/// Duplicated at `-0x5414`, `-0x5404`, `-0x53F4` and `-0x53E4`.
const W97_F32_MINUS_ONE_5424: usize = SDA2_BASE - 0x5424;

/// `r2 - 0x53D0` (`0x8042DCB0`) - `0.0f`, the "no minimum" bound of the
/// render-state record.
const W97_F32_ZERO_53D0: usize = SDA2_BASE - 0x53D0;

/// `r2 - 0x53C8` (`0x8042DCB8`) - `1.0f`, the unit the render-state record
/// subtracts from its value.
const W97_F32_ONE_53C8: usize = SDA2_BASE - 0x53C8;

/// `r2 - 0x53C0` (`0x8042DCC0`) - `0.0f`, the fill colour of the texture
/// builders.
const W97_F32_ZERO_53C0: usize = SDA2_BASE - 0x53C0;

/// `r2 - 0x5360` (`0x8042DD20`) - `0.00390625f` (`1/256`), the fixed-point
/// scale the three axis-matrix builders apply to their angle.
const W97_F32_ANGLE_SCALE_5360: usize = SDA2_BASE - 0x5360;

/// `r2 - 0x535C` (`0x8042DD24`) - `1.0f`, the matrix diagonal entry.
const W97_F32_ONE_535C: usize = SDA2_BASE - 0x535C;

/// `r2 - 0x5358` (`0x8042DD28`) - `0.0f`, the matrix off-diagonal entry.
const W97_F32_ZERO_5358: usize = SDA2_BASE - 0x5358;

/// `r2 - 0x3470` (`0x8042FC10`) - word handed to `0x801C8920` by
/// [`fn_80163220`]. The slot sits in BSS, so the shipped image has no value to
/// read; `// unknown:` the word's meaning.
const W97_WORD_COLOUR_3470: usize = SDA2_BASE - 0x3470;

/// `r13 - 0x5200` (`0x8042A480`) - global whose first word is a display object
/// (the `+0x19`/`+0x1A`/`0x32` fields read by [`fn_80169440`]).
const W97_GLOBAL_DISPLAY_5200: usize = SDA_BASE - 0x5200;

/// `r13 - 0x52AC` (`0x8042A3D4`) - text-object table read by [`fn_80164770`].
const W97_GLOBAL_TEXT_TABLE_A: usize = SDA_BASE - 0x52AC;
/// `r13 - 0x52B0` (`0x8042A3D0`) - entry count of that table.
const W97_GLOBAL_TEXT_COUNT_A: usize = SDA_BASE - 0x52B0;
/// `r13 - 0x52A8` (`0x8042A3D8`) - entry count of the second table.
const W97_GLOBAL_TEXT_COUNT_B: usize = SDA_BASE - 0x52A8;
/// `r13 - 0x52A4` (`0x8042A3DC`) - second text-object table.
const W97_GLOBAL_TEXT_TABLE_B: usize = SDA_BASE - 0x52A4;

/// `0x801611C0` - `atan2`-style angle of a two-float direction.
const W97_FN_801611C0: usize = 0x8016_11C0;
/// `0x801637E0` - store one `f32` at `obj + 0x10`.
const W97_FN_801637E0: usize = 0x8016_37E0;
/// `0x80164990` - bind a text object to a slot.
const W97_FN_80164990: usize = 0x8016_4990;
/// `0x801680A0` - run the `+0x1C` slot of the object at `this + 4`.
const W97_FN_801680A0: usize = 0x8016_80A0;
/// `0x801C14C0` - length of a three-float vector.
const W97_FN_801C14C0: usize = 0x801C_14C0;
/// `0x801C5860` - GX pipeline flush with one argument.
const W97_FN_801C5860: usize = 0x801C_5860;
/// `0x801C8E70` - GX blend/alpha state.
const W97_FN_801C8E70: usize = 0x801C_8E70;
/// `0x801C8920` - GX clear with four floats and a colour word.
const W97_FN_801C8920: usize = 0x801C_8920;
/// `0x801C5960` - present a texture with a source rectangle.
const W97_FN_801C5960: usize = 0x801C_5960;
/// `0x801C59D0` - set the draw rectangle of a texture.
const W97_FN_801C59D0: usize = 0x801C_59D0;
/// `0x801C5F40` - set the blend/compare registers.
const W97_FN_801C5F40: usize = 0x801C_5F40;
/// `0x801C6290` - bind a texture with a wrap/filter byte.
const W97_FN_801C6290: usize = 0x801C_6290;
/// `0x801C6BA0` - create a texture of `w x h` with a format word.
const W97_FN_801C6BA0: usize = 0x801C_6BA0;
/// `0x801C6D90` - configure the texture `+4 + 0x14` block.
const W97_FN_801C6D90: usize = 0x801C_6D90;
/// `0x801C6FF0` - draw the configured texture block.
const W97_FN_801C6FF0: usize = 0x801C_6FF0;
/// `0x802B50B0` - first accessor of the entry's `+4` sub-object.
const W97_FN_802B50B0: usize = 0x802B_50B0;
/// `0x802B50A0` - second accessor of the entry's `+4` sub-object.
const W97_FN_802B50A0: usize = 0x802B_50A0;
/// `0x802B9170` - switch the current heap and return the previous one.
const W97_FN_802B9170: usize = 0x802B_9170;
/// `0x802B9240` - allocate from a named heap.
const W97_FN_802B9240: usize = 0x802B_9240;
/// `0x802BE470` - square root of one `f64`.
const W97_FN_802BE470: usize = 0x802B_E470;
/// `0x802C81F0` - construct a text object in a caller-supplied buffer.
const W97_FN_802C81F0: usize = 0x802C_81F0;
/// `0x80237C40` - cosine of an angle in degrees.
const W97_FN_80237C40: usize = 0x8023_7C40;
/// `0x80237BD0` - sine of an angle in degrees.
const W97_FN_80237BD0: usize = 0x8023_7BD0;

// ---------------------------------------------------------------------------
// Local float-ABI call helpers
// ---------------------------------------------------------------------------

/// Calls a routine shaped `(f32, f32) -> u32`.
///
/// # Safety
/// `addr` must accept two floats and return a word in `r3`.
unsafe fn w97_call_f32x2_u32(addr: usize, a: f32, b: f32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f32, f32) -> u32);
    f(a, b)
}

/// Calls a routine shaped `(f32, u32) -> u32`.
///
/// # Safety
/// `addr` must accept one float and one pointer-shaped word.
unsafe fn w97_call_f32_u32(addr: usize, value: f32, first: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f32, u32) -> u32);
    f(value, first)
}

/// Calls a routine shaped `(u32) -> f32` and keeps the float in `f1`.
///
/// # Safety
/// `addr` must accept one pointer-shaped word and return a float.
unsafe fn w97_call_u32_f32(addr: usize, first: u32) -> f32 {
    let f = crate::platform::guest_fn!(addr, fn(u32) -> f32);
    f(first)
}

/// Calls a routine shaped `(pointer) -> f32` (the vector length).
///
/// # Safety
/// `first` must point at three readable floats.
unsafe fn w97_call_ptr_f32(addr: usize, first: u32) -> f32 {
    let f = crate::platform::guest_fn!(addr, fn(u32) -> f32);
    f(first)
}

/// Calls a routine shaped `(f64) -> f32`.
///
/// # Safety
/// `addr` must accept one double and return a float.
unsafe fn w97_call_f64_f32(addr: usize, value: f64) -> f32 {
    let f = crate::platform::guest_fn!(addr, fn(f64) -> f32);
    f(value)
}

/// Calls a routine shaped `(f64) -> f64`.
///
/// # Safety
/// `addr` must accept one double and return one.
unsafe fn w97_call_f64_f64(addr: usize, value: f64) -> f64 {
    let f = crate::platform::guest_fn!(addr, fn(f64) -> f64);
    f(value)
}

/// Calls a routine shaped `(f32, f32, f32, u32, u32) -> u32`.
///
/// # Safety
/// `addr` must accept the three floats in `f1..f3` and two integer arguments.
unsafe fn w97_call_f32x3_u32x2(
    addr: usize,
    a: f32,
    b: f32,
    c: f32,
    first: u32,
    second: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f32, f32, f32, u32, u32) -> u32);
    f(a, b, c, first, second)
}

/// Calls a routine shaped `(f32, f32, f32, u32 x 6) -> u32`.
///
/// # Safety
/// `addr` must accept the three floats in `f1..f3` and six integer arguments.
unsafe fn w97_call_f32x3_u32x6(
    addr: usize,
    a: f32,
    b: f32,
    c: f32,
    first: u32,
    second: u32,
    third: u32,
    fourth: u32,
    fifth: u32,
    sixth: u32,
) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f32, f32, f32, u32, u32, u32, u32, u32, u32) -> u32);
    f(a, b, c, first, second, third, fourth, fifth, sixth)
}

/// Calls a routine shaped `(f32 x 4, u32, pointer) -> u32`.
///
/// # Safety
/// `addr` must accept four floats in `f1..f4` followed by two words.
unsafe fn w97_call_f32x4_u32_ptr(addr: usize, a: f32, b: f32, c: f32, d: f32, first: u32, second: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f32, f32, f32, f32, u32, u32) -> u32);
    f(a, b, c, d, first, second)
}

/// Doubles the unsigned halfword at `addr` exactly as the export's
/// `CONCAT44(0x43300000, value ^ 0x80000000) - bias` pair does.
///
/// # Safety
/// `addr` must be a readable halfword.
unsafe fn w97_u16_to_f64(addr: usize) -> f64 {
    // observed: the bias is 2^52 + 2^31, so subtracting it from the tagged
    // double leaves the unsigned integer value.
    platform::read_u16(addr) as f64
}

/// Moves `current` toward `target` by at most `step` and reports arrival.
///
/// When `step` is zero the three words are only compared. Otherwise the
/// `current - target` vector's length is measured by `0x801C14C0`; a length at
/// or below `FLT_EPSILON`, or one already within `step`, snaps `current` onto
/// `target`. Any other case subtracts `step / length` of the delta.
///
/// # Original
/// - Ghidra: `FUN_80160d10` at `0x80160D10` (368 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160d10_FUN_80160d10.c`
/// - observed: `if ((float)ABS(dVar5) < *(float *)(unaff_r2 + -0x5480)) ||
///   (dVar5 <= param_1)) { ... *param_2 = *param_3; ... }` and
///   `fVar1 = (float)(param_1 / dVar5); *param_2 = *param_2 - local_2c * fVar1;`
///
/// # Arguments
/// - `step`: maximum distance to travel this call.
/// - `current`: three `f32` at `+0x00`/`+0x04`/`+0x08`, updated in place.
/// - `target`: the three target floats.
///
/// # Returns
/// `1` when `current` now equals `target`, `0` when it only moved.
///
/// # Side effects
/// Writes up to three floats at `current`.
///
/// # Safety
/// Both pointers must name six readable floats each.
///
/// # Confidence
/// medium - every branch, threshold and store is literal; `0x801C14C0` is
/// inferred to be the vector length because its result is divided into `step`.
pub unsafe fn fn_80160d10(step: f32, current: usize, target: usize) -> u32 {
    if step == platform::read_f32(W97_F32_ZERO_547C) {
        let same = platform::read_f32(current) == platform::read_f32(target)
            && platform::read_f32(current + 4) == platform::read_f32(target + 4)
            && platform::read_f32(current + 8) == platform::read_f32(target + 8);
        return same as u32;
    }
    let mut delta = crate::platform::GuestLocal::new( [0.0f32; 3]);
    let mut axis = 0usize;
    while axis < 3 {
        delta[axis] =
            platform::read_f32(current + axis * 4) - platform::read_f32(target + axis * 4);
        axis += 1;
    }
    let distance = w97_call_ptr_f32(W97_FN_801C14C0, delta.as_mut_ptr() as usize as u32);
    if distance.abs() < platform::read_f32(W97_F32_EPS_5480) || distance <= step {
        let mut axis = 0usize;
        while axis < 3 {
            platform::write_f32(current + axis * 4, platform::read_f32(target + axis * 4));
            axis += 1;
        }
        return 1;
    }
    let scale = step / distance;
    let mut axis = 0usize;
    while axis < 3 {
        platform::write_f32(
            current + axis * 4,
            platform::read_f32(current + axis * 4) - delta[axis] * scale,
        );
        axis += 1;
    }
    0
}

/// Reports the horizontal angle from `from` to `to`.
///
/// The two `x`/`z` differences are handed to the `atan2`-style table lookup at
/// `0x801611C0` in `f1`/`f2`.
///
/// # Original
/// - Ghidra: `FUN_80160e80` at `0x80160E80` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160e80_FUN_80160e80.c`
/// - observed: `FUN_801611c0((double)(*param_2 - *param_1),
///   (double)(param_2[2] - param_1[2]));`
///
/// # Arguments
/// - `from`: first position block (three `f32`).
/// - `to`: second position block.
///
/// # Returns
/// Nothing; the angle is whatever `0x801611C0` returns in `r3`, discarded here.
///
/// # Side effects
/// One call to `0x801611C0`.
///
/// # Safety
/// Both blocks must be readable for twelve bytes.
///
/// # Confidence
/// high - the two subtractions and the argument order are literal.
pub unsafe fn fn_80160e80(from: usize, to: usize) {
    let dx = platform::read_f32(to) - platform::read_f32(from);
    let dz = platform::read_f32(to + 8) - platform::read_f32(from + 8);
    w97_call_f32x2_u32(W97_FN_801611C0, dx, dz);
}

/// Reports the vertical angle from `from` to `to`.
///
/// The height difference is the first `f1` argument; the second is the square
/// root of the squared `x`/`z` distance, measured by `0x802BE470`.
///
/// # Original
/// - Ghidra: `FUN_80160ea0` at `0x80160EA0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160ea0_FUN_80160ea0.c`
/// - observed: `uVar3 = FUN_802be470((double)((*param_2 - *param_1) *
///   (*param_2 - *param_1) + (param_2[2] - param_1[2]) *
///   (param_2[2] - param_1[2]))); FUN_801611c0((double)(fVar1 - fVar2),uVar3);`
///
/// # Arguments
/// - `from`: first position block.
/// - `to`: second position block.
///
/// # Returns
/// Nothing; the angle result of `0x801611C0` is discarded.
///
/// # Side effects
/// Calls `0x802BE470` and `0x801611C0`.
///
/// # Safety
/// Both blocks must be readable for twelve bytes.
///
/// # Confidence
/// medium - the arithmetic and the store order are literal; `0x802BE470` is
/// inferred to be the square root from its `sqrt`-shaped result and its use as
/// a distance here and in `src/game/bases.rs`.
pub unsafe fn fn_80160ea0(from: usize, to: usize) {
    let dy = platform::read_f32(to + 4) - platform::read_f32(from + 4);
    let dx = platform::read_f32(to) - platform::read_f32(from);
    let dz = platform::read_f32(to + 8) - platform::read_f32(from + 8);
    let horizontal = w97_call_f64_f32(W97_FN_802BE470, (dx * dx + dz * dz) as f64);
    w97_call_f32x2_u32(W97_FN_801611C0, dy, horizontal);
}

/// Scales an unsigned word by the shared random rate and truncates it.
///
/// The rate is the `f32` returned by [`fn_80161490`] on the SDA word
/// `r13 - 0x52F0`. The export's `CONCAT44(0x43300000, param_1 ^ 0x80000000) -
/// *(double *)(r2 - 0x5460)` pair is the unsigned-word to double idiom, then
/// rounded to `f32` and multiplied by the rate as a double.
///
/// # Original
/// - Ghidra: `FUN_801613a0` at `0x801613A0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801613a0_FUN_801613a0.c`
/// - observed: `dVar1 = (double)FUN_80161390(); return (int)((double)(float)
///   ((double)CONCAT44(0x43300000,param_1 ^ 0x80000000) -
///   *(double *)(unaff_r2 + -0x5460)) * dVar1);`
///
/// # Arguments
/// - `value`: word to scale.
///
/// # Returns
/// The truncated scaled value.
///
/// # Side effects
/// Advances the shared LCG state through [`fn_80161490`].
///
/// # Safety
/// The SDA slot `r13 - 0x52F0` must be writable.
///
/// # Confidence
/// high - the bias, the `f32` rounding and the truncation are all literal.
pub unsafe fn fn_801613a0(value: u32) -> i32 {
    let rate = fn_80161490(G_80161380_TARGET) as f64;
    ((value as f32) as f64 * rate) as i32
}

/// Scales a float by the shared random rate.
///
/// # Original
/// - Ghidra: `FUN_80161400` at `0x80161400` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161400_FUN_80161400.c`
/// - observed: `dVar1 = (double)FUN_80161390(); ... return
///   (double)(float)(param_1 * dVar1);`
///
/// # Arguments
/// - `value`: value to scale.
///
/// # Returns
/// `value * rate`, rounded to `f32`.
///
/// # Side effects
/// Advances the shared LCG state through [`fn_80161490`].
///
/// # Safety
/// The SDA slot `r13 - 0x52F0` must be writable.
///
/// # Confidence
/// high - the multiply and the `f32` rounding are literal; [`fn_80161390`] is
/// the translated tail call to [`fn_80161490`].
pub unsafe fn fn_80161400(value: f32) -> f32 {
    (value as f64 * fn_80161490(G_80161380_TARGET) as f64) as f32
}

/// Resets the GX state to the shipped clear colour.
///
/// Flushes the pipeline, installs the blend state `(0, 7, 0)` and clears with
/// four `0.0` colour components plus the BSS word at `r2 - 0x3470`.
///
/// # Original
/// - Ghidra: `FUN_80163220` at `0x80163220` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163220_FUN_80163220.c`
/// - observed: `FUN_801c5860(0); FUN_801c8e70(0,7,0); dVar1 =
///   (double)*(float *)(unaff_r2 + -0x5450); local_8[0] =
///   *(undefined4 *)(unaff_r2 + -0x3470); FUN_801c8920(dVar1,dVar1,dVar1,dVar1,0,local_8);`
///
/// # Arguments
/// None.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes GX registers and the clear colour.
///
/// # Safety
/// The three GX helpers must be callable; `r2 - 0x3470` must be readable.
///
/// # Confidence
/// medium - the three calls and the four `0.0` arguments are literal; the
/// export initialises only the first word of the eight-byte local it passes,
/// so the callee is given the address of that word and the second word is
/// `// unknown:`.
pub unsafe fn fn_80163220() {
    platform::call1(W97_FN_801C5860, 0);
    platform::call3(W97_FN_801C8E70, 0, 7, 0);
    let grey = platform::read_f32(W89_F32_ZERO);
    let mut colour = crate::platform::GuestLocal::new( platform::read_u32(W97_WORD_COLOUR_3470));
    w97_call_f32x4_u32_ptr(
        W97_FN_801C8920,
        grey,
        grey,
        grey,
        grey,
        0,
        (colour.as_mut_ptr() as *mut u32) as usize as u32,
    );
}

/// Fills one animation record with its start, clamped end and step.
///
/// `+0x04` receives `value`, `+0x08` the end value clamped to `0.0`, `+0x10`
/// the forwarded `start` value, `+0x0C` a copy of `+0x08` and `+0x14` the flag.
///
/// # Original
/// - Ghidra: `FUN_80163760` at `0x80163760` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80163760_FUN_80163760.c`
/// - observed: `if (param_3 < (double)*(float *)(unaff_r2 + -0x5450))
///   { param_3 = ...; } *(float *)(param_4 + 4) = (float)param_1;
///   ... FUN_801637e0(param_2,param_4); ... *(param_4 + 0xc) = *(param_4 + 8);`
///
/// # Arguments
/// - `value`: `f1`, stored at `+0x04`.
/// - `start`: `f2`, forwarded to `0x801637E0` which stores it at `+0x10`.
/// - `end`: `f3`, clamped to `0.0` and stored at `+0x08`.
/// - `target`: record base.
/// - `flag`: byte stored at `+0x14`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Five stores in the record and one call to `0x801637E0`.
///
/// # Safety
/// `target` must be writable for `0x15` bytes.
///
/// # Confidence
/// high - the clamp, the five stores and the call order are literal; the
/// `undefined8` second parameter is the `f2` float Ghidra cannot type.
pub unsafe fn fn_80163760(value: f32, start: f32, end: f32, target: usize, flag: u8) {
    let zero = platform::read_f32(W89_F32_ZERO);
    let end = if end < zero { zero } else { end };
    platform::write_f32(target + 4, value);
    platform::write_f32(target + 8, end);
    w97_call_f32_u32(W97_FN_801637E0, start, target as u32);
    platform::write_u8(target + 0x14, flag);
    platform::write_f32(target + 0x0C, end);
}

/// Looks up a named entry and starts its animation record.
///
/// The descriptor's `+0x04` word is the container handed to
/// [`fn_80163ff0`]; the found entry is stored at `+0x08`. When the entry
/// exists, the two accessors `0x802B50B0`/`0x802B50A0` read its `+4`
/// sub-object, the low halfword of the second result becomes the start value,
/// [`fn_80163760`] fills the record at `descriptor[0]`, and [`fn_801644d0`]
/// republishes the descriptor.
///
/// # Original
/// - Ghidra: `FUN_801643b0` at `0x801643b0` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801643b0_FUN_801643b0.c`
/// - observed: `iVar1 = FUN_80163ff0(param_2,param_3); param_1[2] = iVar1;
///   ... FUN_80163760((double)(float)((double)CONCAT44(0x43300000,uVar2 &
///   0xffff) - *(double *)(unaff_r2 + -0x5440)), ...)` and `return 1;`
///
/// # Arguments
/// - `descriptor`: `{object, container, entry}` triple, updated in place.
/// - `container`: object scanned by [`fn_80163ff0`].
/// - `name`: name word matched by that scan.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// Two descriptor stores, one record fill, one republish.
///
/// # Safety
/// `descriptor` must be writable for twelve bytes and the entry's `+4` word
/// must name the accessors' sub-object.
///
/// # Confidence
/// medium - every call and store is literal; the two accessor names and the
/// low-halfword truncation are `// inferred:` from the surrounding record use.
pub unsafe fn fn_801643b0(descriptor: usize, container: usize, name: u32) -> u32 {
    platform::write_u32(descriptor + 4, container as u32);
    let entry = fn_80163ff0(container, name) as usize;
    platform::write_u32(descriptor + 8, entry as u32);
    if entry != 0 {
        let sub = platform::read_u32(entry + 4);
        let first = platform::call1(W97_FN_802B50B0, sub);
        let second = platform::call1(W97_FN_802B50A0, sub);
        fn_80163760(
            (second & 0xFFFF) as f32,
            platform::read_f32(W97_F32_ONE_5428),
            platform::read_f32(W97_F32_MINUS_ONE_5424),
            platform::read_u32(descriptor) as usize,
            (first == 0) as u8,
        );
        fn_801644d0(descriptor);
    }
    1
}

/// Creates one text object into a free slot of the two text tables.
///
/// The two tables are `r13 - 0x52AC` (count `r13 - 0x52B0`) and
/// `r13 - 0x52A4` (count `r13 - 0x52A8`). The slot is rejected when either
/// table is unset, the index is out of range or occupied. Otherwise the
/// current heap is switched to `heap`, a `0x28`-byte object is allocated and
/// constructed by `0x802C81F0`, the heap is restored, and `0x80164990` binds
/// the object to its slot when `check` is non-zero. A failure runs the
/// object's `+0x18` deleting destructor with `1`.
///
/// # Original
/// - Ghidra: `FUN_80164770` at `0x80164770` (308 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80164770_FUN_80164770.c`
/// - observed: `uVar3 = thunk_FUN_802b9170(); iVar4 = FUN_802b9240(0x28,uVar2,4);
///   ... piVar5 = (int *)FUN_802c81f0(iVar4,(int)uVar6,param_3,param_4);
///   thunk_FUN_802b9170(uVar3);` and `(**(code **)(*piVar5 + 0x18))(piVar5,1);`
///
/// # Arguments
/// - `heap`: heap the object is allocated from (the prologue's high word).
/// - `context`: second `0x802C81F0` argument (the prologue's low word, which
///   Ghidra prints as `(int)uVar6`).
/// - `arg3`, `arg4`: forwarded to `0x802C81F0`.
/// - `check`: non-zero to require the `0x80164990` slot bind.
/// - `index`: slot index in both tables.
///
/// # Returns
/// `1` when the object was created and bound, `0` otherwise.
///
/// # Side effects
/// Allocates `0x28` bytes, writes one table slot, may destroy the object.
///
/// # Safety
/// The four SDA slots must be readable and the tables writable.
///
/// # Confidence
/// medium - the guard chain, the allocation, the destructor and the table
/// store are literal; the first `0x802B9170` argument is dropped by the
/// decompiler and is supplied as `heap` (`// inferred:`), matching the
/// restore call that passes the saved value back.
pub unsafe fn fn_80164770(
    heap: u32,
    context: u32,
    arg3: u32,
    arg4: u32,
    check: u32,
    index: u32,
) -> u32 {
    let table_a = platform::read_u32(W97_GLOBAL_TEXT_TABLE_A);
    let count_a = platform::read_u32(W97_GLOBAL_TEXT_COUNT_A);
    let count_b = platform::read_u32(W97_GLOBAL_TEXT_COUNT_B);
    let table_b = platform::read_u32(W97_GLOBAL_TEXT_TABLE_B);
    let offset = index as usize * 4;
    if table_a == 0
        || (index as i32) < 0
        || count_a <= index
        || count_b <= index
        || platform::read_u32(table_a as usize + offset) != 0
        || platform::read_u32(table_b as usize + offset) != 0
    {
        return 0;
    }
    let saved = platform::call1(W97_FN_802B9170, heap);
    let buffer = platform::call3(W97_FN_802B9240, 0x28, heap, 4) as usize;
    let mut object = 0usize;
    if buffer != 0 {
        object = platform::call4(W97_FN_802C81F0, buffer as u32, context, arg3, arg4) as usize;
    }
    platform::call1(W97_FN_802B9170, saved);
    if object == 0
        || (check != 0 && platform::call3(W97_FN_80164990, table_a, 0x20, index) == 0)
    {
        if object != 0 {
            let destroy = platform::read_u32(platform::read_u32(object) as usize + 0x18) as usize;
            platform::call2(destroy, object as u32, 1);
        }
        return 0;
    }
    platform::write_u32(table_a as usize + offset, object as u32);
    1
}

/// Commits the base element's render state.
///
/// The element's `+0x04` object owns the state; the reference count at
/// `*source + 0x20` becomes the first float argument, `1.0` and `-1.0` the
/// other two. A `mode` of `4` is replaced by `leading_zeros(word) >> 5`, which
/// maps `0` to `1` and any other word to `0`.
///
/// # Original
/// - Ghidra: `FUN_801654b0` at `0x801654b0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801654b0_FUN_801654b0.c`
/// - observed: `if (param_3 == 4) { param_3 =
///   countLeadingZeros(*(undefined4 *)(*param_2 + 0x24)); param_3 = param_3 >> 5; }
///   FUN_80168f10(...,param_1,param_3);`
///
/// # Arguments
/// - `element`: element whose `+0x04` object receives the state.
/// - `source`: pointer to the word holding the reference count.
/// - `mode`: `4` selects the `leading_zeros` form, otherwise the byte written.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls [`fn_80168f10`] and writes the element's state block.
///
/// # Safety
/// `element` and `*source` must be readable/writable as documented.
///
/// # Confidence
/// medium - the `leading_zeros` folding, the offsets and the three constants
/// are literal; the two `f32` values are the bias-trick conversion of the
/// halfword reference count.
pub unsafe fn fn_801654b0(element: usize, source: usize, mode: u32) {
    let mut mode = mode;
    if mode == 4 {
        mode = platform::read_u32(platform::read_u32(source) as usize + 0x24).leading_zeros() >> 5;
    }
    let count = w97_u16_to_f64(platform::read_u32(source) as usize + 0x20);
    fn_80168f10(
        count as f32,
        platform::read_f32(W97_F32_ONE_5428),
        platform::read_f32(W97_F32_MINUS_ONE_5424),
        element,
        mode,
    );
}

/// Commits one first-class element's render state.
///
/// Same offsets as [`fn_801654b0`] (reference count at `+0x20`,
/// `leading_zeros` word at `+0x24`); only the `r2` constant trio differs,
/// `-0x5410`/`-0x5418`/`-0x5414`.
///
/// # Original
/// - Ghidra: `FUN_80165ae0` at `0x80165ae0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165ae0_FUN_80165ae0.c`
/// - observed: `param_3 = countLeadingZeros(*(undefined4 *)(*param_2 + 0x24));
///   ... FUN_80168f10(...,param_1,param_3);`
///
/// # Arguments
/// - `element`: element whose `+0x04` object receives the state.
/// - `source`: pointer to the word holding the reference count.
/// - `mode`: `4` selects the `leading_zeros` form.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `element` and `*source` must be live.
///
/// # Confidence
/// medium - the offsets and the three constants (`r2 - 0x5410/0x5418/0x5414`)
/// are literal; the float conversion is the bias idiom.
pub unsafe fn fn_80165ae0(element: usize, source: usize, mode: u32) {
    let mut mode = mode;
    if mode == 4 {
        mode = platform::read_u32(platform::read_u32(source) as usize + 0x24).leading_zeros() >> 5;
    }
    let count = w97_u16_to_f64(platform::read_u32(source) as usize + 0x20);
    fn_80168f10(
        count as f32,
        platform::read_f32(W97_F32_ONE_5428),
        platform::read_f32(W97_F32_MINUS_ONE_5424),
        element,
        mode,
    );
}

/// Commits one second-class element's render state.
///
/// This class keeps its reference count at `+0x30` and its `leading_zeros`
/// word at `+0x38`; the three forwarded constants are at `r2 - 0x5400`,
/// `-0x5408` and `-0x5404`.
///
/// # Original
/// - Ghidra: `FUN_801666e0` at `0x801666e0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801666e0_FUN_801666e0.c`
/// - observed: `param_3 = countLeadingZeros(*(undefined4 *)(*param_2 + 0x38));
///   ... FUN_80168f10((double)(float)((double)CONCAT44(0x43300000,
///   *(ushort *)(*param_2 + 0x30) ^ 0x80000000) - ...));`
///
/// # Arguments
/// - `element`: element whose `+0x04` object receives the state.
/// - `source`: pointer to the word holding the reference count.
/// - `mode`: `4` selects the `leading_zeros` form.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `element` and `*source` must be live.
///
/// # Confidence
/// medium - the `+0x30`/`+0x38` offsets and the constant addresses are literal.
pub unsafe fn fn_801666e0(element: usize, source: usize, mode: u32) {
    let mut mode = mode;
    if mode == 4 {
        mode = platform::read_u32(platform::read_u32(source) as usize + 0x38).leading_zeros() >> 5;
    }
    let count = w97_u16_to_f64(platform::read_u32(source) as usize + 0x30);
    fn_80168f10(
        count as f32,
        platform::read_f32(W97_F32_ONE_5428),
        platform::read_f32(W97_F32_MINUS_ONE_5424),
        element,
        mode,
    );
}

/// Commits one third-class element's render state.
///
/// The reference count is at `+0x20`, the `leading_zeros` word at `+0x28`,
/// and the constants at `r2 - 0x53F0`, `-0x53F8` and `-0x53F4`.
///
/// # Original
/// - Ghidra: `FUN_801672d0` at `0x801672d0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801672d0_FUN_801672d0.c`
/// - observed: `param_3 = countLeadingZeros(*(undefined4 *)(*param_2 + 0x28));
///   ... FUN_80168f10(...,param_1,param_3);`
///
/// # Arguments
/// - `element`: element whose `+0x04` object receives the state.
/// - `source`: pointer to the word holding the reference count.
/// - `mode`: `4` selects the `leading_zeros` form.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `element` and `*source` must be live.
///
/// # Confidence
/// medium - the `+0x20`/`+0x28` offsets and the constant addresses are literal.
pub unsafe fn fn_801672d0(element: usize, source: usize, mode: u32) {
    let mut mode = mode;
    if mode == 4 {
        mode = platform::read_u32(platform::read_u32(source) as usize + 0x28).leading_zeros() >> 5;
    }
    let count = w97_u16_to_f64(platform::read_u32(source) as usize + 0x20);
    fn_80168f10(
        count as f32,
        platform::read_f32(W97_F32_ONE_5428),
        platform::read_f32(W97_F32_MINUS_ONE_5424),
        element,
        mode,
    );
}

/// Commits the four-argument class's render state.
///
/// The reference count is at `+0x20` and the `leading_zeros` word at `+0x24`;
/// the constants are at `r2 - 0x53E0`, `-0x53E8` and `-0x53E4`.
///
/// # Original
/// - Ghidra: `FUN_80167e50` at `0x80167e50` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80167e50_FUN_80167e50.c`
/// - observed: `param_3 = countLeadingZeros(*(undefined4 *)(*param_2 + 0x24));
///   ... FUN_80168f10(...,param_1,param_3);`
///
/// # Arguments
/// - `element`: element whose `+0x04` object receives the state.
/// - `source`: pointer to the word holding the reference count.
/// - `mode`: `4` selects the `leading_zeros` form.
///
/// # Returns
/// Nothing.
///
/// # Safety
/// `element` and `*source` must be live.
///
/// # Confidence
/// medium - the `+0x20`/`+0x24` offsets and the constant addresses are literal.
pub unsafe fn fn_80167e50(element: usize, source: usize, mode: u32) {
    let mut mode = mode;
    if mode == 4 {
        mode = platform::read_u32(platform::read_u32(source) as usize + 0x24).leading_zeros() >> 5;
    }
    let count = w97_u16_to_f64(platform::read_u32(source) as usize + 0x20);
    fn_80168f10(
        count as f32,
        platform::read_f32(W97_F32_ONE_5428),
        platform::read_f32(W97_F32_MINUS_ONE_5424),
        element,
        mode,
    );
}

/// Pushes one element's render state into its `+0x04` object.
///
/// `value` lands at `+0x28` with a zero at `+0x2c`; `current` is clamped to
/// `0.0` and, when `mode` is not `1`, replaced by `value - 1.0`. The
/// `+0x04` object's `+0x1C` slot receives the clamped value, its `+0x28` slot
/// receives `minimum`, and `+0x34`/`+0x30` take the mode byte and the clamped
/// value.
///
/// # Original
/// - Ghidra: `FUN_80168f10` at `0x80168f10` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168f10_FUN_80168f10.c`
/// - observed: `if ((param_3 < ... -0x53d0) && (param_3 = ...,
///   param_5 != 1)) { param_3 = (float)(param_1 - ... -0x53c8); } ... (**(code **)
///   (**(int **)(param_4 + 4) + 0x1c))(param_3);
///   (**(code **)(**(int **)(param_4 + 4) + 0x28))(param_2);`
///
/// # Arguments
/// - `value`: `f1`, the new value stored at `+0x28`.
/// - `minimum`: `f2`, forwarded to the `+0x28` slot.
/// - `current`: `f3`, the clamped value stored at `+0x30`.
/// - `element`: element base.
/// - `mode`: byte stored at `+0x34`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Four element stores and two virtual calls.
///
/// # Safety
/// `element + 4` must name an object with a vtable carrying slots `0x1C` and
/// `0x28`.
///
/// # Confidence
/// medium - the clamp, the four stores and the two slots are literal; the
/// `undefined8` second parameter is the `f2` float and the elided receiver is
/// the `+0x04` object (`// inferred:`).
pub unsafe fn fn_80168f10(value: f32, minimum: f32, current: f32, element: usize, mode: u32) {
    let zero = platform::read_f32(W97_F32_ZERO_53D0);
    let mut target = current;
    if target < zero {
        target = zero;
        if mode != 1 {
            target = value - platform::read_f32(W97_F32_ONE_53C8);
        }
    }
    platform::write_f32(element + 0x28, value);
    platform::write_u32(element + 0x2C, platform::read_u32(W97_F32_ZERO_53D0));
    let receiver = platform::read_u32(element + 4);
    let vtable = platform::read_u32(receiver as usize) as usize;
    w97_call_f32_u32(platform::read_u32(vtable + 0x1C) as usize, target, receiver);
    w97_call_f32_u32(platform::read_u32(vtable + 0x28) as usize, minimum, receiver);
    platform::write_u8(element + 0x34, mode as u8);
    platform::write_f32(element + 0x30, target);
}

/// Resets a scale record to its idle layout.
///
/// When `scale` is `0.0` the record is handed to `0x80168B90` instead.
/// Otherwise `+0x04`/`+0x10` take `1.0`, `+0x08`/`+0x14`/`+0x18` take `0.0`,
/// `+0x1D` is set and `+0x0C` becomes `1.0 / scale`.
///
/// # Original
/// - Ghidra: `FUN_80168bd0` at `0x80168bd0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168bd0_FUN_80168bd0.c`
/// - observed: `fVar1 = *(float *)(unaff_r2 + -0x53d8);
///   if ((double)fVar1 == param_1) { FUN_80168b90(); return; } ...`
///
/// # Arguments
/// - `scale`: the divisor, read as a double.
/// - `target`: record base.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Six stores, or one call to `0x80168B90`.
///
/// # Safety
/// `target` must be writable for `0x1E` bytes.
///
/// # Confidence
/// medium - the comparison and all six stores are literal; the call's receiver
/// is dropped by the decompiler and is supplied as `target` (`// inferred:`,
/// `0x80168B90` writes exactly the `+0x04..+0x18` fields this record owns).
pub unsafe fn fn_80168bd0(scale: f64, target: usize) {
    let zero = platform::read_f32(W89_F32_TWEEN_ZERO);
    if zero as f64 == scale {
        platform::call1(W89_FN_80168B90, target as u32);
        return;
    }
    let one = platform::read_f32(W89_F32_TWEEN_ONE);
    platform::write_f32(target + 4, one);
    platform::write_f32(target + 8, zero);
    platform::write_f32(target + 0x10, one);
    platform::write_f32(target + 0x14, zero);
    platform::write_f32(target + 0x18, zero);
    platform::write_u8(target + 0x1D, 1);
    platform::write_f32(target + 0x0C, (one as f64 / scale) as f32);
}

/// Runs the `+0x1C` slot and stores the new scale.
///
/// # Original
/// - Ghidra: `FUN_80168fe0` at `0x80168fe0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80168fe0_FUN_80168fe0.c`
/// - observed: `FUN_801680a0(); *(float *)(param_2 + 0x30) = (float)param_1;`
///
/// # Arguments
/// - `value`: the new `+0x30` float.
/// - `target`: element base.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One virtual call through `0x801680A0` and one store.
///
/// # Safety
/// `target` must be writable at `+0x30` and its `+0x04` object must have a
/// vtable.
///
/// # Confidence
/// medium - the call and the store are literal; the elided receiver of
/// `0x801680A0` is supplied as `target` (`// inferred:`).
pub unsafe fn fn_80168fe0(value: f64, target: usize) {
    platform::call1(W97_FN_801680A0, target as u32);
    platform::write_f32(target + 0x30, value as f32);
}

/// Reports whether the element's animation has reached its bound.
///
/// Two virtual reads (`+0x20`, `+0x2C` of the `+0x04` object) give the current
/// value and the rate. A negative rate, or mode byte `3`, compares the current
/// value against `+0x2C`; mode byte `1` compares `+0x28 - 1.0` against it; any
/// other mode reports `false`.
///
/// # Original
/// - Ghidra: `FUN_80169020` at `0x80169020` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169020_FUN_80169020.c`
/// - observed: `dVar2 = (double)(**(code **)(**(int **)(param_1 + 4) + 0x20))();
///   dVar3 = (double)(**(code **)(**(int **)(param_1 + 4) + 0x2c))(); ...
///   if (*(char *)(param_1 + 0x34) == '\x01') { bVar1 = (...) <= dVar2; }`
///
/// # Arguments
/// - `element`: element base.
///
/// # Returns
/// `true` when the bound is reached.
///
/// # Safety
/// `element + 4` must name an object with a vtable.
///
/// # Confidence
/// medium - the comparisons and the mode bytes are literal; the two virtual
/// receivers are dropped by the decompiler and supplied as the `+0x04` object
/// (`// inferred:`, the same object the neighbouring slots use).
pub unsafe fn fn_80169020(element: usize) -> bool {
    let receiver = platform::read_u32(element + 4);
    let vtable = platform::read_u32(receiver as usize) as usize;
    let current = w97_call_u32_f32(platform::read_u32(vtable + 0x20) as usize, receiver);
    let rate = w97_call_u32_f32(platform::read_u32(vtable + 0x2C) as usize, receiver);
    let mode = platform::read_u8(element + 0x34);
    if rate < platform::read_f32(W97_F32_ZERO_53D0) || mode == 3 {
        current as f64 <= platform::read_f32(element + 0x2C) as f64
    } else if mode == 1 {
        (platform::read_f32(element + 0x28) - platform::read_f32(W97_F32_ONE_53C8)) as f64
            <= current as f64
    } else {
        false
    }
}

/// Reports whether `value` lies inside the element's traversed range.
///
/// A current value equal to `+0x30` is compared directly with `value`.
/// Otherwise the rate decides the direction: a negative rate, or bit `2` of the
/// mode byte, walks in one direction, every other mode the other.
///
/// # Original
/// - Ghidra: `FUN_801690e0` at `0x801690e0` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801690e0_FUN_801690e0.c`
/// - observed: `if ((double)*(float *)(param_2 + 0x30) == dVar3) { bVar2 =
///   dVar3 == param_1; ... } ... if ((dVar4 < ... -0x53d0) ||
///   ((*(byte *)(param_2 + 0x34) & 2) != 0)) { ... }`
///
/// # Arguments
/// - `value`: the value looked for in `[current, bound)` or `(bound, current]`.
/// - `element`: element base.
///
/// # Returns
/// `true` when `value` is inside the range.
///
/// # Safety
/// `element + 4` must name an object with a vtable.
///
/// # Confidence
/// medium - every comparison and both strictness choices are literal; the two
/// virtual receivers are `// inferred:` from the `+0x04` object.
pub unsafe fn fn_801690e0(value: f64, element: usize) -> bool {
    let receiver = platform::read_u32(element + 4);
    let vtable = platform::read_u32(receiver as usize) as usize;
    let current = w97_call_u32_f32(platform::read_u32(vtable + 0x20) as usize, receiver) as f64;
    let bound = platform::read_f32(element + 0x30) as f64;
    if bound == current {
        return current == value;
    }
    let rate = w97_call_u32_f32(platform::read_u32(vtable + 0x2C) as usize, receiver);
    if rate < platform::read_f32(W97_F32_ZERO_53D0) || (platform::read_u8(element + 0x34) & 2) != 0
    {
        if bound <= current {
            value < bound || current <= value
        } else {
            value < bound && current <= value
        }
    } else if current <= bound {
        bound < value || value <= current
    } else {
        bound < value && value <= current
    }
}

/// `0x8042AED8`: render-state bit word set to `5` by [`fn_80169220`] before
/// the render-target pass ([`crate::resource::archive::fn_802c7a50`]); the
/// matching end pass reads its bits 0/1/2.
const G_RENDER_STATE_8042AED8: usize = 0x8042_AED8;

/// `0x80424798`: static extent record copy-assigned from the caller's object
/// by [`fn_80169220`].
const G_EXTENT_RECORD_80424798: usize = 0x8042_4798;

/// Binds a render-target object to the context-derived transform block.
///
/// The state id is resolved through [`fn_80164b20`], wrapped in a one-word
/// holder so [`crate::runtime::os::fn_802540b0`] can copy the node's 3x4
/// matrix into the stack buffer. The render-state word is then set to `5`, the
/// matrix is installed with [`crate::resource::archive::fn_802c7a50`] (mode =
/// the low half of `state`, render target = `object`), and `object` is
/// copy-assigned into the static extent record with
/// [`crate::runtime::cpp::fn_802d1140`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80169220`.
/// - Extent: `0x80169220..0x80169290` (112 bytes, 28 words), ending at the
///   `blr` at `0x8016928C` flush against [`fn_80169290`].
/// - Reach: two REL `bl` sites in `d_basesNP`, `0x807B67D8` and `0x807B9110`;
///   no main-image branch and no data word.
/// - Observed: `9421FFB0 stwu r1,-0x50(r1)`; `mflr r0`; `stw r0,0x54(r1)`;
///   `stw r31,0x4C(r1)`; `mr r31,r4`; `stw r30,0x48(r1)`; `mr r30,r3`;
///   `4BFFB8E5 bl 0x80164B20`; `stw r3,8(r1)`; `addi r3,r1,8`;
///   `addi r4,r1,0x10`; `480EAE65 bl 0x802540B0`; `li r0,5`;
///   `stw r0,r13,-18344`; `mr r5,r31`; `rlwinm r3,r30,0,16,31`;
///   `addi r4,r1,0x10`; `4815E7ED bl 0x802C7A50`; `lis r3,0x8042`;
///   `mr r4,r31`; `addi r3,r3,18328`; `48167ECD bl 0x802D1140`; epilogue;
///   `4E800020 blr`.
/// - Layout: `+0x08` is the one-word holder filled with the
///   [`fn_80164b20`] result and `+0x10..+0x40` is the 0x30-byte matrix copied
///   by `0x802540B0`.
/// - Decoder note: `stw r0,r13,-18344` is the SDA word at
///   `0x8042F680 - 0x47A8 = 0x8042AED8`; `lis r3,0x8042` plus the positive
///   `addi 18328` (`0x4798`) is `0x80424798`. `rlwinm r3,r30,0,16,31`
///   zero-extends the low half of `state` for the `u16` mode argument.
///
/// # Arguments
/// * `state` - context state id read by [`fn_80164b20`]; its low 16 bits become
///   the render-target mode.
/// * `object` - render-target object stored at `0x8042AED0` by the begin call
///   and copied into `0x80424798`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Two SDA stores, one matrix copy, one transform install, the 2D draw-state
/// reset of [`crate::resource::archive::fn_802c7a50`] and one C++ member-block
/// copy assignment.
///
/// # Safety
/// The context returned by [`fn_80164b20`] must be live, `object` must be a
/// live instance of the class stored at `0x80424798`, and the two SDA words
/// must be writable.
///
/// # Confidence
/// medium - every call, argument and offset is literal; that `object` is a
/// render target and `0x80424798` its extent record is `// inferred:` from
/// [`crate::resource::archive::fn_802c7a50`] and
/// [`crate::resource::archive::fn_802c5760`], which read the same two words.
pub unsafe fn fn_80169220(state: u32, object: usize) {
    let node = fn_80164b20(state);
    let mut holder = crate::platform::GuestLocal::new( node);
    let mut matrix = crate::platform::GuestLocal::new( [0u32; 12]);
    crate::runtime::os::fn_802540b0(
        (holder.as_ptr() as *const u32) as usize,
        matrix.as_mut_ptr() as usize,
    );
    platform::write_u32(G_RENDER_STATE_8042AED8, 5);
    archive::fn_802c7a50(
        (state & 0xFFFF) as u16,
        matrix.as_mut_ptr() as usize as u32,
        object as u32,
    );
    crate::runtime::cpp::fn_802d1140(G_EXTENT_RECORD_80424798, object);
}

/// Creates one texture and configures its draw block.
///
/// The two dimensions are stored at `+0x2C`/`+0x2E`; when `halve` is set they
/// are halved first. `0x801C6BA0` creates the texture, the allocator object's
/// `+0x14` slot reserves `0x20`-aligned storage for it, and `0x801C6D90`
/// configures the `+0x04` block. The draw then runs with the `0.0` fill
/// colour three times and the `+0x34` word twice.
///
/// # Original
/// - Ghidra: `FUN_80169290` at `0x80169290` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169290_FUN_80169290.c`
/// - observed: `uVar5 = FUN_802dcd68(); ... uVar2 = FUN_801c6ba0(param_3 &
///   0xffff,param_4 & 0xffff,param_5,0,0); iVar3 = (**(code **)(*(int *)uVar5
///   + 0x14))((int *)uVar5,uVar2,0x20); ... FUN_802dcdb4(iVar3 != 0);`
///
/// # Arguments
/// - `this`: texture record (the prologue's high word).
/// - `allocator`: allocator object (the prologue's low word).
/// - `width`, `height`: requested dimensions.
/// - `arg5`, `arg6`: format word and a word stored at `+0x34`.
/// - `halve`: non-zero to halve both dimensions and set `+0x38`.
///
/// # Returns
/// `1` when the storage was reserved, `0` otherwise.
///
/// # Side effects
/// Allocates texture storage and writes `+0x24..+0x38`.
///
/// # Safety
/// `allocator` must have a vtable; `this` must be writable for `0x3A` bytes.
///
/// # Confidence
/// medium - the guards, the five calls and every store are literal; the
/// graphics callee names are `// inferred:` from their argument shapes.
pub unsafe fn fn_80169290(
    this: usize,
    allocator: usize,
    width: u32,
    height: u32,
    arg5: u32,
    arg6: u32,
    halve: u32,
) -> u32 {
    platform::write_u16(this + 0x2C, width as u16);
    platform::write_u16(this + 0x2E, height as u16);
    let mut w = width;
    let mut h = height;
    if halve != 0 {
        w = (width >> 1) & 0x7FFF;
        h = (height >> 1) & 0x7FFF;
    }
    let handle = platform::call5(W97_FN_801C6BA0, w & 0xFFFF, h & 0xFFFF, arg5, 0, 0);
    let vtable = platform::read_u32(allocator) as usize;
    let storage = platform::call3(
        platform::read_u32(vtable + 0x14) as usize,
        allocator as u32,
        handle,
        0x20,
    ) as usize;
    platform::write_u32(this + 0x24, storage as u32);
    if storage == 0 {
        return 0;
    }
    platform::write_u32(this + 0x28, handle);
    platform::write_u8(this + 0x38, halve as u8);
    platform::write_u32(this + 0x30, arg5);
    platform::write_u32(this + 0x34, arg6);
    call8_u32(
        W97_FN_801C6D90,
        (this + 4) as u32,
        storage as u32,
        w & 0xFFFF,
        h & 0xFFFF,
        arg5,
        0,
        0,
        0,
    );
    let zero = platform::read_f32(W97_F32_ZERO_53C0);
    let twice = platform::read_u32(this + 0x34);
    w97_call_f32x3_u32x6(
        W97_FN_801C6FF0,
        zero,
        zero,
        zero,
        (this + 4) as u32,
        twice,
        twice,
        0,
        0,
        0,
    );
    1
}

/// Presents a texture record with a caller-supplied destination rectangle.
///
/// # Original
/// - Ghidra: `FUN_801693f0` at `0x801693f0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801693f0_FUN_801693f0.c`
/// - observed: `FUN_80169440(param_1,*(undefined2 *)(param_1 + 0x2c),
///   *(undefined2 *)(param_1 + 0x2e), *(undefined4 *)(param_1 + 0x30), ...,
///   param_2,param_3,param_4);`
///
/// # Arguments
/// - `record`: texture record built by [`fn_80169290`].
/// - `arg2`, `arg3`, `arg4`: forwarded destination words.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Everything [`fn_80169440`] does.
///
/// # Safety
/// `record` must be live and readable at `+0x2C..+0x38`.
///
/// # Confidence
/// high - the ten argument slots are forwarded in order, `+0x30` twice.
pub unsafe fn fn_801693f0(record: usize, arg2: u32, arg3: u32, arg4: u32) {
    fn_80169440(
        record,
        platform::read_u16(record + 0x2C) as u32,
        platform::read_u16(record + 0x2E) as u32,
        platform::read_u32(record + 0x30),
        platform::read_u32(record + 0x30),
        platform::read_u32(record + 0x34),
        platform::read_u8(record + 0x38) as u32,
        arg2,
        arg3,
        arg4,
    );
}

/// Draws one texture record end to end.
///
/// The block is reconfigured at `+0x04`, drawn with the `0.0` fill colour,
/// then presented through `0x801C5960`/`0x801C59D0` and bound by
/// `0x801C6290`. The blend registers are rebuilt twice: once neutral and once
/// from the global display object's `+0x19`/`+0x1A`/`+0x32` fields.
///
/// # Original
/// - Ghidra: `FUN_80169440` at `0x80169440` (276 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169440_FUN_80169440.c`
/// - observed: `uVar6 = FUN_802dcd5c(); ... FUN_801c6d90(iVar2 + 4,
///   *(undefined4 *)(iVar2 + 0x24),uVar4 & 0xffff,uVar3 & 0xffff); ...
///   iVar2 = **(int **)(unaff_r13 + -0x5200);
///   FUN_801c5f40(*(undefined1 *)(iVar2 + 0x19),iVar2 + 0x1a,1,iVar2 + 0x32);`
///
/// # Arguments
/// - `this`: texture record (the prologue's high word).
/// - `width`, `height`: size, halved when `halve` is set (the low word is the
///   width, the first explicit parameter the height).
/// - `arg4`: source rectangle word for `0x801C59D0`.
/// - `arg5`: `// unknown:` never read by the export.
/// - `arg6`: blend word passed to the draw twice.
/// - `halve`: non-zero to halve both dimensions and set the alpha byte.
/// - `arg8`: destination word for `0x801C5960`.
/// - `arg9`, `arg10`: source rectangle halfword and texture filter byte.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Seven calls and several `GX` register writes.
///
/// # Safety
/// The record must be live and the global display object readable at `+0x19`.
///
/// # Confidence
/// medium - every call and argument slot is literal; `arg5` is unused and the
/// graphics callee names are `// inferred:` from their argument shapes.
pub unsafe fn fn_80169440(
    this: usize,
    width: u32,
    height: u32,
    arg4: u32,
    arg5: u32,
    arg6: u32,
    halve: u32,
    arg8: u32,
    arg9: u32,
    arg10: u32,
) {
    let _ = arg5; // unknown: the export never reads the fifth parameter.
    let mut w = width;
    let mut h = height;
    if halve != 0 {
        w = (width >> 1) & 0x7FFF;
        h = (height >> 1) & 0x7FFF;
    }
    platform::call4(
        W97_FN_801C6D90,
        (this + 4) as u32,
        platform::read_u32(this + 0x24),
        w & 0xFFFF,
        h & 0xFFFF,
    );
    let zero = platform::read_f32(W97_F32_ZERO_53C0);
    w97_call_f32x3_u32x6(
        W97_FN_801C6FF0,
        zero,
        zero,
        zero,
        (this + 4) as u32,
        arg6,
        arg6,
        0,
        0,
        0,
    );
    platform::call4(W97_FN_801C5F40, 0, 0, 0, 0);
    platform::call4(W97_FN_801C5960, arg8, arg9 & 0xFFFF, width, height);
    platform::call4(W97_FN_801C59D0, w & 0xFFFF, h & 0xFFFF, arg4, halve & 0xFF);
    platform::call2(W97_FN_801C6290, platform::read_u32(this + 0x24), arg10);
    let object = platform::read_u32(platform::read_u32(W97_GLOBAL_DISPLAY_5200) as usize) as usize;
    platform::call4(
        W97_FN_801C5F40,
        platform::read_u8(object + 0x19) as u32,
        (object + 0x1A) as u32,
        1,
        (object + 0x32) as u32,
    );
}

/// Builds the rotation matrix about the X axis.
///
/// The halfword at `angle` is scaled by `1/256` and passed to the cosine and
/// sine helpers; the twelve-word block becomes
/// `[1,0,0,0, 0,cos,-sin,0, 0,sin,cos,0]`.
///
/// # Original
/// - Ghidra: `FUN_8016ebf0` at `0x8016ebf0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ebf0_FUN_8016ebf0.c`
/// - observed: `sVar2 = *param_2; ... dVar7 = (double)FUN_80237c40((double)(float)
///   ((double)*(float *)(unaff_r2 + -0x5360) * dVar7)); ... *param_1 =
///   *(undefined4 *)(unaff_r2 + -0x535c); ... param_1[10] = (float)dVar7;`
///
/// # Arguments
/// - `matrix`: twelve-word output block.
/// - `angle`: pointer to the fixed-point angle.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve float stores.
///
/// # Safety
/// `matrix` must be writable for 48 bytes and `angle` readable for two.
///
/// # Confidence
/// medium - the twelve stores, the `1/256` scale and the two helpers are
/// literal; `// inferred:` the paired-single read is a signed 16-bit angle
/// (the only branch that does not read uninitialised stack bytes).
pub unsafe fn fn_8016ebf0(matrix: usize, angle: usize) {
    let raw = platform::read_u16(angle) as i16;
    let radians = raw as f32 * platform::read_f32(W97_F32_ANGLE_SCALE_5360);
    let cosine = w97_call_f64_f64(W97_FN_80237C40, radians as f64) as f32;
    let sine = w97_call_f64_f64(W97_FN_80237BD0, radians as f64) as f32;
    let diagonal = platform::read_f32(W97_F32_ONE_535C);
    let off = platform::read_f32(W97_F32_ZERO_5358);
    platform::write_f32(matrix, diagonal);
    platform::write_f32(matrix + 4, off);
    platform::write_f32(matrix + 8, off);
    platform::write_f32(matrix + 0x0C, off);
    platform::write_f32(matrix + 0x10, off);
    platform::write_f32(matrix + 0x14, cosine);
    platform::write_f32(matrix + 0x18, -sine);
    platform::write_f32(matrix + 0x1C, off);
    platform::write_f32(matrix + 0x20, off);
    platform::write_f32(matrix + 0x24, sine);
    platform::write_f32(matrix + 0x28, cosine);
    platform::write_f32(matrix + 0x2C, off);
}

/// Builds the rotation matrix about the Y axis.
///
/// The block becomes `[cos,0,sin,0, 0,1,0,0, -sin,0,cos,0]`.
///
/// # Original
/// - Ghidra: `FUN_8016ed10` at `0x8016ed10` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ed10_FUN_8016ed10.c`
/// - observed: `*param_1 = (float)dVar8; param_1[1] = fVar1;
///   param_1[2] = (float)dVar9; ... param_1[8] = (float)-dVar9;
///   param_1[10] = (float)dVar8;`
///
/// # Arguments
/// - `matrix`: twelve-word output block.
/// - `angle`: pointer to the fixed-point angle.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve float stores.
///
/// # Safety
/// `matrix` must be writable for 48 bytes and `angle` readable for two.
///
/// # Confidence
/// medium - the twelve stores and the `1/256` scale are literal;
/// `// inferred:` the angle is a signed 16-bit value.
pub unsafe fn fn_8016ed10(matrix: usize, angle: usize) {
    let raw = platform::read_u16(angle) as i16;
    let radians = raw as f32 * platform::read_f32(W97_F32_ANGLE_SCALE_5360);
    let cosine = w97_call_f64_f64(W97_FN_80237C40, radians as f64) as f32;
    let sine = w97_call_f64_f64(W97_FN_80237BD0, radians as f64) as f32;
    let diagonal = platform::read_f32(W97_F32_ONE_535C);
    let off = platform::read_f32(W97_F32_ZERO_5358);
    platform::write_f32(matrix, cosine);
    platform::write_f32(matrix + 4, off);
    platform::write_f32(matrix + 8, sine);
    platform::write_f32(matrix + 0x0C, off);
    platform::write_f32(matrix + 0x10, off);
    platform::write_f32(matrix + 0x14, diagonal);
    platform::write_f32(matrix + 0x18, off);
    platform::write_f32(matrix + 0x1C, off);
    platform::write_f32(matrix + 0x20, -sine);
    platform::write_f32(matrix + 0x24, off);
    platform::write_f32(matrix + 0x28, cosine);
    platform::write_f32(matrix + 0x2C, off);
}

/// Builds the rotation matrix about the Z axis.
///
/// The block becomes `[cos,-sin,0,0, sin,cos,0,0, 0,0,1,0]`.
///
/// # Original
/// - Ghidra: `FUN_8016ee30` at `0x8016ee30` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ee30_FUN_8016ee30.c`
/// - observed: `*param_1 = (float)dVar8; param_1[2] = fVar1;
///   param_1[1] = (float)-dVar9; ... param_1[10] = fVar2;`
///
/// # Arguments
/// - `matrix`: twelve-word output block.
/// - `angle`: pointer to the fixed-point angle.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve float stores.
///
/// # Safety
/// `matrix` must be writable for 48 bytes and `angle` readable for two.
///
/// # Confidence
/// medium - the twelve stores and the `1/256` scale are literal;
/// `// inferred:` the angle is a signed 16-bit value.
pub unsafe fn fn_8016ee30(matrix: usize, angle: usize) {
    let raw = platform::read_u16(angle) as i16;
    let radians = raw as f32 * platform::read_f32(W97_F32_ANGLE_SCALE_5360);
    let cosine = w97_call_f64_f64(W97_FN_80237C40, radians as f64) as f32;
    let sine = w97_call_f64_f64(W97_FN_80237BD0, radians as f64) as f32;
    let diagonal = platform::read_f32(W97_F32_ONE_535C);
    let off = platform::read_f32(W97_F32_ZERO_5358);
    platform::write_f32(matrix, cosine);
    platform::write_f32(matrix + 4, -sine);
    platform::write_f32(matrix + 8, off);
    platform::write_f32(matrix + 0x0C, off);
    platform::write_f32(matrix + 0x10, sine);
    platform::write_f32(matrix + 0x14, cosine);
    platform::write_f32(matrix + 0x18, off);
    platform::write_f32(matrix + 0x1C, off);
    platform::write_f32(matrix + 0x20, off);
    platform::write_f32(matrix + 0x24, off);
    platform::write_f32(matrix + 0x28, diagonal);
    platform::write_f32(matrix + 0x2C, off);
}

// ===========================================================================
// 0x80160000 bucket completion: ABI bridges, record builders and GX leaves
// (wave 103)
// ===========================================================================
//
// Wave 97 left 23 bodies of this page untranslated. Three of the reasons it
// gave do not survive a read of the code bytes:
//
// * the eight 4-byte functions are single-instruction leaves: three `blr` and
//   five direct `b <target>` tail branches. Ghidra exports the *target's*
//   decompiled body under the thunk's address, which is why they looked like
//   duplicates of functions that were already translated. The instruction word
//   of each one was read from `work/dol/main.elf`; it is quoted in the doc
//   block so the reading can be checked.
// * the `0x802DCD58..0x802DCDBC` "calls" at the head of several bodies are the
//   Metrowerks register-save / register-restore thunks. They only spill
//   `r21..r31` into the caller's frame and never write `r3`/`r4`, so the
//   `undefined8` Ghidra shows as their result is simply the incoming `r3:r4`
//   argument pair. That turns `unaff_r24..r31` back into parameters and
//   `extraout_r4`/`extraout_f1` back into ordinary arguments.
// * every `psq_l`/`psq_st` in these bodies either carries a load/store type of
//   `0` (single float) or is a plain `f31..f26` save/restore. No quantised data
//   read occurs, so the paired-single problem the earlier pass reported is not
//   present here.
//
// A fourth useful fact: `0x802DCBFC` (reached from the sound-bank cluster and
// now from the list teardown) takes its descriptor in `r12`, not in `r3`.
// [`fn_80161cc0`] is the one body here that receives that descriptor as an
// argument, so it uses `game::bases::interface_call_desc2` and says so.
//
// The 19 bodies below are everything left on the page except four functions
// that stay skips: the quadrant-table wrapper `0x801611C0` and the three large
// interleaved loops `0x8016DEC0`, `0x8016F220` and `0x8016FA10`. The reasons
// are recorded in `docs/COVERAGE_WAVE103_REGISTRY.md`.

// ---------------------------------------------------------------------------
// Small-data float slots (r2 = SDA2_BASE = 0x80433080, r13 = SDA_BASE)
// ---------------------------------------------------------------------------

/// `r2 - 0x5364` (`0x8042DD1C`) - `0.0f`, the plane threshold of
/// [`fn_8016e3a0`].
const W103_F32_ZERO_5364: usize = SDA2_BASE - 0x5364;
/// `r2 - 0x53B4` (`0x8042DCCC`) - `1.0f`, the per-entry scale written by
/// [`fn_80169b20`].
const W103_F32_ONE_53B4: usize = SDA2_BASE - 0x53B4;
/// `r2 - 0x53B8` (`0x8042DCC8`) - `0.0f` used by [`fn_80169600`].
const W103_F32_ZERO_53B8: usize = SDA2_BASE - 0x53B8;
/// `r2 - 0x5390` (`0x8042DCF0`) - `416.0f`, the wide render-mode half width
/// used by [`fn_8016ad30`].
const W103_F32_WIDTH_5390: usize = SDA2_BASE - 0x5390;
/// `r2 - 0x538C` (`0x8042DCF4`) - `304.0f`, the narrow half width.
const W103_F32_WIDTH_538C: usize = SDA2_BASE - 0x538C;
/// `r2 - 0x5388` (`0x8042DCF8`) - `-912.0f`, the top edge of the background
/// quad.
const W103_F32_TOP_5388: usize = SDA2_BASE - 0x5388;
/// `r2 - 0x5384` (`0x8042DCFC`) - `228.0f`, the bottom edge of that quad.
const W103_F32_BOTTOM_5384: usize = SDA2_BASE - 0x5384;
/// `r2 - 0x5380` (`0x8042DD00`) - `0.0f`, the quad's `z`.
const W103_F32_ZERO_5380: usize = SDA2_BASE - 0x5380;
/// `r2 - 0x537C` (`0x8042DD04`) - `1.0f`, the orthographic `near`/`far`.
const W103_F32_ONE_537C: usize = SDA2_BASE - 0x537C;
/// `0xCC008000` - the GX write-gather FIFO. The original writes it with
/// `stfs`; [`platform::write_f32`] keeps the byte order but writes four bytes
/// one at a time, which is why the doc block calls this a model, not a port.
const W103_GX_FIFO: usize = 0xCC00_8000;

/// `r13 - 0x52B8` (`0x8042A3C8`) - default heap used when a caller passes 0
/// (same slot as [`W78_DEFAULT_HEAP`]).
const W103_DEFAULT_HEAP_52B8: usize = SDA_BASE - 0x52B8;

// ---------------------------------------------------------------------------
// Callees outside this lane (address only; no Nintendo symbol is claimed)
// ---------------------------------------------------------------------------

/// `0x8016A3A0` - start one resource load and store the handle at `this + 4`.
const W103_FN_8016A3A0: usize = 0x8016_A3A0;
/// `0x80164500` - build the reader view the two text tables feed (translated).
const W103_FN_80164500: usize = 0x8016_4500;
/// `0x801C04D0` - overwrite a 3x3 block with the identity.
const W103_FN_801C04D0: usize = 0x801C_04D0;
/// `0x801C0CF0` - extract the rotation/scale of a matrix into a 3x3 block.
const W103_FN_801C0CF0: usize = 0x801C_0CF0;
/// `0x801C1350` - build an orthographic projection matrix.
const W103_FN_801C1350: usize = 0x801C_1350;
/// `0x801C1680` - copy a 3x4 matrix block.
const W103_FN_801C1680: usize = 0x801C_1680;
/// `0x801C1850` - blend a 3x4 matrix toward a vector by a weight.
const W103_FN_801C1850: usize = 0x801C_1850;
/// `0x801C37C0` - begin a primitive of `n` vertices.
const W103_FN_801C37C0: usize = 0x801C_37C0;
/// `0x801C40B0` - set the attribute formats of a primitive.
const W103_FN_801C40B0: usize = 0x801C_40B0;
/// `0x801C4070` / `0x801C47C0` - begin and end a display-list section.
const W103_FN_801C4070: usize = 0x801C_4070;
const W103_FN_801C47C0: usize = 0x801C_47C0;
/// `0x801C4A20` - set the cull mode.
const W103_FN_801C4A20: usize = 0x801C_4A20;
/// `0x801C5570` - set a channel-control word.
const W103_FN_801C5570: usize = 0x801C_5570;
/// `0x801C69E0` - load one texture coordinate pair.
const W103_FN_801C69E0: usize = 0x801C_69E0;
/// `0x801C6AC0` / `0x801C6AF0` - bind a texture and set its filter modes.
const W103_FN_801C6AC0: usize = 0x801C_6AC0;
const W103_FN_801C6AF0: usize = 0x801C_6AF0;
/// `0x801C8110` / `0x801C81F0` - blend-factor setup.
const W103_FN_801C8110: usize = 0x801C_8110;
const W103_FN_801C81F0: usize = 0x801C_81F0;
/// `0x801C8250` - set the alpha compare.
const W103_FN_801C8250: usize = 0x801C_8250;
/// `0x801C8790` - set the depth-test function.
const W103_FN_801C8790: usize = 0x801C_8790;
/// `0x801C88F0` - enable depth writes.
const W103_FN_801C88F0: usize = 0x801C_88F0;
/// `0x801C8DC0` - set the blend mode.
const W103_FN_801C8DC0: usize = 0x801C_8DC0;
/// `0x801C8E10` / `0x801C8E40` - enable alpha blending and its logic op.
const W103_FN_801C8E10: usize = 0x801C_8E10;
const W103_FN_801C8E40: usize = 0x801C_8E40;
/// `0x801C9840` - load a projection matrix.
const W103_FN_801C9840: usize = 0x801C_9840;
/// `0x801C9940` - load a position matrix.
const W103_FN_801C9940: usize = 0x801C_9940;
/// `0x801C9A60` - load the texture matrix.
const W103_FN_801C9A60: usize = 0x801C_9A60;
/// `0x801D4460` - allocate from a heap object, `(heap, size, align)`.
const W103_FN_801D4460: usize = 0x801D_4460;
/// `0x801D49A0` - swap the `u16` at `heap + 0x4C` and return the old value.
const W103_FN_801D49A0: usize = 0x801D_49A0;
/// `0x801D5540` - call the allocator's slot 0 with `(heap, size)`.
const W103_FN_801D5540: usize = 0x801D_5540;
/// `0x80239710` - "two six-float boxes overlap" predicate.
const W103_FN_80239710: usize = 0x8023_9710;
/// `0x8023B360` - read the entry count of a loaded resource header.
const W103_FN_8023B360: usize = 0x8023_B360;
/// `0x80248990` - rebuild one object from a source word.
const W103_FN_80248990: usize = 0x8024_8990;
/// `0x8025BEF0` - set a player/voice property (second argument is the id).
const W103_FN_8025BEF0: usize = 0x8025_BEF0;
/// `0x802B99C0` - run the retire pass for `n` objects.
const W103_FN_802B99C0: usize = 0x802B_99C0;
/// `0x802B9270` / `0x802B9290` - allocate / free through a named heap.
const W103_FN_802B9270: usize = 0x802B_9270;
const W103_FN_802B9290: usize = 0x802B_9290;
/// `0x802B9DF0` - allocate a frame-heap buffer and return its base.
const W103_FN_802B9DF0: usize = 0x802B_9DF0;
/// `0x802C1710` - initialise the 2D draw state for a heap.
const W103_FN_802C1710: usize = 0x802C_1710;
/// `0x802C78F0` - fill a `0x20`-byte render-mode descriptor.
const W103_FN_802C78F0: usize = 0x802C_78F0;
/// `0x802CB930` - create the `n`-slot font/cache table for a heap.
const W103_FN_802CB930: usize = 0x802C_B930;
/// `0x802D0AE0` - publish the viewport ratios of two `u16` pairs.
const W103_FN_802D0AE0: usize = 0x802D_0AE0;
/// `0x802D1B50` - commit the 2D draw state.
const W103_FN_802D1B50: usize = 0x802D_1B50;
/// `0x802D2610` - set the two viewport halfwords, a colour word and a heap.
const W103_FN_802D2610: usize = 0x802D_2610;

/// `0x802F86A0`, `0x802F86C8`, `0x802F8FD8` - type-info words passed by
/// address to the `+0x08` vtable slot (`// inferred:` an
/// `IsDerivedFrom`-style test).
const W103_TYPE_802F86A0: u32 = 0x802F_86A0;
const W103_TYPE_802F86C8: u32 = 0x802F_86C8;
const W103_TYPE_802F8FD8: u32 = 0x802F_8FD8;

// ---------------------------------------------------------------------------
// Local call helpers for shapes `crate::platform` does not cover
// ---------------------------------------------------------------------------

/// Calls a no-argument routine and keeps its `r3` result.
///
/// # Safety
/// `addr` must accept no arguments and return a word in `r3`.
unsafe fn w103_call0_u32(addr: usize) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn() -> u32);
    f()
}

/// Calls a routine shaped `(u32, u32) -> f32` (a vtable slot returning a float).
///
/// # Safety
/// `addr` must accept two pointer-shaped words and return in `f1`.
unsafe fn w103_call_u32x2_f32(addr: usize, a: u32, b: u32) -> f32 {
    let f = crate::platform::guest_fn!(addr, fn(u32, u32) -> f32);
    f(a, b)
}

/// Calls a routine shaped `(f32, u32, u32) -> ()`.
///
/// # Safety
/// `addr` must take its float in `f1` and its two words in `r3`/`r4`.
unsafe fn w103_call_f32_u32x2(addr: usize, value: f32, a: u32, b: u32) {
    let f = crate::platform::guest_fn!(addr, fn(f32, u32, u32));
    f(value, a, b)
}

/// Calls a routine shaped `(f32, u32, u32, u32) -> ()`.
///
/// # Safety
/// `addr` must take its float in `f1` and its words in `r3..r5`.
unsafe fn w103_call_f32_u32x3(addr: usize, value: f32, a: u32, b: u32, c: u32) {
    let f = crate::platform::guest_fn!(addr, fn(f32, u32, u32, u32));
    f(value, a, b, c)
}

/// Calls a routine shaped `(f32 x6, u32) -> ()`; the six floats fill `f1..f6`
/// and the pointer lands in `r3`.
///
/// # Safety
/// `addr` must take six floats and one pointer-shaped word.
unsafe fn w103_call_f32x6_u32(addr: usize, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, p: u32) {
    let function = crate::platform::guest_fn!(addr, fn(f32, f32, f32, f32, f32, f32, u32));
    function(a, b, c, d, e, f, p)
}

/// Forwards to `memset` (`0x800046B4`) and returns the destination word.
///
/// The whole function is the single branch `0x4BEA3BE4` at `0x80160AD0`, i.e.
/// `b 0x800046B4`; every incoming register - including the three `memset`
/// arguments `r3`/`r4`/`r5` - reaches the callee untouched.
///
/// # Original
/// - Ghidra: `thunk_FUN_800046b4` at `0x80160AD0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80160ad0_thunk_FUN_800046b4.c`
/// - observed: the export shows the *target's* body (`FUN_80004600();
///   return param_1;`), not the thunk itself; the instruction word
///   `0x4BEA3BE4` read from `work/dol/main.elf` is the branch.
///
/// # Arguments
/// - `dst`: destination block.
/// - `value`: fill byte.
/// - `len`: byte count.
///
/// # Returns
/// `dst`, as `memset` does.
///
/// # Side effects
/// Writes `len` bytes at `dst`.
///
/// # Safety
/// `dst` must be writable for `len` bytes.
///
/// # Confidence
/// high - one branch instruction, decoded from the image bytes.
pub unsafe fn fn_80160ad0(dst: u32, value: u32, len: u32) -> u32 {
    platform::call3(FN_MEMSET, dst, value, len)
}

/// Empty leaf: the whole 4-byte body is `blr`.
///
/// # Original
/// - Ghidra: `FUN_80161740` at `0x80161740` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161740_FUN_80161740.c`
/// - observed: `void FUN_80161740(void) { return; }`; the instruction word at
///   `0x80161740` is `0x4E800020` (`blr`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Confidence
/// high - a single return instruction.
pub unsafe fn fn_80161740() {}

/// Chooses one of four table selectors through the `r12`-descriptor dispatcher.
///
/// `0x802DCBFC(desc)` is called three times. The first call probes with the
/// descriptor in `arg3`; when it succeeds the second call runs against `arg2`'s
/// descriptor and maps its result to a selector (`0 -> 3`, `1 -> 2`, anything
/// else `-> 1`). The third call commits with `arg4`'s descriptor and that
/// selector in `r4`. A failed probe commits selector `0` and returns `0`.
///
/// # Original
/// - Ghidra: `FUN_80161cc0` at `0x80161CC0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80161cc0_FUN_80161cc0.c`
/// - observed: `iVar1 = FUN_802dcbfc(); if (iVar1 == 0) { uVar2 = 0; }
///   else { iVar1 = FUN_802dcbfc(param_1); if (iVar1 == 0) uVar2 = 3; else if
///   (iVar1 == 1) uVar2 = 2; else uVar2 = 1; } FUN_802dcbfc(param_1,uVar2);`
/// - the register trace (`mr r28,r3`, `mr r29,r4`, `mr r12,r5`, `mr r30,r6`)
///   restores the three argument lists Ghidra dropped: the first call gets
///   `(r3 = arg1)`, the second `(r3 = arg1, r12 = arg2)` and the third
///   `(r3 = arg1, r4 = selector, r12 = arg4)`.
///
/// # Arguments
/// - `probe`: word handed to the first dispatch (`r3`).
/// - `second_desc`: descriptor (`r12`) and `r3` value of the second dispatch.
/// - `probe_desc`: descriptor for the first call, the one the original keeps
///   in `r12`.
/// - `commit_desc`: descriptor for the third call.
///
/// # Returns
/// The second dispatch's result, or `0` when the probe failed.
///
/// # Side effects
/// Three indirect dispatches through `0x802DCBFC`.
///
/// # Safety
/// Both descriptors must point at three readable words and the selected slots
/// must accept one word in `r3`.
///
/// # Confidence
/// high - the selector mapping and all three register lists are literal; the
/// three calls now use [`interface_call_desc2`], which resolves the descriptor
/// that the original keeps in `r12` and forwards the incoming `r4` word
/// (`second_desc` on the first two calls, `selector` on the third).
pub unsafe fn fn_80161cc0(probe: u32, second_desc: u32, probe_desc: u32, commit_desc: u32) -> u32 {
    // observed: r3 = probe throughout; r4 = second_desc for the first two
    // calls and the selector for the third; r12 = probe_desc, second_desc,
    // commit_desc in that order.
    let mut result = interface_call_desc2::<u32>(Descriptor::new(probe_desc as usize), probe, second_desc);
    let selector;
    if result == 0 {
        selector = 0;
    } else {
        result = interface_call_desc2::<u32>(Descriptor::new(second_desc as usize), probe, second_desc);
        selector = match result {
            0 => 3,
            1 => 2,
            _ => 1,
        };
    }
    let _ = interface_call_desc2::<u32>(Descriptor::new(commit_desc as usize), probe, selector);
    result
}

/// Empty leaf: the whole 4-byte body is `blr`.
///
/// # Original
/// - Ghidra: `FUN_80162110` at `0x80162110` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162110_FUN_80162110.c`
/// - observed: `void FUN_80162110(void) { return; }`; the instruction word at
///   `0x80162110` is `0x4E800020` (`blr`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Confidence
/// high - a single return instruction.
pub unsafe fn fn_80162110() {}

/// Builds one bank object from the factory table and queues it.
///
/// The `u16` id indexes [`G_SB_BANK_TABLE`]; a null slot returns `0` without
/// stashing anything. Otherwise the four arguments are stashed through
/// [`fn_80162a50`], the entry's vtable slot 0 runs with the entry as `this`,
/// the stash is cleared, and a non-zero product is handed to [`fn_80162930`].
///
/// # Original
/// - Ghidra: `FUN_80162a70` at `0x80162A70` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80162a70_FUN_80162a70.c`
/// - observed: `iVar1 = (param_1 & 0xffff) * 4; if (table[iVar1] == 0) iVar1 = 0;
///   else { FUN_80162a50(); iVar1 = (**table[iVar1])(); FUN_80162a50(0,0,0,0);
///   if (iVar1 != 0) FUN_80162930(iVar1); }`
/// - the two call sites `0x80162B00` (`FUN_80162a70(param_1,param_2 + 0x10)`)
///   and `0x80162B20` (`FUN_80162a70(param_1,0,param_2,param_3)`) show the
///   three words Ghidra dropped; they are exactly the words the first
///   `FUN_80162a50` call stashes.
///
/// # Arguments
/// - `id`: bank id; only the low 16 bits index the table.
/// - `arg_b`, `arg_c`, `flag`: construction words stashed for the factory.
///
/// # Returns
/// The factory's object, or `0` when the slot is empty or the factory failed.
///
/// # Side effects
/// Writes the four SDA stash slots, calls the factory, and may queue the object
/// through [`fn_80162930`].
///
/// # Safety
/// The table and the selected entry must be live, and its vtable slot 0 must
/// accept the entry as `this`.
///
/// # Confidence
/// medium - the index, the null guard and both stash calls are literal; the
/// vtable slot's receiver is recovered from `mr r3, table[id]` before `bctrl`.
pub unsafe fn fn_80162a70(id: u32, arg_b: u32, arg_c: u32, flag: u32) -> u32 {
    let index = ((id & 0xFFFF) as usize) * 4;
    let table = platform::read_u32(G_SB_BANK_TABLE) as usize;
    if table == 0 {
        return 0;
    }
    let entry = platform::read_u32(table + index) as usize;
    if entry == 0 {
        return 0;
    }
    fn_80162a50(id as u16, arg_b, arg_c, flag as u8);
    let vtable = platform::read_u32(entry) as usize;
    let result = platform::call1(platform::read_u32(vtable) as usize, entry as u32);
    fn_80162a50(0, 0, 0, 0);
    if result != 0 {
        fn_80162930(result as usize);
    }
    result
}

/// Empty leaf: the whole 4-byte body is `blr`.
///
/// # Original
/// - Ghidra: `FUN_801644f0` at `0x801644f0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801644f0_FUN_801644f0.c`
/// - observed: `void FUN_801644f0(void) { return; }`; the instruction word at
///   `0x801644F0` is `0x4E800020` (`blr`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// None.
///
/// # Confidence
/// high - a single return instruction.
pub unsafe fn fn_801644f0() {}

/// Creates the two text-object slot tables for the current render mode.
///
/// The build only runs when either table slot is empty. It fills a
/// `0x20`-byte render-mode descriptor, publishes the viewport ratios, commits
/// the 2D draw state, allocates the two `count * 4` tables from `heap`, zeroes
/// them and then calls [`fn_80164500`]. Any failure after the first allocation
/// frees both tables and returns `0`.
///
/// # Original
/// - Ghidra: `FUN_801645f0` at `0x801645F0` (380 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801645f0_FUN_801645f0.c`
/// - observed: `if ((*(r13 - 0x52ac) == 0) || (*(r13 - 0x52a4) == 0)) {
///   FUN_802c78f0(&local_48); ... FUN_802d2610(local_48,local_46,&local_58,
///   (int)uVar3); FUN_802d0ae0(&local_50,&local_54,0); ... iVar2 =
///   FUN_802b9270(param_8 << 2,uVar1,4); *(r13 - 0x52ac) = iVar2; ... }`
/// - the two globals are the same `W97_GLOBAL_TEXT_TABLE_A`/`_TABLE_B` slots
///   [`fn_80164770`] reads, so this is that factory's initialiser.
///
/// # Arguments
/// - `param_1`: heap/owner word forwarded to `0x802D2610` and `0x80164500`.
/// - `heap`: heap the two tables come from (`uVar1`, the prologue's `r4`).
/// - `colour`: pointer to the 4-byte colour word copied into `local_58`.
/// - `param_4`..`param_7`: forwarded to [`fn_80164500`].
/// - `count_a`, `count_b`: entry counts of the two tables.
///
/// # Returns
/// `1` when the tables and the reader view were built, `0` otherwise.
///
/// # Side effects
/// Writes the four text-table globals, allocates and frees heap blocks, and
/// changes the global 2D draw state.
///
/// # Safety
/// `colour` must be readable for four bytes and the heap must be initialised.
///
/// # Confidence
/// medium - every call, argument slot and guard is literal; the callee roles
/// are `// inferred:` from their effects, and `local_48`/`local_46` are fields
/// of the descriptor `0x802C78F0` fills, not uninitialised stack.
pub unsafe fn fn_801645f0(
    param_1: u32,
    heap: u32,
    colour: usize,
    param_4: u32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    count_a: u32,
    count_b: u32,
) -> u32 {
    if platform::read_u32(W97_GLOBAL_TEXT_TABLE_A) == 0
        || platform::read_u32(W97_GLOBAL_TEXT_TABLE_B) == 0
    {
        let mut mode = crate::platform::GuestLocal::new( [0u32; 8]);
        let mut pair_a = crate::platform::GuestLocal::new( [0u16; 2]);
        let mut pair_b = crate::platform::GuestLocal::new( [0u16; 2]);
        let mut word = crate::platform::GuestLocal::new( [0u32; 1]);
        platform::call1(W103_FN_802C78F0, mode.as_mut_ptr() as u32);
        let mode_base = mode.as_ptr() as usize;
        pair_a[0] = platform::read_u16(mode_base + 0x14);
        pair_a[1] = platform::read_u16(mode_base + 0x16);
        pair_b[0] = platform::read_u16(mode_base + 0x02);
        pair_b[1] = platform::read_u16(mode_base + 0x02);
        word[0] = platform::read_u32(colour);
        platform::call4(
            W103_FN_802D2610,
            platform::read_u16(mode_base) as u32,
            platform::read_u16(mode_base + 2) as u32,
            word.as_ptr() as u32,
            param_1,
        );
        platform::call3(
            W103_FN_802D0AE0,
            pair_a.as_ptr() as u32,
            pair_b.as_ptr() as u32,
            0,
        );
        platform::call0(W103_FN_802D1B50);
        platform::call1(W103_FN_802C1710, heap);
        platform::call2(W103_FN_802CB930, 0x20, heap);
        let table_a = platform::call3(W103_FN_802B9270, count_a << 2, heap, 4);
        platform::write_u32(W97_GLOBAL_TEXT_TABLE_A, table_a);
        if table_a != 0 {
            platform::write_u32(W97_GLOBAL_TEXT_COUNT_A, count_a);
            let table_b = platform::call3(W103_FN_802B9270, count_b << 2, heap, 4);
            platform::write_u32(W97_GLOBAL_TEXT_TABLE_B, table_b);
            if table_b != 0 {
                platform::write_u32(W97_GLOBAL_TEXT_COUNT_B, count_b);
                platform::call3(FN_MEMSET, table_a, 0, count_a << 2);
                platform::call3(FN_MEMSET, table_b, 0, count_b << 2);
                if platform::call5(
                    W103_FN_80164500,
                    param_1,
                    param_4,
                    param_5,
                    param_6,
                    param_7,
                ) != 0
                {
                    return 1;
                }
            }
        }
        platform::call1(
            W103_FN_802B9290,
            platform::read_u32(W97_GLOBAL_TEXT_TABLE_A),
        );
        platform::write_u32(W97_GLOBAL_TEXT_TABLE_A, 0);
        platform::call1(
            W103_FN_802B9290,
            platform::read_u32(W97_GLOBAL_TEXT_TABLE_B),
        );
        platform::write_u32(W97_GLOBAL_TEXT_TABLE_B, 0);
        0
    } else {
        0
    }
}

/// Rebuilds one scene object from the word at `source` and re-attaches it.
///
/// The old object is type-tested against `0x802F86A0`, its index in the
/// `+0x1C` array is found, and the renderer's `+0x4C`/`+0x40` slots are entered.
/// The object is then rebuilt from `word`, committed through [`fn_801654b0`],
/// and the `+0x48`/`+0x3C` slots are left with the previously saved float.
///
/// # Original
/// - Ghidra: `FUN_80165280` at `0x80165280` (548 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80165280_FUN_80165280.c`
/// - observed: `piVar6 = *(int **)(*(int *)(iVar4 + 4) + 4); if (piVar6 != 0) {
///   ... (**(*piVar6 + 8))(piVar6,&local_44); ... in_f31 =
///   (**(*unaff_r29 + 0x4c))(unaff_r29,unaff_r28); (**(*unaff_r29 + 0x40))
///   (unaff_r29,unaff_r28); } ... uVar3 = FUN_80248990(iVar4 + 0xc,auStack_34,
///   &local_38,&local_3c,0); *(iVar4 + 4) = uVar3; ... FUN_801654b0(iVar4,
///   param_3,param_4); ... (**(*unaff_r29 + 0x48))(in_f31,unaff_r29,unaff_r28);`
///
/// # Arguments
/// - `param_1`: word passed to [`fn_80168400`] twice (the frame or mode key).
/// - `owner`: object whose `+0x04` word is the scene node rebuilt here.
/// - `source`: pointer to the source word (`local_38`).
/// - `mode`: third argument for [`fn_801654b0`].
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Replaces `owner + 0x04`, enters and leaves the old node's render slots and
/// commits the new node.
///
/// # Safety
/// `owner` must be live, and the old node's vtable slots must exist when it is
/// non-null.
///
/// # Confidence
/// medium - every slot index, the type words and both [`fn_80168400`] reads are
/// literal; the two slot calls Ghidra prints with an empty argument list pass
/// the receiver in `r3`, which is `// inferred:` from the `bctrl` setup.
pub unsafe fn fn_80165280(param_1: u32, owner: usize, source: usize, mode: u32) {
    let mut tag_a: u32 = W103_TYPE_802F86A0;
    let mut tag_b: u32 = W103_TYPE_802F86C8;
    let old = platform::read_u32(platform::read_u32(owner + 4) as usize + 4) as usize;
    let mut renderer = 0usize;
    let mut saved = 0.0f32;
    let mut slot = 0u32;
    if old != 0 {
        let vtable = platform::read_u32(old) as usize;
        if platform::call2(
            platform::read_u32(vtable + 0x08) as usize,
            old as u32,
            (&mut tag_a) as *mut u32 as u32,
        ) != 0
        {
            renderer = old;
        }
        let count = platform::read_u32(renderer + 0x18) as i32;
        let array = platform::read_u32(renderer + 0x1C) as usize;
        while (slot as i32) < count {
            if platform::read_u32(array + (slot as usize) * 4)
                == platform::read_u32(owner + 4)
            {
                break;
            }
            slot += 1;
        }
        let renderer_vtable = platform::read_u32(renderer) as usize;
        saved = w103_call_u32x2_f32(
            platform::read_u32(renderer_vtable + 0x4C) as usize,
            renderer as u32,
            slot,
        );
        platform::call2(
            platform::read_u32(renderer_vtable + 0x40) as usize,
            renderer as u32,
            slot,
        );
    }
    let node = platform::read_u32(owner + 4) as usize;
    let node_vtable = platform::read_u32(node) as usize;
    platform::call1(platform::read_u32(node_vtable + 0x34) as usize, node as u32);
    platform::call2(W103_FN_802B99C0, platform::read_u32(owner + 8), 3);
    let mut scratch = crate::platform::GuestLocal::new( [0u8; 52]);
    let mut first = crate::platform::GuestLocal::new( [platform::read_u32(source)]);
    let mut word = crate::platform::GuestLocal::new( [fn_80168400(param_1 as usize)]);
    let rebuilt = platform::call5(
        W103_FN_80248990,
        (owner + 0x0C) as u32,
        scratch.as_mut_ptr() as u32,
        first.as_mut_ptr() as u32,
        word.as_mut_ptr() as u32,
        0,
    );
    platform::write_u32(owner + 4, rebuilt);
    let mut second = crate::platform::GuestLocal::new( [fn_80168400(param_1 as usize)]);
    let new_node = platform::read_u32(owner + 4) as usize;
    let new_vtable = platform::read_u32(new_node) as usize;
    platform::call2(
        platform::read_u32(new_vtable + 0x30) as usize,
        new_node as u32,
        second.as_mut_ptr() as u32,
    );
    fn_801654b0(owner, source, mode);
    if old != 0 {
        let current = platform::read_u32(owner + 4) as usize;
        let mut candidate = 0usize;
        if current != 0 {
            let vtable = platform::read_u32(current) as usize;
            if platform::call2(
                platform::read_u32(vtable + 0x08) as usize,
                current as u32,
                (&mut tag_b) as *mut u32 as u32,
            ) != 0
            {
                candidate = current;
            }
        }
        let renderer_vtable = platform::read_u32(renderer) as usize;
        w103_call_f32_u32x2(
            platform::read_u32(renderer_vtable + 0x48) as usize,
            saved,
            renderer as u32,
            slot,
        );
        platform::call3(
            platform::read_u32(renderer_vtable + 0x3C) as usize,
            renderer as u32,
            slot,
            candidate as u32,
        );
    }
}

/// Blends one matrix entry toward a source block and reports it.
///
/// Three cases share the `+0x28` entry array indexed by the `u16` at
/// `param_4 + 6`: a plain copy source-to-entry when `+0x20` is clear or the
/// entry weight is zero, a plain copy entry-to-source when `+0x21` is clear,
/// and otherwise a weighted blend that also honours the `0x08`, `0x20` and
/// `0x40` flag bits. The entry is finally handed to the `+0x2C` callback.
///
/// # Original
/// - Ghidra: `FUN_80169600` at `0x80169600` (1068 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169600_FUN_80169600.c`
/// - observed: `uVar2 = *(ushort *)(param_4 + 6); puVar11 = *(int *)(iVar6 +
///   0x28) + uVar2 * 0x4c; if ((*(char *)(iVar6 + 0x20) == '\0') ||
///   (iVar7 = FUN_80168cb0(iVar6 + 4), iVar7 != 0)) { ... } else if
///   (*(char *)(iVar6 + 0x21) == '\0') { ... } else { ... } piVar9 = *(int **)
///   (iVar6 + 0x2c); if (piVar9 != 0) { ... (**(code **)(*piVar9 + 0xc))(...); }`
/// - the nineteen-word copies are the only stores on the first two paths; the
///   blend path scales by `+0x14`/`+0x18`, uses `+0x1C` as the weight and
///   clears `0x7FFFFF97` from the flag word.
///
/// # Arguments
/// - `source`: nineteen-word matrix block (`r3`).
/// - `state`: blend state object (`r4`): `+0x14`/`+0x18` scales, `+0x1C`
///   weight, `+0x20`/`+0x21` mode bytes, `+0x24`/`+0x28`/`+0x2C` entry count,
///   entry array and callback.
/// - `param_3`: pointer to the word handed to the callback.
/// - `param_4`: record whose `+6` word selects the entry.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Copies or blends a nineteen-word matrix, may call the matrix helpers and
/// calls the `+0x2C` callback with the entry.
///
/// # Safety
/// `source` must be readable and writable for 76 bytes, and the entry array
/// must contain the selected index.
///
/// # Confidence
/// medium - all three paths, the flag bits and the callback slot are literal;
/// the two matrix helpers' roles are `// inferred:` from their shapes, and the
/// `f29..f31` saves at the head are the prologue's own spill, not arguments.
pub unsafe fn fn_80169600(source: usize, state: usize, param_3: usize, param_4: usize) {
    let index = platform::read_u16(param_4 + 6) as usize;
    let entries = platform::read_u32(state + 0x28) as usize;
    let entry = entries + index * 0x4C;
    if platform::read_u8(state + 0x20) == 0 || fn_80168cb0(state + 4) {
        for word in 0..19 {
            platform::write_u32(entry + word * 4, platform::read_u32(source + word * 4));
        }
    } else if platform::read_u8(state + 0x21) == 0 {
        for word in 0..19 {
            platform::write_u32(source + word * 4, platform::read_u32(entry + word * 4));
        }
    } else {
        let mut flags = platform::read_u32(source);
        let weight = platform::read_f32(state + 0x1C);
        let towards = platform::read_f32(state + 0x14);
        let scale = platform::read_f32(state + 0x18);
        for i in 1..4 {
            let from = platform::read_f32(source + i * 4);
            let to = platform::read_f32(entry + i * 4);
            let blended = if flags & 8 == 0 {
                from * scale + to * towards
            } else {
                scale + to * towards
            };
            platform::write_f32(source + i * 4, blended);
        }
        let mut stack = crate::platform::GuestLocal::new( [0f32; 12]);
        platform::call2(
            W103_FN_801C1680,
            stack.as_mut_ptr() as u32,
            (entry + 0x1C) as u32,
        );
        let mut vector = crate::platform::GuestLocal::new( [0u32; 4]);
        if flags & 0x20 == 0 {
            platform::call2(
                W103_FN_801C1680,
                vector.as_mut_ptr() as u32,
                (source + 0x1C) as u32,
            );
        } else {
            vector[0] = platform::read_u32(W103_F32_ZERO_53B8);
            vector[1] = platform::read_u32(W103_F32_ZERO_53B8);
            vector[2] = platform::read_u32(W103_F32_ZERO_53B8);
            vector[3] = platform::read_u32(W103_F32_ONE_53B4);
        }
        w103_call_f32_u32x3(
            W103_FN_801C1850,
            weight,
            stack.as_ptr() as u32,
            vector.as_ptr() as u32,
            stack.as_ptr() as u32,
        );
        let mut third = [
            platform::read_f32(source + 0x28),
            platform::read_f32(source + 0x38),
            platform::read_f32(source + 0x48),
        ];
        platform::call2(
            W103_FN_801C0CF0,
            (source + 0x1C) as u32,
            stack.as_ptr() as u32,
        );
        platform::write_f32(source + 0x28, third[0]);
        platform::write_f32(source + 0x38, third[1]);
        platform::write_f32(source + 0x48, third[2]);
        if flags & 0x40 == 0 {
            third[0] *= scale;
            third[1] *= scale;
            third[2] *= scale;
            platform::write_f32(source + 0x28, third[0]);
            platform::write_f32(source + 0x38, third[1]);
            platform::write_f32(source + 0x48, third[2]);
        }
        flags = platform::read_u32(source);
        for i in [0x28usize, 0x38, 0x48] {
            let value = platform::read_f32(source + i);
            let from = platform::read_f32(entry + i);
            platform::write_f32(source + i, value + from * towards);
        }
        platform::write_u32(source, flags & 0x7FFF_FF97);
        platform::write_u32(entry, flags & 0x7FFF_FF97);
        for word in 1..19 {
            platform::write_u32(entry + word * 4, platform::read_u32(source + word * 4));
        }
    }
    let callback = platform::read_u32(state + 0x2C) as usize;
    if callback != 0 {
        let mut word = crate::platform::GuestLocal::new( [platform::read_u32(param_3)]);
        let vtable = platform::read_u32(callback) as usize;
        platform::call4(
            platform::read_u32(vtable + 0x0C) as usize,
            callback as u32,
            index as u32,
            source as u32,
            word.as_mut_ptr() as u32,
        );
    }
}

/// Allocates and initialises the `0x4C`-byte entry array of a loaded bank.
///
/// The count comes from `0x8023B360(header)`; the array is allocated through
/// the heap object's slot 0 and every entry gets `+4`, `+8` and `+0x0C` set to
/// `1.0` plus an identity block at `+0x1C`. `out_size` receives the aligned
/// byte total of the allocation.
///
/// # Original
/// - Ghidra: `FUN_80169b20` at `0x80169B20` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169b20_FUN_80169b20.c`
/// - observed: `if (param_3 == 0) param_3 = *(r13 - 0x52b8); local_38[0] = 0;
///   if (param_4 == 0) param_4 = local_38; iVar3 = FUN_8023b360((int)uVar7);`
///   `*(iVar2 + 0x24) = iVar3; iVar4 = FUN_801d5540(param_3,iVar3 * 0x4c);
///   *(iVar2 + 0x28) = iVar4; ... *param_4 = iVar3 * 0x4c + (*param_4 + 3U &
///   0xfffffffc); ... FUN_801c04d0(iVar4 + 0x1c);`
///
/// # Arguments
/// - `this`: destination object; `+0x24` count, `+0x28` array, `+0x30` heap.
/// - `header`: loaded resource header read by `0x8023B360` (`r4`).
/// - `heap`: heap object, or `0` to use [`W103_DEFAULT_HEAP_52B8`].
/// - `out_size`: optional pointer that receives the aligned byte total.
///
/// # Returns
/// `1` on success, `0` when the allocation failed.
///
/// # Side effects
/// Allocates a heap block, writes the count, the array and the heap into
/// `this`, and initialises every entry.
///
/// # Safety
/// `this` must be writable and the header readable; the heap slot must be a
/// live allocator object.
///
/// # Confidence
/// medium - the count, both alignments, the constant `1.0` and the loop body
/// are literal; the allocator call is the heap object's vtable slot 0.
pub unsafe fn fn_80169b20(this: usize, header: usize, heap: usize, out_size: usize) -> u32 {
    let mut heap = heap;
    let mut out = out_size;
    let mut fallback = crate::platform::GuestLocal::new( [0u32; 1]);
    if heap == 0 {
        heap = platform::read_u32(W103_DEFAULT_HEAP_52B8) as usize;
    }
    fallback[0] = 0;
    if out == 0 {
        out = fallback.as_mut_ptr() as usize;
    }
    let count = platform::call1(W103_FN_8023B360, header as u32);
    platform::write_u32(this + 0x24, count);
    let bytes = count.wrapping_mul(0x4C);
    let entries = platform::call2(W103_FN_801D5540, heap as u32, bytes);
    platform::write_u32(this + 0x28, entries);
    if entries == 0 {
        return 0;
    }
    let one = platform::read_f32(W103_F32_ONE_53B4);
    let aligned = (platform::read_u32(out) + 3) & !3;
    platform::write_u32(out, (bytes.wrapping_add(aligned) + 3) & !3);
    let mut entry = platform::read_u32(this + 0x28) as usize;
    for _ in 0..count {
        platform::write_f32(entry + 4, one);
        platform::write_f32(entry + 8, one);
        platform::write_f32(entry + 0x0C, one);
        platform::call1(W103_FN_801C04D0, (entry + 0x1C) as u32);
        entry += 0x4C;
    }
    platform::write_u32(this + 0x30, heap as u32);
    1
}

/// Starts a resource load on `this` and attaches the loaded object.
///
/// The first loader (`0x8016A3A0`) receives the name word, the heap and the two
/// caller words plus a three-word out buffer. The second call
/// ([`fn_80169b20`]) builds the entry array at `this + 0xC`; its size word is
/// added to the first buffer's count and written back through `out_size`. The
/// `+0x04` sub-object is then type-tested against `0x802F8FD8`, linked to
/// `this + 0xC` and told to start.
///
/// # Original
/// - Ghidra: `FUN_80169d90` at `0x80169D90` (340 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80169d90_FUN_80169d90.c`
/// - observed: `if (param_3 == 0) param_3 = *(r13 - 0x52b8); ... iVar3 =
///   FUN_8016a3a0(piVar2,&local_30,param_3); ... iVar3 = FUN_80169b20(piVar2 +
///   3,&local_34,param_3,piVar5); ... *param_6 = local_28 + local_2c; ...`
///   `piVar5[0x47] = (int)(piVar2 + 3); FUN_8025bef0(piVar5,7);
///   FUN_8016a000(piVar2,0);`
/// - the register trace shows `r6`/`r7` (`param_4`/`param_5`) are forwarded to
///   `0x8016A3A0` as its fourth and fifth arguments, and `r8` (`param_6`) is
///   both its out buffer and the final size pointer.
///
/// # Arguments
/// - `this`: player/bank object.
/// - `source`: pointer to the name/id word.
/// - `heap`: heap object, or `0` for [`W103_DEFAULT_HEAP_52B8`].
/// - `mode`: fourth argument for `0x8016A3A0` (`0` selects its first loader).
/// - `arg5`: fifth argument for `0x8016A3A0`.
/// - `out_size`: optional pointer receiving the two sizes' sum.
///
/// # Returns
/// `1` when the load started, `0` otherwise.
///
/// # Side effects
/// Loads a resource, allocates the entry array, writes `this + 4`, `this + 0x11C`
/// and the caller's size word, and may call the `+0x10` vtable slot.
///
/// # Safety
/// `this` must be live, `source` readable and the heap initialised.
///
/// # Confidence
/// medium - every call and argument register is literal; the `local_28` read is
/// the buffer `0x8016A3A0` fills (not uninitialised stack), and the
/// `piVar5[0x47]` store is reproduced as written even though the type test can
/// leave the base null.
pub unsafe fn fn_80169d90(
    this: usize,
    source: usize,
    heap: usize,
    mode: u32,
    arg5: u32,
    out_size: usize,
) -> u32 {
    let mut heap = heap;
    let out = out_size;
    let mut info = crate::platform::GuestLocal::new( [0u32; 3]);
    let mut total = crate::platform::GuestLocal::new( [0u32; 1]);
    if heap == 0 {
        heap = platform::read_u32(W103_DEFAULT_HEAP_52B8) as usize;
    }
    let buffer = if out == 0 {
        0
    } else {
        info.as_mut_ptr() as u32
    };
    let word = platform::read_u32(source);
    let mut first = crate::platform::GuestLocal::new( [word]);
    let loaded = platform::call6(
        W103_FN_8016A3A0,
        this as u32,
        first.as_mut_ptr() as u32,
        heap as u32,
        mode,
        arg5,
        buffer,
    );
    if loaded == 0 {
        return 0;
    }
    let mut second = crate::platform::GuestLocal::new( [word]);
    let out_ptr = if out == 0 {
        0
    } else {
        total.as_mut_ptr() as u32
    };
    if fn_80169b20(this + 0x0C, second.as_mut_ptr() as usize, heap, out_ptr as usize) == 0
    {
        let vtable = platform::read_u32(this) as usize;
        platform::call1(platform::read_u32(vtable + 0x10) as usize, this as u32);
        return 0;
    }
    if out != 0 {
        platform::write_u32(out, info[0].wrapping_add(total[0]));
    }
    let sub = platform::read_u32(this + 4) as usize;
    let mut target = 0usize;
    if sub != 0 {
        let mut tag: u32 = W103_TYPE_802F8FD8;
        let vtable = platform::read_u32(sub) as usize;
        if platform::call2(
            platform::read_u32(vtable + 0x08) as usize,
            sub as u32,
            (&mut tag) as *mut u32 as u32,
        ) != 0
        {
            target = sub;
        }
    }
    platform::write_u32(target + 0x11C, (this + 0x0C) as u32);
    platform::call2(W103_FN_8025BEF0, target as u32, 7);
    fn_8016a000(this, 0);
    1
}

/// Draws the full-screen background quad for the current render mode.
///
/// The half width is `416.0` when `+0x14` is `1` and `304.0` otherwise. When
/// `+0x13` is set the body builds an orthographic projection over
/// `x in [-w, w]`, `y in [-912, 228]`, loads it with an identity model matrix,
/// configures a textured quad and writes twelve FIFO floats.
///
/// # Original
/// - Ghidra: `FUN_8016ad30` at `0x8016AD30` (532 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016ad30_FUN_8016ad30.c`
/// - observed: `if (*(char *)(param_1 + 0x14) == '\x01') fVar1 = *(r2 - 0x5390);
///   else fVar1 = *(r2 - 0x538c); if (*(char *)(param_1 + 0x13) != '\0') {
///   FUN_801c1350(...); FUN_801c9840(auStack_60,1); FUN_801c04d0(auStack_90);
///   ... _DAT_cc008000 = ...; }`
/// - the twelve FIFO stores read from `work/dol/main.elf` are
///   `(-w, -912, 0) (w, -912, 0) (w, 228, 0) (-w, 228, 0)`.
///
/// # Arguments
/// - `mode`: object whose `+0x10` word is the texture coordinate and whose
///   `+0x13`/`+0x14` bytes select the quad and the width.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Changes the whole GX pipeline state and writes twelve floats to the FIFO.
///
/// # Safety
/// `mode` must be readable at `+0x13`, `+0x14` and `+0x10`.
///
/// # Confidence
/// medium - every call, argument and constant is literal; the graphics callee
/// names are `// inferred:` from their argument shapes and only the addresses
/// are certain.
pub unsafe fn fn_8016ad30(mode: usize) {
    let width = if platform::read_u8(mode + 0x14) == 1 {
        platform::read_f32(W103_F32_WIDTH_5390)
    } else {
        platform::read_f32(W103_F32_WIDTH_538C)
    };
    let flag = platform::read_u8(mode + 0x13);
    if flag == 0 {
        return;
    }
    let mut projection = crate::platform::GuestLocal::new( [0f32; 16]);
    w103_call_f32x6_u32(
        W103_FN_801C1350,
        platform::read_f32(W103_F32_TOP_5388),
        platform::read_f32(W103_F32_BOTTOM_5384),
        -width,
        width,
        platform::read_f32(W103_F32_ZERO_5380),
        platform::read_f32(W103_F32_ONE_537C),
        projection.as_mut_ptr() as u32,
    );
    platform::call2(W103_FN_801C9840, projection.as_ptr() as u32, 1);
    let mut identity = crate::platform::GuestLocal::new( [0f32; 12]);
    platform::call1(W103_FN_801C04D0, identity.as_mut_ptr() as u32);
    platform::call2(W103_FN_801C9940, identity.as_ptr() as u32, 0);
    platform::call1(W103_FN_801C9A60, 0);
    platform::call0(W103_FN_801C4070);
    platform::call0(W103_FN_801C47C0);
    platform::call2(W103_FN_801C37C0, 9, 1);
    platform::call5(W103_FN_801C40B0, 0, 9, 1, 4, 0);
    platform::call1(W103_FN_801C6AC0, 1);
    let mut texcoord = crate::platform::GuestLocal::new( [platform::read_u32(mode + 0x10)]);
    platform::call2(W103_FN_801C69E0, 4, texcoord.as_mut_ptr() as u32);
    platform::call7(W103_FN_801C6AF0, 4, 0, 0, 0, 0, 0, 2);
    platform::call1(W103_FN_801C4A20, 0);
    platform::call1(W103_FN_801C8110, 0);
    platform::call1(W103_FN_801C81F0, 0);
    platform::call1(W103_FN_801C88F0, 1);
    platform::call2(W103_FN_801C8250, 0, 4);
    platform::call4(W103_FN_801C8790, 0, 0xFF, 0xFF, 4);
    if flag == 0xFF {
        platform::call4(W103_FN_801C8DC0, 0, 1, 0, 0xF);
    } else {
        platform::call4(W103_FN_801C8DC0, 1, 4, 5, 0xF);
    }
    platform::call1(W103_FN_801C8E10, 1);
    platform::call1(W103_FN_801C8E40, 1);
    platform::call3(W97_FN_801C8E70, 0, 0, 0);
    platform::call1(W97_FN_801C5860, 2);
    platform::call3(W103_FN_801C5570, 0x80, 0, 4);
    let top = platform::read_f32(W103_F32_TOP_5388);
    let bottom = platform::read_f32(W103_F32_BOTTOM_5384);
    let zero = platform::read_f32(W103_F32_ZERO_5380);
    for (x, y) in [(-width, top), (width, top), (width, bottom), (-width, bottom)] {
        platform::write_f32(W103_GX_FIFO, x);
        platform::write_f32(W103_GX_FIFO, y);
        platform::write_f32(W103_GX_FIFO, zero);
    }
}

/// Tail-branches to the "call one function" mount command factory.
///
/// The whole function is `b 0x8016B8D0` (instruction word `0x4BFFFF80` at
/// `0x8016B950`), so both arguments and the return value pass straight through
/// to [`mount_call_cmd_create`].
///
/// # Original
/// - Ghidra: `thunk_FUN_8016b8d0` at `0x8016B950` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016b950_thunk_FUN_8016b8d0.c`
/// - observed: the export repeats the target's body (`iVar1 =
///   FUN_8016b4a0(0x18); ... FUN_8016b5c0(...)`); the branch word is the real
///   content.
///
/// # Arguments
/// - `callback`, `argument`: forwarded unchanged.
///
/// # Returns
/// The created command, or `0`.
///
/// # Side effects
/// Everything [`mount_call_cmd_create`] does.
///
/// # Safety
/// Same contract as [`mount_call_cmd_create`].
///
/// # Confidence
/// high - one branch instruction to a translated function.
pub unsafe fn fn_8016b950(callback: u32, argument: u32) -> u32 {
    mount_call_cmd_create(callback, argument)
}

/// Tail-branches to the path probe.
///
/// The whole function is `b 0x8016BC30` (instruction word `0x4BFFFF80` at
/// `0x8016BCB0`), so it is [`mount_probe_path`] under a second address.
///
/// # Original
/// - Ghidra: `thunk_FUN_8016bc30` at `0x8016BCB0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016bcb0_thunk_FUN_8016bc30.c`
/// - observed: the export repeats `mount_probe_path`'s body verbatim.
///
/// # Arguments
/// - `path`, `tag_out`: forwarded unchanged.
///
/// # Returns
/// The DVD entry number, or `-1`.
///
/// # Side effects
/// Everything [`mount_probe_path`] does.
///
/// # Safety
/// Same contract as [`mount_probe_path`].
///
/// # Confidence
/// high - one branch instruction to a translated function.
pub unsafe fn fn_8016bcb0(path: usize, tag_out: usize) -> i32 {
    mount_probe_path(path, tag_out)
}

/// Tests one box against six planes.
///
/// Two boxes are first compared with `0x80239710`; if they do not overlap the
/// answer is `0`. Otherwise the six 4-float planes at `box + 0x90` are
/// evaluated at the point chosen from `points` (the near or far corner,
/// selected per axis by comparing the plane normal with the `0.0` threshold),
/// and any plane whose distance is positive rejects the box.
///
/// # Original
/// - Ghidra: `FUN_8016e3a0` at `0x8016E3A0` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e3a0_FUN_8016e3a0.c`
/// - observed: `iVar10 = FUN_80239710(param_2,param_1 + 0x78); if (iVar10 == 0)
///   uVar11 = 0; else { ... fVar2 = *(r2 - 0x5364); iVar10 = 6; do { if
///   (*(param_1 + 0x90) < fVar2) fVar3 = param_2[3]; else fVar3 = *param_2;
///   ... if (fVar2 < (float)(pfVar12[3] + dVar16 * dVar15 + dVar13 * dVar14 +
///   dVar17 * dVar18)) return 0; param_1 = param_1 + 0x10; pfVar12 = pfVar12 +
///   4; } while (iVar10 != 0); }`
/// - the four `psq_l` at `0x8016E450..0x8016E460` that Ghidra marks unreachable
///   carry load type `0` (single float), so the plain-float branch is the real
///   one; the `unaff_GQR0` byte/short branches are decompiler noise.
///
/// # Arguments
/// - `bounds`: object holding the six planes at `+0x90` and a box at `+0x78`.
/// - `points`: six-float corner set (`[min, max]` per axis).
///
/// # Returns
/// `1` when the box is inside all six planes, `0` otherwise.
///
/// # Side effects
/// None; the body only reads.
///
/// # Safety
/// `box` must be readable through `+0xD0` and `points` for 24 bytes.///
/// # Confidence
/// medium - the plane equation, both corner choices and the six-iteration loop
/// are literal; `// inferred:` the six planes are a clipping set and the
/// threshold slot holds `0.0`.
pub unsafe fn fn_8016e3a0(bounds: usize, points: usize) -> u32 {
    if platform::call2(W103_FN_80239710, points as u32, (bounds + 0x78) as u32) == 0 {
        return 0;
    }
    let threshold = platform::read_f32(W103_F32_ZERO_5364);
    for plane in 0..6usize {
        let base = bounds + 0x90 + plane * 0x10;
        let nx = platform::read_f32(base);
        let ny = platform::read_f32(base + 4);
        let nz = platform::read_f32(base + 8);
        let offset = platform::read_f32(base + 0x0C);
        let x = if nx < threshold {
            platform::read_f32(points + 0x0C)
        } else {
            platform::read_f32(points)
        };
        let y = if ny < threshold {
            platform::read_f32(points + 0x10)
        } else {
            platform::read_f32(points + 4)
        };
        let z = if nz < threshold {
            platform::read_f32(points + 0x14)
        } else {
            platform::read_f32(points + 8)
        };
        if threshold < offset + nx * x + ny * y + nz * z {
            return 0;
        }
    }
    1
}

/// Tail-branches to the heap switch.
///
/// The whole function is `b 0x802B9170` (instruction word `0x4814AC80` at
/// `0x8016E4F0`), i.e. [`W97_FN_802B9170`] under a second address: it installs
/// the argument as the current heap and returns the previous one.
///
/// # Original
/// - Ghidra: `thunk_FUN_802b9170` at `0x8016E4F0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e4f0_thunk_FUN_802b9170.c`
/// - observed: the export shows the target's body (`FUN_801b2e60(&DAT_80423890);
///   uVar1 = *(r13 + -0x4870); *(r13 + -0x4870) = param_1;
///   FUN_801b2f40(&DAT_80423890);`), which is the heap switch.
///
/// # Arguments
/// - `heap`: heap to install.
///
/// # Returns
/// The heap that was current before the call.
///
/// # Side effects
/// Rewrites the current-heap slot under the global lock.
///
/// # Safety
/// `heap` must be a heap object the allocator accepts.
///
/// # Confidence
/// high - one branch instruction to a documented callee.
pub unsafe fn fn_8016e4f0(heap: u32) -> u32 {
    platform::call1(W97_FN_802B9170, heap)
}

/// Tail-branches to the frame-heap allocator.
///
/// The whole function is `b 0x802B9DF0` (instruction word `0x4814B510` at
/// `0x8016E8E0`). The `+ 0x3c` and the inner `0x801D54A0` call that Ghidra
/// prints belong to the *target*, not to this 4-byte thunk.
///
/// # Original
/// - Ghidra: `thunk_FUN_802b9df0` at `0x8016E8E0` (4 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016e8e0_thunk_FUN_802b9df0.c`
/// - observed: `iVar1 = FUN_801d54a0(); return iVar1 + 0x3c;` is the callee's
///   body.
///
/// # Returns
/// The buffer base the callee returns in `r3`.
///
/// # Side effects
/// Whatever the allocator does.
///
/// # Safety
/// The frame heap must be initialised.
///
/// # Confidence
/// medium - the branch is certain; the return type comes from the callee's
/// `+ 0x3c` arithmetic, which implies a word result.
pub unsafe fn fn_8016e8e0() -> u32 {
    w103_call0_u32(W103_FN_802B9DF0)
}

/// Packs twelve floats into a destination block.
///
/// The eight values in `f1..f8` and the four stack floats at `8(r1)..0x14(r1)`
/// are stored as `f32` in order at `dst + 0..0x2C`. The original has no frame,
/// so the four stack values are its own stack arguments.
///
/// # Original
/// - Ghidra: `FUN_8016eba0` at `0x8016EBA0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016eba0_FUN_8016eba0.c`
/// - observed: `*param_9 = (float)param_1; ... param_9[7] = (float)param_8;
///   param_9[8] = in_stack_00000008; ... param_9[0xb] = in_stack_00000014;`
/// - the disassembly is four `lfs` from `8(r1)`/`12(r1)`/`16(r1)`/`20(r1)`
///   followed by twelve `stfs` to `0(r3)..0x2C(r3)` and `blr`.
///
/// # Arguments
/// - `dst`: twelve-float destination.
/// - `values`: the twelve values in store order.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Twelve float stores.
///
/// # Safety
/// `dst` must be writable for 48 bytes.
///
/// # Confidence
/// high - twelve stores with literal offsets and no arithmetic.
pub unsafe fn fn_8016eba0(dst: usize, values: [f32; 12]) {
    for (index, value) in values.iter().enumerate() {
        platform::write_f32(dst + index * 4, *value);
    }
}

/// Allocates and fills a variable-length record.
///
/// The record is `0x1D` bytes of header plus a `len`-byte name: four `u16`
/// fields, two words, one float, one byte, then the zeroed-then-copied name.
/// The allocation comes from the heap stored at `*handle` after its `u16` flag
/// at `+0x4C` is set to zero, with alignment `4` when `kind == 1` and `-4`
/// otherwise.
///
/// # Original
/// - Ghidra: `FUN_8016fd30` at `0x8016FD30` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016fd30_FUN_8016fd30.c`
/// - observed: `puVar1 = (undefined4 *)FUN_802dcb5c(); uVar3 = 0xfffffffc;
///   if (param_7 == 1) uVar3 = 4; dVar4 = extraout_f1; FUN_801d49a0(*puVar1,0);
///   puVar2 = FUN_801d4460(*puVar1,param_9 + 0x20,uVar3); if (puVar2 != 0) {
///   *puVar2 = extraout_r4; ... FUN_800046b4((int)puVar2 + 0x1d,0,param_9);
///   FUN_80004364((int)puVar2 + 0x1d,param_8,param_9); ... }`
/// - `0x802DCD5C` is a register-save thunk, so `puVar1` is `r3` (the handle)
///   and `extraout_r4`/`extraout_f1` are the `r4` argument and the `f1`
///   argument; the disassembly shows `stfd f31,...` before the thunk.
///
/// # Arguments
/// - `handle`: pointer to the heap pointer.
/// - `tag`: first `u16` of the record (`r4`).
/// - `param_3`: second `u16`.
/// - `param_4`, `param_5`: the two words at `+8` and `+0xC`.
/// - `param_6`: byte at `+0x1C`.
/// - `value`: float stored at `+0x10` (the original's `f1` argument).
/// - `kind`: selects positive (`4`) or negative (`-4`) alignment.
/// - `source`: pointer to the `len` name bytes.
/// - `len`: name length.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Allocates from the heap, clears the heap flag, and writes `0x1D + len`
/// bytes.
///
/// # Safety
/// `handle` must point at a live heap object and `source` must be readable for
/// `len` bytes.
///
/// # Confidence
/// medium - every field offset, both alignment values, the clear and the copy
/// are literal; the record's field roles are `// inference:` from their widths.
pub unsafe fn fn_8016fd30(
    handle: usize,
    tag: u16,
    param_3: u16,
    param_4: u32,
    param_5: u32,
    param_6: u8,
    value: f32,
    kind: i32,
    source: u32,
    len: u32,
) {
    let heap = platform::read_u32(handle);
    let align: i32 = if kind == 1 { 4 } else { -4 };
    platform::call2(W103_FN_801D49A0, heap, 0);
    let record = platform::call3(W103_FN_801D4460, heap, len.wrapping_add(0x20), align as u32);
    if record != 0 {
        let base = record as usize;
        platform::write_u16(base, tag);
        platform::write_u16(base + 2, param_3);
        platform::write_u16(base + 4, kind as u16);
        platform::write_u16(base + 6, len as u16);
        platform::write_u32(base + 8, param_4);
        platform::write_u32(base + 0x0C, param_5);
        platform::write_f32(base + 0x10, value);
        platform::write_u8(base + 0x1C, param_6);
        platform::call3(FN_MEMSET, (base + 0x1D) as u32, 0, len);
        platform::call3(MEMCPY, (base + 0x1D) as u32, source, len);
    }
}

// ===========================================================================
// Bucket closure: quadrant table, player frame and text pages (wave 110)
// ===========================================================================
//
// Wave 103 closed nineteen of the twenty-three wave-97 skips and left four
// bodies on this page. All four are below. Each one was read twice: first as
// the Ghidra C export, then as the instruction words in `work/dol/main.elf`,
// because the export drops arguments at indirect calls and at single-argument
// tail branches. The two readings agree on every branch, call argument and
// store; where the export had shown a call with "missing" arguments, the
// listing names the float register that still held the value.
//
// Three of the four skips failed for the same reason: a *tail* call or a
// dropped register. `0x801611C0` is reached by `b` from `0x80160E80`, so its
// `f1`/`f2` are that function's `fsubs` results; its one-argument table lookups
// are `f1`/`f2` pairs whose second member Ghidra could not recover. The
// player-frame loop and the text-page loop keep their whole state in `r13`
// slots and in two BSS tables, which the export shows as `unaff_r13`.
//
// | address | size | Rust | plain-English role |
// |---|---|---|---|
// | `0x801611C0` | 436 B | [`fn_801611c0`] | quadrant angle from two direction components |
// | `0x8016DEC0` | 880 B | [`fn_8016dec0`] | rebuild a box's corners, faces and bounds |
// | `0x8016F220` | 484 B | [`fn_8016f220`] | advance the four-player table one frame |
// | `0x8016FA10` | 604 B | [`fn_8016fa10`] | draw and age one text page |

// ---------------------------------------------------------------------------
// Constants (r2 = SDA2_BASE = 0x80433080, r13 = SDA_BASE = 0x8042F680)
// ---------------------------------------------------------------------------

/// `r2 - 0x5468` (`0x8042DC18`) - `FLT_EPSILON` (`0x34000000`), the
/// "component is zero" threshold of [`fn_801611c0`].
const W110_F32_EPS_5468: usize = SDA2_BASE - 0x5468;
/// `r2 - 0x5464` (`0x8042DC1C`) - `0.0f`, the sign threshold of
/// [`fn_801611c0`].
const W110_F32_ZERO_5464: usize = SDA2_BASE - 0x5464;
/// `r2 - 0x5350` (`0x8042DD30`) - `0.0f`, the value [`fn_8016f220`] writes
/// over a retired player's motion record.
const W110_F32_ZERO_5350: usize = SDA2_BASE - 0x5350;
/// `r2 - 0x5348` (`0x8042DD38`) - `0.8f`, the first of the two motion scales
/// [`fn_8016fa10`] adds to a glyph's position.
const W110_F32_SCALE_5348: usize = SDA2_BASE - 0x5348;
/// `r2 - 0x5344` (`0x8042DD3C`) - `0.1f`, the second motion scale.
const W110_F32_SCALE_5344: usize = SDA2_BASE - 0x5344;
/// `r2 - 0x5340` (`0x8042DD40`) - the `f64` `2^52 + 0x8000`, the bias the
/// signed-halfword-to-double idiom subtracts from the `0x43300000`-tagged
/// value. Read back from `main.elf`.
const W110_F64_HALF_BIAS_5340: usize = SDA2_BASE - 0x5340;

/// `r13 - 0x5210` (`0x8042A470`) - the frame counter [`fn_8016f220`] bumps
/// once per call.
const W110_G_FRAME_5210: usize = SDA_BASE - 0x5210;
/// `r13 - 0x5220` (`0x8042A460`) - the object [`fn_8016f220`] reads its
/// players from (the same slot [`fn_8016f410`] calls a virtual slot on).
const W110_G_PLAYER_MGR_5220: usize = SDA_BASE - 0x5220;
/// `r13 - 0x5214` (`0x8042A46C`) - four per-player "drawn last frame" bytes,
/// indexed by player.
const W110_G_DRAW_FLAGS_5214: usize = SDA_BASE - 0x5214;
/// `r13 - 0x5204` (`0x8042A47C`) - the rotating slot selector of
/// [`fn_8016f220`].
const W110_G_SLOT_5204: usize = SDA_BASE - 0x5204;
/// `r13 - 0x521C` (`0x8042A464`) - index copied into the published slot.
const W110_G_CURSOR_521C: usize = SDA_BASE - 0x521C;
/// `r13 - 0x5218` (`0x8042A468`) - the pointer published as "current player".
const W110_G_CURRENT_5218: usize = SDA_BASE - 0x5218;

/// `0x80377C88` - four published player pointers, one per player index.
const W110_PLAYER_TABLE: usize = 0x8037_7C88;
/// `0x80377CA8` - four `0x18`-byte motion records: `x`, `y`, `dx`, `dy`,
/// `ddx`, `ddy`, each an `f32`.
const W110_PLAYER_MOTION: usize = 0x8037_7CA8;

/// `0x802BD370` - return player `index` of a manager object.
const W110_FN_802BD370: usize = 0x802B_D370;
/// `0x802BC6E0` - `memset(ptr, 0, 0xB0)`.
const W110_FN_802BC6E0: usize = 0x802B_C6E0;
/// `0x802BC800` - reset one player's motion state (five `f32` plus cursors).
const W110_FN_802BC800: usize = 0x802B_C800;
/// `0x8022DDE0` - begin a 2D text pass into a caller-supplied context.
const W110_FN_8022DDE0: usize = 0x8022_DDE0;
/// `0x8022DE40` - end a 2D text pass.
const W110_FN_8022DE40: usize = 0x8022_DE40;
/// `0x8022D5C0` - open a text pass with two mode words.
const W110_FN_8022D5C0: usize = 0x8022_D5C0;
/// `0x8022D3C0` - scale the context's two position factors by one `double`.
const W110_FN_8022D3C0: usize = 0x8022_D3C0;
/// `0x8022D9A0` - commit the four colour bytes at `context + 0x18`.
const W110_FN_8022D9A0: usize = 0x8022_D9A0;
/// `0x8022C9C0` - flush the context's colour change.
const W110_FN_8022C9C0: usize = 0x8022_C9C0;
/// `0x8022EED0` - draw `count` glyph bytes at `context + 0x10` factors.
const W110_FN_8022EED0: usize = 0x8022_EED0;
/// `0x8016FE40` - retire one text record (body lives in `src/ui/menu.rs`).
const W110_FN_8016FE40: usize = 0x8016_FE40;
/// `0x801C07A0` - fill five entries of the 12-float block at its second
/// argument from the first (`param_2[0]`, `[1]`, `[3]`, `[5]`, `[7]`).
const W110_FN_801C07A0: usize = 0x801C_07A0;
/// `0x801C0500` - fill seven entries of the 12-float block at its second
/// argument from the first (`param_2[0]`, `[1]`, `[3]`, `[5]`, `[7]`, `[9]`,
/// `[0xB]`).
const W110_FN_801C0500: usize = 0x801C_0500;
/// `0x801C1160` - combine the first two 12-float arguments into the block at
/// the third (the export writes `param_3[0]`, `[1]`, `[2]`, ...).
const W110_FN_801C1160: usize = 0x801C_1160;
/// `0x802385D0` - normalise `(a - c) x (b - c)` into three floats.
const W110_FN_802385D0: usize = 0x8023_85D0;
/// `0x80238690` - grow a six-float `min`/`max` box by `n` points.
const W110_FN_80238690: usize = 0x8023_8690;

/// Calls a routine shaped `(f64, pointer) -> u32`.
///
/// # Safety
/// `addr` must accept one double in `f1` and one word in `r3`.
unsafe fn w110_call_f64_u32(addr: usize, value: f64, second: u32) -> u32 {
    let f = crate::platform::guest_fn!(addr, fn(f64, u32) -> u32);
    f(value, second)
}

/// Turns a two-component direction into the angle table's quadrant value.
///
/// The two components are compared against `FLT_EPSILON` and `0.0f`, then run
/// through the table lookup [`fn_80161180`] with the operands ordered so the
/// ratio is non-negative. The result is a signed halfword in the table's unit:
/// `0x4000` is a quarter turn and `0x8000` a half turn, so the eight branches
/// are the octants plus the axis cases.
///
/// # Original
/// - Ghidra: `FUN_801611c0` at `0x801611C0` (436 bytes)
/// - Evidence: `work/ghidra/main-gb/export/801611c0_FUN_801611c0.c`
/// - observed: the export shows `FUN_80161180(param_1)`, `FUN_80161180()`,
///   `FUN_80161180(param_2,param_1)` and `FUN_80161180(-param_2,param_1)`
///   depending on the branch. The listing resolves every one of them: the
///   dropped operand is whichever of `f1`/`f2` the branch did not overwrite.
/// - observed: `0x80160E80` reaches this address by a tail `b` with the two
///   `fsubs` results in `f1`/`f2`, so the arguments are single precision.
///
/// # Arguments
/// - `param_1`: first direction component (in `f1`).
/// - `param_2`: second direction component (in `f2`).
///
/// # Returns
/// The table entry, sign-extended from `i16` exactly as the closing `extsh`
/// does.
///
/// # Side effects
/// One angle-table read per call; no game state is written.
///
/// # Safety
/// The table at `0x80328B48` and the two SDA2 words must be mapped.
///
/// # Confidence
/// high - every octant branch, call argument pair and 16-bit adjustment is
/// literal in the listing; the `cror` guards around the comparisons only fold
/// the unordered case into the same branch, so NaN input is not modelled.
pub unsafe fn fn_801611c0(param_1: f32, param_2: f32) -> i32 {
    let eps = platform::read_f32(W110_F32_EPS_5468);
    let zero = platform::read_f32(W110_F32_ZERO_5464);
    // The accumulator stays a 16-bit value; the original sign-extends it once,
    // in the epilogue.
    let value: i32;
    if param_1.abs() < eps {
        // |f1| under epsilon: only the sign of f2 survives.
        value = if param_2 < zero { 0x8000 } else { 0 };
    } else if param_2.abs() < eps {
        value = if param_1 < zero { 0xC000 } else { 0x4000 };
    } else if param_1 < zero {
        if param_2 >= zero {
            let p1 = -param_1;
            if p1 <= param_2 {
                value = -(fn_80161180(p1 as f64, param_2 as f64) as i32);
            } else {
                value = fn_80161180(param_2 as f64, p1 as f64) as i32 - 0x4000;
            }
        } else {
            let p1 = -param_1;
            if -param_2 < p1 {
                value = -0x4000 - fn_80161180(-param_2 as f64, p1 as f64) as i32;
            } else {
                value = fn_80161180(p1 as f64, -param_2 as f64) as i32 - 0x8000;
            }
        }
    } else if param_2 < zero {
        if param_1 <= -param_2 {
            value = 0x8000 - fn_80161180(param_1 as f64, -param_2 as f64) as i32;
        } else {
            value = fn_80161180(-param_2 as f64, param_1 as f64) as i32 + 0x4000;
        }
    } else if param_2 < param_1 {
        value = 0x4000 - fn_80161180(param_2 as f64, param_1 as f64) as i32;
    } else {
        value = fn_80161180(param_1 as f64, param_2 as f64) as i32;
    }
    (value as u16) as i16 as i32
}

/// Rebuilds one box object's corners, face directions and bounding box.
///
/// The six floats are the box's extents: `param_1`/`param_2` are the `y`
/// minimum/maximum, `param_3`/`param_4` the `x` pair and `param_5`/`param_6`
/// the `z` pair, which are stored negated. The body writes the eight corners
/// of that box, transforms each of them in place by the matrix at `context`,
/// then builds ten three-float face directions from corner triples and finally
/// grows the six-float `min`/`max` box at `+0x78` over the eight corners.
///
/// `flag` selects how the far corners are placed: when it is non-zero they are
/// the plain corners, when it is zero their `x`/`y` are scaled by
/// `param_6 / param_5` around the origin, which is the projection the caller
/// wants for a tapered box.
///
/// # Original
/// - Ghidra: `FUN_8016dec0` at `0x8016DEC0` (880 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016dec0_FUN_8016dec0.c`
/// - observed: `FUN_801c07a0(r4, &stack_108)`, `FUN_801c0500(r4, r3)`, then
///   ten `FUN_802385d0(obj + 0x30 ... obj + 0xE0, ...)` calls around one
///   eight-iteration `FUN_801c1160(&stack_108, p, p)` loop, and
///   `FUN_80238690(obj + 0x78, &stack_108, 8)`.
/// - observed: the local block at `r1 + 0x48` is filled as
///   `(p3,p1,-p5) (p4,p1,-p5) (p4,p2,-p5) (p3,p2,-p5)` and the block at
///   `r1 + 0x78` as the same four corners with `-p6`; together they are the
///   eight corners in the order the eight-iteration loop walks.
/// - observed: the `if (param_9 == 0)` branch divides `f31` by `f30`
///   (`fdivs f6,f31,f30`) and multiplies the four far `x`/`y` values by it.
///
/// # Arguments
/// - `param_1`, `param_2`: `y` minimum and maximum.
/// - `param_3`, `param_4`: `x` minimum and maximum.
/// - `param_5`, `param_6`: `z` near and far extents, stored negated at
///   `this + 0x70` / `this + 0x74`.
/// - `this`: the box object (`r3`).
/// - `context`: object passed to the two transform copies (`r4`).
/// - `flag`: `r5`; zero selects the scaled far corners.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// `0x801C0500` writes the lower part of `this` (`+0x00..+0x2C`); the body then
/// writes `this + 0x30 .. this + 0xE0` (ten three-float slots), the six floats
/// at `this + 0x78` and the two extents at `this + 0x70` / `+0x74`. Calls five
/// helpers outside this file.
///
/// # Safety
/// `this` must be writable through `+0xE3` and `context` must be the object
/// `0x801C07A0`/`0x801C0500` expect. The local corner array is 96 bytes.
///
/// # Confidence
/// medium - every corner expression, call argument triple, store offset and
/// the `param_6 / param_5` scale is literal; the *role* of the ten slots as
/// face directions is inferred from `0x802385D0`, whose body normalises
/// `(a - c) x (b - c)` into three floats.
pub unsafe fn fn_8016dec0(
    param_1: f32,
    param_2: f32,
    param_3: f32,
    param_4: f32,
    param_5: f32,
    param_6: f32,
    this: usize,
    context: usize,
    flag: i32,
) {
    let mut matrix = crate::platform::GuestLocal::new( [0.0f32; 12]);
    let mut one = crate::platform::GuestLocal::new( [0.0f32; 3]);
    // Eight corners x three floats, laid out as the original's two
    // twelve-float blocks back to back.
    let mut corner = crate::platform::GuestLocal::new( [0.0f32; 24]);

    let near = -param_5;
    let far = -param_6;

    platform::call2(
        W110_FN_801C07A0,
        context as u32,
        matrix.as_mut_ptr() as usize as u32,
    );
    platform::call2(W110_FN_801C0500, context as u32, this as u32);

    platform::write_f32(this + 0x70, near);
    platform::write_f32(this + 0x74, far);

    // Near four corners: (x,y) in the order the face helpers index them.
    corner[0] = param_3;
    corner[1] = param_1;
    corner[2] = near;
    corner[3] = param_4;
    corner[4] = param_1;
    corner[5] = near;
    corner[6] = param_4;
    corner[7] = param_2;
    corner[8] = near;
    corner[9] = param_3;
    corner[10] = param_2;
    corner[11] = near;

    // Far four corners: the z is shared by both paths.
    corner[14] = far;
    corner[17] = far;
    corner[20] = far;
    corner[23] = far;

    // The eight corner addresses, in the order the helpers index them. They
    // are taken once because the calls below pass three of them at a time.
    let p0 = corner_ptr(&mut corner, 0);
    let p1 = corner_ptr(&mut corner, 3);
    let p2 = corner_ptr(&mut corner, 6);
    let p3 = corner_ptr(&mut corner, 9);
    let p4 = corner_ptr(&mut corner, 12);
    let p5 = corner_ptr(&mut corner, 15);
    let p6 = corner_ptr(&mut corner, 18);
    let p7 = corner_ptr(&mut corner, 21);

    if flag == 0 {
        // Tapered box: the far corners are scaled about the origin.
        let scale = param_6 / param_5;
        let scaled_3 = scale * param_3;
        let scaled_1 = scale * param_1;
        let scaled_4 = scale * param_4;
        let scaled_2 = scale * param_2;
        platform::write_f32(this + 0x70, near);
        platform::write_f32(this + 0x74, far);
        corner[12] = scaled_3;
        corner[13] = scaled_1;
        corner[15] = scaled_4;
        corner[16] = scaled_1;
        corner[18] = scaled_4;
        corner[19] = scaled_2;
        corner[21] = scaled_3;
        corner[22] = scaled_2;

        // The near-side faces use the three zeros at `r1 + 8` as their first
        // point, exactly as the original's `local_118` block does.
        let zero = platform::read_f32(W103_F32_ZERO_5364);
        one[0] = zero;
        one[1] = zero;
        one[2] = zero;
        let origin = one.as_mut_ptr() as usize as u32;

        w110_plane(this + 0x30, origin, p3, p0);
        w110_plane(this + 0x40, origin, p1, p2);
        w110_plane(this + 0x50, origin, p0, p1);
        w110_plane(this + 0x60, origin, p2, p3);

        w110_transform_corners(&mut matrix, &mut corner);

        platform::call3(
            W110_FN_801C1160,
            matrix.as_mut_ptr() as usize as u32,
            origin,
            origin,
        );
        w110_plane(this + 0x90, origin, p3, p0);
        w110_plane(this + 0xA0, origin, p1, p2);
        w110_plane(this + 0xD0, origin, p0, p1);
        w110_plane(this + 0xE0, origin, p2, p3);
    } else {
        // Plain box: the far corners repeat the near x/y values.
        corner[12] = param_3;
        corner[13] = param_1;
        corner[15] = param_4;
        corner[16] = param_1;
        corner[18] = param_4;
        corner[19] = param_2;
        corner[21] = param_3;
        corner[22] = param_2;

        w110_plane(this + 0x30, p3, p7, p0);
        w110_plane(this + 0x40, p1, p5, p2);
        w110_plane(this + 0x50, p0, p4, p1);
        w110_plane(this + 0x60, p2, p6, p3);

        w110_transform_corners(&mut matrix, &mut corner);

        w110_plane(this + 0x90, p3, p7, p0);
        w110_plane(this + 0xA0, p1, p5, p2);
        w110_plane(this + 0xD0, p0, p4, p1);
        w110_plane(this + 0xE0, p2, p6, p3);
    }

    platform::call3(
        W110_FN_80238690,
        (this + 0x78) as u32,
        corner.as_mut_ptr() as usize as u32,
        8,
    );
    w110_plane(this + 0xB0, p0, p1, p2);
    w110_plane(this + 0xC0, p4, p7, p6);
}

/// Calls `0x802385D0(dst, a, b, c)` with three corner pointers.
///
/// # Safety
/// `dst` must be writable for three floats and the three pointers must each
/// name three readable floats.
unsafe fn w110_plane(dst: usize, a: u32, b: u32, c: u32) {
    platform::call4(W110_FN_802385D0, dst as u32, a, b, c);
}

/// `corner.as_mut_ptr() + index` as the `u32` the original passes.
///
/// # Safety
/// `index` must be inside `corner`.
unsafe fn corner_ptr(corner: &mut crate::platform::GuestLocal<[f32; 24]>, index: usize) -> u32 {
    corner.as_mut_ptr().add(index) as usize as u32
}

/// Runs the eight in-place `0x801C1160` calls over the two corner blocks.
///
/// # Safety
/// `matrix` must hold a readable 3x4 transform and `corner` the eight
/// three-float points the loop rewrites.
unsafe fn w110_transform_corners(matrix: &mut crate::platform::GuestLocal<[f32; 12]>, corner: &mut crate::platform::GuestLocal<[f32; 24]>) {
    let dst = matrix.as_mut_ptr() as usize as u32;
    let mut index = 0usize;
    while index < 8 {
        let point = corner.as_mut_ptr().add(index * 3) as usize as u32;
        platform::call3(W110_FN_801C1160, dst, point, point);
        index += 1;
    }
}

/// Advances the four-player table by one frame.
///
/// The call bumps the frame counter, runs the manager's `+0x10 -> +0x08`
/// virtual slot, then walks the four players. A player whose `+0xB1C` bit 0 is
/// clear and whose "drawn" byte is set is retired: its `+0x18` block is
/// cleared, its motion record is zeroed, its task slot is cleared through
/// [`fn_8016f500`] and the drawn byte is reset. A player whose bit is set has
/// its `+0x6C`/`+0x70` position differenced against the six-float record at
/// `0x80377CA8`; the record keeps the position, the first difference and the
/// second difference, and the drawn byte is set on the first frame.
///
/// The mode word at `r13 - 0x5208` (the one [`fn_8016f640`] installs) then
/// selects which players get a [`fn_8016f5d0`] task: mode 1 ticks everyone,
/// larger modes tick one slot and then its parity class, and the slot counter
/// wraps at the mode value. Finally the pointer of the rotated slot is
/// published at `r13 - 0x5218`.
///
/// # Original
/// - Ghidra: `FUN_8016f220` at `0x8016F220` (484 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016f220_FUN_8016f220.c`
/// - observed: `lwz r0,0xb1c(r3)` then `rlwinm r0,r0,0,31,31` and `beq`,
///   which is the `(byte & 1) == 0` test the export prints.
/// - observed: the six motion stores at `0(r27)`/`4`/`8`/`0xC`/`0x10`/`0x14`
///   with `r27 = 0x80377CA8 + index * 0x18`, and the six zero stores of the
///   retire path in the same slots.
/// - observed: `lwz r0,-21000(r13)` then `cmpi`/`cmpli` against `1` and `3`;
///   the `-21000` slot is the mode word `fn_8016f640` writes.
///
/// # Arguments
/// None; every input is an `r13` word or one of the two BSS tables.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes the frame counter, the four pointer slots at `0x80377C88`, the four
/// motion records at `0x80377CA8`, the four bytes at `r13 - 0x5214`, the slot
/// counter and the published pointer. Calls the manager slot, the player
/// lookup, two player resets and up to four task starts.
///
/// # Safety
/// The manager must be live and every returned player pointer must be a real
/// player record. The original has no null check.
///
/// # Confidence
/// medium - the loop, both branches, all six record stores, the mode selector
/// and the publication are literal; the virtual slot's prototype is unknown
/// (the original still has the old frame count in `r4` at the call) and the
/// meaning of the `+0xB1C` bit is inferred from its use as an "active" flag.
pub unsafe fn fn_8016f220() {
    let manager = platform::read_u32(W110_G_PLAYER_MGR_5220) as usize;
    let frame = platform::read_u32(W110_G_FRAME_5210);
    platform::write_u32(W110_G_FRAME_5210, frame.wrapping_add(1));
    let vtable = platform::read_u32(manager + 0x10) as usize;
    let update = platform::read_u32(vtable + 8) as usize;
    platform::call0(update);

    let zero = platform::read_f32(W110_F32_ZERO_5350);
    let mut index = 0usize;
    while index < 4 {
        let player = platform::call2(W110_FN_802BD370, manager as u32, index as u32) as usize;
        platform::write_u32(W110_PLAYER_TABLE + index * 4, player as u32);
        let drawn = W110_G_DRAW_FLAGS_5214 + index;
        let slot = W110_PLAYER_MOTION + index * 0x18;
        if platform::read_u8(player + 0xB1C) & 1 == 0 {
            if platform::read_u8(drawn) != 0 {
                platform::call1(W110_FN_802BC6E0, (player + 0x18) as u32);
                platform::call1(W110_FN_802BC800, player as u32);
                // observed: the six stores are in this order in the listing.
                platform::write_f32(slot, zero);
                platform::write_f32(slot + 4, zero);
                platform::write_f32(slot + 0x10, zero);
                platform::write_f32(slot + 0x14, zero);
                platform::write_f32(slot + 8, zero);
                platform::write_f32(slot + 0xC, zero);
                fn_8016f500(index);
                platform::write_u8(drawn, 0);
            }
        } else {
            let new_y = platform::read_f32(player + 0x70);
            let old_y = platform::read_f32(slot + 4);
            let new_x = platform::read_f32(player + 0x6C);
            let old_x = platform::read_f32(slot);
            let delta_y = new_y - old_y;
            let was_drawn = platform::read_u8(drawn);
            platform::write_f32(slot, new_x);
            let delta_x = new_x - old_x;
            platform::write_f32(slot + 4, new_y);
            let old_dx = platform::read_f32(slot + 8);
            let old_dy = platform::read_f32(slot + 0xC);
            let delta_dx = delta_x - old_dx;
            let delta_dy = delta_y - old_dy;
            // observed: the original also copies new_x/new_y into its
            // outgoing-argument area; nothing reads them back, so they are not
            // modelled here.
            platform::write_f32(slot + 0x10, delta_dx);
            platform::write_f32(slot + 0x14, delta_dy);
            platform::write_f32(slot + 8, delta_x);
            platform::write_f32(slot + 0xC, delta_y);
            if was_drawn == 0 {
                platform::write_u8(drawn, 1);
            }

            let mode = platform::read_u32(G_8016F640_INSTANCE);
            if mode != 0 {
                let alternate = platform::read_u32(W110_G_SLOT_5204);
                let tick = mode == 1
                    || alternate == index as u32
                    || (mode < 4 && (alternate & 1) == (index as u32 & 1));
                if tick {
                    fn_8016f5d0(index as i32);
                }
            }
        }
        index += 1;
    }

    let mode = platform::read_u32(G_8016F640_INSTANCE);
    if mode != 0 {
        let next = platform::read_u32(W110_G_SLOT_5204).wrapping_add(1);
        platform::write_u32(W110_G_SLOT_5204, next);
        if next > mode {
            platform::write_u32(W110_G_SLOT_5204, 0);
        }
    }
    let cursor = platform::read_u32(W110_G_CURSOR_521C) as usize;
    platform::write_u32(
        W110_G_CURRENT_5218,
        platform::read_u32(W110_PLAYER_TABLE + cursor * 4),
    );
}

/// Draws and ages one text page's glyph records.
///
/// The page at `param_1` is active only when its `+0x15` byte and its `+0x04`
/// word are both non-zero; an inactive page only closes the text context. Each
/// record in the list at `page + 8` is drawn twice when the page's `+0x14`
/// byte is set: once with the `+0x0C` colour word, the context's `+0x44` scale
/// set to `0.8 * scale` and the position shifted by `0.1 * scale` on both axes,
/// then again with the `+0x08` colour and the plain position. The two signed
/// halfwords at `+0x00` and `+0x02` are the `x`/`y` position, `+0x10` the float
/// scale, `+0x1D` the glyph bytes and `+0x06` their count.
///
/// After the draw the `+0x04` counter is decremented and the record is retired
/// through `0x8016FE40` when it reaches zero.
///
/// # Original
/// - Ghidra: `FUN_8016fa10` at `0x8016FA10` (604 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8016fa10_FUN_8016fa10.c`
/// - observed: `local_58 = 0x43300000; local_50 = 0x43300000` plus
///   `CONCAT44(...) ^ 0x80000000` around `lfd` and a `dVar7` subtraction; the
///   listing shows `lfd` from the two tagged slots and a `fsubs` of the
///   `2^52 + 0x8000` word, so the pair is the signed-halfword-to-float idiom.
/// - observed: `lha r5,2(r31)` and `lha r0,0(r31)` feed `local_90`/`local_94`,
///   and `dVar6 * *(float *)(psVar4 + 8)` is the `0.1f` scale added only on
///   the first pass.
/// - observed: the four byte copies `lbz 12(r1)` .. `stb 43(r1)` move the
///   colour word into the context's four bytes at `+0x18`.
///
/// # Arguments
/// - `page`: the text page (`r3`): `+0x04` font/context word, `+0x08` list
///   head, `+0x14` draw flag, `+0x15` active flag.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Draws into a 2D context on the stack, then writes each record's `+0x04`
/// counter and may unlink and free the record through `0x8016FE40`.
///
/// # Safety
/// The page, its list and every record must be live; the record's `+0x1D`
/// bytes must be readable for `+0x06` bytes.
///
/// # Confidence
/// medium - the loop, both colour copies, the halfword conversion, the two
/// position stores and the countdown are literal; the record's field roles are
/// cross-checked against [`fn_8016fd30`], which fills the same offsets, but the
/// meaning of the two colour words is inferred.
pub unsafe fn fn_8016fa10(page: usize) {
    // The 2D context is opaque: its callees write as far as `+0x60`, so 0x100
    // bytes cover the whole object with room to spare.
    let mut context = crate::platform::GuestLocal::new( [0u32; 64]);
    let ctx = context.as_mut_ptr() as usize;

    platform::call1(W110_FN_8022DDE0, ctx as u32);
    if platform::read_u8(page + 0x15) == 0 || platform::read_u32(page + 4) == 0 {
        platform::call2(W110_FN_8022DE40, ctx as u32, 0xFFFF_FFFF);
        return;
    }
    platform::call3(W110_FN_8022D5C0, ctx as u32, 0, 0);

    let scale_draw = platform::read_f32(W110_F32_SCALE_5348);
    let scale_shift = platform::read_f32(W110_F32_SCALE_5344);

    let mut record = platform::call2(FN_UT_LIST_NEXT, (page + 8) as u32, 0) as usize;
    while record != 0 {
        if platform::read_u8(page + 0x14) != 0 {
            let scale = platform::read_f32(record + 0x10);
            w110_call_f64_u32(W110_FN_8022D3C0, scale as f64, ctx as u32);

            // observed: the original turns the alpha byte into `byte != 0`
            // with `neg`/`or`/`rlwinm` before storing it at `+0x43`.
            let alpha = platform::read_u8(record + 0x1C);
            platform::write_u8(ctx + 0x43, (alpha != 0) as u8);
            platform::write_f32(ctx + 0x44, scale_draw * scale);
            write_colour(ctx, platform::read_u32(record + 0x0C));
            platform::call1(W110_FN_8022D9A0, ctx as u32);
            platform::call1(W110_FN_8022C9C0, ctx as u32);

            let x = w110_half_to_f32(platform::read_u16(record));
            let y = w110_half_to_f32(platform::read_u16(record + 2));
            let shift = scale_shift * scale;
            platform::write_f32(ctx + 0x30, y + shift);
            platform::write_f32(ctx + 0x2C, x + shift);
            platform::call3(
                W110_FN_8022EED0,
                ctx as u32,
                (record + 0x1D) as u32,
                platform::read_u16(record + 6) as i16 as i32 as u32,
            );

            write_colour(ctx, platform::read_u32(record + 8));
            platform::call1(W110_FN_8022D9A0, ctx as u32);
            platform::call1(W110_FN_8022C9C0, ctx as u32);
            platform::write_f32(ctx + 0x30, y);
            platform::write_f32(ctx + 0x2C, x);
            platform::call3(
                W110_FN_8022EED0,
                ctx as u32,
                (record + 0x1D) as u32,
                platform::read_u16(record + 6) as i16 as i32 as u32,
            );
        }

        let remaining = platform::read_u16(record + 4) as i16;
        let next_remaining = remaining.wrapping_sub(1);
        platform::write_u16(record + 4, next_remaining as u16);
        let next = platform::call2(FN_UT_LIST_NEXT, (page + 8) as u32, record as u32) as usize;
        if next_remaining <= 0 {
            platform::call2(W110_FN_8016FE40, page as u32, record as u32);
        }
        record = next;
    }

    platform::call2(W110_FN_8022DE40, ctx as u32, 0xFFFF_FFFF);
}

/// Copies a big-endian colour word into the context's four bytes at `+0x18`.
///
/// # Safety
/// `ctx` must be writable for `0x1C` bytes.
unsafe fn write_colour(ctx: usize, colour: u32) {
    platform::write_u8(ctx + 0x18, (colour >> 24) as u8);
    platform::write_u8(ctx + 0x19, (colour >> 16) as u8);
    platform::write_u8(ctx + 0x1A, (colour >> 8) as u8);
    platform::write_u8(ctx + 0x1B, colour as u8);
}

/// Reads one big-endian `f64` word pair, as `lfd` would.
///
/// # Safety
/// `addr` must be readable for eight bytes.
unsafe fn w110_read_f64(addr: usize) -> f64 {
    let high = platform::read_u32(addr) as u64;
    let low = platform::read_u32(addr + 4) as u64;
    f64::from_bits((high << 32) | low)
}

/// Converts a signed halfword with the `0x43300000` tag idiom.
///
/// The original stores `0x43300000` above `value ^ 0x8000`, loads the pair as a
/// double and subtracts the `2^52 + 0x8000` constant at `r2 - 0x5340`. The
/// result is the signed halfword value; the wrap is intentional because the
/// tag makes every input a positive 32-bit integer.
///
/// # Safety
/// The bias word at `W110_F64_HALF_BIAS_5340` must be mapped.
unsafe fn w110_half_to_f32(value: u16) -> f32 {
    let tagged = 0x4330_0000u64 << 32 | (value as u32 ^ 0x8000) as u64;
    let raw = f64::from_bits(tagged) - w110_read_f64(W110_F64_HALF_BIAS_5340);
    raw as f32
}

// ===========================================================================
// Wave 277 lane E (`_E277`) - gap batch 7, registry-band bodies
// ===========================================================================
//
// Three untabled gap bodies from the wave-266 candidate set, re-verified
// against `work/dol/main.elf` with the current documented extents. Selection:
// `work/wave277/e_pick.mjs`; ELF re-walk and disassembly: `e_verify.mjs`,
// `e_verify_stdout.txt` and `e_verify_rest.txt`. All three start with
// `9421FFF0`, each descent ends on exactly one `blr`, none overlaps a
// documented extent and none is a pointer-run interior.

/// Deleting destructor of the interpolator value object.
///
/// Slot 0 of [`INTERP_VALUE_VTABLE`] (`0x80329610`); the word at `0x80329618`
/// names this body and [`fn_80168b40`] installs the table.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80168CD0`.
/// - Extent: `0x80168CD0..0x80168D10` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x80168D0C`; two zero words precede it at
///   `0x80168CC8..0x80168CD0` and the next body starts at `0x80168D10`.
/// - Reach: no `b` or `bl` in the executable segments targets it. The one data
///   word naming it, `0x80329618`, is slot 0 (`+0x08`) of
///   [`INTERP_VALUE_VTABLE`].
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`; `bc 4,1,+0x08` (ble);
///   `bl 0x802B9280`; `mr r3,r31`; `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`;
///   `mtlr r0`; `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: interpolator value object, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live object of the [`INTERP_VALUE_VTABLE`] class.
///
/// # Confidence
/// high - the guard, the call and the return are literal, and the vtable slot
/// proves the destructor role.
pub unsafe fn fn_80168cd0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Deleting destructor recorded between the two registry init entries.
///
/// The body sits in the `0x8016F150..0x8016F7D0` init block, directly before
/// [`fn_8016f7d0`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8016F790`.
/// - Extent: `0x8016F790..0x8016F7D0` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x8016F7CC`; two zero words precede it at
///   `0x8016F788..0x8016F790` and [`fn_8016f7d0`] starts at `0x8016F7D0`.
/// - Reach: no `b` or `bl` in the executable segments targets it and no data
///   word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`; `bc 4,1,+0x08` (ble);
///   `bl 0x802B9280`; `mr r3,r31`; `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`;
///   `mtlr r0`; `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: object to delete, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live registry-band object.
///
/// # Confidence
/// medium - the body shape is literal; no table slot or caller names it, so the
/// owning class is not identified.
pub unsafe fn fn_8016f790(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

/// Deleting destructor recorded just before the last registry init entry.
///
/// The body sits directly before [`fn_8016ff60`], the last init entry of the
/// `0x8016C6F0..0x8016FF60` query group.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8016FF20`.
/// - Extent: `0x8016FF20..0x8016FF60` (64 bytes, 15 instructions), ending at
///   the `blr` at `0x8016FF5C`; two zero words precede it at
///   `0x8016FF18..0x8016FF20` and [`fn_8016ff60`] starts at `0x8016FF60`.
/// - Reach: no `b` or `bl` in the executable segments targets it and no data
///   word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `bc 12,2,+0x10` (beq); `cmpi cr0,0,r4,0`; `bc 4,1,+0x08` (ble);
///   `bl 0x802B9280`; `mr r3,r31`; `lwz r31,0x0C(r1)`; `lwz r0,0x14(r1)`;
///   `mtlr r0`; `addi r1,r1,0x10`; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: object to delete, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` through the shared deallocator `0x802B9280`.
///
/// # Safety
/// `this` must be null or a live registry-band object.
///
/// # Confidence
/// medium - the body shape is literal; no table slot or caller names it, so the
/// owning class is not identified.
pub unsafe fn fn_8016ff20(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call1(0x802B_9280, this as u32);
    }
    this
}

// ===========================================================================
// Wave 279 lane E (`_E279`) - gap batch 9, `0x80160000` page bodies
// ===========================================================================
//
// Four untabled gap bodies from the wave-266 candidate set, re-verified against
// `work/dol/main.elf`. Selection: `work/wave279/e_pick.mjs`; ELF re-walk and
// disassembly: `e_verify.mjs` and `e_verify_stdout.txt`. All four start with
// `9421FFF0`, each recursive descent ends on exactly one `blr`, none overlaps a
// documented extent and no pointer-run word names any start.

/// Runs a null-terminated table of initializer function pointers.
///
/// This is the CodeWarrior static-initializer walker that REL entry points call
/// with their module table (`d_profileNP` passes `0x80502098`, `d_basesNP`
/// `0x807C60F4`). It loads the next word, stops when it is zero, calls the
/// entry through `mtctr`/`bcctrl` and advances by four bytes. `4E800421` is a
/// call that falls through, so the walk continues after each initializer.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80160A20`.
/// - Extent: `0x80160A20..0x80160A64` (68 bytes, 15 instructions), ending at
///   the `blr` at `0x80160A60`; three zero words precede it at
///   `0x80160A14..0x80160A20` (a lone `4E800020` word sits at `0x80160A10`)
///   and the twin body below starts at `0x80160A70`.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it. Outside `main.elf` the REL entries call it directly.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0x0C(r1)`; `mr r31,r3`; `48000010 b 0x80160A44`;
///   loop: `7D8903A6 mtctr r12`; `4E800421 bcctrl 20,0l` (call);
///   `3BFF0004 addi r31,r31,4`; `819F0000 lwz r12,0(r31)`;
///   `2C0C0000 cmpi cr0,0,r12,0`; `4082FFEC bc 4,2,-0x14` (bne, back to the
///   `mtctr`); epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `table`: word address of the first initializer pointer. The last word
///   before the terminator must be zero.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls every non-null entry in the table in order.
///
/// # Safety
/// `table` must be readable until its zero word and every entry must be a
/// callable `void(void)` routine.
///
/// # Confidence
/// high - the null test and the call-through-`r12` loop are explicit.
pub unsafe fn fn_80160a20(table: usize) {
    let mut cursor = table;
    loop {
        let init = platform::read_u32(cursor) as usize;
        if init == 0 {
            return;
        }
        platform::call0(init);
        cursor += 4;
    }
}

/// Byte-for-byte twin of [`fn_80160a20`], eight entries later.
///
/// The two 68-byte bodies differ only in their addresses; both are the same
/// null-terminated initializer-table walker. No `b`, `bl` or data word
/// distinguishes them, and no exported REL entry calls this second copy
/// directly, so its purpose beyond the twin is `// unknown:`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80160A70`.
/// - Extent: `0x80160A70..0x80160AB4` (68 bytes, 15 instructions), ending at
///   the `blr` at `0x80160AB0`; three zero words precede it at
///   `0x80160A64..0x80160A70` and three more follow it at
///   `0x80160AB4..0x80160AC0`, where the next body (the eight-byte
///   `addi r3,r13,-0x6244` / `blr` leaf) starts.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Observed: identical to [`fn_80160a20`] with every branch target eight
///   bytes higher: `48000010 b 0x80160A94`; `mtctr r12`; `4E800421 bcctrl`
///   (call); `addi r31,r31,4`; `lwz r12,0(r31)`; `cmpi cr0,0,r12,0`;
///   `4082FFEC bc 4,2,-0x14` (bne); epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `table`: word address of the first initializer pointer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Calls every non-null entry in the table in order.
///
/// # Safety
/// Same contract as [`fn_80160a20`].
///
/// # Confidence
/// high for the body (a literal copy of the twin); low for why two copies
/// exist.
pub unsafe fn fn_80160a70(table: usize) {
    let mut cursor = table;
    loop {
        let init = platform::read_u32(cursor) as usize;
        if init == 0 {
            return;
        }
        platform::call0(init);
        cursor += 4;
    }
}

/// Constructs the raw form of a list-bearing registry record.
///
/// Clears the words at `+0x00`/`+0x04`, clears the state byte at `+0x14`, and
/// initialises the twelve-byte intrusive-list header at `+0x08` through the
/// shared NW4R list constructor [`FN_UT_LIST_INIT`] with link offset `0x14`.
/// The "live" constructor [`fn_8016f7d0`] performs the same steps and then
/// sets `+0x14`/`+0x15` to `1`; this body is the pair's zero-state half.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8016F740`.
/// - Extent: `0x8016F740..0x8016F788` (72 bytes, 17 instructions), ending at
///   the `blr` at `0x8016F784`; one zero word precedes it at `0x8016F73C`
///   (the body before that ends in `b 0x802DCA98`) and the next body is the
///   deleting destructor [`fn_8016f790`] at `0x8016F790`.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `38800014 li r4,0x14`;
///   `stw r0,0x14(r1)`; `38000000 li r0,0`; `stw r31,0x0C(r1)`;
///   `mr r31,r3`; `90030004 stw r0,4(r3)`; `98030014 stb r0,0x14(r3)`;
///   `90030000 stw r0,0(r3)`; `38630008 addi r3,r3,8`;
///   `480B9665 bl 0x80228DD0`; `mr r3,r31`; epilogue; `4E800020 bclr 20,0`
///   (blr).
///
/// # Arguments
/// - `this`: record to initialise; must provide at least `0x15` bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes `+0x00`, `+0x04`, `+0x08..+0x14`.
///
/// # Safety
/// `this..this+0x15` must be writable and must not alias the list constructor's
/// own state.
///
/// # Confidence
/// high - the three clears, the list size/stride and the return are literal.
pub unsafe fn fn_8016f740(this: usize) -> usize {
    platform::write_u32(this + 4, 0);
    platform::write_u8(this + 0x14, 0);
    platform::write_u32(this, 0);
    platform::call2(FN_UT_LIST_INIT, (this + 8) as u32, 0x14);
    this
}

/// Constructs a second raw list-bearing registry record.
///
/// Byte-for-byte twin of [`fn_8016f740`], directly before its own deleting
/// destructor [`fn_8016ff20`]. The same clears (`+0x00`, `+0x04`, `+0x14`) and
/// the same [`FN_UT_LIST_INIT`] call with link offset `0x14` are present. The
/// instruction words match [`fn_8016f740`] except for the `bl` displacement,
/// which differs only because the body sits at a different address.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8016FED0`.
/// - Extent: `0x8016FED0..0x8016FF18` (72 bytes, 17 instructions), ending at
///   the `blr` at `0x8016FF14`; one zero word precedes it at `0x8016FECC` and
///   the next body is the deleting destructor [`fn_8016ff20`] at
///   `0x8016FF20`.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `38800014 li r4,0x14`;
///   `stw r0,0x14(r1)`; `38000000 li r0,0`; `stw r31,0x0C(r1)`;
///   `mr r31,r3`; `90030004 stw r0,4(r3)`; `98030014 stb r0,0x14(r3)`;
///   `90030000 stw r0,0(r3)`; `38630008 addi r3,r3,8`;
///   `480B8ED5 bl 0x80228DD0`; `mr r3,r31`; epilogue; `4E800020 bclr 20,0`
///   (blr).
///
/// # Arguments
/// - `this`: record to initialise.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// Writes `+0x00`, `+0x04`, `+0x08..+0x14`.
///
/// # Safety
/// `this..this+0x15` must be writable.
///
/// # Confidence
/// high - a literal copy of [`fn_8016f740`].
pub unsafe fn fn_8016fed0(this: usize) -> usize {
    platform::write_u32(this + 4, 0);
    platform::write_u8(this + 0x14, 0);
    platform::write_u32(this, 0);
    platform::call2(FN_UT_LIST_INIT, (this + 8) as u32, 0x14);
    this
}

// ===========================================================================
// Wave 286 lane A (`_A286`): the eight uncovered methods of the `d_profile`
// load-handle vtable `0x80328AE8`
// ===========================================================================
//
// ## Shape and identity
//
// `0x80328AE8`/`0x80328AEC` are both zero and `0x80328AF0..0x80328B18` is a
// ten-word method array: the usual two-word class header (`offset-to-top`,
// `typeinfo`) followed by ten slots. The installed pointer is the header, so
// dispatch is `[[this+0x0C] + 8 + 4*slot]` - the handle class keeps its `vptr`
// at `+0x0C`, not at `+0x00`. The constructor [`fn_80160080`] stores this
// header word at `handle+0x0C`; its base constructor [`fn_8015fef0`] stores the
// base header `0x80328B18` at the same offset first.
//
// The class is the profile-file load handle used by the resource registry:
//
// * the constructor and the neighbouring bodies are the wave-41
//   `resource::registry` handle page (`0x80160000` bucket), and the two
//   already-translated slots are ledgered `resource::registry`
//   (`0x80160110`, `0x80160800`);
// * slot 5 and slot 8 format the two paths this class opens,
//   `"%s/%sNP.rel"` (`0x80328AC0`) and `"%s/%sNP.map"` (`0x80328ACC`), which
//   sit directly before the header;
// * `0x80328AD8..0x80328AE8` is a four-word pointer table to
//   `"????"`/`"MEM"`/`"ARAM"`/`"DVD"` (`0x80429424..0x80429444`); slot 3
//   indexes it with the low two bits of `handle+0x24`;
// * the derived table `0x80310E98` (also ten slots, at `0x80310EA0`) is
//   followed by the literal `"d_profile"` at `0x80310EC8`. Its constructor
//   `0x8008F870` calls [`fn_80160080`] and then replaces the `vptr` with
//   `0x80310E98`; its slot 0 is the deleting destructor `0x8008F810` that
//   calls [`fn_80160110`], and its slot 8 `0x8008F8B0` is the four-byte tail
//   branch `b 0x801605B0`. That fixes this array as the base table of the
//   `d_profile` handle family.
//
// `0x80328AE8` itself is *not* named by an adjacent literal: the
// `"%s/%sNP.rel"` / `"%s/%sNP.map"` pair and the four memory-region pointers
// are its layout neighbours, and the `"d_profile"` name belongs to the derived
// `0x80310E98` table. The lane brief's `d_profile` label therefore refers to
// the family, not to a literal flush against this header.
//
// ## Install sites
//
// | address | store | value |
// |---|---|---|
// | `0x801600B4` | `stw r3,0x0C(r29)` | `0x80328AE8` (in [`fn_80160080`]) |
// | `0x8015FEFC` | `stw r4,0x0C(r3)` | `0x80328B18` (base ctor [`fn_8015fef0`]) |
//
// The `lis`/`addi` pairs are `0x801600A8`/`0x801600B0` (`0x8033` + `-0x7518`)
// and `0x8015FEF0`/`0x8015FEF8` (`0x8033` + `-0x74E8`). No data word holds
// either header or the first slot, so the constructors are the only
// materialisation sites.
//
// ## Containment
//
// ```text
// 0x80328AC0  "%s/%sNP.rel"                 ends 0x80328ACC
// 0x80328ACC  "%s/%sNP.map"                 ends 0x80328AD8
// 0x80328AD8  four region-name pointers     0x80429424/2C/30/38
// 0x80328AE8  0x00000000 0x00000000         this header
// 0x80328AF0  ten method words               ends 0x80328B18
// 0x80328B18  0x00000000 0x00000000         base-class header
// 0x80328B20  ten base method words          ends 0x80328B48
// 0x80328B48  0x0000000A + u16 table          (the wave-93 angle table)
// ```
//
// No slot crosses a boundary and all ten targets are distinct main-DOL text
// addresses. None of the eight bodies below lies inside a `functions.tsv` row
// or a translated extent (`inside=[]` for all eight; the nearest previous
// `functions.tsv` row ends 4..0x2B8 bytes before each).
//
// ## Census
//
// | | |
// |---|---|
// | slots | 10 (10 distinct) |
// | covered before this lane | 2 (`0x80160110`, `0x80160800`) |
// | uncovered before this lane | 8 (5 prologue-shaped + 3 leaves) |
// | prologue-shaped words in the array | 7 (`stwu` + `mflr`) |
// | targets outside main-DOL text | 0 |
// | bodies below / bytes | 8 / 1372 |
//
// ## Extent rule
//
// Extents were measured with `work/wave286/a_vt.mjs` and re-checked with
// `work/wave286/a_verify.mjs`. `bcctrl` and conditional `bc`/`bclr`/`bcctr`
// fall through; an unconditional `bclr 20,0`, unconditional `bcctr`, `bctr` or
// a forward `b` out of the body ends a path. Every one of the eight extents
// closes on the returning `bclr`.
//
// ## Scan-first
//
// None of the eight addresses had a `fn_<addr>` body, a semantic body, a
// `functions.tsv` row or a ledger row. The wave-284 survey named the five
// prologue-shaped targets only as remaining work.
//
// ---------------------------------------------------------------------------
// Constants introduced by this lane (suffixed `_A286`)
// ---------------------------------------------------------------------------

/// `0x80328AC0`: `"%s/%sNP.rel"`, the path format used by [`fn_80160300`].
///
/// observed: `lis r28,0x8033` at `0x80160348` and `addi r5,r28,-30016` at
/// `0x80160380`; the resolved word is `0x80328AC0`.
const W286_FMT_REL_80328AC0: usize = 0x8032_8AC0;

/// `0x80328ACC`: `"%s/%sNP.map"`, the path format used by [`fn_801605b0`].
///
/// observed: `lis r5,0x8033` / `addi r5,r5,-30004` at `0x80160730`/`0x80160734`.
const W286_FMT_MAP_80328ACC: usize = 0x8032_8ACC;

/// `0x80328AD8`: four-word pointer table to `"????"`, `"MEM"`, `"ARAM"` and
/// `"DVD"` (`0x80429424`, `0x8042942C`, `0x80429430`, `0x80429438`).
const W286_REGION_NAMES_80328AD8: usize = 0x8032_8AD8;

/// `0x80160230`: four-word virtual thunk
/// `((*(vptr + 0x0C)) + 0x1C)(this)` used as the mount command's callback.
///
/// observed: `lwz r12,12(r3)`; `lwz r12,28(r12)`; `mtlr r12`; `4E800420 bctr`.
const W286_FN_80160230: usize = 0x8016_0230;

/// `0x8016B8D0`: allocate and queue a `0x18`-byte mount call command.
const W286_FN_8016B8D0: usize = 0x8016_B8D0;

/// `0x8016B7E0`: destroying delete for a mount command.
const W286_FN_8016B7E0: usize = 0x8016_B7E0;

/// `0x8016B4F0`: wait for the queued mount command to finish.
const W286_FN_8016B4F0: usize = 0x8016_B4F0;

/// `0x8016BF70`: create a mount archive command from a path buffer.
const W286_FN_8016BF70: usize = 0x8016_BF70;

/// `0x801B1F20`: link a reader into the command chain (short form).
const W286_FN_801B1F20: usize = 0x801B_1F20;

/// `0x801B1F30`: link a reader at a fixed offset into the command chain.
const W286_FN_801B1F30: usize = 0x801B_1F30;

/// `r13 - 0x6268`: name of the REL currently being opened, the first `%s` of
/// the `.rel` path.
const W286_SDA_REL_NAME: usize = SDA_BASE - 0x6268;

/// `r13 - 0x6264`: name of the map currently being opened, the first `%s` of
/// the `.map` path.
const W286_SDA_MAP_NAME: usize = SDA_BASE - 0x6264;

/// `r13 - 0x5300`: mount command currently in flight.
const W286_SDA_PENDING_MOUNT: usize = SDA_BASE - 0x5300;

// ---------------------------------------------------------------------------
// slot 1 - 0x80160180
// ---------------------------------------------------------------------------

/// Returns the owner word of a handle.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80160180`.
/// - Extent: `0x80160180..0x80160188` (8 bytes, 2 words); two zero words
///   follow, then [`fn_80160190`] at `0x80160190`.
/// - Table: slot 1 of the `0x80328AF0` array (header `0x80328AE8`; word
///   `0x80328AF4`, offset `+0x004`).
/// - observed: `lwz r3,0x20(r3)`; `4E800020 bclr 20,0`.
///
/// # Arguments
/// - `this`: live handle.
///
/// # Returns
/// The word stored at `this + 0x20` (the owner set by [`fn_80160080`]).
///
/// # Safety
/// `this` must be readable for `0x24` bytes.
///
/// # Confidence
/// high - a single load and return.
pub unsafe fn fn_80160180(this: usize) -> u32 {
    platform::read_u32(this + 0x20)
}

// ---------------------------------------------------------------------------
// slot 2 - 0x801608B0
// ---------------------------------------------------------------------------

/// Reports the handle's current byte size.
///
/// The base size is read by `archive::fn_802b9700` from `owner - 0x0C`. When
/// the handle has a `+0x18` sticky object, its `+0x20` word is added.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801608B0`.
/// - Extent: `0x801608B0..0x8016090C` (92 bytes, 23 words); one zero word
///   follows, then [`fn_80160910`] at `0x80160910`.
/// - Table: slot 2 of the `0x80328AF0` array (word `0x80328AF8`, offset
///   `+0x008`).
/// - observed: `lwz r0,0x14(r3)`; null -> `li r3,0`; `bl 0x802B9700`;
///   `lwz r0,0x18(r31)`; `lwz r4,0x14(r31)`; `lwz r0,0x20(r4)`;
///   `add r3,r3,r0`; epilogue `bclr`.
///
/// # Arguments
/// - `this`: live handle; `+0x14` is the reader, `+0x18` the sticky object.
///
/// # Returns
/// `0` without a reader, otherwise the reader's size plus the optional
/// sticky object's `+0x20` word.
///
/// # Safety
/// A non-zero `+0x14` must be a live reader; `+0x18`, when non-zero, must be
/// readable at `+0x20`.
///
/// # Confidence
/// high - both loads, the null guard and the single addition are literal.
pub unsafe fn fn_801608b0(this: usize) -> u32 {
    let reader = platform::read_u32(this + 0x14) as usize;
    if reader == 0 {
        return 0;
    }
    let mut size = archive::fn_802b9700(reader);
    if platform::read_u32(this + 0x18) != 0 {
        size = size.wrapping_add(platform::read_u32(reader + 0x20));
    }
    size
}

// ---------------------------------------------------------------------------
// slot 3 - 0x80160910
// ---------------------------------------------------------------------------

/// Returns the memory-region name selected by the handle's `+0x24` byte.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80160910`.
/// - Extent: `0x80160910..0x80160928` (24 bytes, 6 words); two zero words
///   follow, then [`fn_80160930`] at `0x80160930`.
/// - Table: slot 3 of the `0x80328AF0` array (word `0x80328AFC`, offset
///   `+0x00C`).
/// - observed: `lbz r0,0x24(r3)`; `lis r3,0x8033`; `addi r3,r3,-29992`
///   (`0x80328AD8`); `rlwinm r0,r0,2,28,29` (`(byte & 3) * 4`);
///   `lwzx r3,r3,r0`; `bclr`.
///
/// # Arguments
/// - `this`: live handle whose `+0x24` byte holds the region selector in bits
///   0..1.
///
/// # Returns
/// One of the four string pointers `"????"`, `"MEM"`, `"ARAM"`, `"DVD"`.
///
/// # Safety
/// `this + 0x24` must be readable and the table mapped.
///
/// # Confidence
/// high - the mask, scale and indexed load are literal.
pub unsafe fn fn_80160910(this: usize) -> u32 {
    let index = (platform::read_u8(this + 0x24) & 3) as usize;
    platform::read_u32(W286_REGION_NAMES_80328AD8 + index * 4)
}

// ---------------------------------------------------------------------------
// slot 4 - 0x801605A0
// ---------------------------------------------------------------------------

/// Empty dispatch hook.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801605A0`.
/// - Extent: `0x801605A0..0x801605A4` (4 bytes, 1 word); three zero words
///   follow, then [`fn_801605b0`] at `0x801605B0`.
/// - Table: slot 4 of the `0x80328AF0` array (word `0x80328B00`, offset
///   `+0x010`).
/// - observed: `4E800020 bclr 20,0` only.
///
/// # Safety
/// No memory is touched.
///
/// # Confidence
/// high - the body is one `blr`.
pub unsafe fn fn_801605a0() {}

// ---------------------------------------------------------------------------
// slot 5 - 0x80160300
// ---------------------------------------------------------------------------

/// Opens the profile's `.rel` file and adopts the returned reader.
///
/// The body polls up to three times: it formats `"%s/%sNP.rel"` from the
/// current REL name and `this + 0x20`, submits the path through
/// `0x8016BF70`, waits for the command, then compares the half-word checksum
/// reported by [`fn_801602d0`] with the value saved at `+0x26`. A mismatch
/// retries twice; a third mismatch fails.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80160300`.
/// - Extent: `0x80160300..0x801604AC` (428 bytes, 107 words); one zero word
///   follows, then [`fn_801604b0`] at `0x801604B0`.
/// - Table: slot 5 of the `0x80328AF0` array (word `0x80328B04`, offset
///   `+0x014`).
/// - observed: `_savegpr_27`; early `lwz r0,0x14` return `1`; `lwz r0,-21244(r13)`
///   into `+0x30`; per pass `lwzu r12,0x0C`/`lwz r12,0x18` release of the
///   pending owner; `bl 0x802E16E8` with `"%s/%sNP.rel"`; `bl 0x8016BF70`;
///   `stw r3,-21248(r13)`; `bl 0x8016B4F0`; `bl 0x80160240`;
///   `lwz r0,0x10(mount)` / `stw 0,0x10(mount)` / `stw r0,0x14(this)`;
///   `bl 0x8016B7E0`; `li 3` into `+0x24`; `bl 0x801602D0`;
///   `lhz r0,0x26(r31)` compare; `cmpwi r27,2` retry bound; `_restgpr_27`.
///
/// # Arguments
/// - `this`: live handle; `+0x14` pending reader, `+0x18` sticky object,
///   `+0x20` profile name, `+0x24` state byte, `+0x25` retry counter,
///   `+0x26` saved checksum, `+0x28` payload size, `+0x2C` mount command,
///   `+0x30` owner.
///
/// # Returns
/// `1` when the reader was adopted, `0` when the open or the checksum check
/// failed.
///
/// # Side effects
/// Writes the path buffer, the SDA pending-mount word, `+0x14`/`+0x18`/
/// `+0x24`/`+0x25`/`+0x26`/`+0x28`/`+0x30`, and runs up to two virtual
/// `+0x18` releases.
///
/// # Safety
/// `this` must be a live handle whose `+0x30` owner has a callable `+0x18`
/// slot; the formatter and the mount command functions must be initialised.
///
/// # Confidence
/// high - every path, store, compare and retry bound is literal. The
/// formatter's `r6`/`r7` varargs are passed through the reconstructed
/// signature of [`crate::engine::main_loop::fn_802e16e8`].
pub unsafe fn fn_80160300(this: usize) -> u32 {
    if platform::read_u32(this + 0x14) != 0 {
        return 1;
    }
    if platform::read_u32(this + 0x30) == 0 {
        platform::write_u32(this + 0x30, platform::read_u32(G_HANDLE_SELF));
    }
    let mut retries = 0i32;
    loop {
        if platform::read_u32(this + 0x14) != 0 {
            let owner = platform::read_u32(this + 0x30) as usize;
            let release =
                platform::read_u32(platform::read_u32(owner) as usize + 0x18) as usize;
            platform::call2(release, owner as u32, platform::read_u32(this + 0x14));
            platform::write_u32(this + 0x14, 0);
        }
        let mut path = crate::platform::GuestLocal::new( [0u8; 64]);
        crate::engine::main_loop::fn_802e16e8(
            path.as_mut_ptr() as usize,
            64,
            W286_FMT_REL_80328AC0 as u32,
            [
                platform::read_u32(W286_SDA_REL_NAME),
                platform::read_u32(this + 0x20),
                0,
                0,
                0,
            ],
            [0.0; 8],
            [0u32; 8],
        );
        if platform::read_u32(this + 0x14) == 0 {
            let mount = platform::call3(
                W286_FN_8016BF70,
                path.as_ptr() as u32,
                1,
                platform::read_u32(this + 0x30),
            );
            platform::write_u32(W286_SDA_PENDING_MOUNT, mount);
            if mount != 0 {
                platform::call1(W286_FN_8016B4F0, mount);
                fn_80160240(this);
                let reader = platform::read_u32(mount as usize + 0x10);
                platform::write_u32(mount as usize + 0x10, 0);
                platform::write_u32(this + 0x14, reader);
                platform::call1(W286_FN_8016B7E0, mount);
                platform::write_u32(W286_SDA_PENDING_MOUNT, 0);
                if platform::read_u32(this + 0x14) != 0 {
                    platform::write_u32(this + 0x28, 0);
                    platform::write_u8(this + 0x24, 3);
                }
            }
        }
        if platform::read_u32(this + 0x14) == 0 {
            return 0;
        }
        if (platform::read_u32(this + 0x28) as i32) > 0 {
            let checksum = fn_801602d0(
                platform::read_u32(this + 0x14) as usize,
                platform::read_u32(this + 0x28) as i32,
            );
            let seen = platform::read_u8(this + 0x25);
            if seen == 0 {
                platform::write_u16(this + 0x26, checksum as u16);
                platform::write_u8(this + 0x25, 1);
                continue;
            }
            if (checksum as u32 & 0xFFFF) != platform::read_u16(this + 0x26) as u32 {
                platform::write_u8(this + 0x25, 0);
                if retries < 2 {
                    retries += 1;
                    continue;
                }
                return 0;
            }
            if seen < 0xFF {
                platform::write_u8(this + 0x25, seen + 1);
            }
        } else {
            let seen = platform::read_u8(this + 0x25);
            if seen < 0xFF {
                platform::write_u8(this + 0x25, seen + 1);
            }
        }
        fn_80160240(this);
        return 1;
    }
}

// ---------------------------------------------------------------------------
// slot 6 - 0x801604B0
// ---------------------------------------------------------------------------

/// Creates or destroys the handle's mount command.
///
/// When `+0x2C` is empty the body either reports the already-loaded state
/// (`+0x14 != 0`) or allocates a command through `0x8016B8D0` with the
/// `+0x1C` virtual thunk `0x80160230` as its callback. A command whose `+0x08`
/// byte is set is destroying-deleted and cleared.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801604B0`.
/// - Extent: `0x801604B0..0x8016053C` (140 bytes, 35 words); one zero word
///   follows, then [`fn_80160540`] at `0x80160540`.
/// - Table: slot 6 of the `0x80328AF0` array (word `0x80328B08`, offset
///   `+0x018`).
/// - observed: `lwz r0,0x2C(r3)`; `lwz r0,0x14(r3)`; `lis r3,0x8016` /
///   `addi r3,r3,0x230`; `bl 0x8016B8D0`; `stw r3,0x2C(r31)`;
///   `lwz r3,0x2C(r31)`; `lbz r0,8(r3)`; `bl 0x8016B7E0`;
///   `stw 0,0x2C(r31)`; returns `0`/`1`.
///
/// # Arguments
/// - `this`: live handle; `+0x14` reader, `+0x2C` mount command.
///
/// # Returns
/// `1` when a reader is already present or a finished command was deleted,
/// `0` otherwise.
///
/// # Side effects
/// May allocate and queue a command, or destroying-delete one.
///
/// # Safety
/// `this + 0x14`/`+0x2C` must be readable; a non-null `+0x2C` must be a live
/// command whose `+0x08` byte is readable.
///
/// # Confidence
/// high - every guard, store and call is literal.
pub unsafe fn fn_801604b0(this: usize) -> u32 {
    if platform::read_u32(this + 0x2C) == 0 {
        if platform::read_u32(this + 0x14) != 0 {
            return 1;
        }
        let command = platform::call2(W286_FN_8016B8D0, W286_FN_80160230 as u32, this as u32);
        platform::write_u32(this + 0x2C, command);
    }
    let command = platform::read_u32(this + 0x2C) as usize;
    if command != 0 && platform::read_u8(command + 8) != 0 {
        platform::call1(W286_FN_8016B7E0, command as u32);
        platform::write_u32(this + 0x2C, 0);
        return 1;
    }
    0
}

// ---------------------------------------------------------------------------
// slot 7 - 0x80160540
// ---------------------------------------------------------------------------

/// Releases the handle's pending reader through the owner's `+0x18` slot.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80160540`.
/// - Extent: `0x80160540..0x80160594` (84 bytes, 21 words); two zero words
///   follow, then [`fn_801605a0`] at `0x801605A0`.
/// - Table: slot 7 of the `0x80328AF0` array (word `0x80328B0C`, offset
///   `+0x01C`). The same word is also slot `+0x1C` of three `d_basesNP`
///   tables (`0x806619F8`, `0x80661AC4`, `0x80661C70`).
/// - observed: `lwz r4,0x14(r3)`; null -> skip; `lwz r3,0x30(r3)`;
///   `lwz r12,0(r3)`; `lwz r12,0x18(r12)`; `bctrl`; `stw 0,0x14(r31)`;
///   `li r3,1`; `bclr`.
///
/// # Arguments
/// - `this`: live handle; `+0x14` pending reader, `+0x30` owner.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// One virtual release and a zero store at `+0x14`.
///
/// # Safety
/// A non-zero `+0x14` requires `+0x30` to hold a live owner with a callable
/// `+0x18` slot.
///
/// # Confidence
/// high - the guard, slot offset and store are literal.
pub unsafe fn fn_80160540(this: usize) -> u32 {
    let reader = platform::read_u32(this + 0x14);
    if reader != 0 {
        let owner = platform::read_u32(this + 0x30) as usize;
        let release = platform::read_u32(platform::read_u32(owner) as usize + 0x18) as usize;
        platform::call2(release, owner as u32, reader);
        platform::write_u32(this + 0x14, 0);
    }
    1
}

// ---------------------------------------------------------------------------
// slot 8 - 0x801605B0
// ---------------------------------------------------------------------------

/// Opens the profile's `.map` file, links it and installs the map parser.
///
/// The reader's payload is aligned to `0x20`: the body asks the owner's
/// `+0x14` slot for a block, or reuses/link-fixes the current block depending
/// on the size reported by `archive::fn_802b9700`. On success it adds the
/// `+0x10` slot count to [`G_HANDLE_REMAIN`], formats `"%s/%sNP.map"`,
/// creates the `+0x34` lock, and calls the reader's `+0x34` entry to install
/// the map parser into `+0x1C`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x801605B0`.
/// - Extent: `0x801605B0..0x80160800` (592 bytes, 148 words); it is flush
///   against the already-translated [`fn_80160800`] at `0x80160800`.
/// - Table: slot 8 of the `0x80328AF0` array (word `0x80328B10`, offset
///   `+0x020`).
/// - observed: `lwz r5,0x14(r3)`; `lwz r4,0x48(r5)`; `addi r0,r4,0x1F` /
///   `rlwinm r31,r0,0,0,26`; `bl 0x802B9700`; owner slots `+0x14`/`+0x20`
///   with `0x20`, `r31 + owner->0x20` and `r31` arguments; `bl 0x801B1F20` /
///   `0x801B1F30`; `lwz r12,0x0C(r29)` / `lwz r12,0x10(r12)` count;
///   `lwz r0,-21256(r13)` add/store; `"%s/%sNP.map"`; `bl 0x80160990`;
///   `lwz r12,0x34(owner)`; `stw r3,0x1C(r29)`; `bl 0x80160240`; the fail
///   block clears `+0x25`, `+0x18` and `+0x14`.
///
/// # Arguments
/// - `this`: live handle; `+0x14` reader, `+0x18` sticky block, `+0x1C` map
///   parser, `+0x20` profile name, `+0x24` state byte, `+0x25` retry counter,
///   `+0x30` owner, `+0x34` lock.
///
/// # Returns
/// `1` when the map parser is installed, `0` when a link or allocation step
/// fails.
///
/// # Side effects
/// Allocates or reuses a reader block, writes [`G_HANDLE_REMAIN`], formats a
/// 64-byte path, creates the `+0x34` lock, calls the reader's `+0x34` entry
/// and may release `+0x14`/`+0x18`.
///
/// # Safety
/// `this` must be a live handle; `+0x14` must be a live reader whose `+0x20`,
/// `+0x34` and `+0x48` words are readable; `+0x30` must hold a live owner
/// with callable `+0x14`/`+0x20` slots.
///
/// # Confidence
/// medium - every branch, offset and argument is literal; the two owner slots
/// are documented by their offsets only because the export is untabled, and
/// the sign test on the `+0x20` result is reproduced as `<= 0` from
/// `cmpwi`/`bc 4,1`.
pub unsafe fn fn_801605b0(this: usize) -> u32 {
    let mut reader = platform::read_u32(this + 0x14) as usize;
    if reader == 0 {
        return fn_801605b0_fail(this);
    }
    let size = (platform::read_u32(reader + 0x48).wrapping_add(0x1F)) & !0x1F;
    let end = reader + size as usize;
    let available = archive::fn_802b9700(reader);
    let owner = platform::read_u32(this + 0x30) as usize;

    if available == 0 {
        let block = platform::call3(
            platform::read_u32(platform::read_u32(owner) as usize + 0x14) as usize,
            owner as u32,
            platform::read_u32(reader + 0x20),
            0x20,
        );
        if block == 0 {
            return fn_801605b0_fail(this);
        }
        platform::write_u32(this + 0x18, block);
        if platform::call2(W286_FN_801B1F20, reader as u32, block) == 0 {
            return fn_801605b0_fail(this);
        }
    } else {
        let needed = size.wrapping_add(platform::read_u32(reader + 0x20));
        if needed >= available {
            let head = platform::call3(
                platform::read_u32(platform::read_u32(owner) as usize + 0x20) as usize,
                owner as u32,
                reader as u32,
                needed,
            );
            if (head as i32) <= 0 {
                let block = platform::call3(
                    platform::read_u32(platform::read_u32(owner) as usize + 0x14) as usize,
                    owner as u32,
                    platform::read_u32(reader + 0x20),
                    0x20,
                );
                if block == 0 {
                    return fn_801605b0_fail(this);
                }
                platform::write_u32(this + 0x18, block);
                if platform::call2(W286_FN_801B1F30, reader as u32, block) == 0 {
                    return fn_801605b0_fail(this);
                }
                platform::call3(
                    platform::read_u32(platform::read_u32(owner) as usize + 0x20) as usize,
                    owner as u32,
                    reader as u32,
                    size,
                );
            } else if platform::call2(W286_FN_801B1F30, reader as u32, end as u32) == 0 {
                return fn_801605b0_fail(this);
            }
        } else {
            if platform::call2(W286_FN_801B1F30, reader as u32, end as u32) == 0 {
                return fn_801605b0_fail(this);
            }
            platform::call3(
                platform::read_u32(platform::read_u32(owner) as usize + 0x20) as usize,
                owner as u32,
                reader as u32,
                needed,
            );
        }
    }

    let vtable = platform::read_u32(this + 0x0C) as usize;
    let count = platform::call1(platform::read_u32(vtable + 0x10) as usize, this as u32);
    platform::write_u32(
        G_HANDLE_REMAIN,
        platform::read_u32(G_HANDLE_REMAIN).wrapping_add(count),
    );

    let mut path = crate::platform::GuestLocal::new( [0u8; 64]);
    crate::engine::main_loop::fn_802e16e8(
        path.as_mut_ptr() as usize,
        64,
        W286_FMT_MAP_80328ACC as u32,
        [
            platform::read_u32(W286_SDA_MAP_NAME),
            platform::read_u32(this + 0x20),
            0,
            0,
            0,
        ],
        [0.0; 8],
        [0u32; 8],
    );
    fn_80160990(this + 0x34);
    let parser = platform::call2(
        platform::read_u32(reader + 0x34) as usize,
        reader as u32,
        path.as_ptr() as u32,
    );
    platform::write_u32(this + 0x1C, parser);
    fn_80160240(this);
    1
}

/// Tail thunk from the profile entry table to [`fn_801605b0`].
///
/// # Original (untabled)
/// - address `0x8008F8B0`, measured extent `0x8008F8B0..0x8008F8B4` (4 bytes)
/// - `0x8008F8B0: b 0x801605B0` - an unconditional branch, no prologue and no
///   local stack frame, so the whole body is one instruction
/// - referenced by the data word at `0x80310EC0`, slot 8 of the table whose
///   base is `0x80310E98` (array `0x80310EA0..0x80310EC8`, followed by
///   `"d_profile"` at `0x80310EC8`)
/// - delegating alias added by wave 289 lane F so the strict `fn_<addr>`
///   census sees the existing body under the thunk address; nothing is
///   re-translated
///
/// # Safety
/// Same contract as [`fn_801605b0`]: `this` must be a live handle.
///
/// # Confidence
/// high - the single instruction and its `b` target are observed, and the
/// target is the already-translated `fn_801605b0`.
pub unsafe fn fn_8008f8b0(this: usize) -> u32 {
    fn_801605b0(this)
}

/// Releases both pending objects of a handle after a failed `.map` open.
///
/// This is the inlined fail block of [`fn_801605b0`], not a separate original
/// function.
///
/// # Safety
/// `this` must be live; non-zero `+0x18`/`+0x14` require a callable `+0x30`
/// owner slot `+0x18`.
unsafe fn fn_801605b0_fail(this: usize) -> u32 {
    let owner = platform::read_u32(this + 0x30) as usize;
    platform::write_u8(this + 0x25, 0);
    let sticky = platform::read_u32(this + 0x18);
    if sticky != 0 {
        let release = platform::read_u32(platform::read_u32(owner) as usize + 0x18) as usize;
        platform::call2(release, owner as u32, sticky);
        platform::write_u32(this + 0x18, 0);
    }
    let reader = platform::read_u32(this + 0x14);
    if reader != 0 {
        let release = platform::read_u32(platform::read_u32(owner) as usize + 0x18) as usize;
        platform::call2(release, owner as u32, reader);
        platform::write_u32(this + 0x14, 0);
    }
    0
}

// ===========================================================================
// Wave 292 lane E (`_E292`) - the index-4 twin of the two-state flag setter
// ===========================================================================
//
// Re-picked by `work/wave292/e_pick.mjs` from `work/wave266/e_candidates.tsv`
// after the 259 candidates covered by waves 266..291 and the three deferred
// addresses were removed. The body is 120 bytes, starts with `9421xxxx`
// (`stwu r1,-N(r1)`), reproduces its extent with `bad = 0`, ends on exactly one
// `blr`, does not overlap an extent and is not a pointer-run interior.

/// `0x8016A0F0` - jumptable dispatcher used by [`fn_8016a1a0`] and
/// [`fn_8016a220`]. Its export hides the forwarded arguments, so the calls use
/// the address and the argument count observed in the body.
const W292_FN_8016A0F0: usize = 0x8016_A0F0;

/// Sets holder state `4` and switches the `+4` object to mode `3`.
///
/// Runs `0x8016A0F0(this, 4, 0)`, calls virtual `+0xC` of the `+4` object
/// with `(object, 3, 0, 0)`, then - when `flag == 0` - runs
/// `0x8016A0F0(this, 4, 1)`. It is the index-4 twin of [`fn_8016a1a0`], which
/// uses index `2` and mode `1`.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export for `0x8016A220`.
/// - Extent: `0x8016A220..0x8016A298` (120 bytes, 30 words), ending at the
///   `blr` at `0x8016A294`; the next row `0x8016A2A0` starts immediately.
/// - Reach: no `b` and no data word; `d_enemiesNP` calls it from
///   `0x80AF6D94`.
/// - Table: none.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `li r5,0`;
///   `stw r0,0x14(r1)`; `stw r31,0xC(r1)`; `mr r31,r4`; `li r4,4`;
///   `stw r30,8(r1)`; `mr r30,r3`; `bl 0x8016A0F0`;
///   `lwz r3,4(r30)`; `li r4,3`; `li r5,0`; `li r6,0`; `lwz r12,0(r3)`;
///   `lwz r12,0xC(r12)`; `mtspr ctr,r12`; `4E800421 bcctrl` (call);
///   `cmpi cr0,r31,0`; `40820014 bc 4,2` (`bf cr0.eq`, non-zero flag
///   skips); `mr r3,r30`; `li r4,4`; `li r5,1`; `bl 0x8016A0F0`;
///   epilogue; `4E800020 blr`.
/// - Polarity: `40820014` is `bc 4,2` (`bf cr0.eq`), so the second
///   `0x8016A0F0(this, 4, 1)` runs only when `flag == 0`; a non-zero flag
///   leaves the state at `0`.
/// - Decoder note: `4E800421` is the `bcctrl` call and falls through, so the
///   body still ends on one `blr`. The vtable offset `0xC` is the same slot
///   [`fn_8016a1a0`] uses.
///
/// # Arguments
/// * `this` - holder whose `+4` field is the switched object.
/// * `flag` - zero performs the `(4, 1)` half of the state change; non-zero
///   stops after the `(4, 0)` half.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// One or two `0x8016A0F0` dispatches and one virtual `+0xC` call.
///
/// # Safety
/// `this + 4` must hold an object whose vtable has a callable `+0xC` slot.
///
/// # Confidence
/// high - the two dispatcher triples, the virtual argument list and the flag
/// polarity are literal and mirror [`fn_8016a1a0`].
pub unsafe fn fn_8016a220(this: usize, flag: i32) {
    platform::call3(W292_FN_8016A0F0, this as u32, 4, 0);
    let object = platform::read_u32(this + 4) as usize;
    let vtable = platform::read_u32(object) as usize;
    let method = platform::read_u32(vtable + 0xC) as usize;
    platform::call4(method, object as u32, 3, 0, 0);
    if flag == 0 {
        platform::call3(W292_FN_8016A0F0, this as u32, 4, 1);
    }
}
