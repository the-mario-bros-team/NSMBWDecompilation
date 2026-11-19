//! Address-range decompilation lanes (main.dol, entry address ranges).
//! Existing boot/runtime/engine lanes own their specific addresses; range
//! lanes skip those. Each module is owned by one agent.

pub mod r80005_80014;
pub mod r80014_80040;
pub mod r80040_80080;
pub mod r80080_800c0;
pub mod r80100_80140;
pub mod r80140_80180;
pub mod r80180_801c0;
pub mod r801c0_80200;
pub mod r80200_80260;
pub mod r80260_802ed;
