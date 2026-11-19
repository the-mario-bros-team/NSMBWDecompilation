//! # Range `0x80005000..0x80014000` - promoted
//!
//! This address-range lane (`sys/main.dol`, DOL = the Wii's flat executable
//! format) has been **fully promoted**: every function it translated now lives
//! in a semantic module. Nothing is defined here any more.
//!
//! The range was an *address* partition, not a system. It mixed vertex-lane
//! storage, GX (the Wii GPU command FIFO) helpers, path-node accessors and two
//! node-array initialisers. Promotion split it by role:
//!
//! | Original function | Address | Now in |
//! |---|---|---|
//! | [`copy_12_floats`](crate::render::geometry::copy_12_floats) | `0x8000AE50` | `render/geometry.rs` |
//! | [`fn_8000b320`](crate::render::geometry::fn_8000b320) | `0x8000B320` | `render/geometry.rs` |
//! | [`store_four_lane_vertices`](crate::render::geometry::store_four_lane_vertices) | `0x8000D0D0` | `render/geometry.rs` |
//! | [`vec2_sub`](crate::render::geometry::vec2_sub) | `0x8000D870` | `render/geometry.rs` |
//! | [`vec2_differs_from`](crate::render::geometry::vec2_differs_from) | `0x8000D8A0` | `render/geometry.rs` |
//! | [`vec2_length`](crate::render::geometry::vec2_length) | `0x8000F900` | `render/geometry.rs` |
//! | [`fn_8000d190`](crate::game::path::fn_8000d190) | `0x8000D190` | `game/path.rs` |
//! | [`fn_8000d260`](crate::game::path::fn_8000d260) | `0x8000D260` | `game/path.rs` |
//! | [`fn_8000d3b0`](crate::game::path::fn_8000d3b0) | `0x8000D3B0` | `game/path.rs` |
//! | [`fn_8000d5b0`](crate::game::path::fn_8000d5b0) | `0x8000D5B0` | `game/path.rs` |
//! | [`fn_8000d660`](crate::game::path::fn_8000d660) | `0x8000D660` | `game/path.rs` |
//! | [`fn_8000de70`](crate::game::path::fn_8000de70) | `0x8000DE70` | `game/path.rs` |
//! | [`fn_8000df60`](crate::game::path::fn_8000df60) | `0x8000DF60` | `game/path.rs` |
//! | [`fn_8000e030`](crate::game::path::fn_8000e030) | `0x8000E030` | `game/path.rs` |
//! | [`fn_8000e840`](crate::game::path::fn_8000e840) | `0x8000E840` | `game/path.rs` |
//! | [`fn_8000ebf0`](crate::render::gx::fn_8000ebf0) | `0x8000EBF0` | `render/gx.rs` |
//! | [`fn_8000eeb0`](crate::render::gx::fn_8000eeb0) | `0x8000EEB0` | `render/gx.rs` |
//! | [`fn_8000ef20`](crate::render::gx::fn_8000ef20) | `0x8000EF20` | `render/gx.rs` |
//! | [`fn_8000f460`](crate::game::actor::fn_8000f460) | `0x8000F460` | `game/actor.rs` |
//! | [`fn_8000f4b0`](crate::game::actor::fn_8000f4b0) | `0x8000F4B0` | `game/actor.rs` |
//! | [`init_150_node_array`](crate::game::actor::init_150_node_array) | `0x80008440` | `game/actor.rs` |
//! | [`init_500_node_array`](crate::game::actor::init_500_node_array) | `0x800099D0` | `game/actor.rs` |
//!
//! No function of this lane was left unclassified. The evidence chain
//! (`# Original` address + `work/ghidra/main-gb/export/...` path) travels with
//! each function; the per-range ledger is `ledger/agents/r80005_80014.csv` and
//! the completion report is `docs/COVERAGE_r80005_80014.md`.
//!
//! ## Not translated from this range
//!
//! These were already documented as next targets, not promoted:
//!
//! * `0x80006870..0x80006C50` - constructor/destructor cluster (r13 globals,
//!   damaged bodies).
//! * `0x80007610`, `0x80007DE0` - GQR/paired-single effect setup (large, low
//!   Ghidra fidelity).
//! * `0x8000C5C0`, `0x8000CA50` - large state machines.
//! * `0x8000D0A0` - single-lane sibling of `store_four_lane_vertices`.
//! * `0x8000E910`, `0x8000EDD0` - quad fill and flush used by `render/gx.rs`.
//! * `0x8000F950`, `0x8000FBA0` - polyline generator and wrapper.
//! * `0x80011020..0x800126A0` - 2-player course-cursor accessors.
//! * `0x800129A0` - course-select guide state table (6204 bytes).
//!
//! See `docs/COVERAGE_r80005_80014.md` for sizes, rationale and ranking.
