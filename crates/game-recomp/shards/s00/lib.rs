//! Shard 0 of the recompiled guest functions: `gen/s00.rs` (USA revision 1)
//! and `gen/<version>/s00.rs` (the other versions), written by `ppc-recomp`
//! (see `crates/game-recomp`).

#[path = "../../gen/s00.rs"]
mod code;

pub use code::*;

#[path = "../../gen/e2/s00.rs"]
pub mod e2;
#[path = "../../gen/p1/s00.rs"]
pub mod p1;
#[path = "../../gen/p2/s00.rs"]
pub mod p2;
#[path = "../../gen/j1/s00.rs"]
pub mod j1;
#[path = "../../gen/j2/s00.rs"]
pub mod j2;
#[path = "../../gen/k/s00.rs"]
pub mod k;
#[path = "../../gen/w/s00.rs"]
pub mod w;
