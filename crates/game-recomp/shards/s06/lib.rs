//! Shard 6 of the recompiled guest functions: `gen/s06.rs` (USA revision 1)
//! and `gen/<version>/s06.rs` (the other versions), written by `ppc-recomp`
//! (see `crates/game-recomp`).

#[path = "../../gen/s06.rs"]
mod code;

pub use code::*;

#[path = "../../gen/e2/s06.rs"]
pub mod e2;
#[path = "../../gen/p1/s06.rs"]
pub mod p1;
#[path = "../../gen/p2/s06.rs"]
pub mod p2;
#[path = "../../gen/j1/s06.rs"]
pub mod j1;
#[path = "../../gen/j2/s06.rs"]
pub mod j2;
#[path = "../../gen/k/s06.rs"]
pub mod k;
#[path = "../../gen/w/s06.rs"]
pub mod w;
