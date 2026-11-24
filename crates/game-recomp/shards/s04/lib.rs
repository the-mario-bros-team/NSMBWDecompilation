//! Shard 4 of the recompiled guest functions: `gen/s04.rs` (USA revision 1)
//! and `gen/<version>/s04.rs` (the other versions), written by `ppc-recomp`
//! (see `crates/game-recomp`).

#[path = "../../gen/s04.rs"]
mod code;

pub use code::*;

#[path = "../../gen/e2/s04.rs"]
pub mod e2;
#[path = "../../gen/p1/s04.rs"]
pub mod p1;
#[path = "../../gen/p2/s04.rs"]
pub mod p2;
#[path = "../../gen/j1/s04.rs"]
pub mod j1;
#[path = "../../gen/j2/s04.rs"]
pub mod j2;
#[path = "../../gen/k/s04.rs"]
pub mod k;
#[path = "../../gen/w/s04.rs"]
pub mod w;
