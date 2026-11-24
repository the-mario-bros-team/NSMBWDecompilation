//! The WGSL shaders as HLSL (naga), compiled to DXBC by the system's
//! `d3dcompiler_47.dll`.
//!
//! Direct3D differs from Vulkan in ways the WGSL is adapted for, as text,
//! before naga sees it:
//! - Clip space: y points up, so the vertex stage negates y (the CPU puts
//!   vertices in Vulkan's clip space). Window coordinates, winding and
//!   depth are then the same.
//! - `SV_InstanceID` starts at 0 whatever the draw's first instance, so the
//!   draw index comes from a per-instance vertex attribute (location 11, a
//!   buffer holding 0, 1, 2...) that does follow the first instance.
//! - Logic ops work on integer formats only: logic-op draws write the
//!   colour as integers (`fs_logic`) through the EFB's UINT view.
//! - Direct3D 11 (shader model 5.0) cannot index an array of textures with
//!   a value computed in the shader: each draw's eight textures and
//!   samplers are bound to slots 0..7 of their own ([`Api::D3d11`]).

use std::collections::BTreeMap;

use naga::back::hlsl;
use naga::valid::{Capabilities, ValidationFlags, Validator};
use render::{Api, Res};

/// Compiled shader bytecode.
pub type Blob = Vec<u8>;

/// The ubershader's vertex stage, fragment stage and fragment stage without
/// a depth output.
pub struct Ubershader {
    pub vs: Blob,
    pub fs: Blob,
    pub fs_early: Blob,
    /// For logic-op draws: the colour as integers (0..255), into the
    /// EFB's UINT view, where Direct3D has logic ops.
    pub fs_logic: Blob,
}

/// The fragment stage for logic-op draws (`Ubershader::fs_logic`).
const FS_LOGIC: &str = "
struct FOutLogic {
    @location(0) color: vec4<u32>,
    @builtin(frag_depth) depth: f32,
}

@fragment
fn fs_logic(i: VOut) -> FOutLogic {
    let s = shade(i);
    var o: FOutLogic;
    o.color = vec4<u32>(round(clamp(s.color, vec4<f32>(0.0), vec4<f32>(1.0)) * 255.0));
    o.depth = s.depth;
    return o;
}
";

/// The texture and sampler declarations of `shader.wgsl` and its
/// `sample`, replaced for Direct3D 11.
const D3D11_TEXTURES: &str = "
@group(0) @binding(1) var tex0: texture_2d<f32>;
@group(0) @binding(2) var tex1: texture_2d<f32>;
@group(0) @binding(6) var tex2: texture_2d<f32>;
@group(0) @binding(7) var tex3: texture_2d<f32>;
@group(0) @binding(8) var tex4: texture_2d<f32>;
@group(0) @binding(9) var tex5: texture_2d<f32>;
@group(0) @binding(10) var tex6: texture_2d<f32>;
@group(0) @binding(11) var tex7: texture_2d<f32>;
@group(1) @binding(0) var smp0: sampler;
@group(1) @binding(1) var smp1: sampler;
@group(1) @binding(2) var smp2: sampler;
@group(1) @binding(3) var smp3: sampler;
@group(1) @binding(4) var smp4: sampler;
@group(1) @binding(5) var smp5: sampler;
@group(1) @binding(6) var smp6: sampler;
@group(1) @binding(7) var smp7: sampler;
";

const D3D11_SAMPLE: &str = "fn sample(m: u32, uv: vec2<f32>, dx: vec2<f32>, dy: vec2<f32>) -> vec4<f32> {
    if (word(TEX + m) == NONE) {
        return vec4<f32>(1.0);
    }
    switch m {
        case 0u: { return textureSampleGrad(tex0, smp0, uv, dx, dy); }
        case 1u: { return textureSampleGrad(tex1, smp1, uv, dx, dy); }
        case 2u: { return textureSampleGrad(tex2, smp2, uv, dx, dy); }
        case 3u: { return textureSampleGrad(tex3, smp3, uv, dx, dy); }
        case 4u: { return textureSampleGrad(tex4, smp4, uv, dx, dy); }
        case 5u: { return textureSampleGrad(tex5, smp5, uv, dx, dy); }
        case 6u: { return textureSampleGrad(tex6, smp6, uv, dx, dy); }
        default: { return textureSampleGrad(tex7, smp7, uv, dx, dy); }
    }
}
";

fn replace_once(s: &str, from: &str, to: &str) -> Res<String> {
    if s.matches(from).count() != 1 {
        return Err(format!("shader.wgsl: expected one `{}`", from.lines().next().unwrap_or(from)).into());
    }
    Ok(s.replacen(from, to, 1))
}

/// `shader.wgsl` adapted for Direct3D (see the module's documentation).
pub fn ubershader_wgsl(api: Api) -> Res<String> {
    // Line ends as checked out (CRLF on Windows) made plain.
    let mut s = render::UBERSHADER.replace('\r', "");
    s = replace_once(&s, "@builtin(instance_index) inst: u32", "@location(11) inst: u32")?;
    s = replace_once(&s, "    o.draw = inst;\n    return o;", "    o.draw = inst;\n    o.pos.y = -o.pos.y;\n    return o;")?;
    if api == Api::D3d11 {
        s = replace_once(&s, "enable wgpu_binding_array;\n", "")?;
        s = replace_once(
            &s,
            "@group(0) @binding(1) var textures: binding_array<texture_2d<f32>, 4096>;\n@group(0) @binding(2) var samplers: binding_array<sampler, 256>;\n",
            D3D11_TEXTURES,
        )?;
        let start = s.find("fn sample(").ok_or("shader.wgsl: no `fn sample`")?;
        let end = start + s[start..].find("\n}\n").ok_or("shader.wgsl: `fn sample` has no end")? + 3;
        s.replace_range(start..end, D3D11_SAMPLE);
    }
    s.push_str(FS_LOGIC);
    Ok(s)
}

fn target(space: u8, register: u32) -> hlsl::BindTarget {
    hlsl::BindTarget { space, register, binding_array_size: None, dynamic_storage_buffer_offsets_index: None, restrict_indexing: false }
}

fn binding(group: u32, binding: u32) -> naga::ResourceBinding {
    naga::ResourceBinding { group, binding }
}

/// Where each WGSL binding goes. Direct3D 12 (root signature in `d3d12`):
/// - t0: draw states; t1..t3: frame interpolation's buffers (root SRVs);
/// - t0 space1: the textures (a table of 4096);
/// - s0: the sampler heap naga indexes through t4 (the sampler index
///   buffer, 0..255 in order: the frame's samplers start the table);
/// - b0: the vertex stage's immediates.
///
/// Direct3D 11: t0 draw states, t1..t3 interpolation, t4..t11 the draw's
/// textures, s0..s7 its samplers, b0 the immediates; the copy pass's EFB
/// at t0/t1 and its parameters at b0.
fn options(api: Api, kind: Kind) -> hlsl::Options {
    let mut map = BTreeMap::new();
    if kind == Kind::Copy {
        map.insert(binding(0, 0), target(0, 0));
        map.insert(binding(0, 1), target(0, 1));
        return hlsl::Options {
            shader_model: if api == Api::D3d12 { hlsl::ShaderModel::V5_1 } else { hlsl::ShaderModel::V5_0 },
            binding_map: map,
            immediates_target: Some(target(0, 0)),
            ..Default::default()
        };
    }
    map.insert(binding(0, 0), target(0, 0));
    map.insert(binding(0, 3), target(0, 1));
    map.insert(binding(0, 4), target(0, 2));
    map.insert(binding(0, 5), target(0, 3));
    let mut samplers = BTreeMap::new();
    match api {
        Api::D3d12 => {
            map.insert(binding(0, 1), hlsl::BindTarget { binding_array_size: Some(4096), ..target(1, 0) });
            // Samplers: the register is the index into the index buffer.
            map.insert(binding(0, 2), hlsl::BindTarget { binding_array_size: Some(256), ..target(0, 0) });
            samplers.insert(hlsl::SamplerIndexBufferKey { group: 0 }, target(0, 4));
        }
        _ => {
            for (i, b) in [1, 2, 6, 7, 8, 9, 10, 11].into_iter().enumerate() {
                map.insert(binding(0, b), target(0, 4 + i as u32));
                map.insert(binding(1, i as u32), target(0, i as u32));
            }
            samplers.insert(hlsl::SamplerIndexBufferKey { group: 1 }, target(0, 12));
        }
    }
    hlsl::Options {
        shader_model: if api == Api::D3d12 { hlsl::ShaderModel::V5_1 } else { hlsl::ShaderModel::V5_0 },
        binding_map: map,
        fake_missing_bindings: false,
        immediates_target: Some(target(0, 0)),
        sampler_buffer_binding_map: samplers,
        ..Default::default()
    }
}

/// Which WGSL source: the ubershader or the copy pass.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Uber,
    Copy,
}

/// HLSL for every entry point of `source`.
pub fn hlsl(source: &str, api: Api, kind: Kind) -> Res<String> {
    let module = naga::front::wgsl::parse_str(source).map_err(|e| e.emit_to_string(source))?;
    let info = Validator::new(ValidationFlags::all(), Capabilities::all()).validate(&module).map_err(|e| e.emit_to_string(source))?;
    let mut out = String::new();
    let opts = options(api, kind);
    let pipeline = hlsl::PipelineOptions { entry_point: None };
    hlsl::Writer::new(&mut out, &opts, &pipeline).write(&module, &info, None).map_err(|e| format!("HLSL: {e}"))?;
    if api == Api::D3d11 {
        out = shader_model_5_0(&out);
    }
    Ok(out)
}

/// naga writes for Direct3D 12: shader model 5.1's `ConstantBuffer<T>`, and
/// samplers taken from a heap through an index buffer. Shader model 5.0
/// gets `cbuffer`s and each sampler at a register of its own.
fn shader_model_5_0(hlsl: &str) -> String {
    let mut out = String::with_capacity(hlsl.len());
    for line in hlsl.lines() {
        let t = line.trim_start();
        if t.starts_with("SamplerState nagaSamplerHeap") || t.starts_with("SamplerComparisonState nagaComparisonSamplerHeap") || (t.starts_with("StructuredBuffer<uint> nagaGroup") && t.contains("SamplerIndexArray")) {
            continue;
        }
        // static const SamplerState smp0_ = nagaSamplerHeap[nagaGroup1SamplerIndexArray[0]];
        if let Some(rest) = t.strip_prefix("static const SamplerState ") {
            if let (Some((name, _)), Some(i)) = (rest.split_once(" = "), rest.rsplit_once('[').and_then(|(_, i)| i.split(']').next())) {
                out.push_str(&format!("SamplerState {name} : register(s{i});\n"));
                continue;
            }
        }
        // ConstantBuffer<Interp> interp: register(b0);
        if let Some(rest) = t.strip_prefix("ConstantBuffer<") {
            if let Some((ty, rest)) = rest.split_once('>') {
                if let Some((name, reg)) = rest.split_once(':') {
                    out.push_str(&format!("cbuffer {ty}_block : {} {{ {ty} {}; }}\n", reg.trim().trim_end_matches(';'), name.trim()));
                    continue;
                }
            }
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_hlsl() {
        for api in [Api::D3d12, Api::D3d11] {
            let wgsl = ubershader_wgsl(api).unwrap();
            let text = hlsl(&wgsl, api, Kind::Uber).unwrap_or_else(|e| panic!("{e}"));
            if let Some(dir) = std::env::var_os("NSMBW_HLSL_DUMP") {
                std::fs::write(std::path::Path::new(&dir).join(format!("{api:?}.hlsl")), &text).unwrap();
                std::fs::write(std::path::Path::new(&dir).join(format!("copy-{api:?}.hlsl")), hlsl(render::COPY, api, Kind::Copy).unwrap()).unwrap();
            }
        }
    }
}
