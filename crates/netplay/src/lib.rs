//! Online multiplayer, shared by the PC frontend (`crates/frontend`) and
//! the Android app (`crates/android`): [`net`] runs a lockstep session
//! over TCP, [`lan`] finds rooms on the local network and over Tailscale.

pub mod lan;
pub mod net;
