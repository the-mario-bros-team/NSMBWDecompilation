//! Direct3D 12 and Direct3D 11 renderers (Windows), drawing `gx::Output`
//! the way `vk-render` does with Vulkan, through the same
//! [`render::Backend`].

#[cfg(windows)]
mod common;
#[cfg(windows)]
pub mod shaders;
#[cfg(windows)]
pub mod d3d12;
#[cfg(windows)]
pub mod d3d11;
