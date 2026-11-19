//! # Rendering (GX)
//!
//! Semantic home for the GX/NW4R rendering path: GX initialization, state
//! blocks, FIFO command construction, TEV/material setup, and draw calls.
//!
//! Start with [`gx`] for the command/state layer and [`geometry`] for vertex
//! grids and quad emission.

pub mod gx;
pub mod geometry;
pub mod gx_band;
