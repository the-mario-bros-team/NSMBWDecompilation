//! Source-level Rust reconstruction of New SUPER MARIO BROS. Wii (SMNE01).
//!
//! Ghidra pseudocode lives under `work/ghidra/main-gb/export` as evidence; this
//! crate is the authoritative reconstruction.
//!
//! Start with `docs/READING_GUIDE.md`, then `boot::entry`, `runtime::os`, and
//! `engine::startup`. Range modules under `subsystems` are extraction staging;
//! see `docs/ORGANIZATION.md`.

#![no_std]
#![allow(dead_code, non_snake_case, non_camel_case_types, unused_variables, unused_mut)]

pub mod platform;
pub mod boot;
pub mod runtime;
pub mod engine;
pub mod rels;
pub mod render;
pub mod audio;
pub mod ui;
pub mod game;
pub mod resource;
pub mod subsystems;
