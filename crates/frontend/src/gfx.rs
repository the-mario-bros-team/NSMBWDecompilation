//! Picks the renderer (Settings' Renderer, `--renderer`) and makes it.

use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use render::{Api, Backend, Options, Res};

use crate::settings::RendererChoice;

/// A renderer of any backend.
pub type Renderer = Box<dyn Backend>;

/// The APIs tried for `choice`, in order. Automatic: Vulkan when there is a
/// Vulkan 1.3 device with dual-source blending and logic ops (what
/// `vk-render` asks for, so making it fails without one), then Direct3D 12,
/// then Direct3D 11.
fn order(choice: RendererChoice) -> &'static [Api] {
    match choice {
        RendererChoice::Auto if cfg!(windows) => &[Api::Vulkan, Api::D3d12, Api::D3d11],
        RendererChoice::Auto | RendererChoice::Vulkan => &[Api::Vulkan],
        RendererChoice::D3d12 => &[Api::D3d12],
        RendererChoice::D3d11 => &[Api::D3d11],
    }
}

fn make(api: Api, window: Option<(RawDisplayHandle, RawWindowHandle)>, opts: Options) -> Res<Renderer> {
    Ok(match (api, window) {
        (Api::Vulkan, Some((d, w))) => Box::new(vk_render::Renderer::new(d, w, opts)?),
        (Api::Vulkan, None) => Box::new(vk_render::Renderer::headless(opts)?),
        #[cfg(windows)]
        (Api::D3d12, Some((d, w))) => Box::new(d3d_render::d3d12::Renderer::new(d, w, opts)?),
        #[cfg(windows)]
        (Api::D3d12, None) => Box::new(d3d_render::d3d12::Renderer::headless(opts)?),
        #[cfg(windows)]
        (Api::D3d11, Some((d, w))) => Box::new(d3d_render::d3d11::Renderer::new(d, w, opts)?),
        #[cfg(windows)]
        (Api::D3d11, None) => Box::new(d3d_render::d3d11::Renderer::headless(opts)?),
        #[allow(unreachable_patterns)]
        (api, _) => return Err(format!("{} is not available on this platform", api.name()).into()),
    })
}

fn first(choice: RendererChoice, window: Option<(RawDisplayHandle, RawWindowHandle)>, opts: Options) -> Result<Renderer, String> {
    let mut failed = Vec::new();
    for &api in order(choice) {
        match make(api, window, opts.clone()) {
            Ok(r) => {
                for f in &failed {
                    println!("renderer: {f}; using {}", api.name());
                }
                return Ok(r);
            }
            Err(e) => failed.push(format!("{}: {e}", api.name())),
        }
    }
    Err(failed.join("; "))
}

/// The renderer for a window.
pub fn create(choice: RendererChoice, display: RawDisplayHandle, window: RawWindowHandle, opts: Options) -> Result<Renderer, String> {
    first(choice, Some((display, window)), opts)
}

/// A renderer without a window (FIFO-log replay).
pub fn headless(choice: RendererChoice, opts: Options) -> Result<Renderer, String> {
    first(choice, None, opts)
}
