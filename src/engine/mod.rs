//! Engine / main-loop reconstruction.
//!
//! Start with [`startup`] for the thread hand-off and game main. [`globals`]
//! holds the recovered engine-global helpers; [`display`] holds the 2-D display
//! and pause layer; [`main_loop`] documents the MetroTRK debug paths.

pub mod startup;
pub mod main_loop;
pub mod globals;
pub mod display;
pub mod frame;
pub mod dsys;
