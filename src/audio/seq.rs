//! `RSEQ` header, name-table lookup, sequence-player state, `RSAR`
//! sound-archive access, the sound element block that plays them and the
//! sound-system object that consumes it - `0x802700F0..0x8027FF60`.
//!
//! # What this module covers
//!
//! Sixteen parts of the sound-sequence path:
//!
//! * `0x802700F0..0x80270310` - the four JAS sequence-object helpers the
//!   manager-list pair calls: the two cursor words' difference, the `+0x04`
//!   owner link store and clear, and the enable/state byte predicate.
//! * `0x80270D20..0x80270DA0` - `RSEQ` header validation and the archive
//!   name-table lookup that resolves a resource name to its payload word.
//! * `0x80270D90..0x80273DB0` - the JAS *sequence-control* object: its
//!   constructor and reset routines, the 16-entry track table, the helpers that
//!   read/write or broadcast to tracks, and the optional per-frame callback.
//! * `0x80275040..0x80275FD0` - the `RSAR` sound-archive front end: the
//!   allocator/reader header, the four-slot node array and its small track
//!   object, the info-header forwarders, and the id-to-entry decoders that
//!   hand a resolved path to one of two loader vtable slots.
//! * `0x80275750..0x80276BE0` - the vtable forwarders that adapt a wrapper's
//!   `+0x04` archive to the resolver signatures, and the byte/offset table
//!   resolvers they tail-call. The resolvers share the
//!   `0x80283A60(byte, offset, base)` idiom and the `+0x101` version split.
//! * `0x80275850..0x80276F50` - the resolver tail: the two-stream archive
//!   loader, the `(id, sub-id)` two-word resolver, the name-trie lookup that
//!   turns a name back into an id, and the eight-byte descriptor decoder.
//! * `0x80277090..0x8027AFA0` - the sound-system object the resolvers feed:
//!   its constructor and deleting destructors, the three registry containers
//!   and their mutexes, the archive-file loader, the table-size calculator and
//!   the bank-slot/registry helpers.
//! * `0x8027A160..0x8027A4F0` - the variable-size record pool: node attach and
//!   detach helpers, its constructor/destructor, and the lock-wrapped
//!   allocate / append / truncate trio.
//! * `0x8027A610..0x8027B6B0` - the bank-slot entry class (release, re-bucket,
//!   priority insert and admission test) and the sequence-manager singleton
//!   that owns the mutex every one of those routines takes.
//! * `0x8027BA20..0x8027FF60` - the sequence thread's support layer: the
//!   manager-list register/unregister pair, the fixed-slot bitmap allocator, the
//!   two table decoders the allocator feeds, the sub-object arm/release pair and
//!   the small per-frame helpers (wave 90).
//! * `0x8027B010..0x8027B2E0` - the two halves of the JAS sequence-object
//!   list hand-off, the two cached-handle probes, the bare vtable release, the
//!   start-up descriptor and the five-clock sum (wave 95).
//! * `0x8027C210..0x8027FDD0` - the sound element block added by wave 95: the
//!   `RSTM` section reader and its two subscript readers, the block constructor
//!   and destructor, the arm/reset pair, the per-element voice broadcast helpers,
//!   the pause broadcast, the voice detach and the small-element registry.
//! * `0x80270920..0x8027BAF0` - the playback front ends and the sequence thread
//!   added by wave 99: the four-record global track table and its controller
//!   refresh, the priority-bucket rebuild/insert pair, the three start routes
//!   (file/name, small element and bank id), the thread create/body/frame
//!   routines, the `RSTM` block loader with its row reader, the slot-allocator
//!   trigger, the large element player and its ramp helpers.
//! * `0x80271E30..0x8027EAF0` - the wave-104 pass: the control object's
//!   fast-forward/drain pair, the four node-mix polar helpers, the archive
//!   loader constructor, the sequence start-up/wait pair, the element-block
//!   teardown and the three per-frame element routines.
//! * `0x80273DD0..0x8027E080` - the wave-109 pass: the voice start-up that
//!   builds the nine-word request block and splices the result into the
//!   `+0xC8` list, the two remaining node-mix inputs (distance ramp and matrix
//!   fade), and the per-element voice burst.
//! * `0x80273560..0x80274480` - the wave-112 pass that closes the bucket: the
//!   per-track mixer refresh that rewrites nineteen fields of every voice in
//!   the track's `+0xC8` list, and the directional cone gain that ramps between
//!   the source axis and the node normal.
//!
//! | address | size | Rust | plain-English role |
//! |---|---|---|---|
//! | `0x80270D20` | 108 B | [`fn_80270d20`] | validate the `RSEQ` magic/version, split base and data offsets |
//! | `0x80270DA0` | 184 B | [`fn_80270da0`] | find a named entry in a table and return its payload |
//! | `0x80270D90` | 16 B | [`fn_80270d90`] | payload base plus the offset word at payload `+8` |
//! | `0x80270E60` | 76 B | [`fn_80270e60`] | set the 16 global fallback track values to `0xFFFF` |
//! | `0x80270EB0` | 328 B | [`fn_80270eb0`] | construct the sequence-control defaults |
//! | `0x80271190` | 288 B | [`fn_80271190`] | re-initialise play state from two caller words |
//! | `0x80271550` | 124 B | [`fn_80271550`] | start track slot 0 through the track manager |
//! | `0x80271640` | 336 B | [`fn_80271640`] | release both registrations and all 16 tracks |
//! | `0x80271790` | 144 B | [`fn_80271790`] | store a mode byte and forward it to every track |
//! | `0x80271820` | 192 B | [`fn_80271820`] | advance the play clock while tracks are live |
//! | `0x80271910` | 152 B | [`fn_80271910`] | invoke the optional per-frame callback |
//! | `0x802719B0` | 16 B | [`fn_802719b0`] | read one track `i16` value |
//! | `0x802719C0` | 16 B | [`fn_802719c0`] | write one track `i16` value |
//! | `0x802719D0` | 444 B | [`fn_802719d0`] | kill every live track whose id is in a range |
//! | `0x80271B90` | 32 B | [`fn_80271b90`] | track pointer by slot, or null above slot 15 |
//! | `0x80271CE0` | 60 B | [`fn_80271ce0`] | address of a track value (slots 16..31 use a global table) |
//! | `0x80272340` | 40 B | [`fn_80272340`] | forward a notify call through a player vtable slot |
//! | `0x802723A0` | 156 B | [`fn_802723a0`] | construct the outer sequence player around a control object |
//! | `0x802730D0` | 16 B | [`fn_802730d0`] | set a track's start and end offsets |
//! | `0x802730E0` | 28 B | [`fn_802730e0`] | arm a track |
//! | `0x80273100` | 208 B | [`fn_80273100`] | tear down a control object's track list |
//! | `0x80271000` | 392 B | [`fn_80271000`] | destroy the control object from `0x802DCD74` (wave 38 pass 2) |
//! | `0x802712B0` | 660 B | [`fn_802712b0`] | rebuild the control object from a bank and track mask |
//! | `0x802718E0` | 8 B | [`fn_802718e0`] | store a byte at control `+0xE9` |
//! | `0x802718F0` | 8 B | [`fn_802718f0`] | store a byte at control `+0xCF` |
//! | `0x80271900` | 12 B | [`fn_80271900`] | store the callback and context at `+0xF8`/`+0xFC` |
//! | `0x80271BB0` | 300 B | [`fn_80271bb0`] | advance every live track and report armed state |
//! | `0x802724E0` | 124 B | [`fn_802724e0`] | restart a player around a rebuilt control object |
//! | `0x80272560` | 104 B | [`fn_80272560`] | start slot-0 track and mark the player playing |
//! | `0x802725D0` | 244 B | [`fn_802725d0`] | store callback words and register the cycle callback |
//! | `0x802726D0` | 116 B | [`fn_802726d0`] | run the player's per-cycle callback body |
//! | `0x802728A0` | 8 B | [`fn_802728a0`] | forward a byte to control `+0xE9` |
//! | `0x802728B0` | 8 B | [`fn_802728b0`] | forward a byte to control `+0xCF` |
//! | `0x802728C0` | 8 B | [`fn_802728c0`] | forward callback/context to the control object |
//! | `0x802729C0` | 80 B | [`fn_802729c0`] | copy a control track value into a word, return 1 |
//! | `0x80272A10` | 44 B | [`fn_80272a10`] | write a control track value, return 1 |
//! | `0x80272A40` | 28 B | [`fn_80272a40`] | return `+0x270` when enabled, else 0 |
//! | `0x80272C40` | 244 B | [`fn_80272c40`] | link a holder to the track-chain object |
//! | `0x80272D40` | 56 B | [`fn_80272d40`] | detach a holder from its chain object |
//! | `0x80272D80` | 8 B | [`fn_80272d80`] | store a slot byte at track `+0x04` |
//! | `0x80272D90` | 160 B | [`fn_80272d90`] | construct a track object with default fields |
//! | `0x80272E30` | 260 B | [`fn_80272e30`] | tear down a track object's node list |
//! | `0x80272F40` | 396 B | [`fn_80272f40`] | write the track object's default field set |
//! | `0x802731D0` | 208 B | [`fn_802731d0`] | retire nodes in a track's list while armed |
//! | `0x802732A0` | 396 B | [`fn_802732a0`] | advance a track's counters and report its state |
//! | `0x80273430` | 144 B | [`fn_80273430`] | apply a value to every armed node in a track list |
//! | `0x802734C0` | 148 B | [`fn_802734c0`] | forward a mode byte to every armed node |
//! | `0x80273AE0` | 108 B | [`fn_80273ae0`] | release every node in a track's list |
//! | `0x80273B50` | 228 B | [`fn_80273b50`] | unlink one node from a track's list |
//! | `0x80273C40` | 356 B | [`fn_80273c40`] | apply a mode to a track and set its active byte |
//! | `0x80273DB0` | 32 B | [`fn_80273db0`] | address of one of the first 16 track `i16` values |
//! | `0x80275040` | 128 B | [`fn_80275040`] | write the `RSAR` playback default block (wave 45) |
//! | `0x802750C0` | 8 B | [`fn_802750c0`] | store a float at `+0x48` |
//! | `0x802750D0` | 8 B | [`fn_802750d0`] | store a float at `+0x4C` |
//! | `0x802750E0` | 8 B | [`fn_802750e0`] | store a float at `+0x50` |
//! | `0x802750F0` | 88 B | [`fn_802750f0`] | initialise the archive allocator header |
//! | `0x80275150` | 84 B | [`fn_80275150`] | info-record size as `(w0+w2+w5) * 0x28` |
//! | `0x802751B0` | 52 B | [`fn_802751b0`] | append one `0x28`-byte archive element |
//! | `0x802753F0` | 192 B | [`fn_802753f0`] | construct the four-slot `RSAR` node |
//! | `0x802754B0` | 172 B | [`fn_802754b0`] | destroy the four-slot `RSAR` node |
//! | `0x80275560` | 16 B | [`fn_80275560`] | read one node slot word |
//! | `0x80275570` | 24 B | [`fn_80275570`] | write one node slot through `0x80269D00` |
//! | `0x80275640` | 36 B | [`fn_80275640`] | construct the small `RSAR` track object |
//! | `0x80275670` | 64 B | [`fn_80275670`] | destroy the small `RSAR` track object |
//! | `0x802756B0` | 20 B | [`fn_802756b0`] | return whether the `+0x04` word is non-zero |
//! | `0x802756D0` | 8 B | [`fn_802756d0`] | store the `+0x04` state word |
//! | `0x802756E0` | 24 B | [`fn_802756e0`] | reset the track state byte and name |
//! | `0x80275700` | 8 B | [`fn_80275700`] | forward `+0x04` to `0x80276940` |
//! | `0x80275710` | 8 B | [`fn_80275710`] | forward `+0x04` to `0x80276980` |
//! | `0x80275720` | 8 B | [`fn_80275720`] | forward `+0x04` to `0x802769D0` |
//! | `0x80275730` | 16 B | [`fn_80275730`] | forward target `+0x34` and a word to `0x80276E00` |
//! | `0x80275740` | 16 B | [`fn_80275740`] | forward target `+0x38` and a word to `0x80276E00` |
//! | `0x802757D0` | 8 B | [`fn_802757d0`] | read the seven-word archive info header |
//! | `0x80275A80` | 280 B | [`fn_80275a80`] | load an archive entry by id (words 2/3) |
//! | `0x80275BA0` | 280 B | [`fn_80275ba0`] | load an archive entry by id (words 4/5) |
//! | `0x80275CC0` | 124 B | [`fn_80275cc0`] | copy a name and append a `/` when missing |
//! | `0x80275D40` | 36 B | [`fn_80275d40`] | clear the seven header pointer words |
//! | `0x80275D70` | 176 B | [`fn_80275d70`] | validate and copy the `RSAR` header |
//! | `0x80275E20` | 168 B | [`fn_80275e20`] | turn five header offsets into pointers |
//! | `0x80275ED0` | 12 B | [`fn_80275ed0`] | store `header + 8` as the archive base |
//! | `0x80275EE0` | 232 B | [`fn_80275ee0`] | entry type (`1`..`3`) by id |
//! | `0x80275FD0` | 296 B | [`fn_80275fd0`] | fill the eight-word entry info block |
//! | `0x80275750` | 16 B | [`fn_80275750`] | forward archive, `+0x3C` table and name to `0x80276E00` |
//! | `0x80275760` | 8 B | [`fn_80275760`] | forward archive and id to `0x80276AE0` |
//! | `0x80275770` | 8 B | [`fn_80275770`] | forward archive and id to `0x80275EE0` |
//! | `0x80275780` | 8 B | [`fn_80275780`] | forward archive, id and out to `0x80275FD0` |
//! | `0x80275790` | 8 B | [`fn_80275790`] | forward archive, id and out to `0x80276210` |
//! | `0x802757A0` | 8 B | [`fn_802757a0`] | forward archive, id and out to `0x80276300` |
//! | `0x802757B0` | 8 B | [`fn_802757b0`] | forward archive, id and out to `0x80276410` |
//! | `0x802757C0` | 8 B | [`fn_802757c0`] | forward archive, id and out to `0x802765A0` |
//! | `0x802757E0` | 8 B | [`fn_802757e0`] | forward archive, id and out to `0x80276100` |
//! | `0x802757F0` | 8 B | [`fn_802757f0`] | forward archive, id and out to `0x802764F0` |
//! | `0x80275800` | 8 B | [`fn_80275800`] | forward archive, id and out to `0x80276660` |
//! | `0x80275810` | 8 B | [`fn_80275810`] | forward archive, two ids and out to `0x80276770` |
//! | `0x80275820` | 8 B | [`fn_80275820`] | forward archive to `0x80276BA0` |
//! | `0x80275830` | 8 B | [`fn_80275830`] | forward archive, id and out to `0x80276BE0` |
//! | `0x80275840` | 8 B | [`fn_80275840`] | forward archive, two ids and out to `0x80276CE0` |
//! | `0x80276100` | 268 B | [`fn_80276100`] | resolve one id to a seven-byte record |
//! | `0x80276210` | 228 B | [`fn_80276210`] | resolve a type-1 id to four words |
//! | `0x80276300` | 260 B | [`fn_80276300`] | resolve a type-2 id and its bit run |
//! | `0x80276410` | 212 B | [`fn_80276410`] | resolve a type-3 id to three words |
//! | `0x802764F0` | 172 B | [`fn_802764f0`] | resolve one id to its `+0x04` word |
//! | `0x802765A0` | 184 B | [`fn_802765a0`] | resolve one id to a byte/word pair |
//! | `0x80276660` | 272 B | [`fn_80276660`] | resolve one id to a six-word record |
//! | `0x80276770` | 312 B | [`fn_80276770`] | resolve `(id, sub-id)` to five words |
//! | `0x802768B0` | 132 B | [`fn_802768b0`] | read the seven-word info header |
//! | `0x80276940` | 64 B | [`fn_80276940`] | first word of the `+0x10` table |
//! | `0x80276980` | 68 B | [`fn_80276980`] | one less than the `+0x20` table count |
//! | `0x802769D0` | 260 B | [`fn_802769d0`] | resolve an id through the `+0x30` pointer table |
//! | `0x80276AE0` | 184 B | [`fn_80276ae0`] | return an entry's `+0x20` word |
//! | `0x80276BA0` | 64 B | [`fn_80276ba0`] | first word of the `+0x18` table |
//! | `0x80276BE0` | 256 B | [`fn_80276be0`] | resolve an id to four words plus a sub-table |
//! | `0x80275850` | 552 B | [`fn_80275850`] | load an entry plus an optional second stream (wave 66) |
//! | `0x80276CE0` | 288 B | [`fn_80276ce0`] | resolve `(id, sub-id)` to a two-word record |
//! | `0x80276E00` | 328 B | [`fn_80276e00`] | name trie walk: name back to id |
//! | `0x80276F50` | 256 B | [`fn_80276f50`] | resolve an id to the eight-byte descriptor |
//! | `0x80277090` | 64 B | [`fn_80277090`] | free an archive-file object when flagged |
//! | `0x802770D0` | 1492 B | [`fn_802770d0`] | load an archive file plus its second stream |
//! | `0x802776B0` | 288 B | [`fn_802776b0`] | construct the sound-system object |
//! | `0x802777D0` | 64 B | [`fn_802777d0`] | deleting destructor of a sub-object |
//! | `0x80277810` | 64 B | [`fn_80277810`] | deleting destructor of a second sub-object |
//! | `0x80277870` | 88 B | [`fn_80277870`] | list destructor plus optional free |
//! | `0x802778F0` | 88 B | [`fn_802778f0`] | list destructor plus optional free |
//! | `0x80277970` | 88 B | [`fn_80277970`] | list destructor plus optional free |
//! | `0x802779D0` | 184 B | [`fn_802779d0`] | destroy the sound-system object |
//! | `0x80277A90` | 24 B | [`fn_80277a90`] | is an archive handle loaded? |
//! | `0x80277AB0` | 260 B | [`fn_80277ab0`] | build tables and arm the `+0xBC` head |
//! | `0x80277BC0` | 304 B | [`fn_80277bc0`] | clear tables and release bank entries |
//! | `0x80277CF0` | 336 B | [`fn_80277cf0`] | byte size of the archive tables |
//! | `0x80277E40` | 76 B | [`fn_80277e40`] | byte size of the `0xA000` block |
//! | `0x80277E90` | 816 B | [`fn_80277e90`] | carve the four tables out of a buffer |
//! | `0x802781C0` | 204 B | [`fn_802781c0`] | allocate one bank-scan descriptor |
//! | `0x80278290` | 300 B | [`fn_80278290`] | carve the bank slots and records |
//! | `0x80278780` | 16 B | [`fn_80278780`] | address of one bank slot |
//! | `0x80278790` | 400 B | [`fn_80278790`] | bank-group lookup, first column |
//! | `0x80278920` | 400 B | [`fn_80278920`] | bank-group lookup, second column |
//! | `0x80279D50` | 68 B | [`fn_80279d50`] | is a bank-group id present? |
//! | `0x8027A560` | 140 B | [`fn_8027a560`] | construct one 100-byte bank slot |
//! | `0x8027AD90` | 136 B | [`fn_8027ad90`] | register a bank in two lists |
//! | `0x8027AFA0` | 104 B | [`fn_8027afa0`] | link a descriptor into its slot |
//! | `0x8027A160` | 76 B | [`fn_8027a160`] | hang a holder off a node's `+0x0C` link (wave 80) |
//! | `0x8027A1B0` | 76 B | [`fn_8027a1b0`] | hang a holder off a node's `+0x08` link |
//! | `0x8027A200` | 80 B | [`fn_8027a200`] | detach a holder from its node |
//! | `0x8027A250` | 72 B | [`fn_8027a250`] | construct the variable-size record pool |
//! | `0x8027A2A0` | 112 B | [`fn_8027a2a0`] | destroy the record pool |
//! | `0x8027A310` | 8 B | [`fn_8027a310`] | destroy a pool's record list |
//! | `0x8027A320` | 8 B | [`fn_8027a320`] | destroy a pool's list without freeing |
//! | `0x8027A3A0` | 108 B | [`fn_8027a3a0`] | allocate one record under the pool lock |
//! | `0x8027A410` | 84 B | [`fn_8027a410`] | append one zeroed record, return its index |
//! | `0x8027A470` | 116 B | [`fn_8027a470`] | truncate the pool under two locks |
//! | `0x8027A4F0` | 104 B | [`fn_8027a4f0`] | re-register a triple in both registries |
//! | `0x8027A610` | 88 B | [`fn_8027a610`] | destroy one bank-slot entry |
//! | `0x8027A670` | 212 B | [`fn_8027a670`] | destroy an entry and its children |
//! | `0x8027A750` | 124 B | [`fn_8027a750`] | release each child, then re-bucket |
//! | `0x8027A7D0` | 120 B | [`fn_8027a7d0`] | apply one word to every child |
//! | `0x8027A850` | 136 B | [`fn_8027a850`] | apply two words to every child |
//! | `0x8027A8E0` | 16 B | [`fn_8027a8e0`] | read `f32` at `+0x44 + 4i` |
//! | `0x8027A8F0` | 16 B | [`fn_8027a8f0`] | write `f32` at `+0x58 + 4i` |
//! | `0x8027A900` | 248 B | [`fn_8027a900`] | re-insert a child node in priority order |
//! | `0x8027AE20` | 144 B | [`fn_8027ae20`] | clamp a slot cursor, drop entries above it |
//! | `0x8027AEB0` | 8 B | [`fn_8027aeb0`] | store a slot's entry target |
//! | `0x8027AEC0` | 220 B | [`fn_8027aec0`] | admission test against the head priority |
//! | `0x8027B510` | 8 B | [`fn_8027b510`] | sound-system ready byte |
//! | `0x8027B5F0` | 184 B | [`fn_8027b5f0`] | build and return the sequence manager |
//! | `0x8027B6B0` | 112 B | [`fn_8027b6b0`] | destroy the sequence manager |
//! | `0x8027BA20` | 104 B | [`fn_8027ba20`] | insert a node into the manager's `+0x384` list (wave 90) |
//! | `0x8027BA90` | 92 B | [`fn_8027ba90`] | unlink a node from the manager's `+0x384` list |
//! | `0x8027BBF0` | 128 B | [`fn_8027bbf0`] | initialise the fixed-slot allocator |
//! | `0x8027BC70` | 68 B | [`fn_8027bc70`] | clear the allocator's four header words |
//! | `0x8027BCC0` | 232 B | [`fn_8027bcc0`] | allocate the first free slot |
//! | `0x8027BDB0` | 120 B | [`fn_8027bdb0`] | free the slot owning an address |
//! | `0x8027BE30` | 16 B | [`fn_8027be30`] | zero a two-word node header |
//! | `0x8027BE40` | 268 B | [`fn_8027be40`] | decode one entry into a 16-word record |
//! | `0x8027BF50` | 704 B | [`fn_8027bf50`] | decode one entry of the variable table |
//! | `0x8027C3E0` | 72 B | [`fn_8027c3e0`] | type byte of the handle's entry |
//! | `0x8027C430` | 60 B | [`fn_8027c430`] | decode the entry when present |
//! | `0x8027C470` | 60 B | [`fn_8027c470`] | decode the variable entry when present |
//! | `0x8027C8B0` | 88 B | [`fn_8027c8b0`] | tear down, free when flagged |
//! | `0x8027C910` | 88 B | [`fn_8027c910`] | second copy of the same destructor |
//! | `0x8027CC90` | 144 B | [`fn_8027cc90`] | release the block's sub-object |
//! | `0x8027CD20` | 216 B | [`fn_8027cd20`] | arm the block's sub-object |
//! | `0x8027EEF0` | 152 B | [`fn_8027eef0`] | apply a mode word to an output block |
//! | `0x8027EF90` | 128 B | [`fn_8027ef90`] | write an `f32` into mask-selected slots |
//! | `0x8027F020` | 32 B | [`fn_8027f020`] | address of one `0x38`-byte slot |
//! | `0x8027F040` | 80 B | [`fn_8027f040`] | four-word lookup, arm byte on failure |
//! | `0x8027F650` | 124 B | [`fn_8027f650`] | rebuild the sub-object from three words |
//! | `0x8027F6D0` | 96 B | [`fn_8027f6d0`] | arm the sub-object, roll back on failure |
//! | `0x8027F730` | 128 B | [`fn_8027f730`] | advance the eight per-slot counters |
//! | `0x8027FD50` | 56 B | [`fn_8027fd50`] | detach a handle from its owner |
//! | `0x8027FD90` | 64 B | [`fn_8027fd90`] | free an object when flagged |
//! | `0x8027FE70` | 100 B | [`fn_8027fe70`] | register the element destructor |
//! | `0x8027FEE0` | 88 B | [`fn_8027fee0`] | per-element list destructor |
//! | `0x8027FF60` | 124 B | [`fn_8027ff60`] | link a node into an element table |
//! | `0x802700F0` | 16 B | [`fn_802700f0`] | remaining length of a JAS sequence object (wave 95) |
//! | `0x80270100` | 8 B | [`fn_80270100`] | store the JAS object's `+0x04` owner link |
//! | `0x80270110` | 12 B | [`fn_80270110`] | clear the JAS object's `+0x04` owner link |
//! | `0x80270310` | 28 B | [`fn_80270310`] | enable-gated state byte of a JAS object |
//! | `0x8027B010` | 168 B | [`fn_8027b010`] | unlink a sequence object and release it |
//! | `0x8027B0C0` | 160 B | [`fn_8027b0c0`] | link a sequence object back into the list |
//! | `0x8027B160` | 96 B | [`fn_8027b160`] | probe a source for a sequence, drop the handle |
//! | `0x8027B1C0` | 168 B | [`fn_8027b1c0`] | reuse a cached sequence, else create one |
//! | `0x8027B270` | 24 B | [`fn_8027b270`] | release a source through vtable `+0x0C` |
//! | `0x8027B290` | 72 B | [`fn_8027b290`] | start the subsystem with a four-word descriptor |
//! | `0x8027B2E0` | 128 B | [`fn_8027b2e0`] | sum the five subsystem clocks |
//! | `0x8027C210` | 452 B | [`fn_8027c210`] | read and validate an `RSTM` section |
//! | `0x8027C4B0` | 332 B | [`fn_8027c4b0`] | copy a type-2 section record by index |
//! | `0x8027C600` | 436 B | [`fn_8027c600`] | split one block of paired half-words |
//! | `0x8027C7C0` | 240 B | [`fn_8027c7c0`] | construct the sound element block |
//! | `0x8027C970` | 256 B | [`fn_8027c970`] | destroy the sound element block |
//! | `0x8027CA70` | 536 B | [`fn_8027ca70`] | arm the block for a count and bitmap |
//! | `0x8027D3A0` | 156 B | [`fn_8027d3a0`] | read the block's play record |
//! | `0x8027D440` | 212 B | [`fn_8027d440`] | report the block's play position |
//! | `0x8027D520` | 308 B | [`fn_8027d520`] | reset the block's element state |
//! | `0x8027DDD0` | 236 B | [`fn_8027ddd0`] | bind one voice per enabled element |
//! | `0x8027E590` | 264 B | [`fn_8027e590`] | push one time step into every voice |
//! | `0x8027EA00` | 240 B | [`fn_8027ea00`] | start every voice at one time step |
//! | `0x8027ED20` | 180 B | [`fn_8027ed20`] | broadcast the pause state |
//! | `0x8027F270` | 140 B | [`fn_8027f270`] | detach a voice entry and drop it |
//! | `0x8027FDD0` | 156 B | [`fn_8027fdd0`] | build and return the small-element registry |
//! | `0x80270920` | 232 B | [`fn_80270920`] | apply one state code to a track record (wave 99) |
//! | `0x80270AD0` | 152 B | [`fn_80270ad0`] | build and return the global track table |
//! | `0x80270C20` | 248 B | [`fn_80270c20`] | refresh the track records from the ports |
//! | `0x80274CE0` | 444 B | [`fn_80274ce0`] | compute the requested mix values for every node |
//! | `0x802783C0` | 956 B | [`fn_802783c0`] | rebuild three 0x80-bucket priority lists |
//! | `0x80279510` | 872 B | [`fn_80279510`] | start a sequence by file/name or bank id |
//! | `0x80279880` | 432 B | [`fn_80279880`] | start a sequence through the small element |
//! | `0x80279A30` | 312 B | [`fn_80279a30`] | start a sequence from a bank-group id |
//! | `0x80279B70` | 360 B | [`fn_80279b70`] | load or probe one sequence slot |
//! | `0x80279CE0` | 112 B | [`fn_80279ce0`] | resolve a name and start its slot |
//! | `0x8027AA00` | 388 B | [`fn_8027aa00`] | rebuild the priority buckets from a list |
//! | `0x8027ABB0` | 468 B | [`fn_8027abb0`] | insert one bank object into the buckets |
//! | `0x8027B720` | 300 B | [`fn_8027b720`] | start the sequence thread once |
//! | `0x8027B850` | 268 B | [`fn_8027b850`] | initialise the globals and drain releases |
//! | `0x8027B960` | 184 B | [`fn_8027b960`] | the sequence thread body |
//! | `0x8027BAF0` | 256 B | [`fn_8027baf0`] | run one sequence-thread frame |
//! | `0x8027D660` | 768 B | [`fn_8027d660`] | load an `RSTM` block and start it |
//! | `0x8027D960` | 624 B | [`fn_8027d960`] | stream element data into the voices |
//! | `0x8027DBD0` | 504 B | [`fn_8027dbd0`] | compute timing and claim allocator slots |
//! | `0x8027EDE0` | 268 B | [`fn_8027ede0`] | map a position to row, index and wraps |
//! | `0x8027F450` | 180 B | [`fn_8027f450`] | construct the large element player |
//! | `0x8027F7B0` | 224 B | [`fn_8027f7b0`] | advance and apply the eight ramp records |
//! | `0x8027F890` | 324 B | [`fn_8027f890`] | arm the ramp records selected by a mask |
//! | `0x8027FC50` | 244 B | [`fn_8027fc50`] | find and attach the element player |
//! | `0x8027FFE0` | 348 B | [`fn_8027ffe0`] | pop one object from three pending lists |
//! | `0x80271E30` | 612 B | [`fn_80271e30`] | fast-forward the control object by a step count |
//! | `0x802720A0` | 660 B | [`fn_802720a0`] | drain the control object until its clock runs out |
//! | `0x80274630` | 148 B | [`fn_80274630`] | distance-ramp level for one node |
//! | `0x802746D0` | 828 B | [`fn_802746d0`] | two-word distance fade for one node |
//! | `0x80274A10` | 384 B | [`fn_80274a10`] | piecewise distance fade for one node |
//! | `0x80274B90` | 284 B | [`fn_80274b90`] | polar conversion of a three-float direction |
//! | `0x80277050` | 60 B | [`fn_80277050`] | initialise an archive-loader block |
//! | `0x8027B360` | 428 B | [`fn_8027b360`] | first-time sequence-subsystem start-up |
//! | `0x8027B520` | 116 B | [`fn_8027b520`] | wait for the sound manager and one tick |
//! | `0x8027D160` | 456 B | [`fn_8027d160`] | tear down every voice, element and slot |
//! | `0x8027E6A0` | 508 B | [`fn_8027e6a0`] | advance one frame of the sound-element block |
//! | `0x8027E8A0` | 348 B | [`fn_8027e8a0`] | advance one column of the sound-element block |
//! | `0x8027EAF0` | 556 B | [`fn_8027eaf0`] | append one element record and advance its counters |
//! | `0x80273560` | 1400 B | [`fn_80273560`] | refresh every voice of a track from its envelopes |
//! | `0x80274480` | 432 B | [`fn_80274480`] | directional cone gain for one node |
//!
//! # Wave-289 vtable pass
//!
//! `VT_SEQ_PLAYER` (header `0x8034F000`, methods `0x8034F008..0x8034F034`) is
//! the outer sequence player built by [`fn_802723a0`]. Ten of its twelve
//! methods were untabled: the destructor [`fn_80272830`], release
//! [`fn_80272750`], base reset [`fn_802724a0`], priority re-insert
//! [`fn_802728d0`], and the six leaves [`fn_80272c20`], [`fn_80272c10`],
//! [`fn_80272a60`], [`fn_80272a80`], [`fn_80272bf0`], [`fn_80272c00`].
//!
//! `VT_ELEMENT_PLAYER` (header `0x8034F218`, methods `0x8034F220..0x8034F24C`)
//! is the large element player built by [`fn_8027f450`]. Ten of its twelve
//! methods were untabled: the destructor [`fn_8027fa80`], detach
//! [`fn_8027f9e0`], ramp reset [`fn_8027f510`], priority re-insert
//! [`fn_8027fae0`], and the six leaves [`fn_8027fc30`], [`fn_8027fc20`],
//! [`fn_8027fbd0`], [`fn_8027fbf0`], [`fn_8027fc00`], [`fn_8027fc10`].
//!
//! The nine pointer words at `0x8034F250..0x8034F270` are **not** vtable
//! slots. They are the nine-case jump table of the already-translated
//! texture-grid helper [`crate::render::gx_band::fn_802821e0`], reached by its
//! `bctr` at `0x80282434`. Treating them as slots would create nine spurious
//! bodies; the helper and its arms are documented in `gx_band.rs`.
//!
//! # Why this code exists in the image
//!
//! `0x80270D20` is the anchor that identifies this whole address lane: it tests
//! the 32-bit word `0x52534551`, which is the ASCII bytes `R S E Q`, the
//! NintendoWare sound **sequence** container. `0x80270DA0` then walks a table of
//! length-prefixed names (the archive's name table) and returns the `u32`
//! payload of the first match, which is how a caller turns a resource name into
//! an in-memory offset.
//!
//! The rest of the range is the object those files are played through. A
//! sequence-control object owns 16 track pointers plus 16 `i16` values, two
//! registrations that must be released (flags `+0xCC` and `+0xCD`), and an
//! optional callback (`+0xF8`/`+0xFC`). The code is entered from the JAS player
//! frame update; [`fn_802723a0`] is the player constructor that embeds a
//! control object at `+0x110`.
//!
//! `// inferred:` the two header helpers are consumed by
//! [`crate::game::player`]'s four-slot table, and the archive whose blocks they
//! parse is read by [`crate::resource::archive`]. The image has no symbol
//! table, so no Nintendo symbol name is claimed.
//!
//! The `0x80275xxx` block is the archive *front end* those files are fetched
//! through. [`fn_80275d70`] tests the `RSAR` magic word `0x52534152`, and
//! [`fn_80275e20`] converts the five header offsets into absolute pointers.
//! [`fn_80275a80`] and [`fn_80275ba0`] turn an id into a six-word info record,
//! join the record's name onto the archive prefix, and call one of two loader
//! vtable slots. The four-slot node at [`fn_802753f0`] and the small track
//! object at [`fn_80275640`] are the storage these lookups fill.
//!
//! # Start here
//!
//! For the file format read [`fn_80270d20`] then [`fn_80270da0`]. For the
//! player start at [`fn_80270eb0`] (constructor) followed by
//! [`fn_80271190`] (reset); [`fn_802719d0`] is the clearest example of the
//! mutex/track-loop idiom. For the archive front end start at
//! [`fn_80275d70`] (header check) and [`fn_80275a80`] (entry load). For the
//! sound-system object start at [`fn_802776b0`] (constructor) and
//! [`fn_802779d0`] (destructor), then read [`fn_80277ab0`] /
//! [`fn_80277e90`] to see how the archive tables are carved out. For the
//! record pool start at [`fn_8027a250`]; for the bank-slot entry class start at
//! [`fn_8027a900`] (priority insert) and [`fn_8027aec0`] (admission test); for
//! the sequence manager start at [`fn_8027b5f0`]. For the wave-90 support layer
//! start at [`fn_8027bbf0`] (slot allocator) and [`fn_8027cd20`] (sub-object
//! arm), then read [`fn_8027be40`] / [`fn_8027bf50`] for the two decoders. The
//! wave-95 sound element block starts at [`fn_8027c7c0`] (constructor) and
//! [`fn_8027ca70`] (arm), with [`fn_8027c210`] as the stream reader it feeds.
//! The wave-99 playback path starts at [`fn_80279510`] and [`fn_80279880`] for
//! the two sequence-start routes, [`fn_8027d660`] for the `RSTM` block load,
//! and [`fn_8027b960`] / [`fn_8027baf0`] for the sequence thread.
//!
//! # Sequence-control data model
//!
//! Offsets below are byte offsets into the object built by [`fn_80270eb0`].
//! They are `// observed:` from the writers in this file; the *meaning* of each
//! field is inferred from its users.
//!
//! ```text
//! +0x000  u32 vtable                      (0x8034EF98)
//! +0x0B4  u32 registration A              (guard byte +0xCC)
//! +0x0C0  u32 registration B              (guard byte +0xCD)
//! +0x0D0  f32 default A   +0x0D4 f32 A'   (same small-data word)
//! +0x0D8  f32 default B   +0x0E0 f32 B'   (same small-data word)
//! +0x0DC  u32 play counter (advanced by fn_80271820 mode 0)
//! +0x0E0  f32 play clock   (advanced by fn_80271820 mode 1)
//! +0x0E4  u32 caller word  (fn_80271190 arg 2)
//! +0x0E8  u8 0x7F  +0x0E9 u8 0x40  +0x0EA u8 0x30  +0x0EC u16 0x78
//! +0x0F0  u32 caller word  (fn_80271190 arg 3)
//! +0x0F4  u32 track-manager object (vtable slot +0x10 destroys a track)
//! +0x0F8  u32 optional callback   +0x0FC u32 callback context
//! +0x100  u32 track[16]           (null = free slot)
//! +0x140  i16 track_value[16]
//! +0x160  u32 auxiliary counter
//! ```
//!
//! Slots 16..31 of the same logical track table live in the global array at
//! `0x803FCC68`; [`fn_80270e60`] clears it and [`fn_80271ce0`] chooses between
//! the embedded and the global halves.
//!
//! # Sound-system object data model (wave 66)
//!
//! Offsets from the object built by [`fn_802776b0`]. They are `// observed:`
//! where a writer in this file proves them; the *meaning* of each field is
//! inferred from its users.
//!
//! ```text
//! +0x008  u32 vtable A (0x8034F0F8)   +0x00C u32 vtable B (0x8034F10C)
//! +0x010  u32 archive handle          +0x014 u32 id array (count + 8-byte slots)
//! +0x018  u32 second id array         +0x01C u32 bridge object
//! +0x020  u32 vtable C (0x8034F128)   +0x028 u32 vtable D (0x8034F138)
//! +0x030  u32 cursor block            (set to `this + 0xBC` by fn_80277AB0)
//! +0x03C  u32 bank count              +0x040 u32 bank-slot array (100 B each)
//! +0x044  registry 1 (list +0x04C, mutex +0x054)
//! +0x06C  registry 2 (list +0x074, mutex +0x07C)
//! +0x094  registry 3 (list +0x09C, mutex +0x0A4)
//! +0x0BC  allocator head (0xD0-byte entries)
//! +0x0C8  cursor block `(buffer, size, block, count)`
//! +0x0E4  u32 table block             +0x0E8 u32 table block size
//! +0x21C  u32 archive loader currently being filled
//! ```
//!
//! # Record-pool, bank-slot and manager layout (wave 80)
//!
//! The three classes this pass finishes share one shape: a mutex header, one or
//! more circular lists of `{count, head, tail}` triples, and a small block of
//! scalar fields.
//!
//! ```text
//! record pool (fn_8027a250)              bank slot (fn_8027a560, 100 bytes)
//! +0x00  u32 vtable (0x8034F148)         +0x04  circular list 1 (head/tail)
//! +0x04  mutex header                    +0x0C  list object (count/head/tail)
//! +0x1C  record list header              +0x18  list object (count/head/tail)
//!                                        +0x24  u32 cursor   +0x28 u32 limit
//!                                        +0x44  f32 array (read by fn_8027a8e0)
//!                                        +0x50  u32 priority word
//!                                        +0x58  f32 array (written by fn_8027a8f0)
//!                                        +0x98  u8  priority byte
//!
//! sequence manager (fn_8027b5f0, instance at 0x80412BF0)
//! +0x318 mutex body   +0x320 init block   +0x340 template
//! +0x354 mutex header (the one every critical section in this module takes)
//! +0x378 list (count/head/tail)           +0x384 second list
//! ```
//!
//! `// observed:` every offset above is written by the constructor named in the
//! same column, and the manager's `+0x354` is the mutex [`lock_seq_manager`]
//! takes. The *meaning* of the two priority words (`+0x50`/`+0x98`) is
//! inferred from [`fn_8027a900`] and [`fn_8027aec0`], which compare them and
//! clamp the sum into `0..=0x7F`.
//!
//! # Slot allocator and support objects (wave 90)
//!
//! The allocator built by [`fn_8027bbf0`] carves one buffer into fixed-size
//! slots and tracks them with a 32-bit bitmap:
//!
//! ```text
//! allocator (fn_8027bbf0)             handle used by the decoders
//! +0x00  u32 slot base                +0x04 u32 source object (0 = empty)
//! +0x04  u32 total bytes              +0x08 u32 table base
//! +0x08  u32 per-slot stride
//! +0x0C  u32 slot count               source object
//! +0x10  u32 used count               +0x08 u8  table selector
//! +0x14  u32 bitmap (32 slots)        +0x0C u32 table offset
//!                                     +0x10 u8  second table selector
//! contiguous sub-object (0x8027CC90)  +0x14 u32 second table offset
//! +0x100 u8  in-use flag (inferred)   +0x18 u8  third table selector
//! +0x101 u8  armed flag (inferred)    +0x1C u32 third table offset
//! +0x104..+0x106 u8 state bytes
//! +0x110 sub-object (0x0C-byte table at +0x154)
//! +0x14C u32 caller word   +0x150 u32 caller word
//! +0x164..+0x170 u32 mirrors of the three caller words
//! +0x180 u8[0x680] block cleared by 0x8026D9B0
//! +0x804 u32 word cleared by 0x8027CC90   +0x808 u32 caller word
//! ```
//!
//! `// observed:` every field above is written by the constructor or the arm
//! routine named in the same column. The *meaning* of the `+0x100`/`+0x101`
//! bytes and of the three selector/offset pairs is inferred from
//! [`fn_8027cc90`], [`fn_8027cd20`] and the two decoders.
//!
//!
//! # Dependencies
//!
//! * [`crate::platform`] - big-endian volatile loads/stores and indirect calls.
//! * `0x802DC69C` (`strlen`-shaped) and `0x802E1BD0` (`strncmp`-shaped) are
//!   called by address.
//! * `0x8027B5F0` returns the sequence-manager singleton. Its mutex lives at
//!   `manager + 0x354` and is taken/released with `0x801B2E60`/`0x801B2F40`.
//! * Base constructors `0x80265E80`, `0x80265F40`, `0x80265FF0`; registration
//!   helpers `0x8027BA90`, `0x80268C60` + `0x80229190`; track helpers
//!   `0x80273100`, `0x802734C0`, `0x80273560`, `0x802686A0`, `0x80268C40`,
//!   `0x80273DB0`, `0x802730D0`, `0x802730E0`.
//! * `RSAR` helpers: `0x80283A60` (table resolver), `0x802DCD6C` (packed
//!   archive singleton, high word = object, low word = id), `0x802DCDB8`
//!   (release), `0x802DC9A0`/`0x802DCA98` (element array construct/destroy),
//!   the `0x80276100..0x80276F50` info resolvers (every one is a byte/offset
//!   pair fed to `0x80283A60`) and `0x802E19F8`/`0x802E1A68` (bounded
//!   copy/append).
//! * Record-pool helpers (wave 80): `0x80269EE0`/`0x8026A1B0`/`0x80269F00`/
//!   `0x8026A010` (list header create and destroy), `0x8026A2A0` (insert a
//!   record of a given size), `0x8026A350` (append a zeroed record) and
//!   `0x8026A420` (truncate).
//! * Node and list primitives (wave 80): `0x80267700`/`0x80267720` (link
//!   tests), `0x80267740`/`0x80267750` (detach through a link),
//!   `0x80229030` (list teardown), `0x80229160`/`0x80229190` (insert/unlink)
//!   and `0x802DC780` (destructor registration).
//! * Sequence-thread helpers (wave 90): `0x800046B4` (`memset`-shaped fill),
//!   `0x801B1140`/`0x801B1180` (state lock), `0x801B5D80` (wake a waiter),
//!   `0x802DCA98` (register an element destructor), `0x8026D9B0` (fill a block)
//!   and the `0x8027` decoders this pass adds.
//! * All of the above are outside this module and are called by absolute
//!   address with the original argument registers.
//!
//! # Glossary (first use)
//!
//! * **DOL** - the Wii executable container `sys/main.dol`; addresses here are
//!   its *virtual* addresses (image base `0x80004000`), not file offsets.
//! * **NW4R / nw4hbm** - NintendoWare 4 Revolution, the SDK layer this sound
//!   code belongs to (see `docs/GLOSSARY.md`).
//! * **RSEQ** - the sound-sequence file format; magic bytes `R S E Q`.
//! * **RSAR** - the sound-archive container this module's `0x80275xxx` block
//!   reads; magic bytes `R S A R` (`0x52534152`).
//! * **JAS** - the NW4R sound library (J Audio System) that owns these objects.
//! * **Track** - one voice/channel stream inside a sequence; the control object
//!   holds up to 16 of them locally.
//! * **Name table** - the block described in [`fn_80270da0`]: a count, an array
//!   of offsets relative to the table base, then per-entry length/name/payload.
//! * **Relative offset** - a `u32` added to the table base (not a file offset)
//!   to reach an entry.
//!
//! # Status and provenance
//!
//! The two header/lookup functions were promoted from
//! `src/subsystems/r80260_802ed.rs` (ledger module `audio/seq`). The
//! sequence-player bodies were added by wave 7 and keep their `# Original`
//! evidence blocks. Wave 38 pass 2 adds the control-object lifecycle,
//! player wrappers and track object cluster (`0x80271000..0x80273DB0`,
//! 30 bodies), recorded in `ledger/agents/wave38-seq.csv`. Wave 45 adds the
//! `RSAR` sound-archive access block (`0x80275040..0x80275FD0`, 31 bodies),
//! recorded in `ledger/agents/wave45-seq.csv`. Wave 52 adds the vtable
//! forwarders (`0x80275750..0x80275840`, 15 bodies) and the table resolvers
//! (`0x80276100..0x80276BE0`, 15 bodies), recorded in
//! `ledger/agents/wave52-seq.csv`. Wave 66 adds the resolver tail
//! (`0x80276CE0..0x80276F50`, 3 bodies) and the sound-system/archive-consumer
//! layer (`0x80275850` plus `0x80277090..0x8027AFA0`, 24 bodies), recorded in
//! `ledger/agents/wave66-seq.csv`. Wave 80 adds the record pool
//! (`0x8027A160..0x8027A4F0`, 11 bodies), the bank-slot entry class and its
//! priority helpers (`0x8027A610..0x8027AEC0`, 11 bodies) and the sequence
//! manager with its ready byte (`0x8027B510..0x8027B6B0`, 3 bodies), recorded
//! in `ledger/agents/wave80-seq.csv`. Wave 90 adds the manager-list
//! register/unregister pair, the fixed-slot allocator, the two table decoders,
//! the sub-object arm/release pair and the per-frame helpers
//! (`0x8027BA20..0x8027FF60`, 28 bodies), recorded in
//! `ledger/agents/wave90-seq.csv`. Waves 95, 99 and 104 are the three cluster
//! entries above them, recorded in `ledger/agents/wave95-seq.csv`,
//! `ledger/agents/wave99-seq.csv` and `ledger/agents/wave104-seq.csv`. Wave
//! 109 adds the voice start-up and the three remaining mix/burst bodies
//! (`0x80273DD0..0x8027E080`, 4 bodies), recorded in
//! `ledger/agents/wave109-seq.csv`; the two paired-single entries it could not
//! reconstruct are named in `docs/COVERAGE_WAVE109_SEQ.md`. Wave 112 adds those
//! two entries (`0x80273560` and `0x80274480`, 2 bodies), recorded in
//! `ledger/agents/wave112-seq.csv`, which leaves the `0x80270000` bucket with
//! no untranslated function. Voice mixing and JAI bank code are still out of
//! scope.
//! Machine-readable rows are in
//! `ledger/agents/wave7-course.csv`, `ledger/agents/wave38-seq.csv`,
//! `ledger/agents/wave45-seq.csv`, `ledger/agents/wave52-seq.csv`,
//! `ledger/agents/wave66-seq.csv`, `ledger/agents/wave80-seq.csv`,
//! `ledger/agents/wave90-seq.csv`, `ledger/agents/wave109-seq.csv`,
//! `ledger/agents/wave112-seq.csv` and `ledger/functions.csv`.

use crate::platform;

// ===========================================================================
// Field offsets and constants used by the RSEQ checks.
// ===========================================================================

/// `RSEQ` magic, stored big-endian: bytes `52 53 45 51`.
const RSEQ_MAGIC: u32 = 0x5253_4551;

/// `RSEQ` header version floor checked by [`fn_80270d20`] (256).
const RSEQ_MIN_VERSION: u32 = 0x100;

/// `0x802DC69C` - `strlen`-shaped: length of a NUL-terminated string.
///
/// `// inferred:` from its use in [`fn_80270da0`], where the result is compared
/// with the length word stored in each name-table entry.
const STRLEN: usize = 0x802D_C69C;

/// `0x802E1BD0` - `strncmp`-shaped comparator: `0` when equal.
const STRNCMP: usize = 0x802E_1BD0;

// ===========================================================================
// RSEQ header validation and lookup, 0x80270D20..0x80270DA0
// ===========================================================================

/// Validates an `RSEQ` header and returns the pair
/// `(header, header + *(u32 *)(header + 0x10))`.
///
/// The magic word must be `0x52534551` (`"RSEQ"`, read big-endian). The version
/// word at `+0x06` must be at least `0x100`; the C encodes that test as a
/// branchless clamp, reproduced here as the equivalent comparison. On success
/// the two output words are the file base and the base plus the `u32` at
/// `+0x10` (the data-section offset).
///
/// # Original
/// - Ghidra: `FUN_80270d20` at `0x80270D20` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270d20_FUN_80270d20.c`
/// - `// observed:` the bytes at `0x80270D30` are `addis r0,r5,0xADAD` followed
///   by `cmplwi r0,0x4551`, the compiler's two-step test for
///   `word == 0x52534551`; Ghidra folds it into the literal `0x52534551`.
///
/// # Arguments
/// - `out`: address of two writable words (base, data base).
/// - `header`: address of the candidate file header.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// - clears the two output words first, so a rejected file leaves zeros,
/// - on success writes `header` and `header + *(u32 *)(header + 0x10)`.
///
/// # Safety
/// `header` must be readable for at least `0x14` bytes; `out` must be writable
/// for 8 bytes.
///
/// # Confidence
/// high - magic and version check are explicit; the meaning of `+0x10` as a
/// data offset is inferred from the add.
pub unsafe fn fn_80270d20(out: usize, header: u32) {
    platform::write_u32(out, 0);
    platform::write_u32(out + 4, 0);

    if platform::read_u32(header as usize) != RSEQ_MAGIC {
        return;
    }
    let version = platform::read_u16(header as usize + 6) as u32;
    if version < RSEQ_MIN_VERSION {
        return;
    }

    platform::write_u32(out, header);
    let data_offset = platform::read_u32(header as usize + 0x10);
    platform::write_u32(out + 4, data_offset.wrapping_add(header));
}

/// Finds an entry by name in a name table and returns its `u32` payload.
///
/// `archive_ref` points at a word holding the archive object; the table lives
/// at `archive + *(u32 *)(archive + 0x18)` and has this shape:
///
/// ```text
/// +0x08  u32 count
/// +0x0C  u32 offset[count]      // relative to the table base
/// entry = table + offset[i]
/// entry+0x0C  u32 name length
/// entry+0x10  char name[length]
/// entry+0x08  u32 payload (the result)
/// ```
///
/// The search compares the length first, then the bytes with `strncmp`, and
/// stops at the first match.
///
/// # Original
/// - Ghidra: `FUN_80270da0` at `0x80270DA0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270da0_FUN_80270da0.c`
/// - The C types the function `void`, but the machine sets `r3` to the 0/1
///   result and the caller at `0x8027958C` tests `r3` immediately, so it is
///   translated as returning `u32`.
///
/// # Arguments
/// - `archive_ref`: address of a word containing the archive object address.
/// - `name`: address of the NUL-terminated name to find.
/// - `out`: address of one writable word that receives the payload.
///
/// # Returns
/// `1` when found (and `*out` written), `0` otherwise.
///
/// # Side effects
/// - writes `*out` on success,
/// - calls `strlen` and `strncmp` on the supplied name.
///
/// # Safety
/// `archive_ref` must point to a readable word; that archive object must have a
/// valid table at `+0x18`; `name` must be a readable C string; `out` writable.
///
/// # Confidence
/// medium - the table walk and comparison are explicit; the header layout is
/// inferred from the offsets (no string/format documentation in the image).
pub unsafe fn fn_80270da0(archive_ref: u32, name: u32, out: u32) -> u32 {
    let archive = platform::read_u32(archive_ref as usize);
    let table = archive.wrapping_add(platform::read_u32(archive as usize + 0x18));
    if table == 0 {
        return 0;
    }

    let name_len = platform::call1(STRLEN, name) as i32;
    let count = platform::read_u32(table as usize + 0x08);

    let mut offset_word = table as usize + 0x0C;
    let mut i = 0u32;
    while i < count {
        let entry = table.wrapping_add(platform::read_u32(offset_word));
        if name_len == platform::read_i32(entry as usize + 0x0C)
            && platform::call3(STRNCMP, name, entry.wrapping_add(0x10), name_len as u32) == 0
        {
            platform::write_u32(out as usize, platform::read_u32(entry as usize + 0x08));
            return 1;
        }
        offset_word += 4;
        i += 1;
    }

    0
}

// ===========================================================================
// Sequence-player constants
// ===========================================================================

/// `0x8027B5F0` - returns the sequence-manager singleton; called with no
/// arguments. Every lock/unlock sequence re-invokes it, exactly as the C does.
const SEQ_MANAGER: usize = 0x8027_B5F0;

/// `0x801B2E60` - takes the sequence mutex at `manager + 0x354`.
const MUTEX_LOCK: usize = 0x801B_2E60;

/// `0x801B2F40` - releases the sequence mutex at `manager + 0x354`.
const MUTEX_UNLOCK: usize = 0x801B_2F40;

/// `0x80265E80` - base constructor called by [`fn_80270eb0`].
const FUN_80265E80: usize = 0x8026_5E80;

/// `0x80265F40` - base reset called by [`fn_80271190`].
const FUN_80265F40: usize = 0x8026_5F40;

/// `0x80265FF0` - base constructor called by [`fn_802723a0`].
const FUN_80265FF0: usize = 0x8026_5FF0;

/// `0x802660D0` - resets a member record's counters and range ladders.
const FUN_802660D0: usize = 0x8026_60D0;

/// `0x80267190` - tears down one note record's child, links and callbacks.
const FUN_80267190: usize = 0x8026_7190;

/// `0x8027BA90` - releases the registration whose guard byte is `+0xCD`.
const FUN_8027BA90: usize = 0x8027_BA90;

/// `0x80268C60` - returns the object that owns the `+0xCC` registration.
const FUN_80268C60: usize = 0x8026_8C60;

/// `0x80229190` - releases the registration whose guard byte is `+0xCC`.
/// Ghidra prints this call through a thunk; the real body is `0x80229190`.
const FUN_80229190: usize = 0x8022_9190;

/// `0x80273100` - tears down one control object's track list
/// (see [`fn_80273100`]).
const FUN_80273100: usize = 0x8027_3100;

/// `0x802734C0` - forwards the mode byte to one track.
const FUN_802734C0: usize = 0x8027_34C0;

/// `0x80273560` - first phase of the track-list teardown.
const FUN_80273560: usize = 0x8027_3560;

/// `0x802686A0` - per-node cleanup, called when node byte `+0x36` is set.
const FUN_802686A0: usize = 0x8026_86A0;

/// `0x80268C40` - per-node final release.
const FUN_80268C40: usize = 0x8026_8C40;

/// `0x802730D0` - writes a track's start and end offsets.
const FUN_802730D0: usize = 0x8027_30D0;

/// `0x802730E0` - arms a track.
const FUN_802730E0: usize = 0x8027_30E0;

/// `0x80273DB0` - looks up a sequence/table word used in the callback block.
const FUN_80273DB0: usize = 0x8027_3DB0;

/// `0x803FCC68` - 16 global `i16` track values, the second half of the
/// logical 32-slot table. Cleared by [`fn_80270e60`].
const SEQ_GLOBAL_TRACK_VALUES: usize = 0x803F_CC68;

/// `r2 - 0x4690` → `0x8042E9F0`: first of two small-data default words
/// copied into the control object by [`fn_80270eb0`]/[`fn_80271190`].
///
/// `// unknown:` the exact values are in the small-data section, which is not
/// part of the export; the stores are reproduced verbatim.
const SDA2_DEFAULT_A: usize = 0x8042_E9F0;

/// `r2 - 0x468C` → `0x8042E9F4`: second default word (4 bytes after
/// [`SDA2_DEFAULT_A`]).
const SDA2_DEFAULT_B: usize = 0x8042_E9F4;

/// `r2 - 0x4688` → `0x8042E9F8`: the `double` subtracted from the synthetic
/// `0x4330_0000_xxxxxxxx` value in [`fn_80271820`].
///
/// `// inferred:` the pattern is the PowerPC unsigned/signed-integer to double
/// conversion bias (`2^52` or `2^52 + 2^31`), the same idiom documented in
/// `resource/course.rs`. Read at run time rather than assumed.
const SDA2_CONVERSION_BIAS: usize = 0x8042_E9F8;

/// `0x8034EF98` - control-object vtable installed by [`fn_80270eb0`].
const VT_SEQ_CTRL: u32 = 0x8034_EF98;

/// `0x8034EFBC` - second table pointer stored at `+0xBC`.
const VT_SEQ_CTRL_B: u32 = 0x8034_EFBC;

/// `0x8034EFD0` - third table pointer stored at `+0xC8`.
const VT_SEQ_CTRL_C: u32 = 0x8034_EFD0;

/// `0x8034F000` - player vtable installed by [`fn_802723a0`].
const VT_SEQ_PLAYER: u32 = 0x8034_F000;

/// `r13 - 0x4950` = `0x8042AD30`: head of the global sequence-player list.
/// [`fn_80272c20`] returns this address; [`fn_80272c40`] and the two list
/// helpers walk it.
const SEQ_PLAYER_LIST: usize = crate::boot::entry::SDA1_BASE - 0x4950;

/// `0x8034F038` - player table pointer stored at `+0x490`.
const VT_SEQ_PLAYER_B: u32 = 0x8034_F038;

/// `0x8034F050` - track-object vtable installed by [`fn_80272d90`] and
/// [`fn_80272e30`].
const VT_SEQ_TRACK: u32 = 0x8034_F050;

/// `r2 - 0x4678` → `0x8042EA08`: first small-data default word read by
/// [`fn_80272f40`] into the track fields at `+0x08`, `+0x0C`, `+0x18`.
///
/// `// observed:` the image (`work/dol/main.elf`) holds `1.0` here; the same
/// word is the neutral `1.0` factor [`fn_80273560`] starts its gain from.
const SDA2_TRACK_DEFAULT_A: usize = 0x8042_EA08;

/// `r2 - 0x4674` → `0x8042EA0C`: second default word read by
/// [`fn_80272f40`] into the track fields at `+0x10`, `+0x14`, `+0x6C`,
/// `+0x9C`, `+0xA0`.
///
/// `// observed:` the image holds `0.0` here; [`fn_80273560`] uses it as the
/// neutral `0.0` addend.
const SDA2_TRACK_DEFAULT_B: usize = 0x8042_EA0C;

/// `r13 - 0x4950` → `0x8042AD30` (`r13 = 0x8042F680`, as documented in
/// `src/game/player.rs`): SDA sentinel a track-list walk in [`fn_80272c40`]
/// compares against.
const SDA_R13_TRACK_SENTINEL: usize = 0x8042_AD30;

/// `0x802B9280` - destroys an object when its counter word is positive;
/// called by [`fn_80271000`] and [`fn_80272e30`].
const FUN_802B9280: usize = 0x802B_9280;

/// `0x801B1140` - takes the small-data state lock used by [`fn_802712b0`].
const FUN_801B1140: usize = 0x801B_1140;

/// `0x801B1180` - releases the lock from `0x801B1140`.
const FUN_801B1180: usize = 0x801B_1180;

/// `0x80268D50` - re-registers the `+0xB4` registration in [`fn_802712b0`].
const FUN_80268D50: usize = 0x8026_8D50;

/// `0x80267380` - returns the inner word forwarded by [`fn_802724e0`].
const FUN_80267380: usize = 0x8026_7380;

/// `0x80266250` - stops a player from [`fn_802726d0`] when the callback
/// result is null.
const FUN_80266250: usize = 0x8026_6250;

/// `0x8027FDD0` - returns the callback registry used by [`fn_802725d0`].
const FUN_8027FDD0: usize = 0x8027_FDD0;

/// `0x8027FF60` - registers a callback with the registry from `0x8027FDD0`.
const FUN_8027FF60: usize = 0x8027_FF60;

/// `0x8026DB50` - initialises the `+0x58` sub-object of a track.
const FUN_8026DB50: usize = 0x8026_DB50;

/// `0x80268710` - retires a track node from [`fn_802731d0`].
const FUN_80268710: usize = 0x8026_8710;

/// `0x802688C0` - signals a track node from [`fn_802731d0`].
const FUN_802688C0: usize = 0x8026_88C0;

/// `0x80269990` - applies a byte value to a track node from [`fn_80273430`].
const FUN_80269990: usize = 0x8026_9990;

/// `0x80281460` - applies the mode byte to a track voice from [`fn_802734c0`].
const FUN_80281460: usize = 0x8028_1460;

/// `0x80268790` - finalises a track node from the mode-3 path of
/// [`fn_80273c40`].
const FUN_80268790: usize = 0x8026_8790;

// ---------------------------------------------------------------------------
// RSAR sound-archive helpers (wave 45). The `RSAR` block below loads the
// sequence archive resource by name/id; these addresses are the helpers its
// constructors, destructors and forwarders call.
// ---------------------------------------------------------------------------

/// `0x80238110` - base-class initialiser. Ghidra shows no arguments because
/// the caller leaves `r3` (the object) untouched, so the object is passed
/// implicitly in the register.
const FUN_80238110: usize = 0x8023_8110;

/// `0x8026D860` - table insert/copy helper (`table`, source, count, `0x28`).
///
/// `// unknown:` the exact container semantics; [`fn_802751b0`] only forwards
/// its last two arguments plus the fixed element stride.
const FUN_8026D860: usize = 0x8026_D860;

/// `0x802757D0` - `(archive, out[7]) -> found`; see [`fn_802757d0`].
const FUN_802757D0: usize = 0x8027_57D0;

/// `0x802DC9A0` - five-argument container constructor used by the RSAR node
/// constructor; the first argument is the element array at `+0x08`.
const FUN_802DC9A0: usize = 0x802D_C9A0;

/// `0x802DCA98` - matching container destructor (`begin`, element dtor,
/// stride, count).
const FUN_802DCA98: usize = 0x802D_CA98;

/// `0x80269D00` - per-element limit/state store used by the RSAR node
/// constructor and [`fn_80275570`].
const FUN_80269D00: usize = 0x8026_9D00;

/// `0x80269AF0` - per-element teardown used by [`fn_802754b0`].
const FUN_80269AF0: usize = 0x8026_9AF0;

/// `0x80269A50` - element destructor address passed to the container helpers.
const FUN_80269A50: usize = 0x8026_9A50;

/// `0x80269A30` - code label passed as a constructor callback to
/// `0x802DC9A0`.
const LAB_80269A30: usize = 0x8026_9A30;

/// `0x80276940` - archive sub-loader reached through [`fn_80275700`].
const FUN_80276940: usize = 0x8027_6940;

/// `0x80276980` - archive sub-loader reached through [`fn_80275710`].
const FUN_80276980: usize = 0x8027_6980;

/// `0x802769D0` - archive sub-loader reached through [`fn_80275720`].
const FUN_802769D0: usize = 0x8027_69D0;

/// `0x80276E00` - three-argument setter reached through [`fn_80275730`] and
/// [`fn_80275740`].
const FUN_80276E00: usize = 0x8027_6E00;

/// `0x802768B0` - reads the seven-word header of the archive's info node.
const FUN_802768B0: usize = 0x8027_68B0;

/// `0x80276660` - resolves an id to a six-word info record.
const FUN_80276660: usize = 0x8027_6660;

/// `0x80276770` - resolves `(id, sub-id)` to a five-word info record.
const FUN_80276770: usize = 0x8027_6770;

/// `0x80276BE0` - resolves an id to a four-word info record plus a sub-table.
const FUN_80276BE0: usize = 0x8027_6BE0;

/// `0x80276CE0` - resolves `(id, sub-id)` to a two-word info record.
const FUN_80276CE0: usize = 0x8027_6CE0;

/// `0x80275EE0` - returns the entry kind (`1`..`3`) of one archive id.
const FUN_80275EE0: usize = 0x8027_5EE0;

/// `0x80275FD0` - fills the eight-word info block of one archive id.
const FUN_80275FD0: usize = 0x8027_5FD0;

/// `0x80276100` - resolves one id to a seven-byte record.
const FUN_80276100: usize = 0x8027_6100;

/// `0x80276210` - resolves a type-1 id to a four-word record.
const FUN_80276210: usize = 0x8027_6210;

/// `0x80276300` - resolves a type-2 id to a three-word record.
const FUN_80276300: usize = 0x8027_6300;

/// `0x80276410` - resolves a type-3 id to a three-word record.
const FUN_80276410: usize = 0x8027_6410;

/// `0x802764F0` - resolves one id to its single `+0x04` word.
const FUN_802764F0: usize = 0x8027_64F0;

/// `0x802765A0` - resolves one id to a byte/word pair.
const FUN_802765A0: usize = 0x8027_65A0;

/// `0x80276AE0` - returns the `+0x20` word of one archive entry.
const FUN_80276AE0: usize = 0x8027_6AE0;

/// `0x80276BA0` - returns the first word of the `+0x18` table.
const FUN_80276BA0: usize = 0x8027_6BA0;

/// `0x80276F50` - resolves an id to the eight-byte descriptor used by the
/// type-1/2/3 record decoders.
const FUN_80276F50: usize = 0x8027_6F50;

/// `0x80283A60` - `RSAR` table resolver: `(table bytes, table size, base)`.
///
/// `// inferred:` from the repeated `size < *table` bounds test and the
/// relative-offset additions at every call site.
const FUN_80283A60: usize = 0x8028_3A60;

/// `0x802E19F8` - `strncpy`-shaped bounded copy.
///
/// `// inferred:` [`fn_80275cc0`] NUL-terminates the destination itself and
/// then copies exactly the `strlen` of the source.
const FUN_802E19F8: usize = 0x802E_19F8;

/// `0x802E1A68` - `strncat`-shaped bounded append.
///
/// `// inferred:` the load-by-name paths copy the archive prefix, then append
/// the resource name with `len + 1` including the terminator.
const FUN_802E1A68: usize = 0x802E_1A68;

/// `0x8034F080` / `0x8034F098` - the two vtable/data pointers installed at the
/// start of [`fn_802750f0`].
const VT_RSAR_ALLOC: u32 = 0x8034_F080;
const VT_RSAR_ALLOC_B: u32 = 0x8034_F098;

/// `0x803FCC88` - global free-list head stored at `+0x18` by
/// [`fn_802750f0`].
const RSAR_FREE_LIST: usize = 0x803F_CC88;

/// `0x8034F0B8` - vtable of the four-slot RSAR node built by
/// [`fn_802753f0`] and destroyed by [`fn_802754b0`].
const VT_RSAR_NODE: u32 = 0x8034_F0B8;

/// `0x8034F0D8` - vtable of the small RSAR track object built by
/// [`fn_80275640`].
const VT_RSAR_TRACK: u32 = 0x8034_F0D8;

/// `0x8042EA98` / `0x8042EA9C` / `0x8042EAA0` - three small-data defaults
/// read by [`fn_80275040`] (`r2 = 0x80433080`, so these are `r2 - 0x45E8`,
/// `r2 - 0x45E4` and `r2 - 0x45E0`).
const SDA2_RSAR_PLAY_A: usize = 0x8042_EA98;
const SDA2_RSAR_PLAY_B: usize = 0x8042_EA9C;
const SDA2_RSAR_PLAY_C: usize = 0x8042_EAA0;

/// `0x8042EAA8` / `0x8042EAAC` - small-data defaults installed at `+0x20` /
/// `+0x24` of the allocator object by [`fn_802750f0`] (`r2 - 0x45D8` /
/// `r2 - 0x45D4`).
const SDA2_RSAR_ALLOC_A: usize = 0x8042_EAA8;
const SDA2_RSAR_ALLOC_B: usize = 0x8042_EAAC;

/// `0x8042EAB0` / `0x8042EAB4` - small-data defaults written at `+0x48` /
/// `+0x50` of the RSAR node by [`fn_802753f0`] (`r2 - 0x45D0` /
/// `r2 - 0x45CC`).
const SDA2_RSAR_NODE_A: usize = 0x8042_EAB0;
const SDA2_RSAR_NODE_B: usize = 0x8042_EAB4;

/// `RSAR` sound-archive magic, stored big-endian: bytes `52 53 41 52`.
const RSAR_MAGIC: u32 = 0x5253_4152;

// ---------------------------------------------------------------------------
// Wave 66 helpers: the resolver tail and the sound-system object that consumes
// the archive front end.
//
// `0x802DCD64..0x802DCD74` are CodeWarrior register-save prologues and
// `0x802DCDB0..0x802DCDC0` are bare-return relays. Ghidra prints both as
// calls and recovers the saved registers as a 64-bit "return value", so a
// `FUN_802dcd6c()` at the top of an export means "the incoming `r3`/`r4`".
// The bodies below take those values from their own parameters.
// ---------------------------------------------------------------------------

/// `0x801B2E20` - initialises a JAS mutex header (`0x801B4B20` plus two zeroed
/// words at `+0x08`/`+0x0C`).
///
/// `// inferred:` from its position next to the `0x801B2E60`/`0x801B2F40`
/// lock/unlock pair and from the identical `+0x54`/`+0x7C`/`+0xA4` slots.
const MUTEX_INIT: usize = 0x801B_2E20;

/// `0x802E1AB4` - `strcmp`-shaped comparator: `0` when equal.
const STRCMP: usize = 0x802E_1AB4;

/// `0x80229030` - intrusive-list destructor: unlinks every node, then frees the
/// head object when the second argument is positive.
const FUN_80229030: usize = 0x8022_9030;

/// `0x80229160` - intrusive-list insert `(list, node_ref, node)`.
const FUN_80229160: usize = 0x8022_9160;

/// `0x80267330` - links a bank object to its sound-system owner.
///
/// `// unknown:` the fields it writes.
const FUN_80267330: usize = 0x8026_7330;

/// `0x8026D9B0` - unlinks every node whose payload lies in
/// `[ptr, ptr + size)` from an intrusive list.
const FUN_8026D9B0: usize = 0x8026_D9B0;

/// `0x8026F620` - the bounded copy [`fn_80277e90`] uses for the `0xD0`-byte
/// cursor block (`0x8026D860(obj + 8, src, count, 0xD0)`).
const FUN_8026F620: usize = 0x8026_F620;

/// `0x8026F630` - [`FUN_8026D9B0`] applied to the list at `obj + 8`; Ghidra
/// drops the forwarded `r4`/`r5` (start and size).
const FUN_8026F630: usize = 0x8026_F630;

/// `0x8026FF00` - installs the bank-scan vtable `0x8034EF88` and zeroes seven
/// words of the descriptor at the cursor.
const FUN_8026FF00: usize = 0x8026_FF00;

/// `0x80270000` - records `(aligned cursor, cursor + size, cursor)` at
/// `+0x0C`/`+0x10`/`+0x14`; returns `0` when the block would overflow.
const FUN_80270000: usize = 0x8027_0000;

/// `0x8027A560` - constructs one 100-byte bank slot from the small-data
/// defaults and sets its limit to `0x7FFFFFFF`.
const FUN_8027A560: usize = 0x8027_A560;

/// `0x8027A670` - releases every entry of a bank list, then the list and its
/// two sub-objects.
const FUN_8027A670: usize = 0x8027_A670;

/// `0x8027A750` - releases every bank entry hanging off a system-entry array.
const FUN_8027A750: usize = 0x8027_A750;

/// `0x8027AE20` - clamps a bank slot's cursor (`+0x24`) to its limit (`+0x28`).
const FUN_8027AE20: usize = 0x8027_AE20;

/// `0x8027AFA0` - links one bank slot into its parent's list.
const FUN_8027AFA0: usize = 0x8027_AFA0;

/// `0x8027AEB0` - stores the bank slot's entry count at `+0x28`.
const FUN_8027AEB0: usize = 0x8027_AEB0;

/// `0x8027B510` - byte flag at `r13 - 0x4940`; non-zero while the sound system
/// is initialised.
const FUN_8027B510: usize = 0x8027_B510;

/// `0x8027BBF0` - writes the `(buffer, size, block, count)` cursor block at
/// `obj + 0x00..0x14`.
const FUN_8027BBF0: usize = 0x8027_BBF0;

/// `0x8027BC70` - zeroes the four-word cursor block at `+0x00..0x0C` under the
/// caller's lock.
const FUN_8027BC70: usize = 0x8027_BC70;

/// `0x80275700` - forwarder to `0x80276940`; here it yields the archive's
/// `+0x00` table count.
const FN_80275700: usize = 0x8027_5700;

/// `0x80275710` - forwarder to `0x80276980`; here it yields the `+0x20` table
/// count minus one.
const FN_80275710: usize = 0x8027_5710;

/// `0x802756B0` - returns whether the archive handle's `+0x04` word is
/// non-zero.
const FUN_802756B0: usize = 0x8027_56B0;

/// `0x802757C0` - forwarder to `0x802765A0` (`(archive, id, out)` byte/word
/// pair).
const FN_802757C0: usize = 0x8027_57C0;

/// `0x80275800` - forwarder to `0x80276660` (six-word record).
const FN_80275800: usize = 0x8027_5800;

/// `0x80275810` - forwarder to `0x80276770` (five-word record).
const FN_80275810: usize = 0x8027_5810;

/// `0x80275820` - forwarder to `0x80276BA0`; here it yields the `+0x18` table
/// count.
const FN_80275820: usize = 0x8027_5820;

/// `0x80275830` - forwarder to `0x80276BE0` (four-word record plus sub-table).
const FN_80275830: usize = 0x8027_5830;

/// `0x80275840` - forwarder to `0x80276CE0` (two-word record).
const FN_80275840: usize = 0x8027_5840;

/// `0x80275A80` / `0x80275BA0` - load-by-id entry points. Both exports are
/// declared `void` because their body ends in the `0x802DCDB8` relay, but
/// every caller uses the pointer they leave in `r3`.
const FN_80275A80: usize = 0x8027_5A80;
const FN_80275BA0: usize = 0x8027_5BA0;

/// `0x8034EEF0` - vtable installed at `+0xBC` of the sound-system object.
const VT_SOUND_SYS_BANK: u32 = 0x8034_EEF0;

/// `0x8034EED0` - vtable installed at `+0xE0` of the sound-system object.
const VT_SOUND_SYS_BUF: u32 = 0x8034_EED0;

/// `0x8034F0F8` / `0x8034F10C` - the two interface vtables at `+0x08`/`+0x0C`.
const VT_SOUND_SYS_A: u32 = 0x8034_F0F8;
const VT_SOUND_SYS_B: u32 = 0x8034_F10C;

/// `0x8034F128` / `0x8034F138` - the two allocator/list vtables at
/// `+0x20`/`+0x28`.
const VT_SOUND_SYS_C: u32 = 0x8034_F128;
const VT_SOUND_SYS_D: u32 = 0x8034_F138;

/// `0x8042EAC8` / `0x8042EACC` - the `r2 - 0x45B8` / `r2 - 0x45B4` small-data
/// defaults written into every 100-byte bank slot by [`fn_8027a560`]
/// (`r2 = 0x80433080`).
const SDA2_BANK_DEFAULT_A: usize = 0x8042_EAC8;
const SDA2_BANK_DEFAULT_B: usize = 0x8042_EACC;

// ---------------------------------------------------------------------------
// Wave 80 callees: the record-pool helpers of 0x80269Exx..0x8026A4xx, the
// two registries fed by `0x8027A4F0`, the per-entry front ends of the
// sequence-control object, and the sequence-manager singleton.
// ---------------------------------------------------------------------------

/// `0x80267700` - `node + 0x08` is non-zero, computed as `(-u | u) >> 31`.
const FUN_80267700: usize = 0x8026_7700;

/// `0x80267720` - `node + 0x0C` is non-zero, same branchless test.
const FUN_80267720: usize = 0x8026_7720;

/// `0x80267740` - detaches the holder stored at `node + 0x08` by calling
/// [`fn_8027a200`] on it.
const FUN_80267740: usize = 0x8026_7740;

/// `0x80267750` - detaches the holder stored at `node + 0x0C`.
const FUN_80267750: usize = 0x8026_7750;

/// `0x80269EE0` - initialises a four-word list header: two zeroed words, then
/// two sentinel words pointing at the header's `+0x08`.
const FUN_80269EE0: usize = 0x8026_9EE0;

/// `0x80269F00` - destroys the list handed over in `r3` and frees it when the
/// relayed `r4` is positive. Ghidra prints the pair as the `0x802DCD74` relay,
/// so the export shows one argument where the call site passes two.
const FUN_80269F00: usize = 0x8026_9F00;

/// `0x8026A010` - destroying teardown of the list at `r3`, same relay shape;
/// its `r4`/`r5` are not visible in the export.
const FUN_8026A010: usize = 0x8026_A010;

/// `0x8026A1B0` - destroys the list at its argument without freeing it.
const FUN_8026A1B0: usize = 0x8026_A1B0;

/// `0x8026A2A0` - pool insert: allocates `r4` bytes, stores `r5`/`r6` in the
/// new record and links it; the node is returned in `r3`.
const FUN_8026A2A0: usize = 0x8026_A2A0;

/// `0x8026A350` - pool append: allocates one zeroed `0x14`-byte record and
/// returns its index (`count - 1`), or `-1`.
const FUN_8026A350: usize = 0x8026_A350;

/// `0x8026A420` - pool truncate: drops records until the pool holds `r4`.
const FUN_8026A420: usize = 0x8026_A420;

/// `0x80268D90` / `0x80268EC0` - the two global registries [`fn_8027a4f0`]
/// re-registers its triple in; both take `(registry, a, b, c)`.
const FUN_80268D90: usize = 0x8026_8D90;
const FUN_80268EC0: usize = 0x8026_8EC0;

/// `0x802667F0` / `0x80266440` - per-entry front ends of the
/// `0x80265xxx` sequence-control object. Ghidra keeps only the entry pointer
/// (the list node minus `0xF8`) plus the caller's words.
const FUN_802667F0: usize = 0x8026_67F0;
const FUN_80266440: usize = 0x8026_6440;

/// `0x8027AA00` - re-buckets every node of one bank slot's list into the
/// 128-bucket table at `0x803FCCB0`.
const FUN_8027AA00: usize = 0x8027_AA00;

/// `0x8034F148` - vtable of the record-pool object built by [`fn_8027a250`].
const VT_SEQ_POOL: u32 = 0x8034_F148;

/// `0x80412BF0` - the sequence-manager singleton returned by
/// [`fn_8027b5f0`]. Its `+0x354` word is the mutex [`lock_seq_manager`] takes,
/// which is why the singleton is the manager and not an unrelated object.
const SEQ_MANAGER_INSTANCE: usize = 0x8041_2BF0;

/// `r13 - 0x4938` - the manager's one-shot initialisation flag.
const SEQ_MANAGER_READY: usize = crate::boot::entry::SDA1_BASE - 0x4938;

/// `r13 - 0x4940` - the "sound system initialised" byte read by
/// [`fn_8027b510`].
const SOUND_SYSTEM_READY: usize = crate::boot::entry::SDA1_BASE - 0x4940;

/// `0x801B2390` - initialises the three-word structure at the manager's
/// `+0x320` from the template at `+0x340` (`4` = element count).
const FUN_801B2390: usize = 0x801B_2390;

/// `0x801B4B20` - initialises the manager's `+0x318` block (the mutex body that
/// `0x801B2E20` wraps).
const FUN_801B4B20: usize = 0x801B_4B20;

/// `0x802DC780` - registers a destructor (`0x8027B6B0`) and a context word
/// under the block it is given; the same call appears in `0x8027AA00`.
const FUN_802DC780: usize = 0x802D_C780;

/// `0x8027B6B0` - the deleting destructor registered for the manager block.
const FUN_8027B6B0: usize = 0x8027_B6B0;

/// `0x8027BA20` - registers one node in the manager's `+0x384` list with the
/// key word at manager `+0x388` (wave 90).
const FUN_8027BA20: usize = 0x8027_BA20;

/// `0x8027BE40` - decodes one eight-bitmap entry into a 16-word record.
const FUN_8027BE40: usize = 0x8027_BE40;

/// `0x8027BF50` - decodes one entry of a variable-length table.
const FUN_8027BF50: usize = 0x8027_BF50;

/// `0x8027CA70` - builds a sub-object with a caller word plus a shared value.
const FUN_8027CA70: usize = 0x8027_CA70;

/// `0x8027CC90` - releases the sub-object at `this + 0x110` (wave 90).
const FUN_8027CC90: usize = 0x8027_CC90;

/// `0x8027CD20` - arms the sub-object at `this + 0x110` (wave 90).
const FUN_8027CD20: usize = 0x8027_CD20;

/// `0x8027D660` - four-word lookup used by the per-frame helper.
const FUN_8027D660: usize = 0x8027_D660;

/// `0x8027F020` - address of one `0x38`-byte variable slot, `0` out of range.
const FUN_8027F020: usize = 0x8027_F020;

/// `0x8027FD90` - tear-down of an object, optionally freeing it.
const FUN_8027FD90: usize = 0x8027_FD90;

/// `0x8027FEE0` - per-element destructor registered by [`fn_8027fe70`].
const FUN_8027FEE0: usize = 0x8027_FEE0;

/// `0x802810A0` - one-shot reset applied by [`fn_8027eef0`] for modes 0/1.
const FUN_802810A0: usize = 0x8028_10A0;

/// `0x801B5D80` - wakes a thread waiting on the object at `this + 0x2C`.
const FUN_801B5D80: usize = 0x801B_5D80;

/// `0x80266D80` - per-frame hook called before the eight-slot scan in
/// [`fn_8027f730`].
const FUN_80266D80: usize = 0x8026_6D80;

/// `0x800046B4` - `memset`-shaped byte fill.
const FUN_800046B4: usize = 0x8000_46B4;

// ===========================================================================
// Local helpers
// ===========================================================================

/// Calls a no-argument routine and keeps the `r3` result.
///
/// `crate::platform::call0` deliberately drops `r3`; the sequence manager and
/// registration helpers return handles, so this local wrapper is used instead.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call0_r(addr: usize) -> u32 {
    let f: extern "C" fn() -> u32 = core::mem::transmute(addr);
    f()
}

/// Takes the sequence mutex at `manager + 0x354`.
///
/// # Safety
/// `0x8027B5F0` must return a live sequence manager.
#[inline]
unsafe fn lock_seq_manager() {
    let manager = call0_r(SEQ_MANAGER);
    platform::call1(MUTEX_LOCK, manager.wrapping_add(0x354));
}

/// Releases the sequence mutex at `manager + 0x354`.
///
/// # Safety
/// Every call must match an earlier [`lock_seq_manager`] on the same manager.
#[inline]
unsafe fn unlock_seq_manager() {
    let manager = call0_r(SEQ_MANAGER);
    platform::call1(MUTEX_UNLOCK, manager.wrapping_add(0x354));
}

/// Reads a big-endian IEEE-754 double.
///
/// # Safety
/// `addr..addr+8` must be readable and aligned like the original.
unsafe fn read_f64(addr: usize) -> f64 {
    let hi = platform::read_u32(addr) as u64;
    let lo = platform::read_u32(addr + 4) as u64;
    f64::from_bits((hi << 32) | lo)
}

/// Reproduces the compiler's `clamp(v, 0, 0x7F)` idiom.
///
/// The original computes `v = a + b` and then
/// `if ((int)v < 0x80) v &= ~(v >> 31); else v = 0x7F;`, i.e. a signed floor at
/// zero plus an unsigned ceiling of `0x7F`. The same sequence of operations is
/// kept here so the edge cases match the machine code.
#[inline]
fn clamp_priority(a: u32, b: u32) -> u32 {
    let value = a.wrapping_add(b) as i32;
    if value < 0x80 {
        (value & !(value >> 31)) as u32
    } else {
        0x7f
    }
}

/// Detaches a player from the intrusive list reached through `this + offset`.
///
/// The list header is `owner + 4` and its lock is `owner + 0x10`. The player's
/// `+0xF0` word is the node, and its vtable slot `+0x0C` is notified with `-1`
/// before `0x8026DB00` drops the owner link.
///
/// # Safety
/// `owner` must be a live list header and `this + 0xF0` a linked node.
#[inline]
unsafe fn detach_from_owner_list(this: usize, owner_offset: usize) {
    let owner = platform::read_u32(this + owner_offset) as usize;
    let lock = owner + 0x10;
    platform::call1(MUTEX_LOCK, lock as u32);
    if platform::read_u32(owner + 4) != 0 {
        platform::call2(FUN_80229190, (owner + 4) as u32, (this + 0xf0) as u32);
        let vtable = platform::read_u32(this) as usize;
        let notify = platform::read_u32(vtable + 0x0c) as usize;
        platform::call2(notify, this as u32, 0xFFFF_FFFF);
        platform::call2(FUN_8026DB00, owner as u32, this as u32);
    }
    platform::call1(MUTEX_UNLOCK, lock as u32);
}

/// Re-inserts a player into the priority-ordered list at `this + offset`.
///
/// The priority is the module's `byte + 0x98 + word + 0x50` clamp. The list is
/// emptied of the player first, then the first node whose priority is greater
/// than the player's is chosen as the insertion point.
///
/// # Safety
/// The list header and every node must be live.
#[inline]
unsafe fn reinsert_by_priority(this: usize, owner_offset: usize) {
    let priority = clamp_priority(
        platform::read_u8(this + 0x98) as u32,
        platform::read_u32(this + 0x50),
    );
    let owner = platform::read_u32(this + owner_offset) as usize;
    let lock = owner + 0x10;
    platform::call1(MUTEX_LOCK, lock as u32);
    platform::call2(FUN_80229190, (owner + 4) as u32, (this + 0xf0) as u32);

    let end = owner + 8;
    let mut node = platform::read_u32(end) as usize;
    while node != end {
        let node_priority = clamp_priority(
            platform::read_u8(node.wrapping_sub(0x58)) as u32,
            platform::read_u32(node.wrapping_sub(0xa0)),
        );
        if priority < node_priority {
            break;
        }
        node = platform::read_u32(node) as usize;
    }

    let mut cursor = node as u32;
    platform::call3(
        FUN_80229160,
        (owner + 4) as u32,
        (&mut cursor as *mut u32) as u32,
        (this + 0xf0) as u32,
    );
    platform::call1(MUTEX_UNLOCK, lock as u32);
}

/// Builds the archive-relative path the RSAR loaders expect.
///
/// The handle's `+0x08` word starts a NUL-terminated prefix. When `name` does
/// not already begin with `/`, the prefix and the name are copied into `buf`
/// with `0x802E19F8`/`0x802E1A68` (both bounded copies include the terminator).
/// A combined length above `0xFF` is rejected, exactly as the two load
/// wrappers do.
///
/// # Returns
/// `Some(pointer)` - either `name` itself (already absolute) or `buf` - and
/// `None` when the combined length does not fit.
///
/// # Safety
/// `name` must be a readable NUL-terminated string; `buf` must be writable for
/// 256 bytes.
unsafe fn archive_path(this: usize, name: u32, buf: &mut [u8; 256]) -> Option<u32> {
    if platform::read_u8(name as usize) == b'/' {
        return Some(name);
    }
    let prefix = this + 8;
    let name_len = platform::call1(STRLEN, name) as usize;
    let prefix_len = platform::call1(STRLEN, prefix as u32) as usize;
    if name_len + prefix_len > 0xff {
        return None;
    }
    let dst = buf.as_mut_ptr() as u32;
    platform::call3(FUN_802E19F8, dst, prefix as u32, (prefix_len + 1) as u32);
    platform::call3(FUN_802E1A68, dst, name, (name_len + 1) as u32);
    Some(dst)
}

// ===========================================================================
// 0x80270D90
// ===========================================================================

/// Returns the payload base of a loaded sequence file object.
///
/// The object's `+4` word points at the loaded block; the block's `+8` word is
/// an offset from that base, so the result is `block + *(u32 *)(block + 8)`.
/// Callers use it to reach the data section of the file validated by
/// [`fn_80270d20`].
///
/// # Original
/// - Ghidra: `FUN_80270d90` at `0x80270D90` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270d90_FUN_80270d90.c`
/// - observed: `return *(int *)(*(int *)(param_1 + 4) + 8) + *(int *)(param_1 + 4);`
///
/// # Arguments
/// - `sequence`: sequence file object (reads only `+4`).
///
/// # Returns
/// The payload base as a 32-bit address.
///
/// # Safety
/// `sequence + 4` must hold a readable block; the block must be readable for
/// at least `0x0C` bytes.
///
/// # Confidence
/// high - the two loads and the add are explicit; only the field naming is
/// inferred.
pub unsafe fn fn_80270d90(sequence: usize) -> u32 {
    let block = platform::read_u32(sequence + 4);
    block.wrapping_add(platform::read_u32(block as usize + 8))
}

// ===========================================================================
// 0x80270E60
// ===========================================================================

/// Clears the 16 global fallback track values to `0xFFFF`.
///
/// This is the global half of the 32-slot track-value table at
/// `0x803FCC68`; [`fn_80271ce0`] returns it for slots 16..31. `0xFFFF` is the
/// "no value / free" marker.
///
/// # Original
/// - Ghidra: `FUN_80270e60` at `0x80270E60` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270e60_FUN_80270e60.c`
/// - observed: 16 consecutive stores `DAT_803fcc68 + 0,2,..,0x1E = 0xffff`.
///
/// # Side effects
/// Writes 32 bytes of global BSS.
///
/// # Safety
/// The global table must be writable (it is normal `.bss`).
///
/// # Confidence
/// high - every store is visible in the C.
pub unsafe fn fn_80270e60() {
    let mut i = 0usize;
    while i < 16 {
        platform::write_u16(SEQ_GLOBAL_TRACK_VALUES + i * 2, 0xffff);
        i += 1;
    }
}

// ===========================================================================
// 0x80270EB0
// ===========================================================================

/// Constructs (or resets) a sequence-control object and returns it.
///
/// The routine calls the NW4R base constructor, installs three table pointers,
/// copies the two small-data default words into the float fields
/// (`+0xD0`, `+0xD4`, `+0xD8`, `+0xE0`), clears both registration guard bytes,
/// stores the fixed timing/mode bytes, and empties the track tables.
///
/// # Original
/// - Ghidra: `FUN_80270eb0` at `0x80270EB0` (328 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270eb0_FUN_80270eb0.c`
/// - observed: `FUN_80265e80();` then the store sequence reproduced below,
///   including `param_1[0x2d..0x32]`, bytes `+0xCC..+0xCF`,
///   `param_1[0x34..0x39]`, bytes `+0xE8..+0xEA`, `u16 +0xEC = 0x78`,
///   16 × `u16 0xFFFF` from `+0x140`, and 16 zero words from `+0x100`.
///
/// # Arguments
/// - `this`: address of the control object to initialise.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - calls `0x80265E80`,
/// - writes the whole field set listed in the module's data model.
///
/// # Safety
/// `this` must be writable for at least `0x164` bytes.
///
/// # Confidence
/// high - every store and both small-data loads are explicit; only the meaning
/// of the two default words is unknown.
pub unsafe fn fn_80270eb0(this: usize) -> usize {
    platform::call0(FUN_80265E80);

    let default_a = platform::read_u32(SDA2_DEFAULT_A);
    let default_b = platform::read_u32(SDA2_DEFAULT_B);

    platform::write_u32(this + 0xb4, 0);
    platform::write_u32(this + 0xb8, 0);
    platform::write_u32(this + 0xc0, 0);
    platform::write_u32(this + 0xc4, 0);

    platform::write_u32(this, VT_SEQ_CTRL);
    platform::write_u32(this + 0xbc, VT_SEQ_CTRL_B);
    platform::write_u32(this + 0xc8, VT_SEQ_CTRL_C);

    platform::write_u8(this + 0xcc, 0);
    platform::write_u8(this + 0xcd, 0);
    platform::write_u8(this + 0xce, 0);
    platform::write_u8(this + 0xcf, 0);

    platform::write_u32(this + 0xd4, default_a);
    platform::write_u32(this + 0xd8, default_b);
    platform::write_u32(this + 0xdc, 0);
    platform::write_u32(this + 0xe0, default_b);
    platform::write_u32(this + 0xd0, default_a);

    platform::write_u32(this + 0x160, 0);
    platform::write_u32(this + 0xe4, 0);
    platform::write_u32(this + 0xf8, 0);
    platform::write_u32(this + 0xfc, 0);

    platform::write_u16(this + 0xec, 0x78);
    platform::write_u8(this + 0xea, 0x30);
    platform::write_u8(this + 0xe8, 0x7f);
    platform::write_u8(this + 0xe9, 0x40);
    platform::write_u32(this + 0xf0, 0);

    let mut i = 0usize;
    while i < 16 {
        platform::write_u16(this + 0x140 + i * 2, 0xffff);
        i += 1;
    }

    let mut i = 0usize;
    while i < 16 {
        platform::write_u32(this + 0x100 + i * 4, 0);
        i += 1;
    }

    this
}

// ===========================================================================
// 0x80271190
// ===========================================================================

/// Re-initialises a control object's play state from two caller words.
///
/// This is the lighter reset path used when a new sequence is loaded into an
/// existing object: it keeps the vtable and the track tables, reloads the
/// default float words, stores the caller parameters at `+0xE4`/`+0xF0`,
/// clears both registration guard bytes and re-empties the track tables.
///
/// # Original
/// - Ghidra: `FUN_80271190` at `0x80271190` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271190_FUN_80271190.c`
/// - observed: `FUN_80265f40();` then `+0xCD = 0`, `+0xCE = 0`, the default-word
///   stores, `+0xE4 = param_2`, `+0xF0 = param_3`, the fixed bytes, 16 ×
///   `u16 0xFFFF` and 16 zero words.
///
/// # Arguments
/// - `this`: control object.
/// - `param_2`: caller word stored at `+0xE4`.
/// - `param_3`: caller word stored at `+0xF0`.
///
/// # Side effects
/// - calls `0x80265F40`,
/// - writes the play-state block described above.
///
/// # Safety
/// `this` must be writable for at least `0x164` bytes.
///
/// # Confidence
/// high - every store is explicit. Note that, unlike [`fn_80270eb0`], this
/// routine clears `+0xCD`/`+0xCE` but **not** `+0xCC`; that asymmetry is in the
/// C and is preserved.
pub unsafe fn fn_80271190(this: usize, param_2: u32, param_3: u32) {
    platform::call0(FUN_80265F40);

    let default_a = platform::read_u32(SDA2_DEFAULT_A);
    let default_b = platform::read_u32(SDA2_DEFAULT_B);

    platform::write_u8(this + 0xcd, 0);
    platform::write_u8(this + 0xce, 0);

    platform::write_u32(this + 0xd4, default_a);
    platform::write_u32(this + 0xdc, 0);
    platform::write_u32(this + 0xe0, default_b);
    platform::write_u32(this + 0xd0, default_a);
    platform::write_u32(this + 0x160, 0);
    platform::write_u32(this + 0xe4, param_2);

    platform::write_u16(this + 0xec, 0x78);
    platform::write_u8(this + 0xea, 0x30);
    platform::write_u8(this + 0xe8, 0x7f);
    platform::write_u8(this + 0xe9, 0x40);

    platform::write_u32(this + 0xf0, param_3);
    platform::write_u32(this + 0xd8, default_b);

    let mut i = 0usize;
    while i < 16 {
        platform::write_u16(this + 0x140 + i * 2, 0xffff);
        i += 1;
    }

    let mut i = 0usize;
    while i < 16 {
        platform::write_u32(this + 0x100 + i * 4, 0);
        i += 1;
    }
}

// ===========================================================================
// 0x80271550
// ===========================================================================

/// Starts track slot 0 through the track manager.
///
/// Under the sequence mutex, the routine loads the first track pointer
/// (`+0x100`) and, when a start word is non-zero, writes the track's range with
/// [`fn_802730d0`] and arms it with [`fn_802730e0`]. The parameter pair is
/// passed through unchanged.
///
/// # Original
/// - Ghidra: `FUN_80271550` at `0x80271550` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271550_FUN_80271550.c`
/// - observed: lock, `uVar2 = *(u32 *)(param_1 + 0x100)`, the two guarded
///   calls, unlock.
///
/// # Arguments
/// - `this`: control object.
/// - `param_2`: start word; `0` skips the work.
/// - `param_3`: length word passed to [`fn_802730d0`].
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - writes the track's `+0x1C`/`+0x20` and its arm byte.
///
/// # Safety
/// `this` must be a live control object and `+0x100` must be a valid track when
/// `param_2 != 0`.
///
/// # Confidence
/// high - control flow and offsets are explicit; "track 0" means literal slot
/// 0 because the offset is `+0x100` with no index.
pub unsafe fn fn_80271550(this: usize, param_2: u32, param_3: u32) {
    lock_seq_manager();

    let track = platform::read_u32(this + 0x100);
    if param_2 != 0 {
        platform::call3(FUN_802730D0, track, param_2, param_3);
        platform::call1(FUN_802730E0, track);
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80271640
// ===========================================================================

/// Releases both control-object registrations and every live track.
///
/// This is the body shared with the destructor path of `0x802712B0`:
///
/// 1. take the mutex twice (nested),
/// 2. if `+0xCD` is set, call `0x8027BA90(manager, this + 0xC0)` and clear it,
/// 3. if `+0xCC` is set, call `0x80229190(0x80268C60(), this + 0xB4)` and clear
///    it,
/// 4. for each of the 16 slots: take the mutex, if the track is non-null call
///    [`fn_80273100`] and the track manager's vtable slot `+0x10`, then null
///    the slot; release the mutex,
/// 5. release the two nested mutexes.
///
/// # Original
/// - Ghidra: `FUN_80271640` at `0x80271640` (336 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271640_FUN_80271640.c`
/// - observed: two `0x801B2E60` calls, the two guarded registration branches,
///   the 16-iteration loop with `FUN_80273100()` and the `+0xF4` vtable call,
///   then two `0x801B2F40` calls.
/// - `// inferred:` `FUN_80273100` is printed with no argument; `r3` still
///   holds the track pointer loaded for the vtable call, so the track is passed
///   explicitly here.
///
/// # Arguments
/// - `this`: control object.
///
/// # Side effects
/// - releases two registered resources,
/// - destroys and nulls every live track,
/// - takes/releases the sequence mutex (nested).
///
/// # Safety
/// `this` must be a live control object; `+0xF4` must point at an object whose
/// vtable slot `+0x10` accepts `(object, track)`.
///
/// # Confidence
/// high - the branch structure and mutex pairs are explicit; only the dropped
/// argument to `0x80273100` is inferred.
pub unsafe fn fn_80271640(this: usize) {
    lock_seq_manager();
    lock_seq_manager();

    if platform::read_u8(this + 0xcd) != 0 {
        // observed: the C keeps `param_1` itself when the pointer is null.
        let object = if this != 0 { this + 0xc0 } else { this };
        let manager = call0_r(SEQ_MANAGER);
        platform::call2(FUN_8027BA90, manager, object as u32);
        platform::write_u8(this + 0xcd, 0);
    }

    if platform::read_u8(this + 0xcc) != 0 {
        let object = if this != 0 { this + 0xb4 } else { this };
        let owner = call0_r(FUN_80268C60);
        platform::call2(FUN_80229190, owner, object as u32);
        platform::write_u8(this + 0xcc, 0);
    }

    let mut slot = 0usize;
    while slot < 16 {
        lock_seq_manager();

        let track = platform::read_u32(this + 0x100 + slot * 4);
        if track == 0 {
            unlock_seq_manager();
        } else {
            platform::call1(FUN_80273100, track);

            let manager = platform::read_u32(this + 0xf4) as usize;
            let vtable = platform::read_u32(manager) as usize;
            let destroy = platform::read_u32(vtable + 0x10) as usize;
            platform::call2(destroy, manager as u32, track);

            platform::write_u32(this + 0x100 + slot * 4, 0);
            unlock_seq_manager();
        }

        slot += 1;
    }

    unlock_seq_manager();
    unlock_seq_manager();
}

// ===========================================================================
// 0x80271790
// ===========================================================================

/// Stores a mode byte and forwards it to every live track.
///
/// # Original
/// - Ghidra: `FUN_80271790` at `0x80271790` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271790_FUN_80271790.c`
/// - observed: lock, `*(u8 *)(param_1 + 0xCE) = (char)param_2`, a 16-iteration
///   loop calling `FUN_802734C0(track, param_2)` for non-null tracks, unlock.
///
/// # Arguments
/// - `this`: control object.
/// - `mode`: byte stored at `+0xCE` and passed to every track.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - writes `+0xCE` and calls `0x802734C0` once per live track.
///
/// # Safety
/// `this` must be a live control object.
///
/// # Confidence
/// high - the loop bounds and the two effects are explicit.
pub unsafe fn fn_80271790(this: usize, mode: u32) {
    lock_seq_manager();

    platform::write_u8(this + 0xce, mode as u8);

    let mut slot = 0usize;
    while slot < 16 {
        let track = platform::read_u32(this + 0x100 + slot * 4);
        if track != 0 {
            platform::call2(FUN_802734C0, track, mode);
        }
        slot += 1;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80271820
// ===========================================================================

/// Advances the play clock while tracks are live.
///
/// If `+0xCC` is clear (no tracks registered) the routine only unlocks and
/// returns. Otherwise `mode == 0` adds `delta` to the integer counter at
/// `+0xDC`, and `mode == 1` adds the signed conversion of `delta` to the float
/// clock at `+0xE0`. Any other mode performs no arithmetic.
///
/// The float path is the PowerPC integer→double idiom: build
/// `double_bits(0x43300000, delta ^ 0x80000000)` and subtract the small-data
/// bias at `0x8042E9F8`.
///
/// # Original
/// - Ghidra: `FUN_80271820` at `0x80271820` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271820_FUN_80271820.c`
/// - observed: lock; `if (*(char *)(param_1 + 0xcc) == '\0')` unlock; else
///   `param_2 == 0` → `+0xDC += param_3`, `param_2 == 1` → `+0xE0 +=
///   (float)(CONCAT44(0x43300000, param_3 ^ 0x80000000) - *(double *)(r2 -
///   0x4688))`; unlock.
///
/// # Arguments
/// - `this`: control object.
/// - `mode`: `0` = integer counter, `1` = float clock.
/// - `delta`: amount to add.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - reads/writes `+0xDC` or `+0xE0`.
///
/// # Safety
/// `this` must be a live control object.
///
/// # Confidence
/// medium - the control flow and arithmetic are explicit; the physical unit of
/// the clock and the bias constant's value are inferred.
pub unsafe fn fn_80271820(this: usize, mode: u32, delta: u32) {
    lock_seq_manager();

    if platform::read_u8(this + 0xcc) == 0 {
        unlock_seq_manager();
        return;
    }

    if mode == 0 {
        let counter = platform::read_u32(this + 0xdc);
        platform::write_u32(this + 0xdc, counter.wrapping_add(delta));
    } else if mode == 1 {
        let biased = 0x4330_0000_0000_0000u64 | ((delta ^ 0x8000_0000) as u64);
        let converted = f64::from_bits(biased) - read_f64(SDA2_CONVERSION_BIAS);
        let clock = platform::read_f32(this + 0xe0);
        platform::write_f32(this + 0xe0, clock + converted as f32);
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80271910
// ===========================================================================

/// Invokes the optional per-frame callback with a four-field argument block.
///
/// When `+0xF8` is non-null the routine builds this block on its stack:
///
/// ```text
/// +0x00  u32  this + 0x140          (track-value array)
/// +0x04  u32  0x803FCC68            (global track-value table)
/// +0x08  u32  0x80273DB0(track, 0)
/// +0x0C  u8   track[0x24]           (saved copy)
/// ```
///
/// then calls `callback(arg, &block, this[+0xFC])` and restores
/// `track[0x24]` from the saved copy (the callback is allowed to change it for
/// the duration of the call only).
///
/// # Original
/// - Ghidra: `FUN_80271910` at `0x80271910` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271910_FUN_80271910.c`
/// - observed: the `+0xF8` guard, the four local stores (`local_28`,
///   `local_24`, `local_20`, `local_1c`), the indirect call with `param_2`,
///   `&local_28` and `*(u32 *)(param_1 + 0xFC)`, and the byte restore.
///
/// # Arguments
/// - `this`: control object.
/// - `arg`: first callback argument (register `r4` in the original).
/// - `track`: track object whose byte `+0x24` is saved/restored.
///
/// # Side effects
/// - calls the callback when installed,
/// - writes `track + 0x24` twice (save value then restore).
///
/// # Safety
/// `+0xF8` must be null or a function taking `(u32, *mut Block, u32)`; the
/// block is only valid for the duration of the call.
///
/// # Confidence
/// medium - the block layout is explicit; the callback's contract is inferred
/// from the save/restore pattern.
pub unsafe fn fn_80271910(this: usize, arg: u32, track: usize) {
    if platform::read_u32(this + 0xf8) == 0 {
        return;
    }

    let mut block = [0u8; 16];
    let block_addr = block.as_mut_ptr() as usize;
    platform::write_u32(block_addr, this.wrapping_add(0x140) as u32);
    platform::write_u32(block_addr + 4, SEQ_GLOBAL_TRACK_VALUES as u32);

    let field = platform::call2(FUN_80273DB0, track as u32, 0);
    platform::write_u32(block_addr + 8, field);

    let saved = platform::read_u8(track + 0x24);
    platform::write_u8(block_addr + 0x0c, saved);

    let callback = platform::read_u32(this + 0xf8) as usize;
    let context = platform::read_u32(this + 0xfc);
    platform::call3(callback, arg, block_addr as u32, context);

    platform::write_u8(track + 0x24, saved);
}

// ===========================================================================
// 0x802719B0 / 0x802719C0
// ===========================================================================

/// Reads one `i16` value from the embedded track-value table.
///
/// # Original
/// - Ghidra: `FUN_802719b0` at `0x802719B0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802719b0_FUN_802719b0.c`
/// - observed: `return (int)*(short *)(param_1 + param_2 * 2 + 0x140);`
///
/// # Arguments
/// - `this`: control object.
/// - `index`: slot number, sign-extended index arithmetic as in the C.
///
/// # Returns
/// The sign-extended `i16`, or whatever lies at that offset when `index` is out
/// of range (the original does not clamp).
///
/// # Safety
/// The address must be readable.
///
/// # Confidence
/// high - one load, one sign extension.
pub unsafe fn fn_802719b0(this: usize, index: i32) -> i32 {
    let addr = (this as isize)
        .wrapping_add((index as isize).wrapping_mul(2))
        .wrapping_add(0x140) as usize;
    platform::read_u16(addr) as i16 as i32
}

/// Writes one `i16` value into the embedded track-value table.
///
/// # Original
/// - Ghidra: `FUN_802719c0` at `0x802719C0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802719c0_FUN_802719c0.c`
/// - observed: `*(u16 *)(param_1 + param_2 * 2 + 0x140) = param_3;`
///
/// # Arguments
/// - `this`: control object.
/// - `index`: slot number (not clamped).
/// - `value`: value to store.
///
/// # Side effects
/// Writes two bytes.
///
/// # Safety
/// The address must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_802719c0(this: usize, index: i32, value: u16) {
    let addr = (this as isize)
        .wrapping_add((index as isize).wrapping_mul(2))
        .wrapping_add(0x140) as usize;
    platform::write_u16(addr, value);
}

// ===========================================================================
// 0x802719D0
// ===========================================================================

/// Kills every live track whose id falls inside `[low, high]`.
///
/// This is the "stop tracks in range" operation. It scans the 16 slots and, on
/// the first match (`track[+0x1C]` is the id), performs the same teardown as
/// [`fn_80271640`]: release both registrations, then destroy and null all 16
/// tracks. The scan stops after that single match.
///
/// # Original
/// - Ghidra: `FUN_802719d0` at `0x802719D0` (444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802719d0_FUN_802719d0.c`
/// - observed: lock; guarded by `+0xCC`; a 16-iteration scan comparing
///   `param_2 <= *(u32 *)(track + 0x1C) <= param_3`; on a match two more locks,
///   the `+0xCD`/`+0xCC` release branches, the 16-track destroy loop, two
///   unlocks, then `break`; final unlock.
/// - `// inferred:` `FUN_80273100` is printed with no argument; the live track
///   in `r3` is passed here, as in [`fn_80271640`].
///
/// # Arguments
/// - `this`: control object.
/// - `low`, `high`: inclusive id range (unsigned comparison).
///
/// # Side effects
/// - takes/releases the sequence mutex (nested),
/// - may release both registrations and destroy every track.
///
/// # Safety
/// `this` must be a live control object; every non-null track must have a
/// readable `+0x1C`, and `+0xF4` must point at a valid track manager.
///
/// # Confidence
/// high - the scan, comparison and teardown are explicit; the id field's
/// meaning is inferred from this comparison.
pub unsafe fn fn_802719d0(this: usize, low: u32, high: u32) {
    lock_seq_manager();

    if platform::read_u8(this + 0xcc) != 0 {
        let mut offset = 0isize;
        let mut remaining = 0x10i32;

        loop {
            let track = platform::read_u32((this as isize).wrapping_add(offset) as usize + 0x100);

            if track != 0 {
                let id = platform::read_u32(track as usize + 0x1c);
                if low <= id && id <= high {
                    lock_seq_manager();
                    lock_seq_manager();

                    if platform::read_u8(this + 0xcd) != 0 {
                        let object = if this != 0 { this + 0xc0 } else { this };
                        let manager = call0_r(SEQ_MANAGER);
                        platform::call2(FUN_8027BA90, manager, object as u32);
                        platform::write_u8(this + 0xcd, 0);
                    }

                    if platform::read_u8(this + 0xcc) != 0 {
                        let object = if this != 0 { this + 0xb4 } else { this };
                        let owner = call0_r(FUN_80268C60);
                        platform::call2(FUN_80229190, owner, object as u32);
                        platform::write_u8(this + 0xcc, 0);
                    }

                    let mut slot = 0i32;
                    let mut base = this as isize;
                    loop {
                        lock_seq_manager();

                        let current = platform::read_u32(base as usize + 0x100);
                        if current == 0 {
                            unlock_seq_manager();
                        } else {
                            platform::call1(FUN_80273100, current);

                            let manager = platform::read_u32(this + 0xf4) as usize;
                            let vtable = platform::read_u32(manager) as usize;
                            let destroy = platform::read_u32(vtable + 0x10) as usize;
                            platform::call2(destroy, manager as u32, current);

                            platform::write_u32(base as usize + 0x100, 0);
                            unlock_seq_manager();
                        }

                        slot += 1;
                        base += 4;
                        if slot >= 0x10 {
                            break;
                        }
                    }

                    unlock_seq_manager();
                    unlock_seq_manager();
                    break;
                }
            }

            offset += 4;
            remaining -= 1;
            if remaining == 0 {
                break;
            }
        }
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80271B90
// ===========================================================================

/// Returns the track pointer in one of the 16 embedded slots.
///
/// # Original
/// - Ghidra: `FUN_80271b90` at `0x80271B90` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271b90_FUN_80271b90.c`
/// - observed: `if (0xf < param_2) return 0; return *(u32 *)(param_1 +
///   param_2 * 4 + 0x100);`
///
/// # Arguments
/// - `this`: control object.
/// - `index`: slot `0..=15`.
///
/// # Returns
/// The track pointer, or `0` for `index > 15`.
///
/// # Safety
/// `this` must be readable.
///
/// # Confidence
/// high - bound and load are explicit.
pub unsafe fn fn_80271b90(this: usize, index: i32) -> u32 {
    if index > 0xf {
        return 0;
    }
    let addr = (this as isize)
        .wrapping_add((index as isize).wrapping_mul(4))
        .wrapping_add(0x100) as usize;
    platform::read_u32(addr)
}

// ===========================================================================
// 0x80271CE0
// ===========================================================================

/// Returns the address of one track-value slot, local or global.
///
/// Slots `0..15` live in the control object at `+0x140`; slots `16..31` live in
/// the global table at `0x803FCC68`. Anything else returns null.
///
/// # Original
/// - Ghidra: `FUN_80271ce0` at `0x80271CE0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271ce0_FUN_80271ce0.c`
/// - observed: `if (param_2 < 0x10) return (u16 *)(param_1 + param_2 * 2 +
///   0x140); if (0x1f < param_2) return 0; return &DAT_803fcc68 + param_2 -
///   0x10;` (pointer arithmetic in `u16` units).
///
/// # Arguments
/// - `this`: control object.
/// - `index`: logical slot number (signed, as in the C).
///
/// # Returns
/// A `u16 *` address, or `0`.
///
/// # Safety
/// The returned address must be readable/writable by the caller.
///
/// # Confidence
/// high - both ranges and the global fallback are explicit.
pub unsafe fn fn_80271ce0(this: usize, index: i32) -> usize {
    if index < 0x10 {
        return (this as isize)
            .wrapping_add((index as isize).wrapping_mul(2))
            .wrapping_add(0x140) as usize;
    }
    if index > 0x1f {
        return 0;
    }
    (SEQ_GLOBAL_TRACK_VALUES as isize).wrapping_add((index as isize - 0x10).wrapping_mul(2)) as usize
}

// ===========================================================================
// 0x80272340
// ===========================================================================

/// Forwards a three-argument notify call through the player's `+0xF0` vtable.
///
/// Ghidra warns that it could not recover a jump table at `0x80272364` and
/// treats the indirect branch as a call; the instruction is a plain virtual
/// call. The receiver is `*(u32 *)(this + 0xF0)` and the method is vtable slot
/// `+0x0C`, invoked with `(receiver, this, param_2, param_3)`.
///
/// # Original
/// - Ghidra: `FUN_80272340` at `0x80272340` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272340_FUN_80272340.c`
/// - observed: `(**(code **)(**(int **)(param_1 + 0xf0) + 0xc))(...)` with four
///   arguments.
///
/// # Arguments
/// - `this`: player object.
/// - `param_2`, `param_3`: forwarded unchanged.
///
/// # Side effects
/// Depends entirely on the virtual method.
///
/// # Safety
/// `+0xF0` must point at an object with a readable vtable and a valid `+0x0C`
/// slot.
///
/// # Confidence
/// medium - the call shape is explicit; the notify semantics are inferred from
/// the `0x802723A0` constructor that installs the table.
pub unsafe fn fn_80272340(this: usize, param_2: u32, param_3: u32) {
    let receiver = platform::read_u32(this + 0xf0) as usize;
    let vtable = platform::read_u32(receiver) as usize;
    let method = platform::read_u32(vtable + 0x0c) as usize;
    platform::call4(method, receiver as u32, this as u32, param_2, param_3);
}

// ===========================================================================
// 0x802723A0
// ===========================================================================

/// Constructs the outer sequence player around an embedded control object.
///
/// The player's base constructor runs first, then the player vtable and its
/// second table pointer are installed, a control object is constructed at
/// `+0x110`, and the remaining words/bytes are cleared. The caller's second
/// argument is stored at `+0x278`.
///
/// # Original
/// - Ghidra: `FUN_802723a0` at `0x802723A0` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802723a0_FUN_802723a0.c`
/// - observed: `FUN_80265ff0(param_1,param_3,param_4)`, `*param_1 =
///   &DAT_8034f000`, `FUN_80270eb0(param_1 + 0x44)` (word offset `0x44` =
///   byte `0x110`), the byte stores at `+0x288`/`+0x289`/`+0x49C`, and the
///   word stores at `+0x274`, `+0x278`, `+0x284`, `+0x28C`, `+0x490`
///   (`0x8034F038`), `+0x494`, `+0x498`, `+0x4A0`, `+0x4A4`, `+0x4AC`, `+0x4B0`.
///
/// # Arguments
/// - `this`: player object to construct.
/// - `param_2`: caller word stored at `+0x278`.
/// - `param_3`, `param_4`: forwarded to the base constructor.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - calls `0x80265FF0` and [`fn_80270eb0`],
/// - initialises the player field set above.
///
/// # Safety
/// `this` must be writable for at least `0x4B4` bytes.
///
/// # Confidence
/// medium - every store is explicit; why the two table pointers exist and what
/// the caller words mean is inferred from the surrounding player code.
pub unsafe fn fn_802723a0(this: usize, param_2: u32, param_3: u32, param_4: u32) -> usize {
    platform::call3(FUN_80265FF0, this as u32, param_3, param_4);

    platform::write_u32(this, VT_SEQ_PLAYER);
    fn_80270eb0(this + 0x110);

    platform::write_u8(this + 0x288, 0);
    platform::write_u8(this + 0x289, 0);

    platform::write_u32(this + 0x274, 0);
    platform::write_u32(this + 0x278, param_2);
    platform::write_u32(this + 0x284, 0);
    platform::write_u32(this + 0x28c, 0);

    platform::write_u32(this + 0x494, 0);
    platform::write_u32(this + 0x498, 0);
    platform::write_u8(this + 0x49c, 0);

    platform::write_u32(this + 0x490, VT_SEQ_PLAYER_B);

    platform::write_u32(this + 0x4a0, 0);
    platform::write_u32(this + 0x4a4, 0);
    platform::write_u32(this + 0x4ac, 0);
    platform::write_u32(this + 0x4b0, 0);

    this
}

// ===========================================================================
// 0x802730D0 / 0x802730E0
// ===========================================================================

/// Writes a track's start and end offsets.
///
/// The end is computed as `start + length` with 32-bit wrapping, exactly like
/// the original `add`.
///
/// # Original
/// - Ghidra: `FUN_802730d0` at `0x802730D0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802730d0_FUN_802730d0.c`
/// - observed: `*(int *)(param_1 + 0x1c) = param_2; *(int *)(param_1 + 0x20) =
///   param_2 + param_3;`
///
/// # Arguments
/// - `track`: track object.
/// - `start`: start offset.
/// - `length`: length added to `start`.
///
/// # Side effects
/// Writes `+0x1C` and `+0x20`.
///
/// # Safety
/// `track` must be writable.
///
/// # Confidence
/// high - both stores are explicit.
pub unsafe fn fn_802730d0(track: usize, start: u32, length: u32) {
    platform::write_u32(track + 0x1c, start);
    platform::write_u32(track + 0x20, start.wrapping_add(length));
}

/// Arms a track.
///
/// Clears the two "playing"/"pending" bytes and the word at `+0x44`, then sets
/// byte `+5` to `1`.
///
/// # Original
/// - Ghidra: `FUN_802730e0` at `0x802730E0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802730e0_FUN_802730e0.c`
/// - observed: `+0x4A = 0`, `+0x40 = 0`, `+0x44 = 0`, `+5 = 1`.
///
/// # Arguments
/// - `track`: track object.
///
/// # Side effects
/// Writes one word and three bytes.
///
/// # Safety
/// `track` must be writable.
///
/// # Confidence
/// high - all four stores are explicit.
pub unsafe fn fn_802730e0(track: usize) {
    platform::write_u8(track + 0x4a, 0);
    platform::write_u8(track + 0x40, 0);
    platform::write_u32(track + 0x44, 0);
    platform::write_u8(track + 5, 1);
}

// ===========================================================================
// 0x80273100
// ===========================================================================

/// Tears down a control object's linked track list.
///
/// The list head is `+0xC8`; each node links to the next through `+0xD4`. The
/// routine locks twice, calls `0x80273560(this)`, walks the list calling
/// `0x802686A0` for nodes whose byte `+0x36` is set, and unlocks once. It then
/// locks again, walks the list calling `0x80268C40` on every node, clears the
/// head and unlocks. Finally it clears byte `+5` and releases the outer lock.
///
/// # Original
/// - Ghidra: `FUN_80273100` at `0x80273100` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273100_FUN_80273100.c`
/// - observed: `FUN_8027b5f0`/`0x801B2E60` twice, `FUN_80273560(param_1)`, the
///   `+0xC8`/`+0xD4` walk with `FUN_802686a0`, an unlock, a re-lock, the second
///   walk with `FUN_80268c40`, `*(u32 *)(param_1 + 200) = 0`, unlock,
///   `*(u8 *)(param_1 + 5) = 0`, unlock.
///
/// # Arguments
/// - `this`: object whose track list is released.
///
/// # Side effects
/// - calls into the track manager three times per run,
/// - frees/detaches every list node,
/// - takes/releases the sequence mutex (two nested at the start).
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid null-terminated list.
///
/// # Confidence
/// medium - the loop and mutex pairs are explicit; the meanings of the node
/// flags and of `0x80273560` are inferred.
pub unsafe fn fn_80273100(this: usize) {
    lock_seq_manager();
    lock_seq_manager();

    platform::call1(FUN_80273560, this as u32);

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        if platform::read_u8(node + 0x36) != 0 {
            platform::call1(FUN_802686A0, node as u32);
        }
        node = platform::read_u32(node + 0xd4) as usize;
    }

    unlock_seq_manager();

    lock_seq_manager();

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        platform::call1(FUN_80268C40, node as u32);
        node = platform::read_u32(node + 0xd4) as usize;
    }

    platform::write_u32(this + 0xc8, 0);
    unlock_seq_manager();

    platform::write_u8(this + 5, 0);
    unlock_seq_manager();
}

// ===========================================================================
// 0x80271000
// ===========================================================================

/// Destroys the sequence-control object.
///
/// The object is the incoming `this` and the second incoming word is a signed
/// counter: when the object is non-null its vtable and two table pointers are
/// reinstalled, both registrations are released, and every live track is torn
/// down exactly as in [`fn_80271640`]. When the counter is positive
/// `0x802B9280` destroys the object.
///
/// # Original
/// - Ghidra: `FUN_80271000` at `0x80271000` (392 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271000_FUN_80271000.c`
/// - Removed: the modelled `_savegpr_27` prologue call (`0x802DCD74`) and both
///   `_restgpr_27` epilogue calls (`0x802DCDC0`); `this`/`counter` are the
///   incoming `r3`/`r4`.
/// - observed: the `DAT_8034ef98` / `DAT_8034efbc` / `DAT_8034efd0` stores, two locks, the
///   `+0xCD` / `+0x33` (word form of `+0xCC`) release branches, the 16-slot
///   teardown loop, two unlocks and `if (0 < (int)uVar7) FUN_802b9280(puVar1);`.
/// - `// inferred:` the incoming second word is a signed counter; only the
///   `> 0` test is reproduced.
///
/// # Arguments
/// - `this`: sequence-control object (incoming `r3`).
/// - `counter`: signed release counter (incoming `r4`).
///
/// # Side effects
/// - releases registrations and destroys tracks,
/// - calls `0x802B9280`.
///
/// # Safety
/// `this` must be a live sequence-control object when non-null.
///
/// # Confidence
/// medium - the branch and teardown loop are explicit; the second word's role
/// is inferred from the single comparison.
pub unsafe fn fn_80271000(this: usize, counter: u32) {
    if this == 0 {
        return;
    }

    platform::write_u32(this, VT_SEQ_CTRL);
    platform::write_u32(this + 0xbc, VT_SEQ_CTRL_B);
    platform::write_u32(this + 0xc8, VT_SEQ_CTRL_C);

    lock_seq_manager();
    lock_seq_manager();

    if platform::read_u8(this + 0xcd) != 0 {
        let object = if this != 0 { this + 0xc0 } else { this };
        let manager = call0_r(SEQ_MANAGER);
        platform::call2(FUN_8027BA90, manager, object as u32);
        platform::write_u8(this + 0xcd, 0);
    }

    if platform::read_u8(this + 0xcc) != 0 {
        let object = if this != 0 { this + 0xb4 } else { this };
        let owner = call0_r(FUN_80268C60);
        platform::call2(FUN_80229190, owner, object as u32);
        platform::write_u8(this + 0xcc, 0);
    }

    let mut slot = 0usize;
    while slot < 16 {
        lock_seq_manager();
        let track = platform::read_u32(this + slot * 4 + 0x100);
        if track == 0 {
            unlock_seq_manager();
        } else {
            platform::call1(FUN_80273100, track);
            let manager = platform::read_u32(this + 0xf4) as usize;
            let vtable = platform::read_u32(manager) as usize;
            let destroy = platform::read_u32(vtable + 0x10) as usize;
            platform::call2(destroy, manager as u32, track);
            platform::write_u32(this + slot * 4 + 0x100, 0);
            unlock_seq_manager();
        }
        slot += 1;
    }

    unlock_seq_manager();
    unlock_seq_manager();

    if (counter as i32) > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }
}

// ===========================================================================
// 0x802712B0
// ===========================================================================

/// Rebuilds the sequence-control object from a bank object and a track mask.
///
/// `this` is the control object and `bank` the bank object. The routine takes
/// the sequence mutex three times,
/// releases both registrations, tears down all 16 tracks, resets play state
/// through [`fn_80271190`], counts the set bits of `mask`, and compares the
/// count with the bank's vtable slot `+0x14` result. When the bank cannot
/// supply that many tracks it reports `1`; otherwise it
/// creates one track per set bit (through bank slot `+0x0C`), stores the
/// track and its slot index through [`fn_80272d80`], re-registers the
/// `+0xB4` registration, stores the bank at `+0xF4`, sets `+0xCC`, and
/// reports `0`.
///
/// # Original
/// - Ghidra: `FUN_802712b0` at `0x802712B0` (660 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802712b0_FUN_802712b0.c`
/// - Removed: the modelled `_savegpr_24` prologue call (`0x802DCD68`) and the
///   `_restgpr_24` epilogue call (`0x802DCDB4`); `this`/`bank` are the
///   incoming `r3`/`r4` and the outcome is returned directly.
/// - observed: three `0x801B2E60` calls, the `+0xCD` / `+0xCC` release
///   branches, the 16-slot teardown loop, two `0x801B2F40` calls,
///   `FUN_80271190(iVar5, param_4, param_5)`, the `uVar4 & 1` / `uVar4 >> 1`
///   bit count, `(++*piVar7 + 0x14)`, the `if (iVar8 < iVar1)` split, the
///   per-bit `(++*piVar7 + 0xC)` creation with `FUN_80272d80`, the
///   `FUN_80268d50` re-registration, `*(int **)(iVar5 + 0xf4) = piVar7`,
///   `+0xCC = 1` and the `1`/`0` return.
/// - `// inferred:` the customer's two caller words are forwarded unchanged;
///   `FUN_80273100` is printed with no argument and the live track in `r3` is
///   passed, as in [`fn_80271640`].
///
/// # Arguments
/// - `this`: sequence-control object (incoming `r3`).
/// - `bank`: bank object (incoming `r4`).
/// - `mask`: one bit per requested track slot.
/// - `param_4`, `param_5`: caller words forwarded to [`fn_80271190`].
///
/// # Side effects
/// - releases and re-creates registrations and tracks,
/// - takes the `0x801B1140` lock and returns the outcome.
///
/// # Safety
/// `bank` must implement the `+0x0C`/`+0x14` slots; `this` must be live.
///
/// # Confidence
/// medium - the teardown, bit walk and creation loop are explicit; the
/// dropped `FUN_80273100` argument is inferred.
pub unsafe fn fn_802712b0(this: usize, bank: u32, mask: u32, param_4: u32, param_5: u32) -> u32 {
    lock_seq_manager();
    lock_seq_manager();
    lock_seq_manager();

    if platform::read_u8(this + 0xcd) != 0 {
        let object = if this != 0 { this + 0xc0 } else { this };
        let manager = call0_r(SEQ_MANAGER);
        platform::call2(FUN_8027BA90, manager, object as u32);
        platform::write_u8(this + 0xcd, 0);
    }

    if platform::read_u8(this + 0xcc) != 0 {
        let object = if this != 0 { this + 0xb4 } else { this };
        let owner = call0_r(FUN_80268C60);
        platform::call2(FUN_80229190, owner, object as u32);
        platform::write_u8(this + 0xcc, 0);
    }

    let mut slot = 0usize;
    while slot < 16 {
        lock_seq_manager();
        let track = platform::read_u32(this + slot * 4 + 0x100);
        if track == 0 {
            unlock_seq_manager();
        } else {
            platform::call1(FUN_80273100, track);
            let manager = platform::read_u32(this + 0xf4) as usize;
            let vtable = platform::read_u32(manager) as usize;
            let destroy = platform::read_u32(vtable + 0x10) as usize;
            platform::call2(destroy, manager as u32, track);
            platform::write_u32(this + slot * 4 + 0x100, 0);
            unlock_seq_manager();
        }
        slot += 1;
    }

    unlock_seq_manager();
    unlock_seq_manager();

    fn_80271190(this, param_4, param_5);

    let token = call0_r(FUN_801B1140);
    let mut count = 0i32;
    let mut bits = mask;
    loop {
        if bits == 0 {
            break;
        }
        if bits & 1 != 0 {
            count += 1;
        }
        bits >>= 1;
    }

    let bank_vtable = platform::read_u32(bank as usize) as usize;
    let count_slot = platform::read_u32(bank_vtable + 0x14) as usize;
    let capacity = platform::call1(count_slot, bank);

    let outcome: u32;
    if (capacity as i32) < count {
        platform::call1(FUN_801B1180, token);
        unlock_seq_manager();
        outcome = 1;
    } else {
        let mut iter = 0usize;
        let mut remaining = mask;
        let mut base = this;
        loop {
            if remaining == 0 {
                break;
            }
            if remaining & 1 != 0 {
                let create_slot = platform::read_u32(bank_vtable + 0x0c) as usize;
                let track = platform::call2(create_slot, bank, this as u32);
                lock_seq_manager();
                if iter < 16 {
                    platform::write_u32(base + 0x100, track);
                    fn_80272d80(track as usize, iter as u8);
                    unlock_seq_manager();
                } else {
                    unlock_seq_manager();
                }
            }
            remaining >>= 1;
            base += 4;
            iter += 1;
        }
        platform::call1(FUN_801B1180, token);
        let object = if this != 0 { this + 0xb4 } else { this };
        let owner = call0_r(FUN_80268C60);
        platform::call2(FUN_80268D50, owner, object as u32);
        platform::write_u32(this + 0xf4, bank);
        platform::write_u8(this + 0xcc, 1);
        unlock_seq_manager();
        outcome = 0;
    }

    outcome
}

// ===========================================================================
// 0x802718E0 / 0x802718F0 / 0x80271900
// ===========================================================================

/// Stores a byte at the control object's `+0xE9`.
///
/// # Original
/// - Ghidra: `FUN_802718e0` at `0x802718E0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802718e0_FUN_802718e0.c`
/// - observed: `*(undefined1 *)(param_1 + 0xe9) = param_2;`
///
/// # Arguments
/// - `this`: control object.
/// - `value`: byte stored at `+0xE9`.
///
/// # Side effects
/// Writes one byte.
///
/// # Safety
/// `this` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_802718e0(this: usize, value: u8) {
    platform::write_u8(this + 0xe9, value);
}

/// Stores a byte at the control object's `+0xCF`.
///
/// # Original
/// - Ghidra: `FUN_802718f0` at `0x802718F0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802718f0_FUN_802718f0.c`
/// - observed: `*(undefined1 *)(param_1 + 0xcf) = param_2;`
///
/// # Arguments
/// - `this`: control object.
/// - `value`: byte stored at `+0xCF`.
///
/// # Side effects
/// Writes one byte.
///
/// # Safety
/// `this` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_802718f0(this: usize, value: u8) {
    platform::write_u8(this + 0xcf, value);
}

/// Stores the optional callback and its context at `+0xF8` / `+0xFC`.
///
/// # Original
/// - Ghidra: `FUN_80271900` at `0x80271900` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271900_FUN_80271900.c`
/// - observed: `*(undefined4 *)(param_1 + 0xf8) = param_2;
///   *(undefined4 *)(param_1 + 0xfc) = param_3;`
///
/// # Arguments
/// - `this`: control object.
/// - `callback`: callback word (`+0xF8`).
/// - `context`: callback context (`+0xFC`).
///
/// # Side effects
/// Writes two words.
///
/// # Safety
/// `this` must be writable.
///
/// # Confidence
/// high - both stores are explicit.
pub unsafe fn fn_80271900(this: usize, callback: u32, context: u32) {
    platform::write_u32(this + 0xf8, callback);
    platform::write_u32(this + 0xfc, context);
}

// ===========================================================================
// 0x80271BB0
// ===========================================================================

/// Advances every live track once and reports whether any track is armed.
///
/// `this` is the control object and `bank` the bank object. The routine takes
/// the sequence mutex once, and for each of the 16 slots calls [`fn_802731d0`]
/// and [`fn_802732a0`] on the live track. A negative result from `0x802732a0`
/// ends that track through the same teardown as [`fn_80271640`]. If any
/// track has byte `+5` set, the routine reports `0` (the "still playing"
/// answer); otherwise it reports `1`.
///
/// # Original
/// - Ghidra: `FUN_80271bb0` at `0x80271BB0` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271bb0_FUN_80271bb0.c`
/// - Removed: the modelled `_savegpr_24` prologue call (`0x802DCD68`) and the
///   `_restgpr_24` epilogue call (`0x802DCDB4`); `this`/`bank` are the
///   incoming `r3`/`r4` and the outcome is returned directly.
/// - observed: one `FUN_801b2e60`, the 16-iteration loop with `FUN_802731d0(iVar6)`,
///   `iVar3 = FUN_802732a0(iVar6, (int)uVar9)`, the `if (iVar3 < 0)` teardown
///   branch, the `if (*(char *)(iVar6 + 5) != '\0') bVar1 = true;` flag, a
///   final `FUN_801b2f40`, and the `!bVar1` return.
/// - `// inferred:` `FUN_80273100` is printed with no argument and the live
///   track is passed, as in [`fn_80271640`].
///
/// # Arguments
/// - `this`: sequence-control object (incoming `r3`).
/// - `bank`: bank object (incoming `r4`).
///
/// # Side effects
/// - advances all live tracks and may destroy a finished track,
/// - returns the outcome.
///
/// # Safety
/// `this` must be live.
///
/// # Confidence
/// medium - the loop, negative-result branch and flag are explicit.
pub unsafe fn fn_80271bb0(this: usize, bank: u32) -> u32 {
    lock_seq_manager();

    let mut armed = false;
    let mut slot = 0usize;
    while slot < 16 {
        let track = platform::read_u32(this + slot * 4 + 0x100);
        if track != 0 {
            fn_802731d0(track as usize);
            let step = fn_802732a0(track as usize, bank);
            if (step as i32) < 0 {
                lock_seq_manager();
                let current = platform::read_u32(this + slot * 4 + 0x100);
                if current == 0 {
                    unlock_seq_manager();
                } else {
                    platform::call1(FUN_80273100, current);
                    let manager = platform::read_u32(this + 0xf4) as usize;
                    let vtable = platform::read_u32(manager) as usize;
                    let destroy = platform::read_u32(vtable + 0x10) as usize;
                    platform::call2(destroy, manager as u32, current);
                    platform::write_u32(this + slot * 4 + 0x100, 0);
                    unlock_seq_manager();
                }
            }
            if platform::read_u8(track as usize + 5) != 0 {
                armed = true;
            }
        }
        slot += 1;
    }

    unlock_seq_manager();
    if armed { 0 } else { 1 }
}

// ===========================================================================
// 0x802724A0
// ===========================================================================

/// Resets the sequence player's armed word after a base member reset.
///
/// The routine forwards `this` to `0x802660D0` (the member-record reset shared
/// with the sound-element block) and clears the player's `+0x284` word, which
/// [`fn_802725d0`] also writes and [`fn_802726d0`] tests as the clock delta.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x802724A0 (slot 6 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x802724A0..0x802724D4` (52 bytes;
///   the disassembly is `work/wave289/c_dis_seq.txt`).
/// - observed: the prologue `stwu r1,-16(r1)` / `mflr r0`, `mr r31,r3`,
///   `bl 0x802660D0`, `li r0,0`, `stw r0,0x284(r31)` and the `blr`.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Side effects
/// - resets the embedded member record,
/// - clears `+0x284`.
///
/// # Safety
/// `this` must be a live player whose base member record is initialised.
///
/// # Confidence
/// high - the callee, the single store and the frame are literal.
pub unsafe fn fn_802724a0(this: usize) {
    platform::call1(FUN_802660D0, this as u32);
    platform::write_u32(this + 0x284, 0);
}

// ===========================================================================
// 0x802724E0
// ===========================================================================

/// Restarts a player around a freshly built control object.
///
/// The routine notifies the player's vtable through the `+0x20` slot, reads
/// the inner word from `0x80267380`, and rebuilds the embedded control object
/// at `+0x110` through [`fn_802712b0`].
///
/// # Original
/// - Ghidra: `FUN_802724e0` at `0x802724E0` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802724e0_FUN_802724e0.c`
/// - observed: `(**(code **)(*param_1 + 0x20))();`, `uVar1 = FUN_80267380(param_1);`,
///   `FUN_802712b0(param_1 + 0x44, param_2, param_3, uVar1, param_4);`
///   (`0x44` word offset is byte `+0x110`).
///
/// # Arguments
/// - `this`: player object.
/// - `param_2`, `param_3`: forwarded to [`fn_802712b0`] as mask/args.
/// - `param_4`: forwarded as its last argument.
///
/// # Side effects
/// - calls the `+0x20` notify slot, `0x80267380` and [`fn_802712b0`].
///
/// # Safety
/// `this` must point at a live player with a readable vtable.
///
/// # Confidence
/// medium - the three calls are explicit; the notify slot's meaning is
/// inferred from the player constructor [`fn_802723a0`].
pub unsafe fn fn_802724e0(this: usize, param_2: u32, param_3: u32, param_4: u32) {
    let vtable = platform::read_u32(this) as usize;
    let notify = platform::read_u32(vtable + 0x20) as usize;
    platform::call0(notify);

    let inner = platform::call1(FUN_80267380, this as u32);
    fn_802712b0(this.wrapping_add(0x110), param_2, param_3, inner, param_4);
}

// ===========================================================================
// 0x80272560
// ===========================================================================

/// Starts the player's slot-0 track and marks the player playing.
///
/// `FUN_80271550(this + 0x110)` starts the track, `FUN_80271820` advances the
/// clock when the delta is positive, and byte `+0x289` is set.
///
/// # Original
/// - Ghidra: `FUN_80272560` at `0x80272560` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272560_FUN_80272560.c`
/// - observed: `FUN_80271550(param_1 + 0x110); if (0 < param_5) {
///   FUN_80271820(param_1 + 0x110, param_4, param_5); } *(undefined1 *)(param_1 + 0x289) = 1;`
/// - `// inferred:` Ghidra drops the two `FUN_80271550` arguments; the
///   caller's `param_2` / `param_3` in `r4` / `r5` are forwarded, matching the
///   `(this, start, length)` shape of [`fn_80271550`].
///
/// # Arguments
/// - `this`: player object.
/// - `start`, `length`: forwarded to [`fn_80271550`].
/// - `mode`: forwarded to [`fn_80271820`].
/// - `delta`: when positive, passed to [`fn_80271820`].
///
/// # Side effects
/// - starts the track and advances the control clock,
/// - sets byte `+0x289`.
///
/// # Safety
/// `this` must be a live player.
///
/// # Confidence
/// medium - the clock guard and byte store are explicit; the two forwarded
/// start arguments are inferred from the callee shape.
pub unsafe fn fn_80272560(this: usize, start: u32, length: u32, mode: u32, delta: u32) {
    fn_80271550(this.wrapping_add(0x110), start, length);
    if (delta as i32) > 0 {
        fn_80271820(this.wrapping_add(0x110), mode, delta);
    }
    platform::write_u8(this + 0x289, 1);
}

// ===========================================================================
// 0x802725D0
// ===========================================================================

/// Stores the player's callback words and registers a cycle callback.
///
/// The four words are stored at `+0x28C`/`+0x27C`/`+0x280`/`+0x284` and byte
/// `+0x288` is set. When the word at `+0x04` is non-null and the chain
/// through the caller's vtable slots `+0x40`/`+0x0C` yields a non-null
/// result, the words at `+0x4A0`/`+0x4A4`/`+0x4A8` are stored, the
/// [`fn_802726d0`] callback is installed at `+0x4AC`/`+0x4B0`, and the object
/// is registered through `0x8027FDD0` / `0x8027FF60`. Otherwise the player's
/// `+0x10` notify slot runs.
///
/// # Original
/// - Ghidra: `FUN_802725d0` at `0x802725D0` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802725d0_FUN_802725d0.c`
/// - observed: `piVar5 = (int *)param_1[1];`, the `param_1[0xa3] / [0x9f] /
///   [0xa0] / [0xa1]` stores and `*(undefined1 *)(param_1 + 0xa2) = 1`, the
///   `+0x40` / `+0x0C` chain, `param_1[0x128] / [0x129] / [0x12a]`,
///   `param_1[299] = (int)FUN_802726d0; param_1[300] = (int)param_1;`,
///   `FUN_8027ff60(FUN_8027fdd0(), param_1 + 0x124, 1)`, and the fallback
///   `(**(code **)(*param_1 + 0x10))(param_1)`.
/// - `// inferred:` word indices convert to byte offsets with `* 4`.
///
/// # Arguments
/// - `this`: player object.
/// - `param_2`: chain owner whose `+0x40`/`+0x0C` slots are used.
/// - `param_3`, `param_4`, `param_5`: words stored at `+0x27C..+0x284`.
///
/// # Side effects
/// - writes the player callback block,
/// - registers the callback or calls the `+0x10` notify slot.
///
/// # Safety
/// `this` and `param_2` must point at live objects with readable vtables.
///
/// # Confidence
/// medium - the stores and two-way branch are explicit; the notify slot's
/// meaning is inferred.
pub unsafe fn fn_802725d0(this: usize, param_2: usize, param_3: u32, param_4: u32, param_5: u32) {
    let holder = platform::read_u32(this + 4) as usize;

    platform::write_u32(this + 0x28c, param_2 as u32);
    platform::write_u32(this + 0x27c, param_3);
    platform::write_u32(this + 0x280, param_4);
    platform::write_u32(this + 0x284, param_5);
    platform::write_u8(this + 0x288, 1);

    let mut registered = false;
    if holder != 0 {
        let owner_vtable = platform::read_u32(param_2) as usize;
        let first = platform::read_u32(owner_vtable + 0x40) as usize;
        let id = platform::call1(first, param_2 as u32);
        let holder_vtable = platform::read_u32(holder) as usize;
        let second = platform::read_u32(holder_vtable + 0x0c) as usize;
        let chained = platform::call2(second, holder as u32, id);
        if chained != 0 {
            platform::write_u32(this + 0x4a0, param_2 as u32);
            platform::write_u32(this + 0x4a4, chained);
            platform::write_u32(this + 0x4a8, id);
            platform::write_u32(this + 0x4ac, 0x8027_26D0);
            platform::write_u32(this + 0x4b0, this as u32);
            let registry = call0_r(FUN_8027FDD0);
            platform::call3(FUN_8027FF60, registry, this.wrapping_add(0x490) as u32, 1);
            registered = true;
        }
    }

    if !registered {
        let vtable = platform::read_u32(this) as usize;
        let notify = platform::read_u32(vtable + 0x10) as usize;
        platform::call1(notify, this as u32);
    }
}

// ===========================================================================
// 0x802726D0
// ===========================================================================

/// Runs the player's per-cycle callback body.
///
/// Byte `+0x288` is cleared. A null first argument stops the player through
/// `0x80266250`; otherwise slot 0 is restarted through [`fn_80271550`], the
/// clock is advanced when `+0x284` is positive, and byte `+0x289` is set.
///
/// # Original
/// - Ghidra: `FUN_802726d0` at `0x802726D0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802726d0_FUN_802726d0.c`
/// - observed: `*(undefined1 *)(param_3 + 0x288) = 0; if (param_1 == 0) {
///   FUN_80266250(param_3, 0); } else { FUN_80271550(param_3 + 0x110, param_2,
///   *(undefined4 *)(param_3 + 0x27c)); if (0 < *(int *)(param_3 + 0x284)) {
///   FUN_80271820(param_3 + 0x110, *(undefined4 *)(param_3 + 0x280)); manual
///   (second call) }; *(undefined1 *)(param_3 + 0x289) = 1; }`
/// - `// inferred:` the second `FUN_80271820` argument is `+0x280` and the
///   third is the `+0x284` word, matching [`fn_80271820`]'s clock call in
///   [`fn_80272560`].
///
/// # Arguments
/// - `param_1`: null stops the player, non-null restarts it.
/// - `param_2`: start word forwarded to [`fn_80271550`].
/// - `this`: player object (third parameter in the C).
///
/// # Side effects
/// - stops or restarts the player and sets byte `+0x289`.
///
/// # Safety
/// `this` must be a live player.
///
/// # Confidence
/// medium - the branch and byte stores are explicit; the second clock
/// argument is reconstructed from the register shape.
pub unsafe fn fn_802726d0(param_1: u32, param_2: u32, this: usize) {
    platform::write_u8(this + 0x288, 0);
    if param_1 == 0 {
        platform::call2(FUN_80266250, this as u32, 0);
    } else {
        let length = platform::read_u32(this + 0x27c);
        fn_80271550(this.wrapping_add(0x110), param_2, length);
        if (platform::read_u32(this + 0x284) as i32) > 0 {
            let mode = platform::read_u32(this + 0x280);
            let delta = platform::read_u32(this + 0x284);
            fn_80271820(this.wrapping_add(0x110), mode, delta);
        }
        platform::write_u8(this + 0x289, 1);
    }
}

// ===========================================================================
// 0x80272750
// ===========================================================================

/// Releases the sequence player's registrations, second vtable object, note
/// record and list link.
///
/// When byte `+0x288` is set the small-element registry from [`fn_8027fdd0`]
/// is asked to drop the `+0x490` object through `0x802802D0`. The `+0x28C`
/// object is then notified through vtable slot `+0x10` and cleared. Finally
/// `0x80267190` tears down the note record and the player is unlinked from the
/// priority list at `+0x278`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272750 (slot 2 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272750..0x8027282C` (220 bytes).
/// - observed: the `lbz r0,0x288(r3)` guard, the `bl 0x8027FDD0` /
///   `addi r4,r29,0x490` / `bl 0x802802D0` pair, the `+0x28C` vtable `+0x10`
///   call with the zero store, `bl 0x80267190`, then the `+0x278` list teardown
///   (`owner+4` unlink, vtable `+0x0C` with `-1`, `0x8026DB00`, unlock).
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Side effects
/// Releases the two registrations, destroys the note record and unlinks the
/// player from its list.
///
/// # Safety
/// `this` must be a live player; its `+0x278` list and `+0x490` object must be
/// intact.
///
/// # Confidence
/// high - every guard, callee and offset is literal.
pub unsafe fn fn_80272750(this: usize) {
    if platform::read_u8(this + 0x288) != 0 {
        let registry = call0_r(FUN_8027FDD0);
        platform::call2(FUN_802802D0, registry, (this + 0x490) as u32);
    }

    let object = platform::read_u32(this + 0x28c) as usize;
    if object != 0 {
        let vtable = platform::read_u32(object) as usize;
        let notify = platform::read_u32(vtable + 0x10) as usize;
        platform::call1(notify, object as u32);
        platform::write_u32(this + 0x28c, 0);
    }

    platform::call1(FUN_80267190, this as u32);
    detach_from_owner_list(this, 0x278);
}

// ===========================================================================
// 0x80272830
// ===========================================================================

/// Deleting destructor of the sequence player.
///
/// The `+0x490` second-vtable object is passed to `0x8027FD90` with a zero
/// flag, the embedded control object at `+0x110` is destroyed through
/// [`fn_80271000`] with `-1`, and a positive flag releases the player itself
/// through `0x802B9280`. The player address is returned in all cases.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272830 (slot 1 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272830..0x8027289C` (108 bytes).
/// - observed: the `cmpwi cr0,r3,0` guard, `addic. r3,r3,0x490`,
///   `bl 0x8027FD90` with `r4 = 0`, `addi r3,r30,0x110` / `li r4,-1` /
///   `bl 0x80271000`, the positive-`r31` guard around `bl 0x802B9280`, and
///   `mr r3,r30` before the epilogue.
///
/// # Arguments
/// - `this`: sequence player, or null.
/// - `flag`: when positive, free `this`.
///
/// # Returns
/// `this`, unchanged.
///
/// # Safety
/// A non-null `this` must be a live player.
///
/// # Confidence
/// high - the guards, all three calls and the returned address are literal.
pub unsafe fn fn_80272830(this: usize, flag: i32) -> u32 {
    if this != 0 {
        // `addic. r3,r3,0x490` forms the second-vtable address and tests it;
        // the image only skips this call when that address is zero.
        platform::call2(FUN_8027FD90, (this + 0x490) as u32, 0);
        fn_80271000(this.wrapping_add(0x110), 0xFFFF_FFFF);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x802728A0 / 0x802728B0 / 0x802728C0
// ===========================================================================

/// Forwards a byte to the embedded control object's `+0xE9`.
///
/// # Original
/// - Ghidra: `FUN_802728a0` at `0x802728A0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802728a0_FUN_802728a0.c`
/// - observed: `FUN_802718e0(param_1 + 0x110);`
/// - `// inferred:` the value in `r4` is forwarded to [`fn_802718e0`], whose
///   second parameter Ghidra drops here.
///
/// # Arguments
/// - `this`: player object.
/// - `value`: byte forwarded to [`fn_802718e0`].
///
/// # Side effects
/// Writes the control object's `+0xE9`.
///
/// # Safety
/// `this` must be a live player.
///
/// # Confidence
/// medium - the call and `+0x110` offset are explicit; the forwarded value
/// is inferred from the callee signature.
pub unsafe fn fn_802728a0(this: usize, value: u8) {
    fn_802718e0(this.wrapping_add(0x110), value);
}

/// Forwards a byte to the embedded control object's `+0xCF`.
///
/// # Original
/// - Ghidra: `FUN_802728b0` at `0x802728B0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802728b0_FUN_802728b0.c`
/// - observed: `FUN_802718f0(param_1 + 0x110);`
/// - `// inferred:` the value in `r4` is forwarded to [`fn_802718f0`].
///
/// # Arguments
/// - `this`: player object.
/// - `value`: byte forwarded to [`fn_802718f0`].
///
/// # Side effects
/// Writes the control object's `+0xCF`.
///
/// # Safety
/// `this` must be a live player.
///
/// # Confidence
/// medium - the call and `+0x110` offset are explicit; the forwarded value
/// is inferred from the callee signature.
pub unsafe fn fn_802728b0(this: usize, value: u8) {
    fn_802718f0(this.wrapping_add(0x110), value);
}

/// Forwards callback and context to the embedded control object.
///
/// # Original
/// - Ghidra: `FUN_802728c0` at `0x802728C0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802728c0_FUN_802728c0.c`
/// - observed: `FUN_80271900(param_1 + 0x110);`
/// - `// inferred:` the callback/context in `r4`/`r5` are forwarded to
///   [`fn_80271900`], whose parameters Ghidra drops here.
///
/// # Arguments
/// - `this`: player object.
/// - `callback`, `context`: forwarded to [`fn_80271900`].
///
/// # Side effects
/// Writes the control object's `+0xF8` / `+0xFC`.
///
/// # Safety
/// `this` must be a live player.
///
/// # Confidence
/// medium - the call and `+0x110` offset are explicit; the forwarded words
/// are inferred from the callee signature.
pub unsafe fn fn_802728c0(this: usize, callback: u32, context: u32) {
    fn_80271900(this.wrapping_add(0x110), callback, context);
}

// ===========================================================================
// 0x802728D0
// ===========================================================================

/// Re-inserts the sequence player into its `+0x278` list in priority order.
///
/// The player's priority is the `+0x98` byte plus the `+0x50` word, clamped to
/// `0..=0x7F`. After unlinking the `+0xF0` node the routine walks the list from
/// `owner + 8` and stops at the first node whose own priority is greater; the
/// node is inserted before it under the `owner + 0x10` lock.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x802728D0 (slot 9 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x802728D0..0x802729B0` (224 bytes).
/// - observed: `lbz r4,0x98(r3)` + `lwz r0,0x50(r3)` and the `cmpwi 0x7F`
///   clamp, `lwz r30,0x278(r3)`, `addi r31,r30,0x10`, the `bl 0x801B2E60`
///   lock, `bl 0x80229190` unlink, the `-0x58`/`-0xA0` node-priority walk and
///   `bl 0x80229160` insert, then `bl 0x801B2F40` unlock.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Side effects
/// Moves the player's `+0xF0` node inside the `+0x278` list.
///
/// # Safety
/// `this` and its `+0x278` list owner must be live.
///
/// # Confidence
/// high - the clamp, list offsets and both list calls are literal.
pub unsafe fn fn_802728d0(this: usize) {
    reinsert_by_priority(this, 0x278);
}

// ===========================================================================
// 0x802729C0
// ===========================================================================

/// Copies a control track value into a word and returns `1`.
///
/// When byte `+0x85` is clear the word is set to `0xFFFF`; otherwise the
/// value comes from [`fn_802719b0`] on the embedded control object. The
/// routine always returns `1` (the "found" answer used by the bank chain).
///
/// # Original
/// - Ghidra: `FUN_802729c0` at `0x802729C0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802729c0_FUN_802729c0.c`
/// - observed: `if (*(char *)(param_1 + 0x85) == '\0') { *param_3 = 0xffff; }
///   else { uVar1 = FUN_802719b0(param_1 + 0x110); *param_3 = uVar1; } return 1;`
/// - `// inferred:` Ghidra drops the index argument to `FUN_802719b0`; the
///   caller's `param_2` in `r4` is forwarded, matching [`fn_802719b0`]'s
///   `(this, index)` shape.
///
/// # Arguments
/// - `this`: player object.
/// - `index`: forwarded to [`fn_802719b0`].
/// - `out`: writable word receiving the value.
///
/// # Returns
/// `1` always.
///
/// # Side effects
/// Writes `*out`.
///
/// # Safety
/// `this` must be a live player; `out` must be writable.
///
/// # Confidence
/// medium - the guard, copy and return are explicit; the forwarded index is
/// inferred from the callee signature.
pub unsafe fn fn_802729c0(this: usize, index: i32, out: usize) -> u32 {
    if platform::read_u8(this + 0x85) == 0 {
        platform::write_u16(out, 0xffff);
    } else {
        let value = fn_802719b0(this.wrapping_add(0x110), index);
        platform::write_u16(out, value as u16);
    }
    1
}

// ===========================================================================
// 0x80272A10
// ===========================================================================

/// Writes a track value into the embedded control object and returns `1`.
///
/// # Original
/// - Ghidra: `FUN_80272a10` at `0x80272A10` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272a10_FUN_80272a10.c`
/// - observed: `FUN_802719c0(param_1 + 0x110, param_2, (int)param_3); return 1;`
///
/// # Arguments
/// - `this`: player object.
/// - `index`: slot forwarded to [`fn_802719c0`].
/// - `value`: `i16` value forwarded to [`fn_802719c0`].
///
/// # Returns
/// `1` always.
///
/// # Side effects
/// Writes one embedded track value.
///
/// # Safety
/// `this` must be a live player.
///
/// # Confidence
/// high - the forwarded call and constant return are explicit.
pub unsafe fn fn_80272a10(this: usize, index: i32, value: i16) -> u32 {
    fn_802719c0(this.wrapping_add(0x110), index, value as u16);
    1
}

// ===========================================================================
// 0x80272A40
// ===========================================================================

/// Returns the word at `+0x270` when the player is enabled, else `0`.
///
/// # Original
/// - Ghidra: `FUN_80272a40` at `0x80272A40` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272a40_FUN_80272a40.c`
/// - observed: `if (*(char *)(param_1 + 0x85) == '\0') { return 0; } return
///   *(undefined4 *)(param_1 + 0x270);`
///
/// # Arguments
/// - `this`: player object.
///
/// # Returns
/// The `+0x270` word, or `0` when byte `+0x85` is clear.
///
/// # Safety
/// `this` must be readable.
///
/// # Confidence
/// high - the guard and two returns are explicit.
pub unsafe fn fn_80272a40(this: usize) -> u32 {
    if platform::read_u8(this + 0x85) == 0 {
        return 0;
    }
    platform::read_u32(this + 0x270)
}

// ===========================================================================
// 0x80272A60
// ===========================================================================

/// Returns whether the sequence player's `+0x274` holder word is non-zero.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272A60 (slot 4 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272A60..0x80272A70` (16 bytes).
/// - observed: `lwz r3,0x274(r3)`, `neg r0,r3`, `or r0,r0,r3`,
///   `rlwinm r3,r0,1,31,31` and `blr` - the branchless `x != 0` idiom.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Returns
/// `1` when `+0x274` is non-zero, else `0`.
///
/// # Safety
/// `this` must be readable at `+0x274`.
///
/// # Confidence
/// high - the branchless non-zero test is the same one used by [`fn_80272a40`].
pub unsafe fn fn_80272a60(this: usize) -> u32 {
    (platform::read_u32(this + 0x274) != 0) as u32
}

// ===========================================================================
// 0x80272A80
// ===========================================================================

/// Detaches the `+0x274` holder from its chain object.
///
/// The holder word is loaded and handed to [`fn_80272d40`], which unlinks it
/// from the chain.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272A80 (slot 5 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272A80..0x80272A84` (8 bytes).
/// - observed: `lwz r3,0x274(r3)` followed by `b 0x80272D40`.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Safety
/// `this + 0x274` must hold a live chain holder or zero.
///
/// # Confidence
/// high - the load and tail branch are literal.
pub unsafe fn fn_80272a80(this: usize) {
    fn_80272d40(platform::read_u32(this + 0x274) as usize);
}

// ===========================================================================
// 0x80272BF0
// ===========================================================================

/// Returns the address of the embedded control object at `+0x110`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272BF0 (slot 7 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272BF0..0x80272BF4` (8 bytes).
/// - observed: `addi r3,r3,0x110` and `blr`.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Returns
/// `this + 0x110`.
///
/// # Safety
/// `this` must be a valid object base.
///
/// # Confidence
/// high - the single addend is literal.
pub unsafe fn fn_80272bf0(this: usize) -> u32 {
    this.wrapping_add(0x110) as u32
}

// ===========================================================================
// 0x80272C00
// ===========================================================================

/// Returns the address of the embedded control object at `+0x110`.
///
/// This is a second slot with the same eight-byte body as [`fn_80272bf0`];
/// both pointers are real entries in `VT_SEQ_PLAYER`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272C00 (slot 8 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272C00..0x80272C04` (8 bytes).
/// - observed: `addi r3,r3,0x110` and `blr`.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Returns
/// `this + 0x110`.
///
/// # Safety
/// `this` must be a valid object base.
///
/// # Confidence
/// high - the single addend is literal.
pub unsafe fn fn_80272c00(this: usize) -> u32 {
    this.wrapping_add(0x110) as u32
}

// ===========================================================================
// 0x80272C10
// ===========================================================================

/// Returns the sequence player's "playing" byte at `+0x289`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272C10 (slot 3 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272C10..0x80272C14` (8 bytes).
/// - observed: `lbz r3,0x289(r3)` and `blr`.
///
/// # Arguments
/// - `this`: sequence player.
///
/// # Returns
/// The byte at `+0x289`, zero-extended.
///
/// # Safety
/// `this + 0x289` must be readable.
///
/// # Confidence
/// high - the byte load is literal; [`fn_80272560`] and [`fn_802726d0`] both
/// set this byte after starting the track.
pub unsafe fn fn_80272c10(this: usize) -> u32 {
    platform::read_u8(this + 0x289) as u32
}

// ===========================================================================
// 0x80272C20
// ===========================================================================

/// Returns the head of the global sequence-player list.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x80272C20 (slot 0 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x80272C20..0x80272C24` (8 bytes).
/// - observed: `addi r3,r13,-18768` (`r13 - 0x4950`) and `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// [`SEQ_PLAYER_LIST`] (`0x8042AD30`).
///
/// # Safety
/// The static list header must exist, as it does in the shipped image.
///
/// # Confidence
/// high - the `r13` displacement is exactly `-0x4950`.
pub unsafe fn fn_80272c20() -> u32 {
    SEQ_PLAYER_LIST as u32
}

// ===========================================================================
// 0x80272C40
// ===========================================================================

/// Links a holder to the track-chain object found through its list.
///
/// When the word at `*owner` is non-null, the routine walks the chain begun
/// at the `+0x08` slot until it reaches the SDA sentinel
/// (`r13 - 0x4950`). Only when that walk succeeds is the holder set to the
/// chain object, and when that object's `+0x18` returns non-zero its `+0x1C`
/// slot runs. Finally `*(chain + 0x274)` is set to the holder.
///
/// # Original
/// - Ghidra: `FUN_80272c40` at `0x80272C40` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272c40_FUN_80272c40.c`
/// - observed: `*param_1 = 0;`, the null checks on `param_2` and `*param_2`,
///   the `(**(code **)(*piVar4 + 8))(piVar4)` chain walk with
///   `if (puVar2 == (undefined4 *)(unaff_r13 + -0x4950))`, the
///   `*param_1 = (int)piVar4` assignment, the `+0x18` / `+0x1C` pair and
///   `*(int **)(*param_1 + 0x274) = param_1;`.
/// - `// inferred:` `r13 = 0x8042F680` (from `src/game/player.rs`), so the
///   sentinel is `0x8042AD30` (`[`SDA_R13_TRACK_SENTINEL`]`).
///
/// # Arguments
/// - `holder`: writable word that receives the chain object or null.
/// - `owner`: pointer to a word holding the chain object.
///
/// # Returns
/// `holder`, unchanged.
///
/// # Side effects
/// - writes `*holder`,
/// - runs the `+0x1C` slot when `+0x18` is non-zero,
/// - writes `+0x274` of the chain object.
///
/// # Safety
/// `holder` and `owner` must be writable/readable; the chain must end with
/// the SDA sentinel or null.
///
/// # Confidence
/// medium - the walk and assignments are explicit; the sentinel address
/// derives from the documented `r13` base.
pub unsafe fn fn_80272c40(holder: usize, owner: usize) -> usize {
    platform::write_u32(holder, 0);
    if owner == 0 {
        return holder;
    }
    let mut chain = platform::read_u32(owner) as usize;
    if chain == 0 {
        return holder;
    }

    let mut found = false;
    loop {
        let vtable = platform::read_u32(chain) as usize;
        let next_slot = platform::read_u32(vtable + 8) as usize;
        let node = platform::call1(next_slot, chain as u32) as usize;
        if node == SDA_R13_TRACK_SENTINEL {
            found = true;
            break;
        }
        if node == 0 {
            break;
        }
        chain = node;
    }

    if found {
        platform::write_u32(holder, chain as u32);
        let vtable = platform::read_u32(chain) as usize;
        let probe = platform::read_u32(vtable + 0x18) as usize;
        if platform::call1(probe, chain as u32) != 0 {
            let notify = platform::read_u32(vtable + 0x1c) as usize;
            platform::call0(notify);
        }
        platform::write_u32(chain + 0x274, holder as u32);
    }

    holder
}

// ===========================================================================
// 0x80272D40
// ===========================================================================

/// Detaches a holder from its chain object and clears it.
///
/// When the chain object exists and its `+0x274` still points at the holder,
/// that back-link is cleared. The holder word itself is then set to null.
///
/// # Original
/// - Ghidra: `FUN_80272d40` at `0x80272D40` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272d40_FUN_80272d40.c`
/// - observed: `iVar1 = *param_1; if ((iVar1 != 0) && (*(int **)(iVar1 + 0x274)
///   == param_1)) { *(undefined4 *)(iVar1 + 0x274) = 0; } if (*param_1 == 0)
///   { return; } *param_1 = 0;`
///
/// # Arguments
/// - `holder`: writable word holding the chain object or null.
///
/// # Side effects
/// - clears the chain object's `+0x274` back-link when it matches,
/// - sets `*holder` to null.
///
/// # Safety
/// `holder` must be writable; a non-null chain object must be readable at
/// `+0x274`.
///
/// # Confidence
/// high - the two guards and two stores are explicit.
pub unsafe fn fn_80272d40(holder: usize) {
    let chain = platform::read_u32(holder) as usize;
    if chain != 0 && platform::read_u32(chain + 0x274) as usize == holder {
        platform::write_u32(chain + 0x274, 0);
    }
    if platform::read_u32(holder) != 0 {
        platform::write_u32(holder, 0);
    }
}

// ===========================================================================
// 0x80272D80
// ===========================================================================

/// Stores a slot index byte at a track's `+0x04`.
///
/// # Original
/// - Ghidra: `FUN_80272d80` at `0x80272D80` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272d80_FUN_80272d80.c`
/// - observed: `*(undefined1 *)(param_1 + 4) = param_2;`
///
/// # Arguments
/// - `track`: track object.
/// - `slot`: byte stored at `+0x04`.
///
/// # Side effects
/// Writes one byte.
///
/// # Safety
/// `track` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_80272d80(track: usize, slot: u8) {
    platform::write_u8(track + 4, slot);
}

// ===========================================================================
// 0x80272D90
// ===========================================================================

/// Constructs a track object and initialises its default fields.
///
/// Installs the `0x8034F050` vtable, clears byte `+5`, initialises the
/// `+0x58` sub-object through `0x8026DB50`, clears the `+0x70..+0x87` counter
/// block and the `+0xC4`/`+0xC8` words, then finishes with
/// [`fn_80272f40`].
///
/// # Original
/// - Ghidra: `FUN_80272d90` at `0x80272D90` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272d90_FUN_80272d90.c`
/// - observed: `*param_1 = &DAT_8034f050;`, `+5 = 0`, `FUN_8026db50(param_1 +
///   0x16)` (word offset `0x16` is byte `+0x58`), the `+0x70`/`+0x71`/`+0x72`/
///   `+0x74`/`+0x76`/`+0x77`/`+0x78`/`+0x7A`/`+0x7C`/`+0x7D`/`+0x7E`/`+0x80`/
///   `+0x82`/`+0x83`/`+0x84`/`+0x86` zero stores, `param_1[0x31] = 0` and
///   `param_1[0x32] = 0` (`+0xC4`/`+0xC8`), then `FUN_80272f40(param_1)`.
/// - `// inferred:` word indices convert to byte offsets with `* 4`.
///
/// # Arguments
/// - `this`: track object to construct.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - calls `0x8026DB50` and [`fn_80272f40`],
/// - writes the field set listed above.
///
/// # Safety
/// `this` must be writable for at least `0xCC` bytes.
///
/// # Confidence
/// high - every store is explicit; only the word-to-byte conversion is
/// mechanical.
pub unsafe fn fn_80272d90(this: usize) -> usize {
    platform::write_u32(this, VT_SEQ_TRACK);
    platform::write_u8(this + 5, 0);

    platform::call1(FUN_8026DB50, this.wrapping_add(0x58) as u32);

    platform::write_u8(this + 0x70, 0);
    platform::write_u8(this + 0x71, 0);
    platform::write_u16(this + 0x72, 0);
    platform::write_u16(this + 0x74, 0);
    platform::write_u8(this + 0x76, 0);
    platform::write_u8(this + 0x77, 0);
    platform::write_u16(this + 0x78, 0);
    platform::write_u16(this + 0x7a, 0);
    platform::write_u8(this + 0x7c, 0);
    platform::write_u8(this + 0x7d, 0);
    platform::write_u16(this + 0x7e, 0);
    platform::write_u16(this + 0x80, 0);
    platform::write_u8(this + 0x82, 0);
    platform::write_u8(this + 0x83, 0);
    platform::write_u16(this + 0x84, 0);
    platform::write_u16(this + 0x86, 0);

    platform::write_u32(this + 0xc4, 0);
    platform::write_u32(this + 0xc8, 0);

    fn_80272f40(this);

    this
}

// ===========================================================================
// 0x80272E30
// ===========================================================================

/// Tears down a track object's linked node list, then destroys it.
///
/// When `this` is non-null its vtable is reinstalled, the sequence mutex is
/// taken twice, [`FUN_80273560`] runs, the list at `+0xC8` (next pointer
/// `+0xD4`) is walked with `FUN_802686A0` for nodes whose byte `+0x36` is
/// set, then re-walked with `FUN_80268C40` on every node. The head and byte
/// `+5` are cleared; when `param_2` is positive `0x802B9280` destroys the
/// object. This mirrors [`fn_80273100`] for a plain track object.
///
/// # Original
/// - Ghidra: `FUN_80272e30` at `0x80272E30` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272e30_FUN_80272e30.c`
/// - observed: the `if (param_1 != 0)` guard, `*param_1 = &DAT_8034f050`,
///   two `FUN_801b2e60` calls, `FUN_80273560(param_1)`, the two
///   `param_1[0x32]` walks (`+0xD4` next, `+0x36` guard), `param_1[0x32] = 0`,
///   `+5 = 0`, the two `FUN_801b2f40` calls and `if (0 < param_2) {
///   FUN_802b9280(param_1); }`.
///
/// # Arguments
/// - `this`: track object, or null.
/// - `destroy`: when positive, destroy the object at the end.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - calls the track-manager helpers and may destroy the object.
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid list when `this` is non-null.
///
/// # Confidence
/// high - the guards, walks and mutex pairs are explicit.
pub unsafe fn fn_80272e30(this: usize, destroy: i32) -> usize {
    if this == 0 {
        return 0;
    }

    platform::write_u32(this, VT_SEQ_TRACK);

    lock_seq_manager();
    lock_seq_manager();

    platform::call1(FUN_80273560, this as u32);

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        if platform::read_u8(node + 0x36) != 0 {
            platform::call1(FUN_802686A0, node as u32);
        }
        node = platform::read_u32(node + 0xd4) as usize;
    }

    unlock_seq_manager();

    lock_seq_manager();

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        platform::call1(FUN_80268C40, node as u32);
        node = platform::read_u32(node + 0xd4) as usize;
    }

    platform::write_u32(this + 0xc8, 0);
    unlock_seq_manager();

    platform::write_u8(this + 5, 0);
    unlock_seq_manager();

    if destroy > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }

    this
}

// ===========================================================================
// 0x80272F40
// ===========================================================================

/// Writes the default field set of a track object.
///
/// The two small-data words (`r2 - 0x4678` and `r2 - 0x4674`) are copied
/// into the slot `+0x08..+0x18`, `+0x6C`, `+0x9C` and `+0xA0` groups; the
/// remaining bytes/words are cleared or set to the fixed defaults (`0x7F`,
/// `0x40`, `0x3C`, `0xFF`, sixteen `0xFFFF` words at `+0xA4..+0xC3`). The
/// `+0x58` sub-object is initialised through `0x8026DB50`.
///
/// # Original
/// - Ghidra: `FUN_80272f40` at `0x80272F40` (396 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80272f40_FUN_80272f40.c`
/// - observed: `uVar1 = *(undefined4 *)(r2 - 0x4678)`, `uVar2 = *(undefined4
///   *)(r2 - 0x4674)`, the `+8 / +0xC / +0x10 / +0x14 / +0x18` stores, the
///   `+0x1C / +0x20` zeros, the `+0x24..+0x27 / +0x40 / +0x44 / +0x48..
///   +0x4C / +0x50 / +0x54` stores, `FUN_8026db50(param_1 + 0x58)`, the
///   `+0x68 / +0x6C / +0x70..+0xC3` block ending with sixteen `0xFFFF` words.
/// - `// inferred:` `r2 = 0x80433080` (from [`SDA2_DEFAULT_A`]), so the two
///   defaults are `0x8042EA08` / `0x8042EA0C`.
///
/// # Arguments
/// - `this`: track object.
///
/// # Side effects
/// - calls `0x8026DB50`,
/// - writes the whole default field set.
///
/// # Safety
/// `this` must be writable for at least `0xC4` bytes.
///
/// # Confidence
/// high - every store is explicit; only the small-data values are read at
/// run time.
pub unsafe fn fn_80272f40(this: usize) {
    let default_a = platform::read_u32(SDA2_TRACK_DEFAULT_A);
    let default_b = platform::read_u32(SDA2_TRACK_DEFAULT_B);

    platform::write_u32(this + 8, default_a);
    platform::write_u32(this + 0x0c, default_a);
    platform::write_u32(this + 0x10, default_b);
    platform::write_u32(this + 0x14, default_b);
    platform::write_u32(this + 0x18, default_a);
    platform::write_u32(this + 0x1c, 0);
    platform::write_u32(this + 0x20, 0);

    platform::write_u8(this + 0x24, 1);
    platform::write_u8(this + 0x25, 1);
    platform::write_u8(this + 0x26, 0);
    platform::write_u8(this + 0x27, 0);

    platform::write_u8(this + 0x40, 0);
    platform::write_u32(this + 0x44, 0);
    platform::write_u8(this + 0x48, 0);
    platform::write_u8(this + 0x49, 0);
    platform::write_u8(this + 0x4a, 0);
    platform::write_u8(this + 0x4b, 0);
    platform::write_u8(this + 0x4c, 0);

    platform::write_u32(this + 0x50, 0);
    platform::write_u32(this + 0x54, 0);

    platform::call1(FUN_8026DB50, this.wrapping_add(0x58) as u32);

    platform::write_u8(this + 0x68, 0);
    platform::write_u32(this + 0x6c, default_b);

    platform::write_u8(this + 0x70, 0x7f);
    platform::write_u8(this + 0x71, 0x7f);
    platform::write_u16(this + 0x72, 0);
    platform::write_u16(this + 0x74, 0);
    platform::write_u8(this + 0x76, 0);
    platform::write_u8(this + 0x77, 0);
    platform::write_u16(this + 0x78, 0);
    platform::write_u16(this + 0x7a, 0);
    platform::write_u8(this + 0x7c, 0);
    platform::write_u8(this + 0x7d, 0);
    platform::write_u16(this + 0x7e, 0);
    platform::write_u16(this + 0x80, 0);

    platform::write_u8(this + 0x88, 0x7f);
    platform::write_u8(this + 0x89, 0x7f);
    platform::write_u8(this + 0x82, 0);
    platform::write_u8(this + 0x83, 0);
    platform::write_u16(this + 0x84, 0);
    platform::write_u16(this + 0x86, 0);

    platform::write_u8(this + 0x8a, 2);
    platform::write_u8(this + 0x8b, 0);
    platform::write_u8(this + 0x8c, 0);
    platform::write_u8(this + 0x8d, 0x40);
    platform::write_u8(this + 0x8e, 0x3c);
    platform::write_u8(this + 0x8f, 0);

    platform::write_u8(this + 0x90, 0xff);
    platform::write_u8(this + 0x91, 0xff);
    platform::write_u8(this + 0x92, 0xff);
    platform::write_u8(this + 0x93, 0xff);
    platform::write_u16(this + 0x94, 0xff);

    platform::write_u8(this + 0x96, 0x7f);
    platform::write_u8(this + 0x97, 0);
    platform::write_u8(this + 0x98, 0);
    platform::write_u8(this + 0x99, 0);

    platform::write_u32(this + 0x9c, default_b);
    platform::write_u8(this + 0x9a, 0);
    platform::write_u32(this + 0xa0, default_b);

    let mut offset = 0xa4usize;
    while offset < 0xc4 {
        platform::write_u16(this + offset, 0xffff);
        offset += 2;
    }
}

// ===========================================================================
// 0x802731D0
// ===========================================================================

/// Retires nodes in a track's list while its armed byte is set.
///
/// Guarded by byte `+5`: when clear the routine only unlocks and returns.
/// Otherwise it walks the list at `+0xC8` (next via `+0xD4`), decrements each
/// node's counter at `+0xB0` when positive, retires a node whose counter
/// reaches zero (and whose word `+0x00` is not `4`) through `0x80268710`
/// when byte `+0x4C` is clear, and signals a node whose byte `+0x38` is
/// clear through `0x802688C0`.
///
/// # Original
/// - Ghidra: `FUN_802731d0` at `0x802731D0` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802731d0_FUN_802731d0.c`
/// - observed: lock; `if (*(char *)(param_1 + 5) == '\0')` unlock; else the
///   `*(int **)(param_1 + 200)` walk with `piVar2[0x35]` (`+0xD4`) next,
///   `if (0 < piVar2[0x2c]) decrement`, `if (== 0 && *piVar2 != 4 && +0x4C
///   == 0) FUN_80268710`, `if (+0x38 == 0) FUN_802688c0(piVar2, 1)`, unlock.
/// - `// inferred:` word indices convert with `* 4` (`0x35 -> +0xD4`,
///   `0x2C -> +0xB0`, `0xE -> +0x38`).
///
/// # Arguments
/// - `this`: track object.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - decrements node counters and retires/signals nodes.
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid list.
///
/// # Confidence
/// medium - the guard and walk are explicit; the counter/flag roles are
/// inferred from the comparisons.
pub unsafe fn fn_802731d0(this: usize) {
    lock_seq_manager();

    if platform::read_u8(this + 5) == 0 {
        unlock_seq_manager();
        return;
    }

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        let counter = platform::read_u32(node + 0xb0) as i32;
        if counter > 0 {
            platform::write_i32(node + 0xb0, counter - 1);
        }
        let reached = platform::read_u32(node + 0xb0) == 0;
        if reached
            && platform::read_u32(node) != 4
            && platform::read_u8(this + 0x4c) == 0
        {
            platform::call1(FUN_80268710, node as u32);
        }
        if platform::read_u8(node + 0x38) == 0 {
            platform::call2(FUN_802688C0, node as u32, 1);
        }
        node = platform::read_u32(node + 0xd4) as usize;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x802732A0
// ===========================================================================

/// Advances a track's counters and reports its state.
///
/// Under the sequence mutex: when byte `+5` is clear the routine returns
/// `0`. Otherwise the four counter pairs (`+0x74`/`+0x72`,
/// `+0x7A`/`+0x78`, `+0x80`/`+0x7E`, `+0x86`/`+0x84`) are incremented toward
/// their limits, the `+0x4A` flag is resolved, and the countdown at `+0x44`
/// is decremented. When the countdown expires and `+0x20` is zero the
/// routine returns `1`; when `+0x20` is non-zero it loops the track's
/// `+0x0C` slot until success, then returns `-1`. While the countdown is
/// running it returns `1` without touching the slot.
///
/// # Original
/// - Ghidra: `FUN_802732a0` at `0x802732A0` (396 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802732a0_FUN_802732a0.c`
/// - observed: lock; the `+5` guard returning `0`, the four
///   `*(short *)(param_1 + 0x1d/0x1e/0x20/0x21)` vs byte-offset short
///   comparisons, the `+0x4A` / `param_1[0x32]` check returning `1`, the
///   `param_1[0x11]` decrement, `if (param_1[8] == 0)` returning `1`, else
///   the `+0x0C` loop `do { if ((param_1[0x11] != 0) || +0x4A) goto done;
///   iVar1 = (slot+0xC)(param_1, param_2); } while (iVar1 != 1);` returning
///   `0xFFFFFFFF`, else the running-countdown `return 1`.
/// - `// inferred:` word indices convert with `* 4` (`0x32 -> +0xC8`,
///   `0x11 -> +0x44`, `8 -> +0x20`); the return values mean
///   playing/expired/failed.
///
/// # Arguments
/// - `this`: track object.
/// - `param_2`: forwarded to the `+0x0C` slot.
///
/// # Returns
/// `0` when byte `+5` is clear, `1` when running/expired, `-1` after the
/// slot finally succeeds.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - increments the counter pairs and decrements `+0x44`.
///
/// # Safety
/// `this` must be a live track object.
///
/// # Confidence
/// medium - the counter and branch shapes are explicit; the return-value
/// meanings are inferred from [`fn_80271bb0`]'s ` < 0` test.
pub unsafe fn fn_802732a0(this: usize, param_2: u32) -> i32 {
    lock_seq_manager();

    if platform::read_u8(this + 5) == 0 {
        unlock_seq_manager();
        return 0;
    }

    if (platform::read_u16(this + 0x74) as i16) < (platform::read_u16(this + 0x72) as i16) {
        let v = platform::read_u16(this + 0x74).wrapping_add(1);
        platform::write_u16(this + 0x74, v);
    }
    if (platform::read_u16(this + 0x7a) as i16) < (platform::read_u16(this + 0x78) as i16) {
        let v = platform::read_u16(this + 0x7a).wrapping_add(1);
        platform::write_u16(this + 0x7a, v);
    }
    if (platform::read_u16(this + 0x80) as i16) < (platform::read_u16(this + 0x7e) as i16) {
        let v = platform::read_u16(this + 0x80).wrapping_add(1);
        platform::write_u16(this + 0x80, v);
    }
    if (platform::read_u16(this + 0x86) as i16) < (platform::read_u16(this + 0x84) as i16) {
        let v = platform::read_u16(this + 0x86).wrapping_add(1);
        platform::write_u16(this + 0x86, v);
    }

    if platform::read_u8(this + 0x4a) != 0 {
        if platform::read_u32(this + 0xc8) != 0 {
            unlock_seq_manager();
            return 1;
        }
        platform::write_u8(this + 0x4a, 0);
    }

    let countdown = platform::read_u32(this + 0x44) as i32;
    let expired = countdown < 1
        || {
            let next = countdown - 1;
            platform::write_u32(this + 0x44, next as u32);
            next < 1
        };

    if !expired {
        unlock_seq_manager();
        return 1;
    }

    if platform::read_u32(this + 0x20) == 0 {
        unlock_seq_manager();
        return 1;
    }

    loop {
        if platform::read_u32(this + 0x44) != 0 || platform::read_u8(this + 0x4a) != 0 {
            unlock_seq_manager();
            return 1;
        }
        let vtable = platform::read_u32(this) as usize;
        let step = platform::read_u32(vtable + 0x0c) as usize;
        let result = platform::call2(step, this as u32, param_2);
        if result == 1 {
            break;
        }
    }

    unlock_seq_manager();
    -1
}

// ===========================================================================
// 0x80273430
// ===========================================================================

/// Applies a value to every armed node in a track's list.
///
/// Under the sequence mutex, `0x80273560` runs, then the list at `+0xC8` is
/// walked: for each node whose byte `+0x36` is set, the value is applied
/// through `0x80269990` when `param_2` is non-negative, and the node is
/// cleaned up through `0x802686A0`.
///
/// # Original
/// - Ghidra: `FUN_80273430` at `0x80273430` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273430_FUN_80273430.c`
/// - observed: lock, `FUN_80273560(param_1)`, the `+0xC8`/`+0xD4` walk with
///   `if (*(char *)(iVar1 + 0x36) != '\0') { if (-1 < (int)param_2) {
///   FUN_80269990(iVar1, param_2 & 0xff); } FUN_802686a0(iVar1); }`, unlock.
///
/// # Arguments
/// - `this`: track object.
/// - `value`: when non-negative, applied to each armed node.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - calls `0x80273560`, `0x80269990` and `0x802686A0`.
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid list.
///
/// # Confidence
/// high - the walk and two guarded calls are explicit.
pub unsafe fn fn_80273430(this: usize, value: u32) {
    lock_seq_manager();

    platform::call1(FUN_80273560, this as u32);

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        if platform::read_u8(node + 0x36) != 0 {
            if (value as i32) > -1 {
                platform::call2(FUN_80269990, node as u32, value & 0xff);
            }
            platform::call1(FUN_802686A0, node as u32);
        }
        node = platform::read_u32(node + 0xd4) as usize;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x802734C0
// ===========================================================================

/// Forwards a mode byte to every armed node in a track's list.
///
/// Under the sequence mutex, each node whose byte `+0x36` is set and whose
/// byte `+0x35` disagrees with `param_2` gets `+0x35 = param_2` and the call
/// `0x80281460(*(u32 *)(node + 0xD0), param_2)`.
///
/// # Original
/// - Ghidra: `FUN_802734c0` at `0x802734C0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802734c0_FUN_802734c0.c`
/// - observed: lock, the `+0xC8`/`+0xD4` walk with `if ((*(char *)(iVar1 +
///   0x36) != '\0') && (param_2 != -(uint)*(byte *)(iVar1 + 0x35) >> 0x1f)) {
///   *(char *)(iVar1 + 0x35) = (char)param_2; FUN_80281460(*(undefined4
///   *)(iVar1 + 0xd0), param_2); }`, unlock.
///
/// # Arguments
/// - `this`: track object.
/// - `mode`: byte forwarded to every armed node.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - writes `+0x35` and calls `0x80281460`.
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid list.
///
/// # Confidence
/// medium - the walk and writes are explicit; the negated-byte comparison is
/// preserved verbatim (`param_2 != (byte != 0)`) because the C encodes it as
/// a sign trick.
pub unsafe fn fn_802734c0(this: usize, mode: u32) {
    lock_seq_manager();

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        if platform::read_u8(node + 0x36) != 0 {
            let prev = platform::read_u8(node + 0x35) as u32;
            let prev_flag = prev.wrapping_neg() >> 31;
            if mode != prev_flag {
                platform::write_u8(node + 0x35, mode as u8);
                let voice = platform::read_u32(node + 0xd0);
                platform::call2(FUN_80281460, voice, mode);
            }
        }
        node = platform::read_u32(node + 0xd4) as usize;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80273AE0
// ===========================================================================

/// Releases every node in a track's list and clears the head.
///
/// Under the sequence mutex each node is passed to `FUN_80268C40`, then
/// `+0xC8` is set to null.
///
/// # Original
/// - Ghidra: `FUN_80273ae0` at `0x80273AE0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273ae0_FUN_80273ae0.c`
/// - observed: lock, `for (iVar1 = *(int *)(param_1 + 200); iVar1 != 0; iVar1 =
///   *(int *)(iVar1 + 0xd4)) { FUN_80268c40(iVar1); } *(undefined4 *)(param_1
///   + 200) = 0;`, unlock.
///
/// # Arguments
/// - `this`: track object.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - calls `0x80268C40` per node and clears `+0xC8`.
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid list.
///
/// # Confidence
/// high - the walk and clear are explicit.
pub unsafe fn fn_80273ae0(this: usize) {
    lock_seq_manager();

    let mut node = platform::read_u32(this + 0xc8) as usize;
    while node != 0 {
        platform::call1(FUN_80268C40, node as u32);
        node = platform::read_u32(node + 0xd4) as usize;
    }

    platform::write_u32(this + 0xc8, 0);
    unlock_seq_manager();
}

// ===========================================================================
// 0x80273B50
// ===========================================================================

/// Unlinks one node from a track's list and notifies its owner.
///
/// Under the sequence mutex, modes `0` and `2` first release the node through
/// `0x80268C40`. When the owner's word at `+0xC4` is non-null its vtable slot
/// `+0x58` runs with the node. The node is then removed from the list at
/// `+0xC8` (`+0xD4` next): if it is the head the head advances, otherwise
/// the walk stops at the predecessor and relinks it. The routine unlocks
/// and returns in both cases.
///
/// # Original
/// - Ghidra: `FUN_80273b50` at `0x80273B50` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273b50_FUN_80273b50.c`
/// - observed: lock, `if ((param_2 == 0) || (param_2 == 2)) {
///   FUN_80268c40(param_1); }`, the `*(int **)(param_3 + 0xc4)` lookup with
///   `(**(code **)(*piVar2 + 0x58))(piVar2, param_1)`, then the head-equal
///   branch (`*(undefined4 *)(param_3 + 200) = *(undefined4 *)(param_1 +
///   0xd4)`) and the predecessor walk ending in `*(undefined4 *)(iVar3 +
///   0xd4) = *(undefined4 *)(param_1 + 0xd4)`, with `FUN_801b2f40` on every
///   exit.
///
/// # Arguments
/// - `node`: node to unlink.
/// - `mode`: `0`/`2` also release the node; other values only unlink.
/// - `owner`: track object whose list holds the node.
///
/// # Side effects
/// - takes/releases the sequence mutex,
/// - may call `0x80268C40` and the `+0x58` notify slot,
/// - relinks the `+0xC8` list.
///
/// # Safety
/// `owner + 0xC8` must be a valid list containing `node`.
///
/// # Confidence
/// high - both unlink branches and the early exits are explicit.
pub unsafe fn fn_80273b50(node: usize, mode: i32, owner: usize) {
    lock_seq_manager();

    if mode == 0 || mode == 2 {
        platform::call1(FUN_80268C40, node as u32);
    }

    let notifier = platform::read_u32(owner + 0xc4) as usize;
    if notifier != 0 {
        let vtable = platform::read_u32(notifier) as usize;
        let notify = platform::read_u32(vtable + 0x58) as usize;
        platform::call2(notify, notifier as u32, node as u32);
    }

    let head = platform::read_u32(owner + 0xc8) as usize;
    if head == node {
        let next = platform::read_u32(node + 0xd4);
        platform::write_u32(owner + 0xc8, next);
        unlock_seq_manager();
        return;
    }

    let mut previous = head;
    loop {
        let current = platform::read_u32(previous + 0xd4) as usize;
        if current == 0 {
            unlock_seq_manager();
            return;
        }
        if current == node {
            let next = platform::read_u32(node + 0xd4);
            platform::write_u32(previous + 0xd4, next);
            unlock_seq_manager();
            return;
        }
        previous = current;
    }
}

// ===========================================================================
// 0x80273C40
// ===========================================================================

/// Applies a mode to a track and sets its active byte.
///
/// Under the sequence mutex: mode `0` clears byte `+0x48`; mode `3` releases
/// and finalises every node (through `0x80268C40` and `0x80268790`), clears
/// the list and sets `+0x48`; mode `2` runs the two-phase teardown
/// (`0x80273560` + `0x802686A0`, then `0x80268C40`), clears the list and
/// sets `+0x48`; mode `1` only sets `+0x48`. The outer lock is released on
/// exit. Modes `2`/`3` nest an extra lock/unlock pair around the teardown.
///
/// # Original
/// - Ghidra: `FUN_80273c40` at `0x80273C40` (356 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273c40_FUN_80273c40.c`
/// - observed: lock, the `if (param_2 == 0)` / `else if (param_2 == 3)` /
///   `else if (param_2 == 2)` / `else if (param_2 == 1)` chain with
///   `*(undefined1 *)(param_1 + 0x48)` writes, the mode-3 inner locks and
///   `FUN_80268c40` / `FUN_80268790` walk, the mode-2 `FUN_80273560` /
///   `FUN_802686a0` / `FUN_80268c40` walks, and the final `FUN_801b2f40`.
///
/// # Arguments
/// - `this`: track object.
/// - `mode`: `0`..`3` as above.
///
/// # Side effects
/// - takes/releases the sequence mutex (nested for modes `2`/`3`),
/// - tears down the list for modes `2`/`3` and writes byte `+0x48`.
///
/// # Safety
/// `+0xC8`/`+0xD4` must form a valid list.
///
/// # Confidence
/// high - all four branches and the nested lock pairs are explicit.
pub unsafe fn fn_80273c40(this: usize, mode: i32) {
    lock_seq_manager();

    if mode == 0 {
        platform::write_u8(this + 0x48, 0);
    } else if mode == 3 {
        lock_seq_manager();
        let mut node = platform::read_u32(this + 0xc8) as usize;
        while node != 0 {
            platform::call1(FUN_80268C40, node as u32);
            platform::call1(FUN_80268790, node as u32);
            node = platform::read_u32(node + 0xd4) as usize;
        }
        platform::write_u32(this + 0xc8, 0);
        unlock_seq_manager();
        platform::write_u8(this + 0x48, 1);
    } else if mode == 2 {
        lock_seq_manager();
        platform::call1(FUN_80273560, this as u32);
        let mut node = platform::read_u32(this + 0xc8) as usize;
        while node != 0 {
            if platform::read_u8(node + 0x36) != 0 {
                platform::call1(FUN_802686A0, node as u32);
            }
            node = platform::read_u32(node + 0xd4) as usize;
        }
        unlock_seq_manager();

        lock_seq_manager();
        let mut node = platform::read_u32(this + 0xc8) as usize;
        while node != 0 {
            platform::call1(FUN_80268C40, node as u32);
            node = platform::read_u32(node + 0xd4) as usize;
        }
        platform::write_u32(this + 0xc8, 0);
        unlock_seq_manager();

        platform::write_u8(this + 0x48, 1);
    } else if mode == 1 {
        platform::write_u8(this + 0x48, 1);
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80273DB0
// ===========================================================================

/// Returns the address of one of the first 16 `i16` values of a track.
///
/// # Original
/// - Ghidra: `FUN_80273db0` at `0x80273DB0` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273db0_FUN_80273db0.c`
/// - observed: `if (param_2 < 0x10) { return param_1 + param_2 * 2 + 0xa4; } return 0;`
///
/// # Arguments
/// - `this`: track object.
/// - `index`: signed index (only the `< 0x10` test is applied).
///
/// # Returns
/// `this + index * 2 + 0xA4`, or `0` when `index >= 0x10`.
///
/// # Safety
/// The returned address must be readable/writable by the caller.
///
/// # Confidence
/// high - the bound and scaled address are explicit.
pub unsafe fn fn_80273db0(this: usize, index: i32) -> usize {
    if index < 0x10 {
        return (this as isize)
            .wrapping_add((index as isize).wrapping_mul(2))
            .wrapping_add(0xa4) as usize;
    }
    0
}

// ===========================================================================
// RSAR sound-archive object, 0x80275040..0x802756E0
// ===========================================================================

/// Writes the default float/state block of an `RSAR` playback object.
///
/// Six consecutive words at `+0x30..+0x44` receive the first small-data
/// default, `+0x48/+0x4C/+0x50` and `+0x60` receive the second, `+0x5C`
/// receives the third. `+0x54`, `+0x64` and `+0x68` are cleared and the byte
/// at `+0x58` becomes `1`. The routine then runs the base-class initialiser
/// `0x80238110` (which receives the object implicitly through `r3`) and
/// returns the object.
///
/// # Original
/// - Ghidra: `FUN_80275040` at `0x80275040` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275040_FUN_80275040.c`
/// - observed: `uVar1/uVar2/uVar3 = *(r2 + -0x45E8/-0x45E4/-0x45E0)`, the six
///   `+0x30..+0x44` stores, `+0x48..+0x50`, `+0x54 = 0`,
///   `*(undefined1 *)(param_1 + 0x58) = 1`, `+0x5C`, `+0x60`, `+0x64 = 0`,
///   `+0x68 = 0`, then `FUN_80238110()` and `return param_1;`.
/// - `// inferred:` the three small-data words are floats: the setters
///   [`fn_802750c0`], [`fn_802750d0`] and [`fn_802750e0`] write f32 values to
///   the same three slots.
///
/// # Arguments
/// - `this`: playback object.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - writes the default block `+0x30..+0x68`,
/// - calls the base-class initialiser `0x80238110`.
///
/// # Safety
/// `this` must be writable for `0x69` bytes and must be a live object of the
/// class whose initialiser is `0x80238110`.
///
/// # Confidence
/// high - every store and the tail call are explicit; only the float role of
/// the small-data words is inferred.
pub unsafe fn fn_80275040(this: usize) -> usize {
    let play_a = platform::read_u32(SDA2_RSAR_PLAY_A);
    let play_b = platform::read_u32(SDA2_RSAR_PLAY_B);
    let play_c = platform::read_u32(SDA2_RSAR_PLAY_C);

    let mut offset = 0x30;
    while offset <= 0x44 {
        platform::write_u32(this + offset, play_a);
        offset += 4;
    }

    platform::write_u32(this + 0x48, play_b);
    platform::write_u32(this + 0x4c, play_b);
    platform::write_u32(this + 0x50, play_b);
    platform::write_u32(this + 0x54, 0);
    platform::write_u8(this + 0x58, 1);
    platform::write_u32(this + 0x5c, play_c);
    platform::write_u32(this + 0x60, play_b);
    platform::write_u32(this + 0x64, 0);
    platform::write_u32(this + 0x68, 0);

    platform::call1(FUN_80238110, this as u32);
    this
}

// ===========================================================================
// 0x802750C0
// ===========================================================================

/// Stores a float at `+0x48`.
///
/// # Original
/// - Ghidra: `FUN_802750c0` at `0x802750C0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802750c0_FUN_802750c0.c`
/// - observed: `*(float *)(param_2 + 0x48) = (float)param_1;` - the float
///   arrives in `f1`, the object in `r3`, which Ghidra prints as the second
///   parameter.
///
/// # Arguments
/// - `this`: playback object.
/// - `value`: new float value.
///
/// # Side effects
/// - writes `+0x48`.
///
/// # Safety
/// `this + 0x48` must be writable.
///
/// # Confidence
/// high - a single float store.
pub unsafe fn fn_802750c0(this: usize, value: f32) {
    platform::write_f32(this + 0x48, value);
}

// ===========================================================================
// 0x802750D0
// ===========================================================================

/// Stores a float at `+0x4C`.
///
/// # Original
/// - Ghidra: `FUN_802750d0` at `0x802750D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802750d0_FUN_802750d0.c`
/// - observed: `*(float *)(param_2 + 0x4c) = (float)param_1;`
///
/// # Arguments
/// - `this`: playback object.
/// - `value`: new float value.
///
/// # Side effects
/// - writes `+0x4C`.
///
/// # Safety
/// `this + 0x4C` must be writable.
///
/// # Confidence
/// high - a single float store.
pub unsafe fn fn_802750d0(this: usize, value: f32) {
    platform::write_f32(this + 0x4c, value);
}

// ===========================================================================
// 0x802750E0
// ===========================================================================

/// Stores a float at `+0x50`.
///
/// # Original
/// - Ghidra: `FUN_802750e0` at `0x802750E0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802750e0_FUN_802750e0.c`
/// - observed: `*(float *)(param_2 + 0x50) = (float)param_1;`
///
/// # Arguments
/// - `this`: playback object.
/// - `value`: new float value.
///
/// # Side effects
/// - writes `+0x50`.
///
/// # Safety
/// `this + 0x50` must be writable.
///
/// # Confidence
/// high - a single float store.
pub unsafe fn fn_802750e0(this: usize, value: f32) {
    platform::write_f32(this + 0x50, value);
}

// ===========================================================================
// 0x802750F0
// ===========================================================================

/// Initialises the archive allocator/reader header.
///
/// Installs the vtable pair `0x8034F080`/`0x8034F098`, clears `+0x08`/`+0x0C`,
/// makes `+0x10`/`+0x14` point at the embedded list head (`this + 0x10`),
/// stores the global free-list address `0x803FCC88`, the count `0x20`, the two
/// small-data defaults at `+0x20`/`+0x24`, and clears `+0x28`.
///
/// # Original
/// - Ghidra: `FUN_802750f0` at `0x802750F0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802750f0_FUN_802750f0.c`
/// - observed: the two `&DAT_8034f080` / `&DAT_8034f098` stores,
///   `param_1[2] = 0`, `param_1[3] = 0`, `param_1[4] = param_1 + 4` (offset
///   `0x10`), `param_1[5] = param_1 + 4`, `&DAT_803fcc88`, `0x20`, the two
///   `r2 - 0x45D8` / `r2 - 0x45D4` loads and `param_1[10] = 0`.
/// - `// inferred:` `+0x10`/`+0x14` are the `next`/`prev` links of an
///   intrusive list because both are initialised to the head's own address.
///
/// # Arguments
/// - `this`: allocator object.
///
/// # Side effects
/// - writes `+0x00..+0x28`.
///
/// # Safety
/// `this` must be writable for `0x2C` bytes.
///
/// # Confidence
/// high - every store is explicit.
pub unsafe fn fn_802750f0(this: usize) {
    platform::write_u32(this, VT_RSAR_ALLOC);
    platform::write_u32(this + 4, VT_RSAR_ALLOC_B);
    platform::write_u32(this + 8, 0);
    platform::write_u32(this + 0x0c, 0);
    platform::write_u32(this + 0x10, this.wrapping_add(0x10) as u32);
    platform::write_u32(this + 0x14, this.wrapping_add(0x10) as u32);
    platform::write_u32(this + 0x18, RSAR_FREE_LIST as u32);
    platform::write_u32(this + 0x1c, 0x20);
    platform::write_u32(this + 0x20, platform::read_u32(SDA2_RSAR_ALLOC_A));
    platform::write_u32(this + 0x24, platform::read_u32(SDA2_RSAR_ALLOC_B));
    platform::write_u32(this + 0x28, 0);
}

// ===========================================================================
// 0x80275150
// ===========================================================================

/// Returns the byte size of an archive info record group.
///
/// Reads the seven-word info header of `archive` through [`fn_802757d0`] and
/// returns `(word0 + word2 + word5) * 0x28`. The first argument is not used by
/// the original body.
///
/// # Original
/// - Ghidra: `FUN_80275150` at `0x80275150` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275150_FUN_80275150.c`
/// - observed: `iVar1 = FUN_802757d0(param_2,local_28);` followed by
///   `iVar2 = local_28[0] + local_20 + local_14;` and
///   `return iVar2 * 0x28;`.
/// - `// inferred:` the three summed words are counts of `0x28`-byte records,
///   because the only use of the result is a multiply by `0x28`.
///
/// # Arguments
/// - `_param_1`: unused by the original body.
/// - `archive`: archive object whose `+0x04` word holds the info node.
///
/// # Returns
/// `(out[0] + out[2] + out[5]) * 0x28`, or `0` when the lookup fails.
///
/// # Side effects
/// - none beyond the lookup call.
///
/// # Safety
/// `+0x04` of `archive` must reference a live archive info node.
///
/// # Confidence
/// high - the call, the three-word sum and the multiply are explicit.
pub unsafe fn fn_80275150(_param_1: u32, archive: usize) -> i32 {
    let mut out = [0u32; 7];
    let found = fn_802757d0(archive, out.as_mut_ptr() as usize);

    let mut count = 0u32;
    if found != 0 {
        count = out[0].wrapping_add(out[2]).wrapping_add(out[5]);
    }
    (count as i32).wrapping_mul(0x28)
}

// ===========================================================================
// 0x802751B0
// ===========================================================================

/// Appends one `0x28`-byte element to the archive table at `this + 8`.
///
/// # Original
/// - Ghidra: `FUN_802751b0` at `0x802751B0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802751b0_FUN_802751b0.c`
/// - observed: `FUN_8026d860(param_1 + 8,param_3,param_4,0x28); return 1;`
///
/// # Arguments
/// - `this`: archive object; the table starts at `+0x08`.
/// - `_param_2`: unused by the original body.
/// - `param_3`, `param_4`: forwarded unchanged to `0x8026D860`.
///
/// # Returns
/// Always `1`.
///
/// # Side effects
/// - calls `0x8026D860`, which modifies the table.
///
/// # Safety
/// The table at `+0x08` must be a valid container of `0x28`-byte elements.
///
/// # Confidence
/// high - the guard-free forward and constant return are explicit; the exact
/// meaning of the two forwarded words is unknown.
pub unsafe fn fn_802751b0(this: usize, _param_2: u32, param_3: u32, param_4: u32) -> u32 {
    platform::call4(FUN_8026D860, (this + 8) as u32, param_3, param_4, 0x28);
    1
}

// ===========================================================================
// 0x802753F0
// ===========================================================================

/// Constructs the four-slot `RSAR` node and applies its per-slot limits.
///
/// Stores the vtable `0x8034F0B8` and `owner` at `+0x00`/`+0x04`, constructs
/// the four `0x10`-byte elements at `+0x08` through `0x802DC9A0`, writes the
/// two small-data defaults to `+0x48`/`+0x4C`/`+0x50`, then calls `0x80269D00`
/// on each element with limit `0x7FFFFFFF` for slot 0 and `1` for slots
/// `1..3`.
///
/// # Original
/// - Ghidra: `FUN_802753f0` at `0x802753F0` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802753f0_FUN_802753f0.c`
/// - observed: `*param_1 = &DAT_8034f0b8`, `param_1[1] = param_2`,
///   `FUN_802dc9a0(param_1 + 2,&LAB_80269a30,FUN_80269a50,0x10,4)`, the
///   `uVar2 = *(r2 - 0x45D0)` / `uVar1 = *(r2 - 0x45CC)` loads,
///   `param_1[0x12..0x14]`, and the `do { ... } while (iVar3 < 4)` loop calling
///   `FUN_80269d00(param_1 + iVar4 + 8, ...)`.
/// - `// inferred:` the `1`/`0x7FFFFFFF` values are per-slot item limits.
///
/// # Arguments
/// - `this`: node object.
/// - `owner`: word stored at `+0x04` (the parent archive).
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - constructs the element array and writes `+0x00..+0x50`.
///
/// # Safety
/// `this` must be writable for `0x54` bytes and the container helpers must
/// accept the element layout at `+0x08`.
///
/// # Confidence
/// high - the constructor call, both field writes and the four-iteration loop
/// are explicit.
pub unsafe fn fn_802753f0(this: usize, owner: u32) -> usize {
    platform::write_u32(this, VT_RSAR_NODE);
    platform::write_u32(this + 4, owner);

    platform::call5(
        FUN_802DC9A0,
        (this + 8) as u32,
        LAB_80269A30 as u32,
        FUN_80269A50 as u32,
        0x10,
        4,
    );

    let node_a = platform::read_u32(SDA2_RSAR_NODE_A);
    let node_b = platform::read_u32(SDA2_RSAR_NODE_B);
    platform::write_u32(this + 0x48, node_a);
    platform::write_u32(this + 0x4c, node_a);
    platform::write_u32(this + 0x50, node_b);

    let mut slot = 0usize;
    while slot < 4 {
        let limit = if slot == 0 { 0x7fff_ffff } else { 1 };
        platform::call2(FUN_80269D00, (this + slot * 0x10 + 8) as u32, limit);
        slot += 1;
    }

    this
}

// ===========================================================================
// 0x802754B0
// ===========================================================================

/// Destroys an `RSAR` node and its four-slot element array.
///
/// Reinstalls the vtable, tears down each of the four `0x10`-byte elements
/// with `0x80269AF0`, destroys the array through `0x802DCA98`, and frees the
/// node with `0x802B9280` when `destroy` is positive. A null `this` is
/// returned untouched.
///
/// # Original
/// - Ghidra: `FUN_802754b0` at `0x802754B0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802754b0_FUN_802754b0.c`
/// - observed: the `param_1 != 0` guard, `*param_1 = &DAT_8034f0b8`, the
///   four-iteration `FUN_80269af0(param_1 + iVar2 + 8,param_1)` loop,
///   `FUN_802dca98(param_1 + 2,FUN_80269a50,0x10,4)`, the `0 < param_2`
///   free branch and `return param_1;`.
///
/// # Arguments
/// - `this`: node object, or null.
/// - `destroy`: when positive, free the node.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - destroys the element array and may free `this`.
///
/// # Safety
/// `this` must be a live node built by [`fn_802753f0`] when non-null.
///
/// # Confidence
/// high - the guard, the loop, the array destroy and the free branch are
/// explicit.
pub unsafe fn fn_802754b0(this: usize, destroy: i32) -> usize {
    if this != 0 {
        platform::write_u32(this, VT_RSAR_NODE);

        let mut slot = 0usize;
        while slot < 4 {
            platform::call2(FUN_80269AF0, (this + slot * 0x10 + 8) as u32, this as u32);
            slot += 1;
        }

        platform::call4(FUN_802DCA98, (this + 8) as u32, FUN_80269A50 as u32, 0x10, 4);

        if destroy > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 0x80275560
// ===========================================================================

/// Reads one word of the four-slot element array.
///
/// # Original
/// - Ghidra: `FUN_80275560` at `0x80275560` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275560_FUN_80275560.c`
/// - observed: `return *(undefined4 *)(param_1 + param_2 * 0x10 + 8);`
///
/// # Arguments
/// - `this`: node object.
/// - `index`: slot index (not bounds-checked by the original).
///
/// # Returns
/// The word at `this + index * 0x10 + 8`.
///
/// # Safety
/// The computed address must be readable.
///
/// # Confidence
/// high - one scaled load.
pub unsafe fn fn_80275560(this: usize, index: i32) -> u32 {
    platform::read_u32(this.wrapping_add((index as usize).wrapping_mul(0x10)) + 8)
}

// ===========================================================================
// 0x80275570
// ===========================================================================

/// Writes one slot of the four-slot element array through `0x80269D00`.
///
/// # Original
/// - Ghidra: `FUN_80275570` at `0x80275570` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275570_FUN_80275570.c`
/// - observed: `FUN_80269d00(param_1 + param_2 * 0x10 + 8,param_3);`
///
/// # Arguments
/// - `this`: node object.
/// - `index`: slot index (not bounds-checked by the original).
/// - `value`: value forwarded to `0x80269D00`.
///
/// # Side effects
/// - calls `0x80269D00` on the slot at `+8 + index * 0x10`.
///
/// # Safety
/// The computed slot must be a live element.
///
/// # Confidence
/// high - one scaled call.
pub unsafe fn fn_80275570(this: usize, index: i32, value: u32) {
    platform::call2(
        FUN_80269D00,
        this.wrapping_add((index as usize).wrapping_mul(0x10)).wrapping_add(8) as u32,
        value,
    );
}

// ===========================================================================
// 0x80275640
// ===========================================================================

/// Constructs the small `RSAR` track object.
///
/// Stores the vtable `0x8034F0D8`, clears `+0x04`, and sets the two bytes
/// `+0x08`/`+0x09` to `0x2F` (`'/'`) and `0`.
///
/// # Original
/// - Ghidra: `FUN_80275640` at `0x80275640` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275640_FUN_80275640.c`
/// - observed: `*param_1 = &DAT_8034f0d8; param_1[1] = 0;
///   *(undefined1 *)(param_1 + 2) = 0x2f; *(undefined1 *)((int)param_1 + 9) = 0;`
/// - `// inferred:` `0x2F` is the path separator written at the start of the
///   name buffer, matching [`fn_80275cc0`].
///
/// # Arguments
/// - `this`: track object.
///
/// # Side effects
/// - writes `+0x00..+0x09`.
///
/// # Safety
/// `this` must be writable for `0x0A` bytes.
///
/// # Confidence
/// high - every store is explicit.
pub unsafe fn fn_80275640(this: usize) {
    platform::write_u32(this, VT_RSAR_TRACK);
    platform::write_u32(this + 4, 0);
    platform::write_u8(this + 8, b'/');
    platform::write_u8(this + 9, 0);
}

// ===========================================================================
// 0x80275670
// ===========================================================================

/// Destroys the small `RSAR` track object when asked.
///
/// # Original
/// - Ghidra: `FUN_80275670` at `0x80275670` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275670_FUN_80275670.c`
/// - observed: `if ((param_1 != 0) && (0 < param_2)) { FUN_802b9280(); }
///   return param_1;`
/// - `// inferred:` Ghidra prints no argument for `FUN_802b9280`, but `r3`
///   still holds `param_1` at that point; the free helper is called with the
///   object, exactly as in [`fn_802754b0`].
///
/// # Arguments
/// - `this`: track object, or null.
/// - `destroy`: when positive, free the object.
///
/// # Returns
/// `this`, unchanged.
///
/// # Side effects
/// - may call `0x802B9280`.
///
/// # Safety
/// `this` must be a live object when non-null.
///
/// # Confidence
/// high - the guard and the tail call are explicit; only the implicit
/// argument of the free helper is inferred.
pub unsafe fn fn_80275670(this: usize, destroy: i32) -> usize {
    if this != 0 && destroy > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// 0x802756B0
// ===========================================================================

/// Reports whether the word at `+0x04` is non-zero.
///
/// The original computes `(-word | word) >> 31`, the branchless sign test for
/// `word != 0`.
///
/// # Original
/// - Ghidra: `FUN_802756b0` at `0x802756B0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802756b0_FUN_802756b0.c`
/// - observed: `return (-*(uint *)(param_1 + 4) | *(uint *)(param_1 + 4)) >> 0x1f;`
///
/// # Arguments
/// - `this`: object holding the state word at `+0x04`.
///
/// # Returns
/// `1` when the word is non-zero, `0` when it is zero.
///
/// # Safety
/// `this + 4` must be readable.
///
/// # Confidence
/// high - the negation/OR/shift idiom is the standard non-zero test.
pub unsafe fn fn_802756b0(this: usize) -> u32 {
    let word = platform::read_u32(this + 4);
    (word.wrapping_neg() | word) >> 31
}

// ===========================================================================
// 0x802756D0
// ===========================================================================

/// Stores the state word at `+0x04`.
///
/// # Original
/// - Ghidra: `FUN_802756d0` at `0x802756D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802756d0_FUN_802756d0.c`
/// - observed: `*(undefined4 *)(param_1 + 4) = param_2;`
///
/// # Arguments
/// - `this`: object.
/// - `value`: new state word.
///
/// # Side effects
/// - writes `+0x04`.
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - a single store.
pub unsafe fn fn_802756d0(this: usize, value: u32) {
    platform::write_u32(this + 4, value);
}

// ===========================================================================
// 0x802756E0
// ===========================================================================

/// Resets the small `RSAR` track object's state byte and name.
///
/// # Original
/// - Ghidra: `FUN_802756e0` at `0x802756E0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802756e0_FUN_802756e0.c`
/// - observed: `*(undefined4 *)(param_1 + 4) = 0;
///   *(undefined1 *)(param_1 + 8) = 0x2f; *(undefined1 *)(param_1 + 9) = 0;`
///
/// # Arguments
/// - `this`: track object.
///
/// # Side effects
/// - writes `+0x04..+0x09`.
///
/// # Safety
/// `this` must be writable for `0x0A` bytes.
///
/// # Confidence
/// high - every store is explicit.
pub unsafe fn fn_802756e0(this: usize) {
    platform::write_u32(this + 4, 0);
    platform::write_u8(this + 8, b'/');
    platform::write_u8(this + 9, 0);
}

// ===========================================================================
// RSAR accessor forwarders, 0x80275700..0x802757D0
// ===========================================================================
//
// The five wrappers at 0x80275700..0x80275740 and the info-header wrapper at
// 0x802757D0 are member functions that fetch the archive object or target
// object from `this + 4` and tail-forward it. They exist because the original
// class hierarchy exposes this one field through several interfaces.

// ===========================================================================
// 0x80275700
// ===========================================================================

/// Forwards `*(this + 4)` to `0x80276940`.
///
/// # Original
/// - Ghidra: `FUN_80275700` at `0x80275700` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275700_FUN_80275700.c`
/// - observed: `FUN_80276940(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the target at `+0x04`.
///
/// # Side effects
/// - calls `0x80276940` with the target.
///
/// # Safety
/// `this + 4` must hold a valid target object.
///
/// # Confidence
/// high - one load and one call.
pub unsafe fn fn_80275700(this: usize) {
    platform::call1(FUN_80276940, platform::read_u32(this + 4));
}

// ===========================================================================
// 0x80275710
// ===========================================================================

/// Forwards `*(this + 4)` to `0x80276980`.
///
/// # Original
/// - Ghidra: `FUN_80275710` at `0x80275710` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275710_FUN_80275710.c`
/// - observed: `FUN_80276980(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the target at `+0x04`.
///
/// # Side effects
/// - calls `0x80276980` with the target.
///
/// # Safety
/// `this + 4` must hold a valid target object.
///
/// # Confidence
/// high - one load and one call.
pub unsafe fn fn_80275710(this: usize) {
    platform::call1(FUN_80276980, platform::read_u32(this + 4));
}

// ===========================================================================
// 0x80275720
// ===========================================================================

/// Forwards `*(this + 4)` to `0x802769D0`.
///
/// # Original
/// - Ghidra: `FUN_80275720` at `0x80275720` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275720_FUN_80275720.c`
/// - observed: `FUN_802769d0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the target at `+0x04`.
///
/// # Side effects
/// - calls `0x802769D0` with the target.
///
/// # Safety
/// `this + 4` must hold a valid target object.
///
/// # Confidence
/// high - one load and one call.
pub unsafe fn fn_80275720(this: usize) {
    platform::call1(FUN_802769D0, platform::read_u32(this + 4));
}

// ===========================================================================
// 0x80275730
// ===========================================================================

/// Forwards the target, its `+0x34` word and `value` to `0x80276E00`.
///
/// # Original
/// - Ghidra: `FUN_80275730` at `0x80275730` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275730_FUN_80275730.c`
/// - observed: `FUN_80276e00(*(int *)(param_1 + 4),
///   *(undefined4 *)(*(int *)(param_1 + 4) + 0x34),param_2);`
///
/// # Arguments
/// - `this`: wrapper object holding the target at `+0x04`.
/// - `value`: value forwarded as the third argument.
///
/// # Side effects
/// - calls `0x80276E00`.
///
/// # Safety
/// `this + 4` must hold a valid target object readable at `+0x34`.
///
/// # Confidence
/// high - the two loads and the call are explicit.
pub unsafe fn fn_80275730(this: usize, value: u32) {
    let target = platform::read_u32(this + 4) as usize;
    platform::call3(FUN_80276E00, target as u32, platform::read_u32(target + 0x34), value);
}

// ===========================================================================
// 0x80275740
// ===========================================================================

/// Forwards the target, its `+0x38` word and `value` to `0x80276E00`.
///
/// # Original
/// - Ghidra: `FUN_80275740` at `0x80275740` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275740_FUN_80275740.c`
/// - observed: `FUN_80276e00(*(int *)(param_1 + 4),
///   *(undefined4 *)(*(int *)(param_1 + 4) + 0x38),param_2);`
///
/// # Arguments
/// - `this`: wrapper object holding the target at `+0x04`.
/// - `value`: value forwarded as the third argument.
///
/// # Side effects
/// - calls `0x80276E00`.
///
/// # Safety
/// `this + 4` must hold a valid target object readable at `+0x38`.
///
/// # Confidence
/// high - the two loads and the call are explicit.
pub unsafe fn fn_80275740(this: usize, value: u32) {
    let target = platform::read_u32(this + 4) as usize;
    platform::call3(FUN_80276E00, target as u32, platform::read_u32(target + 0x38), value);
}

// ===========================================================================
// 0x802757D0
// ===========================================================================

/// Reads the seven-word info header of the archive stored at `this + 4`.
///
/// Ghidra prints this body as `void FUN_802757d0(int)` that drops both the
/// output pointer and the return value of `0x802768B0`, but all five call
/// sites in the image (`0x80275150`, `0x80277AB0`, `0x80277CF0`,
/// `0x80277E40`, `0x80277E90`) pass a second argument and branch on the
/// result. The translation therefore forwards `out` and returns the helper's
/// result.
///
/// # Original
/// - Ghidra: `FUN_802757d0` at `0x802757D0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802757d0_FUN_802757d0.c`
/// - observed: `FUN_802768b0(*(undefined4 *)(param_1 + 4));`
/// - `// inferred:` the out pointer and the `bool` result come from the
///   callers listed above and from `FUN_802768b0(int, uint *)`'s own export.
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `out`: address of seven writable words.
///
/// # Returns
/// `1` when the info node was found, `0` otherwise.
///
/// # Side effects
/// - fills `out` with the info header when the lookup succeeds.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for 28
/// bytes.
///
/// # Confidence
/// medium - the forwarded call is literal, but the argument and return are
/// recovered from call sites because the decompiler dropped them.
pub unsafe fn fn_802757d0(this: usize, out: usize) -> u32 {
    platform::call2(FUN_802768B0, platform::read_u32(this + 4), out as u32)
}

// ===========================================================================
// RSAR entry lookup and header decoding, 0x80275A80..0x80275FD0
// ===========================================================================

/// Resolves one archive id to its six-word info record and invokes the loader.
///
/// `this` is the archive object and `id` the id to resolve. The id is resolved
/// through `0x80276660`; when the returned name
/// pointer (word 1) is null the routine calls the single-file loader
/// `vtable[0x18]` with words 2/3 of the record. When the name does not start
/// with `/`, the archive prefix at `archive + 8` is copied into a 256-byte
/// stack buffer and the name appended; a combined length above `0xFF` aborts
/// with result `0`. The multi-file loader `vtable[0x1C]` then receives the
/// path plus words 2/3. The loader result is returned.
///
/// # Original
/// - Ghidra: `FUN_80275a80` at `0x80275A80` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275a80_FUN_80275a80.c`
/// - Removed: the modelled `_savegpr_25` prologue call (`0x802DCD6C`) and the
///   three `_restgpr_25` epilogue calls (`0x802DCDB8`); `this`/`id` are the
///   incoming `r3`/`r4` and each exit returns its value directly.
/// - observed: `FUN_80276660(piVar1[1],(int)uVar5,auStack_138)`, the
///   `local_134 == 0` branch calling `(**(code **)(*piVar1 + 0x18))`, the
///   `*local_134 != '/'` branch with the two `FUN_802dc69c` lengths, the
///   `0xff < (uint)(iVar2 + iVar4)` guard, the two copies into `acStack_120`,
///   and the `+ 0x1c` vtable call.
///
/// # Arguments
/// - `this`: archive object (incoming `r3`).
/// - `id`: archive id to resolve (incoming `r4`).
/// - `param_3`, `param_4`: forwarded to the loader vtable slot.
///
/// # Side effects
/// - resolves the id, may copy into a stack buffer, calls one of two vtable
///   loader slots and returns the loader result.
///
/// # Safety
/// `this` must be a live archive with a valid `+4` info node and a loader
/// vtable.
///
/// # Confidence
/// medium - the control flow is explicit; the roles of the two loader
/// arguments are inferred.
pub unsafe fn fn_80275a80(this: usize, id: u32, param_3: u32, param_4: u32) -> u32 {
    let mut info = [0u32; 6];
    let found = platform::call3(
        FUN_80276660,
        platform::read_u32(this + 4),
        id,
        info.as_mut_ptr() as u32,
    );
    if found == 0 {
        return 0;
    }

    let result;
    if info[1] == 0 {
        let vtable = platform::read_u32(this) as usize;
        let load = platform::read_u32(vtable + 0x18) as usize;
        result = platform::call5(load, this as u32, param_3, param_4, info[2], info[3]);
    } else {
        let name = info[1] as usize;
        let mut path = [0u8; 256];
        let name_ptr = if platform::read_u8(name) != b'/' {
            let name_len = platform::call1(STRLEN, name as u32) as i32;
            let prefix_len = platform::call1(STRLEN, (this + 8) as u32) as i32;
            if name_len.wrapping_add(prefix_len) as u32 > 0xff {
                return 0;
            }
            platform::call3(
                FUN_802E19F8,
                path.as_mut_ptr() as u32,
                (this + 8) as u32,
                (prefix_len + 1) as u32,
            );
            platform::call3(
                FUN_802E1A68,
                path.as_mut_ptr() as u32,
                name as u32,
                (name_len + 1) as u32,
            );
            path.as_ptr() as usize
        } else {
            name
        };

        let vtable = platform::read_u32(this) as usize;
        let load = platform::read_u32(vtable + 0x1c) as usize;
        result = platform::call6(
            load,
            this as u32,
            param_3,
            param_4,
            name_ptr as u32,
            info[2],
            info[3],
        );
    }

    result
}

// ===========================================================================
// 0x80275BA0
// ===========================================================================

/// Second id-to-info lookup variant; loads words 4/5 of the info record.
///
/// The body is identical to [`fn_80275a80`] except that the two loader
/// arguments are words 4 and 5 of the six-word record instead of words 2 and
/// 3. Ghidra's stack slots (`local_128`/`local_124`) sit at record offsets
/// `+0x10`/`+0x14`, which is why the different words are used.
///
/// # Original
/// - Ghidra: `FUN_80275ba0` at `0x80275BA0` (280 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275ba0_FUN_80275ba0.c`
/// - Removed: the modelled `_savegpr_25` prologue call (`0x802DCD6C`) and the
///   three `_restgpr_25` epilogue calls (`0x802DCDB8`); `this`/`id` are the
///   incoming `r3`/`r4` and each exit returns its value directly.
/// - observed: `FUN_80276660(piVar1[1],(int)uVar5,auStack_138)`, the
///   `local_134` name test, the `(**(code **)(*piVar1 + 0x18))` call with
///   `local_128,local_124`, the prefix-building branch and the `+ 0x1c` call.
/// - `// inferred:` the different stack slots are record words 4/5; the shared
///   helper `0x80276660` writes all six words.
///
/// # Arguments
/// - `this`: archive object (incoming `r3`).
/// - `id`: archive id to resolve (incoming `r4`).
/// - `param_3`, `param_4`: forwarded to the loader vtable slot.
///
/// # Side effects
/// - resolves the id, may copy into a stack buffer, calls one of two vtable
///   loader slots and returns the loader result.
///
/// # Safety
/// `this` must be a live archive with a valid `+4` info node and a loader
/// vtable.
///
/// # Confidence
/// medium - the control flow is explicit; the record-word choice is read from
/// Ghidra's stack slots.
pub unsafe fn fn_80275ba0(this: usize, id: u32, param_3: u32, param_4: u32) -> u32 {
    let mut info = [0u32; 6];
    let found = platform::call3(
        FUN_80276660,
        platform::read_u32(this + 4),
        id,
        info.as_mut_ptr() as u32,
    );
    if found == 0 {
        return 0;
    }

    let result;
    if info[1] == 0 {
        let vtable = platform::read_u32(this) as usize;
        let load = platform::read_u32(vtable + 0x18) as usize;
        result = platform::call5(load, this as u32, param_3, param_4, info[4], info[5]);
    } else {
        let name = info[1] as usize;
        let mut path = [0u8; 256];
        let name_ptr = if platform::read_u8(name) != b'/' {
            let name_len = platform::call1(STRLEN, name as u32) as i32;
            let prefix_len = platform::call1(STRLEN, (this + 8) as u32) as i32;
            if name_len.wrapping_add(prefix_len) as u32 > 0xff {
                return 0;
            }
            platform::call3(
                FUN_802E19F8,
                path.as_mut_ptr() as u32,
                (this + 8) as u32,
                (prefix_len + 1) as u32,
            );
            platform::call3(
                FUN_802E1A68,
                path.as_mut_ptr() as u32,
                name as u32,
                (name_len + 1) as u32,
            );
            path.as_ptr() as usize
        } else {
            name
        };

        let vtable = platform::read_u32(this) as usize;
        let load = platform::read_u32(vtable + 0x1c) as usize;
        result = platform::call6(
            load,
            this as u32,
            param_3,
            param_4,
            name_ptr as u32,
            info[4],
            info[5],
        );
    }

    result
}

// ===========================================================================
// 0x80275CC0
// ===========================================================================

/// Copies a name into the object's buffer and appends a `/` when missing.
///
/// The destination is `this + 8`. The copy length is `strlen(name)`; when the
/// last source byte is not `/`, a separator is written at `dest[len]` and the
/// terminator moves one byte further. The bytes themselves are copied with
/// `0x802E19F8` after the terminator has been placed.
///
/// # Original
/// - Ghidra: `FUN_80275cc0` at `0x80275CC0` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275cc0_FUN_80275cc0.c`
/// - observed: `iVar1 = FUN_802dc69c(param_2);`, the
///   `*(char *)(param_2 + iVar1 + -1) != '/'` test, the two
///   `*(undefined1 *)(param_1 + ... + 8)` stores, and
///   `FUN_802e19f8(param_1 + 8,param_2,iVar1)`.
/// - `// inferred:` `0x802E19F8` is a bounded copy: the terminator is written
///   by this function and the copy length is exactly `strlen`.
///
/// # Arguments
/// - `this`: track object whose buffer starts at `+0x08`.
/// - `name`: address of the NUL-terminated source path.
///
/// # Side effects
/// - writes the buffer at `+0x08`.
///
/// # Safety
/// `name` must be a readable C string of at least one byte; the buffer at
/// `this + 8` must be large enough for `strlen(name) + 2` bytes.
///
/// # Confidence
/// high - the length, the separator test and both stores are explicit.
pub unsafe fn fn_80275cc0(this: usize, name: u32) {
    let len = platform::call1(STRLEN, name) as i32;
    let mut total = len;

    let last = (name as i32).wrapping_add(len).wrapping_sub(1) as usize;
    if platform::read_u8(last) != b'/' {
        platform::write_u8(this.wrapping_add(len as usize).wrapping_add(8), b'/');
        total = len + 1;
    }
    platform::write_u8(this.wrapping_add(total as usize).wrapping_add(8), 0);

    platform::call3(FUN_802E19F8, (this + 8) as u32, name, len as u32);
}

// ===========================================================================
// 0x80275D40
// ===========================================================================

/// Clears the seven pointer words at `+0x28..+0x40`.
///
/// # Original
/// - Ghidra: `FUN_80275d40` at `0x80275D40` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275d40_FUN_80275d40.c`
/// - observed: `*(undefined4 *)(param_1 + 0x28) = 0;` through
///   `*(undefined4 *)(param_1 + 0x40) = 0;` - seven consecutive stores.
///
/// # Arguments
/// - `this`: archive/header object.
///
/// # Side effects
/// - clears `+0x28..+0x43`.
///
/// # Safety
/// `this` must be writable through `+0x43`.
///
/// # Confidence
/// high - seven explicit zero stores.
pub unsafe fn fn_80275d40(this: usize) {
    let mut offset = 0x28;
    while offset <= 0x40 {
        platform::write_u32(this + offset, 0);
        offset += 4;
    }
}

// ===========================================================================
// 0x80275D70
// ===========================================================================

/// Copies the ten-word `RSAR` file header when the magic and version match.
///
/// The magic word must be `0x52534152` (`"RSAR"`, read big-endian) and the
/// `u16` at `+0x06` must be at least `0x100`. If both checks pass the ten
/// header words `+0x00..+0x27` are copied from `src` to `dst`. The version test
/// is the same branchless clamp idiom as [`fn_80270d20`], reproduced as the
/// equivalent comparison.
///
/// # Original
/// - Ghidra: `FUN_80275d70` at `0x80275D70` (176 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275d70_FUN_80275d70.c`
/// - observed: `if (*param_2 == 0x52534152)`, the `uVar1 = (uint)*(ushort *)
///   ((int)param_2 + 6)` / `if (uVar1 < 0x100)` clamp, then the
///   `param_2[1..9]` loads and the `param_1[1..9]` stores.
///
/// # Arguments
/// - `dst`: ten-word destination header.
/// - `src`: candidate `RSAR` header.
///
/// # Returns
/// Nothing; `dst` is left untouched when the header is rejected.
///
/// # Side effects
/// - writes 40 bytes at `dst` on success.
///
/// # Safety
/// `src` must be readable for `0x28` bytes; `dst` must be writable for 40
/// bytes.
///
/// # Confidence
/// high - the magic, the version clamp and the ten-word copy are explicit.
pub unsafe fn fn_80275d70(dst: usize, src: usize) {
    if platform::read_u32(src) != RSAR_MAGIC {
        return;
    }
    let version = platform::read_u16(src + 6) as u32;
    if version < 0x100 {
        return;
    }

    let mut word = 0usize;
    while word < 10 {
        platform::write_u32(dst + word * 4, platform::read_u32(src + word * 4));
        word += 1;
    }
}

// ===========================================================================
// 0x80275E20
// ===========================================================================

/// Turns the five header offsets into absolute pointers.
///
/// The base is `header + 8`, stored at `+0x2C`. The word at `+0x30` is
/// `*(header + 8)` added to that base (or zero when the offset is zero); the
/// four words at `+0x34..+0x40` are `*(header + 0x0C/+0x10/+0x14/+0x18)` added
/// to the same base, again zero when the offset is zero.
///
/// # Original
/// - Ghidra: `FUN_80275e20` at `0x80275E20` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275e20_FUN_80275e20.c`
/// - observed: `*(int *)(param_1 + 0x2c) = param_2 + 8;`, the five
///   `if (*(int *)(param_2 + N) == 0) iVar1 = 0; else iVar1 = ... + param_2 + 8`
///   blocks for `N = 8, 0xC, 0x10, 0x14, 0x18`, and the stores to
///   `+0x30..+0x40`.
///
/// # Arguments
/// - `this`: header object receiving the pointers.
/// - `header`: `RSAR` header whose offsets are relative to `header + 8`.
///
/// # Side effects
/// - writes `+0x2C..+0x40`.
///
/// # Safety
/// `header` must be readable through `+0x1C`; `this` writable through
/// `+0x43`.
///
/// # Confidence
/// high - all five offset blocks are explicit.
pub unsafe fn fn_80275e20(this: usize, header: usize) {
    let base = header.wrapping_add(8);
    platform::write_u32(this + 0x2c, base as u32);

    let first = platform::read_u32(header + 8);
    let first_ptr = if first == 0 {
        0
    } else {
        first.wrapping_add(base as u32)
    };
    platform::write_u32(this + 0x30, first_ptr);

    let mut field = 0usize;
    while field < 4 {
        let offset = platform::read_u32(header + 0x0c + field * 4);
        let pointer = if offset == 0 {
            0
        } else {
            offset.wrapping_add(base as u32)
        };
        platform::write_u32(this + 0x34 + field * 4, pointer);
        field += 1;
    }
}

// ===========================================================================
// 0x80275ED0
// ===========================================================================

/// Stores `header + 8` as the object's archive base pointer.
///
/// # Original
/// - Ghidra: `FUN_80275ed0` at `0x80275ED0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275ed0_FUN_80275ed0.c`
/// - observed: `*(int *)(param_1 + 0x28) = param_2 + 8;`
///
/// # Arguments
/// - `this`: archive object.
/// - `header`: header base.
///
/// # Side effects
/// - writes `+0x28`.
///
/// # Safety
/// `this + 0x28` must be writable.
///
/// # Confidence
/// high - a single computed store.
pub unsafe fn fn_80275ed0(this: usize, header: usize) {
    platform::write_u32(this + 0x28, header.wrapping_add(8) as u32);
}

// ===========================================================================
// 0x80275EE0
// ===========================================================================

/// Returns the entry type (`1`..`3`) of one archive id, or `0`.
///
/// The archive base is `*(this + 0x28)`. The id is bounds-checked against the
/// first word of the table resolved by `0x80283A60`; for version `< 0x101` the
/// type byte is read from `table + id * 8 + 5`, otherwise the entry is
/// resolved through a second `0x80283A60` call and the type byte read at
/// `entry + 0x16`. Only the values `1`, `2` and `3` are returned; anything
/// else becomes `0`.
///
/// # Original
/// - Ghidra: `FUN_80275ee0` at `0x80275EE0` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275ee0_FUN_80275ee0.c`
/// - observed: `puVar2 = (uint *)FUN_80283a60(**(undefined1 **)(param_1 +
///   0x28), *(undefined4 *)(*(undefined1 **)(param_1 + 0x28) + 4));`, the
///   `param_2 < *puVar2` bound, the `*(ushort *)(param_1 + 6) < 0x101`
///   branch, `*(char *)((int)puVar2 + param_2 * 8 + 5)`, the nested
///   `FUN_80283a60(..., ..., *(undefined4 *)(param_1 + 0x28))` call and the
///   `0x16` byte, then the `1`/`2`/`3` chain.
/// - `// inferred:` the third argument of the first `0x80283A60` call is the
///   same base word; Ghidra omits it because the register was already live.
///
/// # Arguments
/// - `this`: archive object (`+0x28` holds the base).
/// - `index`: entry id.
///
/// # Returns
/// `1`, `2` or `3` for the known entry kinds, `0` when absent or unknown.
///
/// # Safety
/// The base word and the tables it names must be readable.
///
/// # Confidence
/// medium - the bounds test and both type-byte paths are explicit; the
/// resolver's exact table layout is inferred from the offsets.
pub unsafe fn fn_80275ee0(this: usize, index: u32) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base) as u32,
        platform::read_u32(base + 4),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if index >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + index as usize * 8;
    let kind = if (platform::read_u16(this + 6) as u32) < 0x101 {
        platform::read_u8(slot + 5)
    } else {
        let entry = platform::call3(
            FUN_80283A60,
            platform::read_u8(slot + 4) as u32,
            platform::read_u32(slot + 8),
            base as u32,
        ) as usize;
        if entry == 0 {
            return 0;
        }
        platform::read_u8(entry + 0x16)
    };

    match kind {
        1 => 1,
        2 => 2,
        3 => 3,
        _ => 0,
    }
}

// ===========================================================================
// 0x80275FD0
// ===========================================================================

/// Fills the eight-word info block of one archive id.
///
/// Resolves the id exactly like [`fn_80275ee0`]. For version `< 0x101` the
/// entry pointer is `*(table + id * 8 + 8) + base`; otherwise a second
/// `0x80283A60` call resolves it. The block receives `*(entry + 4)`,
/// `*(entry + 8)`, the bytes at `+0x2A`, `+0x15`, `+0x14` and `+0x17`, then
/// either the constants `1`/`0` (version `< 0x102`) or the bytes at
/// `+0x28`/`+0x29`.
///
/// # Original
/// - Ghidra: `FUN_80275fd0` at `0x80275FD0` (296 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275fd0_FUN_80275fd0.c`
/// - observed: the `puVar5 = *(undefined1 **)(param_1 + 0x28)` lookup, the
///   `param_2 < *puVar2` bound, the `*(ushort *)(param_1 + 6) < 0x101` branch
///   with `puVar2[param_2 * 2 + 2] + *(int *)(param_1 + 0x28)`, the eight
///   `param_3[N]` stores, and `uVar4 = 1;`.
///
/// # Arguments
/// - `this`: archive object.
/// - `index`: entry id.
/// - `out`: address of eight writable words.
///
/// # Returns
/// `1` when the entry was found, `0` otherwise.
///
/// # Side effects
/// - writes the eight-word block at `out` on success.
///
/// # Safety
/// The archive tables must be readable; `out` must be writable for 32 bytes.
///
/// # Confidence
/// medium - every field store is explicit; the table layout behind
/// `0x80283A60` is inferred from offsets.
pub unsafe fn fn_80275fd0(this: usize, index: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base) as u32,
        platform::read_u32(base + 4),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if index >= platform::read_u32(table) {
        return 0;
    }

    let version = platform::read_u16(this + 6) as u32;
    let slot = table + index as usize * 8;
    let entry = if version < 0x101 {
        platform::read_u32(slot + 8).wrapping_add(base as u32) as usize
    } else {
        platform::call3(
            FUN_80283A60,
            platform::read_u8(slot + 4) as u32,
            platform::read_u32(slot + 8),
            base as u32,
        ) as usize
    };
    if entry == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(entry + 4));
    platform::write_u32(out + 4, platform::read_u32(entry + 8));
    platform::write_u32(out + 8, platform::read_u8(entry + 0x2a) as u32);
    platform::write_u32(out + 0x0c, platform::read_u8(entry + 0x15) as u32);
    platform::write_u32(out + 0x10, platform::read_u8(entry + 0x14) as u32);
    platform::write_u32(out + 0x14, platform::read_u8(entry + 0x17) as u32);

    if version < 0x102 {
        platform::write_u32(out + 0x18, 1);
        platform::write_u32(out + 0x1c, 0);
    } else {
        platform::write_u32(out + 0x18, platform::read_u8(entry + 0x28) as u32);
        platform::write_u32(out + 0x1c, platform::read_u8(entry + 0x29) as u32);
    }

    1
}

// ===========================================================================
// RSAR vtable forwarders (wave 52), 0x80275750..0x80275840
// ===========================================================================
//
// Each entry loads the archive object at wrapper `+0x04` into `r3` and
// tail-branches to one resolver. Because the branch does not touch the
// argument registers, `r4`..`r7` reach the callee unchanged; the wrapper
// signatures therefore mirror the callee signatures. Ghidra prints only the
// first argument because the tail branch hides the rest of the register set.

// ===========================================================================
// 0x80275750
// ===========================================================================

/// Forwards the archive target, its `+0x3C` table and `name` to `0x80276E00`.
///
/// # Original
/// - Ghidra: `FUN_80275750` at `0x80275750` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275750_FUN_80275750.c`
/// - observed: `FUN_80276e00(*(int *)(param_1 + 4),
///   *(undefined4 *)(*(int *)(param_1 + 4) + 0x3c),param_2);`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `name`: NUL-terminated name forwarded as the third argument.
///
/// # Returns
/// The callee's result word.
///
/// # Side effects
/// - calls `0x80276E00`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `name` must point to a readable
/// NUL-terminated string.
///
/// # Confidence
/// medium - the load and the call are literal; the extra argument is the
/// callee's third parameter, recovered from its own export.
pub unsafe fn fn_80275750(this: usize, name: usize) -> u32 {
    let target = platform::read_u32(this + 4) as usize;
    platform::call3(
        FUN_80276E00,
        target as u32,
        platform::read_u32(target + 0x3c),
        name as u32,
    )
}

// ===========================================================================
// 0x80275760
// ===========================================================================

/// Forwards the archive target and `id` to `0x80276AE0`.
///
/// # Original
/// - Ghidra: `FUN_80275760` at `0x80275760` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275760_FUN_80275760.c`
/// - observed: `FUN_80276ae0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
///
/// # Returns
/// The entry's `+0x20` word, or `0`.
///
/// # Side effects
/// - calls `0x80276AE0`.
///
/// # Safety
/// `this + 4` must hold a live archive object.
///
/// # Confidence
/// medium - the load and tail call are literal; `id` is inferred from the
/// callee's two-argument signature.
pub unsafe fn fn_80275760(this: usize, id: u32) -> u32 {
    platform::call2(FUN_80276AE0, platform::read_u32(this + 4), id)
}

// ===========================================================================
// 0x80275770
// ===========================================================================

/// Forwards the archive target and `id` to `0x80275EE0`.
///
/// # Original
/// - Ghidra: `FUN_80275770` at `0x80275770` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275770_FUN_80275770.c`
/// - observed: `FUN_80275ee0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
///
/// # Returns
/// The entry kind (`1`..`3`), or `0`.
///
/// # Side effects
/// - calls `0x80275EE0`.
///
/// # Safety
/// `this + 4` must hold a live archive object.
///
/// # Confidence
/// medium - the load and tail call are literal; `id` is inferred from the
/// callee's two-argument signature.
pub unsafe fn fn_80275770(this: usize, id: u32) -> u32 {
    platform::call2(FUN_80275EE0, platform::read_u32(this + 4), id)
}

// ===========================================================================
// 0x80275780
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80275FD0`.
///
/// # Original
/// - Ghidra: `FUN_80275780` at `0x80275780` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275780_FUN_80275780.c`
/// - observed: `FUN_80275fd0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: eight-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when the entry was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80275FD0`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 32 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_80275780(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80275FD0, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x80275790
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80276210`.
///
/// # Original
/// - Ghidra: `FUN_80275790` at `0x80275790` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275790_FUN_80275790.c`
/// - observed: `FUN_80276210(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: four-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when a type-1 record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276210`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 20 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_80275790(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80276210, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x802757A0
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80276300`.
///
/// # Original
/// - Ghidra: `FUN_802757a0` at `0x802757A0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802757a0_FUN_802757a0.c`
/// - observed: `FUN_80276300(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: three-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when a type-2 record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276300`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 16 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_802757a0(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80276300, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x802757B0
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80276410`.
///
/// # Original
/// - Ghidra: `FUN_802757b0` at `0x802757B0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802757b0_FUN_802757b0.c`
/// - observed: `FUN_80276410(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: three-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when a type-3 record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276410`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 12 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_802757b0(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80276410, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x802757C0
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x802765A0`.
///
/// # Original
/// - Ghidra: `FUN_802757c0` at `0x802757C0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802757c0_FUN_802757c0.c`
/// - observed: `FUN_802765a0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: two-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when the entry was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x802765A0`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 8 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_802757c0(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_802765A0, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x802757E0
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80276100`.
///
/// # Original
/// - Ghidra: `FUN_802757e0` at `0x802757E0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802757e0_FUN_802757e0.c`
/// - observed: `FUN_80276100(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: seven-byte output record forwarded as the third argument.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276100`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 8 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_802757e0(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80276100, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x802757F0
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x802764F0`.
///
/// # Original
/// - Ghidra: `FUN_802757f0` at `0x802757F0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802757f0_FUN_802757f0.c`
/// - observed: `FUN_802764f0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: one-word output slot forwarded as the third argument.
///
/// # Returns
/// `1` when the entry was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x802764F0`, which writes `out[0]`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 4 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_802757f0(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_802764F0, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x80275800
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80276660`.
///
/// # Original
/// - Ghidra: `FUN_80275800` at `0x80275800` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275800_FUN_80275800.c`
/// - observed: `FUN_80276660(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: six-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276660`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 24 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_80275800(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80276660, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x80275810
// ===========================================================================

/// Forwards the archive target, two ids and `out` to `0x80276770`.
///
/// # Original
/// - Ghidra: `FUN_80275810` at `0x80275810` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275810_FUN_80275810.c`
/// - observed: `FUN_80276770(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: first entry id.
/// - `sub_id`: second-level id.
/// - `out`: five-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276770`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 20 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; the three forwarded arguments
/// come from the callee's four-argument signature.
pub unsafe fn fn_80275810(this: usize, id: u32, sub_id: u32, out: usize) -> u32 {
    platform::call4(
        FUN_80276770,
        platform::read_u32(this + 4),
        id,
        sub_id,
        out as u32,
    )
}

// ===========================================================================
// 0x80275820
// ===========================================================================

/// Forwards the archive target to `0x80276BA0`.
///
/// # Original
/// - Ghidra: `FUN_80275820` at `0x80275820` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275820_FUN_80275820.c`
/// - observed: `FUN_80276ba0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
///
/// # Returns
/// The first word of the `+0x18` table, or `0`.
///
/// # Side effects
/// - calls `0x80276BA0`.
///
/// # Safety
/// `this + 4` must hold a live archive object.
///
/// # Confidence
/// high - the load and the one-argument tail call are literal.
pub unsafe fn fn_80275820(this: usize) -> u32 {
    platform::call1(FUN_80276BA0, platform::read_u32(this + 4))
}

// ===========================================================================
// 0x80275830
// ===========================================================================

/// Forwards the archive target, `id` and `out` to `0x80276BE0`.
///
/// # Original
/// - Ghidra: `FUN_80275830` at `0x80275830` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275830_FUN_80275830.c`
/// - observed: `FUN_80276be0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: entry id forwarded as the second argument.
/// - `out`: four-word output block forwarded as the third argument.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276BE0`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 16 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; `id`/`out` are inferred from
/// the callee's three-argument signature.
pub unsafe fn fn_80275830(this: usize, id: u32, out: usize) -> u32 {
    platform::call3(FUN_80276BE0, platform::read_u32(this + 4), id, out as u32)
}

// ===========================================================================
// 0x80275840
// ===========================================================================

/// Forwards the archive target, two ids and `out` to `0x80276CE0`.
///
/// # Original
/// - Ghidra: `FUN_80275840` at `0x80275840` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275840_FUN_80275840.c`
/// - observed: `FUN_80276ce0(*(undefined4 *)(param_1 + 4));`
///
/// # Arguments
/// - `this`: wrapper object holding the archive at `+0x04`.
/// - `id`: first entry id.
/// - `sub_id`: second-level id.
/// - `out`: two-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - calls `0x80276CE0`, which fills `out`.
///
/// # Safety
/// `this + 4` must hold a live archive object; `out` must be writable for
/// 8 bytes.
///
/// # Confidence
/// medium - the load and tail call are literal; the three forwarded arguments
/// come from the callee's four-argument signature.
pub unsafe fn fn_80275840(this: usize, id: u32, sub_id: u32, out: usize) -> u32 {
    platform::call4(
        FUN_80276CE0,
        platform::read_u32(this + 4),
        id,
        sub_id,
        out as u32,
    )
}

// ===========================================================================
// RSAR table resolvers (wave 52), 0x80276100..0x80276BE0
// ===========================================================================
//
// The archive object at `this + 0x28` is `RSAR`'s info block. Each resolver
// reads a byte/offset pair from that block, asks `0x80283A60` to turn the
// pair into a table pointer, bounds-checks the caller's id against the table's
// first word, and then decodes the entry. Versions before `0x101` name entries
// by a base-relative offset; newer versions use the byte/offset pair again.

// ===========================================================================
// 0x80276100
// ===========================================================================

/// Resolves one archive id to a seven-byte record.
///
/// The entry table is named by the byte and offset at info `+0x00`/`+0x04`.
/// For version `< 0x101` the entry is the base-relative offset at
/// `table + id * 8 + 8`; newer archives resolve the same slot through
/// `0x80283A60`. The entry is then treated as another byte/offset pair that
/// names the record, and the record's first word plus bytes `4`, `5` and `6`
/// are copied to `out`.
///
/// # Original
/// - Ghidra: `FUN_80276100` at `0x80276100` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276100_FUN_80276100.c`
/// - observed: the `*puVar5` / `puVar5 + 4` lookup, the `param_2 < *puVar1`
///   bound, the `*(ushort *)(param_1 + 6) < 0x101` branch, the
///   `puVar1[param_2 * 2 + 2] + *(int *)(param_1 + 0x28)` sum, the nested
///   `FUN_80283a60` call and the four output stores.
///
/// # Arguments
/// - `this`: archive object (`+0x28` is the info base).
/// - `id`: entry id.
/// - `out`: seven-byte output record.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - writes `out[0]` and bytes `out[4..6]` on success.
///
/// # Safety
/// The tables named by the info block must be readable; `out` must be writable
/// for 7 bytes.
///
/// # Confidence
/// medium - every load, branch and store is literal; the record's field roles
/// are inferred from how the load-by-id paths consume it.
pub unsafe fn fn_80276100(this: usize, id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base) as u32,
        platform::read_u32(base + 4),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let version = platform::read_u16(this + 6) as u32;
    let entry = if version < 0x101 {
        platform::read_u32(slot + 8).wrapping_add(base as u32) as usize
    } else {
        platform::call3(
            FUN_80283A60,
            platform::read_u8(slot + 4) as u32,
            platform::read_u32(slot + 8),
            base as u32,
        ) as usize
    };
    if entry == 0 {
        return 0;
    }

    let record = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 0x0c) as u32,
        platform::read_u32(entry + 0x10),
        base as u32,
    ) as usize;
    if record == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(record));
    platform::write_u8(out + 4, platform::read_u8(record + 4));
    platform::write_u8(out + 5, platform::read_u8(record + 5));
    platform::write_u8(out + 6, platform::read_u8(record + 6));
    1
}

// ===========================================================================
// 0x80276210
// ===========================================================================

/// Resolves a type-1 archive id to a four-word record.
///
/// The id kind must be `1`. `0x80276F50` turns the id into an eight-byte
/// descriptor; its first byte and `+0x04` word name the record through
/// `0x80283A60`. The record supplies words `0`, `1` and `2`, the byte at
/// `+0x0C` as word `3`, and one version-gated byte at `+0x10`: zero before
/// version `0x103`, otherwise `1` when record byte `+0x0D` is non-zero.
///
/// # Original
/// - Ghidra: `FUN_80276210` at `0x80276210` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276210_FUN_80276210.c`
/// - observed: the `FUN_80275ee0() == 1` gate, the `FUN_80276f50` descriptor
///   call, the two `FUN_80283a60` lookups, the four `param_3` stores and the
///   `*(ushort *)(param_1 + 6) < 0x103` branch.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: four-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - writes four words and one byte at `out`.
///
/// # Safety
/// The info tables and descriptor must be readable; `out` must be writable for
/// 20 bytes.
///
/// # Confidence
/// medium - the gate, descriptor call and stores are literal; the forward
/// arguments to `0x80275EE0` and `0x80276F50` are inferred from their exports,
/// because Ghidra drops the register-passed tail arguments.
pub unsafe fn fn_80276210(this: usize, id: u32, out: usize) -> u32 {
    if fn_80275ee0(this, id) != 1 {
        return 0;
    }

    let mut descriptor = [0u8; 8];
    let descriptor_addr = descriptor.as_mut_ptr() as usize;
    let found = platform::call3(FUN_80276F50, this as u32, id, descriptor_addr as u32);
    if found == 0 {
        return 0;
    }

    let base = platform::read_u32(this + 0x28) as usize;
    let record = platform::call3(
        FUN_80283A60,
        descriptor[0] as u32,
        platform::read_u32(descriptor_addr + 4),
        base as u32,
    ) as usize;
    if record == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(record));
    platform::write_u32(out + 4, platform::read_u32(record + 4));
    platform::write_u32(out + 0x0c, platform::read_u8(record + 0x0c) as u32);
    platform::write_u32(out + 8, platform::read_u32(record + 8));

    if (platform::read_u16(this + 6) as u32) < 0x103 {
        platform::write_u8(out + 0x10, 0);
    } else {
        platform::write_u8(out + 0x10, u8::from(platform::read_u8(record + 0x0d) != 0));
    }
    1
}

// ===========================================================================
// 0x80276300
// ===========================================================================

/// Resolves a type-2 archive id to a three-word record.
///
/// The id kind must be `2`. After the `0x80276F50` descriptor names the record,
/// word `0` and the `u16` at `+0x06` are copied. For version `< 0x104` the
/// `u16` at record `+0x04` is a run of low `1` bits: the routine stores the run
/// length at `out + 4` and fails if the first zero bit is reached before the
/// value is exhausted. Version `0x104` and later copy the `u16` unchanged.
///
/// # Original
/// - Ghidra: `FUN_80276300` at `0x80276300` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276300_FUN_80276300.c`
/// - observed: the `FUN_80275ee0() == 2` gate, the `FUN_80276f50` call, the
///   `*(undefined2 *)((int)param_3 + 6)` store, the `uVar4 & 1` bit loop and
///   the `*(ushort *)(param_1 + 6) < 0x104` branch.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: three-word output block.
///
/// # Returns
/// `1` when the record was found and the bit run is contiguous, `0` otherwise.
///
/// # Side effects
/// - writes `out[0]`, `out[1]` and the `u16` at `out + 4`.
///
/// # Safety
/// The info tables and descriptor must be readable; `out` must be writable for
/// 8 bytes.
///
/// # Confidence
/// medium - the bit loop and stores are literal; the descriptor convention is
/// inferred from the sibling decoders and the `0x80276F50` export.
pub unsafe fn fn_80276300(this: usize, id: u32, out: usize) -> u32 {
    if fn_80275ee0(this, id) != 2 {
        return 0;
    }

    let mut descriptor = [0u8; 8];
    let descriptor_addr = descriptor.as_mut_ptr() as usize;
    let found = platform::call3(FUN_80276F50, this as u32, id, descriptor_addr as u32);
    if found == 0 {
        return 0;
    }

    let base = platform::read_u32(this + 0x28) as usize;
    let record = platform::call3(
        FUN_80283A60,
        descriptor[0] as u32,
        platform::read_u32(descriptor_addr + 4),
        base as u32,
    ) as usize;
    if record == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(record));
    platform::write_u16(out + 6, platform::read_u16(record + 6));

    if (platform::read_u16(this + 6) as u32) < 0x104 {
        platform::write_u16(out + 4, 0);
        let mut bits = platform::read_u16(record + 4) as u32;
        let mut count: u16 = 0;
        while bits != 0 {
            if bits & 1 == 0 {
                return 0;
            }
            count += 1;
            bits >>= 1;
        }
        platform::write_u16(out + 4, count);
    } else {
        platform::write_u16(out + 4, platform::read_u16(record + 4));
    }
    1
}

// ===========================================================================
// 0x80276410
// ===========================================================================

/// Resolves a type-3 archive id to a three-word record.
///
/// The id kind must be `3`. The record supplies word `0` and the byte at
/// `+0x08` as word `1`. Version `< 0x103` writes zero at `out + 8`; newer
/// archives write `1` when record byte `+0x09` is non-zero.
///
/// # Original
/// - Ghidra: `FUN_80276410` at `0x80276410` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276410_FUN_80276410.c`
/// - observed: the `FUN_80275ee0() == 3` gate, the `FUN_80276f50` call, the
///   `*param_3` / `param_3[1]` stores and the `*(ushort *)(param_1 + 6) <
///   0x103` branch on record byte `+9`.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: three-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - writes `out[0]`, `out[1]` and byte `out + 8`.
///
/// # Safety
/// The info tables and descriptor must be readable; `out` must be writable for
/// 12 bytes.
///
/// # Confidence
/// medium - the gate and stores are literal; the descriptor convention is
/// inferred from the sibling decoders and the `0x80276F50` export.
pub unsafe fn fn_80276410(this: usize, id: u32, out: usize) -> u32 {
    if fn_80275ee0(this, id) != 3 {
        return 0;
    }

    let mut descriptor = [0u8; 8];
    let descriptor_addr = descriptor.as_mut_ptr() as usize;
    let found = platform::call3(FUN_80276F50, this as u32, id, descriptor_addr as u32);
    if found == 0 {
        return 0;
    }

    let base = platform::read_u32(this + 0x28) as usize;
    let record = platform::call3(
        FUN_80283A60,
        descriptor[0] as u32,
        platform::read_u32(descriptor_addr + 4),
        base as u32,
    ) as usize;
    if record == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(record));
    platform::write_u32(out + 4, platform::read_u8(record + 8) as u32);

    if (platform::read_u16(this + 6) as u32) < 0x103 {
        platform::write_u8(out + 8, 0);
    } else {
        platform::write_u8(out + 8, u8::from(platform::read_u8(record + 9) != 0));
    }
    1
}

// ===========================================================================
// 0x802764F0
// ===========================================================================

/// Resolves one archive id to its single `+0x04` word.
///
/// The table is named by info `+0x08`/`+0x0C`; the id is bounds-checked and the
/// slot resolved through `0x80283A60` without a version gate. On success
/// `out[0]` receives the entry's `+0x04` word.
///
/// # Original
/// - Ghidra: `FUN_802764f0` at `0x802764F0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802764f0_FUN_802764f0.c`
/// - observed: the `*(int *)(param_1 + 0x28)` info base, the two
///   `FUN_80283a60` calls, the `param_2 < *puVar1` bound and the
///   `*param_3 = *(undefined4 *)(iVar2 + 4)` store.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: one-word output slot.
///
/// # Returns
/// `1` when the entry was found, `0` otherwise.
///
/// # Side effects
/// - writes `out[0]` on success.
///
/// # Safety
/// The info tables must be readable; `out` must be writable for 4 bytes.
///
/// # Confidence
/// high - the two lookups, the bound and the store are all literal.
pub unsafe fn fn_802764f0(this: usize, id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base + 8) as u32,
        platform::read_u32(base + 0x0c),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(entry + 4));
    1
}

// ===========================================================================
// 0x802765A0
// ===========================================================================

/// Resolves one archive id to a byte/word pair.
///
/// The table is named by info `+0x10`/`+0x14`. After the id bound and the
/// `0x80283A60` slot lookup, `out[0]` receives the entry's `+0x04` byte and
/// `out[1]` its `+0x08` word.
///
/// # Original
/// - Ghidra: `FUN_802765a0` at `0x802765A0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802765a0_FUN_802765a0.c`
/// - observed: the `*(int *)(param_1 + 0x28)`, the `+0x10`/`+0x14` byte and
///   offset loads, the `param_2 < *puVar1` bound, the `FUN_80283a60` slot call
///   and the `*param_3` / `param_3[1]` stores.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: two-word output block.
///
/// # Returns
/// `1` when the entry was found, `0` otherwise.
///
/// # Side effects
/// - writes `out[0]` and `out[1]` on success.
///
/// # Safety
/// The info tables must be readable; `out` must be writable for 8 bytes.
///
/// # Confidence
/// high - the lookups, bound and two stores are literal.
pub unsafe fn fn_802765a0(this: usize, id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base + 0x10) as u32,
        platform::read_u32(base + 0x14),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u8(entry + 4) as u32);
    platform::write_u32(out + 4, platform::read_u32(entry + 8));
    1
}

// ===========================================================================
// 0x80276660
// ===========================================================================

/// Resolves one archive id to a six-word info record.
///
/// The table is named by info `+0x20`/`+0x24`. The entry must resolve a second
/// table through its own `+0x20`/`+0x24` pair; that table's first word becomes
/// `out[0]`. `out[1]` is the pointer produced from entry `+0x08`/`+0x0C`, and
/// `out[2..5]` copy entry words `+0x10`, `+0x14`, `+0x18` and `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_80276660` at `0x80276660` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276660_FUN_80276660.c`
/// - observed: the `+0x20`/`+0x24` byte/offset lookup, the nested
///   `FUN_80283a60(*(undefined1 *)(iVar4 + 0x20), *(undefined4 *)(iVar4 +
///   0x24), ...)` call, the `+0x08`/`+0x0C` lookup and the six `param_3`
///   stores.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: six-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - fills `out[0..5]`.
///
/// # Safety
/// The info tables must be readable; `out` must be writable for 24 bytes.
///
/// # Confidence
/// medium - every lookup and store is literal; `out[1]` is a resolved pointer
/// rather than a raw entry field, inferred from the actual call result.
pub unsafe fn fn_80276660(this: usize, id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base + 0x20) as u32,
        platform::read_u32(base + 0x24),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }

    let nested = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 0x20) as u32,
        platform::read_u32(entry + 0x24),
        base as u32,
    ) as usize;
    if nested == 0 {
        return 0;
    }

    let name = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 8) as u32,
        platform::read_u32(entry + 0x0c),
        base as u32,
    ) as u32;

    platform::write_u32(out, platform::read_u32(nested));
    platform::write_u32(out + 4, name);
    platform::write_u32(out + 8, platform::read_u32(entry + 0x10));
    platform::write_u32(out + 0x0c, platform::read_u32(entry + 0x14));
    platform::write_u32(out + 0x10, platform::read_u32(entry + 0x18));
    platform::write_u32(out + 0x14, platform::read_u32(entry + 0x1c));
    1
}

// ===========================================================================
// 0x80276770
// ===========================================================================

/// Resolves `(id, sub-id)` to a five-word info record.
///
/// The outer table is info `+0x20`/`+0x24`; the selected entry names a second
/// table through its own `+0x20`/`+0x24` pair. `sub_id` is bounds-checked in
/// that table and the resulting entry's words `0..4` are copied to `out`.
///
/// # Original
/// - Ghidra: `FUN_80276770` at `0x80276770` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276770_FUN_80276770.c`
/// - observed: the two `param_2 < *puVar1` / `param_3 < *puVar1` bounds, the
///   nested `FUN_80283a60` calls on the `+0x20`/`+0x24` pairs and the five
///   `param_4` stores.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: first-level entry id.
/// - `sub_id`: second-level entry id.
/// - `out`: five-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - fills `out[0..4]`.
///
/// # Safety
/// Both tables must be readable; `out` must be writable for 20 bytes.
///
/// # Confidence
/// high - both bounds, both lookups and all five stores are literal.
pub unsafe fn fn_80276770(this: usize, id: u32, sub_id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base + 0x20) as u32,
        platform::read_u32(base + 0x24),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }

    let sub_table = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 0x20) as u32,
        platform::read_u32(entry + 0x24),
        base as u32,
    ) as usize;
    if sub_table == 0 {
        return 0;
    }
    if sub_id >= platform::read_u32(sub_table) {
        return 0;
    }

    let sub_slot = sub_table + sub_id as usize * 8;
    let record = platform::call3(
        FUN_80283A60,
        platform::read_u8(sub_slot + 4) as u32,
        platform::read_u32(sub_slot + 8),
        base as u32,
    ) as usize;
    if record == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(record));
    platform::write_u32(out + 4, platform::read_u32(record + 4));
    platform::write_u32(out + 8, platform::read_u32(record + 8));
    platform::write_u32(out + 0x0c, platform::read_u32(record + 0x0c));
    platform::write_u32(out + 0x10, platform::read_u32(record + 0x10));
    1
}

// ===========================================================================
// 0x802768B0
// ===========================================================================

/// Reads the seven-word info header of an archive.
///
/// The header table is named by info `+0x28`/`+0x2C`; `0x80283A60` is called
/// with only the byte and offset (no explicit base). Each of the seven `u16`
/// fields is widened to a word in `out`. The routine returns `false` when
/// `out` is null and otherwise fills all seven slots.
///
/// # Original
/// - Ghidra: `FUN_802768b0` at `0x802768B0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802768b0_FUN_802768b0.c`
/// - observed: the `+0x28`/`+0x2C` lookup, the `param_2 != (uint *)0x0` guard
///   and the seven `param_2[N] = (uint)puVar1[N]` stores.
///
/// # Arguments
/// - `this`: archive object.
/// - `out`: seven-word output block, or null to query only.
///
/// # Returns
/// `1` when `out` is non-null, `0` when it is null.
///
/// # Side effects
/// - fills `out[0..6]` when `out` is non-null.
///
/// # Safety
/// The header table must be readable; `out` must be writable for 28 bytes.
///
/// # Confidence
/// high - the lookup, null test and seven stores are literal.
pub unsafe fn fn_802768b0(this: usize, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let header = platform::call2(
        FUN_80283A60,
        platform::read_u8(base + 0x28) as u32,
        platform::read_u32(base + 0x2c),
    ) as usize;

    if out != 0 {
        platform::write_u32(out, platform::read_u16(header) as u32);
        platform::write_u32(out + 4, platform::read_u16(header + 2) as u32);
        platform::write_u32(out + 8, platform::read_u16(header + 4) as u32);
        platform::write_u32(out + 0x0c, platform::read_u16(header + 6) as u32);
        platform::write_u32(out + 0x10, platform::read_u16(header + 8) as u32);
        platform::write_u32(out + 0x14, platform::read_u16(header + 10) as u32);
        platform::write_u32(out + 0x18, platform::read_u16(header + 12) as u32);
    }

    u32::from(out != 0)
}

// ===========================================================================
// 0x80276940
// ===========================================================================

/// Returns the first word of the `+0x10` archive table.
///
/// The byte and offset at info `+0x10`/`+0x14` are resolved through
/// `0x80283A60`; a null table yields `0`, otherwise its first word is returned.
///
/// # Original
/// - Ghidra: `FUN_80276940` at `0x80276940` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276940_FUN_80276940.c`
/// - observed: `FUN_80283a60(*(undefined1 *)(*(int *)(param_1 + 0x28) + 0x10),
///   *(undefined4 *)(*(int *)(param_1 + 0x28) + 0x14));` and the null branch.
///
/// # Arguments
/// - `this`: archive object.
///
/// # Returns
/// The table's first word, or `0` when the table is absent.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// The info block and the table it names must be readable.
///
/// # Confidence
/// high - the lookup and the null branch are literal.
pub unsafe fn fn_80276940(this: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call2(
        FUN_80283A60,
        platform::read_u8(base + 0x10) as u32,
        platform::read_u32(base + 0x14),
    ) as usize;

    if table == 0 {
        0
    } else {
        platform::read_u32(table)
    }
}

// ===========================================================================
// 0x80276980
// ===========================================================================

/// Returns one less than the first word of the `+0x20` archive table.
///
/// The byte and offset at info `+0x20`/`+0x24` are resolved through
/// `0x80283A60`; a null table yields `0`, otherwise `table[0] - 1` is returned.
/// `// inferred:` the subtraction makes the table word a count.
///
/// # Original
/// - Ghidra: `FUN_80276980` at `0x80276980` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276980_FUN_80276980.c`
/// - observed: `FUN_80283a60(... + 0x20, ... + 0x24);` and
///   `iVar2 = *piVar1 + -1;`.
///
/// # Arguments
/// - `this`: archive object.
///
/// # Returns
/// `table[0] - 1`, or `0` when the table is absent.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// The info block and the table it names must be readable.
///
/// # Confidence
/// high - the lookup, the null branch and the decrement are literal.
pub unsafe fn fn_80276980(this: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call2(
        FUN_80283A60,
        platform::read_u8(base + 0x20) as u32,
        platform::read_u32(base + 0x24),
    ) as usize;

    if table == 0 {
        0
    } else {
        platform::read_u32(table).wrapping_sub(1)
    }
}

// ===========================================================================
// 0x802769D0
// ===========================================================================

/// Resolves one id to a base-relative pointer from the `+0x30` pointer table.
///
/// The first table is named by info `+0x00`/`+0x04`. Version `< 0x101` uses the
/// base-relative offset in the slot; newer archives resolve the slot through
/// `0x80283A60`. The entry's first word is an index into the `+0x30` pointer
/// table at `+0x2C` base; index `-1`, a null table, a null pointer or a null
/// index all return `0`.
///
/// # Original
/// - Ghidra: `FUN_802769d0` at `0x802769D0` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802769d0_FUN_802769d0.c`
/// - observed: the `*puVar5` / `puVar5 + 4` lookup, the `param_2 < *puVar1`
///   bound, the version branch, the `iVar3 == -1` test, the
///   `*(int *)(param_1 + 0x30)` guard and the `+ iVar3 * 4 + 4` load.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
///
/// # Returns
/// The resolved pointer (`pointer + *(this + 0x2C)`), or `0`.
///
/// # Side effects
/// None beyond the lookups.
///
/// # Safety
/// The info tables and the pointer table must be readable.
///
/// # Confidence
/// medium - the loads, bounds and index arithmetic are literal; the role of
/// `+0x30`/`+0x2C` as a pointer table and its base is inferred from the
/// `*4 + 4` indexing.
pub unsafe fn fn_802769d0(this: usize, id: u32) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call2(
        FUN_80283A60,
        platform::read_u8(base) as u32,
        platform::read_u32(base + 4),
    ) as usize;

    let entry = if table == 0 {
        0
    } else if id >= platform::read_u32(table) {
        0
    } else {
        let slot = table + id as usize * 8;
        if (platform::read_u16(this + 6) as u32) < 0x101 {
            platform::read_u32(slot + 8).wrapping_add(base as u32) as usize
        } else {
            platform::call3(
                FUN_80283A60,
                platform::read_u8(slot + 4) as u32,
                platform::read_u32(slot + 8),
                base as u32,
            ) as usize
        }
    };

    let index = if entry == 0 {
        -1
    } else {
        platform::read_i32(entry)
    };
    if index == -1 {
        return 0;
    }

    let pointers = platform::read_u32(this + 0x30) as usize;
    if pointers == 0 {
        return 0;
    }

    let value = platform::read_i32(pointers + index as usize * 4 + 4);
    if value == 0 {
        0
    } else {
        value.wrapping_add(platform::read_u32(this + 0x2c) as i32) as u32
    }
}

// ===========================================================================
// 0x80276AE0
// ===========================================================================

/// Returns the `+0x20` word of one archive entry.
///
/// The table is named by info `+0x00`/`+0x04`. Version `< 0x101` uses the
/// base-relative slot offset; newer archives resolve the slot through
/// `0x80283A60`. A missing table, out-of-range id or null entry returns `0`.
///
/// # Original
/// - Ghidra: `FUN_80276ae0` at `0x80276AE0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276ae0_FUN_80276ae0.c`
/// - observed: the `*puVar5` / `puVar5 + 4` lookup, the `param_2 < *puVar1`
///   bound, the version branch and `*(undefined4 *)(iVar2 + 0x20)`.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
///
/// # Returns
/// The entry's `+0x20` word, or `0`.
///
/// # Side effects
/// None beyond the lookups.
///
/// # Safety
/// The info table must be readable.
///
/// # Confidence
/// high - the two lookup paths and the final load are literal.
pub unsafe fn fn_80276ae0(this: usize, id: u32) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base) as u32,
        platform::read_u32(base + 4),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = if (platform::read_u16(this + 6) as u32) < 0x101 {
        platform::read_u32(slot + 8).wrapping_add(base as u32) as usize
    } else {
        platform::call3(
            FUN_80283A60,
            platform::read_u8(slot + 4) as u32,
            platform::read_u32(slot + 8),
            base as u32,
        ) as usize
    };

    if entry == 0 {
        0
    } else {
        platform::read_u32(entry + 0x20)
    }
}

// ===========================================================================
// 0x80276BA0
// ===========================================================================

/// Returns the first word of the `+0x18` archive table.
///
/// The byte and offset at info `+0x18`/`+0x1C` are resolved through
/// `0x80283A60` with two arguments. A null table yields `0`.
///
/// # Original
/// - Ghidra: `FUN_80276ba0` at `0x80276BA0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276ba0_FUN_80276ba0.c`
/// - observed: `FUN_80283a60(... + 0x18, ... + 0x1c);` and the null branch.
///
/// # Arguments
/// - `this`: archive object.
///
/// # Returns
/// The table's first word, or `0`.
///
/// # Side effects
/// None beyond the lookup.
///
/// # Safety
/// The info block and the table it names must be readable.
///
/// # Confidence
/// high - the lookup and null branch are literal.
pub unsafe fn fn_80276ba0(this: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call2(
        FUN_80283A60,
        platform::read_u8(base + 0x18) as u32,
        platform::read_u32(base + 0x1c),
    ) as usize;

    if table == 0 {
        0
    } else {
        platform::read_u32(table)
    }
}

// ===========================================================================
// 0x80276BE0
// ===========================================================================

/// Resolves one archive id to a four-word record plus a sub-table.
///
/// The table is named by info `+0x18`/`+0x1C`. After the id bound and slot
/// lookup, the entry's `+0x14`/`+0x18` pair names a sub-table whose first word
/// becomes `out[3]`. `out[0]` and `out[1]` copy entry words `0` and `1`;
/// `out[2]` is the pointer produced from the entry's `+0x0C`/`+0x10` pair.
///
/// # Original
/// - Ghidra: `FUN_80276be0` at `0x80276BE0` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276be0_FUN_80276be0.c`
/// - observed: the `+0x18`/`+0x1C` lookup, the `param_2 < *puVar1` bound, the
///   `FUN_80283a60(*(undefined1 *)(puVar3 + 5), puVar3[6], ...)` sub-table
///   call, the `+3`/`[4]` name lookup and the four `param_3` stores.
///
/// # Arguments
/// - `this`: archive object.
/// - `id`: entry id.
/// - `out`: four-word output block.
///
/// # Returns
/// `1` when the record and sub-table were found, `0` otherwise.
///
/// # Side effects
/// - fills `out[0..3]`.
///
/// # Safety
/// The info tables must be readable; `out` must be writable for 16 bytes.
///
/// # Confidence
/// medium - every lookup and store is literal; the split between the raw entry
/// words and the resolved name pointer is inferred from the call result.
pub unsafe fn fn_80276be0(this: usize, id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base + 0x18) as u32,
        platform::read_u32(base + 0x1c),
        base as u32,
    ) as usize;

    if table == 0 {
        return 0;
    }
    if id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }

    let sub_table = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 0x14) as u32,
        platform::read_u32(entry + 0x18),
        base as u32,
    ) as usize;
    if sub_table == 0 {
        return 0;
    }

    let name = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 0x0c) as u32,
        platform::read_u32(entry + 0x10),
        base as u32,
    ) as u32;

    platform::write_u32(out, platform::read_u32(entry));
    platform::write_u32(out + 4, platform::read_u32(entry + 4));
    platform::write_u32(out + 8, name);
    platform::write_u32(out + 0x0c, platform::read_u32(sub_table));
    1
}

// ===========================================================================
// Wave 66: resolver tail and the sound-system archive-consumer layer,
// 0x80275850..0x8027AFA0
// ===========================================================================
//
// The archive front end above resolves ids to records. This block is the
// consumer. `0x80275850` and `0x802770D0` load a whole sound archive through
// the two loader vtable slots; `0x80276CE0..0x80276F50` finish the id-resolver
// run; `0x802776B0`..`0x802779D0` build and tear down the sound-system object
// that owns the three registries; and `0x80277A90`..`0x8027AFA0` size, fill and
// query those registries from the archive's own tables.

// ===========================================================================
// 0x80275850
// ===========================================================================

/// Loads one archive entry, optionally followed by a second stream, and relays
/// the loader result.
///
/// The four-word record of `id` names another id via its `+0x08` word. When
/// that word is non-null the second stream is opened immediately; otherwise the
/// entry is resolved again through `0x80276CE0` (sub-id `0`), the six-word
/// record supplies the stream name/size and the five-word record supplies the
/// second destination offset. Names that do not start with `/` are prefixed
/// with the handle's `+0x08` string.
///
/// # Original
/// - Ghidra: `FUN_80275850` at `0x80275850` (552 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80275850_FUN_80275850.c`
/// - observed: the `FUN_80276be0(piVar1[1],(int)uVar5,local_260)` call, the
///   `local_258 == 0` split, the `FUN_80276ce0(...,0,...)` /
///   `FUN_80276660` / `FUN_80276770` chain, the `*local_234 != '/'` test, the
///   `0xff < (uint)(iVar2 + iVar4)` guard, the two copies into
///   `acStack_220`/`acStack_120`, the `+0x18`/`+0x1c` vtable calls and
///   `FUN_802dcdb8(uVar3)`.
/// - `// inferred:` `param_1` is an archive *handle* (its `+0x04` word is the
///   RSAR info block and `+0x08` the path prefix) because the sibling loader
///   [`fn_80275a80`] is called with `system + 0x18` from [`fn_802770d0`].
///
/// # Arguments
/// - `this`: archive handle; `+0x00` vtable, `+0x04` info block, `+0x08`
///   prefix string.
/// - `id`: entry id.
/// - `arg3`, `arg4`: forwarded untouched to the loader vtable slot.
///
/// # Returns
/// The loader result. The C ends with `FUN_802dcdb8(uVar3)`, a bare-return
/// relay that hands `uVar3` back in `r3`; the Rust body returns it directly.
///
/// # Side effects
/// - loads through vtable slots `+0x18` (single stream) or `+0x1C` (with a
///   second stream),
/// - writes two 256-byte stack path buffers.
///
/// # Safety
/// `this` must be a live archive handle with a readable vtable and prefix; the
/// ids must exist in the archive.
///
/// # Confidence
/// medium - every test, copy and call is literal; the handle layout and the
/// meaning of `arg3`/`arg4` are inferred from the loader's other call site.
pub unsafe fn fn_80275850(this: usize, id: u32, arg3: u32, arg4: u32) -> u32 {
    let mut header = [0u32; 4];
    let mut result: u32 = 0;

    if platform::call3(FUN_80276BE0, this as u32, id, header.as_mut_ptr() as u32) != 0 {
        if header[2] == 0 {
            let mut pair = [0u32; 2];
            if platform::call4(FUN_80276CE0, this as u32, id, 0, pair.as_mut_ptr() as u32) != 0 {
                let mut six = [0u32; 6];
                if platform::call3(FUN_80276660, this as u32, pair[0], six.as_mut_ptr() as u32) != 0
                {
                    let mut five = [0u32; 5];
                    if platform::call4(
                        FUN_80276770,
                        this as u32,
                        pair[0],
                        pair[1],
                        five.as_mut_ptr() as u32,
                    ) != 0
                    {
                        let vtable = platform::read_u32(this) as usize;
                        let size = six[2].wrapping_add(five[1]);
                        if six[1] == 0 {
                            let load = platform::read_u32(vtable + 0x18) as usize;
                            result = platform::call5(load, this as u32, arg3, arg4, size, five[2]);
                        } else {
                            let mut path = [0u8; 256];
                            if let Some(name) = archive_path(this, six[1], &mut path) {
                                let load = platform::read_u32(vtable + 0x1c) as usize;
                                result = platform::call6(
                                    load,
                                    this as u32,
                                    arg3,
                                    arg4,
                                    name,
                                    size,
                                    five[2],
                                );
                            }
                        }
                    }
                }
            }
        } else {
            let mut path = [0u8; 256];
            let mut name = header[2];
            let mut rejected = false;
            if platform::read_u8(name as usize) != b'/' {
                match archive_path(this, name, &mut path) {
                    Some(built) => name = built,
                    None => rejected = true,
                }
            }
            if !rejected {
                let vtable = platform::read_u32(this) as usize;
                let load = platform::read_u32(vtable + 0x1c) as usize;
                result = platform::call6(load, this as u32, arg3, arg4, name, 0, header[0]);
            }
        }
    }

    result
}

// ===========================================================================
// 0x80276CE0
// ===========================================================================

/// Resolves `(id, sub-id)` to a two-word record.
///
/// The `+0x18`/`+0x1C` byte/offset pair of the archive info block names the
/// first table; the id-indexed slot names the entry, whose `+0x14`/`+0x18`
/// pair names the second table, whose sub-id slot names the two-word record.
/// Every hop goes through the shared resolver `0x80283A60` and is bounds-
/// checked against the table's first word.
///
/// # Original
/// - Ghidra: `FUN_80276ce0` at `0x80276CE0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276ce0_FUN_80276ce0.c`
/// - observed: the three `FUN_80283a60` calls, the `param_2 < *puVar1` and
///   `param_3 < *puVar1` bounds, the `+0x14`/`+0x18` sub-table pair and the
///   two `param_4` stores.
///
/// # Arguments
/// - `this`: archive object (`+0x28` is the info base).
/// - `id`: first-level entry id.
/// - `sub_id`: second-level id.
/// - `out`: two-word output block.
///
/// # Returns
/// `1` when the record was found, `0` otherwise.
///
/// # Side effects
/// - fills `out[0..1]` on success.
///
/// # Safety
/// The info tables must be readable; `out` must be writable for 8 bytes.
///
/// # Confidence
/// high - every lookup, bound and store is literal, and the shape matches
/// [`fn_80276be0`]'s already-translated sibling.
pub unsafe fn fn_80276ce0(this: usize, id: u32, sub_id: u32, out: usize) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base + 0x18) as u32,
        platform::read_u32(base + 0x1c),
        base as u32,
    ) as usize;
    if table == 0 || id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }

    let sub_table = platform::call3(
        FUN_80283A60,
        platform::read_u8(entry + 0x14) as u32,
        platform::read_u32(entry + 0x18),
        base as u32,
    ) as usize;
    if sub_table == 0 || sub_id >= platform::read_u32(sub_table) {
        return 0;
    }

    let sub_slot = sub_table + sub_id as usize * 8;
    let record = platform::call3(
        FUN_80283A60,
        platform::read_u8(sub_slot + 4) as u32,
        platform::read_u32(sub_slot + 8),
        base as u32,
    ) as usize;
    if record == 0 {
        return 0;
    }

    platform::write_u32(out, platform::read_u32(record));
    platform::write_u32(out + 4, platform::read_u32(record + 4));
    1
}

// ===========================================================================
// 0x80276E00
// ===========================================================================

/// Walks the archive's name trie and returns the id whose stored name matches.
///
/// Each 20-byte node holds a flag word (`+0x08`), a key word (`+0x0A`), two
/// child indices (`+0x0C`/`+0x10`), a name index (`+0x14`) and the id value
/// (`+0x18`). The walk starts at node `*tree` and follows the right child until
/// a node whose flag has bit `0x10000` set. At that node the key word selects
/// one bit of the caller's name: bit `7 - (key & 7)` of byte `key >> 3`;
/// a set bit follows `+0x10`, a clear or out-of-range bit follows `+0x0C`.
/// Leaf names live in the table at `this + 0x30` (`this + 0x2C` is its base
/// offset) and are compared with `strcmp`.
///
/// # Original
/// - Ghidra: `FUN_80276e00` at `0x80276E00` (328 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276e00_FUN_80276e00.c`
/// - observed: the `param_2 == 0` guard, the `uVar2 < param_2[1]` bound, the
///   `param_2[uVar2 * 5 + 2] & 0x10000` loop test, the `>> 3` /
///   `1 << 7 - (uVar1 & 7)` bit selection, the two child reads, the
///   `0xffffffff` name-index test, the `+0x30`/`+0x2c` name lookup and
///   `FUN_802e1ab4` (strcmp) followed by the `+0x18` value read.
///
/// # Arguments
/// - `this`: archive object (`+0x2C` name base, `+0x30` name-offset table).
/// - `tree`: trie base; word 0 is the root node, word 1 the node count.
/// - `name`: NUL-terminated name to match.
///
/// # Returns
/// The stored id, or `0xFFFFFFFF` when the tree is empty, the walk runs out or
/// the names differ.
///
/// # Safety
/// Every node index read from the trie must be below the node count, as the
/// original assumes; `name` must be NUL-terminated.
///
/// # Confidence
/// medium - the control flow and bit test are literal; the flag bit `0x10000`
/// and the id meaning of `+0x18` are inferred from the result's users.
pub unsafe fn fn_80276e00(this: usize, tree: usize, name: usize) -> u32 {
    if tree == 0 {
        return 0xffff_ffff;
    }
    let mut index = platform::read_u32(tree);
    if index >= platform::read_u32(tree + 4) {
        return 0xffff_ffff;
    }

    let name_len = platform::call1(STRLEN, name as u32) as i32;
    loop {
        let slot = tree + index as usize * 0x14;
        if platform::read_u32(slot + 8) & 0x1_0000 != 0 {
            break;
        }
        let key = platform::read_u16(slot + 0x0a) as u32;
        let byte_index = (key >> 3) as i32;
        let bit = 1u32 << (7 - (key & 7));
        let take_high = byte_index < name_len
            && (bit & platform::read_u8(name + byte_index as usize) as u32) != 0;
        index = platform::read_u32(if take_high { slot + 0x10 } else { slot + 0x0c });
    }

    let slot = tree + index as usize * 0x14;
    let name_index = platform::read_u32(slot + 0x14);
    if name_index == 0xffff_ffff {
        return 0xffff_ffff;
    }
    let offsets = platform::read_u32(this + 0x30) as usize;
    if offsets == 0 {
        return 0xffff_ffff;
    }
    let entry = platform::read_u32(offsets + name_index as usize * 4 + 4);
    if entry == 0 {
        return 0xffff_ffff;
    }
    let stored = entry.wrapping_add(platform::read_u32(this + 0x2c));
    if platform::call2(STRCMP, name as u32, stored) == 0 {
        return platform::read_u32(slot + 0x18);
    }
    0xffff_ffff
}

// ===========================================================================
// 0x80276F50
// ===========================================================================

/// Resolves one id to the eight-byte descriptor the record decoders use.
///
/// For archives older than version `0x101` the descriptor is built from the
/// entry table slot itself: two bytes, the caller's `tag` and the entry's
/// offset plus `0x1C`. Newer archives resolve the slot through `0x80283A60`
/// and copy the entry's `+0x18..+0x1F` bytes.
///
/// # Original
/// - Ghidra: `FUN_80276f50` at `0x80276F50` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80276f50_FUN_80276f50.c`
/// - observed: the `*puVar6`/`*(puVar6 + 4)` table lookup, the
///   `param_2 < *puVar2` bound, the `*(ushort *)(param_1 + 6) < 0x101` split,
///   the two byte stores plus `param_5` and `uVar5 + 0x1c` in the old path,
///   and the four `+0x18..+0x1F` copies in the new path.
///
/// # Arguments
/// - `this`: archive object (`+0x06` version, `+0x28` info base).
/// - `id`: entry id.
/// - `out`: eight-byte descriptor.
/// - `_arg4`: unused by the original body.
/// - `tag`: `u16` stored at `out + 2` in the old-version path.
///
/// # Returns
/// `1` when the descriptor was built, `0` otherwise.
///
/// # Side effects
/// - fills `out[0..8]`.
///
/// # Safety
/// The info table must be readable; `out` must be writable for 8 bytes.
///
/// # Confidence
/// medium - every branch and store is literal; the descriptor field roles are
/// inferred from the three record decoders that call this function.
pub unsafe fn fn_80276f50(this: usize, id: u32, out: usize, _arg4: u32, tag: u16) -> u32 {
    let base = platform::read_u32(this + 0x28) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(base) as u32,
        platform::read_u32(base + 4),
        base as u32,
    ) as usize;
    if table == 0 || id >= platform::read_u32(table) {
        return 0;
    }

    let slot = table + id as usize * 8;
    if platform::read_u16(this + 6) < 0x101 {
        platform::write_u8(out, platform::read_u8(slot + 4));
        platform::write_u8(out + 1, platform::read_u8(slot + 5));
        platform::write_u16(out + 2, tag);
        platform::write_u32(out + 4, platform::read_u32(slot + 8).wrapping_add(0x1c));
        return 1;
    }

    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(slot + 4) as u32,
        platform::read_u32(slot + 8),
        base as u32,
    ) as usize;
    if entry == 0 {
        return 0;
    }
    platform::write_u8(out, platform::read_u8(entry + 0x18));
    platform::write_u8(out + 1, platform::read_u8(entry + 0x19));
    platform::write_u16(out + 2, platform::read_u16(entry + 0x1a));
    platform::write_u32(out + 4, platform::read_u32(entry + 0x1c));
    1
}

// ===========================================================================
// 0x80277090
// ===========================================================================

/// Frees an archive-file object when the caller's delete flag is positive.
///
/// This is the free half of the pair whose construction path is
/// [`fn_802770d0`]; the `this != 0` guard makes it safe on a null pointer.
///
/// # Original
/// - Ghidra: `FUN_80277090` at `0x80277090` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277090_FUN_80277090.c`
/// - observed: `if ((param_1 != 0) && (0 < param_2)) { FUN_802b9280(); }`
///   followed by `return param_1;`
///
/// # Arguments
/// - `this`: object to release.
/// - `flag`: positive to free the storage, zero or negative to leave it.
///
/// # Returns
/// `this`, so the caller can chain the deleting destructor.
///
/// # Side effects
/// - calls `0x802B9280` (the base-object free) when `this != 0 && flag > 0`.
///
/// # Safety
/// `this` must be null or a live archive-file object.
///
/// # Confidence
/// high - the guard and the call are literal.
pub unsafe fn fn_80277090(this: usize, flag: i32) -> usize {
    if this != 0 && flag > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// 0x802770D0
// ===========================================================================

/// Opens a loader object and reserves its whole payload in the caller's sink.
///
/// The loader protocol is `vtable[0x50]()` (prepare), `vtable[0x2C]()`
/// (open/check), `vtable[0x40]()` (payload size) and the sink's
/// `vtable[0x0C](sink, size)` (reserve). The returned word is the sink's
/// destination offset/pointer.
///
/// # Original
/// - Ghidra: `FUN_802770d0` calls this sequence inline twice, once per stream;
///   this helper is the extracted shape, not a separate image function.
/// - Evidence: `work/ghidra/main-gb/export/802770d0_FUN_802770d0.c`
/// - observed: the `*piVar2 + 0x50`, `+0x2c`, `+0x40` slot calls and the
///   `*param_3 + 0xc` reserve call with the size.
///
/// # Returns
/// `Some(offset)` on success, `None` when any step returns zero.
///
/// # Safety
/// Both objects must be live and expose the slots above.
unsafe fn open_archive_stream(stream: usize, sink: usize) -> Option<u32> {
    let vtable = platform::read_u32(stream) as usize;
    let prepare = platform::read_u32(vtable + 0x50) as usize;
    if platform::call1(prepare, stream as u32) == 0 {
        return None;
    }
    let open = platform::read_u32(vtable + 0x2c) as usize;
    if platform::call1(open, stream as u32) == 0 {
        return None;
    }
    let size_fn = platform::read_u32(vtable + 0x40) as usize;
    let size = platform::call1(size_fn, stream as u32);
    let sink_vtable = platform::read_u32(sink) as usize;
    let reserve = platform::read_u32(sink_vtable + 0x0c) as usize;
    let base = platform::call2(reserve, sink as u32, size);
    if base == 0 {
        None
    } else {
        Some(base)
    }
}

/// Calls the `+0x10` destructor slot of an archive loader object.
///
/// # Safety
/// `stream` must be a live loader object with a readable vtable.
unsafe fn destroy_archive_stream(stream: usize) {
    let vtable = platform::read_u32(stream) as usize;
    platform::call1(platform::read_u32(vtable + 0x10) as usize, stream as u32);
}

/// Loads one sound archive into the caller's sink and optionally its second
/// stream.
///
/// The id is first loaded through [`fn_80275a80`] (`0x80275A80`), whose result
/// is a loader object; the payload is read into the sink reserved by
/// `vtable[0x0C]`, either in one call or in `chunk`-sized pieces. The six-word
/// record of the same id then decides whether a second stream exists. When it
/// does, the record of `0x80275BA0`'s loader is read the same way and its sink
/// offset is reported through `second_offset`; otherwise `second_offset`
/// receives zero. `this + 0x21C` holds the loader currently being filled, so a
/// concurrent teardown can find it.
///
/// # Original
/// - Ghidra: `FUN_802770d0` at `0x802770D0` (1492 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802770d0_FUN_802770d0.c`
/// - observed: the `FUN_80275a80(*(iVar1 + 0x18), uVar7, iVar1 + 0x1c, 0x200)`
///   load, the `+0x50`/`+0x2c`/`+0x40`/`+0x14`/`+0x10` slot uses, the
///   `param_5`-sized `while (uVar9 != 0)` read loop, the
///   `FUN_80275800(...)` six-word record, the `local_34 == 0` split,
///   `FUN_80275ba0`, the `*param_4 = 0` / `*param_4 = iVar5` stores and the
///   `FUN_802dcdb0(iVar3)` relay.
/// - `// inferred:` the two bare `FUN_801b2e60()` / `FUN_801b2f40(iVar1)`
///   calls are the sound-system lock at `this + 0x54`; Ghidra dropped the
///   argument (it prints the same call with an explicit `+ 0x54` in
///   [`fn_80277bc0`], from the same object).
/// - `// inferred:` `param_3` is the sink object and `param_4` the optional
///   second-stream offset output.
///
/// # Arguments
/// - `this`: sound-system object; `+0x18` archive handle, `+0x1C` path buffer,
///   `+0x21C` active loader, `+0x54` mutex.
/// - `id`: archive entry id.
/// - `sink`: destination object with a reserve/read vtable.
/// - `second_offset`: optional out word for the second stream's offset.
/// - `chunk`: read size; zero means "read the whole payload in one call".
///
/// # Returns
/// The sink offset of the last stream read, or `0` on any failure.
///
/// # Side effects
/// - locks and unlocks `this + 0x54`,
/// - loads two streams, writes `this + 0x21C`, fills the sink,
/// - writes `*second_offset` when a second stream exists or when it does not,
///
/// # Safety
/// All objects must be live; `sink` and `second_offset` must be writable as
/// described.
///
/// # Confidence
/// medium - the whole control flow is literal, but the mutex argument and the
/// sink/out roles are recovered from sibling functions.
pub unsafe fn fn_802770d0(
    this: usize,
    id: u32,
    sink: usize,
    second_offset: usize,
    chunk: u32,
) -> u32 {
    let mutex = (this + 0x54) as u32;
    platform::call1(MUTEX_LOCK, mutex);

    let handle = platform::read_u32(this + 0x18) as usize;
    let stream =
        platform::call4(FN_80275A80, handle as u32, id, (this + 0x1c) as u32, 0x200) as usize;
    if stream == 0 {
        platform::call1(MUTEX_UNLOCK, mutex);
        return 0;
    }

    let base = match open_archive_stream(stream, sink) {
        Some(base) => base,
        None => {
            destroy_archive_stream(stream);
            platform::call1(MUTEX_UNLOCK, mutex);
            return 0;
        }
    };

    platform::write_u32(this + 0x21c, stream as u32);
    let vtable = platform::read_u32(stream) as usize;
    let size_fn = platform::read_u32(vtable + 0x40) as usize;
    let read_fn = platform::read_u32(vtable + 0x14) as usize;
    let mut result = base;

    if chunk == 0 {
        let size = platform::call1(size_fn, stream as u32);
        if (platform::call3(read_fn, stream as u32, base, size) as i32) < 0 {
            platform::write_u32(this + 0x21c, 0);
            destroy_archive_stream(stream);
            platform::call1(MUTEX_UNLOCK, mutex);
            return 0;
        }
    } else {
        let mut remaining = platform::call1(size_fn, stream as u32);
        let mut cursor = base;
        while remaining != 0 {
            let want = if remaining < chunk { remaining } else { chunk };
            let done = platform::call3(read_fn, stream as u32, cursor, want);
            if (done as i32) < 0 {
                platform::write_u32(this + 0x21c, 0);
                destroy_archive_stream(stream);
                platform::call1(MUTEX_UNLOCK, mutex);
                return 0;
            }
            if done < remaining {
                remaining -= done;
                cursor = cursor.wrapping_add(done);
            } else {
                remaining = 0;
            }
        }
    }

    platform::write_u32(this + 0x21c, 0);

    let mut info = [0u32; 6];
    if platform::call3(FN_80275800, handle as u32, id, info.as_mut_ptr() as u32) == 0 {
        destroy_archive_stream(stream);
        platform::call1(MUTEX_UNLOCK, mutex);
        return 0;
    }

    if info[5] == 0 {
        if second_offset != 0 {
            platform::write_u32(second_offset, 0);
        }
    } else {
        let second =
            platform::call4(FN_80275BA0, handle as u32, id, (this + 0x1c) as u32, 0x200) as usize;
        if second == 0 {
            destroy_archive_stream(stream);
            platform::call1(MUTEX_UNLOCK, mutex);
            return 0;
        }

        let second_base = match open_archive_stream(second, sink) {
            Some(base) => base,
            None => {
                destroy_archive_stream(second);
                destroy_archive_stream(stream);
                platform::call1(MUTEX_UNLOCK, mutex);
                return 0;
            }
        };

        platform::write_u32(this + 0x21c, second as u32);
        let second_vtable = platform::read_u32(second) as usize;
        let second_size = platform::read_u32(second_vtable + 0x40) as usize;
        let second_read = platform::read_u32(second_vtable + 0x14) as usize;
        let mut failed = false;

        if chunk == 0 {
            let size = platform::call1(second_size, second as u32);
            if (platform::call3(second_read, second as u32, second_base, size) as i32) < 0 {
                failed = true;
            }
        } else {
            let mut remaining = platform::call1(second_size, second as u32);
            let mut cursor = second_base;
            while remaining != 0 && !failed {
                let want = if remaining < chunk { remaining } else { chunk };
                let done = platform::call3(second_read, second as u32, cursor, want);
                if (done as i32) < 0 {
                    failed = true;
                } else if done < remaining {
                    remaining -= done;
                    cursor = cursor.wrapping_add(done);
                } else {
                    remaining = 0;
                }
            }
        }

        if failed {
            platform::write_u32(this + 0x21c, 0);
            destroy_archive_stream(second);
            destroy_archive_stream(stream);
            platform::call1(MUTEX_UNLOCK, mutex);
            return 0;
        }

        platform::write_u32(this + 0x21c, 0);
        if second_offset != 0 {
            platform::write_u32(second_offset, second_base);
        }
        result = second_base;
        destroy_archive_stream(second);
    }

    destroy_archive_stream(stream);
    platform::call1(MUTEX_UNLOCK, mutex);
    result
}

// ===========================================================================
// 0x802776B0
// ===========================================================================

/// Constructs the sound-system object and registers it with `0x80268C60`.
///
/// The object owns three intrusive registries (`+0x44`, `+0x6C`, `+0x94`),
/// each with a mutex 16 bytes later, an array slot block at `+0x3C`/`+0x40`,
/// and four vtables (`+0x08`, `+0x0C`, `+0x20`, `+0x28`). Three small allocator
/// headers are embedded at `+0xBC`, `+0xE0` and one at `+0xE0`'s buffer block.
/// The constructor zeroes every scalar, points each list sentinel at itself and
/// initialises the three mutexes.
///
/// # Original
/// - Ghidra: `FUN_802776b0` at `0x802776B0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802776b0_FUN_802776b0.c`
/// - observed: the six `param_1[n]` pointer stores (`&DAT_8034f0f8`,
///   `&DAT_8034f10c`, `&DAT_8034f138`, `&DAT_8034f128`, `&DAT_8034eef0`,
///   `&DAT_8034eed0`), the three self-referential sentinel pairs
///   (`+0x4C`, `+0x74`, `+0x9C`), the three `FUN_801b2e20` calls, and the
///   `FUN_80268c60()` / `FUN_80268d50(owner, this)` pair.
///
/// # Arguments
/// - `this`: storage for the object; at least `0xEC` bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - writes `+0x00..+0xE8` (not `+0x30` and not `+0xC8..+0xDC`),
/// - initialises three mutexes at `+0x54`, `+0x7C`, `+0xA4`,
/// - registers the object in the global sound-system list.
///
/// # Safety
/// `this` must point at `0xEC` writable, aligned bytes.
///
/// # Confidence
/// medium - every store is literal; the mutex and registry roles are inferred
/// from [`fn_80277bc0`] and [`fn_802783c0`], which read the same offsets.
pub unsafe fn fn_802776b0(this: usize) -> usize {
    platform::write_u32(this, 0);
    platform::write_u32(this + 0x04, 0);
    platform::write_u32(this + 0x08, VT_SOUND_SYS_A);
    platform::write_u32(this + 0x0c, VT_SOUND_SYS_B);
    platform::write_u32(this + 0x10, 0);
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this + 0x18, 0);
    platform::write_u32(this + 0x1c, 0);
    platform::write_u32(this + 0x20, VT_SOUND_SYS_D);
    platform::write_u32(this + 0x24, this as u32);
    platform::write_u32(this + 0x28, VT_SOUND_SYS_C);
    platform::write_u32(this + 0x2c, this as u32);
    platform::write_u32(this + 0x34, 0);
    platform::write_u32(this + 0x38, 0);
    platform::write_u32(this + 0x3c, 0);
    platform::write_u32(this + 0x40, 0);
    platform::write_u32(this + 0x44, 0);
    platform::write_u32(this + 0x48, 0);
    platform::write_u32(this + 0x4c, (this + 0x4c) as u32);
    platform::write_u32(this + 0x50, (this + 0x4c) as u32);
    platform::call1(MUTEX_INIT, (this + 0x54) as u32);
    platform::write_u32(this + 0x6c, 0);
    platform::write_u32(this + 0x70, 0);
    platform::write_u32(this + 0x74, (this + 0x74) as u32);
    platform::write_u32(this + 0x78, (this + 0x74) as u32);
    platform::call1(MUTEX_INIT, (this + 0x7c) as u32);
    platform::write_u32(this + 0x94, 0);
    platform::write_u32(this + 0x98, 0);
    platform::write_u32(this + 0x9c, (this + 0x9c) as u32);
    platform::write_u32(this + 0xa0, (this + 0x9c) as u32);
    platform::call1(MUTEX_INIT, (this + 0xa4) as u32);
    platform::write_u32(this + 0xbc, VT_SOUND_SYS_BANK);
    platform::write_u32(this + 0xc0, (this + 0xe0) as u32);
    platform::write_u32(this + 0xc4, 0);
    platform::write_u32(this + 0xe0, VT_SOUND_SYS_BUF);
    platform::write_u32(this + 0xe4, 0);
    platform::write_u32(this + 0xe8, 0);

    let owner = call0_r(FUN_80268C60);
    platform::call2(FUN_80268D50, owner, this as u32);
    this
}

// ===========================================================================
// 0x802777D0
// ===========================================================================

/// Deleting destructor of a small sound-system sub-object.
///
/// # Original
/// - Ghidra: `FUN_802777d0` at `0x802777D0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802777d0_FUN_802777d0.c`
/// - observed: `if ((param_1 != 0) && (0 < param_2)) { FUN_802b9280(); }`
///   followed by `return param_1;`
///
/// # Arguments
/// - `this`: object to release.
/// - `flag`: positive to free the storage.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - frees `this` when `this != 0 && flag > 0`.
///
/// # Safety
/// `this` must be null or a live object of this class.
///
/// # Confidence
/// high - the guard and the call are literal.
pub unsafe fn fn_802777d0(this: usize, flag: i32) -> usize {
    if this != 0 && flag > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// 0x80277810
// ===========================================================================

/// Deleting destructor of a second small sound-system sub-object.
///
/// Byte-identical to [`fn_802777d0`]; the two classes differ only in their
/// vtables, which their constructors install.
///
/// # Original
/// - Ghidra: `FUN_80277810` at `0x80277810` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277810_FUN_80277810.c`
/// - observed: identical guard, `FUN_802b9280` call and `return param_1;`.
///
/// # Arguments
/// - `this`: object to release.
/// - `flag`: positive to free the storage.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - frees `this` when `this != 0 && flag > 0`.
///
/// # Safety
/// `this` must be null or a live object of this class.
///
/// # Confidence
/// high - the guard and the call are literal.
pub unsafe fn fn_80277810(this: usize, flag: i32) -> usize {
    if this != 0 && flag > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }
    this
}

// ===========================================================================
// 0x80277870
// ===========================================================================

/// Deleting destructor of a registry-list sub-object.
///
/// The body first runs the intrusive-list destructor `0x80229030` on `this`
/// (which unlinks every node), then frees the storage when `flag` is positive.
///
/// # Original
/// - Ghidra: `FUN_80277870` at `0x80277870` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277870_FUN_80277870.c`
/// - observed: `if ((param_1 != 0) && (FUN_80229030(param_1,0), 0 < param_2))
///   { FUN_802b9280(param_1); }` followed by `return param_1;`
///
/// # Arguments
/// - `this`: list object.
/// - `flag`: positive to free the storage.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - unlinks every node, then frees `this` when `flag > 0`.
///
/// # Safety
/// `this` must be null or a live intrusive list.
///
/// # Confidence
/// high - the comma expression fixes the order of the two calls.
pub unsafe fn fn_80277870(this: usize, flag: i32) -> usize {
    if this != 0 {
        platform::call2(FUN_80229030, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 0x802778F0
// ===========================================================================

/// Deleting destructor of the second registry-list sub-object.
///
/// Byte-identical to [`fn_80277870`]; the object is the `+0x6C` registry's
/// node list.
///
/// # Original
/// - Ghidra: `FUN_802778f0` at `0x802778F0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802778f0_FUN_802778f0.c`
/// - observed: identical guard, `FUN_80229030(param_1,0)` and
///   `FUN_802b9280(param_1)` calls.
///
/// # Arguments
/// - `this`: list object.
/// - `flag`: positive to free the storage.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - unlinks every node, then frees `this` when `flag > 0`.
///
/// # Safety
/// `this` must be null or a live intrusive list.
///
/// # Confidence
/// high - the comma expression fixes the order of the two calls.
pub unsafe fn fn_802778f0(this: usize, flag: i32) -> usize {
    if this != 0 {
        platform::call2(FUN_80229030, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 0x80277970
// ===========================================================================

/// Deleting destructor of the third registry-list sub-object.
///
/// Byte-identical to [`fn_80277870`]; the object is the `+0x94` registry's
/// node list.
///
/// # Original
/// - Ghidra: `FUN_80277970` at `0x80277970` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277970_FUN_80277970.c`
/// - observed: identical guard, `FUN_80229030(param_1,0)` and
///   `FUN_802b9280(param_1)` calls.
///
/// # Arguments
/// - `this`: list object.
/// - `flag`: positive to free the storage.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - unlinks every node, then frees `this` when `flag > 0`.
///
/// # Safety
/// `this` must be null or a live intrusive list.
///
/// # Confidence
/// high - the comma expression fixes the order of the two calls.
pub unsafe fn fn_80277970(this: usize, flag: i32) -> usize {
    if this != 0 {
        platform::call2(FUN_80229030, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 0x802779D0
// ===========================================================================

/// Destroys the sound-system object built by [`fn_802776b0`].
///
/// The two interface vtables are re-installed first, the object is removed from
/// the global sound-system list (`0x80268C60` / `0x80229190`), then the three
/// registry-list objects at `+0x98`, `+0x70`, `+0x48` are destroyed and the
/// storage is freed when `flag` is positive.
///
/// # Original
/// - Ghidra: `FUN_802779d0` at `0x802779D0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802779d0_FUN_802779d0.c`
/// - observed: the two vtable stores at `+8`/`+0xC`, the
///   `FUN_80268c60()`/`thunk_FUN_80229190(owner, param_1)` pair, the three
///   guarded `FUN_80229030(param_1 + 0x98 / + 0x70 / + 0x48, 0)` calls and the
///   `0 < param_2` free.
/// - `// inferred:` the `param_1 != -0x94` / `-0x6C` / `-0x44` tests are the
///   compiler's null check for `param_1 + 0x98` / `+0x70` / `+0x48`.
///
/// # Arguments
/// - `this`: sound-system object.
/// - `flag`: positive to free the storage.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - unregisters the object, destroys three list objects, frees `this`.
///
/// # Safety
/// `this` must be null or the object built by [`fn_802776b0`].
///
/// # Confidence
/// high - every guard, store and call is literal.
pub unsafe fn fn_802779d0(this: usize, flag: i32) -> usize {
    if this != 0 {
        platform::write_u32(this + 8, VT_SOUND_SYS_A);
        platform::write_u32(this + 0x0c, VT_SOUND_SYS_B);

        let owner = call0_r(FUN_80268C60);
        platform::call2(FUN_80229190, owner, this as u32);

        if this != (-0x94isize) as usize && this.wrapping_add(0x98) != 0 {
            platform::call2(FUN_80229030, this.wrapping_add(0x98) as u32, 0);
        }
        if this != (-0x6cisize) as usize && this.wrapping_add(0x70) != 0 {
            platform::call2(FUN_80229030, this.wrapping_add(0x70) as u32, 0);
        }
        if this != (-0x44isize) as usize && this.wrapping_add(0x48) != 0 {
            platform::call2(FUN_80229030, this.wrapping_add(0x48) as u32, 0);
        }

        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this
}

// ===========================================================================
// 0x80277A90
// ===========================================================================

/// Reports whether the sound system currently has an archive handle loaded.
///
/// `this + 0x10` is the archive handle (the same slot [`fn_80277e90`] stores);
/// the answer is the handle's `+0x04` state test at `0x802756B0`.
///
/// # Original
/// - Ghidra: `FUN_80277a90` at `0x80277A90` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277a90_FUN_80277a90.c`
/// - observed: `if (*(int *)(param_1 + 0x10) == 0) return 0;` then
///   `uVar1 = FUN_802756b0(); return uVar1;`
/// - `// inferred:` the argument of `0x802756B0` is the handle just loaded;
///   Ghidra dropped it, and `this + 0x10` is the only live pointer at the
///   call.
///
/// # Arguments
/// - `this`: sound-system object.
///
/// # Returns
/// `0` without a handle, otherwise the handle's `+0x04` non-zero test.
///
/// # Safety
/// `this + 0x10` must be null or a live archive handle.
///
/// # Confidence
/// medium - the guard is literal; the forwarded pointer is inferred.
pub unsafe fn fn_80277a90(this: usize) -> u32 {
    let handle = platform::read_u32(this + 0x10);
    if handle == 0 {
        return 0;
    }
    platform::call1(FUN_802756B0, handle)
}

// ===========================================================================
// 0x80277AB0
// ===========================================================================

/// Builds the sound system's archive tables and arms the +0xBC allocator head.
///
/// The routine first checks the global "system up" flag, then lets
/// [`fn_80277e90`] carve the tables out of the caller's buffer. A second
/// size pass through `0x802757D0` reads the seven-word info header; when the
/// caller's budget `param_6` covers `word[4] * 0xA000` bytes, the
/// `+0xBC` cursor block is initialised with the caller's buffer and block size,
/// and system `+0x30` is pointed at that block.
///
/// # Original
/// - Ghidra: `FUN_80277ab0` at `0x80277AB0` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277ab0_FUN_80277ab0.c`
/// - observed: the `FUN_8027b510()` flag test, the `FUN_80277e90(iVar2,uVar4,
///   param_3,param_4)` call, the two `FUN_802757d0` calls, the
///   `param_6 < (uint)(iVar3 * 0xa000)` test, the
///   `FUN_8027bbf0(iVar2 + 200,param_5,param_6,uVar4)` call, the
///   `*(int *)(iVar2 + 0x30) = iVar2 + 0xbc` store and the
///   `FUN_802dcdbc(uVar4)` relay.
/// - `// inferred:` the packed `FUN_802dcd70()` at the top is the incoming
///   `r3`/`r4` pair (system, archive handle), not a real call.
///
/// # Arguments
/// - `this`: sound-system object (`+0x30` cursor block, `+0xBC` allocator).
/// - `archive`: archive handle passed to the table builder.
/// - `cursor`: first byte of the table area.
/// - `size`: table area length.
/// - `buffer`: buffer handed to the `+0xBC` cursor block.
/// - `budget`: byte budget checked against `word[4] * 0xA000`.
///
/// # Returns
/// `1` when the tables were built and armed, `0` otherwise.
///
/// # Side effects
/// - writes the system's table pointers and `+0x30`,
/// - initialises the `+0xC8` cursor block.
///
/// # Safety
/// The system, archive and buffer must be live; `cursor..cursor+size` must be
/// writable.
///
/// # Confidence
/// medium - the flow is literal; the seven-word index (`word[4]`) and the
/// buffer roles are inferred from [`fn_80277cf0`] and [`fn_80277e90`].
pub unsafe fn fn_80277ab0(
    this: usize,
    archive: usize,
    cursor: u32,
    size: u32,
    buffer: u32,
    budget: u32,
) -> u32 {
    if call0_r(FUN_8027B510) == 0 {
        return 0;
    }
    if fn_80277e90(this, archive, cursor, size) == 0 {
        return 0;
    }

    let mut info = [0u32; 7];
    let mut needed = 0u32;
    if platform::call2(FUN_802757D0, archive as u32, info.as_mut_ptr() as u32) != 0 {
        needed = info[4];
    }
    let needed = needed.wrapping_mul(0xa000);
    if budget < needed {
        return 0;
    }

    let mut info2 = [0u32; 7];
    let mut block = 0u32;
    if platform::call2(FUN_802757D0, archive as u32, info2.as_mut_ptr() as u32) != 0 {
        block = info2[4];
    }
    platform::call4(FUN_8027BBF0, (this + 0xc8) as u32, buffer, budget, block);
    platform::write_u32(this + 0x30, (this + 0xbc) as u32);
    1
}

// ===========================================================================
// 0x80277BC0
// ===========================================================================

/// Clears the sound system's archive tables and releases its bank entries.
///
/// Every entry of the `+0x3C`-count array at `+0x40` is released with
/// `0x8027A670(entry, -1)`, then the array is dropped. When the `+0xE4`/`+0xE8`
/// block is present, the three registry lists (`+0x44`, `+0x6C`, `+0x94`) are
/// pruned of nodes pointing into that block and the `+0xBC` list is pruned too,
/// each under its own mutex. Finally the `+0xC8` cursor block is zeroed.
///
/// # Original
/// - Ghidra: `FUN_80277bc0` at `0x80277BC0` (304 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277bc0_FUN_80277bc0.c`
/// - observed: the five `iVar1 + 0x10..0x30` clears, the
///   `uVar3 < *(uint *)(iVar1 + 0x3c)` loop with `FUN_8027a670(*(int *)(iVar1
///   + 0x40) + iVar2,0xffffffff)` and `iVar2 += 100`, the `+0x3c`/`+0x40`
///   clears, the `FUN_8027bc70(iVar1 + 200)` call, the three
///   `FUN_801b2e60`/`FUN_8026d9b0`/`FUN_801b2f40` triples, the
///   `FUN_8026f630(iVar1 + 0xbc,...)` call, and finally `FUN_802dcdc0()`.
/// - `// inferred:` the object pointer is the incoming `r3`; the export takes
///   no parameters because of the register-save prologue.
///
/// # Arguments
/// - `this`: sound-system object.
///
/// # Side effects
/// - frees up to `+0x3C` bank entries,
/// - unlinks registry nodes that point into `[+0xE4, +0xE4 + +0xE8)`,
/// - zeroes the `+0xC8` cursor block and the `+0x10..0x1C`, `+0x30`, `+0x3C`,
///   `+0x40` fields.
///
/// # Safety
/// `this` must be the live object built by [`fn_802776b0`].
///
/// # Confidence
/// medium - every loop, store and call is literal; the registry meanings come
/// from [`fn_80278290`] and [`fn_802783c0`], which fill the same offsets.
pub unsafe fn fn_80277bc0(this: usize) {
    platform::write_u32(this + 0x10, 0);
    platform::write_u32(this + 0x14, 0);
    platform::write_u32(this + 0x18, 0);
    platform::write_u32(this + 0x1c, 0);
    platform::write_u32(this + 0x30, 0);

    let count = platform::read_u32(this + 0x3c);
    let mut offset = 0usize;
    let mut index = 0u32;
    while index < count {
        let entry = platform::read_u32(this + 0x40) as usize + offset;
        platform::call2(FUN_8027A670, entry as u32, 0xffff_ffff);
        offset += 100;
        index += 1;
    }
    platform::write_u32(this + 0x3c, 0);
    platform::write_u32(this + 0x40, 0);

    platform::call1(FUN_8027BC70, (this + 200) as u32);

    let block = platform::read_u32(this + 0xe4);
    if block != 0 {
        let block_size = platform::read_u32(this + 0xe8);

        platform::call1(MUTEX_LOCK, (this + 0x54) as u32);
        platform::call3(FUN_8026D9B0, (this + 0x44) as u32, block, block_size);
        platform::call1(MUTEX_UNLOCK, (this + 0x54) as u32);

        platform::call1(MUTEX_LOCK, (this + 0x7c) as u32);
        platform::call3(FUN_8026D9B0, (this + 0x6c) as u32, block, block_size);
        platform::call1(MUTEX_UNLOCK, (this + 0x7c) as u32);

        platform::call1(MUTEX_LOCK, (this + 0xa4) as u32);
        platform::call3(FUN_8026D9B0, (this + 0x94) as u32, block, block_size);
        platform::call1(MUTEX_UNLOCK, (this + 0xa4) as u32);

        platform::call3(FUN_8026F630, (this + 0xbc) as u32, block, block_size);
        platform::write_u32(this + 0xe4, 0);
        platform::write_u32(this + 0xe8, 0);
    }
}

// ===========================================================================
// 0x80277CF0
// ===========================================================================

/// Computes the byte size of the archive tables the sound system needs.
///
/// The size is the sum of three parts: `0x64` bytes per `+0x00` table entry
/// plus the `0x08`-byte id array; the per-id `0x64`-byte records plus their
/// `0x08`-byte sub-records; and the four table blocks scaled by their counts
/// in the seven-word info header (`0x4B4`, `0x10B4`, `0x234`, `0xD0`).
///
/// # Original
/// - Ghidra: `FUN_80277cf0` at `0x80277CF0` (336 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277cf0_FUN_80277cf0.c`
/// - observed: `FUN_80275700(param_2) * 100`, the `FUN_802757c0` loop with
///   `(iVar3 + 0x3fU & 0xffffffe0) + (local_34 + 3U & 0xfffffffc)`,
///   `FUN_80275710(param_2) * 8 + 4`, the `FUN_802757d0` guard and the
///   `local_30 * 0x4b4 + local_28 * 0x10b4 + local_1c * 0x234 +
///   local_2c * 0xd0` sum, then `FUN_80275820(param_2) * 8 + 4`.
///
/// # Arguments
/// - `archive`: archive handle.
/// - `_unused`: unused by the original body.
///
/// # Returns
/// Required table bytes (`i32` in the export).
///
/// # Safety
/// `archive` must be a live handle whose `0x802757C0`/`0x802757D0` lookups
/// succeed.
///
/// # Confidence
/// medium - every term is literal; the block meanings are inferred from
/// [`fn_80277e90`], which allocates exactly these blocks.
pub unsafe fn fn_80277cf0(archive: usize, _unused: u32) -> i32 {
    let count = platform::call1(FN_80275700, archive as u32);
    let mut total = count.wrapping_mul(100) as i32;

    let mut index = 0u32;
    while index < count {
        let mut pair = [0u32; 2];
        if platform::call3(FN_802757C0, archive as u32, index, pair.as_mut_ptr() as u32) != 0 {
            let mut entries = pair[0] as i32;
            while entries > 0 {
                if pair[1] != 0 {
                    total = (((total as u32).wrapping_add(0x3f)) & 0xffff_ffe0)
                        .wrapping_add((pair[1].wrapping_add(3)) & 0xffff_fffc)
                        as i32;
                }
                entries -= 1;
            }
        }
        index += 1;
    }

    let sub = platform::call1(FN_80275710, archive as u32);
    total = total
        .wrapping_add(sub.wrapping_mul(8) as i32)
        .wrapping_add(4);

    let mut info = [0u32; 7];
    if platform::call2(FUN_802757D0, archive as u32, info.as_mut_ptr() as u32) != 0 {
        total = total
            .wrapping_add(info[0].wrapping_mul(0x4b4) as i32)
            .wrapping_add(info[2].wrapping_mul(0x10b4) as i32)
            .wrapping_add(info[5].wrapping_mul(0x234) as i32)
            .wrapping_add(info[1].wrapping_mul(0xd0) as i32);
    }

    let tail = platform::call1(FN_80275820, archive as u32);
    total
        .wrapping_add(tail.wrapping_mul(8) as i32)
        .wrapping_add(4)
}

// ===========================================================================
// 0x80277E40
// ===========================================================================

/// Returns the byte size of the archive's `0xA000`-byte block.
///
/// The seven-word info header's word 4 holds the block count.
///
/// # Original
/// - Ghidra: `FUN_80277e40` at `0x80277E40` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277e40_FUN_80277e40.c`
/// - observed: the `FUN_802757d0(param_2,auStack_28)` guard, the
///   `local_18` read and `iVar2 * 0xa000`.
/// - `// inferred:` `local_18` is word 4 of the seven-word block, the same
///   element [`fn_80277ab0`] compares against its budget.
///
/// # Arguments
/// - `_unused`: unused by the original body.
/// - `archive`: archive handle.
///
/// # Returns
/// `word[4] * 0xA000`, or `0` when the header lookup fails.
///
/// # Safety
/// `archive` must be a live handle.
///
/// # Confidence
/// medium - the arithmetic is literal; the word index is inferred from the
/// block layout used by the allocator.
pub unsafe fn fn_80277e40(_unused: u32, archive: usize) -> i32 {
    let mut info = [0u32; 7];
    let mut count = 0u32;
    if platform::call2(FUN_802757D0, archive as u32, info.as_mut_ptr() as u32) != 0 {
        count = info[4];
    }
    count.wrapping_mul(0xa000) as i32
}

// ===========================================================================
// 0x80277E90
// ===========================================================================

/// Carves the sound system's four archive tables out of the caller's buffer.
///
/// The first pass ([`fn_80278290`]) lays down the bank slots and the
/// `0x64`-byte entry records. This function then writes the id arrays at
/// `+0x14` and `+0x18` (count word followed by two zeroed words per entry),
/// copies the four table blocks with `0x8026D860` under their mutexes and
/// primes the `+0xBC` cursor block. Every block is bounds-checked against
/// `cursor + size`; a failed check returns `0` without publishing the tables.
///
/// # Original
/// - Ghidra: `FUN_80277e90` at `0x80277E90` (816 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277e90_FUN_80277e90.c`
/// - observed: `FUN_80278290(iVar2,uVar4,&local_48,iVar9)`, the two
///   `(x + 7) & 0xfffffffc` size checks with the
///   `**(undefined4 **)(iVar2 + 0x14) = uVar5` count store and the
///   `+ iVar3 + 4` / `+ 8` zeroing loop, the same pair for `+0x18`, the
///   `local_44 * 0x4b4`, `local_3c * 0x10b4`, `local_30 * 0x234`,
///   `local_40 * 0xd0` sums with their three
///   `FUN_801b2e60`/`FUN_8026d860`/`FUN_801b2f40` triples, the
///   `FUN_8026f620(iVar2 + 0xbc)` call, and the `+0x10`, `+0xE4`, `+0xE8`
///   stores before `FUN_802dcdb0(uVar4)`.
/// - `// inferred:` the object and handle are the incoming `r3`/`r4`;
///   `FUN_8026f620`'s two argument registers are dropped by Ghidra, so the
///   body forwards the cursor and the `0xD0` block size, the only live values.
///
/// # Arguments
/// - `this`: sound-system object.
/// - `archive`: archive handle.
/// - `cursor`: first byte of the table area.
/// - `size`: table area length.
///
/// # Returns
/// `1` when the tables fit and were written, `0` otherwise.
///
/// # Side effects
/// - writes the tables into `cursor..cursor+size`,
/// - publishes `this + 0x14`, `+0x18`, `+0x10`, `+0xE4`, `+0xE8`.
///
/// # Safety
/// `this` must be the live sound-system object, `archive` a live handle and
/// `cursor..cursor+size` writable.
///
/// # Confidence
/// medium - all checks and stores are literal; the two dropped arguments and
/// the block roles are inferred.
pub unsafe fn fn_80277e90(this: usize, archive: usize, cursor: u32, size: u32) -> u32 {
    let limit = cursor.wrapping_add(size);
    let mut area = cursor;

    if fn_80278290(this, archive, &mut area as *mut u32, limit) == 0 {
        return 0;
    }

    let count = platform::call1(FN_80275710, archive as u32);
    let next = count.wrapping_mul(8).wrapping_add(area).wrapping_add(7) & 0xffff_fffc;
    if (next as i32) - (limit as i32) > 0 {
        return 0;
    }
    platform::write_u32(this + 0x14, area);
    area = next;
    let written = platform::call1(FN_80275710, archive as u32);
    let ids = platform::read_u32(this + 0x14) as usize;
    platform::write_u32(ids, written);
    let mut offset = 0usize;
    let mut index = 0u32;
    while index < platform::read_u32(ids) {
        platform::write_u32(ids + offset + 4, 0);
        platform::write_u32(ids + offset + 8, 0);
        offset += 8;
        index += 1;
    }

    let count = platform::call1(FN_80275820, archive as u32);
    let next = count.wrapping_mul(8).wrapping_add(area).wrapping_add(7) & 0xffff_fffc;
    if (next as i32) - (limit as i32) > 0 {
        return 0;
    }
    platform::write_u32(this + 0x18, area);
    area = next;
    let written = platform::call1(FN_80275820, archive as u32);
    let ids = platform::read_u32(this + 0x18) as usize;
    platform::write_u32(ids, written);
    let mut offset = 0usize;
    let mut index = 0u32;
    while index < platform::read_u32(ids) {
        platform::write_u32(ids + offset + 4, 0);
        platform::write_u32(ids + offset + 8, 0);
        offset += 8;
        index += 1;
    }

    let mut info = [0u32; 7];
    if platform::call2(FUN_802757D0, archive as u32, info.as_mut_ptr() as u32) != 0 {
        let source = area;

        let next = info[0]
            .wrapping_mul(0x4b4)
            .wrapping_add(area)
            .wrapping_add(3)
            & 0xffff_fffc;
        if (next as i32) - (limit as i32) > 0 {
            return 0;
        }
        platform::call1(MUTEX_LOCK, (this + 0x54) as u32);
        platform::call4(
            FUN_8026D860,
            (this + 0x44) as u32,
            source,
            info[0].wrapping_mul(0x4b4),
            0x4b4,
        );
        platform::call1(MUTEX_UNLOCK, (this + 0x54) as u32);

        let after_banks = info[2].wrapping_mul(0x10b4).wrapping_add(next);
        if (after_banks as i32) - (limit as i32) > 0 {
            return 0;
        }
        platform::call1(MUTEX_LOCK, (this + 0x7c) as u32);
        platform::call4(
            FUN_8026D860,
            (this + 0x6c) as u32,
            next,
            info[2].wrapping_mul(0x10b4),
            0x10b4,
        );
        platform::call1(MUTEX_UNLOCK, (this + 0x7c) as u32);

        let after_waves = info[5].wrapping_mul(0x234).wrapping_add(after_banks);
        if (after_waves as i32) - (limit as i32) > 0 {
            return 0;
        }
        platform::call1(MUTEX_LOCK, (this + 0xa4) as u32);
        platform::call4(
            FUN_8026D860,
            (this + 0x94) as u32,
            after_banks,
            info[5].wrapping_mul(0x234),
            0x234,
        );
        platform::call1(MUTEX_UNLOCK, (this + 0xa4) as u32);

        area = after_waves;
        if (info[1].wrapping_mul(0xd0).wrapping_add(after_waves) as i32) - (limit as i32) > 0 {
            return 0;
        }
        let block = info[1].wrapping_mul(0xd0);
        platform::call3(FUN_8026F620, (this + 0xbc) as u32, area, block);
    }

    platform::write_u32(this + 0x10, archive as u32);
    platform::write_u32(this + 0xe4, cursor);
    platform::write_u32(this + 0xe8, size);
    1
}

// ===========================================================================
// 0x802781C0
// ===========================================================================

/// Allocates and initialises one bank-scan descriptor from a moving cursor.
///
/// The block is laid out `[0x20-byte header][size bytes]`, 32- and 4-byte
/// aligned. `0x8026FF00` installs the descriptor vtable `0x8034EF88` and
/// `0x80270000` records the aligned cursor, end and size. The descriptor's
/// address is the pre-alignment cursor value.
///
/// # Original
/// - Ghidra: `FUN_802781c0` at `0x802781C0` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802781c0_FUN_802781c0.c`
/// - observed: `uVar1 = uVar4 + 0x23 & 0xfffffffc;`, the
///   `(int)(uVar1 - param_3) < 1` guard, `*param_2 = uVar1`, the
///   `if (uVar4 != 0) uVar2 = FUN_8026ff00(uVar4);`, the
///   `(*param_2 + 0x1f) & 0xffffffe0` alignment, the
///   `(param_4 + uVar1 + 3) & 0xfffffffc` end, and the
///   `FUN_80270000(uVar2,uVar1,param_4)` result test.
///
/// # Arguments
/// - `_unused`: unused by the original body.
/// - `cursor`: in/out cursor word; receives the end of the new block.
/// - `limit`: last usable byte of the arena.
/// - `size`: payload bytes after the 0x20-byte header.
///
/// # Returns
/// The descriptor address, or `0` when the block does not fit.
///
/// # Side effects
/// - advances `*cursor`,
/// - initialises the descriptor at the old cursor.
///
/// # Safety
/// `cursor` must be writable; the arena must extend to `limit`.
///
/// # Confidence
/// high - every alignment, guard and store is literal.
pub unsafe fn fn_802781c0(_unused: u32, cursor: *mut u32, limit: i32, size: i32) -> u32 {
    let start = *cursor;
    let header = start.wrapping_add(0x23) & 0xffff_fffc;
    if (header as i32) - limit > 0 {
        return 0;
    }
    *cursor = header;

    let mut object = 0u32;
    if start != 0 {
        platform::call1(FUN_8026FF00, start);
        object = start;
    }

    let cursor_now = *cursor;
    let aligned = cursor_now.wrapping_add(0x1f) & 0xffff_ffe0;
    *cursor = aligned;
    let end = (size as u32).wrapping_add(aligned).wrapping_add(3) & 0xffff_fffc;
    if (end as i32) - limit > 0 {
        return 0;
    }
    *cursor = end;

    if platform::call3(FUN_80270000, object, aligned, size as u32) == 0 {
        object = 0;
    }
    object
}

// ===========================================================================
// 0x80278290
// ===========================================================================

/// Carves the bank-slot array and the per-id records out of the arena.
///
/// `0x80275700` gives the bank count; the array is `count * 0x64` bytes and is
/// published at `this + 0x40` with its count at `+0x3C`. For every bank,
/// [`fn_8027a560`] constructs the slot, `0x802757C0` supplies the record count
/// and record size, `0x8027AE20` sets the slot cursor, and one descriptor per
/// record is allocated by [`fn_802781c0`] and linked with `0x8027AFA0`.
///
/// # Original
/// - Ghidra: `FUN_80278290` at `0x80278290` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80278290_FUN_80278290.c`
/// - observed: `FUN_80275700((int)uVar8)`, the
///   `uVar2 * 100 + uVar7 + 3 & 0xfffffffc` size, the `> 0x40`/`> 0x3C`
///   stores, the `uVar2` loop with `FUN_8027a560(uVar7)`, `FUN_802757c0`,
///   `FUN_8027ae20(uVar3,local_38)`, the nested
///   `FUN_802781c0(iVar1,param_3,param_4,local_34)` /
///   `FUN_8027afa0(uVar3,iVar5)` loop, `FUN_8027aeb0(uVar3)` and the
///   `goto LAB_802783a4` / `FUN_802dcdb0(uVar3)` epilogue.
/// - `// inferred:` the two packed values are the incoming `r3` (system) and
///   `r4` (archive handle).
///
/// # Arguments
/// - `this`: sound-system object (`+0x3C` count, `+0x40` array).
/// - `archive`: archive handle.
/// - `area`: in/out arena cursor.
/// - `limit`: last usable byte of the arena.
///
/// # Returns
/// `1` when every slot was built, `0` when a descriptor did not fit.
///
/// # Side effects
/// - publishes `this + 0x3C`/`+0x40`,
/// - writes bank slots and descriptors into the arena and advances `*area`.
///
/// # Safety
/// The arena must be writable to `limit`; the archive handles must be live.
///
/// # Confidence
/// medium - every call and bound is literal; `local_38`/`local_34` are read as
/// a record count/size pair, matching [`fn_802781c0`]'s arguments.
pub unsafe fn fn_80278290(this: usize, archive: usize, area: *mut u32, limit: u32) -> u32 {
    let count = platform::call1(FN_80275700, archive as u32);
    let cursor = *area;
    let next = count
        .wrapping_mul(100)
        .wrapping_add(cursor)
        .wrapping_add(3)
        & 0xffff_fffc;
    if (next as i32) - (limit as i32) > 0 {
        return 0;
    }
    *area = next;
    platform::write_u32(this + 0x40, cursor);
    platform::write_u32(this + 0x3c, count);

    let mut slot = cursor;
    let mut index = 0u32;
    while index < count {
        let mut entry = 0u32;
        if slot != 0 {
            platform::call1(FUN_8027A560, slot);
            entry = slot;
        }

        let mut pair = [0u32; 2];
        if platform::call3(FN_802757C0, archive as u32, index, pair.as_mut_ptr() as u32) != 0 {
            platform::call2(FUN_8027AE20, entry, pair[0]);
            if pair[1] != 0 {
                let mut record = 0u32;
                while record < pair[0] {
                    let object = fn_802781c0(this as u32, area, limit as i32, pair[1] as i32);
                    if object == 0 {
                        return 0;
                    }
                    platform::call2(FUN_8027AFA0, entry, object);
                    record += 1;
                }
                platform::call1(FUN_8027AEB0, entry);
            }
        }

        slot = slot.wrapping_add(100);
        index += 1;
    }
    1
}

// ===========================================================================
// 0x80278780
// ===========================================================================

/// Returns the address of one 100-byte bank slot.
///
/// # Original
/// - Ghidra: `FUN_80278780` at `0x80278780` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80278780_FUN_80278780.c`
/// - observed: `return *(int *)(param_1 + 0x40) + param_2 * 100;`
///
/// # Arguments
/// - `this`: sound-system object (`+0x40` slot array).
/// - `index`: slot index.
///
/// # Returns
/// `this->array + index * 100`.
///
/// # Safety
/// `this + 0x40` must hold the array published by [`fn_80278290`].
///
/// # Confidence
/// high - one load, one multiply and one add.
pub unsafe fn fn_80278780(this: usize, index: i32) -> i32 {
    platform::read_u32(this + 0x40) as i32 + index * 100
}

// ===========================================================================
// 0x80278790
// ===========================================================================

/// Looks up a bank-group offset by id, first through two caches and then
/// through the archive tables.
///
/// The bridge object at `+0x1C`, when present, may answer through its
/// `vtable[0x08]` slot; the archive's `vtable[0x0C]` slot is consulted next.
/// Both of those return values become the function's result when non-zero.
/// Otherwise the `+0x18` table's first column is tried; a miss falls back to
/// walking the sub-table of the entry (`0x80275830`/`0x80275840`) and adding
/// the third word of its five-word record to the `+0x14` table's first column.
///
/// # Original
/// - Ghidra: `FUN_80278790` at `0x80278790` (400 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80278790_FUN_80278790.c`
/// - observed: the `*(int **)(param_1 + 0x1c)`/`+ 8` call, the
///   `*(int **)(param_1 + 0x10)`/`+ 0xc` call, the `param_2 < *puVar2` bound
///   with the `param_2 * 2 + 1` column, the `FUN_80275830` four-word record,
///   the `uVar1 < local_34` loop, `FUN_80275840`, the second `+0x14` table
///   with the same column, `FUN_80275810` and the `uVar4 + local_2c` return.
///
/// # Arguments
/// - `this`: sound-system object (`+0x10` archive, `+0x14` sub-table,
///   `+0x18` table, `+0x1C` bridge).
/// - `id`: bank-group id.
///
/// # Returns
/// A cached table value, the bridge/archive result, or `0`.
///
/// # Safety
/// All four slots must be null or live objects with the vtables above.
///
/// # Confidence
/// medium - the logic is literal; the bridge's argument and the meaning of the
/// two table columns are inferred from [`fn_80278920`]'s twin.
pub unsafe fn fn_80278790(this: usize, id: u32) -> u32 {
    let bridge = platform::read_u32(this + 0x1c) as usize;
    if bridge != 0 {
        let vtable = platform::read_u32(bridge) as usize;
        let handled = platform::call1(platform::read_u32(vtable + 8) as usize, bridge as u32);
        if handled != 0 {
            return handled;
        }
    }

    let archive = platform::read_u32(this + 0x10) as usize;
    let vtable = platform::read_u32(archive) as usize;
    let probe = platform::call2(platform::read_u32(vtable + 0x0c) as usize, archive as u32, id);
    if probe != 0 {
        return probe;
    }

    let table = platform::read_u32(this + 0x18) as usize;
    let mut result = 0u32;
    if table != 0 && id < platform::read_u32(table) {
        result = platform::read_u32(table + id as usize * 8 + 4);
    }

    if result == 0 {
        let mut record = [0u32; 4];
        if platform::call3(FN_80275830, archive as u32, id, record.as_mut_ptr() as u32) != 0 {
            let mut index = 0u32;
            while index < record[3] {
                let mut pair = [0u32; 2];
                if platform::call4(
                    FN_80275840,
                    archive as u32,
                    id,
                    index,
                    pair.as_mut_ptr() as u32,
                ) != 0
                {
                    let second = platform::read_u32(this + 0x14) as usize;
                    let mut value = 0u32;
                    if second != 0 && pair[0] < platform::read_u32(second) {
                        value = platform::read_u32(second + pair[0] as usize * 8 + 4);
                    }
                    if value != 0 {
                        let mut sub = [0u32; 4];
                        if platform::call4(
                            FN_80275810,
                            archive as u32,
                            pair[0],
                            pair[1],
                            sub.as_mut_ptr() as u32,
                        ) != 0
                        {
                            return value.wrapping_add(sub[1]);
                        }
                    }
                }
                index += 1;
            }
        }
        result = 0;
    }

    result
}

// ===========================================================================
// 0x80278920
// ===========================================================================

/// Looks up a bank-group offset by id through the second cache column.
///
/// The twin of [`fn_80278790`]: the bridge answers through `vtable[0x0C]`, the
/// archive through `vtable[0x10]`, both table reads use the second column
/// (`+0x08` of each eight-byte slot), and the final value adds the fourth word
/// of the five-word record instead of the second.
///
/// # Original
/// - Ghidra: `FUN_80278920` at `0x80278920` (400 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80278920_FUN_80278920.c`
/// - observed: the same structure as `FUN_80278790` with the `+ 0xc` / `+ 0x10`
///   vtable slots, the `param_2 * 2 + 2` and `local_48 * 2 + 2` columns and the
///   `uVar4 + local_24` return.
///
/// # Arguments
/// - `this`: sound-system object.
/// - `id`: bank-group id.
///
/// # Returns
/// A cached table value, the bridge/archive result, or `0`.
///
/// # Safety
/// All four slots must be null or live objects with the vtables above.
///
/// # Confidence
/// medium - the logic is literal; the column and record-word roles are
/// inferred from the paired [`fn_80278790`].
pub unsafe fn fn_80278920(this: usize, id: u32) -> u32 {
    let bridge = platform::read_u32(this + 0x1c) as usize;
    if bridge != 0 {
        let vtable = platform::read_u32(bridge) as usize;
        let handled = platform::call1(platform::read_u32(vtable + 0x0c) as usize, bridge as u32);
        if handled != 0 {
            return handled;
        }
    }

    let archive = platform::read_u32(this + 0x10) as usize;
    let vtable = platform::read_u32(archive) as usize;
    let probe = platform::call2(platform::read_u32(vtable + 0x10) as usize, archive as u32, id);
    if probe != 0 {
        return probe;
    }

    let table = platform::read_u32(this + 0x18) as usize;
    let mut result = 0u32;
    if table != 0 && id < platform::read_u32(table) {
        result = platform::read_u32(table + id as usize * 8 + 8);
    }

    if result == 0 {
        let mut record = [0u32; 4];
        if platform::call3(FN_80275830, archive as u32, id, record.as_mut_ptr() as u32) != 0 {
            let mut index = 0u32;
            while index < record[3] {
                let mut pair = [0u32; 2];
                if platform::call4(
                    FN_80275840,
                    archive as u32,
                    id,
                    index,
                    pair.as_mut_ptr() as u32,
                ) != 0
                {
                    let second = platform::read_u32(this + 0x14) as usize;
                    let mut value = 0u32;
                    if second != 0 && pair[0] < platform::read_u32(second) {
                        value = platform::read_u32(second + pair[0] as usize * 8 + 8);
                    }
                    if value != 0 {
                        let mut sub = [0u32; 4];
                        if platform::call4(
                            FN_80275810,
                            archive as u32,
                            pair[0],
                            pair[1],
                            sub.as_mut_ptr() as u32,
                        ) != 0
                        {
                            return value.wrapping_add(sub[3]);
                        }
                    }
                }
                index += 1;
            }
        }
        result = 0;
    }

    result
}

// ===========================================================================
// 0x80279D50
// ===========================================================================

/// Reports whether a bank-group id is present in the `+0x14` table.
///
/// The export computes `(u != 0)` with the branchless idiom
/// `(-u | u) >> 31`; the Rust body states the equivalent comparison.
///
/// # Original
/// - Ghidra: `FUN_80279d50` at `0x80279D50` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80279d50_FUN_80279d50.c`
/// - observed: the `*(uint **)(param_1 + 0x14)` load, the
///   `param_2 < *puVar1` bound, the `puVar1[param_2 * 2 + 1]` column and
///   `return (-uVar2 | uVar2) >> 0x1f;`
///
/// # Arguments
/// - `this`: sound-system object (`+0x14` table).
/// - `id`: bank-group id.
///
/// # Returns
/// `1` when the table slot is non-zero, `0` otherwise.
///
/// # Safety
/// `this + 0x14` must be null or a table published by the allocator.
///
/// # Confidence
/// high - the loads and the branchless test are literal.
pub unsafe fn fn_80279d50(this: usize, id: u32) -> u32 {
    let table = platform::read_u32(this + 0x14) as usize;
    let mut value = 0u32;
    if table != 0 && id < platform::read_u32(table) {
        value = platform::read_u32(table + id as usize * 8 + 4);
    }
    (value != 0) as u32
}

// ===========================================================================
// 0x8027A560
// ===========================================================================

/// Constructs one 100-byte bank slot with three self-referential lists.
///
/// Three pairs of words (`+0x04`, `+0x10`, `+0x1C`) are sentinels pointing at
/// their own address, the cursor starts at `1` with limit `0x7FFFFFFF`, and the
/// remaining words receive the two small-data defaults at `0x8042EAC8` and
/// `0x8042EACC`.
///
/// # Original
/// - Ghidra: `FUN_8027a560` at `0x8027A560` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a560_FUN_8027a560.c`
/// - observed: `uVar1 = *(undefined4 *)(unaff_r2 + -0x45b8)`,
///   `uVar2 = *(undefined4 *)(unaff_r2 + -0x45b4)`, the three
///   `param_1 + 1` / `param_1 + 4` / `param_1 + 7` sentinel pairs, the
///   `param_1[9] = 1` / `param_1[10] = 0x7fffffff` cursor and the interleaved
///   `uVar1`/`uVar2` stores through `param_1[0x18]`.
///
/// # Arguments
/// - `this`: 100-byte slot.
///
/// # Side effects
/// - writes all 25 words of the slot.
///
/// # Safety
/// `this` must be 100 writable bytes.
///
/// # Confidence
/// high - every store is literal; `r2` is fixed at `0x80433080`, so the two
/// small-data words are absolute addresses.
pub unsafe fn fn_8027a560(this: usize) {
    let default_a = platform::read_u32(SDA2_BANK_DEFAULT_A);
    let default_b = platform::read_u32(SDA2_BANK_DEFAULT_B);

    platform::write_u32(this, 0);
    platform::write_u32(this + 0x04, this.wrapping_add(4) as u32);
    platform::write_u32(this + 0x08, this.wrapping_add(4) as u32);
    platform::write_u32(this + 0x0c, 0);
    platform::write_u32(this + 0x10, this.wrapping_add(0x10) as u32);
    platform::write_u32(this + 0x14, this.wrapping_add(0x10) as u32);
    platform::write_u32(this + 0x18, 0);
    platform::write_u32(this + 0x1c, this.wrapping_add(0x1c) as u32);
    platform::write_u32(this + 0x20, this.wrapping_add(0x1c) as u32);
    platform::write_u32(this + 0x24, 1);
    platform::write_u32(this + 0x28, 0x7fff_ffff);
    platform::write_u32(this + 0x2c, default_a);
    platform::write_u32(this + 0x30, default_b);
    platform::write_u32(this + 0x34, 1);
    platform::write_u32(this + 0x38, default_a);
    platform::write_u32(this + 0x3c, 0);
    platform::write_u32(this + 0x40, default_b);
    platform::write_u32(this + 0x44, default_a);
    platform::write_u32(this + 0x48, default_a);
    platform::write_u32(this + 0x4c, default_a);
    platform::write_u32(this + 0x50, default_a);
    platform::write_u32(this + 0x54, default_b);
    platform::write_u32(this + 0x58, default_b);
    platform::write_u32(this + 0x5c, default_b);
    platform::write_u32(this + 0x60, default_b);
}

// ===========================================================================
// 0x8027AD90
// ===========================================================================

/// Registers a bank object in two of the sound system's intrusive lists.
///
/// Both insertions happen under the sequence-manager mutex (`+0x354`). The
/// first registers the bank's `+0x100` node in the list at `registry + 0x0C`;
/// the second registers its `+0xF8` node in the list at `registry`, then links
/// the bank to the registry with `0x80267330`.
///
/// # Original
/// - Ghidra: `FUN_8027ad90` at `0x8027AD90` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ad90_FUN_8027ad90.c`
/// - observed: the two `FUN_8027b5f0()` + `FUN_801b2e60(iVar1 + 0x354)` /
///   `FUN_801b2f40(iVar1 + 0x354)` pairs, the
///   `FUN_80229190(param_1 + 0xc,param_2 + 0x100)`,
///   `FUN_80229190(param_1,param_2 + 0xf8)` and
///   `FUN_80267330(param_2,param_1)` calls.
///
/// # Arguments
/// - `registry`: bank registry object (`+0x00` and `+0x0C` lists).
/// - `bank`: bank object (`+0xF8` and `+0x100` nodes).
///
/// # Safety
/// Both objects must be live; the sequence manager must be initialised.
///
/// # Confidence
/// high - both critical sections and all three calls are literal.
pub unsafe fn fn_8027ad90(registry: usize, bank: usize) {
    lock_seq_manager();
    platform::call2(
        FUN_80229190,
        registry.wrapping_add(0x0c) as u32,
        bank.wrapping_add(0x100) as u32,
    );
    unlock_seq_manager();

    lock_seq_manager();
    platform::call2(
        FUN_80229190,
        registry as u32,
        bank.wrapping_add(0xf8) as u32,
    );
    platform::call2(FUN_80267330, bank as u32, registry as u32);
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027AFA0
// ===========================================================================

/// Links one bank-scan descriptor into its bank slot's list.
///
/// The descriptor's `+0x08` word points back at the bank slot and its `+0x18`
/// node is inserted into the list at `slot + 0x18` with `slot + 0x1C` as key,
/// all under the sequence-manager mutex.
///
/// # Original
/// - Ghidra: `FUN_8027afa0` at `0x8027AFA0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027afa0_FUN_8027afa0.c`
/// - observed: the `FUN_8027b5f0`/`FUN_801b2e60(iVar1 + 0x354)` pair, the
///   `*(int *)(param_2 + 8) = param_1` store, `local_18[0] = param_1 + 0x1c`,
///   the `FUN_80229160(param_1 + 0x18,local_18,param_2 + 0x18)` insert and
///   `FUN_801b2f40(iVar1 + 0x354)`.
///
/// # Arguments
/// - `slot`: bank slot built by [`fn_8027a560`].
/// - `descriptor`: scan descriptor built by [`fn_802781c0`].
///
/// # Safety
/// Both objects must be live; the sequence manager must be initialised.
///
/// # Confidence
/// high - the store, the key word and the insert are literal.
pub unsafe fn fn_8027afa0(slot: usize, descriptor: usize) {
    lock_seq_manager();
    platform::write_u32(descriptor + 8, slot as u32);
    let mut key = [slot.wrapping_add(0x1c) as u32];
    platform::call3(
        FUN_80229160,
        slot.wrapping_add(0x18) as u32,
        key.as_mut_ptr() as u32,
        descriptor.wrapping_add(0x18) as u32,
    );
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027A160
// ===========================================================================

/// Hangs a holder off a node's second link (`node + 0x0C`).
///
/// The holder stores the node in its own `+0x00` word and the node stores the
/// holder back at `+0x0C`. If that link already named another holder,
/// `0x80267750` detaches the old one first, so the link never carries two.
///
/// # Original
/// - Ghidra: `FUN_8027a160` at `0x8027A160` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a160_FUN_8027a160.c`
/// - observed: `*param_1 = param_2;`, the `FUN_80267720(param_2)` guard on
///   `node + 0x0C`, the `FUN_80267750(*param_1)` release and the closing
///   `*(int **)(*param_1 + 0xc) = param_1;`.
///
/// # Arguments
/// - `this`: holder object; its `+0x00` word receives `node`.
/// - `node`: shared node whose `+0x0C` link is claimed.
///
/// # Side effects
/// - writes `this + 0x00` and `node + 0x0C`,
/// - may detach the previous holder through `0x80267750`.
///
/// # Safety
/// `node` must be writable at `+0x0C`; `this` must be writable for 4 bytes.
///
/// # Confidence
/// high - the two stores and both call sites are literal.
pub unsafe fn fn_8027a160(this: usize, node: u32) {
    platform::write_u32(this, node);
    let link = platform::read_u32(node as usize + 0x0c);
    if link != 0 {
        platform::call1(FUN_80267750, link);
    }
    platform::write_u32(node as usize + 0x0c, this as u32);
}

// ===========================================================================
// 0x8027A1B0
// ===========================================================================

/// Hangs a holder off a node's first link (`node + 0x08`).
///
/// The mirror image of [`fn_8027a160`]: this one uses the `+0x08` link and the
/// `0x80267700`/`0x80267740` pair, which test and detach that link.
///
/// # Original
/// - Ghidra: `FUN_8027a1b0` at `0x8027A1B0` (76 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a1b0_FUN_8027a1b0.c`
/// - observed: `*param_1 = param_2;`, `FUN_80267700(param_2)`,
///   `FUN_80267740(*param_1)` and `*(int **)(*param_1 + 8) = param_1;`.
///
/// # Arguments
/// - `this`: holder object; its `+0x00` word receives `node`.
/// - `node`: shared node whose `+0x08` link is claimed.
///
/// # Side effects
/// - writes `this + 0x00` and `node + 0x08`,
/// - may detach the previous holder through `0x80267740`.
///
/// # Safety
/// `node` must be writable at `+0x08`; `this` must be writable for 4 bytes.
///
/// # Confidence
/// high - the two stores and both call sites are literal.
pub unsafe fn fn_8027a1b0(this: usize, node: u32) {
    platform::write_u32(this, node);
    let link = platform::read_u32(node as usize + 0x08);
    if link != 0 {
        platform::call1(FUN_80267740, link);
    }
    platform::write_u32(node as usize + 0x08, this as u32);
}

// ===========================================================================
// 0x8027A200
// ===========================================================================

/// Detaches a holder from its node.
///
/// Both links of the node are cleared when they point back at this holder, and
/// then the holder's own `+0x00` word is zeroed. The export re-reads the
/// holder's word before the last store, so it is read twice here as well.
///
/// # Original
/// - Ghidra: `FUN_8027a200` at `0x8027A200` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a200_FUN_8027a200.c`
/// - observed: the `node + 8` and `node + 0xc` `== param_1` comparisons with
///   their zeroing stores, then `if (*param_1 == 0) return; *param_1 = 0;`.
///
/// # Arguments
/// - `this`: holder object whose `+0x00` word names the node.
///
/// # Side effects
/// - clears `node + 0x08` and `node + 0x0C` when they name this holder,
/// - clears `this + 0x00`.
///
/// # Safety
/// When `this + 0x00` is non-zero the node must be writable at `+0x08..+0x0C`.
///
/// # Confidence
/// high - the guards and the three stores are literal.
pub unsafe fn fn_8027a200(this: usize) {
    let node = platform::read_u32(this);
    if node != 0 {
        if platform::read_u32(node as usize + 8) == this as u32 {
            platform::write_u32(node as usize + 8, 0);
        }
        if platform::read_u32(node as usize + 0x0c) == this as u32 {
            platform::write_u32(node as usize + 0x0c, 0);
        }
    }
    if platform::read_u32(this) == 0 {
        return;
    }
    platform::write_u32(this, 0);
}

// ===========================================================================
// 0x8027A250
// ===========================================================================

/// Constructs the variable-size record pool.
///
/// The pool is a container with a vtable at `+0x00`, a mutex header at `+0x04`
/// and a list header at `+0x1C`. [`fn_8027a3a0`], [`fn_8027a410`] and
/// [`fn_8027a470`] are its allocator, appender and truncator, all of which take
/// that `+0x04` mutex.
///
/// # Original
/// - Ghidra: `FUN_8027a250` at `0x8027A250` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a250_FUN_8027a250.c`
/// - observed: `*param_1 = &DAT_8034f148;`, `FUN_80269ee0(param_1 + 7)` (which
///   is `+0x1C`) and `FUN_801b2e20(param_1 + 1)` (which is `+0x04`).
/// - `// inferred:` the pool role comes from `0x8026A2A0`/`0x8026A350`/
///   `0x8026A420`, which this object's users call with the `+0x1C` address.
///
/// # Arguments
/// - `this`: pool object to initialise.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - installs the vtable, initialises the list header and the mutex.
///
/// # Safety
/// `this` must be writable for at least `0x24` bytes.
///
/// # Confidence
/// high - the vtable store, the list initialiser and the mutex initialiser are
/// all literal; only the pool naming is inferred.
pub unsafe fn fn_8027a250(this: usize) -> u32 {
    platform::write_u32(this, VT_SEQ_POOL);
    platform::call1(FUN_80269EE0, (this + 0x1c) as u32);
    platform::call1(MUTEX_INIT, (this + 4) as u32);
    this as u32
}

// ===========================================================================
// 0x8027A2A0
// ===========================================================================

/// Destroys the record pool, optionally freeing it.
///
/// The vtable is reinstalled, the record list is torn down twice (`0x8026A1B0`
/// drops the list, then `0x80269F00` receives the same address with a `-1`
/// flag so it does not free it a second time), and `0x802B9280` frees the
/// object only when `flag` is positive.
///
/// # Original
/// - Ghidra: `FUN_8027a2a0` at `0x8027A2A0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a2a0_FUN_8027a2a0.c`
/// - observed: the `param_1 != 0` guard, `*param_1 = &DAT_8034f148;`,
///   `FUN_8026a1b0(param_1 + 7)`, `FUN_80269f00(param_1 + 7,0xffffffff)` and
///   `if (0 < param_2) FUN_802b9280(param_1);`.
///
/// # Arguments
/// - `this`: pool object to destroy.
/// - `flag`: when positive, free the object after teardown.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - destroys the record list and, when `flag > 0`, calls the allocator's free.
///
/// # Safety
/// `this` must be null or a pool built by [`fn_8027a250`].
///
/// # Confidence
/// high - the two list calls, the `-1` flag and the free guard are literal.
pub unsafe fn fn_8027a2a0(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::write_u32(this, VT_SEQ_POOL);
        platform::call1(FUN_8026A1B0, (this + 0x1c) as u32);
        platform::call2(FUN_80269F00, (this + 0x1c) as u32, 0xffff_ffff);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027A310
// ===========================================================================

/// Destroys the record list of a pool.
///
/// # Original
/// - Ghidra: `FUN_8027a310` at `0x8027A310` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a310_FUN_8027a310.c`
/// - observed: `FUN_8026a010(param_1 + 0x1c);`.
/// - `// unknown:` `0x8026A010` takes two further words (`r4`/`r5`) that the
///   export does not show, so only the visible argument is forwarded.
///
/// # Arguments
/// - `this`: pool object.
///
/// # Safety
/// `this + 0x1C` must be a live list header.
///
/// # Confidence
/// medium - the address and the visible argument are literal; the callee's
/// hidden arguments are not observable in this export.
pub unsafe fn fn_8027a310(this: usize) {
    platform::call1(FUN_8026A010, (this + 0x1c) as u32);
}

// ===========================================================================
// 0x8027A320
// ===========================================================================

/// Destroys the record list of a pool without freeing anything.
///
/// # Original
/// - Ghidra: `FUN_8027a320` at `0x8027A320` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a320_FUN_8027a320.c`
/// - observed: `FUN_8026a1b0(param_1 + 0x1c);` - one call, one argument.
///
/// # Arguments
/// - `this`: pool object.
///
/// # Safety
/// `this + 0x1C` must be a live list header.
///
/// # Confidence
/// high - a single one-argument call.
pub unsafe fn fn_8027a320(this: usize) {
    platform::call1(FUN_8026A1B0, (this + 0x1c) as u32);
}

// ===========================================================================
// 0x8027A330
// ===========================================================================

/// `0x8027A4F0`: record tag installed by [`fn_8027a330`]. The word is the
/// entry address of [`fn_8027a4f0`], the triple re-registration callback.
const W290_SEQ_TAG_8027A4F0: u32 = 0x8027_A4F0;

/// Allocates one variable-size record with the `0x8027A4F0` tag.
///
/// Same lock / allocate / unlock sequence as [`fn_8027a3a0`], but the record's
/// first word is the fixed tag `0x8027A4F0` (the re-registration callback of
/// [`fn_8027a4f0`]) and its second word is `0`, so this is the "plain"
/// variable-size append used by the pool's own callers.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x8027A330`.
/// - Extent: `0x8027A330..0x8027A3A0` (112 bytes, 28 words), ending at the
///   `blr` at `0x8027A39C` flush against [`fn_8027a3a0`].
/// - Reach: one data word, `0x8034F154`, in the pointer list whose previous
///   entry is `0x8027A2A0` and whose next word is the `"< N"` / `"W4R - SND
///   fin..."` string at `0x8034F158`; no `b`/`bl` site in the main image or any
///   REL.
/// - Observed: `9421FFE0 stwu r1,-0x20(r1)`; `mflr r0`; `stw r0,0x24(r1)`;
///   `stw r31,0x1C(r1)`; `addi r31,r3,4`; `stw r30,0x18(r1)`; `mr r30,r4`;
///   `stw r29,0x14(r1)`; `mr r29,r3`; `mr r3,r31`;
///   `4BF38B09 bl 0x801B2E60`; `lis r5,0x8028`; `mr r4,r30`;
///   `addi r3,r29,0x1C`; `li r6,0`; `addi r5,r5,-23312`;
///   `4BFEFF31 bl 0x8026A2A0`; `mr r30,r3`; `mr r3,r31`;
///   `4BF38BC5 bl 0x801B2F40`; `mr r3,r30`; epilogue; `4E800020 blr`.
/// - Polarity: none - the body has no conditional branch.
/// - Decoder note: `lis r5,0x8028` + `addi r5,r5,-23312` (`-0x5B10`) is the
///   **sign-extended** pair that resolves to `0x8027A4F0`, not `0x8028A4F0`;
///   `W290_SEQ_TAG_8027A4F0` records the resolved word.
///
/// # Arguments
/// * `this` - pool object; the mutex is at `+0x04` and the list header at
///   `+0x1C`.
/// * `size` - record size in bytes, forwarded as the allocate call's second
///   argument.
///
/// # Returns
/// The new record address, or `0` when the pool is out of memory.
///
/// # Side effects
/// Takes and releases the `+0x04` mutex and allocates/links one record.
///
/// # Safety
/// `this` must be a live pool and its allocator must be live.
///
/// # Confidence
/// high - the lock pair, the two literal tag words and the returned record are
/// literal.
pub unsafe fn fn_8027a330(this: usize, size: u32) -> u32 {
    platform::call1(MUTEX_LOCK, (this + 4) as u32);
    let record = platform::call4(
        FUN_8026A2A0,
        (this + 0x1c) as u32,
        size,
        W290_SEQ_TAG_8027A4F0,
        0,
    );
    platform::call1(MUTEX_UNLOCK, (this + 4) as u32);
    record
}

// ===========================================================================
// 0x8027A3A0
// ===========================================================================

/// Allocates one variable-size record in the pool.
///
/// `0x8026A2A0` receives the record size, the two caller words, and returns the
/// new record (or null). The pool's `+0x04` mutex is held across the call and
/// released afterwards; the trailing `0x802DCDC0` relay hands the record back
/// in `r3`, which Ghidra prints as a `void` function.
///
/// # Original
/// - Ghidra: `FUN_8027a3a0` at `0x8027A3A0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a3a0_FUN_8027a3a0.c`
/// - observed: `uVar4 = FUN_802dcd74();` whose high word is `param_1` and low
///   word `param_2`, `FUN_801b2e60(iVar1 + 4)`,
///   `FUN_8026a2a0(iVar1 + 0x1c,(int)uVar4,param_3,param_4)`,
///   `FUN_801b2f40(iVar3)` and `FUN_802dcdc0(uVar2);`.
/// - `// inferred:` `0x802DCD74` is the register-save stub, so the "returned
///   pair" is this function's own `r3`/`r4`; the wave-66 note at the top of
///   this module records the same convention.
///
/// # Arguments
/// - `this`: pool object.
/// - `size`: record size in bytes (the relayed `r4`).
/// - `first`: first word stored in the new record.
/// - `second`: second word stored in the new record.
///
/// # Returns
/// The new record address, or `0` when the pool is out of memory.
///
/// # Side effects
/// - takes and releases the mutex at `this + 0x04`,
/// - allocates and links a record.
///
/// # Safety
/// `this` must be a live pool; the allocator the pool holds must be live.
///
/// # Confidence
/// medium - every call and the lock/unlock pair are literal; the size word is
/// identified through the register-save stub rather than an explicit argument.
pub unsafe fn fn_8027a3a0(this: usize, size: u32, first: u32, second: u32) -> u32 {
    platform::call1(MUTEX_LOCK, (this + 4) as u32);
    let record = platform::call4(FUN_8026A2A0, (this + 0x1c) as u32, size, first, second);
    platform::call1(MUTEX_UNLOCK, (this + 4) as u32);
    record
}

// ===========================================================================
// 0x8027A410
// ===========================================================================

/// Appends one zeroed `0x14`-byte record and returns its index.
///
/// `0x8026A350` allocates the record, links it and returns `count - 1`, or `-1`
/// when the allocation failed. The pool mutex is held across the call.
///
/// # Original
/// - Ghidra: `FUN_8027a410` at `0x8027A410` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a410_FUN_8027a410.c`
/// - observed: `FUN_801b2e60(param_1 + 4)`, `FUN_8026a350(param_1 + 0x1c)`,
///   `FUN_801b2f40(param_1 + 4)` and `return uVar1;`.
///
/// # Arguments
/// - `this`: pool object.
///
/// # Returns
/// The new record's index, or `0xFFFFFFFF` on failure.
///
/// # Side effects
/// - takes and releases the mutex at `this + 0x04`,
/// - appends one record.
///
/// # Safety
/// `this` must be a live pool.
///
/// # Confidence
/// high - the lock, the call and the unlock are literal and the result is
/// returned unchanged.
pub unsafe fn fn_8027a410(this: usize) -> u32 {
    platform::call1(MUTEX_LOCK, (this + 4) as u32);
    let index = platform::call1(FUN_8026A350, (this + 0x1c) as u32);
    platform::call1(MUTEX_UNLOCK, (this + 4) as u32);
    index
}

// ===========================================================================
// 0x8027A470
// ===========================================================================

/// Truncates the pool to `count` records.
///
/// Two locks are taken in a fixed order - the pool's `+0x04` mutex first, then
/// the sequence manager's `+0x354` mutex - and both are released in reverse
/// order. The export re-fetches the manager for the unlock, which is reproduced
/// here because the singleton accessor is not assumed to be idempotent.
///
/// # Original
/// - Ghidra: `FUN_8027a470` at `0x8027A470` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a470_FUN_8027a470.c`
/// - observed: `FUN_801b2e60(param_1 + 4)`, `iVar1 = FUN_8027b5f0();`,
///   `FUN_801b2e60(iVar1 + 0x354)`, `FUN_8026a420(param_1 + 0x1c,param_2)`,
///   the second `FUN_8027b5f0()` + `FUN_801b2f40(iVar1 + 0x354)` and the final
///   `FUN_801b2f40(param_1 + 4)`.
///
/// # Arguments
/// - `this`: pool object.
/// - `count`: number of records to keep.
///
/// # Side effects
/// - takes and releases both mutexes,
/// - drops records beyond `count`.
///
/// # Safety
/// `this` must be a live pool and the sequence manager must be initialised.
///
/// # Confidence
/// high - both critical sections and the truncation call are literal.
pub unsafe fn fn_8027a470(this: usize, count: u32) {
    platform::call1(MUTEX_LOCK, (this + 4) as u32);

    let manager = call0_r(SEQ_MANAGER);
    platform::call1(MUTEX_LOCK, manager.wrapping_add(0x354));

    platform::call2(FUN_8026A420, (this + 0x1c) as u32, count);

    let manager = call0_r(SEQ_MANAGER);
    platform::call1(MUTEX_UNLOCK, manager.wrapping_add(0x354));

    platform::call1(MUTEX_UNLOCK, (this + 4) as u32);
}

// ===========================================================================
// 0x8027A4F0
// ===========================================================================

/// Re-registers one triple in both global sound registries.
///
/// `0x80268C60` returns the shared registry owner. The same triple is handed to
/// `0x80268D90` and then to `0x80268EC0`, so both tables see the identical
/// `(a, b, c)` record. The owner is fetched again for the second call, exactly
/// as the export does.
///
/// # Original
/// - Ghidra: `FUN_8027a4f0` at `0x8027A4F0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a4f0_FUN_8027a4f0.c`
/// - observed: the two `uVar1 = FUN_80268c60();` fetches, the
///   `FUN_80268d90(uVar1,param_1,param_2,param_3)` and
///   `FUN_80268ec0(uVar1,param_1,param_2,param_3)` calls.
/// - `// inferred:` the two callees register the triple in different tables;
///   their bodies are outside this module and were not reconstructed.
///
/// # Arguments
/// - `a`, `b`, `c`: the three words each registry stores.
///
/// # Side effects
/// - writes into both registries owned by `0x80268C60`.
///
/// # Safety
/// Both registries must be initialised; the words describe whatever record the
/// caller owns (their meaning is not visible in this export).
///
/// # Confidence
/// medium - both calls and their argument order are literal; the registry
/// roles are inferred.
pub unsafe fn fn_8027a4f0(a: u32, b: u32, c: u32) {
    let owner = call0_r(FUN_80268C60);
    platform::call4(FUN_80268D90, owner, a, b, c);

    let owner = call0_r(FUN_80268C60);
    platform::call4(FUN_80268EC0, owner, a, b, c);
}

// ===========================================================================
// 0x8027A610
// ===========================================================================

/// Destroys one bank-slot entry, optionally freeing it.
///
/// The entry itself is a list header: `0x80229030(entry, 0)` unlinks every node
/// it holds. The object is freed with `0x802B9280` only when `flag` is positive,
/// the same convention as [`fn_8027a2a0`].
///
/// # Original
/// - Ghidra: `FUN_8027a610` at `0x8027A610` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a610_FUN_8027a610.c`
/// - observed: `if ((param_1 != 0) && (FUN_80229030(param_1,0), 0 < param_2))` and
///   `FUN_802b9280(param_1);` followed by `return param_1;`.
///
/// # Arguments
/// - `this`: entry to destroy.
/// - `flag`: when positive, free the entry after the list teardown.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or a live list header.
///
/// # Confidence
/// high - the guard, the list teardown and the free are literal.
pub unsafe fn fn_8027a610(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::call2(FUN_80229030, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027A670
// ===========================================================================

/// Destroys a bank-slot entry together with its child objects.
///
/// The circular list at `+0x10` is walked under the sequence-manager mutex; each
/// node belongs to an object `0xF8` bytes before the node, and that object is
/// released with `0x80266250(object, 0)`. The three list headers at `+0x10`,
/// `+0x30` and `+0x60` are then torn down with `0x80229030`, and the entry is
/// freed when `flag` is positive.
///
/// # Original
/// - Ghidra: `FUN_8027a670` at `0x8027A670` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a670_FUN_8027a670.c`
/// - observed: `puVar3 = *(undefined4 **)(param_1 + 4);` and the
///   `while (puVar3 != (undefined4 *)(param_1 + 4))` walk with
///   `FUN_80266250(puVar3 + -0x3e,0);`, then the
///   `param_1 + 0x18` / `param_1 + 0xc` / `param_1` `FUN_80229030` calls and
///   `if (0 < param_2) FUN_802b9280(param_1);`.
/// - `// observed:` `param_1` is an `int *`, so `param_1 + 4` is byte `+0x10`,
///   `+ 0xc` is `+0x30` and `+ 0x18` is `+0x60`; `puVar3 + -0x3e` is `-0xF8`.
///
/// # Arguments
/// - `this`: entry to destroy.
/// - `flag`: when positive, free the entry after teardown.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - takes and releases the sequence-manager mutex,
/// - releases every child object and three list headers.
///
/// # Safety
/// `this` must be null or a live entry; every node in the list must belong to a
/// live child object.
///
/// # Confidence
/// medium - every call is literal; the `-0xF8` node-to-object relation is
/// inferred from the arithmetic and matches [`fn_8027a7d0`]/[`fn_8027a850`].
pub unsafe fn fn_8027a670(this: usize, flag: i32) -> u32 {
    if this != 0 {
        let head = this + 0x10;

        lock_seq_manager();
        let mut node = platform::read_u32(head) as usize;
        while node != head {
            let next = platform::read_u32(node) as usize;
            platform::call2(FUN_80266250, node.wrapping_sub(0xf8) as u32, 0);
            node = next;
        }
        unlock_seq_manager();

        if this + 0x60 != 0 {
            platform::call2(FUN_80229030, (this + 0x60) as u32, 0);
        }
        if this + 0x30 != 0 {
            platform::call2(FUN_80229030, (this + 0x30) as u32, 0);
        }
        platform::call2(FUN_80229030, this as u32, 0);

        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027A750
// ===========================================================================

/// Releases every child of a bank-slot entry and re-buckets what is left.
///
/// Under the sequence-manager mutex the `+0x10` list is walked and each child
/// object receives `0x802667F0`, then `0x8027AA00(this)` moves the surviving
/// nodes into the 128-bucket table. The mutex is released afterwards.
///
/// # Original
/// - Ghidra: `FUN_8027a750` at `0x8027A750` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a750_FUN_8027a750.c`
/// - observed: the `FUN_8027b5f0` / `FUN_801b2e60(iVar1 + 0x354)` pair, the
///   `while (puVar3 != (undefined4 *)(param_1 + 4))` walk with
///   `FUN_802667f0(puVar3 + -0x3e);`, then `FUN_8027aa00(param_1);` and
///   `FUN_801b2f40(iVar1 + 0x354);`.
///
/// # Arguments
/// - `this`: bank-slot entry.
///
/// # Side effects
/// - calls one release routine per child and re-buckets the entry,
/// - takes and releases the sequence-manager mutex.
///
/// # Safety
/// `this` must be a live entry; the sequence manager must be initialised.
///
/// # Confidence
/// medium - the loop, the two calls and the mutex are literal; the callee roles
/// are inferred from their bodies being sequence-control entry points.
pub unsafe fn fn_8027a750(this: usize) {
    let head = this + 0x10;

    lock_seq_manager();
    let mut node = platform::read_u32(head) as usize;
    while node != head {
        let next = platform::read_u32(node) as usize;
        platform::call1(FUN_802667F0, node.wrapping_sub(0xf8) as u32);
        node = next;
    }
    platform::call1(FUN_8027AA00, this as u32);
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027A7D0
// ===========================================================================

/// Applies one argument to every child of a bank-slot entry.
///
/// # Original
/// - Ghidra: `FUN_8027a7d0` at `0x8027A7D0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a7d0_FUN_8027a7d0.c`
/// - observed: the mutex pair, the `param_1 + 4` list walk and
///   `FUN_80266250(puVar3 + -0x3e,param_2);`.
///
/// # Arguments
/// - `this`: bank-slot entry.
/// - `value`: word forwarded to every child object.
///
/// # Side effects
/// - one `0x80266250` call per child,
/// - takes and releases the sequence-manager mutex.
///
/// # Safety
/// `this` must be a live entry; the sequence manager must be initialised.
///
/// # Confidence
/// medium - the walk and the call are literal; the child-object offset is the
/// same inferred `-0xF8` used by [`fn_8027a670`].
pub unsafe fn fn_8027a7d0(this: usize, value: u32) {
    let head = this + 0x10;

    lock_seq_manager();
    let mut node = platform::read_u32(head) as usize;
    while node != head {
        let next = platform::read_u32(node) as usize;
        platform::call2(FUN_80266250, node.wrapping_sub(0xf8) as u32, value);
        node = next;
    }
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027A850
// ===========================================================================

/// Applies two arguments to every child of a bank-slot entry.
///
/// The three-argument sibling of [`fn_8027a7d0`]: the child object receives the
/// caller's two words through `0x80266440`.
///
/// # Original
/// - Ghidra: `FUN_8027a850` at `0x8027A850` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a850_FUN_8027a850.c`
/// - observed: the mutex pair, the `param_1 + 4` list walk and
///   `FUN_80266440(puVar3 + -0x3e,param_2,param_3);`.
///
/// # Arguments
/// - `this`: bank-slot entry.
/// - `first`, `second`: words forwarded to every child object.
///
/// # Side effects
/// - one `0x80266440` call per child,
/// - takes and releases the sequence-manager mutex.
///
/// # Safety
/// `this` must be a live entry; the sequence manager must be initialised.
///
/// # Confidence
/// medium - the walk and the call are literal; the child-object offset is
/// inferred as in [`fn_8027a670`].
pub unsafe fn fn_8027a850(this: usize, first: u32, second: u32) {
    let head = this + 0x10;

    lock_seq_manager();
    let mut node = platform::read_u32(head) as usize;
    while node != head {
        let next = platform::read_u32(node) as usize;
        platform::call3(FUN_80266440, node.wrapping_sub(0xf8) as u32, first, second);
        node = next;
    }
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027A8E0
// ===========================================================================

/// Reads one single-precision value from an entry's `+0x44` array.
///
/// The load is a `float`, but the export widens it to a `double`, so the Rust
/// signature returns `f64` for the same ABI.
///
/// # Original
/// - Ghidra: `FUN_8027a8e0` at `0x8027A8E0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a8e0_FUN_8027a8e0.c`
/// - observed: `return (double)*(float *)(param_1 + param_2 * 4 + 0x44);`.
///
/// # Arguments
/// - `this`: bank-slot entry.
/// - `index`: element index.
///
/// # Returns
/// The `f32` at `this + 0x44 + index * 4`, widened to `f64`.
///
/// # Safety
/// `this + 0x44 + index * 4` must be a readable aligned `f32`.
///
/// # Confidence
/// high - one load and one widening conversion.
pub unsafe fn fn_8027a8e0(this: usize, index: i32) -> f64 {
    platform::read_f32(this + 0x44 + index as usize * 4) as f64
}

// ===========================================================================
// 0x8027A8F0
// ===========================================================================

/// Writes one single-precision value into an entry's `+0x58` array.
///
/// The value arrives in `f1` (hence `f64` in the C prototype) and is narrowed to
/// `f32` for the store, while the two integers arrive in `r4`/`r5`. The parameter
/// order below follows that register layout.
///
/// # Original
/// - Ghidra: `FUN_8027a8f0` at `0x8027A8F0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a8f0_FUN_8027a8f0.c`
/// - observed: `*(float *)(param_2 + param_3 * 4 + 0x58) = (float)param_1;`.
///
/// # Arguments
/// - `value`: the double to narrow and store.
/// - `this`: bank-slot entry.
/// - `index`: element index.
///
/// # Safety
/// `this + 0x58 + index * 4` must be a writable aligned `f32`.
///
/// # Confidence
/// high - one narrowing conversion and one store; the split between `f1` and
/// `r4`/`r5` follows the export's parameter numbering.
pub unsafe fn fn_8027a8f0(value: f64, this: usize, index: i32) {
    platform::write_f32(this + 0x58 + index as usize * 4, value as f32);
}

// ===========================================================================
// 0x8027A900
// ===========================================================================

/// Re-inserts a child node in priority order.
///
/// The child's `+0x100` node is first unlinked from the entry's list object at
/// `+0x0C`. Then, still under the manager mutex, the circular list at `+0x10` is
/// walked until a node whose priority is at least this child's is found, and the
/// node is inserted before it with `0x80229160`. Priority is
/// `clamp(byte +0x98 + word +0x50, 0, 0x7F)` on both sides.
///
/// # Original
/// - Ghidra: `FUN_8027a900` at `0x8027A900` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027a900_FUN_8027a900.c`
/// - observed: `FUN_80229190(param_1 + 0xc,param_2 + 0x100)` between a mutex
///   pair, the `for (local_18[0] = *(undefined4 **)(param_1 + 0x10); ...)`
///   walk comparing `*(byte *)(local_18[0] + -0x1a) + local_18[0][-0x2c]` with
///   `*(byte *)(param_2 + 0x98) + *(int *)(param_2 + 0x50)`, both clamped by
///   the `& ~(v >> 0x1f)` / `0x7f` idiom, then
///   `FUN_80229160(param_1 + 0xc,local_18,param_2 + 0x100)`.
/// - `// observed:` `local_18[0] + -0x1a` words is byte `-0x68` and
///   `local_18[0][-0x2c]` is byte `-0xB0`, i.e. the child object's `+0x98`
///   byte and `+0x50` word.
/// - `// inferred:` the priority is a bucket index, by analogy with the
///   128-bucket table `0x8027AA00` fills.
///
/// # Arguments
/// - `list`: bank-slot entry whose list and node object are used.
/// - `child`: child object with a `+0x100` node.
///
/// # Side effects
/// - unlinks and re-inserts the node under the sequence-manager mutex.
///
/// # Safety
/// Both objects must be live; the sequence manager must be initialised; the
/// list must be circular through `list + 0x10`.
///
/// # Confidence
/// medium - every call, clamp and comparison is literal; the "priority" reading
/// and the list-object relation are inferred.
pub unsafe fn fn_8027a900(list: usize, child: usize) {
    lock_seq_manager();
    platform::call2(FUN_80229190, (list + 0x0c) as u32, (child + 0x100) as u32);
    unlock_seq_manager();

    lock_seq_manager();
    let sentinel = list + 0x10;
    let mut position = platform::read_u32(sentinel) as usize;
    while position != sentinel {
        let node_priority = clamp_priority(
            platform::read_u8(position.wrapping_sub(0x68)) as u32,
            platform::read_u32(position.wrapping_sub(0xb0)),
        );
        let child_priority = clamp_priority(
            platform::read_u8(child + 0x98) as u32,
            platform::read_u32(child + 0x50),
        );
        if child_priority < node_priority {
            break;
        }
        position = platform::read_u32(position) as usize;
    }
    let mut insertion = [position as u32];
    platform::call3(
        FUN_80229160,
        (list + 0x0c) as u32,
        insertion.as_mut_ptr() as u32,
        (child + 0x100) as u32,
    );
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027AE20
// ===========================================================================

/// Clamps a bank slot's cursor and drops entries above it.
///
/// The new cursor is `min(request, slot->+0x28)` with a floor of zero, stored at
/// `+0x24`. While the cursor is below the slot's `+0x00` target, the head
/// entry's virtual `+0x10` slot is called; that call is what removes the entry,
/// so no pointer is advanced here. Everything happens under the manager mutex.
///
/// # Original
/// - Ghidra: `FUN_8027ae20` at `0x8027AE20` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ae20_FUN_8027ae20.c`
/// - observed: `uVar1 = param_1[10];` (byte `+0x28`),
///   `if ((int)param_2 <= (int)uVar1) uVar1 = param_2 & ~((int)param_2 >> 0x1f);`,
///   `param_1[9] = uVar1;` (byte `+0x24`), the
///   `while (param_1[9] < *param_1)` loop with
///   `(**(code **)(*(int *)(param_1[4] + -0x100) + 0x10))()` and the mutex pair.
/// - `// unknown:` the export shows no arguments for the virtual call; the
///   object is the head node minus `0x100`, but its register setup is not
///   visible, so the call is made with no arguments exactly as printed.
///
/// # Arguments
/// - `slot`: 100-byte bank slot built by [`fn_8027a560`].
/// - `request`: requested cursor.
///
/// # Side effects
/// - writes `slot + 0x24`,
/// - one virtual call per dropped entry,
/// - takes and releases the sequence-manager mutex.
///
/// # Safety
/// `slot` must be a live bank slot and its `+0x10` list must be circular; the
/// sequence manager must be initialised.
///
/// # Confidence
/// medium - the clamp, the cursor store and the loop are literal; the virtual
/// call's argument list is not recoverable from the export.
pub unsafe fn fn_8027ae20(slot: usize, request: u32) {
    lock_seq_manager();

    let limit = platform::read_u32(slot + 0x28);
    let cursor = if (request as i32) <= (limit as i32) {
        request & !((request as i32) >> 31) as u32
    } else {
        limit
    };
    platform::write_u32(slot + 0x24, cursor);

    while platform::read_u32(slot + 0x24) < platform::read_u32(slot) {
        let node = platform::read_u32(slot + 0x10) as usize;
        let vtable = platform::read_u32(node.wrapping_sub(0x100)) as usize;
        platform::call0(platform::read_u32(vtable + 0x10) as usize);
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x8027AEB0
// ===========================================================================

/// Stores the bank slot's entry target at `+0x28`.
///
/// [`fn_8027ae20`] reads that word as the upper bound of its clamp, so setting it
/// changes how many entries the slot keeps.
///
/// # Original
/// - Ghidra: `FUN_8027aeb0` at `0x8027AEB0` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027aeb0_FUN_8027aeb0.c`
/// - observed: `*(undefined4 *)(param_1 + 0x28) = param_2;`.
///
/// # Arguments
/// - `slot`: 100-byte bank slot.
/// - `value`: new bound.
///
/// # Safety
/// `slot + 0x28` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_8027aeb0(slot: usize, value: u32) {
    platform::write_u32(slot + 0x28, value);
}

// ===========================================================================
// 0x8027AEC0
// ===========================================================================

/// Reports whether an entry may still be admitted to a bank slot.
///
/// The test has two parts. First the slot must have a non-zero cursor
/// (`+0x24`). Then, when the cursor is at or below the slot's `+0x00` target,
/// the head entry's priority - `clamp(byte +0x98 + word +0x50, 0, 0x7F)` on the
/// head node, read through the node at `-0x100` - must not exceed the requested
/// priority. `1` means "admit", `0` means "reject".
///
/// # Original
/// - Ghidra: `FUN_8027aec0` at `0x8027AEC0` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027aec0_FUN_8027aec0.c`
/// - observed: the early `if (param_1[9] == 0) ... return 0;`, the
///   `if (param_1[9] <= *param_1)` branch, `iVar1 = param_1[4];`,
///   `if (iVar1 == 0x100) ... return 0;`, the
///   `*(byte *)(iVar1 + -0x68) + *(int *)(iVar1 + -0xb0)` clamp and
///   `if (param_2 < (int)uVar2) ... return 0;`, then `return 1;`.
/// - `// observed:` the export folds the head-node null test to `iVar1 == 0x100`;
///   it is reproduced literally rather than translated into a null check.
///
/// # Arguments
/// - `slot`: 100-byte bank slot.
/// - `priority`: the candidate entry's priority.
///
/// # Returns
/// `1` when the entry may be admitted, `0` otherwise.
///
/// # Side effects
/// - takes and releases the sequence-manager mutex on every path.
///
/// # Safety
/// `slot` must be a live bank slot; the sequence manager must be initialised.
///
/// # Confidence
/// medium - every comparison is literal; the admit/reject naming and the
/// priority reading are inferred from [`fn_8027ae20`]'s cursor semantics.
pub unsafe fn fn_8027aec0(slot: usize, priority: i32) -> u32 {
    lock_seq_manager();

    if platform::read_u32(slot + 0x24) == 0 {
        unlock_seq_manager();
        return 0;
    }

    if platform::read_u32(slot + 0x24) <= platform::read_u32(slot) {
        let node = platform::read_u32(slot + 0x10) as usize;
        if node == 0x100 {
            unlock_seq_manager();
            return 0;
        }
        let head_priority = clamp_priority(
            platform::read_u8(node.wrapping_sub(0x68)) as u32,
            platform::read_u32(node.wrapping_sub(0xb0)),
        );
        if priority < head_priority as i32 {
            unlock_seq_manager();
            return 0;
        }
    }

    unlock_seq_manager();
    1
}

// ===========================================================================
// 0x8027B510
// ===========================================================================

/// Returns the "sound system initialised" byte.
///
/// The flag lives in the small-data area that `r13` points at; the crate's
/// `boot::entry::SDA1_BASE` supplies the base, so `r13 - 0x4940` is
/// `0x8042AD40`.
///
/// # Original
/// - Ghidra: `FUN_8027b510` at `0x8027B510` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b510_FUN_8027b510.c`
/// - observed: `return *(undefined1 *)(unaff_r13 + -0x4940);`.
/// - `// inferred:` the byte is the sound system's ready flag; it is the same
///   area the manager's own ready flag at `r13 - 0x4938` lives in.
///
/// # Returns
/// The byte at `r13 - 0x4940`.
///
/// # Safety
/// The small-data area must be mapped (it always is, in the original image).
///
/// # Confidence
/// medium - the load is literal; only the flag's meaning is inferred.
pub unsafe fn fn_8027b510() -> u8 {
    platform::read_u8(SOUND_SYSTEM_READY)
}

// ===========================================================================
// 0x8027B5F0
// ===========================================================================

/// Returns the sequence-manager singleton, building it on first use.
///
/// The one-shot flag at `r13 - 0x4938` gates the whole body. On the first call
/// the manager's fixed fields are cleared, two circular list headers
/// (`+0x37C`/`+0x380` and `+0x388`/`+0x38C`) are pointed at themselves, three
/// structures are initialised (`+0x320` from `+0x340`, `+0x318`, and the mutex at
/// `+0x354`), and [`fn_8027b6b0`] is registered as the block's destructor. The
/// address of the block is returned in both cases.
///
/// The mutex at `+0x354` is the one [`lock_seq_manager`] takes, which is how
/// this object is identified as the sequence manager rather than an unrelated
/// singleton.
///
/// # Original
/// - Ghidra: `FUN_8027b5f0` at `0x8027B5F0` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b5f0_FUN_8027b5f0.c`
/// - observed: the `*(char *)(unaff_r13 + -0x4938) == '\0'` guard, the eleven
///   word/byte stores into `DAT_80412f40..DAT_80412f85`, the
///   `FUN_801b2390(&DAT_80412f10,&DAT_80412f30,4)`,
///   `FUN_801b4b20(&DAT_80412f08)`, `FUN_801b2e20(&DAT_80412f44)` and
///   `FUN_802dc780(&DAT_80412bf0,FUN_8027b6b0,&DAT_80412be0)` calls, the flag
///   store, and `return &DAT_80412bf0;`.
/// - `// observed:` `0x80412F44 - 0x80412BF0 = 0x354`, the mutex word taken by
///   every sequence-manager critical section in this module.
///
/// # Returns
/// The address of the manager block at `0x80412BF0`.
///
/// # Side effects
/// - on first use: writes the manager's fields and initialises its mutex,
/// - registers [`fn_8027b6b0`] as the destructor of the block.
///
/// # Safety
/// The manager's static storage must be writable, as it is in the original
/// image.
///
/// # Confidence
/// high - every store and call is literal and the mutex offset matches the
/// module's own critical sections.
pub unsafe fn fn_8027b5f0() -> u32 {
    if platform::read_u8(SEQ_MANAGER_READY) == 0 {
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x350, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x36c, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x370, 0);

        // First circular list: count 0, head and tail both at +0x37C.
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x378, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x37c, (SEQ_MANAGER_INSTANCE + 0x37c) as u32);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x380, (SEQ_MANAGER_INSTANCE + 0x37c) as u32);

        // Second circular list: count 0, head and tail both at +0x388.
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x384, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x388, (SEQ_MANAGER_INSTANCE + 0x388) as u32);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x38c, (SEQ_MANAGER_INSTANCE + 0x388) as u32);

        platform::write_u8(SEQ_MANAGER_INSTANCE + 0x394, 0);
        platform::write_u8(SEQ_MANAGER_INSTANCE + 0x395, 0);

        platform::call3(
            FUN_801B2390,
            (SEQ_MANAGER_INSTANCE + 0x320) as u32,
            (SEQ_MANAGER_INSTANCE + 0x340) as u32,
            4,
        );
        platform::call1(FUN_801B4B20, (SEQ_MANAGER_INSTANCE + 0x318) as u32);
        platform::call1(MUTEX_INIT, (SEQ_MANAGER_INSTANCE + 0x354) as u32);
        platform::call3(
            FUN_802DC780,
            SEQ_MANAGER_INSTANCE as u32,
            FUN_8027B6B0 as u32,
            (SEQ_MANAGER_INSTANCE - 0x10) as u32,
        );

        platform::write_u8(SEQ_MANAGER_READY, 1);
    }
    SEQ_MANAGER_INSTANCE as u32
}

// ===========================================================================
// 0x8027B6B0
// ===========================================================================

/// Destroys the sequence manager, optionally freeing it.
///
/// The two intrusive lists the constructor created are torn down with
/// `0x80229030`, `+0x384` first and `+0x378` second. Because the block lives in
/// static storage this is normally run without a free, but the free path is
/// reproduced for completeness.
///
/// # Original
/// - Ghidra: `FUN_8027b6b0` at `0x8027B6B0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b6b0_FUN_8027b6b0.c`
/// - observed: `if (param_1 != 0)`, `if (param_1 + 900 != 0)
///   FUN_80229030(param_1 + 900,0);` (900 = `0x384`),
///   `if (param_1 + 0x378 != 0) FUN_80229030(param_1 + 0x378,0);`,
///   `if (0 < param_2) FUN_802b9280(param_1);` and `return param_1;`.
///
/// # Arguments
/// - `this`: manager block from [`fn_8027b5f0`].
/// - `flag`: when positive, free the block after teardown.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or the manager block; its two list headers must be
/// intact.
///
/// # Confidence
/// high - both list calls, the argument offsets and the free guard are literal.
pub unsafe fn fn_8027b6b0(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::call2(FUN_80229030, (this + 0x384) as u32, 0);
        platform::call2(FUN_80229030, (this + 0x378) as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027BA20
// ===========================================================================

/// Inserts one node into the sequence manager's second list.
///
/// The list object lives at manager `+0x384` and its sentinel is the word at
/// `+0x388`, which is also the sort key handed to the insert routine. The whole
/// insertion happens under the manager mutex at `+0x354`.
///
/// # Original
/// - Ghidra: `FUN_8027ba20` at `0x8027BA20` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ba20_FUN_8027ba20.c`
/// - observed: `FUN_801b2e60(param_1 + 0x354)`, `local_18[0] = param_1 + 0x388`,
///   `FUN_80229160(param_1 + 900,local_18,param_2)` and
///   `FUN_801b2f40(param_1 + 0x354)`. `900` is `0x384`.
///
/// # Arguments
/// - `manager`: sequence-manager block from [`fn_8027b5f0`].
/// - `node`: node to link (its `+0x00` word is the forward link).
///
/// # Side effects
/// - inserts one node into `manager + 0x384`,
/// - takes and releases the manager mutex.
///
/// # Safety
/// `manager` must be the live singleton and `node` must be a free list node.
///
/// # Confidence
/// high - the lock pair, the key word and the insert call are literal.
pub unsafe fn fn_8027ba20(manager: usize, node: u32) {
    platform::call1(MUTEX_LOCK, (manager + 0x354) as u32);
    let mut key = [(manager + 0x388) as u32];
    platform::call3(
        FUN_80229160,
        (manager + 0x384) as u32,
        key.as_mut_ptr() as u32,
        node,
    );
    platform::call1(MUTEX_UNLOCK, (manager + 0x354) as u32);
}

// ===========================================================================
// 0x8027BA90
// ===========================================================================

/// Unlinks one node from the sequence manager's second list.
///
/// The counterpart of [`fn_8027ba20`]: `0x80229190` takes the node out of the
/// list object at manager `+0x384` while the manager mutex is held.
///
/// # Original
/// - Ghidra: `FUN_8027ba90` at `0x8027BA90` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ba90_FUN_8027ba90.c`
/// - observed: `FUN_801b2e60(param_1 + 0x354)`,
///   `FUN_80229190(param_1 + 900,param_2)` and
///   `FUN_801b2f40(param_1 + 0x354)`. `900` is `0x384`.
///
/// # Arguments
/// - `manager`: sequence-manager block from [`fn_8027b5f0`].
/// - `node`: node to unlink.
///
/// # Side effects
/// - removes one node from `manager + 0x384`,
/// - takes and releases the manager mutex.
///
/// # Safety
/// `manager` must be the live singleton and `node` must be linked in that list.
///
/// # Confidence
/// high - a single unlink between the manager lock and unlock.
pub unsafe fn fn_8027ba90(manager: usize, node: u32) {
    platform::call1(MUTEX_LOCK, (manager + 0x354) as u32);
    platform::call2(FUN_80229190, (manager + 0x384) as u32, node);
    platform::call1(MUTEX_UNLOCK, (manager + 0x354) as u32);
}

// ===========================================================================
// 0x8027BBF0
// ===========================================================================

/// Initialises the fixed-slot allocator over a caller-supplied buffer.
///
/// The allocator hands out `count` slots of `total / count` bytes each, tracked
/// by a 32-bit bitmap. Fields, `// observed:` from the five stores:
///
/// ```text
/// +0x00  u32 slot base      +0x04 u32 total bytes   +0x08 u32 stride
/// +0x0C  u32 slot count     +0x10 u32 used count    +0x14 u32 bitmap
/// ```
///
/// A zero slot count leaves the object untouched, so the guard is reproduced
/// before any store.
///
/// # Original
/// - Ghidra: `FUN_8027bbf0` at `0x8027BBF0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027bbf0_FUN_8027bbf0.c`
/// - observed: the `FUN_802dcd74` register-save pair whose high word is `this`
///   (the incoming `r3`) and low word `param_2`, the `if (param_4 != 0)` guard,
///   `*puVar1 = (int)uVar3` (= `param_2`), `puVar1[1] = param_3`,
///   `puVar1[2] = param_3 / param_4`, `puVar1[3] = param_4`, `puVar1[4] = 0`,
///   `FUN_800046b4(puVar1 + 5,0,4)` and the `0x801B1140`/`0x801B1180` pair.
/// - `// inferred:` the quotient at `+0x08` is the per-slot stride, because
///   [`fn_8027bcc0`] multiplies it by a slot index and adds the base.
///
/// # Arguments
/// - `this`: allocator object.
/// - `base`: first byte of the slot buffer.
/// - `total`: size of the slot buffer in bytes.
/// - `count`: number of slots.
///
/// # Side effects
/// - writes the five header words and clears the four bitmap bytes.
///
/// # Safety
/// `this` must be writable for `0x18` bytes and `base` must name `total`
/// writable bytes. `count` must be non-zero.
///
/// # Confidence
/// medium - every store is literal; the stride and bitmap roles are inferred
/// from the two allocator bodies that consume them.
pub unsafe fn fn_8027bbf0(this: usize, base: u32, total: u32, count: u32) {
    if count != 0 {
        let token = call0_r(FUN_801B1140);
        platform::write_u32(this, base);
        platform::write_u32(this + 0x04, total);
        platform::write_u32(this + 0x08, total / count);
        platform::write_u32(this + 0x0c, count);
        platform::write_u32(this + 0x10, 0);
        platform::call3(FUN_800046B4, (this + 0x14) as u32, 0, 4);
        platform::call1(FUN_801B1180, token);
    }
}

// ===========================================================================
// 0x8027BC70
// ===========================================================================

/// Clears the allocator's four header words.
///
/// Only the base, total-size, stride and slot-count words are zeroed; the used
/// counter at `+0x10` and the bitmap at `+0x14` are deliberately left alone, so
/// this is a "forget the buffer" reset rather than a full re-initialisation.
///
/// # Original
/// - Ghidra: `FUN_8027bc70` at `0x8027BC70` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027bc70_FUN_8027bc70.c`
/// - observed: the `FUN_801b1140`/`FUN_801b1180` pair around
///   `*param_1 = 0; param_1[1] = 0; param_1[2] = 0; param_1[3] = 0;`.
///
/// # Arguments
/// - `this`: allocator object.
///
/// # Side effects
/// - zeroes `this .. this + 0x0F`.
///
/// # Safety
/// `this` must be writable for 16 bytes.
///
/// # Confidence
/// high - four stores between the two lock calls.
pub unsafe fn fn_8027bc70(this: usize) {
    let token = call0_r(FUN_801B1140);
    platform::write_u32(this, 0);
    platform::write_u32(this + 0x04, 0);
    platform::write_u32(this + 0x08, 0);
    platform::write_u32(this + 0x0c, 0);
    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027BCC0
// ===========================================================================

/// Allocates the first free slot and returns its address.
///
/// The used counter at `+0x10` is compared with the slot count at `+0x0C`
/// first. The bitmap is then scanned byte by byte up to `(count + 7) >> 3`
/// bytes; the first clear bit is set, the used counter is incremented, and the
/// slot address is `base + stride * bit_index`, where the bit index is
/// `byte * 8 + bit`.
///
/// # Original
/// - Ghidra: `FUN_8027bcc0` at `0x8027BCC0` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027bcc0_FUN_8027bcc0.c`
/// - observed: `if (param_1[4] < param_1[3])`, the
///   `for (iVar4 = 0; iVar4 < param_1[3] + 7 >> 3; ...)` scan of
///   `param_1 + iVar4 + 0x14`, the `if (bVar1 != 0xff)` fast path, the
///   `(bVar1 & bVar5) == 0` bit test with `bVar5 <<= 1`, the bitmap OR, the
///   `param_1[4] + 1` store and the
///   `return iVar3 * (iVar6 + iVar4 * 8) + iVar2;` where `iVar3 = param_1[2]`
///   and `iVar2 = *param_1`.
/// - `// observed:` `param_1` is an `int *`, so `param_1 + iVar4 + 0x14` is
///   byte `+0x14`.
///
/// # Arguments
/// - `this`: allocator object from [`fn_8027bbf0`].
///
/// # Returns
/// The address of the first free slot, or `0` when the bitmap is full.
///
/// # Side effects
/// - sets one bitmap bit and increments the used counter.
///
/// # Safety
/// `this` must be a live allocator; the bitmap must cover `count` bits.
///
/// # Confidence
/// high - the bound, the byte walk, the bit test and the address arithmetic are
/// all literal.
pub unsafe fn fn_8027bcc0(this: usize) -> u32 {
    let token = call0_r(FUN_801B1140);
    let count = platform::read_i32(this + 0x0c);
    let used = platform::read_i32(this + 0x10);

    if used < count {
        let bytes = ((count + 7) >> 3) as usize;
        for byte_index in 0..bytes {
            let at = this + 0x14 + byte_index;
            let bits = platform::read_u8(at);
            if bits != 0xff {
                let mut probe: u8 = 1;
                for bit in 0..8i32 {
                    if bits & probe == 0 {
                        platform::write_u8(at, bits | probe);
                        let stride = platform::read_u32(this + 0x08);
                        let base = platform::read_u32(this);
                        platform::write_u32(this + 0x10, (used + 1) as u32);
                        platform::call1(FUN_801B1180, token);
                        let index = bit + byte_index as i32 * 8;
                        return base.wrapping_add(stride.wrapping_mul(index as u32));
                    }
                    probe <<= 1;
                }
            }
        }
    }

    platform::call1(FUN_801B1180, token);
    0
}

// ===========================================================================
// 0x8027BDB0
// ===========================================================================

/// Frees the slot that starts at a given address.
///
/// The slot index is `(address - base) / stride` with unsigned arithmetic; its
/// bit is cleared in the bitmap and the used counter is decremented. No bounds
/// check is performed, exactly as in the export.
///
/// # Original
/// - Ghidra: `FUN_8027bdb0` at `0x8027BDB0` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027bdb0_FUN_8027bdb0.c`
/// - observed: `uVar2 = (uint)(param_2 - *param_1) / (uint)param_1[2];`,
///   `uVar1 = uVar2 >> 3`, the
///   `& ~(byte)(1 << (uVar2 & 7))` clear at `param_1 + uVar1 + 0x14` and
///   `param_1[4] = param_1[4] + -1;`, all between the two lock calls.
///
/// # Arguments
/// - `this`: allocator object from [`fn_8027bbf0`].
/// - `address`: slot address previously returned by [`fn_8027bcc0`].
///
/// # Side effects
/// - clears one bitmap bit and decrements the used counter.
///
/// # Safety
/// `address` must be a slot of this allocator and `stride` must be non-zero.
///
/// # Confidence
/// high - the division, the shift, the bit clear and the counter store are
/// literal.
pub unsafe fn fn_8027bdb0(this: usize, address: u32) {
    let token = call0_r(FUN_801B1140);
    let base = platform::read_u32(this);
    let stride = platform::read_u32(this + 0x08);
    let index = address.wrapping_sub(base) / stride;
    let at = this + 0x14 + (index >> 3) as usize;
    let mask = 1u8 << (index & 7);
    platform::write_u8(at, platform::read_u8(at) & !mask);
    let used = platform::read_u32(this + 0x10).wrapping_sub(1);
    platform::write_u32(this + 0x10, used);
    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027BE30
// ===========================================================================

/// Zeroes the two-word node header.
///
/// # Original
/// - Ghidra: `FUN_8027be30` at `0x8027BE30` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027be30_FUN_8027be30.c`
/// - observed: `*param_1 = 0; param_1[1] = 0;`.
///
/// # Arguments
/// - `this`: node header.
///
/// # Safety
/// `this` must be writable for 8 bytes.
///
/// # Confidence
/// high - two stores, no calls.
pub unsafe fn fn_8027be30(this: usize) {
    platform::write_u32(this, 0);
    platform::write_u32(this + 0x04, 0);
}

// ===========================================================================
// 0x8027BE40
// ===========================================================================

/// Decodes one compact entry into a 16-word record.
///
/// The handle's `+0x04` word points at an object whose `+0x08` byte and `+0x0C`
/// word select a byte string through `0x80283A60`. That byte string is unpacked
/// into the caller's record:
///
/// ```text
/// out[0]  kind: byte 0 -> 2, byte 1 -> 1, byte 2 -> 3, anything else -> 3
/// out[1]  byte 1 of the source, inverted into a 0/1 flag
/// out[2]  byte 2
/// out[3]  byte 3 << 16 | u16 at +4
/// u16 at out+0x10 <- u16 at +6
/// out[5..0x10] <- words at +8 .. +0x30
/// ```
///
/// # Original
/// - Ghidra: `FUN_8027be40` at `0x8027BE40` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027be40_FUN_8027be40.c`
/// - observed: the `FUN_80283a60(*(undefined1 *)(*(int *)(param_1 + 4) + 8),
///   *(undefined4 *)(*(int *)(param_1 + 4) + 0xc))` lookup, the three-way
///   `cVar1` comparison, the `-(uint)(byte)pcVar3[1] >> 0x1f` flag,
///   `(uint)(byte)pcVar3[3] * 0x10000 + (uint)*(ushort *)(pcVar3 + 4)` and the
///   eleven word copies from `+8` to `+0x30`.
/// - `// inferred:` the resolver's third argument is the object's own `+8`
///   base, as it is in the explicitly-printed calls of [`fn_8027bf50`].
///
/// # Arguments
/// - `this`: handle whose `+0x04` word names the source object.
/// - `out`: 16-word output record.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// - writes the whole output record.
///
/// # Safety
/// `this + 4` must name a live source object; `out` must be writable for
/// `0x3C` bytes.
///
/// # Confidence
/// medium - every store is literal; the resolver's hidden third argument is
/// inferred from its sibling call sites.
pub unsafe fn fn_8027be40(this: usize, out: usize) -> u32 {
    let source = platform::read_u32(this + 0x04) as usize;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(source + 0x08) as u32,
        platform::read_u32(source + 0x0c),
        (source + 8) as u32,
    ) as usize;

    let kind = platform::read_u8(entry);
    let decoded = match kind {
        2 => 3,
        1 => 1,
        0 => 2,
        _ => 3,
    };
    platform::write_u32(out, decoded);

    // `-(uint)(byte)pcVar3[1] >> 0x1f` is 1 exactly when byte 1 is zero.
    let flag = if platform::read_u8(entry + 1) == 0 { 1 } else { 0 };
    platform::write_u8(out + 4, flag as u8);
    platform::write_u32(out + 8, platform::read_u8(entry + 2) as u32);
    platform::write_u32(
        out + 0x0c,
        (platform::read_u8(entry + 3) as u32) * 0x1_0000
            + platform::read_u16(entry + 4) as u32,
    );
    platform::write_u16(out + 0x10, platform::read_u16(entry + 6));
    let mut word = 0usize;
    while word < 11 {
        platform::write_u32(
            out + 0x14 + word * 4,
            platform::read_u32(entry + 8 + word * 4),
        );
        word += 1;
    }
    1
}

// ===========================================================================
// 0x8027BF50
// ===========================================================================

/// Decodes one entry of a variable-length table into a `0x28`-byte record.
///
/// The table is found through the handle's `+0x04` object: byte `+0x10` and
/// word `+0x14` are the resolver arguments and `owner + 8` is the base. A table
/// byte 1 of `0` selects the "raw" layout (a count byte followed by that many
/// bytes, plus a fixed `0x7F`/`0x40` prefix) and a byte of `1` selects the
/// "cooked" layout (two bytes copied from the entry, the count byte at `+8` and
/// the payload from `+9`). Both copy at most `0x20` bytes.
///
/// # Original
/// - Ghidra: `FUN_8027bf50` at `0x8027BF50` (704 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027bf50_FUN_8027bf50.c`
/// - observed: the outer `if (param_3 < (int)(uint)*pbVar3)` bound test, the
///   `if (pbVar3[1] == 0)` / `else if (pbVar3[1] == 1)` split, both
///   `FUN_80283a60(pbVar3[param_3 * 8 + 4], *(undefined4 *)(pbVar3 + param_3 * 8
///   + 8), *(int *)(param_1 + 4) + 8)` lookups and their `== 0` early outs,
///   the `*param_2 = 0x7f; param_2[1] = 0x40;` pair, the two count stores, the
///   `uVar6 = 0x20; if (uVar1 < 0x21) uVar6 = uVar1;` clamp and the eight-byte
///   unrolled copy loops.
/// - `// observed:` the two copy loops are one byte copy of
///   `min(count, 0x20)` bytes; the unrolling and the tail loop are reproduced
///   here as a single counted loop.
///
/// # Arguments
/// - `this`: handle whose `+0x04` word names the table owner.
/// - `out`: `0x28`-byte output record.
/// - `index`: table index.
///
/// # Returns
/// `1` when the entry was decoded, `0` when the index is out of range or the
/// nested lookup returned null.
///
/// # Side effects
/// - writes up to `0x28` bytes at `out`.
///
/// # Safety
/// `this` must name a live owner, `index` must address a table slot and `out`
/// must be writable for `0x28` bytes.
///
/// # Confidence
/// medium - the branch structure, the clamps and every store are literal; the
/// "raw"/"cooked" naming and the record size are inferred from the offsets.
pub unsafe fn fn_8027bf50(this: usize, out: usize, index: i32) -> u32 {
    let owner = platform::read_u32(this + 0x04) as usize;
    let table = platform::call3(
        FUN_80283A60,
        platform::read_u8(owner + 0x10) as u32,
        platform::read_u32(owner + 0x14),
        (owner + 8) as u32,
    ) as usize;

    if index >= platform::read_u8(table) as i32 {
        return 0;
    }

    let slot = table + index as usize * 8;
    let layout = platform::read_u8(table + 1);

    if layout == 0 {
        let entry = platform::call3(
            FUN_80283A60,
            platform::read_u8(slot + 4) as u32,
            platform::read_u32(slot + 8),
            (owner + 8) as u32,
        ) as usize;
        if entry == 0 {
            return 0;
        }
        platform::write_u8(out, 0x7f);
        platform::write_u8(out + 1, 0x40);
        let count = platform::read_u8(entry) as u32;
        platform::write_u32(out + 4, count);
        let copied = if count < 0x21 { count } else { 0x20 };
        let mut i = 0usize;
        while i < copied as usize {
            platform::write_u8(out + 8 + i, platform::read_u8(entry + 1 + i));
            i += 1;
        }
    } else if layout == 1 {
        let entry = platform::call3(
            FUN_80283A60,
            platform::read_u8(slot + 4) as u32,
            platform::read_u32(slot + 8),
            (owner + 8) as u32,
        ) as usize;
        if entry == 0 {
            return 0;
        }
        platform::write_u8(out, platform::read_u8(entry));
        platform::write_u8(out + 1, platform::read_u8(entry + 1));
        let count = platform::read_u8(entry + 8) as u32;
        platform::write_u32(out + 4, count);
        let copied = if count < 0x21 { count } else { 0x20 };
        let mut i = 0usize;
        while i < copied as usize {
            platform::write_u8(out + 8 + i, platform::read_u8(entry + 9 + i));
            i += 1;
        }
    }

    1
}

// ===========================================================================
// 0x8027C3E0
// ===========================================================================

/// Reads the type byte of the object named by a handle.
///
/// The handle holds a source object at `+0x04` and its `+0x08` word is the
/// table base; the source's `+0x18` byte and `+0x1C` word are the resolver
/// arguments. A null `+0x04` word yields `0` without calling anything.
///
/// # Original
/// - Ghidra: `FUN_8027c3e0` at `0x8027C3E0` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c3e0_FUN_8027c3e0.c`
/// - observed: `if (*(int *)(param_1 + 4) == 0) uVar2 = 0;` and the
///   `FUN_80283a60(*(undefined1 *)(iVar3 + 0x18), *(undefined4 *)(iVar3 + 0x1c),
///   iVar3 + 8)` lookup followed by `uVar2 = *puVar1;`.
///
/// # Arguments
/// - `this`: handle object.
///
/// # Returns
/// The first byte of the resolved entry, or `0` when the handle is empty.
///
/// # Safety
/// The handle's source object and table base must be readable.
///
/// # Confidence
/// high - the guard, the three resolver arguments and the byte load are
/// literal.
pub unsafe fn fn_8027c3e0(this: usize) -> u8 {
    if platform::read_u32(this + 0x04) == 0 {
        return 0;
    }
    let source = platform::read_u32(this + 0x08) as usize;
    let entry = platform::call3(
        FUN_80283A60,
        platform::read_u8(source + 0x18) as u32,
        platform::read_u32(source + 0x1c),
        (source + 8) as u32,
    ) as usize;
    platform::read_u8(entry)
}

// ===========================================================================
// 0x8027C430
// ===========================================================================

/// Decodes the handle's entry when it is present.
///
/// The `+0x04` word doubles as the presence flag: when it is non-zero the
/// sixteen-word decoder [`fn_8027be40`] is invoked on the handle's `+0x04`
/// address and the flag itself is returned, otherwise nothing is written.
///
/// # Original
/// - Ghidra: `FUN_8027c430` at `0x8027C430` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c430_FUN_8027c430.c`
/// - observed: `bVar1 = *(int *)(param_1 + 4) != 0;`,
///   `if (bVar1) FUN_8027be40(param_1 + 4);` and `return bVar1;`. The C type is
///   `bool`; the body keeps the original `u32` 0/1 result.
/// - `// observed:` the export prints only one argument for `0x8027BE40`,
///   whose own body takes `(handle, out)`; `r4` is not written by this body, so
///   the record pointer is the caller's `out` (`this + 0xC0` at `0x8027D660`).
///
/// # Arguments
/// - `this`: handle object.
/// - `out`: 16-word record written by [`fn_8027be40`].
///
/// # Returns
/// `1` when the handle had an entry, `0` otherwise.
///
/// # Side effects
/// - decodes the entry into `out`.
///
/// # Safety
/// The handle's `+0x04` word, when non-zero, must name a live entry and `out`
/// must be writable for `0x40` bytes.
///
/// # Confidence
/// high - the flag, the call and the returned flag are literal; the `out`
/// register comes from the caller (`work/wave234/laneB` disassembly).
pub unsafe fn fn_8027c430(this: usize, out: usize) -> u32 {
    let present = platform::read_u32(this + 0x04) != 0;
    if present {
        platform::call2(FUN_8027BE40, (this + 0x04) as u32, out as u32);
    }
    present as u32
}

// ===========================================================================
// 0x8027C470
// ===========================================================================

/// Decodes the handle's variable-length entry when it is present.
///
/// The sibling of [`fn_8027c430`] for the other table layout: the same `+0x04`
/// presence word gates a call to [`fn_8027bf50`], and the gate value is what the
/// function returns.
///
/// # Original
/// - Ghidra: `FUN_8027c470` at `0x8027C470` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c470_FUN_8027c470.c`
/// - observed: `bVar1 = *(int *)(param_1 + 4) != 0;`,
///   `if (bVar1) FUN_8027bf50(param_1 + 4);` and `return bVar1;`. The C type is
///   `bool`; the body keeps the original `u32` 0/1 result.
/// - `// observed:` the export shows only one argument for `0x8027BF50`, whose
///   own body takes `(handle, out, index)`; `r4`/`r5` are not written here, so
///   they are the caller's `out` (`this + 0xB60 + 0x38 * i` at `0x8027D660`)
///   and `index`.
///
/// # Arguments
/// - `this`: handle object.
/// - `out`: record written by [`fn_8027bf50`].
/// - `index`: table index used by [`fn_8027bf50`].
///
/// # Returns
/// `1` when the handle had an entry, `0` otherwise.
///
/// # Side effects
/// - decodes the entry into `out`.
///
/// # Safety
/// The handle's `+0x04` word, when non-zero, must name a live entry and `out`
/// must be writable.
///
/// # Confidence
/// high - the flag, the call and the returned flag are literal; the `out` and
/// `index` registers come from the caller (`work/wave234/laneB` disassembly).
pub unsafe fn fn_8027c470(this: usize, out: usize, index: i32) -> u32 {
    let present = platform::read_u32(this + 0x04) != 0;
    if present {
        platform::call3(
            FUN_8027BF50,
            (this + 0x04) as u32,
            out as u32,
            index as u32,
        );
    }
    present as u32
}

// ===========================================================================
// 0x8027C8B0
// ===========================================================================

/// Tears an object down and frees it when a positive flag is given.
///
/// The tear-down is [`fn_8027fd90`] applied with a zero flag, so only its
/// list/registration half runs; the object itself is released with
/// `0x802B9280` when `flag` is greater than zero. The address is returned
/// unchanged, which is the same deleting-destructor shape as [`fn_8027a610`].
///
/// # Original
/// - Ghidra: `FUN_8027c8b0` at `0x8027C8B0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c8b0_FUN_8027c8b0.c`
/// - observed: `if ((param_1 != 0) && (FUN_8027fd90(param_1,0), 0 < param_2))
///   FUN_802b9280(param_1);` and `return param_1;`.
///
/// # Arguments
/// - `this`: object to destroy.
/// - `flag`: when positive, free the object after the tear-down.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or a live object.
///
/// # Confidence
/// high - the guard, the two calls and the returned address are literal.
pub unsafe fn fn_8027c8b0(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::call2(FUN_8027FD90, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027C910
// ===========================================================================

/// Second deleting destructor with the body of [`fn_8027c8b0`].
///
/// The two addresses are distinct functions with byte-identical exports: both
/// call `0x8027FD90(this, 0)`, free when the flag is positive and return
/// `this`. They are kept as separate bodies because they are separate entries
/// in the image (the shape of a class that inherits two sub-object destructors).
///
/// # Original
/// - Ghidra: `FUN_8027c910` at `0x8027C910` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c910_FUN_8027c910.c`
/// - observed: `if ((param_1 != 0) && (FUN_8027fd90(param_1,0), 0 < param_2))
///   FUN_802b9280(param_1);` and `return param_1;`, identical to
///   `0x8027C8B0`.
///
/// # Arguments
/// - `this`: object to destroy.
/// - `flag`: when positive, free the object after the tear-down.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or a live object.
///
/// # Confidence
/// high - the body is identical to the neighbouring address.
pub unsafe fn fn_8027c910(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::call2(FUN_8027FD90, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027CC90
// ===========================================================================

/// Releases the sub-object that owns this block.
///
/// The object's virtual slot `+0x10` is called first. The manager mutex is then
/// taken: when the byte at `+0x100` is clear only the unlock runs; otherwise the
/// word at `+0x804` is cleared, the `0x680`-byte block at `+0x180` is filled by
/// `0x8026D9B0`, the flag is cleared and the mutex is released.
///
/// # Original
/// - Ghidra: `FUN_8027cc90` at `0x8027CC90` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027cc90_FUN_8027cc90.c`
/// - observed: `(**(code **)(*param_1 + 0x10))();`, the
///   `FUN_8027b5f0` + `FUN_801b2e60(iVar1 + 0x354)` pair, the
///   `if (*(char *)(param_1 + 0x40) == '\0')` branch (word `+0x40` is byte
///   `+0x100`), `param_1[0x201] = 0` (byte `+0x804`),
///   `FUN_8026d9b0(param_1 + 0x60,param_1 + 0x61,0x680)` (bytes `+0x180` and
///   `+0x184`), `*(undefined1 *)(param_1 + 0x40) = 0` and the final
///   `FUN_801b2f40(iVar1 + 0x354)`.
/// - `// observed:` the export prints the virtual call with no argument list;
///   the object itself is passed here because `r3` already holds it.
///
/// # Arguments
/// - `this`: sub-object block.
///
/// # Side effects
/// - one virtual call through slot `+0x10`,
/// - clears `+0x804` and the `+0x180` block when `+0x100` is set,
/// - takes and releases the manager mutex.
///
/// # Safety
/// `this` must be a live object with a vtable and the manager must be
/// initialised.
///
/// # Confidence
/// medium - every branch, offset and call is literal; the virtual call's
/// argument list is inferred as the object pointer.
pub unsafe fn fn_8027cc90(this: usize) {
    let vtable = platform::read_u32(this) as usize;
    let release = platform::read_u32(vtable + 0x10) as usize;
    platform::call1(release, this as u32);

    lock_seq_manager();

    if platform::read_u8(this + 0x100) == 0 {
        unlock_seq_manager();
    } else {
        platform::write_u32(this + 0x804, 0);
        platform::call3(
            FUN_8026D9B0,
            (this + 0x180) as u32,
            (this + 0x184) as u32,
            0x680,
        );
        platform::write_u8(this + 0x100, 0);
        unlock_seq_manager();
    }
}

// ===========================================================================
// 0x8027CD20
// ===========================================================================

/// Arms the sub-object at `this + 0x110`.
///
/// Everything happens under the manager mutex. The three caller words are stored
/// at `+0x808`, `+0x14C` and `+0x150`, four state bytes are cleared and `+0x101`
/// is set, the object's `+0xB4` node is unlinked from the manager list, three
/// words are mirrored to `+0x168..+0x170`, and the host returned by
/// `0x8027FDD0` receives `+0x154` as element 1 of its `0x0C`-byte table.
///
/// # Original
/// - Ghidra: `FUN_8027cd20` at `0x8027CD20` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027cd20_FUN_8027cd20.c`
/// - observed: the `FUN_8027b5f0` + `FUN_801b2e60(param_1 + 0x354)` pair, the
///   stores at `+0x808`/`+0x14C`/`+0x150`, the byte clears at
///   `+0x104`/`+0x105`/`+0x106`, `*(undefined1 *)(param_1 + 0x101) = 1`, the
///   `iVar1 = param_1; if (param_1 != 0) iVar1 = param_1 + 0xb4;` node choice,
///   `FUN_8027ba20(uVar2,iVar1)`, the three `+0x168..+0x170` mirrors, the
///   `FUN_8027fdd0()` + `FUN_8027ff60(uVar2,param_1 + 0x154,1)` pair and the
///   final `FUN_801b2f40(iVar1 + 0x354)`.
///
/// # Arguments
/// - `this`: sub-object block.
/// - `first`: stored at `+0x808`.
/// - `second`: stored at `+0x14C`.
/// - `third`: stored at `+0x150`.
///
/// # Returns
/// `1`.
///
/// # Side effects
/// - writes the three caller words, four state bytes and three mirror words,
/// - unlinks `+0xB4` from the manager list,
/// - inserts an element into the host table,
/// - takes and releases the manager mutex.
///
/// # Safety
/// `this` must be a live block and the sequence manager must be initialised.
///
/// # Confidence
/// high - every store, offset and call is literal.
pub unsafe fn fn_8027cd20(this: usize, first: u32, second: u32, third: u32) -> u32 {
    lock_seq_manager();

    platform::write_u32(this + 0x808, first);
    platform::write_u32(this + 0x14c, second);
    platform::write_u32(this + 0x150, third);
    platform::write_u8(this + 0x104, 0);
    platform::write_u8(this + 0x105, 0);
    platform::write_u8(this + 0x106, 0);
    platform::write_u8(this + 0x101, 1);

    let node = if this != 0 { this + 0xb4 } else { this };
    let manager = call0_r(SEQ_MANAGER);
    platform::call2(FUN_8027BA90, manager, node as u32);

    platform::write_u32(this + 0x164, this as u32);
    platform::write_u32(this + 0x168, platform::read_u32(this + 0x808));
    platform::write_u32(this + 0x16c, platform::read_u32(this + 0x14c));
    platform::write_u32(this + 0x170, platform::read_u32(this + 0x150));

    let host = call0_r(FUN_8027FDD0);
    platform::call3(FUN_8027FF60, host, (this + 0x154) as u32, 1);

    unlock_seq_manager();
    1
}

// ===========================================================================
// 0x8027EEF0
// ===========================================================================

/// Applies one mode word to a caller-provided output block.
///
/// Modes `0` and `1` run the one-shot reset `0x802810A0` and then clear the
/// output word. Modes `2` and `3` only clear the output word. Any larger mode
/// leaves the block untouched. The state lock is held on every path.
///
/// # Original
/// - Ghidra: `FUN_8027eef0` at `0x8027EEF0` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027eef0_FUN_8027eef0.c`
/// - observed: `if (param_2 < 2) { FUN_802810a0(param_1); *(undefined4 *)(param_3
///   + 4) = 0; }`, the `else { if (1 < param_2 - 2) { FUN_801b1180(); return; }
///   *(undefined4 *)(param_3 + 4) = 0; }` branch and the two
///   `FUN_801b1140`/`FUN_801b1180` pairs.
/// - `// observed:` `param_2 - 2` is an unsigned subtraction, so the early
///   return covers every mode above `3`.
///
/// # Arguments
/// - `this`: object passed to the reset routine for modes 0/1.
/// - `mode`: mode word.
/// - `out`: two-word output block; its second word is the one cleared.
///
/// # Side effects
/// - may run `0x802810A0(this)`,
/// - clears `out + 4` for modes `0..3`,
/// - takes and releases the state lock.
///
/// # Safety
/// `out` must be writable for 8 bytes; `this` must be valid for the reset call.
///
/// # Confidence
/// medium - the branches and stores are literal; the reset routine's role is
/// inferred from the mode split.
pub unsafe fn fn_8027eef0(this: u32, mode: u32, out: usize) {
    let token = call0_r(FUN_801B1140);

    if mode < 2 {
        platform::call1(FUN_802810A0, this);
        platform::write_u32(out + 4, 0);
    } else if mode.wrapping_sub(2) > 1 {
        platform::call1(FUN_801B1180, token);
        return;
    } else {
        platform::write_u32(out + 4, 0);
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027EF90
// ===========================================================================

/// Writes one `f32` into every slot selected by a bit mask.
///
/// The slot array starts at `this` and has `count` entries of `0x38` bytes; the
/// `f32` field sits at `+0xB88` in each. Bit `i` of `mask` selects entry `i`, and
/// a zero mask stops the walk early.
///
/// # Original
/// - Ghidra: `FUN_8027ef90` at `0x8027EF90` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ef90_FUN_8027ef90.c`
/// - observed: the `for (iVar2 = 0; iVar2 < *(int *)(param_2 + 0x80c); iVar2++)`
///   loop, the `if (param_3 == 0) break;` and `if ((param_3 & 1) != 0)` tests,
///   the `*(float *)(iVar1 + 0xb88) = (float)param_1;` store, `param_3 >>= 1`
///   and the `iVar1 = iVar1 + 0x38` step between the two lock calls.
///
/// # Arguments
/// - `value`: value converted to `f32` and stored.
/// - `this`: first slot of the array.
/// - `mask`: selection mask, bit `i` for slot `i`.
///
/// # Side effects
/// - writes one `f32` per selected slot,
/// - takes and releases the state lock.
///
/// # Safety
/// The array must hold at least the count read from `this + 0x80C` slots and
/// each slot must be writable at `+0xB88`.
///
/// # Confidence
/// high - the loop, the mask test, the widening/narrowing and the store are
/// literal.
pub unsafe fn fn_8027ef90(value: f64, this: usize, mask: u32) {
    let token = call0_r(FUN_801B1140);
    let count = platform::read_i32(this + 0x80c);

    let mut remaining = mask;
    let mut slot = this;
    let mut index = 0i32;
    while index < count {
        if remaining == 0 {
            break;
        }
        if remaining & 1 != 0 {
            platform::write_f32(slot + 0xb88, value as f32);
        }
        remaining >>= 1;
        slot += 0x38;
        index += 1;
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027F020
// ===========================================================================

/// Address of one `0x38`-byte variable slot, or zero when out of range.
///
/// Eight slots live at `this + 0xB58 + index * 0x38`; any index above `7` is
/// rejected before the arithmetic.
///
/// # Original
/// - Ghidra: `FUN_8027f020` at `0x8027F020` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f020_FUN_8027f020.c`
/// - observed: `if (7 < param_2) return 0;` and
///   `return param_1 + param_2 * 0x38 + 0xb58;`.
///
/// # Arguments
/// - `this`: object holding the slot table.
/// - `index`: slot index, `0..=7`.
///
/// # Returns
/// The slot address, or `0` when `index` is above `7`.
///
/// # Safety
/// `this` must be the live object and the returned range must be valid.
///
/// # Confidence
/// high - one bound test and one multiply-add.
pub unsafe fn fn_8027f020(this: usize, index: i32) -> u32 {
    if index > 7 {
        return 0;
    }
    (this + index as usize * 0x38 + 0xb58) as u32
}

// ===========================================================================
// 0x8027F040
// ===========================================================================

/// Runs one four-word lookup and arms a byte when it fails.
///
/// The four words at `this + 0x10 .. +0x1C` are handed to `0x8027D660`. A zero
/// result means "not ready", and the first of the four words then receives byte
/// `1` at `+0x104`.
///
/// # Original
/// - Ghidra: `FUN_8027f040` at `0x8027F040` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f040_FUN_8027f040.c`
/// - observed: `iVar1 = FUN_8027d660(*(undefined4 *)(param_1 + 0x10),
///   *(undefined4 *)(param_1 + 0x14), *(undefined4 *)(param_1 + 0x18),
///   *(undefined4 *)(param_1 + 0x1c));` and
///   `if (iVar1 == 0) *(undefined1 *)(*(int *)(param_1 + 0x10) + 0x104) = 1;`.
///
/// # Arguments
/// - `this`: object holding the four lookup words at `+0x10`.
///
/// # Side effects
/// - on a zero lookup result, writes byte `1` at `word0 + 0x104`.
///
/// # Safety
/// `this + 0x10` must hold four readable words; when the lookup fails, the first
/// word must name a writable object.
///
/// # Confidence
/// high - the four-argument call and the fallback store are literal.
pub unsafe fn fn_8027f040(this: usize) {
    let target = platform::read_u32(this + 0x10) as usize;
    let result = platform::call4(
        FUN_8027D660,
        target as u32,
        platform::read_u32(this + 0x14),
        platform::read_u32(this + 0x18),
        platform::read_u32(this + 0x1c),
    );
    if result == 0 {
        platform::write_u8(target + 0x104, 1);
    }
}

// ===========================================================================
// 0x8027F510
// ===========================================================================

/// Resets all eight ramp records of the large element player.
///
/// `0x802660D0` resets the player's member record. Each of the eight
/// `0x10`-byte records starting at `+0xE30` is then stored as
/// `{ start: 0.0, end: 1.0, length: 1, position: 0 }`. The image writes the
/// two halves of each 32-byte group separately, but both reach the same four
/// fields; the interpolation branch is dead because position equals length.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027F510 (slot 6 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027F510..0x8027F648` (312 bytes).
/// - observed: `bl 0x802660D0`, the `li r0,4` / `mtlr` / `bdnz` loop over
///   `+0xE30` and `+0xE40`, the `lfs f6,-17800(r2)` (0.0) and
///   `lfs f0,-17796(r2)` (1.0) constants, the `stw ... 0xE38/0xE3C` stores and
///   `addi r31,r31,0x20`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Side effects
/// Overwrites the eight ramp records at `+0xE30..+0xEB0`.
///
/// # Safety
/// `this` must be a live player with eight writable records.
///
/// # Confidence
/// high - the loop count, stride, constants and final stores are literal.
pub unsafe fn fn_8027f510(this: usize) {
    platform::call1(FUN_802660D0, this as u32);
    let mut record = this + 0xe30;
    for _ in 0..8 {
        platform::write_f32(record, 0.0);
        platform::write_f32(record + 4, 1.0);
        platform::write_u32(record + 8, 1);
        platform::write_u32(record + 0x0c, 0);
        record += 0x10;
    }
}

// ===========================================================================
// 0x8027F650
// ===========================================================================

/// Rebuilds the sub-object from the caller's three words.
///
/// The object's virtual slot `+0x20` runs first. `0x80267380` then supplies the
/// shared value, and `0x8027CA70` receives the sub-object at `this + 0x110`, the
/// three caller words and that shared value.
///
/// # Original
/// - Ghidra: `FUN_8027f650` at `0x8027F650` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f650_FUN_8027f650.c`
/// - observed: `(**(code **)(*param_1 + 0x20))();`,
///   `uVar1 = FUN_80267380(param_1);` and
///   `FUN_8027ca70(param_1 + 0x44,param_2,param_3,param_4,uVar1);` (word `+0x44`
///   is byte `+0x110`).
/// - `// observed:` the export prints the virtual call without arguments.
/// - `// inferred:` the virtual slot receives the object and the three caller
///   words, which are still live in `r3..r6` at that point.
///
/// # Arguments
/// - `this`: owning object.
/// - `first`, `second`, `third`: words forwarded to the virtual slot and to
///   [`fn_8027cd20`]'s sibling `0x8027CA70`.
///
/// # Side effects
/// - one virtual call through slot `+0x20`,
/// - rebuilds the sub-object at `+0x110`.
///
/// # Safety
/// `this` must be a live object with a vtable and `+0x110` a live sub-object.
///
/// # Confidence
/// medium - every call and offset is literal; the virtual call's argument list
/// is inferred.
pub unsafe fn fn_8027f650(this: usize, first: u32, second: u32, third: u32) {
    let vtable = platform::read_u32(this) as usize;
    let slot = platform::read_u32(vtable + 0x20) as usize;
    platform::call4(slot, this as u32, first, second, third);

    let shared = platform::call1(FUN_80267380, this as u32);
    platform::call5(
        FUN_8027CA70,
        (this + 0x110) as u32,
        first,
        second,
        third,
        shared,
    );
}

// ===========================================================================
// 0x8027F6D0
// ===========================================================================

/// Arms the sub-object and rolls it back when arming fails.
///
/// `0x8027CD20` receives the sub-object at `this + 0x110` together with the
/// caller's words in the order `(third, first, second)`. A zero result triggers
/// the release path [`fn_8027cc90`], and the boolean is inverted into the
/// function's result.
///
/// # Original
/// - Ghidra: `FUN_8027f6d0` at `0x8027F6D0` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f6d0_FUN_8027f6d0.c`
/// - observed: `iVar1 = FUN_8027cd20(param_1 + 0x110,param_4,param_2,param_3);`,
///   `if (iVar1 == 0) FUN_8027cc90(param_1 + 0x110);` and
///   `return iVar1 != 0;`.
///
/// # Arguments
/// - `this`: owning object.
/// - `first`, `second`, `third`: caller words; `third` is passed first.
///
/// # Returns
/// `1` when arming succeeded, `0` otherwise.
///
/// # Side effects
/// - arms the sub-object, or releases it on failure.
///
/// # Safety
/// `this + 0x110` must be a live sub-object.
///
/// # Confidence
/// high - the swapped argument order, the failure branch and the inverted
/// result are literal.
pub unsafe fn fn_8027f6d0(this: usize, first: u32, second: u32, third: u32) -> u32 {
    let armed = platform::call4(FUN_8027CD20, (this + 0x110) as u32, third, first, second);
    if armed == 0 {
        platform::call1(FUN_8027CC90, (this + 0x110) as u32);
    }
    (armed != 0) as u32
}

// ===========================================================================
// 0x8027F730
// ===========================================================================

/// Advances the per-slot counters of the eight-slot table.
///
/// `0x80266D80` runs once, then the eight slots of the sub-object at `+0x110`
/// are visited; each slot that exists and whose own `+0xE3C` counter is below its
/// `+0xE38` limit gets that counter incremented. Note that the counters come from
/// `this + index * 0x10`, i.e. from the owning object, while the existence test
/// uses the sub-object.
///
/// # Original
/// - Ghidra: `FUN_8027f730` at `0x8027F730` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f730_FUN_8027f730.c`
/// - observed: `FUN_80266d80();`, the `do { ... } while (iVar2 < 8)` loop with
///   `iVar1 = FUN_8027f020(param_1 + 0x110,iVar2);` and
///   `if ((iVar1 != 0) && (*(int *)(iVar3 + 0xe3c) < *(int *)(iVar3 + 0xe38)))`,
///   the `+0xe3c` increment, `iVar3 = iVar3 + 0x10` and `iVar2 = iVar2 + 1`.
///
/// # Arguments
/// - `this`: owning object.
///
/// # Side effects
/// - one `0x80266D80` call,
/// - up to eight counter increments at `this + index * 0x10 + 0xE3C`.
///
/// # Safety
/// `this + 0x110` must be the live sub-object and the counters must be writable.
///
/// # Confidence
/// high - the loop bounds, the two counter offsets and the increment guard are
/// literal.
pub unsafe fn fn_8027f730(this: usize) {
    platform::call0(FUN_80266D80);

    let mut index = 0i32;
    let mut slot = this;
    loop {
        let target = platform::call2(FUN_8027F020, (this + 0x110) as u32, index as u32);
        if target != 0 {
            let used = platform::read_i32(slot + 0xe3c);
            let limit = platform::read_i32(slot + 0xe38);
            if used < limit {
                platform::write_u32(slot + 0xe3c, (used + 1) as u32);
            }
        }
        index += 1;
        slot += 0x10;
        if index >= 8 {
            break;
        }
    }
}

// ===========================================================================
// 0x8027FD50
// ===========================================================================

/// Detaches a handle from the object that owns it.
///
/// The handle's first word names the owner. When the owner's `+0xE28` back
/// pointer still names this handle it is cleared, and the handle's own word is
/// then cleared as well.
///
/// # Original
/// - Ghidra: `FUN_8027fd50` at `0x8027FD50` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027fd50_FUN_8027fd50.c`
/// - observed: `iVar1 = *param_1;`,
///   `if ((iVar1 != 0) && (*(int **)(iVar1 + 0xe28) == param_1))
///   *(undefined4 *)(iVar1 + 0xe28) = 0;`, then
///   `if (*param_1 == 0) return;` and `*param_1 = 0;`.
///
/// # Arguments
/// - `handle`: single-word handle.
///
/// # Side effects
/// - clears the owner's `+0xE28` back pointer when it names this handle,
/// - clears the handle word.
///
/// # Safety
/// `handle` and, when set, the owner it names must be readable/writable.
///
/// # Confidence
/// high - the identity test and the two clears are literal.
pub unsafe fn fn_8027fd50(handle: usize) {
    let owner = platform::read_u32(handle) as usize;
    if owner != 0 && platform::read_u32(owner + 0xe28) as usize == handle {
        platform::write_u32(owner + 0xe28, 0);
    }
    if platform::read_u32(handle) != 0 {
        platform::write_u32(handle, 0);
    }
}

// ===========================================================================
// 0x8027FD90
// ===========================================================================

/// Tears an object down and frees it when a positive flag is given.
///
/// The same deleting-destructor shape as [`fn_8027c8b0`]: `0x802B9280` runs only
/// for a positive flag, and the object address is returned unchanged.
///
/// # Original
/// - Ghidra: `FUN_8027fd90` at `0x8027FD90` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027fd90_FUN_8027fd90.c`
/// - observed: `if ((param_1 != 0) && (0 < param_2)) FUN_802b9280();` and
///   `return param_1;`.
/// - `// observed:` this export prints no argument for `0x802B9280`, but every
///   other free site in this module forwards the object, so `this` is passed.
///
/// # Arguments
/// - `this`: object to free.
/// - `flag`: when positive, release the object.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or an object the allocator owns.
///
/// # Confidence
/// medium - the guard and the returned address are literal; the free call's
/// argument is inferred from the module's other free sites.
pub unsafe fn fn_8027fd90(this: usize, flag: i32) -> u32 {
    if this != 0 && flag > 0 {
        platform::call1(FUN_802B9280, this as u32);
    }
    this as u32
}

// ===========================================================================
// 0x8027FE70
// ===========================================================================

/// Registers a per-element destructor and frees the array on request.
///
/// `0x802DCA98` is given the array, the destructor [`fn_8027fee0`], the element
/// size `0x0C` and the element count `3`; the array itself is then released with
/// `0x802B9280` when the flag is positive. The address is returned unchanged.
///
/// # Original
/// - Ghidra: `FUN_8027fe70` at `0x8027FE70` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027fe70_FUN_8027fe70.c`
/// - observed: `if ((param_1 != 0) && (FUN_802dca98(param_1,FUN_8027fee0,0xc,3),
///   0 < param_2)) FUN_802b9280(param_1);` and `return param_1;`.
///
/// # Arguments
/// - `this`: three-element array of `0x0C`-byte elements.
/// - `flag`: when positive, free the array after registering the destructor.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or a live array of three `0x0C`-byte elements.
///
/// # Confidence
/// high - the guard, the four-argument registration and the free are literal.
pub unsafe fn fn_8027fe70(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::call4(FUN_802DCA98, this as u32, FUN_8027FEE0 as u32, 0x0c, 3);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027FEE0
// ===========================================================================

/// Per-element destructor registered by [`fn_8027fe70`].
///
/// The element is a list header: `0x80229030(element, 0)` unlinks everything it
/// holds and the element is freed when the flag is positive. The body is the
/// same shape as the bank-slot destructor [`fn_8027a610`].
///
/// # Original
/// - Ghidra: `FUN_8027fee0` at `0x8027FEE0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027fee0_FUN_8027fee0.c`
/// - observed: `if ((param_1 != 0) && (FUN_80229030(param_1,0), 0 < param_2))
///   FUN_802b9280(param_1);` and `return param_1;`.
///
/// # Arguments
/// - `this`: element to destroy.
/// - `flag`: when positive, free the element after the list teardown.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be null or a live list header.
///
/// # Confidence
/// high - the guard, the list teardown and the free are literal.
pub unsafe fn fn_8027fee0(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::call2(FUN_80229030, this as u32, 0);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027FF60
// ===========================================================================

/// Links one node into a `0x0C`-byte element table and wakes its waiter.
///
/// The element is `table + index * 0x0C`; its `+0x04` word becomes the sort key
/// for the insert routine. The node's `+0x0C` byte is set to `1` first, and the
/// thread waiting on `table + 0x2C` is woken afterwards, all with the state lock
/// held.
///
/// # Original
/// - Ghidra: `FUN_8027ff60` at `0x8027FF60` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ff60_FUN_8027ff60.c`
/// - observed: `uVar1 = FUN_801b1140();`, `*(undefined1 *)(param_2 + 0xc) = 1;`,
///   `iVar2 = param_1 + param_3 * 0xc;`, `local_18[0] = iVar2 + 4;`,
///   `FUN_80229160(iVar2,local_18,param_2 + 4);`,
///   `FUN_801b5d80(param_1 + 0x2c);` and `FUN_801b1180(uVar1);`.
///
/// # Arguments
/// - `table`: element table base.
/// - `node`: node to link; the insert uses its `+0x04` word.
/// - `index`: element index.
///
/// # Side effects
/// - sets `node + 0x0C` to `1`,
/// - inserts into `table + index * 0x0C`,
/// - wakes the waiter on `table + 0x2C`,
/// - takes and releases the state lock.
///
/// # Safety
/// `table` must be a live table, `index` must address one element and `node`
/// must be writable.
///
/// # Confidence
/// high - the offset arithmetic, the byte store, the insert and the wake are
/// literal.
pub unsafe fn fn_8027ff60(table: usize, node: usize, index: i32) {
    let token = call0_r(FUN_801B1140);
    platform::write_u8(node + 0x0c, 1);

    let target = table + index as usize * 0x0c;
    let mut key = [(target + 4) as u32];
    platform::call3(
        FUN_80229160,
        target as u32,
        key.as_mut_ptr() as u32,
        (node + 4) as u32,
    );
    platform::call1(FUN_801B5D80, (table + 0x2c) as u32);
    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// Wave 95 constants
// ===========================================================================

/// `0x802290C0` - unlink a node from a list and return it through the out word.
const FUN_802290C0: usize = 0x8022_90C0;

/// `0x80267300` - register a source object against a sequence object.
const FUN_80267300: usize = 0x8026_7300;

/// `0x80267310` - deregister a sequence object from its source.
const FUN_80267310: usize = 0x8026_7310;

/// `0x80270060` - release one JAS sequence object.
const FUN_80270060: usize = 0x8027_0060;

/// `0x80266230` - drop a stale JAS handle.
const FUN_80266230: usize = 0x8026_6230;

/// `0x802666A0` - move a JAS handle back to its free list.
const FUN_802666A0: usize = 0x8026_66A0;

/// `0x8026DB00` - unlink a voice from a list.
const FUN_8026DB00: usize = 0x8026_DB00;

/// `0x8027C910` - second copy of the block destructor at `0x8027C8B0`.
const FUN_8027C910: usize = 0x8027_C910;

/// `0x8027BCC0` - fixed-slot allocator claim ([`fn_8027bcc0`]).
const FUN_8027BCC0: usize = 0x8027_BCC0;

/// `0x8027BDB0` - fixed-slot allocator release ([`fn_8027bdb0`]).
const FUN_8027BDB0: usize = 0x8027_BDB0;

/// `0x8027B360` - sequence subsystem start-up, called by [`fn_8027b290`].
const FUN_8027B360: usize = 0x8027_B360;

/// `0x801B2E20` - initialise a mutex in place.
const FUN_801B2E20: usize = 0x801B_2E20;

/// `0x802819D0` - per-voice play position of one element.
const FUN_802819D0: usize = 0x8028_19D0;

/// `0x802819F0` - set one voice parameter from a slot word and a context.
const FUN_802819F0: usize = 0x8028_19F0;

/// `0x80281A70` - set the second voice parameter from a slot word.
const FUN_80281A70: usize = 0x8028_1A70;

/// `0x80281AF0` - arm or release one element's voice set.
const FUN_80281AF0: usize = 0x8028_1AF0;

/// `0x80281B80` - push one voice's start offset and time.
const FUN_80281B80: usize = 0x8028_1B80;

/// `0x80281C00` - stop or restart one element's voices.
const FUN_80281C00: usize = 0x8028_1C00;

/// `0x802830A0` - bind a voice with a callback and a user word.
const FUN_802830A0: usize = 0x8028_30A0;

/// `0x80282EF0` - the voice-binding manager.
const FUN_80282EF0: usize = 0x8028_2EF0;

/// `0x80282FE0` - read the manager's per-channel start time.
const FUN_80282FE0: usize = 0x8028_2FE0;

/// `0x80282FF0` - store the manager's per-channel start time.
const FUN_80282FF0: usize = 0x8028_2FF0;

/// `0x80264AE0` - the track-clock manager.
const FUN_80264AE0: usize = 0x8026_4AE0;

/// `0x80264C00` - read a track clock.
const FUN_80264C00: usize = 0x8026_4C00;

/// `0x80264C10` - store a track clock.
const FUN_80264C10: usize = 0x8026_4C10;

/// `0x80267C40` - the sequence-clock manager.
const FUN_80267C40: usize = 0x8026_7C40;

/// `0x80267D20` - read a sequence clock.
const FUN_80267D20: usize = 0x8026_7D20;

/// `0x80267D30` - store a sequence clock.
const FUN_80267D30: usize = 0x8026_7D30;

/// `0x801A3360` - the audio engine's millisecond tick source.
const FUN_801A3360: usize = 0x801A_3360;

/// `0x8027EEF0` - per-element binding callback ([`fn_8027eef0`]).
const FUN_8027EEF0: usize = 0x8027_EEF0;

/// `0x8027FE70` - small-element destructor ([`fn_8027fe70`]).
const FUN_8027FE70: usize = 0x8027_FE70;

/// `0x8034F1A0` - vtable installed at the block's `+0x00` word.
const VT_SND_ELEMENT_BLOCK: u32 = 0x8034_F1A0;

/// `0x8034F1B0` - the base destructor slot of [`VT_SND_ELEMENT_BLOCK`].
const FUN_BASE_DTOR: usize = 0x8034_F1B0;

/// `0x8034F1C4` - second vtable installed at the block's `+0xBC` word.
const VT_SND_ELEMENT_BLOCK_B: u32 = 0x8034_F1C4;

/// `0x8034F200` - third vtable installed at the block's `+0x154` word.
const VT_SND_ELEMENT_BLOCK_C: u32 = 0x8034_F200;

/// `0x8027F150` - element-array destructor callback registered at `+0x184`.
const LAB_8027F150: u32 = 0x8027_F150;

/// `0x80416FA0` - the block class's static mutex.
const DAT_80416FA0: usize = 0x8041_6FA0;

/// `0x80416FC4` - the small-element registry returned by [`fn_8027fdd0`].
const ELEMENT_REGISTRY: usize = 0x8041_6FC4;

/// `0x80416FE8` / `0x80416FEC` - registry count and first-link words.
const ELEMENT_REGISTRY_COUNT: usize = 0x8041_6FE8;
const ELEMENT_REGISTRY_LINK: usize = 0x8041_6FEC;

/// `0x80416FF0` / `0x80416FF8` - the registry's two list headers.
const ELEMENT_REGISTRY_LIST_A: usize = 0x8041_6FF0;
const ELEMENT_REGISTRY_LIST_B: usize = 0x8041_6FF8;

/// `0x80416FB8` - destructor registration block for the small elements.
const DAT_80416FB8: usize = 0x8041_6FB8;

/// `0x8027FF40` - element destructor shim registered with `0x802DC9A0`.
const LAB_8027FF40: u32 = 0x8027_FF40;

/// `0x803FD600` - the audio DMA buffer handed to the start-up routine.
const DAT_803FD600: u32 = 0x803F_D600;

/// `r13 - 0x4930` = `0x8042AD50`: "the element block class is initialised".
const BLOCK_CLASS_READY: usize = crate::boot::entry::SDA1_BASE - 0x4930;

/// `r13 - 0x4920` = `0x8042AD60`: "the small-element registry is initialised".
const ELEMENT_REGISTRY_READY: usize = crate::boot::entry::SDA1_BASE - 0x4920;

/// `0x8042EAE8` = `r2 - 0x4598`: small-data default written at `+0xB88 + 0x38n`.
const SDA2_BLOCK_DEFAULT_A: usize = 0x8042_EAE8;

/// `0x8042EAD0` = `r2 - 0x45B0`: second default, written next to
/// [`SDA2_BLOCK_DEFAULT_A`].
const SDA2_BLOCK_DEFAULT_B: usize = 0x8042_EAD0;

/// `0x5253544D` - ASCII `RSTM`, the stream section header tested by
/// [`fn_8027c210`].
const RSTM_MAGIC: u32 = 0x5253_544D;

/// The minimum accepted `RSTM` version word (`0x0100`).
const RSTM_MIN_VERSION: u32 = 0x100;

// ===========================================================================
// 0x802700F0
// ===========================================================================

/// Difference between a JAS sequence object's two cursor words.
///
/// `+0x10` and `+0x14` are advanced by different paths; the difference is the
/// object's remaining length. [`fn_8027b010`] hands the object to
/// `0x80270060`, which is why the pair is read here.
///
/// # Original
/// - Ghidra: `FUN_802700f0` at `0x802700F0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802700f0_FUN_802700f0.c`
/// - observed: `return *(int *)(param_1 + 0x10) - *(int *)(param_1 + 0x14);`
///
/// # Arguments
/// - `this`: JAS sequence object.
///
/// # Returns
/// `*(i32 *)(this + 0x10) - *(i32 *)(this + 0x14)`.
///
/// # Safety
/// `this + 0x10` must be readable for 8 bytes.
///
/// # Confidence
/// high - the two loads and the subtraction are literal; only the naming of the
/// cursor pair is inferred from [`fn_8027b010`].
pub unsafe fn fn_802700f0(this: usize) -> i32 {
    platform::read_i32(this + 0x10).wrapping_sub(platform::read_i32(this + 0x14))
}

// ===========================================================================
// 0x80270100
// ===========================================================================

/// Stores a caller word at a JAS sequence object's `+0x04`.
///
/// Called by [`fn_8027b010`] right after the object is unlinked from its list,
/// so the stored word is the object's new owner link.
///
/// # Original
/// - Ghidra: `FUN_80270100` at `0x80270100` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270100_FUN_80270100.c`
/// - observed: `*(undefined4 *)(param_1 + 4) = param_2;`
///
/// # Arguments
/// - `this`: JAS sequence object.
/// - `value`: word to store.
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - one store.
pub unsafe fn fn_80270100(this: usize, value: u32) {
    platform::write_u32(this + 4, value);
}

// ===========================================================================
// 0x80270110
// ===========================================================================

/// Clears a JAS sequence object's `+0x04` owner link.
///
/// Called by [`fn_8027b0c0`] before the object is linked back into a list; the
/// export still shows the caller's second word, but the callee ignores it and
/// stores a constant zero.
///
/// # Original
/// - Ghidra: `FUN_80270110` at `0x80270110` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270110_FUN_80270110.c`
/// - observed: `*(undefined4 *)(param_1 + 4) = 0;`
///
/// # Arguments
/// - `this`: JAS sequence object.
///
/// # Safety
/// `this + 4` must be writable.
///
/// # Confidence
/// high - one constant store; the call site's second argument is unused.
pub unsafe fn fn_80270110(this: usize) {
    platform::write_u32(this + 4, 0);
}

// ===========================================================================
// 0x80270310
// ===========================================================================

/// Reports a JAS object's state byte, or zero when the object is disabled.
///
/// The first byte is the enable flag; `+0x02` carries the state that callers
/// poll. A disabled object always reports zero, so the two-value read is kept
/// as one predicate.
///
/// # Original
/// - Ghidra: `FUN_80270310` at `0x80270310` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270310_FUN_80270310.c`
/// - observed: `if (*param_1 != '\0') { return param_1[2]; } return '\0';`
///
/// # Arguments
/// - `this`: JAS object.
///
/// # Returns
/// The byte at `+0x02`, or `0` when the byte at `+0x00` is zero.
///
/// # Safety
/// `this` must be readable for 3 bytes.
///
/// # Confidence
/// high - both byte loads and the guard are literal.
pub unsafe fn fn_80270310(this: usize) -> u8 {
    if platform::read_u8(this) != 0 {
        platform::read_u8(this + 2)
    } else {
        0
    }
}

// ===========================================================================
// 0x8027B010
// ===========================================================================

/// Unlinks a JAS sequence object from the manager's list and returns it.
///
/// The list header sits at `+0x18` and its key word at `+0x1C`; the unlinked
/// node is `0x18` bytes into the object, so the object base is
/// `key - 0x18`. The object then gets the caller's word at `+0x04`, is
/// registered with `0x80267300`, and is released through `0x80270060`. All of
/// it happens under the sequence-manager mutex. An empty list returns `0`.
///
/// # Original
/// - Ghidra: `FUN_8027b010` at `0x8027B010` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b010_FUN_8027b010.c`
/// - observed: the `FUN_8027b5f0()` / `FUN_801b2e60(manager + 0x354)` pair,
///   `if (*(int *)(param_1 + 0x18) == 0)`, `local_18[0] = *(int *)(param_1 +
///   0x1c); iVar2 = local_18[0] + -0x18;`,
///   `FUN_802290c0(param_1 + 0x18,local_18)`, `FUN_80270100(iVar2,param_2)`,
///   `FUN_80267300(param_2,iVar2)`, `FUN_80270060(iVar2)` and the second
///   `FUN_801b2f40`.
/// - `// inferred:` the node is embedded `0x18` bytes into the object, because
///   the same `+0x18` header is used by [`fn_8027b0c0`] as the insert list.
///
/// # Arguments
/// - `list`: object owning the `+0x18` list header.
/// - `value`: word stored at the unlinked object's `+0x04`.
///
/// # Returns
/// The unlinked object's address, or `0` when the list was empty.
///
/// # Side effects
/// - takes and releases the sequence-manager mutex,
/// - removes one node, registers the object and releases it.
///
/// # Safety
/// `list` must be a live list owner and the manager must be initialised.
///
/// # Confidence
/// high - every load, store and call is literal; only the object node offset
/// is inferred.
pub unsafe fn fn_8027b010(list: usize, value: u32) -> u32 {
    lock_seq_manager();

    let result;
    if platform::read_u32(list + 0x18) == 0 {
        unlock_seq_manager();
        result = 0;
    } else {
        let mut key = [platform::read_u32(list + 0x1c)];
        let object = key[0].wrapping_sub(0x18);
        platform::call2(FUN_802290C0, (list + 0x18) as u32, key.as_mut_ptr() as u32);
        fn_80270100(object as usize, value);
        platform::call2(FUN_80267300, value, object);
        platform::call1(FUN_80270060, object);
        unlock_seq_manager();
        result = object;
    }
    result
}

// ===========================================================================
// 0x8027B0C0
// ===========================================================================

/// Links a JAS sequence object back into the manager's list.
///
/// The counterpart of [`fn_8027b010`]: the object is read from the caller's
/// `+0x04` word, its own `+0x04` link is cleared with [`fn_80270110`], it is
/// deregistered with `0x80267310`, and its `+0x18` node is inserted into the
/// list at `+0x18` using the key word at `+0x1C`. The mutex is held throughout
/// and a zero object is a no-op.
///
/// # Original
/// - Ghidra: `FUN_8027b0c0` at `0x8027B0C0` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b0c0_FUN_8027b0c0.c`
/// - observed: `iVar1 = *(int *)(param_2 + 4);`, the null split,
///   `FUN_80270110(iVar1,param_2)`, `FUN_80267310(param_2,iVar1)`,
///   `local_18[0] = param_1 + 0x1c;`,
///   `FUN_80229160(param_1 + 0x18,local_18,iVar1 + 0x18)` and the lock pair.
///
/// # Arguments
/// - `list`: object owning the `+0x18` list header.
/// - `holder`: record whose `+0x04` word addresses the object to link.
///
/// # Side effects
/// - clears the object's `+0x04`, deregisters it, inserts its `+0x18` node,
/// - takes and releases the sequence-manager mutex.
///
/// # Safety
/// `list` and `holder` must be live and the object must be free-standing.
///
/// # Confidence
/// high - the guard, the two helper calls and the insert are literal.
pub unsafe fn fn_8027b0c0(list: usize, holder: usize) {
    lock_seq_manager();

    let object = platform::read_u32(holder + 4) as usize;
    if object == 0 {
        unlock_seq_manager();
        return;
    }

    fn_80270110(object);
    platform::call2(FUN_80267310, holder as u32, object as u32);
    let mut key = [(list + 0x1c) as u32];
    platform::call3(
        FUN_80229160,
        (list + 0x18) as u32,
        key.as_mut_ptr() as u32,
        (object + 0x18) as u32,
    );
    unlock_seq_manager();
}

// ===========================================================================
// 0x8027B160
// ===========================================================================

/// Probes a source object for a playable sequence and clears the stale handle.
///
/// The first virtual slot used is `vtable + 0x0C`, the same "create sequence"
/// slot [`fn_8027b1c0`] calls with arguments. A zero result means the source
/// has no usable sequence: the caller's handle word is dropped through
/// `0x80266230` when it is non-zero and `0` is returned.
///
/// # Original
/// - Ghidra: `FUN_8027b160` at `0x8027B160` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b160_FUN_8027b160.c`
/// - observed: `iVar1 = (**(code **)(*param_1 + 0xc))();`, the
///   `if (iVar1 == 0)` split, `if (*param_2 != 0) { FUN_80266230(); }` and
///   `return iVar1;`.
/// - `// observed:` the export prints the virtual call with no argument list,
///   so none are forwarded; the slot's arguments are not recoverable here.
///
/// # Arguments
/// - `object`: source object whose vtable is used.
/// - `holder`: record whose `+0x00` word is the handle to drop on failure.
///
/// # Returns
/// The virtual call's result, or `0` after the handle was dropped.
///
/// # Safety
/// `object` must have a readable vtable and `holder` must be readable.
///
/// # Confidence
/// medium - the guard and the drop are literal; the virtual callee's arguments
/// are not visible in the export.
pub unsafe fn fn_8027b160(object: usize, holder: usize) -> u32 {
    let vtable = platform::read_u32(object) as usize;
    let result = call0_r(platform::read_u32(vtable + 0x0c) as usize);
    if result == 0 {
        if platform::read_u32(holder) != 0 {
            platform::call0(FUN_80266230);
        }
        0
    } else {
        result
    }
}

// ===========================================================================
// 0x8027B1C0
// ===========================================================================

/// Reuses a cached sequence when its id matches, else creates a new one.
///
/// The holder's current handle is compared against the requested id at
/// `handle + 0x9C`. A match only drops the cached handle (`0x802666A0`) and
/// reports `0`. Otherwise the five-argument virtual slot `vtable + 0x0C` runs;
/// a zero result drops both the stale holder word and the returned handle.
///
/// # Original
/// - Ghidra: `FUN_8027b1c0` at `0x8027B1C0` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b1c0_FUN_8027b1c0.c`
/// - observed: `iVar1 = *param_2;`,
///   `if ((iVar1 == 0) || (param_3 != *(int *)(iVar1 + 0x9c)))`,
///   `iVar1 = (**(code **)(*param_1 + 0xc))(param_1,param_2,param_3,1,param_4);`,
///   `FUN_80266230()`, `FUN_802666a0(*param_2,1)` and `FUN_802666a0(iVar1,1)`.
/// - `// observed:` the unreachable block at `0x8027B1EC` is dropped by the
///   export and is not reproduced.
///
/// # Arguments
/// - `object`: source object whose vtable is used.
/// - `holder`: record holding the cached handle at `+0x00`.
/// - `id`: requested sequence id.
/// - `extra`: fifth argument forwarded to the virtual slot.
///
/// # Returns
/// The virtual call's result, or `0` on the cache-hit and failure paths.
///
/// # Side effects
/// - drops handles through `0x802666A0` / `0x80266230`.
///
/// # Safety
/// `object` must have a readable vtable and `holder` must be readable.
///
/// # Confidence
/// medium - the guard, the five arguments and both drop paths are literal; the
/// meaning of `id` is inferred from the `+0x9C` comparison.
pub unsafe fn fn_8027b1c0(object: usize, holder: usize, id: u32, extra: u32) -> u32 {
    let current = platform::read_u32(holder) as usize;
    if current != 0 && id == platform::read_u32(current + 0x9c) {
        platform::call2(FUN_802666A0, current as u32, 1);
        return 0;
    }

    let vtable = platform::read_u32(object) as usize;
    let result = platform::call5(
        platform::read_u32(vtable + 0x0c) as usize,
        object as u32,
        holder as u32,
        id,
        1,
        extra,
    );
    if result == 0 {
        if platform::read_u32(holder) != 0 {
            platform::call0(FUN_80266230);
        }
        platform::call2(FUN_802666A0, platform::read_u32(holder), 1);
    }
    result
}

// ===========================================================================
// 0x8027B270
// ===========================================================================

/// Releases a source object through its virtual slot `+0x0C`.
///
/// The export prints the call with no argument list and no return value, and
/// warns that the indirect branch was treated as a call, so this is a bare
/// dispatch on the object's own vtable.
///
/// # Original
/// - Ghidra: `FUN_8027b270` at `0x8027B270` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b270_FUN_8027b270.c`
/// - observed: `(**(code **)(*param_1 + 0xc))();` under
///   `/* WARNING: Treating indirect jump as call */`.
///
/// # Arguments
/// - `object`: source object whose vtable is used.
///
/// # Safety
/// `object` must have a readable vtable whose `+0x0C` slot is callable.
///
/// # Confidence
/// low - the dispatch offset is literal; neither the arguments nor the return
/// value are visible, so none are claimed.
pub unsafe fn fn_8027b270(object: usize) {
    let vtable = platform::read_u32(object) as usize;
    platform::call0(platform::read_u32(vtable + 0x0c) as usize);
}

// ===========================================================================
// 0x8027B290
// ===========================================================================

/// Starts the sequence subsystem with two caller words and a fixed buffer.
///
/// A four-word descriptor is built on the stack: the two caller words in slots
/// 0 and 2, and the constant `0x4000` in slots 1 and 3. The descriptor is
/// handed to `0x8027B360` together with the audio buffer at `0x803FD600` and
/// its size `0x155E0`.
///
/// # Original
/// - Ghidra: `FUN_8027b290` at `0x8027B290` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b290_FUN_8027b290.c`
/// - observed: `local_14 = 0x4000; local_c = 0x4000; local_18 = param_1;
///   local_10 = param_2;` then
///   `FUN_8027b360(&local_18,&DAT_803fd600,0x155e0);`
/// - `// observed:` `0x8027B360` is declared `void` in the export, so its real
///   argument list is invisible; the three printed arguments are forwarded.
///
/// # Arguments
/// - `first`: first caller word (descriptor slot 0).
/// - `second`: second caller word (descriptor slot 2).
///
/// # Side effects
/// - runs the one-shot `0x8027B360` start-up.
///
/// # Safety
/// `0x8027B360` must accept a pointer to the four-word descriptor; the buffer
/// at `0x803FD600` must be the audio DMA area.
///
/// # Confidence
/// medium - the descriptor stores and the call are literal; the callee's
/// argument list is inferred from the printed call.
pub unsafe fn fn_8027b290(first: u32, second: u32) {
    let mut descriptor = [first, 0x4000, second, 0x4000];
    platform::call3(
        FUN_8027B360,
        descriptor.as_mut_ptr() as u32,
        DAT_803FD600,
        0x1_55e0,
    );
}

// ===========================================================================
// 0x8027B2E0
// ===========================================================================

/// Sums five subsystem clocks into one elapsed-time word.
///
/// The tick source `0x801A3360` is read once and then used as the reference for
/// three clock readers (channel, track and sequence), each queried twice: once
/// for its start time and once for its stored value. The result adds the block's
/// own `+0x04` and `+0x0C` words.
///
/// # Original
/// - Ghidra: `FUN_8027b2e0` at `0x8027B2E0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b2e0_FUN_8027b2e0.c`
/// - observed: `uVar2 = FUN_801a3360();`, the
///   `FUN_80282ef0()` / `FUN_80282fe0(uVar3,uVar2)`,
///   `FUN_80264ae0()` / `FUN_80264c00(uVar3,uVar2)` and
///   `FUN_80267c40()` / `FUN_80267d20(uVar3,uVar2)` pairs,
///   `iVar1 = *(int *)(param_1 + 0xc); iVar7 = *(int *)(param_1 + 4);` and
///   `return iVar6 + iVar5 + iVar7 + iVar4 + iVar1;`.
///
/// # Arguments
/// - `block`: object whose `+0x04` and `+0x0C` words are added in.
///
/// # Returns
/// The five-term sum.
///
/// # Safety
/// `block` must be readable for `0x10` bytes and the three managers must be
/// initialised.
///
/// # Confidence
/// medium - every call and add is literal; the "clock" naming of the three
/// manager pairs is inferred from their use in [`fn_8027b360`].
pub unsafe fn fn_8027b2e0(block: usize) -> u32 {
    let tick = call0_r(FUN_801A3360);

    let channel = platform::call2(FUN_80282FE0, call0_r(FUN_80282EF0), tick);
    let track = platform::call2(FUN_80264C00, call0_r(FUN_80264AE0), tick);
    let sequence = platform::call2(FUN_80267D20, call0_r(FUN_80267C40), tick);

    platform::read_u32(block + 0x0c)
        .wrapping_add(platform::read_u32(block + 4))
        .wrapping_add(channel)
        .wrapping_add(track)
        .wrapping_add(sequence)
}

// ===========================================================================
// 0x8027C210
// ===========================================================================

/// Opens an `RSTM` stream section and hands it to the block decoder.
///
/// The handle's `+0x00` word is the stream; its vtable slot `+0x44` seeks and
/// slot `+0x14` reads. The routine seeks to the start, reads a `0x40`-byte
/// header, checks the `RSTM` magic and the version word at `+0x06`, and rejects
/// the file when the caller's `limit` is below the total word at `+0x18`.
/// It then re-reads `header[0x10] + header[0x14]` bytes into `out` and, when
/// that second header also validates, records `out` at `+0x04`, the section
/// pointer at `+0x08` and resolves the section with `0x80283A60`.
///
/// # Original
/// - Ghidra: `FUN_8027c210` at `0x8027C210` (452 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c210_FUN_8027c210.c`
/// - observed: the two `(**(code **)(*(int *)*param_1 + 0x44))` seeks, the two
///   `(**(code **)(*(int *)*param_1 + 0x14))` reads, the `iVar2 == 0x40` and
///   `iVar2 == iStack_50 + iStack_4c` length tests, `iStack_60 == 0x5253544d`,
///   the version test, `param_3 < uStack_48`, `param_1[1] = param_2;`,
///   `iVar2 = param_2[4] + (int)param_2; param_1[2] = iVar2;` and the
///   `FUN_80283a60(*(undefined1 *)(iVar2 + 8),*(undefined4 *)(iVar2 + 0xc));`.
/// - `// observed:` the export returns `1` whenever the second read length
///   matches, even when the second magic check fails; that is reproduced.
///
/// # Arguments
/// - `handle`: two-word handle; `+0x00` is the stream, `+0x04`/`+0x08` are the
///   two output words filled on success.
/// - `out`: buffer that receives the whole section.
/// - `limit`: caller's size budget, compared with the header's `+0x18` word.
///
/// # Returns
/// `1` when the section was read, `0` on any header or length failure.
///
/// # Side effects
/// - two seeks and two reads on the stream,
/// - writes `handle + 0x04`, `handle + 0x08` and calls `0x80283A60`.
///
/// # Safety
/// `out` must be writable for `header[0x10] + header[0x14]` bytes and `handle`
/// must be readable and writable for `0x0C` bytes.
///
/// # Confidence
/// high - every load, guard, seek/read and store is literal; only the header
/// field roles are inferred from the offsets.
pub unsafe fn fn_8027c210(handle: usize, out: usize, limit: u32) -> u32 {
    let stream = platform::read_u32(handle) as usize;
    let vtable = platform::read_u32(stream) as usize;
    let seek = platform::read_u32(vtable + 0x44) as usize;
    let read = platform::read_u32(vtable + 0x14) as usize;

    platform::call3(seek, stream as u32, 0, 0);

    let mut header = [0u32; 16];
    let base = header.as_mut_ptr() as usize;
    if platform::call3(read, stream as u32, base as u32, 0x40) != 0x40 {
        return 0;
    }
    if header[0] != RSTM_MAGIC {
        return 0;
    }
    if (platform::read_u16(base + 6) as u32) < RSTM_MIN_VERSION {
        return 0;
    }
    if limit < header[6] {
        return 0;
    }

    platform::call3(seek, stream as u32, 0, 0);
    let size = header[4].wrapping_add(header[5]);
    if platform::call3(read, stream as u32, out as u32, size) != size {
        return 0;
    }

    if platform::read_u32(out) == RSTM_MAGIC
        && (platform::read_u16(out + 6) as u32) >= RSTM_MIN_VERSION
    {
        platform::write_u32(handle + 4, out as u32);
        let section = out.wrapping_add(platform::read_u32(out + 0x10) as usize);
        platform::write_u32(handle + 8, section as u32);
        platform::call2(
            FUN_80283A60,
            platform::read_u8(section + 8) as u32,
            platform::read_u32(section + 0xc),
        );
    }
    1
}

// ===========================================================================
// 0x8027C4B0
// ===========================================================================

/// Copies the record selected by an index out of a type-2 section.
///
/// The handle must have a section at `+0x04`. `0x80283A60` resolves the
/// section's `+0x08`/`+0x0C` pair to a header, and when that header's first byte
/// is `2` the routine resolves the sub-table at header `+0x18`/`+0x1C`, checks
/// the index against the sub-table's count byte, then resolves the entry and its
/// pointed-to record. The record's nine words and seven half-words are copied
/// out. A wrong type byte still reports success, exactly as the export does.
///
/// # Original
/// - Ghidra: `FUN_8027c4b0` at `0x8027C4B0` (332 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c4b0_FUN_8027c4b0.c`
/// - observed: `if (*(int *)(param_1 + 4) == 0)` early exit, the three
///   `FUN_80283a60(...)` resolutions, `if (*pcVar2 == '\x02')`,
///   `if (param_4 < (int)(uint)*pbVar3)`, the eight `param_2[n]` word pairs,
///   the four half-word stores at `+0x20..+0x26`, the three `param_3` stores and
///   `uVar1 = 1;` outside the type test.
///
/// # Arguments
/// - `handle`: record handle; `+0x04` gates the read and `+0x08` is the section
///   pointer.
/// - `out`: nine-word record destination.
/// - `tail`: three half-word destination.
/// - `index`: entry index inside the sub-table.
///
/// # Returns
/// `1` when a section is present, `0` when `+0x04` is zero.
///
/// # Side effects
/// - writes `out[0..9]`, the four half-words at `out + 0x20..0x27` and
///   `tail[0..3]` when the section is type `2`.
///
/// # Safety
/// `handle` must be a live handle and `out`/`tail` must be writable for
/// `0x28` / 6 bytes.
///
/// # Confidence
/// medium - every resolution and store is literal; the record and sub-table
/// roles are inferred from the offsets.
pub unsafe fn fn_8027c4b0(handle: usize, out: usize, tail: usize, index: i32) -> u32 {
    if platform::read_u32(handle + 4) == 0 {
        return 0;
    }

    let section = platform::read_u32(handle + 8) as usize;
    let header = platform::call2(
        FUN_80283A60,
        platform::read_u8(section + 8) as u32,
        platform::read_u32(section + 0xc),
    ) as usize;

    if platform::read_u8(header) == 2 {
        let table = platform::call3(
            FUN_80283A60,
            platform::read_u8(section + 0x18) as u32,
            platform::read_u32(section + 0x1c),
            (section + 8) as u32,
        ) as usize;

        let count = platform::read_u8(table) as i32;
        if index < count {
            let entry_offset = (index as usize) * 8;
            let entry = platform::call3(
                FUN_80283A60,
                platform::read_u8(table + entry_offset + 4) as u32,
                platform::read_u32(table + entry_offset + 8),
                (section + 8) as u32,
            ) as usize;
            let record = platform::call3(
                FUN_80283A60,
                platform::read_u32(entry),
                platform::read_u32(entry + 4),
                (section + 8) as u32,
            ) as usize;

            let mut word = 0usize;
            while word < 8 {
                platform::write_u32(out + word * 4, platform::read_u32(record + word * 4));
                word += 1;
            }
            for offset in [0x20usize, 0x22, 0x24, 0x26] {
                platform::write_u16(out + offset, platform::read_u16(record + offset));
            }
            for offset in [0x00usize, 0x02, 0x04] {
                platform::write_u16(tail + offset, platform::read_u16(record + 0x28 + offset));
            }
        }
    }

    1
}

// ===========================================================================
// 0x8027C600
// ===========================================================================

/// Reads one block of interleaved half-words and splits them into two arrays.
///
/// The handle's `+0x04` word is the section state; its `+0x18` word is the base
/// offset, and `row * count * 4 + 8` selects the block inside it. After a
/// `0x20`-byte read the export copies four pairs at a time, taking the first
/// half-word of every pair to `first` and the second to `second`, each at a
/// two-byte stride.
///
/// # Original
/// - Ghidra: `FUN_8027c600` at `0x8027C600` (436 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c600_FUN_8027c600.c`
/// - observed: `uVar3 = param_1[1];`, the `-1 < (int)(-uVar3 | uVar3)` early
///   return (a zero test in disguise), the vtable `+0x44` seek to
///   `*(int *)(uVar3 + 0x18) + param_4 * param_5 * 4 + 8`, the vtable `+0x14`
///   read of `0x20` bytes, the four-pair unrolled copy with `iVar1 += 0x10` /
///   `iVar2 += 8` and the `param_5 & 3` remainder loop.
/// - `// observed:` the unrolled body and the remainder loop are collapsed into
///   one index loop here; the stores are identical.
///
/// # Arguments
/// - `handle`: record handle; `+0x00` is the stream and `+0x04` the section
///   state.
/// - `first`: destination of the first half-word of each pair.
/// - `second`: destination of the second half-word of each pair.
/// - `row`: block row index.
/// - `count`: number of pairs to copy.
///
/// # Returns
/// `1` when `0x20` bytes were read, `0` otherwise.
///
/// # Side effects
/// - one seek and one read, then `count` half-word stores into each array.
///
/// # Safety
/// `first` and `second` must be writable for `count * 2` bytes each; the
/// section state must be live.
///
/// # Confidence
/// high - the seek arithmetic, the read length and the pair split are literal;
/// the unrolled loop is reproduced as an equivalent index loop.
pub unsafe fn fn_8027c600(
    handle: usize,
    first: usize,
    second: usize,
    row: u32,
    count: u32,
) -> u32 {
    let state = platform::read_u32(handle + 4);
    if state == 0 {
        return 0;
    }

    let stream = platform::read_u32(handle) as usize;
    let vtable = platform::read_u32(stream) as usize;
    let seek = platform::read_u32(vtable + 0x44) as usize;
    let read = platform::read_u32(vtable + 0x14) as usize;

    let target = platform::read_u32(state as usize + 0x18)
        .wrapping_add(row.wrapping_mul(count).wrapping_mul(4))
        .wrapping_add(8);
    platform::call3(seek, stream as u32, target, 0);

    let mut buffer = [0u16; 16];
    if platform::call3(read, stream as u32, buffer.as_mut_ptr() as u32, 0x20) != 0x20 {
        return 0;
    }

    let mut index = 0usize;
    while index < count as usize {
        platform::write_u16(first + index * 2, buffer[index * 2]);
        platform::write_u16(second + index * 2, buffer[index * 2 + 1]);
        index += 1;
    }
    1
}

// ===========================================================================
// 0x8027C7C0
// ===========================================================================

/// Constructs the sound element block that owns eight element records.
///
/// The constructor installs three vtables (`+0x00`, `+0xBC`, `+0x154`), clears
/// the state bytes and the `+0xB4`/`+0xB8` link words, makes the two-word list
/// at `+0x178` point at itself, registers the element array at `+0x184` with
/// `0x802DC9A0` (element size `0x34`, count `0x20`), and - on first use -
/// initialises the class mutex at `0x80416FA0`. `0x8026D860` then links the
/// array at `+0x180` with its storage.
///
/// # Original
/// - Ghidra: `FUN_8027c7c0` at `0x8027C7C0` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c7c0_FUN_8027c7c0.c`
/// - observed: `FUN_80265e80();`, `*param_1 = &DAT_8034f1a0;`,
///   `param_1[0x2f] = &DAT_8034f1c4;`, `param_1[0x55] = &DAT_8034f200;`, the
///   `param_1[0x5e] = param_1 + 0x5e;` / `param_1[0x5f] = param_1 + 0x5e;`
///   self-links, `FUN_802dc9a0(param_1 + 0x61,&LAB_8027f150,FUN_8027c910,0x34,
///   0x20)`, the `unaff_r13 + -0x4930` flag, `FUN_801b2e20(&DAT_80416fa0)` and
///   `FUN_8026d860(param_1 + 0x60,param_1 + 0x61,0x680,0x34)`.
///
/// # Arguments
/// - `this`: block to construct; writable for `0x80C` bytes.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - writes the whole header, installs three vtables, registers the element
///   array, and may initialise the class mutex.
///
/// # Safety
/// `this` must be zeroed storage owned by the sound system.
///
/// # Confidence
/// high - every store, the flag test and all three calls are literal; the
/// "element block" naming is inferred from [`fn_8027ca70`]'s use of the same
/// fields.
pub unsafe fn fn_8027c7c0(this: usize) -> u32 {
    platform::call0(FUN_80265E80);

    platform::write_u32(this, VT_SND_ELEMENT_BLOCK);
    platform::write_u32(this + 0xb4, 0);
    platform::write_u32(this + 0xb8, 0);
    platform::write_u32(this + 0xbc, VT_SND_ELEMENT_BLOCK_B);
    platform::write_u8(this + 0x100, 0);
    platform::write_u8(this + 0x101, 0);
    platform::write_u32(this + 0x158, 0);
    platform::write_u32(this + 0x15c, 0);
    platform::write_u8(this + 0x160, 0);
    platform::write_u32(this + 0x154, VT_SND_ELEMENT_BLOCK_C);
    platform::write_u32(this + 0x164, 0);
    platform::write_u32(this + 0x168, 0);
    platform::write_u32(this + 0x170, 0);
    platform::write_u32(this + 0x174, 0);
    platform::write_u32(this + 0x178, (this + 0x178) as u32);
    platform::write_u32(this + 0x17c, (this + 0x178) as u32);
    platform::write_u32(this + 0x180, 0);
    platform::call5(
        FUN_802DC9A0,
        (this + 0x184) as u32,
        LAB_8027F150,
        FUN_8027C910 as u32,
        0x34,
        0x20,
    );
    platform::write_u32(this + 0x808, 0);

    if platform::read_u8(BLOCK_CLASS_READY) == 0 {
        platform::call1(FUN_801B2E20, DAT_80416FA0 as u32);
        platform::write_u8(BLOCK_CLASS_READY, 1);
    }

    platform::call4(
        FUN_8026D860,
        (this + 0x180) as u32,
        (this + 0x184) as u32,
        0x680,
        0x34,
    );
    this as u32
}

// ===========================================================================
// 0x8027C970
// ===========================================================================

/// Destroys the sound element block and optionally frees it.
///
/// The two vtables and the base destructor are reinstalled/run first, then the
/// block's element array is released under the sequence mutex when `+0x100`
/// is set. `0x802DCA98` unregisters the array's destructor, the two list owners
/// at `+0x174` and `+0x154` are torn down with `0x80229030` / `0x8027FD90`, and
/// a positive `flag` finally frees the block itself.
///
/// # Original
/// - Ghidra: `FUN_8027c970` at `0x8027C970` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027c970_FUN_8027c970.c`
/// - observed: `*param_1 = &DAT_8034f1a0; param_1[0x2f] = &DAT_8034f1c4;`, the
///   `(*(code *)PTR_FUN_8034f1b0)();` dispatch, the `*(char *)(param_1 + 0x40)`
///   lock split with `param_1[0x201] = 0` and
///   `FUN_8026d9b0(param_1 + 0x60,param_1 + 0x61,0x680)`,
///   `FUN_802dca98(param_1 + 0x61,FUN_8027c910,0x34,0x20)`, the
///   `param_1 + 0x5d` / `param_1 + 0x55` null tests and
///   `if (0 < param_2) FUN_802b9280(param_1);`.
/// - `// inferred:` the base destructor at `0x8034F1B0` is forwarded `this`,
///   because every other vtable slot in this module takes the object.
///
/// # Arguments
/// - `this`: block to destroy; may be null, in which case nothing happens.
/// - `flag`: when positive, free the block after the teardown.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - reinstalls two vtables, runs the base destructor, unregisters and tears
///   down the element array and both list owners.
///
/// # Safety
/// `this` must be null or a live block from [`fn_8027c7c0`].
///
/// # Confidence
/// medium - every test, store and call is literal; the base destructor's
/// argument is inferred.
pub unsafe fn fn_8027c970(this: usize, flag: i32) -> u32 {
    if this != 0 {
        platform::write_u32(this, VT_SND_ELEMENT_BLOCK);
        platform::write_u32(this + 0xbc, VT_SND_ELEMENT_BLOCK_B);
        platform::call1(FUN_BASE_DTOR, this as u32);

        lock_seq_manager();
        if platform::read_u8(this + 0x100) == 0 {
            unlock_seq_manager();
        } else {
            platform::write_u32(this + 0x804, 0);
            platform::call3(
                FUN_8026D9B0,
                (this + 0x180) as u32,
                (this + 0x184) as u32,
                0x680,
            );
            platform::write_u8(this + 0x100, 0);
            unlock_seq_manager();
        }

        platform::call4(
            FUN_802DCA98,
            (this + 0x184) as u32,
            FUN_8027C910 as u32,
            0x34,
            0x20,
        );

        if this + 0x174 != 0 {
            platform::call2(FUN_80229030, (this + 0x174) as u32, 0);
        }
        if this + 0x154 != 0 {
            fn_8027fd90(this + 0x154, 0);
        }
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027CA70
// ===========================================================================

/// Arms the sound element block for a track count and a channel bitmap.
///
/// The routine first releases any armed state through the block's own virtual
/// slot `+0x10`, resets the block with [`fn_8027d520`], stores the clamped track
/// count at `+0x810`, and sets one byte per set bit of `bitmap` (at most eight,
/// one `0x38`-byte element each) at element `+0xB58`. The number of set bits,
/// clamped to eight, becomes the element count at `+0x80C`. When that count is
/// zero the call reports `2`. Otherwise the value word goes to `+0x814`, the
/// allocator token to `+0x804`, and one allocator slot is claimed for every
/// element; on the first failure the slots claimed so far are released and the
/// call reports `1`. Success sets element byte `+0x10F`, the armed byte at
/// `+0x100`, and reports `0`.
///
/// # Original
/// - Ghidra: `FUN_8027ca70` at `0x8027CA70` (536 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ca70_FUN_8027ca70.c`
/// - Removed: the three modelled `_restgpr_26` epilogue calls (`0x802DCDBC`);
///   the matching `_savegpr_26` prologue (`0x802DCD70`) was never modelled and
///   each exit now returns its value directly.
/// - observed: the incoming `r3`/`r4` pair (`this` and the allocator token stored
///   at `+0x804`), the `+0x100` guard around the
///   virtual `+0x10` slot and the `0x8026D9B0` release, `FUN_8027d520(piVar2)`,
///   the `param_3 < 0x11` clamp, the `param_4 & 1` / `param_4 >> 1` bitmap walk
///   with its `7 < iVar3` break, the `iVar3 < 9` clamp to `+0x80C`, the
///   `FUN_8027bcc0` / `FUN_8027bdb0` allocation and rollback, the `+0x10F`
///   byte, and the exit values `2` / `1` / `0`.
///
/// # Arguments
/// - `this`: sound element block from [`fn_8027c7c0`] (incoming `r3`).
/// - `token`: allocator base token (incoming `r4`).
/// - `limit`: requested track count, clamped to `0x10`.
/// - `bitmap`: one bit per element to enable, only the low eight are used.
/// - `value`: word stored at `+0x814`.
///
/// # Side effects
/// - releases and re-arms the block, fills `+0x804`, `+0x80C`, `+0x810`,
///   `+0x814`, the element bytes and the allocator slots.
///
/// # Safety
/// `this` must be a live block and `token` must be the allocator base that
/// [`fn_8027bbf0`] was given.
///
/// # Confidence
/// medium - every offset, guard and call is literal; the element and bitmap
/// roles are inferred from [`fn_8027d520`] and [`fn_8027bcc0`].
pub unsafe fn fn_8027ca70(this: usize, token: u32, limit: i32, bitmap: u32, value: u32) -> u32 {
    lock_seq_manager();

    if platform::read_u8(this + 0x100) != 0 {
        let vtable = platform::read_u32(this) as usize;
        platform::call1(platform::read_u32(vtable + 0x10) as usize, this as u32);

        lock_seq_manager();
        if platform::read_u8(this + 0x100) == 0 {
            unlock_seq_manager();
        } else {
            platform::write_u32(this + 0x804, 0);
            platform::call3(
                FUN_8026D9B0,
                (this + 0x180) as u32,
                (this + 0x184) as u32,
                0x680,
            );
            platform::write_u8(this + 0x100, 0);
            unlock_seq_manager();
        }
    }

    fn_8027d520(this);

    let tracks = if limit < 0x11 { limit } else { 0x10 };
    platform::write_u32(this + 0x810, tracks as u32);

    let mut index = 0i32;
    let mut element = this;
    let mut bits = bitmap;
    loop {
        if bits == 0 {
            break;
        }
        if bits & 1 != 0 {
            if index > 7 {
                break;
            }
            platform::write_u8(element + 0xb58, 1);
        }
        bits >>= 1;
        element += 0x38;
        index += 1;
    }

    let slots = if index < 9 { index } else { 8 };
    platform::write_u32(this + 0x80c, slots as u32);

    if slots == 0 {
        unlock_seq_manager();
        return 2;
    }

    platform::write_u32(this + 0x814, value);
    platform::write_u32(this + 0x804, token);
    let lock = call0_r(FUN_801B1140);

    if tracks > 0 {
        let mut claimed = 0i32;
        while claimed < tracks {
            let slot = platform::call1(FUN_8027BCC0, token);
            if slot == 0 {
                let mut release = 0i32;
                while release < claimed {
                    let address = platform::read_u32(this + release as usize * 0x34 + 0x818);
                    platform::call2(FUN_8027BDB0, token, address);
                    platform::write_u32(this + release as usize * 0x34 + 0x818, 0);
                    release += 1;
                }
                platform::call1(FUN_801B1180, lock);
                unlock_seq_manager();
                return 1;
            }
            platform::write_u32(this + claimed as usize * 0x34 + 0x818, slot);
            claimed += 1;
        }
        platform::write_u8(this + 0x10f, 1);
    }

    platform::call1(FUN_801B1180, lock);
    platform::write_u8(this + 0x100, 1);
    unlock_seq_manager();
    0
}

// ===========================================================================
// 0x8027D3A0
// ===========================================================================

/// Reads the block's current play record while it is playing.
///
/// The record is a flag byte plus three words. The flag is the block's `+0xC4`
/// byte folded to `0`/`1`, and the words come from `+0xCC`, `+0xD4` and `+0xD8`.
/// The read is only served while `+0x103` is set; the mutex is released on both
/// paths and the returned value is the same flag the byte received.
///
/// # Original
/// - Ghidra: `FUN_8027d3a0` at `0x8027D3A0` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027d3a0_FUN_8027d3a0.c`
/// - observed: `bVar1 = *(char *)(param_1 + 0x103) != '\0';`, the
///   `if (bVar1)` branch with `uVar5 = *(undefined4 *)(param_1 + 0xcc);`,
///   `uVar4 = *(undefined4 *)(param_1 + 0xd4);`,
///   `uVar2 = *(undefined4 *)(param_1 + 0xd8);` and
///   `*param_2 = (byte)(-(uint)*(byte *)(param_1 + 0xc4) >> 0x1f);`.
///
/// # Arguments
/// - `this`: sound element block.
/// - `out`: `0x10`-byte record destination.
///
/// # Returns
/// `1` when the record was written, `0` when the block is not playing.
///
/// # Side effects
/// - takes and releases the sequence-manager mutex.
///
/// # Safety
/// `out` must be writable for `0x10` bytes.
///
/// # Confidence
/// high - the guard, the three loads and the four stores are literal.
pub unsafe fn fn_8027d3a0(this: usize, out: usize) -> u32 {
    lock_seq_manager();

    let playing = platform::read_u8(this + 0x103) != 0;
    if playing {
        let first = platform::read_u32(this + 0xcc);
        let second = platform::read_u32(this + 0xd4);
        let third = platform::read_u32(this + 0xd8);

        // `-(u32)byte >> 31` is 1 for every non-zero byte, 0 for zero.
        let flag = (-(platform::read_u8(this + 0xc4) as i32)) as u32 >> 31;
        platform::write_u8(out, flag as u8);
        platform::write_u32(out + 4, first);
        platform::write_u32(out + 8, second);
        platform::write_u32(out + 0xc, third);
    }

    unlock_seq_manager();
    playing as u32
}

// ===========================================================================
// 0x8027D440
// ===========================================================================

/// Reports the block's current position in its playing sequence.
///
/// Three guards select the result: `+0x101` and the first element's `+0xB58`
/// byte both report `-1` (not armed), while `+0x103` clear reports `0` (armed
/// but not started). Otherwise the position is the element's voice position
/// (`0x802819D0`, called only when `+0xB5C` is set) plus
/// `+0xE8 * (+0x13C - +0x138)`.
///
/// # Original
/// - Ghidra: `FUN_8027d440` at `0x8027D440` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027d440_FUN_8027d440.c`
/// - observed: the `+0x101` / `+0xb58` / `+0x103` chain, `iVar1 = -1`,
///   `iVar1 = 0`, `if (*(int *)(param_1 + 0xb5c) != 0) { iVar1 =
///   FUN_802819d0(); }` and
///   `iVar1 = *(int *)(param_1 + 0xe8) * (*(int *)(param_1 + 0x13c) -
///   *(int *)(param_1 + 0x138)) + iVar1;`.
///
/// # Arguments
/// - `this`: sound element block.
///
/// # Returns
/// `-1`, `0`, or the computed position.
///
/// # Side effects
/// - takes and releases the sequence-manager mutex; may query a voice.
///
/// # Safety
/// `this` must be a live block.
///
/// # Confidence
/// high - all three guards and the final arithmetic are literal.
pub unsafe fn fn_8027d440(this: usize) -> i32 {
    lock_seq_manager();

    if platform::read_u8(this + 0x101) == 0 {
        unlock_seq_manager();
        return -1;
    }
    if platform::read_u8(this + 0xb58) == 0 {
        unlock_seq_manager();
        return -1;
    }
    if platform::read_u8(this + 0x103) == 0 {
        unlock_seq_manager();
        return 0;
    }

    let voice = if platform::read_u32(this + 0xb5c) != 0 {
        call0_r(FUN_802819D0) as i32
    } else {
        0
    };
    let span = platform::read_i32(this + 0x13c).wrapping_sub(platform::read_i32(this + 0x138));
    let position = platform::read_i32(this + 0xe8).wrapping_mul(span).wrapping_add(voice);

    unlock_seq_manager();
    position
}

// ===========================================================================
// 0x8027D520
// ===========================================================================

/// Resets the block's per-element play state and clears its counters.
///
/// `0x80265F40` runs first, then two small-data defaults are read from
/// `0x8042EAE8`/`0x8042EAD0` and written as the `+0x00`/`+0x04` pair of all
/// eight `0x38`-byte elements starting at `+0xB88`. The arm flag at `+0x814` is
/// set, eleven state bytes and two state words are cleared, and the first
/// element's `+0xB58` byte and `+0xB5C` word plus the remaining elements' pairs
/// and all sixteen allocator-slot words are zeroed.
///
/// # Original
/// - Ghidra: `FUN_8027d520` at `0x8027D520` (308 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027d520_FUN_8027d520.c`
/// - observed: `FUN_80265f40();`, `uVar1 = *(undefined4 *)(unaff_r2 + -0x4598);`,
///   `uVar2 = *(undefined4 *)(unaff_r2 + -0x45b0);`, the sixteen
///   `+0xB88 + 0x38n` default stores, the byte clears at `+0x102`, `+0x103`,
///   `+0x107`..`+0x10F`, the word clears at `+0x110` and `+0x148`, the
///   `+0xB58`/`+0xB5C` pair, the seven further `+0x38`-strided pairs and the
///   sixteen `+0x34`-strided `+0x818` slot words.
/// - `// inferred:` `r2 = 0x80433080`, so the two references are the absolute
///   words `0x8042EAE8` and `0x8042EAD0`.
///
/// # Arguments
/// - `this`: sound element block.
///
/// # Side effects
/// - writes the element defaults, the state bytes and every slot word.
///
/// # Safety
/// `this` must be a live block; the range `+0xB58..+0xD18` must be writable.
///
/// # Confidence
/// high - every store is literal; only the small-data words' roles are
/// inferred.
pub unsafe fn fn_8027d520(this: usize) {
    platform::call0(FUN_80265F40);

    let default_a = platform::read_u32(SDA2_BLOCK_DEFAULT_A);
    let default_b = platform::read_u32(SDA2_BLOCK_DEFAULT_B);

    platform::write_u32(this + 0x814, 1);
    let mut element = 0usize;
    while element < 8 {
        platform::write_u32(this + 0xb88 + element * 0x38, default_a);
        platform::write_u32(this + 0xb8c + element * 0x38, default_b);
        element += 1;
    }

    for offset in [
        0x102usize, 0x103, 0x107, 0x108, 0x109, 0x10a, 0x10b, 0x10c, 0x10d, 0x10e, 0x10f,
    ] {
        platform::write_u8(this + offset, 0);
    }
    platform::write_u32(this + 0x110, 0);
    platform::write_u32(this + 0x148, 0);

    // The first element is written as its own pair, the remaining seven follow
    // the same `0x38` stride.
    platform::write_u8(this + 0xb58, 0);
    platform::write_u32(this + 0xb5c, 0);
    let mut rest = 1usize;
    while rest < 8 {
        platform::write_u8(this + 0xb58 + rest * 0x38, 0);
        platform::write_u32(this + 0xb5c + rest * 0x38, 0);
        rest += 1;
    }

    let mut slot = 0usize;
    while slot < 16 {
        platform::write_u32(this + 0x818 + slot * 0x34, 0);
        slot += 1;
    }
}

// ===========================================================================
// 0x8027DDD0
// ===========================================================================

/// Binds one voice to every element that is currently enabled.
///
/// For each element the routine asks its `+0x0C` count and calls the binding
/// manager `0x802830A0` with the element's callback [`fn_8027eef0`] and the
/// element block at `this + 0xB58`, so the manager can fill the element's
/// `+0x04` slot word. A failed bind rolls back the elements processed so far by
/// passing each stored slot word to `0x802810A0` and clearing `+0x04`, then
/// reports `0`; success reports `1`. The state lock is held across the walk.
///
/// # Original
/// - Ghidra: `FUN_8027ddd0` at `0x8027DDD0` (236 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ddd0_FUN_8027ddd0.c`
/// - Removed: the two `_restgpr_24` epilogue calls (`0x802DCDB4`); the
///   matching `_savegpr_24` prologue (`0x802DCD68`) was never modelled and
///   `this`/`context` are the incoming `r3`/`r4`.
/// - observed: the `*(int *)(iVar6 + 0x80c) <= iVar4` loop bound, the
///   `FUN_80282ef0()` / `FUN_802830a0(uVar2,*(undefined4 *)(pcVar7 + 0xc),
///   (int)uVar8,0xff,FUN_8027eef0,pcVar7)` call, the rollback loop clearing
///   `pcVar5 + 4` for every processed element, and the `1` / `0` returns.
/// - registers: `r24 = this`, `r27 = r24 + 0xB58` and `r28` walks that block in
///   `0x38` strides. `0x8027DE1C` loads `r4 = [r28+0xC]` (the `+0x0C` field),
///   `0x8027DE24` passes `r8 = r28` as the callback object, and `0x8027DE2C`
///   passes `r6 = 0xFF`. On failure `0x8027DE48` loads `r3 = [r27+4]` and calls
///   `0x802810A0` with that slot word, then `0x8027DE58` clears `[r27+4]`.
///
/// # Arguments
/// - `this`: sound element block (incoming `r3`).
/// - `context`: second incoming word, forwarded to the manager (incoming
///   `r4`).
///
/// # Returns
/// `1` when every enabled element bound, `0` after a rollback.
///
/// # Side effects
/// - stores one slot word per element and clears it again on the failure path,
/// - takes and releases the state lock.
///
/// # Safety
/// `this` must be a live block and the binding manager must be initialised.
///
/// # Confidence
/// medium - the loop, the six arguments and the rollback are literal; the
/// manager's arguments are reproduced from the export without their names.
pub unsafe fn fn_8027ddd0(this: usize, context: u32) -> u32 {
    let token = call0_r(FUN_801B1140);

    let count = platform::read_u32(this + 0x80c) as i32;
    let mut index = 0i32;
    while index < count {
        let element = this + 0xb58 + index as usize * 0x38;

        if platform::read_u8(element) != 0 {
            let manager = call0_r(FUN_80282EF0);
            let slot = platform::call6(
                FUN_802830A0,
                manager,
                platform::read_u32(element + 0x0c),
                context,
                0xff,
                FUN_8027EEF0 as u32,
                element as u32,
            );

            if slot == 0 {
                let mut release = 0i32;
                while release < index {
                    let done = this + 0xb58 + release as usize * 0x38;
                    let released = platform::read_u32(done + 4);
                    if released != 0 {
                        platform::call1(FUN_802810A0, released);
                        platform::write_u32(done + 4, 0);
                    }
                    release += 1;
                }
                return 0;
            }

            platform::write_u32(element + 4, slot);
            platform::write_u8(slot as usize + 0xa1, 1);
        }

        index += 1;
    }

    platform::call1(FUN_801B1180, token);
    1
}

// ===========================================================================
// 0x8027E590
// ===========================================================================

/// Pushes one time step into every voice of every enabled element.
///
/// For each element with a slot word, both voice slots are resolved to one of
/// the sixteen `0x34`-byte arrays at `+0x818` through the element's `+0xB68`
/// byte (values `>= 0x10` resolve to null). Each voice gets `0x802819F0` with
/// the caller's context and `0x80281A70` with the caller's value, and the
/// element is then armed with `0x80281AF0`. The state lock is held throughout.
///
/// # Original
/// - Ghidra: `FUN_8027e590` at `0x8027E590` (264 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027e590_FUN_8027e590.c`
/// - Removed: the `_restgpr_22` epilogue call (`0x802DCDAC`); the matching
///   `_savegpr_22` prologue (`0x802DCD60`) was never modelled and
///   `this`/`context` are the incoming `r3`/`r4`.
/// - observed:
///   `FUN_802819f0(iVar5,iVar4,*puVar9,(int)uVar10)`,
///   `FUN_80281a70(iVar5,iVar4,*puVar9,param_3)`,
///   `FUN_80281af0(iVar5,1)`, and the `iVar4 < 2` and `< 0x10` split.
///
/// # Arguments
/// - `this`: sound element block (incoming `r3`).
/// - `context`: second incoming word, forwarded to `0x802819F0` (incoming
///   `r4`).
/// - `value`: value forwarded to `0x80281A70` (incoming `r5`).
///
/// # Side effects
/// - writes both voices of every element and arms each element,
/// - takes and releases the state lock.
///
/// # Safety
/// `this` must be a live block; a slot index `>= 0x10` dereferences null
/// exactly as the original does.
///
/// # Confidence
/// medium - every call and offset is literal; the parameter roles come from the
/// call sites and the slot-array layout.
pub unsafe fn fn_8027e590(this: usize, context: u32, value: u32) {
    let token = call0_r(FUN_801B1140);

    let count = platform::read_u32(this + 0x80c) as i32;
    let mut index = 0i32;
    while index < count {
        let element = this + index as usize * 0x38;
        let slot_word = platform::read_u32(element + 0xb5c) as usize;

        if platform::read_u8(element + 0xb58) != 0 && slot_word != 0 {
            let voices = platform::read_u32(element + 0xb64) as i32;
            let mut voice = 0i32;
            while voice < voices {
                let slot = if voice < 2 {
                    let slot_index = platform::read_u8(element + 0xb68 + voice as usize) as usize;
                    if slot_index < 0x10 {
                        platform::read_u32(this + slot_index * 0x34 + 0x818) as usize
                    } else {
                        0
                    }
                } else {
                    0
                };

                let slot_data = platform::read_u32(slot);
                platform::call4(
                    FUN_802819F0,
                    slot_word as u32,
                    voice as u32,
                    slot_data,
                    context,
                );
                platform::call4(
                    FUN_80281A70,
                    slot_word as u32,
                    voice as u32,
                    slot_data,
                    value,
                );
                voice += 1;
            }
            platform::call2(FUN_80281AF0, slot_word as u32, 1);
        }

        index += 1;
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027EA00
// ===========================================================================

/// Starts every voice of every enabled element at one time step.
///
/// The pass mirrors [`fn_8027e590`] but forwards
/// `*(i32 *)(this + 0xF0) + step * *(i32 *)(this + 0xE8)` to `0x80281B80`
/// instead of a pair of parameter calls, and finishes by setting the block's
/// `+0x10D` byte.
///
/// # Original
/// - Ghidra: `FUN_8027ea00` at `0x8027EA00` (240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ea00_FUN_8027ea00.c`
/// - Removed: the `_restgpr_25` epilogue call (`0x802DCDB8`); the matching
///   `_savegpr_25` prologue (`0x802DCD6C`) was never modelled and
///   `this`/`step` are the incoming `r3`/`r4`.
/// - observed: the `*pcVar7 != '\0'` / `iVar5 != 0` guards, the `< 2` / `< 0x10` slot
///   resolution,
///   `FUN_80281b80(iVar5,iVar4,*puVar3,*(int *)(iVar1 + 0xf0) + (int)uVar8 *
///   *(int *)(iVar1 + 0xe8))` and `*(undefined1 *)(iVar1 + 0x10d) = 1;`.
///
/// # Arguments
/// - `this`: sound element block (incoming `r3`).
/// - `step`: second incoming word, scaled by `+0xE8` (incoming `r4`).
///
/// # Side effects
/// - writes both voices of every element and sets `+0x10D`,
/// - takes and releases the state lock.
///
/// # Safety
/// `this` must be a live block; a slot index `>= 0x10` dereferences null
/// exactly as the original does.
///
/// # Confidence
/// medium - the loop and the call are literal; the `step` role is inferred from
/// the `+0xE8` multiply.
pub unsafe fn fn_8027ea00(this: usize, step: u32) {
    let token = call0_r(FUN_801B1140);

    let count = platform::read_u32(this + 0x80c) as i32;
    let mut index = 0i32;
    while index < count {
        let element = this + index as usize * 0x38;
        let slot_word = platform::read_u32(element + 0xb5c) as usize;

        if platform::read_u8(element + 0xb58) != 0 && slot_word != 0 {
            let voices = platform::read_u32(element + 0xb64) as i32;
            let time = platform::read_u32(this + 0xf0).wrapping_add(
                step.wrapping_mul(platform::read_u32(this + 0xe8)),
            );
            let mut voice = 0i32;
            while voice < voices {
                let slot = if voice < 2 {
                    let slot_index = platform::read_u8(element + 0xb68 + voice as usize) as usize;
                    if slot_index < 0x10 {
                        platform::read_u32(this + slot_index * 0x34 + 0x818) as usize
                    } else {
                        0
                    }
                } else {
                    0
                };

                platform::call4(
                    FUN_80281B80,
                    slot_word as u32,
                    voice as u32,
                    platform::read_u32(slot),
                    time,
                );
                voice += 1;
            }
        }

        index += 1;
    }

    platform::call1(FUN_801B1180, token);
    platform::write_u8(this + 0x10d, 1);
}

// ===========================================================================
// 0x8027ED20
// ===========================================================================

/// Broadcasts the current pause state to every enabled element.
///
/// The new state is `+0x109 || +0x107`; when it differs from the remembered
/// byte at `+0x108`, each enabled element's voice is told through
/// `0x80281460(voice, state)` and the remembered byte is updated. The state lock
/// is held across the walk, which is why the comparison and the store are inside
/// the same critical section.
///
/// # Original
/// - Ghidra: `FUN_8027ed20` at `0x8027ED20` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ed20_FUN_8027ed20.c`
/// - observed: `iVar1 = FUN_802dcd74();`, `bVar4 = *(char *)(iVar1 + 0x109) !=
///   '\0' || *(char *)(iVar1 + 0x107) != '\0';`,
///   `if (bVar4 != (bool)*(char *)(iVar1 + 0x108))`, the
///   `*(int *)(iVar1 + 0x80c)` loop with `FUN_80281460(*(int *)(iVar5 + 0xb5c),
///   bVar4)` and `*(bool *)(iVar1 + 0x108) = bVar4;`.
/// - `// inferred:` the export binds the save stub's 64-bit temporary to a plain
///   `int`; the block it dereferences is the same `+0xB58`/`+0x80C` object as
///   [`fn_8027ddd0`] and [`fn_8027ea00`], so the first argument is taken as
///   `this`.
/// - Removed: the modelled `_savegpr_27` prologue call (`0x802DCD74`) and the
///   `_restgpr_27` epilogue call (`0x802DCDC0`). `this` is the incoming `r3`,
///   copied to `r27` immediately after the prologue `bl`.
///
/// # Arguments
/// - `this`: sound element block.
///
/// # Side effects
/// - calls `0x80281460` once per enabled element with a voice,
/// - writes `+0x108`, takes and releases the state lock.
///
/// # Safety
/// `this` must be a live block.
///
/// # Confidence
/// medium - the predicate, the loop and the store are literal; the object's
/// provenance is inferred from the shared field offsets.
pub unsafe fn fn_8027ed20(this: usize) {
    let token = call0_r(FUN_801B1140);

    let paused = platform::read_u8(this + 0x109) != 0 || platform::read_u8(this + 0x107) != 0;
    if paused != (platform::read_u8(this + 0x108) != 0) {
        let count = platform::read_u32(this + 0x80c) as i32;
        let mut index = 0i32;
        while index < count {
            let element = this + index as usize * 0x38;
            if platform::read_u8(element + 0xb58) != 0 {
                let voice = platform::read_u32(element + 0xb5c);
                if voice != 0 {
                    platform::call2(FUN_80281460, voice, paused as u32);
                }
            }
            index += 1;
        }
        platform::write_u8(this + 0x108, paused as u8);
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027F270
// ===========================================================================

/// Detaches a voice entry from its owner and drops it.
///
/// The owner is the entry's `+0x10` word. The entry is unlinked from the owner's
/// `+0x174` list, the entry's own virtual slot `+0x08` is called with `-1`, and
/// the owner's `+0x180` list drops the entry with `0x8026DB00`. The state lock
/// is held for the whole operation.
///
/// # Original
/// - Ghidra: `FUN_8027f270` at `0x8027F270` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f270_FUN_8027f270.c`
/// - observed: `uVar1 = FUN_801b1140();`,
///   `FUN_80229190(param_1[4] + 0x174,param_1 + 0xb);`, `iVar2 = param_1[4];`,
///   `(**(code **)(*param_1 + 8))(param_1,0xffffffff);`,
///   `FUN_8026db00(iVar2 + 0x180,param_1);` and `FUN_801b1180(uVar1);`.
///
/// # Arguments
/// - `this`: voice entry to detach.
///
/// # Side effects
/// - unlinks the entry from two owner lists and runs its slot `+0x08`,
/// - takes and releases the state lock.
///
/// # Safety
/// `this` must be a live entry whose `+0x10` word addresses its owner.
///
/// # Confidence
/// high - the unlink, the virtual call with `-1`, the list drop and the lock
/// pair are literal.
pub unsafe fn fn_8027f270(this: usize) {
    let token = call0_r(FUN_801B1140);

    let owner = platform::read_u32(this + 0x10) as usize;
    platform::call2(FUN_80229190, (owner + 0x174) as u32, (this + 0x2c) as u32);

    if this != 0 {
        let vtable = platform::read_u32(this) as usize;
        platform::call2(platform::read_u32(vtable + 8) as usize, this as u32, 0xffff_ffff);
        platform::call2(FUN_8026DB00, (owner + 0x180) as u32, this as u32);
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027FDD0
// ===========================================================================

/// Returns the small-element registry, building it on first use.
///
/// The one-shot flag at `r13 - 0x4920` gates the body. On the first call the
/// registry at `0x80416FC4` is registered with `0x802DC9A0` (element size
/// `0x0C`, count `3`), its count and link words are cleared, its two list
/// headers are initialised with `0x801B4B20`, and `0x802DC780` adds
/// [`fn_8027fe70`] as the block's destructor. The address of the registry is
/// returned in both cases.
///
/// # Original
/// - Ghidra: `FUN_8027fdd0` at `0x8027FDD0` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027fdd0_FUN_8027fdd0.c`
/// - observed: the `*(char *)(unaff_r13 + -0x4920) == '\0'` guard,
///   `FUN_802dc9a0(&DAT_80416fc4,&LAB_8027ff40,FUN_8027fee0,0xc,3)`,
///   `DAT_80416fe8 = 0; DAT_80416fec = 0;`,
///   `FUN_801b4b20(&DAT_80416ff0); FUN_801b4b20(&DAT_80416ff8);`,
///   `FUN_802dc780(&DAT_80416fc4,FUN_8027fe70,&DAT_80416fb8);`, the flag store
///   and `return &DAT_80416fc4;`.
///
/// # Returns
/// The address of the registry at `0x80416FC4`.
///
/// # Side effects
/// - on first use: registers the destructor pair, clears the count/link and
///   initialises both list headers.
///
/// # Safety
/// The registry's static storage must be writable, as it is in the original
/// image.
///
/// # Confidence
/// high - the guard, all five calls and every store are literal.
pub unsafe fn fn_8027fdd0() -> u32 {
    if platform::read_u8(ELEMENT_REGISTRY_READY) == 0 {
        platform::call5(
            FUN_802DC9A0,
            ELEMENT_REGISTRY as u32,
            LAB_8027FF40,
            FUN_8027FEE0 as u32,
            0x0c,
            3,
        );
        platform::write_u32(ELEMENT_REGISTRY_COUNT, 0);
        platform::write_u32(ELEMENT_REGISTRY_LINK, 0);
        platform::call1(FUN_801B4B20, ELEMENT_REGISTRY_LIST_A as u32);
        platform::call1(FUN_801B4B20, ELEMENT_REGISTRY_LIST_B as u32);
        platform::call3(
            FUN_802DC780,
            ELEMENT_REGISTRY as u32,
            FUN_8027FE70 as u32,
            DAT_80416FB8 as u32,
        );
        platform::write_u8(ELEMENT_REGISTRY_READY, 1);
    }
    ELEMENT_REGISTRY as u32
}

// ---------------------------------------------------------------------------
// Wave 99 helpers: the global track-record table, the priority-bucket table
// and the float-argument relays used by the playback front ends.
//
// `0x802DCD48..0x802DCD74` are CodeWarrior register-save stubs. Ghidra prints
// them as returning a 64-bit pair whose halves are the incoming argument
// registers; the bodies below take those values from their own parameters.
// ---------------------------------------------------------------------------

/// `0x80270120` - resets one `0xA8`-byte global track record.
const FUN_80270120: usize = 0x8027_0120;

/// `0x80270B70` - returns the record slot for a caller key.
///
/// `// unknown:` the lookup rule; the export only prints `(table, key)`.
const FUN_80270B70: usize = 0x8027_0B70;

/// `0x80270330` / `0x80270410` - per-record tick and 0x28-byte body copy.
const FUN_80270330: usize = 0x8027_0330;
const FUN_80270410: usize = 0x8027_0410;

/// `0x801A2020` / `0x801A2050` / `0x801A2120` - controller-port query and
/// per-port 0x28-byte read used by [`fn_80270c20`].
const FUN_801A2020: usize = 0x801A_2020;
const FUN_801A2050: usize = 0x801A_2050;
const FUN_801A2120: usize = 0x801A_2120;

/// Base, first record and end of the four-entry global track table.
const SEQ_TRACK_TABLE: usize = 0x803F_C990;
const SEQ_TRACK_RECORDS: usize = 0x803F_C9C8;
const SEQ_TRACK_TABLE_END: usize = 0x803F_CC68;

/// `r13 - 0x4958` = `0x8042AD28`: "the global track table is initialised".
const SEQ_TRACK_TABLE_READY: usize = crate::boot::entry::SDA1_BASE - 0x4958;

/// `0x802741E0` / `0x80274320` / `0x80274480` / `0x80274630` - the four
/// per-node mix helpers [`fn_80274ce0`] calls. All four have bodies in this
/// module ([`fn_802741e0`], [`fn_80274320`], [`fn_80274480`], [`fn_80274630`]);
/// the constants remain for the call sites that dispatch by address.
const FUN_802741E0: usize = 0x8027_41E0;
const FUN_80274320: usize = 0x8027_4320;
const FUN_80274480: usize = 0x8027_4480;
const FUN_80274630: usize = 0x8027_4630;

/// `0x8042EA8C` = `r2 - 0x45F4`: default element level written by
/// [`fn_80274ce0`].
const SDA2_ELEM_LEVEL: usize = 0x8042_EA8C;

/// `0x801B23F0` - switches one interrupt-mask bit off.
const FUN_801B23F0: usize = 0x801B_23F0;

/// `0x802833B0` - tail tick of the music scheduler in [`fn_8027b850`].
const FUN_802833B0: usize = 0x8028_33B0;

/// `0x801B5130` / `0x801B5860` - thread create and failure cleanup used by
/// [`fn_8027b720`].
const FUN_801B5130: usize = 0x801B_5130;
const FUN_801B5860: usize = 0x801B_5860;

/// `0x80260860` - returns the sound-thread manager handle.
const FUN_80260860: usize = 0x8026_0860;

/// `0x80260ED0` / `0x80260F40` - register / unregister a thread callback.
const FUN_80260ED0: usize = 0x8026_0ED0;
const FUN_80260F40: usize = 0x8026_0F40;

/// `0x801B24C0` - receives one message from the queue at `manager + 0x320`.
const FUN_801B24C0: usize = 0x801B_24C0;

/// The sequence thread's per-frame ticks, in the order [`fn_8027baf0`] calls
/// them.
const FUN_80265100: usize = 0x8026_5100;
const FUN_80260AF0: usize = 0x8026_0AF0;
const FUN_80267DD0: usize = 0x8026_7DD0;
const FUN_80283A30: usize = 0x8028_3A30;
const FUN_80283300: usize = 0x8028_3300;

/// `0x801B5FA0` - reads the platform tick counter.
const FUN_801B5FA0: usize = 0x801B_5FA0;

/// `0x8026F440` - resolves an archive name to a payload offset.
const FUN_8026F440: usize = 0x8026_F440;

/// `0x80267400` - sets the sequence tempo (first argument is passed in `f1`).
/// `0x80267530` / `0x80267570` / `0x802675B0` set the three volume words.
const FUN_80267400: usize = 0x8026_7400;
const FUN_80267530: usize = 0x8026_7530;
const FUN_80267570: usize = 0x8026_7570;
const FUN_802675B0: usize = 0x8026_75B0;

/// `0x8042EAB8` = `r2 - 0x45C8`: the float divisor that turns the caller's
/// raw clock word into seconds.
const SDA2_PLAY_SCALE: usize = 0x8042_EAB8;

/// `0x80283EC0` - starts one archive entry through the resolver; seven words.
const FUN_80283EC0: usize = 0x8028_3EC0;

/// `0x80284040` / `0x80284050` - two post-start setters.
const FUN_80284040: usize = 0x8028_4040;
const FUN_80284050: usize = 0x8028_4050;

/// `0x80277050` - initialises a 0x220-byte archive-loader block.
const FUN_80277050: usize = 0x8027_7050;

/// `0x802770D0` / `0x80277090` - the loader open and its release.
const FUN_802770D0: usize = 0x8027_70D0;
const FUN_80277090: usize = 0x8027_7090;

/// `0x8027F650` - rebuilds the sub-object; the Rust body discards the `r3`
/// result, but [`fn_80279880`] loops on it, so this caller uses the address.
const FUN_8027F650: usize = 0x8027_F650;

/// `0x80275710` / `0x80275750` - return a slot count / a name id through `r3`;
/// the translated bodies are declared `void`, so callers use the addresses.
const FUN_80275710: usize = 0x8027_5710;
const FUN_80275750: usize = 0x8027_5750;

/// `0x802724E0` - rebuilds the player and reports its state through `r3`; the
/// translated body is declared `void`, so [`fn_80279510`] calls the address.
const FUN_802724E0: usize = 0x8027_24E0;

/// `0x803FCCB0` - 0x80-entry priority bucket table used by
/// [`fn_8027aa00`]; the second constant is the first bucket's link word.
const SEQ_BUCKETS: usize = 0x803F_CCB0;
const SEQ_BUCKETS_LINK: usize = 0x803F_CCB4;

/// `0x8027A5F0` / `0x8027AB90` - bucket element label and cleanup label.
const LAB_8027A5F0: u32 = 0x8027_A5F0;
const LAB_8027AB90: u32 = 0x8027_AB90;

/// `0x80277850` / `0x802778D0` / `0x80277950` - the three bucket labels
/// [`fn_802783c0`] hands to `0x802DC9A0`.
const LAB_80277850: u32 = 0x8027_7850;
const LAB_802778D0: u32 = 0x8027_78D0;
const LAB_80277950: u32 = 0x8027_7950;

/// `r13 - 0x4948` = `0x8042AD38`: "the priority bucket table is initialised".
const SEQ_BUCKET_READY: usize = crate::boot::entry::SDA1_BASE - 0x4948;

/// `0x80267320` - links a bank object to its bucket after insertion.
const FUN_80267320: usize = 0x8026_7320;

/// `0x8034F218` - vtable of the large element player built by
/// [`fn_8027f450`].
const VT_ELEMENT_PLAYER: u32 = 0x8034_F218;

/// `r13 - 0x4928` = `0x8042AD58`: head of the element-player marker list.
/// [`fn_8027fc30`] returns it and [`fn_8027fc50`] compares its chain links
/// against it.
const ELEMENT_PLAYER_LIST: usize = crate::boot::entry::SDA1_BASE - 0x4928;

/// `0x8042EAF8` = `r2 - 0x4588`: the minimum ramp level.
const SDA2_RAMP_MIN: usize = 0x8042_EAF8;

/// `0x802DCEC4` - signed 64-bit divide helper used by [`fn_8027ede0`].
const FUN_802DCEC4: usize = 0x802D_CEC4;

/// `0x80266DC0` - per-frame pre-hook called by [`fn_8027f7b0`].
const FUN_80266DC0: usize = 0x8026_6DC0;

/// `0x801AC440` / `0x801AC470` / `0x80004364` - stream buffer clear, consume
/// and copy used by [`fn_8027d960`].
const FUN_801AC440: usize = 0x801A_C440;
const FUN_801AC470: usize = 0x801A_C470;
const FUN_80004364: usize = 0x8000_4364;

/// `0x8027EAF0` - appends one element record; not reconstructed in this wave.
const FUN_8027EAF0: usize = 0x8027_EAF0;

/// `0x80412FA0` - the `0x4000`-byte stream buffer shared by the block readers.
const SEQ_STREAM_BUFFER: usize = 0x8041_2FA0;

/// Calls a routine whose first argument is an `f64` in `f1` and whose second
/// argument is an integer word.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f64_u32(addr: usize, first: f64, second: u32) {
    let f: extern "C" fn(f64, u32) = core::mem::transmute(addr);
    f(first, second);
}

/// Calls a four-argument routine that returns a 64-bit value in `r3:r4`.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call4_u64(addr: usize, a: u32, b: u32, c: u32, d: u32) -> u64 {
    let f: extern "C" fn(u32, u32, u32, u32) -> u64 = core::mem::transmute(addr);
    f(a, b, c, d)
}

/// Rebuilds one 0x80-bucket priority list from the nodes currently queued at
/// `this + list`.
///
/// The buckets are `0x0C` bytes each and are filled from the list in order,
/// then drained back into the list in bucket order. `label` and `destructor`
/// are the pair the original passes to `0x802DC9A0` / `0x802DCA98`.
///
/// # Safety
/// `this + list` must be a live intrusive list and `this + mutex` its mutex.
unsafe fn rebuild_priority_bucket(
    this: usize,
    list: usize,
    mutex: usize,
    label: u32,
    destructor: usize,
) {
    if platform::read_u32(this + list) <= 1 {
        return;
    }
    platform::call1(MUTEX_LOCK, (this + mutex) as u32);

    let mut buckets = [0u8; 0x600];
    let base = buckets.as_mut_ptr() as u32;
    platform::call5(
        FUN_802DC9A0,
        base,
        label,
        destructor as u32,
        0x0c,
        0x80,
    );

    while platform::read_u32(this + list) != 0 {
        let mut node = platform::read_u32(this + list + 4);
        platform::call2(FUN_802290C0, (this + list) as u32, (&mut node as *mut u32) as u32);
        let priority = clamp_priority(
            platform::read_u8(node as usize - 0x58) as u32,
            platform::read_u32(node as usize - 0xa0),
        );
        let mut bucket_ref = base.wrapping_add(priority.wrapping_mul(0x0c));
        platform::call3(
            FUN_80229160,
            base.wrapping_add(priority.wrapping_mul(0x0c)),
            (&mut bucket_ref as *mut u32) as u32,
            node,
        );
    }

    let mut bucket = base as usize;
    let mut index = 0;
    while index < 0x80 {
        while platform::read_u32(bucket) != 0 {
            let mut node = platform::read_u32(bucket + 4);
            platform::call2(FUN_802290C0, bucket as u32, (&mut node as *mut u32) as u32);
            let mut head_ref = (this + list + 4) as u32;
            platform::call3(
                FUN_80229160,
                (this + list) as u32,
                (&mut head_ref as *mut u32) as u32,
                node,
            );
        }
        index += 1;
        bucket += 0x0c;
    }

    platform::call4(FUN_802DCA98, base, destructor as u32, 0x0c, 0x80);
    platform::call1(MUTEX_UNLOCK, (this + mutex) as u32);
}

// ===========================================================================
// 0x80270920
// ===========================================================================

/// Applies one state code to the global track record selected by key.
///
/// The record table is created by [`fn_80270ad0`] and the key is resolved by
/// `0x80270B70`. State `0` stores `6` at `+0x08`; state `-2` stores `3` at
/// `+0x10`; every other value stores `0` at `+0x08`. Unless the state is `-2`,
/// a pending callback word at `+0x38` is invoked once with `(key, state)` and
/// then cleared. The record's `+0x05` byte is always cleared.
///
/// # Original
/// - Ghidra: `FUN_80270920` at `0x80270920` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270920_FUN_80270920.c`
/// - observed: `FUN_80270ad0()`, `FUN_80270b70(uVar1,param_1)`, the
///   `param_2 == 0` / `-2` / `-3` / `-1` / else store chain, the
///   `param_2 != -2` + `+0x04` + `+0x38` callback guard, the indirect call and
///   `*(undefined1 *)(iVar2 + 5) = 0;`.
///
/// # Arguments
/// - `key`: record key passed to `0x80270B70`.
/// - `state`: state code; `0` and `-2` have dedicated stores.
///
/// # Side effects
/// - writes the record's state fields and may invoke its callback once.
///
/// # Safety
/// The global table must be initialised and the key must resolve to a record.
///
/// # Confidence
/// medium - the stores and callback guard are literal; the lookup rule and the
/// state-code names are not visible in this export.
pub unsafe fn fn_80270920(key: u32, state: i32) {
    let table = fn_80270ad0();
    let record = platform::call2(FUN_80270B70, table, key) as usize;

    match state {
        0 => platform::write_u32(record + 0x08, 6),
        -2 => platform::write_u32(record + 0x10, 3),
        _ => platform::write_u32(record + 0x08, 0),
    }

    if state != -2 && platform::read_u8(record + 0x04) != 0 {
        let callback = platform::read_u32(record + 0x38) as usize;
        if callback != 0 {
            platform::call2(callback, key, state as u32);
            platform::write_u32(record + 0x38, 0);
        }
    }

    platform::write_u8(record + 0x05, 0);
}

// ===========================================================================
// 0x80270AD0
// ===========================================================================

/// Returns the global four-record track table, building it on first use.
///
/// The one-shot flag at `r13 - 0x4958` gates the body. Each of the four
/// `0xA8`-byte records starting at `0x803FC9C8` is reset by `0x80270120`, and
/// the record index `0..3` is stored at `record - 0x38 + 0x6C` (the `+0x6C`
/// word of the record header). The table base `0x803FC990` is returned in both
/// cases.
///
/// # Original
/// - Ghidra: `FUN_80270ad0` at `0x80270AD0` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270ad0_FUN_80270ad0.c`
/// - observed: the `*(char *)(unaff_r13 + -0x4958) == '\0'` guard,
///   `DAT_803fc990._0_1_ = 0;`, the `FUN_80270120(puVar1)` loop with
///   `puVar1 = puVar1 + 0x54` (0xA8 bytes) up to `&DAT_803fcc68`, the four
///   index stores at `0x803FC9FC`, `0x803FCAA4`, `0x803FCB4C`, `0x803FCBF4`,
///   the flag store and `return &DAT_803fc990;`.
///
/// # Returns
/// The address of the table at `0x803FC990`.
///
/// # Side effects
/// - on first use: resets four records and writes their indices.
///
/// # Safety
/// The table's static storage must be writable, as it is in the original image.
///
/// # Confidence
/// high - the guard, the four reset calls, the four index stores and the
/// returned base are literal.
pub unsafe fn fn_80270ad0() -> u32 {
    if platform::read_u8(SEQ_TRACK_TABLE_READY) == 0 {
        platform::write_u8(SEQ_TRACK_TABLE, 0);

        let mut record = SEQ_TRACK_RECORDS;
        while record < SEQ_TRACK_TABLE_END {
            platform::call1(FUN_80270120, record as u32);
            record += 0xa8;
        }

        let mut index = 0u32;
        while index < 4 {
            platform::write_u32(
                SEQ_TRACK_TABLE + 0x6c + index as usize * 0xa8,
                index,
            );
            index += 1;
        }

        platform::write_u8(SEQ_TRACK_TABLE_READY, 1);
    }
    SEQ_TRACK_TABLE as u32
}

// ===========================================================================
// 0x80270C20
// ===========================================================================

/// Refreshes the four global track records from the controller ports.
///
/// The table is initialised exactly as in [`fn_80270ad0`]. When
/// `0x801A2020` reports more than `0x27` ports, each of the four records whose
/// header word `+0x40` equals `4` is refreshed: `0x801A2050` reads `0x28`
/// bytes into a stack buffer and `0x80270410` copies them into the record.
/// `0x80270330` then ticks every record, and `0x801A2120(0x28)` finishes the
/// pass.
///
/// # Original
/// - Ghidra: `FUN_80270c20` at `0x80270C20` (248 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80270c20_FUN_80270c20.c`
/// - observed: the same initialisation block as `0x80270AD0`, then
///   `iVar1 = FUN_801a2020(); if (0x27 < iVar1)` around the four-iteration
///   `puVar4[0x10] == 4` test, `FUN_801a2050(iVar1,auStack_68,0x28)`,
///   `FUN_80270410(puVar3,auStack_68)`, `FUN_80270330(puVar3)` with the
///   `0xA8` cursor step and `FUN_801a2120(0x28)`.
///
/// # Side effects
/// - initialises the table on first use, may copy four 0x28-byte bodies and
///   ticks all four records.
///
/// # Safety
/// The table's static storage and the controller buffers must be live.
///
/// # Confidence
/// medium - all four calls and the loop bounds are literal; the port count and
/// the `+0x40 == 4` test are reproduced without naming the controller state.
pub unsafe fn fn_80270c20() {
    if platform::read_u8(SEQ_TRACK_TABLE_READY) == 0 {
        platform::write_u8(SEQ_TRACK_TABLE, 0);

        let mut record = SEQ_TRACK_RECORDS;
        while record < SEQ_TRACK_TABLE_END {
            platform::call1(FUN_80270120, record as u32);
            record += 0xa8;
        }

        let mut index = 0u32;
        while index < 4 {
            platform::write_u32(
                SEQ_TRACK_TABLE + 0x6c + index as usize * 0xa8,
                index,
            );
            index += 1;
        }

        platform::write_u8(SEQ_TRACK_TABLE_READY, 1);
    }

    if call0_r(FUN_801A2020) > 0x27 {
        let mut body = [0u8; 0x28];
        let body_ptr = body.as_mut_ptr() as u32;
        let mut index = 0u32;
        let mut record = SEQ_TRACK_RECORDS;
        let mut header = SEQ_TRACK_TABLE;
        while index < 4 {
            if platform::read_u32(header + 0x40) == 4 {
                platform::call3(FUN_801A2050, index, body_ptr, 0x28);
                platform::call2(FUN_80270410, record as u32, body_ptr);
            }
            platform::call1(FUN_80270330, record as u32);
            index += 1;
            record += 0xa8;
            header += 0xa8;
        }
        platform::call1(FUN_801A2120, 0x28);
    }
}

// ===========================================================================
// 0x80274CE0
// ===========================================================================

/// Computes the requested mix values for every node in the player's list.
///
/// `mask` selects which outputs are refreshed. Bit `0` writes the default
/// level at `out[0]`; bits `1`/`2` copy the two values from the six-argument
/// helper into `out[2]`/`out[3]`; bit `3` writes the negated node counter into
/// `out[8]`; bit `4` writes one value into `out[1]`; bit `5` writes the
/// object's `+0x28` word into `out[7]` and keeps the smaller of the helper
/// result and `out[6]`. Bits `0` and `3` additionally fold the per-node result
/// into maximum/minimum accumulators, and when the list has more than one node
/// those two bits are cleared first.
///
/// # Original
/// - Ghidra: `FUN_80274ce0` at `0x80274CE0` (444 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80274ce0_FUN_80274ce0.c`
/// - observed: the `param_5 & 1` / `& 8` / `& 0x10` / `& 0x20` stores, the
///   `1 < *(uint *)(iVar2 + 0xc)` mask clear `& 0xffffffe9`, the
///   `puVar4 != iVar2 + 0x10` walk with `puVar3 = puVar4 + -0x19` words,
///   the four calls to `0x802741E0` / `0x80274320` / `0x80274480` /
///   `0x80274630`, and the `param_6[6] < local_58` minimum.
/// - registers: `r28 = r3` (first incoming word), `r29 = r4` (`this`),
///   `r30 = r5`, `r31 = r8` (`out`). All four helpers take `r3 = r29`,
///   `r4 = node` and `r5 = r30`; at `0x80274DE4` the six-argument helper's
///   fourth argument is `r6 = r28 + 4`, i.e. the first incoming word plus four,
///   not `r28` itself. `0x80274D44` sets the list sentinel to `r4 + 0x10`.
/// - Removed: the modelled `_restgpr_17` epilogue call (`0x802DCD98`); the
///   matching `_savegpr_17` prologue (`0x802DCD4C`) was never modelled.
///
/// # Arguments
/// - `this`: sequence player (incoming `r4`; the save-stub "return" is the
///   second incoming word).
/// - `context`: first incoming word (incoming `r3`).
/// - `param_3`: node filter/parameter word (incoming `r5`).
/// - `param_4`: value forwarded to the six-argument helper (incoming `r6`).
/// - `mask`: output selector described above.
/// - `out`: at least nine `u32`/`f32` slots.
///
/// # Side effects
/// - writes the requested `out` slots; no other persistent state.
///
/// # Safety
/// `this` must be a live player whose `+0x10` list is intact; `out` must be
/// writable for nine words and the node records must be live.
///
/// # Confidence
/// medium - the mask tests, stores and list walk are literal; the node layout
/// and the four helper roles are inferred from the fields they touch.
pub unsafe fn fn_80274ce0(
    this: usize,
    context: u32,
    param_3: u32,
    param_4: u32,
    mask: u32,
    out: usize,
) {
    let mut mask = mask;
    if mask & 1 != 0 {
        platform::write_f32(out, platform::read_f32(SDA2_ELEM_LEVEL));
    }
    if mask & 8 != 0 {
        let counter = platform::read_i32(this + 0x1c);
        platform::write_f32(out + 0x20, (-counter) as f32);
    }
    if platform::read_u32(this + 0x0c) > 1 {
        mask &= 0xffff_ffe9;
    }

    let mut values = [0f32; 17];
    let mut max_value = 0f32;
    let mut mid_a = 0f32;
    let mut mid_b = 0f32;
    let mut single = 0f32;

    let head = this + 0x10;
    let mut link = platform::read_u32(head) as usize;
    while link != head {
        let node = link - 0x64;

        if mask & 9 != 0 {
            platform::call5(
                FUN_802741E0,
                this as u32,
                node as u32,
                param_3,
                values.as_mut_ptr() as u32,
                (&mut max_value as *mut f32) as u32,
            );
            if mask & 1 != 0 {
                let value = values[0];
                let current = platform::read_f32(out);
                if value > current {
                    platform::write_f32(out, value);
                }
            }
            if mask & 8 != 0 {
                let value = max_value as i32;
                let current = platform::read_f32(out + 0x20) as i32;
                let result = if current > value { current } else { value };
                platform::write_f32(out + 0x20, result as f32);
            }
        }

        if mask & 6 != 0 {
            platform::call6(
                FUN_80274320,
                this as u32,
                node as u32,
                param_3,
                context.wrapping_add(4),
                (&mut mid_a as *mut f32) as u32,
                (&mut mid_b as *mut f32) as u32,
            );
            if mask & 2 != 0 {
                platform::write_f32(out + 8, mid_a);
            }
            if mask & 4 != 0 {
                platform::write_f32(out + 0x0c, mid_b);
            }
        }

        if mask & 0x10 != 0 {
            platform::call4(
                FUN_80274480,
                this as u32,
                node as u32,
                param_3,
                (&mut single as *mut f32) as u32,
            );
            platform::write_f32(out + 4, single);
        }

        if mask & 0x20 != 0 {
            platform::call4(
                FUN_80274630,
                this as u32,
                node as u32,
                param_3,
                (&mut single as *mut f32) as u32,
            );
            platform::write_f32(out + 0x1c, platform::read_f32(this + 0x28));
            let value = single;
            let current = platform::read_f32(out + 0x18);
            let result = if current < value { current } else { value };
            platform::write_f32(out + 0x18, result);
        }

        link = platform::read_u32(link) as usize;
    }

    let _ = param_4;
}

// ===========================================================================
// 0x802783C0
// ===========================================================================

/// Rebuilds three 0x80-bucket priority lists, releasing each bucket first.
///
/// The bank-slot array at `+0x40` is released one `100`-byte entry at a time.
/// Three intrusive lists then have their buckets rebuilt by draining the list
/// into a local 0x80-entry table and back: the list at `+0x48` (mutex `+0x54`),
/// the list at `+0x70` (mutex `+0x7C`) and the list at `+0x98` (mutex `+0xA4`).
/// Each rebuild only runs when the list holds more than one node.
///
/// # Original
/// - Ghidra: `FUN_802783c0` at `0x802783C0` (956 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802783c0_FUN_802783c0.c`
/// - observed: the `FUN_8027a750(*(int *)(iVar1 + 0x40) + iVar4)` loop with
///   `iVar4 = iVar4 + 100`, the three `1 < *(uint *)(iVar1 + 0x48/0x70/0x98)`
///   guards, the three `FUN_802dc9a0(...,0xc,0x80)` /
///   `FUN_802dca98(...,0xc,0x80)` pairs, the
///   `*(byte *)(iVar4 + -0x58) + *(int *)(iVar4 + -0xa0)` priority clamp, the
///   `FUN_802290c0` / `FUN_80229160` drain-and-refill loops.
/// - Removed: the modelled `_savegpr_26` prologue call (`0x802DCD70`) and the
///   `_restgpr_26` epilogue call (`0x802DCDBC`); `this` is the incoming `r3`.
///
/// # Arguments
/// - `this`: sound-system object that owns the three lists.
///
/// # Side effects
/// - releases the bank-slot array and rebuilds the three bucket tables.
///
/// # Safety
/// `this` must be a live sound-system object and all three lists must be
/// intact.
///
/// # Confidence
/// medium - the offsets, guards and call sequence are literal; the bucket
/// field roles are inferred.
pub unsafe fn fn_802783c0(this: usize) {
    let count = platform::read_u32(this + 0x3c);
    let base = platform::read_u32(this + 0x40) as usize;
    let mut index = 0u32;
    while index < count {
        fn_8027a750(base + index as usize * 100);
        index += 1;
    }

    rebuild_priority_bucket(
        this,
        0x48,
        0x54,
        LAB_80277850,
        fn_80277870 as *const () as usize,
    );
    rebuild_priority_bucket(
        this,
        0x70,
        0x7c,
        LAB_802778D0,
        fn_802778f0 as *const () as usize,
    );
    rebuild_priority_bucket(
        this,
        0x98,
        0xa4,
        LAB_80277950,
        fn_80277970 as *const () as usize,
    );
}

// ===========================================================================
// 0x80279510
// ===========================================================================

/// Starts one sequence in the player, resolving it from a file or a bank id.
///
/// The routine has two resolution routes. When `sequence` is non-null the
/// `RSEQ` header at that address is parsed by [`fn_80270d20`] and, when a
/// `name` is given, the name table is walked by [`fn_80270da0`]; the payload
/// word then goes through `0x8026F440`. When only a bank id is available the
/// id is resolved by [`fn_80278790`], and the caller's second word is the
/// payload. A stream handle is opened with [`fn_80275850`] and checked against
/// the file's remaining length ([`fn_802700f0`]); then [`fn_802724e0`] rebuilds
/// the player until it reports ready. Finally the tempo and three volume words
/// are set, the callback pair is installed, and the start call
/// [`fn_802725d0`] or [`fn_80272560`] receives the resolved payload.
///
/// Status codes are returned: `0x0B` bad name, `4` no
/// file, `6` stream open failed, `5` file too short, `9` a player with the
/// same identity is already live, `0xFF` unexpected rebuild result, `0`
/// success.
///
/// # Original
/// - Ghidra: `FUN_80279510` at `0x80279510` (872 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80279510_FUN_80279510.c`
/// - Removed: the nine modelled `_restgpr_22` epilogue calls (`0x802DCDAC`);
///   the matching `_savegpr_22` prologue (`0x802DCD60`) was never modelled and
///   each exit now returns its status directly.
/// - observed: the `0x802DCD60` pair (high = source object, low = player), the
///   `FUN_80270d20` / `FUN_80270da0` / `FUN_80270d90` name route, the
///   `FUN_80278790` bank route, `FUN_80275850` with `+0x290` and `0x200`, the
///   virtual `+0x40` length test against `FUN_802700f0`, the
///   `FUN_802724e0(player,source[+0x30],value,source+0x20)` loop with
///   `FUN_80266250` retries, the tempo/volume calls, `FUN_802728a0` /
///   `FUN_802728b0` / `FUN_802728c0`, and the `FUN_802725d0` /
///   `FUN_80272560` split.
///
/// # Arguments
/// - `this`: sequence player (incoming `r4`).
/// - `source`: sound-system object selected by the save stub (incoming `r3`).
/// - `param_3`: four-word request block (`[0]` = id, `[4]` = clock word,
///   `[5..7]` = volumes).
/// - `param_4`: second request block (`[0]` = fallback id, `[2]` = length,
///   `[3]` = callback byte, `+0x10` = volume byte).
/// - `mode`: start mode `0..2`.
/// - `delta`: start delta.
/// - `sequence`: `RSEQ` file address, or `0`.
/// - `name`: resource name, or `0`.
///
/// # Side effects
/// - opens and closes a stream, rebuilds the player, writes its tempo/volume
///   and callback fields, and starts it.
///
/// # Safety
/// All pointers must be live blocks of the sizes the original expects.
///
/// # Confidence
/// medium - every guard, offset and call is literal; the exact meaning of the
/// callback bytes is inferred.
pub unsafe fn fn_80279510(
    this: usize,
    source: usize,
    param_3: usize,
    param_4: usize,
    mode: i32,
    delta: u32,
    sequence: u32,
    name: u32,
) -> u32 {
    let mut length = platform::read_u32(param_4 + 8);
    let mut value = 0u32;
    let mut stream = 0usize;
    let mut resolved = 0usize;

    let mut header_a = [0u32; 2];
    let mut header_b = [0u32; 2];
    let mut header_c = [0u32; 2];

    if sequence != 0 {
        value = 0;
        fn_80270d20(header_a.as_mut_ptr() as usize, sequence);
        if name != 0
            && fn_80270da0(header_a.as_mut_ptr() as u32, name, (&mut value as *mut u32) as u32) == 0
        {
            return 0x0b;
        }
        let block = fn_80270d90(header_a.as_mut_ptr() as usize);
        value = platform::call3(
            FUN_8026F440,
            block,
            value,
            (&mut length as *mut u32) as u32,
        );
        resolved = sequence as usize;
    }

    if resolved == 0 {
        resolved = fn_80278790(source, platform::read_u32(param_3)) as usize;
        value = platform::read_u32(param_4);
        if name != 0 {
            fn_80270d20(header_b.as_mut_ptr() as usize, resolved as u32);
            if fn_80270da0(
                header_b.as_mut_ptr() as u32,
                name,
                (&mut value as *mut u32) as u32,
            ) == 0
            {
                return 0x0b;
            }
        }
    }

    if resolved == 0 {
        let file = platform::read_u32(this + 4) as usize;
        if file == 0 {
            return 4;
        }
        stream = fn_80275850(
            platform::read_u32(source + 0x10) as usize,
            platform::read_u32(param_3),
            (this + 0x290) as u32,
            0x200,
        ) as usize;
        if stream == 0 {
            return 6;
        }
        let vtable = platform::read_u32(stream) as usize;
        let required = call0_r(platform::read_u32(vtable + 0x40) as usize);
        let available = fn_802700f0(file) as u32;
        if available < required {
            platform::call0(platform::read_u32(vtable + 0x10) as usize);
            return 5;
        }
    }

    let mut result = platform::call4(
        FUN_802724E0,
        this as u32,
        platform::read_u32(source + 0x30),
        length,
        (source + 0x20) as u32,
    );
    while result != 0 {
        if result != 1 {
            return 0xff;
        }
        if platform::read_i32(source + 0x48) == 1 {
            if stream != 0 {
                platform::call0(
                    platform::read_u32(platform::read_u32(stream) as usize + 0x10) as usize,
                );
            }
            return 9;
        }
        let next = if platform::read_i32(source + 0x48) == 0 {
            0
        } else {
            platform::read_i32(source + 0x4c) - 0xf0
        };
        if this as i32 == next {
            if stream != 0 {
                platform::call0(
                    platform::read_u32(platform::read_u32(stream) as usize + 0x10) as usize,
                );
            }
            return 9;
        }
        platform::call2(FUN_80266250, next as u32, 0);
        result = platform::call4(
            FUN_802724E0,
            this as u32,
            platform::read_u32(source + 0x30),
            length,
            (source + 0x20) as u32,
        );
    }

    call_f64_u32(
        FUN_80267400,
        ((platform::read_u32(param_3 + 0x10) ^ 0x8000_0000) as i32 as f64)
            / platform::read_f32(SDA2_PLAY_SCALE) as f64,
        this as u32,
    );
    platform::call2(FUN_80267530, this as u32, platform::read_u32(param_3 + 0x14));
    platform::call2(FUN_80267570, this as u32, platform::read_u32(param_3 + 0x18));
    platform::call2(FUN_802675B0, this as u32, platform::read_u32(param_3 + 0x1c));
    fn_802728a0(this, platform::read_u32(param_4 + 0x0c) as u8);
    fn_802728b0(this, platform::read_u8(param_4 + 0x10));
    fn_802728c0(
        this,
        platform::read_u32(source + 0x34),
        platform::read_u32(source + 0x38),
    );

    let (start_mode, start_delta) = match mode {
        0 => (1u32, delta),
        1 => (0u32, delta),
        _ => (0u32, 0),
    };

    if resolved == 0 {
        fn_802725d0(this, stream, value, start_mode, start_delta);
    } else {
        fn_80270d20(header_c.as_mut_ptr() as usize, resolved as u32);
        let payload = fn_80270d90(header_c.as_mut_ptr() as usize);
        fn_80272560(this, payload, value, start_mode, start_delta);
    }

    0
}

// ===========================================================================
// 0x80279880
// ===========================================================================

/// Starts a sequence through the small element record and the block player.
///
/// The record at `source + 0xC8` is rebuilt by `0x8027F650` with two half-word
/// ids from the caller's record. While that rebuild keeps reporting `1`, the
/// routine looks for the already-live player at `source + 0x70` and retries;
/// any other non-zero result is an error. When the rebuild reports `0`, a
/// stream is opened with [`fn_80275850`] from `source + 0x10` into the
/// player's `+0xEB4` buffer, armed by [`fn_8027f6d0`], and the tempo/volume
/// trio is written.
///
/// # Original
/// - Ghidra: `FUN_80279880` at `0x80279880` (432 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80279880_FUN_80279880.c`
/// - Removed: the six modelled `_restgpr_26` epilogue calls (`0x802DCDBC`);
///   the matching `_savegpr_26` prologue (`0x802DCD70`) was never modelled and
///   each exit now returns its status directly.
/// - observed: the `0x802DCD70` pair (high = source, low = player),
///   `FUN_8027f650(player,source + 200,*(u16 *)(param_4 + 4),
///   *(u16 *)(param_4 + 6))`, the `source + 0x70` / `+0x74 - 0xf0` retry loop
///   with `FUN_80266250`, `FUN_80275850(*(source + 0x10),*param_3,
///   player + 0xeb4,0x200)`, `FUN_8027f6d0`, the `FUN_80267400` tempo and the
///   three `FUN_802675xx` volume calls.
///
/// # Arguments
/// - `this`: sequence player (incoming `r4`).
/// - `source`: sound-system wrapper object (incoming `r3`).
/// - `param_3`: request block (`[0]` = id, `[4..7]` = clock/volumes).
/// - `param_4`: record with the two half-word ids at `+4`/`+6`.
/// - `mode`: start mode `0..2`.
/// - `delta`: start delta.
///
/// # Side effects
/// - rebuilds and arms the player's sub-object, opens a stream and writes the
///   tempo/volume fields.
///
/// # Safety
/// Every pointer must be a live block of the original size.
///
/// # Confidence
/// medium - the loop, calls and offsets are literal; the `+0x74 - 0xF0`
/// identity rule is inferred.
pub unsafe fn fn_80279880(
    this: usize,
    source: usize,
    param_3: usize,
    param_4: usize,
    mode: i32,
    delta: u32,
) -> u32 {
    let mut result = platform::call4(
        FUN_8027F650,
        this as u32,
        (source + 200) as u32,
        platform::read_u16(param_4 + 4) as u32,
        platform::read_u16(param_4 + 6) as u32,
    );

    loop {
        if result == 0 {
            let (start_mode, start_delta) = match mode {
                0 => (1u32, delta),
                1 => (0u32, 0),
                2 => (0u32, delta),
                _ => (0u32, 0),
            };
            let stream = fn_80275850(
                platform::read_u32(source + 0x10) as usize,
                platform::read_u32(param_3),
                (this + 0xeb4) as u32,
                0x200,
            );
            if stream == 0 {
                return 6;
            }
            let armed = fn_8027f6d0(this, start_mode, start_delta, stream);
            if armed == 0 {
                return 0xff;
            }
            call_f64_u32(
                FUN_80267400,
                ((platform::read_u32(param_3 + 0x10) ^ 0x8000_0000) as i32 as f64)
                    / platform::read_f32(SDA2_PLAY_SCALE) as f64,
                this as u32,
            );
            platform::call2(FUN_80267530, this as u32, platform::read_u32(param_3 + 0x14));
            platform::call2(FUN_80267570, this as u32, platform::read_u32(param_3 + 0x18));
            platform::call2(FUN_802675B0, this as u32, platform::read_u32(param_3 + 0x1c));
            return 0;
        }

        if result != 1 {
            return 0xff;
        }

        if platform::read_i32(source + 0x70) == 1 {
            return 9;
        }
        let next = if platform::read_i32(source + 0x70) == 0 {
            0
        } else {
            platform::read_i32(source + 0x74) - 0xf0
        };
        if this as i32 == next {
            return 9;
        }
        platform::call2(FUN_80266250, next as u32, 0);

        result = platform::call4(
            FUN_8027F650,
            this as u32,
            (source + 200) as u32,
            platform::read_u16(param_4 + 4) as u32,
            platform::read_u16(param_4 + 6) as u32,
        );
    }
}

// ===========================================================================
// 0x80279A30
// ===========================================================================

/// Starts a sequence resolved directly from a bank-group id.
///
/// The id is looked up with [`fn_80278790`]; a zero result reports `4`.
/// Otherwise `0x80283EC0` starts the entry with the caller's request words,
/// then the tempo and three volume words are written and
/// `0x80284040`/`0x80284050` apply two post-start values. A zero start result
/// reports `0xFF`, success reports `0`.
///
/// # Original
/// - Ghidra: `FUN_80279a30` at `0x80279A30` (312 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80279a30_FUN_80279a30.c`
/// - Removed: the modelled `_restgpr_26` epilogue call (`0x802DCDBC`); the
///   matching `_savegpr_26` prologue (`0x802DCD70`) was never modelled and the
///   status is returned directly.
/// - observed: the `0x802DCD70` pair (high = source, low = player),
///   `FUN_80278790(source,*param_3)`, the mode mapping, the seven-argument
///   `FUN_80283ec0(player,id,*param_4,mode,delta,source + 0x28,*param_3)`, the
///   `FUN_80267400` tempo, the three `FUN_802675xx` calls,
///   `FUN_80284040(player,param_4[1])`, `FUN_80284050(player,*(u8 *)(param_4+8))`
///   and the `0x802DCDBC` exits.
///
/// # Arguments
/// - `this`: sequence player (incoming `r4`).
/// - `source`: sound-system wrapper object (incoming `r3`).
/// - `param_3`: request block (`[0]` = bank id, `[4..7]` = clock/volumes).
/// - `param_4`: second block (`[0]` = entry id, `[1]` = extra, `+8` = byte).
/// - `mode`: start mode.
/// - `delta`: start delta.
///
/// # Side effects
/// - starts the sequence and writes its tempo/volume and two extra fields.
///
/// # Safety
/// Both blocks must be live for the fields the export reads.
///
/// # Confidence
/// medium - the calls and offsets are literal; the
/// `0x80284040`/`0x80284050` roles are inferred.
pub unsafe fn fn_80279a30(
    this: usize,
    source: usize,
    param_3: usize,
    param_4: usize,
    mode: i32,
    delta: u32,
) -> u32 {
    let mut status = 4u32;
    let id = fn_80278790(source, platform::read_u32(param_3));
    if id != 0 {
        let (start_mode, start_delta) = match mode {
            0 => (1u32, delta),
            1 => (0u32, 0),
            2 => (0u32, delta),
            _ => (0u32, 0),
        };
        let started = platform::call7(
            FUN_80283EC0,
            this as u32,
            id,
            platform::read_u32(param_4),
            start_mode,
            start_delta,
            (source + 0x28) as u32,
            platform::read_u32(param_3),
        );
        if started == 0 {
            status = 0xff;
        } else {
            call_f64_u32(
                FUN_80267400,
                ((platform::read_u32(param_3 + 0x10) ^ 0x8000_0000) as i32 as f64)
                    / platform::read_f32(SDA2_PLAY_SCALE) as f64,
                this as u32,
            );
            platform::call2(FUN_80267530, this as u32, platform::read_u32(param_3 + 0x14));
            platform::call2(FUN_80267570, this as u32, platform::read_u32(param_3 + 0x18));
            platform::call2(FUN_802675B0, this as u32, platform::read_u32(param_3 + 0x1c));
            platform::call2(FUN_80284040, this as u32, platform::read_u32(param_4 + 4));
            platform::call2(
                FUN_80284050,
                this as u32,
                platform::read_u8(param_4 + 8) as u32,
            );
            status = 0;
        }
    }
    status
}

// ===========================================================================
// 0x80279B70
// ===========================================================================

/// Returns whether one sequence slot is present or can be loaded.
///
/// The slot is only considered when the player's `+0x10` archive handle is
/// non-null and [`fn_802756b0`] reports it usable. The slot index must be below
/// the count returned through `0x80275710`. A live cache entry at
/// `+0x14[slot * 2 + 1]` reports success immediately; otherwise the caller's
/// `source` address is used by `0x80277050` to build a loader block, which
/// `0x802770D0` opens. On success the cache entry and its `+4` word are filled
/// and the loader is released with `0x80277090(block, -1)`.
///
/// # Original
/// - Ghidra: `FUN_80279b70` at `0x80279B70` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80279b70_FUN_80279b70.c`
/// - observed: the `+0x10` null test, `FUN_802756b0`, the
///   `FUN_80275710` count and `param_2 < uVar3` test, the `+0x14` cache walk
///   with `param_2 * 2 + 1`, `FUN_80277050(auStack_230,*(+0x10))`,
///   `FUN_802770d0(auStack_230,param_2,param_3,local_238,param_4)`, the two
///   cache stores (`+param_2*8+4` and `+param_2*8+8`) and
///   `FUN_80277090(auStack_230,0xffffffff)` on both exits.
///
/// # Arguments
/// - `this`: player object; `+0x10` archive handle, `+0x14` slot cache.
/// - `slot`: sequence slot index.
/// - `source`: archive source address passed to the loader.
/// - `chunk`: loader chunk size forwarded to `0x802770D0`.
///
/// # Returns
/// `1` when the slot is live or was loaded, `0` otherwise.
///
/// # Safety
/// `this` and the cache must be live; `source` must be a valid archive address.
///
/// # Confidence
/// medium - every guard, call and store is literal; the loader block size
/// (`0x220`) and the cache's stride are inferred from the surrounding players.
pub unsafe fn fn_80279b70(this: usize, slot: u32, source: u32, chunk: u32) -> u32 {
    let handle = platform::read_u32(this + 0x10);
    if handle == 0 {
        return 0;
    }
    if fn_802756b0(handle as usize) == 0 {
        return 0;
    }
    let count = platform::call1(FUN_80275710, handle);
    if slot >= count {
        return 0;
    }

    let cache = platform::read_u32(this + 0x14) as usize;
    let mut entry = 0u32;
    if cache != 0 {
        if slot < platform::read_u32(cache) {
            entry = platform::read_u32(cache + slot as usize * 8 + 4);
        }
    }
    if entry != 0 {
        return 1;
    }
    if source == 0 {
        return 0;
    }

    let mut loader = [0u32; 0x220 / 4];
    let mut second = [0u32; 2];
    let loader_ptr = loader.as_mut_ptr() as u32;
    platform::call2(FUN_80277050, loader_ptr, handle);
    let opened = fn_802770d0(
        loader.as_mut_ptr() as usize,
        slot,
        source as usize,
        second.as_mut_ptr() as usize,
        chunk,
    );
    if opened == 0 {
        fn_80277090(loader.as_mut_ptr() as usize, -1);
        return 0;
    }

    if cache != 0 {
        let entry_ptr = cache + slot as usize * 8;
        platform::write_u32(entry_ptr + 4, opened);
        platform::write_u32(entry_ptr + 8, second[0]);
    }
    fn_80277090(loader.as_mut_ptr() as usize, -1);
    1
}

// ===========================================================================
// 0x80279CE0
// ===========================================================================

/// Resolves a sequence name and starts it.
///
/// `0x80275750` turns the name stored behind the player's `+0x10` handle into a
/// slot index. `-1` reports failure; any other index is forwarded to
/// [`fn_80279b70`] with the caller's `source` and `chunk`.
///
/// # Original
/// - Ghidra: `FUN_80279ce0` at `0x80279CE0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80279ce0_FUN_80279ce0.c`
/// - observed: `iVar1 = FUN_80275750(*(undefined4 *)(param_1 + 0x10));`, the
///   `iVar1 == -1` test and the `FUN_80279b70(param_1,iVar1,param_3,param_4)`
///   tail.
///
/// # Arguments
/// - `this`: player object.
/// - `source`: archive source address forwarded to the loader.
/// - `chunk`: loader chunk size.
///
/// # Returns
/// `1` when the slot is live or loaded, `0` otherwise.
///
/// # Safety
/// The player's `+0x10` handle must be live.
///
/// # Confidence
/// high - the resolve call, the `-1` guard and the forwarded call are literal.
pub unsafe fn fn_80279ce0(this: usize, source: u32, chunk: u32) -> u32 {
    let slot = platform::call1(FUN_80275750, platform::read_u32(this + 0x10));
    if slot == 0xffff_ffff {
        0
    } else {
        fn_80279b70(this, slot, source, chunk)
    }
}

// ===========================================================================
// 0x8027AA00
// ===========================================================================

/// Rebuilds the 0x80-entry priority bucket table from the manager list.
///
/// The work is done under the sequence-manager mutex and only when the list at
/// `+0x0C` holds at least two nodes. On first use the bucket array at
/// `0x803FCCB0` is registered with `0x802DC9A0` and its destructor with
/// `0x802DC780`. The list is then drained into the buckets (priority clamped to
/// `0..0x7F`), and the buckets are drained back into the list in bucket order.
///
/// # Original
/// - Ghidra: `FUN_8027aa00` at `0x8027AA00` (388 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027aa00_FUN_8027aa00.c`
/// - observed: the `0x802DCD74` low word used as the list object, the
///   `*(uint *)(iVar1 + 0xc) < 2` guard, the `r13 - 0x4948` flag,
///   `FUN_802dc9a0(&DAT_803fccb0,&LAB_8027a5f0,FUN_8027a610,0xc,0x80)`,
///   `FUN_802dc780(0,&LAB_8027ab90,&DAT_803fcca0)`, the
///   `*(byte *)(iVar2 + -0x68) + *(int *)(iVar2 + -0xb0)` clamp, the
///   `FUN_802290c0` / `FUN_80229160` drain loops, and the four manager
///   lock/unlock pairs.
/// - `// inferred:` the function has no declared parameter, so the `r3`
///   recovered from the save stub is the list owner.
///
/// # Arguments
/// - `this`: object whose `+0x0C`/`+0x10` list is rebuilt.
///
/// # Side effects
/// - initialises the static bucket table on first use and rewrites the list.
///
/// # Safety
/// `this` must be a live list owner and the static bucket table writable.
///
/// # Confidence
/// medium - the guard, clamp, calls and stores are literal; the missing
/// parameter and the bucket field roles are inferred.
pub unsafe fn fn_8027aa00(this: usize) {
    lock_seq_manager();

    if platform::read_u32(this + 0x0c) < 2 {
        unlock_seq_manager();
        return;
    }

    if platform::read_u8(SEQ_BUCKET_READY) == 0 {
        platform::call5(
            FUN_802DC9A0,
            SEQ_BUCKETS as u32,
            LAB_8027A5F0,
            fn_8027a610 as *const () as usize as u32,
            0x0c,
            0x80,
        );
        platform::call3(FUN_802DC780, 0, LAB_8027AB90, 0x803F_CCA0);
        platform::write_u8(SEQ_BUCKET_READY, 1);
    }

    while platform::read_u32(this + 0x0c) != 0 {
        let mut node = platform::read_u32(this + 0x10);
        platform::call2(FUN_802290C0, (this + 0x0c) as u32, (&mut node as *mut u32) as u32);
        let priority = clamp_priority(
            platform::read_u8(node as usize - 0x68) as u32,
            platform::read_u32(node as usize - 0xb0),
        );
        let slot = SEQ_BUCKETS + priority as usize * 0x0c;
        let mut bucket_ref = (SEQ_BUCKETS_LINK + priority as usize * 0x0c) as u32;
        platform::call3(
            FUN_80229160,
            slot as u32,
            (&mut bucket_ref as *mut u32) as u32,
            node,
        );
    }

    let mut bucket = SEQ_BUCKETS;
    let mut index = 0;
    while index < 0x80 {
        while platform::read_u32(bucket) != 0 {
            let mut node = platform::read_u32(bucket + 4);
            platform::call2(FUN_802290C0, bucket as u32, (&mut node as *mut u32) as u32);
            let mut head_ref = (this + 0x10) as u32;
            platform::call3(
                FUN_80229160,
                (this + 0x0c) as u32,
                (&mut head_ref as *mut u32) as u32,
                node,
            );
        }
        index += 1;
        bucket += 0x0c;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x8027ABB0
// ===========================================================================

/// Inserts one bank object into the priority buckets when it is admissible.
///
/// The object's priority is the clamped sum of its `+0x98` byte and `+0x50`
/// word. Insertion fails when the list at `+0x24` is empty, when the list's
/// limit is not below its head count, when the head object is missing, or when
/// the new priority is below a head priority. Every rejected head is released
/// through its vtable slot `+0x10`. On success the object is linked into the
/// `+0xF8` list, inserted into the `+0x100` ordering by priority, and
/// `0x80267320` links it to the bank.
///
/// # Original
/// - Ghidra: `FUN_8027abb0` at `0x8027ABB0` (468 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027abb0_FUN_8027abb0.c`
/// - observed: `param_1[9] <= *param_1`, the `iVar1 + -0x100 == 0` test, the
///   two clamp sites, the head release through `+0x10`,
///   `FUN_80229160(param_1,&local_14,param_2 + 0xf8)`, the
///   `local_18 != param_1 + 4` priority walk, the
///   `FUN_80229160(param_1 + 3,&local_18,param_2 + 0x100)` insert, the
///   `FUN_80267320(param_2,param_1)` link and five manager lock/unlock pairs.
///
/// # Arguments
/// - `list`: bucket/list owner.
/// - `object`: bank object to insert.
///
/// # Returns
/// `1` when inserted, `0` when rejected.
///
/// # Safety
/// `list` must be a live bucket owner and `object` a live bank object.
///
/// # Confidence
/// medium - the guards, clamp and calls are literal; the `+0xF8`/`+0x100`
/// link roles are inferred from the list walk.
pub unsafe fn fn_8027abb0(list: usize, object: usize) -> u32 {
    lock_seq_manager();

    let priority = clamp_priority(
        platform::read_u8(object + 0x98) as u32,
        platform::read_u32(object + 0x50),
    );

    if platform::read_u32(list + 0x24) == 0 {
        unlock_seq_manager();
        return 0;
    }

    while platform::read_u32(list + 0x24) <= platform::read_u32(list) {
        let node = platform::read_u32(list + 0x10) as usize;
        if node.wrapping_sub(0x100) == 0 {
            unlock_seq_manager();
            return 0;
        }
        let node_priority = clamp_priority(
            platform::read_u8(node - 0x68) as u32,
            platform::read_u32(node - 0xb0),
        );
        if (priority as i32) < (node_priority as i32) {
            unlock_seq_manager();
            return 0;
        }
        let owner = platform::read_u32(node - 0x100) as usize;
        platform::call0(platform::read_u32(owner + 0x10) as usize);
    }

    let mut first_ref = (list + 4) as u32;
    platform::call3(
        FUN_80229160,
        list as u32,
        (&mut first_ref as *mut u32) as u32,
        (object + 0xf8) as u32,
    );

    lock_seq_manager();
    let mut node = platform::read_u32(list + 0x10) as usize;
    let head = list + 0x10;
    while node != head {
        let node_priority = clamp_priority(
            platform::read_u8(node - 0x68) as u32,
            platform::read_u32(node - 0xb0),
        );
        let object_priority = clamp_priority(
            platform::read_u8(object + 0x98) as u32,
            platform::read_u32(object + 0x50),
        );
        if (object_priority as i32) < (node_priority as i32) {
            break;
        }
        node = platform::read_u32(node) as usize;
    }

    let mut second_ref = node as u32;
    platform::call3(
        FUN_80229160,
        (list + 0x0c) as u32,
        (&mut second_ref as *mut u32) as u32,
        (object + 0x100) as u32,
    );
    unlock_seq_manager();

    platform::call2(FUN_80267320, object as u32, list as u32);
    unlock_seq_manager();
    1
}

// ===========================================================================
// 0x8027B720
// ===========================================================================

/// Starts the sequence thread once.
///
/// The manager's `+0x394` byte is the one-shot guard. On the first call it is
/// set, the caller's start word is stored at `+0x350`, the global thread
/// structures are initialised when `r13 - 0x4938` is clear (the same block as
/// [`fn_8027b5f0`]), and `0x801B5130` creates the thread with
/// [`fn_8027b960`] as its entry point. A nonzero create result is cleaned up by
/// `0x801B5860` and reported as `1`; a zero result reports `0`. A second call
/// reports `1`.
///
/// # Original
/// - Ghidra: `FUN_8027b720` at `0x8027B720` (300 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b720_FUN_8027b720.c`
/// - Removed: the `_restgpr_26` epilogue call (`0x802DCDBC`); the matching
///   `_savegpr_26` prologue (`0x802DCD70`) was never modelled and the result is
///   returned directly.
/// - observed: the `+0x394` guard and store, `+0x350 = param_3`, the `r13 - 0x4938` block,
///   `FUN_801b5130(iVar1,FUN_8027b960,&DAT_80412bf0,param_3 + param_4,
///   param_4,(int)uVar3,0)`, `FUN_801b5860`, and
///   `(-uVar2 | uVar2) >> 0x1f`.
///
/// # Arguments
/// - `this`: sequence manager (incoming `r3`).
/// - `token`: second incoming word, forwarded as the sixth create word
///   (incoming `r4`).
/// - `start`: start time stored at `+0x350`.
/// - `size`: length added to `start` for the create call.
///
/// # Side effects
/// - initialises the thread globals on first use, sets `+0x394`/`+0x350` and
///   creates the sequence thread.
///
/// # Safety
/// `this` must be the live sequence manager.
///
/// # Confidence
/// medium - the guard, all stores and the create call are literal; the thread
/// argument roles are inferred.
pub unsafe fn fn_8027b720(this: usize, token: u32, start: i32, size: i32) -> u32 {
    let result;
    if platform::read_u8(this + 0x394) == 0 {
        platform::write_u8(this + 0x394, 1);
        platform::write_u32(this + 0x350, start as u32);

        if platform::read_u8(SEQ_MANAGER_READY) == 0 {
            platform::write_u32(SEQ_MANAGER_INSTANCE + 0x350, 0);
            platform::write_u32(SEQ_MANAGER_INSTANCE + 0x36c, 0);
            platform::write_u32(SEQ_MANAGER_INSTANCE + 0x370, 0);
            platform::write_u32(SEQ_MANAGER_INSTANCE + 0x378, 0);
            platform::write_u32(
                SEQ_MANAGER_INSTANCE + 0x37c,
                (SEQ_MANAGER_INSTANCE + 0x37c) as u32,
            );
            platform::write_u32(
                SEQ_MANAGER_INSTANCE + 0x380,
                (SEQ_MANAGER_INSTANCE + 0x37c) as u32,
            );
            platform::write_u32(SEQ_MANAGER_INSTANCE + 0x384, 0);
            platform::write_u32(
                SEQ_MANAGER_INSTANCE + 0x388,
                (SEQ_MANAGER_INSTANCE + 0x388) as u32,
            );
            platform::write_u32(
                SEQ_MANAGER_INSTANCE + 0x38c,
                (SEQ_MANAGER_INSTANCE + 0x388) as u32,
            );
            platform::write_u32(SEQ_MANAGER_INSTANCE + 0x394, 0);
            platform::write_u8(SEQ_MANAGER_INSTANCE + 0x395, 0);
            platform::call3(
                FUN_801B2390,
                (SEQ_MANAGER_INSTANCE + 0x320) as u32,
                (SEQ_MANAGER_INSTANCE + 0x340) as u32,
                4,
            );
            platform::call1(FUN_801B4B20, (SEQ_MANAGER_INSTANCE + 0x318) as u32);
            platform::call1(MUTEX_INIT, (SEQ_MANAGER_INSTANCE + 0x354) as u32);
            platform::call3(
                FUN_802DC780,
                SEQ_MANAGER_INSTANCE as u32,
                fn_8027b6b0 as *const () as usize as u32,
                (SEQ_MANAGER_INSTANCE - 0x10) as u32,
            );
            platform::write_u8(SEQ_MANAGER_READY, 1);
        }

        let created = platform::call7(
            FUN_801B5130,
            this as u32,
            fn_8027b960 as *const () as usize as u32,
            SEQ_MANAGER_INSTANCE as u32,
            start.wrapping_add(size) as u32,
            size as u32,
            token,
            0,
        );
        if created != 0 {
            platform::call1(FUN_801B5860, this as u32);
        }
        result = (created.wrapping_neg() | created) >> 31;
    } else {
        result = 1;
    }
    result
}

// ===========================================================================
// 0x8027B850
// ===========================================================================

/// Initialises the thread globals and drains the pending-destruction list.
///
/// On first use the same manager block as [`fn_8027b5f0`] is initialised.
/// When the scheduler flag `+0x395` is clear, `0x801B23F0` clears one
/// interrupt-mask bit. Every node on the `+0x388` list is then released through
/// its owner's vtable slot `+0x10`, and the two tail ticks `0x80282EF0` and
/// `0x802833B0` run.
///
/// # Original
/// - Ghidra: `FUN_8027b850` at `0x8027B850` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b850_FUN_8027b850.c`
/// - observed: the identical `r13 - 0x4938` initialisation block,
///   `DAT_80412f85 == '\0'` around `FUN_801b23f0(&DAT_80412f10,1,0)`, the
///   `puVar2 != &DAT_80412f78` walk with
///   `(**(code **)(puVar2[2] + 0x10))()`, and `FUN_80282ef0(); FUN_802833b0();`.
///
/// # Side effects
/// - initialises the thread globals on first use, clears one mask bit and
///   releases every queued object.
///
/// # Safety
/// The manager's static storage must be writable.
///
/// # Confidence
/// medium - every store and call is literal; the vtable owner at `node + 8`
/// and the two tail routines are not named by the export.
pub unsafe fn fn_8027b850() {
    if platform::read_u8(SEQ_MANAGER_READY) == 0 {
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x350, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x36c, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x370, 0);
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x378, 0);
        platform::write_u32(
            SEQ_MANAGER_INSTANCE + 0x37c,
            (SEQ_MANAGER_INSTANCE + 0x37c) as u32,
        );
        platform::write_u32(
            SEQ_MANAGER_INSTANCE + 0x380,
            (SEQ_MANAGER_INSTANCE + 0x37c) as u32,
        );
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x384, 0);
        platform::write_u32(
            SEQ_MANAGER_INSTANCE + 0x388,
            (SEQ_MANAGER_INSTANCE + 0x388) as u32,
        );
        platform::write_u32(
            SEQ_MANAGER_INSTANCE + 0x38c,
            (SEQ_MANAGER_INSTANCE + 0x388) as u32,
        );
        platform::write_u32(SEQ_MANAGER_INSTANCE + 0x394, 0);
        platform::write_u8(SEQ_MANAGER_INSTANCE + 0x395, 0);
        platform::call3(
            FUN_801B2390,
            (SEQ_MANAGER_INSTANCE + 0x320) as u32,
            (SEQ_MANAGER_INSTANCE + 0x340) as u32,
            4,
        );
        platform::call1(FUN_801B4B20, (SEQ_MANAGER_INSTANCE + 0x318) as u32);
        platform::call1(MUTEX_INIT, (SEQ_MANAGER_INSTANCE + 0x354) as u32);
        platform::call3(
            FUN_802DC780,
            SEQ_MANAGER_INSTANCE as u32,
            fn_8027b6b0 as *const () as usize as u32,
            (SEQ_MANAGER_INSTANCE - 0x10) as u32,
        );
        platform::write_u8(SEQ_MANAGER_READY, 1);
    }

    if platform::read_u8(SEQ_MANAGER_INSTANCE + 0x395) == 0 {
        platform::call3(FUN_801B23F0, (SEQ_MANAGER_INSTANCE + 0x320) as u32, 1, 0);
    }

    let head = SEQ_MANAGER_INSTANCE + 0x388;
    let mut link = platform::read_u32(head) as usize;
    while link != head {
        let next = platform::read_u32(link) as usize;
        let owner = platform::read_u32(link + 8) as usize;
        platform::call0(platform::read_u32(owner + 0x10) as usize);
        link = next;
    }

    platform::call0(FUN_80282EF0);
    platform::call0(FUN_802833B0);
}

// ===========================================================================
// 0x8027B960
// ===========================================================================

/// The sequence thread body.
///
/// It registers [`fn_8027b850`] as the thread's cleanup callback on the
/// `+0x36C` handle, then receives messages from the queue at `+0x320`. A type
/// `1` message runs one frame through [`fn_8027baf0`]; a type `2` message ends
/// the loop. The `+0x388` list is then drained through each owner's vtable slot
/// `+0x14`, and the callback is unregistered.
///
/// # Original
/// - Ghidra: `FUN_8027b960` at `0x8027B960` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b960_FUN_8027b960.c`
/// - observed: `FUN_80260860()`, `FUN_80260ed0(uVar1,param_1 + 0x36c,
///   FUN_8027b850)`, the `FUN_801b24c0(param_1 + 800,local_18,1)` receive with
///   the `local_18[0] == 1` / `!= 2` loop, `FUN_8027baf0(param_1)`, the
///   `param_1 + 0x388` drain through `puVar3[2] + 0x14`, and
///   `FUN_80260f40(uVar1,param_1 + 0x36c)`.
///
/// # Arguments
/// - `this`: sequence manager passed by the thread creator.
///
/// # Returns
/// Always `0`.
///
/// # Safety
/// `this` must be the live sequence manager whose queue and lists are intact.
///
/// # Confidence
/// medium - the registration, receive loop and drain are literal; the message
/// types are inferred from the `1`/`2` tests.
pub unsafe fn fn_8027b960(this: usize) -> u32 {
    let manager = call0_r(FUN_80260860);
    platform::call3(
        FUN_80260ED0,
        manager,
        (this + 0x36c) as u32,
        fn_8027b850 as *const () as usize as u32,
    );

    let mut message = [0u32; 3];
    loop {
        loop {
            platform::call3(
                FUN_801B24C0,
                (this + 0x320) as u32,
                message.as_mut_ptr() as u32,
                1,
            );
            if message[0] != 1 {
                break;
            }
            fn_8027baf0(this);
        }
        if message[0] == 2 {
            break;
        }
    }

    let head = this + 0x388;
    let mut link = platform::read_u32(head) as usize;
    while link != head {
        let next = platform::read_u32(link) as usize;
        let owner = platform::read_u32(link + 8) as usize;
        platform::call0(platform::read_u32(owner + 0x14) as usize);
        link = next;
    }

    let manager = call0_r(FUN_80260860);
    platform::call2(FUN_80260F40, manager, (this + 0x36c) as u32);
    0
}

// ===========================================================================
// 0x8027BAF0
// ===========================================================================

/// Runs one frame of the sequence thread.
///
/// The manager mutex is held while the `+0x37C` list is drained through each
/// owner's vtable slot `+0x0C`. The platform tick is sampled, the audio and
/// bank ticks run, the `+0x388` list is drained through slot `+0x0C`, more
/// scheduler ticks run, and the elapsed tick count is stored at `+0x390`.
/// Finally the `+0x37C` list is drained again through slot `+0x10`.
///
/// # Original
/// - Ghidra: `FUN_8027baf0` at `0x8027BAF0` (256 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027baf0_FUN_8027baf0.c`
/// - Removed: the `_restgpr_27` epilogue call (`0x802DCDC0`); the matching
///   `_savegpr_27` prologue (`0x802DCD74`) was never modelled and `this` is the
///   incoming `r3`.
/// - observed: the two `+0x37C` / `+0x388` walks with slots `+0x0C` / `+0x10`, the
///   `FUN_801b5fa0` pair around the tick block, `FUN_80264ae0`,
///   `FUN_80265100`, `FUN_80260860`, `FUN_80260af0`, `FUN_80267c40`,
///   `FUN_80267dd0`, `FUN_80283a30`, `FUN_80282ef0`, `FUN_80283300` and the
///   `+0x390` elapsed store.
///
/// # Arguments
/// - `this`: sequence manager passed by the thread body.
///
/// # Side effects
/// - runs the audio/scheduler ticks, releases queued objects and stores the
///   elapsed tick count at `+0x390`.
///
/// # Safety
/// `this` must be the live sequence manager and both lists must be intact.
///
/// # Confidence
/// medium - every call, walk and store is literal; the tick routine names are
/// taken from their addresses only.
pub unsafe fn fn_8027baf0(this: usize) {
    lock_seq_manager();

    let first_head = this + 0x37c;
    let mut link = platform::read_u32(first_head) as usize;
    while link != first_head {
        let next = platform::read_u32(link) as usize;
        let owner = platform::read_u32(link + 8) as usize;
        platform::call0(platform::read_u32(owner + 0x0c) as usize);
        link = next;
    }

    let started = call0_r(FUN_801B5FA0);
    platform::call0(FUN_80264AE0);
    platform::call0(FUN_80265100);
    call0_r(FUN_80260860);
    platform::call0(FUN_80260AF0);

    let second_head = this + 0x388;
    let mut link = platform::read_u32(second_head) as usize;
    while link != second_head {
        let next = platform::read_u32(link) as usize;
        let owner = platform::read_u32(link + 8) as usize;
        platform::call0(platform::read_u32(owner + 0x0c) as usize);
        link = next;
    }

    platform::call0(FUN_80267C40);
    platform::call0(FUN_80267DD0);
    platform::call0(FUN_80283A30);
    platform::call0(FUN_80282EF0);
    platform::call0(FUN_80283300);

    let finished = call0_r(FUN_801B5FA0);
    platform::write_u32(this + 0x390, finished.wrapping_sub(started));

    let mut link = platform::read_u32(first_head) as usize;
    while link != first_head {
        let next = platform::read_u32(link) as usize;
        let owner = platform::read_u32(link + 8) as usize;
        platform::call0(platform::read_u32(owner + 0x10) as usize);
        link = next;
    }

    platform::call1(MUTEX_UNLOCK, (call0_r(SEQ_MANAGER) + 0x354) as u32);
}

// ===========================================================================
// 0x8027D660
// ===========================================================================

/// Loads one block of element data out of an `RSTM` stream and starts it.
///
/// The block mutex at `0x80416FA0` is held for the whole load. After the
/// section header is validated by [`fn_8027c210`], the entry is decoded by
/// [`fn_8027c430`] and [`fn_8027c3e0`]; the element count `+0x810` is taken
/// from the entry when it is still zero, the `+0x80C` records are decoded by
/// [`fn_8027c470`], and when the block's `+0xC0` mode is `3` the paired table
/// is split by [`fn_8027c4b0`]. A non-zero `frames` count then reads one row
/// with [`fn_8027c600`] and copies the two half-word arrays into the element
/// records. [`fn_8027dbd0`] allocates the playback slots, the `+0x124`
/// element records are appended by `0x8027EAF0`, and when the mode is below
/// `3` and `+0xC4` is clear [`fn_8027ea00`] starts the first step.
///
/// # Original
/// - Ghidra: `FUN_8027d660` at `0x8027D660` (768 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027d660_FUN_8027d660.c`
/// - observed: the `0x802DCD74` pair (high = block, low = stream), the
///   `FUN_801b2e60(&DAT_80416fa0)` guard with `FUN_801b2f40` on every exit,
///   `FUN_8027be30`, `FUN_8027c210(&local_64,&DAT_80412fa0,0x4000)`, the
///   `FUN_8027c430` / `FUN_8027c3e0` / `FUN_8027c470` / `FUN_8027c4b0`
///   decoders, the `+0x810` clamp to `0x10`, the `+0xC0 == 3` test, the
///   `FUN_8027c600` row read and the two `+0x840`/`+0x842` stores,
///   `FUN_8027dbd0`, the `FUN_8027eaf0` loop with the `+0x10E` break,
///   `FUN_8027ea00` and the final `FUN_802dcdc0(uVar5)`.
/// - `// observed:` the export prints only the handle for `0x8027C470`, but the
///   call site loads its `out` (`this + 0xB60 + 0x38 * i`) and `index`.
///
/// # Arguments
/// - `this`: sound element block.
/// - `stream`: stream object passed by the save stub's low word.
/// - `mode`: duration unit; `1` means milliseconds.
/// - `frames`: requested frame count, or `0` to skip the row read.
///
/// # Returns
/// `1` on success, `0` on any failure.
///
/// # Safety
/// `this` must be a live block, `stream` a live `RSTM` stream, and the block's
/// mutex must be initialised.
///
/// # Confidence
/// medium - the guards, decoders and stores are literal; the save-stub halves
/// and the exact meaning of `+0xC0`/`+0xC4` are inferred.
pub unsafe fn fn_8027d660(this: usize, stream: usize, mode: i32, frames: i32) -> u32 {
    let mut handle = [stream as u32, 0, 0];
    let mut header = [0u32; 2];
    let mut duration = frames;

    platform::call1(MUTEX_LOCK, DAT_80416FA0 as u32);
    fn_8027be30(header.as_mut_ptr() as usize);

    if fn_8027c210(handle.as_mut_ptr() as usize, SEQ_STREAM_BUFFER, 0x4000) == 0 {
        platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
        return 0;
    }
    if fn_8027c430(handle.as_mut_ptr() as usize, this + 0xC0) == 0 {
        platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
        return 0;
    }

    if platform::read_u32(this + 0x810) == 0 {
        let mut tracks = fn_8027c3e0(handle.as_mut_ptr() as usize) as i32;
        if tracks > 0x10 {
            tracks = 0x10;
        }
        platform::write_u32(this + 0x810, tracks as u32);
    }

    let mut offset = 0usize;
    let mut index = 0i32;
    while index < platform::read_i32(this + 0x80c) {
        if fn_8027c470(handle.as_mut_ptr() as usize, this + offset + 0xB60, index) == 0 {
            platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
            return 0;
        }
        offset += 0x38;
        index += 1;
    }

    if platform::read_i32(this + 0xc0) == 3 {
        let mut offset = 0usize;
        let mut index = 0i32;
        while index < platform::read_i32(this + 0x810) {
            if fn_8027c4b0(
                handle.as_mut_ptr() as usize,
                this + offset + 0x81c,
                this + offset + 0x844,
                index,
            ) == 0
            {
                platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
                return 0;
            }
            offset += 0x34;
            index += 1;
        }

        if duration != 0 {
            if mode == 1 {
                duration = duration * platform::read_i32(this + 0xcc);
                duration /= 1000;
            }
            let mut first = [0u16; 28];
            let mut second = [0u16; 16];
            let row = duration / platform::read_i32(this + 0xe8);
            let count = platform::read_u32(this + 0xc8);
            if fn_8027c600(
                handle.as_mut_ptr() as usize,
                first.as_mut_ptr() as usize,
                second.as_mut_ptr() as usize,
                row as u32,
                count,
            ) == 0
            {
                platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
                return 0;
            }

            let mut element = 0usize;
            let mut pair = 0usize;
            let mut index = 0u32;
            while index < count {
                platform::write_u16(this + element + 0x840, first[pair]);
                platform::write_u16(this + element + 0x842, second[pair]);
                element += 0x34;
                pair += 1;
                index += 1;
            }
        }
    }

    if fn_8027dbd0(this) == 0 {
        platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
        return 0;
    }

    platform::write_u32(this + 0x114, 0);
    let mut index = 0i32;
    while index < platform::read_i32(this + 0x124) {
        platform::call1(FUN_8027EAF0, this as u32);
        platform::write_u32(this + 0x114, platform::read_u32(this + 0x114).wrapping_add(1));
        if platform::read_u8(this + 0x10e) != 0 {
            break;
        }
        index += 1;
    }

    let play_mode = platform::read_u32(this + 0xe0);
    if play_mode < 3 && platform::read_u8(this + 0xc4) == 0 {
        fn_8027ea00(this, play_mode.wrapping_sub(1));
    }

    platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
    1
}

// ===========================================================================
// 0x8027D960
// ===========================================================================

/// Streams one block of element data into the per-element voices.
///
/// The source object is accepted only when `0x802DCD48`'s low word is non-null
/// and its `+0x0C` iterator contains the block marker at `r13 - 0x4A60`; an
/// accepted source has its `+0x68` word set to `1`. After the block mutex is
/// taken and the stream buffer is cleared, the routine reads `param_5` bytes
/// at a time for up to two elements, copying each byte into the local half-word
/// array and into the voice slot selected by `+0x11C * param_6 + +0x818`.
/// When all elements are consumed the accumulated half-words are written to
/// the `+0x84A` table, `+0x10C` is set and the armed counter at `+0x114` is
/// decremented; reaching zero sets `+0x103`.
///
/// # Original
/// - Ghidra: `FUN_8027d960` at `0x8027D960` (624 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027d960_FUN_8027d960.c`
/// - Removed: the two `_restgpr_16` epilogue calls (`0x802DCD94`); the
///   matching `_savegpr_16` prologue (`0x802DCD48`) was never modelled and
///   `this`/`source` are the incoming `r3`/`r4`.
/// - observed: the `piVar7[0x1a] = 1` marker store, the
///   `FUN_801b2e60(&DAT_80416fa0)` / `FUN_801b2f40` pair, the
///   `FUN_801ac440(&DAT_80412fa0,0x4000)` clear, the
///   `+0xD0` offset addition, the `+0x810`/`+0x103`/`+0x114` guards, the
///   two-element batch with `+0x44` seek and `+0x14` read virtuals, the
///   `FUN_80004364` / `FUN_801ac470` voice calls, the `+0x84A` half-word
///   table writes and the `1`/`0` returns.
///
/// # Arguments
/// - `this`: sound element block (incoming `r3`).
/// - `source`: stream/source object (incoming `r4`).
/// - `offset`: starting stream offset, advanced by the `+0xD0` byte count.
/// - `marker`: block marker compared with `r13 - 0x4A60`.
/// - `param_5`: bytes per element.
/// - `param_6`: voice stride multiplier.
/// - `param_7`: when non-zero, fills the accumulated half-word array.
///
/// # Safety
/// `this` must be a live block, `source` a live stream, and the buffer at
/// `0x80412FA0` writable for `0x4000` bytes.
///
/// # Confidence
/// medium - the batch arithmetic, virtual calls and stores are literal; the
/// save-stub halves and the stream/source roles are inferred.
pub unsafe fn fn_8027d960(
    this: usize,
    source: usize,
    offset: u32,
    marker: usize,
    param_5: u32,
    param_6: i32,
    param_7: u32,
) -> u32 {
    let mut target = 0usize;
    if source != 0 {
        let mut node = call0_r(platform::read_u32(source + 8) as usize);
        let mut found = false;
        while node != 0 {
            if node as usize == marker {
                found = true;
                break;
            }
            node = platform::read_u32(node as usize);
        }
        if found {
            target = source;
        }
    }
    if target != 0 {
        platform::write_u32(target + 0x68, 1);
    }

    platform::call1(MUTEX_LOCK, DAT_80416FA0 as u32);
    platform::call2(FUN_801AC440, SEQ_STREAM_BUFFER as u32, 0x4000);

    let mut position = offset.wrapping_add(platform::read_u16(this + 0xd0) as u32);
    let mut values = [0u16; 48];
    let mut index = 0i32;

    loop {
        let count = platform::read_i32(this + 0x810);
        if count <= index {
            if param_7 != 0 && platform::read_i32(this + 0xc0) == 3 {
                let mut element = 0usize;
                let mut value_index = 0usize;
                let mut slot = 0i32;
                while slot < count && slot < 0x10 {
                    platform::write_u16(this + element + 0x84a, values[value_index]);
                    element += 0x34;
                    value_index += 1;
                    slot += 1;
                }
                platform::write_u8(this + 0x10c, 1);
            }

            if platform::read_u8(this + 0x103) == 0 {
                let remaining = platform::read_i32(this + 0x114) - 1;
                platform::write_i32(this + 0x114, remaining);
                if remaining == 0 {
                    platform::write_u8(this + 0x103, 1);
                }
            }

            platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
            return 1;
        }

        let count = platform::read_i32(this + 0x810);
        let mut batch = 2i32;
        if count < index + 2 {
            batch = count - index;
        }
        let bytes = param_5.wrapping_mul(batch as u32);

        let vtable = platform::read_u32(source) as usize;
        platform::call3(
            platform::read_u32(vtable + 0x44) as usize,
            source as u32,
            position,
            0,
        );
        let read = platform::call3(
            platform::read_u32(vtable + 0x14) as usize,
            source as u32,
            SEQ_STREAM_BUFFER as u32,
            bytes,
        );
        if read != bytes {
            platform::call1(MUTEX_UNLOCK, DAT_80416FA0 as u32);
            return 0;
        }

        let mut element = index as usize * 0x34;
        let mut value_index = index as usize;
        let mut step = 0i32;
        while step < batch {
            if param_7 != 0 {
                values[value_index] = platform::read_u8(
                    SEQ_STREAM_BUFFER + param_5 as usize * step as usize,
                ) as u16;
            }
            let voice = platform::read_i32(this + 0x11c)
                .wrapping_mul(param_6)
                .wrapping_add(platform::read_i32(this + element + 0x818));
            let source_byte = (SEQ_STREAM_BUFFER + param_5 as usize * step as usize) as u32;
            platform::call3(FUN_80004364, voice as u32, source_byte, param_5);
            platform::call2(FUN_801AC470, voice as u32, param_5);
            index += 1;
            element += 0x34;
            value_index += 1;
            step += 1;
        }
        position = position.wrapping_add(bytes);
    }
}

// ===========================================================================
// 0x8027DBD0
// ===========================================================================

/// Computes the block's timing fields and claims one allocator slot per track.
///
/// [`fn_8027ede0`] converts the block's start position into a row/index pair.
/// The block's `+0xE4` rate is copied to `+0x11C`, `+0xE0 - 1` to `+0x144`, and
/// `+0xD4 / +0xE8` to `+0x140`. Rates above `0x2000` fail. Otherwise the slot
/// count is the allocator's `+8` word divided by the rate, clamped to `0x20`
/// after a minimum of `4`, and the derived counters and step limits are
/// stored. When `+0x10F` is still clear, `0x8027BCC0` is called once per track
/// and the slots are stored at element `+0x818`; the first failure releases
/// every claimed slot and reports `0`.
///
/// # Original
/// - Ghidra: `FUN_8027dbd0` at `0x8027DBD0` (504 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027dbd0_FUN_8027dbd0.c`
/// - Removed: the four modelled `_restgpr_26` epilogue calls (`0x802DCDBC`);
///   the matching `_savegpr_26` prologue (`0x802DCD70`) was never modelled and
///   each exit now returns its value directly.
/// - observed: `*(uint *)(*(int *)(iVar3 + 0x804) + 8)`,
///   `FUN_8027ede0(iVar3,local_20,&local_24,&local_28)`, the `+0x11C`,
///   `+0x144`, `+0x140`, `+0x120`..`+0x13C` stores, the `< 0x2001` and `< 4`
///   guards, the `0x20` clamp, the `+0x10A` branch, the `FUN_801b1140` lock,
///   the `FUN_8027bcc0` / `FUN_8027bdb0` allocation and rollback, the `+0x10F`
///   byte and the `0x802DCDBC` exits.
///
/// # Arguments
/// - `this`: sound element block.
///
/// # Returns
/// `1` when slots were claimed or already present, `0` otherwise.
///
/// # Safety
/// `this + 0x804` must be the live allocator and the element records writable.
///
/// # Confidence
/// medium - every store and guard is literal; the save-stub half and the
/// derived counter meanings are inferred.
pub unsafe fn fn_8027dbd0(this: usize) -> u32 {
    let allocator = platform::read_u32(this + 0x804) as usize;
    let allocator_size = platform::read_u32(allocator + 8);

    let mut row = [0i32; 8];
    let mut index = 0i32;
    let mut count = 0i32;
    if fn_8027ede0(
        this,
        row.as_mut_ptr() as usize,
        (&mut index as *mut i32) as usize,
        (&mut count as *mut i32) as usize,
    ) == 0
    {
        return 0;
    }

    let rate = platform::read_u32(this + 0xe4);
    platform::write_u32(this + 0x11c, rate);
    platform::write_u32(
        this + 0x144,
        platform::read_u32(this + 0xe0).wrapping_sub(1),
    );
    platform::write_u32(
        this + 0x140,
        platform::read_u32(this + 0xd4) / platform::read_u32(this + 0xe8),
    );

    if (rate as i32) >= 0x2001 {
        return 0;
    }

    let mut slots = allocator_size / rate;
    platform::write_u32(this + 0x120, slots);
    if (slots as i32) < 4 {
        return 0;
    }
    if (slots as i32) > 0x20 {
        slots = 0x20;
        platform::write_u32(this + 0x120, slots);
    }

    let slot_count = platform::read_u32(this + 0x120) as i32;
    let last = slot_count - 1;
    platform::write_u32(this + 0x124, last as u32);
    platform::write_u32(this + 0x118, last as u32);
    platform::write_u32(this + 0x13c, row[0] as u32);
    platform::write_u32(this + 0x130, row[0] as u32);
    platform::write_u32(this + 0x12c, 0);
    platform::write_u32(this + 0x138, 0);

    if platform::read_u8(this + 0x10a) == 0 {
        let remainder = slot_count
            - ((platform::read_i32(this + 0x144) - row[0]) + 1);
        let span = (platform::read_i32(this + 0x144) - platform::read_i32(this + 0x140)) + 1;
        let mut limit = last;
        if remainder == (remainder / span) * span {
            limit = slot_count;
        }
        platform::write_u32(this + 0x128, limit as u32);
    } else {
        platform::write_u32(this + 0x128, platform::read_u32(this + 0xe0));
    }
    platform::write_u32(this + 0x134, platform::read_u32(this + 0x128));

    let token = call0_r(FUN_801B1140);
    if platform::read_u8(this + 0x10f) == 0 {
        let mut element = 0usize;
        let mut claimed = 0i32;
        while claimed < platform::read_i32(this + 0x810) {
            let slot = platform::call1(FUN_8027BCC0, allocator as u32);
            if slot != 0 {
                platform::write_u32(this + element + 0x818, slot);
                element += 0x34;
                claimed += 1;
                continue;
            }

            let mut release = 0i32;
            let mut undo = 0usize;
            while release < claimed {
                platform::call2(
                    FUN_8027BDB0,
                    allocator as u32,
                    platform::read_u32(this + undo + 0x818),
                );
                platform::write_u32(this + undo + 0x818, 0);
                undo += 0x34;
                release += 1;
            }
            platform::call1(FUN_801B1180, token);
            return 0;
        }
        platform::write_u8(this + 0x10f, 1);
    }

    platform::call1(FUN_801B1180, token);
    1
}

// ===========================================================================
// 0x8027EDE0
// ===========================================================================

/// Maps a playback position to a row, an in-row index and a wrap count.
///
/// The block's `+0xE8` divisor must be non-zero. The starting value is
/// `+0x150` normally, the caller's own value when `+0x14C` is `0`, or
/// `+0x150 * +0xCC / 1000` when `+0x14C` is `1` (the 64-bit divide helper
/// `0x802DCEC4`). When the value is below `+0xD8` it is used directly;
/// otherwise `+0xC4` must be set and the value wraps by the `+0xD4` span,
/// reporting the number of wraps through the fourth output.
///
/// # Original
/// - Ghidra: `FUN_8027ede0` at `0x8027EDE0` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ede0_FUN_8027ede0.c`
/// - observed: the `+0xE8 == 0` guard, the `+0x14C == 0` / `== 1` branches,
///   `FUN_802dcec4` with `(value * +0xCC) >> 32` as its first word, the
///   `+0xD8` / `+0xC4` / `+0xD4` wrap arithmetic, `*param_4 = iVar4 + 1`,
///   `*param_2 = piVar5 / +0xE8` and the modulo store into `*param_3`.
///
/// # Arguments
/// - `this`: sound element block.
/// - `out_row`: receives the row.
/// - `out_index`: receives the remainder inside the row.
/// - `out_count`: receives the wrap count.
///
/// # Returns
/// `1` when the position was mapped, `0` when the divisor is zero or the
/// position is out of range.
///
/// # Safety
/// All three output pointers must be writable `i32` slots.
///
/// # Confidence
/// medium - the guards and arithmetic are literal; the low word of the
/// `0x802DCEC4` result and the position units are inferred.
pub unsafe fn fn_8027ede0(
    this: usize,
    out_row: usize,
    out_index: usize,
    out_count: usize,
) -> u32 {
    let divisor = platform::read_i32(this + 0xe8);
    if divisor == 0 {
        return 0;
    }

    let mut value = platform::read_i32(this + 0x150);
    let selector = platform::read_u32(this + 0x14c);
    if selector != 0 {
        value = out_row as i32;
        if selector == 1 {
            let product = (platform::read_i32(this + 0x150) as i64)
                .wrapping_mul(platform::read_i32(this + 0xcc) as i64);
            let quotient = call4_u64(
                FUN_802DCEC4,
                (product >> 32) as u32,
                product as u32,
                0,
                1000,
            );
            value = quotient as u32 as i32;
        }
    }

    platform::write_u32(out_count, 0);
    let boundary = platform::read_i32(this + 0xd8);
    if boundary <= value {
        if platform::read_u8(this + 0xc4) == 0 {
            return 0;
        }
        let span = platform::read_i32(this + 0xd4);
        let difference = boundary - span;
        let wraps = (value - boundary) / difference;
        platform::write_u32(out_count, (wraps + 1) as u32);
        value = span + ((value - boundary) - wraps * difference);
    }

    platform::write_u32(out_row, (value / divisor) as u32);
    let unsigned = value as u32;
    platform::write_u32(out_index, unsigned - (unsigned / divisor as u32) * divisor as u32);
    1
}

// ===========================================================================
// 0x8027F450
// ===========================================================================

/// Constructs the large element player around an embedded element block.
///
/// The base constructor `0x80265FF0` runs first, the vtable `0x8034F218` is
/// installed, and the element block at `+0x110` is constructed by
/// [`fn_8027c7c0`]. The `+0xE28`..`+0xE3C` header is cleared or initialised,
/// and seven `0x10`-byte ramp records starting at `+0xE40` are set to the
/// default level pair.
///
/// # Original
/// - Ghidra: `FUN_8027f450` at `0x8027F450` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f450_FUN_8027f450.c`
/// - observed: `FUN_80265ff0(param_1,param_3,param_4)`, `*param_1 = &DAT_8034f218;`,
///   `FUN_8027c7c0(param_1 + 0x44)`, the `param_1[0x38a..0x38f]` stores, the
///   `puVar2 < param_1 + 0x3ac` loop with
///   `(0xebf - (int)puVar2) >> 4` iterations and the `return param_1;`.
///
/// # Arguments
/// - `this`: player to construct.
/// - `param_2`: word stored at `+0xE2C`.
/// - `param_3`, `param_4`: forwarded to the base constructor.
///
/// # Returns
/// `this`.
///
/// # Safety
/// `this` must be zeroed storage large enough for the embedded block.
///
/// # Confidence
/// high - every store, the loop bound and all three calls are literal.
pub unsafe fn fn_8027f450(
    this: usize,
    param_2: u32,
    param_3: u32,
    param_4: u32,
) -> u32 {
    platform::call3(FUN_80265FF0, this as u32, param_3, param_4);
    platform::write_u32(this, VT_ELEMENT_PLAYER);
    fn_8027c7c0(this + 0x110);

    let default = platform::read_u32(SDA2_RAMP_MIN);
    platform::write_u32(this + 0xe28, 0);
    platform::write_u32(this + 0xe2c, param_2);
    platform::write_u32(this + 0xe30, default);
    platform::write_u32(this + 0xe34, default);
    platform::write_u32(this + 0xe38, 0);
    platform::write_u32(this + 0xe3c, 0);

    let mut slot = this + 0xe40;
    while slot < this + 0xeb0 {
        platform::write_u32(slot, default);
        platform::write_u32(slot + 4, default);
        platform::write_u32(slot + 8, 0);
        platform::write_u32(slot + 0x0c, 0);
        slot += 0x10;
    }
    this as u32
}

// ===========================================================================
// 0x8027F7B0
// ===========================================================================

/// Advances the eight ramp records and applies the current level to each slot.
///
/// For every slot selected by [`fn_8027f020`] the record at
/// `this + 0xE30 + index * 0x10` is interpolated: while the position `+0x0C`
/// is below the length `+0x08`, the level lies between the start `+0x00` and
/// end `+0x04` values; otherwise the end value is used. [`fn_8027ef90`] then
/// receives the interpolated value and the bit mask `1 << index`.
///
/// # Original
/// - Ghidra: `FUN_8027f7b0` at `0x8027F7B0` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f7b0_FUN_8027f7b0.c`
/// - observed: `FUN_80266dc0()`, the eight-iteration
///   `FUN_8027f020(iVar2 + 0x110,iVar4)` scan, the `+0xE3C < +0xE38`
///   interpolation over `+0xE30` / `+0xE34`,
///   `FUN_8027ef90((double)fVar1,iVar2 + 0x110,1 << iVar4)` and the
///   `0x802DCDC0()` return.
/// - Removed: the modelled `_savegpr_27` prologue call (`0x802DCD74`) and the
///   `_restgpr_27` epilogue call (`0x802DCDC0`). `this` is the incoming `r3`,
///   copied to `r27` immediately after the prologue `bl`.
///
/// # Arguments
/// - `this`: large element player from [`fn_8027f450`].
///
/// # Safety
/// `this` must be a live player and its `+0x110` block armed.
///
/// # Confidence
/// medium - the loop, fields and call are literal; the ramp record roles are
/// inferred from the four words it touches.
pub unsafe fn fn_8027f7b0(this: usize) {
    platform::call0(FUN_80266DC0);

    let mut index = 0i32;
    let mut record = this;
    while index < 8 {
        if fn_8027f020(this + 0x110, index) != 0 {
            let position = platform::read_u32(record + 0xe3c);
            let length = platform::read_u32(record + 0xe38);
            let value = if (position as i32) < (length as i32) {
                let start = platform::read_f32(record + 0xe30);
                let end = platform::read_f32(record + 0xe34);
                start + ((position as i32 as f32) * (end - start)) / (length as i32 as f32)
            } else {
                platform::read_f32(record + 0xe34)
            };
            fn_8027ef90(value as f64, this + 0x110, 1 << index);
        }
        index += 1;
        record += 0x10;
    }
}

// ===========================================================================
// 0x8027F890
// ===========================================================================

/// Applies a target level and duration to the ramp records selected by a mask.
///
/// The block mutex is taken through `0x801B1140`. For each of the eight records
/// whose slot pointer is live and whose mask bit is set, the current level is
/// clamped up to the minimum at `0x8042EAF8`, the previous segment is closed by
/// evaluating it at its current position, and the record is re-armed with the
/// new level, duration and a zero position.
///
/// # Original
/// - Ghidra: `FUN_8027f890` at `0x8027F890` (324 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027f890_FUN_8027f890.c`
/// - observed: the `FUN_801b1140` / `FUN_801b1180` lock pair,
///   `FUN_8027f020(block + 0x110,i)`, the `+0xE3C < +0xE38` interpolation, the
///   `+0xE30`/`+0xE34`/`+0xE38`/`+0xE3C` stores and the `0x802DCDB4()` return.
/// - observed: `fmr f29,f1`, `mr r24,r3`, `mr r25,r4`, `mr r26,r5` right after
///   the prologue fix the argument order: `r3` block, `r4` mask, `r5`
///   duration, `f1` level.
/// - Removed: the modelled `_savegpr_24` prologue call (`0x802DCD68`) and the
///   `_restgpr_24` epilogue call (`0x802DCDB4`). The earlier transcription read
///   that prologue `bl` as a value-returning pair (`low = mask, high = block`);
///   it is the register-save stub only, and no value crosses it. `this` is the
///   incoming `r3`.
///
/// # Arguments
/// - `this`: large element player.
/// - `mask`: one bit per ramp record, incoming `r4`.
/// - `level`: new target level from `f1`.
/// - `duration`: new segment length stored at `+0xE38`, incoming `r5`.
///
/// # Safety
/// `this` must be a live player and its `+0x110` block armed.
///
/// # Confidence
/// low - the loop, mask and four stores are literal; the `f1` argument exists
/// only as Ghidra's `extraout_f1`, so its role is inferred.
pub unsafe fn fn_8027f890(this: usize, mask: u32, level: f32, duration: u32) {
    let token = call0_r(FUN_801B1140);
    let minimum = platform::read_f32(SDA2_RAMP_MIN);

    let mut bits = mask;
    let mut index = 0i32;
    let mut record = this;
    while index < 8 && bits != 0 {
        if fn_8027f020(this + 0x110, index) != 0 && bits & 1 != 0 {
            let mut value = level;
            if value < minimum {
                value = minimum;
            }
            let position = platform::read_u32(record + 0xe3c);
            let length = platform::read_u32(record + 0xe38);
            let start = platform::read_f32(record + 0xe30);
            let end = platform::read_f32(record + 0xe34);
            let current = if (position as i32) < (length as i32) {
                start + ((position as i32 as f32) * (end - start)) / (length as i32 as f32)
            } else {
                end
            };
            platform::write_f32(record + 0xe30, current);
            platform::write_f32(record + 0xe34, value);
            platform::write_u32(record + 0xe38, duration);
            platform::write_u32(record + 0xe3c, 0);
        }
        bits >>= 1;
        index += 1;
        record += 0x10;
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027F9E0
// ===========================================================================

/// Tears down the large element player's note record and list link.
///
/// `0x80267190` releases the player's child, links and callbacks. The list
/// owner at `+0xE2C` is then locked at `owner + 0x10`; when `owner + 4` is
/// non-empty the player's `+0xF0` node is unlinked, vtable slot `+0x0C` is
/// notified with `-1`, and `0x8026DB00` drops the owner link before the unlock.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027F9E0 (slot 2 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027F9E0..0x8027FA7C` (156 bytes).
/// - observed: `bl 0x80267190`, `lwz r30,0xE2C(r29)`, `addi r31,r30,0x10`,
///   the `lwz r0,4(r30)` guard, `bl 0x80229190(r30+4,this+0xF0)`, the vtable
///   `+0x0C` call with `r4 = -1`, `bl 0x8026DB00(r30,this)` and
///   `bl 0x801B2F40`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Side effects
/// Releases the note record and unlinks the player from its list.
///
/// # Safety
/// `this` and its `+0xE2C` list owner must be live.
///
/// # Confidence
/// high - every call, guard and offset is literal.
pub unsafe fn fn_8027f9e0(this: usize) {
    platform::call1(FUN_80267190, this as u32);
    detach_from_owner_list(this, 0xe2c);
}

// ===========================================================================
// 0x8027FA80
// ===========================================================================

/// Deleting destructor of the large element player.
///
/// The embedded sound-element block at `+0x110` is destroyed through
/// [`fn_8027c970`] with `-1`, and a positive flag releases the player through
/// `0x802B9280`. The player address is returned unchanged.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FA80 (slot 1 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FA80..0x8027FADC` (92 bytes).
/// - observed: the `cmpwi cr0,r3,0` guard, `addi r3,r3,0x110` / `li r4,-1` /
///   `bl 0x8027C970`, the positive-`r31` guard around `bl 0x802B9280`, and
///   `mr r3,r30` before the epilogue.
///
/// # Arguments
/// - `this`: element player, or null.
/// - `flag`: when positive, free `this`.
///
/// # Returns
/// `this`, unchanged.
///
/// # Safety
/// A non-null `this` must be a live player.
///
/// # Confidence
/// high - the guards and both calls are literal; this is the same deleting
/// destructor shape as [`fn_80272830`].
pub unsafe fn fn_8027fa80(this: usize, flag: i32) -> u32 {
    if this != 0 {
        fn_8027c970(this.wrapping_add(0x110), -1);
        if flag > 0 {
            platform::call1(FUN_802B9280, this as u32);
        }
    }
    this as u32
}

// ===========================================================================
// 0x8027FAE0
// ===========================================================================

/// Re-inserts the large element player into its `+0xE2C` list by priority.
///
/// The priority is the `+0x98` byte plus the `+0x50` word, clamped to
/// `0..=0x7F`, exactly as in [`fn_802728d0`]. The player's `+0xF0` node is
/// unlinked from `owner + 4`, the first node whose priority is greater is
/// found from `owner + 8`, and the node is inserted before it under the
/// `owner + 0x10` lock.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FAE0 (slot 9 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FAE0..0x8027FBC4` (228 bytes).
/// - observed: `lbz r4,0x98(r3)` + `lwz r0,0x50(r3)` with the `cmpwi 0x7F`
///   clamp, `lwz r30,0xE2C(r3)`, `addi r31,r30,0x10`, `bl 0x801B2E60`, the
///   `bl 0x80229190` unlink, the `-0x58`/`-0xA0` priority walk, `bl 0x80229160`
///   insert and `bl 0x801B2F40` unlock.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Side effects
/// Moves the player's `+0xF0` node inside the `+0xE2C` list.
///
/// # Safety
/// `this` and its `+0xE2C` list owner must be live.
///
/// # Confidence
/// high - the offsets and list calls are literal; the shared helper was
/// validated on the sequence-player twin [`fn_802728d0`].
pub unsafe fn fn_8027fae0(this: usize) {
    reinsert_by_priority(this, 0xe2c);
}

// ===========================================================================
// 0x8027FBD0
// ===========================================================================

/// Returns whether the element player has an owner back-pointer at `+0xE28`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FBD0 (slot 4 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FBD0..0x8027FBE0` (16 bytes).
/// - observed: `lwz r3,0xE28(r3)`, `neg r0,r3`, `or r0,r0,r3`,
///   `rlwinm r3,r0,1,31,31`, `blr`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Returns
/// `1` when `+0xE28` is non-zero, else `0`.
///
/// # Safety
/// `this + 0xE28` must be readable.
///
/// # Confidence
/// high - the branchless non-zero test is literal; [`fn_8027fc50`] installs
/// this back-pointer.
pub unsafe fn fn_8027fbd0(this: usize) -> u32 {
    (platform::read_u32(this + 0xe28) != 0) as u32
}

// ===========================================================================
// 0x8027FBF0
// ===========================================================================

/// Detaches the element player's `+0xE28` owner handle.
///
/// The word at `+0xE28` is loaded and tail-branched to [`fn_8027fd50`], which
/// clears the owner's matching back-pointer and then the handle itself.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FBF0 (slot 5 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FBF0..0x8027FBF4` (8 bytes).
/// - observed: `lwz r3,0xE28(r3)` then `b 0x8027FD50`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Safety
/// `this + 0xE28` must hold a live handle or zero.
///
/// # Confidence
/// high - the load and tail branch are literal.
pub unsafe fn fn_8027fbf0(this: usize) {
    fn_8027fd50(platform::read_u32(this + 0xe28) as usize);
}

// ===========================================================================
// 0x8027FC00
// ===========================================================================

/// Returns the address of the embedded sound-element block at `+0x110`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FC00 (slot 7 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FC00..0x8027FC04` (8 bytes).
/// - observed: `addi r3,r3,0x110` and `blr`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Returns
/// `this + 0x110`.
///
/// # Safety
/// `this` must be a valid object base.
///
/// # Confidence
/// high - the addend is literal.
pub unsafe fn fn_8027fc00(this: usize) -> u32 {
    this.wrapping_add(0x110) as u32
}

// ===========================================================================
// 0x8027FC10
// ===========================================================================

/// Returns the address of the embedded sound-element block at `+0x110`.
///
/// This second slot has the same eight-byte body as [`fn_8027fc00`]; both are
/// distinct words in the real `VT_ELEMENT_PLAYER` array.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FC10 (slot 8 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FC10..0x8027FC14` (8 bytes).
/// - observed: `addi r3,r3,0x110` and `blr`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Returns
/// `this + 0x110`.
///
/// # Safety
/// `this` must be a valid object base.
///
/// # Confidence
/// high - the addend is literal.
pub unsafe fn fn_8027fc10(this: usize) -> u32 {
    this.wrapping_add(0x110) as u32
}

// ===========================================================================
// 0x8027FC20
// ===========================================================================

/// Returns the element player's byte at `+0x213`.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FC20 (slot 3 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FC20..0x8027FC24` (8 bytes).
/// - observed: `lbz r3,0x213(r3)` and `blr`.
///
/// # Arguments
/// - `this`: large element player.
///
/// # Returns
/// The byte at `+0x213`, zero-extended.
///
/// # Safety
/// `this + 0x213` must be readable.
///
/// # Confidence
/// high - the byte load is literal; the field is written by the embedded
/// sound-element block constructor [`fn_8027c7c0`].
pub unsafe fn fn_8027fc20(this: usize) -> u32 {
    platform::read_u8(this + 0x213) as u32
}

// ===========================================================================
// 0x8027FC30
// ===========================================================================

/// Returns the head of the element-player marker list.
///
/// # Original (untabled)
/// - Ghidra: none - no row for 0x8027FC30 (slot 0 of the 12-word method array).
/// - Evidence: `work/dol/main.elf` at `0x8027FC30..0x8027FC34` (8 bytes).
/// - observed: `addi r3,r13,-18728` (`r13 - 0x4928`) and `blr`.
///
/// # Arguments
/// None.
///
/// # Returns
/// [`ELEMENT_PLAYER_LIST`] (`0x8042AD58`).
///
/// # Safety
/// The static list header must exist, as it does in the shipped image.
///
/// # Confidence
/// high - the `r13` displacement is exactly `-0x4928`, the marker
/// [`fn_8027fc50`] compares its chain links against.
pub unsafe fn fn_8027fc30() -> u32 {
    ELEMENT_PLAYER_LIST as u32
}

// ===========================================================================
// 0x8027FC50
// ===========================================================================

/// Finds the element player behind an object and attaches it to an output slot.
///
/// The input's first word must be a non-null object whose vtable slot `+0x08`
/// returns a node chain containing the player marker at `r13 - 0x4928`. A
/// matching object is stored at `*out`; its vtable slot `+0x18` is then
/// queried, and a non-zero answer runs slot `+0x1C`. Finally the player's
/// `+0xE28` word points back at `out`.
///
/// # Original
/// - Ghidra: `FUN_8027fc50` at `0x8027FC50` (244 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027fc50_FUN_8027fc50.c`
/// - observed: `*param_1 = 0;`, the `param_2` / `*param_2` null guards, the
///   `(**(code **)(*piVar4 + 8))(piVar4)` chain walk with the
///   `unaff_r13 + -0x4928` marker, the `*param_1 = piVar4` store, the
///   `+0x18` query and `+0x1c` call, and `*(int **)(*param_1 + 0xe28) = param_1;`.
///
/// # Arguments
/// - `out`: receives the player address, and is stored back into the player.
/// - `source`: object whose first word is the candidate.
///
/// # Returns
/// `out`.
///
/// # Safety
/// `out` must be writable; `source`, when non-null, must point at a live word.
///
/// # Confidence
/// medium - the guards, chain walk and slots are literal; the marker at
/// `r13 - 0x4928` is inferred to identify the element player.
pub unsafe fn fn_8027fc50(out: usize, source: usize) -> u32 {
    platform::write_u32(out, 0);

    if source != 0 {
        let mut object = platform::read_u32(source) as usize;
        if object != 0 {
            let mut node = call0_r(platform::read_u32(object + 8) as usize);
            let mut found = false;
            while node != 0 {
                if node as usize == crate::boot::entry::SDA1_BASE - 0x4928 {
                    found = true;
                    break;
                }
                node = platform::read_u32(node as usize);
            }
            if !found {
                object = 0;
            }

            if object != 0 {
                platform::write_u32(out, object as u32);
                let vtable = platform::read_u32(object) as usize;
                if call0_r(platform::read_u32(vtable + 0x18) as usize) != 0 {
                    let target = platform::read_u32(out) as usize;
                    let target_vtable = platform::read_u32(target) as usize;
                    platform::call0(platform::read_u32(target_vtable + 0x1c) as usize);
                }
                platform::write_u32(object + 0xe28, out as u32);
            }
        }
    }

    out as u32
}

// ===========================================================================
// 0x8027FFE0
// ===========================================================================

/// Pops one pending object from three lists, in order.
///
/// The first non-empty list wins: `+0x18` first, then `+0x0C`, then `+0x00`.
/// Each list is read under the `0x801B1140` lock held by the caller for the
/// whole call, and the popped node's `-4` word is returned (the node header is
/// four bytes before the object). A zero result means all three lists were
/// empty.
///
/// # Original
/// - Ghidra: `FUN_8027ffe0` at `0x8027FFE0` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027ffe0_FUN_8027ffe0.c`
/// - observed: the two `FUN_801b1140()` tokens, the three
///   `param_1[6]` / `param_1[3]` / `*param_1` guards, the three
///   `FUN_802290c0` removals, the `local + -4` results and the nested
///   `FUN_801b1180` balance.
///
/// # Arguments
/// - `list`: three-list header.
///
/// # Returns
/// The popped object's `-4` word, or `0` when no list had an entry.
///
/// # Safety
/// `list` must be a live header whose three lists are intact.
///
/// # Confidence
/// medium - the list order, removals and lock balance are literal; the
/// `-4` object offset and the two lock tokens are inferred.
pub unsafe fn fn_8027ffe0(list: usize) -> i32 {
    let outer = call0_r(FUN_801B1140);
    let mut first = call0_r(FUN_801B1140);

    let mut result = 0i32;
    if platform::read_u32(list + 0x18) != 0 {
        let mut node = platform::read_u32(list + 0x1c);
        result = node as i32 - 4;
        platform::call2(
            FUN_802290C0,
            (list + 0x18) as u32,
            (&mut node as *mut u32) as u32,
        );
        platform::call1(FUN_801B1180, first);
    } else {
        platform::call1(FUN_801B1180, first);
    }

    if result == 0 {
        first = call0_r(FUN_801B1140);
        if platform::read_u32(list + 0x0c) != 0 {
            let mut node = platform::read_u32(list + 0x10);
            result = node as i32 - 4;
            platform::call2(
                FUN_802290C0,
                (list + 0x0c) as u32,
                (&mut node as *mut u32) as u32,
            );
            platform::call1(FUN_801B1180, first);
        } else {
            platform::call1(FUN_801B1180, first);
        }

        if result == 0 {
            first = call0_r(FUN_801B1140);
            if platform::read_u32(list) != 0 {
                let mut node = platform::read_u32(list + 4);
                result = node as i32 - 4;
                platform::call2(FUN_802290C0, list as u32, (&mut node as *mut u32) as u32);
                platform::call1(FUN_801B1180, first);
            } else {
                platform::call1(FUN_801B1180, first);
            }
        }
    }

    platform::call1(FUN_801B1180, outer);
    result
}

// ===========================================================================
// Wave 104: the sequence-thread start-up pair, the per-frame element tick, the
// control-object seek/drain pair and the node-mix polar helpers.
//
// Small-data naming: `r2 = 0x80433080`, so `r2 - 0x46xx` is
// `0x8042EAxx`. The words are addressed by their absolute address because the
// export does not reveal their values; every load is reproduced verbatim.
// ===========================================================================

/// `r2 - 0x4680` → `0x8042EA00`.
///
/// `// unknown:` a float divisor used by [`fn_80271e30`], [`fn_802720a0`] and
/// [`fn_80274b90`]; its value is not recoverable from the export.
const SDA2_EA00: usize = 0x8042_EA00;

/// `r2 - 0x4644` → `0x8042EA3C`: the node-mix "one" word.
const SDA2_EA3C: usize = 0x8042_EA3C;

/// `r2 - 0x4640` → `0x8042EA40`: the node-mix zero/default word.
const SDA2_EA40: usize = 0x8042_EA40;

/// `r2 - 0x4620` → `0x8042EA60`: the low edge of the piecewise fade in
/// [`fn_80274a10`] and [`fn_802746d0`].
const SDA2_EA60: usize = 0x8042_EA60;

/// `r2 - 0x461C` → `0x8042EA64`: fade endpoint word.
const SDA2_EA64: usize = 0x8042_EA64;

/// `r2 - 0x4618` → `0x8042EA68`: fade endpoint word.
const SDA2_EA68: usize = 0x8042_EA68;

/// `r2 - 0x4614` → `0x8042EA6C`: fade endpoint word.
const SDA2_EA6C: usize = 0x8042_EA6C;

/// `r2 - 0x4610` → `0x8042EA70`: fade endpoint word.
const SDA2_EA70: usize = 0x8042_EA70;

/// `r2 - 0x460C` → `0x8042EA74`: fade endpoint word.
const SDA2_EA74: usize = 0x8042_EA74;

/// `r2 - 0x4608` → `0x8042EA78`: fade endpoint word.
const SDA2_EA78: usize = 0x8042_EA78;

/// `r2 - 0x4604` → `0x8042EA7C`: fade endpoint word.
const SDA2_EA7C: usize = 0x8042_EA7C;

/// `0x801C14C0` - length of a three-float vector (`runtime::dvd`).
const FUN_801C14C0: usize = 0x801C_14C0;

/// `0x802E4C04` - `atan2(y, x)` in `f1`/`f2`.
const FUN_802E4C04: usize = 0x802E_4C04;

/// `0x802E7FBC` - `cos` for `f64` (`engine::main_loop::fn_802e7fbc`).
const FN_COS_F64: usize = 0x802E_7FBC;

/// `0x80271BB0` - advances every live track and reports its state; the
/// translated body is `void`, so the seek/drain pair calls the address.
const FUN_80271BB0: usize = 0x8027_1BB0;

/// `0x80281950` - writes one three-half-word value to every voice of a slot.
const FUN_80281950: usize = 0x8028_1950;

/// `0x80281480` - returns the voice state word of a slot, or `1`.
const FUN_80281480: usize = 0x8028_1480;

/// `0x8026DAA0` - pops one node from the free list at its argument.
const FUN_8026DAA0: usize = 0x8026_DAA0;

/// `0x802802D0` - unlinks an element record from the small-element registry.
const FUN_802802D0: usize = 0x8028_02D0;

/// `0x80281390` - releases the eight voice slots of one element block.
const FUN_80281390: usize = 0x8028_1390;

/// `0x80260950` / `0x801DB840` / `0x801DB8E0` / `0x801DD450` /
/// `0x80260FA0` - the sound-driver bring-up sequence used by
/// [`fn_8027b360`].
const FUN_80260950: usize = 0x8026_0950;
const FUN_801DB840: usize = 0x801D_B840;
const FUN_801DB8E0: usize = 0x801D_B8E0;
const FUN_801DD450: usize = 0x801D_D450;
const FUN_80260FA0: usize = 0x8026_0FA0;

/// `0x801AB1C0` - attaches the audio DMA buffer to the sound driver.
const FUN_801AB1C0: usize = 0x801A_B1C0;

/// `0x80270B80` - second half of the track-table initialisation.
const FUN_80270B80: usize = 0x8027_0B80;

/// `0x80280530` - installs the four-word DMA stream descriptor.
const FUN_80280530: usize = 0x8028_0530;

/// `r13 - 0x5C18` = `0x80429A68`: the audio DMA buffer word.
const SDA1_AUDIO_DMA_BUFFER: usize = crate::boot::entry::SDA1_BASE - 0x5C18;

/// `r13 - 0x493C` = `0x8042AD44`: the sequence tick captured at start-up.
const SDA1_SEQ_START_TICK: usize = crate::boot::entry::SDA1_BASE - 0x493C;

/// `0x800000F8` - the PowerPC time-base/decrementer bus clock.
const BUS_CLOCK: usize = 0x8000_00F8;

/// `0x803FD2D0` - the DMA stream descriptor passed to `0x80280530`.
const DAT_803FD2D0: usize = 0x803F_D2D0;

/// `0x8034F1E8` - vtable installed on an element record by [`fn_8027eaf0`].
const VT_ELEMENT_RECORD: u32 = 0x8034_F1E8;

/// Converts a 32-bit integer with the PowerPC `0x4330_0000` idiom.
///
/// # Safety
/// `SDA2_CONVERSION_BIAS` must be readable.
#[inline]
unsafe fn int_to_f64(value: u32) -> f64 {
    let biased = 0x4330_0000_0000_0000u64 | ((value ^ 0x8000_0000) as u64);
    f64::from_bits(biased) - read_f64(SDA2_CONVERSION_BIAS)
}

/// Calls a two-double routine (`f1`/`f2`) and keeps the `f1` result.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f64_f64(addr: usize, first: f64, second: f64) -> f64 {
    let f: extern "C" fn(f64, f64) -> f64 = core::mem::transmute(addr);
    f(first, second)
}

/// Calls a one-double routine (`f1`) and keeps the `f1` result.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f64(addr: usize, value: f64) -> f64 {
    let f: extern "C" fn(f64) -> f64 = core::mem::transmute(addr);
    f(value)
}

/// Calls a pointer-taking routine that returns a `double` in `f1`.
///
/// # Safety
/// `addr` must point at a function with this ABI and `ptr` must be valid for
/// whatever the callee reads.
#[inline]
unsafe fn call_ptr_f64(addr: usize, ptr: u32) -> f64 {
    let f: extern "C" fn(u32) -> f64 = core::mem::transmute(addr);
    f(ptr)
}

// ===========================================================================
// 0x80277050
// ===========================================================================

/// Initialises an archive-loader block and returns it.
///
/// The block keeps the archive handle at `+0x18`; the stream pointer at `+0x21C`
/// starts null. The mutex that [`fn_802770d0`] locks at `+0x54` is initialised
/// here, which is the only unprinted argument in the export.
///
/// # Original
/// - Ghidra: `FUN_80277050` at `0x80277050` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80277050_FUN_80277050.c`
/// - observed: `*(undefined4 *)(param_1 + 0x18) = param_2;`,
///   `*(undefined4 *)(param_1 + 0x21c) = 0;`, `FUN_801b2e20();` and
///   `return param_1;`.
/// - `// inferred:` the `0x801B2E20` call initialises the mutex at `+0x54`,
///   the word every loader path locks before touching the block.
///
/// # Arguments
/// - `this`: loader block; at least `0x220` bytes.
/// - `handle`: archive handle stored at `+0x18`.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// - writes `+0x18` and `+0x21C`, initialises the mutex at `+0x54`.
///
/// # Safety
/// `this` must point at `0x220` writable bytes.
///
/// # Confidence
/// medium - the two stores are literal; the mutex argument is inferred from the
/// block's own lock offset.
pub unsafe fn fn_80277050(this: usize, handle: u32) -> usize {
    platform::write_u32(this + 0x18, handle);
    platform::write_u32(this + 0x21c, 0);
    platform::call1(MUTEX_INIT, (this + 0x54) as u32);
    this
}

// ===========================================================================
// 0x80274B90
// ===========================================================================

/// Converts one three-float direction into an angle and a radius ratio.
///
/// The routine works on a local `[x, y, z]` triple seeded with the small-data
/// zero at `0x8042EA40`. When the caller's `length` differs from that seed, the
/// triple is built from `source` (x, z) with the seed as y; if the vector is
/// longer than `max_length` it is scaled down. The `x`/`z` components are then
/// normalised by `length`, `atan2(x, -z)` is written to `angle_out`, and the
/// vector magnitude divided by `max_length` is written to `ratio_out`.
///
/// # Original
/// - Ghidra: `FUN_80274b90` at `0x80274B90` (284 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80274b90_FUN_80274b90.c`
/// - observed: `local_3c = *(float *)(unaff_r2 + -0x4640);`, the
///   `if ((double)local_3c != param_1)` guard, the two `FUN_801c14c0` calls,
///   `thunk_FUN_802e4c04((double)local_3c,-(double)local_34)`,
///   `*param_4 = (float)dVar2;` and `*param_5 = (float)(dVar2 / param_2);`.
/// - `// inferred:` `0x801C14C0` is the three-float length helper exported by
///   `runtime::dvd`; `0x802E4C04` is `atan2`.
///
/// # Arguments
/// - `length`: normalising length; the seed value means "no direction".
/// - `max_length`: clamp applied to the source vector.
/// - `source`: three readable floats `[x, y, z]`; only x and z are used.
/// - `angle_out`: receives the angle in radians.
/// - `ratio_out`: receives the clamped ratio.
///
/// # Safety
/// `source` must address three floats; both out pointers must be writable.
///
/// # Confidence
/// medium - every branch and store is literal; the callee identities are
/// inferred from their exports.
pub unsafe fn fn_80274b90(
    length: f64,
    max_length: f64,
    source: usize,
    angle_out: usize,
    ratio_out: usize,
) {
    let seed = platform::read_f32(SDA2_EA40);
    let x0 = platform::read_f32(source);
    let z0 = platform::read_f32(source + 8);

    let mut x = seed;
    let mut y = seed;
    let mut z = seed;

    if seed as f64 != length {
        let mut px = x0;
        let mut pz = z0;

        let mut triple = [px, seed, pz];
        let mut magnitude = call_ptr_f64(FUN_801C14C0, triple.as_mut_ptr() as u32);
        if max_length < magnitude {
            let scale = (max_length / magnitude) as f32;
            px *= scale;
            pz *= scale;
        }

        triple = [px, seed, pz];
        magnitude = call_ptr_f64(FUN_801C14C0, triple.as_mut_ptr() as u32);

        y = seed;
        x = (((x0 as f64) * magnitude) as f32 as f64 / length) as f32;
        z = (((z0 as f64) * magnitude) as f32 as f64 / length) as f32;
    }

    let angle = call_f64_f64(FUN_802E4C04, x as f64, -(z as f64));
    platform::write_f32(angle_out, angle as f32);

    let triple = [x, y, z];
    let magnitude = call_ptr_f64(FUN_801C14C0, triple.as_ptr() as u32);
    platform::write_f32(ratio_out, (magnitude / max_length) as f32);
}

// ===========================================================================
// 0x80274630
// ===========================================================================

/// Returns one node's distance-ramp level.
///
/// `node + 0x4C` is the start distance and `node + 0x50` its length; the ramp
/// slope is `node + 0x5C` and the ceiling is `node + 0x60`. Distances at or
/// below the start produce the small-data zero, and the linear result is capped
/// at the ceiling.
///
/// # Original
/// - Ghidra: `FUN_80274630` at `0x80274630` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80274630_FUN_80274630.c`
/// - observed: `dVar3 = (double)FUN_801c14c0();`, the
///   `*(float *)(param_2 + 0x4c) < dVar3` guard, the
///   `(dVar3 - +0x4C) / +0x50` product with `+0x5C`, the `+0x60 < fVar2`
///   clamp and `*param_4 = fVar2;`.
/// - `// inferred:` the unprinted `0x801C14C0` argument is `param_3`, the
///   three-float vector passed by [`fn_80274ce0`].
///
/// # Arguments
/// - `_this`: owner object; unused by the body.
/// - `node`: node whose `+0x4C..+0x60` floats parameterise the ramp.
/// - `vector`: three floats measured by `0x801C14C0`.
/// - `out`: receives the ramp level.
///
/// # Safety
/// `node` and `vector` must be readable, `out` writable.
///
/// # Confidence
/// medium - the arithmetic and clamp are literal; the dropped callee argument
/// is inferred from the only pointer the caller supplies.
pub unsafe fn fn_80274630(_this: usize, node: usize, vector: u32, out: usize) {
    let distance = call_ptr_f64(FUN_801C14C0, vector);
    let mut level = platform::read_f32(SDA2_EA40);

    if (platform::read_f32(node + 0x4c) as f64) < distance {
        let t = ((distance - platform::read_f32(node + 0x4c) as f64) as f32
            / platform::read_f32(node + 0x50)) as f64;
        level = (t as f32 * platform::read_f32(node + 0x5c)) as f32;
        if platform::read_f32(node + 0x60) < level {
            level = platform::read_f32(node + 0x60);
        }
    }

    platform::write_f32(out, level);
}

// ===========================================================================
// 0x80274A10
// ===========================================================================

/// Returns one node's distance fade as a piecewise linear curve.
///
/// The routine asks [`fn_80274b90`] for the angle and ratio of `vector`, then
/// selects one of seven linear segments based on where the angle falls between
/// `-width`, `width` and the two small-data endpoints. The selected slope is
/// multiplied by `scale` and the ratio; the second out word receives the
/// small-data default at `0x8042EA40`.
///
/// # Original
/// - Ghidra: `FUN_80274a10` at `0x80274A10` (384 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80274a10_FUN_80274a10.c`
/// - observed: `FUN_80274b90(param_2,param_1,param_5,local_34,&local_38);`, the
///   `dVar3 + param_4`, `-param_4`, `param_4` and `dVar4 - param_4` comparisons
///   with `local_34[0]`, the seven `dVar3 = ...` assignments, and
///   `*param_6 = (float)(dVar3 * param_3) * local_38; *param_7 = uVar1;`.
///
/// # Arguments
/// - `length`, `max_length`: forwarded to [`fn_80274b90`].
/// - `scale`: multiplier applied to the selected slope.
/// - `width`: half-width of the flat segment.
/// - `vector`: three readable floats.
/// - `out_level`: receives the fade level.
/// - `out_default`: receives the `0x8042EA40` word.
///
/// # Safety
/// `vector` must address three floats; both out pointers must be writable.
///
/// # Confidence
/// medium - the branches and stores are literal; the small-data values are read
/// at run time and the curve's physical units are inferred.
pub unsafe fn fn_80274a10(
    length: f64,
    max_length: f64,
    scale: f64,
    width: f64,
    vector: u32,
    out_level: usize,
    out_default: usize,
) {
    let mut angle = 0f32;
    let mut ratio = 0f32;
    fn_80274b90(
        max_length,
        length,
        vector as usize,
        (&mut angle as *mut f32) as u32 as usize,
        (&mut ratio as *mut f32) as u32 as usize,
    );

    let low = platform::read_f32(SDA2_EA60) as f64;
    let mut slope: f64;
    let neg = -width;
    let high = platform::read_f32(SDA2_EA68) as f64;
    let plus = (low + width) as f32 as f64;
    let value = angle as f64;
    let minus = (high - width) as f32 as f64;

    if plus <= value {
        if neg <= value {
            if width <= value {
                if minus <= value {
                    if minus == high {
                        slope = platform::read_f32(SDA2_EA74) as f64;
                    } else {
                        let denom = (minus - high) as f32;
                        let term1 = (value * platform::read_f32(SDA2_EA3C) as f64) as f32 / denom;
                        let term2 = (((minus * platform::read_f32(SDA2_EA40) as f64) as f32 as f64)
                            - high) as f32
                            / denom;
                        slope = (term1 + term2) as f64;
                    }
                } else {
                    slope = platform::read_f32(SDA2_EA3C) as f64;
                }
            } else if neg == width {
                slope = platform::read_f32(SDA2_EA40) as f64;
            } else {
                let denom = (neg - width) as f32;
                let term1 = (value * platform::read_f32(SDA2_EA7C) as f64) as f32 / denom;
                let term2 = ((neg * platform::read_f32(SDA2_EA3C) as f64) as f32
                    - (width * platform::read_f32(SDA2_EA70) as f64) as f32)
                    / denom;
                slope = (term1 + term2) as f64;
            }
        } else {
            slope = platform::read_f32(SDA2_EA70) as f64;
        }
    } else if low == plus {
        slope = platform::read_f32(SDA2_EA64) as f64;
    } else {
        let denom = (low - plus) as f32;
        let term1 = (value * platform::read_f32(SDA2_EA3C) as f64) as f32 / denom;
        let term2 = (high - ((plus * platform::read_f32(SDA2_EA40) as f64) as f32 as f64)) as f32
            / denom;
        slope = (term1 + term2) as f64;
    }

    platform::write_f32(out_level, (slope * scale) as f32 * ratio);
    platform::write_u32(out_default, platform::read_u32(SDA2_EA40));
}

// ===========================================================================
// 0x802746D0
// ===========================================================================

/// Returns a two-word distance fade for one node.
///
/// The routine asks [`fn_80274b90`] for the angle and ratio of `vector`, then
/// selects one of ten linear segments (the same shape as [`fn_80274a10`] but
/// with an extra split at `0x8042EA78`). The second word mixes the segment
/// slope with `param_6` and the cosines of `param_4`/`param_5`.
///
/// # Original
/// - Ghidra: `FUN_802746d0` at `0x802746D0` (828 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802746d0_FUN_802746d0.c`
/// - observed: `FUN_80274b90(param_2,param_1,param_7,local_84,&local_88);`, the
///   `-param_5` / `-param_4` / `param_4` / `param_5` comparison cascade, the
///   ten `dVar3` assignments, the three `FUN_802e7fbc` calls (the SDK `cos`)
///   and `*param_8 = (float)(dVar7 * param_3) * local_88;` followed by the
///   `*param_9 = ...` mix.
///
/// # Arguments
/// - `length`, `max_length`: forwarded to [`fn_80274b90`].
/// - `scale`: slope multiplier.
/// - `low`, `high`: the two fade edges.
/// - `offset`: added to the second output.
/// - `vector`: three readable floats.
/// - `out_level`: receives the primary fade level.
/// - `out_second`: receives the mixed second word.
///
/// # Safety
/// `vector` must address three floats; both out pointers must be writable.
///
/// # Confidence
/// medium - every branch is literal; the small-data values and the fade's
/// physical units are inferred.
pub unsafe fn fn_802746d0(
    length: f64,
    max_length: f64,
    scale: f64,
    low: f64,
    high: f64,
    offset: f64,
    vector: u32,
    out_level: usize,
    out_second: usize,
) {
    let mut angle = 0f32;
    let mut ratio = 0f32;
    fn_80274b90(
        max_length,
        length,
        vector as usize,
        (&mut angle as *mut f32) as u32 as usize,
        (&mut ratio as *mut f32) as u32 as usize,
    );

    let value = angle as f64;
    let mut slope: f64;
    let mut other: f64;

    let neg_high = -high;
    let neg_low = -low;

    if neg_high <= value {
        let edge = platform::read_f32(SDA2_EA6C) as f64;
        if edge <= value {
            if neg_low <= value {
                if low <= value {
                    let split = platform::read_f32(SDA2_EA78) as f64;
                    if split <= value {
                        if high <= value {
                            let end = platform::read_f32(SDA2_EA68) as f64;
                            if high == end {
                                slope = platform::read_f32(SDA2_EA74) as f64;
                            } else {
                                let denom = (high - end) as f32;
                                let term1 =
                                    (value * platform::read_f32(SDA2_EA3C) as f64) as f32 / denom;
                                let term2 = ((high * platform::read_f32(SDA2_EA40) as f64) as f32
                                    as f64
                                    - end) as f32
                                    / denom;
                                slope = (term1 + term2) as f64;
                            }
                            other = platform::read_f32(SDA2_EA3C) as f64;
                        } else {
                            slope = platform::read_f32(SDA2_EA3C) as f64;
                            if split == high {
                                other = platform::read_f32(SDA2_EA74) as f64;
                            } else {
                                let denom = (split - high) as f32;
                                let term1 =
                                    (value * platform::read_f32(SDA2_EA70) as f64) as f32 / denom;
                                let product = ((high
                                    * platform::read_f32(SDA2_EA40) as f64)
                                    as f32) as f64;
                                let term2 = ((split - product) as f32) / denom;
                                other = (term1 + term2) as f64;
                            }
                        }
                    } else {
                        slope = platform::read_f32(SDA2_EA3C) as f64;
                        if low == split {
                            other = platform::read_f32(SDA2_EA64) as f64;
                        } else {
                            let denom = (low - split) as f32;
                            let term1 =
                                (value * platform::read_f32(SDA2_EA70) as f64) as f32 / denom;
                            let term2 = ((low * platform::read_f32(SDA2_EA40) as f64) as f32 as f64
                                - edge) as f32
                                / denom;
                            other = (term1 + term2) as f64;
                        }
                    }
                } else {
                    if neg_low == low {
                        slope = platform::read_f32(SDA2_EA40) as f64;
                    } else {
                        let denom = (neg_low - low) as f32;
                        let term1 = (value * platform::read_f32(SDA2_EA7C) as f64) as f32 / denom;
                        let term2 = ((neg_low * platform::read_f32(SDA2_EA3C) as f64) as f32
                            - (low * platform::read_f32(SDA2_EA70) as f64) as f32)
                            / denom;
                        slope = (term1 + term2) as f64;
                    }
                    other = platform::read_f32(SDA2_EA70) as f64;
                }
            } else {
                slope = platform::read_f32(SDA2_EA70) as f64;
                if edge == neg_low {
                    other = platform::read_f32(SDA2_EA64) as f64;
                } else {
                    let denom = (edge - neg_low) as f32;
                    let term1 = (value * platform::read_f32(SDA2_EA3C) as f64) as f32 / denom;
                    let term2 = (platform::read_f32(SDA2_EA78) as f64
                        - (neg_low * platform::read_f32(SDA2_EA40) as f64) as f32 as f64)
                        as f32
                        / denom;
                    other = (term1 + term2) as f64;
                }
            }
        } else {
            slope = platform::read_f32(SDA2_EA70) as f64;
            if neg_high == edge {
                other = platform::read_f32(SDA2_EA74) as f64;
            } else {
                let denom = (neg_high - edge) as f32;
                let term1 = (value * platform::read_f32(SDA2_EA3C) as f64) as f32 / denom;
                let term2 = ((neg_high * platform::read_f32(SDA2_EA40) as f64) as f32 as f64
                    - edge) as f32
                    / denom;
                other = (term1 + term2) as f64;
            }
        }
    } else {
        if platform::read_f32(SDA2_EA60) as f64 == neg_high {
            slope = platform::read_f32(SDA2_EA64) as f64;
        } else {
            let denom = (platform::read_f32(SDA2_EA60) as f64 - neg_high) as f32;
            let term1 = (value * platform::read_f32(SDA2_EA3C) as f64) as f32 / denom;
            let term2 = (platform::read_f32(SDA2_EA68) as f64
                - (neg_high * platform::read_f32(SDA2_EA40) as f64) as f32 as f64) as f32
                / denom;
            slope = (term1 + term2) as f64;
        }
        other = platform::read_f32(SDA2_EA3C) as f64;
    }

    let cos_high = call_f64(FN_COS_F64, high as f32 as f64) as f32 as f64;
    let cos_low = call_f64(FN_COS_F64, low as f32 as f64) as f32 as f64;
    let mixed = ((cos_high as f32 + cos_low as f32) * platform::read_f32(SDA2_EA74)) as f64;
    let cos_high_again = call_f64(FN_COS_F64, high as f32 as f64) as f32 as f64;

    let level = (slope * scale) as f32 * ratio;
    platform::write_f32(out_level, level);

    let blend = platform::read_f32(SDA2_EA3C);
    let negative = (-(cos_high_again as f32)) as f64;
    let denominator = ((mixed + negative) as f32) as f64;
    let weight = (mixed / denominator) as f32;
    let second = offset
        + ((blend + ((other * scale) as f32) * ratio + weight * (blend - ratio)) as f32) as f64;
    platform::write_f32(out_second, second as f32);
}

/// Runs the shared control-object tear-down used by [`fn_80271e30`] and
/// [`fn_802720a0`].
///
/// It takes the sequence mutex, releases the two registrations guarded by
/// `+0xCD`/`+0xCC`, destroys and nulls every live track (one mutex pair per
/// slot), and releases the outer mutex.
///
/// # Safety
/// `this` must be a live sequence-control object.
unsafe fn seq_control_teardown(this: usize) {
    lock_seq_manager();

    if platform::read_u8(this + 0xcd) != 0 {
        let object = if this != 0 { this + 0xc0 } else { this };
        let manager = call0_r(SEQ_MANAGER);
        platform::call2(FUN_8027BA90, manager, object as u32);
        platform::write_u8(this + 0xcd, 0);
    }

    if platform::read_u8(this + 0xcc) != 0 {
        let object = if this != 0 { this + 0xb4 } else { this };
        let owner = call0_r(FUN_80268C60);
        platform::call2(FUN_80229190, owner, object as u32);
        platform::write_u8(this + 0xcc, 0);
    }

    let mut slot = 0usize;
    while slot < 16 {
        lock_seq_manager();

        let track = platform::read_u32(this + 0x100 + slot * 4);
        if track == 0 {
            unlock_seq_manager();
        } else {
            platform::call1(FUN_80273100, track);

            let manager = platform::read_u32(this + 0xf4) as usize;
            let vtable = platform::read_u32(manager) as usize;
            let destroy = platform::read_u32(vtable + 0x10) as usize;
            platform::call2(destroy, manager as u32, track);

            platform::write_u32(this + 0x100 + slot * 4, 0);
            unlock_seq_manager();
        }

        slot += 1;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80271E30
// ===========================================================================

/// Fast-forwards the control object by a number of steps.
///
/// The step length is derived from the control object's `+0xD4` scale and the
/// `+0xEA`/`+0xEC` beat fields. While the accumulated step is below the
/// caller's target, [`fn_80271bb0`] is advanced once, `+0x160` is incremented
/// and the step length is recomputed; when the target is reached the leftover
/// fraction is stored at `+0xD8`. A non-zero track status runs the shared
/// tear-down and returns immediately.
///
/// # Original
/// - Ghidra: `FUN_80271e30` at `0x80271E30` (612 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80271e30_FUN_80271e30.c`
/// - observed: the `dVar11 = *(float *)(r2 - 0x468C)`, `dVar10 = *(float *)(r2
///   - 0x4680)`, `dVar9 = *(double *)(r2 - 0x4688)` and `dVar12 = *(float *)(r2
///   - 0x4690)` loads, the `while (dVar7 < dVar8)` loop with the
///   `FUN_80271bb0(param_1,1)` call, the `+0x160` increment, the two identical
///   step recomputations and `*(float *)(param_1 + 0xd8) = ...`.
///
/// # Arguments
/// - `this`: sequence-control object.
/// - `target`: number of steps to advance.
///
/// # Side effects
/// - advances tracks, increments `+0x160`, writes `+0xD8`,
/// - may release every registration and track through the shared tear-down.
///
/// # Safety
/// `this` must be a live control object.
///
/// # Confidence
/// medium - the loop and arithmetic are literal; the small-data values and the
/// physical meaning of the step are inferred.
pub unsafe fn fn_80271e30(this: usize, target: u32) {
    let base = platform::read_f32(SDA2_DEFAULT_B) as f64;
    let divisor = platform::read_f32(SDA2_EA00) as f64;
    let floor = platform::read_f32(SDA2_DEFAULT_A) as f64;
    let count = (platform::read_u8(this + 0xea) as u32)
        .wrapping_mul(platform::read_u16(this + 0xec) as u32);

    let mut speed = ((platform::read_f32(this + 0xd4) as f64
        * (int_to_f64(count) as f32 as f64))
        / divisor) as f32 as f64;

    if base != speed {
        let mut remaining = int_to_f64(target) as f32 as f64;
        let mut step = (platform::read_f32(this + 0xd8) as f64 / speed) as f32 as f64;

        loop {
            step = (step as f32) as f64;
            if !(step < remaining) {
                break;
            }
            remaining = (remaining - step) as f32 as f64;

            if platform::call2(FUN_80271BB0, this as u32, 1) != 0 {
                seq_control_teardown(this);
                return;
            }

            let counter = platform::read_u32(this + 0x160);
            platform::write_u32(this + 0x160, counter.wrapping_add(1));

            speed = ((platform::read_f32(this + 0xd4) as f64
                * (int_to_f64(count) as f32 as f64))
                / divisor) as f32 as f64;
            if base == speed {
                return;
            }
            step = floor / speed;
        }

        let leftover = ((step - remaining) as f32) as f64 * speed;
        platform::write_f32(this + 0xd8, leftover as f32);
    }
}

// ===========================================================================
// 0x802720A0
// ===========================================================================

/// Drains the control object until its play clock is exhausted.
///
/// Every live track is first degraded to `0x7F` and released. The routine then
/// decrements the play clock (or the integer counter at `+0xDC`) once per
/// iteration, advancing [`fn_80271bb0`] between steps, for at most `0x300`
/// iterations. A cleared clock or a track status ends the drain, runs the
/// shared tear-down and releases the tracks.
///
/// # Original
/// - Ghidra: `FUN_802720a0` at `0x802720A0` (660 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802720a0_FUN_802720a0.c`
/// - observed: the `FUN_80273430(track,0x7f)` / `FUN_80273ae0(track)` pairs, the
///   `0x2ff < iVar3` limit, the `+0xDC` decrement, the `FUN_80271bb0(param_1,0)`
///   call, the `+0x160` increment, and the same registration/track tear-down as
///   [`fn_80271e30`].
///
/// # Arguments
/// - `this`: sequence-control object.
///
/// # Side effects
/// - releases every track and walks the remaining ones down,
/// - takes/releases the sequence mutex.
///
/// # Safety
/// `this` must be a live control object.
///
/// # Confidence
/// medium - the loop, the limit and the teardown are literal; the small-data
/// clock scale is read at run time.
pub unsafe fn fn_802720a0(this: usize) {
    let mut slot = 0usize;
    while slot < 16 {
        let track = platform::read_u32(this + 0x100 + slot * 4);
        if track != 0 {
            fn_80273430(track as usize, 0x7f);
            fn_80273ae0(track as usize);
        }
        slot += 1;
    }

    let divisor = platform::read_f32(SDA2_EA00) as f64;
    let limit = platform::read_f32(SDA2_DEFAULT_A) as f64;
    let mut index = 0i32;

    loop {
        let count = (platform::read_u8(this + 0xea) as u32)
            .wrapping_mul(platform::read_u16(this + 0xec) as u32);
        let inner = ((platform::read_f32(this + 0xd4) as f64
            * (int_to_f64(count) as f32 as f64))
            / divisor) as f32;

        if platform::read_u32(this + 0xdc) == 0
            && ((platform::read_f32(this + 0xe0) * inner) as f64) < limit
        {
            platform::write_u32(this + 0xe0, platform::read_u32(SDA2_DEFAULT_B));
            return;
        }

        if 0x2ff < index {
            return;
        }

        if platform::read_u32(this + 0xdc) == 0 {
            let decrement = (limit / inner as f64) as f32;
            let clock = platform::read_f32(this + 0xe0);
            platform::write_f32(this + 0xe0, clock - decrement);
        } else {
            let counter = platform::read_u32(this + 0xdc);
            platform::write_u32(this + 0xdc, counter.wrapping_sub(1));
        }

        if platform::call2(FUN_80271BB0, this as u32, 0) != 0 {
            seq_control_teardown(this);
            return;
        }

        index += 1;
        let counter = platform::read_u32(this + 0x160);
        platform::write_u32(this + 0x160, counter.wrapping_add(1));
    }
}

// ===========================================================================
// 0x8027E6A0
// ===========================================================================

/// Advances one frame of the sound-element block.
///
/// The routine increments the row counter at `+0x13C`; when it passes the end
/// word at `+0x144` and looping is enabled (`+0xC4`), it wraps to `+0x140`,
/// bumps `+0x110` and pushes one step into every voice. The column counter at
/// `+0x138` advances separately and wraps through `+0x128`. On the last column
/// of a pending restart (`+0x10B == 0`, `+0x10C != 0`) it re-sends the three
/// half-word position of each voice with `0x80281950` and restarts the slot.
///
/// # Original
/// - Ghidra: `FUN_8027e6a0` at `0x8027E6A0` (508 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027e6a0_FUN_8027e6a0.c`
/// - Removed: the `_restgpr_25` epilogue call (`0x802DCDB8`); the matching
///   `_savegpr_25` prologue (`0x802DCD6C`) was never modelled and `this` is
///   the incoming `r3`.
/// - observed: the two counter increments and the `+0x144 < iVar5` /
///   `+0x134 <= iVar5` wraparounds, the three `FUN_8027e590` calls, the
///   `+0x10b`/`+0x10c` guard, the element loop with `FUN_80281480(iVar6) == 3`,
///   `FUN_80281950(iVar6,iVar2,&local_28)`, `FUN_80281c00(iVar6,1)` and the
///   final `FUN_8027e8a0(iVar1,*(int *)(iVar1 + 0x138) + 1)`.
/// - `// inferred:` `this` is the incoming `r3`, as in the neighbouring
///   element routines.
///
/// # Arguments
/// - `this`: sound-element block (incoming `r3`).
///
/// # Side effects
/// - advances the play counters, writes `+0x110`, `+0x13C`, `+0x134`, `+0x138`,
/// - pushes steps into voices and can restart every slot.
///
/// # Safety
/// `this` must be a live element block.
///
/// # Confidence
/// medium - the counters, guards and calls are literal; the field roles are
/// inferred from the neighbouring element routines.
pub unsafe fn fn_8027e6a0(this: usize) {
    let mut row = platform::read_i32(this + 0x13c).wrapping_add(1);
    platform::write_u32(this + 0x13c, row as u32);
    if platform::read_i32(this + 0x144) < row && platform::read_u8(this + 0xc4) != 0 {
        platform::write_u32(this + 0x13c, platform::read_u32(this + 0x140));
        let played = platform::read_i32(this + 0x110);
        if played < 0x7fff_ffff {
            platform::write_u32(this + 0x110, played.wrapping_add(1) as u32);
        }
        let value = platform::read_i32(this + 0x134).wrapping_mul(platform::read_i32(this + 0xe8));
        fn_8027e590(this, 0, value as u32);
    }

    let column = platform::read_i32(this + 0x138).wrapping_add(1);
    platform::write_u32(this + 0x138, column as u32);
    if platform::read_i32(this + 0x134) <= column {
        platform::write_u32(this + 0x138, 0);
        let length = platform::read_i32(this + 0x128);
        platform::write_u32(this + 0x134, length as u32);
        let value = length.wrapping_mul(platform::read_i32(this + 0xe8));
        fn_8027e590(this, 0, value as u32);
    }

    if platform::read_i32(this + 0x138) == platform::read_i32(this + 0x134).wrapping_sub(1) {
        if platform::read_u8(this + 0x10b) == 0 && platform::read_u8(this + 0x10c) != 0 {
            let count = platform::read_i32(this + 0x80c);
            let mut index = 0i32;
            while index < count {
                let element = this + index as usize * 0x38;
                let slot_word = platform::read_u32(element + 0xb5c);
                if platform::read_u8(element + 0xb58) != 0
                    && slot_word != 0
                    && platform::call1(FUN_80281480, slot_word) == 3
                {
                    let token = call0_r(FUN_801B1140);
                    let voices = platform::read_i32(element + 0xb64);
                    let mut voice = 0i32;
                    while voice < voices {
                        let block = if voice < 2 {
                            let slot = platform::read_u8(element + 0xb68 + voice as usize) as usize;
                            if slot < 0x10 {
                                this + slot * 0x34 + 0x818
                            } else {
                                0
                            }
                        } else {
                            0
                        };

                        let mut key = [
                            platform::read_u16(block + 0x32),
                            0u16,
                            0u16,
                        ];
                        platform::call3(
                            FUN_80281950,
                            slot_word,
                            voice as u32,
                            key.as_mut_ptr() as u32,
                        );
                        voice += 1;
                    }
                    platform::call2(FUN_80281C00, slot_word, 1);
                    platform::call1(FUN_801B1180, token);
                }
                index += 1;
            }
        }
        platform::write_u8(this + 0x10c, 0);
        platform::write_u8(this + 0x10b, 0);
    }

    if platform::read_i32(this + 0x13c) == platform::read_i32(this + 0x144).wrapping_sub(1) {
        let next = platform::read_i32(this + 0x138).wrapping_add(1);
        fn_8027e8a0(this, next as u32);
    }
}

// ===========================================================================
// 0x8027E8A0
// ===========================================================================

/// Advances one column of the sound-element block.
///
/// Without looping (`+0xC4 == 0`) the routine simply forwards to
/// [`fn_8027ea00`]. Otherwise it wraps the column, pushes one step into the
/// voices through [`fn_8027e590`], and when the block is in mode `3` restarts
/// every live slot and re-sends each voice's three-half-word position from the
/// slot's `+0x2C` word. The last column sets the pending flag at `+0x10B`.
///
/// # Original
/// - Ghidra: `FUN_8027e8a0` at `0x8027E8A0` (348 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027e8a0_FUN_8027e8a0.c`
/// - Removed: the `_restgpr_25` epilogue call (`0x802DCDB8`); the matching
///   `_savegpr_25` prologue (`0x802DCD6C`) was never modelled and
///   `this`/`step` are the incoming `r3`/`r4`.
/// - observed: the `*(char *)(iVar1 + 0xc4) == '\0'` split with
///   `FUN_8027ea00()`, the `iVar4 + 1` column wrap through `+0x134`, the two
///   `FUN_8027e590` arguments, the `FUN_80281480(iVar6) == 3` element guard,
///   `FUN_80281c00(iVar6,0)`, `FUN_80281950(iVar6,iVar3,iVar5 + 0x2c)` and the
///   `iVar4 == +0x134 - 1` flag.
///
/// # Arguments
/// - `this`: sound-element block (incoming `r3`).
/// - `step`: column index (incoming `r4`).
///
/// # Side effects
/// - advances voices, may restart every live slot, writes `+0x10B`.
///
/// # Safety
/// `this` must be a live element block.
///
/// # Confidence
/// medium - every branch is literal; the register-passed step and the
/// `0x80281950` pointer are inferred from the neighbouring routines.
pub unsafe fn fn_8027e8a0(this: usize, step: u32) {
    if platform::read_u8(this + 0xc4) == 0 {
        fn_8027ea00(this, step);
    } else {
        let mut next = step as i32 + 1;
        let period = platform::read_i32(this + 0x134);
        if period <= next {
            next -= period;
        }

        let token = call0_r(FUN_801B1140);
        let first = next.wrapping_mul(platform::read_i32(this + 0xe8));
        let second = platform::read_i32(this + 0xf0)
            .wrapping_add((step as i32).wrapping_mul(platform::read_i32(this + 0xe8)));
        fn_8027e590(this, first as u32, second as u32);

        if platform::read_i32(this + 0xc0) == 3 {
            let count = platform::read_i32(this + 0x80c);
            let mut index = 0i32;
            while index < count {
                let element = this + index as usize * 0x38;
                let slot_word = platform::read_u32(element + 0xb5c);
                if platform::read_u8(element + 0xb58) != 0
                    && slot_word != 0
                    && platform::call1(FUN_80281480, slot_word) == 3
                {
                    platform::call2(FUN_80281C00, slot_word, 0);
                    let voices = platform::read_i32(element + 0xb64);
                    let mut voice = 0i32;
                    while voice < voices {
                        let block = if voice < 2 {
                            let slot = platform::read_u8(element + 0xb68 + voice as usize) as usize;
                            if slot < 0x10 {
                                this + slot * 0x34 + 0x818
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        platform::call3(
                            FUN_80281950,
                            slot_word,
                            voice as u32,
                            (block + 0x2c) as u32,
                        );
                        voice += 1;
                    }
                }
                index += 1;
            }

            if step as i32 == platform::read_i32(this + 0x134).wrapping_sub(1) {
                platform::write_u8(this + 0x10b, 1);
            }
        }

        platform::call1(FUN_801B1180, token);
    }
}

// ===========================================================================
// 0x8027EAF0
// ===========================================================================

/// Appends one element record to the small-element table.
///
/// A free node is popped from the list at `+0x180` and filled with the block
/// pointer, the `+0x808` word, two position words, the timing words and the
/// state word at `+0x12C`. The record is linked into `+0x174` through its
/// `+0x2C` word and registered with the small-element registry; the block's
/// `+0x130` row and `+0x12C` column counters then advance. When the row limit
/// is reached without looping, `+0x10E` is set and the routine returns.
///
/// # Original
/// - Ghidra: `FUN_8027eaf0` at `0x8027EAF0` (556 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027eaf0_FUN_8027eaf0.c`
/// - Removed: the three `_restgpr_26` epilogue calls (`0x802DCDBC`); the
///   matching `_savegpr_26` prologue (`0x802DCD70`) was never modelled and
///   `this` is the incoming `r3`.
/// - observed: the `FUN_8026daa0(iVar3 + 0x180)`
///   pop, the `&DAT_8034f1e8` vtable store, the eleven record stores, the
///   `FUN_80229160(iVar3 + 0x174,local_28,puVar4 + 0xb)` insert, the
///   `FUN_8027fdd0()` / `FUN_8027ff60` registration, the `+0x130`/`+0x12C`
///   counters and the `+0x128` modulus test.
/// - `// inferred:` the node fields beyond the stores are not read here; the
///   record is `0x34` bytes because the table walks it with that stride.
///
/// # Arguments
/// - `this`: sound-element block.
///
/// # Side effects
/// - pops and initialises one record, inserts it into two lists, advances the
///   row/column counters and may set `+0x10E`.
///
/// # Safety
/// `this` must be a live element block and the free list at `+0x180` must be
/// initialised. The original dereferences a null allocation result, which this
/// body reproduces.
///
/// # Confidence
/// medium - every store, insert and counter is literal; the allocation failure
/// path is preserved as the original writes it.
pub unsafe fn fn_8027eaf0(this: usize) {
    let row = platform::read_i32(this + 0x148).wrapping_add(1);
    platform::write_u32(this + 0x148, row as u32);

    if platform::read_u8(this + 0x10e) != 0 {
        return;
    }

    let row_index = platform::read_i32(this + 0x130);
    let current = if row_index < platform::read_i32(this + 0xe0).wrapping_sub(1) {
        platform::read_i32(this + 0xe4)
    } else {
        platform::read_i32(this + 0xf4)
    };
    let stride = platform::read_i32(this + 0xe4);
    let mut flag = 0u8;
    let inner = platform::read_i32(this + 0xc8);
    let base = platform::read_u16(this + 0xd0) as u32;
    let outer = platform::read_i32(this + 0x810);
    let start = platform::read_i32(this + 0xdc);

    if platform::read_i32(this + 0x12c) == 0 && platform::read_i32(this + 0xc0) == 3 {
        flag = 1;
    }

    let node = platform::call1(FUN_8026DAA0, (this + 0x180) as u32) as usize;
    if node != 0 {
        platform::write_u32(node + 4, 0);
        platform::write_u32(node + 8, 0);
        platform::write_u8(node + 0x0c, 0);
        platform::write_u32(node, VT_ELEMENT_RECORD);
        platform::write_u32(node + 0x10, 0);
        platform::write_u32(node + 0x14, 0);
        platform::write_u32(node + 0x18, 0);
        platform::write_u32(node + 0x1c, 0);
        platform::write_u32(node + 0x20, 0);
        platform::write_u32(node + 0x24, 0xffff_ffff);
        platform::write_u8(node + 0x28, 0);
        platform::write_u32(node + 0x2c, 0);
        platform::write_u32(node + 0x30, 0);
    }

    platform::write_u32(node + 0x10, this as u32);
    platform::write_u32(node + 0x14, platform::read_u32(this + 0x808));
    platform::write_u32(
        node + 0x18,
        base.wrapping_add((current as u32).wrapping_mul(outer as u32)),
    );
    platform::write_u32(
        node + 0x1c,
        (start as u32).wrapping_add(
            (row_index as u32).wrapping_mul(
                base.wrapping_add((stride as u32).wrapping_mul(inner as u32)),
            ),
        ),
    );
    platform::write_u32(node + 0x20, current as u32);
    platform::write_u32(node + 0x24, platform::read_u32(this + 0x12c));
    platform::write_u8(node + 0x28, flag);

    let token = call0_r(FUN_801B1140);
    let mut key = [(this + 0x178) as u32];
    platform::call3(
        FUN_80229160,
        (this + 0x174) as u32,
        key.as_mut_ptr() as u32,
        (node + 0x2c) as u32,
    );

    let mut entry = 1u32;
    if platform::read_u8(this + 0x102) != 0 {
        entry = 2;
    }
    let registry = call0_r(FUN_8027FDD0) as usize;
    fn_8027ff60(registry, node, entry as i32);

    let next_row = platform::read_i32(this + 0x130).wrapping_add(1);
    platform::write_u32(this + 0x130, next_row as u32);
    if platform::read_i32(this + 0x144) < next_row {
        if platform::read_u8(this + 0xc4) == 0 {
            platform::write_u8(this + 0x10e, 1);
            platform::call1(FUN_801B1180, token);
            return;
        }
        platform::write_u32(this + 0x130, platform::read_u32(this + 0x140));
    }

    let next_column = platform::read_i32(this + 0x12c).wrapping_add(1);
    platform::write_u32(this + 0x12c, next_column as u32);
    if platform::read_i32(this + 0x128) <= next_column {
        platform::write_u32(this + 0x12c, 0);
        let candidate = platform::read_i32(this + 0x124).wrapping_add(1);
        let wrap = candidate.wrapping_sub(
            platform::read_i32(this + 0x144)
                .wrapping_sub(platform::read_i32(this + 0x130))
                .wrapping_add(1),
        );
        let period = platform::read_i32(this + 0x144)
            .wrapping_sub(platform::read_i32(this + 0x140))
            .wrapping_add(1);
        let mut length = platform::read_i32(this + 0x124);
        if wrap == (wrap / period).wrapping_mul(period) {
            length = candidate;
        }
        platform::write_u32(this + 0x128, length as u32);
    }

    platform::call1(FUN_801B1180, token);
}

// ===========================================================================
// 0x8027B360
// ===========================================================================

/// Starts the sequence subsystem for the first time.
///
/// The one-shot flag at `r13 - 0x4940` gates the body. It brings up the sound
/// driver, selects the initial output mode, initialises the track table, records
/// the start tick and seeds the three clock managers (channel, track and
/// sequence) with the two DMA buffer halves. The four-word descriptor and the
/// DMA base are then handed to the thread starter.
///
/// # Original
/// - Ghidra: `FUN_8027b360` at `0x8027B360` (428 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b360_FUN_8027b360.c`
/// - Removed: the two `_restgpr_26` epilogue calls (`0x802DCDBC`); the matching
///   `_savegpr_26` prologue (`0x802DCD70`) was never modelled and
///   `descriptor`/`dma_base` are the incoming `r3`/`r4`.
/// - observed: the `*(char *)(unaff_r13 + -0x4940) == '\0'` guard, the
///   `FUN_801ab1c0`/`FUN_80260860`/`FUN_80260950`/`FUN_801db840`/
///   `FUN_801db8e0`/`FUN_801dd450` bring-up, the four `FUN_80260fa0` modes,
///   `FUN_80270ad0()`/`FUN_80270b80()`, the three clock-manager pairs,
///   `FUN_80270e60()`, `FUN_80280530(&DAT_803fd2d0,...)` and
///   `FUN_8027b720(uVar3,*puVar1,iVar6,puVar1[1])`.
/// - registers: `r26 = descriptor`, `r27 = dma_base`. At `0x8027B41C`/`0x8027B424`
///   `r30 = r27 + [r26+0xC]` (`first`) and then `r30 += [r26+4]` (`second`);
///   `0x8027B420` mirrors `first` into `r29` and `0x8027B430` mirrors `second`
///   into `r28`. `0x8027B454` passes `r28 = second` to `0x80264C10`. After
///   `0x8027B440` adds the first `0x80264C00` result, `0x8027B464` copies that
///   `second+value` sum back into `r28`, which `0x8027B48C` passes to
///   `0x80282FF0`; `0x8027B474` then adds the `0x80282FE0` result to `r30`, and
///   `0x8027B4B8` passes that `second+value+channel` sum to `0x80267D30`.
///   `r29 = first` is only reused by the final `0x8027B720` call.
/// - `// inferred:` the first incoming word is the four-word descriptor and the
///   second is the DMA base, matching the call in [`fn_8027b290`].
///
/// # Arguments
/// - `descriptor`: four words `[token, size, second, size]`.
/// - `dma_base`: base of the audio DMA buffer.
/// - `_size`: accepted by the caller but not read by the body.
///
/// # Side effects
/// - initialises the sound driver and every clock manager, sets the ready flag,
/// - starts or updates the sequence thread.
///
/// # Safety
/// `descriptor` must be readable for `0x10` bytes; the original image's
/// small-data and driver globals must be mapped.
///
/// # Confidence
/// medium - every call and store is literal; the descriptor/DMA split, the
/// manager/table arguments and the global names are inferred from the call
/// sites.
pub unsafe fn fn_8027b360(descriptor: usize, dma_base: u32, _size: u32) {
    if platform::read_u8(SOUND_SYSTEM_READY) != 0 {
        return;
    }
    platform::write_u8(SOUND_SYSTEM_READY, 1);
    platform::call1(
        FUN_801AB1C0,
        platform::read_u32(SDA1_AUDIO_DMA_BUFFER),
    );
    let manager = call0_r(FUN_80260860);
    platform::call1(FUN_80260950, manager);
    platform::call0(FUN_801DB840);
    loop {
        if call0_r(FUN_801DB8E0) != 1 {
            break;
        }
    }
    let mode = call0_r(FUN_801DD450) as u8;
    let setting = match mode {
        0 => 3u32,
        1 => 0,
        2 => 2,
        _ => 0,
    };
    platform::call2(FUN_80260FA0, call0_r(FUN_80260860), setting);

    let table = fn_80270ad0();
    platform::call1(FUN_80270B80, table);

    let first = dma_base.wrapping_add(platform::read_u32(descriptor + 0x0c));
    let second = first.wrapping_add(platform::read_u32(descriptor + 4));

    let tick = call0_r(FUN_801A3360);
    platform::write_u32(SDA1_SEQ_START_TICK, tick);

    let sequence_start = second.wrapping_add(platform::call2(
        FUN_80264C00,
        call0_r(FUN_80264AE0),
        tick,
    ));
    let sequence_value = platform::call2(FUN_80264C00, call0_r(FUN_80264AE0), tick);
    platform::call3(
        FUN_80264C10,
        call0_r(FUN_80264AE0),
        second,
        sequence_value,
    );

    let channel_start = platform::call2(FUN_80282FE0, call0_r(FUN_80282EF0), tick);
    let channel_value = platform::call2(FUN_80282FE0, call0_r(FUN_80282EF0), tick);
    platform::call3(
        FUN_80282FF0,
        call0_r(FUN_80282EF0),
        sequence_start,
        channel_value,
    );

    platform::call2(FUN_80267D20, call0_r(FUN_80267C40), tick);
    let track_value = platform::call2(FUN_80267D20, call0_r(FUN_80267C40), tick);
    platform::call3(
        FUN_80267D30,
        call0_r(FUN_80267C40),
        sequence_start.wrapping_add(channel_start),
        track_value,
    );

    fn_80270e60();
    platform::call4(
        FUN_80280530,
        DAT_803FD2D0 as u32,
        platform::read_u32(descriptor + 8),
        dma_base,
        platform::read_u32(descriptor + 0x0c),
    );
    fn_8027b720(
        call0_r(SEQ_MANAGER) as usize,
        platform::read_u32(descriptor),
        first as i32,
        platform::read_u32(descriptor + 4) as i32,
    );
}

// ===========================================================================
// 0x8027B520
// ===========================================================================

/// Waits until the sound manager releases its slot word and one tick elapses.
///
/// Only active after [`fn_8027b360`] set the ready byte. The loop polls the
/// word at `manager + 0x50` and returns as soon as it becomes zero; otherwise
/// it keeps spinning while the time base has not advanced by at least
/// `bus_clock / 4`.
///
/// # Original
/// - Ghidra: `FUN_8027b520` at `0x8027B520` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027b520_FUN_8027b520.c`
/// - observed: `if (*(char *)(unaff_r13 + -0x4940) != '\0')`, the two
///   `FUN_801b5fa0` reads, `countLeadingZeros(*(undefined4 *)(iVar3 + 0x50))`
///   and `(uint)(iVar3 - iVar2) / (_DAT_800000f8 >> 2) == 0`.
/// - `// inferred:` `countLeadingZeros(x) >> 5` is non-zero only for `x == 0`,
///   so the body returns once the manager word clears.
///
/// # Side effects
/// - spins on the manager word and the time base.
///
/// # Safety
/// The manager singleton and `0x800000F8` must be mapped.
///
/// # Confidence
/// medium - the loop is literal; the two globals are inferred from the
/// neighbouring start-up code.
pub unsafe fn fn_8027b520() {
    if platform::read_u8(SOUND_SYSTEM_READY) == 0 {
        return;
    }

    let start = call0_r(FUN_801B5FA0);
    loop {
        let manager = call0_r(FUN_80260860) as usize;
        if platform::read_u32(manager + 0x50) == 0 {
            return;
        }
        let now = call0_r(FUN_801B5FA0);
        let per_tick = platform::read_u32(BUS_CLOCK) >> 2;
        if now.wrapping_sub(start) / per_tick != 0 {
            return;
        }
    }
}

// ===========================================================================
// 0x8027D160
// ===========================================================================

/// Tears down every voice, element and slot of one block.
///
/// The eight fixed slots `+0xB58 + 0x38n` are released first; a pending
/// registration at `+0x101` is unlinked from the manager; the two element
/// tables at `+0x174`/`+0x17C` are drained through the registry; the `0x34`-byte
/// slot array at `+0x818` is freed; the `0x38`-byte element records at `+0xB58`
/// are released; and the object at `+0x808` is destroyed through its vtable.
/// The three state bytes at `+0x101..+0x103` are cleared last.
///
/// # Original
/// - Ghidra: `FUN_8027d160` at `0x8027D160` (456 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027d160_FUN_8027d160.c`
/// - Removed: the `_restgpr_27` epilogue call (`0x802DCDC0`); the matching
///   `_savegpr_27` prologue (`0x802DCD74`) was never modelled and `this` is the
///   incoming `r3`.
/// - observed: the eight-iteration `+0xb58`/`+0xb5c`
///   loop with `FUN_80281390()`, the `+0x101` `FUN_8027ba90` branch, the
///   `FUN_8027fdd0` / `FUN_802802d0` pairs, the `+0x810`/`+0x818` loop with
///   `FUN_8027bdb0`, the `+0x80c`/`+0xb58` loop with `FUN_802810a0`, the
///   `+0x808` virtual call and the `+0x102`/`+0x103`/`+0x101` clears.
/// - `// inferred:` the unprinted argument of `0x80281390`/`0x802810A0` is the
///   voice pointer in the record's `+0x04` word, matching the callers
///   `0x80268790` and `0x802830A0`.
///
/// # Arguments
/// - `this`: sound-element block.
///
/// # Side effects
/// - releases every registered voice/element and destroys `+0x808`.
///
/// # Safety
/// `this` must be a live block and its two tables must be consistent.
///
/// # Confidence
/// medium - the loops, offsets and printed calls are literal; the dropped
/// arguments are inferred from the loop variables.
pub unsafe fn fn_8027d160(this: usize) {
    lock_seq_manager();

    let mut index = 0usize;
    while index < 8 {
        let element = this + index * 0x38;
        let voice = platform::read_u32(element + 0xb5c);
        if platform::read_u8(element + 0xb58) != 0 && voice != 0 {
            platform::call1(FUN_80281390, voice);
        }
        index += 1;
    }

    if platform::read_u8(this + 0x101) != 0 {
        let object = if this != 0 { this + 0xb4 } else { this };
        platform::call2(FUN_8027BA90, call0_r(SEQ_MANAGER), object as u32);
    }

    unlock_seq_manager();

    let registry = call0_r(FUN_8027FDD0);
    platform::call2(FUN_802802D0, registry, (this + 0x154) as u32);

    let token = call0_r(FUN_801B1140);
    while platform::read_u32(this + 0x174) != 0 {
        let node = platform::read_u32(this + 0x17c);
        platform::call2(
            FUN_802802D0,
            call0_r(FUN_8027FDD0),
            node.wrapping_sub(0x2c),
        );
    }
    platform::call1(FUN_801B1180, token);

    let mut slot = 0usize;
    let slots = platform::read_i32(this + 0x810);
    while (slot as i32) < slots {
        let entry = this + slot * 0x34;
        let address = platform::read_u32(entry + 0x818);
        if address != 0 {
            fn_8027bdb0(platform::read_u32(this + 0x804) as usize, address);
            platform::write_u32(entry + 0x818, 0);
        }
        slot += 1;
    }

    let token = call0_r(FUN_801B1140);
    let mut element = this + 0xb58;
    let elements = platform::read_i32(this + 0x80c);
    let mut index = 0i32;
    while index < elements {
        let voice = platform::read_u32(element + 4);
        if platform::read_u8(element) != 0 && voice != 0 {
            platform::call1(FUN_802810A0, voice);
            platform::write_u8(element + 4, 0);
            platform::write_u8(element + 5, 0);
            platform::write_u8(element + 6, 0);
            platform::write_u8(element + 7, 0);
        }
        element += 0x38;
        index += 1;
    }
    platform::call1(FUN_801B1180, token);

    lock_seq_manager();
    let owned = platform::read_u32(this + 0x808) as usize;
    if owned != 0 {
        let vtable = platform::read_u32(owned) as usize;
        let destroy = platform::read_u32(vtable + 0x10) as usize;
        platform::call1(destroy, owned as u32);
        platform::write_u32(this + 0x808, 0);
    }
    unlock_seq_manager();

    platform::write_u8(this + 0x102, 0);
    platform::write_u8(this + 0x103, 0);
    platform::write_u8(this + 0x101, 0);
}

// ---------------------------------------------------------------------------
// Wave 109: the last four reconstructible entries of the `0x80270000` bucket -
// the voice start-up that turns a request block into a voice, the two node-mix
// inputs [`fn_80274ce0`] feeds beside [`fn_80274630`], and the per-element
// voice burst.
//
// `0x80273560` and `0x80274480` also belong to this bucket and were left open
// here: both keep live paired-single register state that the export cannot
// express. Wave 112 separates that noise from the program state and lands both;
// see the wave-112 section at the end of this file.
// ---------------------------------------------------------------------------

/// `0x80272340` - allocates one voice from a request block; `r3` is the
/// sequence object, `r4` the bank word and `r5` the request built by
/// [`fn_80273dd0`]. Returns the voice or zero.
const FUN_80272340: usize = 0x8027_2340;

/// `0x801C1160` - transforms a three-float vector into the caller's `0x30`-byte
/// local (the same helper `render::gx_band` calls for its matrices).
const FUN_801C1160: usize = 0x801C_1160;

/// `0x802E861C` - four-byte thunk to the double `pow` at `0x802E55A4`.
const FUN_802E861C: usize = 0x802E_861C;

/// `0x802610B0` - returns the active sequence mode (`0`, `1` or `2`) that
/// selects the fade curve in [`fn_80274320`].
const FUN_802610B0: usize = 0x8026_10B0;

/// `0x802688E0` - writes the key velocity of a voice (`voice`, `value`,
/// `flag`); [`fn_80273dd0`] calls it once per start.
const FUN_802688E0: usize = 0x8026_88E0;

/// `0x802698A0` / `0x802698E0` / `0x80269980` / `0x80269990` / `0x802698C0` -
/// the five band setters [`fn_80273dd0`] runs when the matching control byte is
/// below `0x80` (`+0x90..+0x94`).
const FUN_802698A0: usize = 0x8026_98A0;
const FUN_802698E0: usize = 0x8026_98E0;
const FUN_80269980: usize = 0x8026_9980;
const FUN_802698C0: usize = 0x8026_98C0;

/// `0x80265F60` - returns one voice parameter of an element (`f1` = current
/// value, `r3` = element, `r4` = index).
const FUN_80265F60: usize = 0x8026_5F60;

/// `0x80265FA0` - returns one voice matrix/position word of an element
/// (`r3` = element, `r4` = index).
const FUN_80265FA0: usize = 0x8026_5FA0;

/// `0x802814A0` .. `0x802817D0` - the setters [`fn_8027e080`] applies to an
/// element's voice; each takes the value in `f1` and the voice in `r4`.
const FUN_802814A0: usize = 0x8028_14A0;
const FUN_80281550: usize = 0x8028_1550;
const FUN_802815B0: usize = 0x8028_15B0;
const FUN_802815D0: usize = 0x8028_15D0;
const FUN_802815F0: usize = 0x8028_15F0;
const FUN_80281610: usize = 0x8028_1610;
const FUN_80281680: usize = 0x8028_1680;
const FUN_802816C0: usize = 0x8028_16C0;
const FUN_802816E0: usize = 0x8028_16E0;
const FUN_80281710: usize = 0x8028_1710;
const FUN_80281750: usize = 0x8028_1750;
const FUN_80281790: usize = 0x8028_1790;
const FUN_802817D0: usize = 0x8028_17D0;

/// `0x8042EA10` = `r2 - 0x4670`: the `127.0` divisor of the level conversion in
/// [`fn_80273dd0`].
const SDA2_EA10: usize = 0x8042_EA10;

/// `0x8042EA38` = `r2 - 0x4648`: the `1/256` scale of the blend byte in
/// [`fn_802741e0`].
const SDA2_EA38: usize = 0x8042_EA38;

/// `0x8042EAE8` = `r2 - 0x4598`: the neutral `1.0` level base of the element
/// mixer (the same word as `SDA2_BLOCK_DEFAULT_A`).
const SEQ_MIX_ONE: usize = 0x8042_EAE8;

/// `0x8042EAD0` = `r2 - 0x45B0`: the neutral `0.0` level base.
const SEQ_MIX_ZERO: usize = 0x8042_EAD0;

/// `0x8042EAEC` = `r2 - 0x4594`: `0.0078125`, the divisor of the first blend
/// byte in [`fn_8027e080`].
const SDA2_EAEC: usize = 0x8042_EAEC;

/// `0x8042EAF0` = `r2 - 0x4590`: `63.0`, the divisor of the second level byte.
const SDA2_EAF0: usize = 0x8042_EAF0;

/// `0x802FA788` - the nine-word voice request template [`fn_80273dd0`] copies
/// onto its stack before filling the fields; the image holds nine zero words
/// except word 7, the unlink callback `0x80273B50`.
const SEQ_VOICE_REQUEST_TEMPLATE: usize = 0x802F_A788;

/// Calls a pointer-taking routine that returns a single-precision result in
/// `f1`.
///
/// The export prints an explicit `(double)` cast around [`FUN_801C14C0`], which
/// is how Ghidra renders a `float` return promoted at the call site, so the
/// helper is declared with the `f32` result the callee actually produces.
///
/// # Safety
/// `addr` must point at a function with this ABI and `ptr` must be valid for
/// whatever the callee reads.
#[inline]
unsafe fn call_ptr_f32(addr: usize, ptr: u32) -> f32 {
    let f: extern "C" fn(u32) -> f32 = core::mem::transmute(addr);
    f(ptr)
}

/// Calls a two-`f32` routine (`f1`/`f2`) and keeps the `f1` result.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f32_f32(addr: usize, first: f32, second: f32) -> f32 {
    let f: extern "C" fn(f32, f32) -> f32 = core::mem::transmute(addr);
    f(first, second)
}

/// Calls a routine with one `f32` in `f1` and one integer word in `r4`.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f32_u32(addr: usize, value: f32, word: u32) {
    let f: extern "C" fn(f32, u32) = core::mem::transmute(addr);
    f(value, word);
}

/// Calls a routine with one `f32` in `f1` and two integer words in `r4`/`r5`,
/// keeping the `f1` result.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f32_u32_u32(addr: usize, value: f32, first: u32, second: u32) -> f32 {
    let f: extern "C" fn(f32, u32, u32) -> f32 = core::mem::transmute(addr);
    f(value, first, second)
}

/// Calls a routine taking two integer words and returning its `f32` in `f1`.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_u32_u32_f32(addr: usize, first: u32, second: u32) -> f32 {
    let f: extern "C" fn(u32, u32) -> f32 = core::mem::transmute(addr);
    f(first, second)
}

// ===========================================================================
// 0x80273DD0
// ===========================================================================

/// Starts one voice for a sequence element and returns it.
///
/// The routine takes the sequence-manager mutex, converts the requested level
/// to `level * this[+0x89] / 127` and, when `flag` is set, updates the voice
/// already installed at `this + 0xC8`: the id byte goes to `+0xA8` and the
/// squared `count / 127` ratio to `+0x90`. A second block re-reads the same
/// head when `this + 0x27` is set: a type-4 voice is destroyed through
/// [`FUN_80268790`], anything else gets the same two fields plus `value` at
/// `+0xB0`.
///
/// Without a live voice it fills the nine-word request block that starts at the
/// template `0x802FA788` - `this + 0x54`, `id`, the converted level, `value`
/// (or `-1` when `flag` is zero), the sign-extended `this + 0x8B`,
/// `this[+0xC4][+0xE9] + this[+0x8D]`, `this[+0xC4][+0xE4]`, the template's
/// callback word and `this` - and hands it to [`FUN_80272340`] together with
/// the sequence object and `this + 0x50`. A rejected request unlocks and
/// returns zero.
///
/// An accepted voice retires every node of the `this + 0xC8` list that carries
/// the same `+0xBC` key, is spliced into that list at `+0xD4`, then receives
/// the five guarded band setters, the pitch/key velocity pair and the
/// `+0xA0..+0xA4` band interpolation.
///
/// # Original
/// - Ghidra: `FUN_80273dd0` at `0x80273DD0` (1032 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273dd0_FUN_80273dd0.c`
/// - observed: `bl 0x8027b5f0` + `addi r3,r3,852` + `bl 0x801b2e60`, the
///   `lbz r0,137(r25)` / `mullw` / `0x81020409` divide-by-127 sequence, the
///   `stw r0,168(r30)` / `stfs f0,144(r30)` pair, the `lwzu r11,-22648(r3)`
///   nine-word copy, `bl 0x80272340`, the `188(r30)` / `212(r27)` walk, the
///   five `lbz`/`cmpli 127` setter guards, the two `bl 0x802688e0` calls and
///   the `160(r30)`..`164(r30)` rewrite at the end.
/// - `// inferred:` the request word roles from the nine store offsets and the
///   `0x802FA788` template; `0x802688E0` writes the key velocity.
///
/// # Arguments
/// - `this`: the sequence-control object; `+0x89` the level scale, `+0x27` the
///   refresh flag, `+0xC8` the voice-list head, `+0xC4` the sequence object and
///   `+0x50` the bank word.
/// - `id`: byte stored at `voice + 0xA8` and in request word 1.
/// - `level`: pre-scale level, multiplied by `this + 0x89` and divided by 127.
/// - `value`: stored at `voice + 0xB0` and used as request word 3.
/// - `flag`: when zero, request word 3 becomes `-1` instead of `value`.
///
/// # Returns
/// The started voice, or zero when the request is rejected.
///
/// # Side effects
/// - takes and releases the sequence-manager mutex, allocates a voice, rewires
///   the `+0xC8` list, retires same-key nodes and writes the band, pitch and
///   key-velocity fields.
///
/// # Safety
/// `this` must be a live control object whose `+0xC8` list and `+0xC4`
/// sequence object are valid.
///
/// # Confidence
/// medium - the control flow, every field offset and every store are literal in
/// the export; the request word roles and two callee names are inferred.
pub unsafe fn fn_80273dd0(this: usize, id: u32, level: i32, value: u32, flag: u32) -> usize {
    lock_seq_manager();

    let sequence = platform::read_u32(this + 0xc4) as usize;
    let scaled = level.wrapping_mul(platform::read_u8(this + 0x89) as i32);
    let quotient = scaled / 0x7f + (scaled >> 31);
    let count = quotient - (quotient >> 31);

    let mut voice = 0usize;
    if flag != 0 {
        voice = platform::read_u32(this + 0xc8) as usize;
        if voice != 0 {
            platform::write_u32(voice + 0xa8, id & 0xff);
            let ratio = ((count as f64) / platform::read_f32(SDA2_EA10) as f64) as f32;
            platform::write_f32(voice + 0x90, ratio * ratio);
        }
    }

    if platform::read_u8(this + 0x27) != 0 {
        voice = platform::read_u32(this + 0xc8) as usize;
        if voice != 0 {
            if platform::read_u32(voice) == 4 {
                platform::call1(FUN_80268790, voice as u32);
                voice = 0;
            } else {
                platform::write_u32(voice + 0xa8, id & 0xff);
                let ratio = ((count as f64) / platform::read_f32(SDA2_EA10) as f64) as f32;
                platform::write_f32(voice + 0x90, ratio * ratio);
                platform::write_u32(voice + 0xb0, value);
            }
        }
    }

    if voice == 0 {
        // The static template is nine words; word 7 is the unlink callback and
        // every other word is overwritten below.
        let mut request = [0u32; 9];
        let mut word = 0;
        while word < 9 {
            request[word] = platform::read_u32(SEQ_VOICE_REQUEST_TEMPLATE + word * 4);
            word += 1;
        }
        request[0] = platform::read_u32(this + 0x54);
        request[1] = id;
        request[2] = count as u32;
        request[3] = if flag == 0 { value } else { 0xffff_ffff };
        request[4] = (platform::read_u8(this + 0x8b) as i8 as i32) as u32;
        request[5] = platform::read_u8(sequence + 0xe9) as u32
            + platform::read_u8(this + 0x8d) as u32;
        request[6] = platform::read_u32(sequence + 0xe4);
        request[8] = this as u32;

        voice = platform::call3(
            FUN_80272340,
            sequence as u32,
            platform::read_u32(this + 0x50),
            request.as_mut_ptr() as u32,
        ) as usize;
        if voice == 0 {
            unlock_seq_manager();
            return 0;
        }

        if platform::read_i32(voice + 0xbc) > 0 {
            let key = platform::read_u32(voice + 0xbc);
            let mut node = platform::read_u32(this + 0xc8) as usize;
            while node != 0 {
                if platform::read_u32(node + 0xbc) == key {
                    platform::call1(FUN_802686A0, node as u32);
                }
                node = platform::read_u32(node + 0xd4) as usize;
            }
        }

        lock_seq_manager();
        platform::write_u32(voice + 0xd4, platform::read_u32(this + 0xc8));
        platform::write_u32(this + 0xc8, voice as u32);
        unlock_seq_manager();
    }

    if platform::read_u8(this + 0x90) < 0x80 {
        platform::call2(
            FUN_802698A0,
            voice as u32,
            platform::read_u8(this + 0x90) as u32,
        );
    }
    if platform::read_u8(this + 0x91) < 0x80 {
        platform::call1(FUN_802698E0, voice as u32);
    }
    if platform::read_u8(this + 0x92) < 0x80 {
        platform::call2(
            FUN_80269980,
            voice as u32,
            platform::read_u8(this + 0x92) as u32,
        );
    }
    if platform::read_u8(this + 0x93) < 0x80 {
        platform::call1(FUN_80269990, voice as u32);
    }
    if (platform::read_u16(this + 0x94) as i16) < 0x80 {
        platform::call1(FUN_802698C0, voice as u32);
    }

    let mut pitch = platform::read_f32(this + 0x6c);
    if platform::read_u8(this + 0x4b) != 0 {
        let delta = (platform::read_u8(this + 0x8e) as i32).wrapping_sub(id as i32);
        pitch += delta as f32;
    }

    let bend = platform::read_u8(this + 0x8f);
    if bend == 0 {
        platform::call3(FUN_802688E0, voice as u32, value, 0);
    } else {
        let mut window = pitch;
        if window < platform::read_f32(SDA2_TRACK_DEFAULT_B) {
            window = -window;
        }
        let squared = bend.wrapping_mul(bend) as i32;
        let key = ((squared as f32) * window) as i32;
        platform::call3(FUN_802688E0, voice as u32, ((key >> 5) * 5) as u32, 1);
    }

    platform::write_u8(this + 0x8e, (id & 0xff) as u8);

    let band = platform::read_u8(this + 0x49);
    let from = platform::read_u8(voice + 0xa0) as i32;
    let to = platform::read_u8(voice + 0xa1) as i32;
    let span = platform::read_u16(voice + 0xa2) as i32;
    let step = platform::read_u16(voice + 0xa4) as i32;
    let mixed = if step < span {
        (from + (step * (to - from)) / span) as u8
    } else {
        platform::read_u8(voice + 0xa1)
    };
    platform::write_u8(voice + 0xa0, mixed);
    let mask = (-(band as i32) >> 31) as u8;
    platform::write_u8(voice + 0xa1, !mask);
    platform::write_u16(voice + 0xa2, 0);
    platform::write_u16(voice + 0xa4, 0);
    platform::write_u8(voice + 0x39, platform::read_u8(sequence + 0xcf));
    platform::write_u32(voice + 0xb4, platform::read_u32(sequence + 0x2c));
    platform::write_u32(voice + 0xb8, platform::read_u32(sequence + 0x30));

    unlock_seq_manager();
    voice
}

// ===========================================================================
// 0x802741E0
// ===========================================================================

/// Returns the distance ramp of one node and the scaled track counter.
///
/// The node caches the position it was mixed against in `+0x30..+0x38`, and
/// the caller's `vector` is a three-float position. The difference is measured
/// by [`FUN_801C14C0`] and turned into a level from the node's start distance
/// `+0x4C` and span `+0x50`. The byte at `vector + 0x1C` selects the curve and
/// `vector + 0x1D / 256` is its parameter:
///
/// - `1`: `pow(parameter, (distance - start) / span)`
/// - `2`: `1 - (1 - parameter) * (distance - start) / span`, clamped at zero
/// - anything else: the level stays at one
///
/// The second output negates `(1 - level) * this[+0x1C]`, the per-node track
/// counter the caller folds into its own accumulator.
///
/// # Original
/// - Ghidra: `FUN_802741e0` at `0x802741E0` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802741e0_FUN_802741e0.c`
/// - observed: `psq_l f2,0(r5)` / `psq_l f1,48(r4)` / `ps_sub`, the two
///   single-element `psq_l ...,1,0` loads at `8(r5)` and `56(r4)` feeding the
///   second `ps_sub`, `bl 0x801c14c0`, `fcmpo` against `76(r31)`, `fdivs` by
///   `80(r31)`, `bl 0x802e861c`, `stfs f5,0(r29)` and `stw r0,0(r30)`.
/// - `// inferred:` `0x801C14C0` returns its length in `f1` as a single;
///   `0x802E861C` is the `pow` thunk.
///
/// # Arguments
/// - `this`: the element player; `+0x1C` is the node counter.
/// - `node`: node record; `+0x30..+0x38` the cached position, `+0x4C` the start
///   distance and `+0x50` the span.
/// - `vector`: three floats at `+0x00..+0x0C`, the curve selector at `+0x1C`
///   and the blend byte at `+0x1D`.
/// - `out_level`: receives the ramp level.
/// - `out_count`: receives `-(1 - level) * counter`.
///
/// # Side effects
/// - writes both output words; no other state is touched.
///
/// # Safety
/// `node` and `vector` must be readable and both outputs writable.
///
/// # Confidence
/// medium - every load, compare and store is literal in the export; the vector
/// layout and the two callee return types are inferred from the call sites.
pub unsafe fn fn_802741e0(
    this: usize,
    node: usize,
    vector: usize,
    out_level: usize,
    out_count: usize,
) {
    let triple = [
        platform::read_f32(vector) - platform::read_f32(node + 0x30),
        platform::read_f32(vector + 4) - platform::read_f32(node + 0x34),
        platform::read_f32(vector + 8) - platform::read_f32(node + 0x38),
    ];
    let distance = call_ptr_f32(FUN_801C14C0, triple.as_ptr() as u32);

    let counter = platform::read_i32(this + 0x1c);
    let mode = platform::read_u8(vector + 0x1c);
    let blend = (platform::read_u8(vector + 0x1d) as f32) * platform::read_f32(SDA2_EA38);
    let start = platform::read_f32(node + 0x4c);
    let span = platform::read_f32(node + 0x50);
    let mut level = platform::read_f32(SDA2_EA3C);

    if start < distance {
        let ratio = (distance - start) / span;
        if mode == 1 {
            level = call_f32_f32(FUN_802E861C, blend, ratio);
        } else if mode == 2 {
            level = 1.0 - (1.0 - blend) * ratio;
            if level < platform::read_f32(SDA2_EA40) {
                level = platform::read_f32(SDA2_EA40);
            }
        }
    }

    platform::write_f32(out_level, level);
    let faded = (1.0 - level) * counter as f32;
    platform::write_i32(out_count, -(faded as i32));
}

// ===========================================================================
// 0x80274320
// ===========================================================================

/// Returns one node's matrix fade and, depending on the sequence mode, its
/// distance fade.
///
/// [`FUN_801C1160`] transforms the node's `+0x30` position by `vector` into the
/// local `0x30`-byte block, whose first three floats are measured by
/// [`FUN_801C14C0`]. The `floats` array supplies four caller values: element
/// `[0]` is the width for the seven-segment fade and elements `[1..4]` are the
/// low/high/offset triple of the ten-segment fade. The word returned by
/// [`FUN_802610B0`] selects the route:
///
/// - `1` or `2`: [`fn_802746d0`] with `node[+0x48]`, the length,
///   `this[+0x20]` and `floats[1..4]`
/// - `0`: [`fn_80274a10`] with `node[+0x48]`, the length, `this[+0x20]` and
///   `floats[0]`
/// - anything else: both outputs receive the small-data zero
///
/// # Original
/// - Ghidra: `FUN_80274320` at `0x80274320` (352 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80274320_FUN_80274320.c`
/// - observed: the `stfd`/`psq_st` save of `f25..f31`, `bl 0x801c1160` with
///   `r3 = node`, `r5 = vector` and `r5 = r1+8` before the call, `bl
///   0x801c14c0` on the same local, `lfs` of `32(r27)`, `72(r28)` and
///   `0..12(r29)`, the `addi r0,r3,-1` / `cmpli 1` dispatch, the two calls to
///   `0x802746d0` / `0x80274a10` with `f1..f6`, and the two `stfs` of
///   `-17984(r2)` in the default arm.
/// - `// inferred:` `0x801C1160` fills the local transformed vector;
///   `0x802610B0` reports the sequence mode.
///
/// # Arguments
/// - `this`: the element player; `+0x20` is the fade scale.
/// - `node`: node record; `+0x30` the direction source and `+0x48` the fade
///   length.
/// - `vector`: three readable floats passed to [`FUN_801C1160`].
/// - `floats`: four readable floats (width, low, high, offset).
/// - `out_a`, `out_b`: the two fade outputs.
///
/// # Side effects
/// - writes both outputs and the local transform block.
///
/// # Safety
/// `node`, `vector` and `floats` must be readable, both outputs writable.
///
/// # Confidence
/// medium - the two callee argument lists and the dispatch are literal; the
/// roles of the four incoming floats are read from the segment helpers they
/// feed.
pub unsafe fn fn_80274320(
    this: usize,
    node: usize,
    vector: u32,
    floats: usize,
    out_a: usize,
    out_b: usize,
) {
    let mut local = [0f32; 12];
    platform::call3(FUN_801C1160, node as u32, vector, local.as_mut_ptr() as u32);
    let length = call_ptr_f32(FUN_801C14C0, local.as_ptr() as u32) as f64;

    let first = platform::read_f32(node + 0x48) as f64;
    let scale = platform::read_f32(this + 0x20) as f64;
    let width = platform::read_f32(floats) as f64;
    let low = platform::read_f32(floats + 4) as f64;
    let high = platform::read_f32(floats + 8) as f64;
    let offset = platform::read_f32(floats + 12) as f64;

    platform::call0(FUN_80260860);
    let mode = call0_r(FUN_802610B0);

    if mode.wrapping_sub(1) < 2 {
        fn_802746d0(
            first,
            length,
            scale,
            low,
            high,
            offset,
            local.as_ptr() as u32,
            out_a,
            out_b,
        );
    } else if mode == 0 {
        fn_80274a10(
            first,
            length,
            scale,
            width,
            local.as_ptr() as u32,
            out_a,
            out_b,
        );
    } else {
        let zero = platform::read_u32(SDA2_EA40);
        platform::write_u32(out_a, zero);
        platform::write_u32(out_b, zero);
    }
}

// ===========================================================================
// 0x8027E080
// ===========================================================================

/// Re-sends one element's parameters to its voice.
///
/// `spec + 0` is the enable byte: when zero nothing happens. Otherwise the
/// routine computes the gain from `spec + 8 / 0.0078125` and `spec + 0x30`, the
/// level base from `spec + 0x34` plus the `(spec + 9 - 63) / 63` term (the
/// `- 64` variant once the selector reaches two), and the seven voice values
/// from the element's own `+0x04..+0x28` floats. [`FUN_80265F60`] supplies three
/// running values and [`FUN_80265FA0`] four more; both are stored in local
/// arrays.
///
/// With a voice at `spec + 4` it locks the sound state ([`FUN_801B1140`]),
/// applies the ten value setters, re-sends the three- and four-word arrays and
/// every `0x18`-byte row of the `+0x50` table (count at `obj + 0x814`), then
/// unlocks.
///
/// # Original
/// - Ghidra: `FUN_8027e080` at `0x8027E080` (804 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8027e080_FUN_8027e080.c`
/// - observed: the `stfd`/`psq_st` save of `f24..f31`, `lbz r0,0(r4)` /
///   `cmpi 0` guard, the `lfs 4(r3)`/`8(r3)`/`0xc(r3)`..`0x28(r3)` reads, the
///   two `cmpi` arms on `9(r4)`, `bl 0x80265f60` three times and
///   `bl 0x80265fa0` four times, `bl 0x801b1140` / `bl 0x801b1180`, the
///   `802814a0`..`802817d0` setter list and the `2068(r31)` row loop.
/// - `// inferred:` the `f1`-only setters take the voice in `r4`; the two
///   `0x80265F*` readers return one current voice value each.
///
/// # Arguments
/// - `obj`: the element; `+0x04..+0x28` its voice parameters, `+0x1C/+0x1D`
///   two bytes forwarded to the voice, `+0x50` the row table and `+0x814` its
///   row count.
/// - `spec`: the per-element request block; `+0` the enable byte, `+4` the
///   voice, `+8`/`+9` the two selector bytes, `+0x30`/`+0x34` the level words.
///
/// # Side effects
/// - writes the voice fields through the setters and takes/releases the sound
///   state lock.
///
/// # Safety
/// `obj` and `spec` must be readable and every pointer they hold valid; the
/// `+0x50` table must have `obj[+0x814]` rows of `0x18` bytes.
///
/// # Confidence
/// medium - the guard, the arithmetic and the setter order are literal; the
/// meaning of the two selector bytes and of the row table is inferred.
pub unsafe fn fn_8027e080(obj: usize, spec: usize) {
    if platform::read_u8(spec) == 0 {
        return;
    }

    let one = platform::read_f32(SEQ_MIX_ONE);
    let zero = platform::read_f32(SEQ_MIX_ZERO);
    let curve = one * platform::read_f32(obj + 8);
    let gain = one
        * platform::read_f32(obj + 4)
        * (platform::read_u8(spec + 8) as f32 / platform::read_f32(SDA2_EAEC))
        * platform::read_f32(spec + 0x30);

    let mut level = zero + platform::read_f32(obj + 0x0c);
    let select = platform::read_u8(spec + 9) as i32;
    let divisor = platform::read_f32(SDA2_EAF0);
    if select < 2 {
        level += (select - 0x3f) as f32 / divisor;
    } else {
        level += (select - 0x40) as f32 / divisor;
    }
    level += platform::read_f32(spec + 0x34);

    let third = zero + platform::read_f32(obj + 0x10);
    let fourth = one + platform::read_f32(obj + 0x14);
    let fifth = platform::read_f32(obj + 0x18);
    let sixth = zero + platform::read_f32(obj + 0x28);

    // The three running values start from the element's +0x14 word; each call
    // receives the previous result in f1.
    let mut running = platform::read_f32(obj + 0x14);
    let mut values = [0f32; 3];
    let mut index = 0;
    while index < 3 {
        values[index] = zero;
        running = call_f32_u32_u32(FUN_80265F60, running, obj as u32, index as u32);
        values[index] += running;
        index += 1;
    }

    let mut words = [0f32; 4];
    index = 0;
    while index < 4 {
        words[index] = call_u32_u32_f32(FUN_80265FA0, obj as u32, index as u32);
        index += 1;
    }

    let token = call0_r(FUN_801B1140);
    let voice = platform::read_u32(spec + 4) as usize;
    if voice != 0 {
        call_f32_u32(FUN_802814A0, gain, voice as u32);
        call_f32_u32(FUN_80281550, curve, voice as u32);
        call_f32_u32(FUN_802815B0, level, voice as u32);
        call_f32_u32(FUN_802815D0, third, voice as u32);
        call_f32_u32(FUN_802815F0, fourth, voice as u32);
        call_f32_u32_u32(
            FUN_80281610,
            fifth,
            voice as u32,
            platform::read_u8(obj + 0x1c) as u32,
        );
        platform::call2(FUN_80281680, voice as u32, platform::read_u8(obj + 0x1d) as u32);
        platform::call2(FUN_802816C0, voice as u32, platform::read_u32(obj + 0x20));
        call_f32_u32(FUN_802816E0, platform::read_f32(obj + 0x24), voice as u32);
        call_f32_u32(FUN_80281710, sixth, voice as u32);

        index = 0;
        while index < 3 {
            call_f32_u32_u32(FUN_80281750, values[index], voice as u32, index as u32);
            index += 1;
        }
        index = 0;
        while index < 4 {
            call_f32_u32_u32(FUN_80281790, words[index], voice as u32, index as u32);
            index += 1;
        }

        let rows = platform::read_i32(obj + 0x814);
        let mut row = obj + 0x50;
        index = 0;
        while (index as i32) < rows {
            platform::call3(FUN_802817D0, voice as u32, index as u32, row as u32);
            row += 0x18;
            index += 1;
        }
    }
    platform::call1(FUN_801B1180, token);
}

// ---------------------------------------------------------------------------
// Wave 112: the two entries the wave-109 pass left open, which together close
// the `0x80270000` bucket. Both are fully scalar once the export's paired-single
// noise is separated from the program state:
//
// * `0x80273560` opens with twenty-two `psq_st` stores of `f21..f31` into a
//   stack block that is never read again - the register save of the epilogue,
//   which Ghidra emitted at the top of the body. Nothing in the remaining
//   export touches those slots.
// * `0x80274480` prints one `psq_l` load four times per field, once for each
//   GQR quantisation class Ghidra cannot resolve statically. The scalar `f32`
//   arm is the one the neighbouring mix helpers ([`fn_802741e0`],
//   [`fn_80274630`]) use for the same loads, so the delta vector and the two
//   dot products are plain `f32`.
// ---------------------------------------------------------------------------

/// `0x8026 8900` - applies a pan value (`f1`) and a byte (`r5`) to a voice
/// (`r4`); [`fn_80273560`] calls it once per voice.
const FUN_80268900: usize = 0x8026_8900;

/// `0x8042EA18` = `r2 - 0x4668`: the `63.0` divisor that turns the two signed
/// envelope bytes of [`fn_80273560`] into a `-1.0..1.0` factor.
const SDA2_EA18: usize = 0x8042_EA18;

/// `0x8042EA1C` = `r2 - 0x4664`: the `-1.0` lower clamp of that factor.
const SDA2_EA1C: usize = 0x8042_EA1C;

/// `0x8042EA20` = `r2 - 0x4660`: the `2.0` upper clamp of the second factor.
const SDA2_EA20: usize = 0x8042_EA20;

/// `0x8042EA58` = `r2 - 0x4628`: `1/32`, the scale of the cone byte in
/// [`fn_80274480`] and of the spread byte in [`fn_80273560`].
const SDA2_EA58: usize = 0x8042_EA58;

/// `0x8042EA5C` = `r2 - 0x4624`: `65535.0`, the full-scale gain
/// [`fn_80274480`] reports when the second cone edge is behind the listener.
const SDA2_EA5C: usize = 0x8042_EA5C;

/// Converts one of the track's 7-bit level bytes to a `0.0..2.0` factor.
///
/// The export reaches this through the `0x4330_0000_xxxxxxxx` / `2^52` idiom,
/// which is only the unsigned-byte to double conversion; the value is then
/// divided by the `127.0` word at [`SDA2_EA10`].
///
/// # Safety
/// `SDA2_EA10` must hold the `127.0` divisor.
#[inline]
unsafe fn seq_byte_level(byte: u8) -> f32 {
    byte as f32 / platform::read_f32(SDA2_EA10)
}

/// Reads one of a track's four envelope counters.
///
/// Each group is four fields: `from` at `base`, `to` at `base + 1`, a signed
/// 16-bit limit at `base + 2` and the signed 16-bit position at `base + 4`.
/// While the position is below the limit the value steps linearly from `from`
/// to `to`; once it reaches the limit the routine returns `to`.
///
/// The first group (`base = 0x70`) interpolates the two bytes as *unsigned*,
/// the other three (`0x76`, `0x7C`, `0x82`) as *signed* `char`s, which is why
/// `signed` selects the interpretation.
///
/// # Safety
/// `track + base .. track + base + 6` must be readable.
unsafe fn track_envelope(track: usize, base: usize, signed: bool) -> i32 {
    let limit = platform::read_u16(track + base + 2) as i16 as i32;
    let position = platform::read_u16(track + base + 4) as i16 as i32;
    let raw_from = platform::read_u8(track + base);
    let raw_to = platform::read_u8(track + base + 1);

    let (from, to) = if signed {
        (raw_from as i8 as i32, raw_to as i8 as i32)
    } else {
        (raw_from as i32, raw_to as i32)
    };

    if position < limit {
        from.wrapping_add(position.wrapping_mul(to.wrapping_sub(from)) / limit)
    } else {
        to
    }
}

// ===========================================================================
// 0x80273560
// ===========================================================================

/// Re-derives every mixer parameter of a track and writes it to each voice of
/// its `+0xC8` list.
///
/// Under the sequence mutex the routine returns immediately when the track's
/// enable byte `+5` is clear or when it owns no voice. Otherwise the current
/// element at `+0xC4` and the track's four envelope counters are read:
///
/// * `+0x70..+0x74` gives the unsigned level byte, `+0x76..+0x7A` a signed
///   pitch offset (`/63`, clamped to `-1..1`), `+0x7C..+0x80` a signed
///   surround offset (`/63`, clamped to `0..2`) and `+0x82..+0x86` an
///   unclamped signed trim byte.
/// * The overall gain is the product of three squared `byte / 127` levels
///   (`+0x70`, the track's `+0x88`, the element's `+0xE8`) and the two linear
///   gains at `+8` and element `+4`.
/// * The element's `+0x1C` byte, when set, replaces the track's `+0x9A` pan
///   byte and `+0xA0` pan value; element `+0x1D` is forwarded as the voice's
///   `+0x3C` byte.
/// * Three running values (`(byte at +0x97..+0x99) / 127` plus
///   [`FUN_80265F60`]) and four words ([`FUN_80265FA0`]) are gathered from the
///   element, and the element's `0x18`-byte row table at `+0x50` (count at
///   `+0xE4`) is re-sent to the voice's `+0xD0` slot.
///
/// # Original
/// - Ghidra: `FUN_80273560` at `0x80273560` (1400 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80273560_FUN_80273560.c`
/// - observed: eleven paired stores at the top (`local_XX = (float)CONCAT13(
///   (char)(dVar17 * in_fNN), CONCAT12((char)(dVar17 * in_psNN_1), ...))` for
///   `f21..f31`), none of whose slots is read again; `iVar10 = FUN_802dcd70()`,
///   `FUN_8027b5f0()` + `FUN_801b2e60(manager + 0x354)`, the
///   `*(char *)(iVar10 + 5) == 0` and `*(int *)(iVar10 + 200) == 0` guards each
///   with `FUN_801b2f40(manager + 0x354)`, the four `*(short *)` / `*(char *)`
///   envelope groups, the four `CONCAT44(0x4330_0000, byte) -
///   *(double *)(r2 - 0x4658)` conversions over `*(float *)(r2 - 0x4670)`,
///   `FUN_80265f60` three times, `FUN_80265fa0` four times,
///   `FUN_80268900(dVar20,iVar11,cVar9)`, the nineteen voice stores, the
///   `*(int *)(iVar10 + 0xc4) + 0xE4` row loop through `FUN_802817d0`, and the
///   final `FUN_801b2f40`.
/// - `// inferred:` the export declares the routine `void`; its object comes
///   from `iVar10 = FUN_802dcd70()`, whose body only spills `r26..r31` (the
///   ABI register-save thunk), so that word is the untouched incoming `r3`.
///   [`fn_80272e30`] is the in-module call site and passes the track object
///   there, which is the same object whose `+0xC8` list and `+0xC4` element are
///   walked here.
/// - `// inferred:` the `f1` input of `FUN_80265F60` is not printed by the
///   export; the previous result is threaded as in [`fn_8027e080`].
///
/// # Arguments
/// - `this`: track object; `+5` enable byte, `+0xC4` element, `+0xC8` voice list
///   (`+0xD4` next), `+0x70..+0x8A` envelope bytes, `+0x96..+0xA0` mixer bytes.
///
/// # Side effects
/// - takes and releases the sequence mutex,
/// - writes nineteen fields of every voice in the `+0xC8` list and re-sends the
///   element's row table through [`FUN_802817D0`].
///
/// # Safety
/// `this` must be a live track object whose `+0xC4` element and `+0xC8` voice
/// list are intact; the element must hold its `0x18`-row table at `+0x50`.
///
/// # Confidence
/// medium - every offset, guard, product and store is literal in the export;
/// the argument source and the two callee contracts are inferred as noted.
pub unsafe fn fn_80273560(this: usize) {
    lock_seq_manager();

    if platform::read_u8(this + 5) == 0 {
        unlock_seq_manager();
        return;
    }
    if platform::read_u32(this + 0xc8) == 0 {
        unlock_seq_manager();
        return;
    }

    let element = platform::read_u32(this + 0xc4) as usize;
    let zero = platform::read_f32(SDA2_TRACK_DEFAULT_B);

    // Gain: three squared 7-bit levels times the two linear gains, multiplied
    // left to right in single precision as the original does.
    let level = seq_byte_level(track_envelope(this, 0x70, false) as u8);
    let track_level = seq_byte_level(platform::read_u8(this + 0x88));
    let element_level = seq_byte_level(platform::read_u8(element + 0xe8));

    let mut gain = platform::read_f32(SDA2_TRACK_DEFAULT_A);
    gain *= level;
    gain *= level;
    gain *= track_level;
    gain *= track_level;
    gain *= element_level;
    gain *= element_level;
    gain *= platform::read_f32(this + 8);
    gain *= platform::read_f32(element + 4);

    // Pitch offset: `envelope / 63`, clamped to `-1.0..1.0`.
    let pitch = track_envelope(this, 0x76, true) as f32 / platform::read_f32(SDA2_EA18);
    let mut pitch_level = platform::read_f32(SDA2_TRACK_DEFAULT_A);
    if pitch <= pitch_level {
        pitch_level = pitch;
        if pitch < platform::read_f32(SDA2_EA1C) {
            pitch_level = platform::read_f32(SDA2_EA1C);
        }
    }

    // Surround offset: `envelope / 63`, clamped to `0.0..2.0`.
    let surround = track_envelope(this, 0x7c, true) as f32 / platform::read_f32(SDA2_EA18);
    let mut surround_level = platform::read_f32(SDA2_EA20);
    if surround <= surround_level {
        surround_level = surround;
        if surround < zero {
            surround_level = zero;
        }
    }

    // Spread: the `+0x8A` byte times the unclamped trim byte, scaled by 1/32.
    let mut spread = platform::read_u8(this + 0x8a) as f32;
    spread *= track_envelope(this, 0x82, true) as f32;
    spread *= platform::read_f32(SDA2_EA58);

    let curve = platform::read_f32(SDA2_TRACK_DEFAULT_A)
        * platform::read_f32(element + 8)
        * platform::read_f32(this + 0x0c);
    let pitch_value = (zero + pitch_level)
        * platform::read_f32(this + 0x18)
        * platform::read_f32(element + 0xd0)
        + platform::read_f32(this + 0x10)
        + platform::read_f32(element + 0x0c);
    let surround_value = zero
        + surround_level
        + platform::read_f32(this + 0x14)
        + platform::read_f32(element + 0x10);
    let tail_value = zero + platform::read_f32(this + 0x9c) + platform::read_f32(element + 0x14);

    let mut pan = platform::read_f32(this + 0xa0);
    let mut pan_byte = platform::read_u8(this + 0x9a) as i8;
    if platform::read_u8(element + 0x1c) != 0 {
        pan = platform::read_f32(element + 0x18);
        pan_byte = platform::read_u8(element + 0x1c) as i8;
    }
    let voice_byte = platform::read_u8(element + 0x1d);
    let attenuation = seq_byte_level(platform::read_u8(this + 0x96))
        - platform::read_f32(SDA2_TRACK_DEFAULT_A)
        + platform::read_f32(element + 0x28);

    // Three running values: each starts at zero, gains the track byte and then
    // the element's current value, which is also threaded back in `f1`.
    let mut running = zero;
    let mut values = [0f32; 3];
    let mut index = 0;
    while index < 3 {
        running = call_f32_u32_u32(FUN_80265F60, running, element as u32, index as u32);
        values[index] = zero
            + seq_byte_level(platform::read_u8(this + 0x97 + index))
            + running;
        index += 1;
    }

    let mut words = [0f32; 4];
    index = 0;
    while index < 4 {
        words[index] = call_u32_u32_f32(FUN_80265FA0, element as u32, index as u32);
        index += 1;
    }

    let mut voice = platform::read_u32(this + 0xc8) as usize;
    while voice != 0 {
        platform::write_f32(voice + 0x40, gain);
        platform::write_f32(voice + 0x80, spread);
        platform::write_f32(voice + 0x44, curve);
        platform::write_f32(voice + 0x48, pitch_value);
        platform::write_f32(voice + 0x4c, surround_value);
        platform::write_f32(voice + 0x50, tail_value);
        call_f32_u32_u32(FUN_80268900, pan, voice as u32, pan_byte as u32);
        platform::write_u8(voice + 0x3c, voice_byte);
        platform::write_u32(voice + 0x58, platform::read_u32(element + 0x20));
        platform::write_u32(voice + 0x5c, platform::read_u32(element + 0x24));
        platform::write_f32(voice + 0x60, attenuation);
        platform::write_f32(voice + 0x64, values[0]);
        platform::write_f32(voice + 0x68, values[1]);
        platform::write_f32(voice + 0x6c, values[2]);

        let mut slot = voice + 0x70;
        index = 0;
        while index < 4 {
            platform::write_f32(slot, words[index]);
            slot += 4;
            index += 1;
        }

        let rows = platform::read_i32(element + 0xe4);
        let mut row = element + 0x50;
        index = 0;
        while (index as i32) < rows {
            platform::call3(
                FUN_802817D0,
                platform::read_u32(voice + 0xd0),
                index as u32,
                row as u32,
            );
            row += 0x18;
            index += 1;
        }

        platform::write_u32(voice + 0x1c, platform::read_u32(this + 0x58));
        platform::write_u32(voice + 0x20, platform::read_u32(this + 0x5c));
        platform::write_u32(voice + 0x24, platform::read_u32(this + 0x60));
        platform::write_u8(voice + 0x28, platform::read_u8(this + 0x64));
        platform::write_u16(voice + 0x29, platform::read_u16(this + 0x65));
        platform::write_u8(voice + 0x2b, platform::read_u8(this + 0x67));
        platform::write_u8(voice + 0x34, platform::read_u8(this + 0x68));

        voice = platform::read_u32(voice + 0xd4) as usize;
    }

    unlock_seq_manager();
}

// ===========================================================================
// 0x80274480
// ===========================================================================

/// Returns the directional (cone) gain of one node's source.
///
/// `this + 0x24` is the cosine limit of the cone. When it is zero the routine
/// reports full gain (`1.0`) without looking at the node. Otherwise the delta
/// from the node's cached position `+0x30..+0x38` to the three-float `vector`
/// is measured by [`FUN_801C14C0`] and, when non-zero, normalised; the cone
/// byte at `vector + 0x1E` scales it by `1/32`.
///
/// Two dot products are then formed against the vector's axis
/// (`vector + 0x0C..+0x14`) and the node's normal (`+0x3C..+0x44`), both
/// negated. A node whose position coincides with the listener avoids the
/// division by zero: the two vector lengths stand in for the dot products
/// (the axis one negated).
///
/// The result is a two-edge ramp:
///
/// - normal dot above the limit: `0.0`
/// - normal dot at or below it, axis dot below it:
///   `(limit - normal) / (limit - axis)`
/// - normal dot at or below it, axis dot at or above it: `65535.0`
///
/// # Original
/// - Ghidra: `FUN_80274480` at `0x80274480` (432 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80274480_FUN_80274480.c`
/// - observed: the early `if ((double)*(float *)(r2 - 0x4640) == dVar16)` arm
///   storing `*(float *)(r2 - 0x4644)`, the three-arm load ladder that repeats
///   for every field (`*(char *)`, `*(short *)` or `*(float *)` after a
///   `bVar4 & 7` test), the `dVar11 = dVar11 - dVar9` / `dVar18 = dVar18 -
///   dVar17` deltas into `local_58` / `local_54` / `local_50`, the
///   `(double)FUN_801c14c0(&local_58)` length and the
///   `*(float *)(r2 - 0x4644) / dVar11` normalisation, the
///   `CONCAT44(0x4330_0000, byte at 0x1E)` scale with
///   `*(float *)(r2 - 0x4628)`, the two
///   `dVar11 = -(dVar10 * dVar12 + dVar17 * dVar11 + dVar13 * dVar9)` dot sums,
///   the `if (dVar11 <= 0.0)` length fallback with `FUN_801c14c0(param_3 + 3)`
///   and `FUN_801c14c0(param_2 + 0x3c)`, and the `dVar16` comparisons with the
///   `*(float *)(r2 - 0x4624)` / `*(float *)(r2 - 0x4640)` arms.
/// - `// inferred:` each field is printed three times, once per GQR
///   quantisation class (signed/unsigned byte, half-word, `f32`). The `f32`
///   arm is the one [`fn_802741e0`] and [`fn_80274630`] take for the same
///   `psq_l` loads in this structure, and the two vectors are plainly float.
/// - `// inferred:` the cone limit is a cosine because the two dot products are
///   taken between the normalised delta and unit-length direction vectors.
///
/// # Arguments
/// - `this`: the element player; `+0x24` the cone cosine limit.
/// - `node`: node record; `+0x30..+0x38` the cached position, `+0x3C..+0x44`
///   its normal.
/// - `vector`: mix-request block; `+0x00..+0x08` the position, `+0x0C..+0x14`
///   the axis, `+0x1E` the cone scale byte.
/// - `out`: receives the gain.
///
/// # Side effects
/// - writes `out` only.
///
/// # Safety
/// `node` and `vector` must be readable, `out` writable, and the two vectors
/// must be valid for [`FUN_801C14C0`].
///
/// # Confidence
/// medium - the guards, the two dot products and the ramp are literal; the GQR
/// class and the physical meaning of the limit are inferred as noted.
pub unsafe fn fn_80274480(this: usize, node: usize, vector: usize, out: usize) {
    let limit = platform::read_f32(this + 0x24) as f64;
    let zero = platform::read_f32(SDA2_EA40) as f64;

    if zero == limit {
        platform::write_f32(out, platform::read_f32(SDA2_EA3C));
        return;
    }

    let mut delta = [
        platform::read_f32(vector) - platform::read_f32(node + 0x30),
        platform::read_f32(vector + 4) - platform::read_f32(node + 0x34),
        platform::read_f32(vector + 8) - platform::read_f32(node + 0x38),
    ];
    let distance = call_ptr_f32(FUN_801C14C0, delta.as_ptr() as u32) as f64;

    if zero < distance {
        let inverse = (platform::read_f32(SDA2_EA3C) as f64 / distance) as f32;
        delta[0] *= inverse;
        delta[1] *= inverse;
        delta[2] *= inverse;
    }

    let scale =
        ((platform::read_u8(vector + 0x1e) as f32) * platform::read_f32(SDA2_EA58)) as f64;

    // Edge A is the axis dot product, edge B the node-normal one; with no
    // distance the original substitutes the two vector lengths.
    let (edge_a, edge_b) = if distance <= zero {
        let axis_length = call_ptr_f32(FUN_801C14C0, (vector + 0x0c) as u32) as f64;
        let normal_length = call_ptr_f32(FUN_801C14C0, (node + 0x3c) as u32) as f64;
        (-axis_length, normal_length)
    } else {
        let x = delta[0] as f64;
        let y = delta[1] as f64;
        let z = delta[2] as f64;
        (
            -(x * platform::read_f32(vector + 0x0c) as f64
                + y * platform::read_f32(vector + 0x10) as f64
                + z * platform::read_f32(vector + 0x14) as f64),
            -(x * platform::read_f32(node + 0x3c) as f64
                + y * platform::read_f32(node + 0x40) as f64
                + z * platform::read_f32(node + 0x44) as f64),
        )
    };

    let first = ((edge_b * scale) as f32) as f64;
    let second = ((edge_a * scale) as f32) as f64;

    let gain = if first <= limit {
        if second < limit {
            ((limit - first) as f32) / ((limit - second) as f32)
        } else {
            platform::read_f32(SDA2_EA5C)
        }
    } else {
        platform::read_f32(SDA2_EA40)
    };

    platform::write_f32(out, gain);
}

// ===========================================================================
// 0x80275590..0x80275624 -- cue-object vtable tail forwarders
//                            (Wave 274 lane D, `_D274`, manual)
// ===========================================================================
//
// **What lives here.** Four of the twelve untranslated methods of the 14-slot
// cue-object vtable `0x8032E040` (header `0x8032E038`): its slots 1..4, the
// generic "forward one argument triple to the archive/sub-object" entries.
// They sit in the archive front end between [`fn_80275560`]/[`fn_80275570`] and
// [`fn_80275640`], and the same four words appear in every sibling cue table
// (`0x803089CC`, `0x80308A4C`, `0x80312324`, `0x80321B7C`, `0x8034F0C4`).
// The four 10-slot small-record arrays `0x8030F060`, `0x8030F090`,
// `0x8030F0F8` and `0x8030F190` carry these same four words in slots 1..4 and
// `0x80197570` in slot 5; their uncovered destructor and publisher methods are
// decoded in `audio::ax` under the `_C287` section.
//
// Slots 3 and 4 tail-call `0x80278AD0`, the shared dispatcher that takes
// `(archive_object, arg1, arg2, arg3, owner, byte, arg4)`; slot 2 tail-calls
// `+0x28` of a sub-object whose vtable pointer lives at `object + 0x08`; slot 1
// tail-calls `+0x14` of its own vtable with a stack byte.

/// `0x80278AD0` - the shared argument-shuffling dispatcher the two leaf
/// forwarders below branch into.
const FN_80278AD0: usize = 0x8027_8AD0;

/// Tail-calls the shared dispatcher `0x80278AD0` from slot 3.
///
/// The shipped body moves `r7` to `r8`, dereferences its low byte, puts the
/// owner in `r7`, zeroes `r6`, and branches with
/// `(this->+4, arg1, arg2, 0, this, *arg4, arg3)`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80275590`.
/// - Extent: `0x80275590..0x802755AC` (28 bytes, seven instructions); the
///   `b 0x80278AD0` at the end is the extent edge.
/// - Table: slot 3, word `0x8032E04C`.
/// - Evidence: `work/wave274/d_dis.txt`.
///
/// # Arguments
/// - `this`: cue object; the archive/resolver object is at `+0x04`.
/// - `a`, `b`: words forwarded unchanged.
/// - `c`: word parked in `r9` before the branch.
/// - `d`: pointer whose first byte becomes `r8`.
///
/// # Returns
/// Whatever `0x80278AD0` returns.
///
/// # Side effects
/// None in this body.
///
/// # Safety
/// `this + 4` must hold a live object and `d` must be readable.
///
/// # Confidence
/// high - every register move and the byte load are literal.
pub unsafe fn fn_80275590(this: usize, a: u32, b: u32, c: u32, d: usize) -> u32 {
    platform::call7(
        FN_80278AD0,
        platform::read_u32(this + 4),
        a,
        b,
        0,
        this as u32,
        platform::read_u8(d) as u32,
        c,
    )
}

/// Tail-calls the shared dispatcher `0x80278AD0` from slot 4.
///
/// The same dispatcher as [`fn_80275590`] with `r6 = arg4`, `r8 = *arg5` and
/// `r9 = arg3`: `(this->+4, arg1, arg2, arg4, this, *arg5, arg3)`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802755B0`.
/// - Extent: `0x802755B0..0x802755CC` (28 bytes, seven instructions).
/// - Table: slot 4, word `0x8032E050`.
/// - Evidence: `work/wave274/d_dis.txt`.
///
/// # Arguments
/// - `this`: cue object.
/// - `a`, `b`: words forwarded unchanged.
/// - `c`: word parked in `r9` before the branch.
/// - `d`: word passed as the dispatcher's fourth argument.
/// - `e`: pointer whose first byte becomes `r8`.
///
/// # Returns
/// Whatever `0x80278AD0` returns.
///
/// # Side effects
/// None in this body.
///
/// # Safety
/// `this + 4` must hold a live object and `e` must be readable.
///
/// # Confidence
/// high - every register move and the byte load are literal.
pub unsafe fn fn_802755b0(this: usize, a: u32, b: u32, c: u32, d: u32, e: usize) -> u32 {
    platform::call7(
        FN_80278AD0,
        platform::read_u32(this + 4),
        a,
        b,
        d,
        this as u32,
        platform::read_u8(e) as u32,
        c,
    )
}

/// Forwards `(arg2, arg3, arg5, &byte(arg4))` to vtable slot `+0x14`.
///
/// The fourth argument is not passed by value: its low byte is spilled to
/// `sp + 8` and the address is passed in `r7`. The virtual call receives
/// `(this, arg1, arg2, arg4, &byte)`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x802755D0`.
/// - Extent: `0x802755D0..0x80275608` (56 bytes), ending at the `bclr`;
///   two zero words follow.
/// - Table: slot 1, word `0x8032E044`.
/// - Evidence: `work/wave274/d_dis.txt`.
///
/// # Arguments
/// - `this`: cue object.
/// - `a`, `b`: words forwarded unchanged.
/// - `c`: byte spilled to the stack; only the low byte is used.
/// - `d`: word moved into the fourth argument slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Depends on the callee.
///
/// # Safety
/// `+0x14` of the vtable must name a routine shaped
/// `(obj, u32, u32, u32, *mut u8)`.
///
/// # Confidence
/// high - the spill, both register moves and the `mtctr`/`bcctr` sequence are
/// literal.
pub unsafe fn fn_802755d0(this: usize, a: u32, b: u32, c: u8, d: u32) {
    let mut flag = c;
    let method = platform::read_u32(platform::read_u32(this) as usize + 0x14) as usize;
    let call: extern "C" fn(u32, u32, u32, u32, *mut u8) = core::mem::transmute(method);
    call(this as u32, a, b, d, &mut flag);
}

/// Tail-calls `+0x28` of the sub-object's vtable from slot 2.
///
/// The sub-object is the word at `this + 4`; unlike the cue object its vtable
/// pointer lives at `+0x08`, and the callee receives
/// `(sub_object, arg1, arg2)`.
///
/// # Original (untabled)
/// - Ghidra: none - `functions.tsv` has no row for `0x80275610`.
/// - Extent: `0x80275610..0x80275624` (20 bytes, five instructions).
/// - Table: slot 2, word `0x8032E048`.
/// - Evidence: `work/wave274/d_dis.txt`.
///
/// # Arguments
/// - `this`: cue object.
/// - `a`, `b`: words forwarded unchanged.
///
/// # Returns
/// Whatever the sub-object's `+0x28` method returns.
///
/// # Side effects
/// None in this body.
///
/// # Safety
/// `this + 4` must hold a live sub-object and `+0x08` of it must name a vtable.
///
/// # Confidence
/// high - the two loads, the third load at `+0x28` and the tail `bcctr` are
/// literal.
pub unsafe fn fn_80275610(this: usize, a: u32, b: u32) -> u32 {
    let object = platform::read_u32(this + 4) as usize;
    let method = platform::read_u32(platform::read_u32(object + 8) as usize + 0x28) as usize;
    let call: extern "C" fn(u32, u32, u32) -> u32 = core::mem::transmute(method);
    call(object as u32, a, b)
}
