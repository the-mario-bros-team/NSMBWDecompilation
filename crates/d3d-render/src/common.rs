//! What the Direct3D 12 and 11 renderers share: compiling HLSL (kept on
//! disk), the small hand-written shaders, and egui's mesh layout.

use std::path::{Path, PathBuf};

use render::{Api, Res};
use windows::core::PCSTR;
use windows::Win32::Graphics::Direct3D::Fxc::{D3DCompile, D3DCOMPILE_ENABLE_STRICTNESS, D3DCOMPILE_OPTIMIZATION_LEVEL3};
use windows::Win32::Graphics::Direct3D::ID3DBlob;

use crate::shaders::{self, Blob};

/// The XFB blit (scaled copies with bilinear filtering: XFB copies, texture
/// patches, presenting): the source texel at a target pixel is
/// `(a.xy + pixel * a.zw) * inv`, as a Vulkan blit samples.
pub const BLIT_HLSL: &str = "
cbuffer Blit : register(b0) { float4 a; float2 inv; float2 pad; };
Texture2D<float4> src : register(t0);
SamplerState smp : register(s0);
float4 vs(uint i : SV_VertexID) : SV_Position {
    float2 p = float2((i << 1) & 2, i & 2);
    return float4(p * float2(2.0, -2.0) + float2(-1.0, 1.0), 0.0, 1.0);
}
float4 ps(float4 p : SV_Position) : SV_Target {
    return src.SampleLevel(smp, (a.xy + p.xy * a.zw) * inv, 0.0);
}
";

/// The menus (egui), as egui-ash-renderer draws them for Vulkan: sRGB
/// vertex colours and textures made linear, multiplied, and turned back
/// into sRGB with a 2.2 gamma for the UNORM swapchain.
pub const UI_HLSL: &str = "
cbuffer Ui : register(b0) { float2 screen; float2 pad; };
Texture2D<float4> tex : register(t0);
SamplerState smp : register(s0);
struct VI { float2 pos : POSITION; float2 uv : TEXCOORD0; float4 color : COLOR0; };
struct VO { float4 pos : SV_Position; float4 color : COLOR0; float2 uv : TEXCOORD0; };
VO vs(VI v) {
    VO o;
    o.pos = float4(2.0 * v.pos.x / screen.x - 1.0, 1.0 - 2.0 * v.pos.y / screen.y, 0.0, 1.0);
    o.color = float4(pow(v.color.rgb, 2.2), v.color.a);
    o.uv = v.uv;
    return o;
}
float4 ps(VO i) : SV_Target {
    float4 c = i.color * tex.Sample(smp, i.uv);
    return float4(pow(c.rgb, 1.0 / 2.2), c.a);
}
";

/// A clear of part of a target (Direct3D 11 clears only whole views): a
/// triangle over the viewport writing `color`, at depth `depth.x`.
pub const CLEAR_HLSL: &str = "
cbuffer Clear : register(b0) { float4 color; float4 depth; };
float4 vs(uint i : SV_VertexID) : SV_Position {
    float2 p = float2((i << 1) & 2, i & 2);
    return float4(p * float2(2.0, -2.0) + float2(-1.0, 1.0), depth.x, 1.0);
}
float4 ps(float4 p : SV_Position) : SV_Target {
    return color;
}
";

/// The blit's constants (`BLIT_HLSL`).
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BlitParams {
    pub a: [f32; 4],
    pub inv: [f32; 2],
    pub pad: [f32; 2],
}

impl BlitParams {
    /// Source rectangle (x, y, w, h) of a `sw` x `sh` texture onto the
    /// target rectangle (x, y, w, h).
    pub fn new(src: (u32, u32, u32, u32), sw: u32, sh: u32, dst: (u32, u32, u32, u32)) -> Self {
        let kx = src.2 as f32 / dst.2 as f32;
        let ky = src.3 as f32 / dst.3 as f32;
        Self { a: [src.0 as f32 - dst.0 as f32 * kx, src.1 as f32 - dst.1 as f32 * ky, kx, ky], inv: [1.0 / sw as f32, 1.0 / sh as f32], pad: [0.0; 2] }
    }
}

/// FNV-1a: a hash that stays the same from one build to the next.
fn fnv(parts: &[&[u8]]) -> u64 {
    let mut h = 0xcbf2_9ce4_8422_2325u64;
    for p in parts {
        for &b in *p {
            h = (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3);
        }
        h = (h ^ 0xFF).wrapping_mul(0x100_0000_01b3);
    }
    h
}

/// Compiles `entry` of `source` for `target` (vs_5_0, ps_5_1...), from the
/// cache in `dir` when it holds this very source.
pub fn compile(source: &str, entry: &str, target: &str, dir: Option<&Path>) -> Res<Blob> {
    let flags = std::env::var("NSMBW_FXC_FLAGS").ok().and_then(|v| u32::from_str_radix(&v, 16).ok()).unwrap_or(D3DCOMPILE_OPTIMIZATION_LEVEL3 | D3DCOMPILE_ENABLE_STRICTNESS);
    let key = fnv(&[source.as_bytes(), entry.as_bytes(), target.as_bytes(), &flags.to_le_bytes()]);
    let file: Option<PathBuf> = dir.map(|d| d.join(format!("{key:016x}.dxbc")));
    if let Some(code) = file.as_ref().and_then(|f| std::fs::read(f).ok()) {
        // DXBC starts with its magic.
        if code.starts_with(b"DXBC") {
            return Ok(code);
        }
    }
    let entry_c = format!("{entry}\0");
    let target_c = format!("{target}\0");
    let mut code: Option<ID3DBlob> = None;
    let mut errors: Option<ID3DBlob> = None;
    let r = unsafe {
        D3DCompile(
            source.as_ptr().cast(),
            source.len(),
            PCSTR(c"nsmbw.hlsl".as_ptr().cast()),
            None,
            None,
            PCSTR(entry_c.as_ptr()),
            PCSTR(target_c.as_ptr()),
            flags,
            0,
            &mut code,
            Some(&mut errors),
        )
    };
    let text = |b: &Option<ID3DBlob>| b.as_ref().map(|b| String::from_utf8_lossy(blob(b)).into_owned()).unwrap_or_default();
    if let Err(e) = r {
        return Err(format!("{entry} ({target}): {e}\n{}", text(&errors)).into());
    }
    let code = blob(code.as_ref().ok_or("D3DCompile returned no code")?).to_vec();
    if let Some(f) = &file {
        if let Some(d) = f.parent() {
            let _ = std::fs::create_dir_all(d);
        }
        let _ = std::fs::write(f, &code);
    }
    Ok(code)
}

pub fn blob(b: &ID3DBlob) -> &[u8] {
    unsafe { std::slice::from_raw_parts(b.GetBufferPointer().cast::<u8>(), b.GetBufferSize()) }
}

/// Every shader a Direct3D renderer uses, compiled.
pub struct Shaders {
    pub uber: shaders::Ubershader,
    pub copy_vs: Blob,
    pub copy_fs: Blob,
    pub blit_vs: Blob,
    pub blit_ps: Blob,
    pub ui_vs: Blob,
    pub ui_ps: Blob,
    pub clear_vs: Blob,
    pub clear_ps: Blob,
}

/// Where compiled shaders are kept: `d3d-shaders/` next to the pipeline
/// cache (`cache/`).
pub fn shader_dir(pipeline_cache: Option<&Path>) -> Option<PathBuf> {
    pipeline_cache.map(|p| p.parent().unwrap_or(Path::new(".")).join("d3d-shaders"))
}

pub fn compile_all(api: Api, dir: Option<&Path>) -> Res<Shaders> {
    let (vs, ps) = if api == Api::D3d12 { ("vs_5_1", "ps_5_1") } else { ("vs_5_0", "ps_5_0") };
    let started = std::time::Instant::now();
    let uber = shaders::hlsl(&shaders::ubershader_wgsl(api)?, api, shaders::Kind::Uber)?;
    let copy = shaders::hlsl(render::COPY, api, shaders::Kind::Copy)?;
    let s = Shaders {
        uber: shaders::Ubershader { vs: compile(&uber, "vs", vs, dir)?, fs: compile(&uber, "fs", ps, dir)?, fs_early: compile(&uber, "fs_early", ps, dir)?, fs_logic: compile(&uber, "fs_logic", ps, dir)? },
        copy_vs: compile(&copy, "vs", vs, dir)?,
        copy_fs: compile(&copy, "fs", ps, dir)?,
        blit_vs: compile(BLIT_HLSL, "vs", vs, dir)?,
        blit_ps: compile(BLIT_HLSL, "ps", ps, dir)?,
        ui_vs: compile(UI_HLSL, "vs", vs, dir)?,
        ui_ps: compile(UI_HLSL, "ps", ps, dir)?,
        clear_vs: compile(CLEAR_HLSL, "vs", vs, dir)?,
        clear_ps: compile(CLEAR_HLSL, "ps", ps, dir)?,
    };
    let took = started.elapsed().as_secs_f32();
    if took > 0.5 {
        println!("shaders: {} HLSL compiled in {took:.2} s", api.name());
    }
    Ok(s)
}

/// An egui texture's pixels (premultiplied sRGB RGBA8), with where they go.
pub fn ui_pixels(delta: &egui::epaint::ImageDelta) -> (Vec<u8>, usize, usize, Option<[usize; 2]>) {
    match &delta.image {
        egui::ImageData::Color(img) => (img.pixels.iter().flat_map(|c| c.to_array()).collect(), img.size[0], img.size[1], delta.pos),
    }
}

/// egui's vertices are 20 bytes: position, uv (f32 pairs), colour (RGBA8).
pub const UI_VERTEX_SIZE: u32 = std::mem::size_of::<egui::epaint::Vertex>() as u32;

#[cfg(test)]
mod tests {
    #[test]
    fn shaders_compile() {
        for api in [render::Api::D3d12, render::Api::D3d11] {
            if let Err(e) = super::compile_all(api, None) {
                panic!("{api:?}: {e}");
            }
        }
    }
}
