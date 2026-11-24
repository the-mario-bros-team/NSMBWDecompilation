//! The guest functions `ppc-recomp` turned into Rust (docs/PORT_PLAN.md,
//! P6), compiled from `gen/`: USA revision 1's in `gen/` itself, each other
//! version's in `gen/<version>/` (`p1`, `k`, ...). That code is generated
//! from the user's own disc and RAM image, so it is not part of the source
//! tree; a version without it has an empty table and is interpreted.
//!
//! Regenerate after changing the hooks in `hle_os::special` (the runtime
//! refuses code made for other hooks) or to cover more of the game:
//!
//! ```text
//! nsmbw-port --unpaced --press-file <script> --code-profile profile.txt --dump-ram <frame>:ram.bin
//! cargo run --release -p ppc-recomp -- --version <E1|P1|...> --ram ram.bin --map MAP.txt --profile profile.txt
//! ```

use game_version::Version;
pub use guest_cpu::recomp::FnInfo;

#[path = "../gen/mod.rs"]
mod e1;
#[path = "../gen/e2/mod.rs"]
mod e2;
#[path = "../gen/p1/mod.rs"]
mod p1;
#[path = "../gen/p2/mod.rs"]
mod p2;
#[path = "../gen/j1/mod.rs"]
mod j1;
#[path = "../gen/j2/mod.rs"]
mod j2;
#[path = "../gen/k/mod.rs"]
mod k;
#[path = "../gen/w/mod.rs"]
mod w;

/// Version `v`'s recompiled functions (empty if none were generated) and
/// the hook addresses they were generated for.
pub fn code(v: Version) -> (&'static [FnInfo], &'static [u32]) {
    match v {
        Version::E1 => (&e1::FUNCTIONS, e1::SPECIAL),
        Version::E2 => (&e2::FUNCTIONS, e2::SPECIAL),
        Version::P1 => (&p1::FUNCTIONS, p1::SPECIAL),
        Version::P2 => (&p2::FUNCTIONS, p2::SPECIAL),
        Version::J1 => (&j1::FUNCTIONS, j1::SPECIAL),
        Version::J2 => (&j2::FUNCTIONS, j2::SPECIAL),
        Version::K => (&k::FUNCTIONS, k::SPECIAL),
        Version::W => (&w::FUNCTIONS, w::SPECIAL),
    }
}
