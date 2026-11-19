//! # Audio (AX / JAS / BRSAR)
//!
//! Semantic home for audio: AX voice/DSP control, JAS/JAI sequencing, BRSAR
//! banks, and BRSTM streaming.
//!
//! Start with [`ax`] for the DSP/AX layer and [`seq`] for RSEQ sequencing.
//! The Windows port replaces the backend, not this logic.

pub mod ax;
pub mod seq;
