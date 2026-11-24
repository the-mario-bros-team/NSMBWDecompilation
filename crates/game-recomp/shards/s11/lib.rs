//! Shard 11 of the recompiled guest functions: `gen/s11.rs` (USA revision 1)
//! and `gen/<version>/s11.rs` (the other versions), written by `ppc-recomp`
//! (see `crates/game-recomp`).

#[path = "../../gen/s11.rs"]
mod code;

pub use code::*;

#[path = "../../gen/e2/s11.rs"]
pub mod e2;
#[path = "../../gen/p1/s11.rs"]
pub mod p1;
#[path = "../../gen/p2/s11.rs"]
pub mod p2;
#[path = "../../gen/j1/s11.rs"]
pub mod j1;
#[path = "../../gen/j2/s11.rs"]
pub mod j2;
#[path = "../../gen/k/s11.rs"]
pub mod k;
#[path = "../../gen/w/s11.rs"]
pub mod w;
