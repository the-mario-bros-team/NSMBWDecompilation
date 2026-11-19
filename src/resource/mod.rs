//! Resource formats and loaders.
//!
//! [`archive`] covers the U8/ARC reader and packed-value decoding; [`course`]
//! covers stage/course data; [`bres`] holds the BRRES loaders; [`registry`]
//! owns the `dRes_c`/`info_c` archive-resource registry. This build has no
//! BCSV container (see `registry.rs`).

pub mod archive;
pub mod course;
pub mod bres;
pub mod registry;
