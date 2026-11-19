//! Runtime / OS reconstruction.
//!
//! Start with [`os`] and [`init`] for the OS initialization sequence. The
//! promoted subsystems are [`dvd`], [`input`], [`bt`], [`ids`] and [`cpp`].

pub mod os;
pub mod init;
pub mod dvd;
pub mod input;
pub mod bt;
pub mod ids;
pub mod cpp;
pub mod link;
