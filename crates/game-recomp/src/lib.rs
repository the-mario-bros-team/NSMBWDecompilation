//! The guest functions `ppc-recomp` turned into Rust (docs/PORT_PLAN.md,
//! P6), compiled from `gen/`. That code is generated from the user's own
//! disc and RAM image, so it is not part of the source tree; without it the
//! table is empty and the port interprets everything.
//!
//! Regenerate after changing the hooks in `hle_os::SPECIAL` (the runtime
//! refuses code made for other hooks) or to cover more of the game:
//!
//! ```text
//! nsmbw-port --unpaced --press-file <script> --code-profile profile.txt --dump-ram <frame>:ram.bin
//! cargo run --release -p ppc-recomp -- --ram ram.bin --map MAP.txt --profile profile.txt
//! ```

#[path = "../gen/mod.rs"]
mod gen;

pub use gen::{FUNCTIONS, SPECIAL};
