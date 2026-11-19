//! # Game systems
//!
//! Semantic home for gameplay: actor/profile framework, stage/course loading,
//! player, enemies, bosses, items, and modes.
//!
//! Start with [`actor`] and [`stage`]. [`player`], [`path`], [`worldmap`],
//! [`interaction`], [`yoshi`] and [`boot_scene`] are promoted from the first
//! translation pass. [`actors`] holds actor-specific modules.

pub mod actor;
pub mod stage;
pub mod player;
pub mod path;
pub mod worldmap;
pub mod interaction;
pub mod yoshi;
pub mod boot_scene;
pub mod scene;
pub mod task;
pub mod enemies;
pub mod bosses;
pub mod profile;
pub mod bases;
pub mod actors;
