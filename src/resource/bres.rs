//! BRRES loading: U8 archive lookup, NW4R resource archives, and the BRRES
//! container entry point.
//!
//! # What this module covers
//!
//! This is the **loader side** of the BRRES story. It answers three questions:
//!
//! 1. Where does the game keep a mounted `.arc` (U8) archive, and how does it
//!    find a member such as `g3d/daikonbou.brres` inside it?
//! 2. What object represents that mounted archive, how is it refcounted, and
//!    how is it released?
//! 3. Given the bytes of a BRRES file, how does actor code look up a named
//!    resource in the BRRES root dictionary?
//!
//! The first half is **container and directory lookup**. The second half now
//! also walks the NW4R `ResDict` tree, exposes the typed BRRES section
//! accessors (`Textures`, `Palettes`, `AnmChr`, ...), validates section
//! revisions, binds every present resource, and provides the MDL0 payload
//! accessors for node trees, bones, materials, shaders, textures, palettes
//! and polygons. The final group adds the deferred bind pass: it resolves the
//! texture/palette names stored in a model material and in an `AnmTexPat`
//! animation, caching the resulting TEX0/PLT0 pointers in the resource. It
//! still does **not** describe the full MDL0/TEX0 memory layout; those records
//! stay opaque.
//!
//! # The loader path, in one picture
//!
//! ```text
//! actor code
//!   -- fn_800df180(table, slot_name, "g3d/foo.brres")
//!        -- fn_800df640 -> fn_800df570     find the named resource slot
//!             -- slot+0x28                  mounted NW4R archive object
//!                  -- fn_802b76c0(archive, path)
//!                       -- fn_8019f700         U8 ARCOpen (find member)
//!                       -    -- fn_8019fa00    U8 path walk over the node table
//!                       -- fn_8019fe50        archive_base + member_data_offset
//!                            = pointer to the first byte of the BRRES file
//!
//! fn_80239e30(file_ptr, name)
//!   -- section "3DModels(NW4R)" in the BRRES root dict
//!        -- named resource inside that section
//! ```
//!
//! Mounting and release:
//!
//! ```text
//! fn_802b75b0(data, heap, flags)     acquire (first use: allocate + mount)
//!   -- fn_802b74c0                  already mounted for this data pointer?
//!   -- fn_802b7380                  construct + register in the global list
//!   -- fn_802b7460                  fn_8019f660 (ARCInitHandle)
//!
//! fn_802b7680(obj)                   release; refcount 0 -> virtual destroy
//! ```
//!
//! # Address ranges and original files
//!
//! | cluster | original range | evidence |
//! |---|---|---|
//! | game resource slots | `0x800DF570`, `0x800DF640`, `0x800DF180` | `800df570_FUN_800df570.c`, `800df640_FUN_800df640.c`, `800df180_FUN_800df180.c` |
//! | U8 archive reader | `0x8019F660..0x8019FF80` | `8019f660_FUN_8019f660.c` - `8019ff80_FUN_8019ff80.c` |
//! | NW4R archive registry | `0x802B7380..0x802B7A40` | `802b7380_FUN_802b7380.c` - `802b7a40_FUN_802b7a40.c` |
//! | BRRES dictionary core | `0x80239930..0x80239DC0` | `80239930_FUN_80239930.c` - `80239dc0_FUN_80239dc0.c` |
//! | BRRES typed access | `0x80239E30..0x8023A270` | `80239e30_FUN_80239e30.c` - `8023a270_FUN_8023a270.c` |
//! | BRRES validation/bind | `0x8023A590`, `0x8023A860` | `8023a590_FUN_8023a590.c`, `8023a860_FUN_8023a860.c` |
//! | MDL0 payload access | `0x8023B1B0..0x8023B3A0` | `8023b1b0_FUN_8023b1b0.c` - `8023b3a0_FUN_8023b3a0.c` |
//! | MDL0 extended payload access | `0x8023B460..0x8023BA10` | `8023b460_FUN_8023b460.c` - `8023ba10_FUN_8023ba10.c` |
//! | deferred resource bind | `0x8023A350`, `0x8023BA50`, `0x8023E250`, `0x8023EAE0..0x8023F720` | `8023a350_FUN_8023a350.c`, `8023ba50_FUN_8023ba50.c`, `8023e250_FUN_8023e250.c`, `8023eae0_FUN_8023eae0.c` - `8023f720_FUN_8023f720.c` |
//! | MDL0 binders and payload accessors | `0x8023B400..0x8023F890` | `8023b400_FUN_8023b400.c` - `8023f890_FUN_8023f890.c` |
//! | material/texture state payload | `0x8023CB60..0x8023EE90` | `8023cb60_FUN_8023cb60.c` - `8023ee90_FUN_8023ee90.c` |
//! | cached-block publish and object-cache accessors | `0x8023C610..0x8023EAC0` | `8023c610_FUN_8023c610.c` - `8023eac0_FUN_8023eac0.c` |
//! | engine debug console and script-stream band | `0x802307E0..0x80237A00` | `802307e0_FUN_802307e0.c` ... `80237a00_FUN_80237a00.c` |
//! | polygon/material build, packed transforms and G3D math | `0x80238420..0x8023F5A0` | `80238420_FUN_80238420.c` - `8023f5a0_FUN_8023f5a0.c` |
//! | console metrics and quantised vector stores | `0x80230F50..0x802384F0` | `80230f50_FUN_80230f50.c` ... `802384f0_FUN_802384f0.c` |
//! | remaining math, glyph, report and queue band (wave 100) | `0x802337B0..0x8023FE50` | `802337b0_FUN_802337b0.c` ... `8023fe50_FUN_8023fe50.c` |
//! | text anchor, console layout chain and frame pump (wave 106) | `0x80230280..0x802348F0` | `80230280_FUN_80230280.c` ... `802348f0_FUN_802348f0.c` |
//! | script-symbol parser and beam-matrix wrapper (wave 109) | `0x80235AE0`, `0x80238930` | `80235ae0_FUN_80235ae0.c`, `80238930_FUN_80238930.c` |
//! | rotation-matrix builder (wave 111) | `0x802382B0` | `802382b0_FUN_802382b0.c` |
//! | segment-to-segment closest point (wave 112) | `0x802390D0` | `802390d0_FUN_802390d0.c` |
//! | beam/shadow-volume record builder (wave 115) | `0x802389F0` | `802389f0_FUN_802389f0.c` |
//!
//! All addresses are virtual addresses of `sys/main.dol` (image base
//! `0x80004000`). This is main-DOL code, not REL code.
//!
//! # BRRES container facts used here
//!
//! A BRRES file begins with the four bytes `bres` (`0x62726573`), a `u16`
//! version at `+0x04`, the total size at `+0x08`, and a `u16` root-section
//! offset at `+0x0C`. The root section is a normal NW4R section: a four-byte
//! tag, a size, and a name dictionary. Its dictionary starts at **file
//! `+0x18`**, which is the base [`fn_80239e30`] passes to
//! [`res_dict_get_section`].
//!
//! The dictionary node records are 16 bytes:
//!
//! | offset | meaning (`// observed:` in `80239a80` and a real file) |
//! |---|---|
//! | `+0x00` | `u16` hash/bit index used by the tree walk |
//! | `+0x02` | `u16` padding |
//! | `+0x04` | `u16` left child node index |
//! | `+0x06` | `u16` right child node index |
//! | `+0x08` | `u32` offset (from the dictionary base) to a length-prefixed name |
//! | `+0x0C` | `u32` offset (from the dictionary base) to the resource/section data |
//!
//! The section names are stored as length-prefixed strings in the image:
//!
//! | tag address | name |
//! |---|---|
//! | `0x8034DF60` | `3DModels(NW4R)` (used by [`fn_80239e30`]) |
//! | `0x8034DF80` | `Palettes(NW4R)` |
//! | `0x8034DFA0` | `Textures(NW4R)` |
//! | `0x8034DFC0` | `AnmChr(NW4R)` |
//! | `0x8034DFE0` | `AnmVis(NW4R)` |
//! | `0x8034E000` | `AnmClr(NW4R)` |
//! | `0x8034E020` | `AnmTexPat(NW4R)` |
//! | `0x8034E040` | `AnmTexSrt(NW4R)` |
//! | `0x8034E060` | `AnmShp(NW4R)` |
//! | `0x8034E080` | `AnmScn(NW4R)` |
//!
//! These were cross-checked against `scene/scene.brres` extracted from
//! `Env/Env_course.arc`. They are **not** invented symbol names; they are the
//! bytes at those addresses.
//!
//! # U8 / ARC facts used here
//!
//! The U8 reader is the standard Nintendo archive layout:
//!
//! * header: `+0x00` magic `55 AA 38 2D`, `+0x04` node-table offset,
//!   `+0x08` header size (also the data offset), `+0x0C` string-table offset.
//! * node record (12 bytes): `+0x00` top byte = type (0 = file, 1 = directory)
//!   and low 24 bits = name offset; `+0x04` = data offset for a file or parent
//!   index for a directory; `+0x08` = size for a file or the index one past the
//!   directory's children for a directory.
//! * root node is index 0; its `+0x08` word is the total node count.
//!
//! The in-memory handle built by [`arc_init_handle`] is 0x1C bytes:
//!
//! | offset | meaning |
//! |---|---|
//! | `+0x00` | archive base (first byte of the `.arc` image) |
//! | `+0x04` | node table (`base + header +0x04`) |
//! | `+0x08` | string table (`base + header +0x0C`) |
//! | `+0x0C` | node count |
//! | `+0x10` | one past the node table |
//! | `+0x14` | archive data offset (the header's `+0x08` value) |
//! | `+0x18` | current directory node index (0 = root) |
//!
//! # Start here
//!
//! Read [`fn_800df180`] first; it is the shortest end-to-end example. Then
//! [`fn_802b76c0`] shows the archive-object hand-off, and [`arc_find_index`]
//! shows the actual U8 path walk. [`fn_802b75b0`] and [`fn_802b7680`] are the
//! acquire/release pair.
//!
//! # Dependencies
//!
//! * [`crate::platform`] - big-endian volatile loads/stores and indirect
//!   calls. The crate is `no_std`; this module does not allocate.
//! * Callees outside this file are called by absolute address and listed as
//!   constants below: `OSReport`/`OSPanic`-shaped logging, libc string
//!   helpers, the NW4R `ut::List` helpers, the archive allocator, the G3D
//!   per-resource binders (`0x8023ECE0`, `0x8023EAE0`), the MDL0 publish
//!   helpers (`0x8023EA20`..`0x8023EAC0`), and the texture/palette revision
//!   validators (`0x8023ECB0`, `0x8023EB00`).
//! * Format consumers are elsewhere: [`crate::resource::archive`] is the
//!   sound-archive reader (a different format), and full MDL0/TEX0 field
//!   interpretation still belongs to a future `resource::g3d` module.
//!
//! # Glossary
//!
//! * **DOL** - the Wii executable container `sys/main.dol`; addresses here are
//!   virtual, not file offsets.
//! * **U8 / ARC** - Nintendo's generic archive (`55 AA 38 2D`); the container
//!   that holds `.brres` members. See `docs/GLOSSARY.md`.
//! * **BRRES** - NintendoWare resource container (`bres`), holding models,
//!   textures, palettes and animations.
//! * **NW4R** - NintendoWare 4 Revolution, the engine library family.
//! * **resource slot** - a 0x40-byte record in the game's global resource
//!   table; the slot at `+0x28` points at a mounted NW4R archive object.
//! * **archive object** - the 0x3C-byte object built by [`fn_802b7380`]; it
//!   owns an embedded U8 handle at `+0x18`, a mode word at `+0x10`, and a
//!   refcount at `+0x14`.
//! * **dictionary** - the BRRES/NW4R name tree; an index selects a 16-byte
//!   node record starting at dictionary `+0x08`.
//!
//! # Status and provenance
//!
//! The wave-90 group adds the **engine debug-console and script-stream band**
//! that shares this address bucket: the console framebuffer defaults and
//! readiness test at `0x802331A0`/`0x80233260`, the rectangle fill and cache
//! flush at `0x80233290`/`0x80233460`, the text block printer and its line
//! layout at `0x80233570`/`0x802336D0`, the console-list unlink pair at
//! `0x80233C60`/`0x80233EC0`, the incremental repaint at `0x80233EF0`, the
//! scrolling grid writer at `0x80234120`, the frame-path install and per-frame
//! entry at `0x80234630`/`0x80234720`, and the script queue at
//! `0x80235710..0x80235EB0` plus the two heap/record pairs at
//! `0x80235FE0..0x80237A00`. None of these functions touches BRRES state; the
//! bucket, not the subsystem, is what this lane owns.
//!
//! One hundred and ninety-four functions, all translated from the Ghidra C.
//! None is a stub. Wave 95 brings the total to **two hundred and sixteen**;
//! wave 98 brings it to **two hundred and twenty-seven**.
//! The BRRES dictionary is now implemented directly by [`res_dict_find`],
//! [`res_dict_find_name`], [`res_dict_get_named`] and
//! [`res_dict_get_section`]; the old opaque helper addresses `0x80239CC0` and
//! `0x80239C40` are therefore no longer called indirectly.
//!
//! The wave-7 group still calls `0x8023E390` by address: that routine writes
//! GX state and takes eight arguments, one more than [`crate::platform`]
//! exposes. Everything it can reach through the BRRES container is already
//! represented here.
//!
//! The wave-9 group adds the alternate indexed accessors at
//! `0x8023B400..0x8023BA10`, the MDL0 model binder `0x8023BB60`, the polygon
//! material/shader accessors at `0x8023C100..0x8023C220`, the `+0x30` entry
//! helpers at `0x8023E320..0x8023E370`, the material texture-state accessors
//! at `0x8023E650..0x8023E960`, the two TEX0 descriptor readers at
//! `0x8023EB30`/`0x8023EBF0`, and the `AnmTexPat` slot clear at `0x8023F890`.
//!
//! The wave-18 group adds the material/texture state payload between
//! `0x8023CB60` and `0x8023EE90`: the eight-entry texture and palette object
//! caches, the per-material texture-matrix and texgen setters, the texture
//! coordinate/colour state getters and emitters, the cache-publish pair, and
//! the two material channel-state writers. It also adds the animation frame
//! ratio helper at `0x8023D240`.
//!
//! The wave-70 group adds the **cached-block publish helpers and the object
//! cache accessors** that surround that payload: the two GX block-word
//! emitters at `0x8023C610`/`0x8023C6F0`, the self-relative 0x200-byte block
//! copy at `0x8023C9B0`, the eight fixed-size flush/store publishers at
//! `0x8023CA20..0x8023CB30`, the three fixed-size block copies at
//! `0x8023CBA0..0x8023CC20`, the remaining texture and palette slot accessors
//! at `0x8023CC90`/`0x8023CDC0`, the record header readers at
//! `0x8023D0A0..0x8023D190`, the present-gated 0x40 emitter and the two
//! texcoord variant readers at `0x8023D5B0..0x8023DCB0`, the count-selected
//! publisher at `0x8023DDD0`, and the six per-entry cache thunks at
//! `0x8023EA20..0x8023EAC0`. The first accessor of each slot pair
//! (`0x8023CC60`, `0x8023CD90`) and the texture bitset test `0x8023CCC0` were
//! claimed earlier by `runtime::bt`, so they are not duplicated here.
//!
//! The wave-85 group finishes the **BRRES material/polygon build path and its
//! math support**: the polygon display-list builder at `0x8023C280`, the
//! texture-coordinate record read/rewrite pair at `0x8023C760`/`0x8023C830`,
//! the packed 11-bit matrix decoder/encoder at `0x8023D5F0`/`0x8023D8B0`/
//! `0x8023DAA0`, the 0x12-byte attribute record decode/write pair at
//! `0x8023DEA0`/`0x8023E050`, the material texture/palette binder at
//! `0x8023E390`, the two per-entry animation resolvers at `0x8023F370`/
//! `0x8023F5A0`, and four NW4R G3D math helpers at `0x80238420`/`0x80238690`/
//! `0x80238750`/`0x80239710` (matrix-times-vector, box expansion, box
//! transform and box overlap).
//!
//! `// partial:` none in the wave-85 group. The remaining five functions of the band
//! (`0x8023F000`, `0x8023F140`, `0x8023F4E0`, `0x8023FA30`, `0x8023FE50`)
//! are key-sampling routines whose Ghidra bodies depend on the caller-set GQR
//! quantization; they stay untranslated rather than guessing that register.
//! Wave 90 leaves the same GQR-dependent group untouched: nearly all of the
//! packed-vector math at `0x802380A0..0x80239780` (and `0x8023F000..0x8023FE50`)
//! reads `unaff_GQR0`/`unaff_GQR3`, which is exactly the register the brief
//! forbids guessing; the single exception, `0x80238270`, is a two-line wrapper
//! that forwards a scaled argument to `0x801C0B50` and carries no field
//! evidence of its own (wave 95 later took it, at low confidence). Three
//! console-band bodies were also skipped because the export has lost the
//! register arguments: the glyph blitter `0x802337B0` (bit-level 5x7 font
//! expansion through a folded shift chain), the frame loop `0x802348F0` and
//! the script parser `0x80235AE0`. All three were later recovered with the
//! calls intact: wave 100 took the glyph blitter, wave 106 the frame loop and
//! wave 109 the parser. Wave 95
//! translated the fourth, the node dispatcher `0x80235DC0`, once the
//! prologue-stub reading explained its lost `r3`/`r4`. Wave 98 then took the
//! seven *vector-store* members of the packed group (`0x802380A0`, `0x802380D0`,
//! `0x80238110`, `0x80238130`, `0x802381A0`, `0x802384B0`, `0x802384F0`) by
//! passing the live GQR word in as an explicit argument; the rest of the group
//! reads a quantised *index* back as raw bits that the export leaves undefined,
//! so it stays skipped. All of them are listed in
//! `docs/COVERAGE_WAVE90_BRES.md`, `docs/COVERAGE_WAVE95_BRES.md` and
//! `docs/COVERAGE_WAVE98_BRES.md`.
//!
//! The wave-95 group closes most of the remaining console band. It adds the
//! 0x64-byte text-stream constructor and its advance probe at
//! `0x80230780`/`0x802308D0`, the copy-and-run helper `0x80231870`, the two
//! variadic script-line entry points at `0x802319B0`/`0x80231AC0`, the text-box
//! measure `0x802322C0`, the framebuffer allocator `0x80233A70`, the console
//! record constructor `0x80233D10`, and the whole exception report at
//! `0x80234B60`/`0x80234F40`/`0x80235180`/`0x802352A0` (header, FPSCR groups,
//! GPR dump, FPR dump). It also adds the eight message-queue routines
//! `0x80235DC0` and `0x80236350..0x80237610`: the script-node opener, the two
//! `snprintf`-shaped formatters per heap, the two record sinks and the two
//! queue drains. The last body, `0x80238270`, is the one double-scaling
//! wrapper wave 90 left behind.
//!
//! Two artefacts of the original toolchain shape these bodies and are worth
//! knowing before reading them. Metrowerks emits `bl _savegpr0_26` in every
//! prologue, and Ghidra models that stub as a call whose "return value" is
//! really the incoming `r3:r4`; six bodies here recover a parameter that way.
//! And several bodies test `r2`-relative small-data words whose values the
//! export never shows, so those constants stay `// unknown:` by address.
//!
//! `// unknown:` the meaning of the archive-object `+0x10`
//! mode word beyond "1 = U8 handle is valid", the MDL0 dictionaries at `+0x28`
//! and `+0x2C` (absent from every sampled model), the exact role of the
//! `+0x30` dictionary beyond "keyed by short material names", and the byte
//! count source in `0x8023E740`. The list-helper names are `// inferred:` from
//! their intrusive-link shape, not from a symbol table. The per-material
//! binder `0x8023E390` remains an address call; the model binder
//! `0x8023BB60` is now [`mdl_model_bind`].
//!
//! The wave-98 group closes the rest of the plain `0x80230000` console band and
//! the six reconstructible members of the packed-vector group: the two text-box
//! measures at `0x80230F50`/`0x802310D0`, the variadic print sink
//! `0x80233480` and its formatting core `0x802344E0`, then the quantised
//! vector stores `0x802380A0`, `0x802380D0`, `0x80238110`, `0x80238130`,
//! `0x802381A0`, `0x802384B0` and `0x802384F0`. The last seven take the live
//! GQR word as an explicit `gqr` argument - the value is the caller's, not the
//! function's - and share the `psq_store`/`psq_load` helpers defined in that
//! section. `docs/COVERAGE_WAVE98_BRES.md` lists the bodies that remain.
//!
//! The wave-100 group takes twenty-four of the thirty-four addresses that wave
//! 98 recorded as skipped and brings the file to **251** translated bodies. It
//! adds the angle/logarithm kernels at `0x80237A60..0x80237ED0`, the 3x4
//! transform and plane builder at `0x80238200`/`0x802385D0`, the box classifier
//! `0x80239780`, the two ray-shape clips `0x80239380`/`0x802394E0`, the whole
//! key-sampling band `0x8023F000..0x8023FE50`, the glyph blitter `0x802337B0`,
//! the exception report sink `0x80234A80`, and the four console queue entry
//! points `0x80236230`/`0x802365E0`/`0x80236FC0`/`0x80237370`.
//!
//! The read-back objection wave 98 raised is answered with a scratch model:
//! every `psq_st`/`psq_l` pair is reproduced by [`psq_round`], which stores into
//! a four-byte slot and reloads it, so the `(bits, value)` pair the export gets
//! from its stack scratch is available without guessing the undefined bytes.
//! Where the original genuinely depends on uninitialised scratch bytes (the
//! 8-bit lane of a 16-bit read), the lane the store actually wrote is used and
//! the doc block says so. Fifteen of the new bodies are `medium`; nine are
//! `low` because a `va_list` descriptor, an FPR argument block, the hardware
//! `fsqrt` or an asymmetric export branch had to be modelled. The ten remaining
//! addresses are listed in
//! `docs/COVERAGE_WAVE100_BRES.md` with the reason each stayed untranslated.
//!
//! The wave-106 group takes five of those ten and brings the file to **256**
//! translated bodies. It adds the two text-anchor twins `0x80230280`/
//! `0x80232C10`, the console layout engine `0x80231CF0`, the per-run renderer
//! `0x802323E0` and the console frame pump `0x802348F0`. Two decoding facts
//! made them possible:
//!
//! * the Metrowerks prologue stubs (`FUN_802DCD58`..`FUN_802DCD74`, really
//!   `_savegpr0_23`..`_savegpr0_27`) hand Ghidra their pair with `r4` in the
//!   low word and `r3` in the high word. Both anchor twins therefore *do*
//!   receive the `(stream, &x, &y, start, count)` signature their call sites
//!   show; the export's apparent `float *` arguments were that swap;
//! * `crate::platform::call_u64` exposes the 64-bit return of `0x801B5F80`, so
//!   the frame pump's millisecond pacing can be written without touching the
//!   shared helper layer. Its 64-bit divide is the `__div2u` runtime helper,
//!   replaced by native `u64` division.
//!
//! The remaining five (`0x80235AE0`, `0x802382B0`, `0x80238930`, `0x802389F0`,
//! `0x802390D0`) were left by wave 106 for the reasons recorded in
//! `docs/COVERAGE_WAVE106_BRES.md`: one reads its two source pointers out of
//! the double-argument save area, two need more than four `f64` arguments at
//! their call sites, one is a 146-line parser whose three calls lost their
//! argument lists, and one is 839 lines of reused `psq` scratch lanes.
//!
//! The wave-109 group takes two of those five and brings the file to **258**
//! translated bodies, leaving three:
//!
//! * `0x80235AE0` is the parser behind [`fn_80235dc0`]'s `SCRIPT_PARSE` call.
//!   Its first two arguments are the incoming `r3:r4` pair of the
//!   `_savegpr0_23` stub `0x802DCD64` - the stub is not a call - and the two
//!   argument lists `0x80235A10` lost are the token addresses `0x80235940`
//!   returns on the line above each. With those recovered the body is a
//!   section/entry search over the block table at `+0x10` plus the line copy
//!   the export printed after its `return`.
//! * `0x80238930` is a 180-byte wrapper: it turns the caller's angle into
//!   radians, halves it through `r2 - 0x4BA0` (`f32` 0.5, not 1.0), takes the
//!   Metrowerks `tan` at `0x802E8590`, and hands four signed products plus the
//!   scale to the matrix builder `0x802389F0`. Both callees take
//!   double-register arguments, which the two local call helpers in the
//!   wave-109 section express as `extern "C"` pointers.
//!
//! The three that remained after wave 109 (`0x802382B0`, `0x802389F0`,
//! `0x802390D0`) are listed in `docs/COVERAGE_WAVE109_BRES.md` with the reason
//! each stayed untranslated.
//!
//! The wave-111 group takes `0x802382B0` and brings the file to **259**
//! translated bodies. The raw instructions at `0x802382B0..0x80238418` show that
//! the export's `unaff_GQR0`/`unaff_GQR3` branches are the compiler's paired
//! single quantisation: `psq_l d(rA),0,0` reads two floats and `...,1,0` reads
//! one, while `psq_st ...,0,3` writes the scratch value whose low byte indexes
//! the 256-entry table at `0x802F74A0`. The body is therefore the standard
//! `PSMTX34RotXYZ` builder: three signed sines, three cosines and a row-major
//! 3x4 product. The export scrambles the result variables (`dVar18` is printed
//! as `dVar14 - dVar14`, i.e. zero, for element `[8]`), but the store at
//! `0x80238404` merges that zero with `-sin_y`; the reconstruction keeps
//! `-sin_y`, which is also the value the sibling builder `0x802A0A90` writes.
//! `0x802389F0` and `0x802390D0` stay untranslated for the reasons recorded in
//! `docs/COVERAGE_WAVE111_BRES.md`.
//!
//! The wave-112 group takes `0x802390D0` and brings the file to **260**
//! translated bodies. Re-reading its raw words shows that every `psq_l ...,0`
//! loads two floats and every `...,1` loads one, so the export's `ldexpf` /
//! `CONCAT` blocks are only its usual quantisation noise and the body is the
//! textbook closest-point-between-two-segments kernel: five dot products, a
//! denominator `a*e - b*b`, three `s` clamps and three `t` clamps, then the
//! squared distance between the two closest points. `param_3`/`param_4` are the
//! optional `s`/`t` outputs. `0x802389F0` (1 752 bytes) remained untranslated
//! until wave 115; `docs/COVERAGE_WAVE112_BRES.md` records why it waited.
//!
//! The wave-115 group takes `0x802389F0` and brings the file to **261**
//! translated bodies. It is the 1 752-byte beam / shadow-volume record builder
//! documented in the wave-115 section at the end: one matrix inverse, a matrix
//! copy, 24 corner stores, four plane builds, an eight-iteration transform loop
//! and the axis-aligned box around the eight rebuilt corners.

use crate::platform;
use crate::game::bases::{interface_call_desc, Descriptor};

// ===========================================================================
// U8 / ARC constants
// ===========================================================================

/// Magic at offset 0 of a U8 archive: ASCII `U`, `0xAA`, `8`, `-`.
const U8_MAGIC: u32 = 0x55AA_382D;
/// U8 header: offset of the 12-byte node table (relative to the image base).
const U8_HEADER_NODES: usize = 0x04;
/// U8 header: header size; equal to the data offset in every shipped archive.
const U8_HEADER_DATA: usize = 0x08;
/// U8 header: offset of the NUL-terminated name string table.
const U8_HEADER_STRINGS: usize = 0x0C;

/// In-memory U8 handle (built by [`arc_init_handle`]): archive base.
const ARC_BASE: usize = 0x00;
/// U8 handle: node table pointer.
const ARC_NODES: usize = 0x04;
/// U8 handle: string table pointer.
const ARC_STRINGS: usize = 0x08;
/// U8 handle: node count.
const ARC_NODE_COUNT: usize = 0x0C;
/// U8 handle: one past the node table.
const ARC_NODES_END: usize = 0x10;
/// U8 handle: data offset relative to [`ARC_BASE`].
const ARC_DATA_OFFSET: usize = 0x14;
/// U8 handle: current directory node index (0 = root).
const ARC_CURRENT_DIR: usize = 0x18;
/// Size of the U8 handle written by [`arc_init_handle`].
const ARC_HANDLE_SIZE: usize = 0x1C;

/// U8 node record size.
const U8_NODE_SIZE: usize = 0x0C;
/// U8 node: top byte flags (0 = file, 1 = directory) + low 24-bit name offset.
const U8_NODE_FLAGS_NAME: usize = 0x00;
/// U8 node: data offset for a file, parent node index for a directory.
const U8_NODE_DATA_OR_PARENT: usize = 0x04;
/// U8 node: size for a file, one-past-last-child index for a directory.
const U8_NODE_SIZE_OR_NEXT: usize = 0x08;
/// U8 node flags: any nonzero top byte means "directory".
const U8_DIRECTORY_MASK: u32 = 0xFF00_0000;
/// U8 node: name offset mask (low 24 bits).
const U8_NAME_MASK: u32 = 0x00FF_FFFF;

/// ARC file-info block: U8 handle that owns the file.
const ARC_FILE_ARCHIVE: usize = 0x00;
/// ARC file-info block: member data offset relative to the archive base.
const ARC_FILE_OFFSET: usize = 0x04;
/// ARC file-info block: member size in bytes.
const ARC_FILE_SIZE: usize = 0x08;
/// Size written by [`arc_open`] and [`arc_file_info_by_index`].
const ARC_FILE_INFO_SIZE: usize = 0x0C;

/// Directory iterator: U8 handle.
const ARC_DIR_ARCHIVE: usize = 0x00;
/// Directory iterator: directory node index.
const ARC_DIR_INDEX: usize = 0x04;
/// Directory iterator: next candidate node index.
const ARC_DIR_CURSOR: usize = 0x08;
/// Directory iterator: first node index outside this directory.
const ARC_DIR_END: usize = 0x0C;
/// Size of the iterator block.
const ARC_DIR_ITER_SIZE: usize = 0x10;

/// Directory entry: U8 handle.
const ARC_ENTRY_ARCHIVE: usize = 0x00;
/// Directory entry: node index.
const ARC_ENTRY_INDEX: usize = 0x04;
/// Directory entry: 1 when the node is a directory, 0 for a file.
const ARC_ENTRY_IS_DIR: usize = 0x08;
/// Directory entry: pointer to the NUL-terminated member name.
const ARC_ENTRY_NAME: usize = 0x0C;
/// Size of the entry block filled by [`arc_read_dir`].
const ARC_ENTRY_SIZE: usize = 0x10;

// ===========================================================================
// NW4R archive-object constants
// ===========================================================================

/// Archive object: vtable pointer (`0x8034FBD0` for this class).
const RES_ARCHIVE_VTABLE: usize = 0x00;
/// Archive object: mode word. `1` means the embedded U8 handle is valid.
///
/// `// unknown:` no value other than `1` is observed in this loader path; the
/// two stores are `0` (not mounted) and `1` (mounted).
const RES_ARCHIVE_MODE: usize = 0x10;
/// Archive object: reference count.
const RES_ARCHIVE_REFS: usize = 0x14;
/// Archive object: embedded U8 handle (`ARC_HANDLE_SIZE` bytes).
const RES_ARCHIVE_HANDLE: usize = 0x18;
/// Archive object allocation size.
const RES_ARCHIVE_SIZE: usize = 0x3C;
/// Node offset inside the archive object for the global NW4R list.
const RES_ARCHIVE_LIST_OFFSET: u16 = 0x34;

/// Game resource table: slot stride.
const RES_SLOT_STRIDE: usize = 0x40;
/// Game resource slot: nonzero `u16` means the slot is occupied.
const RES_SLOT_STATE: usize = 0x20;
/// Game resource slot: mounted NW4R archive object.
const RES_SLOT_ARCHIVE: usize = 0x28;

// ===========================================================================
// BRRES constants
// ===========================================================================

/// Offset of the BRRES root name dictionary from the first byte of the file.
///
/// `// observed:` the root section starts at file `+0x10`; its dictionary
/// begins eight bytes later, so the helpers are handed `file + 0x18`.
const BRRES_ROOT_DICT: usize = 0x18;

/// Dictionary header: `u32` number of resource entries (node 0 excluded).
const RES_DICT_COUNT: usize = 0x04;
/// Dictionary header: `u16` index of the first node after the sentinel.
///
/// The tree walk in [`res_dict_find`] starts here. Node 0 is the sentinel at
/// dictionary `+0x08`; node `n` starts at `dict + 0x08 + n * 0x10`.
const RES_DICT_ROOT: usize = 0x0C;
/// Dictionary node size.
const RES_DICT_NODE: usize = 0x10;
/// Node field: `u16` hash / bit index.
const RES_DICT_HASH: usize = 0x00;
/// Node field: `u16` left child node index.
const RES_DICT_LEFT: usize = 0x04;
/// Node field: `u16` right child node index.
const RES_DICT_RIGHT: usize = 0x06;
/// Node field: `u32` offset from the dictionary base to the name string.
///
/// `// observed:` the offset points at the first character, not at the `u32`
/// length word that precedes it; [`res_dict_find`] subtracts four to compare
/// the length-prefixed record.
const RES_DICT_NAME: usize = 0x08;
/// Node field: `u32` offset from the dictionary base to the resource body.
const RES_DICT_DATA: usize = 0x0C;
/// Number of bytes in the length word before a dictionary name.
const RES_NAME_LENGTH_WORD: usize = 0x04;
/// Section resource table: index `i` uses node `i + 1`; the node data word is
/// read at `section + (i + 1) * 0x10 + 0x14`.
const RES_SECTION_ENTRY_BASE: usize = 0x14;
/// Only the low nibble matters in the signed index correction.
const RES_INDEX_MASK: u32 = 0x0F;

/// `0x8034DF80` - length-prefixed section name `Palettes(NW4R)`.
const BRRES_TYPE_PALETTES: u32 = 0x8034_DF80;
/// `0x8034DFA0` - length-prefixed section name `Textures(NW4R)`.
const BRRES_TYPE_TEXTURES: u32 = 0x8034_DFA0;
/// `0x8034DFC0` - length-prefixed section name `AnmChr(NW4R)`.
const BRRES_TYPE_ANM_CHR: u32 = 0x8034_DFC0;
/// `0x8034DFE0` - length-prefixed section name `AnmVis(NW4R)`.
const BRRES_TYPE_ANM_VIS: u32 = 0x8034_DFE0;
/// `0x8034E000` - length-prefixed section name `AnmClr(NW4R)`.
const BRRES_TYPE_ANM_CLR: u32 = 0x8034_E000;
/// `0x8034E020` - length-prefixed section name `AnmTexPat(NW4R)`.
const BRRES_TYPE_ANM_TEX_PAT: u32 = 0x8034_E020;
/// `0x8034E040` - length-prefixed section name `AnmTexSrt(NW4R)`.
const BRRES_TYPE_ANM_TEX_SRT: u32 = 0x8034_E040;
/// `0x8034E060` - length-prefixed section name `AnmShp(NW4R)`.
const BRRES_TYPE_ANM_SHP: u32 = 0x8034_E060;
/// `0x8034E080` - length-prefixed section name `AnmScn(NW4R)`.
const BRRES_TYPE_ANM_SCN: u32 = 0x8034_E080;

// MDL0 payload dictionary offsets, relative to the first byte of the model.
//
// `// observed:` the model body begins with `MDL0`, size and revision; the
// words below it point at nested dictionaries. The names in `model.brres`
// from `MovieDemo/Baloon/g3d/model.brres` were used to identify each one.
/// MDL0 header: offset to the node-tree dictionary (`NodeTree`, `DrawOpa`).
const MDL_NODE_TREE: usize = 0x10;
/// MDL0 header: offset to the bone/node dictionary (`skl_root`, ...).
const MDL_NODES: usize = 0x14;
/// MDL0 header: offset to the material dictionary (`mat_*`, ...).
const MDL_MATERIALS: usize = 0x18;
/// MDL0 header: offset to the shader dictionary (keys are material names).
const MDL_SHADERS: usize = 0x1C;
/// MDL0 header: offset to the texture-reference dictionary (keys are material
/// names; every entry owns an array of TEX0 references).
const MDL_TEXTURES: usize = 0x20;
/// MDL0 header: offset to the palette dictionary (keys `#0`, `#1`, ... in the
/// sampled `MovieDemo/Baloon` model).
const MDL_PALETTES: usize = 0x24;
/// MDL0 header: offset to the dictionary at `+0x28`.
///
/// `// unknown:` zero in every sampled model. The translated
/// [`mdl_model_bind`] still walks it with the same per-entry cache-store loop
/// as the populated payload dictionaries.
const MDL_SECTION_28: usize = 0x28;
/// MDL0 header: offset to the dictionary at `+0x2C`.
///
/// `// unknown:` zero in every sampled model; the accessor at `0x8023B700`
/// only proves that it is read with the same indexed-entry formula.
const MDL_SECTION_2C: usize = 0x2C;
/// MDL0 header: offset to the dictionary at `+0x30`.
///
/// `// observed:` keys are the short material names (`mat_balloon_ropebolt`,
/// ...) in `MovieDemo/Baloon/g3d/model.brres`; each entry owns a table of
/// 0x34-byte records. `// unknown:` the G3D-side name of this section.
const MDL_SECTION_30: usize = 0x30;
/// MDL0 header: offset to the polygon dictionary (`polygon0`..`polygon5`).
const MDL_POLYGONS: usize = 0x38;

// MDL0 `+0x30` entry layout, used by the deferred bind pass.
/// Entry word: number of 0x34-byte material records.
const MDL_ENTRY_MATERIAL_COUNT: usize = 0x2C;
/// Entry word: offset (from the entry base) to the material-record table.
const MDL_ENTRY_MATERIAL_TABLE: usize = 0x30;
/// Entry word: address of the texture bitmask used while binding.
const MDL_ENTRY_TEX_BITSET: usize = 0x40;
/// Entry word: address of the palette bitmask used while binding.
const MDL_ENTRY_PLT_BITSET: usize = 0x144;
/// Distance between two material records inside one `+0x30` entry.
const MDL_MATERIAL_STRIDE: usize = 0x34;

// `AnmTexPat` object layout used by `0x8023F720` (all offsets relative to the
// animation object). `// inferred:` names come from the two lookup calls: the
// first table resolves TEX0 records, the second resolves palette records.
/// Animation object: offset to the texture-name-offset table.
const ANM_TEX_NAMES: usize = 0x14;
/// Animation object: offset to the palette-name-offset table.
const ANM_PLT_NAMES: usize = 0x18;
/// Animation object: offset to the cached TEX0-pointer table.
const ANM_TEX_SLOTS: usize = 0x1C;
/// Animation object: offset to the cached palette-pointer table.
const ANM_PLT_SLOTS: usize = 0x20;
/// Animation object: `u16` texture-name count.
const ANM_TEX_COUNT: usize = 0x34;
/// Animation object: `u16` palette-name count.
const ANM_PLT_COUNT: usize = 0x36;

// ===========================================================================
// Globals and SDA anchors
// ===========================================================================

/// PowerPC `r13`, set by `__init_registers` at `0x80004244`.
const SDA_BASE: usize = 0x8042_F680;
/// Global list of mounted NW4R archive objects (`DAT_80423860`).
const RES_ARCHIVE_LIST: usize = 0x8042_3860;
/// "List has been initialised" byte, `r13 - 0x4888`.
const RES_ARCHIVE_LIST_READY: usize = SDA_BASE - 0x4888;
/// Static string `/`, `r13 - 0x5B70`; used as the enumeration root.
const ROOT_PATH: usize = SDA_BASE - 0x5B70;

/// PowerPC `r2`, set by `__init_registers` at `0x8000423C`/`0x80004240`.
///
/// `// observed:` the two instructions load `0x8043` and add `0x3080`, so
/// `r2 = 0x80433080`. The wave-18 payload helpers read their constants through
/// negative `r2` offsets; the individual addresses below are those offsets
/// resolved against this base.
const SDA2_BASE: usize = 0x8043_3080;

/// `r2 - 0x4B70` - `f32` zero used as the fallback animation ratio.
const SDA2_ZERO_RATIO: usize = SDA2_BASE - 0x4B70;
/// `r2 - 0x4B68` - `f64` 0.6000000238418579, the easing exponent.
const SDA2_EASE_EXPONENT: usize = SDA2_BASE - 0x4B68;
/// `r2 - 0x4B38` - `f32` 1.0, written to the three channel components.
const SDA2_ONE: usize = SDA2_BASE - 0x4B38;
/// `r2 - 0x4B34` - `f32` 0.7111111283302307, the matrix scale factor.
const SDA2_MTX_SCALE: usize = SDA2_BASE - 0x4B34;
/// `r2 - 0x4B30` - `f32` 0.0, written to the three channel components.
const SDA2_ZERO: usize = SDA2_BASE - 0x4B30;

/// Pointer used by the C library `tolower` table lookup.
///
/// `// observed:` `0x80350E68` holds `0x80350CB8`; `+0x10` points at
/// `0x802FD6B8`, a 256-byte identity table whose `a`..`z` range is lowercase.
/// Reproduced as a runtime lookup rather than a hard-coded ASCII conversion.
const LOCALE_CTYPE_PTR: usize = 0x8035_0E68;

// ===========================================================================
// Callees outside this lane (called by absolute address)
// ===========================================================================

/// `0x8015F730` - variadic report/printf sink. `// inferred:` the two strings
/// in the format are consumed from `r4`/`r5`, matching `...'%s'...%s...`.
const OS_REPORT: usize = 0x8015_F730;
/// `0x801AD4E0` - fatal assert/panic: prints `in %s on line %d` and aborts.
const OS_PANIC: usize = 0x801A_D4E0;

/// `0x802DC69C` - `strlen`.
const STRLEN: usize = 0x802D_C69C;
/// `0x802E1A3C` - `strcat`.
const STRCAT: usize = 0x802E_1A3C;
/// `0x802E1AB4` - `strcmp`.
const STRCMP: usize = 0x802E_1AB4;
/// `0x802E1938` - `strcpy`.
const STRCPY: usize = 0x802E_1938;

/// `0x80228DD0` - NW4R list initialiser: `(list, node_offset)`.
///
/// `// inferred:` the `ut::List` identity comes from the intrusive-link shape
/// (two pointers at `node_offset`, a `u16` count at `+8`), not a symbol table.
const LIST_INIT: usize = 0x8022_8DD0;
/// `0x80228DF0` - NW4R list append: `(list, object)`.
///
/// `// inferred:` see [`LIST_INIT`].
const LIST_APPEND: usize = 0x8022_8DF0;
/// `0x80228F80` - NW4R list remove: `(list, object)`.
///
/// `// inferred:` see [`LIST_INIT`].
const LIST_REMOVE: usize = 0x8022_8F80;
/// `0x80228FF0` - NW4R list next: `(list, node_or_null)`.
///
/// `// inferred:` see [`LIST_INIT`].
const LIST_NEXT: usize = 0x8022_8FF0;

/// `0x800046B4` - `memset` wrapper.
const MEMSET: usize = 0x8000_46B4;
/// `0x802BA2D0` - base-class constructor for the archive object.
///
/// `// inferred:` it is called once at the start of [`fn_802b7380`] and writes
/// the shared base state before the derived vtable is installed.
const RES_ARCHIVE_BASE_CTOR: usize = 0x802B_A2D0;
/// `0x802B9240` - archive-object allocator: `(size, heap, flags)`.
///
/// `// inferred:` the routine forwards to `0x802B8CC0(size, flags, heap)`;
/// callers treat its result as a pointer and check it against 0.
const RES_ARCHIVE_ALLOC: usize = 0x802B_9240;

/// `0x801A0040` - closes a directory iterator (returns 1 in the shipped code).
const ARC_CLOSE_DIR: usize = 0x801A_0040;
/// `0x8019FE90` - closes a file-info block (returns 1 in the shipped code).
const ARC_CLOSE_FILE: usize = 0x8019_FE90;
/// `0x8019FE70` - file-info data offset word.
const ARC_FILE_OFFSET_GET: usize = 0x8019_FE70;
/// `0x8019FE80` - file-info size word.
const ARC_FILE_SIZE_GET: usize = 0x8019_FE80;

/// `0x8023C280` - build one polygon entry's GX display lists.
///
/// `// inferred:` [`mdl_model_bind`] calls it once per `+0x38` dictionary
/// entry; the callee resolves a material through the polygon's `+0x48` slot
/// and writes display-list commands.
const MDL_POLYGON_BUILD: usize = 0x8023_C280;

/// `0x8023EA20` - cache-publish one entry of the MDL0 `+0x18` dictionary.
///
/// `// unknown:` the Ghidra body passes the entry's first word as both the
/// data and size arguments of `0x801AC500`; the call is kept literally.
const MDL_SECTION_18_PUBLISH: usize = 0x8023_EA20;
/// `0x8023EA40` - cache-publish one entry of the MDL0 `+0x1C` dictionary.
///
/// `// unknown:` see [`MDL_SECTION_18_PUBLISH`].
const MDL_SECTION_1C_PUBLISH: usize = 0x8023_EA40;
/// `0x8023EA60` - cache-publish one entry of the MDL0 `+0x20` dictionary.
///
/// `// unknown:` see [`MDL_SECTION_18_PUBLISH`].
const MDL_SECTION_20_PUBLISH: usize = 0x8023_EA60;
/// `0x8023EA80` - cache-publish one entry of the MDL0 `+0x24` dictionary.
///
/// `// unknown:` see [`MDL_SECTION_18_PUBLISH`].
const MDL_SECTION_24_PUBLISH: usize = 0x8023_EA80;
/// `0x8023EAA0` - cache-publish one entry of the MDL0 `+0x28` dictionary.
///
/// `// unknown:` see [`MDL_SECTION_18_PUBLISH`].
const MDL_SECTION_28_PUBLISH: usize = 0x8023_EAA0;
/// `0x8023EAC0` - cache-publish one entry of the MDL0 `+0x2C` dictionary.
///
/// `// unknown:` see [`MDL_SECTION_18_PUBLISH`].
const MDL_SECTION_2C_PUBLISH: usize = 0x8023_EAC0;

/// `0x8023ECE0` - bind/register one TEX0 texture.
///
/// `// inferred:` called once per `Textures(NW4R)` entry by
/// [`brres_bind_all`].
const RES_TEXTURE_BIND: usize = 0x8023_ECE0;
/// `0x8023EAE0` - bind/register one palette.
///
/// `// inferred:` called once per `Palettes(NW4R)` entry by
/// [`brres_bind_all`]; the second argument is always 0 there.
const RES_PALETTE_BIND: usize = 0x8023_EAE0;
/// `0x8023ECB0` - validate one TEX0 texture entry.
///
/// `// inferred:` [`brres_check_revision`] calls it per texture and treats a
/// zero result as a bad revision.
const RES_TEXTURE_CHECK: usize = 0x8023_ECB0;
/// `0x8023EB00` - validate one palette entry.
///
/// `// inferred:` [`brres_check_revision`] calls it per palette and treats a
/// zero result as a bad revision.
const RES_PALETTE_CHECK: usize = 0x8023_EB00;

/// `0x8023E390` - bind one 0x34-byte material record.
///
/// `// inferred:` [`mdl_material_bind_check`] calls it per material. It resolves
/// the record's texture and palette name records through the BRRES
/// dictionaries and writes the resolved pointers plus GX display-list state.
/// It takes eight arguments; [`crate::platform`] only exposes `call0..call7`,
/// so it stays an address call.
const RES_MATERIAL_BIND: usize = 0x8023_E390;

/// `0x801AC4A0` - cache-block store loop followed by `syscall`.
///
/// `// unknown:` the `mode != 0` branch of [`res_palette_bind`] calls it with
/// no arguments in the Ghidra output.
const DC_STORE_SYNC: usize = 0x801A_C4A0;
/// `0x801AC4D0` - cache-block flush loop.
///
/// Used by [`res_texture_bind`] to publish the texture bytes.
const DC_FLUSH: usize = 0x801A_C4D0;
/// `0x801AC500` - cache-block store loop.
///
/// Used by [`res_palette_bind`] in the `mode == 0` branch.
const DC_STORE: usize = 0x801A_C500;

/// `0x801C4780` - write one 32-bit value to the GX FIFO at `0xCC008000`.
///
/// `// observed:` the Ghidra body is a single store of its third argument and
/// ignores the first two; call sites below still pass them to preserve the
/// original call shape.
const GX_FIFO_WRITE: usize = 0x801C_4780;
/// `0x8024A530` - copy whole 32-byte units, using 128-byte blocks.
///
/// `// observed:` it copies `size >> 5` 32-byte units and silently drops a
/// trailing partial unit.
const MEMCPY_BLOCKS_32: usize = 0x8024_A530;

/// `0x801C04D0` - write a 3x4 identity matrix into a 0x30-byte destination.
///
/// `// inferred:` every second argument of `0x801C0500` is a matrix-shaped
/// field (`+0xAC` in a 0x34-byte material record, `+0x1C` in the channel
/// state block), and this routine fills exactly that shape.
const PSMTX_IDENTITY: usize = 0x801C_04D0;
/// `0x801C0500` - copy a 3x4 matrix: `(src, dst)`.
///
/// `// inferred:` see [`PSMTX_IDENTITY`]; the wave-18 payload calls it with a
/// source matrix and a destination field.
const PSMTX_COPY: usize = 0x801C_0500;
/// `0x802382B0` - build a scaled 3x4 matrix from three doubles and a destination.
///
/// `// inferred:` its arguments are `f64, f64, f64, float*`; the three scalars
/// are the material scale fields times 0.7111111.
const MTX34_SCALE: usize = 0x8023_82B0;
/// `0x802E55A4` - `pow(base, exponent)` for `f64`.
///
/// `// observed:` Ghidra calls it through a thunk; the target body is
/// `802e55a4_FUN_802e55a4.c`.
const POW_F64: usize = 0x802E_55A4;
/// `0x80239980` - write one 5-byte GX BP command: `(dst, word)`.
///
/// `// observed:` the body stores `0x61` then the four big-endian bytes of the
/// second argument.
const GX_BP_WRITE: usize = 0x8023_9980;
/// `0x801C95E0` - write one GX FIFO word with the stream-state checks.
///
/// `// observed:` the body tests the stream object at `r2 - 0x4EF8`, flushes
/// if needed and stores the second argument at `0xCC008000`.
const GX_FIFO_WRITE_SYNC: usize = 0x801C_95E0;
/// `0x801A9B90` - bare `syscall` instruction.
///
/// `// observed:` the Ghidra body contains only `syscall`.
const SYSCALL: usize = 0x801A_9B90;
/// `0x802F84D0` - read-only table used by [`res_texcoord_command_emit`].
///
/// `// unknown:` the table's entries are not named; the accessor reads the
/// 32-bit word selected by a one-based record byte.
const TEXCOORD_COMMAND_TABLE: usize = 0x802F_84D0;

/// `0xCC008000` - GX write-gather FIFO.
///
/// `// observed:` the direct-store branch of the wave-70 publish helpers writes
/// one word here, exactly as Ghidra renders `_DAT_cc008000 = ...`.
const GX_FIFO: usize = 0xCC00_8000;
/// Cached-block record: base of the block-address computation (`record + 0x18`).
///
/// `// observed:` `0x8023C610` forms `record + 0x18 + *(record + 0x20)`.
const GX_BLOCK_BASE: usize = 0x18;
/// Cached-block record: block word count.
const GX_BLOCK_SIZE: usize = 0x1C;
/// Cached-block record: block offset from [`GX_BLOCK_BASE`]; 0 means "absent".
const GX_BLOCK_OFFSET: usize = 0x20;
/// Second cached-block record: base of the block-address computation.
const GX_AUX_BASE: usize = 0x24;
/// Second cached-block record: block word count.
const GX_AUX_SIZE: usize = 0x28;
/// Second cached-block record: block offset from [`GX_AUX_BASE`].
const GX_AUX_OFFSET: usize = 0x2C;
/// Byte count skipped by the `skip_header` path of [`mdl_buffer_word_emit`].
const GX_BLOCK_HEADER: u32 = 0x20;
/// Texture-object cache slot stride.
const RES_CACHE_TEXTURE_STRIDE: usize = 0x20;
/// Palette-object cache slot stride.
const RES_CACHE_PALETTE_STRIDE: usize = 0x0C;
/// Number of slots in either object cache.
const RES_CACHE_SLOTS: u32 = 8;
/// Cache block size for one or two populated object slots.
const RES_CACHE_BLOCK_20: u32 = 0x20;
/// Cache block size for three or four populated object slots.
const RES_CACHE_BLOCK_40: u32 = 0x40;
/// Cache block size for five to eight populated object slots.
const RES_CACHE_BLOCK_80: u32 = 0x80;
/// Cache block size for nine or more populated object slots.
const RES_CACHE_BLOCK_A0: u32 = 0xA0;
/// Byte offset of the first object inside an object-cache block (after the
/// 4-byte present bitset).
const RES_CACHE_FIRST_OBJECT: usize = 0x04;

/// `0x8034DF60` - length-prefixed section name `3DModels(NW4R)`.
const BRRES_TYPE_MODELS: u32 = 0x8034_DF60;

/// `0x804294A8` - source file name `arc.c`, passed to `OS_PANIC`.
const ARC_SOURCE_FILE: usize = 0x8042_94A8;
/// `0x8032A3E0` - `ARCInitHandle: bad archive format`.
const ARC_BAD_FORMAT: usize = 0x8032_A3E0;
/// `0x8032E404` - `Warning: ARCOpen(): file '%s' was not found under %s in the archive.`
const ARC_WARN_FORMAT: usize = 0x8032_E404;

/// Line in `arc.c` reported by the `ARCInitHandle` panic.
const ARC_INIT_LINE: u32 = 0x4A;

// ===========================================================================
// Game resource slots, 0x800DF570..0x800DF180
// ===========================================================================

/// Finds the game's resource slot for a name.
///
/// The table is a pair `{base, u16 count}`; slots are `0x40` bytes apart. A
/// slot is skipped while its `u16` at `+0x20` is zero, and is compared by the
/// C string at its first byte.
///
/// # Original
/// - Ghidra: `FUN_800df570` at `0x800DF570` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df570_FUN_800df570.c`
///
/// # Arguments
/// - `table`: address of `{u32 base, u16 count}` (count read at `+4`).
/// - `name`: address of the NUL-terminated name to match.
///
/// # Returns
/// Address of the matching 0x40-byte slot, or 0 when none matches.
///
/// # Side effects
/// None; this is a read-only scan.
///
/// # Safety
/// `table` must point to a readable pair, the slot array must be readable for
/// `count * 0x40` bytes, and `name` must be a readable C string.
///
/// # Confidence
/// high - stride, state word and comparison are explicit. medium for the
/// meaning of the state word (`// inferred:` occupied/live flag).
pub unsafe fn fn_800df570(table: usize, name: usize) -> u32 {
    let base = platform::read_u32(table) as usize;
    let count = platform::read_u16(table + 4) as u32;
    let mut index = 0u32;
    while index < count {
        let slot = base + index as usize * RES_SLOT_STRIDE;
        if platform::read_u16(slot + RES_SLOT_STATE) != 0
            && platform::call2(STRCMP, name as u32, slot as u32) == 0
        {
            return slot as u32;
        }
        index += 1;
    }
    0
}

/// Returns the resource slot only when it is already backed by an archive.
///
/// # Original
/// - Ghidra: `FUN_800df640` at `0x800DF640` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df640_FUN_800df640.c`
/// - `// inferred:` Ghidra shows no parameters because the two arguments are
///   still live in `r3`/`r4` from the caller; the decompiled body passes them
///   to `FUN_800df570`. Callers (e.g. `0x800DF180`) pass `(table, name)`.
///
/// # Arguments
/// - `table`: resource-slot table, as in [`fn_800df570`].
/// - `name`: slot name, as in [`fn_800df570`].
///
/// # Returns
/// The slot address when it exists **and** `slot+0x28` is nonzero; otherwise 0.
///
/// # Side effects
/// None.
///
/// # Safety
/// Same requirements as [`fn_800df570`].
///
/// # Confidence
/// medium - control flow is exact; the argument pass-through is inferred from
/// the caller because Ghidra dropped the parameters.
pub unsafe fn fn_800df640(table: usize, name: usize) -> u32 {
    let slot = fn_800df570(table, name);
    if slot == 0 {
        return 0;
    }
    if platform::read_u32(slot as usize + RES_SLOT_ARCHIVE) == 0 {
        return 0;
    }
    slot
}

/// Loads a BRRES member from the archive attached to a named resource slot.
///
/// This is the game-side convenience wrapper used by actor loaders. The first
/// two arguments select the slot; only the third is the file path inside the
/// archive. `// observed:` the C body does not touch `param_1` or `param_2`
/// directly - they are forwarded to `0x800DF640`, whose Ghidra signature lost
/// its arguments.
///
/// # Original
/// - Ghidra: `FUN_800df180` at `0x800DF180` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/800df180_FUN_800df180.c`
///
/// # Arguments
/// - `table`: resource-slot table (see [`fn_800df570`]).
/// - `name`: slot name; e.g. `daikonbou`.
/// - `path`: member path inside the archive; e.g. `g3d/daikonbou.brres`.
///
/// # Returns
/// Pointer to the first byte of the BRRES member, or 0 when the slot, archive
/// or member is missing.
///
/// # Side effects
/// None directly. [`fn_802b76c0`] opens the U8 member and may call the logging
/// sink on a miss.
///
/// # Safety
/// `table`/`name` must satisfy [`fn_800df570`]; `path` must be a readable C
/// string; the returned pointer is only valid while its archive stays mounted.
///
/// # Confidence
/// high - flow is explicit and the slot-to-archive hand-off is visible.
pub unsafe fn fn_800df180(table: usize, name: usize, path: usize) -> u32 {
    let slot = fn_800df640(table, name);
    if slot != 0 {
        let archive = platform::read_u32(slot as usize + RES_SLOT_ARCHIVE);
        if archive != 0 {
            return fn_802b76c0(archive as usize, path, 0);
        }
    }
    0
}

// ===========================================================================
// U8 / ARC reader, 0x8019F660..0x8019FF80
// ===========================================================================

/// Initialises a U8 archive handle over an in-memory archive image.
///
/// The magic is the only validation: a mismatch calls `OS_PANIC` with
/// `arc.c`, line 74 and the string at `ARC_BAD_FORMAT`. The handle's
/// `current directory` starts at the root (0).
///
/// # Original
/// - Ghidra: `FUN_8019f660` at `0x8019F660` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019f660_FUN_8019f660.c`
/// - Strings: `ARCInitHandle: bad archive format` at `0x8032A3E0`.
///
/// # Arguments
/// - `data`: address of the first byte of the `.arc` image.
/// - `handle`: address of a writable `ARC_HANDLE_SIZE`-byte handle.
///
/// # Returns
/// Always `1` on the non-panicking path.
///
/// # Side effects
/// Writes all seven words of `handle`; may abort through `OS_PANIC`.
///
/// # Safety
/// `data` must be a readable U8 image, `handle` writable for 0x1C bytes, and
/// the header's node/string tables must be in range.
///
/// # Confidence
/// high - magic, offsets and the panic arguments are explicit.
pub unsafe fn arc_init_handle(data: usize, handle: usize) -> u32 {
    if platform::read_u32(data) != U8_MAGIC {
        platform::call3(
            OS_PANIC,
            ARC_SOURCE_FILE as u32,
            ARC_INIT_LINE,
            ARC_BAD_FORMAT as u32,
        );
    }

    let nodes = data + platform::read_u32(data + U8_HEADER_NODES) as usize;
    let strings = data + platform::read_u32(data + U8_HEADER_STRINGS) as usize;
    let count = platform::read_u32(nodes + U8_NODE_SIZE_OR_NEXT);

    platform::write_u32(handle + ARC_BASE, data as u32);
    platform::write_u32(handle + ARC_NODES, nodes as u32);
    platform::write_u32(handle + ARC_STRINGS, strings as u32);
    platform::write_u32(handle + ARC_NODE_COUNT, count);
    platform::write_u32(
        handle + ARC_NODES_END,
        (nodes + count as usize * U8_NODE_SIZE) as u32,
    );
    platform::write_u32(
        handle + ARC_DATA_OFFSET,
        platform::read_u32(data + U8_HEADER_DATA),
    );
    platform::write_u32(handle + ARC_CURRENT_DIR, 0);
    1
}

/// Fills a file-info block from a node index, rejecting directories.
///
/// # Original
/// - Ghidra: `FUN_8019f9b0` at `0x8019F9B0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019f9b0_FUN_8019f9b0.c`
///
/// # Arguments
/// - `handle`: U8 handle from [`arc_init_handle`].
/// - `index`: node index; interpreted as signed for the `< 0` rejection.
/// - `info`: writable `ARC_FILE_INFO_SIZE`-byte block.
///
/// # Returns
/// `1` when `index` is in range and names a file; `0` for an out-of-range
/// index or a directory.
///
/// # Side effects
/// On success writes `info[0] = handle`, `info[1] = node+4`, `info[2] = node+8`.
///
/// # Safety
/// `handle` must be a valid U8 handle and `info` writable.
///
/// # Confidence
/// high - the range check, directory mask and three writes are explicit.
pub unsafe fn arc_file_info_by_index(handle: usize, index: u32, info: usize) -> u32 {
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    if (index as i32) >= 0
        && index < platform::read_u32(handle + ARC_NODE_COUNT)
        && platform::read_u32(nodes + index as usize * U8_NODE_SIZE) & U8_DIRECTORY_MASK == 0
    {
        let node = nodes + index as usize * U8_NODE_SIZE;
        platform::write_u32(info + ARC_FILE_ARCHIVE, handle as u32);
        platform::write_u32(
            info + ARC_FILE_OFFSET,
            platform::read_u32(node + U8_NODE_DATA_OR_PARENT),
        );
        platform::write_u32(
            info + ARC_FILE_SIZE,
            platform::read_u32(node + U8_NODE_SIZE_OR_NEXT),
        );
        return 1;
    }
    0
}

/// Resolves a `/`-separated path to a U8 node index.
///
/// The walk starts at the handle's current directory. Every leading `/`
/// resets it to the root; `.` stays put and `..` moves to the node's parent
/// word. Names are compared case-insensitively through the C library
/// `tolower` table at `LOCALE_CTYPE_PTR`.
///
/// Directories do not store a child count. Instead, a directory node's `+0x08`
/// word is the first node index **after** its children, so its children are
/// the contiguous range `index+1 .. next-1`. The loop skips a whole directory
/// subtree by jumping to `next`, and skips a file by `+1`.
///
/// # Original
/// - Ghidra: `FUN_8019fa00` at `0x8019FA00` (624 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019fa00_FUN_8019fa00.c`
///
/// # Arguments
/// - `handle`: U8 handle from [`arc_init_handle`].
/// - `path`: NUL-terminated path; either relative to the current directory or
///   absolute (leading `/`).
///
/// # Returns
/// The node index, or `0xFFFF_FFFF` when a component is not found.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The handle and all node/name reads must be in range; `path` must be a
/// readable C string.
///
/// # Confidence
/// high - every branch, the sibling-range rule and the case-folding are
/// explicit. medium for the exact `tolower` table indirection, which is
/// verified against the image bytes but still a C-library detail.
pub unsafe fn arc_find_index(handle: usize, path: usize) -> u32 {
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    let strings = platform::read_u32(handle + ARC_STRINGS) as usize;
    let mut index = platform::read_u32(handle + ARC_CURRENT_DIR);
    let mut cursor = path;

    'component: loop {
        // observed: each leading '/' resets the walk to the root.
        loop {
            let ch = platform::read_u8(cursor);
            if ch == 0 {
                return index;
            }
            if ch != b'/' {
                break;
            }
            index = 0;
            cursor += 1;
        }

        // observed: "." and ".." are handled before the ordinary name scan.
        if platform::read_u8(cursor) == b'.' {
            let second = platform::read_u8(cursor + 1);
            if second == b'.' {
                let third = platform::read_u8(cursor + 2);
                if third == b'/' {
                    cursor += 3;
                    index = platform::read_u32(
                        nodes + index as usize * U8_NODE_SIZE + U8_NODE_DATA_OR_PARENT,
                    );
                    continue 'component;
                }
                if third == 0 {
                    return platform::read_u32(
                        nodes + index as usize * U8_NODE_SIZE + U8_NODE_DATA_OR_PARENT,
                    );
                }
            } else if second == b'/' {
                cursor += 2;
                continue 'component;
            } else if second == 0 {
                return index;
            }
        }

        // The component ends at '/' (more path follows) or at NUL.
        let mut scan = cursor;
        let mut more = false;
        while platform::read_u8(scan) != 0 {
            if platform::read_u8(scan) == b'/' {
                more = true;
                break;
            }
            scan += 1;
        }

        let directory = index;
        let directory_node = nodes + directory as usize * U8_NODE_SIZE;
        let mut candidate = directory + 1;

        loop {
            // observed: the directory's next word bounds its child range.
            if platform::read_u32(directory_node + U8_NODE_SIZE_OR_NEXT) <= candidate {
                return 0xFFFF_FFFF;
            }

            let node = nodes + candidate as usize * U8_NODE_SIZE;
            let flags = platform::read_u32(node + U8_NODE_FLAGS_NAME);
            let is_file = flags & U8_DIRECTORY_MASK == 0;
            let name = strings + (flags & U8_NAME_MASK) as usize;

            // observed: a file cannot satisfy a path that still has components.
            let is_dot = platform::read_u8(name) == b'.' && platform::read_u8(name + 1) == 0;
            if !(is_file && more) && !is_dot {
                if arc_name_matches(name, cursor) {
                    if !more {
                        return candidate;
                    }
                    cursor = scan + 1;
                    index = candidate;
                    continue 'component;
                }
            } else if is_dot {
                candidate += 1;
                continue;
            }

            candidate = if is_file {
                candidate + 1
            } else {
                platform::read_u32(node + U8_NODE_SIZE_OR_NEXT)
            };
        }
    }
}

/// Compares a U8 member name with one path component, case-insensitively.
///
/// A match ends when the member name reaches NUL and the path is at `/` or
/// NUL; a mismatch ends at the first differing (lower-cased) byte.
///
/// # Original
/// - Not a separate Ghidra function: this is the inlined `do`/`while` loop in
///   `FUN_8019fa00` at `0x8019FA00`.
/// - Evidence: `work/ghidra/main-gb/export/8019fa00_FUN_8019fa00.c`
///
/// # Safety
/// Both pointers must be readable C strings / path components.
///
/// # Confidence
/// high - the loop condition and terminator test are literal.
unsafe fn arc_name_matches(name: usize, path: usize) -> bool {
    let mut name_ptr = name;
    let mut path_ptr = path;
    loop {
        let ch = platform::read_u8(name_ptr);
        if ch == 0 {
            let end = platform::read_u8(path_ptr);
            return end == b'/' || end == 0;
        }
        name_ptr += 1;
        let a = arc_tolower(ch);
        let b = arc_tolower(platform::read_u8(path_ptr));
        path_ptr += 1;
        if a != b {
            return false;
        }
    }
}

/// Lower-cases one byte through the C library `ctype` table.
///
/// `// observed:` the original reads
/// `*(u8 *)(*(u32 *)(PTR_DAT_80350e68 + 0x10) + byte)`. The first load yields
/// `0x80350CB8`, the second `0x802FD6B8`, and that table maps `A`..`Z` to
/// `a`..`z` while leaving other bytes unchanged.
///
/// # Safety
/// The locale table must remain readable; it is static in the shipped image.
///
/// # Confidence
/// high - the indirection was followed in `work/dol/main.elf`.
unsafe fn arc_tolower(byte: u8) -> u8 {
    let locale = platform::read_u32(LOCALE_CTYPE_PTR) as usize;
    let table = platform::read_u32(locale + 0x10) as usize;
    platform::read_u8(table + byte as usize)
}

/// Finds a member by path and fills an ARC file-info block.
///
/// On a hit, the block receives the handle, the member's data offset and its
/// size. A directory match is treated as a failure. On a miss the routine
/// builds the current-directory path and reports:
///
/// `Warning: ARCOpen(): file '%s' was not found under %s in the archive.`
///
/// # Original
/// - Ghidra: `FUN_8019f700` at `0x8019F700` (672 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019f700_FUN_8019f700.c`
/// - String: `0x8032E404`.
///
/// # Arguments
/// - `handle`: U8 handle from [`arc_init_handle`].
/// - `path`: NUL-terminated member path.
/// - `info`: writable `ARC_FILE_INFO_SIZE`-byte block.
///
/// # Returns
/// `1` on a file hit, `0` for a directory or a miss.
///
/// # Side effects
/// Writes `info` on a hit; on a miss builds a path in a 128-byte stack buffer
/// and calls `OS_REPORT`. The warning path is diagnostic only.
///
/// # Safety
/// `handle`, `path` and `info` must be valid; the built path is bounded to
/// 128 bytes (the original writes the terminator at index 0x7F).
///
/// # Confidence
/// high - the hit path, directory rejection and warning string are explicit.
pub unsafe fn arc_open(handle: usize, path: usize, info: usize) -> u32 {
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    let index = arc_find_index(handle, path);

    if (index as i32) >= 0 {
        let node = nodes + index as usize * U8_NODE_SIZE;
        if platform::read_u32(node + U8_NODE_FLAGS_NAME) & U8_DIRECTORY_MASK == 0 {
            platform::write_u32(info + ARC_FILE_ARCHIVE, handle as u32);
            platform::write_u32(
                info + ARC_FILE_OFFSET,
                platform::read_u32(node + U8_NODE_DATA_OR_PARENT),
            );
            platform::write_u32(
                info + ARC_FILE_SIZE,
                platform::read_u32(node + U8_NODE_SIZE_OR_NEXT),
            );
            return 1;
        }
        return 0;
    }

    // observed: 128-byte buffer (`local_98[127]` plus the terminator byte).
    let mut buffer = crate::platform::GuestLocal::new( [0u8; 128]);
    let base = buffer.as_mut_ptr() as usize;
    arc_warning_dir(handle, base);

    platform::call3(
        OS_REPORT,
        ARC_WARN_FORMAT as u32,
        path as u32,
        base as u32,
    );
    0
}

/// Builds the current-directory path of a U8 handle for [`arc_open`]'s warning.
///
/// The original inlines four levels: recursion through [`arc_build_dir_path`]
/// for the great-grandparent, then appends the grandparent, parent and current
/// directory names with `/` separators. When the current node is a directory
/// it also appends a trailing `/`, so the diagnostic reads "under dir/".
///
/// # Original
/// - Inlined in `FUN_8019f700` at `0x8019F700`.
/// - Evidence: `work/ghidra/main-gb/export/8019f700_FUN_8019f700.c`
///
/// # Safety
/// `out` must be writable for at least 128 bytes.
///
/// # Confidence
/// high - the four-level structure and the 0x7F/0x80 boundary checks are
/// explicit.
unsafe fn arc_warning_dir(handle: usize, out: usize) -> u32 {
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    let strings = platform::read_u32(handle + ARC_STRINGS) as usize;
    let directory = platform::read_u32(handle + ARC_CURRENT_DIR);
    let mut len = 0u32;

    if directory != 0 {
        let current = nodes + directory as usize * U8_NODE_SIZE;
        let parent_index = platform::read_u32(current + U8_NODE_DATA_OR_PARENT);
        let current_name = strings + (platform::read_u32(current) & U8_NAME_MASK) as usize;

        if parent_index != 0 {
            let parent = nodes + parent_index as usize * U8_NODE_SIZE;
            let grand_index = platform::read_u32(parent + U8_NODE_DATA_OR_PARENT);
            let parent_name = strings + (platform::read_u32(parent) & U8_NAME_MASK) as usize;

            if grand_index != 0 {
                let grand = nodes + grand_index as usize * U8_NODE_SIZE;
                let grand_name = strings + (platform::read_u32(grand) & U8_NAME_MASK) as usize;
                len = arc_build_dir_path(
                    handle,
                    platform::read_u32(grand + U8_NODE_DATA_OR_PARENT),
                    out,
                    0x80,
                );
                if len != 0x80 {
                    len = arc_append_component(out, len, 0x80, grand_name);
                }
            }
            if len != 0x80 {
                len = arc_append_component(out, len, 0x80, parent_name);
            }
        }
        if len != 0x80 {
            len = arc_append_component(out, len, 0x80, current_name);
        }
    }

    if len == 0x80 {
        // observed: `local_19` is the byte at index 0x7F of the 128-byte buffer.
        platform::write_u8(out + 0x7F, 0);
    } else {
        if platform::read_u32(nodes + directory as usize * U8_NODE_SIZE) & U8_DIRECTORY_MASK != 0
        {
            if len == 0x7F {
                platform::write_u8(out + 0x7F, 0);
                return len;
            }
            platform::write_u8(out + len as usize, b'/');
            len += 1;
        }
        platform::write_u8(out + len as usize, 0);
    }
    len
}

/// Builds the slash-separated path of a U8 directory node.
///
/// The parent chain is followed through each node's `+0x04` word and the
/// names are appended from the oldest ancestor down to `directory`. The result
/// is **not** NUL-terminated; `cap` is a hard byte limit.
///
/// # Original
/// - Ghidra: `FUN_8019fc80` at `0x8019FC80` (456 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019fc80_FUN_8019fc80.c`
///
/// # Arguments
/// - `handle`: U8 handle.
/// - `directory`: directory node index; 0 returns 0 (root has no path).
/// - `out`: destination buffer.
/// - `cap`: maximum bytes to write, including separators.
///
/// # Returns
/// Number of bytes stored, or `cap` when truncated.
///
/// # Side effects
/// Writes `out[0..len]`; no terminator.
///
/// # Safety
/// `out` must be writable for `cap` bytes; the parent chain must be acyclic.
///
/// # Confidence
/// high - the parent recursion and the three append steps are explicit.
pub unsafe fn arc_build_dir_path(
    handle: usize,
    directory: u32,
    out: usize,
    cap: u32,
) -> u32 {
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    let strings = platform::read_u32(handle + ARC_STRINGS) as usize;
    if directory == 0 {
        return 0;
    }

    let mut len = 0u32;
    let current = nodes + directory as usize * U8_NODE_SIZE;
    let current_name = strings + (platform::read_u32(current) & U8_NAME_MASK) as usize;
    let parent_index = platform::read_u32(current + U8_NODE_DATA_OR_PARENT);

    if parent_index != 0 {
        let parent = nodes + parent_index as usize * U8_NODE_SIZE;
        let parent_name = strings + (platform::read_u32(parent) & U8_NAME_MASK) as usize;
        let grand_index = platform::read_u32(parent + U8_NODE_DATA_OR_PARENT);

        if grand_index != 0 {
            let grand = nodes + grand_index as usize * U8_NODE_SIZE;
            let grand_name = strings + (platform::read_u32(grand) & U8_NAME_MASK) as usize;
            len = arc_build_dir_path(
                handle,
                platform::read_u32(grand + U8_NODE_DATA_OR_PARENT),
                out,
                cap,
            );
            if len != cap {
                len = arc_append_component(out, len, cap, grand_name);
            }
        }
        if len != cap {
            len = arc_append_component(out, len, cap, parent_name);
        }
    }
    if len != cap {
        len = arc_append_component(out, len, cap, current_name);
    }
    len
}

/// Appends `/` + one component to a bounded path buffer.
///
/// # Original
/// - Inlined three times in `FUN_8019fc80` at `0x8019FC80`.
/// - Evidence: `work/ghidra/main-gb/export/8019fc80_FUN_8019fc80.c`
///
/// # Safety
/// `out` must be writable for `cap` bytes.
///
/// # Confidence
/// high - the separator write, bounded copy and length arithmetic mirror the
/// C loop.
unsafe fn arc_append_component(out: usize, len: u32, cap: u32, name: usize) -> u32 {
    if len == cap {
        return len;
    }
    platform::write_u8(out + len as usize, b'/');
    let mut next = len + 1;
    let mut source = name;
    let mut remaining = cap - next;
    while remaining != 0 && platform::read_u8(source) != 0 {
        platform::write_u8(out + next as usize, platform::read_u8(source));
        source += 1;
        next += 1;
        remaining -= 1;
    }
    next
}

/// Sets the U8 handle's current directory when `path` names a directory.
///
/// # Original
/// - Ghidra: `FUN_8019fea0` at `0x8019FEA0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019fea0_FUN_8019fea0.c`
///
/// # Arguments
/// - `handle`: U8 handle.
/// - `path`: directory path.
///
/// # Returns
/// `1` when the path resolved to a directory and `handle+0x18` was updated;
/// `0` otherwise.
///
/// # Side effects
/// Writes `ARC_CURRENT_DIR` on success.
///
/// # Safety
/// `handle`/`path` must be valid.
///
/// # Confidence
/// high - the directory test and single store are explicit.
pub unsafe fn arc_set_current_dir(handle: usize, path: usize) -> u32 {
    let index = arc_find_index(handle, path);
    if (index as i32) < 0 {
        return 0;
    }
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    if platform::read_u32(nodes + index as usize * U8_NODE_SIZE) & U8_DIRECTORY_MASK == 0 {
        return 0;
    }
    platform::write_u32(handle + ARC_CURRENT_DIR, index);
    1
}

/// Opens a directory iterator over `path`.
///
/// # Original
/// - Ghidra: `FUN_8019ff00` at `0x8019FF00` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019ff00_FUN_8019ff00.c`
/// - `// inferred:` Ghidra drops the path argument of `FUN_8019fa00`; the
///   iterator call site in `0x802B78F0` passes `(handle, path, iterator)`.
///
/// # Arguments
/// - `handle`: U8 handle.
/// - `path`: directory path.
/// - `iterator`: writable `ARC_DIR_ITER_SIZE`-byte block.
///
/// # Returns
/// `1` on success, `0` when the path is missing or not a directory.
///
/// # Side effects
/// Writes `iterator[0] = handle`, `[1] = directory index`,
/// `[2] = index+1`, `[3] = node+8` (first index outside the directory).
///
/// # Safety
/// `handle`/`path` valid; `iterator` writable for 0x10 bytes.
///
/// # Confidence
/// high - the four writes and the directory check are explicit.
pub unsafe fn arc_open_dir(handle: usize, path: usize, iterator: usize) -> u32 {
    let index = arc_find_index(handle, path);
    if (index as i32) < 0 {
        return 0;
    }
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    let node = nodes + index as usize * U8_NODE_SIZE;
    if platform::read_u32(node) & U8_DIRECTORY_MASK == 0 {
        return 0;
    }

    platform::write_u32(iterator + ARC_DIR_ARCHIVE, handle as u32);
    platform::write_u32(iterator + ARC_DIR_INDEX, index);
    platform::write_u32(iterator + ARC_DIR_CURSOR, index + 1);
    platform::write_u32(
        iterator + ARC_DIR_END,
        platform::read_u32(node + U8_NODE_SIZE_OR_NEXT),
    );
    1
}

/// Reads the next directory entry, skipping the `.` entry.
///
/// # Original
/// - Ghidra: `FUN_8019ff80` at `0x8019FF80` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019ff80_FUN_8019ff80.c`
///
/// # Arguments
/// - `iterator`: iterator from [`arc_open_dir`].
/// - `entry`: writable `ARC_ENTRY_SIZE`-byte entry block.
///
/// # Returns
/// `1` when an entry was produced, `0` at the end of the directory range.
///
/// # Side effects
/// Writes the entry and advances `iterator[2]` past the returned node (or past
/// the whole subtree when the node is a directory).
///
/// # Safety
/// Iterator/entry must be valid and the archive node table in range.
///
/// # Confidence
/// high - the range test, `.` skip and directory jump are explicit.
pub unsafe fn arc_read_dir(iterator: usize, entry: usize) -> u32 {
    let handle = platform::read_u32(iterator + ARC_DIR_ARCHIVE) as usize;
    let nodes = platform::read_u32(handle + ARC_NODES) as usize;
    let strings = platform::read_u32(handle + ARC_STRINGS) as usize;
    let mut cursor = platform::read_u32(iterator + ARC_DIR_CURSOR);
    let start = platform::read_u32(iterator + ARC_DIR_INDEX);
    let end = platform::read_u32(iterator + ARC_DIR_END);

    loop {
        if cursor <= start || end <= cursor {
            return 0;
        }

        let node = nodes + cursor as usize * U8_NODE_SIZE;
        let flags = platform::read_u32(node + U8_NODE_FLAGS_NAME);
        let name = strings + (flags & U8_NAME_MASK) as usize;

        platform::write_u32(entry + ARC_ENTRY_ARCHIVE, handle as u32);
        platform::write_u32(entry + ARC_ENTRY_INDEX, cursor);
        platform::write_u32(
            entry + ARC_ENTRY_IS_DIR,
            ((flags & U8_DIRECTORY_MASK) != 0) as u32,
        );
        platform::write_u32(entry + ARC_ENTRY_NAME, name as u32);

        // observed: skip a member whose name is exactly ".".
        if platform::read_u8(name) != b'.' || platform::read_u8(name + 1) != 0 {
            break;
        }
        cursor += 1;
    }

    let node = nodes + cursor as usize * U8_NODE_SIZE;
    let flags = platform::read_u32(node + U8_NODE_FLAGS_NAME);
    let next = if flags & U8_DIRECTORY_MASK == 0 {
        cursor + 1
    } else {
        platform::read_u32(node + U8_NODE_SIZE_OR_NEXT)
    };
    platform::write_u32(iterator + ARC_DIR_CURSOR, next);
    1
}

/// Returns the data pointer described by a file-info block.
///
/// `// observed:` the block's first word is the archive base and its second
/// word is the member's data offset, so the result is
/// `*(u32 *)info + info[1]`.
///
/// # Original
/// - Ghidra: `FUN_8019fe50` at `0x8019FE50` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8019fe50_FUN_8019fe50.c`
///
/// # Arguments
/// - `info`: file-info block from [`arc_open`] or [`arc_file_info_by_index`].
///
/// # Returns
/// Address of the first byte of the member.
///
/// # Side effects
/// None.
///
/// # Safety
/// `info` must be readable and the resulting address must be in the archive.
///
/// # Confidence
/// high - two-word arithmetic only.
pub unsafe fn arc_file_data(info: usize) -> u32 {
    platform::read_u32(info) + platform::read_u32(info + 4)
}

// ===========================================================================
// NW4R archive registry, 0x802B7380..0x802B7A40
// ===========================================================================

/// Constructs and registers an NW4R archive object.
///
/// The object begins with a vtable pointer, has a mode word at `+0x10`, a
/// refcount at `+0x14`, and an embedded 0x1C-byte U8 handle at `+0x18`. The
/// constructor clears that handle and appends the object to the global list.
///
/// # Original
/// - Ghidra: `FUN_802b7380` at `0x802B7380` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7380_FUN_802b7380.c`
/// - Vtable: `0x8034FBD0`.
///
/// # Arguments
/// - `object`: writable `RES_ARCHIVE_SIZE`-byte object.
///
/// # Returns
/// `object`, for chaining.
///
/// # Side effects
/// Calls the base constructor, writes the vtable/mode/refcount, clears the
/// handle with `memset`, and appends to `RES_ARCHIVE_LIST`.
///
/// # Safety
/// `object` must be writable for 0x3C bytes and not already in a list.
///
/// # Confidence
/// medium - the field writes and the list append are explicit; the base
/// constructor and vtable identities are inferred from `DAT_8034FBD0`.
pub unsafe fn fn_802b7380(object: usize) -> u32 {
    platform::call1(RES_ARCHIVE_BASE_CTOR, object as u32);
    platform::write_u32(object + RES_ARCHIVE_VTABLE, 0x8034_FBD0);
    platform::write_u32(object + RES_ARCHIVE_REFS, 1);
    platform::write_u32(object + RES_ARCHIVE_MODE, 0);
    platform::call3(
        MEMSET,
        (object + RES_ARCHIVE_HANDLE) as u32,
        0,
        ARC_HANDLE_SIZE as u32,
    );
    fn_802b7540(object);
    object as u32
}

/// Deleting destructor of the NW4R archive object.
///
/// Re-installs the `0x8034FBD0` vtable, unlinks the object from
/// [`RES_ARCHIVE_LIST`] with [`fn_802b75a0`], tears the embedded handle down
/// with `0x802BA330(object, 0)`, frees the object with `0x802B9280` when
/// `release > 0`, and returns `object` for chaining.
///
/// # Original (untabled)
/// - Ghidra: none - no functions.tsv row for 0x802B73F0.
/// - Extent: `0x802B73F0..0x802B745C` (108 bytes, 26 instructions), ending at
///   the `blr` at `0x802B7458`; one zero word follows at `0x802B745C` and the
///   next prologue is at `0x802B7460`.
/// - Reach: no `b`/`bl` in the main image; the only reference is the data word
///   at `0x8034FBD8`, slot `0` of the `0x8034FBD0` vtable (the archive object's
///   own table, installed by [`fn_802b7380`]). `d_basesNP` reaches it through
///   that vtable.
/// - Table: `0x8034FBD0` slot 0.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `cmpi cr0,r3,0`;
///   `bc 12,2` (`bt cr0.eq`) to the return; `lis r4,0x8035` +
///   `addi r4,r4,-1072` = `0x8034FBD0`; `stw r4,0(r3)`; `bl 0x802B75A0`;
///   `mr r3,r30`; `li r4,0`; `bl 0x802BA330`; `cmpi cr0,r31,0`;
///   `bc 4,1` (`bf cr0.gt`) to the return; `mr r3,r30`; `bl 0x802B9280`;
///   the return arm `mr r3,r30`; epilogue; `4E800020 bclr 20,0` (blr).
/// - Polarity: `bc 4,1` is `bf cr0.gt`, so the free is skipped when
///   `release <= 0` and runs only for `release > 0`.
/// - Page arithmetic: `0xFBD0` sign-extends to `-0x430`, so `lis 0x8035` plus
///   that immediate is `0x8034FBD0`, matching the vtable word.
///
/// # Arguments
/// - `object`: archive object being destroyed; may be null.
/// - `release`: positive when the storage must also be freed.
///
/// # Returns
/// `object`.
///
/// # Side effects
/// Writes the vtable word, unlinks the object from the archive list, tears down
/// its U8 handle and optionally frees it.
///
/// # Safety
/// A non-null `object` must be a live archive object currently in
/// [`RES_ARCHIVE_LIST`]; a positive `release` must allow `0x802B9280` to free
/// it.
///
/// # Confidence
/// high - the null guard, the vtable store, the three calls and the ownership
/// test are literal.
pub unsafe fn fn_802b73f0(object: usize, release: i32) -> usize {
    if object != 0 {
        platform::write_u32(object, 0x8034_FBD0);
        fn_802b75a0(object);
        platform::call2(0x802B_A330, object as u32, 0);
        if release > 0 {
            platform::call1(0x802B_9280, object as u32);
        }
    }
    object
}

/// Mounts an in-memory U8 archive into an archive object.
///
/// # Original
/// - Ghidra: `FUN_802b7460` at `0x802B7460` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7460_FUN_802b7460.c`
/// - `// inferred:` the C prototype is `void`, but every caller compares the
///   result with 0; the routine is treated as returning the mount status.
///
/// # Arguments
/// - `object`: archive object from [`fn_802b7380`].
/// - `data`: address of the U8 image.
/// - `mode`: value stored at `+0x10` on success; callers pass `1`.
///
/// # Returns
/// `1` when [`arc_init_handle`] succeeded, `0` otherwise.
///
/// # Side effects
/// Writes the embedded handle at `object+0x18` and the mode word at `+0x10`
/// (`0` on failure).
///
/// # Safety
/// `object` must be writable; `data` must satisfy [`arc_init_handle`].
///
/// # Confidence
/// high - the call and both stores are explicit. medium for the return-value
/// reconstruction (Ghidra prints `void`).
pub unsafe fn fn_802b7460(object: usize, data: usize, mode: u32) -> u32 {
    let mounted = arc_init_handle(data, object + RES_ARCHIVE_HANDLE);
    platform::write_u32(
        object + RES_ARCHIVE_MODE,
        if mounted != 0 { mode } else { 0 },
    );
    mounted
}

/// Finds an already-mounted archive object by its archive data pointer.
///
/// # Original
/// - Ghidra: `FUN_802b74c0` at `0x802B74C0` (128 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b74c0_FUN_802b74c0.c`
/// - `// inferred:` the call from `FUN_802b75b0` prints no argument, but the
///   body compares the caller's data pointer with `object+0x18` (the U8 handle
///   base), so `data` is passed in `r3`.
///
/// # Arguments
/// - `data`: archive image pointer to match.
///
/// # Returns
/// The archive object whose embedded handle base equals `data`, or 0.
///
/// # Side effects
/// None; read-only list walk.
///
/// # Safety
/// The global list and its objects must be valid.
///
/// # Confidence
/// medium - the scan and comparison are exact; the list-helper identity and
/// the pass-through argument are inferred.
pub unsafe fn fn_802b74c0(data: usize) -> u32 {
    if platform::read_u8(RES_ARCHIVE_LIST_READY) == 0 {
        return 0;
    }
    let mut node = 0u32;
    loop {
        node = platform::call2(LIST_NEXT, RES_ARCHIVE_LIST as u32, node);
        if node == 0 {
            return 0;
        }
        if platform::read_u32(node as usize + RES_ARCHIVE_HANDLE) == data as u32 {
            return node;
        }
    }
}

/// Appends an archive object to the global list, initialising it on first use.
///
/// # Original
/// - Ghidra: `FUN_802b7540` at `0x802B7540` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7540_FUN_802b7540.c`
///
/// # Arguments
/// - `object`: archive object with its list links at `+0x34`.
///
/// # Side effects
/// On first use calls `FUN_80228DD0(list, 0x34)` and sets the ready byte; then
/// appends the object.
///
/// # Safety
/// `object` must be writable and its `+0x34` links unused.
///
/// # Confidence
/// high - the lazy-init guard and list calls are explicit.
pub unsafe fn fn_802b7540(object: usize) {
    if platform::read_u8(RES_ARCHIVE_LIST_READY) == 0 {
        platform::call2(
            LIST_INIT,
            RES_ARCHIVE_LIST as u32,
            RES_ARCHIVE_LIST_OFFSET as u32,
        );
        platform::write_u8(RES_ARCHIVE_LIST_READY, 1);
    }
    platform::call2(LIST_APPEND, RES_ARCHIVE_LIST as u32, object as u32);
}

/// Removes an archive object from the global list.
///
/// # Original
/// - Ghidra: `FUN_802b75a0` at `0x802B75A0` (16 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b75a0_FUN_802b75a0.c`
///
/// # Arguments
/// - `object`: archive object previously appended by [`fn_802b7540`].
///
/// # Side effects
/// Unlinks the object's `+0x34` list node and decrements the list count.
///
/// # Safety
/// `object` must currently be in the global list.
///
/// # Confidence
/// high - single call.
pub unsafe fn fn_802b75a0(object: usize) {
    platform::call2(LIST_REMOVE, RES_ARCHIVE_LIST as u32, object as u32);
}

/// Acquires an archive object for an image, allocating and mounting on first use.
///
/// If an object already exists for `data`, its refcount is incremented.
/// Otherwise a `RES_ARCHIVE_SIZE`-byte object is allocated, constructed,
/// mounted with mode `1`, and returned. A mount failure destroys the fresh
/// object and returns 0.
///
/// `// observed:` the C calls `FUN_802b7460` even when allocation returned 0,
/// which would dereference null. `// inferred:` allocation failure is fatal in
/// practice; the null case is guarded here so host checks cannot hit undefined
/// behaviour.
///
/// # Original
/// - Ghidra: `FUN_802b75b0` at `0x802B75B0` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b75b0_FUN_802b75b0.c`
///
/// # Arguments
/// - `data`: address of a U8 image.
/// - `heap`: allocator heap argument (second argument of `FUN_802b9240`).
/// - `flags`: allocator flags (third argument).
///
/// # Returns
/// The archive object, or 0 when allocation or mounting failed.
///
/// # Side effects
/// May allocate, construct/register, mount, or increment the refcount; on
/// failure calls the object's deleting destructor.
///
/// # Safety
/// `data` must satisfy [`arc_init_handle`]; the allocator arguments must be
/// valid for `0x802B9240`.
///
/// # Confidence
/// medium - the normal path is exact; the allocator argument order and the
/// null-allocation guard are inferred.
pub unsafe fn fn_802b75b0(data: usize, heap: u32, flags: u32) -> u32 {
    let mut object = fn_802b74c0(data);
    if object == 0 {
        let memory = platform::call3(
            RES_ARCHIVE_ALLOC,
            RES_ARCHIVE_SIZE as u32,
            heap,
            flags,
        );

        let mut fresh = 0u32;
        if memory != 0 {
            fresh = fn_802b7380(memory as usize);
        }

        let mounted = if fresh != 0 {
            fn_802b7460(fresh as usize, data, 1)
        } else {
            0
        };

        if mounted == 0 {
            if fresh != 0 {
                res_archive_destroy(fresh as usize);
            }
            fresh = 0;
        }
        object = fresh;
    } else {
        let refs = platform::read_u32(object as usize + RES_ARCHIVE_REFS);
        platform::write_u32(object as usize + RES_ARCHIVE_REFS, refs + 1);
    }
    object
}

/// Releases an archive object, destroying it when the refcount reaches zero.
///
/// # Original
/// - Ghidra: `FUN_802b7680` at `0x802B7680` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7680_FUN_802b7680.c`
///
/// # Arguments
/// - `object`: archive object from [`fn_802b75b0`].
///
/// # Side effects
/// Decrements `+0x14`; at zero clears `+0x10` and invokes vtable slot `+0x08`
/// (the deleting destructor) with `(object, 1)`.
///
/// # Safety
/// `object` must be a live archive object. Calling with a zero refcount is a
/// no-op, matching the original guard.
///
/// # Confidence
/// high - the guard, decrement and virtual call are explicit.
pub unsafe fn fn_802b7680(object: usize) {
    let refs = platform::read_u32(object + RES_ARCHIVE_REFS);
    if refs == 0 {
        return;
    }
    let next = refs - 1;
    platform::write_u32(object + RES_ARCHIVE_REFS, next);
    if next != 0 {
        return;
    }
    platform::write_u32(object + RES_ARCHIVE_MODE, 0);
    // observed: the C repeats a null test here even though `object` was already
    // dereferenced above; this is always false and is kept as documentation.
    if object == 0 {
        return;
    }
    res_archive_destroy(object);
}

/// Calls the archive object's deleting destructor through vtable slot `+0x08`.
///
/// # Original
/// - Inlined in `FUN_802b75b0` and `FUN_802b7680`.
/// - Evidence: `work/ghidra/main-gb/export/802b7680_FUN_802b7680.c`
///
/// # Safety
/// `object` must point at a live object whose vtable slot `+0x08` is a
/// deleting destructor with the `(this, int)` ABI.
///
/// # Confidence
/// high - the virtual call shape is explicit.
unsafe fn res_archive_destroy(object: usize) {
    let vtable = platform::read_u32(object + RES_ARCHIVE_VTABLE) as usize;
    let destructor = platform::read_u32(vtable + 8) as usize;
    platform::call2(destructor, object as u32, 1);
}

/// Opens a BRRES/U8 member inside a mounted archive and returns its data.
///
/// The archive object must be in mode `1`. The file-info block is stack-local;
/// on a successful open the routine returns `archive_base + member_offset`,
/// which is the first byte of the BRRES file. The two optional output words
/// receive the member's data offset and size.
///
/// # Original
/// - Ghidra: `FUN_802b76c0` at `0x802B76C0` (160 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b76c0_FUN_802b76c0.c`
///
/// # Arguments
/// - `object`: archive object from [`fn_802b75b0`].
/// - `path`: member path inside the U8 archive.
/// - `size_out`: 0, or a writable two-word block for `(offset, size)`.
///
/// # Returns
/// Pointer to the member's first byte, or 0 when the member is missing or the
/// archive is not in mode `1`.
///
/// # Side effects
/// Calls [`arc_open`] (which may log), writes `size_out` on a hit, and calls
/// the file-info close stub `0x8019FE90`.
///
/// # Safety
/// `object` must be a live archive object; `path` a C string; `size_out` either
/// 0 or writable for 8 bytes. The returned pointer belongs to the archive.
///
/// # Confidence
/// high - the mode check, file-info writes and pointer arithmetic are explicit.
pub unsafe fn fn_802b76c0(object: usize, path: usize, size_out: usize) -> u32 {
    let mut info = crate::platform::GuestLocal::new( [0u32; 7]);
    let opened = arc_open(
        object + RES_ARCHIVE_HANDLE,
        path,
        info.as_mut_ptr() as usize,
    );

    let mut data = 0u32;
    if opened != 0 {
        if platform::read_u32(object + RES_ARCHIVE_MODE) == 1 {
            data = arc_file_data(info.as_ptr() as usize);
        }
        if size_out != 0 {
            platform::write_u32(
                size_out,
                platform::call1(ARC_FILE_OFFSET_GET, info.as_ptr() as u32),
            );
            platform::write_u32(
                size_out + 4,
                platform::call1(ARC_FILE_SIZE_GET, info.as_ptr() as u32),
            );
        }
    }
    platform::call1(ARC_CLOSE_FILE, info.as_ptr() as u32);
    data
}

/// Finds a member's node index by path inside a mounted archive.
///
/// # Original
/// - Ghidra: `FUN_802b7760` at `0x802B7760` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7760_FUN_802b7760.c`
/// - `// inferred:` the C prints `FUN_8019fa00(param_1 + 0x18)` with one
///   argument; the path is still live in `r4` and is passed explicitly here.
///   The C prototype is `void`, but callers compare the result with `-1`, so
///   the routine returns the index as a signed value.
///
/// # Arguments
/// - `object`: archive object.
/// - `path`: member path.
///
/// # Returns
/// Node index, or `-1` when not found.
///
/// # Side effects
/// None beyond [`arc_find_index`].
///
/// # Safety
/// `object`/`path` must be valid.
///
/// # Confidence
/// high - one call and one return; the argument/return reconstruction is
/// inferred from the caller.
pub unsafe fn fn_802b7760(object: usize, path: usize) -> i32 {
    arc_find_index(object + RES_ARCHIVE_HANDLE, path) as i32
}

/// Returns a member's data pointer by node index.
///
/// # Original
/// - Ghidra: `FUN_802b7770` at `0x802B7770` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7770_FUN_802b7770.c`
///
/// # Arguments
/// - `object`: archive object in mode `1`.
/// - `index`: node index from [`fn_802b7760`].
/// - `size_out`: 0, or a writable two-word block for `(offset, size)`.
///
/// # Returns
/// Pointer to the member's first byte, or 0 when the archive is not in mode 1
/// or the index is not a file.
///
/// # Side effects
/// Fills and closes a stack-local file-info block; writes `size_out` on a hit.
///
/// # Safety
/// `object` must be live; `size_out` either 0 or writable for 8 bytes.
///
/// # Confidence
/// high - the mode guard, index call and output writes are explicit.
pub unsafe fn fn_802b7770(object: usize, index: u32, size_out: usize) -> u32 {
    let mut data = 0u32;
    if platform::read_u32(object + RES_ARCHIVE_MODE) == 1 {
        let mut info = crate::platform::GuestLocal::new( [0u32; 7]);
        let found = arc_file_info_by_index(
            object + RES_ARCHIVE_HANDLE,
            index,
            info.as_mut_ptr() as usize,
        );
        if found != 0 {
            data = arc_file_data(info.as_ptr() as usize);
            if size_out != 0 {
                platform::write_u32(
                    size_out,
                    platform::call1(ARC_FILE_OFFSET_GET, info.as_ptr() as u32),
                );
                platform::write_u32(
                    size_out + 4,
                    platform::call1(ARC_FILE_SIZE_GET, info.as_ptr() as u32),
                );
            }
        }
        platform::call1(ARC_CLOSE_FILE, info.as_ptr() as u32);
    }
    data
}

/// Recursively enumerates a U8 directory and calls back for every entry.
///
/// The callback receives `(user, data, entry, path)`:
///
/// * `user` - the caller's context word,
/// * `data` - BRRES data pointer for a file, 0 for a directory,
/// * `entry` - pointer to the four-word entry from [`arc_read_dir`],
/// * `path` - the current path buffer (file path or directory path).
///
/// For a directory the routine appends the name, calls back, appends `/`,
/// recurses with `remaining - (name_len + 1)`, then restores the path to its
/// previous length. For a file it appends the name, opens the member through
/// [`fn_802b76c0`], calls back, then restores the path.
///
/// # Original
/// - Ghidra: `FUN_802b78f0` at `0x802B78F0` (324 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b78f0_FUN_802b78f0.c`
/// - `// inferred:` the first two parameters are recovered from the outlined
///   `__savegprlr` prologue (`0x802DCD60`), whose `r3:r4` pair the decompiler
///   prints as a `u64`: `r3` is the archive object and `r4` the U8 handle.
///
/// # Arguments
/// - `archive`: archive object (used for [`fn_802b76c0`]).
/// - `handle`: embedded U8 handle (used for the directory iterator).
/// - `callback`: 4-argument callback address.
/// - `user`: context word passed through unchanged.
/// - `path`: writable NUL-terminated path buffer.
/// - `remaining`: capacity hint remaining in `path`.
///
/// # Side effects
/// Mutates `path`, opens files, invokes the callback, and closes the iterator.
///
/// # Safety
/// `path` must be writable and large enough for the original's 0x100-byte
/// assumption; `callback` must implement the 4-argument ABI. The original does
/// not check `remaining` before appending a file name, so the caller must size
/// the buffer for the archive's deepest path.
///
/// # Confidence
/// medium - the loop and path edits are explicit; the callback argument order
/// and the outlined-prologue register mapping are inferred from call sites.
pub unsafe fn fn_802b78f0(
    archive: usize,
    handle: usize,
    callback: usize,
    user: u32,
    path: usize,
    remaining: i32,
) {
    let mut iterator = crate::platform::GuestLocal::new( [0u32; 4]);
    let opened = arc_open_dir(handle, path, iterator.as_mut_ptr() as usize);
    if opened == 0 {
        return;
    }

    let mut entry = crate::platform::GuestLocal::new( [0u32; 4]);
    loop {
        if arc_read_dir(iterator.as_mut_ptr() as usize, entry.as_mut_ptr() as usize) == 0 {
            break;
        }

        let old_len = platform::call1(STRLEN, path as u32);
        let name = entry[ARC_ENTRY_NAME / 4] as usize;
        let is_dir = entry[ARC_ENTRY_IS_DIR / 4] != 0;

        if !is_dir {
            // observed: the C computes strlen(name) here but never uses it.
            platform::call2(STRCAT, path as u32, name as u32);
            let data = fn_802b76c0(archive, path, 0);
            platform::call4(
                callback,
                user,
                data,
                entry.as_ptr() as u32,
                path as u32,
            );
        } else {
            let name_len = platform::call1(STRLEN, name as u32);
            platform::call2(STRCAT, path as u32, name as u32);
            platform::call4(
                callback,
                user,
                0,
                entry.as_ptr() as u32,
                path as u32,
            );
            platform::call2(STRCAT, path as u32, ROOT_PATH as u32);
            fn_802b78f0(
                archive,
                handle,
                callback,
                user,
                path,
                remaining - (name_len as i32 + 1),
            );
        }

        platform::write_u8(path + old_len as usize, 0);
    }

    platform::call1(ARC_CLOSE_DIR, iterator.as_mut_ptr() as u32);
}

/// Enumerates the whole archive from `/` with a callback.
///
/// # Original
/// - Ghidra: `FUN_802b7a40` at `0x802B7A40` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802b7a40_FUN_802b7a40.c`
/// - Root string: `0x80429B10` is `/`, at `r13 - 0x5B70`.
///
/// # Arguments
/// - `object`: archive object.
/// - `callback`: callback address, forwarded to [`fn_802b78f0`].
/// - `user`: context word, forwarded unchanged.
///
/// # Side effects
/// Builds a 256-byte path starting at `/`, calls [`fn_802b78f0`] with the
/// remaining capacity `0x100 - (strlen(path) + 1)`, and lets the callback
/// observe every entry.
///
/// # Safety
/// `object` must be a mounted archive; `callback` must implement the callback
/// ABI described in [`fn_802b78f0`].
///
/// # Confidence
/// high - the root copy, length computation and call are explicit.
pub unsafe fn fn_802b7a40(object: usize, callback: usize, user: u32) {
    let mut path = crate::platform::GuestLocal::new( [0u8; 256]);
    let base = path.as_mut_ptr() as usize;
    platform::call2(STRCPY, base as u32, ROOT_PATH as u32);
    let len = platform::call1(STRLEN, base as u32) as i32;
    fn_802b78f0(
        object,
        object + RES_ARCHIVE_HANDLE,
        callback,
        user,
        base,
        0x100 - (len + 1),
    );
}

// ===========================================================================
// BRRES root lookup, 0x80239E30
// ===========================================================================

/// Looks up a named model resource in a BRRES file's `3DModels(NW4R)` section.
///
/// The BRRES bytes are addressed indirectly because the caller stores the file
/// pointer in a field (e.g. actor `+0x540`). The function reads that pointer,
/// takes the root dictionary at file `+0x18`, finds the section named
/// `3DModels(NW4R)` through [`res_dict_get_section`]'s core, then finds `name`
/// inside that section through [`res_dict_get_named`]'s core.
///
/// This is the **container-level** lookup only. The returned pointer is an
/// opaque resource record; interpreting it as an `MDL0` model is the job of the
/// separate model parser.
///
/// # Original
/// - Ghidra: `FUN_80239e30` at `0x80239E30` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239e30_FUN_80239e30.c`
/// - Tag: `0x8034DF60` is the length-prefixed string `3DModels(NW4R)`.
///
/// # Arguments
/// - `file_ptr`: address of the word that holds the BRRES file pointer.
/// - `name`: address of the NUL-terminated resource name.
///
/// # Returns
/// Resource pointer from the section dictionary, or 0 when the section or name
/// is missing.
///
/// # Side effects
/// None; the dictionary helpers are read-only.
///
/// # Safety
/// `*file_ptr` must point at a readable BRRES image with a root dictionary at
/// `+0x18`; `name` must be a readable C string.
///
/// # Confidence
/// high - the two-stage chain and the section tag are explicit and the tag
/// bytes were verified in `work/dol/main.elf`.
pub unsafe fn fn_80239e30(file_ptr: usize, name: u32) -> u32 {
    let base = platform::read_u32(file_ptr) as usize;

    // observed: `local_10[0] = *param_1 + 0x18`.
    let dictionary = (base + BRRES_ROOT_DICT) as u32;
    // observed: `local_14 = &DAT_8034df60`.
    let section = res_dict_get_record_value(dictionary, BRRES_TYPE_MODELS);
    if section == 0 {
        return 0;
    }
    res_dict_get_named_value(section, name)
}

/// Looks up a named animation-scene resource in a BRRES file's `AnmScn(NW4R)`
/// section.
///
/// The `3DModels(NW4R)` twin of [`fn_80239e30`]: it reads the file pointer from
/// `*file_ptr`, takes the root dictionary at file `+0x18`, resolves the
/// `AnmScn(NW4R)` section through [`res_dict_get_record_value`], then resolves
/// `name` inside that section through [`res_dict_get_named_value`].
///
/// # Original (untabled)
/// - Ghidra: none - no functions.tsv row for 0x8023A2E0.
/// - Extent: `0x8023A2E0..0x8023A34C` (108 bytes, 25 instructions), ending at
///   the `blr` at `0x8023A348`; one zero word follows at `0x8023A34C` and the
///   next prologue is at `0x8023A350`.
/// - Reach: seven `bl` sites in `d_basesNP` (first `0x807ABE74`, last
///   `0x807BA6B4`); no main-image branch and no data word names it.
/// - Table: none.
/// - Observed: `stwu r1,-0x20(r1)`; `mflr r0`; `lwz r5,0(r3)`;
///   `lis r3,0x8035` + `addi r3,r3,-8064` = `0x8034E080`
///   (`BRRES_TYPE_ANM_SCN`); `addi r0,r5,0x18`; `mr r31,r4`;
///   `addi r4,r1,0xC`; `stw r3,0xC(r1)`; `addi r3,r1,0x10`;
///   `stw r0,0x10(r1)`; `bl 0x80239CC0`; `cmpi cr0,r3,0`;
///   `bc 12,2` (`bt cr0.eq`) to the `li r3,0` exit; `stw r3,8(r1)`;
///   `mr r4,r31`; `addi r3,r1,8`; `bl 0x80239C40`; `b` to the epilogue; the
///   other arm `li r3,0`; epilogue; `4E800020 bclr 20,0` (blr).
/// - Page arithmetic: `0xE080` sign-extends to `-0x1F80`, so `lis 0x8035` plus
///   that immediate is `0x8034E080`, the length-prefixed `AnmScn(NW4R)` name
///   record already named `BRRES_TYPE_ANM_SCN`.
/// - Decoder note: the two stack words are the `dict_ptr`/`key_ptr` pair the
///   old opaque helpers expect; the translation calls the two dictionary
///   routines directly, as the module header prescribes.
///
/// # Arguments
/// - `file_ptr`: address of the word that holds the BRRES file pointer; the
///   caller at `0x807ABE70` passes `&(record)` from `0x800DF180`.
/// - `name`: address of the NUL-terminated resource name.
///
/// # Returns
/// Resource pointer from the `AnmScn` dictionary, or 0 when the section or name
/// is missing.
///
/// # Side effects
/// None; the dictionary helpers are read-only.
///
/// # Safety
/// `*file_ptr` must point at a readable BRRES image with a root dictionary at
/// `+0x18`, and `name` must be a readable C string.
///
/// # Confidence
/// high - the two-stage chain, the section tag and the null guard are literal.
pub unsafe fn fn_8023a2e0(file_ptr: usize, name: u32) -> u32 {
    let base = platform::read_u32(file_ptr) as usize;
    let dictionary = (base + BRRES_ROOT_DICT) as u32;
    let section = res_dict_get_record_value(dictionary, BRRES_TYPE_ANM_SCN);
    if section == 0 {
        return 0;
    }
    res_dict_get_named_value(section, name)
}

// ===========================================================================
// NW4R ResDict: 0x80239930..0x80239DC0
// ===========================================================================

/// Compares two length-prefixed dictionary-name records.
///
/// The two arguments are addresses of words that hold pointers to name
/// records. A record is `{u32 length; char name[]}`. Records compare equal
/// only when the lengths match and `strcmp` on the name bytes returns zero.
///
/// `// observed:` the original calls `countLeadingZeros(strcmp(...)) >> 5`,
/// which is 1 exactly when `strcmp` returned 0. The equality test below is the
/// same predicate without the bit-count detour.
///
/// # Original
/// - Ghidra: `FUN_80239930` at `0x80239930` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239930_FUN_80239930.c`
///
/// # Arguments
/// - `key_a`: address of a word holding a name-record pointer.
/// - `key_b`: address of the second name-record pointer word.
///
/// # Returns
/// `1` when the records are equal, `0` otherwise.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// Both pointer words and both records must be readable.
///
/// # Confidence
/// high - both the length check and the `strcmp` call are explicit.
pub unsafe fn res_dict_compare(key_a: usize, key_b: usize) -> u32 {
    let a = platform::read_u32(key_a) as usize;
    let b = platform::read_u32(key_b) as usize;
    if platform::read_u32(a) != platform::read_u32(b) {
        return 0;
    }
    if platform::call2(STRCMP, (a + 4) as u32, (b + 4) as u32) == 0 {
        1
    } else {
        0
    }
}

/// Finds the dictionary node for a length-prefixed key record.
///
/// The dictionary is a binary search tree keyed on the bits of the resource
/// name. Node 0 at `dict + 0x08` is the sentinel; the root index is the `u16`
/// at `dict + 0x0C`. The walk stops when the candidate's hash is no longer
/// below the previous node's hash. The candidate is then compared with the
/// requested name record.
///
/// `// observed:` the C uses the key's name bits directly:
/// `key[hash >> 3] >> (hash & 7) & 1` chooses left versus right.
///
/// # Original
/// - Ghidra: `FUN_80239a80` at `0x80239A80` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239a80_FUN_80239a80.c`
///
/// # Arguments
/// - `dict_ptr`: address of a word holding the dictionary base.
/// - `key_ptr`: address of a word holding the key record pointer.
///
/// # Returns
/// Address of the matching 16-byte node, or 0.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The dictionary, its node tree, and the key record must be readable. The
/// node indices are trusted exactly as in the original.
///
/// # Confidence
/// high - the tree walk, child selection and name compare are explicit.
pub unsafe fn res_dict_find(dict_ptr: usize, key_ptr: usize) -> u32 {
    let base = platform::read_u32(dict_ptr) as usize;
    let key = platform::read_u32(key_ptr) as usize;
    let key_len = platform::read_u32(key);

    let mut sentinel = base + 8;
    let mut node = base + platform::read_u16(base + RES_DICT_ROOT) as usize * RES_DICT_NODE + 8;

    loop {
        let hash = platform::read_u16(node + RES_DICT_HASH);
        if hash as u32 >= platform::read_u16(sentinel + RES_DICT_HASH) as u32 {
            break;
        }
        sentinel = node;

        let byte_index = (hash >> 3) as usize;
        let go_right = (hash as u32 >> 3) < key_len
            && ((platform::read_u8(key + byte_index + RES_NAME_LENGTH_WORD) >> (hash & 7)) & 1)
                != 0;
        let child = if go_right {
            platform::read_u16(node + RES_DICT_RIGHT)
        } else {
            platform::read_u16(node + RES_DICT_LEFT)
        };
        node = base + child as usize * RES_DICT_NODE + 8;
    }

    let record = (base + platform::read_u32(node + RES_DICT_NAME) as usize)
        .wrapping_sub(RES_NAME_LENGTH_WORD);
    let mut record_word = crate::platform::GuestLocal::new( record as u32);
    if res_dict_compare(key_ptr, (record_word.as_mut_ptr() as *mut u32) as usize) == 0 {
        0
    } else {
        node as u32
    }
}

/// Finds the dictionary node for a plain C-string name and explicit length.
///
/// This is the string-key variant of [`res_dict_find`]. It walks the same
/// tree, using the name bytes directly for the hash-bit test, then verifies
/// the node's name with `strcmp`.
///
/// # Original
/// - Ghidra: `FUN_80239b60` at `0x80239B60` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239b60_FUN_80239b60.c`
///
/// # Arguments
/// - `dict_ptr`: address of a word holding the dictionary base.
/// - `name`: address of the name string.
/// - `len`: `strlen(name)`, supplied by the caller.
///
/// # Returns
/// Address of the matching node, or 0.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The dictionary and `name[0..len]` must be readable.
///
/// # Confidence
/// high - every test and the final `strcmp` are explicit.
pub unsafe fn res_dict_find_name(dict_ptr: usize, name: usize, len: u32) -> u32 {
    let base = platform::read_u32(dict_ptr) as usize;

    let mut sentinel = base + 8;
    let mut node = base + platform::read_u16(base + RES_DICT_ROOT) as usize * RES_DICT_NODE + 8;

    loop {
        let hash = platform::read_u16(node + RES_DICT_HASH);
        if hash as u32 >= platform::read_u16(sentinel + RES_DICT_HASH) as u32 {
            break;
        }
        sentinel = node;

        let byte_index = (hash >> 3) as usize;
        let go_right = (hash as u32 >> 3) < len
            && ((platform::read_u8(name + byte_index) >> (hash & 7)) & 1) != 0;
        let child = if go_right {
            platform::read_u16(node + RES_DICT_RIGHT)
        } else {
            platform::read_u16(node + RES_DICT_LEFT)
        };
        node = base + child as usize * RES_DICT_NODE + 8;
    }

    let name_offset = platform::read_u32(node + RES_DICT_NAME);
    if name_offset == 0
        || platform::call2(STRCMP, name as u32, (base + name_offset as usize) as u32) != 0
    {
        0
    } else {
        node as u32
    }
}

/// Looks up a named resource in a dictionary and returns its data pointer.
///
/// The dictionary base is `*dict_ptr`; the name is a C string. The returned
/// pointer is `dict_base + node.data_offset`, the first byte of the resource
/// body (an MDL0, TEX0, ... record).
///
/// # Original
/// - Ghidra: `FUN_80239c40` at `0x80239C40` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239c40_FUN_80239c40.c`
///
/// # Arguments
/// - `dict_ptr`: address of a word holding the dictionary base.
/// - `name`: address of the NUL-terminated resource name.
///
/// # Returns
/// Resource data pointer, or 0 when the dictionary or name is null or absent.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The dictionary and `name` must be readable.
///
/// # Confidence
/// high - the two guards, `strlen`, tree lookup and offset add are explicit.
pub unsafe fn res_dict_get_named(dict_ptr: usize, name: u32) -> u32 {
    res_dict_get_named_value(platform::read_u32(dict_ptr), name)
}

/// Looks up a resource by a length-prefixed key record.
///
/// This is the record-key sibling of [`res_dict_get_named`]. It is used for
/// the BRRES tag strings, whose length word is embedded in the image.
///
/// # Original
/// - Ghidra: `FUN_80239cc0` at `0x80239CC0` (104 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239cc0_FUN_80239cc0.c`
///
/// # Arguments
/// - `dict_ptr`: address of a word holding the dictionary base.
/// - `key_ptr`: address of a word holding the key-record pointer.
///
/// # Returns
/// Resource data pointer, or 0 when the dictionary or key is null or absent.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The dictionary, key record and node table must be readable.
///
/// # Confidence
/// high - the guard, tree lookup and offset add are explicit.
pub unsafe fn res_dict_get_section(dict_ptr: usize, key_ptr: usize) -> u32 {
    res_dict_get_record_value(
        platform::read_u32(dict_ptr),
        platform::read_u32(key_ptr),
    )
}

/// Returns a named resource's zero-based dictionary index.
///
/// The index is measured from the dictionary's first resource node (node 1),
/// so node 1 is index 0. The signed correction is the original's C division
/// truncation emulation: arithmetic shift rounds down, so a negative offset
/// with a nonzero remainder is incremented once.
///
/// # Original
/// - Ghidra: `FUN_80239d30` at `0x80239D30` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239d30_FUN_80239d30.c`
///
/// # Arguments
/// - `dict_ptr`: address of a word holding the dictionary base.
/// - `name`: address of the NUL-terminated resource name.
///
/// # Returns
/// Zero-based index, or `-1` when not found.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The dictionary and `name` must be readable.
///
/// # Confidence
/// high - the lookup and the signed index arithmetic are explicit.
pub unsafe fn res_dict_name_index(dict_ptr: usize, name: u32) -> i32 {
    res_dict_name_index_value(platform::read_u32(dict_ptr), name)
}

/// Returns a key-record resource's zero-based dictionary index.
///
/// Same index arithmetic as [`res_dict_name_index`], using the tag-record
/// lookup instead of `strcmp`-on-C-string.
///
/// # Original
/// - Ghidra: `FUN_80239dc0` at `0x80239DC0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239dc0_FUN_80239dc0.c`
///
/// # Arguments
/// - `dict_ptr`: address of a word holding the dictionary base.
/// - `key_ptr`: address of a word holding the key-record pointer.
///
/// # Returns
/// Zero-based index, or `-1` when not found.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// The dictionary and key record must be readable.
///
/// # Confidence
/// high - the lookup and index arithmetic are explicit.
pub unsafe fn res_dict_tag_index(dict_ptr: usize, key_ptr: usize) -> i32 {
    res_dict_record_index_value(
        platform::read_u32(dict_ptr),
        platform::read_u32(key_ptr),
    )
}

/// Returns a named entry from a dictionary value (internal helper).
///
/// # Original
/// - Inlined in `FUN_80239c40` at `0x80239C40`.
/// - Evidence: `work/ghidra/main-gb/export/80239c40_FUN_80239c40.c`
///
/// # Safety
/// `dict` must be 0 or a readable dictionary; `name` must be 0 or a C string.
///
/// # Confidence
/// high - the helper is the exact body of the original after its guards.
unsafe fn res_dict_get_named_value(dict: u32, name: u32) -> u32 {
    if dict == 0 || name == 0 {
        return 0;
    }
    let len = platform::call1(STRLEN, name);
    let mut dict_word = crate::platform::GuestLocal::new( dict);
    let node = res_dict_find_name((dict_word.as_mut_ptr() as *mut u32) as usize, name as usize, len);
    if node == 0 {
        return 0;
    }
    dict.wrapping_add(platform::read_u32(node as usize + RES_DICT_DATA))
}

/// Returns a key-record entry from a dictionary value (internal helper).
///
/// # Original
/// - Inlined in `FUN_80239cc0` at `0x80239CC0`.
/// - Evidence: `work/ghidra/main-gb/export/80239cc0_FUN_80239cc0.c`
///
/// # Safety
/// `dict` must be 0 or a readable dictionary; `key` must be 0 or a name record.
///
/// # Confidence
/// high - the helper is the exact body of the original after its guards.
unsafe fn res_dict_get_record_value(dict: u32, key: u32) -> u32 {
    if dict == 0 || key == 0 {
        return 0;
    }
    let mut dict_word = crate::platform::GuestLocal::new( dict);
    let mut key_word = crate::platform::GuestLocal::new( key);
    let node = res_dict_find(
        (dict_word.as_mut_ptr() as *mut u32) as usize,
        (key_word.as_mut_ptr() as *mut u32) as usize,
    );
    if node == 0 {
        return 0;
    }
    dict.wrapping_add(platform::read_u32(node as usize + RES_DICT_DATA))
}

/// Returns a named entry's index from a dictionary value (internal helper).
///
/// # Original
/// - Inlined in `FUN_80239d30` at `0x80239D30`.
/// - Evidence: `work/ghidra/main-gb/export/80239d30_FUN_80239d30.c`
///
/// # Safety
/// `dict` must be 0 or a readable dictionary; `name` must be 0 or a C string.
///
/// # Confidence
/// high - the helper mirrors the original's lookup and subtraction.
unsafe fn res_dict_name_index_value(dict: u32, name: u32) -> i32 {
    if dict == 0 || name == 0 {
        return -1;
    }
    let len = platform::call1(STRLEN, name);
    let mut dict_word = crate::platform::GuestLocal::new( dict);
    let node = res_dict_find_name((dict_word.as_mut_ptr() as *mut u32) as usize, name as usize, len);
    if node == 0 {
        return -1;
    }
    res_dict_node_index(dict as usize, node as usize)
}

/// Returns a key-record entry's index from a dictionary value (internal helper).
///
/// # Original
/// - Inlined in `FUN_80239dc0` at `0x80239DC0`.
/// - Evidence: `work/ghidra/main-gb/export/80239dc0_FUN_80239dc0.c`
///
/// # Safety
/// `dict` must be 0 or a readable dictionary; `key` must be 0 or a name record.
///
/// # Confidence
/// high - the helper mirrors the original's lookup and subtraction.
unsafe fn res_dict_record_index_value(dict: u32, key: u32) -> i32 {
    if dict == 0 || key == 0 {
        return -1;
    }
    let mut dict_word = crate::platform::GuestLocal::new( dict);
    let mut key_word = crate::platform::GuestLocal::new( key);
    let node = res_dict_find(
        (dict_word.as_mut_ptr() as *mut u32) as usize,
        (key_word.as_mut_ptr() as *mut u32) as usize,
    );
    if node == 0 {
        return -1;
    }
    res_dict_node_index(dict as usize, node as usize)
}

/// Converts a dictionary node address to its zero-based resource index.
///
/// # Original
/// - Inlined in `FUN_80239d30` and `FUN_80239dc0`.
/// - Evidence: `work/ghidra/main-gb/export/80239d30_FUN_80239d30.c`
///
/// # Safety
/// `dict` must be the dictionary base and `node` one of its node addresses.
///
/// # Confidence
/// high - the original subtraction and sign correction are explicit.
unsafe fn res_dict_node_index(dict: usize, node: usize) -> i32 {
    let delta = (node as u32).wrapping_sub((dict + 0x18) as u32);
    let mut index = (delta as i32) >> 4;
    if (delta as i32) < 0 && (delta & RES_INDEX_MASK) != 0 {
        index += 1;
    }
    index
}

// ===========================================================================
// BRRES typed section access, 0x80239EA0..0x8023A270
// ===========================================================================

/// Looks up a palette entry by a length-prefixed name record.
///
/// The `Palettes(NW4R)` section is found from the root dictionary, then the
/// record is resolved inside it.
///
/// # Original
/// - Ghidra: `FUN_80239ea0` at `0x80239EA0` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239ea0_FUN_80239ea0.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `key_ptr`: address of a word holding the palette name record pointer.
///
/// # Returns
/// Palette resource pointer, or 0.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `key_ptr` must be readable.
///
/// # Confidence
/// high - both section and entry lookups are explicit.
pub unsafe fn brres_get_palette_record(this: usize, key_ptr: usize) -> u32 {
    let section = brres_section_value(this, BRRES_TYPE_PALETTES);
    if section == 0 {
        return 0;
    }
    let mut section_word = crate::platform::GuestLocal::new( section);
    let mut key_word = crate::platform::GuestLocal::new( platform::read_u32(key_ptr));
    res_dict_get_section(
        (section_word.as_mut_ptr() as *mut u32) as usize,
        (key_word.as_mut_ptr() as *mut u32) as usize,
    )
}

/// Looks up a texture by its C-string name.
///
/// The `Textures(NW4R)` section is found from the root dictionary, then the
/// name is resolved with the ordinary string dictionary lookup.
///
/// # Original
/// - Ghidra: `FUN_80239f20` at `0x80239F20` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239f20_FUN_80239f20.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `name`: address of the NUL-terminated texture name.
///
/// # Returns
/// TEX0 resource pointer, or 0.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `name` must be a C string.
///
/// # Confidence
/// high - the tag and lookup are explicit.
pub unsafe fn brres_get_texture(this: usize, name: u32) -> u32 {
    brres_named_value(this, BRRES_TYPE_TEXTURES, name)
}

/// Looks up a texture by a length-prefixed name record.
///
/// # Original
/// - Ghidra: `FUN_80239f90` at `0x80239F90` (116 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239f90_FUN_80239f90.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `key_ptr`: address of a word holding the texture name record pointer.
///
/// # Returns
/// TEX0 resource pointer, or 0.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `key_ptr` must be readable.
///
/// # Confidence
/// high - both lookups are explicit.
pub unsafe fn brres_get_texture_record(this: usize, key_ptr: usize) -> u32 {
    let section = brres_section_value(this, BRRES_TYPE_TEXTURES);
    if section == 0 {
        return 0;
    }
    let mut section_word = crate::platform::GuestLocal::new( section);
    let mut key_word = crate::platform::GuestLocal::new( platform::read_u32(key_ptr));
    res_dict_get_section(
        (section_word.as_mut_ptr() as *mut u32) as usize,
        (key_word.as_mut_ptr() as *mut u32) as usize,
    )
}

/// Returns a texture by its index in the `Textures(NW4R)` section.
///
/// Index `i` uses resource node `i + 1`; the node's data offset is added to
/// the section base. This path does **not** check the section's entry count.
///
/// # Original
/// - Ghidra: `FUN_8023a010` at `0x8023A010` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a010_FUN_8023a010.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `index`: zero-based texture index.
///
/// # Returns
/// TEX0 resource pointer, or 0 when the section or entry offset is zero.
///
/// # Side effects
/// None; read-only.
///
/// # Safety
/// `*this` must point at a readable BRRES image and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the null checks and offset add are explicit.
pub unsafe fn brres_texture_by_index(this: usize, index: u32) -> u32 {
    let section = brres_section_value(this, BRRES_TYPE_TEXTURES);
    if section == 0 {
        return 0;
    }
    let offset = platform::read_u32(
        section as usize + (index as usize + 1) * RES_DICT_NODE + RES_SECTION_ENTRY_BASE,
    );
    if offset == 0 {
        0
    } else {
        section.wrapping_add(offset)
    }
}

/// Looks up an `AnmChr(NW4R)` animation by name.
///
/// # Original
/// - Ghidra: `FUN_8023a0b0` at `0x8023A0B0` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a0b0_FUN_8023a0b0.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `name`: animation name.
///
/// # Returns
/// CHR0 resource pointer, or 0.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `name` must be a C string.
///
/// # Confidence
/// high - the section tag and named lookup are explicit.
pub unsafe fn brres_get_anm_chr(this: usize, name: u32) -> u32 {
    brres_named_value(this, BRRES_TYPE_ANM_CHR, name)
}

/// Looks up an `AnmVis(NW4R)` animation by name.
///
/// # Original
/// - Ghidra: `FUN_8023a120` at `0x8023A120` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a120_FUN_8023a120.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `name`: animation name.
///
/// # Returns
/// VIS0 resource pointer, or 0.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `name` must be a C string.
///
/// # Confidence
/// high - the section tag and named lookup are explicit.
pub unsafe fn brres_get_anm_vis(this: usize, name: u32) -> u32 {
    brres_named_value(this, BRRES_TYPE_ANM_VIS, name)
}

/// Looks up an `AnmClr(NW4R)` animation by name.
///
/// # Original
/// - Ghidra: `FUN_8023a190` at `0x8023A190` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a190_FUN_8023a190.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `name`: animation name.
///
/// # Returns
/// CLR0 resource pointer, or 0.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `name` must be a C string.
///
/// # Confidence
/// high - the section tag and named lookup are explicit.
pub unsafe fn brres_get_anm_clr(this: usize, name: u32) -> u32 {
    brres_named_value(this, BRRES_TYPE_ANM_CLR, name)
}

/// Looks up an `AnmTexPat(NW4R)` animation by name.
///
/// # Original
/// - Ghidra: `FUN_8023a200` at `0x8023A200` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a200_FUN_8023a200.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `name`: animation name.
///
/// # Returns
/// PAT0 resource pointer, or 0.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `name` must be a C string.
///
/// # Confidence
/// high - the section tag and named lookup are explicit.
pub unsafe fn brres_get_anm_tex_pat(this: usize, name: u32) -> u32 {
    brres_named_value(this, BRRES_TYPE_ANM_TEX_PAT, name)
}

/// Looks up an `AnmTexSrt(NW4R)` animation by name.
///
/// # Original
/// - Ghidra: `FUN_8023a270` at `0x8023A270` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a270_FUN_8023a270.c`
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
/// - `name`: animation name.
///
/// # Returns
/// SRT0 resource pointer, or 0.
///
/// # Safety
/// `*this` must point at a readable BRRES image; `name` must be a C string.
///
/// # Confidence
/// high - the section tag and named lookup are explicit.
pub unsafe fn brres_get_anm_tex_srt(this: usize, name: u32) -> u32 {
    brres_named_value(this, BRRES_TYPE_ANM_TEX_SRT, name)
}

/// Returns the first resource body of a BRRES section (internal helper).
///
/// # Original
/// - Inlined in `FUN_80239ea0`, `FUN_80239f20`, `FUN_80239f90`, `FUN_8023a0b0`
///   through `FUN_8023a270`, `FUN_8023a590` and `FUN_8023a860`.
/// - Evidence: `work/ghidra/main-gb/export/8023a860_FUN_8023a860.c`
///
/// # Safety
/// `this` must address a word holding a readable BRRES image.
///
/// # Confidence
/// high - the root-dictionary offset is explicit in every listed function.
unsafe fn brres_section_value(this: usize, tag: u32) -> u32 {
    let file = platform::read_u32(this) as usize;
    res_dict_get_record_value((file + BRRES_ROOT_DICT) as u32, tag)
}

/// Looks up a named entry in one BRRES section (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023a0b0` through `FUN_8023a270`.
/// - Evidence: `work/ghidra/main-gb/export/8023a0b0_FUN_8023a0b0.c`
///
/// # Safety
/// `this` must address a word holding a readable BRRES image; `name` a C string.
///
/// # Confidence
/// high - the section lookup plus named lookup is the entire body.
unsafe fn brres_named_value(this: usize, tag: u32, name: u32) -> u32 {
    let section = brres_section_value(this, tag);
    if section == 0 {
        return 0;
    }
    res_dict_get_named_value(section, name)
}

/// Returns a section's declared entry count (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023a590` and `FUN_8023a860`.
/// - Evidence: `work/ghidra/main-gb/export/8023a590_FUN_8023a590.c`
///
/// # Safety
/// `this` must address a word holding a readable BRRES image.
///
/// # Confidence
/// high - every repetition reads `section + 4`.
unsafe fn brres_section_count(this: usize, tag: u32) -> u32 {
    let section = brres_section_value(this, tag);
    if section == 0 {
        0
    } else {
        platform::read_u32(section as usize + RES_DICT_COUNT)
    }
}

/// Returns one indexed resource body from a BRRES section (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023a590` and `FUN_8023a860`.
/// - Evidence: `work/ghidra/main-gb/export/8023a590_FUN_8023a590.c`
///
/// # Safety
/// `this` must address a word holding a readable BRRES image; `index` must be
/// below the section's declared count.
///
/// # Confidence
/// high - the node-table offset arithmetic is repeated literally in the C.
unsafe fn brres_section_entry(this: usize, tag: u32, index: u32) -> u32 {
    let section = brres_section_value(this, tag);
    if section == 0 {
        return 0;
    }
    let offset = platform::read_u32(
        section as usize + (index as usize + 1) * RES_DICT_NODE + RES_SECTION_ENTRY_BASE,
    );
    if offset == 0 {
        0
    } else {
        section.wrapping_add(offset)
    }
}

// ===========================================================================
// BRRES validation and binding, 0x8023A590 and 0x8023A860
// ===========================================================================

/// Checks every section revision in a BRRES file.
///
/// The function reads the root dictionary from `*this + 0x18` and validates
/// each section that is present:
///
/// | section | accepted revision | check |
/// |---|---|---|
/// | `3DModels(NW4R)` | 11 | resource word `+0x08` |
/// | `Textures(NW4R)` | - | `0x8023ECB0` per texture |
/// | `Palettes(NW4R)` | - | `0x8023EB00` per palette |
/// | `AnmChr(NW4R)` | 5 | resource word `+0x08` |
/// | `AnmVis(NW4R)` | 4 | resource word `+0x08` |
/// | `AnmClr(NW4R)` | 4 | resource word `+0x08` |
/// | `AnmTexPat(NW4R)` | 4 | resource word `+0x08` |
/// | `AnmTexSrt(NW4R)` | 5 | resource word `+0x08` |
/// | `AnmShp(NW4R)` | 4 | resource word `+0x08` |
/// | `AnmScn(NW4R)` | 5 | resource word `+0x08` |
///
/// Missing sections contribute zero entries and are accepted.
///
/// # Original
/// - Ghidra: `FUN_8023a860` at `0x8023A860` (2,380 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a860_FUN_8023a860.c`
/// - `// inferred:` the routine has no Ghidra parameters because `r3` is the
///   caller's `this` pointer and is preserved across the outlined
///   `__savegprlr` prologue at `0x802DCD74`.
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
///
/// # Returns
/// `true` when every present section has an accepted revision.
///
/// # Side effects
/// Calls the texture/palette validators; no writes.
///
/// # Safety
/// `*this` must point at a readable BRRES image with complete node tables.
///
/// # Confidence
/// medium - the revisions and control flow are explicit; the `this` argument
/// and the two validator roles are reconstructed from the prologue and call
/// sites.
pub unsafe fn brres_check_revision(this: usize) -> bool {
    brres_check_version(this, BRRES_TYPE_MODELS, 11)
        && brres_check_entries(this, BRRES_TYPE_TEXTURES, RES_TEXTURE_CHECK)
        && brres_check_entries(this, BRRES_TYPE_PALETTES, RES_PALETTE_CHECK)
        && brres_check_version(this, BRRES_TYPE_ANM_CHR, 5)
        && brres_check_version(this, BRRES_TYPE_ANM_VIS, 4)
        && brres_check_version(this, BRRES_TYPE_ANM_CLR, 4)
        && brres_check_version(this, BRRES_TYPE_ANM_TEX_PAT, 4)
        && brres_check_version(this, BRRES_TYPE_ANM_TEX_SRT, 5)
        && brres_check_version(this, BRRES_TYPE_ANM_SHP, 4)
        && brres_check_version(this, BRRES_TYPE_ANM_SCN, 5)
}

/// Checks the revision word of every entry in one section (internal helper).
///
/// # Original
/// - Inlined eight times in `FUN_8023a860` at `0x8023A860`.
/// - Evidence: `work/ghidra/main-gb/export/8023a860_FUN_8023a860.c`
///
/// # Safety
/// `this` must address a word holding a readable BRRES image.
///
/// # Confidence
/// high - the repeated loop is identical for all eight sections.
unsafe fn brres_check_version(this: usize, tag: u32, expected: u32) -> bool {
    let count = brres_section_count(this, tag);
    for index in 0..count {
        let entry = brres_section_entry(this, tag, index);
        if platform::read_u32(entry as usize + 8) != expected {
            return false;
        }
    }
    true
}

/// Runs a section-specific validator over every entry (internal helper).
///
/// # Original
/// - Inlined for textures and palettes in `FUN_8023a860` at `0x8023A860`.
/// - Evidence: `work/ghidra/main-gb/export/8023a860_FUN_8023a860.c`
///
/// # Safety
/// `this` must address a word holding a readable BRRES image; `check` must be
/// a routine accepting a pointer to one entry word.
///
/// # Confidence
/// medium - the call shape is explicit; the exact validator semantics are
/// outside this lane.
unsafe fn brres_check_entries(this: usize, tag: u32, check: usize) -> bool {
    let count = brres_section_count(this, tag);
    for index in 0..count {
        let mut entry = crate::platform::GuestLocal::new( brres_section_entry(this, tag, index));
        if platform::call1(check, (entry.as_mut_ptr() as *mut u32) as u32) == 0 {
            return false;
        }
    }
    true
}

/// Binds/registers every model, texture and palette in a BRRES file.
///
/// The three loops walk `3DModels(NW4R)`, `Textures(NW4R)` and
/// `Palettes(NW4R)` in that order and hand each resource body to the
/// corresponding G3D binding routine.
///
/// # Original
/// - Ghidra: `FUN_8023a590` at `0x8023A590` (708 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a590_FUN_8023a590.c`
/// - `// inferred:` the routine has no Ghidra parameters because `r3` is the
///   caller's `this` pointer and is preserved across the outlined
///   `__savegprlr` prologue at `0x802DCD74`.
///
/// # Arguments
/// - `this`: address of the word that holds the BRRES file pointer.
///
/// # Side effects
/// Each binding routine records the resource in the G3D engine and may write
/// engine-side state.
///
/// # Safety
/// `*this` must point at a readable BRRES image; the three callees must be the
/// real G3D binding routines.
///
/// # Confidence
/// medium - the three section walks are explicit; the binding roles are
/// inferred from the per-resource call names and order.
pub unsafe fn brres_bind_all(this: usize) {
    let models = brres_section_count(this, BRRES_TYPE_MODELS);
    for index in 0..models {
        let mut entry = crate::platform::GuestLocal::new( brres_section_entry(this, BRRES_TYPE_MODELS, index));
        mdl_model_bind((entry.as_mut_ptr() as *mut u32) as usize);
    }

    let textures = brres_section_count(this, BRRES_TYPE_TEXTURES);
    for index in 0..textures {
        let mut entry = crate::platform::GuestLocal::new( brres_section_entry(this, BRRES_TYPE_TEXTURES, index));
        platform::call1(RES_TEXTURE_BIND, (entry.as_mut_ptr() as *mut u32) as u32);
    }

    let palettes = brres_section_count(this, BRRES_TYPE_PALETTES);
    for index in 0..palettes {
        let mut entry = crate::platform::GuestLocal::new( brres_section_entry(this, BRRES_TYPE_PALETTES, index));
        platform::call2(RES_PALETTE_BIND, (entry.as_mut_ptr() as *mut u32) as u32, 0);
    }
}

// ===========================================================================
// MDL0 payload access, 0x8023B1B0..0x8023B3A0
// ===========================================================================

/// Looks up a named node tree (`NodeTree` or `DrawOpa`) in a model.
///
/// # Original
/// - Ghidra: `FUN_8023b1b0` at `0x8023B1B0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b1b0_FUN_8023b1b0.c`
/// - `// inferred:` Ghidra drops the second argument; callers such as
///   `FUN_80250e70` pass a name string in `r4`.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `name`: node-tree name.
///
/// # Returns
/// Named node-tree resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body with a node-tree
/// dictionary at `+0x10`.
///
/// # Confidence
/// medium - the offset and lookup are explicit; the dropped argument is
/// recovered from call sites.
pub unsafe fn mdl_get_node_tree(model_ptr: usize, name: u32) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_NODE_TREE);
    res_dict_get_named_value(section, name)
}

/// Looks up a named bone/node in a model.
///
/// # Original
/// - Ghidra: `FUN_8023b200` at `0x8023B200` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b200_FUN_8023b200.c`
/// - `// inferred:` Ghidra drops the second argument; callers such as
///   `FUN_80033650` pass `"leg_left"` / `"leg_right"` in `r4`.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `name`: bone/node name.
///
/// # Returns
/// Named bone/node resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body with a bone dictionary at
/// `+0x14`.
///
/// # Confidence
/// medium - the offset and lookup are explicit; the dropped argument is
/// recovered from call sites.
pub unsafe fn mdl_get_node(model_ptr: usize, name: u32) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_NODES);
    res_dict_get_named_value(section, name)
}

/// Looks up a bone/node by a length-prefixed name record.
///
/// # Original
/// - Ghidra: `FUN_8023b250` at `0x8023B250` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b250_FUN_8023b250.c`
/// - `// inferred:` Ghidra prints `void`, but `FUN_80248e00` tests the result.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `key_ptr`: address of a word holding the bone name record pointer.
///
/// # Returns
/// Bone/node resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body; `key_ptr` must be readable.
///
/// # Confidence
/// medium - the offset and record lookup are explicit; the return type is
/// inferred from a caller.
pub unsafe fn mdl_get_node_record(model_ptr: usize, key_ptr: usize) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_NODES);
    if section == 0 {
        return 0;
    }
    res_dict_get_record_value(section, platform::read_u32(key_ptr))
}

/// Returns a bone/node by its index.
///
/// # Original
/// - Ghidra: `FUN_8023b2a0` at `0x8023B2A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b2a0_FUN_8023b2a0.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based bone index.
///
/// # Returns
/// Bone/node resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal.
pub unsafe fn mdl_node_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_NODES, index)
}

/// Returns a bone/node by its index (duplicate entry point).
///
/// The body is byte-identical to [`mdl_node_by_index`]; the shipped build has
/// two copies at `0x8023B2A0` and `0x8023B300` because different translation
/// units emitted the same accessor.
///
/// # Original
/// - Ghidra: `FUN_8023b300` at `0x8023B300` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b300_FUN_8023b300.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based bone index.
///
/// # Returns
/// Bone/node resource pointer, or 0.
///
/// # Safety
/// Same requirements as [`mdl_node_by_index`].
///
/// # Confidence
/// high - identical control flow to `0x8023B2A0`.
pub unsafe fn mdl_node_by_index_alt(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_NODES, index)
}

/// Returns the number of bones/nodes in a model.
///
/// # Original
/// - Ghidra: `FUN_8023b360` at `0x8023B360` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b360_FUN_8023b360.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Returns
/// Entry count from the bone dictionary, or 0 when the dictionary is absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body.
///
/// # Confidence
/// high - the count load is explicit.
pub unsafe fn mdl_node_count(model_ptr: usize) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_NODES);
    if section == 0 {
        0
    } else {
        platform::read_u32(section as usize + RES_DICT_COUNT)
    }
}

/// Returns a material by its index.
///
/// # Original
/// - Ghidra: `FUN_8023b3a0` at `0x8023B3A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b3a0_FUN_8023b3a0.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based material index.
///
/// # Returns
/// Material resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal.
pub unsafe fn mdl_material_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_MATERIALS, index)
}

/// Returns a model sub-dictionary base (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023b1b0` through `FUN_8023b3a0`.
/// - Evidence: `work/ghidra/main-gb/export/8023b1b0_FUN_8023b1b0.c`
///
/// # Safety
/// `model_ptr` must address a word holding a readable MDL0 body.
///
/// # Confidence
/// high - every accessor repeats the same load-and-add.
unsafe fn mdl_section_value(model_ptr: usize, offset: usize) -> u32 {
    let model = platform::read_u32(model_ptr) as usize;
    let section = platform::read_u32(model + offset);
    if section == 0 {
        0
    } else {
        model.wrapping_add(section as usize) as u32
    }
}

/// Returns one indexed entry from a model sub-dictionary (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023b2a0`, `FUN_8023b300` and `FUN_8023b3a0`.
/// - Evidence: `work/ghidra/main-gb/export/8023b2a0_FUN_8023b2a0.c`
///
/// # Safety
/// `model_ptr` must address a word holding a readable MDL0 body; `index` must
/// select a real entry.
///
/// # Confidence
/// high - the node offset expression is literal in all three functions.
unsafe fn mdl_section_entry(model_ptr: usize, offset: usize, index: u32) -> u32 {
    let section = mdl_section_value(model_ptr, offset);
    if section == 0 {
        return 0;
    }
    let entry = platform::read_u32(
        section as usize
            + index.wrapping_add(1) as usize * RES_DICT_NODE
            + RES_SECTION_ENTRY_BASE,
    );
    if entry == 0 {
        0
    } else {
        section.wrapping_add(entry)
    }
}

// ===========================================================================
// MDL0 extended payload access, 0x8023B460..0x8023BA10
// ===========================================================================

/// Returns the material count of a model.
///
/// # Original
/// - Ghidra: `FUN_8023b460` at `0x8023B460` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b460_FUN_8023b460.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Returns
/// Entry count of the material dictionary (`+0x18`), or 0 when absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body.
///
/// # Confidence
/// high - the dictionary offset and count load are literal.
pub unsafe fn mdl_material_count(model_ptr: usize) -> u32 {
    mdl_section_count(model_ptr, MDL_MATERIALS)
}

/// Returns a shader by its index.
///
/// # Original
/// - Ghidra: `FUN_8023b4a0` at `0x8023B4A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b4a0_FUN_8023b4a0.c`
/// - `// inferred:` the `+0x1C` dictionary is the shader section: callers
///   `FUN_8023c150` / `FUN_8023c280` index it with the material's shader slot
///   (`+0x4A`) and pass the result to the shader state helpers.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based shader index.
///
/// # Returns
/// Shader resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal; the shader role is inferred from
/// two call sites.
pub unsafe fn mdl_shader_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_SHADERS, index)
}

/// Returns the shader count of a model.
///
/// # Original
/// - Ghidra: `FUN_8023b560` at `0x8023B560` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b560_FUN_8023b560.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Returns
/// Entry count of the shader dictionary (`+0x1C`), or 0 when absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body.
///
/// # Confidence
/// high - the dictionary offset and count load are literal.
pub unsafe fn mdl_shader_count(model_ptr: usize) -> u32 {
    mdl_section_count(model_ptr, MDL_SHADERS)
}

/// Returns a texture reference by its index.
///
/// `// observed:` `0x8025CA20` pairs this accessor with the polygon accessor
/// (`0x8023B9B0`) and iterates polygons inside each texture reference. The
/// `+0x20` dictionary itself is keyed by material name in the sampled model.
///
/// # Original
/// - Ghidra: `FUN_8023b5a0` at `0x8023B5A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b5a0_FUN_8023b5a0.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based texture-reference index.
///
/// # Returns
/// Texture-reference resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal; the texture role is confirmed by
/// the `0x8023E8C0` texture accessor in the same call site.
pub unsafe fn mdl_texture_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_TEXTURES, index)
}

/// Returns the texture-reference count of a model.
///
/// # Original
/// - Ghidra: `FUN_8023b660` at `0x8023B660` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b660_FUN_8023b660.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Returns
/// Entry count of the texture dictionary (`+0x20`), or 0 when absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body.
///
/// # Confidence
/// high - the dictionary offset and count load are literal.
pub unsafe fn mdl_texture_count(model_ptr: usize) -> u32 {
    mdl_section_count(model_ptr, MDL_TEXTURES)
}

/// Returns a palette by its index.
///
/// # Original
/// - Ghidra: `FUN_8023b6a0` at `0x8023B6A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b6a0_FUN_8023b6a0.c`
/// - `// observed:` `0x8023C280` indexes this dictionary with the material's
///   palette slots (`+0x50`..`+0x5E`) and passes each result to
///   `0x8023E960`, the palette accessor.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based palette index.
///
/// # Returns
/// Palette resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal; the palette role is confirmed by
/// the call site.
pub unsafe fn mdl_palette_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_PALETTES, index)
}

/// Returns an entry from the MDL0 dictionary at `+0x2C`.
///
/// # Original
/// - Ghidra: `FUN_8023b700` at `0x8023B700` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b700_FUN_8023b700.c`
/// - `// unknown:` the `+0x2C` word is zero in every sampled model, so the
///   dictionary's contents and purpose are unverified.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based entry index.
///
/// # Returns
/// Entry pointer, or 0 when the dictionary or the entry is absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body; `index` must be valid when
/// the dictionary exists.
///
/// # Confidence
/// high for the control flow; low for the dictionary's role.
pub unsafe fn mdl_section_2c_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_SECTION_2C, index)
}

/// Looks up an entry of the MDL0 `+0x30` dictionary by C-string name.
///
/// # Original
/// - Ghidra: `FUN_8023b760` at `0x8023B760` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b760_FUN_8023b760.c`
/// - `// inferred:` Ghidra drops the second argument; caller `FUN_800ef570`
///   passes a material-name C string in `r4`.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `name`: address of the NUL-terminated entry name.
///
/// # Returns
/// Named entry pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body; `name` must be a readable
/// C string.
///
/// # Confidence
/// medium - lookup is explicit; the dropped argument is recovered from a call
/// site. `// unknown:` the semantic name of the `+0x30` section.
pub unsafe fn mdl_section_30_by_name(model_ptr: usize, name: u32) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_SECTION_30);
    res_dict_get_named_value(section, name)
}

/// Looks up an entry of the MDL0 `+0x30` dictionary by name record.
///
/// # Original
/// - Ghidra: `FUN_8023b7b0` at `0x8023B7B0` (80 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b7b0_FUN_8023b7b0.c`
/// - `// inferred:` Ghidra prints `void`, but caller `FUN_80244190` tests the
///   result and reads the entry's `+0x0C` word.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `key_ptr`: address of a word holding the length-prefixed name record.
///
/// # Returns
/// Entry pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body; `key_ptr` must be readable.
///
/// # Confidence
/// medium - record lookup is explicit; the return type is recovered from a
/// caller.
pub unsafe fn mdl_section_30_by_record(model_ptr: usize, key_ptr: usize) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_SECTION_30);
    if section == 0 {
        return 0;
    }
    res_dict_get_record_value(section, platform::read_u32(key_ptr))
}

/// Returns an entry of the MDL0 `+0x30` dictionary by index.
///
/// # Original
/// - Ghidra: `FUN_8023b800` at `0x8023B800` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b800_FUN_8023b800.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based entry index.
///
/// # Returns
/// Entry pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal.
pub unsafe fn mdl_section_30_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_SECTION_30, index)
}

/// Returns the entry count of the MDL0 `+0x30` dictionary.
///
/// # Original
/// - Ghidra: `FUN_8023b8c0` at `0x8023B8C0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b8c0_FUN_8023b8c0.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Returns
/// Entry count, or 0 when the dictionary is absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body.
///
/// # Confidence
/// high - the count load is literal.
pub unsafe fn mdl_section_30_count(model_ptr: usize) -> u32 {
    mdl_section_count(model_ptr, MDL_SECTION_30)
}

/// Looks up a polygon by C-string name.
///
/// # Original
/// - Ghidra: `FUN_8023b900` at `0x8023B900` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b900_FUN_8023b900.c`
/// - `// observed:` caller `FUN_8008bbd0` passes `polygon0` / `polygon3`; the
///   sampled model keys the `+0x38` dictionary with `polygon0`..`polygon5`.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `name`: address of the NUL-terminated polygon name.
///
/// # Returns
/// Polygon resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body; `name` must be a readable
/// C string.
///
/// # Confidence
/// high - the section keys and the caller's strings agree.
pub unsafe fn mdl_polygon_by_name(model_ptr: usize, name: u32) -> u32 {
    let section = mdl_section_value(model_ptr, MDL_POLYGONS);
    res_dict_get_named_value(section, name)
}

/// Returns a polygon by its index.
///
/// # Original
/// - Ghidra: `FUN_8023b950` at `0x8023B950` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b950_FUN_8023b950.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based polygon index.
///
/// # Returns
/// Polygon resource pointer, or 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - the offset arithmetic is literal.
pub unsafe fn mdl_polygon_by_index(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_POLYGONS, index)
}

/// Returns the polygon count of a model.
///
/// # Original
/// - Ghidra: `FUN_8023ba10` at `0x8023BA10` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ba10_FUN_8023ba10.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Returns
/// Entry count of the polygon dictionary (`+0x38`), or 0 when absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body.
///
/// # Confidence
/// high - the count load is literal.
pub unsafe fn mdl_polygon_count(model_ptr: usize) -> u32 {
    mdl_section_count(model_ptr, MDL_POLYGONS)
}

/// Returns one model sub-dictionary's entry count (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023b460`, `FUN_8023b560`, `FUN_8023b660`,
///   `FUN_8023b8c0` and `FUN_8023ba10`.
/// - Evidence: `work/ghidra/main-gb/export/8023b460_FUN_8023b460.c`
///
/// # Safety
/// `model_ptr` must address a word holding a readable MDL0 body.
///
/// # Confidence
/// high - every count accessor repeats the same load-and-add.
unsafe fn mdl_section_count(model_ptr: usize, offset: usize) -> u32 {
    let section = mdl_section_value(model_ptr, offset);
    if section == 0 {
        0
    } else {
        platform::read_u32(section as usize + RES_DICT_COUNT)
    }
}

// ===========================================================================
// Deferred BRRES resource bind, 0x8023A350..0x8023F720
// ===========================================================================

/// Returns whether every model material and texture-pattern animation in a
/// BRRES file has its texture and palette pointers resolved.
///
/// `// observed:` the body walks `3DModels(NW4R)` first and ANDs the result of
/// [`mdl_bind_check`] for each model. It then walks `AnmTexPat(NW4R)` and
/// assigns (rather than ANDs) the result of [`anm_tex_pat_bind`], so the last
/// animation's result wins.
///
/// # Original
/// - Ghidra: `FUN_8023a350` at `0x8023A350` (564 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023a350_FUN_8023a350.c`
/// - `// inferred:` `r3` and `r4` are preserved by the outlined
///   `__savegprlr_25` prologue at `0x802DCD6C`. `r3` is the address of the
///   word holding the BRRES base; `r4` is the address of the word holding the
///   same base for the child bind/check calls.
///
/// # Arguments
/// - `this`: address of the word holding the BRRES file pointer.
/// - `brres_ptr`: address of a word that holds the BRRES file pointer; passed
///   unchanged to the per-model and per-animation checks.
///
/// # Returns
/// `true` when the bind pass reports success.
///
/// # Side effects
/// Resolves and caches TEX0/palette pointers inside the model and animation
/// resources (see [`mdl_material_bind_check`] and [`anm_tex_pat_bind`]).
///
/// # Safety
/// `*this` must point at a readable BRRES image; `brres_ptr` must address a
/// readable word holding that image's base.
///
/// # Confidence
/// medium - the flow and section tags are explicit; the `r4` hand-off is
/// recovered from the prologue and from the child signatures.
pub unsafe fn brres_bind_check(this: usize, brres_ptr: usize) -> bool {
    let mut ok = true;

    let models = brres_section_count(this, BRRES_TYPE_MODELS);
    let mut index = 0u32;
    while index < models {
        let mut model = crate::platform::GuestLocal::new( brres_section_entry(this, BRRES_TYPE_MODELS, index));
        ok = mdl_bind_check((model.as_mut_ptr() as *mut u32) as usize, brres_ptr) && ok;
        index += 1;
    }

    let animations = brres_section_count(this, BRRES_TYPE_ANM_TEX_PAT);
    index = 0;
    while index < animations {
        let mut animation = crate::platform::GuestLocal::new( brres_section_entry(this, BRRES_TYPE_ANM_TEX_PAT, index));
        // observed: the original overwrites the accumulator here.
        ok = anm_tex_pat_bind((animation.as_mut_ptr() as *mut u32) as usize, brres_ptr);
        index += 1;
    }

    ok
}

/// Returns whether every `+0x30` entry of a model passes
/// [`mdl_material_bind_check`].
///
/// # Original
/// - Ghidra: `FUN_8023ba50` at `0x8023BA50` (268 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ba50_FUN_8023ba50.c`
/// - `// inferred:` `r3` is the address of the word holding the model pointer;
///   `r4` is forwarded unchanged and is the BRRES-base word address used by
///   `0x8023E390`.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `brres_ptr`: address of the word holding the BRRES file pointer.
///
/// # Returns
/// `true` when every entry reports success (also `true` for zero entries).
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body; `brres_ptr` must address a
/// readable word holding the BRRES base.
///
/// # Confidence
/// medium - the loop is explicit; the argument hand-off is inferred from the
/// outlined prologue.
pub unsafe fn mdl_bind_check(model_ptr: usize, brres_ptr: usize) -> bool {
    let section = mdl_section_value(model_ptr, MDL_SECTION_30);
    if section == 0 {
        return true;
    }

    let count = platform::read_u32(section as usize + RES_DICT_COUNT);
    let mut ok = true;
    let mut index = 0u32;
    while index < count {
        let offset = platform::read_u32(
            section as usize
                + (index as usize + 1) * RES_DICT_NODE
                + RES_SECTION_ENTRY_BASE,
        );
        let entry = if offset == 0 {
            0
        } else {
            section.wrapping_add(offset)
        };
        let mut entry_word = crate::platform::GuestLocal::new( entry);
        ok = mdl_material_bind_check((entry_word.as_mut_ptr() as *mut u32) as usize, brres_ptr) && ok;
        index += 1;
    }
    ok
}

/// Returns whether every 0x34-byte material record of one `+0x30` entry was
/// bound by `0x8023E390`.
///
/// # Original
/// - Ghidra: `FUN_8023e250` at `0x8023E250` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e250_FUN_8023e250.c`
/// - `// inferred:` `r3` is the address of the word holding the entry pointer;
///   the count is at entry `+0x2C`, the record table at entry `+0x30`, and the
///   two bitmask-pointer words at `+0x40` / `+0x144`.
///
/// # Arguments
/// - `this`: address of the word holding the `+0x30` entry pointer.
/// - `brres_ptr`: address of the word holding the BRRES file pointer.
///
/// # Returns
/// `true` when every record's bind call returned nonzero.
///
/// # Safety
/// `*this` must point at a readable entry with a valid record table;
/// `brres_ptr` must address a readable word holding the BRRES base.
///
/// # Confidence
/// medium - every load and call is explicit; the entry layout is inferred from
/// this function's own offsets.
pub unsafe fn mdl_material_bind_check(this: usize, brres_ptr: usize) -> bool {
    let base = platform::read_u32(this) as usize;
    let count = platform::read_u32(base + MDL_ENTRY_MATERIAL_COUNT);
    let table_offset = platform::read_u32(base + MDL_ENTRY_MATERIAL_TABLE);
    let table = if table_offset == 0 {
        0usize
    } else {
        base.wrapping_add(table_offset as usize)
    };

    // The original passes the addresses of two locals that hold these pointer
    // values; `0x8023E390` dereferences each once.
    let mut tex_bitset = crate::platform::GuestLocal::new( base.wrapping_add(MDL_ENTRY_TEX_BITSET) as u32);
    let mut plt_bitset = crate::platform::GuestLocal::new( base.wrapping_add(MDL_ENTRY_PLT_BITSET) as u32);

    let mut bound = 0u32;
    let mut index = 0u32;
    while index < count {
        let mut material = crate::platform::GuestLocal::new( table.wrapping_add(index as usize * MDL_MATERIAL_STRIDE) as u32);
        let result = platform::call4(
            RES_MATERIAL_BIND,
            (material.as_mut_ptr() as *mut u32) as u32,
            brres_ptr as u32,
            (tex_bitset.as_mut_ptr() as *mut u32) as u32,
            (plt_bitset.as_mut_ptr() as *mut u32) as u32,
        );
        if result != 0 {
            bound += 1;
        }
        index += 1;
    }
    bound == count
}

/// Resolves the texture and palette names of one `AnmTexPat` animation.
///
/// The animation object holds two parallel tables: name offsets and cached
/// resource pointers. For every non-empty cache slot the pointer is kept; for
/// every empty slot the length-prefixed name is looked up in the BRRES
/// `Textures(NW4R)` / `Palettes(NW4R)` sections and the result is written back.
///
/// # Original
/// - Ghidra: `FUN_8023f720` at `0x8023F720` (368 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f720_FUN_8023f720.c`
/// - `// inferred:` `r3` is the address of the word holding the animation
///   object pointer; `r4` is the address of the word holding the BRRES base
///   and is forwarded to `0x80239F90` / `0x80239EA0`.
///
/// # Arguments
/// - `this`: address of the word holding the `AnmTexPat` resource pointer.
/// - `brres_ptr`: address of the word holding the BRRES file pointer.
///
/// # Returns
/// `true` when every texture and palette name resolved (or was already
/// cached).
///
/// # Side effects
/// Writes resolved TEX0/palette pointers into the animation's slot tables.
///
/// # Safety
/// `*this` must point at a readable animation object whose tables cover the
/// declared counts; `brres_ptr` must address a readable word holding the BRRES
/// base.
///
/// # Confidence
/// medium - the two loops are explicit; the object layout names are inferred
/// from the paired accessors.
pub unsafe fn anm_tex_pat_bind(this: usize, brres_ptr: usize) -> bool {
    let object = platform::read_u32(this) as usize;
    let tex_count = platform::read_u16(object + ANM_TEX_COUNT) as u32;
    let plt_count = platform::read_u16(object + ANM_PLT_COUNT) as u32;

    let tex_names = resource_table(object, ANM_TEX_NAMES);
    let plt_names = resource_table(object, ANM_PLT_NAMES);
    let tex_slots = resource_table(object, ANM_TEX_SLOTS);
    let plt_slots = resource_table(object, ANM_PLT_SLOTS);

    let mut resolved = 0u32;
    let mut index = 0u32;
    while index < tex_count {
        let slot = tex_slots + index as usize * 4;
        if platform::read_u32(slot) == 0 {
            let mut key = crate::platform::GuestLocal::new( tex_names
                .wrapping_add(platform::read_u32(tex_names + index as usize * 4) as usize)
                .wrapping_sub(4) as u32);
            let record = brres_get_texture_record(brres_ptr, (key.as_mut_ptr() as *mut u32) as usize);
            if record != 0 {
                platform::write_u32(slot, record);
                resolved += 1;
            }
        } else {
            resolved += 1;
        }
        index += 1;
    }

    index = 0;
    while index < plt_count {
        let slot = plt_slots + index as usize * 4;
        if platform::read_u32(slot) == 0 {
            let mut key = crate::platform::GuestLocal::new( plt_names
                .wrapping_add(platform::read_u32(plt_names + index as usize * 4) as usize)
                .wrapping_sub(4) as u32);
            let record = brres_get_palette_record(brres_ptr, (key.as_mut_ptr() as *mut u32) as usize);
            if record != 0 {
                platform::write_u32(slot, record);
                resolved += 1;
            }
        } else {
            resolved += 1;
        }
        index += 1;
    }

    resolved == tex_count + plt_count
}

/// Returns a table address stored as an offset in a resource object.
///
/// # Original
/// - Inlined in `FUN_8023f720` at `0x8023F720` (four repetitions).
/// - Evidence: `work/ghidra/main-gb/export/8023f720_FUN_8023f720.c`
///
/// # Safety
/// `base` must be readable and `offset` must be one of the table offsets.
///
/// # Confidence
/// high - all four fields use the same `if zero then 0 else base + value`
/// shape.
unsafe fn resource_table(base: usize, offset: usize) -> usize {
    let delta = platform::read_u32(base + offset);
    if delta == 0 {
        0
    } else {
        base.wrapping_add(delta as usize)
    }
}

/// Flushes or publishes one palette resource.
///
/// `mode != 0` selects the cache-store-plus-sync path; `mode == 0` selects the
/// plain cache-store path. The second path receives the palette data pointer
/// and its size word.
///
/// # Original
/// - Ghidra: `FUN_8023eae0` at `0x8023EAE0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023eae0_FUN_8023eae0.c`
/// - `// unknown:` Ghidra renders the `mode != 0` call as `FUN_801ac4a0()`
///   with no arguments, although the callee itself takes `(data, size)`. The
///   body below therefore calls it with no explicit arguments rather than
///   inventing a pair.
///
/// # Arguments
/// - `this`: address of the word holding the palette data pointer.
/// - `mode`: nonzero selects the synchronising cache path.
///
/// # Side effects
/// Executes data-cache block operations over the palette bytes.
///
/// # Safety
/// `*this` must point at readable palette bytes; `+0x04` must hold the readable
/// byte size.
///
/// # Confidence
/// medium - the `mode == 0` path is explicit; the other path's arguments are
/// unresolved in the Ghidra output.
pub unsafe fn res_palette_bind(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize + 4);
        platform::call2(DC_STORE, data, size);
    }
}

/// Returns whether a palette resource revision is acceptable.
///
/// `// observed:` the accepted values are 1 and 3.
///
/// # Original
/// - Ghidra: `FUN_8023eb00` at `0x8023EB00` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023eb00_FUN_8023eb00.c`
///
/// # Arguments
/// - `this`: address of the word holding the palette resource pointer.
///
/// # Returns
/// `true` when the revision word at `+0x08` is 1 or 3.
///
/// # Safety
/// `*this` must point at a readable palette resource header.
///
/// # Confidence
/// high - the two comparisons are literal.
pub unsafe fn res_palette_revision_ok(this: usize) -> bool {
    let revision = platform::read_u32(platform::read_u32(this) as usize + 8);
    revision == 1 || revision == 3
}

/// Returns whether a texture resource revision is acceptable.
///
/// `// observed:` the body is byte-identical to
/// [`res_palette_revision_ok`]; the accepted values are 1 and 3.
///
/// # Original
/// - Ghidra: `FUN_8023ecb0` at `0x8023ECB0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ecb0_FUN_8023ecb0.c`
///
/// # Arguments
/// - `this`: address of the word holding the texture resource pointer.
///
/// # Returns
/// `true` when the revision word at `+0x08` is 1 or 3.
///
/// # Safety
/// `*this` must point at a readable texture resource header.
///
/// # Confidence
/// high - the two comparisons are literal.
pub unsafe fn res_texture_revision_ok(this: usize) -> bool {
    let revision = platform::read_u32(platform::read_u32(this) as usize + 8);
    revision == 1 || revision == 3
}

/// Flushes one texture resource's bytes.
///
/// # Original
/// - Ghidra: `FUN_8023ece0` at `0x8023ECE0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ece0_FUN_8023ece0.c`
///
/// # Arguments
/// - `this`: address of the word holding the texture data pointer.
///
/// # Side effects
/// Executes a data-cache block flush over the texture bytes.
///
/// # Safety
/// `*this` must point at readable texture bytes; `+0x04` must hold the readable
/// byte size.
///
/// # Confidence
/// high - pointer, size and call target are explicit.
pub unsafe fn res_texture_bind(this: usize) {
    let data = platform::read_u32(this);
    let size = platform::read_u32(data as usize + 4);
    platform::call2(DC_FLUSH, data, size);
}

// ===========================================================================
// MDL0 payload access and material binding, 0x8023B400..0x8023F890
// ===========================================================================

/// Returns a material by index (alternate entry point).
///
/// The body is byte-identical to [`mdl_material_by_index`] at
/// `0x8023B3A0`; the shipped build contains two copies because different
/// translation units emitted the same accessor. It reads the `+0x18`
/// dictionary of the model and resolves entry `index` with the standard
/// `section + (index + 1) * 0x10 + 0x14` formula.
///
/// # Original
/// - Ghidra: `FUN_8023b400` at `0x8023B400` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b400_FUN_8023b400.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based material index.
///
/// # Returns
/// Material resource pointer, or 0 when the dictionary or entry offset is 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - identical control flow to `0x8023B3A0`.
pub unsafe fn mdl_material_by_index_alt(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_MATERIALS, index)
}

/// Returns a shader by index (alternate entry point).
///
/// The body is byte-identical to [`mdl_shader_by_index`] at `0x8023B4A0`;
/// it indexes the model's `+0x1C` dictionary.
///
/// # Original
/// - Ghidra: `FUN_8023b500` at `0x8023B500` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b500_FUN_8023b500.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based shader index.
///
/// # Returns
/// Shader resource pointer, or 0 when the dictionary or entry offset is 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - identical control flow to `0x8023B4A0`.
pub unsafe fn mdl_shader_by_index_alt(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_SHADERS, index)
}

/// Returns a texture reference by index (alternate entry point).
///
/// The body is byte-identical to [`mdl_texture_by_index`] at `0x8023B5A0`;
/// it indexes the model's `+0x20` dictionary.
///
/// # Original
/// - Ghidra: `FUN_8023b600` at `0x8023B600` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b600_FUN_8023b600.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based texture-reference index.
///
/// # Returns
/// Texture-reference resource pointer, or 0 when the entry is absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - identical control flow to `0x8023B5A0`.
pub unsafe fn mdl_texture_by_index_alt(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_TEXTURES, index)
}

/// Returns an entry of the MDL0 `+0x30` dictionary by index (alternate entry).
///
/// The body is byte-identical to [`mdl_section_30_by_index`] at
/// `0x8023B800`. `// unknown:` the semantic name of the `+0x30` section.
///
/// # Original
/// - Ghidra: `FUN_8023b860` at `0x8023B860` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b860_FUN_8023b860.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based entry index.
///
/// # Returns
/// Entry pointer, or 0 when the dictionary or entry offset is 0.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - identical control flow to `0x8023B800`.
pub unsafe fn mdl_section_30_by_index_alt(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_SECTION_30, index)
}

/// Returns a polygon by index (alternate entry point).
///
/// The body is byte-identical to [`mdl_polygon_by_index`] at `0x8023B950`;
/// it indexes the model's `+0x38` dictionary.
///
/// # Original
/// - Ghidra: `FUN_8023b9b0` at `0x8023B9B0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023b9b0_FUN_8023b9b0.c`
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
/// - `index`: zero-based polygon index.
///
/// # Returns
/// Polygon resource pointer, or 0 when the entry is absent.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body and `index` must select a
/// real entry.
///
/// # Confidence
/// high - identical control flow to `0x8023B950`.
pub unsafe fn mdl_polygon_by_index_alt(model_ptr: usize, index: u32) -> u32 {
    mdl_section_entry(model_ptr, MDL_POLYGONS, index)
}

/// Binds every payload table of one MDL0 model.
///
/// The body is eight repetitions of the same loop over the model header
/// offsets `+0x30`, `+0x38`, `+0x18`, `+0x1C`, `+0x20`, `+0x24`, `+0x28` and
/// `+0x2C`. Each loop reads the dictionary count at `section + 0x04`, resolves
/// entry `i` with `section + (i + 1) * 0x10 + 0x14`, and calls that table's
/// binder:
///
/// | model offset | binder |
/// |---|---|
/// | `+0x30` | `0x8023E320` - clear the entry's 0x180-byte draw cache |
/// | `+0x38` | `0x8023C280` - build polygon GX display lists |
/// | `+0x18` | `0x8023EA20` - cache-publish one material entry |
/// | `+0x1C` | `0x8023EA40` - cache-publish one shader entry |
/// | `+0x20` | `0x8023EA60` - cache-publish one texture-reference entry |
/// | `+0x24` | `0x8023EA80` - cache-publish one palette entry |
/// | `+0x28` | `0x8023EAA0` - cache-publish one `+0x28` entry |
/// | `+0x2C` | `0x8023EAC0` - cache-publish one `+0x2C` entry |
///
/// The model header offsets and the entry formula match the accessors above;
/// the ordering (`+0x30` entries, polygons, then the payload dictionaries) is
/// preserved exactly.
///
/// # Original
/// - Ghidra: `FUN_8023bb60` at `0x8023BB60` (1,432 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023bb60_FUN_8023bb60.c`
/// - `// inferred:` the six `0x8023EAxx` publish helpers take
///   `(entry_word_address, 0)`; their bodies pass the entry word to
///   `0x801AC500` as both data and size.
///
/// # Arguments
/// - `model_ptr`: address of the word holding the MDL0 model pointer.
///
/// # Side effects
/// Clear/cache-store operations and GX display-list construction for every
/// entry of every populated payload dictionary.
///
/// # Safety
/// `*model_ptr` must point at a readable MDL0 body whose section counts and
/// entry offsets are valid.
///
/// # Confidence
/// medium - the eight loops, offsets and call targets are explicit; the
/// binder roles are inferred from their bodies and call sites.
pub unsafe fn mdl_model_bind(model_ptr: usize) {
    // observed: `*(int *)(*param_1 + 0x30)` then count at `+4`.
    let materials = mdl_section_count(model_ptr, MDL_SECTION_30);
    let mut index = 0u32;
    while index < materials {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_SECTION_30, index));
        mdl_entry_clear_draw_cache((entry.as_mut_ptr() as *mut u32) as usize);
        index += 1;
    }

    let polygons = mdl_section_count(model_ptr, MDL_POLYGONS);
    index = 0;
    while index < polygons {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_POLYGONS, index));
        platform::call1(MDL_POLYGON_BUILD, (entry.as_mut_ptr() as *mut u32) as u32);
        index += 1;
    }

    let material_entries = mdl_section_count(model_ptr, MDL_MATERIALS);
    index = 0;
    while index < material_entries {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_MATERIALS, index));
        platform::call2(
            MDL_SECTION_18_PUBLISH,
            (entry.as_mut_ptr() as *mut u32) as u32,
            0,
        );
        index += 1;
    }

    let shader_entries = mdl_section_count(model_ptr, MDL_SHADERS);
    index = 0;
    while index < shader_entries {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_SHADERS, index));
        platform::call2(MDL_SECTION_1C_PUBLISH, (entry.as_mut_ptr() as *mut u32) as u32, 0);
        index += 1;
    }

    let texture_entries = mdl_section_count(model_ptr, MDL_TEXTURES);
    index = 0;
    while index < texture_entries {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_TEXTURES, index));
        platform::call2(MDL_SECTION_20_PUBLISH, (entry.as_mut_ptr() as *mut u32) as u32, 0);
        index += 1;
    }

    let palette_entries = mdl_section_count(model_ptr, MDL_PALETTES);
    index = 0;
    while index < palette_entries {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_PALETTES, index));
        platform::call2(MDL_SECTION_24_PUBLISH, (entry.as_mut_ptr() as *mut u32) as u32, 0);
        index += 1;
    }

    let section_28_entries = mdl_section_count(model_ptr, MDL_SECTION_28);
    index = 0;
    while index < section_28_entries {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_SECTION_28, index));
        platform::call2(MDL_SECTION_28_PUBLISH, (entry.as_mut_ptr() as *mut u32) as u32, 0);
        index += 1;
    }

    let section_2c_entries = mdl_section_count(model_ptr, MDL_SECTION_2C);
    index = 0;
    while index < section_2c_entries {
        let mut entry = crate::platform::GuestLocal::new( mdl_section_entry(model_ptr, MDL_SECTION_2C, index));
        platform::call2(MDL_SECTION_2C_PUBLISH, (entry.as_mut_ptr() as *mut u32) as u32, 0);
        index += 1;
    }
}

/// Resolves the material referenced by a polygon entry's own slot.
///
/// The polygon entry at `*this` stores, at `+0x04`, an offset to the owning
/// model and, at `+0x48`, a signed material index. The function resolves that
/// index with [`mdl_material_by_index`]; the callee's return stays in `r3`, so
/// the caller sees the material pointer even though Ghidra typed this routine
/// `void`.
///
/// # Original
/// - Ghidra: `FUN_8023c100` at `0x8023C100` (72 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c100_FUN_8023c100.c`
/// - `// inferred:` the `+0x04` word is the owning-model offset; caller
///   `0x8025CA20` passes a polygon obtained from `0x8023B9B0` and consumes the
///   result, although Ghidra prints the function as `void`.
///
/// # Arguments
/// - `this`: address of the word holding the polygon entry.
///
/// # Returns
/// Material resource pointer, or 0 when the model/material lookup is absent.
///
/// # Side effects
/// None in this function; the callee is read-only.
///
/// # Safety
/// `*this` must point at a readable polygon entry with a valid `+0x04` owner
/// offset.
///
/// # Confidence
/// medium - the index and owner offsets are literal; the return type is
/// recovered from the caller, which consumes `r3`.
pub unsafe fn mdl_polygon_material(this: usize) -> u32 {
    let polygon = platform::read_u32(this) as usize;
    let mut owner = crate::platform::GuestLocal::new( mdl_polygon_owner(polygon));
    let index = platform::read_u16(polygon + 0x48) as i16;
    mdl_material_by_index((owner.as_mut_ptr() as *mut u32) as usize, index as u32)
}

/// Resolves a polygon entry's material shader from its `+0x4A` slot.
///
/// A signed `-1` slot means "no shader" and returns 0. Otherwise the owning
/// model is resolved from the entry's `+0x04` offset and the shader is fetched
/// with [`mdl_shader_by_index`].
///
/// # Original
/// - Ghidra: `FUN_8023c150` at `0x8023C150` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c150_FUN_8023c150.c`
///
/// # Arguments
/// - `this`: address of the word holding the polygon entry.
///
/// # Returns
/// Shader resource pointer, or 0 for slot `-1`.
///
/// # Safety
/// `*this` must point at a readable polygon entry.
///
/// # Confidence
/// high - the `-1` test, owner offset and shader lookup are explicit.
pub unsafe fn mdl_polygon_shader(this: usize) -> u32 {
    let polygon = platform::read_u32(this) as usize;
    let index = platform::read_u16(polygon + 0x4A) as i16;
    if index == -1 {
        return 0;
    }
    let mut owner = crate::platform::GuestLocal::new( mdl_polygon_owner(polygon));
    mdl_shader_by_index((owner.as_mut_ptr() as *mut u32) as usize, index as u32)
}

/// Resolves a polygon entry's texture reference from a `+0x4C` slot.
///
/// The entry holds up to two signed 16-bit texture-reference indices starting
/// at `+0x4C`. Slot `-1` returns 0; otherwise the owning model is resolved from
/// `+0x04` and the reference is fetched with [`mdl_texture_by_index`].
///
/// # Original
/// - Ghidra: `FUN_8023c1b0` at `0x8023C1B0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c1b0_FUN_8023c1b0.c`
/// - `// observed:` `0x8025CA20` calls it with slot 0 and again with slot 1
///   while comparing texture references.
///
/// # Arguments
/// - `this`: address of the word holding the polygon entry.
/// - `slot`: texture slot, 0 or 1 in the observed callers.
///
/// # Returns
/// Texture-reference resource pointer, or 0 for slot `-1`.
///
/// # Safety
/// `*this` must point at a readable polygon entry.
///
/// # Confidence
/// high - the `-1` test, per-slot offset and lookup are explicit.
pub unsafe fn mdl_polygon_texture(this: usize, slot: u32) -> u32 {
    let polygon = platform::read_u32(this) as usize;
    let field = polygon + slot as usize * 2;
    let index = platform::read_u16(field + 0x4C) as i16;
    if index == -1 {
        return 0;
    }
    let mut owner = crate::platform::GuestLocal::new( mdl_polygon_owner(polygon));
    mdl_texture_by_index((owner.as_mut_ptr() as *mut u32) as usize, index as u32)
}

/// Resolves a polygon entry's `+0x2C` dictionary entry from its `+0x62` slot.
///
/// A signed `-1` slot returns 0. Otherwise the owning model is resolved from
/// `+0x04` and the entry is fetched with [`mdl_section_2c_by_index`].
/// `// unknown:` the semantic name of the `+0x2C` section.
///
/// # Original
/// - Ghidra: `FUN_8023c220` at `0x8023C220` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c220_FUN_8023c220.c`
///
/// # Arguments
/// - `this`: address of the word holding the polygon entry.
///
/// # Returns
/// `+0x2C` entry pointer, or 0 for slot `-1`.
///
/// # Safety
/// `*this` must point at a readable polygon entry.
///
/// # Confidence
/// high for the control flow; low for the section's semantic role.
pub unsafe fn mdl_polygon_section_2c(this: usize) -> u32 {
    let polygon = platform::read_u32(this) as usize;
    let index = platform::read_u16(polygon + 0x62) as i16;
    if index == -1 {
        return 0;
    }
    let mut owner = crate::platform::GuestLocal::new( mdl_polygon_owner(polygon));
    mdl_section_2c_by_index((owner.as_mut_ptr() as *mut u32) as usize, index as u32)
}

/// Returns the owning-model pointer of a polygon entry (internal helper).
///
/// The entry's `+0x04` word is an offset from the entry base; zero means
/// "no owner".
///
/// # Original
/// - Inlined in `FUN_8023c100`, `FUN_8023c150`, `FUN_8023c1b0` and
///   `FUN_8023c220`.
/// - Evidence: `work/ghidra/main-gb/export/8023c100_FUN_8023c100.c`
///
/// # Safety
/// `polygon` must be readable at `polygon + 0x04`.
///
/// # Confidence
/// high - every accessor repeats the same load-and-add.
unsafe fn mdl_polygon_owner(polygon: usize) -> u32 {
    let offset = platform::read_u32(polygon + 0x04);
    if offset == 0 {
        0
    } else {
        polygon.wrapping_add(offset as usize) as u32
    }
}

/// Clears one `+0x30` entry's 0x180-byte draw cache.
///
/// The entry's `+0x3C` word is an offset to the cache block; a zero offset
/// passes address 0 to [`DC_STORE`], exactly as the original does.
/// `// unknown:` the purpose of the 0x180-byte block.
///
/// # Original
/// - Ghidra: `FUN_8023e320` at `0x8023E320` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e320_FUN_8023e320.c`
///
/// # Arguments
/// - `this`: address of the word holding the `+0x30` entry pointer.
///
/// # Side effects
/// Executes a data-cache block store over 0x180 bytes.
///
/// # Safety
/// `*this` must point at a readable entry whose `+0x3C` offset addresses a
/// writable 0x180-byte block.
///
/// # Confidence
/// medium - the offset, size and cache call are literal; the block's meaning
/// is not identified.
pub unsafe fn mdl_entry_clear_draw_cache(this: usize) {
    let entry = platform::read_u32(this) as usize;
    let offset = platform::read_u32(entry + 0x3C);
    let data = if offset == 0 {
        0
    } else {
        entry.wrapping_add(offset as usize)
    };
    platform::call2(DC_STORE, data as u32, 0x180);
}

/// Returns the data pointer stored as an offset at a `+0x30` entry's `+0x28`.
///
/// `// inferred:` caller `0x8024DCE0` feeds the result to the GX transform
/// state setup for one material record.
///
/// # Original
/// - Ghidra: `FUN_8023e350` at `0x8023E350` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e350_FUN_8023e350.c`
///
/// # Arguments
/// - `this`: address of the word holding the entry pointer.
///
/// # Returns
/// `entry + *(entry + 0x28)`, or 0 when that offset is zero.
///
/// # Safety
/// `*this` must point at a readable entry.
///
/// # Confidence
/// medium - the offset arithmetic is explicit; the field's role is inferred
/// from the caller.
pub unsafe fn mdl_entry_section_28(this: usize) -> u32 {
    let entry = platform::read_u32(this) as usize;
    let offset = platform::read_u32(entry + 0x28);
    if offset == 0 {
        0
    } else {
        entry.wrapping_add(offset as usize) as u32
    }
}

/// Returns the data pointer stored as an offset at a `+0x30` entry's `+0x34`.
///
/// `// inferred:` caller `0x8024E180` reads this pointer while building the
/// material's draw state; the `+0x34` field is an offset, not an address.
///
/// # Original
/// - Ghidra: `FUN_8023e370` at `0x8023E370` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e370_FUN_8023e370.c`
///
/// # Arguments
/// - `this`: address of the word holding the entry pointer.
///
/// # Returns
/// `entry + *(entry + 0x34)`, or 0 when that offset is zero.
///
/// # Safety
/// `*this` must point at a readable entry.
///
/// # Confidence
/// medium - the offset arithmetic is explicit; the field's role is inferred
/// from the caller.
pub unsafe fn mdl_entry_section_34(this: usize) -> u32 {
    let entry = platform::read_u32(this) as usize;
    let offset = platform::read_u32(entry + 0x34);
    if offset == 0 {
        0
    } else {
        entry.wrapping_add(offset as usize) as u32
    }
}

/// Clears a material record's texture/palette binding and its bitmask bits.
///
/// The record's `+0x08` (TEX0 pointer) and `+0x0C` (palette pointer) are
/// zeroed. The slot index at `+0x10` selects one bit in each of two caller
/// bitmasks; a null bitmask pointer is skipped, and slot indices above 7 are
/// ignored exactly as in the original.
///
/// # Original
/// - Ghidra: `FUN_8023e650` at `0x8023E650` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e650_FUN_8023e650.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
/// - `tex_bits`: address of a word holding the texture bitmask pointer, or 0.
/// - `plt_bits`: address of a word holding the palette bitmask pointer, or 0.
///
/// # Side effects
/// Writes two zero words into the record and clears one bit in each non-null
/// bitmask.
///
/// # Safety
/// `*this` must point at a writable material record; each non-null bitmask
/// word must hold a writable bitmask address.
///
/// # Confidence
/// high - both zero stores and both guarded bit clears are explicit.
pub unsafe fn mdl_material_reset_binding(this: usize, tex_bits: usize, plt_bits: usize) {
    let record = platform::read_u32(this) as usize;
    platform::write_u32(record + 0x08, 0);
    platform::write_u32(record + 0x0C, 0);

    let slot = platform::read_u32(record + 0x10);
    if tex_bits != 0 && slot < 8 {
        let mask = platform::read_u32(tex_bits) as usize;
        if mask != 0 {
            platform::write_u32(mask, platform::read_u32(mask) & !(1u32 << slot));
        }
    }
    if plt_bits == 0 || slot > 7 {
        return;
    }
    let mask = platform::read_u32(plt_bits) as usize;
    platform::write_u32(mask, platform::read_u32(mask) & !(1u32 << slot));
}

/// Emits the texture state byte of a material record as display-list 9.
///
/// The record's `+0x08` word is an offset to the texture data; a zero offset
/// yields address 0. The byte at `+0x1D` is forwarded to the GX FIFO.
///
/// # Original
/// - Ghidra: `FUN_8023e6c0` at `0x8023E6C0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e6c0_FUN_8023e6c0.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
///
/// # Side effects
/// Calls `0x801C4780(9, data, byte)`; the callee writes its third argument to
/// the GX FIFO.
///
/// # Safety
/// `*this` must point at a readable material record.
///
/// # Confidence
/// medium - the offsets and call are literal; the display-list number's
/// meaning is inferred from the call shape.
pub unsafe fn mdl_material_emit_texture_9(this: usize) {
    let record = platform::read_u32(this) as usize;
    if record == 0 {
        return;
    }
    let offset = platform::read_u32(record + 0x08);
    let data = if offset == 0 {
        0
    } else {
        record.wrapping_add(offset as usize)
    };
    platform::call3(
        GX_FIFO_WRITE,
        9,
        data as u32,
        platform::read_u8(record + 0x1D) as u32,
    );
}

/// Reads a material record's texture data pointer and `+0x1D` state byte.
///
/// # Original
/// - Ghidra: `FUN_8023e700` at `0x8023E700` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e700_FUN_8023e700.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
/// - `data_out`: optional address that receives the texture data pointer.
/// - `byte_out`: optional address that receives the byte at record `+0x1D`.
///
/// # Side effects
/// Writes through the two non-null output pointers.
///
/// # Safety
/// `*this` must point at a readable record; each non-null output must be
/// writable.
///
/// # Confidence
/// high - both optional writes and the offset are explicit.
pub unsafe fn mdl_material_get_texture_9(this: usize, data_out: usize, byte_out: usize) {
    let record = platform::read_u32(this) as usize;
    if data_out != 0 {
        let offset = platform::read_u32(record + 0x08);
        let data = if offset == 0 {
            0
        } else {
            record.wrapping_add(offset as usize)
        };
        platform::write_u32(data_out, data as u32);
    }
    if byte_out != 0 {
        platform::write_u8(byte_out, platform::read_u8(record + 0x1D));
    }
}

/// Copies one material record's texture bytes and publishes the cache.
///
/// The record's first word is used as the byte count; the original copies it
/// with `0x8024A530` and then calls the cache-block store helper with the same
/// count. `// unknown:` why the count is the record's first word rather than
/// `*(record + 0x04)`.
///
/// # Original
/// - Ghidra: `FUN_8023e740` at `0x8023E740` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e740_FUN_8023e740.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
/// - `dst`: destination address for the copied bytes.
///
/// # Side effects
/// Copies whole 32-byte units and runs a data-cache block store over `dst`.
///
/// # Safety
/// `*this` must point at a readable record whose first word is a sensible byte
/// count; `dst` must be writable.
///
/// # Confidence
/// medium - the calls and count source are literal; the field's meaning is
/// unknown.
pub unsafe fn mdl_material_publish(this: usize, dst: u32) {
    let record = platform::read_u32(this) as usize;
    if record != 0 {
        let size = platform::read_u32(record);
        platform::call3(MEMCPY_BLOCKS_32, dst, record as u32, size);
        platform::call2(DC_STORE_SYNC, dst, size);
    }
}

/// Emits the texture state byte of a material record as display-list 10.
///
/// This is the display-list-10 sibling of [`mdl_material_emit_texture_9`].
///
/// # Original
/// - Ghidra: `FUN_8023e7a0` at `0x8023E7A0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e7a0_FUN_8023e7a0.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
///
/// # Side effects
/// Calls `0x801C4780(10, data, byte)`.
///
/// # Safety
/// `*this` must point at a readable material record.
///
/// # Confidence
/// medium - the offsets and call are literal; the display-list number's
/// meaning is inferred from the call shape.
pub unsafe fn mdl_material_emit_texture_10(this: usize) {
    let record = platform::read_u32(this) as usize;
    if record == 0 {
        return;
    }
    let offset = platform::read_u32(record + 0x08);
    let data = if offset == 0 {
        0
    } else {
        record.wrapping_add(offset as usize)
    };
    platform::call3(
        GX_FIFO_WRITE,
        10,
        data as u32,
        platform::read_u8(record + 0x1D) as u32,
    );
}

/// Emits an indexed texture state byte as display-list 11 or 12.
///
/// The caller's selector is restricted to 11 and 12; any other value returns
/// without touching the GX FIFO. The texture data pointer is resolved from
/// record `+0x08` and the byte comes from `+0x1C`.
///
/// # Original
/// - Ghidra: `FUN_8023e880` at `0x8023E880` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e880_FUN_8023e880.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
/// - `selector`: display-list selector, observed as 11 or 12.
///
/// # Side effects
/// Calls `0x801C4780(selector, data, byte)` when `selector` is 11 or 12.
///
/// # Safety
/// `*this` must point at a readable material record.
///
/// # Confidence
/// high - the range test and offsets are explicit.
pub unsafe fn mdl_material_emit_texture_indexed(this: usize, selector: u32) {
    let record = platform::read_u32(this) as usize;
    if record == 0 {
        return;
    }
    if selector.wrapping_sub(11) > 1 {
        return;
    }
    let offset = platform::read_u32(record + 0x08);
    let data = if offset == 0 {
        0
    } else {
        record.wrapping_add(offset as usize)
    };
    platform::call3(
        GX_FIFO_WRITE,
        selector,
        data as u32,
        platform::read_u8(record + 0x1C) as u32,
    );
}

/// Reads a material record's texture data pointer and `+0x1C` state byte.
///
/// This is the indexed sibling of [`mdl_material_get_texture_9`]; the byte
/// comes from `+0x1C` rather than `+0x1D`.
///
/// # Original
/// - Ghidra: `FUN_8023e8c0` at `0x8023E8C0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e8c0_FUN_8023e8c0.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
/// - `data_out`: optional address that receives the texture data pointer.
/// - `byte_out`: optional address that receives the byte at record `+0x1C`.
///
/// # Side effects
/// Writes through the two non-null output pointers.
///
/// # Safety
/// `*this` must point at a readable record; each non-null output must be
/// writable.
///
/// # Confidence
/// high - both optional writes and the offset are explicit.
pub unsafe fn mdl_material_get_texture_indexed(
    this: usize,
    data_out: usize,
    byte_out: usize,
) {
    let record = platform::read_u32(this) as usize;
    if data_out != 0 {
        let offset = platform::read_u32(record + 0x08);
        let data = if offset == 0 {
            0
        } else {
            record.wrapping_add(offset as usize)
        };
        platform::write_u32(data_out, data as u32);
    }
    if byte_out != 0 {
        platform::write_u8(byte_out, platform::read_u8(record + 0x1C));
    }
}

/// Reads a material record's texture data pointer and `+0x1D` state byte.
///
/// The body is byte-identical to [`mdl_material_get_texture_9`]; the shipped
/// build keeps a second copy for the display-list-10 path.
///
/// # Original
/// - Ghidra: `FUN_8023e960` at `0x8023E960` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e960_FUN_8023e960.c`
///
/// # Arguments
/// - `this`: address of the word holding the material record.
/// - `data_out`: optional address that receives the texture data pointer.
/// - `byte_out`: optional address that receives the byte at record `+0x1D`.
///
/// # Side effects
/// Writes through the two non-null output pointers.
///
/// # Safety
/// `*this` must point at a readable record; each non-null output must be
/// writable.
///
/// # Confidence
/// high - identical control flow to `0x8023E700`.
pub unsafe fn mdl_material_get_texture_10(this: usize, data_out: usize, byte_out: usize) {
    let record = platform::read_u32(this) as usize;
    if data_out != 0 {
        let offset = platform::read_u32(record + 0x08);
        let data = if offset == 0 {
            0
        } else {
            record.wrapping_add(offset as usize)
        };
        platform::write_u32(data_out, data as u32);
    }
    if byte_out != 0 {
        platform::write_u8(byte_out, platform::read_u8(record + 0x1D));
    }
}

/// Reads a TEX0 header whose `+0x18` flag bit 0 is clear.
///
/// The function copies the header fields into optional output words:
///
/// | output | source |
/// |---|---|
/// | data pointer | `tex + *(tex + 0x10)` |
/// | width | `u16` at `+0x1C` |
/// | height | `u16` at `+0x1E` |
/// | format | `u32` at `+0x20` |
/// | scale X | `u32` at `+0x28` |
/// | scale Y | `u32` at `+0x2C` |
/// | flag | derived from `+0x24` |
///
/// The flag expression is the PowerPC `cntlzw`/`slw`/`srw` idiom. It is
/// evaluated literally, including the 32-bit shift, so a value of 0 or 1
/// yields 0 and every other value yields 1.
///
/// # Original
/// - Ghidra: `FUN_8023eb30` at `0x8023EB30` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023eb30_FUN_8023eb30.c`
/// - `// observed:` `0x8023E390` calls this reader when the material record's
///   `+0x04` palette pointer is zero.
/// - `// inferred:` the `+0x18` bit-0 flag therefore selects the
///   palette-present TEX0 layout.
///
/// # Arguments
/// - `this`: address of the word holding the TEX0 resource pointer.
/// - `out_data`, `out_width`, `out_height`, `out_format`, `out_scale_x`,
///   `out_scale_y`, `out_flag`: optional output addresses; 0 skips a field.
///
/// # Returns
/// 1 when the palette bit is clear and the fields were read; 0 otherwise.
///
/// # Side effects
/// Writes through the non-null output pointers.
///
/// # Safety
/// `*this` must point at a readable TEX0 header and each non-null output must
/// be writable with the documented width.
///
/// # Confidence
/// high - every field offset and guard is explicit; the flag idiom is
/// reproduced literally.
pub unsafe fn res_texture_descriptor(
    this: usize,
    out_data: usize,
    out_width: usize,
    out_height: usize,
    out_format: usize,
    out_scale_x: usize,
    out_scale_y: usize,
    out_flag: usize,
) -> u32 {
    res_texture_descriptor_impl(
        this, 0, out_data, out_width, out_height, out_format, out_scale_x, out_scale_y, out_flag,
    )
}

/// Reads a TEX0 header whose `+0x18` flag bit 0 is set.
///
/// This is the flag-set sibling of [`res_texture_descriptor`]; the field
/// layout and output behavior are identical.
///
/// # Original
/// - Ghidra: `FUN_8023ebf0` at `0x8023EBF0` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ebf0_FUN_8023ebf0.c`
/// - `// observed:` `0x8023E390` calls this reader when the material record's
///   `+0x04` palette pointer is nonzero.
/// - `// inferred:` the `+0x18` bit-0 flag therefore selects the
///   palette-present TEX0 layout.
///
/// # Arguments
/// - `this`: address of the word holding the TEX0 resource pointer.
/// - `out_data`, `out_width`, `out_height`, `out_format`, `out_scale_x`,
///   `out_scale_y`, `out_flag`: optional output addresses; 0 skips a field.
///
/// # Returns
/// 1 when the palette bit is set and the fields were read; 0 otherwise.
///
/// # Side effects
/// Writes through the non-null output pointers.
///
/// # Safety
/// `*this` must point at a readable TEX0 header and each non-null output must
/// be writable with the documented width.
///
/// # Confidence
/// high - the only difference from `0x8023EB30` is the palette-bit test.
pub unsafe fn res_texture_descriptor_flag_set(
    this: usize,
    out_data: usize,
    out_width: usize,
    out_height: usize,
    out_format: usize,
    out_scale_x: usize,
    out_scale_y: usize,
    out_flag: usize,
) -> u32 {
    res_texture_descriptor_impl(
        this, 1, out_data, out_width, out_height, out_format, out_scale_x, out_scale_y, out_flag,
    )
}

/// Shared body of the two TEX0 descriptor readers (internal helper).
///
/// # Original
/// - Inlined in `FUN_8023eb30` and `FUN_8023ebf0`.
/// - Evidence: `work/ghidra/main-gb/export/8023eb30_FUN_8023eb30.c`
///
/// # Safety
/// Same requirements as [`res_texture_descriptor`].
///
/// # Confidence
/// high - the two originals differ only in the expected +0x18 flag bit.
unsafe fn res_texture_descriptor_impl(
    this: usize,
    flag_bit: u32,
    out_data: usize,
    out_width: usize,
    out_height: usize,
    out_format: usize,
    out_scale_x: usize,
    out_scale_y: usize,
    out_flag: usize,
) -> u32 {
    let texture = platform::read_u32(this) as usize;
    if (platform::read_u32(texture + 0x18) & 1) != flag_bit {
        return 0;
    }
    if out_data != 0 {
        let offset = platform::read_u32(texture + 0x10);
        let data = if offset == 0 {
            0
        } else {
            texture.wrapping_add(offset as usize)
        };
        platform::write_u32(out_data, data as u32);
    }
    if out_width != 0 {
        platform::write_u16(out_width, platform::read_u16(texture + 0x1C));
    }
    if out_height != 0 {
        platform::write_u16(out_height, platform::read_u16(texture + 0x1E));
    }
    if out_format != 0 {
        platform::write_u32(out_format, platform::read_u32(texture + 0x20));
    }
    if out_scale_x != 0 {
        platform::write_u32(out_scale_x, platform::read_u32(texture + 0x28));
    }
    if out_scale_y != 0 {
        platform::write_u32(out_scale_y, platform::read_u32(texture + 0x2C));
    }
    if out_flag != 0 {
        let value = platform::read_u32(texture + 0x24);
        let shift = (value ^ 1).leading_zeros();
        // PowerPC `slw` shifts by the low six bits; a shift of 32 or more
        // discards every bit, which is what the guard models here.
        let shifted = if shift >= 32 { 0 } else { value << shift };
        platform::write_u8(out_flag, (shifted >> 31) as u8);
    }
    1
}

/// Clears the cached texture and palette pointer tables of an `AnmTexPat`.
///
/// The animation object stores two counts (`u16` at `+0x34` and `+0x36`) and
/// two pointer tables as offsets (`+0x1C` and `+0x20`). Every slot in both
/// tables is set to zero; the original does not null-check the table offsets,
/// so neither does this translation.
///
/// # Original
/// - Ghidra: `FUN_8023f890` at `0x8023F890` (416 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f890_FUN_8023f890.c`
/// - `// observed:` the two table offsets and two counts are the same fields
///   used by [`anm_tex_pat_bind`] at `0x8023F720`.
///
/// # Arguments
/// - `this`: address of the word holding the `AnmTexPat` resource pointer.
///
/// # Side effects
/// Zeroes `tex_count + plt_count` 32-bit slots.
///
/// # Safety
/// `*this` must point at a readable animation object whose table offsets
/// address writable arrays of the declared lengths.
///
/// # Confidence
/// high - both loops and both count fields are explicit.
pub unsafe fn anm_tex_pat_clear_slots(this: usize) {
    let object = platform::read_u32(this) as usize;
    let tex_count = platform::read_u16(object + ANM_TEX_COUNT) as u32;
    let plt_count = platform::read_u16(object + ANM_PLT_COUNT) as u32;
    let tex_slots = resource_table(object, ANM_TEX_SLOTS);
    let plt_slots = resource_table(object, ANM_PLT_SLOTS);

    let mut index = 0u32;
    while index < tex_count {
        platform::write_u32(tex_slots + index as usize * 4, 0);
        index += 1;
    }

    index = 0;
    while index < plt_count {
        platform::write_u32(plt_slots + index as usize * 4, 0);
        index += 1;
    }
}

// ===========================================================================
// Wave 18: material and texture state payload
// ===========================================================================

/// Reads a big-endian `f64` from a virtual address.
///
/// # Original
/// Host-side helper; no binary address. Models the PowerPC `lfd` load used by
/// [`anm_frame_ratio`] and the two channel-state writers.
///
/// # Safety
/// `addr..addr + 8` must be readable in the target's address space.
///
/// # Confidence
/// high - mechanical wrapper.
unsafe fn read_f64(addr: usize) -> f64 {
    let hi = platform::read_u32(addr) as u64;
    let lo = platform::read_u32(addr + 4) as u64;
    f64::from_bits((hi << 32) | lo)
}

/// Calls a function with two `f64` arguments and an `f64` result.
///
/// # Original
/// Host-side helper; no binary address. [`anm_frame_ratio`] calls
/// `0x802E55A4` with a ratio and an exponent.
///
/// # Safety
/// `addr` must point at a function with this ABI.
///
/// # Confidence
/// high - mechanical wrapper.
unsafe fn call_f64_2(addr: usize, a: f64, b: f64) -> f64 {
    let f = crate::platform::guest_fn!(addr, fn(f64, f64) -> f64);
    f(a, b)
}

/// Calls a function with three `f64` arguments and a pointer argument.
///
/// # Original
/// Host-side helper; no binary address. The two channel-state writers call
/// `0x802382B0(double, double, double, float *)`.
///
/// # Safety
/// `addr` must point at a function with this ABI; `dst` must be writable for
/// the matrix that the callee stores.
///
/// # Confidence
/// high - mechanical wrapper.
unsafe fn call_f64_3_ptr(addr: usize, a: f64, b: f64, c: f64, dst: usize) {
    let f = crate::platform::guest_fn!(addr, fn(f64, f64, f64, usize));
    f(a, b, c, dst);
}

/// Copies the 0x20-byte texture object referenced by `*this`.
///
/// # Original
/// - Ghidra: `FUN_8023cb60` at `0x8023CB60` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cb60_FUN_8023cb60.c`
/// - `// inferred:` the copied size and the neighbouring accessors identify
///   the record as a `GXTexObj`-shaped 0x20-byte texture object.
///
/// # Arguments
/// - `this`: address of the word holding the source texture object.
/// - `dst`: destination for the 0x20-byte copy.
///
/// # Returns
/// `dst`, matching the original's return of its second argument.
///
/// # Side effects
/// Copies 0x20 bytes through `0x8024A530`.
///
/// # Safety
/// `*this` must point at 0x20 readable bytes and `dst` at 0x20 writable bytes.
///
/// # Confidence
/// high - size, argument order and return value are explicit.
pub unsafe fn res_texture_object_copy(this: usize, dst: usize) -> u32 {
    let src = platform::read_u32(this) as usize;
    platform::call3(MEMCPY_BLOCKS_32, dst as u32, src as u32, 0x20);
    dst as u32
}

/// Emits the command word selected by a texture-coordinate record.
///
/// `record[+0xC]` is a one-based index into the 32-bit table at `0x802F84D0`.
/// When the second argument is non-zero the routine first performs a `syscall`.
/// The selected word is then written to `record + 0x20` through the synchronous
/// GX FIFO writer.
///
/// # Original
/// - Ghidra: `FUN_8023c950` at `0x8023C950` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c950_FUN_8023c950.c`
/// - `// inferred:` the table index and the `+0x20` destination place this in
///   the texture-coordinate setup family handled by
///   [`res_texcoord_emit`].
///
/// # Arguments
/// - `this`: address of the record pointer.
/// - `sync`: when non-zero, issue the `syscall` at `0x801A9B90` first.
///
/// # Side effects
/// May execute a `syscall`; writes one GX FIFO word.
///
/// # Safety
/// `*this` must point at a readable record whose `+0xC` byte indexes the
/// read-only table; the GX FIFO must be mapped.
///
/// # Confidence
/// medium - the lookup and command are literal; the table's role is inferred.
pub unsafe fn res_texcoord_command_emit(this: usize, sync: u32) {
    if sync != 0 {
        platform::call0(SYSCALL);
    }
    let record = platform::read_u32(this) as usize;
    let index = platform::read_u8(record + 0x0C) as i32 - 1;
    let entry = (TEXCOORD_COMMAND_TABLE as i32).wrapping_add(index.wrapping_mul(4)) as u32;
    let word = platform::read_u32(entry as usize);
    platform::call2(GX_FIFO_WRITE_SYNC, (record + 0x20) as u32, word);
}

/// Resolves an offset stored at `record + 4` against the record base.
///
/// This is the same offset-pointer idiom used by the BRRES dictionaries and by
/// the texture data accessor: a zero offset means "absent" and returns 0.
///
/// # Original
/// - Ghidra: `FUN_8023ca40` at `0x8023CA40` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ca40_FUN_8023ca40.c`
/// - `// inferred:` the caller family around `0x8023C9B0..0x8023CE30` treats
///   the result as a sub-record pointer.
///
/// # Arguments
/// - `this`: address of the record pointer.
///
/// # Returns
/// `base + offset`, or 0 when the offset word is zero.
///
/// # Safety
/// `*this` must point at a readable record with a word at `+4`.
///
/// # Confidence
/// high - the offset test and addition are explicit.
pub unsafe fn res_subresource_ptr(this: usize) -> u32 {
    let base = platform::read_u32(this) as usize;
    let offset = platform::read_u32(base + 4);
    if offset != 0 {
        base.wrapping_add(offset as usize) as u32
    } else {
        0
    }
}

/// Copies every present texture object into a compact output array.
///
/// The first output word receives the present bitset, then each set bit copies
/// its 0x20-byte object to the next output slot. The source and destination
/// strides both advance for every bit, including clear bits.
///
/// # Original
/// - Ghidra: `FUN_8023cd00` at `0x8023CD00` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cd00_FUN_8023cd00.c`
///
/// # Arguments
/// - `this`: address of the cache base pointer.
/// - `out`: destination for the bitset followed by the copied objects.
///
/// # Returns
/// `out`.
///
/// # Side effects
/// Writes one bitset word and one 0x20-byte copy per set bit.
///
/// # Safety
/// `out` must have room for 4 + `popcount(bitset) * 0x20` bytes; the source
/// cache must contain eight readable object slots.
///
/// # Confidence
/// high - the loop, mask test and strides are explicit.
pub unsafe fn res_texture_cache_copy_present(this: usize, out: usize) -> u32 {
    let base = platform::read_u32(this) as usize;
    let mut bits = platform::read_u32(base);
    platform::write_u32(out, bits);
    let mut src = base + 4;
    let mut dst = out + 4;
    while bits != 0 {
        if (bits & 1) != 0 {
            platform::call3(MEMCPY_BLOCKS_32, dst as u32, src as u32, 0x20);
        }
        bits >>= 1;
        src += 0x20;
        dst += 0x20;
    }
    out as u32
}

/// Copies ten words (0x28 bytes) out of the record referenced by `*this`.
///
/// # Original
/// - Ghidra: `FUN_8023dd70` at `0x8023DD70` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023dd70_FUN_8023dd70.c`
/// - `// inferred:` the 0x28-byte record is the same state shape copied by
///   `0x8023C9B0` and `0x8023CE30`; the exact record name is unknown.
///
/// # Arguments
/// - `this`: address of the record pointer.
/// - `out`: destination for ten words.
///
/// # Returns
/// `out`.
///
/// # Side effects
/// Writes 0x28 bytes.
///
/// # Safety
/// `*this` must point at 0x28 readable bytes and `out` at 0x28 writable bytes.
///
/// # Confidence
/// high - all ten word copies are explicit.
pub unsafe fn res_record_copy_40(this: usize, out: usize) -> u32 {
    let src = platform::read_u32(this) as usize;
    let mut index = 0usize;
    while index < 10 {
        platform::write_u32(out + index * 4, platform::read_u32(src + index * 4));
        index += 1;
    }
    out as u32
}

/// Tests one bit of the palette-cache present bitset.
///
/// # Original
/// - Ghidra: `FUN_8023cdf0` at `0x8023CDF0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cdf0_FUN_8023cdf0.c`
///
/// # Arguments
/// - `this`: address of the cache base pointer.
/// - `index`: palette-object slot, 0..7.
///
/// # Returns
/// 1 when the slot bit is set, otherwise 0.
///
/// # Safety
/// A non-null base must point at a readable bitset word.
///
/// # Confidence
/// high - identical branchless bit test to `0x8023CCC0`.
pub unsafe fn res_palette_cache_present(this: usize, index: u32) -> u32 {
    let base = platform::read_u32(this) as usize;
    if base == 0 || index >= 8 {
        return 0;
    }
    let bit = platform::read_u32(base) & (1u32 << index);
    (bit.wrapping_neg() | bit) >> 31
}

/// Copies the palette object block selected by the present bitset.
///
/// The original chooses 0x20, 0x40 or 0x60 bytes from the same bitset: fewer
/// than two set bits copies 0x20, fewer than four copies 0x40, otherwise 0x60.
///
/// # Original
/// - Ghidra: `FUN_8023ce30` at `0x8023CE30` (136 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ce30_FUN_8023ce30.c`
/// - `// inferred:` the three sizes match one, two-to-three, and four-to-seven
///   populated 0xC-byte palette slots.
///
/// # Arguments
/// - `this`: address of the cache base pointer.
/// - `out`: destination for the bitset word and the copied block.
///
/// # Returns
/// `out`.
///
/// # Side effects
/// Writes the bitset and 0x20/0x40/0x60 copied bytes.
///
/// # Safety
/// `out` must have room for 4 + the selected size; the source must be readable
/// for that many bytes.
///
/// # Confidence
/// medium - the sizes are literal, but the exact object shape is inferred.
pub unsafe fn res_palette_cache_copy(this: usize, out: usize) -> u32 {
    let src = platform::read_u32(this) as usize;
    let bits = platform::read_u32(src);
    platform::write_u32(out, bits);
    if bits != 0 {
        let size = if (bits & 0xFC) == 0 {
            0x20
        } else if (bits & 0xE0) == 0 {
            0x40
        } else {
            0x60
        };
        platform::call3(MEMCPY_BLOCKS_32, (out + 4) as u32, (src + 4) as u32, size);
    }
    out as u32
}

/// Copies the eight-entry texture/palette state table out of a source object.
///
/// The function receives the destination block in `r4` and a pointer to the
/// source-table pointer in `r3`. It copies eight 0x14-byte entries selected by
/// the low nibble of each mask word, copies a parallel 0xD-byte entry table,
/// and stops when the mask word reaches zero. The first word and the second
/// word are copied unconditionally.
///
/// # Original
/// - Ghidra: `FUN_8023cec0` at `0x8023CEC0` (212 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cec0_FUN_8023cec0.c`
/// - Removed: the modelled `0x802DCD70` save / `0x802DCDBC` restore pair. The
///   `bl 0x802DCD70` at `0x8023CED0` is the `_savegpr0_26` prologue fragment
///   (it only spills `r26..r31`) and `0x802DCDBC` at `0x8023CF80` is the
///   matching `_restgpr0_26` epilogue; the `u64` Ghidra reads back is the
///   incoming `r3:r4` (`r3` high, `r4` low), now the two parameters below.
/// - `// inferred:` the 0x14/0xD strides describe a cached material state
///   table; the exact G3D object names are unknown.
///
/// # Arguments
/// - `context`: pointer to the source-table pointer (incoming `r3`).
/// - `dst`: destination block (incoming `r4`), at least 0x140 bytes.
///
/// # Side effects
/// Copies state bytes and calls `0x801C0500` for each selected entry.
///
/// # Safety
/// `context` must address a readable pointer to a source table big enough for
/// eight 0x14/0xD-byte entries; `dst` must be writable for the same size.
///
/// # Confidence
/// medium - the copy loops are literal; the state-table meaning is inferred.
pub unsafe fn res_palette_table_copy(context: usize, dst: usize) {
    let src = platform::read_u32(context) as usize;

    let mut bits = platform::read_u32(src);
    platform::write_u32(dst, bits);
    platform::write_u32(dst + 4, platform::read_u32(src + 4));

    let mut src_words = src + 8;
    let mut dst_words = dst + 8;
    let mut src_bytes = src + 0xA8;
    let mut dst_bytes = dst + 0xA8;
    while bits != 0 {
        if (bits & 0xF) != 0 {
            let mut word = 0usize;
            while word < 5 {
                platform::write_u32(
                    dst_words + word * 4,
                    platform::read_u32(src_words + word * 4),
                );
                word += 1;
            }
            platform::write_u8(dst_bytes, platform::read_u8(src_bytes));
            platform::write_u8(dst_bytes + 1, platform::read_u8(src_bytes + 1));
            platform::write_u8(dst_bytes + 2, platform::read_u8(src_bytes + 2));
            platform::write_u8(
                dst_bytes + 3,
                (platform::read_u32(src_bytes) & 0xFF) as u8,
            );
            platform::call2(
                PSMTX_COPY,
                (src_bytes + 4) as u32,
                (dst_bytes + 4) as u32,
            );
        }
        bits >>= 4;
        src_words += 0x14;
        src_bytes += 0x0D;
        dst_words += 0x14;
        dst_bytes += 0x0D;
    }
}
/// Sets or clears one material's texture matrix.
///
/// The material table is an array of 0x34-byte records. `source == 0` writes
/// an identity matrix to `+0xAC` and sets bit 0 of `+0xAB`; otherwise the
/// source matrix is copied and that bit is cleared.
///
/// # Original
/// - Ghidra: `FUN_8023cfa0` at `0x8023CFA0` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cfa0_FUN_8023cfa0.c`
/// - `// inferred:` `0x801C04D0` fills a 3x4 identity and `0x801C0500` copies
///   one, so `+0xAC` is a matrix-shaped field.
///
/// # Arguments
/// - `this`: address of the material-table base pointer.
/// - `index`: material index, 0..7.
/// - `source`: matrix source address, or 0 for the identity.
///
/// # Returns
/// 1 on success, 0 when `index >= 8`.
///
/// # Side effects
/// Writes a matrix and updates one flag byte.
///
/// # Safety
/// A non-null table must contain eight 0x34-byte records; a non-zero `source`
/// must address a readable 3x4 matrix.
///
/// # Confidence
/// high - the guard, flag update and both callees are explicit.
pub unsafe fn mdl_texmtx_set(this: usize, index: u32, source: usize) -> u32 {
    if index >= 8 {
        return 0;
    }
    let record = platform::read_u32(this) as usize + index as usize * 0x34;
    let matrix = record + 0xAC;
    if source == 0 {
        platform::call1(PSMTX_IDENTITY, matrix as u32);
        platform::write_u8(
            record + 0xAB,
            platform::read_u8(record + 0xAB) | 1,
        );
    } else {
        platform::call2(PSMTX_COPY, source as u32, matrix as u32);
        platform::write_u8(
            record + 0xAB,
            platform::read_u8(record + 0xAB) & 0xFE,
        );
    }
    1
}

/// Stores the texgen selector bytes of one material.
///
/// # Original
/// - Ghidra: `FUN_8023d020` at `0x8023D020` (120 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d020_FUN_8023d020.c`
/// - `// inferred:` the three bytes precede the matrix field used by
///   [`mdl_texmtx_set`] in the same 0x34-byte record, so they describe how the
///   matrix is sourced.
///
/// # Arguments
/// - `this`: address of the material-table base pointer.
/// - `index`: material index, 0..7.
/// - `selector`: texgen index, must be below 0x100.
/// - `source`: clamped to 0x1F; values at or above 0x20 store 0xFF.
/// - `kind`: clamped to 0x7F; values at or above 0x80 store 0xFF.
///
/// # Returns
/// 1 on success, 0 when either range check fails.
///
/// # Side effects
/// Writes `+0xAA`, `+0xA8` and `+0xA9` of the selected record.
///
/// # Safety
/// A non-null table must contain eight 0x34-byte records.
///
/// # Confidence
/// high - both limits and the three byte stores are explicit.
pub unsafe fn mdl_texgen_set(
    this: usize,
    index: u32,
    selector: u32,
    source: u32,
    kind: u32,
) -> u32 {
    if index >= 8 || selector >= 0x100 {
        return 0;
    }
    let record = platform::read_u32(this) as usize + index as usize * 0x34;
    platform::write_u8(record + 0xAA, selector as u8);
    platform::write_u8(
        record + 0xA8,
        if source < 0x20 { source as u8 } else { 0xFF },
    );
    platform::write_u8(
        record + 0xA9,
        if kind < 0x80 { kind as u8 } else { 0xFF },
    );
    1
}

/// Stores the index byte of a texture-coordinate record.
///
/// # Original
/// - Ghidra: `FUN_8023d120` at `0x8023D120` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d120_FUN_8023d120.c`
/// - `// inferred:` the sibling getter at `0x8023D150` returns the same byte
///   as a signed value, and `0x8023D1B0` indexes three parallel byte pairs.
///
/// # Arguments
/// - `this`: address of the record pointer.
/// - `value`: stored when below 0x80, otherwise 0xFF is stored.
///
/// # Side effects
/// Writes `record + 1`.
///
/// # Safety
/// `*this` must be null or point at a readable/writable record.
///
/// # Confidence
/// high - the null guard and the 0x7F limit are explicit.
pub unsafe fn res_texcoord_set_index(this: usize, value: u32) {
    let record = platform::read_u32(this) as usize;
    if record == 0 {
        return;
    }
    if value > 0x7F {
        platform::write_u8(record + 1, 0xFF);
    } else {
        platform::write_u8(record + 1, value as u8);
    }
}

/// Returns the signed index byte of a texture-coordinate record.
///
/// # Original
/// - Ghidra: `FUN_8023d150` at `0x8023D150` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d150_FUN_8023d150.c`
///
/// # Arguments
/// - `this`: address of the record pointer.
///
/// # Returns
/// `(i8)record[+1]`, or -1 when the record pointer is null.
///
/// # Safety
/// `*this` must be null or point at a readable record.
///
/// # Confidence
/// high - the sign extension and null fallback are explicit.
pub unsafe fn res_texcoord_index(this: usize) -> i32 {
    let record = platform::read_u32(this) as usize;
    if record != 0 {
        platform::read_u8(record + 1) as i8 as i32
    } else {
        -1
    }
}

/// Reads one of three indexed byte pairs from a texture-coordinate record.
///
/// # Original
/// - Ghidra: `FUN_8023d1b0` at `0x8023D1B0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d1b0_FUN_8023d1b0.c`
/// - `// inferred:` the two parallel byte arrays at `+3` and `+7` are read by
///   the record-copy helper at `0x8023D200` as an 8-byte selector block.
///
/// # Arguments
/// - `this`: address of the record pointer.
/// - `index`: selector 1..3; anything else returns without writing.
/// - `out_byte`: optional word receiving `record[index + 3]`.
/// - `out_kind`: optional byte receiving `record[index + 7]`.
///
/// # Side effects
/// Writes each non-null output.
///
/// # Safety
/// `*this` must point at a readable record when non-null.
///
/// # Confidence
/// medium - the offsets are literal, the meaning of the two arrays is inferred.
pub unsafe fn res_texcoord_pair(this: usize, index: i32, out_byte: usize, out_kind: usize) {
    let record = platform::read_u32(this) as usize;
    if record == 0 {
        return;
    }
    if !(1..=3).contains(&index) {
        return;
    }
    if out_byte != 0 {
        platform::write_u32(
            out_byte,
            platform::read_u8(record + index as usize + 3) as u32,
        );
    }
    if out_kind != 0 {
        platform::write_u8(
            out_kind,
            platform::read_u8(record + index as usize + 7),
        );
    }
}

/// Copies the 12-byte header of a texture-coordinate record.
///
/// # Original
/// - Ghidra: `FUN_8023d200` at `0x8023D200` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d200_FUN_8023d200.c`
///
/// # Arguments
/// - `this`: address of the record pointer.
/// - `out`: destination for four bytes followed by two words.
///
/// # Returns
/// `out`.
///
/// # Side effects
/// Writes 12 bytes.
///
/// # Safety
/// `*this` must point at 12 readable bytes and `out` at 12 writable bytes.
///
/// # Confidence
/// high - the byte and word copies are explicit.
pub unsafe fn res_texcoord_copy(this: usize, out: usize) -> u32 {
    let record = platform::read_u32(this) as usize;
    let mut index = 0usize;
    while index < 4 {
        platform::write_u8(out + index, platform::read_u8(record + index));
        index += 1;
    }
    platform::write_u32(out + 4, platform::read_u32(record + 4));
    platform::write_u32(out + 8, platform::read_u32(record + 8));
    out as u32
}

/// Returns the normalized frame position of an animation object.
///
/// Mode 0 returns `(frame + 1) / frame_count` rounded to `f32`. Mode 1 applies
/// the easing exponent 0.6000000238418579 through `0x802E55A4`. Any other mode,
/// or a null object, returns `f32` zero.
///
/// # Original
/// - Ghidra: `FUN_8023d240` at `0x8023D240` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d240_FUN_8023d240.c`
/// - `// observed:` the two `f64` constants are `2^52` at `0x8042E520` and
///   0.6000000238418579 at `0x8042E518`; the fallback is `f32` zero at
///   `0x8042E510`.
///
/// # Arguments
/// - `this`: address of the animation-object pointer.
/// - `frame`: zero-based frame index.
///
/// # Returns
/// The frame ratio as `f64`, matching the original's `double` result.
///
/// # Safety
/// `*this` must be null or point at an object with a frame count at `+4` and a
/// mode at `+8`.
///
/// # Confidence
/// medium - the arithmetic is literal; the animation-object role is inferred
/// from the callers at `0x80244360` and `0x8024E180`.
pub unsafe fn anm_frame_ratio(this: usize, frame: i32) -> f64 {
    let object = platform::read_u32(this) as usize;
    if object == 0 {
        return platform::read_f32(SDA2_ZERO_RATIO) as f64;
    }
    let ratio = (((frame.wrapping_add(1) as u32) as f64)
        / (platform::read_u32(object + 4) as f64)) as f32;
    match platform::read_i32(object + 8) {
        0 => ratio as f64,
        1 => {
            let biased = call_f64_2(
                POW_F64,
                ratio as f64,
                read_f64(SDA2_EASE_EXPONENT),
            );
            (biased as f32) as f64
        }
        _ => platform::read_f32(SDA2_ZERO_RATIO) as f64,
    }
}

/// Reads one texture-coordinate record into four selector bytes.
///
/// The record is 0x14 bytes and its first byte is the presence flag. When the
/// flag is set, the routine packs two 12-bit values out of the high/low bytes
/// and copies two more bytes verbatim.
///
/// # Original
/// - Ghidra: `FUN_8023d300` at `0x8023D300` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d300_FUN_8023d300.c`
///
/// # Arguments
/// - `this`: address of the record-table base pointer.
/// - `index`: one-based record index.
/// - `out`: four output bytes.
///
/// # Returns
/// The presence flag as a boolean.
///
/// # Side effects
/// Writes four bytes only when the record is present.
///
/// # Safety
/// A non-null table must contain the selected 0x14-byte record; `out` must be
/// writable for four bytes.
///
/// # Confidence
/// high - every offset and shift is explicit.
pub unsafe fn res_texcoord_get(this: usize, index: i32, out: usize) -> bool {
    let offset = index.wrapping_sub(1).wrapping_mul(0x14);
    let record = ((platform::read_u32(this) as i32).wrapping_add(offset) as u32) as usize;
    let present = platform::read_u8(record) != 0;
    if present {
        let high0 = platform::read_u8(record + 7);
        let low0 = platform::read_u8(record + 8);
        let high1 = platform::read_u8(record + 2);
        let low1 = platform::read_u8(record + 3);
        platform::write_u8(out, platform::read_u8(record + 4));
        platform::write_u8(
            out + 1,
            (((high0 as u32) << 16 | (low0 as u32) << 8) >> 12) as u8,
        );
        platform::write_u8(out + 2, platform::read_u8(record + 9));
        platform::write_u8(
            out + 3,
            (((high1 as u32) << 16 | (low1 as u32) << 8) >> 12) as u8,
        );
    }
    present
}

/// Emits four GX BP commands for one texture-coordinate record.
///
/// # Original
/// - Ghidra: `FUN_8023d3c0` at `0x8023D3C0` (148 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d3c0_FUN_8023d3c0.c`
/// - `// inferred:` the 0xE0/0xE1 register numbers and the 0x61 command prefix
///   come from `0x80239980`; the record layout matches [`res_texcoord_get`].
///
/// # Arguments
/// - `this`: address of the record-table base pointer.
/// - `index`: one-based record index.
/// - `values`: four selector bytes read by this routine.
///
/// # Side effects
/// Writes four five-byte GX BP commands through `0x80239980`.
///
/// # Safety
/// The table must contain the selected 0x14-byte record; `values` must be
/// readable for four bytes.
///
/// # Confidence
/// medium - the command construction is literal, the register roles inferred.
pub unsafe fn res_texcoord_emit(this: usize, index: i32, values: usize) {
    let offset = index.wrapping_sub(1).wrapping_mul(0x14);
    let record = ((platform::read_u32(this) as i32).wrapping_add(offset) as u32) as usize;
    let first = index.wrapping_mul(2).wrapping_add(0xE0) as u32;
    let second = index.wrapping_mul(2).wrapping_add(0xE1) as u32;
    let word0 = (first << 24)
        | ((platform::read_u8(values + 3) as u32) << 12)
        | platform::read_u8(values) as u32;
    let word1 = (second << 24)
        | ((platform::read_u8(values + 1) as u32) << 12)
        | platform::read_u8(values + 2) as u32;
    platform::call2(GX_BP_WRITE, record as u32, word0);
    platform::call2(GX_BP_WRITE, (record + 5) as u32, word1);
    platform::call2(GX_BP_WRITE, (record + 10) as u32, word1);
    platform::call2(GX_BP_WRITE, (record + 15) as u32, word1);
}

/// Reads one colour-channel record into four selector bytes.
///
/// The record is 10 bytes and starts at `+0x40` in the table. Its first byte is
/// the presence flag; the payload is the same shape as the texture-coordinate
/// getter.
///
/// # Original
/// - Ghidra: `FUN_8023d460` at `0x8023D460` (184 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d460_FUN_8023d460.c`
///
/// # Arguments
/// - `this`: address of the record-table base pointer.
/// - `index`: record index.
/// - `out`: four output bytes.
///
/// # Returns
/// The presence flag as a boolean.
///
/// # Side effects
/// Writes four bytes only when the record is present.
///
/// # Safety
/// The table must contain the selected 10-byte record; `out` must be writable
/// for four bytes.
///
/// # Confidence
/// high - every offset and shift is explicit.
pub unsafe fn res_color_get(this: usize, index: i32, out: usize) -> bool {
    let offset = index.wrapping_mul(10);
    let record = ((platform::read_u32(this) as i32).wrapping_add(offset) as u32) as usize;
    let present = platform::read_u8(record + 0x40) != 0;
    if present {
        let high0 = platform::read_u8(record + 0x47);
        let low0 = platform::read_u8(record + 0x48);
        let high1 = platform::read_u8(record + 0x42);
        let low1 = platform::read_u8(record + 0x43);
        platform::write_u8(out, platform::read_u8(record + 0x44));
        platform::write_u8(
            out + 1,
            (((high0 as u32) << 16 | (low0 as u32) << 8) >> 12) as u8,
        );
        platform::write_u8(out + 2, platform::read_u8(record + 0x49));
        platform::write_u8(
            out + 3,
            (((high1 as u32) << 16 | (low1 as u32) << 8) >> 12) as u8,
        );
    }
    present
}

/// Emits two GX BP commands for one colour-channel record.
///
/// # Original
/// - Ghidra: `FUN_8023d520` at `0x8023D520` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d520_FUN_8023d520.c`
/// - `// inferred:` the record layout matches [`res_color_get`]; the extra
///   0x800000 bit is the colour-channel enable seen only in this emitter.
///
/// # Arguments
/// - `this`: address of the record-table base pointer.
/// - `index`: record index.
/// - `values`: four selector bytes read by this routine.
///
/// # Side effects
/// Writes two five-byte GX BP commands through `0x80239980`.
///
/// # Safety
/// The table must contain the selected 10-byte record; `values` must be
/// readable for four bytes.
///
/// # Confidence
/// medium - the command construction is literal, the register roles inferred.
pub unsafe fn res_color_emit(this: usize, index: i32, values: usize) {
    let offset = index.wrapping_mul(10);
    let record = ((platform::read_u32(this) as i32).wrapping_add(offset) as u32) as usize;
    let first = index.wrapping_mul(2).wrapping_add(0xE0) as u32;
    let second = index.wrapping_mul(2).wrapping_add(0xE1) as u32;
    let word0 = (first << 24)
        | ((platform::read_u8(values + 3) as u32) << 12)
        | platform::read_u8(values) as u32
        | 0x0080_0000;
    let word1 = (second << 24)
        | ((platform::read_u8(values + 1) as u32) << 12)
        | platform::read_u8(values + 2) as u32
        | 0x0080_0000;
    platform::call2(GX_BP_WRITE, (record + 0x40) as u32, word0);
    platform::call2(GX_BP_WRITE, (record + 0x45) as u32, word1);
}

/// Reads a texture object's data pointer and `+0x1D` byte for display list 8.
///
/// # Original
/// - Ghidra: `FUN_8023e7e0` at `0x8023E7E0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e7e0_FUN_8023e7e0.c`
/// - `// inferred:` the sibling routines at `0x8023E6C0`/`0x8023E7A0` emit
///   display lists 9 and 10 from the same `+0x08` offset and `+0x1D` byte.
///
/// # Arguments
/// - `this`: address of the word holding the texture object.
/// - `out_data`: optional word receiving `object + *(object + 8)`.
/// - `out_byte`: optional byte receiving `object[+0x1D]`.
///
/// # Side effects
/// Writes each non-null output.
///
/// # Safety
/// `*this` must point at a readable texture object.
///
/// # Confidence
/// high - both outputs and the offset guard are explicit.
pub unsafe fn res_texture_get_data_byte(this: usize, out_data: usize, out_byte: usize) {
    let object = platform::read_u32(this) as usize;
    if out_data != 0 {
        let offset = platform::read_u32(object + 8);
        let data = if offset == 0 {
            0
        } else {
            object.wrapping_add(offset as usize)
        };
        platform::write_u32(out_data, data as u32);
    }
    if out_byte != 0 {
        platform::write_u8(out_byte, platform::read_u8(object + 0x1D));
    }
}

/// Copies a texture object's image into the given cache block.
///
/// The first word of the object is the byte count; the routine copies that
/// many bytes and then runs the cache-block store/sync primitive.
///
/// # Original
/// - Ghidra: `FUN_8023e820` at `0x8023E820` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e820_FUN_8023e820.c`
/// - `// inferred:` the identical body at `0x8023E900` is used for palettes;
///   see [`res_palette_publish`].
///
/// # Arguments
/// - `this`: address of the word holding the texture object.
/// - `dst`: destination cache block.
///
/// # Side effects
/// Copies `*(object)` bytes and calls `0x801AC4A0`.
///
/// # Safety
/// A non-null object must have a readable size word and that many readable
/// bytes; `dst` must be writable for the same length.
///
/// # Confidence
/// medium - the copy is literal; the texture-object role comes from the caller
/// group.
pub unsafe fn res_texture_publish(this: usize, dst: usize) {
    let object = platform::read_u32(this) as usize;
    if object != 0 {
        let size = platform::read_u32(object);
        platform::call3(MEMCPY_BLOCKS_32, dst as u32, object as u32, size);
        platform::call2(DC_STORE_SYNC, dst as u32, size);
    }
}

/// Copies a palette object's image into the given cache block.
///
/// # Original
/// - Ghidra: `FUN_8023e900` at `0x8023E900` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e900_FUN_8023e900.c`
/// - `// observed:` the body is byte-for-byte the same as
///   [`res_texture_publish`] at `0x8023E820`.
///
/// # Arguments
/// - `this`: address of the word holding the palette object.
/// - `dst`: destination cache block.
///
/// # Side effects
/// Copies `*(object)` bytes and calls `0x801AC4A0`.
///
/// # Safety
/// A non-null object must have a readable size word and that many readable
/// bytes; `dst` must be writable for the same length.
///
/// # Confidence
/// medium - the copy is literal; the palette role is inferred from the
/// neighbouring palette accessors.
pub unsafe fn res_palette_publish(this: usize, dst: usize) {
    let object = platform::read_u32(this) as usize;
    if object != 0 {
        let size = platform::read_u32(object);
        platform::call3(MEMCPY_BLOCKS_32, dst as u32, object as u32, size);
        platform::call2(DC_STORE_SYNC, dst as u32, size);
    }
}

/// Emits display list 9 for one indexed texture entry.
///
/// The entry table begins at `object + *(object + 8)`; `+0x20` is the count,
/// `+0x24` the stride and `+0x1D` the byte forwarded to the GX FIFO. A null
/// object, a zero table offset, a negative index or an index at/above the count
/// emits a null data pointer instead.
///
/// # Original
/// - Ghidra: `FUN_8023e9a0` at `0x8023E9A0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e9a0_FUN_8023e9a0.c`
///
/// # Arguments
/// - `this`: address of the word holding the texture object.
/// - `index`: signed texture-table index.
///
/// # Side effects
/// Calls `0x801C4780(9, data, object[+0x1D])`.
///
/// # Safety
/// A non-null object must have readable fields at `+8`, `+0x1D`, `+0x20` and
/// `+0x24`; a valid index must address a readable entry.
///
/// # Confidence
/// high - every guard and offset is explicit.
pub unsafe fn res_texture_emit_indexed(this: usize, index: i32) {
    let object = platform::read_u32(this) as usize;
    if object == 0 {
        return;
    }
    let offset = platform::read_i32(object + 8);
    let count = platform::read_i32(object + 0x20);
    let stride = platform::read_i32(object + 0x24);
    let data = if offset != 0 && index < count && index > -1 {
        (object as i32)
            .wrapping_add(offset)
            .wrapping_add(index.wrapping_mul(stride)) as u32
    } else {
        0
    };
    platform::call3(
        GX_FIFO_WRITE,
        9,
        data,
        platform::read_u8(object + 0x1D) as u32,
    );
}

/// Merges one material's channel state into a dirty-mask state block.
///
/// The material state at `*this` has a flag word at `+0x14` and three-channel
/// values at `+0x20`/`+0x24`/`+0x28` (defaults to 1.0), three scale values at
/// `+0x2C`/`+0x30`/`+0x34` (scaled by 0.7111111 into the matrix at `+0x1C`), and
/// a second colour at `+0x38`/`+0x3C`/`+0x40`. The output block at `state`
/// carries a dirty mask in word 0 and mirrors those fields.
///
/// # Original
/// - Ghidra: `FUN_8023ecf0` at `0x8023ECF0` (412 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ecf0_FUN_8023ecf0.c`
/// - `// observed:` the `r2` constants resolve to `f32` 1.0 at `0x8042E548`,
///   0.7111111283302307 at `0x8042E54C` and `f32` 0.0 at `0x8042E550`.
/// - `// inferred:` the 0x80000000 bit and the `+0x1C` matrix field make this a
///   deferred material-state update, not a resource parser.
///
/// # Arguments
/// - `this`: address of the material-state pointer.
/// - `state`: 0x4C-byte output state block.
///
/// # Side effects
/// Writes several `state` words and may write a scaled 3x4 matrix.
///
/// # Safety
/// `*this` must point at a readable material state; `state` must be writable
/// through at least `+0x48`.
///
/// # Confidence
/// medium - every branch is literal; the flag bit names are inferred.
pub unsafe fn mdl_material_channel_sync(this: usize, state: usize) {
    let material = platform::read_u32(this) as usize;
    if material == 0 {
        return;
    }
    let mut flags = platform::read_u32(state);
    if (flags & 0x80) != 0 {
        if (platform::read_u32(material + 0x14) & 8) == 0 {
            let mut next = flags & !0x10;
            if (platform::read_u32(material + 0x14) & 0x10) != 0 {
                next = flags | 0x10;
            }
            platform::write_u32(state + 4, platform::read_u32(material + 0x20));
            flags = next & 0xFFFF_FFF5;
            platform::write_u32(state + 8, platform::read_u32(material + 0x24));
            platform::write_u32(state + 0x0C, platform::read_u32(material + 0x28));
        } else {
            let one = platform::read_u32(SDA2_ONE);
            flags |= 0x18;
            platform::write_u32(state + 0x0C, one);
            platform::write_u32(state + 8, one);
            platform::write_u32(state + 4, one);
        }
    }
    if (flags & 0x100) != 0 {
        let saved_12 = platform::read_u32(state + 0x48);
        let saved_e = platform::read_u32(state + 0x38);
        let saved_10 = platform::read_u32(state + 0x28);
        if (platform::read_u32(material + 0x14) & 4) == 0 {
            let scale = platform::read_f32(SDA2_MTX_SCALE);
            call_f64_3_ptr(
                MTX34_SCALE,
                (scale * platform::read_f32(material + 0x2C)) as f64,
                (scale * platform::read_f32(material + 0x30)) as f64,
                (scale * platform::read_f32(material + 0x34)) as f64,
                state + 0x1C,
            );
            flags &= 0xFFFF_FFD9;
        } else {
            platform::call1(PSMTX_IDENTITY, (state + 0x1C) as u32);
            flags |= 0x20;
        }
        flags |= 0x8000_0000;
        platform::write_u32(state + 0x28, saved_10);
        platform::write_u32(state + 0x38, saved_e);
        platform::write_u32(state + 0x48, saved_12);
    }
    if (flags & 0x200) != 0 {
        if (platform::read_u32(material + 0x14) & 2) == 0 {
            platform::write_u32(state + 0x28, platform::read_u32(material + 0x38));
            flags &= 0xFFFF_FFB9;
            platform::write_u32(state + 0x38, platform::read_u32(material + 0x3C));
            platform::write_u32(state + 0x48, platform::read_u32(material + 0x40));
        } else {
            let zero = platform::read_u32(SDA2_ZERO);
            flags |= 0x40;
            platform::write_u32(state + 0x48, zero);
            platform::write_u32(state + 0x38, zero);
            platform::write_u32(state + 0x28, zero);
        }
    }
    let mut result = flags;
    if (flags & 0x20) != 0 && (flags & 0x40) != 0 {
        result = flags | 4;
        if (flags & 8) != 0 {
            result = flags | 6;
        }
    }
    platform::write_u32(state, result & 0xFFFF_FC7F);
}

/// Writes one material's channel state into a fresh state block.
///
/// This is the unconditional sibling of [`mdl_material_channel_sync`]. It
/// copies the same fields, but builds the mask from scratch and always sets
/// `0x80000001` (plus the `+0x20`/`+0x40` bits when those flags are present).
///
/// # Original
/// - Ghidra: `FUN_8023ee90` at `0x8023EE90` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ee90_FUN_8023ee90.c`
/// - `// observed:` the `r2` constants resolve to `f32` 1.0 at `0x8042E548`,
///   0.7111111283302307 at `0x8042E54C` and `f32` 0.0 at `0x8042E550`.
/// - `// observed:` the original loads the 0.0 constant in the `+0x14 & 8`
///   branch but never uses it; that dead temporary is omitted here.
///
/// # Arguments
/// - `this`: address of the material-state pointer.
/// - `state`: 0x4C-byte output state block.
///
/// # Side effects
/// Writes several `state` words and may write a scaled 3x4 matrix.
///
/// # Safety
/// `*this` must point at a readable material state; `state` must be writable
/// through at least `+0x48`.
///
/// # Confidence
/// medium - every branch is literal; the flag bit names are inferred.
pub unsafe fn mdl_material_channel_apply(this: usize, state: usize) {
    let material = platform::read_u32(this) as usize;
    if material == 0 {
        return;
    }
    let mut flags = 0u32;
    if (platform::read_u32(material + 0x14) & 8) == 0 {
        if (platform::read_u32(material + 0x14) & 0x10) != 0 {
            flags = 0x10;
        }
        platform::write_u32(state + 4, platform::read_u32(material + 0x20));
        platform::write_u32(state + 8, platform::read_u32(material + 0x24));
        platform::write_u32(state + 0x0C, platform::read_u32(material + 0x28));
    } else {
        let one = platform::read_u32(SDA2_ONE);
        flags = 0x18;
        platform::write_u32(state + 0x0C, one);
        platform::write_u32(state + 8, one);
        platform::write_u32(state + 4, one);
    }
    if (platform::read_u32(material + 0x14) & 4) == 0 {
        let scale_x = platform::read_u32(material + 0x2C);
        let scale_y = platform::read_u32(material + 0x30);
        platform::write_u32(state + 0x18, platform::read_u32(material + 0x34));
        let scale = platform::read_f32(SDA2_MTX_SCALE);
        platform::write_u32(state + 0x10, scale_x);
        platform::write_u32(state + 0x14, scale_y);
        call_f64_3_ptr(
            MTX34_SCALE,
            (scale * platform::read_f32(material + 0x2C)) as f64,
            (scale * platform::read_f32(material + 0x30)) as f64,
            (scale * platform::read_f32(material + 0x34)) as f64,
            state + 0x1C,
        );
    } else {
        platform::call1(PSMTX_IDENTITY, (state + 0x1C) as u32);
        flags |= 0x20;
    }
    if (platform::read_u32(material + 0x14) & 2) == 0 {
        platform::write_u32(state + 0x28, platform::read_u32(material + 0x38));
        platform::write_u32(state + 0x38, platform::read_u32(material + 0x3C));
        platform::write_u32(state + 0x48, platform::read_u32(material + 0x40));
    } else {
        flags |= 0x40;
    }
    let mut result = flags;
    if (flags & 0x20) != 0 && (flags & 0x40) != 0 {
        result = flags | 4;
        if (flags & 8) != 0 {
            result = flags | 6;
        }
    }
    let mut final_flags = result | 0x8000_0001;
    if (platform::read_u32(material + 0x14) & 0x20) != 0 {
        final_flags = result | 0x8000_0401;
    }
    if (platform::read_u32(material + 0x14) & 0x40) != 0 {
        final_flags |= 0x800;
    }
    platform::write_u32(state, final_flags);
}

// ===========================================================================
// Wave 70: cached-block GX publish helpers, object-cache accessors and the
// six per-entry cache thunks. All of these sit in the material/texture state
// neighbourhood of 0x8023C610..0x8023EAC0 and share the pointer-to-pointer
// convention used by the wave-18 group above.
// ===========================================================================

/// Emits the cached block word count of one record to the GX FIFO.
///
/// The block address is `record + 0x18 + *(record + 0x20)` (a zero offset word
/// means "no block" and yields address 0) and the value written is the word
/// count at `*(record + 0x1C)`. `sync` selects the checked FIFO writer
/// `0x801C95E0`; otherwise the word is stored straight to `0xCC008000`.
/// `skip_header` moves the source on by one 0x20-byte header and subtracts that
/// header from the count.
///
/// # Original
/// - Ghidra: `FUN_8023c610` at `0x8023C610` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c610_FUN_8023c610.c`
/// - `// inferred:` the `+0x18`/`+0x1C`/`+0x20` triple has the same shape as
///   the `+0x24`/`+0x28`/`+0x2C` triple in [`mdl_aux_buffer_word_emit`].
///
/// # Arguments
/// - `this`: address of the word holding the record pointer.
/// - `sync`: non-zero selects the checked FIFO writer.
/// - `skip_header`: non-zero skips the first 0x20 bytes of the block.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word to the GX FIFO; the checked path may flush the FIFO first.
///
/// # Safety
/// `*this` must point at a readable record; a non-zero offset word must select
/// a readable block inside that record.
///
/// # Confidence
/// medium - the offset arithmetic and both branches are literal; the block's
/// concrete role is inferred from its neighbours.
pub unsafe fn mdl_buffer_word_emit(this: usize, sync: u32, skip_header: u32) {
    let record = platform::read_u32(this) as usize;
    let offset = platform::read_u32(record + GX_BLOCK_OFFSET);
    let block = if offset == 0 {
        0usize
    } else {
        record + GX_BLOCK_BASE + offset as usize
    };
    let size = platform::read_u32(record + GX_BLOCK_SIZE);
    if skip_header != 0 {
        let data = block.wrapping_add(GX_BLOCK_HEADER as usize) as u32;
        let words = size.wrapping_sub(GX_BLOCK_HEADER);
        if sync != 0 {
            platform::call2(GX_FIFO_WRITE_SYNC, data, words);
        } else {
            platform::write_u32(GX_FIFO, words);
        }
    } else if sync != 0 {
        platform::call2(GX_FIFO_WRITE_SYNC, block as u32, size);
    } else {
        platform::write_u32(GX_FIFO, size);
    }
}

/// Emits the second cached block's word count of one record to the GX FIFO.
///
/// Same shape as [`mdl_buffer_word_emit`], but the descriptor lives at
/// `+0x24`/`+0x28`/`+0x2C` and there is no header-skipping variant.
///
/// # Original
/// - Ghidra: `FUN_8023c6f0` at `0x8023C6F0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c6f0_FUN_8023c6f0.c`
/// - `// inferred:` the offset word, the count word and the base word differ
///   from `0x8023C610` only by 0xC, so this is a second block descriptor.
///
/// # Arguments
/// - `this`: address of the word holding the record pointer.
/// - `sync`: non-zero selects the checked FIFO writer.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes one word to the GX FIFO; the checked path may flush the FIFO first.
///
/// # Safety
/// `*this` must point at a readable record; a non-zero offset word must select
/// a readable block inside that record.
///
/// # Confidence
/// medium - the arithmetic and both branches are literal; the block's role is
/// inferred.
pub unsafe fn mdl_aux_buffer_word_emit(this: usize, sync: u32) {
    let record = platform::read_u32(this) as usize;
    let offset = platform::read_u32(record + GX_AUX_OFFSET);
    let block = if offset == 0 {
        0usize
    } else {
        record + GX_AUX_BASE + offset as usize
    };
    let size = platform::read_u32(record + GX_AUX_SIZE);
    if sync != 0 {
        platform::call2(GX_FIFO_WRITE_SYNC, block as u32, size);
    } else {
        platform::write_u32(GX_FIFO, size);
    }
}

/// Copies a 0x200-byte block and repairs its self-relative pointer.
///
/// The second word of the copied block is a self-relative pointer: the original
/// rebases it by subtracting the distance the block moved, then publishes the
/// result with a data-cache store of `*(dst)` bytes.
///
/// # Original
/// - Ghidra: `FUN_8023c9b0` at `0x8023C9B0` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c9b0_FUN_8023c9b0.c`
/// - `// inferred:` the rebasing subtracts the byte distance between source and
///   destination, which is the standard fix-up for a pointer stored relative to
///   its own block.
///
/// # Arguments
/// - `this`: address of the word holding the source block pointer.
/// - `dst`: destination block, at least 0x200 bytes.
///
/// # Returns
/// `dst`.
///
/// # Side effects
/// Copies 0x200 bytes, rewrites `dst + 4` and runs `0x801AC500` over `*(dst)`
/// bytes of the destination.
///
/// # Safety
/// `*this` must point at 0x200 readable bytes; `dst` must be writable for 0x200
/// bytes and its first word must hold a byte count that fits in the block.
///
/// # Confidence
/// medium - the copy, the subtraction and the cache call are literal; the
/// pointer role of the second word is inferred.
pub unsafe fn res_block_copy_200(this: usize, dst: usize) -> u32 {
    let src = platform::read_u32(this) as usize;
    platform::call3(MEMCPY_BLOCKS_32, dst as u32, src as u32, 0x200);
    let delta = (dst as i32).wrapping_sub(src as i32) as u32;
    let rebased = platform::read_u32(dst + 4).wrapping_sub(delta);
    platform::write_u32(dst + 4, rebased);
    let size = platform::read_u32(dst);
    platform::call2(DC_STORE, dst as u32, size);
    dst as u32
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// `mode != 0` takes the synchronising store-plus-`syscall` path; `mode == 0`
/// takes the plain cache-store path over `*(*this)` bytes.
///
/// # Original
/// - Ghidra: `FUN_8023ca20` at `0x8023CA20` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ca20_FUN_8023ca20.c`
/// - `// observed:` the body is byte-for-byte the same as `0x8023CAE0`; both
///   call `FUN_801ac4a0` with no visible arguments, although the callee itself
///   takes `(data, size)`.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over the resource bytes.
///
/// # Safety
/// `*this` must point at readable bytes and its first word must be that block's
/// byte count; the block must be writable for the store path.
///
/// # Confidence
/// medium - the two branches are literal; the synchronising branch's arguments
/// are unresolved in the Ghidra output, as in [`res_palette_bind`].
pub unsafe fn res_cache_publish(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Flushes or cache-stores one 0x20-byte resource block.
///
/// # Original
/// - Ghidra: `FUN_8023ca60` at `0x8023CA60` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ca60_FUN_8023ca60.c`
/// - `// inferred:` the fixed 0x20 size matches the texture-object stride used
///   by [`res_texture_object_copy`].
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising store path.
///
/// # Side effects
/// Executes data-cache block operations over 0x20 bytes.
///
/// # Safety
/// `*this` must point at 0x20 readable and writable bytes.
///
/// # Confidence
/// high - the size, both callees and both branches are literal.
pub unsafe fn res_cache_publish_20(this: usize, mode: u32) {
    let data = platform::read_u32(this);
    if mode != 0 {
        platform::call2(DC_STORE_SYNC, data, RES_CACHE_BLOCK_20);
    } else {
        platform::call2(DC_STORE, data, RES_CACHE_BLOCK_20);
    }
}

/// Flushes or cache-stores one 0x80-byte resource block.
///
/// # Original
/// - Ghidra: `FUN_8023ca80` at `0x8023CA80` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ca80_FUN_8023ca80.c`
/// - `// inferred:` 0x80 is the four-slot texture-cache block size chosen by
///   [`res_cache_publish_counted`].
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising store path.
///
/// # Side effects
/// Executes data-cache block operations over 0x80 bytes.
///
/// # Safety
/// `*this` must point at 0x80 readable and writable bytes.
///
/// # Confidence
/// high - the size, both callees and both branches are literal.
pub unsafe fn res_cache_publish_80(this: usize, mode: u32) {
    let data = platform::read_u32(this);
    if mode != 0 {
        platform::call2(DC_STORE_SYNC, data, RES_CACHE_BLOCK_80);
    } else {
        platform::call2(DC_STORE, data, RES_CACHE_BLOCK_80);
    }
}

/// Flushes or cache-stores one 0x40-byte resource block.
///
/// # Original
/// - Ghidra: `FUN_8023caa0` at `0x8023CAA0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023caa0_FUN_8023caa0.c`
/// - `// inferred:` 0x40 is the two-slot texture-cache block size chosen by
///   [`res_cache_publish_counted`].
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising store path.
///
/// # Side effects
/// Executes data-cache block operations over 0x40 bytes.
///
/// # Safety
/// `*this` must point at 0x40 readable and writable bytes.
///
/// # Confidence
/// high - the size, both callees and both branches are literal.
pub unsafe fn res_cache_publish_40(this: usize, mode: u32) {
    let data = platform::read_u32(this);
    if mode != 0 {
        platform::call2(DC_STORE_SYNC, data, RES_CACHE_BLOCK_40);
    } else {
        platform::call2(DC_STORE, data, RES_CACHE_BLOCK_40);
    }
}

/// Flushes or cache-stores one 0xA0-byte resource block.
///
/// # Original
/// - Ghidra: `FUN_8023cac0` at `0x8023CAC0` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cac0_FUN_8023cac0.c`
/// - `// inferred:` 0xA0 is the full texture-cache block size chosen by
///   [`res_cache_publish_counted`].
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising store path.
///
/// # Side effects
/// Executes data-cache block operations over 0xA0 bytes.
///
/// # Safety
/// `*this` must point at 0xA0 readable and writable bytes.
///
/// # Confidence
/// high - the size, both callees and both branches are literal.
pub unsafe fn res_cache_publish_a0(this: usize, mode: u32) {
    let data = platform::read_u32(this);
    if mode != 0 {
        platform::call2(DC_STORE_SYNC, data, RES_CACHE_BLOCK_A0);
    } else {
        platform::call2(DC_STORE, data, RES_CACHE_BLOCK_A0);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// Byte-identical to [`res_cache_publish`]; it exists as its own symbol in the
/// original, so it is kept as its own body rather than aliased.
///
/// # Original
/// - Ghidra: `FUN_8023cae0` at `0x8023CAE0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cae0_FUN_8023cae0.c`
/// - `// observed:` the Ghidra body matches `0x8023CA20` instruction for
///   instruction.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the plain branch is explicit; the synchronising branch's arguments
/// are unresolved in the Ghidra output.
pub unsafe fn res_cache_publish_alt(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Emits a cached 0x20-byte block size word to the GX FIFO.
///
/// `mode != 0` writes through the checked FIFO writer; `mode == 0` stores the
/// constant straight to `0xCC008000`. The block pointer is not used by the
/// direct-store path, matching the original.
///
/// # Original
/// - Ghidra: `FUN_8023cb00` at `0x8023CB00` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cb00_FUN_8023cb00.c`
/// - `// inferred:` the constant is the word count that accompanies a 0x20-byte
///   texture-object block, the same size used by [`res_cache_publish_20`].
///
/// # Arguments
/// - `this`: address of the word holding the stream block pointer.
/// - `mode`: non-zero selects the checked FIFO writer.
///
/// # Side effects
/// Writes one FIFO word; the checked path may flush the FIFO first.
///
/// # Safety
/// `*this` must be readable; the FIFO at `0xCC008000` must be mapped.
///
/// # Confidence
/// high - the constant, both callees and both branches are literal.
pub unsafe fn res_gx_word_emit_20(this: usize, mode: u32) {
    if mode != 0 {
        let data = platform::read_u32(this);
        platform::call2(GX_FIFO_WRITE_SYNC, data, RES_CACHE_BLOCK_20);
    } else {
        platform::write_u32(GX_FIFO, RES_CACHE_BLOCK_20);
    }
}

/// Emits a cached 0x80-byte block size word to the GX FIFO.
///
/// Same shape as [`res_gx_word_emit_20`] with the 0x80 count.
///
/// # Original
/// - Ghidra: `FUN_8023cb30` at `0x8023CB30` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cb30_FUN_8023cb30.c`
/// - `// inferred:` the constant is the word count that accompanies a 0x80-byte
///   block, the same size used by [`res_cache_publish_80`].
///
/// # Arguments
/// - `this`: address of the word holding the stream block pointer.
/// - `mode`: non-zero selects the checked FIFO writer.
///
/// # Side effects
/// Writes one FIFO word; the checked path may flush the FIFO first.
///
/// # Safety
/// `*this` must be readable; the FIFO at `0xCC008000` must be mapped.
///
/// # Confidence
/// high - the constant, both callees and both branches are literal.
pub unsafe fn res_gx_word_emit_80(this: usize, mode: u32) {
    if mode != 0 {
        let data = platform::read_u32(this);
        platform::call2(GX_FIFO_WRITE_SYNC, data, RES_CACHE_BLOCK_80);
    } else {
        platform::write_u32(GX_FIFO, RES_CACHE_BLOCK_80);
    }
}

/// Copies a 0x80-byte texture-cache block out of the record.
///
/// # Original
/// - Ghidra: `FUN_8023cba0` at `0x8023CBA0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cba0_FUN_8023cba0.c`
/// - `// inferred:` 0x80 is the four-slot texture-cache block size; the
///   surrounding copies at `0x8023CBE0`/`0x8023CC20` use 0x40/0xA0.
///
/// # Arguments
/// - `this`: address of the word holding the source block pointer.
/// - `dst`: destination for the copy.
///
/// # Returns
/// `dst`.
///
/// # Side effects
/// Copies 0x80 bytes through `0x8024A530`.
///
/// # Safety
/// `*this` must point at 0x80 readable bytes; `dst` must be writable for 0x80
/// bytes.
///
/// # Confidence
/// medium - the size and return are literal; the block role is inferred from
/// the neighbouring fixed-size copies.
pub unsafe fn res_block_copy_80(this: usize, dst: usize) -> u32 {
    let src = platform::read_u32(this);
    platform::call3(MEMCPY_BLOCKS_32, dst as u32, src, RES_CACHE_BLOCK_80);
    dst as u32
}

/// Copies a 0x40-byte texture-cache block out of the record.
///
/// # Original
/// - Ghidra: `FUN_8023cbe0` at `0x8023CBE0` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cbe0_FUN_8023cbe0.c`
/// - `// inferred:` 0x40 is the two-slot texture-cache block size.
///
/// # Arguments
/// - `this`: address of the word holding the source block pointer.
/// - `dst`: destination for the copy.
///
/// # Returns
/// `dst`.
///
/// # Side effects
/// Copies 0x40 bytes through `0x8024A530`.
///
/// # Safety
/// `*this` must point at 0x40 readable bytes; `dst` must be writable for 0x40
/// bytes.
///
/// # Confidence
/// medium - the size and return are literal; the block role is inferred.
pub unsafe fn res_block_copy_40(this: usize, dst: usize) -> u32 {
    let src = platform::read_u32(this);
    platform::call3(MEMCPY_BLOCKS_32, dst as u32, src, RES_CACHE_BLOCK_40);
    dst as u32
}

/// Copies a 0xA0-byte texture-cache block out of the record.
///
/// # Original
/// - Ghidra: `FUN_8023cc20` at `0x8023CC20` (60 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cc20_FUN_8023cc20.c`
/// - `// inferred:` 0xA0 is the full texture-cache block size.
///
/// # Arguments
/// - `this`: address of the word holding the source block pointer.
/// - `dst`: destination for the copy.
///
/// # Returns
/// `dst`.
///
/// # Side effects
/// Copies 0xA0 bytes through `0x8024A530`.
///
/// # Safety
/// `*this` must point at 0xA0 readable bytes; `dst` must be writable for 0xA0
/// bytes.
///
/// # Confidence
/// medium - the size and return are literal; the block role is inferred.
pub unsafe fn res_block_copy_a0(this: usize, dst: usize) -> u32 {
    let src = platform::read_u32(this);
    platform::call3(MEMCPY_BLOCKS_32, dst as u32, src, RES_CACHE_BLOCK_A0);
    dst as u32
}

/// Returns the address of one texture-object cache slot payload.
///
/// The slot is bounds-checked against eight entries; a null cache base or an
/// out-of-range index returns 0. The returned pointer skips the slot's 4-byte
/// present word. This is the second of the two identical slot accessors; the
/// first, `FUN_8023cc60`, is already carried by `runtime::bt`.
///
/// # Original
/// - Ghidra: `FUN_8023cc90` at `0x8023CC90` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cc90_FUN_8023cc90.c`
/// - `// observed:` the Ghidra body matches `0x8023CC60` instruction for
///   instruction.
///
/// # Arguments
/// - `this`: address of the cache base pointer.
/// - `index`: texture-object slot, 0..7.
///
/// # Returns
/// `base + index * 0x20 + 4`, or 0.
///
/// # Safety
/// A non-null base must have eight readable 0x20-byte slots.
///
/// # Confidence
/// high - the guard, the stride and the `+4` are literal.
pub unsafe fn res_texture_object_ptr_alt(this: usize, index: u32) -> u32 {
    let base = platform::read_u32(this);
    if base != 0 && index < RES_CACHE_SLOTS {
        base.wrapping_add(index.wrapping_mul(RES_CACHE_TEXTURE_STRIDE as u32))
            .wrapping_add(RES_CACHE_FIRST_OBJECT as u32)
    } else {
        0
    }
}

/// Returns the address of one palette-object cache slot payload.
///
/// The slot is bounds-checked against eight entries; a null cache base or an
/// out-of-range index returns 0. This is the second of the two identical
/// palette slot accessors; the first, `FUN_8023cd90`, is already carried by
/// `runtime::bt`.
///
/// # Original
/// - Ghidra: `FUN_8023cdc0` at `0x8023CDC0` (44 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023cdc0_FUN_8023cdc0.c`
/// - `// observed:` the Ghidra body matches `0x8023CD90` instruction for
///   instruction.
///
/// # Arguments
/// - `this`: address of the cache base pointer.
/// - `index`: palette-object slot, 0..7.
///
/// # Returns
/// `base + index * 0x0C + 4`, or 0.
///
/// # Safety
/// A non-null base must have eight readable 0x0C-byte slots.
///
/// # Confidence
/// high - the guard, the stride and the `+4` are literal.
pub unsafe fn res_palette_object_ptr_alt(this: usize, index: u32) -> u32 {
    let base = platform::read_u32(this);
    if base != 0 && index < RES_CACHE_SLOTS {
        base.wrapping_add(index.wrapping_mul(RES_CACHE_PALETTE_STRIDE as u32))
            .wrapping_add(RES_CACHE_FIRST_OBJECT as u32)
    } else {
        0
    }
}

/// Stores a byte count into the block descriptor's size word.
///
/// A null block pointer is silently ignored, so this is safe to call on an
/// unattached cache record.
///
/// # Original
/// - Ghidra: `FUN_8023d0a0` at `0x8023D0A0` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d0a0_FUN_8023d0a0.c`
/// - `// inferred:` the word at `+4` of the pointed-to block is read as a byte
///   count by [`res_cache_publish`] and by the descriptor helpers above.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `size`: byte count to store at `block + 4`.
///
/// # Side effects
/// Writes one word when the block pointer is non-null.
///
/// # Safety
/// A non-null block must be writable for 8 bytes.
///
/// # Confidence
/// high - the null guard and the single store are literal.
pub unsafe fn res_cache_set_size(this: usize, size: u32) {
    let block = platform::read_u32(this);
    if block != 0 {
        platform::write_u32(block as usize + 4, size);
    }
}

/// Copies the 8-byte header of the record referenced by `*this`.
///
/// Four leading bytes are copied individually, then the word at `+4` is copied
/// as a unit; Ghidra shows the split because the first four bytes are read as
/// `undefined1` values.
///
/// # Original
/// - Ghidra: `FUN_8023d0c0` at `0x8023D0C0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d0c0_FUN_8023d0c0.c`
///
/// # Arguments
/// - `this`: address of the word holding the source record pointer.
/// - `dst`: destination for the 8-byte copy.
///
/// # Returns
/// `dst`.
///
/// # Side effects
/// Writes 8 bytes.
///
/// # Safety
/// `*this` must point at 8 readable bytes; `dst` must be writable for 8 bytes.
///
/// # Confidence
/// high - every load and store is explicit.
pub unsafe fn res_record_copy_8(this: usize, dst: usize) -> u32 {
    let src = platform::read_u32(this) as usize;
    let mut index = 0usize;
    while index < 4 {
        platform::write_u8(dst + index, platform::read_u8(src + index));
        index += 1;
    }
    platform::write_u32(dst + 4, platform::read_u32(src + 4));
    dst as u32
}

/// Returns the first byte of the record referenced by `*this`.
///
/// # Original
/// - Ghidra: `FUN_8023d100` at `0x8023D100` (28 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d100_FUN_8023d100.c`
/// - `// inferred:` this is the record's "is attached" byte; the sibling at
///   `0x8023D190` reads the signed byte at `+2`, so the record starts with
///   three selector-ish bytes.
///
/// # Arguments
/// - `this`: address of the word holding the record pointer.
///
/// # Returns
/// The byte at offset 0, or 0 when the pointer is null.
///
/// # Safety
/// A non-null record must have at least one readable byte.
///
/// # Confidence
/// medium - the load and the null default are literal; the byte's role is
/// inferred.
pub unsafe fn res_record_first_byte(this: usize) -> u8 {
    let record = platform::read_u32(this) as usize;
    if record != 0 {
        platform::read_u8(record)
    } else {
        0
    }
}

/// Returns the signed selector byte at `record + 2`.
///
/// # Original
/// - Ghidra: `FUN_8023d190` at `0x8023D190` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d190_FUN_8023d190.c`
/// - `// inferred:` Ghidra renders the load as `*(char *)`, so the byte is
///   sign-extended; the companion reader at `0x8023D100` returns the unsigned
///   byte at offset 0.
///
/// # Arguments
/// - `this`: address of the word holding the record pointer.
///
/// # Returns
/// The sign-extended byte at offset 2, or -1 when the pointer is null.
///
/// # Safety
/// A non-null record must have at least three readable bytes.
///
/// # Confidence
/// high - the sign extension and the -1 default are literal.
pub unsafe fn res_record_selector_byte(this: usize) -> i32 {
    let record = platform::read_u32(this) as usize;
    if record != 0 {
        platform::read_u8(record + 2) as i8 as i32
    } else {
        -1
    }
}

/// Emits the 0x40-byte publish word when the record is present.
///
/// A zero `present` flag returns without touching the GX stream, so an
/// unattached record produces no command. Otherwise `mode` selects the checked
/// FIFO writer over the direct `0xCC008000` store.
///
/// # Original
/// - Ghidra: `FUN_8023d5b0` at `0x8023D5B0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d5b0_FUN_8023d5b0.c`
/// - `// inferred:` 0x40 is the two-slot cache block size used by
///   [`res_cache_publish_40`] and [`res_block_copy_40`].
///
/// # Arguments
/// - `this`: address of the word holding the source block pointer.
/// - `present`: zero suppresses the emission entirely.
/// - `mode`: non-zero selects the checked FIFO writer.
///
/// # Side effects
/// Writes one FIFO word; the checked path may flush the FIFO first.
///
/// # Safety
/// `*this` must be readable when `present` is non-zero; the FIFO at
/// `0xCC008000` must be mapped.
///
/// # Confidence
/// high - the guard, the constant and both emission paths are literal.
pub unsafe fn res_cache_emit_40(this: usize, present: u32, mode: u32) {
    if present == 0 {
        return;
    }
    if mode != 0 {
        let data = platform::read_u32(this);
        platform::call2(GX_FIFO_WRITE_SYNC, data, RES_CACHE_BLOCK_40);
    } else {
        platform::write_u32(GX_FIFO, RES_CACHE_BLOCK_40);
    }
}

/// Copies the four selector bytes of one of two 0x14-byte variants.
///
/// `selector & 1` picks the first or second variant record and the four bytes
/// at `+4..+7` are copied out. The function always reports success.
///
/// # Original
/// - Ghidra: `FUN_8023dc70` at `0x8023DC70` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023dc70_FUN_8023dc70.c`
/// - `// inferred:` the 0x14 stride matches the texcoord records used by
///   [`res_texcoord_get`], and the companion reader at `0x8023DCB0` selects
///   the same pair.
///
/// # Arguments
/// - `this`: address of the word holding the record base pointer.
/// - `selector`: bit 0 chooses the variant.
/// - `dst`: destination for four bytes.
///
/// # Returns
/// 1.
///
/// # Side effects
/// Writes four bytes.
///
/// # Safety
/// `*this` must point at two readable 0x14-byte variants; `dst` must be
/// writable for four bytes.
///
/// # Confidence
/// high - the stride, the byte copies and the constant return are literal.
pub unsafe fn res_texcoord_variant_copy(this: usize, selector: u32, dst: usize) -> u32 {
    let record = platform::read_u32(this).wrapping_add((selector & 1).wrapping_mul(0x14)) as usize;
    let mut index = 0usize;
    while index < 4 {
        platform::write_u8(dst + index, platform::read_u8(record + 4 + index));
        index += 1;
    }
    1
}

/// Decodes the packed flags of one of two 0x14-byte variants into six outputs.
///
/// `selector & 1` picks the variant and `selector & 2` picks which of the two
/// words at `+0xC`/`+0x10` is decoded. Every output pointer is optional; a null
/// pointer skips that field.
///
/// # Original
/// - Ghidra: `FUN_8023dcb0` at `0x8023DCB0` (192 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023dcb0_FUN_8023dcb0.c`
/// - `// inferred:` the two-bit kind result (0 = absent, 1 = direct, 2 =
///   indirect) and the packed nibbles look like a texcoord source descriptor,
///   matching the `0x8023DC70` companion reader.
///
/// # Arguments
/// - `this`: address of the word holding the record base pointer.
/// - `selector`: bit 0 chooses the variant, bit 1 chooses the word.
/// - `out_mode`: receives bit 1 as a byte, or null.
/// - `out_bit6`: receives bit 6, or null.
/// - `out_bit0`: receives bit 0, or null.
/// - `out_nibbles`: receives the combined nibble byte, or null.
/// - `out_value`: receives the two-bit value, or null.
/// - `out_kind`: receives 0, 1 or 2, or null.
///
/// # Returns
/// 1.
///
/// # Side effects
/// Writes up to six output fields.
///
/// # Safety
/// `*this` must point at two readable 0x14-byte variants; every non-null output
/// pointer must be suitably aligned and writable.
///
/// # Confidence
/// high - every mask, shift and store is explicit in the export.
pub unsafe fn res_texcoord_variant_fields(
    this: usize,
    selector: u32,
    out_mode: usize,
    out_bit6: usize,
    out_bit0: usize,
    out_nibbles: usize,
    out_value: usize,
    out_kind: usize,
) -> u32 {
    let record = platform::read_u32(this).wrapping_add((selector & 1).wrapping_mul(0x14)) as usize;
    let word = if (selector & 2) == 0 {
        platform::read_u32(record + 0x0C)
    } else {
        platform::read_u32(record + 0x10)
    };
    if out_mode != 0 {
        platform::write_u8(out_mode, ((word >> 1) & 1) as u8);
    }
    if out_bit6 != 0 {
        platform::write_u32(out_bit6, (word >> 6) & 1);
    }
    if out_bit0 != 0 {
        platform::write_u32(out_bit0, word & 1);
    }
    if out_nibbles != 0 {
        let low = (word >> 2) & 0xF;
        let high = (word >> 0x0B) & 0xF;
        platform::write_u32(out_nibbles, low | (high << 4));
    }
    let (value, kind) = if (word >> 10) & 1 == 0 {
        (0, 0)
    } else if (word >> 9) & 1 == 0 {
        ((word >> 7) & 3, 2)
    } else {
        ((word >> 7) & 3, 1)
    };
    if out_value != 0 {
        platform::write_u32(out_value, value);
    }
    if out_kind != 0 {
        platform::write_u32(out_kind, kind);
    }
    1
}

/// Publishes a cache block whose size is selected by the populated slot count.
///
/// The four thresholds pick 0x20, 0x40, 0x80 or 0xA0 bytes: fewer than two
/// populated slots, fewer than four, fewer than eight, otherwise the full
/// block. `mode` selects the checked FIFO writer over the direct FIFO store,
/// and the size word is what reaches the FIFO in both cases.
///
/// # Original
/// - Ghidra: `FUN_8023ddd0` at `0x8023DDD0` (208 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ddd0_FUN_8023ddd0.c`
/// - `// inferred:` the thresholds match the four fixed-size publishers above
///   (`0x8023CA60`/`0x8023CAA0`/`0x8023CA80`/`0x8023CAC0`), so the count is
///   the number of populated object slots.
///
/// # Arguments
/// - `this`: address of the word holding the source block pointer.
/// - `count`: populated slot count that selects the block size.
/// - `mode`: non-zero selects the checked FIFO writer.
///
/// # Returns
/// Nothing; a `count` of 0 returns without touching the FIFO.
///
/// # Side effects
/// Writes one FIFO word; the checked path may flush the FIFO first.
///
/// # Safety
/// `*this` must be readable; the FIFO at `0xCC008000` must be mapped.
///
/// # Confidence
/// high - all four thresholds and both emission paths are literal.
pub unsafe fn res_cache_publish_counted(this: usize, count: u32, mode: u32) {
    if count == 0 {
        return;
    }
    let data = platform::read_u32(this);
    let size = if count < 2 {
        RES_CACHE_BLOCK_20
    } else if count < 4 {
        RES_CACHE_BLOCK_40
    } else if count < 8 {
        RES_CACHE_BLOCK_80
    } else {
        RES_CACHE_BLOCK_A0
    };
    if mode != 0 {
        platform::call2(GX_FIFO_WRITE_SYNC, data, size);
    } else {
        platform::write_u32(GX_FIFO, size);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// One of six byte-identical thunks at `0x8023EA20..0x8023EAC0`; each is a
/// separate symbol in the original, so each keeps its own body.
///
/// # Original
/// - Ghidra: `FUN_8023ea20` at `0x8023EA20` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ea20_FUN_8023ea20.c`
/// - `// observed:` identical body at `0x8023EA40`, `0x8023EA60`, `0x8023EA80`,
///   `0x8023EAA0` and `0x8023EAC0`.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the body is literal; the six thunks' individual roles are not
/// distinguishable from the Ghidra output.
pub unsafe fn res_entry_cache_sync_0(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// Second of the six identical thunks; see [`res_entry_cache_sync_0`].
///
/// # Original
/// - Ghidra: `FUN_8023ea40` at `0x8023EA40` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ea40_FUN_8023ea40.c`
/// - `// observed:` same body as `0x8023EA20`.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the body is literal; the thunk's role is inferred.
pub unsafe fn res_entry_cache_sync_1(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// Third of the six identical thunks; see [`res_entry_cache_sync_0`].
///
/// # Original
/// - Ghidra: `FUN_8023ea60` at `0x8023EA60` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ea60_FUN_8023ea60.c`
/// - `// observed:` same body as `0x8023EA20`.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the body is literal; the thunk's role is inferred.
pub unsafe fn res_entry_cache_sync_2(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// Fourth of the six identical thunks; see [`res_entry_cache_sync_0`].
///
/// # Original
/// - Ghidra: `FUN_8023ea80` at `0x8023EA80` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023ea80_FUN_8023ea80.c`
/// - `// observed:` same body as `0x8023EA20`.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the body is literal; the thunk's role is inferred.
pub unsafe fn res_entry_cache_sync_3(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// Fifth of the six identical thunks; see [`res_entry_cache_sync_0`].
///
/// # Original
/// - Ghidra: `FUN_8023eaa0` at `0x8023EAA0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023eaa0_FUN_8023eaa0.c`
/// - `// observed:` same body as `0x8023EA20`.
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the body is literal; the thunk's role is inferred.
pub unsafe fn res_entry_cache_sync_4(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

/// Flushes or cache-stores one resource block sized by its first word.
///
/// Sixth and last of the identical thunks; see [`res_entry_cache_sync_0`].
///
/// # Original
/// - Ghidra: `FUN_8023eac0` at `0x8023EAC0` (24 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023eac0_FUN_8023eac0.c`
/// - `// observed:` same body as `0x8023EA20`; the next function,
///   `0x8023EAE0`, is the already-translated [`res_palette_bind`].
///
/// # Arguments
/// - `this`: address of the word holding the block pointer.
/// - `mode`: non-zero selects the synchronising path.
///
/// # Side effects
/// Executes data-cache block operations over `*(*this)` bytes.
///
/// # Safety
/// As for [`res_cache_publish`].
///
/// # Confidence
/// medium - the body is literal; the thunk's role is inferred.
pub unsafe fn res_entry_cache_sync_5(this: usize, mode: u32) {
    if mode != 0 {
        platform::call0(DC_STORE_SYNC);
    } else {
        let data = platform::read_u32(this);
        let size = platform::read_u32(data as usize);
        platform::call2(DC_STORE, data, size);
    }
}

// ===========================================================================
// Wave 85: G3D material records, packed transforms and NW4R math
// ===========================================================================
//
// This pass finishes the remaining `0x8023C280..0x8023F5A0` group: the
// polygon display-list builder, the texture-coordinate record pair, the
// packed 11-bit matrix codec, the 0x12-byte attribute records and the two
// per-entry animation resolvers. It also adds four small NW4R G3D math
// helpers (`0x80238420..0x80239710`) from the same address bucket.
//
// Everything here is main-DOL code. `0x802DCD5C..0x802DCD74` are CodeWarrior
// register-save prologues, not real functions: Ghidra models each as a call
// returning a `u64` whose **high** half is the incoming `r3` and whose **low**
// half is `r4`. The matching `0x802DCDA8..0x802DCDBC` helpers restore the
// saved registers, so the `u64` a body re-reads is that body's own incoming
// argument pair; the functions below take the hidden `r3`/`r4` values as
// ordinary arguments.

/// `0x802399B0` - write a 0x10-style tagged record (three byte groups).
///
/// `// inferred:` sibling of [`GX_BP_WRITE`] and `0x80239A40`; the texgen
/// writer at `0x8023C830` uses it for the per-stage command bytes.
const TAG_RECORD_WRITE: usize = 0x8023_99B0;
/// `0x80239A10` - write a type-`0x08` tagged record.
///
/// `// observed:` the body is five bytes: `0x08`, the tag, then the big-endian
/// value. Translated as `fn_80239a10` in `src/runtime/bt.rs`; called by
/// address here so this module does not depend on that file's visibility.
const TAG_RECORD_WRITE_08: usize = 0x8023_9A10;
/// `0x80239A40` - merge masked bits into a type-`0x10` record.
///
/// `// observed:` translated as `fn_80239a40` in `src/runtime/bt.rs`.
const TAG_RECORD_MERGE_10: usize = 0x8023_9A40;

/// `0x801C6D90` - `GXInitTexObj(obj, data, w, h, format, wrap_s, wrap_t, mip)`.
const GX_INIT_TEXOBJ: usize = 0x801C_6D90;
/// `0x801C6FA0` - `GXInitTexObjCI(obj, data, w, h, format, wrap_s, wrap_t,
/// mip, tlut)`.
const GX_INIT_TEXOBJ_CI: usize = 0x801C_6FA0;
/// `0x801C6FF0` - `GXInitTexObjLOD(obj, min, mag, min_lod, max_lod, bias,
/// bias_clamp, edge_lod, max_aniso)`; the three float arguments arrive in
/// `f1..f3`.
const GX_INIT_TEXOBJ_LOD: usize = 0x801C_6FF0;
/// `0x801C7520` - `GXInitTlutObj(obj, data, format, entries)`.
const GX_INIT_TLUT: usize = 0x801C_7520;
/// `0x8023F4E0` - sample one animation key track (translated later; called by
/// address from [`fn_8023f5a0`]).
const ANM_KEY_SAMPLE: usize = 0x8023_F4E0;
/// `0x8023F140` - interpolate one packed RGBA key (called by address from
/// [`fn_8023f370`]).
const ANM_KEY_COLOR: usize = 0x8023_F140;

/// Texgen record: stride between consecutive material stages.
const TEXGEN_RECORD_STRIDE: usize = 0x30;
/// Texgen record: present flag byte.
const TEXGEN_RECORD_PRESENT: usize = 0x8A;
/// Texgen record: first packed byte of the stage descriptor (three bytes).
const TEXGEN_RECORD_DESCRIPTOR: usize = 0x8C;
/// Texgen record: destination base of the per-stage command bytes.
const TEXGEN_RECORD_COMMANDS: usize = 0x10;
/// `0x802F84B0` - read-only 32-bit table selected by the texgen descriptor.
const TEXGEN_SELECT_TABLE: usize = 0x802F_84B0;
/// `0x8034E0C0` - byte table indexed by the texgen source selector.
const TEXGEN_SOURCE_TABLE: usize = 0x8034_E0C0;

/// Packed 11-bit matrix record: first slot (stage 1).
const PACKED_MTX_SLOT_1: usize = 0x0A;
/// Packed 11-bit matrix record: second slot (stage 2).
const PACKED_MTX_SLOT_2: usize = 0x20;
/// Packed 11-bit matrix record: third slot (stage 3).
const PACKED_MTX_SLOT_3: usize = 0x2F;
/// Packed matrix exponent bias (`0x11` subtracted from the six-bit exponent).
const PACKED_MTX_EXPONENT_BIAS: i32 = 0x11;
/// Packed matrix decode scale: the 11-bit signed field is a 1/1024 fixed
/// point value.
///
/// `// inferred:` `0x8023DAA0` encodes with the reciprocal `1024.0`, so this
/// is the only scale that round-trips its own output.
const PACKED_MTX_DECODE_SCALE: f32 = 1.0 / 1024.0;
/// Packed matrix encode scale; see [`PACKED_MTX_DECODE_SCALE`].
const PACKED_MTX_ENCODE_SCALE: f32 = 1024.0;

/// `0x8023C280` - build one polygon's texture and palette display lists.
///
/// The incoming `r3` is the address of the 0x30-byte polygon entry. The body
/// resolves the polygon's material (`+0x48`) and palette (`+0x4A`), the two
/// texture slots at `+0x4C` and the eight texture slots at `+0x50`, then
/// rewrites the matching 6-byte command records in the polygon's display-list
/// block (`model + 0x18 + *(model + 0x20)`). It publishes the block and the
/// `+0x24` block through the data-cache store helper.
///
/// # Original
/// - Ghidra: `FUN_8023c280` at `0x8023C280` (908 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c280_FUN_8023c280.c`
/// - `// observed:` `+0x48` selects the material, `+0x4A` the palette, `+0x4C`
///   two texture slots and `+0x50` eight texture slots; tags are `0xA0`/`0xB0`
///   plus the slot number.
///
/// # Arguments
/// - `this`: address of the polygon entry word (incoming `r3`; the Ghidra body
///   is `void FUN_8023c280(void)` because the register-save prologue hid it).
///
/// # Side effects
/// Writes 6-byte command records into the display-list block, copies cached
/// blocks through `0x801AC500` and restores the saved registers.
///
/// # Safety
/// `*(this)` must address a readable MDL0 body; the display-list block must be
/// writable.
///
/// # Confidence
/// medium - every offset, tag and call is literal; the polygon/material
/// ownership comes from [`mdl_model_bind`]'s call site.
pub unsafe fn fn_8023c280(this: usize) {
    let model = platform::read_u32(this) as usize;

    let mut material_section = crate::platform::GuestLocal::new( section_or_zero(model, 0x04) as u32);
    let material = mdl_material_by_index(
        (material_section.as_mut_ptr() as *mut u32) as usize,
        platform::read_u16(model + 0x48) as u32,
    );

    let mut texture_data = crate::platform::GuestLocal::new( [0u32; 9]);
    let mut texture_flag = crate::platform::GuestLocal::new( [0u8; 4]);
    let mut material_word = crate::platform::GuestLocal::new( material);
    mdl_material_get_texture_9(
        (material_word.as_mut_ptr() as *mut u32) as usize,
        texture_data.as_mut_ptr() as usize,
        texture_flag.as_mut_ptr() as usize,
    );
    let data_pointer = texture_data[0];
    let flag_byte = texture_flag[0];

    let display_list_offset = platform::read_u32(model + 0x20);
    let display_list = if display_list_offset == 0 {
        0
    } else {
        model + 0x18 + display_list_offset as usize
    };
    platform::call3(
        TAG_RECORD_WRITE_08,
        (display_list + 0x32) as u32,
        0xA0,
        data_pointer.wrapping_sub(0x8000_0000),
    );
    platform::call3(
        TAG_RECORD_WRITE_08,
        (display_list + 0x38) as u32,
        0xB0,
        flag_byte as u32,
    );

    // One palette slot at +0x4A.
    let palette_index = platform::read_u16(model + 0x4A);
    if palette_index != 0xFFFF {
        let mut palette_section = crate::platform::GuestLocal::new( section_or_zero(model, 0x04) as u32);
        let mut palette = crate::platform::GuestLocal::new( mdl_palette_by_index(
            (palette_section.as_mut_ptr() as *mut u32) as usize,
            palette_index as u32,
        ));
        if *palette != 0 {
            let mut palette_data = crate::platform::GuestLocal::new( [0u32; 9]);
            let mut palette_flag = crate::platform::GuestLocal::new( [0u8; 4]);
            mdl_material_get_texture_10(
                (palette.as_mut_ptr() as *mut u32) as usize,
                palette_data.as_mut_ptr() as usize,
                palette_flag.as_mut_ptr() as usize,
            );
            platform::call3(
                TAG_RECORD_WRITE_08,
                (display_list + 0x3E) as u32,
                0xA1,
                palette_data[0].wrapping_sub(0x8000_0000),
            );
            platform::call3(
                TAG_RECORD_WRITE_08,
                (display_list + 0x44) as u32,
                0xB1,
                palette_flag[0] as u32,
            );
        }
    }

    // Two texture slots at +0x4C.
    let mut slot = 0usize;
    while slot < 2 {
        let index = platform::read_u16(model + 0x4C + slot * 2);
        if index != 0xFFFF {
            let mut texture_section = crate::platform::GuestLocal::new( section_or_zero(model, 0x04) as u32);
            let mut texture = crate::platform::GuestLocal::new( mdl_texture_by_index(
                (texture_section.as_mut_ptr() as *mut u32) as usize,
                index as u32,
            ));
            if *texture != 0 {
                let mut texture_out = crate::platform::GuestLocal::new( [0u32; 9]);
                let mut texture_flag_out = crate::platform::GuestLocal::new( [0u8; 4]);
                mdl_material_get_texture_indexed(
                    (texture.as_mut_ptr() as *mut u32) as usize,
                    texture_out.as_mut_ptr() as usize,
                    texture_flag_out.as_mut_ptr() as usize,
                );
                let command = display_list + (slot + 2) * 0x0C;
                let tag_low = (slot + 2) as u32;
                platform::call3(
                    TAG_RECORD_WRITE_08,
                    (command + 0x32) as u32,
                    (tag_low + 0xA0) & 0xFF,
                    texture_out[0].wrapping_sub(0x8000_0000),
                );
                platform::call3(
                    TAG_RECORD_WRITE_08,
                    (command + 0x38) as u32,
                    (tag_low + 0xB0) & 0xFF,
                    texture_flag_out[0] as u32,
                );
            }
        }
        slot += 1;
    }

    // Eight palette slots at +0x50; the command base is (slot + 4) * 0x0C.
    let mut slot = 0usize;
    while slot < 8 {
        let index = platform::read_u16(model + 0x50 + slot * 2);
        if index != 0xFFFF {
            let mut palette_section = crate::platform::GuestLocal::new( section_or_zero(model, 0x04) as u32);
            let mut palette = crate::platform::GuestLocal::new( mdl_palette_by_index(
                (palette_section.as_mut_ptr() as *mut u32) as usize,
                index as u32,
            ));
            if *palette != 0 {
                let mut palette_out = crate::platform::GuestLocal::new( [0u32; 9]);
                let mut palette_flag_out = crate::platform::GuestLocal::new( [0u8; 4]);
                mdl_material_get_texture_10(
                    (palette.as_mut_ptr() as *mut u32) as usize,
                    palette_out.as_mut_ptr() as usize,
                    palette_flag_out.as_mut_ptr() as usize,
                );
                let command = display_list + (slot + 4) * 0x0C;
                let tag_low = (slot + 4) as u32;
                platform::call3(
                    TAG_RECORD_WRITE_08,
                    (command + 0x32) as u32,
                    (tag_low + 0xA0) & 0xFF,
                    palette_out[0].wrapping_sub(0x8000_0000),
                );
                platform::call3(
                    TAG_RECORD_WRITE_08,
                    (command + 0x38) as u32,
                    (tag_low + 0xB0) & 0xFF,
                    palette_flag_out[0] as u32,
                );
            }
        }
        slot += 1;
    }

    platform::call2(DC_STORE, display_list as u32, 0xE0);

    let aux_offset = platform::read_u32(model + 0x2C);
    let aux = if aux_offset == 0 {
        0
    } else {
        model + 0x24 + aux_offset as usize
    };
    platform::call2(DC_STORE, aux as u32, platform::read_u32(model + 0x24));
}

/// Returns `base + *(base + offset)`, or 0 when the offset word is zero
/// (internal helper).
///
/// # Safety
/// `base` must point at a readable object with a word at `offset`.
#[inline]
unsafe fn section_or_zero(base: usize, offset: usize) -> usize {
    let delta = platform::read_u32(base + offset);
    if delta == 0 {
        0
    } else {
        base.wrapping_add(delta as usize)
    }
}

/// Reads one big-endian 32-bit group from the packed matrix record.
///
/// # Safety
/// `base + offset .. base + offset + 4` must be readable.
#[inline]
unsafe fn packed_group(base: usize, offset: usize) -> u32 {
    ((platform::read_u8(base + offset) as u32) << 24)
        | ((platform::read_u8(base + offset + 1) as u32) << 16)
        | ((platform::read_u8(base + offset + 2) as u32) << 8)
        | platform::read_u8(base + offset + 3) as u32
}

/// Sign-extends the low 11 bits of `value` (the packed fixed-point field).
#[inline]
fn sign_extend_11(value: u32) -> i32 {
    ((value << 21) as i32) >> 21
}

/// Returns `2^exponent` for the packed matrix exponent field.
#[inline]
fn packed_matrix_exponent_scale(exponent: i32) -> f32 {
    let mut scale = 1.0f32;
    let mut left = exponent;
    if left < 0 {
        while left != 0 {
            scale *= 0.5;
            left += 1;
        }
    } else {
        while left != 0 {
            scale *= 2.0;
            left -= 1;
        }
    }
    scale
}

/// `0x80238420` - transform one 3-vector by a 3x4 matrix.
///
/// # Original
/// - Ghidra: `FUN_80238420` at `0x80238420` (144 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238420_FUN_80238420.c`
/// - `// observed:` `out = m[0..2] * v.x + m[4..6] * v.y + m[8..10] * v.z`;
///   the translation column is not added.
///
/// # Arguments
/// - `out`: destination for three floats.
/// - `matrix`: twelve floats, column-major 3x4.
/// - `vector`: three floats.
///
/// # Safety
/// All three pointers must address the sizes above.
///
/// # Confidence
/// high - the nine multiply-adds are literal.
pub unsafe fn fn_80238420(out: usize, matrix: usize, vector: usize) {
    let vx = platform::read_f32(vector);
    let vy = platform::read_f32(vector + 4);
    let vz = platform::read_f32(vector + 8);
    let m0 = platform::read_f32(matrix);
    let m1 = platform::read_f32(matrix + 4);
    let m2 = platform::read_f32(matrix + 8);
    let m4 = platform::read_f32(matrix + 0x10);
    let m5 = platform::read_f32(matrix + 0x14);
    let m6 = platform::read_f32(matrix + 0x18);
    let m8 = platform::read_f32(matrix + 0x20);
    let m9 = platform::read_f32(matrix + 0x24);
    let m10 = platform::read_f32(matrix + 0x28);
    platform::write_f32(out, m2 * vz + m0 * vx + m1 * vy);
    platform::write_f32(out + 4, m6 * vz + m4 * vx + m5 * vy);
    platform::write_f32(out + 8, m10 * vz + m8 * vx + m9 * vy);
}

/// `0x80238690` - expand an axis-aligned box over an array of points.
///
/// The first point initialises both the minimum and the maximum, then every
/// following point updates each axis independently.
///
/// # Original
/// - Ghidra: `FUN_80238690` at `0x80238690` (188 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238690_FUN_80238690.c`
///
/// # Arguments
/// - `out`: six floats (min xyz, max xyz).
/// - `points`: `count` points of three floats each.
/// - `count`: point count; values below 2 only copy the first point.
///
/// # Safety
/// `out` must hold six floats and `points` must hold `count * 3` floats.
///
/// # Confidence
/// high - the comparison and store pattern is literal.
pub unsafe fn fn_80238690(out: usize, points: usize, count: u32) {
    let x = platform::read_f32(points);
    let y = platform::read_f32(points + 4);
    let z = platform::read_f32(points + 8);
    platform::write_f32(out, x);
    platform::write_f32(out + 4, y);
    platform::write_f32(out + 8, z);
    platform::write_f32(out + 0x0C, x);
    platform::write_f32(out + 0x10, y);
    platform::write_f32(out + 0x14, z);
    if count < 2 {
        return;
    }
    let mut remaining = count - 1;
    let mut point = points + 0x0C;
    while remaining != 0 {
        let px = platform::read_f32(point);
        if platform::read_f32(out) <= px {
            if platform::read_f32(out + 0x0C) < px {
                platform::write_f32(out + 0x0C, px);
            }
        } else {
            platform::write_f32(out, px);
        }
        let py = platform::read_f32(point + 4);
        if platform::read_f32(out + 4) <= py {
            if platform::read_f32(out + 0x10) < py {
                platform::write_f32(out + 0x10, py);
            }
        } else {
            platform::write_f32(out + 4, py);
        }
        let pz = platform::read_f32(point + 8);
        if platform::read_f32(out + 8) <= pz {
            if platform::read_f32(out + 0x14) < pz {
                platform::write_f32(out + 0x14, pz);
            }
        } else {
            platform::write_f32(out + 8, pz);
        }
        remaining -= 1;
        point += 0x0C;
    }
}

/// `0x80238750` - transform an axis-aligned box by a 3x4 matrix.
///
/// Each output axis takes the minimum of the two transformed corners for the
/// lower bound and the maximum for the upper bound.
///
/// # Original
/// - Ghidra: `FUN_80238750` at `0x80238750` (472 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238750_FUN_80238750.c`
///
/// # Arguments
/// - `out`: six floats (min xyz, max xyz).
/// - `box`: six floats (min xyz, max xyz).
/// - `matrix`: twelve floats, column-major 3x4.
///
/// # Safety
/// All three pointers must address the sizes above.
///
/// # Confidence
/// high - the six corner combinations are literal.
pub unsafe fn fn_80238750(out: usize, box_ptr: usize, matrix: usize) {
    let min_x = platform::read_f32(box_ptr);
    let min_y = platform::read_f32(box_ptr + 4);
    let min_z = platform::read_f32(box_ptr + 8);
    let max_x = platform::read_f32(box_ptr + 0x0C);
    let max_y = platform::read_f32(box_ptr + 0x10);
    let max_z = platform::read_f32(box_ptr + 0x14);

    let mut minimums = [0.0f32; 3];
    let mut maximums = [0.0f32; 3];
    let mut axis = 0usize;
    while axis < 3 {
        let row = matrix + axis * 0x10;
        let translation = platform::read_f32(row + 0x0C);
        let scale_x = platform::read_f32(row);
        let scale_y = platform::read_f32(row + 4);
        let scale_z = platform::read_f32(row + 8);

        let ax = translation + scale_x * min_x;
        let bx = translation + scale_x * max_x;
        let ay = scale_y * min_y;
        let by = scale_y * max_y;
        let az = scale_z * min_z;
        let bz = scale_z * max_z;

        let (lo_x, hi_x) = if bx < ax { (bx, ax) } else { (ax, bx) };
        let (lo_y, hi_y) = if ay < by { (ay, by) } else { (by, ay) };
        let (lo_z, hi_z) = if az < bz { (az, bz) } else { (bz, az) };

        minimums[axis] = lo_x + lo_y + lo_z;
        maximums[axis] = hi_x + hi_y + hi_z;
        axis += 1;
    }

    platform::write_f32(out, minimums[0]);
    platform::write_f32(out + 4, minimums[1]);
    platform::write_f32(out + 8, minimums[2]);
    platform::write_f32(out + 0x0C, maximums[0]);
    platform::write_f32(out + 0x10, maximums[1]);
    platform::write_f32(out + 0x14, maximums[2]);
}

/// `0x80239710` - tests two axis-aligned boxes for overlap.
///
/// # Original
/// - Ghidra: `FUN_80239710` at `0x80239710` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239710_FUN_80239710.c`
///
/// # Returns
/// 1 when the boxes overlap on all three axes, 0 otherwise.
///
/// # Safety
/// Both pointers must address six readable floats.
///
/// # Confidence
/// high - the six range tests are literal.
pub unsafe fn fn_80239710(a: usize, b: usize) -> u32 {
    if platform::read_f32(a) <= platform::read_f32(b + 0x0C)
        && platform::read_f32(b) <= platform::read_f32(a + 0x0C)
        && platform::read_f32(a + 4) <= platform::read_f32(b + 0x10)
        && platform::read_f32(b + 4) <= platform::read_f32(a + 0x10)
        && platform::read_f32(a + 8) <= platform::read_f32(b + 0x14)
        && platform::read_f32(b + 8) <= platform::read_f32(a + 0x14)
    {
        1
    } else {
        0
    }
}

/// Reads one texture-coordinate stage record and returns its three selector
/// fields.
///
/// The record is 0x30 bytes; `index / 2` selects the record and `index & 1`
/// selects the half of its packed 24-bit descriptor. The word at
/// [`TEXGEN_SELECT_TABLE`] is returned through `out_table`, the low three bits
/// of the descriptor through `out_value`, and the derived source byte through
/// `out_flag` (0xFF when bit 0 of the half-word is clear).
///
/// # Original
/// - Ghidra: `FUN_8023c760` at `0x8023C760` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c760_FUN_8023c760.c`
/// - `// observed:` present byte `+0x8A`, descriptor bytes `+0x8C..+0x8E`;
///   the table is indexed by a 5-bit field times four.
///
/// # Arguments
/// - `this`: address of the word holding the record base.
/// - `index`: stage record index (the low bit selects one of two descriptors).
/// - `out_value`, `out_flag`, `out_table`: optional output addresses.
///
/// # Returns
/// 1 when the record is present, 0 otherwise.
///
/// # Safety
/// `*(this)` must point at a readable record array; each non-null output must
/// be writable.
///
/// # Confidence
/// high - every shift, mask and table read is literal.
pub unsafe fn fn_8023c760(
    this: usize,
    index: u32,
    out_value: usize,
    out_flag: usize,
    out_table: usize,
) -> u32 {
    let base = platform::read_u32(this) as usize;
    let record = base + (index as i32 / 2) as usize * TEXGEN_RECORD_STRIDE;
    if platform::read_u8(record + TEXGEN_RECORD_PRESENT) == 0 {
        return 0;
    }

    let first = platform::read_u8(record + TEXGEN_RECORD_DESCRIPTOR);
    let high = ((first as u32) << 16)
        | ((platform::read_u8(record + TEXGEN_RECORD_DESCRIPTOR + 1) as u32) << 8);
    let mut packed = high | platform::read_u8(record + TEXGEN_RECORD_DESCRIPTOR + 2) as u32;

    let value_field;
    let table_field;
    let mut source_field;
    if (index & 1) == 0 {
        value_field = packed >> 3;
        table_field = (packed >> 5) & 0x1C;
        source_field = packed >> 6;
    } else {
        value_field = high >> 0xF;
        table_field = ((first >> 1) as u32) & 0x1C;
        source_field = (first >> 2) as u32;
        packed = high >> 0xC;
    }

    if out_value != 0 {
        platform::write_u32(out_value, value_field & 7);
    }
    if out_table != 0 {
        platform::write_u32(
            out_table,
            platform::read_u32(TEXGEN_SELECT_TABLE + table_field as usize),
        );
    }
    if (source_field & 1) == 0 {
        packed = 0xFF;
    } else {
        packed &= 7;
    }
    if out_flag != 0 {
        platform::write_u32(out_flag, packed);
    }
    1
}

/// Rewrites two stage bytes of one texgen record and merges the packed
/// descriptor into the stage's command word.
///
/// # Original
/// - Ghidra: `FUN_8023c830` at `0x8023C830` (284 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023c830_FUN_8023c830.c`
/// - `// observed:` clears the old source byte at `+0x10 + old`, stores the new
///   one at `+0x10 + slot`, then merges a 32-bit word through `0x802399B0`.
/// - `// unknown:` the selector into `0x8034E0C0`; the table is not named.
///
/// # Arguments
/// - `index`: stage index (incoming `r3` behind the register-save prologue).
/// - `slot`: destination byte slot, or 0xFF to leave it unchanged.
/// - `value`: low byte written to `slot`; bit 8 suppresses the "present" bit.
/// - `source`: 4-bit index into `0x8034E0C0`.
///
/// # Side effects
/// Writes bytes in the record and one 5-byte command; restores the saved
/// registers.
///
/// # Safety
/// The context word must address a readable record array and the command base
/// must be writable.
///
/// # Confidence
/// medium - the arithmetic is literal; the table byte's role is inferred.
pub unsafe fn fn_8023c830(index: u32, context: usize, slot: u32, value: u32, source: u32) {
    let base = platform::read_u32(context) as usize;

    let mut value_out = crate::platform::GuestLocal::<u32>::new( 0u32);
    let mut flag_out = crate::platform::GuestLocal::<u32>::new( 0u32);
    let found = fn_8023c760(
        context,
        index,
        (value_out.as_mut_ptr() as *mut u32) as usize,
        (flag_out.as_mut_ptr() as *mut u32) as usize,
        0,
    );
    if found != 0 && *value_out != 0xFF && *flag_out != 0xFF {
        platform::write_u8(base + *value_out as usize + TEXGEN_RECORD_COMMANDS, 0xFF);
    }

    if slot != 0xFF {
        platform::write_u8(
            base + slot as usize + TEXGEN_RECORD_COMMANDS,
            value as u8,
        );
    }

    let shift = 0u32.wrapping_sub(index & 1) & 0x0C;
    let present = if value != 0xFF && (value & 0x100) == 0 {
        1u32
    } else {
        0u32
    };
    let table_byte =
        platform::read_u8(TEXGEN_SOURCE_TABLE + (source & 0xF) as usize) as u32;
    let descriptor = ((table_byte << 7)
        | (value & 7)
        | ((slot & 7) << 3)
        | (present << 6))
        << shift;
    let stage_word = ((index as i32 / 2 + 0x28) as u32).wrapping_mul(0x0100_0000);
    let mask = (0x3FFu32 << shift) | 0xFF00_0000;
    let command = base + (index as i32 / 2) as usize * TEXGEN_RECORD_STRIDE + 0x8A;
    platform::call3(
        TAG_RECORD_WRITE,
        command as u32,
        descriptor | stage_word,
        mask,
    );
}

/// Decodes one 0x12-byte attribute record into a texture format selector.
///
/// # Original
/// - Ghidra: `FUN_8023dea0` at `0x8023DEA0` (428 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023dea0_FUN_8023dea0.c`
/// - `// observed:` present byte 0, flag word `+0x05..+0x08`, colour byte
///   `+0x08`, and scale selector `+0x10`/`+0x11`.
///
/// # Arguments
/// - `this`: address of the word holding the record array.
/// - `index`: record index (0x12-byte stride).
/// - `out_mode`, `out_selector`, `out_flag`, `out_value`: optional outputs.
///
/// # Returns
/// 1 when the record is present and the selector is known, 0 otherwise.
///
/// # Safety
/// `*(this)` must address a readable record array; each non-null output must
/// be writable.
///
/// # Confidence
/// medium - the bitfields are literal; the output roles are inferred from the
/// GX texture-format tables.
pub unsafe fn fn_8023dea0(
    this: usize,
    index: u32,
    out_mode: usize,
    out_selector: usize,
    out_flag: usize,
    out_value: usize,
) -> u32 {
    let record = platform::read_u32(this) as usize + index as usize * 0x12;
    if platform::read_u8(record) == 0 {
        return 0;
    }

    let low = (platform::read_u8(record + 7) as u32) << 8;
    let colour = platform::read_u8(record + 8) as u32;
    let word = ((platform::read_u8(record + 5) as u32) << 24)
        | ((platform::read_u8(record + 6) as u32) << 16)
        | low;
    let scale_byte = platform::read_u8(record + 0x11);

    if out_flag != 0 {
        platform::write_u32(out_flag, (platform::read_u8(record + 0x10) & 1) as u32);
    }
    if out_value != 0 {
        platform::write_u32(out_value, scale_byte as u32 + 0x40);
    }

    let family = (colour & 0x70) >> 4;
    let selector = ((word | colour) >> 7) & 0x1F;
    if family == 0 {
        let mode = if selector.wrapping_sub(5) < 8 {
            selector.wrapping_sub(1) as i32
        } else if selector == 0 {
            0
        } else if selector == 1 {
            1
        } else if selector == 3 {
            2
        } else if selector == 4 {
            3
        } else {
            return 0;
        };
        if out_mode != 0 {
            platform::write_u32(out_mode, (((colour & 2) >> 1) ^ 1) as u32);
        }
        if out_selector != 0 {
            platform::write_u32(out_selector, mode as u32);
        }
        1
    } else if selector == 2 {
        if out_mode != 0 {
            platform::write_u32(out_mode, 10);
        }
        if out_selector != 0 {
            let mode = if family == 2 { 0x13 } else { 0x14 };
            platform::write_u32(out_selector, mode);
        }
        1
    } else if family == 1 {
        if out_mode != 0 {
            platform::write_u32(out_mode, (word >> 0xF) + 2);
        }
        if out_selector != 0 {
            platform::write_u32(out_selector, ((low & 0x7000) >> 0xC) + 0x0C);
        }
        1
    } else {
        0
    }
}

/// Writes the two 9-byte command records of one 0x12-byte attribute record.
///
/// # Original
/// - Ghidra: `FUN_8023e050` at `0x8023E050` (500 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e050_FUN_8023e050.c`
/// - `// observed:` the first command word packs the mode, family and scale
///   fields; the second packs the colour and the signed selector.
///
/// # Arguments
/// - `this`: address of the word holding the record array.
/// - `index`: record index (0x12-byte stride).
/// - `kind`: source texture format (0..0x14).
/// - `mode`: destination mode (0..0x14).
/// - `colour`: colour byte written to the second record.
/// - `selector`: signed selector written to the second record.
///
/// # Safety
/// `*(this)` must address a writable record array.
///
/// # Confidence
/// medium - the switch table is literal; the format names are not recovered.
pub unsafe fn fn_8023e050(
    this: usize,
    index: u32,
    kind: u32,
    mode: u32,
    colour: u32,
    selector: u32,
) {
    let record = platform::read_u32(this) as usize + index as usize * 0x12;

    let mut first = 5u32;
    let mut second = 5u32;
    let mut bit2 = 0u32;
    let mut bit1 = 0u32;
    let mut bit3 = 0u32;
    match mode {
        0 => {
            first = 0;
            bit3 = 1;
        }
        1 => {
            first = 1;
            bit3 = 1;
        }
        2 => {
            first = 3;
            bit3 = 1;
        }
        3 => {
            first = 4;
            bit3 = 1;
        }
        4 => first = 5,
        5 => first = 6,
        6 => first = 7,
        7 => first = 8,
        8 => first = 9,
        9 => first = 10,
        10 => first = 0x0B,
        11 => first = 0x0C,
        12 => second = 0,
        13 => second = 1,
        14 => second = 2,
        15 => second = 3,
        16 => second = 4,
        17 => second = 5,
        18 => second = 6,
        19 => first = 2,
        20 => first = 2,
        _ => {}
    }

    let mut scale_bits = kind.wrapping_sub(2);
    let mut selector_bits;
    if scale_bits < 8 {
        selector_bits = 1;
    } else {
        scale_bits = 0;
        if kind == 1 {
            selector_bits = 0;
        } else if kind == 0 {
            selector_bits = 0;
            bit2 = 1;
        } else if kind == 10 {
            selector_bits = if mode == 0x13 { 2 } else { 3 };
        } else {
            return;
        }
    }

    let first_word = (scale_bits << 0xF)
        | (second << 0x0C)
        | (first << 7)
        | (selector_bits << 4)
        | (bit2 << 1)
        | (bit3 << 2);
    platform::call3(
        TAG_RECORD_MERGE_10,
        record as u32,
        (index + 0x1040) & 0xFFFF,
        first_word,
    );
    platform::call3(
        TAG_RECORD_MERGE_10,
        (record + 9) as u32,
        (index + 0x1050) & 0xFFFF,
        selector.wrapping_sub(0x40) | ((colour & 0xFF) << 8),
    );
}

/// Returns the packed 11-bit matrix slot inside a G3D transform record, or 0
/// when the stage selector is not 1..3 (internal helper).
///
/// # Safety
/// `base` must address the 0x3F-byte record.
#[inline]
unsafe fn packed_matrix_slot(base: usize, index: u32) -> usize {
    match index {
        1 => base + PACKED_MTX_SLOT_1,
        2 => base + PACKED_MTX_SLOT_2,
        3 => base + PACKED_MTX_SLOT_3,
        _ => 0,
    }
}

/// Splits the three packed groups of one matrix slot into six signed 11-bit
/// fields and the six-bit exponent (internal helper).
///
/// # Safety
/// `slot .. slot + 0x0F` must be readable.
#[inline]
unsafe fn packed_matrix_fields(slot: usize) -> ([i32; 6], i32) {
    let group_0 = packed_group(slot, 1);
    let group_1 = packed_group(slot, 6);
    let group_2 = packed_group(slot, 0xB);
    let exponent = ((((group_2 >> 22) & 3) << 4)
        | (((group_1 >> 22) & 3) << 2)
        | ((group_0 >> 22) & 3)) as i32
        - PACKED_MTX_EXPONENT_BIAS;
    let fields = [
        sign_extend_11(group_0 & 0x7FF),
        sign_extend_11((group_0 >> 11) & 0x7FF),
        sign_extend_11(group_1 & 0x7FF),
        sign_extend_11((group_1 >> 11) & 0x7FF),
        sign_extend_11(group_2 & 0x7FF),
        sign_extend_11((group_2 >> 11) & 0x7FF),
    ];
    (fields, exponent)
}

/// Writes the 14-byte packed matrix of one stage into a 12-float matrix.
///
/// # Original
/// - Ghidra: `FUN_8023d5f0` at `0x8023D5F0` (696 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d5f0_FUN_8023d5f0.c`
/// - `// observed:` three 32-bit groups hold two signed 11-bit fields each and
///   two exponent bits; the exponent is biased by `0x11`.
/// - `// inferred:` the `1/1024` scale and the `2^52 + 2^31` conversion bias
///   are the PPC fixed-point and signed-int-to-double idioms.
///
/// # Arguments
/// - `this`: address of the word holding the transform record base.
/// - `stage`: 1, 2 or 3 selects `+0x0A`, `+0x20` or `+0x2F`.
/// - `out`: destination for twelve floats.
///
/// # Returns
/// 1 when the stage is present, 0 for a bad stage selector or a zero present
/// byte.
///
/// # Safety
/// `*(this)` must address a readable record; `out` must hold twelve floats.
///
/// # Confidence
/// medium - the bit packing is literal; the fixed-point scale is inferred
/// from the encoder at `0x8023DAA0`.
pub unsafe fn fn_8023d5f0(this: usize, stage: u32, out: usize) -> u32 {
    let record = platform::read_u32(this) as usize;
    let slot = packed_matrix_slot(record, stage);
    if slot == 0 || platform::read_u8(slot) == 0 {
        return 0;
    }

    let (fields, exponent) = packed_matrix_fields(slot);
    let scale = PACKED_MTX_DECODE_SCALE * packed_matrix_exponent_scale(exponent);

    platform::write_f32(out, scale * fields[0] as f32);
    platform::write_f32(out + 4, scale * fields[2] as f32);
    platform::write_f32(out + 8, scale * fields[4] as f32);
    platform::write_f32(out + 0x0C, 0.0);
    platform::write_f32(out + 0x10, scale * fields[1] as f32);
    platform::write_f32(out + 0x14, scale * fields[3] as f32);
    platform::write_f32(out + 0x18, scale * fields[5] as f32);
    platform::write_f32(out + 0x1C, 0.0);
    platform::write_f32(out + 0x20, 0.0);
    platform::write_f32(out + 0x24, 0.0);
    platform::write_f32(out + 0x28, 1.0);
    platform::write_f32(out + 0x2C, 0.0);
    1
}

/// Reads the packed matrix of one stage and its exponent byte without applying
/// the exponent.
///
/// # Original
/// - Ghidra: `FUN_8023d8b0` at `0x8023D8B0` (488 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023d8b0_FUN_8023d8b0.c`
/// - `// observed:` same record as `0x8023D5F0`; the exponent is returned
///   separately instead of scaling the six fields.
///
/// # Arguments
/// - `this`: address of the word holding the transform record base.
/// - `stage`: 1, 2 or 3.
/// - `out`: optional destination for twelve floats.
/// - `out_exponent`: optional byte output for the biased exponent.
///
/// # Returns
/// 1 when the stage is present, 0 otherwise.
///
/// # Safety
/// `*(this)` must address a readable record; each non-null output must be
/// writable.
///
/// # Confidence
/// medium - the bit packing is literal; the fixed-point scale is inferred.
pub unsafe fn fn_8023d8b0(this: usize, stage: u32, out: usize, out_exponent: usize) -> u32 {
    let record = platform::read_u32(this) as usize;
    let slot = packed_matrix_slot(record, stage);
    if slot == 0 || platform::read_u8(slot) == 0 {
        return 0;
    }

    let (fields, exponent) = packed_matrix_fields(slot);
    if out_exponent != 0 {
        platform::write_u8(out_exponent, exponent as u8);
    }
    if out != 0 {
        let scale = PACKED_MTX_DECODE_SCALE;
        platform::write_f32(out, scale * fields[0] as f32);
        platform::write_f32(out + 4, scale * fields[2] as f32);
        platform::write_f32(out + 8, scale * fields[4] as f32);
        platform::write_f32(out + 0x0C, 0.0);
        platform::write_f32(out + 0x10, scale * fields[1] as f32);
        platform::write_f32(out + 0x14, scale * fields[3] as f32);
        platform::write_f32(out + 0x18, scale * fields[5] as f32);
        platform::write_f32(out + 0x1C, 0.0);
        platform::write_f32(out + 0x20, 0.0);
        platform::write_f32(out + 0x24, 0.0);
        platform::write_f32(out + 0x28, 1.0);
        platform::write_f32(out + 0x2C, 0.0);
    }
    1
}

/// Packs a 3x4 matrix into three 14-byte stage records as 11-bit fields.
///
/// # Original
/// - Ghidra: `FUN_8023daa0` at `0x8023DAA0` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023daa0_FUN_8023daa0.c`
/// - `// observed:` three `0x80239980` command writes; each word carries two
///   rounded `1024 * value` fields and two exponent bits.
///
/// # Arguments
/// - `this`: address of the word holding the transform record base.
/// - `stage`: 1, 2 or 3 selects the same offsets as [`fn_8023d5f0`].
/// - `matrix`: twelve floats, column-major 3x4.
/// - `exponent`: six-bit exponent, biased by `0x11` before packing.
///
/// # Safety
/// `*(this)` must address a writable record; `matrix` must hold twelve floats.
///
/// # Confidence
/// medium - the packing is literal; the exponent bias is shared with the
/// decoder.
pub unsafe fn fn_8023daa0(this: usize, stage: u32, matrix: usize, exponent: u32) {
    let record = platform::read_u32(this) as usize;
    let (slot, tag_base) = match stage {
        1 => (record + PACKED_MTX_SLOT_1, 0u32),
        2 => (record + PACKED_MTX_SLOT_2, 3u32),
        3 => (record + PACKED_MTX_SLOT_3, 6u32),
        _ => return,
    };
    let exponent = exponent.wrapping_add(PACKED_MTX_EXPONENT_BIAS as u32);

    let encode = |offset: usize| -> u32 {
        let value = platform::read_f32(matrix + offset);
        ((PACKED_MTX_ENCODE_SCALE * value) as i32 as u32) & 0x7FF
    };

    let word_0 = ((tag_base + 6) << 24)
        | (encode(0x10) << 0x0B)
        | ((exponent.wrapping_mul(0x40_0000)) & 0xC0_0000)
        | encode(0);
    platform::call2(GX_BP_WRITE, slot as u32, word_0);

    let word_1 = ((tag_base + 7) << 24)
        | ((exponent.wrapping_mul(0x10_0000)) & 0xC0_0000)
        | encode(4)
        | (encode(0x14) << 0x0B);
    platform::call2(GX_BP_WRITE, (slot + 5) as u32, word_1);

    let word_2 = ((tag_base + 8) << 24)
        | ((exponent.wrapping_mul(0x4_0000)) & 0xC0_0000)
        | encode(8)
        | (encode(0x18) << 0x0B);
    platform::call2(GX_BP_WRITE, (slot + 10) as u32, word_2);
}

/// Calls a function with eight integer arguments.
///
/// [`crate::platform`] stops at [`crate::platform::call7`]; the texture
/// initialisers need eight, so this local helper mirrors the same transmute.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call8(
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
    let func = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32, u32) -> u32);
    func(a, b, c, d, e, f, g, h)
}

/// Calls `GXInitTexObjLOD`-shaped code: three floats then six integers.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_lod(
    addr: usize,
    min_lod: f32,
    max_lod: f32,
    lod_bias: f32,
    object: u32,
    min_filter: u32,
    mag_filter: u32,
    bias_clamp: u32,
    edge_lod: u32,
    max_aniso: u32,
) -> u32 {
    let func = crate::platform::guest_fn!(addr, fn(f32, f32, f32, u32, u32, u32, u32, u32, u32) -> u32);
    func(
        min_lod,
        max_lod,
        lod_bias,
        object,
        min_filter,
        mag_filter,
        bias_clamp,
        edge_lod,
        max_aniso,
    )
}

/// Calls a function with nine integer arguments (the CI texture initialiser).
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call9(
    addr: usize,
    a: u32,
    b: u32,
    c: u32,
    d: u32,
    e: u32,
    f: u32,
    g: u32,
    h: u32,
    i: u32,
) -> u32 {
    let func = crate::platform::guest_fn!(addr, fn(u32, u32, u32, u32, u32, u32, u32, u32, u32) -> u32);
    func(a, b, c, d, e, f, g, h, i)
}

/// Calls an `f64, u32` codec such as `0x8023F140` / `0x8023F4E0`.
///
/// # Safety
/// `addr` must point at a function with this ABI.
#[inline]
unsafe fn call_f64_u32(addr: usize, value: f64, arg: u32) -> u32 {
    let func = crate::platform::guest_fn!(addr, fn(f64, u32) -> u32);
    func(value, arg)
}

/// Binds one material's texture and palette into the object caches.
///
/// The body resolves the TEX0 record through [`brres_get_texture_record`] and
/// the PLT0 record through [`brres_get_palette_record`], fills the 0x20-byte
/// texture slot and 0x0C-byte palette slot selected by the context's `+0x10`
/// index, and publishes the two present bitsets.
///
/// # Original
/// - Ghidra: `FUN_8023e390` at `0x8023E390` (700 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023e390_FUN_8023e390.c`
/// - `// observed:` the two cache writes use `GXInitTexObj`/`GXInitTexObjCI`
///   and `GXInitTexObjLOD` for TEX0, and `GXInitTlutObj` for PLT0.
/// - `// inferred:` the context object is the material bind state; its `+0x08`
///   and `+0x0C` words cache the resolved records.
///
/// # Arguments
/// - `this`: address of the word holding the BRRES handle (incoming `r3`).
/// - `context`: material bind state (incoming `r4`).
/// - `texture_bits`, `palette_bits`: addresses of the two present-bitset words
///   (incoming `r7`/`r8`; the register-save prologue hides `r3`/`r4` and the
///   two unused words are `r5`/`r6`).
///
/// # Side effects
/// Writes GX texture objects, caches two pointers, sets present bits and
/// restores the saved registers.
///
/// # Safety
/// The BRRES handle, both bitsets and the GX object caches must be readable or
/// writable as described.
///
/// # Returns
/// The success word the original passes to its register-restore epilogue
/// (1 when both resource families resolved).
///
/// # Confidence
/// medium - the GX calls and cache layout are literal; the context field roles
/// are inferred from the two cache strides.
pub unsafe fn fn_8023e390(
    this: usize,
    context: usize,
    texture_bits: usize,
    palette_bits: usize,
) -> u32 {
    let root = platform::read_u32(context) as usize;
    let mut result = 1u32;

    let mut key = crate::platform::GuestLocal::new( if platform::read_u32(root) == 0 {
        0u32
    } else {
        root.wrapping_add(platform::read_u32(root) as usize)
            .wrapping_sub(4) as u32
    });
    let texture = if platform::read_u32(root + 8) != 0 {
        0
    } else {
        brres_get_texture_record(this, (key.as_mut_ptr() as *mut u32) as usize)
    };
    if platform::read_u32(root + 8) == 0 {
        if texture == 0 {
            result = 0;
        } else if platform::read_u32(root + 4) == 0
            || (platform::read_u32(texture as usize + 0x18) & 1) != 0
        {
            let cache = platform::read_u32(texture_bits) as usize;
            platform::write_u32(root + 8, texture);
            let slot = platform::read_u32(root + 0x10) as usize;
            let object = if cache == 0 || slot >= 8 {
                0
            } else {
                cache + slot * 0x20 + 4
            };

            let mut data = crate::platform::GuestLocal::<u32>::new( 0u32);
            let mut width = crate::platform::GuestLocal::<u16>::new( 0u16);
            let mut height = crate::platform::GuestLocal::<u16>::new( 0u16);
            let mut format = crate::platform::GuestLocal::<u32>::new( 0u32);
            let mut scale_x = crate::platform::GuestLocal::<f32>::new( 0.0f32);
            let mut scale_y = crate::platform::GuestLocal::<f32>::new( 0.0f32);
            let mut flag = crate::platform::GuestLocal::<u8>::new( 0u8);
            let mut record = crate::platform::GuestLocal::new( texture);
            let read_ok = if platform::read_u32(root + 4) == 0 {
                res_texture_descriptor(
                    (record.as_mut_ptr() as *mut u32) as usize,
                    (data.as_mut_ptr() as *mut u32) as usize,
                    (width.as_mut_ptr() as *mut u16) as usize,
                    (height.as_mut_ptr() as *mut u16) as usize,
                    (format.as_mut_ptr() as *mut u32) as usize,
                    (scale_x.as_mut_ptr() as *mut f32) as usize,
                    (scale_y.as_mut_ptr() as *mut f32) as usize,
                    (flag.as_mut_ptr() as *mut u8) as usize,
                )
            } else {
                res_texture_descriptor_flag_set(
                    (record.as_mut_ptr() as *mut u32) as usize,
                    (data.as_mut_ptr() as *mut u32) as usize,
                    (width.as_mut_ptr() as *mut u16) as usize,
                    (height.as_mut_ptr() as *mut u16) as usize,
                    (format.as_mut_ptr() as *mut u32) as usize,
                    (scale_x.as_mut_ptr() as *mut f32) as usize,
                    (scale_y.as_mut_ptr() as *mut f32) as usize,
                    (flag.as_mut_ptr() as *mut u8) as usize,
                )
            };
            if read_ok == 0 {
                result = 0;
            } else {
                if platform::read_u32(root + 4) == 0 {
                    call8(
                        GX_INIT_TEXOBJ,
                        object as u32,
                        *data,
                        *width as u32,
                        *height as u32,
                        *format,
                        platform::read_u32(root + 0x18),
                        platform::read_u32(root + 0x1C),
                        *flag as u32,
                    );
                } else {
                    call9(
                        GX_INIT_TEXOBJ_CI,
                        object as u32,
                        *data,
                        *width as u32,
                        *height as u32,
                        *format,
                        platform::read_u32(root + 0x18),
                        platform::read_u32(root + 0x1C),
                        *flag as u32,
                        platform::read_u32(root + 0x14),
                    );
                }
                call_lod(
                    GX_INIT_TEXOBJ_LOD,
                    *scale_x,
                    *scale_y,
                    platform::read_f32(root + 0x28),
                    object as u32,
                    platform::read_u32(root + 0x20),
                    platform::read_u32(root + 0x24),
                    platform::read_u8(root + 0x30) as u32,
                    platform::read_u8(root + 0x31) as u32,
                    platform::read_u32(root + 0x2C),
                );
                if cache != 0 && slot < 8 {
                    let bits = platform::read_u32(cache);
                    platform::write_u32(cache, bits | (1 << slot));
                }
            }
        } else {
            result = 0;
        }
    }

    if platform::read_u32(root + 0x0C) == 0 {
        let tail = if platform::read_u32(root + 4) == 0 {
            0
        } else {
            root.wrapping_add(platform::read_u32(root + 4) as usize)
                .wrapping_sub(4)
        };
        if tail != 0 {
            let mut key = crate::platform::GuestLocal::new( tail as u32);
            let palette = brres_get_palette_record(this, (key.as_mut_ptr() as *mut u32) as usize);
            if palette == 0 {
                result = 0;
            } else {
                let cache = platform::read_u32(palette_bits) as usize;
                platform::write_u32(root + 0x0C, palette);
                let slot = platform::read_u32(root + 0x10) as usize;
                let object = if cache == 0 || slot >= 8 {
                    0
                } else {
                    cache + slot * 0x0C + 4
                };
                let record = palette as usize;
                let data = section_or_zero(record, 0x10);
                platform::call4(
                    GX_INIT_TLUT,
                    object as u32,
                    data as u32,
                    platform::read_u32(record + 0x18),
                    platform::read_u16(record + 0x1C) as u32,
                );
                if cache != 0 && slot < 8 {
                    let bits = platform::read_u32(cache);
                    platform::write_u32(cache, bits | (1 << slot));
                }
            }
        }
    }

    result
}

/// Binds the eleven animation tracks of one MDL0 sub-entry.
///
/// The body walks the model's `+0x10` dictionary to the selected entry, then
/// for each of up to eleven present tracks stores a `0x8023F140` colour key
/// (or the raw word when bit 1 is set) and the track block pointer.
///
/// # Original
/// - Ghidra: `FUN_8023f370` at `0x8023F370` (364 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f370_FUN_8023f370.c`
/// - `// observed:` the loop shifts the presence mask by two and walks two
///   words per track; eleven iterations maximum.
/// - `// inferred:` the clamp bounds are 0.0 and the model's `+0x20` frame
///   count.
///
/// # Arguments
/// - `out`: address of the 0x34-byte output block (incoming `r3`).
/// - `context`: model context word (incoming `r4`).
/// - `index`: dictionary entry index (incoming `r5`).
/// - `time`: animation time in frames (incoming `f1`).
///
/// # Side effects
/// Writes a present bitset, eleven value words and eleven block pointers.
///
/// # Safety
/// `*(context)` must address a readable model; `out` must hold 0x34 bytes; the
/// `0x8023F140` keys must be readable.
///
/// # Confidence
/// medium - the loop and masks are literal; the track roles are inferred.
pub unsafe fn fn_8023f370(out: usize, context: usize, index: u32, time: f64) {
    let model = platform::read_u32(context) as usize;
    let mut entry = section_or_zero(model, 0x10);
    if entry != 0 {
        entry = section_or_zero(entry, (index as usize + 1) * 0x10 + 0x14);
    }

    let mask = platform::read_u32(entry + 4);
    let frame_limit = platform::read_u16(model + 0x20) as f64;
    let mut clamped = 0.0f64;
    if clamped < time {
        clamped = time;
        if frame_limit <= time {
            clamped = frame_limit;
        }
    }

    platform::write_u32(out, 0);
    if mask != 0 {
        let mut remaining = mask;
        let mut index = 0usize;
        let mut block = entry + 8;
        let mut dst = out;
        while index < 11 {
            if (remaining & 1) != 0 {
                let bits = platform::read_u32(out);
                platform::write_u32(out, bits | (1 << index));
                platform::write_u32(dst + 0x30, platform::read_u32(block));
                let value = if (remaining & 2) == 0 {
                    let offset = platform::read_u32(block + 4) as usize;
                    let key = block + offset + 4;
                    call_f64_u32(ANM_KEY_COLOR, clamped, key as u32)
                } else {
                    platform::read_u32(block + 4)
                };
                platform::write_u32(dst + 4, value);
                block += 8;
            }
            index += 1;
            remaining >>= 2;
            dst += 4;
        }
    }
}

/// Resolves the eight texture and eight palette handles of one animation
/// entry.
///
/// # Original
/// - Ghidra: `FUN_8023f5a0` at `0x8023F5A0` (384 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f5a0_FUN_8023f5a0.c`
/// - `// observed:` the 32-bit mask at the entry's `+0x04` is consumed four
///   bits per slot; bits 0/1 select sampling, bits 2/3 select texture/palette.
/// - `// inferred:` the two `+0x1C`/`+0x20` tables are the per-entry texture
///   and palette handle tables.
///
/// # Arguments
/// - `out`: address of the 0x44-byte output block (incoming `r3`).
/// - `context`: model context word (incoming `r4`).
/// - `index`: dictionary entry index (incoming `r5`).
/// - `time`: animation time in frames (incoming `f1`).
///
/// # Side effects
/// Writes two present bytes at `out + 0`/`out + 1`, eight texture handles at
/// `out + 4` and eight palette handles at `out + 0x24`.
///
/// # Safety
/// `*(context)` must address a readable model; `out` must hold 0x44 bytes.
///
/// # Confidence
/// medium - the masks and strides are literal; the table roles are inferred.
pub unsafe fn fn_8023f5a0(out: usize, context: usize, index: u32, time: f64) {
    let model = platform::read_u32(context) as usize;
    let mut entry = section_or_zero(model, 0x10);
    if entry != 0 {
        entry = section_or_zero(entry, (index as usize + 1) * 0x10 + 0x14);
    }

    let mask = platform::read_u32(entry + 4);
    let mut block = entry + 8;
    let texture_table = section_or_zero(model, 0x1C);
    let palette_table = section_or_zero(model, 0x20);

    platform::write_u8(out, 0);
    platform::write_u8(out + 1, 0);

    let mut remaining = mask;
    let mut slot = 0usize;
    let mut dst = out;
    while slot < 8 {
        if (remaining & 1) != 0 {
            let (texture_index, palette_index) = if (remaining & 2) == 0 {
                let offset = platform::read_u32(block) as usize;
                let key = entry + offset;
                let sampled =
                    call_f64_u32(ANM_KEY_SAMPLE, time, key as u32) as usize;
                (
                    platform::read_u16(sampled + 4) as u32,
                    platform::read_u16(sampled + 6) as u32,
                )
            } else {
                (
                    platform::read_u16(block) as u32,
                    platform::read_u16(block + 2) as u32,
                )
            };

            let bit = 1u8 << slot;
            if (remaining & 4) != 0 {
                let handle = platform::read_u32(texture_table + texture_index as usize * 4);
                if handle != 0 {
                    platform::write_u32(dst + 4, handle);
                    platform::write_u8(out, platform::read_u8(out) | bit);
                }
            }
            if (remaining & 8) != 0 {
                let handle = platform::read_u32(palette_table + palette_index as usize * 4);
                if handle != 0 {
                    platform::write_u32(dst + 0x24, handle);
                    platform::write_u8(out + 1, platform::read_u8(out + 1) | bit);
                }
            }
            block += 4;
        }
        slot += 1;
        remaining >>= 4;
        dst += 4;
    }
}

// ===========================================================================
// Engine debug-console and script-stream band, 0x802307E0..0x80237A00
//
// The `0x80230000` bucket is not only BRRES. Between the NW4R G3D math library
// and the BRRES material band it also holds the engine's on-screen debug
// console (`console.cpp`) and the small script-stream queue that console reads
// its command script through. Wave 90 owns this bucket, so those routines are
// reconstructed here instead of staying address-only callees. They share no
// state with the BRRES loader apart from the [`crate::platform`] helpers.
//
// All of the small-data addresses below are `r13`/`r2` offsets resolved against
// [`SDA_BASE`] and [`SDA2_BASE`]; they are absolute virtual addresses, not
// per-thread pointers.
// ===========================================================================

/// Console framebuffer block; the same record `runtime::bt` documents.
///
/// `// observed:` `802331a0_FUN_802331a0.c` writes all of it:
/// `+0x00` pixel base, `+0x04` byte size, `+0x08` `u16` width, `+0x0A` `u16`
/// height, `+0x0C` `u16` row stride in pixels, `+0x10` four palette bytes and
/// `+0x14`..`+0x20` seven `u16` colour words.
const CONSOLE_CONFIG: usize = 0x803D_A120;
/// Console block: pixel base pointer.
const CONSOLE_PIXELS: usize = CONSOLE_CONFIG;
/// Console block: total pixel-buffer size in bytes.
const CONSOLE_PIXEL_BYTES: usize = CONSOLE_CONFIG + 0x04;
/// Console block: `u16` framebuffer width.
const CONSOLE_WIDTH: usize = CONSOLE_CONFIG + 0x08;
/// Console block: `u16` framebuffer height.
const CONSOLE_HEIGHT: usize = CONSOLE_CONFIG + 0x0A;
/// Console block: `u16` row stride (in 16-bit pixels).
const CONSOLE_STRIDE: usize = CONSOLE_CONFIG + 0x0C;
/// Console block: four palette bytes (`0xFF` each by default).
const CONSOLE_PALETTE_BYTES: usize = CONSOLE_CONFIG + 0x10;
/// Console block: base pixel word (`0xEB00` by default).
const CONSOLE_PIXEL_BASE: usize = CONSOLE_CONFIG + 0x14;
/// Console block: `u16` colour components added to [`CONSOLE_PIXEL_BASE`].
///
/// `// unknown:` the seven words at `+0x16`..`+0x20` are the foreground and
/// background components of the 16-bit console pixel format; only their
/// default values (`0x7F`, `0x3F`, `0x1F`, `0x7F`, `0x3F`, `0x1F`) and the
/// way `0x802337B0` adds them are observed.
const CONSOLE_COMPONENTS: usize = CONSOLE_CONFIG + 0x16;

/// `r13 - 0x4A18` - "console defaults installed" flag.
const CONSOLE_DEFAULTS_READY: usize = SDA_BASE - 0x4A18;
/// `r13 - 0x4A10` - head of the registered-console list; `+0x28` is `next`.
const CONSOLE_LIST: usize = SDA_BASE - 0x4A10;
/// `r13 - 0x5CE0` - text staging buffer handed to the console painters.
const CONSOLE_LINE_BUFFER: usize = SDA_BASE - 0x5CE0;
/// `r13 - 0x4A08` - second argument of the console mutex setup.
const CONSOLE_MUTEX_ARG: usize = SDA_BASE - 0x4A08;

/// `0x803DA148` - mutex guarding the console list and the script reader.
const CONSOLE_MUTEX: usize = 0x803D_A148;
/// `0x803DA560` - 0x360-byte console/frame state block.
const CONSOLE_STATE: usize = 0x803D_A560;
/// `0x803DA878` - retrace/console queue object.
const CONSOLE_QUEUE: usize = 0x803D_A878;
/// `0x803DA898` - frame counter published by the console loop.
const CONSOLE_FRAME_COUNTER: usize = 0x803D_A898;
/// `0x803DA8A0` - active console target (`0` = none).
const CONSOLE_TARGET: usize = 0x803D_A8A0;
/// `0x803DA8A4` - screen record the console is painting.
const CONSOLE_SCREEN: usize = 0x803D_A8A4;
/// `0x803DA8A8` - console update callback (`0` = none).
const CONSOLE_UPDATE: usize = 0x803D_A8A8;
/// `0x803DA8B0` - saved OS flags of the frame path.
const CONSOLE_OS_FLAGS: usize = 0x803D_A8B0;
/// `0x803DA8B4` - screen record word saved by `0x80234720`.
const CONSOLE_SCREEN_SAVED: usize = 0x803D_A8B4;
/// `0x803DA8B8` - frame mode (`0x21` by default).
const CONSOLE_FRAME_MODE: usize = 0x803D_A8B8;
/// `0x803DE8C0` - scratch buffer used as the console render buffer and as the
/// script stream window (0x4000 bytes).
const CONSOLE_SCRATCH: usize = 0x803D_E8C0;
/// `0x803DEAC0` - DVD file-info block for the script stream.
const SCRIPT_FILE_INFO: usize = 0x803D_EAC0;
/// `0x80429508` - frame-path flag cleared by `0x80234630`.
const FRAME_PATH_FLAG: usize = SDA_BASE - 0x6178;

/// `r13 - 0x49FC` - head of the script-path queue; `+0x0C` is `next`.
const SCRIPT_QUEUE_HEAD: usize = SDA_BASE - 0x49FC;
/// `r13 - 0x49F8` - current "read one script byte" callback.
const SCRIPT_READ_CHAR: usize = SDA_BASE - 0x49F8;
/// `r13 - 0x49F0` - heap that owns the console script records (queue A).
const SCRIPT_HEAP_A: usize = SDA_BASE - 0x49F0;
/// `r13 - 0x49EC` - first record allocated on [`SCRIPT_HEAP_A`].
const SCRIPT_RECORD_A: usize = SDA_BASE - 0x49EC;
/// `r13 - 0x49E8` - second script heap (queue B).
const SCRIPT_HEAP_B: usize = SDA_BASE - 0x49E8;
/// `r13 - 0x49E4` - first record allocated on [`SCRIPT_HEAP_B`].
const SCRIPT_RECORD_B: usize = SDA_BASE - 0x49E4;
/// `r13 - 0x5CA0` - start of the currently buffered script window.
const SCRIPT_WINDOW_BASE: usize = SDA_BASE - 0x5CA0;
/// `r13 - 0x4A00` - script length in bytes.
const SCRIPT_LENGTH: usize = SDA_BASE - 0x4A00;

/// `r2 - 0x4C18` - small-data word copied into script record `+0x14`.
///
/// `// unknown:` the value is never given a role; `0x80236AD0` and its sibling
/// `0x80237860` copy it verbatim, so it is modelled as a global word.
const SCRIPT_RECORD_WORD: usize = SDA2_BASE - 0x4C18;

/// `0x802F71E8` - 128-byte signed glyph-index table (`-1` = no glyph,
/// `-3` = tab).
const FONT_INDEX_TABLE: usize = 0x802F_71E8;
/// `0x8034CF40` - source file name `console.cpp` for [`OS_PANIC`].
const CONSOLE_SOURCE_FILE: usize = 0x8034_CF40;
/// `0x8034CF50` - `illegal console handle` assert message.
const CONSOLE_BAD_HANDLE: usize = 0x8034_CF50;
/// Line in `console.cpp` reported by that assert (`0x182`).
const CONSOLE_REMOVE_LINE: u32 = 0x182;

/// `0x801B1140` - `OSDisableInterrupts`; returns the saved MSR word.
const INTERRUPT_DISABLE: usize = 0x801B_1140;
/// `0x801B1160` - interrupt-enable helper; returns the saved MSR word.
const INTERRUPT_ENABLE: usize = 0x801B_1160;
/// `0x801B1180` - `OSRestoreInterrupts(token)`.
const INTERRUPT_RESTORE: usize = 0x801B_1180;
/// `0x801B2E60` - lock the mutex at its argument.
const MUTEX_LOCK: usize = 0x801B_2E60;
/// `0x801B2F40` - unlock the mutex at its argument.
const MUTEX_UNLOCK: usize = 0x801B_2F40;
/// `0x801B2390` - initialise a mutex/queue object.
const MUTEX_INIT: usize = 0x801B_2390;
/// `0x801B23F0` - swap a mutex/queue object's state block.
const MUTEX_SWAP: usize = 0x801B_23F0;
/// `0x801B4B30` - true when the OS is running threads.
const THREADS_RUNNING: usize = 0x801B_4B30;
/// `0x801B5130` - install the console render buffer and its draw callback.
const CONSOLE_BUFFER_INIT: usize = 0x801B_5130;
/// `0x801B5860` - start the console render buffer.
const CONSOLE_BUFFER_BEGIN: usize = 0x801B_5860;
/// `0x801AD610` - install a retrace (VI) callback.
const RETRACE_CALLBACK: usize = 0x801A_D610;
/// `0x801AD3B0` - detach the retrace callback of a screen record.
const RETRACE_DETACH: usize = 0x801A_D3B0;
/// `0x801B2670` - clear one of the four EFB copy filters.
const EFB_COPY_CLEAR: usize = 0x801B_2670;
/// `0x801BC3E0` - GX FIFO helper taking `0`.
const GX_FIFO_A: usize = 0x801B_C3E0;
/// `0x801BC430` - second GX FIFO helper taking `0`.
const GX_FIFO_B: usize = 0x801B_C430;
/// `0x801B4BB0` - "frame still pending" poll.
const FRAME_POLL: usize = 0x801B_4BB0;
/// `0x801B50F0` - end the console render buffer frame.
const CONSOLE_BUFFER_END: usize = 0x801B_50F0;
/// `0x801B4B70` - release the frame-path lock.
const FRAME_RELEASE: usize = 0x801B_4B70;
/// `0x801A9B20` - read the OS/console flag word.
const OS_FLAG_GET: usize = 0x801A_9B20;
/// `0x801A9B30` - write the OS/console flag word.
const OS_FLAG_SET: usize = 0x801A_9B30;

/// `0x801D4380` - create an `EXPH`-tagged heap: `(size, size2, flags)`.
const HEAP_CREATE: usize = 0x801D_4380;
/// `0x801D4460` - allocate `(heap, size, align)`; negative align means
/// "allocate from the top".
const HEAP_ALLOC: usize = 0x801D_4460;
/// `0x801D49A0` - store the `u16` heap lock word at `+0x4C`.
const HEAP_SET_LOCK: usize = 0x801D_49A0;

/// `0x801CAA30` - start a priority-2 DVD read into the scratch buffer.
const DVD_READ: usize = 0x801C_AA30;
/// `0x801CEE90` - poll an async DVD command until it completes.
const DVD_POLL: usize = 0x801C_EE90;
/// `0x801CA680` - resolve a DVD path to an entry number.
const DVD_PATH_ENTRY: usize = 0x801C_A680;

/// `0x802E1C10` - `strchr`.
const STRCHR: usize = 0x802E_1C10;

/// `0x80233290` - fill one console framebuffer rectangle.
const CONSOLE_FILL_RECT: usize = 0x8023_3290;
/// `0x80234720` - per-retrace console frame entry point.
const FRAME_DRAW: usize = 0x8023_4720;

/// `0x802337B0` - blit one glyph into the console framebuffer.
///
/// `// observed:` `0x802336D0` calls it once per printable glyph with the
/// current pen position and the character code.
const CONSOLE_GLYPH_BLIT: usize = 0x8023_37B0;
/// `0x80233480` - print one console row through the GX text path.
const CONSOLE_PRINT_ROW: usize = 0x8023_3480;
/// `0x8022EAF0` - paint one console row through an attached renderer.
const CONSOLE_PAINT_ROW: usize = 0x8022_EAF0;
/// `0x80235DC0` - dispatch one script-path queue node.
const SCRIPT_DISPATCH_NODE: usize = 0x8023_5DC0;
/// `0x802348F0` - main console frame loop (still untranslated).
const CONSOLE_FRAME_LOOP: usize = 0x8023_48F0;
/// `0x80234860` - console frame draw callback installed by `0x80234630`.
const CONSOLE_DRAW_CALLBACK: usize = 0x8023_4860;

/// `0x8022C980` - free a resource block when its count is positive.
const RES_OBJECT_DISPOSE: usize = 0x8022_C980;
/// `0x802B9280` - free one archive-heap block.
///
/// This is `resource::archive`'s `arc_heap_free_object`; it stays an
/// address call so this file does not depend on the module's visibility.
const ARC_HEAP_FREE_OBJECT: usize = 0x802B_9280;
/// `0x802323E0` - console-script executor called by the `0x80231B70` thunk.
const SCRIPT_EXECUTE: usize = 0x8023_23E0;

/// Calls a function with no arguments that returns a word.
///
/// [`crate::platform::call0`] discards the result. Several OS helpers used by
/// the console path (`0x801B4B30`, `0x801A9B20`, `0x801B1160`, `0x801B4BB0`,
/// `0x801B1140`) return the state word the caller needs, so this local helper
/// mirrors the same transmute with a return type.
///
/// # Safety
/// `addr` must point at a no-argument function returning a word.
#[inline]
unsafe fn call0_word(addr: usize) -> u32 {
    let func = crate::platform::guest_fn!(addr, fn() -> u32);
    func()
}

/// Releases one resource object and frees it when the count is positive.
///
/// # Original
/// - Ghidra: `FUN_802307e0` at `0x802307E0` (88 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802307e0_FUN_802307e0.c`
/// - `// observed:` the `if` condition contains the comma expression
///   `(FUN_8022c980(param_1, 0), 0 < param_2)`, so the dispose call always runs
///   for a non-null object and the free is gated on `count > 0`.
///
/// # Arguments
/// - `object`: the resource object; `0` is accepted and returned unchanged.
/// - `count`: remaining references; only a positive value frees the block.
///
/// # Returns
/// `object`, including the null case.
///
/// # Side effects
/// Calls `0x8022C980` (dispose) and, when `count > 0`, `0x802B9280`
/// ([`ARC_HEAP_FREE_OBJECT`]).
///
/// # Safety
/// `object` must be null or a live object of the archive heap.
///
/// # Confidence
/// high - the guard, both calls and the returned value are literal; the
/// "reference count" role of `count` is `// inferred:` from the sibling
/// `0x8022C980`, which has the identical shape.
pub unsafe fn fn_802307e0(object: usize, count: i32) -> u32 {
    if object != 0 {
        platform::call2(RES_OBJECT_DISPOSE, object as u32, 0);
        if count > 0 {
            platform::call1(ARC_HEAP_FREE_OBJECT, object as u32);
        }
    }
    object as u32
}

/// Tail call into the console-script executor.
///
/// # Original
/// - Ghidra: `FUN_80231b70` at `0x80231B70` (8 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80231b70_FUN_80231b70.c`
/// - `// observed:` the whole body is `FUN_802323e0(); return;`.
/// - `// unknown:` Ghidra shows no arguments; the callee reads whatever the
///   caller left in `r3..`, and this thunk does not touch them.
///
/// # Side effects
/// Everything `0x802323E0` does.
///
/// # Safety
/// The caller must have set up the argument registers the executor expects.
///
/// # Confidence
/// medium - the tail call is exact, but the argument list is invisible.
pub unsafe fn fn_80231b70() {
    platform::call0(SCRIPT_EXECUTE);
}

/// Installs the default console framebuffer geometry once.
///
/// # Original
/// - Ghidra: `FUN_802331a0` at `0x802331A0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802331a0_FUN_802331a0.c`
/// - `// observed:` sixteen stores, gated on the "defaults installed" flag at
///   `r13 - 0x4A18`; the 640x480 geometry matches a 320x240 console surface
///   scaled by two (see [`fn_80233290`]).
///
/// # Side effects
/// Writes the whole [`CONSOLE_CONFIG`] block and sets the ready flag.
///
/// # Safety
/// The console config block must be writable.
///
/// # Confidence
/// high - every value is a literal in the export.
pub unsafe fn fn_802331a0() {
    if platform::read_u32(CONSOLE_DEFAULTS_READY) != 0 {
        return;
    }
    platform::write_u16(CONSOLE_WIDTH, 0x0280);
    platform::write_u32(CONSOLE_PIXELS, 0);
    platform::write_u16(CONSOLE_HEIGHT, 0x01E0);
    platform::write_u16(CONSOLE_STRIDE, 0x0280);
    platform::write_u32(CONSOLE_PIXEL_BYTES, 0x0009_6000);
    platform::write_u8(CONSOLE_PALETTE_BYTES, 0xFF);
    platform::write_u8(CONSOLE_PALETTE_BYTES + 1, 0xFF);
    platform::write_u8(CONSOLE_PALETTE_BYTES + 2, 0xFF);
    platform::write_u8(CONSOLE_PALETTE_BYTES + 3, 0xFF);
    platform::write_u16(CONSOLE_PIXEL_BASE, 0xEB00);
    platform::write_u16(CONSOLE_COMPONENTS, 0x7F);
    platform::write_u16(CONSOLE_COMPONENTS + 2, 0x3F);
    platform::write_u16(CONSOLE_COMPONENTS + 4, 0x1F);
    platform::write_u16(CONSOLE_COMPONENTS + 6, 0x7F);
    platform::write_u16(CONSOLE_COMPONENTS + 8, 0x3F);
    platform::write_u16(CONSOLE_COMPONENTS + 0x0A, 0x1F);
    platform::write_u32(CONSOLE_DEFAULTS_READY, 1);
}

/// Reports whether a console can be drawn right now.
///
/// # Original
/// - Ghidra: `FUN_80233260` at `0x80233260` (40 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233260_FUN_80233260.c`
/// - `// observed:` two guards: the defaults flag and the pixel base.
///
/// # Returns
/// 1 when the console defaults are installed and a framebuffer is attached,
/// 0 otherwise.
///
/// # Safety
/// Reads the console config block only.
///
/// # Confidence
/// high - both loads and the two early returns are literal.
pub unsafe fn fn_80233260() -> u32 {
    if platform::read_u32(CONSOLE_DEFAULTS_READY) == 0 {
        return 0;
    }
    if platform::read_u32(CONSOLE_PIXELS) == 0 {
        return 0;
    }
    1
}

/// Fills an axis-aligned rectangle of the console framebuffer.
///
/// The console surface is 320x240 logical pixels; when the framebuffer is at
/// least 400 pixels wide (or 300 high) both the origin and the extent are
/// doubled, which is why the shipped 640x480 mode draws at 2x.
///
/// # Original
/// - Ghidra: `FUN_80233290` at `0x80233290` (396 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233290_FUN_80233290.c`
/// - `// observed:` the clip test is `max(x, 0)` and `min(x + width, fb_width)`
///   on unsigned words; the row fill writes the constant `0x1080` eight times
///   per unrolled step.
/// - `// unknown:` `0x1080` is the console background pixel word; its
///   component split inside the 16-bit YUV format is not documented here.
///
/// # Arguments
/// - `x`, `y`: top-left corner in logical console pixels (may be negative).
/// - `width`, `height`: extent in logical console pixels.
///
/// # Side effects
/// Writes `width * height` 16-bit pixels into [`CONSOLE_PIXELS`].
///
/// # Safety
/// A framebuffer must be attached at [`CONSOLE_PIXELS`] and large enough for
/// the clipped rectangle.
///
/// # Confidence
/// medium - the clipping, the 2x scale test and the fill value are literal;
/// the pixel encoding is unknown.
pub unsafe fn fn_80233290(x: u32, y: u32, width: i32, height: i32) {
    if platform::read_u32(CONSOLE_PIXELS) == 0 {
        return;
    }

    // `// observed:` `if (399 < uVar5) { param_1 <<= 1; param_3 <<= 1; }`
    let (x, width) = if platform::read_u16(CONSOLE_WIDTH) > 399 {
        (x.wrapping_mul(2), width.wrapping_mul(2))
    } else {
        (x, width)
    };
    let left = if (x as i32) < 0 { 0 } else { x };
    let right = {
        let edge = x.wrapping_add(width as u32);
        if (edge as i32) <= platform::read_u16(CONSOLE_WIDTH) as i32 {
            edge
        } else {
            platform::read_u16(CONSOLE_WIDTH) as u32
        }
    };
    let span = right.wrapping_sub(left);

    let (y, height) = if platform::read_u16(CONSOLE_HEIGHT) > 299 {
        (y.wrapping_mul(2), height.wrapping_mul(2))
    } else {
        (y, height)
    };
    let top = if (y as i32) < 0 { 0 } else { y };
    let bottom = {
        let edge = y.wrapping_add(height as u32);
        if (edge as i32) <= platform::read_u16(CONSOLE_HEIGHT) as i32 {
            edge
        } else {
            platform::read_u16(CONSOLE_HEIGHT) as u32
        }
    };

    let stride = platform::read_u16(CONSOLE_STRIDE) as usize;
    let base = platform::read_u32(CONSOLE_PIXELS) as usize;
    let mut pixel = base + (left as usize) * 2 + stride * (top as usize) * 2;
    let rows = bottom.wrapping_sub(top);
    let mut row = 0u32;
    while row < rows {
        let mut column = 0u32;
        while column < span {
            platform::write_u16(pixel, 0x1080);
            pixel += 2;
            column += 1;
        }
        pixel += (stride - span as usize) * 2;
        row += 1;
    }
}

/// Publishes the console framebuffer with a cache-block store.
///
/// # Original
/// - Ghidra: `FUN_80233460` at `0x80233460` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233460_FUN_80233460.c`
/// - `// observed:` one call, `FUN_801ac4a0(DAT_803da120, DAT_803da124)`.
///
/// # Side effects
/// Flushes the whole pixel buffer; no memory in this module is written.
///
/// # Safety
/// [`CONSOLE_PIXELS`] must be a valid buffer of [`CONSOLE_PIXEL_BYTES`] bytes.
///
/// # Confidence
/// high - the two loads and the call are literal.
pub unsafe fn fn_80233460() {
    platform::call2(
        DC_STORE_SYNC,
        platform::read_u32(CONSOLE_PIXELS),
        platform::read_u32(CONSOLE_PIXEL_BYTES),
    );
}

/// Draws a block of text at a console pixel position.
///
/// Each source line is laid out by [`fn_802336d0`]; the pen then either wraps
/// to the left margin or abandons the rest of the line, depending on `wrap`.
///
/// # Original
/// - Ghidra: `FUN_80233570` at `0x80233570` (340 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233570_FUN_80233570.c`
/// - `// observed:` the pen is the `(x, y)` pair the prologue leaves in `r3`/
///   `r4`, advanced by 10 pixels per line; the tab measure adds four and
///   truncates to a multiple of four.
/// - `// inferred:` `param_4` selects "wrap to the left margin" (nonzero) or
///   "skip to the next newline" (zero); `param_5` clears the line background
///   before drawing.
///
/// # Arguments
/// - `x`, `y`: console pixel position of the first line.
/// - `text`: NUL-terminated text.
/// - `wrap`: nonzero restarts a long line at `x = 0`.
/// - `erase`: nonzero paints the line rectangle before the glyphs.
///
/// # Side effects
/// Calls `0x80233290` (erase), `0x802336D0` (layout + glyph blit).
///
/// # Safety
/// `text` must be a readable C string.
///
/// # Confidence
/// medium - the loop and the pen arithmetic are literal; the two boolean
/// argument roles are inferred from their use.
pub unsafe fn fn_80233570(x: i32, y: i32, text: usize, wrap: u32, erase: u32) {
    let mut pen_x = x;
    let mut pen_y = y;
    let mut src = text;
    let width = platform::read_u16(CONSOLE_WIDTH) as i32;

    while platform::read_u8(src) != 0 {
        if erase != 0 {
            let mut columns = 0u32;
            let mut scan = src;
            loop {
                let c = platform::read_u8(scan);
                scan += 1;
                if c == 0 || c == b'\n' {
                    break;
                }
                if c == b'\t' {
                    columns = columns.wrapping_add(4) & !3;
                } else {
                    columns = columns.wrapping_add(1);
                }
            }
            platform::call4(
                CONSOLE_FILL_RECT,
                (pen_x - 6) as u32,
                (pen_y - 3) as u32,
                columns.wrapping_add(2).wrapping_mul(6),
                0x0D,
            );
        }

        // `// observed:` `((width / (2 - (width < 400))) - x) / 6`
        let divisor = if width < 400 { 2 } else { 1 };
        let limit = (width / divisor - pen_x) / 6;
        src = fn_802336d0(pen_x, pen_y, src, limit) as usize;

        pen_y += 10;
        if platform::read_u8(src) == b'\n' {
            src += 1;
        } else if platform::read_u8(src) != 0 {
            src += 1;
            if wrap == 0 {
                let next = platform::call2(STRCHR, src as u32, 10);
                if next == 0 {
                    break;
                }
                src = next as usize + 1;
            } else {
                pen_x = 0;
            }
        }
    }
}

/// Lays out one run of text and blits each glyph.
///
/// # Original
/// - Ghidra: `FUN_802336d0` at `0x802336D0` (220 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802336d0_FUN_802336d0.c`
/// - `// observed:` the 128-byte table at `0x802F71E8` marks tab (`-3`) and
///   missing glyphs (`-1`); a tab advances to the next four-column stop, any
///   other printable byte advances one six-pixel cell.
/// - `// inferred:` the blit call shows only `(x, y)` because the character
///   code stayed in `r5`; it is re-supplied as the third argument.
///
/// # Arguments
/// - `x`, `y`: pen position in console pixels.
/// - `text`: NUL-terminated text.
/// - `max_columns`: stop after this many layout columns.
///
/// # Returns
/// The address of the byte that stopped the layout (NUL or newline).
///
/// # Side effects
/// Calls `0x802337B0` once per printable glyph.
///
/// # Safety
/// `text` must be readable.
///
/// # Confidence
/// medium - the table lookups and the two advances are literal; the blit's
/// third argument is inferred from the byte just loaded.
pub unsafe fn fn_802336d0(x: i32, y: i32, text: usize, max_columns: i32) -> u32 {
    let mut src = text;
    let mut pen_x = x;
    let mut columns = 0u32;

    loop {
        let c = platform::read_u8(src);
        if c == 0 || c == b'\n' {
            break;
        }
        let entry = platform::read_u8(FONT_INDEX_TABLE + (c & 0x7F) as usize) as i8;
        if entry == -3 {
            let stop = 4 - (columns & 3);
            columns = columns.wrapping_add(stop);
            pen_x += (stop as i32) * 6;
        } else {
            if entry != -1 {
                platform::call3(CONSOLE_GLYPH_BLIT, pen_x as u32, y as u32, c as u32);
            }
            pen_x += 6;
            columns = columns.wrapping_add(1);
        }
        if (columns as i32) >= max_columns {
            break;
        }
        src += 1;
    }

    // `// observed:` `if (param_3[1] == '\n') param_3 = param_3 + 1;`
    if platform::read_u8(src + 1) == b'\n' {
        src += 1;
    }
    src as u32
}

/// Removes a console record from the registered-console list.
///
/// # Original
/// - Ghidra: `FUN_80233c60` at `0x80233C60` (168 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233c60_FUN_80233c60.c`
/// - `// observed:` the panic at `0x801AD4E0` names `console.cpp`, line
///   `0x182`, and the message `illegal console handle`, which is what fixes the
///   role of this routine.
///
/// # Arguments
/// - `record`: console record to unlink; `+0x28` is the `next` field.
///
/// # Side effects
/// Locks and unlocks [`CONSOLE_MUTEX`], rewrites the list links and clears the
/// removed record's `next` word.
///
/// # Safety
/// `record` must be a console record that is (or should be) in the list; the
/// mutex object must be a valid OS mutex.
///
/// # Confidence
/// high - the list walk, the two unlink forms and the assert are literal.
pub unsafe fn fn_80233c60(record: usize) {
    platform::call1(MUTEX_LOCK, CONSOLE_MUTEX as u32);
    let head = platform::read_u32(CONSOLE_LIST);
    if head == record as u32 {
        platform::write_u32(CONSOLE_LIST, platform::read_u32(record + 0x28));
        platform::write_u32(record + 0x28, 0);
    } else {
        let mut node = head;
        loop {
            let previous = node as usize;
            node = platform::read_u32(previous + 0x28);
            if node == 0 {
                platform::call3(
                    OS_PANIC,
                    CONSOLE_SOURCE_FILE as u32,
                    CONSOLE_REMOVE_LINE,
                    CONSOLE_BAD_HANDLE as u32,
                );
                break;
            }
            if node == record as u32 {
                platform::write_u32(previous + 0x28, platform::read_u32(record + 0x28));
                platform::write_u32(record + 0x28, 0);
                break;
            }
        }
    }
    platform::call1(MUTEX_UNLOCK, CONSOLE_MUTEX as u32);
}

/// Destroys a console record and returns its handle.
///
/// # Original
/// - Ghidra: `FUN_80233ec0` at `0x80233EC0` (48 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233ec0_FUN_80233ec0.c`
/// - `// observed:` `FUN_80233c60(); return param_1;` - the unlink routine
///   takes the record in `r3`, which the export does not repeat.
///
/// # Arguments
/// - `record`: console record to remove.
///
/// # Returns
/// `record`, unchanged.
///
/// # Side effects
/// Everything [`fn_80233c60`] does.
///
/// # Safety
/// Same requirements as [`fn_80233c60`].
///
/// # Confidence
/// high - the call and the returned word are literal; the pass-through of
/// `record` is the only inference and matches `0x80233C60`'s signature.
pub unsafe fn fn_80233ec0(record: usize) -> u32 {
    fn_80233c60(record);
    record as u32
}

/// Repaints the rows of one console record that have not been painted yet.
///
/// # Original
/// - Ghidra: `FUN_80233ef0` at `0x80233EF0` (420 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233ef0_FUN_80233ef0.c`
/// - `// observed:` the pending count is `+0x18 - +0x14`; the first row is
///   `(+0x10 + pending) mod +0x06`; the end row is `+0x0C`, plus one when the
///   column cursor `+0x0E` is nonzero; each painted row advances the pen by ten
///   pixels and wraps to row 0 unless flag bit 1 is set.
/// - `// unknown:` the non-null `renderer` branch passes the renderer and the
///   staging buffer to `0x8022EAF0`; the export shows no row pointer there.
///
/// # Arguments
/// - `record`: console record. `+0x00` grid base, `+0x04` `u16` columns,
///   `+0x06` `u16` ring rows, `+0x0A` `u16` flags, `+0x0C` `u16` bottom row,
///   `+0x0E` `u16` column cursor, `+0x10` `u16` top row, `+0x14` `u32` line
///   counter, `+0x18` `u32` target line, `+0x1C`/`+0x1E` `i16` x/y,
///   `+0x20` `u16` visible rows.
/// - `renderer`: nonzero paints through `0x8022EAF0`, zero uses the built-in
///   GX text path `0x80233480`.
///
/// # Side effects
/// Locks/unlocks [`CONSOLE_MUTEX`] and calls the row painter once per row.
///
/// # Safety
/// `record` must point at a readable console record whose grid base is
/// readable for `(+0x04 + 1) * +0x06` bytes.
///
/// # Confidence
/// medium - the ring arithmetic is literal; the renderer branch's arguments
/// are incomplete in the export.
pub unsafe fn fn_80233ef0(record: usize, renderer: usize) {
    if call0_word(THREADS_RUNNING) != 0 {
        platform::call1(MUTEX_LOCK, CONSOLE_MUTEX as u32);
    }

    let mut pending = platform::read_i32(record + 0x18).wrapping_sub(platform::read_i32(record + 0x14));
    if pending < 0 {
        pending = 0;
    } else {
        let mut drawn = (platform::read_u16(record + 0x0C) as i32)
            .wrapping_sub(platform::read_u16(record + 0x10) as i32);
        if drawn < 0 {
            drawn = drawn.wrapping_add(platform::read_u16(record + 0x06) as i32);
        }
        let mut drawn = (drawn as u32) & 0xFFFF;
        if platform::read_u16(record + 0x0E) != 0 {
            drawn = drawn.wrapping_add(1) & 0xFFFF;
        }
        if (drawn as i32) < pending {
            platform::call1(MUTEX_UNLOCK, CONSOLE_MUTEX as u32);
            return;
        }
    }

    let ring = platform::read_u16(record + 0x06) as u32;
    let mut row = (platform::read_u16(record + 0x10) as u32)
        .wrapping_add(pending as u32)
        & 0xFFFF;
    if ring <= row {
        row = row.wrapping_sub(ring) & 0xFFFF;
    }
    let cursor = platform::read_u16(record + 0x0E);
    let mut end = if cursor != 0 {
        (platform::read_u16(record + 0x0C) as u32).wrapping_add(1) & 0xFFFF
    } else {
        platform::read_u16(record + 0x0C) as u32
    };
    if end == ring {
        end = 0;
    }

    let columns = platform::read_u16(record + 0x04) as usize;
    let base = platform::read_u32(record) as usize;
    let x = platform::read_u16(record + 0x1C) as i16 as i32 as u32;
    let y = platform::read_u16(record + 0x1E) as i16 as i32 as u32;
    let visible = platform::read_u16(record + 0x20) as u32;

    let mut line = 0u32;
    loop {
        if row == end {
            break;
        }
        if renderer == 0 {
            platform::call5(
                CONSOLE_PRINT_ROW,
                x,
                y.wrapping_add(line.wrapping_mul(10)),
                0,
                CONSOLE_LINE_BUFFER as u32,
                base.wrapping_add((columns + 1).wrapping_mul(row as usize)) as u32,
            );
        } else {
            platform::call2(CONSOLE_PAINT_ROW, renderer as u32, CONSOLE_LINE_BUFFER as u32);
        }
        row = row.wrapping_add(1) & 0xFFFF;
        line = line.wrapping_add(1);
        if row == ring {
            if (platform::read_u16(record + 0x0A) & 2) != 0 {
                break;
            }
            row = 0;
        }
        if line >= visible {
            break;
        }
    }

    platform::call1(MUTEX_UNLOCK, CONSOLE_MUTEX as u32);
}

/// Writes text into a console record's scrolling character grid.
///
/// The grid is `+0x06` rows of `(+0x04 + 1)` bytes: one NUL after each row.
/// The writer handles newline, tab stops of four columns, Shift-JIS lead bytes
/// (two bytes per glyph) and both overflow policies selected by `+0x0A`
/// bit 0 (skip to the next newline vs. wrap to the next row).
///
/// # Original
/// - Ghidra: `FUN_80234120` at `0x80234120` (948 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80234120_FUN_80234120.c`
/// - `// observed:` the per-character width helper folds to a sign-extended
///   `c | 0x7E` test, i.e. one byte below `0x80` and two bytes from `0x80` up;
///   advancing a row clears `+0x0E`, bumps `+0x0C`, wraps it at `+0x06` unless
///   flag bit 1 is set, and pushes `+0x10` plus the `+0x14` counter when the
///   new row collides with the oldest one.
///
/// # Arguments
/// - `record`: console record (same layout as [`fn_80233ef0`]).
/// - `text`: NUL-terminated text.
///
/// # Side effects
/// Writes grid bytes, row terminators and the record's cursor fields; disables
/// interrupts for the duration.
///
/// # Safety
/// The record's grid base must be writable for `(+0x04 + 1) * +0x06` bytes and
/// `text` must be readable.
///
/// # Confidence
/// medium - the control flow and every store are literal; the Shift-JIS width
/// rule is reconstructed from Ghidra's folded mask expression.
pub unsafe fn fn_80234120(record: usize, text: usize) {
    let state = call0_word(INTERRUPT_DISABLE);
    let columns = platform::read_u16(record + 0x04) as usize;
    let row_stride = columns + 1;
    let mut src = text;
    let mut pen = grid_row_start(record, row_stride);

    loop {
        if platform::read_u8(src) == 0
            || ((platform::read_u16(record + 0x0A) & 2) != 0
                && platform::read_u16(record + 0x0C) == platform::read_u16(record + 0x06))
        {
            platform::call1(INTERRUPT_RESTORE, state);
            return;
        }

        loop {
            let c = platform::read_u8(src);
            if c == 0 {
                break;
            }
            let mut overflow = false;

            if c == b'\n' {
                src += 1;
                console_end_row(record);
                pen = grid_row_start(record, row_stride);
                break;
            }

            if c == b'\t' {
                src += 1;
                let flags = platform::read_u16(record + 0x0A);
                let mask = (2u16).wrapping_shl(((flags >> 2) & 3) as u32).wrapping_sub(1);
                loop {
                    platform::write_u8(pen, b' ');
                    pen += 1;
                    let cursor = platform::read_u16(record + 0x0E).wrapping_add(1);
                    platform::write_u16(record + 0x0E, cursor);
                    if columns as u16 <= cursor {
                        break;
                    }
                    if (cursor & mask) == 0 {
                        break;
                    }
                }
            } else {
                let mut width = if c >= 0x80 { 2usize } else { 1 };
                let cursor = platform::read_u16(record + 0x0E) as usize;
                if columns < cursor + width {
                    width = 0;
                } else {
                    platform::write_u16(record + 0x0E, (cursor + width) as u16);
                    let mut index = 0usize;
                    while index < width {
                        platform::write_u8(pen + index, platform::read_u8(src + index));
                        index += 1;
                    }
                }
                if width == 0 {
                    overflow = true;
                } else {
                    src += width;
                    pen += width;
                }
            }

            if platform::read_u16(record + 0x04) as u32
                <= platform::read_u16(record + 0x0E) as u32
            {
                overflow = true;
            }

            if overflow {
                if (platform::read_u16(record + 0x0A) & 1) == 0 {
                    if platform::read_u8(src) == b'\n' {
                        src += 1;
                    }
                    console_end_row(record);
                    pen = grid_row_start(record, row_stride);
                } else {
                    while platform::read_u8(src) != b'\n' && platform::read_u8(src) != 0 {
                        src += 1;
                    }
                }
                break;
            }

            if platform::read_u8(src) == 0 {
                platform::write_u8(pen, 0);
            }
        }
    }
}

/// Installs the console frame path and its retrace callbacks.
///
/// # Original
/// - Ghidra: `FUN_80234630` at `0x80234630` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80234630_FUN_80234630.c`
/// - `// observed:` zeroes the 0x360-byte [`CONSOLE_STATE`], clears the five
///   console globals, installs `0x80234860` as the draw callback with the
///   0x4000-byte scratch buffer, initialises [`CONSOLE_QUEUE`], and hooks
///   `0x80234720` on retrace slots 2, 3, 5 and 0xF.
/// - `// unknown:` `0x801B5130`, `0x801B5860` and `0x801B2390` are OS-side
///   helpers whose names are not exported; only their argument shapes are
///   observed here.
///
/// # Side effects
/// Writes the console globals and registers four retrace callbacks.
///
/// # Safety
/// All of the console state blocks must be writable and the OS callback table
/// must accept the four slots.
///
/// # Confidence
/// medium - every store and call is literal; the helper roles are inferred.
pub unsafe fn fn_80234630() {
    platform::call3(MEMSET, CONSOLE_STATE as u32, 0, 0x360);
    platform::write_u32(CONSOLE_UPDATE, 0);
    platform::write_u32(CONSOLE_TARGET, 0);
    platform::write_u16(CONSOLE_FRAME_MODE, 0x21);
    platform::write_u32(CONSOLE_FRAME_COUNTER, 0);
    platform::write_u32(CONSOLE_SCREEN, 0);
    platform::call7(
        CONSOLE_BUFFER_INIT,
        CONSOLE_STATE as u32,
        CONSOLE_DRAW_CALLBACK as u32,
        0,
        CONSOLE_SCRATCH as u32,
        0x4000,
        0,
        1,
    );
    platform::call3(MUTEX_INIT, CONSOLE_QUEUE as u32, CONSOLE_MUTEX_ARG as u32, 1);
    platform::call1(CONSOLE_BUFFER_BEGIN, CONSOLE_STATE as u32);
    platform::call2(RETRACE_CALLBACK, 2, FRAME_DRAW as u32);
    platform::call2(RETRACE_CALLBACK, 3, FRAME_DRAW as u32);
    platform::call2(RETRACE_CALLBACK, 5, FRAME_DRAW as u32);
    platform::call2(RETRACE_CALLBACK, 0x0F, FRAME_DRAW as u32);
    platform::write_u32(FRAME_PATH_FLAG, 0);
    platform::call2(RETRACE_CALLBACK, 0x10, 0);
}

/// Runs one console frame for a retrace slot.
///
/// # Original
/// - Ghidra: `FUN_80234720` at `0x80234720` (316 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80234720_FUN_80234720.c`
/// - `// observed:` a 0x10-byte record `{u16 mode, u16 pad, screen, arg3, arg4}`
///   is built on the stack and handed to `0x801B23F0`, then `0x802348F0`
///   paints while `0x801B4BB0` reports pending work.
/// - `// unknown:` `0x801AD3B0`, `0x801BC3E0`, `0x801BC430`, `0x801B50F0` and
///   `0x801B4B70` are OS/GX helpers with no exported names.
///
/// # Arguments
/// - `mode`: retrace slot index (`0xF` also clears the four EFB copy filters).
/// - `screen`: screen record whose `+0x194` word is saved.
/// - `arg3`, `arg4`: copied verbatim into the frame record.
///
/// # Side effects
/// Grows the OS flag word by bit 1, swaps the console queue state, and runs the
/// frame loop `0x802348F0` once.
///
/// # Safety
/// `screen` must be a readable screen record; the queue object must be a valid
/// OS queue and the callback table writable.
///
/// # Confidence
/// medium - the record layout and the call order are literal; the helper roles
/// are inferred from their arguments.
pub unsafe fn fn_80234720(mode: i32, screen: usize, arg3: u32, arg4: u32) {
    platform::write_u32(CONSOLE_OS_FLAGS, call0_word(OS_FLAG_GET));
    platform::write_u32(
        CONSOLE_SCREEN_SAVED,
        platform::read_u32(screen + 0x194),
    );
    platform::call1(RETRACE_DETACH, screen as u32);
    platform::call2(RETRACE_CALLBACK, mode as u32, 0);
    if mode == 0x0F {
        let mut filter = 0u32;
        while filter < 4 {
            platform::call4(EFB_COPY_CLEAR, filter, 0, 0, 3);
            filter += 1;
        }
    }

    let mut record = crate::platform::GuestLocal::new( [0u32; 4]);
    record[0] = mode as u32 & 0xFFFF;
    record[1] = screen as u32;
    record[2] = arg3;
    record[3] = arg4;
    let record_ptr = record.as_mut_ptr() as usize;
    platform::call3(MUTEX_SWAP, CONSOLE_QUEUE as u32, record_ptr as u32, 1);

    if call0_word(THREADS_RUNNING) == 0 {
        platform::call1(GX_FIFO_A, 0);
        platform::call1(GX_FIFO_B, 0);
        let flags = call0_word(OS_FLAG_GET);
        platform::call1(OS_FLAG_SET, flags | 2);
        platform::call1(CONSOLE_FRAME_LOOP, record_ptr as u32);
    }
    while call0_word(FRAME_POLL) > 0 {}
    platform::call0(CONSOLE_BUFFER_END);
    platform::call0(FRAME_RELEASE);
}

/// Links one script-path record into the queue.
///
/// # Original
/// - Ghidra: `FUN_80235710` at `0x80235710` (140 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235710_FUN_80235710.c`
/// - `// observed:` `+0x00` is cleared, `+0x04` takes the caller's priority,
///   `+0x08` takes the entry number resolved by `0x801CA680`, `+0x0C` is the
///   `next` link; priority `0` pushes to the front, any other value inserts
///   after the current head.
///
/// # Arguments
/// - `record`: 0x10-byte (or larger) queue node to link.
/// - `path`: DVD path string passed to `0x801CA680`.
/// - `priority`: `0` = push front, otherwise insert after the head.
///
/// # Returns
/// `record`.
///
/// # Safety
/// `record` must be writable for 0x10 bytes and `path` a readable C string.
///
/// # Confidence
/// high - every store and the three-way link are literal; the "priority" role
/// is `// inferred:` from the front-of-queue test.
pub unsafe fn fn_80235710(record: usize, path: u32, priority: u32) -> u32 {
    platform::write_u32(record, 0);
    platform::write_u32(record + 4, priority);
    platform::write_u32(record + 8, platform::call1(DVD_PATH_ENTRY, path));
    platform::write_u32(record + 0x0C, 0);

    let head = platform::read_u32(SCRIPT_QUEUE_HEAD);
    if head == 0 {
        platform::write_u32(SCRIPT_QUEUE_HEAD, record as u32);
    } else if priority == 0 {
        platform::write_u32(record + 0x0C, head);
        platform::write_u32(SCRIPT_QUEUE_HEAD, record as u32);
    } else {
        platform::write_u32(
            record + 0x0C,
            platform::read_u32(head as usize + 0x0C),
        );
        platform::write_u32(head as usize + 0x0C, record as u32);
    }
    record as u32
}

/// Unlinks one script-path record.
///
/// # Original
/// - Ghidra: `FUN_802357a0` at `0x802357A0` (64 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802357a0_FUN_802357a0.c`
/// - `// observed:` the head case rewrites [`SCRIPT_QUEUE_HEAD`]; otherwise the
///   walk stops at the node whose `next` is the target.
///
/// # Arguments
/// - `record`: node to remove; it does not have to be present.
///
/// # Side effects
/// Rewrites one `next` link, or the head word.
///
/// # Safety
/// Every node in the queue must be readable for 0x10 bytes.
///
/// # Confidence
/// high - both unlink forms are literal.
pub unsafe fn fn_802357a0(record: usize) {
    let head = platform::read_u32(SCRIPT_QUEUE_HEAD);
    if record as u32 == head {
        platform::write_u32(SCRIPT_QUEUE_HEAD, platform::read_u32(head as usize + 0x0C));
        return;
    }
    let mut node = head;
    loop {
        let previous = node as usize;
        if previous == 0 {
            return;
        }
        node = platform::read_u32(previous + 0x0C);
        if node == record as u32 {
            platform::write_u32(previous + 0x0C, platform::read_u32(record + 0x0C));
            return;
        }
    }
}

/// Empties the script-path queue.
///
/// # Original
/// - Ghidra: `FUN_802357f0` at `0x802357F0` (12 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802357f0_FUN_802357f0.c`
/// - `// observed:` a single store of zero to `r13 - 0x49FC`.
///
/// # Side effects
/// Clears [`SCRIPT_QUEUE_HEAD`]; the nodes themselves are not freed.
///
/// # Safety
/// Writes one word of the small-data area.
///
/// # Confidence
/// high - the body is one store.
pub unsafe fn fn_802357f0() {
    platform::write_u32(SCRIPT_QUEUE_HEAD, 0);
}

/// Tests whether the script-path queue is non-empty.
///
/// # Original
/// - Ghidra: `FUN_80235800` at `0x80235800` (20 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235800_FUN_80235800.c`
/// - `// observed:` `(-head | head) >> 31`, the sign-bit test of `-head | head`.
///
/// # Returns
/// 1 when the head word is nonzero, 0 when it is zero.
///
/// # Safety
/// Reads [`SCRIPT_QUEUE_HEAD`].
///
/// # Confidence
/// high - the expression is reproduced literally.
pub unsafe fn fn_80235800() -> u32 {
    let head = platform::read_u32(SCRIPT_QUEUE_HEAD);
    (head.wrapping_neg() | head) >> 31
}

/// Reads one byte of the console script through a 0x200-byte read window.
///
/// # Original
/// - Ghidra: `FUN_80235830` at `0x80235830` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235830_FUN_80235830.c`
/// - `// observed:` the window base is `offset & 0x7FFFFFE0`; a miss (negative
///   base, negative index, index above 0x1FF) re-reads at most 0x200 bytes with
///   `0x801CAA30`, spins on `0x801CEE90`, and returns 0 when the read reports
///   nothing; a short final window is rounded up to 0x20 bytes.
/// - `// unknown:` the six arguments of `0x801CAA30` are reproduced verbatim
///   (`file info, buffer, size, offset, 0, 2`); the two constants are not named.
///
/// # Arguments
/// - `offset`: byte offset into the script; bit 31 is ignored.
///
/// # Returns
/// The byte at that offset, or 0 when it is past the script end or unreadable.
///
/// # Side effects
/// May read from disc into [`CONSOLE_SCRATCH`] and move the window base.
///
/// # Safety
/// The file-info block and the window globals must be valid.
///
/// # Confidence
/// medium - the window arithmetic is literal; the DVD helper roles are
/// inferred from their argument shapes.
pub unsafe fn fn_80235830(offset: u32) -> u8 {
    let masked = offset & 0x7FFF_FFFF;
    let mut index = (masked as i32).wrapping_sub(platform::read_i32(SCRIPT_WINDOW_BASE));
    if masked >= platform::read_u32(SCRIPT_LENGTH) {
        return 0;
    }

    if platform::read_i32(SCRIPT_WINDOW_BASE) < 0 || index < 0 || index > 0x1FF {
        let base = offset & 0x7FFF_FFE0;
        platform::write_u32(SCRIPT_WINDOW_BASE, base);
        index = masked.wrapping_sub(base) as i32;

        let length = platform::read_u32(SCRIPT_LENGTH);
        let size = if length <= base.wrapping_add(0x200) {
            length
                .wrapping_sub(base)
                .wrapping_add(0x1F)
                & 0xFFFF_FFE0
        } else {
            0x200
        };

        let state = call0_word(INTERRUPT_ENABLE);
        let result = platform::call6(
            DVD_READ,
            SCRIPT_FILE_INFO as u32,
            CONSOLE_SCRATCH as u32,
            size,
            base,
            0,
            2,
        );
        loop {
            if platform::call1(DVD_POLL, SCRIPT_FILE_INFO as u32) == 0 {
                break;
            }
        }
        platform::call1(INTERRUPT_RESTORE, state);
        if (result as i32) < 1 {
            return 0;
        }
    }

    platform::read_u8(CONSOLE_SCRATCH + index as usize)
}

/// Finds the start of the n-th whitespace-separated token of the script.
///
/// # Original
/// - Ghidra: `FUN_80235940` at `0x80235940` (196 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235940_FUN_80235940.c`
/// - `// observed:` every byte is fetched through the callback installed in
///   `r13 - 0x49F8`; the scan stops at NUL or newline, skips runs of the
///   separator and counts down `index` at each token start.
/// - `// inferred:` the separator is passed as a byte (`0x20` at every call
///   site in `0x80235AE0`).
///
/// # Arguments
/// - `text`: script address to scan from (0 gives 0).
/// - `index`: zero-based token number.
/// - `separator`: byte that separates tokens.
///
/// # Returns
/// The address of the token start, or 0 when the text ends first.
///
/// # Safety
/// `text` must be a readable script range.
///
/// # Confidence
/// medium - the scan is literal; the callback call goes through the live
/// function pointer.
pub unsafe fn fn_80235940(text: u32, index: i32, separator: u32) -> u32 {
    if text == 0 {
        return 0;
    }
    let mut address = text;
    let mut remaining = index;
    let mut in_token = false;
    loop {
        let c = script_read_char(address) & 0xFF;
        if c == 0 || c == 10 {
            break;
        }
        if in_token {
            if c == separator {
                in_token = false;
            }
        } else if c != separator {
            in_token = remaining == 0;
            remaining = remaining.wrapping_sub(1);
            if in_token {
                return address;
            }
            in_token = true;
        }
        address = address.wrapping_add(1);
    }
    0
}

/// Parses a hexadecimal number from the console script.
///
/// # Original
/// - Ghidra: `FUN_80235a10` at `0x80235A10` (204 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235a10_FUN_80235a10.c`
/// - `// observed:` digits are `0x30..0x39`, `0x41..0x5A` and `0x61..0x7A`;
///   the value accumulates as `digit + value * 16` and returns 0 once it would
///   exceed `0x0FFFFFFF`.
///
/// # Arguments
/// - `text`: script address of the first digit.
///
/// # Returns
/// The parsed value, or 0 on overflow.
///
/// # Safety
/// `text` must be a readable script range.
///
/// # Confidence
/// high - the character ranges and the accumulator are literal.
pub unsafe fn fn_80235a10(text: u32) -> u32 {
    let mut address = text;
    let mut value = 0u32;
    loop {
        let c = script_read_char(address) & 0xFF;
        let digit = if (0x30..=0x39).contains(&c) {
            c - 0x30
        } else if (0x61..=0x7A).contains(&c) {
            c - 0x57
        } else if (0x41..=0x5A).contains(&c) {
            c - 0x37
        } else {
            return value;
        };
        if value > 0x0FFF_FFFF {
            return 0;
        }
        address = address.wrapping_add(1);
        value = digit.wrapping_add(value.wrapping_mul(16));
    }
}

/// Offers one script selector to every queued path node until one accepts it.
///
/// # Original
/// - Ghidra: `FUN_80235eb0` at `0x80235EB0` (132 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235eb0_FUN_80235eb0.c`
/// - `// observed:` walks `+0x0C` from [`SCRIPT_QUEUE_HEAD`] and stops at the
///   first node whose `0x80235DC0` dispatch returns nonzero.
///
/// # Arguments
/// - `a`, `b`, `c`: forwarded verbatim to `0x80235DC0`.
///
/// # Returns
/// 1 when a node accepted the selector, 0 when the queue is exhausted.
///
/// # Safety
/// Every queued node must be readable for 0x10 bytes.
///
/// # Confidence
/// high - the walk and the two exits are literal; the meaning of the three
/// arguments lives in the untranslated dispatcher.
pub unsafe fn fn_80235eb0(a: u32, b: u32, c: u32) -> u32 {
    let mut node = platform::read_u32(SCRIPT_QUEUE_HEAD);
    loop {
        if node == 0 {
            return 0;
        }
        if platform::call4(SCRIPT_DISPATCH_NODE, node, a, b, c) != 0 {
            return 1;
        }
        node = platform::read_u32(node as usize + 0x0C);
    }
}

/// Creates script queue A and its first record.
///
/// # Original
/// - Ghidra: `FUN_80235fe0` at `0x80235FE0` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235fe0_FUN_80235fe0.c`
/// - `// observed:` `0x801D4380(size, size2, 0)` builds the heap, the heap lock
///   word is cleared, and a 0x1C-byte record is allocated with 4-byte
///   alignment; the record is `{kind, list at +0x04, copy of *source at +0x10,
///   small-data word at +0x14, byte 1 at +0x18}`.
/// - `// unknown:` the small-data word at `r2 - 0x4C18` is copied but never
///   given a role; the heap helpers are `EXPH`-tagged allocator routines.
///
/// # Arguments
/// - `size`, `size2`: the two heap sizes (`// inferred:` usable and total).
/// - `kind`: record type word.
/// - `source`: word copied into the record's `+0x10` field.
///
/// # Side effects
/// Writes [`SCRIPT_HEAP_A`] and [`SCRIPT_RECORD_A`].
///
/// # Safety
/// The heap helpers must be initialised and `source` readable.
///
/// # Confidence
/// medium - all stores are literal; the two sizes and the small-data word are
/// not named by the binary.
pub unsafe fn fn_80235fe0(size: u32, size2: u32, kind: u32, source: usize) {
    let heap = platform::call3(HEAP_CREATE, size, size2, 0);
    platform::write_u32(SCRIPT_HEAP_A, heap);
    platform::call2(HEAP_SET_LOCK, heap, 0);

    let record = platform::call3(HEAP_ALLOC, platform::read_u32(SCRIPT_HEAP_A), 0x1C, 4) as usize;
    if record != 0 {
        platform::write_u32(record, kind);
        platform::write_u32(record + 0x10, platform::read_u32(source));
        platform::write_u32(record + 0x14, platform::read_u32(SCRIPT_RECORD_WORD));
        platform::write_u8(record + 0x18, 1);
        platform::call2(LIST_INIT, (record + 4) as u32, 0x10);
    }
    platform::write_u32(SCRIPT_RECORD_A, record as u32);
}

/// Initialises one script record in caller-provided storage.
///
/// # Original
/// - Ghidra: `FUN_80236ad0` at `0x80236AD0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236ad0_FUN_80236ad0.c`
/// - `// observed:` same record shape as [`fn_80235fe0`], but the storage is
///   the caller's and no heap is touched.
///
/// # Arguments
/// - `record`: writable 0x1C-byte record.
/// - `source`: word copied to `+0x10`.
/// - `kind`: record type word.
///
/// # Returns
/// `record`.
///
/// # Safety
/// `record` must be writable for 0x1C bytes and `source` readable.
///
/// # Confidence
/// high - the five stores and the list initialiser are literal.
pub unsafe fn fn_80236ad0(record: usize, source: usize, kind: u32) -> u32 {
    platform::write_u32(record, kind);
    platform::write_u32(record + 0x10, platform::read_u32(source));
    platform::write_u32(record + 0x14, platform::read_u32(SCRIPT_RECORD_WORD));
    platform::write_u8(record + 0x18, 1);
    platform::call2(LIST_INIT, (record + 4) as u32, 0x10);
    record as u32
}

// ---------------------------------------------------------------------------
// 0x80236C10 - tear down script queue A
// ---------------------------------------------------------------------------

/// Destroys script queue A: frees its first record, then its heap.
///
/// In plain English: when [`SCRIPT_HEAP_A`] is non-null, free
/// [`SCRIPT_RECORD_A`] back to that heap with `0x801D4710`, release the heap
/// block itself through [`crate::runtime::dvd::fn_801d4430`], and clear both
/// globals. The body returns the heap pointer it tore down (or `0` when the
/// queue was already empty).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x80236C10`.
/// - Extent: `0x80236C10..0x80236C70` (96 bytes, 24 words), ending at the
///   `blr` at `0x80236C6C`; the next body starts immediately at `0x80236C70`
///   ([`fn_80236c70`]).
/// - Reach: no `b`/`bl` and no data word in the main image or any REL.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `li r31,0`; `lwz r3,-0x49F0(r13)`;
///   `cmpi cr0,0,r3,0`; `4182002C bc 12,2,+0x2C` (beq to the return);
///   `lwz r4,-0x49EC(r13)`; `cmpi cr0,0,r4,0`; `41820008 bc 12,2`
///   (beq over the free); `bl 0x801D4710`; `lwz r31,-0x49F0(r13)`;
///   `mr r3,r31`; `bl 0x801D4430`; `li r0,0`;
///   `stw r0,-0x49F0(r13)`; `stw r0,-0x49EC(r13)`; `mr r3,r31`; epilogue;
///   `4E800020 blr`.
/// - Decoder note: the two `0x801D4710` arguments are left in `r3`/`r4` by the
///   consecutive global loads; `0x801D4430` is the one-argument block release
///   that [`crate::runtime::dvd::fn_801d4430`] models.
/// - Evidence: `work/wave285/e_dis.txt`, `work/wave285/e_refs.txt`.
///
/// # Returns
/// The destroyed heap pointer, or `0` when [`SCRIPT_HEAP_A`] was already null.
///
/// # Side effects
/// Frees the record and the heap and clears both queue-A globals.
///
/// # Safety
/// The globals must describe a live queue-A heap/record pair when non-null.
///
/// # Confidence
/// high - the guard order, the two release calls and the global clears are
/// literal.
pub unsafe fn fn_80236c10() -> u32 {
    let mut heap = platform::read_u32(SCRIPT_HEAP_A);
    if heap == 0 {
        return 0;
    }
    let record = platform::read_u32(SCRIPT_RECORD_A);
    if record != 0 {
        platform::call2(HEAP_FREE, heap, record);
    }
    heap = platform::read_u32(SCRIPT_HEAP_A);
    let _ = crate::runtime::dvd::fn_801d4430(heap as usize);
    platform::write_u32(SCRIPT_HEAP_A, 0);
    platform::write_u32(SCRIPT_RECORD_A, 0);
    heap
}

/// Reserves storage from script queue A's heap.
///
/// # Original
/// - Ghidra: `FUN_80236c70` at `0x80236C70` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236c70_FUN_80236c70.c`
/// - `// observed:` clears the heap lock word then calls `0x801D4460` with
///   `(heap, size, 4)`; the returned block is not stored, so the caller relies
///   on the side effect only.
/// - `// unknown:` why the block is not kept; no caller in this image uses the
///   result.
///
/// # Arguments
/// - `size`: allocation size.
///
/// # Side effects
/// Writes the heap lock word and allocates one block.
///
/// # Safety
/// [`SCRIPT_HEAP_A`] must hold a live heap.
///
/// # Confidence
/// medium - both calls are literal; the purpose of the unreferenced block is
/// unknown.
pub unsafe fn fn_80236c70(size: u32) {
    platform::call2(HEAP_SET_LOCK, platform::read_u32(SCRIPT_HEAP_A), 0);
    platform::call3(HEAP_ALLOC, platform::read_u32(SCRIPT_HEAP_A), size, 4);
}

/// Creates script queue B and its first record.
///
/// # Original
/// - Ghidra: `FUN_80236d70` at `0x80236D70` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236d70_FUN_80236d70.c`
/// - `// observed:` byte-for-byte the shape of [`fn_80235fe0`] with
///   [`SCRIPT_HEAP_B`]/[`SCRIPT_RECORD_B`] as the targets.
///
/// # Arguments
/// - `size`, `size2`: the two heap sizes.
/// - `kind`: record type word.
/// - `source`: word copied into the record's `+0x10` field.
///
/// # Safety
/// The heap helpers must be initialised and `source` readable.
///
/// # Confidence
/// medium - same reasoning as [`fn_80235fe0`].
pub unsafe fn fn_80236d70(size: u32, size2: u32, kind: u32, source: usize) {
    let heap = platform::call3(HEAP_CREATE, size, size2, 0);
    platform::write_u32(SCRIPT_HEAP_B, heap);
    platform::call2(HEAP_SET_LOCK, heap, 0);

    let record = platform::call3(HEAP_ALLOC, platform::read_u32(SCRIPT_HEAP_B), 0x1C, 4) as usize;
    if record != 0 {
        platform::write_u32(record, kind);
        platform::write_u32(record + 0x10, platform::read_u32(source));
        platform::write_u32(record + 0x14, platform::read_u32(SCRIPT_RECORD_WORD));
        platform::write_u8(record + 0x18, 1);
        platform::call2(LIST_INIT, (record + 4) as u32, 0x10);
    }
    platform::write_u32(SCRIPT_RECORD_B, record as u32);
}

// ---------------------------------------------------------------------------
// Wave 275 lane E - untabled gap bodies in the script-heap cluster (`_E275`)
// ---------------------------------------------------------------------------

/// Creates the script queue A heap without allocating its first record.
///
/// The body at `0x80236C70` ([`fn_80236c70`]) allocates from [`SCRIPT_HEAP_A`],
/// so this body is the heap-only half of [`fn_80235fe0`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80236BE0`.
/// - Extent: `0x80236BE0..0x80236C08` (40 bytes, 10 instructions), ending at
///   the `blr` at `0x80236C04`; one zero word precedes it at `0x80236BDC`
///   (the `blr` at `0x80236BD8` closes the previous body) and two zero words
///   follow before the next body at `0x80236C10`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: no data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `li r5,0`;
///   `stw r0,0x14(r1)`; `bl 0x801D4380`; `stw r3,-0x49F0(r13)`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `size`, `size2`: heap sizes forwarded to `0x801D4380`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Publishes the new heap at [`SCRIPT_HEAP_A`].
///
/// # Safety
/// The `EXPH` heap allocator at `0x801D4380` must be initialised.
///
/// # Confidence
/// high - the allocator call and the single SDA store are literal.
pub unsafe fn fn_80236be0(size: u32, size2: u32) {
    let heap = platform::call3(HEAP_CREATE, size, size2, 0);
    platform::write_u32(SCRIPT_HEAP_A, heap);
}

/// Creates the script queue B heap without allocating its first record.
///
/// The queue-B twin of [`fn_80236be0`]: [`fn_80236d70`] allocates the first
/// record on this heap afterwards.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80237970`.
/// - Extent: `0x80237970..0x80237998` (40 bytes, 10 instructions), ending at
///   the `blr` at `0x80237994`; one zero word precedes it at `0x8023796C`
///   (the `blr` at `0x80237968` closes the previous body) and two zero words
///   follow before the next body at `0x802379A0`.
/// - Reach: no `b` or `bl` in the executable segments targets it.
/// - Table: no data word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `li r5,0`;
///   `stw r0,0x14(r1)`; `bl 0x801D4380`; `stw r3,-0x49E8(r13)`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `size`, `size2`: heap sizes forwarded to `0x801D4380`.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Publishes the new heap at [`SCRIPT_HEAP_B`].
///
/// # Safety
/// The `EXPH` heap allocator at `0x801D4380` must be initialised.
///
/// # Confidence
/// high - the allocator call and the single SDA store are literal (the store
/// uses `-0x49E8(r13)`, the queue-B heap word).
pub unsafe fn fn_80237970(size: u32, size2: u32) {
    let heap = platform::call3(HEAP_CREATE, size, size2, 0);
    platform::write_u32(SCRIPT_HEAP_B, heap);
}

/// Initialises one script record into queue B's storage (duplicate shape).
///
/// # Original
/// - Ghidra: `FUN_80237860` at `0x80237860` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237860_FUN_80237860.c`
/// - `// observed:` identical instruction sequence to `0x80236AD0`.
///
/// # Arguments
/// - `record`: writable 0x1C-byte record.
/// - `source`: word copied to `+0x10`.
/// - `kind`: record type word.
///
/// # Returns
/// `record`.
///
/// # Safety
/// `record` must be writable for 0x1C bytes and `source` readable.
///
/// # Confidence
/// high - the five stores and the list initialiser are literal.
pub unsafe fn fn_80237860(record: usize, source: usize, kind: u32) -> u32 {
    platform::write_u32(record, kind);
    platform::write_u32(record + 0x10, platform::read_u32(source));
    platform::write_u32(record + 0x14, platform::read_u32(SCRIPT_RECORD_WORD));
    platform::write_u8(record + 0x18, 1);
    platform::call2(LIST_INIT, (record + 4) as u32, 0x10);
    record as u32
}

/// Reserves storage from script queue B's heap.
///
/// # Original
/// - Ghidra: `FUN_80237a00` at `0x80237A00` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237a00_FUN_80237a00.c`
/// - `// observed:` [`fn_80236c70`] with [`SCRIPT_HEAP_B`].
///
/// # Arguments
/// - `size`: allocation size.
///
/// # Safety
/// [`SCRIPT_HEAP_B`] must hold a live heap.
///
/// # Confidence
/// medium - same reasoning as [`fn_80236c70`].
pub unsafe fn fn_80237a00(size: u32) {
    platform::call2(HEAP_SET_LOCK, platform::read_u32(SCRIPT_HEAP_B), 0);
    platform::call3(HEAP_ALLOC, platform::read_u32(SCRIPT_HEAP_B), size, 4);
}

/// Reads one byte of the console script through the current reader callback.
///
/// `// observed:` every script reader calls `(**(code **)(r13 - 0x49F8))(addr)`
/// and keeps only the low byte, so this helper repeats that indirect call.
///
/// # Safety
/// [`SCRIPT_READ_CHAR`] must hold a valid function address.
#[inline]
unsafe fn script_read_char(address: u32) -> u32 {
    let reader = platform::read_u32(SCRIPT_READ_CHAR) as usize;
    platform::call1(reader, address)
}

/// Returns the address of the current row inside a console grid.
///
/// `// observed:` the grid rows are `(+0x04 + 1)` bytes long and the row index
/// lives at `+0x0C`.
///
/// # Safety
/// The record's grid base must be readable.
#[inline]
unsafe fn grid_row_start(record: usize, row_stride: usize) -> usize {
    platform::read_u32(record) as usize
        + row_stride * platform::read_u16(record + 0x0C) as usize
}

/// Terminates the current console row and advances the grid cursors.
///
/// This is the row-advance block that `0x80234120` repeats twice: write the
/// row terminator at the column cursor, clear `+0x0E`, bump `+0x0C` (wrapping
/// at `+0x06` unless flag bit 1 is set), and when the new row collides with the
/// oldest row `+0x10` push that row and increase the `+0x14` counter.
///
/// # Side effects
/// Writes the grid terminator and the record's cursor fields.
///
/// # Safety
/// Same requirements as [`fn_80234120`].
#[inline]
unsafe fn console_end_row(record: usize) {
    let columns = platform::read_u16(record + 0x04);
    let row_stride = columns as usize + 1;
    let row = platform::read_u16(record + 0x0C);
    let cursor = platform::read_u16(record + 0x0E) as usize;
    let base = platform::read_u32(record) as usize;
    platform::write_u8(base + row_stride * row as usize + cursor, 0);

    platform::write_u16(record + 0x0E, 0);
    let ring = platform::read_u16(record + 0x06);
    let mut next = row.wrapping_add(1);
    if next == ring && (platform::read_u16(record + 0x0A) & 2) == 0 {
        next = 0;
    }
    platform::write_u16(record + 0x0C, next);

    if next == platform::read_u16(record + 0x10) {
        let mut top = platform::read_u16(record + 0x10).wrapping_add(1);
        if top == ring {
            top = 0;
        }
        platform::write_u16(record + 0x10, top);
        platform::write_u32(
            record + 0x14,
            platform::read_u32(record + 0x14).wrapping_add(1),
        );
    }
}

// ===========================================================================
// Wave 95: console text streams, the exception dump and the message queues,
// 0x80230780..0x80238270
// ===========================================================================
//
// Everything below sits in the same `0x80230000` bucket as the BRRES loader,
// but it belongs to the engine's on-screen debug console (`console.cpp`): the
// 0x64-byte text-stream object, the exception-handler report, and the two
// heap-backed message queues that report prints through. None of it touches
// BRRES state; the bucket, not the subsystem, is what this lane owns.
//
// Three Ghidra artefacts recur in the band and are called out per body:
//
// * Metrowerks emits `bl _savegpr0_26` in every non-leaf prologue. Ghidra
//   renders those stubs as calls to `0x802DCD68`..`0x802DCDC0` and treats
//   their (non-existent) return as the new `r3:r4`. Several bodies below
//   therefore recover a parameter the export declares but never reads; the
//   recovered argument is marked `// inferred:` with its evidence.
// * `psq_st` quantisation probes (`unaff_GQR0`) are decompiler noise when the
//   probe's result is discarded. Only those discarded probes are omitted;
//   bodies whose result depends on the GQR were skipped here (wave 98 later
//   took the pure vector-store ones by passing the GQR word in as an
//   argument).
// * an indirect `call` printed with an empty argument list keeps its arguments
//   in `r3..`; the reconstruction passes the receiver explicitly.

/// `0x8022C600` - base constructor of the 0x64-byte console text stream.
///
/// `// observed:` it writes `-1` to words 0..7, `0xFF`/`0` to `+0x42`/`+0x43`,
/// a small-data word to `+0x44` and zero to `+0x48`, so the object it
/// initialises is the same 0x64-byte stream [`fn_80230780`] finishes.
const CONSOLE_STREAM_BASE_CTOR: usize = 0x8022_C600;
/// `0x8022DDE0` - constructor of the sibling stream flavour.
///
/// `// observed:` byte-for-byte the same body as [`fn_80230780`] except that
/// `+0x60` receives `r13 - 0x4A2C` instead of `r13 - 0x4A24`; the two streams
/// differ only by that class record.
const MSG_STREAM_CTOR: usize = 0x8022_DDE0;
/// `0x8022D9A0` - starts one text run on a message stream.
const MSG_STREAM_BEGIN: usize = 0x8022_D9A0;
/// `0x8022D3C0` - narrow/wide branch taken when the queued line is long.
const MSG_STREAM_NARROW: usize = 0x8022_D3C0;
/// `0x8022C9C0` - commits the pending run before the item walk.
const MSG_STREAM_COMMIT: usize = 0x8022_C9C0;
/// `0x8022DE40` - finishes a message stream; `0xFFFFFFFF` means "flush all".
const MSG_STREAM_END: usize = 0x8022_DE40;
/// `0x8022EED0` - runs one text run on a *copy* of a message stream.
///
/// `// observed:` its body is the same copy / call / copy-back / dispose shape
/// as [`fn_80231870`], with the console stream executor `0x8022FA70`.
const MSG_STREAM_RUN: usize = 0x8022_EED0;

/// `0x8022FA70` - the executor [`MSG_STREAM_RUN`] runs the copy through.
const MSG_STREAM_EXECUTE: usize = 0x8022_FA70;

/// `r13 - 0x4A28` - pointer to the global console line buffer, or 0.
const CONSOLE_STAGE_PTR: usize = SDA_BASE - 0x4A28;
/// `r13 - 0x5CE4` - capacity of that line buffer, in bytes.
const CONSOLE_LINE_CAPACITY: usize = SDA_BASE - 0x5CE4;
/// `r13 - 0x4A0C` - "console mutex has been initialised" byte.
const CONSOLE_MUTEX_READY: usize = SDA_BASE - 0x4A0C;
/// `r13 - 0x4A24` - class record stored at stream `+0x60` by [`fn_80230780`].
const CONSOLE_STREAM_CLASS: usize = SDA_BASE - 0x4A24;

/// `r2 - 0x4C38` - small-data word stored at stream `+0x4C`.
const SDA2_STREAM_WORD_A: usize = SDA2_BASE - 0x4C38;
/// `r2 - 0x4C34` - small-data word stored twice, at stream `+0x50`/`+0x54`.
///
/// `// inferred:` a float constant: the same word seeds both the text box of
/// [`fn_802322c0`] and the four corner floats of the caller at `0x802310D0`.
const SDA2_STREAM_WORD_B: usize = SDA2_BASE - 0x4C34;
/// `r2 - 0x4C30` - the `u32 -> f64` conversion bias (`2^52`).
///
/// `// observed:` [`fn_802308d0`] builds `0x43300000_<word ^ 0x80000000>` and
/// subtracts this word, the published MSL unsigned-integer conversion idiom.
const SDA2_U32_TO_DOUBLE: usize = SDA2_BASE - 0x4C30;
/// `r2 - 0x4C14` - float threshold compared with the queue's `+0x14` word.
const SDA2_QUEUE_THRESHOLD: usize = SDA2_BASE - 0x4C14;
/// `r2 - 0x4C10` - `u32 -> f64` bias used for the queued pen coordinates.
const SDA2_QUEUE_COORD_BIAS: usize = SDA2_BASE - 0x4C10;

/// `0x80234A80` - the shared variadic report sink of this band.
///
/// `// observed:` every dump below calls it as `(format, word, word, ...)`; the
/// `f64` variant used by [`fn_802352a0`] passes its value in `f1`.
const REPORT: usize = 0x8023_4A80;
/// `0x802DF3B4` - `fpclassify`: 1 = NaN, 2 = infinite, otherwise finite.
const FP_CLASSIFY: usize = 0x802D_F3B4;
/// `0x80235EB0` - offers an address to every queued script node.
///
/// `// observed:` it walks the queue at `r13 - 0x49FC`, calls `0x80235DC0`
/// with `(node, address, buffer, size)` and returns 1 as soon as one node
/// reports success. Wave 90 documented the same routine as "offer a selector
/// to every queued node"; the console dump uses it to symbolize an address.
const SCRIPT_SYMBOL: usize = 0x8023_5EB0;
/// `0x80235AE0` - parses the script command that follows the current node.
///
/// `// observed:` returns a word through `r3` although Ghidra types it `void`
/// (the value is the argument of the `_restgpr0` epilogue stub). Translated as
/// [`fn_80235ae0`] in the wave-109 section.
const SCRIPT_PARSE: usize = 0x8023_5AE0;
/// `0x80235830` - the DVD-window byte reader installed by [`fn_80235dc0`].
const SCRIPT_WINDOW_READ: usize = 0x8023_5830;
/// `0x80235820` - entry point used when the script is already buffered.
const SCRIPT_DIRECT_READ: usize = 0x8023_5820;
/// `0x803DEAF4` - file size word inside the `0x803DEAC0` DVD info block.
const SCRIPT_FILE_SIZE: usize = 0x803D_EAF4;
/// `0x801CA990` - `DVDFastOpen(entry, info)`; returns 0 on failure.
const DVD_FAST_OPEN: usize = 0x801C_A990;
/// `0x801CAA00` - `DVDClose(info)`.
const DVD_CLOSE: usize = 0x801C_AA00;

/// `0x801B25A0` - the console's memory-size word at `0x80003100`.
///
/// `// inferred:` used as the upper bound of the back-chain walk; the export
/// only shows `return _DAT_80003100`.
const MEM1_SIZE: usize = 0x801B_25A0;
/// `0x801B25B0` - the second memory-size word, used for the `0x10000000` region.
const MEM2_SIZE: usize = 0x801B_25B0;
/// `0x801B2E20` - initialise the console mutex at `0x803DA148`.
///
/// `// observed:` the wave-90 constant [`MUTEX_INIT`] is `0x801B2390`; this
/// band uses the neighbouring entry, which takes the mutex object only.
const MUTEX_INIT_ONE: usize = 0x801B_2E20;
/// `0x803DA150` - mutex state word; zero means "never contended".
const MUTEX_STATE: usize = 0x803D_A150;

/// `0x801BDE40` - current video/frame-buffer base, or 0 when unset.
const VIDEO_BASE_GET: usize = 0x801B_DE40;
/// `0x801BE010` - video mode index used to pick a default mode record.
const VIDEO_MODE_INDEX: usize = 0x801B_E010;
/// `0x801ABE80` - arena cursor the frame buffer is carved from.
const ARENA_CURSOR: usize = 0x801A_BE80;
/// `0x801BD280` - apply a video mode record.
const VIDEO_MODE_APPLY: usize = 0x801B_D280;
/// `0x801BDDD0` - publish the frame-buffer base.
const VIDEO_BASE_SET: usize = 0x801B_DDD0;
/// `0x801BDE50` - finish the mode change (`0` = no forced mode).
const VIDEO_MODE_FINISH: usize = 0x801B_DE50;
/// `0x801BDCB0` - flush the pending video state.
const VIDEO_FLUSH: usize = 0x801B_DCB0;
/// `0x801BDED0` - retrace counter, polled until it changes.
const RETRACE_COUNT: usize = 0x801B_DED0;
/// `0x803410A8` / `0x8034115C` / `0x80341198` / `0x80341120` - the four
/// built-in video mode records selected by [`VIDEO_MODE_INDEX`].
const VIDEO_MODE_NTSC: usize = 0x8034_10A8;
const VIDEO_MODE_PAL: usize = 0x8034_115C;
const VIDEO_MODE_MPAL: usize = 0x8034_1198;
const VIDEO_MODE_DEBUG: usize = 0x8034_1120;

/// `0x801D4860` - largest free block of a heap for a given alignment.
const HEAP_AVAILABLE: usize = 0x801D_4860;
/// `0x801D4710` - free one block back to its heap.
const HEAP_FREE: usize = 0x801D_4710;
/// `0x80004364` - `memcpy(dst, src, len)`.
const MEMCPY: usize = 0x8000_4364;
/// `0x802E4390` - format into a buffer and return the write count.
const FORMAT_INTO_BUFFER: usize = 0x802E_4390;
/// `0x802E15DC` - bounded `snprintf` (see `engine::main_loop`).
const SNPRINTF: usize = 0x802E_15DC;

/// Message-queue record: byte offset of the payload, past the 6-word header.
const QUEUE_RECORD_PAYLOAD: usize = 0x18;
/// Message-queue record: `+0x00` owner stream that owns the record.
const QUEUE_RECORD_OWNER: usize = 0x00;
/// Message-queue record: `+0x04` caller tag.
const QUEUE_RECORD_TAG: usize = 0x04;
/// Message-queue record: `+0x08` direction/flag word.
const QUEUE_RECORD_FLAG: usize = 0x08;
/// Message-queue record: `+0x0C` payload byte count.
const QUEUE_RECORD_LENGTH: usize = 0x0C;
/// Message-queue: list object offset inside the owner (`ut::List` at `+0x04`).
const QUEUE_LIST_OFFSET: usize = 0x04;

/// Exception dump: `EXCEPTION OCCURRED` header (format).
const EXC_FMT_OCCURRED: u32 = 0x8034_D068;
/// Exception dump: `USER HALT` header (format).
const EXC_FMT_HALT: u32 = 0x8034_D0A0;
/// Exception dump: `MainInfo` section header.
const EXC_FMT_MAIN: u32 = 0x8034_D0D0;
/// Exception dump: `EXCEPTION_INFO GPR` section header.
const EXC_FMT_GPR: u32 = 0x8034_D0E0;
/// Exception dump: rule line under a section header.
const EXC_FMT_RULE: u32 = 0x8034_D0FC;
/// Exception dump: three GPRs per line.
const EXC_FMT_GPR_ROW: u32 = 0x8034_D124;
/// Exception dump: the trailing two GPRs.
const EXC_FMT_GPR_LAST: u32 = 0x8034_D14C;
/// Exception dump: `EXCEPTION_INFO SRR0MAP` section header.
const EXC_FMT_SRR0_MAP: u32 = 0x8034_D168;
/// Exception dump: rule line of the SRR0 map.
const EXC_FMT_SRR0_RULE: u32 = 0x8034_D188;
/// Exception dump: `SRR0 = %08XH` line.
const EXC_FMT_SRR0: u32 = 0x8034_D1B4;
/// Exception dump: `no information`.
const EXC_FMT_NO_INFO: u32 = 0x8034_D1C4;
/// Exception dump: `EXCEPTION_INFO GPRMAP` section header.
const EXC_FMT_GPR_MAP: u32 = 0x8034_D1D8;
/// Exception dump: `EXCEPTION_INFO FPR` section header.
const EXC_FMT_FPR: u32 = 0x8034_D1F8;
/// Exception dump: `MSR/%08XH FPSCR/%08XH` line.
const EXC_FMT_MSR_FPSCR: u32 = 0x8034_D214;
/// Exception dump: `EXCEPTION_INFO TRACE` section header.
const EXC_FMT_TRACE: u32 = 0x8034_D230;
/// Exception dump: rule line of the trace section.
const EXC_FMT_TRACE_RULE: u32 = 0x8034_D250;
/// Exception dump: `Address BackChain LR save` column header.
const EXC_FMT_BACKCHAIN: u32 = 0x8034_D278;
/// Exception dump: one back-chain frame.
const EXC_FMT_FRAME: u32 = 0x8034_D298;
/// Exception dump: closing rule line.
const EXC_FMT_END: u32 = 0x8034_D2B0;

/// Exception dump: `CONTEXT/%08XH (%s) EXCEPTION` line.
const EXC_FMT_CONTEXT: u32 = 0x8034_D2D4;
/// Exception dump: `CONTEXT/%08XH` line when the kind is out of range.
const EXC_FMT_CONTEXT_PLAIN: u32 = 0x8034_D2F4;
/// FPE dump: `FPE: Invalid operation` group header.
const FPE_FMT_INVALID: u32 = 0x8034_D304;
/// FPE dump: positive infinity result.
const FPE_FMT_INF_POS: u32 = 0x8034_D320;
/// FPE dump: negative infinity result.
const FPE_FMT_INF_NEG: u32 = 0x8034_D338;
/// FPE dump: infinity divided by zero.
const FPE_FMT_INF_DIV_ZERO: u32 = 0x8034_D350;
/// FPE dump: invalid compare.
const FPE_FMT_BAD_COMPARE: u32 = 0x8034_D360;
/// FPE dump: software-requested exception.
const FPE_FMT_SOFTWARE: u32 = 0x8034_D374;
/// FPE dump: invalid square root.
const FPE_FMT_BAD_SQRT: u32 = 0x8034_D388;
/// FPE dump: invalid integer conversion.
const FPE_FMT_BAD_CONVERT: u32 = 0x8034_D3A0;
/// FPE dump: `FPE: Overflow` group header.
const FPE_FMT_OVERFLOW: u32 = 0x8034_D3C0;
/// FPE dump: `FPE: Underflow` group header.
const FPE_FMT_UNDERFLOW: u32 = 0x8034_D3D0;
/// FPE dump: `FPE: Zero division` group header.
const FPE_FMT_ZERO_DIV: u32 = 0x8034_D3E4;
/// FPE dump: `FPE: Inexact result` group header.
const FPE_FMT_INEXACT: u32 = 0x8034_D3FC;
/// FPE dump: `SRR0/%08XH SRR1/%08XH` line.
const FPE_FMT_SRR: u32 = 0x8034_D414;
/// FPE dump: `DSISR/%08XH DAR/%08XH` line.
const FPE_FMT_DSISR: u32 = 0x8034_D430;
/// `0x8034D024` - table of exception-name string pointers, indexed by kind.
const EXC_NAME_TABLE: usize = 0x8034_D024;
/// Schedule-1 exception kinds whose names the table holds (0..0x10).
const EXC_KIND_LIMIT: u32 = 0x11;
/// Program exception: the only kind that reports FPSCR flags.
const EXC_KIND_PROGRAM: u32 = 0x10;

/// Register dump: `s----------------------------------` header.
const REG_FMT_HEADER: u32 = 0x8034_D44C;
/// Register dump: one `R%02d  %08XH` row.
const REG_FMT_ROW: u32 = 0x8034_D478;
/// Register dump: `no register which seem to address...` trailer.
const REG_FMT_NONE: u32 = 0x8034_D488;

/// FPR dump: `----------------------------------` separator.
const FPR_FMT_RULE: u32 = 0x8034_D4B0;
/// FPR dump: `F%02d  Nan`.
const FPR_FMT_NAN: u32 = 0x8034_D4D8;
/// FPR dump: `F%02d  -Inf`.
const FPR_FMT_NEG_INF: u32 = 0x8034_D4EC;
/// FPR dump: `F%02d  Inf`.
const FPR_FMT_POS_INF: u32 = 0x8034_D500;
/// FPR dump: `F%02d  0.0`.
const FPR_FMT_ZERO: u32 = 0x8034_D514;
/// FPR dump: `F%02d  %.3E` - one integer label plus one `f64` value.
const FPR_FMT_VALUE: u32 = 0x8034_D528;

/// `r13 - 0x5CA8` - newline/indent string between dump columns.
const DUMP_GAP_A: usize = SDA_BASE - 0x5CA8;
/// `r13 - 0x5CBC` - second newline/indent string between dump columns.
const DUMP_GAP_B: usize = SDA_BASE - 0x5CBC;
/// `r13 - 0x5CC0` - newline plus the symbol buffer handed to [`REPORT`].
const DUMP_SYMBOL_LINE: usize = SDA_BASE - 0x5CC0;
/// `r13 - 0x5CB8` - FPE sub-flag line for the infinity operands.
const FPE_GAP_INF: usize = SDA_BASE - 0x5CB8;
/// `r13 - 0x5CB0` - FPE sub-flag line for the 0/0 operand pair.
const FPE_GAP_ZERO_DIV: usize = SDA_BASE - 0x5CB0;

/// `0x801C0B50` - G3D double sink used by [`fn_80238270`].
const G3D_DOUBLE_SINK: usize = 0x801C_0B50;

/// `r2 - 0x4BB0` - scale applied before [`G3D_DOUBLE_SINK`].
const SDA2_G3D_SCALE: usize = SDA2_BASE - 0x4BB0;

/// Reads one big-endian `f64` from the target image.
///
/// [`crate::platform`] only exposes `read_f32`; the FPR save area holds
/// doubles, so this helper assembles one from two `read_u32` calls.
///
/// # Safety
/// `addr` must be readable for 8 bytes.
#[inline]
unsafe fn read_f64_be(addr: usize) -> f64 {
    let high = platform::read_u32(addr) as u64;
    let low = platform::read_u32(addr + 4) as u64;
    f64::from_bits((high << 32) | low)
}

/// Calls [`REPORT`] with a format and two words.
///
/// # Safety
/// `fmt` must be a format string that consumes exactly the words passed.
#[inline]
unsafe fn report_words(fmt: u32, first: u32, second: u32) {
    platform::call3(REPORT, fmt, first, second);
}

/// Calls [`REPORT`] with an integer label and one `f64` value.
///
/// `// observed:` the export prints the value before the format because the
/// value travels in `f1`; the register order is `(format, label, f1)`.
///
/// # Safety
/// `fmt` must be a `%02d` / `%.3E` format pair.
#[inline]
unsafe fn report_index_float(fmt: u32, label: u32, value: f64) {
    let report = crate::platform::guest_fn!(REPORT, fn(u32, u32, f64));
    report(fmt, label, value);
}

/// Classifies one `f64` through `0x802DF3B4`.
///
/// # Safety
/// None; the callee only reads its argument.
#[inline]
unsafe fn classify_f64(value: f64) -> i32 {
    let classify = crate::platform::guest_fn!(FP_CLASSIFY, fn(f64) -> i32);
    classify(value)
}

/// Formats one line into the console staging buffer and runs it.
///
/// This is the shared tail of [`fn_802319b0`] and [`fn_80231ac0`]: format into
/// the staging buffer (or a frame-carved substitute when it is absent), clamp
/// the length to `capacity - 1`, and hand the text to the stream executor with
/// `mode = 1`.
///
/// `// partial:` when the global staging buffer is null the original carves
/// exactly `capacity` bytes from its own frame (`r13 - 0x5CE4`); Ghidra does
/// not reveal that capacity, so a fixed 0x800-byte local is used and the
/// capacity is clamped to it. Every shipped call uses the global buffer.
///
/// # Safety
/// `stream` must be a live console text stream and `args` a `va_list`.
#[inline]
unsafe fn console_script_format(stream: usize, fmt: u32, args: u32) {
    let capacity = platform::read_u32(CONSOLE_LINE_CAPACITY);
    let staged = platform::read_u32(CONSOLE_STAGE_PTR) as usize;
    let mut scratch = crate::platform::GuestLocal::new( [0_u8; 0x800]);
    let (buffer, capacity) = if staged == 0 {
        (
            scratch.as_mut_ptr() as usize,
            if capacity > 0x800 { 0x800 } else { capacity },
        )
    } else {
        (staged, capacity)
    };
    let written = platform::call4(FORMAT_INTO_BUFFER, buffer as u32, capacity, fmt, args);
    let limit = capacity.wrapping_sub(1);
    let length = if written > limit { limit } else { written };
    platform::call4(SCRIPT_EXECUTE, stream as u32, buffer as u32, length, 1);
}

/// Emits one queued message record.
///
/// The shared body of [`fn_80236420`] and [`fn_802371b0`]: allocate
/// `len + 0x18` bytes from the queue's heap in the direction opposite to the
/// temporary formatter, fill the four-word header, copy the payload and append
/// the record to the `ut::List` at `owner + 4`. The last two header words
/// (`+0x10`/`+0x14`) are the list links and are written by `0x80228DF0`.
///
/// # Safety
/// `owner` must be a live queue whose `+0x04` list is initialised; `data` must
/// be readable for `len` bytes.
#[inline]
unsafe fn queue_record_publish(
    heap: usize,
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    data: usize,
    len: u32,
) {
    // observed: the sink allocates from the opposite end of the heap than the
    // formatter that produced `data`.
    let mode: u32 = if direction == 1 { 4 } else { 0xFFFF_FFFC };
    platform::call2(HEAP_SET_LOCK, platform::read_u32(heap), 0);
    let block = platform::call3(
        HEAP_ALLOC,
        platform::read_u32(heap),
        len.wrapping_add(QUEUE_RECORD_PAYLOAD as u32),
        mode,
    ) as usize;
    if block != 0 {
        platform::write_u32(block + QUEUE_RECORD_OWNER, owner as u32);
        platform::write_u32(block + QUEUE_RECORD_TAG, tag);
        platform::write_u32(block + QUEUE_RECORD_FLAG, flag);
        platform::write_u32(block + QUEUE_RECORD_LENGTH, len);
        platform::call3(
            MEMCPY,
            (block + QUEUE_RECORD_PAYLOAD) as u32,
            data as u32,
            len,
        );
        platform::call2(LIST_APPEND, (owner + QUEUE_LIST_OFFSET) as u32, block as u32);
    }
}

/// Formats one message into a heap temporary and queues it.
///
/// The shared body of [`fn_80236350`], [`fn_80236700`], [`fn_802370e0`] and
/// [`fn_80237490`]. `// observed:` the temporary is allocated with the
/// direction word flipped relative to [`queue_record_publish`], so the
/// formatter's block and the published record can coexist in the same heap.
///
/// # Safety
/// `owner` must be a live queue; `args` must match `fmt`.
#[inline]
unsafe fn queue_message_format(
    heap: usize,
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    fmt: u32,
    args: u32,
    formatter: usize,
) {
    let mode: u32 = if direction == 1 { 0xFFFF_FFFC } else { 4 };
    let available = platform::call2(HEAP_AVAILABLE, platform::read_u32(heap), 4);
    let capacity = available.wrapping_sub(QUEUE_RECORD_PAYLOAD as u32 + 4) >> 1;
    platform::call2(HEAP_SET_LOCK, platform::read_u32(heap), 0);
    let buffer = platform::call3(HEAP_ALLOC, platform::read_u32(heap), capacity, mode) as usize;
    if buffer != 0 {
        let written = platform::call4(formatter, buffer as u32, capacity, fmt, args);
        queue_record_publish(heap, owner, tag, flag, direction, buffer, written);
        platform::call2(HEAP_FREE, platform::read_u32(heap), buffer as u32);
    }
}

/// `0x80231CF0` - lays out one text quad and advances the character cursor.
///
/// `// observed:` called as `(stream, quad_out, cursor)`; its own body is
/// GQR-quantised (`unaff_GQR0`) and stays untranslated, but the caller-side
/// contract is three arguments and two output blocks.
const TEXT_LINE_LAYOUT: usize = 0x8023_1CF0;

/// Initialises one 0x64-byte console text stream.
///
/// In plain English: "run the stream base constructor, then install the two
/// default colour words, the tab width, the pending-run flag and this stream
/// flavour's class record."
///
/// # Original
/// - Ghidra: `FUN_80230780` at `0x80230780` (92 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80230780_FUN_80230780.c`
/// - `// observed:` one base-constructor call, six stores, `return param_1`.
///
/// # Arguments
/// - `stream`: the 0x64-byte object to initialise; returned unchanged.
///
/// # Returns
/// `stream`.
///
/// # Side effects
/// Calls `0x8022C600` and writes `+0x4C..+0x60`.
///
/// # Safety
/// `stream` must be writable for 0x64 bytes.
///
/// # Confidence
/// high - the call and all six stores are literal. The base-constructor
/// argument is `// inferred:` the same object, because `0x8022C600` writes
/// exactly the same 0x64-byte record. The field roles stay `// unknown:`
/// except `+0x60`, which the sibling constructor `0x8022DDE0` fills with the
/// neighbouring class record.
pub unsafe fn fn_80230780(stream: usize) -> u32 {
    platform::call1(CONSOLE_STREAM_BASE_CTOR, stream as u32);
    let shared = platform::read_u32(SDA2_STREAM_WORD_B);
    platform::write_u32(stream + 0x4C, platform::read_u32(SDA2_STREAM_WORD_A));
    platform::write_u32(stream + 0x50, shared);
    platform::write_u32(stream + 0x54, shared);
    platform::write_u32(stream + 0x58, 4);
    platform::write_u32(stream + 0x5C, 0);
    platform::write_u32(stream + 0x60, CONSOLE_STREAM_CLASS as u32);
    stream as u32
}

/// Measures the advance of one stream object in pixels.
///
/// In plain English: "ask the attached source object for its count, convert
/// the count to a float, and return `origin + count * step`." A null source
/// counts as zero.
///
/// # Original
/// - Ghidra: `FUN_802308d0` at `0x802308D0` (124 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802308d0_FUN_802308d0.c`
/// - `// observed:` `CONCAT44(0x43300000, count ^ 0x80000000)` minus the word
///   at [`SDA2_U32_TO_DOUBLE`] is the published unsigned-integer conversion,
///   so the count is used as a `u32`.
///
/// # Arguments
/// - `stream`: object with a source pointer at `+0x48`, a float scale at
///   `+0x28` and a float origin at `+0x54`.
///
/// # Returns
/// The measured advance, widened to `f64` exactly as the export returns it.
///
/// # Side effects
/// One virtual call through `source->vtable[+0x34]`.
///
/// # Safety
/// `stream` must be a live stream; when `+0x48` is non-zero it must point at an
/// object whose first word points at a table with a `+0x34` entry.
///
/// # Confidence
/// medium - the guard, the virtual call, the conversion and the two float
/// fields are literal; the receiver of the virtual call is `// inferred:`
/// because Ghidra prints the call with an empty argument list.
pub unsafe fn fn_802308d0(stream: usize) -> f64 {
    let source = platform::read_u32(stream + 0x48) as usize;
    let count = if source == 0 {
        0
    } else {
        let table = platform::read_u32(source) as usize;
        let method = platform::read_u32(table + 0x34) as usize;
        platform::call1(method, source as u32)
    };
    let origin = platform::read_f32(stream + 0x54);
    let step = platform::read_f32(stream + 0x28);
    (origin + (count as f32) * step) as f64
}

/// Runs one queued text run through a copy of a console stream.
///
/// In plain English: "snapshot the stream, draw the text with the snapshot,
/// keep the advanced cursor, discard the rest."
///
/// # Original
/// - Ghidra: `FUN_80231870` at `0x80231870` (320 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80231870_FUN_80231870.c`
/// - `// observed:` 0x64 bytes are copied word by word (with the `u16` at
///   `+0x40` and the bytes at `+0x42`/`+0x43`), the executor is called with
///   `mode = 0`, then `+0x2C`/`+0x30` are copied back and the snapshot is
///   disposed with a non-positive count (so nothing is freed).
/// - `// observed:` the two `unaff_GQR0` probes only call `ldexpf` and discard
///   the result; they are the decompiler's rendering of the paired-single
///   conversions and are omitted.
///
/// # Arguments
/// - `stream`: the live 0x64-byte stream object.
/// - `text`: bytes of the run to draw.
/// - `len`: byte count of the run.
///
/// # Side effects
/// Draws through `0x802323E0` and writes the two cursor words back.
///
/// # Safety
/// `stream` must be writable for 0x64 bytes and `text` readable for `len`.
///
/// # Confidence
/// medium - the copy, the call, the copy-back and the dispose are literal; the
/// roles of `+0x2C`/`+0x30` as the advanced cursor are `// inferred:` from the
/// caller `0x80237610`, which stores the queued pen position there first.
pub unsafe fn fn_80231870(stream: usize, text: usize, len: u32) {
    let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
    let base = snapshot.as_mut_ptr() as usize;
    for offset in 0..0x64 {
        platform::write_u8(base + offset, platform::read_u8(stream + offset));
    }
    platform::call4(SCRIPT_EXECUTE, base as u32, text as u32, len, 0);
    platform::write_u32(stream + 0x2C, platform::read_u32(base + 0x2C));
    platform::write_u32(stream + 0x30, platform::read_u32(base + 0x30));
    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
}

/// Formats one console-script line and runs it.
///
/// In plain English: "`fprintf` the caller's arguments into the console line
/// buffer, truncate to `capacity - 1`, and execute the result as one script
/// line."
///
/// # Original
/// - Ghidra: `FUN_802319b0` at `0x802319B0` (272 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802319b0_FUN_802319b0.c`
/// - `// observed:` the body builds the `va_list` descriptor `{2, 0, stack+8,
///   gpr_save}` from the eight GPR and eight FPR save slots Ghidra prints as
///   `param_1..param_16`, then calls `0x802E4390` and `0x802323E0`.
///
/// # Arguments
/// - `stream`: `r3`; the console text stream to execute on.
/// - `fmt`: `r4`; the format string.
/// - `args`: the variadic argument descriptor the original builds in its own
///   frame. A caller in this crate passes the address of an equivalent
///   descriptor because Rust cannot forward C varargs.
///
/// # Side effects
/// Writes into the console line buffer and runs the formatted text.
///
/// # Safety
/// `fmt` and `args` must describe a valid format call, and the global staging
/// buffer must be absent or at least [`CONSOLE_LINE_CAPACITY`] bytes long.
///
/// # Confidence
/// medium - the two calls, the length clamp and the buffer choice are literal;
/// the fixed-point arguments are reconstructed from the save-area layout.
pub unsafe fn fn_802319b0(stream: usize, fmt: u32, args: u32) {
    console_script_format(stream, fmt, args);
}

/// Formats one console-script line from an explicit `va_list`.
///
/// In plain English: the `va_list` half of [`fn_802319b0`] - same buffer, same
/// clamp, same executor call.
///
/// # Original
/// - Ghidra: `FUN_80231ac0` at `0x80231AC0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80231ac0_FUN_80231ac0.c`
/// - `// observed:` `FUN_802E4390(buffer, capacity, param_2, param_3)` followed
///   by `FUN_802323E0(param_1, buffer, min(written, capacity - 1), 1)`.
///
/// # Arguments
/// - `stream`: `param_1`; the console text stream.
/// - `fmt`: `param_2`; the format string.
/// - `args`: `param_3`; the `va_list` pointer.
///
/// # Side effects
/// Writes into the console line buffer and runs the formatted text.
///
/// # Safety
/// As [`fn_802319b0`].
///
/// # Confidence
/// high - the argument positions, the clamp and both calls are literal; only
/// the frame-carved fallback buffer is approximated (see
/// [`console_script_format`]).
pub unsafe fn fn_80231ac0(stream: usize, fmt: u32, args: u32) {
    console_script_format(stream, fmt, args);
}

/// Measures the bounding box of a run of text.
///
/// In plain English: "seed four box edges, then lay out one text quad at a time
/// and widen the box until the character cursor passes the end of the run."
///
/// # Original
/// - Ghidra: `FUN_802322c0` at `0x802322C0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802322c0_FUN_802322c0.c`
/// - `// observed:` four `min` merges for the first two edges, two `max` merges
///   for the last two, and the do/while test
///   `0 < (start + count * 2 - cursor) / 2`.
///
/// # Arguments
/// - `stream`: the stream the quads are laid out on; `+0x2C`/`+0x30` are
///   seeded here and advanced by the layout engine.
/// - `rect`: four writable floats, `{x0, y0, x1, y1}`.
/// - `start`: first character index of the run.
/// - `count`: half the number of characters to cover (the test doubles it).
///
/// # Side effects
/// Writes `rect` and the stream's pen words.
///
/// # Safety
/// `rect` must be writable for 16 bytes and `stream` a live stream.
///
/// # Confidence
/// medium - every store, comparison and the loop test are literal; the seed
/// constant's value is `// unknown:` (only its address is known) and the
/// layout engine `0x80231CF0` is a GQR-quantised body called by address.
pub unsafe fn fn_802322c0(stream: usize, rect: usize, start: u32, count: i32) {
    let seed = platform::read_f32(SDA2_STREAM_WORD_B);
    platform::write_f32(rect, seed);
    platform::write_f32(rect + 4, seed);
    platform::write_f32(rect + 8, seed);
    platform::write_f32(rect + 12, seed);
    platform::write_f32(stream + 0x2C, seed);
    platform::write_f32(stream + 0x30, seed);

    let mut cursor = crate::platform::GuestLocal::new( [start]);
    let limit = start.wrapping_add((count as u32).wrapping_mul(2));
    loop {
        let mut quad = crate::platform::GuestLocal::new( [seed; 4]);
        // observed 0x8023232C: `r6` is still the caller's `count` here, and the
        // layout engine reads it as its fourth argument.
        platform::call4(
            TEXT_LINE_LAYOUT,
            stream as u32,
            quad.as_mut_ptr() as u32,
            cursor.as_mut_ptr() as u32,
            count as u32,
        );
        if quad[0] < platform::read_f32(rect) {
            platform::write_f32(rect, quad[0]);
        }
        if quad[1] < platform::read_f32(rect + 4) {
            platform::write_f32(rect + 4, quad[1]);
        }
        if platform::read_f32(rect + 8) < quad[2] {
            platform::write_f32(rect + 8, quad[2]);
        }
        if platform::read_f32(rect + 12) < quad[3] {
            platform::write_f32(rect + 12, quad[3]);
        }
        let remaining = (limit as i32).wrapping_sub(cursor[0] as i32) / 2;
        if remaining <= 0 {
            break;
        }
    }
}

/// Allocates (or reuses) the console framebuffer and publishes its geometry.
///
/// In plain English: "install the 640x480 defaults once, pick a video mode if
/// the caller did not, carve a buffer from the top of the arena, wait for a
/// retrace, then publish the size and geometry back into the console block."
///
/// # Original
/// - Ghidra: `FUN_80233a70` at `0x80233A70` (492 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233a70_FUN_80233a70.c`
/// - `// observed:` the default block writes the same `0x803DA120` record that
///   `0x802331A0` writes; the size expression is
///   `arena + (w + 0xF & ~0xF) * h * -2 & ~0x1F`.
///
/// # Arguments
/// - `mode`: a video mode record, or 0 to select the console default.
///
/// # Returns
/// The framebuffer base, also written to [`CONSOLE_PIXELS`].
///
/// # Side effects
/// Rewrites the console geometry block, calls the video-mode helpers
/// `0x801BDE40`, `0x801BD280`, `0x801BDDD0`, `0x801BDE50`, `0x801BDCB0` and
/// spins on `0x801BDED0` until the retrace counter changes.
///
/// # Safety
/// `mode` must be 0 or a readable mode record with a `u16` width at `+4` and a
/// `u16` height at `+8`.
///
/// # Confidence
/// medium - every store and the size arithmetic are literal; the callee roles
/// (mode query, arena cursor, retrace counter) are `// inferred:` from their
/// argument shapes and from the same call group in `0x802348F0`.
pub unsafe fn fn_80233a70(mode: usize) -> u32 {
    if platform::read_u32(CONSOLE_DEFAULTS_READY) == 0 {
        platform::write_u16(CONSOLE_WIDTH, 0x0280);
        platform::write_u32(CONSOLE_PIXELS, 0);
        platform::write_u16(CONSOLE_HEIGHT, 0x01E0);
        platform::write_u16(CONSOLE_STRIDE, 0x0280);
        platform::write_u32(CONSOLE_PIXEL_BYTES, 0x0009_6000);
        // observed: four separate 0xFF byte stores in the export.
        platform::write_u16(CONSOLE_PALETTE_BYTES, 0xFFFF);
        platform::write_u16(CONSOLE_PALETTE_BYTES + 2, 0xFFFF);
        platform::write_u32(CONSOLE_PIXEL_BASE, 0xEB00);
        platform::write_u16(CONSOLE_COMPONENTS, 0x7F);
        platform::write_u16(CONSOLE_COMPONENTS + 2, 0x3F);
        platform::write_u16(CONSOLE_COMPONENTS + 4, 0x1F);
        platform::write_u16(CONSOLE_COMPONENTS + 6, 0x7F);
        platform::write_u16(CONSOLE_COMPONENTS + 8, 0x3F);
        platform::write_u16(CONSOLE_COMPONENTS + 0x0A, 0x1F);
        platform::write_u32(CONSOLE_DEFAULTS_READY, 1);
    }

    let mut mode = mode;
    let mut base = call0_word(VIDEO_BASE_GET);
    if base == 0 {
        if mode == 0 {
            let selected = match call0_word(VIDEO_MODE_INDEX) {
                0 => VIDEO_MODE_NTSC,
                1 => VIDEO_MODE_PAL,
                5 => VIDEO_MODE_MPAL,
                2 => VIDEO_MODE_DEBUG,
                // unknown: an index outside the four built-in records leaves
                // the caller's null mode in place.
                _ => 0,
            };
            mode = selected;
        }
        let arena = call0_word(ARENA_CURSOR);
        let aligned_width =
            (platform::read_u16(mode + 4) as u32).wrapping_add(0xF) & 0xFFFF_FFF0;
        let bytes = aligned_width
            .wrapping_mul(platform::read_u16(mode + 8) as u32)
            .wrapping_mul(0xFFFF_FFFE);
        base = arena.wrapping_add(bytes) & 0xFFFF_FFE0;
        platform::call1(VIDEO_MODE_APPLY, mode as u32);
        platform::call1(VIDEO_BASE_SET, base);
    }
    platform::call1(VIDEO_MODE_FINISH, 0);
    platform::call0(VIDEO_FLUSH);

    // observed: enable interrupts, spin until the retrace counter moves, then
    // restore the saved MSR word.
    let token = call0_word(INTERRUPT_ENABLE);
    let first = call0_word(RETRACE_COUNT);
    loop {
        if call0_word(RETRACE_COUNT) != first {
            break;
        }
    }
    platform::call1(INTERRUPT_RESTORE, token);

    if mode != 0 {
        platform::write_u16(CONSOLE_WIDTH, platform::read_u16(mode + 4));
        platform::write_u16(CONSOLE_HEIGHT, platform::read_u16(mode + 8));
        let aligned = (platform::read_u16(mode + 4) as u32).wrapping_add(0xF) & 0xFFF0;
        platform::write_u16(CONSOLE_STRIDE, aligned as u16);
        platform::write_u32(
            CONSOLE_PIXEL_BYTES,
            aligned
                .wrapping_mul(platform::read_u16(mode + 8) as u32)
                .wrapping_mul(2),
        );
    }
    platform::write_u32(CONSOLE_PIXELS, base);
    base
}

/// Fills in and registers one console record.
///
/// In plain English: "initialise the console mutex once, write the record's
/// geometry and cursor fields, and link it into the registered-console list in
/// descending order of the record's `+0x08` word."
///
/// # Original
/// - Ghidra: `FUN_80233d10` at `0x80233D10` (424 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233d10_FUN_80233d10.c`
/// - `// observed:` the sort key is the `u16` at `+0x08`; the list head is at
///   `r13 - 0x4A10` and the link word is `+0x28`. Every store is relative to
///   the record block and covers `+0x00..+0x24` plus the link.
/// - `// inferred:` `r3` is the record. The export reads the block from
///   `FUN_802DCD70` (`_savegpr0_26`), which returns nothing; its single caller
///   `0x802D7570` passes the freshly allocated `(n + 1) * stride + 0x2C` block
///   there. That size matches `record + 0x2C`, the value stored at `+0x00`.
///
/// # Arguments
/// - `record`: the block to initialise, at least `0x2C` bytes plus the line
///   array.
/// - `field_04`, `field_06`, `field_20`, `field_08`, `field_0a`: `r4..r8`,
///   stored verbatim at `+0x04`, `+0x06`, `+0x20`, `+0x08` and `+0x0A`. The
///   order follows the register assignment, which the export permutes.
///
/// # Side effects
/// Initialises `0x803DA148` on first use, takes and releases that mutex, and
/// rewrites the registered-console list.
///
/// # Safety
/// `record` must be writable for 0x2C bytes and every entry already in the
/// list must be a live record.
///
/// # Confidence
/// medium - every store and the whole sorted insert are literal; the record
/// argument and the five field roles are `// inferred:` as described above.
pub unsafe fn fn_80233d10(
    record: usize,
    field_04: u16,
    field_06: u16,
    field_20: u16,
    field_08: u16,
    field_0a: u16,
) {
    if platform::read_u8(CONSOLE_MUTEX_READY) == 0 {
        platform::call1(MUTEX_INIT_ONE, CONSOLE_MUTEX as u32);
        platform::write_u8(CONSOLE_MUTEX_READY, 1);
    }

    platform::write_u32(record, (record + 0x2C) as u32);
    platform::write_u16(record + 0x04, field_04);
    platform::write_u16(record + 0x06, field_06);
    platform::write_u16(record + 0x08, field_08);
    platform::write_u16(record + 0x0A, field_0a);
    platform::write_u8(record + 0x22, 0);
    platform::write_u16(record + 0x0C, 0);
    platform::write_u16(record + 0x0E, 0);
    platform::write_u16(record + 0x10, 0);
    platform::write_u32(record + 0x14, 0);
    platform::write_u32(record + 0x18, 0);
    platform::write_u16(record + 0x1C, 0x1E);
    platform::write_u16(record + 0x1E, 0x32);
    platform::write_u16(record + 0x20, field_20);
    platform::write_u32(record + 0x24, 0);

    // observed: `countLeadingZeros(state) >> 5` is a "state word is zero"
    // test. With threads running the mutex is taken instead and the test is
    // treated as satisfied.
    let contended = if call0_word(THREADS_RUNNING) == 0 {
        platform::read_u32(MUTEX_STATE) == 0
    } else {
        platform::call1(MUTEX_LOCK, CONSOLE_MUTEX as u32);
        true
    };
    if contended {
        platform::call0(INTERRUPT_DISABLE);
        platform::write_u16(record + 0x0C, 0);
        platform::write_u16(record + 0x0E, 0);
        platform::write_u16(record + 0x10, 0);
        platform::write_u32(record + 0x14, 0);
        platform::write_u32(record + 0x18, 0);
        platform::call0(INTERRUPT_RESTORE);
        if call0_word(THREADS_RUNNING) != 0 {
            platform::call1(MUTEX_UNLOCK, CONSOLE_MUTEX as u32);
        }
    }

    platform::call1(MUTEX_LOCK, CONSOLE_MUTEX as u32);
    let head = platform::read_u32(CONSOLE_LIST) as usize;
    let mut previous = 0usize;
    if head != 0 && platform::read_u16(head + 0x08) >= field_08 {
        let mut node = head;
        loop {
            previous = node;
            let next = platform::read_u32(node + 0x28) as usize;
            if next == 0 {
                break;
            }
            if field_08 <= platform::read_u16(next + 0x08) {
                node = next;
            } else {
                break;
            }
        }
    }
    if previous == 0 {
        platform::write_u32(record + 0x28, head as u32);
        platform::write_u32(CONSOLE_LIST, record as u32);
    } else {
        platform::write_u32(record + 0x28, platform::read_u32(previous + 0x28));
        platform::write_u32(previous + 0x28, record as u32);
    }
    platform::call1(MUTEX_UNLOCK, CONSOLE_MUTEX as u32);
}

/// `0x803DA89C` - saved back-chain frame pointer of the exception dump.
const CONSOLE_LAST_FRAME: usize = 0x803D_A89C;
/// `r2 - 0x4C20` - float zero used by the FPR formatter's `0.0` test.
const SDA2_FPR_ZERO: usize = SDA2_BASE - 0x4C20;

/// Reports one `f32`-narrowed FPR value with its classification.
///
/// The shared row body of [`fn_802352a0`]: NaN, positive/negative infinity,
/// `0.0`, otherwise `%.3E`. `// observed:` the narrowed value is compared with
/// the sdata zero *after* widening, so `-0.0` also prints as `0.0`.
///
/// # Safety
/// None; every argument is by value.
#[inline]
unsafe fn report_fpr(label: u32, raw: f64) {
    let value = raw as f32;
    match classify_f64(value as f64) {
        1 => {
            platform::call2(REPORT, FPR_FMT_NAN, label);
        }
        2 => {
            let negative = (value.to_bits() & 0x8000_0000) != 0;
            platform::call2(
                REPORT,
                if negative {
                    FPR_FMT_NEG_INF
                } else {
                    FPR_FMT_POS_INF
                },
                label,
            );
        }
        _ => {
            if platform::read_f32(SDA2_FPR_ZERO) as f64 == value as f64 {
                platform::call2(REPORT, FPR_FMT_ZERO, label);
            } else {
                report_index_float(FPR_FMT_VALUE, label, value as f64);
            }
        }
    }
}

/// Prints the full exception report selected by the dump mask.
///
/// In plain English: "if the exception kind is in range print its name, then
/// print whichever sections the mask at `0x803DA8B8` selects: main info, GPRs,
/// SRR0 map, GPR map, FPRs and a back-chain walk."
///
/// # Original
/// - Ghidra: `FUN_80234b60` at `0x80234B60` (988 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80234b60_FUN_80234b60.c`
/// - `// observed:` the mask bits are 1 main, 2 GPR, 4 GPR map, 8 SRR0 map,
///   0x10 FPR, 0x20 trace; the frame counter is `0x803DA898` and the back-chain
///   starts at `0x803DA89C`.
/// - `// inferred:` `r3:r4` are the exception kind and the saved context. The
///   export reads them from `FUN_802DCD70` (`_savegpr0_26`), which returns
///   nothing; its only caller `0x802348F0` passes exactly `(kind, context,
///   dsisr, dar)`.
///
/// # Arguments
/// - `kind`: exception vector number (0..0x10).
/// - `context`: saved-context block: GPRs at `+0x00`, SRR0/SRR1 at
///   `+0x198`/`+0x19C`, FPRs at `+0x90`.
/// - `dsisr`, `dar`: DSISR and DAR values of the fault.
///
/// # Side effects
/// Prints through `0x80234A80` and reads the dump globals.
///
/// # Safety
/// `context` must be readable for 0x1A0 bytes, and the symbol lookups require
/// the script map to be initialised.
///
/// # Confidence
/// medium - every store, call, mask bit and loop bound is literal; the two
/// register arguments are recovered from the prologue stub as described above.
pub unsafe fn fn_80234b60(kind: u32, context: usize, dsisr: u32, dar: u32) {
    let frame_counter = platform::read_u32(CONSOLE_FRAME_COUNTER);
    if kind < EXC_KIND_LIMIT {
        platform::call2(REPORT, EXC_FMT_OCCURRED, frame_counter);
    } else {
        platform::call2(REPORT, EXC_FMT_HALT, frame_counter);
    }
    let mask = platform::read_u16(CONSOLE_FRAME_MODE);

    if (mask & 1) != 0 {
        platform::call1(REPORT, EXC_FMT_MAIN);
        fn_80234f40(kind, context, dsisr, dar);
    }

    if (mask & 2) != 0 {
        platform::call1(REPORT, EXC_FMT_GPR);
        platform::call1(REPORT, EXC_FMT_RULE);
        for index in 0..10_u32 {
            let offset = index as usize * 4;
            platform::call7(
                REPORT,
                EXC_FMT_GPR_ROW,
                index,
                platform::read_u32(context + offset),
                index + 0xB,
                platform::read_u32(context + offset + 0x2C),
                index + 0x16,
                platform::read_u32(context + offset + 0x58),
            );
        }
        platform::call5(
            REPORT,
            EXC_FMT_GPR_LAST,
            10,
            platform::read_u32(context + 0x28),
            0x15,
            platform::read_u32(context + 0x54),
        );
    }

    if (mask & 8) != 0 {
        platform::call1(REPORT, EXC_FMT_SRR0_MAP);
        platform::call1(REPORT, EXC_FMT_SRR0_RULE);
        let srr0 = platform::read_u32(context + 0x198);
        if srr0 > 0x7FFF_FFFF && srr0 < 0x8300_0000 {
            platform::call2(REPORT, EXC_FMT_SRR0, srr0);
            let mut buffer = crate::platform::GuestLocal::new( [0_u8; 0x104]);
            let mut found = false;
            if fn_80235800() != 0 && srr0 >= 0x8000_0000 && srr0 <= 0x82FF_FFFF {
                if platform::call3(SCRIPT_SYMBOL, srr0, buffer.as_mut_ptr() as u32, 0x104) != 0 {
                    platform::call2(REPORT, DUMP_SYMBOL_LINE as u32, buffer.as_mut_ptr() as u32);
                    found = true;
                }
            }
            if !found {
                platform::call1(REPORT, EXC_FMT_NO_INFO);
            }
        }
    }

    if (mask & 4) != 0 {
        platform::call1(REPORT, EXC_FMT_GPR_MAP);
        fn_80235180(context);
    }

    if (mask & 0x10) != 0 {
        platform::call1(REPORT, EXC_FMT_FPR);
        fn_802352a0(context);
        // observed: the same two words the frame path saves; during a dump
        // they hold MSR and FPSCR.
        platform::call3(
            REPORT,
            EXC_FMT_MSR_FPSCR,
            platform::read_u32(CONSOLE_OS_FLAGS),
            platform::read_u32(CONSOLE_SCREEN_SAVED),
        );
    }

    if (mask & 0x20) != 0 {
        platform::call1(REPORT, EXC_FMT_TRACE);
        platform::call1(REPORT, EXC_FMT_TRACE_RULE);
        platform::call1(REPORT, EXC_FMT_BACKCHAIN);
        let mut frame = platform::read_u32(CONSOLE_LAST_FRAME) as usize;
        for _ in 0..0x10 {
            let mut valid = false;
            if frame != 0 && frame != 0xFFFF_FFFF {
                let region = frame as u32 & 0x3000_0000;
                if region == 0 {
                    valid = (frame as u32 & 0x0FFF_FFFF) < call0_word(MEM1_SIZE);
                } else if region == 0x1000_0000 {
                    valid = (frame as u32 & 0x0FFF_FFFF) < call0_word(MEM2_SIZE);
                }
            }
            if !valid {
                break;
            }
            platform::call4(
                REPORT,
                EXC_FMT_FRAME,
                frame as u32,
                platform::read_u32(frame),
                platform::read_u32(frame + 4),
            );
            let saved = platform::read_u32(frame + 4);
            let mut buffer = crate::platform::GuestLocal::new( [0_u8; 0x104]);
            let mut found = false;
            if fn_80235800() != 0 && saved >= 0x8000_0000 && saved <= 0x82FF_FFFF {
                if platform::call3(SCRIPT_SYMBOL, saved, buffer.as_mut_ptr() as u32, 0x104) != 0 {
                    platform::call2(REPORT, DUMP_SYMBOL_LINE as u32, buffer.as_mut_ptr() as u32);
                    found = true;
                }
            }
            if !found {
                platform::call1(REPORT, DUMP_GAP_B as u32);
            }
            frame = platform::read_u32(frame) as usize;
        }
    }

    platform::call1(REPORT, EXC_FMT_END);
}

/// Prints the exception context header and, for program exceptions, the FPSCR
/// flag groups.
///
/// # Original
/// - Ghidra: `FUN_80234f40` at `0x80234F40` (572 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80234f40_FUN_80234f40.c`
/// - `// observed:` the `kind < 0x11` test is written as the 64-bit compare
///   `r3:r4 < 0x1100000000`; the exception name comes from the pointer table at
///   `0x8034D024`, and the FPE group walks FPSCR bits `3..7` shifted left by 22
///   plus the explicit overflow/underflow/zero/inexact bits.
/// - `// inferred:` `r3` is the kind and `r4` the context, for the reason given
///   in [`fn_80234b60`]; `0x80234B60` calls this as `(kind, context, ...)`.
///
/// # Arguments
/// - `kind`: exception vector number.
/// - `context`: saved-context block; SRR0/SRR1 are read at `+0x198`/`+0x19C`.
/// - `dsisr`, `dar`: printed verbatim on the last line.
///
/// # Side effects
/// Prints through `0x80234A80` only.
///
/// # Safety
/// `context` must be readable for 0x1A0 bytes.
///
/// # Confidence
/// medium - every branch, bit test and format call is literal; the argument
/// order is recovered from the prologue stub, and the FPE sub-lines at
/// `r13 - 0x5CB8` / `r13 - 0x5CB0` are only known by address.
pub unsafe fn fn_80234f40(kind: u32, context: usize, dsisr: u32, dar: u32) {
    if kind < EXC_KIND_LIMIT {
        let name = platform::read_u32(EXC_NAME_TABLE + kind as usize * 4);
        report_words(EXC_FMT_CONTEXT, context as u32, name);
    } else {
        platform::call2(REPORT, EXC_FMT_CONTEXT_PLAIN, context as u32);
    }

    if kind == EXC_KIND_PROGRAM {
        let fpscr = platform::read_u32(CONSOLE_SCREEN_SAVED);
        let flags = fpscr & (fpscr & 0xF8) << 0x16;
        if (flags & 0x2000_0000) != 0 {
            platform::call1(REPORT, FPE_FMT_INVALID);
            if (fpscr & 0x0100_0000) != 0 {
                platform::call1(REPORT, FPE_GAP_INF as u32);
            }
            if (fpscr & 0x0080_0000) != 0 {
                platform::call1(REPORT, FPE_FMT_INF_POS);
            }
            if (fpscr & 0x0040_0000) != 0 {
                platform::call1(REPORT, FPE_FMT_INF_NEG);
            }
            if (fpscr & 0x0020_0000) != 0 {
                platform::call1(REPORT, FPE_GAP_ZERO_DIV as u32);
            }
            if (fpscr & 0x0010_0000) != 0 {
                platform::call1(REPORT, FPE_FMT_INF_DIV_ZERO);
            }
            if (fpscr & 0x0008_0000) != 0 {
                platform::call1(REPORT, FPE_FMT_BAD_COMPARE);
            }
            if (fpscr & 0x0000_0400) != 0 {
                platform::call1(REPORT, FPE_FMT_SOFTWARE);
            }
            if (fpscr & 0x0000_0200) != 0 {
                platform::call1(REPORT, FPE_FMT_BAD_SQRT);
            }
            if (fpscr & 0x0000_0100) != 0 {
                platform::call1(REPORT, FPE_FMT_BAD_CONVERT);
            }
        }
        if (flags & 0x1000_0000) != 0 {
            platform::call1(REPORT, FPE_FMT_OVERFLOW);
        }
        if (flags & 0x0800_0000) != 0 {
            platform::call1(REPORT, FPE_FMT_UNDERFLOW);
        }
        if (flags & 0x0400_0000) != 0 {
            platform::call1(REPORT, FPE_FMT_ZERO_DIV);
        }
        if (flags & 0x0200_0000) != 0 {
            platform::call1(REPORT, FPE_FMT_INEXACT);
        }
    }

    platform::call3(
        REPORT,
        FPE_FMT_SRR,
        platform::read_u32(context + 0x198),
        platform::read_u32(context + 0x19C),
    );
    platform::call3(REPORT, FPE_FMT_DSISR, dsisr, dar);
}

/// Dumps the 31 general-purpose registers of a saved context and symbolizes
/// every value that falls inside the code regions.
///
/// # Original
/// - Ghidra: `FUN_80235180` at `0x80235180` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235180_FUN_80235180.c`
/// - `// observed:` the loop walks 0x1F words from the context, prints
///   `R%02d  %08XH`, and recurses into the symbol map only for values in
///   `0x80000000..0x82FFFFFF`.
/// - `// inferred:` the context pointer is `r3`. The export declares the body
///   as `void f(void)` and reads the pointer out of `FUN_802DCD6C`
///   (`_savegpr0_25`), which returns nothing; its only caller `0x80234B60`
///   passes its own context argument here.
///
/// # Arguments
/// - `context`: saved-context block; register `n` is the word at `4 * n`.
///
/// # Side effects
/// Prints through `0x80234A80`.
///
/// # Safety
/// `context` must be readable for 0x7C bytes and the script map initialised
/// for the symbol lookups.
///
/// # Confidence
/// medium - the loop, the range test and every print are literal; the argument
/// is reconstructed as described above.
pub unsafe fn fn_80235180(context: usize) {
    platform::call1(REPORT, REG_FMT_HEADER);
    let mut buffer = crate::platform::GuestLocal::new( [0_u8; 0x104]);
    let mut any = false;
    for index in 0..0x1F_u32 {
        let value = platform::read_u32(context + index as usize * 4);
        if value > 0x7FFF_FFFF && value < 0x8300_0000 {
            any = true;
            report_words(REG_FMT_ROW, index, value);
            let mut found = false;
            if fn_80235800() != 0 && value >= 0x8000_0000 && value <= 0x82FF_FFFF {
                if platform::call3(SCRIPT_SYMBOL, value, buffer.as_mut_ptr() as u32, 0x104) != 0 {
                    platform::call2(REPORT, DUMP_SYMBOL_LINE as u32, buffer.as_mut_ptr() as u32);
                    found = true;
                }
            }
            if !found {
                platform::call1(REPORT, EXC_FMT_NO_INFO);
            }
        }
    }
    if !any {
        platform::call1(REPORT, REG_FMT_NONE);
    }
}

/// Dumps the 32 floating-point registers of a saved context in three columns.
///
/// # Original
/// - Ghidra: `FUN_802352a0` at `0x802352A0` (1032 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802352a0_FUN_802352a0.c`
/// - `// observed:` the FPR block starts at `context + 0x90` and each value is
///   narrowed to `f32` before it is classified; the loop prints registers
///   `i`, `i + 0xB` and `i + 0x16` for `i = 0..9`, then completes columns two
///   and three with registers 10 and 21.
/// - `// observed:` the two `unaff_GQR0` probes only discard their result.
///
/// # Arguments
/// - `context`: saved-context block; FPR `n` is the `f64` at `0x90 + 8 * n`.
///
/// # Side effects
/// Prints through `0x80234A80`, including the two `r13`-based separator lines.
///
/// # Safety
/// `context` must be readable for `0x90 + 0x100` bytes.
///
/// # Confidence
/// medium - the layout arithmetic and every branch are literal; the `%.3E`
/// row is emitted with the label in `r4` and the value in `f1`, which is the
/// published MSL varargs order.
pub unsafe fn fn_802352a0(context: usize) {
    platform::call1(REPORT, FPR_FMT_RULE);
    for index in 0..10_usize {
        for (column, base) in [0x90_usize, 0xE8, 0x140].iter().enumerate() {
            report_fpr(
                index as u32 + column as u32 * 0xB,
                read_f64_be(context + base + index * 8),
            );
            platform::call1(
                REPORT,
                if column == 2 { DUMP_GAP_B as u32 } else { DUMP_GAP_A as u32 },
            );
        }
    }
    report_fpr(10, read_f64_be(context + 0xE0));
    platform::call1(REPORT, DUMP_GAP_A as u32);
    report_fpr(21, read_f64_be(context + 0x138));
    platform::call1(REPORT, DUMP_GAP_B as u32);
}

/// Runs one script node: opens the script file if it is not buffered yet,
/// installs the matching byte reader, and parses the next command.
///
/// # Original
/// - Ghidra: `FUN_80235dc0` at `0x80235DC0` (232 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235dc0_FUN_80235dc0.c`
/// - `// observed:` the buffered path installs `0x80235820`; the file path
///   calls `DVDFastOpen(record[2], 0x803DEAC0)`, stores `-1` in the script
///   window, copies the file size from `0x803DEAF4`, installs `0x80235830`,
///   parses with `0x80000000` as the first argument and closes the file.
/// - `// inferred:` `r3` is the script record and `r4` the mode word. The
///   export reads both out of `FUN_802DCD74` (`_savegpr0_27`), which returns
///   nothing; its only caller is the parser dispatch, and `param_1`/`param_2`
///   are otherwise unused in the body.
///
/// # Arguments
/// - `record`: script record: `+0x00` buffered command, `+0x04` mode,
///   `+0x08` DVD entry number (`< 0` when not opened).
/// - `address`: `r4`; the parser's second argument. The console dump reaches
///   this routine through `0x80235EB0`, which passes the address it wants
///   symbolized here.
/// - `out`: `r5`; output buffer. Its first byte is cleared when nothing was
///   parsed; a node that recognises the address writes its name here.
/// - `size`: `r6`; capacity of `out` (0x104 in the dump path).
///
/// # Returns
/// The parser's result word (0 when nothing was parsed).
///
/// # Side effects
/// Opens and closes a DVD file and rewrites the script reader globals.
///
/// # Safety
/// `record` must be readable, `out` writable, and the DVD info block free.
///
/// # Confidence
/// medium - every call, store and branch is literal; the two lost arguments
/// are recovered from the prologue stub as described above.
pub unsafe fn fn_80235dc0(record: usize, address: u32, out: usize, buffer_size: u32) -> u32 {
    if platform::read_u32(record) == 0 {
        let entry = platform::read_u32(record + 0x08) as i32;
        if entry < 0 {
            platform::write_u8(out, 0);
            return 0;
        }
        let opened = platform::call2(DVD_FAST_OPEN, entry as u32, SCRIPT_FILE_INFO as u32);
        let file_size = platform::read_u32(SCRIPT_FILE_SIZE);
        if opened == 0 {
            platform::write_u8(out, 0);
            return 0;
        }
        platform::write_u32(SCRIPT_WINDOW_BASE, 0xFFFF_FFFF);
        platform::write_u32(SCRIPT_LENGTH, file_size);
        platform::write_u32(SCRIPT_READ_CHAR, SCRIPT_WINDOW_READ as u32);
        let result = platform::call5(
            SCRIPT_PARSE,
            0x8000_0000,
            platform::read_u32(record + 0x04),
            address,
            out as u32,
            buffer_size,
        );
        platform::call1(DVD_CLOSE, SCRIPT_FILE_INFO as u32);
        result
    } else {
        platform::write_u32(SCRIPT_READ_CHAR, SCRIPT_DIRECT_READ as u32);
        platform::call5(
            SCRIPT_PARSE,
            platform::read_u32(record),
            platform::read_u32(record + 0x04),
            address,
            out as u32,
            buffer_size,
        )
    }
}

/// Formats one message and queues it on the first script heap.
///
/// # Original
/// - Ghidra: `FUN_80236350` at `0x80236350` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236350_FUN_80236350.c`
/// - `// observed:` the temporary is allocated with direction `4` (or `-4`
///   when the caller passes 1), formatted with `0x802E4390`, handed to
///   `0x80236420` and freed.
/// - `// inferred:` the two words Ghidra takes from `FUN_802DCD68`
///   (`_savegpr0_24`) are the incoming `r3`/`r4`, i.e. the owner queue and the
///   tag; the sink stores exactly those two words in the record header.
///
/// # Arguments
/// - `owner`: queue object whose `+0x04` list receives the record.
/// - `tag`, `flag`, `direction`: stored verbatim in the record header.
/// - `fmt`: format string; `args`: matching `va_list`.
///
/// # Side effects
/// Allocates and frees on [`SCRIPT_HEAP_A`] and appends one record.
///
/// # Safety
/// `owner` must be a live queue and `fmt`/`args` a valid format call.
///
/// # Confidence
/// medium - the allocation, the formatter, the sink call and the free are
/// literal; the two lost arguments are recovered as described above.
pub unsafe fn fn_80236350(
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    fmt: u32,
    args: u32,
) {
    queue_message_format(
        SCRIPT_HEAP_A,
        owner,
        tag,
        flag,
        direction,
        fmt,
        args,
        FORMAT_INTO_BUFFER,
    );
}

/// Appends one prepared message record to the first script queue.
///
/// # Original
/// - Ghidra: `FUN_80236420` at `0x80236420` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236420_FUN_80236420.c`
/// - `// observed:` `HEAP_ALLOC(heap, len + 0x18, dir == 1 ? 4 : -4)`, four
///   header stores, `memcpy(record + 0x18, data, len)` and
///   `LIST_APPEND(owner + 4, record)`.
/// - `// inferred:` as [`fn_80236350`]: `r3` and `r4` are the owner and tag
///   that the prologue stub appears to return.
///
/// # Arguments
/// - `owner`, `tag`, `flag`, `direction`, `data`, `len`: see
///   [`queue_record_publish`].
///
/// # Side effects
/// Allocates on [`SCRIPT_HEAP_A`] and appends one record when the allocation
/// succeeds.
///
/// # Safety
/// `owner` must be a live queue; `data` readable for `len` bytes.
///
/// # Confidence
/// medium - the allocation and all five stores are literal; the two recovered
/// arguments are as described above.
pub unsafe fn fn_80236420(
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    data: usize,
    len: u32,
) {
    queue_record_publish(SCRIPT_HEAP_A, owner, tag, flag, direction, data, len);
}

/// Formats one message and queues it on the first script heap through
/// `snprintf`.
///
/// # Original
/// - Ghidra: `FUN_80236700` at `0x80236700` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236700_FUN_80236700.c`
/// - `// observed:` byte-for-byte [`fn_80236350`] except that the formatter is
///   `0x802E15DC` (bounded `snprintf`) instead of `0x802E4390`.
///
/// # Arguments
/// As [`fn_80236350`].
///
/// # Safety
/// As [`fn_80236350`].
///
/// # Confidence
/// medium - same reasoning as [`fn_80236350`]; the only difference is the
/// formatter address, which is literal.
pub unsafe fn fn_80236700(
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    fmt: u32,
    args: u32,
) {
    queue_message_format(
        SCRIPT_HEAP_A,
        owner,
        tag,
        flag,
        direction,
        fmt,
        args,
        SNPRINTF,
    );
}

/// Drains the first script queue: draws every pending line and unlinks the
/// records whose use count reaches zero.
///
/// # Original
/// - Ghidra: `FUN_80236880` at `0x80236880` (396 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236880_FUN_80236880.c`
/// - `// observed:` the 0x64-byte stream is constructed by `0x8022DDE0`, the
///   queued `+0x10` word is stored at the stream's `+0x18`, each item's
///   `+0x04`/`+0x00` words become the pen floats at `+0x30`/`+0x2C`, and the
///   payload at `+0x18` is run through `0x8022EED0`.
/// - `// observed:` `CONCAT44(0x43300000, word ^ 0x80000000)` minus
///   [`SDA2_QUEUE_COORD_BIAS`] is the `u32 -> f64` conversion.
/// - `// observed:` the two `unaff_GQR0` probes discard their result.
///
/// # Arguments
/// - `queue`: queue object: `+0x00` pending command, `+0x04` the `ut::List`,
///   `+0x14` a float threshold, `+0x18` the "draw coordinates" flag byte.
///
/// # Side effects
/// Draws through [`MSG_STREAM_EXECUTE`], decrements each record's count and
/// frees the exhausted records back to [`SCRIPT_HEAP_A`].
///
/// # Safety
/// `queue` must be a live initialised queue and every list entry a valid
/// record with a `len`-byte payload at `+0x18`.
///
/// # Confidence
/// medium - the construction, the two pen stores, the count decrement and the
/// unlink/free pair are literal; the callee roles are `// inferred:` from
/// their argument shapes and from the twin below.
pub unsafe fn fn_80236880(queue: usize) {
    let mut stream = crate::platform::GuestLocal::new( [0_u32; 0x19]);
    let base = stream.as_mut_ptr() as usize;
    platform::call1(MSG_STREAM_CTOR, base as u32);
    if platform::read_u32(queue) == 0 {
        platform::call2(MSG_STREAM_END, base as u32, 0xFFFF_FFFF);
        return;
    }

    platform::write_u32(base + 0x18, platform::read_u32(queue + 0x10));
    platform::call1(MSG_STREAM_BEGIN, base as u32);
    if platform::read_f32(SDA2_QUEUE_THRESHOLD) <= platform::read_u32(queue + 0x14) as i32 as f32 {
        platform::call1(MSG_STREAM_NARROW, base as u32);
    }
    platform::call1(MSG_STREAM_COMMIT, base as u32);

    let list = (queue + QUEUE_LIST_OFFSET) as u32;
    let mut item = platform::call2(LIST_NEXT, list, 0) as usize;
    while item != 0 {
        let next = platform::call2(LIST_NEXT, list, item as u32) as usize;
        if platform::read_u8(queue + 0x18) != 0 {
            platform::write_f32(base + 0x2C, platform::read_u32(item) as f32);
            platform::write_f32(base + 0x30, platform::read_u32(item + 4) as f32);
            platform::call3(
                MSG_STREAM_RUN,
                base as u32,
                (item + QUEUE_RECORD_PAYLOAD) as u32,
                platform::read_u32(item + QUEUE_RECORD_LENGTH),
            );
        }
        let remaining = platform::read_u32(item + QUEUE_RECORD_FLAG).wrapping_sub(1);
        platform::write_u32(item + QUEUE_RECORD_FLAG, remaining);
        if (remaining as i32) < 1 {
            platform::call2(LIST_REMOVE, list, item as u32);
            platform::call2(HEAP_FREE, platform::read_u32(SCRIPT_HEAP_A), item as u32);
        }
        item = next;
    }
    platform::call2(MSG_STREAM_END, base as u32, 0xFFFF_FFFF);
}

/// Formats one message and queues it on the second script heap.
///
/// # Original
/// - Ghidra: `FUN_802370e0` at `0x802370E0` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802370e0_FUN_802370e0.c`
/// - `// observed:` the queue-B twin of [`fn_80236350`]; it targets
///   [`SCRIPT_HEAP_B`] and hands the result to `0x802371B0`.
///
/// # Arguments
/// As [`fn_80236350`].
///
/// # Safety
/// As [`fn_80236350`].
///
/// # Confidence
/// medium - same reasoning as [`fn_80236350`].
pub unsafe fn fn_802370e0(
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    fmt: u32,
    args: u32,
) {
    queue_message_format(
        SCRIPT_HEAP_B,
        owner,
        tag,
        flag,
        direction,
        fmt,
        args,
        FORMAT_INTO_BUFFER,
    );
}

/// Appends one prepared message record to the second script queue.
///
/// # Original
/// - Ghidra: `FUN_802371b0` at `0x802371B0` (172 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802371b0_FUN_802371b0.c`
/// - `// observed:` byte-for-byte [`fn_80236420`] except that every heap
///   access uses [`SCRIPT_HEAP_B`].
///
/// # Arguments
/// As [`fn_80236420`].
///
/// # Safety
/// As [`fn_80236420`].
///
/// # Confidence
/// medium - same reasoning as [`fn_80236420`].
pub unsafe fn fn_802371b0(
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    data: usize,
    len: u32,
) {
    queue_record_publish(SCRIPT_HEAP_B, owner, tag, flag, direction, data, len);
}

/// Formats one message and queues it on the second script heap (second
/// formatter entry point).
///
/// # Original
/// - Ghidra: `FUN_80237490` at `0x80237490` (200 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237490_FUN_80237490.c`
/// - `// observed:` byte-for-byte [`fn_802370e0`].
///
/// # Arguments
/// As [`fn_80236350`].
///
/// # Safety
/// As [`fn_80236350`].
///
/// # Confidence
/// medium - same reasoning as [`fn_80236350`].
pub unsafe fn fn_80237490(
    owner: usize,
    tag: u32,
    flag: u32,
    direction: i32,
    fmt: u32,
    args: u32,
) {
    queue_message_format(
        SCRIPT_HEAP_B,
        owner,
        tag,
        flag,
        direction,
        fmt,
        args,
        FORMAT_INTO_BUFFER,
    );
}

/// Drains the second script queue.
///
/// # Original
/// - Ghidra: `FUN_80237610` at `0x80237610` (396 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237610_FUN_80237610.c`
/// - `// observed:` the queue-B twin of [`fn_80236880`]: the stream comes from
///   [`fn_80230780`], the text run goes through [`fn_80231870`], and exhausted
///   records are freed to [`SCRIPT_HEAP_B`].
/// - `// observed:` the two `unaff_GQR0` probes discard their result.
///
/// # Arguments
/// - `queue`: same layout as [`fn_80236880`].
///
/// # Side effects
/// Draws through [`fn_80231870`] and frees exhausted records.
///
/// # Safety
/// As [`fn_80236880`].
///
/// # Confidence
/// medium - every load, store and call is literal; the callee roles are
/// `// inferred:` from their argument shapes.
pub unsafe fn fn_80237610(queue: usize) {
    let mut stream = crate::platform::GuestLocal::new( [0_u32; 0x19]);
    let base = stream.as_mut_ptr() as usize;
    fn_80230780(base);
    if platform::read_u32(queue) == 0 {
        fn_802307e0(base, -1);
        return;
    }

    platform::write_u32(base + 0x18, platform::read_u32(queue + 0x10));
    platform::call1(MSG_STREAM_BEGIN, base as u32);
    if platform::read_f32(SDA2_QUEUE_THRESHOLD) <= platform::read_u32(queue + 0x14) as i32 as f32 {
        platform::call1(MSG_STREAM_NARROW, base as u32);
    }
    platform::call1(MSG_STREAM_COMMIT, base as u32);

    let list = (queue + QUEUE_LIST_OFFSET) as u32;
    let mut item = platform::call2(LIST_NEXT, list, 0) as usize;
    while item != 0 {
        let next = platform::call2(LIST_NEXT, list, item as u32) as usize;
        if platform::read_u8(queue + 0x18) != 0 {
            platform::write_f32(base + 0x2C, platform::read_u32(item) as f32);
            platform::write_f32(base + 0x30, platform::read_u32(item + 4) as f32);
            fn_80231870(
                base,
                item + QUEUE_RECORD_PAYLOAD,
                platform::read_u32(item + QUEUE_RECORD_LENGTH),
            );
        }
        let remaining = platform::read_u32(item + QUEUE_RECORD_FLAG).wrapping_sub(1);
        platform::write_u32(item + QUEUE_RECORD_FLAG, remaining);
        if (remaining as i32) < 1 {
            platform::call2(LIST_REMOVE, list, item as u32);
            platform::call2(HEAP_FREE, platform::read_u32(SCRIPT_HEAP_B), item as u32);
        }
        item = next;
    }
    fn_802307e0(base, -1);
}

/// Scales a double by a small-data factor, hands it to a G3D sink and returns
/// the second argument unchanged.
///
/// # Original
/// - Ghidra: `FUN_80238270` at `0x80238270` (56 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238270_FUN_80238270.c`
/// - `// observed:` `FUN_801C0B50((double)(float)(factor * param_1))` then
///   `return param_2`.
///
/// # Arguments
/// - `value`: the double to scale.
/// - `passthrough`: returned untouched.
///
/// # Returns
/// `passthrough`.
///
/// # Side effects
/// One call to `0x801C0B50`, which itself scales by a uniform and calls
/// `0x801C0AA0`.
///
/// # Safety
/// None beyond a valid target call.
///
/// # Confidence
/// low - the scale, the narrowing to `f32` and the pass-through return are
/// literal, but the sink takes three arguments and this caller sets only the
/// first; its `r4`/`r5` are `// unknown:` and are passed as zero here. The
/// factor's value is likewise only known by address.
pub unsafe fn fn_80238270(value: f64, passthrough: u32) -> u32 {
    let factor = platform::read_f32(SDA2_G3D_SCALE) as f64;
    let sink = crate::platform::guest_fn!(G3D_DOUBLE_SINK, fn(f64, u32, u32));
    sink((factor * value) as f32 as f64, 0, 0);
    passthrough
}

// ===========================================================================
// Wave 98: the last plainly reconstructible bodies of the 0x80230000 bucket -
// console text metrics, the console print sinks, and the quantised vector
// stores that waves 85, 90 and 95 left behind.
// ===========================================================================
//
// Wave 85, 90 and 95 refused every body whose arithmetic reads `unaff_GQR0`,
// `unaff_GQR3` or `unaff_GQR5`, on the grounds that the register is set by the
// caller and the export never shows its value. That objection applies to the
// *value* of the register, not to the shape of the code: every such body
// derives its arithmetic from `gqr & 7` (the lane format), the six-bit exponent
// at `(gqr >> 8) & 0x3F` (store direction) or `(gqr >> 24) & 0x3F` (load
// direction), and nothing else. The helpers below take the live GQR word as an
// as an explicit argument, so a caller supplies the value the original read
// from the register. They are a literal restatement of the repeated idiom:
//
//     scale = (gqr & 0x3F00) == 0 ? 1.0 : 2^((gqr >> 8) & 0x3F)
//     format 4 / 6 -> (char)(scale * value)    // 8-bit fixed point
//     format 5 / 7 -> (short)(scale * value)   // 16-bit fixed point
//     otherwise    -> the float itself         // raw single
//
// and the mirrored load form, which multiplies by `2^-exponent` instead. The
// `2^n` factor is the paired-single quantisation constant; Ghidra prints it as
// the one-argument `ldexpf(n)` because the original builds it by writing
// `127 + n` into the biased-exponent field of an `f32`, which is exactly what
// `gqr_exp2` does.
//
// Bodies that also *read back* a quantised value as a raw `u16` index (the
// angle/curve helpers at `0x80237A60..0x80237ED0`, `0x8023F000..0x8023FE50`)
// are still skipped: their 8-bit and raw-single branches read bytes the export
// leaves undefined, so only the 16-bit branch is coherent and reconstructing
// the others would mean inventing them. They are listed in
// `docs/COVERAGE_WAVE98_BRES.md`.

/// `r2 - 0x4BB8` - first of the two small-data floats broadcast into a whole
/// vector by [`fn_802380a0`] and [`fn_802384b0`].
const SDA2_VEC_FILL_A: usize = SDA2_BASE - 0x4BB8;
/// `r2 - 0x4BB4` - the second broadcast float.
const SDA2_VEC_FILL_B: usize = SDA2_BASE - 0x4BB4;
/// `0x803DA160` - 0x400-byte staging buffer of the console printer.
const CONSOLE_FORMAT_BUFFER: usize = 0x803D_A160;
/// `r13 - 0x5CDC` - console stream record handed to [`CONSOLE_SINK`].
const CONSOLE_SINK_STREAM: usize = SDA_BASE - 0x5CDC;
/// `0x8015F730` - print one already-formatted line to a console stream.
///
/// `// inferred:` it is the stream flavour of `printf`: the export calls it as
/// `(stream, text)` right after the formatter filled the staging buffer.
const CONSOLE_SINK: usize = 0x8015_F730;

/// `2^n` as an `f32`, the store-direction quantisation constant.
///
/// # Derivation
/// `// observed:` written as `ldexpf(n)` in the export. `n` is the six-bit
/// exponent field of the live GQR word, so only `0..=63` occurs; the shift
/// writes `127 + n` into the biased-exponent field and cannot overflow.
fn gqr_exp2(exp: u32) -> f32 {
    f32::from_bits((exp + 127) << 23)
}

/// `2^-n` as an `f32`, the load-direction quantisation constant.
///
/// # Derivation
/// `// observed:` written as `ldexpf(-n)` in the export; same six-bit field.
fn gqr_exp2_recip(exp: u32) -> f32 {
    f32::from_bits((127 - exp) << 23)
}

/// Stores one lane of a vector through the GQR store format.
///
/// # Arguments
/// - `dst`: address of the lane.
/// - `value`: the float to store.
/// - `gqr`: the live paired-single quantisation register word.
///
/// # Side effects
/// Writes 1, 2 or 4 bytes at `dst`.
///
/// # Safety
/// `dst` must be writable for four bytes.
///
/// # Confidence
/// medium - all three branches are the literal export branches; the multiply is
/// done in `f32` here while the original widens to `f64` first, which can move
/// a value that sits exactly on a truncation boundary by one unit.
unsafe fn psq_store(dst: usize, value: f32, gqr: u32) {
    let format = gqr & 7;
    let scaled = if gqr & 0x3F00 == 0 {
        value
    } else {
        gqr_exp2((gqr >> 8) & 0x3F) * value
    };
    match format {
        // observed: `(char)(scale * value)` - truncate toward zero, keep a byte.
        4 | 6 => platform::write_u8(dst, (scaled as i32 as i8) as u8),
        // observed: `(short)(scale * value)` - truncate toward zero, keep a half.
        5 | 7 => platform::write_u16(dst, (scaled as i32 as i16) as u16),
        // observed: the raw single goes out unchanged, unscaled.
        _ => platform::write_f32(dst, value),
    }
}

/// Loads one lane of a vector through the GQR load format.
///
/// # Arguments
/// - `src`: address of the lane.
/// - `gqr`: the live quantisation register word; its high half selects the
///   load format and exponent.
///
/// # Returns
/// The lane as a float, dequantised by `2^-exponent` in the fixed-point formats.
///
/// # Safety
/// `src` must be readable for four bytes.
///
/// # Confidence
/// medium - branches and scaling are literal; see [`psq_store`] for the `f32`
/// versus `f64` note.
unsafe fn psq_load(src: usize, gqr: u32) -> f32 {
    let format = (gqr >> 16) & 7;
    let scale = if gqr & 0x3F00_0000 == 0 {
        1.0
    } else {
        gqr_exp2_recip((gqr >> 24) & 0x3F)
    };
    match format {
        4 | 6 => (platform::read_u8(src) as i8) as f32 * scale,
        5 | 7 => (platform::read_u16(src) as i16) as f32 * scale,
        _ => platform::read_f32(src),
    }
}

/// Stores two adjacent lanes, the order the paired-single stores use.
///
/// # Safety
/// `dst` must be writable for eight bytes.
unsafe fn psq_store_pair(dst: usize, x: f32, y: f32, gqr: u32) {
    psq_store(dst, x, gqr);
    psq_store(dst + 4, y, gqr);
}

/// Measures the box of one formatted console-script line.
///
/// In plain English: "format the caller's arguments into the shared console
/// line buffer, measure that line on a snapshot of the stream, and hand the box
/// back through `rect`."
///
/// # Original
/// - Ghidra: `FUN_80230f50` at `0x80230F50` (384 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80230f50_FUN_80230f50.c`
/// - `// observed:` the buffer is `r13 - 0x4A28` when non-null, otherwise a
///   `capacity`-sized block carved below the declared locals; the write count
///   is clamped to `capacity - 1`; 0x64 bytes of the stream are copied, measured
///   with `0x802322C0`, then disposed with a zero count.
/// - `// inferred:` the snapshot pointer and the rect both arrive through
///   `FUN_802DCD44` (`_savegpr0_25`), which returns nothing; by the convention
///   recorded in `docs/COVERAGE_WAVE95_BRES.md` the high half of that "return"
///   is `r3` (the stream) and the low half is `r4` (the rect). The body then
///   reads exactly those two values as a stream and as [`fn_802322c0`]'s second
///   argument.
///
/// # Arguments
/// - `stream`: the console text stream to snapshot.
/// - `rect`: four writable floats that receive `{x0, y0, x1, y1}`.
/// - `fmt`: format string for the line.
/// - `args`: a `va_list`-shaped descriptor for `fmt`.
///
/// # Side effects
/// Writes the staging buffer, `rect` and the snapshot's pen words; calls the
/// formatter `0x802E4390`, [`fn_802322c0`] and the stream disposer
/// `0x8022C980`.
///
/// # Safety
/// `stream` must be a live 0x64-byte stream, `rect` writable for 16 bytes and
/// `args` a valid descriptor for `fmt`.
///
/// # Confidence
/// medium - the buffer choice, the clamp, the copy and the three calls are
/// literal; the two recovered arguments are `// inferred:` as described above.
/// This function has no direct caller in the image (it is reached through a
/// pointer), so the register reading rests on the stub convention alone.
pub unsafe fn fn_80230f50(stream: usize, rect: usize, fmt: u32, args: u32) {
    let mut fallback = crate::platform::GuestLocal::new( [0_u8; 0x800]);
    let capacity = platform::read_u32(CONSOLE_LINE_CAPACITY);
    let global = platform::read_u32(CONSOLE_STAGE_PTR);
    let buffer = if global != 0 {
        global
    } else {
        // partial: the original carves exactly `capacity` bytes below its
        // locals. Ghidra never shows that size, so this model clamps into a
        // fixed 0x800-byte local instead.
        fallback.as_mut_ptr() as u32
    };
    let written = platform::call4(FORMAT_INTO_BUFFER, buffer, capacity, fmt, args);
    let limit = capacity.wrapping_sub(1) as i32;
    let length = if limit < written as i32 {
        limit
    } else {
        written as i32
    };

    let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
    let base = snapshot.as_mut_ptr() as usize;
    for offset in 0..0x64 {
        platform::write_u8(base + offset, platform::read_u8(stream + offset));
    }
    fn_802322c0(base, rect, buffer, length);
    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
}

/// Measures the advance width of a run of text in a console stream.
///
/// In plain English: "snapshot the stream, measure the box of characters
/// `start..start + count` into a fresh rectangle and return its width."
///
/// # Original
/// - Ghidra: `FUN_802310d0` at `0x802310D0` (324 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802310d0_FUN_802310d0.c`
/// - `// observed:` the four local floats are all seeded from the small-data
///   word at `r2 - 0x4C34`, the 0x64-byte stream copy is measured with
///   `0x802322C0(copy, rect, start, count)`, the copy is disposed, and the
///   epilogue returns `rect[2] - rect[0]` widened to `f64`.
/// - `// inferred:` `r3` is the stream (high half of the `FUN_802DCD48`
///   "return"), `r4` is the start index and `r5` the count; the three direct
///   callers in `0x800E5AD0` and `0x800E6680` pass exactly
///   `(stream, index, count)` and use the result as a text height.
///
/// # Arguments
/// - `stream`: the console text stream to snapshot.
/// - `start`: first character index of the run.
/// - `count`: number of characters to cover (the layout test doubles it).
///
/// # Returns
/// `x1 - x0` of the measured box, as the original returns it in `f1`.
///
/// # Side effects
/// Copies 0x64 bytes, measures through [`fn_802322c0`], then calls the stream
/// disposer `0x8022C980` with a zero count.
///
/// # Safety
/// `stream` must be a live 0x64-byte stream.
///
/// # Confidence
/// medium - the seed, the copy, the measure and the subtraction are literal;
/// the argument roles are `// inferred:` from the three call sites.
pub unsafe fn fn_802310d0(stream: usize, start: u32, count: u32) -> f64 {
    let seed = platform::read_f32(SDA2_STREAM_WORD_B);
    let mut rect = crate::platform::GuestLocal::new( [seed; 4]);
    let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
    let base = snapshot.as_mut_ptr() as usize;
    for offset in 0..0x64 {
        platform::write_u8(base + offset, platform::read_u8(stream + offset));
    }
    fn_802322c0(base, rect.as_mut_ptr() as usize, start, count as i32);
    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
    (rect[2] - rect[0]) as f64
}

/// Prints one formatted line through the active console.
///
/// In plain English: "if a console is installed, format into a 0x100-byte local
/// and blit the text at `(x, y)`."
///
/// # Original
/// - Ghidra: `FUN_80233480` at `0x80233480` (228 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80233480_FUN_80233480.c`
/// - `// observed:` the gate is the word at `0x803DA120`; the body builds a
///   `va_list` descriptor `{0x04000000, stack+8, gpr_save}` from the eight
///   saved GPR words it prints as `param_9..param_16`, formats with
///   `0x802E15DC(buffer, 0x100, fmt, descriptor)` and, when the count is
///   positive, calls `0x80233570(x, y, buffer, wrap, 0)`.
/// - `// inferred:` the descriptor carries the "GPR only" type word
///   `0x04000000` - unlike the queue entry points that wave 95 skipped, no FPR
///   save area is involved - so a caller-built integer descriptor is a faithful
///   model. `param_9..param_12` are `r3..r6` (the saved words are copies of the
///   incoming registers, which is why Ghidra lists them twice); the sole caller
///   `0x80233EF0` passes `(x, y, 0, format, value)`.
///
/// # Arguments
/// - `x`, `y`: console pixel position of the line.
/// - `wrap`: forwarded to [`fn_80233570`]; nonzero restarts a long line.
/// - `fmt`: format string.
/// - `args`: a `va_list`-shaped descriptor for `fmt`, built by the caller.
///
/// # Side effects
/// Fills the local buffer and draws through [`fn_80233570`].
///
/// # Safety
/// A console must be installed, and `args` must describe a valid call to `fmt`.
///
/// # Confidence
/// low - the gate, the descriptor layout, the clamp-free format call and the
/// blit are literal, but the original's varargs are forwarded as a caller-built
/// descriptor: Rust cannot forward C varargs, so the descriptor construction
/// itself is not reproduced.
pub unsafe fn fn_80233480(x: u32, y: u32, wrap: u32, fmt: u32, args: u32) {
    if platform::read_u32(CONSOLE_CONFIG) != 0 {
        let mut buffer = crate::platform::GuestLocal::new( [0_u8; 0x100]);
        let written = platform::call4(
            SNPRINTF,
            buffer.as_mut_ptr() as u32,
            0x100,
            fmt,
            args,
        );
        if (written as i32) > 0 {
            fn_80233570(
                x as i32,
                y as i32,
                buffer.as_ptr() as usize,
                wrap,
                0,
            );
        }
    }
}

/// Formats into the shared staging buffer and prints it on a console target.
///
/// In plain English: "take the console lock, `vsnprintf` the line into the
/// 0x400-byte staging buffer, optionally echo it to the log stream and print it
/// on the target console, then unlock."
///
/// # Original
/// - Ghidra: `FUN_802344e0` at `0x802344E0` (216 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802344e0_FUN_802344e0.c`
/// - `// observed:` the lock path tests `countLeadingZeros(0x803DA150) >> 5`
///   when `0x801B4B30` says no threads are running, otherwise takes the mutex
///   at `0x803DA148`; the format call is
///   `0x802E15DC(0x803DA160, 0x400, format, descriptor)`; bit 1 of the mask
///   calls `0x8015F730(r13 - 0x5CDC, buffer)`, bit 0 calls
///   `0x80234120(console, buffer)`, and the unlock is `0x801B2F40`.
/// - `// inferred:` the mask and the console pointer are the two halves of the
///   `FUN_802DCD74` "return" (`r3`/`r4`); the sole caller `0x80234A80` passes
///   `(1, target, format, descriptor)`, and only the `0x80234120` call uses the
///   second word.
///
/// # Arguments
/// - `mask`: bit 0 prints on the target console, bit 1 echoes to the stream.
/// - `console`: the console record handed to [`fn_80234120`].
/// - `fmt`: format string.
/// - `args`: a `va_list`-shaped descriptor for `fmt`.
///
/// # Side effects
/// Takes and releases the console mutex, rewrites the staging buffer and prints.
///
/// # Safety
/// `console` must be a live console record and `args` must match `fmt`.
///
/// # Confidence
/// medium - the lock protocol, the mask bits and all four calls are literal;
/// the mask/console split across the recovered pair is `// inferred:` from the
/// one call site.
pub unsafe fn fn_802344e0(mask: u32, console: usize, fmt: u32, args: u32) {
    let locked = if call0_word(THREADS_RUNNING) == 0 {
        // observed: `countLeadingZeros(word) >> 5` is exactly `word == 0`.
        platform::read_u32(MUTEX_STATE) == 0
    } else {
        platform::call1(MUTEX_LOCK, CONSOLE_MUTEX as u32);
        true
    };
    if locked {
        platform::call4(SNPRINTF, CONSOLE_FORMAT_BUFFER as u32, 0x400, fmt, args);
        if mask & 0x2 != 0 {
            platform::call2(
                CONSOLE_SINK,
                CONSOLE_SINK_STREAM as u32,
                CONSOLE_FORMAT_BUFFER as u32,
            );
        }
        if mask & 0x1 != 0 {
            fn_80234120(console, CONSOLE_FORMAT_BUFFER);
        }
        if call0_word(THREADS_RUNNING) != 0 {
            platform::call1(MUTEX_UNLOCK, CONSOLE_MUTEX as u32);
        }
    }
}

/// Writes one fixed 2-D vector with the caller's quantisation.
///
/// In plain English: "broadcast two shared floats into five alternating
/// positions of a 6-vector and leave the sixth as a raw float."
///
/// # Original
/// - Ghidra: `FUN_802380a0` at `0x802380A0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802380a0_FUN_802380a0.c`
/// - `// observed:` the two sources are the small-data floats at `r2 - 0x4BB8`
///   and `r2 - 0x4BB4`; the stores cover lanes 0..9 and the last store
///   (`param_1[8]`) is a plain `f32` write that ignores the GQR.
///
/// # Arguments
/// - `vector`: 10-float destination.
/// - `gqr`: the live quantisation register word (see [`psq_store`]).
///
/// # Side effects
/// Writes 0x24 bytes at `vector`.
///
/// # Safety
/// `vector` must be writable for 0x24 bytes.
///
/// # Confidence
/// medium - every lane value and store offset is literal; only the GQR word is
/// an added argument, and the two broadcast floats are known by address only.
pub unsafe fn fn_802380a0(vector: usize, gqr: u32) {
    let a = platform::read_f32(SDA2_VEC_FILL_A);
    let b = platform::read_f32(SDA2_VEC_FILL_B);
    psq_store_pair(vector + 0x08, a, a, gqr);
    psq_store_pair(vector + 0x18, a, a, gqr);
    psq_store_pair(vector + 0x00, b, a, gqr);
    psq_store_pair(vector + 0x10, b, a, gqr);
    platform::write_f32(vector + 0x20, b);
}

/// Repacks three strided 3-vectors into nine contiguous lanes.
///
/// In plain English: "read three `(x, y, z)` triples that sit 16 bytes apart and
/// write them back to back."
///
/// # Original
/// - Ghidra: `FUN_802380d0` at `0x802380D0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802380d0_FUN_802380d0.c`
/// - `// observed:` the loads use the GQR load format on `src + 0x00`, `+0x04`,
///   `+0x08`, `+0x10`, `+0x14`, `+0x18`, `+0x20`, `+0x24`, `+0x28` and the
///   stores are the nine contiguous lanes `dst + 0x00..0x20`. The skipped words
///   `src + 0x0C`, `+0x1C`, `+0x2C` are the record padding.
///
/// # Arguments
/// - `dst`: nine-float destination.
/// - `src`: three 16-byte records.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x24 bytes at `dst`.
///
/// # Safety
/// `dst` writable for 0x24 bytes, `src` readable for 0x30 bytes.
///
/// # Confidence
/// medium - the nine loads and nine stores are literal; the "three 16-byte
/// records" reading is `// inferred:` from the constant stride.
pub unsafe fn fn_802380d0(dst: usize, src: usize, gqr: u32) {
    let mut lanes = [0.0_f32; 9];
    for record in 0..3 {
        let base = record * 0x10;
        lanes[record * 3] = psq_load(src + base, gqr);
        lanes[record * 3 + 1] = psq_load(src + base + 4, gqr);
        lanes[record * 3 + 2] = psq_load(src + base + 8, gqr);
    }
    for (index, value) in lanes.iter().enumerate() {
        psq_store(dst + index * 4, *value, gqr);
    }
}

/// Fills twelve lanes with the same shared float.
///
/// # Original
/// - Ghidra: `FUN_80238110` at `0x80238110` (32 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238110_FUN_80238110.c`
/// - `// observed:` six paired stores, all from the small-data float at
///   `r2 - 0x4BB8`, covering `param_1[0..0xB]`.
///
/// # Arguments
/// - `vector`: twelve-float destination.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x30 bytes at `vector`.
///
/// # Safety
/// `vector` must be writable for 0x30 bytes.
///
/// # Confidence
/// medium - the six stores and their source are literal; the source value is
/// known by address only.
pub unsafe fn fn_80238110(vector: usize, gqr: u32) {
    let fill = platform::read_f32(SDA2_VEC_FILL_A);
    for offset in (0..0x30).step_by(8) {
        psq_store_pair(vector + offset, fill, fill, gqr);
    }
}

/// Adds two twelve-lane vectors.
///
/// # Original
/// - Ghidra: `FUN_80238130` at `0x80238130` (100 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238130_FUN_80238130.c`
/// - `// observed:` six paired loads from each source, the lane-wise sum, and
///   six paired stores; both directions use the same GQR word, so the sum runs
///   in the dequantised domain and is re-quantised on the way out.
///
/// # Arguments
/// - `dst`: twelve-float destination.
/// - `a`, `b`: twelve-float sources.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x30 bytes at `dst`.
///
/// # Safety
/// `dst` writable for 0x30 bytes; `a` and `b` readable for 0x30 bytes.
///
/// # Confidence
/// medium - all eighteen operations are literal; the `f32`/`f64` truncation
/// note of [`psq_store`] applies.
pub unsafe fn fn_80238130(dst: usize, a: usize, b: usize, gqr: u32) {
    for lane in 0..12 {
        let offset = lane * 4;
        let sum = psq_load(a + offset, gqr) + psq_load(b + offset, gqr);
        psq_store(dst + offset, sum, gqr);
    }
}

/// Scales the three columns of a 3x4 matrix by a 3-vector.
///
/// In plain English: "multiply column `i` by `b[i]` and leave the fourth lane of
/// each column alone."
///
/// # Original
/// - Ghidra: `FUN_802381a0` at `0x802381A0` (84 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802381a0_FUN_802381a0.c`
/// - `// observed:` three sources are read from `param_2`: the pair at `+0x00`,
///   the single at `+0x08`, and the pair at `+0x10`; the stores are the six
///   pairs at `param_1 + 0x00..0x28`. The lanes multiplied by the single at
///   `+0x10` are followed by `* 1.0`, which is the reason the fourth lane of
///   each column comes out unchanged.
///
/// # Arguments
/// - `dst`: twelve-float 3x4 destination.
/// - `matrix`: twelve-float 3x4 source.
/// - `scale`: three floats `(sx, sy, sz)`.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x30 bytes at `dst`.
///
/// # Safety
/// `dst` writable for 0x30 bytes; `matrix` and `scale` readable for 0x30 and
/// 0x0C bytes.
///
/// # Confidence
/// medium - every product and store is literal; the "3x4 columns" wording is
/// `// inferred:` from the 16-byte grouping the stores fall into.
pub unsafe fn fn_802381a0(dst: usize, matrix: usize, scale: usize, gqr: u32) {
    let sx = psq_load(scale, gqr);
    let sy = psq_load(scale + 4, gqr);
    let sz = psq_load(scale + 8, gqr);
    for column in 0..3 {
        let base = column * 0x10;
        psq_store(dst + base, psq_load(matrix + base, gqr) * sx, gqr);
        psq_store(dst + base + 4, psq_load(matrix + base + 4, gqr) * sy, gqr);
        psq_store(dst + base + 8, psq_load(matrix + base + 8, gqr) * sz, gqr);
        // observed: `* 1.0` in the export - the fourth lane is passed through.
        psq_store(
            dst + base + 12,
            psq_load(matrix + base + 12, gqr),
            gqr,
        );
    }
}

/// Fills sixteen lanes with a pattern of two shared floats.
///
/// # Original
/// - Ghidra: `FUN_802384b0` at `0x802384B0` (52 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802384b0_FUN_802384b0.c`
/// - `// observed:` eight paired stores over `param_1[0..0xF]`, all drawn from
///   the small-data floats at `r2 - 0x4BB8` (first lane) and `r2 - 0x4BB4`
///   (second lane).
///
/// # Arguments
/// - `matrix`: sixteen-float destination.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x40 bytes at `matrix`.
///
/// # Safety
/// `matrix` must be writable for 0x40 bytes.
///
/// # Confidence
/// medium - the eight pairs and their order are literal; the two source values
/// are known by address only.
pub unsafe fn fn_802384b0(matrix: usize, gqr: u32) {
    let a = platform::read_f32(SDA2_VEC_FILL_A);
    let b = platform::read_f32(SDA2_VEC_FILL_B);
    psq_store_pair(matrix + 0x08, a, a, gqr);
    psq_store_pair(matrix + 0x18, a, a, gqr);
    psq_store_pair(matrix + 0x20, a, a, gqr);
    psq_store_pair(matrix + 0x10, a, b, gqr);
    psq_store_pair(matrix + 0x00, b, a, gqr);
    psq_store_pair(matrix + 0x28, b, a, gqr);
    psq_store_pair(matrix + 0x30, a, a, gqr);
    psq_store_pair(matrix + 0x38, a, b, gqr);
}

/// Copies a sixteen-lane packed matrix.
///
/// # Original
/// - Ghidra: `FUN_802384f0` at `0x802384F0` (68 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802384f0_FUN_802384f0.c`
/// - `// observed:` eight paired loads from `param_2`, then eight paired stores
///   to `param_1`; every load and store uses the same GQR word, so the copy
///   dequantises and re-quantises each lane.
///
/// # Arguments
/// - `dst`: sixteen-float matrix.
/// - `src`: sixteen-float matrix.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x40 bytes at `dst`.
///
/// # Safety
/// Both matrices must be readable/writable for 0x40 bytes; the model loads all
/// lanes before storing, so `dst` may equal `src`.
///
/// # Confidence
/// medium - the sixteen loads and sixteen stores are literal.
pub unsafe fn fn_802384f0(dst: usize, src: usize, gqr: u32) {
    let mut lanes = [0.0_f32; 16];
    for (lane, slot) in lanes.iter_mut().enumerate() {
        *slot = psq_load(src + lane * 4, gqr);
    }
    for (lane, value) in lanes.iter().enumerate() {
        psq_store(dst + lane * 4, *value, gqr);
    }
}

// ===========================================================================
// Wave 100: the paired-single read-back group, the glyph blitter, the report
// sink, the queue entry points and the remaining key samplers
// ===========================================================================

/// `r2 - 0x4BE0` - period used by the angle reducers (`// inferred:` 2*pi or
/// the equivalent fixed-point turn; the export only shows the address).
const SDA2_TRIG_PERIOD: usize = SDA2_BASE - 0x4BE0;
/// `r2 - 0x4BDC` - `f32` 0.0, the sign-flip threshold of the reducers.
const SDA2_TRIG_SIGN_ZERO: usize = SDA2_BASE - 0x4BDC;
/// `r2 - 0x4BD8` - scale applied to the reduced ratio before interpolation.
const SDA2_TRIG_RATIO: usize = SDA2_BASE - 0x4BD8;
/// `r2 - 0x4BD4` - quadrant constant for the third octant.
const SDA2_TRIG_QUADRANT_C: usize = SDA2_BASE - 0x4BD4;
/// `r2 - 0x4BD0` - quadrant constant for the first octant.
const SDA2_TRIG_QUADRANT_A: usize = SDA2_BASE - 0x4BD0;
/// `r2 - 0x4BCC` - quadrant constant for the second octant.
const SDA2_TRIG_QUADRANT_B: usize = SDA2_BASE - 0x4BCC;
/// `r2 - 0x4BC8` - quadrant constant for the fourth octant.
const SDA2_TRIG_QUADRANT_D: usize = SDA2_BASE - 0x4BC8;
/// `r2 - 0x4BC4` - quadrant constant of the packed-angle variant.
const SDA2_TRIG_QUADRANT_E: usize = SDA2_BASE - 0x4BC4;
/// `r2 - 0x4BC0` - full-turn scale of the packed-angle variant.
const SDA2_TRIG_TURN: usize = SDA2_BASE - 0x4BC0;
/// `r2 - 0x4BF4` - log2 mantissa scale (`// inferred:` 1.0).
const SDA2_LOG2_SCALE: usize = SDA2_BASE - 0x4BF4;
/// `r2 - 0x4BF0` - log2 mantissa bias (`// inferred:` 1.0).
const SDA2_LOG2_BIAS: usize = SDA2_BASE - 0x4BF0;
/// `r2 - 0x4BFC` - log2 exponent weight (`// inferred:` the bit conversion).
const SDA2_LOG2_WEIGHT: usize = SDA2_BASE - 0x4BFC;

/// `r2 - 0x4B28` - `f32` 1.0, the Hermite basis offset.
const SDA2_SPLINE_ONE: usize = SDA2_BASE - 0x4B28;
/// `r2 - 0x4B24` - `f32` 2.0, the quadratic coefficient.
const SDA2_SPLINE_TWO: usize = SDA2_BASE - 0x4B24;
/// `r2 - 0x4B20` - `f32` 3.0, the cubic coefficient.
const SDA2_SPLINE_THREE: usize = SDA2_BASE - 0x4B20;
/// `r2 - 0x4B1C` - sentinel returned by the split helper when it fails.
const SDA2_ANM_SENTINEL: usize = SDA2_BASE - 0x4B1C;
/// `r2 - 0x4B18` - byte-lerp fraction scale.
const SDA2_ANM_BYTE_SCALE: usize = SDA2_BASE - 0x4B18;

/// `r2 - 0x4B9C` - `f32` 0.0, the zero/epsilon word of the G3D math band.
const SDA2_MATH_ZERO: usize = SDA2_BASE - 0x4B9C;
/// `r2 - 0x4BA0` - `f32` 0.5, the root-scale factor of the ray kernel.
const SDA2_MATH_HALF: usize = SDA2_BASE - 0x4BA0;
/// `r2 - 0x4B90` - `f32` 1.0, reciprocal numerator of the ray kernel.
const SDA2_MATH_RECIP: usize = SDA2_BASE - 0x4B90;
/// `r2 - 0x4B80` - `f32` 2.0, the quadratic coefficient scale.
const SDA2_MATH_TWO: usize = SDA2_BASE - 0x4B80;
/// `r2 - 0x4B7C` - `f32` 4.0, the discriminant scale.
const SDA2_MATH_FOUR: usize = SDA2_BASE - 0x4B7C;
/// `r2 - 0x4B78` - `f32` -0.5, the tangent-root scale.
const SDA2_MATH_NEG_HALF: usize = SDA2_BASE - 0x4B78;
/// `0x802F74A0` - first float of the 256-entry angle table used by the
/// reducers (`// inferred:` a sine/cosine pair per low byte).
const TRIG_TABLE_SIN: usize = 0x802F_74A0;
/// `0x802F74A4` - the companion column of [`TRIG_TABLE_SIN`].
const TRIG_TABLE_COS: usize = 0x802F_74A4;
/// `0x8034D640` - two-float log2 interpolation table.
const LOG2_TABLE: usize = 0x8034_D640;
/// `0x8034DE48` - two-float arctangent interpolation table.
const ATAN_TABLE: usize = 0x8034_DE48;

/// `0x8034CF30` - 4-byte-to-nibble expansion table of the glyph blitter.
const GLYPH_EXPAND: usize = 0x8034_CF30;
/// `0x802F7268` - 5x7 glyph records for codes below 100.
const GLYPH_LOW: usize = 0x802F_7268;
/// `0x802F7368` - 5x7 glyph records for codes 100 and above.
const GLYPH_HIGH: usize = 0x802F_7368;

/// `0x803DA134` - glyph background/colour word.
const CONSOLE_COLOUR_BG: usize = 0x803D_A134;
/// `0x803DA138` - glyph colour word 0.
const CONSOLE_COLOUR_0: usize = 0x803D_A138;
/// `0x803DA13A` - glyph colour word 1.
const CONSOLE_COLOUR_1: usize = 0x803D_A13A;
/// `0x803DA13E` - glyph colour word 2.
const CONSOLE_COLOUR_2: usize = 0x803D_A13E;
/// `0x803DA140` - glyph colour word 3.
const CONSOLE_COLOUR_3: usize = 0x803D_A140;
/// `0x803DA89C` - cached console line height.
const CONSOLE_LINE_HEIGHT: usize = 0x803D_A89C;
/// `0x803DA898` - cached console origin.
const CONSOLE_ORIGIN: usize = 0x803D_A898;
/// `0x803DA8A0` - active console record.
const CONSOLE_ACTIVE: usize = 0x803D_A8A0;
/// `0x803DA8A4` - console configuration record.
const CONSOLE_RECORD: usize = 0x803D_A8A4;
/// `0x803DA8A8` - installed frame callback.
const CONSOLE_CALLBACK: usize = 0x803D_A8A8;
/// `0x803DA8AC` - frame-callback context word.
const CONSOLE_CALLBACK_CTX: usize = 0x803D_A8AC;
/// `0x803DA8B8` - frame tick counter tested by the frame loop.
const CONSOLE_TICK: usize = 0x803D_A8B8;

/// `r13 - 0x49F0` - first console heap handle.
const CONSOLE_HEAP_A: usize = SDA_BASE - 0x49F0;
/// `r13 - 0x49E8` - second console heap handle.
const CONSOLE_HEAP_B: usize = SDA_BASE - 0x49E8;

/// `0x801C1530` - 3-D cross product `(a, b, out)`.
const G3D_CROSS3: usize = 0x801C_1530;
/// `0x801C1470` - 3-D normalise `(in, out)`.
const G3D_NORMALIZE3: usize = 0x801C_1470;
/// `0x802E8590` - the Metrowerks `tan` (MSL `__kernel_tan` is `0x802E79C4`).
///
/// `// observed:` [`fn_80238930`] passes the half angle in radians and reads the
/// result from `f1`; `engine::main_loop::fn_802e8590` reconstructs the same body
/// as `tan` (`|x| < pi/4` uses the kernel directly, otherwise the `pi/2`
/// reduction flips odd quadrants).
const G3D_TAN: usize = 0x802E_8590;
/// `0x802389F0` - the beam/billboard matrix builder [`fn_80238930`] calls.
///
/// It is reconstructed as [`fn_802389f0`] (wave 115). Its export declares
/// `(undefined8, double x5)`, the two pointer words plus five doubles
/// [`call_matrix_builder`] forwards; the sixth double at `+0xB8` is the one
/// that helper drops.
const G3D_BILLBOARD: usize = 0x8023_89F0;

/// `0x801D4860` - heap block-size probe.
const HEAP_SIZE: usize = 0x801D_4860;
/// `0x801D49A0` - heap bookkeeping reset.
const HEAP_RESET: usize = 0x801D_49A0;
/// `0x802E15DC` - wide-character formatter.
const FORMAT_WIDE: usize = 0x802E_15DC;
/// `0x802E4390` - narrow-character formatter.
const FORMAT_NARROW: usize = 0x802E_4390;
/// `0x80236420` - queue record sink for heap A.
const QUEUE_SINK_A: usize = 0x8023_6420;
/// `0x802371B0` - queue record sink for heap B.
const QUEUE_SINK_B: usize = 0x8023_71B0;

/// `0x8015F6E0` - low-level report thunk used by the exception sink.
const REPORT_THUNK: usize = 0x8015_F6E0;
/// `0x801BDE40` - console timestamp/sequence source.
const CONSOLE_TIMESTAMP: usize = 0x801B_DE40;
/// `0x80233420` - console line counter.
const CONSOLE_LINE_COUNTER: usize = 0x8023_3420;

/// Returns the load-direction scale of a paired-single word.
fn gqr_load_scale(gqr: u32) -> f32 {
    if gqr & 0x3F00_0000 == 0 {
        1.0
    } else {
        gqr_exp2_recip((gqr >> 24) & 0x3F)
    }
}

/// Quantises a float through the low half of `gqr`, then dequantises it through
/// the high half, using a four-byte scratch slot exactly like the original
/// `psq_st`/`psq_l` pair.
///
/// # Returns
/// `(raw_word_left_by_the_store, value_read_back)`.
///
/// # Safety
/// Pure computation; no target memory is touched.
///
/// # Confidence
/// medium - the two halves of the GQR word are exactly the store and load
/// descriptors the export uses. When the store format is narrower than the load
/// format the original would read bytes it never wrote; those bytes are zero
/// here instead of whatever the caller's stack held.
unsafe fn psq_round(value: f32, gqr: u32) -> (u32, f32) {
    let mut scratch = crate::platform::GuestLocal::new( [0_u8; 4]);
    let base = scratch.as_mut_ptr() as usize;
    psq_store(base, value, gqr);
    let bits = platform::read_u32(base);
    (bits, psq_load(base, gqr))
}

/// Extracts the integer lane a paired store leaves in the scratch word.
fn psq_lane(bits: u32, gqr: u32) -> u32 {
    match gqr & 7 {
        // observed: the 8-bit store writes the first byte of the slot.
        4 | 6 => bits >> 24,
        // observed: the 16-bit store writes the first half of the slot.
        5 | 7 => bits >> 16,
        // observed: the raw store leaves the whole word.
        _ => bits,
    }
}

/// Dequantises an integer lane that the original already placed in memory.
///
/// # Arguments
/// - `value`: the signed 16-bit lane.
/// - `gqr`: the word whose *high* half supplies format and scale.
///
/// # Confidence
/// medium - matches the export's `local_8._0_2_` reads; the 8-bit branch uses
/// the high byte, exactly like Ghidra's `cStack_16`.
fn psq_load_i16(value: i16, gqr: u32) -> f32 {
    match (gqr >> 16) & 7 {
        4 | 6 => ((value >> 8) as i8) as f32 * gqr_load_scale(gqr),
        5 | 7 => value as f32 * gqr_load_scale(gqr),
        _ => f32::from_bits((value as u16 as u32) << 16),
    }
}

/// Square root of a positive double, built from exponent halving and Newton
/// refinement.
///
/// # Why it is not `sqrt`
/// The crate is `#![no_std]` and has no libm. The original emits the PowerPC
/// `fsqrt` instruction; this helper converges to the same value from a
/// half-exponent seed. Five Newton steps are enough to take a ~1% seed to full
/// double precision, but the last bit is not rounded exactly like `fsqrt`.
///
/// # Confidence
/// low - a one-unit difference from the hardware result is possible.
fn sqrt64(x: f64) -> f64 {
    if !(x > 0.0) {
        return 0.0;
    }
    // observed: exponent halving is the standard seed `sqrt(x) ~ 2^(e/2)`.
    let mut y = f64::from_bits((x.to_bits() >> 1) + 0x1FF8_0000_0000_0000);
    for _ in 0..5 {
        y = 0.5 * (y + x / y);
    }
    y
}

/// Refines an approximate reciprocal square root.
///
/// In plain English: "one Newton correction of `1/sqrt(x)`, scaled by two
/// small-data words."
///
/// # Original
/// - Ghidra: `FUN_80237ba0` at `0x80237BA0` (36 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237ba0_FUN_80237ba0.c`
/// - `// observed:` `dVar1 = 1.0 / SQRT(param_1)`; the result is
///   `-((float)((float)(dVar1*dVar1)*param_1 - sda1) * (float)(dVar1 * sda2))`
///   with `sda1 = r2 - 0x4BE8` and `sda2 = r2 - 0x4BEC`.
///
/// # Arguments
/// - `x`: the positive input.
///
/// # Returns
/// The corrected reciprocal square root, widened to `f64`.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// low - every step is literal, but [`sqrt64`] replaces the hardware `fsqrt`
/// and can differ in the last bit.
pub unsafe fn fn_80237ba0(x: f64) -> f64 {
    let inv = 1.0 / sqrt64(x);
    let residual = ((inv * inv) as f32 as f64 * x - platform::read_f32(SDA2_BASE - 0x4BE8) as f64) as f32;
    let scale = (inv * platform::read_f32(SDA2_BASE - 0x4BEC) as f64) as f32;
    -(residual * scale) as f64
}

/// Reduces `|x|` by the period, then interpolates the low-byte angle table.
///
/// In plain English: "wrap the angle into one turn and read the sine-like value
/// from the 256-entry table at `0x802F74A0`."
///
/// # Original
/// - Ghidra: `FUN_80237BD0` at `0x80237BD0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237bd0_FUN_80237bd0.c`
/// - `// observed:` the reduction is `while (period < v) v = (float)(v - period)`;
///   the table index is `(stored_half & 0xFF) * 0x10`; the interpolation is
///   `table[0x802F74A0 + i] + (v - back) * table[0x802F74A8 + i]`; a negative
///   input negates the result.
///
/// # Arguments
/// - `x`: the angle, in the same fixed-point scale as the period word.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The interpolated table value.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// medium - the reduction, index mask, table bases and sign test are literal.
/// The export's 8-bit scratch read is incoherent; this model uses the lane the
/// store actually wrote.
pub unsafe fn fn_80237bd0(x: f64, gqr: u32) -> f64 {
    let period = platform::read_f32(SDA2_TRIG_PERIOD) as f64;
    let sign_zero = platform::read_f32(SDA2_TRIG_SIGN_ZERO) as f64;
    let mut v = x.abs();
    while period < v {
        v = (v - period) as f32 as f64;
    }
    let (bits, back) = psq_round(v as f32, gqr);
    let index = (psq_lane(bits, gqr) & 0xFF) as usize * 0x10;
    let first = platform::read_f32(TRIG_TABLE_SIN + index);
    let second = platform::read_f32(TRIG_TABLE_SIN + 8 + index);
    let mut out = first + (v - back as f64) as f32 * second;
    if x < sign_zero {
        out = -out;
    }
    out as f64
}

/// Reduces `|x|` by the period, then interpolates the cosine-like column.
///
/// In plain English: "same reducer as [`fn_80237bd0`], but the table pair starts
/// four bytes later and the sign is not folded."
///
/// # Original
/// - Ghidra: `FUN_80237C40` at `0x80237C40` (96 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237c40_FUN_80237c40.c`
/// - `// observed:` the table bases are `0x802F74A4` and `0x802F74AC`; the
///   reducer overwrites its argument with `ABS(param_1)` and there is no sign
///   branch.
///
/// # Arguments
/// - `x`: the angle.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The interpolated table value.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// medium - literal except for the same 8-bit scratch note as
/// [`fn_80237bd0`].
pub unsafe fn fn_80237c40(x: f64, gqr: u32) -> f64 {
    let period = platform::read_f32(SDA2_TRIG_PERIOD) as f64;
    let mut v = x.abs();
    while period < v {
        v = (v - period) as f32 as f64;
    }
    let (bits, back) = psq_round(v as f32, gqr);
    let index = (psq_lane(bits, gqr) & 0xFF) as usize * 0x10;
    let first = platform::read_f32(TRIG_TABLE_COS + index);
    let second = platform::read_f32(TRIG_TABLE_COS + 8 + index);
    (first + (v - back as f64) as f32 * second) as f64
}

/// Evaluates a sine/cosine pair into two caller-supplied quantised outputs.
///
/// In plain English: "wrap the angle, look up the sine/cosine table through the
/// packed load format, interpolate both columns and store them with the packed
/// store format."
///
/// # Original
/// - Ghidra: `FUN_80237CA0` at `0x80237CA0` (112 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237ca0_FUN_80237ca0.c`
/// - `// observed:` the reduced value is stored to `param_2` with the GQR3 store
///   format and read back to form `(*(u16 *)param_2 & 0xFF) * 0x10`; the six
///   table floats are read with the GQR0 load format from `0x802F74A0..+0xC`;
///   the two results are stored to `param_3` and `param_2` with the GQR0 store
///   format; a negative input negates the first result.
///
/// # Arguments
/// - `angle`: the input angle.
/// - `out_a`: four-byte slot that receives the first result.
/// - `out_b`: four-byte slot that receives the second result.
/// - `gqr3`: quantisation word of the scratch/index conversion.
/// - `gqr0`: quantisation word of the table and the outputs.
///
/// # Side effects
/// Writes `out_a` and `out_b`.
///
/// # Safety
/// Both output slots must be writable for four bytes.
///
/// # Confidence
/// medium - every store, table read and branch is literal; the 8-bit lane note
/// of [`fn_80237bd0`] applies to the index read.
pub unsafe fn fn_80237ca0(angle: f64, out_a: usize, out_b: usize, gqr3: u32, gqr0: u32) {
    let period = platform::read_f32(SDA2_TRIG_PERIOD) as f64;
    let mut v = angle.abs();
    psq_store(out_a, v as f32, gqr3);
    if period < v {
        loop {
            let reduced = v - period;
            v = reduced as f32 as f64;
            if !(period < v) {
                break;
            }
        }
        psq_store(out_a, v as f32, gqr3);
    }
    let index = (platform::read_u16(out_a) & 0xFF) as usize * 0x10;
    let back = psq_load(out_a, gqr3) as f64;
    let first = psq_load(TRIG_TABLE_SIN + index, gqr0) as f64;
    let second = psq_load(TRIG_TABLE_SIN + 4 + index, gqr0) as f64;
    let slope_a = psq_load(TRIG_TABLE_SIN + 8 + index, gqr0) as f64;
    let slope_b = psq_load(TRIG_TABLE_SIN + 12 + index, gqr0) as f64;
    let delta = (v - back) as f32 as f64;
    let result_a = slope_a * delta + first;
    let result_b = slope_b * delta + second;
    psq_store(out_b, result_b as f32, gqr0);
    let signed_a = if angle < 0.0 { -result_a } else { result_a };
    psq_store(out_a, signed_a as f32, gqr0);
}

/// Octant-reduced arctangent of `(y, x)`.
///
/// In plain English: "fold the two coordinates into the first octant, take the
/// ratio, quantise it, interpolate the arctangent table and add the selected
/// quadrant constant."
///
/// # Original
/// - Ghidra: `FUN_80237D10` at `0x80237D10` (440 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237d10_FUN_80237d10.c`
/// - `// observed:` the four quadrant constants are `r2 - 0x4BD0`, `-0x4BCC`,
///   `-0x4BD4` and `-0x4BC8`; the ratio is scaled by `r2 - 0x4BD8`; the table
///   pair is `0x8034DE48`/`+4` indexed by `lane * 8`; the flag selects addition
///   or subtraction.
///
/// # Arguments
/// - `y`: first coordinate.
/// - `x`: second coordinate.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The angle, in the same scale as the quadrant constants.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// medium - the fold, the constants, the table and the sign are literal; the
/// only inference is that the two arguments are `(y, x)` rather than `(x, y)`,
/// which the fold order implies.
pub unsafe fn fn_80237d10(y: f64, x: f64, gqr: u32) -> f64 {
    let zero = platform::read_f32(SDA2_TRIG_SIGN_ZERO) as f64;
    if zero == x && zero == y {
        return zero;
    }
    let mut p1 = y;
    let mut p2 = x;
    let mut corner = zero;
    let small;
    let flag;
    if p2 < zero {
        if p1 < zero {
            p2 = -p2;
            p1 = -p1;
            if p2 < p1 {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_A) as f64;
                flag = true;
                small = p2;
            } else {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_D) as f64;
                flag = false;
                small = p1;
                p1 = p2;
            }
        } else {
            p2 = -p2;
            if p2 < p1 {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_C) as f64;
                flag = false;
                small = p2;
            } else {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_B) as f64;
                flag = true;
                small = p1;
                p1 = p2;
            }
        }
    } else {
        small = p2;
        if p1 < zero {
            p1 = -p1;
            if p2 < p1 {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_A) as f64;
                flag = false;
            } else {
                flag = true;
                p1 = p2;
            }
        } else if p2 < p1 {
            corner = platform::read_f32(SDA2_TRIG_QUADRANT_C) as f64;
            flag = true;
        } else {
            flag = false;
            p1 = p2;
        }
    }
    let ratio = ((small / p1) as f32) * platform::read_f32(SDA2_TRIG_RATIO);
    let (bits, back) = psq_round(ratio, gqr);
    let index = psq_lane(bits, gqr) as usize * 8;
    let interp = platform::read_f32(ATAN_TABLE + index)
        + (ratio - back) * platform::read_f32(ATAN_TABLE + 4 + index);
    if flag {
        (corner - interp as f64) as f32 as f64
    } else {
        (corner + interp as f64) as f32 as f64
    }
}

/// Octant-reduced arctangent returned as a packed 16-bit angle.
///
/// In plain English: "the same fold as [`fn_80237d10`], scaled to a full turn
/// and stored through the 16-bit quantisation format."
///
/// # Original
/// - Ghidra: `FUN_80237ED0` at `0x80237ED0` (464 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237ed0_FUN_80237ed0.c`
/// - `// observed:` the extra quadrant constants are `r2 - 0x4BC4` and
///   `r2 - 0x4BC0`; the final value is `r2 - 0x4BC0 * (float)angle` stored with
///   the GQR3 store format and returned as its first 16-bit lane; equal inputs
///   return zero.
///
/// # Arguments
/// - `y`: first coordinate.
/// - `x`: second coordinate.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The packed 16-bit angle.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// medium - the fold, constants, scale and store are literal; the returned lane
/// follows the same 8-bit note as [`fn_80237bd0`].
pub unsafe fn fn_80237ed0(y: f64, x: f64, gqr: u32) -> u16 {
    let zero = platform::read_f32(SDA2_TRIG_SIGN_ZERO) as f64;
    if zero == x && zero == y {
        return 0;
    }
    let mut p1 = y;
    let mut p2 = x;
    let mut corner = zero;
    let small;
    let flag;
    if p2 < zero {
        if p1 < zero {
            p2 = -p2;
            p1 = -p1;
            if p2 < p1 {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_E) as f64;
                flag = true;
                small = p2;
            } else {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_B) as f64;
                flag = false;
                small = p1;
                p1 = p2;
            }
        } else {
            p2 = -p2;
            if p2 < p1 {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_C) as f64;
                flag = false;
                small = p2;
            } else {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_B) as f64;
                flag = true;
                small = p1;
                p1 = p2;
            }
        }
    } else {
        small = p2;
        if p1 < zero {
            p1 = -p1;
            if p2 < p1 {
                corner = platform::read_f32(SDA2_TRIG_QUADRANT_E) as f64;
                flag = false;
            } else {
                corner = platform::read_f32(SDA2_TRIG_TURN) as f64;
                flag = true;
                p1 = p2;
            }
        } else if p2 < p1 {
            corner = platform::read_f32(SDA2_TRIG_QUADRANT_C) as f64;
            flag = true;
        } else {
            flag = false;
            p1 = p2;
        }
    }
    let ratio = ((small / p1) as f32) * platform::read_f32(SDA2_TRIG_RATIO);
    let (bits, back) = psq_round(ratio, gqr);
    let index = psq_lane(bits, gqr) as usize * 8;
    let interp = platform::read_f32(ATAN_TABLE + index)
        + (ratio - back) * platform::read_f32(ATAN_TABLE + 4 + index);
    let folded = if flag {
        corner - interp as f64
    } else {
        corner + interp as f64
    };
    let scaled = platform::read_f32(SDA2_TRIG_TURN) * (folded as f32);
    let (stored, _) = psq_round(scaled, gqr);
    psq_lane(stored, gqr) as u16
}

/// Approximate base-2 logarithm from the exponent and a mantissa table.
///
/// In plain English: "split the input into exponent and mantissa, quantise the
/// exponent, interpolate the mantissa table and add the weighted exponent."
///
/// # Original
/// - Ghidra: `FUN_80237B00` at `0x80237B00` (152 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237b00_FUN_80237b00.c`
/// - `// observed:` the exponent is `((bits >> 23) & 0xFF) - 0x7F`; the mantissa
///   is `bits & 0x807FFFFF | 0x3F800000`; the interpolation table is
///   `0x8034D640`/`+4` indexed by `lane * 8`; the exponent weight is
///   `r2 - 0x4BFC`.
///
/// # Arguments
/// - `x`: the positive input.
/// - `gqr5`: word used to dequantise the stored exponent lane.
/// - `gqr3`: word used for the mantissa round trip.
///
/// # Returns
/// The approximate logarithm.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// medium - the bit split, table and weights are literal; the exponent lane is
/// dequantised from an integer the original stores by hand, modelled by
/// [`psq_load_i16`].
pub unsafe fn fn_80237b00(x: f64, gqr5: u32, gqr3: u32) -> f64 {
    let bits = (x as f32).to_bits();
    let exponent = (((bits >> 23) & 0xFF) as i32 - 0x7F) as i16;
    let exponent_value = psq_load_i16(exponent, gqr5) as f64;
    let mantissa = f32::from_bits((bits & 0x807F_FFFF) | 0x3F80_0000);
    let scaled = platform::read_f32(SDA2_LOG2_SCALE) * (mantissa - platform::read_f32(SDA2_LOG2_BIAS));
    let (stored, back) = psq_round(scaled, gqr3);
    let index = psq_lane(stored, gqr3) as usize * 8;
    let interp = platform::read_f32(LOG2_TABLE + index)
        + (scaled - back) * platform::read_f32(LOG2_TABLE + 4 + index);
    (interp + platform::read_f32(SDA2_LOG2_WEIGHT) * exponent_value as f32) as f64
}

/// Transforms a 4-vector by a 3x4 matrix.
///
/// In plain English: "copy the three rotation rows and compute the three
/// translation lanes as dot products with `(vx, vy, vz, 1)`."
///
/// # Original
/// - Ghidra: `FUN_80238200` at `0x80238200` (108 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238200_FUN_80238200.c`
/// - `// observed:` twelve source lanes are loaded with the GQR load format;
///   the final stores are `dst[0..11]`, and the translation lanes `dst[3]`,
///   `dst[7]`, `dst[11]` are written twice (once as part of a pair, then again
///   with the dot product). The dot products are
///   `s2*v2 + s0*v0 + s3 + s1*v1` for each row.
///
/// # Arguments
/// - `dst`: twelve-float destination matrix.
/// - `src`: twelve-float source matrix (`// inferred:` row-major 3x4).
/// - `vec`: three-float input vector.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes 0x30 bytes at `dst`.
///
/// # Safety
/// All three blocks must be readable/writable for the stated sizes. Every
/// source lane is read before the first store, so `dst` may alias `src`.
///
/// # Confidence
/// medium - the lane order, the dot products and the double stores are literal;
/// the row-major interpretation is inferred from the offsets.
pub unsafe fn fn_80238200(dst: usize, src: usize, vec: usize, gqr: u32) {
    let s0 = psq_load(src + 0x00, gqr);
    let s1 = psq_load(src + 0x04, gqr);
    let v0 = psq_load(vec + 0x00, gqr);
    let v1 = psq_load(vec + 0x04, gqr);
    let s2 = psq_load(src + 0x08, gqr);
    let s3 = psq_load(src + 0x0C, gqr);
    let s4 = psq_load(src + 0x10, gqr);
    let s5 = psq_load(src + 0x14, gqr);
    let v2 = psq_load(vec + 0x08, gqr);
    let s6 = psq_load(src + 0x18, gqr);
    let s7 = psq_load(src + 0x1C, gqr);
    let s8 = psq_load(src + 0x20, gqr);
    let s9 = psq_load(src + 0x24, gqr);
    let s10 = psq_load(src + 0x28, gqr);
    let s11 = psq_load(src + 0x2C, gqr);

    psq_store(dst + 0x08, s2, gqr);
    psq_store(dst + 0x0C, s3, gqr);
    let row0 = s2 * v2 + s0 * v0 + s3 + s1 * v1;
    psq_store(dst + 0x18, s6, gqr);
    psq_store(dst + 0x1C, s7, gqr);
    // observed: the second store of the lane overwrites the pair copy.
    psq_store(dst + 0x0C, row0, gqr);
    let row1 = s6 * v2 + s4 * v0 + s7 + s5 * v1;
    psq_store(dst + 0x28, s10, gqr);
    psq_store(dst + 0x2C, s11, gqr);
    psq_store(dst + 0x1C, row1, gqr);
    let row2 = s10 * v2 + s8 * v0 + s11 + s9 * v1;
    psq_store(dst + 0x00, s0, gqr);
    psq_store(dst + 0x04, s1, gqr);
    psq_store(dst + 0x10, s4, gqr);
    psq_store(dst + 0x14, s5, gqr);
    psq_store(dst + 0x20, s8, gqr);
    psq_store(dst + 0x24, s9, gqr);
    psq_store(dst + 0x2C, row2, gqr);
}

/// Builds a plane from three points: normalized normal plus plane distance.
///
/// In plain English: "take `c - a` and `b - a`, cross them, normalize, and set
/// `out[3] = -dot(normal, a)`."
///
/// # Original
/// - Ghidra: `FUN_802385D0` at `0x802385D0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802385d0_FUN_802385d0.c`
/// - `// observed:` the two edge vectors are stored into adjacent scratch
///   triples, `0x801C1530` is called as `(edgeB, edgeA, cross)`, `0x801C1470`
///   normalizes the cross into `param_1`, and the last store is the plain float
///   `-(n0*a0 + n1*a1 + n2*a2)`.
///
/// # Arguments
/// - `out`: four-float plane `{nx, ny, nz, d}`.
/// - `a`: first point (`// inferred:` the shared origin of both edges).
/// - `b`: second point.
/// - `c`: third point.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes four floats at `out`; calls the cross and normalize helpers.
///
/// # Safety
/// `out` writable for 16 bytes and `a`/`b`/`c` readable for 12 bytes each.
/// The helpers expect raw-single lanes, which is how the shipped GQR0 is
/// configured.
///
/// # Confidence
/// medium - the two differences, both calls, the normalized normal and the
/// plane-distance formula are literal; the point roles are inferred from the
/// call order.
pub unsafe fn fn_802385d0(out: usize, a: usize, b: usize, c: usize, gqr: u32) {
    let cx = psq_load(c, gqr);
    let cy = psq_load(c + 4, gqr);
    let ax = psq_load(a, gqr);
    let ay = psq_load(a + 4, gqr);
    let bx = psq_load(b, gqr);
    let by = psq_load(b + 4, gqr);
    let az = psq_load(a + 8, gqr);
    let bz = psq_load(b + 8, gqr);
    let cz = psq_load(c + 8, gqr);

    let edge_c = crate::platform::GuestLocal::new( [cx - ax, cy - ay, cz - az]);
    let edge_b = crate::platform::GuestLocal::new( [bx - ax, by - ay, bz - az]);
    let mut cross = crate::platform::GuestLocal::new( [0.0_f32; 3]);
    // observed: the call order is `(edge c-a, edge b-a, out)`.
    platform::call3(
        G3D_CROSS3,
        edge_c.as_ptr() as u32,
        edge_b.as_ptr() as u32,
        cross.as_mut_ptr() as u32,
    );
    platform::call2(G3D_NORMALIZE3, cross.as_ptr() as u32, out as u32);

    let nx = psq_load(out, gqr);
    let ny = psq_load(out + 4, gqr);
    let nz = psq_load(out + 8, gqr);
    let distance = -(nx * ax + ny * ay + nz * az);
    // observed: the plane offset is a plain f32 store, not a quantised one.
    platform::write_f32(out + 12, distance);
}

/// Classifies an axis-aligned box against six planes.
///
/// In plain English: "reject a box that lies outside the bounds, then test the
/// two stored corner combinations against each of six planes."
///
/// # Original
/// - Ghidra: `FUN_80239780` at `0x80239780` (428 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239780_FUN_80239780.c`
/// - `// observed:` the bounds are `param_1+0x78..+0x8C`; each plane is 0x10
///   bytes at `param_1+0x90`; the first corner uses the x lane selected by the
///   normal's sign, the y/z lanes from the *opposite* side; the second corner
///   keeps x and flips y/z; a positive first test returns 0, a positive second
///   test raises the result to 2.
///
/// # Arguments
/// - `bounds`: object with the six bounds at `+0x78..+0x8C` and six planes at
///   `+0x90`.
/// - `points`: six floats `{min xyz, max xyz}`.
///
/// # Returns
/// 0 if the box is fully outside, 1 if fully inside, 2 if it straddles a plane.
///
/// # Side effects
/// None.
///
/// # Safety
/// `bounds` readable through `+0x120` and `points` readable for 24 bytes.
///
/// # Confidence
/// low - the bounds and the arithmetic are literal, but the export's asymmetric
/// x treatment (it selects x once and reuses it for both corners) may be a
/// decompiler artifact; the packed 8/16-bit branches are modelled on the
/// raw-single lane, which is how GQR0 is configured.
pub unsafe fn fn_80239780(bounds: usize, points: usize) -> u32 {
    let zero = platform::read_f32(SDA2_MATH_ZERO);
    let x_max = platform::read_f32(bounds + 0x84);
    let y_max = platform::read_f32(bounds + 0x88);
    let z_max = platform::read_f32(bounds + 0x8C);
    let x_min = platform::read_f32(bounds + 0x78);
    let y_min = platform::read_f32(bounds + 0x7C);
    let z_min = platform::read_f32(bounds + 0x80);
    let x0 = platform::read_f32(points);
    let y0 = platform::read_f32(points + 4);
    let z0 = platform::read_f32(points + 8);
    let x1 = platform::read_f32(points + 12);
    let y1 = platform::read_f32(points + 16);
    let z1 = platform::read_f32(points + 20);
    if x_max < x0 || x1 < x_min || y_max < y0 || y1 < y_min || z_max < z0 || z1 < z_min {
        return 0;
    }

    let mut result = 1_u32;
    for plane_index in 0..6 {
        let plane = bounds + 0x90 + plane_index * 0x10;
        let nx = platform::read_f32(plane);
        let ny = platform::read_f32(plane + 4);
        let nz = platform::read_f32(plane + 8);
        let offset = platform::read_f32(plane + 12);

        // observed: x is selected by the normal sign and reused by both tests.
        let corner_x = if nx < zero { x0 } else { x1 };
        // observed: y and z of the first test come from the opposite side.
        let first_y = if ny < zero { y1 } else { y0 };
        let first_z = if nz < zero { z1 } else { z0 };
        // observed: the second test flips y and z back.
        let second_y = if ny < zero { y0 } else { y1 };
        let second_z = if nz < zero { z0 } else { z1 };

        let first = offset + nx * corner_x + ny * first_y + nz * first_z;
        if zero < first {
            return 0;
        }
        let second = offset + nx * corner_x + ny * second_y + nz * second_z;
        if zero < second {
            result = 2;
        }
    }
    result
}

/// `r2 - 0x4AE8` - `f32` zero used as the default component value.
const SDA2_ANM_ZERO: usize = SDA2_BASE - 0x4AE8;

/// Samples one Hermite key track.
///
/// In plain English: "use the quantised count as an index hint, walk to the key
/// whose time brackets `time`, and evaluate the cubic Hermite segment."
///
/// # Original
/// - Ghidra: `FUN_8023F000` at `0x8023F000` (308 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f000_FUN_8023f000.c`
/// - `// observed:` the track is `u16 count` at `+0`, a rate at `+4`, the first
///   key time at `+8`, and 12-byte keys `{time, value, tangent}`; the quantised
///   count scales `(time - start)` into a key index; the final value is
///   `value + u^2*(3u-2)*(value-next_value) + (t-key_time)*(u-1)*((u-1)*tangent + u*next_tangent)`
///   with `u` built from the segment length.
///
/// # Arguments
/// - `time`: sample time.
/// - `track`: the key track.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The interpolated value.
///
/// # Side effects
/// None.
///
/// # Safety
/// `track` must be readable through the last key.
///
/// # Confidence
/// medium - the layout, the search and the Hermite expression are literal; the
/// key tangent role is inferred from the two tangent terms.
pub unsafe fn fn_8023f000(time: f64, track: usize, gqr: u32) -> f64 {
    let count = platform::read_u16(track) as u32;
    let start = platform::read_f32(track + 8) as f64;
    let last_key = track + count.wrapping_sub(1) as usize * 12 + 8;
    let last = platform::read_f32(last_key) as f64;
    if start < time {
        if time < last {
            let (_, count_value) = psq_round(count as f32, gqr);
            let rate = platform::read_f32(track + 4);
            let scaled = rate * (((time - start) * count_value as f64) as f32);
            let (stored, _) = psq_round(scaled, gqr);
            let guess = psq_lane(stored, gqr) as usize;
            let mut key = track + 8 + guess * 12;
            if platform::read_f32(key) as f64 <= time {
                loop {
                    let next = key + 12;
                    if !(platform::read_f32(next) as f64 <= time) {
                        break;
                    }
                    key = next;
                }
            } else {
                loop {
                    key -= 12;
                    if !(time < platform::read_f32(key) as f64) {
                        break;
                    }
                }
            }
            let key_time = platform::read_f32(key) as f64;
            if key_time == time {
                return platform::read_f32(key + 4) as f64;
            }
            let span = platform::read_f32(key + 12) - platform::read_f32(key);
            let inv = 1.0_f32 / span;
            let u = (time - key_time) as f32 * -(span * inv * inv - (inv + inv));
            let v = u - platform::read_f32(SDA2_SPLINE_ONE);
            let value = platform::read_f32(key + 4);
            let tangent = platform::read_f32(key + 8);
            let next_value = platform::read_f32(key + 16);
            let next_tangent = platform::read_f32(key + 20);
            let two = platform::read_f32(SDA2_SPLINE_TWO);
            let three = platform::read_f32(SDA2_SPLINE_THREE);
            let result = value
                + u * u * (two * u - three) * (value - next_value)
                + (time - key_time) as f32 * v * (v * tangent + u * next_tangent);
            return result as f64;
        }
        return platform::read_f32(last_key + 4) as f64;
    }
    platform::read_f32(track + 12) as f64
}

/// Finds the key whose time brackets `time`.
///
/// In plain English: "use the quantised count as an index hint, then walk the
/// 8-byte `{time, value}` keys until the bracket is found."
///
/// # Original
/// - Ghidra: `FUN_8023F4E0` at `0x8023F4E0` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f4e0_FUN_8023f4e0.c`
/// - `// observed:` the track header is `u16 count`, a rate at `+4` and the
///   first time at `+8`; keys are 8 bytes and the search returns a pointer to
///   the key at or before `time`; out-of-range times return the header or last
///   key.
///
/// # Arguments
/// - `time`: sample time.
/// - `track`: the key track.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// Address of the bracketing key.
///
/// # Side effects
/// None.
///
/// # Safety
/// `track` must be readable through the last key.
///
/// # Confidence
/// medium - the layout, hint and two search loops are literal.
pub unsafe fn fn_8023f4e0(time: f64, track: usize, gqr: u32) -> usize {
    let count = platform::read_u16(track) as u32;
    let start = platform::read_f32(track + 8) as f64;
    let last_key = track + count.wrapping_sub(1) as usize * 8 + 8;
    let last = platform::read_f32(last_key) as f64;
    if start < time && time < last {
        let (_, count_value) = psq_round(count as f32, gqr);
        let rate = platform::read_f32(track + 4);
        let scaled = rate * (((time - start) * count_value as f64) as f32);
        let (stored, _) = psq_round(scaled, gqr);
        let guess = psq_lane(stored, gqr) as usize;
        let mut key = track + 8 + guess * 8;
        if platform::read_f32(key) as f64 <= time {
            loop {
                let next = key + 8;
                if !(platform::read_f32(next) as f64 <= time) {
                    break;
                }
                key = next;
            }
        } else {
            loop {
                key -= 8;
                if !(time < platform::read_f32(key) as f64) {
                    break;
                }
            }
        }
        return key;
    }
    if start < time {
        last_key
    } else {
        track + 8
    }
}

/// Lerps one packed four-byte colour through the fixed-point fraction.
///
/// In plain English: "take the integer part and fraction produced by the split
/// helper, quantise the fraction and interpolate each of the four bytes."
///
/// # Original
/// - Ghidra: `FUN_8023F140` at `0x8023F140` (260 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023f140_FUN_8023f140.c`
/// - `// observed:` `FUN_802E83C8` returns the fraction and writes the integer
///   index through its argument; the fraction is scaled by `r2 - 0x4B18`, the
///   quantised lane is `(int)local_28._0_2_`, and each byte becomes
///   `a + (char)((lane * (b - a)) >> 15)`.
///
/// # Arguments
/// - `table`: base of the u32 colour array.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The interpolated four-byte colour.
///
/// # Side effects
/// None.
///
/// # Safety
/// `table` must be readable for `(index + 2) * 4` bytes.
///
/// # Confidence
/// high - the byte arithmetic, both `lwzx` loads and the `psq_st` quantisation
/// are literal; `0x802E83C8(r1+24)` is called through
/// [`call_f64_out_f64`], which supplies the `f1` result and the `f64` index
/// out-parameter the export describes.
pub unsafe fn fn_8023f140(table: usize, gqr: u32) -> u32 {
    let mut split: f64 = 0.0;
    let fraction = call_f64_out_f64(0x802E_83C8, &mut split);
    let index = split as i32;

    let sentinel = platform::read_f32(SDA2_ANM_SENTINEL);
    if sentinel == fraction as f32 {
        return platform::read_u32(table + index as usize * 4);
    }
    let from = platform::read_u32(table + index as usize * 4);
    let to = platform::read_u32(table + (index + 1) as usize * 4);
    let scaled = platform::read_f32(SDA2_ANM_BYTE_SCALE) * (fraction as f32);
    let (stored, _) = psq_round(scaled, gqr);
    let lane = psq_lane(stored, gqr) as i32;
    let mix = |a: u8, b: u8| -> u8 {
        let delta = b as i32 - a as i32;
        a.wrapping_add(((lane * delta) >> 15) as i8 as u8)
    };
    let b0 = mix((from >> 24) as u8, (to >> 24) as u8) as u32;
    let b1 = mix((from >> 16) as u8, (to >> 16) as u8) as u32;
    let b2 = mix((from >> 8) as u8, (to >> 8) as u8) as u32;
    let b3 = mix(from as u8, to as u8) as u32;
    (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
}

/// Calls a PowerPC function that takes one pointer in `r3` and returns its
/// scalar in `f1` while also filling that pointer with a second `f64`.
///
/// # Original
/// Host-side helper; no binary address. Models the split helper `0x802E83C8`
/// as called from [`fn_8023f140`]: `f1` carries the fraction and the argument
/// at `r1+24` receives the integral part.
///
/// # Safety
/// `addr` must use this mixed ABI and `out` must be writable.
///
/// # Confidence
/// high - `addi r3,r1,24` / `bl 0x802E83C8` / `lfd f2,24(r1)` at
/// `0x8023F154..0x8023F15C`.
#[inline]
unsafe fn call_f64_out_f64(addr: usize, out: *mut f64) -> f64 {
    let function = crate::platform::guest_fn!(addr, fn(u32) -> f64);
    function(out as u32)
}

/// `0x802403C0` - component decoder for the 8-bit animation format.
const ANM_DECODE_8: usize = 0x8024_03C0;
/// `0x80240240` - component decoder for the 16-bit animation format.
const ANM_DECODE_16: usize = 0x8024_0240;
/// `0x80240100` - component decoder for the 32-bit animation format.
const ANM_DECODE_32: usize = 0x8024_0100;

/// Reads one three-component animation element through its format flags.
///
/// In plain English: "look at the format word's `0x06000000` field, then copy
/// or decode the three component floats."
///
/// # Original
/// - Ghidra: `FUN_8023FE50` at `0x8023FE50` (680 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023fe50_FUN_8023fe50.c`
/// - `// observed:` `flags = *(u32 *)(param_3 + 4)`; the `0x06000000` field
///   selects raw floats, one of three decoders, or the zero word; bit `0x10`
///   says "single component"; bits `0x2000`/`0x4000`/`0x8000` say "already a
///   float" per component; the caller's data pointer advances by one or three
///   floats.
///
/// # Arguments
/// - `ctx`: first argument forwarded to the decoders.
/// - `out`: three-float destination.
/// - `desc`: animation descriptor (flags at `+4`).
/// - `data`: current element pointer.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// The advanced element pointer.
///
/// # Side effects
/// Writes three floats at `out`; calls one of the component decoders.
///
/// # Safety
/// `data` must be readable for the element size and the decoder must accept
/// `ctx`.
///
/// # Confidence
/// low - all branches and flag masks are literal, but Ghidra drops the first
/// argument of the first decoder call and types the whole function as variadic;
/// `ctx` is supplied explicitly and the first call is given the same argument
/// list as its siblings.
pub unsafe fn fn_8023fe50(ctx: u32, out: usize, desc: usize, data: usize, gqr: u32) -> usize {
    let _ = gqr;
    let flags = platform::read_u32(desc + 4);
    let selector = flags & 0x0600_0000;
    let mut ptr;
    let values;
    match selector {
        0 => {
            let first = platform::read_f32(data);
            let mut second = first;
            let mut third = first;
            ptr = data + 4;
            if flags & 0x10 == 0 {
                second = platform::read_f32(ptr);
                ptr = data + 12;
                third = platform::read_f32(data + 8);
            }
            values = [first, second, third];
        }
        0x0200_0000 | 0x0400_0000 | 0x0600_0000 => {
            let decoder = match selector {
                0x0200_0000 => ANM_DECODE_8,
                0x0400_0000 => ANM_DECODE_16,
                _ => ANM_DECODE_32,
            };
            ptr = data + 4;
            // observed: dVar8 is decoded first and copied into dVar7/dVar6.
            let first = if flags & 0x2000 == 0 {
                platform::call2(
                    decoder,
                    ctx,
                    desc.wrapping_add(platform::read_f32(data) as i32 as usize) as u32,
                ) as f32
            } else {
                platform::read_f32(data)
            };
            let mut second = first;
            let mut third = first;
            if flags & 0x10 == 0 {
                if flags & 0x4000 == 0 {
                    second = platform::call2(
                        decoder,
                        ctx,
                        desc.wrapping_add(platform::read_f32(ptr) as i32 as usize) as u32,
                    ) as f32;
                } else {
                    second = platform::read_f32(ptr);
                }
                ptr = data + 12;
                if flags & 0x8000 == 0 {
                    third = platform::call2(
                        decoder,
                        ctx,
                        desc.wrapping_add(platform::read_f32(data + 8) as i32 as usize) as u32,
                    ) as f32;
                } else {
                    third = platform::read_f32(data + 8);
                }
            }
            values = [first, second, third];
        }
        _ => {
            let zero = platform::read_f32(SDA2_ANM_ZERO);
            values = [zero, zero, zero];
            ptr = data;
        }
    }
    platform::write_f32(out, values[0]);
    platform::write_f32(out + 4, values[1]);
    platform::write_f32(out + 8, values[2]);
    ptr
}

/// Blits one 5x7 console glyph, optionally doubled in both axes.
///
/// In plain English: "pick the 28-byte glyph record for `code`, expand each of
/// its seven 30-bit rows through the nibble table, and write the coloured pixel
/// pairs into the console framebuffer."
///
/// # Original
/// - Ghidra: `FUN_802337B0` at `0x802337B0` (704 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802337b0_FUN_802337b0.c`
/// - `// observed:` codes below 100 use the table at `0x802F7268`, the rest
///   `0x802F7368`; the record stride is 0x1C and the row shift is
///   `(code % 5) * 6`; the scale is 1 when the framebuffer is narrower than
///   300 / shorter than 400 and 2 otherwise; the nibble table is
///   `0x8034CF30`; colours come from `0x803DA134..0x803DA140`.
///
/// # Arguments
/// - `x`: pixel column.
/// - `y`: pixel row.
/// - `code`: character code (100 and above use the second glyph table).
///
/// # Side effects
/// Writes up to 7x6 (or 14x12) 16-bit pixels through [`CONSOLE_PIXELS`].
///
/// # Safety
/// The console framebuffer must be mapped and the glyph tables readable.
///
/// # Confidence
/// medium - every shift, mask, table base and colour word is literal; only the
/// recovered `x`/`y` arguments come from the `_savegpr0` prologue, which the
/// export renders as a pair-returning stub.
pub unsafe fn fn_802337b0(x: i32, y: i32, code: i32) {
    if x < 0 || y < 0 {
        return;
    }
    let mut glyph = code;
    if code > 99 {
        glyph -= 100;
    }
    let table = if code < 100 { GLYPH_LOW } else { GLYPH_HIGH };
    let cell = (glyph / 5) as usize;
    let mut record = table + cell * 0x1C;
    // observed: the `+0x08` word gates the vertical scale and the `+0x0A`
    // word the horizontal one; the existing console constants name them the
    // other way round, so the roles are spelled out here.
    let vertical_limit = platform::read_u16(CONSOLE_WIDTH) as i32;
    let horizontal_limit = platform::read_u16(CONSOLE_HEIGHT) as i32;
    let y_scale = if vertical_limit < 400 { 1 } else { 2 };
    let x_scale = if horizontal_limit < 300 { 1 } else { 2 };
    if !(y_scale * (y + 6) < vertical_limit && x_scale * (x + 7) < horizontal_limit) {
        return;
    }
    let stride = platform::read_u32(CONSOLE_STRIDE) as i32;
    let base = platform::read_u32(CONSOLE_PIXELS) as i32;
    let mut ptr = (y * y_scale * 2 + base + x * x_scale * stride * 2) as usize;
    let background = platform::read_u16(CONSOLE_COLOUR_BG) as i32;
    let colour_0 = platform::read_u16(CONSOLE_COLOUR_0) as i16 as i32;
    let colour_1 = platform::read_u16(CONSOLE_COLOUR_1) as i16 as i32;
    let colour_2 = platform::read_u16(CONSOLE_COLOUR_2) as i16 as i32;
    let colour_3 = platform::read_u16(CONSOLE_COLOUR_3) as i16 as i32;
    let shift = (glyph - cell as i32 * 5) * 6;
    for _ in 0..7 {
        let mut word = platform::read_u32(record).wrapping_shl(shift as u32);
        if y_scale == 1 {
            word = (word >> 1) & 0x7E00_0000;
        } else {
            word = ((platform::read_u32(GLYPH_EXPAND + ((word >> 28) & 0xC) as usize) << 8)
                | platform::read_u32(GLYPH_EXPAND + ((word >> 24) & 0xC) as usize)
                | (platform::read_u32(GLYPH_EXPAND + ((word >> 26) & 0xC) as usize) << 4))
                << 19;
        }
        let pairs = (y_scale * 6 + 1) >> 1;
        for _ in 0..pairs {
            let bit_30 = word & 0x4000_0000;
            let bit_31 = word & 0x8000_0000;
            let bit_29 = word & 0x2000_0000;
            let bit_28 = word & 0x1000_0000;
            let first = (if bit_30 == 0 { 0 } else { background })
                | (if bit_30 == 0 { 0x40 } else { colour_0 })
                    .wrapping_add(if bit_29 == 0 { 0x20 } else { colour_1 })
                    .wrapping_add(if bit_31 == 0 { 0x20 } else { colour_1 });
            platform::write_u16(ptr, first as u16);
            if x_scale > 1 {
                platform::write_u16(ptr + stride as usize * 2, first as u16);
            }
            let second = (if bit_29 == 0 { 0 } else { background })
                | (if bit_29 == 0 { 0x40 } else { colour_2 })
                    .wrapping_add(if bit_30 == 0 { 0x20 } else { colour_3 })
                    .wrapping_add(if bit_28 == 0 { 0x20 } else { colour_3 });
            platform::write_u16(ptr + 2, second as u16);
            if x_scale > 1 {
                platform::write_u16(ptr + 2 + stride as usize * 2, second as u16);
            }
            word = word.wrapping_shl(2);
            ptr += 4;
        }
        ptr = (ptr as i32 + (stride * x_scale - 6 * y_scale) * 2) as usize;
        record += 4;
    }
}

/// Prints one exception-report line and echoes it to the console when active.
///
/// In plain English: "hand the format and its argument descriptor to the
/// low-level report thunk, then, if a console is installed, refresh its line
/// counter and print the same line through the console sink."
///
/// # Original
/// - Ghidra: `FUN_80234A80` at `0x80234A80` (224 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80234a80_FUN_80234a80.c`
/// - `// observed:` `FUN_8015F6E0` receives the recovered format word and a
///   descriptor whose type word is `0x01000000`; when `0x803DA8A0` is nonzero
///   the body calls `0x801BDE40`, feeds the two `u16`s at
///   `0x803DA8A4 + 4`/`+ 8` to `0x80233420`, and calls
///   `fn_802344e0(1, console, format, descriptor)`.
///
/// # Arguments
/// - `fmt`: format string.
/// - `args`: a `va_list`-shaped descriptor for `fmt`.
///
/// # Side effects
/// Emits the report and may print on the active console.
///
/// # Safety
/// `args` must describe a valid call to `fmt`.
///
/// # Confidence
/// low - the gate, the line-counter calls and both sinks are literal, but the
/// descriptor is built from the incoming FPR/GPR save area, which this model
/// supplies as a caller-owned descriptor (`// partial:`).
pub unsafe fn fn_80234a80(fmt: u32, args: u32) {
    let record = platform::read_u32(CONSOLE_RECORD);
    platform::call2(REPORT_THUNK, fmt, args);
    let console = platform::read_u32(CONSOLE_ACTIVE);
    if console != 0 {
        let stamp = call0_word(CONSOLE_TIMESTAMP);
        let line = platform::read_u16(record as usize + 4) as u32;
        let column = platform::read_u16(record as usize + 8) as u32;
        platform::call3(CONSOLE_LINE_COUNTER, stamp, line, column);
        fn_802344e0(1, console as usize, fmt, args);
    }
}

/// Intersects a sphere/plane record with a point and a normal.
///
/// In plain English: "form `point - center`, take half the dot product with the
/// normal, subtract the squared length minus the radius squared, and return the
/// two roots of the resulting quadratic."
///
/// # Original
/// - Ghidra: `FUN_80239380` at `0x80239380` (344 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80239380_FUN_80239380.c`
/// - `// observed:` the difference is `param_1[0..2] - param_2[0..2]`; the
///   normal is `param_1[3..5]`; the quadratic coefficient is
///   `r2 - 0x4B80 * dot(normal, difference)`; the discriminant is
///   `b*b - r2(-0x4B7C) * (|d|^2 - param_2[3]^2)`; the roots are scaled by
///   `r2 - 0x4BA0`, the tangent root by `r2 - 0x4B78`, and the square root is
///   the refined reciprocal of [`fn_80237ba0`].
///
/// # Arguments
/// - `point`: six floats: position `xyz` and normal `xyz`.
/// - `shape`: four floats: center `xyz` and radius.
/// - `out_near`, `out_far`: optional float destinations for the roots.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// 0 no hit, 1 tangent hit (only `out_near`), 2 two hits.
///
/// # Side effects
/// Writes `out_near`/`out_far` when non-null and the corresponding hit exists.
///
/// # Safety
/// Both input records must be readable for the stated sizes; the output
/// pointers may be null.
///
/// # Confidence
/// medium - the quadratic and both roots are literal; the role assignment
/// `point`/`shape` is inferred from the offsets, and the scratch pair is
/// modelled on raw-single lanes.
pub unsafe fn fn_80239380(
    point: usize,
    shape: usize,
    out_near: usize,
    out_far: usize,
    gqr: u32,
) -> u32 {
    let zero = platform::read_f32(SDA2_MATH_ZERO);
    let dx = psq_load(point, gqr) - psq_load(shape, gqr);
    let dy = psq_load(point + 4, gqr) - psq_load(shape + 4, gqr);
    let dz = psq_load(point + 8, gqr) - psq_load(shape + 8, gqr);
    let nx = psq_load(point + 12, gqr);
    let ny = psq_load(point + 16, gqr);
    let nz = psq_load(point + 20, gqr);
    let radius = psq_load(shape + 12, gqr);
    let b = platform::read_f32(SDA2_MATH_TWO) * (nx * dx + ny * dy + nz * dz);
    let discriminant = b * b
        - platform::read_f32(SDA2_MATH_FOUR) * (dx * dx + dy * dy + dz * dz - radius * radius);
    if discriminant < zero {
        return 0;
    }
    if discriminant == zero {
        if out_near != 0 {
            platform::write_f32(out_near, platform::read_f32(SDA2_MATH_NEG_HALF) * b);
        }
        return 1;
    }
    let root = sqrt64(discriminant as f64) as f32;
    let scale = platform::read_f32(SDA2_MATH_HALF);
    if out_near != 0 {
        platform::write_f32(out_near, scale * (-b - root));
    }
    if out_far != 0 {
        platform::write_f32(out_far, scale * (-b + root));
    }
    2
}

/// Clips a directed segment against the shape accepted by [`fn_80239380`].
///
/// In plain English: "normalize `origin + 3..5 - origin[0..2]`, intersect the
/// shape, then turn the first two roots into fractions of the segment length."
///
/// # Original
/// - Ghidra: `FUN_802394E0` at `0x802394E0` (548 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802394e0_FUN_802394e0.c`
/// - `// observed:` the direction is `param_1[3..5] - param_1[0..2]`; its length
///   is `sqrt(|d|^2)` via [`fn_80237ba0`]; the direction is normalized by
///   `0x801C1470` in place; `0x80239380` supplies the two roots; negative or
///   over-long roots reject the hit; the returned fractions are root/length.
///
/// # Arguments
/// - `origin`: six floats: base `xyz` and segment end `xyz`.
/// - `shape`: the record handed to [`fn_80239380`].
/// - `out_near`, `out_far`: optional float destinations.
/// - `gqr`: the live quantisation register word.
///
/// # Returns
/// 0 no clip, 1 the first root, 2 both roots.
///
/// # Side effects
/// Writes the output fractions and normalizes a local direction copy.
///
/// # Safety
/// `origin` readable for 24 bytes; output pointers may be null.
///
/// # Confidence
/// medium - the length, normalize call, root selection and rejections are
/// literal; the raw-single lane assumption of [`fn_80239380`] applies.
pub unsafe fn fn_802394e0(
    origin: usize,
    shape: usize,
    out_near: usize,
    out_far: usize,
    gqr: u32,
) -> i32 {
    let zero = platform::read_f32(SDA2_MATH_ZERO);
    let ox = psq_load(origin, gqr);
    let oy = psq_load(origin + 4, gqr);
    let oz = psq_load(origin + 8, gqr);
    let dx = psq_load(origin + 12, gqr) - ox;
    let dy = psq_load(origin + 16, gqr) - oy;
    let dz = psq_load(origin + 20, gqr) - oz;
    let length_sq = dx * dx + dy * dy + dz * dz;
    let mut length = 0.0_f64;
    if zero < length_sq {
        length = (length_sq as f64 * fn_80237ba0(length_sq as f64)) as f32 as f64;
    }
    let mut direction = crate::platform::GuestLocal::new( [dx, dy, dz]);
    platform::call2(
        G3D_NORMALIZE3,
        direction.as_ptr() as u32,
        direction.as_mut_ptr() as u32,
    );
    // observed: the original copies `origin.xyz` and the normalized direction
    // into one six-float block before calling `0x80239380`.
    let point = crate::platform::GuestLocal::new( [
        ox,
        oy,
        oz,
        direction[0],
        direction[1],
        direction[2],
    ]);
    let mut near = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut far = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut result = fn_80239380(
        point.as_ptr() as usize,
        shape,
        (near.as_mut_ptr() as *mut f32) as usize,
        (far.as_mut_ptr() as *mut f32) as usize,
        gqr,
    ) as i32;
    if result != 0 {
        if result == 1 {
            let t = *near as f64;
            if t < zero as f64 || length < t {
                result = 0;
            } else if out_near != 0 {
                platform::write_f32(out_near, (t / length) as f32);
            }
        } else {
            let first = *near as f64;
            if first < zero as f64 {
                let second = *far as f64;
                if second < zero as f64 || length < second {
                    result = 0;
                } else {
                    if out_near != 0 {
                        platform::write_f32(out_near, (second / length) as f32);
                    }
                    result = 1;
                }
            } else if length < *far as f64 {
                if length < first {
                    result = 0;
                } else {
                    if out_near != 0 {
                        platform::write_f32(out_near, (first / length) as f32);
                    }
                    result = 1;
                }
            } else {
                let inv = platform::read_f32(SDA2_MATH_RECIP) as f64 / length;
                if out_near != 0 {
                    platform::write_f32(out_near, (first * inv) as f32);
                }
                if out_far != 0 {
                    platform::write_f32(out_far, (*far as f64 * inv) as f32);
                }
            }
        }
    }
    result
}

/// Shared body of the four console queue entry points.
///
/// In plain English: "ask the heap for the formatting size, reserve a
/// temporary block, format the caller's descriptor into it, hand the text to
/// the queue sink and free the block."
///
/// # Original
/// - Ghidra: the repeated body of `FUN_80236230`, `FUN_802365E0`,
///   `FUN_80236FC0` and `FUN_80237370`
/// - Evidence: `work/ghidra/main-gb/export/80236230_FUN_80236230.c` (and the
///   three sibling exports)
/// - `// observed:` `size = (heap_size(heap, 4) - 0x1C) >> 1`;
///   `heap_reset(heap, 0)`; `block = heap_alloc(heap, size, flag == 1 ? -4 : 4)`;
///   `written = format(block, size, fmt, descriptor)`; `sink(hi, lo, kind,
///   flag, block, written)`; `heap_free(heap, block)`.
///
/// # Arguments
/// - `heap_slot`: SDA slot holding the heap handle.
/// - `formatter`: wide or narrow formatter address.
/// - `sink`: the queue record sink.
/// - `target_hi`, `target_lo`: the two words recovered from the variadic
///   prologue.
/// - `kind`, `flag`: the two words the sink receives before the text.
/// - `fmt`: format string.
/// - `args`: a `va_list`-shaped descriptor for `fmt`.
///
/// # Side effects
/// Allocates and frees a temporary block and enqueues one formatted record.
///
/// # Safety
/// The heap must be initialised; `args` must match `fmt`.
///
/// # Confidence
/// low - the size arithmetic, allocate/format/free order and all four callees
/// are literal, but the FPR part of the descriptor is supplied by the caller
/// (`// partial:`).
unsafe fn console_queue_entry(
    heap_slot: usize,
    formatter: usize,
    sink: usize,
    target_hi: u32,
    target_lo: u32,
    kind: u32,
    flag: i32,
    fmt: u32,
    args: u32,
) {
    let heap = platform::read_u32(heap_slot);
    let size_word = platform::call2(HEAP_SIZE, heap, 4);
    let chars = size_word.wrapping_sub(0x1C) >> 1;
    platform::call2(HEAP_RESET, heap, 0);
    let grow = if flag == 1 { 0xFFFF_FFFC } else { 4 };
    let block = platform::call3(HEAP_ALLOC, heap, chars, grow);
    if block != 0 {
        let written = platform::call4(formatter, block, chars, fmt, args);
        platform::call6(
            sink,
            target_hi,
            target_lo,
            kind,
            flag as u32,
            block,
            written,
        );
        platform::call2(HEAP_FREE, heap, block);
    }
}

/// Enqueues one wide-character console line through heap A.
///
/// # Original
/// - Ghidra: `FUN_80236230` at `0x80236230` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236230_FUN_80236230.c`
/// - `// observed:` heap slot `r13 - 0x49F0`, formatter `0x802E15DC`, sink
///   `0x80236420`, descriptor type word `0x05000000`, size delta `-4` when
///   `param_12 == 1`.
///
/// # Arguments
/// - `target_hi`, `target_lo`: words recovered from the prologue save area.
/// - `kind`, `flag`: forwarded to the sink.
/// - `fmt`, `args`: format and descriptor.
///
/// # Safety
/// The console heap must be live and `args` must match `fmt`.
///
/// # Confidence
/// low - see [`console_queue_entry`]; the FPR descriptor is caller-supplied.
pub unsafe fn fn_80236230(
    target_hi: u32,
    target_lo: u32,
    kind: u32,
    flag: i32,
    fmt: u32,
    args: u32,
) {
    console_queue_entry(
        CONSOLE_HEAP_A,
        FORMAT_WIDE,
        QUEUE_SINK_A,
        target_hi,
        target_lo,
        kind,
        flag,
        fmt,
        args,
    );
}

/// Enqueues one wide-character console line through heap A (second entry).
///
/// # Original
/// - Ghidra: `FUN_802365E0` at `0x802365E0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802365e0_FUN_802365e0.c`
/// - `// observed:` the body is byte-for-byte the same shape as
///   `0x80236230`: heap slot `r13 - 0x49F0`, formatter `0x802E15DC`, sink
///   `0x80236420`. The two entry points differ only in their caller-visible
///   identity.
///
/// # Arguments
/// - `target_hi`, `target_lo`: words recovered from the prologue save area.
/// - `kind`, `flag`: forwarded to the sink.
/// - `fmt`, `args`: format and descriptor.
///
/// # Safety
/// The console heap must be live and `args` must match `fmt`.
///
/// # Confidence
/// low - see [`console_queue_entry`].
pub unsafe fn fn_802365e0(
    target_hi: u32,
    target_lo: u32,
    kind: u32,
    flag: i32,
    fmt: u32,
    args: u32,
) {
    console_queue_entry(
        CONSOLE_HEAP_A,
        FORMAT_WIDE,
        QUEUE_SINK_A,
        target_hi,
        target_lo,
        kind,
        flag,
        fmt,
        args,
    );
}

/// Enqueues one narrow-character console line through heap B.
///
/// # Original
/// - Ghidra: `FUN_80236FC0` at `0x80236FC0` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80236fc0_FUN_80236fc0.c`
/// - `// observed:` heap slot `r13 - 0x49E8`, formatter `0x802E4390`, sink
///   `0x802371B0`, descriptor type word `0x05000000`.
///
/// # Arguments
/// - `target_hi`, `target_lo`: words recovered from the prologue save area.
/// - `kind`, `flag`: forwarded to the sink.
/// - `fmt`, `args`: format and descriptor.
///
/// # Safety
/// The console heap must be live and `args` must match `fmt`.
///
/// # Confidence
/// low - see [`console_queue_entry`].
pub unsafe fn fn_80236fc0(
    target_hi: u32,
    target_lo: u32,
    kind: u32,
    flag: i32,
    fmt: u32,
    args: u32,
) {
    console_queue_entry(
        CONSOLE_HEAP_B,
        FORMAT_NARROW,
        QUEUE_SINK_B,
        target_hi,
        target_lo,
        kind,
        flag,
        fmt,
        args,
    );
}

/// Enqueues one narrow-character console line through heap B (second entry).
///
/// # Original
/// - Ghidra: `FUN_80237370` at `0x80237370` (288 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237370_FUN_80237370.c`
/// - `// observed:` same body as `0x80236FC0`: heap slot `r13 - 0x49E8`,
///   formatter `0x802E4390`, sink `0x802371B0`.
///
/// # Arguments
/// - `target_hi`, `target_lo`: words recovered from the prologue save area.
/// - `kind`, `flag`: forwarded to the sink.
/// - `fmt`, `args`: format and descriptor.
///
/// # Safety
/// The console heap must be live and `args` must match `fmt`.
///
/// # Confidence
/// low - see [`console_queue_entry`].
pub unsafe fn fn_80237370(
    target_hi: u32,
    target_lo: u32,
    kind: u32,
    flag: i32,
    fmt: u32,
    args: u32,
) {
    console_queue_entry(
        CONSOLE_HEAP_B,
        FORMAT_NARROW,
        QUEUE_SINK_B,
        target_hi,
        target_lo,
        kind,
        flag,
        fmt,
        args,
    );
}

/// `0x8034D538` - two-float sine-offset table.
const SINE_OFFSET_TABLE: usize = 0x8034_D538;

/// Evaluates a sine plus a quantisation-corrected table offset.
///
/// In plain English: "scale the input, round it through GQR5, wrap the residual
/// into one turn, round that through GQR3 and combine the low-byte table entry
/// with the raw lane."
///
/// # Original
/// - Ghidra: `FUN_80237A60` at `0x80237A60` (156 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80237a60_FUN_80237a60.c`
/// - `// observed:` the first conversion is `r2 - 0x4C00 * param_1` through
///   GQR5; the second is `r2 - 0x4BF8 * (r2 - 0x4BFC + (param_1 - r2 - 0x4BFC *
///   first))` through GQR3; the final value is
///   `ABS((float)(lane5 * 0x800000 + (int)(table[0x8034D538 + lane3 * 8] +
///   (second - back3) * table[+4])))`.
///
/// # Arguments
/// - `x`: the input angle.
/// - `gqr5`: quantisation word of the first conversion.
/// - `gqr3`: quantisation word of the second conversion.
///
/// # Returns
/// The absolute combined value.
///
/// # Side effects
/// None.
///
/// # Safety
/// Pure computation.
///
/// # Confidence
/// low - every operation is literal, including the `lane * 0x800000` integer
/// term that the export produces; its purpose is not explained by the body.
pub unsafe fn fn_80237a60(x: f64, gqr5: u32, gqr3: u32) -> f64 {
    let first = (platform::read_f32(SDA2_BASE - 0x4C00) as f64 * x) as f32;
    let (bits5, back5) = psq_round(first, gqr5);
    let lane5 = psq_lane(bits5, gqr5) as i32;
    let inner = (platform::read_f32(SDA2_BASE - 0x4BFC) as f64 * back5 as f64) as f32;
    let second = platform::read_f32(SDA2_BASE - 0x4BF8)
        * (platform::read_f32(SDA2_BASE - 0x4BFC) + ((x - inner as f64) as f32));
    let (bits3, back3) = psq_round(second, gqr3);
    let index = psq_lane(bits3, gqr3) as usize * 8;
    let table = platform::read_f32(SINE_OFFSET_TABLE + index)
        + (second - back3) * platform::read_f32(SINE_OFFSET_TABLE + 4 + index);
    let combined = lane5
        .wrapping_mul(0x0080_0000)
        .wrapping_add(table as i32);
    (combined as f32).abs() as f64
}

/// Applies one animation key set to the output block.
///
/// In plain English: "walk the two component masks of the animation record,
/// sample every present track at `time`, and write the results into the packed
/// output block."
///
/// # Original
/// - Ghidra: `FUN_8023FA30` at `0x8023FA30` (1044 bytes)
/// - Evidence: `work/ghidra/main-gb/export/8023fa30_FUN_8023fa30.c`
/// - `// observed:` `FUN_802DCD5C` supplies the output pointer (low half) and
///   the resource pointer (high half); the resource is resolved through `+0x10`
///   and `(index + 1) * 0x10 + 0x14`; two bit loops consume the masks at `+4`
///   and `+8`, each selected bit reading a 32-bit offset table at `+0xC`; every
///   component either is a raw float (flags `0x20`/`0x40`/`0x80`/`0x100`/
///   `0x200`) or a track offset sampled with `FUN_8023F000`.
///
/// # Arguments
/// - `time`: sample time.
/// - `out`: the output block (at least 0x110 bytes in the original callers).
/// - `resource`: pointer to the resource word.
/// - `index`: sub-resource index.
/// - `gqr`: the live quantisation register word.
///
/// # Side effects
/// Writes the packed output block.
///
/// # Safety
/// `out` must be writable for the packed size, `resource` readable, and every
/// track offset must point at a valid [`fn_8023f000`] track.
///
/// # Confidence
/// medium - both loops, the masks, the offset table and the per-component
/// choices are literal; the two prologue results are modelled as explicit
/// `out`/`resource` arguments.
pub unsafe fn fn_8023fa30(time: f64, out: usize, resource: usize, index: i32, gqr: u32) {
    let zero = platform::read_f32(SDA2_BASE - 0x4AF0);
    let one = platform::read_f32(SDA2_BASE - 0x4AEC);
    let root = platform::read_u32(resource) as usize;
    let first_offset = platform::read_u32(root + 0x10) as usize;
    let mut node = if first_offset == 0 { 0 } else { root + first_offset };
    if node != 0 {
        let second_offset = platform::read_u32(node + (index as usize + 1) * 0x10 + 0x14) as usize;
        node = if second_offset == 0 { 0 } else { node + second_offset };
    }
    let mask_a = platform::read_u32(node + 4);
    let mask_b = platform::read_u32(node + 8);
    let mut offsets = node + 0x0C;

    platform::write_u32(out, 0);
    platform::write_u32(out + 4, 0);
    platform::write_u32(out + 8, platform::read_u32(root + 0x24));
    let mut dst = out + 12;
    let mut shift = 0_u32;
    let mut mask = mask_a;
    while mask != 0 {
        if mask & 1 != 0 {
            let offset = platform::read_u32(offsets);
            offsets += 4;
            let record = node + offset as usize;
            let flags = platform::read_u32(record);
            let mut lane = 0_usize;
            if flags & 2 == 0 {
                let at = record + 4;
                let value = if flags & 0x20 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(dst, value);
                lane = 1;
                if flags & 0x10 == 0 {
                    let at = record + 8;
                    let second = if flags & 0x40 == 0 {
                        fn_8023f000(
                            time,
                            at.wrapping_add(platform::read_f32(at) as i32 as usize),
                            gqr,
                        ) as f32
                    } else {
                        platform::read_f32(at)
                    };
                    platform::write_f32(dst + 4, second);
                    lane = 2;
                } else {
                    platform::write_f32(dst + 4, value);
                }
            } else {
                platform::write_f32(dst, zero);
                platform::write_f32(dst + 4, zero);
            }
            if flags & 4 == 0 {
                let at = record + lane * 4 + 4;
                let value = if flags & 0x80 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(dst + 8, value);
                lane += 1;
            } else {
                platform::write_f32(dst + 8, one);
            }
            if flags & 8 == 0 {
                let at = record + lane * 4 + 4;
                let value = if flags & 0x100 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(dst + 12, value);
                let at = record + (lane + 1) * 4 + 4;
                let second = if (flags >> 9) & 1 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(dst + 16, second);
            } else {
                platform::write_f32(dst + 12, one);
                platform::write_f32(dst + 16, one);
            }
            platform::write_u32(
                out,
                platform::read_u32(out) | (flags & 0xF).wrapping_shl(shift),
            );
        }
        dst += 20;
        shift += 4;
        mask >>= 1;
    }

    let mut shift = 0_u32;
    let mut counter = 0_usize;
    let mut mask = mask_b;
    while mask != 0 {
        if mask & 1 != 0 {
            let row = counter + 8;
            let offset = platform::read_u32(offsets);
            offsets += 4;
            let record = node + offset as usize;
            let flags = platform::read_u32(record);
            let mut lane = 0_usize;
            if flags & 2 == 0 {
                let at = record + 4;
                let value = if flags & 0x20 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(out + (row * 5 + 3) * 4, value);
                lane = 1;
                if flags & 0x10 == 0 {
                    let at = record + 8;
                    let second = if flags & 0x40 == 0 {
                        fn_8023f000(
                            time,
                            at.wrapping_add(platform::read_f32(at) as i32 as usize),
                            gqr,
                        ) as f32
                    } else {
                        platform::read_f32(at)
                    };
                    platform::write_f32(out + (row * 5 + 4) * 4, second);
                    lane = 2;
                } else {
                    let previous = platform::read_u32(out + (row * 5 + 3) * 4);
                    platform::write_u32(out + (row * 5 + 4) * 4, previous);
                }
            } else {
                platform::write_f32(out + (row * 5 + 3) * 4, zero);
                platform::write_f32(out + (row * 5 + 4) * 4, zero);
            }
            if flags & 4 == 0 {
                let at = record + lane * 4 + 4;
                let value = if flags & 0x80 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(out + (row * 5 + 5) * 4, value);
                lane += 1;
            } else {
                platform::write_f32(out + (row * 5 + 5) * 4, one);
            }
            if flags & 8 == 0 {
                let at = record + lane * 4 + 4;
                let value = if flags & 0x100 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(out + (row * 5 + 6) * 4, value);
                let at = record + (lane + 1) * 4 + 4;
                let second = if (flags >> 9) & 1 == 0 {
                    fn_8023f000(
                        time,
                        at.wrapping_add(platform::read_f32(at) as i32 as usize),
                        gqr,
                    ) as f32
                } else {
                    platform::read_f32(at)
                };
                platform::write_f32(out + (row * 5 + 7) * 4, second);
            } else {
                platform::write_f32(out + (row * 5 + 6) * 4, one);
                platform::write_f32(out + (row * 5 + 7) * 4, one);
            }
            platform::write_u32(
                out + 4,
                platform::read_u32(out + 4) | (flags & 0xF).wrapping_shl(shift),
            );
        }
        shift += 4;
        counter += 1;
        mask >>= 1;
    }
}

// ===========================================================================
// Wave 106: the text-anchor twins, the console layout chain and the frame pump.
//
// Wave 100 recorded ten addresses in this bucket as unreconstructible. Five of
// them are answered here; the argument shapes that made them look impossible
// are resolved by their call sites:
//
//   * `0x80230280` / `0x80232C10` are called as `(stream, &x, &y, start, count)`
//     by `0x8022FA70` and `0x802323E0`. Ghidra's `FUN_802DCD74` prologue stub,
//     which it models as returning `r3:r4`, is really `_savegpr0_25`: the pair
//     carries `r4` in the low word and `r3` in the high word. That is why the
//     export appears to hand the function two `float *` it never receives.
//   * `0x802348F0` needed a 64-bit return from `0x801B5F80` and a 64-bit divide;
//     [`crate::platform::call_u64`] now exposes the first, and the second is the
//     Metrowerks `__div2u` helper, replaced here by native `u64` division.
//
// The remaining five (`0x80235AE0`, `0x802382B0`, `0x80238930`, `0x802389F0`,
// `0x802390D0`) are still skipped; `docs/COVERAGE_WAVE106_BRES.md` records the
// exact reason for each.
// ===========================================================================

/// Text stream: `u32` mode/anchor word (`puVar4[0x17]` in the exports).
const TEXT_STREAM_MODE: usize = 0x5C;
/// Text stream: `f32` pen X (`puVar4[0x0B]`), the first field written here.
const TEXT_STREAM_PEN_X: usize = 0x2C;
/// Text stream: `f32` pen Y (`puVar4[0x0C]`), the second field written here.
const TEXT_STREAM_PEN_Y: usize = 0x30;
/// Text stream: `u32` pointer to the layout object (`puVar6[0x18]`).
const TEXT_STREAM_LAYOUT: usize = 0x60;
/// Text stream: `u32` pointer to the character source (`puVar6[0x12]`).
const TEXT_STREAM_SOURCE: usize = 0x48;
/// `r2 - 0x4C28` - multiplier applied to half an extent by the anchor modes.
///
/// `// unknown:` the stored value; the export only ever multiplies an extent by
/// it, and the anchor arithmetic is consistent with a half.
const SDA2_TEXT_CENTRE: usize = SDA2_BASE - 0x4C28;
/// `r2 - 0x4C38` - height above which the layout engine wraps the run.
///
/// This is the same word the stream constructor stores at `+0x4C`, where the
/// crate names it [`SDA2_STREAM_WORD_A`]; it is named for its role here.
const SDA2_TEXT_LIMIT: usize = SDA2_BASE - 0x4C38;
/// `r2 - 0x4C34` - the zero the text band seeds pens and rects with.
///
/// This is the same word the stream constructor stores at `+0x50`/`+0x54`,
/// where the crate names it [`SDA2_STREAM_WORD_B`].
const SDA2_TEXT_ZERO: usize = SDA2_BASE - 0x4C34;

/// `0x8022F960` - measures one run of engine text into a four-float box.
///
/// `// inferred:` the name; the argument shape `(stream, rect, start, count)`
/// is observed at both call sites in this file.
const TEXT_ENGINE_MEASURE: usize = 0x8022_F960;
/// `0x8022F350` - measures one rotated run into a four-float box.
///
/// `// inferred:` the name; called exactly like [`TEXT_ENGINE_MEASURE`] but from
/// the two rotated anchor modes.
const TEXT_ROTATED_MEASURE: usize = 0x8022_F350;
/// `0x8022F910` - seeds the four-word character iterator from a text source.
///
/// `runtime::bt::fn_8022f910` records the same body as "seeds a four-word
/// record with 0 plus source `+4`/`+8`/`+0xC`".
const TEXT_SOURCE_ITERATOR_INIT: usize = 0x8022_F910;
/// `0x802DCBFC` - reads one character from the iterator and advances it.
///
/// The character reader is the shared pointer-to-member dispatcher
/// `0x802DCBFC`; the four-word cursor record holds the receiver in word 0 and
/// the three-word descriptor in words 1..3, so the call sites pass
/// `interface_call_desc(Descriptor::new(record + 4), record)`.
/// `0x8022D5F0` - scrolls the page source by one character.
///
/// `runtime::bt::fn_8022d5f0` records the same body as "scroll advance over the
/// inner page descriptor with the `0x43` blend flag".
const TEXT_RUN_SCROLL: usize = 0x8022_D5F0;
/// `0x8022D500` - extra vertical extent of the current run, returned in `f1`.
///
/// `// inferred:` the name; `runtime::bt` records the same body as "height at
/// `+0x28` times inner vtable slot `+0x14`".
const TEXT_RUN_HEIGHT: usize = 0x8022_D500;
/// `0x802340A0` - one console paint pass (`runtime::bt::fn_802340a0`).
const CONSOLE_PAINT_PASS: usize = 0x8023_40A0;
/// `0x802345C0` - console queue height used by the frame pump.
const CONSOLE_QUEUE_HEIGHT: usize = 0x8023_45C0;
/// `0x801B5F80` - `OSGetTime`: 64-bit tick counter.
const OS_GET_TIME: usize = 0x801B_5F80;
/// `0x800000F8` - the bus clock word of the OS boot info.
const BUS_CLOCK: usize = 0x8000_00F8;
/// One frame of the console pump is paced to this many milliseconds.
const CONSOLE_FRAME_MILLISECONDS: u64 = 200;

/// Copies the 0x64-byte text-stream record and clears its two pen words.
///
/// In plain English: "snapshot everything the measure routine looks at except
/// the pen position, which must start at the origin."
///
/// # Original
/// Host-side helper for [`fn_80230280`] and [`fn_80232c10`]; no binary address.
/// - `// observed:` both exports copy the record word by word, then seed the
///   two floats at `+0x2C`/`+0x30` with the `r2 - 0x4C34` zero before handing
///   the copy to the measure routine. Every other byte, including the mode word
///   at `+0x5C`, is copied unchanged.
///
/// # Safety
/// `dst` must be writable for 0x64 bytes, `src` readable for 0x64 bytes.
///
/// # Confidence
/// medium - the byte count and the two cleared words are literal; the copy is
/// expressed as a byte loop instead of the export's mixed word/u16/byte stores.
unsafe fn copy_text_stream_without_pen(dst: usize, src: usize) {
    for offset in 0..0x64 {
        platform::write_u8(dst + offset, platform::read_u8(src + offset));
    }
    platform::write_f32(dst + TEXT_STREAM_PEN_X, platform::read_f32(SDA2_TEXT_ZERO));
    platform::write_f32(dst + TEXT_STREAM_PEN_Y, platform::read_f32(SDA2_TEXT_ZERO));
}

/// Anchors a measured engine-text run in its box and publishes the pen.
///
/// In plain English: "measure the run through a copy with a zero pen, move the
/// caller's anchor point by the part of the alignment the measure did not
/// already absorb, then store the final pen X/Y back into the stream."
///
/// # Original
/// - Ghidra: `FUN_80230280` at `0x80230280` (1240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80230280_FUN_80230280.c`
/// - `// observed:` bits `0x30` of the mode word select the horizontal anchor
///   (`0x10` subtracts half the width, `0x20` subtracts the whole width); bits
///   `0x300` do the same vertically (`0x100`/`0x200`); bits `0x0..1` select the
///   plain, mode-1 or mode-2 pen write at `+0x2C` (modes 1 and 2 re-measure the
///   run before writing it).
/// - `// observed:` the run is measured only when `mode & 0x333` is neither
///   zero nor `0x300`; the four-float box is seeded with the `r2 - 0x4C34` zero
///   and the width/height are `box[0] + box[2]` and `box[1] + box[3]`.
/// - `// observed:` the two incoming paired-single pairs (`f30`/`ps30_1` and
///   `f31`/`ps31_1`) are stored to the frame and never read again.
///
/// # Arguments
/// - `stream`: the 0x64-byte text-stream record. `+0x5C` is the mode word and
///   `+0x2C`/`+0x30` are the two pen floats the body rewrites.
/// - `x`: address of the caller's horizontal anchor, updated in place.
/// - `y`: address of the caller's vertical anchor, updated in place.
/// - `start`, `count`: the run handed to `0x8022F960` / `0x8022F350`
///   (first character, character count). See [`fn_802322c0`].
///
/// # Returns
/// The measured width, which the export's tail hands to the restore stub and
/// which the callers read from `f1` (`0x8022FA70` and `0x802323E0` both widen
/// it to `f64`).
///
/// # Side effects
/// Reads and writes the stream record, copies it three times into the frame,
/// and calls `0x8022F960`, `0x8022F350`, `0x8022C980` and `0x8022D500`.
///
/// # Safety
/// `stream` must be a live 0x64-byte record, `x`/`y` must be writable floats,
/// and the layout source reached through `+0x48` must be valid while the
/// measure routines run.
///
/// # Confidence
/// medium - every branch, mask, store and call is literal. The two dead FPR
/// pairs are dropped as parameters, the returned width is `// inferred:` from
/// the two call sites, and `0x8022D500`'s name is inferred.
pub unsafe fn fn_80230280(stream: usize, x: usize, y: usize, start: u32, count: u32) -> f32 {
    let zero = platform::read_f32(SDA2_TEXT_ZERO) as f64;
    let centre = platform::read_f32(SDA2_TEXT_CENTRE) as f64;
    let mode = platform::read_u32(stream + TEXT_STREAM_MODE);
    let masked = mode & 0x333;

    let mut width = zero;
    let mut height = zero;
    if masked != 0x300 && masked != 0 {
        let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
        let base = snapshot.as_mut_ptr() as usize;
        copy_text_stream_without_pen(base, stream);
        let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
        let rect_ptr = rect.as_mut_ptr() as usize;
        platform::call4(TEXT_ENGINE_MEASURE, base as u32, rect_ptr as u32, start, count);
        platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
        width = (rect[0] + rect[2]) as f64;
        height = (rect[1] + rect[3]) as f64;
    }

    if mode & 0x30 == 0x10 {
        let delta = (width * centre) as f32;
        platform::write_f32(x, platform::read_f32(x) - delta);
    } else if mode & 0x30 == 0x20 {
        platform::write_f32(x, (platform::read_f32(x) as f64 - width) as f32);
    }
    if mode & 0x300 == 0x100 {
        let delta = (height * centre) as f32;
        platform::write_f32(y, platform::read_f32(y) - delta);
    } else if mode & 0x300 == 0x200 {
        platform::write_f32(y, (platform::read_f32(y) as f64 - height) as f32);
    }

    let mode = platform::read_u32(stream + TEXT_STREAM_MODE);
    if mode & 3 == 1 {
        let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
        let base = snapshot.as_mut_ptr() as usize;
        copy_text_stream_without_pen(base, stream);
        let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
        let rect_ptr = rect.as_mut_ptr() as usize;
        let mut cursor = crate::platform::GuestLocal::new( start);
        platform::call4(
            TEXT_ROTATED_MEASURE,
            base as u32,
            rect_ptr as u32,
            (cursor.as_mut_ptr() as *mut u32) as u32,
            count,
        );
        let measured = (rect[2] - rect[0]) as f64;
        platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
        let delta = ((width - measured) as f32) * centre as f32;
        platform::write_f32(stream + TEXT_STREAM_PEN_X, platform::read_f32(x) + delta);
    } else if mode & 3 == 2 {
        let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
        let base = snapshot.as_mut_ptr() as usize;
        copy_text_stream_without_pen(base, stream);
        let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
        let rect_ptr = rect.as_mut_ptr() as usize;
        let mut cursor = crate::platform::GuestLocal::new( start);
        platform::call4(
            TEXT_ROTATED_MEASURE,
            base as u32,
            rect_ptr as u32,
            (cursor.as_mut_ptr() as *mut u32) as u32,
            count,
        );
        let measured = (rect[2] - rect[0]) as f64;
        platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
        platform::write_f32(
            stream + TEXT_STREAM_PEN_X,
            platform::read_f32(x) + (width - measured) as f32,
        );
    } else {
        platform::write_f32(stream + TEXT_STREAM_PEN_X, platform::read_f32(x));
    }

    if mode & 0x300 == 0x300 {
        platform::write_f32(stream + TEXT_STREAM_PEN_Y, platform::read_f32(y));
    } else {
        let height_of = crate::platform::guest_fn!(TEXT_RUN_HEIGHT, fn(u32) -> f32);
        let extra = height_of(stream as u32) as f64;
        platform::write_f32(
            stream + TEXT_STREAM_PEN_Y,
            (platform::read_f32(y) as f64 + extra) as f32,
        );
    }

    width as f32
}

/// Anchors a measured console-text run in its box and publishes the pen.
///
/// In plain English: the console twin of [`fn_80230280`] - same anchor masks,
/// same pen writes, but it measures through [`fn_802322c0`] and lays the
/// rotated modes out through [`fn_80231cf0`].
///
/// # Original
/// - Ghidra: `FUN_80232C10` at `0x80232C10` (1240 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80232c10_FUN_80232c10.c`
/// - `// observed:` the body is byte-identical to `0x80230280` apart from the
///   two callees: `0x802322C0` replaces `0x8022F960` and `0x80231CF0`
///   replaces `0x8022F350`.
/// - `// observed:` the rotated modes differ from [`fn_80230280`] in the pen
///   formula: mode 2 has no `r2 - 0x4C28` multiplier at all.
///
/// # Arguments
/// As [`fn_80230280`]; `start`/`count` are the console run indices that
/// [`fn_802322c0`] expects.
///
/// # Returns
/// The measured width, read from `f1` by [`fn_802323e0`].
///
/// # Side effects
/// As [`fn_80230280`], calling `0x802322C0` and `0x80231CF0` instead.
///
/// # Safety
/// As [`fn_80230280`].
///
/// # Confidence
/// medium - the twin relation is exact (only the callee addresses differ) and
/// every store is literal; the callee roles are shared with [`fn_80230280`].
pub unsafe fn fn_80232c10(stream: usize, x: usize, y: usize, start: u32, count: u32) -> f32 {
    let zero = platform::read_f32(SDA2_TEXT_ZERO) as f64;
    let centre = platform::read_f32(SDA2_TEXT_CENTRE) as f64;
    let mode = platform::read_u32(stream + TEXT_STREAM_MODE);
    let masked = mode & 0x333;

    let mut width = zero;
    let mut height = zero;
    if masked != 0x300 && masked != 0 {
        let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
        let base = snapshot.as_mut_ptr() as usize;
        copy_text_stream_without_pen(base, stream);
        let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
        let rect_ptr = rect.as_mut_ptr() as usize;
        fn_802322c0(base, rect_ptr, start, count as i32);
        platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
        width = (rect[0] + rect[2]) as f64;
        height = (rect[1] + rect[3]) as f64;
    }

    if mode & 0x30 == 0x10 {
        let delta = (width * centre) as f32;
        platform::write_f32(x, platform::read_f32(x) - delta);
    } else if mode & 0x30 == 0x20 {
        platform::write_f32(x, (platform::read_f32(x) as f64 - width) as f32);
    }
    if mode & 0x300 == 0x100 {
        let delta = (height * centre) as f32;
        platform::write_f32(y, platform::read_f32(y) - delta);
    } else if mode & 0x300 == 0x200 {
        platform::write_f32(y, (platform::read_f32(y) as f64 - height) as f32);
    }

    let mode = platform::read_u32(stream + TEXT_STREAM_MODE);
    if mode & 3 == 1 {
        let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
        let base = snapshot.as_mut_ptr() as usize;
        copy_text_stream_without_pen(base, stream);
        let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
        let rect_ptr = rect.as_mut_ptr() as usize;
        let mut cursor = crate::platform::GuestLocal::new( start);
        fn_80231cf0(base, rect_ptr, (cursor.as_mut_ptr() as *mut u32) as usize, count as i32);
        let measured = (rect[2] - rect[0]) as f64;
        platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
        let delta = ((width - measured) as f32) * centre as f32;
        platform::write_f32(stream + TEXT_STREAM_PEN_X, platform::read_f32(x) + delta);
    } else if mode & 3 == 2 {
        let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
        let base = snapshot.as_mut_ptr() as usize;
        copy_text_stream_without_pen(base, stream);
        let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
        let rect_ptr = rect.as_mut_ptr() as usize;
        let mut cursor = crate::platform::GuestLocal::new( start);
        fn_80231cf0(base, rect_ptr, (cursor.as_mut_ptr() as *mut u32) as usize, count as i32);
        let measured = (rect[2] - rect[0]) as f64;
        platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
        platform::write_f32(
            stream + TEXT_STREAM_PEN_X,
            platform::read_f32(x) + (width - measured) as f32,
        );
    } else {
        platform::write_f32(stream + TEXT_STREAM_PEN_X, platform::read_f32(x));
    }

    if mode & 0x300 == 0x300 {
        platform::write_f32(stream + TEXT_STREAM_PEN_Y, platform::read_f32(y));
    } else {
        let height_of = crate::platform::guest_fn!(TEXT_RUN_HEIGHT, fn(u32) -> f32);
        let extra = height_of(stream as u32) as f64;
        platform::write_f32(
            stream + TEXT_STREAM_PEN_Y,
            (platform::read_f32(y) as f64 + extra) as f32,
        );
    }

    width as f32
}

/// Runs the console frame pump until its per-frame callback asks to stop.
///
/// In plain English: "report the exception record, paint the console, then spin
/// on the retrace callback; when no callback is installed, keep repainting at a
/// fixed 200 ms pace forever."
///
/// # Original
/// - Ghidra: `FUN_802348F0` at `0x802348F0` (400 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802348f0_FUN_802348f0.c`
/// - `// observed:` the 16-byte record `{u16 kind, u16 pad, screen, arg3, arg4}`
///   is read from the prologue stub `FUN_802DCD64`; kind below `0x11` copies
///   `screen + 4` into `0x803DA89C`.
/// - `// observed:` the callback `0x803DA8A8` is called repeatedly while the
///   frame tick `0x803DA8B8` does not change, and a zero result returns at once.
/// - `// observed:` the no-callback path calls `0x802345C0` and `0x802340A0`
///   and paces itself with `(OSGetTime() - start) / (bus_clock / 4000)` against
///   a 200 unit bound; the export's `FUN_802DCEC4` is `__div2u`.
///
/// # Arguments
/// - `record`: the 0x10-byte frame record built by [`fn_80234720`].
///
/// # Side effects
/// Writes the console globals, may allocate the framebuffer through
/// [`fn_80233a70`], paints, and calls the installed retrace callback.
///
/// # Safety
/// `record` must be readable for 0x10 bytes; the screen record and the console
/// objects it names must be live while the callback chain runs. The no-callback
/// path never returns, exactly like the original.
///
/// # Confidence
/// medium - the record fields, the callback protocol and the pacing arithmetic
/// are literal. The Metrowerks `__div2u` helper is replaced by native `u64`
/// division, which is exact for the unsigned operands the export builds.
pub unsafe fn fn_802348f0(record: usize) {
    let kind = platform::read_u16(record) as u32;
    let screen = platform::read_u32(record + 4);
    let arg3 = platform::read_u32(record + 8);
    let arg4 = platform::read_u32(record + 0x0C);

    if kind < EXC_KIND_LIMIT {
        platform::write_u32(CONSOLE_LAST_FRAME, platform::read_u32(screen as usize + 4));
    }
    if platform::read_u32(CONSOLE_ACTIVE) != 0 {
        let mode = platform::read_u32(CONSOLE_SCREEN) as usize;
        platform::write_u32(CONSOLE_ORIGIN, fn_80233a70(mode));
    }

    loop {
        fn_80234b60(kind, screen as usize, arg3, arg4);
        let mut active = platform::read_u32(CONSOLE_ACTIVE);
        if active != 0 && fn_80233260() != 0 {
            platform::call1(CONSOLE_PAINT_PASS, active);
        }
        let tick = platform::read_u16(CONSOLE_TICK);
        if platform::read_u32(CONSOLE_CALLBACK) == 0 {
            break;
        }
        loop {
            active = platform::read_u32(CONSOLE_ACTIVE);
            let frame_callback = crate::platform::guest_fn!(platform::read_u32(CONSOLE_CALLBACK) as usize, fn(u32, u32) -> u32);
            let result = frame_callback(active, platform::read_u32(CONSOLE_CALLBACK_CTX));
            if result == 0 {
                return;
            }
            if tick != platform::read_u16(CONSOLE_TICK) {
                break;
            }
        }
    }

    if platform::read_u32(CONSOLE_ACTIVE) != 0 {
        let active = platform::read_u32(CONSOLE_ACTIVE) as usize;
        let mut line = platform::read_i32(active + 0x14);
        loop {
            let height = platform::call1(CONSOLE_QUEUE_HEIGHT, active as u32) as i32;
            if height <= line {
                line = platform::read_i32(active + 0x14);
            }
            platform::write_i32(active + 0x18, line);
            platform::call1(CONSOLE_PAINT_PASS, active as u32);

            let start = platform::call_u64(OS_GET_TIME);
            let divider = (platform::read_u32(BUS_CLOCK) / 4000) as u64;
            loop {
                let now = platform::call_u64(OS_GET_TIME);
                let elapsed = now.wrapping_sub(start);
                // observed: __div2u(hi, lo, 0, bus_clock / 4000) -> milliseconds.
                let millis = if divider == 0 { 0 } else { elapsed / divider };
                if !(millis < CONSOLE_FRAME_MILLISECONDS) {
                    break;
                }
            }
            line += 1;
        }
    }
}

/// Lays out one run of console text and merges its boxes into a rectangle.
///
/// In plain English: "walk the run one character at a time through the layout
/// object, merging every glyph box into the caller's rectangle, wrapping at the
/// height limit, and leaving the cursor where the run stopped."
///
/// # Original
/// - Ghidra: `FUN_80231CF0` at `0x80231CF0` (1468 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80231cf0_FUN_80231cf0.c`
/// - `// observed:` the loop stops when `start + count * 2 < cursor`, so the
///   cursor moves in two-byte steps and `count` is a character count.
/// - `// observed:` control characters (`< 0x20`) go through the layout slot
///   `+0x10`; printable ones advance the pen by the source slot `+0x48` result
///   scaled by `stream[+0x24]`, or by the constant `stream[+0x44]` when the
///   byte at `+0x43` is set.
/// - `// observed:` the layout slot returns 1 (clear the "last was printable"
///   flag), 2 (set it), 3 (stop and publish the cursor) or 4 (skip `count * 2`
///   bytes and stop).
/// - `// observed:` the `u64` scratch pairs around `0x43300000` are the
///   published `u32 -> f64` conversion; they are written as `as f64` here.
/// - `// observed:` the five incoming paired-single pairs (`f27`..`f31`) are
///   stored to the frame and never read again.
///
/// # Arguments
/// - `stream`: the 0x64-byte console stream whose pen words are advanced.
/// - `rect`: four writable floats `{x0, y0, x1, y1}` merged in place.
/// - `cursor`: a `u32` in/out character cursor (byte offset into the run).
/// - `count`: number of characters to lay out.
///
/// # Returns
/// The export's `r3`: `1` when the run had to wrap or reach the limit, `0`
/// otherwise.
///
/// # Side effects
/// Advances the stream pen at `+0x2C`, writes `cursor` and `rect`, copies the
/// stream three times into the frame, and disposes each copy with `0x8022C980`.
///
/// # Safety
/// `stream` must be a live 0x64-byte record with a valid layout object at
/// `+0x60` and a source at `+0x48`; `rect` must be writable for 16 bytes and
/// `cursor` for 4.
///
/// # Confidence
/// low - every branch, mask, offset and call is literal. What the two opaque
/// cursor blocks passed to slot `+0x10` contain is `// unknown:`; they are
/// filled in the order the export assigns them and never read back. The
/// character reader `0x802DCBFC` is opaque (a 40-byte dispatch veneer) and is
/// called by address, so the two-byte step stays inside the original callee.
pub unsafe fn fn_80231cf0(stream: usize, rect: usize, cursor: usize, count: i32) -> u32 {
    let zero = platform::read_f32(SDA2_TEXT_ZERO) as f64;
    let limit = platform::read_f32(SDA2_TEXT_LIMIT) as f64;
    let start = platform::read_u32(cursor);
    let layout = platform::read_u32(stream + TEXT_STREAM_LAYOUT) as usize;
    let source = platform::read_u32(stream + TEXT_STREAM_SOURCE) as usize;
    let tall = (platform::read_f32(stream + 0x4C) as f64) < limit;

    let mut iterator = crate::platform::GuestLocal::new( [0_u32; 4]);
    let iterator_ptr = iterator.as_mut_ptr() as usize;
    platform::call2(TEXT_SOURCE_ITERATOR_INIT, iterator_ptr as u32, source as u32);

    platform::write_f32(rect, zero as f32);
    platform::write_f32(rect + 8, zero as f32);

    let first = if source == 0 {
        0
    } else {
        let slot = platform::read_u32(source) as usize;
        call0_word(platform::read_u32(slot + 0x34) as usize)
    };
    let extent = platform::read_f32(stream + 0x54) + (first as f32) * platform::read_f32(stream + 0x28);
    platform::write_f32(rect + 4, if extent < 0.0 { extent } else { 0.0 });

    // observed: the export calls the same slot a second time for the far edge.
    let second = if source == 0 {
        0
    } else {
        let slot = platform::read_u32(source) as usize;
        call0_word(platform::read_u32(slot + 0x34) as usize)
    };
    let extent = platform::read_f32(stream + 0x54) + (second as f32) * platform::read_f32(stream + 0x28);
    platform::write_f32(rect + 12, if 0.0 < extent { extent } else { 0.0 });

    platform::write_u32(iterator_ptr, start);
    let mut ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;
    let mut pen = zero;
    let mut printable = 0_i32;
    let mut line_start = 0_u32;
    let mut result = 0_u32;

    loop {
        if start.wrapping_add((count as u32).wrapping_mul(2)) < platform::read_u32(iterator_ptr) {
            platform::write_u32(cursor, platform::read_u32(iterator_ptr));
            return result;
        }

        let mut restart = false;
        if ch < 0x20 {
            let saved_cursor = platform::read_u32(iterator_ptr);
            platform::write_f32(stream + TEXT_STREAM_PEN_X, pen as f32);
            if tall && ch != 10 && line_start != 0 {
                let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
                let base = snapshot.as_mut_ptr() as usize;
                for offset in 0..0x64 {
                    platform::write_u8(base + offset, platform::read_u8(stream + offset));
                }
                // unknown: the two words of the cursor block; the export passes
                // the copy pointer and the "previous was printable" flag.
                let mut block = crate::platform::GuestLocal::new( [base as u32, if printable == 0 { 1 } else { 0 }]);
                let block_ptr = block.as_mut_ptr() as usize;
                let mut probe = crate::platform::GuestLocal::new( [zero as f32; 4]);
                let probe_ptr = probe.as_mut_ptr() as usize;
                let slot = platform::read_u32(layout) as usize;
                let method = platform::read_u32(slot + 0x10) as usize;
                platform::call4(
                    method,
                    layout as u32,
                    probe_ptr as u32,
                    ch,
                    block_ptr as u32,
                );
                let advance = (probe[2] - probe[0]) as f64;
                let pen_after = platform::read_f32(base + TEXT_STREAM_PEN_X) as f64;
                if !(advance <= zero) && !(pen_after <= platform::read_f32(stream + 0x4C) as f64) {
                    result = 1;
                    ch = 10;
                    platform::write_u32(iterator_ptr, line_start);
                    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
                    restart = true;
                } else {
                    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
                }
            }

            if !restart {
                // unknown: as above; the export passes the stream pointer and
                // the "previous was printable" flag, the same pair it hands the
                // probe path with the snapshot pointer in the first word.
                let mut block = crate::platform::GuestLocal::new( [stream as u32, if printable == 0 { 1 } else { 0 }]);
                let block_ptr = block.as_mut_ptr() as usize;
                let mut glyph = crate::platform::GuestLocal::new( [pen as f32, zero as f32, zero as f32, zero as f32]);
                let glyph_ptr = glyph.as_mut_ptr() as usize;
                let slot = platform::read_u32(layout) as usize;
                let method = platform::read_u32(slot + 0x10) as usize;
                let kind = platform::call4(
                    method,
                    layout as u32,
                    glyph_ptr as u32,
                    ch,
                    block_ptr as u32,
                );
                platform::write_u32(iterator_ptr, saved_cursor);

                let left = platform::read_f32(rect) as f64;
                let left = if (glyph[0] as f64) < left { glyph[0] as f64 } else { left };
                platform::write_f32(rect, left as f32);
                let bottom = platform::read_f32(rect + 4) as f64;
                let bottom = if (glyph[1] as f64) < bottom { glyph[1] as f64 } else { bottom };
                platform::write_f32(rect + 4, bottom as f32);
                let right = platform::read_f32(rect + 8) as f64;
                let right = if right < glyph[2] as f64 { glyph[2] as f64 } else { right };
                platform::write_f32(rect + 8, right as f32);
                let top = platform::read_f32(rect + 12) as f64;
                let top = if top < glyph[3] as f64 { glyph[3] as f64 } else { top };
                platform::write_f32(rect + 12, top as f32);

                pen = platform::read_f32(stream + TEXT_STREAM_PEN_X) as f64;
                if kind == 4 {
                    let skipped = platform::read_u32(cursor).wrapping_add((count as u32).wrapping_mul(2));
                    platform::write_u32(cursor, skipped);
                    return 0;
                }
                if kind == 1 {
                    printable = 0;
                } else if kind == 2 {
                    printable = 1;
                    if tall {
                        line_start = platform::read_u32(iterator_ptr);
                    }
                    ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;
                    continue;
                } else if kind == 3 {
                    platform::write_u32(cursor, platform::read_u32(iterator_ptr));
                    return result;
                }

                if tall {
                    line_start = platform::read_u32(iterator_ptr);
                }
                ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;
                continue;
            }
            continue;
        }

        let mut advance = if printable != 0 {
            platform::read_f32(stream + 0x50) as f64
        } else {
            zero
        };
        if platform::read_u8(stream + 0x43) == 0 {
            let scale = platform::read_f32(stream + 0x24) as f64;
            let slot = platform::read_u32(source) as usize;
            let method = platform::read_u32(slot + 0x48) as usize;
            let steps = platform::call2(method, source as u32, ch);
            advance += ((steps as f32 as f64 * scale) as f32) as f64;
        } else {
            advance += platform::read_f32(stream + 0x44) as f64;
        }

        if tall && line_start != 0 && (platform::read_f32(stream + 0x4C) as f64) < pen + advance {
            result = 1;
            ch = 10;
            platform::write_u32(iterator_ptr, line_start);
            continue;
        }

        pen = ((pen + advance) as f32) as f64;
        let left = platform::read_f32(rect) as f64;
        let left = if left < pen { left } else { pen };
        platform::write_f32(rect, left as f32);
        let right = platform::read_f32(rect + 8) as f64;
        let right = if pen < right { right } else { pen };
        platform::write_f32(rect + 8, right as f32);
        printable = 1;

        if tall {
            line_start = platform::read_u32(iterator_ptr);
        }
        ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;
    }
}

/// Runs one console text run through the layout object and the page source.
///
/// In plain English: "anchor the run with [`fn_80232c10`], then feed it to the
/// layout object character by character - wrapping at the height limit,
/// scrolling the page for control characters, and fixing up the pen at the
/// end."
///
/// # Original
/// - Ghidra: `FUN_802323E0` at `0x802323E0` (2088 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802323e0_FUN_802323e0.c`
/// - `// observed:` the prologue stub `FUN_802DCD5C` carries `r4` in its low
///   word and `r3` in its high word, so the body's `puVar6` is the stream
///   (`r3`) and `iVar9` is the text base (`r4`).
/// - `// observed:` the first act is
///   `0x80232C10(stream, &pen_x_copy, &pen_y_copy, text, length)`; the returned
///   width drives the end-of-run pen adjustment, and `pen_y_before - pen_y`
///   becomes the vertical shift added back when `mode` is zero.
/// - `// observed:` the run walks `0x802DCBFC` two-byte characters while
///   `length >= (cursor - text) / 2`; slot `+0x0C` of the layout object returns
///   1/2/3/4 (clear flag, set flag, finish, flush) and advances the cursor
///   through the two-word block it is handed.
/// - `// observed:` on 3 the body re-enters [`fn_80231cf0`] with the remaining
///   count for the two rotated modes, and copies `extent - measured` back into
///   the pen.
/// - `// observed:` the ten incoming paired-single pairs (`f22`..`f31`) are
///   stored to the frame and never read again.
/// - `// observed:` the `0x43300000` scratch pairs are the published
///   `u32`/`i32 -> f64` conversions; they are written as `as f64` here, with
///   the signed slot `+0x1C` negated before the conversion.
///
/// # Arguments
/// - `stream`: the 0x64-byte console stream; pen X/Y at `+0x2C`/`+0x30`, mode
///   word at `+0x5C`, layout object at `+0x60`, page source at `+0x48`.
/// - `text`: base address of the two-byte character run.
/// - `length`: number of characters in the run.
/// - `mode`: the caller's flag (`0` = add the vertical shift, non-zero = keep
///   the anchored pen).
///
/// # Returns
/// The largest run extent seen, which the export's tail hands to the restore
/// stub and the callers ignore.
///
/// # Side effects
/// Advances the pen words, copies the stream up to three times into the frame,
/// disposes every copy with `0x8022C980`, and calls `0x8022D5F0` per printable
/// character.
///
/// # Safety
/// `stream` must be a live record with a valid layout object and page source,
/// and `text` must be readable for `2 * length` bytes.
///
/// # Confidence
/// low - every branch, mask, offset and call is literal. What the virtual slots
/// `+0x0C`/`+0x10`/`+0x1C`/`+0x48` do is `// unknown:`; only their argument
/// shapes and return registers are transcribed. The two-word cursor block is
/// filled as `{stream, cursor}`, the order the export builds in its `u64`
/// scratch, and is `// unknown:` beyond that.
pub unsafe fn fn_802323e0(stream: usize, text: usize, length: i32, mode: i32) -> f32 {
    let zero = platform::read_f32(SDA2_TEXT_ZERO) as f64;
    let centre = platform::read_f32(SDA2_TEXT_CENTRE) as f64;
    let limit = platform::read_f32(SDA2_TEXT_LIMIT) as f64;
    let layout = platform::read_u32(stream + TEXT_STREAM_LAYOUT) as usize;
    let source = platform::read_u32(stream + TEXT_STREAM_SOURCE) as usize;
    let tall = (platform::read_f32(stream + 0x4C) as f64) < limit;

    let mut anchor_x = crate::platform::GuestLocal::new( platform::read_f32(stream + TEXT_STREAM_PEN_X));
    let mut anchor_y = crate::platform::GuestLocal::new( platform::read_f32(stream + TEXT_STREAM_PEN_Y));
    let mut pen_y_before = *anchor_y as f64;
    let mut extent = fn_80232c10(
        stream,
        (anchor_x.as_mut_ptr() as *mut f32) as usize,
        (anchor_y.as_mut_ptr() as *mut f32) as usize,
        text as u32,
        length as u32,
    ) as f64;
    let mut y_shift =
        (pen_y_before - platform::read_f32(stream + TEXT_STREAM_PEN_Y) as f64) as f32 as f64;

    let mut iterator = crate::platform::GuestLocal::new( [0_u32; 4]);
    let iterator_ptr = iterator.as_mut_ptr() as usize;
    platform::call2(TEXT_SOURCE_ITERATOR_INIT, iterator_ptr as u32, source as u32);
    platform::write_u32(iterator_ptr, text as u32);
    let mut ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;

    let mut line_start = text as u32;
    let mut last_finish = text as u32;
    let mut printable = 0_i32;
    // unknown: the layout cursor block; the export keeps it in its `u64`
    // scratch with the stream pointer in the low word and the cursor in the
    // high word.
    let mut block = crate::platform::GuestLocal::new( [stream as u32, text as u32]);

    loop {
        block[0] = stream as u32;
        let index = (platform::read_u32(iterator_ptr).wrapping_sub(text as u32) as i32) / 2;
        if length < index {
            let advance = platform::read_f32(stream + TEXT_STREAM_PEN_X) as f64 - *anchor_x as f64;
            if extent < advance {
                extent = advance;
            }
            let anchor = platform::read_u32(stream + TEXT_STREAM_MODE) & 0x300;
            if anchor == 0x100 || anchor == 0x200 {
                platform::write_f32(stream + TEXT_STREAM_PEN_Y, pen_y_before as f32);
            } else if mode == 0 {
                let shifted = platform::read_f32(stream + TEXT_STREAM_PEN_Y) as f64 + y_shift;
                platform::write_f32(stream + TEXT_STREAM_PEN_Y, shifted as f32);
            } else if anchor == 0 {
                pen_y_before = platform::read_f32(stream + TEXT_STREAM_PEN_Y) as f64;
                let height_of = crate::platform::guest_fn!(TEXT_RUN_HEIGHT, fn(u32) -> f32);
                y_shift = height_of(stream as u32) as f64;
                platform::write_f32(
                    stream + TEXT_STREAM_PEN_Y,
                    (pen_y_before - y_shift) as f32,
                );
            }
            return extent as f32;
        }

        if ch < 0x20 {
            let has_previous = if printable == 0 { 1 } else { 0 };
            block[1] = platform::read_u32(iterator_ptr);
            if tall && ch != 10 && line_start != last_finish {
                let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
                let base = snapshot.as_mut_ptr() as usize;
                for offset in 0..0x64 {
                    platform::write_u8(base + offset, platform::read_u8(stream + offset));
                }
                let mut probe = crate::platform::GuestLocal::new( [zero as f32; 4]);
                let probe_ptr = probe.as_mut_ptr() as usize;
                let mut probe_block = crate::platform::GuestLocal::new( [base as u32, has_previous]);
                let probe_block_ptr = probe_block.as_mut_ptr() as usize;
                let slot = platform::read_u32(layout) as usize;
                let method = platform::read_u32(slot + 0x10) as usize;
                platform::call4(
                    method,
                    layout as u32,
                    probe_ptr as u32,
                    ch,
                    probe_block_ptr as u32,
                );
                let width = (probe[2] - probe[0]) as f64;
                let advance =
                    platform::read_f32(base + TEXT_STREAM_PEN_X) as f64 - *anchor_x as f64;
                let overflow = (platform::read_f32(stream + 0x4C) as f64) < advance;
                platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
                if !(width <= zero) && overflow {
                    ch = 10;
                    platform::write_u32(iterator_ptr, line_start);
                    block[1] = line_start;
                    continue;
                }
            }

            let mut glyph = crate::platform::GuestLocal::new( [zero as f32; 4]);
            let glyph_ptr = glyph.as_mut_ptr() as usize;
            let block_ptr = block.as_mut_ptr() as usize;
            let slot = platform::read_u32(layout) as usize;
            let method = platform::read_u32(slot + 0x0C) as usize;
            let kind = platform::call3(method, layout as u32, ch, block_ptr as u32);

            if kind == 3 {
                let mode_word = platform::read_u32(stream + TEXT_STREAM_MODE);
                if mode_word & 3 == 1 {
                    let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
                    let base = snapshot.as_mut_ptr() as usize;
                    copy_text_stream_without_pen(base, stream);
                    let mut cursor = crate::platform::GuestLocal::new( block[1]);
                    let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
                    let rect_ptr = rect.as_mut_ptr() as usize;
                    let remaining = length
                        - (block[1].wrapping_sub(text as u32) as i32) / 2;
                    fn_80231cf0(
                        base,
                        rect_ptr,
                        (cursor.as_mut_ptr() as *mut u32) as usize,
                        remaining,
                    );
                    let measured = (rect[2] - rect[0]) as f64;
                    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
                    let delta = (((extent - measured) as f32) as f64 * centre) as f32;
                    platform::write_f32(
                        stream + TEXT_STREAM_PEN_X,
                        *anchor_x + delta,
                    );
                } else if mode_word & 3 == 2 {
                    let mut snapshot = crate::platform::GuestLocal::new( [0_u32; 0x19]);
                    let base = snapshot.as_mut_ptr() as usize;
                    copy_text_stream_without_pen(base, stream);
                    let mut cursor = crate::platform::GuestLocal::new( block[1]);
                    let mut rect = crate::platform::GuestLocal::new( [zero as f32; 4]);
                    let rect_ptr = rect.as_mut_ptr() as usize;
                    let remaining = length
                        - (block[1].wrapping_sub(text as u32) as i32) / 2;
                    fn_80231cf0(
                        base,
                        rect_ptr,
                        (cursor.as_mut_ptr() as *mut u32) as usize,
                        remaining,
                    );
                    let measured = (rect[2] - rect[0]) as f64;
                    platform::call2(RES_OBJECT_DISPOSE, base as u32, 0);
                    platform::write_f32(
                        stream + TEXT_STREAM_PEN_X,
                        *anchor_x + (extent - measured) as f32,
                    );
                } else {
                    let advance =
                        platform::read_f32(stream + TEXT_STREAM_PEN_X) as f64 - *anchor_x as f64;
                    if extent < advance {
                        extent = advance;
                    }
                    platform::write_f32(stream + TEXT_STREAM_PEN_X, *anchor_x);
                }
                if tall {
                    last_finish = platform::read_u32(iterator_ptr);
                }
                printable = 0;
            } else if kind == 1 {
                printable = 0;
            } else if kind == 2 {
                printable = 1;
            } else if kind == 4 {
                let advance =
                    platform::read_f32(stream + TEXT_STREAM_PEN_X) as f64 - *anchor_x as f64;
                if extent < advance {
                    extent = advance;
                }
                let anchor = platform::read_u32(stream + TEXT_STREAM_MODE) & 0x300;
                if anchor == 0x100 || anchor == 0x200 {
                    platform::write_f32(stream + TEXT_STREAM_PEN_Y, pen_y_before as f32);
                } else if mode == 0 {
                    let shifted = platform::read_f32(stream + TEXT_STREAM_PEN_Y) as f64 + y_shift;
                    platform::write_f32(stream + TEXT_STREAM_PEN_Y, shifted as f32);
                } else if anchor == 0 {
                    pen_y_before = platform::read_f32(stream + TEXT_STREAM_PEN_Y) as f64;
                    let height_of = crate::platform::guest_fn!(TEXT_RUN_HEIGHT, fn(u32) -> f32);
                    y_shift = height_of(stream as u32) as f64;
                    platform::write_f32(
                        stream + TEXT_STREAM_PEN_Y,
                        (pen_y_before - y_shift) as f32,
                    );
                }
                return extent as f32;
            }

            platform::write_u32(iterator_ptr, block[1]);
            if tall {
                line_start = platform::read_u32(iterator_ptr);
            }
            ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;
            continue;
        }

        // Printable character: the page advance is only probed when the run
        // started a fresh line.
        let saved_pen_y = platform::read_f32(stream + TEXT_STREAM_PEN_Y) as f64;
        if tall && line_start != last_finish {
            let pen_x = platform::read_f32(stream + TEXT_STREAM_PEN_X) as f64;
            let step = if printable == 0 {
                0.0_f64
            } else {
                platform::read_f32(stream + 0x50) as f64
            };
            let advance = if platform::read_u8(stream + 0x43) == 0 {
                let scale = platform::read_f32(stream + 0x24) as f64;
                let slot = platform::read_u32(source) as usize;
                let method = platform::read_u32(slot + 0x48) as usize;
                let steps = platform::call2(method, source as u32, ch);
                ((steps as f32 as f64 * scale) as f32) as f64
            } else {
                platform::read_f32(stream + 0x44) as f64
            };
            let projected = (advance as f32)
                + ((step + ((pen_x - *anchor_x as f64) as f32) as f64) as f32);
            if !(projected <= platform::read_f32(stream + 0x4C)) {
                ch = 10;
                platform::write_u32(iterator_ptr, line_start);
                block[1] = line_start;
                continue;
            }
        }

        if printable != 0 {
            let pen = platform::read_f32(stream + TEXT_STREAM_PEN_X);
            platform::write_f32(
                stream + TEXT_STREAM_PEN_X,
                pen + platform::read_f32(stream + 0x50),
            );
        }
        printable = 1;

        let scale = platform::read_f32(stream + 0x28) as f64;
        let slot = platform::read_u32(source) as usize;
        let method = platform::read_u32(slot + 0x1C) as usize;
        let steps = call0_word(method) as i32;
        let shift = ((-(steps as f64)) as f32 as f64 * scale) as f32;
        let pen_y = platform::read_f32(stream + TEXT_STREAM_PEN_Y);
        platform::write_f32(stream + TEXT_STREAM_PEN_Y, pen_y + shift);
        platform::call2(TEXT_RUN_SCROLL, stream as u32, ch);
        platform::write_f32(stream + TEXT_STREAM_PEN_Y, saved_pen_y as f32);

        if tall {
            line_start = platform::read_u32(iterator_ptr);
        }
        ch = interface_call_desc(Descriptor::new(iterator_ptr + 4), iterator_ptr as u32) as u16 as u32;
    }
}

// ===========================================================================
// Wave 109: the script-symbol parser and the beam-matrix wrapper.
//
// Two of the five addresses wave 106 left in this bucket are answered here.
// Both needed decoding facts the earlier passes had already established:
//
//   * `0x802DCD64` is the `_savegpr0_23` prologue stub, not a call: its
//     "return" is the incoming `r3:r4` pair with `r4` in the low word and `r3`
//     in the high word. That is what gives `0x80235AE0` its real
//     `(script, block, address, out, size)` shape; the export appears to start
//     with an argument-less call and two undefined words because of it.
//   * the two argument lists Ghidra dropped in `0x80235AE0` belong to
//     `0x80235A10`, and in both cases the argument is the token address the
//     preceding `0x80235940` just returned. `fn_80235dc0` calls the same pair
//     the same way, which is the evidence for the recovery.
//
// `0x80238930` is small but needs double-register calls: `0x802E8590` is the
// Metrowerks `tan` and `0x802389F0` takes two pointer words plus five doubles.
// [`crate::platform`] only exposes word-shaped calls, so this section adds two
// local `extern "C"` pointer helpers instead of touching the shared layer.
//
// The three addresses that stay skipped are `0x802382B0`, `0x802389F0` and
// `0x802390D0`; `docs/COVERAGE_WAVE109_BRES.md` records why.
// ===========================================================================

/// `r2 - 0x4B98` - `f32` 0.024543693, one step of the 256-entry angle table.
///
/// `// observed:` the word at `0x8042E4E8` is `0x3CC90FDB`, i.e. `2*pi/256`.
const SDA2_TABLE_RADIAN: usize = SDA2_BASE - 0x4B98;
/// `r2 - 0x4BA0` - `f32` 0.5, the half-angle factor of the beam wrapper.
///
/// `// observed:` `0x8042E4E0` holds `0x3F000000`. [`SDA2_MATH_HALF`] names the
/// same address for the math band and reads it as 0.5; `0.5 * (2*pi/256) *
/// (256/360)` is exactly one degree in radians.
const SDA2_BEAM_HALF_ANGLE: usize = SDA2_BASE - 0x4BA0;

/// Script block table: `u32` number of 8-byte block records.
const SCRIPT_BLOCK_COUNT: usize = 0x0C;
/// Script block table: pointer to the first 8-byte block record.
const SCRIPT_BLOCK_RECORDS: usize = 0x10;
/// Script block record: `u32` first address covered by the block.
const SCRIPT_BLOCK_START: usize = 0x00;
/// Script block record: `u32` size of the block in bytes.
const SCRIPT_BLOCK_SIZE: usize = 0x04;
/// Script block record: stride from one record to the next.
const SCRIPT_BLOCK_STRIDE: usize = 0x08;
/// The separator `0x80235AE0` passes to [`fn_80235940`] (an ASCII space).
const SCRIPT_TOKEN_SEPARATOR: u32 = 0x20;
/// ASCII `.`, the character that marks a section line in the script text.
const SCRIPT_SECTION_MARK: u32 = 0x2E;

/// Moves a script cursor past the requested number of newlines.
///
/// In plain English: "walk the byte reader until it has seen `lines` newlines,
/// step over the last one, and report 0 if the script ended first."
///
/// `// observed:` `0x80235AE0` contains four copies of the same loop - a
/// guard that leaves a zero cursor alone, a down-counter seeded with the number
/// of lines, decremented on `'\n'`, exiting once it drops below one - and each
/// copy ends by writing 0 into the cursor when the reader returns a NUL.
///
/// # Arguments
/// - `cursor`: first byte to inspect; 0 is returned unchanged.
/// - `lines`: newlines to consume (1 or 3 at the call sites).
///
/// # Returns
/// The cursor just past the last consumed newline, or 0 at end of script.
///
/// # Safety
/// [`SCRIPT_READ_CHAR`] must hold the live reader callback.
unsafe fn script_skip_lines(cursor: u32, lines: i32) -> u32 {
    if cursor == 0 {
        return 0;
    }
    let mut address = cursor;
    let mut remaining = lines;
    loop {
        let c = script_read_char(address) & 0xFF;
        if c == 0 {
            return 0;
        }
        if c == 10 {
            remaining -= 1;
            if remaining < 1 {
                return address.wrapping_add(1);
            }
        }
        address = address.wrapping_add(1);
    }
}

/// Calls one `f64(f64)` routine through a function pointer.
///
/// `// observed:` `0x802E8590` takes its angle in `f1` and returns in `f1`
/// (`resource::archive` records the same shape). [`crate::platform`] only
/// exposes word-register calls, so the two double-register calls of this band
/// are written as `extern "C"` pointers; this is the same helper shape
/// `resource::archive` uses.
///
/// # Safety
/// `addr` must be the entry point of a routine with that signature.
unsafe fn call_double(addr: usize, value: f64) -> f64 {
    let target = crate::platform::guest_fn!(addr, fn(f64) -> f64);
    target(value)
}

/// Calls `0x802389F0` with two opaque pointer words and five doubles.
///
/// `// observed:` the callee's export declares
/// `(undefined8, double, double, double, double, double)`, i.e. two words in
/// `r3`/`r4` followed by five doubles in `f1`..`f5`. The PowerPC EABI assigns
/// the integer and floating-point register files independently, so the Rust
/// parameter order below reproduces that mapping.
///
/// # Safety
/// `addr` must be the builder entry, and the two words must be the pointers it
/// dereferences while writing its matrix.
unsafe fn call_matrix_builder(
    addr: usize,
    first: usize,
    second: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
) {
    let target = crate::platform::guest_fn!(addr, fn(usize, usize, f64, f64, f64, f64, f64));
    target(first, second, a, b, c, d, e);
}

/// Finds the script entry that describes `address` and copies its name out.
///
/// In plain English: "walk the console script, section by section, and for each
/// `length offset ... name` line whose `offset` range contains the address,
/// copy the line's fifth token into the caller's buffer."
///
/// # Original
/// - Ghidra: `FUN_80235ae0` at `0x80235AE0` (716 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80235ae0_FUN_80235ae0.c`
/// - `// observed:` every byte comes through the callback in
///   [`SCRIPT_READ_CHAR`]; the body skips to a line beginning with `.`, skips
///   three more lines, and then tests each following line.
/// - `// observed:` the range test is
///   `base + offset <= address < base + offset + length`, where `base` is the
///   current block record's first word and `length`/`offset` are the
///   hexadecimal values of tokens 1 and 2 (`fn_80235a10`).
/// - `// observed:` the block table is `+0x0C` (count) and `+0x10` (pointer to
///   eight-byte `{start, size}` records); a null table disables the filter, and
///   the export's `do { ... } while (block == 0)` then re-scans instead of
///   advancing to the next record.
/// - `// observed:` the copy loop is printed after the `return`, but it is
///   reachable through the `goto` at `0x80235D20`; it copies the fifth token
///   until a newline or NUL and stops after `capacity - 1` bytes.
/// - `// inferred:` the first two arguments are the `r3:r4` pair of the
///   `_savegpr0_23` stub `0x802DCD64` (`r4` low, `r3` high). `r3` is the script
///   address, which `fn_80235dc0` passes as `0x80000000` or as the buffered
///   command word, and `r4` is `record[1]`, dereferenced here at `+0x0C`/`+0x10`.
/// - `// inferred:` the two argument lists Ghidra dropped are the token
///   addresses returned by the `fn_80235940` call on the line above each one,
///   the same pairing `fn_80235dc0` uses.
/// - `// unknown:` why the name token may not begin with `.`, and the meaning
///   of the fifth token beyond "the text to copy".
///
/// # Arguments
/// - `script`: first script byte (`r3`), the base the readers walk from.
/// - `block`: script record (`r4`); `+0x04` of the record reaches this word.
///   Zero selects the unfiltered scan.
/// - `address`: the address being symbolized.
/// - `out`: buffer that receives the name; always NUL-terminated on success.
/// - `capacity`: buffer size in bytes; the copy keeps `capacity - 1` characters.
///
/// # Returns
/// 1 when the buffer was written (including the empty name of a dangling
/// entry), 0 when no line matched or the text ended.
///
/// # Side effects
/// Writes `out` and calls the reader callback, `fn_80235940` and `fn_80235a10`.
///
/// # Safety
/// The reader callback must accept the script addresses, `out` must be writable
/// for `capacity` bytes, `block` (when nonzero) must point at a readable table,
/// and the block record must be readable for eight bytes.
///
/// # Confidence
/// low - every call, mask and store is literal, but the export lost three
/// argument lists, prints the copy loop after the `return`, and leaves the block
/// table's role to be inferred from the two offsets it reads.
pub unsafe fn fn_80235ae0(
    script: u32,
    block: usize,
    address: u32,
    out: usize,
    capacity: u32,
) -> u32 {
    let mut record = 0usize;
    let mut remaining = 0i32;
    if block != 0 {
        record = platform::read_u32(block + SCRIPT_BLOCK_RECORDS) as usize;
        remaining = platform::read_u32(block + SCRIPT_BLOCK_COUNT) as i32;
    }

    let mut cursor = script;
    'blocks: loop {
        // observed: `uVar6` (the block base) is zeroed at the top of every pass
        // and only assigned inside the range test.
        let mut base = 0u32;

        // Skip to a line that begins with '.'; 0 means the text ended.
        loop {
            cursor = script_skip_lines(cursor, 1);
            if cursor == 0 {
                break;
            }
            if script_read_char(cursor) & 0xFF == SCRIPT_SECTION_MARK {
                break;
            }
        }
        // observed: the section header occupies three lines before the entries.
        cursor = script_skip_lines(cursor, 3);

        let mut in_block = record == 0;
        if record != 0 {
            base = platform::read_u32(record + SCRIPT_BLOCK_START);
            let size = platform::read_u32(record + SCRIPT_BLOCK_SIZE);
            in_block = base <= address && address < base.wrapping_add(size);
        }

        if in_block {
            loop {
                cursor = script_skip_lines(cursor, 1);
                if cursor == 0 {
                    return 0;
                }
                let first = fn_80235940(cursor, 1, SCRIPT_TOKEN_SEPARATOR);
                if first == 0 {
                    // No first token: the entry list of this block is over.
                    break;
                }
                let length = fn_80235a10(first);
                let second = fn_80235940(cursor, 2, SCRIPT_TOKEN_SEPARATOR);
                if second == 0 {
                    break;
                }
                let offset = fn_80235a10(second);
                if offset != 0
                    && offset.wrapping_add(base) <= address
                    && address < offset.wrapping_add(base).wrapping_add(length)
                {
                    let text = fn_80235940(cursor, 5, SCRIPT_TOKEN_SEPARATOR);
                    if text == 0 {
                        platform::write_u8(out, 0);
                        return 1;
                    }
                    let mut c = script_read_char(text) & 0xFF;
                    if c != SCRIPT_SECTION_MARK {
                        // observed: the copy loop the export printed after its
                        // `return`; it starts at the fifth token and stops at a
                        // newline, a NUL, or after `capacity - 1` characters.
                        let mut dst = out;
                        let mut at = text;
                        let mut stored = 0u32;
                        loop {
                            if c == 10 || c == 0 {
                                platform::write_u8(dst, 0);
                                break;
                            }
                            stored = stored.wrapping_add(1);
                            platform::write_u8(dst, c as u8);
                            dst += 1;
                            at = at.wrapping_add(1);
                            if capacity.wrapping_sub(1) <= stored {
                                platform::write_u8(dst, 0);
                                break;
                            }
                            c = script_read_char(at) & 0xFF;
                        }
                        return 1;
                    }
                }
            }
        }

        // observed: end of this block record; a null table re-scans the text.
        if record == 0 {
            continue 'blocks;
        }
        remaining -= 1;
        if remaining != 0 {
            record += SCRIPT_BLOCK_STRIDE;
            continue 'blocks;
        }
        return 0;
    }
}

/// Builds the beam matrix for a half angle and a scale.
///
/// In plain English: "turn the caller's angle into radians, take the tangent of
/// half of it, scale it, and hand the four signed products plus the scale to
/// the matrix builder."
///
/// # Original
/// - Ghidra: `FUN_80238930` at `0x80238930` (180 bytes)
/// - Evidence: `work/ghidra/main-gb/export/80238930_FUN_80238930.c`
/// - `// observed:` the angle is reduced by
///   `x = (0x8042E4E8) * (0x8042E4EC) * (float)((0x8042E4E0) * angle)`, i.e.
///   `(2*pi/256) * (256/360) * (0.5 * angle)`; `0x802E8590` is the Metrowerks
///   `tan`, so `x` is half the caller's angle in radians.
/// - `// observed:` the tangent is narrowed to `f32`, multiplied by the scale
///   and narrowed again; the callee receives
///   `(s, -s, -(s * ratio), s * ratio, scale)` plus the two words of
///   `param_4`, which the export types as `undefined8`.
/// - `// observed:` the three `psq_l` probes at the top and bottom feed only
///   `ldexpf`, whose results are discarded; Ghidra removed the dead blocks at
///   `0x802389B8`, `0x802389C0` and `0x802389C8`.
/// - `// inferred:` the two words are the pointers `0x80259190` passes as
///   `auStack_100` (published through `r13 - 0x49B0`) and `auStack_130` (filled
///   by `0x802540B0`), i.e. the caller's output and input matrices.
/// - `// unknown:` the fourth float of that call site (`+0xB8`) is passed but
///   never read by the visible body.
///
/// # Arguments
/// - `angle`: rotation angle in the caller's 360-degree scale.
/// - `cos_ratio`: the second angle term; multiplies the tangent before the
///   pair is mirrored.
/// - `scale`: length multiplier applied to the tangent.
/// - `first`, `second`: the two opaque words forwarded to the builder.
///
/// # Returns
/// Nothing; the builder writes its matrix through `first`/`second`.
///
/// # Side effects
/// Calls the tangent helper and the builder at `0x802389F0`.
///
/// # Safety
/// `0x802E8590` and `0x802389F0` must be the mapped routines, and the two words
/// must satisfy whatever the builder writes through them.
///
/// # Confidence
/// medium - the arithmetic, the three constant addresses, the call shape and
/// the mirroring are literal; only the two pointer roles and the unused fourth
/// float are inferred.
pub unsafe fn fn_80238930(angle: f64, cos_ratio: f64, scale: f64, first: usize, second: usize) {
    // observed: r2 - 0x4B98 = 0x8042E4E8, f32 2*pi/256.
    let table_radian = platform::read_f32(SDA2_TABLE_RADIAN);
    // observed: r2 - 0x4B94 = 0x8042E4EC, f32 256/360.
    let degree_scale = platform::read_f32(SDA2_MTX_SCALE);
    // observed: r2 - 0x4BA0 = 0x8042E4E0, f32 0.5.
    let half = platform::read_f32(SDA2_BEAM_HALF_ANGLE);

    let radians = (table_radian * degree_scale) * ((half as f64 * angle) as f32);
    let tangent = call_double(G3D_TAN, radians as f64);
    let scaled = (((tangent as f32) as f64 * scale) as f32) as f64;
    let cross = (scaled * cos_ratio) as f32 as f64;

    call_matrix_builder(
        G3D_BILLBOARD,
        first,
        second,
        scaled,
        -scaled,
        -cross,
        cross,
        scale,
    );
}

// ===========================================================================
// Wave 111: the 3x4 rotation-matrix builder that the material path calls as
// `MTX34_SCALE` (see the wave-18 call sites above).
// ===========================================================================
//
// `0x802382B0` is another member of the packed-vector band recovered from the
// caller-set GQR state. Unlike the vector stores of wave 98, it needs *two*
// registers: GQR0 is the float format used for the sine/cosine table and the
// matrix stores, GQR3 is the quantisation used for the four-byte scratch
// value whose low byte selects a table entry. Both are passed in explicitly, so
// the caller supplies the values the original read from the registers.
//
// The table layout and the reduction are the same ones `fn_80237bd0` and
// `fn_80237ca0` already model, so this body calls `fn_80237ca0` once per angle
// instead of repeating the interpolator. The remaining arithmetic is the
// standard X-then-Y-then-Z product; it was cross-checked against
// `fn_802a0a90`'s combined arm, which writes the same twelve floats in the same
// order.

/// Builds a 3x4 rotation matrix from three angles.
///
/// In plain English: "wrap each angle into one turn, read the interpolated
/// sine/cosine pair from the 256-entry table, then write the row-major XYZ
/// rotation matrix."
///
/// # Original
/// - Ghidra: `FUN_802382b0` at `0x802382B0` (360 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802382b0_FUN_802382b0.c`
/// - `// observed:` the reduction is `while (period < |angle|) v -= period`
///   with `period = f32` at `r2 - 0x4BA8` (`0x8042E4D8`, `0x47800000` = 65536);
///   it is the same value as [`SDA2_TRIG_PERIOD`], which the table looked up by
///   [`fn_80237ca0`] reads.
/// - `// observed:` the table pair is read from `0x802F74A0 + index` with the
///   index `(stored_low_byte & 0xFF) * 0x10`; `+0`/`+4` are the sine/cosine
///   bases and `+8`/`+0xC` their interpolation slopes.
/// - `// observed:` the three reduced values are stored and reloaded through
///   GQR3 (`psq_st`/`psq_l` with `I=3`, both the two-lane and the one-lane form)
///   to obtain both the index word and the "back" value used as `v - back`.
/// - `// observed:` a negative angle negates its sine, never its cosine; the
///   three tests are `param_1 < 0`, `param_2 < 0`, `param_3 < 0`.
/// - `// observed:` the raw stores at `0x802383D0..0x8023840C` write
///   `[0..1]`, `[1..2]`, `[3..4]`, `[5..6]`, `[7..8]`, `[9..10]` and `[11]`.
///   Element `[8]` is the merge of a zero with `-sin_y` (`0x802383FC` negates
///   the Y pair before `ps_merge00`), so the export's `dVar14 - dVar14` for
///   that slot is a decompiler artefact; the matrix element is `-sin_y`.
/// - `// inferred:` the fourth argument is the destination and the first three
///   are X, Y and Z, in that order; the wave-18 callers pass
///   `(scale*x, scale*y, scale*z, state+0x1C)`.
///
/// # Arguments
/// - `rx`, `ry`, `rz`: rotation angles in the table's fixed-point scale
///   (65536 per full turn).
/// - `matrix`: destination of twelve `f32` values, row-major, three rows of
///   four; the fourth column is `{0, 0, 0}`.
/// - `gqr0`: live GQR0 word - format of the table reads and matrix stores.
/// - `gqr3`: live GQR3 word - format of the reduction scratch slot.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 48 bytes at `matrix`. Reads the shared table at `0x802F74A0` and the
/// small-data period word.
///
/// # Safety
/// `matrix` must be writable for 48 bytes. The selected GQR formats must be the
/// caller's actual ones; passing an unrelated word changes the table index and
/// the store widths.
///
/// # Confidence
/// medium - the reduction, the table bases, the sign tests and all twelve
/// matrix terms are literal in the export and match the sibling builder, but
/// the paired-single lane shuffles had to be expanded into scalar terms, and
/// the index conversion is delegated to [`fn_80237ca0`]'s established model.
pub unsafe fn fn_802382b0(
    rx: f64,
    ry: f64,
    rz: f64,
    matrix: usize,
    gqr0: u32,
    gqr3: u32,
) {
    // observed: six four-byte stack slots; `fn_80237ca0` uses exactly this
    // store-then-reload sequence and writes (sine, cosine) into its two slots.
    let mut sin_x = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut cos_x = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut sin_y = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut cos_y = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut sin_z = crate::platform::GuestLocal::<f32>::new( 0.0_f32);
    let mut cos_z = crate::platform::GuestLocal::<f32>::new( 0.0_f32);

    fn_80237ca0(
        rx,
        (sin_x.as_mut_ptr() as *mut f32) as usize,
        (cos_x.as_mut_ptr() as *mut f32) as usize,
        gqr3,
        gqr0,
    );
    fn_80237ca0(
        ry,
        (sin_y.as_mut_ptr() as *mut f32) as usize,
        (cos_y.as_mut_ptr() as *mut f32) as usize,
        gqr3,
        gqr0,
    );
    fn_80237ca0(
        rz,
        (sin_z.as_mut_ptr() as *mut f32) as usize,
        (cos_z.as_mut_ptr() as *mut f32) as usize,
        gqr3,
        gqr0,
    );

    let sx = *sin_x as f64;
    let cx = *cos_x as f64;
    let sy = *sin_y as f64;
    let cy = *cos_y as f64;
    let sz = *sin_z as f64;
    let cz = *cos_z as f64;

    // observed: row-major XYZ product; the fourth column is zero.
    psq_store(matrix, (cy * cz) as f32, gqr0);
    psq_store(matrix + 0x04, (sx * cz * sy - cx * sz) as f32, gqr0);
    psq_store(matrix + 0x08, (cx * cz * sy + sx * sz) as f32, gqr0);
    psq_store(matrix + 0x0C, 0.0, gqr0);

    psq_store(matrix + 0x10, (sz * cy) as f32, gqr0);
    psq_store(matrix + 0x14, (sx * sz * sy + cx * cz) as f32, gqr0);
    psq_store(matrix + 0x18, (cx * sz * sy - sx * cz) as f32, gqr0);
    psq_store(matrix + 0x1C, 0.0, gqr0);

    psq_store(matrix + 0x20, (-sy) as f32, gqr0);
    psq_store(matrix + 0x24, (sx * cy) as f32, gqr0);
    psq_store(matrix + 0x28, (cx * cy) as f32, gqr0);
    psq_store(matrix + 0x2C, 0.0, gqr0);
}

// ===========================================================================
// Wave 112: the segment-to-segment closest-point kernel
// ===========================================================================
//
// `0x802390D0` is the routine that both the stage sweep (`0x80075FD0`) and the
// background edge-snap (`0x80080900`, `bg_scene_edge_snap`) call for a distance
// between two 3D segments. Waves 106, 109 and 111 left it as an address call
// because the export interleaves the dot products through reused `psq` scratch
// slots.
//
// The raw words at `0x802390D0..0x802391B4` resolve that: the `psq_l` forms
// are the plain `W=0` (two floats) and `W=1` (one float) loads, so every
// `ldexpf`/`CONCAT` block in the export is quantisation noise. The body is the
// textbook `ClosestPtSegmentSegment`:
//
// * `d1 = q1 - p1`, `d2 = q2 - p2`, `r = p1 - p2`;
// * `a = |d1|^2`, `e = |d2|^2`, `b = d1.d2`, `c = d1.r`, `f = d2.r`;
// * `denom = a*e - b*b`; `s = (b*f - e*c)/denom`, `t = (a*f - b*c)/denom`;
// * `s` is clamped first (the `t = (b*s + f)/e` numerator is rewritten for the
//   `s` ends), then `t` is clamped and re-derives `s = clamp(-c/a)` or
//   `s = clamp((b-c)/a)`.
//
// The two tails at `0x80239298` divide only when the numerator is at least
// `1e-4`; the distance is then the squared length of `(p1 + d1*s) - (p2 + d2*t)`.

/// `f32` zero at `0x8042E4E4` (`r2 - 0x4B9C`), used by the clamp arms.
const W112_F32_ZERO: f32 = 0.0;
/// `f32` one at `0x8042E4F0` (`r2 - 0x4B90`), the `s` denominator when the
/// two directions are parallel.
const W112_F32_ONE: f32 = 1.0;
/// `f32` 1e-4 at `0x804299F8` (`r13 - 0x5C88`), the parallel/tangent epsilon.
const W112_F32_EPSILON: f32 = 0.000_1;

/// Returns the squared distance between the closest points of two 3D segments.
///
/// In plain English: "given segment `p1..q1` and segment `p2..q2`, find the
/// pair of points (one on each) that is closest, optionally report how far
/// along each segment they lie, and return the squared distance."
///
/// # Original
/// - Ghidra: `FUN_802390d0` at `0x802390D0` (688 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802390d0_FUN_802390d0.c`
/// - `// observed:` each input is six floats: `p` at `+0x00/+0x04/+0x08` and
///   `q` at `+0x0C/+0x10/+0x14` (loads at `0x802390D0`, `0x802390D8`,
///   `0x802390EC`, `0x802390FC`, `0x80239114`, `0x8023911C`).
/// - `// observed:` the five dot products are assembled with `ps_mul` and
///   `ps_madd` and finished with `ps_sum0` at `0x80239144`, `0x80239160`,
///   `0x80239180`, `0x802391AC` and `0x802391B4`; the element order is
///   `(dz*dz + dx*dx) + dy*dy`, not `x*x + y*y + z*z`.
/// - `// observed:` `denom = a*e - b*b` (`fmuls`/`fsubs` at
///   `0x802391A4..0x802391B0`); when it is below the `1e-4` epsilon the body
///   uses `s = 0`, `t = f/e` (`0x802391C4..0x802391D4`).
/// - `// observed:` the `s` clamps are at `0x802391F8` (below zero) and
///   `0x80239210` (above `denom`); the `t` clamps are at `0x80239228`,
///   `0x80239260`, `0x80239278` and `0x80239284`. Both tails divide only when
///   `1e-4 <= |numerator|` (`0x80239298..0x802392CC`).
/// - `// observed:` `param_3`/`param_4` are written only when non-null
///   (`cmpwi`/`stfs` at `0x802392D0..0x802392E4`).
/// - `// observed:` the result is `(dz*dz + dx*dx) + dy*dy`, where `d` is
///   `(p1 + d1*s) - (p2 + d2*t)`; the final `ps_sum0` at `0x80239374` leaves it
///   in `f1`'s first paired-single lane (`PS0`).
/// - `// inferred:` the function returns one `f32` (the paired-single lane),
///   not the `double` Ghidra declares; the callers compare it with `lfs` floats
///   (for example `fcmpo f0,f26,f1` after the call at `0x80076434`, where
///   `f26` is the `0.1f` at `0x8042BC78`).
///
/// # Arguments
/// - `segment_a`: six `f32` - `p1.x, p1.y, p1.z, q1.x, q1.y, q1.z`.
/// - `segment_b`: six `f32` - `p2.x, p2.y, p2.z, q2.x, q2.y, q2.z`.
/// - `out_s`: optional address of one `f32`; receives the parameter along
///   `segment_a`'s direction (`0` means unused).
/// - `out_t`: optional address of one `f32`; receives the parameter along
///   `segment_b`'s direction.
///
/// # Returns
/// The squared distance between the closest points, as a single-precision
/// float.
///
/// # Side effects
/// Writes `out_s` and `out_t` when they are non-null. Reads only the two
/// 24-byte segment records and the SDA2/SDA1 constants named above.
///
/// # Safety
/// Both segment pointers must be readable for 24 bytes. Each non-null output
/// pointer must be writable for four bytes.
///
/// # Confidence
/// medium - every load, store, branch and dot product is literal in the raw
/// instruction stream; the only reconstruction is treating `lfs`/`fcmpo`/the
/// single-precision ops as scalar `f32` operations, which is what the
/// paired-single lane and the callers require.
pub unsafe fn fn_802390d0(
    segment_a: usize,
    segment_b: usize,
    out_s: usize,
    out_t: usize,
) -> f32 {
    let p1x = platform::read_f32(segment_a);
    let p1y = platform::read_f32(segment_a + 0x04);
    let p1z = platform::read_f32(segment_a + 0x08);
    let q1x = platform::read_f32(segment_a + 0x0C);
    let q1y = platform::read_f32(segment_a + 0x10);
    let q1z = platform::read_f32(segment_a + 0x14);

    let p2x = platform::read_f32(segment_b);
    let p2y = platform::read_f32(segment_b + 0x04);
    let p2z = platform::read_f32(segment_b + 0x08);
    let q2x = platform::read_f32(segment_b + 0x0C);
    let q2y = platform::read_f32(segment_b + 0x10);
    let q2z = platform::read_f32(segment_b + 0x14);

    // observed: d1, d2 and r are formed by the ps_sub pairs at 0x802390E4,
    // 0x802390F4, 0x8023910C, 0x8023912C, 0x8023914C and 0x80239168.
    let d1x = q1x - p1x;
    let d1y = q1y - p1y;
    let d1z = q1z - p1z;
    let d2x = q2x - p2x;
    let d2y = q2y - p2y;
    let d2z = q2z - p2z;
    let rx = p1x - p2x;
    let ry = p1y - p2y;
    let rz = p1z - p2z;

    // observed: the element order is the `ps_madd`/`ps_sum0` order, whose
    // first product is z*z + x*x.
    let len1 = (d1z * d1z + d1x * d1x) + d1y * d1y;
    let len2 = (d2z * d2z + d2x * d2x) + d2y * d2y;
    let dot12 = (d1x * d2x + d1y * d2y) + d1z * d2z;
    let dot1r = (d1x * rx + d1y * ry) + d1z * rz;
    let dot2r = (d2x * rx + d2y * ry) + d2z * rz;

    let denom = len1 * len2 - dot12 * dot12;

    let mut s_num;
    let mut s_den;
    let mut t_num;
    let mut t_den;

    // observed: bge at 0x802391C0 enters the normal arm when denom is not
    // below the epsilon. NaN takes the branch because `!(NaN < eps)` is true.
    if !(denom < W112_F32_EPSILON) {
        s_num = dot12 * dot2r - len2 * dot1r;
        t_num = len1 * dot2r - dot12 * dot1r;
        s_den = denom;
        t_den = denom;

        // observed: s below zero (0x802391F8) resets s and sets t = f/e.
        if s_num < W112_F32_ZERO {
            s_num = W112_F32_ZERO;
            t_num = dot2r;
            t_den = len2;
        } else if s_num > s_den {
            // observed: s above one (0x80239210) sets s = 1 and t = (b + f)/e.
            s_num = s_den;
            t_num = dot2r + dot12;
            t_den = len2;
        }
    } else {
        // observed: parallel directions use s = 0 and t = f/e
        // (0x802391C4..0x802391D4).
        s_num = W112_F32_ZERO;
        s_den = W112_F32_ONE;
        t_num = dot2r;
        t_den = len2;
    }

    // observed: the t clamp at 0x80239228. A negative t re-derives s from
    // -c/a; a t above one re-derives s from (b - c)/a.
    if !(t_num < W112_F32_ZERO) {
        if t_num > t_den {
            t_num = t_den;
            let candidate = dot12 - dot1r;
            s_num = W112_F32_ZERO;
            if !(candidate < W112_F32_ZERO) {
                if !(candidate > len1) {
                    s_num = candidate;
                    s_den = len1;
                } else {
                    s_num = s_den;
                }
            }
        }
    } else {
        t_num = W112_F32_ZERO;
        let candidate = -dot1r;
        s_num = candidate;
        if !(candidate < W112_F32_ZERO) {
            if !(candidate > len1) {
                s_den = len1;
            } else {
                s_num = s_den;
            }
        } else {
            s_num = W112_F32_ZERO;
        }
    }

    // observed: the two `fabs`/epsilon tests at 0x80239298 and 0x802392B4.
    let s = if !(s_num.abs() < W112_F32_EPSILON) {
        s_num / s_den
    } else {
        W112_F32_ZERO
    };
    let t = if !(t_num.abs() < W112_F32_EPSILON) {
        t_num / t_den
    } else {
        W112_F32_ZERO
    };

    if out_s != 0 {
        platform::write_f32(out_s, s);
    }
    if out_t != 0 {
        platform::write_f32(out_t, t);
    }

    // observed: closest points are p1 + d1*s and p2 + d2*t; the tail returns
    // (dz*dz + dx*dx) + dy*dy (0x80239300..0x80239374).
    let dx = (rx + s * d1x) - t * d2x;
    let dy = (ry + s * d1y) - t * d2y;
    let dz = (rz + s * d1z) - t * d2z;
    (dz * dz + dx * dx) + dy * dy
}

// ===========================================================================
// Wave 115: the beam / shadow-volume record builder
// ===========================================================================
//
// `0x802389F0` is the 1 752-byte routine that the pass at `0x80259190` calls
// immediately before it publishes `auStack_100` through `r13 - 0x49B0`. It
// takes a 3x4 matrix plus six floats and fills a 240-byte record: a copy of the
// matrix, ten four-float planes, two scalars and an axis-aligned box around the
// eight rebuilt corners.
//
// The export is 3 030 lines with 126 "removing unreachable block" markers and
// is unreadable as C, but the raw words disassemble into a flat, regular
// sequence: one inverse, one copy, 24 `stfs` corner stores, four plane builds,
// an eight-iteration transform loop, one box build and six more plane builds.
// Every plane build is the same five-step pattern `fn_802385d0` documents: two
// differences, the `0x801C1530` cross, the `0x801C1470` normalize, and
// `d = -dot(normal, base)`.

/// `0x801C07A0` - inverse of a 3x4 matrix: `(src, dst)`, returns 1 on success.
///
/// `// observed:` the callee returns 0 when the determinant is zero and
/// otherwise writes twelve floats. `0x80238A5C` calls it with the *source*
/// matrix and a stack scratch block, not with the output record.
const PSMTX_INVERSE: usize = 0x801C_07A0;
/// `0x801C1160` - transform one point by a 3x4 matrix: `(matrix, point, out)`.
///
/// `// observed:` the callee adds the translation column (`m3`, `m7`, `m11`)
/// with an implicit fourth component of one; [`fn_80238420`] is its
/// no-translation sibling.
const PSMTX_MULT_VEC: usize = 0x801C_1160;

/// Builds one four-float plane `{nx, ny, nz, d}` from a base point and two
/// points expressed relative to it.
///
/// `// observed:` this is the body of every plane block in [`fn_802389f0`]:
/// `cross(first - base, second - base)` through `0x801C1530`, `0x801C1470`
/// normalises the cross into `out`, and `out[3] = -(n . base)` is the plain
/// `f32` store that closes each block. `0x802389F0` inlines these three steps
/// (it does not call `0x802385D0`), so the helper calls the same two G3D
/// routines the instruction stream calls.
///
/// # Arguments
/// - `out`: four floats `{nx, ny, nz, d}`.
/// - `base`: the shared origin of both edges; also the point the distance is
///   measured from.
/// - `first`: end of the first edge.
/// - `second`: end of the second edge.
///
/// # Safety
/// `base`, `first` and `second` must be readable for twelve bytes each and
/// `out` must be writable for sixteen bytes.
unsafe fn build_plane(out: usize, base: usize, first: usize, second: usize) {
    let bx = platform::read_f32(base);
    let by = platform::read_f32(base + 0x04);
    let bz = platform::read_f32(base + 0x08);

    let edge_first = crate::platform::GuestLocal::new( [
        platform::read_f32(first) - bx,
        platform::read_f32(first + 0x04) - by,
        platform::read_f32(first + 0x08) - bz,
    ]);
    let edge_second = crate::platform::GuestLocal::new( [
        platform::read_f32(second) - bx,
        platform::read_f32(second + 0x04) - by,
        platform::read_f32(second + 0x08) - bz,
    ]);

    let mut cross = crate::platform::GuestLocal::new( [0.0_f32; 3]);
    // observed: the call order is `(first - base, second - base, out)`.
    platform::call3(
        G3D_CROSS3,
        edge_first.as_ptr() as u32,
        edge_second.as_ptr() as u32,
        cross.as_mut_ptr() as u32,
    );
    platform::call2(G3D_NORMALIZE3, cross.as_ptr() as u32, out as u32);

    let nx = platform::read_f32(out);
    let ny = platform::read_f32(out + 0x04);
    let nz = platform::read_f32(out + 0x08);
    // observed: `ps_mul`/`ps_madd`/`ps_sum0`, then `fneg` and a plain `stfs`.
    platform::write_f32(out + 0x0C, -((nx * bx + ny * by) + nz * bz));
}

/// Fills the beam / shadow-volume record from a matrix and six parameters.
///
/// In plain English: "copy the incoming 3x4 matrix into the record, build the
/// eight corners of a near/far quad from the six parameters, cut ten planes
/// through those corners, and report their bounding box."
///
/// # Original
/// - Ghidra: `FUN_802389f0` at `0x802389F0` (1752 bytes)
/// - Evidence: `work/ghidra/main-gb/export/802389f0_FUN_802389f0.c`
/// - `// observed:` the ABI is two pointer words in `r3`/`r4` plus six doubles
///   in `f1..f6`. `0x80238A34..0x80238A58` moves `r4` to `r23` (the source
///   matrix), `r3` to `r31` (the record) and each `f1..f6` to `f26..f31`;
///   `0x80259190` passes the 252-byte `auStack_100` record first and the
///   48-byte matrix second.
/// - `// observed:` `0x80238A5C` calls `0x801C07A0` with the source matrix, so
///   the stack block at `sp+0x180` holds its inverse; `0x80238A68` then calls
///   `0x801C0500` with `(source, record)`, copying twelve floats to
///   `record+0x00`.
/// - `// observed:` `0x80238A6C` is `fdivs f6,f31,f30` (`p6 / p5`), the near
///   depth is `-p5` and the far depth `-p6` (`fneg` at `0x80238A84` and
///   `0x80238A8C`), and the four `fmuls` at `0x80238A94..0x80238AAC` scale the
///   far corners by that ratio.
/// - `// observed:` the 24 `stfs` at `0x80238AA0..0x80238B10 write eight
///   3-float corners. Each row is `(x, y, z)`; the first four are
///   `(p3,p1)`, `(p4,p1)`, `(p4,p2)`, `(p3,p2)` at `z = -p5`, and the last
///   four are the same pairs scaled by `p6/p5` at `z = -p6`. `// inferred:`
///   the `p1`/`p2` pair is the second coordinate and `p3`/`p4` the first, i.e.
///   a rectangle in the record's x/y plane.
/// - `// observed:` `0x80238B14`/`0x80238B18` store `-p5` and `-p6` at
///   `record+0x70` and `record+0x74`.
/// - `// observed:` the first four planes (`record+0x30`, `+0x40`, `+0x50`,
///   `+0x60`) are built with the all-zero triple at `sp+0x170` as their base,
///   so their distance is `-0.0`. That triple is three `stfs` of
///   `r2 - 0x4B9C`, which is `0x8042E4E4 = 0.0f`, at
///   `0x80238A74..0x80238A98`.
/// - `// observed:` the loop at `0x80238D1C..0x80238D3C` runs eight times and
///   calls `0x801C1160(inverse, corner, corner)`; `0x80238D4C` transforms the
///   zero triple the same way, leaving the inverse's translation column in
///   `sp+0x170` as the origin of the last six planes.
/// - `// observed:` `0x80238D5C` calls `0x80238690(record+0x78, corners, 8)`,
///   the min/max box already reconstructed as [`fn_80238690`].
/// - `// observed:` the six post-transform planes use `cross(row0, row3)`,
///   `cross(row2, row1)`, `cross(row2 - row0, row1 - row0)`,
///   `cross(row4 - row0, row6 - row0)`, `cross(row1, row0)` and
///   `cross(row3, row2)` around the transformed origin/corner; their
///   destinations are `record+0x90` through `record+0xE0`.
/// - `// inferred:` `0x801C07A0` is `PSMTXInverse` and `0x801C1160` is
///   `PSMTXMultVec`; both identities come from their exported bodies (the
///   determinant/cofactor stores and the nine multiply-adds plus the
///   translation column), not from a symbol name.
/// - `// inferred:` the caller's `+0xB4`/`+0xB8` word pair is the near/far
///   depth: it is the pair that becomes `p5`/`p6` here, and the ratio
///   `p6/p5` scales the far quad.
/// - `// unknown:` why the first four planes measure their distance from the
///   zero point instead of the record's origin; the arithmetic is literal, but
///   the intent is not visible in this body.
///
/// # Arguments
/// - `record`: 0xFC-byte destination. `+0x00..0x2C` receives the source
///   matrix, `+0x30/+0x40/+0x50/+0x60` and `+0x90..0xEC` receive ten plane
///   records, `+0x70/+0x74` receive the negated depths, and `+0x78..0x8C`
///   receives the min/max box.
/// - `source_matrix`: twelve floats, a 3x4 matrix.
/// - `p1`, `p2`: the second coordinate of the four corners (stored at
///   `+0x04`/`+0x10` of each row). `// inferred:` the y coordinate.
/// - `p3`, `p4`: the first coordinate of the four corners (stored at
///   `+0x00`/`+0x0C`). `// inferred:` the x coordinate.
/// - `p5`: near depth; also the divisor of the far scale.
/// - `p6`: far depth; `p6 / p5` scales the far quad.
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Writes 0xF0 bytes of `record`, stores its inverse scratch on the stack, and
/// calls `0x801C07A0`, `0x801C0500`, `0x801C1530`, `0x801C1470`,
/// `0x801C1160` and [`fn_80238690`].
///
/// # Safety
/// `record` must be writable for 0xF0 bytes and `source_matrix` readable for
/// 0x30 bytes. Every listed helper must be the mapped routine; the transform
/// loop writes each corner in place.
///
/// # Confidence
/// medium - all 438 instructions are literal and the plane, loop and box
/// shapes are read directly from the disassembly, but two helper identities and
/// the record's field roles are inferred from the callees and the caller.
pub unsafe fn fn_802389f0(
    record: usize,
    source_matrix: usize,
    p1: f64,
    p2: f64,
    p3: f64,
    p4: f64,
    p5: f64,
    p6: f64,
) {
    // observed: the doubles are narrowed to singles by the first `stfs`/`fmuls`
    // pair, so every value below is `f32` arithmetic.
    let f1 = p1 as f32;
    let f2 = p2 as f32;
    let f3 = p3 as f32;
    let f4 = p4 as f32;
    let f5 = p5 as f32;
    let f6 = p6 as f32;

    // observed: 0x80238A5C inverts the source matrix into a stack block;
    // 0x80238A68 copies the source matrix into record+0x00.
    let mut inverse = crate::platform::GuestLocal::new( [0.0_f32; 12]);
    platform::call2(
        PSMTX_INVERSE,
        source_matrix as u32,
        inverse.as_mut_ptr() as u32,
    );
    platform::call2(PSMTX_COPY, source_matrix as u32, record as u32);

    // observed: fdivs f6,f31,f30 and the two fneg ops at 0x80238A84/8C.
    let ratio = f6 / f5;
    let near_z = -f5;
    let far_z = -f6;

    let mut corners = crate::platform::GuestLocal::new( [0.0_f32; 24]);
    // observed: rows 0..3 are the near rectangle, rows 4..7 the far one.
    corners[0x00] = f3;
    corners[0x01] = f1;
    corners[0x02] = near_z;
    corners[0x03] = f4;
    corners[0x04] = f1;
    corners[0x05] = near_z;
    corners[0x06] = f4;
    corners[0x07] = f2;
    corners[0x08] = near_z;
    corners[0x09] = f3;
    corners[0x0A] = f2;
    corners[0x0B] = near_z;
    corners[0x0C] = ratio * f3;
    corners[0x0D] = ratio * f1;
    corners[0x0E] = far_z;
    corners[0x0F] = ratio * f4;
    corners[0x10] = ratio * f1;
    corners[0x11] = far_z;
    corners[0x12] = ratio * f4;
    corners[0x13] = ratio * f2;
    corners[0x14] = far_z;
    corners[0x15] = ratio * f3;
    corners[0x16] = ratio * f2;
    corners[0x17] = far_z;

    platform::write_f32(record + 0x70, near_z);
    platform::write_f32(record + 0x74, far_z);

    // observed: the first four planes are built around the zero triple, so
    // their fourth component is zero.
    let zero = crate::platform::GuestLocal::new( [0.0_f32; 3]);
    let zero_ptr = zero.as_ptr() as usize;
    let corner_ptr = corners.as_ptr() as usize;
    build_plane(record + 0x30, zero_ptr, corner_ptr, corner_ptr + 0x24);
    build_plane(record + 0x40, zero_ptr, corner_ptr + 0x18, corner_ptr + 0x0C);
    build_plane(record + 0x50, zero_ptr, corner_ptr + 0x0C, corner_ptr);
    build_plane(record + 0x60, zero_ptr, corner_ptr + 0x24, corner_ptr + 0x18);

    // observed: eight in-place transforms through the inverse matrix.
    let inverse_ptr = inverse.as_ptr() as u32;
    let mut index = 0_u32;
    while index < 8 {
        let corner = corner_ptr as u32 + index * 0x0C;
        platform::call3(PSMTX_MULT_VEC, inverse_ptr, corner, corner);
        index += 1;
    }

    // observed: transforming the zero triple leaves the inverse's translation
    // column, which the last six planes use as their origin.
    let mut origin = crate::platform::GuestLocal::new( [0.0_f32; 3]);
    let origin_ptr = origin.as_mut_ptr();
    platform::call3(
        PSMTX_MULT_VEC,
        inverse_ptr,
        origin_ptr as u32,
        origin_ptr as u32,
    );

    // observed: 80238690(record+0x78, corners, 8) fills min xyz then max xyz.
    fn_80238690(record + 0x78, corner_ptr, 8);

    let origin_addr = origin.as_ptr() as usize;
    build_plane(record + 0x90, origin_addr, corner_ptr, corner_ptr + 0x24);
    build_plane(record + 0xA0, origin_addr, corner_ptr + 0x18, corner_ptr + 0x0C);
    build_plane(record + 0xB0, corner_ptr, corner_ptr + 0x18, corner_ptr + 0x0C);
    build_plane(record + 0xC0, corner_ptr, corner_ptr + 0x30, corner_ptr + 0x48);
    build_plane(record + 0xD0, origin_addr, corner_ptr + 0x0C, corner_ptr);
    build_plane(record + 0xE0, origin_addr, corner_ptr + 0x24, corner_ptr + 0x18);
}

// ===========================================================================
// Wave 276 lane E (`_E276`) - gap batch 6, script-heap release bodies
// ===========================================================================
//
// Two untabled gap bodies from the wave-266 candidate set, re-verified against
// `work/dol/main.elf` with the current documented extents. Selection:
// `work/wave276/e_pick.mjs`; ELF re-walk and disassembly: `e_verify.mjs`,
// `e_verify_stdout.txt` and `e_dis_rest.txt`. They are the release halves of
// the queue-A/queue-B script records allocated by [`fn_80235fe0`] /
// [`fn_80236d70`]: unlink from the record's `+0x04` list, then free the node
// back to its heap.

// ---------------------------------------------------------------------------
// 0x80236BA0 - release one queue-A script record
// ---------------------------------------------------------------------------

/// Unlinks one queue-A script record and frees it.
///
/// The intrusive list at `record+0x04` is the one initialised by
/// [`fn_80235fe0`] with [`LIST_INIT`]. The node is removed with
/// [`LIST_REMOVE`], then freed on [`SCRIPT_HEAP_A`] through [`HEAP_FREE`].
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80236BA0`.
/// - Extent: `0x80236BA0..0x80236BDC` (60 bytes, 14 instructions), ending at
///   the `blr` at `0x80236BD8`; two zero words precede it at
///   `0x80236B98..0x80236BA0` (two `b 0x80228FF0` stubs sit at
///   `0x80236B80`/`0x80236B90`) and the next body starts at `0x80236BE0`.
/// - Reach: no `b` or `bl` in the executable segments targets it; no data word
///   names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `addi r3,r3,4`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r4`;
///   `bl 0x80228F80`; `lwz r3,-0x49F0(r13)`; `mr r4,r31`;
///   `bl 0x801D4710`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: script record; the list head is at `+0x04`.
/// - `node`: the record node to remove and free (the original's `r4`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Unlinks `node` from the list and frees it on the queue-A heap.
///
/// # Safety
/// `this+0x04` must be an initialised list containing `node`, and `node` must
/// be a block owned by [`SCRIPT_HEAP_A`].
///
/// # Confidence
/// high - the `addi`, the two calls and the two SDA/heap words are literal; the
/// queue-A heap word is the same one [`fn_80236c70`] allocates from.
pub unsafe fn fn_80236ba0(this: usize, node: usize) {
    platform::call2(LIST_REMOVE, (this + 4) as u32, node as u32);
    platform::call2(HEAP_FREE, platform::read_u32(SCRIPT_HEAP_A), node as u32);
}

// ---------------------------------------------------------------------------
// 0x80237930 - release one queue-B script record
// ---------------------------------------------------------------------------

/// Unlinks one queue-B script record and frees it.
///
/// The queue-B twin of [`fn_80236ba0`]: the same `+0x04` list remove, but the
/// free goes to [`SCRIPT_HEAP_B`] (`-0x49E8(r13)`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80237930`.
/// - Extent: `0x80237930..0x8023796C` (60 bytes, 14 instructions), ending at
///   the `blr` at `0x80237968`; two zero words precede it at
///   `0x80237928..0x80237930` (two `b 0x80228FF0` stubs sit at
///   `0x80237910`/`0x80237920`) and the next body starts at `0x80237970`.
/// - Reach: no `b` or `bl` in the executable segments targets it; no data word
///   names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `addi r3,r3,4`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r4`;
///   `bl 0x80228F80`; `lwz r3,-0x49E8(r13)`; `mr r4,r31`;
///   `bl 0x801D4710`; epilogue; `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: script record; the list head is at `+0x04`.
/// - `node`: the record node to remove and free (the original's `r4`).
///
/// # Returns
/// Nothing.
///
/// # Side effects
/// Unlinks `node` from the list and frees it on the queue-B heap.
///
/// # Safety
/// `this+0x04` must be an initialised list containing `node`, and `node` must
/// be a block owned by [`SCRIPT_HEAP_B`].
///
/// # Confidence
/// high - the only difference from [`fn_80236ba0`] is the `-0x49E8(r13)` heap
/// word.
pub unsafe fn fn_80237930(this: usize, node: usize) {
    platform::call2(LIST_REMOVE, (this + 4) as u32, node as u32);
    platform::call2(HEAP_FREE, platform::read_u32(SCRIPT_HEAP_B), node as u32);
}

// ===========================================================================
// Wave 279 lane E (`_E279`) - gap batch 9, script-queue destructors
// ===========================================================================
//
// Two untabled gap bodies from the wave-266 candidate set, re-verified against
// `work/dol/main.elf`. Selection: `work/wave279/e_pick.mjs`; ELF re-walk and
// disassembly: `e_verify.mjs` and `e_verify_stdout.txt`. Both start with
// `9421FFF0`, each recursive descent ends on exactly one `blr`, neither
// overlaps a documented extent and no pointer-run word names either start.
//
// They are the deleting destructors of the two queue-A/queue-B script record
// families whose list removal and freeing already live in
// [`fn_80236ba0`]/[`fn_80237930`].

/// Deleting destructor for a record owned by [`SCRIPT_HEAP_A`].
///
/// When `this` is non-null and `release` is positive, the block is freed on the
/// queue-A script heap (`r13 - 0x49F0`) through [`HEAP_FREE`] (`0x801D4710`).
/// The object is returned unchanged, exactly like the other deleting
/// destructors in this band.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x80236B30`.
/// - Extent: `0x80236B30..0x80236B78` (72 bytes, 17 instructions), ending at
///   the `blr` at `0x80236B74`; three zero words precede it at
///   `0x80236B24..0x80236B30` and the next body is [`fn_80236ba0`] at
///   `0x80236BA0`.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `2C030000 cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `41820018 bc 12,2,+0x18` (beq); `2C040000 cmpi cr0,0,r4,0`;
///   `40810010 bc 4,1,+0x10` (ble); `806DB610 lwz r3,-0x49F0(r13)`;
///   `mr r4,r31`; `4BF9DBB5 bl 0x801D4710`; `mr r3,r31`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: queue-A record to delete, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` on the queue-A script heap.
///
/// # Safety
/// A non-null `this` must be a block owned by [`SCRIPT_HEAP_A`].
///
/// # Confidence
/// high - the guard, the heap word and the call are literal.
pub unsafe fn fn_80236b30(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call2(HEAP_FREE, platform::read_u32(SCRIPT_HEAP_A), this as u32);
    }
    this
}

/// Deleting destructor for a record owned by [`SCRIPT_HEAP_B`].
///
/// The queue-B twin of [`fn_80236b30`]: the same null/positive-release guard,
/// but the free goes to the queue-B script heap (`r13 - 0x49E8`).
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row and no export file for `0x802378C0`.
/// - Extent: `0x802378C0..0x80237908` (72 bytes, 17 instructions), ending at
///   the `blr` at `0x80237904`; three zero words precede it at
///   `0x802378B4..0x802378C0` and the next body is [`fn_80237930`] at
///   `0x80237930`.
/// - Reach: no `b` or `bl` in the executable segments targets it, and no data
///   word names it.
/// - Observed: `stwu r1,-0x10(r1)`; `mflr r0`; `2C030000 cmpi cr0,0,r3,0`;
///   `stw r0,0x14(r1)`; `stw r31,0x0C(r1)`; `mr r31,r3`;
///   `41820018 bc 12,2,+0x18` (beq); `2C040000 cmpi cr0,0,r4,0`;
///   `40810010 bc 4,1,+0x10` (ble); `806DB618 lwz r3,-0x49E8(r13)`;
///   `mr r4,r31`; `4BF9CE25 bl 0x801D4710`; `mr r3,r31`; epilogue;
///   `4E800020 bclr 20,0` (blr).
///
/// # Arguments
/// - `this`: queue-B record to delete, or `0`.
/// - `release`: positive to release the allocation.
///
/// # Returns
/// `this`.
///
/// # Side effects
/// May free `this` on the queue-B script heap.
///
/// # Safety
/// A non-null `this` must be a block owned by [`SCRIPT_HEAP_B`].
///
/// # Confidence
/// high - identical to [`fn_80236b30`] except for the heap word.
pub unsafe fn fn_802378c0(this: usize, release: i32) -> usize {
    if this != 0 && release > 0 {
        platform::call2(HEAP_FREE, platform::read_u32(SCRIPT_HEAP_B), this as u32);
    }
    this
}

// ---------------------------------------------------------------------------
// 0x802379A0 - tear down script queue B
// ---------------------------------------------------------------------------

/// Destroys script queue B: frees its first record, then its heap.
///
/// The byte-for-byte sibling of [`fn_80236c10`] with [`SCRIPT_HEAP_B`] and
/// [`SCRIPT_RECORD_B`] as the targets. When the heap is non-null it frees the
/// record through `0x801D4710`, releases the heap block through
/// [`crate::runtime::dvd::fn_801d4430`], clears both globals and returns the
/// heap pointer.
///
/// # Original (untabled)
/// - Ghidra: none - no `functions.tsv` row for `0x802379A0`.
/// - Extent: `0x802379A0..0x80237A00` (96 bytes, 24 words), ending at the
///   `blr` at `0x802379FC`; the next body starts immediately at `0x80237A00`.
/// - Reach: no `b`/`bl` and no data word in the main image or any REL.
/// - Observed: `9421FFF0 stwu r1,-0x10(r1)`; `mflr r0`; `stw r0,0x14(r1)`;
///   `stw r31,0xC(r1)`; `li r31,0`; `lwz r3,-0x49E8(r13)`;
///   `cmpi cr0,0,r3,0`; `4182002C bc 12,2,+0x2C` (beq to the return);
///   `lwz r4,-0x49E4(r13)`; `cmpi cr0,0,r4,0`; `41820008 bc 12,2`;
///   `bl 0x801D4710`; `lwz r31,-0x49E8(r13)`; `mr r3,r31`;
///   `bl 0x801D4430`; `li r0,0`; `stw r0,-0x49E8(r13)`;
///   `stw r0,-0x49E4(r13)`; `mr r3,r31`; epilogue; `4E800020 blr`.
/// - Evidence: `work/wave285/e_dis.txt`, `work/wave285/e_refs.txt`.
///
/// # Returns
/// The destroyed heap pointer, or `0` when [`SCRIPT_HEAP_B`] was already null.
///
/// # Side effects
/// Frees the record and the heap and clears both queue-B globals.
///
/// # Safety
/// The globals must describe a live queue-B heap/record pair when non-null.
///
/// # Confidence
/// high - identical to [`fn_80236c10`] except for the two global words.
pub unsafe fn fn_802379a0() -> u32 {
    let mut heap = platform::read_u32(SCRIPT_HEAP_B);
    if heap == 0 {
        return 0;
    }
    let record = platform::read_u32(SCRIPT_RECORD_B);
    if record != 0 {
        platform::call2(HEAP_FREE, heap, record);
    }
    heap = platform::read_u32(SCRIPT_HEAP_B);
    let _ = crate::runtime::dvd::fn_801d4430(heap as usize);
    platform::write_u32(SCRIPT_HEAP_B, 0);
    platform::write_u32(SCRIPT_RECORD_B, 0);
    heap
}

// ===========================================================================
// Layout sanity: the constants above describe fixed-size structures. These
// asserts are compile-time documentation, not runtime checks.
// ===========================================================================
const _: () = assert!(ARC_FILE_INFO_SIZE == 0x0C);
const _: () = assert!(ARC_DIR_ITER_SIZE == 0x10);
const _: () = assert!(ARC_ENTRY_SIZE == 0x10);
const _: () = assert!(U8_NODE_SIZE == 0x0C);
const _: () = assert!(RES_ARCHIVE_HANDLE + ARC_HANDLE_SIZE <= RES_ARCHIVE_SIZE);
